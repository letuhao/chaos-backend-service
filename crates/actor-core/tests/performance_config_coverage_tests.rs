//! Coverage tests for performance config module.

use actor_core::performance::config::*;
use std::time::Duration;

#[test]
fn test_performance_config_default() {
    let config = PerformanceConfig::default();
    assert!(config.general.enable_monitoring);
    assert!(config.general.enable_profiling);
    assert!(config.general.enable_benchmarking);
    assert_eq!(config.general.monitoring_interval, Duration::from_secs(5));
    assert_eq!(config.general.max_degradation, 10.0);
    assert_eq!(config.general.optimization_level, 2);
}

#[test]
fn test_performance_config_creation() {
    let config = PerformanceConfig {
        general: GeneralPerformanceConfig {
            enable_monitoring: false,
            enable_profiling: false,
            enable_benchmarking: false,
            monitoring_interval: Duration::from_secs(30),
            max_degradation: 5.0,
            optimization_level: 1,
        },
        aggregation: AggregationPerformanceConfig::default(),
        cache: CachePerformanceConfig::default(),
        memory: MemoryPerformanceConfig::default(),
        system: SystemPerformanceConfig::default(),
        monitoring: MonitoringConfig::default(),
        benchmarking: BenchmarkingConfig::default(),
        alerting: AlertingConfig::default(),
    };
    
    assert!(!config.general.enable_monitoring);
    assert!(!config.general.enable_profiling);
    assert!(!config.general.enable_benchmarking);
    assert_eq!(config.general.monitoring_interval, Duration::from_secs(30));
    assert_eq!(config.general.max_degradation, 5.0);
    assert_eq!(config.general.optimization_level, 1);
}

#[test]
fn test_performance_config_clone() {
    let config1 = PerformanceConfig::default();
    let config2 = config1.clone();
    
    assert_eq!(config1.general.enable_monitoring, config2.general.enable_monitoring);
    assert_eq!(config1.general.enable_profiling, config2.general.enable_profiling);
    assert_eq!(config1.general.enable_benchmarking, config2.general.enable_benchmarking);
    assert_eq!(config1.general.monitoring_interval, config2.general.monitoring_interval);
    assert_eq!(config1.general.max_degradation, config2.general.max_degradation);
    assert_eq!(config1.general.optimization_level, config2.general.optimization_level);
}

#[test]
fn test_performance_config_debug() {
    let config = PerformanceConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("PerformanceConfig"));
    assert!(debug_str.contains("general:"));
    assert!(debug_str.contains("aggregation:"));
    assert!(debug_str.contains("cache:"));
    assert!(debug_str.contains("memory:"));
    assert!(debug_str.contains("system:"));
    assert!(debug_str.contains("monitoring:"));
    assert!(debug_str.contains("benchmarking:"));
    assert!(debug_str.contains("alerting:"));
}

#[test]
fn test_performance_config_serialization() {
    let config = PerformanceConfig::default();
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: PerformanceConfig = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(config.general.enable_monitoring, deserialized.general.enable_monitoring);
    assert_eq!(config.general.enable_profiling, deserialized.general.enable_profiling);
    assert_eq!(config.general.enable_benchmarking, deserialized.general.enable_benchmarking);
    assert_eq!(config.general.monitoring_interval, deserialized.general.monitoring_interval);
    assert_eq!(config.general.max_degradation, deserialized.general.max_degradation);
    assert_eq!(config.general.optimization_level, deserialized.general.optimization_level);
}

#[test]
fn test_general_performance_config_default() {
    let config = GeneralPerformanceConfig::default();
    assert!(config.enable_monitoring);
    assert!(config.enable_profiling);
    assert!(config.enable_benchmarking);
    assert_eq!(config.monitoring_interval, Duration::from_secs(5));
    assert_eq!(config.max_degradation, 10.0);
    assert_eq!(config.optimization_level, 2);
}

#[test]
fn test_general_performance_config_creation() {
    let config = GeneralPerformanceConfig {
        enable_monitoring: false,
        enable_profiling: false,
        enable_benchmarking: false,
        monitoring_interval: Duration::from_secs(30),
        max_degradation: 5.0,
        optimization_level: 1,
    };
    
    assert!(!config.enable_monitoring);
    assert!(!config.enable_profiling);
    assert!(!config.enable_benchmarking);
    assert_eq!(config.monitoring_interval, Duration::from_secs(30));
    assert_eq!(config.max_degradation, 5.0);
    assert_eq!(config.optimization_level, 1);
}

#[test]
fn test_general_performance_config_clone() {
    let config1 = GeneralPerformanceConfig::default();
    let config2 = config1.clone();
    
    assert_eq!(config1.enable_monitoring, config2.enable_monitoring);
    assert_eq!(config1.enable_profiling, config2.enable_profiling);
    assert_eq!(config1.enable_benchmarking, config2.enable_benchmarking);
    assert_eq!(config1.monitoring_interval, config2.monitoring_interval);
    assert_eq!(config1.max_degradation, config2.max_degradation);
    assert_eq!(config1.optimization_level, config2.optimization_level);
}

#[test]
fn test_general_performance_config_debug() {
    let config = GeneralPerformanceConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("GeneralPerformanceConfig"));
    assert!(debug_str.contains("enable_monitoring: true"));
    assert!(debug_str.contains("enable_profiling: true"));
    assert!(debug_str.contains("enable_benchmarking: true"));
    assert!(debug_str.contains("monitoring_interval:"));
    assert!(debug_str.contains("max_degradation: 10.0"));
    assert!(debug_str.contains("optimization_level: 2"));
}

