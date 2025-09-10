//! Advanced Usage Example
//!
//! This example demonstrates advanced features of the Enhanced Hybrid Resource Manager
//! including multiple resource systems, event handling, and performance monitoring.

use chaos_backend_service::crates::actor_core::subsystems::*;
use chaos_backend_service::crates::actor_core::types::Actor;
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Enhanced Hybrid Resource Manager - Advanced Usage Example");
    
    // 1. Set up comprehensive resource management system
    let (resource_manager, event_manager, performance_monitor) = setup_advanced_system().await?;
    
    // 2. Create multiple characters with different builds
    let characters = create_diverse_characters();
    
    // 3. Test resource calculation for all characters
    println!("\n📊 Testing resource calculation for all characters...");
    for character in &characters {
        test_character_resources(&resource_manager, character).await?;
    }
    
    // 4. Set up event handling
    println!("\n📡 Setting up event handling...");
    setup_event_handling(&event_manager).await?;
    
    // 5. Simulate combat scenario
    println!("\n⚔️ Simulating combat scenario...");
    simulate_combat_scenario(&resource_manager, &event_manager, &characters).await?;
    
    // 6. Performance testing
    println!("\n📈 Running performance tests...");
    run_performance_tests(&performance_monitor, &characters).await?;
    
    // 7. Resource regeneration simulation
    println!("\n🔄 Simulating resource regeneration over time...");
    simulate_regeneration_over_time(&resource_manager, &characters).await?;
    
    println!("\n✅ Advanced usage example completed successfully!");
    Ok(())
}

/// Set up advanced resource management system with all components
async fn setup_advanced_system() -> Result<
    (Arc<EnhancedHybridResourceManager>, Arc<ResourceEventManager>, Arc<PerformanceMonitor>),
    Box<dyn std::error::Error>
> {
    println!("🔧 Setting up advanced resource management system...");
    
    // Create cache with optimized configuration
    let cache_config = CacheConfig {
        l1_ttl: 600,        // 10 minutes
        l2_ttl: 7200,       // 2 hours
        l3_ttl: 86400,      // 24 hours
        max_l1_size: 50000,
        max_l2_size: 500000,
        warming_enabled: true,
        batch_enabled: true,
    };
    let resource_cache = Arc::new(ResourceCache::new(cache_config));
    
    // Create stat change notifier with dependency tracking
    let notifier_config = NotifierConfig {
        max_history_size: 50000,
        enable_batching: true,
        batch_timeout_ms: 50,
        enable_dependency_tracking: true,
    };
    let stat_notifier = Arc::new(StatChangeNotifier::new(notifier_config));
    
    // Create event manager with comprehensive configuration
    let event_config = EventConfig {
        max_history_size: 100000,
        enable_batching: true,
        batch_size: 1000,
        batch_timeout_ms: 100,
        enable_filtering: true,
        enable_persistence: true,
        enable_monitoring: true,
    };
    let event_manager = Arc::new(ResourceEventManager::new(event_config));
    
    // Create enhanced hybrid resource manager
    let mut resource_manager = EnhancedHybridResourceManager::new(
        "advanced_system".to_string(),
        100,
        resource_cache,
        stat_notifier,
        event_manager.clone(),
    );
    
    // Add multiple resource systems
    let rpg_manager = RpgResourceManager::new();
    let magic_manager = MagicResourceManager::new();
    let jindan_manager = JindanSystemResourceCalculator::new();
    
    resource_manager.add_system_manager(Box::new(rpg_manager));
    resource_manager.add_system_manager(Box::new(magic_manager));
    resource_manager.add_system_manager(Box::new(jindan_manager));
    
    let resource_manager = Arc::new(resource_manager);
    
    // Create performance monitor
    let performance_config = PerformanceConfig {
        enable_monitoring: true,
        collection_interval: 30, // 30 seconds
        max_metrics_history: 10000,
        enable_automatic_testing: true,
        test_interval: 300, // 5 minutes
    };
    let performance_monitor = Arc::new(PerformanceMonitor::new(performance_config));
    
    println!("✅ Advanced system setup complete");
    Ok((resource_manager, event_manager, performance_monitor))
}

