//! Unit tests for Element Functions
//! 
//! This module contains comprehensive unit tests for all element condition functions
//! using the new plugin-based architecture.

use condition_core::*;
use condition_core::element_functions::*;
use condition_core::data_accessor::{ElementDataAccessor, ElementFunctionRegistry};
use std::sync::Arc;
use std::time::SystemTime;

// Mock data provider for testing
struct TestElementDataProvider;

#[async_trait::async_trait]
impl ElementDataProvider for TestElementDataProvider {
    async fn get_element_mastery(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        match element_id {
            "fire" => Ok(200.0),
            "water" => Ok(150.0),
            "earth" => Ok(100.0),
            "air" => Ok(50.0),
            _ => Ok(0.0),
        }
    }

    async fn get_element_resistance(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        match element_id {
            "fire" => Ok(0.3),
            "water" => Ok(0.4),
            "earth" => Ok(0.2),
            "air" => Ok(0.1),
            _ => Ok(0.0),
        }
    }

    async fn has_element_affinity(&self, element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match element_id {
            "fire" | "water" => Ok(true),
            "earth" | "air" => Ok(false),
            _ => Ok(false),
        }
    }

    async fn is_element_weakness(&self, element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match element_id {
            "air" => Ok(true),
            _ => Ok(false),
        }
    }

    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String> {
        match (source_element, target_element) {
            ("fire", "water") => Ok("suppress".to_string()),
            ("water", "fire") => Ok("extinguish".to_string()),
            ("fire", "earth") => Ok("enhance".to_string()),
            ("earth", "air") => Ok("neutralize".to_string()),
            _ => Ok("neutral".to_string()),
        }
    }

