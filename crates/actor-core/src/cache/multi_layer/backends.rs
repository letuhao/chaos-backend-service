//! Cache backend implementations for the multi-layer cache system.
//!
//! This module provides concrete implementations of the L1, L2, and L3 cache traits
//! using various storage backends and optimization techniques.

use async_trait::async_trait;
use dashmap::DashMap;
#[cfg(feature = "memory-mapped")]
use memmap2::Mmap;
use std::collections::HashMap;
use std::fs::{File, create_dir_all, read_dir, remove_file};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tracing::error;

use crate::ActorCoreResult;
use super::layers::{L1Cache, L2Cache, L3Cache, CacheEntry, LayerConfig};
use super::metrics::{L1CacheStats, L2CacheStats, L3CacheStats};

/// Lock-free L1 cache implementation using DashMap.
pub struct LockFreeL1Cache {
    /// The underlying cache storage
    cache: DashMap<String, CacheEntry>,
    /// Maximum capacity
    max_capacity: usize,
    /// Statistics
    stats: Arc<RwLock<L1CacheStats>>,
    /// Configuration
    config: LayerConfig,
}

impl LockFreeL1Cache {
    /// Create a new lock-free L1 cache.
    pub fn new(max_capacity: usize) -> Self {
        Self {
            cache: DashMap::new(),
            max_capacity,
            stats: Arc::new(RwLock::new(L1CacheStats::new())),
            config: LayerConfig::l1(max_capacity),
        }
    }

    /// Create a new lock-free L1 cache with configuration.
    pub fn with_config(config: LayerConfig) -> Self {
        Self {
            cache: DashMap::new(),
            max_capacity: config.max_capacity,
            stats: Arc::new(RwLock::new(L1CacheStats::new())),
            config,
        }
    }

    /// Evict entries based on the configured policy.
    fn evict_entries(&self) -> ActorCoreResult<()> {
        if self.cache.len() < self.max_capacity {
            return Ok(());
        }

        let entries_to_remove = self.cache.len() - self.max_capacity + 1;
        let mut entries: Vec<(String, CacheEntry)> = self.cache
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect();

        // Sort based on eviction policy
        match self.config.eviction_policy {
            crate::cache::multi_layer::policy::EvictionPolicy::Lru => {
                entries.sort_by(|a, b| a.1.last_accessed.cmp(&b.1.last_accessed));
            }
            crate::cache::multi_layer::policy::EvictionPolicy::Lfu => {
                entries.sort_by(|a, b| a.1.access_count.cmp(&b.1.access_count));
            }
            crate::cache::multi_layer::policy::EvictionPolicy::Fifo => {
                entries.sort_by(|a, b| a.1.created_at.cmp(&b.1.created_at));
            }
            crate::cache::multi_layer::policy::EvictionPolicy::Random => {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut rng = DefaultHasher::new();
                SystemTime::now().hash(&mut rng);
                entries.sort_by(|a, b| {
                    let mut hasher_a = DefaultHasher::new();
                    a.0.hash(&mut hasher_a);
                    let mut hasher_b = DefaultHasher::new();
                    b.0.hash(&mut hasher_b);
                    hasher_a.finish().cmp(&hasher_b.finish())
                });
            }
        }

        // Remove the oldest/least used entries
        for (key, _) in entries.iter().take(entries_to_remove) {
            if self.cache.remove(key).is_some() {
                self.stats.blocking_write().record_eviction();
            }
        }

        Ok(())
    }

    /// Update statistics after an operation.
    async fn update_stats(&self, hit: bool, response_time: Duration) {
        let mut stats = self.stats.write().await;
        stats.update_operation(hit, response_time);
        stats.update_memory_usage(self.calculate_memory_usage());
    }

    /// Calculate current memory usage.
    fn calculate_memory_usage(&self) -> u64 {
        self.cache.iter()
            .map(|entry| entry.value().size as u64)
            .sum()
    }
}

impl L1Cache for LockFreeL1Cache {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        let start_time = Instant::now();
        
        if let Some(mut entry) = self.cache.get_mut(key) {
            // Check if expired
            if entry.is_expired() {
                drop(entry);
                let _ = self.cache.remove(key);
                return None;
            }

            // Update access information
            entry.touch();
            let response_time = start_time.elapsed();
            
        // Update stats synchronously
        let mut stats = self.stats.blocking_write();
        stats.update_operation(true, response_time);

            return Some(entry.value.clone());
        }

        let response_time = start_time.elapsed();
        let mut stats = self.stats.blocking_write();
        stats.update_operation(false, response_time);

