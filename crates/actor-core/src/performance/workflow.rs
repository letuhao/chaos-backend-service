//! Performance workflow manager for Actor Core.
//!
//! This module provides automated performance workflow management,
//! including CI/CD integration, regression detection, and reporting.

use std::time::{Duration, SystemTime, UNIX_EPOCH, Instant};
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use crate::ActorCoreResult;
use crate::config::manager::ConfigurationManager;
use super::profiler::{PerformanceProfiler, PerformanceReport, ProfilerConfig};
use super::test_suite::{PerformanceTestSuite, TestSuiteConfig, TestSuiteResults};

/// Performance workflow manager that orchestrates performance testing.
pub struct PerformanceWorkflow {
    /// Performance profiler
    profiler: PerformanceProfiler,
    /// Performance test suite
    test_suite: PerformanceTestSuite,
    /// Workflow configuration
    config: WorkflowConfig,
    /// Workflow state
    state: Arc<RwLock<WorkflowState>>,
    /// Performance history
    history: Arc<RwLock<Vec<WorkflowExecution>>>,
    /// Configuration manager
    config_manager: Arc<ConfigurationManager>,
}

/// Configuration for the performance workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    /// Enable automated performance testing
    pub enable_automated_testing: bool,
    /// Performance test interval
    pub test_interval: Duration,
    /// Enable regression detection
    pub enable_regression_detection: bool,
    /// Regression threshold (percentage)
    pub regression_threshold: f64,
    /// Enable performance alerts
    pub enable_alerts: bool,
    /// Alert threshold (performance score)
    pub alert_threshold: f64,
    /// Enable CI/CD integration
    pub enable_ci_integration: bool,
    /// CI/CD failure threshold (performance score)
    pub ci_failure_threshold: f64,
    /// Enable performance reporting
    pub enable_reporting: bool,
    /// Report generation interval
    pub report_interval: Duration,
    /// Enable performance optimization suggestions
    pub enable_optimization_suggestions: bool,
    /// Performance baseline configuration
    pub baseline_config: BaselineConfig,
}

/// Configuration for performance baselines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineConfig {
    /// Baseline performance score
    pub baseline_score: f64,
    /// Baseline throughput (operations per second)
    pub baseline_throughput: u64,
    /// Baseline latency (microseconds)
    pub baseline_latency: u64,
    /// Baseline memory usage (bytes)
    pub baseline_memory_usage: u64,
    /// Baseline cache hit rate (percentage)
    pub baseline_cache_hit_rate: f64,
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            enable_automated_testing: true,
            test_interval: Duration::from_secs(300), // 5 minutes
            enable_regression_detection: true,
            regression_threshold: 10.0, // 10% degradation
            enable_alerts: true,
            alert_threshold: 70.0, // Alert if score drops below 70
            enable_ci_integration: true,
            ci_failure_threshold: 60.0, // Fail CI if score drops below 60
            enable_reporting: true,
            report_interval: Duration::from_secs(3600), // 1 hour
            enable_optimization_suggestions: true,
            baseline_config: BaselineConfig::default(),
        }
    }
}

impl Default for BaselineConfig {
    fn default() -> Self {
        Self {
            baseline_score: 85.0,
            baseline_throughput: 1000,
            baseline_latency: 5000, // 5ms
            baseline_memory_usage: 50 * 1024 * 1024, // 50MB
            baseline_cache_hit_rate: 90.0,
        }
    }
}

/// Current workflow state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    /// Current execution status
    pub status: WorkflowStatus,
    /// Last test execution time
    pub last_test_time: Option<u64>,
    /// Last report generation time
    pub last_report_time: Option<u64>,
    /// Current performance score
    pub current_score: f64,
    /// Number of consecutive failures
    pub consecutive_failures: u32,
    /// Last alert time
    pub last_alert_time: Option<u64>,
    /// Performance trends
    pub trends: PerformanceTrends,
}

/// Workflow execution status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// Workflow is idle
    Idle,
    /// Running performance tests
    Testing,
    /// Generating reports
    Reporting,
    /// Checking for regressions
    CheckingRegressions,
    /// Workflow is paused
    Paused,
    /// Workflow encountered an error
    Error(String),
}

