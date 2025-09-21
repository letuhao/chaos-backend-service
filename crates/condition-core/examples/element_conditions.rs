//! Element Core Condition Functions Examples
//! 
//! This example demonstrates how to use the new element condition functions
//! that integrate with Element Core through Condition Core.

use condition_core::{
    ConditionConfig, ConditionContext, ConditionOperator, ConditionParameter, ConditionValue,
    ConditionResolverTrait, create_function_registry_with_providers, DataProviderRegistry,
    ActorTarget, WeatherType, WorldState,
};
use std::time::SystemTime;

// Mock Element Data Provider for demonstration
struct MockElementDataProvider;

#[async_trait::async_trait]
impl condition_core::ElementDataProvider for MockElementDataProvider {
    async fn get_element_mastery(&self, element_id: &str, actor_id: &str) -> condition_core::ConditionResult<f64> {
        // Mock implementation - in real usage, this would query Element Core
        match (element_id, actor_id) {
            ("fire", "player_1") => Ok(150.0),
            ("water", "player_1") => Ok(80.0),
            ("ice", "player_1") => Ok(120.0),
            _ => Ok(0.0),
        }
    }
    
    async fn get_element_resistance(&self, element_id: &str, actor_id: &str) -> condition_core::ConditionResult<f64> {
        match (element_id, actor_id) {
            ("fire", "player_1") => Ok(60.0),
            ("water", "player_1") => Ok(90.0),
            ("ice", "player_1") => Ok(40.0),
            _ => Ok(0.0),
        }
    }
    
    async fn has_element_affinity(&self, element_id: &str, actor_id: &str) -> condition_core::ConditionResult<bool> {
        match (element_id, actor_id) {
            ("fire", "player_1") => Ok(true),
            ("water", "player_1") => Ok(false),
            ("ice", "player_1") => Ok(true),
            _ => Ok(false),
        }
    }
    
    async fn is_element_weakness(&self, element_id: &str, actor_id: &str) -> condition_core::ConditionResult<bool> {
        match (element_id, actor_id) {
            ("fire", "player_1") => Ok(false),
            ("water", "player_1") => Ok(true),
            ("ice", "player_1") => Ok(false),
            _ => Ok(false),
        }
    }
    
    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> condition_core::ConditionResult<String> {
        match (source_element, target_element) {
            ("fire", "water") => Ok("overcoming".to_string()),
            ("water", "fire") => Ok("generating".to_string()),
            ("fire", "ice") => Ok("overcoming".to_string()),
            ("ice", "fire") => Ok("generating".to_string()),
            _ => Ok("neutral".to_string()),
        }
    }
    
    async fn list_elements(&self) -> condition_core::ConditionResult<Vec<String>> {
        Ok(vec!["fire".to_string(), "water".to_string(), "ice".to_string(), "earth".to_string()])
    }
    
    // Element interaction functions
    async fn is_element_same_category(&self, element1: &str, element2: &str) -> condition_core::ConditionResult<bool> {
        let fire_elements = vec!["fire", "lava", "magma"];
        let water_elements = vec!["water", "ice", "snow"];
        let earth_elements = vec!["earth", "stone", "metal"];
        
        let category1 = if fire_elements.contains(&element1) { "fire" }
            else if water_elements.contains(&element1) { "water" }
            else if earth_elements.contains(&element1) { "earth" }
            else { "unknown" };
            
        let category2 = if fire_elements.contains(&element2) { "fire" }
            else if water_elements.contains(&element2) { "water" }
            else if earth_elements.contains(&element2) { "earth" }
            else { "unknown" };
            
        Ok(category1 == category2)
    }
    
    async fn is_element_generating(&self, source_element: &str, target_element: &str) -> condition_core::ConditionResult<bool> {
        Ok(matches!((source_element, target_element), 
            ("water", "fire") | ("fire", "earth") | ("earth", "metal") | ("metal", "water")))
    }
    
    async fn is_element_overcoming(&self, source_element: &str, target_element: &str) -> condition_core::ConditionResult<bool> {
        Ok(matches!((source_element, target_element), 
            ("fire", "water") | ("water", "earth") | ("earth", "metal") | ("metal", "fire")))
    }
    
    async fn is_element_neutral(&self, source_element: &str, target_element: &str) -> condition_core::ConditionResult<bool> {
        Ok(!self.is_element_generating(source_element, target_element).await? && 
           !self.is_element_overcoming(source_element, target_element).await?)
    }
    
    // Element status functions
    async fn has_element_status_effect(&self, element_id: &str, status_id: &str, actor_id: &str) -> condition_core::ConditionResult<bool> {
        match (element_id, status_id, actor_id) {
            ("fire", "burning", "player_1") => Ok(true),
            ("water", "wet", "player_1") => Ok(false),
            ("ice", "frozen", "player_1") => Ok(true),
            _ => Ok(false),
        }
    }
    
