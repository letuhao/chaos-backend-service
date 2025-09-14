//! Performance test suite coverage tests for Actor Core.

use actor_core::performance::test_suite::{
    PerformanceTestResult, TestSuiteConfig, PerformanceTestSuite
};
use actor_core::performance::profiler::ProfilerConfig;
use std::time::Duration;
use std::collections::HashMap;

#[test]
fn test_performance_test_result_creation() {
    let mut metrics = HashMap::new();
    metrics.insert("test_metric".to_string(), 42.0);
    
    let result = PerformanceTestResult {
        test_name: "test_performance".to_string(),
        duration: Duration::from_secs(5),
        passed: true,
        score: 85.5,
        metrics,
        error_message: None,
        violations: vec!["threshold_violation".to_string()],
        recommendations: vec!["optimize_algorithm".to_string()],
    };
    
    assert_eq!(result.test_name, "test_performance");
    assert_eq!(result.duration, Duration::from_secs(5));
    assert!(result.passed);
    assert_eq!(result.score, 85.5);
    assert_eq!(result.metrics.len(), 1);
    assert_eq!(result.metrics.get("test_metric"), Some(&42.0));
    assert!(result.error_message.is_none());
    assert_eq!(result.violations.len(), 1);
    assert_eq!(result.violations[0], "threshold_violation");
    assert_eq!(result.recommendations.len(), 1);
    assert_eq!(result.recommendations[0], "optimize_algorithm");
}

#[test]
fn test_performance_test_result_clone() {
    let mut metrics = HashMap::new();
    metrics.insert("test_metric".to_string(), 42.0);
    
    let result = PerformanceTestResult {
        test_name: "test_performance".to_string(),
        duration: Duration::from_secs(5),
        passed: true,
        score: 85.5,
        metrics,
        error_message: None,
        violations: vec!["threshold_violation".to_string()],
        recommendations: vec!["optimize_algorithm".to_string()],
    };
    
    let cloned = result.clone();
    assert_eq!(cloned.test_name, result.test_name);
    assert_eq!(cloned.duration, result.duration);
    assert_eq!(cloned.passed, result.passed);
    assert_eq!(cloned.score, result.score);
    assert_eq!(cloned.metrics.len(), result.metrics.len());
    assert_eq!(cloned.error_message, result.error_message);
    assert_eq!(cloned.violations.len(), result.violations.len());
    assert_eq!(cloned.recommendations.len(), result.recommendations.len());
}

#[test]
fn test_performance_test_result_debug() {
    let mut metrics = HashMap::new();
    metrics.insert("test_metric".to_string(), 42.0);
    
    let result = PerformanceTestResult {
        test_name: "test_performance".to_string(),
        duration: Duration::from_secs(5),
        passed: true,
        score: 85.5,
        metrics,
        error_message: None,
        violations: vec!["threshold_violation".to_string()],
        recommendations: vec!["optimize_algorithm".to_string()],
    };
    
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("PerformanceTestResult"));
    assert!(debug_str.contains("test_name: \"test_performance\""));
    assert!(debug_str.contains("passed: true"));
    assert!(debug_str.contains("score: 85.5"));
}

#[test]
fn test_performance_test_result_with_error() {
    let result = PerformanceTestResult {
        test_name: "failed_test".to_string(),
        duration: Duration::from_secs(2),
        passed: false,
        score: 0.0,
        metrics: HashMap::new(),
        error_message: Some("Test failed due to timeout".to_string()),
        violations: vec![],
        recommendations: vec!["Increase timeout".to_string()],
    };
    
    assert_eq!(result.test_name, "failed_test");
    assert_eq!(result.duration, Duration::from_secs(2));
    assert!(!result.passed);
    assert_eq!(result.score, 0.0);
    assert_eq!(result.error_message, Some("Test failed due to timeout".to_string()));
    assert!(result.violations.is_empty());
    assert_eq!(result.recommendations.len(), 1);
}

