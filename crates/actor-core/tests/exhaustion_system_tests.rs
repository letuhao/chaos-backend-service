//! Tests for the Resource Exhaustion System

use actor_core::subsystems::{
    ResourceExhaustionSubsystem, ExhaustionConfig, ExhaustionEventType,
    InMemoryEventPublisher, LoggingEventPublisher, NoOpEventPublisher,
    ExhaustionEventPublisher
};
use actor_core::interfaces::Subsystem;
use actor_core::types::{Actor, Snapshot};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Create a test actor with specific archetype
fn create_test_actor(archetype: &str) -> Actor {
    let mut actor = Actor::new("test_actor".to_string(), "Human".to_string());
    let mut data = HashMap::new();
    data.insert("archetype".to_string(), serde_json::Value::String(archetype.to_string()));
    actor.set_data(data);
    actor
}

/// Create a test snapshot with specific resource values
fn create_test_snapshot(mana_current: f64, mana_max: f64, stamina_current: f64, stamina_max: f64) -> Snapshot {
    let mut snapshot = Snapshot::new(Uuid::new_v4(), 1);
    snapshot.primary.insert("mana_current".to_string(), mana_current);
    snapshot.primary.insert("mana_max".to_string(), mana_max);
    snapshot.primary.insert("stamina_current".to_string(), stamina_current);
    snapshot.primary.insert("stamina_max".to_string(), stamina_max);
    snapshot
}

/// Create a test exhaustion config
fn create_test_config() -> ExhaustionConfig {
    let mut archetypes = HashMap::new();
    
    // Mage archetype
    let mut mage_resources = HashMap::new();
    mage_resources.insert("mana".to_string(), actor_core::subsystems::resource_exhaustion::ResourceConfig {
        thresholds: vec![
            actor_core::subsystems::resource_exhaustion::ThresholdConfig {
                id: "low_mana".to_string(),
                order: Some(10),
                enter_percent_lte: Some(0.10),
                exit_percent_gte: Some(0.12),
                enter_value_eq: None,
                exit_value_ge: None,
                effects: vec![
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "disable_tags".to_string(),
                        values: Some(vec!["shield_activation".to_string()]),
                        categories: None,
                        modifier: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    }
                ],
            },
            actor_core::subsystems::resource_exhaustion::ThresholdConfig {
                id: "no_mana".to_string(),
                order: Some(20),
                enter_percent_lte: None,
                exit_percent_gte: None,
                enter_value_eq: Some(0.0),
                exit_value_ge: Some(1.0),
                effects: vec![
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "disable_cost_type".to_string(),
                        values: Some(vec!["mana".to_string()]),
                        categories: None,
                        modifier: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    }
                ],
            },
        ],
    });
    
    archetypes.insert("mage".to_string(), actor_core::subsystems::resource_exhaustion::ArchetypeConfig {
        resources: mage_resources,
    });

    ExhaustionConfig {
        version: 1,
        hysteresis_default: 0.02,
        events: actor_core::subsystems::resource_exhaustion::EventConfig {
            coalesce_window_ms: 200,
        },
        priorities: None,
        archetypes,
    }
}

#[tokio::test]
async fn test_exhaustion_subsystem_creation() {
    let config = create_test_config();
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    
    let subsystem = ResourceExhaustionSubsystem::new(config, event_publisher);
    
    assert_eq!(subsystem.system_id(), "resource_exhaustion");
    assert_eq!(subsystem.priority(), 200);
}

#[tokio::test]
async fn test_exhaustion_evaluation_low_mana() {
    let config = create_test_config();
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    let subsystem = ResourceExhaustionSubsystem::new(config, event_publisher.clone());
    
    let actor = create_test_actor("mage");
    let snapshot = create_test_snapshot(50.0, 1000.0, 500.0, 1000.0); // 5% mana
    
    let transitions = subsystem.evaluate(&actor, &snapshot).await.unwrap();
    
    // Should trigger low_mana threshold (5% <= 10%)
    assert_eq!(transitions.len(), 1);
    assert_eq!(transitions[0].resource, "mana");
    assert_eq!(transitions[0].threshold_id, "low_mana");
    assert!(transitions[0].entering);
    assert_eq!(transitions[0].effects.len(), 1);
    assert_eq!(transitions[0].effects[0].effect_type, "disable_tags");
}