    async fn list_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["fire".to_string(), "water".to_string(), "earth".to_string(), "air".to_string()])
    }

    // Element interaction functions
    async fn is_element_same_category(&self, element1: &str, element2: &str) -> ConditionResult<bool> {
        Ok(element1 == element2)
    }

    async fn is_element_generating(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        match (source_element, target_element) {
            ("fire", "earth") => Ok(true),
            ("water", "air") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn is_element_overcoming(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        match (source_element, target_element) {
            ("fire", "air") => Ok(true),
            ("water", "fire") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn is_element_neutral(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        match (source_element, target_element) {
            ("earth", "air") => Ok(true),
            ("air", "earth") => Ok(true),
            _ => Ok(false),
        }
    }

    // Element status functions
    async fn has_element_status_effect(&self, element_id: &str, status_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match (element_id, status_id) {
            ("fire", "burning") => Ok(true),
            ("water", "wet") => Ok(true),
            ("earth", "rooted") => Ok(true),
            ("air", "floating") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn get_element_status_effect_count(&self, element_id: &str, status_id: &str, _actor_id: &str) -> ConditionResult<i64> {
        match (element_id, status_id) {
            ("fire", "burning") => Ok(3),
            ("water", "wet") => Ok(2),
            ("earth", "rooted") => Ok(1),
            ("air", "floating") => Ok(4),
            _ => Ok(0),
        }
    }

    async fn is_element_status_effect_active(&self, element_id: &str, status_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match (element_id, status_id) {
            ("fire", "burning") => Ok(true),
            ("water", "wet") => Ok(true),
            ("earth", "rooted") => Ok(false),
            ("air", "floating") => Ok(true),
            _ => Ok(false),
        }
    }

    // Element resource functions
    async fn has_element_resource(&self, element_id: &str, resource_type: &str, _actor_id: &str) -> ConditionResult<bool> {
        match (element_id, resource_type) {
            ("fire", "mana") => Ok(true),
            ("water", "mana") => Ok(true),
            ("earth", "stamina") => Ok(true),
            ("air", "mana") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn get_element_resource_value(&self, element_id: &str, resource_type: &str, _actor_id: &str) -> ConditionResult<f64> {
        match (element_id, resource_type) {
            ("fire", "mana") => Ok(100.0),
            ("water", "mana") => Ok(80.0),
            ("earth", "stamina") => Ok(120.0),
            ("air", "mana") => Ok(60.0),
            _ => Ok(0.0),
        }
    }

    async fn is_element_resource_below_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let value = self.get_element_resource_value(element_id, resource_type, "test").await?;
        Ok(value < threshold)
    }

    async fn is_element_resource_above_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let value = self.get_element_resource_value(element_id, resource_type, "test").await?;
        Ok(value > threshold)
    }

    // Hybrid element functions
    async fn has_hybrid_element(&self, hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match hybrid_id {
            "steam" | "lava" | "mud" => Ok(true),
            _ => Ok(false),
        }
    }

    async fn is_hybrid_element_activated(&self, hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match hybrid_id {
            "steam" => Ok(true),
            "lava" => Ok(false),
            "mud" => Ok(true),
            _ => Ok(false),
        }
    }

    async fn get_hybrid_element_parents(&self, hybrid_id: &str) -> ConditionResult<Vec<String>> {
        match hybrid_id {
            "steam" => Ok(vec!["fire".to_string(), "water".to_string()]),
            "lava" => Ok(vec!["fire".to_string(), "earth".to_string()]),
            "mud" => Ok(vec!["water".to_string(), "earth".to_string()]),
            _ => Ok(vec![]),
        }
    }

    async fn list_hybrid_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["steam".to_string(), "lava".to_string(), "mud".to_string()])
    }

    // Element derived stats functions
    async fn get_element_derived_stat(&self, element_id: &str, stat_name: &str, _actor_id: &str) -> ConditionResult<f64> {
        match (element_id, stat_name) {
            ("fire", "damage") => Ok(150.0),
            ("water", "healing") => Ok(120.0),
            ("earth", "defense") => Ok(180.0),
            ("air", "speed") => Ok(200.0),
            _ => Ok(0.0),
        }
    }

    async fn has_element_derived_stat(&self, element_id: &str, stat_name: &str, _actor_id: &str) -> ConditionResult<bool> {
        match (element_id, stat_name) {
            ("fire", "damage") => Ok(true),
            ("water", "healing") => Ok(true),
            ("earth", "defense") => Ok(true),
            ("air", "speed") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn list_element_derived_stats(&self, element_id: &str) -> ConditionResult<Vec<String>> {
        match element_id {
            "fire" => Ok(vec!["damage".to_string(), "burn_rate".to_string()]),
            "water" => Ok(vec!["healing".to_string(), "flow_rate".to_string()]),
            "earth" => Ok(vec!["defense".to_string(), "stability".to_string()]),
            "air" => Ok(vec!["speed".to_string(), "agility".to_string()]),
            _ => Ok(vec![]),
        }
    }
}

// Helper function to create test context
fn create_test_context() -> ConditionContext {
    ConditionContext {
        target: ActorTarget { id: "test_actor".to_string() },
        world_id: "test_world".to_string(),
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

// Helper function to create test registry
fn create_test_registry() -> ElementFunctionRegistry {
    let provider = Arc::new(TestElementDataProvider);
    let data_accessor = Arc::new(ElementDataAccessor::new(provider));
    create_element_function_registry(data_accessor)
}

#[tokio::test]
async fn test_get_element_mastery() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire mastery (should be > 0)
    let params = vec![ConditionParameter::String("fire".to_string())];
    let result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    assert!(result, "Fire mastery should be > 0");
    
    // Test unknown element (should be 0)
    let params = vec![ConditionParameter::String("unknown".to_string())];
    let result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    assert!(!result, "Unknown element mastery should be 0");
}

#[tokio::test]
async fn test_has_element_affinity() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire affinity (should be true)
    let params = vec![ConditionParameter::String("fire".to_string())];
    let result = registry.execute_function("has_element_affinity", &params, &context).await.unwrap();
    assert!(result, "Fire should have affinity");
    
    // Test earth affinity (should be false)
    let params = vec![ConditionParameter::String("earth".to_string())];
    let result = registry.execute_function("has_element_affinity", &params, &context).await.unwrap();
    assert!(!result, "Earth should not have affinity");
}

#[tokio::test]
async fn test_get_element_resistance() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire resistance (should be > 0)
    let params = vec![ConditionParameter::String("fire".to_string())];
    let result = registry.execute_function("get_element_resistance", &params, &context).await.unwrap();
    assert!(result, "Fire should have resistance > 0");
    
    // Test unknown element (should be 0)
    let params = vec![ConditionParameter::String("unknown".to_string())];
    let result = registry.execute_function("get_element_resistance", &params, &context).await.unwrap();
    assert!(!result, "Unknown element should have no resistance");
}

#[tokio::test]
async fn test_is_element_weakness() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test air weakness (should be true)
    let params = vec![ConditionParameter::String("air".to_string())];
    let result = registry.execute_function("is_element_weakness", &params, &context).await.unwrap();
    assert!(result, "Air should be a weakness");
    
    // Test fire weakness (should be false)
    let params = vec![ConditionParameter::String("fire".to_string())];
    let result = registry.execute_function("is_element_weakness", &params, &context).await.unwrap();
    assert!(!result, "Fire should not be a weakness");
}

#[tokio::test]
async fn test_get_element_interaction() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire vs water (should be non-neutral)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("water".to_string())
    ];
    let result = registry.execute_function("get_element_interaction", &params, &context).await.unwrap();
    assert!(result, "Fire vs Water should have non-neutral interaction");
    
    // Test fire vs fire (should be neutral)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("fire".to_string())
    ];
    let result = registry.execute_function("get_element_interaction", &params, &context).await.unwrap();
    assert!(!result, "Fire vs Fire should be neutral");
}

