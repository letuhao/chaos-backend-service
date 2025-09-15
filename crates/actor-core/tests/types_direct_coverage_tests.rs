//! Direct coverage tests for types.rs module.
//! This file provides direct tests that exercise the actual types module code.

use actor_core::types::*;
use actor_core::enums::{Bucket, CapMode};
use shared::types::GameEntity;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

// ============================================================================
// ACTOR TESTS
// ============================================================================

#[test]
fn test_actor_creation() {
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    assert_eq!(actor.name, "TestActor");
    assert_eq!(actor.race, "Human");
    assert_eq!(actor.lifespan, 0);
    assert_eq!(actor.age, 0);
    assert_eq!(actor.version, 1);
    assert!(actor.subsystems.is_empty());
    assert!(actor.data.is_empty());
}

#[test]
fn test_actor_is_valid() {
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    assert!(actor.is_valid());
    
    let mut invalid_actor = Actor::new("".to_string(), "Human".to_string());
    assert!(!invalid_actor.is_valid());
    
    invalid_actor = Actor::new("TestActor".to_string(), "".to_string());
    assert!(!invalid_actor.is_valid());
    
    invalid_actor = Actor::new("TestActor".to_string(), "Human".to_string());
    invalid_actor.version = 0;
    assert!(!invalid_actor.is_valid());
}

#[test]
fn test_actor_touch() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    let original_version = actor.version;
    let original_updated_at = actor.updated_at;
    
    actor.touch();
    
    assert_eq!(actor.version, original_version + 1);
    assert!(actor.updated_at > original_updated_at);
}

#[test]
fn test_actor_subsystem_management() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    let subsystem = Subsystem::new("combat".to_string(), 100);
    
    // Add subsystem
    actor.add_subsystem(subsystem);
    assert_eq!(actor.subsystems.len(), 1);
    assert!(actor.has_subsystem("combat"));
    
    // Get subsystem
    let retrieved = actor.get_subsystem("combat");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().system_id, "combat");
    
    // Remove subsystem
    let removed = actor.remove_subsystem("combat");
    assert!(removed);
    assert_eq!(actor.subsystems.len(), 0);
    assert!(!actor.has_subsystem("combat"));
    
    // Try to remove non-existent subsystem
    let not_removed = actor.remove_subsystem("nonexistent");
    assert!(!not_removed);
}

#[test]
fn test_actor_combat_status() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Initially not in combat
    assert!(!actor.is_in_combat());
    
    // Set in combat
    actor.set_in_combat(true);
    assert!(actor.is_in_combat());
    
    // Set not in combat
    actor.set_in_combat(false);
    assert!(!actor.is_in_combat());
}

#[test]
fn test_actor_buff_management() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Initially no buffs
    assert!(!actor.has_buffs());
    assert!(!actor.has_buff("strength_buff"));
    
    // Add buff
    actor.add_buff("strength_buff".to_string());
    assert!(actor.has_buffs());
    assert!(actor.has_buff("strength_buff"));
    
    // Add same buff again (should not duplicate)
    actor.add_buff("strength_buff".to_string());
    let buffs = actor.get_active_buffs();
    assert_eq!(buffs.len(), 1);
    
    // Add another buff
    actor.add_buff("speed_buff".to_string());
    let buffs = actor.get_active_buffs();
    assert_eq!(buffs.len(), 2);
    
    // Remove buff
    let removed = actor.remove_buff("strength_buff");
    assert!(removed);
    assert!(!actor.has_buff("strength_buff"));
    assert!(actor.has_buff("speed_buff"));
    
    // Clear all buffs
    actor.clear_buffs();
    assert!(!actor.has_buffs());
}

#[test]
fn test_actor_guild_management() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Initially no guild
    assert!(actor.get_guild_id().is_none());
    assert!(!actor.is_guild_member());
    
    // Set guild
    actor.set_guild_id("guild_123".to_string());
    assert_eq!(actor.get_guild_id(), Some("guild_123"));
    assert!(actor.is_guild_member());
}

