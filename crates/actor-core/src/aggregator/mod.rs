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

use crate::interfaces::{
    Aggregator, PluginRegistry, Cache, CombinerRegistry
};
use crate::metrics::AggregatorMetrics;
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

    /// Resolve actor stats by aggregating contributions from all subsystems.
    async fn resolve_impl(&self, actor: &Actor) -> ActorCoreResult<Snapshot> {
        let start_time = std::time::Instant::now();
        
        // Check cache first
        let cache_key = self.generate_cache_key(actor);
        if let Ok(Some(cached_snapshot)) = self.cache.get(&cache_key).await {
            info!("Cache hit for actor {}", actor.id);
            return Ok(cached_snapshot);
        }
        
        info!("Cache miss for actor {}, computing snapshot", actor.id);
        
        // Get subsystems for this actor
        let subsystems = self.subsystem_registry.get_subsystems_for_actor(actor).await?;
        
        // Collect contributions from all subsystems
        let mut all_contributions = Vec::new();
        let mut all_cap_contributions = Vec::new();
        
        for subsystem in subsystems {
            match subsystem.contribute(actor).await {
                Ok(output) => {
                    all_contributions.extend(output.primary);
                    all_contributions.extend(output.derived);
                    all_cap_contributions.extend(output.caps);
                }
                Err(e) => {
                    error!("Subsystem {} failed to contribute: {}", subsystem.system_id(), e);
                    continue;
                }
            }
        }
        
        // Group contributions by dimension
        let contributions_by_dimension = self.group_contributions_by_dimension(all_contributions);
        
        // Calculate effective caps
        let effective_caps = self.calculate_effective_caps(all_cap_contributions).await?;
        
        // Aggregate stats for each dimension
        let mut aggregated_stats = HashMap::new();
        for (dimension, contributions) in contributions_by_dimension {
            let initial_value = 0.0;
            let caps = effective_caps.get(&dimension);
            
            let final_value = process_contributions_in_order(contributions, initial_value, caps)?;
            aggregated_stats.insert(dimension, final_value);
        }
        
        // Create snapshot
        let snapshot = Snapshot {
            actor_id: actor.id,
            stats: aggregated_stats,
            caps: effective_caps,
            timestamp: chrono::Utc::now(),
            version: actor.version,
        };
        
        // Cache the result
        if let Err(e) = self.cache.set(&cache_key, &snapshot, Some(300)).await {
            warn!("Failed to cache snapshot: {}", e);
        }
        
        // Update metrics
        let processing_time = start_time.elapsed();
        self.update_metrics(processing_time).await;
        
        Ok(snapshot)
    }
    
    /// Group contributions by dimension for processing.
    fn group_contributions_by_dimension(
        &self,
        contributions: Vec<Contribution>,
    ) -> HashMap<String, Vec<Contribution>> {
        let mut groups: HashMap<String, Vec<Contribution>> = HashMap::new();
        
        for contrib in contributions {
            groups.entry(contrib.dimension).or_insert_with(Vec::new).push(contrib);
        }
        
        groups
    }
    
    /// Calculate effective caps from cap contributions.
    async fn calculate_effective_caps(
        &self,
        cap_contributions: Vec<CapContribution>,
    ) -> ActorCoreResult<HashMap<String, Caps>> {
        let mut caps_map: HashMap<String, Caps> = HashMap::new();
        
        for cap_contrib in cap_contributions {
            let caps_entry = caps_map.entry(cap_contrib.dimension.clone())
                .or_insert_with(|| Caps {
                    min: None,
                    max: None,
                });
            
            match cap_contrib.kind.as_str() {
                "min" => {
                    if let Some(current_min) = caps_entry.min {
                        caps_entry.min = Some(current_min.max(cap_contrib.value));
                    } else {
                        caps_entry.min = Some(cap_contrib.value);
                    }
                }
                "max" => {
                    if let Some(current_max) = caps_entry.max {
                        caps_entry.max = Some(current_max.min(cap_contrib.value));
                    } else {
                        caps_entry.max = Some(cap_contrib.value);
                    }
                }
            }
        }
        
        Ok(caps_map)
    }
    
    /// Generate cache key for an actor.
    fn generate_cache_key(&self, actor: &Actor) -> String {
        format!("actor_{}_{}", actor.id, actor.version)
    }
    
    /// Update aggregator metrics.
    async fn update_metrics(&self, processing_time: std::time::Duration) {
        let mut metrics = self.metrics.write().await;
        metrics.total_aggregations += 1;
        metrics.total_processing_time += processing_time;
        metrics.avg_aggregation_time = metrics.total_processing_time.as_nanos() as f64 / metrics.total_aggregations as f64;
    }
}

#[async_trait]
impl Aggregator for AggregatorImpl {
    /// Resolve actor stats by aggregating contributions from all subsystems.
    async fn resolve(&self, actor: &Actor) -> ActorCoreResult<Snapshot> {
        self.resolve_impl(actor).await
    }
    
    /// Resolve actor stats with additional context.
    async fn resolve_with_context(
        &self,
        actor: &Actor,
        context: &HashMap<String, serde_json::Value>,
    ) -> ActorCoreResult<Snapshot> {
        // For now, ignore context
        // TODO: Implement context-aware aggregation
        self.resolve_impl(actor).await
    }
    
    /// Get aggregator metrics.
    async fn get_metrics(&self) -> AggregatorMetrics {
        self.metrics.read().await.clone()
    }
}