#[tokio::test]
async fn test_is_element_same_category() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test same elements
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("fire".to_string())
    ];
    let result = registry.execute_function("is_element_same_category", &params, &context).await.unwrap();
    assert!(result, "Fire should be same category as Fire");
    
    // Test different elements
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("water".to_string())
    ];
    let result = registry.execute_function("is_element_same_category", &params, &context).await.unwrap();
    assert!(!result, "Fire should not be same category as Water");
}

#[tokio::test]
async fn test_is_element_generating() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire generates earth
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("earth".to_string())
    ];
    let result = registry.execute_function("is_element_generating", &params, &context).await.unwrap();
    assert!(result, "Fire should generate Earth");
    
    // Test fire generates water (should be false)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("water".to_string())
    ];
    let result = registry.execute_function("is_element_generating", &params, &context).await.unwrap();
    assert!(!result, "Fire should not generate Water");
}

#[tokio::test]
async fn test_is_element_overcoming() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire overcomes air
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("air".to_string())
    ];
    let result = registry.execute_function("is_element_overcoming", &params, &context).await.unwrap();
    assert!(result, "Fire should overcome Air");
    
    // Test fire overcomes earth (should be false)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("earth".to_string())
    ];
    let result = registry.execute_function("is_element_overcoming", &params, &context).await.unwrap();
    assert!(!result, "Fire should not overcome Earth");
}

#[tokio::test]
async fn test_is_element_neutral() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test earth vs air (should be neutral)
    let params = vec![
        ConditionParameter::String("earth".to_string()),
        ConditionParameter::String("air".to_string())
    ];
    let result = registry.execute_function("is_element_neutral", &params, &context).await.unwrap();
    assert!(result, "Earth vs Air should be neutral");
    
    // Test fire vs water (should not be neutral)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("water".to_string())
    ];
    let result = registry.execute_function("is_element_neutral", &params, &context).await.unwrap();
    assert!(!result, "Fire vs Water should not be neutral");
}

#[tokio::test]
async fn test_has_element_status_effect() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire has burning effect
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("burning".to_string())
    ];
    let result = registry.execute_function("has_element_status_effect", &params, &context).await.unwrap();
    assert!(result, "Fire should have burning effect");
    
    // Test fire has wet effect (should be false)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("wet".to_string())
    ];
    let result = registry.execute_function("has_element_status_effect", &params, &context).await.unwrap();
    assert!(!result, "Fire should not have wet effect");
}

#[tokio::test]
async fn test_get_element_status_effect_count() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire burning count (should be > 0)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("burning".to_string())
    ];
    let result = registry.execute_function("get_element_status_effect_count", &params, &context).await.unwrap();
    assert!(result, "Fire burning count should be > 0");
    
    // Test unknown effect count (should be 0)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("unknown".to_string())
    ];
    let result = registry.execute_function("get_element_status_effect_count", &params, &context).await.unwrap();
    assert!(!result, "Unknown effect count should be 0");
}

#[tokio::test]
async fn test_is_element_status_effect_active() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire burning is active
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("burning".to_string())
    ];
    let result = registry.execute_function("is_element_status_effect_active", &params, &context).await.unwrap();
    assert!(result, "Fire burning should be active");
    
    // Test earth rooted is not active
    let params = vec![
        ConditionParameter::String("earth".to_string()),
        ConditionParameter::String("rooted".to_string())
    ];
    let result = registry.execute_function("is_element_status_effect_active", &params, &context).await.unwrap();
    assert!(!result, "Earth rooted should not be active");
}