/// Performance trends over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrends {
    /// Score trend (positive = improving, negative = degrading)
    pub score_trend: f64,
    /// Throughput trend
    pub throughput_trend: f64,
    /// Latency trend
    pub latency_trend: f64,
    /// Memory usage trend
    pub memory_trend: f64,
    /// Cache hit rate trend
    pub cache_hit_rate_trend: f64,
}

/// A workflow execution record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    /// Execution timestamp
    pub timestamp: u64,
    /// Execution duration
    pub duration: Duration,
    /// Test results
    pub test_results: TestSuiteResults,
    /// Performance report
    pub performance_report: PerformanceReport,
    /// Regression detected
    pub regression_detected: bool,
    /// Alerts triggered
    pub alerts_triggered: Vec<PerformanceAlert>,
    /// CI/CD status
    pub ci_status: CiStatus,
}

/// Performance alert types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceAlert {
    /// Performance score dropped below threshold
    ScoreAlert { current_score: f64, threshold: f64 },
    /// Throughput dropped below baseline
    ThroughputAlert { current: u64, baseline: u64 },
    /// Latency exceeded threshold
    LatencyAlert { current: u64, threshold: u64 },
    /// Memory usage exceeded threshold
    MemoryAlert { current: u64, threshold: u64 },
    /// Cache hit rate dropped below threshold
    CacheHitRateAlert { current: f64, threshold: f64 },
    /// Performance regression detected
    RegressionAlert { degradation: f64, metric: String },
}

/// CI/CD integration status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CiStatus {
    /// CI tests passed
    Passed,
    /// CI tests failed
    Failed,
    /// CI tests were skipped
    Skipped,
}

impl PerformanceWorkflow {
    /// Create a new performance workflow.
    pub fn new(config: WorkflowConfig, config_manager: Arc<ConfigurationManager>) -> Self {
        let profiler_config = ProfilerConfig::default();
        let test_suite_config = TestSuiteConfig::default();

        Self {
            profiler: PerformanceProfiler::new(profiler_config.clone(), config_manager.clone()),
            test_suite: PerformanceTestSuite::new(test_suite_config, profiler_config, config_manager.clone()),
            config,
            state: Arc::new(RwLock::new(WorkflowState::default())),
            history: Arc::new(RwLock::new(Vec::new())),
            config_manager,
        }
    }

    /// Create a new workflow with default configuration.
    pub fn new_default(config_manager: Arc<ConfigurationManager>) -> Self {
        Self::new(WorkflowConfig::default(), config_manager)
    }

    /// Start the performance workflow.
    pub async fn start(&self) -> ActorCoreResult<()> {
        tracing::info!("Starting performance workflow");

        // Initial performance test
        self.run_performance_cycle().await?;

        // Schedule regular performance tests if enabled
        if self.config.enable_automated_testing {
            self.schedule_performance_tests().await?;
        }

        Ok(())
    }

    /// Stop the performance workflow.
    pub fn stop(&self) {
        let mut state = self.state.write();
        state.status = WorkflowStatus::Paused;
        tracing::info!("Performance workflow stopped");
    }

    /// Run a complete performance cycle.
    pub async fn run_performance_cycle(&self) -> ActorCoreResult<WorkflowExecution> {
        let start_time = Instant::now();
        let timestamp = current_timestamp_ms();

        // Update state
        {
            let mut state = self.state.write();
            state.status = WorkflowStatus::Testing;
            state.last_test_time = Some(timestamp);
        }

        tracing::info!("Running performance cycle");

        // Run performance tests
        let test_results = self.test_suite.run_all_tests().await?;
        
        // Update state
        {
            let mut state = self.state.write();
            state.status = WorkflowStatus::CheckingRegressions;
        }

        // Generate performance report
        let performance_report = self.profiler.generate_report();
        
        // Check for regressions
        let regression_detected = self.check_for_regressions(&performance_report);
        
        // Check for alerts
        let alerts_triggered = self.check_for_alerts(&performance_report);
        
        // Determine CI/CD status
        let ci_status = self.determine_ci_status(&performance_report);

        // Update workflow state
        {
            let mut state = self.state.write();
            state.status = WorkflowStatus::Reporting;
            state.current_score = performance_report.performance_score;
            
            if performance_report.performance_score < self.config.ci_failure_threshold {
                state.consecutive_failures += 1;
            } else {
                state.consecutive_failures = 0;
            }

            // Update trends
            state.trends = self.calculate_trends(&performance_report);
        }

        // Generate report if enabled
        if self.config.enable_reporting {
            self.generate_workflow_report(&performance_report, &test_results).await?;
        }

        let duration = start_time.elapsed();

        // Create execution record
        let execution = WorkflowExecution {
            timestamp,
            duration,
            test_results,
            performance_report,
            regression_detected,
            alerts_triggered,
            ci_status,
        };

        // Add to history
        {
            let mut history = self.history.write();
            history.push(execution.clone());
            
            // Trim history if it gets too large
            if history.len() > 1000 {
                history.remove(0);
            }
        }

        // Update state
        {
            let mut state = self.state.write();
            state.status = WorkflowStatus::Idle;
        }

        tracing::info!("Performance cycle completed in {:?}", duration);
        Ok(execution)
    }

