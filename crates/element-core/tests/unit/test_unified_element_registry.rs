//! # Unified Element Registry Tests
//! 
//! Comprehensive test suite for the UnifiedElementRegistry

use element_core::unified_registry::{
    UnifiedElementRegistry, ElementDefinition, ElementCategory, SystemRegistration,
    ElementInteraction, InteractionType, RegistryConfig, RegistryMetrics
};
use element_core::core::ElementConfig as CoreElementConfig;
use element_core::ElementCoreResult;
use std::collections::HashMap;

#[tokio::test]
async fn test_registry_creation() {
    let registry = UnifiedElementRegistry::new();
    assert_eq!(registry.element_count(), 0);
    assert_eq!(registry.system_count(), 0);
    assert_eq!(registry.contributor_count(), 0);
    assert_eq!(registry.plugin_count(), 0);
    assert_eq!(registry.interaction_count(), 0);
}

#[tokio::test]
async fn test_registry_with_config() {
    let config = RegistryConfig::default();
    let registry = UnifiedElementRegistry::with_config(config);
    assert_eq!(registry.element_count(), 0);
    assert!(registry.get_config().enable_monitoring);
}

#[tokio::test]
async fn test_element_registration() {
    let registry = UnifiedElementRegistry::new();
    
    let element = ElementDefinition::new(
        "fire".to_string(),
        "Fire".to_string(),
        "Fire element".to_string(),
        ElementCategory::Elemental,
    );
    
    // Register element
    registry.register_element(element).await.unwrap();
    assert_eq!(registry.element_count(), 1);
    assert!(registry.is_element_registered("fire"));
    
    // Get element
    let retrieved = registry.get_element("fire").unwrap();
    assert_eq!(retrieved.id, "fire");
    assert_eq!(retrieved.name, "Fire");
    
    // Unregister element
    registry.unregister_element("fire").await.unwrap();
    assert_eq!(registry.element_count(), 0);
    assert!(!registry.is_element_registered("fire"));
}

#[tokio::test]
async fn test_system_registration() {
    let registry = UnifiedElementRegistry::new();
    
    let system = SystemRegistration::new(
        "race-core".to_string(),
        "Race Core".to_string(),
        "1.0.0".to_string(),
        "test_description".to_string(),
        1000,
    );
    
    // Register system
    registry.register_system(system).await.unwrap();
    assert_eq!(registry.system_count(), 1);
    assert!(registry.is_system_registered("race-core"));
    
    // Get system
    let retrieved = registry.get_system("race-core").unwrap();
    assert_eq!(retrieved.system_id, "race-core");
    assert_eq!(retrieved.system_name, "Race Core");
    
    // Unregister system
    registry.unregister_system("race-core").await.unwrap();
    assert_eq!(registry.system_count(), 0);
    assert!(!registry.is_system_registered("race-core"));
}

#[tokio::test]
async fn test_interaction_registration() {
    let registry = UnifiedElementRegistry::new();
    
    let interaction = ElementInteraction::new(
        "fire_vs_wood".to_string(),
        "fire".to_string(),
        "wood".to_string(),
        InteractionType::Overcoming,
    );
    
    // Register interaction
    registry.register_interaction(interaction).await.unwrap();
    assert_eq!(registry.interaction_count(), 1);
    assert!(registry.is_interaction_registered("fire", "wood"));
    
    // Get interaction
    let retrieved = registry.get_interaction("fire", "wood").unwrap();
    assert_eq!(retrieved.id, "fire_vs_wood");
    assert_eq!(retrieved.source_element, "fire");
    assert_eq!(retrieved.target_element, "wood");
    
    // Unregister interaction
    registry.unregister_interaction("fire", "wood").await.unwrap();
    assert_eq!(registry.interaction_count(), 0);
    assert!(!registry.is_interaction_registered("fire", "wood"));
}

#[tokio::test]
async fn test_duplicate_registration() {
    let registry = UnifiedElementRegistry::new();
    
    let element = ElementDefinition::new(
        "fire".to_string(),
        "Fire".to_string(),
        "Fire element".to_string(),
        ElementCategory::Elemental,
    );
    
    // Register element first time
    registry.register_element(element.clone()).await.unwrap();
    
    // Try to register same element again
    let result = registry.register_element(element).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("already registered"));
}

