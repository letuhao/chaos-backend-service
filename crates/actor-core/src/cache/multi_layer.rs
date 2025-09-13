//! Multi-layer cache system implementation.
//!
//! This module provides a sophisticated three-layer cache system:
//! - L1: Lock-free in-memory cache (fastest access)
//! - L2: Memory-mapped file cache (persistent, medium speed)
//! - L3: Persistent disk cache (slowest, largest capacity)

use std::collections::HashMap;
use std::fs::{File, create_dir_all, read_dir, remove_file};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::sync::RwLock;
use memmap2::Mmap;
use serde::{Deserialize, Serialize};
use crate::ActorCoreResult;
use crate::interfaces::Cache;
use async_trait::async_trait;

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
    pub l3_compression: bool,
    
    /// Performance settings
    pub enable_preloading: bool,
    pub preload_workers: usize,
    pub sync_interval: Duration,
}

/// Eviction policies for cache layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvictionPolicy {
    /// Least Recently Used
    Lru,
    /// Least Frequently Used
    Lfu,
    /// First In First Out
    Fifo,
    /// Random
    Random,
}

/// Statistics for the multi-layer cache system.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiLayerStats {
    /// Overall statistics
    pub total_hits: u64,
    pub total_misses: u64,
    pub total_sets: u64,
    pub total_gets: u64,
    
    /// Layer-specific statistics
    pub l1_hits: u64,
    pub l1_misses: u64,
    pub l2_hits: u64,
    pub l2_misses: u64,
    pub l3_hits: u64,
    pub l3_misses: u64,
    
    /// Performance statistics
    pub average_latency: Duration,
    pub last_sync_time: Option<u64>,
    pub sync_count: u64,
}

/// L1 cache trait for lock-free in-memory caching.
pub trait L1Cache: Send + Sync {
    /// Get a value from the L1 cache.
    fn get(&self, key: &str) -> Option<serde_json::Value>;
    
    /// Set a value in the L1 cache.
    fn set(&self, key: String, value: serde_json::Value, ttl: Option<Duration>) -> ActorCoreResult<()>;
    
    /// Delete a value from the L1 cache.
    fn delete(&self, key: &str) -> ActorCoreResult<()>;
    
    /// Clear all values from the L1 cache.
    fn clear(&self) -> ActorCoreResult<()>;
    
    /// Get L1 cache statistics.
    fn get_stats(&self) -> L1CacheStats;
    
    /// Get the current size of the L1 cache.
    fn size(&self) -> usize;
    
    /// Get the maximum size of the L1 cache.
    fn max_size(&self) -> usize;
}

/// L2 cache trait for memory-mapped file caching.
pub trait L2Cache: Send + Sync {
    /// Get a value from the L2 cache.
    fn get(&self, key: &str) -> Option<serde_json::Value>;
    
    /// Set a value in the L2 cache.
    fn set(&self, key: String, value: serde_json::Value, ttl: Option<Duration>) -> ActorCoreResult<()>;
    
    /// Delete a value from the L2 cache.
    fn delete(&self, key: &str) -> ActorCoreResult<()>;
    
    /// Clear all values from the L2 cache.
    fn clear(&self) -> ActorCoreResult<()>;
    
    /// Get L2 cache statistics.
    fn get_stats(&self) -> L2CacheStats;
    
    /// Sync the L2 cache to disk.
    fn sync(&self) -> ActorCoreResult<()>;
    
    /// Get the current size of the L2 cache.
    fn size(&self) -> usize;
}

/// L3 cache trait for persistent disk caching.
pub trait L3Cache: Send + Sync {
    /// Get a value from the L3 cache.
    fn get(&self, key: &str) -> Option<serde_json::Value>;
    
    /// Set a value in the L3 cache.
    fn set(&self, key: String, value: serde_json::Value, ttl: Option<Duration>) -> ActorCoreResult<()>;
    
    /// Delete a value from the L3 cache.
    fn delete(&self, key: &str) -> ActorCoreResult<()>;
    
    /// Clear all values from the L3 cache.
    fn clear(&self) -> ActorCoreResult<()>;
    
    /// Get L3 cache statistics.
    fn get_stats(&self) -> L3CacheStats;
    
    /// Compact the L3 cache.
    fn compact(&self) -> ActorCoreResult<()>;
    
    /// Get the current size of the L3 cache.
    fn size(&self) -> usize;
}

/// L1 cache statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct L1CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub sets: u64,
    pub deletes: u64,
    pub evictions: u64,
    pub memory_usage: usize,
    pub hit_rate: f64,
}

