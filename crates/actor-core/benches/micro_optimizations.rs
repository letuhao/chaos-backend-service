//! Micro-optimization benchmarks for Actor Core.
//!
//! This module benchmarks the performance improvements from micro-optimizations
//! including smallvec, fxhash, atomic operations, and other optimizations.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use smallvec::{SmallVec, smallvec};
use fxhash::FxHashMap;
use ahash::AHashMap;

use actor_core::types::{Actor, Contribution, SubsystemOutput, SubsystemMeta};
use actor_core::enums::Bucket;
use actor_core::bucket_processor::{process_contributions_in_order, optimized::OptimizedBucketProcessor};
use actor_core::cache::optimized::{OptimizedL1Cache, BatchCacheOperations};
use actor_core::registry::optimized::{OptimizedPluginRegistry, RegistryBatchOperations};

/// Benchmark standard vs optimized bucket processing.
fn bench_bucket_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("bucket_processing");
    
    // Create test contributions
    let contributions: Vec<Contribution> = (0..100)
        .map(|i| Contribution::new(
            format!("dimension_{}", i % 10),
            if i % 4 == 0 { Bucket::Flat } else { Bucket::Mult },
            i as f64,
            format!("system_{}", i % 5)
        ))
        .collect();
    
    group.throughput(Throughput::Elements(contributions.len() as u64));
    
    // Benchmark standard processing
    group.bench_function("standard", |b| {
        b.iter(|| {
            black_box(process_contributions_in_order(
                black_box(contributions.clone()),
                black_box(100.0),
                black_box(None)
            ))
        })
    });
    
    // Benchmark optimized processing
    group.bench_function("optimized", |b| {
        b.iter(|| {
            black_box(OptimizedBucketProcessor::process_contributions_optimized(
                black_box(contributions.clone()),
                black_box(100.0),
                black_box(None)
            ))
        })
    });
    
    group.finish();
}

/// Benchmark hash map performance comparisons.
fn bench_hash_maps(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_maps");
    
    let keys: Vec<String> = (0..1000)
        .map(|i| format!("key_{}", i))
        .collect();
    
    let values: Vec<i32> = (0..1000).collect();
    
    group.throughput(Throughput::Elements(keys.len() as u64));
    
    // Benchmark standard HashMap
    group.bench_function("std_hashmap", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for (key, value) in keys.iter().zip(values.iter()) {
                map.insert(black_box(key.clone()), black_box(*value));
            }
            black_box(map)
        })
    });
    
    // Benchmark FxHashMap
    group.bench_function("fx_hashmap", |b| {
        b.iter(|| {
            let mut map = FxHashMap::default();
            for (key, value) in keys.iter().zip(values.iter()) {
                map.insert(black_box(key.clone()), black_box(*value));
            }
            black_box(map)
        })
    });
    
    // Benchmark AHashMap
    group.bench_function("ahash", |b| {
        b.iter(|| {
            let mut map = AHashMap::new();
            for (key, value) in keys.iter().zip(values.iter()) {
                map.insert(black_box(key.clone()), black_box(*value));
            }
            black_box(map)
        })
    });
    
    group.finish();
}

/// Benchmark SmallVec vs Vec performance.
fn bench_vectors(c: &mut Criterion) {
    let mut group = c.benchmark_group("vectors");
    
    let sizes = [1, 4, 8, 16, 32, 64];
    
    for size in sizes {
        group.throughput(Throughput::Elements(size as u64));
        
        // Benchmark Vec
        group.bench_with_input(BenchmarkId::new("vec", size), &size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::with_capacity(size);
                for i in 0..size {
                    vec.push(black_box(i));
                }
                black_box(vec)
            })
        });
        
        // Benchmark SmallVec
        group.bench_with_input(BenchmarkId::new("smallvec", size), &size, |b, &size| {
            b.iter(|| {
                let mut smallvec = SmallVec::new();
                for i in 0..size {
                    smallvec.push(black_box(i));
                }
                black_box(smallvec)
            })
        });
    }
    
    group.finish();
}

/// Benchmark cache operations.
fn bench_cache_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_operations");
    
    let cache = OptimizedL1Cache::new(1000);
    let keys: Vec<String> = (0..100)
        .map(|i| format!("key_{}", i))
        .collect();
    
    let value = serde_json::json!({"test": "value"});
    
    // Benchmark single cache operations
    group.bench_function("single_set", |b| {
        b.iter(|| {
            cache.set(black_box("test_key"), black_box(&value), black_box(None))
        })
    });
    
    group.bench_function("single_get", |b| {
        b.iter(|| {
            cache.get(black_box("test_key"))
        })
    });
    
    // Benchmark batch operations
    group.bench_function("batch_set", |b| {
        b.iter(|| {
            let items: Vec<(String, serde_json::Value)> = keys
                .iter()
                .map(|key| (key.clone(), value.clone()))
                .collect();
            BatchCacheOperations::set_many(&cache, &items, None)
        })
    });
    
    group.finish();
}

