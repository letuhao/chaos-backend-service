//! Cache layer implementations for the multi-layer cache system.
//!
//! This module provides the trait definitions and implementations for
//! the three cache layers: L1 (in-memory), L2 (memory-mapped), and L3 (persistent).

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::ActorCoreResult;

/// L1 cache trait for lock-free in-memory caching.
pub trait L1Cache: Send + Sync {
    /// Get a value from the L1 cache.
    fn get(&self, key: &str) -> Option<serde_json::Value>;
    
    /// Set a value in the L1 cache.
    fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()>;
    
    /// Delete a value from the L1 cache.
    fn delete(&self, key: &str) -> ActorCoreResult<()>;
    
    /// Clear all values from the L1 cache.
    fn clear(&self) -> ActorCoreResult<()>;
    
    /// Get L1 cache statistics.
    fn get_stats(&self) -> crate::cache::multi_layer::metrics::L1CacheStats;
    
    /// Get the current memory usage.
    fn memory_usage(&self) -> u64;
    
    /// Get the maximum memory capacity.
    fn max_capacity(&self) -> usize;
}

/// L2 cache trait for memory-mapped file caching.
#[async_trait]
pub trait L2Cache: Send + Sync {
    /// Get a value from the L2 cache.
    async fn get(&self, key: &str) -> ActorCoreResult<Option<serde_json::Value>>;
    
    /// Set a value in the L2 cache.
    async fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()>;
    
    /// Delete a value from the L2 cache.
    async fn delete(&self, key: &str) -> ActorCoreResult<()>;
    
    /// Clear all values from the L2 cache.
    async fn clear(&self) -> ActorCoreResult<()>;
    
    /// Get L2 cache statistics.
    fn get_stats(&self) -> crate::cache::multi_layer::metrics::L2CacheStats;
    
    /// Get the current memory usage.
    fn memory_usage(&self) -> u64;
    
    /// Get the maximum memory capacity.
    fn max_capacity(&self) -> usize;
    
    /// Sync the cache to disk.
    async fn sync(&self) -> ActorCoreResult<()>;
}

/// L3 cache trait for persistent disk caching.
#[async_trait]
pub trait L3Cache: Send + Sync {
    /// Get a value from the L3 cache.
    async fn get(&self, key: &str) -> ActorCoreResult<Option<serde_json::Value>>;
    
    /// Set a value in the L3 cache.
    async fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()>;
    
    /// Delete a value from the L3 cache.
    async fn delete(&self, key: &str) -> ActorCoreResult<()>;
    
    /// Clear all values from the L3 cache.
    async fn clear(&self) -> ActorCoreResult<()>;
    
    /// Get L3 cache statistics.
    fn get_stats(&self) -> crate::cache::multi_layer::metrics::L3CacheStats;
    
    /// Get the current disk usage.
    fn disk_usage(&self) -> u64;
    
    /// Get the maximum disk capacity.
    fn max_capacity(&self) -> usize;
    
    /// Compact the cache to free up space.
    async fn compact(&self) -> ActorCoreResult<()>;
}

/// Cache entry with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// The cached value
    pub value: serde_json::Value,
    /// Time to live in seconds
    pub ttl: Option<u64>,
    /// Creation timestamp
    pub created_at: u64,
    /// Last access timestamp
    pub last_accessed: u64,
    /// Access count
    pub access_count: u64,
    /// Entry size in bytes
    pub size: usize,
}

impl CacheEntry {
    /// Create a new cache entry.
    pub fn new(value: serde_json::Value, ttl: Option<u64>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let size = serde_json::to_string(&value)
            .map(|s| s.len())
            .unwrap_or(0);

        Self {
            value,
            ttl,
            created_at: now,
            last_accessed: now,
            access_count: 0,
            size,
        }
    }

    /// Check if the entry has expired.
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            if ttl == 0 {
                return true; // TTL of 0 means expired immediately
            }
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now - self.created_at > ttl
        } else {
            false
        }
    }

    /// Update access information.
    pub fn touch(&mut self) {
        self.last_accessed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.access_count += 1;
    }

    /// Get the age of the entry in seconds.
    pub fn age(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now - self.created_at
    }

    /// Get the time since last access in seconds.
    pub fn time_since_last_access(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now - self.last_accessed
    }
}

