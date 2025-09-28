//! # Element Aggregator Tests
//! 
//! Comprehensive test suite for the ElementAggregator

use element_core::aggregation::{
    ElementAggregator, AggregationStrategy, ElementCache, CacheConfig, 
    EvictionPolicy, AggregatorMetrics, CacheStats, CachedElementData
};
use element_core::unified_registry::UnifiedElementRegistry;
use element_core::contributor::{ElementContributor, ElementContribution, ElementEvent};
use element_core::ElementCoreResult;
use actor_core::Actor;
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;

// Mock contributor for testing
struct MockContributor {
    system_id: String,
    priority: i64,
}

impl MockContributor {
    fn new(system_id: String, priority: i64) -> Self {
        Self { system_id, priority }
    }
}

#[async_trait]
impl ElementContributor for MockContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }

    fn priority(&self) -> i64 {
        self.priority
    }

    async fn contribute_element_stats(
        &self,
        _actor: &Actor,
        _element_type: &str,
    ) -> ElementCoreResult<ElementContribution> {
        let mut contribution = ElementContribution::new(
            self.system_id.clone(),
            "fire".to_string(),
            "1.0.0".to_string(),
        );
        contribution.add_primary_stat("mastery_level".to_string(), 10.0);
        contribution.add_derived_stat("fire_power".to_string(), 100.0);
        Ok(contribution)
    }

    async fn handle_element_event(&self, _event: &ElementEvent) -> ElementCoreResult<()> {
        Ok(())
    }

    fn get_metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata
    }
}

fn create_test_actor() -> Actor {
    Actor {
        id: "test_actor".to_string(),
        version: 1,
        // Add other required fields as needed
    }
}

#[tokio::test]
async fn test_aggregator_creation() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    let aggregator = ElementAggregator::new(registry);
    
    let metrics = aggregator.get_metrics();
    assert_eq!(metrics.total_operations, 0);
    assert_eq!(metrics.successful_operations, 0);
    assert_eq!(metrics.failed_operations, 0);
}

#[tokio::test]
async fn test_aggregator_with_cache_config() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    let cache_config = CacheConfig {
        enable_cache: true,
        max_cache_entries: 100,
        cache_ttl_seconds: 300,
        eviction_policy: EvictionPolicy::Lru,
    };
    
    let aggregator = ElementAggregator::with_cache_config(registry, cache_config);
    let cache_metrics = aggregator.cache.get_metrics();
    assert_eq!(cache_metrics.size, 0);
}

#[tokio::test]
async fn test_strategy_management() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    let aggregator = ElementAggregator::new(registry);
    
    // Set strategy
    aggregator.set_strategy("fire_power", AggregationStrategy::Sum);
    
    // Get strategy
    let strategy = aggregator.get_strategy("fire_power");
    match strategy {
        AggregationStrategy::Sum => {}, // Expected
        _ => panic!("Expected Sum strategy"),
    }
    
    // Test default strategy for unknown stat
    let default_strategy = aggregator.get_strategy("unknown_stat");
    match default_strategy {
        AggregationStrategy::Sum => {}, // Expected default
        _ => panic!("Expected Sum as default strategy"),
    }
}

#[tokio::test]
async fn test_cache_operations() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    let aggregator = ElementAggregator::new(registry);
    
    // Test cache clear
    aggregator.clear_cache().await.unwrap();
    
    let cache_metrics = aggregator.cache.get_metrics();
    assert_eq!(cache_metrics.size, 0);
}

#[tokio::test]
async fn test_aggregation_strategies() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    let aggregator = ElementAggregator::new(registry);
    
    // Test Sum strategy
    aggregator.set_strategy("sum_stat", AggregationStrategy::Sum);
    let sum_strategy = aggregator.get_strategy("sum_stat");
    match sum_strategy {
        AggregationStrategy::Sum => {},
        _ => panic!("Expected Sum strategy"),
    }
    
    // Test Multiply strategy
    aggregator.set_strategy("multiply_stat", AggregationStrategy::Multiply);
    let multiply_strategy = aggregator.get_strategy("multiply_stat");
    match multiply_strategy {
        AggregationStrategy::Multiply => {},
        _ => panic!("Expected Multiply strategy"),
    }
    
    // Test Max strategy
    aggregator.set_strategy("max_stat", AggregationStrategy::Max);
    let max_strategy = aggregator.get_strategy("max_stat");
    match max_strategy {
        AggregationStrategy::Max => {},
        _ => panic!("Expected Max strategy"),
    }
    
    // Test Min strategy
    aggregator.set_strategy("min_stat", AggregationStrategy::Min);
    let min_strategy = aggregator.get_strategy("min_stat");
    match min_strategy {
        AggregationStrategy::Min => {},
        _ => panic!("Expected Min strategy"),
    }
    
    // Test Average strategy
    aggregator.set_strategy("avg_stat", AggregationStrategy::Average);
    let avg_strategy = aggregator.get_strategy("avg_stat");
    match avg_strategy {
        AggregationStrategy::Average => {},
        _ => panic!("Expected Average strategy"),
    }
    
    // Test First strategy
    aggregator.set_strategy("first_stat", AggregationStrategy::First);
    let first_strategy = aggregator.get_strategy("first_stat");
    match first_strategy {
        AggregationStrategy::First => {},
        _ => panic!("Expected First strategy"),
    }
    
    // Test Last strategy
    aggregator.set_strategy("last_stat", AggregationStrategy::Last);
    let last_strategy = aggregator.get_strategy("last_stat");
    match last_strategy {
        AggregationStrategy::Last => {},
        _ => panic!("Expected Last strategy"),
    }
}

