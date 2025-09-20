# Actor Core Performance Implementation Plan

## Overview

This document outlines the detailed implementation plan for `actor-core-performance`, including file structure, code examples, and step-by-step implementation guide.

## File Structure

```
chaos-backend-service/
├── crates/
│   ├── actor-core/                    # Existing base crate
│   │   ├── src/
│   │   │   ├── types/
│   │   │   │   ├── actor.rs          # Base Actor implementation
│   │   │   │   └── mod.rs
│   │   │   ├── registry/
│   │   │   │   ├── registry_manager.rs
│   │   │   │   └── mod.rs
│   │   │   ├── config/
│   │   │   │   ├── manager.rs
│   │   │   │   └── mod.rs
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── actor-core-performance/        # New performance crate
│       ├── src/
│       │   ├── types/
│       │   │   ├── actor_god_class.rs
│       │   │   ├── jindan_stats.rs
│       │   │   ├── rpg_stats.rs
│       │   │   └── mod.rs
│       │   ├── registry/
│       │   │   ├── performance_registry_manager.rs
│       │   │   └── mod.rs
│       │   ├── config/
│       │   │   ├── performance_config_manager.rs
│       │   │   └── mod.rs
│       │   ├── traits/
│       │   │   ├── actor_interface.rs
│       │   │   └── mod.rs
│       │   └── lib.rs
│       ├── Cargo.toml
│       └── README.md
└── docs/
    └── actor-core-performance/
        ├── README.md
        ├── IMPLEMENTATION_PLAN.md
        ├── PERFORMANCE_ANALYSIS.md
        └── MIGRATION_GUIDE.md
```

## Implementation Steps

### Step 1: Create Performance Crate

#### 1.1 Create Cargo.toml
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

#### 1.2 Create lib.rs
```rust
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
    pub total_strength: f64,
    pub total_agility: f64,
    pub total_endurance: f64,
    pub total_perception: f64,
    pub total_luck: f64,
    pub total_experience: f64,
    pub total_level: i32,
    pub total_lifespan: i64,
    pub total_wisdom: f64,
    
    // Subsystem stats
    pub jindan: JindanStats,
    pub rpg: RpgStats,
}

impl ActorGodClass {
    pub fn new(name: String, race: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            race,
            lifespan: 100,
            age: 0,
            created_at: now,
            updated_at: now,
            version: 1,
            subsystems: Vec::new(),
            data: std::collections::HashMap::new(),
            
            // Initialize performance fields
            total_health: 100.0,
            total_mana: 100.0,
            total_stamina: 100.0,
            total_strength: 10.0,
            total_agility: 10.0,
            total_endurance: 10.0,
            total_perception: 10.0,
            total_luck: 10.0,
            total_experience: 0.0,
            total_level: 1,
            total_lifespan: 100,
            total_wisdom: 10.0,
            
            jindan: JindanStats::new(),
            rpg: RpgStats::new(),
        }
    }
    
    // Zero-cost access to base actor
    pub fn as_base(&self) -> &BaseActor {
        unsafe { std::mem::transmute(self) }
    }
    
    // Conversion methods
    pub fn from_base(base: BaseActor) -> Self {
        // Convert base actor to god class
        // Implementation details...
    }
    
    pub fn to_base(self) -> BaseActor {
        BaseActor {
            id: self.id,
            name: self.name,
            race: self.race,
            lifespan: self.lifespan,
            age: self.age,
            created_at: self.created_at,
            updated_at: self.updated_at,
            version: self.version,
            subsystems: self.subsystems,
            data: self.data,
        }
    }
    
    // Performance-optimized methods
    pub fn get_health_direct(&self) -> f64 { self.total_health }
    pub fn get_mana_direct(&self) -> f64 { self.total_mana }
    pub fn get_stamina_direct(&self) -> f64 { self.total_stamina }
    
    pub fn set_health_direct(&mut self, value: f64) { self.total_health = value; }
    pub fn set_mana_direct(&mut self, value: f64) { self.total_mana = value; }
    pub fn set_stamina_direct(&mut self, value: f64) { self.total_stamina = value; }
    
    // Calculate total stats
    pub fn calculate_total_stats(&mut self) {
        self.total_health = self.jindan.vital_essence + self.rpg.strength * 10.0;
        self.total_mana = self.jindan.qi_control + self.rpg.intelligence * 10.0;
        self.total_stamina = self.jindan.body_constitution + self.rpg.endurance * 10.0;
        // ... other calculations
    }
}
```

#### 2.2 Create JindanStats
```rust
// src/types/jindan_stats.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JindanStats {
    pub vital_essence: f64,
    pub qi_control: f64,
    pub meridian_strength: f64,
    pub body_constitution: f64,
    pub soul_consciousness: f64,
    pub dao_comprehension: f64,
    pub fortune: f64,
}

impl JindanStats {
    pub fn new() -> Self {
        Self {
            vital_essence: 100.0,
            qi_control: 80.0,
            meridian_strength: 60.0,
            body_constitution: 70.0,
            soul_consciousness: 50.0,
            dao_comprehension: 40.0,
            fortune: 30.0,
        }
    }
    
    pub fn calculate_derived_stats(&mut self) {
        // Calculate derived stats
    }
}
```

