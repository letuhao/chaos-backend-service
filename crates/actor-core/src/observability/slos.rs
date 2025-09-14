//! Service Level Objectives (SLOs) for Actor Core.
//!
//! This module defines and tracks Service Level Objectives for the Actor Core system,
//! providing automated monitoring and alerting capabilities.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};

use crate::ActorCoreResult;
use crate::ActorCoreError;

/// Service Level Objective definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SLO {
    /// Unique identifier for the SLO
    pub id: String,
    /// Human-readable name for the SLO
    pub name: String,
    /// Description of what this SLO measures
    pub description: String,
    /// The target success rate (0.0 to 1.0)
    pub target_success_rate: f64,
    /// The time window over which to measure
    pub measurement_window: Duration,
    /// The type of metric being measured
    pub metric_type: SLOMetricType,
    /// Labels for filtering and grouping
    pub labels: HashMap<String, String>,
    /// Whether this SLO is currently enabled
    pub enabled: bool,
    /// The severity level for alerts
    pub severity: SLOSeverity,
    /// The threshold for triggering alerts (as a percentage of target)
    pub alert_threshold: f64,
}

/// Types of metrics that can be measured by SLOs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SLOMetricType {
    /// Availability - percentage of successful requests
    Availability,
    /// Latency - percentage of requests under a threshold
    Latency { threshold_ms: u64 },
    /// Error rate - percentage of requests without errors
    ErrorRate,
    /// Throughput - requests per second
    Throughput,
    /// Custom metric with a specific name
    Custom { metric_name: String },
}

/// Severity levels for SLO violations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SLOSeverity {
    /// Low severity - informational
    Low,
    /// Medium severity - warning
    Medium,
    /// High severity - critical
    High,
    /// Critical severity - immediate attention required
    Critical,
}

/// SLO measurement result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SLOMeasurement {
    /// The SLO ID this measurement is for
    pub slo_id: String,
    /// The timestamp of this measurement
    pub timestamp: SystemTime,
    /// The current success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// The number of successful events
    pub successful_events: u64,
    /// The total number of events
    pub total_events: u64,
    /// The current error budget remaining (0.0 to 1.0)
    pub error_budget_remaining: f64,
    /// Whether this measurement indicates a violation
    pub is_violation: bool,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// SLO violation alert.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SLOViolation {
    /// The SLO ID that was violated
    pub slo_id: String,
    /// The timestamp when the violation occurred
    pub timestamp: SystemTime,
    /// The severity of the violation
    pub severity: SLOSeverity,
    /// The current success rate
    pub current_success_rate: f64,
    /// The target success rate
    pub target_success_rate: f64,
    /// The error budget consumed
    pub error_budget_consumed: f64,
    /// The error budget remaining
    pub error_budget_remaining: f64,
    /// Additional context about the violation
    pub context: HashMap<String, serde_json::Value>,
}

/// SLO manager for tracking and evaluating Service Level Objectives.
pub struct SLOManager {
    /// Registered SLOs
    slos: HashMap<String, SLO>,
    /// Recent measurements for each SLO
    measurements: HashMap<String, Vec<SLOMeasurement>>,
    /// Violation handlers
    violation_handlers: Vec<Box<dyn SLOViolationHandler + Send + Sync>>,
    /// Measurement history retention period
    retention_period: Duration,
}

/// Handler for SLO violations.
pub trait SLOViolationHandler: Send + Sync {
    /// Handle an SLO violation.
    fn handle_violation(&self, violation: &SLOViolation) -> ActorCoreResult<()>;
    
    /// Get the handler name for identification.
    fn name(&self) -> &str;
}

impl SLOManager {
    /// Create a new SLO manager.
    pub fn new() -> Self {
        Self {
            slos: HashMap::new(),
            measurements: HashMap::new(),
            violation_handlers: Vec::new(),
            retention_period: Duration::from_secs(24 * 60 * 60), // 24 hours
        }
    }

