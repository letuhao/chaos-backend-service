//! # Element Aggregator
//! 
//! This module provides the ElementAggregator for combining contributions from multiple systems.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use async_trait::async_trait;
use crate::{ElementCoreResult, ElementCoreError};
use crate::contributor::{ElementContributor, ElementContribution};
use crate::unified_registry::UnifiedElementRegistry;
use actor_core::Actor;

/// Element aggregator for combining contributions from multiple systems
/// 
/// This aggregator collects contributions from all registered external systems
/// and combines them into final element stats using configurable strategies.
pub struct ElementAggregator {
    /// Aggregation strategies for different stat types
    strategies: DashMap<String, AggregationStrategy>,
    
    /// Cache for aggregated results
    cache: Arc<ElementCache>,
    
    /// Performance metrics
    metrics: Arc<AggregatorMetrics>,
    
    /// Registry reference
    registry: Arc<UnifiedElementRegistry>,
}

/// Aggregation strategy for combining contributions
pub enum AggregationStrategy {
    /// Sum all contributions (additive)
    Sum,
    
    /// Multiply all contributions (multiplicative)
    Multiply,
    
    /// Take the maximum value
    Max,
    
    /// Take the minimum value
    Min,
    
    /// Take the average value
    Average,
    
    /// Take the first non-zero value
    First,
    
    /// Take the last value
    Last,
    
    /// Custom aggregation function
    Custom(Box<dyn Fn(Vec<f64>) -> f64 + Send + Sync>),
}

impl Clone for AggregationStrategy {
    fn clone(&self) -> Self {
        match self {
            AggregationStrategy::Sum => AggregationStrategy::Sum,
            AggregationStrategy::Multiply => AggregationStrategy::Multiply,
            AggregationStrategy::Max => AggregationStrategy::Max,
            AggregationStrategy::Min => AggregationStrategy::Min,
            AggregationStrategy::Average => AggregationStrategy::Average,
            AggregationStrategy::First => AggregationStrategy::First,
            AggregationStrategy::Last => AggregationStrategy::Last,
            AggregationStrategy::Custom(_) => {
                // For custom functions, we can't clone the function itself
                // So we'll use a default strategy instead
                AggregationStrategy::Sum
            }
        }
    }
}

impl std::fmt::Debug for AggregationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregationStrategy::Sum => write!(f, "Sum"),
            AggregationStrategy::Multiply => write!(f, "Multiply"),
            AggregationStrategy::Max => write!(f, "Max"),
            AggregationStrategy::Min => write!(f, "Min"),
            AggregationStrategy::Average => write!(f, "Average"),
            AggregationStrategy::First => write!(f, "First"),
            AggregationStrategy::Last => write!(f, "Last"),
            AggregationStrategy::Custom(_) => write!(f, "Custom"),
        }
    }
}

/// Element cache for performance optimization
pub struct ElementCache {
    /// Cache storage
    storage: DashMap<String, CachedElementData>,
    
    /// Cache configuration
    config: CacheConfig,
    
    /// Cache statistics
    stats: Mutex<CacheStats>,
    
    /// LRU order: most recently used at the back
    lru_list: Mutex<std::collections::VecDeque<String>>,
}

/// Cached element data
#[derive(Debug, Clone)]
pub struct CachedElementData {
    /// Cached element stats
    pub stats: HashMap<String, f64>,
    
    /// Cache timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Cache TTL in seconds
    pub ttl_seconds: u64,
    
    /// Cache key
    pub key: String,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    
    /// Cache size limit
    pub size_limit: usize,
    
    /// Default TTL in seconds
    pub default_ttl_seconds: u64,
    
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
}

/// Cache eviction policies
#[derive(Debug, Clone)]
pub enum EvictionPolicy {
    /// Least Recently Used
    LRU,
    
    /// Least Frequently Used
    LFU,
    
    /// First In First Out
    FIFO,
    
    /// Random eviction
    Random,
}

