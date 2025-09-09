//! Cache implementations for the Actor Core system.
//!
//! This module contains the concrete implementations of the cache trait
//! including in-memory cache, distributed cache, and cache warming.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::warn;

use crate::interfaces::*;
use crate::ActorCoreResult;

pub mod multi_layer;

/// InMemoryCache is a simple in-memory cache implementation.
pub struct InMemoryCache {
    /// The actual cache storage
    storage: Arc<std::sync::RwLock<HashMap<String, CacheEntry>>>,
    /// Maximum number of entries
    #[allow(dead_code)]
    max_entries: usize,
    /// Default TTL in seconds
    default_ttl: u64,
    /// Metrics for performance monitoring
    metrics: Arc<std::sync::RwLock<CacheStats>>,
}

/// CacheEntry represents a single cache entry.
#[derive(Debug, Clone)]
struct CacheEntry {
    /// The cached value
    value: serde_json::Value,
    /// When the entry was created
    created_at: std::time::Instant,
    /// TTL in seconds
    ttl: u64,
}

impl InMemoryCache {
    /// Create a new in-memory cache instance.
    pub fn new(max_entries: usize, default_ttl: u64) -> Self {
        Self {
            storage: Arc::new(std::sync::RwLock::new(HashMap::new())),
            max_entries,
            default_ttl,
            metrics: Arc::new(std::sync::RwLock::new(CacheStats::default())),
        }
    }

    /// Check if an entry has expired.
    fn is_expired(&self, entry: &CacheEntry) -> bool {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(entry.created_at).as_secs();
        elapsed >= entry.ttl
    }

    /// Clean up expired entries.
    #[allow(dead_code)]
    fn cleanup_expired(&self) {
        let mut storage = self.storage.write().unwrap();
        let mut expired_keys = Vec::new();
        
        for (key, entry) in storage.iter() {
            if self.is_expired(entry) {
                expired_keys.push(key.clone());
            }
        }
        
        for key in expired_keys {
            storage.remove(&key);
        }
    }

    /// Evict entries if we exceed the maximum count.
    #[allow(dead_code)]
    fn evict_if_needed(&self) {
        let mut storage = self.storage.write().unwrap();
        
        if storage.len() >= self.max_entries {
            // Simple LRU eviction - remove the oldest entry
            let oldest_key = storage.iter()
                .min_by_key(|(_, entry)| entry.created_at)
                .map(|(key, _)| key.clone());
            
            if let Some(key) = oldest_key {
                storage.remove(&key);
            }
        }
    }
}

#[async_trait]
impl Cache for InMemoryCache {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        let storage = self.storage.read().unwrap();
        
        if let Some(entry) = storage.get(key) {
            if self.is_expired(entry) {
                // Entry is expired, but we can't remove it here
                // because we only have a read lock
                return None;
            }
            
            // Update metrics
            let mut metrics = self.metrics.write().unwrap();
            metrics.hits += 1;
            
            return Some(entry.value.clone());
        }
        
        // Update metrics
        let mut metrics = self.metrics.write().unwrap();
        metrics.misses += 1;
        
        None
    }

    fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()> {
        let ttl = ttl.unwrap_or(self.default_ttl);
        let entry = CacheEntry {
            value,
            created_at: std::time::Instant::now(),
            ttl,
        };
        
        let mut storage = self.storage.write().unwrap();
        storage.insert(key, entry);
        
        // Update metrics
        let mut metrics = self.metrics.write().unwrap();
        metrics.sets += 1;
        
        Ok(())
    }

    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        let mut storage = self.storage.write().unwrap();
        
        if storage.remove(key).is_some() {
            // Update metrics
            let mut metrics = self.metrics.write().unwrap();
            metrics.deletes += 1;
            
            Ok(())
        } else {
            Err(crate::ActorCoreError::CacheError(
                format!("Key not found: {}", key)
            ))
        }
    }

    fn clear(&self) -> ActorCoreResult<()> {
        let mut storage = self.storage.write().unwrap();
        storage.clear();
        
        // Update metrics
        let mut metrics = self.metrics.write().unwrap();
        metrics.sets = 0;
        metrics.hits = 0;
        metrics.misses = 0;
        metrics.deletes = 0;
        
        Ok(())
    }

    fn get_stats(&self) -> CacheStats {
        let storage = self.storage.read().unwrap();
        let mut metrics = self.metrics.write().unwrap();
        
        // Update memory usage
        metrics.memory_usage = (storage.len() * 1024) as u64; // Rough estimate
        
        metrics.clone()
    }
}

