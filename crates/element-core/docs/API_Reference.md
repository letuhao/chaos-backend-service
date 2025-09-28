# Element-Core API Reference

## Table of Contents

1. [Overview](#overview)
2. [Core Types](#core-types)
3. [Common Traits](#common-traits)
4. [Unified Element Registry](#unified-element-registry)
5. [Elemental System Data](#elemental-system-data)
6. [Contributor System](#contributor-system)
7. [Factory System](#factory-system)
8. [Configuration](#configuration)
9. [Error Handling](#error-handling)
10. [Examples](#examples)

## Overview

Element-Core is a high-performance, thread-safe elemental system for game development. It provides a centralized registry for elemental data, interaction matrices, and contributor systems.

### Key Features

- **Thread-Safe**: All operations are safe for concurrent access
- **High-Performance**: Array-based data structures for optimal performance
- **Extensible**: Easy to add new elements and interaction types
- **Validated**: Comprehensive data integrity checking
- **Configurable**: YAML-based configuration system

## Core Types

### ElementCoreResult

```rust
pub type ElementCoreResult<T> = Result<T, ElementCoreError>;
```

Standard result type used throughout the element-core system.

### ElementCoreError

```rust
#[derive(Debug, thiserror::Error)]
pub enum ElementCoreError {
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Configuration error: {message}")]
    Config { message: String },
    
    #[error("Registry error: {message}")]
    Registry { message: String },
    
    #[error("Serialization error: {message}")]
    Serialization { message: String },
    
    #[error("YAML parsing error: {message}")]
    YamlParsing { message: String },
    
    #[error("Element not found: {element_id}")]
    ElementNotFound { element_id: String },
    
    #[error("Index out of bounds: {index}")]
    IndexOutOfBounds { index: usize },
}
```

Comprehensive error types with detailed context information.

## Common Traits

### ElementGetter<T>

```rust
pub trait ElementGetter<T> {
    fn get_element(&self, identifier: &str) -> ElementCoreResult<T>;
    fn get_all_elements(&self) -> ElementCoreResult<Vec<T>>;
    fn has_element(&self, identifier: &str) -> bool;
    fn element_count(&self) -> usize;
}
```

Standard trait for retrieving elements from containers.

### ElementSetter<T>

```rust
pub trait ElementSetter<T> {
    fn set_element(&self, identifier: &str, element: T) -> ElementCoreResult<()>;
    fn remove_element(&self, identifier: &str) -> ElementCoreResult<()>;
}
```

Standard trait for modifying elements in containers.

### Validatable

```rust
pub trait Validatable {
    fn validate(&self) -> ElementCoreResult<()>;
    fn get_validation_errors(&self) -> Vec<String>;
}
```

Trait for data integrity validation.

### Cacheable

```rust
pub trait Cacheable {
    fn clear_cache(&self);
    fn get_cache_stats(&self) -> HashMap<String, f64>;
    fn reset_cache_stats(&self);
}
```

Trait for cache management and statistics.

### MetricsProvider

```rust
pub trait MetricsProvider {
    fn get_metrics(&self) -> HashMap<String, f64>;
    fn reset_metrics(&self);
    fn display_metrics(&self) -> String;
}
```

Trait for performance monitoring and metrics collection.

### Configurable

```rust
pub trait Configurable {
    fn get_config(&self) -> serde_json::Value;
    fn update_config(&self, config: serde_json::Value) -> ElementCoreResult<()>;
    fn validate_config(&self, config: &serde_json::Value) -> ElementCoreResult<()>;
}
```

Trait for configuration management.

### Serializable

```rust
pub trait Serializable: Sized {
    fn to_json(&self) -> ElementCoreResult<String>;
    fn to_yaml(&self) -> ElementCoreResult<String>;
    fn from_json(json: &str) -> ElementCoreResult<Self>;
    fn from_yaml(yaml: &str) -> ElementCoreResult<Self>;
}
```

Trait for data persistence and serialization.

### ElementHelper

```rust
pub trait ElementHelper {
    fn validate_identifier(identifier: &str) -> ElementCoreResult<()>;
}
```

Trait providing utility functions for element operations.

## Unified Element Registry

### UnifiedElementRegistry

The central registry for all elemental data and interactions.

```rust
pub struct UnifiedElementRegistry {
    elements: Arc<RwLock<HashMap<String, ElementDefinition>>>,
    element_interactions: Arc<RwLock<[[f64; MAX_ELEMENTS]; MAX_ELEMENTS]>>,
    feature_flags: Arc<RwLock<[[bool; 16]; MAX_ELEMENTS]>>,
    contributors: Arc<RwLock<HashMap<String, Arc<dyn ElementContributor + Send + Sync>>>>,
    metrics: Arc<RwLock<RegistryMetrics>>,
    config: Arc<RwLock<RegistryConfig>>,
}
```

#### Key Methods

```rust
impl UnifiedElementRegistry {
    /// Create a new registry instance
    pub fn new() -> Self;
    
    /// Register an element definition
    pub async fn register_element(&self, element: ElementDefinition) -> ElementCoreResult<()>;
    
    /// Get element by ID
    pub async fn get_element(&self, element_id: &str) -> ElementCoreResult<ElementDefinition>;
    
    /// Get all elements
    pub async fn get_all_elements(&self) -> ElementCoreResult<Vec<ElementDefinition>>;
    
    /// Set element interaction bonus
    pub async fn set_interaction_bonus(&self, source: &str, target: &str, bonus: f64) -> ElementCoreResult<()>;
    
    /// Get element interaction bonus
    pub async fn get_interaction_bonus(&self, source: &str, target: &str) -> ElementCoreResult<f64>;
    
    /// Register a contributor system
    pub async fn register_contributor(&self, contributor: Arc<dyn ElementContributor + Send + Sync>) -> ElementCoreResult<()>;
    
    /// Get contributor by system ID
    pub async fn get_contributor(&self, system_id: &str) -> ElementCoreResult<Arc<dyn ElementContributor + Send + Sync>>;
}
```

### ElementDefinition

```rust
pub struct ElementDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ElementCategory,
    pub base_properties: ElementProperties,
    pub derived_stats: Vec<DerivedStatConfig>,
    pub status_effects: Vec<StatusEffectConfig>,
    pub environment_mods: Vec<EnvironmentMod>,
    pub references: ElementReferences,
    pub aliases: ElementAliases,
}
```

### ElementProperties

```rust
pub struct ElementProperties {
    pub power_multiplier: f64,
    pub defense_multiplier: f64,
    pub mastery_gain_rate: f64,
    pub qi_efficiency: f64,
    pub interaction_bonus: f64,
    pub status_effect_resistance: f64,
    pub environmental_adaptation: f64,
    pub synergy_bonus: f64,
}
```

## Elemental System Data

### ElementalSystemData

```rust
pub struct ElementalSystemData {
    pub element_mastery_levels: [f64; MAX_ELEMENTS],
    pub element_mastery_experience: [f64; MAX_ELEMENTS],
    pub power_point: [f64; MAX_ELEMENTS],
    pub defense_point: [f64; MAX_ELEMENTS],
    pub qi_amount: [f64; MAX_ELEMENTS],
    pub qi_regeneration: [f64; MAX_ELEMENTS],
    pub status_effects: [Vec<StatusEffect>; MAX_ELEMENTS],
    pub environmental_modifiers: [f64; MAX_ELEMENTS],
    pub synergy_bonuses: [f64; MAX_ELEMENTS],
    pub last_updated: DateTime<Utc>,
}
```

#### Key Methods

```rust
impl ElementalSystemData {
    /// Create new elemental system data
    pub fn new() -> Self;
    
    /// Get total elemental mastery
    pub fn get_total_elemental_mastery(&self) -> f64;
    
    /// Get total power points
    pub fn get_total_power_points(&self) -> f64;
    
    /// Get total defense points
    pub fn get_total_defense_points(&self) -> f64;
    
    /// Get total qi amount
    pub fn get_total_qi_amount(&self) -> f64;
    
    /// Update mastery level for specific element
    pub fn update_mastery_level(&mut self, element_index: usize, new_level: f64) -> ElementCoreResult<()>;
    
    /// Add experience to element
    pub fn add_experience(&mut self, element_index: usize, experience: f64) -> ElementCoreResult<()>;
}
```

### ElementMasteryRank

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ElementMasteryRank {
    Novice = 0,
    Apprentice = 1,
    Adept = 2,
    Expert = 3,
    Master = 4,
    Grandmaster = 5,
    Transcendent = 6,
    Divine = 7,
}
```

## Contributor System

### ElementContributor

```rust
#[async_trait]
pub trait ElementContributor: Send + Sync {
    fn system_id(&self) -> &str;
    fn priority(&self) -> i64;
    
    async fn contribute_element_stats(
        &self,
        actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<ElementContribution>;
    
    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()>;
    
    fn get_metadata(&self) -> ContributorMetadata;
}
```

### ElementContribution

```rust
pub struct ElementContribution {
    pub system_id: String,
    pub element_type: String,
    pub stat_contributions: HashMap<String, f64>,
    pub priority: i64,
    pub timestamp: DateTime<Utc>,
}
```

### ElementEvent

```rust
#[derive(Debug, Clone)]
pub enum ElementEvent {
    MasteryLevelChanged {
        element_type: String,
        old_level: f64,
        new_level: f64,
        actor_id: String,
    },
    ElementInteraction {
        attacker_element: String,
        defender_element: String,
        interaction_type: String,
        actor_id: String,
    },
    TrainingCompleted {
        element_type: String,
        experience_gained: f64,
        actor_id: String,
    },
    StatusEffectApplied {
        element_type: String,
        effect_name: String,
        intensity: f64,
        actor_id: String,
    },
}
```

## Factory System

### ElementalFactory

```rust
pub struct ElementalFactory {
    registry: Arc<UnifiedElementRegistry>,
    config: ElementalFactoryConfig,
}
```

#### Key Methods

```rust
impl ElementalFactory {
    /// Create new factory instance
    pub fn new(registry: Arc<UnifiedElementRegistry>) -> Self;
    
    /// Create elemental system builder
    pub fn create_builder(&self) -> ElementalSystemBuilder;
    
    /// Build elemental system from configuration
    pub async fn build_from_config(&self, config: &ElementalSystemConfig) -> ElementCoreResult<ElementalSystem>;
}
```

### ElementalSystemBuilder

```rust
pub struct ElementalSystemBuilder {
    data: ElementalSystemData,
    registry: Arc<UnifiedElementRegistry>,
    config: ElementalSystemConfig,
}
```

#### Key Methods

```rust
impl ElementalSystemBuilder {
    /// Set initial mastery level for element
    pub fn with_mastery_level(mut self, element_index: usize, level: f64) -> Self;
    
    /// Set power points for element
    pub fn with_power_points(mut self, element_index: usize, points: f64) -> Self;
    
    /// Set defense points for element
    pub fn with_defense_points(mut self, element_index: usize, points: f64) -> Self;
    
    /// Build the elemental system
    pub fn build(self) -> ElementCoreResult<ElementalSystem>;
}
```

## Configuration

### RegistryConfig

```rust
pub struct RegistryConfig {
    pub cache: CacheConfig,
    pub performance: PerformanceConfig,
    pub validation: ValidationConfig,
    pub logging: LoggingConfig,
}
```

### CacheConfig

```rust
pub struct CacheConfig {
    pub max_size: usize,
    pub ttl_seconds: u64,
    pub eviction_policy: EvictionPolicy,
    pub enable_metrics: bool,
}
```

### PerformanceConfig

```rust
pub struct PerformanceConfig {
    pub max_elements: usize,
    pub thread_pool_size: usize,
    pub batch_size: usize,
    pub enable_parallel_processing: bool,
}
```

## Error Handling

### Error Types

- **Validation**: Data validation failures
- **Config**: Configuration errors
- **Registry**: Registry operation failures
- **Serialization**: JSON/YAML serialization errors
- **ElementNotFound**: Element not found in registry
- **IndexOutOfBounds**: Array index out of bounds

### Error Context

All errors include detailed context information to help with debugging:

```rust
// Example error usage
match registry.get_element("fire") {
    Ok(element) => println!("Found element: {}", element.name),
    Err(ElementCoreError::ElementNotFound { element_id }) => {
        println!("Element '{}' not found in registry", element_id);
    },
    Err(e) => println!("Other error: {}", e),
}
```

## Examples

### Basic Usage

```rust
use element_core::{UnifiedElementRegistry, ElementDefinition, ElementProperties};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create registry
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Create element definition
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
        // ... other fields
    };
    
    // Register element
    registry.register_element(fire_element).await?;
    
    // Get element
    let element = registry.get_element("fire").await?;
    println!("Element: {}", element.name);
    
    Ok(())
}
```

### Contributor Integration

```rust
use element_core::{ElementContributor, ElementContribution, ContributorMetadata};
use async_trait::async_trait;

struct MyContributor {
    system_id: String,
    priority: i64,
}

#[async_trait]
impl ElementContributor for MyContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute_element_stats(
        &self,
        _actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<ElementContribution> {
        let mut stat_contributions = HashMap::new();
        stat_contributions.insert("power".to_string(), 100.0);
        
        Ok(self.create_contribution(element_type, stat_contributions))
    }
    
    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()> {
        println!("Handling event: {:?}", event);
        Ok(())
    }
    
    fn get_metadata(&self) -> ContributorMetadata {
        ContributorMetadata {
            system_id: self.system_id.clone(),
            priority: self.priority,
            version: "1.0.0".to_string(),
            description: "My custom contributor".to_string(),
        }
    }
}
```

## Performance Considerations

### Thread Safety

All operations are thread-safe and can be used in concurrent environments:

```rust
// Safe to use across threads
let registry = Arc::new(UnifiedElementRegistry::new());
let registry_clone = Arc::clone(&registry);

tokio::spawn(async move {
    registry_clone.get_element("fire").await;
});
```

### Memory Usage

- Fixed-size arrays for optimal performance
- `MAX_ELEMENTS = 50` by default
- Efficient HashMap usage for dynamic data
- Arc<RwLock<>> for shared ownership

### Caching

Built-in caching system with configurable TTL and eviction policies:

```rust
let config = RegistryConfig {
    cache: CacheConfig {
        max_size: 1000,
        ttl_seconds: 3600,
        eviction_policy: EvictionPolicy::LRU,
        enable_metrics: true,
    },
    // ... other config
};
```

## Migration Guide

### From v0.0.x to v0.1.x

1. **Error Handling**: Update error handling to use new struct variants
2. **API Changes**: Some method signatures have changed
3. **Configuration**: New configuration system with YAML support
4. **Traits**: New common traits for consistent API patterns

### Breaking Changes

- `ElementCoreError` variants are now struct variants
- Some method signatures have changed for better type safety
- Configuration system has been redesigned

## Support

For questions, issues, or contributions, please refer to the main repository documentation or create an issue in the project repository.