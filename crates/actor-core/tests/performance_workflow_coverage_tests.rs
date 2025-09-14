//! Performance workflow coverage tests for Actor Core.

use actor_core::performance::workflow::{
    WorkflowConfig, BaselineConfig, WorkflowState, WorkflowStatus,
    PerformanceTrends, WorkflowExecution, PerformanceAlert, CiStatus
};
use actor_core::performance::test_suite::TestSuiteResults;
use actor_core::performance::profiler::PerformanceReport;
use std::time::Duration;

#[test]
fn test_workflow_config_default() {
    let config = WorkflowConfig::default();
    
    assert!(config.enable_automated_testing);
    assert_eq!(config.test_interval, Duration::from_secs(300));
    assert!(config.enable_regression_detection);
    assert_eq!(config.regression_threshold, 10.0);
    assert!(config.enable_alerts);
    assert_eq!(config.alert_threshold, 70.0);
    assert!(config.enable_ci_integration);
    assert_eq!(config.ci_failure_threshold, 60.0);
    assert!(config.enable_reporting);
    assert_eq!(config.report_interval, Duration::from_secs(3600));
    assert!(config.enable_optimization_suggestions);
}

#[test]
fn test_workflow_config_creation() {
    let config = WorkflowConfig {
        enable_automated_testing: false,
        test_interval: Duration::from_secs(600),
        enable_regression_detection: false,
        regression_threshold: 15.0,
        enable_alerts: false,
        alert_threshold: 80.0,
        enable_ci_integration: false,
        ci_failure_threshold: 70.0,
        enable_reporting: false,
        report_interval: Duration::from_secs(7200),
        enable_optimization_suggestions: false,
        baseline_config: BaselineConfig::default(),
    };
    
    assert!(!config.enable_automated_testing);
    assert_eq!(config.test_interval, Duration::from_secs(600));
    assert!(!config.enable_regression_detection);
    assert_eq!(config.regression_threshold, 15.0);
    assert!(!config.enable_alerts);
    assert_eq!(config.alert_threshold, 80.0);
    assert!(!config.enable_ci_integration);
    assert_eq!(config.ci_failure_threshold, 70.0);
    assert!(!config.enable_reporting);
    assert_eq!(config.report_interval, Duration::from_secs(7200));
    assert!(!config.enable_optimization_suggestions);
}

#[test]
fn test_workflow_config_clone() {
    let config = WorkflowConfig::default();
    let cloned = config.clone();
    
    assert_eq!(cloned.enable_automated_testing, config.enable_automated_testing);
    assert_eq!(cloned.test_interval, config.test_interval);
    assert_eq!(cloned.enable_regression_detection, config.enable_regression_detection);
    assert_eq!(cloned.regression_threshold, config.regression_threshold);
    assert_eq!(cloned.enable_alerts, config.enable_alerts);
    assert_eq!(cloned.alert_threshold, config.alert_threshold);
    assert_eq!(cloned.enable_ci_integration, config.enable_ci_integration);
    assert_eq!(cloned.ci_failure_threshold, config.ci_failure_threshold);
    assert_eq!(cloned.enable_reporting, config.enable_reporting);
    assert_eq!(cloned.report_interval, config.report_interval);
    assert_eq!(cloned.enable_optimization_suggestions, config.enable_optimization_suggestions);
}

#[test]
fn test_workflow_config_debug() {
    let config = WorkflowConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("WorkflowConfig"));
    assert!(debug_str.contains("enable_automated_testing: true"));
    assert!(debug_str.contains("test_interval: 300s"));
}

#[test]
fn test_baseline_config_default() {
    let config = BaselineConfig::default();
    
    assert_eq!(config.baseline_score, 85.0);
    assert_eq!(config.baseline_throughput, 1000);
    assert_eq!(config.baseline_latency, 5000);
    assert_eq!(config.baseline_memory_usage, 50 * 1024 * 1024);
    assert_eq!(config.baseline_cache_hit_rate, 90.0);
}

