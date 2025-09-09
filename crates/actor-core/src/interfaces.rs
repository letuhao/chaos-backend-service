//! Core interfaces for the Actor Core system.
//!
//! This module defines the trait interfaces that components must implement
//! to participate in the actor stat aggregation system.

use async_trait::async_trait;
use crate::types::{Actor, SubsystemOutput, Snapshot, Caps};
use crate::ActorCoreResult;

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

/// SubsystemMetrics contains performance metrics for a subsystem.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubsystemMetrics {
    /// Number of contributions made
    pub contributions_count: u64,
    /// Average processing time in microseconds
    pub avg_processing_time: u64,
    /// Maximum processing time in microseconds
    pub max_processing_time: u64,
    /// Number of errors encountered
    pub error_count: u64,
    /// Last contribution timestamp
    pub last_contribution: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for SubsystemMetrics {
    fn default() -> Self {
        Self {
            contributions_count: 0,
            avg_processing_time: 0,
            max_processing_time: 0,
            error_count: 0,
            last_contribution: None,
        }
    }
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
    fn get_cached_snapshot(&self, actor_id: &uuid::Uuid) -> Option<Snapshot>;
    
    /// Invalidate cache for a specific actor.
    fn invalidate_cache(&self, actor_id: &uuid::Uuid);
    
    /// Clear all caches.
    fn clear_cache(&self);
    
    /// Get aggregator metrics.
    async fn get_metrics(&self) -> AggregatorMetrics;
}

/// AggregatorMetrics contains performance metrics for the aggregator.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregatorMetrics {
    /// Total number of resolutions performed
    pub total_resolutions: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Average resolution time in microseconds
    pub avg_resolution_time: u64,
    /// Maximum resolution time in microseconds
    pub max_resolution_time: u64,
    /// Number of errors encountered
    pub error_count: u64,
    /// Number of active subsystems
    pub active_subsystems: usize,
}

impl Default for AggregatorMetrics {
    fn default() -> Self {
        Self {
            total_resolutions: 0,
            cache_hits: 0,
            cache_misses: 0,
            avg_resolution_time: 0,
            max_resolution_time: 0,
            error_count: 0,
            active_subsystems: 0,
        }
    }
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

/// AcrossLayerPolicy defines how caps are combined across layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AcrossLayerPolicy {
    /// Intersect caps (most restrictive)
    Intersect,
    /// Union caps (least restrictive)
    Union,
    /// Prioritized override (later layers override earlier ones)
    PrioritizedOverride,
}

/// CapStatistics contains statistics about cap usage.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CapStatistics {
    /// Total number of cap calculations
    pub total_calculations: u64,
    /// Number of dimensions with caps
    pub dimensions_with_caps: usize,
    /// Average cap calculation time in microseconds
    pub avg_calculation_time: u64,
    /// Maximum cap calculation time in microseconds
    pub max_calculation_time: u64,
}

impl Default for CapStatistics {
    fn default() -> Self {
        Self {
            total_calculations: 0,
            dimensions_with_caps: 0,
            avg_calculation_time: 0,
            max_calculation_time: 0,
        }
    }
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MergeRule {
    /// Whether to use pipeline processing
    pub use_pipeline: bool,
    /// The operator to use for merging
    pub operator: crate::enums::Operator,
    /// Default clamp range
    pub clamp_default: Option<Caps>,
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
    fn register(&self, subsystem: Box<dyn Subsystem>) -> ActorCoreResult<()>;
    
    /// Unregister a subsystem by ID.
    fn unregister(&self, system_id: &str) -> ActorCoreResult<()>;
    
    /// Get a subsystem by ID.
    fn get_by_id(&self, system_id: &str) -> Option<Box<dyn Subsystem>>;
    
    /// Get all subsystems ordered by priority.
    fn get_by_priority(&self) -> Vec<Box<dyn Subsystem>>;
    
    /// Get subsystems by priority range.
    fn get_by_priority_range(&self, min_priority: i64, max_priority: i64) -> Vec<Box<dyn Subsystem>>;
    
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

/// CacheStats contains statistics about cache usage.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of cache sets
    pub sets: u64,
    /// Number of cache deletes
    pub deletes: u64,
    /// Current memory usage in bytes
    pub memory_usage: u64,
    /// Maximum memory usage in bytes
    pub max_memory_usage: u64,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            sets: 0,
            deletes: 0,
            memory_usage: 0,
            max_memory_usage: 0,
        }
    }
}
