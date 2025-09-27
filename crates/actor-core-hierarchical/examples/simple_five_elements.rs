//! # Simple Five Elements Actor Example
//!
//! This example demonstrates how to create an actor with five elements (ngũ hành) configuration.

use actor_core_hierarchical::ActorFactory;
use element_core::{ElementalParams, ElementalRegistry, ElementConfig, ElementDefinition, ElementAliases, BaseProperties};
use std::collections::HashMap;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Creating Simple Five Elements Actor Example");
    
    // 1. Create Elemental Registry with Five Elements
    let mut registry = ElementalRegistry::new();
    
    // Register Fire Element (Hỏa)
    let fire_config = create_fire_config();
    registry.register_element("fire".to_string(), fire_config, 0)?;
    
    // Register Water Element (Thủy)
    let water_config = create_water_config();
    registry.register_element("water".to_string(), water_config, 1)?;
    
    // Register Earth Element (Thổ)
    let earth_config = create_earth_config();
    registry.register_element("earth".to_string(), earth_config, 2)?;
    
    // Register Wood Element (Mộc)
    let wood_config = create_wood_config();
    registry.register_element("wood".to_string(), wood_config, 3)?;
    
    // Register Metal Element (Kim)
    let metal_config = create_metal_config();
    registry.register_element("metal".to_string(), metal_config, 4)?;
    
    println!("✅ Registered 5 elements in registry");
    
    // 2. Create Actor Factory
    let factory = ActorFactory::new(Arc::new(registry));
    
    // 3. Create Five Elements Parameters
    let elemental_params = create_five_elements_params();
    
    println!("✅ Created Five Elements Parameters");
    
    // 4. Create Actor with Five Elements
    let actor = factory.create_actor_with_elemental("five_elements_master", elemental_params)?;
    
    println!("✅ Created Five Elements Master Actor");
    println!("Actor ID: {}", actor.get_id());
    println!("Actor Name: {}", actor.get_name());
    
    // 5. Display Elemental System Data (simplified)
    let elemental_data = actor.get_elemental_system().get_data();
    
    println!("\n🔥 Five Elements Master Stats:");
    println!("================================");
    
    // Fire Element (Index 0)
    println!("🔥 Fire (Hỏa) - Primary Element:");
    println!("  Mastery Level: {:.1}", elemental_data.element_mastery_levels[0]);
    println!("  Experience: {:.1}", elemental_data.element_mastery_experience[0]);
    println!("  Qi Amount: {:.1}", elemental_data.element_qi_amounts[0]);
    
    // Water Element (Index 1)
    println!("\n💧 Water (Thủy) - Secondary Element:");
    println!("  Mastery Level: {:.1}", elemental_data.element_mastery_levels[1]);
    println!("  Experience: {:.1}", elemental_data.element_mastery_experience[1]);
    println!("  Qi Amount: {:.1}", elemental_data.element_qi_amounts[1]);
    
    // Earth Element (Index 2)
    println!("\n🌍 Earth (Thổ) - Support Element:");
    println!("  Mastery Level: {:.1}", elemental_data.element_mastery_levels[2]);
    println!("  Experience: {:.1}", elemental_data.element_mastery_experience[2]);
    println!("  Qi Amount: {:.1}", elemental_data.element_qi_amounts[2]);
    
    // Wood Element (Index 3)
    println!("\n🌿 Wood (Mộc) - Support Element:");
    println!("  Mastery Level: {:.1}", elemental_data.element_mastery_levels[3]);
    println!("  Experience: {:.1}", elemental_data.element_mastery_experience[3]);
    println!("  Qi Amount: {:.1}", elemental_data.element_qi_amounts[3]);
    
    // Metal Element (Index 4)
    println!("\n⚔️ Metal (Kim) - Support Element:");
    println!("  Mastery Level: {:.1}", elemental_data.element_mastery_levels[4]);
    println!("  Experience: {:.1}", elemental_data.element_mastery_experience[4]);
    println!("  Qi Amount: {:.1}", elemental_data.element_qi_amounts[4]);
    
    // 6. Display Total Stats
    println!("\n📊 Total Elemental Stats:");
    println!("=========================");
    let total_mastery: f64 = elemental_data.element_mastery_levels.iter().sum();
    let total_experience: f64 = elemental_data.element_mastery_experience.iter().sum();
    let total_qi: f64 = elemental_data.element_qi_amounts.iter().sum();
    
    println!("  Total Mastery Level: {:.1}", total_mastery);
    println!("  Total Experience: {:.1}", total_experience);
    println!("  Total Qi Amount: {:.1}", total_qi);
    
    println!("\n🎉 Five Elements Actor created successfully!");
    
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
    }
}

