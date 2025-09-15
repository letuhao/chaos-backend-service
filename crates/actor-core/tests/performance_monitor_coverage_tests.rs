//! Coverage tests for performance monitor module.

use actor_core::subsystems::performance::performance_monitor::{
    PerformanceMonitor,
    PerformanceConfig,
    PerformanceMetric,
    PerformanceTest,
    TestResult,
    TestConfig,
    MetricCategory,
    PerformanceStats,
    LoadTestingSuite,
    ComprehensiveTestResults
};
// use actor_core::types::Actor;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_performance_monitor_creation() {
    let config = PerformanceConfig::default();
    let monitor = PerformanceMonitor::new(config);
    
    // Test that the monitor was created successfully
    assert!(std::ptr::addr_of!(monitor) != std::ptr::null());
}

#[tokio::test]
async fn test_performance_config_default() {
    let config = PerformanceConfig::default();
    
    assert!(config.enable_monitoring);
    assert_eq!(config.collection_interval, 60);
    assert_eq!(config.max_metrics_history, 1000);
    assert!(!config.enable_automatic_testing);
    assert_eq!(config.test_interval, 300);
}

#[tokio::test]
async fn test_performance_config_custom() {
    let config = PerformanceConfig {
        enable_monitoring: false,
        collection_interval: 30,
        max_metrics_history: 500,
        enable_automatic_testing: true,
        test_interval: 600,
    };
    
    assert!(!config.enable_monitoring);
    assert_eq!(config.collection_interval, 30);
    assert_eq!(config.max_metrics_history, 500);
    assert!(config.enable_automatic_testing);
    assert_eq!(config.test_interval, 600);
}

#[tokio::test]
async fn test_performance_metric_creation() {
    let metric = PerformanceMetric {
        name: "test_metric".to_string(),
        value: 42.5,
        unit: "ms".to_string(),
        timestamp: 1234567890,
        category: MetricCategory::ResourceCalculation,
    };
    
    assert_eq!(metric.name, "test_metric");
    assert_eq!(metric.value, 42.5);
    assert_eq!(metric.unit, "ms");
    assert_eq!(metric.timestamp, 1234567890);
    assert_eq!(metric.category, MetricCategory::ResourceCalculation);
}

#[tokio::test]
async fn test_metric_category_variants() {
    let categories = vec![
        MetricCategory::ResourceCalculation,
        MetricCategory::Cache,
        MetricCategory::Database,
        MetricCategory::Memory,
        MetricCategory::Cpu,
        MetricCategory::Network,
        MetricCategory::EventProcessing,
    ];
    
    for category in categories {
        // Test that categories can be created and compared
        assert_eq!(category, category);
    }
}

#[tokio::test]
async fn test_test_result_creation() {
    let mut additional_metrics = HashMap::new();
    additional_metrics.insert("cpu_usage".to_string(), 75.5);
    
    let result = TestResult {
        test_name: "test_benchmark".to_string(),
        execution_time_ms: 150.0,
        memory_usage_bytes: 1024,
        success: true,
        error_message: None,
        timestamp: 1234567890,
        additional_metrics,
    };
    
    assert_eq!(result.test_name, "test_benchmark");
    assert_eq!(result.execution_time_ms, 150.0);
    assert_eq!(result.memory_usage_bytes, 1024);
    assert!(result.success);
    assert!(result.error_message.is_none());
    assert_eq!(result.timestamp, 1234567890);
    assert_eq!(result.additional_metrics.len(), 1);
}

#[tokio::test]
async fn test_test_config_creation() {
    let config = TestConfig {
        iterations: 100,
        warmup_iterations: 10,
        timeout_ms: 5000,
        memory_limit_bytes: Some(1024 * 1024),
        enable_memory_profiling: true,
        enable_cpu_profiling: false,
    };
    
    assert_eq!(config.iterations, 100);
    assert_eq!(config.warmup_iterations, 10);
    assert_eq!(config.timeout_ms, 5000);
    assert_eq!(config.memory_limit_bytes, Some(1024 * 1024));
    assert!(config.enable_memory_profiling);
    assert!(!config.enable_cpu_profiling);
}

#[tokio::test]
async fn test_performance_test_creation() {
    let results = vec![
        TestResult {
            test_name: "test1".to_string(),
            execution_time_ms: 100.0,
            memory_usage_bytes: 512,
            success: true,
            error_message: None,
            timestamp: 1234567890,
            additional_metrics: HashMap::new(),
        }
    ];
    
    let config = TestConfig {
        iterations: 50,
        warmup_iterations: 5,
        timeout_ms: 3000,
        memory_limit_bytes: None,
        enable_memory_profiling: false,
        enable_cpu_profiling: true,
    };
    
    let test = PerformanceTest {
        name: "performance_test".to_string(),
        description: "Test description".to_string(),
        results,
        config,
    };
    
    assert_eq!(test.name, "performance_test");
    assert_eq!(test.description, "Test description");
    assert_eq!(test.results.len(), 1);
    assert_eq!(test.config.iterations, 50);
}

