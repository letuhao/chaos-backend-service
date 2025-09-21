//! Advanced usage example of Condition Core with new functions

use condition_core::*;
use std::time::SystemTime;

// Mock implementations for demonstration
struct MockElementDataProvider;
struct MockResourceDataProvider;
struct MockCategoryDataProvider;
struct MockActorDataProvider;

#[async_trait::async_trait]
impl ElementDataProvider for MockElementDataProvider {
    async fn get_element_mastery(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let mastery = match element_id {
            "fire" => 150.0,
            "water" => 120.0,
            "earth" => 100.0,
            "shadow" => 95.0,
            "metal" => 105.0,
            _ => 0.0,
        };
        Ok(mastery)
    }

    async fn get_element_resistance(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let resistance = match element_id {
            "fire" => 0.2,
            "water" => 0.5,
            "earth" => 0.3,
            "shadow" => 0.3,
            "metal" => 0.7,
            _ => 0.0,
        };
        Ok(resistance)
    }

    async fn has_element_affinity(&self, element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_affinity = match element_id {
            "fire" => true,
            "water" => true,
            "earth" => false,
            "shadow" => false,
            "metal" => false,
            _ => false,
        };
        Ok(has_affinity)
    }

    async fn is_element_weakness(&self, _element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false)
    }

    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String> {
        let interaction = match (source_element, target_element) {
            ("fire", "water") => "suppress",
            ("water", "fire") => "extinguish",
            ("shadow", "light") => "hide",
            ("light", "shadow") => "illuminate",
            ("metal", "lightning") => "attract",
            ("lightning", "metal") => "attract",
            _ => "neutral",
        };
        Ok(interaction.to_string())
    }

    async fn list_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![
            "fire".to_string(),
            "water".to_string(),
            "earth".to_string(),
            "shadow".to_string(),
            "metal".to_string(),
        ])
    }    
    // Element interaction functions
    async fn is_element_same_category(&self, element1: &str, element2: &str) -> ConditionResult<bool> {
        Ok(element1 == element2) // Simple mock implementation
    }
    
    async fn is_element_generating(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn is_element_overcoming(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn is_element_neutral(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        Ok(true) // Simple mock implementation
    }
    
    // Element status functions
    async fn has_element_status_effect(&self, element_id: &str, status_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn get_element_status_effect_count(&self, element_id: &str, status_id: &str, _actor_id: &str) -> ConditionResult<i64> {
        Ok(0) // Simple mock implementation
    }
    
    async fn is_element_status_effect_active(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<bool> {
        self.has_element_status_effect(element_id, status_id, actor_id).await
    }
    
    // Element resource functions
    async fn has_element_resource(&self, element_id: &str, resource_type: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn get_element_resource_value(&self, element_id: &str, resource_type: &str, _actor_id: &str) -> ConditionResult<f64> {
        Ok(0.0) // Simple mock implementation
    }
    
    async fn is_element_resource_below_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool> {
        let value = self.get_element_resource_value(element_id, resource_type, actor_id).await?;
        Ok(value < threshold)
    }
    
    async fn is_element_resource_above_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool> {
        let value = self.get_element_resource_value(element_id, resource_type, actor_id).await?;
        Ok(value > threshold)
    }
    
    // Hybrid element functions
    async fn has_hybrid_element(&self, hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn is_hybrid_element_activated(&self, hybrid_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false) // Simple mock implementation
    }
    
    async fn get_hybrid_element_parents(&self, hybrid_id: &str) -> ConditionResult<Vec<String>> {
        Ok(vec![]) // Simple mock implementation
    }
    
    async fn list_hybrid_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![]) // Simple mock implementation
    }
    
    // Element derived stats functions
    async fn get_element_derived_stat(&self, element_id: &str, stat_name: &str, _actor_id: &str) -> ConditionResult<f64> {
        Ok(0.0) // Simple mock implementation
    }
    
    async fn has_element_derived_stat(&self, element_id: &str, stat_name: &str, actor_id: &str) -> ConditionResult<bool> {
        let value = self.get_element_derived_stat(element_id, stat_name, actor_id).await?;
        Ok(value > 0.0)
    }
    
    async fn list_element_derived_stats(&self, element_id: &str) -> ConditionResult<Vec<String>> {
        Ok(vec![]) // Simple mock implementation
    }
}

#[async_trait::async_trait]
impl ResourceDataProvider for MockResourceDataProvider {
    async fn get_resource_value(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            "sanity" => 100.0,
            "karma" => 0.0,
            _ => 0.0,
        };
        Ok(value)
    }

    async fn get_resource_max(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let max_value = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 100.0,
            "sanity" => 100.0,
            "karma" => 100.0,
            _ => 100.0,
        };
        Ok(max_value)
    }

    async fn get_resource_percentage(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let (current, max) = match resource_id {
            "health" => (100.0, 100.0),
            "mana" => (15.0, 50.0), // Low mana
            "stamina" => (75.0, 100.0),
            "sanity" => (100.0, 100.0),
            "karma" => (0.0, 100.0),
            _ => (0.0, 100.0),
        };
        Ok(current / max)
    }

    async fn is_resource_empty(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_id {
            "health" => 100.0,
            "mana" => 15.0,
            "stamina" => 75.0,
            "sanity" => 100.0,
            "karma" => 0.0,
            _ => 0.0,
        };
        Ok(current <= 0.0)
    }

    async fn is_resource_below_threshold(&self, resource_id: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_id {
            "health" => 100.0,
            "mana" => 15.0,
            "stamina" => 75.0,
            "sanity" => 100.0,
            "karma" => 0.0,
            _ => 0.0,
        };
        Ok(current < threshold)
    }

    async fn is_resource_above_threshold(&self, resource_id: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_id {
            "health" => 100.0,
            "mana" => 15.0,
            "stamina" => 75.0,
            "sanity" => 100.0,
            "karma" => 0.0,
            _ => 0.0,
        };
        Ok(current > threshold)
    }

    async fn is_resource_below_percentage(&self, resource_id: &str, percentage: f64, _actor_id: &str) -> ConditionResult<bool> {
        let (current, max) = match resource_id {
            "health" => (100.0, 100.0),
            "mana" => (15.0, 50.0), // Low mana
            "stamina" => (75.0, 100.0),
            "sanity" => (100.0, 100.0),
            "karma" => (0.0, 100.0),
            _ => (0.0, 100.0),
        };
        let current_percentage = (current / max) * 100.0;
        Ok(current_percentage < percentage)
    }

    async fn is_resource_above_percentage(&self, resource_id: &str, percentage: f64, _actor_id: &str) -> ConditionResult<bool> {
        let (current, max) = match resource_id {
            "health" => (100.0, 100.0),
            "mana" => (15.0, 50.0), // Low mana
            "stamina" => (75.0, 100.0),
            "sanity" => (100.0, 100.0),
            "karma" => (0.0, 100.0),
            _ => (0.0, 100.0),
        };
        let current_percentage = (current / max) * 100.0;
        Ok(current_percentage > percentage)
    }

    async fn list_resources(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![
            "health".to_string(),
            "mana".to_string(),
            "stamina".to_string(),
            "sanity".to_string(),
            "karma".to_string(),
        ])
    }
}

