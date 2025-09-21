//! Add New Function Demo - Demonstrates how easy it is to add new functions
//! 
//! This example shows how to add a new element condition function using the
//! new plugin-based architecture. The key benefits are:
//! 
//! 1. **No Breaking Changes**: Adding new functions doesn't break existing code
//! 2. **No Trait Hell**: Mock implementations don't need to implement new methods
//! 3. **Easy Testing**: Each function can be tested independently
//! 4. **Clean Architecture**: Functions are self-contained and focused

#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use condition_core::*;
use condition_core::element_functions::create_element_function_registry;
use condition_core::data_accessor::{ElementConditionFunction, ElementDataAccessor};
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

    // Only implement the methods that are actually used
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

// ============================================================================
// 🆕 NEW FUNCTION: Check if element is legendary
// ============================================================================
// This is how easy it is to add a new function with the new architecture!

/// Check if an element is legendary (has mastery > 200)
pub struct IsElementLegendaryFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsElementLegendaryFunction {
    fn name(&self) -> &str {
        "is_element_legendary"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.is_empty() {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: 0 });
        }

        let element_id = params[0].as_string()?;
        let mastery = data_accessor.get_element_mastery(element_id, &context.target.id).await?;
        
        // Element is legendary if mastery > 200
        Ok(mastery > 200.0)
    }
}

// ============================================================================
// 🆕 NEW FUNCTION: Check element power level
// ============================================================================
// Another example of how easy it is to add functions!

/// Check if element power level meets threshold
pub struct CheckElementPowerLevelFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for CheckElementPowerLevelFunction {
    fn name(&self) -> &str {
        "check_element_power_level"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let element_id = params[0].as_string()?;
        let threshold = params[1].as_float()?;
        
        let mastery = data_accessor.get_element_mastery(element_id, &context.target.id).await?;
        let resistance = data_accessor.get_element_resistance(element_id, &context.target.id).await?;
        
        // Power level = mastery * (1 + resistance)
        let power_level = mastery * (1.0 + resistance);
        
        Ok(power_level >= threshold)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🆕 Add New Function Demo - Plugin-based Architecture");
    println!("==================================================\n");

    // 1. Create data provider
    let element_provider = Arc::new(SimpleElementDataProvider);
    
    // 2. Create data accessor
    let data_accessor = Arc::new(ElementDataAccessor::new(element_provider));
    
    // 3. Create function registry with existing functions
    let mut function_registry = create_element_function_registry(data_accessor);
    
    // 4. 🆕 ADD NEW FUNCTIONS - This is super easy!
    function_registry.register_function(IsElementLegendaryFunction);
    function_registry.register_function(CheckElementPowerLevelFunction);
    
    // 5. Create test context
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

    println!("📋 Available Functions (including new ones):");
    for function_name in function_registry.list_functions() {
        if function_name.contains("legendary") || function_name.contains("power") {
            println!("   - {} 🆕", function_name);
        } else {
            println!("   - {}", function_name);
        }
    }
    println!();

    // 6. Test existing functions (still work!)
    println!("🧪 Testing Existing Functions:");
    println!("------------------------------");
    
    let fire_mastery_params = vec![ConditionParameter::String("fire".to_string())];
    let result = function_registry.execute_function("get_element_mastery", &fire_mastery_params, &context).await?;
    println!("✅ Fire mastery check: {}", result);

    // 7. Test NEW functions
    println!("\n🆕 Testing NEW Functions:");
    println!("-------------------------");
    
    // Test legendary function
    let legendary_params = vec![ConditionParameter::String("fire".to_string())];
    let result = function_registry.execute_function("is_element_legendary", &legendary_params, &context).await?;
    println!("✅ Is Fire legendary? {}", result);
    
    let legendary_params = vec![ConditionParameter::String("water".to_string())];
    let result = function_registry.execute_function("is_element_legendary", &legendary_params, &context).await?;
    println!("✅ Is Water legendary? {}", result);

    // Test power level function
    let power_params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::Float(100.0)
    ];
    let result = function_registry.execute_function("check_element_power_level", &power_params, &context).await?;
    println!("✅ Fire power level >= 100? {}", result);
    
    let power_params = vec![
        ConditionParameter::String("fire".to_string()),
        ConditionParameter::Float(200.0)
    ];
    let result = function_registry.execute_function("check_element_power_level", &power_params, &context).await?;
    println!("✅ Fire power level >= 200? {}", result);

    println!("\n🎉 Demo completed successfully!");
    println!("\n💡 Key Benefits Demonstrated:");
    println!("   ✅ No Breaking Changes: Existing functions still work");
    println!("   ✅ Easy to Add: New functions added in just a few lines");
    println!("   ✅ No Trait Hell: Mock doesn't need new methods");
    println!("   ✅ Self-Contained: Each function is independent");
    println!("   ✅ Testable: Functions can be tested individually");
    println!("   ✅ Scalable: Can add hundreds of functions easily");

    Ok(())
}