/// L2 cache statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct L2CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub sets: u64,
    pub deletes: u64,
    pub syncs: u64,
    pub memory_usage: usize,
    pub disk_usage: usize,
    pub hit_rate: f64,
}

/// L3 cache statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct L3CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub sets: u64,
    pub deletes: u64,
    pub compactions: u64,
    pub disk_usage: usize,
    pub compressed_size: usize,
    pub hit_rate: f64,
}

impl MultiLayerCacheManager {
    /// Create a new multi-layer cache manager.
    pub async fn new(config: MultiLayerConfig) -> ActorCoreResult<Self> {
        // Create L1 cache
        let l1_cache = Arc::new(LockFreeL1Cache::new(
            config.l1_max_size,
            config.l1_eviction_policy,
        )?);
        
        // Create L2 cache
        let l2_cache = Arc::new(MemoryMappedL2Cache::new(
            &config.l2_cache_path,
            config.l2_max_size,
        ).await?);
        
        // Create L3 cache
        let l3_cache = Arc::new(PersistentL3Cache::new(
            &config.l3_cache_dir,
            config.l3_max_size,
            config.l3_compression,
        ).await?);
        
        let manager = Self {
            l1_cache,
            l2_cache,
            l3_cache,
            config,
            stats: Arc::new(RwLock::new(MultiLayerStats::default())),
            sync_handle: None,
        };
        
        // Start background sync if enabled
        if manager.config.enable_preloading {
            manager.start_background_sync().await;
        }
        
        Ok(manager)
    }
    
    /// Get a value from the cache (tries L1 -> L2 -> L3).
    pub async fn get(&self, key: &str) -> Option<serde_json::Value> {
        let _start_time = Instant::now();
        
        // Try L1 first
        if let Some(value) = self.l1_cache.get(key) {
            self.update_stats_l1_hit().await;
            return Some(value);
        }
        
        // Try L2
        if let Some(value) = self.l2_cache.get(key) {
            // Promote to L1
            if let Ok(()) = self.l1_cache.set(key.to_string(), value.clone(), None) {
                self.update_stats_l2_hit().await;
                return Some(value);
            }
        }
        
        // Try L3
        if let Some(value) = self.l3_cache.get(key) {
            // Promote to L1 and L2
            let _ = self.l1_cache.set(key.to_string(), value.clone(), None);
            let _ = self.l2_cache.set(key.to_string(), value.clone(), None);
            self.update_stats_l3_hit().await;
            return Some(value);
        }
        
        // Cache miss
        self.update_stats_miss().await;
        None
    }
    
    /// Set a value in all cache layers.
    pub async fn set(&self, key: String, value: serde_json::Value, ttl: Option<Duration>) -> ActorCoreResult<()> {
        // Set in all layers
        self.l1_cache.set(key.clone(), value.clone(), ttl)?;
        self.l2_cache.set(key.clone(), value.clone(), ttl)?;
        self.l3_cache.set(key, value, ttl)?;
        
        self.update_stats_set().await;
        Ok(())
    }
    
    /// Delete a value from all cache layers.
    pub async fn delete(&self, key: &str) -> ActorCoreResult<()> {
        self.l1_cache.delete(key)?;
        self.l2_cache.delete(key)?;
        self.l3_cache.delete(key)?;
        
        self.update_stats_delete().await;
        Ok(())
    }
    
    /// Clear all cache layers.
    pub async fn clear(&self) -> ActorCoreResult<()> {
        self.l1_cache.clear()?;
        self.l2_cache.clear()?;
        self.l3_cache.clear()?;
        
        self.update_stats_clear().await;
        Ok(())
    }
    
    /// Get comprehensive statistics.
    pub async fn get_stats(&self) -> MultiLayerStats {
        self.stats.read().unwrap().clone()
    }
    
