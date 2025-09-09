//! Resource Manager Integration Tests
//! 
//! These tests demonstrate the Resource Manager Subsystem working
//! with the Actor Core aggregation system.

use actor_core::*;
use std::collections::HashMap;

#[tokio::test]
async fn test_resource_manager_integration() {
    // Create a Resource Manager Subsystem
    let resource_manager = ResourceManagerSubsystem::new();
    
    // Create an actor with different characteristics
    let mut actor = Actor::new("Test Warrior".to_string(), "Human".to_string());
    actor.set_lifespan(100);
    actor.set_age(25);
    
    // Add some buffs
    actor.add_buff("strength_boost".to_string());
    actor.add_buff("health_regeneration".to_string());
    
    // Set combat status
    actor.set_in_combat(true);
    
    // Get resource contributions
    let result = resource_manager.contribute(&actor).await;
    assert!(result.is_ok());
    
    let output = result.unwrap();
    
    // Verify system ID and priority
    assert_eq!(output.meta.system, "resource_manager");
    
    // Verify we have primary resources
    assert!(!output.primary.is_empty());
    
    // Check for expected resource dimensions
    let dimensions: Vec<String> = output.primary.iter()
        .map(|c| c.dimension.clone())
        .collect();
    
    assert!(dimensions.contains(&"hp_current".to_string()));
    assert!(dimensions.contains(&"hp_max".to_string()));
    assert!(dimensions.contains(&"mana_current".to_string()));
    assert!(dimensions.contains(&"mana_max".to_string()));
    assert!(dimensions.contains(&"stamina_current".to_string()));
    assert!(dimensions.contains(&"stamina_max".to_string()));
    
    // Verify derived resources (percentages)
    assert!(!output.derived.is_empty());
    let derived_dimensions: Vec<String> = output.derived.iter()
        .map(|c| c.dimension.clone())
        .collect();
    
    assert!(derived_dimensions.contains(&"hp_percentage".to_string()));
    assert!(derived_dimensions.contains(&"mana_percentage".to_string()));
    
    // Verify caps are applied
    assert!(!output.caps.is_empty());
}

#[tokio::test]
async fn test_resource_calculation_with_different_races() {
    let resource_manager = ResourceManagerSubsystem::new();
    
    let races = vec!["Human", "Elf", "Dwarf", "Orc"];
    let mut results = HashMap::new();
    
    for race in races {
        let mut actor = Actor::new(format!("{} Warrior", race), race.to_string());
        actor.set_lifespan(100);
        actor.set_age(25);
        
        let result = resource_manager.contribute(&actor).await.unwrap();
        
        // Extract HP values for comparison
        let hp_max = result.primary.iter()
            .find(|c| c.dimension == "hp_max")
            .map(|c| c.value)
            .unwrap_or(0.0);
        
        results.insert(race, hp_max);
    }
    
    // Verify different races have different HP values
    // (due to race modifiers - Orcs get 1.3x for Health)
    let human_hp = results.get("Human").unwrap();
    let elf_hp = results.get("Elf").unwrap();
    let dwarf_hp = results.get("Dwarf").unwrap();
    let orc_hp = results.get("Orc").unwrap();
    
    // Orcs should have the highest HP (1.3x modifier for Health)
    assert!(orc_hp > human_hp);
    
    // Humans and Elfs should have the same HP (both get 1.0x for Health)
    assert_eq!(human_hp, elf_hp);
    
    // Dwarfs should have the same HP as Humans (both get 1.0x for Health)
    assert_eq!(human_hp, dwarf_hp);
}

