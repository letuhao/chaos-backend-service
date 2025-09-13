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
        // Simple formula evaluator with proper operator precedence
        // First, tokenize the formula
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        
        for ch in formula.chars() {
            match ch {
                '+' | '-' | '*' | '/' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    tokens.push(ch.to_string());
                }
                ' ' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
                _ => current_token.push(ch),
            }
        }
        if !current_token.is_empty() {
            tokens.push(current_token);
        }
        
        // Convert tokens to values and operators
        let mut values = Vec::new();
        let mut operators = Vec::new();
        
        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => operators.push(token),
                _ => {
                    // Try to parse as number or variable
                    let value = if let Ok(num) = token.parse::<f64>() {
                        num
                    } else {
                        self.get_actor_stat_value(actor, &token).await?
                    };
                    values.push(value);
                }
            }
        }
        
        // Handle multiplication and division first (higher precedence)
        let mut i = 0;
        while i < operators.len() {
            match operators[i].as_str() {
                "*" => {
                    let result = values[i] * values[i + 1];
                    values[i] = result;
                    values.remove(i + 1);
                    operators.remove(i);
                }
                "/" => {
                    if values[i + 1] != 0.0 {
                        let result = values[i] / values[i + 1];
                        values[i] = result;
                    }
                    values.remove(i + 1);
                    operators.remove(i);
                }
                _ => i += 1,
            }
        }
        
        // Handle addition and subtraction (lower precedence)
        let mut result = values[0];
        for (i, op) in operators.iter().enumerate() {
            match op.as_str() {
                "+" => result += values[i + 1],
                "-" => result -= values[i + 1],
                _ => {}
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