//! # Element Aggregation Performance Benchmarks
//! 
//! This module contains performance benchmarks for the Element Core aggregation system.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio;
use element_core::{
    ElementContributor, ElementContribution, ElementContributorRegistry,
    UnifiedElementRegistry, ElementAggregator, AggregationStrategy,
    CacheConfig, EvictionPolicy
};
use actor_core::Actor;

/// High-performance mock contributor for benchmarking
struct BenchmarkContributor {
    system_id: String,
    priority: i64,
    stat_count: usize,
}

impl BenchmarkContributor {
    fn new(system_id: &str, priority: i64, stat_count: usize) -> Self {
        Self {
            system_id: system_id.to_string(),
            priority,
            stat_count,
        }
    }
}

#[async_trait::async_trait]
impl ElementContributor for BenchmarkContributor {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute_element_stats(
        &self, 
        _actor: &Actor, 
        element_type: &str
    ) -> element_core::ElementCoreResult<ElementContribution> {
        let mut stats = HashMap::with_capacity(self.stat_count);
        
        // Generate multiple stats for benchmarking
        for i in 0..self.stat_count {
            stats.insert(
                format!("{}_{}_stat_{}", element_type, self.system_id, i),
                (i as f64) * 10.0 + self.priority as f64,
            );
        }
        
        Ok(ElementContribution::new(
            self.system_id.clone(),
            element_type.to_string(),
            stats,
            self.priority,
        ))
    }
    
    async fn handle_element_event(&self, _event: &element_core::ElementEvent) -> element_core::ElementCoreResult<()> {
        Ok(())
    }
}

/// Benchmark single aggregation operation
pub async fn benchmark_single_aggregation() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Single Aggregation Benchmark ===");
    
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Register multiple contributors with different stat counts
    let contributors = vec![
        Arc::new(BenchmarkContributor::new("race_core", 1000, 10)),
        Arc::new(BenchmarkContributor::new("item_core", 800, 15)),
        Arc::new(BenchmarkContributor::new("skill_core", 600, 20)),
        Arc::new(BenchmarkContributor::new("talent_core", 400, 5)),
        Arc::new(BenchmarkContributor::new("destiny_core", 200, 8)),
    ];
    
    for contributor in contributors {
        registry.register_contributor(contributor).await?;
    }
    
    let aggregator = ElementAggregator::new(registry.clone());
    let actor = Actor::simple("benchmark_actor", "test_race", 50);
    
    // Warm up
    for _ in 0..10 {
        let _ = aggregator.aggregate_contributions(&actor, "fire").await?;
    }
    
    // Benchmark
    let iterations = 1000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = aggregator.aggregate_contributions(&actor, "fire").await?;
    }
    
    let duration = start.elapsed();
    let avg_time = duration.as_nanos() as f64 / iterations as f64;
    
    println!("Single aggregation benchmark:");
    println!("  Iterations: {}", iterations);
    println!("  Total time: {:?}", duration);
    println!("  Average time per operation: {:.2} ns", avg_time);
    println!("  Operations per second: {:.0}", 1_000_000_000.0 / avg_time);
    
    Ok(())
}

/// Benchmark batch aggregation operations
pub async fn benchmark_batch_aggregation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Batch Aggregation Benchmark ===");
    
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Register contributors
    let contributors = vec![
        Arc::new(BenchmarkContributor::new("race_core", 1000, 10)),
        Arc::new(BenchmarkContributor::new("item_core", 800, 15)),
        Arc::new(BenchmarkContributor::new("skill_core", 600, 20)),
    ];
    
    for contributor in contributors {
        registry.register_contributor(contributor).await?;
    }
    
    let aggregator = ElementAggregator::new(registry.clone());
    
    // Create multiple actors
    let actors: Vec<Actor> = (0..100)
        .map(|i| Actor::simple(&format!("actor_{}", i), "test_race", i as i64))
        .collect();
    
    let elements = vec!["fire", "water", "earth", "air", "lightning"];
    
    // Warm up
    for actor in &actors[0..10] {
        for element in &elements {
            let _ = aggregator.aggregate_contributions(actor, element).await?;
        }
    }
    
    // Benchmark
    let start = Instant::now();
    
    for actor in &actors {
        for element in &elements {
            let _ = aggregator.aggregate_contributions(actor, element).await?;
        }
    }
    
    let duration = start.elapsed();
    let total_operations = actors.len() * elements.len();
    let avg_time = duration.as_nanos() as f64 / total_operations as f64;
    
    println!("Batch aggregation benchmark:");
    println!("  Actors: {}", actors.len());
    println!("  Elements per actor: {}", elements.len());
    println!("  Total operations: {}", total_operations);
    println!("  Total time: {:?}", duration);
    println!("  Average time per operation: {:.2} ns", avg_time);
    println!("  Operations per second: {:.0}", 1_000_000_000.0 / avg_time);
    
    Ok(())
}

