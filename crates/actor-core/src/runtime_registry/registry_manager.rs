//! Registry manager for coordinating all runtime registries

use std::sync::Arc;
use tracing::{info, warn};
use serde::{Deserialize, Serialize};

use crate::runtime_registry::resource_registry::*;
use crate::runtime_registry::category_registry::*;
use crate::runtime_registry::tag_registry::*;
use crate::registry::runtime_registries::{ResourceRegistryConfig, TagRegistryConfig};
use crate::config::manager::ConfigurationManager;
use crate::ActorCoreResult;

/// Registry manager for coordinating all runtime registries
pub struct RegistryManager {
    resource_registry: Arc<dyn ResourceRegistry>,
    category_registry: Arc<dyn CategoryRegistry>,
    tag_registry: Arc<dyn TagRegistry>,
    config_manager: Arc<ConfigurationManager>,
}

impl RegistryManager {
    pub fn new(
        resource_registry: Arc<dyn ResourceRegistry>,
        category_registry: Arc<dyn CategoryRegistry>,
        tag_registry: Arc<dyn TagRegistry>,
        config_manager: Arc<ConfigurationManager>,
    ) -> Self {
        Self {
            resource_registry,
            category_registry,
            tag_registry,
            config_manager,
        }
    }

    pub fn new_with_config() -> ActorCoreResult<Self> {
        let _config = RegistryManagerConfig::load_config()?;
        
        // Create registries with configuration
        let resource_registry = Arc::new(ResourceRegistryImpl::new());
        let category_registry = Arc::new(CategoryRegistryImpl::new_with_config()?);
        let tag_registry = Arc::new(TagRegistryImpl::new());
        
        // Create configuration manager (simplified for now)
        let registry = Arc::new(crate::config::registry::ConfigurationRegistryImpl::new());
        let combiner = Arc::new(crate::config::combiner::ConfigurationCombinerImpl::new());
        let aggregator = Arc::new(crate::config::aggregator::ConfigurationAggregatorImpl::new(registry.clone(), combiner.clone()));
        let loader = Arc::new(crate::config::loader::ConfigurationLoader::new(registry, combiner, aggregator));
        let config_manager = Arc::new(ConfigurationManager::new(
            Arc::new(crate::config::registry::ConfigurationRegistryImpl::new()),
            Arc::new(crate::config::combiner::ConfigurationCombinerImpl::new()),
            Arc::new(crate::config::aggregator::ConfigurationAggregatorImpl::new(
                Arc::new(crate::config::registry::ConfigurationRegistryImpl::new()),
                Arc::new(crate::config::combiner::ConfigurationCombinerImpl::new()),
            )),
            loader,
        ));
        
        Ok(Self {
            resource_registry,
            category_registry,
            tag_registry,
            config_manager,
        })
    }

    /// Initialize all registries
    pub async fn initialize(&self) -> ActorCoreResult<()> {
        info!("Initializing Registry Manager");
        
        // Load default resources, categories, and tags
        self.load_default_definitions().await?;
        
        info!("Registry Manager initialized successfully");
        Ok(())
    }

    /// Load default definitions for all registries
    async fn load_default_definitions(&self) -> ActorCoreResult<()> {
        // Load default resources
        self.load_default_resources().await?;
        
        // Load default categories
        self.load_default_categories().await?;
        
        // Load default tags
        self.load_default_tags().await?;
        
        Ok(())
    }

