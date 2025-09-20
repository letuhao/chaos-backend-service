//! Cache implementations for the Actor Core system.
//!
//! This module contains the concrete implementations of the cache trait
//! including in-memory cache, distributed cache, and cache warming.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::warn;

use crate::interfaces::*;
use crate::metrics::CacheStats;
use crate::ActorCoreResult;

pub mod multi_layer;
pub mod optimized;

/// Lock-free in-memory cache implementation using DashMap.
pub struct LockFreeInMemoryCache {
    storage: Arc<dashmap::DashMap<String, CacheEntry>>,
    default_ttl: u64,
    max_entries: usize,
    metrics: Arc<parking_lot::RwLock<CacheStats>>,
}

impl LockFreeInMemoryCache {
    pub fn new(max_entries: usize, default_ttl: u64) -> Self {
        Self {
            storage: Arc::new(dashmap::DashMap::new()),
            default_ttl,
            max_entries,
            metrics: Arc::new(parking_lot::RwLock::new(CacheStats::default())),
        }
    }

    fn is_expired(&self, entry: &CacheEntry) -> bool {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(entry.created_at).as_secs();
        elapsed >= entry.ttl
    }

    fn evict_if_needed(&self) {
        if self.storage.len() <= self.max_entries { return; }
        // Remove oldest entries until under max_entries
        let mut entries: Vec<(String, std::time::Instant)> = Vec::new();
        for item in self.storage.iter() {
            entries.push((item.key().clone(), item.created_at));
        }
        entries.sort_by_key(|(_, created_at)| *created_at);
        let mut to_remove = self.storage.len().saturating_sub(self.max_entries);
        for (k, _) in entries {
            if to_remove == 0 { break; }
            self.storage.remove(&k);
            to_remove -= 1;
        }
    }
}

#[async_trait]
impl Cache for LockFreeInMemoryCache {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        if let Some(entry) = self.storage.get(key) {
            if self.is_expired(&entry) { return None; }
            let mut metrics = self.metrics.write();
            metrics.hits += 1;
            return Some(entry.value.clone());
        }
        let mut metrics = self.metrics.write();
        metrics.misses += 1;
        None
    }

    fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()> {
        let ttl = ttl.unwrap_or(self.default_ttl);
        let entry = CacheEntry { value, created_at: std::time::Instant::now(), ttl };
        self.storage.insert(key, entry);
        self.evict_if_needed();
        let mut metrics = self.metrics.write();
        metrics.sets += 1;
        Ok(())
    }

    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        self.storage.remove(key);
        let mut metrics = self.metrics.write();
        metrics.deletes += 1;
        Ok(())
    }

    fn clear(&self) -> ActorCoreResult<()> {
        self.storage.clear();
        let mut metrics = self.metrics.write();
        metrics.sets = 0; metrics.hits = 0; metrics.misses = 0; metrics.deletes = 0;
        Ok(())
    }

    fn get_stats(&self) -> CacheStats {
        let mut metrics = self.metrics.write();
        metrics.memory_usage = (self.storage.len() * 1024) as u64; // rough estimate
        metrics.clone()
    }
}

/// InMemoryCache is a simple in-memory cache implementation.
pub struct InMemoryCache {
    /// The actual cache storage
    storage: Arc<parking_lot::RwLock<HashMap<String, CacheEntry>>>,
    /// Maximum number of entries
    #[allow(dead_code)]
    max_entries: usize,
    /// Default TTL in seconds
    default_ttl: u64,
    /// Metrics for performance monitoring
    metrics: Arc<parking_lot::RwLock<CacheStats>>,
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
            storage: Arc::new(parking_lot::RwLock::new(HashMap::new())),
            max_entries,
            default_ttl,
            metrics: Arc::new(parking_lot::RwLock::new(CacheStats::default())),
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
        let mut storage = self.storage.write();
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
        let mut storage = self.storage.write();
        
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
        let storage = self.storage.read();
        
        if let Some(entry) = storage.get(key) {
            if self.is_expired(entry) {
                // Entry is expired, but we can't remove it here
                // because we only have a read lock
                return None;
            }
            
            // Update metrics
            let mut metrics = self.metrics.write();
            metrics.hits += 1;
            
            return Some(entry.value.clone());
        }
        
        // Update metrics
        let mut metrics = self.metrics.write();
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
        
        let mut storage = self.storage.write();
        storage.insert(key, entry);
        
        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.sets += 1;
        