    /// Start background synchronization.
    async fn start_background_sync(&self) {
        let l2_cache = self.l2_cache.clone();
        let _l3_cache = self.l3_cache.clone();
        let stats = self.stats.clone();
        let interval = self.config.sync_interval;
        
        let handle = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                // Sync L2 cache
                if let Err(e) = l2_cache.sync() {
                    eprintln!("L2 cache sync failed: {}", e);
                }
                
                // Update sync statistics
                {
                    let mut stats = stats.write().unwrap();
                    stats.last_sync_time = Some(Instant::now().elapsed().as_secs());
                    stats.sync_count += 1;
                }
            }
        });
        
        // Store the handle (this would need to be handled properly in a real implementation)
        drop(handle);
    }
    
    /// Update statistics for L1 hit.
    async fn update_stats_l1_hit(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.total_hits += 1;
        stats.l1_hits += 1;
    }
    
    /// Update statistics for L2 hit.
    async fn update_stats_l2_hit(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.total_hits += 1;
        stats.l2_hits += 1;
    }
    
    /// Update statistics for L3 hit.
    async fn update_stats_l3_hit(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.total_hits += 1;
        stats.l3_hits += 1;
    }
    
    /// Update statistics for cache miss.
    async fn update_stats_miss(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.total_misses += 1;
    }
    
    /// Update statistics for cache set.
    async fn update_stats_set(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.total_sets += 1;
    }
    
    /// Update statistics for cache delete.
    async fn update_stats_delete(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.total_gets += 1;
    }
    
    /// Update statistics for cache clear.
    async fn update_stats_clear(&self) {
        // Clear all statistics
        let mut stats = self.stats.write().unwrap();
        *stats = MultiLayerStats::default();
    }
}

// Placeholder implementations for the cache layers
// These would be implemented with proper lock-free data structures and memory mapping

/// Lock-free L1 cache implementation.
pub struct LockFreeL1Cache {
    max_size: usize,
    #[allow(dead_code)]
    eviction_policy: EvictionPolicy,
    // In a real implementation, this would use lock-free data structures
    data: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

impl LockFreeL1Cache {
    fn new(max_size: usize, eviction_policy: EvictionPolicy) -> ActorCoreResult<Self> {
        Ok(Self {
            max_size,
            eviction_policy,
            data: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}

impl L1Cache for LockFreeL1Cache {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.data.read().unwrap().get(key).cloned()
    }
    
    fn set(&self, key: String, value: serde_json::Value, _ttl: Option<Duration>) -> ActorCoreResult<()> {
        let mut data = self.data.write().unwrap();
        data.insert(key, value);
        Ok(())
    }
    
    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        let mut data = self.data.write().unwrap();
        data.remove(key);
        Ok(())
    }
    
    fn clear(&self) -> ActorCoreResult<()> {
        let mut data = self.data.write().unwrap();
        data.clear();
        Ok(())
    }
    
    fn get_stats(&self) -> L1CacheStats {
        L1CacheStats::default()
    }
    
    fn size(&self) -> usize {
        self.data.read().unwrap().len()
    }
    
    fn max_size(&self) -> usize {
        self.max_size
    }
}

/// Memory-mapped L2 cache implementation.
pub struct MemoryMappedL2Cache {
    cache_path: String,
    max_size: usize,
    // In-memory index for fast lookups
    data: Arc<std::sync::RwLock<HashMap<String, serde_json::Value>>>,
    // Memory-mapped file for on-disk snapshot
    mmap: Arc<std::sync::RwLock<Option<Mmap>>>,
}

impl MemoryMappedL2Cache {
    async fn new(cache_path: &str, max_size: usize) -> ActorCoreResult<Self> {
        let path = Path::new(cache_path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                create_dir_all(parent).map_err(|e| crate::ActorCoreError::CacheError(format!("Failed to create dir: {}", e)))?;
            }
        }
        let mut cache = Self {
            cache_path: cache_path.to_string(),
            max_size,
            data: Arc::new(std::sync::RwLock::new(HashMap::new())),
            mmap: Arc::new(std::sync::RwLock::new(None)),
        };
        // Try load existing snapshot
        let _ = cache.load_from_disk();
        Ok(cache)
    }
}

impl L2Cache for MemoryMappedL2Cache {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.data.read().unwrap().get(key).cloned()
    }
    
    fn set(&self, key: String, value: serde_json::Value, _ttl: Option<Duration>) -> ActorCoreResult<()> {
        let mut data = self.data.write().unwrap();
        if data.len() >= self.max_size { 
            // simple eviction: remove a random key
            if let Some(k) = data.keys().next().cloned() { data.remove(&k); }
        }
        data.insert(key, value);
        // write-through: persist snapshot
        self.sync()
    }
    
    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        let mut data = self.data.write().unwrap();
        data.remove(key);
        self.sync()
    }
    
    fn clear(&self) -> ActorCoreResult<()> {
        let mut data = self.data.write().unwrap();
        data.clear();
        self.sync()
    }
    
    fn get_stats(&self) -> L2CacheStats {
        L2CacheStats::default()
    }
    
    fn sync(&self) -> ActorCoreResult<()> {
        let data = self.data.read().unwrap();
        let bytes = serde_json::to_vec(&*data)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Serialize L2 failed: {}", e)))?;
        let mut file = File::create(&self.cache_path)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Create L2 file failed: {}", e)))?;
        file.write_all(&bytes)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Write L2 file failed: {}", e)))?;
        file.flush().ok();
        // Remap
        let file = File::open(&self.cache_path)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Open L2 file failed: {}", e)))?;
        let mmap = unsafe { Mmap::map(&file) }
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Mmap L2 failed: {}", e)))?;
        let mut guard = self.mmap.write().unwrap();
        *guard = Some(mmap);
        Ok(())
    }
    
    fn size(&self) -> usize {
        self.data.read().unwrap().len()
    }
}

