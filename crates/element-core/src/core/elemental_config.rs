//! # Elemental Configuration Structures
//! 
//! This module contains the elemental configuration structures for loading from YAML files.

use std::collections::HashMap;

/// Element configuration loaded from YAML
#[derive(Debug, Clone)]
pub struct ElementConfig {
    pub version: u32,
    pub element: ElementDefinition,
}

/// Element definition structure
#[derive(Debug, Clone)]
pub struct ElementDefinition {
    pub id: String,
    pub name: String,
    pub aliases: ElementAliases,
    pub category: String,
    pub description: String,
    pub base_properties: BaseProperties,
    pub probability_overrides: HashMap<String, String>, // Simplified for now
    pub derived_stats: Vec<String>,
    pub status_effects: Vec<StatusEffectConfig>,
    pub same_element_effects: Vec<SameElementEffect>,
    pub neutral_effects: Vec<NeutralEffect>,
    pub environment_mods: HashMap<String, EnvironmentMod>,
    pub references: ElementReferences,
}

/// Element aliases for different languages
#[derive(Debug, Clone)]
pub struct ElementAliases {
    pub vi: Option<String>,
    pub zh_pinyin: Option<String>,
}

/// Base properties of an element
#[derive(Debug, Clone)]
pub struct BaseProperties {
    pub base_damage: f64,
    pub base_defense: f64,
    pub base_crit_rate: f64,
    pub base_crit_damage: f64,
    pub base_accuracy: f64,
}

/// Status effect configuration
#[derive(Debug, Clone)]
pub struct StatusEffectConfig {
    pub name: String,
    // #[serde(rename = "type")] // Removed for now
    pub effect_type: String,
    pub base_probability: f64,
    pub base_duration: f64,
    pub base_intensity: f64,
    pub tick_interval: f64,
    pub max_stacks: u32,
    pub stackable: bool,
    pub refresh_duration: bool,
    pub spread_rules: Option<SpreadRules>,
    pub effects: Option<HashMap<String, f64>>,
    pub hp_heal_per_tick: Option<f64>,
    pub stamina_heal_per_tick: Option<f64>,
    pub dynamics: Dynamics,
}

/// Spread rules for status effects
#[derive(Debug, Clone)]
pub struct SpreadRules {
    pub enabled: bool,
    pub spread_chance_base: f64,
    pub spread_range: f64,
    pub spread_max_targets: u32,
}

/// Dynamics configuration for status effects
#[derive(Debug, Clone)]
pub struct Dynamics {
    pub intensity_gain: f64,
    pub intensity_damping: f64,
    pub decay_rate: f64,
    pub refractory_gain: f64,
    pub refractory_decay: f64,
}

/// Same element effects
#[derive(Debug, Clone)]
pub struct SameElementEffect {
    pub pool_id: String,
    pub apply_to: String,
}

/// Neutral effects
#[derive(Debug, Clone)]
pub struct NeutralEffect {
    pub pool_id: String,
    pub apply_to: String,
    pub probability: EffectProbability,
    pub dynamics_override: HashMap<String, String>, // Simplified for now
}

/// Effect probability configuration
#[derive(Debug, Clone)]
pub struct EffectProbability {
    pub base: String,
    pub use_probability_engine: bool,
    pub scaling_factor_key: String,
}

/// Environment modifications
#[derive(Debug, Clone)]
pub struct EnvironmentMod {
    // #[serde(flatten)] // Removed for now
    pub modifiers: HashMap<String, f64>,
}

/// Element references to other config files
#[derive(Debug, Clone)]
pub struct ElementReferences {
    pub probability_config_path: Option<String>,
    pub interaction_config_path: Option<String>,
    pub status_pool_path: Option<String>,
    pub golden_vectors_path: Option<String>,
    pub dynamics_design: Option<String>,
}

/// Element registry for managing all elements
#[derive(Debug, Clone)]
pub struct ElementRegistry {
    pub elements: HashMap<String, ElementConfig>,
    pub element_indices: HashMap<String, usize>,
}

impl ElementRegistry {
    /// Create a new element registry
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            element_indices: HashMap::new(),
        }
    }

    /// Register an element with its index
    pub fn register_element(&mut self, id: String, config: ElementConfig, index: usize) {
        self.element_indices.insert(id.clone(), index);
        self.elements.insert(id, config);
    }

    /// Get element config by ID
    pub fn get_element_config(&self, id: &str) -> Option<&ElementConfig> {
        self.elements.get(id)
    }

    /// Get element index by ID
    pub fn get_element_index(&self, id: &str) -> Option<usize> {
        self.element_indices.get(id).copied()
    }

    /// Get all registered element IDs
    pub fn get_element_ids(&self) -> Vec<String> {
        self.elements.keys().cloned().collect()
    }

    /// Get element count
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }
}

impl Default for ElementRegistry {
    fn default() -> Self {
        Self::new()
    }
}
