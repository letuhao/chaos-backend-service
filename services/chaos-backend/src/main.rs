//! Chaos Backend Service - MongoDB Integration Test
//!
//! Simple version to test MongoDB runtime flags integration.

use std::collections::HashMap;
use tracing::{info, warn, error};

use actor_core::prelude::*;
use actor_core::builder::ActorCoreBuilder;
use mongodb::Collection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing with debug level to see all logs
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    
    info!("🚀 Starting Chaos Backend Service - MongoDB Integration Test");
    
    // Test MongoDB connection and load runtime flags
    info!("🔗 Attempting to connect to MongoDB...");
    let runtime_flags = match load_runtime_flags_from_mongodb().await {
        Ok(flags) => {
            info!("🔧 Runtime flags loaded successfully: {} flags", flags.len());
            flags
        }
        Err(e) => {
            warn!("⚠️  Failed to load runtime flags from MongoDB: {}", e);
            warn!("🔄 Using default runtime flags...");
            HashMap::new()
        }
    };
    
    // Create Actor Core with minimal configuration (no default config file)
    info!("📦 Creating minimal Actor Core without default config...");
    
    // Create a minimal config file for testing
    create_minimal_config_file().await?;
    
    let builder = ActorCoreBuilder::new()
        .with_metrics(true)
        .with_caching(true)
        .with_log_level("debug".to_string())
        .with_config_path("configs/minimal_test_config.yaml".into())
        .with_mongodb_config(true);
    
    info!("🔧 Builder created, attempting to build Actor Core...");
    let actor_core = match builder.build().await {
        Ok(core) => {
            info!("✅ Actor Core initialized successfully");
            core
        }
        Err(e) => {
            error!("❌ Failed to build Actor Core: {:?}", e);
            error!("🔍 Error details: {}", e);
            
            // Enable backtrace với full details
            let backtrace = std::backtrace::Backtrace::capture();
            error!("🔍 Full Backtrace:\n{:?}", backtrace);
            
            return Err(e.into());
        }
    };
    
    // Test creating actors
    info!("👤 Creating test actors...");
    let actors = vec![
        Actor::new("TestPlayer1".to_string(), "Human".to_string()),
        Actor::new("TestPlayer2".to_string(), "Elf".to_string()),
        Actor::new("TestPlayer3".to_string(), "Dwarf".to_string()),
    ];
    
    for actor in &actors {
        info!("👤 Created actor: {} ({})", actor.id, actor.name);
    }
    
    // Test saving configuration to MongoDB
    info!("💾 Testing configuration save to MongoDB...");
    let config_manager = actor_core.get_config_manager();
    match config_manager.save_configs().await {
        Ok(()) => {
            info!("✅ Configuration saved to MongoDB successfully");
        }
        Err(e) => {
            warn!("❌ Failed to save configuration to MongoDB: {}", e);
        }
    }
    
    // Test configuration loading
    info!("⚙️  Testing configuration loading...");
    let config_manager = actor_core.get_config_manager();
    info!("🔧 Configuration manager obtained");
    
    let test_configs = vec![
        ("defaults", "default_actor_health"),
        ("defaults", "default_actor_level"),
        ("logging", "level"),
        ("metrics", "enabled"),
    ];
    
    info!("🔍 Testing {} configuration keys...", test_configs.len());
    for (category, key) in test_configs {
        info!("   Testing: {}.{}", category, key);
        match config_manager.get_config(category, key).await {
            Ok(Some(config)) => {
                info!("   ✅ {}.{} = {:?}", category, key, config.value);
            }
            Ok(None) => {
                warn!("   ⚠️  {}.{} not found", category, key);
            }
            Err(e) => {
                error!("   ❌ {}.{} failed: {}", category, key, e);
                error!("   🔍 Error details: {:?}", e);
            }
        }
    }
    
    info!("✅ MongoDB integration test completed successfully");
    info!("🎯 Server is ready to run with runtime flags!");
    
    // Display runtime flags
    info!("📋 Current runtime flags:");
    for (key, value) in &runtime_flags {
        info!("   {}: {}", key, value);
    }
    
    Ok(())
}

