//! Integration tests for Condition Core

use condition_core::*;
use std::time::SystemTime;

// Mock implementations for testing
struct MockElementDataProvider;
struct MockResourceDataProvider;
struct MockCategoryDataProvider;
struct MockActorDataProvider;

#[async_trait::async_trait]
impl ElementDataProvider for MockElementDataProvider {
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
            ("shadow", "light") => Ok("hide".to_string()),
            ("metal", "lightning") => Ok("attract".to_string()),
            _ => Ok("neutral".to_string()),
        }
    }

    async fn list_elements(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![
            "fire".to_string(),
            "water".to_string(),
            "earth".to_string(),
            "air".to_string(),
            "light".to_string(),
            "dark".to_string(),
        ])
    }
}

#[async_trait::async_trait]
impl ResourceDataProvider for MockResourceDataProvider {
    async fn get_resource_value(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        match resource_id {
            "health" => Ok(100.0),
            "mana" => Ok(50.0),
            "stamina" => Ok(75.0),
            "sanity" => Ok(30.0),
            _ => Ok(50.0),
        }
    }

    async fn get_resource_percentage(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        match resource_id {
            "health" => Ok(1.0),
            "mana" => Ok(0.5),
            "stamina" => Ok(0.75),
            "sanity" => Ok(0.3),
            _ => Ok(0.5),
        }
    }

    async fn is_resource_empty(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match resource_id {
            "sanity" => Ok(false),
            _ => Ok(false),
        }
    }

    async fn is_resource_below_threshold(&self, resource_id: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            "sanity" => 30.0,
            _ => 50.0,
        };
        Ok(current < threshold)
    }

    async fn is_resource_above_threshold(&self, resource_id: &str, threshold: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            "sanity" => 30.0,
            _ => 50.0,
        };
        Ok(current > threshold)
    }

    async fn is_resource_below_percentage(&self, resource_id: &str, percentage: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current_percentage = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            "sanity" => 30.0,
            _ => 50.0,
        };
        Ok(current_percentage < percentage)
    }

    async fn is_resource_above_percentage(&self, resource_id: &str, percentage: f64, _actor_id: &str) -> ConditionResult<bool> {
        let current_percentage = match resource_id {
            "health" => 100.0,
            "mana" => 50.0,
            "stamina" => 75.0,
            "sanity" => 30.0,
            _ => 50.0,
        };
        Ok(current_percentage > percentage)
    }

    async fn get_resource_max(&self, resource_id: &str, _actor_id: &str) -> ConditionResult<f64> {
        match resource_id {
            "health" => Ok(100.0),
            "mana" => Ok(100.0),
            "stamina" => Ok(100.0),
            "sanity" => Ok(100.0),
            _ => Ok(100.0),
        }
    }

    async fn list_resources(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![
            "health".to_string(),
            "mana".to_string(),
            "stamina".to_string(),
            "sanity".to_string(),
        ])
    }
}

#[async_trait::async_trait]
impl CategoryDataProvider for MockCategoryDataProvider {
    async fn has_category_item(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match category_id {
            "weapon" => Ok(true),
            "armor" => Ok(true),
            "potion" => Ok(false),
            _ => Ok(true),
        }
    }

    async fn is_category_available(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match category_id {
            "combat" => Ok(true),
            "magic" => Ok(true),
            "raid" => Ok(false),
            _ => Ok(true),
        }
    }

    async fn is_category_blocked(&self, category_id: &str, _actor_id: &str) -> ConditionResult<bool> {
        match category_id {
            "raid" => Ok(true),
            _ => Ok(false),
        }
    }

    async fn get_category_item_count(&self, category_id: &str, _actor_id: &str) -> ConditionResult<i64> {
        match category_id {
            "weapon" => Ok(2),
            "armor" => Ok(1),
            "potion" => Ok(0),
            _ => Ok(1),
        }
    }