#[tokio::test]
async fn test_cultivation_modifiers() {
    let resource_manager = ResourceManagerSubsystem::new();
    
    // Create actor without cultivation
    let mut actor_no_cult = Actor::new("Normal Human".to_string(), "Human".to_string());
    actor_no_cult.set_lifespan(100);
    actor_no_cult.set_age(25);
    
    // Create actor with cultivation
    let mut actor_with_cult = Actor::new("Cultivator".to_string(), "Human".to_string());
    actor_with_cult.set_lifespan(100);
    actor_with_cult.set_age(25);
    actor_with_cult.add_subsystem(SubsystemStruct::new("jindan_system".to_string(), 100));
    
    // Get results
    let result_no_cult = resource_manager.contribute(&actor_no_cult).await.unwrap();
    let result_with_cult = resource_manager.contribute(&actor_with_cult).await.unwrap();
    
    // Extract HP values
    let hp_no_cult = result_no_cult.primary.iter()
        .find(|c| c.dimension == "hp_max")
        .map(|c| c.value)
        .unwrap_or(0.0);
    
    let hp_with_cult = result_with_cult.primary.iter()
        .find(|c| c.dimension == "hp_max")
        .map(|c| c.value)
        .unwrap_or(0.0);
    
    // Cultivation should increase HP (1.5x modifier)
    assert!(hp_with_cult > hp_no_cult);
    assert!((hp_with_cult / hp_no_cult - 1.5).abs() < 0.1); // Within 10% of expected 1.5x
}

#[tokio::test]
async fn test_resource_bucket_processing() {
    // Test that the Resource Manager works with bucket processing
    let resource_manager = ResourceManagerSubsystem::new();
    
    let mut actor = Actor::new("Test Actor".to_string(), "Human".to_string());
    actor.set_lifespan(100);
    actor.set_age(25);
    
    let result = resource_manager.contribute(&actor).await.unwrap();
    
    // Verify that contributions use appropriate bucket types
    for contribution in &result.primary {
        if contribution.dimension.ends_with("_percentage") {
            // Percentages should use Override bucket
            assert_eq!(contribution.bucket, Bucket::Override);
        } else {
            // Most resources should use Flat bucket
            assert_eq!(contribution.bucket, Bucket::Flat);
        }
    }
}

#[tokio::test]
async fn test_resource_caps() {
    let resource_manager = ResourceManagerSubsystem::new();
    
    let mut actor = Actor::new("Test Actor".to_string(), "Human".to_string());
    actor.set_lifespan(100);
    actor.set_age(25);
    
    let result = resource_manager.contribute(&actor).await.unwrap();
    
    // Verify that caps are applied
    assert!(!result.caps.is_empty());
    
    // Check for specific caps
    let hp_caps = result.caps.iter()
        .find(|c| c.dimension == "hp_current")
        .is_some();
    assert!(hp_caps);
    
    let mana_caps = result.caps.iter()
        .find(|c| c.dimension == "mana_current")
        .is_some();
    assert!(mana_caps);
}

#[tokio::test]
async fn test_resource_regeneration_rates() {
    let resource_manager = ResourceManagerSubsystem::new();
    
    let mut actor = Actor::new("Test Actor".to_string(), "Human".to_string());
    actor.set_lifespan(100);
    actor.set_age(25);
    
    let result = resource_manager.contribute(&actor).await.unwrap();
    
    // Check that regeneration rates are calculated
    let hp_regen = result.primary.iter()
        .find(|c| c.dimension == "hp_regen")
        .map(|c| c.value)
        .unwrap_or(0.0);
    
    let mana_regen = result.primary.iter()
        .find(|c| c.dimension == "mana_regen")
        .map(|c| c.value)
        .unwrap_or(0.0);
    
    let stamina_regen = result.primary.iter()
        .find(|c| c.dimension == "stamina_regen")
        .map(|c| c.value)
        .unwrap_or(0.0);
    
    // Verify regeneration rates are positive
    assert!(hp_regen > 0.0);
    assert!(mana_regen > 0.0);
    assert!(stamina_regen > 0.0);
    
    // Verify expected regeneration rates
    assert_eq!(hp_regen, 1.0);
    assert_eq!(mana_regen, 2.0);
    assert_eq!(stamina_regen, 3.0);
}

#[tokio::test]
async fn test_resource_percentages() {
    let resource_manager = ResourceManagerSubsystem::new();
    
    let mut actor = Actor::new("Test Actor".to_string(), "Human".to_string());
    actor.set_lifespan(100);
    actor.set_age(25);
    
    let result = resource_manager.contribute(&actor).await.unwrap();
    
    // Check that percentages are calculated and are 100% initially
    for contribution in &result.derived {
        if contribution.dimension.ends_with("_percentage") {
            assert_eq!(contribution.value, 100.0);
        }
    }
}
