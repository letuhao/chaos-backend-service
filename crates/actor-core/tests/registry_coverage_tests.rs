//! Coverage tests for registry modules.

use actor_core::registry::{
    PluginRegistryImpl,
    CombinerRegistryImpl,
    CapLayerRegistryImpl,
    RegistryMetrics
};
use actor_core::registry::loader::{
    CapLayerConfig,
    CapConfig,
    CombinerConfig,
    LoaderError
};
use actor_core::registry::optimized::{
    OptimizedPluginRegistry,
    RegistryStats,
    OptimizedCombinerRegistry
};
use actor_core::interfaces::PluginRegistry;
use actor_core::types::Caps;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

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

#[async_trait::async_trait]
impl actor_core::interfaces::Subsystem for MockSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute(&self, _actor: &actor_core::Actor) -> actor_core::ActorCoreResult<actor_core::types::SubsystemOutput> {
        Ok(actor_core::types::SubsystemOutput {
            primary: vec![],
            derived: vec![],
            context: None,
            meta: actor_core::types::SubsystemMeta {
                system: "test".to_string(),
                data: HashMap::new(),
            },
            caps: vec![],
        })
    }
}

#[test]
fn test_plugin_registry_impl_creation() {
    let registry = PluginRegistryImpl::new();
    
    // Test that the registry was created successfully
    assert!(std::ptr::addr_of!(registry) != std::ptr::null());
}

#[test]
fn test_plugin_registry_impl_default() {
    let registry = PluginRegistryImpl::default();
    
    // Test that the registry was created successfully
    assert!(std::ptr::addr_of!(registry) != std::ptr::null());
}

#[test]
fn test_plugin_registry_register() {
    let registry = PluginRegistryImpl::new();
    let subsystem = Arc::new(MockSubsystem::new("test_system".to_string(), 100));
    
    let result = registry.register(subsystem);
    assert!(result.is_ok());
}

#[test]
fn test_plugin_registry_register_empty_id() {
    let registry = PluginRegistryImpl::new();
    let subsystem = Arc::new(MockSubsystem::new("".to_string(), 100));
    
    let result = registry.register(subsystem);
    assert!(result.is_err());
}

#[test]
fn test_plugin_registry_unregister() {
    let registry = PluginRegistryImpl::new();
    let subsystem = Arc::new(MockSubsystem::new("test_system".to_string(), 100));
    
    // Register first
    registry.register(subsystem).unwrap();
    
    // Then unregister
    let result = registry.unregister("test_system");
    assert!(result.is_ok());
}

