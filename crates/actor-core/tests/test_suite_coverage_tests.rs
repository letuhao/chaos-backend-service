//! Coverage tests for performance/test_suite.rs module.

use actor_core::performance::test_suite::{
    PerformanceTestResult, PerformanceTestSuite, TestSuiteConfig, TestSuiteResults, 
    OverallTestResults, TestStatus
};
use actor_core::performance::profiler::{ProfilerConfig, PerformanceTestResult as ProfilerTestResult, PerformanceMetrics, AggregationMetrics, CacheMetrics, SystemMetrics, ErrorMetrics, ThresholdViolation, ViolationSeverity};
use std::time::Duration;
use std::collections::HashMap;

#[tokio::test]
async fn test_performance_test_result_creation() {
    let mut metrics = HashMap::new();
    metrics.insert("test_metric".to_string(), 100.0);
    
    let result = PerformanceTestResult {
        test_name: "test_cache_creation".to_string(),
        duration: Duration::from_millis(100),
        passed: true,
        score: 85.5,
        metrics,
        error_message: None,
        violations: vec!["latency: 5ms (threshold: 1ms)".to_string()],
        recommendations: vec!["Consider optimizing cache operations".to_string()],
    };
    
    assert_eq!(result.test_name, "test_cache_creation");
    assert_eq!(result.duration, Duration::from_millis(100));
    assert!(result.passed);
    assert_eq!(result.score, 85.5);
    assert_eq!(result.metrics.get("test_metric"), Some(&100.0));
    assert!(result.error_message.is_none());
    assert_eq!(result.violations.len(), 1);
    assert_eq!(result.recommendations.len(), 1);
}

#[tokio::test]
async fn test_performance_test_result_from_profiler_result() {
    let profiler_result = ProfilerTestResult {
        test_name: "profiler_test".to_string(),
        duration: Duration::from_millis(200),
        passed: true,
        score: 90.0,
        metrics: PerformanceMetrics {
            aggregation: AggregationMetrics {
                avg_aggregation_time: 5000, // microseconds
                max_aggregation_time: 10000,
                min_aggregation_time: 1000,
                total_aggregations: 1000,
                aggregations_per_second: 100.0,
                avg_contributions_per_aggregation: 10.0,
                avg_subsystems_per_aggregation: 5.0,
            },
            cache: CacheMetrics {
                hit_rate: 0.85,
                miss_rate: 0.15,
                avg_operation_time: 1000, // microseconds
                max_operation_time: 5000,
                total_operations: 2000,
                operations_per_second: 200.0,
                current_size: 1024 * 1024,
                eviction_rate: 0.1,
            },
            system: SystemMetrics {
                cpu_usage: 50.0,
                memory_usage: 1024 * 1024,
                memory_usage_percent: 50.0,
                thread_count: 4,
                uptime: 3600,
                load_average: 1.5,
            },
            errors: ErrorMetrics {
                total_errors: 5,
                error_rate: 0.01,
                errors_per_second: 0.1,
                error_types: HashMap::new(),
                last_error_time: Some(1234567890),
            },
            last_updated: 1234567890,
        },
        violations: vec![ThresholdViolation {
            threshold_name: "latency".to_string(),
            actual_value: 5.0, // milliseconds
            threshold_value: 1.0, // milliseconds
            severity: ViolationSeverity::Warning,
            timestamp: 1234567890,
        }],
        recommendations: vec!["Optimize performance".to_string()],
    };
    
    let result = PerformanceTestResult::from_profiler_result(profiler_result);
    
    assert_eq!(result.test_name, "profiler_test");
    assert_eq!(result.duration, Duration::from_millis(200));
    assert!(result.passed);
    assert_eq!(result.score, 90.0);
    assert_eq!(result.metrics.get("aggregation_ops"), Some(&1000.0));
    assert_eq!(result.metrics.get("cache_hit_rate"), Some(&0.85));
    assert_eq!(result.metrics.get("cache_miss_rate"), Some(&0.15));
    assert_eq!(result.metrics.get("system_memory"), Some((1024 * 1024) as f64).as_ref());
    assert_eq!(result.metrics.get("error_count"), Some(&5.0));
    assert_eq!(result.violations.len(), 1);
    assert_eq!(result.recommendations.len(), 1);
}

