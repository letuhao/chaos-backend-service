//! Performance benchmarks for the Actor Core system.
//!
//! This module provides comprehensive benchmarking tools for measuring
//! performance of cache operations, aggregation, and memory usage.

use std::time::{Duration, Instant};
use std::collections::HashMap;
use rand::Rng;
use crate::ActorCoreResult;

/// Configuration for benchmarks.
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of keys to test
    pub key_count: usize,
    /// Size of values in bytes
    pub value_size: usize,
    /// Duration of the benchmark
    pub duration: Duration,
    /// Number of concurrent operations
    pub concurrency: usize,
    /// L1 cache maximum size
    pub l1_max_size: usize,
    /// L2 cache maximum size
    pub l2_max_size: usize,
    /// L3 cache maximum size
    pub l3_max_size: usize,
    /// Target latency
    pub target_latency: Duration,
    /// Target throughput (operations per second)
    pub target_throughput: u64,
    /// Target memory usage in bytes
    pub target_memory_usage: u64,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            key_count: 10000,
            value_size: 1024,
            duration: Duration::from_secs(30),
            concurrency: 4,
            l1_max_size: 1000,
            l2_max_size: 10000,
            l3_max_size: 100000,
            target_latency: Duration::from_millis(1),
            target_throughput: 10000,
            target_memory_usage: 100 * 1024 * 1024, // 100MB
        }
    }
}

/// Results of a benchmark run.
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    /// Performance metrics
    pub latency: Duration,
    pub throughput: f64,
    pub memory_usage: u64,
    pub cpu_usage: f64,
    
    /// Cache metrics
    pub hit_rate: f64,
    pub miss_rate: f64,
    pub eviction_rate: f64,
    
    /// System metrics
    pub thread_count: usize,
    pub gc_pause_time: Duration,
    
    /// Test configuration
    pub config: BenchmarkConfig,
    pub duration: Duration,
    pub operations_performed: u64,
    pub errors_encountered: u64,
    
    /// Performance targets
    pub latency_target_met: bool,
    pub throughput_target_met: bool,
    pub memory_target_met: bool,
}

/// Benchmark runner for actor core operations.
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
    /// Random number generator
    rng: rand::rngs::ThreadRng,
}