        None
    }

    fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()> {
        let start_time = Instant::now();
        
        // Create cache entry
        let entry = CacheEntry::new(value, ttl.or(self.config.default_ttl));
        
        // Check capacity and evict if necessary
        if self.cache.len() >= self.max_capacity {
            self.evict_entries()?;
        }

        // Insert the entry
        self.cache.insert(key, entry);
        
        let response_time = start_time.elapsed();
        let mut stats = self.stats.blocking_write();
        stats.record_set();
        stats.update_operation(true, response_time);
        stats.update_memory_usage(self.calculate_memory_usage());

        Ok(())
    }

    fn delete(&self, key: &str) -> ActorCoreResult<()> {
        let start_time = Instant::now();
        
        if self.cache.remove(key).is_some() {
            let response_time = start_time.elapsed();
            let mut stats = self.stats.blocking_write();
            stats.record_delete();
            stats.update_operation(true, response_time);
            stats.update_memory_usage(self.calculate_memory_usage());
        }

        Ok(())
    }

    fn clear(&self) -> ActorCoreResult<()> {
        self.cache.clear();
        
        let mut stats = self.stats.blocking_write();
        stats.reset();

        Ok(())
    }

    fn get_stats(&self) -> L1CacheStats {
        // This is a simplified implementation
        // In a real implementation, you'd want to return the current stats
        L1CacheStats::new()
    }

    fn memory_usage(&self) -> u64 {
        self.calculate_memory_usage()
    }

    fn max_capacity(&self) -> usize {
        self.max_capacity
    }
}

/// Memory-mapped L2 cache implementation.
#[cfg(feature = "memory-mapped")]
pub struct MemoryMappedL2Cache {
    /// The underlying cache storage
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// Memory-mapped file
    mmap: Arc<RwLock<Option<Mmap>>>,
    /// Cache file path
    cache_path: String,
    /// Maximum capacity
    max_capacity: usize,
    /// Statistics
    stats: Arc<RwLock<L2CacheStats>>,
    /// Configuration
    config: LayerConfig,
}

#[cfg(feature = "memory-mapped")]
impl MemoryMappedL2Cache {
    /// Create a new memory-mapped L2 cache.
    pub fn new(cache_path: String, max_capacity: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            mmap: Arc::new(RwLock::new(None)),
            cache_path,
            max_capacity,
            stats: Arc::new(RwLock::new(L2CacheStats::new())),
            config: LayerConfig::l2(max_capacity),
        }
    }

    /// Create a new memory-mapped L2 cache with configuration.
    pub fn with_config(cache_path: String, config: LayerConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            mmap: Arc::new(RwLock::new(None)),
            cache_path,
            max_capacity: config.max_capacity,
            stats: Arc::new(RwLock::new(L2CacheStats::new())),
            config,
        }
    }

    /// Load cache from disk.
    async fn load_from_disk(&self) -> ActorCoreResult<()> {
        if !Path::new(&self.cache_path).exists() {
            return Ok(());
        }

        let file = File::open(&self.cache_path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        
        // Deserialize cache data
        let cache_data: HashMap<String, CacheEntry> = serde_json::from_slice(&mmap)?;
        
        let mut cache = self.cache.write().await;
        *cache = cache_data;
        
        let mut mmap_guard = self.mmap.write().await;
        *mmap_guard = Some(mmap);

        Ok(())
    }

    /// Save cache to disk.
    async fn save_to_disk(&self) -> ActorCoreResult<()> {
        let cache = self.cache.read().await;
        let serialized = serde_json::to_vec(&*cache)?;
        
        let mut file = File::create(&self.cache_path)?;
        file.write_all(&serialized)?;
        file.sync_all()?;

        Ok(())
    }

    /// Calculate current memory usage.
    fn calculate_memory_usage(&self) -> u64 {
        // This is a simplified implementation
        // In a real implementation, you'd calculate the actual memory usage
        0
    }
}

#[cfg(feature = "memory-mapped")]
#[async_trait]
impl L2Cache for MemoryMappedL2Cache {
    async fn get(&self, key: &str) -> ActorCoreResult<Option<serde_json::Value>> {
        let start_time = Instant::now();
        
        let cache = self.cache.read().await;
        if let Some(entry) = cache.get(key) {
            if entry.is_expired() {
                return Ok(None);
            }

            let response_time = start_time.elapsed();
            let mut stats = self.stats.write().await;
            stats.update_operation(true, response_time);

            return Ok(Some(entry.value.clone()));
        }

        let response_time = start_time.elapsed();
        let mut stats = self.stats.write().await;
        stats.update_operation(false, response_time);

        Ok(None)
    }

    async fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()> {
        let start_time = Instant::now();
        
        let entry = CacheEntry::new(value, ttl.or(self.config.default_ttl));
        
        let mut cache = self.cache.write().await;
        cache.insert(key, entry);
        
        let response_time = start_time.elapsed();
        let mut stats = self.stats.write().await;
        stats.record_set();
        stats.update_operation(true, response_time);
        stats.update_memory_usage(self.calculate_memory_usage());

        Ok(())
    }

    async fn delete(&self, key: &str) -> ActorCoreResult<()> {
        let start_time = Instant::now();
        
        let mut cache = self.cache.write().await;
        cache.remove(key);
        
        let response_time = start_time.elapsed();
        let mut stats = self.stats.write().await;
        stats.record_delete();
        stats.update_operation(true, response_time);
        stats.update_memory_usage(self.calculate_memory_usage());

        Ok(())
    }

