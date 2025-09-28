//! # Element Definition
//! 
//! This module defines the core element definition structure.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::ElementConfig;
use crate::unified_registry::ElementCategory;

/// Core element definition
/// 
/// This struct represents a complete element definition with all its properties,
/// derived stats, status effects, and configuration references.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefinition {
    /// Unique identifier
    pub id: String,
    
    /// Display name
    pub name: String,
    
    /// Element description
    pub description: String,
    
    /// Element category
    pub category: ElementCategory,
    
    /// Base properties
    pub base_properties: ElementProperties,
    
    /// Derived stats configuration
    pub derived_stats: Vec<DerivedStatConfig>,
    
    /// Status effects
    pub status_effects: Vec<StatusEffectConfig>,
    
    /// Environment modifications
    pub environment_mods: HashMap<String, EnvironmentMod>,
    
    /// References to other configs
    pub references: ElementReferences,
    
    /// Element aliases for different languages
    pub aliases: ElementAliases,
    
    /// Version of this element definition
    pub version: u32,
    
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Element properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementProperties {
    /// Base damage value
    pub base_damage: f64,
    
    /// Base defense value
    pub base_defense: f64,
    
    /// Base critical hit rate
    pub base_crit_rate: f64,
    
    /// Base critical hit damage multiplier
    pub base_crit_damage: f64,
    
    /// Base accuracy rate
    pub base_accuracy: f64,
    
    /// Base penetration value
    pub base_penetration: f64,
    
    /// Base absorption value
    pub base_absorption: f64,
    
    /// Base amplification value
    pub base_amplification: f64,
    
    /// Base reduction value
    pub base_reduction: f64,
}

/// Derived stat configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedStatConfig {
    /// Stat name
    pub name: String,
    
    /// Calculation formula
    pub formula: String,
    
    /// Base value
    pub base_value: f64,
    
    /// Scaling factor
    pub scaling_factor: f64,
    
    /// Maximum value (if any)
    pub max_value: Option<f64>,
    
    /// Minimum value (if any)
    pub min_value: Option<f64>,
    
    /// Whether this stat is enabled
    pub enabled: bool,
}

impl DerivedStatConfig {
    /// Validate derived stat configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Derived stat name cannot be empty".to_string());
        }
        
        if self.scaling_factor < 0.0 {
            return Err("Scaling factor cannot be negative".to_string());
        }
        
        if let Some(max) = self.max_value {
            if let Some(min) = self.min_value {
                if max < min {
                    return Err("Max value cannot be less than min value".to_string());
                }
            }
        }
        
        Ok(())
    }
}

/// Status effect configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectConfig {
    /// Effect name
    pub name: String,
    
    /// Effect type
    pub effect_type: String,
    
    /// Base probability
    pub base_probability: f64,
    
    /// Base duration
    pub base_duration: f64,
    
    /// Base intensity
    pub base_intensity: f64,
    
    /// Tick interval
    pub tick_interval: f64,
    
    /// Maximum stacks
    pub max_stacks: u32,
    
    /// Whether effect is stackable
    pub stackable: bool,
    
    /// Whether duration refreshes on re-application
    pub refresh_duration: bool,
    
    /// Spread rules (if any)
    pub spread_rules: Option<SpreadRules>,
    
    /// Effect values
    pub effects: Option<HashMap<String, f64>>,
    
    /// HP heal per tick
    pub hp_heal_per_tick: Option<f64>,
    
    /// Stamina heal per tick
    pub stamina_heal_per_tick: Option<f64>,
    
    /// Dynamics configuration
    pub dynamics: StatusDynamics,
}

impl StatusEffectConfig {
    /// Validate status effect configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Status effect name cannot be empty".to_string());
        }
        
        if self.effect_type.is_empty() {
            return Err("Status effect type cannot be empty".to_string());
        }
        
        if self.base_probability < 0.0 || self.base_probability > 1.0 {
            return Err("Base probability must be between 0.0 and 1.0".to_string());
        }
        
        if self.base_duration < 0.0 {
            return Err("Base duration cannot be negative".to_string());
        }
        
        if self.base_intensity < 0.0 {
            return Err("Base intensity cannot be negative".to_string());
        }
        
        if self.tick_interval <= 0.0 {
            return Err("Tick interval must be greater than 0".to_string());
        }
        
        if self.max_stacks == 0 {
            return Err("Max stacks must be greater than 0".to_string());
        }
        
        Ok(())
    }
}

