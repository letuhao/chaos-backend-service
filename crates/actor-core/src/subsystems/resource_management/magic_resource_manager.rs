//! Magic Resource Manager
//!
//! This module provides a comprehensive magic resource management system
//! for the Enhanced Hybrid Resource Manager, handling magical resources
//! like mana, spell slots, arcane energy, and magical focus.

use async_trait::async_trait;
use std::collections::HashMap;
use crate::types::Actor;
use crate::ActorCoreResult;
use super::system_resource_manager::SystemResourceCalculator;

/// Convert string error to ActorCoreError
fn to_actor_core_error(msg: String) -> crate::ActorCoreError {
    crate::ActorCoreError::SubsystemError(msg)
}

/// Magic Resource Manager for magical systems
pub struct MagicResourceManager {
    /// System identifier
    system_id: String,
    /// Resource definitions
    resource_definitions: HashMap<String, MagicResourceDefinition>,
    /// Spell slot definitions
    spell_slot_definitions: HashMap<i32, SpellSlotDefinition>,
}

/// Magic Resource Definition
#[derive(Debug, Clone)]
pub struct MagicResourceDefinition {
    /// Resource name
    pub name: String,
    /// Base value calculation formula
    pub base_formula: String,
    /// Regeneration rate per second
    pub regen_rate: f64,
    /// Maximum value formula
    pub max_formula: String,
    /// Resource category
    pub category: MagicResourceCategory,
    /// Dependencies on other resources
    pub dependencies: Vec<String>,
    /// Magical school affinity
    pub school: MagicSchool,
}

/// Magic Resource Categories
#[derive(Debug, Clone, PartialEq)]
pub enum MagicResourceCategory {
    /// Mana and magical energy
    Mana,
    /// Spell slots and casting capacity
    SpellSlots,
    /// Arcane focus and concentration
    Focus,
    /// Magical knowledge and understanding
    Knowledge,
    /// Magical essence and power
    Essence,
}

/// Magic Schools
#[derive(Debug, Clone, PartialEq)]
pub enum MagicSchool {
    /// Elemental magic
    Elemental,
    /// Divine magic
    Divine,
    /// Arcane magic
    Arcane,
    /// Nature magic
    Nature,
    /// Shadow magic
    Shadow,
    /// Universal magic
    Universal,
}

/// Spell Slot Definition
#[derive(Debug, Clone)]
pub struct SpellSlotDefinition {
    /// Spell level
    pub level: i32,
    /// Base number of slots
    pub base_slots: i32,
    /// Calculation formula
    pub formula: String,
    /// Regeneration time in seconds
    pub regen_time: f64,
}

impl MagicResourceManager {
    /// Create a new Magic Resource Manager
    pub fn new() -> Self {
        let mut manager = Self {
            system_id: "magic_system".to_string(),
            resource_definitions: HashMap::new(),
            spell_slot_definitions: HashMap::new(),
        };
        
        // Initialize default resource definitions
        manager.initialize_default_resources();
        manager.initialize_spell_slots();
        manager
    }
    
