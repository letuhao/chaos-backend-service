//! Coverage tests for caps_provider.rs module.

use actor_core::caps_provider::CapsProviderImpl;
use actor_core::interfaces::{CapsProvider, CapLayerRegistry};
use actor_core::enums::AcrossLayerPolicy;
use actor_core::types::{Actor, SubsystemOutput, CapContribution, Caps, SubsystemMeta};
use actor_core::ActorCoreResult;
use std::sync::Arc;

// Mock CapLayerRegistry for testing
#[derive(Debug)]
struct MockCapLayerRegistry {
    layer_order: Vec<String>,
    policy: AcrossLayerPolicy,
}

impl MockCapLayerRegistry {
    fn new(layer_order: Vec<String>, policy: AcrossLayerPolicy) -> Self {
        Self { layer_order, policy }
    }
}

#[async_trait::async_trait]
impl CapLayerRegistry for MockCapLayerRegistry {
    fn get_layer_order(&self) -> Vec<String> {
        self.layer_order.clone()
    }

    fn get_across_layer_policy(&self) -> AcrossLayerPolicy {
        self.policy
    }

    fn set_layer_order(&self, _order: Vec<String>) -> ActorCoreResult<()> {
        Ok(())
    }

    fn set_across_layer_policy(&self, _policy: AcrossLayerPolicy) {
        // Mock implementation
    }

    fn validate(&self) -> ActorCoreResult<()> {
        if self.layer_order.is_empty() {
            Err(actor_core::ActorCoreError::InvalidInput("Empty layer order".to_string()))
        } else {
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_caps_provider_impl_new() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string(), "layer2".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let _provider = CapsProviderImpl::new(registry);
    
    // Test that the provider was created successfully
    assert!(true);
}

#[tokio::test]
async fn test_caps_provider_impl_get_layer_order() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string(), "layer2".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let layer_order = provider.get_layer_order();
    assert_eq!(layer_order, vec!["layer1", "layer2"]);
}

#[tokio::test]
async fn test_caps_provider_impl_get_across_layer_policy() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Union,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let policy = provider.get_across_layer_policy();
    assert!(matches!(policy, AcrossLayerPolicy::Union));
}

#[tokio::test]
async fn test_caps_provider_impl_validate_caps_valid() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let caps = Caps::new(10.0, 100.0);
    let result = provider.validate_caps("test_dimension", &caps);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_caps_provider_impl_validate_caps_invalid_range() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let caps = Caps::new(100.0, 10.0); // min > max
    let result = provider.validate_caps("test_dimension", &caps);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_caps_provider_impl_validate_caps_negative_min() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let caps = Caps::new(-10.0, 100.0);
    let result = provider.validate_caps("test_dimension", &caps);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_caps_provider_impl_get_supported_dimensions() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let dimensions = provider.get_supported_dimensions();
    assert!(!dimensions.is_empty());
}

#[tokio::test]
async fn test_caps_provider_impl_get_cap_statistics() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let stats = provider.get_cap_statistics();
    assert_eq!(stats.total_calculations, 0);
}

#[tokio::test]
async fn test_caps_provider_impl_validate() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let result = provider.validate();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_caps_provider_impl_validate_empty_layers() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec![],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let result = provider.validate();
    assert!(result.is_err());
}

#[tokio::test]
async fn test_caps_provider_impl_get_caps_for_dimension() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let result = provider.get_caps_for_dimension("test_dimension", &actor).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_caps_provider_impl_effective_caps_within_layer_empty() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let outputs = vec![];
    let result = provider.effective_caps_within_layer(&actor, &outputs, "layer1").await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_caps_provider_impl_effective_caps_within_layer_with_caps() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let cap_contrib = CapContribution {
        system: "test_system".to_string(),
        dimension: "health".to_string(),
        mode: actor_core::enums::CapMode::HardMin,
        kind: "min".to_string(),
        value: 50.0,
        priority: Some(1),
        scope: Some("layer1".to_string()),
        realm: Some("test_realm".to_string()),
        tags: None,
    };
    let output = SubsystemOutput {
        primary: vec![],
        derived: vec![],
        caps: vec![cap_contrib],
        context: None,
        meta: SubsystemMeta::new("test_system".to_string()),
    };
    let outputs = vec![output];
    
    let result = provider.effective_caps_within_layer(&actor, &outputs, "layer1").await;
    assert!(result.is_ok());
    let caps = result.unwrap();
    assert_eq!(caps.len(), 1);
    assert!(caps.contains_key("health"));
}

#[tokio::test]
async fn test_caps_provider_impl_effective_caps_within_layer_max_caps() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let cap_contrib = CapContribution {
        system: "test_system".to_string(),
        dimension: "mana".to_string(),
        mode: actor_core::enums::CapMode::HardMax,
        kind: "max".to_string(),
        value: 100.0,
        priority: Some(1),
        scope: Some("layer1".to_string()),
        realm: Some("test_realm".to_string()),
        tags: None,
    };
    let output = SubsystemOutput {
        primary: vec![],
        derived: vec![],
        caps: vec![cap_contrib],
        context: None,
        meta: SubsystemMeta::new("test_system".to_string()),
    };
    let outputs = vec![output];
    
    let result = provider.effective_caps_within_layer(&actor, &outputs, "layer1").await;
    assert!(result.is_ok());
    let caps = result.unwrap();
    assert_eq!(caps.len(), 1);
    assert!(caps.contains_key("mana"));
}

