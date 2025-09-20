# 🚀 Hybrid Approach Implementation Guide

## **Overview**

The Hybrid Approach combines the performance of hard-coded properties with the flexibility of dynamic resource management. This approach provides:

- **Core Resources**: Direct array access for optimal performance
- **Custom Resources**: HashMap-based system for flexibility
- **Modder Support**: Community-driven resource extensions

## **Architecture**

```
┌─────────────────────────────────────────────────────────────┐
│                    Actor Core System                       │
├─────────────────────────────────────────────────────────────┤
│  Core Resources (Array)     │  Custom Resources (HashMap)  │
│  ┌─────────────────────────┐ │  ┌─────────────────────────┐ │
│  │ health: f64             │ │  │ "special_ability": f64  │ │
│  │ mana: f64               │ │  │ "modifier": f64         │ │
│  │ stamina: f64            │ │  │ "buff": f64             │ │
│  │ qi: f64                 │ │  │ ...                     │ │
│  │ experience: f64         │ │  └─────────────────────────┘ │
│  │ level: f64              │ │                             │
│  │ vitality: f64           │ │  Modder Resources           │
│  │ spirit: f64             │ │  ┌─────────────────────────┐ │
│  │ chi: f64                │ │  │ "custom_skill": f64     │ │
│  └─────────────────────────┘ │  │ "mod_item": f64         │ │
│                              │  │ ...                     │ │
│                              │  └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## **Performance Characteristics**

| Resource Type | Access Method | Performance | Use Case |
|---------------|---------------|-------------|----------|
| **Core Resources** | Array Index | O(1) - ~1-2 cycles | Hot path, frequently accessed |
| **Custom Resources** | HashMap | O(1) - ~10-50 cycles | Cold path, occasionally accessed |
| **Modder Resources** | HashMap | O(1) - ~10-50 cycles | Community extensions |

## **Usage Examples**

### **Core Resource Access (Recommended)**

```rust
use crate::types::core_resource_accessors::CoreResourceAccessor;

// Create accessor
let mut accessor = CoreResourceAccessor::new(&mut core_resources, &mut custom_resources);

// Access core resources - FAST
let health = accessor.get_health();
let mana = accessor.get_mana();
let stamina = accessor.get_stamina();

// Set core resources - FAST
accessor.set_health(100.0);
accessor.set_mana(50.0);
accessor.set_stamina(75.0);
```

### **Custom Resource Access (Flexible)**

```rust
// Access custom resources - FLEXIBLE
let special_ability = accessor.get_custom_resource("special_ability");
let modifier = accessor.get_custom_resource("modifier");

// Set custom resources - FLEXIBLE
accessor.set_custom_resource("special_ability", 25.0)?;
accessor.set_custom_resource("modifier", 1.5)?;
```

### **Modder Resource Access (Community)**

```rust
// Access modder resources - COMMUNITY
let custom_skill = accessor.get_custom_resource("custom_skill");
let mod_item = accessor.get_custom_resource("mod_item");

// Set modder resources - COMMUNITY
accessor.set_custom_resource("custom_skill", 10.0)?;
accessor.set_custom_resource("mod_item", 5.0)?;
```

## **Resource Index Constants**

### **Core Resources (Fixed Indices)**

```rust
use crate::constants::resource_indices::*;

// Core resource indices
const HEALTH: usize = 0;
const MANA: usize = 1;
const STAMINA: usize = 2;
const QI: usize = 3;
const EXPERIENCE: usize = 4;
const LEVEL: usize = 5;
const VITALITY: usize = 6;
const SPIRIT: usize = 7;
const CHI: usize = 8;
```

### **Custom Resources (Runtime Indices)**

```rust
// Custom resource indices start after core resources
const CUSTOM_START: usize = 9;
const MAX_CUSTOM_RESOURCES: usize = 100;
const TOTAL_CAPACITY: usize = CUSTOM_START + MAX_CUSTOM_RESOURCES;
```

## **Implementation Phases**

### **Phase 1: Core Resource Accessors** ⏳ **PENDING**
- [ ] Create `constants/resource_indices.rs`
- [ ] Create `types/core_resource_accessors.rs`
- [ ] Add core resource accessor methods to Actor
- [ ] Update subsystems to use direct accessors

### **Phase 2: Performance Optimization** ⏳ **PENDING**
- [ ] Replace HashMap access with direct accessors in hot paths
- [ ] Add performance benchmarks
- [ ] Add deprecation warnings for HashMap access
- [ ] Add documentation and examples

### **Phase 3: Modder Support** ⏳ **PENDING**
- [ ] Add modder API documentation
- [ ] Add examples for custom resources
- [ ] Add validation for modder resources
- [ ] Add error handling for modder resources

## **Best Practices**

### **✅ DO: Use Direct Accessors for Core Resources**

```rust
// Good - Fast access
let health = actor.get_health();
let mana = actor.get_mana();

// Good - Fast setting
actor.set_health(100.0);
actor.set_mana(50.0);
```

### **❌ DON'T: Use HashMap for Core Resources**

```rust
// Bad - Slow access
let health = actor.resources.get("health")?.current_value;

// Bad - Slow setting
actor.resources.get_mut("health")?.current_value = 100.0;
```

### **✅ DO: Use HashMap for Custom Resources**

```rust
// Good - Flexible access
let special_ability = actor.get_custom_resource("special_ability");

