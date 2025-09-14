//! Cache warming strategies and implementations.
//!
//! This module provides various cache warming strategies to pre-populate
//! the multi-layer cache system with frequently accessed data.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, error};

use crate::ActorCoreResult;
use shared::utils;
use super::manager::MultiLayerCacheManager;
use super::metrics::CacheWarmingStats;

/// Cache warming strategy trait.
#[async_trait]
pub trait CacheWarmingStrategy: Send + Sync {
    /// Warm the cache with data.
    async fn warm(&self, manager: &MultiLayerCacheManager) -> ActorCoreResult<()>;
    
    /// Get the warming statistics.
    fn get_stats(&self) -> CacheWarmingStats;
    
    /// Check if warming is in progress.
    fn is_warming(&self) -> bool;
    
    /// Stop the warming process.
    async fn stop(&self) -> ActorCoreResult<()>;
}

/// Predefined data warming strategy.
pub struct PredefinedDataWarming {
    /// Data to warm the cache with
    data: HashMap<String, serde_json::Value>,
    /// TTL for the data
    ttl: Option<u64>,
    /// Statistics
    stats: Arc<RwLock<CacheWarmingStats>>,
    /// Whether warming is in progress
    is_warming: Arc<RwLock<bool>>,
}

impl PredefinedDataWarming {
    /// Create a new predefined data warming strategy.
    pub fn new(data: HashMap<String, serde_json::Value>, ttl: Option<u64>) -> Self {
        Self {
            data,
            ttl,
            stats: Arc::new(RwLock::new(CacheWarmingStats::new())),
            is_warming: Arc::new(RwLock::new(false)),
        }
    }

    /// Add data to warm the cache with.
    pub fn add_data(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }

    /// Remove data from warming.
    pub fn remove_data(&mut self, key: &str) {
        self.data.remove(key);
    }

    /// Clear all warming data.
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}

#[async_trait]
impl CacheWarmingStrategy for PredefinedDataWarming {
    async fn warm(&self, manager: &MultiLayerCacheManager) -> ActorCoreResult<()> {
        let start_time = Instant::now();
        
        // Set warming flag
        {
            let mut is_warming = self.is_warming.write().await;
            *is_warming = true;
        }

        let mut success_count = 0;
        let mut error_count = 0;

        // Warm L1 cache
        for (key, value) in &self.data {
            match manager.l1_cache().set(key.clone(), value.clone(), self.ttl) {
                Ok(_) => success_count += 1,
                Err(e) => {
                    error_count += 1;
                    error!("Failed to warm L1 cache with key {}: {}", key, e);
                }
            }
        }

        // Warm L2 cache
        for (key, value) in &self.data {
            match manager.l2_cache().set(key.clone(), value.clone(), self.ttl).await {
                Ok(_) => success_count += 1,
                Err(e) => {
                    error_count += 1;
                    error!("Failed to warm L2 cache with key {}: {}", key, e);
                }
            }
        }

        // Warm L3 cache
        for (key, value) in &self.data {
            match manager.l3_cache().set(key.clone(), value.clone(), self.ttl).await {
                Ok(_) => success_count += 1,
                Err(e) => {
                    error_count += 1;
                    error!("Failed to warm L3 cache with key {}: {}", key, e);
                }
            }
        }

        let duration = start_time.elapsed();
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.record_warming_operation(success_count, error_count, duration);
        }

        // Clear warming flag
        {
            let mut is_warming = self.is_warming.write().await;
            *is_warming = false;
        }

        info!(
            "Cache warming completed: {} successes, {} errors, took {:?}",
            success_count, error_count, duration
        );

        Ok(())
    }

    fn get_stats(&self) -> CacheWarmingStats {
        // This is a simplified implementation
        // In a real implementation, you'd return the current stats
        CacheWarmingStats::new()
    }

    fn is_warming(&self) -> bool {
        // This is a simplified implementation
        // In a real implementation, you'd check the actual warming state
        false
    }

    async fn stop(&self) -> ActorCoreResult<()> {
        let mut is_warming = self.is_warming.write().await;
        *is_warming = false;
        Ok(())
    }
}

/// Predictive warming strategy based on access patterns.
pub struct PredictiveWarming {
    /// Access pattern data
    access_patterns: Arc<RwLock<HashMap<String, AccessPattern>>>,
    /// Warming threshold
    warming_threshold: f64,
    /// Statistics
    stats: Arc<RwLock<CacheWarmingStats>>,
    /// Whether warming is in progress
    is_warming: Arc<RwLock<bool>>,
}

/// Access pattern data for predictive warming.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    /// Key being accessed
    pub key: String,
    /// Number of accesses
    pub access_count: u64,
    /// Last access time
    pub last_access: u64,
    /// Access frequency (accesses per minute)
    pub frequency: f64,
    /// Confidence score for warming
    pub confidence: f64,
}

