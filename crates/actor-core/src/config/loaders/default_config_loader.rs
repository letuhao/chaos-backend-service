//! Default configuration loader for Actor Core

// use std::sync::Arc; // Unused import
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
// use tracing::{info, warn, error}; // Unused imports

use crate::config::provider::{ConfigurationProvider, BaseConfigurationProvider};
use crate::config::types::*;
use crate::ActorCoreResult;
// use crate::ActorCoreError; // Unused import

/// Default configuration data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultConfig {
    pub defaults: DefaultsConfig,
    pub timeouts: TimeoutsConfig,
    pub performance_thresholds: PerformanceThresholdsConfig,
    pub validation_rules: ValidationRulesConfig,
    pub cache_keys: CacheKeysConfig,
    pub log_levels: LogLevelsConfig,
    pub cache_policies: CachePoliciesConfig,
    pub system_ids: Vec<String>,
    pub context_types: Vec<String>,
}

/// Defaults configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
    pub resources: std::collections::HashMap<String, ResourceDefaults>,
    pub stats: std::collections::HashMap<String, StatDefaults>,
    pub elements: std::collections::HashMap<String, ElementDefaults>,
}

/// Resource defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDefaults {
    pub base_value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub regen_rate: f64,
    pub regen_type: String,
}

/// Stat defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatDefaults {
    pub base_value: f64,
    pub min_value: f64,
    pub max_value: f64,
}

/// Element defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDefaults {
    pub base_affinity: f64,
    pub min_affinity: f64,
    pub max_affinity: f64,
}

/// Timeouts configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutsConfig {
    pub cache_ttl: u64,
    pub aggregation_timeout: f64,
    pub validation_timeout: f64,
    pub regeneration_interval: f64,
    pub subsystem_timeout: f64,
}

/// Performance thresholds configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholdsConfig {
    pub max_actors: usize,
    pub max_contributions_per_actor: usize,
    pub max_caps_per_actor: usize,
    pub max_subsystems: usize,
    pub cache_size_mb: usize,
    pub memory_usage_mb: usize,
    pub cpu_usage_percent: f64,
}

/// Validation rules configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRulesConfig {
    pub resource_values: ValueRange,
    pub stat_values: ValueRange,
    pub element_affinities: ValueRange,
    pub contribution_values: ValueRange,
}

/// Value range for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueRange {
    pub min_value: f64,
    pub max_value: f64,
}

/// Cache keys configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheKeysConfig {
    pub actor_snapshot: String,
    pub resource_regeneration: String,
    pub stat_aggregation: String,
    pub subsystem_data: String,
}

/// Log levels configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogLevelsConfig {
    pub actor_core: String,
    pub config: String,
    pub registry: String,
    pub cache: String,
    pub performance: String,
}

/// Cache policies configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePoliciesConfig {
    pub actor_snapshot: CachePolicy,
    pub resource_regeneration: CachePolicy,
    pub stat_aggregation: CachePolicy,
}

/// Cache policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePolicy {
    pub ttl: u64,
    pub max_size: usize,
    pub eviction_policy: String,
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_id: String,
    pub priority: i64,
    pub supported_categories: Vec<String>,
}

/// Default configuration provider
pub struct DefaultConfigProvider {
    base: BaseConfigurationProvider,
    config_data: DefaultConfig,
}

impl DefaultConfigProvider {
    pub fn new(config_path: PathBuf) -> ActorCoreResult<Self> {
        // Load configuration from file
        tracing::info!("📄 Loading configuration from file: {:?}", config_path);
        let config_content = std::fs::read_to_string(&config_path)?;
        tracing::info!("📄 File content loaded, size: {} bytes", config_content.len());
        
        let config_data: DefaultConfig = match serde_yaml::from_str(&config_content) {
            Ok(data) => {
                tracing::info!("✅ Successfully parsed YAML configuration from {:?}", config_path);
                data
            }
            Err(e) => {
                tracing::error!("❌ Failed to parse YAML configuration from {:?}", config_path);
                tracing::error!("🔍 YAML parsing error: {}", e);
                tracing::error!("🔍 Error location: line {}, column {}", e.location().map(|l| l.line()).unwrap_or(0), e.location().map(|l| l.column()).unwrap_or(0));
                tracing::error!("🔍 File content preview (first 500 chars):\n{}", &config_content.chars().take(500).collect::<String>());
                return Err(e.into());
            }
        };
        
        // Load provider configuration from config file
        let provider_config = Self::load_provider_config(&config_path)?;
        
        Ok(Self {
            base: BaseConfigurationProvider::new(
                provider_config.provider_id,
                provider_config.priority,
                provider_config.supported_categories,
            ),
            config_data,
        })
    }

