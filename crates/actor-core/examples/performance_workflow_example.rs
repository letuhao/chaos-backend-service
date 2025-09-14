//! Performance Workflow Example
//!
//! This example demonstrates how to use the performance profiling workflow
//! to monitor and optimize Actor Core performance.

use actor_core::performance::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    #[cfg(feature = "cli-tools")]
    tracing_subscriber::fmt::init();

    println!("Performance Workflow Example");
    println!("===========================");

    // Create performance configuration
    let config = PerfConfig::default();
    println!("✅ Loaded performance configuration");
    println!("   General config: {:?}", config.general);
    println!("   Cache config: {:?}", config.cache);

    // Create performance workflow
    let workflow = PerformanceWorkflow::new(WorkflowConfig {
        enable_automated_testing: false, // Disable for demo
        test_interval: Duration::from_secs(60),
        enable_regression_detection: true,
        regression_threshold: 10.0,
        enable_alerts: true,
        alert_threshold: 70.0,
        enable_ci_integration: true,
        ci_failure_threshold: 60.0,
        enable_reporting: true,
        report_interval: Duration::from_secs(300),
        enable_optimization_suggestions: true,
        baseline_config: BaselineConfig {
            baseline_score: 85.0,
            baseline_throughput: 1000,
            baseline_latency: 5000,
            baseline_memory_usage: 50 * 1024 * 1024, // 50MB
            baseline_cache_hit_rate: 90.0,
        },
    });

    println!("✅ Created performance workflow");

    // Run a performance cycle
    println!("\n🔄 Running performance cycle...");
    let execution = workflow.run_performance_cycle().await?;

    // Display results
    println!("\n📊 Performance Test Results:");
    println!("  Overall Status: {}", execution.test_results.overall.overall_status);
    println!("  Pass Rate: {:.1}%", execution.test_results.overall.pass_rate);
    println!("  Average Score: {:.1}", execution.test_results.overall.average_score);
    println!("  Total Tests: {}", execution.test_results.overall.total_tests);
    println!("  Passed Tests: {}", execution.test_results.overall.passed_tests);
    println!("  Failed Tests: {}", execution.test_results.overall.failed_tests);

    // Display performance metrics
    println!("\n📈 Performance Metrics:");
    println!("  Performance Score: {:.1}", execution.performance_report.performance_score);
    println!("  Aggregation Time: {}μs", execution.performance_report.current_metrics.aggregation.avg_aggregation_time);
    println!("  Cache Hit Rate: {:.1}%", execution.performance_report.current_metrics.cache.hit_rate);
    println!("  Memory Usage: {:.1}MB", execution.performance_report.current_metrics.system.memory_usage as f64 / 1024.0 / 1024.0);
    println!("  CPU Usage: {:.1}%", execution.performance_report.current_metrics.system.cpu_usage);

    // Display threshold violations
    if !execution.performance_report.threshold_violations.is_empty() {
        println!("\n⚠️  Threshold Violations:");
        for violation in &execution.performance_report.threshold_violations {
            println!("  - {}: {:.2} (threshold: {:.2})", 
                violation.threshold_name, 
                violation.actual_value, 
                violation.threshold_value
            );
        }
    }

    // Display performance regressions
    if !execution.performance_report.performance_regressions.is_empty() {
        println!("\n📉 Performance Regressions:");
        for regression in &execution.performance_report.performance_regressions {
            println!("  - {}: {:.1}% degradation", 
                regression.metric_name, 
                regression.degradation_percentage
            );
        }
    }

    // Display alerts
    if !execution.alerts_triggered.is_empty() {
        println!("\n🚨 Performance Alerts:");
        for alert in &execution.alerts_triggered {
            match alert {
                PerformanceAlert::ScoreAlert { current_score, threshold } => {
                    println!("  - Score Alert: {:.1} (threshold: {:.1})", current_score, threshold);
                }
                PerformanceAlert::ThroughputAlert { current, baseline } => {
                    println!("  - Throughput Alert: {} ops/sec (baseline: {} ops/sec)", current, baseline);
                }
                PerformanceAlert::LatencyAlert { current, threshold } => {
                    println!("  - Latency Alert: {}μs (threshold: {}μs)", current, threshold);
                }
                PerformanceAlert::MemoryAlert { current, threshold } => {
                    println!("  - Memory Alert: {}MB (threshold: {}MB)", 
                        *current as f64 / 1024.0 / 1024.0, 
                        *threshold as f64 / 1024.0 / 1024.0
                    );
                }
                PerformanceAlert::CacheHitRateAlert { current, threshold } => {
                    println!("  - Cache Hit Rate Alert: {:.1}% (threshold: {:.1}%)", current, threshold);
                }
                PerformanceAlert::RegressionAlert { degradation, metric } => {
                    println!("  - Regression Alert: {} {:.1}% degradation", metric, degradation);
                }
            }
        }
    }

    // Display CI/CD status
    println!("\n🔧 CI/CD Status: {:?}", execution.ci_status);

    // Display recommendations
    if !execution.performance_report.recommendations.is_empty() {
        println!("\n💡 Recommendations:");
        for (i, recommendation) in execution.performance_report.recommendations.iter().enumerate() {
            println!("  {}. {}", i + 1, recommendation);
        }
    }

    // Display test-specific results
    println!("\n🧪 Test Results Summary:");
    for test_result in execution.test_results.all_tests() {
        let status = if test_result.passed { "✅ PASS" } else { "❌ FAIL" };
        println!("  {} {} (Score: {:.1})", status, test_result.test_name, test_result.score);
        
        if !test_result.violations.is_empty() {
            println!("    Violations: {}", test_result.violations.len());
        }
    }

    println!("\n✅ Performance workflow example completed successfully!");

    Ok(())
}
