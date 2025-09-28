//! # Elemental Factory
//! 
//! This module provides factory functions for creating elemental system instances.

use crate::core::{ElementalSystemData, ElementalSystem, ElementMasteryLevel};
use crate::unified_registry::UnifiedElementRegistry;
use crate::config::elemental_config_loader::ElementConfigLoader;
use crate::{ElementalParams, common_traits::Validatable};
use crate::common_traits::ElementSetter;
use crate::unified_registry::element_interaction::{ElementInteraction, InteractionType};
use std::sync::Arc;

/// Elemental factory for creating elemental system instances
pub struct ElementalFactory {
    registry: Arc<UnifiedElementRegistry>,
}

impl ElementalFactory {
    /// Create a new elemental factory
    pub fn new(registry: Arc<UnifiedElementRegistry>) -> Self {
        Self { registry }
    }

    /// Create elemental factory from config directory
    pub fn from_config_dir(config_dir: String) -> Result<Self, crate::ElementCoreError> {
        let loader = ElementConfigLoader::new(config_dir);
        let element_registry = loader.load_all_elements()
            .map_err(|e| crate::ElementCoreError::Config { message: e })?;

        // Populate unified registry with loaded elements and stable indices
        let unified = Arc::new(UnifiedElementRegistry::new());

        // Sort IDs for deterministic index assignment order
        let mut ids = element_registry.get_element_ids();
        ids.sort();

        for id in ids {
            if let Some(cfg) = element_registry.get_element_config(&id) {
                // Map loader's ElementDefinition to unified_registry::ElementDefinition
                let unified_def = crate::unified_registry::element_definition::ElementDefinition {
                    id: cfg.element.id.clone(),
                    name: cfg.element.name.clone(),
                    description: cfg.element.description.clone(),
                    // Map category string to ElementCategory via FromStr; fallback to Special::Neutral
                    category: cfg.element.category.parse().unwrap_or(
                        crate::unified_registry::element_category::ElementCategory::Special(
                            crate::unified_registry::element_category::SpecialElement::Neutral
                        )
                    ),
                    base_properties: crate::unified_registry::element_definition::ElementProperties {
                        base_damage: cfg.element.base_properties.base_damage,
                        base_defense: cfg.element.base_properties.base_defense,
                        base_crit_rate: cfg.element.base_properties.base_crit_rate,
                        base_crit_damage: cfg.element.base_properties.base_crit_damage,
                        base_accuracy: cfg.element.base_properties.base_accuracy,
                        base_penetration: 0.0,
                        base_absorption: 0.0,
                        base_amplification: 0.0,
                        base_reduction: 0.0,
                    },
                    derived_stats: Vec::new(),
                    status_effects: Vec::new(),
                    environment_mods: std::collections::HashMap::new(),
                    references: crate::unified_registry::element_definition::ElementReferences::default(),
                    aliases: crate::unified_registry::element_definition::ElementAliases {
                        vi: cfg.element.aliases.vi.clone(),
                        zh_pinyin: cfg.element.aliases.zh_pinyin.clone(),
                        ja: None,
                        ko: None,
                    },
                    version: cfg.version,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };

                // Register and let registry assign stable index (sync path)
                unified.set_element(&cfg.element.id, unified_def)
                    .map_err(|e| crate::ElementCoreError::Registry { message: e.to_string() })?;
                // Also ensure element_indices is filled by the setter/register path (already handled)
            }
        }

        // Optionally load interactions from central config if available under docs path
        // Using YamlConfigLoader if caller points to that directory structure
        let interactions_dir = std::path::Path::new("docs/element-core/configs");
        if interactions_dir.exists() {
            if let Ok(content) = std::fs::read_to_string(interactions_dir.join("interaction_config.yaml")) {
                if let Ok(cfg) = serde_yaml::from_str::<crate::config::yaml_loader::InteractionConfig>(&content) {
                    // Build ElementInteraction entries based on cfg.pairs and relationships
                    for (src, pair) in cfg.pairs.iter() {
                        for tgt in &pair.generating {
                            let _ = unified.set_interaction_sync(ElementInteraction::new(
                                format!("{}_generating_{}", src, tgt),
                                src.clone(),
                                tgt.clone(),
                                InteractionType::Generating,
                            ));
                        }
                        for tgt in &pair.overcoming {
                            let _ = unified.set_interaction_sync(ElementInteraction::new(
                                format!("{}_overcoming_{}", src, tgt),
                                src.clone(),
                                tgt.clone(),
                                InteractionType::Overcoming,
                            ));
                        }
                        for tgt in &pair.neutral {
                            let _ = unified.set_interaction_sync(ElementInteraction::new(
                                format!("{}_neutral_{}", src, tgt),
                                src.clone(),
                                tgt.clone(),
                                InteractionType::Neutral,
                            ));
                        }
                        // same relation entries can be inferred dynamically (src==tgt)
                    }
                }
            }
        }

        Ok(Self::new(unified))
    }

