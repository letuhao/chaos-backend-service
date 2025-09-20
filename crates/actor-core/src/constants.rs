//! System-level constants for the Actor Core system.
//!
//! This module contains only system-level constants that are not game-specific.
//! All configurable values are loaded from configuration files at runtime
//! through the ConfigManager.

use std::sync::Arc;
use crate::config::manager::ConfigurationManager;
use crate::config::loaders::*;

/// System identifiers for various game systems.
pub mod system_ids {
    /// Luyen The (Cultivation) system
    pub const LUYEN_THE: &str = "luyen_the";
    /// Kim Dan (Golden Pill) system
    pub const KIM_DAN: &str = "kim_dan";
    /// Combat system
    pub const COMBAT: &str = "combat";
    /// Equipment system
    pub const EQUIPMENT: &str = "equipment";
    /// Buff system
    pub const BUFF: &str = "buff";
    /// Guild system
    pub const GUILD: &str = "guild";
    /// Event system
    pub const EVENT: &str = "event";
    /// World system
    pub const WORLD: &str = "world";
    /// Magic system
    pub const MAGIC: &str = "magic";
    /// Cultivation system
    pub const CULTIVATION: &str = "cultivation";
    /// Experience system
    pub const EXPERIENCE: &str = "experience";
    /// Reputation system
    pub const REPUTATION: &str = "reputation";
    /// Trading system
    pub const TRADING: &str = "trading";
    /// Weather system
    pub const WEATHER: &str = "weather";
    /// Location system
    pub const LOCATION: &str = "location";
    /// Time system
    pub const TIME: &str = "time";
    /// Stealth system
    pub const STEALTH: &str = "stealth";
    /// Perception system
    pub const PERCEPTION: &str = "perception";
}

/// Error codes for the system.
pub mod error_codes {
    /// Invalid actor
    pub const INVALID_ACTOR: &str = "INVALID_ACTOR";
    /// Invalid contribution
    pub const INVALID_CONTRIBUTION: &str = "INVALID_CONTRIBUTION";
    /// Invalid cap
    pub const INVALID_CAP: &str = "INVALID_CAP";
    /// Subsystem error
    pub const SUBSYSTEM_ERROR: &str = "SUBSYSTEM_ERROR";
    /// Cache error
    pub const CACHE_ERROR: &str = "CACHE_ERROR";
    /// Registry error
    pub const REGISTRY_ERROR: &str = "REGISTRY_ERROR";
    /// Aggregation error
    pub const AGGREGATION_ERROR: &str = "AGGREGATION_ERROR";
    /// Configuration error
    pub const CONFIGURATION_ERROR: &str = "CONFIGURATION_ERROR";
}

/// Error types for categorization.
pub mod error_types {
    /// Validation error
    pub const VALIDATION: &str = "VALIDATION";
    /// System error
    pub const SYSTEM: &str = "SYSTEM";
    /// Network error
    pub const NETWORK: &str = "NETWORK";
    /// Database error
    pub const DATABASE: &str = "DATABASE";
    /// Cache error
    pub const CACHE: &str = "CACHE";
    /// Configuration error
    pub const CONFIGURATION: &str = "CONFIGURATION";
}

/// Clamp ranges for various dimensions (loaded from configuration at runtime)
pub mod clamp_ranges {
    use std::sync::Arc;
    use crate::config::manager::ConfigurationManager;
    use crate::ActorCoreResult;
    
