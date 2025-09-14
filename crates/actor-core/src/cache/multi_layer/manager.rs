//! Multi-layer cache manager implementation.
//!
//! This module provides the main MultiLayerCacheManager that coordinates
//! between the three cache layers (L1, L2, L3) and implements the Cache trait.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{warn, error, debug};

use crate::interfaces::Cache;
use crate::ActorCoreResult;
use super::layers::{L1Cache, L2Cache, L3Cache};
use super::metrics::{MultiLayerStats, CacheLayer};
use super::policy::EvictionPolicy;

/// Multi-layer cache manager that coordinates L1, L2, and L3 caches.
pub struct MultiLayerCacheManager {
    /// L1 cache (lock-free in-memory)
    l1_cache: Arc<dyn L1Cache>,
    /// L2 cache (memory-mapped file)
    l2_cache: Arc<dyn L2Cache>,
    /// L3 cache (persistent disk)
    l3_cache: Arc<dyn L3Cache>,
    /// Configuration
    config: MultiLayerConfig,
    /// Statistics
    stats: Arc<RwLock<MultiLayerStats>>,
    /// Background sync task handle
    #[allow(dead_code)]
    sync_handle: Option<tokio::task::JoinHandle<()>>,
}

/// Configuration for the multi-layer cache system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiLayerConfig {
    /// L1 cache settings
    pub l1_max_size: usize,
    pub l1_eviction_policy: EvictionPolicy,
    
    /// L2 cache settings
    pub l2_cache_path: String,
    pub l2_max_size: usize,
    
    /// L3 cache settings
    pub l3_cache_dir: String,
    pub l3_max_size: usize,
    
    /// Sync settings
    pub sync_interval: Duration,
    pub enable_background_sync: bool,
    
    /// Performance settings
    pub enable_metrics: bool,
    pub enable_tracing: bool,
}

impl MultiLayerConfig {
    /// Create a new multi-layer cache configuration.
    pub fn new() -> Self {
        Self {
            l1_max_size: 1000,
            l1_eviction_policy: EvictionPolicy::Lru,
            l2_cache_path: "/tmp/actor_cache_l2".to_string(),
            l2_max_size: 10000,
            l3_cache_dir: "/tmp/actor_cache_l3".to_string(),
            l3_max_size: 100000,
            sync_interval: Duration::from_secs(60),
            enable_background_sync: true,
            enable_metrics: true,
            enable_tracing: true,
        }
    }

    /// Validate the configuration.
    pub fn validate(&self) -> ActorCoreResult<()> {
        if self.l1_max_size == 0 {
            return Err(crate::ActorCoreError::ConfigurationError(
                "L1 max size must be greater than 0".to_string()
            ));
        }

        if self.l2_max_size == 0 {
            return Err(crate::ActorCoreError::ConfigurationError(
                "L2 max size must be greater than 0".to_string()
            ));
        }

        if self.l3_max_size == 0 {
            return Err(crate::ActorCoreError::ConfigurationError(
                "L3 max size must be greater than 0".to_string()
            ));
        }

        if self.l2_cache_path.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "L2 cache path cannot be empty".to_string()
            ));
        }

        if self.l3_cache_dir.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "L3 cache directory cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

impl Default for MultiLayerConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiLayerCacheManager {
    /// Create a new multi-layer cache manager.
    pub fn new(
        l1_cache: Arc<dyn L1Cache>,
        l2_cache: Arc<dyn L2Cache>,
        l3_cache: Arc<dyn L3Cache>,
        config: MultiLayerConfig,
    ) -> Self {
        let stats = Arc::new(RwLock::new(MultiLayerStats::new()));
        
        let mut manager = Self {
            l1_cache,
            l2_cache,
            l3_cache,
            config,
            stats,
            sync_handle: None,
        };

        // Start background sync if enabled
        if manager.config.enable_background_sync {
            manager.start_background_sync();
        }

        manager
    }

