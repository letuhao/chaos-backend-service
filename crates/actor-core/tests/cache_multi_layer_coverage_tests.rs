//! Coverage tests for cache/multi_layer modules.

use actor_core::cache::multi_layer::manager::{
    MultiLayerCacheManager, 
    MultiLayerConfig, 
    CacheHealthStatus, 
    OverallHealth, 
    LayerHealth
};
use actor_core::cache::multi_layer::layers::{
    L1Cache, 
    L2Cache, 
    L3Cache, 
    CacheEntry, 
    LayerConfig
};
use actor_core::cache::multi_layer::metrics::{
    MultiLayerStats, 
    L1CacheStats, 
    L2CacheStats, 
    L3CacheStats, 
    CacheLayer
};
use actor_core::cache::multi_layer::policy::EvictionPolicy;
use actor_core::interfaces::Cache;
use actor_core::ActorCoreResult;
use std::sync::Arc;
use std::time::Duration;

// Mock implementations for testing
struct MockL1Cache;
struct MockL2Cache;
struct MockL3Cache;

impl L1Cache for MockL1Cache {
    fn get(&self, _key: &str) -> Option<serde_json::Value> {
        None
    }
    
    fn set(&self, _key: String, _value: serde_json::Value, _ttl: Option<u64>) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn delete(&self, _key: &str) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn clear(&self) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn get_stats(&self) -> L1CacheStats {
        L1CacheStats::new()
    }
    
    fn memory_usage(&self) -> u64 {
        0
    }
    
    fn max_capacity(&self) -> usize {
        1000
    }
}

#[async_trait::async_trait]
impl L2Cache for MockL2Cache {
    async fn get(&self, _key: &str) -> ActorCoreResult<Option<serde_json::Value>> {
        Ok(None)
    }
    
    async fn set(&self, _key: String, _value: serde_json::Value, _ttl: Option<u64>) -> ActorCoreResult<()> {
        Ok(())
    }
    
    async fn delete(&self, _key: &str) -> ActorCoreResult<()> {
        Ok(())
    }
    
    async fn clear(&self) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn get_stats(&self) -> L2CacheStats {
        L2CacheStats::new()
    }
    
    fn memory_usage(&self) -> u64 {
        0
    }
    
    fn max_capacity(&self) -> usize {
        10000
    }
    
    async fn sync(&self) -> ActorCoreResult<()> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl L3Cache for MockL3Cache {
    async fn get(&self, _key: &str) -> ActorCoreResult<Option<serde_json::Value>> {
        Ok(None)
    }
    
    async fn set(&self, _key: String, _value: serde_json::Value, _ttl: Option<u64>) -> ActorCoreResult<()> {
        Ok(())
    }
    
    async fn delete(&self, _key: &str) -> ActorCoreResult<()> {
        Ok(())
    }
    
    async fn clear(&self) -> ActorCoreResult<()> {
        Ok(())
    }
    
    fn get_stats(&self) -> L3CacheStats {
        L3CacheStats::new()
    }
    
    fn disk_usage(&self) -> u64 {
        0
    }
    
    fn max_capacity(&self) -> usize {
        100000
    }
    
    async fn compact(&self) -> ActorCoreResult<()> {
        Ok(())
    }
}

#[test]
fn test_multi_layer_config_new() {
    let config = MultiLayerConfig::new();
    
    assert_eq!(config.l1_max_size, 1000);
    assert_eq!(config.l1_eviction_policy, EvictionPolicy::Lru);
    assert_eq!(config.l2_cache_path, "/tmp/actor_cache_l2");
    assert_eq!(config.l2_max_size, 10000);
    assert_eq!(config.l3_cache_dir, "/tmp/actor_cache_l3");
    assert_eq!(config.l3_max_size, 100000);
    assert_eq!(config.sync_interval, Duration::from_secs(60));
    assert!(config.enable_background_sync);
    assert!(config.enable_metrics);
    assert!(config.enable_tracing);
}

#[test]
fn test_multi_layer_config_default() {
    let config = MultiLayerConfig::default();
    
    assert_eq!(config.l1_max_size, 1000);
    assert_eq!(config.l1_eviction_policy, EvictionPolicy::Lru);
    assert_eq!(config.l2_cache_path, "/tmp/actor_cache_l2");
    assert_eq!(config.l2_max_size, 10000);
    assert_eq!(config.l3_cache_dir, "/tmp/actor_cache_l3");
    assert_eq!(config.l3_max_size, 100000);
    assert_eq!(config.sync_interval, Duration::from_secs(60));
    assert!(config.enable_background_sync);
    assert!(config.enable_metrics);
    assert!(config.enable_tracing);
}

#[test]
fn test_multi_layer_config_validate_success() {
    let config = MultiLayerConfig::new();
    
    let result = config.validate();
    assert!(result.is_ok());
}

#[test]
fn test_multi_layer_config_validate_l1_zero_size() {
    let mut config = MultiLayerConfig::new();
    config.l1_max_size = 0;
    
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("L1 max size must be greater than 0"));
}