    async fn clear(&self) -> ActorCoreResult<()> {
        let mut cache = self.cache.write().await;
        cache.clear();
        
        let mut stats = self.stats.write().await;
        stats.reset();

        Ok(())
    }

    fn get_stats(&self) -> L2CacheStats {
        L2CacheStats::new()
    }

    fn memory_usage(&self) -> u64 {
        self.calculate_memory_usage()
    }

    fn max_capacity(&self) -> usize {
        self.max_capacity
    }

    async fn sync(&self) -> ActorCoreResult<()> {
        self.save_to_disk().await
    }
}

/// Persistent L3 cache implementation.
pub struct PersistentL3Cache {
    /// Cache directory
    cache_dir: String,
    /// Maximum capacity
    max_capacity: usize,
    /// Statistics
    stats: Arc<RwLock<L3CacheStats>>,
    /// Configuration
    config: LayerConfig,
}

impl PersistentL3Cache {
    /// Create a new persistent L3 cache.
    pub fn new(cache_dir: String, max_capacity: usize) -> Self {
        Self {
            cache_dir,
            max_capacity,
            stats: Arc::new(RwLock::new(L3CacheStats::new())),
            config: LayerConfig::l3(max_capacity),
        }
    }

    /// Create a new persistent L3 cache with configuration.
    pub fn with_config(cache_dir: String, config: LayerConfig) -> Self {
        Self {
            cache_dir,
            max_capacity: config.max_capacity,
            stats: Arc::new(RwLock::new(L3CacheStats::new())),
            config,
        }
    }

    /// Get the file path for a cache key.
    fn get_file_path(&self, key: &str) -> String {
        format!("{}/{}.cache", self.cache_dir, key)
    }

    /// Calculate current disk usage.
    fn calculate_disk_usage(&self) -> u64 {
        if let Ok(entries) = read_dir(&self.cache_dir) {
            entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| entry.metadata().ok())
                .map(|metadata| metadata.len())
                .sum()
        } else {
            0
        }
    }
}

#[async_trait]
impl L3Cache for PersistentL3Cache {
    async fn get(&self, key: &str) -> ActorCoreResult<Option<serde_json::Value>> {
        let start_time = Instant::now();
        
        let file_path = self.get_file_path(key);
        if !Path::new(&file_path).exists() {
            let response_time = start_time.elapsed();
            let mut stats = self.stats.write().await;
            stats.update_operation(false, response_time);
            return Ok(None);
        }

        let file = File::open(&file_path)?;
        let mut contents = String::new();
        file.take(1024 * 1024).read_to_string(&mut contents)?; // Limit to 1MB
        
        let entry: CacheEntry = serde_json::from_str(&contents)?;
        
        if entry.is_expired() {
            let _ = remove_file(&file_path);
            return Ok(None);
        }

        let response_time = start_time.elapsed();
        let mut stats = self.stats.write().await;
        stats.update_operation(true, response_time);
        stats.update_disk_usage(self.calculate_disk_usage());

        Ok(Some(entry.value))
    }

    async fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()> {
        let start_time = Instant::now();
        
        // Ensure cache directory exists
        create_dir_all(&self.cache_dir)?;
        
        let entry = CacheEntry::new(value, ttl.or(self.config.default_ttl));
        let serialized = serde_json::to_string(&entry)?;
        
        let file_path = self.get_file_path(&key);
        let mut file = File::create(&file_path)?;
        file.write_all(serialized.as_bytes())?;
        file.sync_all()?;

        let response_time = start_time.elapsed();
        let mut stats = self.stats.write().await;
        stats.record_set();
        stats.update_operation(true, response_time);
        stats.update_disk_usage(self.calculate_disk_usage());

        Ok(())
    }

    async fn delete(&self, key: &str) -> ActorCoreResult<()> {
        let start_time = Instant::now();
        
        let file_path = self.get_file_path(key);
        if Path::new(&file_path).exists() {
            remove_file(&file_path)?;
        }

        let response_time = start_time.elapsed();
        let mut stats = self.stats.write().await;
        stats.record_delete();
        stats.update_operation(true, response_time);
        stats.update_disk_usage(self.calculate_disk_usage());

        Ok(())
    }

    async fn clear(&self) -> ActorCoreResult<()> {
        if let Ok(entries) = read_dir(&self.cache_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let _ = remove_file(entry.path());
                }
            }
        }

        let mut stats = self.stats.write().await;
        stats.reset();

        Ok(())
    }

    fn get_stats(&self) -> L3CacheStats {
        L3CacheStats::new()
    }

    fn disk_usage(&self) -> u64 {
        self.calculate_disk_usage()
    }

    fn max_capacity(&self) -> usize {
        self.max_capacity
    }

    async fn compact(&self) -> ActorCoreResult<()> {
        // This is a simplified implementation
        // In a real implementation, you'd implement proper compaction
        Ok(())
    }
}