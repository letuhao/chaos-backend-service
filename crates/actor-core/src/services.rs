//! Service implementations for the Actor Core system.
//!
//! This module contains the concrete implementations of the core services
//! including the aggregator, caps provider, and related functionality.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use crate::interfaces::{
    Aggregator, CapsProvider, AcrossLayerPolicy,
    PluginRegistry, Cache, CapLayerRegistry, CombinerRegistry, AggregatorMetrics, CapStatistics
};
use crate::types::*;
use crate::ActorCoreResult;
use crate::bucket_processor::*;

/// AggregatorImpl is the main implementation of the Aggregator trait.
pub struct AggregatorImpl {
    /// Registry for managing subsystems
    subsystem_registry: Arc<dyn PluginRegistry>,
    /// Registry for merge rules and operators
    combiner_registry: Arc<dyn CombinerRegistry>,
    /// Provider for cap calculations
    caps_provider: Arc<dyn CapsProvider>,
    /// Cache for storing snapshots
    cache: Arc<dyn Cache>,
    /// Metrics for performance monitoring
    metrics: Arc<RwLock<AggregatorMetrics>>,
}

impl AggregatorImpl {
    /// Create a new aggregator instance.
    pub fn new(
        subsystem_registry: Arc<dyn PluginRegistry>,
        combiner_registry: Arc<dyn CombinerRegistry>,
        caps_provider: Arc<dyn CapsProvider>,
        cache: Arc<dyn Cache>,
    ) -> Self {
        Self {
            subsystem_registry,
            combiner_registry,
            caps_provider,
            cache,
            metrics: Arc::new(RwLock::new(AggregatorMetrics::default())),
        }
    }

    /// Aggregate primary stats from subsystem outputs.
    async fn aggregate_primary_stats(
        &self,
        outputs: &[SubsystemOutput],
        effective_caps: &HashMap<String, Caps>,
    ) -> ActorCoreResult<HashMap<String, f64>> {
        let mut primary_stats = HashMap::new();
        
        // Group contributions by dimension
        let mut contributions_by_dimension: HashMap<String, Vec<&Contribution>> = HashMap::new();
        
        for output in outputs {
            for contribution in &output.primary {
                contributions_by_dimension
                    .entry(contribution.dimension.clone())
                    .or_insert_with(Vec::new)
                    .push(contribution);
            }
        }

        // Process each dimension
        for (dimension, contributions) in contributions_by_dimension {
            // Convert references to owned values for the bucket processor
            let owned_contributions: Vec<Contribution> = contributions
                .into_iter()
                .map(|c| c.clone())
                .collect();
            
            // Validate contributions before processing
            validate_contributions(&owned_contributions)?;
            
            // Get merge rule for this dimension from CombinerRegistry
            let merge_rule = self.combiner_registry.get_rule(&dimension);

            // Determine clamping caps in priority order:
            // 1) effective caps (runtime, layered)
            // 2) combiner rule default clamp (if configured)
            // 3) constants::clamp_ranges fallback (built-in)
            let rule_clamp_default = merge_rule.as_ref().and_then(|r| r.clamp_default.as_ref());
            let mut _builtin_caps_holder: Option<Caps> = None;
            let mut clamp_caps = effective_caps.get(&dimension).or(rule_clamp_default);
            if clamp_caps.is_none() {
                if let Some((min, max)) = crate::constants::clamp_ranges::get_range(&dimension) {
                    _builtin_caps_holder = Some(Caps::new(min, max));
                    clamp_caps = _builtin_caps_holder.as_ref();
                }
            }
            
            // Process contributions using the appropriate method based on merge rule
            let value = if let Some(rule) = merge_rule.as_ref() {
                if rule.use_pipeline {
                    // Use pipeline processing (bucket order)
                    process_contributions_in_order(
                        owned_contributions,
                        0.0, // Initial value
                        clamp_caps,
                    )?
                } else {
                    // Use operator-based processing
                    self.process_contributions_with_operator(
                        owned_contributions,
                        rule.operator,
                        clamp_caps,
                    )?
                }
            } else {
                // Default to pipeline processing if no rule is found
                process_contributions_in_order(
                    owned_contributions,
                    0.0, // Initial value
                    clamp_caps,
                )?
            };

            primary_stats.insert(dimension, value);
        }

        Ok(primary_stats)
    }

