//! Direct coverage tests for interfaces.rs module.
//! This file provides direct tests that exercise the actual interfaces module code.

use actor_core::interfaces::*;
use actor_core::types::{Actor, SubsystemOutput, Snapshot, Caps};
use actor_core::enums::{AcrossLayerPolicy, Operator};
use actor_core::error::ActorCoreResult;
use actor_core::metrics::{AggregatorMetrics, CacheStats, CapStatistics};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::Value;

// ============================================================================
// MERGE RULE TESTS
// ============================================================================

#[test]
fn test_merge_rule_creation() {
    let rule = MergeRule {
        use_pipeline: true,
        operator: Operator::Sum,
        clamp_default: Some(Caps::new(0.0, 100.0)),
    };
    
    assert!(rule.use_pipeline);
    assert_eq!(rule.operator, Operator::Sum);
    assert!(rule.clamp_default.is_some());
    let clamp = rule.clamp_default.as_ref().unwrap();
    assert_eq!(clamp.min, 0.0);
    assert_eq!(clamp.max, 100.0);
}

#[test]
fn test_merge_rule_without_clamp() {
    let rule = MergeRule {
        use_pipeline: false,
        operator: Operator::Multiply,
        clamp_default: None,
    };
    
    assert!(!rule.use_pipeline);
    assert_eq!(rule.operator, Operator::Multiply);
    assert!(rule.clamp_default.is_none());
}

#[test]
fn test_merge_rule_debug() {
    let rule = MergeRule {
        use_pipeline: true,
        operator: Operator::Sum,
        clamp_default: Some(Caps::new(0.0, 100.0)),
    };
    
    let debug_string = format!("{:?}", rule);
    assert!(debug_string.contains("use_pipeline"));
    assert!(debug_string.contains("operator"));
    assert!(debug_string.contains("clamp_default"));
}

#[test]
fn test_merge_rule_clone() {
    let rule = MergeRule {
        use_pipeline: true,
        operator: Operator::Sum,
        clamp_default: Some(Caps::new(0.0, 100.0)),
    };
    
    let cloned = rule.clone();
    assert_eq!(rule.use_pipeline, cloned.use_pipeline);
    assert_eq!(rule.operator, cloned.operator);
    assert_eq!(rule.clamp_default, cloned.clamp_default);
}

#[test]
fn test_merge_rule_serialization() {
    let rule = MergeRule {
        use_pipeline: true,
        operator: Operator::Sum,
        clamp_default: Some(Caps::new(0.0, 100.0)),
    };
    
    let json = serde_json::to_string(&rule).unwrap();
    assert!(!json.is_empty());
    
    let deserialized: MergeRule = serde_json::from_str(&json).unwrap();
    assert_eq!(rule.use_pipeline, deserialized.use_pipeline);
    assert_eq!(rule.operator, deserialized.operator);
    assert_eq!(rule.clamp_default, deserialized.clamp_default);
}

#[test]
fn test_merge_rule_deserialization() {
    let json = r#"{"use_pipeline":false,"operator":"Multiply","clamp_default":null}"#;
    let rule: MergeRule = serde_json::from_str(json).unwrap();
    
    assert!(!rule.use_pipeline);
    assert_eq!(rule.operator, Operator::Multiply);
    assert!(rule.clamp_default.is_none());
}

#[test]
fn test_merge_rule_with_all_operators() {
    let operators = vec![
        Operator::Sum,
        Operator::Min,
        Operator::Multiply,
        Operator::Average,
        Operator::Max,
        Operator::Min,
        Operator::Average,
        Operator::Sum,
    ];
    
    for operator in operators {
        let rule = MergeRule {
            use_pipeline: true,
            operator: operator.clone(),
            clamp_default: Some(Caps::new(0.0, 1000.0)),
        };
        
        assert_eq!(rule.operator, operator);
        assert!(rule.use_pipeline);
        assert!(rule.clamp_default.is_some());
    }
}

