//! Multi-Layer Resource Cache System
//!
//! This module provides a sophisticated caching system for the Enhanced Hybrid Resource Manager
//! with L1 (in-memory), L2 (memory-mapped), and L3 (MongoDB) cache layers.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::ActorCoreResult;
use serde::{Deserialize, Serialize};
#[cfg(feature = "mongodb-storage")]
use bson;
// use futures::StreamExt;
// use std::time::{Duration, Instant};

/// Convert string error to ActorCoreError
fn to_actor_core_error(msg: String) -> crate::ActorCoreError {
    crate::ActorCoreError::SubsystemError(msg)
}

/// Multi-layer resource cache system
pub struct ResourceCache {
    /// L1 cache (in-memory, fastest)
    l1_cache: Arc<RwLock<HashMap<String, CachedResource>>>,
    /// L2 cache (memory-mapped, fast)
    l2_cache: Option<Arc<dyn L2Cache + Send + Sync>>,
    /// L3 cache (MongoDB, persistent)
    l3_cache: Option<Arc<dyn L3Cache + Send + Sync>>,
    /// Cache configuration
    config: CacheConfig,
}

/// Cached resource entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResource {
    /// Resource value
    pub value: f64,
    /// Timestamp when cached
    pub timestamp: u64,
    /// Time to live in seconds
    pub ttl: u64,
    /// Cache layer where this was stored
    pub layer: CacheLayer,
    /// Resource metadata
    pub metadata: ResourceMetadata,
}

/// Resource metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadata {
    /// Resource category
    pub category: String,
    /// Resource dependencies
    pub dependencies: Vec<String>,
    /// Cache priority (higher = more important)
    pub priority: u32,
    /// Whether this resource is shared
    pub is_shared: bool,
}

/// Cache layers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CacheLayer {
    L1, // In-memory
    L2, // Memory-mapped
    L3, // MongoDB
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// L1 cache TTL in seconds
    pub l1_ttl: u64,
    /// L2 cache TTL in seconds
    pub l2_ttl: u64,
    /// L3 cache TTL in seconds
    pub l3_ttl: u64,
    /// Maximum L1 cache size
    pub max_l1_size: usize,
    /// Maximum L2 cache size
    pub max_l2_size: usize,
    /// Cache warming enabled
    pub warming_enabled: bool,
    /// Batch operations enabled
    pub batch_enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            l1_ttl: 300, // 5 minutes
            l2_ttl: 3600, // 1 hour
            l3_ttl: 86400, // 24 hours
            max_l1_size: 10000,
            max_l2_size: 100000,
            warming_enabled: true,
            batch_enabled: true,
        }
    }
}

/// L2 Cache trait (memory-mapped)
#[async_trait]
pub trait L2Cache: Send + Sync {
    /// Get value from L2 cache
    async fn get(&self, key: &str) -> ActorCoreResult<Option<CachedResource>>;
    
    /// Set value in L2 cache
    async fn set(&self, key: &str, value: &CachedResource) -> ActorCoreResult<()>;
    
    /// Remove value from L2 cache
    async fn remove(&self, key: &str) -> ActorCoreResult<()>;
    
    /// Clear all L2 cache
    async fn clear(&self) -> ActorCoreResult<()>;
    
    /// Get cache size
    async fn size(&self) -> ActorCoreResult<usize>;
}

/// L3 Cache trait (MongoDB)
#[async_trait]
pub trait L3Cache: Send + Sync {
    /// Get value from L3 cache
    async fn get(&self, key: &str) -> ActorCoreResult<Option<CachedResource>>;
    
    /// Set value in L3 cache
    async fn set(&self, key: &str, value: &CachedResource) -> ActorCoreResult<()>;
    
    /// Remove value from L3 cache
    async fn remove(&self, key: &str) -> ActorCoreResult<()>;
    
    /// Clear all L3 cache
    async fn clear(&self) -> ActorCoreResult<()>;
    
    /// Get cache size
    async fn size(&self) -> ActorCoreResult<usize>;
    
    /// Batch get multiple values
    async fn batch_get(&self, keys: &[String]) -> ActorCoreResult<HashMap<String, CachedResource>>;
    
    /// Batch set multiple values
    async fn batch_set(&self, values: &HashMap<String, CachedResource>) -> ActorCoreResult<()>;
}