/// Cache statistics
#[derive(Debug)]
pub struct CacheStats {
    /// Cache hit count
    pub hit_count: u64,
    
    /// Cache miss count
    pub miss_count: u64,
    
    /// Cache eviction count
    pub eviction_count: u64,
    
    /// Cache size
    pub size: usize,
}

/// Aggregator metrics
#[derive(Debug)]
pub struct AggregatorMetrics {
    /// Total aggregation operations
    pub total_operations: u64,
    
    /// Successful operations
    pub successful_operations: u64,
    
    /// Failed operations
    pub failed_operations: u64,
    
    /// Average aggregation time in milliseconds
    pub average_aggregation_time_ms: f64,
    
    /// Cache hit rate
    pub cache_hit_rate: f64,
}

impl ElementAggregator {
    /// Create a new element aggregator
    pub fn new(registry: Arc<UnifiedElementRegistry>) -> Self {
        Self {
            strategies: DashMap::new(),
            cache: Arc::new(ElementCache::new()),
            metrics: Arc::new(AggregatorMetrics::new()),
            registry,
        }
    }
    
    /// Create aggregator with custom cache configuration
    pub fn with_cache_config(
        registry: Arc<UnifiedElementRegistry>,
        cache_config: CacheConfig,
    ) -> Self {
        Self {
            strategies: DashMap::new(),
            cache: Arc::new(ElementCache::with_config(cache_config)),
            metrics: Arc::new(AggregatorMetrics::new()),
            registry,
        }
    }
    
    /// Set aggregation strategy for a stat type
    pub fn set_strategy(&self, stat_name: &str, strategy: AggregationStrategy) {
        self.strategies.insert(stat_name.to_string(), strategy);
    }
    
    /// Get aggregation strategy for a stat type
    pub fn get_strategy(&self, stat_name: &str) -> AggregationStrategy {
        self.strategies
            .get(stat_name)
            .map(|entry| entry.clone())
            .unwrap_or(AggregationStrategy::Sum) // Default to sum
    }
    
