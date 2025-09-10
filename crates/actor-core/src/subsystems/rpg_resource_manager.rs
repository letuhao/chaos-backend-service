//! RPG Resource Manager
//!
//! This module provides a comprehensive RPG resource management system
//! for the Enhanced Hybrid Resource Manager, handling traditional RPG
//! resources like HP, MP, stamina, and experience points.

use async_trait::async_trait;
use std::collections::HashMap;
use crate::types::Actor;
use crate::ActorCoreResult;
use super::system_resource_manager::SystemResourceCalculator;

/// Convert string error to ActorCoreError
fn to_actor_core_error(msg: String) -> crate::ActorCoreError {
    crate::ActorCoreError::SubsystemError(msg)
}

/// RPG Resource Manager for traditional RPG systems
pub struct RpgResourceManager {
    /// System identifier
    system_id: String,
    /// Resource definitions
    resource_definitions: HashMap<String, RpgResourceDefinition>,
}

/// RPG Resource Definition
#[derive(Debug, Clone)]
pub struct RpgResourceDefinition {
    /// Resource name
    pub name: String,
    /// Base value calculation formula
    pub base_formula: String,
    /// Regeneration rate per second
    pub regen_rate: f64,
    /// Maximum value formula
    pub max_formula: String,
    /// Resource category
    pub category: RpgResourceCategory,
    /// Dependencies on other resources
    pub dependencies: Vec<String>,
}

/// RPG Resource Categories
#[derive(Debug, Clone, PartialEq)]
pub enum RpgResourceCategory {
    /// Health and vitality
    Health,
    /// Magic and mana
    Magic,
    /// Physical stamina
    Stamina,
    /// Experience and progression
    Experience,
    /// Special abilities
    Special,
}

impl RpgResourceManager {
    /// Create a new RPG Resource Manager
    pub fn new() -> Self {
        let mut manager = Self {
            system_id: "rpg_system".to_string(),
            resource_definitions: HashMap::new(),
        };
        
        // Initialize default resource definitions
        manager.initialize_default_resources();
        manager
    }
    
    /// Initialize default RPG resources
    fn initialize_default_resources(&mut self) {
        // Health Points (HP)
        self.add_resource_definition(RpgResourceDefinition {
            name: "hp_current".to_string(),
            base_formula: "vitality * 10 + level * 5".to_string(),
            regen_rate: 0.1, // 0.1 HP per second
            max_formula: "vitality * 10 + level * 5 + equipment_bonus".to_string(),
            category: RpgResourceCategory::Health,
            dependencies: vec!["vitality".to_string(), "level".to_string()],
        });
        
        // Magic Points (MP)
        self.add_resource_definition(RpgResourceDefinition {
            name: "mp_current".to_string(),
            base_formula: "intelligence * 8 + level * 3".to_string(),
            regen_rate: 0.2, // 0.2 MP per second
            max_formula: "intelligence * 8 + level * 3 + equipment_bonus".to_string(),
            category: RpgResourceCategory::Magic,
            dependencies: vec!["intelligence".to_string(), "level".to_string()],
        });
        
        // Stamina
        self.add_resource_definition(RpgResourceDefinition {
            name: "stamina_current".to_string(),
            base_formula: "constitution * 6 + level * 2".to_string(),
            regen_rate: 0.5, // 0.5 stamina per second
            max_formula: "constitution * 6 + level * 2 + equipment_bonus".to_string(),
            category: RpgResourceCategory::Stamina,
            dependencies: vec!["constitution".to_string(), "level".to_string()],
        });
        
        // Experience Points
        self.add_resource_definition(RpgResourceDefinition {
            name: "exp_current".to_string(),
            base_formula: "0".to_string(), // Experience starts at 0
            regen_rate: 0.0, // Experience doesn't regenerate
            max_formula: "level * 1000 + (level - 1) * 500".to_string(),
            category: RpgResourceCategory::Experience,
            dependencies: vec!["level".to_string()],
        });
        
        // Special Ability Points
        self.add_resource_definition(RpgResourceDefinition {
            name: "special_points_current".to_string(),
            base_formula: "charisma * 4 + level * 1".to_string(),
            regen_rate: 0.05, // 0.05 points per second
            max_formula: "charisma * 4 + level * 1 + equipment_bonus".to_string(),
            category: RpgResourceCategory::Special,
            dependencies: vec!["charisma".to_string(), "level".to_string()],
        });
    }
    