fn create_water_config() -> ElementConfig {
    ElementConfig {
        version: 1,
        element: ElementDefinition {
            id: "water".to_string(),
            name: "Water".to_string(),
            aliases: ElementAliases {
                vi: Some("Thủy".to_string()),
                zh_pinyin: Some("shui".to_string()),
            },
            category: "five_elements".to_string(),
            description: "Water element - represents flow, wisdom, and healing".to_string(),
            base_properties: BaseProperties {
                base_damage: 100.0,
                base_defense: 90.0,
                base_crit_rate: 0.15,
                base_crit_damage: 1.5,
                base_accuracy: 0.88,
            },
            probability_overrides: HashMap::new(),
            derived_stats: vec!["water_mastery".to_string(), "water_resistance".to_string()],
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

fn create_earth_config() -> ElementConfig {
    ElementConfig {
        version: 1,
        element: ElementDefinition {
            id: "earth".to_string(),
            name: "Earth".to_string(),
            aliases: ElementAliases {
                vi: Some("Thổ".to_string()),
                zh_pinyin: Some("tu".to_string()),
            },
            category: "five_elements".to_string(),
            description: "Earth element - represents stability, endurance, and protection".to_string(),
            base_properties: BaseProperties {
                base_damage: 90.0,
                base_defense: 130.0,
                base_crit_rate: 0.12,
                base_crit_damage: 1.4,
                base_accuracy: 0.85,
            },
            probability_overrides: HashMap::new(),
            derived_stats: vec!["earth_mastery".to_string(), "earth_resistance".to_string()],
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

fn create_wood_config() -> ElementConfig {
    ElementConfig {
        version: 1,
        element: ElementDefinition {
            id: "wood".to_string(),
            name: "Wood".to_string(),
            aliases: ElementAliases {
                vi: Some("Mộc".to_string()),
                zh_pinyin: Some("mu".to_string()),
            },
            category: "five_elements".to_string(),
            description: "Wood element - represents growth, vitality, and nature".to_string(),
            base_properties: BaseProperties {
                base_damage: 110.0,
                base_defense: 80.0,
                base_crit_rate: 0.16,
                base_crit_damage: 1.55,
                base_accuracy: 0.87,
            },
            probability_overrides: HashMap::new(),
            derived_stats: vec!["wood_mastery".to_string(), "wood_resistance".to_string()],
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

fn create_metal_config() -> ElementConfig {
    ElementConfig {
        version: 1,
        element: ElementDefinition {
            id: "metal".to_string(),
            name: "Metal".to_string(),
            aliases: ElementAliases {
                vi: Some("Kim".to_string()),
                zh_pinyin: Some("jin".to_string()),
            },
            category: "five_elements".to_string(),
            description: "Metal element - represents strength, precision, and cutting power".to_string(),
            base_properties: BaseProperties {
                base_damage: 130.0,
                base_defense: 100.0,
                base_crit_rate: 0.20,
                base_crit_damage: 1.7,
                base_accuracy: 0.90,
            },
            probability_overrides: HashMap::new(),
            derived_stats: vec!["metal_mastery".to_string(), "metal_resistance".to_string()],
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

fn create_five_elements_params() -> ElementalParams {
    let mut mastery_levels = HashMap::new();
    mastery_levels.insert("fire".to_string(), 25.0);    // Primary element - highest mastery
    mastery_levels.insert("water".to_string(), 18.0);   // Secondary element
    mastery_levels.insert("earth".to_string(), 15.0);   // Support element
    mastery_levels.insert("wood".to_string(), 12.0);    // Support element
    mastery_levels.insert("metal".to_string(), 10.0);   // Support element
    
    let mut experience = HashMap::new();
    experience.insert("fire".to_string(), 2500.0);      // Fire experience
    experience.insert("water".to_string(), 1800.0);     // Water experience
    experience.insert("earth".to_string(), 1500.0);     // Earth experience
    experience.insert("wood".to_string(), 1200.0);      // Wood experience
    experience.insert("metal".to_string(), 1000.0);     // Metal experience
    
    let mut qi_amounts = HashMap::new();
    qi_amounts.insert("fire".to_string(), 2500.0);      // Fire qi
    qi_amounts.insert("water".to_string(), 2200.0);     // Water qi
    qi_amounts.insert("earth".to_string(), 2000.0);     // Earth qi
    qi_amounts.insert("wood".to_string(), 1800.0);      // Wood qi
    qi_amounts.insert("metal".to_string(), 1600.0);     // Metal qi
    
    ElementalParams {
        primary_element: "fire".to_string(),
        initial_mastery_levels: mastery_levels,
        initial_experience: experience,
        initial_qi_amounts: qi_amounts,
        elemental_preferences: vec![
            "fire".to_string(),    // Primary
            "water".to_string(),   // Secondary
            "earth".to_string(),   // Tertiary
            "wood".to_string(),    // Quaternary
            "metal".to_string(),   // Quinary
        ],
    }
}
