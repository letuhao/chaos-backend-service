//! Performance profiler coverage tests for Actor Core.

use actor_core::performance::profiler::{
    ProfilerConfig, PerformanceThresholds, PerformanceMetrics,
    AggregationMetrics, CacheMetrics, SystemMetrics, ErrorMetrics,
    PerformanceSnapshot, ThresholdViolation, ViolationSeverity,
    PerformanceTestResult
};
use std::time::Duration;
use std::collections::HashMap;

#[test]
fn test_profiler_config_default() {
    let config = ProfilerConfig::default();
    
    assert!(config.enable_continuous_profiling);
    assert_eq!(config.profiling_interval, Duration::from_secs(5));
    assert_eq!(config.max_history_size, 1000);
    assert!(config.enable_threshold_checking);
    assert!(config.enable_regression_detection);
    assert_eq!(config.regression_window, 10);
    assert_eq!(config.degradation_threshold, 10.0);
    assert!(!config.enable_detailed_profiling);
}

#[test]
fn test_profiler_config_creation() {
    let config = ProfilerConfig {
        enable_continuous_profiling: false,
        profiling_interval: Duration::from_secs(10),
        max_history_size: 500,
        enable_threshold_checking: false,
        enable_regression_detection: false,
        regression_window: 5,
        degradation_threshold: 15.0,
        enable_detailed_profiling: true,
    };
    
    assert!(!config.enable_continuous_profiling);
    assert_eq!(config.profiling_interval, Duration::from_secs(10));
    assert_eq!(config.max_history_size, 500);
    assert!(!config.enable_threshold_checking);
    assert!(!config.enable_regression_detection);
    assert_eq!(config.regression_window, 5);
    assert_eq!(config.degradation_threshold, 15.0);
    assert!(config.enable_detailed_profiling);
}

#[test]
fn test_profiler_config_clone() {
    let config = ProfilerConfig::default();
    let cloned = config.clone();
    
    assert_eq!(cloned.enable_continuous_profiling, config.enable_continuous_profiling);
    assert_eq!(cloned.profiling_interval, config.profiling_interval);
    assert_eq!(cloned.max_history_size, config.max_history_size);
    assert_eq!(cloned.enable_threshold_checking, config.enable_threshold_checking);
    assert_eq!(cloned.enable_regression_detection, config.enable_regression_detection);
    assert_eq!(cloned.regression_window, config.regression_window);
    assert_eq!(cloned.degradation_threshold, config.degradation_threshold);
    assert_eq!(cloned.enable_detailed_profiling, config.enable_detailed_profiling);
}

#[test]
fn test_profiler_config_debug() {
    let config = ProfilerConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("ProfilerConfig"));
    assert!(debug_str.contains("enable_continuous_profiling: true"));
    assert!(debug_str.contains("profiling_interval: 5s"));
}

#[test]
fn test_performance_thresholds_default() {
    let thresholds = PerformanceThresholds::default();
    
    assert!(thresholds.max_aggregation_time > 0);
    assert!(thresholds.max_cache_time > 0);
    assert!(thresholds.max_subsystem_time > 0);
    assert!(thresholds.max_memory_per_actor > 0);
    assert!(thresholds.max_cache_size > 0);
    assert_eq!(thresholds.min_throughput, 1000);
    assert_eq!(thresholds.max_latency, 10_000);
    assert_eq!(thresholds.max_error_rate, 1.0);
    assert_eq!(thresholds.max_cpu_usage, 80.0);
    assert_eq!(thresholds.max_memory_usage, 85.0);
}

