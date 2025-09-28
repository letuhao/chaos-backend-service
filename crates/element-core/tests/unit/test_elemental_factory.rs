//! # Elemental Factory Tests
//! 
//! Comprehensive test suite for the ElementalFactory

use element_core::factory::{ElementalFactory, ElementalSystemBuilder};
use element_core::core::{ElementalSystem, ElementalSystemData, ElementMasteryRank};
use element_core::registry::ElementalRegistry;
use element_core::ElementalParams;
use element_core::ElementCoreResult;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn test_factory_creation() {
    let registry = Arc::new(ElementalRegistry::new());
    let factory = ElementalFactory::new(registry);
    
    // Test that factory was created successfully
    let registry_ref = factory.get_registry();
    assert_eq!(registry_ref.element_count(), 0);
}

#[test]
fn test_factory_from_config_dir() {
    // Test with non-existent directory (should return error)
    let result = ElementalFactory::from_config_dir("nonexistent_directory".to_string());
    assert!(result.is_err());
}

#[test]
fn test_create_elemental_system() {
    let registry = Arc::new(ElementalRegistry::new());
    let factory = ElementalFactory::new(registry);
    
    let system = factory.create_elemental_system();
    let data = system.get_data();
    
    // Test that system was created with default data
    assert_eq!(data.element_mastery_levels[0], 0.0);
    assert_eq!(data.element_mastery_experience[0], 0.0);
    assert_eq!(data.element_mastery_ranks[0], ElementMasteryRank::Novice);
}

#[test]
fn test_create_elemental_system_from_data() {
    let registry = Arc::new(ElementalRegistry::new());
    let factory = ElementalFactory::new(registry);
    
    let data = ElementalSystemData::new();
    let system = factory.create_elemental_system_from_data(data.clone());
    
    // Test that system was created from provided data
    let retrieved_data = system.get_data();
    assert_eq!(retrieved_data.element_mastery_levels[0], data.element_mastery_levels[0]);
}

#[test]
fn test_create_elemental_system_with_configs() {
    let registry = Arc::new(ElementalRegistry::new());
    let factory = ElementalFactory::new(registry);
    
    // Test with empty element list
    let element_ids = vec![];
    let result = factory.create_elemental_system_with_configs(&element_ids);
    assert!(result.is_ok());
    
    let system = result.unwrap();
    let data = system.get_data();
    assert_eq!(data.element_mastery_levels[0], 0.0);
}

#[test]
fn test_create_elemental_system_all() {
    let registry = Arc::new(ElementalRegistry::new());
    let factory = ElementalFactory::new(registry);
    
    // Test creating system for all elements (empty registry)
    let result = factory.create_elemental_system_all();
    assert!(result.is_ok());
    
    let system = result.unwrap();
    let data = system.get_data();
    assert_eq!(data.element_mastery_levels[0], 0.0);
}

#[test]
fn test_create_elemental_system_with_params() {
    let registry = Arc::new(ElementalRegistry::new());
    let factory = ElementalFactory::new(registry);
    
    // Create params with non-existent primary element (should return error)
    let mut mastery_levels = HashMap::new();
    mastery_levels.insert("fire".to_string(), 10.0);
    
    let mut experience = HashMap::new();
    experience.insert("fire".to_string(), 100.0);
    
    let mut qi_amounts = HashMap::new();
    qi_amounts.insert("fire".to_string(), 500.0);
    
    let params = ElementalParams {
        primary_element: "fire".to_string(),
        initial_mastery_levels: mastery_levels,
        initial_experience: experience,
        initial_qi_amounts: qi_amounts,
        elemental_preferences: vec!["fire".to_string()],
    };
    
    // Test that creating system with non-existent element returns error
    let result = factory.create_elemental_system_with_params(params);
    assert!(result.is_err());
}

#[test]
fn test_create_builder() {
    let registry = Arc::new(ElementalRegistry::new());
    let factory = ElementalFactory::new(registry);
    
    let builder = factory.create_builder();
    
    // Test that builder was created successfully
    let system = builder.build();
    let data = system.get_data();
    assert_eq!(data.element_mastery_levels[0], 0.0);
}

#[test]
fn test_builder_creation() {
    let registry = Arc::new(ElementalRegistry::new());
    let builder = ElementalSystemBuilder::new(registry);
    
    // Test that builder was created successfully
    let system = builder.build();
    let data = system.get_data();
    assert_eq!(data.element_mastery_levels[0], 0.0);
}

