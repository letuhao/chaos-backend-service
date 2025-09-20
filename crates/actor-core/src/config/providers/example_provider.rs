//! Example configuration provider for testing and demonstration

use async_trait::async_trait;
use std::collections::HashMap;
// use serde_json::Value; // Unused import

use crate::config::provider::{ConfigurationProvider, BaseConfigurationProvider};
use crate::config::types::*;
use crate::ActorCoreResult;
use crate::ActorCoreError;

/// Example configuration provider for testing
pub struct ExampleConfigurationProvider {
    base: BaseConfigurationProvider,
    config_data: HashMap<String, HashMap<String, ConfigurationValue>>,
    example_config: ExampleConfig,
}

/// Example configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExampleConfig {
    pub provider_id: String,
    pub priority: i64,
    pub supported_categories: Vec<String>,
    pub default_values: HashMap<String, HashMap<String, serde_json::Value>>,
    pub merge_rules: HashMap<String, ExampleMergeRule>,
    pub validation_rules: HashMap<String, ExampleValidationRule>,
}

/// Example merge rule
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExampleMergeRule {
    pub strategy: String,
    pub use_pipeline: bool,
    pub default_value: Option<serde_json::Value>,
}

/// Example validation rule
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExampleValidationRule {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub required: bool,
}

impl ExampleConfigurationProvider {
    pub fn new() -> Self {
        // Load example configuration
        let example_config = Self::load_example_config().unwrap_or_else(|_| {
            tracing::warn!("Failed to load example config, using defaults");
            Self::get_default_example_config()
        });

        let mut config_data = HashMap::new();
        
        // Load configuration data from config
        for (category, values) in &example_config.default_values {
            let mut category_data = HashMap::new();
            for (key, value) in values {
                let value_type = Self::determine_value_type(value);
                let config_value = ConfigurationValue {
                    value: value.clone(),
                    value_type,
                    source_provider: example_config.provider_id.clone(),
                    priority: example_config.priority,
                    timestamp: chrono::Utc::now(),
                    can_override: true,
                    can_merge: true,
                };
                category_data.insert(key.clone(), config_value);
            }
            config_data.insert(category.clone(), category_data);
        }
        
        Self {
            base: BaseConfigurationProvider::new(
                example_config.provider_id.clone(),
                example_config.priority,
                example_config.supported_categories.clone(),
            ),
            config_data,
            example_config,
        }
    }

    /// Load example configuration from config file
    pub fn load_example_config() -> ActorCoreResult<ExampleConfig> {
        // Try to load from example_config.yaml first
        let example_config_path = std::path::Path::new("configs/example_config.yaml");
            
        if example_config_path.exists() {
            match Self::load_example_config_from_file(example_config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    tracing::warn!("Failed to load example config from file: {}. Using fallback defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_example_config())
    }

    /// Load example configuration from file
    fn load_example_config_from_file(path: &std::path::Path) -> ActorCoreResult<ExampleConfig> {
        let content = std::fs::read_to_string(path)?;
        let config: ExampleConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default example configuration
    fn get_default_example_config() -> ExampleConfig {
        let mut default_values = HashMap::new();
        
        // Element affinities defaults
        let mut element_affinities = HashMap::new();
        element_affinities.insert("fire_affinity".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap_or(serde_json::Number::from(0))));
        element_affinities.insert("water_affinity".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap_or(serde_json::Number::from(0))));
        default_values.insert("element_affinities".to_string(), element_affinities);
        
        // Primary stats defaults
        let mut primary_stats = HashMap::new();
        primary_stats.insert("strength".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(10.0).unwrap_or(serde_json::Number::from(10))));
        primary_stats.insert("agility".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(10.0).unwrap_or(serde_json::Number::from(10))));
        default_values.insert("primary_stats".to_string(), primary_stats);

        let mut merge_rules = HashMap::new();
        merge_rules.insert("element_affinities".to_string(), ExampleMergeRule {
            strategy: "Sum".to_string(),
            use_pipeline: true,
            default_value: Some(serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap_or(serde_json::Number::from(0)))),
        });
        merge_rules.insert("primary_stats".to_string(), ExampleMergeRule {
            strategy: "Sum".to_string(),
            use_pipeline: true,
            default_value: Some(serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap_or(serde_json::Number::from(0)))),
        });

        let mut validation_rules = HashMap::new();
        validation_rules.insert("element_affinities".to_string(), ExampleValidationRule {
            min_value: Some(0.0),
            max_value: Some(1.0),
            required: false,
        });
        validation_rules.insert("primary_stats".to_string(), ExampleValidationRule {
            min_value: Some(0.0),
            max_value: None,
            required: false,
        });

        ExampleConfig {
            provider_id: "example_provider".to_string(),
            priority: 100,
            supported_categories: vec!["element_affinities".to_string(), "primary_stats".to_string()],
            default_values,
            merge_rules,
            validation_rules,
        }
    }

    /// Determine value type from serde_json::Value
    fn determine_value_type(value: &serde_json::Value) -> ConfigurationValueType {
        match value {
            serde_json::Value::String(_) => ConfigurationValueType::String,
            serde_json::Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    ConfigurationValueType::Integer
                } else {
                    ConfigurationValueType::Float
                }
            },
            serde_json::Value::Bool(_) => ConfigurationValueType::Boolean,
            serde_json::Value::Array(_) => ConfigurationValueType::Array,
            serde_json::Value::Object(_) => ConfigurationValueType::Object,
            serde_json::Value::Null => ConfigurationValueType::String,
        }
    }
}

