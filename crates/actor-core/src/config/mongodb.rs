//! MongoDB configuration provider for Actor Core
//!
//! This module provides functionality to load and save configuration
//! from MongoDB database, with fallback to file-based configuration.

// use std::sync::Arc; // Unused import
use serde::{Deserialize, Serialize};
#[cfg(feature = "mongodb-storage")]
use mongodb::{
    Client, Database, Collection,
    bson::doc,
    options::{ClientOptions, FindOptions, ReplaceOptions},
};

#[cfg(feature = "mongodb-storage")]
use async_trait::async_trait;
#[cfg(feature = "mongodb-storage")]
use std::collections::HashMap;
#[cfg(feature = "mongodb-storage")]
use tracing::{info, warn};

#[cfg(feature = "mongodb-storage")]
use crate::config::provider::{ConfigurationProvider, BaseConfigurationProvider};
#[cfg(feature = "mongodb-storage")]
use crate::config::types::*;
#[cfg(feature = "mongodb-storage")]
use crate::ActorCoreResult;
#[cfg(feature = "mongodb-storage")]
use crate::ActorCoreError;

#[cfg(feature = "mongodb-storage")]
/// MongoDB configuration provider
pub struct MongoDBConfigurationProvider {
    base: BaseConfigurationProvider,
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    database: Database,
    collection: Collection<ConfigurationDocument>,
    config_data: HashMap<String, HashMap<String, ConfigurationValue>>,
    mongodb_config: MongoDBConfig,
}

/// MongoDB configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDBConfig {
    pub connection_string: String,
    pub database_name: String,
    pub collection_name: String,
    pub enable_auto_sync: bool,
    pub sync_interval_seconds: u64,
    pub enable_fallback_to_file: bool,
    pub fallback_file_path: String,
}

/// Configuration document stored in MongoDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationDocument {
    #[serde(rename = "_id")]
    pub id: String,
    pub category: String,
    pub key: String,
    pub value: serde_json::Value,
    pub value_type: String,
    pub source_provider: String,
    pub priority: i64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub can_override: bool,
    pub can_merge: bool,
    pub version: u32,
}

#[cfg(feature = "mongodb-storage")]
impl MongoDBConfigurationProvider {
    /// Create a new MongoDB configuration provider
    pub async fn new(provider_id: String, priority: i64, mongodb_config: MongoDBConfig) -> ActorCoreResult<Self> {
        // Connect to MongoDB
        let client_options = ClientOptions::parse(&mongodb_config.connection_string).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(&mongodb_config.database_name);
        let collection = database.collection::<ConfigurationDocument>(&mongodb_config.collection_name);

        // Test connection
        database.run_command(mongodb::bson::doc! {"ping": 1}, None).await?;

        let mut provider = Self {
            base: BaseConfigurationProvider::new(
                provider_id,
                priority,
                Vec::new(), // Will be populated after loading
            ),
            client,
            database,
            collection,
            config_data: HashMap::new(),
            mongodb_config,
        };

        // Load initial configuration
        provider.load_from_database().await?;

        Ok(provider)
    }

    /// Load configuration from MongoDB
    pub async fn load_from_database(&mut self) -> ActorCoreResult<()> {
        info!("Loading configuration from MongoDB...");

        let mut config_data = HashMap::new();
        let mut supported_categories = Vec::new();

        // Find all configuration documents
        let find_options = FindOptions::builder()
            .sort(mongodb::bson::doc! {"category": 1, "key": 1})
            .build();

        let mut cursor = self.collection.find(None, find_options).await?;

        while cursor.advance().await? {
            let doc = cursor.deserialize_current()?;
            
            let config_value = ConfigurationValue {
                value: doc.value,
                value_type: match doc.value_type.as_str() {
                    "string" => ConfigurationValueType::String,
                    "integer" => ConfigurationValueType::Integer,
                    "float" => ConfigurationValueType::Float,
                    "number" => ConfigurationValueType::Number,
                    "boolean" => ConfigurationValueType::Boolean,
                    "array" => ConfigurationValueType::Array,
                    "object" => ConfigurationValueType::Object,
                    "duration" => ConfigurationValueType::Duration,
                    "size" => ConfigurationValueType::Size,
                    "percentage" => ConfigurationValueType::Percentage,
                    _ => ConfigurationValueType::String,
                },
                source_provider: doc.source_provider,
                priority: doc.priority,
                timestamp: doc.timestamp,
                can_override: doc.can_override,
                can_merge: doc.can_merge,
            };

            config_data
                .entry(doc.category.clone())
                .or_insert_with(HashMap::new)
                .insert(doc.key.clone(), config_value);

            if !supported_categories.contains(&doc.category) {
                supported_categories.push(doc.category);
            }
        }

        self.config_data = config_data;

        // Update the base provider with supported categories
        self.base = BaseConfigurationProvider::new(
            self.base.provider_id().to_string(),
            self.base.priority(),
            supported_categories,
        );

        info!("Loaded {} configuration categories from MongoDB", self.config_data.len());
        Ok(())
    }

