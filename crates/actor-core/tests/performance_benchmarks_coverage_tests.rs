//! Coverage tests for performance benchmarks module.

use actor_core::performance::benchmarks::*;
use std::time::Duration;

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
        duration: Duration::from_secs(60),
        concurrency: 8,
        l1_max_size: 500,
        l2_max_size: 5000,
        l3_max_size: 50000,
        target_latency: Duration::from_millis(2),
        target_throughput: 5000,
        target_memory_usage: 50 * 1024 * 1024,
    };
    
    assert_eq!(config.key_count, 5000);
    assert_eq!(config.value_size, 512);
    assert_eq!(config.duration, Duration::from_secs(60));
    assert_eq!(config.concurrency, 8);
    assert_eq!(config.l1_max_size, 500);
    assert_eq!(config.l2_max_size, 5000);
    assert_eq!(config.l3_max_size, 50000);
    assert_eq!(config.target_latency, Duration::from_millis(2));
    assert_eq!(config.target_throughput, 5000);
    assert_eq!(config.target_memory_usage, 50 * 1024 * 1024);
}

#[test]
fn test_benchmark_config_clone() {
    let config1 = BenchmarkConfig::default();
    let config2 = config1.clone();
    
    assert_eq!(config1.key_count, config2.key_count);
    assert_eq!(config1.value_size, config2.value_size);
    assert_eq!(config1.duration, config2.duration);
    assert_eq!(config1.concurrency, config2.concurrency);
    assert_eq!(config1.l1_max_size, config2.l1_max_size);
    assert_eq!(config1.l2_max_size, config2.l2_max_size);
    assert_eq!(config1.l3_max_size, config2.l3_max_size);
    assert_eq!(config1.target_latency, config2.target_latency);
    assert_eq!(config1.target_throughput, config2.target_throughput);
    assert_eq!(config1.target_memory_usage, config2.target_memory_usage);
}

#[test]
fn test_benchmark_config_debug() {
    let config = BenchmarkConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("BenchmarkConfig"));
    assert!(debug_str.contains("key_count: 10000"));
    assert!(debug_str.contains("value_size: 1024"));
    assert!(debug_str.contains("concurrency: 4"));
}

#[test]
fn test_benchmark_config_serialization() {
    // Note: BenchmarkConfig doesn't implement Serialize/Deserialize
    // This test is removed since the struct doesn't support serialization
}

#[test]
fn test_benchmark_config_custom_values() {
    let config = BenchmarkConfig {
        key_count: 1,
        value_size: 1,
        duration: Duration::from_nanos(1),
        concurrency: 1,
        l1_max_size: 1,
        l2_max_size: 1,
        l3_max_size: 1,
        target_latency: Duration::from_nanos(1),
        target_throughput: 1,
        target_memory_usage: 1,
    };
    
    assert_eq!(config.key_count, 1);
    assert_eq!(config.value_size, 1);
    assert_eq!(config.duration, Duration::from_nanos(1));
    assert_eq!(config.concurrency, 1);
    assert_eq!(config.l1_max_size, 1);
    assert_eq!(config.l2_max_size, 1);
    assert_eq!(config.l3_max_size, 1);
    assert_eq!(config.target_latency, Duration::from_nanos(1));
    assert_eq!(config.target_throughput, 1);
    assert_eq!(config.target_memory_usage, 1);
}

#[test]
fn test_benchmark_config_large_values() {
    let config = BenchmarkConfig {
        key_count: usize::MAX,
        value_size: usize::MAX,
        duration: Duration::from_secs(u64::MAX),
        concurrency: usize::MAX,
        l1_max_size: usize::MAX,
        l2_max_size: usize::MAX,
        l3_max_size: usize::MAX,
        target_latency: Duration::from_secs(u64::MAX),
        target_throughput: u64::MAX,
        target_memory_usage: u64::MAX,
    };
    
    assert_eq!(config.key_count, usize::MAX);
    assert_eq!(config.value_size, usize::MAX);
    assert_eq!(config.duration, Duration::from_secs(u64::MAX));
    assert_eq!(config.concurrency, usize::MAX);
    assert_eq!(config.l1_max_size, usize::MAX);
    assert_eq!(config.l2_max_size, usize::MAX);
    assert_eq!(config.l3_max_size, usize::MAX);
    assert_eq!(config.target_latency, Duration::from_secs(u64::MAX));
    assert_eq!(config.target_throughput, u64::MAX);
    assert_eq!(config.target_memory_usage, u64::MAX);
}

#[test]
fn test_benchmark_config_zero_values() {
    let config = BenchmarkConfig {
        key_count: 0,
        value_size: 0,
        duration: Duration::from_secs(0),
        concurrency: 0,
        l1_max_size: 0,
        l2_max_size: 0,
        l3_max_size: 0,
        target_latency: Duration::from_secs(0),
        target_throughput: 0,
        target_memory_usage: 0,
    };
    
    assert_eq!(config.key_count, 0);
    assert_eq!(config.value_size, 0);
    assert_eq!(config.duration, Duration::from_secs(0));
    assert_eq!(config.concurrency, 0);
    assert_eq!(config.l1_max_size, 0);
    assert_eq!(config.l2_max_size, 0);
    assert_eq!(config.l3_max_size, 0);
    assert_eq!(config.target_latency, Duration::from_secs(0));
    assert_eq!(config.target_throughput, 0);
    assert_eq!(config.target_memory_usage, 0);
}

#[test]
fn test_benchmark_config_equality() {
    let config1 = BenchmarkConfig::default();
    let config2 = BenchmarkConfig::default();
    
    // Test that two identical configs are equal
    assert_eq!(config1.key_count, config2.key_count);
    assert_eq!(config1.value_size, config2.value_size);
    assert_eq!(config1.duration, config2.duration);
    assert_eq!(config1.concurrency, config2.concurrency);
    assert_eq!(config1.l1_max_size, config2.l1_max_size);
    assert_eq!(config1.l2_max_size, config2.l2_max_size);
    assert_eq!(config1.l3_max_size, config2.l3_max_size);
    assert_eq!(config1.target_latency, config2.target_latency);
    assert_eq!(config1.target_throughput, config2.target_throughput);
    assert_eq!(config1.target_memory_usage, config2.target_memory_usage);
}

#[test]
fn test_benchmark_config_duration_conversion() {
    let config = BenchmarkConfig {
        duration: Duration::from_millis(1500),
        target_latency: Duration::from_micros(500),
        ..Default::default()
    };
    
    assert_eq!(config.duration.as_secs(), 1);
    assert_eq!(config.duration.subsec_millis(), 500);
    assert_eq!(config.target_latency.as_micros(), 500);
}

#[test]
fn test_benchmark_config_memory_sizes() {
    let config = BenchmarkConfig {
        l1_max_size: 1024,
        l2_max_size: 1024 * 1024,
        l3_max_size: 1024 * 1024 * 1024,
        target_memory_usage: 1024 * 1024 * 1024 * 1024,
        ..Default::default()
    };
    
    assert_eq!(config.l1_max_size, 1024);
    assert_eq!(config.l2_max_size, 1024 * 1024);
    assert_eq!(config.l3_max_size, 1024 * 1024 * 1024);
    assert_eq!(config.target_memory_usage, 1024 * 1024 * 1024 * 1024);
}