    /// Create a new elemental system with default data
    pub fn create_elemental_system(&self) -> ElementalSystem {
        ElementalSystem::new()
    }

    /// Create elemental system from existing data
    pub fn create_elemental_system_from_data(&self, data: ElementalSystemData) -> ElementalSystem {
        ElementalSystem::from_data(data)
    }

    /// Create elemental system with specific element configurations
    pub fn create_elemental_system_with_configs(&self, element_ids: &[String]) -> Result<ElementalSystem, crate::ElementCoreError> {
        let mut system = ElementalSystem::new();
        let mut data = system.get_data_mut().clone();

        // Initialize specific elements
        for element_id in element_ids {
            if let Some(index) = self.registry.get_element_index(element_id)? {
                // Initialize element with default values
                self.initialize_element_data(&mut data, index, element_id)?;
            } else {
                return Err(crate::ElementCoreError::ElementNotFound { element_id: element_id.clone() });
            }
        }

        system.set_data(data);
        Ok(system)
    }

    /// Create elemental system for all registered elements
    pub fn create_elemental_system_all(&self) -> Result<ElementalSystem, crate::ElementCoreError> {
        let element_ids = self.registry.get_element_ids()?;
        self.create_elemental_system_with_configs(&element_ids)
    }

    /// Create elemental system with custom parameters
    pub fn create_elemental_system_with_params(&self, params: ElementalParams) -> Result<ElementalSystem, crate::ElementCoreError> {
        // Validate that primary element exists in registry
        let _config = self.registry.get_element_config(&params.primary_element)?;

        // Create elemental system using builder pattern
        let mut builder = self.create_builder();
        
        // Initialize primary element first
        builder = builder.with_element(&params.primary_element)?;
        
        // Initialize other elements from mastery levels
        for element in params.initial_mastery_levels.keys() {
            if element != &params.primary_element {
                builder = builder.with_element(element)?;
            }
        }
        
        // Set custom mastery levels
        for (element, level) in &params.initial_mastery_levels {
            builder = builder.with_mastery_level(element, *level)?;
        }
        
        // Set custom qi amounts
        for (element, qi_amount) in &params.initial_qi_amounts {
            builder = builder.with_qi_amount(element, *qi_amount)?;
        }
        
        // Build the system
        let mut system = builder.build();
        
        // Set experience values (not supported by builder yet, so set directly)
        let mut data = system.get_data_mut();
        for (element, experience) in &params.initial_experience {
            if let Some(index) = self.registry.get_element_index(element)? {
                data.element_mastery_experience[index] = *experience;
            }
        }
        
        Ok(system)
    }

    /// Initialize element data with default values (CORRECTED VERSION)
    fn initialize_element_data(&self, data: &mut ElementalSystemData, index: usize, element_id: &str) -> Result<(), crate::ElementCoreError> {
        // Get element config
        let config = self.registry.get_element_config(element_id)?;

        // Initialize PRIMARY STATS only
        data.element_mastery_levels[index] = 1.0;
        data.element_mastery_experience[index] = 0.0;
        data.element_mastery_level_enums[index] = ElementMasteryLevel::Beginner;
        data.element_qi_amounts[index] = 100.0;
        data.element_qi_capacities[index] = 1000.0;
        data.element_qi_regeneration_rates[index] = 10.0;

        // Calculate DERIVED STATS from primary stats and base properties
        data.calculate_derived_stats(
            index,
            config.base_properties.base_damage,
            config.base_properties.base_defense,
            config.base_properties.base_crit_rate,
            config.base_properties.base_crit_damage,
            config.base_properties.base_accuracy,
        )?;

        Ok(())
    }

