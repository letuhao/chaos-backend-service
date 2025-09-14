//! Aggregator implementation for the Actor Core system.
//!
//! This module contains the concrete implementation of the Aggregator trait
//! responsible for stat aggregation and snapshot generation.

pub mod optimized;

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::interfaces::{
    Aggregator, PluginRegistry, Cache, CombinerRegistry
};
use crate::metrics::AggregatorMetrics;
use crate::types::*;
use crate::enums::{Bucket, Operator, CapMode};
use crate::ActorCoreResult;

/// AggregatorImpl is the main implementation of the Aggregator trait.
pub struct AggregatorImpl {
    /// Registry for managing subsystems
    subsystem_registry: Arc<dyn PluginRegistry>,
    /// Registry for merge rules and operators
    combiner_registry: Arc<dyn CombinerRegistry>,
    /// Provider for cap calculations
    caps_provider: Arc<dyn crate::interfaces::CapsProvider>,
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
        caps_provider: Arc<dyn crate::interfaces::CapsProvider>,
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

    /// Get subsystems for an actor (helper method).
    fn get_subsystems_for_actor(&self, _actor: &Actor) -> Vec<Arc<dyn crate::interfaces::Subsystem>> {
        // Get all subsystems from the registry
        self.subsystem_registry.get_by_priority()
    }

    /// Process contributions using bucket processor.
    async fn process_contributions(
        &self,
        contributions: Vec<Contribution>,
    ) -> ActorCoreResult<HashMap<String, f64>> {
        // Group contributions by dimension
        let mut grouped: HashMap<String, Vec<Contribution>> = HashMap::new();
        for contrib in contributions {
            grouped.entry(contrib.dimension.clone()).or_insert_with(Vec::new).push(contrib);
        }

        let mut results = HashMap::new();
        
        // Process each dimension
        for (dimension, contribs) in grouped {
            // Get merge rule for this dimension
            let merge_rule = self.combiner_registry.get_rule(&dimension);
            
            // Process the contributions
            let result = self.process_dimension_contributions(contribs, merge_rule).await?;
            results.insert(dimension, result);
        }

        Ok(results)
    }

    /// Process contributions for a specific dimension.
    async fn process_dimension_contributions(
        &self,
        contributions: Vec<Contribution>,
        merge_rule: Option<crate::interfaces::MergeRule>,
    ) -> ActorCoreResult<f64> {
        if contributions.is_empty() {
            return Ok(0.0);
        }

        // Use the default merge rule if none specified
        let rule = merge_rule.unwrap_or(crate::interfaces::MergeRule {
            use_pipeline: false,
            operator: Operator::Sum,
            clamp_default: None,
        });

        // Apply operator logic first, then bucket processing
        let mut result = match rule.operator {
            Operator::Sum => {
                // Process based on bucket type for SUM
                let mut bucket_result = 0.0;
                for contrib in &contributions {
                    match contrib.bucket {
                        Bucket::Flat => {
                            bucket_result += contrib.value;
                        }
                        Bucket::Mult => {
                            bucket_result *= contrib.value;
                        }
                        Bucket::Override => {
                            bucket_result = contrib.value;
                        }
                        Bucket::PostAdd => {
                            bucket_result += contrib.value;
                        }
                    }
                }
                bucket_result
            }
            Operator::Max => {
                // For MAX operator, find the maximum value from all contributions
                contributions.iter()
                    .map(|c| c.value)
                    .fold(f64::NEG_INFINITY, f64::max)
            }
            Operator::Min => {
                // For MIN operator, find the minimum value from all contributions
                contributions.iter()
                    .map(|c| c.value)
                    .fold(f64::INFINITY, f64::min)
            }
            Operator::Average => {
                // For AVERAGE operator, calculate the mean
                if contributions.is_empty() {
                    0.0
                } else {
                    contributions.iter().map(|c| c.value).sum::<f64>() / contributions.len() as f64
                }
            }
            Operator::Multiply => {
                // For MULTIPLY operator, multiply all values
                contributions.iter()
                    .map(|c| c.value)
                    .fold(1.0, |acc, val| acc * val)
            }
            Operator::Intersect => {
                // For INTERSECT operator, find the intersection of ranges
                // This is a simplified implementation - in practice, this would be more complex
                if contributions.is_empty() {
                    0.0
                } else {
                    let min_val = contributions.iter().map(|c| c.value).fold(f64::INFINITY, f64::min);
                    let max_val = contributions.iter().map(|c| c.value).fold(f64::NEG_INFINITY, f64::max);
                    (min_val + max_val) / 2.0 // Return the midpoint for simplicity
                }
            }
        };

        // Apply clamp_default if specified and no effective caps
        if let Some(clamp_default) = rule.clamp_default {
            result = result.max(clamp_default.min).min(clamp_default.max);
        }

        Ok(result)
    }

