//! Performance analytics coverage tests for Actor Core.

use actor_core::performance::analytics::{
    AnalyticsConfig, MetricValue, TimeSeriesPoint,
    PerformanceMetrics, SystemMetrics, CacheMetrics, AggregationMetrics,
    MemoryMetrics, ErrorMetrics
};
use std::time::Duration;
use std::collections::HashMap;

#[test]
fn test_analytics_config_default() {
    let config = AnalyticsConfig::default();
    
    assert!(config.enable_analytics);
    assert_eq!(config.collection_interval, Duration::from_secs(1));
    assert_eq!(config.max_time_series_points, 10000);
    assert!(config.enable_counters);
    assert!(config.enable_memory_tracking);
    assert!(config.enable_latency_tracking);
}

#[test]
fn test_analytics_config_creation() {
    let config = AnalyticsConfig {
        enable_analytics: false,
        collection_interval: Duration::from_secs(5),
        max_time_series_points: 5000,
        enable_counters: false,
        enable_memory_tracking: false,
        enable_latency_tracking: false,
    };
    
    assert!(!config.enable_analytics);
    assert_eq!(config.collection_interval, Duration::from_secs(5));
    assert_eq!(config.max_time_series_points, 5000);
    assert!(!config.enable_counters);
    assert!(!config.enable_memory_tracking);
    assert!(!config.enable_latency_tracking);
}

#[test]
fn test_analytics_config_debug() {
    let config = AnalyticsConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("AnalyticsConfig"));
    assert!(debug_str.contains("enable_analytics: true"));
    assert!(debug_str.contains("collection_interval"));
}

#[test]
fn test_analytics_config_clone() {
    let config = AnalyticsConfig::default();
    let cloned_config = config.clone();
    
    assert_eq!(config.enable_analytics, cloned_config.enable_analytics);
    assert_eq!(config.collection_interval, cloned_config.collection_interval);
    assert_eq!(config.max_time_series_points, cloned_config.max_time_series_points);
    assert_eq!(config.enable_counters, cloned_config.enable_counters);
    assert_eq!(config.enable_memory_tracking, cloned_config.enable_memory_tracking);
    assert_eq!(config.enable_latency_tracking, cloned_config.enable_latency_tracking);
}

#[test]
fn test_metric_value_counter() {
    let counter = MetricValue::Counter(42);
    
    match counter {
        MetricValue::Counter(value) => assert_eq!(value, 42),
        _ => panic!("Expected Counter variant"),
    }
}

#[test]
fn test_metric_value_gauge() {
    let gauge = MetricValue::Gauge(3.14);
    
    match gauge {
        MetricValue::Gauge(value) => assert_eq!(value, 3.14),
        _ => panic!("Expected Gauge variant"),
    }
}

#[test]
fn test_metric_value_histogram() {
    let histogram = MetricValue::Histogram(vec![1.0, 2.0, 3.0]);
    
    match histogram {
        MetricValue::Histogram(values) => {
            assert_eq!(values.len(), 3);
            assert_eq!(values[0], 1.0);
            assert_eq!(values[1], 2.0);
            assert_eq!(values[2], 3.0);
        },
        _ => panic!("Expected Histogram variant"),
    }
}

#[test]
fn test_metric_value_string() {
    let string_value = MetricValue::String("test".to_string());
    
    match string_value {
        MetricValue::String(value) => assert_eq!(value, "test"),
        _ => panic!("Expected String variant"),
    }
}

#[test]
fn test_metric_value_debug() {
    let counter = MetricValue::Counter(42);
    let debug_str = format!("{:?}", counter);
    
    assert!(debug_str.contains("Counter"));
    assert!(debug_str.contains("42"));
}

#[test]
fn test_metric_value_clone() {
    let counter = MetricValue::Counter(42);
    let cloned_counter = counter.clone();
    
    match (counter, cloned_counter) {
        (MetricValue::Counter(a), MetricValue::Counter(b)) => assert_eq!(a, b),
        _ => panic!("Expected Counter variants"),
    }
}

