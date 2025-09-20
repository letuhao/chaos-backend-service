//! Configuration registry implementation for the Configuration Hub system

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, warn};

use crate::config::provider::{ConfigurationProvider, self};
use crate::config::types::ConfigurationRegistryMetrics;
use crate::ActorCoreResult;

/// Registry for managing configuration providers
#[async_trait]
pub trait ConfigurationRegistry: Send + Sync {
    /// Register a configuration provider
    async fn register_provider(&self, provider: Arc<dyn ConfigurationProvider>) -> ActorCoreResult<()>;
    
    /// Unregister a configuration provider
    async fn unregister_provider(&self, provider_id: &str) -> ActorCoreResult<()>;
    
    /// Get configuration provider by ID
    async fn get_provider(&self, provider_id: &str) -> Option<Arc<dyn ConfigurationProvider>>;
    
    /// Get all providers sorted by priority
    async fn get_providers_by_priority(&self) -> Vec<Arc<dyn ConfigurationProvider>>;
    
    /// Get providers for a specific category
    async fn get_providers_for_category(&self, category: &str) -> Vec<Arc<dyn ConfigurationProvider>>;
    
    /// Validate all registered providers
    async fn validate_all_providers(&self) -> ActorCoreResult<()>;
    
    /// Get registry metrics
    async fn get_metrics(&self) -> ConfigurationRegistryMetrics;
}

/// Configuration registry implementation
pub struct ConfigurationRegistryImpl {
    providers: Arc<RwLock<HashMap<String, Arc<dyn ConfigurationProvider>>>>,
    metrics: Arc<RwLock<ConfigurationRegistryMetrics>>,
    registry_config: RegistryConfig,
}

/// Registry configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegistryConfig {
    pub max_providers: usize,
    pub validation_timeout_ms: u64,
    pub enable_metrics: bool,
    pub auto_validation: bool,
    pub validation_interval_ms: u64,
    pub error_messages: HashMap<String, String>,
}

/// Registry error messages
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegistryErrorMessages {
    pub empty_provider_id: String,
    pub provider_not_found: String,
    pub invalid_priority: String,
    pub validation_failed: String,
    pub empty_provider_id_found: String,
    pub max_providers_exceeded: String,
}

