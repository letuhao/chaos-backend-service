//! Performance profiler and workflow manager for Actor Core.
//!
//! This module provides comprehensive performance profiling capabilities,
//! including automated benchmarking, threshold monitoring, and performance
//! regression detection.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use crate::constants::performance_thresholds;
use crate::ActorCoreResult;

/// Performance profiler that manages benchmarking and threshold monitoring.
pub struct PerformanceProfiler {
    /// Current performance metrics
    metrics: Arc<RwLock<PerformanceMetrics>>,
    /// Performance thresholds
    thresholds: Arc<RwLock<PerformanceThresholds>>,
    /// Historical performance data
    history: Arc<RwLock<Vec<PerformanceSnapshot>>>,
    /// Configuration
    config: ProfilerConfig,
    /// Start time for profiling session
    session_start: Instant,
}

/// Configuration for the performance profiler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerConfig {
    /// Enable continuous profiling
    pub enable_continuous_profiling: bool,
    /// Profiling interval
    pub profiling_interval: Duration,
    /// Maximum history size
    pub max_history_size: usize,
    /// Enable automatic threshold checking
    pub enable_threshold_checking: bool,
    /// Enable performance regression detection
    pub enable_regression_detection: bool,
    /// Regression detection window (number of recent samples)
    pub regression_window: usize,
    /// Performance degradation threshold (percentage)
    pub degradation_threshold: f64,
    /// Enable detailed profiling
    pub enable_detailed_profiling: bool,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            enable_continuous_profiling: true,
            profiling_interval: Duration::from_secs(5),
            max_history_size: 1000,
            enable_threshold_checking: true,
            enable_regression_detection: true,
            regression_window: 10,
            degradation_threshold: 10.0, // 10% degradation
            enable_detailed_profiling: false,
        }
    }
}

/// Performance thresholds for different operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum aggregation time (microseconds)
    pub max_aggregation_time: u64,
    /// Maximum cache operation time (microseconds)
    pub max_cache_time: u64,
    /// Maximum subsystem execution time (microseconds)
    pub max_subsystem_time: u64,
    /// Maximum memory usage per actor (bytes)
    pub max_memory_per_actor: u64,
    /// Maximum cache size (bytes)
    pub max_cache_size: u64,
    /// Maximum throughput threshold (operations per second)
    pub min_throughput: u64,
    /// Maximum latency threshold (microseconds)
    pub max_latency: u64,
    /// Maximum error rate (percentage)
    pub max_error_rate: f64,
    /// Maximum CPU usage (percentage)
    pub max_cpu_usage: f64,
    /// Maximum memory usage (percentage)
    pub max_memory_usage: f64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_aggregation_time: performance_thresholds::MAX_AGGREGATION_TIME,
            max_cache_time: performance_thresholds::MAX_CACHE_TIME,
            max_subsystem_time: performance_thresholds::MAX_SUBSYSTEM_TIME,
            max_memory_per_actor: performance_thresholds::MAX_MEMORY_PER_ACTOR,
            max_cache_size: performance_thresholds::MAX_CACHE_SIZE,
            min_throughput: 1000, // 1000 ops/sec
            max_latency: 10_000, // 10ms
            max_error_rate: 1.0, // 1%
            max_cpu_usage: 80.0, // 80%
            max_memory_usage: 85.0, // 85%
        }
    }
}

/// Current performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Aggregation metrics
    pub aggregation: AggregationMetrics,
    /// Cache metrics
    pub cache: CacheMetrics,
    /// System metrics
    pub system: SystemMetrics,
    /// Error metrics
    pub errors: ErrorMetrics,
    /// Timestamp of last update
    pub last_updated: u64,
}

/// Aggregation-specific performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationMetrics {
    /// Average aggregation time (microseconds)
    pub avg_aggregation_time: u64,
    /// Maximum aggregation time (microseconds)
    pub max_aggregation_time: u64,
    /// Minimum aggregation time (microseconds)
    pub min_aggregation_time: u64,
    /// Total aggregations performed
    pub total_aggregations: u64,
    /// Aggregations per second
    pub aggregations_per_second: f64,
    /// Average contributions per aggregation
    pub avg_contributions_per_aggregation: f64,
    /// Average subsystems per aggregation
    pub avg_subsystems_per_aggregation: f64,
}

