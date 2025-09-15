//! Simple direct coverage tests for constants.rs module.
//! This file provides direct tests that exercise the actual constants module code.

use actor_core::constants::*;

// ============================================================================
// SYSTEM IDS TESTS
// ============================================================================

#[test]
fn test_system_ids_constants() {
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

// ============================================================================
// PRIMARY DIMENSIONS TESTS
// ============================================================================

#[test]
fn test_primary_dimensions_constants() {
    assert_eq!(primary_dimensions::STRENGTH, "strength");
    assert_eq!(primary_dimensions::AGILITY, "agility");
    assert_eq!(primary_dimensions::INTELLIGENCE, "intelligence");
}

// ============================================================================
// META DIMENSIONS TESTS
// ============================================================================

#[test]
fn test_meta_dimensions_constants() {
    assert_eq!(meta_dimensions::REALM_ID, "realm_id");
    assert_eq!(meta_dimensions::WORLD_ID, "world_id");
    assert_eq!(meta_dimensions::ZONE_ID, "zone_id");
    assert_eq!(meta_dimensions::GUILD_ID, "guild_id");
    assert_eq!(meta_dimensions::PARTY_ID, "party_id");
    assert_eq!(meta_dimensions::EVENT_ID, "event_id");
}

// ============================================================================
// CONTEXT TYPES TESTS
// ============================================================================

#[test]
fn test_context_types_constants() {
    assert_eq!(context_types::DAMAGE, "damage");
    assert_eq!(context_types::HEALING, "healing");
    assert_eq!(context_types::EXPERIENCE_GAIN, "experience_gain");
    assert_eq!(context_types::ITEM_DROP, "item_drop");
    assert_eq!(context_types::COMBAT, "combat");
    assert_eq!(context_types::MOVEMENT, "movement");
    assert_eq!(context_types::CASTING, "casting");
}

// ============================================================================
// ERROR CODES TESTS
// ============================================================================

#[test]
fn test_error_codes_constants() {
    assert_eq!(error_codes::INVALID_ACTOR, "INVALID_ACTOR");
    assert_eq!(error_codes::INVALID_CONTRIBUTION, "INVALID_CONTRIBUTION");
    assert_eq!(error_codes::INVALID_CAP, "INVALID_CAP");
    assert_eq!(error_codes::SUBSYSTEM_ERROR, "SUBSYSTEM_ERROR");
    assert_eq!(error_codes::CACHE_ERROR, "CACHE_ERROR");
    assert_eq!(error_codes::REGISTRY_ERROR, "REGISTRY_ERROR");
    assert_eq!(error_codes::AGGREGATION_ERROR, "AGGREGATION_ERROR");
    assert_eq!(error_codes::CONFIGURATION_ERROR, "CONFIGURATION_ERROR");
}

// ============================================================================
// ERROR TYPES TESTS
// ============================================================================

#[test]
fn test_error_types_constants() {
    assert_eq!(error_types::VALIDATION, "VALIDATION");
    assert_eq!(error_types::SYSTEM, "SYSTEM");
    assert_eq!(error_types::NETWORK, "NETWORK");
    assert_eq!(error_types::DATABASE, "DATABASE");
    assert_eq!(error_types::CACHE, "CACHE");
    assert_eq!(error_types::CONFIGURATION, "CONFIGURATION");
}

// ============================================================================
// DEFAULTS TESTS
// ============================================================================

#[test]
fn test_defaults_constants() {
    assert_eq!(defaults::ACTOR_LIFESPAN, 365 * 24 * 60 * 60);
    assert_eq!(defaults::ACTOR_AGE, 0);
    assert_eq!(defaults::SUBSYSTEM_PRIORITY, 100);
    assert_eq!(defaults::CONTRIBUTION_PRIORITY, 100);
    assert_eq!(defaults::CAP_PRIORITY, 100);
    assert_eq!(defaults::CACHE_TTL, 3600);
    assert_eq!(defaults::BATCH_SIZE, 100);
    assert_eq!(defaults::MAX_RETRIES, 3);
}

// ============================================================================
// CLAMP RANGES FUNCTION TESTS
// ============================================================================

#[test]
fn test_clamp_ranges_function() {
    // Test primary dimension ranges
    let strength_range = clamp_ranges::primary_dimension_range(primary_dimensions::STRENGTH);
    assert!(strength_range.is_some());
    let (min, max) = strength_range.unwrap();
    assert_eq!(min, 0.0);
    assert_eq!(max, 10000.0);
    
    let agility_range = clamp_ranges::primary_dimension_range(primary_dimensions::AGILITY);
    assert!(agility_range.is_some());
    let (min, max) = agility_range.unwrap();
    assert_eq!(min, 0.0);
    assert_eq!(max, 10000.0);
    
    let intelligence_range = clamp_ranges::primary_dimension_range(primary_dimensions::INTELLIGENCE);
    assert!(intelligence_range.is_some());
    let (min, max) = intelligence_range.unwrap();
    assert_eq!(min, 0.0);
    assert_eq!(max, 10000.0);
}

#[test]
fn test_clamp_ranges_unknown_dimension() {
    let unknown_range = clamp_ranges::primary_dimension_range("unknown_dimension");
    assert!(unknown_range.is_none());
}

// ============================================================================
// COMPREHENSIVE CONSTANT USAGE TESTS
// ============================================================================

#[test]
fn test_constants_in_collections() {
    use std::collections::HashMap;
    
    let mut system_map = HashMap::new();
    system_map.insert(system_ids::COMBAT, "combat_system");
    system_map.insert(system_ids::MAGIC, "magic_system");
    
    assert_eq!(system_map.get(system_ids::COMBAT), Some(&"combat_system"));
    assert_eq!(system_map.get(system_ids::MAGIC), Some(&"magic_system"));
}

#[test]
fn test_constants_in_conditionals() {
    let system_id = system_ids::COMBAT;
    
    if system_id == system_ids::COMBAT {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn test_constants_in_calculations() {
    let base_value = defaults::BATCH_SIZE;
    let multiplier = 2;
    let result = base_value * multiplier;
    
    assert_eq!(result, 200);
}

#[test]
fn test_constants_in_string_operations() {
    let error_code = error_codes::INVALID_ACTOR;
    let error_message = format!("Error: {}", error_code);
    
    assert_eq!(error_message, "Error: INVALID_ACTOR");
}

#[test]
fn test_constants_in_numeric_operations() {
    let lifespan = defaults::ACTOR_LIFESPAN;
    let age = defaults::ACTOR_AGE;
    let remaining = lifespan - age;
    
    assert_eq!(remaining, lifespan);
    assert!(remaining > 0);
}

#[test]
fn test_constants_in_pattern_matching() {
    let error_type = error_types::VALIDATION;
    
    match error_type {
        error_types::VALIDATION => assert!(true),
        error_types::SYSTEM => assert!(false),
        error_types::NETWORK => assert!(false),
        _ => assert!(false),
    }
}

#[test]
fn test_constants_in_function_calls() {
    // Test that constants can be used as function arguments
    let strength_range = clamp_ranges::primary_dimension_range(primary_dimensions::STRENGTH);
    assert!(strength_range.is_some());
    
    let agility_range = clamp_ranges::primary_dimension_range(primary_dimensions::AGILITY);
    assert!(agility_range.is_some());
}

#[test]
fn test_constants_in_assertions() {
    // Test that constants can be used in assertions
    assert!(defaults::ACTOR_LIFESPAN > 0);
    assert!(defaults::CACHE_TTL > 0);
    assert!(defaults::BATCH_SIZE > 0);
    assert!(defaults::MAX_RETRIES > 0);
}

#[test]
fn test_constants_in_variable_assignments() {
    let system_id = system_ids::COMBAT;
    let error_code = error_codes::INVALID_ACTOR;
    let error_type = error_types::VALIDATION;
    let batch_size = defaults::BATCH_SIZE;
    
    assert_eq!(system_id, "combat");
    assert_eq!(error_code, "INVALID_ACTOR");
    assert_eq!(error_type, "VALIDATION");
    assert_eq!(batch_size, 100);
}