#[test]
fn test_multi_layer_config_validate_l2_zero_size() {
    let mut config = MultiLayerConfig::new();
    config.l2_max_size = 0;
    
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("L2 max size must be greater than 0"));
}

#[test]
fn test_multi_layer_config_validate_l3_zero_size() {
    let mut config = MultiLayerConfig::new();
    config.l3_max_size = 0;
    
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("L3 max size must be greater than 0"));
}

#[test]
fn test_multi_layer_config_validate_l2_empty_path() {
    let mut config = MultiLayerConfig::new();
    config.l2_cache_path = String::new();
    
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("L2 cache path cannot be empty"));
}

#[test]
fn test_multi_layer_config_validate_l3_empty_dir() {
    let mut config = MultiLayerConfig::new();
    config.l3_cache_dir = String::new();
    
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("L3 cache directory cannot be empty"));
}

#[tokio::test]
async fn test_multi_layer_cache_manager_new() {
    let l1_cache = Arc::new(MockL1Cache);
    let l2_cache = Arc::new(MockL2Cache);
    let l3_cache = Arc::new(MockL3Cache);
    let config = MultiLayerConfig::new();
    
    let manager = MultiLayerCacheManager::new(
        l1_cache,
        l2_cache,
        l3_cache,
        config,
    );
    
    // Test that the manager was created successfully
    assert!(manager.l1_cache().max_capacity() > 0);
    assert!(manager.l2_cache().max_capacity() > 0);
    assert!(manager.l3_cache().max_capacity() > 0);
}

#[tokio::test]
async fn test_multi_layer_cache_manager_get_comprehensive_stats() {
    let l1_cache = Arc::new(MockL1Cache);
    let l2_cache = Arc::new(MockL2Cache);
    let l3_cache = Arc::new(MockL3Cache);
    let config = MultiLayerConfig::new();
    
    let manager = MultiLayerCacheManager::new(
        l1_cache,
        l2_cache,
        l3_cache,
        config,
    );
    
    // Test that we can get comprehensive stats
    let stats = manager.get_comprehensive_stats().await;
    assert_eq!(stats.total_operations, 0);
    assert_eq!(stats.total_hits, 0);
    assert_eq!(stats.total_misses, 0);
    assert_eq!(stats.hit_ratio, 0.0);
}

#[tokio::test]
async fn test_multi_layer_cache_manager_get_health_status() {
    let l1_cache = Arc::new(MockL1Cache);
    let l2_cache = Arc::new(MockL2Cache);
    let l3_cache = Arc::new(MockL3Cache);
    let config = MultiLayerConfig::new();
    
    let manager = MultiLayerCacheManager::new(
        l1_cache,
        l2_cache,
        l3_cache,
        config,
    );
    
    // Test that we can get health status
    let health = manager.get_health_status().await;
    assert_eq!(health.overall, OverallHealth::Healthy);
    assert_eq!(health.l1_health, LayerHealth::Healthy);
    assert_eq!(health.l2_health, LayerHealth::Healthy);
    assert_eq!(health.l3_health, LayerHealth::Healthy);
    assert!(health.efficiency_score >= 0.0);
    assert!(health.efficiency_score <= 1.0);
    assert_eq!(health.total_operations, 0);
    assert_eq!(health.hit_ratio, 0.0);
}

#[tokio::test]
async fn test_multi_layer_cache_manager_cache_interface() {
    let l1_cache = Arc::new(MockL1Cache);
    let l2_cache = Arc::new(MockL2Cache);
    let l3_cache = Arc::new(MockL3Cache);
    let config = MultiLayerConfig::new();
    
    let manager = MultiLayerCacheManager::new(
        l1_cache,
        l2_cache,
        l3_cache,
        config,
    );
    
    // Test Cache trait implementation
    let result = manager.get("test_key");
    assert!(result.is_none());
    
    let set_result = manager.set("test_key".to_string(), serde_json::Value::String("test_value".to_string()), None);
    assert!(set_result.is_ok());
    
    let delete_result = manager.delete("test_key");
    assert!(delete_result.is_ok());
    
    let clear_result = manager.clear();
    assert!(clear_result.is_ok());
    
    let stats = manager.get_stats();
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.sets, 0);
    assert_eq!(stats.deletes, 0);
}

