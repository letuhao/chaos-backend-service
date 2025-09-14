//! Comprehensive tests for observability/dashboard.rs coverage.
//!
//! This module contains detailed tests for all dashboard functionality,
//! including status generation, health summaries, report generation,
//! and error handling to achieve 80%+ line coverage.

use actor_core::prelude::*;
use actor_core::observability::dashboard::*;
use actor_core::observability::slos::{SLOManager, SLO, SLOMetricType, SLOSeverity};
use actor_core::observability::metrics_collector::MetricsCollector;
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    // === Dashboard Creation and Configuration Tests ===

    #[tokio::test]
    async fn test_dashboard_creation() {
        let slo_manager = Arc::new(SLOManager::new());
        let metrics_collector = Arc::new(MetricsCollector::new());
        let config = DashboardConfig::default();
        
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        // Test that dashboard was created successfully
        assert!(dashboard.get_status().await.is_ok());
    }

    #[tokio::test]
    async fn test_dashboard_config_default() {
        let config = DashboardConfig::default();
        
        assert_eq!(config.refresh_interval, Duration::from_secs(30));
        assert!(config.include_detailed_metrics);
        assert!(config.include_slo_status);
        assert!(config.include_system_health);
        assert_eq!(config.max_recent_measurements, 10);
        assert!(config.auto_refresh);
    }

    #[tokio::test]
    async fn test_dashboard_config_custom() {
        let config = DashboardConfig {
            refresh_interval: Duration::from_secs(60),
            include_detailed_metrics: false,
            include_slo_status: false,
            include_system_health: false,
            max_recent_measurements: 50,
            auto_refresh: false,
        };
        
        assert_eq!(config.refresh_interval, Duration::from_secs(60));
        assert!(!config.include_detailed_metrics);
        assert!(!config.include_slo_status);
        assert!(!config.include_system_health);
        assert_eq!(config.max_recent_measurements, 50);
        assert!(!config.auto_refresh);
    }

    // === System Health Status Tests ===

    #[tokio::test]
    async fn test_system_health_status_variants() {
        // Test all SystemHealthStatus variants
        let healthy = SystemHealthStatus::Healthy;
        let warning = SystemHealthStatus::Warning;
        let critical = SystemHealthStatus::Critical;
        let down = SystemHealthStatus::Down;
        
        // Test equality
        assert_eq!(healthy, SystemHealthStatus::Healthy);
        assert_ne!(healthy, warning);
        assert_ne!(warning, critical);
        assert_ne!(critical, down);
    }

    #[tokio::test]
    async fn test_alert_severity_variants() {
        // Test all AlertSeverity variants
        let info = AlertSeverity::Info;
        let warning = AlertSeverity::Warning;
        let error = AlertSeverity::Error;
        let critical = AlertSeverity::Critical;
        
        // Test equality
        assert_eq!(info, AlertSeverity::Info);
        assert_ne!(info, warning);
        assert_ne!(warning, error);
        assert_ne!(error, critical);
    }

    // === Dashboard Status Tests ===

    #[tokio::test]
    async fn test_dashboard_status_creation() {
        let timestamp = std::time::SystemTime::now();
        let system_health = SystemHealthStatus::Healthy;
        let slo_statuses = HashMap::new();
        let metrics = None;
        let system_info = SystemInfo {
            version: "1.0.0".to_string(),
            uptime_seconds: 3600,
            slo_count: 0,
            metric_count: 0,
            load_info: LoadInfo {
                cpu_usage_percent: 50.0,
                memory_usage_percent: 60.0,
                active_actors: 100,
                active_subsystems: 50,
            },
        };
        let recent_alerts = Vec::new();
        
        let status = DashboardStatus {
            timestamp,
            system_health,
            slo_statuses,
            metrics,
            system_info,
            recent_alerts,
        };
        
        assert_eq!(status.system_health, SystemHealthStatus::Healthy);
        assert!(status.slo_statuses.is_empty());
        assert!(status.metrics.is_none());
        assert_eq!(status.system_info.version, "1.0.0");
        assert_eq!(status.system_info.uptime_seconds, 3600);
    }

    #[tokio::test]
    async fn test_system_info_creation() {
        let system_info = SystemInfo {
            version: "2.0.0".to_string(),
            uptime_seconds: 7200,
            slo_count: 5,
            metric_count: 10,
            load_info: LoadInfo {
                cpu_usage_percent: 75.0,
                memory_usage_percent: 80.0,
                active_actors: 200,
                active_subsystems: 100,
            },
        };
        
        assert_eq!(system_info.version, "2.0.0");
        assert_eq!(system_info.uptime_seconds, 7200);
        assert_eq!(system_info.slo_count, 5);
        assert_eq!(system_info.metric_count, 10);
        assert_eq!(system_info.load_info.cpu_usage_percent, 75.0);
        assert_eq!(system_info.load_info.memory_usage_percent, 80.0);
        assert_eq!(system_info.load_info.active_actors, 200);
        assert_eq!(system_info.load_info.active_subsystems, 100);
    }

    #[tokio::test]
    async fn test_load_info_creation() {
        let load_info = LoadInfo {
            cpu_usage_percent: 25.0,
            memory_usage_percent: 30.0,
            active_actors: 50,
            active_subsystems: 25,
        };
        
        assert_eq!(load_info.cpu_usage_percent, 25.0);
        assert_eq!(load_info.memory_usage_percent, 30.0);
        assert_eq!(load_info.active_actors, 50);
        assert_eq!(load_info.active_subsystems, 25);
    }

    #[tokio::test]
    async fn test_system_alert_creation() {
        let alert = SystemAlert {
            id: "alert_001".to_string(),
            severity: AlertSeverity::Warning,
            message: "High CPU usage detected".to_string(),
            timestamp: std::time::SystemTime::now(),
            source: "system_monitor".to_string(),
            active: true,
        };
        
        assert_eq!(alert.id, "alert_001");
        assert_eq!(alert.severity, AlertSeverity::Warning);
        assert_eq!(alert.message, "High CPU usage detected");
        assert_eq!(alert.source, "system_monitor");
        assert!(alert.active);
    }

    // === Dashboard with SLOs Tests ===

    #[tokio::test]
    async fn test_dashboard_with_slos() {
        let mut slo_manager = SLOManager::new();
        
        // Register some SLOs
        let slo1 = SLO {
            id: "availability".to_string(),
            name: "Availability SLO".to_string(),
            description: "System availability".to_string(),
            metric_type: SLOMetricType::Availability,
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            severity: SLOSeverity::High,
            enabled: true,
            labels: HashMap::new(),
            alert_threshold: 0.95,
        };
        
        let slo2 = SLO {
            id: "latency".to_string(),
            name: "Latency SLO".to_string(),
            description: "Response latency".to_string(),
            metric_type: SLOMetricType::Latency { threshold_ms: 100 },
            target_success_rate: 0.95,
            measurement_window: Duration::from_secs(300),
            severity: SLOSeverity::Medium,
            enabled: true,
            labels: HashMap::new(),
            alert_threshold: 0.90,
        };
        
        slo_manager.register_slo(slo1).unwrap();
        slo_manager.register_slo(slo2).unwrap();
        
        // Record some events
        slo_manager.record_event("availability", true, None).unwrap();
        slo_manager.record_event("latency", true, None).unwrap();
        
        let config = DashboardConfig {
            include_slo_status: true,
            include_detailed_metrics: false,
            include_system_health: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(slo_manager);
        let metrics_collector = Arc::new(MetricsCollector::new());
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let status = dashboard.get_status().await.unwrap();
        assert_eq!(status.slo_statuses.len(), 2);
        assert!(status.slo_statuses.contains_key("availability"));
        assert!(status.slo_statuses.contains_key("latency"));
    }

    #[tokio::test]
    async fn test_dashboard_without_slos() {
        let config = DashboardConfig {
            include_slo_status: false,
            include_detailed_metrics: false,
            include_system_health: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(SLOManager::new());
        let metrics_collector = Arc::new(MetricsCollector::new());
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let status = dashboard.get_status().await.unwrap();
        assert!(status.slo_statuses.is_empty());
        assert_eq!(status.system_health, SystemHealthStatus::Healthy);
    }

    // === Dashboard with Metrics Tests ===

    #[tokio::test]
    async fn test_dashboard_with_metrics() {
        let mut metrics_collector = MetricsCollector::new();
        
        // Register some metrics
        metrics_collector.register_counter("requests_total".to_string(), "Total requests".to_string(), HashMap::new()).unwrap();
        metrics_collector.register_histogram("request_duration".to_string(), "Request duration".to_string(), HashMap::new(), vec![10.0, 50.0, 100.0]).unwrap();
        metrics_collector.register_gauge("active_connections".to_string(), "Active connections".to_string(), HashMap::new(), None).unwrap();
        
        // Record some metrics
        metrics_collector.increment_counter("requests_total", 100).unwrap();
        metrics_collector.observe_histogram("request_duration", 50.0).unwrap();
        metrics_collector.set_gauge("active_connections", 25).unwrap();
        
        let config = DashboardConfig {
            include_slo_status: false,
            include_detailed_metrics: true,
            include_system_health: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(SLOManager::new());
        let metrics_collector = Arc::new(metrics_collector);
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let status = dashboard.get_status().await.unwrap();
        assert!(status.metrics.is_some());
        let metrics = status.metrics.unwrap();
        assert!(metrics.counters.contains_key("requests_total"));
        assert!(metrics.histograms.contains_key("request_duration"));
        assert!(metrics.gauges.contains_key("active_connections"));
    }

    #[tokio::test]
    async fn test_dashboard_without_metrics() {
        let config = DashboardConfig {
            include_slo_status: false,
            include_detailed_metrics: false,
            include_system_health: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(SLOManager::new());
        let metrics_collector = Arc::new(MetricsCollector::new());
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let status = dashboard.get_status().await.unwrap();
        assert!(status.metrics.is_none());
    }

    // === Health Summary Tests ===

    #[tokio::test]
    async fn test_health_summary_with_healthy_slos() {
        let mut slo_manager = SLOManager::new();
        
        // Register and record successful events for SLOs
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO".to_string(),
            metric_type: SLOMetricType::Availability,
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            severity: SLOSeverity::High,
            enabled: true,
            labels: HashMap::new(),
            alert_threshold: 0.95,
        };
        
        slo_manager.register_slo(slo).unwrap();
        slo_manager.record_event("test_slo", true, None).unwrap();
        
        let config = DashboardConfig {
            include_slo_status: true,
            include_detailed_metrics: false,
            include_system_health: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(slo_manager);
        let metrics_collector = Arc::new(MetricsCollector::new());
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let health_summary = dashboard.get_health_summary().await.unwrap();
        assert_eq!(health_summary.total_slos, 1);
        assert_eq!(health_summary.healthy_slos, 1);
        assert_eq!(health_summary.slo_health_percentage, 100.0);
        assert_eq!(health_summary.active_alerts, 0);
    }

    #[tokio::test]
    async fn test_health_summary_with_no_slos() {
        let config = DashboardConfig {
            include_slo_status: true,
            include_detailed_metrics: false,
            include_system_health: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(SLOManager::new());
        let metrics_collector = Arc::new(MetricsCollector::new());
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let health_summary = dashboard.get_health_summary().await.unwrap();
        assert_eq!(health_summary.total_slos, 0);
        assert_eq!(health_summary.healthy_slos, 0);
        assert_eq!(health_summary.slo_health_percentage, 100.0);
        assert_eq!(health_summary.active_alerts, 0);
    }

    #[tokio::test]
    async fn test_health_summary_with_mixed_slo_health() {
        let mut slo_manager = SLOManager::new();
        
        // Register two SLOs
        let slo1 = SLO {
            id: "healthy_slo".to_string(),
            name: "Healthy SLO".to_string(),
            description: "Healthy SLO".to_string(),
            metric_type: SLOMetricType::Availability,
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            severity: SLOSeverity::High,
            enabled: true,
            labels: HashMap::new(),
            alert_threshold: 0.95,
        };
        
        let slo2 = SLO {
            id: "unhealthy_slo".to_string(),
            name: "Unhealthy SLO".to_string(),
            description: "Unhealthy SLO".to_string(),
            metric_type: SLOMetricType::Availability,
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            severity: SLOSeverity::High,
            enabled: true,
            labels: HashMap::new(),
            alert_threshold: 0.95,
        };
        
        slo_manager.register_slo(slo1).unwrap();
        slo_manager.register_slo(slo2).unwrap();
        
        // Record successful events for healthy SLO, failed events for unhealthy SLO
        slo_manager.record_event("healthy_slo", true, None).unwrap();
        slo_manager.record_event("unhealthy_slo", false, None).unwrap();
        
        let config = DashboardConfig {
            include_slo_status: true,
            include_detailed_metrics: false,
            include_system_health: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(slo_manager);
        let metrics_collector = Arc::new(MetricsCollector::new());
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let health_summary = dashboard.get_health_summary().await.unwrap();
        assert_eq!(health_summary.total_slos, 2);
        assert_eq!(health_summary.healthy_slos, 1);
        assert_eq!(health_summary.slo_health_percentage, 50.0);
    }

    // === Report Generation Tests ===

    #[tokio::test]
    async fn test_generate_text_report() {
        let config = DashboardConfig::default();
        let slo_manager = Arc::new(SLOManager::new());
        let metrics_collector = Arc::new(MetricsCollector::new());
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let report = dashboard.generate_text_report().await.unwrap();
        assert!(!report.is_empty());
        assert!(report.contains("Actor Core Observability Dashboard"));
        assert!(report.contains("System Health"));
        assert!(report.contains("Uptime"));
    }

    #[tokio::test]
    async fn test_generate_text_report_with_slos() {
        let mut slo_manager = SLOManager::new();
        
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO".to_string(),
            metric_type: SLOMetricType::Availability,
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            severity: SLOSeverity::High,
            enabled: true,
            labels: HashMap::new(),
            alert_threshold: 0.95,
        };
        
        slo_manager.register_slo(slo).unwrap();
        slo_manager.record_event("test_slo", true, None).unwrap();
        
        let config = DashboardConfig {
            include_slo_status: true,
            include_detailed_metrics: false,
            include_system_health: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(slo_manager);
        let metrics_collector = Arc::new(MetricsCollector::new());
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let report = dashboard.generate_text_report().await.unwrap();
        assert!(!report.is_empty());
        // The report should contain SLO information
        assert!(report.contains("test_slo") || report.contains("SLO") || report.contains("slo"));
    }

    #[tokio::test]
    async fn test_generate_json_report() {
        let config = DashboardConfig::default();
        let slo_manager = Arc::new(SLOManager::new());
        let metrics_collector = Arc::new(MetricsCollector::new());
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let report = dashboard.generate_json_report().await;
        assert!(report.is_ok());
        let report = report.unwrap();
        assert!(!report.is_empty());
        
        // Verify it's valid JSON
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&report);
        assert!(parsed.is_ok());
    }

    #[tokio::test]
    async fn test_generate_json_report_with_data() {
        let mut slo_manager = SLOManager::new();
        let mut metrics_collector = MetricsCollector::new();
        
        // Set up SLO
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO".to_string(),
            metric_type: SLOMetricType::Availability,
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            severity: SLOSeverity::High,
            enabled: true,
            labels: HashMap::new(),
            alert_threshold: 0.95,
        };
        
        slo_manager.register_slo(slo).unwrap();
        slo_manager.record_event("test_slo", true, None).unwrap();
        
        // Set up metrics
        metrics_collector.register_counter("requests_total".to_string(), "Total requests".to_string(), HashMap::new()).unwrap();
        metrics_collector.increment_counter("requests_total", 100).unwrap();
        
        let config = DashboardConfig {
            include_slo_status: true,
            include_detailed_metrics: true,
            include_system_health: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(slo_manager);
        let metrics_collector = Arc::new(metrics_collector);
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let report = dashboard.generate_json_report().await;
        assert!(report.is_ok());
        let report = report.unwrap();
        assert!(!report.is_empty());
        
        // Verify it's valid JSON and contains expected data
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&report);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();
        // The JSON should contain some of these fields
        assert!(parsed.get("system_health").is_some() || 
                parsed.get("slo_statuses").is_some() || 
                parsed.get("metrics").is_some() ||
                parsed.get("timestamp").is_some());
    }

    // === Error Handling Tests ===

    #[tokio::test]
    async fn test_dashboard_with_invalid_slo_events() {
        let mut slo_manager = SLOManager::new();
        
        // Try to record events for non-existent SLO
        let result = slo_manager.record_event("non_existent_slo", true, None);
        assert!(result.is_err());
        
        let config = DashboardConfig::default();
        let slo_manager = Arc::new(slo_manager);
        let metrics_collector = Arc::new(MetricsCollector::new());
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        // Dashboard should still work even with no SLOs
        let status = dashboard.get_status().await.unwrap();
        assert!(status.slo_statuses.is_empty());
    }

    #[tokio::test]
    async fn test_dashboard_with_metrics_errors() {
        let metrics_collector = MetricsCollector::new();
        
        // Try to record metrics without registering them first
        let result = metrics_collector.increment_counter("unregistered_counter", 1);
        assert!(result.is_err());
        
        let config = DashboardConfig {
            include_detailed_metrics: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(SLOManager::new());
        let metrics_collector = Arc::new(metrics_collector);
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        // Dashboard should still work even with no metrics
        let status = dashboard.get_status().await.unwrap();
        assert!(status.metrics.is_some());
        let metrics = status.metrics.unwrap();
        assert!(metrics.counters.is_empty());
        assert!(metrics.histograms.is_empty());
        assert!(metrics.gauges.is_empty());
    }

    // === Edge Cases and Performance Tests ===

    #[tokio::test]
    async fn test_dashboard_with_many_slos() {
        let mut slo_manager = SLOManager::new();
        
        // Register many SLOs
        for i in 0..100 {
            let slo = SLO {
                id: format!("slo_{}", i),
                name: format!("SLO {}", i),
                description: format!("SLO {}", i),
                metric_type: SLOMetricType::Availability,
                target_success_rate: 0.99,
                measurement_window: Duration::from_secs(300),
                severity: SLOSeverity::High,
                enabled: true,
                labels: HashMap::new(),
                alert_threshold: 0.95,
            };
            
            slo_manager.register_slo(slo).unwrap();
            slo_manager.record_event(&format!("slo_{}", i), true, None).unwrap();
        }
        
        let config = DashboardConfig {
            include_slo_status: true,
            include_detailed_metrics: false,
            include_system_health: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(slo_manager);
        let metrics_collector = Arc::new(MetricsCollector::new());
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let status = dashboard.get_status().await.unwrap();
        assert_eq!(status.slo_statuses.len(), 100);
        
        let health_summary = dashboard.get_health_summary().await.unwrap();
        assert_eq!(health_summary.total_slos, 100);
        assert_eq!(health_summary.healthy_slos, 100);
        assert_eq!(health_summary.slo_health_percentage, 100.0);
    }

    #[tokio::test]
    async fn test_dashboard_with_many_metrics() {
        let mut metrics_collector = MetricsCollector::new();
        
        // Register many metrics
        for i in 0..50 {
            let counter_name = format!("counter_{}", i);
            let histogram_name = format!("histogram_{}", i);
            let gauge_name = format!("gauge_{}", i);
            
            metrics_collector.register_counter(counter_name.clone(), format!("Counter {}", i), HashMap::new()).unwrap();
            metrics_collector.register_histogram(histogram_name.clone(), format!("Histogram {}", i), HashMap::new(), vec![10.0, 50.0, 100.0]).unwrap();
            metrics_collector.register_gauge(gauge_name.clone(), format!("Gauge {}", i), HashMap::new(), None).unwrap();
            
            metrics_collector.increment_counter(&counter_name, i as u64).unwrap();
            metrics_collector.observe_histogram(&histogram_name, i as f64 * 10.0).unwrap();
            metrics_collector.set_gauge(&gauge_name, i as u64).unwrap();
        }
        
        let config = DashboardConfig {
            include_slo_status: false,
            include_detailed_metrics: true,
            include_system_health: true,
            ..Default::default()
        };
        
        let slo_manager = Arc::new(SLOManager::new());
        let metrics_collector = Arc::new(metrics_collector);
        let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
        
        let status = dashboard.get_status().await.unwrap();
        assert!(status.metrics.is_some());
        let metrics = status.metrics.unwrap();
        assert_eq!(metrics.counters.len(), 50);
        assert_eq!(metrics.histograms.len(), 50);
        assert_eq!(metrics.gauges.len(), 50);
    }

    #[tokio::test]
    async fn test_dashboard_configuration_variations() {
        // Test different configuration combinations
        let configs = vec![
            DashboardConfig {
                include_slo_status: true,
                include_detailed_metrics: true,
                include_system_health: true,
                ..Default::default()
            },
            DashboardConfig {
                include_slo_status: true,
                include_detailed_metrics: false,
                include_system_health: true,
                ..Default::default()
            },
            DashboardConfig {
                include_slo_status: false,
                include_detailed_metrics: true,
                include_system_health: true,
                ..Default::default()
            },
            DashboardConfig {
                include_slo_status: false,
                include_detailed_metrics: false,
                include_system_health: true,
                ..Default::default()
            },
        ];
        
        for config in configs {
            let slo_manager = Arc::new(SLOManager::new());
            let metrics_collector = Arc::new(MetricsCollector::new());
            let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
            
            let status = dashboard.get_status().await.unwrap();
            assert_eq!(status.system_health, SystemHealthStatus::Healthy);
        }
    }

    // === Serialization Tests ===

    #[tokio::test]
    async fn test_dashboard_status_serialization() {
        let timestamp = std::time::SystemTime::now();
        let system_health = SystemHealthStatus::Warning;
        let slo_statuses = HashMap::new();
        let metrics = None;
        let system_info = SystemInfo {
            version: "1.0.0".to_string(),
            uptime_seconds: 3600,
            slo_count: 0,
            metric_count: 0,
            load_info: LoadInfo {
                cpu_usage_percent: 50.0,
                memory_usage_percent: 60.0,
                active_actors: 100,
                active_subsystems: 50,
            },
        };
        let recent_alerts = Vec::new();
        
        let status = DashboardStatus {
            timestamp,
            system_health,
            slo_statuses,
            metrics,
            system_info,
            recent_alerts,
        };
        
        // Test serialization
        let json = serde_json::to_string(&status).unwrap();
        assert!(!json.is_empty());
        
        // Test deserialization
        let deserialized: DashboardStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.system_health, SystemHealthStatus::Warning);
        assert_eq!(deserialized.system_info.version, "1.0.0");
    }

    #[tokio::test]
    async fn test_alert_serialization() {
        let alert = SystemAlert {
            id: "test_alert".to_string(),
            severity: AlertSeverity::Critical,
            message: "System critical error".to_string(),
            timestamp: std::time::SystemTime::now(),
            source: "error_monitor".to_string(),
            active: true,
        };
        
        // Test serialization
        let json = serde_json::to_string(&alert).unwrap();
        assert!(!json.is_empty());
        assert!(json.contains("test_alert"));
        assert!(json.contains("Critical"));
        assert!(json.contains("System critical error"));
        
        // Test deserialization
        let deserialized: SystemAlert = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "test_alert");
        assert_eq!(deserialized.severity, AlertSeverity::Critical);
        assert_eq!(deserialized.message, "System critical error");
        assert!(deserialized.active);
    }
}
