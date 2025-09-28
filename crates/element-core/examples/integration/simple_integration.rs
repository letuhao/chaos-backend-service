//! # Simple Integration Example
//! 
//! This example demonstrates a simple integration with Element-Core using
//! the current API. It shows how to create elemental systems and use
//! the basic functionality.

use element_core::{
    ElementContributor, ElementContribution, ContributorMetadata, 
    UnifiedElementRegistry, ElementalSystemData,
    ElementCoreResult, ElementCoreError, ElementEvent
};
use element_core::contributor::ElementContributorHelper;
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use actor_core::Actor;

/// Simple contributor that provides basic elemental data
pub struct SimpleContributor {
    system_id: String,
    priority: i64,
    version: String,
}

impl SimpleContributor {
    pub fn new(system_id: String, priority: i64) -> Self {
        Self {
            system_id,
            priority,
            version: "1.0.0".to_string(),
        }
    }
}

#[async_trait]
impl ElementContributor for SimpleContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute_element_stats(
        &self,
        _actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<ElementContribution> {
        let mut stat_contributions = HashMap::new();
        
        // Add some basic stats based on element type
        match element_type {
            "fire" => {
                stat_contributions.insert("power".to_string(), 100.0);
                stat_contributions.insert("defense".to_string(), 50.0);
            },
            "water" => {
                stat_contributions.insert("power".to_string(), 80.0);
                stat_contributions.insert("defense".to_string(), 120.0);
            },
            "earth" => {
                stat_contributions.insert("power".to_string(), 90.0);
                stat_contributions.insert("defense".to_string(), 150.0);
            },
            _ => {
                stat_contributions.insert("power".to_string(), 50.0);
                stat_contributions.insert("defense".to_string(), 50.0);
            }
        }
        
        Ok(self.create_contribution(element_type, stat_contributions))
    }
    
    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()> {
        match event {
            ElementEvent::MasteryLevelChanged { element_type, new_level, actor_id, .. } => {
                println!("SimpleContributor: Actor {} gained {} mastery level in {}", 
                    actor_id, new_level, element_type);
            },
            ElementEvent::ElementInteraction { attacker_element, defender_element, .. } => {
                println!("SimpleContributor: Element interaction {} vs {}", 
                    attacker_element, defender_element);
            },
            ElementEvent::TrainingCompleted { element_type, experience_gained, actor_id } => {
                println!("SimpleContributor: Actor {} completed {} training, gained {} XP", 
                    actor_id, element_type, experience_gained);
            },
            ElementEvent::StatusEffectApplied { element_type, effect_name, intensity, actor_id } => {
                println!("SimpleContributor: Actor {} affected by {} {} (intensity: {})", 
                    actor_id, effect_name, element_type, intensity);
            },
        }
        Ok(())
    }
    
    fn get_metadata(&self) -> ContributorMetadata {
        ContributorMetadata {
            system_id: self.system_id.clone(),
            priority: self.priority,
            version: self.version.clone(),
            description: "Simple contributor for demonstration".to_string(),
        }
    }
}

/// Example function demonstrating simple integration
pub async fn demonstrate_simple_integration() -> ElementCoreResult<()> {
    println!("=== Simple Integration Example ===");
    
    // Create Element-Core registry
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Create simple contributors
    let fire_contributor = SimpleContributor::new("fire-system".to_string(), 1000);
    let water_contributor = SimpleContributor::new("water-system".to_string(), 800);
    let earth_contributor = SimpleContributor::new("earth-system".to_string(), 600);
    
    // Create a mock actor
    let actor = Actor::new("test_actor_001".to_string(), "human".to_string());
    
    // Test contributions for different elements
    let test_elements = ["fire", "water", "earth", "ice", "wood"];
    
    for element in &test_elements {
        println!("\n=== Testing {} Element ===", element);
        
        // Get contribution from fire system
        let fire_contribution = fire_contributor.contribute_element_stats(&actor, element).await?;
        println!("Fire System Contribution:");
        println!("  System: {}", fire_contribution.system_id);
        println!("  Priority: {}", fire_contribution.priority);
        println!("  Stats: {:?}", fire_contribution.stat_contributions);
        
        // Get contribution from water system
        let water_contribution = water_contributor.contribute_element_stats(&actor, element).await?;
        println!("Water System Contribution:");
        println!("  System: {}", water_contribution.system_id);
        println!("  Priority: {}", water_contribution.priority);
        println!("  Stats: {:?}", water_contribution.stat_contributions);
        
        // Get contribution from earth system
        let earth_contribution = earth_contributor.contribute_element_stats(&actor, element).await?;
        println!("Earth System Contribution:");
        println!("  System: {}", earth_contribution.system_id);
        println!("  Priority: {}", earth_contribution.priority);
        println!("  Stats: {:?}", earth_contribution.stat_contributions);
    }
    
    // Test event handling
    println!("\n=== Testing Event Handling ===");
    let events = vec![
        ElementEvent::MasteryLevelChanged {
            element_type: "fire".to_string(),
            old_level: 10.0,
            new_level: 11.0,
            actor_id: "test_actor_001".to_string(),
        },
        ElementEvent::ElementInteraction {
            attacker_element: "fire".to_string(),
            defender_element: "water".to_string(),
            interaction_type: "overcome".to_string(),
            actor_id: "test_actor_001".to_string(),
        },
        ElementEvent::TrainingCompleted {
            element_type: "earth".to_string(),
            experience_gained: 1000.0,
            actor_id: "test_actor_001".to_string(),
        },
    ];
    
    for event in events {
        println!("\nHandling event: {:?}", event);
        fire_contributor.handle_element_event(&event).await?;
        water_contributor.handle_element_event(&event).await?;
        earth_contributor.handle_element_event(&event).await?;
    }
    
    // Test metadata
    println!("\n=== System Metadata ===");
    let contributors = [&fire_contributor, &water_contributor, &earth_contributor];
    
    for contributor in &contributors {
        let metadata = contributor.get_metadata();
        println!("System: {}", metadata.system_id);
        println!("  Priority: {}", metadata.priority);
        println!("  Version: {}", metadata.version);
        println!("  Description: {}", metadata.description);
        println!();
    }
    
    println!("=== Integration Benefits ===");
    println!("✅ Simple integration with Element-Core");
    println!("✅ Event-driven architecture");
    println!("✅ Priority-based contribution system");
    println!("✅ Easy to extend and modify");
    println!("✅ Clean separation of concerns");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_contributor_creation() {
        let contributor = SimpleContributor::new("test-system".to_string(), 500);
        assert_eq!(contributor.system_id, "test-system");
        assert_eq!(contributor.priority, 500);
    }
    
    #[tokio::test]
    async fn test_contribute_element_stats() {
        let contributor = SimpleContributor::new("test-system".to_string(), 500);
        let actor = Actor::new("test".to_string(), "human".to_string());
        
        let contribution = contributor.contribute_element_stats(&actor, "fire").await.unwrap();
        assert_eq!(contribution.system_id, "test-system");
        assert_eq!(contribution.priority, 500);
        assert!(contribution.stat_contributions.contains_key("power"));
    }
    
    #[tokio::test]
    async fn test_handle_element_event() {
        let contributor = SimpleContributor::new("test-system".to_string(), 500);
        let event = ElementEvent::MasteryLevelChanged {
            element_type: "fire".to_string(),
            old_level: 10.0,
            new_level: 11.0,
            actor_id: "test".to_string(),
        };
        
        // Should not panic
        contributor.handle_element_event(&event).await.unwrap();
    }
}
