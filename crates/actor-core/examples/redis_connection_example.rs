//! Redis Connection Example
//!
//! This example demonstrates how to connect to Redis Cloud and use the cache system.
//! 
//! # Prerequisites
//! 1. Copy `env.example` to `.env` and fill in your Redis Cloud credentials
//! 2. Make sure the `redis-cache` feature is enabled
//! 3. Run with: `cargo run --example redis_connection_example --features redis-cache`

use actor_core::{
    cache::CacheFactory,
    config::ActorCoreConfig,
    interfaces::Cache,
};
use serde_json::json;
use std::time::Duration;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    #[cfg(feature = "cli-tools")]
    {
        tracing_subscriber::fmt::init();
    }
    
    info!("Starting Redis connection example...");
    
    // Load configuration from environment variables
    let config = match ActorCoreConfig::from_env() {
        Ok(config) => {
            info!("Configuration loaded successfully");
            config
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(e.into());
        }
    };
    
    // Validate configuration
    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        return Err(e.into());
    }
    
    info!("Redis URL: {}", config.redis.url);
    info!("Redis enabled: {}", config.cache.enable_redis);
    info!("Cache TTL: {} seconds", config.cache.default_ttl);
    
    // Create cache instance
    let cache = if config.cache.enable_redis {
        info!("Creating Redis-backed cache...");
        match CacheFactory::create_distributed_cache(&config.get_redis_url(), config.cache.default_ttl) {
            Ok(cache) => {
                info!("Redis cache created successfully");
                cache
            }
            Err(e) => {
                error!("Failed to create Redis cache: {}", e);
                error!("Falling back to in-memory cache");
                CacheFactory::create_in_memory_cache(config.cache.max_entries, config.cache.default_ttl)
            }
        }
    } else {
        info!("Creating in-memory cache...");
        CacheFactory::create_in_memory_cache(config.cache.max_entries, config.cache.default_ttl)
    };
    
    // Test basic cache operations
    test_basic_operations(cache.as_ref()).await?;
    
    // Test JSON serialization
    test_json_operations(cache.as_ref()).await?;
    
    // Test cache statistics
    test_cache_statistics(cache.as_ref()).await?;
    
    // Test cache expiration
    test_cache_expiration(cache.as_ref()).await?;
    
    info!("Redis connection example completed successfully!");
    Ok(())
}

async fn test_basic_operations(cache: &dyn Cache) -> Result<(), Box<dyn std::error::Error>> {
    info!("Testing basic cache operations...");
    
    // Test set operation
    let key = "test_key";
    let value = json!("Hello, Redis!");
    
    cache.set(key.to_string(), value.clone(), Some(300))?;
    info!("Set key '{}' with value: {}", key, value);
    
    // Test get operation
    if let Some(retrieved_value) = cache.get(key) {
        info!("Retrieved key '{}' with value: {}", key, retrieved_value);
        assert_eq!(retrieved_value, value);
    } else {
        error!("Failed to retrieve key '{}'", key);
        return Err("Cache get operation failed".into());
    }
    
    // Test delete operation
    cache.delete(key)?;
    info!("Deleted key '{}'", key);
    
    // Verify deletion
    if cache.get(key).is_none() {
        info!("Key '{}' successfully deleted", key);
    } else {
        error!("Key '{}' still exists after deletion", key);
        return Err("Cache delete operation failed".into());
    }
    
    Ok(())
}

async fn test_json_operations(cache: &dyn Cache) -> Result<(), Box<dyn std::error::Error>> {
    info!("Testing JSON serialization operations...");
    
    let key = "json_test";
    let complex_value = json!({
        "name": "Chaos World Player",
        "level": 42,
        "stats": {
            "health": 1000,
            "mana": 500,
            "strength": 85,
            "agility": 72,
            "intelligence": 90
        },
        "inventory": [
            {"item": "Sword of Power", "quantity": 1},
            {"item": "Health Potion", "quantity": 5},
            {"item": "Mana Crystal", "quantity": 3}
        ],
        "last_login": "2024-01-15T10:30:00Z"
    });
    
    // Store complex JSON data
    cache.set(key.to_string(), complex_value.clone(), Some(600))?;
    info!("Stored complex JSON data for key '{}'", key);
    
    // Retrieve and verify
    if let Some(retrieved) = cache.get(key) {
        info!("Retrieved complex JSON data: {}", retrieved);
        
        // Verify specific fields
        if let Some(name) = retrieved.get("name") {
            info!("Player name: {}", name);
        }
        
        if let Some(stats) = retrieved.get("stats") {
            if let Some(health) = stats.get("health") {
                info!("Player health: {}", health);
            }
        }
        
        if let Some(inventory) = retrieved.get("inventory") {
            if let Some(items) = inventory.as_array() {
                info!("Player has {} items in inventory", items.len());
            }
        }
    } else {
        error!("Failed to retrieve complex JSON data");
        return Err("JSON retrieval failed".into());
    }
    
    Ok(())
}

async fn test_cache_statistics(cache: &dyn Cache) -> Result<(), Box<dyn std::error::Error>> {
    info!("Testing cache statistics...");
    
    let stats = cache.get_stats();
    info!("Cache statistics:");
    info!("  Hits: {}", stats.hits);
    info!("  Misses: {}", stats.misses);
    info!("  Sets: {}", stats.sets);
    info!("  Deletes: {}", stats.deletes);
    info!("  Memory usage: {} bytes", stats.memory_usage);
    info!("  Max memory usage: {} bytes", stats.max_memory_usage);
    
    Ok(())
}

async fn test_cache_expiration(cache: &dyn Cache) -> Result<(), Box<dyn std::error::Error>> {
    info!("Testing cache expiration...");
    
    let key = "expiration_test";
    let value = json!("This will expire soon");
    
    // Set with short TTL (2 seconds)
    cache.set(key.to_string(), value, Some(2))?;
    info!("Set key '{}' with 2-second TTL", key);
    
    // Verify it exists
    if cache.get(key).is_some() {
        info!("Key '{}' exists before expiration", key);
    } else {
        error!("Key '{}' not found immediately after setting", key);
        return Err("Immediate cache retrieval failed".into());
    }
    
    // Wait for expiration
    info!("Waiting for key to expire...");
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Verify it's expired
    if cache.get(key).is_none() {
        info!("Key '{}' successfully expired", key);
    } else {
        error!("Key '{}' did not expire as expected", key);
        return Err("Cache expiration failed".into());
    }
    
    Ok(())
}

