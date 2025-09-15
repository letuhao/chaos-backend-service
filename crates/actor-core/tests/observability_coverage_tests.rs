use actor_core::observability::{StandardMetrics, CacheMetrics, SubsystemMetrics, ObservabilityManager, ObservabilityConfig, tracing_fields};
use std::time::Duration;

#[tokio::test]
async fn test_standard_metrics_default() {
    let metrics = StandardMetrics::default();
    assert!(metrics.operations.is_empty());
    assert_eq!(metrics.memory_usage, 0);
    assert!(metrics.errors.is_empty());
    assert_eq!(metrics.cache_stats.hits, 0);
    assert_eq!(metrics.subsystem_stats.total_executions, 0);
}

#[tokio::test]
async fn test_cache_metrics_default() {
    let cache_metrics = CacheMetrics::default();
    assert_eq!(cache_metrics.hits, 0);
    assert_eq!(cache_metrics.misses, 0);
    assert_eq!(cache_metrics.hit_rate, 0.0);
}

#[tokio::test]
async fn test_subsystem_metrics_default() {
    let subsystem_metrics = SubsystemMetrics::default();
    assert_eq!(subsystem_metrics.total_executions, 0);
    assert_eq!(subsystem_metrics.successful_executions, 0);
    assert_eq!(subsystem_metrics.failed_executions, 0);
}

#[tokio::test]
async fn test_standard_metrics_record_operation() {
    let mut metrics = StandardMetrics::default();
    metrics.record_operation("test_op", 1000);
    assert_eq!(metrics.operations.get("test_op"), Some(&1));
}

#[tokio::test]
async fn test_standard_metrics_record_cache_operation() {
    let mut metrics = StandardMetrics::default();
    metrics.record_cache_operation("get", true, 500);
    assert_eq!(metrics.cache_stats.hits, 1);
    assert_eq!(metrics.cache_stats.hit_rate, 1.0);
}

#[tokio::test]
async fn test_standard_metrics_record_subsystem_execution() {
    let mut metrics = StandardMetrics::default();
    metrics.record_subsystem_execution(true, 1000);
    assert_eq!(metrics.subsystem_stats.total_executions, 1);
    assert_eq!(metrics.subsystem_stats.successful_executions, 1);
}

#[tokio::test]
async fn test_standard_metrics_record_error() {
    let mut metrics = StandardMetrics::default();
    metrics.record_error("test_error");
    assert_eq!(metrics.errors.get("test_error"), Some(&1));
}

#[tokio::test]
async fn test_observability_config_default() {
    let config = ObservabilityConfig::default();
    assert!(config.enable_detailed_tracing);
    assert!(config.enable_metrics);
    assert_eq!(config.metrics_interval, Duration::from_secs(5));
}

#[tokio::test]
async fn test_observability_manager_new() {
    let manager = ObservabilityManager::new(ObservabilityConfig::default());
    let global_metrics = manager.get_global_metrics();
    assert!(global_metrics.operations.is_empty());
}

#[tokio::test]
async fn test_observability_manager_record_component_operation() {
    let manager = ObservabilityManager::new(ObservabilityConfig::default());
    manager.record_component_operation("test_component", "test_op", 1000);
    
    let component_metrics = manager.get_component_metrics("test_component");
    assert!(component_metrics.is_some());
    let metrics = component_metrics.unwrap();
    assert_eq!(metrics.operations.get("test_op"), Some(&1));
}

#[tokio::test]
async fn test_tracing_fields_constants() {
    assert_eq!(tracing_fields::ACTOR_ID, "actor_id");
    assert_eq!(tracing_fields::SYSTEM_ID, "system_id");
    assert_eq!(tracing_fields::OPERATION, "operation");
}

#[tokio::test]
async fn test_standard_metrics_record_multiple_operations() {
    let mut metrics = StandardMetrics::default();
    metrics.record_operation("test_op", 1000);
    metrics.record_operation("test_op", 2000);
    assert_eq!(metrics.operations.get("test_op"), Some(&2));
}

#[tokio::test]
async fn test_standard_metrics_record_cache_operation_miss() {
    let mut metrics = StandardMetrics::default();
    metrics.record_cache_operation("get", false, 500);
    assert_eq!(metrics.cache_stats.hits, 0);
    assert_eq!(metrics.cache_stats.misses, 1);
    assert_eq!(metrics.cache_stats.hit_rate, 0.0);
}

#[tokio::test]
async fn test_standard_metrics_record_cache_operation_set() {
    let mut metrics = StandardMetrics::default();
    metrics.record_cache_operation("set", false, 300);
    assert_eq!(metrics.cache_stats.sets, 1);
}

#[tokio::test]
async fn test_standard_metrics_record_cache_operation_delete() {
    let mut metrics = StandardMetrics::default();
    metrics.record_cache_operation("delete", false, 200);
    assert_eq!(metrics.cache_stats.deletes, 1);
}

