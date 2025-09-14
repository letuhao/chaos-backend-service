//! Cache Tests
//!
//! This module contains tests for all cache-related functionality including
//! in-memory caches, distributed caches, and multi-layer cache systems.

use actor_core::prelude::*;
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lock_free_cache_creation() {
        let cache = LockFreeInMemoryCache::new(100, 60);
        // Test that creation doesn't panic
        let _ = cache;
    }

    #[test]
    fn test_lock_free_cache_basic_operations() {
        let cache = LockFreeInMemoryCache::new(100, 60);
        
        // Test set and get
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        let result = cache.get("key1");
        assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
        
        // Test miss
        let result = cache.get("nonexistent");
        assert_eq!(result, None);
    }

    #[test]
    fn test_lock_free_cache_delete() {
        let cache = LockFreeInMemoryCache::new(100, 60);
        
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        cache.delete("key1").unwrap();
        
        let result = cache.get("key1");
        assert_eq!(result, None);
    }

    #[test]
    fn test_lock_free_cache_clear() {
        let cache = LockFreeInMemoryCache::new(100, 60);
        
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        cache.set("key2".to_string(), serde_json::Value::String("value2".to_string()), Some(60)).unwrap();
        cache.clear().unwrap();
        
        assert_eq!(cache.get("key1"), None);
        assert_eq!(cache.get("key2"), None);
    }

    #[test]
    fn test_lock_free_cache_stats() {
        let cache = LockFreeInMemoryCache::new(100, 60);
        
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        cache.get("key1");
        cache.get("nonexistent");
        let _ = cache.delete("key1");
        
        let stats = cache.get_stats();
        assert_eq!(stats.sets, 1);
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.deletes, 1);
    }

    #[test]
    fn test_lock_free_cache_eviction() {
        let cache = LockFreeInMemoryCache::new(2, 60);
        
        // Fill cache beyond max_entries
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        cache.set("key2".to_string(), serde_json::Value::String("value2".to_string()), Some(60)).unwrap();
        cache.set("key3".to_string(), serde_json::Value::String("value3".to_string()), Some(60)).unwrap();
        
        // At least one key should be evicted
        let key1_exists = cache.get("key1").is_some();
        let key2_exists = cache.get("key2").is_some();
        let key3_exists = cache.get("key3").is_some();
        
        // At least one key should be missing due to eviction
        assert!(!(key1_exists && key2_exists && key3_exists));
    }

    #[test]
    fn test_in_memory_cache_creation() {
        let cache = InMemoryCache::new(100, 60);
        // Test that creation doesn't panic
        let _ = cache;
    }

    #[test]
    fn test_in_memory_cache_basic_operations() {
        let cache = InMemoryCache::new(100, 60);
        
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        let result = cache.get("key1");
        assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
    }

    #[test]
    fn test_in_memory_cache_delete_nonexistent() {
        let cache = InMemoryCache::new(100, 60);
        
        // Delete non-existent key should not error
        let _ = cache.delete("nonexistent"); // Just test that it doesn't panic
    }

    #[test]
    fn test_in_memory_cache_clear() {
        let cache = InMemoryCache::new(100, 60);
        
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        cache.clear().unwrap();
        
        let result = cache.get("key1");
        assert_eq!(result, None);
    }

    #[test]
    fn test_in_memory_cache_stats() {
        let cache = InMemoryCache::new(100, 60);
        
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        cache.get("key1");
        cache.get("nonexistent");
        let _ = cache.delete("key1");
        
        let stats = cache.get_stats();
        assert_eq!(stats.sets, 1);
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.deletes, 1);
    }

    #[cfg(feature = "redis-cache")]
    #[test]
    fn test_distributed_cache_creation_invalid_url() {
        use crate::cache::DistributedCache;
        let result = DistributedCache::new("invalid://url", 60);
        assert!(result.is_err());
    }

    #[cfg(feature = "redis-cache")]
    #[test]
    fn test_distributed_cache_creation_valid_url() {
        use crate::cache::DistributedCache;
        // This will fail in test environment but tests the code path
        let result = DistributedCache::new("redis://localhost:6379", 60);
        // We expect this to fail in test environment, but it tests the creation logic
        // The actual implementation might succeed in parsing the URL, so we just test it doesn't panic
        let _ = result; // Just test that it doesn't panic
    }

    #[test]
    fn test_multi_layer_cache_creation() {
        let l1 = Arc::new(InMemoryCache::new(10, 60));
        let l2 = Arc::new(InMemoryCache::new(20, 120));
        let l3 = Arc::new(InMemoryCache::new(30, 180));
        
        let cache = MultiLayerCache::new(l1, l2, l3);
        // Test that creation doesn't panic
        let _ = cache;
    }

    #[test]
    fn test_multi_layer_cache_l1_hit() {
        let l1 = Arc::new(InMemoryCache::new(10, 60));
        let l2 = Arc::new(InMemoryCache::new(20, 120));
        let l3 = Arc::new(InMemoryCache::new(30, 180));
        
        let cache = MultiLayerCache::new(l1, l2, l3);
        
        // Set value in L1
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        
        // Get should hit L1
        let result = cache.get("key1");
        assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
        
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 1);
    }

    #[test]
    fn test_multi_layer_cache_l2_hit() {
        let l1 = Arc::new(InMemoryCache::new(10, 60));
        let l2 = Arc::new(InMemoryCache::new(20, 120));
        let l3 = Arc::new(InMemoryCache::new(30, 180));
        
        let cache = MultiLayerCache::new(l1, l2.clone(), l3);
        
        // Set value in L2 directly
        l2.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        
        // Get should hit L2
        let result = cache.get("key1");
        assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
    }

    #[test]
    fn test_multi_layer_cache_l3_hit() {
        let l1 = Arc::new(InMemoryCache::new(10, 60));
        let l2 = Arc::new(InMemoryCache::new(20, 120));
        let l3 = Arc::new(InMemoryCache::new(30, 180));
        
        let cache = MultiLayerCache::new(l1, l2, l3.clone());
        
        // Set value in L3 directly
        l3.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        
        // Get should hit L3
        let result = cache.get("key1");
        assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
    }

    #[test]
    fn test_multi_layer_cache_miss() {
        let l1 = Arc::new(InMemoryCache::new(10, 60));
        let l2 = Arc::new(InMemoryCache::new(20, 120));
        let l3 = Arc::new(InMemoryCache::new(30, 180));
        
        let cache = MultiLayerCache::new(l1, l2, l3);
        
        // Get non-existent key
        let result = cache.get("nonexistent");
        assert_eq!(result, None);
        
        let stats = cache.get_stats();
        // Multi-layer cache aggregates stats from all layers, so misses >= 1
        assert!(stats.misses >= 1);
    }

    #[test]
    fn test_multi_layer_cache_set() {
        let l1 = Arc::new(InMemoryCache::new(10, 60));
        let l2 = Arc::new(InMemoryCache::new(20, 120));
        let l3 = Arc::new(InMemoryCache::new(30, 180));
        
        let cache = MultiLayerCache::new(l1, l2, l3);
        
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        let result = cache.get("key1");
        assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
    }

    #[test]
    fn test_multi_layer_cache_delete() {
        let l1 = Arc::new(InMemoryCache::new(10, 60));
        let l2 = Arc::new(InMemoryCache::new(20, 120));
        let l3 = Arc::new(InMemoryCache::new(30, 180));
        
        let cache = MultiLayerCache::new(l1, l2, l3);
        
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        cache.delete("key1").unwrap();
        let result = cache.get("key1");
        assert_eq!(result, None);
    }

    #[test]
    fn test_multi_layer_cache_clear_all() {
        let l1 = Arc::new(InMemoryCache::new(10, 60));
        let l2 = Arc::new(InMemoryCache::new(20, 120));
        let l3 = Arc::new(InMemoryCache::new(30, 180));
        
        let cache = MultiLayerCache::new(l1, l2, l3);
        
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        cache.clear().unwrap();
        let result = cache.get("key1");
        assert_eq!(result, None);
    }

    #[test]
    fn test_multi_layer_cache_stats_aggregation() {
        let l1 = Arc::new(InMemoryCache::new(10, 60));
        let l2 = Arc::new(InMemoryCache::new(20, 120));
        let l3 = Arc::new(InMemoryCache::new(30, 180));
        
        let cache = MultiLayerCache::new(l1, l2, l3);
        
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        cache.get("key1");
        cache.get("nonexistent");
        let _ = cache.delete("key1");
        
        let stats = cache.get_stats();
        // Multi-layer cache aggregates stats from all layers
        assert!(stats.sets >= 1);
        assert!(stats.hits >= 1);
        assert!(stats.misses >= 1);
        assert!(stats.deletes >= 1);
    }

    #[test]
    fn test_cache_factory_in_memory() {
        let cache = CacheFactory::create_in_memory_cache(100, 60);
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        let result = cache.get("key1");
        assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
    }

    #[test]
    fn test_cache_factory_lock_free_in_memory() {
        let cache = CacheFactory::create_lock_free_in_memory_cache(100, 60);
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        let result = cache.get("key1");
        assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
    }

    #[cfg(feature = "redis-cache")]
    #[test]
    fn test_cache_factory_distributed_invalid() {
        let result = CacheFactory::create_distributed_cache("invalid://url", 60);
        assert!(result.is_err());
    }

    #[test]
    fn test_cache_factory_multi_layer() {
        let l1 = CacheFactory::create_in_memory_cache(10, 60);
        let l2 = CacheFactory::create_in_memory_cache(20, 120);
        let l3 = CacheFactory::create_in_memory_cache(30, 180);
        
        let cache = CacheFactory::create_multi_layer_cache(l1, l2, l3);
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        let result = cache.get("key1");
        assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
    }

    #[test]
    fn test_cache_factory_default_multi_layer() {
        let cache = CacheFactory::create_default_multi_layer_cache();
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(60)).unwrap();
        let result = cache.get("key1");
        assert_eq!(result, Some(serde_json::Value::String("value1".to_string())));
    }


    #[tokio::test]
    async fn test_lock_free_cache_concurrent_access() {
        let cache = Arc::new(LockFreeInMemoryCache::new(1000, 60));
        let mut handles = vec![];
        
        // Spawn multiple tasks that write and read concurrently
        for i in 0..10 {
            let cache_clone = cache.clone();
            let handle = tokio::spawn(async move {
                for j in 0..10 {
                    let key = format!("key_{}_{}", i, j);
                    let value = format!("value_{}_{}", i, j);
                    cache_clone.set(key.clone(), serde_json::Value::String(value.clone()), Some(60)).unwrap();
                    let result = cache_clone.get(&key);
                    assert_eq!(result, Some(serde_json::Value::String(value)));
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.await.unwrap();
        }
        
        let stats = cache.get_stats();
        assert!(stats.sets >= 100);
        assert!(stats.hits >= 100);
    }

    #[tokio::test]
    async fn test_multi_layer_cache_concurrent_access() {
        let l1 = Arc::new(InMemoryCache::new(100, 60));
        let l2 = Arc::new(InMemoryCache::new(200, 120));
        let l3 = Arc::new(InMemoryCache::new(300, 180));
        let cache = Arc::new(MultiLayerCache::new(l1, l2, l3));
        let mut handles = vec![];
        
        for i in 0..5 {
            let cache_clone = cache.clone();
            let handle = tokio::spawn(async move {
                for j in 0..5 {
                    let key = format!("key_{}_{}", i, j);
                    let value = format!("value_{}_{}", i, j);
                    cache_clone.set(key.clone(), serde_json::Value::String(value.clone()), Some(60)).unwrap();
                    let result = cache_clone.get(&key);
                    assert_eq!(result, Some(serde_json::Value::String(value)));
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.await.unwrap();
        }
        
        let stats = cache.get_stats();
        // Stats are aggregated from all layers, so we expect at least these values
        assert!(stats.sets >= 25);
        assert!(stats.hits >= 25);
    }
}
