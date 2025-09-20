# Actor Core Performance Architecture Decisions

## Overview

This document outlines the key architectural decisions made during the design of `actor-core-performance`, including rationale, alternatives considered, and trade-offs.

## Decision 1: Inheritance vs Composition

### Decision
**Use inheritance pattern with embedded base actor**

### Rationale
- Maintains full compatibility with existing `actor-core` API
- Allows zero-cost access to base functionality
- Enables gradual migration without breaking changes
- Preserves all existing behavior and interfaces

### Alternatives Considered

#### Option A: Pure Composition
```rust
pub struct ActorGodClass {
    base_actor: Actor,
    performance_fields: PerformanceFields,
}
```
**Rejected because**: Requires wrapper methods for all base functionality, adds overhead

#### Option B: Trait-Only Approach
```rust
pub trait ActorInterface {
    fn get_health(&self) -> f64;
    // ... other methods
}
```
**Rejected because**: Loses type safety, requires dynamic dispatch

#### Option C: Complete Rewrite
```rust
pub struct ActorGodClass {
    // All fields embedded directly
}
```
**Rejected because**: Breaks backward compatibility, requires complete migration

### Trade-offs
- **Pros**: Full compatibility, zero overhead, gradual migration
- **Cons**: Slightly larger memory footprint, more complex implementation

## Decision 2: Memory Layout Strategy

### Decision
**Embed all fields directly in struct, optimize for cache performance**

### Rationale
- Maximizes cache locality
- Enables compiler optimizations
- Reduces memory indirection
- Improves performance for sequential access

### Alternatives Considered

#### Option A: HashMap for Performance Fields
```rust
pub struct ActorGodClass {
    base: Actor,
    performance_data: HashMap<String, f64>,
}
```
**Rejected because**: Loses performance benefits, adds HashMap overhead

#### Option B: Separate Performance Struct
```rust
pub struct ActorGodClass {
    base: Actor,
    performance: PerformanceStats,
}
```
**Rejected because**: Adds indirection, reduces cache locality

#### Option C: Union/Enum Approach
```rust
pub enum Actor {
    Base(BaseActor),
    Performance(PerformanceActor),
}
```
**Rejected because**: Adds runtime overhead, complicates usage

### Trade-offs
- **Pros**: Optimal performance, cache-friendly, compiler-optimized
- **Cons**: Larger struct size, less flexible for dynamic fields

## Decision 3: Type System Integration

### Decision
**Use generic functions with trait bounds for maximum compatibility**

### Rationale
- Enables zero-overhead abstraction
- Works with both base and performance types
- Maintains type safety
- Allows compile-time optimization

### Alternatives Considered

#### Option A: Trait Objects
```rust
fn process_actor(actor: &dyn ActorInterface) {
    // Dynamic dispatch
}
```
**Rejected because**: Adds runtime overhead, prevents inlining

#### Option B: Concrete Types Only
```rust
fn process_actor(actor: &ActorGodClass) {
    // Only works with performance type
}
```
**Rejected because**: Breaks compatibility, requires separate implementations

#### Option C: Enum Dispatch
```rust
pub enum ActorType {
    Base(Actor),
    Performance(ActorGodClass),
}
```
**Rejected because**: Adds runtime overhead, complicates usage

### Trade-offs
- **Pros**: Zero overhead, type safe, compatible
- **Cons**: More complex generic code, requires trait bounds

## Decision 4: Feature Flag Strategy

### Decision
**Use Cargo features for compile-time selection**

### Rationale
- Zero runtime overhead
- Clean separation of concerns
- Easy to enable/disable
- Standard Rust practice

### Alternatives Considered

#### Option A: Runtime Selection
```rust
let actor = if use_performance {
    ActorGodClass::new(...).into()
} else {
    Actor::new(...).into()
};
```
**Rejected because**: Adds runtime overhead, requires trait objects

#### Option B: Environment Variables
```rust
let use_performance = std::env::var("USE_PERFORMANCE").is_ok();
```
**Rejected because**: Runtime overhead, less reliable

