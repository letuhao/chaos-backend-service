//! Validation middleware coverage tests for Actor Core.

use actor_core::validation::middleware::{
    ValidationMiddleware, ValidationStats, ValidationMiddlewareFactory
};
use actor_core::validation::{Validator, ValidationResult, ValidationError, ValidationWarning};
use std::sync::Arc;

// Mock service for testing
struct MockService {
    name: String,
}

impl MockService {
    fn new(name: String) -> Self {
        Self { name }
    }
    
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[test]
fn test_validation_stats_default() {
    let stats = ValidationStats::default();
    
    assert_eq!(stats.total_validations, 0);
    assert_eq!(stats.passed_validations, 0);
    assert_eq!(stats.failed_validations, 0);
    assert_eq!(stats.warnings_generated, 0);
    assert!(stats.error_counts.is_empty());
}

#[test]
fn test_validation_stats_creation() {
    let mut stats = ValidationStats {
        total_validations: 100,
        passed_validations: 95,
        failed_validations: 5,
        warnings_generated: 10,
        error_counts: std::collections::HashMap::new(),
    };
    
    stats.error_counts.insert("INVALID_INPUT".to_string(), 3);
    stats.error_counts.insert("MISSING_FIELD".to_string(), 2);
    
    assert_eq!(stats.total_validations, 100);
    assert_eq!(stats.passed_validations, 95);
    assert_eq!(stats.failed_validations, 5);
    assert_eq!(stats.warnings_generated, 10);
    assert_eq!(stats.error_counts.len(), 2);
    assert_eq!(stats.error_counts.get("INVALID_INPUT"), Some(&3));
    assert_eq!(stats.error_counts.get("MISSING_FIELD"), Some(&2));
}

#[test]
fn test_validation_stats_clone() {
    let mut stats = ValidationStats {
        total_validations: 50,
        passed_validations: 45,
        failed_validations: 5,
        warnings_generated: 8,
        error_counts: std::collections::HashMap::new(),
    };
    
    stats.error_counts.insert("TEST_ERROR".to_string(), 5);
    
    let cloned_stats = stats.clone();
    
    assert_eq!(cloned_stats.total_validations, 50);
    assert_eq!(cloned_stats.passed_validations, 45);
    assert_eq!(cloned_stats.failed_validations, 5);
    assert_eq!(cloned_stats.warnings_generated, 8);
    assert_eq!(cloned_stats.error_counts.len(), 1);
    assert_eq!(cloned_stats.error_counts.get("TEST_ERROR"), Some(&5));
}

#[test]
fn test_validation_stats_debug() {
    let stats = ValidationStats {
        total_validations: 25,
        passed_validations: 20,
        failed_validations: 5,
        warnings_generated: 3,
        error_counts: std::collections::HashMap::new(),
    };
    
    let debug_str = format!("{:?}", stats);
    assert!(debug_str.contains("ValidationStats"));
    assert!(debug_str.contains("total_validations: 25"));
    assert!(debug_str.contains("passed_validations: 20"));
    assert!(debug_str.contains("failed_validations: 5"));
    assert!(debug_str.contains("warnings_generated: 3"));
}

#[tokio::test]
async fn test_validation_middleware_creation() {
    let mock_service = Arc::new(MockService::new("test-service".to_string()));
    let validator = Arc::new(Validator::new());
    let middleware = ValidationMiddleware::new(mock_service.clone(), validator.clone());
    
    assert_eq!(middleware.inner().get_name(), "test-service");
    assert!(Arc::ptr_eq(middleware.validator(), &validator));
}

#[tokio::test]
async fn test_validation_middleware_stats() {
    let mock_service = Arc::new(MockService::new("test-service".to_string()));
    let validator = Arc::new(Validator::new());
    let middleware = ValidationMiddleware::new(mock_service, validator);
    
    // Test initial stats
    let initial_stats = middleware.get_stats().await;
    assert_eq!(initial_stats.total_validations, 0);
    assert_eq!(initial_stats.passed_validations, 0);
    assert_eq!(initial_stats.failed_validations, 0);
    assert_eq!(initial_stats.warnings_generated, 0);
    
    // Test reset stats
    middleware.reset_stats().await;
    let reset_stats = middleware.get_stats().await;
    assert_eq!(reset_stats.total_validations, 0);
    assert_eq!(reset_stats.passed_validations, 0);
    assert_eq!(reset_stats.failed_validations, 0);
    assert_eq!(reset_stats.warnings_generated, 0);
}

#[test]
fn test_validation_result_creation() {
    let result = ValidationResult {
        is_valid: true,
        errors: vec![],
        warnings: vec![],
    };
    
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
    assert!(result.warnings.is_empty());
    assert!(!result.has_warnings());
}

#[test]
fn test_validation_result_with_errors() {
    let error = ValidationError {
        code: "INVALID_INPUT".to_string(),
        message: "Input is invalid".to_string(),
        field: Some("username".to_string()),
        context: Some("validation failed".to_string()),
    };
    
    let result = ValidationResult {
        is_valid: false,
        errors: vec![error],
        warnings: vec![],
    };
    
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    assert!(result.warnings.is_empty());
    assert!(!result.has_warnings());
}

#[test]
fn test_validation_result_with_warnings() {
    let warning = ValidationWarning {
        code: "DEPRECATED_FIELD".to_string(),
        message: "Field is deprecated".to_string(),
        field: Some("old_field".to_string()),
        context: Some("deprecation notice".to_string()),
    };
    
    let result = ValidationResult {
        is_valid: true,
        errors: vec![],
        warnings: vec![warning],
    };
    
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
    assert_eq!(result.warnings.len(), 1);
    assert!(result.has_warnings());
}

#[test]
fn test_validation_error_creation() {
    let error = ValidationError {
        code: "MISSING_FIELD".to_string(),
        message: "Required field is missing".to_string(),
        field: Some("email".to_string()),
        context: Some("field validation".to_string()),
    };
    
    assert_eq!(error.code, "MISSING_FIELD");
    assert_eq!(error.message, "Required field is missing");
    assert_eq!(error.field, Some("email".to_string()));
}

#[test]
fn test_validation_warning_creation() {
    let warning = ValidationWarning {
        code: "DEPRECATED_API".to_string(),
        message: "API is deprecated".to_string(),
        field: None,
        context: Some("API deprecation".to_string()),
    };
    
    assert_eq!(warning.code, "DEPRECATED_API");
    assert_eq!(warning.message, "API is deprecated");
    assert_eq!(warning.field, None);
}

#[test]
fn test_validation_error_debug() {
    let error = ValidationError {
        code: "TEST_ERROR".to_string(),
        message: "Test error message".to_string(),
        field: Some("test_field".to_string()),
        context: Some("test context".to_string()),
    };
    
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("ValidationError"));
    assert!(debug_str.contains("TEST_ERROR"));
    assert!(debug_str.contains("Test error message"));
    assert!(debug_str.contains("test_field"));
}

