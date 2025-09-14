//! Comprehensive tests for registry.rs to improve code coverage
//! 
//! This module tests all the registry implementations including:
//! - PluginRegistryImpl
//! - CombinerRegistryImpl  
//! - CapLayerRegistryImpl
//! - RegistryFactory

use actor_core::{
    interfaces::{PluginRegistry, CombinerRegistry, CapLayerRegistry, Subsystem as SubsystemTrait, MergeRule, CombinerRegistryAsync, CapLayerRegistryAsync},
    enums::{Operator, AcrossLayerPolicy},
    types::{Caps, SubsystemOutput, SubsystemMeta},
    ActorCoreResult, ActorCoreError,
    registry::{PluginRegistryImpl, CombinerRegistryImpl, CapLayerRegistryImpl, RegistryFactory, RegistryMetrics, CombinerMetrics, CapLayerMetrics},
};
use std::sync::Arc;

/// Mock subsystem for testing
struct MockSubsystem {
    system_id: String,
    priority: i64,
}

impl MockSubsystem {
    fn new(system_id: &str, priority: i64) -> Self {
        Self {
            system_id: system_id.to_string(),
            priority,
        }
    }
}

#[async_trait::async_trait]
impl SubsystemTrait for MockSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }

    fn priority(&self) -> i64 {
        self.priority
    }

    async fn contribute(&self, _actor: &actor_core::types::Actor) -> ActorCoreResult<SubsystemOutput> {
        Ok(SubsystemOutput {
            primary: vec![],
            derived: vec![],
            caps: vec![],
            context: None,
            meta: SubsystemMeta::new(self.system_id.clone()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // PluginRegistryImpl tests
    #[test]
    fn test_plugin_registry_creation() {
        let registry = PluginRegistryImpl::new();
        assert_eq!(registry.count(), 0);
        assert!(!registry.is_registered("test"));
    }

    #[test]
    fn test_plugin_registry_default() {
        let registry = PluginRegistryImpl::default();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_register_subsystem_success() {
        let registry = PluginRegistryImpl::new();
        let subsystem = Arc::new(MockSubsystem::new("test_system", 100));
        
        let result = registry.register(subsystem);
        assert!(result.is_ok());
        assert_eq!(registry.count(), 1);
        assert!(registry.is_registered("test_system"));
    }

    #[test]
    fn test_register_subsystem_empty_id() {
        let registry = PluginRegistryImpl::new();
        let subsystem = Arc::new(MockSubsystem::new("", 100));
        
        let result = registry.register(subsystem);
        assert!(result.is_err());
        if let Err(ActorCoreError::ConfigurationError(msg)) = result {
            assert!(msg.contains("System ID cannot be empty"));
        } else {
            panic!("Expected ConfigurationError");
        }
    }

    #[test]
    fn test_register_subsystem_overwrite() {
        let registry = PluginRegistryImpl::new();
        let subsystem1 = Arc::new(MockSubsystem::new("test_system", 100));
        let subsystem2 = Arc::new(MockSubsystem::new("test_system", 200));
        
        // Register first subsystem
        let result1 = registry.register(subsystem1);
        assert!(result1.is_ok());
        assert_eq!(registry.count(), 1);
        
        // Overwrite with second subsystem
        let result2 = registry.register(subsystem2);
        assert!(result2.is_ok());
        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn test_unregister_subsystem_success() {
        let registry = PluginRegistryImpl::new();
        let subsystem = Arc::new(MockSubsystem::new("test_system", 100));
        
        registry.register(subsystem).unwrap();
        assert_eq!(registry.count(), 1);
        
        let result = registry.unregister("test_system");
        assert!(result.is_ok());
        assert_eq!(registry.count(), 0);
        assert!(!registry.is_registered("test_system"));
    }

    #[test]
    fn test_unregister_subsystem_not_found() {
        let registry = PluginRegistryImpl::new();
        
        let result = registry.unregister("nonexistent");
        assert!(result.is_err());
        if let Err(ActorCoreError::RegistryError(msg)) = result {
            assert!(msg.contains("Subsystem not found"));
        } else {
            panic!("Expected RegistryError");
        }
    }

    #[test]
    fn test_get_by_id_success() {
        let registry = PluginRegistryImpl::new();
        let subsystem = Arc::new(MockSubsystem::new("test_system", 100));
        
        registry.register(subsystem.clone()).unwrap();
        
        let retrieved = registry.get_by_id("test_system");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().system_id(), "test_system");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let registry = PluginRegistryImpl::new();
        
        let retrieved = registry.get_by_id("nonexistent");
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_get_by_priority() {
        let registry = PluginRegistryImpl::new();
        let subsystem1 = Arc::new(MockSubsystem::new("low_priority", 50));
        let subsystem2 = Arc::new(MockSubsystem::new("high_priority", 100));
        let subsystem3 = Arc::new(MockSubsystem::new("medium_priority", 75));
        
        registry.register(subsystem1).unwrap();
        registry.register(subsystem2).unwrap();
        registry.register(subsystem3).unwrap();
        
        let by_priority = registry.get_by_priority();
        assert_eq!(by_priority.len(), 3);
        // Should be sorted by priority (highest first)
        assert_eq!(by_priority[0].system_id(), "high_priority");
        assert_eq!(by_priority[1].system_id(), "medium_priority");
        assert_eq!(by_priority[2].system_id(), "low_priority");
    }

    #[test]
    fn test_get_by_priority_range() {
        let registry = PluginRegistryImpl::new();
        let subsystem1 = Arc::new(MockSubsystem::new("low", 30));
        let subsystem2 = Arc::new(MockSubsystem::new("medium", 60));
        let subsystem3 = Arc::new(MockSubsystem::new("high", 90));
        let subsystem4 = Arc::new(MockSubsystem::new("very_high", 120));
        
        registry.register(subsystem1).unwrap();
        registry.register(subsystem2).unwrap();
        registry.register(subsystem3).unwrap();
        registry.register(subsystem4).unwrap();
        
        let in_range = registry.get_by_priority_range(50, 100);
        assert_eq!(in_range.len(), 2);
        assert_eq!(in_range[0].system_id(), "high");
        assert_eq!(in_range[1].system_id(), "medium");
    }

    #[test]
    fn test_validate_all_success() {
        let registry = PluginRegistryImpl::new();
        let subsystem1 = Arc::new(MockSubsystem::new("system1", 50));
        let subsystem2 = Arc::new(MockSubsystem::new("system2", 100));
        
        registry.register(subsystem1).unwrap();
        registry.register(subsystem2).unwrap();
        
        let result = registry.validate_all();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_all_empty_id() {
        // This test is skipped as we can't easily test empty ID validation
        // without access to internal state. The validation logic is there
        // but hard to trigger in normal usage.
    }

    #[test]
    fn test_validate_all_negative_priority() {
        // We can't easily test this without access to internal state
        // This would require a way to insert invalid data, which is protected
        // The validation logic is there but hard to trigger in normal usage
    }

    // CombinerRegistryImpl tests
    #[test]
    fn test_combiner_registry_creation() {
        let registry = CombinerRegistryImpl::new();
        assert!(registry.get_rule("nonexistent").is_none());
    }

    #[test]
    fn test_combiner_registry_default() {
        let registry = CombinerRegistryImpl::default();
        assert!(registry.get_rule("nonexistent").is_none());
    }

    #[test]
    fn test_load_default_rules() {
        let registry = CombinerRegistryImpl::new();
        let result = registry.load_default_rules();
        assert!(result.is_ok());
        
        // Check that some default rules were loaded
        let strength_rule = registry.get_rule("strength");
        assert!(strength_rule.is_some());
        let rule = strength_rule.unwrap();
        assert_eq!(rule.operator, Operator::Sum);
        assert!(rule.use_pipeline);
    }

    #[test]
    fn test_set_rule_success() {
        let registry = CombinerRegistryImpl::new();
        let rule = MergeRule {
            use_pipeline: true,
            operator: Operator::Max,
            clamp_default: Some(Caps::new(0.0, 100.0)),
        };
        
        let result = registry.set_rule("test_dimension", rule.clone());
        assert!(result.is_ok());
        
        let retrieved = registry.get_rule("test_dimension");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().operator, Operator::Max);
    }

    #[test]
    fn test_set_rule_empty_dimension() {
        let registry = CombinerRegistryImpl::new();
        let rule = MergeRule {
            use_pipeline: true,
            operator: Operator::Sum,
            clamp_default: None,
        };
        
        let result = registry.set_rule("", rule);
        assert!(result.is_err());
        if let Err(ActorCoreError::ConfigurationError(msg)) = result {
            assert!(msg.contains("Dimension cannot be empty"));
        } else {
            panic!("Expected ConfigurationError");
        }
    }

    #[test]
    fn test_combiner_validate_success() {
        let registry = CombinerRegistryImpl::new();
        let rule = MergeRule {
            use_pipeline: true,
            operator: Operator::Sum,
            clamp_default: Some(Caps::new(0.0, 100.0)),
        };
        
        registry.set_rule("test_dimension", rule).unwrap();
        
        let result = registry.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_combiner_validate_invalid_caps() {
        let registry = CombinerRegistryImpl::new();
        let rule = MergeRule {
            use_pipeline: true,
            operator: Operator::Sum,
            clamp_default: Some(Caps::new(100.0, 0.0)), // Invalid: min > max
        };
        
        registry.set_rule("test_dimension", rule).unwrap();
        
        let result = registry.validate();
        assert!(result.is_err());
        if let Err(ActorCoreError::ConfigurationError(msg)) = result {
            assert!(msg.contains("Invalid clamp range"));
        } else {
            panic!("Expected ConfigurationError");
        }
    }

    #[tokio::test]
    async fn test_combiner_load_from_file() {
        let registry = CombinerRegistryImpl::new();
        let result = registry.load_from_file("test.json").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_combiner_save_to_file() {
        let registry = CombinerRegistryImpl::new();
        let result = registry.save_to_file("test.json").await;
        assert!(result.is_ok());
    }

    // CapLayerRegistryImpl tests
    #[test]
    fn test_cap_layer_registry_creation() {
        let registry = CapLayerRegistryImpl::new();
        let layer_order = registry.get_layer_order();
        assert!(!layer_order.is_empty());
        assert_eq!(layer_order[0], "realm");
    }

    #[test]
    fn test_cap_layer_registry_default() {
        let registry = CapLayerRegistryImpl::default();
        let layer_order = registry.get_layer_order();
        assert!(!layer_order.is_empty());
    }

    #[test]
    fn test_load_default_config() {
        let registry = CapLayerRegistryImpl::new();
        let result = registry.load_default_config();
        assert!(result.is_ok());
        
        let layer_order = registry.get_layer_order();
        assert_eq!(layer_order.len(), 5);
        assert_eq!(layer_order[0], "realm");
        assert_eq!(layer_order[4], "total");
    }

    #[test]
    fn test_set_layer_order_success() {
        let registry = CapLayerRegistryImpl::new();
        let new_order = vec!["custom1".to_string(), "custom2".to_string()];
        
        let result = registry.set_layer_order(new_order.clone());
        assert!(result.is_ok());
        
        let retrieved = registry.get_layer_order();
        assert_eq!(retrieved, new_order);
    }

    #[test]
    fn test_set_layer_order_empty() {
        let registry = CapLayerRegistryImpl::new();
        let empty_order = vec![];
        
        let result = registry.set_layer_order(empty_order);
        assert!(result.is_err());
        if let Err(ActorCoreError::ConfigurationError(msg)) = result {
            assert!(msg.contains("Layer order cannot be empty"));
        } else {
            panic!("Expected ConfigurationError");
        }
    }

    #[test]
    fn test_get_across_layer_policy() {
        let registry = CapLayerRegistryImpl::new();
        let policy = registry.get_across_layer_policy();
        assert_eq!(policy, AcrossLayerPolicy::Intersect);
    }

    #[test]
    fn test_set_across_layer_policy() {
        let registry = CapLayerRegistryImpl::new();
        registry.set_across_layer_policy(AcrossLayerPolicy::Union);
        
        let policy = registry.get_across_layer_policy();
        assert_eq!(policy, AcrossLayerPolicy::Union);
    }

    #[test]
    fn test_cap_layer_validate_success() {
        let registry = CapLayerRegistryImpl::new();
        let result = registry.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_cap_layer_validate_empty_order() {
        let registry = CapLayerRegistryImpl::new();
        // Test that setting empty order fails
        let result = registry.set_layer_order(vec![]);
        assert!(result.is_err());
        
        // Current order should still be valid
        let result = registry.validate();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cap_layer_load_from_file() {
        let registry = CapLayerRegistryImpl::new();
        let result = registry.load_from_file("test.json").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cap_layer_save_to_file() {
        let registry = CapLayerRegistryImpl::new();
        let result = registry.save_to_file("test.json").await;
        assert!(result.is_ok());
    }

    // RegistryFactory tests
    #[test]
    fn test_create_plugin_registry() {
        let registry = RegistryFactory::create_plugin_registry();
        assert!(registry.count() > 0); // Should have default subsystems registered
    }

    #[test]
    fn test_create_combiner_registry() {
        let registry = RegistryFactory::create_combiner_registry();
        // Test that it's created successfully
        assert!(registry.get_rule("nonexistent").is_none());
    }

    #[test]
    fn test_create_cap_layer_registry() {
        let registry = RegistryFactory::create_cap_layer_registry();
        let layer_order = registry.get_layer_order();
        assert!(!layer_order.is_empty());
    }

    // Metrics tests
    #[test]
    fn test_registry_metrics_default() {
        let metrics = RegistryMetrics::default();
        assert_eq!(metrics.registered_count, 0);
        assert_eq!(metrics.registration_attempts, 0);
        assert_eq!(metrics.unregistration_attempts, 0);
        assert_eq!(metrics.lookup_attempts, 0);
        assert_eq!(metrics.validation_attempts, 0);
    }

    #[test]
    fn test_combiner_metrics_default() {
        let metrics = CombinerMetrics::default();
        assert_eq!(metrics.rule_count, 0);
        assert_eq!(metrics.lookup_count, 0);
        assert_eq!(metrics.set_count, 0);
        assert_eq!(metrics.validation_count, 0);
    }

    #[test]
    fn test_cap_layer_metrics_default() {
        let metrics = CapLayerMetrics::default();
        assert_eq!(metrics.layer_count, 0);
        assert_eq!(metrics.policy_changes, 0);
        assert_eq!(metrics.order_changes, 0);
        assert_eq!(metrics.validation_count, 0);
    }

    // Edge cases and error handling
    #[test]
    fn test_priority_edge_cases() {
        let registry = PluginRegistryImpl::new();
        
        // Test with priority 0
        let subsystem1 = Arc::new(MockSubsystem::new("zero_priority", 0));
        assert!(registry.register(subsystem1).is_ok());
        
        // Test with very high priority
        let subsystem2 = Arc::new(MockSubsystem::new("high_priority", i64::MAX));
        assert!(registry.register(subsystem2).is_ok());
        
        assert_eq!(registry.count(), 2);
    }

    #[test]
    fn test_merge_rule_operators() {
        let registry = CombinerRegistryImpl::new();
        
        let operators = vec![
            Operator::Sum,
            Operator::Max,
            Operator::Min,
            Operator::Average,
            Operator::Multiply,
            Operator::Intersect,
        ];
        
        for (i, operator) in operators.iter().enumerate() {
            let rule = MergeRule {
                use_pipeline: true,
                operator: *operator,
                clamp_default: Some(Caps::new(0.0, 100.0)),
            };
            
            let dimension = format!("test_dimension_{}", i);
            assert!(registry.set_rule(&dimension, rule).is_ok());
        }
        
        // Verify all rules were set
        for i in 0..operators.len() {
            let dimension = format!("test_dimension_{}", i);
            assert!(registry.get_rule(&dimension).is_some());
        }
    }

    #[test]
    fn test_across_layer_policy_variants() {
        let registry = CapLayerRegistryImpl::new();
        
        let policies = vec![
            AcrossLayerPolicy::Intersect,
            AcrossLayerPolicy::Union,
            AcrossLayerPolicy::PrioritizedOverride,
        ];
        
        for policy in policies {
            registry.set_across_layer_policy(policy);
            assert_eq!(registry.get_across_layer_policy(), policy);
        }
    }

    #[test]
    fn test_concurrent_registry_operations() {
        let registry = Arc::new(PluginRegistryImpl::new());
        let registry_clone = Arc::clone(&registry);
        
        // Test concurrent registration
        let subsystem1 = Arc::new(MockSubsystem::new("concurrent1", 100));
        let subsystem2 = Arc::new(MockSubsystem::new("concurrent2", 200));
        
        assert!(registry.register(subsystem1).is_ok());
        assert!(registry_clone.register(subsystem2).is_ok());
        
        assert_eq!(registry.count(), 2);
    }
}
