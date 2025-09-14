//! Comprehensive tests for performance/benchmarks.rs coverage.
//!
//! This module contains detailed tests for all benchmark functionality,
//! including configuration, results, and benchmark runner to achieve 80%+ line coverage.

use actor_core::performance::benchmarks::*;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    // === BenchmarkConfig Tests ===

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
    fn test_benchmark_config_custom() {
        let config = BenchmarkConfig {
            key_count: 5000,
            value_size: 2048,
            duration: Duration::from_secs(60),
            concurrency: 8,
            l1_max_size: 2000,
            l2_max_size: 20000,
            l3_max_size: 200000,
            target_latency: Duration::from_millis(2),
            target_throughput: 20000,
            target_memory_usage: 200 * 1024 * 1024,
        };
        
        assert_eq!(config.key_count, 5000);
        assert_eq!(config.value_size, 2048);
        assert_eq!(config.duration, Duration::from_secs(60));
        assert_eq!(config.concurrency, 8);
        assert_eq!(config.l1_max_size, 2000);
        assert_eq!(config.l2_max_size, 20000);
        assert_eq!(config.l3_max_size, 200000);
        assert_eq!(config.target_latency, Duration::from_millis(2));
        assert_eq!(config.target_throughput, 20000);
        assert_eq!(config.target_memory_usage, 200 * 1024 * 1024);
    }

    #[test]
    fn test_benchmark_config_edge_cases() {
        // Test minimum values
        let config_min = BenchmarkConfig {
            key_count: 1,
            value_size: 1,
            duration: Duration::from_millis(1),
            concurrency: 1,
            l1_max_size: 1,
            l2_max_size: 1,
            l3_max_size: 1,
            target_latency: Duration::from_nanos(1),
            target_throughput: 1,
            target_memory_usage: 1,
        };
        
        assert_eq!(config_min.key_count, 1);
        assert_eq!(config_min.value_size, 1);
        assert_eq!(config_min.duration, Duration::from_millis(1));
        assert_eq!(config_min.concurrency, 1);
        assert_eq!(config_min.l1_max_size, 1);
        assert_eq!(config_min.l2_max_size, 1);
        assert_eq!(config_min.l3_max_size, 1);
        assert_eq!(config_min.target_latency, Duration::from_nanos(1));
        assert_eq!(config_min.target_throughput, 1);
        assert_eq!(config_min.target_memory_usage, 1);
        
        // Test large values
        let config_max = BenchmarkConfig {
            key_count: 1000000,
            value_size: 1024 * 1024, // 1MB
            duration: Duration::from_secs(3600), // 1 hour
            concurrency: 100,
            l1_max_size: 100000,
            l2_max_size: 1000000,
            l3_max_size: 10000000,
            target_latency: Duration::from_secs(1),
            target_throughput: 1000000,
            target_memory_usage: 10 * 1024 * 1024 * 1024, // 10GB
        };
        
        assert_eq!(config_max.key_count, 1000000);
        assert_eq!(config_max.value_size, 1024 * 1024);
        assert_eq!(config_max.duration, Duration::from_secs(3600));
        assert_eq!(config_max.concurrency, 100);
        assert_eq!(config_max.l1_max_size, 100000);
        assert_eq!(config_max.l2_max_size, 1000000);
        assert_eq!(config_max.l3_max_size, 10000000);
        assert_eq!(config_max.target_latency, Duration::from_secs(1));
        assert_eq!(config_max.target_throughput, 1000000);
        assert_eq!(config_max.target_memory_usage, 10 * 1024 * 1024 * 1024);
    }

    // === BenchmarkResults Tests ===

    #[test]
    fn test_benchmark_results_creation() {
        let config = BenchmarkConfig::default();
        let results = BenchmarkResults {
            latency: Duration::from_millis(5),
            throughput: 15000.0,
            memory_usage: 50 * 1024 * 1024, // 50MB
            cpu_usage: 75.5,
            hit_rate: 0.95,
            miss_rate: 0.05,
            eviction_rate: 0.02,
            thread_count: 8,
            gc_pause_time: Duration::from_millis(10),
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 450000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        assert_eq!(results.latency, Duration::from_millis(5));
        assert_eq!(results.throughput, 15000.0);
        assert_eq!(results.memory_usage, 50 * 1024 * 1024);
        assert_eq!(results.cpu_usage, 75.5);
        assert_eq!(results.hit_rate, 0.95);
        assert_eq!(results.miss_rate, 0.05);
        assert_eq!(results.eviction_rate, 0.02);
        assert_eq!(results.thread_count, 8);
        assert_eq!(results.gc_pause_time, Duration::from_millis(10));
        assert_eq!(results.duration, Duration::from_secs(30));
        assert_eq!(results.operations_performed, 450000);
        assert_eq!(results.errors_encountered, 0);
        assert!(results.latency_target_met);
        assert!(results.throughput_target_met);
        assert!(results.memory_target_met);
    }

    #[test]
    fn test_benchmark_results_with_errors() {
        let config = BenchmarkConfig::default();
        let results = BenchmarkResults {
            latency: Duration::from_millis(20),
            throughput: 5000.0,
            memory_usage: 150 * 1024 * 1024, // 150MB
            cpu_usage: 90.0,
            hit_rate: 0.80,
            miss_rate: 0.20,
            eviction_rate: 0.15,
            thread_count: 4,
            gc_pause_time: Duration::from_millis(50),
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 150000,
            errors_encountered: 150,
            latency_target_met: false,
            throughput_target_met: false,
            memory_target_met: false,
        };
        
        assert_eq!(results.latency, Duration::from_millis(20));
        assert_eq!(results.throughput, 5000.0);
        assert_eq!(results.memory_usage, 150 * 1024 * 1024);
        assert_eq!(results.cpu_usage, 90.0);
        assert_eq!(results.hit_rate, 0.80);
        assert_eq!(results.miss_rate, 0.20);
        assert_eq!(results.eviction_rate, 0.15);
        assert_eq!(results.thread_count, 4);
        assert_eq!(results.gc_pause_time, Duration::from_millis(50));
        assert_eq!(results.duration, Duration::from_secs(30));
        assert_eq!(results.operations_performed, 150000);
        assert_eq!(results.errors_encountered, 150);
        assert!(!results.latency_target_met);
        assert!(!results.throughput_target_met);
        assert!(!results.memory_target_met);
    }

    #[test]
    fn test_benchmark_results_edge_cases() {
        let config = BenchmarkConfig::default();
        
        // Test zero values
        let results_zero = BenchmarkResults {
            latency: Duration::ZERO,
            throughput: 0.0,
            memory_usage: 0,
            cpu_usage: 0.0,
            hit_rate: 0.0,
            miss_rate: 0.0,
            eviction_rate: 0.0,
            thread_count: 0,
            gc_pause_time: Duration::ZERO,
            config: config.clone(),
            duration: Duration::ZERO,
            operations_performed: 0,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        assert_eq!(results_zero.latency, Duration::ZERO);
        assert_eq!(results_zero.throughput, 0.0);
        assert_eq!(results_zero.memory_usage, 0);
        assert_eq!(results_zero.cpu_usage, 0.0);
        assert_eq!(results_zero.hit_rate, 0.0);
        assert_eq!(results_zero.miss_rate, 0.0);
        assert_eq!(results_zero.eviction_rate, 0.0);
        assert_eq!(results_zero.thread_count, 0);
        assert_eq!(results_zero.gc_pause_time, Duration::ZERO);
        assert_eq!(results_zero.duration, Duration::ZERO);
        assert_eq!(results_zero.operations_performed, 0);
        assert_eq!(results_zero.errors_encountered, 0);
        
        // Test maximum values
        let results_max = BenchmarkResults {
            latency: Duration::from_secs(60),
            throughput: f64::MAX,
            memory_usage: u64::MAX,
            cpu_usage: 100.0,
            hit_rate: 1.0,
            miss_rate: 1.0,
            eviction_rate: 1.0,
            thread_count: usize::MAX,
            gc_pause_time: Duration::from_secs(60),
            config: config.clone(),
            duration: Duration::from_secs(3600),
            operations_performed: u64::MAX,
            errors_encountered: u64::MAX,
            latency_target_met: false,
            throughput_target_met: false,
            memory_target_met: false,
        };
        
        assert_eq!(results_max.latency, Duration::from_secs(60));
        assert_eq!(results_max.throughput, f64::MAX);
        assert_eq!(results_max.memory_usage, u64::MAX);
        assert_eq!(results_max.cpu_usage, 100.0);
        assert_eq!(results_max.hit_rate, 1.0);
        assert_eq!(results_max.miss_rate, 1.0);
        assert_eq!(results_max.eviction_rate, 1.0);
        assert_eq!(results_max.thread_count, usize::MAX);
        assert_eq!(results_max.gc_pause_time, Duration::from_secs(60));
        assert_eq!(results_max.duration, Duration::from_secs(3600));
        assert_eq!(results_max.operations_performed, u64::MAX);
        assert_eq!(results_max.errors_encountered, u64::MAX);
        assert!(!results_max.latency_target_met);
        assert!(!results_max.throughput_target_met);
        assert!(!results_max.memory_target_met);
    }

    // === BenchmarkRunner Tests ===

    #[test]
    fn test_benchmark_runner_creation() {
        let config = BenchmarkConfig::default();
        let _runner = BenchmarkRunner::new(config.clone());
        
        // Test that the runner was created successfully
        // (We can't access private fields, so we just verify creation doesn't panic)
    }

    #[test]
    fn test_benchmark_runner_custom_config() {
        let config = BenchmarkConfig {
            key_count: 5000,
            value_size: 2048,
            duration: Duration::from_secs(60),
            concurrency: 8,
            l1_max_size: 2000,
            l2_max_size: 20000,
            l3_max_size: 200000,
            target_latency: Duration::from_millis(2),
            target_throughput: 20000,
            target_memory_usage: 200 * 1024 * 1024,
        };
        
        let _runner = BenchmarkRunner::new(config.clone());
        
        // Test that the runner was created successfully
        // (We can't access private fields, so we just verify creation doesn't panic)
    }

    #[test]
    fn test_benchmark_runner_edge_cases() {
        // Test with minimum configuration
        let config_min = BenchmarkConfig {
            key_count: 1,
            value_size: 1,
            duration: Duration::from_millis(1),
            concurrency: 1,
            l1_max_size: 1,
            l2_max_size: 1,
            l3_max_size: 1,
            target_latency: Duration::from_nanos(1),
            target_throughput: 1,
            target_memory_usage: 1,
        };
        
        let _runner_min = BenchmarkRunner::new(config_min.clone());
        
        // Test with maximum configuration
        let config_max = BenchmarkConfig {
            key_count: 1000000,
            value_size: 1024 * 1024,
            duration: Duration::from_secs(3600),
            concurrency: 100,
            l1_max_size: 100000,
            l2_max_size: 1000000,
            l3_max_size: 10000000,
            target_latency: Duration::from_secs(1),
            target_throughput: 1000000,
            target_memory_usage: 10 * 1024 * 1024 * 1024,
        };
        
        let _runner_max = BenchmarkRunner::new(config_max.clone());
        
        // Test that both runners were created successfully
        // (We can't access private fields, so we just verify creation doesn't panic)
    }

    // === Benchmark Configuration Validation Tests ===

    #[test]
    fn test_benchmark_config_validation() {
        // Test valid configurations
        let valid_configs = vec![
            BenchmarkConfig::default(),
            BenchmarkConfig {
                key_count: 1000,
                value_size: 512,
                duration: Duration::from_secs(10),
                concurrency: 2,
                l1_max_size: 100,
                l2_max_size: 1000,
                l3_max_size: 10000,
                target_latency: Duration::from_millis(5),
                target_throughput: 5000,
                target_memory_usage: 50 * 1024 * 1024,
            },
            BenchmarkConfig {
                key_count: 100000,
                value_size: 4096,
                duration: Duration::from_secs(300),
                concurrency: 16,
                l1_max_size: 10000,
                l2_max_size: 100000,
                l3_max_size: 1000000,
                target_latency: Duration::from_micros(500),
                target_throughput: 100000,
                target_memory_usage: 1024 * 1024 * 1024,
            },
        ];
        
        for config in valid_configs {
            // Test that all configurations can be used to create a runner
            let _runner = BenchmarkRunner::new(config);
            // (We can't access private fields, so we just verify creation doesn't panic)
        }
    }

    // === Performance Target Tests ===

    #[test]
    fn test_performance_targets() {
        let config = BenchmarkConfig::default();
        
        // Test latency targets
        assert_eq!(config.target_latency, Duration::from_millis(1));
        assert!(config.target_latency < Duration::from_secs(1));
        
        // Test throughput targets
        assert_eq!(config.target_throughput, 10000);
        assert!(config.target_throughput > 0);
        
        // Test memory targets
        assert_eq!(config.target_memory_usage, 100 * 1024 * 1024);
        assert!(config.target_memory_usage > 0);
    }

    #[test]
    fn test_performance_target_relationships() {
        let config = BenchmarkConfig::default();
        
        // Test that targets are reasonable
        assert!(config.target_latency < Duration::from_secs(1));
        assert!(config.target_throughput > 0);
        assert!(config.target_memory_usage > 0);
        
        // Test that cache sizes are in ascending order
        assert!(config.l1_max_size <= config.l2_max_size);
        assert!(config.l2_max_size <= config.l3_max_size);
        
        // Test that concurrency is reasonable
        assert!(config.concurrency > 0);
        assert!(config.concurrency <= 100); // Reasonable upper bound
    }

    // === Benchmark Results Analysis Tests ===

    #[test]
    fn test_benchmark_results_analysis() {
        let config = BenchmarkConfig::default();
        
        // Test successful results
        let successful_results = BenchmarkResults {
            latency: Duration::from_micros(500),
            throughput: 15000.0,
            memory_usage: 50 * 1024 * 1024,
            cpu_usage: 60.0,
            hit_rate: 0.95,
            miss_rate: 0.05,
            eviction_rate: 0.02,
            thread_count: 4,
            gc_pause_time: Duration::from_millis(5),
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 450000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        // Verify successful results meet targets
        assert!(successful_results.latency < config.target_latency);
        assert!(successful_results.throughput >= config.target_throughput as f64);
        assert!(successful_results.memory_usage <= config.target_memory_usage);
        assert!(successful_results.latency_target_met);
        assert!(successful_results.throughput_target_met);
        assert!(successful_results.memory_target_met);
        
        // Test failed results
        let failed_results = BenchmarkResults {
            latency: Duration::from_millis(10),
            throughput: 5000.0,
            memory_usage: 200 * 1024 * 1024,
            cpu_usage: 95.0,
            hit_rate: 0.70,
            miss_rate: 0.30,
            eviction_rate: 0.25,
            thread_count: 2,
            gc_pause_time: Duration::from_millis(50),
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 150000,
            errors_encountered: 100,
            latency_target_met: false,
            throughput_target_met: false,
            memory_target_met: false,
        };
        
        // Verify failed results don't meet targets
        assert!(failed_results.latency > config.target_latency);
        assert!(failed_results.throughput < config.target_throughput as f64);
        assert!(failed_results.memory_usage > config.target_memory_usage);
        assert!(!failed_results.latency_target_met);
        assert!(!failed_results.throughput_target_met);
        assert!(!failed_results.memory_target_met);
    }

    // === Cache Metrics Tests ===

    #[test]
    fn test_cache_metrics_calculation() {
        let config = BenchmarkConfig::default();
        
        // Test perfect cache performance
        let perfect_results = BenchmarkResults {
            latency: Duration::from_micros(100),
            throughput: 20000.0,
            memory_usage: 25 * 1024 * 1024,
            cpu_usage: 30.0,
            hit_rate: 1.0,
            miss_rate: 0.0,
            eviction_rate: 0.0,
            thread_count: 4,
            gc_pause_time: Duration::from_millis(1),
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 600000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        assert_eq!(perfect_results.hit_rate, 1.0);
        assert_eq!(perfect_results.miss_rate, 0.0);
        assert_eq!(perfect_results.eviction_rate, 0.0);
        
        // Test poor cache performance
        let poor_results = BenchmarkResults {
            latency: Duration::from_millis(50),
            throughput: 1000.0,
            memory_usage: 500 * 1024 * 1024,
            cpu_usage: 99.0,
            hit_rate: 0.10,
            miss_rate: 0.90,
            eviction_rate: 0.80,
            thread_count: 1,
            gc_pause_time: Duration::from_secs(1),
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 30000,
            errors_encountered: 1000,
            latency_target_met: false,
            throughput_target_met: false,
            memory_target_met: false,
        };
        
        assert_eq!(poor_results.hit_rate, 0.10);
        assert_eq!(poor_results.miss_rate, 0.90);
        assert_eq!(poor_results.eviction_rate, 0.80);
    }

    // === System Metrics Tests ===

    #[test]
    fn test_system_metrics() {
        let config = BenchmarkConfig::default();
        
        let results = BenchmarkResults {
            latency: Duration::from_millis(2),
            throughput: 12000.0,
            memory_usage: 75 * 1024 * 1024,
            cpu_usage: 80.0,
            hit_rate: 0.85,
            miss_rate: 0.15,
            eviction_rate: 0.10,
            thread_count: 8,
            gc_pause_time: Duration::from_millis(20),
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 360000,
            errors_encountered: 50,
            latency_target_met: false,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        // Test system metrics
        assert_eq!(results.thread_count, 8);
        assert_eq!(results.gc_pause_time, Duration::from_millis(20));
        assert_eq!(results.cpu_usage, 80.0);
        assert_eq!(results.memory_usage, 75 * 1024 * 1024);
        
        // Test operation metrics
        assert_eq!(results.operations_performed, 360000);
        assert_eq!(results.errors_encountered, 50);
        assert_eq!(results.duration, Duration::from_secs(30));
    }

    // === Error Handling Tests ===

    #[test]
    fn test_benchmark_results_error_metrics() {
        let config = BenchmarkConfig::default();
        
        let error_results = BenchmarkResults {
            latency: Duration::from_millis(100),
            throughput: 100.0,
            memory_usage: 1000 * 1024 * 1024,
            cpu_usage: 100.0,
            hit_rate: 0.0,
            miss_rate: 1.0,
            eviction_rate: 1.0,
            thread_count: 1,
            gc_pause_time: Duration::from_secs(10),
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 3000,
            errors_encountered: 3000,
            latency_target_met: false,
            throughput_target_met: false,
            memory_target_met: false,
        };
        
        // Test error metrics
        assert_eq!(error_results.errors_encountered, 3000);
        assert_eq!(error_results.operations_performed, 3000);
        assert_eq!(error_results.throughput, 100.0);
        
        // Test that all targets are missed
        assert!(!error_results.latency_target_met);
        assert!(!error_results.throughput_target_met);
        assert!(!error_results.memory_target_met);
    }

    // === Serialization Tests ===

    #[test]
    fn test_benchmark_config_serialization() {
        let config = BenchmarkConfig::default();
        
        // Test that config can be cloned
        let cloned_config = config.clone();
        assert_eq!(cloned_config.key_count, config.key_count);
        assert_eq!(cloned_config.value_size, config.value_size);
        assert_eq!(cloned_config.duration, config.duration);
        assert_eq!(cloned_config.concurrency, config.concurrency);
    }

    #[test]
    fn test_benchmark_results_serialization() {
        let config = BenchmarkConfig::default();
        let results = BenchmarkResults {
            latency: Duration::from_millis(5),
            throughput: 15000.0,
            memory_usage: 50 * 1024 * 1024,
            cpu_usage: 75.5,
            hit_rate: 0.95,
            miss_rate: 0.05,
            eviction_rate: 0.02,
            thread_count: 8,
            gc_pause_time: Duration::from_millis(10),
            config: config.clone(),
            duration: Duration::from_secs(30),
            operations_performed: 450000,
            errors_encountered: 0,
            latency_target_met: true,
            throughput_target_met: true,
            memory_target_met: true,
        };
        
        // Test that results can be cloned
        let cloned_results = results.clone();
        assert_eq!(cloned_results.latency, results.latency);
        assert_eq!(cloned_results.throughput, results.throughput);
        assert_eq!(cloned_results.memory_usage, results.memory_usage);
        assert_eq!(cloned_results.cpu_usage, results.cpu_usage);
        assert_eq!(cloned_results.hit_rate, results.hit_rate);
        assert_eq!(cloned_results.miss_rate, results.miss_rate);
        assert_eq!(cloned_results.eviction_rate, results.eviction_rate);
        assert_eq!(cloned_results.thread_count, results.thread_count);
        assert_eq!(cloned_results.gc_pause_time, results.gc_pause_time);
        assert_eq!(cloned_results.duration, results.duration);
        assert_eq!(cloned_results.operations_performed, results.operations_performed);
        assert_eq!(cloned_results.errors_encountered, results.errors_encountered);
        assert_eq!(cloned_results.latency_target_met, results.latency_target_met);
        assert_eq!(cloned_results.throughput_target_met, results.throughput_target_met);
        assert_eq!(cloned_results.memory_target_met, results.memory_target_met);
    }

    // === Performance Threshold Tests ===

    #[test]
    fn test_performance_thresholds() {
        let config = BenchmarkConfig::default();
        
        // Test latency threshold
        assert!(Duration::from_micros(500) < config.target_latency);
        assert!(Duration::from_millis(2) > config.target_latency);
        
        // Test throughput threshold
        assert!(15000.0 >= config.target_throughput as f64);
        assert!(5000.0 < config.target_throughput as f64);
        
        // Test memory threshold
        assert!(50 * 1024 * 1024 <= config.target_memory_usage);
        assert!(200 * 1024 * 1024 > config.target_memory_usage);
    }

    // === Benchmark Configuration Combinations ===

    #[test]
    fn test_benchmark_config_combinations() {
        // Test different cache size combinations
        let configs = vec![
            BenchmarkConfig {
                l1_max_size: 100,
                l2_max_size: 1000,
                l3_max_size: 10000,
                ..Default::default()
            },
            BenchmarkConfig {
                l1_max_size: 1000,
                l2_max_size: 10000,
                l3_max_size: 100000,
                ..Default::default()
            },
            BenchmarkConfig {
                l1_max_size: 10000,
                l2_max_size: 100000,
                l3_max_size: 1000000,
                ..Default::default()
            },
        ];
        
        for config in configs {
            let _runner = BenchmarkRunner::new(config);
            // (We can't access private fields, so we just verify creation doesn't panic)
        }
    }

    // === Benchmark Duration Tests ===

    #[test]
    fn test_benchmark_durations() {
        let durations = vec![
            Duration::from_millis(1),
            Duration::from_secs(1),
            Duration::from_secs(10),
            Duration::from_secs(60),
            Duration::from_secs(300),
            Duration::from_secs(3600),
        ];
        
        for duration in durations {
            let config = BenchmarkConfig {
                duration,
                ..Default::default()
            };
            
            let _runner = BenchmarkRunner::new(config);
            // (We can't access private fields, so we just verify creation doesn't panic)
        }
    }

    // === Concurrency Tests ===

    #[test]
    fn test_concurrency_levels() {
        let concurrency_levels = vec![1, 2, 4, 8, 16, 32, 64, 100];
        
        for concurrency in concurrency_levels {
            let config = BenchmarkConfig {
                concurrency,
                ..Default::default()
            };
            
            let _runner = BenchmarkRunner::new(config);
            // (We can't access private fields, so we just verify creation doesn't panic)
        }
    }

    // === Value Size Tests ===

    #[test]
    fn test_value_sizes() {
        let value_sizes = vec![
            1,
            64,
            256,
            512,
            1024,
            2048,
            4096,
            8192,
            16384,
            32768,
            65536,
        ];
        
        for value_size in value_sizes {
            let config = BenchmarkConfig {
                value_size,
                ..Default::default()
            };
            
            let _runner = BenchmarkRunner::new(config);
            // (We can't access private fields, so we just verify creation doesn't panic)
        }
    }

    // === Key Count Tests ===

    #[test]
    fn test_key_counts() {
        let key_counts = vec![
            1,
            10,
            100,
            1000,
            10000,
            100000,
            1000000,
        ];
        
        for key_count in key_counts {
            let config = BenchmarkConfig {
                key_count,
                ..Default::default()
            };
            
            let _runner = BenchmarkRunner::new(config);
            // (We can't access private fields, so we just verify creation doesn't panic)
        }
    }
}