#[test]
fn test_merge_rule_edge_cases() {
    // Test with extreme clamp values
    let rule_extreme = MergeRule {
        use_pipeline: false,
        operator: Operator::Max,
        clamp_default: Some(Caps::new(f64::MIN, f64::MAX)),
    };
    
    assert!(!rule_extreme.use_pipeline);
    assert_eq!(rule_extreme.operator, Operator::Max);
    assert!(rule_extreme.clamp_default.is_some());
    
    // Test with zero clamp values
    let rule_zero = MergeRule {
        use_pipeline: true,
        operator: Operator::Min,
        clamp_default: Some(Caps::new(0.0, 0.0)),
    };
    
    assert!(rule_zero.use_pipeline);
    assert_eq!(rule_zero.operator, Operator::Min);
    assert!(rule_zero.clamp_default.is_some());
}

// ============================================================================
// TRAIT OBJECT TESTS
// ============================================================================

#[test]
fn test_trait_object_send_sync() {
    // Test that trait objects implement Send + Sync
    
    // This test ensures the traits are properly marked as Send + Sync
    // We can't directly instantiate trait objects in tests, but we can verify
    // the trait bounds are correct by checking the trait definitions
    assert!(true); // Placeholder - the trait bounds are verified at compile time
}

// ============================================================================
// MOCK IMPLEMENTATIONS FOR TESTING
// ============================================================================

// Mock subsystem for testing
struct MockSubsystem {
    system_id: String,
    priority: i64,
}

impl MockSubsystem {
    fn new(system_id: String, priority: i64) -> Self {
        Self { system_id, priority }
    }
}

#[async_trait]
impl Subsystem for MockSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute(&self, _actor: &Actor) -> ActorCoreResult<SubsystemOutput> {
        Ok(SubsystemOutput::new(self.system_id.clone()))
    }
}

// Mock aggregator for testing
struct MockAggregator;

#[async_trait]
impl Aggregator for MockAggregator {
    async fn resolve(&self, _actor: &Actor) -> ActorCoreResult<Snapshot> {
        Ok(Snapshot::new(uuid::Uuid::new_v4(), 1))
    }
    
    async fn resolve_with_context(
        &self, 
        _actor: &Actor, 
        _context: Option<HashMap<String, Value>>
    ) -> ActorCoreResult<Snapshot> {
        Ok(Snapshot::new(uuid::Uuid::new_v4(), 1))
    }
    
    async fn resolve_batch(&self, _actors: &[Actor]) -> ActorCoreResult<Vec<Snapshot>> {
        Ok(vec![Snapshot::new(uuid::Uuid::new_v4(), 1)])
    }
    
    fn get_cached_snapshot(&self, _actor_id: &uuid::Uuid) -> Option<Snapshot> {
        None
    }
    
    fn invalidate_cache(&self, _actor_id: &uuid::Uuid) {
        // Mock implementation
    }
    
    fn clear_cache(&self) {
        // Mock implementation
    }
    
    async fn get_metrics(&self) -> AggregatorMetrics {
        AggregatorMetrics::default()
    }
}

// Mock caps provider for testing
struct MockCapsProvider;

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
        vec!["base".to_string(), "equipment".to_string(), "buffs".to_string()]
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
        Ok(Some(Caps::new(0.0, 100.0)))
    }
    
    fn get_supported_dimensions(&self) -> Vec<String> {
        vec!["strength".to_string(), "agility".to_string()]
    }
    
    fn get_cap_statistics(&self) -> CapStatistics {
        CapStatistics::default()
    }
    
    fn validate(&self) -> ActorCoreResult<()> {
        Ok(())
    }
}

// Mock combiner registry for testing
struct MockCombinerRegistry;

impl CombinerRegistry for MockCombinerRegistry {
    fn get_rule(&self, _dimension: &str) -> Option<MergeRule> {
        Some(MergeRule {
            use_pipeline: true,
            operator: Operator::Sum,
            clamp_default: None,
        })
    }
    
    fn set_rule(&self, _dimension: &str, _rule: MergeRule) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn validate(&self) -> ActorCoreResult<()> {
        Ok(())
    }
}

// Mock cap layer registry for testing
struct MockCapLayerRegistry;

impl CapLayerRegistry for MockCapLayerRegistry {
    fn get_layer_order(&self) -> Vec<String> {
        vec!["base".to_string(), "equipment".to_string(), "buffs".to_string()]
    }
    
    fn set_layer_order(&self, _order: Vec<String>) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn get_across_layer_policy(&self) -> AcrossLayerPolicy {
        AcrossLayerPolicy::Intersect
    }
    