impl BenchmarkRunner {
    /// Create a new benchmark runner.
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            config,
            rng: rand::thread_rng(),
        }
    }

    /// Run cache performance benchmark.
    pub async fn run_cache_benchmark(&mut self) -> ActorCoreResult<BenchmarkResults> {
        let start_time = Instant::now();
        let mut operations = 0u64;
        let errors = 0u64;
        let mut total_latency = Duration::ZERO;
        let mut cache_hits = 0u64;
        let mut cache_misses = 0u64;

        // Generate test data
        let test_data = self.generate_test_data();
        let mut cache = HashMap::new();

        // Run benchmark
        while start_time.elapsed() < self.config.duration {
            let operation_start = Instant::now();
            
            match self.rng.gen_range(0..4) {
                0 => {
                    // Get operation
                    if let Some(key) = self.get_random_key() {
                        if cache.get(&key).is_some() {
                            cache_hits += 1;
                        } else {
                            cache_misses += 1;
                        }
                    }
                }
                1 => {
                    // Set operation
                    if let Some((key, value)) = self.get_random_key_value(&test_data) {
                        cache.insert(key, value);
                    }
                }
                2 => {
                    // Delete operation
                    if let Some(key) = self.get_random_key() {
                        cache.remove(&key);
                    }
                }
                3 => {
                    // Update operation
                    if let Some((key, value)) = self.get_random_key_value(&test_data) {
                        cache.insert(key, value);
                    }
                }
                _ => {}
            }

            operations += 1;
            total_latency += operation_start.elapsed();
        }

        let duration = start_time.elapsed();
        let avg_latency = if operations > 0 {
            Duration::from_nanos(total_latency.as_nanos() as u64 / operations)
        } else {
            Duration::ZERO
        };

        let throughput = operations as f64 / duration.as_secs_f64();
        let hit_rate = if cache_hits + cache_misses > 0 {
            cache_hits as f64 / (cache_hits + cache_misses) as f64
        } else {
            0.0
        };

        Ok(BenchmarkResults {
            latency: avg_latency,
            throughput,
            memory_usage: self.estimate_memory_usage(&cache),
            cpu_usage: 0.0, // Would need system monitoring
            hit_rate,
            miss_rate: 1.0 - hit_rate,
            eviction_rate: 0.0, // Would need cache eviction tracking
            thread_count: self.config.concurrency,
            gc_pause_time: Duration::ZERO, // Not applicable in Rust
            config: self.config.clone(),
            duration,
            operations_performed: operations,
            errors_encountered: errors,
            latency_target_met: avg_latency <= self.config.target_latency,
            throughput_target_met: throughput >= self.config.target_throughput as f64,
            memory_target_met: self.estimate_memory_usage(&cache) <= self.config.target_memory_usage,
        })
    }

    /// Run aggregation performance benchmark.
    pub async fn run_aggregation_benchmark(&mut self) -> ActorCoreResult<BenchmarkResults> {
        let start_time = Instant::now();
        let mut operations = 0u64;
        let errors = 0u64;
        let mut total_latency = Duration::ZERO;

        // Generate test contributions
        let contributions = self.generate_test_contributions();

        // Run benchmark
        while start_time.elapsed() < self.config.duration {
            let operation_start = Instant::now();
            
            // Simulate aggregation operation
            let _result = self.simulate_aggregation(&contributions);
            
            operations += 1;
            total_latency += operation_start.elapsed();
        }

        let duration = start_time.elapsed();
        let avg_latency = if operations > 0 {
            Duration::from_nanos(total_latency.as_nanos() as u64 / operations)
        } else {
            Duration::ZERO
        };

        let throughput = operations as f64 / duration.as_secs_f64();

        Ok(BenchmarkResults {
            latency: avg_latency,
            throughput,
            memory_usage: self.estimate_aggregation_memory_usage(),
            cpu_usage: 0.0,
            hit_rate: 0.0,
            miss_rate: 0.0,
            eviction_rate: 0.0,
            thread_count: self.config.concurrency,
            gc_pause_time: Duration::ZERO,
            config: self.config.clone(),
            duration,
            operations_performed: operations,
            errors_encountered: errors,
            latency_target_met: avg_latency <= self.config.target_latency,
            throughput_target_met: throughput >= self.config.target_throughput as f64,
            memory_target_met: self.estimate_aggregation_memory_usage() <= self.config.target_memory_usage,
        })
    }

    /// Run memory pool performance benchmark.
    pub async fn run_memory_pool_benchmark(&mut self) -> ActorCoreResult<BenchmarkResults> {
        let start_time = Instant::now();
        let mut operations = 0u64;
        let errors = 0u64;
        let mut total_latency = Duration::ZERO;

        // Run benchmark
        while start_time.elapsed() < self.config.duration {
            let operation_start = Instant::now();
            
            // Simulate memory pool operations
            self.simulate_memory_pool_operations();
            
            operations += 1;
            total_latency += operation_start.elapsed();
        }

        let duration = start_time.elapsed();
        let avg_latency = if operations > 0 {
            Duration::from_nanos(total_latency.as_nanos() as u64 / operations)
        } else {
            Duration::ZERO
        };

        let throughput = operations as f64 / duration.as_secs_f64();

        Ok(BenchmarkResults {
            latency: avg_latency,
            throughput,
            memory_usage: self.estimate_memory_pool_usage(),
            cpu_usage: 0.0,
            hit_rate: 0.0,
            miss_rate: 0.0,
            eviction_rate: 0.0,
            thread_count: self.config.concurrency,
            gc_pause_time: Duration::ZERO,
            config: self.config.clone(),
            duration,
            operations_performed: operations,
            errors_encountered: errors,
            latency_target_met: avg_latency <= self.config.target_latency,
            throughput_target_met: throughput >= self.config.target_throughput as f64,
            memory_target_met: self.estimate_memory_pool_usage() <= self.config.target_memory_usage,
        })
    }

    /// Run comprehensive benchmark suite.
    pub async fn run_comprehensive_benchmark(&mut self) -> ActorCoreResult<ComprehensiveBenchmarkResults> {
        let cache_results = self.run_cache_benchmark().await?;
        let aggregation_results = self.run_aggregation_benchmark().await?;
        let memory_pool_results = self.run_memory_pool_benchmark().await?;

        Ok(ComprehensiveBenchmarkResults {
            cache: cache_results.clone(),
            aggregation: aggregation_results.clone(),
            memory_pool: memory_pool_results.clone(),
            overall_score: self.calculate_overall_score(&cache_results, &aggregation_results, &memory_pool_results),
        })
    }

    // Helper methods
    fn generate_test_data(&mut self) -> Vec<(String, Vec<u8>)> {
        let mut data = Vec::new();
        for i in 0..self.config.key_count {
            let key = format!("key_{}", i);
            let value = vec![0u8; self.config.value_size];
            data.push((key, value));
        }
        data
    }

    fn generate_test_contributions(&mut self) -> Vec<crate::types::Contribution> {
        let mut contributions = Vec::new();
        for i in 0..1000 {
            contributions.push(crate::types::Contribution::new(
                format!("dimension_{}", i % 10),
                crate::enums::Bucket::Flat,
                self.rng.gen_range(0.0..100.0),
                format!("system_{}", i % 5),
            ));
        }
        contributions
    }

    fn get_random_key(&mut self) -> Option<String> {
        if self.config.key_count > 0 {
            Some(format!("key_{}", self.rng.gen_range(0..self.config.key_count)))
        } else {
            None
        }
    }

    fn get_random_key_value(&mut self, test_data: &[(String, Vec<u8>)]) -> Option<(String, Vec<u8>)> {
        test_data.get(self.rng.gen_range(0..test_data.len())).cloned()
    }

    fn simulate_aggregation(&self, contributions: &[crate::types::Contribution]) -> f64 {
        // Simulate aggregation logic
        contributions.iter().map(|c| c.value).sum()
    }

    fn simulate_memory_pool_operations(&mut self) {
        // Simulate memory pool operations
        let _dummy_data = vec![0u8; self.config.value_size];
    }

    fn estimate_memory_usage(&self, cache: &HashMap<String, Vec<u8>>) -> u64 {
        let mut total = 0;
        for (key, value) in cache {
            total += key.len() + value.len();
        }
        total as u64
    }

    fn estimate_aggregation_memory_usage(&self) -> u64 {
        // Estimate memory usage for aggregation operations
        self.config.key_count as u64 * self.config.value_size as u64
    }

    fn estimate_memory_pool_usage(&self) -> u64 {
        // Estimate memory usage for memory pool
        self.config.key_count as u64 * self.config.value_size as u64
    }

    fn calculate_overall_score(&self, cache: &BenchmarkResults, aggregation: &BenchmarkResults, memory_pool: &BenchmarkResults) -> f64 {
        let mut score = 0.0;
        
        // Latency score (lower is better)
        if cache.latency_target_met { score += 25.0; }
        if aggregation.latency_target_met { score += 25.0; }
        if memory_pool.latency_target_met { score += 25.0; }
        
        // Throughput score (higher is better)
        if cache.throughput_target_met { score += 25.0; }
        if aggregation.throughput_target_met { score += 25.0; }
        if memory_pool.throughput_target_met { score += 25.0; }
        
        // Memory score (lower is better)
        if cache.memory_target_met { score += 25.0; }
        if aggregation.memory_target_met { score += 25.0; }
        if memory_pool.memory_target_met { score += 25.0; }
        
        score / 3.0 // Average across the three benchmarks
    }
}

