//! Configuration Hub Builder for complex configuration setup

use std::sync::Arc;
use std::path::PathBuf;
use tracing::info;

use crate::config::*;
use crate::ActorCoreResult;

/// Configuration Hub Builder for complex configuration setup
pub struct ConfigurationHubBuilder {
    providers: Vec<Arc<dyn ConfigurationProvider>>,
    config_paths: Vec<PathBuf>,
    enable_hot_reload: bool,
    enable_metrics: bool,
    enable_caching: bool,
    cache_size_mb: usize,
    log_level: String,
}

impl ConfigurationHubBuilder {
    /// Create a new Configuration Hub Builder
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            config_paths: Vec::new(),
            enable_hot_reload: false,
            enable_metrics: true,
            enable_caching: true,
            cache_size_mb: 100,
            log_level: "info".to_string(),
        }
    }

    /// Add a configuration provider
    pub fn with_provider(mut self, provider: Arc<dyn ConfigurationProvider>) -> Self {
        self.providers.push(provider);
        self
    }

    /// Add a configuration file path
    pub fn with_config_path(mut self, path: PathBuf) -> Self {
        self.config_paths.push(path);
        self
    }

    /// Enable hot reload for configuration files
    pub fn with_hot_reload(mut self, enable: bool) -> Self {
        self.enable_hot_reload = enable;
        self
    }

    /// Enable metrics collection
    pub fn with_metrics(mut self, enable: bool) -> Self {
        self.enable_metrics = enable;
        self
    }

    /// Enable caching
    pub fn with_caching(mut self, enable: bool) -> Self {
        self.enable_caching = enable;
        self
    }

    /// Set cache size in MB
    pub fn with_cache_size(mut self, size_mb: usize) -> Self {
        self.cache_size_mb = size_mb;
        self
    }

    /// Set log level
    pub fn with_log_level(mut self, level: String) -> Self {
        self.log_level = level;
        self
    }

    /// Build the Configuration Hub
    pub async fn build(self) -> ActorCoreResult<ConfigurationHubSystem> {
        info!("Building Configuration Hub with Builder pattern");
        
        // Create configuration registry
        let registry = Arc::new(crate::config::registry::ConfigurationRegistryImpl::new());
        
        // Create configuration combiner
        let combiner = Arc::new(crate::config::combiner::ConfigurationCombinerImpl::new());
        combiner.load_default_rules()?;
        
        // Create configuration aggregator
        let aggregator = Arc::new(crate::config::aggregator::ConfigurationAggregatorImpl::new(registry.clone(), combiner.clone()));
        
        // Create configuration loader
        let mut loader = ConfigurationLoader::new(registry.clone(), combiner.clone(), aggregator.clone());
        
        // Add all providers
        for provider in self.providers {
            loader.add_provider(provider);
        }
        
        // Add configuration files
        for config_path in self.config_paths {
            let file_provider = Arc::new(crate::config::providers::FileConfigurationProvider::new(
                format!("file_provider_{}", config_path.file_name().unwrap().to_string_lossy()),
                100,
                config_path,
            ));
            file_provider.load_from_file().await?;
            loader.add_provider(file_provider);
        }
        
        // Create configuration manager
        let config_manager = ConfigurationManager::new(registry, combiner, aggregator, Arc::new(loader));
        
        // Initialize the configuration system
        config_manager.initialize().await?;
        
        // Create the complete system
        let system = ConfigurationHubSystem {
            config_manager: Arc::new(config_manager),
            enable_hot_reload: self.enable_hot_reload,
            enable_metrics: self.enable_metrics,
            enable_caching: self.enable_caching,
            cache_size_mb: self.cache_size_mb,
            log_level: self.log_level,
        };
        
        info!("Configuration Hub built successfully");
        Ok(system)
    }
}

/// Complete Configuration Hub System
pub struct ConfigurationHubSystem {
    pub config_manager: Arc<ConfigurationManager>,
    pub enable_hot_reload: bool,
    pub enable_metrics: bool,
    pub enable_caching: bool,
    pub cache_size_mb: usize,
    pub log_level: String,
}

impl ConfigurationHubSystem {
    /// Get configuration manager
    pub fn get_config_manager(&self) -> Arc<ConfigurationManager> {
        self.config_manager.clone()
    }

    /// Get system health status
    pub async fn get_health_status(&self) -> ConfigurationHubHealth {
        let config_health = self.config_manager.get_health_status().await;
        
        ConfigurationHubHealth {
            config_health,
            enable_hot_reload: self.enable_hot_reload,
            enable_metrics: self.enable_metrics,
            enable_caching: self.enable_caching,
            cache_size_mb: self.cache_size_mb,
            log_level: self.log_level.clone(),
        }
    }

    /// Shutdown the system
    pub async fn shutdown(&self) -> ActorCoreResult<()> {
        info!("Shutting down Configuration Hub");
        
        // Save configurations
        self.config_manager.save_configs().await?;
        
        info!("Configuration Hub shutdown complete");
        Ok(())
    }
}

/// Configuration Hub Health Status
#[derive(Debug, Clone)]
pub struct ConfigurationHubHealth {
    pub config_health: crate::config::manager::ConfigurationHealthStatus,
    pub enable_hot_reload: bool,
    pub enable_metrics: bool,
    pub enable_caching: bool,
    pub cache_size_mb: usize,
    pub log_level: String,
}