    /// Register an SLO.
    pub fn register_slo(&mut self, slo: SLO) -> ActorCoreResult<()> {
        if slo.target_success_rate < 0.0 || slo.target_success_rate > 1.0 {
            return Err(ActorCoreError::InvalidInput(
                "Target success rate must be between 0.0 and 1.0".to_string()
            ));
        }

        if slo.alert_threshold < 0.0 || slo.alert_threshold > 1.0 {
            return Err(ActorCoreError::InvalidInput(
                "Alert threshold must be between 0.0 and 1.0".to_string()
            ));
        }

        info!(
            slo_id = %slo.id,
            slo_name = %slo.name,
            target_success_rate = slo.target_success_rate,
            "Registered new SLO"
        );

        self.slos.insert(slo.id.clone(), slo);
        let last_key = self.slos.keys().last().unwrap();
        self.measurements.insert(self.slos[last_key].id.clone(), Vec::new());
        Ok(())
    }

    /// Unregister an SLO.
    pub fn unregister_slo(&mut self, slo_id: &str) -> ActorCoreResult<()> {
        if self.slos.remove(slo_id).is_some() {
            self.measurements.remove(slo_id);
            info!(slo_id = %slo_id, "Unregistered SLO");
            Ok(())
        } else {
            Err(ActorCoreError::InvalidInput(format!("SLO '{}' not found", slo_id)))
        }
    }

    /// Get an SLO by ID.
    pub fn get_slo(&self, slo_id: &str) -> Option<&SLO> {
        self.slos.get(slo_id)
    }

    /// List all registered SLOs.
    pub fn list_slos(&self) -> Vec<&SLO> {
        self.slos.values().collect()
    }

    /// Record an event for SLO measurement.
    pub fn record_event(&mut self, slo_id: &str, success: bool, metadata: Option<HashMap<String, serde_json::Value>>) -> ActorCoreResult<()> {
        let slo = self.slos.get(slo_id)
            .ok_or_else(|| ActorCoreError::InvalidInput(format!("SLO '{}' not found", slo_id)))?;

        if !slo.enabled {
            return Ok(());
        }

        // Create a measurement entry
        let measurement = SLOMeasurement {
            slo_id: slo_id.to_string(),
            timestamp: SystemTime::now(),
            success_rate: if success { 1.0 } else { 0.0 },
            successful_events: if success { 1 } else { 0 },
            total_events: 1,
            error_budget_remaining: 1.0 - (if success { 0.0 } else { 1.0 - slo.target_success_rate }),
            is_violation: !success && slo.target_success_rate > 0.0,
            metadata: metadata.unwrap_or_default(),
        };

        // Add to measurements
        self.measurements.entry(slo_id.to_string())
            .or_insert_with(Vec::new)
            .push(measurement);

        // Clean up old measurements
        self.cleanup_old_measurements();

        // Evaluate SLO
        self.evaluate_slo(slo_id)?;

        Ok(())
    }

    /// Record multiple events for batch processing.
    pub fn record_events(&mut self, slo_id: &str, events: Vec<(bool, Option<HashMap<String, serde_json::Value>>)>) -> ActorCoreResult<()> {
        let slo = self.slos.get(slo_id)
            .ok_or_else(|| ActorCoreError::InvalidInput(format!("SLO '{}' not found", slo_id)))?;

        if !slo.enabled {
            return Ok(());
        }

        let successful_events = events.iter().filter(|(success, _)| *success).count() as u64;
        let total_events = events.len() as u64;
        let success_rate = if total_events > 0 {
            successful_events as f64 / total_events as f64
        } else {
            1.0
        };

        let measurement = SLOMeasurement {
            slo_id: slo_id.to_string(),
            timestamp: SystemTime::now(),
            success_rate,
            successful_events,
            total_events,
            error_budget_remaining: 1.0 - ((1.0 - success_rate) / (1.0 - slo.target_success_rate)),
            is_violation: success_rate < slo.target_success_rate,
            metadata: HashMap::new(),
        };

        self.measurements.entry(slo_id.to_string())
            .or_insert_with(Vec::new)
            .push(measurement);

        self.cleanup_old_measurements();
        self.evaluate_slo(slo_id)?;

        Ok(())
    }

    /// Get recent measurements for an SLO.
    pub fn get_measurements(&self, slo_id: &str, limit: Option<usize>) -> Vec<&SLOMeasurement> {
        if let Some(measurements) = self.measurements.get(slo_id) {
            let mut sorted_measurements: Vec<_> = measurements.iter().collect();
            sorted_measurements.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            
            if let Some(limit) = limit {
                sorted_measurements.truncate(limit);
            }
            
            sorted_measurements
        } else {
            Vec::new()
        }
    }