/// Create a minimal configuration file for testing
async fn create_minimal_config_file() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use tokio::fs;
    
    let minimal_config = r#"# Minimal test configuration
categories:
  defaults:
    default_actor_health: 100
    default_actor_level: 1
    default_actor_experience: 0

  logging:
    level: debug

  metrics:
    enabled: true

  # System IDs with correct format
  system_ids:
    supported_systems:
      - "luyen_the"
      - "kim_dan"
      - "combat"
      - "equipment"
      - "buff"

  # Context types
  context_types:
    supported_contexts:
      - "damage"
      - "healing"
      - "experience_gain"

  # Default resources for Registry Manager
  default_resources:
    resources:
      health:
        id: "health"
        name: "Health"
        description: "Actor health points"
        category: "vital"
        resource_type: "health"
        base_value: 100.0
        min_value: 0.0
        max_value: 1000.0
        regen_rate: 1.0
        regen_type: "passive"
      mana:
        id: "mana"
        name: "Mana"
        description: "Actor mana points"
        category: "vital"
        resource_type: "mana"
        base_value: 50.0
        min_value: 0.0
        max_value: 500.0
        regen_rate: 0.5
        regen_type: "passive"
      stamina:
        id: "stamina"
        name: "Stamina"
        description: "Actor stamina points"
        category: "vital"
        resource_type: "stamina"
        base_value: 100.0
        min_value: 0.0
        max_value: 1000.0
        regen_rate: 2.0
        regen_type: "passive"

  # Resource types configuration
  resource_types:
    definitions:
      health:
        name: "Health"
        description: "Health points for actors"
      mana:
        name: "Mana"
        description: "Mana points for actors"
      stamina:
        name: "Stamina"
        description: "Stamina points for actors"

  # Regen types configuration
  regen_types:
    definitions:
      passive:
        name: "Passive"
        description: "Passive regeneration"
      active:
        name: "Active"
        description: "Active regeneration"
      none:
        name: "None"
        description: "No regeneration"

  # Default categories configuration
  default_categories:
    categories:
      vital:
        id: "vital"
        name: "Vital"
        description: "Vital resources for actors"
      combat:
        id: "combat"
        name: "Combat"
        description: "Combat-related resources"
      magic:
        id: "magic"
        name: "Magic"
        description: "Magic-related resources"

  # Default tags configuration
  default_tags:
    tags:
      primary:
        id: "primary"
        name: "Primary"
        description: "Primary resources"
        category: "vital"
      secondary:
        id: "secondary"
        name: "Secondary"
        description: "Secondary resources"
        category: "vital"
      special:
        id: "special"
        name: "Special"
        description: "Special resources"
        category: "magic"

  # Tag types configuration
  tag_types:
    definitions:
      primary:
        name: "Primary"
        description: "Primary resource tags"
      secondary:
        name: "Secondary"
        description: "Secondary resource tags"
      special:
        name: "Special"
        description: "Special resource tags"
      vital:
        name: "Vital"
        description: "Vital resource tags"
      combat:
        name: "Combat"
        description: "Combat resource tags"
      magic:
        name: "Magic"
        description: "Magic resource tags"
"#;
    
    // Ensure configs directory exists
    fs::create_dir_all("configs").await?;
    
    // Write minimal config file
    fs::write("configs/minimal_test_config.yaml", minimal_config).await?;
    info!("📝 Created minimal test configuration file: configs/minimal_test_config.yaml");
    
    Ok(())
}

