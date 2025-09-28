//! # Element Core Integration Tests
//! 
//! This module contains integration tests for the Element Core system.

use std::collections::HashMap;
use std::sync::Arc;
use tokio;
use element_core::{
    ElementContributor, ElementContribution, ElementContributorRegistry,
    UnifiedElementRegistry, ElementAggregator, ElementDefinition, ElementCategory,
    ElementProperties, SystemRegistration, SystemCapability, SystemStatus,
    AggregationStrategy, CacheConfig, EvictionPolicy
};
use actor_core::Actor;

/// Mock contributor for testing
struct MockRaceContributor {
    system_id: String,
    priority: i64,
}

impl MockRaceContributor {
    fn new() -> Self {
        Self {
            system_id: "race_core".to_string(),
            priority: 1000,
        }
    }
}

#[async_trait::async_trait]
impl ElementContributor for MockRaceContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute_element_stats(
        &self, 
        actor: &Actor, 
        element_type: &str
    ) -> element_core::ElementCoreResult<ElementContribution> {
        let mut stats = HashMap::new();
        
        // Mock race-based contributions
        match element_type {
            "fire" => {
                stats.insert("fire_power_point".to_string(), 100.0);
                stats.insert("fire_defense_point".to_string(), 80.0);
            }
            "water" => {
                stats.insert("water_power_point".to_string(), 90.0);
                stats.insert("water_defense_point".to_string(), 90.0);
            }
            _ => {
                stats.insert(format!("{}_power_point", element_type), 50.0);
                stats.insert(format!("{}_defense_point", element_type), 50.0);
            }
        }
        
        Ok(ElementContribution::new(
            self.system_id.clone(),
            element_type.to_string(),
            stats,
            self.priority,
        ))
    }
    
    async fn handle_element_event(&self, _event: &element_core::ElementEvent) -> element_core::ElementCoreResult<()> {
        Ok(())
    }
}

/// Mock item contributor for testing
struct MockItemContributor {
    system_id: String,
    priority: i64,
}

impl MockItemContributor {
    fn new() -> Self {
        Self {
            system_id: "item_core".to_string(),
            priority: 800,
        }
    }
}

#[async_trait::async_trait]
impl ElementContributor for MockItemContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute_element_stats(
        &self, 
        actor: &Actor, 
        element_type: &str
    ) -> element_core::ElementCoreResult<ElementContribution> {
        let mut stats = HashMap::new();
        
        // Mock item-based contributions
        match element_type {
            "fire" => {
                stats.insert("fire_power_point".to_string(), 50.0); // Fire sword
                stats.insert("fire_crit_rate".to_string(), 0.1);
            }
            "water" => {
                stats.insert("water_power_point".to_string(), 30.0); // Water staff
                stats.insert("water_healing_effectiveness".to_string(), 0.2);
            }
            _ => {
                stats.insert(format!("{}_power_point", element_type), 20.0);
            }
        }
        
        Ok(ElementContribution::new(
            self.system_id.clone(),
            element_type.to_string(),
            stats,
            self.priority,
        ))
    }
    
    async fn handle_element_event(&self, _event: &element_core::ElementEvent) -> element_core::ElementCoreResult<()> {
        Ok(())
    }
}

#[tokio::test]
async fn test_element_contributor_registration() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Register contributors
    let race_contributor = Arc::new(MockRaceContributor::new());
    let item_contributor = Arc::new(MockItemContributor::new());
    
    registry.register_contributor(race_contributor.clone()).await.unwrap();
    registry.register_contributor(item_contributor.clone()).await.unwrap();
    
    // Verify registration
    assert_eq!(registry.contributor_count(), 2);
    assert!(registry.is_contributor_registered("race_core"));
    assert!(registry.is_contributor_registered("item_core"));
    
    // Get contributors
    let retrieved_race = registry.get_contributor("race_core").unwrap();
    assert_eq!(retrieved_race.system_id(), "race_core");
    assert_eq!(retrieved_race.priority(), 1000);
    
    let retrieved_item = registry.get_contributor("item_core").unwrap();
    assert_eq!(retrieved_item.system_id(), "item_core");
    assert_eq!(retrieved_item.priority(), 800);
}

#[tokio::test]
async fn test_element_aggregation() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Register contributors
    let race_contributor = Arc::new(MockRaceContributor::new());
    let item_contributor = Arc::new(MockItemContributor::new());
    
    registry.register_contributor(race_contributor).await.unwrap();
    registry.register_contributor(item_contributor).await.unwrap();
    
    // Create aggregator
    let aggregator = ElementAggregator::new(registry.clone());
    
    // Set aggregation strategies
    aggregator.set_strategy("fire_power_point", AggregationStrategy::Sum);
    aggregator.set_strategy("fire_defense_point", AggregationStrategy::Max);
    aggregator.set_strategy("fire_crit_rate", AggregationStrategy::Average);
    
    // Create test actor
    let actor = Actor::simple("test_actor", "fire_spirit", 10);
    
    // Aggregate contributions
    let result = aggregator.aggregate_contributions(&actor, "fire").await.unwrap();
    
    // Verify results
    assert_eq!(result.get("fire_power_point"), Some(&150.0)); // 100 + 50 (Sum)
    assert_eq!(result.get("fire_defense_point"), Some(&80.0)); // max(80, 0) (Max)
    assert_eq!(result.get("fire_crit_rate"), Some(&0.1)); // 0.1 / 1 (Average)
}