#[async_trait]
impl ConfigurationProvider for ExampleConfigurationProvider {
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
        Ok(self.config_data.get(category).ok_or(ActorCoreError::ConfigurationError("Category not found".to_string()))?.get(key).cloned())
    }
    
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        // Load from config data first, fallback to default values
        if let Some(config) = self.config_data.get(category) {
            return Ok(config.clone());
        }

        // Load default values from configuration
        if let Some(default_values) = self.example_config.default_values.get(category) {
            let mut category_data = HashMap::new();
            for (key, value) in default_values {
                let value_type = Self::determine_value_type(value);
                let config_value = ConfigurationValue {
                    value: value.clone(),
                    value_type,
                    source_provider: self.example_config.provider_id.clone(),
                    priority: self.example_config.priority,
                    timestamp: chrono::Utc::now(),
                    can_override: true,
                    can_merge: true,
                };
                category_data.insert(key.clone(), config_value);
            }
            return Ok(category_data);
        }

        Ok(HashMap::new())
    }
    
    fn get_merge_rule(&self, category: &str, _key: &str) -> Option<ConfigurationMergeRule> {
        // Try to load merge rule from configuration
        match self.load_merge_rule_from_config(category) {
            Ok(Some(rule)) => Some(rule),
            Ok(None) => {
                // Fallback to default rule
                Some(ConfigurationMergeRule {
                    strategy: ConfigurationMergeStrategy::Sum,
                    use_pipeline: true,
                    default_value: Some(serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap_or(serde_json::Number::from(0)))),
                    validation_rules: vec![],
                })
            },
            Err(e) => {
                tracing::warn!("Failed to load merge rule for category {}: {}. Using default.", category, e);
                Some(ConfigurationMergeRule {
                    strategy: ConfigurationMergeStrategy::Sum,
                    use_pipeline: true,
                    default_value: Some(serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap_or(serde_json::Number::from(0)))),
                    validation_rules: vec![],
                })
            }
        }
    }
    
    async fn validate_config(&self) -> ActorCoreResult<()> {
        // Validate each category using configured validation rules
        for (category, properties) in &self.config_data {
            if let Some(validation_rule) = self.example_config.validation_rules.get(category) {
                for (key, value) in properties {
                    if let Some(num_value) = value.value.as_f64() {
                        // Check min value
                        if let Some(min_val) = validation_rule.min_value {
                            if num_value < min_val {
                                return Err(crate::ActorCoreError::ConfigurationError(
                                    format!("{} {} must be >= {}, got {}", category, key, min_val, num_value)
                                ));
                            }
                        }
                        
                        // Check max value
                        if let Some(max_val) = validation_rule.max_value {
                            if num_value > max_val {
                                return Err(crate::ActorCoreError::ConfigurationError(
                                    format!("{} {} must be <= {}, got {}", category, key, max_val, num_value)
                                ));
                            }
                        }
                    }
                    
                    // Check required
                    if validation_rule.required && value.value.is_null() {
                        return Err(crate::ActorCoreError::ConfigurationError(
                            format!("{} {} is required but not provided", category, key)
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ExampleConfigurationProvider {
    /// Load merge rule from configuration
    fn load_merge_rule_from_config(&self, category: &str) -> ActorCoreResult<Option<ConfigurationMergeRule>> {
        if let Some(rule) = self.example_config.merge_rules.get(category) {
            let strategy = match rule.strategy.as_str() {
                "Override" => ConfigurationMergeStrategy::Override,
                "Sum" => ConfigurationMergeStrategy::Sum,
                "Concat" => ConfigurationMergeStrategy::Concat,
                "Min" => ConfigurationMergeStrategy::Min,
                "Max" => ConfigurationMergeStrategy::Max,
                _ => ConfigurationMergeStrategy::Sum,
            };

            return Ok(Some(ConfigurationMergeRule {
                strategy,
                use_pipeline: rule.use_pipeline,
                default_value: rule.default_value.clone(),
                validation_rules: vec![],
            }));
        }

        Ok(None)
    }
}
