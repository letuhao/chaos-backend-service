//! System Resource Manager Interface
//!
//! This module provides the interface for system-specific resource managers
//! that can be delegated to by the Enhanced Hybrid Resource Manager.

use async_trait::async_trait;
use std::collections::HashMap;
use crate::types::Actor;
use crate::ActorCoreResult;

/// System Resource Calculator trait for delegation
#[async_trait]
pub trait SystemResourceCalculator: Send + Sync {
    /// Calculate resources for this system
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>>;
    
    /// Get system identifier
    fn system_id(&self) -> &str;
    
    /// Check if this system affects a specific resource
    fn affects_resource(&self, resource_id: &str) -> bool;
    
    /// Notify of stat changes
    async fn notify_stat_change(&self, actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()>;
    
    /// Get resource dependencies for this system
    fn get_resource_dependencies(&self) -> Vec<String>;
    
    /// Get resource categories this system handles
    fn get_resource_categories(&self) -> Vec<ResourceCategory>;
    
    /// Check if this system is active for the actor
    async fn is_active(&self, actor: &Actor) -> ActorCoreResult<bool>;
}

/// Resource categories that systems can handle
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceCategory {
    Health,      // HP, lifespan
    Energy,      // Mana, spiritual energy
    Physical,    // Stamina, vitality
    Cultivation, // Qi, dao energy
    Special,     // Shield, temporary effects
}

/// Base implementation for system resource calculators
#[derive(Debug)]
pub struct BaseSystemResourceCalculator {
    /// System identifier
    system_id: String,
    /// Resource categories this system handles
    resource_categories: Vec<ResourceCategory>,
    /// Resource dependencies
    resource_dependencies: Vec<String>,
    /// Resource definitions for this system
    resource_definitions: HashMap<String, SystemResourceDefinition>,
}

/// System resource definition
#[derive(Debug, Clone)]
pub struct SystemResourceDefinition {
    /// Resource identifier
    pub id: String,
    /// Resource name
    pub name: String,
    /// Resource category
    pub category: ResourceCategory,
    /// Base value calculation function
    pub base_value_calculator: fn(&Actor) -> f64,
    /// Regeneration rate calculation function
    pub regen_rate_calculator: fn(&Actor) -> f64,
    /// Dependencies on other resources
    pub dependencies: Vec<String>,
    /// Additional tags
    pub tags: HashMap<String, String>,
}

impl BaseSystemResourceCalculator {
    /// Create a new base system resource calculator
    pub fn new(
        system_id: String,
        resource_categories: Vec<ResourceCategory>,
        resource_dependencies: Vec<String>,
    ) -> Self {
        Self {
            system_id,
            resource_categories,
            resource_dependencies,
            resource_definitions: HashMap::new(),
        }
    }
    
    /// Add a resource definition
    pub fn add_resource_definition(&mut self, definition: SystemResourceDefinition) {
        self.resource_definitions.insert(definition.id.clone(), definition);
    }
    
    /// Calculate base value for a resource
    fn calculate_base_value(&self, definition: &SystemResourceDefinition, actor: &Actor) -> f64 {
        (definition.base_value_calculator)(actor)
    }
    
    /// Calculate regeneration rate for a resource
    fn calculate_regen_rate(&self, definition: &SystemResourceDefinition, actor: &Actor) -> f64 {
        (definition.regen_rate_calculator)(actor)
    }
}

#[async_trait]
impl SystemResourceCalculator for BaseSystemResourceCalculator {
    /// Calculate resources for this system
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut resources = HashMap::new();
        
        for (resource_id, definition) in &self.resource_definitions {
            // Calculate base value
            let base_value = self.calculate_base_value(definition, actor);
            
            // Set current and max values
            resources.insert(format!("{}_current", resource_id), base_value);
            resources.insert(format!("{}_max", resource_id), base_value);
            
            // Calculate regeneration rate
            let regen_rate = self.calculate_regen_rate(definition, actor);
            if regen_rate > 0.0 {
                resources.insert(format!("{}_regen", resource_id), regen_rate);
            }
        }
        
        Ok(resources)
    }
    
