//! # Hierarchical Actor Tests
//! 
//! Integration tests for the hierarchical actor functionality.

use actor_core_hierarchical::HierarchicalActor;
use chrono::Utc;

#[test]
fn test_hierarchical_actor_creation() {
    let actor = HierarchicalActor::new();
    assert!(!actor.id.is_empty());
    assert_eq!(actor.name, "Unnamed Actor");
    assert!(actor.elemental_system.get_data().element_mastery_levels[0] == 0.0);
}

#[test]
fn test_hierarchical_actor_with_id_and_name() {
    let actor = HierarchicalActor::with_id_and_name("test-id".to_string(), "Test Actor".to_string());
    assert_eq!(actor.id, "test-id");
    assert_eq!(actor.name, "Test Actor");
}

#[test]
fn test_system_contribution() {
    let mut actor = HierarchicalActor::new();
    let contribution = actor_core_hierarchical::SystemContribution {
        system_name: "elemental".to_string(),
        stat_name: "health".to_string(),
        value: 100.0,
        priority: 1,
        timestamp: Utc::now(),
    };
    
    actor.add_system_contribution(contribution);
    let contributions = actor.get_system_contributions("elemental").unwrap();
    assert_eq!(contributions.len(), 1);
    assert_eq!(contributions[0].value, 100.0);
}

#[test]
fn test_metadata_operations() {
    let mut actor = HierarchicalActor::new();
    actor.set_metadata("race".to_string(), "human".to_string());
    actor.set_metadata("class".to_string(), "warrior".to_string());
    
    assert_eq!(actor.get_metadata("race").unwrap(), "human");
    assert_eq!(actor.get_metadata("class").unwrap(), "warrior");
    assert_eq!(actor.get_all_metadata().len(), 2);
}

#[test]
fn test_elemental_system_access() {
    let mut actor = HierarchicalActor::new();
    
    // Test getting elemental data
    let elemental_data = actor.get_elemental_data();
    assert_eq!(elemental_data.element_mastery_levels[0], 0.0);
    
    // Test mutable access
    let mut_data = actor.get_elemental_data_mut();
    mut_data.element_mastery_levels[0] = 5.0;
    assert_eq!(mut_data.element_mastery_levels[0], 5.0);
}

#[test]
fn test_actor_timestamps() {
    let actor = HierarchicalActor::new();
    let created_at = actor.get_created_at();
    let updated_at = actor.get_updated_at();
    
    assert!(created_at <= updated_at);
}

#[test]
fn test_global_stats_cache() {
    let mut actor = HierarchicalActor::new();
    let mut stats = std::collections::HashMap::new();
    stats.insert("total_health".to_string(), 1000.0);
    stats.insert("total_mana".to_string(), 500.0);
    
    actor.update_global_stats_cache(stats);
    let cached_stats = actor.get_global_stats_cache();
    
    assert_eq!(cached_stats.get("total_health").unwrap(), &1000.0);
    assert_eq!(cached_stats.get("total_mana").unwrap(), &500.0);
}
