//! # Actor Factory
//! 
//! Factory for creating hierarchical actors with different configurations.

use crate::core::HierarchicalActor;
use element_core::{ElementalSystem, UnifiedElementRegistry as ElementalRegistry, ElementalSystemData, ElementalParams};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Factory for creating hierarchical actors
#[derive(Clone)]
pub struct ActorFactory {
    /// Elemental registry for creating elemental systems
    pub elemental_registry: Arc<ElementalRegistry>,
    
    /// Default configurations for different actor types (commented out for now)
    pub default_configs: HashMap<String, ActorConfig>,
    
    /// Elemental system configurations (commented out for now)
    pub elemental_configs: HashMap<String, ElementalSystemConfig>,
}

impl std::fmt::Debug for ActorFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActorFactory")
            .field("elemental_registry", &"ElementalRegistry")
            .field("default_configs", &self.default_configs)
            .field("elemental_configs", &self.elemental_configs)
            .finish()
    }
}

/// Configuration for creating actors
#[derive(Debug, Clone)]
pub struct ActorConfig {
    /// Default name for this actor type
    pub name: String,
    
    /// Default metadata
    pub metadata: HashMap<String, String>,
    
    /// Default system contributions
    pub default_contributions: Vec<DefaultContribution>,
}

/// Configuration for elemental system
#[derive(Debug, Clone)]
pub struct ElementalSystemConfig {
    /// Initial elemental mastery levels
    pub initial_mastery_levels: HashMap<String, f64>,
    
    /// Initial elemental experience
    pub initial_experience: HashMap<String, f64>,
    
    /// Elemental preferences
    pub elemental_preferences: Vec<String>,
}

/// Default contribution for actor creation
#[derive(Debug, Clone)]
pub struct DefaultContribution {
    /// System name
    pub system_name: String,
    
    /// Stat name
    pub stat_name: String,
    
    /// Default value
    pub value: f64,
    
    /// Priority
    pub priority: u32,
}

impl Default for ActorFactory {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl ActorFactory {
    /// Create a new actor factory with elemental registry
    pub fn new(elemental_registry: Arc<ElementalRegistry>) -> Self {
        let mut factory = Self {
            elemental_registry,
            default_configs: HashMap::new(),
            elemental_configs: HashMap::new(),
        };
        
        // TODO: Remove setup_default_configs when we implement proper configuration loading
        factory.setup_default_configs();
        factory
    }
    
    /// Create a new actor factory with empty elemental registry (for testing)
    pub fn new_empty() -> Self {
        Self::new(Arc::new(ElementalRegistry::new()))
    }
    
    /// Setup default configurations
    fn setup_default_configs(&mut self) {
        // TODO: This is garbage code - should be removed or replaced with proper configuration loading
        // Commenting out for now until we implement proper configuration system
        
        /*
        // Human warrior configuration
        let mut warrior_metadata = HashMap::new();
        warrior_metadata.insert("race".to_string(), "human".to_string());
        warrior_metadata.insert("class".to_string(), "warrior".to_string());
        
        let warrior_config = ActorConfig {
            name: "Human Warrior".to_string(),
            metadata: warrior_metadata,
            default_contributions: vec![
                DefaultContribution {
                    system_name: "race".to_string(),
                    stat_name: "health".to_string(),
                    value: 100.0,
                    priority: 1,
                },
                DefaultContribution {
                    system_name: "class".to_string(),
                    stat_name: "physical_attack".to_string(),
                    value: 50.0,
                    priority: 2,
                },
            ],
        };
        
        self.default_configs.insert("warrior".to_string(), warrior_config);
        
        // Mage configuration
        let mut mage_metadata = HashMap::new();
        mage_metadata.insert("race".to_string(), "elf".to_string());
        mage_metadata.insert("class".to_string(), "mage".to_string());
        
        let mage_config = ActorConfig {
            name: "Elf Mage".to_string(),
            metadata: mage_metadata,
            default_contributions: vec![
                DefaultContribution {
                    system_name: "race".to_string(),
                    stat_name: "mana".to_string(),
                    value: 150.0,
                    priority: 1,
                },
                DefaultContribution {
                    system_name: "class".to_string(),
                    stat_name: "magical_attack".to_string(),
                    value: 80.0,
                    priority: 2,
                },
            ],
        };
        
        self.default_configs.insert("mage".to_string(), mage_config);
        
        // Elemental system configurations
        let fire_elemental_config = ElementalSystemConfig {
            initial_mastery_levels: {
                let mut levels = HashMap::new();
                levels.insert("fire".to_string(), 10.0);
                levels.insert("water".to_string(), 1.0);
                levels.insert("earth".to_string(), 1.0);
                levels.insert("wind".to_string(), 1.0);
                levels
            },
            initial_experience: {
                let mut exp = HashMap::new();
                exp.insert("fire".to_string(), 100.0);
                exp.insert("water".to_string(), 10.0);
                exp.insert("earth".to_string(), 10.0);
                exp.insert("wind".to_string(), 10.0);
                exp
            },
            elemental_preferences: vec!["fire".to_string(), "earth".to_string()],
        };
        
        self.elemental_configs.insert("fire_specialist".to_string(), fire_elemental_config);
        */
    }
    