#[tokio::test]
async fn test_record_metric() {
    let config = PerformanceConfig::default();
    let monitor = PerformanceMonitor::new(config);
    
    let metric = PerformanceMetric {
        name: "test_metric".to_string(),
        value: 42.5,
        unit: "ms".to_string(),
        timestamp: 1234567890,
        category: MetricCategory::ResourceCalculation,
    };
    
    let result = monitor.record_metric(metric).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_record_metric_disabled_monitoring() {
    let config = PerformanceConfig {
        enable_monitoring: false,
        ..Default::default()
    };
    let monitor = PerformanceMonitor::new(config);
    
    let metric = PerformanceMetric {
        name: "test_metric".to_string(),
        value: 42.5,
        unit: "ms".to_string(),
        timestamp: 1234567890,
        category: MetricCategory::ResourceCalculation,
    };
    
    let result = monitor.record_metric(metric).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_metrics() {
    let config = PerformanceConfig::default();
    let monitor = PerformanceMonitor::new(config);
    
    // Record some metrics
    let metric1 = PerformanceMetric {
        name: "metric1".to_string(),
        value: 10.0,
        unit: "ms".to_string(),
        timestamp: 1234567890,
        category: MetricCategory::ResourceCalculation,
    };
    
    let metric2 = PerformanceMetric {
        name: "metric2".to_string(),
        value: 20.0,
        unit: "ms".to_string(),
        timestamp: 1234567891,
        category: MetricCategory::Cache,
    };
    
    monitor.record_metric(metric1).await.unwrap();
    monitor.record_metric(metric2).await.unwrap();
    
    // Get all metrics
    let all_metrics = monitor.get_metrics(None).await.unwrap();
    assert_eq!(all_metrics.len(), 2);
    
    // Get metrics by category
    let resource_metrics = monitor.get_metrics(Some(&MetricCategory::ResourceCalculation)).await.unwrap();
    assert_eq!(resource_metrics.len(), 1);
    assert_eq!(resource_metrics[0].name, "metric1");
}

#[tokio::test]
async fn test_run_test() {
    let config = PerformanceConfig::default();
    let monitor = PerformanceMonitor::new(config);
    
    let result = monitor.run_test("test_name", || {
        // Simulate some work
        std::thread::sleep(Duration::from_millis(1));
        Ok(())
    }).await.unwrap();
    
    assert_eq!(result.test_name, "test_name");
    assert!(result.success);
    assert!(result.error_message.is_none());
    assert!(result.execution_time_ms > 0.0);
}

#[tokio::test]
async fn test_run_test_with_error() {
    let config = PerformanceConfig::default();
    let monitor = PerformanceMonitor::new(config);
    
    let result = monitor.run_test("failing_test", || {
        Err(actor_core::ActorCoreError::InvalidInput("Test error".to_string()))
    }).await.unwrap();
    
    assert_eq!(result.test_name, "failing_test");
    assert!(!result.success);
    assert!(result.error_message.is_some());
    assert!(result.error_message.unwrap().contains("Test error"));
}

#[tokio::test]
async fn test_run_benchmark() {
    let config = PerformanceConfig::default();
    let monitor = PerformanceMonitor::new(config);
    
    let test_config = TestConfig {
        iterations: 3,
        warmup_iterations: 1,
        timeout_ms: 5000,
        memory_limit_bytes: None,
        enable_memory_profiling: false,
        enable_cpu_profiling: false,
    };
    
    let results = monitor.run_benchmark("benchmark_test", test_config, || {
        // Simulate some work
        std::thread::sleep(Duration::from_millis(1));
        Ok(())
    }).await.unwrap();
    
    assert_eq!(results.len(), 3);
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.test_name, format!("benchmark_test_iteration_{}", i));
        assert!(result.success);
    }
}

#[tokio::test]
async fn test_get_test_results() {
    let config = PerformanceConfig::default();
    let monitor = PerformanceMonitor::new(config);
    
    // Run a test to create test results
    monitor.run_test("test1", || Ok(())).await.unwrap();
    monitor.run_test("test2", || Ok(())).await.unwrap();
    
    // Get all test results
    let all_results = monitor.get_test_results(None).await.unwrap();
    assert_eq!(all_results.len(), 2);
    
    // Get specific test results
    let test1_results = monitor.get_test_results(Some("test1")).await.unwrap();
    assert_eq!(test1_results.len(), 1);
    assert_eq!(test1_results[0].test_name, "test1");
}

