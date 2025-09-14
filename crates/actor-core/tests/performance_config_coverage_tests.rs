//! Performance configuration coverage tests for Actor Core.

use actor_core::performance::config::{PerformanceConfig, PerformanceConfigLoader};
use std::time::Duration;

#[test]
fn test_performance_config_default() {
    let config = PerformanceConfig::default();
    
    // Test general config
    assert!(config.general.enable_monitoring);
    assert!(config.general.enable_profiling);
    assert!(config.general.enable_benchmarking);
    assert_eq!(config.general.monitoring_interval, Duration::from_secs(5));
    assert_eq!(config.general.max_degradation, 10.0);
    assert_eq!(config.general.optimization_level, 2);
    
    // Test aggregation config
    assert!(config.aggregation.max_aggregation_time > 0);
    assert!(config.aggregation.max_contributions_per_aggregation > 0);
    assert!(config.aggregation.max_subsystems_per_aggregation > 0);
    assert!(config.aggregation.min_throughput > 0);
    assert!(config.aggregation.max_latency > 0);
    assert!(config.aggregation.enable_parallel_aggregation);
    assert!(config.aggregation.parallel_threshold > 0);
    assert!(config.aggregation.enable_aggregation_caching);
    assert!(config.aggregation.aggregation_cache_ttl > 0);
    
    // Test cache config
    assert!(config.cache.max_operation_time > 0);
    assert!(config.cache.min_hit_rate > 0.0);
    assert!(config.cache.max_cache_size > 0);
    assert!(config.cache.eviction_threshold > 0.0);
    assert!(config.cache.enable_cache_warming);
    assert!(config.cache.warming_interval > Duration::from_secs(0));
    assert!(config.cache.enable_compression);
    assert!(config.cache.compression_threshold > 0);
    
    // Test memory config
    assert!(config.memory.max_memory_per_actor > 0);
    assert!(config.memory.max_total_memory > 0);
    assert!(config.memory.warning_threshold > 0.0);
    assert!(config.memory.critical_threshold > 0.0);
    assert!(config.memory.enable_memory_pooling);
    assert!(config.memory.memory_pool_size > 0);
    assert!(config.memory.enable_memory_compaction);
    assert!(config.memory.compaction_threshold > 0.0);
    
    // Test system config
    assert!(config.system.max_cpu_usage > 0.0);
    assert!(config.system.cpu_warning_threshold > 0.0);
    assert!(config.system.max_thread_count > 0);
    assert!(config.system.thread_pool_size > 0);
    assert!(!config.system.enable_cpu_affinity);
    assert!(!config.system.enable_numa_awareness);
    assert!(config.system.load_warning_threshold > 0.0);
    assert!(config.system.load_critical_threshold > 0.0);
    
    // Test monitoring config
    assert!(config.monitoring.enable_realtime_monitoring);
    assert!(config.monitoring.enable_historical_monitoring);
    assert!(config.monitoring.data_retention_days > 0);
    assert!(config.monitoring.enable_metrics_export);
    assert!(config.monitoring.export_interval > Duration::from_secs(0));
    assert!(config.monitoring.enable_dashboards);
    assert!(config.monitoring.dashboard_refresh_interval > Duration::from_secs(0));
    
    // Test benchmarking config
    assert!(config.benchmarking.enable_automated_benchmarking);
    assert!(config.benchmarking.benchmarking_interval > Duration::from_secs(0));
    assert!(config.benchmarking.benchmark_duration > Duration::from_secs(0));
    assert!(config.benchmarking.benchmark_iterations > 0);
    assert!(config.benchmarking.enable_stress_testing);
    assert!(config.benchmarking.stress_test_duration > Duration::from_secs(0));
    assert!(config.benchmarking.stress_load_multiplier > 0.0);
    assert!(config.benchmarking.enable_regression_testing);
    assert!(config.benchmarking.regression_threshold > 0.0);
    
    // Test alerting config
    assert!(config.alerting.enable_alerts);
    assert!(config.alerting.severity_levels.warning_threshold > 0.0);
    assert!(config.alerting.severity_levels.critical_threshold > 0.0);
    assert!(config.alerting.severity_levels.fatal_threshold > 0.0);
    assert!(config.alerting.channels.enable_console);
    assert!(config.alerting.channels.enable_file);
    assert!(!config.alerting.channels.enable_email);
    assert!(!config.alerting.channels.enable_webhook);
    assert!(config.alerting.throttling.enable_throttling);
    assert!(config.alerting.throttling.max_alerts_per_minute > 0);
    assert!(config.alerting.throttling.cooldown_period > Duration::from_secs(0));
    assert!(config.alerting.throttling.enable_grouping);
    assert!(config.alerting.throttling.group_timeout > Duration::from_secs(0));
    assert!(config.alerting.escalation.enable_escalation);
    assert!(config.alerting.escalation.escalation_delay > Duration::from_secs(0));
    assert!(config.alerting.escalation.max_escalation_levels > 0);
    assert!(!config.alerting.escalation.escalation_recipients.is_empty());
}

