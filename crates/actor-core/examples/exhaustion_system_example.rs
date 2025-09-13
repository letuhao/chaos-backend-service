//! Resource Exhaustion System Example
//!
//! This example demonstrates how to use the Resource Exhaustion System
//! to handle resource-based debuffs and effects in a game.

use actor_core::subsystems::{
    ResourceExhaustionSubsystem, ExhaustionConfig, InMemoryEventPublisher
};
use actor_core::types::{Actor, Snapshot};
use std::collections::HashMap;
use std::sync::Arc;

/// Create a sample exhaustion configuration
fn create_sample_config() -> ExhaustionConfig {
    let mut archetypes = HashMap::new();
    
    // Mage archetype with mana-based exhaustion
    let mut mage_resources = HashMap::new();
    mage_resources.insert("mana".to_string(), actor_core::subsystems::resource_exhaustion::ResourceConfig {
        thresholds: vec![
            // Low mana threshold (10% or less)
            actor_core::subsystems::resource_exhaustion::ThresholdConfig {
                id: "low_mana".to_string(),
                order: Some(10),
                enter_percent_lte: Some(0.10),
                exit_percent_gte: Some(0.12),
                enter_value_eq: None,
                exit_value_ge: None,
                effects: vec![
                    // Disable shield and buff activation
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
                    // Reduce magical damage output
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
                    // Increase incoming magical damage
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "incoming_multiplier".to_string(),
                        categories: Some(vec!["magical".to_string(), "elemental".to_string()]),
                        modifier: Some(0.25),
                        values: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    },
                    // Increase cast time
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "cast_time_modifier".to_string(),
                        modifier: Some(0.30),
                        values: None,
                        categories: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    },
                ],
            },
            // No mana threshold (exactly 0)
            actor_core::subsystems::resource_exhaustion::ThresholdConfig {
                id: "no_mana".to_string(),
                order: Some(20),
                enter_percent_lte: None,
                exit_percent_gte: None,
                enter_value_eq: Some(0.0),
                exit_value_ge: Some(1.0),
                effects: vec![
                    // Disable mana-based actions
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "disable_cost_type".to_string(),
                        values: Some(vec!["mana".to_string()]),
                        categories: None,
                        modifier: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    },
                    // Break active mana-based shields
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "break_active_shields".to_string(),
                        values: Some(vec!["MagicShield".to_string(), "ElementalShield".to_string(), "SpiritualShield".to_string()]),
                        categories: None,
                        modifier: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    },
                    // Set vulnerable flag
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
            },
        ],
    });
    
    archetypes.insert("mage".to_string(), actor_core::subsystems::resource_exhaustion::ArchetypeConfig {
        resources: mage_resources,
    });
    
    // Warrior archetype with stamina-based exhaustion
    let mut warrior_resources = HashMap::new();
    warrior_resources.insert("stamina".to_string(), actor_core::subsystems::resource_exhaustion::ResourceConfig {
        thresholds: vec![
            // Low stamina threshold (15% or less)
            actor_core::subsystems::resource_exhaustion::ThresholdConfig {
                id: "low_stamina".to_string(),
                order: Some(10),
                enter_percent_lte: Some(0.15),
                exit_percent_gte: Some(0.17),
                enter_value_eq: None,
                exit_value_ge: None,
                effects: vec![
                    // Disable stamina-based actions
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "disable_cost_type".to_string(),
                        values: Some(vec!["stamina".to_string()]),
                        categories: None,
                        modifier: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    },
                    // Reduce physical damage output
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "damage_multiplier".to_string(),
                        categories: Some(vec!["physical".to_string()]),
                        modifier: Some(-0.30),
                        values: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    },
                    // Reduce movement speed
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "move_speed_modifier".to_string(),
                        modifier: Some(-0.20),
                        values: None,
                        categories: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    },
                    // Disable defensive abilities
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "disable_tags".to_string(),
                        values: Some(vec!["parry".to_string(), "block".to_string()]),
                        categories: None,
                        modifier: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    },
                ],
            },
            // No stamina threshold (exactly 0)
            actor_core::subsystems::resource_exhaustion::ThresholdConfig {
                id: "no_stamina".to_string(),
                order: Some(20),
                enter_percent_lte: None,
                exit_percent_gte: None,
                enter_value_eq: Some(0.0),
                exit_value_ge: Some(1.0),
                effects: vec![
                    // Lock out physical actions
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "action_lockout".to_string(),
                        values: Some(vec!["physical".to_string()]),
                        categories: None,
                        modifier: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    },
                    // Heavy stagger susceptibility
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "stagger_susceptibility".to_string(),
                        level: Some("heavy".to_string()),
                        values: None,
                        categories: None,
                        modifier: None,
                        name: None,
                        value: None,
                        resource: None,
                    },
                    // Reduce taunt effectiveness
                    actor_core::subsystems::resource_exhaustion::EffectConfig {
                        effect_type: "taunt_effectiveness_modifier".to_string(),
                        modifier: Some(-0.50),
                        values: None,
                        categories: None,
                        name: None,
                        value: None,
                        level: None,
                        resource: None,
                    },
                ],
            },
        ],
    });
    
    archetypes.insert("warrior".to_string(), actor_core::subsystems::resource_exhaustion::ArchetypeConfig {
        resources: warrior_resources,
    });

    ExhaustionConfig {
        version: 1,
        hysteresis_default: 0.02,
        events: actor_core::subsystems::resource_exhaustion::EventConfig {
            coalesce_window_ms: 200,
        },
        priorities: Some(actor_core::subsystems::resource_exhaustion::PriorityConfig {
            categories: vec![
                "health".to_string(),
                "lifeforce".to_string(),
                "lifespan".to_string(),
                "qi".to_string(),
                "spiritual".to_string(),
                "mana".to_string(),
                "stamina".to_string(),
                "other".to_string(),
            ],
        }),
        archetypes,
    }
}