#[test]
fn test_builder_with_element() {
    let registry = Arc::new(ElementalRegistry::new());
    let builder = ElementalSystemBuilder::new(registry);
    
    // Test with non-existent element (should return error)
    let result = builder.with_element("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_builder_with_elements() {
    let registry = Arc::new(ElementalRegistry::new());
    let builder = ElementalSystemBuilder::new(registry);
    
    // Test with empty element list
    let element_ids = vec![];
    let result = builder.with_elements(&element_ids);
    assert!(result.is_ok());
    
    let system = result.unwrap().build();
    let data = system.get_data();
    assert_eq!(data.element_mastery_levels[0], 0.0);
}

#[test]
fn test_builder_with_mastery_level() {
    let registry = Arc::new(ElementalRegistry::new());
    let builder = ElementalSystemBuilder::new(registry);
    
    // Test with non-existent element (should return error)
    let result = builder.with_mastery_level("nonexistent", 10.0);
    assert!(result.is_err());
}

#[test]
fn test_builder_with_qi_amount() {
    let registry = Arc::new(ElementalRegistry::new());
    let builder = ElementalSystemBuilder::new(registry);
    
    // Test with non-existent element (should return error)
    let result = builder.with_qi_amount("nonexistent", 100.0);
    assert!(result.is_err());
}

#[test]
fn test_builder_build() {
    let registry = Arc::new(ElementalRegistry::new());
    let builder = ElementalSystemBuilder::new(registry);
    
    let system = builder.build();
    let data = system.get_data();
    
    // Test that system was built successfully
    assert_eq!(data.element_mastery_levels[0], 0.0);
    assert_eq!(data.element_mastery_experience[0], 0.0);
    assert_eq!(data.element_mastery_ranks[0], ElementMasteryRank::Novice);
}

#[test]
fn test_elemental_params_creation() {
    let mut mastery_levels = HashMap::new();
    mastery_levels.insert("fire".to_string(), 10.0);
    mastery_levels.insert("water".to_string(), 5.0);
    
    let mut experience = HashMap::new();
    experience.insert("fire".to_string(), 100.0);
    experience.insert("water".to_string(), 50.0);
    
    let mut qi_amounts = HashMap::new();
    qi_amounts.insert("fire".to_string(), 500.0);
    qi_amounts.insert("water".to_string(), 300.0);
    
    let params = ElementalParams {
        primary_element: "fire".to_string(),
        initial_mastery_levels: mastery_levels,
        initial_experience: experience,
        initial_qi_amounts: qi_amounts,
        elemental_preferences: vec!["fire".to_string(), "water".to_string()],
    };
    
    assert_eq!(params.primary_element, "fire");
    assert_eq!(params.initial_mastery_levels.len(), 2);
    assert_eq!(params.initial_experience.len(), 2);
    assert_eq!(params.initial_qi_amounts.len(), 2);
    assert_eq!(params.elemental_preferences.len(), 2);
    
    // Test specific values
    assert_eq!(params.initial_mastery_levels.get("fire"), Some(&10.0));
    assert_eq!(params.initial_mastery_levels.get("water"), Some(&5.0));
    assert_eq!(params.initial_experience.get("fire"), Some(&100.0));
    assert_eq!(params.initial_experience.get("water"), Some(&50.0));
    assert_eq!(params.initial_qi_amounts.get("fire"), Some(&500.0));
    assert_eq!(params.initial_qi_amounts.get("water"), Some(&300.0));
    assert_eq!(params.elemental_preferences[0], "fire");
    assert_eq!(params.elemental_preferences[1], "water");
}

#[test]
fn test_elemental_params_empty() {
    let params = ElementalParams {
        primary_element: "fire".to_string(),
        initial_mastery_levels: HashMap::new(),
        initial_experience: HashMap::new(),
        initial_qi_amounts: HashMap::new(),
        elemental_preferences: vec![],
    };
    
    assert_eq!(params.primary_element, "fire");
    assert_eq!(params.initial_mastery_levels.len(), 0);
    assert_eq!(params.initial_experience.len(), 0);
    assert_eq!(params.initial_qi_amounts.len(), 0);
    assert_eq!(params.elemental_preferences.len(), 0);
}

#[test]
fn test_factory_registry_access() {
    let registry = Arc::new(ElementalRegistry::new());
    let factory = ElementalFactory::new(registry.clone());
    
    let retrieved_registry = factory.get_registry();
    
    // Test that we can access the registry
    assert_eq!(retrieved_registry.element_count(), 0);
    
    // Test that it's the same registry instance
    assert!(Arc::ptr_eq(&registry, &retrieved_registry));
}

#[test]
fn test_builder_chain_operations() {
    let registry = Arc::new(ElementalRegistry::new());
    let builder = ElementalSystemBuilder::new(registry);
    
    // Test chaining operations (even though they will fail due to non-existent elements)
    let result = builder
        .with_element("fire")
        .and_then(|b| b.with_mastery_level("fire", 10.0))
        .and_then(|b| b.with_qi_amount("fire", 100.0));
    
    // Should fail due to non-existent element
    assert!(result.is_err());
}

#[test]
fn test_elemental_system_data_consistency() {
    let registry = Arc::new(ElementalRegistry::new());
    let factory = ElementalFactory::new(registry);
    
    let system1 = factory.create_elemental_system();
    let system2 = factory.create_elemental_system();
    
    let data1 = system1.get_data();
    let data2 = system2.get_data();
    
    // Test that both systems have consistent default data
    assert_eq!(data1.element_mastery_levels[0], data2.element_mastery_levels[0]);
    assert_eq!(data1.element_mastery_experience[0], data2.element_mastery_experience[0]);
    assert_eq!(data1.element_mastery_ranks[0], data2.element_mastery_ranks[0]);
}

#[test]
fn test_factory_error_handling() {
    let registry = Arc::new(ElementalRegistry::new());
    let factory = ElementalFactory::new(registry);
    
    // Test error handling for non-existent elements
    let element_ids = vec!["nonexistent".to_string()];
    let result = factory.create_elemental_system_with_configs(&element_ids);
    assert!(result.is_err());
    
    // Test error handling for non-existent primary element in params
    let params = ElementalParams {
        primary_element: "nonexistent".to_string(),
        initial_mastery_levels: HashMap::new(),
        initial_experience: HashMap::new(),
        initial_qi_amounts: HashMap::new(),
        elemental_preferences: vec![],
    };
    
    let result = factory.create_elemental_system_with_params(params);
    assert!(result.is_err());
}