#[test]
fn test_cache_entry_new() {
    let value = serde_json::Value::String("test_value".to_string());
    let entry = CacheEntry::new(value.clone(), Some(300));
    
    assert_eq!(entry.value, value);
    assert_eq!(entry.ttl, Some(300));
    assert!(entry.created_at > 0);
    assert_eq!(entry.created_at, entry.last_accessed);
    assert_eq!(entry.access_count, 0);
    assert!(entry.size > 0);
}

#[test]
fn test_cache_entry_is_expired() {
    let value = serde_json::Value::String("test_value".to_string());
    let entry = CacheEntry::new(value, Some(0)); // TTL of 0 means expired immediately
    
    assert!(entry.is_expired());
}

#[test]
fn test_cache_entry_is_not_expired() {
    let value = serde_json::Value::String("test_value".to_string());
    let entry = CacheEntry::new(value, Some(300)); // TTL of 300 seconds
    
    assert!(!entry.is_expired());
}

#[test]
fn test_cache_entry_is_not_expired_no_ttl() {
    let value = serde_json::Value::String("test_value".to_string());
    let entry = CacheEntry::new(value, None); // No TTL
    
    assert!(!entry.is_expired());
}

#[test]
fn test_cache_entry_touch() {
    let value = serde_json::Value::String("test_value".to_string());
    let mut entry = CacheEntry::new(value, Some(300));
    
    let initial_access_count = entry.access_count;
    
    entry.touch();
    
    assert_eq!(entry.access_count, initial_access_count + 1);
    // Note: last_accessed is updated in touch(), but time comparison is not reliable in tests
}

#[test]
fn test_cache_entry_age() {
    let value = serde_json::Value::String("test_value".to_string());
    let entry = CacheEntry::new(value, Some(300));
    
    let age = entry.age();
    assert!(age >= 0);
}

#[test]
fn test_cache_entry_time_since_last_access() {
    let value = serde_json::Value::String("test_value".to_string());
    let entry = CacheEntry::new(value, Some(300));
    
    let time_since = entry.time_since_last_access();
    assert!(time_since >= 0);
}

#[test]
fn test_layer_config_new() {
    let config = LayerConfig::new(1000);
    
    assert_eq!(config.max_capacity, 1000);
    assert_eq!(config.eviction_policy, EvictionPolicy::Lru);
    assert_eq!(config.default_ttl, None);
    assert!(!config.enable_compression);
    assert_eq!(config.compression_level, 6);
    assert!(!config.enable_encryption);
    assert_eq!(config.encryption_key, None);
}

#[test]
fn test_layer_config_l1() {
    let config = LayerConfig::l1(1000);
    
    assert_eq!(config.max_capacity, 1000);
    assert_eq!(config.eviction_policy, EvictionPolicy::Lru);
    assert_eq!(config.default_ttl, Some(300));
    assert!(!config.enable_compression);
    assert_eq!(config.compression_level, 0);
    assert!(!config.enable_encryption);
    assert_eq!(config.encryption_key, None);
}

#[test]
fn test_layer_config_l2() {
    let config = LayerConfig::l2(10000);
    
    assert_eq!(config.max_capacity, 10000);
    assert_eq!(config.eviction_policy, EvictionPolicy::Lru);
    assert_eq!(config.default_ttl, Some(3600));
    assert!(config.enable_compression);
    assert_eq!(config.compression_level, 6);
    assert!(!config.enable_encryption);
    assert_eq!(config.encryption_key, None);
}

#[test]
fn test_layer_config_l3() {
    let config = LayerConfig::l3(100000);
    
    assert_eq!(config.max_capacity, 100000);
    assert_eq!(config.eviction_policy, EvictionPolicy::Lru);
    assert_eq!(config.default_ttl, Some(86400));
    assert!(config.enable_compression);
    assert_eq!(config.compression_level, 9);
    assert!(config.enable_encryption);
    assert_eq!(config.encryption_key, None);
}

#[test]
fn test_layer_config_validate() {
    let config = LayerConfig::new(1000);
    
    let result = config.validate();
    assert!(result.is_ok());
}

#[test]
fn test_layer_config_validate_zero_capacity() {
    let config = LayerConfig::new(0);
    
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Max capacity must be greater than 0"));
}

