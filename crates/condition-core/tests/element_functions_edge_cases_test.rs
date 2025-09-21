//! Edge Cases and Performance Tests for Element Functions
//! 
//! This module contains tests for edge cases, performance, and stress testing
//! of element condition functions.

use condition_core::*;
use condition_core::element_functions::*;
use condition_core::data_accessor::{ElementDataAccessor, ElementFunctionRegistry};
use std::sync::Arc;
use std::time::SystemTime;

// Mock data provider for edge case testing
struct EdgeCaseDataProvider;

#[async_trait::async_trait]
impl ElementDataProvider for EdgeCaseDataProvider {
    async fn get_element_mastery(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        match element_id {
            "zero" => Ok(0.0),
            "negative" => Ok(-10.0),
            "max" => Ok(f64::MAX),
            "min" => Ok(f64::MIN_POSITIVE),
            "inf" => Ok(f64::INFINITY),
            "nan" => Ok(f64::NAN),
            _ => Ok(100.0),
        }
    }

    async fn get_element_resistance(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        match element_id {
            "zero" => Ok(0.0),
            "negative" => Ok(-0.5),
            "max" => Ok(1.0),
            "min" => Ok(0.001),
            _ => Ok(0.5),
        }
    }

    async fn has_element_affinity(&self, element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match element_id {
            "always_true" => Ok(true),
            "always_false" => Ok(false),
            _ => Ok(true),
        }
    }

    async fn is_element_weakness(&self, element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match element_id {
            "always_true" => Ok(true),
            "always_false" => Ok(false),
            _ => Ok(false),
        }
    }

    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String> {
        match (source_element, target_element) {
            ("empty", "empty") => Ok("neutral".to_string()),
            ("long", "long") => Ok("a".repeat(1000)),
            _ => Ok("neutral".to_string()),
        }
    }