        Ok(())
    }

    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        let mut storage = self.storage.write();
        
        if storage.remove(key).is_some() {
            // Update metrics
            let mut metrics = self.metrics.write();
            metrics.deletes += 1;
            
            Ok(())
        } else {
            Err(crate::ActorCoreError::CacheError(
                format!("Key not found: {}", key)
            ))
        }
    }

    fn clear(&self) -> ActorCoreResult<()> {
        let mut storage = self.storage.write();
        storage.clear();
        
        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.sets = 0;
        metrics.hits = 0;
        metrics.misses = 0;
        metrics.deletes = 0;
        
        Ok(())
    }

    fn get_stats(&self) -> CacheStats {
        let storage = self.storage.read();
        let mut metrics = self.metrics.write();
        
        // Update memory usage
        metrics.memory_usage = (storage.len() * 1024) as u64; // Rough estimate
        
        metrics.clone()
    }
}

/// DistributedCache is a distributed cache implementation using Redis.
#[cfg(feature = "redis-cache")]
pub struct DistributedCache {
    /// Redis client
    #[allow(dead_code)]
    redis_client: Arc<redis::Client>,
    /// Default TTL in seconds
    #[allow(dead_code)]
    default_ttl: u64,
    /// Metrics for performance monitoring
    #[allow(dead_code)]
    metrics: Arc<parking_lot::RwLock<CacheStats>>,
}

#[cfg(feature = "redis-cache")]
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
            metrics: Arc::new(parking_lot::RwLock::new(CacheStats::default())),
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

#[cfg(feature = "redis-cache")]
#[async_trait]
impl Cache for DistributedCache {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        // For now, we'll use a blocking approach since the Cache trait is not async
        // In a real implementation, this would need to be async or use a different approach
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            match self.get_connection().await {
                Ok(mut conn) => {
                    let result: Result<Option<String>, redis::RedisError> = redis::cmd("GET")
                        .arg(key)
                        .query_async(&mut conn)
                        .await;
                    
                    match result {
                        Ok(Some(value)) => {
                            match serde_json::from_str(&value) {
                                Ok(json_value) => {
                                    let mut metrics = self.metrics.write();
                                    metrics.hits += 1;
                                    Some(json_value)
                                }
                                Err(_) => {
                                    let mut metrics = self.metrics.write();
                                    metrics.misses += 1;
                                    None
                                }
                            }
                        }
                        Ok(None) => {
                            let mut metrics = self.metrics.write();
                            metrics.misses += 1;
                            None
                        }
                        Err(e) => {
                            warn!("Redis GET error: {}", e);
                            let mut metrics = self.metrics.write();
                            metrics.misses += 1;
                            None
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to get Redis connection: {}", e);
                    let mut metrics = self.metrics.write();
                    metrics.misses += 1;
                    None
                }
            }
        })
    }

    fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()> {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            match self.get_connection().await {
                Ok(mut conn) => {
                    let json_str = serde_json::to_string(&value)
                        .map_err(|e| crate::ActorCoreError::CacheError(
                            format!("Failed to serialize value: {}", e)
                        ))?;
                    
                    let ttl_seconds = ttl.unwrap_or(self.default_ttl);
                    let result: Result<(), redis::RedisError> = if ttl_seconds > 0 {
                        redis::cmd("SETEX")
                            .arg(&key)
                            .arg(ttl_seconds)
                            .arg(&json_str)
                            .query_async(&mut conn)
                            .await
                    } else {
                        redis::cmd("SET")
                            .arg(&key)
                            .arg(&json_str)
                            .query_async(&mut conn)
                            .await
                    };
                    
                    match result {
                        Ok(()) => {
                            let mut metrics = self.metrics.write();
                            metrics.sets += 1;
                            Ok(())
                        }
                        Err(e) => {
                            warn!("Redis SET error: {}", e);
                            Err(crate::ActorCoreError::CacheError(
                                format!("Failed to set cache value: {}", e)
                            ))
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to get Redis connection: {}", e);
                    Err(crate::ActorCoreError::CacheError(
                        format!("Failed to get Redis connection: {}", e)
                    ))
                }
            }
        })
    }

    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            match self.get_connection().await {
                Ok(mut conn) => {
                    let result: Result<u32, redis::RedisError> = redis::cmd("DEL")
                        .arg(key)
                        .query_async(&mut conn)
                        .await;
                    
                    match result {
                        Ok(_) => {
                            let mut metrics = self.metrics.write();
                            metrics.deletes += 1;
                            Ok(())
                        }
                        Err(e) => {
                            warn!("Redis DEL error: {}", e);
                            Err(crate::ActorCoreError::CacheError(
                                format!("Failed to delete cache value: {}", e)
                            ))
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to get Redis connection: {}", e);
                    Err(crate::ActorCoreError::CacheError(
                        format!("Failed to get Redis connection: {}", e)
                    ))
                }
            }
        })
    }

    fn clear(&self) -> ActorCoreResult<()> {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            match self.get_connection().await {
                Ok(mut conn) => {
                    let result: Result<(), redis::RedisError> = redis::cmd("FLUSHDB")
                        .query_async(&mut conn)
                        .await;
                    
                    match result {
                        Ok(()) => {
                    let mut metrics = self.metrics.write();
                    metrics.sets += 1; // Using sets as a proxy for clears
                            Ok(())
                        }
                        Err(e) => {
                            warn!("Redis FLUSHDB error: {}", e);
                            Err(crate::ActorCoreError::CacheError(
                                format!("Failed to clear cache: {}", e)
                            ))
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to get Redis connection: {}", e);
                    Err(crate::ActorCoreError::CacheError(
                        format!("Failed to get Redis connection: {}", e)
                    ))
                }
            }
        })
    }

    fn get_stats(&self) -> CacheStats {
        let metrics = self.metrics.read();
        metrics.clone()
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
    metrics: Arc<parking_lot::RwLock<CacheStats>>,
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
            metrics: Arc::new(parking_lot::RwLock::new(CacheStats::default())),
        }
    }
}