/// Create diverse characters with different builds
fn create_diverse_characters() -> Vec<Actor> {
    let mut characters = Vec::new();
    
    // Warrior - High vitality and constitution
    let mut warrior = Actor::new("Warrior".to_string(), "Human".to_string());
    let mut data = HashMap::new();
    data.insert("level".to_string(), serde_json::json!(20));
    data.insert("vitality".to_string(), serde_json::json!(35));
    data.insert("intelligence".to_string(), serde_json::json!(10));
    data.insert("constitution".to_string(), serde_json::json!(40));
    data.insert("charisma".to_string(), serde_json::json!(15));
    data.insert("equipment_bonus".to_string(), serde_json::json!(20));
    data.insert("class".to_string(), serde_json::json!("Warrior"));
    warrior.set_data(data);
    characters.push(warrior);
    
    // Mage - High intelligence and wisdom
    let mut mage = Actor::new("Mage".to_string(), "Elf".to_string());
    let mut data = HashMap::new();
    data.insert("level".to_string(), serde_json::json!(18));
    data.insert("vitality".to_string(), serde_json::json!(15));
    data.insert("intelligence".to_string(), serde_json::json!(45));
    data.insert("wisdom".to_string(), serde_json::json!(40));
    data.insert("constitution".to_string(), serde_json::json!(12));
    data.insert("charisma".to_string(), serde_json::json!(25));
    data.insert("equipment_bonus".to_string(), serde_json::json!(15));
    data.insert("class".to_string(), serde_json::json!("Mage"));
    mage.set_data(data);
    characters.push(mage);
    
    // Paladin - Balanced stats
    let mut paladin = Actor::new("Paladin".to_string(), "Human".to_string());
    let mut data = HashMap::new();
    data.insert("level".to_string(), serde_json::json!(22));
    data.insert("vitality".to_string(), serde_json::json!(30));
    data.insert("intelligence".to_string(), serde_json::json!(20));
    data.insert("wisdom".to_string(), serde_json::json!(35));
    data.insert("constitution".to_string(), serde_json::json!(28));
    data.insert("charisma".to_string(), serde_json::json!(32));
    data.insert("equipment_bonus".to_string(), serde_json::json!(18));
    data.insert("class".to_string(), serde_json::json!("Paladin"));
    paladin.set_data(data);
    characters.push(paladin);
    
    // Rogue - High dexterity and charisma
    let mut rogue = Actor::new("Rogue".to_string(), "Halfling".to_string());
    let mut data = HashMap::new();
    data.insert("level".to_string(), serde_json::json!(19));
    data.insert("vitality".to_string(), serde_json::json!(20));
    data.insert("intelligence".to_string(), serde_json::json!(25));
    data.insert("constitution".to_string(), serde_json::json!(18));
    data.insert("charisma".to_string(), serde_json::json!(35));
    data.insert("dexterity".to_string(), serde_json::json!(40));
    data.insert("equipment_bonus".to_string(), serde_json::json!(12));
    data.insert("class".to_string(), serde_json::json!("Rogue"));
    rogue.set_data(data);
    characters.push(rogue);
    
    characters
}

/// Test resource calculation for a character
async fn test_character_resources(
    resource_manager: &Arc<EnhancedHybridResourceManager>,
    character: &Actor,
) -> Result<(), Box<dyn std::error::Error>> {
    let class = character.get_data()
        .get("class")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    
    println!("   Testing {} (Level {})...", 
        character.id,
        character.get_data().get("level").and_then(|v| v.as_i64()).unwrap_or(0)
    );
    
    let result = resource_manager.contribute(character).await?;
    
    println!("     Class: {}", class);
    println!("     Contributions: {} items", result.contributions.len());
    
    // Display key resources
    for (key, value) in &result.contributions {
        if key.contains("hp") || key.contains("mp") || key.contains("mana") || key.contains("qi") {
            println!("     {}: {:.2}", key, value);
        }
    }
    
    Ok(())
}

