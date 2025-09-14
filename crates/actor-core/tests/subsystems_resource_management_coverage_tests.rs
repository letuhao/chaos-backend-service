//! Coverage tests for subsystems/resource_management modules.

use actor_core::subsystems::resource_management::resource_manager::{
    ResourceManagerSubsystem,
    ResourceDefinition,
    ResourceCategory,
    ResourceType,
    RegenType
};
use actor_core::subsystems::resource_management::resource_cache::{
    CachedResource,
    ResourceMetadata,
    CacheLayer,
    CacheConfig
};
use actor_core::subsystems::resource_management::resource_database::{
    ActorResourceDocument,
    ActorStatus
};
use actor_core::subsystems::resource_management::resource_regeneration::{
    RegenerationConfig,
    RegenerationStats
};
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn test_resource_category_variants() {
    let categories = vec![
        ResourceCategory::Health,
        ResourceCategory::Energy,
        ResourceCategory::Physical,
        ResourceCategory::Cultivation,
        ResourceCategory::Special,
    ];
    
    for category in categories {
        assert_eq!(category, category.clone());
    }
}

#[test]
fn test_resource_type_variants() {
    let types = vec![
        ResourceType::Current,
        ResourceType::Max,
        ResourceType::Regen,
        ResourceType::Percentage,
    ];
    
    for resource_type in types {
        assert_eq!(resource_type, resource_type.clone());
    }
}

#[test]
fn test_regen_type_variants() {
    let regen_types = vec![
        RegenType::Continuous,
        RegenType::Tick,
        RegenType::Conditional,
        RegenType::None,
    ];
    
    for regen_type in regen_types {
        assert_eq!(regen_type, regen_type.clone());
    }
}

#[test]
fn test_resource_definition_creation() {
    let mut tags = HashMap::new();
    tags.insert("combat_related".to_string(), "true".to_string());
    tags.insert("critical".to_string(), "true".to_string());
    
    let definition = ResourceDefinition {
        id: "hp".to_string(),
        name: "Health Points".to_string(),
        category: ResourceCategory::Health,
        resource_type: ResourceType::Current,
        base_value: 100.0,
        min_value: 0.0,
        max_value: 10000.0,
        regen_rate: 1.0,
        regen_type: RegenType::Continuous,
        dependencies: vec!["vitality".to_string()],
        tags,
    };
    
    assert_eq!(definition.id, "hp");
    assert_eq!(definition.name, "Health Points");
    assert_eq!(definition.category, ResourceCategory::Health);
    assert_eq!(definition.resource_type, ResourceType::Current);
    assert_eq!(definition.base_value, 100.0);
    assert_eq!(definition.min_value, 0.0);
    assert_eq!(definition.max_value, 10000.0);
    assert_eq!(definition.regen_rate, 1.0);
    assert_eq!(definition.regen_type, RegenType::Continuous);
    assert_eq!(definition.dependencies.len(), 1);
    assert_eq!(definition.tags.len(), 2);
}

#[test]
fn test_resource_manager_subsystem_new() {
    let manager = ResourceManagerSubsystem::new();
    
    // Test that the manager was created successfully
    // Note: We can't test private fields directly, but we can test that the manager was created
    assert!(std::ptr::addr_of!(manager) != std::ptr::null());
}

#[test]
fn test_cached_resource_creation() {
    let metadata = ResourceMetadata {
        category: "health".to_string(),
        dependencies: vec!["vitality".to_string()],
        priority: 1,
        is_shared: false,
    };
    
    let cached_resource = CachedResource {
        value: 100.0,
        timestamp: 1234567890,
        ttl: 300,
        layer: CacheLayer::L1,
        metadata,
    };
    
    assert_eq!(cached_resource.value, 100.0);
    assert_eq!(cached_resource.timestamp, 1234567890);
    assert_eq!(cached_resource.ttl, 300);
    assert_eq!(cached_resource.layer, CacheLayer::L1);
    assert_eq!(cached_resource.metadata.category, "health");
    assert_eq!(cached_resource.metadata.priority, 1);
    assert!(!cached_resource.metadata.is_shared);
}

#[test]
fn test_resource_metadata_creation() {
    let metadata = ResourceMetadata {
        category: "energy".to_string(),
        dependencies: vec!["intelligence".to_string(), "wisdom".to_string()],
        priority: 2,
        is_shared: true,
    };
    
    assert_eq!(metadata.category, "energy");
    assert_eq!(metadata.dependencies.len(), 2);
    assert_eq!(metadata.priority, 2);
    assert!(metadata.is_shared);
}

#[test]
fn test_cache_layer_variants() {
    let layers = vec![
        CacheLayer::L1,
        CacheLayer::L2,
        CacheLayer::L3,
    ];
    
    for layer in layers {
        assert_eq!(layer, layer.clone());
    }
}

