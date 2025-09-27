//! # Elemental Registry
//! 
//! This module provides the elemental registry for managing element configurations and indices.

use crate::core::elemental_config::{ElementConfig, ElementRegistry};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Thread-safe elemental registry
pub struct ElementalRegistry {
    registry: Arc<RwLock<ElementRegistry>>,
}

impl ElementalRegistry {
    /// Create a new elemental registry
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(ElementRegistry::new())),
        }
    }

    /// Create registry from existing registry
    pub fn from_registry(registry: ElementRegistry) -> Self {
        Self {
            registry: Arc::new(RwLock::new(registry)),
        }
    }

    /// Register an element with its configuration and index
    pub fn register_element(&self, id: String, config: ElementConfig, index: usize) -> Result<(), String> {
        let mut registry = self.registry.write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        
        registry.register_element(id, config, index);
        Ok(())
    }

    /// Get element configuration by ID
    pub fn get_element_config(&self, id: &str) -> Result<Option<ElementConfig>, String> {
        let registry = self.registry.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        
        Ok(registry.get_element_config(id).cloned())
    }

    /// Get element index by ID
    pub fn get_element_index(&self, id: &str) -> Result<Option<usize>, String> {
        let registry = self.registry.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        
        Ok(registry.get_element_index(id))
    }

    /// Get element ID by index
    pub fn get_element_id_by_index(&self, index: usize) -> Result<Option<String>, String> {
        let registry = self.registry.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        
        // Find element ID by index
        for (id, &element_index) in registry.element_indices.iter() {
            if element_index == index {
                return Ok(Some(id.clone()));
            }
        }
        Ok(None)
    }

    /// Get all registered element IDs
    pub fn get_element_ids(&self) -> Result<Vec<String>, String> {
        let registry = self.registry.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        
        Ok(registry.get_element_ids())
    }

    /// Get element count
    pub fn element_count(&self) -> Result<usize, String> {
        let registry = self.registry.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        
        Ok(registry.element_count())
    }

    /// Check if element is registered
    pub fn is_element_registered(&self, id: &str) -> Result<bool, String> {
        let registry = self.registry.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        
        Ok(registry.get_element_config(id).is_some())
    }

    /// Get all element configurations
    pub fn get_all_configs(&self) -> Result<HashMap<String, ElementConfig>, String> {
        let registry = self.registry.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        
        Ok(registry.elements.clone())
    }

    /// Clear all registered elements
    pub fn clear(&self) -> Result<(), String> {
        let mut registry = self.registry.write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        
        registry.elements.clear();
        registry.element_indices.clear();
        Ok(())
    }

    /// Get element statistics
    pub fn get_statistics(&self) -> Result<ElementStatistics, String> {
        let registry = self.registry.read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
        
        let mut categories = HashMap::new();
        for config in registry.elements.values() {
            let category = &config.element.category;
            *categories.entry(category.clone()).or_insert(0) += 1;
        }

        Ok(ElementStatistics {
            total_elements: registry.element_count(),
            categories,
        })
    }
}

/// Element statistics
#[derive(Debug, Clone)]
pub struct ElementStatistics {
    pub total_elements: usize,
    pub categories: HashMap<String, usize>,
}

impl Default for ElementalRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Element index constants for compile-time optimization
pub mod element_indices {
    // Core elements
    pub const FIRE: usize = 0;
    pub const WATER: usize = 1;
    pub const EARTH: usize = 2;
    pub const WOOD: usize = 3;
    pub const METAL: usize = 4;
    
    // Extended elements
    pub const ICE: usize = 5;
    pub const LIGHTNING: usize = 6;
    pub const WIND: usize = 7;
    
    // Special elements (can be extended)
    pub const LIGHT: usize = 8;
    pub const DARK: usize = 9;
    pub const VOID: usize = 10;
    
    // Maximum supported elements
    pub const MAX_ELEMENTS: usize = 50;
}

/// Element ID constants
pub mod element_ids {
    pub const FIRE: &str = "fire";
    pub const WATER: &str = "water";
    pub const EARTH: &str = "earth";
    pub const WOOD: &str = "wood";
    pub const METAL: &str = "metal";
    pub const ICE: &str = "ice";
    pub const LIGHTNING: &str = "lightning";
    pub const WIND: &str = "wind";
    pub const LIGHT: &str = "light";
    pub const DARK: &str = "dark";
    pub const VOID: &str = "void";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::elemental_config::{ElementDefinition, ElementAliases, BaseProperties};

    fn create_test_element_config(id: &str) -> ElementConfig {
        ElementConfig {
            version: 1,
            element: ElementDefinition {
                id: id.to_string(),
                name: id.to_uppercase(),
                aliases: ElementAliases {
                    vi: Some(format!("{}_vi", id)),
                    zh_pinyin: Some(format!("{}_zh", id)),
                },
                category: "test".to_string(),
                description: format!("Test element: {}", id),
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
        }
    }

    #[test]
    fn test_registry_creation() {
        let registry = ElementalRegistry::new();
        assert_eq!(registry.element_count().unwrap(), 0);
    }

    #[test]
    fn test_register_element() {
        let registry = ElementalRegistry::new();
        let config = create_test_element_config("fire");
        
        registry.register_element("fire".to_string(), config, 0).unwrap();
        assert_eq!(registry.element_count().unwrap(), 1);
        assert!(registry.is_element_registered("fire").unwrap());
    }

    #[test]
    fn test_get_element_config() {
        let registry = ElementalRegistry::new();
        let config = create_test_element_config("fire");
        
        registry.register_element("fire".to_string(), config.clone(), 0).unwrap();
        
        let retrieved_config = registry.get_element_config("fire").unwrap().unwrap();
        assert_eq!(retrieved_config.element.id, "fire");
    }

    #[test]
    fn test_get_element_index() {
        let registry = ElementalRegistry::new();
        let config = create_test_element_config("fire");
        
        registry.register_element("fire".to_string(), config, 0).unwrap();
        
        assert_eq!(registry.get_element_index("fire").unwrap(), Some(0));
        assert_eq!(registry.get_element_id_by_index(0).unwrap(), Some("fire".to_string()));
    }
}
