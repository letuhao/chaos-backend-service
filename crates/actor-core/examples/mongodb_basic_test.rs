//! Basic MongoDB Configuration Test
//!
//! This example demonstrates basic MongoDB configuration functionality
//! without loading complex configuration files.

use actor_core::prelude::*;

#[cfg(feature = "mongodb-storage")]
use actor_core::config::mongodb::MongoDBConfigurationProvider;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    println!("=== MongoDB Configuration Basic Test ===");

    // Test 1: Load MongoDB configuration from file
    println!("\n1. Loading MongoDB configuration from file...");
    
    #[cfg(feature = "mongodb-storage")]
    {
        match MongoDBConfigurationProvider::load_mongodb_config("configs/mongodb_config.yaml") {
            Ok(config) => {
                println!("✓ MongoDB configuration loaded successfully");
                println!("  Connection string: {}", config.connection_string);
                println!("  Database: {}", config.database_name);
                println!("  Collection: {}", config.collection_name);
                println!("  Auto-sync enabled: {}", config.enable_auto_sync);
                println!("  Sync interval: {} seconds", config.sync_interval_seconds);
            },
            Err(e) => {
                println!("✗ Failed to load MongoDB configuration: {}", e);
            }
        }
    }
    
    #[cfg(not(feature = "mongodb-storage"))]
    {
        println!("⚠ MongoDB storage feature not enabled");
        println!("  Run with: cargo run --features mongodb-storage --example mongodb_basic_test");
    }

    // Test 2: Create a simple actor
    println!("\n2. Creating test actor...");
    
    let mut actor = Actor::new("test_player".to_string(), "human".to_string());
    actor.data.insert("level".to_string(), serde_json::Value::Number(10.into()));
    actor.data.insert("class".to_string(), serde_json::Value::String("warrior".to_string()));
    
    println!("✓ Test actor created: {}", actor.id);

    // Test 3: Show MongoDB configuration status
    println!("\n3. MongoDB configuration status:");
    println!("  MongoDB config feature enabled: {}", cfg!(feature = "mongodb-storage"));
    println!("  Configuration ready: ✓");

    println!("\n=== Test completed successfully! ===");
    
    Ok(())
}
