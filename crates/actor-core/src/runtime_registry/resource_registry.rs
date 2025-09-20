//! Resource registry for dynamic resource management

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tracing::{info, warn};

use crate::ActorCoreResult;

/// Resource definition for runtime registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub resource_type: ResourceType,
    pub base_value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub regen_rate: f64,
    pub regen_type: RegenType,
    pub dependencies: Vec<String>,
    pub tags: Vec<String>,
    pub subsystem_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Resource type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResourceType {
    Health,
    Mana,
    Stamina,
    Experience,
    Level,
    Energy,
    Spirit,
    Custom(String),
}

/// Regeneration type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegenType {
    Passive,
    Active,
    Conditional,
    None,
    Custom(String),
}

/// Resource registry trait
#[async_trait]
pub trait ResourceRegistry: Send + Sync {
    /// Register a resource definition
    async fn register_resource(&self, resource: ResourceDefinition) -> ActorCoreResult<()>;
    
    /// Get all registered resources
    async fn get_all_resources(&self) -> ActorCoreResult<Vec<ResourceDefinition>>;
    
    /// Get resources by category
    async fn get_resources_by_category(&self, category: &str) -> ActorCoreResult<Vec<ResourceDefinition>>;
    
    /// Get resource by ID
    async fn get_resource(&self, id: &str) -> ActorCoreResult<Option<ResourceDefinition>>;
    
    /// Check if resource exists
    async fn has_resource(&self, id: &str) -> ActorCoreResult<bool>;
    
    /// Get resource categories
    async fn get_resource_categories(&self) -> ActorCoreResult<Vec<String>>;
    
    /// Unregister resource
    async fn unregister_resource(&self, id: &str) -> ActorCoreResult<()>;
    
    /// Get resources by subsystem
    async fn get_resources_by_subsystem(&self, subsystem_id: &str) -> ActorCoreResult<Vec<ResourceDefinition>>;
}

/// Resource registry implementation
pub struct ResourceRegistryImpl {
    resources: Arc<RwLock<HashMap<String, ResourceDefinition>>>,
    metrics: Arc<RwLock<ResourceRegistryMetrics>>,
}

impl ResourceRegistryImpl {
    pub fn new() -> Self {
        Self::new_with_config().unwrap_or_else(|_| {
            warn!("Failed to load resource registry config, using hardcoded defaults");
            Self {
                resources: Arc::new(RwLock::new(HashMap::new())),
                metrics: Arc::new(RwLock::new(ResourceRegistryMetrics::default())),
            }
        })
    }

    pub fn new_with_config() -> ActorCoreResult<Self> {
        let _config = ResourceRegistryConfig::load_config()?;
        
        Ok(Self {
            resources: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(ResourceRegistryMetrics::load_default_metrics()?)),
        })
    }
}

#[async_trait]
impl ResourceRegistry for ResourceRegistryImpl {
    async fn register_resource(&self, resource: ResourceDefinition) -> ActorCoreResult<()> {
        let resource_id = resource.id.clone();
        
        if resource_id.is_empty() {
            return Err(crate::ActorCoreError::RegistryError(
                "Resource ID cannot be empty".to_string()
            ));
        }

        let mut resources = self.resources.write();
        
        if resources.contains_key(&resource_id) {
            warn!("Overwriting existing resource: {}", resource_id);
        }
        
        resources.insert(resource_id.clone(), resource);
        
        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.registered_count = resources.len();
        metrics.registration_attempts += 1;
        
        info!("Registered resource: {}", resource_id);
        Ok(())
    }

    async fn get_all_resources(&self) -> ActorCoreResult<Vec<ResourceDefinition>> {
        let resources = self.resources.read();
        Ok(resources.values().cloned().collect())
    }

    async fn get_resources_by_category(&self, category: &str) -> ActorCoreResult<Vec<ResourceDefinition>> {
        let resources = self.resources.read();
        Ok(resources
            .values()
            .filter(|resource| resource.category == category)
            .cloned()
            .collect())
    }

    async fn get_resource(&self, id: &str) -> ActorCoreResult<Option<ResourceDefinition>> {
        let resources = self.resources.read();
        Ok(resources.get(id).cloned())
    }

