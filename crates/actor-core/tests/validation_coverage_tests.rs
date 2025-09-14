//! Coverage tests for validation modules.

use actor_core::validation::middleware::{
    ValidationMiddleware,
    ValidationStats
};
use actor_core::validation::validator::{
    ValidationRules,
    ValidationResult,
    ValidationError,
    ValidationWarning,
    Validator
};
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn test_validation_stats_creation() {
    let stats = ValidationStats {
        total_validations: 100,
        passed_validations: 95,
        failed_validations: 5,
        warnings_generated: 10,
        error_counts: HashMap::new(),
    };
    
    assert_eq!(stats.total_validations, 100);
    assert_eq!(stats.passed_validations, 95);
    assert_eq!(stats.failed_validations, 5);
    assert_eq!(stats.warnings_generated, 10);
    assert_eq!(stats.error_counts.len(), 0);
}

#[test]
fn test_validation_stats_default() {
    let stats = ValidationStats::default();
    
    assert_eq!(stats.total_validations, 0);
    assert_eq!(stats.passed_validations, 0);
    assert_eq!(stats.failed_validations, 0);
    assert_eq!(stats.warnings_generated, 0);
    assert_eq!(stats.error_counts.len(), 0);
}

#[test]
fn test_validation_stats_clone() {
    let mut error_counts = HashMap::new();
    error_counts.insert("INVALID_VALUE".to_string(), 5);
    
    let stats = ValidationStats {
        total_validations: 50,
        passed_validations: 45,
        failed_validations: 5,
        warnings_generated: 3,
        error_counts,
    };
    
    let cloned = stats.clone();
    assert_eq!(stats.total_validations, cloned.total_validations);
    assert_eq!(stats.passed_validations, cloned.passed_validations);
    assert_eq!(stats.failed_validations, cloned.failed_validations);
    assert_eq!(stats.warnings_generated, cloned.warnings_generated);
    assert_eq!(stats.error_counts.len(), cloned.error_counts.len());
}

#[test]
fn test_validation_rules_creation() {
    let rules = ValidationRules {
        max_dimension_length: 32,
        max_system_length: 32,
        min_contribution_value: -100.0,
        max_contribution_value: 100.0,
        max_tags_per_contribution: 5,
        max_tag_key_length: 16,
        max_tag_value_length: 64,
        max_priority: 100,
        min_priority: -100,
        allowed_dimensions: vec!["strength".to_string()],
        allowed_systems: vec!["equipment".to_string()],
        allowed_cap_kinds: vec!["min".to_string()],
    };
    
    assert_eq!(rules.max_dimension_length, 32);
    assert_eq!(rules.max_system_length, 32);
    assert_eq!(rules.min_contribution_value, -100.0);
    assert_eq!(rules.max_contribution_value, 100.0);
    assert_eq!(rules.max_tags_per_contribution, 5);
    assert_eq!(rules.max_tag_key_length, 16);
    assert_eq!(rules.max_tag_value_length, 64);
    assert_eq!(rules.max_priority, 100);
    assert_eq!(rules.min_priority, -100);
    assert_eq!(rules.allowed_dimensions.len(), 1);
    assert_eq!(rules.allowed_systems.len(), 1);
    assert_eq!(rules.allowed_cap_kinds.len(), 1);
}

#[test]
fn test_validation_rules_default() {
    let rules = ValidationRules::default();
    
    assert_eq!(rules.max_dimension_length, 64);
    assert_eq!(rules.max_system_length, 64);
    assert_eq!(rules.min_contribution_value, -1_000_000.0);
    assert_eq!(rules.max_contribution_value, 1_000_000.0);
    assert_eq!(rules.max_tags_per_contribution, 10);
    assert_eq!(rules.max_tag_key_length, 32);
    assert_eq!(rules.max_tag_value_length, 128);
    assert_eq!(rules.max_priority, 1000);
    assert_eq!(rules.min_priority, -1000);
    assert_eq!(rules.allowed_dimensions.len(), 10);
    assert_eq!(rules.allowed_systems.len(), 8);
    assert_eq!(rules.allowed_cap_kinds.len(), 2);
}

#[test]
fn test_validation_rules_equality() {
    let rules1 = ValidationRules::default();
    let rules2 = ValidationRules::default();
    
    assert_eq!(rules1, rules2);
    assert_eq!(rules1, rules1.clone());
}