#[tokio::test]
async fn test_performance_test_result_debug_clone() {
    let mut metrics = HashMap::new();
    metrics.insert("test_metric".to_string(), 100.0);
    
    let result = PerformanceTestResult {
        test_name: "test_cache_creation".to_string(),
        duration: Duration::from_millis(100),
        passed: true,
        score: 85.5,
        metrics,
        error_message: None,
        violations: vec!["latency: 5ms (threshold: 1ms)".to_string()],
        recommendations: vec!["Consider optimizing cache operations".to_string()],
    };
    
    // Test Debug trait
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("test_cache_creation"));
    
    // Test Clone trait
    let cloned_result = result.clone();
    assert_eq!(cloned_result.test_name, result.test_name);
    assert_eq!(cloned_result.score, result.score);
}

#[tokio::test]
async fn test_test_suite_config_default() {
    let config = TestSuiteConfig::default();
    
    assert!(config.enable_cache_tests);
    assert!(config.enable_aggregation_tests);
    assert!(config.enable_memory_tests);
    assert!(config.enable_concurrency_tests);
    assert_eq!(config.stress_test_actor_count, 1000);
    assert_eq!(config.contributions_per_actor, 10);
    assert_eq!(config.stress_test_duration, Duration::from_secs(30));
    assert_eq!(config.concurrency_thread_count, 4);
}

#[tokio::test]
async fn test_test_suite_config_custom() {
    let config = TestSuiteConfig {
        enable_cache_tests: false,
        enable_aggregation_tests: true,
        enable_memory_tests: false,
        enable_concurrency_tests: true,
        stress_test_actor_count: 500,
        contributions_per_actor: 5,
        stress_test_duration: Duration::from_secs(15),
        concurrency_thread_count: 2,
    };
    
    assert!(!config.enable_cache_tests);
    assert!(config.enable_aggregation_tests);
    assert!(!config.enable_memory_tests);
    assert!(config.enable_concurrency_tests);
    assert_eq!(config.stress_test_actor_count, 500);
    assert_eq!(config.contributions_per_actor, 5);
    assert_eq!(config.stress_test_duration, Duration::from_secs(15));
    assert_eq!(config.concurrency_thread_count, 2);
}

#[tokio::test]
async fn test_test_suite_config_debug_clone() {
    let config = TestSuiteConfig::default();
    
    // Test Debug trait
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("enable_cache_tests: true"));
    
    // Test Clone trait
    let cloned_config = config.clone();
    assert_eq!(cloned_config.stress_test_actor_count, config.stress_test_actor_count);
    assert_eq!(cloned_config.concurrency_thread_count, config.concurrency_thread_count);
}

#[tokio::test]
async fn test_performance_test_suite_creation() {
    let config = TestSuiteConfig::default();
    let profiler_config = ProfilerConfig::default();
    let _test_suite = PerformanceTestSuite::new(config, profiler_config);
    
    assert!(true); // Basic creation test
}

#[tokio::test]
async fn test_performance_test_suite_new_default() {
    let _test_suite = PerformanceTestSuite::new_default();
    assert!(true); // Basic creation test
}

#[tokio::test]
async fn test_test_suite_results_creation() {
    let results = TestSuiteResults::new();
    
    assert!(results.cache_tests.is_empty());
    assert!(results.aggregation_tests.is_empty());
    assert!(results.memory_tests.is_empty());
    assert!(results.concurrency_tests.is_empty());
    assert_eq!(results.overall.total_tests, 0);
}

