//! Criterion Benchmark Configuration
//! 
//! This module provides shared configuration and utilities for all benchmarks.

use criterion::{Criterion, BenchmarkId, Throughput, PlotConfiguration, AxisScale};
use std::time::Duration;

/// Create a standard criterion configuration
pub fn create_criterion() -> Criterion {
    Criterion::default()
        .sample_size(100) // Number of samples to collect
        .measurement_time(Duration::from_secs(10)) // Maximum measurement time
        .warm_up_time(Duration::from_secs(2)) // Warm-up time
        .plot_config(PlotConfiguration::default()
            .summary_scale(AxisScale::Logarithmic) // Logarithmic scale for better visualization
        )
}

/// Create a fast criterion configuration for quick benchmarks
pub fn create_fast_criterion() -> Criterion {
    Criterion::default()
        .sample_size(10) // Fewer samples for faster execution
        .measurement_time(Duration::from_secs(5)) // Shorter measurement time
        .warm_up_time(Duration::from_secs(1)) // Shorter warm-up time
        .sampling_mode(SamplingMode::Auto)
}

/// Create a thorough criterion configuration for detailed benchmarks
pub fn create_thorough_criterion() -> Criterion {
    Criterion::default()
        .sample_size(200) // More samples for better accuracy
        .measurement_time(Duration::from_secs(30)) // Longer measurement time
        .warm_up_time(Duration::from_secs(5)) // Longer warm-up time
        .sampling_mode(SamplingMode::Auto)
        .plot_config(PlotConfiguration::default()
            .summary_scale(AxisScale::Logarithmic)
        )
}

/// Common benchmark input sizes for scaling tests
pub const COMMON_SIZES: &[usize] = &[1, 10, 100, 1000, 10000];

/// Large benchmark input sizes for stress tests
pub const LARGE_SIZES: &[usize] = &[1000, 10000, 100000, 1000000];

/// Small benchmark input sizes for micro-benchmarks
pub const SMALL_SIZES: &[usize] = &[1, 10, 100];

/// Medium benchmark input sizes for standard tests
pub const MEDIUM_SIZES: &[usize] = &[10, 100, 1000, 10000];

/// Create a benchmark ID with size
pub fn create_benchmark_id(name: &str, size: usize) -> BenchmarkId {
    BenchmarkId::new(name, size)
}

/// Create a benchmark ID with size and additional info
pub fn create_benchmark_id_with_info(name: &str, size: usize, info: &str) -> BenchmarkId {
    BenchmarkId::new(format!("{}_{}", name, info), size)
}

/// Create throughput for elements
pub fn create_throughput_elements(count: usize) -> Throughput {
    Throughput::Elements(count as u64)
}

/// Create throughput for bytes
pub fn create_throughput_bytes(bytes: usize) -> Throughput {
    Throughput::Bytes(bytes as u64)
}

/// Common performance thresholds (in nanoseconds)
pub mod thresholds {
    /// Microsecond threshold (1000 ns)
    pub const MICROSECOND: u64 = 1_000;
    
    /// Millisecond threshold (1,000,000 ns)
    pub const MILLISECOND: u64 = 1_000_000;
    
    /// 10 millisecond threshold (10,000,000 ns)
    pub const TEN_MILLISECONDS: u64 = 10_000_000;
    
    /// 100 millisecond threshold (100,000,000 ns)
    pub const HUNDRED_MILLISECONDS: u64 = 100_000_000;
    
    /// Second threshold (1,000,000,000 ns)
    pub const SECOND: u64 = 1_000_000_000;
}

/// Performance categories for different operations
pub mod categories {
    /// Fast operations (should be < 1ms)
    pub const FAST: &str = "fast";
    
    /// Medium operations (should be < 10ms)
    pub const MEDIUM: &str = "medium";
    
    /// Slow operations (should be < 100ms)
    pub const SLOW: &str = "slow";
    
    /// Very slow operations (can be > 100ms)
    pub const VERY_SLOW: &str = "very_slow";
}

