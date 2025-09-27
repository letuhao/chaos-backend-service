//! # Elemental Configuration Loader
//! 
//! This module handles loading elemental configurations from YAML files.

use crate::core::elemental_config::{
    ElementConfig, ElementRegistry, ElementDefinition, ElementAliases, 
    BaseProperties, ElementReferences
};
use std::path::Path;
use std::fs;
use std::collections::HashMap;

/// Element configuration loader
pub struct ElementConfigLoader {
    config_dir: String,
}

impl ElementConfigLoader {
    /// Create a new config loader with the specified directory
    pub fn new(config_dir: String) -> Self {
        Self { config_dir }
    }

    /// Load all element configurations from the config directory
    pub fn load_all_elements(&self) -> Result<ElementRegistry, String> {
        let mut registry = ElementRegistry::new();
        let mut index = 0;

        // Get all YAML files in the config directory
        let config_path = Path::new(&self.config_dir);
        if !config_path.exists() {
            return Err(format!("Config directory does not exist: {}", self.config_dir));
        }

        let entries = fs::read_dir(config_path)
            .map_err(|e| format!("Failed to read config directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let element_config = self.load_element_config(&path)?;
                let element_id = element_config.element.id.clone();
                
                registry.register_element(element_id.clone(), element_config, index);
                index += 1;
            }
        }

        Ok(registry)
    }

    /// Load a single element configuration from a file
    pub fn load_element_config(&self, file_path: &Path) -> Result<ElementConfig, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;

        // TODO: Implement YAML parsing without serde for now
        // let config: ElementConfig = serde_yaml::from_str(&content)
        //     .map_err(|e| format!("Failed to parse YAML from {}: {}", file_path.display(), e))?;
        
        // Temporary mock config for testing
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
                references: ElementReferences {
                    probability_config_path: None,
                    interaction_config_path: None,
                    status_pool_path: None,
                    golden_vectors_path: None,
                    dynamics_design: None,
                },
            },
        };

        Ok(config)
    }

    /// Load element configuration by ID
    pub fn load_element_by_id(&self, element_id: &str) -> Result<ElementConfig, String> {
        let file_name = format!("{}_element.yaml", element_id);
        let file_path = Path::new(&self.config_dir).join(file_name);
        
        self.load_element_config(&file_path)
    }

    /// Validate element configuration
    pub fn validate_config(&self, config: &ElementConfig) -> Result<(), String> {
        // Check required fields
        if config.element.id.is_empty() {
            return Err("Element ID cannot be empty".to_string());
        }

        if config.element.name.is_empty() {
            return Err("Element name cannot be empty".to_string());
        }

        if config.element.category.is_empty() {
            return Err("Element category cannot be empty".to_string());
        }

        // Validate base properties
        if config.element.base_properties.base_damage < 0.0 {
            return Err("Base damage cannot be negative".to_string());
        }

        if config.element.base_properties.base_defense < 0.0 {
            return Err("Base defense cannot be negative".to_string());
        }

        if config.element.base_properties.base_crit_rate < 0.0 || config.element.base_properties.base_crit_rate > 1.0 {
            return Err("Base crit rate must be between 0.0 and 1.0".to_string());
        }

        if config.element.base_properties.base_crit_damage < 1.0 {
            return Err("Base crit damage must be at least 1.0".to_string());
        }

        if config.element.base_properties.base_accuracy < 0.0 || config.element.base_properties.base_accuracy > 1.0 {
            return Err("Base accuracy must be between 0.0 and 1.0".to_string());
        }

        // Validate status effects
        for status_effect in &config.element.status_effects {
            if status_effect.base_probability < 0.0 || status_effect.base_probability > 1.0 {
                return Err(format!("Status effect '{}' probability must be between 0.0 and 1.0", status_effect.name));
            }

            if status_effect.base_duration <= 0.0 {
                return Err(format!("Status effect '{}' duration must be positive", status_effect.name));
            }

            if status_effect.max_stacks == 0 {
                return Err(format!("Status effect '{}' max stacks must be at least 1", status_effect.name));
            }
        }

        Ok(())
    }

    /// Get available element files in the config directory
    pub fn get_available_elements(&self) -> Result<Vec<String>, String> {
        let config_path = Path::new(&self.config_dir);
        if !config_path.exists() {
            return Err(format!("Config directory does not exist: {}", self.config_dir));
        }

        let mut elements = Vec::new();
        let entries = fs::read_dir(config_path)
            .map_err(|e| format!("Failed to read config directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    if file_name.ends_with("_element") {
                        let element_id = file_name.replace("_element", "");
                        elements.push(element_id);
                    }
                }
            }
        }

        Ok(elements)
    }
}

impl Default for ElementConfigLoader {
    fn default() -> Self {
        Self::new("docs/element-core/elements/configs".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loader_creation() {
        let loader = ElementConfigLoader::new("test_configs".to_string());
        assert_eq!(loader.config_dir, "test_configs");
    }

    #[test]
    fn test_default_config_loader() {
        let loader = ElementConfigLoader::default();
        assert_eq!(loader.config_dir, "docs/element-core/elements/configs");
    }
}