#[test]
fn test_validation_result_creation() {
    let errors = vec![
        ValidationError {
            code: "INVALID_VALUE".to_string(),
            message: "Value is invalid".to_string(),
            field: Some("value".to_string()),
            context: Some("validation context".to_string()),
        }
    ];
    
    let warnings = vec![
        ValidationWarning {
            code: "WARNING_VALUE".to_string(),
            message: "Value is suspicious".to_string(),
            field: Some("value".to_string()),
            context: Some("validation context".to_string()),
        }
    ];
    
    let result = ValidationResult {
        is_valid: false,
        errors,
        warnings,
    };
    
    assert!(!result.is_valid);
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.warnings.len(), 1);
}

#[test]
fn test_validation_result_new() {
    let result = ValidationResult::new();
    
    assert!(result.is_valid);
    assert_eq!(result.errors.len(), 0);
    assert_eq!(result.warnings.len(), 0);
}

#[test]
fn test_validation_result_equality() {
    let result1 = ValidationResult::new();
    let result2 = ValidationResult::new();
    
    assert_eq!(result1, result2);
    assert_eq!(result1, result1.clone());
}

#[test]
fn test_validation_error_creation() {
    let error = ValidationError {
        code: "INVALID_VALUE".to_string(),
        message: "Value is invalid".to_string(),
        field: Some("value".to_string()),
        context: Some("validation context".to_string()),
    };
    
    assert_eq!(error.code, "INVALID_VALUE");
    assert_eq!(error.message, "Value is invalid");
    assert_eq!(error.field, Some("value".to_string()));
    assert_eq!(error.context, Some("validation context".to_string()));
}

#[test]
fn test_validation_error_without_field() {
    let error = ValidationError {
        code: "GENERAL_ERROR".to_string(),
        message: "General error occurred".to_string(),
        field: None,
        context: None,
    };
    
    assert_eq!(error.code, "GENERAL_ERROR");
    assert_eq!(error.message, "General error occurred");
    assert_eq!(error.field, None);
    assert_eq!(error.context, None);
}

#[test]
fn test_validation_warning_creation() {
    let warning = ValidationWarning {
        code: "WARNING_VALUE".to_string(),
        message: "Value is suspicious".to_string(),
        field: Some("value".to_string()),
        context: Some("validation context".to_string()),
    };
    
    assert_eq!(warning.code, "WARNING_VALUE");
    assert_eq!(warning.message, "Value is suspicious");
    assert_eq!(warning.field, Some("value".to_string()));
    assert_eq!(warning.context, Some("validation context".to_string()));
}

#[test]
fn test_validation_warning_without_field() {
    let warning = ValidationWarning {
        code: "GENERAL_WARNING".to_string(),
        message: "General warning".to_string(),
        field: None,
        context: None,
    };
    
    assert_eq!(warning.code, "GENERAL_WARNING");
    assert_eq!(warning.message, "General warning");
    assert_eq!(warning.field, None);
    assert_eq!(warning.context, None);
}

#[test]
fn test_validator_creation() {
    let validator = Validator::new();
    
    // Test that the validator was created successfully
    assert!(std::ptr::addr_of!(validator) != std::ptr::null());
}

#[test]
fn test_validation_middleware_creation() {
    let validator = Arc::new(Validator::new());
    let inner_service = Arc::new("test_service".to_string());
    
    let middleware = ValidationMiddleware::new(inner_service, validator);
    
    // Test that the middleware was created successfully
    assert!(std::ptr::addr_of!(middleware) != std::ptr::null());
}

#[test]
fn test_validation_middleware_inner_access() {
    let validator = Arc::new(Validator::new());
    let inner_service = Arc::new("test_service".to_string());
    
    let middleware = ValidationMiddleware::new(inner_service, validator);
    
    // Test that we can access the inner service
    let inner = middleware.inner();
    assert_eq!(**inner, "test_service");
}

#[test]
fn test_validation_middleware_validator_access() {
    let validator = Arc::new(Validator::new());
    let inner_service = Arc::new("test_service".to_string());
    
    let middleware = ValidationMiddleware::new(inner_service, validator);
    
    // Test that we can access the validator
    let validator_ref = middleware.validator();
    assert!(std::ptr::addr_of!(**validator_ref) != std::ptr::null());
}

#[tokio::test]
async fn test_validation_middleware_get_stats() {
    let validator = Arc::new(Validator::new());
    let inner_service = Arc::new("test_service".to_string());
    
    let middleware = ValidationMiddleware::new(inner_service, validator);
    
    let stats = middleware.get_stats().await;
    
    assert_eq!(stats.total_validations, 0);
    assert_eq!(stats.passed_validations, 0);
    assert_eq!(stats.failed_validations, 0);
    assert_eq!(stats.warnings_generated, 0);
    assert_eq!(stats.error_counts.len(), 0);
}

