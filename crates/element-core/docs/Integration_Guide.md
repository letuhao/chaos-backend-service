# Element-Core Integration Guide

## Table of Contents

1. [Overview](#overview)
2. [Quick Start](#quick-start)
3. [Basic Integration](#basic-integration)
4. [Advanced Integration](#advanced-integration)
5. [Contributor System](#contributor-system)
6. [Configuration](#configuration)
7. [Performance Optimization](#performance-optimization)
8. [Testing](#testing)
9. [Troubleshooting](#troubleshooting)
10. [Best Practices](#best-practices)

## Overview

This guide provides step-by-step instructions for integrating Element-Core into your game system. Element-Core is designed to be a central data hub that aggregates elemental data from multiple sources while maintaining high performance and thread safety.

### Architecture Principles

- **Data Hub Pattern**: Element-Core aggregates data without business logic
- **External Contributors**: Other systems contribute data via standardized interfaces
- **Loose Coupling**: Systems can be developed independently
- **Thread Safety**: All operations are safe for concurrent access
- **High Performance**: Array-based structures for optimal performance

## Quick Start

### 1. Add Dependency

Add Element-Core to your `Cargo.toml`:

```toml
[dependencies]
element-core = { path = "../element-core" }
tokio = { version = "1.0", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
```

### 2. Basic Setup

```rust
use element_core::{UnifiedElementRegistry, ElementDefinition, ElementProperties};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create registry
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Your integration code here
    
    Ok(())
}
```

### 3. Run Example

```bash
cd element-core
cargo run --example integration_examples
```

## Basic Integration

### Step 1: Create Element Definitions

```rust
use element_core::{ElementDefinition, ElementProperties, ElementCategory};

async fn setup_basic_elements(registry: &UnifiedElementRegistry) -> ElementCoreResult<()> {
    // Fire Element
    let fire_element = ElementDefinition {
        id: "fire".to_string(),
        name: "Fire".to_string(),
        description: "Element of flame and heat".to_string(),
        category: ElementCategory::Elemental,
        base_properties: ElementProperties {
            power_multiplier: 1.2,
            defense_multiplier: 0.8,
            mastery_gain_rate: 1.0,
            qi_efficiency: 0.9,
            interaction_bonus: 0.0,
            status_effect_resistance: 0.5,
            environmental_adaptation: 0.7,
            synergy_bonus: 0.0,
        },
        derived_stats: vec![],
        status_effects: vec![],
        environment_mods: vec![],
        references: ElementReferences::default(),
        aliases: ElementAliases::default(),
    };
    
    // Water Element
    let water_element = ElementDefinition {
        id: "water".to_string(),
        name: "Water".to_string(),
        description: "Element of flow and adaptability".to_string(),
        category: ElementCategory::Elemental,
        base_properties: ElementProperties {
            power_multiplier: 0.9,
            defense_multiplier: 1.3,
            mastery_gain_rate: 1.1,
            qi_efficiency: 1.0,
            interaction_bonus: 0.0,
            status_effect_resistance: 0.8,
            environmental_adaptation: 1.0,
            synergy_bonus: 0.0,
        },
        derived_stats: vec![],
        status_effects: vec![],
        environment_mods: vec![],
        references: ElementReferences::default(),
        aliases: ElementAliases::default(),
    };
    
    // Register elements
    registry.register_element(fire_element).await?;
    registry.register_element(water_element).await?;
    
    Ok(())
}
```

### Step 2: Set Up Element Interactions

```rust
async fn setup_element_interactions(registry: &UnifiedElementRegistry) -> ElementCoreResult<()> {
    // Fire overcomes Water
    registry.set_interaction_bonus("fire", "water", 1.5).await?;
    
    // Water overcomes Fire
    registry.set_interaction_bonus("water", "fire", 1.3).await?;
    
    // Neutral interactions
    registry.set_interaction_bonus("fire", "fire", 1.0).await?;
    registry.set_interaction_bonus("water", "water", 1.0).await?;
    
    Ok(())
}
```

### Step 3: Create Elemental System Data

```rust
use element_core::{ElementalSystemData, ElementMasteryRank};

async fn create_character_elemental_system() -> ElementalSystemData {
    let mut system = ElementalSystemData::new();
    
    // Set initial mastery levels
    system.update_mastery_level(0, 10.0).unwrap(); // Fire
    system.update_mastery_level(1, 8.0).unwrap();  // Water
    
    // Set power points
    system.power_point[0] = 150.0; // Fire power
    system.power_point[1] = 120.0; // Water power
    
    // Set defense points
    system.defense_point[0] = 80.0;  // Fire defense
    system.defense_point[1] = 130.0; // Water defense
    
    system
}
```

## Advanced Integration

### Step 1: Create Custom Contributor

```rust
use element_core::{
    ElementContributor, ElementContribution, ContributorMetadata,
    ElementCoreResult, ElementEvent
};
use async_trait::async_trait;
use std::collections::HashMap;

pub struct RaceContributor {
    system_id: String,
    priority: i64,
    racial_bonuses: HashMap<String, HashMap<String, f64>>,
}

impl RaceContributor {
    pub fn new() -> Self {
        let mut racial_bonuses = HashMap::new();
        
        // Fire Spirit bonuses
        let mut fire_spirit = HashMap::new();
        fire_spirit.insert("fire".to_string(), 2.0);
        fire_spirit.insert("water".to_string(), 0.5);
        racial_bonuses.insert("fire_spirit".to_string(), fire_spirit);
        
        // Water Nymph bonuses
        let mut water_nymph = HashMap::new();
        water_nymph.insert("water".to_string(), 2.2);
        water_nymph.insert("fire".to_string(), 0.4);
        racial_bonuses.insert("water_nymph".to_string(), water_nymph);
        
        Self {
            system_id: "race-core".to_string(),
            priority: 1000,
            racial_bonuses,
        }
    }
}

#[async_trait]
impl ElementContributor for RaceContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute_element_stats(
        &self,
        actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<ElementContribution> {
        let mut stat_contributions = HashMap::new();
        
        // Get actor's race (assuming it's stored in actor data)
        let race = actor.data.get("race").and_then(|v| v.as_str()).unwrap_or("human");
        
        if let Some(racial_bonuses) = self.racial_bonuses.get(race) {
            if let Some(&bonus) = racial_bonuses.get(element_type) {
                stat_contributions.insert("racial_affinity".to_string(), bonus);
            }
        }
        
        Ok(self.create_contribution(element_type, stat_contributions))
    }
    
    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()> {
        match event {
            ElementEvent::MasteryLevelChanged { element_type, new_level, actor_id, .. } => {
                println!("Race-Core: Actor {} gained {} mastery level in {}", 
                    actor_id, new_level, element_type);
            },
            _ => {} // Handle other events as needed
        }
        Ok(())
    }
    
    fn get_metadata(&self) -> ContributorMetadata {
        ContributorMetadata {
            system_id: self.system_id.clone(),
            priority: self.priority,
            version: "1.0.0".to_string(),
            description: "Provides racial elemental bonuses".to_string(),
        }
    }
}
```

### Step 2: Register Contributors

```rust
async fn register_contributors(registry: &UnifiedElementRegistry) -> ElementCoreResult<()> {
    // Register race contributor
    let race_contributor = Arc::new(RaceContributor::new());
    registry.register_contributor(race_contributor).await?;
    
    // Register other contributors as needed
    // let item_contributor = Arc::new(ItemContributor::new());
    // registry.register_contributor(item_contributor).await?;
    
    Ok(())
}
```

### Step 3: Aggregate Contributions

```rust
async fn aggregate_contributions(
    registry: &UnifiedElementRegistry,
    actor: &Actor,
    element_type: &str,
) -> ElementCoreResult<ElementContribution> {
    let mut aggregated_stats = HashMap::new();
    
    // Get all contributors
    let contributors = registry.get_all_contributors().await?;
    
    for contributor in contributors {
        let contribution = contributor.contribute_element_stats(actor, element_type).await?;
        
        // Merge stats based on priority
        for (stat_name, stat_value) in contribution.stat_contributions {
            let current_value = aggregated_stats.get(&stat_name).copied().unwrap_or(0.0);
            aggregated_stats.insert(stat_name, current_value + stat_value);
        }
    }
    
    // Create final contribution
    Ok(ElementContribution {
        system_id: "aggregated".to_string(),
        element_type: element_type.to_string(),
        stat_contributions: aggregated_stats,
        priority: 0,
        timestamp: chrono::Utc::now(),
    })
}
```

## Contributor System

### Creating a Contributor

1. **Implement ElementContributor trait**:

```rust
#[async_trait]
impl ElementContributor for MyContributor {
    fn system_id(&self) -> &str { "my-system" }
    fn priority(&self) -> i64 { 500 }
    
    async fn contribute_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementContribution> {
        // Your contribution logic here
    }
    
    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()> {
        // Your event handling logic here
    }
    
    fn get_metadata(&self) -> ContributorMetadata {
        // Return metadata
    }
}
```

2. **Register with registry**:

```rust
let contributor = Arc::new(MyContributor::new());
registry.register_contributor(contributor).await?;
```

### Priority System

Contributors are processed in priority order (higher priority first):

- **Race-Core**: 1000 (base racial bonuses)
- **Item-Core**: 800 (equipment bonuses)
- **Skill-Core**: 600 (skill bonuses)
- **Event-Core**: 400 (temporary effects)

### Event Handling

Contributors can react to elemental events:

```rust
async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()> {
    match event {
        ElementEvent::MasteryLevelChanged { element_type, new_level, actor_id, .. } => {
            // React to mastery level changes
            if *new_level >= 50.0 {
                self.unlock_advanced_abilities(actor_id, element_type).await?;
            }
        },
        ElementEvent::ElementInteraction { attacker_element, defender_element, .. } => {
            // React to element interactions
            self.apply_interaction_bonuses(attacker_element, defender_element).await?;
        },
        _ => {} // Handle other events
    }
    Ok(())
}
```

## Configuration

### YAML Configuration

Create a configuration file `element_config.yaml`:

```yaml
registry:
  cache:
    max_size: 1000
    ttl_seconds: 3600
    eviction_policy: "lru"
    enable_metrics: true
  
  performance:
    max_elements: 50
    thread_pool_size: 4
    batch_size: 100
    enable_parallel_processing: true
  
  validation:
    strict_mode: true
    validate_on_load: true
    validate_on_save: true
  
  logging:
    level: "info"
    enable_metrics: true
    log_contributions: false

elements:
  fire:
    name: "Fire"
    description: "Element of flame and heat"
    category: "elemental"
    base_properties:
      power_multiplier: 1.2
      defense_multiplier: 0.8
      mastery_gain_rate: 1.0
      qi_efficiency: 0.9
      interaction_bonus: 0.0
      status_effect_resistance: 0.5
      environmental_adaptation: 0.7
      synergy_bonus: 0.0

interactions:
  fire:
    water: 1.5
    earth: 0.8
    ice: 1.3
  water:
    fire: 1.3
    earth: 1.1
    ice: 0.9
```

### Loading Configuration

```rust
use element_core::config::YamlLoader;

async fn load_configuration() -> ElementCoreResult<RegistryConfig> {
    let loader = YamlLoader::new();
    let config = loader.load_config("element_config.yaml").await?;
    Ok(config)
}
```

## Performance Optimization

### 1. Use Appropriate Data Structures

```rust
// Use fixed-size arrays for performance-critical data
let mut mastery_levels = [0.0; MAX_ELEMENTS];

// Use HashMap for dynamic data
let mut dynamic_stats = HashMap::new();
```

### 2. Batch Operations

```rust
// Batch multiple operations
let mut batch = Vec::new();
for element in elements {
    batch.push(registry.register_element(element));
}
futures::future::join_all(batch).await;
```

### 3. Cache Frequently Accessed Data

```rust
// Cache element definitions
let element_cache = Arc::new(RwLock::new(HashMap::new()));

// Use cache for frequently accessed elements
if let Some(cached) = element_cache.read().unwrap().get(element_id) {
    return Ok(cached.clone());
}
```

### 4. Use Async Operations

```rust
// Use async for I/O operations
async fn load_elements_from_file(path: &str) -> ElementCoreResult<Vec<ElementDefinition>> {
    let content = tokio::fs::read_to_string(path).await?;
    let elements: Vec<ElementDefinition> = serde_yaml::from_str(&content)?;
    Ok(elements)
}
```

### 5. Monitor Performance

```rust
// Enable metrics collection
let config = RegistryConfig {
    cache: CacheConfig {
        enable_metrics: true,
        // ... other config
    },
    // ... other config
};

// Get performance metrics
let metrics = registry.get_metrics().await;
println!("Cache hit rate: {:.2}%", metrics.get("cache_hit_rate").unwrap_or(&0.0));
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_element_registration() {
        let registry = UnifiedElementRegistry::new();
        let element = create_test_element();
        
        registry.register_element(element).await.unwrap();
        let retrieved = registry.get_element("test").await.unwrap();
        
        assert_eq!(retrieved.id, "test");
    }
    
    #[tokio::test]
    async fn test_contributor_integration() {
        let registry = Arc::new(UnifiedElementRegistry::new());
        let contributor = Arc::new(TestContributor::new());
        
        registry.register_contributor(contributor).await.unwrap();
        let contributors = registry.get_all_contributors().await.unwrap();
        
        assert_eq!(contributors.len(), 1);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_full_integration() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Setup elements
    setup_basic_elements(&registry).await.unwrap();
    
    // Setup contributors
    let race_contributor = Arc::new(RaceContributor::new());
    registry.register_contributor(race_contributor).await.unwrap();
    
    // Test aggregation
    let actor = create_test_actor();
    let contribution = aggregate_contributions(&registry, &actor, "fire").await.unwrap();
    
    assert!(contribution.stat_contributions.contains_key("racial_affinity"));
}
```

### Performance Tests

```rust
#[tokio::test]
async fn test_performance() {
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    let start = std::time::Instant::now();
    
    // Perform many operations
    for i in 0..1000 {
        let element = create_test_element_with_id(&format!("element_{}", i));
        registry.register_element(element).await.unwrap();
    }
    
    let duration = start.elapsed();
    println!("Registered 1000 elements in {:?}", duration);
    
    assert!(duration.as_secs() < 1); // Should complete in less than 1 second
}
```

## Troubleshooting

### Common Issues

#### 1. Compilation Errors

**Problem**: `ElementCoreError` variants not found
**Solution**: Use struct variants instead of unit variants:

```rust
// Wrong
Err(ElementCoreError::Validation)

// Correct
Err(ElementCoreError::Validation { message: "Invalid data".to_string() })
```

#### 2. Thread Safety Issues

**Problem**: `cannot be shared between threads safely`
**Solution**: Wrap in `Arc` and use `Send + Sync`:

```rust
// Wrong
let registry = UnifiedElementRegistry::new();

// Correct
let registry = Arc::new(UnifiedElementRegistry::new());
```

#### 3. Async/Await Issues

**Problem**: `cannot be called in a synchronous context`
**Solution**: Use `async` functions and `await`:

```rust
// Wrong
let element = registry.get_element("fire");

// Correct
let element = registry.get_element("fire").await?;
```

#### 4. Configuration Loading Issues

**Problem**: Configuration file not found
**Solution**: Check file path and permissions:

```rust
let config_path = std::env::current_dir()?.join("config").join("element_config.yaml");
if !config_path.exists() {
    return Err(ElementCoreError::Config { 
        message: format!("Configuration file not found: {:?}", config_path) 
    });
}
```

### Debug Mode

Enable debug logging:

```rust
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

// Or set log level in configuration
let config = RegistryConfig {
    logging: LoggingConfig {
        level: "debug".to_string(),
        enable_metrics: true,
        log_contributions: true,
    },
    // ... other config
};
```

### Performance Debugging

```rust
// Get detailed metrics
let metrics = registry.get_metrics().await;
for (key, value) in metrics {
    println!("{}: {}", key, value);
}

// Check cache statistics
let cache_stats = registry.get_cache_stats().await;
println!("Cache stats: {:?}", cache_stats);
```

## Best Practices

### 1. Error Handling

Always use proper error handling:

```rust
match registry.get_element("fire").await {
    Ok(element) => {
        // Handle success
    },
    Err(ElementCoreError::ElementNotFound { element_id }) => {
        // Handle specific error
        eprintln!("Element '{}' not found", element_id);
    },
    Err(e) => {
        // Handle other errors
        eprintln!("Error: {}", e);
    }
}
```

### 2. Resource Management

Use `Arc` for shared ownership:

```rust
let registry = Arc::new(UnifiedElementRegistry::new());
let registry_clone = Arc::clone(&registry);

// Safe to use in multiple threads
tokio::spawn(async move {
    registry_clone.get_element("fire").await;
});
```

### 3. Configuration Management

Use configuration files for flexibility:

```rust
let config = load_configuration().await?;
let registry = UnifiedElementRegistry::with_config(config);
```

### 4. Testing

Write comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Test individual components
    #[test]
    fn test_element_creation() { /* ... */ }
    
    // Test integration
    #[tokio::test]
    async fn test_full_workflow() { /* ... */ }
    
    // Test performance
    #[tokio::test]
    async fn test_performance() { /* ... */ }
}
```

### 5. Documentation

Document your contributors and configurations:

```rust
/// Race-based elemental contributor
/// 
/// This contributor provides racial bonuses to elemental stats.
/// Different races have different affinities for various elements.
pub struct RaceContributor {
    // ... implementation
}
```

### 6. Monitoring

Enable metrics and monitoring:

```rust
let config = RegistryConfig {
    cache: CacheConfig {
        enable_metrics: true,
        // ... other config
    },
    logging: LoggingConfig {
        level: "info".to_string(),
        enable_metrics: true,
        log_contributions: false,
    },
    // ... other config
};
```

This integration guide should help you successfully integrate Element-Core into your game system. For more specific examples, refer to the `examples/` directory in the Element-Core repository.