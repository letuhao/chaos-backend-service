//! MongoDB Configuration Demo for Actor Core
//!
//! This example demonstrates how to use MongoDB for configuration management
//! in Actor Core, including loading and saving configurations.

use actor_core::prelude::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("=== MongoDB Configuration Demo ===");

    // Example 1: Build Actor Core with MongoDB configuration
    println!("\n1. Building Actor Core with MongoDB configuration...");
    
    let actor_core = ActorCoreBuilder::new()
        .with_mongodb_config(true)  // Enable MongoDB configuration
        .with_metrics(true)
        .with_caching(true)
        .build()
        .await?;

    println!("✓ Actor Core built successfully with MongoDB support");

    // Example 2: Create MongoDB configuration manager
    println!("\n2. Creating MongoDB configuration manager...");
    
    let mongodb_config = crate::config::mongodb::MongoDBConfigurationProvider::load_mongodb_config("configs/mongodb_config.yaml")?;
    let mongodb_manager = crate::config::mongodb_manager::MongoDBConfigManager::new(mongodb_config).await?;
    
    println!("✓ MongoDB configuration manager created");

    // Example 3: Check sync status
    println!("\n3. Checking sync status...");
    
    let sync_status = mongodb_manager.get_sync_status().await;
    println!("  Sync enabled: {}", sync_status.enabled);
    println!("  Sync in progress: {}", sync_status.in_progress);
    println!("  Sync interval: {} seconds", sync_status.interval_seconds);

    // Example 4: Start auto-sync (in a separate task)
    println!("\n4. Starting auto-sync daemon...");
    
    let mongodb_manager_clone = mongodb_manager.clone();
    tokio::spawn(async move {
        if let Err(e) = mongodb_manager_clone.start_auto_sync().await {
            eprintln!("Auto-sync error: {}", e);
        }
    });
    
    println!("✓ Auto-sync daemon started");

    // Example 5: Manual sync operation
    println!("\n5. Performing manual sync from files to MongoDB...");
    
    match mongodb_manager.sync_from_files_to_db().await {
        Ok(_) => println!("✓ Manual sync completed successfully"),
        Err(e) => println!("✗ Manual sync failed: {}", e),
    }

    // Example 6: Create a test actor and resolve stats
    println!("\n6. Creating test actor and resolving stats...");
    
    let mut actor = Actor::new("test_player".to_string(), "human".to_string());
    
    // Add some test data
    actor.data.insert("level".to_string(), serde_json::Value::Number(10.into()));
    actor.data.insert("class".to_string(), serde_json::Value::String("warrior".to_string()));
    
    // Resolve actor stats using the configuration
    let snapshot = actor_core.config_manager.get_aggregator().resolve(&actor).await?;
    
    println!("✓ Actor stats resolved:");
    println!("  Actor ID: {}", snapshot.actor_id);
    println!("  Total stats: {}", snapshot.stats.len());

    // Example 7: Demonstrate configuration loading from MongoDB
    println!("\n7. Demonstrating configuration loading from MongoDB...");
    
    // This would typically load configuration values from MongoDB
    // For now, we'll just show the concept
    println!("  Configuration would be loaded from MongoDB collection: 'configuration'");
    println!("  Categories available: {:?}", actor_core.config_manager.get_registry().get_supported_categories().await);

    // Example 8: Show MongoDB configuration details
    println!("\n8. MongoDB configuration details:");
    
    let mongodb_provider = mongodb_manager.get_mongodb_provider();
    println!("  Provider ID: {}", mongodb_provider.provider_id());
    println!("  Priority: {}", mongodb_provider.priority());
    println!("  Supported categories: {:?}", mongodb_provider.get_supported_categories());

    println!("\n=== Demo completed successfully! ===");
    
    // Keep the program running for a bit to see auto-sync in action
    println!("\nAuto-sync daemon is running. Press Ctrl+C to exit...");
    tokio::signal::ctrl_c().await?;
    
    println!("\nShutting down...");
    mongodb_manager.stop_auto_sync().await;
    
    Ok(())
}