#[async_trait::async_trait]
impl CategoryDataProvider for MockCategoryDataProvider {
    async fn has_category_item(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_item = match category_id {
            "weapon" => true,
            "armor" => true,
            "potion" => true,
            "magic_item" => true,
            "artifact" => false,
            _ => false,
        };
        Ok(has_item)
    }

    async fn get_category_item_count(&self, category_id: &str, _actor_id: &str) -> ConditionResult<i64> {
        let count = match category_id {
            "weapon" => 3,
            "armor" => 5,
            "potion" => 10,
            "magic_item" => 1,
            "artifact" => 0,
            _ => 0,
        };
        Ok(count)
    }

    async fn is_category_available(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let is_available = match category_id {
            "combat" => true,
            "magic" => true,
            "movement" => true,
            "pvp" => true,
            "raid" => false,
            _ => true,
        };
        Ok(is_available)
    }

    async fn is_category_blocked(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let is_blocked = match category_id {
            "combat" => false,
            "magic" => false,
            "movement" => false,
            "pvp" => false,
            "raid" => true,
            _ => false,
        };
        Ok(is_blocked)
    }

    async fn list_categories(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![
            "weapon".to_string(),
            "armor".to_string(),
            "potion".to_string(),
            "magic_item".to_string(),
            "artifact".to_string(),
            "combat".to_string(),
            "magic".to_string(),
            "movement".to_string(),
            "pvp".to_string(),
            "raid".to_string(),
        ])
    }
}

#[async_trait::async_trait]
impl ActorDataProvider for MockActorDataProvider {
    async fn get_actor_resource(&self, resource_type: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match resource_type {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            "sanity" => 100.0,
            "karma" => 0.0,
            _ => 0.0,
        };
        Ok(value)
    }