#[test]
fn test_baseline_config_creation() {
    let config = BaselineConfig {
        baseline_score: 90.0,
        baseline_throughput: 2000,
        baseline_latency: 3000,
        baseline_memory_usage: 100 * 1024 * 1024,
        baseline_cache_hit_rate: 95.0,
    };
    
    assert_eq!(config.baseline_score, 90.0);
    assert_eq!(config.baseline_throughput, 2000);
    assert_eq!(config.baseline_latency, 3000);
    assert_eq!(config.baseline_memory_usage, 100 * 1024 * 1024);
    assert_eq!(config.baseline_cache_hit_rate, 95.0);
}

#[test]
fn test_baseline_config_clone() {
    let config = BaselineConfig::default();
    let cloned = config.clone();
    
    assert_eq!(cloned.baseline_score, config.baseline_score);
    assert_eq!(cloned.baseline_throughput, config.baseline_throughput);
    assert_eq!(cloned.baseline_latency, config.baseline_latency);
    assert_eq!(cloned.baseline_memory_usage, config.baseline_memory_usage);
    assert_eq!(cloned.baseline_cache_hit_rate, config.baseline_cache_hit_rate);
}

#[test]
fn test_baseline_config_debug() {
    let config = BaselineConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("BaselineConfig"));
    assert!(debug_str.contains("baseline_score: 85.0"));
    assert!(debug_str.contains("baseline_throughput: 1000"));
}

#[test]
fn test_workflow_state_creation() {
    let trends = PerformanceTrends {
        score_trend: 0.5,
        throughput_trend: 1.2,
        latency_trend: -0.3,
        memory_trend: 0.1,
        cache_hit_rate_trend: 0.8,
    };
    
    let state = WorkflowState {
        status: WorkflowStatus::Idle,
        last_test_time: Some(1234567890),
        last_report_time: Some(1234567891),
        current_score: 85.5,
        consecutive_failures: 0,
        last_alert_time: None,
        trends,
    };
    
    assert_eq!(state.status, WorkflowStatus::Idle);
    assert_eq!(state.last_test_time, Some(1234567890));
    assert_eq!(state.last_report_time, Some(1234567891));
    assert_eq!(state.current_score, 85.5);
    assert_eq!(state.consecutive_failures, 0);
    assert!(state.last_alert_time.is_none());
    assert_eq!(state.trends.score_trend, 0.5);
}

#[test]
fn test_workflow_state_clone() {
    let trends = PerformanceTrends {
        score_trend: 0.5,
        throughput_trend: 1.2,
        latency_trend: -0.3,
        memory_trend: 0.1,
        cache_hit_rate_trend: 0.8,
    };
    
    let state = WorkflowState {
        status: WorkflowStatus::Idle,
        last_test_time: Some(1234567890),
        last_report_time: Some(1234567891),
        current_score: 85.5,
        consecutive_failures: 0,
        last_alert_time: None,
        trends,
    };
    
    let cloned = state.clone();
    assert_eq!(cloned.status, state.status);
    assert_eq!(cloned.last_test_time, state.last_test_time);
    assert_eq!(cloned.last_report_time, state.last_report_time);
    assert_eq!(cloned.current_score, state.current_score);
    assert_eq!(cloned.consecutive_failures, state.consecutive_failures);
    assert_eq!(cloned.last_alert_time, state.last_alert_time);
    assert_eq!(cloned.trends.score_trend, state.trends.score_trend);
}

#[test]
fn test_workflow_state_debug() {
    let trends = PerformanceTrends {
        score_trend: 0.5,
        throughput_trend: 1.2,
        latency_trend: -0.3,
        memory_trend: 0.1,
        cache_hit_rate_trend: 0.8,
    };
    
    let state = WorkflowState {
        status: WorkflowStatus::Idle,
        last_test_time: Some(1234567890),
        last_report_time: Some(1234567891),
        current_score: 85.5,
        consecutive_failures: 0,
        last_alert_time: None,
        trends,
    };
    
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("WorkflowState"));
    assert!(debug_str.contains("status: Idle"));
    assert!(debug_str.contains("current_score: 85.5"));
}

