//! Configuration manager for the Configuration Hub system

use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, warn};

use crate::config::types::*;
use crate::config::registry::ConfigurationRegistry;
use crate::config::combiner::ConfigurationCombiner;
use crate::config::aggregator::ConfigurationAggregator;
use crate::config::loader::ConfigurationLoader;
use crate::ActorCoreResult;

/// High-level configuration manager
pub struct ConfigurationManager {
    registry: Arc<dyn ConfigurationRegistry>,
    combiner: Arc<dyn ConfigurationCombiner>,
    aggregator: Arc<dyn ConfigurationAggregator>,
    loader: Arc<ConfigurationLoader>,
}

impl ConfigurationManager {
    pub fn new(
        registry: Arc<dyn ConfigurationRegistry>,
        combiner: Arc<dyn ConfigurationCombiner>,
        aggregator: Arc<dyn ConfigurationAggregator>,
        loader: Arc<ConfigurationLoader>,
    ) -> Self {
        Self {
            registry,
            combiner,
            aggregator,
            loader,
        }
    }

    /// Initialize the configuration manager
    pub async fn initialize(&self) -> ActorCoreResult<()> {
        info!("Initializing Configuration Manager");
        
        // Default rules are now registered by subsystems through the Runtime Registry
        // No need to load hardcoded rules
        
        // Load all configurations
        self.loader.load_all_configs().await?;
        
        // Validate all providers
        self.registry.validate_all_providers().await?;
        
        info!("Configuration Manager initialized successfully");
        Ok(())
    }

    /// Get configuration value
    pub async fn get_config(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>> {
        self.aggregator.get_config_value(category, key).await
    }

    /// Set configuration value (for runtime changes)
    pub async fn set_config(&self, category: &str, key: &str, _value: ConfigurationValue) -> ActorCoreResult<()> {
        // This would require a runtime configuration provider
        // For now, we'll just log the attempt
        warn!("Runtime configuration changes not yet implemented: {}:{}", category, key);
        Ok(())
    }

    /// Get all configuration for a category
    pub async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        tracing::debug!("🔍 Configuration Manager: Requesting category '{}'", category);
        let result = self.aggregator.get_category_config(category).await;
        match &result {
            Ok(config) => {
                tracing::debug!("✅ Configuration Manager: Found {} keys in category '{}'", config.len(), category);
                for (key, value) in config {
                    tracing::debug!("   {}: {:?}", key, value);
                }
            }
            Err(e) => {
                tracing::debug!("❌ Configuration Manager: Failed to load category '{}': {}", category, e);
            }
        }
        result
    }

    /// Get all configuration
    pub async fn get_all_config(&self) -> ActorCoreResult<HashMap<String, HashMap<String, ConfigurationValue>>> {
        self.aggregator.get_all_config().await
    }

    /// Refresh all configuration
    pub async fn refresh_config(&self) -> ActorCoreResult<()> {
        info!("Refreshing configuration");
        self.aggregator.refresh_config().await
    }

    /// Save configuration (for persistence)
    pub async fn save_configs(&self) -> ActorCoreResult<()> {
        info!("💾 Saving configuration to MongoDB...");
        
        // Get all configuration data
        let _all_config = self.aggregator.get_all_config().await?;
        
        // Find MongoDB provider and save each category
        let providers = self.aggregator.get_providers();
        for _provider in providers {
            #[cfg(feature = "mongodb-storage")]
            if let Some(mongodb_provider) = _provider.as_any().downcast_ref::<crate::config::mongodb::MongoDBConfigurationProvider>() {
                info!("💾 Found MongoDB provider, saving {} categories", _all_config.len());
                
                for (category, configs) in _all_config {
                    info!("💾 Saving category '{}' with {} keys", category, configs.len());
                    for (key, value) in configs {
                        match mongodb_provider.save_to_database(&category, &key, &value).await {
                            Ok(()) => {
                                info!("✅ Saved {}.{} to MongoDB", category, key);
                            }
                            Err(e) => {
                                warn!("❌ Failed to save {}.{} to MongoDB: {}", category, key, e);
                            }
                        }
                    }
                }
                info!("💾 Configuration save completed");
                return Ok(());
            }
            #[cfg(not(feature = "mongodb-storage"))]
            {
                // MongoDB feature not enabled, skip this provider
            }
        }
        
        warn!("⚠️  No MongoDB provider found, configuration not saved");
        Ok(())
    }

    /// Get registry reference
    pub fn get_registry(&self) -> Arc<dyn ConfigurationRegistry> {
        self.registry.clone()
    }

    /// Get combiner reference
    pub fn get_combiner(&self) -> Arc<dyn ConfigurationCombiner> {
        self.combiner.clone()
    }

    /// Get aggregator reference
    pub fn get_aggregator(&self) -> Arc<dyn ConfigurationAggregator> {
        self.aggregator.clone()
    }

    /// Get loader reference
    pub fn get_loader(&self) -> Arc<ConfigurationLoader> {
        self.loader.clone()
    }

    /// Get system health status
    pub async fn get_health_status(&self) -> ConfigurationHealthStatus {
        let registry_metrics = self.registry.get_metrics().await;
        let combiner_metrics = self.combiner.get_metrics().await;
        let aggregator_metrics = self.aggregator.get_metrics().await;

        ConfigurationHealthStatus {
            registry_health: registry_metrics.registered_count > 0,
            combiner_health: combiner_metrics.rule_count > 0,
            aggregator_health: true, // total_requests is always >= 0 (u64)
            total_providers: registry_metrics.registered_count,
            total_merge_rules: combiner_metrics.rule_count,
            total_requests: aggregator_metrics.total_requests,
            cache_hit_ratio: if aggregator_metrics.total_requests > 0 {
                aggregator_metrics.cache_hits as f64 / aggregator_metrics.total_requests as f64
            } else {
                0.0
            },
        }
    }

    /// Get system IDs from configuration
    pub async fn get_system_ids(&self) -> ActorCoreResult<Vec<String>> {
        let config = self.get_category_config("system_ids").await?;
        
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
    pub async fn get_context_types(&self) -> ActorCoreResult<Vec<String>> {
        let config = self.get_category_config("context_types").await?;
        
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

/// Configuration health status
#[derive(Debug, Clone)]
pub struct ConfigurationHealthStatus {
    pub registry_health: bool,
    pub combiner_health: bool,
    pub aggregator_health: bool,
    pub total_providers: usize,
    pub total_merge_rules: usize,
    pub total_requests: u64,
    pub cache_hit_ratio: f64,
}