#[async_trait]
impl Cache for MultiLayerCache {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        // Try L1 cache first
        if let Some(value) = self.l1_cache.get(key) {
            let mut metrics = self.metrics.write();
            metrics.hits += 1;
            return Some(value);
        }
        
        // Try L2 cache
        if let Some(value) = self.l2_cache.get(key) {
            // Store in L1 for faster access next time
            if let Err(e) = self.l1_cache.set(key.to_string(), value.clone(), Some(300)) {
                warn!("Failed to store in L1 cache: {}", e);
            }
            
            let mut metrics = self.metrics.write();
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
            
            let mut metrics = self.metrics.write();
            metrics.hits += 1;
            return Some(value);
        }
        
        // Cache miss
        let mut metrics = self.metrics.write();
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
        
        let mut metrics = self.metrics.write();
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
        
        let mut metrics = self.metrics.write();
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
        
        let mut metrics = self.metrics.write();
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

    /// Create a new lock-free in-memory cache instance.
    pub fn create_lock_free_in_memory_cache(max_entries: usize, default_ttl: u64) -> Arc<dyn Cache> {
        Arc::new(LockFreeInMemoryCache::new(max_entries, default_ttl))
    }

    /// Create a new distributed cache instance.
    #[cfg(feature = "redis-cache")]
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

    /// Create a sensible default multi-layer cache.
    /// L1: lock-free in-memory; L2: in-memory; L3: distributed if REDIS url is provided, otherwise in-memory.
    /// TODO: Load cache configuration from config files instead of hardcoded values
    pub fn create_default_multi_layer_cache() -> Arc<dyn Cache> {
        // TODO: Load these values from configuration
        let l1_max_entries = 50_000;
        let l1_ttl = 300;
        let l2_max_entries = 200_000;
        let l2_ttl = 600;
        let l3_max_entries = 500_000;
        let l3_ttl = 1800;
        
        let l1 = Self::create_lock_free_in_memory_cache(l1_max_entries, l1_ttl);
        let l2 = Self::create_in_memory_cache(l2_max_entries, l2_ttl);
        let l3 = if let Ok(_url) = std::env::var("ACTOR_CORE_REDIS_URL") {
            #[cfg(feature = "redis-cache")]
            {
                if let Ok(redis_cache) = Self::create_distributed_cache(&_url, l3_ttl) {
                    redis_cache
                } else {
                    Self::create_in_memory_cache(l3_max_entries, l3_ttl)
                }
            }
            #[cfg(not(feature = "redis-cache"))]
            {
                Self::create_in_memory_cache(l3_max_entries, l3_ttl)
            }
        } else {
            Self::create_in_memory_cache(l3_max_entries, l3_ttl)
        };
        Self::create_multi_layer_cache(l1, l2, l3)
    }
}