#[test]
fn test_actor_online_status() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Default to online
    assert!(actor.is_online());
    
    // Set offline
    actor.set_online(false);
    assert!(!actor.is_online());
    
    // Set online
    actor.set_online(true);
    assert!(actor.is_online());
}

#[test]
fn test_actor_combat_duration() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Not in combat initially
    assert!(actor.get_combat_duration().is_none());
    
    // Set in combat
    actor.set_in_combat(true);
    let duration = actor.get_combat_duration();
    assert!(duration.is_some());
    assert!(duration.unwrap() >= 0);
    
    // Set combat duration manually
    actor.set_combat_duration(300); // 5 minutes
    let duration = actor.get_combat_duration();
    assert!(duration.is_some());
    assert!(duration.unwrap() >= 300);
}

#[test]
fn test_actor_getter_methods() {
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    assert_eq!(actor.get_id(), &actor.id);
    assert_eq!(actor.get_name(), "TestActor");
    assert_eq!(actor.get_race(), "Human");
    assert_eq!(actor.get_lifespan(), 0);
    assert_eq!(actor.get_age(), 0);
    assert_eq!(actor.get_version(), 1);
    assert_eq!(actor.get_subsystems().len(), 0);
    assert_eq!(actor.get_data().len(), 0);
}

#[test]
fn test_actor_setter_methods() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    let new_timestamp = Utc::now();
    
    actor.set_name("NewName".to_string());
    assert_eq!(actor.get_name(), "NewName");
    
    actor.set_race("Elf".to_string());
    assert_eq!(actor.get_race(), "Elf");
    
    actor.set_lifespan(1000);
    assert_eq!(actor.get_lifespan(), 1000);
    
    actor.set_age(100);
    assert_eq!(actor.get_age(), 100);
    
    actor.set_updated_at(new_timestamp);
    assert_eq!(actor.get_updated_at(), new_timestamp);
    
    actor.set_version(5);
    assert_eq!(actor.get_version(), 5);
    
    let subsystems = vec![Subsystem::new("test".to_string(), 50)];
    actor.set_subsystems(subsystems);
    assert_eq!(actor.get_subsystems().len(), 1);
    
    let mut data = HashMap::new();
    data.insert("key".to_string(), serde_json::Value::String("value".to_string()));
    actor.set_data(data);
    assert_eq!(actor.get_data().len(), 1);
}

#[test]
fn test_actor_subsystem_priority_sorting() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    let subsystem1 = Subsystem::new("low_priority".to_string(), 10);
    let subsystem2 = Subsystem::new("high_priority".to_string(), 100);
    let subsystem3 = Subsystem::new("medium_priority".to_string(), 50);
    
    actor.add_subsystem(subsystem1);
    actor.add_subsystem(subsystem2);
    actor.add_subsystem(subsystem3);
    
    let sorted = actor.get_subsystem_by_priority();
    assert_eq!(sorted.len(), 3);
    assert_eq!(sorted[0].system_id, "high_priority");
    assert_eq!(sorted[1].system_id, "medium_priority");
    assert_eq!(sorted[2].system_id, "low_priority");
}

#[test]
fn test_actor_subsystem_count() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    assert_eq!(actor.get_subsystem_count(), 0);
    
    actor.add_subsystem(Subsystem::new("combat".to_string(), 100));
    assert_eq!(actor.get_subsystem_count(), 1);
    
    actor.add_subsystem(Subsystem::new("magic".to_string(), 50));
    assert_eq!(actor.get_subsystem_count(), 2);
}

// ============================================================================
// SUBSYSTEM TESTS
// ============================================================================

#[test]
fn test_subsystem_creation() {
    let subsystem = Subsystem::new("combat".to_string(), 100);
    
    assert_eq!(subsystem.system_id, "combat");
    assert_eq!(subsystem.priority, 100);
    assert!(subsystem.enabled);
    assert!(subsystem.config.is_empty());
    assert!(subsystem.data.is_empty());
}