    /// Initialize default magic resources
    fn initialize_default_resources(&mut self) {
        // Mana Points
        self.add_resource_definition(MagicResourceDefinition {
            name: "mana_current".to_string(),
            base_formula: "intelligence * 12 + wisdom * 8 + level * 4".to_string(),
            regen_rate: 0.3, // 0.3 mana per second
            max_formula: "intelligence * 12 + wisdom * 8 + level * 4 + equipment_bonus".to_string(),
            category: MagicResourceCategory::Mana,
            dependencies: vec!["intelligence".to_string(), "wisdom".to_string(), "level".to_string()],
            school: MagicSchool::Universal,
        });
        
        // Arcane Focus
        self.add_resource_definition(MagicResourceDefinition {
            name: "arcane_focus_current".to_string(),
            base_formula: "wisdom * 6 + charisma * 4 + level * 2".to_string(),
            regen_rate: 0.1, // 0.1 focus per second
            max_formula: "wisdom * 6 + charisma * 4 + level * 2 + equipment_bonus".to_string(),
            category: MagicResourceCategory::Focus,
            dependencies: vec!["wisdom".to_string(), "charisma".to_string(), "level".to_string()],
            school: MagicSchool::Arcane,
        });
        
        // Magical Knowledge
        self.add_resource_definition(MagicResourceDefinition {
            name: "magical_knowledge_current".to_string(),
            base_formula: "intelligence * 10 + level * 5".to_string(),
            regen_rate: 0.0, // Knowledge doesn't regenerate
            max_formula: "intelligence * 10 + level * 5 + study_bonus".to_string(),
            category: MagicResourceCategory::Knowledge,
            dependencies: vec!["intelligence".to_string(), "level".to_string()],
            school: MagicSchool::Universal,
        });
        
        // Divine Energy
        self.add_resource_definition(MagicResourceDefinition {
            name: "divine_energy_current".to_string(),
            base_formula: "wisdom * 8 + charisma * 6 + level * 3".to_string(),
            regen_rate: 0.2, // 0.2 energy per second
            max_formula: "wisdom * 8 + charisma * 6 + level * 3 + faith_bonus".to_string(),
            category: MagicResourceCategory::Essence,
            dependencies: vec!["wisdom".to_string(), "charisma".to_string(), "level".to_string()],
            school: MagicSchool::Divine,
        });
        
        // Nature Essence
        self.add_resource_definition(MagicResourceDefinition {
            name: "nature_essence_current".to_string(),
            base_formula: "wisdom * 7 + constitution * 5 + level * 2".to_string(),
            regen_rate: 0.15, // 0.15 essence per second
            max_formula: "wisdom * 7 + constitution * 5 + level * 2 + nature_bonus".to_string(),
            category: MagicResourceCategory::Essence,
            dependencies: vec!["wisdom".to_string(), "constitution".to_string(), "level".to_string()],
            school: MagicSchool::Nature,
        });
    }
    
    /// Initialize spell slot definitions
    fn initialize_spell_slots(&mut self) {
        // Level 1 spells
        self.add_spell_slot_definition(SpellSlotDefinition {
            level: 1,
            base_slots: 2,
            formula: "level / 2 + 1".to_string(),
            regen_time: 3600.0, // 1 hour
        });
        
        // Level 2 spells
        self.add_spell_slot_definition(SpellSlotDefinition {
            level: 2,
            base_slots: 1,
            formula: "level / 3".to_string(),
            regen_time: 7200.0, // 2 hours
        });
        
        // Level 3 spells
        self.add_spell_slot_definition(SpellSlotDefinition {
            level: 3,
            base_slots: 1,
            formula: "level / 4".to_string(),
            regen_time: 14400.0, // 4 hours
        });
        
        // Level 4 spells
        self.add_spell_slot_definition(SpellSlotDefinition {
            level: 4,
            base_slots: 1,
            formula: "level / 6".to_string(),
            regen_time: 28800.0, // 8 hours
        });
        
        // Level 5 spells
        self.add_spell_slot_definition(SpellSlotDefinition {
            level: 5,
            base_slots: 1,
            formula: "level / 8".to_string(),
            regen_time: 86400.0, // 24 hours
        });
    }
    
    /// Add a resource definition
    pub fn add_resource_definition(&mut self, definition: MagicResourceDefinition) {
        self.resource_definitions.insert(definition.name.clone(), definition);
    }
    