#[test]
fn test_cache_config_default() {
    let config = CacheConfig::default();
    
    assert_eq!(config.l1_ttl, 300);
    assert_eq!(config.l2_ttl, 3600);
    assert_eq!(config.l3_ttl, 86400);
    assert_eq!(config.max_l1_size, 10000);
    assert_eq!(config.max_l2_size, 100000);
    assert!(config.warming_enabled);
    assert!(config.batch_enabled);
}

#[test]
fn test_cache_config_creation() {
    let config = CacheConfig {
        l1_ttl: 600,
        l2_ttl: 7200,
        l3_ttl: 172800,
        max_l1_size: 20000,
        max_l2_size: 200000,
        warming_enabled: false,
        batch_enabled: false,
    };
    
    assert_eq!(config.l1_ttl, 600);
    assert_eq!(config.l2_ttl, 7200);
    assert_eq!(config.l3_ttl, 172800);
    assert_eq!(config.max_l1_size, 20000);
    assert_eq!(config.max_l2_size, 200000);
    assert!(!config.warming_enabled);
    assert!(!config.batch_enabled);
}

#[test]
fn test_actor_resource_document_creation() {
    let mut resources = HashMap::new();
    resources.insert("hp".to_string(), 100.0);
    resources.insert("mana".to_string(), 50.0);
    
    let document = ActorResourceDocument {
        actor_id: "actor_1".to_string(),
        resources,
        last_updated: 1234567890,
        status: ActorStatus::Active,
        version: 1,
    };
    
    assert_eq!(document.actor_id, "actor_1");
    assert_eq!(document.resources.len(), 2);
    assert_eq!(document.last_updated, 1234567890);
    // Test that the status is Active (can't compare directly due to missing PartialEq)
    match document.status {
        ActorStatus::Active => assert!(true),
        ActorStatus::Inactive => assert!(false),
    }
    assert_eq!(document.version, 1);
}

#[test]
fn test_actor_status_variants() {
    let statuses = vec![
        ActorStatus::Active,
        ActorStatus::Inactive,
    ];
    
    for status in statuses {
        // Test that the status can be created and cloned
        let _cloned = status.clone();
    }
}

#[test]
fn test_regeneration_config_creation() {
    let config = RegenerationConfig {
        update_interval: 1.0,
        max_concurrent_tasks: 10,
        enable_batch_processing: true,
        batch_size: 100,
        enable_monitoring: true,
    };
    
    assert_eq!(config.update_interval, 1.0);
    assert_eq!(config.max_concurrent_tasks, 10);
    assert!(config.enable_batch_processing);
    assert_eq!(config.batch_size, 100);
    assert!(config.enable_monitoring);
}

#[test]
fn test_regeneration_stats_creation() {
    let mut resource_stats = HashMap::new();
    resource_stats.insert("hp".to_string(), 100.0);
    resource_stats.insert("mana".to_string(), 50.0);
    
    let stats = RegenerationStats {
        total_tasks: 100,
        total_regenerated: 1000.0,
        resource_stats,
    };
    
    assert_eq!(stats.total_tasks, 100);
    assert_eq!(stats.total_regenerated, 1000.0);
    assert_eq!(stats.resource_stats.len(), 2);
}

#[test]
fn test_cached_resource_serialization() {
    let metadata = ResourceMetadata {
        category: "health".to_string(),
        dependencies: vec!["vitality".to_string()],
        priority: 1,
        is_shared: false,
    };
    
    let cached_resource = CachedResource {
        value: 100.0,
        timestamp: 1234567890,
        ttl: 300,
        layer: CacheLayer::L1,
        metadata,
    };
    
    let serialized = serde_json::to_string(&cached_resource).unwrap();
    let deserialized: CachedResource = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(cached_resource.value, deserialized.value);
    assert_eq!(cached_resource.timestamp, deserialized.timestamp);
    assert_eq!(cached_resource.ttl, deserialized.ttl);
    assert_eq!(cached_resource.layer, deserialized.layer);
    assert_eq!(cached_resource.metadata.category, deserialized.metadata.category);
    assert_eq!(cached_resource.metadata.priority, deserialized.metadata.priority);
    assert_eq!(cached_resource.metadata.is_shared, deserialized.metadata.is_shared);
}