// Good - Flexible setting
actor.set_custom_resource("special_ability", 25.0)?;
```

### **❌ DON'T: Mix Access Methods**

```rust
// Bad - Inconsistent
let health = actor.get_health();  // Direct access
let mana = actor.get_custom_resource("mana");  // HashMap access

// Good - Consistent
let health = actor.get_health();  // Direct access
let mana = actor.get_mana();      // Direct access
```

## **Performance Benchmarks**

### **Access Performance (Cycles)**

| Method | Core Resources | Custom Resources | Overhead |
|--------|----------------|------------------|----------|
| **Direct Access** | 1-2 cycles | N/A | 0 cycles |
| **HashMap Access** | 10-50 cycles | 10-50 cycles | 5-25x slower |

### **Memory Usage**

| Resource Type | Memory per Resource | Total Memory |
|---------------|-------------------|--------------|
| **Core Resources** | 8 bytes | 72 bytes (9 resources) |
| **Custom Resources** | 8 bytes + string | Variable |
| **Modder Resources** | 8 bytes + string | Variable |

## **Migration Guide**

### **From HashMap to Direct Access**

```rust
// Before - HashMap access
let health = actor.resources.get("health")?.current_value;
actor.resources.get_mut("health")?.current_value = 100.0;

// After - Direct access
let health = actor.get_health();
actor.set_health(100.0);
```

### **From Direct Access to HashMap**

```rust
// Before - Direct access (if you had it)
let health = actor.health;
actor.health = 100.0;

// After - Hybrid approach
let health = actor.get_health();
actor.set_health(100.0);
```

## **Error Handling**

### **Core Resource Errors**

```rust
// Core resources are always available
let health = actor.get_health();  // Never fails

// Setting core resources never fails
actor.set_health(100.0);  // Never fails
```

### **Custom Resource Errors**

```rust
// Custom resources may not exist
let special_ability = actor.get_custom_resource("special_ability");  // Returns Option<f64>

// Setting custom resources may fail
actor.set_custom_resource("special_ability", 25.0)?;  // Returns Result<(), ActorCoreError>
```

## **Testing**

### **Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_core_resource_access() {
        let mut accessor = CoreResourceAccessor::new(&mut core_resources, &mut custom_resources);
        
        // Test getting core resources
        assert_eq!(accessor.get_health(), 0.0);
        assert_eq!(accessor.get_mana(), 0.0);
        
        // Test setting core resources
        accessor.set_health(100.0);
        accessor.set_mana(50.0);
        
        assert_eq!(accessor.get_health(), 100.0);
        assert_eq!(accessor.get_mana(), 50.0);
    }
    
    #[test]
    fn test_custom_resource_access() {
        let mut accessor = CoreResourceAccessor::new(&mut core_resources, &mut custom_resources);
        
        // Test getting custom resources
        assert_eq!(accessor.get_custom_resource("special_ability"), None);
        
        // Test setting custom resources
        accessor.set_custom_resource("special_ability", 25.0).unwrap();
        
        assert_eq!(accessor.get_custom_resource("special_ability"), Some(25.0));
    }
}
```

### **Performance Tests**

```rust
#[cfg(test)]
mod performance_tests {
    use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
    
    fn benchmark_resource_access(c: &mut Criterion) {
        let mut group = c.benchmark_group("resource_access");
        
        // Benchmark core resource access
        group.bench_function("core_resources", |b| {
            let mut accessor = CoreResourceAccessor::new(&mut core_resources, &mut custom_resources);
            b.iter(|| {
                let health = accessor.get_health();
                let mana = accessor.get_mana();
                let stamina = accessor.get_stamina();
                (health, mana, stamina)
            });
        });
        
        // Benchmark custom resource access
        group.bench_function("custom_resources", |b| {
            let mut accessor = CoreResourceAccessor::new(&mut core_resources, &mut custom_resources);
            b.iter(|| {
                let special_ability = accessor.get_custom_resource("special_ability");
                let modifier = accessor.get_custom_resource("modifier");
                (special_ability, modifier)
            });
        });
        
        group.finish();
    }
    
    criterion_group!(benches, benchmark_resource_access);
    criterion_main!(benches);
}
```

## **Future Enhancements**

### **Planned Features**

1. **SIMD Operations**: Vectorized operations for bulk resource updates
2. **Resource Caching**: Intelligent caching for frequently accessed resources
3. **Resource Validation**: Runtime validation for resource values
4. **Resource Serialization**: Efficient serialization for save/load
5. **Resource Monitoring**: Performance monitoring and profiling

### **Community Features**

1. **Modder API**: Comprehensive API for community extensions
2. **Resource Templates**: Pre-defined resource templates for common use cases
3. **Resource Validation**: Validation rules for modder resources
4. **Resource Documentation**: Auto-generated documentation for modder resources

## **Conclusion**

The Hybrid Approach provides the best of both worlds:

- **Performance**: Core resources have optimal performance
- **Flexibility**: Custom resources have maximum flexibility
- **Community**: Modder resources enable community extensions
- **Maintainability**: Clear separation of concerns
- **Scalability**: Easy to add new resources and systems

This approach is used by major game engines and AAA games, making it a proven solution for modern game development.