/// Benchmark utilities for common patterns
pub mod utils {
    use super::*;
    use std::collections::HashMap;
    use actor_core::types::*;
    use actor_core::enums::*;
    
    /// Generate test actors for benchmarking
    pub fn generate_actors(count: usize) -> Vec<Actor> {
        (0..count)
            .map(|i| Actor::new(format!("Actor_{}", i), "Human".to_string()))
            .collect()
    }
    
    /// Generate test caps for benchmarking
    pub fn generate_caps(count: usize) -> HashMap<String, Caps> {
        (0..count)
            .map(|i| {
                let stat_name = format!("stat_{}", i % 20);
                let min = (i as f64) * 0.5;
                let max = (i as f64) * 0.5 + 100.0;
                (stat_name, Caps::new(min, max))
            })
            .collect()
    }
    
    /// Generate test contributions for benchmarking
    pub fn generate_contributions(count: usize) -> Vec<Contribution> {
        (0..count)
            .map(|i| {
                let bucket = match i % 4 {
                    0 => Bucket::Flat,
                    1 => Bucket::Mult,
                    2 => Bucket::PostAdd,
                    _ => Bucket::Override,
                };
                Contribution::new(
                    format!("stat_{}", i % 20),
                    bucket,
                    (i as f64) * 0.1 + 1.0,
                    format!("source_{}", i % 10)
                )
            })
            .collect()
    }
    
    /// Generate test snapshots for benchmarking
    pub fn generate_snapshots(count: usize) -> Vec<Snapshot> {
        (0..count)
            .map(|i| {
                let mut snapshot = Snapshot::new(uuid::Uuid::new_v4(), i as u64);
                
                // Add primary stats
                for j in 0..10 {
                    let contrib = Contribution::new(
                        format!("stat_{}", j),
                        Bucket::Flat,
                        (i + j) as f64,
                        "benchmark".to_string()
                    );
                    snapshot.add_primary(contrib);
                }
                
                // Add derived stats
                for j in 0..5 {
                    let contrib = Contribution::new(
                        format!("derived_{}", j),
                        Bucket::Flat,
                        (i + j) as f64 * 1.5,
                        "benchmark".to_string()
                    );
                    snapshot.add_derived(contrib);
                }
                
                snapshot
            })
            .collect()
    }
    
    /// Generate test data for actors
    pub fn generate_actor_data(count: usize) -> HashMap<String, serde_json::Value> {
        let mut data = HashMap::new();
        for i in 0..count {
            data.insert(
                format!("key_{}", i),
                serde_json::Value::String(format!("value_{}", i))
            );
        }
        data
    }
}

/// Benchmark result analysis utilities
pub mod analysis {
    use criterion::measurement::Measurement;
    use std::collections::HashMap;
    
    /// Analyze benchmark results for performance regressions
    pub fn analyze_performance_regression<T: Measurement>(
        baseline: &criterion::BenchmarkResult<T>,
        current: &criterion::BenchmarkResult<T>,
        threshold: f64,
    ) -> bool {
        let baseline_mean = baseline.measurement.mean();
        let current_mean = current.measurement.mean();
        
        let regression_ratio = current_mean / baseline_mean;
        regression_ratio > (1.0 + threshold)
    }
    
    /// Calculate performance improvement ratio
    pub fn calculate_improvement_ratio<T: Measurement>(
        baseline: &criterion::BenchmarkResult<T>,
        current: &criterion::BenchmarkResult<T>,
    ) -> f64 {
        let baseline_mean = baseline.measurement.mean();
        let current_mean = current.measurement.mean();
        
        baseline_mean / current_mean
    }
    
    /// Generate performance report
    pub fn generate_performance_report<T: Measurement>(
        results: &HashMap<String, criterion::BenchmarkResult<T>>,
    ) -> String {
        let mut report = String::new();
        report.push_str("Performance Benchmark Report\n");
        report.push_str("==========================\n\n");
        
        for (name, result) in results {
            report.push_str(&format!("{}: {:.2}ns\n", name, result.measurement.mean()));
        }
        
        report
    }
}