#[test]
fn test_performance_config_validation() {
    let config = PerformanceConfig::default();
    
    // Test valid configuration
    assert!(PerformanceConfigLoader::validate(&config).is_ok());
    
    // Test invalid configuration - max_degradation out of range
    let mut invalid_config = config.clone();
    invalid_config.general.max_degradation = 150.0;
    assert!(PerformanceConfigLoader::validate(&invalid_config).is_err());
    
    // Test invalid configuration - optimization_level out of range
    let mut invalid_config = config.clone();
    invalid_config.general.optimization_level = 5;
    assert!(PerformanceConfigLoader::validate(&invalid_config).is_err());
    
    // Test invalid configuration - max_contributions_per_aggregation is 0
    let mut invalid_config = config.clone();
    invalid_config.aggregation.max_contributions_per_aggregation = 0;
    assert!(PerformanceConfigLoader::validate(&invalid_config).is_err());
    
    // Test invalid configuration - min_hit_rate out of range
    let mut invalid_config = config.clone();
    invalid_config.cache.min_hit_rate = 150.0;
    assert!(PerformanceConfigLoader::validate(&invalid_config).is_err());
    
    // Test invalid configuration - critical_threshold < warning_threshold
    let mut invalid_config = config.clone();
    invalid_config.memory.critical_threshold = 50.0;
    invalid_config.memory.warning_threshold = 75.0;
    assert!(PerformanceConfigLoader::validate(&invalid_config).is_err());
    
    // Test invalid configuration - max_cpu_usage out of range
    let mut invalid_config = config.clone();
    invalid_config.system.max_cpu_usage = 150.0;
    assert!(PerformanceConfigLoader::validate(&invalid_config).is_err());
    
    // Test invalid configuration - cpu_warning_threshold > max_cpu_usage
    let mut invalid_config = config.clone();
    invalid_config.system.cpu_warning_threshold = 90.0;
    invalid_config.system.max_cpu_usage = 80.0;
    assert!(PerformanceConfigLoader::validate(&invalid_config).is_err());
    
    // Test invalid configuration - warning_threshold out of range
    let mut invalid_config = config.clone();
    invalid_config.alerting.severity_levels.warning_threshold = 150.0;
    assert!(PerformanceConfigLoader::validate(&invalid_config).is_err());
    
    // Test invalid configuration - critical_threshold > warning_threshold
    let mut invalid_config = config.clone();
    invalid_config.alerting.severity_levels.critical_threshold = 90.0;
    invalid_config.alerting.severity_levels.warning_threshold = 80.0;
    assert!(PerformanceConfigLoader::validate(&invalid_config).is_err());
    
    // Test invalid configuration - fatal_threshold > critical_threshold
    let mut invalid_config = config.clone();
    invalid_config.alerting.severity_levels.fatal_threshold = 70.0;
    invalid_config.alerting.severity_levels.critical_threshold = 60.0;
    assert!(PerformanceConfigLoader::validate(&invalid_config).is_err());
}

