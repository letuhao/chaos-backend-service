//! # Step by Step Example
//!
//! This example demonstrates creating an actor with elemental parameters step by step.

use actor_core_hierarchical::ActorFactory;
use element_core::{ElementalParams, ElementalRegistry, ElementConfig, ElementDefinition, ElementAliases, BaseProperties};
use std::collections::HashMap;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Step by Step Example");
    
    // 1. Create Elemental Registry with Fire Element Only
    let mut registry = ElementalRegistry::new();
    println!("✅ Created registry");
    
    // Register Fire Element (Hỏa)
    let fire_config = create_fire_config();
    registry.register_element("fire".to_string(), fire_config, 0)?;
    println!("✅ Registered Fire element");
    
    // 2. Create Actor Factory
    let factory = ActorFactory::new(Arc::new(registry));
    println!("✅ Created factory");
    
    // 3. Test basic actor creation first
    let basic_actor = factory.create_actor("warrior")?;
    println!("✅ Created basic actor: {}", basic_actor.get_name());
    
    // 4. Now test with elemental params
    println!("🔧 Testing elemental params...");
    
    let mut mastery_levels = HashMap::new();
    mastery_levels.insert("fire".to_string(), 10.0);
    
    let mut experience = HashMap::new();
    experience.insert("fire".to_string(), 1000.0);
    
    let mut qi_amounts = HashMap::new();
    qi_amounts.insert("fire".to_string(), 500.0);
    
    let elemental_params = ElementalParams {
        primary_element: "fire".to_string(),
        initial_mastery_levels: mastery_levels,
        initial_experience: experience,
        initial_qi_amounts: qi_amounts,
        elemental_preferences: vec!["fire".to_string()],
    };
    
    println!("✅ Created elemental params");
    
    // 5. Create actor with elemental params
    let actor = factory.create_actor_with_elemental("fire_warrior", elemental_params)?;
    println!("✅ Created fire warrior actor: {}", actor.get_name());
    
    // 6. Display results
    let elemental_data = actor.get_elemental_system().get_data();
    println!("Fire Mastery Level: {}", elemental_data.element_mastery_levels[0]);
    println!("Fire Experience: {}", elemental_data.element_mastery_experience[0]);
    println!("Fire Qi Amount: {}", elemental_data.element_qi_amounts[0]);
    
    println!("\n🎉 Step by step example completed successfully!");
    
    Ok(())
}

fn create_fire_config() -> ElementConfig {
    ElementConfig {
        version: 1,
        element: ElementDefinition {
            id: "fire".to_string(),
            name: "Fire".to_string(),
            aliases: ElementAliases {
                vi: Some("Hỏa".to_string()),
                zh_pinyin: Some("huo".to_string()),
            },
            category: "five_elements".to_string(),
            description: "Fire element".to_string(),
            base_properties: BaseProperties {
                base_damage: 100.0,
                base_defense: 80.0,
                base_crit_rate: 0.15,
                base_crit_damage: 1.5,
                base_accuracy: 0.85,
            },
            probability_overrides: HashMap::new(),
            derived_stats: vec![],
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
    }
}
