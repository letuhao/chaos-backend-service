//! Multi-layer cache system implementation.
//!
//! This module provides a sophisticated multi-layer cache system with L1 (in-memory),
//! L2 (memory-mapped), and L3 (persistent) cache layers. It includes advanced features
//! like cache warming, eviction policies, and comprehensive metrics.

// Re-export submodules
pub mod policy;
pub mod metrics;
pub mod layers;
pub mod manager;
pub mod backends;
pub mod warming;

// Re-export key types for convenience
pub use policy::EvictionPolicy;
pub use metrics::{L1CacheStats, L2CacheStats, L3CacheStats, CacheLayer, CacheWarmingStats};
pub use layers::{L1Cache, L2Cache, L3Cache, CacheEntry, LayerConfig};
pub use manager::MultiLayerCacheManager;
pub use backends::LockFreeL1Cache;
#[cfg(feature = "memory-mapped")]
pub use backends::MemoryMappedL2Cache;
pub use backends::PersistentL3Cache;
pub use warming::{CacheWarmingStrategy, PredefinedDataWarming, PredictiveWarming, ScheduledWarming, CacheWarmingManager};

// Legacy re-exports for backward compatibility
pub use manager::MultiLayerConfig;
pub use metrics::MultiLayerStats;

// Re-export the main cache trait
pub use crate::interfaces::Cache;