#[test]
fn test_general_performance_config_serialization() {
    let config = GeneralPerformanceConfig::default();
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: GeneralPerformanceConfig = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(config.enable_monitoring, deserialized.enable_monitoring);
    assert_eq!(config.enable_profiling, deserialized.enable_profiling);
    assert_eq!(config.enable_benchmarking, deserialized.enable_benchmarking);
    assert_eq!(config.monitoring_interval, deserialized.monitoring_interval);
    assert_eq!(config.max_degradation, deserialized.max_degradation);
    assert_eq!(config.optimization_level, deserialized.optimization_level);
}

#[test]
fn test_aggregation_performance_config_default() {
    let config = AggregationPerformanceConfig::default();
    assert_eq!(config.max_aggregation_time, 1000000); // 1 second in microseconds
    assert_eq!(config.max_contributions_per_aggregation, 1000);
    assert_eq!(config.max_subsystems_per_aggregation, 100);
    assert_eq!(config.min_throughput, 1000);
    assert_eq!(config.max_latency, 10000); // 10ms in microseconds
    assert_eq!(config.enable_parallel_aggregation, true);
    assert_eq!(config.parallel_threshold, 50);
    assert_eq!(config.enable_aggregation_caching, true);
    assert_eq!(config.aggregation_cache_ttl, 300); // 300 seconds
}

#[test]
fn test_aggregation_performance_config_creation() {
    let config = AggregationPerformanceConfig {
        max_aggregation_time: 500000, // 0.5 seconds in microseconds
        max_contributions_per_aggregation: 500,
        max_subsystems_per_aggregation: 5,
        min_throughput: 500,
        max_latency: 50000, // 50ms in microseconds
        enable_parallel_aggregation: false,
        parallel_threshold: 50,
        enable_aggregation_caching: false,
        aggregation_cache_ttl: 150, // 150 seconds
    };
    
    assert_eq!(config.max_aggregation_time, 500000);
    assert_eq!(config.max_contributions_per_aggregation, 500);
    assert_eq!(config.max_subsystems_per_aggregation, 5);
    assert_eq!(config.min_throughput, 500);
    assert_eq!(config.max_latency, 50000);
    assert_eq!(config.enable_parallel_aggregation, false);
    assert_eq!(config.parallel_threshold, 50);
    assert_eq!(config.enable_aggregation_caching, false);
    assert_eq!(config.aggregation_cache_ttl, 150);
}

#[test]
fn test_aggregation_performance_config_clone() {
    let config1 = AggregationPerformanceConfig::default();
    let config2 = config1.clone();
    
    assert_eq!(config1.max_aggregation_time, config2.max_aggregation_time);
    assert_eq!(config1.max_contributions_per_aggregation, config2.max_contributions_per_aggregation);
    assert_eq!(config1.max_subsystems_per_aggregation, config2.max_subsystems_per_aggregation);
    assert_eq!(config1.min_throughput, config2.min_throughput);
    assert_eq!(config1.max_latency, config2.max_latency);
    assert_eq!(config1.enable_parallel_aggregation, config2.enable_parallel_aggregation);
    assert_eq!(config1.parallel_threshold, config2.parallel_threshold);
    assert_eq!(config1.enable_aggregation_caching, config2.enable_aggregation_caching);
    assert_eq!(config1.aggregation_cache_ttl, config2.aggregation_cache_ttl);
}

#[test]
fn test_aggregation_performance_config_debug() {
    let config = AggregationPerformanceConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("AggregationPerformanceConfig"));
    assert!(debug_str.contains("max_aggregation_time: 1000000"));
    assert!(debug_str.contains("max_contributions_per_aggregation: 1000"));
    assert!(debug_str.contains("max_subsystems_per_aggregation: 100"));
    assert!(debug_str.contains("min_throughput: 1000"));
    assert!(debug_str.contains("max_latency: 10000"));
    assert!(debug_str.contains("enable_parallel_aggregation: true"));
    assert!(debug_str.contains("parallel_threshold: 50"));
    assert!(debug_str.contains("enable_aggregation_caching: true"));
    assert!(debug_str.contains("aggregation_cache_ttl: 300"));
}

#[test]
fn test_aggregation_performance_config_serialization() {
    let config = AggregationPerformanceConfig::default();
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: AggregationPerformanceConfig = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(config.max_aggregation_time, deserialized.max_aggregation_time);
    assert_eq!(config.max_contributions_per_aggregation, deserialized.max_contributions_per_aggregation);
    assert_eq!(config.max_subsystems_per_aggregation, deserialized.max_subsystems_per_aggregation);
    assert_eq!(config.min_throughput, deserialized.min_throughput);
    assert_eq!(config.max_latency, deserialized.max_latency);
    assert_eq!(config.enable_parallel_aggregation, deserialized.enable_parallel_aggregation);
    assert_eq!(config.parallel_threshold, deserialized.parallel_threshold);
    assert_eq!(config.enable_aggregation_caching, deserialized.enable_aggregation_caching);
    assert_eq!(config.aggregation_cache_ttl, deserialized.aggregation_cache_ttl);
}