/// Cache-specific performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    /// Cache hit rate (percentage)
    pub hit_rate: f64,
    /// Cache miss rate (percentage)
    pub miss_rate: f64,
    /// Average cache operation time (microseconds)
    pub avg_operation_time: u64,
    /// Maximum cache operation time (microseconds)
    pub max_operation_time: u64,
    /// Total cache operations
    pub total_operations: u64,
    /// Cache operations per second
    pub operations_per_second: f64,
    /// Current cache size (bytes)
    pub current_size: u64,
    /// Cache eviction rate
    pub eviction_rate: f64,
}

/// System-level performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Current CPU usage (percentage)
    pub cpu_usage: f64,
    /// Current memory usage (bytes)
    pub memory_usage: u64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// Number of active threads
    pub thread_count: usize,
    /// System uptime (seconds)
    pub uptime: u64,
    /// Average load
    pub load_average: f64,
}

/// Error-related performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    /// Total errors
    pub total_errors: u64,
    /// Error rate (percentage)
    pub error_rate: f64,
    /// Errors per second
    pub errors_per_second: f64,
    /// Error types breakdown
    pub error_types: HashMap<String, u64>,
    /// Last error timestamp
    pub last_error_time: Option<u64>,
}

/// A snapshot of performance at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    /// Timestamp
    pub timestamp: u64,
    /// Performance metrics
    pub metrics: PerformanceMetrics,
    /// Threshold violations
    pub violations: Vec<ThresholdViolation>,
    /// Performance score (0-100)
    pub performance_score: f64,
}

/// A threshold violation record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdViolation {
    /// Threshold name
    pub threshold_name: String,
    /// Actual value
    pub actual_value: f64,
    /// Threshold value
    pub threshold_value: f64,
    /// Severity level
    pub severity: ViolationSeverity,
    /// Timestamp
    pub timestamp: u64,
}

/// Severity levels for threshold violations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViolationSeverity {
    /// Warning level violation
    Warning,
    /// Critical level violation
    Critical,
    /// Fatal level violation
    Fatal,
}

/// Performance test result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTestResult {
    /// Test name
    pub test_name: String,
    /// Test passed
    pub passed: bool,
    /// Performance score (0-100)
    pub score: f64,
    /// Threshold violations
    pub violations: Vec<ThresholdViolation>,
    /// Detailed metrics
    pub metrics: PerformanceMetrics,
    /// Test duration
    pub duration: Duration,
    /// Recommendations
    pub recommendations: Vec<String>,
}