/// Set up comprehensive event handling
async fn setup_event_handling(event_manager: &Arc<ResourceEventManager>) -> Result<(), Box<dyn std::error::Error>> {
    // Create custom event listener
    struct CombatEventListener {
        listener_id: String,
    }
    
    #[async_trait]
    impl ResourceEventListener for CombatEventListener {
        async fn handle_event(&self, event: &ResourceEvent) -> ActorCoreResult<()> {
            match event.event_type {
                ResourceEventType::ResourceDepleted => {
                    println!("🚨 ALERT: {} depleted for actor {}!", event.resource_name, event.actor_id);
                }
                ResourceEventType::ResourceChanged => {
                    if event.resource_name.contains("hp") {
                        let change = event.new_value - event.old_value;
                        if change < 0.0 {
                            println!("💔 {} took {:.1} damage (HP: {:.1} -> {:.1})", 
                                event.actor_id, change.abs(), event.old_value, event.new_value);
                        } else {
                            println!("💚 {} healed {:.1} HP (HP: {:.1} -> {:.1})", 
                                event.actor_id, change, event.old_value, event.new_value);
                        }
                    }
                }
                _ => {
                    // Handle other event types
                }
            }
            Ok(())
        }
        
        fn listener_id(&self) -> &str {
            &self.listener_id
        }
        
        fn interested_event_types(&self) -> Vec<ResourceEventType> {
            vec![
                ResourceEventType::ResourceChanged,
                ResourceEventType::ResourceDepleted,
                ResourceEventType::ResourceFullyRestored,
            ]
        }
        
        fn event_priority(&self) -> EventPriority {
            EventPriority::High
        }
    }
    
    // Add event listener
    let listener = Box::new(CombatEventListener {
        listener_id: "combat_listener".to_string(),
    });
    event_manager.add_listener(listener).await?;
    
    // Add event filter for high-priority events
    let filter = EventFilter {
        name: "combat_events".to_string(),
        event_types: vec![
            ResourceEventType::ResourceChanged,
            ResourceEventType::ResourceDepleted,
        ],
        actor_ids: None,
        resource_names: Some(vec!["hp_current".to_string(), "mp_current".to_string()]),
        priorities: vec![EventPriority::High, EventPriority::Critical],
        min_value_change: Some(5.0),
        max_value_change: None,
    };
    event_manager.add_event_filter(filter).await?;
    
    println!("   Event handling setup complete");
    Ok(())
}

