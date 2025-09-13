//! Optimized cache implementations with micro-optimizations.
//!
//! This module provides high-performance cache implementations using
//! smallvec, fxhash, and atomic operations for hot paths.

use async_trait::async_trait;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{warn, error, debug};

use smallvec::{SmallVec, smallvec};
use fxhash::FxHashMap;
use ahash::AHashMap;

use crate::interfaces::Cache;
use crate::ActorCoreResult;

/// High-performance L1 cache using FxHash and SmallVec.
pub struct OptimizedL1Cache {
    /// Cache storage using FxHash for faster lookups
    storage: Arc<RwLock<FxHashMap<String, CacheEntry>>>,
    /// Maximum cache size
    max_size: usize,
    /// Current size (atomic for thread safety)
    current_size: std::sync::atomic::AtomicUsize,
    /// Hit/miss counters (atomic for thread safety)
    hits: std::sync::atomic::AtomicU64,
    misses: std::sync::atomic::AtomicU64,
}

/// Cache entry with optimized storage.
#[derive(Debug, Clone)]
struct CacheEntry {
    value: serde_json::Value,
    created_at: Instant,
    ttl: Option<Duration>,
}

impl CacheEntry {
    /// Check if the entry is expired.
    fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            self.created_at.elapsed() >= ttl
        } else {
            false
        }
    }
}

impl OptimizedL1Cache {
    /// Create a new optimized L1 cache.
    pub fn new(max_size: usize) -> Self {
        Self {
            storage: Arc::new(RwLock::new(FxHashMap::default())),
            max_size,
            current_size: std::sync::atomic::AtomicUsize::new(0),
            hits: std::sync::atomic::AtomicU64::new(0),
            misses: std::sync::atomic::AtomicU64::new(0),
        }
    }
    