impl PerformanceProfiler {
    /// Create a new performance profiler.
    pub fn new(config: ProfilerConfig) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
            thresholds: Arc::new(RwLock::new(PerformanceThresholds::default())),
            history: Arc::new(RwLock::new(Vec::new())),
            config,
            session_start: Instant::now(),
        }
    }

    /// Create a new profiler with default configuration.
    pub fn new_default() -> Self {
        Self::new(ProfilerConfig::default())
    }

    /// Update performance metrics.
    pub fn update_metrics(&self, metrics: PerformanceMetrics) {
        let mut current_metrics = self.metrics.write();
        *current_metrics = metrics;

        // Add to history if continuous profiling is enabled
        if self.config.enable_continuous_profiling {
            self.add_to_history();
        }

        // Check thresholds if enabled
        if self.config.enable_threshold_checking {
            self.check_thresholds();
        }
    }

    /// Get current performance metrics.
    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.read().clone()
    }

    /// Get performance thresholds.
    pub fn get_thresholds(&self) -> PerformanceThresholds {
        self.thresholds.read().clone()
    }

    /// Update performance thresholds.
    pub fn update_thresholds(&self, thresholds: PerformanceThresholds) {
        let mut current_thresholds = self.thresholds.write();
        *current_thresholds = thresholds;
    }

    /// Run a performance test.
    pub fn run_performance_test<F>(
        &self,
        test_name: &str,
        test_fn: F,
    ) -> ActorCoreResult<PerformanceTestResult>
    where
        F: FnOnce() -> ActorCoreResult<()>,
    {
        let start_time = Instant::now();
        let _start_metrics = self.get_metrics();

        // Run the test
        let test_result = test_fn();

        let end_time = Instant::now();
        let end_metrics = self.get_metrics();
        let duration = end_time.duration_since(start_time);

        // Calculate performance metrics
        let violations = self.check_thresholds_for_metrics(&end_metrics);
        let score = self.calculate_performance_score(&end_metrics, &violations);
        let recommendations = self.generate_recommendations(&violations);

        Ok(PerformanceTestResult {
            test_name: test_name.to_string(),
            passed: test_result.is_ok() && violations.is_empty(),
            score,
            violations,
            metrics: end_metrics,
            duration,
            recommendations,
        })
    }

    /// Get performance history.
    pub fn get_history(&self) -> Vec<PerformanceSnapshot> {
        self.history.read().clone()
    }

    /// Clear performance history.
    pub fn clear_history(&self) {
        let mut history = self.history.write();
        history.clear();
    }

    /// Check for performance regressions.
    pub fn check_for_regressions(&self) -> Vec<PerformanceRegression> {
        let history = self.history.read();
        if history.len() < self.config.regression_window {
            return Vec::new();
        }

        let recent_samples: Vec<&PerformanceSnapshot> = history
            .iter()
            .rev()
            .take(self.config.regression_window)
            .collect();

        let mut regressions = Vec::new();

        // Check for degradation in key metrics
        for metric_name in ["aggregation_time", "cache_hit_rate", "throughput"] {
            if let Some(regression) = self.detect_metric_regression(metric_name, &recent_samples) {
                regressions.push(regression);
            }
        }

        regressions
    }

    /// Generate a performance report.
    pub fn generate_report(&self) -> PerformanceReport {
        let metrics = self.get_metrics();
        let history = self.get_history();
        let violations = self.check_thresholds_for_metrics(&metrics);
        let regressions = self.check_for_regressions();
        let score = self.calculate_performance_score(&metrics, &violations);

        PerformanceReport {
            generated_at: current_timestamp_ms(),
            session_duration: self.session_start.elapsed(),
            current_metrics: metrics,
            performance_score: score,
            threshold_violations: violations.clone(),
            performance_regressions: regressions,
            history_summary: self.summarize_history(&history),
            recommendations: self.generate_recommendations(&violations),
        }
    }

    /// Add current metrics to history.
    fn add_to_history(&self) {
        let metrics = self.get_metrics();
        let violations = self.check_thresholds_for_metrics(&metrics);
        let score = self.calculate_performance_score(&metrics, &violations);

        let snapshot = PerformanceSnapshot {
            timestamp: current_timestamp_ms(),
            metrics,
            violations,
            performance_score: score,
        };

        let mut history = self.history.write();
        history.push(snapshot);

        // Trim history if it exceeds max size
        if history.len() > self.config.max_history_size {
            history.remove(0);
        }
    }

    /// Check current metrics against thresholds.
    fn check_thresholds(&self) {
        let metrics = self.get_metrics();
        let violations = self.check_thresholds_for_metrics(&metrics);
        
        if !violations.is_empty() {
            // Log violations or trigger alerts
            tracing::warn!(
                "Performance threshold violations detected: {} violations",
                violations.len()
            );
        }
    }

    /// Check metrics against thresholds and return violations.
    fn check_thresholds_for_metrics(&self, metrics: &PerformanceMetrics) -> Vec<ThresholdViolation> {
        let thresholds = self.get_thresholds();
        let mut violations = Vec::new();
        let timestamp = current_timestamp_ms();

        // Check aggregation time
        if metrics.aggregation.avg_aggregation_time > thresholds.max_aggregation_time {
            violations.push(ThresholdViolation {
                threshold_name: "max_aggregation_time".to_string(),
                actual_value: metrics.aggregation.avg_aggregation_time as f64,
                threshold_value: thresholds.max_aggregation_time as f64,
                severity: ViolationSeverity::Critical,
                timestamp,
            });
        }

        // Check cache operation time
        if metrics.cache.avg_operation_time > thresholds.max_cache_time {
            violations.push(ThresholdViolation {
                threshold_name: "max_cache_time".to_string(),
                actual_value: metrics.cache.avg_operation_time as f64,
                threshold_value: thresholds.max_cache_time as f64,
                severity: ViolationSeverity::Warning,
                timestamp,
            });
        }

        // Check throughput
        if metrics.aggregation.aggregations_per_second < thresholds.min_throughput as f64 {
            violations.push(ThresholdViolation {
                threshold_name: "min_throughput".to_string(),
                actual_value: metrics.aggregation.aggregations_per_second,
                threshold_value: thresholds.min_throughput as f64,
                severity: ViolationSeverity::Critical,
                timestamp,
            });
        }

        // Check error rate
        if metrics.errors.error_rate > thresholds.max_error_rate {
            violations.push(ThresholdViolation {
                threshold_name: "max_error_rate".to_string(),
                actual_value: metrics.errors.error_rate,
                threshold_value: thresholds.max_error_rate,
                severity: ViolationSeverity::Fatal,
                timestamp,
            });
        }

        violations
    }

    /// Calculate overall performance score (0-100).
    fn calculate_performance_score(&self, metrics: &PerformanceMetrics, violations: &[ThresholdViolation]) -> f64 {
        let mut score = 100.0;

        // Deduct points for violations
        for violation in violations {
            let deduction = match violation.severity {
                ViolationSeverity::Warning => 5.0,
                ViolationSeverity::Critical => 15.0,
                ViolationSeverity::Fatal => 30.0,
            };
            score -= deduction;
        }

        // Deduct points for poor metrics
        if metrics.cache.hit_rate < 90.0 {
            score -= (90.0 - metrics.cache.hit_rate) * 0.5;
        }

        if metrics.system.cpu_usage > 80.0 {
            score -= (metrics.system.cpu_usage - 80.0) * 0.3;
        }

        if metrics.system.memory_usage_percent > 85.0 {
            score -= (metrics.system.memory_usage_percent - 85.0) * 0.4;
        }

        score.max(0.0).min(100.0)
    }

    /// Generate recommendations based on violations.
    fn generate_recommendations(&self, violations: &[ThresholdViolation]) -> Vec<String> {
        let mut recommendations = Vec::new();

        for violation in violations {
            match violation.threshold_name.as_str() {
                "max_aggregation_time" => {
                    recommendations.push("Consider optimizing aggregation algorithms or reducing subsystem complexity".to_string());
                }
                "max_cache_time" => {
                    recommendations.push("Consider increasing cache size or optimizing cache access patterns".to_string());
                }
                "min_throughput" => {
                    recommendations.push("Consider increasing concurrency or optimizing hot paths".to_string());
                }
                "max_error_rate" => {
                    recommendations.push("Investigate and fix error sources immediately".to_string());
                }
                _ => {
                    recommendations.push(format!("Review {} threshold configuration", violation.threshold_name));
                }
            }
        }

        recommendations
    }

    /// Detect performance regression for a specific metric.
    fn detect_metric_regression(&self, metric_name: &str, samples: &[&PerformanceSnapshot]) -> Option<PerformanceRegression> {
        if samples.len() < 2 {
            return None;
        }

        let recent_avg = self.calculate_average_metric(metric_name, &samples[..samples.len()/2]);
        let older_avg = self.calculate_average_metric(metric_name, &samples[samples.len()/2..]);

        if older_avg == 0.0 {
            return None;
        }

        let degradation = ((recent_avg - older_avg) / older_avg) * 100.0;

        if degradation.abs() > self.config.degradation_threshold {
            Some(PerformanceRegression {
                metric_name: metric_name.to_string(),
                degradation_percentage: degradation,
                recent_average: recent_avg,
                historical_average: older_avg,
                severity: if degradation > 20.0 {
                    ViolationSeverity::Critical
                } else {
                    ViolationSeverity::Warning
                },
                detected_at: current_timestamp_ms(),
            })
        } else {
            None
        }
    }

    /// Calculate average value for a specific metric.
    fn calculate_average_metric(&self, metric_name: &str, samples: &[&PerformanceSnapshot]) -> f64 {
        let mut sum = 0.0;
        let mut count = 0;

        for sample in samples {
            let value = match metric_name {
                "aggregation_time" => sample.metrics.aggregation.avg_aggregation_time as f64,
                "cache_hit_rate" => sample.metrics.cache.hit_rate,
                "throughput" => sample.metrics.aggregation.aggregations_per_second,
                _ => continue,
            };
            sum += value;
            count += 1;
        }

        if count > 0 { sum / count as f64 } else { 0.0 }
    }

    /// Summarize performance history.
    fn summarize_history(&self, history: &[PerformanceSnapshot]) -> HistorySummary {
        if history.is_empty() {
            return HistorySummary::default();
        }

        let scores: Vec<f64> = history.iter().map(|s| s.performance_score).collect();
        let avg_score = scores.iter().sum::<f64>() / scores.len() as f64;
        let min_score = scores.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_score = scores.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let total_violations = history.iter().map(|s| s.violations.len()).sum();
        let avg_violations = total_violations as f64 / history.len() as f64;

        HistorySummary {
            total_samples: history.len(),
            average_performance_score: avg_score,
            min_performance_score: min_score,
            max_performance_score: max_score,
            total_threshold_violations: total_violations,
            average_violations_per_sample: avg_violations,
            time_span: if history.len() > 1 {
                history.last().unwrap().timestamp - history.first().unwrap().timestamp
            } else {
                0
            },
        }
    }
}

