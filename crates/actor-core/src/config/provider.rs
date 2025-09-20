//! Configuration provider trait for the Configuration Hub system

use async_trait::async_trait;
use std::collections::HashMap;
use crate::config::types::{ConfigurationValue, ConfigurationMergeRule};
use crate::ActorCoreResult;

/// Trait for subsystems to provide configuration data
#[async_trait]
pub trait ConfigurationProvider: Send + Sync {
    /// Unique identifier for this configuration provider
    fn provider_id(&self) -> &str;
    
    /// Priority for this provider (higher = more important)
    fn priority(&self) -> i64;
    
    /// Get configuration categories this provider supports
    fn get_supported_categories(&self) -> Vec<String>;
    
    /// Get configuration value for a specific key
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>>;
    
    /// Get all configuration values for a category
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>>;
    
    /// Get merge rule for a specific configuration key
    fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule>;
    
    /// Validate configuration data
    async fn validate_config(&self) -> ActorCoreResult<()>;
    
    /// Check if this provider supports a specific category
    fn supports_category(&self, category: &str) -> bool {
        self.get_supported_categories().contains(&category.to_string())
    }
    
    /// Get provider metadata
    fn get_metadata(&self) -> ConfigurationProviderMetadata {
        ConfigurationProviderMetadata {
            provider_id: self.provider_id().to_string(),
            priority: self.priority(),
            supported_categories: self.get_supported_categories(),
            provider_type: "base".to_string(), // Can be overridden by subclasses
            version: "1.0.0".to_string(), // Can be overridden by subclasses
            description: None, // Can be overridden by subclasses
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
        }
    }
}

/// Configuration provider metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigurationProviderMetadata {
    pub provider_id: String,
    pub priority: i64,
    pub supported_categories: Vec<String>,
    pub provider_type: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Provider configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProviderConfig {
    pub default_priority: i64,
    pub max_categories: usize,
    pub enable_metadata: bool,
    pub default_provider_type: String,
    pub default_version: String,
    pub validation_enabled: bool,
    pub cache_enabled: bool,
    pub cache_ttl_ms: u64,
}

/// Base implementation for configuration providers
pub struct BaseConfigurationProvider {
    provider_id: String,
    priority: i64,
    supported_categories: Vec<String>,
    provider_config: ProviderConfig,
}

impl BaseConfigurationProvider {
    pub fn new(provider_id: String, priority: i64, supported_categories: Vec<String>) -> Self {
        // Load provider configuration
        let provider_config = Self::load_provider_config().unwrap_or_else(|_| {
            tracing::warn!("Failed to load provider config, using defaults");
            Self::get_default_provider_config()
        });

        Self {
            provider_id,
            priority,
            supported_categories,
            provider_config,
        }
    }

    /// Load provider configuration from config file
    pub fn load_provider_config() -> ActorCoreResult<ProviderConfig> {
        // Try to load from provider_config.yaml first
        let provider_config_path = std::path::Path::new("configs/provider_config.yaml");
            
        if provider_config_path.exists() {
            match Self::load_provider_config_from_file(provider_config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    tracing::warn!("Failed to load provider config from file: {}. Using fallback defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_provider_config())
    }

    /// Load provider configuration from file
    fn load_provider_config_from_file(path: &std::path::Path) -> ActorCoreResult<ProviderConfig> {
        let content = std::fs::read_to_string(path)?;
        let config: ProviderConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default provider configuration
    fn get_default_provider_config() -> ProviderConfig {
        ProviderConfig {
            default_priority: 100,
            max_categories: 50,
            enable_metadata: true,
            default_provider_type: "base".to_string(),
            default_version: "1.0.0".to_string(),
            validation_enabled: true,
            cache_enabled: false,
            cache_ttl_ms: 300000, // 5 minutes
        }
    }
}

#[async_trait]
impl ConfigurationProvider for BaseConfigurationProvider {
    fn provider_id(&self) -> &str {
        &self.provider_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    fn get_supported_categories(&self) -> Vec<String> {
        self.supported_categories.clone()
    }
    
    async fn get_config_value(&self, _category: &str, _key: &str) -> ActorCoreResult<Option<ConfigurationValue>> {
        Ok(None)
    }
    
    async fn get_category_config(&self, _category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        Ok(HashMap::new())
    }
    
    fn get_merge_rule(&self, _category: &str, _key: &str) -> Option<ConfigurationMergeRule> {
        None
    }
    
    async fn validate_config(&self) -> ActorCoreResult<()> {
        // Check if validation is enabled
        if !self.provider_config.validation_enabled {
            return Ok(());
        }

        // Validate provider ID
        if self.provider_id.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Provider ID cannot be empty".to_string()
            ));
        }

        // Validate priority
        if self.priority < 0 {
            return Err(crate::ActorCoreError::ConfigurationError(
                format!("Provider priority must be non-negative, got: {}", self.priority)
            ));
        }

        // Validate categories count
        if self.supported_categories.len() > self.provider_config.max_categories {
            return Err(crate::ActorCoreError::ConfigurationError(
                format!("Too many categories: {} (max: {})", 
                    self.supported_categories.len(), 
                    self.provider_config.max_categories)
            ));
        }

        // Validate category names
        for category in &self.supported_categories {
            if category.is_empty() {
                return Err(crate::ActorCoreError::ConfigurationError(
                    "Category name cannot be empty".to_string()
                ));
            }
        }

        Ok(())
    }
}
