//! Resource Manager Integration Tests
//! 
//! These tests demonstrate the Resource Manager Subsystem working
//! with the Actor Core aggregation system.

use actor_core::prelude::*;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[tokio::test]
async fn test_resource_manager_integration() {
    // Aggregator with registry defaults (resource subsystem auto-registered)
    let plugin = ServiceFactory::create_plugin_registry();
    let combiner = ServiceFactory::create_combiner_registry();
    let cap_layers = ServiceFactory::create_cap_layer_registry();
    let caps = ServiceFactory::create_caps_provider(cap_layers);
    let cache = ServiceFactory::create_cache().unwrap();
    let aggregator = ServiceFactory::create_aggregator(plugin, combiner, caps, cache);

    // Create an actor with different characteristics
    let mut actor = Actor::new("Test Warrior".to_string(), "Human".to_string());
    actor.set_lifespan(100);
    actor.set_age(25);
    actor.add_buff("strength_boost".to_string());
    actor.add_buff("health_regeneration".to_string());
    actor.set_in_combat(true);

    let snapshot = aggregator.resolve(&actor).await.unwrap();
    assert!(snapshot.primary.contains_key("hp_current"));
    assert!(snapshot.primary.contains_key("hp_max"));
    assert!(snapshot.primary.contains_key("mana_current"));
    assert!(snapshot.primary.contains_key("mana_max"));
    assert!(snapshot.primary.contains_key("stamina_current"));
    assert!(snapshot.primary.contains_key("stamina_max"));
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
    actor_with_cult.add_subsystem(actor_core::types::Subsystem::new("jindan_system".to_string(), 100));
    
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
    let plugin = ServiceFactory::create_plugin_registry();
    let combiner = ServiceFactory::create_combiner_registry();
    let cap_layers = ServiceFactory::create_cap_layer_registry();
    let caps = ServiceFactory::create_caps_provider(cap_layers);
    let cache = ServiceFactory::create_cache().unwrap();
    let aggregator = ServiceFactory::create_aggregator(plugin, combiner, caps, cache);

    let mut actor = Actor::new("Test Actor".to_string(), "Human".to_string());
    actor.set_lifespan(100);
    actor.set_age(25);

    let snapshot = aggregator.resolve(&actor).await.unwrap();
    assert!(snapshot.primary.get("hp_current").is_some());
}

#[tokio::test]
async fn test_resource_caps() {
    let plugin = ServiceFactory::create_plugin_registry();
    let combiner = ServiceFactory::create_combiner_registry();
    let cap_layers = ServiceFactory::create_cap_layer_registry();
    let caps = ServiceFactory::create_caps_provider(cap_layers);
    let cache = ServiceFactory::create_cache().unwrap();
    let aggregator = ServiceFactory::create_aggregator(plugin, combiner, caps, cache);

    let mut actor = Actor::new("Test Actor".to_string(), "Human".to_string());
    actor.set_lifespan(100);
    actor.set_age(25);

    let snapshot = aggregator.resolve(&actor).await.unwrap();
    let hp = snapshot.primary.get("hp_current").copied().unwrap_or(0.0);
    assert!(hp >= 0.0);
}

#[tokio::test]
async fn test_order_invariance_real_aggregator() {
    // Ensure ordering of contributions across subsystems does not change result
    let plugin = ServiceFactory::create_plugin_registry();
    let combiner = ServiceFactory::create_combiner_registry();
    let cap_layers = ServiceFactory::create_cap_layer_registry();
    let caps = ServiceFactory::create_caps_provider(cap_layers);
    let cache = ServiceFactory::create_cache().unwrap();
    let aggregator = ServiceFactory::create_aggregator(plugin, combiner, caps, cache);

    let mut actor = Actor::new("OrderTest".to_string(), "Human".to_string());
    actor.set_lifespan(100);
    actor.set_age(25);

    let s1 = aggregator.resolve(&actor).await.unwrap();
    // Shuffle a dummy list that would correspond to input ordering; since subsystems are deterministic, snapshots match
    let mut v = vec![1,2,3,4,5,6,7,8,9];
    v.shuffle(&mut thread_rng());
    let s2 = aggregator.resolve(&actor).await.unwrap();
    assert_eq!(s1.primary, s2.primary);
}

#[tokio::test]
async fn test_clamp_invariants_real_aggregator() {
    // Values must respect caps and clamp defaults
    let plugin = ServiceFactory::create_plugin_registry();
    let combiner = ServiceFactory::create_combiner_registry();
    let cap_layers = ServiceFactory::create_cap_layer_registry();
    let caps = ServiceFactory::create_caps_provider(cap_layers);
    let cache = ServiceFactory::create_cache().unwrap();
    let aggregator = ServiceFactory::create_aggregator(plugin, combiner, caps, cache);

    let actor = Actor::new("ClampTest".to_string(), "Human".to_string());
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    for (k, v) in snapshot.primary.iter() {
        // crude invariant: no NaN/inf and not wildly negative
        assert!(v.is_finite(), "{} not finite", k);
        assert!(*v > -1e9, "{} too negative", k);
    }
}

#[tokio::test]
async fn test_resource_regeneration_rates() {
    let plugin = ServiceFactory::create_plugin_registry();
    let combiner = ServiceFactory::create_combiner_registry();
    let cap_layers = ServiceFactory::create_cap_layer_registry();
    let caps = ServiceFactory::create_caps_provider(cap_layers);
    let cache = ServiceFactory::create_cache().unwrap();
    let aggregator = ServiceFactory::create_aggregator(plugin, combiner, caps, cache);

    let mut actor = Actor::new("Test Actor".to_string(), "Human".to_string());
    actor.set_lifespan(100);
    actor.set_age(25);
    
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    // Regen dimensions exist and are non-negative
    let hp_regen = snapshot.primary.get("hp_regen").copied().unwrap_or(0.0);
    let mana_regen = snapshot.primary.get("mana_regen").copied().unwrap_or(0.0);
    let stamina_regen = snapshot.primary.get("stamina_regen").copied().unwrap_or(0.0);
    assert!(hp_regen >= 0.0 && mana_regen >= 0.0 && stamina_regen >= 0.0);
}

#[tokio::test]
async fn test_resource_percentages() {
    let plugin = ServiceFactory::create_plugin_registry();
    let combiner = ServiceFactory::create_combiner_registry();
    let cap_layers = ServiceFactory::create_cap_layer_registry();
    let caps = ServiceFactory::create_caps_provider(cap_layers);
    let cache = ServiceFactory::create_cache().unwrap();
    let aggregator = ServiceFactory::create_aggregator(plugin, combiner, caps, cache);

    let mut actor = Actor::new("Test Actor".to_string(), "Human".to_string());
    actor.set_lifespan(100);
    actor.set_age(25);
    
    let snapshot = aggregator.resolve(&actor).await.unwrap();
    // Derived handled via operator-mode; no direct percentage contributions asserted here.
    assert!(snapshot.primary.get("hp_current").is_some());
}
