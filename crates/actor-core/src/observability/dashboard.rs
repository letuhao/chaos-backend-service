//! Observability dashboard for Actor Core.
//!
//! This module provides a dashboard system for monitoring and visualizing
//! Actor Core metrics, SLOs, and system health.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, debug};

use crate::ActorCoreResult;
use crate::ActorCoreError;

use super::slos::{SLOManager, SLOStatus};
use super::metrics_collector::{MetricsCollector, MetricsSnapshot};

/// Dashboard for monitoring Actor Core system health.
pub struct ObservabilityDashboard {
    /// SLO manager for tracking Service Level Objectives
    slo_manager: Arc<SLOManager>,
    /// Metrics collector for gathering performance data
    metrics_collector: Arc<MetricsCollector>,
    /// Dashboard configuration
    config: DashboardConfig,
    /// Last dashboard update time
    last_update: SystemTime,
}

/// Dashboard configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Refresh interval for the dashboard
    pub refresh_interval: Duration,
    /// Whether to include detailed metrics
    pub include_detailed_metrics: bool,
    /// Whether to include SLO status
    pub include_slo_status: bool,
    /// Whether to include system health
    pub include_system_health: bool,
    /// Maximum number of recent measurements to show
    pub max_recent_measurements: usize,
    /// Whether to enable auto-refresh
    pub auto_refresh: bool,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            refresh_interval: Duration::from_secs(30),
            include_detailed_metrics: true,
            include_slo_status: true,
            include_system_health: true,
            max_recent_measurements: 10,
            auto_refresh: true,
        }
    }
}

/// Dashboard status snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStatus {
    /// Timestamp when this status was generated
    pub timestamp: SystemTime,
    /// Overall system health status
    pub system_health: SystemHealthStatus,
    /// SLO statuses
    pub slo_statuses: HashMap<String, SLOStatus>,
    /// Metrics snapshot
    pub metrics: Option<MetricsSnapshot>,
    /// System information
    pub system_info: SystemInfo,
    /// Recent alerts
    pub recent_alerts: Vec<SystemAlert>,
}

/// System health status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SystemHealthStatus {
    /// System is healthy
    Healthy,
    /// System has warnings
    Warning,
    /// System has critical issues
    Critical,
    /// System is down
    Down,
}

/// System information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Actor Core version
    pub version: String,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Number of registered SLOs
    pub slo_count: usize,
    /// Number of registered metrics
    pub metric_count: usize,
    /// System load information
    pub load_info: LoadInfo,
}

/// System load information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadInfo {
    /// CPU usage percentage (0.0 to 100.0)
    pub cpu_usage_percent: f64,
    /// Memory usage percentage (0.0 to 100.0)
    pub memory_usage_percent: f64,
    /// Number of active actors
    pub active_actors: u64,
    /// Number of active subsystems
    pub active_subsystems: u64,
}

/// System alert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAlert {
    /// Alert ID
    pub id: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Alert timestamp
    pub timestamp: SystemTime,
    /// Alert source
    pub source: String,
    /// Whether the alert is active
    pub active: bool,
}

/// Alert severity levels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Information level
    Info,
    /// Warning level
    Warning,
    /// Error level
    Error,
    /// Critical level
    Critical,
}

impl ObservabilityDashboard {
    /// Create a new observability dashboard.
    pub fn new(
        slo_manager: Arc<SLOManager>,
        metrics_collector: Arc<MetricsCollector>,
        config: DashboardConfig,
    ) -> Self {
        Self {
            slo_manager,
            metrics_collector,
            config,
            last_update: SystemTime::now(),
        }
    }

