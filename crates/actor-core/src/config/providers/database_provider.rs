//! Database configuration provider for the Configuration Hub system
//! 
//! This provider loads configuration from database tables and supports
//! dynamic configuration updates at runtime.

use async_trait::async_trait;
use std::collections::HashMap;
use tracing::info;

use crate::config::provider::{ConfigurationProvider, BaseConfigurationProvider};
use crate::config::types::*;
use crate::ActorCoreResult;
use crate::ActorCoreError;

/// Database configuration provider
pub struct DatabaseConfigurationProvider {
    base: BaseConfigurationProvider,
    config_data: HashMap<String, HashMap<String, ConfigurationValue>>,
    _database_config: DatabaseConfig,
}

/// Database configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub table_name: String,
    pub category_column: String,
    pub key_column: String,
    pub value_column: String,
    pub priority_column: String,
    pub enabled_column: String,
}

impl DatabaseConfigurationProvider {
    pub fn new(provider_id: String, priority: i64, database_config: DatabaseConfig) -> Self {
        Self {
            base: BaseConfigurationProvider::new(
                provider_id,
                priority,
                Vec::new(), // Will be populated after loading
            ),
            config_data: HashMap::new(),
            _database_config: database_config,
        }
    }

    /// Load database configuration from config file
    pub fn load_database_config(config_path: &str) -> ActorCoreResult<DatabaseConfig> {
        // Try to load from database_config.yaml first
        let db_config_path = std::path::Path::new(config_path)
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("database_config.yaml");
            
        if db_config_path.exists() {
            match Self::load_database_config_from_file(&db_config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    tracing::warn!("Failed to load database config from file: {}. Using fallback defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(DatabaseConfig {
            connection_string: "postgresql://localhost:5432/actor_core_config".to_string(),
            table_name: "configuration".to_string(),
            category_column: "category".to_string(),
            key_column: "key_name".to_string(),
            value_column: "value_data".to_string(),
            priority_column: "priority".to_string(),
            enabled_column: "is_enabled".to_string(),
        })
    }

    /// Load database configuration from file
    fn load_database_config_from_file(path: &std::path::Path) -> ActorCoreResult<DatabaseConfig> {
        let content = std::fs::read_to_string(path)?;
        let config: DatabaseConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration from database
    pub async fn load_from_database(&mut self) -> ActorCoreResult<()> {
        // Try to load from actual database first
        match self.load_from_actual_database().await {
            Ok(config_data) => {
                self.config_data = config_data;
                let supported_categories: Vec<String> = self.config_data.keys().cloned().collect();
                
                // Update the base provider with supported categories
                self.base = BaseConfigurationProvider::new(
                    self.base.provider_id().to_string(),
                    self.base.priority(),
                    supported_categories,
                );

                info!("Loaded database configuration with {} categories", self.config_data.len());
                return Ok(());
            },
            Err(e) => {
                tracing::warn!("Failed to load from actual database: {}. Using fallback configuration.", e);
            }
        }

        // Fallback to configuration file if database is not available
        self.load_from_fallback_config().await
    }

    /// Load configuration from actual database
    async fn load_from_actual_database(&self) -> ActorCoreResult<HashMap<String, HashMap<String, ConfigurationValue>>> {
        // TODO: Implement actual database connection and querying
        // This would use a database driver like sqlx or diesel
        // For now, return an error to trigger fallback
        Err(ActorCoreError::ConfigurationError(
            "Database connection not implemented yet".to_string()
        ))
    }

    /// Load fallback configuration from config files
    async fn load_from_fallback_config(&mut self) -> ActorCoreResult<()> {
        let mut config_data = HashMap::new();
        let mut supported_categories = Vec::new();

        // Load database configuration from config file
        let database_config = self.load_database_config_from_config_file().await?;
        config_data.insert("database".to_string(), database_config);
        supported_categories.push("database".to_string());

        // Load system configuration from config file
        let system_config = self.load_system_config_from_config_file().await?;
        config_data.insert("system".to_string(), system_config);
        supported_categories.push("system".to_string());

        self.config_data = config_data;
        
        // Update the base provider with supported categories
        self.base = BaseConfigurationProvider::new(
            self.base.provider_id().to_string(),
            self.base.priority(),
            supported_categories,
        );

        info!("Loaded fallback configuration with {} categories", self.config_data.len());
        Ok(())
    }

    /// Load database configuration from config file
    async fn load_database_config_from_config_file(&self) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        // TODO: Load from database_config.yaml
        // For now, return default values
        let mut config = HashMap::new();
        config.insert("connection_pool_size".to_string(), ConfigurationValue::new(
            serde_json::Value::Number(10.into()),
            ConfigurationValueType::Integer,
            self.base.provider_id().to_string(),
            self.base.priority(),
        ));
        config.insert("query_timeout".to_string(), ConfigurationValue::new(
            serde_json::Value::Number(30.into()),
            ConfigurationValueType::Integer,
            self.base.provider_id().to_string(),
            self.base.priority(),
        ));
        config.insert("enable_caching".to_string(), ConfigurationValue::new(
            serde_json::Value::Bool(true),
            ConfigurationValueType::Boolean,
            self.base.provider_id().to_string(),
            self.base.priority(),
        ));
        Ok(config)
    }

    /// Load system configuration from config file
    async fn load_system_config_from_config_file(&self) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        // TODO: Load from system_config.yaml
        // For now, return default values
        let mut config = HashMap::new();
        config.insert("max_actors".to_string(), ConfigurationValue::new(
            serde_json::Value::Number(10000.into()),
            ConfigurationValueType::Integer,
            self.base.provider_id().to_string(),
            self.base.priority(),
        ));
        config.insert("cache_ttl".to_string(), ConfigurationValue::new(
            serde_json::Value::Number(3600.into()),
            ConfigurationValueType::Integer,
            self.base.provider_id().to_string(),
            self.base.priority(),
        ));
        Ok(config)
    }

