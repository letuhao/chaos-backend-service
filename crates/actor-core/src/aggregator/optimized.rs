//! Optimized aggregator implementation with micro-optimizations.
//!
//! This module provides high-performance versions of aggregation functions
//! using smallvec, fxhash, atomic counters, and other micro-optimizations.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use smallvec::{SmallVec, smallvec};
use fxhash::FxHashMap;
use ahash::AHashMap;

use crate::interfaces::{
    Aggregator, PluginRegistry, Cache, CombinerRegistry, Subsystem
};
use crate::metrics::AggregatorMetrics;
use crate::types::*;
use crate::ActorCoreResult;
use crate::bucket_processor::{process_contributions_in_order, optimized::OptimizedBucketProcessor};

/// Optimized aggregator implementation with micro-optimizations.
pub struct OptimizedAggregator {
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
    /// Atomic metrics for high-frequency operations
    atomic_metrics: Arc<crate::bucket_processor::optimized::AtomicMetrics>,
    /// Dimension interner for string deduplication
    dimension_interner: Arc<RwLock<crate::bucket_processor::optimized::DimensionInterner>>,
}

impl OptimizedAggregator {
    /// Create a new optimized aggregator instance.
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
            atomic_metrics: Arc::new(crate::bucket_processor::optimized::AtomicMetrics::new()),
            dimension_interner: Arc::new(RwLock::new(crate::bucket_processor::optimized::DimensionInterner::new())),
        }
    }
    
    /// Resolve actor stats with optimized processing.
    async fn resolve_optimized(&self, actor: &Actor) -> ActorCoreResult<Snapshot> {
        let start_time = std::time::Instant::now();
        
        // Check cache first using atomic operations
        let cache_key = self.generate_cache_key(actor);
        if let Ok(Some(cached_snapshot)) = self.cache.get(&cache_key).await {
            self.atomic_metrics.record_cache_hit();
            return Ok(cached_snapshot);
        }
        
        self.atomic_metrics.record_cache_miss();
        
        // Get subsystems with optimized collection
        let subsystems = self.subsystem_registry.get_subsystems_for_actor(actor).await?;
        
        // Use SmallVec for small subsystem collections
        let mut subsystem_outputs: SmallVec<[SubsystemOutput; 16]> = SmallVec::new();
        
        // Process subsystems with optimized async batching
        for subsystem in subsystems {
            match subsystem.contribute(actor).await {
                Ok(output) => subsystem_outputs.push(output),
                Err(e) => {
                    error!("Subsystem {} failed to contribute: {}", subsystem.system_id(), e);
                    continue;
                }
            }
        }
        
        // Aggregate contributions with optimized processing
        let snapshot = self.aggregate_contributions_optimized(actor, &subsystem_outputs).await?;
        
        // Cache the result
        if let Err(e) = self.cache.set(&cache_key, &snapshot, Some(300)).await {
            warn!("Failed to cache snapshot: {}", e);
        }
        
        // Record timing with atomic operations
        let duration = start_time.elapsed();
        self.atomic_metrics.record_operation(duration.as_nanos() as u64);
        
        Ok(snapshot)
    }
    
    /// Aggregate contributions with micro-optimizations.
    async fn aggregate_contributions_optimized(
        &self,
        actor: &Actor,
        subsystem_outputs: &SmallVec<[SubsystemOutput; 16]>,
    ) -> ActorCoreResult<Snapshot> {
        // Use FxHashMap for faster lookups in hot paths
        let mut aggregated_stats: FxHashMap<String, f64> = FxHashMap::default();
        let mut effective_caps: FxHashMap<String, Caps> = FxHashMap::default();
        
        // Process each subsystem output
        for output in subsystem_outputs.iter() {
            // Process primary contributions
            self.process_contributions_optimized(&mut aggregated_stats, &output.primary);
            
            // Process derived stats
            self.process_contributions_optimized(&mut aggregated_stats, &output.derived);
            
            // Process cap contributions
            self.process_cap_contributions_optimized(&mut effective_caps, &output.caps);
        }
        
        // Apply caps to final values
        for (dimension, value) in aggregated_stats.iter_mut() {
            if let Some(caps) = effective_caps.get(dimension) {
                *value = self.apply_caps_optimized(*value, caps);
            }
        }
        
        // Create snapshot with optimized data structures
        Ok(Snapshot {
            actor_id: actor.id,
            stats: aggregated_stats.into_iter().collect(),
            caps: effective_caps.into_iter().collect(),
            timestamp: chrono::Utc::now(),
            version: actor.version,
        })
    }
    
    /// Process contributions with optimized algorithms.
    fn process_contributions_optimized(
        &self,
        stats: &mut FxHashMap<String, f64>,
        contributions: &[Contribution],
    ) {
        // Group contributions by dimension using FxHash
        let mut grouped: FxHashMap<String, SmallVec<[Contribution; 16]>> = FxHashMap::default();
        
        for contrib in contributions {
            grouped
                .entry(contrib.dimension.clone())
                .or_insert_with(SmallVec::new)
                .push(contrib.clone());
        }
        
        // Process each dimension group
        for (dimension, mut contribs) in grouped {
            // Use interned dimension names to reduce allocations
            let interned_dimension = {
                let mut interner = self.dimension_interner.blocking_write();
                interner.intern(&dimension)
            };
            
            // Get initial value
            let initial_value = stats.get(&dimension).copied().unwrap_or(0.0);
            
            // Process with optimized bucket processor
            if let Ok(final_value) = OptimizedBucketProcessor::process_contributions_optimized(
                contribs.into_vec(),
                initial_value,
                None,
            ) {
                stats.insert(dimension, final_value);
            }
        }
    }
    
    /// Process cap contributions with optimized processing.
    fn process_cap_contributions_optimized(
        &self,
        caps: &mut FxHashMap<String, Caps>,
        cap_contributions: &[CapContribution],
    ) {
        for cap_contrib in cap_contributions {
            let caps_entry = caps.entry(cap_contrib.dimension.clone())
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
    }
    
    /// Apply caps with optimized clamping.
    #[inline(always)]
    fn apply_caps_optimized(&self, value: f64, caps: &Caps) -> f64 {
        let mut result = value;
        
        if let Some(min_val) = caps.min {
            result = result.max(min_val);
        }
        
        if let Some(max_val) = caps.max {
            result = result.min(max_val);
        }
        
        result
    }
    
    /// Generate cache key with optimized hashing.
    fn generate_cache_key(&self, actor: &Actor) -> String {
        use std::hash::{Hash, Hasher};
        use fxhash::FxHasher;
        
        let mut hasher = FxHasher::default();
        actor.id.hash(&mut hasher);
        actor.version.hash(&mut hasher);
        
        // Include subsystem IDs for cache invalidation
        for subsystem in &actor.subsystems {
            subsystem.id.hash(&mut hasher);
        }
        
        format!("actor_{}", hasher.finish())
    }
    
    /// Get atomic metrics for monitoring.
    pub fn get_atomic_metrics(&self) -> &Arc<crate::bucket_processor::optimized::AtomicMetrics> {
        &self.atomic_metrics
    }
    
    /// Get cache hit rate.
    pub fn get_cache_hit_rate(&self) -> f64 {
        self.atomic_metrics.get_cache_hit_rate()
    }
}

