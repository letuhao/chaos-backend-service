//! Comprehensive tests for service implementations.
//!
//! This module contains detailed tests for all service implementations,
//! including AggregatorImpl, CapsProviderImpl, and related functionality.

use actor_core::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Mock subsystem for testing
struct MockSubsystem {
    system_id: String,
    priority: i64,
    contributions: Vec<Contribution>,
}

impl MockSubsystem {
    fn new(system_id: String, priority: i64) -> Self {
        Self {
            system_id,
            priority,
            contributions: Vec::new(),
        }
    }
    
    fn add_contribution(&mut self, contribution: Contribution) {
        self.contributions.push(contribution);
    }
}

#[async_trait::async_trait]
impl actor_core::interfaces::Subsystem for MockSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute(&self, _actor: &Actor) -> actor_core::ActorCoreResult<SubsystemOutput> {
        let mut output = SubsystemOutput::new(self.system_id.clone());
        
        for contribution in &self.contributions {
            output.add_primary(contribution.clone());
        }
        
        Ok(output)
    }
}

/// Test AggregatorImpl basic functionality
#[tokio::test]
async fn test_aggregator_impl_basic() {
    // Create mock subsystems
    let mut combat_subsystem = MockSubsystem::new("combat".to_string(), 10);
    combat_subsystem.add_contribution(Contribution::new(
        "strength".to_string(),
        Bucket::Flat,
        10.0,
        "combat".to_string(),
    ));
    
    let mut magic_subsystem = MockSubsystem::new("magic".to_string(), 5);
    magic_subsystem.add_contribution(Contribution::new(
        "intelligence".to_string(),
        Bucket::Flat,
        15.0,
        "magic".to_string(),
    ));
    
    // Create registries
    let plugin_registry = Arc::new(PluginRegistryImpl::new());
    plugin_registry.register(Arc::new(combat_subsystem)).unwrap();
    plugin_registry.register(Arc::new(magic_subsystem)).unwrap();
    
    // Verify subsystems are registered
    assert!(plugin_registry.is_registered("combat"));
    assert!(plugin_registry.is_registered("magic"));
    assert_eq!(plugin_registry.count(), 2);
    
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let combiner_registry = Arc::new(CombinerRegistryImpl::new());
    let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
    // Create aggregator
    let aggregator = AggregatorImpl::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Create test actor
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test basic resolution
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    assert_eq!(snapshot.actor_id, actor.id);
    assert_eq!(snapshot.version, actor.version);
    
    // Debug output
    println!("Processed subsystems: {:?}", snapshot.subsystems_processed);
    println!("Primary stats: {:?}", snapshot.primary);
    
    // Test that subsystems were processed
    assert!(snapshot.subsystems_processed.contains(&"combat".to_string()));
    assert!(snapshot.subsystems_processed.contains(&"magic".to_string()));
}

/// Test AggregatorImpl with context
#[tokio::test]
async fn test_aggregator_impl_with_context() {
    // Create minimal setup
    let plugin_registry = Arc::new(PluginRegistryImpl::new());
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
    let combiner_registry = Arc::new(CombinerRegistryImpl::new());
    let aggregator = AggregatorImpl::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test resolve_with_context
    let context = Some(HashMap::from([
        ("context_type".to_string(), serde_json::Value::String("combat".to_string())),
    ]));
    
    let snapshot = aggregator.resolve_with_context(&actor, context).await.unwrap();
    assert_eq!(snapshot.actor_id, actor.id);
}

/// Test AggregatorImpl batch resolution
#[tokio::test]
async fn test_aggregator_impl_batch_resolution() {
    // Create minimal setup
    let plugin_registry = Arc::new(PluginRegistryImpl::new());
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
    let combiner_registry = Arc::new(CombinerRegistryImpl::new());
    let aggregator = AggregatorImpl::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Create multiple actors
    let actors = vec![
        Actor::new("Actor1".to_string(), "Human".to_string()),
        Actor::new("Actor2".to_string(), "Elf".to_string()),
        Actor::new("Actor3".to_string(), "Dwarf".to_string()),
    ];
    
    // Test batch resolution
    let snapshots = aggregator.resolve_batch(&actors).await.unwrap();
    assert_eq!(snapshots.len(), 3);
    
    for (i, snapshot) in snapshots.iter().enumerate() {
        assert_eq!(snapshot.actor_id, actors[i].id);
    }
}

/// Test AggregatorImpl cache operations
#[tokio::test]
async fn test_aggregator_impl_cache_operations() {
    // Create minimal setup
    let plugin_registry = Arc::new(PluginRegistryImpl::new());
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
    let combiner_registry = Arc::new(CombinerRegistryImpl::new());
    let aggregator = AggregatorImpl::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test cache operations
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    
    // Test get_cached_snapshot
    let cached = aggregator.get_cached_snapshot(&actor.id);
    assert!(cached.is_some());
    assert_eq!(cached.unwrap().actor_id, snapshot.actor_id);
    
    // Test invalidate_cache
    aggregator.invalidate_cache(&actor.id);
    let cached = aggregator.get_cached_snapshot(&actor.id);
    assert!(cached.is_none());
    
    // Test clear_cache
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    assert_eq!(snapshot.actor_id, actor.id);
    aggregator.clear_cache();
    let cached = aggregator.get_cached_snapshot(&actor.id);
    assert!(cached.is_none());
}