    /// Calculate current SLO status.
    pub fn calculate_slo_status(&self, slo_id: &str) -> ActorCoreResult<SLOStatus> {
        let slo = self.slos.get(slo_id)
            .ok_or_else(|| ActorCoreError::InvalidInput(format!("SLO '{}' not found", slo_id)))?;

        let measurements = self.measurements.get(slo_id)
            .ok_or_else(|| ActorCoreError::InvalidInput(format!("No measurements found for SLO '{}'", slo_id)))?;

        let now = SystemTime::now();
        let window_start = now - slo.measurement_window;
        
        // Filter measurements within the time window
        let recent_measurements: Vec<_> = measurements
            .iter()
            .filter(|m| m.timestamp >= window_start)
            .collect();

        if recent_measurements.is_empty() {
            return Ok(SLOStatus {
                slo_id: slo_id.to_string(),
                current_success_rate: 1.0,
                target_success_rate: slo.target_success_rate,
                error_budget_remaining: 1.0,
                error_budget_consumed: 0.0,
                is_healthy: true,
                last_measurement: None,
                measurement_count: 0,
            });
        }

        let total_successful = recent_measurements.iter().map(|m| m.successful_events).sum::<u64>();
        let total_events = recent_measurements.iter().map(|m| m.total_events).sum::<u64>();
        
        let current_success_rate = if total_events > 0 {
            total_successful as f64 / total_events as f64
        } else {
            1.0
        };

        let error_budget_consumed = (1.0 - current_success_rate) / (1.0 - slo.target_success_rate);
        let error_budget_remaining = 1.0 - error_budget_consumed;
        let is_healthy = current_success_rate >= slo.target_success_rate;

        Ok(SLOStatus {
            slo_id: slo_id.to_string(),
            current_success_rate,
            target_success_rate: slo.target_success_rate,
            error_budget_remaining,
            error_budget_consumed,
            is_healthy,
            last_measurement: recent_measurements.first().map(|m| m.timestamp),
            measurement_count: recent_measurements.len(),
        })
    }

    /// Register a violation handler.
    pub fn register_violation_handler(&mut self, handler: Box<dyn SLOViolationHandler + Send + Sync>) {
        info!(handler_name = %handler.name(), "Registered SLO violation handler");
        self.violation_handlers.push(handler);
    }

    /// Set the retention period for measurements.
    pub fn set_retention_period(&mut self, period: Duration) {
        self.retention_period = period;
    }

    /// Evaluate an SLO and trigger violations if necessary.
    fn evaluate_slo(&self, slo_id: &str) -> ActorCoreResult<()> {
        let slo = self.slos.get(slo_id)
            .ok_or_else(|| ActorCoreError::InvalidInput(format!("SLO '{}' not found", slo_id)))?;

        let status = self.calculate_slo_status(slo_id)?;

        if !status.is_healthy {
            let violation = SLOViolation {
                slo_id: slo_id.to_string(),
                timestamp: SystemTime::now(),
                severity: slo.severity.clone(),
                current_success_rate: status.current_success_rate,
                target_success_rate: status.target_success_rate,
                error_budget_consumed: status.error_budget_consumed,
                error_budget_remaining: status.error_budget_remaining,
                context: HashMap::new(),
            };

            warn!(
                slo_id = %slo_id,
                current_success_rate = status.current_success_rate,
                target_success_rate = status.target_success_rate,
                error_budget_remaining = status.error_budget_remaining,
                "SLO violation detected"
            );

            // Notify all violation handlers
            for handler in &self.violation_handlers {
                if let Err(e) = handler.handle_violation(&violation) {
                    error!(
                        handler_name = %handler.name(),
                        error = %e,
                        "Failed to handle SLO violation"
                    );
                }
            }
        }

        Ok(())
    }

    /// Clean up old measurements beyond the retention period.
    fn cleanup_old_measurements(&mut self) {
        let cutoff_time = SystemTime::now() - self.retention_period;
        
        for measurements in self.measurements.values_mut() {
            measurements.retain(|m| m.timestamp > cutoff_time);
        }
    }
}

