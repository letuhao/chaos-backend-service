//! # Element Contributor Tests
//! 
//! Comprehensive test suite for the Element Contributor system

use element_core::contributor::{
    ElementContributor, ElementContribution, ElementContributorRegistry, ElementEvent
};
use element_core::ElementCoreResult;
use actor_core::Actor;
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;

// Mock contributor for testing
struct MockContributor {
    system_id: String,
    priority: i64,
    should_fail: bool,
}

impl MockContributor {
    fn new(system_id: String, priority: i64) -> Self {
        Self {
            system_id,
            priority,
            should_fail: false,
        }
    }
    
    fn new_with_failure(system_id: String, priority: i64) -> Self {
        Self {
            system_id,
            priority,
            should_fail: true,
        }
    }
}

#[async_trait]
impl ElementContributor for MockContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }

    fn priority(&self) -> i64 {
        self.priority
    }

    async fn contribute_element_stats(
        &self,
        _actor: &Actor,
        _element_type: &str,
    ) -> ElementCoreResult<ElementContribution> {
        if self.should_fail {
            return Err(element_core::ElementCoreError::Registry("Mock failure".to_string()));
        }
        
        let mut contribution = ElementContribution::new(
            self.system_id.clone(),
            "fire".to_string(),
            "1.0.0".to_string(),
        );
        contribution.add_primary_stat("mastery_level".to_string(), 10.0);
        contribution.add_derived_stat("fire_power".to_string(), 100.0);
        contribution.add_interaction_modifier("fire:wood".to_string(), 1.5);
        contribution.add_tag("test_tag".to_string());
        contribution.add_metadata("test_key".to_string(), serde_json::Value::String("test_value".to_string()));
        Ok(contribution)
    }

    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()> {
        if self.should_fail {
            return Err(element_core::ElementCoreError::Registry("Mock failure".to_string()));
        }
        
        // Mock event handling
        match event {
            ElementEvent::ElementCreated { element_id, .. } => {
                println!("Mock contributor {} handled element created: {}", self.system_id, element_id);
            }
            ElementEvent::ElementModified { element_id, .. } => {
                println!("Mock contributor {} handled element modified: {}", self.system_id, element_id);
            }
            ElementEvent::ElementInteractionOccurred { source_element, target_element, .. } => {
                println!("Mock contributor {} handled interaction: {} vs {}", self.system_id, source_element, target_element);
            }
            ElementEvent::ContributorStatusChange { system_id, is_registered } => {
                println!("Mock contributor {} handled status change: {} -> {}", self.system_id, system_id, is_registered);
            }
            ElementEvent::Warning { message } => {
                println!("Mock contributor {} handled warning: {}", self.system_id, message);
            }
            ElementEvent::Error { message, source } => {
                println!("Mock contributor {} handled error: {} from {}", self.system_id, message, source);
            }
        }
        Ok(())
    }

    fn get_metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("test_mode".to_string(), "true".to_string());
        metadata
    }
}

fn create_test_actor() -> Actor {
    Actor {
        id: "test_actor".to_string(),
        version: 1,
        // Add other required fields as needed
    }
}

#[tokio::test]
async fn test_contributor_registry_creation() {
    let registry = ElementContributorRegistry::new();
    let metrics = registry.get_metrics().await;
    
    assert_eq!(metrics.registered_contributors, 0);
    assert_eq!(metrics.total_contributions_collected, 0);
    assert_eq!(metrics.total_events_handled, 0);
    assert_eq!(metrics.errors_count, 0);
}

#[tokio::test]
async fn test_contributor_registration() {
    let registry = ElementContributorRegistry::new();
    let contributor = Arc::new(MockContributor::new("test_system".to_string(), 1000));
    
    // Register contributor
    registry.register_contributor(contributor).await.unwrap();
    
    let metrics = registry.get_metrics().await;
    assert_eq!(metrics.registered_contributors, 1);
}

#[tokio::test]
async fn test_contributor_unregistration() {
    let registry = ElementContributorRegistry::new();
    let contributor = Arc::new(MockContributor::new("test_system".to_string(), 1000));
    
    // Register contributor
    registry.register_contributor(contributor).await.unwrap();
    
    // Unregister contributor
    registry.unregister_contributor("test_system").await.unwrap();
    
    let metrics = registry.get_metrics().await;
    assert_eq!(metrics.registered_contributors, 0);
}