#[test]
fn test_workflow_status_variants() {
    let idle = WorkflowStatus::Idle;
    let testing = WorkflowStatus::Testing;
    let reporting = WorkflowStatus::Reporting;
    let checking_regressions = WorkflowStatus::CheckingRegressions;
    let paused = WorkflowStatus::Paused;
    let error = WorkflowStatus::Error("Test error".to_string());
    
    assert_eq!(idle, WorkflowStatus::Idle);
    assert_eq!(testing, WorkflowStatus::Testing);
    assert_eq!(reporting, WorkflowStatus::Reporting);
    assert_eq!(checking_regressions, WorkflowStatus::CheckingRegressions);
    assert_eq!(paused, WorkflowStatus::Paused);
    assert_eq!(error, WorkflowStatus::Error("Test error".to_string()));
    
    assert_ne!(idle, testing);
    assert_ne!(testing, reporting);
    assert_ne!(reporting, checking_regressions);
    assert_ne!(checking_regressions, paused);
    assert_ne!(paused, error);
}

#[test]
fn test_workflow_status_clone() {
    let idle = WorkflowStatus::Idle;
    let testing = WorkflowStatus::Testing;
    let error = WorkflowStatus::Error("Test error".to_string());
    
    let idle_cloned = idle.clone();
    let testing_cloned = testing.clone();
    let error_cloned = error.clone();
    
    assert_eq!(idle_cloned, idle);
    assert_eq!(testing_cloned, testing);
    assert_eq!(error_cloned, error);
}

#[test]
fn test_workflow_status_debug() {
    let idle = WorkflowStatus::Idle;
    let testing = WorkflowStatus::Testing;
    let reporting = WorkflowStatus::Reporting;
    let checking_regressions = WorkflowStatus::CheckingRegressions;
    let paused = WorkflowStatus::Paused;
    let error = WorkflowStatus::Error("Test error".to_string());
    
    let idle_str = format!("{:?}", idle);
    let testing_str = format!("{:?}", testing);
    let reporting_str = format!("{:?}", reporting);
    let checking_str = format!("{:?}", checking_regressions);
    let paused_str = format!("{:?}", paused);
    let error_str = format!("{:?}", error);
    
    assert!(idle_str.contains("Idle"));
    assert!(testing_str.contains("Testing"));
    assert!(reporting_str.contains("Reporting"));
    assert!(checking_str.contains("CheckingRegressions"));
    assert!(paused_str.contains("Paused"));
    assert!(error_str.contains("Error"));
    assert!(error_str.contains("Test error"));
}

#[test]
fn test_performance_trends_creation() {
    let trends = PerformanceTrends {
        score_trend: 0.5,
        throughput_trend: 1.2,
        latency_trend: -0.3,
        memory_trend: 0.1,
        cache_hit_rate_trend: 0.8,
    };
    
    assert_eq!(trends.score_trend, 0.5);
    assert_eq!(trends.throughput_trend, 1.2);
    assert_eq!(trends.latency_trend, -0.3);
    assert_eq!(trends.memory_trend, 0.1);
    assert_eq!(trends.cache_hit_rate_trend, 0.8);
}

#[test]
fn test_performance_trends_clone() {
    let trends = PerformanceTrends {
        score_trend: 0.5,
        throughput_trend: 1.2,
        latency_trend: -0.3,
        memory_trend: 0.1,
        cache_hit_rate_trend: 0.8,
    };
    
    let cloned = trends.clone();
    assert_eq!(cloned.score_trend, trends.score_trend);
    assert_eq!(cloned.throughput_trend, trends.throughput_trend);
    assert_eq!(cloned.latency_trend, trends.latency_trend);
    assert_eq!(cloned.memory_trend, trends.memory_trend);
    assert_eq!(cloned.cache_hit_rate_trend, trends.cache_hit_rate_trend);
}

