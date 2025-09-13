//! Basic Usage Example
//! 
//! This example demonstrates the basic usage of Actor Core with the new prelude-based API.

use actor_core::prelude::*;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    println!("🎮 Actor Core - Basic Usage Example");
    println!("===================================");

    // Quick setup with default configurations
    let (aggregator, _cache) = quick_setup().await?;
    println!("✅ Initialized Actor Core services");

    // Create a simple actor with level
    let mut actor = create_simple_actor("Player1", "Human", 10);
    println!("✅ Created actor: {}", actor.get_name());

    // Add additional data
    let mut data = HashMap::new();
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

    // Create contributions using convenience functions
    let contributions = vec![
        create_basic_contribution("strength", 10.0, "equipment"),
        Contribution::new("strength".to_string(), Bucket::Mult, 1.2, "buff".to_string()),
        Contribution::new("strength".to_string(), Bucket::PostAdd, 5.0, "talent".to_string()),
        create_basic_contribution("health", 50.0, "equipment"),
        Contribution::new("health".to_string(), Bucket::Mult, 1.5, "buff".to_string()),
    ];
    println!("✅ Created {} contributions", contributions.len());

    // Process contributions using the aggregator
    let snapshot = aggregator.resolve(&actor).await?;
    println!("✅ Generated actor snapshot");

    // Create caps using convenience function
    let strength_caps = create_basic_caps(0.0, 100.0);
    let health_caps = create_basic_caps(0.0, 1000.0);
    println!("✅ Created caps for strength and health");

    // Display final results
    println!("\n📊 Final Results:");
    println!("==================");
    println!("Actor: {}", actor.get_name());
    println!("Race: {}", actor.get_race());
    println!("Level: {}", actor.get_data().get("level").unwrap_or(&serde_json::Value::Null));
    println!("Class: {}", actor.get_data().get("class").unwrap_or(&serde_json::Value::Null));
    println!("Experience: {}", actor.get_data().get("experience").unwrap_or(&serde_json::Value::Null));
    println!("Snapshot Version: {}", snapshot.version);
    println!("Buffs: {:?}", actor.get_active_buffs());
    println!("Guild: {}", actor.get_guild_id().unwrap_or("None"));
    println!("Combat Duration: {} seconds", actor.get_combat_duration().unwrap_or(0));

    // Show build information
    let build_info = get_build_info();
    println!("\n🔧 Build Information:");
    println!("====================");
    println!("Version: {}", build_info.version);
    println!("MSRV: {}", build_info.msrv);
    println!("Features: {:?}", build_info.features);

    println!("\n🎉 Example completed successfully!");
    Ok(())
}