#[test]
fn test_layer_config_validate_invalid_compression_level() {
    let mut config = LayerConfig::new(1000);
    config.compression_level = 10; // Invalid compression level
    
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Compression level must be between 1 and 9"));
}

#[test]
fn test_layer_config_validate_encryption_without_key() {
    let mut config = LayerConfig::new(1000);
    config.enable_encryption = true;
    config.encryption_key = None;
    
    let result = config.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Encryption key must be provided when encryption is enabled"));
}

#[test]
fn test_multi_layer_stats_new() {
    let stats = MultiLayerStats::new();
    
    assert_eq!(stats.total_operations, 0);
    assert_eq!(stats.total_hits, 0);
    assert_eq!(stats.total_misses, 0);
    assert_eq!(stats.total_memory_usage, 0);
    assert_eq!(stats.avg_response_time, Duration::from_micros(0));
    assert_eq!(stats.max_response_time, Duration::from_micros(0));
    assert_eq!(stats.hit_ratio, 0.0);
}

#[test]
fn test_multi_layer_stats_update_operation() {
    let mut stats = MultiLayerStats::new();
    
    stats.update_operation(CacheLayer::L1, true, Duration::from_millis(1));
    
    assert_eq!(stats.total_operations, 1);
    assert_eq!(stats.total_hits, 1);
    assert_eq!(stats.total_misses, 0);
    assert_eq!(stats.hit_ratio, 1.0);
}

#[test]
fn test_multi_layer_stats_update_operation_miss() {
    let mut stats = MultiLayerStats::new();
    
    stats.update_operation(CacheLayer::L1, false, Duration::from_millis(1));
    
    assert_eq!(stats.total_operations, 1);
    assert_eq!(stats.total_hits, 0);
    assert_eq!(stats.total_misses, 1);
    assert_eq!(stats.hit_ratio, 0.0);
}

#[test]
fn test_multi_layer_stats_update_operation_multiple() {
    let mut stats = MultiLayerStats::new();
    
    stats.update_operation(CacheLayer::L1, true, Duration::from_millis(1));
    stats.update_operation(CacheLayer::L2, false, Duration::from_millis(2));
    stats.update_operation(CacheLayer::L3, true, Duration::from_millis(3));
    
    assert_eq!(stats.total_operations, 3);
    assert_eq!(stats.total_hits, 2);
    assert_eq!(stats.total_misses, 1);
    assert!((stats.hit_ratio - 0.6666666666666666).abs() < 0.0001);
}

#[test]
fn test_multi_layer_stats_efficiency_score() {
    let mut stats = MultiLayerStats::new();
    
    stats.update_operation(CacheLayer::L1, true, Duration::from_millis(1));
    stats.update_operation(CacheLayer::L2, false, Duration::from_millis(2));
    
    let efficiency_score = stats.efficiency_score();
    assert!(efficiency_score >= 0.0);
    assert!(efficiency_score <= 1.0);
}

#[test]
fn test_multi_layer_stats_reset() {
    let mut stats = MultiLayerStats::new();
    
    stats.update_operation(CacheLayer::L1, true, Duration::from_millis(1));
    stats.update_operation(CacheLayer::L2, false, Duration::from_millis(2));
    
    assert_eq!(stats.total_operations, 2);
    assert_eq!(stats.total_hits, 1);
    assert_eq!(stats.total_misses, 1);
    
    stats.reset();
    
    assert_eq!(stats.total_operations, 0);
    assert_eq!(stats.total_hits, 0);
    assert_eq!(stats.total_misses, 0);
    assert_eq!(stats.hit_ratio, 0.0);
}

#[test]
fn test_eviction_policy_display_name() {
    assert_eq!(EvictionPolicy::Lru.display_name(), "LRU");
    assert_eq!(EvictionPolicy::Lfu.display_name(), "LFU");
    assert_eq!(EvictionPolicy::Fifo.display_name(), "FIFO");
    assert_eq!(EvictionPolicy::Random.display_name(), "Random");
}

#[test]
fn test_eviction_policy_description() {
    assert!(EvictionPolicy::Lru.description().contains("Least Recently Used"));
    assert!(EvictionPolicy::Lfu.description().contains("Least Frequently Used"));
    assert!(EvictionPolicy::Fifo.description().contains("First In, First Out"));
    assert!(EvictionPolicy::Random.description().contains("Random"));
}