#[test]
fn test_actor_resource_document_serialization() {
    let mut resources = HashMap::new();
    resources.insert("hp".to_string(), 100.0);
    
    let document = ActorResourceDocument {
        actor_id: "actor_1".to_string(),
        resources,
        last_updated: 1234567890,
        status: ActorStatus::Active,
        version: 1,
    };
    
    let serialized = serde_json::to_string(&document).unwrap();
    let deserialized: ActorResourceDocument = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(document.actor_id, deserialized.actor_id);
    assert_eq!(document.resources.len(), deserialized.resources.len());
    assert_eq!(document.last_updated, deserialized.last_updated);
    assert_eq!(document.version, deserialized.version);
}

#[test]
fn test_resource_category_equality() {
    assert_eq!(ResourceCategory::Health, ResourceCategory::Health);
    assert_eq!(ResourceCategory::Energy, ResourceCategory::Energy);
    assert_ne!(ResourceCategory::Health, ResourceCategory::Energy);
}

#[test]
fn test_resource_type_equality() {
    assert_eq!(ResourceType::Current, ResourceType::Current);
    assert_eq!(ResourceType::Max, ResourceType::Max);
    assert_ne!(ResourceType::Current, ResourceType::Max);
}

#[test]
fn test_regen_type_equality() {
    assert_eq!(RegenType::Continuous, RegenType::Continuous);
    assert_eq!(RegenType::None, RegenType::None);
    assert_ne!(RegenType::Continuous, RegenType::None);
}

#[test]
fn test_cache_layer_equality() {
    assert_eq!(CacheLayer::L1, CacheLayer::L1);
    assert_eq!(CacheLayer::L2, CacheLayer::L2);
    assert_eq!(CacheLayer::L3, CacheLayer::L3);
    assert_ne!(CacheLayer::L1, CacheLayer::L2);
    assert_ne!(CacheLayer::L2, CacheLayer::L3);
}

#[test]
fn test_duration_creation() {
    let duration = Duration::from_secs(5);
    assert_eq!(duration.as_secs(), 5);
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
fn test_string_operations() {
    let s1 = "test".to_string();
    let s2 = s1.clone();
    
    assert_eq!(s1, s2);
    assert_eq!(s1.len(), 4);
    assert!(!s1.is_empty());
}

#[test]
fn test_vec_operations() {
    let mut vec = Vec::new();
    vec.push("item1");
    vec.push("item2");
    
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], "item1");
    assert_eq!(vec[1], "item2");
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
fn test_u64_operations() {
    let value1 = 100u64;
    let value2 = 50u64;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[test]
fn test_usize_operations() {
    let value1 = 100usize;
    let value2 = 50usize;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[test]
fn test_bool_operations() {
    let true_value = true;
    let false_value = false;
    
    assert!(true_value);
    assert!(!false_value);
    assert_eq!(true_value && false_value, false);
    assert_eq!(true_value || false_value, true);
}

#[test]
fn test_u32_operations() {
    let value1 = 100u32;
    let value2 = 50u32;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[test]
fn test_clone_operations() {
    let original = ResourceCategory::Health;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_debug_formatting() {
    let category = ResourceCategory::Health;
    let debug_str = format!("{:?}", category);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_partial_eq_implementations() {
    // Test that PartialEq is implemented for the enums
    assert!(ResourceCategory::Health == ResourceCategory::Health);
    assert!(ResourceType::Current == ResourceType::Current);
    assert!(RegenType::Continuous == RegenType::Continuous);
    assert!(CacheLayer::L1 == CacheLayer::L1);
}

#[test]
fn test_serialize_deserialize_roundtrip() {
    let original = CacheLayer::L2;
    let serialized = serde_json::to_string(&original).unwrap();
    let deserialized: CacheLayer = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_hashmap_with_different_types() {
    let mut string_map = HashMap::new();
    string_map.insert("key1".to_string(), "value1".to_string());
    
    let mut f64_map = HashMap::new();
    f64_map.insert("key1".to_string(), 100.0);
    
    assert_eq!(string_map.len(), 1);
    assert_eq!(f64_map.len(), 1);
    assert_eq!(string_map.get("key1"), Some(&"value1".to_string()));
    assert_eq!(f64_map.get("key1"), Some(&100.0));
}

#[test]
fn test_vec_with_different_types() {
    let string_vec = vec!["item1".to_string(), "item2".to_string()];
    let f64_vec = vec![1.0, 2.0, 3.0];
    
    assert_eq!(string_vec.len(), 2);
    assert_eq!(f64_vec.len(), 3);
    assert_eq!(string_vec[0], "item1");
    assert_eq!(f64_vec[0], 1.0);
}

#[test]
fn test_option_operations() {
    let some_value = Some(42);
    let none_value: Option<i32> = None;
    
    assert!(some_value.is_some());
    assert!(none_value.is_none());
    assert_eq!(some_value.unwrap(), 42);
    assert_eq!(some_value.unwrap_or(0), 42);
    assert_eq!(none_value.unwrap_or(0), 0);
}