#[test]
fn test_performance_thresholds_creation() {
    let thresholds = PerformanceThresholds {
        max_aggregation_time: 1000,
        max_cache_time: 500,
        max_subsystem_time: 2000,
        max_memory_per_actor: 1024 * 1024 * 100, // 100 MB
        max_cache_size: 1024 * 1024 * 500, // 500 MB
        min_throughput: 2000,
        max_latency: 5000,
        max_error_rate: 0.5,
        max_cpu_usage: 70.0,
        max_memory_usage: 80.0,
    };
    
    assert_eq!(thresholds.max_aggregation_time, 1000);
    assert_eq!(thresholds.max_cache_time, 500);
    assert_eq!(thresholds.max_subsystem_time, 2000);
    assert_eq!(thresholds.max_memory_per_actor, 1024 * 1024 * 100);
    assert_eq!(thresholds.max_cache_size, 1024 * 1024 * 500);
    assert_eq!(thresholds.min_throughput, 2000);
    assert_eq!(thresholds.max_latency, 5000);
    assert_eq!(thresholds.max_error_rate, 0.5);
    assert_eq!(thresholds.max_cpu_usage, 70.0);
    assert_eq!(thresholds.max_memory_usage, 80.0);
}

#[test]
fn test_performance_thresholds_clone() {
    let thresholds = PerformanceThresholds::default();
    let cloned = thresholds.clone();
    
    assert_eq!(cloned.max_aggregation_time, thresholds.max_aggregation_time);
    assert_eq!(cloned.max_cache_time, thresholds.max_cache_time);
    assert_eq!(cloned.max_subsystem_time, thresholds.max_subsystem_time);
    assert_eq!(cloned.max_memory_per_actor, thresholds.max_memory_per_actor);
    assert_eq!(cloned.max_cache_size, thresholds.max_cache_size);
    assert_eq!(cloned.min_throughput, thresholds.min_throughput);
    assert_eq!(cloned.max_latency, thresholds.max_latency);
    assert_eq!(cloned.max_error_rate, thresholds.max_error_rate);
    assert_eq!(cloned.max_cpu_usage, thresholds.max_cpu_usage);
    assert_eq!(cloned.max_memory_usage, thresholds.max_memory_usage);
}

#[test]
fn test_performance_thresholds_debug() {
    let thresholds = PerformanceThresholds::default();
    let debug_str = format!("{:?}", thresholds);
    
    assert!(debug_str.contains("PerformanceThresholds"));
    assert!(debug_str.contains("max_aggregation_time:"));
    assert!(debug_str.contains("min_throughput: 1000"));
}

#[test]
fn test_aggregation_metrics_creation() {
    let metrics = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    assert_eq!(metrics.avg_aggregation_time, 1000);
    assert_eq!(metrics.max_aggregation_time, 2000);
    assert_eq!(metrics.min_aggregation_time, 500);
    assert_eq!(metrics.total_aggregations, 1000);
    assert_eq!(metrics.aggregations_per_second, 10.5);
    assert_eq!(metrics.avg_contributions_per_aggregation, 5.2);
    assert_eq!(metrics.avg_subsystems_per_aggregation, 3.1);
}

#[test]
fn test_aggregation_metrics_clone() {
    let metrics = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    let cloned = metrics.clone();
    assert_eq!(cloned.avg_aggregation_time, metrics.avg_aggregation_time);
    assert_eq!(cloned.max_aggregation_time, metrics.max_aggregation_time);
    assert_eq!(cloned.min_aggregation_time, metrics.min_aggregation_time);
    assert_eq!(cloned.total_aggregations, metrics.total_aggregations);
    assert_eq!(cloned.aggregations_per_second, metrics.aggregations_per_second);
    assert_eq!(cloned.avg_contributions_per_aggregation, metrics.avg_contributions_per_aggregation);
    assert_eq!(cloned.avg_subsystems_per_aggregation, metrics.avg_subsystems_per_aggregation);
}

#[test]
fn test_aggregation_metrics_debug() {
    let metrics = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    let debug_str = format!("{:?}", metrics);
    assert!(debug_str.contains("AggregationMetrics"));
    assert!(debug_str.contains("avg_aggregation_time: 1000"));
    assert!(debug_str.contains("total_aggregations: 1000"));
}

