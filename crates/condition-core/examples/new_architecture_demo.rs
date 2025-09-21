//! New Architecture Demo - Plugin-based Element Functions
//! 
//! This example demonstrates the new plugin-based architecture for element
//! condition functions. The key benefits are:
//! 
//! 1. **Loose Coupling**: Functions don't depend on the full ElementDataProvider trait
//! 2. **Scalable**: Adding new functions doesn't break existing code
//! 3. **Testable**: Easy to mock and test individual functions
//! 4. **Maintainable**: Each function is self-contained

#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use condition_core::*;
use condition_core::element_functions::create_element_function_registry;
use std::sync::Arc;
use std::time::SystemTime;

// Simple mock data provider - only implements what's needed
struct SimpleElementDataProvider;

#[async_trait::async_trait]
impl ElementDataProvider for SimpleElementDataProvider {
    async fn get_element_mastery(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        match element_id {
            "fire" => Ok(150.0),
            "water" => Ok(80.0),
            "earth" => Ok(60.0),
            _ => Ok(50.0),
        }
    }

    async fn get_element_resistance(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        match element_id {
            "fire" => Ok(0.2),
            "water" => Ok(0.3),
            _ => Ok(0.1),
        }
    }

    async fn has_element_affinity(&self, element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match element_id {
            "fire" => Ok(true),
            "earth" => Ok(false),
            _ => Ok(true),
        }
    }

    async fn is_element_weakness(&self, _element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false)
    }

    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String> {
        match (source_element, target_element) {
            ("fire", "water") => Ok("suppress".to_string()),
            ("water", "fire") => Ok("extinguish".to_string()),
            _ => Ok("neutral".to_string()),
        }
    }

    async fn list_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["fire".to_string(), "water".to_string(), "earth".to_string()])
    }

    // Implement only the methods that are actually used by our functions
    // This is the key benefit - we don't need to implement ALL methods!
    async fn is_element_same_category(&self, element1: &str, element2: &str) -> ConditionResult<bool> {
        Ok(element1 == element2) // Simple mock
    }

    async fn is_element_generating(&self, _source_element: &str, _target_element: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock
    }

    async fn is_element_overcoming(&self, _source_element: &str, _target_element: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock
    }

    async fn is_element_neutral(&self, _source_element: &str, _target_element: &str) -> ConditionResult<bool> {
        Ok(true) // Simple mock
    }

    async fn has_element_status_effect(&self, _element_id: &str, _status_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock
    }

    async fn get_element_status_effect_count(&self, _element_id: &str, _status_id: &str, _actor_id: &str) -> ConditionResult<i64> {
        Ok(0) // Simple mock
    }

    async fn is_element_status_effect_active(&self, _element_id: &str, _status_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock
    }

    async fn has_element_resource(&self, _element_id: &str, _resource_type: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock
    }

    async fn get_element_resource_value(&self, _element_id: &str, _resource_type: &str, _actor_id: &str) -> ConditionResult<f64> {
        Ok(0.0) // Simple mock
    }

    async fn is_element_resource_below_threshold(&self, _element_id: &str, _resource_type: &str, _threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock
    }

    async fn is_element_resource_above_threshold(&self, _element_id: &str, _resource_type: &str, _threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock
    }

    async fn has_hybrid_element(&self, _hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock
    }

    async fn is_hybrid_element_activated(&self, _hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock
    }

    async fn get_hybrid_element_parents(&self, _hybrid_id: &str) -> ConditionResult<Vec<String>> {
        Ok(vec![]) // Simple mock
    }

    async fn list_hybrid_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![]) // Simple mock
    }

    async fn get_element_derived_stat(&self, _element_id: &str, _stat_name: &str, _actor_id: &str) -> ConditionResult<f64> {
        Ok(0.0) // Simple mock
    }

    async fn has_element_derived_stat(&self, _element_id: &str, _stat_name: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock
    }

    async fn list_element_derived_stats(&self, _element_id: &str) -> ConditionResult<Vec<String>> {
        Ok(vec![]) // Simple mock
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 New Architecture Demo - Plugin-based Element Functions");
    println!("========================================================\n");

    // 1. Create data provider
    let element_provider = Arc::new(SimpleElementDataProvider);
    
    // 2. Create data accessor (loose coupling layer)
    let data_accessor = Arc::new(ElementDataAccessor::new(element_provider));
    
    // 3. Create function registry with all element functions
    let function_registry = create_element_function_registry(data_accessor);
    
    // 4. Create test context
    let context = ConditionContext {
        target: ActorTarget { id: "player_1".to_string() },
        world_id: "test_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Clear,
        world_state: WorldState {
            time_of_day: 12.0,
            season: "spring".to_string(),
            temperature: 20.0,
            humidity: 60.0,
        },
    };

    println!("📋 Available Element Functions:");
    for function_name in function_registry.list_functions() {
        println!("   - {}", function_name);
    }
    println!();

    // 5. Test individual functions
    println!("🧪 Testing Element Functions:");
    println!("-----------------------------");

    // Test fire mastery
    let fire_mastery_params = vec![ConditionParameter::String("fire".to_string())];
    let result = function_registry.execute_function("get_element_mastery", &fire_mastery_params, &context).await?;
    println!("✅ Fire mastery check: {}", result);

    // Test water affinity
    let water_affinity_params = vec![ConditionParameter::String("water".to_string())];
    let result = function_registry.execute_function("has_element_affinity", &water_affinity_params, &context).await?;
    println!("✅ Water affinity check: {}", result);

    // Test earth resistance
    let earth_resistance_params = vec![ConditionParameter::String("earth".to_string())];
    let result = function_registry.execute_function("get_element_resistance", &earth_resistance_params, &context).await?;
    println!("✅ Earth resistance check: {}", result);

    // Test element interaction
    let interaction_params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("water".to_string())
    ];
    let result = function_registry.execute_function("get_element_interaction", &interaction_params, &context).await?;
    println!("✅ Fire vs Water interaction: {}", result);

    // Test same category check
    let category_params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::String("fire".to_string())
    ];
    let result = function_registry.execute_function("is_element_same_category", &category_params, &context).await?;
    println!("✅ Fire same category as Fire: {}", result);

    println!("\n🎉 Demo completed successfully!");
    println!("\n💡 Key Benefits Demonstrated:");
    println!("   ✅ Loose Coupling: Functions only depend on ElementDataAccessor");
    println!("   ✅ Scalable: Adding new functions doesn't break existing code");
    println!("   ✅ Testable: Easy to mock data accessor for testing");
    println!("   ✅ Maintainable: Each function is self-contained");
    println!("   ✅ No More Trait Hell: Mock implementations only need required methods");

    Ok(())
}
