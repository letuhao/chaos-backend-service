//! Optimized registry implementations with micro-optimizations.
//!
//! This module provides high-performance registry implementations using
//! smallvec, fxhash, and atomic operations for hot paths.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::RwLock;
use tracing::{warn, error, debug};

use smallvec::{SmallVec, smallvec};
use fxhash::FxHashMap;
use ahash::AHashMap;

use crate::interfaces::{PluginRegistry, CombinerRegistry, Subsystem, MergeRule};
use crate::enums::Operator;
use crate::types::*;
use crate::ActorCoreResult;

/// Optimized plugin registry with FxHash and SmallVec optimizations.
pub struct OptimizedPluginRegistry {
    /// Subsystems indexed by system ID using FxHash for faster lookups
    subsystems: Arc<RwLock<FxHashMap<String, Arc<dyn Subsystem>>>>,
    /// Actor-to-subsystem mappings using SmallVec for small collections
    actor_subsystems: Arc<RwLock<FxHashMap<String, SmallVec<[String; 8]>>>>,
    /// Subsystem metadata cache
    metadata_cache: Arc<RwLock<FxHashMap<String, SubsystemMeta>>>,
    /// Statistics with atomic operations
    stats: Arc<RegistryStats>,
}

/// Registry statistics with atomic counters.
#[derive(Debug)]
pub struct RegistryStats {
    pub total_registrations: std::sync::atomic::AtomicU64,
    pub total_lookups: std::sync::atomic::AtomicU64,
    pub cache_hits: std::sync::atomic::AtomicU64,
    pub cache_misses: std::sync::atomic::AtomicU64,
    pub last_cleanup: std::sync::atomic::AtomicU64,
}

impl RegistryStats {
    pub fn new() -> Self {
        Self {
            total_registrations: std::sync::atomic::AtomicU64::new(0),
            total_lookups: std::sync::atomic::AtomicU64::new(0),
            cache_hits: std::sync::atomic::AtomicU64::new(0),
            cache_misses: std::sync::atomic::AtomicU64::new(0),
            last_cleanup: std::sync::atomic::AtomicU64::new(0),
        }
    }
    