#[tokio::test]
async fn test_cache_stats() {
    let cache = ElementCache::new();
    let stats = cache.get_metrics();
    
    assert_eq!(stats.hit_count, 0);
    assert_eq!(stats.miss_count, 0);
    assert_eq!(stats.eviction_count, 0);
    assert_eq!(stats.size, 0);
}

#[tokio::test]
async fn test_cache_config() {
    let config = CacheConfig::default();
    assert!(config.enable_cache);
    assert_eq!(config.max_cache_entries, 1000);
    assert_eq!(config.cache_ttl_seconds, 300);
    match config.eviction_policy {
        EvictionPolicy::Lru => {},
        _ => panic!("Expected LRU eviction policy"),
    }
}

#[tokio::test]
async fn test_eviction_policies() {
    let lru_policy = EvictionPolicy::Lru;
    let lfu_policy = EvictionPolicy::Lfu;
    let fifo_policy = EvictionPolicy::Fifo;
    let random_policy = EvictionPolicy::Random;
    
    // Test that all policies can be created
    match lru_policy {
        EvictionPolicy::Lru => {},
        _ => panic!("Expected LRU policy"),
    }
    
    match lfu_policy {
        EvictionPolicy::Lfu => {},
        _ => panic!("Expected LFU policy"),
    }
    
    match fifo_policy {
        EvictionPolicy::Fifo => {},
        _ => panic!("Expected FIFO policy"),
    }
    
    match random_policy {
        EvictionPolicy::Random => {},
        _ => panic!("Expected Random policy"),
    }
}

#[tokio::test]
async fn test_aggregator_metrics() {
    let metrics = AggregatorMetrics::new();
    assert_eq!(metrics.total_operations, 0);
    assert_eq!(metrics.successful_operations, 0);
    assert_eq!(metrics.failed_operations, 0);
    assert_eq!(metrics.average_aggregation_time_ms, 0.0);
    assert_eq!(metrics.cache_hit_rate, 0.0);
}

#[tokio::test]
async fn test_cached_element_data() {
    let mut stats = HashMap::new();
    stats.insert("fire_power".to_string(), 100.0);
    
    let cached_data = CachedElementData {
        stats,
        created_at: Utc::now(),
        ttl_seconds: 300,
    };
    
    assert!(cached_data.is_valid());
    assert_eq!(cached_data.stats.get("fire_power"), Some(&100.0));
}

#[tokio::test]
async fn test_cached_element_data_expired() {
    let mut stats = HashMap::new();
    stats.insert("fire_power".to_string(), 100.0);
    
    let cached_data = CachedElementData {
        stats,
        created_at: Utc::now() - chrono::Duration::seconds(400), // 400 seconds ago
        ttl_seconds: 300, // TTL is 300 seconds
    };
    
    assert!(!cached_data.is_valid());
}

#[tokio::test]
async fn test_cache_operations_detailed() {
    let cache = ElementCache::new();
    
    // Test storing data
    let mut stats = HashMap::new();
    stats.insert("fire_power".to_string(), 100.0);
    cache.store("test_key", &stats).await.unwrap();
    
    // Test retrieving data
    let retrieved = cache.get("test_key").await.unwrap();
    assert!(retrieved.is_some());
    let cached_data = retrieved.unwrap();
    assert_eq!(cached_data.stats.get("fire_power"), Some(&100.0));
    
    // Test cache metrics
    let metrics = cache.get_metrics();
    assert!(metrics.size > 0);
}

#[tokio::test]
async fn test_cache_clear() {
    let cache = ElementCache::new();
    
    // Add some data
    let mut stats = HashMap::new();
    stats.insert("fire_power".to_string(), 100.0);
    cache.store("test_key", &stats).await.unwrap();
    
    // Verify data exists
    let metrics_before = cache.get_metrics();
    assert!(metrics_before.size > 0);
    
    // Clear cache
    cache.clear().await.unwrap();
    
    // Verify cache is cleared
    let metrics_after = cache.get_metrics();
    assert_eq!(metrics_after.size, 0);
}

#[tokio::test]
async fn test_aggregation_strategy_clone() {
    // Test that all strategies can be cloned
    let sum_strategy = AggregationStrategy::Sum;
    let cloned_sum = sum_strategy.clone();
    match cloned_sum {
        AggregationStrategy::Sum => {},
        _ => panic!("Expected Sum strategy"),
    }
    
    let multiply_strategy = AggregationStrategy::Multiply;
    let cloned_multiply = multiply_strategy.clone();
    match cloned_multiply {
        AggregationStrategy::Multiply => {},
        _ => panic!("Expected Multiply strategy"),
    }
    
    // Test custom strategy (should fallback to Sum)
    let custom_strategy = AggregationStrategy::Custom(Box::new(|_| 42.0));
    let cloned_custom = custom_strategy.clone();
    match cloned_custom {
        AggregationStrategy::Sum => {}, // Should fallback to Sum
        _ => panic!("Expected Sum strategy as fallback"),
    }
}

#[tokio::test]
async fn test_aggregation_strategy_debug() {
    // Test that all strategies can be debug printed
    let strategies = vec![
        AggregationStrategy::Sum,
        AggregationStrategy::Multiply,
        AggregationStrategy::Max,
        AggregationStrategy::Min,
        AggregationStrategy::Average,
        AggregationStrategy::First,
        AggregationStrategy::Last,
        AggregationStrategy::Custom(Box::new(|_| 42.0)),
    ];
    
    for strategy in strategies {
        let debug_str = format!("{:?}", strategy);
        assert!(!debug_str.is_empty());
    }
}