#[test]
fn test_performance_trends_debug() {
    let trends = PerformanceTrends {
        score_trend: 0.5,
        throughput_trend: 1.2,
        latency_trend: -0.3,
        memory_trend: 0.1,
        cache_hit_rate_trend: 0.8,
    };
    
    let debug_str = format!("{:?}", trends);
    assert!(debug_str.contains("PerformanceTrends"));
    assert!(debug_str.contains("score_trend: 0.5"));
    assert!(debug_str.contains("throughput_trend: 1.2"));
    assert!(debug_str.contains("latency_trend: -0.3"));
}

#[test]
fn test_workflow_execution_creation() {
    let test_results = TestSuiteResults::new();
    let performance_report = PerformanceReport {
        generated_at: 1234567890,
        session_duration: Duration::from_secs(5),
        current_metrics: actor_core::performance::profiler::PerformanceMetrics {
            aggregation: actor_core::performance::profiler::AggregationMetrics {
                avg_aggregation_time: 1000,
                max_aggregation_time: 2000,
                min_aggregation_time: 500,
                total_aggregations: 1000,
                aggregations_per_second: 10.5,
                avg_contributions_per_aggregation: 5.2,
                avg_subsystems_per_aggregation: 3.1,
            },
            cache: actor_core::performance::profiler::CacheMetrics {
                hit_rate: 85.5,
                miss_rate: 14.5,
                avg_operation_time: 100,
                max_operation_time: 500,
                total_operations: 10000,
                operations_per_second: 100.0,
                current_size: 1024 * 1024 * 100,
                eviction_rate: 5.2,
            },
            system: actor_core::performance::profiler::SystemMetrics {
                cpu_usage: 75.5,
                memory_usage: 1024 * 1024 * 512,
                memory_usage_percent: 50.0,
                thread_count: 8,
                uptime: 3600,
                load_average: 1.5,
            },
            errors: actor_core::performance::profiler::ErrorMetrics {
                total_errors: 10,
                error_rate: 0.1,
                errors_per_second: 0.5,
                error_types: std::collections::HashMap::new(),
                last_error_time: Some(1234567890),
            },
            last_updated: 1234567890,
        },
        performance_score: 85.5,
        threshold_violations: vec![],
        performance_regressions: vec![],
        history_summary: actor_core::performance::profiler::HistorySummary::default(),
        recommendations: vec!["Optimize algorithm".to_string()],
    };
    
    let execution = WorkflowExecution {
        timestamp: 1234567890,
        duration: Duration::from_secs(5),
        test_results,
        performance_report,
        regression_detected: false,
        alerts_triggered: vec![],
        ci_status: CiStatus::Passed,
    };
    
    assert_eq!(execution.timestamp, 1234567890);
    assert_eq!(execution.duration, Duration::from_secs(5));
    assert!(!execution.regression_detected);
    assert!(execution.alerts_triggered.is_empty());
    assert_eq!(execution.ci_status, CiStatus::Passed);
    assert_eq!(execution.performance_report.performance_score, 85.5);
    assert_eq!(execution.performance_report.recommendations.len(), 1);
}