    async fn get_element_status_effect_count(&self, element_id: &str, status_id: &str, actor_id: &str) -> condition_core::ConditionResult<i64> {
        match (element_id, status_id, actor_id) {
            ("fire", "burning", "player_1") => Ok(3),
            ("water", "wet", "player_1") => Ok(0),
            ("ice", "frozen", "player_1") => Ok(1),
            _ => Ok(0),
        }
    }
    
    async fn is_element_status_effect_active(&self, element_id: &str, status_id: &str, actor_id: &str) -> condition_core::ConditionResult<bool> {
        self.has_element_status_effect(element_id, status_id, actor_id).await
    }
    
    // Element resource functions
    async fn has_element_resource(&self, element_id: &str, resource_type: &str, actor_id: &str) -> condition_core::ConditionResult<bool> {
        match (element_id, resource_type, actor_id) {
            ("fire", "mana", "player_1") => Ok(true),
            ("water", "mana", "player_1") => Ok(true),
            ("ice", "mana", "player_1") => Ok(false),
            _ => Ok(false),
        }
    }
    
    async fn get_element_resource_value(&self, element_id: &str, resource_type: &str, actor_id: &str) -> condition_core::ConditionResult<f64> {
        match (element_id, resource_type, actor_id) {
            ("fire", "mana", "player_1") => Ok(100.0),
            ("water", "mana", "player_1") => Ok(80.0),
            ("ice", "mana", "player_1") => Ok(0.0),
            _ => Ok(0.0),
        }
    }
    
    async fn is_element_resource_below_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> condition_core::ConditionResult<bool> {
        let value = self.get_element_resource_value(element_id, resource_type, actor_id).await?;
        Ok(value < threshold)
    }
    
    async fn is_element_resource_above_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> condition_core::ConditionResult<bool> {
        let value = self.get_element_resource_value(element_id, resource_type, actor_id).await?;
        Ok(value > threshold)
    }
    
    // Hybrid element functions
    async fn has_hybrid_element(&self, hybrid_id: &str, actor_id: &str) -> condition_core::ConditionResult<bool> {
        match (hybrid_id, actor_id) {
            ("steam", "player_1") => Ok(true),
            ("mud", "player_1") => Ok(false),
            _ => Ok(false),
        }
    }
    
    async fn is_hybrid_element_activated(&self, hybrid_id: &str, actor_id: &str) -> condition_core::ConditionResult<bool> {
        match (hybrid_id, actor_id) {
            ("steam", "player_1") => Ok(true),
            ("mud", "player_1") => Ok(false),
            _ => Ok(false),
        }
    }
    
    async fn get_hybrid_element_parents(&self, hybrid_id: &str) -> condition_core::ConditionResult<Vec<String>> {
        match hybrid_id {
            "steam" => Ok(vec!["fire".to_string(), "water".to_string()]),
            "mud" => Ok(vec!["earth".to_string(), "water".to_string()]),
            "lava" => Ok(vec!["fire".to_string(), "earth".to_string()]),
            _ => Ok(vec![]),
        }
    }
    
    async fn list_hybrid_elements(&self) -> condition_core::ConditionResult<Vec<String>> {
        Ok(vec!["steam".to_string(), "mud".to_string(), "lava".to_string()])
    }
    
    // Element derived stats functions
    async fn get_element_derived_stat(&self, element_id: &str, stat_name: &str, actor_id: &str) -> condition_core::ConditionResult<f64> {
        match (element_id, stat_name, actor_id) {
            ("fire", "power", "player_1") => Ok(200.0),
            ("fire", "defense", "player_1") => Ok(50.0),
            ("water", "power", "player_1") => Ok(120.0),
            ("water", "defense", "player_1") => Ok(180.0),
            _ => Ok(0.0),
        }
    }
    
    async fn has_element_derived_stat(&self, element_id: &str, stat_name: &str, actor_id: &str) -> condition_core::ConditionResult<bool> {
        let value = self.get_element_derived_stat(element_id, stat_name, actor_id).await?;
        Ok(value > 0.0)
    }
    
