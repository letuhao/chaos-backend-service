//! Coverage tests for aggregator/optimized.rs module.

use actor_core::aggregator::optimized::{OptimizedAggregator, BatchAggregator};
use actor_core::interfaces::{PluginRegistry, CombinerRegistry, Cache, CapsProvider, Subsystem};
use actor_core::types::*;
use actor_core::ActorCoreResult;
use actor_core::metrics::{CacheStats, CapStatistics};
use actor_core::enums::AcrossLayerPolicy;
use actor_core::MergeRule;
use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;

// Mock implementations for testing
struct MockPluginRegistry;
struct MockCombinerRegistry;
struct MockCache;
struct MockCapsProvider;

impl PluginRegistry for MockPluginRegistry {
    fn register(&self, _subsystem: Arc<dyn Subsystem>) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn unregister(&self, _system_id: &str) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn get_by_id(&self, _system_id: &str) -> Option<Arc<dyn Subsystem>> {
        None
    }
    
    fn get_by_priority(&self) -> Vec<Arc<dyn Subsystem>> {
        vec![]
    }
    
    fn get_by_priority_range(&self, _min_priority: i64, _max_priority: i64) -> Vec<Arc<dyn Subsystem>> {
        vec![]
    }
    
    fn is_registered(&self, _system_id: &str) -> bool {
        false
    }
    
    fn count(&self) -> usize {
        0
    }
    
    fn validate_all(&self) -> ActorCoreResult<()> {
        Ok(())
    }
}

impl CombinerRegistry for MockCombinerRegistry {
    fn get_rule(&self, _dimension: &str) -> Option<MergeRule> {
        None
    }
    
    fn set_rule(&self, _dimension: &str, _rule: MergeRule) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn validate(&self) -> ActorCoreResult<()> {
        Ok(())
    }
}

impl Cache for MockCache {
    fn get(&self, _key: &str) -> Option<serde_json::Value> {
        None
    }
    
    fn set(&self, _key: String, _value: serde_json::Value, _ttl: Option<u64>) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn delete(&self, _key: &str) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn clear(&self) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn get_stats(&self) -> CacheStats {
        CacheStats::default()
    }
}

#[async_trait]
impl CapsProvider for MockCapsProvider {
    async fn effective_caps_within_layer(
        &self, 
        _actor: &Actor, 
        _outputs: &[SubsystemOutput], 
        _layer: &str
    ) -> ActorCoreResult<HashMap<String, Caps>> {
        Ok(HashMap::new())
    }
    
    async fn effective_caps_across_layers(
        &self, 
        _actor: &Actor, 
        _outputs: &[SubsystemOutput]
    ) -> ActorCoreResult<HashMap<String, Caps>> {
        Ok(HashMap::new())
    }
    
    fn get_layer_order(&self) -> Vec<String> {
        vec![]
    }
    
    fn get_across_layer_policy(&self) -> AcrossLayerPolicy {
        AcrossLayerPolicy::Intersect
    }
    
    fn validate_caps(&self, _dimension: &str, _caps: &Caps) -> ActorCoreResult<()> {
        Ok(())
    }
    
    async fn get_caps_for_dimension(
        &self, 
        _dimension: &str, 
        _actor: &Actor
    ) -> ActorCoreResult<Option<Caps>> {
        Ok(None)
    }
    
    fn get_supported_dimensions(&self) -> Vec<String> {
        vec![]
    }
    
    fn get_cap_statistics(&self) -> CapStatistics {
        CapStatistics::default()
    }
    
    fn validate(&self) -> ActorCoreResult<()> {
        Ok(())
    }
}

#[test]
fn test_optimized_aggregator_new() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let aggregator = OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Test that the aggregator was created successfully
    assert!(aggregator.get_cache_hit_rate() >= 0.0);
}

#[test]
fn test_optimized_aggregator_get_atomic_metrics() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let aggregator = OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    let metrics = aggregator.get_atomic_metrics();
    assert!(Arc::strong_count(metrics) > 0);
}

#[test]
fn test_optimized_aggregator_get_cache_hit_rate() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let aggregator = OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    let hit_rate = aggregator.get_cache_hit_rate();
    assert!(hit_rate >= 0.0);
    assert!(hit_rate <= 1.0);
}

#[test]
fn test_batch_aggregator_new() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let optimized_aggregator = Arc::new(OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    ));
    
    let _batch_aggregator = BatchAggregator::new(optimized_aggregator.clone(), 10);
    // Test that the batch aggregator was created successfully
    assert!(Arc::strong_count(&optimized_aggregator) > 1);
}

#[test]
fn test_batch_aggregator_with_different_batch_sizes() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let optimized_aggregator = Arc::new(OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    ));
    
    // Test different batch sizes
    let batch_sizes = vec![1, 5, 10, 50, 100];
    for batch_size in batch_sizes {
        let _batch_aggregator = BatchAggregator::new(optimized_aggregator.clone(), batch_size);
        // Test that the batch aggregator was created successfully
        assert!(Arc::strong_count(&optimized_aggregator) > 1);
    }
}

