//! File-based configuration provider for the Configuration Hub system

use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;
use serde_yaml;
use tokio::fs;
use tracing::info;

use crate::config::provider::{ConfigurationProvider, BaseConfigurationProvider};
use crate::config::types::*;
use crate::ActorCoreResult;
use crate::ActorCoreError;

/// File-based configuration provider
pub struct FileConfigurationProvider {
    base: BaseConfigurationProvider,
    config_path: PathBuf,
    config_data: Arc<RwLock<HashMap<String, HashMap<String, ConfigurationValue>>>>,
    file_config: FileConfig,
}

/// File configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileConfig {
    pub provider_id: String,
    pub priority: i64,
    pub default_fallback_type: String,
    pub merge_rules: HashMap<String, FileMergeRule>,
    pub validation_rules: HashMap<String, FileValidationRule>,
    pub supported_file_types: Vec<String>,
    pub auto_reload: bool,
    pub watch_interval_ms: u64,
}

/// File merge rule
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileMergeRule {
    pub strategy: String,
    pub use_pipeline: bool,
    pub default_value: Option<serde_json::Value>,
}

/// File validation rule
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileValidationRule {
    pub required: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
}

impl FileConfigurationProvider {
    pub fn new(provider_id: String, priority: i64, config_path: PathBuf) -> Self {
        // Load file configuration
        let file_config = Self::load_file_config(&config_path).unwrap_or_else(|_| {
            tracing::warn!("Failed to load file config, using defaults");
            Self::get_default_file_config(&provider_id, priority)
        });

        Self {
            base: BaseConfigurationProvider::new(
                file_config.provider_id.clone(),
                file_config.priority,
                Vec::new(), // Will be populated after loading
            ),
            config_path,
            config_data: Arc::new(RwLock::new(HashMap::new())),
            file_config,
        }
    }