#[tokio::test]
async fn test_standard_metrics_record_cache_operation_clear() {
    let mut metrics = StandardMetrics::default();
    metrics.record_cache_operation("clear", false, 100);
    assert_eq!(metrics.cache_stats.clears, 1);
}

#[tokio::test]
async fn test_standard_metrics_record_subsystem_execution_failure() {
    let mut metrics = StandardMetrics::default();
    metrics.record_subsystem_execution(false, 2000);
    assert_eq!(metrics.subsystem_stats.total_executions, 1);
    assert_eq!(metrics.subsystem_stats.successful_executions, 0);
    assert_eq!(metrics.subsystem_stats.failed_executions, 1);
}

#[tokio::test]
async fn test_standard_metrics_update_memory_usage() {
    let mut metrics = StandardMetrics::default();
    metrics.update_memory_usage(1024 * 1024);
    assert_eq!(metrics.memory_usage, 1024 * 1024);
    assert_eq!(metrics.cache_stats.memory_usage, 1024 * 1024);
}

#[tokio::test]
async fn test_standard_metrics_total_operations() {
    let mut metrics = StandardMetrics::default();
    metrics.record_operation("op1", 100);
    metrics.record_operation("op2", 200);
    assert_eq!(metrics.total_operations(), 2);
}

#[tokio::test]
async fn test_standard_metrics_error_rate() {
    let mut metrics = StandardMetrics::default();
    metrics.record_operation("op1", 100);
    metrics.record_error("error1");
    assert_eq!(metrics.error_rate(), 1.0);
}

#[tokio::test]
async fn test_observability_manager_record_component_cache_operation() {
    let manager = ObservabilityManager::new(ObservabilityConfig::default());
    manager.record_component_cache_operation("test_component", "get", true, 500);
    
    let component_metrics = manager.get_component_metrics("test_component");
    assert!(component_metrics.is_some());
    let metrics = component_metrics.unwrap();
    assert_eq!(metrics.cache_stats.hits, 1);
}

#[tokio::test]
async fn test_observability_manager_record_subsystem_execution() {
    let manager = ObservabilityManager::new(ObservabilityConfig::default());
    manager.record_subsystem_execution("test_subsystem", true, 1000);
    
    let component_metrics = manager.get_component_metrics("test_subsystem");
    assert!(component_metrics.is_some());
    let metrics = component_metrics.unwrap();
    assert_eq!(metrics.subsystem_stats.total_executions, 1);
}

#[tokio::test]
async fn test_observability_manager_record_component_error() {
    let manager = ObservabilityManager::new(ObservabilityConfig::default());
    manager.record_component_error("test_component", "test_error");
    
    let component_metrics = manager.get_component_metrics("test_component");
    assert!(component_metrics.is_some());
    let metrics = component_metrics.unwrap();
    assert_eq!(metrics.errors.get("test_error"), Some(&1));
}

#[tokio::test]
async fn test_observability_manager_get_component_metrics_nonexistent() {
    let manager = ObservabilityManager::new(ObservabilityConfig::default());
    let metrics = manager.get_component_metrics("nonexistent");
    assert!(metrics.is_none());
}

#[tokio::test]
async fn test_observability_config_custom() {
    let config = ObservabilityConfig {
        enable_detailed_tracing: false,
        enable_metrics: false,
        enable_performance_monitoring: false,
        metrics_interval: Duration::from_secs(10),
        max_component_metrics: 50,
    };
    assert!(!config.enable_detailed_tracing);
    assert!(!config.enable_metrics);
    assert_eq!(config.metrics_interval, Duration::from_secs(10));
}

#[tokio::test]
async fn test_observability_manager_disabled_metrics() {
    let config = ObservabilityConfig {
        enable_metrics: false,
        ..Default::default()
    };
    let manager = ObservabilityManager::new(config);
    manager.record_component_operation("test_component", "test_op", 1000);
    
    let component_metrics = manager.get_component_metrics("test_component");
    assert!(component_metrics.is_none());
}

#[tokio::test]
async fn test_tracing_fields_all_constants() {
    assert_eq!(tracing_fields::DIMENSION, "dimension");
    assert_eq!(tracing_fields::DURATION_US, "duration_us");
    assert_eq!(tracing_fields::CACHE_HIT, "cache_hit");
    assert_eq!(tracing_fields::CACHE_LAYER, "cache_layer");
    assert_eq!(tracing_fields::ERROR, "error");
    assert_eq!(tracing_fields::MEMORY_USAGE_BYTES, "memory_usage_bytes");
    assert_eq!(tracing_fields::QUEUE_SIZE, "queue_size");
    assert_eq!(tracing_fields::PROCESSING_TIME_US, "processing_time_us");
    assert_eq!(tracing_fields::BATCH_SIZE, "batch_size");
    assert_eq!(tracing_fields::VERSION, "version");
    assert_eq!(tracing_fields::PRIORITY, "priority");
    assert_eq!(tracing_fields::SUBSYSTEM_COUNT, "subsystem_count");
    assert_eq!(tracing_fields::CONTRIBUTION_COUNT, "contribution_count");
}