#[tokio::test]
async fn test_contributor_retrieval() {
    let registry = ElementContributorRegistry::new();
    let contributor = Arc::new(MockContributor::new("test_system".to_string(), 1000));
    
    // Register contributor
    registry.register_contributor(contributor).await.unwrap();
    
    // Retrieve contributor
    let retrieved = registry.get_contributor("test_system");
    assert!(retrieved.is_some());
    
    let retrieved_contributor = retrieved.unwrap();
    assert_eq!(retrieved_contributor.system_id(), "test_system");
    assert_eq!(retrieved_contributor.priority(), 1000);
}

#[tokio::test]
async fn test_contributor_priority_sorting() {
    let registry = ElementContributorRegistry::new();
    
    // Register contributors with different priorities
    let low_priority = Arc::new(MockContributor::new("low_priority".to_string(), 100));
    let high_priority = Arc::new(MockContributor::new("high_priority".to_string(), 1000));
    let medium_priority = Arc::new(MockContributor::new("medium_priority".to_string(), 500));
    
    registry.register_contributor(low_priority).await.unwrap();
    registry.register_contributor(high_priority).await.unwrap();
    registry.register_contributor(medium_priority).await.unwrap();
    
    // Get contributors by priority (should be sorted highest first)
    let contributors = registry.get_contributors_by_priority();
    assert_eq!(contributors.len(), 3);
    assert_eq!(contributors[0].system_id(), "high_priority");
    assert_eq!(contributors[1].system_id(), "medium_priority");
    assert_eq!(contributors[2].system_id(), "low_priority");
}

#[tokio::test]
async fn test_contribution_collection() {
    let registry = ElementContributorRegistry::new();
    let contributor = Arc::new(MockContributor::new("test_system".to_string(), 1000));
    
    // Register contributor
    registry.register_contributor(contributor).await.unwrap();
    
    // Collect contributions
    let actor = create_test_actor();
    let contributions = registry.collect_contributions(&actor, "fire").await.unwrap();
    
    assert_eq!(contributions.len(), 1);
    let contribution = &contributions[0];
    assert_eq!(contribution.system_id, "test_system");
    assert_eq!(contribution.element_id, "fire");
    assert_eq!(contribution.primary_stats.get("mastery_level"), Some(&10.0));
    assert_eq!(contribution.derived_stats.get("fire_power"), Some(&100.0));
    assert_eq!(contribution.interaction_modifiers.get("fire:wood"), Some(&1.5));
    assert!(contribution.tags.contains(&"test_tag".to_string()));
    assert!(contribution.metadata.contains_key("test_key"));
}

