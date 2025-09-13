//! Performance benchmark tests for the Actor Core system.
//!
//! This module contains performance tests that measure the efficiency
//! of various operations and help identify performance bottlenecks.

use actor_core::types::{Actor, Subsystem as SubsystemStruct, Contribution, SubsystemOutput, Caps};
use actor_core::enums::Bucket;
use actor_core::interfaces::{Aggregator, PluginRegistry, Cache};
use actor_core::{AggregatorImpl, CapsProviderImpl};
use actor_core::registry::{CapLayerRegistryImpl, PluginRegistryImpl, CombinerRegistryImpl};
use actor_core::InMemoryCache;
use std::sync::Arc;
use std::time::Instant;

/// Mock subsystem for performance testing
struct PerformanceTestSubsystem {
    system_id: String,
    priority: i64,
    contribution_count: usize,
}

impl PerformanceTestSubsystem {
    fn new(system_id: String, priority: i64, contribution_count: usize) -> Self {
        Self {
            system_id,
            priority,
            contribution_count,
        }
    }
}

#[async_trait::async_trait]
impl actor_core::interfaces::Subsystem for PerformanceTestSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute(&self, _actor: &Actor) -> actor_core::ActorCoreResult<SubsystemOutput> {
        let mut output = SubsystemOutput::new(self.system_id.clone());
        
        // Generate multiple contributions for performance testing
        for i in 0..self.contribution_count {
            let contribution = Contribution::new(
                format!("dimension_{}", i % 10), // 10 different dimensions
                Bucket::Flat,
                i as f64,
                self.system_id.clone(),
            );
            output.add_primary(contribution);
        }
        
        Ok(output)
    }
}

/// Benchmark actor creation performance
#[tokio::test]
async fn benchmark_actor_creation() {
    const ITERATIONS: usize = 10000;
    
    let start = Instant::now();
    
    for i in 0..ITERATIONS {
        let _actor = Actor::new(format!("Actor_{}", i), "Human".to_string());
    }
    
    let duration = start.elapsed();
    let per_actor = duration.as_nanos() / ITERATIONS as u128;
    
    println!("Actor creation benchmark:");
    println!("  Total time: {:?}", duration);
    println!("  Per actor: {}ns", per_actor);
    println!("  Actors per second: {}", ITERATIONS as f64 / duration.as_secs_f64());
    
    // Should be very fast (less than 1000ns per actor)
    assert!(per_actor < 1000, "Actor creation too slow: {}ns per actor", per_actor);
}

/// Benchmark actor operations performance
#[tokio::test]
async fn benchmark_actor_operations() {
    const ITERATIONS: usize = 1000;
    
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Benchmark subsystem operations
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let subsystem = SubsystemStruct::new(format!("system_{}", i), i as i64);
        actor.add_subsystem(subsystem);
    }
    let subsystem_add_duration = start.elapsed();
    
    // Benchmark subsystem lookups
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let _found = actor.has_subsystem(&format!("system_{}", i));
    }
    let subsystem_lookup_duration = start.elapsed();
    
    // Benchmark buff operations
    let start = Instant::now();
    for i in 0..ITERATIONS {
        actor.add_buff(format!("buff_{}", i));
    }
    let buff_add_duration = start.elapsed();
    
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let _found = actor.has_buff(&format!("buff_{}", i));
    }
    let buff_lookup_duration = start.elapsed();
    
    println!("Actor operations benchmark:");
    println!("  Subsystem add: {:?} ({}ns per operation)", subsystem_add_duration, subsystem_add_duration.as_nanos() / ITERATIONS as u128);
    println!("  Subsystem lookup: {:?} ({}ns per operation)", subsystem_lookup_duration, subsystem_lookup_duration.as_nanos() / ITERATIONS as u128);
    println!("  Buff add: {:?} ({}ns per operation)", buff_add_duration, buff_add_duration.as_nanos() / ITERATIONS as u128);
    println!("  Buff lookup: {:?} ({}ns per operation)", buff_lookup_duration, buff_lookup_duration.as_nanos() / ITERATIONS as u128);
    
    // All operations should be reasonably fast
    assert!(subsystem_add_duration.as_nanos() / (ITERATIONS as u128) < 10000);
    assert!(subsystem_lookup_duration.as_nanos() / (ITERATIONS as u128) < 10000); // More realistic threshold
    assert!(buff_add_duration.as_nanos() / (ITERATIONS as u128) < 20000); // More realistic threshold
    assert!(buff_lookup_duration.as_nanos() / (ITERATIONS as u128) < 10000); // More realistic threshold
}

