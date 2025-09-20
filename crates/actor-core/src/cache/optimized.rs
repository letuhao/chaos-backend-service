//! Optimized cache implementations with micro-optimizations.
//!
//! This module provides high-performance cache implementations using
//! smallvec, fxhash, and atomic operations for hot paths.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::error;

use fxhash::FxHashMap;

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
    #[allow(dead_code)]
    async fn cleanup_expired(&self) {
        let mut storage = self.storage.write().await;
        let mut expired_keys = Vec::new();
        
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

impl Cache for OptimizedL1Cache {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        let mut storage = futures::executor::block_on(self.storage.write());
        
        if let Some(entry) = storage.get(key) {
            if entry.is_expired() {
                storage.remove(key);
                self.current_size.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                None
            } else {
                self.hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Some(entry.value.clone())
            }
        } else {
            self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            None
        }
    }
    
    fn set(
        &self,
        key: String,
        value: serde_json::Value,
        ttl: Option<u64>,
    ) -> ActorCoreResult<()> {
        let mut storage = futures::executor::block_on(self.storage.write());
        
        // Check if we need to evict
        let current_size = self.current_size.load(std::sync::atomic::Ordering::Relaxed);
        if current_size >= self.max_size && !storage.contains_key(&key) {
            futures::executor::block_on(self.evict_lru());
        }
        
        let entry = CacheEntry {
            value: value.clone(),
            created_at: Instant::now(),
            ttl: ttl.map(|t| Duration::from_secs(t)),
        };
        
        let is_new_entry = !storage.contains_key(&key);
        storage.insert(key, entry);
        
        if is_new_entry {
            self.current_size.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        
        Ok(())
    }
    
    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        let mut storage = futures::executor::block_on(self.storage.write());
        
        if storage.remove(key).is_some() {
            self.current_size.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        } else {
            Ok(())
        }
    }
    
    fn clear(&self) -> ActorCoreResult<()> {
        let mut storage = futures::executor::block_on(self.storage.write());
        storage.clear();
        self.current_size.store(0, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
    
    fn get_stats(&self) -> crate::metrics::CacheStats {
        crate::metrics::CacheStats {
            hits: self.hits.load(std::sync::atomic::Ordering::Relaxed),
            misses: self.misses.load(std::sync::atomic::Ordering::Relaxed),
            // TODO: Implement sets and deletes counters for complete metrics
            sets: 0, // should be tracked with atomic counters
            deletes: 0, // should be tracked with atomic counters
            memory_usage: self.current_size.load(std::sync::atomic::Ordering::Relaxed) as u64 * 1024, // Estimate
            max_memory_usage: self.max_size as u64 * 1024, // Estimate
        }
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
    pub fn get_many<C: Cache + ?Sized>(
        cache: &C,
        keys: &[String],
    ) -> ActorCoreResult<Vec<Option<serde_json::Value>>> {
        let mut results = Vec::with_capacity(keys.len());
        
        // Process in batches
        let batch_size = 16; // Optimal batch size for most use cases
        for chunk in keys.chunks(batch_size) {
            for key in chunk {
                results.push(cache.get(key));
            }
        }
        
        Ok(results)
    }
    
    /// Set multiple key-value pairs in a single operation.
    pub fn set_many<C: Cache + ?Sized>(
        cache: &C,
        items: &[(String, serde_json::Value)],
        ttl: Option<u64>,
    ) -> ActorCoreResult<()> {
        // Process in batches
        let batch_size = 16;
        for chunk in items.chunks(batch_size) {
            for (key, value) in chunk {
                cache.set(key.clone(), value.clone(), ttl)?;
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
#[derive(Debug, Default, Clone)]
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
            match BatchCacheOperations::set_many(&*self.cache, chunk, ttl.map(|d| d.as_secs())) {
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
    _hasher: Arc<RwLock<fxhash::FxHasher>>,
}

impl OptimizedKeyGenerator {
    /// Create a new key generator.
    pub fn new() -> Self {
        Self {
            _hasher: Arc::new(RwLock::new(fxhash::FxHasher::default())),
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