    async fn list_categories(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![
            "weapon".to_string(),
            "armor".to_string(),
            "potion".to_string(),
            "combat".to_string(),
            "magic".to_string(),
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

/// Create a test context for testing
fn create_test_context() -> ConditionContext {
    ConditionContext {
        target: ActorTarget {
            id: "test_actor".to_string(),
        },
        world_id: "test_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Clear,
        world_state: WorldState {
            time_of_day: 12.0,
            season: "spring".to_string(),
            temperature: 20.0,
            humidity: 0.5,
        },
    }
}

#[tokio::test]
async fn test_single_condition_resolution() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let condition = ConditionConfig {
        condition_id: "test_health".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(25.0),
        parameters: vec![ConditionParameter::String("mana".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await;
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_multiple_conditions_resolution() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let conditions = vec![
        ConditionConfig {
        condition_id: "health_check".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(25.0),
        parameters: vec![ConditionParameter::String("health".to_string())],
        },
        ConditionConfig {
            condition_id: "mana_check".to_string(),
            function_name: "get_actor_resource".to_string(),
            operator: ConditionOperator::GreaterThan,
            value: ConditionValue::Float(25.0),
            parameters: vec![ConditionParameter::String("mana".to_string())],
        },
    ];

    let results = resolver.resolve_conditions(&conditions, &context).await;
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 2);
    assert!(results[0]);
    assert!(results[1]);
}

#[tokio::test]
async fn test_condition_chain_and_logic() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let chain_config = ConditionChainConfig {
        chain_id: "test_chain".to_string(),
        logic: ChainLogic::And,
        conditions: vec![
            ConditionConfig {
                condition_id: "health_check".to_string(),
                function_name: "get_actor_resource".to_string(),
                operator: ConditionOperator::GreaterThan,
                value: ConditionValue::Float(25.0),
                parameters: vec![ConditionParameter::String("health".to_string())],
            },
            ConditionConfig {
                condition_id: "mana_check".to_string(),
                function_name: "get_actor_resource".to_string(),
                operator: ConditionOperator::GreaterThan,
                value: ConditionValue::Float(25.0),
                parameters: vec![ConditionParameter::String("mana".to_string())],
            },
        ],
    };

    let result = resolver.resolve_condition_chain(&chain_config, &context).await;
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_condition_chain_or_logic() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let chain_config = ConditionChainConfig {
        chain_id: "test_chain".to_string(),
        logic: ChainLogic::Or,
        conditions: vec![
            ConditionConfig {
                condition_id: "health_check".to_string(),
                function_name: "get_actor_resource".to_string(),
                operator: ConditionOperator::LessThan,
                value: ConditionValue::Float(10.0), // This should be false
                parameters: vec![ConditionParameter::String("health".to_string())],
            },
            ConditionConfig {
                condition_id: "mana_check".to_string(),
                function_name: "get_actor_resource".to_string(),
                operator: ConditionOperator::GreaterThan,
                value: ConditionValue::Float(25.0), // This should be true
                parameters: vec![ConditionParameter::String("mana".to_string())],
            },
        ],
    };

    let result = resolver.resolve_condition_chain(&chain_config, &context).await;
    assert!(result.is_ok());
    assert!(result.unwrap()); // Should be true because of OR logic
}

#[tokio::test]
async fn test_condition_chain_not_logic() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let chain_config = ConditionChainConfig {
        chain_id: "test_chain".to_string(),
        logic: ChainLogic::Not,
        conditions: vec![
            ConditionConfig {
                condition_id: "health_check".to_string(),
                function_name: "get_actor_resource".to_string(),
                operator: ConditionOperator::LessThan,
                value: ConditionValue::Float(10.0), // This should be false
                parameters: vec![ConditionParameter::String("health".to_string())],
            },
        ],
    };

    let result = resolver.resolve_condition_chain(&chain_config, &context).await;
    assert!(result.is_ok());
    assert!(result.unwrap()); // Should be true because of NOT logic
}

#[tokio::test]
async fn test_condition_chain_xor_logic() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let chain_config = ConditionChainConfig {
        chain_id: "test_chain".to_string(),
        logic: ChainLogic::Xor,
        conditions: vec![
            ConditionConfig {
                condition_id: "health_check".to_string(),
                function_name: "get_actor_resource".to_string(),
                operator: ConditionOperator::GreaterThan,
                value: ConditionValue::Float(25.0), // This should be true
                parameters: vec![ConditionParameter::String("mana".to_string())],
            },
            ConditionConfig {
                condition_id: "mana_check".to_string(),
                function_name: "get_actor_resource".to_string(),
                operator: ConditionOperator::LessThan,
                value: ConditionValue::Float(10.0), // This should be false
                parameters: vec![ConditionParameter::String("mana".to_string())],
            },
        ],
    };

    let result = resolver.resolve_condition_chain(&chain_config, &context).await;
    assert!(result.is_ok());
    assert!(result.unwrap()); // Should be true because exactly one condition is true
}

#[tokio::test]
async fn test_function_not_found_error() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let condition = ConditionConfig {
        condition_id: "test_unknown".to_string(),
        function_name: "unknown_function".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("mana".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        ConditionError::FunctionNotFound { function_name } => {
            assert_eq!(function_name, "unknown_function");
        }
        _ => panic!("Expected FunctionNotFound error"),
    }
}

#[tokio::test]
async fn test_invalid_parameter_error() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let condition = ConditionConfig {
        condition_id: "test_invalid".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![], // Missing required parameter
    };

    let result = resolver.resolve_condition(&condition, &context).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        ConditionError::InvalidParameter { function_name, parameter } => {
            assert_eq!(function_name, "get_actor_resource");
            assert_eq!(parameter, "resource_type");
        }
        _ => panic!("Expected InvalidParameter error"),
    }
}

#[tokio::test]
async fn test_yaml_parsing() {
    let yaml = r#"
condition_id: "test_yaml"
function_name: "get_actor_resource"
operator: GreaterThan
value: !Float 50.0
parameters: 
  - !String "health"
"#;

    let condition = parse_condition_config(yaml);
    if let Err(e) = &condition {
        println!("YAML parsing error: {:?}", e);
    }
    assert!(condition.is_ok());
    let condition = condition.unwrap();
    assert_eq!(condition.condition_id, "test_yaml");
    assert_eq!(condition.function_name, "get_actor_resource");
}

#[tokio::test]
async fn test_config_validation() {
    let valid_config = ConditionConfig {
        condition_id: "test".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Float(25.0),
        parameters: vec![ConditionParameter::String("mana".to_string())],
    };

    let result = validate_condition_config(&valid_config);
    assert!(result.is_ok());

    let invalid_config = ConditionConfig {
        condition_id: "".to_string(), // Empty ID
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Float(25.0),
        parameters: vec![ConditionParameter::String("mana".to_string())],
    };

    let result = validate_condition_config(&invalid_config);
    assert!(result.is_err());
}