    /// Get the clamp range for a dimension from configuration
    /// 
    /// # Errors
    /// Returns `ActorCoreError::ConfigurationError` if:
    /// - Configuration category "clamp_ranges" is not found
    /// - Dimension is not configured
    /// - Min/max values are missing or invalid
    pub async fn get_range(
        dimension: &str, 
        config_manager: Arc<ConfigurationManager>
    ) -> ActorCoreResult<(f64, f64)> {
        // Load clamp ranges from configuration
        let config = config_manager.get_category_config("clamp_ranges").await?;
        
        let ranges_config = config.get("ranges")
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                format!("Clamp ranges configuration not found for category 'clamp_ranges'")
            ))?;
        
        let dimension_config = ranges_config.value.get(dimension)
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                format!("Clamp range not configured for dimension '{}'", dimension)
            ))?;
        
        let min = dimension_config.get("min")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                format!("Missing or invalid 'min' value for dimension '{}'", dimension)
            ))?;
        
        let max = dimension_config.get("max")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                format!("Missing or invalid 'max' value for dimension '{}'", dimension)
            ))?;
        
        if min >= max {
            return Err(crate::ActorCoreError::ConfigurationError(
                format!("Invalid clamp range for dimension '{}': min ({}) must be less than max ({})", dimension, min, max)
            ));
        }
        
        Ok((min, max))
    }
    
}

/// Configuration-based constants loader
pub struct ConfigConstants {
    config_manager: Arc<ConfigurationManager>,
}

impl ConfigConstants {
    /// Create a new configuration constants loader
    pub fn new(config_manager: Arc<ConfigurationManager>) -> Self {
        Self { config_manager }
    }

