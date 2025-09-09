//! Resource Manager Example
//! 
//! This example demonstrates how to use the Resource Manager Subsystem
//! to calculate and manage character resources in the Chaos World MMORPG.

use actor_core::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Resource Manager Example");
    println!("==========================");

    // Create a Resource Manager Subsystem
    let resource_manager = ResourceManagerSubsystem::new();
    println!("✅ Created Resource Manager Subsystem");

    // Create different types of actors
    let actors = vec![
        ("Human Warrior", "Human", false),
        ("Elf Mage", "Elf", true),
        ("Dwarf Fighter", "Dwarf", false),
        ("Orc Berserker", "Orc", true),
    ];

    for (name, race, has_cultivation) in actors {
        println!("\n--- {} ---", name);
        
        // Create actor
        let mut actor = Actor::new(name.to_string(), race.to_string());
        
        // Set some basic data
        let mut data = HashMap::new();
        data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(10)));
        data.insert("class".to_string(), serde_json::Value::String("warrior".to_string()));
        actor.set_data(data);
        
        // Add cultivation subsystem if applicable
        if has_cultivation {
            actor.add_subsystem(SubsystemStruct::new("jindan_system".to_string(), 100));
            println!("  Added cultivation subsystem");
        }
        
        // Set lifespan and age
        actor.set_lifespan(100);
        actor.set_age(25);
        
        // Get resource contributions from the Resource Manager
        match resource_manager.contribute(&actor).await {
            Ok(output) => {
                println!("  System ID: {}", output.meta.system);
                println!("  Primary Resources:");
                
                // Display primary resources
                for contribution in &output.primary {
                    println!("    {}: {:.1}", contribution.dimension, contribution.value);
                }
                
                // Display derived resources
                if !output.derived.is_empty() {
                    println!("  Derived Resources:");
                    for contribution in &output.derived {
                        println!("    {}: {:.1}%", contribution.dimension, contribution.value);
                    }
                }
                
                // Display caps
                if !output.caps.is_empty() {
                    println!("  Resource Caps:");
                    for cap in &output.caps {
                        println!("    {} {}: {:.1}", cap.dimension, cap.kind, cap.value);
                    }
                }
            }
            Err(e) => {
                println!("  Error calculating resources: {}", e);
            }
        }
    }

    // Demonstrate resource calculation with different scenarios
    println!("\n--- Resource Calculation Scenarios ---");
    
    // Scenario 1: Young Human without cultivation
    let mut young_human = Actor::new("Young Human".to_string(), "Human".to_string());
    young_human.set_lifespan(50);
    young_human.set_age(20);
    
    if let Ok(output) = resource_manager.contribute(&young_human).await {
        println!("Young Human (50 lifespan, 20 age):");
        for contribution in &output.primary {
            if contribution.dimension.starts_with("hp") {
                println!("  {}: {:.1}", contribution.dimension, contribution.value);
            }
        }
    }
    
    // Scenario 2: Old Elf with cultivation
    let mut old_elf = Actor::new("Old Elf".to_string(), "Elf".to_string());
    old_elf.set_lifespan(200);
    old_elf.set_age(150);
    old_elf.add_subsystem(SubsystemStruct::new("jindan_system".to_string(), 100));
    
    if let Ok(output) = resource_manager.contribute(&old_elf).await {
        println!("Old Elf with Cultivation (200 lifespan, 150 age):");
        for contribution in &output.primary {
            if contribution.dimension.starts_with("hp") || contribution.dimension.starts_with("mana") {
                println!("  {}: {:.1}", contribution.dimension, contribution.value);
            }
        }
    }

    // Demonstrate bucket processing
    println!("\n--- Bucket Processing Example ---");
    
    // Create contributions manually to show bucket processing
    let contributions = vec![
        Contribution::new("hp_current".to_string(), Bucket::Flat, 1000.0, "baseline".to_string()),
        Contribution::new("hp_current".to_string(), Bucket::Flat, -50.0, "damage".to_string()),
        Contribution::new("hp_current".to_string(), Bucket::Flat, 30.0, "healing".to_string()),
    ];
    
    let result = bucket_processor::process_contributions_in_order(
        contributions,
        0.0,
        None
    );
    
    match result {
        Ok(final_hp) => {
            println!("HP calculation: 1000 - 50 + 30 = {:.1}", final_hp);
        }
        Err(e) => {
            println!("Error in bucket processing: {}", e);
        }
    }

    println!("\n🎉 Resource Manager Example Complete!");
    Ok(())
}