    async fn list_element_derived_stats(&self, element_id: &str) -> condition_core::ConditionResult<Vec<String>> {
        match element_id {
            "fire" => Ok(vec!["power".to_string(), "defense".to_string(), "crit_rate".to_string()]),
            "water" => Ok(vec!["power".to_string(), "defense".to_string(), "healing".to_string()]),
            _ => Ok(vec![]),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Element Core Condition Functions Examples");
    println!("=============================================\n");
    
    // Create data provider registry
    let mut data_registry = DataProviderRegistry::new();
    data_registry.register_element_provider(Box::new(MockElementDataProvider));
    
    // Create condition resolver
    let resolver = condition_core::ConditionResolver::new(data_registry);
    
    // Create test context
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
    
    // Example 1: Check if player has fire mastery
    println!("1. Checking Fire Mastery:");
    let fire_mastery_condition = ConditionConfig {
        condition_id: "fire_mastery_check".to_string(),
        function_name: "get_element_mastery".to_string(),
        operator: ConditionOperator::GreaterThanOrEqual,
        value: ConditionValue::Float(100.0),
        parameters: vec![ConditionParameter::String("fire".to_string())],
    };
    
    let result = resolver.resolve_condition(&fire_mastery_condition, &context).await?;
    println!("   Fire mastery >= 100: {}", result);
    
    // Example 2: Check if player has water affinity
    println!("\n2. Checking Water Affinity:");
    let water_affinity_condition = ConditionConfig {
        condition_id: "water_affinity_check".to_string(),
        function_name: "has_element_affinity".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("water".to_string())],
    };
    
    let result = resolver.resolve_condition(&water_affinity_condition, &context).await?;
    println!("   Has water affinity: {}", result);
    
    // Example 3: Check element interaction (fire vs water)
    println!("\n3. Checking Element Interaction (Fire vs Water):");
    let fire_water_interaction = ConditionConfig {
        condition_id: "fire_water_interaction".to_string(),
        function_name: "is_element_overcoming".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![
            ConditionParameter::String("fire".to_string()),
            ConditionParameter::String("water".to_string()),
        ],
    };
    
    let result = resolver.resolve_condition(&fire_water_interaction, &context).await?;
    println!("   Fire overcomes water: {}", result);
    
    // Example 4: Check if player has fire burning status effect
    println!("\n4. Checking Fire Burning Status Effect:");
    let fire_burning_condition = ConditionConfig {
        condition_id: "fire_burning_check".to_string(),
        function_name: "has_element_status_effect".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![
            ConditionParameter::String("fire".to_string()),
            ConditionParameter::String("burning".to_string()),
        ],
    };
    
    let result = resolver.resolve_condition(&fire_burning_condition, &context).await?;
    println!("   Has fire burning effect: {}", result);
    
    // Example 5: Check if fire mana resource is above threshold
    println!("\n5. Checking Fire Mana Resource:");
    let fire_mana_condition = ConditionConfig {
        condition_id: "fire_mana_check".to_string(),
        function_name: "is_element_resource_above_threshold".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![
            ConditionParameter::String("fire".to_string()),
            ConditionParameter::String("mana".to_string()),
            ConditionParameter::Float(50.0),
        ],
    };
    
    let result = resolver.resolve_condition(&fire_mana_condition, &context).await?;
    println!("   Fire mana > 50: {}", result);
    
    // Example 6: Check if player has hybrid element (steam)
    println!("\n6. Checking Hybrid Element (Steam):");
    let steam_hybrid_condition = ConditionConfig {
        condition_id: "steam_hybrid_check".to_string(),
        function_name: "has_hybrid_element".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("steam".to_string())],
    };
    
    let result = resolver.resolve_condition(&steam_hybrid_condition, &context).await?;
    println!("   Has steam hybrid element: {}", result);
    
    // Example 7: Check fire derived stat (power)
    println!("\n7. Checking Fire Power Derived Stat:");
    let fire_power_condition = ConditionConfig {
        condition_id: "fire_power_check".to_string(),
        function_name: "get_element_derived_stat".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(150.0),
        parameters: vec![
            ConditionParameter::String("fire".to_string()),
            ConditionParameter::String("power".to_string()),
        ],
    };
    
    let result = resolver.resolve_condition(&fire_power_condition, &context).await?;
    println!("   Fire power > 150: {}", result);
    
    // Example 8: Complex condition chain (fire mastery AND fire affinity AND fire resource)
    println!("\n8. Complex Condition Chain:");
    let complex_condition = ConditionConfig {
        condition_id: "complex_fire_condition".to_string(),
        function_name: "get_element_mastery".to_string(),
        operator: ConditionOperator::GreaterThanOrEqual,
        value: ConditionValue::Float(100.0),
        parameters: vec![ConditionParameter::String("fire".to_string())],
    };
    
    let fire_affinity_condition = ConditionConfig {
        condition_id: "fire_affinity_condition".to_string(),
        function_name: "has_element_affinity".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("fire".to_string())],
    };
    
    let fire_resource_condition = ConditionConfig {
        condition_id: "fire_resource_condition".to_string(),
        function_name: "has_element_resource".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![
            ConditionParameter::String("fire".to_string()),
            ConditionParameter::String("mana".to_string()),
        ],
    };
    
    let mastery_result = resolver.resolve_condition(&complex_condition, &context).await?;
    let affinity_result = resolver.resolve_condition(&fire_affinity_condition, &context).await?;
    let resource_result = resolver.resolve_condition(&fire_resource_condition, &context).await?;
    
    let complex_result = mastery_result && affinity_result && resource_result;
    println!("   Fire mastery >= 100: {}", mastery_result);
    println!("   Has fire affinity: {}", affinity_result);
    println!("   Has fire mana resource: {}", resource_result);
    println!("   Complex condition (ALL): {}", complex_result);
    
    println!("\n✅ All element condition examples completed successfully!");
    
    Ok(())
}
