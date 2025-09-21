//! Example of loading conditions from YAML configuration

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
            _ => 0.0,
        };
        Ok(mastery)
    }

    async fn get_element_resistance(&self, element_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let resistance = match element_id {
            "fire" => 0.2,
            "water" => 0.5,
            "earth" => 0.3,
            _ => 0.0,
        };
        Ok(resistance)
    }

    async fn has_element_affinity(&self, element_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_affinity = match element_id {
            "fire" => true,
            "water" => true,
            "earth" => false,
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
            _ => "neutral",
        };
        Ok(interaction.to_string())
    }

    async fn list_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["fire".to_string(), "water".to_string(), "earth".to_string()])
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
            _ => 0.0,
        };
        Ok(value)
    }

    async fn get_resource_max(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let max_value = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 100.0,
            _ => 100.0,
        };
        Ok(max_value)
    }

    async fn get_resource_percentage(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        let (current, max) = match resource_id {
            "health" => (100.0, 100.0),
            "mana" => (15.0, 50.0), // Low mana
            "stamina" => (75.0, 100.0),
            _ => (0.0, 100.0),
        };
        Ok(current / max)
    }

    async fn is_resource_empty(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_id {
            "health" => 100.0,
            "mana" => 15.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(current <= 0.0)
    }

    async fn is_resource_below_threshold(&self, resource_id: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_id {
            "health" => 100.0,
            "mana" => 15.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(current < threshold)
    }

    async fn is_resource_above_threshold(&self, resource_id: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_id {
            "health" => 100.0,
            "mana" => 15.0,
            "stamina" => 75.0,
            _ => 0.0,
        };
        Ok(current > threshold)
    }

    async fn is_resource_below_percentage(&self, resource_id: &str, percentage: f64, _actor_id: &str) -> ConditionResult<bool> {
        let (current, max) = match resource_id {
            "health" => (100.0, 100.0),
            "mana" => (15.0, 50.0), // Low mana
            "stamina" => (75.0, 100.0),
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
            _ => (0.0, 100.0),
        };
        let current_percentage = (current / max) * 100.0;
        Ok(current_percentage > percentage)
    }

    async fn list_resources(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["health".to_string(), "mana".to_string(), "stamina".to_string()])
    }
}

#[async_trait::async_trait]
impl CategoryDataProvider for MockCategoryDataProvider {
    async fn has_category_item(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        let has_item = match category_id {
            "weapon" => true,
            "armor" => true,
            "potion" => true,
            _ => false,
        };
        Ok(has_item)
    }

    async fn get_category_item_count(&self, category_id: &str, _actor_id: &str) -> ConditionResult<i64> {
        let count = match category_id {
            "weapon" => 3,
            "armor" => 5,
            "potion" => 10,
            _ => 0,
        };
        Ok(count)
    }

    async fn is_category_available(&self, _category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(true)
    }

    async fn is_category_blocked(&self, _category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        Ok(false)
    }

    async fn list_categories(&self) -> ConditionResult<Vec<String>> {
        Ok(vec!["weapon".to_string(), "armor".to_string(), "potion".to_string()])
    }
}

#[async_trait::async_trait]
impl ActorDataProvider for MockActorDataProvider {
    async fn get_actor_resource(&self, resource_type: &str, _actor_id: &str) -> ConditionResult<f64> {
        let value = match resource_type {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
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

    // Example 1: Load single condition from YAML string
    let yaml_config = r#"
condition_id: "check_health"
function_name: "get_actor_resource"
operator: GreaterThan
value: !Float 75.0
parameters: 
  - !String "health"
"#;

    let condition = parse_condition_config(yaml_config)?;
    let result = resolver.resolve_condition(&condition, &context).await?;
    println!("Health check result: {}", result);

    // Example 2: Load multiple conditions from YAML string
    let yaml_configs = r#"
- condition_id: "has_mana"
  function_name: "get_actor_resource"
  operator: GreaterThanOrEqual
  value: !Float 30.0
  parameters: 
  - !String "health"

- condition_id: "has_category_item"
  function_name: "has_category_item"
  operator: Equal
  value: !Boolean true
  parameters: 
  - !String "health"

- condition_id: "has_category_item"
  function_name: "has_category_item"
  operator: Equal
  value: !Boolean true
  parameters: 
  - !String "health"
"#;

    let conditions = parse_condition_configs(yaml_configs)?;
    let results = resolver.resolve_conditions(&conditions, &context).await?;
    println!("Multiple conditions results: {:?}", results);

    // Example 3: Load condition chain from YAML string
    let yaml_chain = r#"
chain_id: "weather_spell_conditions"
logic: And
conditions:
  - condition_id: "has_weapon"
    function_name: "has_category_item"
    operator: Equal
    value: !Boolean true
    parameters: 
      - !String "weapon"

  - condition_id: "has_water_spell"
    function_name: "get_element_mastery"
    operator: GreaterThan
    value: !Float 100.0
    parameters:
      - !String "water"

  - condition_id: "sufficient_mana"
    function_name: "get_actor_resource"
    operator: GreaterThanOrEqual
    value: !Float 20.0
    parameters: 
      - !String "mana"
"#;

    let chain_config = parse_condition_chain_config(yaml_chain)?;
    let chain_result = resolver.resolve_condition_chain(&chain_config, &context).await?;
    println!("Weather spell conditions result: {}", chain_result);

    // Example 4: Demonstrate different operators
    let operators_example = vec![
        ("Equal", ConditionOperator::Equal),
        ("NotEqual", ConditionOperator::NotEqual),
        ("GreaterThan", ConditionOperator::GreaterThan),
        ("LessThan", ConditionOperator::LessThan),
        ("GreaterThanOrEqual", ConditionOperator::GreaterThanOrEqual),
        ("LessThanOrEqual", ConditionOperator::LessThanOrEqual),
    ];

    for (name, operator) in operators_example {
        let condition = ConditionConfig {
            condition_id: format!("test_{}", name.to_lowercase()),
            function_name: "get_actor_resource".to_string(),
            operator: operator.clone(),
            value: ConditionValue::Float(50.0),
            parameters: vec![ConditionParameter::String("health".to_string())],
        };

        let result = resolver.resolve_condition(&condition, &context).await?;
        println!("{} (health >= 50): {}", name, result);
    }

    Ok(())
}