/// Benchmark caps operations performance
#[tokio::test]
async fn benchmark_caps_operations() {
    const ITERATIONS: usize = 100000;
    
    let caps = Caps::new(0.0, 1000.0);
    
    // Benchmark containment checks
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let _contains = caps.contains(i as f64 % 2000.0);
    }
    let containment_duration = start.elapsed();
    
    // Benchmark clamping operations
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let _clamped = caps.clamp(i as f64 % 2000.0);
    }
    let clamping_duration = start.elapsed();
    
    // Benchmark range operations
    let start = Instant::now();
    for _i in 0..ITERATIONS {
        let _range = caps.get_range();
        let _center = caps.get_center();
    }
    let range_duration = start.elapsed();
    
    println!("Caps operations benchmark:");
    println!("  Containment: {:?} ({}ns per operation)", containment_duration, containment_duration.as_nanos() / ITERATIONS as u128);
    println!("  Clamping: {:?} ({}ns per operation)", clamping_duration, clamping_duration.as_nanos() / ITERATIONS as u128);
    println!("  Range ops: {:?} ({}ns per operation)", range_duration, range_duration.as_nanos() / ITERATIONS as u128);
    
    // All operations should be very fast
    assert!(containment_duration.as_nanos() / (ITERATIONS as u128) < 100);
    assert!(clamping_duration.as_nanos() / (ITERATIONS as u128) < 100);
    assert!(range_duration.as_nanos() / (ITERATIONS as u128) < 100);
}

/// Benchmark caps modifications performance
#[tokio::test]
async fn benchmark_caps_modifications() {
    const ITERATIONS: usize = 10000;
    
    let mut caps = Caps::new(0.0, 1000.0);
    
    // Benchmark expansion operations
    let start = Instant::now();
    for _i in 0..ITERATIONS {
        caps.expand(1.0);
    }
    let expansion_duration = start.elapsed();
    
    // Reset caps
    caps = Caps::new(0.0, 1000.0);
    
    // Benchmark shrinking operations
    let start = Instant::now();
    for _i in 0..ITERATIONS {
        caps.shrink(0.1);
    }
    let shrinking_duration = start.elapsed();
    
    // Reset caps
    caps = Caps::new(0.0, 1000.0);
    
    // Benchmark set operations
    let start = Instant::now();
    for i in 0..ITERATIONS {
        caps.set(i as f64, (i + 100) as f64);
    }
    let set_duration = start.elapsed();
    
    println!("Caps modifications benchmark:");
    println!("  Expansion: {:?} ({}ns per operation)", expansion_duration, expansion_duration.as_nanos() / ITERATIONS as u128);
    println!("  Shrinking: {:?} ({}ns per operation)", shrinking_duration, shrinking_duration.as_nanos() / ITERATIONS as u128);
    println!("  Set: {:?} ({}ns per operation)", set_duration, set_duration.as_nanos() / ITERATIONS as u128);
    
    // All operations should be reasonably fast
    assert!(expansion_duration.as_nanos() / (ITERATIONS as u128) < 1000);
    assert!(shrinking_duration.as_nanos() / (ITERATIONS as u128) < 1000);
    assert!(set_duration.as_nanos() / (ITERATIONS as u128) < 1000);
}

