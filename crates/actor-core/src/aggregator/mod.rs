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
use crate::enums::{Bucket, Operator};
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
        let _rule = merge_rule.unwrap_or(crate::interfaces::MergeRule {
            use_pipeline: false,
            operator: Operator::Sum,
            clamp_default: None,
        });

        // Process based on bucket type (simplified)
        let mut result = 0.0;
        for contrib in contributions {
            match contrib.bucket {
                Bucket::Flat => {
                    result += contrib.value;
                }
                Bucket::Mult => {
                    result *= contrib.value;
                }
                Bucket::Override => {
                    result = contrib.value;
                }
                Bucket::PostAdd => {
                    result += contrib.value;
                }
            }
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
            let capped_value = self.apply_caps(&dimension, value, actor).await?;
            capped_stats.insert(dimension.clone(), capped_value);
            
            // Store caps used
            let caps = self.caps_provider.get_caps_for_dimension(&dimension, actor).await?;
            if let Some(caps_struct) = caps {
                caps_used.insert(dimension, caps_struct);
            }
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