#[test]
fn test_subsystem_is_valid() {
    let valid_subsystem = Subsystem::new("combat".to_string(), 100);
    assert!(valid_subsystem.is_valid());
    
    let invalid_subsystem = Subsystem::new("".to_string(), 100);
    assert!(!invalid_subsystem.is_valid());
    
    let invalid_priority = Subsystem::new("combat".to_string(), -1);
    assert!(!invalid_priority.is_valid());
}

#[test]
fn test_subsystem_getter_methods() {
    let subsystem = Subsystem::new("combat".to_string(), 100);
    
    assert_eq!(subsystem.get_system_id(), "combat");
    assert_eq!(subsystem.get_priority(), 100);
    assert!(subsystem.is_enabled());
    assert_eq!(subsystem.get_config().len(), 0);
    assert_eq!(subsystem.get_data().len(), 0);
}

#[test]
fn test_subsystem_setter_methods() {
    let mut subsystem = Subsystem::new("combat".to_string(), 100);
    let mut config = HashMap::new();
    let mut data = HashMap::new();
    
    config.insert("key".to_string(), serde_json::Value::String("value".to_string()));
    data.insert("data_key".to_string(), serde_json::Value::Number(serde_json::Number::from(42)));
    
    subsystem.set_config(config);
    subsystem.set_data(data);
    subsystem.set_enabled(false);
    
    assert_eq!(subsystem.get_config().len(), 1);
    assert_eq!(subsystem.get_data().len(), 1);
    assert!(!subsystem.is_enabled());
}

// ============================================================================
// CONTRIBUTION TESTS
// ============================================================================

#[test]
fn test_contribution_creation() {
    let contribution = Contribution::new(
        "strength".to_string(),
        Bucket::Flat,
        10.0,
        "combat".to_string(),
    );
    
    assert_eq!(contribution.dimension, "strength");
    assert_eq!(contribution.bucket, Bucket::Flat);
    assert_eq!(contribution.value, 10.0);
    assert_eq!(contribution.system, "combat");
    assert!(contribution.priority.is_none());
    assert!(contribution.tags.is_none());
}

#[test]
fn test_contribution_is_valid() {
    let valid_contribution = Contribution::new(
        "strength".to_string(),
        Bucket::Flat,
        10.0,
        "combat".to_string(),
    );
    assert!(valid_contribution.is_valid());
    
    let invalid_dimension = Contribution::new(
        "".to_string(),
        Bucket::Flat,
        10.0,
        "combat".to_string(),
    );
    assert!(!invalid_dimension.is_valid());
    
    let invalid_system = Contribution::new(
        "strength".to_string(),
        Bucket::Flat,
        10.0,
        "".to_string(),
    );
    assert!(!invalid_system.is_valid());
    
    let invalid_value = Contribution::new(
        "strength".to_string(),
        Bucket::Flat,
        f64::NAN,
        "combat".to_string(),
    );
    assert!(!invalid_value.is_valid());
}

// ============================================================================
// CAP CONTRIBUTION TESTS
// ============================================================================

#[test]
fn test_cap_contribution_creation() {
    let cap_contribution = CapContribution::new(
        "combat".to_string(),
        "strength".to_string(),
        CapMode::HardMax,
        "max".to_string(),
        100.0,
    );
    
    assert_eq!(cap_contribution.system, "combat");
    assert_eq!(cap_contribution.dimension, "strength");
    assert_eq!(cap_contribution.mode, CapMode::HardMax);
    assert_eq!(cap_contribution.kind, "max");
    assert_eq!(cap_contribution.value, 100.0);
    assert!(cap_contribution.priority.is_none());
    assert!(cap_contribution.scope.is_none());
    assert!(cap_contribution.realm.is_none());
    assert!(cap_contribution.tags.is_none());
}