    async fn has_resource(&self, id: &str) -> ActorCoreResult<bool> {
        let resources = self.resources.read();
        Ok(resources.contains_key(id))
    }

    async fn get_resource_categories(&self) -> ActorCoreResult<Vec<String>> {
        let resources = self.resources.read();
        let mut categories = std::collections::HashSet::new();
        
        for resource in resources.values() {
            categories.insert(resource.category.clone());
        }
        
        Ok(categories.into_iter().collect())
    }

    async fn unregister_resource(&self, id: &str) -> ActorCoreResult<()> {
        let mut resources = self.resources.write();
        
        if resources.remove(id).is_some() {
            // Update metrics
            let mut metrics = self.metrics.write();
            metrics.registered_count = resources.len();
            metrics.unregistration_attempts += 1;
            
            info!("Unregistered resource: {}", id);
            Ok(())
        } else {
            Err(crate::ActorCoreError::RegistryError(
                format!("Resource not found: {}", id)
            ))
        }
    }

    async fn get_resources_by_subsystem(&self, subsystem_id: &str) -> ActorCoreResult<Vec<ResourceDefinition>> {
        let resources = self.resources.read();
        Ok(resources
            .values()
            .filter(|resource| resource.subsystem_id == subsystem_id)
            .cloned()
            .collect())
    }
}

/// Resource registry metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRegistryMetrics {
    pub registered_count: usize,
    pub registration_attempts: u64,
    pub unregistration_attempts: u64,
    pub lookup_attempts: u64,
}

impl Default for ResourceRegistryMetrics {
    fn default() -> Self {
        Self::load_default_metrics().unwrap_or_else(|_| {
            warn!("Failed to load resource registry metrics config, using hardcoded defaults");
            Self {
                registered_count: 0,
                registration_attempts: 0,
                unregistration_attempts: 0,
                lookup_attempts: 0,
            }
        })
    }
}

impl ResourceRegistryMetrics {
    /// Load default resource registry metrics from configuration
    pub fn load_default_metrics() -> ActorCoreResult<Self> {
        // Try to load from resource_registry_config.yaml first
        let config_path = std::path::Path::new("configs/resource_registry_config.yaml");
            
        if config_path.exists() {
            match Self::load_metrics_from_file(config_path) {
                Ok(metrics) => return Ok(metrics),
                Err(e) => {
                    warn!("Failed to load resource registry metrics from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self {
            registered_count: 0,
            registration_attempts: 0,
            unregistration_attempts: 0,
            lookup_attempts: 0,
        })
    }

    /// Load metrics from file
    fn load_metrics_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: ResourceRegistryConfig = serde_yaml::from_str(&content)?;
        Ok(Self {
            registered_count: config.default_registered_count,
            registration_attempts: config.default_registration_attempts,
            unregistration_attempts: config.default_unregistration_attempts,
            lookup_attempts: config.default_lookup_attempts,
        })
    }
}

/// Resource registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRegistryConfig {
    pub enable_validation: bool,
    pub enable_caching: bool,
    pub max_resources: usize,
    pub cache_ttl_seconds: u64,
    pub default_registered_count: usize,
    pub default_registration_attempts: u64,
    pub default_unregistration_attempts: u64,
    pub default_lookup_attempts: u64,
}

impl ResourceRegistryConfig {
    /// Load resource registry configuration from config file
    pub fn load_config() -> ActorCoreResult<Self> {
        // Try to load from resource_registry_config.yaml first
        let config_path = std::path::Path::new("configs/resource_registry_config.yaml");
            
        if config_path.exists() {
            match Self::load_config_from_file(config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    warn!("Failed to load resource registry config from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_config())
    }

    /// Load configuration from file
    fn load_config_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: ResourceRegistryConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default configuration
    fn get_default_config() -> Self {
        Self {
            enable_validation: true,
            enable_caching: true,
            max_resources: 10000,
            cache_ttl_seconds: 3600,
            default_registered_count: 0,
            default_registration_attempts: 0,
            default_unregistration_attempts: 0,
            default_lookup_attempts: 0,
        }
    }
}