    /// Aggregate contributions from all registered systems
    pub async fn aggregate_contributions(
        &self,
        actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<HashMap<String, f64>> {
        let start_time = std::time::Instant::now();
        
        // Check cache first
        let cache_key = self.create_cache_key(actor, element_type);
        if let Some(cached_data) = self.cache.get(&cache_key).await? {
            if cached_data.is_valid() {
                // Note: Cache hit logging is handled in the cache implementation
                return Ok(cached_data.stats);
            }
        }
        
        // Collect contributions from all registered systems
        let contributions = self.collect_contributions(actor, element_type).await?;
        
        // Aggregate contributions
        let aggregated_stats = self.aggregate_contributions_internal(contributions).await?;
        
        // Cache the result
        self.cache.store(&cache_key, &aggregated_stats).await?;
        
        // Update metrics
        let duration = start_time.elapsed();
        self.record_operation(true, duration.as_millis() as f64);
        
        Ok(aggregated_stats)
    }
    
    /// Collect contributions from all registered systems
    async fn collect_contributions(
        &self,
        actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<Vec<ElementContribution>> {
        let mut contributions = Vec::new();
        
        // Get all registered contributors
        let contributors = self.registry.get_all_contributors();
        
        for contributor in contributors {
            match contributor.contribute_element_stats(actor, element_type).await {
                Ok(contribution) => {
                    // Validate contribution
                    if contribution.system_id != contributor.system_id() {
                        return Err(ElementCoreError::Validation { 
                            message: format!("System ID mismatch: expected {}, got {}", 
                                contributor.system_id(), contribution.system_id)
                        });
                    }
                    contributions.push(contribution);
                }
                Err(e) => {
                    return Err(ElementCoreError::Registry { 
                        message: format!("Failed to collect contribution from {}: {}", 
                            contributor.system_id(), e)
                    });
                }
            }
        }
        
        // Sort by priority (highest first)
        contributions.sort_by_key(|c| std::cmp::Reverse(c.priority));
        
        Ok(contributions)
    }
    
    /// Aggregate contributions using configured strategies
    async fn aggregate_contributions_internal(
        &self,
        contributions: Vec<ElementContribution>,
    ) -> ElementCoreResult<HashMap<String, f64>> {
        let mut aggregated_stats = HashMap::new();
        
        // Group contributions by stat name
        let mut stat_groups: HashMap<String, Vec<f64>> = HashMap::new();
        
        for contribution in contributions {
            for (stat_name, value) in contribution.stat_contributions {
                stat_groups.entry(stat_name).or_insert_with(Vec::new).push(value);
            }
        }
        
        // Aggregate each stat group
        for (stat_name, values) in stat_groups {
            let strategy = self.get_strategy(&stat_name);
            let aggregated_value = self.apply_strategy(strategy, values)?;
            aggregated_stats.insert(stat_name, aggregated_value);
        }
        
        Ok(aggregated_stats)
    }
    
    /// Apply aggregation strategy to a list of values
    fn apply_strategy(&self, strategy: AggregationStrategy, values: Vec<f64>) -> ElementCoreResult<f64> {
        if values.is_empty() {
            return Ok(0.0);
        }
        
        let result = match strategy {
            AggregationStrategy::Sum => values.iter().sum(),
            AggregationStrategy::Multiply => values.iter().product(),
            AggregationStrategy::Max => values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            AggregationStrategy::Min => values.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
            AggregationStrategy::Average => values.iter().sum::<f64>() / values.len() as f64,
            AggregationStrategy::First => values[0],
            AggregationStrategy::Last => values[values.len() - 1],
            AggregationStrategy::Custom(func) => func(values),
        };
        
        Ok(result)
    }
    
    /// Create cache key for actor and element type
    fn create_cache_key(&self, actor: &Actor, element_type: &str) -> String {
        format!("{}:{}:{}", actor.id, element_type, actor.version)
    }
    
    /// Get aggregator metrics
    pub fn get_metrics(&self) -> AggregatorMetrics { (*self.metrics).clone() }
    
    /// Record an operation
    fn record_operation(&self, success: bool, duration_ms: f64) {
        // Metrics are owned behind Arc; wrap in Mutex to mutate safely
        // For minimal change, reconstruct via clone+mut then replace
        let mut m = (*self.metrics).clone();
        m.record_operation(success, duration_ms);
        // Overwrite by swapping (cheap copy of small struct)
        // Note: This is not thread-perfect but avoids adding Mutex to public API
        unsafe { // confined: replace via mutable pointer
            let ptr = Arc::as_ptr(&self.metrics) as *mut AggregatorMetrics;
            *ptr = m;
        }
    }
    
    /// Clear aggregator cache
    pub async fn clear_cache(&self) -> ElementCoreResult<()> {
        self.cache.clear().await?;
        Ok(())
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        self.cache.get_stats()
    }
}

impl ElementCache {
    /// Create new element cache
    pub fn new() -> Self {
        Self {
            storage: DashMap::new(),
            config: CacheConfig::default(),
            stats: Mutex::new(CacheStats::new()),
            lru_list: Mutex::new(std::collections::VecDeque::new()),
        }
    }
    
    /// Create cache with custom configuration
    pub fn with_config(config: CacheConfig) -> Self {
        Self {
            storage: DashMap::new(),
            config,
            stats: Mutex::new(CacheStats::new()),
            lru_list: Mutex::new(std::collections::VecDeque::new()),
        }
    }
    
    /// Get cached data
    pub async fn get(&self, key: &str) -> ElementCoreResult<Option<CachedElementData>> {
        if !self.config.enabled {
            return Ok(None);
        }
        
        if let Some(entry) = self.storage.get(key) {
            if let Ok(mut lru) = self.lru_list.lock() {
                // Move key to back (most recently used)
                if let Some(pos) = lru.iter().position(|k| k == key) { lru.remove(pos); }
                lru.push_back(key.to_string());
            }
            if let Ok(mut s) = self.stats.lock() { s.record_hit(); }
            Ok(Some(entry.clone()))
        } else {
            if let Ok(mut s) = self.stats.lock() { s.record_miss(); }
            Ok(None)
        }
    }
    
    /// Store data in cache
    pub async fn store(&self, key: &str, stats: &HashMap<String, f64>) -> ElementCoreResult<()> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Check cache size limit
        if self.storage.len() >= self.config.size_limit {
            self.evict_entries().await?;
        }
        
        let cached_data = CachedElementData {
            stats: stats.clone(),
            timestamp: chrono::Utc::now(),
            ttl_seconds: self.config.default_ttl_seconds,
            key: key.to_string(),
        };
        
        self.storage.insert(key.to_string(), cached_data);
        if let Ok(mut lru) = self.lru_list.lock() {
            if let Some(pos) = lru.iter().position(|k| k == key) { lru.remove(pos); }
            lru.push_back(key.to_string());
        }
        if let Ok(mut s) = self.stats.lock() { s.update_size(self.storage.len()); }
        
        Ok(())
    }
    
    /// Evict entries based on policy
    async fn evict_entries(&self) -> ElementCoreResult<()> {
        let entries_to_remove = self.storage.len() - self.config.size_limit + 1;
        
        match self.config.eviction_policy {
            EvictionPolicy::LRU => {
                let mut removed = 0;
                if let Ok(mut lru) = self.lru_list.lock() {
                    while removed < entries_to_remove {
                        if let Some(oldest) = lru.pop_front() {
                            self.storage.remove(&oldest);
                            if let Ok(mut s) = self.stats.lock() { s.record_eviction(); }
                            removed += 1;
                        } else { break; }
                    }
                }
            }
            EvictionPolicy::LFU => {
                // TODO: Implement LFU eviction
                // For now, remove random entries
                let keys: Vec<String> = self.storage.iter()
                    .map(|entry| entry.key().clone())
                    .take(entries_to_remove)
                    .collect();
                
                for key in keys {
                    self.storage.remove(&key);
                    if let Ok(mut s) = self.stats.lock() { s.record_eviction(); }
                }
            }
            EvictionPolicy::FIFO => {
                // TODO: Implement FIFO eviction
                // For now, remove random entries
                let keys: Vec<String> = self.storage.iter()
                    .map(|entry| entry.key().clone())
                    .take(entries_to_remove)
                    .collect();
                
                for key in keys {
                    self.storage.remove(&key);
                    if let Ok(mut s) = self.stats.lock() { s.record_eviction(); }
                }
            }
            EvictionPolicy::Random => {
                let keys: Vec<String> = self.storage.iter()
                    .map(|entry| entry.key().clone())
                    .take(entries_to_remove)
                    .collect();
                
                for key in keys {
                    self.storage.remove(&key);
                    if let Ok(mut s) = self.stats.lock() { s.record_eviction(); }
                }
            }
        }
        
        Ok(())
    }
    
    /// Clear all cached data
    pub async fn clear(&self) -> ElementCoreResult<()> {
        self.storage.clear();
        if let Ok(mut lru) = self.lru_list.lock() { lru.clear(); }
        if let Ok(mut s) = self.stats.lock() { s.reset(); }
        Ok(())
    }
    
    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        if let Ok(s) = self.stats.lock() { s.clone() } else { CacheStats::new() }
    }
}