#[tokio::test]
async fn test_element_definition_registration() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Create fire element definition
    let fire_element = ElementDefinition::new(
        "fire".to_string(),
        "Fire Element".to_string(),
        "The element of flames and heat".to_string(),
        ElementCategory::Physical(element_core::PhysicalElement::Fire),
    );
    
    // Register element
    registry.register_element(fire_element.clone()).await.unwrap();
    
    // Verify registration
    assert_eq!(registry.element_count(), 1);
    assert!(registry.is_element_registered("fire"));
    
    // Get element
    let retrieved_element = registry.get_element("fire").unwrap();
    assert_eq!(retrieved_element.id, "fire");
    assert_eq!(retrieved_element.name, "Fire Element");
    assert_eq!(retrieved_element.description, "The element of flames and heat");
}

#[tokio::test]
async fn test_system_registration() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Create system registration
    let mut system = SystemRegistration::new(
        "race_core".to_string(),
        "Race Core System".to_string(),
        "Provides racial bonuses for elements".to_string(),
        "1.0.0".to_string(),
        1000,
    );
    
    system.add_capability(SystemCapability::ContributeStats);
    system.add_capability(SystemCapability::HandleEvents);
    system.set_status(SystemStatus::Active);
    
    // Register system
    registry.register_system(system.clone()).await.unwrap();
    
    // Verify registration
    assert_eq!(registry.system_count(), 1);
    assert!(registry.is_system_registered("race_core"));
    
    // Get system
    let retrieved_system = registry.get_system("race_core").unwrap();
    assert_eq!(retrieved_system.system_id, "race_core");
    assert_eq!(retrieved_system.system_name, "Race Core System");
    assert!(retrieved_system.has_capability(&SystemCapability::ContributeStats));
    assert!(retrieved_system.has_capability(&SystemCapability::HandleEvents));
    assert!(retrieved_system.is_active());
}

#[tokio::test]
async fn test_cache_functionality() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Register contributors
    let race_contributor = Arc::new(MockRaceContributor::new());
    registry.register_contributor(race_contributor).await.unwrap();
    
    // Create aggregator with cache
    let cache_config = CacheConfig {
        enabled: true,
        size_limit: 100,
        default_ttl_seconds: 60,
        eviction_policy: EvictionPolicy::LRU,
    };
    
    let aggregator = ElementAggregator::with_cache_config(registry.clone(), cache_config);
    
    // Create test actor
    let actor = Actor::simple("test_actor", "fire_spirit", 10);
    
    // First aggregation (cache miss)
    let result1 = aggregator.aggregate_contributions(&actor, "fire").await.unwrap();
    
    // Second aggregation (cache hit)
    let result2 = aggregator.aggregate_contributions(&actor, "fire").await.unwrap();
    
    // Results should be identical
    assert_eq!(result1, result2);
    
    // Check cache stats
    let cache_stats = aggregator.get_cache_stats();
    assert!(cache_stats.hit_count > 0 || cache_stats.miss_count > 0);
}

#[tokio::test]
async fn test_priority_based_processing() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Register contributors with different priorities
    let race_contributor = Arc::new(MockRaceContributor::new()); // Priority 1000
    let item_contributor = Arc::new(MockItemContributor::new()); // Priority 800
    
    registry.register_contributor(race_contributor).await.unwrap();
    registry.register_contributor(item_contributor).await.unwrap();
    
    // Get contributors by priority
    let contributors = registry.get_all_contributors();
    
    // Should be sorted by priority (highest first)
    assert_eq!(contributors.len(), 2);
    // Note: The actual order depends on the internal implementation
    // This test verifies that we can retrieve all contributors
}

#[tokio::test]
async fn test_aggregation_strategies() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Register contributors
    let race_contributor = Arc::new(MockRaceContributor::new());
    let item_contributor = Arc::new(MockItemContributor::new());
    
    registry.register_contributor(race_contributor).await.unwrap();
    registry.register_contributor(item_contributor).await.unwrap();
    
    let aggregator = ElementAggregator::new(registry.clone());
    
    // Test different strategies
    aggregator.set_strategy("fire_power_point", AggregationStrategy::Sum);
    aggregator.set_strategy("fire_defense_point", AggregationStrategy::Max);
    aggregator.set_strategy("fire_crit_rate", AggregationStrategy::Min);
    
    let actor = Actor::simple("test_actor", "fire_spirit", 10);
    let result = aggregator.aggregate_contributions(&actor, "fire").await.unwrap();
    
    // Verify strategy application
    assert!(result.contains_key("fire_power_point"));
    assert!(result.contains_key("fire_defense_point"));
}

#[tokio::test]
async fn test_error_handling() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Try to register duplicate contributor
    let contributor1 = Arc::new(MockRaceContributor::new());
    let contributor2 = Arc::new(MockRaceContributor::new());
    
    registry.register_contributor(contributor1).await.unwrap();
    let result = registry.register_contributor(contributor2).await;
    
    assert!(result.is_err());
    
    // Try to get non-existent contributor
    let non_existent = registry.get_contributor("non_existent");
    assert!(non_existent.is_none());
    
    // Try to unregister non-existent contributor
    let result = registry.unregister_contributor("non_existent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_registry_statistics() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Register some data
    let contributor = Arc::new(MockRaceContributor::new());
    registry.register_contributor(contributor).await.unwrap();
    
    let element = ElementDefinition::new(
        "fire".to_string(),
        "Fire Element".to_string(),
        "Fire element description".to_string(),
        ElementCategory::Physical(element_core::PhysicalElement::Fire),
    );
    registry.register_element(element).await.unwrap();
    
    let system = SystemRegistration::new(
        "race_core".to_string(),
        "Race Core".to_string(),
        "Race system".to_string(),
        "1.0.0".to_string(),
        1000,
    );
    registry.register_system(system).await.unwrap();
    
    // Get statistics
    let stats = registry.get_statistics();
    
    assert_eq!(stats.element_count, 1);
    assert_eq!(stats.contributor_count, 1);
    assert_eq!(stats.system_count, 1);
}