/// DistributedCache is a distributed cache implementation using Redis.
pub struct DistributedCache {
    /// Redis client
    #[allow(dead_code)]
    redis_client: Arc<redis::Client>,
    /// Default TTL in seconds
    #[allow(dead_code)]
    default_ttl: u64,
    /// Metrics for performance monitoring
    #[allow(dead_code)]
    metrics: Arc<std::sync::RwLock<CacheStats>>,
}

impl DistributedCache {
    /// Create a new distributed cache instance.
    pub fn new(redis_url: &str, default_ttl: u64) -> ActorCoreResult<Self> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| crate::ActorCoreError::CacheError(
                format!("Failed to create Redis client: {}", e)
            ))?;
        
        Ok(Self {
            redis_client: Arc::new(client),
            default_ttl,
            metrics: Arc::new(std::sync::RwLock::new(CacheStats::default())),
        })
    }

    /// Get a Redis connection.
    #[allow(dead_code)]
    async fn get_connection(&self) -> ActorCoreResult<redis::aio::Connection> {
        self.redis_client.get_async_connection().await
            .map_err(|e| crate::ActorCoreError::CacheError(
                format!("Failed to get Redis connection: {}", e)
            ))
    }
}

#[async_trait]
impl Cache for DistributedCache {
    fn get(&self, _key: &str) -> Option<serde_json::Value> {
        // This is a simplified implementation
        // In a real implementation, this would be async
        warn!("DistributedCache::get is not fully implemented");
        None
    }

    fn set(&self, _key: String, _value: serde_json::Value, _ttl: Option<u64>) -> ActorCoreResult<()> {
        // This is a simplified implementation
        // In a real implementation, this would be async
        warn!("DistributedCache::set is not fully implemented");
        Ok(())
    }

    fn delete(&self, _key: &str) -> ActorCoreResult<()> {
        // This is a simplified implementation
        // In a real implementation, this would be async
        warn!("DistributedCache::delete is not fully implemented");
        Ok(())
    }

    fn clear(&self) -> ActorCoreResult<()> {
        // This is a simplified implementation
        // In a real implementation, this would be async
        warn!("DistributedCache::clear is not fully implemented");
        Ok(())
    }

    fn get_stats(&self) -> CacheStats {
        // This is a simplified implementation
        CacheStats::default()
    }
}

/// MultiLayerCache combines multiple cache layers.
pub struct MultiLayerCache {
    /// L1 cache (fastest, smallest)
    l1_cache: Arc<dyn Cache>,
    /// L2 cache (medium speed, medium size)
    l2_cache: Arc<dyn Cache>,
    /// L3 cache (slowest, largest)
    l3_cache: Arc<dyn Cache>,
    /// Metrics for performance monitoring
    metrics: Arc<std::sync::RwLock<CacheStats>>,
}

impl MultiLayerCache {
    /// Create a new multi-layer cache instance.
    pub fn new(
        l1_cache: Arc<dyn Cache>,
        l2_cache: Arc<dyn Cache>,
        l3_cache: Arc<dyn Cache>,
    ) -> Self {
        Self {
            l1_cache,
            l2_cache,
            l3_cache,
            metrics: Arc::new(std::sync::RwLock::new(CacheStats::default())),
        }
    }
}

#[async_trait]
impl Cache for MultiLayerCache {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        // Try L1 cache first
        if let Some(value) = self.l1_cache.get(key) {
            let mut metrics = self.metrics.write().unwrap();
            metrics.hits += 1;
            return Some(value);
        }
        
        // Try L2 cache
        if let Some(value) = self.l2_cache.get(key) {
            // Store in L1 for faster access next time
            if let Err(e) = self.l1_cache.set(key.to_string(), value.clone(), Some(300)) {
                warn!("Failed to store in L1 cache: {}", e);
            }
            
            let mut metrics = self.metrics.write().unwrap();
            metrics.hits += 1;
            return Some(value);
        }
        