#[tokio::test]
async fn test_has_element_resource() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire has mana resource
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("mana".to_string())
    ];
    let result = registry.execute_function("has_element_resource", &params, &context).await.unwrap();
    assert!(result, "Fire should have mana resource");
    
    // Test fire has stamina resource (should be false)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("stamina".to_string())
    ];
    let result = registry.execute_function("has_element_resource", &params, &context).await.unwrap();
    assert!(!result, "Fire should not have stamina resource");
}

#[tokio::test]
async fn test_get_element_resource_value() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire mana value (should be > 0)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("mana".to_string())
    ];
    let result = registry.execute_function("get_element_resource_value", &params, &context).await.unwrap();
    assert!(result, "Fire mana value should be > 0");
    
    // Test unknown resource value (should be 0)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("unknown".to_string())
    ];
    let result = registry.execute_function("get_element_resource_value", &params, &context).await.unwrap();
    assert!(!result, "Unknown resource value should be 0");
}

#[tokio::test]
async fn test_is_element_resource_below_threshold() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire mana below 150 (should be true, value is 100)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("mana".to_string()),
        ConditionParameter::Float(150.0)
    ];
    let result = registry.execute_function("is_element_resource_below_threshold", &params, &context).await.unwrap();
    assert!(result, "Fire mana should be below 150");
    
    // Test fire mana below 50 (should be false, value is 100)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("mana".to_string()),
        ConditionParameter::Float(50.0)
    ];
    let result = registry.execute_function("is_element_resource_below_threshold", &params, &context).await.unwrap();
    assert!(!result, "Fire mana should not be below 50");
}

#[tokio::test]
async fn test_is_element_resource_above_threshold() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire mana above 50 (should be true, value is 100)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("mana".to_string()),
        ConditionParameter::Float(50.0)
    ];
    let result = registry.execute_function("is_element_resource_above_threshold", &params, &context).await.unwrap();
    assert!(result, "Fire mana should be above 50");
    
    // Test fire mana above 150 (should be false, value is 100)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("mana".to_string()),
        ConditionParameter::Float(150.0)
    ];
    let result = registry.execute_function("is_element_resource_above_threshold", &params, &context).await.unwrap();
    assert!(!result, "Fire mana should not be above 150");
}

#[tokio::test]
async fn test_has_hybrid_element() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test steam hybrid element
    let params = vec![ConditionParameter::String("steam".to_string())];
    let result = registry.execute_function("has_hybrid_element", &params, &context).await.unwrap();
    assert!(result, "Actor should have steam hybrid element");
    
    // Test unknown hybrid element
    let params = vec![ConditionParameter::String("unknown".to_string())];
    let result = registry.execute_function("has_hybrid_element", &params, &context).await.unwrap();
    assert!(!result, "Actor should not have unknown hybrid element");
}

#[tokio::test]
async fn test_is_hybrid_element_activated() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test steam is activated
    let params = vec![ConditionParameter::String("steam".to_string())];
    let result = registry.execute_function("is_hybrid_element_activated", &params, &context).await.unwrap();
    assert!(result, "Steam should be activated");
    
    // Test lava is not activated
    let params = vec![ConditionParameter::String("lava".to_string())];
    let result = registry.execute_function("is_hybrid_element_activated", &params, &context).await.unwrap();
    assert!(!result, "Lava should not be activated");
}

#[tokio::test]
async fn test_get_hybrid_element_parents() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test steam has parents
    let params = vec![ConditionParameter::String("steam".to_string())];
    let result = registry.execute_function("get_hybrid_element_parents", &params, &context).await.unwrap();
    assert!(result, "Steam should have parent elements");
    
    // Test unknown hybrid has no parents
    let params = vec![ConditionParameter::String("unknown".to_string())];
    let result = registry.execute_function("get_hybrid_element_parents", &params, &context).await.unwrap();
    assert!(!result, "Unknown hybrid should not have parent elements");
}