    /// Start background synchronization task.
    fn start_background_sync(&mut self) {
        let l2_cache = Arc::clone(&self.l2_cache);
        let l3_cache = Arc::clone(&self.l3_cache);
        let sync_interval = self.config.sync_interval;
        let stats = Arc::clone(&self.stats);

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(sync_interval);
            
            loop {
                interval.tick().await;
                
                if let Err(e) = Self::sync_caches(&l2_cache, &l3_cache, &stats).await {
                    error!("Background sync failed: {}", e);
                }
            }
        });

        self.sync_handle = Some(handle);
    }

    /// Synchronize caches between layers.
    async fn sync_caches(
        l2_cache: &Arc<dyn L2Cache>,
        _l3_cache: &Arc<dyn L3Cache>,
        stats: &Arc<RwLock<MultiLayerStats>>,
    ) -> ActorCoreResult<()> {
        debug!("Starting cache synchronization");
        
        // Sync L2 to L3
        if let Err(e) = l2_cache.sync().await {
            warn!("Failed to sync L2 cache: {}", e);
        }

        // Update statistics
        {
            let mut stats = stats.write().await;
            stats.update_operation(CacheLayer::L2, true, Duration::from_micros(0));
        }

        debug!("Cache synchronization completed");
        Ok(())
    }

    /// Get a value from the cache hierarchy.
    #[allow(dead_code)]
    async fn get_hierarchical(&self, key: &str) -> ActorCoreResult<Option<serde_json::Value>> {
        let start_time = Instant::now();

        // Try L1 first (fastest)
        if let Some(value) = self.l1_cache.get(key) {
            let response_time = start_time.elapsed();
            {
                let mut stats = self.stats.write().await;
                stats.update_operation(CacheLayer::L1, true, response_time);
            }
            return Ok(Some(value));
        }

        // Try L2 (medium speed)
        if let Some(value) = self.l2_cache.get(key).await? {
            let response_time = start_time.elapsed();
            {
                let mut stats = self.stats.write().await;
                stats.update_operation(CacheLayer::L2, true, response_time);
            }
            
            // Promote to L1 for faster future access
            if let Err(e) = self.l1_cache.set(key.to_string(), value.clone(), None) {
                warn!("Failed to promote value to L1: {}", e);
            }
            
            return Ok(Some(value));
        }

        // Try L3 (slowest)
        if let Some(value) = self.l3_cache.get(key).await? {
            let response_time = start_time.elapsed();
            {
                let mut stats = self.stats.write().await;
                stats.update_operation(CacheLayer::L3, true, response_time);
            }
            
            // Promote to L2 and L1 for faster future access
            if let Err(e) = self.l2_cache.set(key.to_string(), value.clone(), None).await {
                warn!("Failed to promote value to L2: {}", e);
            }
            
            if let Err(e) = self.l1_cache.set(key.to_string(), value.clone(), None) {
                warn!("Failed to promote value to L1: {}", e);
            }
            
            return Ok(Some(value));
        }

        // Cache miss
        let response_time = start_time.elapsed();
        {
            let mut stats = self.stats.write().await;
            stats.update_operation(CacheLayer::L1, false, response_time);
        }

        Ok(None)
    }

    /// Set a value in the cache hierarchy.
    #[allow(dead_code)]
    async fn set_hierarchical(
        &self,
        key: String,
        value: serde_json::Value,
        ttl: Option<u64>,
    ) -> ActorCoreResult<()> {
        let start_time = Instant::now();

        // Set in all layers
        if let Err(e) = self.l1_cache.set(key.clone(), value.clone(), ttl) {
            warn!("Failed to set value in L1: {}", e);
        }

        if let Err(e) = self.l2_cache.set(key.clone(), value.clone(), ttl).await {
            warn!("Failed to set value in L2: {}", e);
        }

        if let Err(e) = self.l3_cache.set(key, value, ttl).await {
            warn!("Failed to set value in L3: {}", e);
        }

        let response_time = start_time.elapsed();
        {
            let mut stats = self.stats.write().await;
            stats.update_operation(CacheLayer::L1, true, response_time);
        }

        Ok(())
    }

    /// Delete a value from all cache layers.
    #[allow(dead_code)]
    async fn delete_hierarchical(&self, key: &str) -> ActorCoreResult<()> {
        let start_time = Instant::now();

        // Delete from all layers
        if let Err(e) = self.l1_cache.delete(key) {
            warn!("Failed to delete value from L1: {}", e);
        }

        if let Err(e) = self.l2_cache.delete(key).await {
            warn!("Failed to delete value from L2: {}", e);
        }

        if let Err(e) = self.l3_cache.delete(key).await {
            warn!("Failed to delete value from L3: {}", e);
        }

        let response_time = start_time.elapsed();
        {
            let mut stats = self.stats.write().await;
            stats.update_operation(CacheLayer::L1, true, response_time);
        }

        Ok(())
    }

    /// Get comprehensive cache statistics.
    pub async fn get_comprehensive_stats(&self) -> MultiLayerStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Get cache health status.
    pub async fn get_health_status(&self) -> CacheHealthStatus {
        let stats = self.stats.read().await;
        
        let l1_health = if self.l1_cache.memory_usage() > self.l1_cache.max_capacity() as u64 * 90 / 100 {
            LayerHealth::Warning
        } else {
            LayerHealth::Healthy
        };

        let l2_health = if self.l2_cache.memory_usage() > self.l2_cache.max_capacity() as u64 * 90 / 100 {
            LayerHealth::Warning
        } else {
            LayerHealth::Healthy
        };

        let l3_health = if self.l3_cache.disk_usage() > self.l3_cache.max_capacity() as u64 * 90 / 100 {
            LayerHealth::Warning
        } else {
            LayerHealth::Healthy
        };

        let overall_health = match (l1_health, l2_health, l3_health) {
            (LayerHealth::Healthy, LayerHealth::Healthy, LayerHealth::Healthy) => OverallHealth::Healthy,
            (LayerHealth::Warning, _, _) | (_, LayerHealth::Warning, _) | (_, _, LayerHealth::Warning) => OverallHealth::Warning,
            _ => OverallHealth::Warning, // Default to warning for mixed states
        };

        CacheHealthStatus {
            overall: overall_health,
            l1_health,
            l2_health,
            l3_health,
            efficiency_score: stats.efficiency_score(),
            total_operations: stats.total_operations,
            hit_ratio: stats.hit_ratio,
        }
    }
}