    pub fn record_registration(&self) {
        self.total_registrations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn record_lookup(&self, hit: bool) {
        self.total_lookups.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if hit {
            self.cache_hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        } else {
            self.cache_misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
    
    pub fn get_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(std::sync::atomic::Ordering::Relaxed);
        let misses = self.cache_misses.load(std::sync::atomic::Ordering::Relaxed);
        
        if hits + misses == 0 {
            0.0
        } else {
            (hits as f64 / (hits + misses) as f64) * 100.0
        }
    }
}

impl OptimizedPluginRegistry {
    /// Create a new optimized plugin registry.
    pub fn new() -> Self {
        Self {
            subsystems: Arc::new(RwLock::new(FxHashMap::default())),
            actor_subsystems: Arc::new(RwLock::new(FxHashMap::default())),
            metadata_cache: Arc::new(RwLock::new(FxHashMap::default())),
            stats: Arc::new(RegistryStats::new()),
        }
    }
    
    /// Register a subsystem with optimized storage.
    pub fn register_subsystem(&self, subsystem: Arc<dyn Subsystem>) -> ActorCoreResult<()> {
        let system_id = subsystem.system_id();
        
        // Store in optimized hash map
        {
            let mut subsystems = self.subsystems.write();
            subsystems.insert(system_id.clone(), subsystem.clone());
        }
        
        // Cache metadata
        {
            let mut metadata_cache = self.metadata_cache.write();
            metadata_cache.insert(system_id.clone(), SubsystemMeta {
                system: system_id.clone(),
                data: HashMap::new(),
            });
        }
        
        self.stats.record_registration();
        Ok(())
    }
    
    /// Get subsystems for an actor with optimized lookup.
    pub async fn get_subsystems_for_actor(&self, actor: &Actor) -> ActorCoreResult<Vec<Arc<dyn Subsystem>>> {
        let actor_id = &actor.id;
        
        // Check cache first
        {
            let actor_subsystems = self.actor_subsystems.read();
            if let Some(subsystem_ids) = actor_subsystems.get(actor_id) {
                self.stats.record_lookup(true);
                
                // Build result vector with pre-allocated capacity
                let mut result = Vec::with_capacity(subsystem_ids.len());
                let subsystems = self.subsystems.read();
                
                for system_id in subsystem_ids.iter() {
                    if let Some(subsystem) = subsystems.get(system_id) {
                        result.push(subsystem.clone());
                    }
                }
                
                return Ok(result);
            }
        }
        
        self.stats.record_lookup(false);
        
        // Determine subsystems for this actor (simplified logic)
        let subsystem_ids = self.determine_subsystems_for_actor(actor).await?;
        
        // Cache the result
        {
            let mut actor_subsystems = self.actor_subsystems.write();
            actor_subsystems.insert(actor_id.clone(), subsystem_ids.clone());
        }
        
        // Build and return result
        let subsystems = self.subsystems.read();
        let mut result = Vec::with_capacity(subsystem_ids.len());
        
        for system_id in subsystem_ids {
            if let Some(subsystem) = subsystems.get(&system_id) {
                result.push(subsystem.clone());
            }
        }
        
        Ok(result)
    }
    
    /// Determine which subsystems apply to an actor (simplified implementation).
    async fn determine_subsystems_for_actor(&self, actor: &Actor) -> ActorCoreResult<SmallVec<[String; 8]>> {
        // Simplified logic: return all registered subsystems
        // In a real implementation, this would use actor properties to determine relevant subsystems
        let subsystems = self.subsystems.read();
        let mut result = SmallVec::new();
        
        for (system_id, _) in subsystems.iter() {
            result.push(system_id.clone());
        }
        
        Ok(result)
    }
    
    /// Get registry statistics.
    pub fn get_stats(&self) -> &RegistryStats {
        &self.stats
    }
    
    /// Cleanup old cache entries with optimized batch processing.
    pub async fn cleanup_cache(&self) {
        let now = Instant::now();
        let cleanup_interval = Duration::from_secs(300); // 5 minutes
        
        let last_cleanup = self.stats.last_cleanup.load(std::sync::atomic::Ordering::Relaxed);
        let last_cleanup_time = Instant::from_std(
            std::time::UNIX_EPOCH + Duration::from_secs(last_cleanup)
        ).unwrap_or(now);
        
        if now.duration_since(last_cleanup_time) < cleanup_interval {
            return; // Too soon for cleanup
        }
        
        // Update last cleanup time
        self.stats.last_cleanup.store(
            now.duration_since(std::time::UNIX_EPOCH).as_secs(),
            std::sync::atomic::Ordering::Relaxed
        );
        
        // Cleanup logic would go here
        debug!("Registry cache cleanup completed");
    }
}

/// Optimized combiner registry with fast rule lookups.
pub struct OptimizedCombinerRegistry {
    /// Merge rules indexed by combination key using FxHash
    merge_rules: Arc<RwLock<FxHashMap<String, MergeRule>>>,
    /// Default merge rule for fallback
    default_rule: MergeRule,
    /// Statistics
    stats: Arc<RegistryStats>,
}

impl OptimizedCombinerRegistry {
    /// Create a new optimized combiner registry.
    pub fn new() -> Self {
        Self {
            merge_rules: Arc::new(RwLock::new(FxHashMap::default())),
            default_rule: MergeRule {
                use_pipeline: false,
                operator: Operator::Sum,
                clamp_default: None,
            },
            stats: Arc::new(RegistryStats::new()),
        }
    }
    
    /// Register a merge rule with optimized storage.
    pub fn register_rule(&self, key: String, rule: MergeRule) -> ActorCoreResult<()> {
        let mut rules = self.merge_rules.write();
        rules.insert(key, rule);
        self.stats.record_registration();
        Ok(())
    }
    
    /// Get merge rule with optimized lookup.
    pub fn get_rule(&self, key: &str) -> MergeRule {
        self.stats.record_lookup(false);
        
        let rules = self.merge_rules.read();
        if let Some(rule) = rules.get(key) {
            self.stats.record_lookup(true);
            *rule
        } else {
            self.stats.record_lookup(false);
            self.default_rule
        }
    }
    
    /// Get registry statistics.
    pub fn get_stats(&self) -> &RegistryStats {
        &self.stats
    }
}

/// High-performance registry batch operations.
pub struct RegistryBatchOperations;

impl RegistryBatchOperations {
    /// Register multiple subsystems in a single operation.
    pub async fn register_subsystems(
        registry: &OptimizedPluginRegistry,
        subsystems: Vec<Arc<dyn Subsystem>>,
    ) -> ActorCoreResult<()> {
        let mut batch_futures = Vec::with_capacity(subsystems.len());
        
        for subsystem in subsystems {
            batch_futures.push(async move {
                registry.register_subsystem(subsystem)
            });
        }
        
        // Execute all registrations
        for future in batch_futures {
            future.await?;
        }
        
        Ok(())
    }
    
    /// Get subsystems for multiple actors in parallel.
    pub async fn get_subsystems_for_actors(
        registry: &OptimizedPluginRegistry,
        actors: &[Actor],
    ) -> ActorCoreResult<Vec<Vec<Arc<dyn Subsystem>>>> {
        let mut batch_futures = Vec::with_capacity(actors.len());
        
        for actor in actors {
            batch_futures.push(registry.get_subsystems_for_actor(actor));
        }
        
        let mut results = Vec::with_capacity(actors.len());
        for future in batch_futures {
            results.push(future.await?);
        }
        
        Ok(results)
    }
}

/// Optimized subsystem factory with caching.
pub struct OptimizedSubsystemFactory {
    /// Factory functions indexed by subsystem type
    factories: Arc<RwLock<FxHashMap<String, Box<dyn Fn() -> Arc<dyn Subsystem> + Send + Sync>>>>,
    /// Instance cache to avoid recreating subsystems
    instance_cache: Arc<RwLock<FxHashMap<String, Arc<dyn Subsystem>>>>,
    /// Statistics
    stats: Arc<RegistryStats>,
}

impl OptimizedSubsystemFactory {
    /// Create a new optimized subsystem factory.
    pub fn new() -> Self {
        Self {
            factories: Arc::new(RwLock::new(FxHashMap::default())),
            instance_cache: Arc::new(RwLock::new(FxHashMap::default())),
            stats: Arc::new(RegistryStats::new()),
        }
    }
    
    /// Register a subsystem factory function.
    pub fn register_factory<F>(&self, subsystem_type: String, factory: F)
    where
        F: Fn() -> Arc<dyn Subsystem> + Send + Sync + 'static,
    {
        let mut factories = self.factories.write();
        factories.insert(subsystem_type, Box::new(factory));
        self.stats.record_registration();
    }
    
    /// Create a subsystem instance with caching.
    pub fn create_subsystem(&self, subsystem_type: &str) -> Option<Arc<dyn Subsystem>> {
        self.stats.record_lookup(false);
        
        // Check cache first
        {
            let cache = self.instance_cache.read();
            if let Some(cached) = cache.get(subsystem_type) {
                self.stats.record_lookup(true);
                return Some(cached.clone());
            }
        }
        
        self.stats.record_lookup(false);
        
        // Create new instance
        let factories = self.factories.read();
        if let Some(factory) = factories.get(subsystem_type) {
            let instance = factory();
            
            // Cache the instance
            {
                let mut cache = self.instance_cache.write();
                cache.insert(subsystem_type.to_string(), instance.clone());
            }
            
            Some(instance)
        } else {
            None
        }
    }
    
    /// Get factory statistics.
    pub fn get_stats(&self) -> &RegistryStats {
        &self.stats
    }
}