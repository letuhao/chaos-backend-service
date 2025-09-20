# Actor Core API Reference

## 🚀 Overview

This document provides a comprehensive API reference for the refactored Actor Core system. The API is organized into logical modules and includes detailed documentation for all public interfaces.

## 📋 Table of Contents

- [Configuration Hub](#configuration-hub)
- [Runtime Registry](#runtime-registry)
- [Builder Pattern](#builder-pattern)
- [Types and Enums](#types-and-enums)
- [Error Handling](#error-handling)
- [Examples](#examples)

## 🔧 Configuration Hub

### ConfigurationManager

The main configuration management interface.

```rust
pub struct ConfigurationManager {
    // ... private fields
}

impl ConfigurationManager {
    /// Initialize the configuration manager
    pub async fn initialize(&self) -> ActorCoreResult<()>
    
    /// Get configuration value for a specific key
    pub async fn get_config(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>>
    
    /// Set configuration value (for runtime changes)
    pub async fn set_config(&self, category: &str, key: &str, value: ConfigurationValue) -> ActorCoreResult<()>
    
    /// Get all configuration for a category
    pub async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>>
    
    /// Get all configuration
    pub async fn get_all_config(&self) -> ActorCoreResult<HashMap<String, HashMap<String, ConfigurationValue>>>
    
    /// Refresh all configuration
    pub async fn refresh_config(&self) -> ActorCoreResult<()>
    
    /// Save configuration (for persistence)
    pub async fn save_configs(&self) -> ActorCoreResult<()>
    
    /// Get system health status
    pub async fn get_health_status(&self) -> ConfigurationHealthStatus
}
```

### ConfigurationProvider

Trait for subsystems to provide configuration data.

```rust
#[async_trait]
pub trait ConfigurationProvider: Send + Sync {
    /// Unique identifier for this configuration provider
    fn provider_id(&self) -> &str;
    
    /// Priority for this provider (higher = more important)
    fn priority(&self) -> i64;
    
    /// Get configuration categories this provider supports
    fn get_supported_categories(&self) -> Vec<String>;
    
    /// Get configuration value for a specific key
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>>;
    
    /// Get all configuration values for a category
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>>;
    
    /// Get merge rule for a specific configuration key
    fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule>;
    
    /// Validate configuration data
    async fn validate_config(&self) -> ActorCoreResult<()>;
}
```

### ConfigurationRegistry

Registry for managing configuration providers.

```rust
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
```

### ConfigurationCombiner

Combiner for merging configuration values.

```rust
#[async_trait]
pub trait ConfigurationCombiner: Send + Sync {
    /// Get merge rule for a specific configuration key
    async fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule>;
    
    /// Set merge rule for a specific configuration key
    async fn set_merge_rule(&self, category: &str, key: &str, rule: ConfigurationMergeRule) -> ActorCoreResult<()>;
    
    /// Merge configuration values from multiple providers
    async fn merge_values(&self, values: Vec<ConfigurationValue>, rule: &ConfigurationMergeRule) -> ActorCoreResult<ConfigurationValue>;
    
    /// Validate merged configuration
    async fn validate_merged_config(&self, config: &ConfigurationValue) -> ActorCoreResult<()>;
    
    /// Get combiner metrics
    async fn get_metrics(&self) -> ConfigurationCombinerMetrics;
}
```

### ConfigurationAggregator

Aggregator for processing configurations from all providers.

```rust
#[async_trait]
pub trait ConfigurationAggregator: Send + Sync {
    /// Get configuration value for a specific key
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>>;
    
    /// Get all configuration values for a category
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>>;
    
    /// Get all configuration values
    async fn get_all_config(&self) -> ActorCoreResult<HashMap<String, HashMap<String, ConfigurationValue>>>;
    
    /// Refresh configuration from all providers
    async fn refresh_config(&self) -> ActorCoreResult<()>;
    
    /// Invalidate configuration cache
    async fn invalidate_cache(&self) -> ActorCoreResult<()>;
    
    /// Get aggregator metrics
    async fn get_metrics(&self) -> ConfigurationAggregatorMetrics;
}
```

## 📊 Runtime Registry

### ResourceRegistry

Registry for managing resources.

```rust
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
```

### CategoryRegistry

Registry for managing categories.

```rust
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
```

### TagRegistry

Registry for managing tags.

```rust
#[async_trait]
pub trait TagRegistry: Send + Sync {
    /// Register a tag definition
    async fn register_tag(&self, tag: TagDefinition) -> ActorCoreResult<()>;
    
    /// Get all registered tags
    async fn get_all_tags(&self) -> ActorCoreResult<Vec<TagDefinition>>;
    
    /// Get tag by ID
    async fn get_tag(&self, id: &str) -> ActorCoreResult<Option<TagDefinition>>;
    
    /// Check if tag exists
    async fn has_tag(&self, id: &str) -> ActorCoreResult<bool>;
    
    /// Get tags by type
    async fn get_tags_by_type(&self, tag_type: &TagType) -> ActorCoreResult<Vec<TagDefinition>>;
    
    /// Unregister tag
    async fn unregister_tag(&self, id: &str) -> ActorCoreResult<()>;
    
    /// Get tags by subsystem
    async fn get_tags_by_subsystem(&self, subsystem_id: &str) -> ActorCoreResult<Vec<TagDefinition>>;
}
```

### RegistryManager

Manager for coordinating all runtime registries.

```rust
pub struct RegistryManager {
    // ... private fields
}

impl RegistryManager {
    /// Create a new registry manager
    pub fn new(
        resource_registry: Arc<dyn ResourceRegistry>,
        category_registry: Arc<dyn CategoryRegistry>,
        tag_registry: Arc<dyn TagRegistry>,
    ) -> Self
    
    /// Initialize all registries
    pub async fn initialize(&self) -> ActorCoreResult<()>
    
    /// Get resource registry
    pub fn get_resource_registry(&self) -> Arc<dyn ResourceRegistry>
    
    /// Get category registry
    pub fn get_category_registry(&self) -> Arc<dyn CategoryRegistry>
    
    /// Get tag registry
    pub fn get_tag_registry(&self) -> Arc<dyn TagRegistry>
    
    /// Get system health status
    pub async fn get_health_status(&self) -> RegistryHealthStatus
}
```

## 🏗️ Builder Pattern

### ActorCoreBuilder

Builder for the complete Actor Core system.

```rust
pub struct ActorCoreBuilder {
    // ... private fields
}

impl ActorCoreBuilder {
    /// Create a new Actor Core Builder
    pub fn new() -> Self
    
    /// Add a configuration file path
    pub fn with_config_path(self, path: PathBuf) -> Self
    
    /// Enable hot reload for configuration files
    pub fn with_hot_reload(self, enable: bool) -> Self
    
    /// Enable metrics collection
    pub fn with_metrics(self, enable: bool) -> Self
    
    /// Enable caching
    pub fn with_caching(self, enable: bool) -> Self
    
    /// Set cache size in MB
    pub fn with_cache_size(self, size_mb: usize) -> Self
    
    /// Set log level
    pub fn with_log_level(self, level: String) -> Self
    
    /// Build the Actor Core system
    pub async fn build(self) -> ActorCoreResult<ActorCoreSystem>
}
```

### ConfigurationHubBuilder

Builder for the Configuration Hub system.

```rust
pub struct ConfigurationHubBuilder {
    // ... private fields
}

impl ConfigurationHubBuilder {
    /// Create a new Configuration Hub Builder
    pub fn new() -> Self
    
    /// Add a configuration provider
    pub fn with_provider(self, provider: Arc<dyn ConfigurationProvider>) -> Self
    
    /// Add a configuration file path
    pub fn with_config_path(self, path: PathBuf) -> Self
    
    /// Enable hot reload for configuration files
    pub fn with_hot_reload(self, enable: bool) -> Self
    
    /// Enable metrics collection
    pub fn with_metrics(self, enable: bool) -> Self
    
    /// Enable caching
    pub fn with_caching(self, enable: bool) -> Self
    
    /// Set cache size in MB
    pub fn with_cache_size(self, size_mb: usize) -> Self
    
    /// Set log level
    pub fn with_log_level(self, level: String) -> Self
    
    /// Build the Configuration Hub
    pub async fn build(self) -> ActorCoreResult<ConfigurationHubSystem>
}
```

### RegistryBuilder

Builder for the Registry system.

```rust
pub struct RegistryBuilder {
    // ... private fields
}

impl RegistryBuilder {
    /// Create a new Registry Builder
    pub fn new() -> Self
    
    /// Add a resource definition
    pub fn with_resource(self, resource: ResourceDefinition) -> Self
    
    /// Add a category definition
    pub fn with_category(self, category: CategoryDefinition) -> Self
    
    /// Add a tag definition
    pub fn with_tag(self, tag: TagDefinition) -> Self
    
    /// Enable metrics collection
    pub fn with_metrics(self, enable: bool) -> Self
    
    /// Enable caching
    pub fn with_caching(self, enable: bool) -> Self
    
    /// Set cache size in MB
    pub fn with_cache_size(self, size_mb: usize) -> Self
    
    /// Set log level
    pub fn with_log_level(self, level: String) -> Self
    
    /// Build the Registry System
    pub async fn build(self) -> ActorCoreResult<RegistrySystem>
}
```

## 📋 Types and Enums

### ConfigurationValue

Configuration value with metadata.

```rust
pub struct ConfigurationValue {
    pub value: serde_json::Value,
    pub value_type: ConfigurationValueType,
    pub source_provider: String,
    pub priority: i64,
    pub timestamp: DateTime<Utc>,
    pub can_override: bool,
    pub can_merge: bool,
}
```

### ConfigurationValueType

Configuration value types.

```rust
pub enum ConfigurationValueType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
    Duration,
    Size,
    Percentage,
}
```

### ConfigurationMergeStrategy

Merge strategies for configuration values.

```rust
pub enum ConfigurationMergeStrategy {
    Override,
    Baseline,
    Sum,
    Max,
    Min,
    Average,
    Multiply,
    Intersect,
    Merge,
    Concat,
}
```

### ResourceDefinition

Resource definition for runtime registry.

```rust
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
```

### ResourceType

Resource type enumeration.

```rust
pub enum ResourceType {
    Health,
    Mana,
    Stamina,
    Experience,
    Level,
    Energy,
    Custom(String),
}
```

### RegenType

Regeneration type enumeration.

```rust
pub enum RegenType {
    Passive,
    Active,
    Conditional,
    None,
}
```

### CategoryDefinition

Category definition for runtime registry.

```rust
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
```

### TagDefinition

Tag definition for runtime registry.

```rust
pub struct TagDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub tag_type: TagType,
    pub subsystem_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### TagType

Tag type enumeration.

```rust
pub enum TagType {
    Resource,
    Category,
    Action,
    Element,
    Status,
    Buff,
    Debuff,
    Custom(String),
}
```

## ❌ Error Handling

### ActorCoreResult

Result type for Actor Core operations.

```rust
pub type ActorCoreResult<T> = Result<T, ActorCoreError>;
```

### ActorCoreError

Error types for Actor Core operations.

```rust
pub enum ActorCoreError {
    ConfigurationError(String),
    RegistryError(String),
    ValidationError(String),
    SystemError(String),
    NetworkError(String),
    DatabaseError(String),
    CacheError(String),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    YamlError(serde_yaml::Error),
    ChronoError(chrono::ParseError),
    TokioError(tokio::task::JoinError),
}
```

## 📚 Examples

### Basic Usage

```rust
use actor_core::builder::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple setup with defaults
    let actor_core = ActorCoreBuilder::new().build().await?;
    
    // Use the system
    let config_manager = actor_core.get_config_manager();
    let registry_manager = actor_core.get_registry_manager();
    
    // Get configuration
    let value = config_manager.get_config("defaults", "resources").await?;
    println!("Configuration: {:?}", value);
    
    // Get resources
    let resources = registry_manager.get_resource_registry().get_all_resources().await?;
    println!("Resources: {:?}", resources);
    
    // Shutdown when done
    actor_core.shutdown().await?;
    
    Ok(())
}
```

### Advanced Usage

```rust
use actor_core::builder::*;
use actor_core::runtime_registry::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Advanced setup with custom configuration
    let actor_core = ActorCoreBuilder::new()
        .with_config_path(PathBuf::from("configs/actor_core_defaults.yaml"))
        .with_hot_reload(true)
        .with_metrics(true)
        .with_caching(true)
        .with_cache_size(200)
        .with_log_level("debug".to_string())
        .build()
        .await?;
    
    // Register custom resources
    let custom_resource = ResourceDefinition {
        id: "energy".to_string(),
        name: "Energy".to_string(),
        description: Some("Character energy points".to_string()),
        category: "vital".to_string(),
        resource_type: ResourceType::Custom("energy".to_string()),
        base_value: 100.0,
        min_value: 0.0,
        max_value: 1000.0,
        regen_rate: 1.5,
        regen_type: RegenType::Passive,
        dependencies: vec![],
        tags: vec!["vital".to_string(), "energy".to_string()],
        subsystem_id: "custom_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    actor_core.get_registry_manager()
        .get_resource_registry()
        .register_resource(custom_resource)
        .await?;
    
    // Use the system
    let config_manager = actor_core.get_config_manager();
    let registry_manager = actor_core.get_registry_manager();
    
    // Get configuration
    let value = config_manager.get_config("defaults", "resources").await?;
    println!("Configuration: {:?}", value);
    
    // Get resources
    let resources = registry_manager.get_resource_registry().get_all_resources().await?;
    println!("Resources: {:?}", resources);
    
    // Get system health
    let health = actor_core.get_health_status().await;
    println!("System Health: {:?}", health);
    
    // Shutdown when done
    actor_core.shutdown().await?;
    
    Ok(())
}
```

### Custom Configuration Provider

```rust
use actor_core::config::*;
use std::sync::Arc;

struct CustomConfigurationProvider {
    provider_id: String,
    priority: i64,
    config_data: HashMap<String, HashMap<String, ConfigurationValue>>,
}

#[async_trait]
impl ConfigurationProvider for CustomConfigurationProvider {
    fn provider_id(&self) -> &str {
        &self.provider_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    fn get_supported_categories(&self) -> Vec<String> {
        self.config_data.keys().cloned().collect()
    }
    
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>> {
        Ok(self.config_data.get(category)?.get(key).cloned())
    }
    
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        Ok(self.config_data.get(category).cloned().unwrap_or_default())
    }
    
    fn get_merge_rule(&self, _category: &str, _key: &str) -> Option<ConfigurationMergeRule> {
        Some(ConfigurationMergeRule {
            strategy: ConfigurationMergeStrategy::Override,
            use_pipeline: false,
            default_value: None,
            validation_rules: vec![],
        })
    }
    
    async fn validate_config(&self) -> ActorCoreResult<()> {
        // Custom validation logic
        Ok(())
    }
}
```

---

**Note**: This API reference provides comprehensive documentation for all public interfaces in the refactored Actor Core system. For more detailed examples and usage patterns, see the [Examples](examples/) directory.
