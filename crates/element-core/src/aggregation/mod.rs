//! # Aggregation Module
//! 
//! This module provides high-performance aggregation functionality for elemental stats,
//! including caching, metrics collection, and performance optimization.
//! 
//! ## Components
//! 
//! ### `ElementAggregator`
//! - **Stats Aggregation**: Combines elemental stats from multiple sources
//! - **Caching**: High-performance caching with configurable eviction policies
//! - **Metrics**: Performance monitoring and statistics collection
//! - **Thread Safety**: Safe concurrent access to aggregated data
//! 
//! ### `AggregationStrategy`
//! - **Strategy Pattern**: Different aggregation algorithms
//! - **Configurable**: Runtime strategy selection
//! - **Extensible**: Easy to add new aggregation methods
//! 
//! ### `ElementCache`
//! - **High Performance**: O(1) cache operations
//! - **Eviction Policies**: LRU, LFU, and custom eviction strategies
//! - **Memory Management**: Configurable cache size limits
//! - **Statistics**: Cache hit/miss ratios and performance metrics
//! 
//! ### `CacheStats`
//! - **Performance Metrics**: Hit rates, miss rates, eviction counts
//! - **Memory Usage**: Cache size and memory consumption
//! - **Performance Analysis**: Access patterns and optimization insights
//! 
//! ## Key Features
//! 
//! - **High Performance**: Optimized for game loop requirements
//! - **Flexible Caching**: Multiple eviction policies and strategies
//! - **Comprehensive Metrics**: Detailed performance monitoring
//! - **Thread Safety**: Safe for concurrent access patterns
//! - **Configurable**: Runtime configuration of aggregation behavior
//! 
//! ## Usage Examples
//! 
//! ### Basic Aggregation
//! ```rust
//! let aggregator = ElementAggregator::new();
//! let stats = aggregator.aggregate_stats(&sources)?;
//! ```
//! 
//! ### Caching
//! ```rust
//! let cache = ElementCache::new(CacheConfig::default());
//! cache.insert("fire_stats", stats);
//! let cached_stats = cache.get("fire_stats");
//! ```
//! 
//! ### Metrics Collection
//! ```rust
//! let metrics = aggregator.get_metrics();
//! println!("Cache hit rate: {:.2}%", metrics.cache_hit_rate * 100.0);
//! ```

pub mod element_aggregator;

pub use element_aggregator::*;
