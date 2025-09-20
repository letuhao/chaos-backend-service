//! Cache eviction policies for the multi-layer cache system.
//!
//! This module defines the eviction policies used by different cache layers
//! to manage memory usage and determine which items to remove when the cache is full.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Eviction policies for cache layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvictionPolicy {
    /// Least Recently Used - removes the least recently accessed item
    Lru,
    /// Least Frequently Used - removes the least frequently accessed item
    Lfu,
    /// First In, First Out - removes the oldest item by insertion time
    Fifo,
    /// Random - removes a random item
    Random,
}

impl EvictionPolicy {
    /// Get the display name of this eviction policy.
    pub fn display_name(&self) -> &'static str {
        match self {
            EvictionPolicy::Lru => "LRU",
            EvictionPolicy::Lfu => "LFU",
            EvictionPolicy::Fifo => "FIFO",
            EvictionPolicy::Random => "Random",
        }
    }

    /// Get the description of this eviction policy.
    pub fn description(&self) -> &'static str {
        match self {
            EvictionPolicy::Lru => "Least Recently Used - removes the least recently accessed item",
            EvictionPolicy::Lfu => "Least Frequently Used - removes the least frequently accessed item",
            EvictionPolicy::Fifo => "First In, First Out - removes the oldest item by insertion time",
            EvictionPolicy::Random => "Random - removes a random item",
        }
    }

    /// Check if this policy is suitable for the given cache size.
    pub fn is_suitable_for_size(&self, cache_size: usize) -> bool {
        match self {
            EvictionPolicy::Lru => cache_size > 0,
            EvictionPolicy::Lfu => cache_size > 0,
            EvictionPolicy::Fifo => cache_size > 0,
            EvictionPolicy::Random => cache_size > 0,
        }
    }

    /// Get the memory overhead of this policy.
    pub fn memory_overhead(&self) -> usize {
        match self {
            // TODO: Load memory overhead values from configuration
            EvictionPolicy::Lru => 8, // timestamp per item
            EvictionPolicy::Lfu => 8, // counter per item
            EvictionPolicy::Fifo => 8, // insertion order per item
            EvictionPolicy::Random => 0, // no additional overhead
        }
    }
}

impl fmt::Display for EvictionPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl Default for EvictionPolicy {
    fn default() -> Self {
        EvictionPolicy::Lru
    }
}