    /// Save configuration to database
    pub async fn save_to_database(&self, config: &ConfigurationValue) -> ActorCoreResult<()> {
        // Try to save to actual database first
        match self.save_to_actual_database(config).await {
            Ok(()) => {
                info!("Configuration saved to database successfully");
                return Ok(());
            },
            Err(e) => {
                tracing::warn!("Failed to save to actual database: {}. Configuration not saved.", e);
                return Err(e);
            }
        }
    }

    /// Save configuration to actual database
    async fn save_to_actual_database(&self, _config: &ConfigurationValue) -> ActorCoreResult<()> {
        // TODO: Implement actual database save operation
        // This would use a database driver like sqlx or diesel
        // For now, return an error to indicate not implemented
        Err(ActorCoreError::ConfigurationError(
            "Database save operation not implemented yet".to_string()
        ))
    }
}

#[async_trait]
impl ConfigurationProvider for DatabaseConfigurationProvider {
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
        // Load from database first, fallback to config data
        match self.load_category_from_database(category).await {
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
                // Fallback to default rules based on category
                match category {
                    "database" => Some(ConfigurationMergeRule {
                        strategy: ConfigurationMergeStrategy::Override, // Database configs typically override
                        use_pipeline: false,
                        default_value: None,
                        validation_rules: vec![],
                    }),
                    "system" => Some(ConfigurationMergeRule {
                        strategy: ConfigurationMergeStrategy::Override, // System configs typically override
                        use_pipeline: false,
                        default_value: None,
                        validation_rules: vec![],
                    }),
                    _ => None,
                }
            },
            Err(e) => {
                tracing::warn!("Failed to load merge rule for category {}: {}. Using default.", category, e);
                None
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
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl DatabaseConfigurationProvider {
    /// Load category configuration from database
    async fn load_category_from_database(&self, _category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        // TODO: Implement actual database query for specific category
        // For now, return an error to trigger fallback
        Err(ActorCoreError::ConfigurationError(
            "Database category loading not implemented yet".to_string()
        ))
    }

    /// Load merge rule from configuration
    fn load_merge_rule_from_config(&self, _category: &str) -> ActorCoreResult<Option<ConfigurationMergeRule>> {
        // TODO: Implement merge rule loading from merge_rules.yaml
        // This should check for category-specific rules
        // For now, return None to use fallback
        Ok(None)
    }
}
