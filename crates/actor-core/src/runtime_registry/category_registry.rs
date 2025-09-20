//! Category registry for dynamic category management

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tracing::{info, warn};

use crate::ActorCoreResult;

/// Category definition for runtime registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_category: Option<String>,
    pub tags: Vec<String>,
    pub subsystem_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Category registry trait
#[async_trait]
pub trait CategoryRegistry: Send + Sync {
    /// Register a category definition
    async fn register_category(&self, category: CategoryDefinition) -> ActorCoreResult<()>;
    
    /// Get all registered categories
    async fn get_all_categories(&self) -> ActorCoreResult<Vec<CategoryDefinition>>;
    
    /// Get category by ID
    async fn get_category(&self, id: &str) -> ActorCoreResult<Option<CategoryDefinition>>;
    
    /// Check if category exists
    async fn has_category(&self, id: &str) -> ActorCoreResult<bool>;
    
    /// Get category tags
    async fn get_category_tags(&self, category_id: &str) -> ActorCoreResult<Vec<String>>;
    
    /// Unregister category
    async fn unregister_category(&self, id: &str) -> ActorCoreResult<()>;
    
    /// Get categories by subsystem
    async fn get_categories_by_subsystem(&self, subsystem_id: &str) -> ActorCoreResult<Vec<CategoryDefinition>>;
    
    /// Get child categories
    async fn get_child_categories(&self, parent_id: &str) -> ActorCoreResult<Vec<CategoryDefinition>>;
}

/// Category registry implementation
pub struct CategoryRegistryImpl {
    categories: Arc<RwLock<HashMap<String, CategoryDefinition>>>,
    metrics: Arc<RwLock<CategoryRegistryMetrics>>,
}

impl CategoryRegistryImpl {
    pub fn new() -> Self {
        Self::new_with_config().unwrap_or_else(|_| {
            warn!("Failed to load category registry config, using hardcoded defaults");
            Self {
                categories: Arc::new(RwLock::new(HashMap::new())),
                metrics: Arc::new(RwLock::new(CategoryRegistryMetrics::default())),
            }
        })
    }

    pub fn new_with_config() -> ActorCoreResult<Self> {
        let _config = CategoryRegistryConfig::load_config()?;
        
        Ok(Self {
            categories: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(CategoryRegistryMetrics::load_default_metrics()?)),
        })
    }
}

#[async_trait]
impl CategoryRegistry for CategoryRegistryImpl {
    async fn register_category(&self, category: CategoryDefinition) -> ActorCoreResult<()> {
        let category_id = category.id.clone();
        
        if category_id.is_empty() {
            return Err(crate::ActorCoreError::RegistryError(
                "Category ID cannot be empty".to_string()
            ));
        }

        let mut categories = self.categories.write();
        
        if categories.contains_key(&category_id) {
            warn!("Overwriting existing category: {}", category_id);
        }
        
        categories.insert(category_id.clone(), category);
        
        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.registered_count = categories.len();
        metrics.registration_attempts += 1;
        
        info!("Registered category: {}", category_id);
        Ok(())
    }

    async fn get_all_categories(&self) -> ActorCoreResult<Vec<CategoryDefinition>> {
        let categories = self.categories.read();
        Ok(categories.values().cloned().collect())
    }

    async fn get_category(&self, id: &str) -> ActorCoreResult<Option<CategoryDefinition>> {
        let categories = self.categories.read();
        Ok(categories.get(id).cloned())
    }

    async fn has_category(&self, id: &str) -> ActorCoreResult<bool> {
        let categories = self.categories.read();
        Ok(categories.contains_key(id))
    }

    async fn get_category_tags(&self, category_id: &str) -> ActorCoreResult<Vec<String>> {
        let categories = self.categories.read();
        if let Some(category) = categories.get(category_id) {
            Ok(category.tags.clone())
        } else {
            Ok(Vec::new())
        }
    }

    async fn unregister_category(&self, id: &str) -> ActorCoreResult<()> {
        let mut categories = self.categories.write();
        
        if categories.remove(id).is_some() {
            // Update metrics
            let mut metrics = self.metrics.write();
            metrics.registered_count = categories.len();
            metrics.unregistration_attempts += 1;
            
            info!("Unregistered category: {}", id);
            Ok(())
        } else {
            Err(crate::ActorCoreError::RegistryError(
                format!("Category not found: {}", id)
            ))
        }
    }

    async fn get_categories_by_subsystem(&self, subsystem_id: &str) -> ActorCoreResult<Vec<CategoryDefinition>> {
        let categories = self.categories.read();
        Ok(categories
            .values()
            .filter(|category| category.subsystem_id == subsystem_id)
            .cloned()
            .collect())
    }

    async fn get_child_categories(&self, parent_id: &str) -> ActorCoreResult<Vec<CategoryDefinition>> {
        let categories = self.categories.read();
        Ok(categories
            .values()
            .filter(|category| category.parent_category.as_ref().map_or(false, |p| p == parent_id))
            .cloned()
            .collect())
    }
}

/// Category registry metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRegistryMetrics {
    pub registered_count: usize,
    pub registration_attempts: u64,
    pub unregistration_attempts: u64,
    pub lookup_attempts: u64,
}

impl Default for CategoryRegistryMetrics {
    fn default() -> Self {
        Self::load_default_metrics().unwrap_or_else(|_| {
            warn!("Failed to load category registry metrics config, using hardcoded defaults");
            Self {
                registered_count: 0,
                registration_attempts: 0,
                unregistration_attempts: 0,
                lookup_attempts: 0,
            }
        })
    }
}

impl CategoryRegistryMetrics {
    /// Load default category registry metrics from configuration
    pub fn load_default_metrics() -> ActorCoreResult<Self> {
        // Try to load from category_registry_config.yaml first
        let config_path = std::path::Path::new("configs/category_registry_config.yaml");
            
        if config_path.exists() {
            match Self::load_metrics_from_file(config_path) {
                Ok(metrics) => return Ok(metrics),
                Err(e) => {
                    warn!("Failed to load category registry metrics from file: {}. Using hardcoded defaults.", e);
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
        let config: CategoryRegistryConfig = serde_yaml::from_str(&content)?;
        Ok(Self {
            registered_count: config.default_registered_count,
            registration_attempts: config.default_registration_attempts,
            unregistration_attempts: config.default_unregistration_attempts,
            lookup_attempts: config.default_lookup_attempts,
        })
    }
}

/// Category registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRegistryConfig {
    pub enable_validation: bool,
    pub enable_caching: bool,
    pub max_categories: usize,
    pub cache_ttl_seconds: u64,
    pub default_registered_count: usize,
    pub default_registration_attempts: u64,
    pub default_unregistration_attempts: u64,
    pub default_lookup_attempts: u64,
}

impl CategoryRegistryConfig {
    /// Load category registry configuration from config file
    pub fn load_config() -> ActorCoreResult<Self> {
        // Try to load from category_registry_config.yaml first
        let config_path = std::path::Path::new("configs/category_registry_config.yaml");
            
        if config_path.exists() {
            match Self::load_config_from_file(config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    warn!("Failed to load category registry config from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_config())
    }

    /// Load configuration from file
    fn load_config_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: CategoryRegistryConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default configuration
    fn get_default_config() -> Self {
        Self {
            enable_validation: true,
            enable_caching: true,
            max_categories: 1000,
            cache_ttl_seconds: 3600,
            default_registered_count: 0,
            default_registration_attempts: 0,
            default_unregistration_attempts: 0,
            default_lookup_attempts: 0,
        }
    }
}
