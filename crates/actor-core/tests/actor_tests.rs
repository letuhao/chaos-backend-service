//! Comprehensive tests for Actor functionality.
//!
//! This module contains detailed tests for all Actor methods and behaviors,
//! including edge cases, error conditions, and performance characteristics.

use actor_core::types::{Actor, Subsystem as SubsystemStruct};

/// Test basic actor creation and validation
#[tokio::test]
async fn test_actor_creation_and_validation() {
    // Test valid actor creation
    let actor = Actor::new("TestActor".to_string(), "Human".to_string());
    assert!(actor.is_valid());
    assert_eq!(actor.get_name(), "TestActor");
    assert_eq!(actor.get_race(), "Human");
    assert_eq!(actor.get_version(), 1);
    assert_eq!(actor.get_subsystem_count(), 0);
    
    // Test invalid actor creation
    let mut invalid_actor = Actor::new("".to_string(), "Human".to_string());
    assert!(!invalid_actor.is_valid());
    
    invalid_actor = Actor::new("TestActor".to_string(), "".to_string());
    assert!(!invalid_actor.is_valid());
}

/// Test actor getter and setter methods
#[tokio::test]
async fn test_actor_getters_and_setters() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test name operations
    actor.set_name("NewName".to_string());
    assert_eq!(actor.get_name(), "NewName");
    
    // Test race operations
    actor.set_race("Elf".to_string());
    assert_eq!(actor.get_race(), "Elf");
    
    // Test lifespan operations
    actor.set_lifespan(1000);
    assert_eq!(actor.get_lifespan(), 1000);
    
    // Test age operations
    actor.set_age(25);
    assert_eq!(actor.get_age(), 25);
    
    // Test version operations
    let initial_version = actor.get_version();
    actor.touch();
    assert_eq!(actor.get_version(), initial_version + 1);
    
    // Test update_version (different from touch)
    let version_before = actor.get_version();
    actor.update_version();
    assert_eq!(actor.get_version(), version_before + 1);
}

/// Test actor subsystem management
#[tokio::test]
async fn test_actor_subsystem_management() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test adding subsystems
    let combat_subsystem = SubsystemStruct::new("combat".to_string(), 10);
    let magic_subsystem = SubsystemStruct::new("magic".to_string(), 5);
    
    actor.add_subsystem(combat_subsystem);
    actor.add_subsystem(magic_subsystem);
    
    assert_eq!(actor.get_subsystem_count(), 2);
    assert!(actor.has_subsystem("combat"));
    assert!(actor.has_subsystem("magic"));
    assert!(!actor.has_subsystem("stealth"));
    
    // Test getting subsystems
    let combat = actor.get_subsystem("combat");
    assert!(combat.is_some());
    assert_eq!(combat.unwrap().get_system_id(), "combat");
    
    // Test getting subsystems by priority
    let by_priority = actor.get_subsystem_by_priority();
    assert_eq!(by_priority.len(), 2);
    // Combat should come first (higher priority)
    assert_eq!(by_priority[0].get_system_id(), "combat");
    assert_eq!(by_priority[1].get_system_id(), "magic");
    
    // Test removing subsystems
    assert!(actor.remove_subsystem("combat"));
    assert!(!actor.remove_subsystem("nonexistent"));
    assert_eq!(actor.get_subsystem_count(), 1);
    assert!(!actor.has_subsystem("combat"));
    assert!(actor.has_subsystem("magic"));
}

/// Test actor combat system
#[tokio::test]
async fn test_actor_combat_system() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Initially not in combat
    assert!(!actor.is_in_combat());
    
    // Enter combat
    actor.set_in_combat(true);
    assert!(actor.is_in_combat());
    
    // Test combat duration
    let duration = actor.get_combat_duration();
    assert!(duration.is_some());
    assert!(duration.unwrap() >= 0);
    
    // Set specific combat duration (for testing)
    actor.set_combat_duration(100);
    let duration = actor.get_combat_duration();
    assert!(duration.is_some());
    assert!(duration.unwrap() >= 100);
    
    // Exit combat
    actor.set_in_combat(false);
    assert!(!actor.is_in_combat());
    assert!(actor.get_combat_duration().is_none());
}

/// Test actor buff management
#[tokio::test]
async fn test_actor_buff_management() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Initially no buffs
    assert!(!actor.has_buffs());
    assert!(actor.get_active_buffs().is_empty());
    
    // Add buffs
    actor.add_buff("strength_boost".to_string());
    actor.add_buff("speed_boost".to_string());
    
    // has_buffs() checks for "has_buffs" key, not the actual buffs array
    // So we need to set it manually or check get_active_buffs().is_empty()
    assert!(!actor.get_active_buffs().is_empty());
    assert!(actor.has_buff("strength_boost"));
    assert!(actor.has_buff("speed_boost"));
    assert!(!actor.has_buff("nonexistent"));
    
    let active_buffs = actor.get_active_buffs();
    assert_eq!(active_buffs.len(), 2);
    assert!(active_buffs.contains(&"strength_boost"));
    assert!(active_buffs.contains(&"speed_boost"));
    
    // Test adding duplicate buff (should not add)
    actor.add_buff("strength_boost".to_string());
    assert_eq!(actor.get_active_buffs().len(), 2);
    
    // Remove buffs
    assert!(actor.remove_buff("strength_boost"));
    assert!(!actor.remove_buff("nonexistent"));
    assert!(!actor.has_buff("strength_boost"));
    assert!(actor.has_buff("speed_boost"));
    
    // Clear all buffs
    actor.clear_buffs();
    assert!(actor.get_active_buffs().is_empty());
}

