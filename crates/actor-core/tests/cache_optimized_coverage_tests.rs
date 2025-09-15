//! Coverage tests for cache/optimized.rs module.

use actor_core::cache::optimized::{
    OptimizedL1Cache, CacheStats, BatchCacheOperations, CacheWarmer, WarmingStats, OptimizedKeyGenerator
};
use actor_core::interfaces::Cache;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_optimized_l1_cache_creation() {
    let cache = OptimizedL1Cache::new(1000);
    let stats = cache.get_stats();
    assert_eq!(stats.max_size, 1000);
    assert_eq!(stats.size, 0);
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.hit_rate, 0.0);
}

#[tokio::test]
async fn test_optimized_l1_cache_basic_operations() {
    let cache = OptimizedL1Cache::new(100);
    
    // Test set operation
    let result = cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), None);
    assert!(result.is_ok());
    
    // Test get operation
    let value = cache.get("key1");
    assert!(value.is_some());
    assert_eq!(value.unwrap(), serde_json::Value::String("value1".to_string()));
    
    // Test delete operation
    let result = cache.delete("key1");
    assert!(result.is_ok());
    
    // Test get after delete
    let value = cache.get("key1");
    assert!(value.is_none());
}

#[tokio::test]
async fn test_optimized_l1_cache_ttl() {
    let cache = OptimizedL1Cache::new(100);
    
    // Set with TTL
    let result = cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(1));
    assert!(result.is_ok());
    
    // Should be available immediately
    let value = cache.get("key1");
    assert!(value.is_some());
    
    // Wait for expiration (this test might be flaky in CI)
    tokio::time::sleep(Duration::from_millis(1100)).await;
    
    // Should be expired
    let value = cache.get("key1");
    assert!(value.is_none());
}

#[tokio::test]
async fn test_optimized_l1_cache_clear() {
    let cache = OptimizedL1Cache::new(100);
    
    // Set some values
    cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), None).unwrap();
    cache.set("key2".to_string(), serde_json::Value::String("value2".to_string()), None).unwrap();
    
    // Clear cache
    let result = cache.clear();
    assert!(result.is_ok());
    
    // Values should be gone
    assert!(cache.get("key1").is_none());
    assert!(cache.get("key2").is_none());
}

#[tokio::test]
async fn test_optimized_l1_cache_stats() {
    let cache = OptimizedL1Cache::new(100);
    
    // Initial stats
    let stats = cache.get_stats();
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    
    // Miss
    cache.get("nonexistent");
    let stats = cache.get_stats();
    assert_eq!(stats.misses, 1);
    
    // Hit
    cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), None).unwrap();
    cache.get("key1");
    let stats = cache.get_stats();
    assert!(stats.hits >= 1);
}

#[tokio::test]
async fn test_batch_cache_operations_get_many() {
    let cache = OptimizedL1Cache::new(100);
    
    // Set some values
    cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), None).unwrap();
    cache.set("key2".to_string(), serde_json::Value::String("value2".to_string()), None).unwrap();
    
    // Test get_many
    let keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];
    let results = BatchCacheOperations::get_many(&cache, &keys).unwrap();
    
    assert_eq!(results.len(), 3);
    assert!(results[0].is_some());
    assert!(results[1].is_some());
    assert!(results[2].is_none());
}

#[tokio::test]
async fn test_batch_cache_operations_set_many() {
    let cache = OptimizedL1Cache::new(100);
    
    // Test set_many
    let items = vec![
        ("key1".to_string(), serde_json::Value::String("value1".to_string())),
        ("key2".to_string(), serde_json::Value::String("value2".to_string())),
    ];
    
    let result = BatchCacheOperations::set_many(&cache, &items, None);
    assert!(result.is_ok());
    
    // Verify values were set
    assert!(cache.get("key1").is_some());
    assert!(cache.get("key2").is_some());
}

#[tokio::test]
async fn test_cache_warmer_creation() {
    let cache = Arc::new(OptimizedL1Cache::new(100));
    let warmer = CacheWarmer::new(cache, 10);
    
    // Test initial stats
    let stats = warmer.get_stats().await;
    assert_eq!(stats.items_warmed, 0);
    assert_eq!(stats.errors, 0);
}