    /// Create a new actor with all systems initialized
    /// This is the correct way: create actor -> loop and call create for each system
    pub fn create_actor(&self, actor_type: &str) -> Result<HierarchicalActor, String> {
        self.create_actor_with_options(actor_type, None)
    }
    
    /// Create a new actor with custom elemental parameters
    pub fn create_actor_with_elemental(&self, actor_type: &str, elemental_params: ElementalParams) -> Result<HierarchicalActor, String> {
        self.create_actor_with_options(actor_type, Some(elemental_params))
    }
    
    /// Create actor with optional elemental parameters
    fn create_actor_with_options(&self, actor_type: &str, elemental_params: Option<ElementalParams>) -> Result<HierarchicalActor, String> {
        // 1. Create basic actor structure
        let mut actor = HierarchicalActor::with_id_and_name(
            Uuid::new_v4().to_string(),
            format!("{} Actor", actor_type),
        );
        
        // 2. Set basic metadata based on actor type
        match actor_type {
            "warrior" => {
                actor.set_metadata("class".to_string(), "warrior".to_string());
                actor.set_metadata("race".to_string(), "human".to_string());
            },
            "mage" => {
                actor.set_metadata("class".to_string(), "mage".to_string());
                actor.set_metadata("race".to_string(), "elf".to_string());
            },
            _ => {
                // For unknown types, just set the type as metadata
                actor.set_metadata("class".to_string(), actor_type.to_string());
            }
        }
        
        // 3. Initialize all systems by calling their create methods
        self.initialize_elemental_system(&mut actor, elemental_params)?;
        // TODO: Add other systems here as they are implemented
        // self.initialize_cultivation_system(&mut actor)?;
        // self.initialize_magic_system(&mut actor)?;
        // self.initialize_race_system(&mut actor)?;
        // self.initialize_talent_system(&mut actor)?;
        // self.initialize_item_system(&mut actor)?;
        // self.initialize_luck_system(&mut actor)?;
        
        Ok(actor)
    }
    
    /// Initialize elemental system for the actor
    /// All elemental initialization logic is handled by ElementalFactory in element-core
    fn initialize_elemental_system(&self, actor: &mut HierarchicalActor, elemental_params: Option<ElementalParams>) -> Result<(), String> {
        // Use ElementalFactory from element-core to create and configure elemental system
        let elemental_factory = element_core::ElementalFactory::new(self.elemental_registry.clone());
        
        let elemental_system = if let Some(params) = elemental_params {
            // Create elemental system with custom parameters using element-core factory
            elemental_factory.create_elemental_system_with_params(params)
                .map_err(|e| format!("Failed to create elemental system with params: {}", e))?
        } else {
            // Create default elemental system using element-core factory
            elemental_factory.create_elemental_system()
        };
        
        // Replace the elemental system in the actor
        actor.elemental_system = elemental_system;
        
        // Add elemental metadata
        actor.set_metadata("elemental_system_initialized".to_string(), "true".to_string());
        
        Ok(())
    }
    
    /// Add custom actor configuration
    pub fn add_actor_config(&mut self, name: String, config: ActorConfig) {
        self.default_configs.insert(name, config);
    }
    
    /// Add custom elemental configuration
    pub fn add_elemental_config(&mut self, name: String, config: ElementalSystemConfig) {
        self.elemental_configs.insert(name, config);
    }
    
    /// Get available actor types
    pub fn get_available_actor_types(&self) -> Vec<String> {
        self.default_configs.keys().cloned().collect()
    }
    
    /// Get available elemental types
    pub fn get_available_elemental_types(&self) -> Vec<String> {
        self.elemental_configs.keys().cloned().collect()
    }
}

