//! Environment variable configuration provider for the Configuration Hub system

use async_trait::async_trait;
use std::collections::HashMap;
use std::env;
use tracing::info;

use crate::config::provider::{ConfigurationProvider, BaseConfigurationProvider};
use crate::config::types::*;
use crate::ActorCoreResult;
use crate::ActorCoreError;

/// Environment variable configuration provider
pub struct EnvironmentConfigurationProvider {
    base: BaseConfigurationProvider,
    env_prefix: String,
    config_data: HashMap<String, HashMap<String, ConfigurationValue>>,
    env_config: EnvironmentConfig,
}

/// Environment configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnvironmentConfig {
    pub prefix: String,
    pub default_fallback_type: String,
    pub merge_rules: HashMap<String, EnvironmentMergeRule>,
    pub validation_rules: HashMap<String, EnvironmentValidationRule>,
}

/// Environment merge rule
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnvironmentMergeRule {
    pub strategy: String,
    pub use_pipeline: bool,
    pub default_value: Option<serde_json::Value>,
}

/// Environment validation rule
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnvironmentValidationRule {
    pub required: bool,
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl EnvironmentConfigurationProvider {
    pub fn new(provider_id: String, priority: i64, env_prefix: String) -> Self {
        // Load environment configuration
        let env_config = Self::load_environment_config(&env_prefix).unwrap_or_else(|_| {
            tracing::warn!("Failed to load environment config, using defaults");
            Self::get_default_environment_config(&env_prefix)
        });

        Self {
            base: BaseConfigurationProvider::new(
                provider_id,
                priority,
                Vec::new(), // Will be populated after loading
            ),
            env_prefix,
            config_data: HashMap::new(),
            env_config,
        }
    }

    /// Load environment configuration from config file
    pub fn load_environment_config(env_prefix: &str) -> ActorCoreResult<EnvironmentConfig> {
        // Try to load from environment_config.yaml first
        let env_config_path = std::path::Path::new("configs/environment_config.yaml");
            
        if env_config_path.exists() {
            match Self::load_environment_config_from_file(env_config_path) {
                Ok(mut config) => {
                    // Override prefix if not set in config
                    if config.prefix.is_empty() {
                        config.prefix = env_prefix.to_string();
                    }
                    return Ok(config);
                },
                Err(e) => {
                    tracing::warn!("Failed to load environment config from file: {}. Using fallback defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_environment_config(env_prefix))
    }

    /// Load environment configuration from file
    fn load_environment_config_from_file(path: &std::path::Path) -> ActorCoreResult<EnvironmentConfig> {
        let content = std::fs::read_to_string(path)?;
        let config: EnvironmentConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default environment configuration
    fn get_default_environment_config(env_prefix: &str) -> EnvironmentConfig {
        let mut merge_rules = HashMap::new();
        merge_rules.insert("default".to_string(), EnvironmentMergeRule {
            strategy: "Override".to_string(),
            use_pipeline: false,
            default_value: None,
        });

        let mut validation_rules = HashMap::new();
        validation_rules.insert("default".to_string(), EnvironmentValidationRule {
            required: false,
            pattern: None,
            min_length: None,
            max_length: None,
        });

        EnvironmentConfig {
            prefix: env_prefix.to_string(),
            default_fallback_type: "string".to_string(),
            merge_rules,
            validation_rules,
        }
    }

    /// Load configuration from environment variables
    pub fn load_from_environment(&mut self) -> ActorCoreResult<()> {
        let mut config_data = HashMap::new();
        let mut supported_categories = Vec::new();

        // Load all environment variables with the prefix
        for (key, value) in env::vars() {
            if key.starts_with(&self.env_prefix) {
                // Parse the key format: PREFIX_CATEGORY_KEY
                let parts: Vec<&str> = key.split('_').collect();
                if parts.len() >= 3 {
                    let category = parts[1].to_lowercase();
                    let config_key = parts[2..].join("_").to_lowercase();
                    
                    // Parse the value
                    let parsed_value = self.parse_environment_value(&value);
                    let value_type = self.determine_value_type(&parsed_value);
                    
                    let config_value = ConfigurationValue {
                        value: parsed_value,
                        value_type,
                        source_provider: self.base.provider_id().to_string(),
                        priority: self.base.priority(),
                        timestamp: chrono::Utc::now(),
                        can_override: true,
                        can_merge: true,
                    };
                    
                    config_data
                        .entry(category.clone())
                        .or_insert_with(HashMap::new)
                        .insert(config_key, config_value);
                    
                    if !supported_categories.contains(&category) {
                        supported_categories.push(category);
                    }
                }
            }
        }

        self.config_data = config_data;
        
        // Update the base provider with supported categories
        self.base = BaseConfigurationProvider::new(
            self.base.provider_id().to_string(),
            self.base.priority(),
            supported_categories,
        );

        info!("Loaded {} environment variables with prefix: {}", 
              self.config_data.values().map(|v| v.len()).sum::<usize>(), 
              self.env_prefix);
        
        Ok(())
    }

    /// Parse environment variable value
    fn parse_environment_value(&self, value: &str) -> serde_json::Value {
        // Try to parse as number
        if let Ok(int_val) = value.parse::<i64>() {
            return serde_json::Value::Number(int_val.into());
        }
        if let Ok(float_val) = value.parse::<f64>() {
            return serde_json::Value::Number(serde_json::Number::from_f64(float_val).unwrap_or(serde_json::Number::from(0)));
        }
        
        // Try to parse as boolean
        if let Ok(bool_val) = value.parse::<bool>() {
            return serde_json::Value::Bool(bool_val);
        }
        
        // Try to parse as JSON array or object
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(value) {
            return json_val;
        }
        
        // Use configured default fallback type
        match self.env_config.default_fallback_type.as_str() {
            "number" => {
                if let Ok(num) = value.parse::<f64>() {
                    serde_json::Value::Number(serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from(0)))
                } else {
                    serde_json::Value::Number(serde_json::Number::from(0))
                }
            },
            "boolean" => {
                if let Ok(bool_val) = value.parse::<bool>() {
                    serde_json::Value::Bool(bool_val)
                } else {
                    serde_json::Value::Bool(false)
                }
            },
            "array" => {
                serde_json::Value::Array(vec![serde_json::Value::String(value.to_string())])
            },
            "object" => {
                let mut obj = serde_json::Map::new();
                obj.insert("value".to_string(), serde_json::Value::String(value.to_string()));
                serde_json::Value::Object(obj)
            },
            _ => serde_json::Value::String(value.to_string()), // Default to string
        }
    }

    /// Determine value type from serde_json::Value
    fn determine_value_type(&self, value: &serde_json::Value) -> ConfigurationValueType {
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
impl ConfigurationProvider for EnvironmentConfigurationProvider {
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
        // Load from environment first, fallback to config data
        match self.load_category_from_environment(category).await {
            Ok(config) => Ok(config),
            Err(_) => {
                // Fallback to in-memory config data
                Ok(self.config_data.get(category).cloned().unwrap_or_default())
            }
        }
    }

    fn get_merge_rule(&self, category: &str, _key: &str) -> Option<ConfigurationMergeRule> {
        // Try to load merge rule from configuration
        match self.load_merge_rule_from_config(category) {
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
                tracing::warn!("Failed to load merge rule for category {}: {}. Using default.", category, e);
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
        for (category, properties) in &self.config_data {
            if category.is_empty() {
                return Err(crate::ActorCoreError::ConfigurationError(
                    "Category name cannot be empty".to_string()
                ));
            }
            
            for (key, value) in properties.iter() {
                if key.is_empty() {
                    return Err(crate::ActorCoreError::ConfigurationError(
                        format!("Key name cannot be empty in category: {}", category)
                    ));
                }
                
                if value.priority < 0 {
                    return Err(crate::ActorCoreError::ConfigurationError(
                        format!("Priority must be non-negative for key {} in category {}", key, category)
                    ));
                }
            }
        }
        
        Ok(())
    }
}

impl EnvironmentConfigurationProvider {
    /// Load category configuration from environment
    async fn load_category_from_environment(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        let mut config = HashMap::new();
        let category_prefix = format!("{}_{}_", self.env_prefix, category.to_uppercase());

        for (key, value) in env::vars() {
            if key.starts_with(&category_prefix) {
                let config_key = key.strip_prefix(&category_prefix)
                    .unwrap_or(&key)
                    .to_lowercase();
                
                let parsed_value = self.parse_environment_value(&value);
                let value_type = self.determine_value_type(&parsed_value);
                
                let config_value = ConfigurationValue {
                    value: parsed_value,
                    value_type,
                    source_provider: self.base.provider_id().to_string(),
                    priority: self.base.priority(),
                    timestamp: chrono::Utc::now(),
                    can_override: true,
                    can_merge: true,
                };
                
                config.insert(config_key, config_value);
            }
        }

        Ok(config)
    }

    /// Load merge rule from configuration
    fn load_merge_rule_from_config(&self, category: &str) -> ActorCoreResult<Option<ConfigurationMergeRule>> {
        // Check for category-specific rule first
        if let Some(rule) = self.env_config.merge_rules.get(category) {
            let strategy = match rule.strategy.as_str() {
                "Override" => ConfigurationMergeStrategy::Override,
                "Sum" => ConfigurationMergeStrategy::Sum,
                "Concat" => ConfigurationMergeStrategy::Concat,
                "Min" => ConfigurationMergeStrategy::Min,
                "Max" => ConfigurationMergeStrategy::Max,
                _ => ConfigurationMergeStrategy::Override,
            };

            return Ok(Some(ConfigurationMergeRule {
                strategy,
                use_pipeline: rule.use_pipeline,
                default_value: rule.default_value.clone(),
                validation_rules: vec![],
            }));
        }

        // Check for default rule
        if let Some(rule) = self.env_config.merge_rules.get("default") {
            let strategy = match rule.strategy.as_str() {
                "Override" => ConfigurationMergeStrategy::Override,
                "Sum" => ConfigurationMergeStrategy::Sum,
                "Concat" => ConfigurationMergeStrategy::Concat,
                "Min" => ConfigurationMergeStrategy::Min,
                "Max" => ConfigurationMergeStrategy::Max,
                _ => ConfigurationMergeStrategy::Override,
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