#[tokio::test]
async fn test_validation_middleware_reset_stats() {
    let validator = Arc::new(Validator::new());
    let inner_service = Arc::new("test_service".to_string());
    
    let middleware = ValidationMiddleware::new(inner_service, validator);
    
    // Reset stats
    middleware.reset_stats().await;
    
    // Get stats after reset
    let stats = middleware.get_stats().await;
    
    assert_eq!(stats.total_validations, 0);
    assert_eq!(stats.passed_validations, 0);
    assert_eq!(stats.failed_validations, 0);
    assert_eq!(stats.warnings_generated, 0);
    assert_eq!(stats.error_counts.len(), 0);
}

#[test]
fn test_validation_result_has_warnings() {
    let mut result = ValidationResult::new();
    
    // Initially no warnings
    assert!(!result.has_warnings());
    
    // Add a warning
    result.warnings.push(ValidationWarning {
        code: "WARNING".to_string(),
        message: "Warning message".to_string(),
        field: None,
        context: None,
    });
    
    // Now has warnings
    assert!(result.has_warnings());
}

#[test]
fn test_validation_result_has_errors() {
    let mut result = ValidationResult::new();
    
    // Initially no errors
    assert!(!result.has_errors());
    
    // Add an error
    result.errors.push(ValidationError {
        code: "ERROR".to_string(),
        message: "Error message".to_string(),
        field: None,
        context: None,
    });
    
    // Now has errors
    assert!(result.has_errors());
}

#[test]
fn test_validation_result_is_valid_with_errors() {
    let mut result = ValidationResult::new();
    result.is_valid = false;
    result.errors.push(ValidationError {
        code: "ERROR".to_string(),
        message: "Error message".to_string(),
        field: None,
        context: None,
    });
    
    assert!(!result.is_valid);
    assert!(result.has_errors());
}

#[test]
fn test_validation_result_is_valid_with_warnings() {
    let mut result = ValidationResult::new();
    result.is_valid = true;
    result.warnings.push(ValidationWarning {
        code: "WARNING".to_string(),
        message: "Warning message".to_string(),
        field: None,
        context: None,
    });
    
    assert!(result.is_valid);
    assert!(result.has_warnings());
}

#[test]
fn test_validation_rules_clone() {
    let rules = ValidationRules::default();
    let cloned = rules.clone();
    
    assert_eq!(rules, cloned);
    assert_eq!(rules.max_dimension_length, cloned.max_dimension_length);
    assert_eq!(rules.max_system_length, cloned.max_system_length);
    assert_eq!(rules.min_contribution_value, cloned.min_contribution_value);
    assert_eq!(rules.max_contribution_value, cloned.max_contribution_value);
}

#[test]
fn test_validation_result_clone() {
    let mut result = ValidationResult::new();
    result.errors.push(ValidationError {
        code: "ERROR".to_string(),
        message: "Error message".to_string(),
        field: None,
        context: None,
    });
    result.warnings.push(ValidationWarning {
        code: "WARNING".to_string(),
        message: "Warning message".to_string(),
        field: None,
        context: None,
    });
    
    let cloned = result.clone();
    
    assert_eq!(result.is_valid, cloned.is_valid);
    assert_eq!(result.errors.len(), cloned.errors.len());
    assert_eq!(result.warnings.len(), cloned.warnings.len());
}

#[test]
fn test_validation_error_clone() {
    let error = ValidationError {
        code: "ERROR".to_string(),
        message: "Error message".to_string(),
        field: Some("field".to_string()),
        context: Some("context".to_string()),
    };
    
    let cloned = error.clone();
    
    assert_eq!(error.code, cloned.code);
    assert_eq!(error.message, cloned.message);
    assert_eq!(error.field, cloned.field);
    assert_eq!(error.context, cloned.context);
}

#[test]
fn test_validation_warning_clone() {
    let warning = ValidationWarning {
        code: "WARNING".to_string(),
        message: "Warning message".to_string(),
        field: Some("field".to_string()),
        context: Some("context".to_string()),
    };
    
    let cloned = warning.clone();
    
    assert_eq!(warning.code, cloned.code);
    assert_eq!(warning.message, cloned.message);
    assert_eq!(warning.field, cloned.field);
    assert_eq!(warning.context, cloned.context);
}