    /// Apply caps to a stat value.
    async fn apply_caps(
        &self,
        dimension: &str,
        value: f64,
        actor: &Actor,
    ) -> ActorCoreResult<f64> {
        // Get caps for this dimension
        let caps = self.caps_provider.get_caps_for_dimension(dimension, actor).await?;
        
        let mut capped_value = value;
        
        // Apply minimum cap
        if let Some(caps_struct) = caps {
            capped_value = capped_value.max(caps_struct.min);
            capped_value = capped_value.min(caps_struct.max);
        }
        
        Ok(capped_value)
    }

    /// Apply a cap contribution to the caps_used map.
    fn apply_cap_contribution(
        &self,
        caps_used: &mut HashMap<String, Caps>,
        cap_contrib: CapContribution,
    ) {
        let caps = caps_used.entry(cap_contrib.dimension.clone())
            .or_insert_with(|| Caps::new(0.0, 1000.0));
        
        match cap_contrib.mode {
            CapMode::Baseline => {
                caps.set_min(cap_contrib.value);
                caps.set_max(cap_contrib.value);
            },
            CapMode::Additive => {
                caps.expand(cap_contrib.value);
            },
            CapMode::HardMax => {
                caps.set_max(cap_contrib.value);
            },
            CapMode::HardMin => {
                caps.set_min(cap_contrib.value);
            },
            CapMode::Override => {
                caps.set_min(cap_contrib.value);
                caps.set_max(cap_contrib.value);
            },
            CapMode::SoftMax => {
                // SoftMax allows exceeding the cap but applies a penalty
                // For now, treat it the same as HardMax
                caps.set_max(cap_contrib.value);
            },
        }
    }

    /// Create a snapshot from processed stats.
    fn create_snapshot(
        &self,
        actor: &Actor,
        primary_stats: HashMap<String, f64>,
        caps_used: HashMap<String, Caps>,
        subsystems_processed: &[String],
        processing_time: u64,
    ) -> Snapshot {
        Snapshot {
            actor_id: actor.id,
            primary: primary_stats,
            derived: HashMap::new(), // Simplified - no derived stats for now
            caps_used,
            version: actor.version,
            created_at: chrono::Utc::now(),
            subsystems_processed: subsystems_processed.to_vec(),
            processing_time: Some(processing_time),
            metadata: HashMap::new(),
        }
    }
}

#[async_trait]
impl Aggregator for AggregatorImpl {
    async fn resolve(&self, actor: &Actor) -> ActorCoreResult<Snapshot> {
        self.resolve_with_context(actor, None).await
    }

