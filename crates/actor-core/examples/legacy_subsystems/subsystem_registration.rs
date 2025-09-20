//! Subsystem Registration Interface
//!
//! This module provides interfaces for subsystems to register their resources,
//! categories, and tags with the runtime registries.

use async_trait::async_trait;
use std::sync::Arc;
use crate::interfaces::Subsystem;
use crate::ActorCoreResult;
use super::runtime_registries::{ResourceDefinition, CategoryDefinition, TagDefinition, RegistryManager};

/// Trait for subsystems that can register resources, categories, and tags
#[async_trait]
pub trait RegistryAwareSubsystem: Subsystem {
    /// Register resources provided by this subsystem
    async fn register_resources(&self, registry: Arc<dyn super::runtime_registries::ResourceRegistry>) -> ActorCoreResult<()>;
    
    /// Register categories provided by this subsystem
    async fn register_categories(&self, registry: Arc<dyn super::runtime_registries::CategoryRegistry>) -> ActorCoreResult<()>;
    
    /// Register tags provided by this subsystem
    async fn register_tags(&self, registry: Arc<dyn super::runtime_registries::TagRegistry>) -> ActorCoreResult<()>;
    
    /// Unregister all resources/categories/tags from this subsystem
    async fn unregister_all(&self, registry_manager: &RegistryManager) -> ActorCoreResult<()>;
}

/// Helper trait for creating default resource definitions
pub trait ResourceDefinitionBuilder {
    fn create_health_resource(subsystem_id: &str) -> ResourceDefinition;
    fn create_mana_resource(subsystem_id: &str) -> ResourceDefinition;
    fn create_stamina_resource(subsystem_id: &str) -> ResourceDefinition;
    fn create_sanity_resource(subsystem_id: &str) -> ResourceDefinition;
    fn create_custom_resource(
        id: &str,
        name: &str,
        category: &str,
        resource_type: super::runtime_registries::ResourceType,
        subsystem_id: &str,
    ) -> ResourceDefinition;
}

