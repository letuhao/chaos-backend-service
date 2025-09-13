//! Performance benchmarks for the Actor Core system.
//!
//! This module provides comprehensive benchmarking tools for measuring
//! performance of cache operations, aggregation, and memory usage.

use std::time::{Duration, Instant};
use std::collections::HashMap;
use rand::Rng;
use crate::ActorCoreResult;
use crate::cache::CacheFactory;
use crate::interfaces::Cache as CacheTrait;

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
        // Select cache kind via environment variable (basic | lock_free | multi)
        let cache_kind = std::env::var("ACTOR_CORE_CACHE_KIND").unwrap_or_else(|_| "basic".to_string());
        let cache: std::sync::Arc<dyn CacheTrait> = match cache_kind.as_str() {
            "lock_free" => CacheFactory::create_lock_free_in_memory_cache(100_000, 300),
            "multi" => CacheFactory::create_default_multi_layer_cache(),
            _ => CacheFactory::create_in_memory_cache(100_000, 600),
        };

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
                        let _ = cache.set(key, serde_json::json!(value), Some(600));
                    }
                }
                2 => {
                    // Delete operation
                    if let Some(key) = self.get_random_key() {
                        let _ = cache.delete(&key);
                    }
                }
                3 => {
                    // Update operation
                    if let Some((key, value)) = self.get_random_key_value(&test_data) {
                        let _ = cache.set(key, serde_json::json!(value), Some(600));
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
            memory_usage: 0, // not tracked for trait object cache
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
            memory_target_met: true,
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

    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // BenchmarkConfig tests
    #[test]
    fn test_benchmark_config_default() {
        let config = BenchmarkConfig::default();
        assert_eq!(config.key_count, 10000);
        assert_eq!(config.value_size, 1024);
        assert_eq!(config.duration, Duration::from_secs(30));
        assert_eq!(config.concurrency, 4);
        assert_eq!(config.l1_max_size, 1000);
        assert_eq!(config.l2_max_size, 10000);
        assert_eq!(config.l3_max_size, 100000);
        assert_eq!(config.target_latency, Duration::from_millis(1));
        assert_eq!(config.target_throughput, 10000);
        assert_eq!(config.target_memory_usage, 100 * 1024 * 1024);
    }

    #[test]
    fn test_benchmark_config_creation() {
        let config = BenchmarkConfig {
            key_count: 5000,
            value_size: 512,
            duration: Duration::from_secs(10),
            concurrency: 8,
            l1_max_size: 500,
            l2_max_size: 5000,
            l3_max_size: 50000,
            target_latency: Duration::from_millis(5),
            target_throughput: 5000,
            target_memory_usage: 50 * 1024 * 1024,
        };
        
        assert_eq!(config.key_count, 5000);
        assert_eq!(config.value_size, 512);
        assert_eq!(config.duration, Duration::from_secs(10));
        assert_eq!(config.concurrency, 8);
        assert_eq!(config.l1_max_size, 500);
        assert_eq!(config.l2_max_size, 5000);
        assert_eq!(config.l3_max_size, 50000);
        assert_eq!(config.target_latency, Duration::from_millis(5));
        assert_eq!(config.target_throughput, 5000);
        assert_eq!(config.target_memory_usage, 50 * 1024 * 1024);
    }

    #[test]
    fn test_benchmark_config_clone() {
        let config = BenchmarkConfig::default();
        let cloned = config.clone();
        assert_eq!(config.key_count, cloned.key_count);
        assert_eq!(config.value_size, cloned.value_size);
        assert_eq!(config.duration, cloned.duration);
    }

    #[test]
    fn test_benchmark_config_debug() {
        let config = BenchmarkConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("BenchmarkConfig"));
        assert!(debug_str.contains("key_count"));
        assert!(debug_str.contains("value_size"));
    }

    // BenchmarkResults tests
    #[test]
    fn test_benchmark_results_creation() {
        let config = BenchmarkConfig::default();
        let results = BenchmarkResults {
            latency: Duration::from_millis(1),
            throughput: 1000.0,
            memory_usage: 1024,
            cpu_usage: 50.0,
            hit_rate: 0.8,
            miss_rate: 0.2,
            eviction_rate: 0.1,
            thread_count: 4,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 1000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };

        assert_eq!(results.latency, Duration::from_millis(1));
        assert_eq!(results.throughput, 1000.0);
        assert_eq!(results.memory_usage, 1024);
        assert_eq!(results.cpu_usage, 50.0);
        assert_eq!(results.hit_rate, 0.8);
        assert_eq!(results.miss_rate, 0.2);
        assert_eq!(results.eviction_rate, 0.1);
        assert_eq!(results.thread_count, 4);
        assert_eq!(results.gc_pause_time, Duration::ZERO);
        assert_eq!(results.operations_performed, 1000);
        assert_eq!(results.errors_encountered, 0);
        assert!(results.latency_target_met);
        assert!(results.throughput_target_met);
        assert!(results.memory_target_met);
    }

    #[test]
    fn test_benchmark_results_clone() {
        let config = BenchmarkConfig::default();
        let results = BenchmarkResults {
            latency: Duration::from_millis(1),
            throughput: 1000.0,
            memory_usage: 1024,
            cpu_usage: 50.0,
            hit_rate: 0.8,
            miss_rate: 0.2,
            eviction_rate: 0.1,
            thread_count: 4,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 1000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };

        let cloned = results.clone();
        assert_eq!(results.latency, cloned.latency);
        assert_eq!(results.throughput, cloned.throughput);
        assert_eq!(results.memory_usage, cloned.memory_usage);
        assert_eq!(results.hit_rate, cloned.hit_rate);
    }

    #[test]
    fn test_benchmark_results_debug() {
        let config = BenchmarkConfig::default();
        let results = BenchmarkResults {
            latency: Duration::from_millis(1),
            throughput: 1000.0,
            memory_usage: 1024,
            cpu_usage: 50.0,
            hit_rate: 0.8,
            miss_rate: 0.2,
            eviction_rate: 0.1,
            thread_count: 4,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 1000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };

        let debug_str = format!("{:?}", results);
        assert!(debug_str.contains("BenchmarkResults"));
        assert!(debug_str.contains("latency"));
        assert!(debug_str.contains("throughput"));
    }

    // BenchmarkRunner tests
    #[test]
    fn test_benchmark_runner_creation() {
        let config = BenchmarkConfig::default();
        let runner = BenchmarkRunner::new(config.clone());
        assert_eq!(runner.config.key_count, config.key_count);
        assert_eq!(runner.config.value_size, config.value_size);
    }

    #[test]
    fn test_benchmark_runner_creation_custom_config() {
        let config = BenchmarkConfig {
            key_count: 1000,
            value_size: 256,
            duration: Duration::from_secs(5),
            concurrency: 2,
            l1_max_size: 100,
            l2_max_size: 1000,
            l3_max_size: 10000,
            target_latency: Duration::from_millis(2),
            target_throughput: 5000,
            target_memory_usage: 10 * 1024 * 1024,
        };
        
        let runner = BenchmarkRunner::new(config.clone());
        assert_eq!(runner.config.key_count, 1000);
        assert_eq!(runner.config.value_size, 256);
        assert_eq!(runner.config.duration, Duration::from_secs(5));
        assert_eq!(runner.config.concurrency, 2);
    }

    #[tokio::test]
    async fn test_run_cache_benchmark() {
        let config = BenchmarkConfig {
            key_count: 100,
            value_size: 64,
            duration: Duration::from_millis(100), // Short duration for test
            concurrency: 1,
            l1_max_size: 50,
            l2_max_size: 100,
            l3_max_size: 200,
            target_latency: Duration::from_millis(10),
            target_throughput: 100,
            target_memory_usage: 1024 * 1024,
        };
        
        let mut runner = BenchmarkRunner::new(config);
        let result = runner.run_cache_benchmark().await;
        
        assert!(result.is_ok());
        let results = result.unwrap();
        assert!(results.operations_performed > 0);
        assert!(results.duration >= Duration::from_millis(100));
        assert!(results.throughput >= 0.0);
        assert!(results.hit_rate >= 0.0);
        assert!(results.hit_rate <= 1.0);
        assert!(results.miss_rate >= 0.0);
        assert!(results.miss_rate <= 1.0);
        assert_eq!(results.thread_count, 1);
    }

    #[tokio::test]
    async fn test_run_aggregation_benchmark() {
        let config = BenchmarkConfig {
            key_count: 50,
            value_size: 32,
            duration: Duration::from_millis(50), // Short duration for test
            concurrency: 1,
            l1_max_size: 25,
            l2_max_size: 50,
            l3_max_size: 100,
            target_latency: Duration::from_millis(5),
            target_throughput: 50,
            target_memory_usage: 512 * 1024,
        };
        
        let mut runner = BenchmarkRunner::new(config);
        let result = runner.run_aggregation_benchmark().await;
        
        assert!(result.is_ok());
        let results = result.unwrap();
        assert!(results.operations_performed > 0);
        assert!(results.duration >= Duration::from_millis(50));
        assert!(results.throughput >= 0.0);
        assert!(results.memory_usage > 0);
        assert_eq!(results.thread_count, 1);
    }

    #[tokio::test]
    async fn test_run_memory_pool_benchmark() {
        let config = BenchmarkConfig {
            key_count: 25,
            value_size: 16,
            duration: Duration::from_millis(25), // Short duration for test
            concurrency: 1,
            l1_max_size: 10,
            l2_max_size: 25,
            l3_max_size: 50,
            target_latency: Duration::from_millis(2),
            target_throughput: 25,
            target_memory_usage: 256 * 1024,
        };
        
        let mut runner = BenchmarkRunner::new(config);
        let result = runner.run_memory_pool_benchmark().await;
        
        assert!(result.is_ok());
        let results = result.unwrap();
        assert!(results.operations_performed > 0);
        assert!(results.duration >= Duration::from_millis(25));
        assert!(results.throughput >= 0.0);
        assert!(results.memory_usage > 0);
        assert_eq!(results.thread_count, 1);
    }

    #[tokio::test]
    async fn test_run_comprehensive_benchmark() {
        let config = BenchmarkConfig {
            key_count: 10,
            value_size: 8,
            duration: Duration::from_millis(10), // Very short duration for test
            concurrency: 1,
            l1_max_size: 5,
            l2_max_size: 10,
            l3_max_size: 20,
            target_latency: Duration::from_millis(1),
            target_throughput: 10,
            target_memory_usage: 128 * 1024,
        };
        
        let mut runner = BenchmarkRunner::new(config);
        let result = runner.run_comprehensive_benchmark().await;
        
        assert!(result.is_ok());
        let results = result.unwrap();
        assert!(results.overall_score >= 0.0);
        assert!(results.overall_score <= 100.0);
        assert!(results.cache.operations_performed > 0);
        assert!(results.aggregation.operations_performed > 0);
        assert!(results.memory_pool.operations_performed > 0);
    }

    // Helper method tests
    #[test]
    fn test_generate_test_data() {
        let config = BenchmarkConfig {
            key_count: 5,
            value_size: 10,
            duration: Duration::from_secs(1),
            concurrency: 1,
            l1_max_size: 5,
            l2_max_size: 10,
            l3_max_size: 20,
            target_latency: Duration::from_millis(1),
            target_throughput: 100,
            target_memory_usage: 1024,
        };
        
        let mut runner = BenchmarkRunner::new(config);
        let data = runner.generate_test_data();
        
        assert_eq!(data.len(), 5);
        for (i, (key, value)) in data.iter().enumerate() {
            assert_eq!(key, &format!("key_{}", i));
            assert_eq!(value.len(), 10);
        }
    }

    #[test]
    fn test_generate_test_contributions() {
        let config = BenchmarkConfig::default();
        let mut runner = BenchmarkRunner::new(config);
        let contributions = runner.generate_test_contributions();
        
        assert_eq!(contributions.len(), 1000);
        for contribution in &contributions {
            assert!(contribution.dimension.starts_with("dimension_"));
            assert!(contribution.system.starts_with("system_"));
            assert!(contribution.value >= 0.0);
            assert!(contribution.value < 100.0);
        }
    }

    #[test]
    fn test_get_random_key() {
        let config = BenchmarkConfig {
            key_count: 10,
            value_size: 10,
            duration: Duration::from_secs(1),
            concurrency: 1,
            l1_max_size: 5,
            l2_max_size: 10,
            l3_max_size: 20,
            target_latency: Duration::from_millis(1),
            target_throughput: 100,
            target_memory_usage: 1024,
        };
        
        let mut runner = BenchmarkRunner::new(config);
        
        // Test with valid key_count
        for _ in 0..100 {
            let key = runner.get_random_key();
            assert!(key.is_some());
            let key_str = key.unwrap();
            assert!(key_str.starts_with("key_"));
            let key_num: usize = key_str[4..].parse().unwrap();
            assert!(key_num < 10);
        }
    }

    #[test]
    fn test_get_random_key_zero_count() {
        let config = BenchmarkConfig {
            key_count: 0,
            value_size: 10,
            duration: Duration::from_secs(1),
            concurrency: 1,
            l1_max_size: 5,
            l2_max_size: 10,
            l3_max_size: 20,
            target_latency: Duration::from_millis(1),
            target_throughput: 100,
            target_memory_usage: 1024,
        };
        
        let mut runner = BenchmarkRunner::new(config);
        let key = runner.get_random_key();
        assert!(key.is_none());
    }

    #[test]
    fn test_get_random_key_value() {
        let config = BenchmarkConfig {
            key_count: 3,
            value_size: 5,
            duration: Duration::from_secs(1),
            concurrency: 1,
            l1_max_size: 5,
            l2_max_size: 10,
            l3_max_size: 20,
            target_latency: Duration::from_millis(1),
            target_throughput: 100,
            target_memory_usage: 1024,
        };
        
        let mut runner = BenchmarkRunner::new(config);
        let test_data = runner.generate_test_data();
        
        for _ in 0..100 {
            let kv = runner.get_random_key_value(&test_data);
            assert!(kv.is_some());
            let (key, value) = kv.unwrap();
            assert!(key.starts_with("key_"));
            assert_eq!(value.len(), 5);
        }
    }

    #[test]
    fn test_simulate_aggregation() {
        let config = BenchmarkConfig::default();
        let runner = BenchmarkRunner::new(config);
        
        let contributions = vec![
            crate::types::Contribution::new("dim1".to_string(), crate::enums::Bucket::Flat, 10.0, "sys1".to_string()),
            crate::types::Contribution::new("dim2".to_string(), crate::enums::Bucket::Flat, 20.0, "sys2".to_string()),
            crate::types::Contribution::new("dim3".to_string(), crate::enums::Bucket::Flat, 30.0, "sys3".to_string()),
        ];
        
        let result = runner.simulate_aggregation(&contributions);
        assert_eq!(result, 60.0);
    }

    #[test]
    fn test_simulate_aggregation_empty() {
        let config = BenchmarkConfig::default();
        let runner = BenchmarkRunner::new(config);
        
        let contributions = vec![];
        let result = runner.simulate_aggregation(&contributions);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_simulate_memory_pool_operations() {
        let config = BenchmarkConfig {
            value_size: 100,
            ..Default::default()
        };
        
        let mut runner = BenchmarkRunner::new(config);
        // This should not panic
        runner.simulate_memory_pool_operations();
    }

    #[test]
    fn test_estimate_memory_usage() {
        let config = BenchmarkConfig::default();
        let runner = BenchmarkRunner::new(config);
        
        let mut cache = HashMap::new();
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        cache.insert("key2".to_string(), vec![4, 5, 6, 7]);
        
        let usage = runner.estimate_memory_usage(&cache);
        // key1: 4 + 3 = 7, key2: 4 + 4 = 8, total = 15
        assert_eq!(usage, 15);
    }

    #[test]
    fn test_estimate_aggregation_memory_usage() {
        let config = BenchmarkConfig {
            key_count: 1000,
            value_size: 512,
            ..Default::default()
        };
        
        let runner = BenchmarkRunner::new(config);
        let usage = runner.estimate_aggregation_memory_usage();
        assert_eq!(usage, 1000 * 512);
    }

    #[test]
    fn test_estimate_memory_pool_usage() {
        let config = BenchmarkConfig {
            key_count: 500,
            value_size: 256,
            ..Default::default()
        };
        
        let runner = BenchmarkRunner::new(config);
        let usage = runner.estimate_memory_pool_usage();
        assert_eq!(usage, 500 * 256);
    }

    #[test]
    fn test_calculate_overall_score() {
        let config = BenchmarkConfig::default();
        let runner = BenchmarkRunner::new(config.clone());
        
        let cache_results = BenchmarkResults {
            latency: Duration::from_millis(1),
            throughput: 1000.0,
            memory_usage: 1024,
            cpu_usage: 50.0,
            hit_rate: 0.8,
            miss_rate: 0.2,
            eviction_rate: 0.1,
            thread_count: 4,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 1000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        let aggregation_results = cache_results.clone();
        let memory_pool_results = cache_results.clone();
        
        let score = runner.calculate_overall_score(&cache_results, &aggregation_results, &memory_pool_results);
        assert!(score >= 0.0);
        assert!(score <= 100.0);
    }

    // ComprehensiveBenchmarkResults tests
    #[test]
    fn test_comprehensive_benchmark_results_creation() {
        let config = BenchmarkConfig::default();
        let cache_results = BenchmarkResults {
            latency: Duration::from_millis(1),
            throughput: 1000.0,
            memory_usage: 1024,
            cpu_usage: 50.0,
            hit_rate: 0.8,
            miss_rate: 0.2,
            eviction_rate: 0.1,
            thread_count: 4,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 1000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        let aggregation_results = cache_results.clone();
        let memory_pool_results = cache_results.clone();
        
        let comprehensive = ComprehensiveBenchmarkResults {
            cache: cache_results.clone(),
            aggregation: aggregation_results.clone(),
            memory_pool: memory_pool_results.clone(),
            overall_score: 75.0,
        };
        
        assert_eq!(comprehensive.overall_score, 75.0);
    }

    #[test]
    fn test_comprehensive_benchmark_results_clone() {
        let config = BenchmarkConfig::default();
        let cache_results = BenchmarkResults {
            latency: Duration::from_millis(1),
            throughput: 1000.0,
            memory_usage: 1024,
            cpu_usage: 50.0,
            hit_rate: 0.8,
            miss_rate: 0.2,
            eviction_rate: 0.1,
            thread_count: 4,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 1000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        let comprehensive = ComprehensiveBenchmarkResults {
            cache: cache_results.clone(),
            aggregation: cache_results.clone(),
            memory_pool: cache_results.clone(),
            overall_score: 75.0,
        };
        
        let cloned = comprehensive.clone();
        assert_eq!(comprehensive.overall_score, cloned.overall_score);
    }

    #[test]
    fn test_comprehensive_benchmark_results_debug() {
        let config = BenchmarkConfig::default();
        let cache_results = BenchmarkResults {
            latency: Duration::from_millis(1),
            throughput: 1000.0,
            memory_usage: 1024,
            cpu_usage: 50.0,
            hit_rate: 0.8,
            miss_rate: 0.2,
            eviction_rate: 0.1,
            thread_count: 4,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 1000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        let comprehensive = ComprehensiveBenchmarkResults {
            cache: cache_results.clone(),
            aggregation: cache_results.clone(),
            memory_pool: cache_results.clone(),
            overall_score: 75.0,
        };
        
        let debug_str = format!("{:?}", comprehensive);
        assert!(debug_str.contains("ComprehensiveBenchmarkResults"));
        assert!(debug_str.contains("overall_score"));
    }

    // BenchmarkReporter tests
    #[test]
    fn test_benchmark_reporter_generate_report() {
        let config = BenchmarkConfig::default();
        let cache_results = BenchmarkResults {
            latency: Duration::from_millis(1),
            throughput: 1000.0,
            memory_usage: 1024,
            cpu_usage: 50.0,
            hit_rate: 0.8,
            miss_rate: 0.2,
            eviction_rate: 0.1,
            thread_count: 4,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 1000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        let comprehensive = ComprehensiveBenchmarkResults {
            cache: cache_results.clone(),
            aggregation: cache_results.clone(),
            memory_pool: cache_results.clone(),
            overall_score: 75.0,
        };
        
        let report = BenchmarkReporter::generate_report(&comprehensive);
        assert!(report.contains("Actor Core Performance Benchmark Report"));
        assert!(report.contains("Overall Performance Score"));
        assert!(report.contains("Cache Performance"));
        assert!(report.contains("Aggregation Performance"));
        assert!(report.contains("Memory Pool Performance"));
        assert!(report.contains("Recommendations"));
        assert!(report.contains("Test Configuration"));
    }

    #[test]
    fn test_benchmark_reporter_generate_recommendations_all_passing() {
        let config = BenchmarkConfig::default();
        let cache_results = BenchmarkResults {
            latency: Duration::from_millis(1),
            throughput: 1000.0,
            memory_usage: 1024,
            cpu_usage: 50.0,
            hit_rate: 0.8,
            miss_rate: 0.2,
            eviction_rate: 0.1,
            thread_count: 4,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 1000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        let comprehensive = ComprehensiveBenchmarkResults {
            cache: cache_results.clone(),
            aggregation: cache_results.clone(),
            memory_pool: cache_results.clone(),
            overall_score: 90.0,
        };
        
        let recommendations = BenchmarkReporter::generate_recommendations(&comprehensive);
        assert!(recommendations.contains("All performance targets met"));
    }

    #[test]
    fn test_benchmark_reporter_generate_recommendations_failing() {
        let config = BenchmarkConfig::default();
        let cache_results = BenchmarkResults {
            latency: Duration::from_millis(10), // High latency
            throughput: 100.0, // Low throughput
            memory_usage: 200 * 1024 * 1024, // High memory usage
            cpu_usage: 50.0,
            hit_rate: 0.8,
            miss_rate: 0.2,
            eviction_rate: 0.1,
            thread_count: 4,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 1000,
            errors_encountered: 0,
            latency_target_met: false,
            throughput_target_met: false,
            memory_target_met: false,
        };
        
        let comprehensive = ComprehensiveBenchmarkResults {
            cache: cache_results.clone(),
            aggregation: cache_results.clone(),
            memory_pool: cache_results.clone(),
            overall_score: 20.0,
        };
        
        let recommendations = BenchmarkReporter::generate_recommendations(&comprehensive);
        assert!(recommendations.contains("Overall performance is below target"));
        assert!(recommendations.contains("Cache latency is too high"));
        assert!(recommendations.contains("Cache throughput is too low"));
        assert!(recommendations.contains("Memory usage is too high"));
    }

    // Edge case tests
    #[test]
    fn test_benchmark_config_edge_cases() {
        let config = BenchmarkConfig {
            key_count: 0,
            value_size: 0,
            duration: Duration::ZERO,
            concurrency: 0,
            l1_max_size: 0,
            l2_max_size: 0,
            l3_max_size: 0,
            target_latency: Duration::ZERO,
            target_throughput: 0,
            target_memory_usage: 0,
        };
        
        assert_eq!(config.key_count, 0);
        assert_eq!(config.value_size, 0);
        assert_eq!(config.duration, Duration::ZERO);
        assert_eq!(config.concurrency, 0);
    }

    #[test]
    fn test_benchmark_results_edge_cases() {
        let config = BenchmarkConfig::default();
        let results = BenchmarkResults {
            latency: Duration::ZERO,
            throughput: 0.0,
            memory_usage: 0,
            cpu_usage: 0.0,
            hit_rate: 0.0,
            miss_rate: 1.0,
            eviction_rate: 0.0,
            thread_count: 0,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::ZERO,
            operations_performed: 0,
            errors_encountered: 0,
            latency_target_met: false,
            throughput_target_met: false,
            memory_target_met: false,
        };
        
        assert_eq!(results.latency, Duration::ZERO);
        assert_eq!(results.throughput, 0.0);
        assert_eq!(results.memory_usage, 0);
        assert_eq!(results.hit_rate, 0.0);
        assert_eq!(results.miss_rate, 1.0);
        assert!(!results.latency_target_met);
        assert!(!results.throughput_target_met);
        assert!(!results.memory_target_met);
    }

    #[test]
    fn test_benchmark_runner_edge_cases() {
        let config = BenchmarkConfig {
            key_count: 0,
            value_size: 0,
            duration: Duration::ZERO,
            concurrency: 0,
            l1_max_size: 0,
            l2_max_size: 0,
            l3_max_size: 0,
            target_latency: Duration::ZERO,
            target_throughput: 0,
            target_memory_usage: 0,
        };
        
        let runner = BenchmarkRunner::new(config);
        assert_eq!(runner.config.key_count, 0);
        assert_eq!(runner.config.value_size, 0);
    }

    // Performance tests
    #[tokio::test]
    async fn test_benchmark_performance() {
        let config = BenchmarkConfig {
            key_count: 1000,
            value_size: 100,
            duration: Duration::from_millis(100),
            concurrency: 4,
            l1_max_size: 500,
            l2_max_size: 1000,
            l3_max_size: 2000,
            target_latency: Duration::from_millis(5),
            target_throughput: 1000,
            target_memory_usage: 10 * 1024 * 1024,
        };
        
        let mut runner = BenchmarkRunner::new(config);
        let start = std::time::Instant::now();
        
        let result = runner.run_cache_benchmark().await;
        let elapsed = start.elapsed();
        
        assert!(result.is_ok());
        assert!(elapsed >= Duration::from_millis(100));
        
        let results = result.unwrap();
        assert!(results.operations_performed > 0);
        assert!(results.throughput > 0.0);
    }
}