#[tokio::test]
async fn test_test_suite_results_calculate_overall_results() {
    let mut results = TestSuiteResults::new();
    
    // Add some test results
    let test_result1 = PerformanceTestResult {
        test_name: "test1".to_string(),
        duration: Duration::from_millis(100),
        passed: true,
        score: 90.0,
        metrics: HashMap::new(),
        error_message: None,
        violations: vec![],
        recommendations: vec![],
    };
    
    let test_result2 = PerformanceTestResult {
        test_name: "test2".to_string(),
        duration: Duration::from_millis(200),
        passed: false,
        score: 60.0,
        metrics: HashMap::new(),
        error_message: Some("Test failed".to_string()),
        violations: vec!["violation1".to_string()],
        recommendations: vec!["recommendation1".to_string()],
    };
    
    results.cache_tests.push(test_result1.clone());
    results.aggregation_tests.push(test_result2);
    results.memory_tests.push(test_result1.clone());
    results.concurrency_tests.push(test_result1);
    
    results.calculate_overall_results();
    
    assert_eq!(results.overall.total_tests, 4);
    assert_eq!(results.overall.passed_tests, 3);
    assert_eq!(results.overall.failed_tests, 1);
    assert_eq!(results.overall.pass_rate, 75.0);
    assert_eq!(results.overall.average_score, 82.5);
    assert_eq!(results.overall.total_violations, 1);
    assert_eq!(results.overall.total_duration, Duration::from_millis(500));
    assert_eq!(results.overall.overall_status, TestStatus::Warning);
}

#[tokio::test]
async fn test_test_suite_results_all_tests() {
    let mut results = TestSuiteResults::new();
    
    let test_result = PerformanceTestResult {
        test_name: "test1".to_string(),
        duration: Duration::from_millis(100),
        passed: true,
        score: 90.0,
        metrics: HashMap::new(),
        error_message: None,
        violations: vec![],
        recommendations: vec![],
    };
    
    results.cache_tests.push(test_result.clone());
    results.aggregation_tests.push(test_result.clone());
    
    let all_tests = results.all_tests();
    assert_eq!(all_tests.len(), 2);
}

#[tokio::test]
async fn test_test_suite_results_failed_tests() {
    let mut results = TestSuiteResults::new();
    
    let passed_test = PerformanceTestResult {
        test_name: "passed_test".to_string(),
        duration: Duration::from_millis(100),
        passed: true,
        score: 90.0,
        metrics: HashMap::new(),
        error_message: None,
        violations: vec![],
        recommendations: vec![],
    };
    
    let failed_test = PerformanceTestResult {
        test_name: "failed_test".to_string(),
        duration: Duration::from_millis(200),
        passed: false,
        score: 60.0,
        metrics: HashMap::new(),
        error_message: Some("Test failed".to_string()),
        violations: vec!["violation1".to_string()],
        recommendations: vec!["recommendation1".to_string()],
    };
    
    results.cache_tests.push(passed_test);
    results.aggregation_tests.push(failed_test);
    
    let failed_tests = results.failed_tests();
    assert_eq!(failed_tests.len(), 1);
    assert_eq!(failed_tests[0].test_name, "failed_test");
}

#[tokio::test]
async fn test_test_suite_results_get_recommendations() {
    let mut results = TestSuiteResults::new();
    
    let failed_test1 = PerformanceTestResult {
        test_name: "failed_test1".to_string(),
        duration: Duration::from_millis(100),
        passed: false,
        score: 60.0,
        metrics: HashMap::new(),
        error_message: Some("Test failed".to_string()),
        violations: vec!["violation1".to_string()],
        recommendations: vec!["recommendation1".to_string(), "recommendation2".to_string()],
    };
    
    let failed_test2 = PerformanceTestResult {
        test_name: "failed_test2".to_string(),
        duration: Duration::from_millis(200),
        passed: false,
        score: 50.0,
        metrics: HashMap::new(),
        error_message: Some("Test failed".to_string()),
        violations: vec!["violation2".to_string()],
        recommendations: vec!["recommendation2".to_string(), "recommendation3".to_string()],
    };
    
    results.cache_tests.push(failed_test1);
    results.aggregation_tests.push(failed_test2);
    
    let recommendations = results.get_recommendations();
    assert_eq!(recommendations.len(), 3); // Should deduplicate "recommendation2"
    assert!(recommendations.contains(&"recommendation1".to_string()));
    assert!(recommendations.contains(&"recommendation2".to_string()));
    assert!(recommendations.contains(&"recommendation3".to_string()));
}

