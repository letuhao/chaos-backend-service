# Runtime Registry System Design

## Overview

The Runtime Registry System enables Actor Core to be a pure hub/aggregator without hardcoded resources, categories, or tags. Subsystems can register their resources, categories, and tags at runtime, and Actor Core can query these registries to provide dynamic, extensible functionality.

## Problem Statement

Currently, Actor Core has hardcoded resources like "health", "mana", "stamina" in the condition integration, which violates the principle that Actor Core should be a pure hub without hardcoded game-specific data. We need:

1. **Runtime Resource Discovery** - Query available resources at runtime
2. **Runtime Category Discovery** - Query available categories at runtime  
3. **Runtime Tag Discovery** - Query available tags at runtime
4. **Dynamic Registration** - Subsystems can register their resources/categories/tags
5. **Extensibility** - New subsystems can add new resources without modifying Actor Core

## Architecture Design

### Core Components

#### 1. ResourceRegistry
```rust
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
}
```

#### 2. CategoryRegistry
```rust
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
}
```

#### 3. TagRegistry
```rust
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
}
```

### Data Structures

#### ResourceDefinition
```rust
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
    pub subsystem_id: String, // Which subsystem registered this
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

#### CategoryDefinition
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_category: Option<String>,
    pub tags: Vec<String>,
    pub subsystem_id: String, // Which subsystem registered this
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

#### TagDefinition
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub tag_type: String, // "resource", "category", "action", etc.
    pub color: Option<String>, // For UI purposes
    pub subsystem_id: String, // Which subsystem registered this
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

### Integration with Actor Core

#### 1. Registry Manager
```rust
pub struct RegistryManager {
    resource_registry: Arc<dyn ResourceRegistry>,
    category_registry: Arc<dyn CategoryRegistry>,
    tag_registry: Arc<dyn TagRegistry>,
}

impl RegistryManager {
    pub fn new() -> Self {
        Self {
            resource_registry: Arc::new(ResourceRegistryImpl::new()),
            category_registry: Arc::new(CategoryRegistryImpl::new()),
            tag_registry: Arc::new(TagRegistryImpl::new()),
        }
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
```

#### 2. Subsystem Registration Interface
```rust
pub trait RegistryAwareSubsystem: Subsystem {
    /// Register resources provided by this subsystem
    async fn register_resources(&self, registry: Arc<dyn ResourceRegistry>) -> ActorCoreResult<()>;
    
    /// Register categories provided by this subsystem
    async fn register_categories(&self, registry: Arc<dyn CategoryRegistry>) -> ActorCoreResult<()>;
    
    /// Register tags provided by this subsystem
    async fn register_tags(&self, registry: Arc<dyn TagRegistry>) -> ActorCoreResult<()>;
    
    /// Unregister all resources/categories/tags from this subsystem
    async fn unregister_all(&self, registry_manager: &RegistryManager) -> ActorCoreResult<()>;
}
```

### Usage Examples

#### 1. Subsystem Registration
```rust
// In a game subsystem (e.g., Magic System)
impl RegistryAwareSubsystem for MagicSubsystem {
    async fn register_resources(&self, registry: Arc<dyn ResourceRegistry>) -> ActorCoreResult<()> {
        // Register magic-specific resources
        registry.register_resource(ResourceDefinition {
            id: "mana".to_string(),
            name: "Mana".to_string(),
            category: "magic".to_string(),
            resource_type: ResourceType::Energy,
            base_value: 100.0,
            min_value: 0.0,
            max_value: 1000.0,
            regen_rate: 5.0,
            regen_type: RegenType::Passive,
            dependencies: vec![],
            tags: vec!["magic".to_string(), "energy".to_string()],
            subsystem_id: self.system_id().to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }).await?;
        
        Ok(())
    }
    
    async fn register_categories(&self, registry: Arc<dyn CategoryRegistry>) -> ActorCoreResult<()> {
        // Register magic categories
        registry.register_category(CategoryDefinition {
            id: "magic".to_string(),
            name: "Magic".to_string(),
            description: Some("Magic-related resources and abilities".to_string()),
            parent_category: None,
            tags: vec!["magic".to_string(), "spell".to_string()],
            subsystem_id: self.system_id().to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }).await?;
        
        Ok(())
    }
}
```

#### 2. Runtime Query
```rust
// In condition integration
async fn get_available_resources(registry: Arc<dyn ResourceRegistry>) -> ActorCoreResult<Vec<String>> {
    let resources = registry.get_all_resources().await?;
    Ok(resources.into_iter().map(|r| r.id).collect())
}

// In UI or other systems
async fn display_resources(registry: Arc<dyn ResourceRegistry>) -> ActorCoreResult<()> {
    let resources = registry.get_all_resources().await?;
    for resource in resources {
        println!("Resource: {} - {}", resource.id, resource.name);
    }
    Ok(())
}
```

#### 3. Dynamic Condition Building
```rust
// Build conditions dynamically based on available resources
async fn build_health_condition(registry: Arc<dyn ResourceRegistry>) -> ActorCoreResult<ConditionConfig> {
    if registry.has_resource("health").await? {
        Ok(ConditionConfig {
            condition_id: "has_health".to_string(),
            function_name: "get_actor_resource".to_string(),
            parameters: vec![ConditionParameter::String("health".to_string())],
            operator: ConditionOperator::GreaterThan,
            value: ConditionValue::Float(0.0),
        })
    } else {
        Err(ActorCoreError::ResourceNotFound("health".to_string()))
    }
}
```

## Implementation Plan

### Phase 1: Core Registry Traits and Implementations
1. Create `ResourceRegistry` trait and `ResourceRegistryImpl`
2. Create `CategoryRegistry` trait and `CategoryRegistryImpl`  
3. Create `TagRegistry` trait and `TagRegistryImpl`
4. Create `RegistryManager` to coordinate all registries

### Phase 2: Integration with Actor Core
1. Add `RegistryManager` to Actor Core
2. Update `PluginRegistry` to support `RegistryAwareSubsystem`
3. Add registry discovery methods to Actor Core

### Phase 3: Update Condition Integration
1. Remove hardcoded resources from condition integration
2. Update data providers to use runtime registries
3. Add dynamic condition building based on available resources

### Phase 4: Examples and Documentation
1. Create examples showing dynamic resource registration
2. Update documentation with runtime registry usage
3. Create migration guide from hardcoded to dynamic approach

## Benefits

1. **✅ Pure Hub Architecture** - Actor Core has no hardcoded game data
2. **✅ Runtime Extensibility** - New subsystems can add resources without code changes
3. **✅ Dynamic Discovery** - Systems can query available resources at runtime
4. **✅ Plugin Architecture** - Subsystems can be loaded/unloaded dynamically
5. **✅ Maintainability** - No hardcoded values to maintain
6. **✅ Flexibility** - Different game modes can have different resource sets

## Migration Strategy

1. **Backward Compatibility** - Keep existing hardcoded resources as fallback
2. **Gradual Migration** - Migrate subsystems one by one to use registries
3. **Deprecation** - Mark hardcoded approaches as deprecated
4. **Removal** - Remove hardcoded resources after migration is complete

This design ensures Actor Core remains a pure hub while providing the flexibility needed for dynamic, extensible game systems.
