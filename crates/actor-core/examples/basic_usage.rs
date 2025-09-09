//! Basic Usage Example
//! 
//! This example demonstrates the basic usage of Actor Core.

use actor_core::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Actor Core - Basic Usage Example");
    println!("===================================");

    // Create an actor
    let mut actor = Actor::new("Player1".to_string(), "Human".to_string());
    println!("✅ Created actor: {}", actor.get_name());

    // Set actor data
    let mut data = HashMap::new();
    data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(10)));
    data.insert("class".to_string(), serde_json::Value::String("warrior".to_string()));
    data.insert("experience".to_string(), serde_json::Value::Number(serde_json::Number::from(1500)));
    actor.set_data(data);
    println!("✅ Set actor data");

    // Add some buffs
    actor.add_buff("strength_boost".to_string());
    actor.add_buff("health_regeneration".to_string());
    actor.add_buff("combat_experience".to_string());
    println!("✅ Added buffs: {:?}", actor.get_active_buffs());

    // Set combat status
    actor.set_combat_duration(60);
    println!("✅ Set combat duration: {} seconds", actor.get_combat_duration().unwrap_or(0));

    // Set guild
    actor.set_guild_id("Guild of Warriors".to_string());
    println!("✅ Set guild: {}", actor.get_guild_id().unwrap_or("None"));

    // Create contributions
    let contributions = vec![
        Contribution::new("strength".to_string(), Bucket::Flat, 10.0, "equipment".to_string()),
        Contribution::new("strength".to_string(), Bucket::Mult, 1.2, "buff".to_string()),
        Contribution::new("strength".to_string(), Bucket::PostAdd, 5.0, "talent".to_string()),
        Contribution::new("health".to_string(), Bucket::Flat, 50.0, "equipment".to_string()),
        Contribution::new("health".to_string(), Bucket::Mult, 1.5, "buff".to_string()),
    ];
    println!("✅ Created {} contributions", contributions.len());

    // Process contributions
    let strength_result = bucket_processor::process_contributions_in_order(
        contributions.iter().filter(|c| c.dimension == "strength").cloned().collect(),
        0.0,
        None
    )?;
    println!("✅ Processed strength contributions: {:.2}", strength_result);

    let health_result = bucket_processor::process_contributions_in_order(
        contributions.iter().filter(|c| c.dimension == "health").cloned().collect(),
        0.0,
        None
    )?;
    println!("✅ Processed health contributions: {:.2}", health_result);

    // Create caps
    let strength_caps = Caps::new(0.0, 100.0);
    let health_caps = Caps::new(0.0, 1000.0);

    // Apply caps
    let clamped_strength = strength_caps.clamp(strength_result);
    let clamped_health = health_caps.clamp(health_result);
    println!("✅ Applied caps - Strength: {:.2}, Health: {:.2}", clamped_strength, clamped_health);

    // Create a snapshot
    let mut snapshot = Snapshot::new(actor.get_id().clone(), 1);
    
    // Add primary stats
    let strength_contrib = Contribution::new("strength".to_string(), Bucket::Flat, clamped_strength, "final".to_string());
    let health_contrib = Contribution::new("health".to_string(), Bucket::Flat, clamped_health, "final".to_string());
    
    snapshot.add_primary(strength_contrib);
    snapshot.add_derived(health_contrib);
    
    println!("✅ Created snapshot with version {}", snapshot.version);

    // Display final results
    println!("\n📊 Final Results:");
    println!("==================");
    println!("Actor: {}", actor.get_name());
    println!("Race: {}", actor.get_race());
    println!("Level: {}", actor.get_data().get("level").unwrap_or(&serde_json::Value::Null));
    println!("Class: {}", actor.get_data().get("class").unwrap_or(&serde_json::Value::Null));
    println!("Experience: {}", actor.get_data().get("experience").unwrap_or(&serde_json::Value::Null));
    println!("Strength: {:.2}", snapshot.get_primary("strength").unwrap_or(0.0));
    println!("Health: {:.2}", snapshot.get_derived("health").unwrap_or(0.0));
    println!("Buffs: {:?}", actor.get_active_buffs());
    println!("Guild: {}", actor.get_guild_id().unwrap_or("None"));
    println!("Combat Duration: {} seconds", actor.get_combat_duration().unwrap_or(0));

    println!("\n🎉 Example completed successfully!");
    Ok(())
}
