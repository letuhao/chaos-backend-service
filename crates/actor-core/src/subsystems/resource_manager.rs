//! Resource Manager Subsystem
//!
//! This subsystem manages all types of game resources (HP, Mana, Stamina, etc.)
//! through the Actor Core v3 architecture. It follows the "metadata-only aggregator"
//! principle, calculating resource values based on Actor metadata without storing state.

use async_trait::async_trait;
use std::collections::HashMap;
use crate::interfaces::Subsystem;
use crate::types::{Actor, SubsystemOutput, Contribution, CapContribution};
use crate::enums::{Bucket, CapMode};
use crate::ActorCoreResult;

/// Resource Manager Subsystem for managing game resources
#[derive(Debug, Clone)]
pub struct ResourceManagerSubsystem {
    /// Unique system identifier
    system_id: String,
    /// Processing priority (higher = more important)
    priority: i64,
    /// Resource definitions
    resource_definitions: HashMap<String, ResourceDefinition>,
}

/// Resource definition for different resource types
#[derive(Debug, Clone)]
pub struct ResourceDefinition {
    /// Resource identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Resource category
    pub category: ResourceCategory,
    /// Resource type
    pub resource_type: ResourceType,
    /// Base value calculation
    pub base_value: f64,
    /// Minimum value
    pub min_value: f64,
    /// Maximum value
    pub max_value: f64,
    /// Regeneration rate per second
    pub regen_rate: f64,
    /// Regeneration type
    pub regen_type: RegenType,
    /// Dependencies on other resources
    pub dependencies: Vec<String>,
    /// Additional tags
    pub tags: HashMap<String, String>,
}

/// Resource categories
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceCategory {
    Health,
    Energy,
    Physical,
    Cultivation,
    Special,
}

/// Resource types
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Current,    // Current value (e.g., hp_current)
    Max,        // Maximum value (e.g., hp_max)
    Regen,      // Regeneration rate (e.g., hp_regen)
    Percentage, // Percentage value (e.g., hp_percentage)
}

/// Regeneration types
#[derive(Debug, Clone, PartialEq)]
pub enum RegenType {
    Continuous,  // Regenerates continuously
    Tick,        // Regenerates per tick
    Conditional, // Regenerates based on conditions
    None,        // No regeneration
}

impl ResourceManagerSubsystem {
    /// Create a new Resource Manager Subsystem
    pub fn new() -> Self {
        let mut resource_definitions = HashMap::new();
        
        // Initialize core resource definitions
        Self::initialize_core_resources(&mut resource_definitions);
        
        Self {
            system_id: "resource_manager".to_string(),
            priority: 100,
            resource_definitions,
        }
    }
    
    /// Initialize core resource definitions
    fn initialize_core_resources(definitions: &mut HashMap<String, ResourceDefinition>) {
        // Health Resources
        definitions.insert("hp".to_string(), ResourceDefinition {
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
            tags: HashMap::from([
                ("combat_related".to_string(), "true".to_string()),
                ("critical".to_string(), "true".to_string()),
            ]),
        });
        
        definitions.insert("mana".to_string(), ResourceDefinition {
            id: "mana".to_string(),
            name: "Mana".to_string(),
            category: ResourceCategory::Energy,
            resource_type: ResourceType::Current,
            base_value: 50.0,
            min_value: 0.0,
            max_value: 5000.0,
            regen_rate: 2.0,
            regen_type: RegenType::Continuous,
            dependencies: vec!["intelligence".to_string()],
            tags: HashMap::from([
                ("magic_related".to_string(), "true".to_string()),
                ("castable".to_string(), "true".to_string()),
            ]),
        });
        
        definitions.insert("stamina".to_string(), ResourceDefinition {
            id: "stamina".to_string(),
            name: "Stamina".to_string(),
            category: ResourceCategory::Physical,
            resource_type: ResourceType::Current,
            base_value: 100.0,
            min_value: 0.0,
            max_value: 10000.0,
            regen_rate: 3.0,
            regen_type: RegenType::Continuous,
            dependencies: vec!["vitality".to_string()],
            tags: HashMap::from([
                ("movement_related".to_string(), "true".to_string()),
                ("combat_related".to_string(), "true".to_string()),
            ]),
        });
        
        definitions.insert("shield".to_string(), ResourceDefinition {
            id: "shield".to_string(),
            name: "Shield".to_string(),
            category: ResourceCategory::Special,
            resource_type: ResourceType::Current,
            base_value: 0.0,
            min_value: 0.0,
            max_value: 1000.0,
            regen_rate: 0.0,
            regen_type: RegenType::None,
            dependencies: vec![],
            tags: HashMap::from([
                ("temporary".to_string(), "true".to_string()),
                ("decay".to_string(), "true".to_string()),
            ]),
        });
    }
    