    /// Schedule regular performance tests.
    async fn schedule_performance_tests(&self) -> ActorCoreResult<()> {
        let config = self.config.clone();
        let workflow = Arc::new(self.clone());

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.test_interval);
            
            loop {
                interval.tick().await;
                
                if let Err(e) = workflow.run_performance_cycle().await {
                    tracing::error!("Performance cycle failed: {}", e);
                    
                    let mut state = workflow.state.write();
                    state.status = WorkflowStatus::Error(e.to_string());
                }
            }
        });

        Ok(())
    }

    /// Check for performance regressions.
    fn check_for_regressions(&self, report: &PerformanceReport) -> bool {
        if !self.config.enable_regression_detection {
            return false;
        }

        !report.performance_regressions.is_empty()
    }

    /// Check for performance alerts.
    fn check_for_alerts(&self, report: &PerformanceReport) -> Vec<PerformanceAlert> {
        let mut alerts = Vec::new();

        if !self.config.enable_alerts {
            return alerts;
        }

        let current_score = report.performance_score;
        if current_score < self.config.alert_threshold {
            alerts.push(PerformanceAlert::ScoreAlert {
                current_score,
                threshold: self.config.alert_threshold,
            });
        }

        let baseline = &self.config.baseline_config;
        
        // Check throughput
        if let Some(throughput) = self.get_current_throughput(report) {
            if throughput < baseline.baseline_throughput {
                alerts.push(PerformanceAlert::ThroughputAlert {
                    current: throughput,
                    baseline: baseline.baseline_throughput,
                });
            }
        }

        // Check latency
        if let Some(latency) = self.get_current_latency(report) {
            if latency > baseline.baseline_latency {
                alerts.push(PerformanceAlert::LatencyAlert {
                    current: latency,
                    threshold: baseline.baseline_latency,
                });
            }
        }

        // Check memory usage
        let memory_usage = report.current_metrics.system.memory_usage;
        if memory_usage > baseline.baseline_memory_usage {
            alerts.push(PerformanceAlert::MemoryAlert {
                current: memory_usage,
                threshold: baseline.baseline_memory_usage,
            });
        }

        // Check cache hit rate
        let cache_hit_rate = report.current_metrics.cache.hit_rate;
        if cache_hit_rate < baseline.baseline_cache_hit_rate {
            alerts.push(PerformanceAlert::CacheHitRateAlert {
                current: cache_hit_rate,
                threshold: baseline.baseline_cache_hit_rate,
            });
        }

        // Check for regressions
        for regression in &report.performance_regressions {
            alerts.push(PerformanceAlert::RegressionAlert {
                degradation: regression.degradation_percentage,
                metric: regression.metric_name.clone(),
            });
        }

        alerts
    }

    /// Determine CI/CD status based on performance.
    fn determine_ci_status(&self, report: &PerformanceReport) -> CiStatus {
        if !self.config.enable_ci_integration {
            return CiStatus::Skipped;
        }

        if report.performance_score >= self.config.ci_failure_threshold {
            CiStatus::Passed
        } else {
            CiStatus::Failed
        }
    }

    /// Calculate performance trends.
    fn calculate_trends(&self, report: &PerformanceReport) -> PerformanceTrends {
        let history = self.history.read();
        
        if history.len() < 2 {
            return PerformanceTrends::default();
        }

        let _recent = &history[history.len() - 1];
        let previous = &history[history.len() - 2];

        let score_trend = report.performance_score - previous.performance_report.performance_score;
        
        // Calculate other trends (simplified)
        PerformanceTrends {
            score_trend,
            throughput_trend: 0.0, // Would need more data
            latency_trend: 0.0,    // Would need more data
            memory_trend: 0.0,     // Would need more data
            cache_hit_rate_trend: 0.0, // Would need more data
        }
    }

    /// Generate workflow report.
    async fn generate_workflow_report(&self, performance_report: &PerformanceReport, test_results: &TestSuiteResults) -> ActorCoreResult<()> {
        let timestamp = current_timestamp_ms();
        
        // Generate comprehensive report
        let report = WorkflowReport {
            generated_at: timestamp,
            workflow_config: self.config.clone(),
            performance_report: performance_report.clone(),
            test_results: test_results.clone(),
            workflow_state: self.get_state(),
            execution_history: self.get_recent_executions(10),
            recommendations: self.generate_optimization_recommendations(performance_report, test_results),
        };

        // Log the report
        tracing::info!("Generated workflow report: {}", serde_json::to_string_pretty(&report)?);

        Ok(())
    }

    /// Get current workflow state.
    pub fn get_state(&self) -> WorkflowState {
        self.state.read().clone()
    }

    /// Get recent execution history.
    pub fn get_recent_executions(&self, count: usize) -> Vec<WorkflowExecution> {
        let history = self.history.read();
        history.iter().rev().take(count).cloned().collect()
    }

    /// Generate optimization recommendations.
    fn generate_optimization_recommendations(&self, report: &PerformanceReport, test_results: &TestSuiteResults) -> Vec<String> {
        let mut recommendations = Vec::new();

        if !self.config.enable_optimization_suggestions {
            return recommendations;
        }

        // Add recommendations from failed tests
        recommendations.extend(test_results.get_recommendations());

        // Add recommendations from performance report
        recommendations.extend(report.recommendations.clone());

        // Add workflow-specific recommendations
        if report.performance_score < self.config.baseline_config.baseline_score {
            recommendations.push("Performance is below baseline. Consider optimizing hot paths and reducing complexity.".to_string());
        }

        if !report.performance_regressions.is_empty() {
            recommendations.push("Performance regressions detected. Review recent changes for performance impact.".to_string());
        }

        let state = self.state.read();
        if state.consecutive_failures > 3 {
            recommendations.push("Multiple consecutive performance failures. Consider immediate investigation.".to_string());
        }

        recommendations.sort();
        recommendations.dedup();
        recommendations
    }

    /// Get current throughput from report.
    fn get_current_throughput(&self, report: &PerformanceReport) -> Option<u64> {
        Some(report.current_metrics.aggregation.aggregations_per_second as u64)
    }

    /// Get current latency from report.
    fn get_current_latency(&self, report: &PerformanceReport) -> Option<u64> {
        Some(report.current_metrics.aggregation.avg_aggregation_time)
    }
}