#[test]
fn test_metric_value_serialization() {
    let counter = MetricValue::Counter(42);
    let gauge = MetricValue::Gauge(3.14);
    let histogram = MetricValue::Histogram(vec![1.0, 2.0, 3.0]);
    let string_value = MetricValue::String("test".to_string());
    
    // Test JSON serialization
    let counter_json = serde_json::to_string(&counter).unwrap();
    let gauge_json = serde_json::to_string(&gauge).unwrap();
    let histogram_json = serde_json::to_string(&histogram).unwrap();
    let string_json = serde_json::to_string(&string_value).unwrap();
    
    // Test deserialization
    let deserialized_counter: MetricValue = serde_json::from_str(&counter_json).unwrap();
    let deserialized_gauge: MetricValue = serde_json::from_str(&gauge_json).unwrap();
    let deserialized_histogram: MetricValue = serde_json::from_str(&histogram_json).unwrap();
    let deserialized_string: MetricValue = serde_json::from_str(&string_json).unwrap();
    
    match deserialized_counter {
        MetricValue::Counter(value) => assert_eq!(value, 42),
        _ => panic!("Expected Counter variant"),
    }
    
    match deserialized_gauge {
        MetricValue::Gauge(value) => assert_eq!(value, 3.14),
        _ => panic!("Expected Gauge variant"),
    }
    
    match deserialized_histogram {
        MetricValue::Histogram(values) => {
            assert_eq!(values.len(), 3);
            assert_eq!(values[0], 1.0);
        },
        _ => panic!("Expected Histogram variant"),
    }
    
    match deserialized_string {
        MetricValue::String(value) => assert_eq!(value, "test"),
        _ => panic!("Expected String variant"),
    }
}

#[test]
fn test_time_series_point_creation() {
    let mut tags = HashMap::new();
    tags.insert("service".to_string(), "actor-core".to_string());
    tags.insert("version".to_string(), "1.0.0".to_string());
    
    let point = TimeSeriesPoint {
        timestamp: 1234567890,
        metric: "cpu_usage".to_string(),
        value: MetricValue::Gauge(75.5),
        tags: tags.clone(),
    };
    
    assert_eq!(point.timestamp, 1234567890);
    assert_eq!(point.metric, "cpu_usage");
    assert_eq!(point.tags.len(), 2);
    assert_eq!(point.tags.get("service"), Some(&"actor-core".to_string()));
    assert_eq!(point.tags.get("version"), Some(&"1.0.0".to_string()));
    
    match point.value {
        MetricValue::Gauge(value) => assert_eq!(value, 75.5),
        _ => panic!("Expected Gauge variant"),
    }
}

#[test]
fn test_time_series_point_debug() {
    let point = TimeSeriesPoint {
        timestamp: 1234567890,
        metric: "test_metric".to_string(),
        value: MetricValue::Counter(100),
        tags: HashMap::new(),
    };
    
    let debug_str = format!("{:?}", point);
    assert!(debug_str.contains("TimeSeriesPoint"));
    assert!(debug_str.contains("1234567890"));
    assert!(debug_str.contains("test_metric"));
}

#[test]
fn test_time_series_point_clone() {
    let mut tags = HashMap::new();
    tags.insert("key".to_string(), "value".to_string());
    
    let point = TimeSeriesPoint {
        timestamp: 1234567890,
        metric: "test_metric".to_string(),
        value: MetricValue::Counter(100),
        tags: tags.clone(),
    };
    
    let cloned_point = point.clone();
    
    assert_eq!(point.timestamp, cloned_point.timestamp);
    assert_eq!(point.metric, cloned_point.metric);
    assert_eq!(point.tags.len(), cloned_point.tags.len());
    assert_eq!(point.tags.get("key"), cloned_point.tags.get("key"));
}

#[test]
fn test_time_series_point_serialization() {
    let mut tags = HashMap::new();
    tags.insert("service".to_string(), "actor-core".to_string());
    
    let point = TimeSeriesPoint {
        timestamp: 1234567890,
        metric: "cpu_usage".to_string(),
        value: MetricValue::Gauge(75.5),
        tags: tags.clone(),
    };
    
    // Test JSON serialization
    let json = serde_json::to_string(&point).unwrap();
    let deserialized: TimeSeriesPoint = serde_json::from_str(&json).unwrap();
    
    assert_eq!(point.timestamp, deserialized.timestamp);
    assert_eq!(point.metric, deserialized.metric);
    assert_eq!(point.tags.len(), deserialized.tags.len());
    assert_eq!(point.tags.get("service"), deserialized.tags.get("service"));
}