    /// Process contributions using a specific operator instead of pipeline processing.
    fn process_contributions_with_operator(
        &self,
        contributions: Vec<Contribution>,
        operator: crate::enums::Operator,
        clamp_caps: Option<&Caps>,
    ) -> ActorCoreResult<f64> {
        if contributions.is_empty() {
            return Ok(0.0);
        }

        let values: Vec<f64> = contributions.into_iter().map(|c| c.value).collect();
        let mut result = match operator {
            crate::enums::Operator::Sum => {
                values.iter().sum()
            }
            crate::enums::Operator::Max => {
                values.into_iter().fold(f64::NEG_INFINITY, f64::max)
            }
            crate::enums::Operator::Min => {
                values.into_iter().fold(f64::INFINITY, f64::min)
            }
            crate::enums::Operator::Average => {
                let sum: f64 = values.iter().sum();
                sum / values.len() as f64
            }
            crate::enums::Operator::Multiply => {
                values.into_iter().fold(1.0, |acc, val| acc * val)
            }
            crate::enums::Operator::Intersect => {
                // For intersect, we take the minimum value (most restrictive)
                values.into_iter().fold(f64::INFINITY, f64::min)
            }
        };

        // Apply clamping if provided
        if let Some(caps) = clamp_caps {
            result = caps.clamp(result);
        }

        Ok(result)
    }

    /// Aggregate derived stats from subsystem outputs.
    async fn aggregate_derived_stats(
        &self,
        outputs: &[SubsystemOutput],
        _primary_stats: &HashMap<String, f64>,
        effective_caps: &HashMap<String, Caps>,
    ) -> ActorCoreResult<HashMap<String, f64>> {
        let mut derived_stats = HashMap::new();
        
        // Group contributions by dimension
        let mut contributions_by_dimension: HashMap<String, Vec<&Contribution>> = HashMap::new();
        
        for output in outputs {
            for contribution in &output.derived {
                contributions_by_dimension
                    .entry(contribution.dimension.clone())
                    .or_insert_with(Vec::new)
                    .push(contribution);
            }
        }

        // Process each dimension
        for (dimension, contributions) in contributions_by_dimension {
            // Convert references to owned values for the bucket processor
            let owned_contributions: Vec<Contribution> = contributions
                .into_iter()
                .map(|c| c.clone())
                .collect();
            
            // Validate contributions before processing
            validate_contributions(&owned_contributions)?;
            
            // Get merge rule for this dimension from CombinerRegistry
            let merge_rule = self.combiner_registry.get_rule(&dimension);

            // Determine clamping caps in priority order:
            // 1) effective caps (runtime, layered)
            // 2) combiner rule default clamp (if configured)
            // 3) constants::clamp_ranges fallback (built-in)
            let rule_clamp_default = merge_rule.as_ref().and_then(|r| r.clamp_default.as_ref());
            let mut _builtin_caps_holder: Option<Caps> = None;
            let mut clamp_caps = effective_caps.get(&dimension).or(rule_clamp_default);
            if clamp_caps.is_none() {
                if let Some((min, max)) = crate::constants::clamp_ranges::get_range(&dimension) {
                    _builtin_caps_holder = Some(Caps::new(min, max));
                    clamp_caps = _builtin_caps_holder.as_ref();
                }
            }
            
            // Process contributions using the appropriate method based on merge rule
            let value = if let Some(rule) = merge_rule.as_ref() {
                if rule.use_pipeline {
                    // Use pipeline processing (bucket order)
                    process_contributions_in_order(
                        owned_contributions,
                        0.0, // Initial value
                        clamp_caps,
                    )?
                } else {
                    // Use operator-based processing
                    self.process_contributions_with_operator(
                        owned_contributions,
                        rule.operator,
                        clamp_caps,
                    )?
                }
            } else {
                // Default to pipeline processing if no rule is found
                process_contributions_in_order(
                    owned_contributions,
                    0.0, // Initial value
                    clamp_caps,
                )?
            };

            derived_stats.insert(dimension, value);
        }

        Ok(derived_stats)
    }
}