impl ResourceCache {
    /// Create a new resource cache
    pub fn new(config: CacheConfig) -> Self {
        Self {
            l1_cache: Arc::new(RwLock::new(HashMap::new())),
            l2_cache: None,
            l3_cache: None,
            config,
        }
    }
    
    /// Set L2 cache
    pub fn set_l2_cache(&mut self, l2_cache: Arc<dyn L2Cache + Send + Sync>) {
        self.l2_cache = Some(l2_cache);
    }
    
    /// Set L3 cache
    pub fn set_l3_cache(&mut self, l3_cache: Arc<dyn L3Cache + Send + Sync>) {
        self.l3_cache = Some(l3_cache);
    }
    
    /// Get resource value from cache
    pub async fn get(&self, actor_id: &str, resource_id: &str) -> ActorCoreResult<Option<f64>> {
        let key = format!("{}:{}", actor_id, resource_id);
        
        // Try L1 cache first
        if let Some(cached) = self.get_from_l1(&key).await? {
            if !self.is_expired(&cached) {
                return Ok(Some(cached.value));
            }
        }
        
        // Try L2 cache
        if let Some(l2) = &self.l2_cache {
            if let Some(cached) = l2.get(&key).await? {
                if !self.is_expired(&cached) {
                    // Store in L1 cache
                    self.set_in_l1(&key, &cached).await?;
                    return Ok(Some(cached.value));
                }
            }
        }
        
        // Try L3 cache
        if let Some(l3) = &self.l3_cache {
            if let Some(cached) = l3.get(&key).await? {
                if !self.is_expired(&cached) {
                    // Store in L2 and L1 cache
                    if let Some(l2) = &self.l2_cache {
                        l2.set(&key, &cached).await?;
                    }
                    self.set_in_l1(&key, &cached).await?;
                    return Ok(Some(cached.value));
                }
            }
        }
        
        Ok(None)
    }
    
    /// Set resource value in cache
    pub async fn set(&self, actor_id: &str, resource_id: &str, value: f64, metadata: ResourceMetadata) -> ActorCoreResult<()> {
        let key = format!("{}:{}", actor_id, resource_id);
        
        let cached = CachedResource {
            value,
            timestamp: chrono::Utc::now().timestamp() as u64,
            ttl: self.config.l1_ttl,
            layer: CacheLayer::L1,
            metadata,
        };
        
        // Store in all available layers
        self.set_in_l1(&key, &cached).await?;
        
        if let Some(l2) = &self.l2_cache {
            let mut l2_cached = cached.clone();
            l2_cached.ttl = self.config.l2_ttl;
            l2_cached.layer = CacheLayer::L2;
            l2.set(&key, &l2_cached).await?;
        }
        
        if let Some(l3) = &self.l3_cache {
            let mut l3_cached = cached.clone();
            l3_cached.ttl = self.config.l3_ttl;
            l3_cached.layer = CacheLayer::L3;
            l3.set(&key, &l3_cached).await?;
        }
        
        Ok(())
    }
    
    /// Invalidate cache for actor
    pub async fn invalidate_actor(&self, actor_id: &str) -> ActorCoreResult<()> {
        let prefix = format!("{}:", actor_id);
        
        // Clear from L1 cache
        self.clear_l1_by_prefix(&prefix).await?;
        
        // Clear from L2 cache
        if let Some(l2) = &self.l2_cache {
            // Note: This is a simplified implementation
            // In practice, you'd need to iterate through keys
            l2.clear().await?;
        }
        
        // Clear from L3 cache
        if let Some(l3) = &self.l3_cache {
            l3.clear().await?;
        }
        
        Ok(())
    }
    
    /// Invalidate cache for resource
    pub async fn invalidate_resource(&self, resource_id: &str) -> ActorCoreResult<()> {
        // Clear from L1 cache
        self.clear_l1_by_suffix(&format!(":{}", resource_id)).await?;
        
        // Clear from L2 cache
        if let Some(l2) = &self.l2_cache {
            l2.clear().await?;
        }
        
        // Clear from L3 cache
        if let Some(l3) = &self.l3_cache {
            l3.clear().await?;
        }
        
        Ok(())
    }
    