    /// Calculate base resource values for an actor
    fn calculate_base_resources(&self, actor: &Actor) -> HashMap<String, f64> {
        let mut resources = HashMap::new();
        
        for (resource_id, definition) in &self.resource_definitions {
            let base_value = self.calculate_base_value(definition, actor);
            
            // Set current and max values
            resources.insert(format!("{}_current", resource_id), base_value);
            resources.insert(format!("{}_max", resource_id), base_value);
            
            // Calculate regeneration rate
            if definition.regen_rate > 0.0 {
                resources.insert(format!("{}_regen", resource_id), definition.regen_rate);
            }
            
            // Calculate percentage (initially 100%)
            resources.insert(format!("{}_percentage", resource_id), 100.0);
        }
        
        resources
    }
    
    /// Calculate base value for a specific resource
    fn calculate_base_value(&self, definition: &ResourceDefinition, actor: &Actor) -> f64 {
        let mut base_value = definition.base_value;
        
        // Apply lifespan modifier for health resources
        if definition.category == ResourceCategory::Health {
            base_value += (actor.lifespan as f64) * 10.0;
        }
        
        // Apply age modifier for energy resources
        if definition.category == ResourceCategory::Energy {
            base_value += (actor.age as f64) * 5.0;
        }
        
        // Apply race modifier
        let race_modifier = self.get_race_modifier(&actor.race, &definition.category);
        base_value *= race_modifier;
        
        // Apply cultivation modifier if applicable
        let cultivation_modifier = self.get_cultivation_modifier(actor, &definition.category);
        base_value *= cultivation_modifier;
        
        // Ensure within bounds
        base_value.max(definition.min_value).min(definition.max_value)
    }
    
    /// Get race modifier for resource calculation
    fn get_race_modifier(&self, race: &str, category: &ResourceCategory) -> f64 {
        match (race, category) {
            ("Human", ResourceCategory::Health) => 1.0,
            ("Elf", ResourceCategory::Energy) => 1.2,
            ("Dwarf", ResourceCategory::Physical) => 1.1,
            ("Orc", ResourceCategory::Health) => 1.3,
            _ => 1.0,
        }
    }
    
    /// Get cultivation modifier for resource calculation
    fn get_cultivation_modifier(&self, actor: &Actor, category: &ResourceCategory) -> f64 {
        // Check if actor has cultivation subsystems
        if actor.has_subsystem("jindan_system") || actor.has_subsystem("cultivation_system") {
            match category {
                ResourceCategory::Health => 1.5,
                ResourceCategory::Energy => 2.0,
                ResourceCategory::Cultivation => 3.0,
                _ => 1.1,
            }
        } else {
            1.0
        }
    }
    
    /// Create contributions from resource values
    fn create_contributions(&self, resources: HashMap<String, f64>) -> SubsystemOutput {
        let mut output = SubsystemOutput::new(self.system_id.clone());
        
        for (dimension, value) in resources {
            // Determine bucket type based on dimension
            let bucket = self.get_bucket_for_dimension(&dimension);
            
            // Create primary contribution
            let contribution = Contribution::new(
                dimension.clone(),
                bucket,
                value,
                self.system_id.clone(),
            );
            
            // Add to appropriate category
            if dimension.ends_with("_percentage") {
                output.add_derived(contribution);
            } else {
                output.add_primary(contribution);
            }
        }
        
        // Add cap contributions for resource limits
        self.add_cap_contributions(&mut output);
        
        output
    }
    