/// Performance regression detected by the profiler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRegression {
    /// Metric name
    pub metric_name: String,
    /// Degradation percentage
    pub degradation_percentage: f64,
    /// Recent average value
    pub recent_average: f64,
    /// Historical average value
    pub historical_average: f64,
    /// Severity level
    pub severity: ViolationSeverity,
    /// Detection timestamp
    pub detected_at: u64,
}

/// Comprehensive performance report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    /// Report generation timestamp
    pub generated_at: u64,
    /// Session duration
    pub session_duration: Duration,
    /// Current performance metrics
    pub current_metrics: PerformanceMetrics,
    /// Overall performance score
    pub performance_score: f64,
    /// Current threshold violations
    pub threshold_violations: Vec<ThresholdViolation>,
    /// Performance regressions detected
    pub performance_regressions: Vec<PerformanceRegression>,
    /// History summary
    pub history_summary: HistorySummary,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Summary of performance history.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistorySummary {
    /// Total number of samples
    pub total_samples: usize,
    /// Average performance score
    pub average_performance_score: f64,
    /// Minimum performance score
    pub min_performance_score: f64,
    /// Maximum performance score
    pub max_performance_score: f64,
    /// Total threshold violations
    pub total_threshold_violations: usize,
    /// Average violations per sample
    pub average_violations_per_sample: f64,
    /// Time span covered (milliseconds)
    pub time_span: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            aggregation: AggregationMetrics::default(),
            cache: CacheMetrics::default(),
            system: SystemMetrics::default(),
            errors: ErrorMetrics::default(),
            last_updated: current_timestamp_ms(),
        }
    }
}

impl Default for AggregationMetrics {
    fn default() -> Self {
        Self {
            avg_aggregation_time: 0,
            max_aggregation_time: 0,
            min_aggregation_time: u64::MAX,
            total_aggregations: 0,
            aggregations_per_second: 0.0,
            avg_contributions_per_aggregation: 0.0,
            avg_subsystems_per_aggregation: 0.0,
        }
    }
}

impl Default for CacheMetrics {
    fn default() -> Self {
        Self {
            hit_rate: 0.0,
            miss_rate: 0.0,
            avg_operation_time: 0,
            max_operation_time: 0,
            total_operations: 0,
            operations_per_second: 0.0,
            current_size: 0,
            eviction_rate: 0.0,
        }
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            memory_usage_percent: 0.0,
            thread_count: 0,
            uptime: 0,
            load_average: 0.0,
        }
    }
}

impl Default for ErrorMetrics {
    fn default() -> Self {
        Self {
            total_errors: 0,
            error_rate: 0.0,
            errors_per_second: 0.0,
            error_types: HashMap::new(),
            last_error_time: None,
        }
    }
}

/// Get current timestamp in milliseconds.
fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}