    /// Clear all cache
    pub async fn clear(&self) -> ActorCoreResult<()> {
        // Clear L1 cache
        {
            let mut cache = self.l1_cache.write().await;
            cache.clear();
        }
        
        // Clear L2 cache
        if let Some(l2) = &self.l2_cache {
            l2.clear().await?;
        }
        
        // Clear L3 cache
        if let Some(l3) = &self.l3_cache {
            l3.clear().await?;
        }
        
        Ok(())
    }
    
    /// Warm cache for active actors
    pub async fn warm_cache(&self, _active_actors: &[String]) -> ActorCoreResult<()> {
        if !self.config.warming_enabled {
            return Ok(());
        }
        
        // This would be implemented to preload resources for active actors
        // For now, it's a placeholder
        Ok(())
    }
    
    /// Get cache statistics
    pub async fn get_stats(&self) -> ActorCoreResult<CacheStats> {
        let l1_size = {
            let cache = self.l1_cache.read().await;
            cache.len()
        };
        
        let l2_size = if let Some(l2) = &self.l2_cache {
            l2.size().await.unwrap_or(0)
        } else {
            0
        };
        
        let l3_size = if let Some(l3) = &self.l3_cache {
            l3.size().await.unwrap_or(0)
        } else {
            0
        };
        
        Ok(CacheStats {
            l1_size,
            l2_size,
            l3_size,
            total_size: l1_size + l2_size + l3_size,
        })
    }
    
    /// Get from L1 cache
    async fn get_from_l1(&self, key: &str) -> ActorCoreResult<Option<CachedResource>> {
        let cache = self.l1_cache.read().await;
        Ok(cache.get(key).cloned())
    }
    
    /// Set in L1 cache
    async fn set_in_l1(&self, key: &str, cached: &CachedResource) -> ActorCoreResult<()> {
        let mut cache = self.l1_cache.write().await;
        
        // Check if we need to evict
        if cache.len() >= self.config.max_l1_size {
            self.evict_l1(&mut cache).await?;
        }
        
        cache.insert(key.to_string(), cached.clone());
        Ok(())
    }
    
    /// Evict from L1 cache
    async fn evict_l1(&self, cache: &mut HashMap<String, CachedResource>) -> ActorCoreResult<()> {
        // Simple LRU eviction - remove oldest entries
        let mut entries: Vec<_> = cache.iter().map(|(k, v)| (k.clone(), v.timestamp)).collect();
        entries.sort_by_key(|(_, timestamp)| *timestamp);
        
        // Remove oldest 10% of entries
        let to_remove = (entries.len() / 10).max(1);
        for (key, _) in entries.iter().take(to_remove) {
            cache.remove(key);
        }
        
        Ok(())
    }
    
    /// Clear L1 cache by prefix
    async fn clear_l1_by_prefix(&self, prefix: &str) -> ActorCoreResult<()> {
        let mut cache = self.l1_cache.write().await;
        cache.retain(|key, _| !key.starts_with(prefix));
        Ok(())
    }
    
    /// Clear L1 cache by suffix
    async fn clear_l1_by_suffix(&self, suffix: &str) -> ActorCoreResult<()> {
        let mut cache = self.l1_cache.write().await;
        cache.retain(|key, _| !key.ends_with(suffix));
        Ok(())
    }
    
