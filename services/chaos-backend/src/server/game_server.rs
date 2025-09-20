//! Game Server
//!
//! This is the main game server that orchestrates all game systems
//! in a single process with shared memory architecture.

use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, warn, error, debug};

use actor_core::prelude::*;

use crate::server::{
    SharedGameState, 
    shared_state::ServerConfig,
    ConfigSyncService, 
    ActorManager, 
    performance_monitor::PerformanceMonitor
};

/// Main game server
pub struct GameServer {
    /// Shared game state
    shared_state: Arc<SharedGameState>,
    
    /// Configuration sync service
    config_sync: Option<ConfigSyncService>,
    
    /// Actor manager
    actor_manager: ActorManager,
    
    /// Performance monitor
    performance_monitor: Option<PerformanceMonitor>,
    
    /// Server running flag
    is_running: Arc<std::sync::atomic::AtomicBool>,
    
    /// Main game loop task
    game_loop_task: Option<tokio::task::JoinHandle<()>>,
}

impl GameServer {
    /// Create a new game server
    pub async fn new(server_config: ServerConfig) -> ActorCoreResult<Self> {
        info!("Initializing game server: {}", server_config.name);
        
        // Create shared game state
        let shared_state = Arc::new(SharedGameState::new(server_config).await?);
        
        // Create actor manager
        let actor_manager = ActorManager::new(shared_state.clone());
        
        // Create configuration sync service
        let config_sync = Some(ConfigSyncService::new(shared_state.clone()));
        
        // Create performance monitor
        let performance_monitor = Some(PerformanceMonitor::new(shared_state.clone()));
        
        Ok(Self {
            shared_state,
            config_sync,
            actor_manager,
            performance_monitor,
            is_running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            game_loop_task: None,
        })
    }
    
    /// Start the game server
    pub async fn start(&mut self) -> ActorCoreResult<()> {
        if self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            warn!("Game server is already running");
            return Ok(());
        }
        
        info!("Starting game server...");
        
        // Start configuration sync service
        if let Some(ref mut config_sync) = self.config_sync {
            config_sync.start().await?;
            info!("Configuration sync service started");
        }
        
        // Start performance monitor
        if let Some(ref mut performance_monitor) = self.performance_monitor {
            performance_monitor.start().await?;
            info!("Performance monitor started");
        }
        
        // Start main game loop
        self.start_game_loop().await?;
        
        // Set running flag
        self.is_running.store(true, std::sync::atomic::Ordering::Relaxed);
        
        info!("Game server started successfully");
        Ok(())
    }
    
    /// Stop the game server
    pub async fn stop(&mut self) {
        if !self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        
        info!("Stopping game server...");
        
        // Set running flag to false
        self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);
        
        // Stop game loop
        if let Some(task) = self.game_loop_task.take() {
            task.abort();
            let _ = task.await;
        }
        
        // Stop performance monitor
        if let Some(ref mut performance_monitor) = self.performance_monitor {
            performance_monitor.stop().await;
        }
        
        // Stop configuration sync service
        if let Some(ref mut config_sync) = self.config_sync {
            config_sync.stop().await;
        }
        
        info!("Game server stopped");
    }
    
    /// Start the main game loop
    async fn start_game_loop(&mut self) -> ActorCoreResult<()> {
        let shared_state = self.shared_state.clone();
        let is_running = self.is_running.clone();
        let server_config = self.shared_state.server_config.read();
        let tick_rate = server_config.tick_rate;
        let tick_duration = Duration::from_millis(1000 / tick_rate as u64);
        
        let game_loop_task = tokio::spawn(async move {
            let mut interval = interval(tick_duration);
            let mut frame_count = 0u64;
            
            info!("Game loop started with tick rate: {} Hz", tick_rate);
            
            loop {
                interval.tick().await;
                
                if !is_running.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
                
                // Record frame time for performance monitoring
                if let Some(performance_monitor) = shared_state.performance_monitor.as_ref() {
                    performance_monitor.record_frame_time();
                }
                
                // Update game world
                Self::update_game_world(&shared_state).await;
                
                // Update frame count
                frame_count += 1;
                
                // Log every 60 frames (1 second at 60 FPS)
                if frame_count % 60 == 0 {
                    debug!("Game loop running: {} frames processed", frame_count);
                }
            }
            
            info!("Game loop stopped after {} frames", frame_count);
        });
        
        self.game_loop_task = Some(game_loop_task);
        Ok(())
    }
    
    /// Update game world state
    async fn update_game_world(shared_state: &SharedGameState) {
        // Update world time
        shared_state.update_world_state(|world_state| {
            world_state.game_time += 1;
        });
        
        // Update actors
        let actors = shared_state.actor_manager.get_all_actors();
        for mut actor in actors {
            // Apply any periodic updates to actors
            // This is where you would add game logic like:
            // - Health regeneration
            // - Experience gain
            // - Status effect updates
            // - AI processing
            
            // For now, just update the actor
            if let Err(e) = shared_state.actor_manager.update_actor(actor).await {
                error!("Failed to update actor: {}", e);
            }
        }
    }
    
    /// Get shared game state
    pub fn get_shared_state(&self) -> Arc<SharedGameState> {
        self.shared_state.clone()
    }
    
    /// Get actor manager
    pub fn get_actor_manager(&self) -> &ActorManager {
        &self.actor_manager
    }
    
    /// Get server status
    pub fn get_server_status(&self) -> ServerStatus {
        ServerStatus {
            is_running: self.is_running.load(std::sync::atomic::Ordering::Relaxed),
            uptime_seconds: 0, // TODO: Calculate actual uptime
            active_actors: self.actor_manager.get_active_count(),
            performance_metrics: self.shared_state.get_metrics(),
        }
    }
    
    /// Create a test actor
    pub async fn create_test_actor(&self, name: String, race: String) -> ActorCoreResult<Actor> {
        self.actor_manager.create_actor(name, race).await
    }
    
    /// Get all actors
    pub fn get_all_actors(&self) -> Vec<Actor> {
        self.actor_manager.get_all_actors()
    }
}

/// Server status
#[derive(Debug, Clone)]
pub struct ServerStatus {
    pub is_running: bool,
    pub uptime_seconds: u64,
    pub active_actors: u32,
    pub performance_metrics: crate::server::shared_state::PerformanceMetrics,
}

impl Drop for GameServer {
    fn drop(&mut self) {
        if self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            // Set running flag to false
            self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);
            
            // Abort game loop task if running
            if let Some(task) = self.game_loop_task.take() {
                task.abort();
            }
        }
    }
}