#[tokio::test]
async fn test_get_element_derived_stat() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test fire damage stat (should be > 0)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("damage".to_string())
    ];
    let result = registry.execute_function("get_element_derived_stat", &params, &context).await.unwrap();
    assert!(result, "Fire damage stat should be > 0");
    
    // Test unknown stat (should be 0)
    let params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("unknown".to_string())
    ];
    let result = registry.execute_function("get_element_derived_stat", &params, &context).await.unwrap();
    assert!(!result, "Unknown stat should be 0");
}

// Test error handling
#[tokio::test]
async fn test_invalid_parameter_count() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test function with no parameters
    let params = vec![];
    let result = registry.execute_function("get_element_mastery", &params, &context).await;
    assert!(result.is_err(), "Should return error for invalid parameter count");
    
    // Test function with wrong parameter count
    let params = vec![ConditionParameter::String("fire".to_string())];
    let result = registry.execute_function("get_element_interaction", &params, &context).await;
    assert!(result.is_err(), "Should return error for wrong parameter count");
}

#[tokio::test]
async fn test_invalid_parameter_type() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test function with wrong parameter type
    let params = vec![ConditionParameter::Integer(123)];
    let result = registry.execute_function("get_element_mastery", &params, &context).await;
    assert!(result.is_err(), "Should return error for invalid parameter type");
}

#[tokio::test]
async fn test_unknown_function() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    // Test unknown function
    let params = vec![ConditionParameter::String("test".to_string())];
    let result = registry.execute_function("unknown_function", &params, &context).await;
    assert!(result.is_err(), "Should return error for unknown function");
}

// Test function registry
#[tokio::test]
async fn test_function_registry_list() {
    let registry = create_test_registry();
    let functions = registry.list_functions();
    
    // Check that all expected functions are registered
    assert!(functions.contains(&"get_element_mastery".to_string()));
    assert!(functions.contains(&"has_element_affinity".to_string()));
    assert!(functions.contains(&"get_element_resistance".to_string()));
    assert!(functions.contains(&"is_element_weakness".to_string()));
    assert!(functions.contains(&"get_element_interaction".to_string()));
    assert!(functions.contains(&"is_element_same_category".to_string()));
    assert!(functions.contains(&"is_element_generating".to_string()));
    assert!(functions.contains(&"is_element_overcoming".to_string()));
    assert!(functions.contains(&"is_element_neutral".to_string()));
    assert!(functions.contains(&"has_element_status_effect".to_string()));
    assert!(functions.contains(&"get_element_status_effect_count".to_string()));
    assert!(functions.contains(&"is_element_status_effect_active".to_string()));
    assert!(functions.contains(&"has_element_resource".to_string()));
    assert!(functions.contains(&"get_element_resource_value".to_string()));
    assert!(functions.contains(&"is_element_resource_below_threshold".to_string()));
    assert!(functions.contains(&"is_element_resource_above_threshold".to_string()));
    assert!(functions.contains(&"has_hybrid_element".to_string()));
    assert!(functions.contains(&"is_hybrid_element_activated".to_string()));
    assert!(functions.contains(&"get_hybrid_element_parents".to_string()));
    assert!(functions.contains(&"get_element_derived_stat".to_string()));
    
    // Check total count
    assert_eq!(functions.len(), 20, "Should have 20 element functions");
}

// Test performance
#[tokio::test]
async fn test_function_performance() {
    let registry = create_test_registry();
    let context = create_test_context();
    
    let start = std::time::Instant::now();
    
    // Run multiple function calls
    for _ in 0..100 {
        let params = vec![ConditionParameter::String("fire".to_string())];
        let _result = registry.execute_function("get_element_mastery", &params, &context).await.unwrap();
    }
    
    let duration = start.elapsed();
    
    // Should complete within reasonable time (100ms for 100 calls)
    assert!(duration.as_millis() < 100, "Function calls should be fast");
}

// Test concurrent execution
#[tokio::test]
async fn test_concurrent_execution() {
    let registry = Arc::new(create_test_registry());
    let context = Arc::new(create_test_context());
    
    let mut handles = vec![];
    
    // Spawn multiple concurrent tasks
    for i in 0..10 {
        let registry = registry.clone();
        let context = context.clone();
        let handle = tokio::spawn(async move {
            let params = vec![ConditionParameter::String(format!("element_{}", i))];
            registry.execute_function("get_element_mastery", &params, &context).await
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.unwrap();
        // Some may succeed, some may fail (unknown elements)
        // The important thing is that they don't panic
        let _ = result;
    }
}