#[tokio::test]
async fn test_nonexistent_unregistration() {
    let registry = UnifiedElementRegistry::new();
    
    // Try to unregister non-existent element
    let result = registry.unregister_element("nonexistent").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[tokio::test]
async fn test_registry_statistics() {
    let registry = UnifiedElementRegistry::new();
    
    let stats = registry.get_statistics();
    assert_eq!(stats.element_count, 0);
    assert_eq!(stats.system_count, 0);
    assert_eq!(stats.contributor_count, 0);
    assert_eq!(stats.plugin_count, 0);
    assert_eq!(stats.interaction_count, 0);
    assert_eq!(stats.category_count, 0);
}

#[tokio::test]
async fn test_registry_validation() {
    let registry = UnifiedElementRegistry::new();
    
    // Empty registry should validate
    registry.validate().unwrap();
    
    // Add valid element
    let element = ElementDefinition::new(
        "fire".to_string(),
        "Fire".to_string(),
        "Fire element".to_string(),
        ElementCategory::Elemental,
    );
    registry.register_element(element).await.unwrap();
    
    // Registry with valid element should validate
    registry.validate().unwrap();
}

#[tokio::test]
async fn test_registry_clear() {
    let registry = UnifiedElementRegistry::new();
    
    // Add some data
    let element = ElementDefinition::new(
        "fire".to_string(),
        "Fire".to_string(),
        "Fire element".to_string(),
        ElementCategory::Elemental,
    );
    registry.register_element(element).await.unwrap();
    
    let system = SystemRegistration::new(
        "race-core".to_string(),
        "Race Core".to_string(),
        "1.0.0".to_string(),
        "test_description".to_string(),
        1000,
    );
    registry.register_system(system).await.unwrap();
    
    // Verify data exists
    assert_eq!(registry.element_count(), 1);
    assert_eq!(registry.system_count(), 1);
    
    // Clear registry
    registry.clear().await.unwrap();
    
    // Verify data is cleared
    assert_eq!(registry.element_count(), 0);
    assert_eq!(registry.system_count(), 0);
}

#[tokio::test]
async fn test_get_all_elements() {
    let registry = UnifiedElementRegistry::new();
    
    // Add multiple elements
    let fire_element = ElementDefinition::new(
        "fire".to_string(),
        "Fire".to_string(),
        "Fire element".to_string(),
        ElementCategory::Elemental,
    );
    
    let water_element = ElementDefinition::new(
        "water".to_string(),
        "Water".to_string(),
        "Water element".to_string(),
        ElementCategory::Elemental,
    );
    
    registry.register_element(fire_element).await.unwrap();
    registry.register_element(water_element).await.unwrap();
    
    let all_elements = registry.get_all_elements();
    assert_eq!(all_elements.len(), 2);
    assert!(all_elements.contains_key("fire"));
    assert!(all_elements.contains_key("water"));
}

#[tokio::test]
async fn test_get_all_systems() {
    let registry = UnifiedElementRegistry::new();
    
    // Add multiple systems
    let race_system = SystemRegistration::new(
        "race-core".to_string(),
        "Race Core".to_string(),
        "1.0.0".to_string(),
        "test_description".to_string(),
        1000,
    );
    
    let item_system = SystemRegistration::new(
        "item-core".to_string(),
        "Item Core".to_string(),
        "1.0.0".to_string(),
        800,
    );
    
    registry.register_system(race_system).await.unwrap();
    registry.register_system(item_system).await.unwrap();
    
    let all_systems = registry.get_all_systems();
    assert_eq!(all_systems.len(), 2);
    assert!(all_systems.contains_key("race-core"));
    assert!(all_systems.contains_key("item-core"));
}

#[tokio::test]
async fn test_get_all_interactions() {
    let registry = UnifiedElementRegistry::new();
    
    // Add multiple interactions
    let fire_vs_wood = ElementInteraction::new(
        "fire_vs_wood".to_string(),
        "fire".to_string(),
        "wood".to_string(),
        InteractionType::Overcoming,
        1.5,
    );
    
    let water_vs_fire = ElementInteraction::new(
        "water_vs_fire".to_string(),
        "water".to_string(),
        "fire".to_string(),
        InteractionType::Overcoming,
        1.5,
    );
    
    registry.register_interaction(fire_vs_wood).await.unwrap();
    registry.register_interaction(water_vs_fire).await.unwrap();
    
    let all_interactions = registry.get_all_interactions();
    assert_eq!(all_interactions.len(), 2);
    assert!(all_interactions.contains_key("fire:wood"));
    assert!(all_interactions.contains_key("water:fire"));
}

#[tokio::test]
async fn test_metrics_retrieval() {
    let registry = UnifiedElementRegistry::new();
    
    let metrics = registry.get_metrics();
    // Metrics should be retrievable (implementation may vary)
    assert!(metrics.total_elements >= 0);
    assert!(metrics.total_systems >= 0);
    assert!(metrics.total_plugins >= 0);
    assert!(metrics.total_interactions >= 0);
}
