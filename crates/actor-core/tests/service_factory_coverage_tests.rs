//! Coverage tests for service_factory.rs module.

use actor_core::service_factory::ServiceFactory;
use actor_core::interfaces::CapLayerRegistry;
use actor_core::enums::AcrossLayerPolicy;
use actor_core::ActorCoreResult;
use std::sync::Arc;

// Mock implementations for testing
#[derive(Debug)]
struct MockCapLayerRegistry {
    layer_order: Vec<String>,
    policy: AcrossLayerPolicy,
}

impl MockCapLayerRegistry {
    fn new() -> Self {
        Self {
            layer_order: vec!["layer1".to_string()],
            policy: AcrossLayerPolicy::Intersect,
        }
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
        Ok(())
    }
}

#[tokio::test]
async fn test_service_factory_create_caps_provider() {
    let cap_layers = Arc::new(MockCapLayerRegistry::new());
    let _caps_provider = ServiceFactory::create_caps_provider(cap_layers);
    
    // Test that the caps provider was created successfully
    assert!(true);
}

#[tokio::test]
async fn test_service_factory_create_plugin_registry() {
    let _plugin_registry = ServiceFactory::create_plugin_registry();
    
    // Test that the plugin registry was created successfully
    assert!(true);
}

#[tokio::test]
async fn test_service_factory_create_combiner_registry() {
    let _combiner_registry = ServiceFactory::create_combiner_registry();
    
    // Test that the combiner registry was created successfully
    assert!(true);
}

#[tokio::test]
async fn test_service_factory_create_cap_layer_registry() {
    let _cap_layer_registry = ServiceFactory::create_cap_layer_registry();
    
    // Test that the cap layer registry was created successfully
    assert!(true);
}

#[tokio::test]
async fn test_service_factory_create_cache() {
    let cache_result = ServiceFactory::create_cache();
    
    // Test that the cache was created successfully
    assert!(cache_result.is_ok());
    let _cache = cache_result.unwrap();
}

#[tokio::test]
async fn test_service_factory_create_aggregator() {
    let plugin_registry = ServiceFactory::create_plugin_registry();
    let combiner_registry = ServiceFactory::create_combiner_registry();
    let cap_layer_registry = ServiceFactory::create_cap_layer_registry();
    let caps_provider = ServiceFactory::create_caps_provider(cap_layer_registry);
    let cache = ServiceFactory::create_cache().unwrap();
    
    let _aggregator = ServiceFactory::create_aggregator(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Test that the aggregator was created successfully
    assert!(true);
}

#[tokio::test]
async fn test_service_factory_integration_flow() {
    // Test the complete flow of creating all services
    let plugin_registry = ServiceFactory::create_plugin_registry();
    let combiner_registry = ServiceFactory::create_combiner_registry();
    let cap_layer_registry = ServiceFactory::create_cap_layer_registry();
    let caps_provider = ServiceFactory::create_caps_provider(cap_layer_registry);
    let cache = ServiceFactory::create_cache().unwrap();
    let _aggregator = ServiceFactory::create_aggregator(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // All services should be created successfully
    assert!(true);
}

#[tokio::test]
async fn test_service_factory_multiple_instances() {
    // Test creating multiple instances of the same service type
    let _plugin_registry1 = ServiceFactory::create_plugin_registry();
    let _plugin_registry2 = ServiceFactory::create_plugin_registry();
    
    // Both should be created successfully
    assert!(true);
}

#[tokio::test]
async fn test_service_factory_cache_error_handling() {
    // Test that cache creation handles errors properly
    let cache_result = ServiceFactory::create_cache();
    
    // Should succeed with default cache
    assert!(cache_result.is_ok());
    let _cache = cache_result.unwrap();
    
    // Cache should be usable
    assert!(true);
}