#[test]
fn test_cache_metrics_creation() {
    let metrics = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100, // 100 MB
        eviction_rate: 5.2,
    };
    
    assert_eq!(metrics.hit_rate, 85.5);
    assert_eq!(metrics.miss_rate, 14.5);
    assert_eq!(metrics.avg_operation_time, 100);
    assert_eq!(metrics.max_operation_time, 500);
    assert_eq!(metrics.total_operations, 10000);
    assert_eq!(metrics.operations_per_second, 100.0);
    assert_eq!(metrics.current_size, 1024 * 1024 * 100);
    assert_eq!(metrics.eviction_rate, 5.2);
}

#[test]
fn test_cache_metrics_clone() {
    let metrics = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100,
        eviction_rate: 5.2,
    };
    
    let cloned = metrics.clone();
    assert_eq!(cloned.hit_rate, metrics.hit_rate);
    assert_eq!(cloned.miss_rate, metrics.miss_rate);
    assert_eq!(cloned.avg_operation_time, metrics.avg_operation_time);
    assert_eq!(cloned.max_operation_time, metrics.max_operation_time);
    assert_eq!(cloned.total_operations, metrics.total_operations);
    assert_eq!(cloned.operations_per_second, metrics.operations_per_second);
    assert_eq!(cloned.current_size, metrics.current_size);
    assert_eq!(cloned.eviction_rate, metrics.eviction_rate);
}

#[test]
fn test_cache_metrics_debug() {
    let metrics = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100,
        eviction_rate: 5.2,
    };
    
    let debug_str = format!("{:?}", metrics);
    assert!(debug_str.contains("CacheMetrics"));
    assert!(debug_str.contains("hit_rate: 85.5"));
    assert!(debug_str.contains("total_operations: 10000"));
}

#[test]
fn test_system_metrics_creation() {
    let metrics = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512, // 512 MB
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600, // 1 hour
        load_average: 1.5,
    };
    
    assert_eq!(metrics.cpu_usage, 75.5);
    assert_eq!(metrics.memory_usage, 1024 * 1024 * 512);
    assert_eq!(metrics.memory_usage_percent, 50.0);
    assert_eq!(metrics.thread_count, 8);
    assert_eq!(metrics.uptime, 3600);
    assert_eq!(metrics.load_average, 1.5);
}

#[test]
fn test_system_metrics_clone() {
    let metrics = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600,
        load_average: 1.5,
    };
    
    let cloned = metrics.clone();
    assert_eq!(cloned.cpu_usage, metrics.cpu_usage);
    assert_eq!(cloned.memory_usage, metrics.memory_usage);
    assert_eq!(cloned.memory_usage_percent, metrics.memory_usage_percent);
    assert_eq!(cloned.thread_count, metrics.thread_count);
    assert_eq!(cloned.uptime, metrics.uptime);
    assert_eq!(cloned.load_average, metrics.load_average);
}

#[test]
fn test_system_metrics_debug() {
    let metrics = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600,
        load_average: 1.5,
    };
    
    let debug_str = format!("{:?}", metrics);
    assert!(debug_str.contains("SystemMetrics"));
    assert!(debug_str.contains("cpu_usage: 75.5"));
    assert!(debug_str.contains("thread_count: 8"));
}

#[test]
fn test_error_metrics_creation() {
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    error_types.insert("timeout".to_string(), 3);
    error_types.insert("memory".to_string(), 2);
    
    let metrics = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types: error_types.clone(),
        last_error_time: Some(1234567890),
    };
    
    assert_eq!(metrics.total_errors, 10);
    assert_eq!(metrics.error_rate, 0.1);
    assert_eq!(metrics.errors_per_second, 0.5);
    assert_eq!(metrics.error_types.len(), 3);
    assert_eq!(metrics.error_types.get("validation"), Some(&5));
    assert_eq!(metrics.error_types.get("timeout"), Some(&3));
    assert_eq!(metrics.error_types.get("memory"), Some(&2));
    assert_eq!(metrics.last_error_time, Some(1234567890));
}

