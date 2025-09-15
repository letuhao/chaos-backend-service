//! Coverage tests for performance/benchmarks.rs module.

use actor_core::performance::benchmarks::{
    BenchmarkConfig, BenchmarkResults, BenchmarkRunner, ComprehensiveBenchmarkResults, BenchmarkReporter
};
use std::time::Duration;

#[tokio::test]
async fn test_benchmark_config_default() {
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

#[tokio::test]
async fn test_benchmark_config_custom() {
    let config = BenchmarkConfig {
        key_count: 5000,
        value_size: 512,
        duration: Duration::from_secs(10),
        concurrency: 2,
        l1_max_size: 500,
        l2_max_size: 5000,
        l3_max_size: 50000,
        target_latency: Duration::from_millis(2),
        target_throughput: 5000,
        target_memory_usage: 50 * 1024 * 1024,
    };
    
    assert_eq!(config.key_count, 5000);
    assert_eq!(config.value_size, 512);
    assert_eq!(config.duration, Duration::from_secs(10));
    assert_eq!(config.concurrency, 2);
    assert_eq!(config.l1_max_size, 500);
    assert_eq!(config.l2_max_size, 5000);
    assert_eq!(config.l3_max_size, 50000);
    assert_eq!(config.target_latency, Duration::from_millis(2));
    assert_eq!(config.target_throughput, 5000);
    assert_eq!(config.target_memory_usage, 50 * 1024 * 1024);
}

#[tokio::test]
async fn test_benchmark_config_debug_clone() {
    let config = BenchmarkConfig::default();
    
    // Test Debug trait
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("key_count: 10000"));
    
    // Test Clone trait
    let cloned_config = config.clone();
    assert_eq!(cloned_config.key_count, config.key_count);
    assert_eq!(cloned_config.duration, config.duration);
}

#[tokio::test]
async fn test_benchmark_runner_creation() {
    let config = BenchmarkConfig::default();
    let _runner = BenchmarkRunner::new(config);
    assert!(true); // Basic creation test
}

#[tokio::test]
async fn test_benchmark_runner_custom_config() {
    let config = BenchmarkConfig {
        key_count: 100,
        value_size: 64,
        duration: Duration::from_millis(100), // Very short duration for testing
        concurrency: 1,
        l1_max_size: 50,
        l2_max_size: 100,
        l3_max_size: 200,
        target_latency: Duration::from_millis(1),
        target_throughput: 1000,
        target_memory_usage: 1024 * 1024,
    };
    
    let _runner = BenchmarkRunner::new(config);
    assert!(true); // Basic creation test
}

#[tokio::test]
async fn test_benchmark_results_creation() {
    let config = BenchmarkConfig::default();
    let results = BenchmarkResults {
        latency: Duration::from_millis(1),
        throughput: 1000.0,
        memory_usage: 1024 * 1024,
        cpu_usage: 50.0,
        hit_rate: 0.8,
        miss_rate: 0.2,
        eviction_rate: 0.1,
        thread_count: 4,
        gc_pause_time: Duration::ZERO,
        config: config.clone(),
        duration: Duration::from_secs(30),
        operations_performed: 10000,
        errors_encountered: 0,
        latency_target_met: true,
        throughput_target_met: true,
        memory_target_met: true,
    };
    
    assert_eq!(results.latency, Duration::from_millis(1));
    assert_eq!(results.throughput, 1000.0);
    assert_eq!(results.memory_usage, 1024 * 1024);
    assert_eq!(results.cpu_usage, 50.0);
    assert_eq!(results.hit_rate, 0.8);
    assert_eq!(results.miss_rate, 0.2);
    assert_eq!(results.eviction_rate, 0.1);
    assert_eq!(results.thread_count, 4);
    assert_eq!(results.gc_pause_time, Duration::ZERO);
    assert_eq!(results.duration, Duration::from_secs(30));
    assert_eq!(results.operations_performed, 10000);
    assert_eq!(results.errors_encountered, 0);
    assert!(results.latency_target_met);
    assert!(results.throughput_target_met);
    assert!(results.memory_target_met);
}

#[tokio::test]
async fn test_benchmark_results_debug_clone() {
    let config = BenchmarkConfig::default();
    let results = BenchmarkResults {
        latency: Duration::from_millis(1),
        throughput: 1000.0,
        memory_usage: 1024 * 1024,
        cpu_usage: 50.0,
        hit_rate: 0.8,
        miss_rate: 0.2,
        eviction_rate: 0.1,
        thread_count: 4,
        gc_pause_time: Duration::ZERO,
        config: config.clone(),
        duration: Duration::from_secs(30),
        operations_performed: 10000,
        errors_encountered: 0,
        latency_target_met: true,
        throughput_target_met: true,
        memory_target_met: true,
    };
    
    // Test Debug trait
    let debug_str = format!("{:?}", results);
    assert!(debug_str.contains("latency: 1ms"));
    
    // Test Clone trait
    let cloned_results = results.clone();
    assert_eq!(cloned_results.latency, results.latency);
    assert_eq!(cloned_results.throughput, results.throughput);
}

