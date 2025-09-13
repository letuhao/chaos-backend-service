//! Subsystem Example
//! 
//! This example demonstrates how to create and use custom subsystems with the new prelude-based API.

use actor_core::prelude::*;

// Custom Combat Subsystem
struct CombatSubsystem {
    system_id: String,
    priority: i64,
}

impl CombatSubsystem {
    fn new() -> Self {
        Self {
            system_id: "combat".to_string(),
            priority: 100,
        }
    }
}

#[async_trait]
impl Subsystem for CombatSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }

    fn priority(&self) -> i64 {
        self.priority
    }

    async fn contribute(&self, actor: &Actor) -> Result<SubsystemOutput, ActorCoreError> {
        println!("⚔️  Combat subsystem processing actor: {}", actor.get_name());

        // Check if actor is in combat
        let is_in_combat = actor.get_combat_duration().is_some();
        
        let mut caps = HashMap::new();
        let mut context = HashMap::new();
        let mut meta = HashMap::new();

        if is_in_combat {
            // Combat bonuses
            caps.insert("strength".to_string(), Caps::new(0.0, 200.0)); // Higher caps in combat
            caps.insert("health".to_string(), Caps::new(0.0, 2000.0));
            
            context.insert("combat_bonus".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(1.2).unwrap()));
            context.insert("combat_duration".to_string(), serde_json::Value::Number(serde_json::Number::from(actor.get_combat_duration().unwrap_or(0))));
            
            meta.insert("subsystem".to_string(), serde_json::Value::String("combat".to_string()));
            meta.insert("processing_time".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
        } else {
            // Normal caps
            caps.insert("strength".to_string(), Caps::new(0.0, 100.0));
            caps.insert("health".to_string(), Caps::new(0.0, 1000.0));
            
            meta.insert("subsystem".to_string(), serde_json::Value::String("combat".to_string()));
            meta.insert("processing_time".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
        }

        Ok(SubsystemOutput {
            caps: caps.into_iter().map(|(k, v)| CapContribution::new(
                "combat".to_string(),
                k,
                CapMode::HardMax,
                "combat_bonus".to_string(),
                v.get_max()
            )).collect(),
            context: Some(ModifierPack::new()),
            meta: SubsystemMeta::new("combat".to_string()),
            primary: vec![],
            derived: vec![],
        })
    }
}

// Custom Magic Subsystem
struct MagicSubsystem {
    system_id: String,
    priority: i64,
}

impl MagicSubsystem {
    fn new() -> Self {
        Self {
            system_id: "magic".to_string(),
            priority: 200,
        }
    }
}

#[async_trait]
impl Subsystem for MagicSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }

    fn priority(&self) -> i64 {
        self.priority
    }

    async fn contribute(&self, actor: &Actor) -> Result<SubsystemOutput, ActorCoreError> {
        println!("🔮 Magic subsystem processing actor: {}", actor.get_name());

        let mut caps = HashMap::new();
        let mut context = HashMap::new();
        let mut meta = HashMap::new();

        // Check if actor has magic-related buffs
        let has_magic_buffs = actor.get_active_buffs().iter().any(|buff| 
            buff.contains("magic") || buff.contains("mana") || buff.contains("spell")
        );

        if has_magic_buffs {
            caps.insert("intelligence".to_string(), Caps::new(0.0, 150.0));
            caps.insert("mana".to_string(), Caps::new(0.0, 500.0));
            
            context.insert("magic_affinity".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(1.5).unwrap()));
            context.insert("mana_regeneration".to_string(), serde_json::Value::Number(serde_json::Number::from(2)));
            
            meta.insert("subsystem".to_string(), serde_json::Value::String("magic".to_string()));
            meta.insert("magic_enhanced".to_string(), serde_json::Value::Bool(true));
        } else {
            caps.insert("intelligence".to_string(), Caps::new(0.0, 100.0));
            caps.insert("mana".to_string(), Caps::new(0.0, 200.0));
            
            meta.insert("subsystem".to_string(), serde_json::Value::String("magic".to_string()));
            meta.insert("magic_enhanced".to_string(), serde_json::Value::Bool(false));
        }

        Ok(SubsystemOutput {
            caps: caps.into_iter().map(|(k, v)| CapContribution::new(
                "combat".to_string(),
                k,
                CapMode::HardMax,
                "combat_bonus".to_string(),
                v.get_max()
            )).collect(),
            context: Some(ModifierPack::new()),
            meta: SubsystemMeta::new("combat".to_string()),
            primary: vec![],
            derived: vec![],
        })
    }
}