#[test]
fn test_error_metrics_clone() {
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    error_types.insert("timeout".to_string(), 3);
    
    let metrics = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types: error_types.clone(),
        last_error_time: Some(1234567890),
    };
    
    let cloned = metrics.clone();
    assert_eq!(cloned.total_errors, metrics.total_errors);
    assert_eq!(cloned.error_rate, metrics.error_rate);
    assert_eq!(cloned.errors_per_second, metrics.errors_per_second);
    assert_eq!(cloned.error_types.len(), metrics.error_types.len());
    assert_eq!(cloned.last_error_time, metrics.last_error_time);
}

#[test]
fn test_error_metrics_debug() {
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    
    let metrics = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types,
        last_error_time: Some(1234567890),
    };
    
    let debug_str = format!("{:?}", metrics);
    assert!(debug_str.contains("ErrorMetrics"));
    assert!(debug_str.contains("total_errors: 10"));
    assert!(debug_str.contains("error_rate: 0.1"));
}

#[test]
fn test_performance_metrics_creation() {
    let aggregation = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    let cache = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100,
        eviction_rate: 5.2,
    };
    
    let system = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600,
        load_average: 1.5,
    };
    
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    
    let errors = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types,
        last_error_time: Some(1234567890),
    };
    
    let metrics = PerformanceMetrics {
        aggregation,
        cache,
        system,
        errors,
        last_updated: 1234567890,
    };
    
    assert_eq!(metrics.aggregation.avg_aggregation_time, 1000);
    assert_eq!(metrics.cache.hit_rate, 85.5);
    assert_eq!(metrics.system.cpu_usage, 75.5);
    assert_eq!(metrics.errors.total_errors, 10);
    assert_eq!(metrics.last_updated, 1234567890);
}

#[test]
fn test_performance_metrics_clone() {
    let aggregation = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    let cache = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100,
        eviction_rate: 5.2,
    };
    
    let system = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600,
        load_average: 1.5,
    };
    
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    
    let errors = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types,
        last_error_time: Some(1234567890),
    };
    
    let metrics = PerformanceMetrics {
        aggregation,
        cache,
        system,
        errors,
        last_updated: 1234567890,
    };
    
    let cloned = metrics.clone();
    assert_eq!(cloned.aggregation.avg_aggregation_time, metrics.aggregation.avg_aggregation_time);
    assert_eq!(cloned.cache.hit_rate, metrics.cache.hit_rate);
    assert_eq!(cloned.system.cpu_usage, metrics.system.cpu_usage);
    assert_eq!(cloned.errors.total_errors, metrics.errors.total_errors);
    assert_eq!(cloned.last_updated, metrics.last_updated);
}

#[test]
fn test_performance_metrics_debug() {
    let aggregation = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    let cache = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100,
        eviction_rate: 5.2,
    };
    
    let system = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600,
        load_average: 1.5,
    };
    
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    
    let errors = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types,
        last_error_time: Some(1234567890),
    };
    
    let metrics = PerformanceMetrics {
        aggregation,
        cache,
        system,
        errors,
        last_updated: 1234567890,
    };
    
    let debug_str = format!("{:?}", metrics);
    assert!(debug_str.contains("PerformanceMetrics"));
    assert!(debug_str.contains("last_updated: 1234567890"));
}

#[test]
fn test_violation_severity_variants() {
    let warning = ViolationSeverity::Warning;
    let critical = ViolationSeverity::Critical;
    let fatal = ViolationSeverity::Fatal;
    
    assert_eq!(warning, ViolationSeverity::Warning);
    assert_eq!(critical, ViolationSeverity::Critical);
    assert_eq!(fatal, ViolationSeverity::Fatal);
    assert_ne!(warning, critical);
    assert_ne!(critical, fatal);
    assert_ne!(warning, fatal);
}