/// Results of a comprehensive benchmark suite.
#[derive(Debug, Clone)]
pub struct ComprehensiveBenchmarkResults {
    /// Cache benchmark results
    pub cache: BenchmarkResults,
    /// Aggregation benchmark results
    pub aggregation: BenchmarkResults,
    /// Memory pool benchmark results
    pub memory_pool: BenchmarkResults,
    /// Overall performance score (0-100)
    pub overall_score: f64,
}

/// Benchmark report generator.
pub struct BenchmarkReporter;

impl BenchmarkReporter {
    /// Generate a detailed benchmark report.
    pub fn generate_report(results: &ComprehensiveBenchmarkResults) -> String {
        format!(
            r#"
# Actor Core Performance Benchmark Report

## Overall Performance Score: {:.1}/100

## Cache Performance
- **Latency**: {:?} (Target: {:?}) - {}
- **Throughput**: {:.2} ops/sec (Target: {}) - {}
- **Memory Usage**: {} bytes (Target: {}) - {}
- **Hit Rate**: {:.2}%

## Aggregation Performance
- **Latency**: {:?} (Target: {:?}) - {}
- **Throughput**: {:.2} ops/sec (Target: {}) - {}
- **Memory Usage**: {} bytes (Target: {}) - {}

## Memory Pool Performance
- **Latency**: {:?} (Target: {:?}) - {}
- **Throughput**: {:.2} ops/sec (Target: {}) - {}
- **Memory Usage**: {} bytes (Target: {}) - {}

## Recommendations
{}

## Test Configuration
- **Duration**: {:?}
- **Concurrency**: {}
- **Key Count**: {}
- **Value Size**: {} bytes
"#,
            results.overall_score,
            results.cache.latency, results.cache.config.target_latency,
            if results.cache.latency_target_met { "✅ PASS" } else { "❌ FAIL" },
            results.cache.throughput, results.cache.config.target_throughput,
            if results.cache.throughput_target_met { "✅ PASS" } else { "❌ FAIL" },
            results.cache.memory_usage, results.cache.config.target_memory_usage,
            if results.cache.memory_target_met { "✅ PASS" } else { "❌ FAIL" },
            results.cache.hit_rate * 100.0,
            results.aggregation.latency, results.aggregation.config.target_latency,
            if results.aggregation.latency_target_met { "✅ PASS" } else { "❌ FAIL" },
            results.aggregation.throughput, results.aggregation.config.target_throughput,
            if results.aggregation.throughput_target_met { "✅ PASS" } else { "❌ FAIL" },
            results.aggregation.memory_usage, results.aggregation.config.target_memory_usage,
            if results.aggregation.memory_target_met { "✅ PASS" } else { "❌ FAIL" },
            results.memory_pool.latency, results.memory_pool.config.target_latency,
            if results.memory_pool.latency_target_met { "✅ PASS" } else { "❌ FAIL" },
            results.memory_pool.throughput, results.memory_pool.config.target_throughput,
            if results.memory_pool.throughput_target_met { "✅ PASS" } else { "❌ FAIL" },
            results.memory_pool.memory_usage, results.memory_pool.config.target_memory_usage,
            if results.memory_pool.memory_target_met { "✅ PASS" } else { "❌ FAIL" },
            Self::generate_recommendations(results),
            results.cache.config.duration,
            results.cache.config.concurrency,
            results.cache.config.key_count,
            results.cache.config.value_size,
        )
    }

    fn generate_recommendations(results: &ComprehensiveBenchmarkResults) -> String {
        let mut recommendations = Vec::new();

        if results.overall_score < 50.0 {
            recommendations.push("⚠️  Overall performance is below target. Consider optimizing core algorithms.");
        }

        if !results.cache.latency_target_met {
            recommendations.push("🔧 Cache latency is too high. Consider implementing L1 cache or reducing cache size.");
        }

        if !results.cache.throughput_target_met {
            recommendations.push("⚡ Cache throughput is too low. Consider using SIMD optimizations or increasing concurrency.");
        }

        if !results.aggregation.latency_target_met {
            recommendations.push("📊 Aggregation latency is too high. Consider parallel processing or algorithm optimization.");
        }

        if !results.memory_pool.memory_target_met {
            recommendations.push("💾 Memory usage is too high. Consider implementing object pooling or reducing buffer sizes.");
        }

        if recommendations.is_empty() {
            "✅ All performance targets met. System is performing optimally.".to_string()
        } else {
            recommendations.join("\n")
        }
    }
}
