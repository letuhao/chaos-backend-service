# Actor Core API Documentation

This document provides comprehensive API documentation for the Actor Core system.

## Table of Contents

- [Core Types](#core-types)
- [Traits](#traits)
- [Enums](#enums)
- [Error Handling](#error-handling)
- [Configuration](#configuration)
- [Examples](#examples)

## Core Types

### Actor

The central character representation in the system.

```rust
pub struct Actor {
    pub id: Uuid,
    pub name: String,
    pub race: String,
    pub data: HashMap<String, serde_json::Value>,
    pub buffs: HashSet<String>,
    pub subsystems: Vec<Subsystem>,
    pub version: u64,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub lifespan: i64,
    pub age: i64,
}
```

#### Key Methods

- `new(name: String, race: String) -> Self` - Create a new actor
- `set_data(data: HashMap<String, serde_json::Value)` - Set actor data
- `get_data() -> &HashMap<String, serde_json::Value>` - Get actor data
- `add_buff(buff_id: String)` - Add a buff to the actor
- `remove_buff(buff_id: &str)` - Remove a buff from the actor
- `has_buff(buff_id: &str) -> bool` - Check if actor has a buff
- `add_subsystem(subsystem: Subsystem)` - Add a subsystem to the actor
- `has_subsystem(system_id: &str) -> bool` - Check if actor has a subsystem

### Contribution

Represents a stat modification from a specific source.

```rust
pub struct Contribution {
    pub dimension: String,
    pub bucket: Bucket,
    pub value: f64,
    pub source: String,
    pub priority: Option<i64>,
}
```

#### Key Methods

- `new(dimension: String, bucket: Bucket, value: f64, source: String) -> Self` - Create a new contribution
- `is_valid() -> bool` - Check if the contribution is valid

### Caps

Represents min/max constraints for a stat value.

```rust
pub struct Caps {
    min: f64,
    max: f64,
}
```

#### Key Methods

- `new(min: f64, max: f64) -> Self` - Create new caps
- `clamp(value: f64) -> f64` - Clamp a value to the caps range
- `contains(value: f64) -> bool` - Check if a value is within the caps
- `union(other: &Caps) -> Caps` - Union with another caps
- `intersection(other: &Caps) -> Caps` - Intersection with another caps
- `is_valid() -> bool` - Check if the caps are valid

### Snapshot

Represents the aggregated state of an actor at a point in time.

```rust
pub struct Snapshot {
    pub actor_id: Uuid,
    pub version: u64,
    pub primary: HashMap<String, f64>,
    pub derived: HashMap<String, f64>,
    pub caps_used: EffectiveCaps,
    pub subsystems_processed: Vec<String>,
    pub processing_time: Option<u64>,
    pub created_at: Timestamp,
}
```

#### Key Methods

- `new(actor_id: Uuid, version: u64) -> Self` - Create a new snapshot
- `add_primary(contribution: Contribution)` - Add a primary stat contribution
- `add_derived(contribution: Contribution)` - Add a derived stat contribution
- `get_primary(dimension: &str) -> Option<f64>` - Get a primary stat value
- `get_derived(dimension: &str) -> Option<f64>` - Get a derived stat value

## Traits

### Subsystem

Trait for modular stat processing components.

```rust
pub trait Subsystem: Send + Sync {
    fn system_id(&self) -> &str;
    fn priority(&self) -> i64;
    fn contribute(&self, actor: &Actor) -> Pin<Box<dyn Future<Output = Result<SubsystemOutput, ActorCoreError>> + Send + '_>>;
}
```

### Aggregator

Trait for stat aggregation functionality.

```rust
pub trait Aggregator: Send + Sync {
    async fn resolve(&self, actor: &Actor) -> ActorCoreResult<Snapshot>;
    async fn resolve_with_context(&self, actor: &Actor, context: Option<HashMap<String, serde_json::Value>>) -> ActorCoreResult<Snapshot>;
    async fn get_metrics(&self) -> AggregatorMetrics;
}
```

### CapsProvider

Trait for caps management.

```rust
pub trait CapsProvider: Send + Sync {
    async fn effective_caps_within_layer(&self, actor: &Actor, outputs: &[SubsystemOutput], layer: &str) -> ActorCoreResult<HashMap<String, Caps>>;
    async fn effective_caps_across_layers(&self, actor: &Actor, outputs: &[SubsystemOutput]) -> ActorCoreResult<EffectiveCaps>;
    async fn get_caps_for_dimension(&self, dimension: &str, actor: &Actor) -> ActorCoreResult<Option<Caps>>;
    async fn get_metrics(&self) -> CapStatistics;
}
```

### PluginRegistry

Trait for subsystem registration and management.

```rust
pub trait PluginRegistry: Send + Sync {
    fn register(&self, subsystem: Box<dyn Subsystem>) -> ActorCoreResult<()>;
    fn unregister(&self, system_id: &str) -> ActorCoreResult<()>;
    fn get_by_id(&self, system_id: &str) -> Option<Box<dyn Subsystem>>;
    fn get_by_priority(&self) -> Vec<Box<dyn Subsystem>>;
    fn is_registered(&self, system_id: &str) -> bool;
    fn count(&self) -> usize;
    fn validate_all(&self) -> ActorCoreResult<()>;
}
```

## Enums

### Bucket

Defines how contributions are processed.

```rust
pub enum Bucket {
    Flat,        // Additive contributions
    Mult,        // Multiplicative contributions
    PostAdd,     // Post-addition contributions
    Override,    // Override contributions
    #[cfg(feature = "extra_buckets")]
    Exponential, // Exponential contributions
    #[cfg(feature = "extra_buckets")]
    Logarithmic, // Logarithmic contributions
    #[cfg(feature = "extra_buckets")]
    Conditional, // Conditional contributions
}
```

### CapMode

Defines how caps are applied.

```rust
pub enum CapMode {
    Baseline,  // Base caps
    Additive,  // Additive caps
    HardMax,   // Hard maximum caps
    SoftMax,   // Soft maximum caps
}
```

### Operator

Defines mathematical operations.

```rust
pub enum Operator {
    Sum,        // Sum operation
    Multiply,   // Multiply operation
    Max,        // Maximum operation
    Min,        // Minimum operation
}
```

## Error Handling

### ActorCoreError

The main error type for the system.

```rust
pub enum ActorCoreError {
    ValidationError(String),
    ProcessingError(String),
    ConfigurationError(String),
    CacheError(String),
    RegistryError(String),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
}
```

### ActorCoreResult

Type alias for results.

```rust
pub type ActorCoreResult<T> = Result<T, ActorCoreError>;
```

## Configuration

### Cap Layers Configuration

```yaml
cap_layers:
  - name: base
    priority: 100
    cap_mode: BASELINE
  - name: equipment
    priority: 200
    cap_mode: ADDITIVE
  - name: buffs
    priority: 300
    cap_mode: HARD_MAX

across_layer_policy: STRICT
```

### Combiner Configuration

```yaml
combiner_rules:
  - name: attack
    bucket_order: [Flat, Mult, PostAdd, Override]
    clamp: true
  - name: defense
    bucket_order: [Flat, Mult, PostAdd, Override]
    clamp: true
```

## Examples

### Basic Actor Creation

```rust
use actor_core::*;
use std::collections::HashMap;

// Create an actor
let mut actor = Actor::new("Player1".to_string(), "Human".to_string());

// Set actor data
let mut data = HashMap::new();
data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(10)));
data.insert("class".to_string(), serde_json::Value::String("warrior".to_string()));
actor.set_data(data);

// Add a buff
actor.add_buff("strength_boost".to_string());
```

### Contribution Processing

```rust
use actor_core::*;

// Create contributions
let contributions = vec![
    Contribution::new("strength".to_string(), Bucket::Flat, 10.0, "equipment".to_string()),
    Contribution::new("strength".to_string(), Bucket::Mult, 1.2, "buff".to_string()),
    Contribution::new("strength".to_string(), Bucket::PostAdd, 5.0, "talent".to_string()),
];

// Process contributions
let result = bucket_processor::process_contributions_in_order(
    contributions,
    0.0,
    None
);
```

### Caps Management

```rust
use actor_core::*;

// Create caps
let caps = Caps::new(0.0, 100.0);

// Clamp a value
let clamped_value = caps.clamp(150.0); // Returns 100.0

// Check if value is within caps
let is_valid = caps.contains(50.0); // Returns true

// Union with another caps
let other_caps = Caps::new(10.0, 80.0);
let union_caps = caps.union(&other_caps); // Returns Caps { min: 0.0, max: 100.0 }
```

### Subsystem Implementation

```rust
use actor_core::*;
use async_trait::async_trait;

struct CombatSubsystem {
    system_id: String,
    priority: i64,
}

#[async_trait]
impl Subsystem for CombatSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }

    fn priority(&self) -> i64 {
        self.priority
    }

    async fn contribute(&self, actor: &Actor) -> Result<SubsystemOutput, ActorCoreError> {
        // Process actor and return contributions
        Ok(SubsystemOutput {
            caps: HashMap::new(),
            context: HashMap::new(),
            meta: HashMap::new(),
        })
    }
}
```

### Aggregator Usage

```rust
use actor_core::*;
use std::sync::Arc;

// Create aggregator
let cache = Arc::new(InMemoryCache::new(1000, 3600));
let registry = Arc::new(PluginRegistryImpl::new());
let caps_provider = Arc::new(CapsProviderImpl::new(Arc::new(CapLayerRegistryImpl::new())));
let aggregator = AggregatorImpl::new(registry, caps_provider, cache);

// Resolve actor
let snapshot = aggregator.resolve(&actor).await?;
```

## Performance Considerations

### Memory Management

- Use `Arc<T>` for shared ownership
- Implement `Drop` for custom cleanup
- Use `Box<dyn Trait>` for trait objects

### Async Operations

- Use `async_trait` for async trait methods
- Prefer `Pin<Box<dyn Future>>` for trait methods
- Use `tokio::spawn` for concurrent operations

### Caching

- Cache expensive calculations
- Use appropriate TTL values
- Implement cache invalidation strategies

## Best Practices

1. **Error Handling**: Always use `ActorCoreResult<T>` for fallible operations
2. **Validation**: Validate inputs early and fail fast
3. **Documentation**: Document all public APIs
4. **Testing**: Write comprehensive tests for all functionality
5. **Performance**: Profile and optimize critical paths
6. **Security**: Validate all external inputs
7. **Maintainability**: Keep code simple and readable
