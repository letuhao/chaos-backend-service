# Actor Core Performance Migration Guide

## Overview

This guide provides step-by-step instructions for migrating from the current `actor-core` system to the new `actor-core-performance` system while maintaining full backward compatibility.

## Migration Strategy

### Phase 1: Preparation (Week 1-2)
- Set up `actor-core-performance` crate
- Implement core types and traits
- Add feature flags to main crate
- Run comprehensive tests

### Phase 2: Gradual Migration (Week 3-6)
- Migrate performance-critical systems first
- Use A/B testing to validate improvements
- Monitor performance metrics
- Fix any compatibility issues

### Phase 3: Full Migration (Week 7-8)
- All systems use performance version
- Remove base version (optional)
- Optimize further based on usage patterns

## Step-by-Step Migration

### Step 1: Set Up Performance Crate

#### 1.1 Create New Crate
```bash
cd chaos-backend-service/crates
cargo new actor-core-performance
```

#### 1.2 Update Cargo.toml
```toml
[package]
name = "actor-core-performance"
version = "0.1.0"
edition = "2021"

[dependencies]
actor-core = { path = "../actor-core" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

#### 1.3 Create Basic Structure
```rust
// src/lib.rs
pub mod types;
pub mod registry;
pub mod config;
pub mod traits;

// Re-export main types
pub use types::*;
pub use traits::*;
```

### Step 2: Implement Core Types

#### 2.1 Create ActorGodClass
```rust
// src/types/actor_god_class.rs
use actor_core::Actor as BaseActor;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGodClass {
    // Base actor fields (inherited)
    pub id: Uuid,
    pub name: String,
    pub race: String,
    pub lifespan: i64,
    pub age: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,
    pub subsystems: Vec<actor_core::Subsystem>,
    pub data: std::collections::HashMap<String, serde_json::Value>,
    
    // Performance-optimized hardcoded fields
    pub total_health: f64,
    pub total_mana: f64,
    pub total_stamina: f64,
    // ... other fields
}

impl ActorGodClass {
    pub fn new(name: String, race: String) -> Self {
        // Implementation
    }
    
    // Zero-cost access to base actor
    pub fn as_base(&self) -> &BaseActor {
        unsafe { std::mem::transmute(self) }
    }
    
    // Conversion methods
    pub fn from_base(base: BaseActor) -> Self { /* ... */ }
    pub fn to_base(self) -> BaseActor { /* ... */ }
}
```

#### 2.2 Create Subsystem Stats
```rust
// src/types/jindan_stats.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JindanStats {
    pub vital_essence: f64,
    pub qi_control: f64,
    pub meridian_strength: f64,
    // ... other fields
}

// src/types/rpg_stats.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgStats {
    pub strength: f64,
    pub intelligence: f64,
    pub willpower: f64,
    // ... other fields
}
```

### Step 3: Implement Traits

#### 3.1 Create ActorInterface
```rust
// src/traits/actor_interface.rs
pub trait ActorInterface {
    // Core stats
    fn get_health(&self) -> f64;
    fn get_mana(&self) -> f64;
    fn get_stamina(&self) -> f64;
    // ... other methods
    
    // Setters
    fn set_health(&mut self, value: f64);
    fn set_mana(&mut self, value: f64);
    fn set_stamina(&mut self, value: f64);
    // ... other methods
}
```

#### 3.2 Implement for Both Types
```rust
// For BaseActor
impl ActorInterface for actor_core::Actor {
    fn get_health(&self) -> f64 {
        self.data.get("health").and_then(|v| v.as_f64()).unwrap_or(0.0)
    }
    // ... other implementations
}

// For ActorGodClass
impl ActorInterface for ActorGodClass {
    fn get_health(&self) -> f64 {
        self.total_health
    }
    // ... other implementations
}
```

### Step 4: Update Main Crate

#### 4.1 Add Feature Flags
```toml
# Cargo.toml
[features]
default = ["base"]
base = []
performance = ["actor-core-performance"]

[dependencies]
actor-core-performance = { path = "../actor-core-performance", optional = true }
```

#### 4.2 Update lib.rs
```rust
// src/lib.rs
// Always export base types
pub use actor_core_base::*;

// Conditionally export performance types
#[cfg(feature = "performance")]
pub use actor_core_performance::*;
```

### Step 5: Migrate Systems

#### 5.1 Identify Performance-Critical Systems
```rust
// Systems that should be migrated first:
// - Game simulation loops
// - Real-time processing
// - High-frequency operations
// - Systems with performance bottlenecks
```

#### 5.2 Update System Code
```rust
// Before (using base Actor)
use actor_core::Actor;

fn process_actor(actor: &Actor) {
    let health = actor.data.get("health").and_then(|v| v.as_f64()).unwrap_or(0.0);
    // Process...
}

// After (using ActorInterface)
use actor_core::ActorInterface;

fn process_actor<T: ActorInterface>(actor: &T) {
    let health = actor.get_health();
    // Process...
}
```

#### 5.3 Gradual Migration
```rust
// Phase 1: Add performance types alongside base types
#[cfg(feature = "performance")]
use actor_core_performance::ActorGodClass;

use actor_core::Actor;

// Phase 2: Use generics for compatibility
fn process_actor<T: ActorInterface>(actor: &T) {
    // Works with both types
}

// Phase 3: Migrate to performance types
#[cfg(feature = "performance")]
type ActorType = ActorGodClass;

#[cfg(not(feature = "performance"))]
type ActorType = Actor;
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_actor_god_class_creation() {
        let actor = ActorGodClass::new("Test".to_string(), "Human".to_string());
        assert_eq!(actor.get_health(), 100.0);
    }
    
    #[test]
    fn test_performance_improvement() {
        // Benchmark tests
    }
}
```

### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_backward_compatibility() {
        // Test that existing code still works
    }
    
    #[test]
    fn test_performance_migration() {
        // Test migration from base to performance
    }
}
```