    /// Load provider configuration from config file
    fn load_provider_config(config_path: &PathBuf) -> ActorCoreResult<ProviderConfig> {
        // Try to load from provider_config.yaml first
        let provider_config_path = config_path.parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("provider_config.yaml");
            
        if provider_config_path.exists() {
            match Self::load_provider_config_from_file(&provider_config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    tracing::warn!("Failed to load provider config from file: {}. Using fallback defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(ProviderConfig {
            provider_id: "default_config_provider".to_string(),
            priority: 1000, // High priority for defaults
            supported_categories: vec![
                "defaults".to_string(),
                "timeouts".to_string(),
                "performance_thresholds".to_string(),
                "validation_rules".to_string(),
                "cache_keys".to_string(),
                "log_levels".to_string(),
                "cache_policies".to_string(),
                "system_ids".to_string(),
                "context_types".to_string(),
            ],
        })
    }

    /// Load provider configuration from file
    fn load_provider_config_from_file(path: &PathBuf) -> ActorCoreResult<ProviderConfig> {
        let content = std::fs::read_to_string(path)?;
        let config: ProviderConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}


impl DefaultConfigProvider {
    /// Load merge rule from configuration
    fn load_merge_rule_from_config(&self, _category: &str, _key: &str) -> ActorCoreResult<Option<ConfigurationMergeRule>> {
        // TODO: Implement merge rule loading from merge_rules.yaml
        // This should check for category-specific and key-specific rules
        // For now, return None to use fallback
        Ok(None)
    }
}

#[async_trait::async_trait]
impl ConfigurationProvider for DefaultConfigProvider {
    fn provider_id(&self) -> &str {
        self.base.provider_id()
    }
    
    fn priority(&self) -> i64 {
        self.base.priority()
    }
    
    fn get_supported_categories(&self) -> Vec<String> {
        self.base.get_supported_categories()
    }
    
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>> {
        let value = match category {
            "defaults" => {
                match key {
                    "resources" => Some(serde_json::to_value(&self.config_data.defaults.resources)?),
                    "stats" => Some(serde_json::to_value(&self.config_data.defaults.stats)?),
                    "elements" => Some(serde_json::to_value(&self.config_data.defaults.elements)?),
                    _ => None,
                }
            },
            "timeouts" => {
                match key {
                    "cache_ttl" => Some(serde_json::Value::Number(serde_json::Number::from(self.config_data.timeouts.cache_ttl))),
                    "aggregation_timeout" => Some(serde_json::Value::Number(serde_json::Number::from_f64(self.config_data.timeouts.aggregation_timeout).unwrap_or(serde_json::Number::from(5)))),
                    "validation_timeout" => Some(serde_json::Value::Number(serde_json::Number::from_f64(self.config_data.timeouts.validation_timeout).unwrap_or(serde_json::Number::from(1)))),
                    "regeneration_interval" => Some(serde_json::Value::Number(serde_json::Number::from_f64(self.config_data.timeouts.regeneration_interval).unwrap_or(serde_json::Number::from(1)))),
                    "subsystem_timeout" => Some(serde_json::Value::Number(serde_json::Number::from_f64(self.config_data.timeouts.subsystem_timeout).unwrap_or(serde_json::Number::from(10)))),
                    _ => None,
                }
            },
            "performance_thresholds" => {
                match key {
                    "max_actors" => Some(serde_json::Value::Number(serde_json::Number::from(self.config_data.performance_thresholds.max_actors))),
                    "max_contributions_per_actor" => Some(serde_json::Value::Number(serde_json::Number::from(self.config_data.performance_thresholds.max_contributions_per_actor))),
                    "max_caps_per_actor" => Some(serde_json::Value::Number(serde_json::Number::from(self.config_data.performance_thresholds.max_caps_per_actor))),
                    "max_subsystems" => Some(serde_json::Value::Number(serde_json::Number::from(self.config_data.performance_thresholds.max_subsystems))),
                    "cache_size_mb" => Some(serde_json::Value::Number(serde_json::Number::from(self.config_data.performance_thresholds.cache_size_mb))),
                    "memory_usage_mb" => Some(serde_json::Value::Number(serde_json::Number::from(self.config_data.performance_thresholds.memory_usage_mb))),
                    "cpu_usage_percent" => Some(serde_json::Value::Number(serde_json::Number::from_f64(self.config_data.performance_thresholds.cpu_usage_percent).unwrap_or(serde_json::Number::from(80)))),
                    _ => None,
                }
            },
            "validation_rules" => {
                match key {
                    "resource_values" => Some(serde_json::to_value(&self.config_data.validation_rules.resource_values)?),
                    "stat_values" => Some(serde_json::to_value(&self.config_data.validation_rules.stat_values)?),
                    "element_affinities" => Some(serde_json::to_value(&self.config_data.validation_rules.element_affinities)?),
                    "contribution_values" => Some(serde_json::to_value(&self.config_data.validation_rules.contribution_values)?),
                    _ => None,
                }
            },
            "cache_keys" => {
                match key {
                    "actor_snapshot" => Some(serde_json::Value::String(self.config_data.cache_keys.actor_snapshot.clone())),
                    "resource_regeneration" => Some(serde_json::Value::String(self.config_data.cache_keys.resource_regeneration.clone())),
                    "stat_aggregation" => Some(serde_json::Value::String(self.config_data.cache_keys.stat_aggregation.clone())),
                    "subsystem_data" => Some(serde_json::Value::String(self.config_data.cache_keys.subsystem_data.clone())),
                    _ => None,
                }
            },
            "log_levels" => {
                match key {
                    "actor_core" => Some(serde_json::Value::String(self.config_data.log_levels.actor_core.clone())),
                    "config" => Some(serde_json::Value::String(self.config_data.log_levels.config.clone())),
                    "registry" => Some(serde_json::Value::String(self.config_data.log_levels.registry.clone())),
                    "cache" => Some(serde_json::Value::String(self.config_data.log_levels.cache.clone())),
                    "performance" => Some(serde_json::Value::String(self.config_data.log_levels.performance.clone())),
                    _ => None,
                }
            },
            "cache_policies" => {
                match key {
                    "actor_snapshot" => Some(serde_json::to_value(&self.config_data.cache_policies.actor_snapshot)?),
                    "resource_regeneration" => Some(serde_json::to_value(&self.config_data.cache_policies.resource_regeneration)?),
                    "stat_aggregation" => Some(serde_json::to_value(&self.config_data.cache_policies.stat_aggregation)?),
                    _ => None,
                }
            },
            "system_ids" => {
                match key {
                    "list" => Some(serde_json::to_value(&self.config_data.system_ids)?),
                    _ => None,
                }
            },
            "context_types" => {
                match key {
                    "list" => Some(serde_json::to_value(&self.config_data.context_types)?),
                    _ => None,
                }
            },
            _ => None,
        };
        
        if let Some(value) = value {
            Ok(Some(ConfigurationValue {
                value,
                value_type: ConfigurationValueType::Object,
                source_provider: self.base.provider_id().to_string(),
                priority: self.base.priority(),
                timestamp: chrono::Utc::now(),
                can_override: true,
                can_merge: true,
            }))
        } else {
            Ok(None)
        }
    }
    
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<std::collections::HashMap<String, ConfigurationValue>> {
        let mut result = std::collections::HashMap::new();
        
        match category {
            "defaults" => {
                result.insert("resources".to_string(), self.get_config_value(category, "resources").await?.unwrap());
                result.insert("stats".to_string(), self.get_config_value(category, "stats").await?.unwrap());
                result.insert("elements".to_string(), self.get_config_value(category, "elements").await?.unwrap());
            },
            "timeouts" => {
                result.insert("cache_ttl".to_string(), self.get_config_value(category, "cache_ttl").await?.unwrap());
                result.insert("aggregation_timeout".to_string(), self.get_config_value(category, "aggregation_timeout").await?.unwrap());
                result.insert("validation_timeout".to_string(), self.get_config_value(category, "validation_timeout").await?.unwrap());
                result.insert("regeneration_interval".to_string(), self.get_config_value(category, "regeneration_interval").await?.unwrap());
                result.insert("subsystem_timeout".to_string(), self.get_config_value(category, "subsystem_timeout").await?.unwrap());
            },
            "performance_thresholds" => {
                result.insert("max_actors".to_string(), self.get_config_value(category, "max_actors").await?.unwrap());
                result.insert("max_contributions_per_actor".to_string(), self.get_config_value(category, "max_contributions_per_actor").await?.unwrap());
                result.insert("max_caps_per_actor".to_string(), self.get_config_value(category, "max_caps_per_actor").await?.unwrap());
                result.insert("max_subsystems".to_string(), self.get_config_value(category, "max_subsystems").await?.unwrap());
                result.insert("cache_size_mb".to_string(), self.get_config_value(category, "cache_size_mb").await?.unwrap());
                result.insert("memory_usage_mb".to_string(), self.get_config_value(category, "memory_usage_mb").await?.unwrap());
                result.insert("cpu_usage_percent".to_string(), self.get_config_value(category, "cpu_usage_percent").await?.unwrap());
            },
            "validation_rules" => {
                result.insert("resource_values".to_string(), self.get_config_value(category, "resource_values").await?.unwrap());
                result.insert("stat_values".to_string(), self.get_config_value(category, "stat_values").await?.unwrap());
                result.insert("element_affinities".to_string(), self.get_config_value(category, "element_affinities").await?.unwrap());
                result.insert("contribution_values".to_string(), self.get_config_value(category, "contribution_values").await?.unwrap());
            },
            "cache_keys" => {
                result.insert("actor_snapshot".to_string(), self.get_config_value(category, "actor_snapshot").await?.unwrap());
                result.insert("resource_regeneration".to_string(), self.get_config_value(category, "resource_regeneration").await?.unwrap());
                result.insert("stat_aggregation".to_string(), self.get_config_value(category, "stat_aggregation").await?.unwrap());
                result.insert("subsystem_data".to_string(), self.get_config_value(category, "subsystem_data").await?.unwrap());
            },
            "log_levels" => {
                result.insert("actor_core".to_string(), self.get_config_value(category, "actor_core").await?.unwrap());
                result.insert("config".to_string(), self.get_config_value(category, "config").await?.unwrap());
                result.insert("registry".to_string(), self.get_config_value(category, "registry").await?.unwrap());
                result.insert("cache".to_string(), self.get_config_value(category, "cache").await?.unwrap());
                result.insert("performance".to_string(), self.get_config_value(category, "performance").await?.unwrap());
            },
            "cache_policies" => {
                result.insert("actor_snapshot".to_string(), self.get_config_value(category, "actor_snapshot").await?.unwrap());
                result.insert("resource_regeneration".to_string(), self.get_config_value(category, "resource_regeneration").await?.unwrap());
                result.insert("stat_aggregation".to_string(), self.get_config_value(category, "stat_aggregation").await?.unwrap());
            },
            "system_ids" => {
                result.insert("list".to_string(), self.get_config_value(category, "list").await?.unwrap());
            },
            "context_types" => {
                result.insert("list".to_string(), self.get_config_value(category, "list").await?.unwrap());
            },
            _ => {}
        }
        
        Ok(result)
    }
    
    fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule> {
        // Try to load merge rule from configuration
        match self.load_merge_rule_from_config(category, key) {
            Ok(Some(rule)) => Some(rule),
            Ok(None) => {
                // Fallback to default rule
                Some(ConfigurationMergeRule {
                    strategy: ConfigurationMergeStrategy::Override,
                    use_pipeline: false,
                    default_value: None,
                    validation_rules: vec![],
                })
            },
            Err(e) => {
                tracing::warn!("Failed to load merge rule for {}:{}: {}. Using default.", category, key, e);
                Some(ConfigurationMergeRule {
                    strategy: ConfigurationMergeStrategy::Override,
                    use_pipeline: false,
                    default_value: None,
                    validation_rules: vec![],
                })
            }
        }
    }
    
    async fn validate_config(&self) -> ActorCoreResult<()> {
        // Validate resource defaults
        for (resource_name, resource_defaults) in &self.config_data.defaults.resources {
            if resource_defaults.base_value < resource_defaults.min_value {
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Resource {} base_value cannot be less than min_value", resource_name)
                ));
            }
            if resource_defaults.base_value > resource_defaults.max_value {
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Resource {} base_value cannot be greater than max_value", resource_name)
                ));
            }
        }
        
        // Validate stat defaults
        for (stat_name, stat_defaults) in &self.config_data.defaults.stats {
            if stat_defaults.base_value < stat_defaults.min_value {
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Stat {} base_value cannot be less than min_value", stat_name)
                ));
            }
            if stat_defaults.base_value > stat_defaults.max_value {
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Stat {} base_value cannot be greater than max_value", stat_name)
                ));
            }
        }
        
        // Validate element defaults
        for (element_name, element_defaults) in &self.config_data.defaults.elements {
            if element_defaults.base_affinity < element_defaults.min_affinity {
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Element {} base_affinity cannot be less than min_affinity", element_name)
                ));
            }
            if element_defaults.base_affinity > element_defaults.max_affinity {
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Element {} base_affinity cannot be greater than max_affinity", element_name)
                ));
            }
        }
        
        Ok(())
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