#[test]
fn test_eviction_policy_is_suitable_for_size() {
    assert!(EvictionPolicy::Lru.is_suitable_for_size(1000));
    assert!(EvictionPolicy::Lfu.is_suitable_for_size(1000));
    assert!(EvictionPolicy::Fifo.is_suitable_for_size(1000));
    assert!(EvictionPolicy::Random.is_suitable_for_size(1000));
    
    assert!(!EvictionPolicy::Lru.is_suitable_for_size(0));
    assert!(!EvictionPolicy::Lfu.is_suitable_for_size(0));
    assert!(!EvictionPolicy::Fifo.is_suitable_for_size(0));
    assert!(!EvictionPolicy::Random.is_suitable_for_size(0));
}

#[test]
fn test_eviction_policy_default() {
    let policy = EvictionPolicy::default();
    assert_eq!(policy, EvictionPolicy::Lru);
}

#[test]
fn test_eviction_policy_serialization() {
    let policy = EvictionPolicy::Lru;
    let serialized = serde_json::to_string(&policy).unwrap();
    let deserialized: EvictionPolicy = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(policy, deserialized);
}

#[test]
fn test_eviction_policy_all_variants() {
    let variants = vec![
        EvictionPolicy::Lru,
        EvictionPolicy::Lfu,
        EvictionPolicy::Fifo,
        EvictionPolicy::Random,
    ];
    
    for variant in variants {
        assert!(variant.is_suitable_for_size(1000));
        assert!(!variant.is_suitable_for_size(0));
        assert!(!variant.display_name().is_empty());
        assert!(!variant.description().is_empty());
    }
}

#[test]
fn test_cache_health_status_creation() {
    let health = CacheHealthStatus {
        overall: OverallHealth::Healthy,
        l1_health: LayerHealth::Healthy,
        l2_health: LayerHealth::Healthy,
        l3_health: LayerHealth::Healthy,
        efficiency_score: 0.95,
        total_operations: 1000,
        hit_ratio: 0.85,
    };
    
    assert_eq!(health.overall, OverallHealth::Healthy);
    assert_eq!(health.l1_health, LayerHealth::Healthy);
    assert_eq!(health.l2_health, LayerHealth::Healthy);
    assert_eq!(health.l3_health, LayerHealth::Healthy);
    assert_eq!(health.efficiency_score, 0.95);
    assert_eq!(health.total_operations, 1000);
    assert_eq!(health.hit_ratio, 0.85);
}

#[test]
fn test_overall_health_variants() {
    assert_eq!(OverallHealth::Healthy as u8, 0);
    assert_eq!(OverallHealth::Warning as u8, 1);
    assert_eq!(OverallHealth::Unhealthy as u8, 2);
}

#[test]
fn test_layer_health_variants() {
    assert_eq!(LayerHealth::Healthy as u8, 0);
    assert_eq!(LayerHealth::Warning as u8, 1);
    assert_eq!(LayerHealth::Unhealthy as u8, 2);
}

#[test]
fn test_cache_layer_variants() {
    assert_eq!(CacheLayer::L1 as u8, 0);
    assert_eq!(CacheLayer::L2 as u8, 1);
    assert_eq!(CacheLayer::L3 as u8, 2);
}

#[test]
fn test_l1_cache_stats_new() {
    let stats = L1CacheStats::new();
    
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.sets, 0);
    assert_eq!(stats.deletes, 0);
    assert_eq!(stats.memory_usage, 0);
    assert_eq!(stats.max_memory_usage, 0);
    assert_eq!(stats.avg_response_time, Duration::from_micros(0));
    assert_eq!(stats.max_response_time, Duration::from_micros(0));
    assert_eq!(stats.evictions, 0);
}

#[test]
fn test_l2_cache_stats_new() {
    let stats = L2CacheStats::new();
    
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.sets, 0);
    assert_eq!(stats.deletes, 0);
    assert_eq!(stats.memory_usage, 0);
    assert_eq!(stats.max_memory_usage, 0);
    assert_eq!(stats.avg_response_time, Duration::from_micros(0));
    assert_eq!(stats.max_response_time, Duration::from_micros(0));
    assert_eq!(stats.file_operations, 0);
}

#[test]
fn test_l3_cache_stats_new() {
    let stats = L3CacheStats::new();
    
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.sets, 0);
    assert_eq!(stats.deletes, 0);
    assert_eq!(stats.disk_usage, 0);
    assert_eq!(stats.max_disk_usage, 0);
    assert_eq!(stats.avg_response_time, Duration::from_micros(0));
    assert_eq!(stats.max_response_time, Duration::from_micros(0));
    assert_eq!(stats.disk_operations, 0);
}