    /// Get default values from configuration
    pub async fn get_defaults(&self) -> crate::ActorCoreResult<DefaultsConfig> {
        let config = self.config_manager.get_category_config("defaults").await?;
        
        let mut resources = std::collections::HashMap::new();
        let mut stats = std::collections::HashMap::new();
        let mut elements = std::collections::HashMap::new();
        
        // Load resource defaults
        if let Some(resources_config) = config.get("resources") {
            for (key, value) in resources_config.value.as_object().unwrap_or(&serde_json::Map::new()) {
                if let Some(resource_obj) = value.as_object() {
                    let resource_defaults = ResourceDefaults {
                        base_value: resource_obj.get("base_value")
                            .and_then(|v| v.as_f64())
                            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                                format!("Missing or invalid 'base_value' for resource '{}'", key)
                            ))?,
                        min_value: resource_obj.get("min_value")
                            .and_then(|v| v.as_f64())
                            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                                format!("Missing or invalid 'min_value' for resource '{}'", key)
                            ))?,
                        max_value: resource_obj.get("max_value")
                            .and_then(|v| v.as_f64())
                            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                                format!("Missing or invalid 'max_value' for resource '{}'", key)
                            ))?,
                        regen_rate: resource_obj.get("regen_rate")
                            .and_then(|v| v.as_f64())
                            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                                format!("Missing or invalid 'regen_rate' for resource '{}'", key)
                            ))?,
                        regen_type: resource_obj.get("regen_type")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                                format!("Missing or invalid 'regen_type' for resource '{}'", key)
                            ))?
                            .to_string(),
                    };
                    resources.insert(key.clone(), resource_defaults);
                }
            }
        }
        
        // Load stat defaults
        if let Some(stats_config) = config.get("stats") {
            for (key, value) in stats_config.value.as_object().unwrap_or(&serde_json::Map::new()) {
                if let Some(stat_obj) = value.as_object() {
                    let stat_defaults = StatDefaults {
                        base_value: stat_obj.get("base_value")
                            .and_then(|v| v.as_f64())
                            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                                format!("Missing or invalid 'base_value' for stat '{}'", key)
                            ))?,
                        min_value: stat_obj.get("min_value")
                            .and_then(|v| v.as_f64())
                            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                                format!("Missing or invalid 'min_value' for stat '{}'", key)
                            ))?,
                        max_value: stat_obj.get("max_value")
                            .and_then(|v| v.as_f64())
                            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                                format!("Missing or invalid 'max_value' for stat '{}'", key)
                            ))?,
                    };
                    stats.insert(key.clone(), stat_defaults);
                }
            }
        }
        
        // Load element defaults
        if let Some(elements_config) = config.get("elements") {
            for (key, value) in elements_config.value.as_object().unwrap_or(&serde_json::Map::new()) {
                if let Some(element_obj) = value.as_object() {
                    let element_defaults = ElementDefaults {
                        base_affinity: element_obj.get("base_affinity")
                            .and_then(|v| v.as_f64())
                            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                                format!("Missing or invalid 'base_affinity' for element '{}'", key)
                            ))?,
                        min_affinity: element_obj.get("min_affinity")
                            .and_then(|v| v.as_f64())
                            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                                format!("Missing or invalid 'min_affinity' for element '{}'", key)
                            ))?,
                        max_affinity: element_obj.get("max_affinity")
                            .and_then(|v| v.as_f64())
                            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                                format!("Missing or invalid 'max_affinity' for element '{}'", key)
                            ))?,
                    };
                    elements.insert(key.clone(), element_defaults);
                }
            }
        }
        
        Ok(DefaultsConfig {
            resources,
            stats,
            elements,
        })
    }

    /// Get timeouts from configuration
    pub async fn get_timeouts(&self) -> crate::ActorCoreResult<TimeoutsConfig> {
        let config = self.config_manager.get_category_config("timeouts").await?;
        
        Ok(TimeoutsConfig {
            cache_ttl: config.get("cache_ttl")
                .and_then(|v| v.value.as_u64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'cache_ttl' in timeouts configuration".to_string()
                ))?,
            aggregation_timeout: config.get("aggregation_timeout")
                .and_then(|v| v.value.as_f64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'aggregation_timeout' in timeouts configuration".to_string()
                ))?,
            validation_timeout: config.get("validation_timeout")
                .and_then(|v| v.value.as_f64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'validation_timeout' in timeouts configuration".to_string()
                ))?,
            regeneration_interval: config.get("regeneration_interval")
                .and_then(|v| v.value.as_f64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'regeneration_interval' in timeouts configuration".to_string()
                ))?,
            subsystem_timeout: config.get("subsystem_timeout")
                .and_then(|v| v.value.as_f64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'subsystem_timeout' in timeouts configuration".to_string()
                ))?,
        })
    }

    /// Get performance thresholds from configuration
    pub async fn get_performance_thresholds(&self) -> crate::ActorCoreResult<PerformanceThresholdsConfig> {
        let config = self.config_manager.get_category_config("performance_thresholds").await?;
        
        Ok(PerformanceThresholdsConfig {
            max_actors: config.get("max_actors")
                .and_then(|v| v.value.as_u64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'max_actors' in performance_thresholds configuration".to_string()
                ))? as usize,
            max_contributions_per_actor: config.get("max_contributions_per_actor")
                .and_then(|v| v.value.as_u64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'max_contributions_per_actor' in performance_thresholds configuration".to_string()
                ))? as usize,
            max_caps_per_actor: config.get("max_caps_per_actor")
                .and_then(|v| v.value.as_u64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'max_caps_per_actor' in performance_thresholds configuration".to_string()
                ))? as usize,
            max_subsystems: config.get("max_subsystems")
                .and_then(|v| v.value.as_u64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'max_subsystems' in performance_thresholds configuration".to_string()
                ))? as usize,
            cache_size_mb: config.get("cache_size_mb")
                .and_then(|v| v.value.as_u64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'cache_size_mb' in performance_thresholds configuration".to_string()
                ))? as usize,
            memory_usage_mb: config.get("memory_usage_mb")
                .and_then(|v| v.value.as_u64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'memory_usage_mb' in performance_thresholds configuration".to_string()
                ))? as usize,
            cpu_usage_percent: config.get("cpu_usage_percent")
                .and_then(|v| v.value.as_f64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'cpu_usage_percent' in performance_thresholds configuration".to_string()
                ))?,
        })
    }

    /// Get validation rules from configuration
    pub async fn get_validation_rules(&self) -> crate::ActorCoreResult<ValidationRulesConfig> {
        let config = self.config_manager.get_category_config("validation_rules").await?;
        
        let get_value_range = |key: &str| -> crate::ActorCoreResult<ValueRange> {
            let range_config = config.get(key)
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Missing '{}' in validation_rules configuration", key)
                ))?;
            
            let min_value = range_config.value.get("min")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Missing or invalid 'min' value for '{}' in validation_rules configuration", key)
                ))?;
            
            let max_value = range_config.value.get("max")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Missing or invalid 'max' value for '{}' in validation_rules configuration", key)
                ))?;
            
            if min_value >= max_value {
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Invalid range for '{}': min ({}) must be less than max ({})", key, min_value, max_value)
                ));
            }
            
            Ok(ValueRange { min_value, max_value })
        };
        
        Ok(ValidationRulesConfig {
            resource_values: get_value_range("resource_values")?,
            stat_values: get_value_range("stat_values")?,
            element_affinities: get_value_range("element_affinities")?,
            contribution_values: get_value_range("contribution_values")?,
        })
    }

    /// Get cache keys from configuration
    pub async fn get_cache_keys(&self) -> crate::ActorCoreResult<CacheKeysConfig> {
        let config = self.config_manager.get_category_config("cache_keys").await?;
        
        Ok(CacheKeysConfig {
            actor_snapshot: config.get("actor_snapshot")
                .and_then(|v| v.value.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'actor_snapshot' in cache_keys configuration".to_string()
                ))?
                .to_string(),
            resource_regeneration: config.get("resource_regeneration")
                .and_then(|v| v.value.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'resource_regeneration' in cache_keys configuration".to_string()
                ))?
                .to_string(),
            stat_aggregation: config.get("stat_aggregation")
                .and_then(|v| v.value.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'stat_aggregation' in cache_keys configuration".to_string()
                ))?
                .to_string(),
            subsystem_data: config.get("subsystem_data")
                .and_then(|v| v.value.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'subsystem_data' in cache_keys configuration".to_string()
                ))?
                .to_string(),
        })
    }

    /// Get log levels from configuration
    pub async fn get_log_levels(&self) -> crate::ActorCoreResult<LogLevelsConfig> {
        let config = self.config_manager.get_category_config("log_levels").await?;
        
        Ok(LogLevelsConfig {
            actor_core: config.get("actor_core")
                .and_then(|v| v.value.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'actor_core' in log_levels configuration".to_string()
                ))?
                .to_string(),
            config: config.get("config")
                .and_then(|v| v.value.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'config' in log_levels configuration".to_string()
                ))?
                .to_string(),
            registry: config.get("registry")
                .and_then(|v| v.value.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'registry' in log_levels configuration".to_string()
                ))?
                .to_string(),
            cache: config.get("cache")
                .and_then(|v| v.value.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'cache' in log_levels configuration".to_string()
                ))?
                .to_string(),
            performance: config.get("performance")
                .and_then(|v| v.value.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    "Missing or invalid 'performance' in log_levels configuration".to_string()
                ))?
                .to_string(),
        })
    }

    /// Get cache policies from configuration
    pub async fn get_cache_policies(&self) -> crate::ActorCoreResult<CachePoliciesConfig> {
        let config = self.config_manager.get_category_config("cache_policies").await?;
        
        let get_cache_policy = |key: &str| -> crate::ActorCoreResult<CachePolicy> {
            let policy_config = config.get(key)
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Missing '{}' in cache_policies configuration", key)
                ))?;
            
            let ttl = policy_config.value.get("ttl")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Missing or invalid 'ttl' for '{}' in cache_policies configuration", key)
                ))?;
            
            let max_size = policy_config.value.get("max_size")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Missing or invalid 'max_size' for '{}' in cache_policies configuration", key)
                ))? as usize;
            
            let eviction_policy = policy_config.value.get("eviction_policy")
                .and_then(|v| v.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Missing or invalid 'eviction_policy' for '{}' in cache_policies configuration", key)
                ))?
                .to_string();
            
            Ok(CachePolicy {
                ttl,
                max_size,
                eviction_policy,
            })
        };
        
        Ok(CachePoliciesConfig {
            actor_snapshot: get_cache_policy("actor_snapshot")?,
            resource_regeneration: get_cache_policy("resource_regeneration")?,
            stat_aggregation: get_cache_policy("stat_aggregation")?,
        })
    }

    /// Get system IDs from configuration
    pub async fn get_system_ids(&self) -> crate::ActorCoreResult<Vec<String>> {
        let config = self.config_manager.get_category_config("system_ids").await?;
        
        let system_ids = config.get("supported_systems")
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Missing 'supported_systems' in system_ids configuration".to_string()
            ))?;
        
        let systems_array = system_ids.value.as_array()
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Invalid system_ids configuration: 'supported_systems' must be an array".to_string()
            ))?;
        
        let systems: Vec<String> = systems_array
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        
        if systems.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "No valid system IDs found in configuration".to_string()
            ));
        }
        
        Ok(systems)
    }

    /// Get context types from configuration
    pub async fn get_context_types(&self) -> crate::ActorCoreResult<Vec<String>> {
        let config = self.config_manager.get_category_config("context_types").await?;
        
        let context_types = config.get("supported_contexts")
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Missing 'supported_contexts' in context_types configuration".to_string()
            ))?;
        
        let contexts_array = context_types.value.as_array()
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Invalid context_types configuration: 'supported_contexts' must be an array".to_string()
            ))?;
        
        let contexts: Vec<String> = contexts_array
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        
        if contexts.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "No valid context types found in configuration".to_string()
            ));
        }
        
        Ok(contexts)
    }
}