    /// Add a resource definition
    pub fn add_resource_definition(&mut self, definition: RpgResourceDefinition) {
        self.resource_definitions.insert(definition.name.clone(), definition);
    }
    
    /// Calculate resource value using formula
    async fn calculate_resource_value(&self, actor: &Actor, resource_name: &str) -> ActorCoreResult<f64> {
        let definition = self.resource_definitions.get(resource_name)
            .ok_or_else(|| to_actor_core_error(format!("Resource definition not found: {}", resource_name)))?;
        
        // Parse and evaluate the formula
        let value = self.evaluate_formula(actor, &definition.base_formula).await?;
        Ok(value)
    }
    
    /// Calculate maximum resource value
    async fn calculate_max_resource_value(&self, actor: &Actor, resource_name: &str) -> ActorCoreResult<f64> {
        let definition = self.resource_definitions.get(resource_name)
            .ok_or_else(|| to_actor_core_error(format!("Resource definition not found: {}", resource_name)))?;
        
        // Parse and evaluate the max formula
        let value = self.evaluate_formula(actor, &definition.max_formula).await?;
        Ok(value)
    }
    
    /// Evaluate a mathematical formula
    async fn evaluate_formula(&self, actor: &Actor, formula: &str) -> ActorCoreResult<f64> {
        // Simple formula evaluator - in practice, you'd use a proper expression parser
        let mut result = 0.0;
        #[allow(unused_assignments)]
        let mut current_value = 0.0;
        let mut current_operator = '+';
        
        let tokens: Vec<&str> = formula.split_whitespace().collect();
        
        for token in tokens {
            match token {
                "+" => current_operator = '+',
                "-" => current_operator = '-',
                "*" => current_operator = '*',
                "/" => current_operator = '/',
                _ => {
                    // Try to parse as number or variable
                    if let Ok(num) = token.parse::<f64>() {
                        current_value = num;
                    } else {
                        // It's a variable - get from actor stats
                        current_value = self.get_actor_stat_value(actor, token).await?;
                    }
                    
                    // Apply the operator
                    match current_operator {
                        '+' => result += current_value,
                        '-' => result -= current_value,
                        '*' => result *= current_value,
                        '/' => if current_value != 0.0 { result /= current_value; },
                        _ => {}
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    /// Get actor stat value
    async fn get_actor_stat_value(&self, actor: &Actor, stat_name: &str) -> ActorCoreResult<f64> {
        // Get stat value from actor data
        let data = actor.get_data();
        
        match stat_name {
            "level" => Ok(data.get("level").and_then(|v| v.as_f64()).unwrap_or(1.0)),
            "vitality" => Ok(data.get("vitality").and_then(|v| v.as_f64()).unwrap_or(10.0)),
            "intelligence" => Ok(data.get("intelligence").and_then(|v| v.as_f64()).unwrap_or(10.0)),
            "constitution" => Ok(data.get("constitution").and_then(|v| v.as_f64()).unwrap_or(10.0)),
            "charisma" => Ok(data.get("charisma").and_then(|v| v.as_f64()).unwrap_or(10.0)),
            "equipment_bonus" => Ok(data.get("equipment_bonus").and_then(|v| v.as_f64()).unwrap_or(0.0)),
            _ => Ok(0.0), // Unknown stat
        }
    }
    
    /// Get resource regeneration rate
    pub fn get_regen_rate(&self, resource_name: &str) -> f64 {
        self.resource_definitions.get(resource_name)
            .map(|def| def.regen_rate)
            .unwrap_or(0.0)
    }
    
    /// Get resource category
    pub fn get_resource_category(&self, resource_name: &str) -> Option<&RpgResourceCategory> {
        self.resource_definitions.get(resource_name)
            .map(|def| &def.category)
    }
    
    /// Get all resources in a category
    pub fn get_resources_by_category(&self, category: &RpgResourceCategory) -> Vec<String> {
        self.resource_definitions.iter()
            .filter(|(_, def)| &def.category == category)
            .map(|(name, _)| name.clone())
            .collect()
    }
    
    /// Calculate resource regeneration
    pub async fn calculate_regeneration(&self, actor: &Actor, resource_name: &str, time_delta: f64) -> ActorCoreResult<f64> {
        let regen_rate = self.get_regen_rate(resource_name);
        let regen_amount = regen_rate * time_delta;
        
        // Get current value
        let current_value = self.calculate_resource_value(actor, resource_name).await?;
        let max_value = self.calculate_max_resource_value(actor, resource_name).await?;
        
        // Calculate new value (don't exceed maximum)
        let new_value = (current_value + regen_amount).min(max_value);
        
        Ok(new_value - current_value) // Return the amount regenerated
    }
}

#[async_trait]
impl SystemResourceCalculator for RpgResourceManager {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn affects_resource(&self, resource_id: &str) -> bool {
        self.resource_definitions.contains_key(resource_id)
    }
    
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut resources = HashMap::new();
        
        for (resource_name, _definition) in &self.resource_definitions {
            let value = self.calculate_resource_value(actor, resource_name).await?;
            resources.insert(resource_name.clone(), value);
        }
        
        Ok(resources)
    }
    
    async fn notify_stat_change(&self, _actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()> {
        // Check if this stat change affects any of our resources
        for stat_id in changed_stats {
            for (resource_name, definition) in &self.resource_definitions {
                if definition.dependencies.contains(stat_id) {
                    // This stat change affects this resource
                    // In a real implementation, you'd trigger recalculation
                    println!("RPG Resource {} affected by stat change: {}", resource_name, stat_id);
                }
            }
        }
        
        Ok(())
    }
    
    fn get_resource_dependencies(&self) -> Vec<String> {
        let mut dependencies = Vec::new();
        for definition in self.resource_definitions.values() {
            dependencies.extend(definition.dependencies.clone());
        }
        dependencies.sort();
        dependencies.dedup();
        dependencies
    }
    
    fn get_resource_categories(&self) -> Vec<super::system_resource_manager::ResourceCategory> {
        use super::system_resource_manager::ResourceCategory;
        vec![
            ResourceCategory::Health,
            ResourceCategory::Energy,
            ResourceCategory::Physical,
            ResourceCategory::Special,
        ]
    }
    
    async fn is_active(&self, _actor: &Actor) -> ActorCoreResult<bool> {
        // RPG system is always active
        Ok(true)
    }
}

/// RPG Resource Events
#[derive(Debug, Clone)]
pub enum RpgResourceEvent {
    /// Resource value changed
    ResourceChanged {
        resource_name: String,
        old_value: f64,
        new_value: f64,
    },
    /// Resource regenerated
    ResourceRegenerated {
        resource_name: String,
        amount: f64,
        new_value: f64,
    },
    /// Resource depleted
    ResourceDepleted {
        resource_name: String,
    },
    /// Resource fully restored
    ResourceFullyRestored {
        resource_name: String,
    },
    /// Level up occurred
    LevelUp {
        new_level: i32,
        new_max_hp: f64,
        new_max_mp: f64,
    },
}

/// RPG Resource Event Handler
pub struct RpgResourceEventHandler {
    /// Event listeners
    listeners: Vec<Box<dyn Fn(RpgResourceEvent) + Send + Sync>>,
}

impl RpgResourceEventHandler {
    /// Create a new event handler
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
    
    /// Add an event listener
    pub fn add_listener<F>(&mut self, listener: F)
    where
        F: Fn(RpgResourceEvent) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }
    
    /// Emit an event
    pub fn emit(&self, event: RpgResourceEvent) {
        for listener in &self.listeners {
            listener(event.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Actor;
    use serde_json::json;
    
    #[tokio::test]
    async fn test_rpg_resource_manager() {
        let manager = RpgResourceManager::new();
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Set actor stats
        let mut data = std::collections::HashMap::new();
        data.insert("level".to_string(), json!(5));
        data.insert("vitality".to_string(), json!(15));
        data.insert("intelligence".to_string(), json!(12));
        data.insert("constitution".to_string(), json!(18));
        data.insert("charisma".to_string(), json!(10));
        data.insert("equipment_bonus".to_string(), json!(5));
        actor.set_data(data);
        
        // Test resource calculation
        let resources = manager.calculate_resources(&actor).await.unwrap();
        
        // Check HP calculation: vitality * 10 + level * 5 = 15 * 10 + 5 * 5 = 150 + 25 = 175
        assert_eq!(resources.get("hp_current"), Some(&175.0));
        
        // Check MP calculation: intelligence * 8 + level * 3 = 12 * 8 + 5 * 3 = 96 + 15 = 111
        assert_eq!(resources.get("mp_current"), Some(&111.0));
        
        // Check stamina calculation: constitution * 6 + level * 2 = 18 * 6 + 5 * 2 = 108 + 10 = 118
        assert_eq!(resources.get("stamina_current"), Some(&118.0));
    }
    
    #[tokio::test]
    async fn test_resource_regeneration() {
        let manager = RpgResourceManager::new();
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Set actor stats
        let mut data = std::collections::HashMap::new();
        data.insert("level".to_string(), json!(1));
        data.insert("vitality".to_string(), json!(10));
        data.insert("intelligence".to_string(), json!(10));
        data.insert("constitution".to_string(), json!(10));
        data.insert("charisma".to_string(), json!(10));
        actor.set_data(data);
        
        // Test HP regeneration over 10 seconds
        let regen_amount = manager.calculate_regeneration(&actor, "hp_current", 10.0).await.unwrap();
        // HP regen rate is 0.1 per second, so 10 seconds = 1.0 HP
        assert_eq!(regen_amount, 1.0);
        
        // Test MP regeneration over 5 seconds
        let regen_amount = manager.calculate_regeneration(&actor, "mp_current", 5.0).await.unwrap();
        // MP regen rate is 0.2 per second, so 5 seconds = 1.0 MP
        assert_eq!(regen_amount, 1.0);
    }
    
    #[tokio::test]
    async fn test_stat_change_notification() {
        let manager = RpgResourceManager::new();
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Set initial stats
        let mut data = std::collections::HashMap::new();
        data.insert("level".to_string(), json!(1));
        data.insert("vitality".to_string(), json!(10));
        actor.set_data(data);
        
        // Notify of vitality change
        manager.notify_stat_change(&actor, &["vitality".to_string()]).await.unwrap();
        
        // This should trigger recalculation of HP since vitality is a dependency
        // The actual implementation would handle this
    }
    
    #[tokio::test]
    async fn test_resource_categories() {
        let manager = RpgResourceManager::new();
        
        // Test health resources
        let health_resources = manager.get_resources_by_category(&RpgResourceCategory::Health);
        assert!(health_resources.contains(&"hp_current".to_string()));
        
        // Test magic resources
        let magic_resources = manager.get_resources_by_category(&RpgResourceCategory::Magic);
        assert!(magic_resources.contains(&"mp_current".to_string()));
        
        // Test stamina resources
        let stamina_resources = manager.get_resources_by_category(&RpgResourceCategory::Stamina);
        assert!(stamina_resources.contains(&"stamina_current".to_string()));
    }
}