#[test]
fn test_performance_config_loader_env() {
    // Test loading from environment variables
    std::env::set_var("ACTOR_CORE_ENABLE_MONITORING", "false");
    std::env::set_var("ACTOR_CORE_MONITORING_INTERVAL", "10");
    std::env::set_var("ACTOR_CORE_MAX_AGGREGATION_TIME", "5000");
    std::env::set_var("ACTOR_CORE_MIN_THROUGHPUT", "2000");
    std::env::set_var("ACTOR_CORE_MAX_CACHE_TIME", "1000");
    std::env::set_var("ACTOR_CORE_MIN_CACHE_HIT_RATE", "95.0");
    std::env::set_var("ACTOR_CORE_MAX_MEMORY_PER_ACTOR", "2097152");
    std::env::set_var("ACTOR_CORE_MAX_CPU_USAGE", "90.0");
    
    let config = PerformanceConfigLoader::load_from_env();
    
    assert!(!config.general.enable_monitoring);
    assert_eq!(config.general.monitoring_interval, Duration::from_secs(10));
    assert_eq!(config.aggregation.max_aggregation_time, 5000);
    assert_eq!(config.aggregation.min_throughput, 2000);
    assert_eq!(config.cache.max_operation_time, 1000);
    assert_eq!(config.cache.min_hit_rate, 95.0);
    assert_eq!(config.memory.max_memory_per_actor, 2097152);
    assert_eq!(config.system.max_cpu_usage, 90.0);
    
    // Clean up environment variables
    std::env::remove_var("ACTOR_CORE_ENABLE_MONITORING");
    std::env::remove_var("ACTOR_CORE_MONITORING_INTERVAL");
    std::env::remove_var("ACTOR_CORE_MAX_AGGREGATION_TIME");
    std::env::remove_var("ACTOR_CORE_MIN_THROUGHPUT");
    std::env::remove_var("ACTOR_CORE_MAX_CACHE_TIME");
    std::env::remove_var("ACTOR_CORE_MIN_CACHE_HIT_RATE");
    std::env::remove_var("ACTOR_CORE_MAX_MEMORY_PER_ACTOR");
    std::env::remove_var("ACTOR_CORE_MAX_CPU_USAGE");
}

#[test]
fn test_performance_config_serialization() {
    let config = PerformanceConfig::default();
    
    // Test JSON serialization
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: PerformanceConfig = serde_json::from_str(&json).unwrap();
    
    assert_eq!(config.general.enable_monitoring, deserialized.general.enable_monitoring);
    assert_eq!(config.general.enable_profiling, deserialized.general.enable_profiling);
    assert_eq!(config.general.enable_benchmarking, deserialized.general.enable_benchmarking);
    assert_eq!(config.general.monitoring_interval, deserialized.general.monitoring_interval);
    assert_eq!(config.general.max_degradation, deserialized.general.max_degradation);
    assert_eq!(config.general.optimization_level, deserialized.general.optimization_level);
    
    // Test YAML serialization
    let yaml = serde_yaml::to_string(&config).unwrap();
    let deserialized_yaml: PerformanceConfig = serde_yaml::from_str(&yaml).unwrap();
    
    assert_eq!(config.general.enable_monitoring, deserialized_yaml.general.enable_monitoring);
    assert_eq!(config.general.enable_profiling, deserialized_yaml.general.enable_profiling);
    assert_eq!(config.general.enable_benchmarking, deserialized_yaml.general.enable_benchmarking);
    assert_eq!(config.general.monitoring_interval, deserialized_yaml.general.monitoring_interval);
    assert_eq!(config.general.max_degradation, deserialized_yaml.general.max_degradation);
    assert_eq!(config.general.optimization_level, deserialized_yaml.general.optimization_level);
}