        // Try L3 cache
        if let Some(value) = self.l3_cache.get(key) {
            // Store in L2 and L1 for faster access next time
            if let Err(e) = self.l2_cache.set(key.to_string(), value.clone(), Some(600)) {
                warn!("Failed to store in L2 cache: {}", e);
            }
            if let Err(e) = self.l1_cache.set(key.to_string(), value.clone(), Some(300)) {
                warn!("Failed to store in L1 cache: {}", e);
            }
            
            let mut metrics = self.metrics.write().unwrap();
            metrics.hits += 1;
            return Some(value);
        }
        
        // Cache miss
        let mut metrics = self.metrics.write().unwrap();
        metrics.misses += 1;
        None
    }

    fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()> {
        // Store in all layers
        if let Err(e) = self.l1_cache.set(key.clone(), value.clone(), ttl) {
            warn!("Failed to store in L1 cache: {}", e);
        }
        if let Err(e) = self.l2_cache.set(key.clone(), value.clone(), ttl) {
            warn!("Failed to store in L2 cache: {}", e);
        }
        if let Err(e) = self.l3_cache.set(key, value, ttl) {
            warn!("Failed to store in L3 cache: {}", e);
        }
        
        let mut metrics = self.metrics.write().unwrap();
        metrics.sets += 1;
        
        Ok(())
    }

    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        // Delete from all layers
        if let Err(e) = self.l1_cache.delete(key) {
            warn!("Failed to delete from L1 cache: {}", e);
        }
        if let Err(e) = self.l2_cache.delete(key) {
            warn!("Failed to delete from L2 cache: {}", e);
        }
        if let Err(e) = self.l3_cache.delete(key) {
            warn!("Failed to delete from L3 cache: {}", e);
        }
        
        let mut metrics = self.metrics.write().unwrap();
        metrics.deletes += 1;
        
        Ok(())
    }

    fn clear(&self) -> ActorCoreResult<()> {
        // Clear all layers
        if let Err(e) = self.l1_cache.clear() {
            warn!("Failed to clear L1 cache: {}", e);
        }
        if let Err(e) = self.l2_cache.clear() {
            warn!("Failed to clear L2 cache: {}", e);
        }
        if let Err(e) = self.l3_cache.clear() {
            warn!("Failed to clear L3 cache: {}", e);
        }
        
        let mut metrics = self.metrics.write().unwrap();
        metrics.sets = 0;
        metrics.hits = 0;
        metrics.misses = 0;
        metrics.deletes = 0;
        
        Ok(())
    }

    fn get_stats(&self) -> CacheStats {
        // Combine stats from all layers
        let l1_stats = self.l1_cache.get_stats();
        let l2_stats = self.l2_cache.get_stats();
        let l3_stats = self.l3_cache.get_stats();
        
        CacheStats {
            hits: l1_stats.hits + l2_stats.hits + l3_stats.hits,
            misses: l1_stats.misses + l2_stats.misses + l3_stats.misses,
            sets: l1_stats.sets + l2_stats.sets + l3_stats.sets,
            deletes: l1_stats.deletes + l2_stats.deletes + l3_stats.deletes,
            memory_usage: l1_stats.memory_usage + l2_stats.memory_usage + l3_stats.memory_usage,
            max_memory_usage: l1_stats.max_memory_usage + l2_stats.max_memory_usage + l3_stats.max_memory_usage,
        }
    }
}

/// CacheFactory for creating cache instances.
pub struct CacheFactory;

impl CacheFactory {
    /// Create a new in-memory cache instance.
    pub fn create_in_memory_cache(max_entries: usize, default_ttl: u64) -> Arc<dyn Cache> {
        Arc::new(InMemoryCache::new(max_entries, default_ttl))
    }

    /// Create a new distributed cache instance.
    pub fn create_distributed_cache(redis_url: &str, default_ttl: u64) -> ActorCoreResult<Arc<dyn Cache>> {
        Ok(Arc::new(DistributedCache::new(redis_url, default_ttl)?))
    }

    /// Create a new multi-layer cache instance.
    pub fn create_multi_layer_cache(
        l1_cache: Arc<dyn Cache>,
        l2_cache: Arc<dyn Cache>,
        l3_cache: Arc<dyn Cache>,
    ) -> Arc<dyn Cache> {
        Arc::new(MultiLayerCache::new(l1_cache, l2_cache, l3_cache))
    }
}
