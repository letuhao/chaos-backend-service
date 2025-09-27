//! Core interfaces for the Actor Core system.
//!
//! This module defines the trait interfaces that components must implement
//! to participate in the actor stat aggregation system.

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use tracing;
use crate::types::{Actor, SubsystemOutput, Snapshot, Caps};
use crate::ActorCoreResult;
use crate::enums::{AcrossLayerPolicy, Operator};

// Import metrics types from the metrics module
use crate::metrics::{SubsystemMetrics, AggregatorMetrics, CapStatistics, CacheStats};

/// Subsystem represents a game system that contributes to actor stats.
#[async_trait]
pub trait Subsystem: Send + Sync {
    /// Get the unique identifier for this subsystem.
    fn system_id(&self) -> &str;
    
    /// Get the priority of this subsystem (higher = more important).
    fn priority(&self) -> i64;
    
    /// Contribute to actor stats.
    /// This method is called during stat aggregation to generate contributions.
    async fn contribute(&self, actor: &Actor) -> ActorCoreResult<SubsystemOutput>;
}

/// Optional trait for subsystems that can be configured.
pub trait ConfigurableSubsystem: Subsystem {
    /// Configure the subsystem with the given configuration.
    fn configure(&mut self, config: std::collections::HashMap<String, serde_json::Value>) -> ActorCoreResult<()>;
    
    /// Get the current configuration.
    fn get_config(&self) -> &std::collections::HashMap<String, serde_json::Value>;
}

/// Optional trait for subsystems that can validate their output.
pub trait ValidatingSubsystem: Subsystem {
    /// Validate the subsystem output.
    fn validate_output(&self, output: &SubsystemOutput) -> ActorCoreResult<()>;
}

/// Optional trait for subsystems that support caching.
pub trait CachingSubsystem: Subsystem {
    /// Get the cache key for this subsystem.
    fn cache_key(&self, actor: &Actor) -> String;
    
    /// Get the cache TTL in seconds.
    fn cache_ttl(&self) -> u64;
}

/// Optional trait for subsystems with lifecycle management.
pub trait LifecycleSubsystem: Subsystem {
    /// Initialize the subsystem.
    #[allow(async_fn_in_trait)]
    async fn initialize(&mut self) -> ActorCoreResult<()>;
    
    /// Shutdown the subsystem.
    #[allow(async_fn_in_trait)]
    async fn shutdown(&mut self) -> ActorCoreResult<()>;
    
    /// Check if the subsystem is healthy.
    fn is_healthy(&self) -> bool;
}

/// Optional trait for event-driven subsystems.
pub trait EventDrivenSubsystem: Subsystem {
    /// Handle an event.
    #[allow(async_fn_in_trait)]
    async fn handle_event(&self, event: &str, data: serde_json::Value) -> ActorCoreResult<()>;
}

/// Optional trait for stateful subsystems.
pub trait StatefulSubsystem: Subsystem {
    /// Get the current state.
    fn get_state(&self) -> serde_json::Value;
    
    /// Set the state.
    fn set_state(&mut self, state: serde_json::Value) -> ActorCoreResult<()>;
}

/// Optional trait for conditional subsystems.
pub trait ConditionalSubsystem: Subsystem {
    /// Check if the subsystem should contribute for the given actor.
    fn should_contribute(&self, actor: &Actor) -> bool;
}

/// Optional trait for performance monitoring.
pub trait PerformanceSubsystem: Subsystem {
    /// Get performance metrics for this subsystem.
    fn get_metrics(&self) -> SubsystemMetrics;
}


/// Aggregator is the main interface for stat aggregation.
#[async_trait]
pub trait Aggregator: Send + Sync {
    /// Resolve actor stats by aggregating contributions from all subsystems.
    async fn resolve(&self, actor: &Actor) -> ActorCoreResult<Snapshot>;
    
    /// Resolve actor stats with additional context.
    async fn resolve_with_context(
        &self, 
        actor: &Actor, 
        context: Option<std::collections::HashMap<String, serde_json::Value>>
    ) -> ActorCoreResult<Snapshot>;
    
    /// Resolve stats for multiple actors in batch.
    async fn resolve_batch(&self, actors: &[Actor]) -> ActorCoreResult<Vec<Snapshot>>;
    
    /// Get a cached snapshot if available.
    fn get_cached_snapshot(&self, actor_id: &String) -> Option<Snapshot>;
    
    /// Invalidate cache for a specific actor.
    fn invalidate_cache(&self, actor_id: &String);
    
    /// Clear all caches.
    fn clear_cache(&self);
    
    /// Get aggregator metrics.
    async fn get_metrics(&self) -> AggregatorMetrics;
}


/// CapsProvider handles cap calculation and management.
#[async_trait]
pub trait CapsProvider: Send + Sync {
    /// Calculate effective caps within a single layer.
    async fn effective_caps_within_layer(
        &self, 
        actor: &Actor, 
        outputs: &[SubsystemOutput], 
        layer: &str
    ) -> ActorCoreResult<std::collections::HashMap<String, Caps>>;
    
    /// Calculate effective caps across all layers.
    async fn effective_caps_across_layers(
        &self, 
        actor: &Actor, 
        outputs: &[SubsystemOutput]
    ) -> ActorCoreResult<std::collections::HashMap<String, Caps>>;
    
    /// Get the order of layers for cap processing.
    fn get_layer_order(&self) -> Vec<String>;
    
    /// Get the policy for combining caps across layers.
    fn get_across_layer_policy(&self) -> AcrossLayerPolicy;
    
    /// Validate caps for a dimension.
    fn validate_caps(&self, dimension: &str, caps: &Caps) -> ActorCoreResult<()>;
    