    /// Get bucket type for a dimension
    fn get_bucket_for_dimension(&self, dimension: &str) -> Bucket {
        if dimension.ends_with("_percentage") {
            Bucket::Override // Percentages are calculated, not accumulated
        } else {
            Bucket::Flat // Most resources are additive
        }
    }
    
    /// Add cap contributions for resource limits
    fn add_cap_contributions(&self, output: &mut SubsystemOutput) {
        for (resource_id, definition) in &self.resource_definitions {
            // Add min cap
            if definition.min_value > 0.0 {
                let cap_contribution = CapContribution::new(
                    self.system_id.clone(),
                    format!("{}_current", resource_id),
                    CapMode::HardMin,
                    "min".to_string(),
                    definition.min_value,
                );
                output.add_cap(cap_contribution);
            }
            
            // Add max cap
            if definition.max_value < f64::INFINITY {
                let cap_contribution = CapContribution::new(
                    self.system_id.clone(),
                    format!("{}_current", resource_id),
                    CapMode::HardMax,
                    "max".to_string(),
                    definition.max_value,
                );
                output.add_cap(cap_contribution);
            }
        }
    }
}

#[async_trait]
impl Subsystem for ResourceManagerSubsystem {
    /// Get the unique identifier for this subsystem
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    /// Get the priority of this subsystem
    fn priority(&self) -> i64 {
        self.priority
    }
    
    /// Contribute to actor stats
    async fn contribute(&self, actor: &Actor) -> ActorCoreResult<SubsystemOutput> {
        // Calculate base resources
        let resources = self.calculate_base_resources(actor);
        
        // Create contributions
        let output = self.create_contributions(resources);
        
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Actor;
    
    #[test]
    fn test_resource_manager_creation() {
        let rm = ResourceManagerSubsystem::new();
        
        assert_eq!(rm.system_id(), "resource_manager");
        assert_eq!(rm.priority(), 100);
        assert!(!rm.resource_definitions.is_empty());
    }
    
    #[test]
    fn test_resource_definitions() {
        let rm = ResourceManagerSubsystem::new();
        
        // Check that core resources are defined
        assert!(rm.resource_definitions.contains_key("hp"));
        assert!(rm.resource_definitions.contains_key("mana"));
        assert!(rm.resource_definitions.contains_key("stamina"));
        assert!(rm.resource_definitions.contains_key("shield"));
        
        // Check HP resource definition
        let hp_def = rm.resource_definitions.get("hp").unwrap();
        assert_eq!(hp_def.name, "Health Points");
        assert_eq!(hp_def.category, ResourceCategory::Health);
        assert_eq!(hp_def.base_value, 100.0);
    }
    
    #[tokio::test]
    async fn test_contribute_basic() {
        let rm = ResourceManagerSubsystem::new();
        let actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        let result = rm.contribute(&actor).await;
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(!output.primary.is_empty());
        assert!(!output.derived.is_empty());
        assert!(!output.caps.is_empty());
    }
    
    #[test]
    fn test_race_modifiers() {
        let rm = ResourceManagerSubsystem::new();
        
        // Test different race modifiers
        assert_eq!(rm.get_race_modifier("Human", &ResourceCategory::Health), 1.0);
        assert_eq!(rm.get_race_modifier("Elf", &ResourceCategory::Energy), 1.2);
        assert_eq!(rm.get_race_modifier("Dwarf", &ResourceCategory::Physical), 1.1);
        assert_eq!(rm.get_race_modifier("Orc", &ResourceCategory::Health), 1.3);
    }
    
    #[test]
    fn test_cultivation_modifiers() {
        let rm = ResourceManagerSubsystem::new();
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Test without cultivation
        assert_eq!(rm.get_cultivation_modifier(&actor, &ResourceCategory::Health), 1.0);
        
        // Test with cultivation subsystem
        actor.add_subsystem(crate::types::Subsystem::new("jindan_system".to_string(), 100));
        assert_eq!(rm.get_cultivation_modifier(&actor, &ResourceCategory::Health), 1.5);
        assert_eq!(rm.get_cultivation_modifier(&actor, &ResourceCategory::Energy), 2.0);
        assert_eq!(rm.get_cultivation_modifier(&actor, &ResourceCategory::Cultivation), 3.0);
    }
}