#[tokio::test]
async fn test_overall_test_results_default() {
    let results = OverallTestResults::default();
    
    assert_eq!(results.total_tests, 0);
    assert_eq!(results.passed_tests, 0);
    assert_eq!(results.failed_tests, 0);
    assert_eq!(results.pass_rate, 0.0);
    assert_eq!(results.average_score, 0.0);
    assert_eq!(results.total_violations, 0);
    assert_eq!(results.total_duration, Duration::ZERO);
    assert_eq!(results.overall_status, TestStatus::Passed);
}

#[tokio::test]
async fn test_overall_test_results_creation() {
    let results = OverallTestResults {
        total_tests: 10,
        passed_tests: 8,
        failed_tests: 2,
        pass_rate: 80.0,
        average_score: 85.5,
        total_violations: 3,
        total_duration: Duration::from_secs(30),
        overall_status: TestStatus::Warning,
    };
    
    assert_eq!(results.total_tests, 10);
    assert_eq!(results.passed_tests, 8);
    assert_eq!(results.failed_tests, 2);
    assert_eq!(results.pass_rate, 80.0);
    assert_eq!(results.average_score, 85.5);
    assert_eq!(results.total_violations, 3);
    assert_eq!(results.total_duration, Duration::from_secs(30));
    assert_eq!(results.overall_status, TestStatus::Warning);
}

#[tokio::test]
async fn test_overall_test_results_debug_clone() {
    let results = OverallTestResults {
        total_tests: 10,
        passed_tests: 8,
        failed_tests: 2,
        pass_rate: 80.0,
        average_score: 85.5,
        total_violations: 3,
        total_duration: Duration::from_secs(30),
        overall_status: TestStatus::Warning,
    };
    
    // Test Debug trait
    let debug_str = format!("{:?}", results);
    assert!(debug_str.contains("total_tests: 10"));
    
    // Test Clone trait
    let cloned_results = results.clone();
    assert_eq!(cloned_results.total_tests, results.total_tests);
    assert_eq!(cloned_results.overall_status, results.overall_status);
}

#[tokio::test]
async fn test_test_status_variants() {
    assert_eq!(TestStatus::Passed, TestStatus::Passed);
    assert_eq!(TestStatus::Warning, TestStatus::Warning);
    assert_eq!(TestStatus::Failed, TestStatus::Failed);
    
    assert_ne!(TestStatus::Passed, TestStatus::Warning);
    assert_ne!(TestStatus::Passed, TestStatus::Failed);
    assert_ne!(TestStatus::Warning, TestStatus::Failed);
}

#[tokio::test]
async fn test_test_status_default() {
    let status = TestStatus::default();
    assert_eq!(status, TestStatus::Passed);
}

#[tokio::test]
async fn test_test_status_display() {
    assert_eq!(format!("{}", TestStatus::Passed), "PASSED");
    assert_eq!(format!("{}", TestStatus::Warning), "WARNING");
    assert_eq!(format!("{}", TestStatus::Failed), "FAILED");
}

#[tokio::test]
async fn test_test_suite_results_debug_clone() {
    let mut results = TestSuiteResults::new();
    
    let test_result = PerformanceTestResult {
        test_name: "test1".to_string(),
        duration: Duration::from_millis(100),
        passed: true,
        score: 90.0,
        metrics: HashMap::new(),
        error_message: None,
        violations: vec![],
        recommendations: vec![],
    };
    
    results.cache_tests.push(test_result);
    
    // Test Debug trait
    let debug_str = format!("{:?}", results);
    assert!(debug_str.contains("cache_tests"));
    
    // Test Clone trait
    let cloned_results = results.clone();
    assert_eq!(cloned_results.cache_tests.len(), results.cache_tests.len());
    assert_eq!(cloned_results.overall.total_tests, results.overall.total_tests);
}