#[async_trait]
impl Aggregator for AggregatorImpl {
    async fn resolve(&self, actor: &Actor) -> ActorCoreResult<Snapshot> {
        let start_time = std::time::Instant::now();
        
        // Check cache first
        let cache_key = format!("{}:{}", actor.id, actor.version);
        if let Some(cached_value) = self.cache.get(&cache_key) {
            if let Ok(snapshot) = serde_json::from_value::<Snapshot>(cached_value) {
                // Update metrics
                let mut metrics = self.metrics.write().await;
                metrics.cache_hits += 1;
                metrics.total_resolutions += 1;
                return Ok(snapshot);
            }
        }

        // Update cache miss
        {
            let mut metrics = self.metrics.write().await;
            metrics.cache_misses += 1;
        }

        // Get all subsystems
        let subsystems = self.subsystem_registry.get_by_priority();
        
        // Collect contributions from all subsystems
        let mut outputs = Vec::new();
        let mut subsystems_processed = Vec::new();
        
        for subsystem in subsystems {
            match subsystem.contribute(actor).await {
                Ok(output) => {
                    outputs.push(output);
                    subsystems_processed.push(subsystem.system_id().to_string());
                }
                Err(e) => {
                    warn!("Subsystem {} failed to contribute: {}", subsystem.system_id(), e);
                }
            }
        }

        // Calculate effective caps
        let effective_caps = self.caps_provider
            .effective_caps_across_layers(actor, &outputs)
            .await?;

        // Aggregate primary stats
        let primary_stats = self.aggregate_primary_stats(&outputs, &effective_caps).await?;
        
        // Aggregate derived stats
        let derived_stats = self.aggregate_derived_stats(&outputs, &primary_stats, &effective_caps).await?;

        // Create snapshot
        let mut snapshot = Snapshot::new(actor.id, actor.version);
        snapshot.primary = primary_stats;
        snapshot.derived = derived_stats;
        snapshot.caps_used = effective_caps;
        snapshot.subsystems_processed = subsystems_processed;
        snapshot.processing_time = Some(start_time.elapsed().as_micros() as u64);

        // Cache the result
        if let Ok(snapshot_value) = serde_json::to_value(&snapshot) {
            if let Err(e) = self.cache.set(cache_key, snapshot_value, Some(3600)) {
                warn!("Failed to cache snapshot: {}", e);
            }
        }

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_resolutions += 1;
            let processing_time = start_time.elapsed().as_micros() as u64;
            metrics.avg_resolution_time = (metrics.avg_resolution_time + processing_time) / 2;
            metrics.max_resolution_time = metrics.max_resolution_time.max(processing_time);
        }

