//! # Elemental Adapter Tests
//! 
//! Integration tests for the elemental adapter functionality.

use actor_core_hierarchical::{ElementalAdapter, ElementalDataConverter};
use element_core::{ElementalSystemData, ElementalRegistry};
use std::sync::Arc;

fn create_test_registry() -> ElementalRegistry {
    let mut registry = ElementalRegistry::new();
    
    let config = element_core::ElementConfig {
        version: 1,
        element: element_core::ElementDefinition {
            id: "fire".to_string(),
            name: "Fire".to_string(),
            aliases: element_core::ElementAliases {
                vi: Some("hỏa".to_string()),
                zh_pinyin: Some("huo".to_string()),
            },
            category: "five_elements".to_string(),
            description: "Fire element".to_string(),
            base_properties: element_core::BaseProperties {
                base_damage: 100.0,
                base_defense: 80.0,
                base_crit_rate: 0.15,
                base_crit_damage: 1.5,
                base_accuracy: 0.85,
            },
            probability_overrides: std::collections::HashMap::new(),
            derived_stats: vec!["element_mastery".to_string()],
            status_effects: vec![],
            same_element_effects: vec![],
            neutral_effects: vec![],
            environment_mods: std::collections::HashMap::new(),
            references: element_core::ElementReferences {
                probability_config_path: None,
                interaction_config_path: None,
                status_pool_path: None,
                golden_vectors_path: None,
                dynamics_design: None,
            },
        },
    };
    
    let _ = registry.register_element("fire".to_string(), config, 0);
    registry
}

#[test]
fn test_elemental_adapter_creation() {
    let registry = Arc::new(create_test_registry());
    let adapter = ElementalAdapter::new(registry);
    assert!(adapter.get_registry().element_count().unwrap() == 1);
}

#[test]
fn test_create_elemental_system() {
    let registry = Arc::new(create_test_registry());
    let adapter = ElementalAdapter::new(registry);
    let elemental_system = adapter.create_elemental_system();
    assert!(elemental_system.get_data().element_mastery_levels[0] == 0.0);
}

#[test]
fn test_elemental_data_converter_trait() {
    let elemental_data = ElementalSystemData::new();
    let converted = elemental_data.to_elemental_system_data();
    assert!(converted.element_mastery_levels[0] == 0.0);
}

#[test]
fn test_elemental_adapter_data_access() {
    let registry = Arc::new(create_test_registry());
    let adapter = ElementalAdapter::new(registry);
    let mut elemental_system = adapter.create_elemental_system();
    
    // Test data access
    let data = adapter.get_elemental_data(&elemental_system);
    assert!(data.element_mastery_levels[0] == 0.0);
    
    // Test mutable data access
    let mut_data = adapter.get_elemental_data_mut(&mut elemental_system);
    mut_data.element_mastery_levels[0] = 5.0;
    assert!(mut_data.element_mastery_levels[0] == 5.0);
}

#[test]
fn test_elemental_registry_integration() {
    let registry = Arc::new(create_test_registry());
    let adapter = ElementalAdapter::new(registry.clone());
    
    // Test registry access
    assert!(adapter.get_registry().element_count().unwrap() == 1);
    assert!(adapter.get_registry().get_element_config("fire").is_ok());
    
    // Test non-existent element returns Ok(None)
    let water_result = adapter.get_registry().get_element_config("water");
    match water_result {
        Ok(Some(_)) => assert!(false, "Expected None for non-existent element"),
        Ok(None) => assert!(true, "Correctly returned None for non-existent element"),
        Err(_) => assert!(false, "Expected Ok(None), not Err"),
    }
}