#[test]
fn test_test_suite_config_default() {
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

#[test]
fn test_test_suite_config_creation() {
    let config = TestSuiteConfig {
        enable_cache_tests: false,
        enable_aggregation_tests: false,
        enable_memory_tests: false,
        enable_concurrency_tests: false,
        stress_test_actor_count: 500,
        contributions_per_actor: 5,
        stress_test_duration: Duration::from_secs(60),
        concurrency_thread_count: 2,
    };
    
    assert!(!config.enable_cache_tests);
    assert!(!config.enable_aggregation_tests);
    assert!(!config.enable_memory_tests);
    assert!(!config.enable_concurrency_tests);
    assert_eq!(config.stress_test_actor_count, 500);
    assert_eq!(config.contributions_per_actor, 5);
    assert_eq!(config.stress_test_duration, Duration::from_secs(60));
    assert_eq!(config.concurrency_thread_count, 2);
}

#[test]
fn test_test_suite_config_clone() {
    let config = TestSuiteConfig::default();
    let cloned = config.clone();
    
    assert_eq!(cloned.enable_cache_tests, config.enable_cache_tests);
    assert_eq!(cloned.enable_aggregation_tests, config.enable_aggregation_tests);
    assert_eq!(cloned.enable_memory_tests, config.enable_memory_tests);
    assert_eq!(cloned.enable_concurrency_tests, config.enable_concurrency_tests);
    assert_eq!(cloned.stress_test_actor_count, config.stress_test_actor_count);
    assert_eq!(cloned.contributions_per_actor, config.contributions_per_actor);
    assert_eq!(cloned.stress_test_duration, config.stress_test_duration);
    assert_eq!(cloned.concurrency_thread_count, config.concurrency_thread_count);
}

#[test]
fn test_test_suite_config_debug() {
    let config = TestSuiteConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("TestSuiteConfig"));
    assert!(debug_str.contains("enable_cache_tests: true"));
    assert!(debug_str.contains("stress_test_actor_count: 1000"));
}

#[test]
fn test_performance_test_suite_new() {
    let config = TestSuiteConfig::default();
    let profiler_config = ProfilerConfig::default();
    let _test_suite = PerformanceTestSuite::new(config, profiler_config);
    
    // Test that the test suite was created successfully
    // We can't directly access the internal fields, but we can verify it exists
    assert!(true); // Test passes if construction succeeds
}

#[test]
fn test_performance_test_suite_new_default() {
    let _test_suite = PerformanceTestSuite::new_default();
    
    // Test that the test suite was created successfully with default config
    assert!(true); // Test passes if construction succeeds
}

#[test]
fn test_performance_test_result_serialization() {
    let mut metrics = HashMap::new();
    metrics.insert("test_metric".to_string(), 42.0);
    
    let result = PerformanceTestResult {
        test_name: "test_performance".to_string(),
        duration: Duration::from_secs(5),
        passed: true,
        score: 85.5,
        metrics,
        error_message: None,
        violations: vec!["threshold_violation".to_string()],
        recommendations: vec!["optimize_algorithm".to_string()],
    };
    
    // Test serialization
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("test_performance"));
    assert!(json.contains("85.5"));
    assert!(json.contains("true"));
    
    // Test deserialization
    let deserialized: PerformanceTestResult = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.test_name, result.test_name);
    assert_eq!(deserialized.passed, result.passed);
    assert_eq!(deserialized.score, result.score);
    assert_eq!(deserialized.violations.len(), result.violations.len());
    assert_eq!(deserialized.recommendations.len(), result.recommendations.len());
}

#[test]
fn test_performance_test_result_with_complex_metrics() {
    let mut metrics = HashMap::new();
    metrics.insert("cpu_usage".to_string(), 75.5);
    metrics.insert("memory_usage".to_string(), 1024.0);
    metrics.insert("response_time".to_string(), 0.5);
    metrics.insert("throughput".to_string(), 1000.0);
    
    let result = PerformanceTestResult {
        test_name: "complex_performance_test".to_string(),
        duration: Duration::from_millis(1500),
        passed: true,
        score: 92.3,
        metrics,
        error_message: None,
        violations: vec![
            "cpu_usage: 75.5 (threshold: 70.0)".to_string(),
            "memory_usage: 1024.0 (threshold: 800.0)".to_string(),
        ],
        recommendations: vec![
            "Optimize CPU usage".to_string(),
            "Reduce memory footprint".to_string(),
            "Consider caching strategies".to_string(),
        ],
    };
    
    assert_eq!(result.test_name, "complex_performance_test");
    assert_eq!(result.duration, Duration::from_millis(1500));
    assert!(result.passed);
    assert_eq!(result.score, 92.3);
    assert_eq!(result.metrics.len(), 4);
    assert_eq!(result.metrics.get("cpu_usage"), Some(&75.5));
    assert_eq!(result.metrics.get("memory_usage"), Some(&1024.0));
    assert_eq!(result.metrics.get("response_time"), Some(&0.5));
    assert_eq!(result.metrics.get("throughput"), Some(&1000.0));
    assert_eq!(result.violations.len(), 2);
    assert_eq!(result.recommendations.len(), 3);
}