/// Current status of an SLO.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SLOStatus {
    /// The SLO ID
    pub slo_id: String,
    /// Current success rate
    pub current_success_rate: f64,
    /// Target success rate
    pub target_success_rate: f64,
    /// Error budget remaining (0.0 to 1.0)
    pub error_budget_remaining: f64,
    /// Error budget consumed (0.0 to 1.0)
    pub error_budget_consumed: f64,
    /// Whether the SLO is currently healthy
    pub is_healthy: bool,
    /// Timestamp of the last measurement
    pub last_measurement: Option<SystemTime>,
    /// Number of measurements in the current window
    pub measurement_count: usize,
}

/// Default SLO definitions for common Actor Core metrics.
pub mod default_slos {
    use super::*;

    /// Create default SLOs for Actor Core.
    pub fn create_default_slos() -> Vec<SLO> {
        vec![
            // Availability SLO
            SLO {
                id: "actor_core_availability".to_string(),
                name: "Actor Core Availability".to_string(),
                description: "Percentage of successful actor stat resolutions".to_string(),
                target_success_rate: 0.999, // 99.9% availability
                measurement_window: Duration::from_secs(300), // 5 minutes
                metric_type: SLOMetricType::Availability,
                labels: {
                    let mut labels = HashMap::new();
                    labels.insert("service".to_string(), "actor_core".to_string());
                    labels.insert("component".to_string(), "aggregator".to_string());
                    labels
                },
                enabled: true,
                severity: SLOSeverity::High,
                alert_threshold: 0.95, // Alert when success rate drops below 95% of target
            },
            
            // Latency SLO
            SLO {
                id: "actor_core_latency".to_string(),
                name: "Actor Core Latency".to_string(),
                description: "Percentage of actor stat resolutions under 100ms".to_string(),
                target_success_rate: 0.95, // 95% under 100ms
                measurement_window: Duration::from_secs(300), // 5 minutes
                metric_type: SLOMetricType::Latency { threshold_ms: 100 },
                labels: {
                    let mut labels = HashMap::new();
                    labels.insert("service".to_string(), "actor_core".to_string());
                    labels.insert("component".to_string(), "aggregator".to_string());
                    labels
                },
                enabled: true,
                severity: SLOSeverity::Medium,
                alert_threshold: 0.9, // Alert when latency success rate drops below 90% of target
            },
            
            // Error rate SLO
            SLO {
                id: "actor_core_error_rate".to_string(),
                name: "Actor Core Error Rate".to_string(),
                description: "Percentage of actor stat resolutions without errors".to_string(),
                target_success_rate: 0.99, // 99% success rate
                measurement_window: Duration::from_secs(300), // 5 minutes
                metric_type: SLOMetricType::ErrorRate,
                labels: {
                    let mut labels = HashMap::new();
                    labels.insert("service".to_string(), "actor_core".to_string());
                    labels.insert("component".to_string(), "aggregator".to_string());
                    labels
                },
                enabled: true,
                severity: SLOSeverity::High,
                alert_threshold: 0.95, // Alert when error rate exceeds 5% of target
            },
            
            // Cache hit rate SLO
            SLO {
                id: "actor_core_cache_hit_rate".to_string(),
                name: "Actor Core Cache Hit Rate".to_string(),
                description: "Percentage of cache hits for actor stat resolutions".to_string(),
                target_success_rate: 0.8, // 80% cache hit rate
                measurement_window: Duration::from_secs(300), // 5 minutes
                metric_type: SLOMetricType::Custom { metric_name: "cache_hit_rate".to_string() },
                labels: {
                    let mut labels = HashMap::new();
                    labels.insert("service".to_string(), "actor_core".to_string());
                    labels.insert("component".to_string(), "cache".to_string());
                    labels
                },
                enabled: true,
                severity: SLOSeverity::Low,
                alert_threshold: 0.8, // Alert when cache hit rate drops below 80% of target
            },
        ]
    }
}

/// Console-based SLO violation handler for development and testing.
pub struct ConsoleSLOViolationHandler;

impl SLOViolationHandler for ConsoleSLOViolationHandler {
    fn handle_violation(&self, violation: &SLOViolation) -> ActorCoreResult<()> {
        println!(
            "🚨 SLO VIOLATION: {} - Current: {:.2}%, Target: {:.2}%, Error Budget: {:.2}%",
            violation.slo_id,
            violation.current_success_rate * 100.0,
            violation.target_success_rate * 100.0,
            violation.error_budget_remaining * 100.0
        );
        Ok(())
    }

    fn name(&self) -> &str {
        "console"
    }
}