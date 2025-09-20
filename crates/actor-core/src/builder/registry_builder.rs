//! Registry Builder for complex registry setup

use std::sync::Arc;
use tracing::info;

use crate::runtime_registry::*;
use crate::config::manager::ConfigurationManager;
use crate::ActorCoreResult;

/// Registry Builder for complex registry setup
pub struct RegistryBuilder {
    resources: Vec<ResourceDefinition>,
    categories: Vec<CategoryDefinition>,
    tags: Vec<TagDefinition>,
    enable_metrics: bool,
    enable_caching: bool,
    cache_size_mb: usize,
    log_level: String,
    config_manager: Arc<ConfigurationManager>,
}

impl RegistryBuilder {
    /// Create a new Registry Builder
    pub fn new(config_manager: Arc<ConfigurationManager>) -> Self {
        Self {
            resources: Vec::new(),
            categories: Vec::new(),
            tags: Vec::new(),
            enable_metrics: true,
            enable_caching: true,
            cache_size_mb: 50,
            log_level: "info".to_string(),
            config_manager,
        }
    }

    /// Add a resource definition
    pub fn with_resource(mut self, resource: ResourceDefinition) -> Self {
        self.resources.push(resource);
        self
    }

    /// Add a category definition
    pub fn with_category(mut self, category: CategoryDefinition) -> Self {
        self.categories.push(category);
        self
    }

    /// Add a tag definition
    pub fn with_tag(mut self, tag: TagDefinition) -> Self {
        self.tags.push(tag);
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

    /// Build the Registry System
    pub async fn build(self) -> ActorCoreResult<RegistrySystem> {
        info!("Building Registry System with Builder pattern");
        
        // Create registries
        let resource_registry = Arc::new(ResourceRegistryImpl::new());
        let category_registry = Arc::new(CategoryRegistryImpl::new());
        let tag_registry = Arc::new(TagRegistryImpl::new());
        
        // Create registry manager
        let registry_manager = RegistryManager::new(
            resource_registry.clone(),
            category_registry.clone(),
            tag_registry.clone(),
            self.config_manager.clone(),
        );
        
        // Initialize the registry system
        registry_manager.initialize().await?;
        
        // Register custom resources
        for resource in self.resources {
            resource_registry.register_resource(resource).await?;
        }
        
        // Register custom categories
        for category in self.categories {
            category_registry.register_category(category).await?;
        }
        
        // Register custom tags
        for tag in self.tags {
            tag_registry.register_tag(tag).await?;
        }
        
        // Create the complete system
        let system = RegistrySystem {
            resource_registry,
            category_registry,
            tag_registry,
            registry_manager: Arc::new(registry_manager),
            enable_metrics: self.enable_metrics,
            enable_caching: self.enable_caching,
            cache_size_mb: self.cache_size_mb,
            log_level: self.log_level,
        };
        
        info!("Registry System built successfully");
        Ok(system)
    }
}

/// Complete Registry System
pub struct RegistrySystem {
    pub resource_registry: Arc<dyn ResourceRegistry>,
    pub category_registry: Arc<dyn CategoryRegistry>,
    pub tag_registry: Arc<dyn TagRegistry>,
    pub registry_manager: Arc<RegistryManager>,
    pub enable_metrics: bool,
    pub enable_caching: bool,
    pub cache_size_mb: usize,
    pub log_level: String,
}

impl RegistrySystem {
    /// Get resource registry
    pub fn get_resource_registry(&self) -> Arc<dyn ResourceRegistry> {
        self.resource_registry.clone()
    }

    /// Get category registry
    pub fn get_category_registry(&self) -> Arc<dyn CategoryRegistry> {
        self.category_registry.clone()
    }

    /// Get tag registry
    pub fn get_tag_registry(&self) -> Arc<dyn TagRegistry> {
        self.tag_registry.clone()
    }

    /// Get registry manager
    pub fn get_registry_manager(&self) -> Arc<RegistryManager> {
        self.registry_manager.clone()
    }

    /// Get system health status
    pub async fn get_health_status(&self) -> ActorCoreResult<RegistrySystemHealth> {
        let registry_health = self.registry_manager.get_health_status().await?;
        
        Ok(RegistrySystemHealth {
            registry_health,
            enable_metrics: self.enable_metrics,
            enable_caching: self.enable_caching,
            cache_size_mb: self.cache_size_mb,
            log_level: self.log_level.clone(),
        })
    }

    /// Shutdown the system
    pub async fn shutdown(&self) -> ActorCoreResult<()> {
        info!("Shutting down Registry System");
        
        // Note: In a real implementation, this would save registry state
        
        info!("Registry System shutdown complete");
        Ok(())
    }
}

/// Registry System Health Status
#[derive(Debug, Clone)]
pub struct RegistrySystemHealth {
    pub registry_health: RegistryHealthStatus,
    pub enable_metrics: bool,
    pub enable_caching: bool,
    pub cache_size_mb: usize,
    pub log_level: String,
}
