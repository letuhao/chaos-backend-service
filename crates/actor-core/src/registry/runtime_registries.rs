//! Runtime Registry System
//!
//! This module provides runtime registries for resources, categories, and tags,
//! enabling Actor Core to be a pure hub without hardcoded game data.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tracing::warn;
use crate::ActorCoreResult;

/// Resource definition for different resource types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDefinition {
    /// Resource identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Resource category
    pub category: String,
    /// Resource type
    pub resource_type: ResourceType,
    /// Base value calculation
    pub base_value: f64,
    /// Minimum value
    pub min_value: f64,
    /// Maximum value
    pub max_value: f64,
    /// Regeneration rate per second
    pub regen_rate: f64,
    /// Regeneration type
    pub regen_type: RegenType,
    /// Dependencies on other resources
    pub dependencies: Vec<String>,
    /// Additional tags
    pub tags: Vec<String>,
    /// Which subsystem registered this
    pub subsystem_id: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Resource type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResourceType {
    Health,
    Energy,
    Mana,
    Stamina,
    Sanity,
    Experience,
    Currency,
    Custom(String),
}

/// Regeneration type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegenType {
    Passive,
    Active,
    Conditional,
    None,
}

/// Category definition for different category types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDefinition {
    /// Category identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Parent category (for hierarchical categories)
    pub parent_category: Option<String>,
    /// Additional tags
    pub tags: Vec<String>,
    /// Which subsystem registered this
    pub subsystem_id: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Tag definition for different tag types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDefinition {
    /// Tag identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Tag type (resource, category, action, etc.)
    pub tag_type: String,
    /// Optional color for UI purposes
    pub color: Option<String>,
    /// Which subsystem registered this
    pub subsystem_id: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Resource registry trait for managing resource definitions
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

/// Category registry trait for managing category definitions
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
}

/// Tag registry trait for managing tag definitions
#[async_trait]
pub trait TagRegistry: Send + Sync {
    /// Register a tag
    async fn register_tag(&self, tag: TagDefinition) -> ActorCoreResult<()>;
    
    /// Get all registered tags
    async fn get_all_tags(&self) -> ActorCoreResult<Vec<TagDefinition>>;
    
    /// Get tags by type
    async fn get_tags_by_type(&self, tag_type: &str) -> ActorCoreResult<Vec<TagDefinition>>;
    
    /// Get tag by ID
    async fn get_tag(&self, id: &str) -> ActorCoreResult<Option<TagDefinition>>;
    
    /// Check if tag exists
    async fn has_tag(&self, id: &str) -> ActorCoreResult<bool>;
    
    /// Unregister tag
    async fn unregister_tag(&self, id: &str) -> ActorCoreResult<()>;
    
    /// Get tags by subsystem
    async fn get_tags_by_subsystem(&self, subsystem_id: &str) -> ActorCoreResult<Vec<TagDefinition>>;
}

/// Default implementation of ResourceRegistry
pub struct ResourceRegistryImpl {
    resources: Arc<tokio::sync::RwLock<HashMap<String, ResourceDefinition>>>,
}