/// Benchmark registry operations.
fn bench_registry_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_operations");
    
    let registry = OptimizedPluginRegistry::new();
    
    // Create mock subsystems
    let subsystems: Vec<Arc<dyn actor_core::interfaces::Subsystem>> = (0..10)
        .map(|i| Arc::new(actor_core::production::MockSubsystem::new(format!("system_{}", i))))
        .collect();
    
    // Benchmark registration
    group.bench_function("register_subsystem", |b| {
        b.iter(|| {
            for subsystem in &subsystems {
                registry.register_subsystem(black_box(subsystem.clone()));
            }
        })
    });
    
    // Benchmark lookup
    group.bench_function("lookup_subsystems", |b| {
        let actor = Actor::new("test".to_string(), "human".to_string());
        b.iter(|| {
            registry.get_subsystems_for_actor(black_box(&actor))
        })
    });
    
    group.finish();
}

/// Benchmark atomic operations.
fn bench_atomic_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("atomic_operations");
    
    let counter = std::sync::atomic::AtomicU64::new(0);
    
    // Benchmark atomic increment
    group.bench_function("atomic_increment", |b| {
        b.iter(|| {
            counter.fetch_add(black_box(1), std::sync::atomic::Ordering::Relaxed)
        })
    });
    
    // Benchmark atomic load
    group.bench_function("atomic_load", |b| {
        b.iter(|| {
            counter.load(std::sync::atomic::Ordering::Relaxed)
        })
    });
    
    // Benchmark atomic store
    group.bench_function("atomic_store", |b| {
        b.iter(|| {
            counter.store(black_box(42), std::sync::atomic::Ordering::Relaxed)
        })
    });
    
    group.finish();
}

/// Benchmark string interning.
fn bench_string_interning(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_interning");
    
    let strings: Vec<String> = (0..100)
        .map(|i| format!("dimension_{}", i % 20)) // Many duplicates
        .collect();
    
    // Benchmark without interning
    group.bench_function("no_interning", |b| {
        b.iter(|| {
            let mut map = FxHashMap::default();
            for string in &strings {
                map.insert(black_box(string.clone()), black_box(42));
            }
            black_box(map)
        })
    });
    
    // Benchmark with interning
    group.bench_function("with_interning", |b| {
        b.iter(|| {
            let mut interner = actor_core::bucket_processor::optimized::DimensionInterner::new();
            let mut map = FxHashMap::default();
            for string in &strings {
                let interned = interner.intern(string);
                map.insert(black_box(interned), black_box(42));
            }
            black_box(map)
        })
    });
    
    group.finish();
}

/// Benchmark memory allocation patterns.
fn bench_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");
    
    let size = 1000;
    
    // Benchmark Vec allocation
    group.bench_function("vec_allocation", |b| {
        b.iter(|| {
            let vec = Vec::with_capacity(black_box(size));
            black_box(vec)
        })
    });
    
    // Benchmark SmallVec allocation
    group.bench_function("smallvec_allocation", |b| {
        b.iter(|| {
            let smallvec = SmallVec::<[i32; 16]>::new();
            black_box(smallvec)
        })
    });
    
    // Benchmark Box allocation
    group.bench_function("box_allocation", |b| {
        b.iter(|| {
            let boxed = Box::new(black_box(42));
            black_box(boxed)
        })
    });
    
    group.finish();
}

/// Benchmark branch prediction optimization.
fn bench_branch_prediction(c: &mut Criterion) {
    let mut group = c.benchmark_group("branch_prediction");
    
    let data: Vec<bool> = (0..1000)
        .map(|i| i % 10 == 0) // 90% false, 10% true
        .collect();
    
    // Benchmark with branch prediction hints
    group.bench_function("with_hints", |b| {
        b.iter(|| {
            let mut sum = 0;
            for &value in &data {
                if std::intrinsics::likely(!value) {
                    sum += 1;
                } else {
                    sum += 10;
                }
            }
            black_box(sum)
        })
    });
    
    // Benchmark without hints
    group.bench_function("without_hints", |b| {
        b.iter(|| {
            let mut sum = 0;
            for &value in &data {
                if value {
                    sum += 10;
                } else {
                    sum += 1;
                }
            }
            black_box(sum)
        })
    });
    
    group.finish();
}

/// Benchmark SIMD-friendly operations.
fn bench_simd_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_operations");
    
    let data: Vec<f64> = (0..1000).map(|i| i as f64).collect();
    
    // Benchmark standard summation
    group.bench_function("standard_sum", |b| {
        b.iter(|| {
            let sum: f64 = data.iter().sum();
            black_box(sum)
        })
    });
    
    // Benchmark chunked summation (SIMD-friendly)
    group.bench_function("chunked_sum", |b| {
        b.iter(|| {
            let mut sum = 0.0;
            for chunk in data.chunks(8) {
                sum += chunk.iter().sum::<f64>();
            }
            black_box(sum)
        })
    });
    
    // Benchmark parallel summation
    group.bench_function("parallel_sum", |b| {
        b.iter(|| {
            let sum: f64 = data.par_iter().sum();
            black_box(sum)
        })
    });
    
    group.finish();
}

// Configure criterion
criterion_group!(
    name = micro_optimizations;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(3));
    targets = 
        bench_bucket_processing,
        bench_hash_maps,
        bench_vectors,
        bench_cache_operations,
        bench_registry_operations,
        bench_atomic_operations,
        bench_string_interning,
        bench_memory_allocation,
        bench_branch_prediction,
        bench_simd_operations
);

criterion_main!(micro_optimizations);
