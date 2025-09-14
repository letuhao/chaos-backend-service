use actor_core::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

/// Test aggregator coverage for missing lines
mod tests {
    use super::*;

    /// Test cache deserialization error handling (lines 391-393)
    #[tokio::test]
    async fn test_cache_deserialization_error() {
        let plugin_registry = Arc::new(PluginRegistryImpl::new());
        let combiner_registry = Arc::new(CombinerRegistryImpl::new());
        let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
        let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
        let cache = Arc::new(MockCache::new());
        
        let aggregator = AggregatorImpl::new(
            plugin_registry,
            combiner_registry,
            caps_provider,
            cache,
        );

        // This should return None due to deserialization error
        let result = aggregator.get_cached_snapshot(&Uuid::new_v4());
        assert!(result.is_none());
    }

    /// Test cache operation error handling (lines 403, 409)
    #[tokio::test]
    async fn test_cache_operation_errors() {
        let plugin_registry = Arc::new(PluginRegistryImpl::new());
        let combiner_registry = Arc::new(CombinerRegistryImpl::new());
        let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
        let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
        let cache = Arc::new(MockCache::new());
        
        let aggregator = AggregatorImpl::new(
            plugin_registry,
            combiner_registry,
            caps_provider,
            cache,
        );

        let actor_id = Uuid::new_v4();

        // Test invalidate_cache error handling
        aggregator.invalidate_cache(&actor_id);

        // Test clear_cache error handling
        aggregator.clear_cache();
    }

    /// Test batch resolution to cover more code paths
    #[tokio::test]
    async fn test_batch_resolution() {
        let plugin_registry = Arc::new(PluginRegistryImpl::new());
        let combiner_registry = Arc::new(CombinerRegistryImpl::new());
        let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
        let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
        let cache = Arc::new(InMemoryCache::new(1000, 3600));
        
        let aggregator = AggregatorImpl::new(
            plugin_registry,
            combiner_registry,
            caps_provider,
            cache,
        );

        let actors = vec![
            Actor::new("test1".to_string(), "human".to_string()),
            Actor::new("test2".to_string(), "elf".to_string()),
        ];

        let snapshots = aggregator.resolve_batch(&actors).await.unwrap();
        assert_eq!(snapshots.len(), 2);
    }

    /// Test metrics retrieval
    #[tokio::test]
    async fn test_metrics_retrieval() {
        let plugin_registry = Arc::new(PluginRegistryImpl::new());
        let combiner_registry = Arc::new(CombinerRegistryImpl::new());
        let cap_layer_registry = Arc::new(CapLayerRegistryImpl::new());
        let caps_provider = Arc::new(CapsProviderImpl::new(cap_layer_registry));
        let cache = Arc::new(InMemoryCache::new(1000, 3600));
        
        let aggregator = AggregatorImpl::new(
            plugin_registry,
            combiner_registry,
            caps_provider,
            cache,
        );

        let metrics = aggregator.get_metrics().await;
        assert_eq!(metrics.total_resolutions, 0);
        assert_eq!(metrics.cache_hits, 0);
    }
}


/// Mock cache that returns invalid JSON for testing deserialization errors
struct MockCache;

impl MockCache {
    fn new() -> Self {
        Self
    }
}

impl Cache for MockCache {
    fn get(&self, _key: &str) -> Option<serde_json::Value> {
        // Return invalid JSON to trigger deserialization error
        Some(serde_json::Value::String("invalid json".to_string()))
    }

    fn set(&self, _key: String, _value: serde_json::Value, _ttl: Option<u64>) -> ActorCoreResult<()> {
        Ok(())
    }

    fn delete(&self, _key: &str) -> ActorCoreResult<()> {
        Err(ActorCoreError::CacheError("Mock error".to_string()))
    }

    fn clear(&self) -> ActorCoreResult<()> {
        Err(ActorCoreError::CacheError("Mock error".to_string()))
    }

    fn get_stats(&self) -> CacheStats {
        CacheStats::default()
    }
}