    /// Load default resource definitions from configuration
    async fn load_default_resources(&self) -> ActorCoreResult<()> {
        let config = self.config_manager.get_category_config("default_resources").await?;
        
        let resources_config = config.get("resources")
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Missing 'resources' in default_resources configuration".to_string()
            ))?;
        
        let resources_obj = resources_config.value.as_object()
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Invalid resources configuration: 'resources' must be an object".to_string()
            ))?;
        
        for (resource_id, resource_config) in resources_obj {
            let resource_obj = resource_config.as_object()
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Invalid resource configuration for '{}': must be an object", resource_id)
                ))?;
            
            // Load resource type from configuration instead of hardcoding
            let resource_type_str = resource_obj.get("resource_type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Missing 'resource_type' for resource '{}'", resource_id)
                ))?;
            
            let resource_type = self.get_resource_type_from_config(resource_type_str).await?;
            
            // Load regen type from configuration instead of hardcoding
            let regen_type_str = resource_obj.get("regen_type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Missing 'regen_type' for resource '{}'", resource_id)
                ))?;
            
            let regen_type = self.get_regen_type_from_config(regen_type_str).await?;
            
            let tags = resource_obj.get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect())
                .unwrap_or_else(Vec::new);
            
            let dependencies = resource_obj.get("dependencies")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect())
                .unwrap_or_else(Vec::new);
            
            let resource = ResourceDefinition {
                id: resource_obj.get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                        format!("Missing 'id' for resource '{}'", resource_id)
                    ))?
                    .to_string(),
                name: resource_obj.get("name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                        format!("Missing 'name' for resource '{}'", resource_id)
                    ))?
                    .to_string(),
                description: resource_obj.get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                category: resource_obj.get("category")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                        format!("Missing 'category' for resource '{}'", resource_id)
                    ))?
                    .to_string(),
                resource_type,
                base_value: resource_obj.get("base_value")
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                        format!("Missing or invalid 'base_value' for resource '{}'", resource_id)
                    ))?,
                min_value: resource_obj.get("min_value")
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                        format!("Missing or invalid 'min_value' for resource '{}'", resource_id)
                    ))?,
                max_value: resource_obj.get("max_value")
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                        format!("Missing or invalid 'max_value' for resource '{}'", resource_id)
                    ))?,
                regen_rate: resource_obj.get("regen_rate")
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                        format!("Missing or invalid 'regen_rate' for resource '{}'", resource_id)
                    ))?,
                regen_type,
                dependencies,
                tags,
                subsystem_id: resource_obj.get("subsystem_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&self.get_default_subsystem_id())
                    .to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            
            self.resource_registry.register_resource(resource).await?;
        }

        Ok(())
    }

    /// Load default category definitions from configuration
    async fn load_default_categories(&self) -> ActorCoreResult<()> {
        let config = self.config_manager.get_category_config("default_categories").await?;
        
        let categories_config = config.get("categories")
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Missing 'categories' in default_categories configuration".to_string()
            ))?;
        
        let categories_obj = categories_config.value.as_object()
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Invalid categories configuration: 'categories' must be an object".to_string()
            ))?;
        
        for (category_id, category_config) in categories_obj {
            let category_obj = category_config.as_object()
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Invalid category configuration for '{}': must be an object", category_id)
                ))?;
            
            let tags = category_obj.get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect())
                .unwrap_or_else(Vec::new);
            
            let category = CategoryDefinition {
                id: category_obj.get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                        format!("Missing 'id' for category '{}'", category_id)
                    ))?
                    .to_string(),
                name: category_obj.get("name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                        format!("Missing 'name' for category '{}'", category_id)
                    ))?
                    .to_string(),
                description: category_obj.get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                parent_category: category_obj.get("parent_category")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                tags,
                subsystem_id: category_obj.get("subsystem_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&self.get_default_subsystem_id())
                    .to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            
            self.category_registry.register_category(category).await?;
        }

        Ok(())
    }

    /// Load default tag definitions from configuration
    async fn load_default_tags(&self) -> ActorCoreResult<()> {
        let config = self.config_manager.get_category_config("default_tags").await?;
        
        let tags_config = config.get("tags")
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Missing 'tags' in default_tags configuration".to_string()
            ))?;
        
        let tags_obj = tags_config.value.as_object()
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Invalid tags configuration: 'tags' must be an object".to_string()
            ))?;
        
        for (tag_id, tag_config) in tags_obj {
            let tag_obj = tag_config.as_object()
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Invalid tag configuration for '{}': must be an object", tag_id)
                ))?;
            
            // Load tag type from configuration instead of hardcoding
            let tag_type_str = tag_obj.get("category")
                .and_then(|v| v.as_str())
                .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                    format!("Missing 'category' for tag '{}'", tag_id)
                ))?;
            
            let tag_type = self.get_tag_type_from_config(tag_type_str).await?;
            
            let tag = TagDefinition {
                id: tag_obj.get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                        format!("Missing 'id' for tag '{}'", tag_id)
                    ))?
                    .to_string(),
                name: tag_obj.get("name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                        format!("Missing 'name' for tag '{}'", tag_id)
                    ))?
                    .to_string(),
                description: tag_obj.get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                tag_type,
                subsystem_id: tag_obj.get("subsystem_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&self.get_default_subsystem_id())
                    .to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            
            self.tag_registry.register_tag(tag).await?;
        }

        Ok(())
    }

    /// Get resource type from configuration
    async fn get_resource_type_from_config(&self, resource_type_str: &str) -> ActorCoreResult<ResourceType> {
        // Validate that the resource type exists in configuration
        let config = self.config_manager.get_category_config("resource_types").await?;
        
        let type_definitions = config.get("definitions")
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Missing 'definitions' in resource_types configuration".to_string()
            ))?;
        
        let definitions_obj = type_definitions.value.as_object()
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Invalid resource_types configuration: 'definitions' must be an object".to_string()
            ))?;
        
        // Check if the resource type is defined in configuration
        if definitions_obj.contains_key(resource_type_str) {
            // All resource types are treated as Custom to avoid hardcoding
            Ok(ResourceType::Custom(resource_type_str.to_string()))
        } else {
            Err(crate::ActorCoreError::ConfigurationError(
                format!("Resource type '{}' is not defined in configuration", resource_type_str)
            ))
        }
    }

    /// Get regen type from configuration
    async fn get_regen_type_from_config(&self, regen_type_str: &str) -> ActorCoreResult<RegenType> {
        // Validate that the regen type exists in configuration
        let config = self.config_manager.get_category_config("regen_types").await?;
        
        let type_definitions = config.get("definitions")
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Missing 'definitions' in regen_types configuration".to_string()
            ))?;
        
        let definitions_obj = type_definitions.value.as_object()
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Invalid regen_types configuration: 'definitions' must be an object".to_string()
            ))?;
        
        // Check if the regen type is defined in configuration
        if definitions_obj.contains_key(regen_type_str) {
            // All regen types are treated as Custom to avoid hardcoding
            Ok(RegenType::Custom(regen_type_str.to_string()))
        } else {
            Err(crate::ActorCoreError::ConfigurationError(
                format!("Regen type '{}' is not defined in configuration", regen_type_str)
            ))
        }
    }

    /// Get tag type from configuration
    async fn get_tag_type_from_config(&self, tag_type_str: &str) -> ActorCoreResult<TagType> {
        // Validate that the tag type exists in configuration
        let config = self.config_manager.get_category_config("tag_types").await?;
        
        let type_definitions = config.get("definitions")
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Missing 'definitions' in tag_types configuration".to_string()
            ))?;
        
        let definitions_obj = type_definitions.value.as_object()
            .ok_or_else(|| crate::ActorCoreError::ConfigurationError(
                "Invalid tag_types configuration: 'definitions' must be an object".to_string()
            ))?;
        
        // Check if the tag type is defined in configuration
        if definitions_obj.contains_key(tag_type_str) {
            // All tag types are treated as Custom to avoid hardcoding
            Ok(TagType::Custom(tag_type_str.to_string()))
        } else {
            Err(crate::ActorCoreError::ConfigurationError(
                format!("Tag type '{}' is not defined in configuration", tag_type_str)
            ))
        }
    }

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

    /// Get system health status
    pub async fn get_health_status(&self) -> ActorCoreResult<RegistryHealthStatus> {
        let resource_metrics = self.resource_registry.get_all_resources().await?;
        let category_metrics = self.category_registry.get_all_categories().await?;
        let tag_metrics = self.tag_registry.get_all_tags().await?;

        Ok(RegistryHealthStatus {
            resource_count: resource_metrics.len(),
            category_count: category_metrics.len(),
            tag_count: tag_metrics.len(),
            total_definitions: resource_metrics.len() + category_metrics.len() + tag_metrics.len(),
        })
    }

    /// Get default subsystem ID from configuration
    fn get_default_subsystem_id(&self) -> String {
        RegistryManagerConfig::load_config().unwrap_or_else(|_| {
            warn!("Failed to load registry manager config, using hardcoded defaults");
            RegistryManagerConfig::get_default_config()
        }).default_subsystem_id
    }
}