    async fn resolve_with_context(
        &self,
        actor: &Actor,
        _context: Option<HashMap<String, serde_json::Value>>,
    ) -> ActorCoreResult<Snapshot> {
        // Check cache first
        if let Some(cached_snapshot) = self.get_cached_snapshot(&actor.id) {
            // Update cache hit metrics
            {
                let mut metrics = self.metrics.write().await;
                metrics.cache_hits += 1;
            }
            return Ok(cached_snapshot);
        }
        
        let start_time = std::time::Instant::now();
        
        // Get subsystems for this actor
        let subsystems = self.get_subsystems_for_actor(actor);
        let mut subsystems_processed = Vec::new();
        let mut all_contributions = Vec::new();
        let mut caps_used = HashMap::new();

        // Process each subsystem
        for subsystem in subsystems {
            let subsystem_id = subsystem.system_id();
            
            // Get contributions from subsystem
            match subsystem.contribute(actor).await {
                Ok(output) => {
                    // Extract contributions from SubsystemOutput
                    all_contributions.extend(output.primary);
                    all_contributions.extend(output.derived);
                    
                    // Extract caps from SubsystemOutput and apply them to the snapshot
                    for cap_contrib in output.caps {
                        // Apply cap contribution to the snapshot
                        self.apply_cap_contribution(&mut caps_used, cap_contrib);
                    }
                    
                    subsystems_processed.push(subsystem_id.to_string());
                }
                Err(e) => {
                    warn!("Subsystem {} failed to contribute: {}", subsystem_id, e);
                    // Continue with other subsystems
                }
            }
        }

        // Process all contributions
        let primary_stats = self.process_contributions(all_contributions).await?;

        // Apply caps to each stat
        let mut capped_stats = HashMap::new();
        for (dimension, value) in primary_stats {
            let capped_value = if let Some(caps_struct) = caps_used.get(&dimension) {
                caps_struct.clamp(value)
            } else {
                // Fallback to caps provider if no caps from subsystems
                let caps_provider_value = self.apply_caps(&dimension, value, actor).await?;
                
                // If caps provider doesn't provide caps, use constants-based clamping
                if caps_provider_value == value {
                    if let Some((min, max)) = crate::constants::clamp_ranges::get_range(&dimension) {
                        value.max(min).min(max)
                    } else {
                        caps_provider_value
                    }
                } else {
                    caps_provider_value
                }
            };
            capped_stats.insert(dimension.clone(), capped_value);
        }

        let processing_time = start_time.elapsed().as_micros() as u64;

        // Create snapshot
        let snapshot = self.create_snapshot(
            actor,
            capped_stats,
            caps_used,
            &subsystems_processed,
            processing_time,
        );

        // Cache the snapshot
        self.cache.set(
            actor.id.to_string(),
            serde_json::to_value(&snapshot)?,
            Some(3600), // 1 hour TTL
        )?;

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_resolutions += 1;
            metrics.avg_resolution_time = (metrics.avg_resolution_time + processing_time) / 2;
            metrics.max_resolution_time = metrics.max_resolution_time.max(processing_time);
            metrics.active_subsystems = subsystems_processed.len();
        }

        info!(
            "Resolved actor {} with {} subsystems in {}μs",
            actor.id,
            subsystems_processed.len(),
            processing_time
        );

        Ok(snapshot)
    }

    async fn resolve_batch(&self, actors: &[Actor]) -> ActorCoreResult<Vec<Snapshot>> {
        let mut results = Vec::new();
        
        for actor in actors {
            match self.resolve(actor).await {
                Ok(snapshot) => results.push(snapshot),
                Err(e) => {
                    error!("Failed to resolve actor {}: {}", actor.id, e);
                    return Err(e);
                }
            }
        }
        
        Ok(results)
    }

    fn get_cached_snapshot(&self, actor_id: &Uuid) -> Option<Snapshot> {
        match self.cache.get(&actor_id.to_string()) {
            Some(value) => {
                match serde_json::from_value(value) {
                    Ok(snapshot) => Some(snapshot),
                    Err(e) => {
                        warn!("Failed to deserialize cached snapshot for {}: {}", actor_id, e);
                        None
                    }
                }
            }
            None => None,
        }
    }

    fn invalidate_cache(&self, actor_id: &Uuid) {
        if let Err(e) = self.cache.delete(&actor_id.to_string()) {
            warn!("Failed to invalidate cache for {}: {}", actor_id, e);
        }
    }

    fn clear_cache(&self) {
        if let Err(e) = self.cache.clear() {
            warn!("Failed to clear cache: {}", e);
        }
    }

    async fn get_metrics(&self) -> AggregatorMetrics {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }
}