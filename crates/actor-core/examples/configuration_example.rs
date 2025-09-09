//! Configuration Example
//! 
//! This example demonstrates how to use configuration files with Actor Core.

use actor_core::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Actor Core - Configuration Example");
    println!("====================================");

    // Load cap layers configuration
    println!("📋 Loading cap layers configuration...");
    let cap_layers = registry::loader::load_cap_layers("configs/cap_layers.yaml")?;
    println!("✅ Loaded cap layers configuration");

    // Display cap layers
    println!("\n📊 Cap Layers:");
    println!("==============");
    let layer_order = cap_layers.get_layer_order();
    for (i, layer_name) in layer_order.iter().enumerate() {
        println!("   {}. {} (loaded from config)", i + 1, layer_name);
    }

    // Load combiner configuration
    println!("\n📋 Loading combiner configuration...");
    let combiner = registry::loader::load_combiner("configs/combiner.yaml")?;
    println!("✅ Loaded combiner configuration");

    // Display combiner rules
    println!("\n📊 Combiner Rules:");
    println!("==================");
    let rule_names = ["attack", "defense", "magic", "social"];
    for rule_name in &rule_names {
        if let Some(rule) = combiner.get_rule(rule_name) {
            println!("   {}: (clamp: {:?})",
                     rule_name, rule.clamp_default);
        }
    }

    // Validate configurations
    println!("\n🔍 Validating configurations...");
    cap_layers.validate()?;
    combiner.validate()?;
    println!("✅ All configurations are valid");

    // Create an actor with configuration-based processing
    let mut actor = Actor::new("Configured Player".to_string(), "Human".to_string());
    
    // Set actor data
    let mut data = std::collections::HashMap::new();
    data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(20)));
    data.insert("class".to_string(), serde_json::Value::String("paladin".to_string()));
    actor.set_data(data);

    // Add some buffs
    actor.add_buff("divine_protection".to_string());
    actor.add_buff("holy_aura".to_string());
    actor.add_buff("combat_training".to_string());

    // Set combat status
    actor.set_combat_duration(180);

    // Set guild
    actor.set_guild_id("Order of the Light".to_string());

    println!("\n👤 Actor Configuration:");
    println!("=======================");
    println!("   Name: {}", actor.get_name());
    println!("   Race: {}", actor.get_race());
    println!("   Level: {}", actor.get_data().get("level").unwrap_or(&serde_json::Value::Null));
    println!("   Class: {}", actor.get_data().get("class").unwrap_or(&serde_json::Value::Null));
    println!("   Buffs: {:?}", actor.get_active_buffs());
    println!("   Combat Duration: {} seconds", actor.get_combat_duration().unwrap_or(0));
    println!("   Guild: {}", actor.get_guild_id().unwrap_or("None"));

    // Create contributions based on configuration
    let mut contributions = Vec::new();

    // Attack contributions (based on attack rule)
    if let Some(_attack_rule) = combiner.get_rule("attack") {
        println!("\n⚔️  Attack Contributions:");
        
        // Flat contributions (equipment)
        contributions.push(Contribution::new(
            "attack_power".to_string(),
            Bucket::Flat,
            25.0,
            "sword".to_string()
        ));
        contributions.push(Contribution::new(
            "attack_power".to_string(),
            Bucket::Flat,
            10.0,
            "armor".to_string()
        ));

        // Mult contributions (buffs)
        contributions.push(Contribution::new(
            "attack_power".to_string(),
            Bucket::Mult,
            1.3,
            "divine_protection".to_string()
        ));
        contributions.push(Contribution::new(
            "attack_power".to_string(),
            Bucket::Mult,
            1.1,
            "combat_training".to_string()
        ));

        // PostAdd contributions (talents)
        contributions.push(Contribution::new(
            "attack_power".to_string(),
            Bucket::PostAdd,
            15.0,
            "weapon_mastery".to_string()
        ));

        // Override contributions (special abilities)
        contributions.push(Contribution::new(
            "attack_power".to_string(),
            Bucket::Override,
            200.0,
            "divine_smite".to_string()
        ));
    }

    // Defense contributions (based on defense rule)
    if let Some(_defense_rule) = combiner.get_rule("defense") {
        println!("\n🛡️  Defense Contributions:");
        
        // Flat contributions
        contributions.push(Contribution::new(
            "defense".to_string(),
            Bucket::Flat,
            30.0,
            "shield".to_string()
        ));
        contributions.push(Contribution::new(
            "defense".to_string(),
            Bucket::Flat,
            20.0,
            "armor".to_string()
        ));

        // Mult contributions
        contributions.push(Contribution::new(
            "defense".to_string(),
            Bucket::Mult,
            1.2,
            "holy_aura".to_string()
        ));

        // PostAdd contributions
        contributions.push(Contribution::new(
            "defense".to_string(),
            Bucket::PostAdd,
            10.0,
            "defensive_stance".to_string()
        ));
    }

    // Magic contributions (based on magic rule)
    if let Some(_magic_rule) = combiner.get_rule("magic") {
        println!("\n🔮 Magic Contributions:");
        
        // Flat contributions
        contributions.push(Contribution::new(
            "magic_power".to_string(),
            Bucket::Flat,
            40.0,
            "staff".to_string()
        ));

        // Mult contributions
        contributions.push(Contribution::new(
            "magic_power".to_string(),
            Bucket::Mult,
            1.5,
            "divine_protection".to_string()
        ));

        // PostAdd contributions
        contributions.push(Contribution::new(
            "magic_power".to_string(),
            Bucket::PostAdd,
            25.0,
            "holy_magic".to_string()
        ));
    }

    // Social contributions (based on social rule)
    if let Some(_social_rule) = combiner.get_rule("social") {
        println!("\n👥 Social Contributions:");
        
        // Flat contributions
        contributions.push(Contribution::new(
            "charisma".to_string(),
            Bucket::Flat,
            15.0,
            "guild_leadership".to_string()
        ));

        // Mult contributions
        contributions.push(Contribution::new(
            "charisma".to_string(),
            Bucket::Mult,
            1.1,
            "holy_aura".to_string()
        ));

        // PostAdd contributions
        contributions.push(Contribution::new(
            "charisma".to_string(),
            Bucket::PostAdd,
            5.0,
            "diplomatic_training".to_string()
        ));
    }

    // Display all contributions
    println!("\n📋 All Contributions:");
    println!("====================");
    for (i, contrib) in contributions.iter().enumerate() {
        println!("   {}. {}: {:.2} (bucket: {:?}, source: {})", 
                 i + 1, contrib.dimension, contrib.value, contrib.bucket, contrib.system);
    }

    // Process contributions by dimension
    println!("\n🔄 Processing Contributions:");
    println!("===========================");

    let dimensions = ["attack_power", "defense", "magic_power", "charisma"];
    let mut final_stats = std::collections::HashMap::new();

    for dimension in &dimensions {
        let dimension_contribs: Vec<_> = contributions.iter()
            .filter(|c| c.dimension == *dimension)
            .cloned()
            .collect();

        if !dimension_contribs.is_empty() {
            // Get caps for this dimension from cap layers
            let mut caps = None;
            for layer_name in cap_layers.get_layer_order() {
                if let Some(_layer) = cap_layers.get_layer_order().iter().find(|l| **l == *layer_name) {
                    // In a real implementation, you would check if this layer applies to this dimension
                    // For this example, we'll use a simple approach
                    if *dimension == "attack_power" || *dimension == "defense" {
                        caps = Some(Caps::new(0.0, 200.0));
                        break;
                    } else if *dimension == "magic_power" {
                        caps = Some(Caps::new(0.0, 150.0));
                        break;
                    } else if *dimension == "charisma" {
                        caps = Some(Caps::new(0.0, 100.0));
                        break;
                    }
                }
            }

            let result = bucket_processor::process_contributions_in_order(
                dimension_contribs,
                0.0, // Base value
                caps.as_ref()
            )?;

            final_stats.insert(*dimension, result);
            println!("   {}: {:.2}", dimension, result);
        }
    }

    // Create final snapshot
    let mut snapshot = Snapshot::new(actor.get_id().clone(), 1);
    
    for (dimension, value) in &final_stats {
        let contrib = Contribution::new(
            dimension.to_string(),
            Bucket::Flat,
            *value,
            "final_calculation".to_string()
        );
        snapshot.add_primary(contrib);
    }

    // Display final results
    println!("\n📊 Final Results:");
    println!("=================");
    println!("Actor: {}", actor.get_name());
    println!("Race: {}", actor.get_race());
    println!("Level: {}", actor.get_data().get("level").unwrap_or(&serde_json::Value::Null));
    println!("Class: {}", actor.get_data().get("class").unwrap_or(&serde_json::Value::Null));
    
    for dimension in &dimensions {
        if let Some(&value) = final_stats.get(dimension) {
            println!("{}: {:.2}", dimension, value);
        }
    }

    println!("\n🎉 Configuration example completed successfully!");
    Ok(())
}