/// All supported system IDs (loaded from configuration at runtime)
pub async fn all_system_ids(config_manager: Arc<ConfigurationManager>) -> crate::ActorCoreResult<Vec<String>> {
    config_manager.get_system_ids().await
}

/// All supported context types (loaded from configuration at runtime)
pub async fn all_context_types(config_manager: Arc<ConfigurationManager>) -> crate::ActorCoreResult<Vec<String>> {
    config_manager.get_context_types().await
}

/// All supported dimensions (loaded from configuration at runtime)
/// 
/// # Errors
/// Returns `ActorCoreError::ConfigurationError` if:
/// - Configuration category "dimensions" is not found
/// - "supported_dimensions" is not configured
/// - No valid dimensions are found
pub async fn all_dimensions(config_manager: Arc<ConfigurationManager>) -> crate::ActorCoreResult<Vec<String>> {
    // Load dimensions from configuration
    let config = config_manager.get_category_config("dimensions").await?;
    
    let dimensions_config = config.get("supported_dimensions")
        .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
            "Dimensions configuration not found for category 'dimensions'".to_string()
        ))?;
    
    let dimensions_array = dimensions_config.value.as_array()
        .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
            "Invalid dimensions configuration: 'supported_dimensions' must be an array".to_string()
        ))?;
    
    let dimensions: Vec<String> = dimensions_array
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();
    
    if dimensions.is_empty() {
        return Err(crate::ActorCoreError::ConfigurationError(
            "No valid dimensions found in configuration".to_string()
        ));
    }
    
    Ok(dimensions)
}


// NOTE: All dimensions, stats, and performance thresholds are now loaded
// from the Runtime Registry and Configuration Hub at runtime.
// 
// - Dimensions: Registered by subsystems via RegistryManager
// - Performance thresholds: Loaded from YAML configs via ConfigurationManager
// - No hardcoded constants should exist here

// NOTE: All configurable values (defaults, timeouts, performance_thresholds, validation_rules)
// are now loaded from configuration files at runtime through the ConfigManager.
// Use the ConfigConstants struct to access these values dynamically.
//
// For examples of how to define configuration values, see:
// - `configs/actor_core_config.yaml`
// - `examples/rpg_resource_config.yaml`
// - `examples/magic_resource_config.yaml`
// - `examples/legacy_subsystems/` (for reference)