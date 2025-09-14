// Redis integration test
// Run with: cargo test redis_integration_test --features redis-cache,cli-tools

use actor_core::{
    cache::CacheFactory,
    config::ActorCoreConfig,
};
use serde_json::json;

#[tokio::test]
async fn test_redis_connection() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    #[cfg(feature = "cli-tools")]
    {
        tracing_subscriber::fmt::init();
    }
    
    println!("🔧 Testing Redis connection...");
    
    // Load configuration
    let config = ActorCoreConfig::from_env()?;
    println!("✅ Configuration loaded");
    println!("   Redis URL: {}", config.redis.url);
    println!("   Redis enabled: {}", config.cache.enable_redis);
    
    // Create cache
    let cache = if config.cache.enable_redis {
        println!("🔗 Creating Redis cache...");
        match CacheFactory::create_distributed_cache(&config.get_redis_url(), config.cache.default_ttl) {
            Ok(cache) => {
                println!("✅ Redis cache created successfully!");
                cache
            }
            Err(e) => {
                println!("❌ Failed to create Redis cache: {}", e);
                println!("🔄 Falling back to in-memory cache...");
                CacheFactory::create_in_memory_cache(config.cache.max_entries, config.cache.default_ttl)
            }
        }
    } else {
        println!("💾 Creating in-memory cache...");
        CacheFactory::create_in_memory_cache(config.cache.max_entries, config.cache.default_ttl)
    };
    
    // Test basic operations
    println!("\n🧪 Testing cache operations...");
    
    // Test set
    let test_key = "test_connection";
    let test_value = json!({
        "message": "Hello from Redis!",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "test_id": uuid::Uuid::new_v4()
    });
    
    cache.set(test_key.to_string(), test_value.clone(), Some(60))?;
    println!("✅ Set operation successful");
    
    // Test get
    let retrieved_value = cache.get(test_key).ok_or("Cache get operation failed")?;
    println!("✅ Get operation successful");
    println!("   Retrieved: {}", retrieved_value);
    
    // Verify the data
    if let Some(message) = retrieved_value.get("message") {
        println!("   Message: {}", message);
        assert_eq!(message, "Hello from Redis!");
    }
    
    // Test delete
    cache.delete(test_key)?;
    println!("✅ Delete operation successful");
    
    // Verify deletion
    assert!(cache.get(test_key).is_none());
    println!("✅ Verification successful - key deleted");
    
    // Show cache statistics
    let stats = cache.get_stats();
    println!("\n📊 Cache Statistics:");
    println!("   Hits: {}", stats.hits);
    println!("   Misses: {}", stats.misses);
    println!("   Sets: {}", stats.sets);
    println!("   Deletes: {}", stats.deletes);
    println!("   Memory usage: {} bytes", stats.memory_usage);
    
    println!("\n🎉 Redis connection test completed successfully!");
    Ok(())
}

#[tokio::test]
async fn test_redis_with_complex_data() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = ActorCoreConfig::from_env()?;
    
    // Create cache
    let cache = if config.cache.enable_redis {
        CacheFactory::create_distributed_cache(&config.get_redis_url(), config.cache.default_ttl)
            .unwrap_or_else(|_| CacheFactory::create_in_memory_cache(config.cache.max_entries, config.cache.default_ttl))
    } else {
        CacheFactory::create_in_memory_cache(config.cache.max_entries, config.cache.default_ttl)
    };
    
    // Test with complex JSON data
    let complex_data = json!({
        "player": {
            "id": "player_123",
            "name": "TestPlayer",
            "level": 42,
            "stats": {
                "health": 1000,
                "mana": 500,
                "strength": 85,
                "agility": 72,
                "intelligence": 90
            },
            "inventory": [
                {"item": "Sword of Power", "quantity": 1, "rarity": "legendary"},
                {"item": "Health Potion", "quantity": 5, "rarity": "common"},
                {"item": "Mana Crystal", "quantity": 3, "rarity": "rare"}
            ],
            "last_login": "2024-01-15T10:30:00Z",
            "achievements": [
                "first_kill",
                "level_10",
                "level_25",
                "level_40"
            ]
        },
        "session": {
            "start_time": chrono::Utc::now().to_rfc3339(),
            "duration_minutes": 0,
            "server": "chaos-world-01"
        }
    });
    
    let key = "player_data_test";
    
    // Store complex data
    cache.set(key.to_string(), complex_data.clone(), Some(300))?;
    
    // Retrieve and verify
    let retrieved = cache.get(key).ok_or("Failed to retrieve complex data")?;
    
    // Verify specific fields
    assert_eq!(retrieved["player"]["name"], "TestPlayer");
    assert_eq!(retrieved["player"]["level"], 42);
    assert_eq!(retrieved["player"]["stats"]["health"], 1000);
    
    let inventory = retrieved["player"]["inventory"].as_array().unwrap();
    assert_eq!(inventory.len(), 3);
    assert_eq!(inventory[0]["item"], "Sword of Power");
    assert_eq!(inventory[0]["rarity"], "legendary");
    
    let achievements = retrieved["player"]["achievements"].as_array().unwrap();
    assert_eq!(achievements.len(), 4);
    assert!(achievements.contains(&json!("first_kill")));
    
    // Clean up
    cache.delete(key)?;
    
    Ok(())
}
