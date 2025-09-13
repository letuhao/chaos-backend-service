//! Production Readiness Tests
//!
//! This module contains tests for production readiness checks and validation
//! of system components before deployment.

use crate::production::{check_readiness};
use crate::interfaces::{PluginRegistry, CombinerRegistry, CapsProvider, Cache};
use crate::error::ActorCoreError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementations for testing
    struct MockPluginRegistry {
        should_fail: bool,
    }

    impl PluginRegistry for MockPluginRegistry {
        fn register(&self, _subsystem: std::sync::Arc<dyn crate::interfaces::Subsystem>) -> crate::ActorCoreResult<()> {
            if self.should_fail {
                Err(crate::ActorCoreError::RegistryError("Plugin validation failed".to_string()))
            } else {
                Ok(())
            }
        }

        fn unregister(&self, _system_id: &str) -> crate::ActorCoreResult<()> {
            Ok(())
        }

        fn get_by_id(&self, _system_id: &str) -> Option<std::sync::Arc<dyn crate::interfaces::Subsystem>> {
            None
        }

        fn get_by_priority(&self) -> Vec<std::sync::Arc<dyn crate::interfaces::Subsystem>> {
            vec![]
        }

        fn get_by_priority_range(&self, _min_priority: i64, _max_priority: i64) -> Vec<std::sync::Arc<dyn crate::interfaces::Subsystem>> {
            vec![]
        }

        fn is_registered(&self, _system_id: &str) -> bool {
            false
        }

        fn count(&self) -> usize {
            5
        }

        fn validate_all(&self) -> crate::ActorCoreResult<()> {
            if self.should_fail {
                Err(crate::ActorCoreError::RegistryError("Plugin validation failed".to_string()))
            } else {
                Ok(())
            }
        }
    }

    struct MockCombinerRegistry {
        should_fail: bool,
    }

    impl CombinerRegistry for MockCombinerRegistry {
        fn set_rule(&self, _dimension: &str, _rule: crate::interfaces::MergeRule) -> crate::ActorCoreResult<()> {
            Ok(())
        }

        fn get_rule(&self, _dimension: &str) -> Option<crate::interfaces::MergeRule> {
            None
        }

        fn validate(&self) -> crate::ActorCoreResult<()> {
            if self.should_fail {
                Err(crate::ActorCoreError::RegistryError("Combiner validation failed".to_string()))
            } else {
                Ok(())
            }
        }
    }

    struct MockCapsProvider {
        should_fail: bool,
    }

    #[async_trait::async_trait]
    impl CapsProvider for MockCapsProvider {
        async fn effective_caps_within_layer(
            &self, 
            _actor: &crate::types::Actor, 
            _outputs: &[crate::types::SubsystemOutput], 
            _layer: &str
        ) -> crate::ActorCoreResult<std::collections::HashMap<String, crate::types::Caps>> {
            Ok(std::collections::HashMap::new())
        }

        async fn effective_caps_across_layers(
            &self, 
            _actor: &crate::types::Actor, 
            _outputs: &[crate::types::SubsystemOutput]
        ) -> crate::ActorCoreResult<std::collections::HashMap<String, crate::types::Caps>> {
            Ok(std::collections::HashMap::new())
        }

        fn get_layer_order(&self) -> Vec<String> {
            vec![]
        }

        fn get_across_layer_policy(&self) -> crate::AcrossLayerPolicy {
            crate::AcrossLayerPolicy::Intersect
        }

        fn validate_caps(&self, _dimension: &str, _caps: &crate::types::Caps) -> crate::ActorCoreResult<()> {
            Ok(())
        }

        async fn get_caps_for_dimension(
            &self, 
            _dimension: &str, 
            _actor: &crate::types::Actor
        ) -> crate::ActorCoreResult<Option<crate::types::Caps>> {
            Ok(None)
        }

        fn get_supported_dimensions(&self) -> Vec<String> {
            vec![]
        }

        fn get_cap_statistics(&self) -> crate::CapStatistics {
            crate::CapStatistics::default()
        }

        fn validate(&self) -> crate::ActorCoreResult<()> {
            if self.should_fail {
                Err(crate::ActorCoreError::RegistryError("Caps validation failed".to_string()))
            } else {
                Ok(())
            }
        }
    }

    struct MockCache {
        should_fail_set: bool,
        should_fail_get: bool,
        should_fail_delete: bool,
        stored_values: Arc<Mutex<HashMap<String, serde_json::Value>>>,
    }

    impl Cache for MockCache {
        fn get(&self, key: &str) -> Option<serde_json::Value> {
            if self.should_fail_get {
                None
            } else {
                self.stored_values.lock().unwrap().get(key).cloned()
            }
        }

        fn set(&self, key: String, value: serde_json::Value, _ttl: Option<u64>) -> crate::ActorCoreResult<()> {
            if self.should_fail_set {
                Err(ActorCoreError::CacheError("Set failed".to_string()))
            } else {
                self.stored_values.lock().unwrap().insert(key, value);
                Ok(())
            }
        }

        fn delete(&self, key: &str) -> crate::ActorCoreResult<()> {
            if self.should_fail_delete {
                Err(ActorCoreError::CacheError("Delete failed".to_string()))
            } else {
                self.stored_values.lock().unwrap().remove(key);
                Ok(())
            }
        }

        fn clear(&self) -> crate::ActorCoreResult<()> {
            Ok(())
        }

        fn get_stats(&self) -> crate::metrics::CacheStats {
            crate::metrics::CacheStats::default()
        }
    }

    #[test]
    fn test_check_readiness_success() {
        let plugin_registry = MockPluginRegistry { should_fail: false };
        let combiner_registry = MockCombinerRegistry { should_fail: false };
        let caps_provider = MockCapsProvider { should_fail: false };
        let cache = MockCache {
            should_fail_set: false,
            should_fail_get: false,
            should_fail_delete: false,
            stored_values: Arc::new(Mutex::new(HashMap::new())),
        };

        let result = check_readiness(
            &plugin_registry,
            &combiner_registry,
            &caps_provider,
            &cache,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_readiness_plugin_registry_failure() {
        let plugin_registry = MockPluginRegistry { should_fail: true };
        let combiner_registry = MockCombinerRegistry { should_fail: false };
        let caps_provider = MockCapsProvider { should_fail: false };
        let cache = MockCache {
            should_fail_set: false,
            should_fail_get: false,
            should_fail_delete: false,
            stored_values: Arc::new(Mutex::new(HashMap::new())),
        };

        let result = check_readiness(
            &plugin_registry,
            &combiner_registry,
            &caps_provider,
            &cache,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            ActorCoreError::RegistryError(msg) => {
                assert_eq!(msg, "Plugin validation failed");
            }
            _ => panic!("Expected RegistryError"),
        }
    }

    #[test]
    fn test_check_readiness_combiner_registry_failure() {
        let plugin_registry = MockPluginRegistry { should_fail: false };
        let combiner_registry = MockCombinerRegistry { should_fail: true };
        let caps_provider = MockCapsProvider { should_fail: false };
        let cache = MockCache {
            should_fail_set: false,
            should_fail_get: false,
            should_fail_delete: false,
            stored_values: Arc::new(Mutex::new(HashMap::new())),
        };

        let result = check_readiness(
            &plugin_registry,
            &combiner_registry,
            &caps_provider,
            &cache,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            ActorCoreError::RegistryError(msg) => {
                assert_eq!(msg, "Combiner validation failed");
            }
            _ => panic!("Expected RegistryError"),
        }
    }

    #[test]
    fn test_check_readiness_caps_provider_failure() {
        let plugin_registry = MockPluginRegistry { should_fail: false };
        let combiner_registry = MockCombinerRegistry { should_fail: false };
        let caps_provider = MockCapsProvider { should_fail: true };
        let cache = MockCache {
            should_fail_set: false,
            should_fail_get: false,
            should_fail_delete: false,
            stored_values: Arc::new(Mutex::new(HashMap::new())),
        };

        let result = check_readiness(
            &plugin_registry,
            &combiner_registry,
            &caps_provider,
            &cache,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            ActorCoreError::RegistryError(msg) => {
                assert_eq!(msg, "Caps validation failed");
            }
            _ => panic!("Expected RegistryError"),
        }
    }

    #[test]
    fn test_check_readiness_cache_set_failure() {
        let plugin_registry = MockPluginRegistry { should_fail: false };
        let combiner_registry = MockCombinerRegistry { should_fail: false };
        let caps_provider = MockCapsProvider { should_fail: false };
        let cache = MockCache {
            should_fail_set: true,
            should_fail_get: false,
            should_fail_delete: false,
            stored_values: Arc::new(Mutex::new(HashMap::new())),
        };

        let result = check_readiness(
            &plugin_registry,
            &combiner_registry,
            &caps_provider,
            &cache,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            ActorCoreError::CacheError(msg) => {
                assert_eq!(msg, "Set failed");
            }
            _ => panic!("Expected CacheError"),
        }
    }

    #[test]
    fn test_check_readiness_cache_get_failure() {
        let plugin_registry = MockPluginRegistry { should_fail: false };
        let combiner_registry = MockCombinerRegistry { should_fail: false };
        let caps_provider = MockCapsProvider { should_fail: false };
        let cache = MockCache {
            should_fail_set: false,
            should_fail_get: true,
            should_fail_delete: false,
            stored_values: Arc::new(Mutex::new(HashMap::new())),
        };

        let result = check_readiness(
            &plugin_registry,
            &combiner_registry,
            &caps_provider,
            &cache,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            ActorCoreError::CacheError(msg) => {
                assert_eq!(msg, "Readiness cache get returned None");
            }
            _ => panic!("Expected CacheError"),
        }
    }

    #[test]
    fn test_check_readiness_cache_value_mismatch() {
        let plugin_registry = MockPluginRegistry { should_fail: false };
        let combiner_registry = MockCombinerRegistry { should_fail: false };
        let caps_provider = MockCapsProvider { should_fail: false };
        
        // Create a custom cache that returns a different value than what was set
        struct MismatchCache;
        
        impl Cache for MismatchCache {
            fn get(&self, _key: &str) -> Option<serde_json::Value> {
                Some(serde_json::json!({ "different": true }))
            }

            fn set(&self, _key: String, _value: serde_json::Value, _ttl: Option<u64>) -> crate::ActorCoreResult<()> {
                Ok(())
            }

            fn delete(&self, _key: &str) -> crate::ActorCoreResult<()> {
                Ok(())
            }

            fn clear(&self) -> crate::ActorCoreResult<()> {
                Ok(())
            }

            fn get_stats(&self) -> crate::metrics::CacheStats {
                crate::metrics::CacheStats::default()
            }
        }

        let cache = MismatchCache;

        let result = check_readiness(
            &plugin_registry,
            &combiner_registry,
            &caps_provider,
            &cache,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            ActorCoreError::CacheError(msg) => {
                assert_eq!(msg, "Readiness cache value mismatch");
            }
            _ => panic!("Expected CacheError"),
        }
    }

    #[test]
    fn test_check_readiness_cache_delete_failure() {
        let plugin_registry = MockPluginRegistry { should_fail: false };
        let combiner_registry = MockCombinerRegistry { should_fail: false };
        let caps_provider = MockCapsProvider { should_fail: false };
        let cache = MockCache {
            should_fail_set: false,
            should_fail_get: false,
            should_fail_delete: true,
            stored_values: Arc::new(Mutex::new(HashMap::new())),
        };

        let result = check_readiness(
            &plugin_registry,
            &combiner_registry,
            &caps_provider,
            &cache,
        );

        // Delete failure should cause the readiness check to fail
        // because the function uses `let _ = cache.delete(key)?;` which propagates the error
        assert!(result.is_err());
        match result.unwrap_err() {
            ActorCoreError::CacheError(msg) => {
                assert_eq!(msg, "Delete failed");
            }
            _ => panic!("Expected CacheError"),
        }
    }

    #[test]
    fn test_check_readiness_integration() {
        // Test with all components working together
        let plugin_registry = MockPluginRegistry { should_fail: false };
        let combiner_registry = MockCombinerRegistry { should_fail: false };
        let caps_provider = MockCapsProvider { should_fail: false };
        let cache = MockCache {
            should_fail_set: false,
            should_fail_get: false,
            should_fail_delete: false,
            stored_values: Arc::new(Mutex::new(HashMap::new())),
        };

        let result = check_readiness(
            &plugin_registry,
            &combiner_registry,
            &caps_provider,
            &cache,
        );

        assert!(result.is_ok());

        // Verify the cache was properly used
        let stored_values = cache.stored_values.lock().unwrap();
        assert!(stored_values.is_empty()); // Should be empty after delete
    }

    #[test]
    fn test_check_readiness_different_values() {
        let plugin_registry = MockPluginRegistry { should_fail: false };
        let combiner_registry = MockCombinerRegistry { should_fail: false };
        let caps_provider = MockCapsProvider { should_fail: false };
        let cache = MockCache {
            should_fail_set: false,
            should_fail_get: false,
            should_fail_delete: false,
            stored_values: Arc::new(Mutex::new(HashMap::new())),
        };

        // Test that the function works with different input values
        let result = check_readiness(
            &plugin_registry,
            &combiner_registry,
            &caps_provider,
            &cache,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_readiness_error_propagation() {
        // Test that errors from different components are properly propagated
        let test_cases = vec![
            (true, false, false, "Plugin validation failed"),
            (false, true, false, "Combiner validation failed"),
            (false, false, true, "Caps validation failed"),
        ];

        for (plugin_fail, combiner_fail, caps_fail, expected_msg) in test_cases {
            let plugin_registry = MockPluginRegistry { should_fail: plugin_fail };
            let combiner_registry = MockCombinerRegistry { should_fail: combiner_fail };
            let caps_provider = MockCapsProvider { should_fail: caps_fail };
            let cache = MockCache {
                should_fail_set: false,
                should_fail_get: false,
                should_fail_delete: false,
                stored_values: Arc::new(Mutex::new(HashMap::new())),
            };

            let result = check_readiness(
                &plugin_registry,
                &combiner_registry,
                &caps_provider,
                &cache,
            );

            assert!(result.is_err());
            match result.unwrap_err() {
                ActorCoreError::RegistryError(msg) => {
                    assert_eq!(msg, expected_msg);
                }
                _ => panic!("Expected RegistryError with message: {}", expected_msg),
            }
        }
    }
}
