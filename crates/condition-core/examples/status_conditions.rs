#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use condition_core::*;
use condition_core::status_functions::create_status_function_registry;
use std::sync::Arc;
use std::time::SystemTime;

// Mock StatusDataProvider for demonstration
struct MockStatusDataProvider;

#[async_trait::async_trait]
impl data_provider::StatusDataProvider for MockStatusDataProvider {
    // Basic Status Functions
    async fn has_status_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok(true),
            ("player_1", "burn") => Ok(false),
            ("player_2", "stun") => Ok(true),
            ("player_2", "charm") => Ok(false),
            _ => Ok(false),
        }
    }

    async fn get_status_effect_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok(3),
            ("player_1", "burn") => Ok(0),
            ("player_2", "stun") => Ok(1),
            ("player_2", "charm") => Ok(0),
            _ => Ok(0),
        }
    }

    async fn get_status_effect_magnitude(&self, actor_id: &str, effect_id: &str) -> ConditionResult<f64> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok(15.5),
            ("player_1", "burn") => Ok(0.0),
            ("player_2", "stun") => Ok(100.0),
            ("player_2", "charm") => Ok(0.0),
            _ => Ok(0.0),
        }
    }

    async fn is_status_effect_active(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok(true),
            ("player_1", "burn") => Ok(false),
            ("player_2", "stun") => Ok(true),
            ("player_2", "charm") => Ok(false),
            _ => Ok(false),
        }
    }

    async fn is_status_effect_expired(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok(false),
            ("player_1", "burn") => Ok(true),
            ("player_2", "stun") => Ok(false),
            ("player_2", "charm") => Ok(true),
            _ => Ok(true),
        }
    }

    // Status Immunity Functions
    async fn has_status_immunity(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok(false),
            ("player_1", "charm") => Ok(true),
            ("player_2", "stun") => Ok(false),
            ("player_2", "fear") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn get_status_immunity_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok(0),
            ("player_1", "charm") => Ok(2),
            ("player_2", "stun") => Ok(0),
            ("player_2", "fear") => Ok(1),
            _ => Ok(0),
        }
    }

    async fn is_status_immunity_active(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok(false),
            ("player_1", "charm") => Ok(true),
            ("player_2", "stun") => Ok(false),
            ("player_2", "fear") => Ok(true),
            _ => Ok(false),
        }
    }

    // Status Category Functions
    async fn has_status_category(&self, actor_id: &str, category: &str) -> ConditionResult<bool> {
        match (actor_id, category) {
            ("player_1", "debuff") => Ok(true),
            ("player_1", "buff") => Ok(false),
            ("player_2", "debuff") => Ok(true),
            ("player_2", "buff") => Ok(false),
            _ => Ok(false),
        }
    }

    async fn get_status_category_count(&self, actor_id: &str, category: &str) -> ConditionResult<u32> {
        match (actor_id, category) {
            ("player_1", "debuff") => Ok(2),
            ("player_1", "buff") => Ok(0),
            ("player_2", "debuff") => Ok(1),
            ("player_2", "buff") => Ok(0),
            _ => Ok(0),
        }
    }

    async fn list_status_categories(&self, actor_id: &str) -> ConditionResult<Vec<String>> {
        match actor_id {
            "player_1" => Ok(vec!["debuff".to_string(), "poison".to_string()]),
            "player_2" => Ok(vec!["debuff".to_string(), "stun".to_string()]),
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
            ("player_1", "poison") => Ok(true),
            ("player_1", "burn") => Ok(true),
            ("player_2", "stun") => Ok(false),
            ("player_2", "charm") => Ok(false),
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
            ("player_1", "poison") => Ok("spider_bite".to_string()),
            ("player_2", "stun") => Ok("lightning_bolt".to_string()),
            _ => Ok("unknown".to_string()),
        }
    }

    async fn get_status_effect_target(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok("player_1".to_string()),
            ("player_2", "stun") => Ok("player_2".to_string()),
            _ => Ok("unknown".to_string()),
        }
    }

    // Status Movement Functions
    async fn has_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<bool> {
        match (actor_id, restriction_type) {
            ("player_1", "root") => Ok(false),
            ("player_1", "slow") => Ok(true),
            ("player_2", "root") => Ok(true),
            ("player_2", "slow") => Ok(false),
            _ => Ok(false),
        }
    }

    async fn get_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<f64> {
        match (actor_id, restriction_type) {
            ("player_1", "root") => Ok(0.0),
            ("player_1", "slow") => Ok(0.5),
            ("player_2", "root") => Ok(1.0),
            ("player_2", "slow") => Ok(0.0),
            _ => Ok(0.0),
        }
    }

    // Status Visual/Audio Functions
    async fn has_status_visual_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok(true),
            ("player_1", "burn") => Ok(false),
            ("player_2", "stun") => Ok(true),
            ("player_2", "charm") => Ok(false),
            _ => Ok(false),
        }
    }

    async fn get_status_visual_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok("green_aura".to_string()),
            ("player_2", "stun") => Ok("sparkles".to_string()),
            _ => Ok("none".to_string()),
        }
    }

    async fn has_status_audio_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok(true),
            ("player_1", "burn") => Ok(false),
            ("player_2", "stun") => Ok(true),
            ("player_2", "charm") => Ok(false),
            _ => Ok(false),
        }
    }

    async fn get_status_audio_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => Ok("hissing".to_string()),
            ("player_2", "stun") => Ok("buzzing".to_string()),
            _ => Ok("none".to_string()),
        }
    }

    // Status Properties Functions
    async fn get_status_effect_properties(&self, actor_id: &str, effect_id: &str) -> ConditionResult<std::collections::HashMap<String, serde_json::Value>> {
        let mut properties = std::collections::HashMap::new();
        match (actor_id, effect_id) {
            ("player_1", "poison") => {
                properties.insert("damage_per_tick".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(5.0).unwrap()));
                properties.insert("duration".to_string(), serde_json::Value::Number(serde_json::Number::from(30)));
                properties.insert("stackable".to_string(), serde_json::Value::Bool(true));
            },
            ("player_2", "stun") => {
                properties.insert("duration".to_string(), serde_json::Value::Number(serde_json::Number::from(5)));
                properties.insert("stackable".to_string(), serde_json::Value::Bool(false));
            },
            _ => {}
        }
        Ok(properties)
    }

    async fn has_status_effect_property(&self, actor_id: &str, effect_id: &str, property: &str) -> ConditionResult<bool> {
        match (actor_id, effect_id, property) {
            ("player_1", "poison", "stackable") => Ok(true),
            ("player_1", "poison", "damage_per_tick") => Ok(true),
            ("player_2", "stun", "stackable") => Ok(false),
            ("player_2", "stun", "duration") => Ok(true),
            _ => Ok(false),
        }
    }

    async fn get_status_effect_property(&self, actor_id: &str, effect_id: &str, property: &str) -> ConditionResult<serde_json::Value> {
        match (actor_id, effect_id, property) {
            ("player_1", "poison", "damage_per_tick") => Ok(serde_json::Value::Number(serde_json::Number::from_f64(5.0).unwrap())),
            ("player_1", "poison", "duration") => Ok(serde_json::Value::Number(serde_json::Number::from(30))),
            ("player_2", "stun", "duration") => Ok(serde_json::Value::Number(serde_json::Number::from(5))),
            _ => Ok(serde_json::Value::Null),
        }
    }

    // Status History Functions
    async fn get_status_effect_history(&self, actor_id: &str, effect_id: &str) -> ConditionResult<Vec<StatusEffectHistory>> {
        match (actor_id, effect_id) {
            ("player_1", "poison") => {
                let history = vec![
                    StatusEffectHistory {
                        effect_id: "poison".to_string(),
                        actor_id: "player_1".to_string(),
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
            ("player_1", "poison") => {
                let timeline = vec![
                    StatusEffectTimeline {
                        effect_id: "poison".to_string(),
                        actor_id: "player_1".to_string(),
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
            ("player_1", "poison") => Ok(3),
            ("player_2", "stun") => Ok(1),
            _ => Ok(0),
        }
    }

    async fn has_category_status(&self, category_id: &str, actor_id: &str) -> ConditionResult<bool> {
        match (actor_id, category_id) {
            ("player_1", "debuff") => Ok(true),
            ("player_2", "debuff") => Ok(true),
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Status Conditions Example - Chaos World Backend Service");
    println!("{}", "=".repeat(60));

    // Create status data provider
    let status_provider = Arc::new(MockStatusDataProvider);
    
    // Create status function registry
    let status_registry = create_status_function_registry(status_provider);
    
    // Create condition resolver with status functions
    let mut data_registry = DataProviderRegistry::new();
    data_registry.register_status_provider(Box::new(MockStatusDataProvider));
    let resolver = ConditionResolver::new(data_registry);

    // Create test contexts
    let player1_context = ConditionContext {
        target: ActorTarget { id: "player_1".to_string() },
        world_id: "test_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Clear,
        world_state: WorldState {
            time_of_day: 12.0,
            season: "spring".to_string(),
            temperature: 20.0,
            humidity: 0.5,
        },
    };

    let player2_context = ConditionContext {
        target: ActorTarget { id: "player_2".to_string() },
        world_id: "test_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Clear,
        world_state: WorldState {
            time_of_day: 12.0,
            season: "spring".to_string(),
            temperature: 20.0,
            humidity: 0.5,
        },
    };

    // Example 1: Basic Status Effect Checks
    println!("\n📋 Example 1: Basic Status Effect Checks");
    println!("{}", "-".repeat(40));

    let conditions = vec![
        ("has_status_effect", "poison", "player_1"),
        ("has_status_effect", "burn", "player_1"),
        ("is_status_effect_active", "poison", "player_1"),
        ("is_status_effect_expired", "burn", "player_1"),
        ("get_status_effect_count", "poison", "player_1"),
        ("get_status_effect_magnitude", "poison", "player_1"),
    ];

    for (function_name, effect_id, player_id) in conditions {
        let context = if player_id == "player_1" { &player1_context } else { &player2_context };
        let condition = ConditionConfig {
            condition_id: format!("check_{}", effect_id),
            function_name: function_name.to_string(),
            operator: ConditionOperator::Equal,
            value: ConditionValue::Boolean(true),
            parameters: vec![ConditionParameter::String(effect_id.to_string())],
        };

        let result = resolver.resolve_condition(&condition, context).await?;
        println!("  {} {} on {}: {}", function_name, effect_id, player_id, result);
    }

    // Example 2: Status Immunity Checks
    println!("\n🛡️ Example 2: Status Immunity Checks");
    println!("{}", "-".repeat(40));

    let immunity_conditions = vec![
        ("has_status_immunity", "poison", "player_1"),
        ("has_status_immunity", "charm", "player_1"),
        ("is_status_immunity_active", "charm", "player_1"),
        ("get_status_immunity_count", "charm", "player_1"),
    ];

    for (function_name, effect_id, player_id) in immunity_conditions {
        let context = if player_id == "player_1" { &player1_context } else { &player2_context };
        let condition = ConditionConfig {
            condition_id: format!("check_{}_immunity", effect_id),
            function_name: function_name.to_string(),
            operator: ConditionOperator::Equal,
            value: ConditionValue::Boolean(true),
            parameters: vec![ConditionParameter::String(effect_id.to_string())],
        };

        let result = resolver.resolve_condition(&condition, context).await?;
        println!("  {} {} on {}: {}", function_name, effect_id, player_id, result);
    }

    // Example 3: Status Category Checks
    println!("\n📂 Example 3: Status Category Checks");
    println!("{}", "-".repeat(40));

    let category_conditions = vec![
        ("has_status_category", "debuff", "player_1"),
        ("has_status_category", "buff", "player_1"),
        ("get_status_category_count", "debuff", "player_1"),
    ];

    for (function_name, category, player_id) in category_conditions {
        let context = if player_id == "player_1" { &player1_context } else { &player2_context };
        let condition = ConditionConfig {
            condition_id: format!("check_{}_category", category),
            function_name: function_name.to_string(),
            operator: ConditionOperator::Equal,
            value: ConditionValue::Boolean(true),
            parameters: vec![ConditionParameter::String(category.to_string())],
        };

        let result = resolver.resolve_condition(&condition, context).await?;
        println!("  {} {} on {}: {}", function_name, category, player_id, result);
    }

    // Example 4: Status Interaction Checks
    println!("\n🔄 Example 4: Status Interaction Checks");
    println!("{}", "-".repeat(40));

    let interaction_conditions = vec![
        ("is_status_effect_stackable", "poison", ""),
        ("is_status_effect_stackable", "stun", ""),
        ("can_status_effect_stack", "poison", "player_1"),
        ("can_status_effect_stack", "stun", "player_2"),
        ("get_status_effect_priority", "stun", ""),
        ("get_status_effect_priority", "poison", ""),
    ];

    for (function_name, effect_id, player_id) in interaction_conditions {
        let context = if player_id == "player_1" { &player1_context } else if player_id == "player_2" { &player2_context } else { &player1_context };
        let condition = ConditionConfig {
            condition_id: format!("check_{}", effect_id),
            function_name: function_name.to_string(),
            operator: ConditionOperator::Equal,
            value: ConditionValue::Boolean(true),
            parameters: vec![ConditionParameter::String(effect_id.to_string())],
        };

        let result = resolver.resolve_condition(&condition, context).await?;
        println!("  {} {}: {}", function_name, effect_id, result);
    }

    // Example 5: Status Movement Restrictions
    println!("\n🚶 Example 5: Status Movement Restrictions");
    println!("{}", "-".repeat(40));

    let movement_conditions = vec![
        ("has_status_movement_restriction", "root", "player_1"),
        ("has_status_movement_restriction", "slow", "player_1"),
        ("has_status_movement_restriction", "root", "player_2"),
        ("get_status_movement_restriction", "slow", "player_1"),
        ("get_status_movement_restriction", "root", "player_2"),
    ];

    for (function_name, restriction_type, player_id) in movement_conditions {
        let context = if player_id == "player_1" { &player1_context } else { &player2_context };
        let condition = ConditionConfig {
            condition_id: format!("check_{}_movement", restriction_type),
            function_name: function_name.to_string(),
            operator: ConditionOperator::Equal,
            value: ConditionValue::Boolean(true),
            parameters: vec![ConditionParameter::String(restriction_type.to_string())],
        };

        let result = resolver.resolve_condition(&condition, context).await?;
        println!("  {} {} on {}: {}", function_name, restriction_type, player_id, result);
    }

    // Example 6: Status Visual/Audio Effects
    println!("\n🎨 Example 6: Status Visual/Audio Effects");
    println!("{}", "-".repeat(40));

    let visual_audio_conditions = vec![
        ("has_status_visual_effect", "poison", "player_1"),
        ("has_status_visual_effect", "burn", "player_1"),
        ("has_status_audio_effect", "poison", "player_1"),
        ("has_status_audio_effect", "stun", "player_2"),
    ];

    for (function_name, effect_id, player_id) in visual_audio_conditions {
        let context = if player_id == "player_1" { &player1_context } else { &player2_context };
        let condition = ConditionConfig {
            condition_id: format!("check_{}_{}", function_name, effect_id),
            function_name: function_name.to_string(),
            operator: ConditionOperator::Equal,
            value: ConditionValue::Boolean(true),
            parameters: vec![ConditionParameter::String(effect_id.to_string())],
        };

        let result = resolver.resolve_condition(&condition, context).await?;
        println!("  {} {} on {}: {}", function_name, effect_id, player_id, result);
    }

    // Example 7: Status Properties
    println!("\n⚙️ Example 7: Status Properties");
    println!("{}", "-".repeat(40));

    let property_conditions = vec![
        ("has_status_effect_property", "poison", "player_1"),
        ("has_status_effect_property", "stun", "player_2"),
    ];

    for (function_name, effect_id, player_id) in property_conditions {
        let context = if player_id == "player_1" { &player1_context } else { &player2_context };
        let condition = ConditionConfig {
            condition_id: format!("check_{}_property", effect_id),
            function_name: function_name.to_string(),
            operator: ConditionOperator::Equal,
            value: ConditionValue::Boolean(true),
            parameters: vec![
                ConditionParameter::String(effect_id.to_string()),
                ConditionParameter::String("stackable".to_string()),
            ],
        };

        let result = resolver.resolve_condition(&condition, context).await?;
        println!("  {} {} stackable on {}: {}", function_name, effect_id, player_id, result);
    }

    // Example 8: Complex Status Condition Chain
    println!("\n🔗 Example 8: Complex Status Condition Chain");
    println!("{}", "-".repeat(40));

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

    let result = resolver.resolve_condition_chain(&chain_config, &player1_context).await?;
    println!("  Player 1 is poisoned and not immune: {}", result);

    let result = resolver.resolve_condition_chain(&chain_config, &player2_context).await?;
    println!("  Player 2 is poisoned and not immune: {}", result);

    // Example 9: Status Effect Interaction
    println!("\n⚔️ Example 9: Status Effect Interaction");
    println!("{}", "-".repeat(40));

    let interaction_condition = ConditionConfig {
        condition_id: "poison_burn_interaction".to_string(),
        function_name: "get_status_effect_interaction".to_string(),
        operator: ConditionOperator::Equal,
        value: ConditionValue::String("amplify".to_string()),
        parameters: vec![
            ConditionParameter::String("poison".to_string()),
            ConditionParameter::String("burn".to_string()),
        ],
    };

    let result = resolver.resolve_condition(&interaction_condition, &player1_context).await?;
    println!("  Poison and Burn interaction is amplify: {}", result);

    // Example 10: Performance Test
    println!("\n⚡ Example 10: Performance Test");
    println!("{}", "-".repeat(40));

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

        let _result = resolver.resolve_condition(&condition, &player1_context).await?;
    }

    let duration = start.elapsed();
    let avg_time = duration.as_nanos() as f64 / iterations as f64;
    let calls_per_second = 1_000_000_000.0 / avg_time;

    println!("  Processed {} status condition checks in {:?}", iterations, duration);
    println!("  Average time per check: {:.2} ns", avg_time);
    println!("  Calls per second: {:.0}", calls_per_second);

    println!("\n🎉 Status Conditions Example completed successfully!");
    println!("{}", "=".repeat(60));

    Ok(())
}