#### Option C: Configuration Files
```rust
let config = load_config();
let use_performance = config.use_performance;
```
**Rejected because**: Runtime overhead, adds complexity

### Trade-offs
- **Pros**: Zero overhead, clean, standard
- **Cons**: Requires recompilation, less flexible

## Decision 5: Conversion Strategy

### Decision
**Provide both zero-cost and full conversion methods**

### Rationale
- Zero-cost access for performance-critical code
- Full conversion for compatibility
- Flexible usage patterns
- Clear performance implications

### Implementation
```rust
impl ActorGodClass {
    // Zero-cost access
    pub fn as_base(&self) -> &Actor { &self.base }
    
    // Full conversion (with cost)
    pub fn to_base(self) -> Actor { self.base }
    pub fn from_base(base: Actor) -> Self { /* ... */ }
}
```

### Alternatives Considered

#### Option A: Only Zero-Cost Access
```rust
impl ActorGodClass {
    pub fn as_base(&self) -> &Actor { &self.base }
}
```
**Rejected because**: Limits flexibility, some use cases need ownership

#### Option B: Only Full Conversion
```rust
impl ActorGodClass {
    pub fn to_base(self) -> Actor { self.base }
    pub fn from_base(base: Actor) -> Self { /* ... */ }
}
```
**Rejected because**: Loses zero-cost access benefits

#### Option C: Automatic Conversion
```rust
impl From<ActorGodClass> for Actor {
    fn from(god: ActorGodClass) -> Self { god.base }
}
```
**Rejected because**: Implicit conversions can be confusing

### Trade-offs
- **Pros**: Flexible, clear performance implications
- **Cons**: More API surface, requires understanding of costs

## Decision 6: Subsystem Stats Organization

### Decision
**Use separate structs for subsystem stats with direct embedding**

### Rationale
- Clear separation of concerns
- Enables subsystem-specific optimizations
- Maintains logical organization
- Allows independent evolution

### Implementation
```rust
pub struct ActorGodClass {
    // ... base fields
    jindan: JindanStats,
    rpg: RpgStats,
}

pub struct JindanStats {
    vital_essence: f64,
    qi_control: f64,
    // ... other fields
}

pub struct RpgStats {
    strength: f64,
    intelligence: f64,
    // ... other fields
}
```

### Alternatives Considered

#### Option A: Flat Structure
```rust
pub struct ActorGodClass {
    // ... base fields
    jindan_vital_essence: f64,
    jindan_qi_control: f64,
    rpg_strength: f64,
    rpg_intelligence: f64,
    // ... other fields
}
```
**Rejected because**: Becomes unwieldy, loses logical organization

#### Option B: HashMap for Subsystems
```rust
pub struct ActorGodClass {
    // ... base fields
    subsystems: HashMap<String, HashMap<String, f64>>,
}
```
**Rejected because**: Loses performance benefits, adds complexity

#### Option C: Enum for Subsystems
```rust
pub enum SubsystemStats {
    Jindan(JindanStats),
    Rpg(RpgStats),
}
```
**Rejected because**: Adds runtime overhead, complicates access

### Trade-offs
- **Pros**: Clear organization, subsystem-specific optimizations
- **Cons**: More structs, potential code duplication

## Decision 7: Error Handling Strategy

### Decision
**Use same error types as base crate for consistency**

### Rationale
- Maintains API consistency
- Enables seamless integration
- Preserves existing error handling patterns
- Reduces learning curve

### Implementation
```rust
use actor_core::ActorCoreResult;
use actor_core::ActorCoreError;

impl ActorGodClass {
    pub fn new(name: String, race: String) -> ActorCoreResult<Self> {
        // Use same error types
    }
}
```

### Alternatives Considered

#### Option A: Custom Error Types
```rust
pub enum PerformanceError {
    InvalidStats,
    ConversionFailed,
    // ...
}
```
**Rejected because**: Breaks consistency, adds complexity

#### Option B: Generic Error Types
```rust
pub enum Error<T> {
    Base(T),
    Performance(PerformanceError),
}
```
**Rejected because**: Overly complex, not necessary