#[test]
fn test_cap_contribution_is_valid() {
    let valid_cap = CapContribution::new(
        "combat".to_string(),
        "strength".to_string(),
        CapMode::HardMax,
        "max".to_string(),
        100.0,
    );
    assert!(valid_cap.is_valid());
    
    let invalid_system = CapContribution::new(
        "".to_string(),
        "strength".to_string(),
        CapMode::HardMax,
        "max".to_string(),
        100.0,
    );
    assert!(!invalid_system.is_valid());
    
    let invalid_dimension = CapContribution::new(
        "combat".to_string(),
        "".to_string(),
        CapMode::HardMax,
        "max".to_string(),
        100.0,
    );
    assert!(!invalid_dimension.is_valid());
    
    let invalid_kind = CapContribution::new(
        "combat".to_string(),
        "strength".to_string(),
        CapMode::HardMax,
        "".to_string(),
        100.0,
    );
    assert!(!invalid_kind.is_valid());
    
    let invalid_value = CapContribution::new(
        "combat".to_string(),
        "strength".to_string(),
        CapMode::HardMax,
        "max".to_string(),
        f64::NAN,
    );
    assert!(!invalid_value.is_valid());
}

// ============================================================================
// SUBSYSTEM OUTPUT TESTS
// ============================================================================

#[test]
fn test_subsystem_output_creation() {
    let output = SubsystemOutput::new("combat".to_string());
    
    assert!(output.primary.is_empty());
    assert!(output.derived.is_empty());
    assert!(output.caps.is_empty());
    assert!(output.context.is_none());
    assert_eq!(output.meta.system, "combat");
}

#[test]
fn test_subsystem_output_add_contributions() {
    let mut output = SubsystemOutput::new("combat".to_string());
    
    let primary_contribution = Contribution::new(
        "strength".to_string(),
        Bucket::Flat,
        10.0,
        "combat".to_string(),
    );
    
    let derived_contribution = Contribution::new(
        "health".to_string(),
        Bucket::Mult,
        1.2,
        "combat".to_string(),
    );
    
    let cap_contribution = CapContribution::new(
        "combat".to_string(),
        "strength".to_string(),
        CapMode::HardMax,
        "max".to_string(),
        100.0,
    );
    
    output.add_primary(primary_contribution);
    output.add_derived(derived_contribution);
    output.add_cap(cap_contribution);
    
    assert_eq!(output.primary.len(), 1);
    assert_eq!(output.derived.len(), 1);
    assert_eq!(output.caps.len(), 1);
}

// ============================================================================
// SUBSYSTEM META TESTS
// ============================================================================

#[test]
fn test_subsystem_meta_creation() {
    let meta = SubsystemMeta::new("combat".to_string());
    
    assert_eq!(meta.system, "combat");
    assert!(meta.data.is_empty());
}

// ============================================================================
// MODIFIER PACK TESTS
// ============================================================================

#[test]
fn test_modifier_pack_creation() {
    let modifier_pack = ModifierPack::new();
    
    assert!(modifier_pack.additive_percent.is_empty());
    assert!(modifier_pack.multipliers.is_empty());
    assert!(modifier_pack.post_add.is_empty());
}

#[test]
fn test_modifier_pack_default() {
    let modifier_pack = ModifierPack::default();
    
    assert!(modifier_pack.additive_percent.is_empty());
    assert!(modifier_pack.multipliers.is_empty());
    assert!(modifier_pack.post_add.is_empty());
}

#[test]
fn test_modifier_pack_apply() {
    let mut modifier_pack = ModifierPack::new();
    
    // Add some modifiers
    modifier_pack.additive_percent.insert("strength".to_string(), 10.0);
    modifier_pack.multipliers.insert("strength".to_string(), 1.5);
    modifier_pack.post_add.insert("strength".to_string(), 5.0);
    
    let base_value = 100.0;
    let result = modifier_pack.apply("strength", base_value);
    
    // Expected: (100 + 100 * 0.1) * 1.5 + 5 = 110 * 1.5 + 5 = 165 + 5 = 170
    assert_eq!(result, 170.0);
    
    // Test with dimension that has no modifiers
    let result_no_modifiers = modifier_pack.apply("agility", base_value);
    assert_eq!(result_no_modifiers, base_value);
}