/// Benchmark aggregation performance with varying subsystem counts
#[tokio::test]
async fn benchmark_aggregation_subsystem_scaling() {
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    for subsystem_count in [1, 10, 50, 100, 500] {
        let plugin_registry = Arc::new(PluginRegistryImpl::new());
        
        // Add subsystems
        for i in 0..subsystem_count {
            let subsystem = PerformanceTestSubsystem::new(
                format!("system_{}", i),
                i as i64,
                5, // 5 contributions per subsystem
            );
            plugin_registry.register(Arc::new(subsystem)).unwrap();
        }
        
        let combiner_registry = Arc::new(CombinerRegistryImpl::new());
        let aggregator = AggregatorImpl::new(
            plugin_registry,
            combiner_registry,
            caps_provider.clone(),
            cache.clone(),
        );
        
        // Warm up
        let _ = aggregator.resolve(&actor).await.unwrap();
        
        // Benchmark
        let start = Instant::now();
        const ITERATIONS: usize = 100;
        for _ in 0..ITERATIONS {
            let _snapshot = aggregator.resolve(&actor).await.unwrap();
        }
        let duration = start.elapsed();
        
        let per_resolution = duration.as_nanos() / ITERATIONS as u128;
        let per_subsystem = per_resolution / subsystem_count as u128;
        
        println!("Aggregation benchmark ({} subsystems):", subsystem_count);
        println!("  Total time: {:?}", duration);
        println!("  Per resolution: {}ns", per_resolution);
        println!("  Per subsystem: {}ns", per_subsystem);
        println!("  Resolutions per second: {}", ITERATIONS as f64 / duration.as_secs_f64());
    }
}

/// Benchmark cache performance
#[tokio::test]
async fn benchmark_cache_performance() {
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
    // Benchmark cache set operations
    let start = Instant::now();
    const ITERATIONS: usize = 10000;
    for i in 0..ITERATIONS {
        let key = format!("key_{}", i);
        let value = serde_json::Value::String(format!("value_{}", i));
        let _ = cache.set(key, value, Some(3600));
    }
    let set_duration = start.elapsed();
    
    // Benchmark cache get operations
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let key = format!("key_{}", i);
        let _ = cache.get(&key);
    }
    let get_duration = start.elapsed();
    
    // Benchmark cache delete operations
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let key = format!("key_{}", i);
        let _ = cache.delete(&key);
    }
    let delete_duration = start.elapsed();
    
    println!("Cache performance benchmark:");
    println!("  Set: {:?} ({}ns per operation)", set_duration, set_duration.as_nanos() / ITERATIONS as u128);
    println!("  Get: {:?} ({}ns per operation)", get_duration, get_duration.as_nanos() / ITERATIONS as u128);
    println!("  Delete: {:?} ({}ns per operation)", delete_duration, delete_duration.as_nanos() / ITERATIONS as u128);
    
    // Cache operations should be reasonably fast
    assert!(set_duration.as_nanos() / (ITERATIONS as u128) < 2000); // More realistic threshold
    assert!(get_duration.as_nanos() / (ITERATIONS as u128) < 1000);
    assert!(delete_duration.as_nanos() / (ITERATIONS as u128) < 1000);
}

/// Benchmark batch processing performance
#[tokio::test]
async fn benchmark_batch_processing() {
    let plugin_registry = Arc::new(PluginRegistryImpl::new());
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
        let combiner_registry = Arc::new(CombinerRegistryImpl::new());
        let aggregator = AggregatorImpl::new(
            plugin_registry,
            combiner_registry,
            caps_provider,
            cache,
        );
    
    for batch_size in [1, 10, 50, 100, 500] {
        // Create batch of actors
        let actors: Vec<Actor> = (0..batch_size)
            .map(|i| Actor::new(format!("Actor_{}", i), "Human".to_string()))
            .collect();
        
        // Warm up
        let _ = aggregator.resolve_batch(&actors).await.unwrap();
        
        // Benchmark
        let start = Instant::now();
        const ITERATIONS: usize = 10;
        for _ in 0..ITERATIONS {
            let _snapshots = aggregator.resolve_batch(&actors).await.unwrap();
        }
        let duration = start.elapsed();
        
        let per_batch = duration.as_nanos() / ITERATIONS as u128;
        let per_actor = per_batch / batch_size as u128;
        
        println!("Batch processing benchmark ({} actors):", batch_size);
        println!("  Total time: {:?}", duration);
        println!("  Per batch: {}ns", per_batch);
        println!("  Per actor: {}ns", per_actor);
        println!("  Actors per second: {}", (batch_size * ITERATIONS) as f64 / duration.as_secs_f64());
    }
}

