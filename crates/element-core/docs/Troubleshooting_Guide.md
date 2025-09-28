# Element-Core Troubleshooting Guide

## Table of Contents

1. [Overview](#overview)
2. [Common Compilation Errors](#common-compilation-errors)
3. [Runtime Errors](#runtime-errors)
4. [Performance Issues](#performance-issues)
5. [Memory Issues](#memory-issues)
6. [Concurrency Issues](#concurrency-issues)
7. [Configuration Issues](#configuration-issues)
8. [Integration Issues](#integration-issues)
9. [Debugging Techniques](#debugging-techniques)
10. [Prevention Strategies](#prevention-strategies)

## Overview

This guide helps you diagnose and resolve common issues when working with Element-Core. It covers compilation errors, runtime issues, performance problems, and integration challenges.

### Quick Diagnosis Checklist

- [ ] Check compilation errors first
- [ ] Verify all dependencies are correct
- [ ] Ensure proper async/await usage
- [ ] Check thread safety requirements
- [ ] Validate configuration files
- [ ] Monitor memory usage
- [ ] Check error logs

## Common Compilation Errors

### 1. ElementCoreError Variants

**Error**: `expected value, found struct variant ElementCoreError::Validation`

**Problem**: Using old unit variant syntax instead of struct variants

**Solution**: Update to struct variant syntax

```rust
// Wrong
Err(ElementCoreError::Validation)

// Correct
Err(ElementCoreError::Validation { 
    message: "Invalid data".to_string() 
})
```

**Fix Script**:
```bash
# Find and replace all instances
find . -name "*.rs" -exec sed -i 's/ElementCoreError::Validation/ElementCoreError::Validation { message: "Error".to_string() }/g' {} \;
```

### 2. Missing Imports

**Error**: `unresolved import element_core::CoreElementalRegistry`

**Problem**: Module structure changed, old imports are invalid

**Solution**: Update imports to new module structure

```rust
// Wrong
use element_core::CoreElementalRegistry;

// Correct
use element_core::unified_registry::UnifiedElementRegistry;
```

### 3. Trait Implementation Issues

**Error**: `not all trait items implemented, missing: get_validation_errors`

**Problem**: Missing trait method implementation

**Solution**: Implement all required trait methods

```rust
impl Validatable for MyStruct {
    fn validate(&self) -> ElementCoreResult<()> {
        // Validation logic
        Ok(())
    }
    
    fn get_validation_errors(&self) -> Vec<String> {
        // Return validation errors
        vec![]
    }
}
```

### 4. Async/Await Issues

**Error**: `cannot be called in a synchronous context`

**Problem**: Calling async functions from sync context

**Solution**: Use proper async/await

```rust
// Wrong
let element = registry.get_element("fire");

// Correct
let element = registry.get_element("fire").await?;
```

### 5. Type Mismatches

**Error**: `mismatched types: expected i64, found i32`

**Problem**: Inconsistent integer types

**Solution**: Use consistent types throughout

```rust
// Ensure consistent types
let level: i64 = 10;
let experience: i64 = 1000;
```

## Runtime Errors

### 1. Element Not Found

**Error**: `ElementCoreError::ElementNotFound { element_id: "fire" }`

**Problem**: Trying to access non-existent element

**Solution**: Check element exists before accessing

```rust
// Check if element exists first
if registry.has_element("fire") {
    let element = registry.get_element("fire").await?;
    // Use element
} else {
    eprintln!("Element 'fire' not found");
    return Err(ElementCoreError::ElementNotFound { 
        element_id: "fire".to_string() 
    });
}
```

### 2. Validation Errors

**Error**: `ElementCoreError::Validation { message: "Invalid data" }`

**Problem**: Data validation failed

**Solution**: Validate data before use

```rust
// Validate before using
let element = ElementDefinition::new();
if let Err(e) = element.validate() {
    eprintln!("Validation failed: {}", e);
    return Err(e);
}
```

### 3. Index Out of Bounds

**Error**: `ElementCoreError::IndexOutOfBounds { index: 50, max: 49 }`

**Problem**: Accessing array index beyond bounds

**Solution**: Check bounds before access

```rust
// Check bounds before access
if index < MAX_ELEMENTS {
    let value = mastery_levels[index];
    // Use value
} else {
    return Err(ElementCoreError::IndexOutOfBounds { 
        index, 
        max: MAX_ELEMENTS - 1 
    });
}
```

### 4. Configuration Errors

**Error**: `ElementCoreError::Config { message: "Invalid configuration" }`

**Problem**: Configuration file is invalid or missing

**Solution**: Validate configuration

```rust
// Validate configuration
let config = load_config().await?;
if let Err(e) = config.validate() {
    eprintln!("Configuration validation failed: {}", e);
    return Err(e);
}
```

## Performance Issues

### 1. Slow Element Lookups

**Symptoms**: Element lookups taking > 1μs

**Diagnosis**: Check cache hit rate and data structure choice

**Solution**:
```rust
// Check cache performance
let cache_stats = registry.get_cache_stats().await;
println!("Cache hit rate: {:.2}%", 
    cache_stats.get("hit_rate").unwrap_or(&0.0) * 100.0);

// Optimize cache size if needed
if cache_stats.get("hit_rate").unwrap_or(&0.0) < 0.95 {
    registry.update_cache_config(CacheConfig {
        max_size: 2000,
        // ... other config
    }).await?;
}
```

### 2. High Memory Usage

**Symptoms**: Memory usage growing over time

**Diagnosis**: Check for memory leaks and inefficient data structures

**Solution**:
```rust
// Monitor memory usage
use std::alloc::{GlobalAlloc, System, Layout};

pub struct MemoryMonitor {
    total_allocated: std::sync::atomic::AtomicUsize,
}

unsafe impl GlobalAlloc for MemoryMonitor {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.total_allocated.fetch_add(layout.size(), std::sync::atomic::Ordering::Relaxed);
        System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.total_allocated.fetch_sub(layout.size(), std::sync::atomic::Ordering::Relaxed);
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static GLOBAL: MemoryMonitor = MemoryMonitor {
    total_allocated: std::sync::atomic::AtomicUsize::new(0),
};
```

### 3. Slow Contribution Aggregation

**Symptoms**: Aggregation taking > 100μs

**Diagnosis**: Check contributor count and individual contributor performance

**Solution**:
```rust
// Profile individual contributors
for contributor in contributors {
    let start = std::time::Instant::now();
    let contribution = contributor.contribute_element_stats(actor, element_type).await?;
    let duration = start.elapsed();
    
    if duration.as_micros() > 10 {
        eprintln!("Slow contributor: {} took {:?}", 
            contributor.system_id(), duration);
    }
}
```

## Memory Issues

### 1. Memory Leaks

**Symptoms**: Memory usage continuously growing

**Diagnosis**: Check for circular references and unclosed resources

**Solution**:
```rust
// Use weak references to break cycles
use std::rc::{Rc, Weak};

pub struct ElementNode {
    pub data: ElementDefinition,
    pub parent: Option<Weak<ElementNode>>,
    pub children: Vec<Rc<ElementNode>>,
}

// Properly close resources
impl Drop for ElementRegistry {
    fn drop(&mut self) {
        // Clean up resources
        self.cache.clear();
        self.metrics.reset();
    }
}
```

### 2. Out of Memory

**Symptoms**: Program crashes with out of memory error

**Diagnosis**: Check for excessive allocations and large data structures

**Solution**:
```rust
// Use streaming for large datasets
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

async fn load_large_dataset(path: &str) -> ElementCoreResult<()> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    while let Some(line) = lines.next_line().await? {
        let element: ElementDefinition = serde_json::from_str(&line)?;
        registry.register_element(element).await?;
    }
    
    Ok(())
}
```

### 3. Memory Fragmentation

**Symptoms**: High memory usage despite small data

**Diagnosis**: Frequent allocations/deallocations causing fragmentation

**Solution**:
```rust
// Use object pooling
pub struct ElementPool {
    available: Vec<ElementDefinition>,
    in_use: HashSet<*mut ElementDefinition>,
}

impl ElementPool {
    pub fn get(&mut self) -> Option<ElementDefinition> {
        self.available.pop()
    }
    
    pub fn return_element(&mut self, element: ElementDefinition) {
        self.available.push(element);
    }
}
```

## Concurrency Issues

### 1. Deadlocks

**Symptoms**: Program hangs indefinitely

**Diagnosis**: Check for lock ordering and nested locks

**Solution**:
```rust
// Always acquire locks in the same order
let registry1 = registry1.lock().unwrap();
let registry2 = registry2.lock().unwrap();
// Use both registries

// Avoid nested locks
// Bad
let registry = registry.lock().unwrap();
let element = registry.get_element("fire").lock().unwrap();

// Good
let element = {
    let registry = registry.lock().unwrap();
    registry.get_element("fire")
};
```

### 2. Race Conditions

**Symptoms**: Inconsistent results, data corruption

**Diagnosis**: Check for shared mutable state

**Solution**:
```rust
// Use atomic operations for shared state
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ThreadSafeCounter {
    count: AtomicUsize,
}

impl ThreadSafeCounter {
    pub fn increment(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }
}
```

### 3. Lock Contention

**Symptoms**: Poor performance with multiple threads

**Diagnosis**: Check for excessive lock usage

**Solution**:
```rust
// Use read-write locks for read-heavy workloads
use std::sync::RwLock;

let registry = Arc::new(RwLock::new(UnifiedElementRegistry::new()));

// Multiple readers can access simultaneously
let element = registry.read().unwrap().get_element("fire");

// Only one writer at a time
let mut registry = registry.write().unwrap();
registry.register_element(element).await?;
```

## Configuration Issues

### 1. Invalid Configuration File

**Error**: `ElementCoreError::Config { message: "Invalid YAML" }`

**Problem**: Configuration file has syntax errors

**Solution**: Validate YAML syntax

```rust
// Validate YAML before loading
use serde_yaml;

let content = std::fs::read_to_string("config.yaml")?;
let config: RegistryConfig = serde_yaml::from_str(&content)
    .map_err(|e| ElementCoreError::Config { 
        message: format!("Invalid YAML: {}", e) 
    })?;
```

### 2. Missing Configuration

**Error**: `ElementCoreError::Config { message: "Configuration file not found" }`

**Problem**: Configuration file is missing

**Solution**: Provide default configuration

```rust
// Provide default configuration
fn load_config_with_defaults(path: &str) -> ElementCoreResult<RegistryConfig> {
    if std::path::Path::new(path).exists() {
        load_config(path).await
    } else {
        Ok(RegistryConfig::default())
    }
}
```

### 3. Configuration Validation Errors

**Error**: `ElementCoreError::Config { message: "Invalid cache size" }`

**Problem**: Configuration values are invalid

**Solution**: Validate configuration values

```rust
impl Validatable for RegistryConfig {
    fn validate(&self) -> ElementCoreResult<()> {
        if self.cache.max_size == 0 {
            return Err(ElementCoreError::Config { 
                message: "Cache size must be greater than 0".to_string() 
            });
        }
        
        if self.performance.max_elements > MAX_ELEMENTS {
            return Err(ElementCoreError::Config { 
                message: format!("Max elements cannot exceed {}", MAX_ELEMENTS) 
            });
        }
        
        Ok(())
    }
}
```

## Integration Issues

### 1. Dependency Conflicts

**Error**: `failed to select a version for element-core`

**Problem**: Version conflicts between dependencies

**Solution**: Resolve version conflicts

```toml
# Cargo.toml
[dependencies]
element-core = { path = "../element-core", version = "0.1.0" }
actor-core = { path = "../actor-core", version = "0.1.0" }

# Use specific versions to avoid conflicts
tokio = "=1.0.0"
serde = "=1.0.0"
```

### 2. Trait Implementation Conflicts

**Error**: `conflicting implementations of trait`

**Problem**: Multiple trait implementations for the same type

**Solution**: Use trait bounds to disambiguate

```rust
// Use trait bounds to disambiguate
impl<T: ElementContributor + Send + Sync> MyTrait for T {
    // Implementation
}
```

### 3. Async Runtime Issues

**Error**: `no reactor running`

**Problem**: Calling async functions without async runtime

**Solution**: Use proper async runtime

```rust
// Use tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Your async code here
    Ok(())
}

// Or use block_on for sync context
let result = tokio::runtime::Runtime::new()?.block_on(async {
    registry.get_element("fire").await
});
```

## Debugging Techniques

### 1. Enable Debug Logging

```rust
// Enable debug logging
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

// Or use tracing
use tracing::{info, debug, error};

#[instrument]
async fn get_element(&self, element_id: &str) -> ElementCoreResult<ElementDefinition> {
    debug!("Getting element: {}", element_id);
    // Implementation
}
```

### 2. Use Debugger

```rust
// Add debug breakpoints
#[cfg(debug_assertions)]
fn debug_breakpoint() {
    std::process::abort(); // Or use debugger breakpoint
}

// Use in your code
if element_id == "debug" {
    debug_breakpoint();
}
```

### 3. Add Assertions

```rust
// Add assertions for debugging
fn validate_element(element: &ElementDefinition) -> ElementCoreResult<()> {
    assert!(!element.id.is_empty(), "Element ID cannot be empty");
    assert!(element.base_properties.power_multiplier > 0.0, "Power multiplier must be positive");
    
    // Rest of validation
    Ok(())
}
```

### 4. Use Profiling Tools

```rust
// Use flamegraph for profiling
use flame;

fn expensive_function() {
    flame::start("expensive_function");
    
    // Your code here
    
    flame::end("expensive_function");
}

// Generate flamegraph
flame::dump_html(&mut std::fs::File::create("flamegraph.html").unwrap()).unwrap();
```

## Prevention Strategies

### 1. Use Type Safety

```rust
// Use newtype pattern for type safety
#[derive(Debug, Clone, PartialEq)]
pub struct ElementId(String);

impl ElementId {
    pub fn new(id: String) -> ElementCoreResult<Self> {
        if id.is_empty() {
            return Err(ElementCoreError::Validation { 
                message: "Element ID cannot be empty".to_string() 
            });
        }
        Ok(Self(id))
    }
}
```

### 2. Validate Early

```rust
// Validate data as early as possible
pub fn create_element(id: String, properties: ElementProperties) -> ElementCoreResult<ElementDefinition> {
    // Validate inputs
    ElementId::new(id.clone())?;
    properties.validate()?;
    
    // Create element
    Ok(ElementDefinition {
        id,
        properties,
        // ... other fields
    })
}
```

### 3. Use Error Handling

```rust
// Use proper error handling
fn process_element(element: ElementDefinition) -> ElementCoreResult<()> {
    element.validate()?;
    
    // Process element
    Ok(())
}

// Handle errors appropriately
match process_element(element) {
    Ok(()) => println!("Element processed successfully"),
    Err(ElementCoreError::Validation { message }) => {
        eprintln!("Validation error: {}", message);
    },
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

### 4. Write Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_element_creation() {
        let element = ElementDefinition::new("fire", ElementProperties::default());
        assert!(element.is_ok());
    }
    
    #[test]
    fn test_validation() {
        let mut element = ElementDefinition::new("fire", ElementProperties::default()).unwrap();
        element.id = String::new(); // Make invalid
        assert!(element.validate().is_err());
    }
}
```

### 5. Monitor Performance

```rust
// Monitor performance metrics
let metrics = registry.get_metrics().await;
for (key, value) in metrics {
    if value > 1000.0 { // Threshold
        eprintln!("Warning: {} is high: {}", key, value);
    }
}
```

This troubleshooting guide should help you resolve most issues with Element-Core. If you encounter issues not covered here, please check the error logs and consider opening an issue with detailed information about your problem.