//! Refactored RPG Resource Manager
//!
//! This example shows how the RPG Resource Manager should be refactored
//! to use the Runtime Registry System instead of hardcoded resource definitions.

use std::sync::Arc;
use actor_core::types::Actor;
use actor_core::ActorCoreResult;
use actor_core::ActorCoreError;
use actor_core::registry::{
    ResourceRegistry, CategoryRegistry, TagRegistry, RegistryManager,
    ResourceDefinition, CategoryDefinition, TagDefinition,
    ResourceType, RegenType, CategoryDefinitionBuilder, TagDefinitionBuilder
};
use tracing::info;

/// Convert string error to ActorCoreError
fn to_actor_core_error(msg: String) -> ActorCoreError {
    ActorCoreError::SubsystemError(msg)
}

/// Refactored RPG Resource Manager using Runtime Registry System
pub struct RpgResourceManagerRefactored {
    /// System identifier
    system_id: String,
    /// Runtime registry manager
    registry_manager: Arc<RegistryManager>,
}

impl RpgResourceManagerRefactored {
    /// Create a new RPG Resource Manager with runtime registries
    pub fn new(registry_manager: Arc<RegistryManager>) -> Self {
        Self {
            system_id: "rpg_system".to_string(),
            registry_manager,
        }
    }
    
    /// Initialize RPG resources from configuration
    /// This should be called during subsystem startup
    pub async fn initialize_from_config(&self) -> ActorCoreResult<()> {
        let resource_registry = self.registry_manager.get_resource_registry();
        let category_registry = self.registry_manager.get_category_registry();
        let tag_registry = self.registry_manager.get_tag_registry();
        
        // Register RPG categories
        self.register_rpg_categories(&category_registry).await?;
        
        // Register RPG tags
        self.register_rpg_tags(&tag_registry).await?;
        
        // Register RPG resources
        self.register_rpg_resources(&resource_registry).await?;
        
        info!("RPG Resource Manager initialized with runtime registries");
        Ok(())
    }
    
    /// Register RPG categories
    async fn register_rpg_categories(&self, category_registry: &Arc<dyn CategoryRegistry>) -> ActorCoreResult<()> {
        // Health category
        let health_category = CategoryDefinition::create_custom_category(
            "health",
            "Health",
            Some("Health and vitality related resources"),
            &self.system_id,
        );
        category_registry.register_category(health_category).await?;
        
        // Magic category
        let magic_category = CategoryDefinition::create_custom_category(
            "magic",
            "Magic",
            Some("Magic and mana related resources"),
            &self.system_id,
        );
        category_registry.register_category(magic_category).await?;
        
        // Physical category
        let physical_category = CategoryDefinition::create_custom_category(
            "physical",
            "Physical",
            Some("Physical and stamina related resources"),
            &self.system_id,
        );
        category_registry.register_category(physical_category).await?;
        
        // Progression category
        let progression_category = CategoryDefinition::create_custom_category(
            "progression",
            "Progression",
            Some("Character progression related resources"),
            &self.system_id,
        );
        category_registry.register_category(progression_category).await?;
        
        // Special category
        let special_category = CategoryDefinition::create_custom_category(
            "special",
            "Special",
            Some("Special ability related resources"),
            &self.system_id,
        );
        category_registry.register_category(special_category).await?;
        
        Ok(())
    }
    
    /// Register RPG tags
    async fn register_rpg_tags(&self, tag_registry: &Arc<dyn TagRegistry>) -> ActorCoreResult<()> {
        // Vital tag
        let vital_tag = TagDefinition::create_vital_tag(&self.system_id);
        tag_registry.register_tag(vital_tag).await?;
        
        // Health tag
        let health_tag = TagDefinition::create_custom_tag(
            "health",
            "Health",
            "resource",
            &self.system_id,
        );
        tag_registry.register_tag(health_tag).await?;
        
        // Magic tag
        let magic_tag = TagDefinition::create_magic_tag(&self.system_id);
        tag_registry.register_tag(magic_tag).await?;
        
        // Physical tag
        let physical_tag = TagDefinition::create_physical_tag(&self.system_id);
        tag_registry.register_tag(physical_tag).await?;
        
        // Progression tag
        let progression_tag = TagDefinition::create_custom_tag(
            "progression",
            "Progression",
            "resource",
            &self.system_id,
        );
        tag_registry.register_tag(progression_tag).await?;
        
        // Special tag
        let special_tag = TagDefinition::create_custom_tag(
            "special",
            "Special",
            "resource",
            &self.system_id,
        );
        tag_registry.register_tag(special_tag).await?;
        
        Ok(())
    }
    