#[test]
fn test_workflow_execution_clone() {
    let test_results = TestSuiteResults::new();
    let performance_report = PerformanceReport {
        generated_at: 1234567890,
        session_duration: Duration::from_secs(5),
        current_metrics: actor_core::performance::profiler::PerformanceMetrics {
            aggregation: actor_core::performance::profiler::AggregationMetrics {
                avg_aggregation_time: 1000,
                max_aggregation_time: 2000,
                min_aggregation_time: 500,
                total_aggregations: 1000,
                aggregations_per_second: 10.5,
                avg_contributions_per_aggregation: 5.2,
                avg_subsystems_per_aggregation: 3.1,
            },
            cache: actor_core::performance::profiler::CacheMetrics {
                hit_rate: 85.5,
                miss_rate: 14.5,
                avg_operation_time: 100,
                max_operation_time: 500,
                total_operations: 10000,
                operations_per_second: 100.0,
                current_size: 1024 * 1024 * 100,
                eviction_rate: 5.2,
            },
            system: actor_core::performance::profiler::SystemMetrics {
                cpu_usage: 75.5,
                memory_usage: 1024 * 1024 * 512,
                memory_usage_percent: 50.0,
                thread_count: 8,
                uptime: 3600,
                load_average: 1.5,
            },
            errors: actor_core::performance::profiler::ErrorMetrics {
                total_errors: 10,
                error_rate: 0.1,
                errors_per_second: 0.5,
                error_types: std::collections::HashMap::new(),
                last_error_time: Some(1234567890),
            },
            last_updated: 1234567890,
        },
        performance_score: 85.5,
        threshold_violations: vec![],
        performance_regressions: vec![],
        history_summary: actor_core::performance::profiler::HistorySummary::default(),
        recommendations: vec!["Optimize algorithm".to_string()],
    };
    
    let execution = WorkflowExecution {
        timestamp: 1234567890,
        duration: Duration::from_secs(5),
        test_results,
        performance_report,
        regression_detected: false,
        alerts_triggered: vec![],
        ci_status: CiStatus::Passed,
    };
    
    let cloned = execution.clone();
    assert_eq!(cloned.timestamp, execution.timestamp);
    assert_eq!(cloned.duration, execution.duration);
    assert_eq!(cloned.regression_detected, execution.regression_detected);
    assert_eq!(cloned.alerts_triggered.len(), execution.alerts_triggered.len());
    assert_eq!(cloned.ci_status, execution.ci_status);
    assert_eq!(cloned.performance_report.performance_score, execution.performance_report.performance_score);
    assert_eq!(cloned.performance_report.recommendations.len(), execution.performance_report.recommendations.len());
}

#[test]
fn test_workflow_execution_debug() {
    let test_results = TestSuiteResults::new();
    let performance_report = PerformanceReport {
        generated_at: 1234567890,
        session_duration: Duration::from_secs(5),
        current_metrics: actor_core::performance::profiler::PerformanceMetrics {
            aggregation: actor_core::performance::profiler::AggregationMetrics {
                avg_aggregation_time: 1000,
                max_aggregation_time: 2000,
                min_aggregation_time: 500,
                total_aggregations: 1000,
                aggregations_per_second: 10.5,
                avg_contributions_per_aggregation: 5.2,
                avg_subsystems_per_aggregation: 3.1,
            },
            cache: actor_core::performance::profiler::CacheMetrics {
                hit_rate: 85.5,
                miss_rate: 14.5,
                avg_operation_time: 100,
                max_operation_time: 500,
                total_operations: 10000,
                operations_per_second: 100.0,
                current_size: 1024 * 1024 * 100,
                eviction_rate: 5.2,
            },
            system: actor_core::performance::profiler::SystemMetrics {
                cpu_usage: 75.5,
                memory_usage: 1024 * 1024 * 512,
                memory_usage_percent: 50.0,
                thread_count: 8,
                uptime: 3600,
                load_average: 1.5,
            },
            errors: actor_core::performance::profiler::ErrorMetrics {
                total_errors: 10,
                error_rate: 0.1,
                errors_per_second: 0.5,
                error_types: std::collections::HashMap::new(),
                last_error_time: Some(1234567890),
            },
            last_updated: 1234567890,
        },
        performance_score: 85.5,
        threshold_violations: vec![],
        performance_regressions: vec![],
        history_summary: actor_core::performance::profiler::HistorySummary::default(),
        recommendations: vec!["Optimize algorithm".to_string()],
    };
    
    let execution = WorkflowExecution {
        timestamp: 1234567890,
        duration: Duration::from_secs(5),
        test_results,
        performance_report,
        regression_detected: false,
        alerts_triggered: vec![],
        ci_status: CiStatus::Passed,
    };
    
    let debug_str = format!("{:?}", execution);
    assert!(debug_str.contains("WorkflowExecution"));
    assert!(debug_str.contains("timestamp: 1234567890"));
    assert!(debug_str.contains("performance_score: 85.5"));
}