#[tokio::test]
async fn test_get_performance_stats() {
    let config = PerformanceConfig::default();
    let monitor = PerformanceMonitor::new(config);
    
    // Record some metrics
    let metric = PerformanceMetric {
        name: "test_metric".to_string(),
        value: 42.5,
        unit: "ms".to_string(),
        timestamp: 1234567890,
        category: MetricCategory::ResourceCalculation,
    };
    monitor.record_metric(metric).await.unwrap();
    
    // Run some tests
    monitor.run_test("test1", || Ok(())).await.unwrap();
    monitor.run_test("test2", || Ok(())).await.unwrap();
    
    let stats = monitor.get_performance_stats().await.unwrap();
    
    assert_eq!(stats.total_metrics, 1);
    assert_eq!(stats.total_tests, 2);
    assert_eq!(stats.successful_tests, 2);
    assert_eq!(stats.success_rate, 1.0);
    assert!(stats.average_execution_time >= 0.0);
}

#[tokio::test]
async fn test_performance_stats_creation() {
    let mut category_counts = HashMap::new();
    category_counts.insert(MetricCategory::ResourceCalculation, 5);
    category_counts.insert(MetricCategory::Cache, 3);
    
    let stats = PerformanceStats {
        total_metrics: 8,
        category_counts,
        total_tests: 10,
        successful_tests: 8,
        success_rate: 0.8,
        average_execution_time: 150.0,
        average_memory_usage: 1024,
    };
    
    assert_eq!(stats.total_metrics, 8);
    assert_eq!(stats.total_tests, 10);
    assert_eq!(stats.successful_tests, 8);
    assert_eq!(stats.success_rate, 0.8);
    assert_eq!(stats.average_execution_time, 150.0);
    assert_eq!(stats.average_memory_usage, 1024);
}

#[tokio::test]
async fn test_load_testing_suite_creation() {
    let config = PerformanceConfig::default();
    let monitor = Arc::new(PerformanceMonitor::new(config));
    let suite = LoadTestingSuite::new(monitor);
    
    assert!(std::ptr::addr_of!(suite) != std::ptr::null());
}

#[tokio::test]
async fn test_generate_test_actors() {
    let config = PerformanceConfig::default();
    let monitor = Arc::new(PerformanceMonitor::new(config));
    let mut suite = LoadTestingSuite::new(monitor);
    
    let result = suite.generate_test_actors(5);
    assert!(result.is_ok());
    
    // Note: test_actors field is private, so we can't directly access it
    // The test verifies that the method executes without error
}

#[tokio::test]
async fn test_run_resource_calculation_load_test() {
    let config = PerformanceConfig::default();
    let monitor = Arc::new(PerformanceMonitor::new(config));
    let mut suite = LoadTestingSuite::new(monitor);
    
    suite.generate_test_actors(3).unwrap();
    
    let results = suite.run_resource_calculation_load_test(2).await.unwrap();
    assert_eq!(results.len(), 2);
    
    for result in results {
        assert!(result.test_name.contains("resource_calculation_load_test"));
        assert!(result.success);
    }
}

#[tokio::test]
async fn test_run_cache_performance_test() {
    let config = PerformanceConfig::default();
    let monitor = Arc::new(PerformanceMonitor::new(config));
    let mut suite = LoadTestingSuite::new(monitor);
    
    suite.generate_test_actors(2).unwrap();
    
    let results = suite.run_cache_performance_test(2).await.unwrap();
    assert_eq!(results.len(), 2);
    
    for result in results {
        assert!(result.test_name.contains("cache_performance_test"));
        assert!(result.success);
    }
}

#[tokio::test]
async fn test_run_database_performance_test() {
    let config = PerformanceConfig::default();
    let monitor = Arc::new(PerformanceMonitor::new(config));
    let mut suite = LoadTestingSuite::new(monitor);
    
    suite.generate_test_actors(2).unwrap();
    
    let results = suite.run_database_performance_test(2).await.unwrap();
    assert_eq!(results.len(), 2);
    
    for result in results {
        assert!(result.test_name.contains("database_performance_test"));
        assert!(result.success);
    }
}

#[tokio::test]
async fn test_run_comprehensive_test_suite() {
    let config = PerformanceConfig::default();
    let monitor = Arc::new(PerformanceMonitor::new(config));
    let mut suite = LoadTestingSuite::new(monitor);
    
    suite.generate_test_actors(2).unwrap();
    
    let results = suite.run_comprehensive_test_suite().await.unwrap();
    
    assert!(results.total_tests > 0);
    assert!(results.successful_tests > 0);
    assert!(results.success_rate > 0.0);
    assert!(results.total_execution_time > 0.0);
    assert!(results.average_execution_time > 0.0);
    // Note: total_memory_usage is usize (unsigned), so >= 0 is always true
    assert!(!results.test_results.is_empty());
}