/// Spread rules for status effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadRules {
    /// Whether effect can spread
    pub can_spread: bool,
    
    /// Spread probability
    pub spread_probability: f64,
    
    /// Spread range
    pub spread_range: f64,
    
    /// Maximum spread targets
    pub max_spread_targets: u32,
}

/// Status effect dynamics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDynamics {
    /// Intensity gain rate
    pub intensity_gain: f64,
    
    /// Intensity damping rate
    pub intensity_damping: f64,
    
    /// Decay rate
    pub decay_rate: f64,
    
    /// Refractory gain
    pub refractory_gain: f64,
    
    /// Refractory decay
    pub refractory_decay: f64,
}

/// Environment modification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentMod {
    /// Modification type
    pub mod_type: String,
    
    /// Modification value
    pub value: f64,
    
    /// Duration (if temporary)
    pub duration: Option<f64>,
    
    /// Area of effect
    pub area_of_effect: Option<f64>,
}

impl EnvironmentMod {
    /// Validate environment modification
    pub fn validate(&self) -> Result<(), String> {
        if self.mod_type.is_empty() {
            return Err("Environment mod type cannot be empty".to_string());
        }
        
        if !self.value.is_finite() {
            return Err("Environment mod value must be finite".to_string());
        }
        
        if let Some(duration) = self.duration {
            if duration < 0.0 {
                return Err("Environment mod duration cannot be negative".to_string());
            }
        }
        
        if let Some(aoe) = self.area_of_effect {
            if aoe < 0.0 {
                return Err("Environment mod area of effect cannot be negative".to_string());
            }
        }
        
        Ok(())
    }
}

/// Element references to other configuration files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementReferences {
    /// Probability configuration path
    pub probability_config_path: Option<String>,
    
    /// Interaction configuration path
    pub interaction_config_path: Option<String>,
    
    /// Status pool path
    pub status_pool_path: Option<String>,
    
    /// Golden vectors path
    pub golden_vectors_path: Option<String>,
    
    /// Dynamics design path
    pub dynamics_design: Option<String>,
}

/// Element aliases for different languages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementAliases {
    /// Vietnamese alias
    pub vi: Option<String>,
    
    /// Chinese pinyin alias
    pub zh_pinyin: Option<String>,
    
    /// Japanese alias
    pub ja: Option<String>,
    
    /// Korean alias
    pub ko: Option<String>,
}