#[test]
fn test_serialization_deserialization() {
    let config = WorkflowConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: WorkflowConfig = serde_json::from_str(&json).unwrap();
    
    assert_eq!(config.enable_automated_testing, deserialized.enable_automated_testing);
    assert_eq!(config.test_interval, deserialized.test_interval);
    assert_eq!(config.regression_threshold, deserialized.regression_threshold);
    
    let baseline = BaselineConfig::default();
    let json = serde_json::to_string(&baseline).unwrap();
    let deserialized: BaselineConfig = serde_json::from_str(&json).unwrap();
    
    assert_eq!(baseline.baseline_score, deserialized.baseline_score);
    assert_eq!(baseline.baseline_throughput, deserialized.baseline_throughput);
    
    let status = WorkflowStatus::Testing;
    let json = serde_json::to_string(&status).unwrap();
    let deserialized: WorkflowStatus = serde_json::from_str(&json).unwrap();
    
    assert_eq!(status, deserialized);
}

#[test]
fn test_ci_status_variants() {
    let passed = CiStatus::Passed;
    let failed = CiStatus::Failed;
    let skipped = CiStatus::Skipped;
    
    assert_eq!(passed, CiStatus::Passed);
    assert_eq!(failed, CiStatus::Failed);
    assert_eq!(skipped, CiStatus::Skipped);
    
    assert_ne!(passed, failed);
    assert_ne!(failed, skipped);
    assert_ne!(passed, skipped);
}

#[test]
fn test_ci_status_clone() {
    let passed = CiStatus::Passed;
    let failed = CiStatus::Failed;
    let skipped = CiStatus::Skipped;
    
    let passed_cloned = passed.clone();
    let failed_cloned = failed.clone();
    let skipped_cloned = skipped.clone();
    
    assert_eq!(passed_cloned, passed);
    assert_eq!(failed_cloned, failed);
    assert_eq!(skipped_cloned, skipped);
}

#[test]
fn test_ci_status_debug() {
    let passed = CiStatus::Passed;
    let failed = CiStatus::Failed;
    let skipped = CiStatus::Skipped;
    
    let passed_str = format!("{:?}", passed);
    let failed_str = format!("{:?}", failed);
    let skipped_str = format!("{:?}", skipped);
    
    assert!(passed_str.contains("Passed"));
    assert!(failed_str.contains("Failed"));
    assert!(skipped_str.contains("Skipped"));
}