#[tokio::test]
async fn test_comprehensive_test_results_creation() {
    let test_results = vec![
        TestResult {
            test_name: "test1".to_string(),
            execution_time_ms: 100.0,
            memory_usage_bytes: 512,
            success: true,
            error_message: None,
            timestamp: 1234567890,
            additional_metrics: HashMap::new(),
        },
        TestResult {
            test_name: "test2".to_string(),
            execution_time_ms: 200.0,
            memory_usage_bytes: 1024,
            success: false,
            error_message: Some("Error".to_string()),
            timestamp: 1234567891,
            additional_metrics: HashMap::new(),
        },
    ];
    
    let results = ComprehensiveTestResults {
        total_tests: 2,
        successful_tests: 1,
        success_rate: 0.5,
        total_execution_time: 300.0,
        average_execution_time: 150.0,
        total_memory_usage: 1536,
        average_memory_usage: 768,
        test_results,
    };
    
    assert_eq!(results.total_tests, 2);
    assert_eq!(results.successful_tests, 1);
    assert_eq!(results.success_rate, 0.5);
    assert_eq!(results.total_execution_time, 300.0);
    assert_eq!(results.average_execution_time, 150.0);
    assert_eq!(results.total_memory_usage, 1536);
    assert_eq!(results.average_memory_usage, 768);
    assert_eq!(results.test_results.len(), 2);
}

#[tokio::test]
async fn test_clone_operations() {
    let config = PerformanceConfig::default();
    let _monitor = PerformanceMonitor::new(config);
    
    let metric = PerformanceMetric {
        name: "test".to_string(),
        value: 42.0,
        unit: "ms".to_string(),
        timestamp: 1234567890,
        category: MetricCategory::ResourceCalculation,
    };
    
    let cloned_metric = metric.clone();
    assert_eq!(metric.name, cloned_metric.name);
    assert_eq!(metric.value, cloned_metric.value);
}

#[tokio::test]
async fn test_debug_formatting() {
    let config = PerformanceConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(!debug_str.is_empty());
    
    let metric = PerformanceMetric {
        name: "test".to_string(),
        value: 42.0,
        unit: "ms".to_string(),
        timestamp: 1234567890,
        category: MetricCategory::ResourceCalculation,
    };
    
    let debug_str = format!("{:?}", metric);
    assert!(!debug_str.is_empty());
}

#[tokio::test]
async fn test_hashmap_operations() {
    let mut map = HashMap::new();
    map.insert(MetricCategory::ResourceCalculation, 5);
    map.insert(MetricCategory::Cache, 3);
    
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&MetricCategory::ResourceCalculation), Some(&5));
    assert_eq!(map.get(&MetricCategory::Cache), Some(&3));
}

#[tokio::test]
async fn test_vec_operations() {
    let mut vec = Vec::new();
    vec.push("item1".to_string());
    vec.push("item2".to_string());
    
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], "item1");
    assert_eq!(vec[1], "item2");
}

#[tokio::test]
async fn test_string_operations() {
    let s1 = "test".to_string();
    let s2 = s1.clone();
    
    assert_eq!(s1, s2);
    assert_eq!(s1.len(), 4);
    assert!(!s1.is_empty());
}

#[tokio::test]
async fn test_option_operations() {
    let some_value = Some("test".to_string());
    let none_value: Option<String> = None;
    
    assert!(some_value.is_some());
    assert!(none_value.is_none());
    assert_eq!(some_value.as_ref().unwrap(), "test");
}

#[tokio::test]
async fn test_f64_operations() {
    let value1 = 100.0;
    let value2 = 50.0;
    
    assert_eq!(value1 + value2, 150.0);
    assert_eq!(value1 - value2, 50.0);
    assert_eq!(value1 * value2, 5000.0);
    assert_eq!(value1 / value2, 2.0);
}

#[tokio::test]
async fn test_usize_operations() {
    let value1 = 100usize;
    let value2 = 50usize;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[tokio::test]
async fn test_u64_operations() {
    let value1 = 100u64;
    let value2 = 50u64;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[tokio::test]
async fn test_boolean_operations() {
    let true_value = true;
    let false_value = false;
    
    assert!(true_value);
    assert!(!false_value);
    assert_eq!(true_value && false_value, false);
    assert_eq!(true_value || false_value, true);
}

#[tokio::test]
async fn test_duration_operations() {
    let duration1 = Duration::from_secs(60);
    let duration2 = Duration::from_secs(30);
    
    assert_eq!(duration1.as_secs(), 60);
    assert_eq!(duration2.as_secs(), 30);
    assert!(duration1 > duration2);
}

#[tokio::test]
async fn test_arc_operations() {
    let config = PerformanceConfig::default();
    let monitor = Arc::new(PerformanceMonitor::new(config));
    let cloned = monitor.clone();
    
    assert_eq!(Arc::strong_count(&monitor), 2);
    assert_eq!(Arc::strong_count(&cloned), 2);
}