#[test]
fn test_violation_severity_clone() {
    let warning = ViolationSeverity::Warning;
    let cloned = warning.clone();
    assert_eq!(cloned, warning);
}

#[test]
fn test_violation_severity_debug() {
    let warning = ViolationSeverity::Warning;
    let critical = ViolationSeverity::Critical;
    let fatal = ViolationSeverity::Fatal;
    
    let warning_str = format!("{:?}", warning);
    let critical_str = format!("{:?}", critical);
    let fatal_str = format!("{:?}", fatal);
    
    assert!(warning_str.contains("Warning"));
    assert!(critical_str.contains("Critical"));
    assert!(fatal_str.contains("Fatal"));
}

#[test]
fn test_threshold_violation_creation() {
    let violation = ThresholdViolation {
        threshold_name: "max_aggregation_time".to_string(),
        actual_value: 1500.0,
        threshold_value: 1000.0,
        severity: ViolationSeverity::Warning,
        timestamp: 1234567890,
    };
    
    assert_eq!(violation.threshold_name, "max_aggregation_time");
    assert_eq!(violation.actual_value, 1500.0);
    assert_eq!(violation.threshold_value, 1000.0);
    assert_eq!(violation.severity, ViolationSeverity::Warning);
    assert_eq!(violation.timestamp, 1234567890);
}

#[test]
fn test_threshold_violation_clone() {
    let violation = ThresholdViolation {
        threshold_name: "max_aggregation_time".to_string(),
        actual_value: 1500.0,
        threshold_value: 1000.0,
        severity: ViolationSeverity::Warning,
        timestamp: 1234567890,
    };
    
    let cloned = violation.clone();
    assert_eq!(cloned.threshold_name, violation.threshold_name);
    assert_eq!(cloned.actual_value, violation.actual_value);
    assert_eq!(cloned.threshold_value, violation.threshold_value);
    assert_eq!(cloned.severity, violation.severity);
    assert_eq!(cloned.timestamp, violation.timestamp);
}

#[test]
fn test_threshold_violation_debug() {
    let violation = ThresholdViolation {
        threshold_name: "max_aggregation_time".to_string(),
        actual_value: 1500.0,
        threshold_value: 1000.0,
        severity: ViolationSeverity::Warning,
        timestamp: 1234567890,
    };
    
    let debug_str = format!("{:?}", violation);
    assert!(debug_str.contains("ThresholdViolation"));
    assert!(debug_str.contains("threshold_name: \"max_aggregation_time\""));
    assert!(debug_str.contains("actual_value: 1500.0"));
}

#[test]
fn test_performance_snapshot_creation() {
    let aggregation = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    let cache = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100,
        eviction_rate: 5.2,
    };
    
    let system = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600,
        load_average: 1.5,
    };
    
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    
    let errors = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types,
        last_error_time: Some(1234567890),
    };
    
    let metrics = PerformanceMetrics {
        aggregation,
        cache,
        system,
        errors,
        last_updated: 1234567890,
    };
    
    let violation = ThresholdViolation {
        threshold_name: "max_aggregation_time".to_string(),
        actual_value: 1500.0,
        threshold_value: 1000.0,
        severity: ViolationSeverity::Warning,
        timestamp: 1234567890,
    };
    
    let snapshot = PerformanceSnapshot {
        timestamp: 1234567890,
        metrics,
        violations: vec![violation],
        performance_score: 85.5,
    };
    
    assert_eq!(snapshot.timestamp, 1234567890);
    assert_eq!(snapshot.metrics.aggregation.avg_aggregation_time, 1000);
    assert_eq!(snapshot.violations.len(), 1);
    assert_eq!(snapshot.performance_score, 85.5);
}