/// Cache configuration for a specific layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerConfig {
    /// Maximum capacity for this layer
    pub max_capacity: usize,
    /// Eviction policy for this layer
    pub eviction_policy: crate::cache::multi_layer::policy::EvictionPolicy,
    /// Default TTL for entries in this layer
    pub default_ttl: Option<u64>,
    /// Whether to enable compression
    pub enable_compression: bool,
    /// Compression level (1-9)
    pub compression_level: u8,
    /// Whether to enable encryption
    pub enable_encryption: bool,
    /// Encryption key (if encryption is enabled)
    pub encryption_key: Option<String>,
}

impl LayerConfig {
    /// Create a new layer configuration.
    pub fn new(max_capacity: usize) -> Self {
        Self {
            max_capacity,
            eviction_policy: crate::cache::multi_layer::policy::EvictionPolicy::Lru,
            default_ttl: None,
            enable_compression: false,
            compression_level: 6,
            enable_encryption: false,
            encryption_key: None,
        }
    }

    /// Create a configuration for L1 cache.
    pub fn l1(max_capacity: usize) -> Self {
        Self {
            max_capacity,
            eviction_policy: crate::cache::multi_layer::policy::EvictionPolicy::Lru,
            // TODO: Load default TTL from configuration
            default_ttl: Some(300), // 5 minutes
            enable_compression: false,
            compression_level: 0,
            enable_encryption: false,
            encryption_key: None,
        }
    }

    /// Create a configuration for L2 cache.
    pub fn l2(max_capacity: usize) -> Self {
        Self {
            max_capacity,
            eviction_policy: crate::cache::multi_layer::policy::EvictionPolicy::Lru,
            // TODO: Load default TTL from configuration
            default_ttl: Some(3600), // 1 hour
            enable_compression: true,
            // TODO: Load compression level from configuration
            compression_level: 6,
            enable_encryption: false,
            encryption_key: None,
        }
    }

    /// Create a configuration for L3 cache.
    pub fn l3(max_capacity: usize) -> Self {
        Self {
            max_capacity,
            eviction_policy: crate::cache::multi_layer::policy::EvictionPolicy::Lru,
            // TODO: Load default TTL from configuration
            default_ttl: Some(86400), // 24 hours
            enable_compression: true,
            // TODO: Load compression level from configuration
            compression_level: 9,
            enable_encryption: true,
            encryption_key: None,
        }
    }

    /// Validate the configuration.
    pub fn validate(&self) -> ActorCoreResult<()> {
        if self.max_capacity == 0 {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Max capacity must be greater than 0".to_string()
            ));
        }

        if self.compression_level > 9 {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Compression level must be between 1 and 9".to_string()
            ));
        }

        if self.enable_encryption && self.encryption_key.is_none() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Encryption key must be provided when encryption is enabled".to_string()
            ));
        }

        Ok(())
    }
}

impl Default for LayerConfig {
    fn default() -> Self {
        // TODO: Load default capacity from configuration
        Self::new(1000)
    }
}

/// Cache warming strategy for preloading data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarmingStrategy {
    /// No warming - cache starts empty
    None,
    /// Warm on startup - load data during initialization
    OnStartup,
    /// Warm on demand - load data when first accessed
    OnDemand,
    /// Warm periodically - load data at regular intervals
    Periodic(Duration),
    /// Warm based on access patterns
    Adaptive,
}

impl WarmingStrategy {
    /// Get the display name of this warming strategy.
    pub fn display_name(&self) -> &'static str {
        match self {
            WarmingStrategy::None => "None",
            WarmingStrategy::OnStartup => "On Startup",
            WarmingStrategy::OnDemand => "On Demand",
            WarmingStrategy::Periodic(_) => "Periodic",
            WarmingStrategy::Adaptive => "Adaptive",
        }
    }

    /// Check if this strategy requires periodic updates.
    pub fn is_periodic(&self) -> bool {
        matches!(self, WarmingStrategy::Periodic(_) | WarmingStrategy::Adaptive)
    }
}

impl Default for WarmingStrategy {
    fn default() -> Self {
        WarmingStrategy::OnDemand
    }
}