/// Cache health status for monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheHealthStatus {
    /// Overall cache health
    pub overall: OverallHealth,
    /// L1 cache health
    pub l1_health: LayerHealth,
    /// L2 cache health
    pub l2_health: LayerHealth,
    /// L3 cache health
    pub l3_health: LayerHealth,
    /// Cache efficiency score (0.0 to 1.0)
    pub efficiency_score: f64,
    /// Total operations performed
    pub total_operations: u64,
    /// Cache hit ratio
    pub hit_ratio: f64,
}

/// Overall cache health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverallHealth {
    /// Cache is healthy
    Healthy,
    /// Cache has warnings
    Warning,
    /// Cache is unhealthy
    Unhealthy,
}

/// Individual layer health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerHealth {
    /// Layer is healthy
    Healthy,
    /// Layer has warnings
    Warning,
    /// Layer is unhealthy
    Unhealthy,
}

impl MultiLayerCacheManager {
    /// Get a reference to the L1 cache.
    pub fn l1_cache(&self) -> &Arc<dyn L1Cache> {
        &self.l1_cache
    }

    /// Get a reference to the L2 cache.
    pub fn l2_cache(&self) -> &Arc<dyn L2Cache> {
        &self.l2_cache
    }

    /// Get a reference to the L3 cache.
    pub fn l3_cache(&self) -> &Arc<dyn L3Cache> {
        &self.l3_cache
    }
}

#[async_trait]
impl Cache for MultiLayerCacheManager {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        // For sync interface, we can only check L1
        self.l1_cache.get(key)
    }

    fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()> {
        // For sync interface, we can only set in L1
        self.l1_cache.set(key, value, ttl)
    }

    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        // For sync interface, we can only delete from L1
        self.l1_cache.delete(key)
    }

    fn clear(&self) -> ActorCoreResult<()> {
        // Clear all layers
        self.l1_cache.clear()?;
        // Note: L2 and L3 clear operations are async, so we can't call them here
        // In a real implementation, you might want to use a different approach
        Ok(())
    }

    fn get_stats(&self) -> crate::metrics::CacheStats {
        let l1_stats = self.l1_cache.get_stats();
        crate::metrics::CacheStats {
            hits: l1_stats.hits,
            misses: l1_stats.misses,
            sets: l1_stats.sets,
            deletes: l1_stats.deletes,
            memory_usage: l1_stats.memory_usage,
            max_memory_usage: l1_stats.max_memory_usage,
        }
    }
}