#[test]
fn test_system_metrics_creation() {
    let metrics = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512, // 512 MB
        thread_count: 8,
        uptime: 3600, // 1 hour
        active_connections: 100,
    };
    
    assert_eq!(metrics.cpu_usage, 75.5);
    assert_eq!(metrics.memory_usage, 1024 * 1024 * 512);
    assert_eq!(metrics.thread_count, 8);
    assert_eq!(metrics.uptime, 3600);
    assert_eq!(metrics.active_connections, 100);
}

#[test]
fn test_system_metrics_debug() {
    let metrics = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        thread_count: 8,
        uptime: 3600,
        active_connections: 100,
    };
    
    let debug_str = format!("{:?}", metrics);
    assert!(debug_str.contains("SystemMetrics"));
    assert!(debug_str.contains("cpu_usage: 75.5"));
    assert!(debug_str.contains("memory_usage: 536870912"));
}

#[test]
fn test_system_metrics_clone() {
    let metrics = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        thread_count: 8,
        uptime: 3600,
        active_connections: 100,
    };
    
    let cloned_metrics = metrics.clone();
    
    assert_eq!(metrics.cpu_usage, cloned_metrics.cpu_usage);
    assert_eq!(metrics.memory_usage, cloned_metrics.memory_usage);
    assert_eq!(metrics.thread_count, cloned_metrics.thread_count);
    assert_eq!(metrics.uptime, cloned_metrics.uptime);
    assert_eq!(metrics.active_connections, cloned_metrics.active_connections);
}

#[test]
fn test_cache_metrics_creation() {
    let metrics = CacheMetrics {
        hit_rate: 0.95,
        miss_rate: 0.05,
        avg_access_time: Duration::from_millis(1),
        cache_size: 1024 * 1024 * 100, // 100 MB
        evictions: 1000,
        l1_hit_rate: 0.98,
        l2_hit_rate: 0.90,
        l3_hit_rate: 0.85,
    };
    
    assert_eq!(metrics.hit_rate, 0.95);
    assert_eq!(metrics.miss_rate, 0.05);
    assert_eq!(metrics.avg_access_time, Duration::from_millis(1));
    assert_eq!(metrics.cache_size, 1024 * 1024 * 100);
    assert_eq!(metrics.evictions, 1000);
    assert_eq!(metrics.l1_hit_rate, 0.98);
    assert_eq!(metrics.l2_hit_rate, 0.90);
    assert_eq!(metrics.l3_hit_rate, 0.85);
}

#[test]
fn test_cache_metrics_debug() {
    let metrics = CacheMetrics {
        hit_rate: 0.95,
        miss_rate: 0.05,
        avg_access_time: Duration::from_millis(1),
        cache_size: 1024 * 1024 * 100,
        evictions: 1000,
        l1_hit_rate: 0.98,
        l2_hit_rate: 0.90,
        l3_hit_rate: 0.85,
    };
    
    let debug_str = format!("{:?}", metrics);
    assert!(debug_str.contains("CacheMetrics"));
    assert!(debug_str.contains("hit_rate: 0.95"));
    assert!(debug_str.contains("miss_rate: 0.05"));
}

#[test]
fn test_aggregation_metrics_creation() {
    let metrics = AggregationMetrics {
        total_aggregations: 10000,
        avg_aggregation_time: Duration::from_millis(10),
        active_actors: 1000,
        active_subsystems: 50,
        avg_contributions: 25.5,
        avg_caps: 5.2,
    };
    
    assert_eq!(metrics.total_aggregations, 10000);
    assert_eq!(metrics.avg_aggregation_time, Duration::from_millis(10));
    assert_eq!(metrics.active_actors, 1000);
    assert_eq!(metrics.active_subsystems, 50);
    assert_eq!(metrics.avg_contributions, 25.5);
    assert_eq!(metrics.avg_caps, 5.2);
}