    /// Get the current dashboard status.
    pub async fn get_status(&self) -> ActorCoreResult<DashboardStatus> {
        let timestamp = SystemTime::now();
        
        // Get SLO statuses
        let mut slo_statuses = HashMap::new();
        if self.config.include_slo_status {
            for slo in self.slo_manager.list_slos() {
                if let Ok(status) = self.slo_manager.calculate_slo_status(&slo.id) {
                    slo_statuses.insert(slo.id.clone(), status);
                }
            }
        }

        // Get metrics snapshot
        let metrics = if self.config.include_detailed_metrics {
            Some(self.metrics_collector.snapshot())
        } else {
            None
        };

        // Determine system health
        let system_health = self.determine_system_health(&slo_statuses).await;

        // Get system information
        let system_info = self.get_system_info().await;

        // Get recent alerts
        let recent_alerts = self.get_recent_alerts().await;

        Ok(DashboardStatus {
            timestamp,
            system_health,
            slo_statuses,
            metrics,
            system_info,
            recent_alerts,
        })
    }

    /// Get a health summary for the dashboard.
    pub async fn get_health_summary(&self) -> ActorCoreResult<HealthSummary> {
        let status = self.get_status().await?;
        
        let total_slos = status.slo_statuses.len();
        let healthy_slos = status.slo_statuses.values()
            .filter(|slo| slo.is_healthy)
            .count();
        
        let slo_health_percentage = if total_slos > 0 {
            (healthy_slos as f64 / total_slos as f64) * 100.0
        } else {
            100.0
        };

        Ok(HealthSummary {
            overall_health: status.system_health,
            slo_health_percentage,
            healthy_slos,
            total_slos,
            active_alerts: status.recent_alerts.iter().filter(|a| a.active).count(),
            uptime_seconds: status.system_info.uptime_seconds,
        })
    }

    /// Generate a dashboard report in text format.
    pub async fn generate_text_report(&self) -> ActorCoreResult<String> {
        let status = self.get_status().await?;
        let health_summary = self.get_health_summary().await?;
        
        let mut report = String::new();
        
        // Header
        report.push_str("🎮 Actor Core Observability Dashboard\n");
        report.push_str("=====================================\n\n");
        
        // System Health
        report.push_str(&format!("📊 System Health: {:?}\n", status.system_health));
        report.push_str(&format!("⏱️  Uptime: {} seconds\n", status.system_info.uptime_seconds));
        report.push_str(&format!("📈 SLO Health: {:.1}% ({} of {} SLOs healthy)\n", 
            health_summary.slo_health_percentage, 
            health_summary.healthy_slos, 
            health_summary.total_slos));
        report.push_str(&format!("🚨 Active Alerts: {}\n\n", health_summary.active_alerts));
        
        // SLO Status
        if !status.slo_statuses.is_empty() {
            report.push_str("🎯 Service Level Objectives\n");
            report.push_str("----------------------------\n");
            
            for (slo_id, slo_status) in &status.slo_statuses {
                let health_icon = if slo_status.is_healthy { "✅" } else { "❌" };
                report.push_str(&format!("{} {}: {:.1}% (target: {:.1}%)\n",
                    health_icon,
                    slo_id,
                    slo_status.current_success_rate * 100.0,
                    slo_status.target_success_rate * 100.0
                ));
            }
            report.push_str("\n");
        }
        
        // Key Metrics
        if let Some(metrics) = &status.metrics {
            report.push_str("📊 Key Metrics\n");
            report.push_str("--------------\n");
            
            // Actor resolutions
            if let Some(resolutions) = metrics.counters.get("actor_resolutions_total") {
                report.push_str(&format!("Actor Resolutions: {:.0}\n", resolutions.value));
            }
            
            if let Some(errors) = metrics.counters.get("actor_resolution_errors_total") {
                report.push_str(&format!("Resolution Errors: {:.0}\n", errors.value));
            }
            
            // Cache metrics
            if let Some(hits) = metrics.counters.get("cache_hits_total") {
                report.push_str(&format!("Cache Hits: {:.0}\n", hits.value));
            }
            
            if let Some(misses) = metrics.counters.get("cache_misses_total") {
                report.push_str(&format!("Cache Misses: {:.0}\n", misses.value));
            }
            
            // Memory usage
            if let Some(memory) = metrics.gauges.get("memory_usage_bytes") {
                let memory_mb = memory.value / (1024.0 * 1024.0);
                report.push_str(&format!("Memory Usage: {:.1} MB\n", memory_mb));
            }
            
            // Active actors
            if let Some(actors) = metrics.gauges.get("active_actors_count") {
                report.push_str(&format!("Active Actors: {:.0}\n", actors.value));
            }
            
            report.push_str("\n");
        }
        
        // Recent Alerts
        if !status.recent_alerts.is_empty() {
            report.push_str("🚨 Recent Alerts\n");
            report.push_str("----------------\n");
            
            for alert in &status.recent_alerts {
                if alert.active {
                    let severity_icon = match alert.severity {
                        AlertSeverity::Info => "ℹ️",
                        AlertSeverity::Warning => "⚠️",
                        AlertSeverity::Error => "❌",
                        AlertSeverity::Critical => "🚨",
                    };
                    
                    report.push_str(&format!("{} {}: {}\n",
                        severity_icon,
                        alert.source,
                        alert.message
                    ));
                }
            }
            report.push_str("\n");
        }
        
        // System Information
        report.push_str("🔧 System Information\n");
        report.push_str("---------------------\n");
        report.push_str(&format!("Version: {}\n", status.system_info.version));
        report.push_str(&format!("Registered SLOs: {}\n", status.system_info.slo_count));
        report.push_str(&format!("Registered Metrics: {}\n", status.system_info.metric_count));
        report.push_str(&format!("CPU Usage: {:.1}%\n", status.system_info.load_info.cpu_usage_percent));
        report.push_str(&format!("Memory Usage: {:.1}%\n", status.system_info.load_info.memory_usage_percent));
        report.push_str(&format!("Active Subsystems: {}\n", status.system_info.load_info.active_subsystems));
        
        Ok(report)
    }