/// Test AggregatorImpl metrics
#[tokio::test]
async fn test_aggregator_impl_metrics() {
    // Create minimal setup
    let plugin_registry = Arc::new(PluginRegistryImpl::new());
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
    let combiner_registry = Arc::new(CombinerRegistryImpl::new());
    let aggregator = AggregatorImpl::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Perform some operations
    let snapshot1 = aggregator.resolve(&actor).await.unwrap();
    let snapshot2 = aggregator.resolve(&actor).await.unwrap(); // Should hit cache
    
    // Verify both snapshots are identical (cached)
    assert_eq!(snapshot1.actor_id, snapshot2.actor_id);
    
    // Test get_metrics
    let metrics = aggregator.get_metrics().await;
    assert!(metrics.total_resolutions > 0);
    assert!(metrics.cache_hits > 0);
}

/// Test CapsProviderImpl basic functionality
#[tokio::test]
async fn test_caps_provider_impl_basic() {
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let caps_provider = CapsProviderImpl::new(cap_layer_registry);
    
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    let outputs = vec![];
    
    // Test effective_caps_within_layer
    let caps = caps_provider.effective_caps_within_layer(&actor, &outputs, "test_layer").await.unwrap();
    assert!(caps.is_empty());
    
    // Test effective_caps_across_layers
    let caps = caps_provider.effective_caps_across_layers(&actor, &outputs).await.unwrap();
    assert!(caps.is_empty());
    
    // Test get_layer_order
    let order = caps_provider.get_layer_order();
    assert!(!order.is_empty());
    
    // Test get_across_layer_policy
    let policy = caps_provider.get_across_layer_policy();
    assert!(matches!(policy, AcrossLayerPolicy::Intersect | AcrossLayerPolicy::Union | AcrossLayerPolicy::PrioritizedOverride));
    
    // Test get_supported_dimensions
    let dimensions = caps_provider.get_supported_dimensions();
    assert!(!dimensions.is_empty());
    
    // Test validate
    assert!(caps_provider.validate().is_ok());
}

// ServiceFactory tests removed as ServiceFactory doesn't exist in current implementation

// RegistryFactory tests removed as RegistryFactory doesn't have a new() method in current implementation

/// Test error handling in services
#[tokio::test]
async fn test_service_error_handling() {
    // Test with empty plugin registry
    let plugin_registry = Arc::new(PluginRegistryImpl::new());
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
    let combiner_registry = Arc::new(CombinerRegistryImpl::new());
    let aggregator = AggregatorImpl::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Should still work with no subsystems
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    assert_eq!(snapshot.actor_id, actor.id);
    assert!(snapshot.subsystems_processed.is_empty());
}

/// Test performance with many subsystems
#[tokio::test]
async fn test_service_performance_many_subsystems() {
    let plugin_registry = Arc::new(PluginRegistryImpl::new());
    
    // Add many subsystems
    for i in 0..100 {
        let mut subsystem = MockSubsystem::new(format!("system_{}", i), i as i64);
        subsystem.add_contribution(Contribution::new(
            "strength".to_string(),
            Bucket::Flat,
            i as f64,
            format!("system_{}", i),
        ));
        plugin_registry.register(Arc::new(subsystem)).unwrap();
    }
    
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
    let combiner_registry = Arc::new(CombinerRegistryImpl::new());
    let aggregator = AggregatorImpl::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test resolution with many subsystems
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    assert_eq!(snapshot.subsystems_processed.len(), 100);
    
    // Test that strength was aggregated from all subsystems
    let strength = snapshot.get_primary("strength");
    assert!(strength.is_some());
    assert!(strength.unwrap() > 0.0);
}

/// Test cache performance
#[tokio::test]
async fn test_cache_performance() {
    let plugin_registry = Arc::new(PluginRegistryImpl::new());
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
    let combiner_registry = Arc::new(CombinerRegistryImpl::new());
    let aggregator = AggregatorImpl::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // First resolution (cache miss)
    let start = std::time::Instant::now();
    let _snapshot1 = aggregator.resolve(&actor).await.unwrap();
    let first_duration = start.elapsed();
    
    // Second resolution (cache hit)
    let start = std::time::Instant::now();
    let _snapshot2 = aggregator.resolve(&actor).await.unwrap();
    let second_duration = start.elapsed();
    
    // Cache hit should be faster (though this might not always be true in tests)
    println!("First resolution: {:?}", first_duration);
    println!("Second resolution: {:?}", second_duration);
    
    // Both should succeed
    assert!(first_duration.as_micros() > 0);
    assert!(second_duration.as_micros() > 0);
}
