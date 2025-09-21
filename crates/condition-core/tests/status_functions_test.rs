#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use condition_core::*;
use condition_core::status_functions::create_status_function_registry;
use std::sync::Arc;
use std::time::SystemTime;

// Mock StatusDataProvider for testing
struct MockStatusDataProvider;

#[async_trait::async_trait]
impl data_provider::StatusDataProvider for MockStatusDataProvider {
    // Basic Status Functions
    async fn has_status_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok(true),
            ("test_player", "burn") => Ok(false),
            ("test_player", "stun") => Ok(true),
            ("test_player", "charm") => Ok(false),
            _ => Ok(false),
        }
    }

    async fn get_status_effect_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok(3),
            ("test_player", "burn") => Ok(0),
            ("test_player", "stun") => Ok(1),
            ("test_player", "charm") => Ok(0),
            _ => Ok(0),
        }
    }

    async fn get_status_effect_magnitude(&self, actor_id: &str, effect_id: &str) -> ConditionResult<f64> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok(15.5),
            ("test_player", "burn") => Ok(0.0),
            ("test_player", "stun") => Ok(100.0),
            ("test_player", "charm") => Ok(0.0),
            _ => Ok(0.0),
        }
    }

    async fn is_status_effect_active(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok(true),
            ("test_player", "burn") => Ok(false),
            ("test_player", "stun") => Ok(true),
            ("test_player", "charm") => Ok(false),
            _ => Ok(false),
        }
    }

    async fn is_status_effect_expired(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok(false),
            ("test_player", "burn") => Ok(true),
            ("test_player", "stun") => Ok(false),
            ("test_player", "charm") => Ok(true),
            _ => Ok(true),
        }
    }

    // Status Immunity Functions
    async fn has_status_immunity(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok(false),
            ("test_player", "charm") => Ok(true),
            ("test_player", "stun") => Ok(false),
            ("test_player", "fear") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn get_status_immunity_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok(0),
            ("test_player", "charm") => Ok(2),
            ("test_player", "stun") => Ok(0),
            ("test_player", "fear") => Ok(1),
            _ => Ok(0),
        }
    }

    async fn is_status_immunity_active(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok(false),
            ("test_player", "charm") => Ok(true),
            ("test_player", "stun") => Ok(false),
            ("test_player", "fear") => Ok(true),
            _ => Ok(false),
        }
    }

    // Status Category Functions
    async fn has_status_category(&self, actor_id: &str, category: &str) -> ConditionResult<bool> {
        match (actor_id, category) {
            ("test_player", "debuff") => Ok(true),
            ("test_player", "buff") => Ok(false),
            ("test_player", "curse") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn get_status_category_count(&self, actor_id: &str, category: &str) -> ConditionResult<u32> {
        match (actor_id, category) {
            ("test_player", "debuff") => Ok(2),
            ("test_player", "buff") => Ok(0),
            ("test_player", "curse") => Ok(1),
            _ => Ok(0),
        }
    }

    async fn list_status_categories(&self, actor_id: &str) -> ConditionResult<Vec<String>> {
        match actor_id {
            "test_player" => Ok(vec!["debuff".to_string(), "curse".to_string()]),
            _ => Ok(vec![]),
        }
    }

    // Status Interaction Functions
    async fn is_status_effect_stackable(&self, effect_id: &str) -> ConditionResult<bool> {
        match effect_id {
            "poison" => Ok(true),
            "burn" => Ok(true),
            "stun" => Ok(false),
            "charm" => Ok(false),
            _ => Ok(false),
        }
    }

    async fn can_status_effect_stack(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok(true),
            ("test_player", "burn") => Ok(true),
            ("test_player", "stun") => Ok(false),
            ("test_player", "charm") => Ok(false),
            _ => Ok(false),
        }
    }

    async fn get_status_effect_interaction(&self, effect_id: &str, target_effect_id: &str) -> ConditionResult<String> {
        match (effect_id, target_effect_id) {
            ("poison", "burn") => Ok("amplify".to_string()),
            ("burn", "poison") => Ok("amplify".to_string()),
            ("stun", "charm") => Ok("suppress".to_string()),
            ("charm", "stun") => Ok("suppress".to_string()),
            _ => Ok("neutral".to_string()),
        }
    }

    async fn get_status_effect_priority(&self, effect_id: &str) -> ConditionResult<i32> {
        match effect_id {
            "stun" => Ok(100),
            "charm" => Ok(90),
            "poison" => Ok(50),
            "burn" => Ok(40),
            _ => Ok(0),
        }
    }

    async fn get_status_effect_source(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok("spider_bite".to_string()),
            ("test_player", "stun") => Ok("lightning_bolt".to_string()),
            _ => Ok("unknown".to_string()),
        }
    }

    async fn get_status_effect_target(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok("test_player".to_string()),
            ("test_player", "stun") => Ok("test_player".to_string()),
            _ => Ok("unknown".to_string()),
        }
    }

    // Status Movement Functions
    async fn has_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<bool> {
        match (actor_id, restriction_type) {
            ("test_player", "root") => Ok(false),
            ("test_player", "slow") => Ok(true),
            ("test_player", "paralyze") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn get_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<f64> {
        match (actor_id, restriction_type) {
            ("test_player", "root") => Ok(0.0),
            ("test_player", "slow") => Ok(0.5),
            ("test_player", "paralyze") => Ok(1.0),
            _ => Ok(0.0),
        }
    }

    // Status Visual/Audio Functions
    async fn has_status_visual_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok(true),
            ("test_player", "burn") => Ok(false),
            ("test_player", "stun") => Ok(true),
            ("test_player", "charm") => Ok(false),
            _ => Ok(false),
        }
    }

    async fn get_status_visual_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok("green_aura".to_string()),
            ("test_player", "stun") => Ok("sparkles".to_string()),
            _ => Ok("none".to_string()),
        }
    }

    async fn has_status_audio_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok(true),
            ("test_player", "burn") => Ok(false),
            ("test_player", "stun") => Ok(true),
            ("test_player", "charm") => Ok(false),
            _ => Ok(false),
        }
    }

    async fn get_status_audio_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => Ok("hissing".to_string()),
            ("test_player", "stun") => Ok("buzzing".to_string()),
            _ => Ok("none".to_string()),
        }
    }

    // Status Properties Functions
    async fn get_status_effect_properties(&self, actor_id: &str, effect_id: &str) -> ConditionResult<std::collections::HashMap<String, serde_json::Value>> {
        let mut properties = std::collections::HashMap::new();
        match (actor_id, effect_id) {
            ("test_player", "poison") => {
                properties.insert("damage_per_tick".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(5.0).unwrap()));
                properties.insert("duration".to_string(), serde_json::Value::Number(serde_json::Number::from(30)));
                properties.insert("stackable".to_string(), serde_json::Value::Bool(true));
            },
            ("test_player", "stun") => {
                properties.insert("duration".to_string(), serde_json::Value::Number(serde_json::Number::from(5)));
                properties.insert("stackable".to_string(), serde_json::Value::Bool(false));
            },
            _ => {}
        }
        Ok(properties)
    }

    async fn has_status_effect_property(&self, actor_id: &str, effect_id: &str, property: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id, property) {
            ("test_player", "poison", "stackable") => Ok(true),
            ("test_player", "poison", "damage_per_tick") => Ok(true),
            ("test_player", "stun", "stackable") => Ok(false),
            ("test_player", "stun", "duration") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn get_status_effect_property(&self, actor_id: &str, effect_id: &str, property: &str) -> ConditionResult<serde_json::Value> {
        match (actor_id, effect_id, property) {
            ("test_player", "poison", "damage_per_tick") => Ok(serde_json::Value::Number(serde_json::Number::from_f64(5.0).unwrap())),
            ("test_player", "poison", "duration") => Ok(serde_json::Value::Number(serde_json::Number::from(30))),
            ("test_player", "stun", "duration") => Ok(serde_json::Value::Number(serde_json::Number::from(5))),
            _ => Ok(serde_json::Value::Null),
        }
    }

    // Status History Functions
    async fn get_status_effect_history(&self, actor_id: &str, effect_id: &str) -> ConditionResult<Vec<StatusEffectHistory>> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => {
                let history = vec![
                    StatusEffectHistory {
                        effect_id: "poison".to_string(),
                        actor_id: "test_player".to_string(),
                        applied_at: SystemTime::now(),
                        removed_at: None,
                        magnitude: 15.5,
                        source: "spider_bite".to_string(),
                        duration: 30.0,
                        stack_count: 3,
                    }
                ];
                Ok(history)
            },
            _ => Ok(vec![]),
        }
    }

    async fn get_status_effect_timeline(&self, actor_id: &str, effect_id: &str) -> ConditionResult<Vec<StatusEffectTimeline>> {
        match (actor_id, effect_id) {
            ("test_player", "poison") => {
                let timeline = vec![
                    StatusEffectTimeline {
                        effect_id: "poison".to_string(),
                        actor_id: "test_player".to_string(),
                        timestamp: SystemTime::now(),
                        event_type: StatusEffectEventType::Applied,
                        magnitude: 15.5,
                        stack_count: 3,
                        source: "spider_bite".to_string(),
                    }
                ];
                Ok(timeline)
            },
            _ => Ok(vec![]),
        }
    }

    // Legacy functions for backward compatibility
    async fn get_status_effect_level(&self, status_id: &str, actor_id: &str) -> ConditionResult<i64> {
        match (actor_id, status_id) {
            ("test_player", "poison") => Ok(3),
            ("test_player", "stun") => Ok(1),
            _ => Ok(0),
        }
    }

    async fn has_category_status(&self, category_id: &str, actor_id: &str) -> ConditionResult<bool> {
        match (actor_id, category_id) {
            ("test_player", "debuff") => Ok(true),
            ("test_player", "curse") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn list_status_effects(&self) -> ConditionResult<Vec<String>> {
        Ok(vec![
            "poison".to_string(),
            "burn".to_string(),
            "stun".to_string(),
            "charm".to_string(),
            "fear".to_string(),
        ])
    }
}

// Test helper function
fn create_test_context() -> ConditionContext {
    ConditionContext {
        target: ActorTarget { id: "test_player".to_string() },
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

// Test helper function to create resolver
fn create_test_resolver() -> ConditionResolver {
    let mut data_registry = DataProviderRegistry::new();
    data_registry.register_status_provider(Box::new(MockStatusDataProvider));
    ConditionResolver::new(data_registry)
}

#[tokio::test]
async fn test_has_status_effect() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test positive case
    let condition = ConditionConfig {
        condition_id: "has_poison".to_string(),
        function_name: "has_status_effect".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("poison".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Player should have poison effect");

    // Test negative case
    let condition = ConditionConfig {
        condition_id: "has_burn".to_string(),
        function_name: "has_status_effect".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("burn".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(!result, "Player should not have burn effect");
}

#[tokio::test]
async fn test_get_status_effect_count() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let condition = ConditionConfig {
        condition_id: "poison_count".to_string(),
        function_name: "get_status_effect_count".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Integer(2),
        parameters: vec![ConditionParameter::String("poison".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Player should have more than 2 poison stacks");
}

#[tokio::test]
async fn test_get_status_effect_magnitude() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let condition = ConditionConfig {
        condition_id: "poison_magnitude".to_string(),
        function_name: "get_status_effect_magnitude".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(10.0),
        parameters: vec![ConditionParameter::String("poison".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Poison magnitude should be greater than 10.0");
}

#[tokio::test]
async fn test_is_status_effect_active() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test active effect
    let condition = ConditionConfig {
        condition_id: "poison_active".to_string(),
        function_name: "is_status_effect_active".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("poison".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Poison should be active");

    // Test inactive effect
    let condition = ConditionConfig {
        condition_id: "burn_active".to_string(),
        function_name: "is_status_effect_active".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("burn".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(!result, "Burn should not be active");
}

#[tokio::test]
async fn test_is_status_effect_expired() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test expired effect
    let condition = ConditionConfig {
        condition_id: "burn_expired".to_string(),
        function_name: "is_status_effect_expired".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("burn".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Burn should be expired");

    // Test non-expired effect
    let condition = ConditionConfig {
        condition_id: "poison_expired".to_string(),
        function_name: "is_status_effect_expired".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("poison".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(!result, "Poison should not be expired");
}

#[tokio::test]
async fn test_has_status_immunity() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test immunity
    let condition = ConditionConfig {
        condition_id: "charm_immunity".to_string(),
        function_name: "has_status_immunity".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("charm".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Player should have charm immunity");

    // Test no immunity
    let condition = ConditionConfig {
        condition_id: "poison_immunity".to_string(),
        function_name: "has_status_immunity".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("poison".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(!result, "Player should not have poison immunity");
}

#[tokio::test]
async fn test_get_status_immunity_count() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let condition = ConditionConfig {
        condition_id: "charm_immunity_count".to_string(),
        function_name: "get_status_immunity_count".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Integer(1),
        parameters: vec![ConditionParameter::String("charm".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Player should have more than 1 charm immunity");
}

#[tokio::test]
async fn test_is_status_immunity_active() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let condition = ConditionConfig {
        condition_id: "charm_immunity_active".to_string(),
        function_name: "is_status_immunity_active".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("charm".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Charm immunity should be active");
}

#[tokio::test]
async fn test_has_status_category() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test category exists
    let condition = ConditionConfig {
        condition_id: "has_debuff".to_string(),
        function_name: "has_status_category".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("debuff".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Player should have debuff category");

    // Test category doesn't exist
    let condition = ConditionConfig {
        condition_id: "has_buff".to_string(),
        function_name: "has_status_category".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("buff".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(!result, "Player should not have buff category");
}

#[tokio::test]
async fn test_get_status_category_count() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let condition = ConditionConfig {
        condition_id: "debuff_count".to_string(),
        function_name: "get_status_category_count".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Integer(1),
        parameters: vec![ConditionParameter::String("debuff".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Player should have more than 1 debuff");
}

#[tokio::test]
async fn test_is_status_effect_stackable() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test stackable effect
    let condition = ConditionConfig {
        condition_id: "poison_stackable".to_string(),
        function_name: "is_status_effect_stackable".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("poison".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Poison should be stackable");

    // Test non-stackable effect
    let condition = ConditionConfig {
        condition_id: "stun_stackable".to_string(),
        function_name: "is_status_effect_stackable".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("stun".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(!result, "Stun should not be stackable");
}

#[tokio::test]
async fn test_can_status_effect_stack() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test can stack
    let condition = ConditionConfig {
        condition_id: "poison_can_stack".to_string(),
        function_name: "can_status_effect_stack".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("poison".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Poison should be able to stack");

    // Test cannot stack
    let condition = ConditionConfig {
        condition_id: "stun_can_stack".to_string(),
        function_name: "can_status_effect_stack".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("stun".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(!result, "Stun should not be able to stack");
}

#[tokio::test]
async fn test_get_status_effect_interaction() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let condition = ConditionConfig {
        condition_id: "poison_burn_interaction".to_string(),
        function_name: "get_status_effect_interaction".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::String("amplify".to_string()),
        parameters: vec![
            ConditionParameter::String("poison".to_string()),
            ConditionParameter::String("burn".to_string()),
        ],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Poison and burn should have amplify interaction");
}

#[tokio::test]
async fn test_get_status_effect_priority() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let condition = ConditionConfig {
        condition_id: "stun_priority".to_string(),
        function_name: "get_status_effect_priority".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Integer(90),
        parameters: vec![ConditionParameter::String("stun".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Stun should have priority greater than 90");
}

#[tokio::test]
async fn test_has_status_movement_restriction() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test has restriction
    let condition = ConditionConfig {
        condition_id: "has_slow".to_string(),
        function_name: "has_status_movement_restriction".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("slow".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Player should have slow movement restriction");

    // Test no restriction
    let condition = ConditionConfig {
        condition_id: "has_root".to_string(),
        function_name: "has_status_movement_restriction".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("root".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(!result, "Player should not have root movement restriction");
}

#[tokio::test]
async fn test_get_status_movement_restriction() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let condition = ConditionConfig {
        condition_id: "slow_magnitude".to_string(),
        function_name: "get_status_movement_restriction".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(0.3),
        parameters: vec![ConditionParameter::String("slow".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Slow movement restriction should be greater than 0.3");
}

#[tokio::test]
async fn test_has_status_visual_effect() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test has visual effect
    let condition = ConditionConfig {
        condition_id: "poison_visual".to_string(),
        function_name: "has_status_visual_effect".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("poison".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Poison should have visual effect");

    // Test no visual effect
    let condition = ConditionConfig {
        condition_id: "burn_visual".to_string(),
        function_name: "has_status_visual_effect".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("burn".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(!result, "Burn should not have visual effect");
}

#[tokio::test]
async fn test_has_status_audio_effect() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test has audio effect
    let condition = ConditionConfig {
        condition_id: "poison_audio".to_string(),
        function_name: "has_status_audio_effect".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("poison".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Poison should have audio effect");

    // Test no audio effect
    let condition = ConditionConfig {
        condition_id: "burn_audio".to_string(),
        function_name: "has_status_audio_effect".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![ConditionParameter::String("burn".to_string())],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(!result, "Burn should not have audio effect");
}

#[tokio::test]
async fn test_has_status_effect_property() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test has property
    let condition = ConditionConfig {
        condition_id: "poison_stackable_property".to_string(),
        function_name: "has_status_effect_property".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![
            ConditionParameter::String("poison".to_string()),
            ConditionParameter::String("stackable".to_string()),
        ],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(result, "Poison should have stackable property");

    // Test no property
    let condition = ConditionConfig {
        condition_id: "stun_stackable_property".to_string(),
        function_name: "has_status_effect_property".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![
            ConditionParameter::String("stun".to_string()),
            ConditionParameter::String("stackable".to_string()),
        ],
    };

    let result = resolver.resolve_condition(&condition, &context).await.unwrap();
    assert!(!result, "Stun should not have stackable property");
}

#[tokio::test]
async fn test_error_handling() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    // Test invalid parameter count
    let condition = ConditionConfig {
        condition_id: "invalid_params".to_string(),
        function_name: "get_status_effect_interaction".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::String("amplify".to_string()),
        parameters: vec![ConditionParameter::String("poison".to_string())], // Missing second parameter
    };

    let result = resolver.resolve_condition(&condition, &context).await;
    assert!(result.is_err(), "Should return error for invalid parameter count");

    // Test function not found
    let condition = ConditionConfig {
        condition_id: "unknown_function".to_string(),
        function_name: "unknown_function".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::Boolean(true),
        parameters: vec![],
    };

    let result = resolver.resolve_condition(&condition, &context).await;
    assert!(result.is_err(), "Should return error for unknown function");
}

#[tokio::test]
async fn test_status_condition_chain() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let chain_config = ConditionChainConfig {
        chain_id: "poisoned_and_not_immune".to_string(),
        logic: ChainLogic::And,
        conditions: vec![
            ConditionConfig {
                condition_id: "has_poison".to_string(),
                function_name: "has_status_effect".to_string(),
                operator: ConditionOperator::Equal,
                value: ConditionValue::Boolean(true),
                parameters: vec![ConditionParameter::String("poison".to_string())],
            },
            ConditionConfig {
                condition_id: "not_immune_to_poison".to_string(),
                function_name: "has_status_immunity".to_string(),
                operator: ConditionOperator::Equal,
                value: ConditionValue::Boolean(false),
                parameters: vec![ConditionParameter::String("poison".to_string())],
            },
        ],
    };

    let result = resolver.resolve_condition_chain(&chain_config, &context).await.unwrap();
    assert!(result, "Player should be poisoned and not immune to poison");
}

#[tokio::test]
async fn test_performance() {
    let resolver = create_test_resolver();
    let context = create_test_context();

    let start = std::time::Instant::now();
    let iterations = 1000;

    for i in 0..iterations {
        let condition = ConditionConfig {
            condition_id: format!("perf_test_{}", i),
            function_name: "has_status_effect".to_string(),
            operator: ConditionOperator::Equal,
            value: ConditionValue::Boolean(true),
            parameters: vec![ConditionParameter::String("poison".to_string())],
        };

        let _result = resolver.resolve_condition(&condition, &context).await.unwrap();
    }

    let duration = start.elapsed();
    let avg_time = duration.as_nanos() as f64 / iterations as f64;
    let calls_per_second = 1_000_000_000.0 / avg_time;

    println!("Processed {} status condition checks in {:?}", iterations, duration);
    println!("Average time per check: {:.2} ns", avg_time);
    println!("Calls per second: {:.0}", calls_per_second);

    // Performance should be reasonable (less than 1ms per call)
    assert!(avg_time < 1_000_000.0, "Average time per call should be less than 1ms");
}
