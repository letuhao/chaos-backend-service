//! Observability tests for Actor Core.
//!
//! This module contains comprehensive tests for the observability system,
//! including SLO management, metrics collection, and dashboard monitoring.

use actor_core::prelude::*;
use std::time::Duration;

#[test]
fn test_slo_registration() {
    let mut manager = SLOManager::new();
    
    let slo = SLO {
        name: "test_slo".to_string(),
        description: "Test SLO".to_string(),
        metric_type: SLOMetricType::Latency,
        target_value: 100.0,
        measurement_window: Duration::from_secs(60),
        severity: SLOSeverity::High,
        enabled: true,
    };
    
    manager.register_slo(slo.clone());
    assert!(manager.is_slo_registered("test_slo"));
    assert_eq!(manager.get_slo("test_slo"), Some(&slo));
    
    manager.unregister_slo("test_slo");
    assert!(!manager.is_slo_registered("test_slo"));
    assert_eq!(manager.get_slo("test_slo"), None);
}

#[test]
fn test_slo_measurement() {
    let mut manager = SLOManager::new();
    
    let slo = SLO {
        name: "test_slo".to_string(),
        description: "Test SLO".to_string(),
        metric_type: SLOMetricType::Latency,
        target_value: 100.0,
        measurement_window: Duration::from_secs(60),
        severity: SLOSeverity::High,
        enabled: true,
    };
    
    manager.register_slo(slo);
    
    // Record measurements within target
    for _ in 0..10 {
        manager.record_measurement("test_slo", 50.0);
    }
    
    let status = manager.evaluate_slo("test_slo");
    assert!(status.is_some());
    let status = status.unwrap();
    assert!(status.is_healthy);
}

#[test]
fn test_slo_violation() {
    let mut manager = SLOManager::new();
    
    let slo = SLO {
        name: "test_slo".to_string(),
        description: "Test SLO".to_string(),
        metric_type: SLOMetricType::Latency,
        target_value: 100.0,
        measurement_window: Duration::from_secs(60),
        severity: SLOSeverity::High,
        enabled: true,
    };
    
    manager.register_slo(slo);
    
    // Record measurements exceeding target
    for _ in 0..10 {
        manager.record_measurement("test_slo", 150.0);
    }
    
    let status = manager.evaluate_slo("test_slo");
    assert!(status.is_some());
    let status = status.unwrap();
    assert!(!status.is_healthy);
}

#[test]
fn test_default_slos() {
    let slos = default_slos::create_default_slos();
    assert_eq!(slos.len(), 4);
    
    let slo_names: Vec<String> = slos.iter().map(|s| s.name.clone()).collect();
    assert!(slo_names.contains(&"availability".to_string()));
    assert!(slo_names.contains(&"latency".to_string()));
    assert!(slo_names.contains(&"error_rate".to_string()));
    assert!(slo_names.contains(&"cache_hit_rate".to_string()));
}

#[test]
fn test_metrics_collector_basic() {
    let mut collector = MetricsCollector::new();
    
    // Record some metrics
    collector.record_counter("test_counter", 1.0);
    collector.record_histogram("test_histogram", 50.0);
    collector.record_gauge("test_gauge", 75.0);
    
    let snapshot = collector.get_snapshot();
    
    assert!(snapshot.counters.contains_key("test_counter"));
    assert!(snapshot.histograms.contains_key("test_histogram"));
    assert!(snapshot.gauges.contains_key("test_gauge"));
    
    assert_eq!(snapshot.counters["test_counter"], MetricValue::Counter(1.0));
    assert_eq!(snapshot.gauges["test_gauge"], MetricValue::Gauge(75.0));
}

#[test]
fn test_metrics_collector_aggregation() {
    let mut collector = MetricsCollector::new();
    
    // Record multiple values for histogram
    for i in 0..10 {
        collector.record_histogram("test_histogram", i as f64 * 10.0);
    }
    
    let snapshot = collector.get_snapshot();
    let histogram = &snapshot.histograms["test_histogram"];
    
    match histogram {
        MetricValue::Histogram { count, sum, .. } => {
            assert_eq!(*count, 10);
            assert_eq!(*sum, 450.0); // 0 + 10 + 20 + ... + 90
        }
        _ => panic!("Expected histogram metric"),
    }
}

#[test]
fn test_dashboard_basic() {
    let config = DashboardConfig {
        refresh_interval: Duration::from_secs(1),
        enable_alerts: true,
        log_level: tracing::Level::INFO,
    };
    
    let dashboard = ObservabilityDashboard::new(config);
    
    let status = dashboard.get_status();
    assert_eq!(status.health, SystemHealthStatus::Healthy);
    assert!(status.slos.is_empty());
    assert!(status.metrics.is_empty());
}

#[test]
fn test_dashboard_with_slos() {
    let mut manager = SLOManager::new();
    let slos = default_slos::create_default_slos();
    
    for slo in slos {
        manager.register_slo(slo);
    }
    
    let config = DashboardConfig::default();
    let mut dashboard = ObservabilityDashboard::new(config);
    
    dashboard.set_slo_manager(manager);
    
    let status = dashboard.get_status();
    assert_eq!(status.slos.len(), 4);
    assert!(status.health != SystemHealthStatus::Unknown);
}

#[tokio::test]
async fn test_observability_integration() {
    let mut slo_manager = SLOManager::new();
    let mut metrics_collector = MetricsCollector::new();
    
    // Set up SLOs
    let slos = default_slos::create_default_slos();
    for slo in slos {
        slo_manager.register_slo(slo);
    }
    
    // Record some metrics
    metrics_collector.record_counter("requests_total", 100.0);
    metrics_collector.record_histogram("request_duration", 50.0);
    metrics_collector.record_gauge("active_connections", 25.0);
    
    // Record SLO measurements
    slo_manager.record_measurement("availability", 99.9);
    slo_manager.record_measurement("latency", 45.0);
    slo_manager.record_measurement("error_rate", 0.1);
    slo_manager.record_measurement("cache_hit_rate", 95.0);
    
    // Create dashboard
    let config = DashboardConfig::default();
    let mut dashboard = ObservabilityDashboard::new(config);
    
    dashboard.set_slo_manager(slo_manager);
    dashboard.set_metrics_collector(metrics_collector);
    
    let status = dashboard.get_status();
    
    // Verify integration
    assert_eq!(status.slos.len(), 4);
    assert!(!status.metrics.is_empty());
    assert!(status.health != SystemHealthStatus::Unknown);
    
    // Test report generation
    let report = dashboard.generate_report().await;
    assert!(report.is_ok());
    let report = report.unwrap();
    assert!(!report.is_empty());
}
