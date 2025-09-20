# Actor Core Migration Guide

## 🚀 Overview

This guide helps you migrate from the old Actor Core system to the new refactored system. The refactor introduces a modern, plugin-based architecture that eliminates hardcoded values and provides dynamic configuration management.

## 📋 Migration Checklist

### Phase 1: Preparation
- [ ] Review the new architecture
- [ ] Identify hardcoded values in your code
- [ ] Plan configuration file structure
- [ ] Update dependencies

### Phase 2: Configuration Migration
- [ ] Create configuration files
- [ ] Move hardcoded values to configuration
- [ ] Set up environment variables
- [ ] Configure database settings

### Phase 3: Code Migration
- [ ] Update imports
- [ ] Replace hardcoded values with configuration access
- [ ] Update registry usage
- [ ] Implement builder pattern

### Phase 4: Testing
- [ ] Update unit tests
- [ ] Update integration tests
- [ ] Run performance tests
- [ ] Validate configuration

### Phase 5: Deployment
- [ ] Deploy configuration files
- [ ] Set environment variables
- [ ] Monitor system health
- [ ] Validate functionality

## 🔧 Step-by-Step Migration

### Step 1: Update Dependencies

#### Old System
```rust
use actor_core::prelude::*;
use actor_core::ActorCoreResult;
```

#### New System
```rust
use actor_core::builder::*;
use actor_core::config::*;
use actor_core::runtime_registry::*;
use actor_core::ActorCoreResult;
```

### Step 2: Replace Hardcoded Values

#### Old System
```rust
// Hardcoded values
const MAX_ACTORS: usize = 10000;
const CACHE_TTL: u64 = 3600;
const HEALTH_BASE: f64 = 100.0;
```

#### New System
```rust
// Configuration-based values
let config_manager = actor_core.get_config_manager();
let max_actors = config_manager.get_config("performance_thresholds", "max_actors").await?;
let cache_ttl = config_manager.get_config("timeouts", "cache_ttl").await?;
let health_base = config_manager.get_config("defaults", "resources").await?;
```

### Step 3: Update System Initialization

#### Old System
```rust
// Old initialization
let cache = ServiceFactory::create_cache()?;
let plugin_registry = ServiceFactory::create_plugin_registry();
let combiner_registry = ServiceFactory::create_combiner_registry();
let cap_layer_registry = ServiceFactory::create_cap_layer_registry();
let caps_provider = ServiceFactory::create_caps_provider(cap_layer_registry);
let aggregator = ServiceFactory::create_aggregator(
    plugin_registry,
    combiner_registry,
    caps_provider,
    cache,
);
```

#### New System
```rust
// New initialization with builder pattern
let actor_core = ActorCoreBuilder::new()
    .with_config_path(PathBuf::from("configs/actor_core_defaults.yaml"))
    .with_hot_reload(true)
    .with_metrics(true)
    .with_caching(true)
    .with_cache_size(200)
    .with_log_level("debug".to_string())
    .build()
    .await?;

let config_manager = actor_core.get_config_manager();
let registry_manager = actor_core.get_registry_manager();
```

### Step 4: Update Resource Management

#### Old System
```rust
// Hardcoded resource definitions
let health_resource = ResourceDefinition {
    id: "health".to_string(),
    name: "Health".to_string(),
    base_value: 100.0,
    min_value: 0.0,
    max_value: 1000.0,
    // ... other fields
};
```

#### New System
```rust
// Dynamic resource registration
let health_resource = ResourceDefinition {
    id: "health".to_string(),
    name: "Health".to_string(),
    description: Some("Character health points".to_string()),
    category: "vital".to_string(),
    resource_type: ResourceType::Health,
    base_value: 100.0,
    min_value: 0.0,
    max_value: 1000.0,
    regen_rate: 1.0,
    regen_type: RegenType::Passive,
    dependencies: vec![],
    tags: vec!["vital".to_string(), "health".to_string()],
    subsystem_id: "actor_core".to_string(),
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
};

registry_manager.get_resource_registry().register_resource(health_resource).await?;
```

### Step 5: Update Configuration Access

#### Old System
```rust
// Direct access to hardcoded values
let max_health = 1000.0;
let regen_rate = 1.0;
```

#### New System
```rust
// Configuration-based access
let health_config = config_manager.get_config("defaults", "resources").await?;
let max_health = health_config.unwrap().value["health"]["max_value"].as_f64().unwrap();
let regen_rate = health_config.unwrap().value["health"]["regen_rate"].as_f64().unwrap();
```

### Step 6: Update Registry Usage

#### Old System
```rust
// Static registry access
let resources = get_all_resources();
let categories = get_all_categories();
```

#### New System
```rust
// Dynamic registry access
let resources = registry_manager.get_resource_registry().get_all_resources().await?;
let categories = registry_manager.get_category_registry().get_all_categories().await?;
```

