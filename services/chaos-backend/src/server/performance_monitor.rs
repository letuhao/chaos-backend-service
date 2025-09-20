//! Performance Monitor
//!
//! This module provides real-time performance monitoring for the game server.
//! Tracks FPS, memory usage, network stats, and other critical metrics.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::time::interval;
use tracing::{info, warn, debug};

use crate::server::SharedGameState;

/// Performance monitor for the game server
pub struct PerformanceMonitor {
    /// Shared game state
    shared_state: Arc<SharedGameState>,
    
    /// Monitor task handle
    monitor_task: Option<tokio::task::JoinHandle<()>>,
    
    /// Monitor running flag
    is_running: Arc<std::sync::atomic::AtomicBool>,
    
    /// Monitor interval
    monitor_interval: Duration,
    
    /// Frame time tracking
    frame_times: Arc<RwLock<Vec<Duration>>>,
    
    /// Last frame time
    last_frame_time: Arc<RwLock<Option<Instant>>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(shared_state: Arc<SharedGameState>) -> Self {
        Self {
            shared_state,
            monitor_task: None,
            is_running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            monitor_interval: Duration::from_millis(100), // 10 FPS monitoring
            frame_times: Arc::new(RwLock::new(Vec::new())),
            last_frame_time: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Start the performance monitor
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            warn!("Performance monitor is already running");
            return Ok(());
        }
        
        info!("Starting performance monitor...");
        
        let shared_state = self.shared_state.clone();
        let monitor_interval = self.monitor_interval;
        let is_running = self.is_running.clone();
        let frame_times = self.frame_times.clone();
        let last_frame_time = self.last_frame_time.clone();
        
        let monitor_task = tokio::spawn(async move {
            let mut interval = interval(monitor_interval);
            
            // Set running flag
            is_running.store(true, std::sync::atomic::Ordering::Relaxed);
            info!("Performance monitor started");
            
            loop {
                interval.tick().await;
                
                if !is_running.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
                
                // Update performance metrics
                Self::update_performance_metrics(
                    &shared_state,
                    &frame_times,
                    &last_frame_time,
                ).await;
            }
            
            info!("Performance monitor stopped");
        });
        
        self.monitor_task = Some(monitor_task);
        
        info!("Performance monitor started successfully");
        Ok(())
    }
    
    /// Stop the performance monitor
    pub async fn stop(&mut self) {
        if !self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        
        info!("Stopping performance monitor...");
        
        // Set running flag to false
        self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);
        
        // Wait for monitor task to finish
        if let Some(task) = self.monitor_task.take() {
            task.abort();
            let _ = task.await;
        }
        
        info!("Performance monitor stopped");
    }
    
    /// Record a frame time
    pub fn record_frame_time(&self) {
        let now = Instant::now();
        let mut last_frame = self.last_frame_time.write();
        
        if let Some(prev_time) = last_frame.replace(now) {
            let frame_duration = now.duration_since(prev_time);
            self.frame_times.write().push(frame_duration);
            
            // Keep only last 100 frame times for rolling average
            let mut frame_times = self.frame_times.write();
            if frame_times.len() > 100 {
                frame_times.remove(0);
            }
        }
    }
    
    /// Update performance metrics
    async fn update_performance_metrics(
        shared_state: &SharedGameState,
        frame_times: &Arc<RwLock<Vec<Duration>>>,
        last_frame_time: &Arc<RwLock<Option<Instant>>>,
    ) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Calculate FPS
        let fps = Self::calculate_fps(frame_times);
        
        // Calculate average frame time
        let avg_frame_time = Self::calculate_avg_frame_time(frame_times);
        
        // Get memory usage
        let memory_usage = Self::get_memory_usage();
        
        // Get active actors count
        let active_actors = shared_state.actors.read().len() as u32;
        
        // Create performance metrics
        let metrics = crate::server::PerformanceMetrics {
            fps,
            avg_frame_time_ms: avg_frame_time.as_secs_f64() * 1000.0,
            memory_usage_mb: memory_usage,
            active_actors,
            packets_per_second: 0, // TODO: Implement network monitoring
            db_queries_per_second: 0, // TODO: Implement database monitoring
            last_update: current_time,
        };
        
        // Update shared state
        shared_state.update_metrics(metrics);
        
        // Log performance every 5 seconds
        if current_time % 5 == 0 {
            debug!(
                "Performance: FPS={:.1}, FrameTime={:.2}ms, Memory={:.1}MB, Actors={}",
                fps, avg_frame_time.as_secs_f64() * 1000.0, memory_usage, active_actors
            );
        }
    }
    
    /// Calculate FPS from frame times
    fn calculate_fps(frame_times: &Arc<RwLock<Vec<Duration>>>) -> f64 {
        let frame_times = frame_times.read();
        
        if frame_times.is_empty() {
            return 0.0;
        }
        
        let total_time: Duration = frame_times.iter().sum();
        let avg_frame_time = total_time / frame_times.len() as u32;
        
        if avg_frame_time.as_secs_f64() > 0.0 {
            1.0 / avg_frame_time.as_secs_f64()
        } else {
            0.0
        }
    }
    
    /// Calculate average frame time
    fn calculate_avg_frame_time(frame_times: &Arc<RwLock<Vec<Duration>>>) -> Duration {
        let frame_times = frame_times.read();
        
        if frame_times.is_empty() {
            return Duration::from_millis(16); // 60 FPS default
        }
        
        let total_time: Duration = frame_times.iter().sum();
        total_time / frame_times.len() as u32
    }
    
    /// Get current memory usage in MB
    fn get_memory_usage() -> f64 {
        // Simple memory usage estimation
        // In a real implementation, you would use system APIs
        // For now, return a placeholder value
        100.0
    }
    
    /// Get current performance metrics
    pub fn get_current_metrics(&self) -> crate::server::PerformanceMetrics {
        self.shared_state.get_metrics()
    }
    
    /// Check if performance is within acceptable limits
    pub fn is_performance_acceptable(&self) -> bool {
        let metrics = self.get_current_metrics();
        
        // Check FPS (should be > 30)
        if metrics.fps < 30.0 {
            return false;
        }
        
        // Check frame time (should be < 33ms for 30 FPS)
        if metrics.avg_frame_time_ms > 33.0 {
            return false;
        }
        
        // Check memory usage (should be < 1GB)
        if metrics.memory_usage_mb > 1024.0 {
            return false;
        }
        
        true
    }
    
    /// Get performance report
    pub fn get_performance_report(&self) -> PerformanceReport {
        let metrics = self.get_current_metrics();
        
        PerformanceReport {
            fps: metrics.fps,
            avg_frame_time_ms: metrics.avg_frame_time_ms,
            memory_usage_mb: metrics.memory_usage_mb,
            active_actors: metrics.active_actors,
            is_acceptable: self.is_performance_acceptable(),
            timestamp: metrics.last_update,
        }
    }
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub fps: f64,
    pub avg_frame_time_ms: f64,
    pub memory_usage_mb: f64,
    pub active_actors: u32,
    pub is_acceptable: bool,
    pub timestamp: u64,
}

impl Drop for PerformanceMonitor {
    fn drop(&mut self) {
        if self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            // Set running flag to false
            self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);
            
            // Abort monitor task if running
            if let Some(task) = self.monitor_task.take() {
                task.abort();
            }
        }
    }
}