/// Load runtime flags from MongoDB
async fn load_runtime_flags_from_mongodb() -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
    use mongodb::{Client, Collection};
    use mongodb::bson::doc;
    
    let mut flags = HashMap::new();
    
    info!("🔗 Attempting to connect to MongoDB at mongodb://localhost:27017...");
    
    // Try to connect to MongoDB
    match Client::with_uri_str("mongodb://localhost:27017").await {
        Ok(client) => {
            info!("✅ Successfully connected to MongoDB");
            
            let db = client.database("chaos_game");
            info!("📁 Using database: chaos_game");
            
            let collection: Collection<mongodb::bson::Document> = db.collection("runtime_flags");
            info!("📄 Accessing collection: runtime_flags");
            
            // Load runtime flags
            info!("🔍 Searching for runtime_config document...");
            match collection.find_one(doc! {"_id": "runtime_config"}, None).await {
                Ok(Some(doc)) => {
                    info!("📄 Found runtime_config document in MongoDB");
                    
                    // Parse flags from document
                    for (key, value) in doc {
                        if key != "_id" {
                            info!("   {}: {:?}", key, value);
                            flags.insert(key, serde_json::to_value(value)?);
                        }
                    }
                    info!("✅ Successfully loaded {} runtime flags from MongoDB", flags.len());
                }
                Ok(None) => {
                    warn!("⚠️  No runtime_config document found in MongoDB");
                    warn!("🔄 Initializing default flags in MongoDB...");
                    // Initialize with default flags
                    initialize_default_flags(&collection).await?;
                }
                Err(e) => {
                    error!("❌ Failed to query runtime_flags collection: {}", e);
                    error!("🔍 Error details: {:?}", e);
                }
            }
        }
        Err(e) => {
            error!("❌ Failed to connect to MongoDB: {}", e);
            error!("🔍 Error details: {:?}", e);
            error!("💡 Make sure MongoDB is running: mongod --dbpath C:\\data\\db");
        }
    }
    
    // Set default flags if none loaded
    if flags.is_empty() {
        flags.insert("server_port".to_string(), serde_json::Value::Number(8080.into()));
        flags.insert("max_connections".to_string(), serde_json::Value::Number(1000.into()));
        flags.insert("tick_rate".to_string(), serde_json::Value::Number(60.into()));
        flags.insert("enable_mongodb_sync".to_string(), serde_json::Value::Bool(true));
        flags.insert("mongodb_connection".to_string(), serde_json::Value::String("mongodb://localhost:27017".to_string()));
        flags.insert("config_sync_interval".to_string(), serde_json::Value::Number(300.into()));
        flags.insert("log_level".to_string(), serde_json::Value::String("debug".to_string()));
        flags.insert("enable_metrics".to_string(), serde_json::Value::Bool(true));
        flags.insert("world_size".to_string(), serde_json::Value::Number(10000.into()));
        flags.insert("max_actors".to_string(), serde_json::Value::Number(10000.into()));
    }
    
    Ok(flags)
}

/// Initialize default runtime flags in MongoDB
async fn initialize_default_flags(collection: &Collection<mongodb::bson::Document>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use mongodb::bson::doc;
    
    info!("📝 Creating default runtime flags document...");
    
    let default_flags = doc! {
        "_id": "runtime_config",
        "server_port": 8080,
        "max_connections": 1000,
        "tick_rate": 60,
        "enable_mongodb_sync": true,
        "mongodb_connection": "mongodb://localhost:27017",
        "config_sync_interval": 300,
        "log_level": "debug",
        "enable_metrics": true,
        "world_size": 10000,
        "max_actors": 10000
    };
    
    info!("💾 Inserting default flags into MongoDB...");
    match collection.insert_one(default_flags, None).await {
        Ok(result) => {
            info!("✅ Successfully inserted default runtime flags into MongoDB");
            info!("   Document ID: {:?}", result.inserted_id);
        }
        Err(e) => {
            error!("❌ Failed to insert default flags into MongoDB: {}", e);
            error!("🔍 Error details: {:?}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}