#[tokio::test]
async fn test_contribution_collection_with_failure() {
    let registry = ElementContributorRegistry::new();
    let contributor = Arc::new(MockContributor::new_with_failure("test_system".to_string(), 1000));
    
    // Register contributor
    registry.register_contributor(contributor).await.unwrap();
    
    // Collect contributions (should fail)
    let actor = create_test_actor();
    let result = registry.collect_contributions(&actor, "fire").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_event_broadcasting() {
    let registry = ElementContributorRegistry::new();
    let contributor = Arc::new(MockContributor::new("test_system".to_string(), 1000));
    
    // Register contributor
    registry.register_contributor(contributor).await.unwrap();
    
    // Broadcast event
    let event = ElementEvent::ElementCreated {
        element_id: "fire".to_string(),
        config: "test_config".to_string(),
    };
    
    registry.broadcast_event(&event).await.unwrap();
    
    let metrics = registry.get_metrics().await;
    assert_eq!(metrics.total_events_handled, 1);
}

#[tokio::test]
async fn test_event_broadcasting_with_failure() {
    let registry = ElementContributorRegistry::new();
    let contributor = Arc::new(MockContributor::new_with_failure("test_system".to_string(), 1000));
    
    // Register contributor
    registry.register_contributor(contributor).await.unwrap();
    
    // Broadcast event (should fail)
    let event = ElementEvent::ElementCreated {
        element_id: "fire".to_string(),
        config: "test_config".to_string(),
    };
    
    let result = registry.broadcast_event(&event).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_duplicate_registration() {
    let registry = ElementContributorRegistry::new();
    let contributor1 = Arc::new(MockContributor::new("test_system".to_string(), 1000));
    let contributor2 = Arc::new(MockContributor::new("test_system".to_string(), 2000));
    
    // Register first contributor
    registry.register_contributor(contributor1).await.unwrap();
    
    // Register second contributor with same system_id (should replace first)
    registry.register_contributor(contributor2).await.unwrap();
    
    let metrics = registry.get_metrics().await;
    assert_eq!(metrics.registered_contributors, 1);
    
    // Verify the second contributor is registered
    let retrieved = registry.get_contributor("test_system").unwrap();
    assert_eq!(retrieved.priority(), 2000);
}

#[tokio::test]
async fn test_nonexistent_unregistration() {
    let registry = ElementContributorRegistry::new();
    
    // Try to unregister non-existent contributor
    let result = registry.unregister_contributor("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_contribution_validation() {
    let registry = ElementContributorRegistry::new();
    let contributor = Arc::new(MockContributor::new("test_system".to_string(), 1000));
    
    // Register contributor
    registry.register_contributor(contributor).await.unwrap();
    
    // Collect contributions
    let actor = create_test_actor();
    let contributions = registry.collect_contributions(&actor, "fire").await.unwrap();
    
    // Verify contribution structure
    let contribution = &contributions[0];
    assert_eq!(contribution.system_id, "test_system");
    assert_eq!(contribution.element_id, "fire");
    assert_eq!(contribution.version, "1.0.0");
    assert!(!contribution.created_at.to_string().is_empty());
}

#[tokio::test]
async fn test_contributor_metadata() {
    let contributor = MockContributor::new("test_system".to_string(), 1000);
    let metadata = contributor.get_metadata();
    
    assert_eq!(metadata.get("version"), Some(&"1.0.0".to_string()));
    assert_eq!(metadata.get("test_mode"), Some(&"true".to_string()));
}

#[tokio::test]
async fn test_element_event_types() {
    // Test all event types
    let events = vec![
        ElementEvent::ElementCreated {
            element_id: "fire".to_string(),
            config: "test_config".to_string(),
        },
        ElementEvent::ElementModified {
            element_id: "fire".to_string(),
            changes: HashMap::new(),
        },
        ElementEvent::ElementInteractionOccurred {
            source_element: "fire".to_string(),
            target_element: "wood".to_string(),
            outcome: "overcoming".to_string(),
        },
        ElementEvent::ContributorStatusChange {
            system_id: "test_system".to_string(),
            is_registered: true,
        },
        ElementEvent::Warning {
            message: "Test warning".to_string(),
        },
        ElementEvent::Error {
            message: "Test error".to_string(),
            source: "test_source".to_string(),
        },
    ];
    
    for event in events {
        // Test that events can be created and debug printed
        let debug_str = format!("{:?}", event);
        assert!(!debug_str.is_empty());
    }
}

#[tokio::test]
async fn test_contribution_builder_methods() {
    let mut contribution = ElementContribution::new(
        "test_system".to_string(),
        "fire".to_string(),
        "1.0.0".to_string(),
    );
    
    // Test adding primary stat
    contribution.add_primary_stat("mastery_level".to_string(), 10.0);
    assert_eq!(contribution.primary_stats.get("mastery_level"), Some(&10.0));
    
    // Test adding derived stat
    contribution.add_derived_stat("fire_power".to_string(), 100.0);
    assert_eq!(contribution.derived_stats.get("fire_power"), Some(&100.0));
    
    // Test adding interaction modifier
    contribution.add_interaction_modifier("fire:wood".to_string(), 1.5);
    assert_eq!(contribution.interaction_modifiers.get("fire:wood"), Some(&1.5));
    
    // Test adding tag
    contribution.add_tag("test_tag".to_string());
    assert!(contribution.tags.contains(&"test_tag".to_string()));
    
    // Test adding metadata
    contribution.add_metadata("test_key".to_string(), serde_json::Value::String("test_value".to_string()));
    assert!(contribution.metadata.contains_key("test_key"));
}

#[tokio::test]
async fn test_contribution_serialization() {
    let mut contribution = ElementContribution::new(
        "test_system".to_string(),
        "fire".to_string(),
        "1.0.0".to_string(),
    );
    contribution.add_primary_stat("mastery_level".to_string(), 10.0);
    contribution.add_derived_stat("fire_power".to_string(), 100.0);
    
    // Test JSON serialization
    let json = serde_json::to_string(&contribution).unwrap();
    let deserialized: ElementContribution = serde_json::from_str(&json).unwrap();
    
    assert_eq!(contribution.system_id, deserialized.system_id);
    assert_eq!(contribution.element_id, deserialized.element_id);
    assert_eq!(contribution.version, deserialized.version);
    assert_eq!(contribution.primary_stats, deserialized.primary_stats);
    assert_eq!(contribution.derived_stats, deserialized.derived_stats);
}