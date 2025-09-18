//! Unit tests for Condition Core

use condition_core::*;

#[test]
fn test_condition_value_equality() {
    let val1 = ConditionValue::Integer(42);
    let val2 = ConditionValue::Integer(42);
    let val3 = ConditionValue::Integer(43);

    assert_eq!(val1, val2);
    assert_ne!(val1, val3);
}

#[test]
fn test_condition_parameter_equality() {
    let param1 = ConditionParameter::String("test".to_string());
    let param2 = ConditionParameter::String("test".to_string());
    let param3 = ConditionParameter::String("different".to_string());

    assert_eq!(param1, param2);
    assert_ne!(param1, param3);
}

#[test]
fn test_weather_type_serialization() {
    let weather = WeatherType::Rain;
    let serialized = serde_yaml::to_string(&weather).unwrap();
    let deserialized: WeatherType = serde_yaml::from_str(&serialized).unwrap();
    
    assert!(matches!(deserialized, WeatherType::Rain));
}

#[test]
fn test_condition_operator_serialization() {
    let operator = ConditionOperator::GreaterThan;
    let serialized = serde_yaml::to_string(&operator).unwrap();
    let deserialized: ConditionOperator = serde_yaml::from_str(&serialized).unwrap();
    
    assert!(matches!(deserialized, ConditionOperator::GreaterThan));
}

#[test]
fn test_chain_logic_serialization() {
    let logic = ChainLogic::And;
    let serialized = serde_yaml::to_string(&logic).unwrap();
    let deserialized: ChainLogic = serde_yaml::from_str(&serialized).unwrap();
    
    assert!(matches!(deserialized, ChainLogic::And));
}

#[test]
fn test_function_registry() {
    let mut registry = FunctionRegistry::new();
    
    // Test empty registry
    assert!(registry.get("unknown_function").is_none());
    assert!(registry.list().is_empty());
    
    // Register a function
    registry.register(Box::new(GetActorResourceFunction::new(None)));
    
    // Test function exists
    assert!(registry.get("get_actor_resource").is_some());
    assert_eq!(registry.list().len(), 1);
    assert!(registry.list().contains(&"get_actor_resource"));
}

#[test]
fn test_condition_config_serialization() {
    let config = ConditionConfig {
        condition_id: "test_condition".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(50.0),
        parameters: vec![
            ConditionParameter::String("test_param".to_string()),
            ConditionParameter::Integer(42),
        ],
    };

    let serialized = serde_yaml::to_string(&config).unwrap();
    let deserialized: ConditionConfig = serde_yaml::from_str(&serialized).unwrap();
    
    assert_eq!(config.condition_id, deserialized.condition_id);
    assert_eq!(config.function_name, deserialized.function_name);
    assert!(matches!(deserialized.operator, ConditionOperator::GreaterThan));
    assert!(matches!(deserialized.value, ConditionValue::Float(50.0)));
    assert_eq!(config.parameters.len(), deserialized.parameters.len());
}

#[test]
fn test_condition_chain_config_serialization() {
    let chain_config = ConditionChainConfig {
        chain_id: "test_chain".to_string(),
        logic: ChainLogic::And,
        conditions: vec![
            ConditionConfig {
                condition_id: "condition1".to_string(),
                function_name: "get_actor_resource".to_string(),
                operator: ConditionOperator::GreaterThan,
                value: ConditionValue::Float(50.0),
                parameters: vec![ConditionParameter::String("health".to_string())],
            },
            ConditionConfig {
                condition_id: "condition2".to_string(),
                function_name: "get_actor_mana".to_string(),
                operator: ConditionOperator::GreaterThan,
                value: ConditionValue::Float(25.0),
                parameters: vec![ConditionParameter::String("health".to_string())],
            },
        ],
    };

    let serialized = serde_yaml::to_string(&chain_config).unwrap();
    let deserialized: ConditionChainConfig = serde_yaml::from_str(&serialized).unwrap();
    
    assert_eq!(chain_config.chain_id, deserialized.chain_id);
    assert!(matches!(deserialized.logic, ChainLogic::And));
    assert_eq!(chain_config.conditions.len(), deserialized.conditions.len());
}

#[test]
fn test_error_types() {
    let error = ConditionError::FunctionNotFound {
        function_name: "test_function".to_string(),
    };
    
    assert!(error.to_string().contains("Function not found"));
    assert!(error.to_string().contains("test_function"));
}

#[test]
fn test_actor_target_creation() {
    let target = ActorTarget {
        id: "test_actor".to_string(),
    };
    
    assert_eq!(target.id, "test_actor");
}

#[test]
fn test_world_state_creation() {
    let world_state = WorldState {
        time_of_day: 12.0,
        season: "spring".to_string(),
        temperature: 20.0,
        humidity: 0.5,
    };
    
    assert_eq!(world_state.time_of_day, 12.0);
    assert_eq!(world_state.season, "spring");
    assert_eq!(world_state.temperature, 20.0);
    assert_eq!(world_state.humidity, 0.5);
}
