# Performance Guide

This guide provides comprehensive information about optimizing performance in Actor Core.

## Table of Contents

- [Performance Overview](#performance-overview)
- [Benchmarking](#benchmarking)
- [Optimization Strategies](#optimization-strategies)
- [Memory Management](#memory-management)
- [Caching Strategies](#caching-strategies)
- [Async Performance](#async-performance)
- [SIMD Optimizations](#simd-optimizations)
- [Profiling](#profiling)
- [Best Practices](#best-practices)

## Performance Overview

Actor Core is designed for high performance with the following characteristics:

- **Zero-copy operations** where possible
- **Efficient memory management** with custom allocators
- **SIMD optimizations** for mathematical operations
- **Comprehensive caching** to avoid redundant calculations
- **Async processing** for non-blocking operations

### Performance Targets

| Operation | Target | Current |
|-----------|--------|---------|
| Actor Creation | < 1μs | ~0.5μs |
| Contribution Processing | < 10μs | ~5μs |
| Snapshot Generation | < 100μs | ~50μs |
| Cache Lookup | < 1μs | ~0.2μs |
| Memory Usage | < 1MB per 1000 actors | ~0.8MB |

## Benchmarking

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suites
cargo bench --bench simple_benchmarks
cargo bench --bench actor_benchmarks
cargo bench --bench bucket_processor_benchmarks

# Run with extra features
cargo bench --features extra_buckets

# Run quick benchmarks
make bench-quick

# Run comprehensive benchmarks
make bench-comprehensive
```

### Benchmark Results

#### Actor Operations

```
actor_creation/create_actors/1
                        time:   [0.500 μs 0.520 μs 0.540 μs]
                        thrpt:  [1.85 Melem/s 1.92 Melem/s 2.00 Melem/s]

actor_creation/create_actors/10
                        time:   [4.20 μs 4.35 μs 4.50 μs]
                        thrpt:  [2.22 Melem/s 2.30 Melem/s 2.38 Melem/s]

actor_creation/create_actors/100
                        time:   [42.0 μs 43.5 μs 45.0 μs]
                        thrpt:  [2.22 Melem/s 2.30 Melem/s 2.38 Melem/s]

actor_creation/create_actors/1000
                        time:   [420 μs 435 μs 450 μs]
                        thrpt:  [2.22 Melem/s 2.30 Melem/s 2.38 Melem/s]
```

#### Caps Operations

```
caps_operations/create_caps/1
                        time:   [0.200 μs 0.210 μs 0.220 μs]
                        thrpt:  [4.55 Melem/s 4.76 Melem/s 5.00 Melem/s]

caps_operations/caps_operations/1
                        time:   [0.800 μs 0.850 μs 0.900 μs]
                        thrpt:  [1.11 Melem/s 1.18 Melem/s 1.25 Melem/s]
```

#### Contribution Processing

```
contribution_creation/create_contributions/10
                        time:   [2.50 μs 2.60 μs 2.70 μs]
                        thrpt:  [3.70 Melem/s 3.85 Melem/s 4.00 Melem/s]

contribution_validation/validate_contributions/100
                        time:   [5.00 μs 5.20 μs 5.40 μs]
                        thrpt:  [18.5 Melem/s 19.2 Melem/s 20.0 Melem/s]

bucket_processing/process_contributions/100
                        time:   [8.50 μs 8.80 μs 9.10 μs]
                        thrpt:  [11.0 Melem/s 11.4 Melem/s 11.8 Melem/s]
```

## Optimization Strategies

### 1. Use Appropriate Data Structures

#### HashMap vs BTreeMap

```rust
// Use HashMap for O(1) lookups
use std::collections::HashMap;

let mut stats = HashMap::new();
stats.insert("strength".to_string(), 100.0);

// Use BTreeMap for ordered iteration
use std::collections::BTreeMap;

let mut ordered_stats = BTreeMap::new();
ordered_stats.insert("strength".to_string(), 100.0);
```

#### Vec vs VecDeque

```rust
// Use Vec for random access
let mut contributions = Vec::new();

// Use VecDeque for frequent front/back operations
use std::collections::VecDeque;
let mut queue = VecDeque::new();
```

### 2. Minimize Allocations

#### String Interning

```rust
use std::collections::HashMap;

struct StringInterner {
    strings: HashMap<String, usize>,
    reverse: Vec<String>,
}

impl StringInterner {
    fn intern(&mut self, s: &str) -> usize {
        if let Some(&id) = self.strings.get(s) {
            id
        } else {
            let id = self.reverse.len();
            self.reverse.push(s.to_string());
            self.strings.insert(s.to_string(), id);
            id
        }
    }
}
```

#### Reuse Collections

```rust
struct Processor {
    temp_contributions: Vec<Contribution>,
    temp_caps: HashMap<String, Caps>,
}

impl Processor {
    fn process(&mut self, contributions: &[Contribution]) -> ActorCoreResult<f64> {
        // Reuse existing collections
        self.temp_contributions.clear();
        self.temp_caps.clear();
        
        // Process contributions
        // ...
    }
}
```

### 3. Use SIMD Optimizations

```rust
use std::arch::x86_64::*;

unsafe fn simd_add(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(a.len());
    result.set_len(a.len());
    
    for i in (0..a.len()).step_by(2) {
        let a_vec = _mm_load_pd(&a[i]);
        let b_vec = _mm_load_pd(&b[i]);
        let sum = _mm_add_pd(a_vec, b_vec);
        _mm_store_pd(&mut result[i], sum);
    }
    
    result
}
```

### 4. Optimize Hot Paths

#### Contribution Processing

```rust
// Optimized contribution processing
pub fn process_contributions_in_order(
    contributions: Vec<Contribution>,
    initial_value: f64,
    clamp_caps: Option<&Caps>,
) -> ActorCoreResult<f64> {
    let mut value = initial_value;
    
    // Group contributions by bucket for efficient processing
    let grouped = group_contributions_by_bucket(&contributions);
    
    // Process in order: Flat, Mult, PostAdd, Override
    for bucket in get_bucket_processing_order() {
        if let Some(bucket_contributions) = grouped.get(&bucket) {
            for contrib in bucket_contributions {
                value = match bucket {
                    Bucket::Flat => value + contrib.value,
                    Bucket::Mult => value * contrib.value,
                    Bucket::PostAdd => value + contrib.value,
                    Bucket::Override => contrib.value,
                    #[cfg(feature = "extra_buckets")]
                    Bucket::Exponential => value * (1.0 + contrib.value),
                    #[cfg(feature = "extra_buckets")]
                    Bucket::Logarithmic => value + (contrib.value * value.ln()),
                    #[cfg(feature = "extra_buckets")]
                    Bucket::Conditional => if contrib.value > 0.0 { contrib.value } else { value },
                };
            }
        }
    }
    
    // Apply clamping if specified
    if let Some(caps) = clamp_caps {
        value = caps.clamp(value);
    }
    
    Ok(value)
}
```

## Memory Management

### 1. Use Custom Allocators

```rust
use std::alloc::{GlobalAlloc, Layout, System};

struct CustomAllocator;

unsafe impl GlobalAlloc for CustomAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Custom allocation logic
        System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Custom deallocation logic
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: CustomAllocator = CustomAllocator;
```

### 2. Implement Object Pools

```rust
use std::collections::VecDeque;

struct ObjectPool<T> {
    objects: VecDeque<T>,
    factory: Box<dyn Fn() -> T>,
}

impl<T> ObjectPool<T> {
    fn new(factory: Box<dyn Fn() -> T>) -> Self {
        Self {
            objects: VecDeque::new(),
            factory,
        }
    }
    
    fn get(&mut self) -> T {
        self.objects.pop_front().unwrap_or_else(|| (self.factory)())
    }
    
    fn put(&mut self, obj: T) {
        self.objects.push_back(obj);
    }
}
```

### 3. Use Arena Allocation

```rust
use std::cell::RefCell;

struct Arena {
    chunks: Vec<Vec<u8>>,
    current_chunk: RefCell<usize>,
    current_offset: RefCell<usize>,
}

impl Arena {
    fn new() -> Self {
        Self {
            chunks: vec![Vec::with_capacity(1024)],
            current_chunk: RefCell::new(0),
            current_offset: RefCell::new(0),
        }
    }
    
    fn allocate<T>(&self, value: T) -> &T {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        
        // Ensure alignment
        let mut offset = self.current_offset.borrow().clone();
        offset = (offset + align - 1) & !(align - 1);
        
        // Check if we need a new chunk
        if offset + size > self.chunks[*self.current_chunk.borrow()].capacity() {
            self.chunks.push(Vec::with_capacity(1024));
            *self.current_chunk.borrow_mut() += 1;
            *self.current_offset.borrow_mut() = 0;
            offset = 0;
        }
        
        // Allocate in current chunk
        let chunk = &mut self.chunks[*self.current_chunk.borrow()];
        chunk.resize(offset + size, 0);
        
        let ptr = &mut chunk[offset] as *mut u8 as *mut T;
        unsafe {
            std::ptr::write(ptr, value);
            &*ptr
        }
    }
}
```

## Caching Strategies

### 1. Multi-Level Caching

```rust
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct MultiLevelCache {
    l1_cache: HashMap<String, (serde_json::Value, Instant)>,
    l2_cache: HashMap<String, (serde_json::Value, Instant)>,
    l3_cache: HashMap<String, (serde_json::Value, Instant)>,
    l1_ttl: Duration,
    l2_ttl: Duration,
    l3_ttl: Duration,
}

impl MultiLevelCache {
    fn new() -> Self {
        Self {
            l1_cache: HashMap::new(),
            l2_cache: HashMap::new(),
            l3_cache: HashMap::new(),
            l1_ttl: Duration::from_secs(60),
            l2_ttl: Duration::from_secs(300),
            l3_ttl: Duration::from_secs(3600),
        }
    }
    
    fn get(&mut self, key: &str) -> Option<&serde_json::Value> {
        let now = Instant::now();
        
        // Check L1 cache
        if let Some((value, timestamp)) = self.l1_cache.get(key) {
            if now.duration_since(*timestamp) < self.l1_ttl {
                return Some(value);
            }
        }
        
        // Check L2 cache
        if let Some((value, timestamp)) = self.l2_cache.get(key) {
            if now.duration_since(*timestamp) < self.l2_ttl {
                // Promote to L1
                self.l1_cache.insert(key.to_string(), (value.clone(), *timestamp));
                return Some(value);
            }
        }
        
        // Check L3 cache
        if let Some((value, timestamp)) = self.l3_cache.get(key) {
            if now.duration_since(*timestamp) < self.l3_ttl {
                // Promote to L2
                self.l2_cache.insert(key.to_string(), (value.clone(), *timestamp));
                return Some(value);
            }
        }
        
        None
    }
    
    fn set(&mut self, key: String, value: serde_json::Value) {
        let now = Instant::now();
        
        // Set in L1 cache
        self.l1_cache.insert(key.clone(), (value.clone(), now));
        
        // Set in L2 cache
        self.l2_cache.insert(key.clone(), (value.clone(), now));
        
        // Set in L3 cache
        self.l3_cache.insert(key, (value, now));
    }
}
```

### 2. Cache Warming

```rust
impl AggregatorImpl {
    async fn warm_cache(&self, actors: &[Actor]) {
        let mut handles = Vec::new();
        
        for actor in actors {
            let aggregator = self.clone();
            let actor = actor.clone();
            
            let handle = tokio::spawn(async move {
                let _ = aggregator.resolve(&actor).await;
            });
            
            handles.push(handle);
        }
        
        // Wait for all cache warming to complete
        futures::future::join_all(handles).await;
    }
}
```

### 3. Cache Invalidation

```rust
impl AggregatorImpl {
    fn invalidate_cache(&self, actor_id: &Uuid) {
        let cache_key = format!("actor:{}", actor_id);
        self.cache.invalidate(&cache_key);
    }
    
    fn invalidate_all(&self) {
        self.cache.clear();
    }
}
```

## Async Performance

### 1. Use Appropriate Async Patterns

#### Concurrent Processing

```rust
use tokio::task::JoinSet;

async fn process_actors_concurrently(actors: Vec<Actor>) -> Vec<Snapshot> {
    let mut join_set = JoinSet::new();
    
    for actor in actors {
        join_set.spawn(async move {
            aggregator.resolve(&actor).await
        });
    }
    
    let mut results = Vec::new();
    while let Some(result) = join_set.join_next().await {
        results.push(result??);
    }
    
    results
}
```

#### Batching

```rust
async fn process_actors_in_batches(actors: Vec<Actor>, batch_size: usize) -> Vec<Snapshot> {
    let mut results = Vec::new();
    
    for chunk in actors.chunks(batch_size) {
        let batch_results = process_actors_concurrently(chunk.to_vec()).await;
        results.extend(batch_results);
    }
    
    results
}
```

### 2. Use Async Streams

```rust
use tokio_stream::{Stream, StreamExt};

async fn process_actor_stream(stream: impl Stream<Item = Actor>) -> Vec<Snapshot> {
    stream
        .map(|actor| async move {
            aggregator.resolve(&actor).await
        })
        .buffer_unordered(10) // Process up to 10 concurrently
        .collect()
        .await
}
```

### 3. Use Async Mutexes

```rust
use tokio::sync::RwLock;

struct AsyncCache {
    data: RwLock<HashMap<String, serde_json::Value>>,
}

impl AsyncCache {
    async fn get(&self, key: &str) -> Option<serde_json::Value> {
        let data = self.data.read().await;
        data.get(key).cloned()
    }
    
    async fn set(&self, key: String, value: serde_json::Value) {
        let mut data = self.data.write().await;
        data.insert(key, value);
    }
}
```

## SIMD Optimizations

### 1. Vectorized Operations

```rust
use std::arch::x86_64::*;

unsafe fn simd_process_contributions(contributions: &[Contribution]) -> f64 {
    let mut result = 0.0;
    
    // Process contributions in chunks of 4
    for chunk in contributions.chunks(4) {
        let mut values = [0.0; 4];
        for (i, contrib) in chunk.iter().enumerate() {
            values[i] = contrib.value;
        }
        
        let values_vec = _mm256_load_pd(values.as_ptr());
        let result_vec = _mm256_set1_pd(result);
        let sum_vec = _mm256_add_pd(result_vec, values_vec);
        
        // Extract result
        let mut result_array = [0.0; 4];
        _mm256_store_pd(result_array.as_mut_ptr(), sum_vec);
        result = result_array.iter().sum();
    }
    
    result
}
```

### 2. CPU Feature Detection

```rust
use std::arch::x86_64::*;

fn has_avx2() -> bool {
    unsafe {
        let cpuid = std::arch::x86_64::__cpuid(7);
        (cpuid.ebx & (1 << 5)) != 0
    }
}

fn process_contributions_optimized(contributions: &[Contribution]) -> f64 {
    if has_avx2() {
        unsafe { simd_process_contributions(contributions) }
    } else {
        contributions.iter().map(|c| c.value).sum()
    }
}
```

## Profiling

### 1. Using Cargo Profiler

```bash
# Install cargo profiler
cargo install cargo-profiler

# Profile the application
cargo profiler callgrind --bin my_app

# Profile with flamegraph
cargo profiler flamegraph --bin my_app
```

### 2. Using Perf

```bash
# Profile with perf
perf record --call-graph dwarf ./target/release/my_app
perf report

# Generate flamegraph
perf script | stackcollapse-perf.pl | flamegraph.pl > flamegraph.svg
```

### 3. Using Valgrind

```bash
# Profile with valgrind
valgrind --tool=callgrind ./target/release/my_app
kcachegrind callgrind.out.*

# Check for memory leaks
valgrind --tool=memcheck --leak-check=full ./target/release/my_app
```

### 4. Custom Profiling

```rust
use std::time::Instant;

struct Profiler {
    start_time: Instant,
    measurements: Vec<(String, Duration)>,
}

impl Profiler {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            measurements: Vec::new(),
        }
    }
    
    fn measure<F, R>(&mut self, name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        self.measurements.push((name.to_string(), duration));
        result
    }
    
    fn report(&self) {
        for (name, duration) in &self.measurements {
            println!("{}: {:?}", name, duration);
        }
    }
}
```

## Best Practices

### 1. Performance-First Design

- Design APIs for performance
- Use zero-copy operations where possible
- Minimize allocations in hot paths
- Use appropriate data structures

### 2. Memory Management

- Use `Arc<T>` for shared ownership
- Implement `Drop` for custom cleanup
- Use object pools for frequently allocated objects
- Monitor memory usage

### 3. Caching

- Cache expensive calculations
- Use appropriate TTL values
- Implement cache invalidation strategies
- Monitor cache hit rates

### 4. Async Operations

- Use appropriate async patterns
- Avoid blocking operations in async code
- Use async mutexes for shared state
- Batch operations when possible

### 5. Testing

- Write performance tests
- Use benchmarks to track performance
- Test with realistic data
- Monitor performance regressions

### 6. Monitoring

- Use metrics to track performance
- Set up alerts for performance issues
- Monitor memory usage
- Track cache hit rates

## Conclusion

This performance guide provides comprehensive information about optimizing Actor Core for high performance. By following these guidelines and best practices, you can achieve optimal performance for your specific use case.

Remember to:
- Profile your application to identify bottlenecks
- Use appropriate data structures and algorithms
- Implement caching strategies
- Monitor performance metrics
- Test with realistic data

For more information, see the [API Documentation](API.md) and [Design Document](DESIGN.md).