### Performance Tests
```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[test]
    fn test_performance_regression() {
        let iterations = 1_000_000;
        let start = std::time::Instant::now();
        
        // Perform benchmark
        let duration = start.elapsed();
        
        // Assert performance threshold
        assert!(duration.as_nanos() < 500_000_000); // 500ms threshold
    }
}
```

## A/B Testing

### Configuration-Based Selection
```rust
// config.toml
[performance]
use_performance_actors = true
performance_threshold = 1000

// Code
fn create_actor(name: String, race: String) -> Box<dyn ActorInterface> {
    if config.use_performance_actors {
        Box::new(ActorGodClass::new(name, race))
    } else {
        Box::new(Actor::new(name, race))
    }
}
```

### Runtime Selection
```rust
fn process_actors(actors: Vec<Box<dyn ActorInterface>>) {
    for actor in actors {
        // Process using common interface
        let health = actor.get_health();
        let mana = actor.get_mana();
        // ...
    }
}
```

## Monitoring and Validation

### Performance Metrics
```rust
use std::time::Instant;

fn benchmark_actor_operations() {
    let start = Instant::now();
    
    // Perform operations
    let duration = start.elapsed();
    
    // Log metrics
    println!("Operation took: {:?}", duration);
    
    // Assert performance threshold
    assert!(duration.as_nanos() < 1_000_000); // 1ms threshold
}
```

### Memory Usage Monitoring
```rust
use std::alloc::{GlobalAlloc, System};

fn monitor_memory_usage() {
    // Monitor memory usage
    let memory_usage = get_memory_usage();
    println!("Memory usage: {} bytes", memory_usage);
}
```

## Rollback Strategy

### Feature Flag Rollback
```toml
# Cargo.toml
[features]
default = ["base"]  # Rollback to base version
# performance = ["actor-core-performance"]  # Comment out
```

### Code Rollback
```rust
// Rollback to base types
use actor_core::Actor;

// Remove performance-specific code
// fn process_actor<T: ActorInterface>(actor: &T) {
//     // ...
// }

// Use base implementation
fn process_actor(actor: &Actor) {
    let health = actor.data.get("health").and_then(|v| v.as_f64()).unwrap_or(0.0);
    // ...
}
```

## Common Issues and Solutions

### Issue 1: Type Mismatches
```rust
// Problem: Type mismatch between base and performance types
fn process_actor(actor: &Actor) { ... }
let god_actor = ActorGodClass::new(...);
process_actor(&god_actor); // ❌ Type mismatch

// Solution: Use trait objects or generics
fn process_actor<T: ActorInterface>(actor: &T) { ... }
process_actor(&god_actor); // ✅ Works
```

### Issue 2: Performance Regression
```rust
// Problem: Performance worse than expected
// Solution: Check for unnecessary conversions
fn process_actor(actor: &ActorGodClass) {
    // ❌ BAD: Unnecessary conversion
    let base_actor = actor.to_base();
    process_base_actor(&base_actor);
    
    // ✅ GOOD: Direct access
    let health = actor.get_health_direct();
    process_health(health);
}
```

### Issue 3: Memory Usage Increase
```rust
// Problem: Memory usage higher than expected
// Solution: Check for memory leaks and unnecessary allocations
impl ActorGodClass {
    // ❌ BAD: Unnecessary allocation
    pub fn get_stats(&self) -> Vec<f64> {
        vec![self.total_health, self.total_mana, self.total_stamina]
    }
    
    // ✅ GOOD: Return references
    pub fn get_stats_ref(&self) -> &[f64] {
        &[self.total_health, self.total_mana, self.total_stamina]
    }
}
```

## Best Practices

### 1. Use Generics for Compatibility
```rust
// ✅ GOOD: Generic functions work with both types
fn process_actor<T: ActorInterface>(actor: &T) {
    let health = actor.get_health();
    // ...
}

// ❌ BAD: Concrete types limit compatibility
fn process_actor(actor: &ActorGodClass) {
    // Only works with ActorGodClass
}
```

### 2. Prefer Direct Access
```rust
// ✅ GOOD: Direct field access
let health = actor.get_health_direct();

// ❌ BAD: Unnecessary conversion
let base_actor = actor.to_base();
let health = base_actor.data.get("health").and_then(|v| v.as_f64()).unwrap_or(0.0);
```

### 3. Use Feature Flags
```rust
// ✅ GOOD: Feature-based compilation
#[cfg(feature = "performance")]
type ActorType = ActorGodClass;

#[cfg(not(feature = "performance"))]
type ActorType = Actor;
```

### 4. Monitor Performance
```rust
// ✅ GOOD: Continuous performance monitoring
fn benchmark_operation() {
    let start = Instant::now();
    // Perform operation
    let duration = start.elapsed();
    
    // Log and assert performance
    println!("Operation took: {:?}", duration);
    assert!(duration.as_nanos() < 1_000_000);
}
```

## Conclusion

This migration guide provides a comprehensive roadmap for migrating from the current `actor-core` system to the new `actor-core-performance` system. By following these steps and best practices, you can achieve significant performance improvements while maintaining full backward compatibility.

The key to successful migration is:
1. **Gradual approach**: Migrate systems incrementally
2. **Comprehensive testing**: Validate performance and compatibility
3. **Monitoring**: Track performance metrics throughout migration
4. **Rollback plan**: Be prepared to rollback if issues arise

With proper planning and execution, the migration can be completed successfully with minimal risk and maximum benefit.
