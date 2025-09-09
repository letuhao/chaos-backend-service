# Migration Guide

This guide helps you migrate from previous versions of Actor Core or integrate it into your existing system.

## Table of Contents

- [Version History](#version-history)
- [Breaking Changes](#breaking-changes)
- [Migration Steps](#migration-steps)
- [API Changes](#api-changes)
- [Configuration Changes](#configuration-changes)
- [Performance Considerations](#performance-considerations)
- [Troubleshooting](#troubleshooting)

## Version History

### v0.1.0 (Current)

- Initial release
- Core actor system
- Bucket processing
- Caps management
- Subsystem architecture
- Configuration loading
- Comprehensive testing
- Performance benchmarks

## Breaking Changes

### From Pre-v0.1.0

If you're migrating from a pre-release version, the following breaking changes apply:

#### 1. Actor Constructor

**Before:**
```rust
let actor = Actor::new(id, name, race, level);
```

**After:**
```rust
let actor = Actor::new(name, race);
```

#### 2. Contribution Processing

**Before:**
```rust
let result = process_contributions(contributions, caps);
```

**After:**
```rust
let result = process_contributions_in_order(contributions, initial_value, clamp_caps);
```

#### 3. Subsystem Interface

**Before:**
```rust
trait Subsystem {
    fn process(&self, actor: &Actor) -> Result<SubsystemOutput, Error>;
}
```

**After:**
```rust
#[async_trait]
trait Subsystem {
    fn system_id(&self) -> &str;
    fn priority(&self) -> i64;
    fn contribute(&self, actor: &Actor) -> Pin<Box<dyn Future<Output = Result<SubsystemOutput, ActorCoreError>> + Send + '_>>;
}
```

## Migration Steps

### Step 1: Update Dependencies

Update your `Cargo.toml`:

```toml
[dependencies]
actor-core = "0.1.0"
```

### Step 2: Update Imports

Update your imports:

```rust
// Old
use actor_core::{Actor, Contribution, Caps};

// New
use actor_core::*;
// or
use actor_core::{Actor, Contribution, Caps, Bucket, ActorCoreResult};
```

### Step 3: Update Actor Creation

**Before:**
```rust
let actor = Actor::new(
    uuid::Uuid::new_v4(),
    "Player1".to_string(),
    "Human".to_string(),
    10
);
```

**After:**
```rust
let actor = Actor::new("Player1".to_string(), "Human".to_string());
```

### Step 4: Update Contribution Processing

**Before:**
```rust
let contributions = vec![
    Contribution::new("strength", 10.0, "equipment"),
    Contribution::new("strength", 1.2, "buff"),
];

let result = process_contributions(contributions, &caps);
```

**After:**
```rust
let contributions = vec![
    Contribution::new("strength".to_string(), Bucket::Flat, 10.0, "equipment".to_string()),
    Contribution::new("strength".to_string(), Bucket::Mult, 1.2, "buff".to_string()),
];

let result = bucket_processor::process_contributions_in_order(
    contributions,
    0.0,
    Some(&caps)
);
```

### Step 5: Update Subsystem Implementation

**Before:**
```rust
struct MySubsystem;

impl Subsystem for MySubsystem {
    fn process(&self, actor: &Actor) -> Result<SubsystemOutput, Error> {
        // Implementation
    }
}
```

**After:**
```rust
use async_trait::async_trait;

struct MySubsystem {
    system_id: String,
    priority: i64,
}

#[async_trait]
impl Subsystem for MySubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }

    fn priority(&self) -> i64 {
        self.priority
    }

    async fn contribute(&self, actor: &Actor) -> Result<SubsystemOutput, ActorCoreError> {
        // Implementation
    }
}
```

### Step 6: Update Error Handling

**Before:**
```rust
use std::error::Error;

fn process_actor(actor: &Actor) -> Result<Snapshot, Box<dyn Error>> {
    // Implementation
}
```

**After:**
```rust
use actor_core::ActorCoreResult;

async fn process_actor(actor: &Actor) -> ActorCoreResult<Snapshot> {
    // Implementation
}
```

## API Changes

### New Types

#### EffectiveCaps

```rust
pub type EffectiveCaps = HashMap<String, Caps>;
```

#### Bucket

```rust
pub enum Bucket {
    Flat,
    Mult,
    PostAdd,
    Override,
    #[cfg(feature = "extra_buckets")]
    Exponential,
    #[cfg(feature = "extra_buckets")]
    Logarithmic,
    #[cfg(feature = "extra_buckets")]
    Conditional,
}
```

#### ActorCoreError

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

### New Traits

#### Aggregator

```rust
pub trait Aggregator: Send + Sync {
    async fn resolve(&self, actor: &Actor) -> ActorCoreResult<Snapshot>;
    async fn resolve_with_context(&self, actor: &Actor, context: Option<HashMap<String, serde_json::Value>>) -> ActorCoreResult<Snapshot>;
    async fn get_metrics(&self) -> AggregatorMetrics;
}
```

#### CapsProvider

```rust
pub trait CapsProvider: Send + Sync {
    async fn effective_caps_within_layer(&self, actor: &Actor, outputs: &[SubsystemOutput], layer: &str) -> ActorCoreResult<HashMap<String, Caps>>;
    async fn effective_caps_across_layers(&self, actor: &Actor, outputs: &[SubsystemOutput]) -> ActorCoreResult<EffectiveCaps>;
    async fn get_caps_for_dimension(&self, dimension: &str, actor: &Actor) -> ActorCoreResult<Option<Caps>>;
    async fn get_metrics(&self) -> CapStatistics;
}
```

### New Modules

#### bucket_processor

```rust
pub mod bucket_processor {
    pub fn process_contributions_in_order(
        contributions: Vec<Contribution>,
        initial_value: f64,
        clamp_caps: Option<&Caps>,
    ) -> ActorCoreResult<f64>;
    
    pub fn validate_contributions(contributions: &[Contribution]) -> ActorCoreResult<()>;
    pub fn group_contributions_by_bucket(contributions: &[Contribution]) -> HashMap<Bucket, Vec<&Contribution>>;
    pub fn get_bucket_processing_order() -> Vec<Bucket>;
}
```

#### registry::loader

```rust
pub mod registry::loader {
    pub fn load_cap_layers<P: AsRef<Path>>(path: P) -> Result<CapLayerRegistryImpl, LoaderError>;
    pub fn load_combiner<P: AsRef<Path>>(path: P) -> Result<CombinerRegistryImpl, LoaderError>;
    pub fn load_all<P: AsRef<Path>>(cfg_dir: P) -> Result<(CapLayerRegistryImpl, CombinerRegistryImpl), LoaderError>;
}
```

## Configuration Changes

### New Configuration Files

#### cap_layers.yaml

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

#### combiner.yaml

```yaml
combiner_rules:
  - name: attack
    bucket_order: [Flat, Mult, PostAdd, Override]
    clamp: true
  - name: defense
    bucket_order: [Flat, Mult, PostAdd, Override]
    clamp: true
```

### Configuration Loading

**Before:**
```rust
// No configuration loading
```

**After:**
```rust
use actor_core::registry::loader::*;

let cap_layers = load_cap_layers("configs/cap_layers.yaml")?;
let combiner = load_combiner("configs/combiner.yaml")?;
```

## Performance Considerations

### Caching

The new version includes comprehensive caching:

```rust
use actor_core::InMemoryCache;

let cache = Arc::new(InMemoryCache::new(1000, 3600));
```

### Async Operations

Many operations are now async:

```rust
// Before
let snapshot = aggregator.resolve(actor);

// After
let snapshot = aggregator.resolve(actor).await?;
```

### Memory Management

Use `Arc<T>` for shared ownership:

```rust
let registry = Arc::new(PluginRegistryImpl::new());
let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
let aggregator = AggregatorImpl::new(registry, caps_provider, cache);
```

## Troubleshooting

### Common Issues

#### 1. Compilation Errors

**Error**: `expected struct `Actor`, found struct `Actor``

**Solution**: Update your imports to use the new types:

```rust
use actor_core::*;
```

#### 2. Async/Await Issues

**Error**: `expected `Future`, found `()``

**Solution**: Make your functions async and await the results:

```rust
async fn process_actor(actor: &Actor) -> ActorCoreResult<Snapshot> {
    let snapshot = aggregator.resolve(actor).await?;
    Ok(snapshot)
}
```

#### 3. Trait Implementation Issues

**Error**: `not all trait items implemented`

**Solution**: Implement all required trait methods:

```rust
#[async_trait]
impl Subsystem for MySubsystem {
    fn system_id(&self) -> &str { /* ... */ }
    fn priority(&self) -> i64 { /* ... */ }
    async fn contribute(&self, actor: &Actor) -> Result<SubsystemOutput, ActorCoreError> { /* ... */ }
}
```

#### 4. Configuration Loading Issues

**Error**: `failed to load configuration`

**Solution**: Ensure configuration files exist and are valid:

```bash
# Validate configuration files
cargo test --test config_validation_tests
```

### Performance Issues

#### 1. Slow Performance

**Solution**: Enable caching and use appropriate cache sizes:

```rust
let cache = Arc::new(InMemoryCache::new(10000, 3600)); // Larger cache
```

#### 2. Memory Usage

**Solution**: Use `Arc<T>` for shared ownership and implement proper cleanup:

```rust
impl Drop for MySubsystem {
    fn drop(&mut self) {
        // Cleanup resources
    }
}
```

#### 3. Async Performance

**Solution**: Use appropriate async patterns:

```rust
// Use tokio::spawn for concurrent operations
let handles: Vec<_> = actors.into_iter()
    .map(|actor| tokio::spawn(async move {
        aggregator.resolve(&actor).await
    }))
    .collect();

let results: Vec<_> = futures::future::join_all(handles).await;
```

### Testing Issues

#### 1. Test Failures

**Solution**: Update your tests to use the new API:

```rust
#[tokio::test]
async fn test_actor_processing() {
    let actor = Actor::new("Test".to_string(), "Human".to_string());
    let result = aggregator.resolve(&actor).await;
    assert!(result.is_ok());
}
```

#### 2. Benchmark Failures

**Solution**: Update your benchmarks to use the new API:

```rust
fn bench_actor_creation(c: &mut Criterion) {
    c.bench_function("actor_creation", |b| {
        b.iter(|| {
            let actor = Actor::new("Test".to_string(), "Human".to_string());
            black_box(actor)
        })
    });
}
```

## Getting Help

If you encounter issues during migration:

1. **Check the documentation**: [API.md](API.md)
2. **Run the tests**: `cargo test`
3. **Check the examples**: [examples/](examples/)
4. **Open an issue**: [GitHub Issues](https://github.com/chaos-world/actor-core/issues)
5. **Join the community**: [Discord](https://discord.gg/chaos-world)

## Conclusion

This migration guide should help you successfully migrate to Actor Core v0.1.0. The new version provides significant improvements in performance, flexibility, and maintainability while maintaining backward compatibility where possible.

If you have any questions or need assistance, please don't hesitate to reach out to the community or open an issue on GitHub.