#[test]
fn test_performance_snapshot_clone() {
    let aggregation = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    let cache = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100,
        eviction_rate: 5.2,
    };
    
    let system = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600,
        load_average: 1.5,
    };
    
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    
    let errors = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types,
        last_error_time: Some(1234567890),
    };
    
    let metrics = PerformanceMetrics {
        aggregation,
        cache,
        system,
        errors,
        last_updated: 1234567890,
    };
    
    let violation = ThresholdViolation {
        threshold_name: "max_aggregation_time".to_string(),
        actual_value: 1500.0,
        threshold_value: 1000.0,
        severity: ViolationSeverity::Warning,
        timestamp: 1234567890,
    };
    
    let snapshot = PerformanceSnapshot {
        timestamp: 1234567890,
        metrics,
        violations: vec![violation],
        performance_score: 85.5,
    };
    
    let cloned = snapshot.clone();
    assert_eq!(cloned.timestamp, snapshot.timestamp);
    assert_eq!(cloned.metrics.aggregation.avg_aggregation_time, snapshot.metrics.aggregation.avg_aggregation_time);
    assert_eq!(cloned.violations.len(), snapshot.violations.len());
    assert_eq!(cloned.performance_score, snapshot.performance_score);
}

#[test]
fn test_performance_snapshot_debug() {
    let aggregation = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    let cache = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100,
        eviction_rate: 5.2,
    };
    
    let system = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600,
        load_average: 1.5,
    };
    
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    
    let errors = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types,
        last_error_time: Some(1234567890),
    };
    
    let metrics = PerformanceMetrics {
        aggregation,
        cache,
        system,
        errors,
        last_updated: 1234567890,
    };
    
    let snapshot = PerformanceSnapshot {
        timestamp: 1234567890,
        metrics,
        violations: vec![],
        performance_score: 85.5,
    };
    
    let debug_str = format!("{:?}", snapshot);
    assert!(debug_str.contains("PerformanceSnapshot"));
    assert!(debug_str.contains("timestamp: 1234567890"));
    assert!(debug_str.contains("performance_score: 85.5"));
}

#[test]
fn test_performance_test_result_creation() {
    let aggregation = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    let cache = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100,
        eviction_rate: 5.2,
    };
    
    let system = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600,
        load_average: 1.5,
    };
    
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    
    let errors = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types,
        last_error_time: Some(1234567890),
    };
    
    let metrics = PerformanceMetrics {
        aggregation,
        cache,
        system,
        errors,
        last_updated: 1234567890,
    };
    
    let violation = ThresholdViolation {
        threshold_name: "max_aggregation_time".to_string(),
        actual_value: 1500.0,
        threshold_value: 1000.0,
        severity: ViolationSeverity::Warning,
        timestamp: 1234567890,
    };
    
    let result = PerformanceTestResult {
        test_name: "aggregation_performance".to_string(),
        passed: true,
        score: 85.5,
        violations: vec![violation],
        metrics,
        duration: Duration::from_secs(30),
        recommendations: vec!["Optimize aggregation algorithm".to_string()],
    };
    
    assert_eq!(result.test_name, "aggregation_performance");
    assert!(result.passed);
    assert_eq!(result.score, 85.5);
    assert_eq!(result.violations.len(), 1);
    assert_eq!(result.metrics.aggregation.avg_aggregation_time, 1000);
    assert_eq!(result.duration, Duration::from_secs(30));
    assert_eq!(result.recommendations.len(), 1);
    assert_eq!(result.recommendations[0], "Optimize aggregation algorithm");
}