#[test]
fn test_performance_alert_variants() {
    let score_alert = PerformanceAlert::ScoreAlert { current_score: 50.0, threshold: 70.0 };
    let throughput_alert = PerformanceAlert::ThroughputAlert { current: 500, baseline: 1000 };
    let latency_alert = PerformanceAlert::LatencyAlert { current: 10000, threshold: 5000 };
    let memory_alert = PerformanceAlert::MemoryAlert { current: 100 * 1024 * 1024, threshold: 50 * 1024 * 1024 };
    let cache_alert = PerformanceAlert::CacheHitRateAlert { current: 80.0, threshold: 90.0 };
    let regression_alert = PerformanceAlert::RegressionAlert { degradation: 15.0, metric: "throughput".to_string() };
    
    match score_alert {
        PerformanceAlert::ScoreAlert { current_score, threshold } => {
            assert_eq!(current_score, 50.0);
            assert_eq!(threshold, 70.0);
        }
        _ => panic!("Expected ScoreAlert"),
    }
    
    match throughput_alert {
        PerformanceAlert::ThroughputAlert { current, baseline } => {
            assert_eq!(current, 500);
            assert_eq!(baseline, 1000);
        }
        _ => panic!("Expected ThroughputAlert"),
    }
    
    match latency_alert {
        PerformanceAlert::LatencyAlert { current, threshold } => {
            assert_eq!(current, 10000);
            assert_eq!(threshold, 5000);
        }
        _ => panic!("Expected LatencyAlert"),
    }
    
    match memory_alert {
        PerformanceAlert::MemoryAlert { current, threshold } => {
            assert_eq!(current, 100 * 1024 * 1024);
            assert_eq!(threshold, 50 * 1024 * 1024);
        }
        _ => panic!("Expected MemoryAlert"),
    }
    
    match cache_alert {
        PerformanceAlert::CacheHitRateAlert { current, threshold } => {
            assert_eq!(current, 80.0);
            assert_eq!(threshold, 90.0);
        }
        _ => panic!("Expected CacheHitRateAlert"),
    }
    
    match regression_alert {
        PerformanceAlert::RegressionAlert { degradation, metric } => {
            assert_eq!(degradation, 15.0);
            assert_eq!(metric, "throughput");
        }
        _ => panic!("Expected RegressionAlert"),
    }
}

#[test]
fn test_performance_alert_clone() {
    let score_alert = PerformanceAlert::ScoreAlert { current_score: 50.0, threshold: 70.0 };
    let cloned = score_alert.clone();
    
    match cloned {
        PerformanceAlert::ScoreAlert { current_score, threshold } => {
            assert_eq!(current_score, 50.0);
            assert_eq!(threshold, 70.0);
        }
        _ => panic!("Expected ScoreAlert"),
    }
}

#[test]
fn test_performance_alert_debug() {
    let score_alert = PerformanceAlert::ScoreAlert { current_score: 50.0, threshold: 70.0 };
    let debug_str = format!("{:?}", score_alert);
    
    assert!(debug_str.contains("ScoreAlert"));
    assert!(debug_str.contains("current_score: 50.0"));
    assert!(debug_str.contains("threshold: 70.0"));
}

#[test]
fn test_performance_trends_negative_values() {
    let trends = PerformanceTrends {
        score_trend: -0.5,
        throughput_trend: -1.2,
        latency_trend: 0.3,
        memory_trend: -0.1,
        cache_hit_rate_trend: -0.8,
    };
    
    assert_eq!(trends.score_trend, -0.5);
    assert_eq!(trends.throughput_trend, -1.2);
    assert_eq!(trends.latency_trend, 0.3);
    assert_eq!(trends.memory_trend, -0.1);
    assert_eq!(trends.cache_hit_rate_trend, -0.8);
}

#[test]
fn test_workflow_state_with_alert() {
    let trends = PerformanceTrends {
        score_trend: -0.5,
        throughput_trend: -1.2,
        latency_trend: 0.3,
        memory_trend: -0.1,
        cache_hit_rate_trend: -0.8,
    };
    
    let state = WorkflowState {
        status: WorkflowStatus::Error("Performance degraded".to_string()),
        last_test_time: Some(1234567890),
        last_report_time: Some(1234567891),
        current_score: 45.0,
        consecutive_failures: 3,
        last_alert_time: Some(1234567892),
        trends,
    };
    
    assert_eq!(state.status, WorkflowStatus::Error("Performance degraded".to_string()));
    assert_eq!(state.last_test_time, Some(1234567890));
    assert_eq!(state.last_report_time, Some(1234567891));
    assert_eq!(state.current_score, 45.0);
    assert_eq!(state.consecutive_failures, 3);
    assert_eq!(state.last_alert_time, Some(1234567892));
    assert_eq!(state.trends.score_trend, -0.5);
}