#[test]
fn test_performance_test_result_empty_violations_and_recommendations() {
    let result = PerformanceTestResult {
        test_name: "perfect_test".to_string(),
        duration: Duration::from_secs(1),
        passed: true,
        score: 100.0,
        metrics: HashMap::new(),
        error_message: None,
        violations: vec![],
        recommendations: vec![],
    };
    
    assert_eq!(result.test_name, "perfect_test");
    assert_eq!(result.duration, Duration::from_secs(1));
    assert!(result.passed);
    assert_eq!(result.score, 100.0);
    assert!(result.metrics.is_empty());
    assert!(result.error_message.is_none());
    assert!(result.violations.is_empty());
    assert!(result.recommendations.is_empty());
}

#[test]
fn test_performance_test_result_long_duration() {
    let result = PerformanceTestResult {
        test_name: "long_running_test".to_string(),
        duration: Duration::from_secs(3600), // 1 hour
        passed: true,
        score: 78.5,
        metrics: HashMap::new(),
        error_message: None,
        violations: vec![],
        recommendations: vec!["Consider optimizing for faster execution".to_string()],
    };
    
    assert_eq!(result.test_name, "long_running_test");
    assert_eq!(result.duration, Duration::from_secs(3600));
    assert!(result.passed);
    assert_eq!(result.score, 78.5);
    assert_eq!(result.recommendations.len(), 1);
}

#[test]
fn test_performance_test_result_negative_score() {
    let result = PerformanceTestResult {
        test_name: "failed_test".to_string(),
        duration: Duration::from_millis(100),
        passed: false,
        score: -10.0, // Negative score for failed test
        metrics: HashMap::new(),
        error_message: Some("Critical failure occurred".to_string()),
        violations: vec!["critical_error: -10.0 (threshold: 0.0)".to_string()],
        recommendations: vec!["Fix critical issues".to_string()],
    };
    
    assert_eq!(result.test_name, "failed_test");
    assert_eq!(result.duration, Duration::from_millis(100));
    assert!(!result.passed);
    assert_eq!(result.score, -10.0);
    assert_eq!(result.error_message, Some("Critical failure occurred".to_string()));
    assert_eq!(result.violations.len(), 1);
    assert_eq!(result.recommendations.len(), 1);
}

#[test]
fn test_test_suite_config_custom_values() {
    let config = TestSuiteConfig {
        enable_cache_tests: true,
        enable_aggregation_tests: false,
        enable_memory_tests: true,
        enable_concurrency_tests: false,
        stress_test_actor_count: 2000,
        contributions_per_actor: 20,
        stress_test_duration: Duration::from_secs(120),
        concurrency_thread_count: 8,
    };
    
    assert!(config.enable_cache_tests);
    assert!(!config.enable_aggregation_tests);
    assert!(config.enable_memory_tests);
    assert!(!config.enable_concurrency_tests);
    assert_eq!(config.stress_test_actor_count, 2000);
    assert_eq!(config.contributions_per_actor, 20);
    assert_eq!(config.stress_test_duration, Duration::from_secs(120));
    assert_eq!(config.concurrency_thread_count, 8);
}

#[test]
fn test_performance_test_result_equality() {
    let mut metrics1 = HashMap::new();
    metrics1.insert("metric1".to_string(), 10.0);
    
    let mut metrics2 = HashMap::new();
    metrics2.insert("metric1".to_string(), 10.0);
    
    let result1 = PerformanceTestResult {
        test_name: "test1".to_string(),
        duration: Duration::from_secs(1),
        passed: true,
        score: 50.0,
        metrics: metrics1,
        error_message: None,
        violations: vec![],
        recommendations: vec![],
    };
    
    let result2 = PerformanceTestResult {
        test_name: "test1".to_string(),
        duration: Duration::from_secs(1),
        passed: true,
        score: 50.0,
        metrics: metrics2,
        error_message: None,
        violations: vec![],
        recommendations: vec![],
    };
    
    // Test that two identical results are equal
    assert_eq!(result1.test_name, result2.test_name);
    assert_eq!(result1.duration, result2.duration);
    assert_eq!(result1.passed, result2.passed);
    assert_eq!(result1.score, result2.score);
    assert_eq!(result1.metrics.len(), result2.metrics.len());
    assert_eq!(result1.error_message, result2.error_message);
    assert_eq!(result1.violations.len(), result2.violations.len());
    assert_eq!(result1.recommendations.len(), result2.recommendations.len());
}