impl Clone for PerformanceWorkflow {
    fn clone(&self) -> Self {
        Self {
            profiler: PerformanceProfiler::new(ProfilerConfig::default(), self.config_manager.clone()),
            test_suite: PerformanceTestSuite::new_default(self.config_manager.clone()),
            config: self.config.clone(),
            state: Arc::clone(&self.state),
            history: Arc::clone(&self.history),
            config_manager: Arc::clone(&self.config_manager),
        }
    }
}

impl Default for WorkflowState {
    fn default() -> Self {
        Self {
            status: WorkflowStatus::Idle,
            last_test_time: None,
            last_report_time: None,
            current_score: 0.0,
            consecutive_failures: 0,
            last_alert_time: None,
            trends: PerformanceTrends::default(),
        }
    }
}

impl Default for PerformanceTrends {
    fn default() -> Self {
        Self {
            score_trend: 0.0,
            throughput_trend: 0.0,
            latency_trend: 0.0,
            memory_trend: 0.0,
            cache_hit_rate_trend: 0.0,
        }
    }
}

/// Comprehensive workflow report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowReport {
    /// Report generation timestamp
    pub generated_at: u64,
    /// Workflow configuration
    pub workflow_config: WorkflowConfig,
    /// Performance report
    pub performance_report: PerformanceReport,
    /// Test results
    pub test_results: TestSuiteResults,
    /// Current workflow state
    pub workflow_state: WorkflowState,
    /// Recent execution history
    pub execution_history: Vec<WorkflowExecution>,
    /// Optimization recommendations
    pub recommendations: Vec<String>,
}

/// Get current timestamp in milliseconds.
fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}