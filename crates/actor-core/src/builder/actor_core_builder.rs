//! Actor Core Builder for complex setup scenarios

use std::sync::Arc;
use std::path::PathBuf;
use tracing::info;

use crate::config::*;
use crate::runtime_registry::*;
use crate::ActorCoreResult;

/// Actor Core Builder for complex setup scenarios
pub struct ActorCoreBuilder {
    config_manager: Option<Arc<ConfigurationManager>>,
    _registry_manager: Option<Arc<RegistryManager>>,
    #[cfg(feature = "mongodb-storage")]
    mongodb_manager: Option<Arc<crate::config::mongodb_manager::MongoDBConfigManager>>,
    config_paths: Vec<PathBuf>,
    enable_hot_reload: bool,
    enable_metrics: bool,
    enable_caching: bool,
    cache_size_mb: usize,
    log_level: String,
    use_mongodb_config: bool,
}

impl ActorCoreBuilder {
    /// Create a new Actor Core Builder
    /// TODO: Load default values from configuration instead of hardcoded values
    pub fn new() -> Self {
        Self {
            config_manager: None,
            _registry_manager: None,
            #[cfg(feature = "mongodb-storage")]
            mongodb_manager: None,
            config_paths: Vec::new(),
            // TODO: Load these defaults from configuration
            enable_hot_reload: false,
            enable_metrics: true,
            enable_caching: true,
            cache_size_mb: 100,
            log_level: "info".to_string(),
            use_mongodb_config: false,
        }
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

    /// Enable MongoDB configuration
    pub fn with_mongodb_config(mut self, enabled: bool) -> Self {
        self.use_mongodb_config = enabled;
        self
    }

    /// Build the Actor Core system
    pub async fn build(self) -> ActorCoreResult<ActorCoreSystem> {
        info!("Building Actor Core system with Builder pattern");
        
        // Build Configuration Hub
        let config_manager = self.build_configuration_hub().await?;
        
        // Build Runtime Registry
        let registry_manager = self.build_runtime_registry().await?;
        
        // Create the complete system
        let system = ActorCoreSystem {
            config_manager,
            registry_manager,
            enable_hot_reload: self.enable_hot_reload,
            enable_metrics: self.enable_metrics,
            enable_caching: self.enable_caching,
            cache_size_mb: self.cache_size_mb,
            log_level: self.log_level,
            use_mongodb_config: self.use_mongodb_config,
        };
        
        info!("Actor Core system built successfully");
        Ok(system)
    }

    /// Build the Configuration Hub
    async fn build_configuration_hub(&self) -> ActorCoreResult<Arc<ConfigurationManager>> {
        info!("Building Configuration Hub");
        
        // Create configuration registry
        let registry = Arc::new(crate::config::registry::ConfigurationRegistryImpl::new());
        
        // Create configuration combiner
        let combiner = Arc::new(crate::config::combiner::ConfigurationCombinerImpl::new());
        combiner.load_default_rules()?;
        
        // Create configuration aggregator
        let aggregator = Arc::new(crate::config::aggregator::ConfigurationAggregatorImpl::new(registry.clone(), combiner.clone()));
        
        // Create configuration loader
        let mut loader = ConfigurationLoader::new(registry.clone(), combiner.clone(), aggregator.clone());
        
        // Add default configuration provider
        // TODO: Load default config path from configuration
        let default_config_path = PathBuf::from("configs/actor_core_defaults.yaml");
        let default_provider = Arc::new(crate::config::loaders::DefaultConfigProvider::new(default_config_path)?);
        loader.add_provider(default_provider);
        
        // Add example provider
        let example_provider = Arc::new(crate::config::providers::ExampleConfigurationProvider::new());
        loader.add_provider(example_provider);
        
        // Add environment provider
        // TODO: Load provider priority and prefix from configuration
        let mut env_provider = crate::config::providers::EnvironmentConfigurationProvider::new(
            "env_provider".to_string(),
            200, // TODO: Load from config
            "ACTOR_CORE".to_string(),
        );
        env_provider.load_from_environment()?;
        loader.add_provider(Arc::new(env_provider));
        
        // Add database provider
        let db_config = crate::config::providers::DatabaseConfigurationProvider::load_database_config("configs/database_config.yaml")?;
        let mut db_provider = crate::config::providers::DatabaseConfigurationProvider::new(
            "db_provider".to_string(),
            300,
            db_config,
        );
        db_provider.load_from_database().await?;
        loader.add_provider(Arc::new(db_provider));

        // Add MongoDB provider if enabled
        #[cfg(feature = "mongodb-storage")]
        if self.use_mongodb_config {
            info!("Adding MongoDB configuration provider");
            let mongodb_config = crate::config::mongodb::MongoDBConfigurationProvider::load_mongodb_config("configs/mongodb_config.yaml")?;
            let mongodb_provider = Arc::new(crate::config::mongodb::MongoDBConfigurationProvider::new(
                "mongodb_provider".to_string(),
                50, // High priority for MongoDB
                mongodb_config,
            ).await?);
            loader.add_provider(mongodb_provider);
        }
        
        // Add custom configuration files
        for config_path in &self.config_paths {
            let file_provider = Arc::new(crate::config::providers::FileConfigurationProvider::new(
                format!("file_provider_{}", config_path.file_name().unwrap().to_string_lossy()),
                100, // TODO: Load from config
                config_path.clone(),
            ));
            file_provider.load_from_file().await?;
            loader.add_provider(file_provider);
        }
        
        // Create configuration manager
        let config_manager = ConfigurationManager::new(registry, combiner, aggregator, Arc::new(loader));
        
        // Initialize the configuration system
        config_manager.initialize().await?;
        
        info!("Configuration Hub built successfully");
        Ok(Arc::new(config_manager))
    }

    /// Build the Runtime Registry
    async fn build_runtime_registry(&self) -> ActorCoreResult<Arc<RegistryManager>> {
        info!("Building Runtime Registry");
        
        // Create registries
        let resource_registry = Arc::new(crate::runtime_registry::resource_registry::ResourceRegistryImpl::new());
        let category_registry = Arc::new(crate::runtime_registry::category_registry::CategoryRegistryImpl::new());
        let tag_registry = Arc::new(crate::runtime_registry::tag_registry::TagRegistryImpl::new());
        
        // Create registry manager
        let registry_manager = crate::runtime_registry::registry_manager::RegistryManager::new(
            resource_registry,
            category_registry,
            tag_registry,
            self.config_manager.clone().expect("ConfigurationManager must be initialized"),
        );
        
        // Initialize the registry system
        registry_manager.initialize().await?;
        
        info!("Runtime Registry built successfully");
        Ok(Arc::new(registry_manager))
    }
}

/// Complete Actor Core System
pub struct ActorCoreSystem {
    pub config_manager: Arc<ConfigurationManager>,
    pub registry_manager: Arc<RegistryManager>,
    pub enable_hot_reload: bool,
    pub enable_metrics: bool,
    pub enable_caching: bool,
    pub cache_size_mb: usize,
    pub log_level: String,
    pub use_mongodb_config: bool,
}

impl ActorCoreSystem {
    /// Get configuration manager
    pub fn get_config_manager(&self) -> Arc<ConfigurationManager> {
        self.config_manager.clone()
    }