#[tokio::test]
async fn test_cache_warmer_warm_cache() {
    let cache = Arc::new(OptimizedL1Cache::new(100));
    let warmer = CacheWarmer::new(cache.clone(), 5);
    
    // Prepare items to warm
    let items = vec![
        ("key1".to_string(), serde_json::Value::String("value1".to_string())),
        ("key2".to_string(), serde_json::Value::String("value2".to_string())),
        ("key3".to_string(), serde_json::Value::String("value3".to_string())),
    ];
    
    // Warm cache
    let result = warmer.warm_cache(items, Some(Duration::from_secs(60))).await;
    assert!(result.is_ok());
    
    // Check stats
    let stats = warmer.get_stats().await;
    assert_eq!(stats.items_warmed, 3);
    assert_eq!(stats.errors, 0);
    
    // Verify items were cached
    assert!(cache.get("key1").is_some());
    assert!(cache.get("key2").is_some());
    assert!(cache.get("key3").is_some());
}

#[tokio::test]
async fn test_optimized_key_generator_creation() {
    let _generator = OptimizedKeyGenerator::new();
    assert!(true); // Basic creation test
}

#[tokio::test]
async fn test_optimized_key_generator_generate_key() {
    let generator = OptimizedKeyGenerator::new();
    
    // Test basic key generation
    let key1 = generator.generate_key(&["component1", "component2"]);
    let key2 = generator.generate_key(&["component1", "component2"]);
    let key3 = generator.generate_key(&["component1", "component3"]);
    
    // Same components should generate same key
    assert_eq!(key1, key2);
    // Different components should generate different keys
    assert_ne!(key1, key3);
    
    // Keys should start with "cache_"
    assert!(key1.starts_with("cache_"));
    assert!(key3.starts_with("cache_"));
}

#[tokio::test]
async fn test_optimized_key_generator_generate_actor_key() {
    let generator = OptimizedKeyGenerator::new();
    
    // Test actor key generation
    let actor_id = uuid::Uuid::new_v4();
    let version = 1;
    
    let key1 = generator.generate_actor_key(actor_id, version);
    let key2 = generator.generate_actor_key(actor_id, version);
    let key3 = generator.generate_actor_key(actor_id, version + 1);
    
    // Same actor and version should generate same key
    assert_eq!(key1, key2);
    // Different version should generate different key
    assert_ne!(key1, key3);
    
    // Keys should start with "actor_"
    assert!(key1.starts_with("actor_"));
    assert!(key3.starts_with("actor_"));
}

#[tokio::test]
async fn test_cache_stats_debug_clone() {
    let stats = CacheStats {
        size: 10,
        max_size: 100,
        hits: 5,
        misses: 3,
        hit_rate: 62.5,
    };
    
    // Test Debug trait
    let debug_str = format!("{:?}", stats);
    assert!(debug_str.contains("size: 10"));
    
    // Test Clone trait
    let cloned_stats = stats.clone();
    assert_eq!(cloned_stats.size, stats.size);
    assert_eq!(cloned_stats.hit_rate, stats.hit_rate);
}

#[tokio::test]
async fn test_warming_stats_default() {
    let stats = WarmingStats::default();
    assert_eq!(stats.items_warmed, 0);
    assert_eq!(stats.warming_time, Duration::from_secs(0));
    assert_eq!(stats.errors, 0);
}

#[tokio::test]
async fn test_warming_stats_debug_clone() {
    let stats = WarmingStats {
        items_warmed: 100,
        warming_time: Duration::from_millis(500),
        errors: 2,
    };
    
    // Test Debug trait
    let debug_str = format!("{:?}", stats);
    assert!(debug_str.contains("items_warmed: 100"));
    
    // Test Clone trait
    let cloned_stats = stats.clone();
    assert_eq!(cloned_stats.items_warmed, stats.items_warmed);
    assert_eq!(cloned_stats.errors, stats.errors);
}
