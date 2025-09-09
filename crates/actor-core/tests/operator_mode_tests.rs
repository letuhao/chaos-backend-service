use actor_core::interfaces::{Aggregator, Cache, CapLayerRegistry, CombinerRegistry, MergeRule, PluginRegistry};
use actor_core::interfaces::{CombinerRegistryAsync};
use actor_core::{RegistryFactory, CacheFactory, ServiceFactory};
use actor_core::types::*;
use actor_core::enums::Operator;
use async_trait::async_trait;
use std::sync::Arc;

// Mock combiner registry that returns operator-mode for a specific dimension
struct MockCombinerRegistry {
    rule: MergeRule,
    dim: String,
}

impl CombinerRegistry for MockCombinerRegistry {
    fn get_rule(&self, dimension: &str) -> Option<MergeRule> {
        if dimension == self.dim { Some(self.rule.clone()) } else { None }
    }
    fn set_rule(&self, _dimension: &str, _rule: MergeRule) -> actor_core::ActorCoreResult<()> { Ok(()) }
    fn validate(&self) -> actor_core::ActorCoreResult<()> { Ok(()) }
}

#[async_trait]
impl CombinerRegistryAsync for MockCombinerRegistry {
    async fn load_from_file(&self, _path: &str) -> actor_core::ActorCoreResult<()> { Ok(()) }
    async fn save_to_file(&self, _path: &str) -> actor_core::ActorCoreResult<()> { Ok(()) }
}

// Simple subsystem that contributes fixed values to a dimension
struct FixedSubsystem {
    id: String,
    prio: i64,
    dim: String,
    values: Vec<f64>,
}

#[async_trait]
impl actor_core::interfaces::Subsystem for FixedSubsystem {
    fn system_id(&self) -> &str { &self.id }
    fn priority(&self) -> i64 { self.prio }
    async fn contribute(&self, _actor: &Actor) -> actor_core::ActorCoreResult<SubsystemOutput> {
        let mut out = SubsystemOutput::new(self.id.clone());
        for (i, v) in self.values.iter().enumerate() {
            out.add_primary(Contribution {
                dimension: self.dim.clone(),
                bucket: actor_core::enums::Bucket::Flat,
                value: *v,
                system: format!("s{}", i),
                priority: Some(100 - i as i64),
                tags: None,
            });
        }
        Ok(out)
    }
}

fn make_actor() -> Actor {
    Actor::new("Tester".to_string(), "Human".to_string())
}

#[tokio::test]
async fn operator_max_is_applied() {
    let plugin_registry: Arc<dyn PluginRegistry> = RegistryFactory::create_plugin_registry();
    let caps_registry: Arc<dyn CapLayerRegistry> = RegistryFactory::create_cap_layer_registry();
    let caps_provider = ServiceFactory::create_caps_provider(caps_registry);
    let cache: Arc<dyn Cache> = CacheFactory::create_in_memory_cache(1024, 60);

    // Register a subsystem producing three values for dimension "stat"
    let subsystem = FixedSubsystem { id: "fixed".to_string(), prio: 10, dim: "stat".to_string(), values: vec![5.0, 9.0, 7.0] };
    plugin_registry.register(Box::new(subsystem)).unwrap();

    // Operator-mode: MAX with no clamp default
    let combiner: Arc<dyn CombinerRegistry> = Arc::new(MockCombinerRegistry{
        rule: MergeRule { use_pipeline: false, operator: Operator::Max, clamp_default: None },
        dim: "stat".to_string()
    });

    let aggregator: Arc<dyn Aggregator> = ServiceFactory::create_aggregator(
        plugin_registry.clone(),
        combiner.clone(),
        caps_provider.clone(),
        cache.clone(),
    );

    let actor = make_actor();
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    let val = snapshot.primary.get("stat").copied().unwrap_or_default();
    assert_eq!(val, 9.0);
}

#[tokio::test]
async fn operator_clamp_default_is_applied_when_no_effective_caps() {
    let plugin_registry: Arc<dyn PluginRegistry> = RegistryFactory::create_plugin_registry();
    let caps_registry: Arc<dyn CapLayerRegistry> = RegistryFactory::create_cap_layer_registry();
    let caps_provider = ServiceFactory::create_caps_provider(caps_registry);
    let cache: Arc<dyn Cache> = CacheFactory::create_in_memory_cache(1024, 60);

    // Contribute a value that exceeds clamp default
    let subsystem = FixedSubsystem { id: "fixed".to_string(), prio: 10, dim: "stat".to_string(), values: vec![150.0] };
    plugin_registry.register(Box::new(subsystem)).unwrap();

    let combiner: Arc<dyn CombinerRegistry> = Arc::new(MockCombinerRegistry{
        rule: MergeRule { use_pipeline: false, operator: Operator::Sum, clamp_default: Some(Caps::new(0.0, 100.0)) },
        dim: "stat".to_string()
    });

    let aggregator: Arc<dyn Aggregator> = ServiceFactory::create_aggregator(
        plugin_registry.clone(),
        combiner.clone(),
        caps_provider.clone(),
        cache.clone(),
    );

    let actor = make_actor();
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    let val = snapshot.primary.get("stat").copied().unwrap_or_default();
    assert_eq!(val, 100.0);
}

#[tokio::test]
async fn constants_clamp_fallback_applies_without_rule_or_caps() {
    let plugin_registry: Arc<dyn PluginRegistry> = RegistryFactory::create_plugin_registry();
    let caps_registry: Arc<dyn CapLayerRegistry> = RegistryFactory::create_cap_layer_registry();
    let caps_provider = ServiceFactory::create_caps_provider(caps_registry);
    let cache: Arc<dyn Cache> = CacheFactory::create_in_memory_cache(1024, 60);

    // Contribute a value that exceeds constants clamp for derived ATTACK_POWER (0..100000)
    let subsystem = FixedSubsystem { id: "fixed".to_string(), prio: 10, dim: "attack_power".to_string(), values: vec![250_000.0] };
    plugin_registry.register(Box::new(subsystem)).unwrap();

    // Combiner returns no rule for attack_power (set rule only for unrelated dimension)
    let combiner: Arc<dyn CombinerRegistry> = Arc::new(MockCombinerRegistry{
        rule: MergeRule { use_pipeline: false, operator: Operator::Sum, clamp_default: None },
        dim: "unrelated".to_string()
    });

    let aggregator: Arc<dyn Aggregator> = ServiceFactory::create_aggregator(
        plugin_registry.clone(),
        combiner.clone(),
        caps_provider.clone(),
        cache.clone(),
    );

    let actor = make_actor();
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    let val = snapshot.primary.get("attack_power").copied().unwrap_or_default();
    // Expect clamped to constants max (100000.0)
    assert_eq!(val, 100_000.0);
}