    /// Get system identifier
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    /// Check if this system affects a specific resource
    fn affects_resource(&self, resource_id: &str) -> bool {
        // Check if any of our resource definitions match
        for (def_id, _) in &self.resource_definitions {
            if resource_id.starts_with(def_id) {
                return true;
            }
        }
        false
    }
    
    /// Notify of stat changes
    async fn notify_stat_change(&self, _actor: &Actor, _changed_stats: &[String]) -> ActorCoreResult<()> {
        // Default implementation - no-op
        // Subclasses can override this to handle stat changes
        Ok(())
    }
    
    /// Get resource dependencies for this system
    fn get_resource_dependencies(&self) -> Vec<String> {
        self.resource_dependencies.clone()
    }
    
    /// Get resource categories this system handles
    fn get_resource_categories(&self) -> Vec<ResourceCategory> {
        self.resource_categories.clone()
    }
    
    /// Check if this system is active for the actor
    async fn is_active(&self, actor: &Actor) -> ActorCoreResult<bool> {
        // Default implementation - check if actor has this system
        Ok(actor.has_subsystem(&self.system_id))
    }
}

/// Jindan System Resource Calculator
#[derive(Debug)]
pub struct JindanSystemResourceCalculator {
    /// Base calculator
    base: BaseSystemResourceCalculator,
}

impl JindanSystemResourceCalculator {
    /// Create a new Jindan system resource calculator
    pub fn new() -> Self {
        let mut base = BaseSystemResourceCalculator::new(
            "jindan_system".to_string(),
            vec![ResourceCategory::Cultivation, ResourceCategory::Energy],
            vec!["cultivation_level".to_string(), "realm".to_string()],
        );
        
        // Add Jindan-specific resources
        base.add_resource_definition(SystemResourceDefinition {
            id: "qi".to_string(),
            name: "Qi Energy".to_string(),
            category: ResourceCategory::Cultivation,
            base_value_calculator: |actor| {
                // Base qi from cultivation level
                let cultivation_level = actor.get_data().get("cultivation_level")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                cultivation_level * 10.0
            },
            regen_rate_calculator: |actor| {
                // Qi regeneration based on realm
                let realm = actor.get_data().get("realm")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                realm * 0.5
            },
            dependencies: vec!["cultivation_level".to_string()],
            tags: HashMap::from([
                ("cultivation_related".to_string(), "true".to_string()),
                ("realm_dependent".to_string(), "true".to_string()),
            ]),
        });
        
        base.add_resource_definition(SystemResourceDefinition {
            id: "spiritual_energy".to_string(),
            name: "Spiritual Energy".to_string(),
            category: ResourceCategory::Energy,
            base_value_calculator: |actor| {
                // Spiritual energy from age and cultivation
                let age = actor.age as f64;
                let cultivation_level = actor.get_data().get("cultivation_level")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                age * 2.0 + cultivation_level * 5.0
            },
            regen_rate_calculator: |actor| {
                // Spiritual energy regeneration
                let cultivation_level = actor.get_data().get("cultivation_level")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                cultivation_level * 1.5
            },
            dependencies: vec!["age".to_string(), "cultivation_level".to_string()],
            tags: HashMap::from([
                ("cultivation_related".to_string(), "true".to_string()),
                ("meditation_required".to_string(), "true".to_string()),
            ]),
        });
        
        Self { base }
    }
}

#[async_trait]
impl SystemResourceCalculator for JindanSystemResourceCalculator {
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        self.base.calculate_resources(actor).await
    }
    
    fn system_id(&self) -> &str {
        self.base.system_id()
    }
    
    fn affects_resource(&self, resource_id: &str) -> bool {
        self.base.affects_resource(resource_id)
    }
    
    async fn notify_stat_change(&self, _actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()> {
        // Jindan system specific stat change handling
        if changed_stats.contains(&"cultivation_level".to_string()) {
            // Recalculate qi and spiritual energy
            // This would trigger resource recalculation
        }
        Ok(())
    }
    
    fn get_resource_dependencies(&self) -> Vec<String> {
        self.base.get_resource_dependencies()
    }
    
    fn get_resource_categories(&self) -> Vec<ResourceCategory> {
        self.base.get_resource_categories()
    }
    
    async fn is_active(&self, actor: &Actor) -> ActorCoreResult<bool> {
        self.base.is_active(actor).await
    }
}