impl ConfigurationRegistryImpl {
    pub fn new() -> Self {
        // Load registry configuration
        let registry_config = Self::load_registry_config().unwrap_or_else(|_| {
            tracing::warn!("Failed to load registry config, using defaults");
            Self::get_default_registry_config()
        });

        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(ConfigurationRegistryMetrics::default())),
            registry_config,
        }
    }

    /// Load registry configuration from config file
    pub fn load_registry_config() -> ActorCoreResult<RegistryConfig> {
        // Try to load from registry_config.yaml first
        let registry_config_path = std::path::Path::new("configs/registry_config.yaml");
            
        if registry_config_path.exists() {
            match Self::load_registry_config_from_file(registry_config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    tracing::warn!("Failed to load registry config from file: {}. Using fallback defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_registry_config())
    }

    /// Load registry configuration from file
    fn load_registry_config_from_file(path: &std::path::Path) -> ActorCoreResult<RegistryConfig> {
        let content = std::fs::read_to_string(path)?;
        let config: RegistryConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default registry configuration
    fn get_default_registry_config() -> RegistryConfig {
        let mut error_messages = HashMap::new();
        error_messages.insert("empty_provider_id".to_string(), "Provider ID cannot be empty".to_string());
        error_messages.insert("provider_not_found".to_string(), "Configuration provider not found: {}".to_string());
        error_messages.insert("invalid_priority".to_string(), "Invalid priority for provider {}: {}".to_string());
        error_messages.insert("validation_failed".to_string(), "Provider {} validation failed: {}".to_string());
        error_messages.insert("empty_provider_id_found".to_string(), "Empty provider ID found".to_string());
        error_messages.insert("max_providers_exceeded".to_string(), "Maximum number of providers exceeded: {}".to_string());

        RegistryConfig {
            max_providers: 100,
            validation_timeout_ms: 5000,
            enable_metrics: true,
            auto_validation: true,
            validation_interval_ms: 30000,
            error_messages,
        }
    }

    /// Get all providers sorted by priority
    fn get_providers_by_priority(&self) -> Vec<Arc<dyn ConfigurationProvider>> {
        let providers = self.providers.read();
        let mut provider_list: Vec<Arc<dyn ConfigurationProvider>> = providers.values().cloned().collect();
        
        // Sort by priority (higher priority first)
        provider_list.sort_by(|a, b| b.priority().cmp(&a.priority()));
        provider_list
    }

    /// Get providers for a specific category
    fn get_providers_for_category(&self, category: &str) -> Vec<Arc<dyn ConfigurationProvider>> {
        let providers = self.providers.read();
        let mut category_providers = Vec::new();
        
        for provider in providers.values() {
            if provider.get_supported_categories().contains(&category.to_string()) {
                category_providers.push(provider.clone());
            }
        }
        
        // Sort by priority (higher priority first)
        category_providers.sort_by(|a, b| b.priority().cmp(&a.priority()));
        category_providers
    }
}

#[async_trait]
impl ConfigurationRegistry for ConfigurationRegistryImpl {
    async fn register_provider(&self, provider: Arc<dyn ConfigurationProvider>) -> ActorCoreResult<()> {
        let provider_id = provider.provider_id().to_string();
        
        if provider_id.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                self.registry_config.error_messages.get("empty_provider_id")
                    .unwrap_or(&"Provider ID cannot be empty".to_string())
                    .clone()
            ));
        }

        // Check max providers limit
        let current_count = {
            let providers = self.providers.read();
            providers.len()
        };
        
        if current_count >= self.registry_config.max_providers {
            return Err(crate::ActorCoreError::ConfigurationError(
                self.registry_config.error_messages.get("max_providers_exceeded")
                    .unwrap_or(&"Maximum number of providers exceeded: {}".to_string())
                    .replace("{}", &self.registry_config.max_providers.to_string())
            ));
        }

        // Validate provider
        provider.validate_config().await?;

        let mut providers = self.providers.write();
        
        if providers.contains_key(&provider_id) {
            warn!("Overwriting existing configuration provider: {}", provider_id);
        }
        
        providers.insert(provider_id.clone(), provider);
        
        // Update metrics if enabled
        if self.registry_config.enable_metrics {
            let mut metrics = self.metrics.write();
            metrics.registered_count = providers.len();
            metrics.registration_attempts += 1;
        }
        
        info!("Registered configuration provider: {}", provider_id);
        Ok(())
    }

    async fn unregister_provider(&self, provider_id: &str) -> ActorCoreResult<()> {
        let mut providers = self.providers.write();
        
        if providers.remove(provider_id).is_some() {
            // Update metrics if enabled
            if self.registry_config.enable_metrics {
                let mut metrics = self.metrics.write();
                metrics.registered_count = providers.len();
                metrics.unregistration_attempts += 1;
            }
            
            info!("Unregistered configuration provider: {}", provider_id);
            Ok(())
        } else {
            Err(crate::ActorCoreError::RegistryError(
                self.registry_config.error_messages.get("provider_not_found")
                    .unwrap_or(&"Configuration provider not found: {}".to_string())
                    .replace("{}", provider_id)
            ))
        }
    }

    async fn get_provider(&self, provider_id: &str) -> Option<Arc<dyn ConfigurationProvider>> {
        let providers = self.providers.read();
        providers.get(provider_id).cloned()
    }

    async fn get_providers_by_priority(&self) -> Vec<Arc<dyn ConfigurationProvider>> {
        self.get_providers_by_priority()
    }

    async fn get_providers_for_category(&self, category: &str) -> Vec<Arc<dyn ConfigurationProvider>> {
        self.get_providers_for_category(category)
    }

    async fn validate_all_providers(&self) -> ActorCoreResult<()> {
        // Collect providers into a vector to avoid holding the lock across await points
        let providers: Vec<(String, Arc<dyn provider::ConfigurationProvider>)> = {
            let providers_guard = self.providers.read();
            providers_guard.iter().map(|(id, provider)| (id.clone(), provider.clone())).collect()
        };
        
        for (provider_id, provider) in providers.iter() {
            if provider_id.is_empty() {
                return Err(crate::ActorCoreError::ConfigurationError(
                    self.registry_config.error_messages.get("empty_provider_id_found")
                        .unwrap_or(&"Empty provider ID found".to_string())
                        .clone()
                ));
            }
            
            if provider.priority() < 0 {
                return Err(crate::ActorCoreError::ConfigurationError(
                    self.registry_config.error_messages.get("invalid_priority")
                        .unwrap_or(&"Invalid priority for provider {}: {}".to_string())
                        .replace("{}", &format!("{}: {}", provider_id, provider.priority()))
                ));
            }

            // Validate provider configuration
            if let Err(e) = provider.validate_config().await {
                return Err(crate::ActorCoreError::ConfigurationError(
                    self.registry_config.error_messages.get("validation_failed")
                        .unwrap_or(&"Provider {} validation failed: {}".to_string())
                        .replace("{}", &format!("{}: {}", provider_id, e))
                ));
            }
        }
        
        // Update metrics if enabled
        if self.registry_config.enable_metrics {
            let mut metrics = self.metrics.write();
            metrics.validation_attempts += 1;
        }
        
        Ok(())
    }

    async fn get_metrics(&self) -> ConfigurationRegistryMetrics {
        let metrics = self.metrics.read();
        metrics.clone()
    }
}