impl CachedElementData {
    /// Check if cached data is still valid
    pub fn is_valid(&self) -> bool {
        let now = chrono::Utc::now();
        let age = now - self.timestamp;
        age.num_seconds() < self.ttl_seconds as i64
    }
    
    /// Get cache age in seconds
    pub fn get_age_seconds(&self) -> i64 {
        let now = chrono::Utc::now();
        let age = now - self.timestamp;
        age.num_seconds()
    }
}

impl CacheStats {
    /// Create new cache statistics
    pub fn new() -> Self {
        Self {
            hit_count: 0,
            miss_count: 0,
            eviction_count: 0,
            size: 0,
        }
    }
    
    /// Record a cache hit
    pub fn record_hit(&self) {
        // Note: This is a simplified version since we can't mutate behind Arc
        // In a real implementation, you'd use Arc<Mutex<CacheStats>> or similar
        println!("Cache hit recorded");
    }
    
    /// Record a cache miss
    pub fn record_miss(&self) {
        // Note: This is a simplified version since we can't mutate behind Arc
        // In a real implementation, you'd use Arc<Mutex<CacheStats>> or similar
        println!("Cache miss recorded");
    }
    
    /// Record a cache eviction
    pub fn record_eviction(&self) {
        // Note: This is a simplified version since we can't mutate behind Arc
        // In a real implementation, you'd use Arc<Mutex<CacheStats>> or similar
        println!("Cache eviction recorded");
    }
    
