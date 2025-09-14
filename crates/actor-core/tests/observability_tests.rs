//! Observability tests for Actor Core.
//!
//! This module contains comprehensive tests for the observability system,
//! including SLO management, metrics collection, and dashboard monitoring.

use actor_core::prelude::*;
use std::time::Duration;
use std::sync::Arc;

#[test]
fn test_slo_registration() {
    let mut manager = SLOManager::new();
    
    let slo = SLO {
        id: "test_slo".to_string(),
        name: "test_slo".to_string(),
        description: "Test SLO".to_string(),
        metric_type: SLOMetricType::Latency { threshold_ms: 100 },
        target_success_rate: 0.95,
        measurement_window: Duration::from_secs(60),
        severity: SLOSeverity::High,
        enabled: true,
        labels: std::collections::HashMap::new(),
        alert_threshold: 0.8,
    };
    
    manager.register_slo(slo.clone()).unwrap();
    assert!(manager.get_slo("test_slo").is_some());
    assert_eq!(manager.get_slo("test_slo"), Some(&slo));
    
    manager.unregister_slo("test_slo").unwrap();
    assert!(manager.get_slo("test_slo").is_none());
}

#[test]
fn test_slo_measurement() {
    let mut manager = SLOManager::new();
    
    let slo = SLO {
        id: "test_slo".to_string(),
        name: "test_slo".to_string(),
        description: "Test SLO".to_string(),
        metric_type: SLOMetricType::Latency { threshold_ms: 100 },
        target_success_rate: 0.95,
        measurement_window: Duration::from_secs(60),
        severity: SLOSeverity::High,
        enabled: true,
        labels: std::collections::HashMap::new(),
        alert_threshold: 0.8,
    };
    
    manager.register_slo(slo).unwrap();
    
    // Record events (mostly successful)
    for _ in 0..10 {
        manager.record_event("test_slo", true, None).unwrap();
    }
    
    let status = manager.calculate_slo_status("test_slo");
    assert!(status.is_ok());
    let status = status.unwrap();
    assert!(status.is_healthy);
}

#[test]
fn test_slo_violation() {
    let mut manager = SLOManager::new();
    
    let slo = SLO {
        id: "test_slo".to_string(),
        name: "test_slo".to_string(),
        description: "Test SLO".to_string(),
        metric_type: SLOMetricType::Latency { threshold_ms: 100 },
        target_success_rate: 0.95,
        measurement_window: Duration::from_secs(60),
        severity: SLOSeverity::High,
        enabled: true,
        labels: std::collections::HashMap::new(),
        alert_threshold: 0.8,
    };
    
    manager.register_slo(slo).unwrap();
    
    // Record mostly failed events
    for _ in 0..10 {
        manager.record_event("test_slo", false, None).unwrap();
    }
    
    let status = manager.calculate_slo_status("test_slo");
    assert!(status.is_ok());
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
    
    // Register metrics first
    collector.register_counter("test_counter".to_string(), "Test counter".to_string(), std::collections::HashMap::new()).unwrap();
    collector.register_histogram("test_histogram".to_string(), "Test histogram".to_string(), std::collections::HashMap::new(), vec![10.0, 50.0, 100.0]).unwrap();
    collector.register_gauge("test_gauge".to_string(), "Test gauge".to_string(), std::collections::HashMap::new(), None).unwrap();
    
    // Record some metrics
    collector.increment_counter("test_counter", 1).unwrap();
    collector.observe_histogram("test_histogram", 50.0).unwrap();
    collector.set_gauge("test_gauge", 75).unwrap();
    
    let snapshot = collector.snapshot();
    
    assert!(snapshot.counters.contains_key("test_counter"));
    assert!(snapshot.histograms.contains_key("test_histogram"));
    assert!(snapshot.gauges.contains_key("test_gauge"));
    
    assert_eq!(snapshot.counters["test_counter"].value, 1.0);
    assert_eq!(snapshot.gauges["test_gauge"].value, 75.0);
}