    /// Generate a dashboard report in JSON format.
    pub async fn generate_json_report(&self) -> ActorCoreResult<String> {
        let status = self.get_status().await?;
        let health_summary = self.get_health_summary().await?;
        
        let dashboard_data = DashboardData {
            timestamp: status.timestamp,
            health_summary,
            slo_statuses: status.slo_statuses,
            key_metrics: self.extract_key_metrics(&status.metrics),
            system_info: status.system_info,
            recent_alerts: status.recent_alerts,
        };
        
        Ok(serde_json::to_string_pretty(&dashboard_data)?)
    }

    /// Update dashboard configuration.
    pub fn update_config(&mut self, config: DashboardConfig) {
        self.config = config;
        info!("Updated dashboard configuration");
    }

    /// Determine overall system health based on SLO statuses.
    async fn determine_system_health(&self, slo_statuses: &HashMap<String, SLOStatus>) -> SystemHealthStatus {
        if slo_statuses.is_empty() {
            return SystemHealthStatus::Healthy;
        }

        let unhealthy_slos = slo_statuses.values()
            .filter(|slo| !slo.is_healthy)
            .count();

        let total_slos = slo_statuses.len();
        let unhealthy_percentage = (unhealthy_slos as f64 / total_slos as f64) * 100.0;

        if unhealthy_percentage >= 50.0 {
            SystemHealthStatus::Critical
        } else if unhealthy_percentage >= 25.0 {
            SystemHealthStatus::Warning
        } else {
            SystemHealthStatus::Healthy
        }
    }