/// Create a test actor with specific archetype and resources
fn create_test_actor(archetype: &str, mana_current: f64, mana_max: f64, stamina_current: f64, stamina_max: f64) -> (Actor, Snapshot) {
    let mut actor = Actor::new(format!("actor_{}_001", archetype), "Human".to_string());
    let mut data = HashMap::new();
    data.insert("archetype".to_string(), serde_json::Value::String(archetype.to_string()));
    actor.set_data(data);
    
    let mut snapshot = Snapshot::new(actor.id, actor.version);
    snapshot.primary.insert("mana_current".to_string(), mana_current);
    snapshot.primary.insert("mana_max".to_string(), mana_max);
    snapshot.primary.insert("stamina_current".to_string(), stamina_current);
    snapshot.primary.insert("stamina_max".to_string(), stamina_max);
    
    (actor, snapshot)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    #[cfg(feature = "cli-tools")]
    tracing_subscriber::fmt::init();
    
    println!("Resource Exhaustion System Example");
    println!("==================================");
    
    // Create configuration
    let config = create_sample_config();
    
    // Create event publisher
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    
    // Create exhaustion subsystem
    let exhaustion_subsystem = ResourceExhaustionSubsystem::new(config, event_publisher);
    
    // Test scenarios
    println!("\n1. Testing Mage with Low Mana (5%)");
    let (mage_actor, mage_snapshot) = create_test_actor("mage", 50.0, 1000.0, 500.0, 1000.0);
    let transitions = exhaustion_subsystem.evaluate(&mage_actor, &mage_snapshot).await?;
    println!("Found {} transitions", transitions.len());
    for transition in &transitions {
        println!("  - {} {}: {} effects", 
            if transition.entering { "Entering" } else { "Exiting" },
            transition.threshold_id,
            transition.effects.len()
        );
    }
    exhaustion_subsystem.apply_effects(&mage_actor.id.to_string(), &transitions).await?;
    
    println!("\n2. Testing Mage with No Mana (0%)");
    let (mage_actor, mage_snapshot) = create_test_actor("mage", 0.0, 1000.0, 500.0, 1000.0);
    let transitions = exhaustion_subsystem.evaluate(&mage_actor, &mage_snapshot).await?;
    println!("Found {} transitions", transitions.len());
    for transition in &transitions {
        println!("  - {} {}: {} effects", 
            if transition.entering { "Entering" } else { "Exiting" },
            transition.threshold_id,
            transition.effects.len()
        );
    }
    exhaustion_subsystem.apply_effects(&mage_actor.id.to_string(), &transitions).await?;
    
    println!("\n3. Testing Warrior with Low Stamina (10%)");
    let (warrior_actor, warrior_snapshot) = create_test_actor("warrior", 500.0, 1000.0, 100.0, 1000.0);
    let transitions = exhaustion_subsystem.evaluate(&warrior_actor, &warrior_snapshot).await?;
    println!("Found {} transitions", transitions.len());
    for transition in &transitions {
        println!("  - {} {}: {} effects", 
            if transition.entering { "Entering" } else { "Exiting" },
            transition.threshold_id,
            transition.effects.len()
        );
    }
    exhaustion_subsystem.apply_effects(&warrior_actor.id.to_string(), &transitions).await?;
    
    println!("\n4. Testing Recovery (Mage with 15% mana)");
    let (mage_actor, mage_snapshot) = create_test_actor("mage", 150.0, 1000.0, 500.0, 1000.0);
    let transitions = exhaustion_subsystem.evaluate(&mage_actor, &mage_snapshot).await?;
    println!("Found {} transitions", transitions.len());
    for transition in &transitions {
        println!("  - {} {}: {} effects", 
            if transition.entering { "Entering" } else { "Exiting" },
            transition.threshold_id,
            transition.effects.len()
        );
    }
    exhaustion_subsystem.apply_effects(&mage_actor.id.to_string(), &transitions).await?;
    
    println!("\n5. Testing Hysteresis (Mage bouncing around 10%)");
    let scenarios = vec![
        (50.0, "5% - should trigger low_mana"),
        (80.0, "8% - should stay in low_mana (hysteresis)"),
        (60.0, "6% - should stay in low_mana (hysteresis)"),
        (120.0, "12% - should exit low_mana"),
        (100.0, "10% - should stay out (hysteresis)"),
        (90.0, "9% - should re-enter low_mana"),
    ];
    
    for (mana_percent, description) in scenarios {
        let mana_current = mana_percent * 10.0; // 1000 max
        let (actor, snapshot) = create_test_actor("mage", mana_current, 1000.0, 500.0, 1000.0);
        let transitions = exhaustion_subsystem.evaluate(&actor, &snapshot).await?;
        println!("  {}: {} transitions", description, transitions.len());
        for transition in &transitions {
            println!("    - {} {}", 
                if transition.entering { "Entering" } else { "Exiting" },
                transition.threshold_id
            );
        }
    }
    
    println!("\nExample completed successfully!");
    Ok(())
}