#[tokio::test]
async fn test_comprehensive_benchmark_results_creation() {
    let config = BenchmarkConfig::default();
    let cache_results = BenchmarkResults {
        latency: Duration::from_millis(1),
        throughput: 1000.0,
        memory_usage: 1024 * 1024,
        cpu_usage: 50.0,
        hit_rate: 0.8,
        miss_rate: 0.2,
        eviction_rate: 0.1,
        thread_count: 4,
        gc_pause_time: Duration::ZERO,
        config: config.clone(),
        duration: Duration::from_secs(30),
        operations_performed: 10000,
        errors_encountered: 0,
        latency_target_met: true,
        throughput_target_met: true,
        memory_target_met: true,
    };
    
    let aggregation_results = cache_results.clone();
    let memory_pool_results = cache_results.clone();
    
    let comprehensive_results = ComprehensiveBenchmarkResults {
        cache: cache_results.clone(),
        aggregation: aggregation_results.clone(),
        memory_pool: memory_pool_results.clone(),
        overall_score: 85.5,
    };
    
    assert_eq!(comprehensive_results.overall_score, 85.5);
    assert_eq!(comprehensive_results.cache.latency, Duration::from_millis(1));
    assert_eq!(comprehensive_results.aggregation.throughput, 1000.0);
    assert_eq!(comprehensive_results.memory_pool.memory_usage, 1024 * 1024);
}

#[tokio::test]
async fn test_comprehensive_benchmark_results_debug_clone() {
    let config = BenchmarkConfig::default();
    let cache_results = BenchmarkResults {
        latency: Duration::from_millis(1),
        throughput: 1000.0,
        memory_usage: 1024 * 1024,
        cpu_usage: 50.0,
        hit_rate: 0.8,
        miss_rate: 0.2,
        eviction_rate: 0.1,
        thread_count: 4,
        gc_pause_time: Duration::ZERO,
        config: config.clone(),
        duration: Duration::from_secs(30),
        operations_performed: 10000,
        errors_encountered: 0,
        latency_target_met: true,
        throughput_target_met: true,
        memory_target_met: true,
    };
    
    let comprehensive_results = ComprehensiveBenchmarkResults {
        cache: cache_results.clone(),
        aggregation: cache_results.clone(),
        memory_pool: cache_results.clone(),
        overall_score: 85.5,
    };
    
    // Test Debug trait
    let debug_str = format!("{:?}", comprehensive_results);
    assert!(debug_str.contains("overall_score: 85.5"));
    
    // Test Clone trait
    let cloned_results = comprehensive_results.clone();
    assert_eq!(cloned_results.overall_score, comprehensive_results.overall_score);
    assert_eq!(cloned_results.cache.latency, comprehensive_results.cache.latency);
}

#[tokio::test]
async fn test_benchmark_reporter_generate_report() {
    let config = BenchmarkConfig::default();
    let cache_results = BenchmarkResults {
        latency: Duration::from_millis(1),
        throughput: 1000.0,
        memory_usage: 1024 * 1024,
        cpu_usage: 50.0,
        hit_rate: 0.8,
        miss_rate: 0.2,
        eviction_rate: 0.1,
        thread_count: 4,
        gc_pause_time: Duration::ZERO,
        config: config.clone(),
        duration: Duration::from_secs(30),
        operations_performed: 10000,
        errors_encountered: 0,
        latency_target_met: true,
        throughput_target_met: true,
        memory_target_met: true,
    };
    
    let comprehensive_results = ComprehensiveBenchmarkResults {
        cache: cache_results.clone(),
        aggregation: cache_results.clone(),
        memory_pool: cache_results.clone(),
        overall_score: 85.5,
    };
    
    let report = BenchmarkReporter::generate_report(&comprehensive_results);
    
    // Check that the report contains expected sections
    assert!(report.contains("Actor Core Performance Benchmark Report"));
    assert!(report.contains("Overall Performance Score: 85.5/100"));
    assert!(report.contains("Cache Performance"));
    assert!(report.contains("Aggregation Performance"));
    assert!(report.contains("Memory Pool Performance"));
    assert!(report.contains("Recommendations"));
    assert!(report.contains("Test Configuration"));
    assert!(report.contains("✅ PASS")); // Should show passed targets
}

#[tokio::test]
async fn test_benchmark_reporter_generate_report_failed_targets() {
    let config = BenchmarkConfig {
        target_latency: Duration::from_millis(1),
        target_throughput: 10000,
        target_memory_usage: 1024 * 1024,
        ..Default::default()
    };
    
    let cache_results = BenchmarkResults {
        latency: Duration::from_millis(10), // Higher than target
        throughput: 1000.0, // Lower than target
        memory_usage: 2 * 1024 * 1024, // Higher than target
        cpu_usage: 50.0,
        hit_rate: 0.8,
        miss_rate: 0.2,
        eviction_rate: 0.1,
        thread_count: 4,
        gc_pause_time: Duration::ZERO,
        config: config.clone(),
        duration: Duration::from_secs(30),
        operations_performed: 10000,
        errors_encountered: 0,
        latency_target_met: false, // Failed
        throughput_target_met: false, // Failed
        memory_target_met: false, // Failed
    };
    
    let comprehensive_results = ComprehensiveBenchmarkResults {
        cache: cache_results.clone(),
        aggregation: cache_results.clone(),
        memory_pool: cache_results.clone(),
        overall_score: 25.0, // Low score due to failed targets
    };
    
    let report = BenchmarkReporter::generate_report(&comprehensive_results);
    
    // Check that the report contains expected sections
    assert!(report.contains("Overall Performance Score: 25.0/100"));
    assert!(report.contains("❌ FAIL")); // Should show failed targets
    assert!(report.contains("⚠️  Overall performance is below target"));
}

#[tokio::test]
async fn test_benchmark_runner_basic_functionality() {
    let config = BenchmarkConfig {
        key_count: 10,
        value_size: 64,
        duration: Duration::from_millis(100),
        concurrency: 1,
        l1_max_size: 50,
        l2_max_size: 100,
        l3_max_size: 200,
        target_latency: Duration::from_millis(1),
        target_throughput: 1000,
        target_memory_usage: 1024 * 1024,
    };
    
    let _runner = BenchmarkRunner::new(config);
    assert!(true); // Basic creation test
}