    /// Get registry manager
    pub fn get_registry_manager(&self) -> Arc<RegistryManager> {
        self.registry_manager.clone()
    }

    /// Get system health status
    pub async fn get_health_status(&self) -> ActorCoreResult<ActorCoreSystemHealth> {
        let config_health = self.config_manager.get_health_status().await;
        let registry_health = self.registry_manager.get_health_status().await?;
        
        Ok(ActorCoreSystemHealth {
            config_health,
            registry_health,
            enable_hot_reload: self.enable_hot_reload,
            enable_metrics: self.enable_metrics,
            enable_caching: self.enable_caching,
            cache_size_mb: self.cache_size_mb,
            log_level: self.log_level.clone(),
        })
    }

    /// Shutdown the system
    pub async fn shutdown(&self) -> ActorCoreResult<()> {
        info!("Shutting down Actor Core system");
        
        // Save configurations
        self.config_manager.save_configs().await?;
        
        // Clear registries
        // Note: In a real implementation, this would save registry state
        
        info!("Actor Core system shutdown complete");
        Ok(())
    }
}

/// Actor Core System Health Status
#[derive(Debug, Clone)]
pub struct ActorCoreSystemHealth {
    pub config_health: crate::config::manager::ConfigurationHealthStatus,
    pub registry_health: crate::runtime_registry::registry_manager::RegistryHealthStatus,
    pub enable_hot_reload: bool,
    pub enable_metrics: bool,
    pub enable_caching: bool,
    pub cache_size_mb: usize,
    pub log_level: String,
}