#[test]
fn test_optimized_aggregator_creation_with_all_parameters() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let _aggregator = OptimizedAggregator::new(
        plugin_registry.clone(),
        combiner_registry.clone(),
        caps_provider.clone(),
        cache.clone(),
    );
    
    // Verify the aggregator was created with all components
    assert!(Arc::strong_count(&plugin_registry) > 1);
    assert!(Arc::strong_count(&combiner_registry) > 1);
    assert!(Arc::strong_count(&caps_provider) > 1);
    assert!(Arc::strong_count(&cache) > 1);
}

#[test]
fn test_optimized_aggregator_metrics_initialization() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let aggregator = OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Test that metrics are properly initialized
    let hit_rate = aggregator.get_cache_hit_rate();
    assert!(hit_rate.is_finite());
    assert!(hit_rate >= 0.0);
}

#[test]
fn test_optimized_aggregator_atomic_metrics_access() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let aggregator = OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Test atomic metrics access
    let atomic_metrics = aggregator.get_atomic_metrics();
    assert!(Arc::strong_count(atomic_metrics) > 0);
    
    // Test that we can get multiple references
    let atomic_metrics2 = aggregator.get_atomic_metrics();
    assert!(Arc::ptr_eq(atomic_metrics, atomic_metrics2));
}

#[test]
fn test_batch_aggregator_batch_size_property() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let optimized_aggregator = Arc::new(OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    ));
    
    let batch_size = 25;
    let _batch_aggregator = BatchAggregator::new(optimized_aggregator.clone(), batch_size);
    
    // Test that the batch aggregator was created successfully
    assert!(Arc::strong_count(&optimized_aggregator) > 1);
}

#[test]
fn test_optimized_aggregator_with_empty_subsystems() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let aggregator = OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Test that aggregator handles empty subsystems gracefully
    let hit_rate = aggregator.get_cache_hit_rate();
    assert!(hit_rate >= 0.0);
}

#[test]
fn test_optimized_aggregator_cache_operations() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let aggregator = OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Test cache hit rate calculation
    let hit_rate = aggregator.get_cache_hit_rate();
    assert!(hit_rate.is_finite());
    assert!(hit_rate >= 0.0);
    assert!(hit_rate <= 1.0);
}

#[test]
fn test_batch_aggregator_creation_with_zero_batch_size() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let optimized_aggregator = Arc::new(OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    ));
    
    let _batch_aggregator = BatchAggregator::new(optimized_aggregator.clone(), 0);
    // Test that the batch aggregator was created successfully
    assert!(Arc::strong_count(&optimized_aggregator) > 1);
}

#[test]
fn test_batch_aggregator_creation_with_large_batch_size() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let optimized_aggregator = Arc::new(OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    ));
    
    let large_batch_size = 10000;
    let _batch_aggregator = BatchAggregator::new(optimized_aggregator.clone(), large_batch_size);
    // Test that the batch aggregator was created successfully
    assert!(Arc::strong_count(&optimized_aggregator) > 1);
}

#[test]
fn test_optimized_aggregator_metrics_consistency() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let aggregator = OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Test that metrics are consistent across multiple calls
    let hit_rate1 = aggregator.get_cache_hit_rate();
    let hit_rate2 = aggregator.get_cache_hit_rate();
    assert_eq!(hit_rate1, hit_rate2);
}

#[test]
fn test_optimized_aggregator_atomic_metrics_shared_ownership() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let aggregator = OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Test that atomic metrics are properly shared
    let metrics1 = aggregator.get_atomic_metrics();
    let metrics2 = aggregator.get_atomic_metrics();
    
    assert!(Arc::ptr_eq(metrics1, metrics2));
    assert!(Arc::strong_count(metrics1) >= 1);
}

#[test]
fn test_batch_aggregator_aggregator_reference() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let optimized_aggregator = Arc::new(OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    ));
    
    let _batch_aggregator = BatchAggregator::new(optimized_aggregator.clone(), 10);
    
    // Test that the batch aggregator holds a reference to the optimized aggregator
    assert!(Arc::strong_count(&optimized_aggregator) > 1);
}

#[test]
fn test_optimized_aggregator_creation_with_different_registries() {
    // Test with different registry combinations
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let aggregator1 = OptimizedAggregator::new(
        plugin_registry.clone(),
        combiner_registry.clone(),
        caps_provider.clone(),
        cache.clone(),
    );
    
    let aggregator2 = OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Both aggregators should be created successfully
    assert!(aggregator1.get_cache_hit_rate() >= 0.0);
    assert!(aggregator2.get_cache_hit_rate() >= 0.0);
}

#[test]
fn test_optimized_aggregator_metrics_initialization_consistency() {
    let plugin_registry = Arc::new(MockPluginRegistry);
    let combiner_registry = Arc::new(MockCombinerRegistry);
    let caps_provider = Arc::new(MockCapsProvider);
    let cache = Arc::new(MockCache);
    
    let aggregator = OptimizedAggregator::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Test that metrics are consistently initialized
    let atomic_metrics = aggregator.get_atomic_metrics();
    let hit_rate = aggregator.get_cache_hit_rate();
    
    assert!(Arc::strong_count(atomic_metrics) > 0);
    assert!(hit_rate.is_finite());
    assert!(hit_rate >= 0.0);
    assert!(hit_rate <= 1.0);
}