    /// Get caps for a specific dimension.
    async fn get_caps_for_dimension(
        &self, 
        dimension: &str, 
        actor: &Actor
    ) -> ActorCoreResult<Option<Caps>>;
    
    /// Get all supported dimensions.
    fn get_supported_dimensions(&self) -> Vec<String>;
    
    /// Get cap statistics.
    fn get_cap_statistics(&self) -> CapStatistics;
    
    /// Validate the caps provider configuration.
    fn validate(&self) -> ActorCoreResult<()>;
}



/// CombinerRegistry manages merge rules for dimensions.
pub trait CombinerRegistry: Send + Sync {
    /// Get the merge rule for a dimension.
    fn get_rule(&self, dimension: &str) -> Option<MergeRule>;
    
    /// Set the merge rule for a dimension.
    fn set_rule(&self, dimension: &str, rule: MergeRule) -> ActorCoreResult<()>;
    
    /// Validate all rules.
    fn validate(&self) -> ActorCoreResult<()>;
}

/// CombinerRegistryAsync provides async file operations for CombinerRegistry.
#[async_trait]
pub trait CombinerRegistryAsync: Send + Sync {
    /// Load rules from a file.
    async fn load_from_file(&self, path: &str) -> ActorCoreResult<()>;
    
    /// Save rules to a file.
    async fn save_to_file(&self, path: &str) -> ActorCoreResult<()>;
}

/// MergeRule defines how contributions should be merged for a dimension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRule {
    /// Whether to use pipeline processing
    pub use_pipeline: bool,
    /// The operator to use for merging
    pub operator: Operator,
    /// Default clamp range
    pub clamp_default: Option<Caps>,
}

impl Default for MergeRule {
    fn default() -> Self {
        Self::load_default_rule().unwrap_or_else(|_| {
            tracing::warn!("Failed to load merge rule from config, using hardcoded defaults");
            Self {
                use_pipeline: false,
                operator: Operator::Sum,
                clamp_default: None,
            }
        })
    }
}

impl MergeRule {
    /// Load default merge rule from configuration
    pub fn load_default_rule() -> ActorCoreResult<Self> {
        // Try to load from merge_rule_config.yaml first
        let config_path = std::path::Path::new("configs/merge_rule_config.yaml");
            
        if config_path.exists() {
            match Self::load_rule_from_file(config_path) {
                Ok(rule) => return Ok(rule),
                Err(e) => {
                    tracing::warn!("Failed to load merge rule from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self {
            use_pipeline: false,
            operator: Operator::Sum,
            clamp_default: None,
        })
    }

    /// Load merge rule from file
    fn load_rule_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let rule: MergeRule = serde_yaml::from_str(&content)?;
        Ok(rule)
    }
}

/// CapLayerRegistry manages cap layer configuration.
pub trait CapLayerRegistry: Send + Sync {
    /// Get the order of layers.
    fn get_layer_order(&self) -> Vec<String>;
    
    /// Set the order of layers.
    fn set_layer_order(&self, order: Vec<String>) -> ActorCoreResult<()>;
    
    /// Get the across-layer policy.
    fn get_across_layer_policy(&self) -> AcrossLayerPolicy;
    
    /// Set the across-layer policy.
    fn set_across_layer_policy(&self, policy: AcrossLayerPolicy);
    
    /// Validate the configuration.
    fn validate(&self) -> ActorCoreResult<()>;
}

/// CapLayerRegistryAsync provides async file operations for CapLayerRegistry.
#[async_trait]
pub trait CapLayerRegistryAsync: Send + Sync {
    /// Load configuration from a file.
    async fn load_from_file(&self, path: &str) -> ActorCoreResult<()>;
    
    /// Save configuration to a file.
    async fn save_to_file(&self, path: &str) -> ActorCoreResult<()>;
}

/// PluginRegistry manages subsystem registration and retrieval.
pub trait PluginRegistry: Send + Sync {
    /// Register a subsystem.
    fn register(&self, subsystem: std::sync::Arc<dyn Subsystem>) -> ActorCoreResult<()>;
    
    /// Unregister a subsystem by ID.
    fn unregister(&self, system_id: &str) -> ActorCoreResult<()>;
    
    /// Get a subsystem by ID.
    fn get_by_id(&self, system_id: &str) -> Option<std::sync::Arc<dyn Subsystem>>;
    
    /// Get all subsystems ordered by priority.
    fn get_by_priority(&self) -> Vec<std::sync::Arc<dyn Subsystem>>;
    
    /// Get subsystems by priority range.
    fn get_by_priority_range(&self, min_priority: i64, max_priority: i64) -> Vec<std::sync::Arc<dyn Subsystem>>;
    
    /// Check if a subsystem is registered.
    fn is_registered(&self, system_id: &str) -> bool;
    
    /// Get the number of registered subsystems.
    fn count(&self) -> usize;
    
    /// Validate all registered subsystems.
    fn validate_all(&self) -> ActorCoreResult<()>;
}

/// Cache provides caching functionality for the system.
pub trait Cache: Send + Sync {
    /// Get a value from the cache.
    fn get(&self, key: &str) -> Option<serde_json::Value>;
    
    /// Set a value in the cache.
    fn set(&self, key: String, value: serde_json::Value, ttl: Option<u64>) -> ActorCoreResult<()>;
    
    /// Delete a value from the cache.
    fn delete(&self, key: &str) -> ActorCoreResult<()>;
    
    /// Clear all values from the cache.
    fn clear(&self) -> ActorCoreResult<()>;
    
    /// Get cache statistics.
    fn get_stats(&self) -> CacheStats;
}