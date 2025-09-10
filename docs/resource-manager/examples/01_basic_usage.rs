//! Basic Usage Example
//!
//! This example demonstrates the basic usage of the Enhanced Hybrid Resource Manager
//! for simple resource calculation and management.

use chaos_backend_service::crates::actor_core::subsystems::*;
use chaos_backend_service::crates::actor_core::types::Actor;
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Enhanced Hybrid Resource Manager - Basic Usage Example");
    
    // 1. Initialize the resource manager
    let resource_manager = setup_resource_manager().await?;
    
    // 2. Create a player character
    let mut player = create_player_character("Hero", "Human", 15, 20, 18, 16, 14);
    
    // 3. Calculate resources
    println!("\n📊 Calculating resources for {}...", player.id);
    let result = resource_manager.contribute(&player).await?;
    
    // 4. Display results
    display_resource_results(&result);
    
    // 5. Simulate stat changes
    println!("\n⚡ Simulating stat changes...");
    simulate_stat_changes(&mut player, &resource_manager).await?;
    
    // 6. Test resource regeneration
    println!("\n🔄 Testing resource regeneration...");
    test_resource_regeneration(&player).await?;
    
    println!("\n✅ Basic usage example completed successfully!");
    Ok(())
}

/// Set up the Enhanced Hybrid Resource Manager with all components
async fn setup_resource_manager() -> Result<Arc<EnhancedHybridResourceManager>, Box<dyn std::error::Error>> {
    println!("🔧 Setting up resource manager...");
    
    // Create cache configuration
    let cache_config = CacheConfig {
        l1_ttl: 300,        // 5 minutes
        l2_ttl: 3600,       // 1 hour
        l3_ttl: 86400,      // 24 hours
        max_l1_size: 10000,
        max_l2_size: 100000,
        warming_enabled: true,
        batch_enabled: true,
    };
    let resource_cache = Arc::new(ResourceCache::new(cache_config));
    
    // Create stat change notifier
    let notifier_config = NotifierConfig::default();
    let stat_notifier = Arc::new(StatChangeNotifier::new(notifier_config));
    
    // Create event manager
    let event_config = EventConfig::default();
    let event_manager = Arc::new(ResourceEventManager::new(event_config));
    
    // Create enhanced hybrid resource manager
    let resource_manager = Arc::new(EnhancedHybridResourceManager::new(
        "basic_example".to_string(),
        100,
        resource_cache,
        stat_notifier,
        event_manager,
    ));
    
    println!("✅ Resource manager setup complete");
    Ok(resource_manager)
}

/// Create a player character with specified stats
fn create_player_character(
    name: &str,
    race: &str,
    level: i32,
    vitality: i32,
    intelligence: i32,
    constitution: i32,
    charisma: i32,
) -> Actor {
    let mut actor = Actor::new(name.to_string(), race.to_string());
    
    let mut data = HashMap::new();
    data.insert("level".to_string(), serde_json::json!(level));
    data.insert("vitality".to_string(), serde_json::json!(vitality));
    data.insert("intelligence".to_string(), serde_json::json!(intelligence));
    data.insert("constitution".to_string(), serde_json::json!(constitution));
    data.insert("charisma".to_string(), serde_json::json!(charisma));
    data.insert("equipment_bonus".to_string(), serde_json::json!(5));
    data.insert("in_combat".to_string(), serde_json::json!(false));
    data.insert("resting".to_string(), serde_json::json!(true));
    data.insert("moving".to_string(), serde_json::json!(false));
    
    actor.set_data(data);
    actor
}

/// Display resource calculation results
fn display_resource_results(result: &crate::ActorCoreResult<crate::subsystems::SubsystemOutput>) {
    match result {
        Ok(output) => {
            println!("📈 Resource Calculation Results:");
            println!("   System ID: {}", output.system_id);
            println!("   Priority: {}", output.priority);
            println!("   Contributions: {} items", output.contributions.len());
            
            for (key, value) in &output.contributions {
                println!("   {}: {}", key, value);
            }
        }
        Err(e) => {
            println!("❌ Error calculating resources: {}", e);
        }
    }
}

/// Simulate stat changes and recalculate resources
async fn simulate_stat_changes(
    player: &mut Actor,
    resource_manager: &Arc<EnhancedHybridResourceManager>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate level up
    let mut data = player.get_data();
    let current_level = data.get("level").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
    data.insert("level".to_string(), serde_json::json!(current_level + 1));
    data.insert("vitality".to_string(), serde_json::json!(25));
    data.insert("intelligence".to_string(), serde_json::json!(22));
    player.set_data(data);
    
    println!("   Level up! New level: {}", current_level + 1);
    
    // Recalculate resources
    let result = resource_manager.contribute(player).await?;
    display_resource_results(&Ok(result));
    
    // Simulate equipment change
    let mut data = player.get_data();
    data.insert("equipment_bonus".to_string(), serde_json::json!(15));
    player.set_data(data);
    
    println!("   Equipment upgraded! New bonus: +15");
    
    // Recalculate resources
    let result = resource_manager.contribute(player).await?;
    display_resource_results(&Ok(result));
    
    Ok(())
}

/// Test resource regeneration system
async fn test_resource_regeneration(player: &Actor) -> Result<(), Box<dyn std::error::Error>> {
    // Create regeneration manager
    let regen_config = RegenerationConfig::default();
    let regen_manager = ResourceRegenerationManager::new(regen_config);
    
    // Start regeneration for HP
    regen_manager.start_regeneration(player, "hp_current").await?;
    println!("   Started HP regeneration");
    
    // Start regeneration for MP
    regen_manager.start_regeneration(player, "mp_current").await?;
    println!("   Started MP regeneration");
    
    // Simulate time passing and update regeneration
    let mut actor_map = HashMap::new();
    actor_map.insert(player.id.to_string(), player.clone());
    
    regen_manager.update_regeneration(&actor_map).await?;
    println!("   Updated regeneration after time passage");
    
    // Get regeneration statistics
    let stats = regen_manager.get_regeneration_stats().await?;
    println!("   Active regeneration tasks: {}", stats.total_tasks);
    println!("   Total regenerated: {:.2}", stats.total_regenerated);
    
    Ok(())
}