#[test]
fn test_performance_test_result_clone() {
    let aggregation = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    let cache = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100,
        eviction_rate: 5.2,
    };
    
    let system = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600,
        load_average: 1.5,
    };
    
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    
    let errors = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types,
        last_error_time: Some(1234567890),
    };
    
    let metrics = PerformanceMetrics {
        aggregation,
        cache,
        system,
        errors,
        last_updated: 1234567890,
    };
    
    let result = PerformanceTestResult {
        test_name: "aggregation_performance".to_string(),
        passed: true,
        score: 85.5,
        violations: vec![],
        metrics,
        duration: Duration::from_secs(30),
        recommendations: vec!["Optimize aggregation algorithm".to_string()],
    };
    
    let cloned = result.clone();
    assert_eq!(cloned.test_name, result.test_name);
    assert_eq!(cloned.passed, result.passed);
    assert_eq!(cloned.score, result.score);
    assert_eq!(cloned.violations.len(), result.violations.len());
    assert_eq!(cloned.metrics.aggregation.avg_aggregation_time, result.metrics.aggregation.avg_aggregation_time);
    assert_eq!(cloned.duration, result.duration);
    assert_eq!(cloned.recommendations.len(), result.recommendations.len());
}

#[test]
fn test_performance_test_result_debug() {
    let aggregation = AggregationMetrics {
        avg_aggregation_time: 1000,
        max_aggregation_time: 2000,
        min_aggregation_time: 500,
        total_aggregations: 1000,
        aggregations_per_second: 10.5,
        avg_contributions_per_aggregation: 5.2,
        avg_subsystems_per_aggregation: 3.1,
    };
    
    let cache = CacheMetrics {
        hit_rate: 85.5,
        miss_rate: 14.5,
        avg_operation_time: 100,
        max_operation_time: 500,
        total_operations: 10000,
        operations_per_second: 100.0,
        current_size: 1024 * 1024 * 100,
        eviction_rate: 5.2,
    };
    
    let system = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        memory_usage_percent: 50.0,
        thread_count: 8,
        uptime: 3600,
        load_average: 1.5,
    };
    
    let mut error_types = HashMap::new();
    error_types.insert("validation".to_string(), 5);
    
    let errors = ErrorMetrics {
        total_errors: 10,
        error_rate: 0.1,
        errors_per_second: 0.5,
        error_types,
        last_error_time: Some(1234567890),
    };
    
    let metrics = PerformanceMetrics {
        aggregation,
        cache,
        system,
        errors,
        last_updated: 1234567890,
    };
    
    let result = PerformanceTestResult {
        test_name: "aggregation_performance".to_string(),
        passed: true,
        score: 85.5,
        violations: vec![],
        metrics,
        duration: Duration::from_secs(30),
        recommendations: vec!["Optimize aggregation algorithm".to_string()],
    };
    
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("PerformanceTestResult"));
    assert!(debug_str.contains("test_name: \"aggregation_performance\""));
    assert!(debug_str.contains("passed: true"));
    assert!(debug_str.contains("score: 85.5"));
}

#[test]
fn test_serialization_deserialization() {
    let config = ProfilerConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: ProfilerConfig = serde_json::from_str(&json).unwrap();
    
    assert_eq!(config.enable_continuous_profiling, deserialized.enable_continuous_profiling);
    assert_eq!(config.profiling_interval, deserialized.profiling_interval);
    assert_eq!(config.max_history_size, deserialized.max_history_size);
    
    let thresholds = PerformanceThresholds::default();
    let json = serde_json::to_string(&thresholds).unwrap();
    let deserialized: PerformanceThresholds = serde_json::from_str(&json).unwrap();
    
    assert_eq!(thresholds.max_aggregation_time, deserialized.max_aggregation_time);
    assert_eq!(thresholds.min_throughput, deserialized.min_throughput);
    
    let violation = ThresholdViolation {
        threshold_name: "max_aggregation_time".to_string(),
        actual_value: 1500.0,
        threshold_value: 1000.0,
        severity: ViolationSeverity::Warning,
        timestamp: 1234567890,
    };
    
    let json = serde_json::to_string(&violation).unwrap();
    let deserialized: ThresholdViolation = serde_json::from_str(&json).unwrap();
    
    assert_eq!(violation.threshold_name, deserialized.threshold_name);
    assert_eq!(violation.actual_value, deserialized.actual_value);
    assert_eq!(violation.severity, deserialized.severity);
}