        info!("Resolved actor {} in {}μs", actor.id, start_time.elapsed().as_micros());
        Ok(snapshot)
    }

    async fn resolve_with_context(
        &self,
        actor: &Actor,
        _context: Option<HashMap<String, serde_json::Value>>,
    ) -> ActorCoreResult<Snapshot> {
        // For now, context is ignored
        // In a real implementation, this would affect how subsystems contribute
        self.resolve(actor).await
    }

    async fn resolve_batch(&self, actors: &[Actor]) -> ActorCoreResult<Vec<Snapshot>> {
        let mut snapshots = Vec::new();
        
        for actor in actors {
            match self.resolve(actor).await {
                Ok(snapshot) => snapshots.push(snapshot),
                Err(e) => {
                    error!("Failed to resolve actor {}: {}", actor.id, e);
                }
            }
        }
        
        Ok(snapshots)
    }

    fn get_cached_snapshot(&self, actor_id: &uuid::Uuid) -> Option<Snapshot> {
        // Try different cache key formats
        let cache_key1 = format!("{}:1", actor_id); // Format used in resolve method
        let cache_key2 = format!("actor_snapshot:{}", actor_id); // Alternative format
        
        self.cache.get(&cache_key1)
            .or_else(|| self.cache.get(&cache_key2))
            .and_then(|v| serde_json::from_value::<Snapshot>(v).ok())
    }

    fn invalidate_cache(&self, actor_id: &uuid::Uuid) {
        // Try both cache key formats
        let cache_key1 = format!("{}:1", actor_id);
        let cache_key2 = format!("actor_snapshot:{}", actor_id);
        
        if let Err(e) = self.cache.delete(&cache_key1) {
            warn!("Failed to invalidate cache for actor {}: {}", actor_id, e);
        }
        if let Err(e) = self.cache.delete(&cache_key2) {
            warn!("Failed to invalidate cache for actor {}: {}", actor_id, e);
        }
    }

    fn clear_cache(&self) {
        if let Err(e) = self.cache.clear() {
            warn!("Failed to clear cache: {}", e);
        }
    }

    async fn get_metrics(&self) -> AggregatorMetrics {
        // Return the actual metrics from the aggregator
        let metrics = self.metrics.read().await;
        AggregatorMetrics {
            total_resolutions: metrics.total_resolutions,
            cache_hits: metrics.cache_hits,
            cache_misses: metrics.cache_misses,
            avg_resolution_time: metrics.avg_resolution_time,
            max_resolution_time: metrics.max_resolution_time,
            error_count: metrics.error_count,
            active_subsystems: metrics.active_subsystems,
        }
    }
}

/// CapsProviderImpl is the implementation of the CapsProvider trait.
pub struct CapsProviderImpl {
    /// Registry for cap layer configuration
    cap_layer_registry: Arc<dyn CapLayerRegistry>,
    /// Metrics for performance monitoring
    #[allow(dead_code)]
    metrics: Arc<RwLock<CapStatistics>>,
}

impl CapsProviderImpl {
    /// Create a new caps provider instance.
    pub fn new(cap_layer_registry: Arc<dyn CapLayerRegistry>) -> Self {
        Self {
            cap_layer_registry,
            metrics: Arc::new(RwLock::new(CapStatistics::default())),
        }
    }
}

#[async_trait]
impl CapsProvider for CapsProviderImpl {
    async fn effective_caps_within_layer(
        &self,
        _actor: &Actor,
        outputs: &[SubsystemOutput],
        layer: &str,
    ) -> ActorCoreResult<HashMap<String, Caps>> {
        let mut caps_by_dimension: HashMap<String, Vec<&CapContribution>> = HashMap::new();
        
        // Group cap contributions by dimension
        for output in outputs {
            for cap in &output.caps {
                if cap.scope.as_deref() == Some(layer) {
                    caps_by_dimension
                        .entry(cap.dimension.clone())
                        .or_insert_with(Vec::new)
                        .push(cap);
                }
            }
        }

        let mut effective_caps = HashMap::new();
        
        // Process each dimension
        for (dimension, caps) in caps_by_dimension {
            let mut min_cap = f64::NEG_INFINITY;
            let mut max_cap = f64::INFINITY;
            
            // Sort caps by priority
            let mut sorted_caps = caps;
            sorted_caps.sort_by(|a, b| {
                b.priority.unwrap_or(0).cmp(&a.priority.unwrap_or(0))
            });

            // Apply caps based on mode
            for cap in sorted_caps {
                match cap.mode {
                    crate::enums::CapMode::Baseline => {
                        if cap.kind == "min" {
                            min_cap = cap.value;
                        } else if cap.kind == "max" {
                            max_cap = cap.value;
                        }
                    }
                    crate::enums::CapMode::Additive => {
                        if cap.kind == "min" {
                            min_cap += cap.value;
                        } else if cap.kind == "max" {
                            max_cap += cap.value;
                        }
                    }
                    crate::enums::CapMode::HardMax => {
                        if cap.kind == "max" {
                            max_cap = max_cap.min(cap.value);
                        }
                    }
                    crate::enums::CapMode::HardMin => {
                        if cap.kind == "min" {
                            min_cap = min_cap.max(cap.value);
                        }
                    }
                    crate::enums::CapMode::Override => {
                        if cap.kind == "min" {
                            min_cap = cap.value;
                        } else if cap.kind == "max" {
                            max_cap = cap.value;
                        }
                    }
                }
            }

            // Create effective caps
            if min_cap.is_finite() || max_cap.is_finite() {
                effective_caps.insert(dimension, Caps::new(min_cap, max_cap));
            }
        }

        Ok(effective_caps)
    }