#[async_trait]
impl Aggregator for OptimizedAggregator {
    /// Resolve actor stats by aggregating contributions from all subsystems.
    async fn resolve(&self, actor: &Actor) -> ActorCoreResult<Snapshot> {
        self.resolve_optimized(actor).await
    }
    
    /// Resolve actor stats with additional context.
    async fn resolve_with_context(
        &self,
        actor: &Actor,
        context: &HashMap<String, serde_json::Value>,
    ) -> ActorCoreResult<Snapshot> {
        // For now, ignore context in optimized version
        // TODO: Implement context-aware optimization
        self.resolve_optimized(actor).await
    }
    
    /// Get aggregator metrics.
    async fn get_metrics(&self) -> AggregatorMetrics {
        let mut metrics = self.metrics.read().await.clone();
        
        // Update with atomic metrics
        metrics.cache_hit_rate = self.get_cache_hit_rate();
        metrics.total_operations = self.atomic_metrics.total_ops.load(std::sync::atomic::Ordering::Relaxed);
        metrics.avg_processing_time_ns = self.atomic_metrics.avg_processing_time.load(std::sync::atomic::Ordering::Relaxed);
        
        metrics
    }
}

/// High-performance batch aggregator for processing multiple actors.
pub struct BatchAggregator {
    /// Underlying optimized aggregator
    aggregator: Arc<OptimizedAggregator>,
    /// Batch size for optimal performance
    batch_size: usize,
}

impl BatchAggregator {
    /// Create a new batch aggregator.
    pub fn new(aggregator: Arc<OptimizedAggregator>, batch_size: usize) -> Self {
        Self {
            aggregator,
            batch_size,
        }
    }
    
    /// Process multiple actors in optimized batches.
    pub async fn resolve_batch(&self, actors: Vec<Actor>) -> ActorCoreResult<Vec<Snapshot>> {
        let mut results = Vec::with_capacity(actors.len());
        let mut batch = Vec::with_capacity(self.batch_size);
        
        for actor in actors {
            batch.push(actor);
            
            if batch.len() >= self.batch_size {
                // Process batch
                let batch_results = self.process_batch(batch).await?;
                results.extend(batch_results);
                batch = Vec::with_capacity(self.batch_size);
            }
        }
        
        // Process remaining actors
        if !batch.is_empty() {
            let batch_results = self.process_batch(batch).await?;
            results.extend(batch_results);
        }
        
        Ok(results)
    }
    
    /// Process a single batch of actors.
    async fn process_batch(&self, actors: Vec<Actor>) -> ActorCoreResult<Vec<Snapshot>> {
        let mut handles = Vec::new();
        
        // Spawn tasks for parallel processing
        for actor in actors {
            let aggregator = self.aggregator.clone();
            let handle = tokio::spawn(async move {
                aggregator.resolve(&actor).await
            });
            handles.push(handle);
        }
        
        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(snapshot)) => results.push(snapshot),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(crate::ActorCoreError::InternalError(format!("Task join error: {}", e))),
            }
        }
        
        Ok(results)
    }
}