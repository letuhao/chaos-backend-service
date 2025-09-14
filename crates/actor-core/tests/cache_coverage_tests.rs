use actor_core::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Test cache coverage for missing lines
mod tests {
    use super::*;

    /// Test expired entry handling in get method
    #[tokio::test]
    async fn test_get_expired_entry() {
        let cache = LockFreeInMemoryCache::new(1000, 1); // 1 second TTL
        
        // Set a value
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(1)).unwrap();
        
        // Wait for expiration
        sleep(Duration::from_millis(1100)).await;
        
        // Try to get expired value - should return None
        let result = cache.get("key1");
        assert!(result.is_none());
    }

    /// Test expired key cleanup through get operations
    #[tokio::test]
    async fn test_expired_entries_handling() {
        let cache = LockFreeInMemoryCache::new(1000, 1); // 1 second TTL
        
        // Set multiple values
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(1)).unwrap();
        cache.set("key2".to_string(), serde_json::Value::String("value2".to_string()), Some(1)).unwrap();
        cache.set("key3".to_string(), serde_json::Value::String("value3".to_string()), Some(1)).unwrap();
        
        // Wait for expiration
        sleep(Duration::from_millis(1100)).await;
        
        // Try to get expired entries - they should return None
        assert!(cache.get("key1").is_none());
        assert!(cache.get("key2").is_none());
        assert!(cache.get("key3").is_none());
    }

    /// Test LRU eviction when cache is full
    #[tokio::test]
    async fn test_lru_eviction() {
        let cache = LockFreeInMemoryCache::new(2, 3600); // Max 2 entries, long TTL
        
        // Fill cache to capacity
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(3600)).unwrap();
        cache.set("key2".to_string(), serde_json::Value::String("value2".to_string()), Some(3600)).unwrap();
        
        // Add third entry - should evict one of the existing entries
        cache.set("key3".to_string(), serde_json::Value::String("value3".to_string()), Some(3600)).unwrap();
        
        // Only 2 entries should remain
        let mut count = 0;
        if cache.get("key1").is_some() { count += 1; }
        if cache.get("key2").is_some() { count += 1; }
        if cache.get("key3").is_some() { count += 1; }
        assert_eq!(count, 2);
    }

    /// Test cache statistics
    #[tokio::test]
    async fn test_cache_statistics() {
        let cache = LockFreeInMemoryCache::new(1000, 3600);
        
        // Initial stats should be zero
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.sets, 0);
        
        // Set and get values
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(3600)).unwrap();
        cache.get("key1"); // Hit
        cache.get("nonexistent"); // Miss
        
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.sets, 1);
    }

    /// Test cache clear
    #[tokio::test]
    async fn test_cache_clear() {
        let cache = LockFreeInMemoryCache::new(1000, 3600);
        
        // Add some values
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(3600)).unwrap();
        cache.set("key2".to_string(), serde_json::Value::String("value2".to_string()), Some(3600)).unwrap();
        
        // Verify they exist
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_some());
        
        // Clear cache
        cache.clear().unwrap();
        
        // Verify they're gone
        assert!(cache.get("key1").is_none());
        assert!(cache.get("key2").is_none());
        
        // Stats should show 0 sets
        let stats = cache.get_stats();
        assert_eq!(stats.sets, 0);
    }

    /// Test cache delete
    #[tokio::test]
    async fn test_cache_delete() {
        let cache = LockFreeInMemoryCache::new(1000, 3600);
        
        // Add a value
        cache.set("key1".to_string(), serde_json::Value::String("value1".to_string()), Some(3600)).unwrap();
        assert!(cache.get("key1").is_some());
        
        // Delete it
        cache.delete("key1").unwrap();
        assert!(cache.get("key1").is_none());
        
        // Try to delete non-existent key - should not error
        let result = cache.delete("nonexistent");
        assert!(result.is_ok());
    }

    /// Test cache with different TTL values
    #[tokio::test]
    async fn test_cache_different_ttls() {
        let cache = LockFreeInMemoryCache::new(1000, 3600);
        
        // Set values with different TTLs
        cache.set("short".to_string(), serde_json::Value::String("short_value".to_string()), Some(1)).unwrap();
        cache.set("long".to_string(), serde_json::Value::String("long_value".to_string()), Some(3600)).unwrap();
        
        // Both should exist initially
        assert!(cache.get("short").is_some());
        assert!(cache.get("long").is_some());
        
        // Wait for short TTL to expire
        sleep(Duration::from_millis(1100)).await;
        
        // Short should be expired, long should still exist
        assert!(cache.get("short").is_none());
        assert!(cache.get("long").is_some());
    }

    /// Test cache with no TTL (never expires)
    #[tokio::test]
    async fn test_cache_no_ttl() {
        let cache = LockFreeInMemoryCache::new(1000, 3600);
        
        // Set value with no TTL
        cache.set("permanent".to_string(), serde_json::Value::String("permanent_value".to_string()), None).unwrap();
        
        // Wait a bit
        sleep(Duration::from_millis(100)).await;
        
        // Should still exist
        assert!(cache.get("permanent").is_some());
    }

    /// Test cache size limits
    #[tokio::test]
    async fn test_cache_size_limits() {
        let cache = LockFreeInMemoryCache::new(3, 3600); // Max 3 entries
        
        // Fill cache beyond capacity
        for i in 0..5 {
            let key = format!("key{}", i);
            let value = serde_json::Value::String(format!("value{}", i));
            cache.set(key, value, Some(3600)).unwrap();
        }
        
        // Only the last 3 entries should exist
        assert!(cache.get("key0").is_none());
        assert!(cache.get("key1").is_none());
        assert!(cache.get("key2").is_some());
        assert!(cache.get("key3").is_some());
        assert!(cache.get("key4").is_some());
    }

    /// Test concurrent cache operations
    #[tokio::test]
    async fn test_concurrent_cache_operations() {
        let cache = Arc::new(LockFreeInMemoryCache::new(1000, 3600));
        let mut handles = vec![];
        
        // Spawn multiple tasks doing different operations
        for i in 0..10 {
            let cache_clone = cache.clone();
            let handle = tokio::spawn(async move {
                let key = format!("key{}", i);
                let value = serde_json::Value::String(format!("value{}", i));
                
                // Set value
                cache_clone.set(key.clone(), value, Some(3600)).unwrap();
                
                // Get value
                let result = cache_clone.get(&key);
                assert!(result.is_some());
                
                // Delete value
                cache_clone.delete(&key).unwrap();
                
                // Verify it's gone
                let result = cache_clone.get(&key);
                assert!(result.is_none());
            });
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }
    }

    /// Test cache error handling
    #[tokio::test]
    async fn test_cache_error_handling() {
        let cache = LockFreeInMemoryCache::new(1000, 3600);
        
        // Test with invalid JSON (should not cause panic)
        let invalid_value = serde_json::Value::Null;
        cache.set("invalid".to_string(), invalid_value, Some(3600)).unwrap();
        
        // Should be able to retrieve it
        let result = cache.get("invalid");
        assert!(result.is_some());
    }
}