/// Persistent L3 cache implementation.
pub struct PersistentL3Cache {
    cache_dir: String,
    max_size: usize,
    #[allow(dead_code)]
    compression: bool,
    // In-memory index of known keys (filenames)
    index: Arc<std::sync::RwLock<HashMap<String, ()>>>,
}

impl PersistentL3Cache {
    async fn new(cache_dir: &str, max_size: usize, compression: bool) -> ActorCoreResult<Self> {
        create_dir_all(cache_dir)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Create L3 dir failed: {}", e)))?;
        // Build initial index
        let mut index_map = HashMap::new();
        for entry in read_dir(cache_dir).map_err(|e| crate::ActorCoreError::CacheError(format!("Read L3 dir failed: {}", e)))? {
            if let Ok(entry) = entry { if let Some(name) = entry.file_name().to_str() { index_map.insert(name.to_string(), ()); } }
        }
        Ok(Self {
            cache_dir: cache_dir.to_string(),
            max_size,
            compression,
            index: Arc::new(std::sync::RwLock::new(index_map)),
        })
    }
}

impl L3Cache for PersistentL3Cache {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        let path = Path::new(&self.cache_dir).join(format!("{}.json", sanitize_filename(key)));
        if !path.exists() { return None; }
        let mut file = File::open(&path).ok()?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).ok()?;
        serde_json::from_slice(&buf).ok()
    }
    
    fn set(&self, key: String, value: serde_json::Value, _ttl: Option<Duration>) -> ActorCoreResult<()> {
        let path = Path::new(&self.cache_dir).join(format!("{}.json", sanitize_filename(&key)));
        // Simple bounded index: if over capacity, remove an arbitrary file
        if self.size() >= self.max_size {
            if let Some(k) = self.index.read().unwrap().keys().next().cloned() {
                let path_old = Path::new(&self.cache_dir).join(format!("{}.json", sanitize_filename(&k)));
                let _ = remove_file(&path_old);
                self.index.write().unwrap().remove(&k);
            }
        }
        let mut file = File::create(&path)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Create L3 file failed: {}", e)))?;
        let bytes = serde_json::to_vec(&value)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Serialize L3 failed: {}", e)))?;
        file.write_all(&bytes)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Write L3 file failed: {}", e)))?;
        self.index.write().unwrap().insert(key, ());
        Ok(())
    }
    
    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        let path = Path::new(&self.cache_dir).join(format!("{}.json", sanitize_filename(key)));
        let _ = remove_file(&path);
        self.index.write().unwrap().remove(key);
        Ok(())
    }
    
    fn clear(&self) -> ActorCoreResult<()> {
        for entry in read_dir(&self.cache_dir).map_err(|e| crate::ActorCoreError::CacheError(format!("Read L3 dir failed: {}", e)))? {
            if let Ok(entry) = entry { let _ = remove_file(entry.path()); }
        }
        self.index.write().unwrap().clear();
        Ok(())
    }
    
    fn get_stats(&self) -> L3CacheStats {
        L3CacheStats::default()
    }
    
    fn compact(&self) -> ActorCoreResult<()> {
        // Placeholder: nothing to compact for simple per-file store
        Ok(())
    }
    
    fn size(&self) -> usize {
        self.index.read().unwrap().len()
    }
}

