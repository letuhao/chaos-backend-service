//! # Element Contributor Trait
//! 
//! This module defines the ElementContributor trait for external system integration.

use async_trait::async_trait;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::{ElementCoreResult, ElementCoreError};
use crate::contributor::ElementContribution;
use actor_core::Actor;

/// Element event types that contributors can handle
#[derive(Debug, Clone)]
pub enum ElementEvent {
    /// Element mastery level changed
    MasteryLevelChanged {
        element_type: String,
        old_level: f64,
        new_level: f64,
        actor_id: String,
    },
    /// Element interaction occurred
    ElementInteraction {
        attacker_element: String,
        defender_element: String,
        interaction_type: String,
        actor_id: String,
    },
    /// Element training completed
    TrainingCompleted {
        element_type: String,
        experience_gained: f64,
        actor_id: String,
    },
    /// Element status effect applied
    StatusEffectApplied {
        element_type: String,
        effect_name: String,
        intensity: f64,
        actor_id: String,
    },
}

/// External system integration trait
/// 
/// This trait allows external systems (Race-Core, Item-Core, Skill-Core) to contribute
/// elemental data to Element-Core through a standardized interface.
#[async_trait]
pub trait ElementContributor: Send + Sync {
    /// System identifier (e.g., "race_core", "item_core", "skill_core")
    fn system_id(&self) -> &str;
    
    /// Priority (higher = more important, processed first)
    /// 
    /// Typical priorities:
    /// - Race-Core: 1000 (base racial bonuses)
    /// - Item-Core: 800 (equipment bonuses)
    /// - Skill-Core: 600 (skill bonuses)
    /// - Event-Core: 400 (temporary effects)
    fn priority(&self) -> i64;
    
    /// Contribute to element stats for a specific actor and element
    /// 
    /// This method is called by Element-Core to gather contributions from
    /// external systems when calculating final element stats.
    async fn contribute_element_stats(
        &self, 
        actor: &Actor, 
        element_type: &str
    ) -> ElementCoreResult<ElementContribution>;
    
    /// Handle element events
    /// 
    /// This method is called when element-related events occur, allowing
    /// external systems to react to changes in the elemental system.
    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()>;
    
    /// Get system metadata
    /// 
    /// Returns metadata about this contributor system for debugging and monitoring.
    fn get_metadata(&self) -> ContributorMetadata {
        ContributorMetadata {
            system_id: self.system_id().to_string(),
            priority: self.priority(),
            version: "1.0.0".to_string(),
            description: "External system contributor".to_string(),
        }
    }
}

/// Metadata about a contributor system
#[derive(Debug, Clone)]
pub struct ContributorMetadata {
    /// System identifier
    pub system_id: String,
    /// Priority level
    pub priority: i64,
    /// System version
    pub version: String,
    /// System description
    pub description: String,
}

/// Helper trait for common contributor operations
pub trait ElementContributorHelper {
    /// Create a basic contribution with system info
    fn create_contribution(
        &self,
        element_type: &str,
        stat_contributions: HashMap<String, f64>,
    ) -> ElementContribution;
    
    /// Validate contribution data
    fn validate_contribution(&self, contribution: &ElementContribution) -> ElementCoreResult<()>;
}

impl<T: ElementContributor> ElementContributorHelper for T {
    fn create_contribution(
        &self,
        element_type: &str,
        stat_contributions: HashMap<String, f64>,
    ) -> ElementContribution {
        ElementContribution {
            system_id: self.system_id().to_string(),
            element_type: element_type.to_string(),
            stat_contributions,
            priority: self.priority(),
            timestamp: Utc::now(),
        }
    }
    
    fn validate_contribution(&self, contribution: &ElementContribution) -> ElementCoreResult<()> {
        // Validate system ID matches
        if contribution.system_id != self.system_id() {
            return Err(ElementCoreError::Validation { 
                message: format!("System ID mismatch: expected {}, got {}", 
                    self.system_id(), contribution.system_id)
            });
        }
        
        // Validate priority matches
        if contribution.priority != self.priority() {
            return Err(ElementCoreError::Validation { 
                message: format!("Priority mismatch: expected {}, got {}", 
                    self.priority(), contribution.priority)
            });
        }
        
        // Validate element type is not empty
        if contribution.element_type.is_empty() {
            return Err(ElementCoreError::Validation { 
                message: "Element type cannot be empty".to_string()
            });
        }
        
        // Validate timestamp is not in the future
        if contribution.timestamp > Utc::now() {
            return Err(ElementCoreError::Validation { 
                message: "Contribution timestamp cannot be in the future".to_string()
            });
        }
        
        Ok(())
    }
}