impl ResourceRegistryImpl {
    pub fn new() -> Self {
        Self {
            resources: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    pub fn new_with_config(_config: &ResourceRegistryConfig) -> Self {
        Self {
            resources: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ResourceRegistry for ResourceRegistryImpl {
    async fn register_resource(&self, resource: ResourceDefinition) -> ActorCoreResult<()> {
        let mut resources = self.resources.write().await;
        resources.insert(resource.id.clone(), resource);
        Ok(())
    }
    
    async fn get_all_resources(&self) -> ActorCoreResult<Vec<ResourceDefinition>> {
        let resources = self.resources.read().await;
        Ok(resources.values().cloned().collect())
    }
    
    async fn get_resources_by_category(&self, category: &str) -> ActorCoreResult<Vec<ResourceDefinition>> {
        let resources = self.resources.read().await;
        Ok(resources
            .values()
            .filter(|r| r.category == category)
            .cloned()
            .collect())
    }
    
    async fn get_resource(&self, id: &str) -> ActorCoreResult<Option<ResourceDefinition>> {
        let resources = self.resources.read().await;
        Ok(resources.get(id).cloned())
    }
    
    async fn has_resource(&self, id: &str) -> ActorCoreResult<bool> {
        let resources = self.resources.read().await;
        Ok(resources.contains_key(id))
    }
    
    async fn get_resource_categories(&self) -> ActorCoreResult<Vec<String>> {
        let resources = self.resources.read().await;
        let mut categories: Vec<String> = resources
            .values()
            .map(|r| r.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        Ok(categories)
    }
    
    async fn unregister_resource(&self, id: &str) -> ActorCoreResult<()> {
        let mut resources = self.resources.write().await;
        resources.remove(id);
        Ok(())
    }
    
    async fn get_resources_by_subsystem(&self, subsystem_id: &str) -> ActorCoreResult<Vec<ResourceDefinition>> {
        let resources = self.resources.read().await;
        Ok(resources
            .values()
            .filter(|r| r.subsystem_id == subsystem_id)
            .cloned()
            .collect())
    }
}

/// Default implementation of CategoryRegistry
pub struct CategoryRegistryImpl {
    categories: Arc<tokio::sync::RwLock<HashMap<String, CategoryDefinition>>>,
}

impl CategoryRegistryImpl {
    pub fn new() -> Self {
        Self {
            categories: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    pub fn new_with_config(_config: &CategoryRegistryConfig) -> Self {
        Self {
            categories: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl CategoryRegistry for CategoryRegistryImpl {
    async fn register_category(&self, category: CategoryDefinition) -> ActorCoreResult<()> {
        let mut categories = self.categories.write().await;
        categories.insert(category.id.clone(), category);
        Ok(())
    }
    
    async fn get_all_categories(&self) -> ActorCoreResult<Vec<CategoryDefinition>> {
        let categories = self.categories.read().await;
        Ok(categories.values().cloned().collect())
    }
    
    async fn get_category(&self, id: &str) -> ActorCoreResult<Option<CategoryDefinition>> {
        let categories = self.categories.read().await;
        Ok(categories.get(id).cloned())
    }
    
    async fn has_category(&self, id: &str) -> ActorCoreResult<bool> {
        let categories = self.categories.read().await;
        Ok(categories.contains_key(id))
    }
    
    async fn get_category_tags(&self, category_id: &str) -> ActorCoreResult<Vec<String>> {
        let categories = self.categories.read().await;
        if let Some(category) = categories.get(category_id) {
            Ok(category.tags.clone())
        } else {
            Ok(vec![])
        }
    }
    
    async fn unregister_category(&self, id: &str) -> ActorCoreResult<()> {
        let mut categories = self.categories.write().await;
        categories.remove(id);
        Ok(())
    }
    
    async fn get_categories_by_subsystem(&self, subsystem_id: &str) -> ActorCoreResult<Vec<CategoryDefinition>> {
        let categories = self.categories.read().await;
        Ok(categories
            .values()
            .filter(|c| c.subsystem_id == subsystem_id)
            .cloned()
            .collect())
    }
}

/// Default implementation of TagRegistry
pub struct TagRegistryImpl {
    tags: Arc<tokio::sync::RwLock<HashMap<String, TagDefinition>>>,
}

impl TagRegistryImpl {
    pub fn new() -> Self {
        Self {
            tags: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    pub fn new_with_config(_config: &TagRegistryConfig) -> Self {
        Self {
            tags: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl TagRegistry for TagRegistryImpl {
    async fn register_tag(&self, tag: TagDefinition) -> ActorCoreResult<()> {
        let mut tags = self.tags.write().await;
        tags.insert(tag.id.clone(), tag);
        Ok(())
    }
    
    async fn get_all_tags(&self) -> ActorCoreResult<Vec<TagDefinition>> {
        let tags = self.tags.read().await;
        Ok(tags.values().cloned().collect())
    }
    
    async fn get_tags_by_type(&self, tag_type: &str) -> ActorCoreResult<Vec<TagDefinition>> {
        let tags = self.tags.read().await;
        Ok(tags
            .values()
            .filter(|t| t.tag_type == tag_type)
            .cloned()
            .collect())
    }
    
    async fn get_tag(&self, id: &str) -> ActorCoreResult<Option<TagDefinition>> {
        let tags = self.tags.read().await;
        Ok(tags.get(id).cloned())
    }
    
    async fn has_tag(&self, id: &str) -> ActorCoreResult<bool> {
        let tags = self.tags.read().await;
        Ok(tags.contains_key(id))
    }
    
    async fn unregister_tag(&self, id: &str) -> ActorCoreResult<()> {
        let mut tags = self.tags.write().await;
        tags.remove(id);
        Ok(())
    }
    
    async fn get_tags_by_subsystem(&self, subsystem_id: &str) -> ActorCoreResult<Vec<TagDefinition>> {
        let tags = self.tags.read().await;
        Ok(tags
            .values()
            .filter(|t| t.subsystem_id == subsystem_id)
            .cloned()
            .collect())
    }
}

/// Registry manager to coordinate all registries
pub struct RegistryManager {
    resource_registry: Arc<dyn ResourceRegistry>,
    category_registry: Arc<dyn CategoryRegistry>,
    tag_registry: Arc<dyn TagRegistry>,
}

impl RegistryManager {
    pub fn new() -> Self {
        Self::new_with_config().unwrap_or_else(|_| {
            warn!("Failed to load runtime registry config, using hardcoded defaults");
            Self {
                resource_registry: Arc::new(ResourceRegistryImpl::new()),
                category_registry: Arc::new(CategoryRegistryImpl::new()),
                tag_registry: Arc::new(TagRegistryImpl::new()),
            }
        })
    }

    pub fn new_with_config() -> ActorCoreResult<Self> {
        let config = RuntimeRegistryConfig::load_config()?;
        
        Ok(Self {
            resource_registry: Arc::new(ResourceRegistryImpl::new_with_config(&config.resource_config)),
            category_registry: Arc::new(CategoryRegistryImpl::new_with_config(&config.category_config)),
            tag_registry: Arc::new(TagRegistryImpl::new_with_config(&config.tag_config)),
        })
    }
    
    pub fn get_resource_registry(&self) -> Arc<dyn ResourceRegistry> {
        self.resource_registry.clone()
    }
    
    pub fn get_category_registry(&self) -> Arc<dyn CategoryRegistry> {
        self.category_registry.clone()
    }
    
    pub fn get_tag_registry(&self) -> Arc<dyn TagRegistry> {
        self.tag_registry.clone()
    }
}

impl Default for RegistryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Runtime registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeRegistryConfig {
    pub resource_config: ResourceRegistryConfig,
    pub category_config: CategoryRegistryConfig,
    pub tag_config: TagRegistryConfig,
    pub enable_caching: bool,
    pub enable_statistics: bool,
    pub max_entries_per_registry: usize,
}

/// Resource registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRegistryConfig {
    pub enable_validation: bool,
    pub enable_caching: bool,
    pub max_resources: usize,
    pub cache_ttl_seconds: u64,
}

/// Category registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRegistryConfig {
    pub enable_validation: bool,
    pub enable_caching: bool,
    pub max_categories: usize,
    pub cache_ttl_seconds: u64,
}

/// Tag registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagRegistryConfig {
    pub enable_validation: bool,
    pub enable_caching: bool,
    pub max_tags: usize,
    pub cache_ttl_seconds: u64,
}

impl RuntimeRegistryConfig {
    /// Load runtime registry configuration from config file
    pub fn load_config() -> ActorCoreResult<Self> {
        // Try to load from runtime_registry_config.yaml first
        let config_path = std::path::Path::new("configs/runtime_registry_config.yaml");
            
        if config_path.exists() {
            match Self::load_config_from_file(config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    warn!("Failed to load runtime registry config from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_config())
    }

    /// Load configuration from file
    fn load_config_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: RuntimeRegistryConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default configuration
    fn get_default_config() -> Self {
        Self {
            resource_config: ResourceRegistryConfig {
                enable_validation: true,
                enable_caching: true,
                max_resources: 10000,
                cache_ttl_seconds: 3600,
            },
            category_config: CategoryRegistryConfig {
                enable_validation: true,
                enable_caching: true,
                max_categories: 1000,
                cache_ttl_seconds: 3600,
            },
            tag_config: TagRegistryConfig {
                enable_validation: true,
                enable_caching: true,
                max_tags: 5000,
                cache_ttl_seconds: 3600,
            },
            enable_caching: true,
            enable_statistics: true,
            max_entries_per_registry: 10000,
        }
    }
}