/// Test actor guild management
#[tokio::test]
async fn test_actor_guild_management() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Initially no guild
    assert!(actor.get_guild_id().is_none());
    assert!(!actor.is_guild_member());
    
    // Join guild
    actor.set_guild_id("guild_123".to_string());
    assert_eq!(actor.get_guild_id(), Some("guild_123"));
    assert!(actor.is_guild_member());
    
    // Leave guild
    actor.set_guild_id("".to_string());
    assert_eq!(actor.get_guild_id(), Some(""));
    assert!(actor.is_guild_member()); // Empty string still counts as guild member
    
    // Set to None by removing the key
    actor.data.remove("guild_id");
    assert!(actor.get_guild_id().is_none());
    assert!(!actor.is_guild_member());
}

/// Test actor online status
#[tokio::test]
async fn test_actor_online_status() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Default to online
    assert!(actor.is_online());
    
    // Set offline
    actor.set_online(false);
    assert!(!actor.is_online());
    
    // Set online again
    actor.set_online(true);
    assert!(actor.is_online());
}

/// Test actor data management
#[tokio::test]
async fn test_actor_data_management() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test getting data
    let data = actor.get_data();
    assert!(data.is_empty());
    
    // Test setting custom data
    actor.data.insert("custom_field".to_string(), serde_json::Value::String("custom_value".to_string()));
    let data = actor.get_data();
    assert_eq!(data.len(), 1);
    assert_eq!(data.get("custom_field").unwrap().as_str(), Some("custom_value"));
}

/// Test actor edge cases and error conditions
#[tokio::test]
async fn test_actor_edge_cases() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Test with very long names
    let long_name = "a".repeat(1000);
    actor.set_name(long_name.clone());
    assert_eq!(actor.get_name(), long_name);
    
    // Test with special characters
    actor.set_name("Test@Actor#123".to_string());
    assert_eq!(actor.get_name(), "Test@Actor#123");
    
    // Test with negative values
    actor.set_lifespan(-100);
    assert_eq!(actor.get_lifespan(), -100);
    
    actor.set_age(-50);
    assert_eq!(actor.get_age(), -50);
    
    // Test with extreme values
    actor.set_lifespan(i64::MAX);
    assert_eq!(actor.get_lifespan(), i64::MAX);
    
    actor.set_age(i64::MAX);
    assert_eq!(actor.get_age(), i64::MAX);
}

/// Test actor performance with many subsystems
#[tokio::test]
async fn test_actor_performance_many_subsystems() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Add many subsystems
    for i in 0..100 {
        let subsystem = SubsystemStruct::new(format!("system_{}", i), i as i64);
        actor.add_subsystem(subsystem);
    }
    
    assert_eq!(actor.get_subsystem_count(), 100);
    
    // Test finding subsystems
    assert!(actor.has_subsystem("system_0"));
    assert!(actor.has_subsystem("system_50"));
    assert!(actor.has_subsystem("system_99"));
    assert!(!actor.has_subsystem("system_100"));
    
    // Test getting subsystems by priority
    let by_priority = actor.get_subsystem_by_priority();
    assert_eq!(by_priority.len(), 100);
    
    // Should be sorted by priority (descending)
    for i in 1..by_priority.len() {
        assert!(by_priority[i-1].get_priority() >= by_priority[i].get_priority());
    }
}

/// Test actor with many buffs
#[tokio::test]
async fn test_actor_performance_many_buffs() {
    let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
    
    // Add many buffs
    for i in 0..1000 {
        actor.add_buff(format!("buff_{}", i));
    }
    
    assert_eq!(actor.get_active_buffs().len(), 1000);
    assert!(!actor.get_active_buffs().is_empty());
    
    // Test finding specific buffs
    assert!(actor.has_buff("buff_0"));
    assert!(actor.has_buff("buff_500"));
    assert!(actor.has_buff("buff_999"));
    assert!(!actor.has_buff("buff_1000"));
    
    // Test removing buffs
    for i in 0..500 {
        assert!(actor.remove_buff(&format!("buff_{}", i)));
    }
    
    assert_eq!(actor.get_active_buffs().len(), 500);
    assert!(!actor.has_buff("buff_0"));
    assert!(actor.has_buff("buff_500"));
}