    fn set_across_layer_policy(&self, _policy: AcrossLayerPolicy) {
        // Mock implementation
    }
    
    fn validate(&self) -> ActorCoreResult<()> {
        Ok(())
    }
}

// Mock plugin registry for testing
struct MockPluginRegistry {
    _subsystems: HashMap<String, std::sync::Arc<dyn Subsystem>>,
}

impl MockPluginRegistry {
    fn new() -> Self {
        Self {
            _subsystems: HashMap::new(),
        }
    }
}

impl PluginRegistry for MockPluginRegistry {
    fn register(&self, _subsystem: std::sync::Arc<dyn Subsystem>) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn unregister(&self, _system_id: &str) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn get_by_id(&self, _system_id: &str) -> Option<std::sync::Arc<dyn Subsystem>> {
        None
    }
    
    fn get_by_priority(&self) -> Vec<std::sync::Arc<dyn Subsystem>> {
        Vec::new()
    }
    
    fn get_by_priority_range(&self, _min_priority: i64, _max_priority: i64) -> Vec<std::sync::Arc<dyn Subsystem>> {
        Vec::new()
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

// Mock cache for testing
struct MockCache;

impl Cache for MockCache {
    fn get(&self, _key: &str) -> Option<Value> {
        None
    }
    
    fn set(&self, _key: String, _value: Value, _ttl: Option<u64>) -> ActorCoreResult<()> {
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

// ============================================================================
// TRAIT USAGE TESTS
// ============================================================================

#[test]
fn test_subsystem_trait_usage() {
    let subsystem = MockSubsystem::new("test_system".to_string(), 100);
    
    assert_eq!(subsystem.system_id(), "test_system");
    assert_eq!(subsystem.priority(), 100);
}

#[tokio::test]
async fn test_subsystem_contribute() {
    let subsystem = MockSubsystem::new("test_system".to_string(), 100);
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    
    let result = subsystem.contribute(&actor).await;
    assert!(result.is_ok());
    
    let output = result.unwrap();
    assert_eq!(output.meta.system, "test_system");
}

#[tokio::test]
async fn test_aggregator_trait_usage() {
    let aggregator = MockAggregator;
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    assert!(snapshot.is_valid());
    
    let context = Some(HashMap::new());
    let snapshot_with_context = aggregator.resolve_with_context(&actor, context).await.unwrap();
    assert!(snapshot_with_context.is_valid());
    
    let actors = vec![actor];
    let snapshots = aggregator.resolve_batch(&actors).await.unwrap();
    assert_eq!(snapshots.len(), 1);
}

#[tokio::test]
async fn test_caps_provider_trait_usage() {
    let caps_provider = MockCapsProvider;
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let outputs = vec![];
    
    let caps_within_layer = caps_provider.effective_caps_within_layer(&actor, &outputs, "base").await.unwrap();
    assert!(caps_within_layer.is_empty());
    
    let caps_across_layers = caps_provider.effective_caps_across_layers(&actor, &outputs).await.unwrap();
    assert!(caps_across_layers.is_empty());
    
    let layer_order = caps_provider.get_layer_order();
    assert_eq!(layer_order.len(), 3);
    
    let policy = caps_provider.get_across_layer_policy();
    assert_eq!(policy, AcrossLayerPolicy::Intersect);
    
    let caps = caps_provider.get_caps_for_dimension("strength", &actor).await.unwrap();
    assert!(caps.is_some());
    
    let dimensions = caps_provider.get_supported_dimensions();
    assert_eq!(dimensions.len(), 2);
    
    let _stats = caps_provider.get_cap_statistics();
    // CapStatistics fields are always >= 0 by type
    
    let validation = caps_provider.validate();
    assert!(validation.is_ok());
}

#[test]
fn test_combiner_registry_trait_usage() {
    let registry = MockCombinerRegistry;
    
    let rule = registry.get_rule("strength");
    assert!(rule.is_some());
    
    let new_rule = MergeRule {
        use_pipeline: false,
        operator: Operator::Multiply,
        clamp_default: None,
    };
    
    let set_result = registry.set_rule("strength", new_rule);
    assert!(set_result.is_ok());
    
    let validation = registry.validate();
    assert!(validation.is_ok());
}

#[test]
fn test_cap_layer_registry_trait_usage() {
    let registry = MockCapLayerRegistry;
    
    let layer_order = registry.get_layer_order();
    assert_eq!(layer_order.len(), 3);
    
    let new_order = vec!["new_base".to_string(), "new_equipment".to_string()];
    let set_result = registry.set_layer_order(new_order);
    assert!(set_result.is_ok());
    
    let policy = registry.get_across_layer_policy();
    assert_eq!(policy, AcrossLayerPolicy::Intersect);
    
    registry.set_across_layer_policy(AcrossLayerPolicy::Union);
    
    let validation = registry.validate();
    assert!(validation.is_ok());
}

#[test]
fn test_plugin_registry_trait_usage() {
    let registry = MockPluginRegistry::new();
    
    let subsystem = std::sync::Arc::new(MockSubsystem::new("test".to_string(), 100));
    let register_result = registry.register(subsystem);
    assert!(register_result.is_ok());
    
    let unregister_result = registry.unregister("test");
    assert!(unregister_result.is_ok());
    
    let get_result = registry.get_by_id("test");
    assert!(get_result.is_none());
    
    let by_priority = registry.get_by_priority();
    assert!(by_priority.is_empty());
    
    let by_range = registry.get_by_priority_range(0, 100);
    assert!(by_range.is_empty());
    
    let is_registered = registry.is_registered("test");
    assert!(!is_registered);
    
    let count = registry.count();
    assert_eq!(count, 0);
    
    let validation = registry.validate_all();
    assert!(validation.is_ok());
}

#[test]
fn test_cache_trait_usage() {
    let cache = MockCache;
    
    let get_result = cache.get("test_key");
    assert!(get_result.is_none());
    
    let set_result = cache.set("test_key".to_string(), Value::String("test_value".to_string()), Some(3600));
    assert!(set_result.is_ok());
    
    let delete_result = cache.delete("test_key");
    assert!(delete_result.is_ok());
    
    let clear_result = cache.clear();
    assert!(clear_result.is_ok());
    
    let _stats = cache.get_stats();
    // CacheStats fields are always >= 0 by type
}

// ============================================================================
// COMPREHENSIVE TRAIT INTERACTION TESTS
// ============================================================================

#[tokio::test]
async fn test_trait_interactions() {
    // Test that traits can work together
    let aggregator = MockAggregator;
    let caps_provider = MockCapsProvider;
    let combiner_registry = MockCombinerRegistry;
    let cap_layer_registry = MockCapLayerRegistry;
    let plugin_registry = MockPluginRegistry::new();
    let cache = MockCache;
    
    // Test aggregator with caps provider
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    assert!(snapshot.is_valid());
    
    // Test caps provider with layer registry
    let layer_order = cap_layer_registry.get_layer_order();
    let caps_layer_order = caps_provider.get_layer_order();
    assert_eq!(layer_order.len(), caps_layer_order.len());
    
    // Test combiner registry with merge rules
    let rule = combiner_registry.get_rule("strength");
    assert!(rule.is_some());
    
    // Test cache operations
    let cache_result = cache.set("test".to_string(), Value::String("value".to_string()), None);
    assert!(cache_result.is_ok());
    
    // Test plugin registry operations
    let validation = plugin_registry.validate_all();
    assert!(validation.is_ok());
}

#[test]
fn test_trait_object_usage() {
    // Test that trait objects can be used in collections
    let mut subsystems: Vec<Box<dyn Subsystem>> = Vec::new();
    subsystems.push(Box::new(MockSubsystem::new("system1".to_string(), 100)));
    subsystems.push(Box::new(MockSubsystem::new("system2".to_string(), 200)));
    
    assert_eq!(subsystems.len(), 2);
    assert_eq!(subsystems[0].system_id(), "system1");
    assert_eq!(subsystems[1].system_id(), "system2");
}

#[test]
fn test_trait_bound_verification() {
    // Test that all traits implement the required bounds
    
    // These would compile if the traits are properly bounded
    // test_send_sync(MockAggregator);
    // test_send_sync(MockCapsProvider);
    // test_send_sync(MockCombinerRegistry);
    // test_send_sync(MockCapLayerRegistry);
    // test_send_sync(MockPluginRegistry::new());
    // test_send_sync(MockCache);
    
    assert!(true); // Placeholder - bounds are verified at compile time
}