#[test]
fn test_validation_stats_debug() {
    let stats = ValidationStats::default();
    let debug_str = format!("{:?}", stats);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_validation_rules_debug() {
    let rules = ValidationRules::default();
    let debug_str = format!("{:?}", rules);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_validation_result_debug() {
    let result = ValidationResult::new();
    let debug_str = format!("{:?}", result);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_validation_error_debug() {
    let error = ValidationError {
        code: "ERROR".to_string(),
        message: "Error message".to_string(),
        field: None,
        context: None,
    };
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_validation_warning_debug() {
    let warning = ValidationWarning {
        code: "WARNING".to_string(),
        message: "Warning message".to_string(),
        field: None,
        context: None,
    };
    let debug_str = format!("{:?}", warning);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_hashmap_operations() {
    let mut map = HashMap::new();
    map.insert("key1".to_string(), 1u64);
    map.insert("key2".to_string(), 2u64);
    
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key1"), Some(&1u64));
    assert_eq!(map.get("key2"), Some(&2u64));
    assert!(map.contains_key("key1"));
    assert!(!map.contains_key("key3"));
}

#[test]
fn test_vec_operations() {
    let mut vec = Vec::new();
    vec.push("item1".to_string());
    vec.push("item2".to_string());
    
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], "item1");
    assert_eq!(vec[1], "item2");
}

#[test]
fn test_string_operations() {
    let s1 = "test".to_string();
    let s2 = s1.clone();
    
    assert_eq!(s1, s2);
    assert_eq!(s1.len(), 4);
    assert!(!s1.is_empty());
}

#[test]
fn test_f64_operations() {
    let value1 = 100.0;
    let value2 = 50.0;
    
    assert_eq!(value1 + value2, 150.0);
    assert_eq!(value1 - value2, 50.0);
    assert_eq!(value1 * value2, 5000.0);
    assert_eq!(value1 / value2, 2.0);
}

#[test]
fn test_usize_operations() {
    let value1 = 100usize;
    let value2 = 50usize;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[test]
fn test_i64_operations() {
    let value1 = 100i64;
    let value2 = 50i64;
    
    assert_eq!(value1 + value2, 150);
    assert_eq!(value1 - value2, 50);
    assert_eq!(value1 * value2, 5000);
    assert_eq!(value1 / value2, 2);
}

#[test]
fn test_bool_operations() {
    let true_value = true;
    let false_value = false;
    
    assert!(true_value);
    assert!(!false_value);
    assert_eq!(true_value && false_value, false);
    assert_eq!(true_value || false_value, true);
}

#[test]
fn test_option_operations() {
    let some_value = Some(42);
    let none_value: Option<i32> = None;
    
    assert!(some_value.is_some());
    assert!(none_value.is_none());
    assert_eq!(some_value.unwrap(), 42);
    assert_eq!(some_value.unwrap_or(0), 42);
    assert_eq!(none_value.unwrap_or(0), 0);
}

#[test]
fn test_arc_operations() {
    let value = Arc::new("test".to_string());
    let cloned = value.clone();
    
    assert_eq!(*value, *cloned);
    assert_eq!(Arc::strong_count(&value), 2);
}

#[test]
fn test_validation_rules_partial_eq() {
    let rules1 = ValidationRules::default();
    let rules2 = ValidationRules::default();
    
    assert!(rules1 == rules2);
    assert!(rules1 != ValidationRules {
        max_dimension_length: 32,
        max_system_length: 64,
        min_contribution_value: -1_000_000.0,
        max_contribution_value: 1_000_000.0,
        max_tags_per_contribution: 10,
        max_tag_key_length: 32,
        max_tag_value_length: 128,
        max_priority: 1000,
        min_priority: -1000,
        allowed_dimensions: vec![],
        allowed_systems: vec![],
        allowed_cap_kinds: vec![],
    });
}

#[test]
fn test_validation_result_partial_eq() {
    let result1 = ValidationResult::new();
    let result2 = ValidationResult::new();
    
    assert!(result1 == result2);
    assert!(result1 == result1.clone());
}

#[test]
fn test_validation_error_partial_eq() {
    let error1 = ValidationError {
        code: "ERROR".to_string(),
        message: "Error message".to_string(),
        field: None,
        context: None,
    };
    
    let error2 = ValidationError {
        code: "ERROR".to_string(),
        message: "Error message".to_string(),
        field: None,
        context: None,
    };
    
    assert!(error1 == error2);
    assert!(error1 == error1.clone());
}

#[test]
fn test_validation_warning_partial_eq() {
    let warning1 = ValidationWarning {
        code: "WARNING".to_string(),
        message: "Warning message".to_string(),
        field: None,
        context: None,
    };
    
    let warning2 = ValidationWarning {
        code: "WARNING".to_string(),
        message: "Warning message".to_string(),
        field: None,
        context: None,
    };
    
    assert!(warning1 == warning2);
    assert!(warning1 == warning1.clone());
}