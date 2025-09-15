//! Simple coverage tests for constants.rs module.

use actor_core::constants::*;

#[tokio::test]
async fn test_system_ids_constants() {
    // Test all system ID constants
    assert_eq!(system_ids::LUYEN_THE, "luyen_the");
    assert_eq!(system_ids::KIM_DAN, "kim_dan");
    assert_eq!(system_ids::COMBAT, "combat");
    assert_eq!(system_ids::EQUIPMENT, "equipment");
    assert_eq!(system_ids::BUFF, "buff");
    assert_eq!(system_ids::GUILD, "guild");
    assert_eq!(system_ids::EVENT, "event");
    assert_eq!(system_ids::WORLD, "world");
    assert_eq!(system_ids::MAGIC, "magic");
    assert_eq!(system_ids::CULTIVATION, "cultivation");
    assert_eq!(system_ids::EXPERIENCE, "experience");
    assert_eq!(system_ids::REPUTATION, "reputation");
    assert_eq!(system_ids::TRADING, "trading");
    assert_eq!(system_ids::WEATHER, "weather");
    assert_eq!(system_ids::LOCATION, "location");
    assert_eq!(system_ids::TIME, "time");
    assert_eq!(system_ids::STEALTH, "stealth");
    assert_eq!(system_ids::PERCEPTION, "perception");
}

#[tokio::test]
async fn test_primary_dimensions_constants() {
    // Test primary dimension constants
    assert_eq!(primary_dimensions::STRENGTH, "strength");
    assert_eq!(primary_dimensions::AGILITY, "agility");
    assert_eq!(primary_dimensions::INTELLIGENCE, "intelligence");
}

#[tokio::test]
async fn test_derived_dimensions_constants() {
    // Test derived dimension constants
    assert_eq!(derived_dimensions::ATTACK_POWER, "attack_power");
    assert_eq!(derived_dimensions::DEFENSE_POWER, "defense_power");
    assert_eq!(derived_dimensions::CRITICAL_HIT_CHANCE, "critical_hit_chance");
    assert_eq!(derived_dimensions::CRITICAL_HIT_DAMAGE, "critical_hit_damage");
    assert_eq!(derived_dimensions::ATTACK_SPEED, "attack_speed");
    assert_eq!(derived_dimensions::MOVEMENT_SPEED, "movement_speed");
    assert_eq!(derived_dimensions::CASTING_SPEED, "casting_speed");
    assert_eq!(derived_dimensions::COOLDOWN_REDUCTION, "cooldown_reduction");
    assert_eq!(derived_dimensions::LIFE_STEAL, "life_steal");
    assert_eq!(derived_dimensions::MANA_STEAL, "mana_steal");
    assert_eq!(derived_dimensions::DAMAGE_REDUCTION, "damage_reduction");
    assert_eq!(derived_dimensions::ELEMENTAL_RESISTANCE, "elemental_resistance");
}

#[tokio::test]
async fn test_error_codes_constants() {
    // Test error code constants
    assert_eq!(error_codes::INVALID_ACTOR, "INVALID_ACTOR");
    assert_eq!(error_codes::INVALID_CONTRIBUTION, "INVALID_CONTRIBUTION");
    assert_eq!(error_codes::INVALID_CAP, "INVALID_CAP");
    assert_eq!(error_codes::SUBSYSTEM_ERROR, "SUBSYSTEM_ERROR");
    assert_eq!(error_codes::CACHE_ERROR, "CACHE_ERROR");
    assert_eq!(error_codes::REGISTRY_ERROR, "REGISTRY_ERROR");
    assert_eq!(error_codes::AGGREGATION_ERROR, "AGGREGATION_ERROR");
    assert_eq!(error_codes::CONFIGURATION_ERROR, "CONFIGURATION_ERROR");
}

#[tokio::test]
async fn test_all_dimensions_function() {
    // Test the all_dimensions function
    let dimensions = all_dimensions();
    assert!(!dimensions.is_empty());
    
    // Check that it contains expected dimensions
    assert!(dimensions.contains(&"health"));
    assert!(dimensions.contains(&"mana"));
    assert!(dimensions.contains(&"strength"));
    assert!(dimensions.contains(&"agility"));
    assert!(dimensions.contains(&"intelligence"));
}

#[tokio::test]
async fn test_all_system_ids_function() {
    // Test the all_system_ids function
    let system_ids = all_system_ids();
    assert!(!system_ids.is_empty());
    
    // Check that it contains expected system IDs
    assert!(system_ids.contains(&"luyen_the"));
    assert!(system_ids.contains(&"combat"));
    assert!(system_ids.contains(&"equipment"));
    assert!(system_ids.contains(&"magic"));
}

#[tokio::test]
async fn test_constants_are_immutable() {
    // Test that constants are truly constant (can't be modified)
    // This is more of a compile-time check, but we can verify the values
    let strength_const = primary_dimensions::STRENGTH;
    assert_eq!(strength_const, "strength");
    
    let health_const = primary_dimensions::HEALTH;
    assert_eq!(health_const, "health");
}

#[tokio::test]
async fn test_constants_completeness() {
    // Test that all expected constants are present
    let all_dims = all_dimensions();
    let all_systems = all_system_ids();
    
    // Should have a reasonable number of dimensions and systems
    assert!(all_dims.len() > 5);
    assert!(all_systems.len() > 5);
    
    // All dimensions should be non-empty strings
    for dim in &all_dims {
        assert!(!dim.is_empty());
    }
    
    // All system IDs should be non-empty strings
    for sys_id in &all_systems {
        assert!(!sys_id.is_empty());
    }
}

#[tokio::test]
async fn test_constants_consistency() {
    // Test that related constants are consistent
    let health_dim = primary_dimensions::HEALTH;
    assert_eq!(health_dim, "health");
    
    let mana_dim = primary_dimensions::MANA;
    assert_eq!(mana_dim, "mana");
    
    // Test that system IDs are consistent
    let combat_system = system_ids::COMBAT;
    assert_eq!(combat_system, "combat");
    
    let magic_system = system_ids::MAGIC;
    assert_eq!(magic_system, "magic");
}

#[tokio::test]
async fn test_constants_string_values() {
    // Test that constants have expected string values
    assert_eq!(primary_dimensions::STRENGTH, "strength");
    assert_eq!(primary_dimensions::AGILITY, "agility");
    assert_eq!(primary_dimensions::INTELLIGENCE, "intelligence");
    
    assert_eq!(primary_dimensions::HEALTH, "health");
    assert_eq!(primary_dimensions::MANA, "mana");
    assert_eq!(primary_dimensions::STAMINA, "stamina");
    
    assert_eq!(system_ids::LUYEN_THE, "luyen_the");
    assert_eq!(system_ids::COMBAT, "combat");
    assert_eq!(system_ids::EQUIPMENT, "equipment");
}

#[tokio::test]
async fn test_error_codes_string_values() {
    // Test that error codes have expected string values
    assert_eq!(error_codes::INVALID_CAP, "INVALID_CAP");
    assert_eq!(error_codes::CACHE_ERROR, "CACHE_ERROR");
    assert_eq!(error_codes::SUBSYSTEM_ERROR, "SUBSYSTEM_ERROR");
    assert_eq!(error_codes::CONFIGURATION_ERROR, "CONFIGURATION_ERROR");
}