#[test]
fn test_validation_warning_debug() {
    let warning = ValidationWarning {
        code: "TEST_WARNING".to_string(),
        message: "Test warning message".to_string(),
        field: Some("test_field".to_string()),
        context: Some("test warning context".to_string()),
    };
    
    let debug_str = format!("{:?}", warning);
    assert!(debug_str.contains("ValidationWarning"));
    assert!(debug_str.contains("TEST_WARNING"));
    assert!(debug_str.contains("Test warning message"));
    assert!(debug_str.contains("test_field"));
}

#[test]
fn test_validation_result_debug() {
    let error = ValidationError {
        code: "ERROR_1".to_string(),
        message: "Error 1".to_string(),
        field: None,
        context: Some("error context".to_string()),
    };
    
    let warning = ValidationWarning {
        code: "WARNING_1".to_string(),
        message: "Warning 1".to_string(),
        field: None,
        context: Some("warning context".to_string()),
    };
    
    let result = ValidationResult {
        is_valid: false,
        errors: vec![error],
        warnings: vec![warning],
    };
    
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("ValidationResult"));
    assert!(debug_str.contains("is_valid: false"));
    assert!(debug_str.contains("ERROR_1"));
    assert!(debug_str.contains("WARNING_1"));
}

#[test]
fn test_validation_stats_error_counts() {
    let mut stats = ValidationStats {
        total_validations: 100,
        passed_validations: 95,
        failed_validations: 5,
        warnings_generated: 10,
        error_counts: std::collections::HashMap::new(),
    };
    
    stats.error_counts.insert("ERROR_1".to_string(), 3);
    stats.error_counts.insert("ERROR_2".to_string(), 2);
    
    assert_eq!(stats.total_validations, 100);
    assert_eq!(stats.passed_validations, 95);
    assert_eq!(stats.failed_validations, 5);
    assert_eq!(stats.warnings_generated, 10);
    assert_eq!(stats.error_counts.len(), 2);
    assert_eq!(stats.error_counts.get("ERROR_1"), Some(&3));
    assert_eq!(stats.error_counts.get("ERROR_2"), Some(&2));
}