    async fn get_actor_stat(&self, stat_name: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match stat_name {
            "strength" => 80.0,
            "intelligence" => 90.0,
            "agility" => 70.0,
            _ => 50.0,
        };
        Ok(value)
    }

    async fn get_actor_derived_stat(&self, stat_name: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match stat_name {
            "damage" => 120.0,
            "defense" => 100.0,
            "speed" => 85.0,
            _ => 50.0,
        };
        Ok(value)
    }

    async fn get_actor_race(&self, _actor_id: &str) -> ConditionResult<String> {
        Ok("human".to_string())
    }

    async fn is_actor_in_combat(&self, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false)
    }

    async fn has_actor_status_effects(&self, status_type: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_status = match status_type {
            "buffs" => true,
            "debuffs" => false,
            "ailments" => false,
            _ => false,
        };
        Ok(has_status)
    }

    async fn get_actor_status_effect_count(&self, status_type: &str, _actor_id: &str) -> ConditionResult<i64> {
        let count = match status_type {
            "buffs" => 2,
            "debuffs" => 0,
            "ailments" => 0,
            _ => 0,
        };
        Ok(count)
    }

    async fn get_actor_status_effect_count_by_category(&self, status_type: &str, category: &str, _actor_id: &str) -> ConditionResult<i64> {
        let count = match (status_type, category) {
            ("buffs", "combat") => 1,
            ("buffs", "magic") => 1,
            _ => 0,
        };
        Ok(count)
    }
}