impl ResourceDefinitionBuilder for ResourceDefinition {
    fn create_health_resource(subsystem_id: &str) -> ResourceDefinition {
        ResourceDefinition {
            id: "health".to_string(),
            name: "Health".to_string(),
            description: Some("Character health points".to_string()),
            category: "vital".to_string(),
            resource_type: super::runtime_registries::ResourceType::Health,
            base_value: 100.0,
            min_value: 0.0,
            max_value: 1000.0,
            regen_rate: 1.0,
            regen_type: super::runtime_registries::RegenType::Passive,
            dependencies: vec![],
            tags: vec!["vital".to_string(), "health".to_string()],
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_mana_resource(subsystem_id: &str) -> ResourceDefinition {
        ResourceDefinition {
            id: "mana".to_string(),
            name: "Mana".to_string(),
            description: Some("Magical energy for casting spells".to_string()),
            category: "magic".to_string(),
            resource_type: super::runtime_registries::ResourceType::Mana,
            base_value: 100.0,
            min_value: 0.0,
            max_value: 1000.0,
            regen_rate: 5.0,
            regen_type: super::runtime_registries::RegenType::Passive,
            dependencies: vec![],
            tags: vec!["magic".to_string(), "energy".to_string()],
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_stamina_resource(subsystem_id: &str) -> ResourceDefinition {
        ResourceDefinition {
            id: "stamina".to_string(),
            name: "Stamina".to_string(),
            description: Some("Physical energy for actions".to_string()),
            category: "physical".to_string(),
            resource_type: super::runtime_registries::ResourceType::Stamina,
            base_value: 100.0,
            min_value: 0.0,
            max_value: 1000.0,
            regen_rate: 3.0,
            regen_type: super::runtime_registries::RegenType::Passive,
            dependencies: vec![],
            tags: vec!["physical".to_string(), "energy".to_string()],
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_sanity_resource(subsystem_id: &str) -> ResourceDefinition {
        ResourceDefinition {
            id: "sanity".to_string(),
            name: "Sanity".to_string(),
            description: Some("Mental stability and sanity points".to_string()),
            category: "mental".to_string(),
            resource_type: super::runtime_registries::ResourceType::Sanity,
            base_value: 100.0,
            min_value: 0.0,
            max_value: 100.0,
            regen_rate: 0.5,
            regen_type: super::runtime_registries::RegenType::Passive,
            dependencies: vec![],
            tags: vec!["mental".to_string(), "sanity".to_string()],
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_custom_resource(
        id: &str,
        name: &str,
        category: &str,
        resource_type: super::runtime_registries::ResourceType,
        subsystem_id: &str,
    ) -> ResourceDefinition {
        ResourceDefinition {
            id: id.to_string(),
            name: name.to_string(),
            description: None,
            category: category.to_string(),
            resource_type,
            base_value: 100.0,
            min_value: 0.0,
            max_value: 1000.0,
            regen_rate: 1.0,
            regen_type: super::runtime_registries::RegenType::Passive,
            dependencies: vec![],
            tags: vec![],
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

/// Helper trait for creating default category definitions
pub trait CategoryDefinitionBuilder {
    fn create_weapon_category(subsystem_id: &str) -> CategoryDefinition;
    fn create_armor_category(subsystem_id: &str) -> CategoryDefinition;
    fn create_potion_category(subsystem_id: &str) -> CategoryDefinition;
    fn create_combat_category(subsystem_id: &str) -> CategoryDefinition;
    fn create_magic_category(subsystem_id: &str) -> CategoryDefinition;
    fn create_custom_category(
        id: &str,
        name: &str,
        description: Option<&str>,
        subsystem_id: &str,
    ) -> CategoryDefinition;
}

impl CategoryDefinitionBuilder for CategoryDefinition {
    fn create_weapon_category(subsystem_id: &str) -> CategoryDefinition {
        CategoryDefinition {
            id: "weapon".to_string(),
            name: "Weapon".to_string(),
            description: Some("Weapons and combat equipment".to_string()),
            parent_category: Some("equipment".to_string()),
            tags: vec!["weapon".to_string(), "combat".to_string(), "equipment".to_string()],
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_armor_category(subsystem_id: &str) -> CategoryDefinition {
        CategoryDefinition {
            id: "armor".to_string(),
            name: "Armor".to_string(),
            description: Some("Protective armor and clothing".to_string()),
            parent_category: Some("equipment".to_string()),
            tags: vec!["armor".to_string(), "protection".to_string(), "equipment".to_string()],
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_potion_category(subsystem_id: &str) -> CategoryDefinition {
        CategoryDefinition {
            id: "potion".to_string(),
            name: "Potion".to_string(),
            description: Some("Consumable potions and elixirs".to_string()),
            parent_category: Some("consumable".to_string()),
            tags: vec!["potion".to_string(), "consumable".to_string(), "healing".to_string()],
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_combat_category(subsystem_id: &str) -> CategoryDefinition {
        CategoryDefinition {
            id: "combat".to_string(),
            name: "Combat".to_string(),
            description: Some("Combat-related actions and abilities".to_string()),
            parent_category: None,
            tags: vec!["combat".to_string(), "action".to_string()],
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_magic_category(subsystem_id: &str) -> CategoryDefinition {
        CategoryDefinition {
            id: "magic".to_string(),
            name: "Magic".to_string(),
            description: Some("Magic-related actions and abilities".to_string()),
            parent_category: None,
            tags: vec!["magic".to_string(), "spell".to_string(), "action".to_string()],
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_custom_category(
        id: &str,
        name: &str,
        description: Option<&str>,
        subsystem_id: &str,
    ) -> CategoryDefinition {
        CategoryDefinition {
            id: id.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            parent_category: None,
            tags: vec![],
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

/// Helper trait for creating default tag definitions
pub trait TagDefinitionBuilder {
    fn create_vital_tag(subsystem_id: &str) -> TagDefinition;
    fn create_magic_tag(subsystem_id: &str) -> TagDefinition;
    fn create_physical_tag(subsystem_id: &str) -> TagDefinition;
    fn create_mental_tag(subsystem_id: &str) -> TagDefinition;
    fn create_combat_tag(subsystem_id: &str) -> TagDefinition;
    fn create_custom_tag(
        id: &str,
        name: &str,
        tag_type: &str,
        subsystem_id: &str,
    ) -> TagDefinition;
}

impl TagDefinitionBuilder for TagDefinition {
    fn create_vital_tag(subsystem_id: &str) -> TagDefinition {
        TagDefinition {
            id: "vital".to_string(),
            name: "Vital".to_string(),
            description: Some("Vital resources and stats".to_string()),
            tag_type: "resource".to_string(),
            color: Some("#FF0000".to_string()),
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_magic_tag(subsystem_id: &str) -> TagDefinition {
        TagDefinition {
            id: "magic".to_string(),
            name: "Magic".to_string(),
            description: Some("Magic-related content".to_string()),
            tag_type: "category".to_string(),
            color: Some("#8000FF".to_string()),
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_physical_tag(subsystem_id: &str) -> TagDefinition {
        TagDefinition {
            id: "physical".to_string(),
            name: "Physical".to_string(),
            description: Some("Physical-related content".to_string()),
            tag_type: "category".to_string(),
            color: Some("#FF8000".to_string()),
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_mental_tag(subsystem_id: &str) -> TagDefinition {
        TagDefinition {
            id: "mental".to_string(),
            name: "Mental".to_string(),
            description: Some("Mental-related content".to_string()),
            tag_type: "category".to_string(),
            color: Some("#0080FF".to_string()),
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_combat_tag(subsystem_id: &str) -> TagDefinition {
        TagDefinition {
            id: "combat".to_string(),
            name: "Combat".to_string(),
            description: Some("Combat-related content".to_string()),
            tag_type: "action".to_string(),
            color: Some("#FF0000".to_string()),
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_custom_tag(
        id: &str,
        name: &str,
        tag_type: &str,
        subsystem_id: &str,
    ) -> TagDefinition {
        TagDefinition {
            id: id.to_string(),
            name: name.to_string(),
            description: None,
            tag_type: tag_type.to_string(),
            color: None,
            subsystem_id: subsystem_id.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
