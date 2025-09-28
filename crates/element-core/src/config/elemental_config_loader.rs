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
use serde_yaml;
use crate::ElementCoreError;
use crate::common_traits::ElementSetter;
use crate::unified_registry::unified_element_registry::UnifiedElementRegistry;
use crate::unified_registry::element_definition as unified_def;
use crate::unified_registry::element_category::ElementCategory;
use crate::unified_registry::element_interaction::{ElementInteraction, InteractionType};

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

        let config: ElementConfig = serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse YAML from {}: {}", file_path.display(), e))?;

        Ok(config)
    }

    /// Populate a unified registry from all YAML element configs and central interactions config
    pub fn populate_unified_registry(&self, unified: &UnifiedElementRegistry) -> Result<(), ElementCoreError> {
        let registry = self.load_all_elements()
            .map_err(|e| ElementCoreError::Config { message: e })?;

        // Deterministic ordering for stable indices
        let mut ids = registry.get_element_ids();
        ids.sort();

        for id in ids {
            if let Some(cfg) = registry.get_element_config(&id) {
                let def = unified_def::ElementDefinition {
                    id: cfg.element.id.clone(),
                    name: cfg.element.name.clone(),
                    description: cfg.element.description.clone(),
                    category: cfg.element.category.parse().unwrap_or(ElementCategory::Special(
                        crate::unified_registry::element_category::SpecialElement::Neutral
                    )),
                    base_properties: unified_def::ElementProperties {
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
                    environment_mods: HashMap::new(),
                    references: unified_def::ElementReferences::default(),
                    aliases: unified_def::ElementAliases {
                        vi: cfg.element.aliases.vi.clone(),
                        zh_pinyin: cfg.element.aliases.zh_pinyin.clone(),
                        ja: None,
                        ko: None,
                    },
                    version: cfg.version,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };
                unified.set_element(&cfg.element.id, def)?;
            }
        }

        // Load central interactions config based on directory structure: ../../configs/interaction_config.yaml
        let base = Path::new(&self.config_dir);
        if let (Some(parent), Some(grand)) = (base.parent(), base.parent().and_then(|p| p.parent())) {
            let interactions_path = grand.join("configs").join("interaction_config.yaml");
            if interactions_path.exists() {
                if let Ok(content) = fs::read_to_string(&interactions_path) {
                    if let Ok(cfg) = serde_yaml::from_str::<crate::config::yaml_loader::InteractionConfig>(&content) {
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
                        }
                    }
                }
            }
        }

        Ok(())
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