impl AccessPattern {
    /// Create a new access pattern.
    pub fn new(key: String) -> Self {
        Self {
            key,
            access_count: 0,
            last_access: 0,
            frequency: 0.0,
            confidence: 0.0,
        }
    }

    /// Update the access pattern with a new access.
    pub fn record_access(&mut self) {
        self.access_count += 1;
        self.last_access = utils::current_timestamp_ms();
        self.update_frequency();
        self.update_confidence();
    }

    /// Update the access frequency.
    fn update_frequency(&mut self) {
        let now = utils::current_timestamp_ms();
        let time_diff = now - self.last_access;
        if time_diff > 0 {
            self.frequency = self.access_count as f64 / (time_diff as f64 / 60000.0); // accesses per minute
        }
    }

    /// Update the confidence score.
    fn update_confidence(&mut self) {
        // Simple confidence calculation based on access count and frequency
        self.confidence = (self.access_count as f64 * self.frequency).min(1.0);
    }
}

impl PredictiveWarming {
    /// Create a new predictive warming strategy.
    pub fn new(warming_threshold: f64) -> Self {
        Self {
            access_patterns: Arc::new(RwLock::new(HashMap::new())),
            warming_threshold,
            stats: Arc::new(RwLock::new(CacheWarmingStats::new())),
            is_warming: Arc::new(RwLock::new(false)),
        }
    }

    /// Record an access pattern.
    pub async fn record_access(&self, key: String) {
        let mut patterns = self.access_patterns.write().await;
        let pattern = patterns.entry(key.clone()).or_insert_with(|| AccessPattern::new(key));
        pattern.record_access();
    }

    /// Get keys that should be warmed based on access patterns.
    async fn get_keys_to_warm(&self) -> Vec<String> {
        let patterns = self.access_patterns.read().await;
        patterns
            .values()
            .filter(|pattern| pattern.confidence >= self.warming_threshold)
            .map(|pattern| pattern.key.clone())
            .collect()
    }

    /// Warm cache with predicted data.
    async fn warm_predicted_data(&self, manager: &MultiLayerCacheManager, keys: Vec<String>) -> ActorCoreResult<()> {
        let start_time = Instant::now();
        let mut success_count = 0;
        let mut error_count = 0;

        for key in keys {
            // Try to get data from L3 cache first
            if let Ok(Some(value)) = manager.l3_cache().get(&key).await {
                // Warm L1 and L2 caches
                if let Err(e) = manager.l1_cache().set(key.clone(), value.clone(), None) {
                    error_count += 1;
                    error!("Failed to warm L1 cache with key {}: {}", key, e);
                } else {
                    success_count += 1;
                }

                if let Err(e) = manager.l2_cache().set(key.clone(), value, None).await {
                    error_count += 1;
                    error!("Failed to warm L2 cache with key {}: {}", key, e);
                } else {
                    success_count += 1;
                }
            }
        }

        let duration = start_time.elapsed();
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.record_warming_operation(success_count, error_count, duration);
        }

        Ok(())
    }
}

#[async_trait]
impl CacheWarmingStrategy for PredictiveWarming {
    async fn warm(&self, manager: &MultiLayerCacheManager) -> ActorCoreResult<()> {
        let start_time = Instant::now();
        
        // Set warming flag
        {
            let mut is_warming = self.is_warming.write().await;
            *is_warming = true;
        }

        // Get keys to warm based on access patterns
        let keys_to_warm = self.get_keys_to_warm().await;
        
        if keys_to_warm.is_empty() {
            debug!("No keys to warm based on access patterns");
            return Ok(());
        }

        // Warm the predicted data
        self.warm_predicted_data(manager, keys_to_warm).await?;

        let duration = start_time.elapsed();
        
        // Clear warming flag
        {
            let mut is_warming = self.is_warming.write().await;
            *is_warming = false;
        }

        info!("Predictive cache warming completed in {:?}", duration);

        Ok(())
    }

    fn get_stats(&self) -> CacheWarmingStats {
        CacheWarmingStats::new()
    }

    fn is_warming(&self) -> bool {
        // This is a simplified implementation
        // In a real implementation, you'd check the actual warming state
        false
    }

    async fn stop(&self) -> ActorCoreResult<()> {
        let mut is_warming = self.is_warming.write().await;
        *is_warming = false;
        Ok(())
    }
}

/// Scheduled warming strategy that warms cache at regular intervals.
pub struct ScheduledWarming {
    /// The underlying warming strategy
    strategy: Arc<dyn CacheWarmingStrategy>,
    /// Warming interval
    interval: Duration,
    /// Statistics
    stats: Arc<RwLock<CacheWarmingStats>>,
    /// Whether warming is in progress
    is_warming: Arc<RwLock<bool>>,
    /// Whether the warming task is running
    is_running: Arc<RwLock<bool>>,
}