    /// Load file configuration from config file
    pub fn load_file_config(config_path: &PathBuf) -> ActorCoreResult<FileConfig> {
        // Try to load from file_config.yaml first
        let file_config_path = config_path.parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("file_config.yaml");
            
        if file_config_path.exists() {
            match Self::load_file_config_from_file(&file_config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    tracing::warn!("Failed to load file config from file: {}. Using fallback defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_file_config("file_provider", 200))
    }

    /// Load file configuration from file
    fn load_file_config_from_file(path: &std::path::Path) -> ActorCoreResult<FileConfig> {
        let content = std::fs::read_to_string(path)?;
        let config: FileConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default file configuration
    fn get_default_file_config(provider_id: &str, priority: i64) -> FileConfig {
        let mut merge_rules = HashMap::new();
        merge_rules.insert("default".to_string(), FileMergeRule {
            strategy: "Override".to_string(),
            use_pipeline: false,
            default_value: None,
        });

        let mut validation_rules = HashMap::new();
        validation_rules.insert("default".to_string(), FileValidationRule {
            required: false,
            min_length: None,
            max_length: None,
            pattern: None,
        });

        FileConfig {
            provider_id: provider_id.to_string(),
            priority,
            default_fallback_type: "string".to_string(),
            merge_rules,
            validation_rules,
            supported_file_types: vec!["yaml".to_string(), "json".to_string()],
            auto_reload: false,
            watch_interval_ms: 1000,
        }
    }

    /// Load configuration from file
    pub async fn load_from_file(&self) -> ActorCoreResult<()> {
        if !self.config_path.exists() {
            return Err(crate::ActorCoreError::ConfigurationError(
                format!("Configuration file not found: {:?}", self.config_path)
            ));
        }

        let content = fs::read_to_string(&self.config_path).await?;
        let config: serde_yaml::Value = serde_yaml::from_str(&content)?;

        let mut config_data = self.config_data.write();
        config_data.clear();

        if let Some(categories) = config.get("categories").and_then(|v| v.as_mapping()) {
            for (category_key, category_value) in categories {
                let category_name = category_key.as_str().unwrap_or("unknown");
                let mut category_data = HashMap::new();

                if let Some(properties) = category_value.as_mapping() {
                    for (key, value) in properties {
                        let key_name = key.as_str().unwrap_or("unknown");
                        let config_value = ConfigurationValue {
                            value: serde_json::to_value(value)?,
                            value_type: self.determine_value_type(&serde_json::to_value(value)?),
                            source_provider: self.base.provider_id().to_string(),
                            priority: self.base.priority(),
                            timestamp: chrono::Utc::now(),
                            can_override: true,
                            can_merge: true,
                        };
                        category_data.insert(key_name.to_string(), config_value);
                    }
                }

                config_data.insert(category_name.to_string(), category_data);
            }
        }

        info!("Loaded configuration from file: {:?}", self.config_path);
        Ok(())
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
            serde_json::Value::Null => {
                // Use configured default fallback type
                match self.file_config.default_fallback_type.as_str() {
                    "number" => ConfigurationValueType::Integer,
                    "float" => ConfigurationValueType::Float,
                    "boolean" => ConfigurationValueType::Boolean,
                    "array" => ConfigurationValueType::Array,
                    "object" => ConfigurationValueType::Object,
                    _ => ConfigurationValueType::String, // Default to string
                }
            }
        }
    }
}

#[async_trait]
impl ConfigurationProvider for FileConfigurationProvider {
    fn provider_id(&self) -> &str {
        self.base.provider_id()
    }
    
    fn priority(&self) -> i64 {
        self.base.priority()
    }
    
    fn get_supported_categories(&self) -> Vec<String> {
        let config_data = self.config_data.read();
        config_data.keys().cloned().collect()
    }
    
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>> {
        let config_data = self.config_data.read();
        Ok(config_data.get(category).ok_or(ActorCoreError::ConfigurationError("Category not found".to_string()))?.get(key).cloned())
    }
    
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        let config_data = self.config_data.read();
        
        // Load from config data first
        if let Some(config) = config_data.get(category) {
            return Ok(config.clone());
        }

        // Load default values from configuration if available
        if let Some(validation_rule) = self.file_config.validation_rules.get(category) {
            if validation_rule.required {
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Category {} is required but not found in file", category)
                ));
            }
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
        let config_data = self.config_data.read();
        
        for (category, properties) in config_data.iter() {
            if category.is_empty() {
                return Err(crate::ActorCoreError::ConfigurationError(
                    "Category name cannot be empty".to_string()
                ));
            }
            
            // Apply category-specific validation rules
            if let Some(validation_rule) = self.file_config.validation_rules.get(category) {
                for (key, value) in properties.iter() {
                    if key.is_empty() {
                        return Err(crate::ActorCoreError::ConfigurationError(
                            format!("Key name cannot be empty in category: {}", category)
                        ));
                    }
                    
                    // Check required
                    if validation_rule.required && value.value.is_null() {
                        return Err(crate::ActorCoreError::ConfigurationError(
                            format!("Key {} in category {} is required but not provided", key, category)
                        ));
                    }
                    
                    // Check min/max length for strings
                    if let Some(str_value) = value.value.as_str() {
                        if let Some(min_len) = validation_rule.min_length {
                            if str_value.len() < min_len {
                                return Err(crate::ActorCoreError::ConfigurationError(
                                    format!("Key {} in category {} must be at least {} characters long", key, category, min_len)
                                ));
                            }
                        }
                        
                        if let Some(max_len) = validation_rule.max_length {
                            if str_value.len() > max_len {
                                return Err(crate::ActorCoreError::ConfigurationError(
                                    format!("Key {} in category {} must be at most {} characters long", key, category, max_len)
                                ));
                            }
                        }
                        
                        // Check pattern if provided (simple string matching for now)
                        if let Some(pattern) = &validation_rule.pattern {
                            // Simple pattern matching - can be enhanced with regex later
                            if pattern == "^[a-z_]+$" {
                                if !str_value.chars().all(|c| c.is_lowercase() || c == '_') {
                                    return Err(crate::ActorCoreError::ConfigurationError(
                                        format!("Key {} in category {} must contain only lowercase letters and underscores", key, category)
                                    ));
                                }
                            } else if pattern == "^[a-zA-Z0-9_+\\-*/()\\s]+$" {
                                if !str_value.chars().all(|c| c.is_alphanumeric() || "+-*/() ".contains(c)) {
                                    return Err(crate::ActorCoreError::ConfigurationError(
                                        format!("Key {} in category {} must contain only alphanumeric characters and formula symbols", key, category)
                                    ));
                                }
                            }
                        }
                    }
                    
                    if value.priority < 0 {
                        return Err(crate::ActorCoreError::ConfigurationError(
                            format!("Priority must be non-negative for key {} in category {}", key, category)
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
}

impl FileConfigurationProvider {
    /// Load merge rule from configuration
    fn load_merge_rule_from_config(&self, category: &str) -> ActorCoreResult<Option<ConfigurationMergeRule>> {
        if let Some(rule) = self.file_config.merge_rules.get(category) {
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
        if let Some(rule) = self.file_config.merge_rules.get("default") {
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