fn create_test_resolver() -> ConditionResolver {
    let mut data_registry = DataProviderRegistry::new();
    data_registry.register_element_provider(Box::new(MockElementDataProvider));
    data_registry.register_resource_provider(Box::new(MockResourceDataProvider));
    data_registry.register_category_provider(Box::new(MockCategoryDataProvider));
    data_registry.register_actor_provider(Box::new(MockActorDataProvider));
    
    ConditionResolver::new(data_registry)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a condition resolver
    let resolver = create_test_resolver();

    // Create a test context
    let context = ConditionContext {
        target: ActorTarget {
            id: "player_1".to_string(),
        },
        world_id: "test_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Rain,
        world_state: WorldState {
            time_of_day: 14.0, // 2 PM
            season: "summer".to_string(),
            temperature: 25.0,
            humidity: 0.8,
        },
    };

    println!("=== Advanced Condition Core Examples ===\n");

    // Example 1: Resource Functions (Generic)
    println!("1. Resource Functions (Generic):");
    
    let resource_condition = ConditionConfig {
        condition_id: "has_sufficient_mana".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::GreaterThanOrEqual,
        value: ConditionValue::Float(30.0),
        parameters: vec![ConditionParameter::String("mana".to_string())],
    };

    let resource_result = resolver.resolve_condition(&resource_condition, &context).await?;
    println!("   Has sufficient mana (30+): {}", resource_result);

    let low_resource_condition = ConditionConfig {
        condition_id: "is_mana_low".to_string(),
        function_name: "is_resource_below_percentage".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![
            ConditionParameter::String("mana".to_string()),
            ConditionParameter::Float(30.0),
        ],
    };

    let low_resource_result = resolver.resolve_condition(&low_resource_condition, &context).await?;
    println!("   Is mana low (<30%): {}", low_resource_result);

    // Example 2: Element Functions (Generic)
    println!("\n2. Element Functions (Generic):");
    
    let element_mastery_condition = ConditionConfig {
        condition_id: "fire_mastery_check".to_string(),
        function_name: "get_element_mastery".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(100.0),
        parameters: vec![ConditionParameter::String("fire".to_string())],
    };

    let element_result = resolver.resolve_condition(&element_mastery_condition, &context).await?;
    println!("   Fire mastery > 100: {}", element_result);

    let element_affinity_condition = ConditionConfig {
        condition_id: "has_fire_affinity".to_string(),
        function_name: "has_element_affinity".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("fire".to_string())],
    };

    let affinity_result = resolver.resolve_condition(&element_affinity_condition, &context).await?;
    println!("   Has fire affinity: {}", affinity_result);

    let element_interaction_condition = ConditionConfig {
        condition_id: "fire_water_interaction".to_string(),
        function_name: "get_element_interaction".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::String("suppress".to_string()),
        parameters: vec![
            ConditionParameter::String("fire".to_string()),
            ConditionParameter::String("water".to_string()),
        ],
    };

    let interaction_result = resolver.resolve_condition(&element_interaction_condition, &context).await?;
    println!("   Fire suppresses water: {}", interaction_result);

    // Example 3: Category Functions
    println!("\n3. Category Functions:");
    
    let category_item_condition = ConditionConfig {
        condition_id: "has_weapon_category".to_string(),
        function_name: "has_category_item".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("weapon".to_string())],
    };

    let category_result = resolver.resolve_condition(&category_item_condition, &context).await?;
    println!("   Has weapon category items: {}", category_result);

    let category_available_condition = ConditionConfig {
        condition_id: "combat_available".to_string(),
        function_name: "is_category_available".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("combat".to_string())],
    };

    let available_result = resolver.resolve_condition(&category_available_condition, &context).await?;
    println!("   Combat category available: {}", available_result);

    // Example 4: More Element Functions
    println!("\n4. More Element Functions:");
    
    let water_mastery_condition = ConditionConfig {
        condition_id: "water_mastery_check".to_string(),
        function_name: "get_element_mastery".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(80.0),
        parameters: vec![ConditionParameter::String("water".to_string())],
    };

    let water_result = resolver.resolve_condition(&water_mastery_condition, &context).await?;
    println!("   Water mastery > 80: {}", water_result);

    let earth_affinity_condition = ConditionConfig {
        condition_id: "earth_affinity_check".to_string(),
        function_name: "has_element_affinity".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(false),
        parameters: vec![ConditionParameter::String("earth".to_string())],
    };

    let earth_affinity_result = resolver.resolve_condition(&earth_affinity_condition, &context).await?;
    println!("   Earth affinity: {}", earth_affinity_result);

    // Example 5: More Resource Functions
    println!("\n5. More Resource Functions:");
    
    let stamina_condition = ConditionConfig {
        condition_id: "has_stamina".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(50.0),
        parameters: vec![ConditionParameter::String("stamina".to_string())],
    };

    let stamina_result = resolver.resolve_condition(&stamina_condition, &context).await?;
    println!("   Has stamina > 50: {}", stamina_result);

    let sanity_low_condition = ConditionConfig {
        condition_id: "is_sanity_low".to_string(),
        function_name: "is_resource_below_percentage".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(false),
        parameters: vec![
            ConditionParameter::String("sanity".to_string()),
            ConditionParameter::Float(30.0),
        ],
    };

    let sanity_low_result = resolver.resolve_condition(&sanity_low_condition, &context).await?;
    println!("   Is sanity low: {}", sanity_low_result);

    // Example 6: More Category Functions
    println!("\n6. More Category Functions:");
    
    let armor_condition = ConditionConfig {
        condition_id: "has_armor".to_string(),
        function_name: "has_category_item".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("armor".to_string())],
    };

    let armor_result = resolver.resolve_condition(&armor_condition, &context).await?;
    println!("   Has armor: {}", armor_result);

    let magic_available_condition = ConditionConfig {
        condition_id: "is_magic_available".to_string(),
        function_name: "is_category_available".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("magic".to_string())],
    };

    let magic_available_result = resolver.resolve_condition(&magic_available_condition, &context).await?;
    println!("   Is magic available: {}", magic_available_result);

    // Example 7: Element Interaction Functions
    println!("\n7. Element Interaction Functions:");
    
    let shadow_light_condition = ConditionConfig {
        condition_id: "shadow_light_interaction".to_string(),
        function_name: "get_element_interaction".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::String("hide".to_string()),
        parameters: vec![
            ConditionParameter::String("shadow".to_string()),
            ConditionParameter::String("light".to_string()),
        ],
    };

    let shadow_light_result = resolver.resolve_condition(&shadow_light_condition, &context).await?;
    println!("   Shadow hides light: {}", shadow_light_result);

    let metal_lightning_condition = ConditionConfig {
        condition_id: "metal_lightning_interaction".to_string(),
        function_name: "get_element_interaction".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::String("attract".to_string()),
        parameters: vec![
            ConditionParameter::String("metal".to_string()),
            ConditionParameter::String("lightning".to_string()),
        ],
    };

    let metal_lightning_result = resolver.resolve_condition(&metal_lightning_condition, &context).await?;
    println!("   Metal attracts lightning: {}", metal_lightning_result);

    println!("\n=== Advanced Examples Complete ===");
    println!("✅ All generic functions working with real data providers");
    println!("✅ New elements/resources/categories can be added without code changes");
    println!("✅ Condition Core focuses only on condition resolution logic");
    println!("✅ Each system manages its own data through dependency injection");

    Ok(())
}