#[tokio::test]
async fn test_caps_provider_impl_effective_caps_within_layer_multiple_caps() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let cap_contribs = vec![
        CapContribution {
            system: "test_system".to_string(),
            dimension: "health".to_string(),
            mode: actor_core::enums::CapMode::HardMin,
            kind: "min".to_string(),
            value: 50.0,
            priority: Some(1),
            scope: Some("layer1".to_string()),
            realm: Some("test_realm".to_string()),
            tags: None,
        },
        CapContribution {
            system: "test_system".to_string(),
            dimension: "health".to_string(),
            mode: actor_core::enums::CapMode::HardMin,
            kind: "min".to_string(),
            value: 30.0,
            priority: Some(1),
            scope: Some("layer1".to_string()),
            realm: Some("test_realm".to_string()),
            tags: None,
        },
        CapContribution {
            system: "test_system".to_string(),
            dimension: "health".to_string(),
            mode: actor_core::enums::CapMode::HardMax,
            kind: "max".to_string(),
            value: 100.0,
            priority: Some(1),
            scope: Some("layer1".to_string()),
            realm: Some("test_realm".to_string()),
            tags: None,
        },
    ];
    let output = SubsystemOutput {
        primary: vec![],
        derived: vec![],
        caps: cap_contribs,
        context: None,
        meta: SubsystemMeta::new("test_system".to_string()),
    };
    let outputs = vec![output];
    
    let result = provider.effective_caps_within_layer(&actor, &outputs, "layer1").await;
    assert!(result.is_ok());
    let caps = result.unwrap();
    assert_eq!(caps.len(), 1);
    let health_caps = caps.get("health").unwrap();
    assert_eq!(health_caps.min, 50.0); // max of min caps
    assert_eq!(health_caps.max, 100.0); // min of max caps
}

#[tokio::test]
async fn test_caps_provider_impl_effective_caps_across_layers_intersect() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string(), "layer2".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let outputs = vec![];
    
    let result = provider.effective_caps_across_layers(&actor, &outputs).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_caps_provider_impl_effective_caps_across_layers_union() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string(), "layer2".to_string()],
        AcrossLayerPolicy::Union,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let outputs = vec![];
    
    let result = provider.effective_caps_across_layers(&actor, &outputs).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_caps_provider_impl_effective_caps_across_layers_prioritized_override() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string(), "layer2".to_string()],
        AcrossLayerPolicy::PrioritizedOverride,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let outputs = vec![];
    
    let result = provider.effective_caps_across_layers(&actor, &outputs).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_caps_provider_impl_effective_caps_within_layer_wrong_scope() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let cap_contrib = CapContribution {
        system: "test_system".to_string(),
        dimension: "health".to_string(),
        mode: actor_core::enums::CapMode::HardMin,
        kind: "min".to_string(),
        value: 50.0,
        priority: Some(1),
        scope: Some("layer2".to_string()), // Wrong layer
        realm: Some("test_realm".to_string()),
        tags: None,
    };
    let output = SubsystemOutput {
        primary: vec![],
        derived: vec![],
        caps: vec![cap_contrib],
        context: None,
        meta: SubsystemMeta::new("test_system".to_string()),
    };
    let outputs = vec![output];
    
    let result = provider.effective_caps_within_layer(&actor, &outputs, "layer1").await;
    assert!(result.is_ok());
    let caps = result.unwrap();
    assert!(caps.is_empty()); // No caps should match the wrong scope
}

#[tokio::test]
async fn test_caps_provider_impl_effective_caps_within_layer_unknown_kind() {
    let registry = Arc::new(MockCapLayerRegistry::new(
        vec!["layer1".to_string()],
        AcrossLayerPolicy::Intersect,
    ));
    let provider = CapsProviderImpl::new(registry);
    
    let actor = Actor::new("test_actor".to_string(), "human".to_string());
    let cap_contrib = CapContribution {
        system: "test_system".to_string(),
        dimension: "health".to_string(),
        mode: actor_core::enums::CapMode::HardMin,
        kind: "unknown".to_string(), // Unknown kind
        value: 50.0,
        priority: Some(1),
        scope: Some("layer1".to_string()),
        realm: Some("test_realm".to_string()),
        tags: None,
    };
    let output = SubsystemOutput {
        primary: vec![],
        derived: vec![],
        caps: vec![cap_contrib],
        context: None,
        meta: SubsystemMeta::new("test_system".to_string()),
    };
    let outputs = vec![output];
    
    let result = provider.effective_caps_within_layer(&actor, &outputs, "layer1").await;
    assert!(result.is_ok());
    let caps = result.unwrap();
    // Unknown kinds are ignored, but the dimension is still processed
    // Since there are no min/max caps, it should create a cap with default values
    assert_eq!(caps.len(), 1);
    assert!(caps.contains_key("health"));
}