// Custom Social Subsystem
struct SocialSubsystem {
    system_id: String,
    priority: i64,
}

impl SocialSubsystem {
    fn new() -> Self {
        Self {
            system_id: "social".to_string(),
            priority: 300,
        }
    }
}

#[async_trait]
impl Subsystem for SocialSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }

    fn priority(&self) -> i64 {
        self.priority
    }

    async fn contribute(&self, actor: &Actor) -> Result<SubsystemOutput, ActorCoreError> {
        println!("👥 Social subsystem processing actor: {}", actor.get_name());

        let mut caps = HashMap::new();
        let mut context = HashMap::new();
        let mut meta = HashMap::new();

        // Check if actor is in a guild
        let has_guild = actor.get_guild_id().is_some();
        
        if has_guild {
            caps.insert("charisma".to_string(), Caps::new(0.0, 120.0));
            caps.insert("reputation".to_string(), Caps::new(0.0, 1000.0));
            
            context.insert("guild_bonus".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(1.1).unwrap()));
            context.insert("guild_name".to_string(), serde_json::Value::String(actor.get_guild_id().unwrap_or("Unknown").to_string()));
            
            meta.insert("subsystem".to_string(), serde_json::Value::String("social".to_string()));
            meta.insert("guild_member".to_string(), serde_json::Value::Bool(true));
        } else {
            caps.insert("charisma".to_string(), Caps::new(0.0, 100.0));
            caps.insert("reputation".to_string(), Caps::new(0.0, 500.0));
            
            meta.insert("subsystem".to_string(), serde_json::Value::String("social".to_string()));
            meta.insert("guild_member".to_string(), serde_json::Value::Bool(false));
        }

        Ok(SubsystemOutput {
            caps: caps.into_iter().map(|(k, v)| CapContribution::new(
                "combat".to_string(),
                k,
                CapMode::HardMax,
                "combat_bonus".to_string(),
                v.get_max()
            )).collect(),
            context: Some(ModifierPack::new()),
            meta: SubsystemMeta::new("combat".to_string()),
            primary: vec![],
            derived: vec![],
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Actor Core - Subsystem Example");
    println!("=================================");

    // Create an actor
    let mut actor = Actor::new("Mage Warrior".to_string(), "Elf".to_string());
    println!("✅ Created actor: {}", actor.get_name());

    // Set actor data
    let mut data = HashMap::new();
    data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(15)));
    data.insert("class".to_string(), serde_json::Value::String("battle_mage".to_string()));
    actor.set_data(data);

    // Add some buffs
    actor.add_buff("magic_enhancement".to_string());
    actor.add_buff("combat_experience".to_string());
    actor.add_buff("guild_leadership".to_string());

    // Set combat status
    actor.set_combat_duration(120);

    // Set guild
    actor.set_guild_id("Arcane Warriors".to_string());

    println!("✅ Actor setup complete");
    println!("   Level: {}", actor.get_data().get("level").unwrap_or(&serde_json::Value::Null));
    println!("   Class: {}", actor.get_data().get("class").unwrap_or(&serde_json::Value::Null));
    println!("   Buffs: {:?}", actor.get_active_buffs());
    println!("   Combat Duration: {} seconds", actor.get_combat_duration().unwrap_or(0));
    println!("   Guild: {}", actor.get_guild_id().unwrap_or("None"));

    // Create subsystems
    let combat_subsystem = CombatSubsystem::new();
    let magic_subsystem = MagicSubsystem::new();
    let social_subsystem = SocialSubsystem::new();

    // Create registry
    let registry = Arc::new(PluginRegistryImpl::new());

    // Register subsystems
    registry.register(Arc::new(combat_subsystem))?;
    registry.register(Arc::new(magic_subsystem))?;
    registry.register(Arc::new(social_subsystem))?;

    println!("✅ Registered {} subsystems", registry.count());

    // Get subsystems by priority
    let subsystems = registry.get_by_priority();
    println!("✅ Subsystems by priority:");
    for subsystem in &subsystems {
        println!("   - {} (priority: {})", subsystem.system_id(), subsystem.priority());
    }

    // Process actor through subsystems
    println!("\n🔄 Processing actor through subsystems:");
    println!("=====================================");

    let mut all_caps = HashMap::new();
    let mut all_context = HashMap::new();
    let mut all_meta = HashMap::new();

    for subsystem in subsystems {
        let output = subsystem.contribute(&actor).await?;
        
        // Merge caps
        for cap_contrib in output.caps {
            all_caps.insert(cap_contrib.dimension, Caps::new(0.0, cap_contrib.value));
        }
        
        // Merge context
        if let Some(context) = output.context {
            for (key, value) in context.additive_percent {
                all_context.insert(key, serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap()));
            }
            for (key, value) in context.multipliers {
                all_context.insert(key, serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap()));
            }
            for (key, value) in context.post_add {
                all_context.insert(key, serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap()));
            }
        }
        
        // Merge meta
        for (key, value) in output.meta.data {
            all_meta.insert(key, value);
        }
    }

    // Display results
    println!("\n📊 Subsystem Processing Results:");
    println!("================================");
    
    println!("Caps:");
    for (dimension, caps) in &all_caps {
        println!("   {}: min={:.2}, max={:.2}", dimension, caps.get_min(), caps.get_max());
    }
    
    println!("\nContext:");
    for (key, value) in &all_context {
        println!("   {}: {}", key, value);
    }
    
    println!("\nMeta:");
    for (key, value) in &all_meta {
        println!("   {}: {}", key, value);
    }

    // Create contributions based on subsystem results
    let mut contributions = Vec::new();
    
    // Combat contributions
    if all_context.contains_key("combat_bonus") {
        let bonus = all_context["combat_bonus"].as_f64().unwrap_or(1.0);
        contributions.push(Contribution::new(
            "strength".to_string(),
            Bucket::Mult,
            bonus,
            "combat_subsystem".to_string()
        ));
    }
    
    // Magic contributions
    if all_context.contains_key("magic_affinity") {
        let affinity = all_context["magic_affinity"].as_f64().unwrap_or(1.0);
        contributions.push(Contribution::new(
            "intelligence".to_string(),
            Bucket::Mult,
            affinity,
            "magic_subsystem".to_string()
        ));
    }
    
    // Social contributions
    if all_context.contains_key("guild_bonus") {
        let bonus = all_context["guild_bonus"].as_f64().unwrap_or(1.0);
        contributions.push(Contribution::new(
            "charisma".to_string(),
            Bucket::Mult,
            bonus,
            "social_subsystem".to_string()
        ));
    }

    println!("\n🎯 Generated Contributions:");
    println!("===========================");
    for contrib in &contributions {
        println!("   {}: {} (bucket: {:?}, source: {})", 
                 contrib.dimension, contrib.value, contrib.bucket, contrib.system);
    }

    // Process contributions
    if !contributions.is_empty() {
        let strength_contribs: Vec<_> = contributions.iter()
            .filter(|c| c.dimension == "strength")
            .cloned()
            .collect();
        
        if !strength_contribs.is_empty() {
            let strength_result = bucket_processor::process_contributions_in_order(
                strength_contribs,
                100.0, // Base strength
                all_caps.get("strength")
            )?;
            println!("\n💪 Final Strength: {:.2}", strength_result);
        }
    }

    println!("\n🎉 Subsystem example completed successfully!");
    Ok(())
}