/// Benchmark cache performance
pub async fn benchmark_cache_performance() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Cache Performance Benchmark ===");
    
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Register contributors
    let contributor = Arc::new(BenchmarkContributor::new("race_core", 1000, 50));
    registry.register_contributor(contributor).await?;
    
    // Create aggregator with cache
    let cache_config = CacheConfig {
        enabled: true,
        size_limit: 1000,
        default_ttl_seconds: 3600,
        eviction_policy: EvictionPolicy::LRU,
    };
    
    let aggregator = ElementAggregator::with_cache_config(registry.clone(), cache_config);
    let actor = Actor::simple("cache_test_actor", "test_race", 25);
    
    // Benchmark cache miss (first access)
    let start = Instant::now();
    let _ = aggregator.aggregate_contributions(&actor, "fire").await?;
    let cache_miss_time = start.elapsed();
    
    // Benchmark cache hit (subsequent accesses)
    let iterations = 1000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = aggregator.aggregate_contributions(&actor, "fire").await?;
    }
    
    let cache_hit_duration = start.elapsed();
    let avg_cache_hit_time = cache_hit_duration.as_nanos() as f64 / iterations as f64;
    
    println!("Cache performance benchmark:");
    println!("  Cache miss time: {:?}", cache_miss_time);
    println!("  Cache hit iterations: {}", iterations);
    println!("  Total cache hit time: {:?}", cache_hit_duration);
    println!("  Average cache hit time: {:.2} ns", avg_cache_hit_time);
    println!("  Cache speedup: {:.2}x", cache_miss_time.as_nanos() as f64 / avg_cache_hit_time);
    
    // Check cache stats
    let cache_stats = aggregator.get_cache_stats();
    println!("  Cache hit rate: {:.2}%", cache_stats.get_hit_rate() * 100.0);
    
    Ok(())
}

/// Benchmark different aggregation strategies
pub async fn benchmark_aggregation_strategies() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Aggregation Strategies Benchmark ===");
    
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Register multiple contributors to create multiple values for aggregation
    for i in 0..10 {
        let contributor = Arc::new(BenchmarkContributor::new(&format!("system_{}", i), 1000 - i * 100, 5));
        registry.register_contributor(contributor).await?;
    }
    
    let aggregator = ElementAggregator::new(registry.clone());
    let actor = Actor::simple("strategy_test_actor", "test_race", 30);
    
    let strategies = vec![
        ("Sum", AggregationStrategy::Sum),
        ("Multiply", AggregationStrategy::Multiply),
        ("Max", AggregationStrategy::Max),
        ("Min", AggregationStrategy::Min),
        ("Average", AggregationStrategy::Average),
        ("First", AggregationStrategy::First),
        ("Last", AggregationStrategy::Last),
    ];
    
    let iterations = 500;
    
    for (name, strategy) in strategies {
        // Set strategy for all stats
        aggregator.set_strategy("fire_race_core_stat_0", strategy);
        
        // Warm up
        for _ in 0..10 {
            let _ = aggregator.aggregate_contributions(&actor, "fire").await?;
        }
        
        // Benchmark
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _ = aggregator.aggregate_contributions(&actor, "fire").await?;
        }
        
        let duration = start.elapsed();
        let avg_time = duration.as_nanos() as f64 / iterations as f64;
        
        println!("  {} strategy: {:.2} ns per operation", name, avg_time);
    }
    
    Ok(())
}

/// Benchmark memory usage
pub async fn benchmark_memory_usage() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Memory Usage Benchmark ===");
    
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Register contributors with varying stat counts
    let stat_counts = vec![1, 10, 50, 100, 500];
    
    for (i, &stat_count) in stat_counts.iter().enumerate() {
        let contributor = Arc::new(BenchmarkContributor::new(&format!("system_{}", i), 1000, stat_count));
        registry.register_contributor(contributor).await?;
        
        let aggregator = ElementAggregator::new(registry.clone());
        let actor = Actor::simple("memory_test_actor", "test_race", 40);
        
        // Perform aggregation
        let start = Instant::now();
        let result = aggregator.aggregate_contributions(&actor, "fire").await?;
        let duration = start.elapsed();
        
        println!("  {} stats: {} result entries, {:?} processing time", 
                 stat_count, result.len(), duration);
    }
    
    Ok(())
}

/// Run all benchmarks
pub async fn run_all_benchmarks() -> Result<(), Box<dyn std::error::Error>> {
    println!("Element Core Performance Benchmarks");
    println!("===================================");
    
    benchmark_single_aggregation().await?;
    benchmark_batch_aggregation().await?;
    benchmark_cache_performance().await?;
    benchmark_aggregation_strategies().await?;
    benchmark_memory_usage().await?;
    
    println!("\nAll benchmarks completed successfully!");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_all_benchmarks().await
}