    async fn effective_caps_across_layers(
        &self,
        actor: &Actor,
        outputs: &[SubsystemOutput],
    ) -> ActorCoreResult<HashMap<String, Caps>> {
        let layer_order = self.get_layer_order();
        let mut all_caps: HashMap<String, Caps> = HashMap::new();

        // Process each layer
        for layer in layer_order {
            let layer_caps = self.effective_caps_within_layer(actor, outputs, &layer).await?;
            
            // Combine with existing caps based on policy
            let policy = self.get_across_layer_policy();
            for (dimension, caps) in layer_caps {
                match policy {
                    AcrossLayerPolicy::Intersect => {
                        if let Some(existing_caps) = all_caps.get(&dimension) {
                            all_caps.insert(dimension, existing_caps.intersection(&caps));
                        } else {
                            all_caps.insert(dimension, caps);
                        }
                    }
                    AcrossLayerPolicy::Union => {
                        if let Some(existing_caps) = all_caps.get(&dimension) {
                            all_caps.insert(dimension, existing_caps.union(&caps));
                        } else {
                            all_caps.insert(dimension, caps);
                        }
                    }
                    AcrossLayerPolicy::PrioritizedOverride => {
                        // Later layers override earlier ones
                        all_caps.insert(dimension, caps);
                    }
                }
            }
        }

        Ok(all_caps)
    }

    fn get_layer_order(&self) -> Vec<String> {
        self.cap_layer_registry.get_layer_order()
    }

    fn get_across_layer_policy(&self) -> AcrossLayerPolicy {
        self.cap_layer_registry.get_across_layer_policy()
    }

    fn validate_caps(&self, dimension: &str, caps: &Caps) -> ActorCoreResult<()> {
        if !caps.is_valid() {
            return Err(crate::ActorCoreError::InvalidCap(
                format!("Invalid caps for dimension {}: {:?}", dimension, caps)
            ));
        }
        Ok(())
    }

    async fn get_caps_for_dimension(
        &self,
        _dimension: &str,
        _actor: &Actor,
    ) -> ActorCoreResult<Option<Caps>> {
        // This is a simplified implementation
        // In a real implementation, this would query the database or cache
        Ok(None)
    }

    fn get_supported_dimensions(&self) -> Vec<String> {
        crate::constants::all_dimensions().into_iter().map(String::from).collect()
    }

    fn get_cap_statistics(&self) -> CapStatistics {
        // This is a simplified implementation
        CapStatistics::default()
    }

    fn validate(&self) -> ActorCoreResult<()> {
        self.cap_layer_registry.validate()
    }
}


/// Factory for creating service instances.
pub struct ServiceFactory;

impl ServiceFactory {
    /// Create a new aggregator instance.
    pub fn create_aggregator(
        subsystem_registry: Arc<dyn PluginRegistry>,
        combiner_registry: Arc<dyn CombinerRegistry>,
        caps_provider: Arc<dyn CapsProvider>,
        cache: Arc<dyn Cache>,
    ) -> Arc<dyn Aggregator> {
        Arc::new(AggregatorImpl::new(subsystem_registry, combiner_registry, caps_provider, cache))
    }