#### 2.3 Create RpgStats
```rust
// src/types/rpg_stats.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgStats {
    pub strength: f64,
    pub intelligence: f64,
    pub willpower: f64,
    pub agility: f64,
    pub speed: f64,
    pub endurance: f64,
    pub perception: f64,
    pub luck: f64,
}

impl RpgStats {
    pub fn new() -> Self {
        Self {
            strength: 10.0,
            intelligence: 10.0,
            willpower: 10.0,
            agility: 10.0,
            speed: 10.0,
            endurance: 10.0,
            perception: 10.0,
            luck: 10.0,
        }
    }
    
    pub fn calculate_derived_stats(&mut self) {
        // Calculate derived stats
    }
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
    fn get_strength(&self) -> f64;
    fn get_agility(&self) -> f64;
    fn get_endurance(&self) -> f64;
    fn get_perception(&self) -> f64;
    fn get_luck(&self) -> f64;
    fn get_experience(&self) -> f64;
    fn get_level(&self) -> i32;
    fn get_lifespan(&self) -> i64;
    fn get_wisdom(&self) -> f64;
    
    // Setters
    fn set_health(&mut self, value: f64);
    fn set_mana(&mut self, value: f64);
    fn set_stamina(&mut self, value: f64);
    fn set_strength(&mut self, value: f64);
    fn set_agility(&mut self, value: f64);
    fn set_endurance(&mut self, value: f64);
    fn set_perception(&mut self, value: f64);
    fn set_luck(&mut self, value: f64);
    fn set_experience(&mut self, value: f64);
    fn set_level(&mut self, value: i32);
    fn set_lifespan(&mut self, value: i64);
    fn set_wisdom(&mut self, value: f64);
}
```

### Step 4: Implement Performance Components

#### 4.1 Create PerformanceRegistryManager
```rust
// src/registry/performance_registry_manager.rs
use actor_core::RegistryManager as BaseRegistryManager;
use std::collections::HashMap;
use std::sync::Arc;

pub struct PerformanceRegistryManager {
    base: BaseRegistryManager,
    // Performance-optimized caches
    resource_cache: HashMap<String, actor_core::ResourceDefinition>,
    category_cache: HashMap<String, actor_core::CategoryDefinition>,
    tag_cache: HashMap<String, actor_core::TagDefinition>,
}

impl PerformanceRegistryManager {
    pub fn new(config_manager: Arc<actor_core::ConfigurationManager>) -> Self {
        Self {
            base: BaseRegistryManager::new(config_manager),
            resource_cache: HashMap::new(),
            category_cache: HashMap::new(),
            tag_cache: HashMap::new(),
        }
    }
    
    // Performance-optimized methods
    pub fn get_resource_cached(&self, id: &str) -> Option<&actor_core::ResourceDefinition> {
        self.resource_cache.get(id)
    }
    
    pub fn get_category_cached(&self, id: &str) -> Option<&actor_core::CategoryDefinition> {
        self.category_cache.get(id)
    }
    
    pub fn get_tag_cached(&self, id: &str) -> Option<&actor_core::TagDefinition> {
        self.tag_cache.get(id)
    }
}
```

### Step 5: Integration with Main Crate

#### 5.1 Update Main Crate Cargo.toml
```toml
[features]
default = ["base"]
base = []
performance = ["actor-core-performance"]

[dependencies]
actor-core-performance = { path = "../actor-core-performance", optional = true }
```

#### 5.2 Update Main Crate lib.rs
```rust
// Re-export based on features
#[cfg(feature = "performance")]
pub use actor_core_performance::*;

// Always export base types
pub use actor_core_base::*;
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
        assert_eq!(actor.get_health_direct(), 100.0);
        assert_eq!(actor.get_mana_direct(), 100.0);
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

## Performance Benchmarks

### Benchmark Suite
```rust
// benchmarks/performance_comparison.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use actor_core::Actor;
use actor_core_performance::ActorGodClass;

fn benchmark_actor_hashmap(c: &mut Criterion) {
    let mut actor = Actor::new("Test".to_string(), "Human".to_string());
    c.bench_function("actor_hashmap", |b| {
        b.iter(|| {
            // Benchmark HashMap operations
        })
    });
}

fn benchmark_actor_god_class(c: &mut Criterion) {
    let mut actor = ActorGodClass::new("Test".to_string(), "Human".to_string());
    c.bench_function("actor_god_class", |b| {
        b.iter(|| {
            // Benchmark direct field access
        })
    });
}

criterion_group!(benches, benchmark_actor_hashmap, benchmark_actor_god_class);
criterion_main!(benches);
```

## Migration Guide

### Phase 1: Preparation
1. Create `actor-core-performance` crate
2. Implement core types and traits
3. Add feature flags to main crate
4. Run comprehensive tests

### Phase 2: Gradual Migration
1. Migrate performance-critical systems first
2. Use A/B testing to validate improvements
3. Monitor performance metrics
4. Fix any compatibility issues

### Phase 3: Full Migration
1. All systems use performance version
2. Remove base version (optional)
3. Optimize further based on usage patterns

## Conclusion

This implementation plan provides a comprehensive roadmap for creating `actor-core-performance` while maintaining full backward compatibility and achieving significant performance improvements. The modular approach allows for gradual migration and easy maintenance.