    /// Check if cached resource is expired
    fn is_expired(&self, cached: &CachedResource) -> bool {
        let now = chrono::Utc::now().timestamp() as u64;
        now - cached.timestamp > cached.ttl
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub l1_size: usize,
    pub l2_size: usize,
    pub l3_size: usize,
    pub total_size: usize,
}

/// Memory-mapped L2 cache implementation
pub struct MemoryMappedL2Cache {
    /// Memory-mapped file path
    #[allow(dead_code)]
    file_path: String,
    /// Cache data
    data: Arc<RwLock<HashMap<String, CachedResource>>>,
}

impl MemoryMappedL2Cache {
    /// Create a new memory-mapped L2 cache
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl L2Cache for MemoryMappedL2Cache {
    async fn get(&self, key: &str) -> ActorCoreResult<Option<CachedResource>> {
        let data = self.data.read().await;
        Ok(data.get(key).cloned())
    }
    
    async fn set(&self, key: &str, value: &CachedResource) -> ActorCoreResult<()> {
        let mut data = self.data.write().await;
        data.insert(key.to_string(), value.clone());
        Ok(())
    }
    
    async fn remove(&self, key: &str) -> ActorCoreResult<()> {
        let mut data = self.data.write().await;
        data.remove(key);
        Ok(())
    }
    
    async fn clear(&self) -> ActorCoreResult<()> {
        let mut data = self.data.write().await;
        data.clear();
        Ok(())
    }
    
    async fn size(&self) -> ActorCoreResult<usize> {
        let data = self.data.read().await;
        Ok(data.len())
    }
}

/// MongoDB L3 cache implementation
#[cfg(feature = "mongodb-storage")]
pub struct MongoL3Cache {
    /// MongoDB collection
    collection: mongodb::Collection<CachedResource>,
}

#[cfg(feature = "mongodb-storage")]
impl MongoL3Cache {
    /// Create a new MongoDB L3 cache
    pub async fn new(client: &mongodb::Client, database_name: &str) -> ActorCoreResult<Self> {
        let database = client.database(database_name);
        let collection = database.collection("resource_cache");
        
        // Create index for better performance
        let index_model = mongodb::IndexModel::builder()
            .keys(bson::doc! { "timestamp": 1 })
            .build();
        
        collection.create_index(index_model, None)
            .await
            .map_err(|e| to_actor_core_error(format!("Failed to create index: {}", e)))?;
        
        Ok(Self { collection })
    }
}

#[cfg(feature = "mongodb-storage")]
#[async_trait]
impl L3Cache for MongoL3Cache {
    async fn get(&self, key: &str) -> ActorCoreResult<Option<CachedResource>> {
        let filter = bson::doc! { "_id": key };
        let result = self.collection.find_one(filter, None)
            .await
            .map_err(|e| to_actor_core_error(format!("Failed to get from L3 cache: {}", e)))?;
        
        Ok(result)
    }
    
    async fn set(&self, key: &str, value: &CachedResource) -> ActorCoreResult<()> {
        let options = mongodb::options::ReplaceOptions::builder()
            .upsert(true)
            .build();
        
        self.collection.replace_one(
            bson::doc! { "_id": key },
            value,
            options,
        )
        .await
        .map_err(|e| to_actor_core_error(format!("Failed to set in L3 cache: {}", e)))?;
        
        Ok(())
    }
    
    async fn remove(&self, key: &str) -> ActorCoreResult<()> {
        let filter = bson::doc! { "_id": key };
        self.collection.delete_one(filter, None)
            .await
            .map_err(|e| to_actor_core_error(format!("Failed to remove from L3 cache: {}", e)))?;
        
        Ok(())
    }
    
    async fn clear(&self) -> ActorCoreResult<()> {
        self.collection.delete_many(bson::doc! {}, None)
            .await
            .map_err(|e| to_actor_core_error(format!("Failed to clear L3 cache: {}", e)))?;
        
        Ok(())
    }
    
    async fn size(&self) -> ActorCoreResult<usize> {
        let count = self.collection.count_documents(bson::doc! {}, None)
            .await
            .map_err(|e| to_actor_core_error(format!("Failed to count L3 cache: {}", e)))?;
        
        Ok(count as usize)
    }
    
    async fn batch_get(&self, keys: &[String]) -> ActorCoreResult<HashMap<String, CachedResource>> {
        let mut result = HashMap::new();
        
        for key in keys {
            let filter = bson::doc! { "_id": key };
            if let Some(cached_resource) = self.collection.find_one(filter, None)
                .await
                .map_err(|e| to_actor_core_error(format!("Failed to get from L3 cache: {}", e)))? {
                result.insert(key.clone(), cached_resource);
            }
        }
        
        Ok(result)
    }
    
    async fn batch_set(&self, values: &HashMap<String, CachedResource>) -> ActorCoreResult<()> {
        for (key, value) in values {
            // Use the key as a filter and update each one individually
            let filter = bson::doc! { "_id": key };
            let options = mongodb::options::ReplaceOptions::builder()
                .upsert(true)
                .build();
            
            self.collection.replace_one(filter, value, options)
                .await
                .map_err(|e| to_actor_core_error(format!("Failed to set resource {}: {}", key, e)))?;
        }
        
        Ok(())
    }
}