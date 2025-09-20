//! Exhaustion Configuration Loader
//!
//! This module provides configuration loading and deep merging capabilities
//! for the Resource Exhaustion System, supporting global, area, and PvP overrides.

use std::collections::HashMap;
use std::path::Path;
// Removed unused imports

#[allow(unused_imports)]
use super::resource_exhaustion::{
    ExhaustionConfig, ArchetypeConfig, ResourceConfig, ThresholdConfig, EffectConfig, EventConfig
};

/// Configuration loader for exhaustion system
pub struct ExhaustionConfigLoader {
    /// Global configuration
    global_config: Option<ExhaustionConfig>,
    /// Area-specific overrides
    area_overrides: HashMap<String, ExhaustionConfig>,
    /// PvP template overrides
    pvp_overrides: HashMap<String, ExhaustionConfig>,
}

/// Configuration source for debugging
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigSource {
    Global,
    Area(String),
    PvP(String),
}

/// Configuration merge result with source tracking
#[derive(Debug, Clone)]
pub struct MergedConfig {
    /// Final merged configuration
    pub config: ExhaustionConfig,
    /// Source tracking for each configuration key
    pub sources: HashMap<String, ConfigSource>,
}

/// Configuration loading error
#[derive(Debug, thiserror::Error)]
pub enum ConfigLoaderError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Invalid YAML: {0}")]
    InvalidYaml(String),
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Merge error: {0}")]
    MergeError(String),
}

impl ExhaustionConfigLoader {
    /// Create a new configuration loader
    pub fn new() -> Self {
        Self {
            global_config: None,
            area_overrides: HashMap::new(),
            pvp_overrides: HashMap::new(),
        }
    }

    /// Load global configuration from file
    pub async fn load_global_config<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ConfigLoaderError> {
        let path = path.as_ref();
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|_| ConfigLoaderError::FileNotFound(path.to_string_lossy().to_string()))?;

        let config: ExhaustionConfig = serde_yaml::from_str(&content)
            .map_err(|e| ConfigLoaderError::InvalidYaml(e.to_string()))?;