#### Option C: Panic on Error
```rust
impl ActorGodClass {
    pub fn new(name: String, race: String) -> Self {
        // Panic on error
    }
}
```
**Rejected because**: Not idiomatic Rust, breaks error handling

### Trade-offs
- **Pros**: Consistent, familiar, integrates well
- **Cons**: Tied to base crate error types

## Decision 8: Serialization Strategy

### Decision
**Use same serialization approach as base crate**

### Rationale
- Maintains compatibility with existing serialized data
- Enables seamless data migration
- Preserves existing serialization patterns
- Reduces complexity

### Implementation
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGodClass {
    // ... fields with same serialization as base
}
```

### Alternatives Considered

#### Option A: Custom Serialization
```rust
impl Serialize for ActorGodClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Custom serialization logic
    }
}
```
**Rejected because**: Adds complexity, breaks compatibility

#### Option B: Different Serialization Format
```rust
// Use different format for performance data
#[derive(Serialize, Deserialize)]
pub struct PerformanceData {
    // Different format
}
```
**Rejected because**: Breaks compatibility, adds migration complexity

#### Option C: No Serialization
```rust
// Performance types don't support serialization
```
**Rejected because**: Limits usage, breaks compatibility

### Trade-offs
- **Pros**: Compatible, familiar, works with existing data
- **Cons**: Tied to base crate serialization

## Decision 9: Testing Strategy

### Decision
**Comprehensive testing with performance benchmarks**

### Rationale
- Ensures performance improvements are maintained
- Validates backward compatibility
- Catches regressions early
- Provides confidence in migration

### Implementation
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_improvement() {
        // Benchmark tests
    }
    
    #[test]
    fn test_backward_compatibility() {
        // Compatibility tests
    }
}
```

### Alternatives Considered

#### Option A: Minimal Testing
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_functionality() {
        // Basic tests only
    }
}
```
**Rejected because**: Insufficient for performance-critical code

#### Option B: Manual Testing Only
```rust
// No automated tests
```
**Rejected because**: Error-prone, not scalable

#### Option C: Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_actor_properties(actor in any::<ActorGodClass>()) {
        // Property-based tests
    }
}
```
**Rejected because**: Overkill for this use case, adds complexity

### Trade-offs
- **Pros**: Comprehensive, catches issues early
- **Cons**: More test code, longer CI times

## Decision 10: Documentation Strategy

### Decision
**Comprehensive documentation with examples and migration guides**

### Rationale
- Reduces learning curve
- Enables successful migration
- Provides clear guidance
- Reduces support burden

### Implementation
- README with overview and examples
- Implementation plan with step-by-step instructions
- Performance analysis with benchmarks
- Migration guide with best practices
- Architecture decisions document

### Alternatives Considered

#### Option A: Minimal Documentation
```rust
/// Actor God Class for performance
pub struct ActorGodClass {
    // ...
}
```
**Rejected because**: Insufficient for complex migration

#### Option B: Code-Only Documentation
```rust
/// Actor God Class for performance
/// 
/// This struct provides performance-optimized actor functionality
/// by using hardcoded fields instead of HashMap lookups.
pub struct ActorGodClass {
    // ...
}
```
**Rejected because**: Doesn't cover migration and usage patterns

#### Option C: External Documentation Only
```rust
// No inline documentation
```
**Rejected because**: Reduces discoverability, harder to maintain

### Trade-offs
- **Pros**: Clear guidance, reduces support burden
- **Cons**: More documentation to maintain

## Conclusion

These architectural decisions were made to balance performance, compatibility, and maintainability. The key principles guiding these decisions were:

1. **Performance First**: Optimize for the performance-critical use case
2. **Compatibility**: Maintain full backward compatibility
3. **Gradual Migration**: Enable incremental adoption
4. **Zero Overhead**: Minimize runtime costs
5. **Clear APIs**: Provide intuitive and well-documented interfaces

The resulting architecture provides significant performance improvements while maintaining full compatibility with the existing `actor-core` system, enabling successful migration with minimal risk.
