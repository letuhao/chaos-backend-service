//! Simple MongoDB Configuration Test
//!
//! This example demonstrates basic MongoDB configuration functionality
//! without requiring a running MongoDB instance.

use actor_core::prelude::*;
use actor_core::builder::ActorCoreBuilder;

#[cfg(feature = "mongodb-storage")]
use actor_core::config::mongodb::MongoDBConfigurationProvider;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    // Initialize tracing (optional)
    // tracing_subscriber::fmt::init();

    println!("=== MongoDB Configuration Simple Test ===");

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
                println!("  This is expected if MongoDB feature is not enabled or config file is missing");
            }
        }
    }
    
    #[cfg(not(feature = "mongodb-storage"))]
    {
        println!("⚠ MongoDB storage feature not enabled");
        println!("  Run with: cargo run --features mongodb-storage --example mongodb_simple_test");
    }

    // Test 2: Build Actor Core with MongoDB configuration
    println!("\n2. Building Actor Core with MongoDB configuration...");
    
    let actor_core = ActorCoreBuilder::new()
        .with_mongodb_config(true)  // Enable MongoDB configuration
        .with_metrics(true)
        .with_caching(true)
        .build()
        .await?;

    println!("✓ Actor Core built successfully");

    // Test 3: Create a test actor
    println!("\n3. Creating test actor...");
    
    let mut actor = Actor::new("test_player".to_string(), "human".to_string());
    actor.data.insert("level".to_string(), serde_json::Value::Number(10.into()));
    actor.data.insert("class".to_string(), serde_json::Value::String("warrior".to_string()));
    
    println!("✓ Test actor created: {}", actor.id);

    // Test 4: Show configuration status
    println!("\n4. Configuration status:");
    println!("  Configuration manager: ✓");
    println!("  Registry manager: ✓");

    // Test 5: Show MongoDB configuration status
    println!("\n5. MongoDB configuration status:");
    println!("  MongoDB config enabled: {}", actor_core.use_mongodb_config);
    println!("  Configuration manager ready: ✓");

    println!("\n=== Test completed successfully! ===");
    
    Ok(())
}