// ============================================================================
// SNAPSHOT TESTS
// ============================================================================

#[test]
fn test_snapshot_creation() {
    let actor_id = Uuid::new_v4();
    let snapshot = Snapshot::new(actor_id, 1);
    
    assert_eq!(snapshot.actor_id, actor_id);
    assert_eq!(snapshot.version, 1);
    assert!(snapshot.primary.is_empty());
    assert!(snapshot.derived.is_empty());
    assert!(snapshot.caps_used.is_empty());
    assert!(snapshot.subsystems_processed.is_empty());
    assert!(snapshot.processing_time.is_none());
    assert!(snapshot.metadata.is_empty());
}

#[test]
fn test_snapshot_is_valid() {
    let actor_id = Uuid::new_v4();
    let valid_snapshot = Snapshot::new(actor_id, 1);
    assert!(valid_snapshot.is_valid());
    
    let invalid_snapshot = Snapshot::new(actor_id, 0);
    assert!(!invalid_snapshot.is_valid());
}

#[test]
fn test_snapshot_get_values() {
    let actor_id = Uuid::new_v4();
    let mut snapshot = Snapshot::new(actor_id, 1);
    
    // Add some values
    snapshot.primary.insert("strength".to_string(), 100.0);
    snapshot.derived.insert("health".to_string(), 200.0);
    
    assert_eq!(snapshot.get_primary("strength"), Some(100.0));
    assert_eq!(snapshot.get_primary("agility"), None);
    assert_eq!(snapshot.get_derived("health"), Some(200.0));
    assert_eq!(snapshot.get_derived("mana"), None);
}

#[test]
fn test_snapshot_clone_with_new_timestamp() {
    let actor_id = Uuid::new_v4();
    let snapshot = Snapshot::new(actor_id, 1);
    let original_timestamp = snapshot.created_at;
    
    let cloned = snapshot.clone_with_new_timestamp();
    
    assert_eq!(cloned.actor_id, snapshot.actor_id);
    assert_eq!(cloned.version, snapshot.version);
    assert!(cloned.created_at > original_timestamp);
}

// ============================================================================
// CAPS TESTS
// ============================================================================

#[test]
fn test_caps_creation() {
    let caps = Caps::new(0.0, 100.0);
    
    assert_eq!(caps.min, 0.0);
    assert_eq!(caps.max, 100.0);
}

#[test]
fn test_caps_is_valid() {
    let valid_caps = Caps::new(0.0, 100.0);
    assert!(valid_caps.is_valid());
    
    let invalid_caps = Caps::new(100.0, 0.0);
    assert!(!invalid_caps.is_valid());
    
    let nan_caps = Caps::new(f64::NAN, 100.0);
    assert!(!nan_caps.is_valid());
}

#[test]
fn test_caps_clamp() {
    let caps = Caps::new(10.0, 90.0);
    
    assert_eq!(caps.clamp(5.0), 10.0);
    assert_eq!(caps.clamp(50.0), 50.0);
    assert_eq!(caps.clamp(100.0), 90.0);
}

#[test]
fn test_caps_intersection() {
    let caps1 = Caps::new(0.0, 100.0);
    let caps2 = Caps::new(50.0, 150.0);
    let intersection = caps1.intersection(&caps2);
    
    assert_eq!(intersection.min, 50.0);
    assert_eq!(intersection.max, 100.0);
}

#[test]
fn test_caps_union() {
    let caps1 = Caps::new(0.0, 100.0);
    let caps2 = Caps::new(50.0, 150.0);
    let union = caps1.union(&caps2);
    
    assert_eq!(union.min, 0.0);
    assert_eq!(union.max, 150.0);
}