#[tokio::test]
async fn test_exhaustion_evaluation_no_mana() {
    let config = create_test_config();
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    let subsystem = ResourceExhaustionSubsystem::new(config, event_publisher.clone());
    
    let actor = create_test_actor("mage");
    let snapshot = create_test_snapshot(0.0, 1000.0, 500.0, 1000.0); // 0 mana
    
    let transitions = subsystem.evaluate(&actor, &snapshot).await.unwrap();
    
    // Should trigger both low_mana and no_mana thresholds
    assert_eq!(transitions.len(), 2);
    
    let low_mana_transition = transitions.iter().find(|t| t.threshold_id == "low_mana").unwrap();
    assert!(low_mana_transition.entering);
    
    let no_mana_transition = transitions.iter().find(|t| t.threshold_id == "no_mana").unwrap();
    assert!(no_mana_transition.entering);
}

#[tokio::test]
async fn test_exhaustion_evaluation_recovery() {
    let config = create_test_config();
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    let subsystem = ResourceExhaustionSubsystem::new(config, event_publisher.clone());
    
    let actor = create_test_actor("mage");
    let snapshot = create_test_snapshot(150.0, 1000.0, 500.0, 1000.0); // 15% mana
    
    let transitions = subsystem.evaluate(&actor, &snapshot).await.unwrap();
    
    // Should not trigger any thresholds (15% > 12% exit threshold)
    assert_eq!(transitions.len(), 0);
}

#[tokio::test]
async fn test_exhaustion_effect_application() {
    let config = create_test_config();
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    let subsystem = ResourceExhaustionSubsystem::new(config, event_publisher.clone());
    
    let actor = create_test_actor("mage");
    let snapshot = create_test_snapshot(50.0, 1000.0, 500.0, 1000.0);
    
    let transitions = subsystem.evaluate(&actor, &snapshot).await.unwrap();
    subsystem.apply_effects(&actor.id.to_string(), &transitions).await.unwrap();
    
    // Check that events were published
    let events = event_publisher.get_events().await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event_type, ExhaustionEventType::ResourceExhausted);
    assert_eq!(events[0].actor_id, actor.id.to_string());
    assert_eq!(events[0].resource_type, "mana");
    assert_eq!(events[0].threshold_id, "low_mana");
}

#[tokio::test]
async fn test_exhaustion_hysteresis() {
    let config = create_test_config();
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    let subsystem = ResourceExhaustionSubsystem::new(config, event_publisher.clone());
    
    let actor = create_test_actor("mage");
    
    // Test entering threshold (5% <= 10%)
    let snapshot1 = create_test_snapshot(50.0, 1000.0, 500.0, 1000.0);
    let transitions1 = subsystem.evaluate(&actor, &snapshot1).await.unwrap();
    assert_eq!(transitions1.len(), 1);
    assert!(transitions1[0].entering);
    
    // Test staying in threshold (8% - still <= 10%)
    let snapshot2 = create_test_snapshot(80.0, 1000.0, 500.0, 1000.0);
    let transitions2 = subsystem.evaluate(&actor, &snapshot2).await.unwrap();
    assert_eq!(transitions2.len(), 0); // No new transitions
    
    // Test exiting threshold (15% >= 12%)
    let snapshot3 = create_test_snapshot(150.0, 1000.0, 500.0, 1000.0);
    let transitions3 = subsystem.evaluate(&actor, &snapshot3).await.unwrap();
    assert_eq!(transitions3.len(), 1);
    assert!(!transitions3[0].entering); // Exiting
}

#[tokio::test]
async fn test_exhaustion_config_validation() {
    // Test valid config
    let config = create_test_config();
    assert!(ResourceExhaustionSubsystem::validate_config(&config).is_ok());
    
    // Test invalid version
    let mut invalid_config = config.clone();
    invalid_config.version = 0;
    assert!(ResourceExhaustionSubsystem::validate_config(&invalid_config).is_err());
    
    // Test invalid hysteresis
    let mut invalid_config = config.clone();
    invalid_config.hysteresis_default = 1.5;
    assert!(ResourceExhaustionSubsystem::validate_config(&invalid_config).is_err());
}

#[tokio::test]
async fn test_exhaustion_event_publishers() {
    // Test in-memory publisher
    let in_memory_publisher = InMemoryEventPublisher::new();
    let logging_publisher = LoggingEventPublisher;
    let no_op_publisher = NoOpEventPublisher;
    
    let event = actor_core::subsystems::resource_exhaustion::ExhaustionEvent {
        event_type: ExhaustionEventType::ResourceExhausted,
        actor_id: "test_actor".to_string(),
        resource_type: "mana".to_string(),
        threshold_id: "low_mana".to_string(),
        effects: vec![],
        timestamp: chrono::Utc::now(),
        idempotency_key: "test_key".to_string(),
        coalesced: false,
    };
    
    // All publishers should work without panicking
    in_memory_publisher.publish_event(event.clone()).await.unwrap();
    logging_publisher.publish_event(event.clone()).await.unwrap();
    no_op_publisher.publish_event(event).await.unwrap();
    
    // Check in-memory publisher stored the event
    let events = in_memory_publisher.get_events().await;
    assert_eq!(events.len(), 1);
}