impl ElementDefinition {
    /// Create a new element definition
    pub fn new(
        id: String,
        name: String,
        description: String,
        category: ElementCategory,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            name,
            description,
            category,
            base_properties: ElementProperties::default(),
            derived_stats: Vec::new(),
            status_effects: Vec::new(),
            environment_mods: HashMap::new(),
            references: ElementReferences::default(),
            aliases: ElementAliases::default(),
            version: 1,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Update the element definition
    pub fn update(&mut self) {
        self.updated_at = chrono::Utc::now();
        self.version += 1;
    }
    
    /// Add a derived stat configuration
    pub fn add_derived_stat(&mut self, stat_config: DerivedStatConfig) {
        self.derived_stats.push(stat_config);
        self.update();
    }
    
    /// Add a status effect configuration
    pub fn add_status_effect(&mut self, effect_config: StatusEffectConfig) {
        self.status_effects.push(effect_config);
        self.update();
    }
    
    /// Add an environment modification
    pub fn add_environment_mod(&mut self, name: String, env_mod: EnvironmentMod) {
        self.environment_mods.insert(name, env_mod);
        self.update();
    }
    
    /// Get a derived stat configuration by name
    pub fn get_derived_stat(&self, name: &str) -> Option<&DerivedStatConfig> {
        self.derived_stats.iter().find(|stat| stat.name == name)
    }
    
    /// Get a status effect configuration by name
    pub fn get_status_effect(&self, name: &str) -> Option<&StatusEffectConfig> {
        self.status_effects.iter().find(|effect| effect.name == name)
    }
    
    /// Get an environment modification by name
    pub fn get_environment_mod(&self, name: &str) -> Option<&EnvironmentMod> {
        self.environment_mods.get(name)
    }
    
    /// Check if the element has a specific derived stat
    pub fn has_derived_stat(&self, name: &str) -> bool {
        self.derived_stats.iter().any(|stat| stat.name == name)
    }
    
    /// Check if the element has a specific status effect
    pub fn has_status_effect(&self, name: &str) -> bool {
        self.status_effects.iter().any(|effect| effect.name == name)
    }
    
    /// Get all enabled derived stats
    pub fn get_enabled_derived_stats(&self) -> Vec<&DerivedStatConfig> {
        self.derived_stats.iter().filter(|stat| stat.enabled).collect()
    }
    
}

impl ElementProperties {
    /// Validate element properties
    pub fn validate(&self) -> Result<(), String> {
        if self.base_damage < 0.0 {
            return Err("Base damage cannot be negative".to_string());
        }
        
        if self.base_defense < 0.0 {
            return Err("Base defense cannot be negative".to_string());
        }
        
        if self.base_crit_rate < 0.0 || self.base_crit_rate > 1.0 {
            return Err("Base crit rate must be between 0.0 and 1.0".to_string());
        }
        
        if self.base_crit_damage < 1.0 {
            return Err("Base crit damage must be at least 1.0".to_string());
        }
        
        if self.base_accuracy < 0.0 || self.base_accuracy > 1.0 {
            return Err("Base accuracy must be between 0.0 and 1.0".to_string());
        }
        
        if self.base_penetration < 0.0 {
            return Err("Base penetration cannot be negative".to_string());
        }
        
        if self.base_absorption < 0.0 {
            return Err("Base absorption cannot be negative".to_string());
        }
        
        if self.base_amplification < 0.0 {
            return Err("Base amplification cannot be negative".to_string());
        }
        
        if self.base_reduction < 0.0 {
            return Err("Base reduction cannot be negative".to_string());
        }
        
        Ok(())
    }
}

impl Default for ElementProperties {
    fn default() -> Self {
        Self {
            base_damage: 100.0,
            base_defense: 80.0,
            base_crit_rate: 0.15,
            base_crit_damage: 1.5,
            base_accuracy: 0.85,
            base_penetration: 0.0,
            base_absorption: 0.0,
            base_amplification: 0.0,
            base_reduction: 0.0,
        }
    }
}

impl ElementReferences {
    /// Validate element references
    pub fn validate(&self) -> Result<(), String> {
        // No validation needed for optional references
        Ok(())
    }
}

impl Default for ElementReferences {
    fn default() -> Self {
        Self {
            probability_config_path: None,
            interaction_config_path: None,
            status_pool_path: None,
            golden_vectors_path: None,
            dynamics_design: None,
        }
    }
}

impl ElementAliases {
    /// Validate element aliases
    pub fn validate(&self) -> Result<(), String> {
        // No validation needed for optional aliases
        Ok(())
    }
}

impl Default for ElementAliases {
    fn default() -> Self {
        Self {
            vi: None,
            zh_pinyin: None,
            ja: None,
            ko: None,
        }
    }
}

impl Default for StatusDynamics {
    fn default() -> Self {
        Self {
            intensity_gain: 0.02,
            intensity_damping: 0.01,
            decay_rate: 0.05,
            refractory_gain: 0.5,
            refractory_decay: 0.1,
        }
    }
}

impl ElementDefinition {
    /// Validate element definition
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Element ID cannot be empty".to_string());
        }
        
        if self.name.is_empty() {
            return Err("Element name cannot be empty".to_string());
        }
        
        if self.description.is_empty() {
            return Err("Element description cannot be empty".to_string());
        }
        
        if self.version == 0 {
            return Err("Element version must be greater than 0".to_string());
        }
        
        // Validate base properties
        self.base_properties.validate()?;
        
        // Validate derived stats
        for (i, stat) in self.derived_stats.iter().enumerate() {
            stat.validate()
                .map_err(|e| format!("Derived stat {} validation error: {}", i, e))?;
        }
        
        // Validate status effects
        for (i, effect) in self.status_effects.iter().enumerate() {
            effect.validate()
                .map_err(|e| format!("Status effect {} validation error: {}", i, e))?;
        }
        
        // Validate environment mods
        for (key, env_mod) in &self.environment_mods {
            env_mod.validate()
                .map_err(|e| format!("Environment mod '{}' validation error: {}", key, e))?;
        }
        
        // Validate references
        self.references.validate()?;
        
        // Validate aliases
        self.aliases.validate()?;
        
        Ok(())
    }
}