    /// Get registry reference
    pub fn get_registry(&self) -> Arc<UnifiedElementRegistry> {
        self.registry.clone()
    }

    /// Create elemental system builder
    pub fn create_builder(&self) -> ElementalSystemBuilder {
        ElementalSystemBuilder::new(self.registry.clone())
    }
}

/// Elemental system builder for step-by-step construction
pub struct ElementalSystemBuilder {
    registry: Arc<UnifiedElementRegistry>,
    data: ElementalSystemData,
    initialized_elements: Vec<String>,
}

impl ElementalSystemBuilder {
    /// Create a new builder
    pub fn new(registry: Arc<UnifiedElementRegistry>) -> Self {
        Self {
            registry,
            data: ElementalSystemData::new(),
            initialized_elements: Vec::new(),
        }
    }

    /// Add element to the system
    pub fn with_element(mut self, element_id: &str) -> Result<Self, crate::ElementCoreError> {
        if let Some(index) = self.registry.get_element_index(element_id)? {
            self.initialize_element_data(index, element_id)?;
            self.initialized_elements.push(element_id.to_string());
            Ok(self)
        } else {
            Err(crate::ElementCoreError::ElementNotFound { element_id: element_id.to_string() })
        }
    }

    /// Add multiple elements
    pub fn with_elements(mut self, element_ids: &[String]) -> Result<Self, crate::ElementCoreError> {
        for element_id in element_ids {
            self = self.with_element(element_id)?;
        }
        Ok(self)
    }

    /// Set element mastery level
    pub fn with_mastery_level(mut self, element_id: &str, level: f64) -> Result<Self, crate::ElementCoreError> {
        if let Some(index) = self.registry.get_element_index(element_id)? {
            self.data.element_mastery_levels[index] = level;
            Ok(self)
        } else {
            Err(crate::ElementCoreError::ElementNotFound { element_id: element_id.to_string() })
        }
    }

    /// Set element qi amount
    pub fn with_qi_amount(mut self, element_id: &str, amount: f64) -> Result<Self, crate::ElementCoreError> {
        if let Some(index) = self.registry.get_element_index(element_id)? {
            self.data.element_qi_amounts[index] = amount;
            Ok(self)
        } else {
            Err(crate::ElementCoreError::ElementNotFound { element_id: element_id.to_string() })
        }
    }

    /// Build the elemental system
    pub fn build(self) -> ElementalSystem {
        ElementalSystem::from_data(self.data)
    }