impl ScheduledWarming {
    /// Create a new scheduled warming strategy.
    pub fn new(strategy: Arc<dyn CacheWarmingStrategy>, interval: Duration) -> Self {
        Self {
            strategy,
            interval,
            stats: Arc::new(RwLock::new(CacheWarmingStats::new())),
            is_warming: Arc::new(RwLock::new(false)),
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start the scheduled warming task.
    pub async fn start(&self, manager: MultiLayerCacheManager) -> ActorCoreResult<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            return Ok(());
        }
        *is_running = true;
        drop(is_running);

        let strategy = Arc::clone(&self.strategy);
        let interval = self.interval;
        let stats = Arc::clone(&self.stats);
        let is_warming = Arc::clone(&self.is_warming);

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                // Check if we should stop
                {
                    let is_running = is_warming.read().await;
                    if !*is_running {
                        break;
                    }
                }

                // Perform warming
                {
                    let mut is_warming_guard = is_warming.write().await;
                    *is_warming_guard = true;
                }

                let start_time = Instant::now();
                match strategy.warm(&manager).await {
                    Ok(_) => {
                        let duration = start_time.elapsed();
                        let mut stats_guard = stats.write().await;
                        stats_guard.record_scheduled_warming(duration);
                    }
                    Err(e) => {
                        error!("Scheduled cache warming failed: {}", e);
                    }
                }

                {
                    let mut is_warming_guard = is_warming.write().await;
                    *is_warming_guard = false;
                }
            }
        });

        Ok(())
    }

    /// Stop the scheduled warming task.
    pub async fn stop(&self) -> ActorCoreResult<()> {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        Ok(())
    }
}

#[async_trait]
impl CacheWarmingStrategy for ScheduledWarming {
    async fn warm(&self, manager: &MultiLayerCacheManager) -> ActorCoreResult<()> {
        self.strategy.warm(manager).await
    }

    fn get_stats(&self) -> CacheWarmingStats {
        CacheWarmingStats::new()
    }

    fn is_warming(&self) -> bool {
        // This is a simplified implementation
        // In a real implementation, you'd check the actual warming state
        false
    }

    async fn stop(&self) -> ActorCoreResult<()> {
        self.strategy.stop().await
    }
}

/// Cache warming manager that coordinates multiple warming strategies.
pub struct CacheWarmingManager {
    /// Warming strategies
    strategies: Vec<Arc<dyn CacheWarmingStrategy>>,
    /// Statistics
    stats: Arc<RwLock<CacheWarmingStats>>,
    /// Whether warming is in progress
    is_warming: Arc<RwLock<bool>>,
}

impl CacheWarmingManager {
    /// Create a new cache warming manager.
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
            stats: Arc::new(RwLock::new(CacheWarmingStats::new())),
            is_warming: Arc::new(RwLock::new(false)),
        }
    }

    /// Add a warming strategy.
    pub fn add_strategy(&mut self, strategy: Arc<dyn CacheWarmingStrategy>) {
        self.strategies.push(strategy);
    }

    /// Warm the cache using all strategies.
    pub async fn warm_all(&self, manager: &MultiLayerCacheManager) -> ActorCoreResult<()> {
        let start_time = Instant::now();
        
        // Set warming flag
        {
            let mut is_warming = self.is_warming.write().await;
            *is_warming = true;
        }

        let mut total_success_count = 0;
        let mut total_error_count = 0;

        // Execute all warming strategies
        for strategy in &self.strategies {
            match strategy.warm(manager).await {
                Ok(_) => {
                    total_success_count += 1;
                }
                Err(e) => {
                    total_error_count += 1;
                    error!("Cache warming strategy failed: {}", e);
                }
            }
        }

        let duration = start_time.elapsed();
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.record_warming_operation(total_success_count, total_error_count, duration);
        }

        // Clear warming flag
        {
            let mut is_warming = self.is_warming.write().await;
            *is_warming = false;
        }

        info!(
            "All cache warming strategies completed: {} successes, {} errors, took {:?}",
            total_success_count, total_error_count, duration
        );

        Ok(())
    }

    /// Get the warming statistics.
    pub async fn get_stats(&self) -> CacheWarmingStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Check if warming is in progress.
    pub async fn is_warming(&self) -> bool {
        let is_warming = self.is_warming.read().await;
        *is_warming
    }

    /// Stop all warming strategies.
    pub async fn stop_all(&self) -> ActorCoreResult<()> {
        for strategy in &self.strategies {
            if let Err(e) = strategy.stop().await {
                error!("Failed to stop warming strategy: {}", e);
            }
        }

        let mut is_warming = self.is_warming.write().await;
        *is_warming = false;

        Ok(())
    }
}

impl Default for CacheWarmingManager {
    fn default() -> Self {
        Self::new()
    }
}