/// RPG System Resource Calculator
#[derive(Debug)]
pub struct RpgSystemResourceCalculator {
    /// Base calculator
    base: BaseSystemResourceCalculator,
}

impl RpgSystemResourceCalculator {
    /// Create a new RPG system resource calculator
    pub fn new() -> Self {
        let mut base = BaseSystemResourceCalculator::new(
            "rpg_system".to_string(),
            vec![ResourceCategory::Health, ResourceCategory::Physical],
            vec!["level".to_string(), "vitality".to_string()],
        );
        
        // Add RPG-specific resources
        base.add_resource_definition(SystemResourceDefinition {
            id: "vitality".to_string(),
            name: "Vitality".to_string(),
            category: ResourceCategory::Physical,
            base_value_calculator: |actor| {
                // Vitality from level and race
                let level = actor.get_data().get("level")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                level * 5.0
            },
            regen_rate_calculator: |actor| {
                // Vitality regeneration
                let level = actor.get_data().get("level")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                level * 0.5
            },
            dependencies: vec!["level".to_string()],
            tags: HashMap::from([
                ("rpg_related".to_string(), "true".to_string()),
                ("level_dependent".to_string(), "true".to_string()),
            ]),
        });
        
        base.add_resource_definition(SystemResourceDefinition {
            id: "experience".to_string(),
            name: "Experience Points".to_string(),
            category: ResourceCategory::Special,
            base_value_calculator: |actor| {
                // Experience from level
                let level = actor.get_data().get("level")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                level * 100.0
            },
            regen_rate_calculator: |_| 0.0, // Experience doesn't regenerate
            dependencies: vec!["level".to_string()],
            tags: HashMap::from([
                ("rpg_related".to_string(), "true".to_string()),
                ("level_dependent".to_string(), "true".to_string()),
            ]),
        });
        
        Self { base }
    }
}

#[async_trait]
impl SystemResourceCalculator for RpgSystemResourceCalculator {
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        self.base.calculate_resources(actor).await
    }
    
    fn system_id(&self) -> &str {
        self.base.system_id()
    }
    
    fn affects_resource(&self, resource_id: &str) -> bool {
        self.base.affects_resource(resource_id)
    }
    
    async fn notify_stat_change(&self, _actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()> {
        // RPG system specific stat change handling
        if changed_stats.contains(&"level".to_string()) {
            // Recalculate vitality and experience
            // This would trigger resource recalculation
        }
        Ok(())
    }
    
    fn get_resource_dependencies(&self) -> Vec<String> {
        self.base.get_resource_dependencies()
    }
    
    fn get_resource_categories(&self) -> Vec<ResourceCategory> {
        self.base.get_resource_categories()
    }
    
    async fn is_active(&self, actor: &Actor) -> ActorCoreResult<bool> {
        self.base.is_active(actor).await
    }
}

/// Magic System Resource Calculator
#[derive(Debug)]
pub struct MagicSystemResourceCalculator {
    /// Base calculator
    base: BaseSystemResourceCalculator,
}

impl MagicSystemResourceCalculator {
    /// Create a new Magic system resource calculator
    pub fn new() -> Self {
        let mut base = BaseSystemResourceCalculator::new(
            "magic_system".to_string(),
            vec![ResourceCategory::Energy, ResourceCategory::Special],
            vec!["intelligence".to_string(), "magic_level".to_string()],
        );
        
        // Add Magic-specific resources
        base.add_resource_definition(SystemResourceDefinition {
            id: "magic_power".to_string(),
            name: "Magic Power".to_string(),
            category: ResourceCategory::Energy,
            base_value_calculator: |actor| {
                // Magic power from intelligence and magic level
                let intelligence = actor.get_data().get("intelligence")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(10.0);
                let magic_level = actor.get_data().get("magic_level")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                intelligence * magic_level * 2.0
            },
            regen_rate_calculator: |actor| {
                // Magic power regeneration
                let magic_level = actor.get_data().get("magic_level")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                magic_level * 1.0
            },
            dependencies: vec!["intelligence".to_string(), "magic_level".to_string()],
            tags: HashMap::from([
                ("magic_related".to_string(), "true".to_string()),
                ("intelligence_dependent".to_string(), "true".to_string()),
            ]),
        });
        
        base.add_resource_definition(SystemResourceDefinition {
            id: "spell_slots".to_string(),
            name: "Spell Slots".to_string(),
            category: ResourceCategory::Special,
            base_value_calculator: |actor| {
                // Spell slots from magic level
                let magic_level = actor.get_data().get("magic_level")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);
                magic_level * 2.0
            },
            regen_rate_calculator: |_| 0.0, // Spell slots don't regenerate automatically
            dependencies: vec!["magic_level".to_string()],
            tags: HashMap::from([
                ("magic_related".to_string(), "true".to_string()),
                ("spell_dependent".to_string(), "true".to_string()),
            ]),
        });
        