## 📁 Configuration File Structure

### Main Configuration File
```yaml
# configs/actor_core_defaults.yaml
defaults:
  resources:
    health:
      base_value: 100.0
      min_value: 0.0
      max_value: 1000.0
      regen_rate: 1.0
      regen_type: "passive"
    mana:
      base_value: 50.0
      min_value: 0.0
      max_value: 500.0
      regen_rate: 0.5
      regen_type: "passive"
    stamina:
      base_value: 100.0
      min_value: 0.0
      max_value: 1000.0
      regen_rate: 2.0
      regen_type: "passive"
  
  stats:
    strength:
      base_value: 10.0
      min_value: 0.0
      max_value: 100.0
    agility:
      base_value: 10.0
      min_value: 0.0
      max_value: 100.0
    intelligence:
      base_value: 10.0
      min_value: 0.0
      max_value: 100.0
    constitution:
      base_value: 10.0
      min_value: 0.0
      max_value: 100.0
    wisdom:
      base_value: 10.0
      min_value: 0.0
      max_value: 100.0
    charisma:
      base_value: 10.0
      min_value: 0.0
      max_value: 100.0
  
  elements:
    fire:
      base_affinity: 0.0
      min_affinity: 0.0
      max_affinity: 1.0
    water:
      base_affinity: 0.0
      min_affinity: 0.0
      max_affinity: 1.0
    earth:
      base_affinity: 0.0
      min_affinity: 0.0
      max_affinity: 1.0
    air:
      base_affinity: 0.0
      min_affinity: 0.0
      max_affinity: 1.0
    light:
      base_affinity: 0.0
      min_affinity: 0.0
      max_affinity: 1.0
    dark:
      base_affinity: 0.0
      min_affinity: 0.0
      max_affinity: 1.0

timeouts:
  cache_ttl: 3600
  aggregation_timeout: 5.0
  validation_timeout: 1.0
  regeneration_interval: 1.0
  subsystem_timeout: 10.0

performance_thresholds:
  max_actors: 10000
  max_contributions_per_actor: 1000
  max_caps_per_actor: 100
  max_subsystems: 50
  cache_size_mb: 100
  memory_usage_mb: 500
  cpu_usage_percent: 80.0

validation_rules:
  resource_values:
    min_value: 0.0
    max_value: 1000000.0
  stat_values:
    min_value: 0.0
    max_value: 1000.0
  element_affinities:
    min_value: 0.0
    max_value: 1.0
  contribution_values:
    min_value: -1000.0
    max_value: 1000.0

cache_keys:
  actor_snapshot: "actor_snapshot"
  resource_regeneration: "resource_regeneration"
  stat_aggregation: "stat_aggregation"
  subsystem_data: "subsystem_data"

log_levels:
  actor_core: "info"
  config: "info"
  registry: "info"
  cache: "warn"
  performance: "debug"

cache_policies:
  actor_snapshot:
    ttl: 3600
    max_size: 1000
    eviction_policy: "lru"
  resource_regeneration:
    ttl: 300
    max_size: 500
    eviction_policy: "lru"
  stat_aggregation:
    ttl: 1800
    max_size: 2000
    eviction_policy: "lru"

system_ids:
  - "luyen_the"
  - "kim_dan"
  - "combat"
  - "equipment"
  - "buff"
  - "guild"
  - "event"
  - "world"
  - "magic"
  - "cultivation"
  - "experience"
  - "reputation"
  - "trading"
  - "weather"
  - "location"
  - "time"
  - "stealth"
  - "perception"

context_types:
  - "combat"
  - "exploration"
  - "social"
  - "crafting"
  - "cultivation"
  - "trading"
  - "guild"
  - "event"
```

### Environment Variables
```bash
# Set environment variables for runtime configuration
export ACTOR_CORE_ELEMENT_FIRE_AFFINITY=0.8
export ACTOR_CORE_ELEMENT_WATER_AFFINITY=0.6
export ACTOR_CORE_STAT_STRENGTH=15
export ACTOR_CORE_STAT_AGILITY=12
export ACTOR_CORE_FLAG_ENABLE_CACHING=true
export ACTOR_CORE_FLAG_ENABLE_METRICS=true
export ACTOR_CORE_CACHE_SIZE_MB=200
export ACTOR_CORE_LOG_LEVEL=debug
```

## 🧪 Testing Migration

### Update Unit Tests

#### Old System
```rust
#[test]
fn test_resource_creation() {
    let resource = create_resource("health", 100.0);
    assert_eq!(resource.value, 100.0);
}
```