    /// Save configuration to MongoDB
    pub async fn save_to_database(&self, category: &str, key: &str, value: &ConfigurationValue) -> ActorCoreResult<()> {
        let doc_id = format!("{}:{}", category, key);
        
        let config_doc = ConfigurationDocument {
            id: doc_id,
            category: category.to_string(),
            key: key.to_string(),
            value: value.value.clone(),
            value_type: match value.value_type {
                ConfigurationValueType::String => "string".to_string(),
                ConfigurationValueType::Integer => "integer".to_string(),
                ConfigurationValueType::Float => "float".to_string(),
                ConfigurationValueType::Number => "number".to_string(),
                ConfigurationValueType::Boolean => "boolean".to_string(),
                ConfigurationValueType::Array => "array".to_string(),
                ConfigurationValueType::Object => "object".to_string(),
                ConfigurationValueType::Duration => "duration".to_string(),
                ConfigurationValueType::Size => "size".to_string(),
                ConfigurationValueType::Percentage => "percentage".to_string(),
            },
            source_provider: value.source_provider.clone(),
            priority: value.priority,
            timestamp: value.timestamp,
            can_override: value.can_override,
            can_merge: value.can_merge,
            version: 1, // TODO: Implement versioning
        };

        let filter = mongodb::bson::doc! {"_id": &config_doc.id};
        let options = ReplaceOptions::builder()
            .upsert(true)
            .build();

        self.collection.replace_one(filter, &config_doc, options).await?;
        
        info!("Saved configuration {}:{} to MongoDB", category, key);
        Ok(())
    }

    /// Sync configuration from files to MongoDB
    pub async fn sync_from_files(&mut self) -> ActorCoreResult<()> {
        if !self.mongodb_config.enable_auto_sync {
            info!("Auto-sync disabled, skipping file sync");
            return Ok(());
        }

        info!("Syncing configuration from files to MongoDB...");

        // Load configuration from files
        let file_config = self.load_from_files().await?;
        
        // Save to MongoDB
        for (category, properties) in file_config {
            for (key, value) in properties {
                self.save_to_database(&category, &key, &value).await?;
            }
        }

        info!("Configuration sync from files to MongoDB completed");
        Ok(())
    }

    /// Load configuration from files (fallback)
    async fn load_from_files(&self) -> ActorCoreResult<HashMap<String, HashMap<String, ConfigurationValue>>> {
        if !self.mongodb_config.enable_fallback_to_file {
            return Ok(HashMap::new());
        }

        // TODO: Implement file loading logic
        // This should load from the fallback_file_path
        warn!("File loading not implemented yet, using empty config");
        Ok(HashMap::new())
    }

    /// Get MongoDB configuration
    pub fn load_mongodb_config(config_path: &str) -> ActorCoreResult<MongoDBConfig> {
        // Try to load from mongodb_config.yaml first
        let mongodb_config_path = std::path::Path::new(config_path)
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("mongodb_config.yaml");
            
        if mongodb_config_path.exists() {
            match Self::load_mongodb_config_from_file(&mongodb_config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    warn!("Failed to load MongoDB config from file: {}. Using fallback defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_mongodb_config())
    }

    /// Load MongoDB configuration from file
    fn load_mongodb_config_from_file(path: &std::path::Path) -> ActorCoreResult<MongoDBConfig> {
        let content = std::fs::read_to_string(path)?;
        let config: MongoDBConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default MongoDB configuration
    fn get_default_mongodb_config() -> MongoDBConfig {
        MongoDBConfig {
            connection_string: "mongodb://localhost:27017".to_string(),
            database_name: "actor_core_config".to_string(),
            collection_name: "configuration".to_string(),
            enable_auto_sync: true,
            sync_interval_seconds: 300, // 5 minutes
            enable_fallback_to_file: true,
            fallback_file_path: "configs/".to_string(),
        }
    }
}

#[cfg(feature = "mongodb-storage")]
#[async_trait]
impl ConfigurationProvider for MongoDBConfigurationProvider {
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
        if let Some(category_data) = self.config_data.get(category) {
            Ok(category_data.get(key).cloned())
        } else {
            Ok(None)
        }
    }

    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        Ok(self.config_data.get(category).cloned().unwrap_or_default())
    }

    fn get_merge_rule(&self, _category: &str, _key: &str) -> Option<ConfigurationMergeRule> {
        // MongoDB configs typically override
        Some(ConfigurationMergeRule {
            strategy: ConfigurationMergeStrategy::Override,
            use_pipeline: false,
            default_value: None,
            validation_rules: vec![],
        })
    }

    async fn validate_config(&self) -> ActorCoreResult<()> {
        for (category, properties) in &self.config_data {
            if category.is_empty() {
                return Err(ActorCoreError::ConfigurationError(
                    "Category name cannot be empty".to_string()
                ));
            }
            
            for (key, value) in properties.iter() {
                if key.is_empty() {
                    return Err(ActorCoreError::ConfigurationError(
                        format!("Key name cannot be empty in category: {}", category)
                    ));
                }
                
                if value.priority < 0 {
                    return Err(ActorCoreError::ConfigurationError(
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