#[test]
fn test_performance_config_clone() {
    let config = PerformanceConfig::default();
    let cloned_config = config.clone();
    
    // Test that all fields are properly cloned
    assert_eq!(cloned_config.general.enable_monitoring, config.general.enable_monitoring);
    assert_eq!(cloned_config.general.enable_profiling, config.general.enable_profiling);
    assert_eq!(cloned_config.general.enable_benchmarking, config.general.enable_benchmarking);
    assert_eq!(cloned_config.general.monitoring_interval, config.general.monitoring_interval);
    assert_eq!(cloned_config.general.max_degradation, config.general.max_degradation);
    assert_eq!(cloned_config.general.optimization_level, config.general.optimization_level);
    
    assert_eq!(cloned_config.aggregation.max_aggregation_time, config.aggregation.max_aggregation_time);
    assert_eq!(cloned_config.aggregation.max_contributions_per_aggregation, config.aggregation.max_contributions_per_aggregation);
    assert_eq!(cloned_config.aggregation.max_subsystems_per_aggregation, config.aggregation.max_subsystems_per_aggregation);
    assert_eq!(cloned_config.aggregation.min_throughput, config.aggregation.min_throughput);
    assert_eq!(cloned_config.aggregation.max_latency, config.aggregation.max_latency);
    assert_eq!(cloned_config.aggregation.enable_parallel_aggregation, config.aggregation.enable_parallel_aggregation);
    assert_eq!(cloned_config.aggregation.parallel_threshold, config.aggregation.parallel_threshold);
    assert_eq!(cloned_config.aggregation.enable_aggregation_caching, config.aggregation.enable_aggregation_caching);
    assert_eq!(cloned_config.aggregation.aggregation_cache_ttl, config.aggregation.aggregation_cache_ttl);
    
    assert_eq!(cloned_config.cache.max_operation_time, config.cache.max_operation_time);
    assert_eq!(cloned_config.cache.min_hit_rate, config.cache.min_hit_rate);
    assert_eq!(cloned_config.cache.max_cache_size, config.cache.max_cache_size);
    assert_eq!(cloned_config.cache.eviction_threshold, config.cache.eviction_threshold);
    assert_eq!(cloned_config.cache.enable_cache_warming, config.cache.enable_cache_warming);
    assert_eq!(cloned_config.cache.warming_interval, config.cache.warming_interval);
    assert_eq!(cloned_config.cache.enable_compression, config.cache.enable_compression);
    assert_eq!(cloned_config.cache.compression_threshold, config.cache.compression_threshold);
    
    assert_eq!(cloned_config.memory.max_memory_per_actor, config.memory.max_memory_per_actor);
    assert_eq!(cloned_config.memory.max_total_memory, config.memory.max_total_memory);
    assert_eq!(cloned_config.memory.warning_threshold, config.memory.warning_threshold);
    assert_eq!(cloned_config.memory.critical_threshold, config.memory.critical_threshold);
    assert_eq!(cloned_config.memory.enable_memory_pooling, config.memory.enable_memory_pooling);
    assert_eq!(cloned_config.memory.memory_pool_size, config.memory.memory_pool_size);
    assert_eq!(cloned_config.memory.enable_memory_compaction, config.memory.enable_memory_compaction);
    assert_eq!(cloned_config.memory.compaction_threshold, config.memory.compaction_threshold);
    
    assert_eq!(cloned_config.system.max_cpu_usage, config.system.max_cpu_usage);
    assert_eq!(cloned_config.system.cpu_warning_threshold, config.system.cpu_warning_threshold);
    assert_eq!(cloned_config.system.max_thread_count, config.system.max_thread_count);
    assert_eq!(cloned_config.system.thread_pool_size, config.system.thread_pool_size);
    assert_eq!(cloned_config.system.enable_cpu_affinity, config.system.enable_cpu_affinity);
    assert_eq!(cloned_config.system.enable_numa_awareness, config.system.enable_numa_awareness);
    assert_eq!(cloned_config.system.load_warning_threshold, config.system.load_warning_threshold);
    assert_eq!(cloned_config.system.load_critical_threshold, config.system.load_critical_threshold);
    
    assert_eq!(cloned_config.monitoring.enable_realtime_monitoring, config.monitoring.enable_realtime_monitoring);
    assert_eq!(cloned_config.monitoring.enable_historical_monitoring, config.monitoring.enable_historical_monitoring);
    assert_eq!(cloned_config.monitoring.data_retention_days, config.monitoring.data_retention_days);
    assert_eq!(cloned_config.monitoring.enable_metrics_export, config.monitoring.enable_metrics_export);
    assert_eq!(cloned_config.monitoring.export_interval, config.monitoring.export_interval);
    assert_eq!(cloned_config.monitoring.enable_dashboards, config.monitoring.enable_dashboards);
    assert_eq!(cloned_config.monitoring.dashboard_refresh_interval, config.monitoring.dashboard_refresh_interval);
    
    assert_eq!(cloned_config.benchmarking.enable_automated_benchmarking, config.benchmarking.enable_automated_benchmarking);
    assert_eq!(cloned_config.benchmarking.benchmarking_interval, config.benchmarking.benchmarking_interval);
    assert_eq!(cloned_config.benchmarking.benchmark_duration, config.benchmarking.benchmark_duration);
    assert_eq!(cloned_config.benchmarking.benchmark_iterations, config.benchmarking.benchmark_iterations);
    assert_eq!(cloned_config.benchmarking.enable_stress_testing, config.benchmarking.enable_stress_testing);
    assert_eq!(cloned_config.benchmarking.stress_test_duration, config.benchmarking.stress_test_duration);
    assert_eq!(cloned_config.benchmarking.stress_load_multiplier, config.benchmarking.stress_load_multiplier);
    assert_eq!(cloned_config.benchmarking.enable_regression_testing, config.benchmarking.enable_regression_testing);
    assert_eq!(cloned_config.benchmarking.regression_threshold, config.benchmarking.regression_threshold);
    
    assert_eq!(cloned_config.alerting.enable_alerts, config.alerting.enable_alerts);
    assert_eq!(cloned_config.alerting.severity_levels.warning_threshold, config.alerting.severity_levels.warning_threshold);
    assert_eq!(cloned_config.alerting.severity_levels.critical_threshold, config.alerting.severity_levels.critical_threshold);
    assert_eq!(cloned_config.alerting.severity_levels.fatal_threshold, config.alerting.severity_levels.fatal_threshold);
    assert_eq!(cloned_config.alerting.channels.enable_console, config.alerting.channels.enable_console);
    assert_eq!(cloned_config.alerting.channels.enable_file, config.alerting.channels.enable_file);
    assert_eq!(cloned_config.alerting.channels.enable_email, config.alerting.channels.enable_email);
    assert_eq!(cloned_config.alerting.channels.enable_webhook, config.alerting.channels.enable_webhook);
    assert_eq!(cloned_config.alerting.throttling.enable_throttling, config.alerting.throttling.enable_throttling);
    assert_eq!(cloned_config.alerting.throttling.max_alerts_per_minute, config.alerting.throttling.max_alerts_per_minute);
    assert_eq!(cloned_config.alerting.throttling.cooldown_period, config.alerting.throttling.cooldown_period);
    assert_eq!(cloned_config.alerting.throttling.enable_grouping, config.alerting.throttling.enable_grouping);
    assert_eq!(cloned_config.alerting.throttling.group_timeout, config.alerting.throttling.group_timeout);
    assert_eq!(cloned_config.alerting.escalation.enable_escalation, config.alerting.escalation.enable_escalation);
    assert_eq!(cloned_config.alerting.escalation.escalation_delay, config.alerting.escalation.escalation_delay);
    assert_eq!(cloned_config.alerting.escalation.max_escalation_levels, config.alerting.escalation.max_escalation_levels);
    assert_eq!(cloned_config.alerting.escalation.escalation_recipients.len(), config.alerting.escalation.escalation_recipients.len());
}

#[test]
fn test_performance_config_debug() {
    let config = PerformanceConfig::default();
    let debug_str = format!("{:?}", config);
    
    // Test that debug output contains expected fields
    assert!(debug_str.contains("PerformanceConfig"));
    assert!(debug_str.contains("general"));
    assert!(debug_str.contains("aggregation"));
    assert!(debug_str.contains("cache"));
    assert!(debug_str.contains("memory"));
    assert!(debug_str.contains("system"));
    assert!(debug_str.contains("monitoring"));
    assert!(debug_str.contains("benchmarking"));
    assert!(debug_str.contains("alerting"));
}
