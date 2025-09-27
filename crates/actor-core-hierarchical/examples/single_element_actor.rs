//! # Single Element Actor Example
//!
//! This example demonstrates how to create an actor with a single element (Fire).

use actor_core_hierarchical::ActorFactory;
use element_core::{ElementalParams, ElementalRegistry, ElementConfig, ElementDefinition, ElementAliases, BaseProperties};
use std::collections::HashMap;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Creating Single Element Actor Example");
    
    // 1. Create Elemental Registry with Fire Element Only
    let mut registry = ElementalRegistry::new();
    
    // Register Fire Element (Hỏa)
    let fire_config = ElementConfig {
        version: 1,
        element: ElementDefinition {
            id: "fire".to_string(),
            name: "Fire".to_string(),
            aliases: ElementAliases {
                vi: Some("Hỏa".to_string()),
                zh_pinyin: Some("huo".to_string()),
            },
            category: "five_elements".to_string(),
            description: "Fire element - represents passion, energy, and destruction".to_string(),
            base_properties: BaseProperties {
                base_damage: 120.0,
                base_defense: 70.0,
                base_crit_rate: 0.18,
                base_crit_damage: 1.6,
                base_accuracy: 0.82,
            },
            probability_overrides: HashMap::new(),
            derived_stats: vec!["fire_mastery".to_string(), "fire_resistance".to_string()],
            status_effects: vec![],
            same_element_effects: vec![],
            neutral_effects: vec![],
            environment_mods: HashMap::new(),
            references: element_core::core::elemental_config::ElementReferences {
                probability_config_path: None,
                interaction_config_path: None,
                status_pool_path: None,
                golden_vectors_path: None,
                dynamics_design: None,
            },
        },
    };
    registry.register_element("fire".to_string(), fire_config, 0)?;
    
    println!("✅ Registered Fire element in registry");
    
    // 2. Create Actor Factory
    let factory = ActorFactory::new(Arc::new(registry));
    
    // 3. Create Fire Element Parameters
    let mut mastery_levels = HashMap::new();
    mastery_levels.insert("fire".to_string(), 20.0);
    
    let mut experience = HashMap::new();
    experience.insert("fire".to_string(), 2000.0);
    
    let mut qi_amounts = HashMap::new();
    qi_amounts.insert("fire".to_string(), 1500.0);
    
    let elemental_params = ElementalParams {
        primary_element: "fire".to_string(),
        initial_mastery_levels: mastery_levels,
        initial_experience: experience,
        initial_qi_amounts: qi_amounts,
        elemental_preferences: vec!["fire".to_string()],
    };
    
    println!("✅ Created Fire Element Parameters");
    
    // 4. Create Actor with Fire Element
    let actor = factory.create_actor_with_elemental("fire_master", elemental_params)?;
    
    println!("✅ Created Fire Master Actor");
    println!("Actor ID: {}", actor.get_id());
    println!("Actor Name: {}", actor.get_name());
    
    // 5. Display Fire Element Data
    let elemental_data = actor.get_elemental_system().get_data();
    
    println!("\n🔥 Fire Master Stats:");
    println!("====================");
    println!("Fire Mastery Level: {:.1}", elemental_data.element_mastery_levels[0]);
    println!("Fire Experience: {:.1}", elemental_data.element_mastery_experience[0]);
    println!("Fire Qi Amount: {:.1}", elemental_data.element_qi_amounts[0]);
    println!("Fire Qi Capacity: {:.1}", elemental_data.element_qi_capacities[0]);
    println!("Fire Power Points: {:.1}", elemental_data.power_point[0]);
    println!("Fire Defense Points: {:.1}", elemental_data.defense_point[0]);
    println!("Fire Crit Rate: {:.2}%", elemental_data.crit_rate[0] * 100.0);
    println!("Fire Crit Damage: {:.2}x", elemental_data.crit_damage[0]);
    println!("Fire Accuracy: {:.2}%", elemental_data.accurate_rate[0] * 100.0);
    
    println!("\n🎉 Single Element Actor created successfully!");
    
    Ok(())
}
