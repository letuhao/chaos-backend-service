//! # Elemental Factory
//! 
//! This module provides factory functions for creating elemental system instances.

use crate::core::{ElementalSystemData, ElementalSystem, ElementMasteryRank};
use crate::registry::ElementalRegistry;
use crate::config::elemental_config_loader::ElementConfigLoader;
use crate::ElementalParams;
use std::sync::Arc;

/// Elemental factory for creating elemental system instances
pub struct ElementalFactory {
    registry: Arc<ElementalRegistry>,
}

impl ElementalFactory {
    /// Create a new elemental factory
    pub fn new(registry: Arc<ElementalRegistry>) -> Self {
        Self { registry }
    }

    /// Create elemental factory from config directory
    pub fn from_config_dir(config_dir: String) -> Result<Self, crate::ElementCoreError> {
        let loader = ElementConfigLoader::new(config_dir);
        let registry = loader.load_all_elements()?;
        let registry = Arc::new(ElementalRegistry::from_registry(registry));
        
        Ok(Self::new(registry))
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
        let _config = self.registry.get_element_config(&params.primary_element)?
            .ok_or_else(|| crate::ElementCoreError::ElementNotFound { element_id: params.primary_element.clone() })?;

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
        let config = self.registry.get_element_config(element_id)?
            .ok_or_else(|| crate::ElementCoreError::ElementNotFound { element_id: element_id.to_string() })?;

        // Initialize PRIMARY STATS only
        data.element_mastery_levels[index] = 1.0;
        data.element_mastery_experience[index] = 0.0;
        data.element_mastery_ranks[index] = ElementMasteryRank::Novice;
        data.element_qi_amounts[index] = 100.0;
        data.element_qi_capacities[index] = 1000.0;
        data.element_qi_regeneration_rates[index] = 10.0;

        // Calculate DERIVED STATS from primary stats and base properties
        data.calculate_derived_stats(
            index,
            config.element.base_properties.base_damage,
            config.element.base_properties.base_defense,
            config.element.base_properties.base_crit_rate,
            config.element.base_properties.base_crit_damage,
            config.element.base_properties.base_accuracy,
        )?;

        Ok(())
    }

    /// Get registry reference
    pub fn get_registry(&self) -> Arc<ElementalRegistry> {
        self.registry.clone()
    }

    /// Create elemental system builder
    pub fn create_builder(&self) -> ElementalSystemBuilder {
        ElementalSystemBuilder::new(self.registry.clone())
    }
}

/// Elemental system builder for step-by-step construction
pub struct ElementalSystemBuilder {
    registry: Arc<ElementalRegistry>,
    data: ElementalSystemData,
    initialized_elements: Vec<String>,
}

impl ElementalSystemBuilder {
    /// Create a new builder
    pub fn new(registry: Arc<ElementalRegistry>) -> Self {
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
        let config = self.registry.get_element_config(element_id)?
            .ok_or_else(|| crate::ElementCoreError::ElementNotFound { element_id: element_id.to_string() })?;

        // Initialize PRIMARY STATS only
        self.data.element_mastery_levels[index] = 1.0;
        self.data.element_mastery_experience[index] = 0.0;
        self.data.element_mastery_ranks[index] = ElementMasteryRank::Novice;
        self.data.element_qi_amounts[index] = 100.0;
        self.data.element_qi_capacities[index] = 1000.0;
        self.data.element_qi_regeneration_rates[index] = 10.0;

        // Calculate DERIVED STATS from primary stats and base properties
        self.data.calculate_derived_stats(
            index,
            config.element.base_properties.base_damage,
            config.element.base_properties.base_defense,
            config.element.base_properties.base_crit_rate,
            config.element.base_properties.base_crit_damage,
            config.element.base_properties.base_accuracy,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::elemental_config::{ElementConfig, ElementDefinition, ElementAliases, BaseProperties};
    use std::collections::HashMap;

    fn create_test_registry() -> ElementalRegistry {
        let mut registry = ElementalRegistry::new();
        
        let config = ElementConfig {
            version: 1,
            element: ElementDefinition {
                id: "fire".to_string(),
                name: "Fire".to_string(),
                aliases: ElementAliases {
                    vi: Some("hỏa".to_string()),
                    zh_pinyin: Some("huo".to_string()),
                },
                category: "five_elements".to_string(),
                description: "Fire element".to_string(),
                base_properties: BaseProperties {
                    base_damage: 100.0,
                    base_defense: 80.0,
                    base_crit_rate: 0.15,
                    base_crit_damage: 1.5,
                    base_accuracy: 0.85,
                },
                probability_overrides: HashMap::new(),
                derived_stats: vec!["element_mastery".to_string()],
                status_effects: vec![],
                same_element_effects: vec![],
                neutral_effects: vec![],
                environment_mods: HashMap::new(),
                references: crate::core::elemental_config::ElementReferences {
                    probability_config_path: None,
                    interaction_config_path: None,
                    status_pool_path: None,
                    golden_vectors_path: None,
                    dynamics_design: None,
                },
            },
        };
        
        registry.register_element("fire".to_string(), config, 0).unwrap();
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