    /// Initialize element data with default values (CORRECTED VERSION)
    fn initialize_element_data(&mut self, index: usize, element_id: &str) -> Result<(), crate::ElementCoreError> {
        // Get element config
        let config = self.registry.get_element_config(element_id)?;

        // Initialize PRIMARY STATS only
        self.data.element_mastery_levels[index] = 1.0;
        self.data.element_mastery_experience[index] = 0.0;
        self.data.element_mastery_level_enums[index] = ElementMasteryLevel::Beginner;
        self.data.element_qi_amounts[index] = 100.0;
        self.data.element_qi_capacities[index] = 1000.0;
        self.data.element_qi_regeneration_rates[index] = 10.0;

        // Calculate DERIVED STATS from primary stats and base properties
        self.data.calculate_derived_stats(
            index,
            config.base_properties.base_damage,
            config.base_properties.base_defense,
            config.base_properties.base_crit_rate,
            config.base_properties.base_crit_damage,
            config.base_properties.base_accuracy,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common_traits::ElementSetter;

    fn create_test_registry() -> UnifiedElementRegistry {
        let registry = UnifiedElementRegistry::new();

        // Minimal unified element definition for tests
        let unified_def = crate::unified_registry::element_definition::ElementDefinition {
            id: "fire".to_string(),
            name: "Fire".to_string(),
            description: "Fire element".to_string(),
            category: "five_elements".parse().unwrap_or(
                crate::unified_registry::element_category::ElementCategory::Special(
                    crate::unified_registry::element_category::SpecialElement::Neutral
                )
            ),
            base_properties: crate::unified_registry::element_definition::ElementProperties::default(),
            derived_stats: Vec::new(),
            status_effects: Vec::new(),
            environment_mods: std::collections::HashMap::new(),
            references: crate::unified_registry::element_definition::ElementReferences::default(),
            aliases: crate::unified_registry::element_definition::ElementAliases::default(),
            version: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Set element synchronously (assigns stable index internally)
        registry.set_element("fire", unified_def).unwrap();
        registry
    }

    #[test]
    fn test_factory_creation() {
        let registry = Arc::new(create_test_registry());
        let factory = ElementalFactory::new(registry);
        let system = factory.create_elemental_system();
        assert!(system.get_data().element_mastery_levels[0] == 0.0);
    }

    #[test]
    fn test_create_system_with_configs() {
        let registry = Arc::new(create_test_registry());
        let factory = ElementalFactory::new(registry);
        let system = factory.create_elemental_system_with_configs(&["fire".to_string()]).unwrap();
        assert!(system.get_data().element_mastery_levels[0] == 1.0);
    }

    #[test]
    fn test_builder_pattern() {
        let registry = Arc::new(create_test_registry());
        let factory = ElementalFactory::new(registry);
        let builder = factory.create_builder();
        
        let system = builder
            .with_element("fire").unwrap()
            .with_mastery_level("fire", 5.0).unwrap()
            .with_qi_amount("fire", 500.0).unwrap()
            .build();
            
        assert_eq!(system.get_data().element_mastery_levels[0], 5.0);
        assert_eq!(system.get_data().element_qi_amounts[0], 500.0);
    }

    #[test]
    fn test_create_system_with_params() {
        let registry = Arc::new(create_test_registry());
        let factory = ElementalFactory::new(registry);
        
        let mut mastery_levels = std::collections::HashMap::new();
        mastery_levels.insert("fire".to_string(), 10.0);
        
        let mut experience = std::collections::HashMap::new();
        experience.insert("fire".to_string(), 100.0);
        
        let mut qi_amounts = std::collections::HashMap::new();
        qi_amounts.insert("fire".to_string(), 500.0);
        
        let params = ElementalParams {
            primary_element: "fire".to_string(),
            initial_mastery_levels: mastery_levels,
            initial_experience: experience,
            initial_qi_amounts: qi_amounts,
            elemental_preferences: vec!["fire".to_string()],
        };
        
        let system = factory.create_elemental_system_with_params(params).unwrap();
        assert_eq!(system.get_data().element_mastery_levels[0], 10.0);
        assert_eq!(system.get_data().element_mastery_experience[0], 100.0);
        assert_eq!(system.get_data().element_qi_amounts[0], 500.0);
    }
}

impl Validatable for ElementalFactory {
    fn validate(&self) -> crate::ElementCoreResult<()> {
        // Validate the registry
        self.registry.validate()?;
        
        Ok(())
    }
    
    fn get_validation_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Check registry validation
        if let Err(e) = self.registry.validate() {
            errors.push(format!("Registry validation failed: {}", e));
        }
        
        errors
    }
}

impl Validatable for ElementalSystemBuilder {
    fn validate(&self) -> crate::ElementCoreResult<()> {
        // Validate the data
        self.data.validate()?;
        
        // Validate the registry
        self.registry.validate()?;
        
        Ok(())
    }
    
    fn get_validation_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Check data validation
        if let Err(e) = self.data.validate() {
            errors.push(format!("Elemental system data validation failed: {}", e));
        }
        
        // Check registry validation
        if let Err(e) = self.registry.validate() {
            errors.push(format!("Registry validation failed: {}", e));
        }
        
        errors
    }
}