/// Benchmark memory usage with large datasets
#[tokio::test]
async fn benchmark_memory_usage() {
    const LARGE_COUNT: usize = 10000;
    
    // Test memory usage with many actors
    let start = Instant::now();
    let actors: Vec<Actor> = (0..LARGE_COUNT)
        .map(|i| {
            let mut actor = Actor::new(format!("Actor_{}", i), "Human".to_string());
            
            // Add some data to each actor
            for j in 0..10 {
                actor.add_subsystem(SubsystemStruct::new(format!("system_{}", j), j as i64));
                actor.add_buff(format!("buff_{}", j));
            }
            
            actor
        })
        .collect();
    let creation_duration = start.elapsed();
    
    println!("Memory usage benchmark:");
    println!("  Created {} actors with data in {:?}", LARGE_COUNT, creation_duration);
    println!("  Per actor: {}ns", creation_duration.as_nanos() / LARGE_COUNT as u128);
    
    // Test that we can still access the data efficiently
    let start = Instant::now();
    for actor in &actors[0..1000] { // Test first 1000
        let _ = actor.get_subsystem_count();
        let _ = actor.get_active_buffs().len();
    }
    let access_duration = start.elapsed();
    
    println!("  Accessed 1000 actors in {:?}", access_duration);
    println!("  Per access: {}ns", access_duration.as_nanos() / 1000);
    
    // Should be reasonably fast
    assert!(creation_duration.as_nanos() / (LARGE_COUNT as u128) < 10000);
    assert!(access_duration.as_nanos() / 1000 < 1000);
}

/// Benchmark concurrent operations
#[tokio::test]
async fn benchmark_concurrent_operations() {
    let plugin_registry = Arc::new(PluginRegistryImpl::new());
    let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
    let combiner_registry = Arc::new(CombinerRegistryImpl::new());
    let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
    let cache = Arc::new(InMemoryCache::new(1000, 3600));
    
    let aggregator = Arc::new(AggregatorImpl::new(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    ));
    
    const CONCURRENT_TASKS: usize = 100;
    const ITERATIONS_PER_TASK: usize = 10;
    
    let start = Instant::now();
    
    let handles: Vec<_> = (0..CONCURRENT_TASKS)
        .map(|i| {
            let aggregator = aggregator.clone();
            tokio::spawn(async move {
                for j in 0..ITERATIONS_PER_TASK {
                    let actor = Actor::new(format!("Actor_{}_{}", i, j), "Human".to_string());
                    let _snapshot = aggregator.resolve(&actor).await.unwrap();
                }
            })
        })
        .collect();
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    let duration = start.elapsed();
    let total_operations = CONCURRENT_TASKS * ITERATIONS_PER_TASK;
    let operations_per_second = total_operations as f64 / duration.as_secs_f64();
    
    println!("Concurrent operations benchmark:");
    println!("  Total time: {:?}", duration);
    println!("  Total operations: {}", total_operations);
    println!("  Operations per second: {:.2}", operations_per_second);
    println!("  Concurrent tasks: {}", CONCURRENT_TASKS);
    
    // Should handle concurrency well
    assert!(operations_per_second > 100.0, "Concurrent performance too low: {:.2} ops/sec", operations_per_second);
}