    /// Get system information.
    async fn get_system_info(&self) -> SystemInfo {
        let start_time = self.metrics_collector.get_metadata().values()
            .next()
            .map(|_| SystemTime::now() - Duration::from_secs(3600)) // Placeholder
            .unwrap_or(SystemTime::now());

        let uptime_seconds = start_time.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        SystemInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds,
            slo_count: self.slo_manager.list_slos().len(),
            metric_count: self.metrics_collector.list_metrics().len(),
            load_info: LoadInfo {
                cpu_usage_percent: 25.0, // Placeholder - would be real system metrics
                memory_usage_percent: 45.0, // Placeholder
                active_actors: self.get_active_actors_count().await,
                active_subsystems: self.get_active_subsystems_count().await,
            },
        }
    }

    /// Get recent alerts.
    async fn get_recent_alerts(&self) -> Vec<SystemAlert> {
        // Placeholder implementation - would integrate with real alerting system
        vec![]
    }

    /// Extract key metrics from metrics snapshot.
    fn extract_key_metrics(&self, metrics: &Option<MetricsSnapshot>) -> HashMap<String, f64> {
        let mut key_metrics = HashMap::new();
        
        if let Some(metrics) = metrics {
            // Extract important counter metrics
            for (name, metric) in &metrics.counters {
                if name.contains("total") || name.contains("resolutions") || name.contains("errors") {
                    key_metrics.insert(name.clone(), metric.value);
                }
            }
            
            // Extract important gauge metrics
            for (name, metric) in &metrics.gauges {
                if name.contains("usage") || name.contains("count") || name.contains("active") {
                    key_metrics.insert(name.clone(), metric.value);
                }
            }
        }
        
        key_metrics
    }

    /// Get active actors count (placeholder).
    async fn get_active_actors_count(&self) -> u64 {
        // Placeholder - would query actual actor registry
        1000
    }

    /// Get active subsystems count (placeholder).
    async fn get_active_subsystems_count(&self) -> u64 {
        // Placeholder - would query actual subsystem registry
        25
    }
}

/// Health summary for quick overview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    /// Overall system health
    pub overall_health: SystemHealthStatus,
    /// SLO health percentage
    pub slo_health_percentage: f64,
    /// Number of healthy SLOs
    pub healthy_slos: usize,
    /// Total number of SLOs
    pub total_slos: usize,
    /// Number of active alerts
    pub active_alerts: usize,
    /// System uptime in seconds
    pub uptime_seconds: u64,
}

/// Dashboard data structure for JSON export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    /// Timestamp
    pub timestamp: SystemTime,
    /// Health summary
    pub health_summary: HealthSummary,
    /// SLO statuses
    pub slo_statuses: HashMap<String, SLOStatus>,
    /// Key metrics
    pub key_metrics: HashMap<String, f64>,
    /// System information
    pub system_info: SystemInfo,
    /// Recent alerts
    pub recent_alerts: Vec<SystemAlert>,
}

/// Dashboard builder for creating configured dashboards.
pub struct DashboardBuilder {
    slo_manager: Option<Arc<SLOManager>>,
    metrics_collector: Option<Arc<MetricsCollector>>,
    config: DashboardConfig,
}

impl DashboardBuilder {
    /// Create a new dashboard builder.
    pub fn new() -> Self {
        Self {
            slo_manager: None,
            metrics_collector: None,
            config: DashboardConfig::default(),
        }
    }

    /// Set the SLO manager.
    pub fn with_slo_manager(mut self, slo_manager: Arc<SLOManager>) -> Self {
        self.slo_manager = Some(slo_manager);
        self
    }

    /// Set the metrics collector.
    pub fn with_metrics_collector(mut self, metrics_collector: Arc<MetricsCollector>) -> Self {
        self.metrics_collector = Some(metrics_collector);
        self
    }

    /// Set the dashboard configuration.
    pub fn with_config(mut self, config: DashboardConfig) -> Self {
        self.config = config;
        self
    }

    /// Build the dashboard.
    pub fn build(self) -> ActorCoreResult<ObservabilityDashboard> {
        let slo_manager = self.slo_manager
            .ok_or_else(|| ActorCoreError::InvalidInput("SLO manager is required".to_string()))?;
        
        let metrics_collector = self.metrics_collector
            .ok_or_else(|| ActorCoreError::InvalidInput("Metrics collector is required".to_string()))?;

        Ok(ObservabilityDashboard::new(slo_manager, metrics_collector, self.config))
    }
}