#[test]
fn test_caps_contains() {
    let caps = Caps::new(10.0, 90.0);
    
    assert!(!caps.contains(5.0));
    assert!(caps.contains(50.0));
    assert!(!caps.contains(100.0));
}

#[test]
fn test_caps_is_empty() {
    let empty_caps = Caps::new(100.0, 0.0);
    assert!(empty_caps.is_empty());
    
    let non_empty_caps = Caps::new(0.0, 100.0);
    assert!(!non_empty_caps.is_empty());
}

#[test]
fn test_caps_get_range() {
    let caps = Caps::new(10.0, 90.0);
    assert_eq!(caps.get_range(), 80.0);
}

#[test]
fn test_caps_get_center() {
    let caps = Caps::new(10.0, 90.0);
    assert_eq!(caps.get_center(), 50.0);
}

#[test]
fn test_caps_expand() {
    let mut caps = Caps::new(50.0, 50.0);
    caps.expand(10.0);
    
    assert_eq!(caps.min, 40.0);
    assert_eq!(caps.max, 60.0);
}

#[test]
fn test_caps_shrink() {
    let mut caps = Caps::new(0.0, 100.0);
    caps.shrink(20.0);
    
    assert_eq!(caps.min, 20.0);
    assert_eq!(caps.max, 80.0);
}

#[test]
fn test_caps_shrink_to_empty() {
    let mut caps = Caps::new(0.0, 100.0);
    caps.shrink(60.0); // This would make min > max
    
    // Should be adjusted to center point
    assert_eq!(caps.min, 50.0);
    assert_eq!(caps.max, 50.0);
}

#[test]
fn test_caps_set() {
    let mut caps = Caps::new(0.0, 100.0);
    caps.set(20.0, 80.0);
    
    assert_eq!(caps.min, 20.0);
    assert_eq!(caps.max, 80.0);
}

#[test]
fn test_caps_getter_setter_methods() {
    let mut caps = Caps::new(0.0, 100.0);
    
    assert_eq!(caps.get_min(), 0.0);
    assert_eq!(caps.get_max(), 100.0);
    
    caps.set_min(10.0);
    caps.set_max(90.0);
    
    assert_eq!(caps.get_min(), 10.0);
    assert_eq!(caps.get_max(), 90.0);
}

// ============================================================================
// SERIALIZATION TESTS
// ============================================================================

#[test]
fn test_actor_serialization() {
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    let json = serde_json::to_string(&actor).unwrap();
    assert!(!json.is_empty());
    
    let deserialized: Actor = serde_json::from_str(&json).unwrap();
    assert_eq!(actor.name, deserialized.name);
    assert_eq!(actor.race, deserialized.race);
}

#[test]
fn test_contribution_serialization() {
    let contribution = Contribution::new(
        "strength".to_string(),
        Bucket::Flat,
        10.0,
        "combat".to_string(),
    );
    let json = serde_json::to_string(&contribution).unwrap();
    assert!(!json.is_empty());
    
    let deserialized: Contribution = serde_json::from_str(&json).unwrap();
    assert_eq!(contribution.dimension, deserialized.dimension);
    assert_eq!(contribution.value, deserialized.value);
}

#[test]
fn test_caps_serialization() {
    let caps = Caps::new(0.0, 100.0);
    let json = serde_json::to_string(&caps).unwrap();
    assert!(!json.is_empty());
    
    let deserialized: Caps = serde_json::from_str(&json).unwrap();
    assert_eq!(caps.min, deserialized.min);
    assert_eq!(caps.max, deserialized.max);
}

// ============================================================================
// GAME ENTITY IMPLEMENTATION TESTS
// ============================================================================

#[test]
fn test_actor_game_entity_implementation() {
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    assert_eq!(actor.id(), actor.id);
    assert_eq!(actor.version(), actor.version);
    assert_eq!(actor.created_at(), actor.created_at);
    assert_eq!(actor.updated_at(), actor.updated_at);
}