/// Simulate a combat scenario with resource changes
async fn simulate_combat_scenario(
    resource_manager: &Arc<EnhancedHybridResourceManager>,
    event_manager: &Arc<ResourceEventManager>,
    characters: &[Actor],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   Simulating combat between {} characters...", characters.len());
    
    // Simulate combat rounds
    for round in 1..=5 {
        println!("   Round {}:", round);
        
        for (i, character) in characters.iter().enumerate() {
            // Simulate damage/healing
            let mut modified_character = character.clone();
            let mut data = modified_character.get_data();
            
            if i % 2 == 0 {
                // Even characters take damage
                let current_hp = data.get("hp_current").and_then(|v| v.as_f64()).unwrap_or(100.0);
                let damage = 15.0 + (round as f64 * 5.0);
                let new_hp = (current_hp - damage).max(0.0);
                data.insert("hp_current".to_string(), serde_json::json!(new_hp));
                
                // Emit damage event
                let event = event_manager.create_resource_changed_event(
                    &character.id,
                    "hp_current",
                    current_hp,
                    new_hp,
                    "combat_system",
                );
                event_manager.emit_event(event).await?;
                
                println!("     {} took {:.1} damage (HP: {:.1} -> {:.1})", 
                    character.id, damage, current_hp, new_hp);
            } else {
                // Odd characters heal
                let current_hp = data.get("hp_current").and_then(|v| v.as_f64()).unwrap_or(100.0);
                let healing = 10.0 + (round as f64 * 2.0);
                let new_hp = (current_hp + healing).min(100.0);
                data.insert("hp_current".to_string(), serde_json::json!(new_hp));
                
                // Emit healing event
                let event = event_manager.create_resource_changed_event(
                    &character.id,
                    "hp_current",
                    current_hp,
                    new_hp,
                    "healing_system",
                );
                event_manager.emit_event(event).await?;
                
                println!("     {} healed {:.1} HP (HP: {:.1} -> {:.1})", 
                    character.id, healing, current_hp, new_hp);
            }
            
            modified_character.set_data(data);
            
            // Recalculate resources
            let _result = resource_manager.contribute(&modified_character).await?;
        }
        
        // Brief pause between rounds
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    // Get event statistics
    let stats = event_manager.get_event_stats().await?;
    println!("   Combat complete! Total events: {}", stats.total_events);
    
    Ok(())
}

/// Run comprehensive performance tests
async fn run_performance_tests(
    performance_monitor: &Arc<PerformanceMonitor>,
    characters: &[Actor],
) -> Result<(), Box<dyn std::error::Error>> {
    // Test resource calculation performance
    let test_config = TestConfig {
        iterations: 100,
        warmup_iterations: 10,
        timeout_ms: 5000,
        memory_limit_bytes: Some(100 * 1024 * 1024), // 100MB
        enable_memory_profiling: true,
        enable_cpu_profiling: true,
    };
    
    let results = performance_monitor.run_benchmark(
        "resource_calculation_benchmark",
        test_config,
        || {
            // Simulate resource calculation
            std::thread::sleep(std::time::Duration::from_micros(100));
            Ok(())
        },
    ).await?;
    
    let successful_tests = results.iter().filter(|r| r.success).count();
    let average_time = results.iter().map(|r| r.execution_time_ms).sum::<f64>() / results.len() as f64;
    
    println!("   Performance test results:");
    println!("     Total tests: {}", results.len());
    println!("     Successful: {}", successful_tests);
    println!("     Success rate: {:.2}%", (successful_tests as f64 / results.len() as f64) * 100.0);
    println!("     Average execution time: {:.2}ms", average_time);
    
    // Record performance metrics
    let metric = PerformanceMetric {
        name: "resource_calculation_avg_time".to_string(),
        value: average_time,
        unit: "ms".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        category: MetricCategory::ResourceCalculation,
    };
    performance_monitor.record_metric(metric).await?;
    
    Ok(())
}

/// Simulate resource regeneration over time
async fn simulate_regeneration_over_time(
    resource_manager: &Arc<EnhancedHybridResourceManager>,
    characters: &[Actor],
) -> Result<(), Box<dyn std::error::Error>> {
    // Create regeneration manager
    let regen_config = RegenerationConfig::default();
    let regen_manager = ResourceRegenerationManager::new(regen_config);
    
    // Start regeneration for all characters
    for character in characters {
        regen_manager.start_regeneration(character, "hp_current").await?;
        regen_manager.start_regeneration(character, "mp_current").await?;
    }
    
    println!("   Started regeneration for {} characters", characters.len());
    
    // Simulate time passing
    for hour in 1..=6 {
        println!("   Hour {}:", hour);
        
        // Create actor map for regeneration update
        let mut actor_map = HashMap::new();
        for character in characters {
            actor_map.insert(character.id.to_string(), character.clone());
        }
        
        // Update regeneration
        regen_manager.update_regeneration(&actor_map).await?;
        
        // Get regeneration statistics
        let stats = regen_manager.get_regeneration_stats().await?;
        println!("     Active tasks: {}, Total regenerated: {:.2}", 
            stats.total_tasks, stats.total_regenerated);
        
        // Brief pause to simulate time
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    
    Ok(())
}
