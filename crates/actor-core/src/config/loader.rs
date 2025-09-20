//! Configuration loader for the Configuration Hub system

use std::sync::Arc;
use std::path::PathBuf;
use tracing::{info, error};

use crate::config::provider::ConfigurationProvider;
use crate::config::registry::ConfigurationRegistry;
use crate::config::combiner::ConfigurationCombiner;
use crate::config::aggregator::ConfigurationAggregator;
use crate::ActorCoreResult;

/// Configuration loader for loading configurations from multiple sources
pub struct ConfigurationLoader {
    providers: Vec<Arc<dyn ConfigurationProvider>>,
    registry: Arc<dyn ConfigurationRegistry>,
    combiner: Arc<dyn ConfigurationCombiner>,
    aggregator: Arc<dyn ConfigurationAggregator>,
    config_paths: Vec<PathBuf>,
}

impl ConfigurationLoader {
    pub fn new(
        registry: Arc<dyn ConfigurationRegistry>,
        combiner: Arc<dyn ConfigurationCombiner>,
        aggregator: Arc<dyn ConfigurationAggregator>,
    ) -> Self {
        Self {
            providers: Vec::new(),
            registry,
            combiner,
            aggregator,
            config_paths: Vec::new(),
        }
    }

    /// Add a configuration provider
    pub fn add_provider(&mut self, provider: Arc<dyn ConfigurationProvider>) {
        self.providers.push(provider);
    }

    /// Add configuration file path
    pub fn add_config_path(&mut self, path: PathBuf) {
        self.config_paths.push(path);
    }

    /// Load all configurations
    pub async fn load_all_configs(&self) -> ActorCoreResult<()> {
        info!("Loading all configurations");
        
        // Register all providers
        for provider in &self.providers {
            if let Err(e) = self.registry.register_provider(provider.clone()).await {
                error!("Failed to register provider {}: {}", provider.provider_id(), e);
                return Err(e);
            }
        }
        
        // Load configurations from file paths
        self.load_config_files().await?;
        
        // Validate all providers
        self.registry.validate_all_providers().await?;
        
        info!("Loaded {} configuration providers and {} config files", 
              self.providers.len(), self.config_paths.len());
        Ok(())
    }

    /// Load configuration files from registered paths
    async fn load_config_files(&self) -> ActorCoreResult<()> {
        if self.config_paths.is_empty() {
            info!("No configuration file paths specified, skipping file loading");
            return Ok(());
        }

        for config_path in &self.config_paths {
            if !config_path.exists() {
                error!("Configuration file does not exist: {:?}", config_path);
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Configuration file not found: {:?}", config_path)
                ));
            }

            if !config_path.is_file() {
                error!("Configuration path is not a file: {:?}", config_path);
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Configuration path is not a file: {:?}", config_path)
                ));
            }

            info!("Loading configuration from: {:?}", config_path);
            // TODO: Implement actual file loading logic
            // This should load YAML/JSON files and register them with appropriate providers
        }

        Ok(())
    }

    /// Reload all configurations
    pub async fn reload_configs(&self) -> ActorCoreResult<()> {
        info!("Reloading all configurations");
        
        // Refresh aggregator cache
        self.aggregator.refresh_config().await?;
        
        info!("Configuration reload completed");
        Ok(())
    }

    /// Validate all configurations
    pub async fn validate_all_configs(&self) -> ActorCoreResult<()> {
        info!("Validating all configurations");
        
        // Validate all providers
        self.registry.validate_all_providers().await?;
        
        info!("Configuration validation completed");
        Ok(())
    }

    /// Get configuration file paths
    pub fn get_config_paths(&self) -> &[PathBuf] {
        &self.config_paths
    }

    /// Get provider count
    pub fn get_provider_count(&self) -> usize {
        self.providers.len()
    }

    /// Get combiner instance for advanced configuration merging
    pub fn get_combiner(&self) -> &Arc<dyn ConfigurationCombiner> {
        &self.combiner
    }

    /// Load default merge rules using the combiner
    pub async fn load_default_merge_rules(&self) -> ActorCoreResult<()> {
        info!("Loading default merge rules");
        
        // Note: The combiner trait doesn't have load_default_rules method
        // This would need to be implemented in the concrete combiner implementation
        // For now, we'll just log that this functionality is not yet implemented
        info!("Default merge rules loading not yet implemented in trait interface");
        Ok(())
    }

    /// Get configuration statistics
    pub async fn get_config_stats(&self) -> ActorCoreResult<ConfigurationLoaderStats> {
        let provider_count = self.providers.len();
        let config_file_count = self.config_paths.len();
        
        // Get aggregator metrics
        let aggregator_metrics = self.aggregator.get_metrics().await;
        
        // Get combiner metrics
        let combiner_metrics = self.combiner.get_metrics().await;
        
        Ok(ConfigurationLoaderStats {
            provider_count,
            config_file_count,
            aggregator_metrics,
            combiner_metrics,
        })
    }
}

/// Statistics for configuration loader
#[derive(Debug, Clone)]
pub struct ConfigurationLoaderStats {
    pub provider_count: usize,
    pub config_file_count: usize,
    pub aggregator_metrics: crate::config::types::ConfigurationAggregatorMetrics,
    pub combiner_metrics: crate::config::types::ConfigurationCombinerMetrics,
}