        Self { base }
    }
}

#[async_trait]
impl SystemResourceCalculator for MagicSystemResourceCalculator {
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        self.base.calculate_resources(actor).await
    }
    
    fn system_id(&self) -> &str {
        self.base.system_id()
    }
    
    fn affects_resource(&self, resource_id: &str) -> bool {
        self.base.affects_resource(resource_id)
    }
    
    async fn notify_stat_change(&self, _actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()> {
        // Magic system specific stat change handling
        if changed_stats.contains(&"magic_level".to_string()) {
            // Recalculate magic power and spell slots
            // This would trigger resource recalculation
        }
        Ok(())
    }
    
    fn get_resource_dependencies(&self) -> Vec<String> {
        self.base.get_resource_dependencies()
    }
    
    fn get_resource_categories(&self) -> Vec<ResourceCategory> {
        self.base.get_resource_categories()
    }
    
    async fn is_active(&self, actor: &Actor) -> ActorCoreResult<bool> {
        self.base.is_active(actor).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Actor;
    
    #[test]
    fn test_base_system_resource_calculator() {
        let calculator = BaseSystemResourceCalculator::new(
            "test_system".to_string(),
            vec![ResourceCategory::Health],
            vec!["vitality".to_string()],
        );
        
        assert_eq!(calculator.system_id(), "test_system");
        assert!(calculator.get_resource_categories().contains(&ResourceCategory::Health));
        assert!(calculator.get_resource_dependencies().contains(&"vitality".to_string()));
    }
    
    #[tokio::test]
    async fn test_jindan_system_calculator() {
        let calculator = JindanSystemResourceCalculator::new();
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Add cultivation level
        let mut data = std::collections::HashMap::new();
        data.insert("cultivation_level".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(5.0).unwrap()));
        data.insert("realm".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(2.0).unwrap()));
        actor.set_data(data);
        
        let resources = calculator.calculate_resources(&actor).await.unwrap();
        
        // Check that qi and spiritual energy are calculated
        assert!(resources.contains_key("qi_current"));
        assert!(resources.contains_key("qi_max"));
        assert!(resources.contains_key("spiritual_energy_current"));
        assert!(resources.contains_key("spiritual_energy_max"));
    }
    
    #[tokio::test]
    async fn test_rpg_system_calculator() {
        let calculator = RpgSystemResourceCalculator::new();
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Add level
        let mut data = std::collections::HashMap::new();
        data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(10.0).unwrap()));
        actor.set_data(data);
        
        let resources = calculator.calculate_resources(&actor).await.unwrap();
        
        // Check that vitality and experience are calculated
        assert!(resources.contains_key("vitality_current"));
        assert!(resources.contains_key("vitality_max"));
        assert!(resources.contains_key("experience_current"));
        assert!(resources.contains_key("experience_max"));
    }
    
    #[tokio::test]
    async fn test_magic_system_calculator() {
        let calculator = MagicSystemResourceCalculator::new();
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Add intelligence and magic level
        let mut data = std::collections::HashMap::new();
        data.insert("intelligence".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(15.0).unwrap()));
        data.insert("magic_level".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(3.0).unwrap()));
        actor.set_data(data);
        
        let resources = calculator.calculate_resources(&actor).await.unwrap();
        
        // Check that magic power and spell slots are calculated
        assert!(resources.contains_key("magic_power_current"));
        assert!(resources.contains_key("magic_power_max"));
        assert!(resources.contains_key("spell_slots_current"));
        assert!(resources.contains_key("spell_slots_max"));
    }
}