#[test]
fn test_metrics_collector_aggregation() {
    let mut collector = MetricsCollector::new();
    
    // Register histogram first
    collector.register_histogram("test_histogram".to_string(), "Test histogram".to_string(), std::collections::HashMap::new(), vec![10.0, 50.0, 100.0]).unwrap();
    
    // Record multiple values for histogram
    for i in 0..10 {
        collector.observe_histogram("test_histogram", i as f64 * 10.0).unwrap();
    }
    
    let snapshot = collector.snapshot();
    let histogram = &snapshot.histograms["test_histogram"];
    
    assert_eq!(histogram.count, 10);
    assert_eq!(histogram.sum, 450.0); // 0 + 10 + 20 + ... + 90
}

#[tokio::test]
async fn test_dashboard_basic() {
    let config = DashboardConfig {
        refresh_interval: Duration::from_secs(30),
        include_detailed_metrics: true,
        include_slo_status: true,
        include_system_health: true,
        max_recent_measurements: 100,
        auto_refresh: true,
    };
    
    let slo_manager = Arc::new(SLOManager::new());
    let metrics_collector = Arc::new(MetricsCollector::new());
    let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
    
    let status = dashboard.get_status().await.unwrap();
    assert_eq!(status.system_health, SystemHealthStatus::Healthy);
    assert!(status.slo_statuses.is_empty());
    assert!(status.metrics.is_none());
}

#[tokio::test]
async fn test_dashboard_with_slos() {
    let mut manager = SLOManager::new();
    let slos = default_slos::create_default_slos();
    
    for slo in slos {
        manager.register_slo(slo).unwrap();
    }
    
    let config = DashboardConfig::default();
    let slo_manager = Arc::new(manager);
    let metrics_collector = Arc::new(MetricsCollector::new());
    let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
    
    let status = dashboard.get_status().await.unwrap();
    assert_eq!(status.slo_statuses.len(), 4);
    assert!(status.system_health != SystemHealthStatus::Down);
}

#[tokio::test]
async fn test_observability_integration() {
    let mut slo_manager = SLOManager::new();
    let mut metrics_collector = MetricsCollector::new();
    
    // Set up SLOs
    let slos = default_slos::create_default_slos();
    for slo in slos {
        slo_manager.register_slo(slo).unwrap();
    }
    
    // Register and record some metrics
    metrics_collector.register_counter("requests_total".to_string(), "Total requests".to_string(), std::collections::HashMap::new()).unwrap();
    metrics_collector.register_histogram("request_duration".to_string(), "Request duration".to_string(), std::collections::HashMap::new(), vec![10.0, 50.0, 100.0]).unwrap();
    metrics_collector.register_gauge("active_connections".to_string(), "Active connections".to_string(), std::collections::HashMap::new(), None).unwrap();
    
    metrics_collector.increment_counter("requests_total", 100).unwrap();
    metrics_collector.observe_histogram("request_duration", 50.0).unwrap();
    metrics_collector.set_gauge("active_connections", 25).unwrap();
    
    // Record SLO events
    slo_manager.record_event("availability", true, None).unwrap();
    slo_manager.record_event("latency", true, None).unwrap();
    slo_manager.record_event("error_rate", true, None).unwrap();
    slo_manager.record_event("cache_hit_rate", true, None).unwrap();
    
    // Create dashboard
    let config = DashboardConfig::default();
    let slo_manager = Arc::new(slo_manager);
    let metrics_collector = Arc::new(metrics_collector);
    let dashboard = ObservabilityDashboard::new(slo_manager, metrics_collector, config);
    
    let status = dashboard.get_status().await.unwrap();
    
    // Verify integration
    assert_eq!(status.slo_statuses.len(), 4);
    assert!(status.metrics.is_some());
    assert!(status.system_health != SystemHealthStatus::Down);
    
    // Test report generation
    let report = dashboard.generate_json_report().await;
    assert!(report.is_ok());
    let report = report.unwrap();
    assert!(!report.is_empty());
}