/// Registry health status
#[derive(Debug, Clone)]
pub struct RegistryHealthStatus {
    pub resource_count: usize,
    pub category_count: usize,
    pub tag_count: usize,
    pub total_definitions: usize,
}

/// Registry manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryManagerConfig {
    pub default_subsystem_id: String,
    pub resource_config: ResourceRegistryConfig,
    pub category_config: CategoryRegistryConfig,
    pub tag_config: TagRegistryConfig,
    pub enable_validation: bool,
    pub enable_caching: bool,
    pub max_definitions_per_registry: usize,
}

impl RegistryManagerConfig {
    /// Load registry manager configuration from config file
    pub fn load_config() -> ActorCoreResult<Self> {
        // Try to load from registry_manager_config.yaml first
        let config_path = std::path::Path::new("configs/registry_manager_config.yaml");
            
        if config_path.exists() {
            match Self::load_config_from_file(config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    warn!("Failed to load registry manager config from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_config())
    }

    /// Load configuration from file
    fn load_config_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: RegistryManagerConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default configuration
    fn get_default_config() -> Self {
        Self {
            default_subsystem_id: "actor_core".to_string(),
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
                default_registered_count: 0,
                default_registration_attempts: 0,
                default_unregistration_attempts: 0,
                default_lookup_attempts: 0,
            },
            tag_config: TagRegistryConfig {
                enable_validation: true,
                enable_caching: true,
                max_tags: 5000,
                cache_ttl_seconds: 3600,
            },
            enable_validation: true,
            enable_caching: true,
            max_definitions_per_registry: 10000,
        }
    }
}