    /// Register RPG resources
    async fn register_rpg_resources(&self, resource_registry: &Arc<dyn ResourceRegistry>) -> ActorCoreResult<()> {
        // Health Points (HP)
        let hp_resource = ResourceDefinition {
            id: "hp_current".to_string(),
            name: "Health Points".to_string(),
            description: Some("Character's current health points".to_string()),
            category: "health".to_string(),
            resource_type: ResourceType::Health,
            base_value: 0.0, // Will be calculated by formula
            min_value: 0.0,
            max_value: 1000.0, // Will be calculated by formula
            regen_rate: 0.1,
            regen_type: RegenType::Passive,
            dependencies: vec!["vitality".to_string(), "level".to_string()],
            tags: vec!["vital".to_string(), "health".to_string(), "rpg".to_string()],
            subsystem_id: self.system_id.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        resource_registry.register_resource(hp_resource).await?;
        
        // Magic Points (MP)
        let mp_resource = ResourceDefinition {
            id: "mp_current".to_string(),
            name: "Magic Points".to_string(),
            description: Some("Character's current magic points".to_string()),
            category: "magic".to_string(),
            resource_type: ResourceType::Mana,
            base_value: 0.0,
            min_value: 0.0,
            max_value: 1000.0,
            regen_rate: 0.2,
            regen_type: RegenType::Passive,
            dependencies: vec!["intelligence".to_string(), "level".to_string()],
            tags: vec!["magic".to_string(), "mana".to_string(), "rpg".to_string()],
            subsystem_id: self.system_id.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        resource_registry.register_resource(mp_resource).await?;
        
        // Stamina
        let stamina_resource = ResourceDefinition {
            id: "stamina_current".to_string(),
            name: "Stamina".to_string(),
            description: Some("Character's current stamina".to_string()),
            category: "physical".to_string(),
            resource_type: ResourceType::Stamina,
            base_value: 0.0,
            min_value: 0.0,
            max_value: 1000.0,
            regen_rate: 0.5,
            regen_type: RegenType::Passive,
            dependencies: vec!["constitution".to_string(), "level".to_string()],
            tags: vec!["physical".to_string(), "stamina".to_string(), "rpg".to_string()],
            subsystem_id: self.system_id.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        resource_registry.register_resource(stamina_resource).await?;
        
        // Experience Points
        let exp_resource = ResourceDefinition {
            id: "exp_current".to_string(),
            name: "Experience Points".to_string(),
            description: Some("Character's current experience points".to_string()),
            category: "progression".to_string(),
            resource_type: ResourceType::Experience,
            base_value: 0.0,
            min_value: 0.0,
            max_value: 1000000.0,
            regen_rate: 0.0,
            regen_type: RegenType::None,
            dependencies: vec!["level".to_string()],
            tags: vec!["progression".to_string(), "experience".to_string(), "rpg".to_string()],
            subsystem_id: self.system_id.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        resource_registry.register_resource(exp_resource).await?;
        
        // Special Ability Points
        let special_resource = ResourceDefinition {
            id: "special_points_current".to_string(),
            name: "Special Ability Points".to_string(),
            description: Some("Character's special ability points".to_string()),
            category: "special".to_string(),
            resource_type: ResourceType::Custom("special".to_string()),
            base_value: 0.0,
            min_value: 0.0,
            max_value: 100.0,
            regen_rate: 0.05,
            regen_type: RegenType::Passive,
            dependencies: vec!["charisma".to_string(), "level".to_string()],
            tags: vec!["special".to_string(), "ability".to_string(), "rpg".to_string()],
            subsystem_id: self.system_id.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        resource_registry.register_resource(special_resource).await?;
        
        Ok(())
    }
    
    /// Get available resources from registry
    pub async fn get_available_resources(&self) -> ActorCoreResult<Vec<String>> {
        let resource_registry = self.registry_manager.get_resource_registry();
        let resources = resource_registry.get_resources_by_subsystem(&self.system_id).await?;
        Ok(resources.into_iter().map(|r| r.id).collect())
    }
    
    /// Calculate resource value using formula (from registry)
    pub async fn calculate_resource_value(&self, actor: &Actor, resource_name: &str) -> ActorCoreResult<f64> {
        let resource_registry = self.registry_manager.get_resource_registry();
        let _resource_def = resource_registry.get_resource(resource_name).await?
            .ok_or_else(|| to_actor_core_error(format!("Resource not found: {}", resource_name)))?;
        
        // Use the formula from the resource definition
        // This would need to be implemented based on the formula parsing system
        // For now, return a simple calculation
        let value = match resource_name {
            "hp_current" => self.calculate_hp(actor).await?,
            "mp_current" => self.calculate_mp(actor).await?,
            "stamina_current" => self.calculate_stamina(actor).await?,
            "exp_current" => self.calculate_exp(actor).await?,
            "special_points_current" => self.calculate_special_points(actor).await?,
            _ => 0.0,
        };
        
        Ok(value)
    }
    
    /// Calculate HP based on vitality and level
    async fn calculate_hp(&self, actor: &Actor) -> ActorCoreResult<f64> {
        // This would use the formula from the resource definition
        // For now, return a simple calculation
        let vitality = self.get_actor_stat(actor, "vitality").await?;
        let level = self.get_actor_stat(actor, "level").await?;
        Ok(vitality * 10.0 + level * 5.0)
    }
    
    /// Calculate MP based on intelligence and level
    async fn calculate_mp(&self, actor: &Actor) -> ActorCoreResult<f64> {
        let intelligence = self.get_actor_stat(actor, "intelligence").await?;
        let level = self.get_actor_stat(actor, "level").await?;
        Ok(intelligence * 8.0 + level * 3.0)
    }
    
    /// Calculate stamina based on constitution and level
    async fn calculate_stamina(&self, actor: &Actor) -> ActorCoreResult<f64> {
        let constitution = self.get_actor_stat(actor, "constitution").await?;
        let level = self.get_actor_stat(actor, "level").await?;
        Ok(constitution * 6.0 + level * 2.0)
    }
    
    /// Calculate experience (starts at 0)
    async fn calculate_exp(&self, _actor: &Actor) -> ActorCoreResult<f64> {
        Ok(0.0) // Experience starts at 0
    }
    
    /// Calculate special points based on charisma and level
    async fn calculate_special_points(&self, actor: &Actor) -> ActorCoreResult<f64> {
        let charisma = self.get_actor_stat(actor, "charisma").await?;
        let level = self.get_actor_stat(actor, "level").await?;
        Ok(charisma * 4.0 + level * 1.0)
    }
    
    /// Get actor stat value (placeholder implementation)
    async fn get_actor_stat(&self, _actor: &Actor, stat_name: &str) -> ActorCoreResult<f64> {
        // This would typically query the actor's stats
        // For now, return mock values
        let value = match stat_name {
            "vitality" => 10.0,
            "intelligence" => 12.0,
            "constitution" => 8.0,
            "charisma" => 14.0,
            "level" => 5.0,
            _ => 0.0,
        };
        Ok(value)
    }
}

/// Example usage of the refactored RPG Resource Manager
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Refactored RPG Resource Manager Example");
    println!("==========================================");
    
    // Create registry manager
    let registry_manager = Arc::new(RegistryManager::new());
    
    // Create RPG Resource Manager
    let rpg_manager = RpgResourceManagerRefactored::new(registry_manager);
    
    // Initialize from configuration
    rpg_manager.initialize_from_config().await?;
    
    // Get available resources
    let resources = rpg_manager.get_available_resources().await?;
    println!("📋 Available RPG Resources:");
    for resource in resources {
        println!("  • {}", resource);
    }
    
    // Create a mock actor
    let actor = Actor::new("player1".to_string(), "human".to_string());
    
    // Calculate resource values
    println!("\n🧮 Resource Calculations:");
    let hp = rpg_manager.calculate_resource_value(&actor, "hp_current").await?;
    let mp = rpg_manager.calculate_resource_value(&actor, "mp_current").await?;
    let stamina = rpg_manager.calculate_resource_value(&actor, "stamina_current").await?;
    let exp = rpg_manager.calculate_resource_value(&actor, "exp_current").await?;
    let special = rpg_manager.calculate_resource_value(&actor, "special_points_current").await?;
    
    println!("  • HP: {:.1}", hp);
    println!("  • MP: {:.1}", mp);
    println!("  • Stamina: {:.1}", stamina);
    println!("  • Experience: {:.1}", exp);
    println!("  • Special Points: {:.1}", special);
    
    println!("\n✅ Refactored RPG Resource Manager working with Runtime Registry System!");
    Ok(())
}