    /// Create a new caps provider instance.
    pub fn create_caps_provider(
        cap_layer_registry: Arc<dyn CapLayerRegistry>,
    ) -> Arc<dyn CapsProvider> {
        Arc::new(CapsProviderImpl::new(cap_layer_registry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple tests for basic functionality
    #[test]
    fn test_aggregator_metrics_default() {
        let metrics = AggregatorMetrics::default();
        assert_eq!(metrics.cache_hits, 0);
        assert_eq!(metrics.cache_misses, 0);
    }

    #[test]
    fn test_aggregator_metrics_creation() {
        let metrics = AggregatorMetrics {
            total_resolutions: 10,
            cache_hits: 8,
            cache_misses: 2,
            avg_resolution_time: 100,
            max_resolution_time: 200,
            error_count: 0,
            active_subsystems: 5,
        };
        assert_eq!(metrics.cache_hits, 8);
        assert_eq!(metrics.cache_misses, 2);
    }

    #[test]
    fn test_actor_creation() {
        let actor = Actor::new("test_actor".to_string(), "human".to_string());
        assert_eq!(actor.name, "test_actor");
        assert_eq!(actor.race, "human");
        assert!(actor.is_valid());
    }

    #[test]
    fn test_actor_validation() {
        let mut actor = Actor::new("".to_string(), "human".to_string()); // Empty name
        assert!(!actor.is_valid());
        
        actor.name = "valid_name".to_string();
        actor.race = "".to_string(); // Empty race
        assert!(!actor.is_valid());
        
        actor.race = "valid_race".to_string();
        assert!(actor.is_valid());
    }

    #[test]
    fn test_actor_operations() {
        let mut actor = Actor::new("test_actor".to_string(), "human".to_string());
        
        // Test basic actor operations
        assert!(actor.is_valid());
        assert!(!actor.is_in_combat());
        assert!(!actor.has_buffs());
        
        // Test touch operation
        let original_version = actor.version;
        actor.touch();
        assert!(actor.version > original_version);
    }

    #[test]
    fn test_caps_creation() {
        let caps = Caps::new(100.0, 200.0);
        assert_eq!(caps.min, 100.0);
        assert_eq!(caps.max, 200.0);
    }

    #[test]
    fn test_caps_operations() {
        let caps = Caps::new(100.0, 200.0);
        
        // Test individual cap values
        assert_eq!(caps.min, 100.0);
        assert_eq!(caps.max, 200.0);
        
        // Test range calculation
        let range = caps.max - caps.min;
        assert_eq!(range, 100.0);
    }

    #[test]
    fn test_caps_validation() {
        let caps = Caps::new(-100.0, 200.0); // Negative min
        // Caps should allow negative values for testing purposes
        assert_eq!(caps.min, -100.0);
        assert_eq!(caps.max, 200.0);
    }

    #[test]
    fn test_zero_caps() {
        let caps = Caps::new(0.0, 0.0);
        assert_eq!(caps.min, 0.0);
        assert_eq!(caps.max, 0.0);
    }

    #[test]
    fn test_large_caps() {
        let caps = Caps::new(1000000.0, 2000000.0);
        assert_eq!(caps.min, 1000000.0);
        assert_eq!(caps.max, 2000000.0);
    }

    // Test concurrent access patterns
    #[tokio::test]
    async fn test_concurrent_actor_creation() {
        let mut handles = vec![];
        
        for i in 0..10 {
            let handle = tokio::spawn(async move {
                let actor = Actor::new(format!("actor_{}", i), "human".to_string());
                assert_eq!(actor.name, format!("actor_{}", i));
                actor
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let actor = handle.await.unwrap();
            assert!(actor.is_valid());
        }
    }

    // Test edge cases
    #[test]
    fn test_actor_edge_cases() {
        // Test very long names
        let long_name = "a".repeat(1000);
        let actor = Actor::new(long_name.clone(), "human".to_string());
        assert_eq!(actor.name, long_name);
        assert!(actor.is_valid());
    }

    #[test]
    fn test_caps_edge_cases() {
        // Test very large values
        let caps = Caps::new(f64::MAX, f64::MAX);
        assert_eq!(caps.min, f64::MAX);
        assert_eq!(caps.max, f64::MAX);
    }

    #[test]
    fn test_metrics_edge_cases() {
        // Test very large metric values
        let metrics = AggregatorMetrics {
            total_resolutions: u64::MAX,
            cache_hits: u64::MAX,
            cache_misses: u64::MAX,
            avg_resolution_time: u64::MAX,
            max_resolution_time: u64::MAX,
            error_count: u64::MAX,
            active_subsystems: usize::MAX,
        };
        assert_eq!(metrics.cache_hits, u64::MAX);
        assert_eq!(metrics.cache_misses, u64::MAX);
    }
}