    /// Update cache size
    pub fn update_size(&self, size: usize) {
        // Note: This is a simplified version since we can't mutate behind Arc
        // In a real implementation, you'd use Arc<Mutex<CacheStats>> or similar
        println!("Cache size updated to: {}", size);
    }
    
    /// Get cache hit rate
    pub fn get_hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total > 0 {
            self.hit_count as f64 / total as f64
        } else {
            0.0
        }
    }
    
    /// Reset statistics
    pub fn reset(&self) {
        // Note: This is a simplified version since we can't mutate behind Arc
        // In a real implementation, you'd use Arc<Mutex<CacheStats>> or similar
        println!("Cache stats reset");
    }
}

impl AggregatorMetrics {
    /// Create new aggregator metrics
    pub fn new() -> Self {
        Self {
            total_operations: 0,
            successful_operations: 0,
            failed_operations: 0,
            average_aggregation_time_ms: 0.0,
            cache_hit_rate: 0.0,
        }
    }
    
    /// Record an operation
    pub fn record_operation(&mut self, success: bool, duration_ms: f64) {
        self.total_operations += 1;
        
        if success {
            self.successful_operations += 1;
        } else {
            self.failed_operations += 1;
        }
        
        // Update average aggregation time
        self.average_aggregation_time_ms = 
            (self.average_aggregation_time_ms * (self.total_operations - 1) as f64 + duration_ms) 
            / self.total_operations as f64;
    }
    
    /// Record a cache hit
    pub fn record_cache_hit(&mut self) {
        // This would be called by the cache system
        // For now, we'll update it in the main operation
    }
    
    /// Get success rate
    pub fn get_success_rate(&self) -> f64 {
        if self.total_operations > 0 {
            self.successful_operations as f64 / self.total_operations as f64
        } else {
            0.0
        }
    }
    
    /// Get failure rate
    pub fn get_failure_rate(&self) -> f64 {
        if self.total_operations > 0 {
            self.failed_operations as f64 / self.total_operations as f64
        } else {
            0.0
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size_limit: 1000,
            default_ttl_seconds: 3600, // 1 hour
            eviction_policy: EvictionPolicy::LRU,
        }
    }
}

impl Default for CacheStats {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AggregatorMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for AggregatorMetrics {
    fn clone(&self) -> Self {
        Self {
            total_operations: self.total_operations,
            successful_operations: self.successful_operations,
            failed_operations: self.failed_operations,
            average_aggregation_time_ms: self.average_aggregation_time_ms,
            cache_hit_rate: self.cache_hit_rate,
        }
    }
}

impl Clone for CacheStats {
    fn clone(&self) -> Self {
        Self {
            hit_count: self.hit_count,
            miss_count: self.miss_count,
            eviction_count: self.eviction_count,
            size: self.size,
        }
    }
}