#### New System
```rust
#[tokio::test]
async fn test_resource_creation() -> Result<(), Box<dyn std::error::Error>> {
    let registry_system = RegistryBuilder::new().build().await?;
    
    let resource = ResourceDefinition {
        id: "health".to_string(),
        name: "Health".to_string(),
        description: Some("Character health points".to_string()),
        category: "vital".to_string(),
        resource_type: ResourceType::Health,
        base_value: 100.0,
        min_value: 0.0,
        max_value: 1000.0,
        regen_rate: 1.0,
        regen_type: RegenType::Passive,
        dependencies: vec![],
        tags: vec!["vital".to_string(), "health".to_string()],
        subsystem_id: "test_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    registry_system.get_resource_registry().register_resource(resource).await?;
    
    let retrieved_resource = registry_system.get_resource_registry().get_resource("health").await?;
    assert!(retrieved_resource.is_some());
    assert_eq!(retrieved_resource.unwrap().base_value, 100.0);
    
    Ok(())
}
```

### Update Integration Tests

#### Old System
```rust
#[test]
fn test_system_integration() {
    let system = create_system();
    let result = system.process_actor(actor);
    assert!(result.is_ok());
}
```

#### New System
```rust
#[tokio::test]
async fn test_system_integration() -> Result<(), Box<dyn std::error::Error>> {
    let actor_core = ActorCoreBuilder::new().build().await?;
    
    let config_manager = actor_core.get_config_manager();
    let registry_manager = actor_core.get_registry_manager();
    
    // Test configuration access
    let value = config_manager.get_config("defaults", "resources").await?;
    assert!(value.is_some());
    
    // Test registry access
    let resources = registry_manager.get_resource_registry().get_all_resources().await?;
    assert!(!resources.is_empty());
    
    actor_core.shutdown().await?;
    Ok(())
}
```

## 🚀 Performance Considerations

### Configuration Caching
The new system includes comprehensive caching to improve performance:

```rust
// Configuration is automatically cached
let value = config_manager.get_config("category", "key").await?; // First call loads from source
let value = config_manager.get_config("category", "key").await?; // Subsequent calls use cache
```

### Registry Performance
The registry system is optimized for high-performance operations:

```rust
// Batch operations for better performance
let mut resources = Vec::new();
for i in 0..1000 {
    resources.push(create_resource(i));
}
registry_manager.get_resource_registry().register_resources(resources).await?;
```

### Memory Management
The system uses efficient memory management:

```rust
// Resources are automatically cleaned up
let resource = registry_manager.get_resource_registry().get_resource("health").await?;
// Resource is automatically dropped when out of scope
```

## 🔒 Security Considerations

### Configuration Security
- **Validation**: All configuration values are validated
- **Sanitization**: Input sanitization prevents injection attacks
- **Access Control**: Role-based access control for configuration management
- **Audit Logging**: Comprehensive audit logging for all changes

### Registry Security
- **Access Control**: Fine-grained access control for registry operations
- **Validation**: All registry operations are validated
- **Audit Logging**: Comprehensive audit logging for all changes
- **Rate Limiting**: Rate limiting prevents abuse

## 🐛 Troubleshooting

### Common Issues

#### Configuration Not Loading
```rust
// Check if configuration file exists
if !Path::new("configs/actor_core_defaults.yaml").exists() {
    eprintln!("Configuration file not found!");
    return Err("Configuration file not found".into());
}
```

#### Registry Registration Failing
```rust
// Check if resource ID is valid
if resource.id.is_empty() {
    return Err("Resource ID cannot be empty".into());
}
```

#### Builder Pattern Errors
```rust
// Check if all required parameters are set
let actor_core = ActorCoreBuilder::new()
    .with_config_path(PathBuf::from("configs/actor_core_defaults.yaml"))
    .build()
    .await?;
```

### Debug Mode
Enable debug mode for detailed logging:

```rust
let actor_core = ActorCoreBuilder::new()
    .with_log_level("debug".to_string())
    .build()
    .await?;
```

## 📚 Additional Resources

### Documentation
- [README_REFACTORED.md](README_REFACTORED.md) - Complete system documentation
- [API Reference](docs/api_reference.md) - Detailed API documentation
- [Configuration Guide](docs/configuration_guide.md) - Configuration management guide
- [Performance Guide](docs/performance_guide.md) - Performance optimization guide

### Examples
- [Basic Usage](examples/basic_usage.rs) - Basic usage examples
- [Advanced Usage](examples/advanced_usage.rs) - Advanced usage examples
- [Builder Pattern](examples/builder_pattern_example.rs) - Builder pattern examples
- [Complete System](examples/complete_refactor_example.rs) - Complete system examples

### Support
- **GitHub Issues**: Open an issue on GitHub
- **Discord Community**: Join our Discord community
- **Documentation**: Check the comprehensive documentation
- **Examples**: Review the example programs

---

**Note**: This migration guide is designed to help you transition from the old Actor Core system to the new refactored system. If you encounter any issues during migration, please refer to the troubleshooting section or open an issue on GitHub.