impl MemoryMappedL2Cache {
    fn load_from_disk(&mut self) -> ActorCoreResult<()> {
        let path = Path::new(&self.cache_path);
        if !path.exists() { return Ok(()); }
        let mut file = File::open(path)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Open L2 file failed: {}", e)))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Read L2 file failed: {}", e)))?;
        if buf.is_empty() { return Ok(()); }
        let map: HashMap<String, serde_json::Value> = serde_json::from_slice(&buf)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Parse L2 failed: {}", e)))?;
        *self.data.write().unwrap() = map;
        let file = File::open(path)
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Open L2 file failed: {}", e)))?;
        let mmap = unsafe { Mmap::map(&file) }
            .map_err(|e| crate::ActorCoreError::CacheError(format!("Mmap L2 failed: {}", e)))?;
        *self.mmap.write().unwrap() = Some(mmap);
        Ok(())
    }
}

fn sanitize_filename(key: &str) -> String {
    key.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect()
}

#[async_trait]
impl Cache for MultiLayerCacheManager {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        // Try L1 cache first
        if let Some(value) = self.l1_cache.get(key) {
            let mut stats = self.stats.write().unwrap();
            stats.l1_hits += 1;
            return Some(value);
        }
        
        // Try L2 cache
        if let Some(value) = self.l2_cache.get(key) {
            // Store in L1 for faster access next time
            if let Err(e) = self.l1_cache.set(key.to_string(), value.clone(), Some(Duration::from_secs(300))) {
                tracing::warn!("Failed to store in L1 cache: {}", e);
            }
            
            let mut stats = self.stats.write().unwrap();
            stats.l2_hits += 1;
            return Some(value);
        }
        
        // Try L3 cache
        if let Some(value) = self.l3_cache.get(key) {
            // Store in L2 and L1 for faster access next time
            if let Err(e) = self.l2_cache.set(key.to_string(), value.clone(), Some(Duration::from_secs(600))) {
                tracing::warn!("Failed to store in L2 cache: {}", e);
            }
            if let Err(e) = self.l1_cache.set(key.to_string(), value.clone(), Some(Duration::from_secs(300))) {
                tracing::warn!("Failed to store in L1 cache: {}", e);
            }
            
            let mut stats = self.stats.write().unwrap();
            stats.l3_hits += 1;
            return Some(value);
        }
        
        // Cache miss
        let mut stats = self.stats.write().unwrap();
        stats.total_misses += 1;
        None
    }

    fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()> {
        let ttl_duration = ttl.map(Duration::from_secs);
        
        // Store in all layers
        if let Err(e) = self.l1_cache.set(key.clone(), value.clone(), ttl_duration) {
            tracing::warn!("Failed to store in L1 cache: {}", e);
        }
        if let Err(e) = self.l2_cache.set(key.clone(), value.clone(), ttl_duration) {
            tracing::warn!("Failed to store in L2 cache: {}", e);
        }
        if let Err(e) = self.l3_cache.set(key, value, ttl_duration) {
            tracing::warn!("Failed to store in L3 cache: {}", e);
        }
        
        let mut stats = self.stats.write().unwrap();
        stats.total_sets += 1;
        
        Ok(())
    }

    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        // Delete from all layers
        if let Err(e) = self.l1_cache.delete(key) {
            tracing::warn!("Failed to delete from L1 cache: {}", e);
        }
        if let Err(e) = self.l2_cache.delete(key) {
            tracing::warn!("Failed to delete from L2 cache: {}", e);
        }
        if let Err(e) = self.l3_cache.delete(key) {
            tracing::warn!("Failed to delete from L3 cache: {}", e);
        }
        
        let mut stats = self.stats.write().unwrap();
        stats.total_sets += 1; // Using total_sets as a proxy for deletes
        
        Ok(())
    }

    fn clear(&self) -> ActorCoreResult<()> {
        // Clear all layers
        if let Err(e) = self.l1_cache.clear() {
            tracing::warn!("Failed to clear L1 cache: {}", e);
        }
        if let Err(e) = self.l2_cache.clear() {
            tracing::warn!("Failed to clear L2 cache: {}", e);
        }
        if let Err(e) = self.l3_cache.clear() {
            tracing::warn!("Failed to clear L3 cache: {}", e);
        }
        
        let mut stats = self.stats.write().unwrap();
        stats.total_sets += 1; // Using total_sets as a proxy for clears
        
        Ok(())
    }

    fn get_stats(&self) -> crate::interfaces::CacheStats {
        let stats = self.stats.read().unwrap();
        crate::interfaces::CacheStats {
            hits: stats.l1_hits + stats.l2_hits + stats.l3_hits,
            misses: stats.total_misses,
            sets: stats.total_sets,
            deletes: 0, // Not tracked in MultiLayerStats
            memory_usage: 0, // Not tracked in MultiLayerStats
            max_memory_usage: 0, // Not tracked in MultiLayerStats
        }
    }
}