#[test]
fn test_aggregation_metrics_debug() {
    let metrics = AggregationMetrics {
        total_aggregations: 10000,
        avg_aggregation_time: Duration::from_millis(10),
        active_actors: 1000,
        active_subsystems: 50,
        avg_contributions: 25.5,
        avg_caps: 5.2,
    };
    
    let debug_str = format!("{:?}", metrics);
    assert!(debug_str.contains("AggregationMetrics"));
    assert!(debug_str.contains("total_aggregations: 10000"));
    assert!(debug_str.contains("active_actors: 1000"));
}

#[test]
fn test_memory_metrics_creation() {
    let metrics = MemoryMetrics {
        total_memory: 1024 * 1024 * 1024, // 1 GB
        pool_memory: 1024 * 1024 * 512, // 512 MB
        cache_memory: 1024 * 1024 * 256, // 256 MB
        allocated_memory: 1024 * 1024 * 64, // 64 MB
        peak_memory: 1024 * 1024 * 300, // 300 MB
        fragmentation: 0.05,
    };
    
    assert_eq!(metrics.total_memory, 1024 * 1024 * 1024);
    assert_eq!(metrics.pool_memory, 1024 * 1024 * 512);
    assert_eq!(metrics.cache_memory, 1024 * 1024 * 256);
    assert_eq!(metrics.allocated_memory, 1024 * 1024 * 64);
    assert_eq!(metrics.total_memory, 1024 * 1024 * 1024);
    assert_eq!(metrics.pool_memory, 1024 * 1024 * 512);
    assert_eq!(metrics.fragmentation, 0.05);
    assert_eq!(metrics.peak_memory, 1024 * 1024 * 300);
}

#[test]
fn test_memory_metrics_debug() {
    let metrics = MemoryMetrics {
        total_memory: 1024 * 1024 * 1024,
        pool_memory: 1024 * 1024 * 512,
        cache_memory: 1024 * 1024 * 256,
        allocated_memory: 1024 * 1024 * 64,
        peak_memory: 1024 * 1024 * 300,
        fragmentation: 0.05,
    };
    
    let debug_str = format!("{:?}", metrics);
    assert!(debug_str.contains("MemoryMetrics"));
    assert!(debug_str.contains("total_memory: 1073741824"));
    assert!(debug_str.contains("allocated_memory:"));
}

#[test]
fn test_error_metrics_creation() {
    let metrics = ErrorMetrics {
        total_errors: 100,
        error_rate: 0.01,
        cache_errors: 5,
        aggregation_errors: 20,
        memory_errors: 10,
        validation_errors: 15,
    };
    
    assert_eq!(metrics.total_errors, 100);
    assert_eq!(metrics.error_rate, 0.01);
    assert_eq!(metrics.cache_errors, 5);
    assert_eq!(metrics.aggregation_errors, 20);
    assert_eq!(metrics.memory_errors, 10);
    assert_eq!(metrics.validation_errors, 15);
}

#[test]
fn test_error_metrics_debug() {
    let metrics = ErrorMetrics {
        total_errors: 100,
        error_rate: 0.01,
        cache_errors: 5,
        aggregation_errors: 20,
        memory_errors: 10,
        validation_errors: 15,
    };
    
    let debug_str = format!("{:?}", metrics);
    assert!(debug_str.contains("ErrorMetrics"));
    assert!(debug_str.contains("total_errors: 100"));
    assert!(debug_str.contains("error_rate: 0.01"));
}


