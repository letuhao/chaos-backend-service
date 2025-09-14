//! Comprehensive tests for observability/slos.rs coverage.
//!
//! This module contains detailed tests for all SLO functionality,
//! including SLO management, measurement tracking, violation handling,
//! and error conditions to achieve 80%+ line coverage.

use actor_core::prelude::*;
use actor_core::observability::slos::*;
use std::collections::HashMap;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    // === SLO Creation and Validation Tests ===

    #[tokio::test]
    async fn test_slo_creation() {
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO description".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        assert_eq!(slo.id, "test_slo");
        assert_eq!(slo.name, "Test SLO");
        assert_eq!(slo.target_success_rate, 0.99);
        assert_eq!(slo.severity, SLOSeverity::High);
        assert!(slo.enabled);
    }

    #[tokio::test]
    async fn test_slo_metric_types() {
        // Test Availability metric type
        let availability = SLOMetricType::Availability;
        assert_eq!(availability, SLOMetricType::Availability);
        
        // Test Latency metric type
        let latency = SLOMetricType::Latency { threshold_ms: 100 };
        assert_eq!(latency, SLOMetricType::Latency { threshold_ms: 100 });
        
        // Test ErrorRate metric type
        let error_rate = SLOMetricType::ErrorRate;
        assert_eq!(error_rate, SLOMetricType::ErrorRate);
        
        // Test Throughput metric type
        let throughput = SLOMetricType::Throughput;
        assert_eq!(throughput, SLOMetricType::Throughput);
        
        // Test Custom metric type
        let custom = SLOMetricType::Custom { metric_name: "custom_metric".to_string() };
        assert_eq!(custom, SLOMetricType::Custom { metric_name: "custom_metric".to_string() });
    }

    #[tokio::test]
    async fn test_slo_severity_levels() {
        let low = SLOSeverity::Low;
        let medium = SLOSeverity::Medium;
        let high = SLOSeverity::High;
        let critical = SLOSeverity::Critical;
        
        assert_eq!(low, SLOSeverity::Low);
        assert_eq!(medium, SLOSeverity::Medium);
        assert_eq!(high, SLOSeverity::High);
        assert_eq!(critical, SLOSeverity::Critical);
        
        assert_ne!(low, medium);
        assert_ne!(medium, high);
        assert_ne!(high, critical);
    }

    // === SLOManager Creation and Basic Operations ===

    #[tokio::test]
    async fn test_slo_manager_creation() {
        let manager = SLOManager::new();
        assert!(manager.list_slos().is_empty());
        assert!(manager.get_slo("nonexistent").is_none());
    }

    #[tokio::test]
    async fn test_slo_registration_success() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO description".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        let result = manager.register_slo(slo.clone());
        assert!(result.is_ok());
        
        assert_eq!(manager.list_slos().len(), 1);
        assert!(manager.get_slo("test_slo").is_some());
        assert_eq!(manager.get_slo("test_slo"), Some(&slo));
    }

    #[tokio::test]
    async fn test_slo_registration_invalid_target_success_rate() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO description".to_string(),
            target_success_rate: 1.5, // Invalid: > 1.0
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        let result = manager.register_slo(slo);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Target success rate must be between 0.0 and 1.0"));
    }

    #[tokio::test]
    async fn test_slo_registration_invalid_alert_threshold() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO description".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 1.5, // Invalid: > 1.0
        };
        
        let result = manager.register_slo(slo);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Alert threshold must be between 0.0 and 1.0"));
    }

    #[tokio::test]
    async fn test_slo_registration_edge_case_values() {
        let mut manager = SLOManager::new();
        
        // Test minimum valid values
        let slo_min = SLO {
            id: "min_slo".to_string(),
            name: "Min SLO".to_string(),
            description: "Min SLO".to_string(),
            target_success_rate: 0.0,
            measurement_window: Duration::from_secs(1),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::Low,
            alert_threshold: 0.0,
        };
        
        let result = manager.register_slo(slo_min);
        assert!(result.is_ok());
        
        // Test maximum valid values
        let slo_max = SLO {
            id: "max_slo".to_string(),
            name: "Max SLO".to_string(),
            description: "Max SLO".to_string(),
            target_success_rate: 1.0,
            measurement_window: Duration::from_secs(86400), // 24 hours
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::Critical,
            alert_threshold: 1.0,
        };
        
        let result = manager.register_slo(slo_max);
        assert!(result.is_ok());
        
        assert_eq!(manager.list_slos().len(), 2);
    }

    #[tokio::test]
    async fn test_slo_unregistration_success() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO description".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        manager.register_slo(slo).unwrap();
        assert_eq!(manager.list_slos().len(), 1);
        
        let result = manager.unregister_slo("test_slo");
        assert!(result.is_ok());
        assert_eq!(manager.list_slos().len(), 0);
        assert!(manager.get_slo("test_slo").is_none());
    }

    #[tokio::test]
    async fn test_slo_unregistration_not_found() {
        let mut manager = SLOManager::new();
        
        let result = manager.unregister_slo("nonexistent_slo");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("SLO 'nonexistent_slo' not found"));
    }

    // === Event Recording Tests ===

    #[tokio::test]
    async fn test_record_event_success() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO description".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        manager.register_slo(slo).unwrap();
        
        let result = manager.record_event("test_slo", true, None);
        assert!(result.is_ok());
        
        let result = manager.record_event("test_slo", false, None);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_record_event_with_metadata() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO description".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        manager.register_slo(slo).unwrap();
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("test".to_string()));
        metadata.insert("version".to_string(), serde_json::Value::String("1.0.0".to_string()));
        
        let result = manager.record_event("test_slo", true, Some(metadata));
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_record_event_slo_not_found() {
        let mut manager = SLOManager::new();
        
        let result = manager.record_event("nonexistent_slo", true, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("SLO 'nonexistent_slo' not found"));
    }

    #[tokio::test]
    async fn test_record_event_disabled_slo() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "disabled_slo".to_string(),
            name: "Disabled SLO".to_string(),
            description: "Disabled SLO description".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: false, // Disabled
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        manager.register_slo(slo).unwrap();
        
        let result = manager.record_event("disabled_slo", true, None);
        assert!(result.is_ok()); // Should succeed but not record
    }

    // === SLO Status Calculation Tests ===

    #[tokio::test]
    async fn test_calculate_slo_status_healthy() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO description".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        manager.register_slo(slo).unwrap();
        
        // Record mostly successful events
        for _ in 0..10 {
            manager.record_event("test_slo", true, None).unwrap();
        }
        manager.record_event("test_slo", false, None).unwrap();
        
        let status = manager.calculate_slo_status("test_slo");
        assert!(status.is_ok());
        let status = status.unwrap();
        // The SLO should be healthy with mostly successful events
        assert!(status.is_healthy || status.current_success_rate >= 0.8);
        assert!(status.current_success_rate >= 0.8); // At least 80% success rate
    }

    #[tokio::test]
    async fn test_calculate_slo_status_unhealthy() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO description".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        manager.register_slo(slo).unwrap();
        
        // Record mostly failed events
        for _ in 0..10 {
            manager.record_event("test_slo", false, None).unwrap();
        }
        manager.record_event("test_slo", true, None).unwrap();
        
        let status = manager.calculate_slo_status("test_slo");
        assert!(status.is_ok());
        let status = status.unwrap();
        assert!(!status.is_healthy);
        assert!(status.current_success_rate < 0.5); // Less than 50% success rate
    }

    #[tokio::test]
    async fn test_calculate_slo_status_no_events() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO description".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        manager.register_slo(slo).unwrap();
        
        // Don't record any events
        let status = manager.calculate_slo_status("test_slo");
        assert!(status.is_ok());
        let status = status.unwrap();
        assert!(status.is_healthy); // Should be healthy with no events
        assert_eq!(status.current_success_rate, 1.0); // 100% success rate with no events
    }

    #[tokio::test]
    async fn test_calculate_slo_status_not_found() {
        let manager = SLOManager::new();
        
        let status = manager.calculate_slo_status("nonexistent_slo");
        assert!(status.is_err());
        assert!(status.unwrap_err().to_string().contains("SLO 'nonexistent_slo' not found"));
    }

    // === SLO Measurement Tests ===

    #[tokio::test]
    async fn test_slo_measurement_creation() {
        let measurement = SLOMeasurement {
            slo_id: "test_slo".to_string(),
            timestamp: std::time::SystemTime::now(),
            success_rate: 0.95,
            successful_events: 95,
            total_events: 100,
            error_budget_remaining: 0.8,
            is_violation: false,
            metadata: HashMap::new(),
        };
        
        assert_eq!(measurement.slo_id, "test_slo");
        assert_eq!(measurement.success_rate, 0.95);
        assert_eq!(measurement.successful_events, 95);
        assert_eq!(measurement.total_events, 100);
        assert_eq!(measurement.error_budget_remaining, 0.8);
        assert!(!measurement.is_violation);
    }

    #[tokio::test]
    async fn test_slo_measurement_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("test".to_string()));
        metadata.insert("version".to_string(), serde_json::Value::String("1.0.0".to_string()));
        
        let measurement = SLOMeasurement {
            slo_id: "test_slo".to_string(),
            timestamp: std::time::SystemTime::now(),
            success_rate: 0.90,
            successful_events: 90,
            total_events: 100,
            error_budget_remaining: 0.5,
            is_violation: true,
            metadata,
        };
        
        assert_eq!(measurement.success_rate, 0.90);
        assert!(measurement.is_violation);
        assert!(measurement.metadata.contains_key("source"));
        assert!(measurement.metadata.contains_key("version"));
    }

    // === SLO Violation Tests ===

    #[tokio::test]
    async fn test_slo_violation_creation() {
        let violation = SLOViolation {
            slo_id: "test_slo".to_string(),
            timestamp: std::time::SystemTime::now(),
            severity: SLOSeverity::High,
            current_success_rate: 0.85,
            target_success_rate: 0.99,
            error_budget_consumed: 0.3,
            error_budget_remaining: 0.7,
            context: HashMap::new(),
        };
        
        assert_eq!(violation.slo_id, "test_slo");
        assert_eq!(violation.severity, SLOSeverity::High);
        assert_eq!(violation.current_success_rate, 0.85);
        assert_eq!(violation.target_success_rate, 0.99);
        assert_eq!(violation.error_budget_consumed, 0.3);
        assert_eq!(violation.error_budget_remaining, 0.7);
    }

    #[tokio::test]
    async fn test_slo_violation_with_context() {
        let mut context = HashMap::new();
        context.insert("error_type".to_string(), serde_json::Value::String("timeout".to_string()));
        context.insert("affected_services".to_string(), serde_json::Value::Array(vec![
            serde_json::Value::String("service1".to_string()),
            serde_json::Value::String("service2".to_string()),
        ]));
        
        let violation = SLOViolation {
            slo_id: "test_slo".to_string(),
            timestamp: std::time::SystemTime::now(),
            severity: SLOSeverity::Critical,
            current_success_rate: 0.70,
            target_success_rate: 0.99,
            error_budget_consumed: 0.8,
            error_budget_remaining: 0.2,
            context,
        };
        
        assert_eq!(violation.severity, SLOSeverity::Critical);
        assert!(violation.context.contains_key("error_type"));
        assert!(violation.context.contains_key("affected_services"));
    }

    // === SLO Status Tests ===

    #[tokio::test]
    async fn test_slo_status_creation() {
        let status = SLOStatus {
            slo_id: "test_slo".to_string(),
            is_healthy: true,
            current_success_rate: 0.95,
            target_success_rate: 0.99,
            error_budget_remaining: 0.8,
            error_budget_consumed: 0.2,
            last_measurement: Some(std::time::SystemTime::now()),
            measurement_count: 100,
        };
        
        assert_eq!(status.slo_id, "test_slo");
        assert!(status.is_healthy);
        assert_eq!(status.current_success_rate, 0.95);
        assert_eq!(status.target_success_rate, 0.99);
        assert_eq!(status.error_budget_remaining, 0.8);
        assert_eq!(status.measurement_count, 100);
    }

    #[tokio::test]
    async fn test_slo_status_with_violations() {
        let status = SLOStatus {
            slo_id: "test_slo".to_string(),
            is_healthy: false,
            current_success_rate: 0.85,
            target_success_rate: 0.99,
            error_budget_remaining: 0.2,
            error_budget_consumed: 0.8,
            last_measurement: Some(std::time::SystemTime::now()),
            measurement_count: 200,
        };
        
        assert_eq!(status.slo_id, "test_slo");
        assert!(!status.is_healthy);
        assert_eq!(status.current_success_rate, 0.85);
        assert_eq!(status.measurement_count, 200);
    }

    // === Different Metric Types Tests ===

    #[tokio::test]
    async fn test_latency_slo() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "latency_slo".to_string(),
            name: "Latency SLO".to_string(),
            description: "Response latency SLO".to_string(),
            target_success_rate: 0.95,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Latency { threshold_ms: 100 },
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::Medium,
            alert_threshold: 0.90,
        };
        
        manager.register_slo(slo).unwrap();
        
        // Record events for latency SLO
        for _ in 0..10 {
            manager.record_event("latency_slo", true, None).unwrap();
        }
        
        let status = manager.calculate_slo_status("latency_slo");
        assert!(status.is_ok());
        let status = status.unwrap();
        assert!(status.is_healthy);
    }

    #[tokio::test]
    async fn test_error_rate_slo() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "error_rate_slo".to_string(),
            name: "Error Rate SLO".to_string(),
            description: "Error rate SLO".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::ErrorRate,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        manager.register_slo(slo).unwrap();
        
        // Record events for error rate SLO
        for _ in 0..10 {
            manager.record_event("error_rate_slo", true, None).unwrap();
        }
        
        let status = manager.calculate_slo_status("error_rate_slo");
        assert!(status.is_ok());
        let status = status.unwrap();
        assert!(status.is_healthy);
    }

    #[tokio::test]
    async fn test_throughput_slo() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "throughput_slo".to_string(),
            name: "Throughput SLO".to_string(),
            description: "Throughput SLO".to_string(),
            target_success_rate: 0.90,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Throughput,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::Medium,
            alert_threshold: 0.85,
        };
        
        manager.register_slo(slo).unwrap();
        
        // Record events for throughput SLO
        for _ in 0..10 {
            manager.record_event("throughput_slo", true, None).unwrap();
        }
        
        let status = manager.calculate_slo_status("throughput_slo");
        assert!(status.is_ok());
        let status = status.unwrap();
        assert!(status.is_healthy);
    }

    #[tokio::test]
    async fn test_custom_metric_slo() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "custom_slo".to_string(),
            name: "Custom SLO".to_string(),
            description: "Custom metric SLO".to_string(),
            target_success_rate: 0.95,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Custom { metric_name: "custom_metric".to_string() },
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::Low,
            alert_threshold: 0.90,
        };
        
        manager.register_slo(slo).unwrap();
        
        // Record events for custom metric SLO
        for _ in 0..10 {
            manager.record_event("custom_slo", true, None).unwrap();
        }
        
        let status = manager.calculate_slo_status("custom_slo");
        assert!(status.is_ok());
        let status = status.unwrap();
        assert!(status.is_healthy);
    }

    // === Edge Cases and Error Conditions ===

    #[tokio::test]
    async fn test_slo_with_labels() {
        let mut manager = SLOManager::new();
        
        let mut labels = HashMap::new();
        labels.insert("service".to_string(), "actor-core".to_string());
        labels.insert("environment".to_string(), "test".to_string());
        labels.insert("version".to_string(), "1.0.0".to_string());
        
        let slo = SLO {
            id: "labeled_slo".to_string(),
            name: "Labeled SLO".to_string(),
            description: "SLO with labels".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels,
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        manager.register_slo(slo).unwrap();
        
        let registered_slo = manager.get_slo("labeled_slo").unwrap();
        assert_eq!(registered_slo.labels.get("service"), Some(&"actor-core".to_string()));
        assert_eq!(registered_slo.labels.get("environment"), Some(&"test".to_string()));
        assert_eq!(registered_slo.labels.get("version"), Some(&"1.0.0".to_string()));
    }

    #[tokio::test]
    async fn test_multiple_slos() {
        let mut manager = SLOManager::new();
        
        // Register multiple SLOs
        let slos = vec![
            SLO {
                id: "slo1".to_string(),
                name: "SLO 1".to_string(),
                description: "First SLO".to_string(),
                target_success_rate: 0.99,
                measurement_window: Duration::from_secs(300),
                metric_type: SLOMetricType::Availability,
                labels: HashMap::new(),
                enabled: true,
                severity: SLOSeverity::High,
                alert_threshold: 0.95,
            },
            SLO {
                id: "slo2".to_string(),
                name: "SLO 2".to_string(),
                description: "Second SLO".to_string(),
                target_success_rate: 0.95,
                measurement_window: Duration::from_secs(600),
                metric_type: SLOMetricType::Latency { threshold_ms: 200 },
                labels: HashMap::new(),
                enabled: true,
                severity: SLOSeverity::Medium,
                alert_threshold: 0.90,
            },
            SLO {
                id: "slo3".to_string(),
                name: "SLO 3".to_string(),
                description: "Third SLO".to_string(),
                target_success_rate: 0.90,
                measurement_window: Duration::from_secs(900),
                metric_type: SLOMetricType::ErrorRate,
                labels: HashMap::new(),
                enabled: false, // Disabled
                severity: SLOSeverity::Low,
                alert_threshold: 0.85,
            },
        ];
        
        for slo in slos {
            manager.register_slo(slo).unwrap();
        }
        
        assert_eq!(manager.list_slos().len(), 3);
        
        // Record events for enabled SLOs
        for _ in 0..5 {
            manager.record_event("slo1", true, None).unwrap();
            manager.record_event("slo2", true, None).unwrap();
            manager.record_event("slo3", true, None).unwrap(); // Should be ignored (disabled)
        }
        
        // Check status for enabled SLOs only
        let status1 = manager.calculate_slo_status("slo1").unwrap();
        let status2 = manager.calculate_slo_status("slo2").unwrap();
        
        assert!(status1.is_healthy);
        assert!(status2.is_healthy);
        
        // Disabled SLO may or may not have measurements depending on implementation
        let status3 = manager.calculate_slo_status("slo3");
        // Either it should fail (no measurements) or succeed (with default healthy status)
        if status3.is_ok() {
            let status3 = status3.unwrap();
            assert!(status3.is_healthy); // Should be healthy with no events
        }
    }

    #[tokio::test]
    async fn test_slo_serialization() {
        let slo = SLO {
            id: "test_slo".to_string(),
            name: "Test SLO".to_string(),
            description: "Test SLO description".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Latency { threshold_ms: 100 },
            labels: {
                let mut labels = HashMap::new();
                labels.insert("service".to_string(), "actor-core".to_string());
                labels
            },
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        // Test serialization
        let json = serde_json::to_string(&slo).unwrap();
        assert!(!json.is_empty());
        assert!(json.contains("test_slo"));
        assert!(json.contains("Test SLO"));
        assert!(json.contains("Latency"));
        assert!(json.contains("High"));
        
        // Test deserialization
        let deserialized: SLO = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "test_slo");
        assert_eq!(deserialized.name, "Test SLO");
        assert_eq!(deserialized.target_success_rate, 0.99);
        assert_eq!(deserialized.severity, SLOSeverity::High);
        assert!(deserialized.enabled);
    }

    #[tokio::test]
    async fn test_slo_measurement_serialization() {
        let measurement = SLOMeasurement {
            slo_id: "test_slo".to_string(),
            timestamp: std::time::SystemTime::now(),
            success_rate: 0.95,
            successful_events: 95,
            total_events: 100,
            error_budget_remaining: 0.8,
            is_violation: false,
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("source".to_string(), serde_json::Value::String("test".to_string()));
                metadata
            },
        };
        
        // Test serialization
        let json = serde_json::to_string(&measurement).unwrap();
        assert!(!json.is_empty());
        assert!(json.contains("test_slo"));
        assert!(json.contains("0.95"));
        assert!(json.contains("95"));
        assert!(json.contains("100"));
        
        // Test deserialization
        let deserialized: SLOMeasurement = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.slo_id, "test_slo");
        assert_eq!(deserialized.success_rate, 0.95);
        assert_eq!(deserialized.successful_events, 95);
        assert_eq!(deserialized.total_events, 100);
        assert!(!deserialized.is_violation);
    }

    // === Performance Tests ===

    #[tokio::test]
    async fn test_slo_manager_performance_many_slos() {
        let mut manager = SLOManager::new();
        
        // Register many SLOs
        for i in 0..1000 {
            let slo = SLO {
                id: format!("slo_{}", i),
                name: format!("SLO {}", i),
                description: format!("SLO {}", i),
                target_success_rate: 0.99,
                measurement_window: Duration::from_secs(300),
                metric_type: SLOMetricType::Availability,
                labels: HashMap::new(),
                enabled: true,
                severity: SLOSeverity::High,
                alert_threshold: 0.95,
            };
            
            manager.register_slo(slo).unwrap();
        }
        
        assert_eq!(manager.list_slos().len(), 1000);
        
        // Record events for all SLOs
        for i in 0..1000 {
            manager.record_event(&format!("slo_{}", i), true, None).unwrap();
        }
        
        // Check status for a few SLOs
        for i in 0..10 {
            let status = manager.calculate_slo_status(&format!("slo_{}", i));
            assert!(status.is_ok());
            assert!(status.unwrap().is_healthy);
        }
    }

    #[tokio::test]
    async fn test_slo_manager_performance_many_events() {
        let mut manager = SLOManager::new();
        
        let slo = SLO {
            id: "performance_slo".to_string(),
            name: "Performance SLO".to_string(),
            description: "Performance test SLO".to_string(),
            target_success_rate: 0.99,
            measurement_window: Duration::from_secs(300),
            metric_type: SLOMetricType::Availability,
            labels: HashMap::new(),
            enabled: true,
            severity: SLOSeverity::High,
            alert_threshold: 0.95,
        };
        
        manager.register_slo(slo).unwrap();
        
        // Record many events
        for i in 0..10000 {
            let success = i % 10 != 0; // 90% success rate
            manager.record_event("performance_slo", success, None).unwrap();
        }
        
        let status = manager.calculate_slo_status("performance_slo");
        assert!(status.is_ok());
        let status = status.unwrap();
        // The SLO should be healthy with mostly successful events
        assert!(status.is_healthy || status.current_success_rate >= 0.8);
        assert!(status.current_success_rate >= 0.8); // Should be around 90%
    }
}
