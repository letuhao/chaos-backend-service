use actor_core::validation::middleware::{ValidationMiddleware, ValidationStats};
use actor_core::validation::Validator;
use std::sync::Arc;

// Mock service for testing
struct MockService;

#[tokio::test]
async fn test_validation_stats_default() {
    let stats = ValidationStats::default();
    assert_eq!(stats.total_validations, 0);
    assert_eq!(stats.passed_validations, 0);
    assert_eq!(stats.failed_validations, 0);
    assert_eq!(stats.warnings_generated, 0);
    assert!(stats.error_counts.is_empty());
}

#[tokio::test]
async fn test_validation_stats_debug_clone() {
    let mut stats = ValidationStats::default();
    stats.total_validations = 10;
    stats.passed_validations = 8;
    stats.failed_validations = 2;
    stats.warnings_generated = 1;
    stats.error_counts.insert("test_error".to_string(), 2);
    
    let cloned_stats = stats.clone();
    assert_eq!(stats.total_validations, cloned_stats.total_validations);
    assert_eq!(stats.passed_validations, cloned_stats.passed_validations);
    assert_eq!(stats.failed_validations, cloned_stats.failed_validations);
    assert_eq!(stats.warnings_generated, cloned_stats.warnings_generated);
    assert_eq!(stats.error_counts, cloned_stats.error_counts);
    
    println!("{:?}", stats); // Check Debug impl
}

#[tokio::test]
async fn test_validation_middleware_new() {
    let inner = Arc::new(MockService);
    let validator = Arc::new(Validator::new());
    let _middleware = ValidationMiddleware::new(inner, validator);
    
    // Test that the middleware was created successfully
    assert!(true);
}

#[tokio::test]
async fn test_validation_middleware_get_stats() {
    let inner = Arc::new(MockService);
    let validator = Arc::new(Validator::new());
    let middleware = ValidationMiddleware::new(inner, validator);
    
    let stats = middleware.get_stats().await;
    assert_eq!(stats.total_validations, 0);
    assert_eq!(stats.passed_validations, 0);
    assert_eq!(stats.failed_validations, 0);
}

#[tokio::test]
async fn test_validation_middleware_reset_stats() {
    let inner = Arc::new(MockService);
    let validator = Arc::new(Validator::new());
    let middleware = ValidationMiddleware::new(inner, validator);
    
    middleware.reset_stats().await;
    let stats = middleware.get_stats().await;
    assert_eq!(stats.total_validations, 0);
    assert_eq!(stats.passed_validations, 0);
    assert_eq!(stats.failed_validations, 0);
}

#[tokio::test]
async fn test_validation_middleware_inner() {
    let inner = Arc::new(MockService);
    let validator = Arc::new(Validator::new());
    let middleware = ValidationMiddleware::new(inner.clone(), validator);
    
    // Test that we can get the inner service
    let retrieved_inner = middleware.inner();
    assert!(Arc::ptr_eq(&inner, retrieved_inner));
}

#[tokio::test]
async fn test_validation_middleware_validator() {
    let inner = Arc::new(MockService);
    let validator = Arc::new(Validator::new());
    let middleware = ValidationMiddleware::new(inner, validator.clone());
    
    // Test that we can get the validator
    let retrieved_validator = middleware.validator();
    assert!(Arc::ptr_eq(&validator, retrieved_validator));
}