#[test]
fn test_performance_metrics_creation() {
    let system_metrics = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        thread_count: 8,
        uptime: 3600,
        active_connections: 100,
    };
    
    let cache_metrics = CacheMetrics {
        hit_rate: 0.95,
        miss_rate: 0.05,
        avg_access_time: Duration::from_millis(1),
        cache_size: 1024 * 1024 * 100,
        evictions: 1000,
        l1_hit_rate: 0.98,
        l2_hit_rate: 0.90,
        l3_hit_rate: 0.85,
    };
    
    let aggregation_metrics = AggregationMetrics {
        total_aggregations: 10000,
        avg_aggregation_time: Duration::from_millis(10),
        active_actors: 1000,
        active_subsystems: 50,
        avg_contributions: 25.5,
        avg_caps: 5.2,
    };
    
    let memory_metrics = MemoryMetrics {
        total_memory: 1024 * 1024 * 1024,
        pool_memory: 1024 * 1024 * 512,
        cache_memory: 1024 * 1024 * 256,
        allocated_memory: 1024 * 1024 * 64,
        peak_memory: 1024 * 1024 * 300,
        fragmentation: 0.05,
    };
    
    let error_metrics = ErrorMetrics {
        total_errors: 100,
        error_rate: 0.01,
        cache_errors: 5,
        aggregation_errors: 20,
        memory_errors: 10,
        validation_errors: 15,
    };
    
    let performance_metrics = PerformanceMetrics {
        system: system_metrics,
        cache: cache_metrics,
        aggregation: aggregation_metrics,
        memory: memory_metrics,
        errors: error_metrics,
    };
    
    assert_eq!(performance_metrics.system.cpu_usage, 75.5);
    assert_eq!(performance_metrics.cache.hit_rate, 0.95);
    assert_eq!(performance_metrics.aggregation.total_aggregations, 10000);
    assert_eq!(performance_metrics.memory.total_memory, 1024 * 1024 * 1024);
    assert_eq!(performance_metrics.errors.total_errors, 100);
}

#[test]
fn test_performance_metrics_debug() {
    let system_metrics = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        thread_count: 8,
        uptime: 3600,
        active_connections: 100,
    };
    
    let cache_metrics = CacheMetrics {
        hit_rate: 0.95,
        miss_rate: 0.05,
        avg_access_time: Duration::from_millis(1),
        cache_size: 1024 * 1024 * 100,
        evictions: 1000,
        l1_hit_rate: 0.98,
        l2_hit_rate: 0.90,
        l3_hit_rate: 0.85,
    };
    
    let aggregation_metrics = AggregationMetrics {
        total_aggregations: 10000,
        avg_aggregation_time: Duration::from_millis(10),
        active_actors: 1000,
        active_subsystems: 50,
        avg_contributions: 25.5,
        avg_caps: 5.2,
    };
    
    let memory_metrics = MemoryMetrics {
        total_memory: 1024 * 1024 * 1024,
        pool_memory: 1024 * 1024 * 512,
        cache_memory: 1024 * 1024 * 256,
        allocated_memory: 1024 * 1024 * 64,
        peak_memory: 1024 * 1024 * 300,
        fragmentation: 0.05,
    };
    
    let error_metrics = ErrorMetrics {
        total_errors: 100,
        error_rate: 0.01,
        cache_errors: 5,
        aggregation_errors: 20,
        memory_errors: 10,
        validation_errors: 15,
    };
    
    let performance_metrics = PerformanceMetrics {
        system: system_metrics,
        cache: cache_metrics,
        aggregation: aggregation_metrics,
        memory: memory_metrics,
        errors: error_metrics,
    };
    
    let debug_str = format!("{:?}", performance_metrics);
    assert!(debug_str.contains("PerformanceMetrics"));
    assert!(debug_str.contains("system: SystemMetrics"));
    assert!(debug_str.contains("cache: CacheMetrics"));
    assert!(debug_str.contains("aggregation: AggregationMetrics"));
    assert!(debug_str.contains("memory: MemoryMetrics"));
    assert!(debug_str.contains("errors: ErrorMetrics"));
}