    /// Add a spell slot definition
    pub fn add_spell_slot_definition(&mut self, definition: SpellSlotDefinition) {
        self.spell_slot_definitions.insert(definition.level, definition);
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
    
    /// Calculate spell slots for a given level
    async fn calculate_spell_slots(&self, actor: &Actor, spell_level: i32) -> ActorCoreResult<i32> {
        let definition = self.spell_slot_definitions.get(&spell_level)
            .ok_or_else(|| to_actor_core_error(format!("Spell slot definition not found for level: {}", spell_level)))?;
        
        // Evaluate the formula
        let slots = self.evaluate_formula(actor, &definition.formula).await?;
        Ok(slots as i32)
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
            "intelligence" => Ok(data.get("intelligence").and_then(|v| v.as_f64()).unwrap_or(10.0)),
            "wisdom" => Ok(data.get("wisdom").and_then(|v| v.as_f64()).unwrap_or(10.0)),
            "charisma" => Ok(data.get("charisma").and_then(|v| v.as_f64()).unwrap_or(10.0)),
            "constitution" => Ok(data.get("constitution").and_then(|v| v.as_f64()).unwrap_or(10.0)),
            "equipment_bonus" => Ok(data.get("equipment_bonus").and_then(|v| v.as_f64()).unwrap_or(0.0)),
            "study_bonus" => Ok(data.get("study_bonus").and_then(|v| v.as_f64()).unwrap_or(0.0)),
            "faith_bonus" => Ok(data.get("faith_bonus").and_then(|v| v.as_f64()).unwrap_or(0.0)),
            "nature_bonus" => Ok(data.get("nature_bonus").and_then(|v| v.as_f64()).unwrap_or(0.0)),
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
    pub fn get_resource_category(&self, resource_name: &str) -> Option<&MagicResourceCategory> {
        self.resource_definitions.get(resource_name)
            .map(|def| &def.category)
    }
    
    /// Get magic school
    pub fn get_magic_school(&self, resource_name: &str) -> Option<&MagicSchool> {
        self.resource_definitions.get(resource_name)
            .map(|def| &def.school)
    }
    
    /// Get all resources in a category
    pub fn get_resources_by_category(&self, category: &MagicResourceCategory) -> Vec<String> {
        self.resource_definitions.iter()
            .filter(|(_, def)| &def.category == category)
            .map(|(name, _)| name.clone())
            .collect()
    }
    
    /// Get all resources for a magic school
    pub fn get_resources_by_school(&self, school: &MagicSchool) -> Vec<String> {
        self.resource_definitions.iter()
            .filter(|(_, def)| &def.school == school)
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
    
    /// Calculate spell slot regeneration
    pub async fn calculate_spell_slot_regeneration(&self, _actor: &Actor, spell_level: i32, time_delta: f64) -> ActorCoreResult<f64> {
        let definition = self.spell_slot_definitions.get(&spell_level)
            .ok_or_else(|| to_actor_core_error(format!("Spell slot definition not found for level: {}", spell_level)))?;
        
        // Calculate regeneration based on time
        let regen_progress = time_delta / definition.regen_time;
        Ok(regen_progress.min(1.0)) // Return progress (0.0 to 1.0)
    }
}

#[async_trait]
impl SystemResourceCalculator for MagicResourceManager {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn affects_resource(&self, resource_id: &str) -> bool {
        self.resource_definitions.contains_key(resource_id)
    }
    
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut resources = HashMap::new();
        
        // Calculate regular resources
        for (resource_name, _definition) in &self.resource_definitions {
            let value = self.calculate_resource_value(actor, resource_name).await?;
            resources.insert(resource_name.clone(), value);
        }
        
        // Calculate spell slots
        for (spell_level, _definition) in &self.spell_slot_definitions {
            let slots = self.calculate_spell_slots(actor, *spell_level).await?;
            resources.insert(format!("spell_slots_level_{}", spell_level), slots as f64);
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
                    println!("Magic Resource {} affected by stat change: {}", resource_name, stat_id);
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
            ResourceCategory::Energy,
            ResourceCategory::Cultivation,
            ResourceCategory::Special,
        ]
    }
    
    async fn is_active(&self, _actor: &Actor) -> ActorCoreResult<bool> {
        // Magic system is always active
        Ok(true)
    }
}

/// Magic Resource Events
#[derive(Debug, Clone)]
pub enum MagicResourceEvent {
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
    /// Spell slot used
    SpellSlotUsed {
        spell_level: i32,
        remaining_slots: i32,
    },
    /// Spell slot regenerated
    SpellSlotRegenerated {
        spell_level: i32,
        new_slots: i32,
    },
    /// Magical school affinity changed
    SchoolAffinityChanged {
        school: MagicSchool,
        new_affinity: f64,
    },
}

/// Magic Resource Event Handler
pub struct MagicResourceEventHandler {
    /// Event listeners
    listeners: Vec<Box<dyn Fn(MagicResourceEvent) + Send + Sync>>,
}

impl MagicResourceEventHandler {
    /// Create a new event handler
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
    
    /// Add an event listener
    pub fn add_listener<F>(&mut self, listener: F)
    where
        F: Fn(MagicResourceEvent) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }
    
    /// Emit an event
    pub fn emit(&self, event: MagicResourceEvent) {
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
    async fn test_magic_resource_manager() {
        let manager = MagicResourceManager::new();
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Set actor stats
        let mut data = std::collections::HashMap::new();
        data.insert("level".to_string(), json!(5));
        data.insert("intelligence".to_string(), json!(15));
        data.insert("wisdom".to_string(), json!(12));
        data.insert("charisma".to_string(), json!(10));
        data.insert("constitution".to_string(), json!(8));
        data.insert("equipment_bonus".to_string(), json!(3));
        actor.set_data(data);
        
        // Test resource calculation
        let resources = manager.calculate_resources(&actor).await.unwrap();
        
        // Check mana calculation: intelligence * 12 + wisdom * 8 + level * 4 = 15 * 12 + 12 * 8 + 5 * 4 = 180 + 96 + 20 = 296
        assert_eq!(resources.get("mana_current"), Some(&296.0));
        
        // Check arcane focus calculation: wisdom * 6 + charisma * 4 + level * 2 = 12 * 6 + 10 * 4 + 5 * 2 = 72 + 40 + 10 = 122
        assert_eq!(resources.get("arcane_focus_current"), Some(&122.0));
    }
    
    #[tokio::test]
    async fn test_spell_slots() {
        let manager = MagicResourceManager::new();
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Set actor level
        let mut data = std::collections::HashMap::new();
        data.insert("level".to_string(), json!(6));
        actor.set_data(data);
        
        // Test spell slot calculation
        let resources = manager.calculate_resources(&actor).await.unwrap();
        
        // Level 1 spells: level / 2 + 1 = 6 / 2 + 1 = 3 + 1 = 4
        assert_eq!(resources.get("spell_slots_level_1"), Some(&4.0));
        
        // Level 2 spells: level / 3 = 6 / 3 = 2
        assert_eq!(resources.get("spell_slots_level_2"), Some(&2.0));
    }
    
    #[tokio::test]
    async fn test_resource_regeneration() {
        let manager = MagicResourceManager::new();
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Set actor stats
        let mut data = std::collections::HashMap::new();
        data.insert("level".to_string(), json!(1));
        data.insert("intelligence".to_string(), json!(10));
        data.insert("wisdom".to_string(), json!(10));
        data.insert("equipment_bonus".to_string(), json!(10)); // Add equipment bonus to create room for regen
        actor.set_data(data);
        
        // Test mana regeneration over 10 seconds
        let regen_amount = manager.calculate_regeneration(&actor, "mana_current", 10.0).await.unwrap();
        // Mana regen rate is 0.3 per second, so 10 seconds = 3.0 mana
        assert_eq!(regen_amount, 3.0);
    }
    
    #[tokio::test]
    async fn test_magic_schools() {
        let manager = MagicResourceManager::new();
        
        // Test universal resources
        let universal_resources = manager.get_resources_by_school(&MagicSchool::Universal);
        assert!(universal_resources.contains(&"mana_current".to_string()));
        assert!(universal_resources.contains(&"magical_knowledge_current".to_string()));
        
        // Test divine resources
        let divine_resources = manager.get_resources_by_school(&MagicSchool::Divine);
        assert!(divine_resources.contains(&"divine_energy_current".to_string()));
        
        // Test nature resources
        let nature_resources = manager.get_resources_by_school(&MagicSchool::Nature);
        assert!(nature_resources.contains(&"nature_essence_current".to_string()));
    }
}