#[tokio::test]
async fn test_exhaustion_unknown_archetype() {
    let config = create_test_config();
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    let subsystem = ResourceExhaustionSubsystem::new(config, event_publisher);
    
    let actor = create_test_actor("unknown_archetype");
    let snapshot = create_test_snapshot(50.0, 1000.0, 500.0, 1000.0);
    
    let result = subsystem.evaluate(&actor, &snapshot).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unknown archetype"));
}

#[tokio::test]
async fn test_exhaustion_deterministic_ordering() {
    let config = create_test_config();
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    let subsystem = ResourceExhaustionSubsystem::new(config, event_publisher);
    
    let actor = create_test_actor("mage");
    let snapshot = create_test_snapshot(0.0, 1000.0, 500.0, 1000.0); // 0 mana
    
    let transitions = subsystem.evaluate(&actor, &snapshot).await.unwrap();
    
    // Should have both transitions in deterministic order
    assert_eq!(transitions.len(), 2);
    assert_eq!(transitions[0].threshold_id, "low_mana"); // order: 10
    assert_eq!(transitions[1].threshold_id, "no_mana");  // order: 20
}

#[tokio::test]
async fn test_exhaustion_idempotency_keys() {
    let config = create_test_config();
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    let subsystem = ResourceExhaustionSubsystem::new(config, event_publisher.clone());
    
    let actor = create_test_actor("mage");
    let snapshot = create_test_snapshot(50.0, 1000.0, 500.0, 1000.0);
    
    let transitions = subsystem.evaluate(&actor, &snapshot).await.unwrap();
    subsystem.apply_effects(&actor.id.to_string(), &transitions).await.unwrap();
    
    let events = event_publisher.get_events().await;
    assert_eq!(events.len(), 1);
    
    // Check that idempotency key is generated
    assert!(!events[0].idempotency_key.is_empty());
    assert!(events[0].idempotency_key.starts_with("exhaustion_"));
}

#[tokio::test]
async fn test_exhaustion_effect_types() {
    let mut config = create_test_config();
    
    // Add a threshold with various effect types
    let mut mage_resources = config.archetypes.get_mut("mage").unwrap().resources.clone();
    let mut mana_config = mage_resources.get("mana").unwrap().clone();
    
    mana_config.thresholds.push(actor_core::subsystems::resource_exhaustion::ThresholdConfig {
        id: "test_effects".to_string(),
        order: Some(30),
        enter_percent_lte: Some(0.05),
        exit_percent_gte: Some(0.07),
        enter_value_eq: None,
        exit_value_ge: None,
        effects: vec![
            // disable_tags
            actor_core::subsystems::resource_exhaustion::EffectConfig {
                effect_type: "disable_tags".to_string(),
                values: Some(vec!["shield_activation".to_string(), "buff_activation".to_string()]),
                categories: None,
                modifier: None,
                name: None,
                value: None,
                level: None,
                resource: None,
            },
            // damage_multiplier
            actor_core::subsystems::resource_exhaustion::EffectConfig {
                effect_type: "damage_multiplier".to_string(),
                categories: Some(vec!["magical".to_string(), "elemental".to_string()]),
                modifier: Some(-0.40),
                values: None,
                name: None,
                value: None,
                level: None,
                resource: None,
            },
            // set_flag
            actor_core::subsystems::resource_exhaustion::EffectConfig {
                effect_type: "set_flag".to_string(),
                name: Some("vulnerable_mage".to_string()),
                value: Some(true),
                values: None,
                categories: None,
                modifier: None,
                level: None,
                resource: None,
            },
        ],
    });
    
    mage_resources.insert("mana".to_string(), mana_config);
    config.archetypes.get_mut("mage").unwrap().resources = mage_resources;
    
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    let subsystem = ResourceExhaustionSubsystem::new(config, event_publisher.clone());
    
    let actor = create_test_actor("mage");
    let snapshot = create_test_snapshot(30.0, 1000.0, 500.0, 1000.0); // 3% mana
    
    let transitions = subsystem.evaluate(&actor, &snapshot).await.unwrap();
    
    // Should trigger the test_effects threshold
    let test_effects_transition = transitions.iter().find(|t| t.threshold_id == "test_effects");
    assert!(test_effects_transition.is_some());
    
    let transition = test_effects_transition.unwrap();
    assert_eq!(transition.effects.len(), 3);
    
    // Check effect types
    let effect_types: Vec<&str> = transition.effects.iter().map(|e| e.effect_type.as_str()).collect();
    assert!(effect_types.contains(&"disable_tags"));
    assert!(effect_types.contains(&"damage_multiplier"));
    assert!(effect_types.contains(&"set_flag"));
}