#[test]
fn test_performance_metrics_clone() {
    let system_metrics = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        thread_count: 8,
        uptime: 3600,
        active_connections: 100,
    };
    
    let cache_metrics = CacheMetrics {
        hit_rate: 0.95,
        miss_rate: 0.05,
        avg_access_time: Duration::from_millis(1),
        cache_size: 1024 * 1024 * 100,
        evictions: 1000,
        l1_hit_rate: 0.98,
        l2_hit_rate: 0.90,
        l3_hit_rate: 0.85,
    };
    
    let aggregation_metrics = AggregationMetrics {
        total_aggregations: 10000,
        avg_aggregation_time: Duration::from_millis(10),
        active_actors: 1000,
        active_subsystems: 50,
        avg_contributions: 25.5,
        avg_caps: 5.2,
    };
    
    let memory_metrics = MemoryMetrics {
        total_memory: 1024 * 1024 * 1024,
        pool_memory: 1024 * 1024 * 512,
        cache_memory: 1024 * 1024 * 256,
        allocated_memory: 1024 * 1024 * 64,
        peak_memory: 1024 * 1024 * 300,
        fragmentation: 0.05,
    };
    
    let error_metrics = ErrorMetrics {
        total_errors: 100,
        error_rate: 0.01,
        cache_errors: 5,
        aggregation_errors: 20,
        memory_errors: 10,
        validation_errors: 15,
    };
    
    let performance_metrics = PerformanceMetrics {
        system: system_metrics,
        cache: cache_metrics,
        aggregation: aggregation_metrics,
        memory: memory_metrics,
        errors: error_metrics,
    };
    
    let cloned_metrics = performance_metrics.clone();
    
    assert_eq!(performance_metrics.system.cpu_usage, cloned_metrics.system.cpu_usage);
    assert_eq!(performance_metrics.cache.hit_rate, cloned_metrics.cache.hit_rate);
    assert_eq!(performance_metrics.aggregation.total_aggregations, cloned_metrics.aggregation.total_aggregations);
    assert_eq!(performance_metrics.memory.total_memory, cloned_metrics.memory.total_memory);
    assert_eq!(performance_metrics.errors.total_errors, cloned_metrics.errors.total_errors);
}

#[test]
fn test_performance_metrics_serialization() {
    let system_metrics = SystemMetrics {
        cpu_usage: 75.5,
        memory_usage: 1024 * 1024 * 512,
        thread_count: 8,
        uptime: 3600,
        active_connections: 100,
    };
    
    let cache_metrics = CacheMetrics {
        hit_rate: 0.95,
        miss_rate: 0.05,
        avg_access_time: Duration::from_millis(1),
        cache_size: 1024 * 1024 * 100,
        evictions: 1000,
        l1_hit_rate: 0.98,
        l2_hit_rate: 0.90,
        l3_hit_rate: 0.85,
    };
    
    let aggregation_metrics = AggregationMetrics {
        total_aggregations: 10000,
        avg_aggregation_time: Duration::from_millis(10),
        active_actors: 1000,
        active_subsystems: 50,
        avg_contributions: 25.5,
        avg_caps: 5.2,
    };
    
    let memory_metrics = MemoryMetrics {
        total_memory: 1024 * 1024 * 1024,
        pool_memory: 1024 * 1024 * 512,
        cache_memory: 1024 * 1024 * 256,
        allocated_memory: 1024 * 1024 * 64,
        peak_memory: 1024 * 1024 * 300,
        fragmentation: 0.05,
    };
    
    let error_metrics = ErrorMetrics {
        total_errors: 100,
        error_rate: 0.01,
        cache_errors: 5,
        aggregation_errors: 20,
        memory_errors: 10,
        validation_errors: 15,
    };
    
    let performance_metrics = PerformanceMetrics {
        system: system_metrics,
        cache: cache_metrics,
        aggregation: aggregation_metrics,
        memory: memory_metrics,
        errors: error_metrics,
    };
    
    // Test JSON serialization
    let json = serde_json::to_string(&performance_metrics).unwrap();
    let deserialized: PerformanceMetrics = serde_json::from_str(&json).unwrap();
    
    assert_eq!(performance_metrics.system.cpu_usage, deserialized.system.cpu_usage);
    assert_eq!(performance_metrics.cache.hit_rate, deserialized.cache.hit_rate);
    assert_eq!(performance_metrics.aggregation.total_aggregations, deserialized.aggregation.total_aggregations);
    assert_eq!(performance_metrics.memory.total_memory, deserialized.memory.total_memory);
    assert_eq!(performance_metrics.errors.total_errors, deserialized.errors.total_errors);
}