    /// Get cache statistics with atomic operations.
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            size: self.current_size.load(std::sync::atomic::Ordering::Relaxed),
            max_size: self.max_size,
            hits: self.hits.load(std::sync::atomic::Ordering::Relaxed),
            misses: self.misses.load(std::sync::atomic::Ordering::Relaxed),
            hit_rate: self.calculate_hit_rate(),
        }
    }
    
    /// Calculate hit rate with atomic operations.
    fn calculate_hit_rate(&self) -> f64 {
        let hits = self.hits.load(std::sync::atomic::Ordering::Relaxed);
        let misses = self.misses.load(std::sync::atomic::Ordering::Relaxed);
        
        if hits + misses == 0 {
            0.0
        } else {
            (hits as f64 / (hits + misses) as f64) * 100.0
        }
    }
    
    /// Evict expired entries with optimized cleanup.
    async fn cleanup_expired(&self) {
        let mut storage = self.storage.write().await;
        let mut expired_keys = SmallVec::<[String; 32]>::new();
        
        // Find expired entries
        for (key, entry) in storage.iter() {
            if entry.is_expired() {
                expired_keys.push(key.clone());
            }
        }
        
        // Remove expired entries
        for key in expired_keys {
            storage.remove(&key);
            self.current_size.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
    
    /// Evict least recently used entries when cache is full.
    async fn evict_lru(&self) {
        let mut storage = self.storage.write().await;
        
        // Simple LRU: remove oldest entry
        if let Some(oldest_key) = storage
            .iter()
            .min_by_key(|(_, entry)| entry.created_at)
            .map(|(key, _)| key.clone())
        {
            storage.remove(&oldest_key);
            self.current_size.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

#[async_trait]
impl Cache for OptimizedL1Cache {
    async fn get(&self, key: &str) -> ActorCoreResult<Option<serde_json::Value>> {
        let mut storage = self.storage.write().await;
        
        if let Some(entry) = storage.get(key) {
            if entry.is_expired() {
                storage.remove(key);
                self.current_size.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Ok(None)
            } else {
                self.hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Ok(Some(entry.value.clone()))
            }
        } else {
            self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Ok(None)
        }
    }
    
    async fn set(
        &self,
        key: &str,
        value: &serde_json::Value,
        ttl: Option<Duration>,
    ) -> ActorCoreResult<()> {
        let mut storage = self.storage.write().await;
        
        // Check if we need to evict
        let current_size = self.current_size.load(std::sync::atomic::Ordering::Relaxed);
        if current_size >= self.max_size && !storage.contains_key(key) {
            self.evict_lru().await;
        }
        
        let entry = CacheEntry {
            value: value.clone(),
            created_at: Instant::now(),
            ttl,
        };
        
        let is_new_entry = !storage.contains_key(key);
        storage.insert(key.to_string(), entry);
        
        if is_new_entry {
            self.current_size.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> ActorCoreResult<bool> {
        let mut storage = self.storage.write().await;
        
        if storage.remove(key).is_some() {
            self.current_size.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    async fn clear(&self) -> ActorCoreResult<()> {
        let mut storage = self.storage.write().await;
        storage.clear();
        self.current_size.store(0, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
    
    async fn get_stats(&self) -> ActorCoreResult<crate::metrics::CacheStats> {
        Ok(self.get_stats())
    }
}

/// Cache statistics with atomic operations.
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub max_size: usize,
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
}

/// High-performance multi-get cache operations.
pub struct BatchCacheOperations;

impl BatchCacheOperations {
    /// Get multiple keys in a single operation with optimized batching.
    pub async fn get_many<C: Cache>(
        cache: &C,
        keys: &[String],
    ) -> ActorCoreResult<Vec<Option<serde_json::Value>>> {
        let mut results = Vec::with_capacity(keys.len());
        
        // Use SmallVec for small key collections
        let keys_small: SmallVec<[&String; 32]> = keys.iter().collect();
        
        // Process in parallel batches
        let batch_size = 16; // Optimal batch size for most use cases
        for chunk in keys_small.chunks(batch_size) {
            let mut batch_futures = Vec::new();
            
            for key in chunk {
                batch_futures.push(cache.get(key));
            }
            
            // Wait for batch to complete
            for future in batch_futures {
                results.push(future.await?);
            }
        }
        
        Ok(results)
    }
    
    /// Set multiple key-value pairs in a single operation.
    pub async fn set_many<C: Cache>(
        cache: &C,
        items: &[(String, serde_json::Value)],
        ttl: Option<Duration>,
    ) -> ActorCoreResult<()> {
        // Use SmallVec for small item collections
        let items_small: SmallVec<[&(String, serde_json::Value); 32]> = items.iter().collect();
        
        // Process in parallel batches
        let batch_size = 16;
        for chunk in items_small.chunks(batch_size) {
            let mut batch_futures = Vec::new();
            
            for (key, value) in chunk {
                batch_futures.push(cache.set(key, value, ttl));
            }
            
            // Wait for batch to complete
            for future in batch_futures {
                future.await?;
            }
        }
        
        Ok(())
    }
}

/// Optimized cache warming with batching.
pub struct CacheWarmer {
    /// Cache to warm
    cache: Arc<dyn Cache>,
    /// Warming batch size
    batch_size: usize,
    /// Statistics
    stats: Arc<RwLock<WarmingStats>>,
}

/// Cache warming statistics.
#[derive(Debug, Default)]
pub struct WarmingStats {
    pub items_warmed: u64,
    pub warming_time: Duration,
    pub errors: u64,
}

impl CacheWarmer {
    /// Create a new cache warmer.
    pub fn new(cache: Arc<dyn Cache>, batch_size: usize) -> Self {
        Self {
            cache,
            batch_size,
            stats: Arc::new(RwLock::new(WarmingStats::default())),
        }
    }
    
    /// Warm cache with multiple items using optimized batching.
    pub async fn warm_cache(
        &self,
        items: Vec<(String, serde_json::Value)>,
        ttl: Option<Duration>,
    ) -> ActorCoreResult<()> {
        let start_time = Instant::now();
        let mut errors = 0;
        
        // Process in optimized batches
        for chunk in items.chunks(self.batch_size) {
            match BatchCacheOperations::set_many(&*self.cache, chunk, ttl).await {
                Ok(()) => {
                    let mut stats = self.stats.write().await;
                    stats.items_warmed += chunk.len() as u64;
                }
                Err(e) => {
                    error!("Cache warming batch failed: {}", e);
                    errors += chunk.len() as u64;
                }
            }
        }
        
        // Update statistics
        let mut stats = self.stats.write().await;
        stats.warming_time = start_time.elapsed();
        stats.errors = errors;
        
        Ok(())
    }
    
    /// Get warming statistics.
    pub async fn get_stats(&self) -> WarmingStats {
        self.stats.read().await.clone()
    }
}

/// High-performance cache key generator with optimized hashing.
pub struct OptimizedKeyGenerator {
    /// Hash state for incremental hashing
    hasher: Arc<RwLock<fxhash::FxHasher>>,
}

impl OptimizedKeyGenerator {
    /// Create a new key generator.
    pub fn new() -> Self {
        Self {
            hasher: Arc::new(RwLock::new(fxhash::FxHasher::default())),
        }
    }
    
    /// Generate a cache key with optimized hashing.
    pub fn generate_key(&self, components: &[&str]) -> String {
        use std::hash::{Hash, Hasher};
        
        let mut hasher = fxhash::FxHasher::default();
        
        for component in components {
            component.hash(&mut hasher);
        }
        
        format!("cache_{}", hasher.finish())
    }
    
    /// Generate a cache key from a UUID and version.
    pub fn generate_actor_key(&self, actor_id: uuid::Uuid, version: i64) -> String {
        use std::hash::{Hash, Hasher};
        
        let mut hasher = fxhash::FxHasher::default();
        actor_id.hash(&mut hasher);
        version.hash(&mut hasher);
        
        format!("actor_{}", hasher.finish())
    }
}