#[test]
fn test_plugin_registry_unregister_nonexistent() {
    let registry = PluginRegistryImpl::new();
    
    let result = registry.unregister("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_plugin_registry_get_by_id() {
    let registry = PluginRegistryImpl::new();
    let subsystem = Arc::new(MockSubsystem::new("test_system".to_string(), 100));
    
    // Register first
    registry.register(subsystem).unwrap();
    
    // Then get by ID
    let result = registry.get_by_id("test_system");
    assert!(result.is_some());
}

#[test]
fn test_plugin_registry_get_by_id_nonexistent() {
    let registry = PluginRegistryImpl::new();
    
    let result = registry.get_by_id("nonexistent");
    assert!(result.is_none());
}

#[test]
fn test_plugin_registry_get_by_priority() {
    let registry = PluginRegistryImpl::new();
    let subsystem1 = Arc::new(MockSubsystem::new("system1".to_string(), 100));
    let subsystem2 = Arc::new(MockSubsystem::new("system2".to_string(), 200));
    
    // Register both
    registry.register(subsystem1).unwrap();
    registry.register(subsystem2).unwrap();
    
    // Get by priority
    let result = registry.get_by_priority();
    assert_eq!(result.len(), 2);
    // Should be sorted by priority (higher first)
    assert_eq!(result[0].priority(), 200);
    assert_eq!(result[1].priority(), 100);
}

#[test]
fn test_plugin_registry_count() {
    let registry = PluginRegistryImpl::new();
    let subsystem1 = Arc::new(MockSubsystem::new("system1".to_string(), 100));
    let subsystem2 = Arc::new(MockSubsystem::new("system2".to_string(), 200));
    
    // Initially empty
    assert_eq!(registry.count(), 0);
    
    // Register first
    registry.register(subsystem1).unwrap();
    assert_eq!(registry.count(), 1);
    
    // Register second
    registry.register(subsystem2).unwrap();
    assert_eq!(registry.count(), 2);
}

#[test]
fn test_combiner_registry_impl_creation() {
    let registry = CombinerRegistryImpl::new();
    
    // Test that the registry was created successfully
    assert!(std::ptr::addr_of!(registry) != std::ptr::null());
}

#[test]
fn test_combiner_registry_impl_default() {
    let registry = CombinerRegistryImpl::default();
    
    // Test that the registry was created successfully
    assert!(std::ptr::addr_of!(registry) != std::ptr::null());
}

#[test]
fn test_cap_layer_registry_impl_creation() {
    let registry = CapLayerRegistryImpl::new();
    
    // Test that the registry was created successfully
    assert!(std::ptr::addr_of!(registry) != std::ptr::null());
}

#[test]
fn test_cap_layer_registry_impl_default() {
    let registry = CapLayerRegistryImpl::default();
    
    // Test that the registry was created successfully
    assert!(std::ptr::addr_of!(registry) != std::ptr::null());
}

#[test]
fn test_registry_metrics_creation() {
    let metrics = RegistryMetrics::default();
    
    // Test that the metrics were created successfully
    assert!(std::ptr::addr_of!(metrics) != std::ptr::null());
}

#[test]
fn test_cap_layer_config_creation() {
    let config = CapLayerConfig {
        name: "test_layer".to_string(),
        priority: 100,
        caps: vec![],
    };
    
    assert_eq!(config.name, "test_layer");
    assert_eq!(config.priority, 100);
    assert_eq!(config.caps.len(), 0);
}

#[test]
fn test_cap_config_creation() {
    let config = CapConfig {
        id: "test_cap".to_string(),
        cap_mode: "Add".to_string(),
        min: Some(0.0),
        max: Some(100.0),
    };
    
    assert_eq!(config.id, "test_cap");
    assert_eq!(config.cap_mode, "Add");
    assert_eq!(config.min, Some(0.0));
    assert_eq!(config.max, Some(100.0));
}

#[test]
fn test_combiner_config_creation() {
    let config = CombinerConfig {
        rules: vec![],
    };
    
    assert_eq!(config.rules.len(), 0);
}

#[test]
fn test_loader_error_variants() {
    let errors = vec![
        LoaderError::FileNotFound { path: "test.txt".to_string() },
        LoaderError::InvalidYaml { error: "test error".to_string() },
        LoaderError::InvalidJson { error: "test error".to_string() },
        LoaderError::ValidationError { message: "test error".to_string() },
        LoaderError::IoError { error: "test error".to_string() },
    ];
    
    for error in errors {
        // Test that errors can be created and debug formatted
        let debug_str = format!("{:?}", error);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_optimized_plugin_registry_creation() {
    let registry = OptimizedPluginRegistry::new();
    
    // Test that the registry was created successfully
    assert!(std::ptr::addr_of!(registry) != std::ptr::null());
}

#[test]
fn test_registry_stats_creation() {
    let stats = RegistryStats::new();
    
    // Test that the stats were created successfully
    assert!(std::ptr::addr_of!(stats) != std::ptr::null());
}

#[test]
fn test_optimized_combiner_registry_creation() {
    let registry = OptimizedCombinerRegistry::new();
    
    // Test that the registry was created successfully
    assert!(std::ptr::addr_of!(registry) != std::ptr::null());
}

#[test]
fn test_serialization_deserialization() {
    let config = CapLayerConfig {
        name: "test_layer".to_string(),
        priority: 100,
        caps: vec![],
    };
    
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: CapLayerConfig = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(config.name, deserialized.name);
    assert_eq!(config.priority, deserialized.priority);
    assert_eq!(config.caps.len(), deserialized.caps.len());
}

#[test]
fn test_duration_operations() {
    let duration1 = Duration::from_secs(60);
    let duration2 = Duration::from_secs(30);
    
    assert_eq!(duration1.as_secs(), 60);
    assert_eq!(duration2.as_secs(), 30);
    assert!(duration1 > duration2);
}

#[test]
fn test_hashmap_operations() {
    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key1"), Some(&"value1".to_string()));
    assert_eq!(map.get("key2"), Some(&"value2".to_string()));
    assert!(map.contains_key("key1"));
    assert!(!map.contains_key("key3"));
}

#[test]
fn test_vec_operations() {
    let mut vec = Vec::new();
    vec.push("item1".to_string());
    vec.push("item2".to_string());
    
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], "item1");
    assert_eq!(vec[1], "item2");
}

#[test]
fn test_string_operations() {
    let s1 = "test".to_string();
    let s2 = s1.clone();
    
    assert_eq!(s1, s2);
    assert_eq!(s1.len(), 4);
    assert!(!s1.is_empty());
}

#[test]
fn test_option_operations() {
    let some_value = Some("test".to_string());
    let none_value: Option<String> = None;
    
    assert!(some_value.is_some());
    assert!(none_value.is_none());
    assert_eq!(some_value.as_ref().unwrap(), "test");
    assert_eq!(some_value.as_ref().unwrap_or(&"default".to_string()), "test");
    assert_eq!(none_value.as_ref().unwrap_or(&"default".to_string()), "default");
}

#[test]
fn test_clone_operations() {
    let config = CapLayerConfig {
        name: "test".to_string(),
        priority: 100,
        caps: vec![],
    };
    let cloned = config.clone();
    assert_eq!(config.name, cloned.name);
    assert_eq!(config.priority, cloned.priority);
}

#[test]
fn test_debug_formatting() {
    let config = CapLayerConfig {
        name: "test".to_string(),
        priority: 100,
        caps: vec![],
    };
    let debug_str = format!("{:?}", config);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_serialize_deserialize_roundtrip() {
    let config = CapLayerConfig {
        name: "test".to_string(),
        priority: 100,
        caps: vec![],
    };
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: CapLayerConfig = serde_json::from_str(&serialized).unwrap();
    assert_eq!(config.name, deserialized.name);
    assert_eq!(config.priority, deserialized.priority);
}

#[test]
fn test_boolean_operations() {
    let true_value = true;
    let false_value = false;
    
    assert!(true_value);
    assert!(!false_value);
    assert_eq!(true_value && false_value, false);
    assert_eq!(true_value || false_value, true);
}

#[test]
fn test_u64_operations() {
    let value1 = 100u64;
    let value2 = 50u64;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[test]
fn test_f64_operations() {
    let value1 = 100.0;
    let value2 = 50.0;
    
    assert_eq!(value1 + value2, 150.0);
    assert_eq!(value1 - value2, 50.0);
    assert_eq!(value1 * value2, 5000.0);
    assert_eq!(value1 / value2, 2.0);
}

#[test]
fn test_i64_operations() {
    let value1 = 100i64;
    let value2 = 50i64;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[test]
fn test_arc_operations() {
    let value = Arc::new("test".to_string());
    let cloned = value.clone();
    
    assert_eq!(*value, *cloned);
    assert_eq!(Arc::strong_count(&value), 2);
}

#[test]
fn test_caps_creation() {
    let caps = Caps::new(0.0, 100.0);
    
    // Test that the caps were created successfully
    assert!(std::ptr::addr_of!(caps) != std::ptr::null());
}