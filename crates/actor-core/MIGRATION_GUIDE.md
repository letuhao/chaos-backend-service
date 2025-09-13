# Actor Core Migration Guide

This guide helps you migrate your code to use the new prelude-based API and understand the changes in the public API surface.

## Overview of Changes

The main changes in this version focus on:

1. **Cleaner API Surface**: Most implementation details are now hidden behind `#[doc(hidden)]`
2. **Prelude Module**: A new `prelude` module provides convenient access to commonly used types
3. **Simplified Imports**: Use `use actor_core::prelude::*;` for most use cases
4. **Stability Guarantees**: Clear documentation of what's stable vs experimental

## Migration Steps

### 1. Update Your Imports

**Before:**
```rust
use actor_core::*;
use actor_core::types::{Actor, Contribution, Snapshot};
use actor_core::interfaces::{Subsystem, Aggregator};
```

**After:**
```rust
use actor_core::prelude::*;

// That's it! All commonly used types are now available
```

### 2. Update Service Creation

**Before:**
```rust
use actor_core::service_factory::ServiceFactory;

let aggregator = ServiceFactory::create_aggregator(
    plugin_registry,
    combiner_registry,
    caps_provider,
    cache,
)?;
```

**After:**
```rust
use actor_core::prelude::*;

// Option 1: Quick setup for simple cases
let (aggregator, cache) = quick_setup().await?;

// Option 2: Manual setup for advanced cases
let cache = ServiceFactory::create_cache()?;
let plugin_registry = ServiceFactory::create_plugin_registry();
let combiner_registry = ServiceFactory::create_combiner_registry();
let caps_provider = ServiceFactory::create_caps_provider();

let aggregator = ServiceFactory::create_aggregator(
    plugin_registry,
    combiner_registry,
    caps_provider,
    cache,
)?;
```

### 3. Update Actor Creation

**Before:**
```rust
use actor_core::types::Actor;
use std::collections::HashMap;

let mut actor = Actor::new("player1".to_string(), "human".to_string());
let mut data = HashMap::new();
data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(10)));
actor.set_data(data);
```

**After:**
```rust
use actor_core::prelude::*;

// Option 1: Simple actor creation
let actor = create_simple_actor("player1", "human", 10);

// Option 2: Manual creation (still available)
let mut actor = Actor::new("player1".to_string(), "human".to_string());
let mut data = HashMap::new();
data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(10)));
actor.set_data(data);
```

### 4. Update Contribution Creation

**Before:**
```rust
use actor_core::types::Contribution;
use actor_core::enums::Bucket;

let contribution = Contribution::new(
    "strength".to_string(),
    Bucket::Flat,
    10.0,
    "equipment".to_string()
);
```

**After:**
```rust
use actor_core::prelude::*;

// Option 1: Simple contribution creation
let contribution = create_basic_contribution("strength", 10.0, "equipment");

// Option 2: Manual creation (still available)
let contribution = Contribution::new(
    "strength".to_string(),
    Bucket::Flat,
    10.0,
    "equipment".to_string()
);
```

### 5. Update Caps Creation

**Before:**
```rust
use actor_core::types::Caps;

let caps = Caps {
    min: Some(0.0),
    max: Some(100.0),
};
```

**After:**
```rust
use actor_core::prelude::*;

// Option 1: Simple caps creation
let caps = create_basic_caps(0.0, 100.0);

// Option 2: Manual creation (still available)
let caps = Caps {
    min: Some(0.0),
    max: Some(100.0),
};
```

## API Stability

### Stable API (v1.0.0+)

These components are guaranteed to remain compatible across minor versions:

- **Core Types**: `Actor`, `Contribution`, `CapContribution`, `Snapshot`, `Caps`
- **Enums**: `Bucket`, `CapMode`, `CapKind`, `AcrossLayerPolicy`, `Operator`
- **Traits**: `Subsystem`, `Aggregator`, `CapsProvider`, `PluginRegistry`, `CombinerRegistry`, `Cache`
- **Error Types**: `ActorCoreError`, `ActorCoreResult`
- **Service Factory**: `ServiceFactory`
- **Prelude Module**: `prelude`

### Beta API (may change in minor versions)

These components may change in minor versions but will be deprecated first:

- **Performance Monitoring**: `PerformanceProfiler`, `PerformanceTestSuite`, `PerformanceWorkflowManager`
- **Observability**: `ObservabilityManager`
- **Metrics**: `SubsystemMetrics`, `AggregatorMetrics`, `CacheStats`

### Internal API (not part of public API)

These modules are now hidden and may change without notice:

- `bucket_processor`
- `aggregator`
- `cache`
- `registry`
- `production`
- `metrics`
- `constants`
- `pools`
- `subsystems`
- `observability`

## Breaking Changes

### 1. Module Visibility

Some modules that were previously public are now hidden:

```rust
// This will no longer work:
use actor_core::bucket_processor::process_contributions_in_order;

// Use the prelude instead:
use actor_core::prelude::*;
// The function is still available, but through the prelude
```

### 2. Direct Implementation Access

Direct access to implementation modules is no longer recommended:

```rust
// This will no longer work:
use actor_core::aggregator::AggregatorImpl;

// Use the trait instead:
use actor_core::prelude::*;
// Create through ServiceFactory
```

## New Features

### 1. Quick Setup Function

```rust
use actor_core::prelude::*;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    let (aggregator, cache) = quick_setup().await?;
    // Ready to use!
    Ok(())
}
```

### 2. Convenience Functions

```rust
use actor_core::prelude::*;

// Create a simple actor with level
let actor = create_simple_actor("player1", "human", 10);

// Create a basic contribution
let contribution = create_basic_contribution("strength", 10.0, "equipment");

// Create basic caps
let caps = create_basic_caps(0.0, 100.0);
```

### 3. Feature Detection

```rust
use actor_core::prelude::*;

if has_feature("redis-cache") {
    println!("Redis cache support is available");
}
```

### 4. Build Information

```rust
use actor_core::prelude::*;

let info = get_build_info();
println!("Actor Core version: {}", info.version);
println!("Available features: {:?}", info.features);
```

## Migration Checklist

- [ ] Update imports to use `actor_core::prelude::*`
- [ ] Replace manual service creation with `quick_setup()` where appropriate
- [ ] Update actor creation to use `create_simple_actor()` where appropriate
- [ ] Update contribution creation to use `create_basic_contribution()` where appropriate
- [ ] Update caps creation to use `create_basic_caps()` where appropriate
- [ ] Remove direct imports of internal modules
- [ ] Test your application thoroughly
- [ ] Update documentation to reflect new API usage

## Getting Help

If you encounter issues during migration:

1. Check the [API Stability Report](api_stability.md)
2. Review the [examples](examples/) directory for updated usage patterns
3. Open an issue on GitHub with your specific migration question
4. Join our Discord community for real-time help

## Examples

See the updated examples in the `examples/` directory:

- `basic_usage.rs`: Shows the new prelude-based API
- `configuration_example.rs`: Demonstrates configuration usage
- `subsystem_example.rs`: Shows custom subsystem implementation
- `performance_workflow_example.rs`: Demonstrates performance monitoring

## Performance Notes

The new API surface doesn't change the underlying performance characteristics. All optimizations remain in place, and the new convenience functions are thin wrappers around the existing functionality.