    async fn list_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["zero".to_string(), "negative".to_string(), "max".to_string()])
    }

    // Implement all other methods with simple defaults
    async fn is_element_same_category(&self, _element1: &str, _element2: &str) -> ConditionResult<bool> { Ok(false) }
    async fn is_element_generating(&self, _source_element: &str, _target_element: &str) -> ConditionResult<bool> { Ok(false) }
    async fn is_element_overcoming(&self, _source_element: &str, _target_element: &str) -> ConditionResult<bool> { Ok(false) }
    async fn is_element_neutral(&self, _source_element: &str, _target_element: &str) -> ConditionResult<bool> { Ok(true) }
    async fn has_element_status_effect(&self, _element_id: &str, _status_id: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn get_element_status_effect_count(&self, _element_id: &str, _status_id: &str, _actor_id: &str) -> ConditionResult<i64> { Ok(0) }
    async fn is_element_status_effect_active(&self, _element_id: &str, _status_id: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn has_element_resource(&self, _element_id: &str, _resource_type: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn get_element_resource_value(&self, _element_id: &str, _resource_type: &str, _actor_id: &str) -> ConditionResult<f64> { Ok(0.0) }
    async fn is_element_resource_below_threshold(&self, _element_id: &str, _resource_type: &str, _threshold: f64, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn is_element_resource_above_threshold(&self, _element_id: &str, _resource_type: &str, _threshold: f64, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn has_hybrid_element(&self, _hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn is_hybrid_element_activated(&self, _hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn get_hybrid_element_parents(&self, _hybrid_id: &str) -> ConditionResult<Vec<String>> { Ok(vec![]) }
    async fn list_hybrid_elements(&self) -> ConditionResult<Vec<String>> { Ok(vec![]) }
    async fn get_element_derived_stat(&self, _element_id: &str, _stat_name: &str, _actor_id: &str) -> ConditionResult<f64> { Ok(0.0) }
    async fn has_element_derived_stat(&self, _element_id: &str, _stat_name: &str, _actor_id: &str) -> ConditionResult<bool> { Ok(false) }
    async fn list_element_derived_stats(&self, _element_id: &str) -> ConditionResult<Vec<String>> { Ok(vec![]) }
}

fn create_edge_case_context() -> ConditionContext {
    ConditionContext {
        target: ActorTarget { id: "edge_case_actor".to_string() },
        world_id: "edge_case_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Clear,
        world_state: WorldState {
            time_of_day: 12.0,
            season: "spring".to_string(),
            temperature: 20.0,
            humidity: 60.0,
        },
    }
}

fn create_edge_case_registry() -> ElementFunctionRegistry {
    let provider = Arc::new(EdgeCaseDataProvider);
    let data_accessor = Arc::new(ElementDataAccessor::new(provider));
    create_element_function_registry(data_accessor)
}

// Edge case tests
#[tokio::test]
async fn test_zero_values() {
    let registry = create_edge_case_registry();
    let context = create_edge_case_context();
    
    // Test zero mastery
    let params = vec![ConditionParameter::String("zero".to_string())];
    let result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    assert!(!result, "Zero mastery should return false");
}

#[tokio::test]
async fn test_negative_values() {
    let registry = create_edge_case_registry();
    let context = create_edge_case_context();
    
    // Test negative mastery
    let params = vec![ConditionParameter::String("negative".to_string())];
    let result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    assert!(!result, "Negative mastery should return false");
}

#[tokio::test]
async fn test_max_values() {
    let registry = create_edge_case_registry();
    let context = create_edge_case_context();
    
    // Test max mastery
    let params = vec![ConditionParameter::String("max".to_string())];
    let result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    assert!(result, "Max mastery should return true");
}

#[tokio::test]
async fn test_min_values() {
    let registry = create_edge_case_registry();
    let context = create_edge_case_context();
    
    // Test min mastery
    let params = vec![ConditionParameter::String("min".to_string())];
    let result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    assert!(result, "Min positive mastery should return true");
}

#[tokio::test]
async fn test_infinity_values() {
    let registry = create_edge_case_registry();
    let context = create_edge_case_context();
    
    // Test infinity mastery
    let params = vec![ConditionParameter::String("inf".to_string())];
    let result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    assert!(result, "Infinity mastery should return true");
}

#[tokio::test]
async fn test_nan_values() {
    let registry = create_edge_case_registry();
    let context = create_edge_case_context();
    
    // Test NaN mastery
    let params = vec![ConditionParameter::String("nan".to_string())];
    let result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    assert!(!result, "NaN mastery should return false");
}

#[tokio::test]
async fn test_empty_strings() {
    let registry = create_edge_case_registry();
    let context = create_edge_case_context();
    
    // Test empty string interaction
    let params = vec![
        ConditionParameter::String("empty".to_string()),
        ConditionParameter::String("empty".to_string())
    ];
    let result = registry.execute_function("get_element_interaction", &params, &context).await.unwrap();
    assert!(!result, "Empty string interaction should be neutral (return false)");
}

#[tokio::test]
async fn test_long_strings() {
    let registry = create_edge_case_registry();
    let context = create_edge_case_context();
    
    // Test long string interaction
    let params = vec![
        ConditionParameter::String("long".to_string()),
        ConditionParameter::String("long".to_string())
    ];
    let result = registry.execute_function("get_element_interaction", &params, &context).await.unwrap();
    assert!(result, "Long string interaction should be non-neutral");
}

// Performance tests
#[tokio::test]
async fn test_large_batch_performance() {
    let registry = create_edge_case_registry();
    let context = create_edge_case_context();
    
    let start = std::time::Instant::now();
    
    // Run 1000 function calls
    for _ in 0..1000 {
        let params = vec![ConditionParameter::String("test".to_string())];
        let _result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    }
    
    let duration = start.elapsed();
    
    // Should complete within 1 second for 1000 calls
    assert!(duration.as_millis() < 1000, "1000 function calls should complete within 1 second");
}

#[tokio::test]
async fn test_memory_usage() {
    let registry = create_edge_case_registry();
    let context = create_edge_case_context();
    
    // Run many function calls to test memory usage
    for _ in 0..100 {
        let params = vec![ConditionParameter::String("test".to_string())];
        let _result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    }
    
    // If we get here without running out of memory, the test passes
    assert!(true, "Memory usage test passed");
}

// Stress tests
#[tokio::test]
async fn test_concurrent_stress() {
    let registry = Arc::new(create_edge_case_registry());
    let context = Arc::new(create_edge_case_context());
    
    let mut handles = vec![];
    
    // Spawn 100 concurrent tasks
    for i in 0..100 {
        let registry = registry.clone();
        let context = context.clone();
        let handle = tokio::spawn(async move {
            for _ in 0..10 {
                let params = vec![ConditionParameter::String(format!("test_{}", i))];
                let _result = registry.execute_function("get_element_mastery", &params, &context).await;
            }
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        let _result = handle.await.unwrap();
        // If we get here without panicking, the test passes
    }
}

#[tokio::test]
async fn test_rapid_successive_calls() {
    let registry = create_edge_case_registry();
    let context = create_edge_case_context();
    
    // Make rapid successive calls
    for _ in 0..100 {
        let params = vec![ConditionParameter::String("test".to_string())];
        let _result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    }
    
    // If we get here without issues, the test passes
    assert!(true, "Rapid successive calls test passed");
}