        self.validate_config(&config)?;
        self.global_config = Some(config);
        Ok(())
    }

    /// Load area-specific configuration override
    pub async fn load_area_override<P: AsRef<Path>>(&mut self, area_id: &str, path: P) -> Result<(), ConfigLoaderError> {
        let path = path.as_ref();
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|_| ConfigLoaderError::FileNotFound(path.to_string_lossy().to_string()))?;

        let config: ExhaustionConfig = serde_yaml::from_str(&content)
            .map_err(|e| ConfigLoaderError::InvalidYaml(e.to_string()))?;

        self.validate_config(&config)?;
        self.area_overrides.insert(area_id.to_string(), config);
        Ok(())
    }

    /// Load PvP template configuration override
    pub async fn load_pvp_override<P: AsRef<Path>>(&mut self, template_id: &str, path: P) -> Result<(), ConfigLoaderError> {
        let path = path.as_ref();
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|_| ConfigLoaderError::FileNotFound(path.to_string_lossy().to_string()))?;

        let config: ExhaustionConfig = serde_yaml::from_str(&content)
            .map_err(|e| ConfigLoaderError::InvalidYaml(e.to_string()))?;

        self.validate_config(&config)?;
        self.pvp_overrides.insert(template_id.to_string(), config);
        Ok(())
    }

    /// Resolve final configuration for an actor
    pub fn resolve_config(&self, area_id: Option<&str>, pvp_template: Option<&str>) -> Result<MergedConfig, ConfigLoaderError> {
        let global_config = self.global_config.as_ref()
            .ok_or_else(|| ConfigLoaderError::ValidationError("No global configuration loaded".to_string()))?;

        let mut merged_config = global_config.clone();
        let mut sources = HashMap::new();

        // Track global config sources
        self.track_config_sources(&merged_config, ConfigSource::Global, &mut sources);

        // Apply area overrides
        if let Some(area_id) = area_id {
            if let Some(area_config) = self.area_overrides.get(area_id) {
                merged_config = self.deep_merge_configs(merged_config, area_config.clone())?;
                self.track_config_sources(area_config, ConfigSource::Area(area_id.to_string()), &mut sources);
            }
        }

        // Apply PvP template overrides
        if let Some(template_id) = pvp_template {
            if let Some(pvp_config) = self.pvp_overrides.get(template_id) {
                merged_config = self.deep_merge_configs(merged_config, pvp_config.clone())?;
                self.track_config_sources(pvp_config, ConfigSource::PvP(template_id.to_string()), &mut sources);
            }
        }

        // Validate final merged configuration
        self.validate_config(&merged_config)?;

        Ok(MergedConfig {
            config: merged_config,
            sources,
        })
    }

    /// Deep merge two configurations
    fn deep_merge_configs(&self, mut base: ExhaustionConfig, override_config: ExhaustionConfig) -> Result<ExhaustionConfig, ConfigLoaderError> {
        // Merge hysteresis_default (override takes precedence)
        // TODO: Load default hysteresis value from configuration instead of hardcoding 0.0
        if override_config.hysteresis_default != 0.0 {
            base.hysteresis_default = override_config.hysteresis_default;
        }

        // Merge events
        if override_config.events.coalesce_window_ms != 0 {
            base.events.coalesce_window_ms = override_config.events.coalesce_window_ms;
        }

        // Merge priorities
        if let Some(override_priorities) = override_config.priorities {
            base.priorities = Some(override_priorities);
        }

        // Merge archetypes
        for (archetype_name, override_archetype) in override_config.archetypes {
            let base_archetype = base.archetypes.entry(archetype_name.clone())
                .or_insert_with(|| ArchetypeConfig {
                    resources: HashMap::new(),
                });

            // Merge resources within archetype
            for (resource_name, override_resource) in override_archetype.resources {
                let base_resource = base_archetype.resources.entry(resource_name.clone())
                    .or_insert_with(|| ResourceConfig {
                        thresholds: Vec::new(),
                    });

        // Merge thresholds
        self.merge_thresholds(&mut base_resource.thresholds, override_resource.thresholds)?;
    }
}

// Fill hysteresis defaults before validation
self.fill_hysteresis_defaults(&mut base)?;

Ok(base)
    }

    /// Merge thresholds, handling duplicates by ID
    fn merge_thresholds(&self, base_thresholds: &mut Vec<ThresholdConfig>, override_thresholds: Vec<ThresholdConfig>) -> Result<(), ConfigLoaderError> {
        for override_threshold in override_thresholds {
            // Check if threshold with same ID already exists
            if let Some(existing_index) = base_thresholds.iter().position(|t| t.id == override_threshold.id) {
                // Replace existing threshold
                base_thresholds[existing_index] = override_threshold;
            } else {
                // Add new threshold
                base_thresholds.push(override_threshold);
            }
        }

        // Sort thresholds by order
        // TODO: Load default order value from configuration instead of hardcoding 0
        base_thresholds.sort_by_key(|t| t.order.unwrap_or(0));

        Ok(())
    }

    /// Fill hysteresis defaults for thresholds that don't have exit conditions
    fn fill_hysteresis_defaults(&self, config: &mut ExhaustionConfig) -> Result<(), ConfigLoaderError> {
        let h = config.hysteresis_default;
        
        for (_, arch) in &mut config.archetypes {
            for (_, res) in &mut arch.resources {
                for th in &mut res.thresholds {
                    // Fill percentage-based hysteresis
                    if let Some(enter_p) = th.enter_percent_lte {
                        if th.exit_percent_gte.is_none() {
                            th.exit_percent_gte = Some((enter_p + h).min(1.0));
                        }
                    }
                    
                    // Fill value-based hysteresis
                    if let Some(enter_v) = th.enter_value_eq {
                        if th.exit_value_ge.is_none() {
                            th.exit_value_ge = Some(enter_v + 1.0);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Validate canonical enum values
    fn validate_canonical_enums(&self, config: &ExhaustionConfig) -> Result<(), ConfigLoaderError> {
        // Canonical action tags
        let valid_action_tags = [
            "shield_activation", "buff_activation", "parry", "block", "cast", "sprint",
            "dodge", "counter_attack", "special_ability", "ultimate_ability"
        ];
        
        // Canonical effect types
        let valid_effect_types = [
            "disable_tags", "damage_multiplier", "set_flag", "resource_multiplier",
            "cooldown_multiplier", "movement_speed_multiplier", "cast_time_multiplier"
        ];

        for (archetype_name, archetype_config) in &config.archetypes {
            for (resource_name, resource_config) in &archetype_config.resources {
                for threshold in &resource_config.thresholds {
                    for effect in &threshold.effects {
                        // Validate effect type
                        if !valid_effect_types.contains(&effect.effect_type.as_str()) {
                            return Err(ConfigLoaderError::ValidationError(
                                format!("Invalid effect type '{}' in {}.{}.{}", 
                                    effect.effect_type, archetype_name, resource_name, threshold.id)
                            ));
                        }

                        // Validate action tags if effect type is disable_tags
                        if effect.effect_type == "disable_tags" {
                            if let Some(values) = &effect.values {
                                for tag in values {
                                    if !valid_action_tags.contains(&tag.as_str()) {
                                        return Err(ConfigLoaderError::ValidationError(
                                            format!("Invalid action tag '{}' in {}.{}.{}", 
                                                tag, archetype_name, resource_name, threshold.id)
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Track configuration sources for debugging
    fn track_config_sources(&self, config: &ExhaustionConfig, source: ConfigSource, sources: &mut HashMap<String, ConfigSource>) {
        // Track top-level sources
        sources.insert("version".to_string(), source.clone());
        sources.insert("hysteresis_default".to_string(), source.clone());
        sources.insert("events.coalesce_window_ms".to_string(), source.clone());

        if config.priorities.is_some() {
            sources.insert("priorities".to_string(), source.clone());
        }

        // Track archetype sources
        for (archetype_name, archetype_config) in &config.archetypes {
            for (resource_name, resource_config) in &archetype_config.resources {
                for threshold in &resource_config.thresholds {
                    let key = format!("archetypes.{}.{}.thresholds.{}", archetype_name, resource_name, threshold.id);
                    sources.insert(key, source.clone());
                }
            }
        }
    }

    /// Validate configuration
    fn validate_config(&self, config: &ExhaustionConfig) -> Result<(), ConfigLoaderError> {
        // Validate canonical enums
        self.validate_canonical_enums(config)?;
        // Validate version
        if config.version == 0 {
            return Err(ConfigLoaderError::ValidationError("Version must be >= 1".to_string()));
        }

        // Validate hysteresis default
        if config.hysteresis_default < 0.0 || config.hysteresis_default > 1.0 {
            return Err(ConfigLoaderError::ValidationError(
                "Hysteresis default must be between 0.0 and 1.0".to_string()
            ));
        }

        // Validate archetypes and thresholds
        for (archetype_name, archetype_config) in &config.archetypes {
            for (resource_name, resource_config) in &archetype_config.resources {
                let mut threshold_ids = std::collections::HashSet::new();
                
                for threshold in &resource_config.thresholds {
                    // Validate threshold ID uniqueness
                    if !threshold_ids.insert(&threshold.id) {
                        return Err(ConfigLoaderError::ValidationError(
                            format!("Duplicate threshold ID '{}' in {}.{}", 
                                threshold.id, archetype_name, resource_name)
                        ));
                    }

                    // Validate enter condition
                    let has_enter_percent = threshold.enter_percent_lte.is_some();
                    let has_enter_value = threshold.enter_value_eq.is_some();
                    
                    if !has_enter_percent && !has_enter_value {
                        return Err(ConfigLoaderError::ValidationError(
                            format!("Threshold '{}' must have either enter_percent_lte or enter_value_eq", 
                                threshold.id)
                        ));
                    }

                    if has_enter_percent && has_enter_value {
                        return Err(ConfigLoaderError::ValidationError(
                            format!("Threshold '{}' cannot have both enter_percent_lte and enter_value_eq", 
                                threshold.id)
                        ));
                    }

                    // Validate exit condition
                    if let Some(enter_percent) = threshold.enter_percent_lte {
                        if let Some(exit_percent) = threshold.exit_percent_gte {
                            if exit_percent < enter_percent {
                                return Err(ConfigLoaderError::ValidationError(
                                    format!("Threshold '{}' exit_percent_gte ({}) must be >= enter_percent_lte ({})", 
                                        threshold.id, exit_percent, enter_percent)
                                ));
                            }
                        }
                    }

                    if let Some(enter_value) = threshold.enter_value_eq {
                        if let Some(exit_value) = threshold.exit_value_ge {
                            if exit_value < enter_value {
                                return Err(ConfigLoaderError::ValidationError(
                                    format!("Threshold '{}' exit_value_ge ({}) must be >= enter_value_eq ({})", 
                                        threshold.id, exit_value, enter_value)
                                ));
                            }
                        }
                    }

                    // Validate effects
                    if threshold.effects.is_empty() {
                        return Err(ConfigLoaderError::ValidationError(
                            format!("Threshold '{}' must have at least one effect", threshold.id)
                        ));
                    }

                    for effect in &threshold.effects {
                        self.validate_effect(effect)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate effect configuration
    fn validate_effect(&self, effect: &EffectConfig) -> Result<(), ConfigLoaderError> {
        match effect.effect_type.as_str() {
            "disable_tags" | "disable_cost_type" | "break_active_shields" | "action_lockout" => {
                if effect.values.is_none() || effect.values.as_ref().map_or(true, |v| v.is_empty()) {
                    return Err(ConfigLoaderError::ValidationError(
                        format!("Effect '{}' requires non-empty values", effect.effect_type)
                    ));
                }
            }
            "damage_multiplier" | "incoming_multiplier" => {
                if effect.categories.is_none() || effect.categories.as_ref().map_or(true, |c| c.is_empty()) {
                    return Err(ConfigLoaderError::ValidationError(
                        format!("Effect '{}' requires non-empty categories", effect.effect_type)
                    ));
                }
                if effect.modifier.is_none() {
                    return Err(ConfigLoaderError::ValidationError(
                        format!("Effect '{}' requires modifier", effect.effect_type)
                    ));
                }
            }
            "cast_time_modifier" | "move_speed_modifier" | "taunt_effectiveness_modifier" => {
                if effect.modifier.is_none() {
                    return Err(ConfigLoaderError::ValidationError(
                        format!("Effect '{}' requires modifier", effect.effect_type)
                    ));
                }
            }
            "set_flag" => {
                if effect.name.is_none() || effect.value.is_none() {
                    return Err(ConfigLoaderError::ValidationError(
                        "Effect 'set_flag' requires name and value".to_string()
                    ));
                }
            }
            "stagger_susceptibility" => {
                if effect.level.is_none() {
                    return Err(ConfigLoaderError::ValidationError(
                        "Effect 'stagger_susceptibility' requires level".to_string()
                    ));
                }
                // TODO: Load valid stagger levels from configuration instead of hardcoding
                let level = effect.level.as_ref().unwrap();
                if !["light", "medium", "heavy"].contains(&level.as_str()) {
                    return Err(ConfigLoaderError::ValidationError(
                        format!("Effect 'stagger_susceptibility' level must be light, medium, or heavy, got: {}", level)
                    ));
                }
            }
            "regen_modifier" => {
                if effect.resource.is_none() || effect.modifier.is_none() {
                    return Err(ConfigLoaderError::ValidationError(
                        "Effect 'regen_modifier' requires resource and modifier".to_string()
                    ));
                }
            }
            _ => {
                return Err(ConfigLoaderError::ValidationError(
                    format!("Unknown effect type: {}", effect.effect_type)
                ));
            }
        }

        Ok(())
    }

    /// Get debug information about configuration sources
    pub fn get_debug_info(&self, merged_config: &MergedConfig) -> String {
        let mut info = String::new();
        info.push_str("Configuration Sources:\n");
        info.push_str("===================\n");

        for (key, source) in &merged_config.sources {
            let source_str = match source {
                ConfigSource::Global => "global",
                ConfigSource::Area(area_id) => &area_id,
                ConfigSource::PvP(template_id) => &template_id,
            };
            info.push_str(&format!("  {}: {}\n", key, source_str));
        }

        info
    }

    /// Pretty print merged configuration with source information
    pub fn pretty_print_merged(&self, merged: &MergedConfig) -> String {
        let mut output = String::new();
        output.push_str("=== Merged Exhaustion Configuration ===\n");
        output.push_str(&format!("Version: {} (source: {})\n", 
            merged.config.version, 
            self.get_source_name(&merged.sources.get("version"))
        ));
        output.push_str(&format!("Hysteresis Default: {} (source: {})\n", 
            merged.config.hysteresis_default,
            self.get_source_name(&merged.sources.get("hysteresis_default"))
        ));
        output.push_str(&format!("Coalesce Window: {}ms (source: {})\n", 
            merged.config.events.coalesce_window_ms,
            self.get_source_name(&merged.sources.get("events.coalesce_window_ms"))
        ));

        if let Some(priorities) = &merged.config.priorities {
            output.push_str(&format!("Priorities: {:?} (source: {})\n", 
                priorities.categories,
                self.get_source_name(&merged.sources.get("priorities"))
            ));
        }

        output.push_str("\n=== Archetypes ===\n");
        for (archetype_name, archetype_config) in &merged.config.archetypes {
            output.push_str(&format!("\n[{}]\n", archetype_name));
            for (resource_name, resource_config) in &archetype_config.resources {
                output.push_str(&format!("  {}:\n", resource_name));
                for threshold in &resource_config.thresholds {
                    let key = format!("archetypes.{}.{}.thresholds.{}", archetype_name, resource_name, threshold.id);
                    let source = self.get_source_name(&merged.sources.get(&key));
                    output.push_str(&format!("    {} (source: {})\n", threshold.id, source));
                    output.push_str(&format!("      Order: {:?}\n", threshold.order));
                    if let Some(enter_p) = threshold.enter_percent_lte {
                        output.push_str(&format!("      Enter: {}% (percentage)\n", enter_p * 100.0));
                    }
                    if let Some(exit_p) = threshold.exit_percent_gte {
                        output.push_str(&format!("      Exit: {}% (percentage)\n", exit_p * 100.0));
                    }
                    if let Some(enter_v) = threshold.enter_value_eq {
                        output.push_str(&format!("      Enter: {} (value)\n", enter_v));
                    }
                    if let Some(exit_v) = threshold.exit_value_ge {
                        output.push_str(&format!("      Exit: {} (value)\n", exit_v));
                    }
                    output.push_str(&format!("      Effects: {} items\n", threshold.effects.len()));
                }
            }
        }

        output
    }

    /// Get source name for display
    fn get_source_name<'a>(&self, source: &Option<&'a ConfigSource>) -> &'a str {
        match source {
            Some(ConfigSource::Global) => "global",
            Some(ConfigSource::Area(area_id)) => area_id,
            Some(ConfigSource::PvP(template_id)) => template_id,
            None => "unknown",
        }
    }
}

impl Default for ExhaustionConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}