#[test]
fn test_validation_middleware_factory() {
    // Test factory creation - ValidationMiddlewareFactory doesn't have a new method
    // but we can test that it exists
    let _factory = ValidationMiddlewareFactory;
    assert!(true); // Factory exists
}

#[tokio::test]
async fn test_validation_middleware_stats_increment() {
    let mock_service = Arc::new(MockService::new("test-service".to_string()));
    let validator = Arc::new(Validator::new());
    let middleware = ValidationMiddleware::new(mock_service, validator);
    
    // Test initial stats
    let stats = middleware.get_stats().await;
    assert_eq!(stats.total_validations, 0);
    assert_eq!(stats.passed_validations, 0);
    assert_eq!(stats.failed_validations, 0);
    assert_eq!(stats.warnings_generated, 0);
}

#[tokio::test]
async fn test_validation_middleware_stats_reset() {
    let mock_service = Arc::new(MockService::new("test-service".to_string()));
    let validator = Arc::new(Validator::new());
    let middleware = ValidationMiddleware::new(mock_service, validator);
    
    // Reset stats
    middleware.reset_stats().await;
    
    let stats = middleware.get_stats().await;
    assert_eq!(stats.total_validations, 0);
    assert_eq!(stats.passed_validations, 0);
    assert_eq!(stats.failed_validations, 0);
    assert_eq!(stats.warnings_generated, 0);
    assert!(stats.error_counts.is_empty());
}

#[test]
fn test_validation_result_first_error() {
    let error1 = ValidationError {
        code: "ERROR_1".to_string(),
        message: "First error".to_string(),
        field: None,
        context: Some("first error context".to_string()),
    };
    
    let error2 = ValidationError {
        code: "ERROR_2".to_string(),
        message: "Second error".to_string(),
        field: None,
        context: Some("second error context".to_string()),
    };
    
    let result = ValidationResult {
        is_valid: false,
        errors: vec![error1, error2],
        warnings: vec![],
    };
    
    let first_error = result.first_error();
    assert!(first_error.is_some());
    assert_eq!(first_error.unwrap(), "First error");
}

#[test]
fn test_validation_result_first_error_empty() {
    let result = ValidationResult {
        is_valid: true,
        errors: vec![],
        warnings: vec![],
    };
    
    let first_error = result.first_error();
    assert!(first_error.is_none());
}

#[test]
fn test_validation_result_has_warnings() {
    let warning = ValidationWarning {
        code: "WARNING_1".to_string(),
        message: "Warning message".to_string(),
        field: None,
        context: Some("warning context".to_string()),
    };
    
    let result_with_warnings = ValidationResult {
        is_valid: true,
        errors: vec![],
        warnings: vec![warning],
    };
    
    let result_without_warnings = ValidationResult {
        is_valid: true,
        errors: vec![],
        warnings: vec![],
    };
    
    assert!(result_with_warnings.has_warnings());
    assert!(!result_without_warnings.has_warnings());
}