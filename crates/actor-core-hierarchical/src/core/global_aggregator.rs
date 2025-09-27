//! # Global Aggregator
//! 
//! Global stats aggregation system for combining contributions from all game systems.

use crate::core::{HierarchicalActor, SystemContribution};
use std::collections::HashMap;
use chrono::Utc;

/// Global aggregator for combining stats from all systems
#[derive(Debug, Clone)]
pub struct GlobalAggregator {
    /// Aggregation strategies for different stat types
    pub aggregation_strategies: HashMap<String, AggregationStrategy>,
    
    /// Cache for aggregated results
    pub aggregation_cache: HashMap<String, HashMap<String, f64>>,
    
    /// Last cache update timestamp
    last_cache_update: chrono::DateTime<Utc>,
}

/// Aggregation strategy for different stat types
#[derive(Debug, Clone)]
pub enum AggregationStrategy {
    /// Sum all contributions
    Sum,
    /// Take maximum value
    Max,
    /// Take minimum value
    Min,
    /// Calculate average
    Average,
    /// Multiply all contributions
    Multiply,
    /// Custom aggregation function
    Custom(fn(&[f64]) -> f64),
}

impl Default for GlobalAggregator {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalAggregator {
    /// Create a new global aggregator
    pub fn new() -> Self {
        let mut aggregator = Self {
            aggregation_strategies: HashMap::new(),
            aggregation_cache: HashMap::new(),
            last_cache_update: Utc::now(),
        };
        
        // Set default aggregation strategies
        aggregator.set_default_strategies();
        aggregator
    }
    
    /// Set default aggregation strategies
    fn set_default_strategies(&mut self) {
        // Health, mana, and similar stats should be summed
        self.aggregation_strategies.insert("health".to_string(), AggregationStrategy::Sum);
        self.aggregation_strategies.insert("mana".to_string(), AggregationStrategy::Sum);
        self.aggregation_strategies.insert("stamina".to_string(), AggregationStrategy::Sum);
        self.aggregation_strategies.insert("experience".to_string(), AggregationStrategy::Sum);
        
        // Attack, defense stats should be summed
        self.aggregation_strategies.insert("attack".to_string(), AggregationStrategy::Sum);
        self.aggregation_strategies.insert("defense".to_string(), AggregationStrategy::Sum);
        self.aggregation_strategies.insert("physical_attack".to_string(), AggregationStrategy::Sum);
        self.aggregation_strategies.insert("magical_attack".to_string(), AggregationStrategy::Sum);
        self.aggregation_strategies.insert("physical_defense".to_string(), AggregationStrategy::Sum);
        self.aggregation_strategies.insert("magical_defense".to_string(), AggregationStrategy::Sum);
        
        // Critical rate should take maximum
        self.aggregation_strategies.insert("critical_rate".to_string(), AggregationStrategy::Max);
        self.aggregation_strategies.insert("critical_damage".to_string(), AggregationStrategy::Max);
        
        // Speed should take maximum
        self.aggregation_strategies.insert("speed".to_string(), AggregationStrategy::Max);
        self.aggregation_strategies.insert("movement_speed".to_string(), AggregationStrategy::Max);
        self.aggregation_strategies.insert("attack_speed".to_string(), AggregationStrategy::Max);
        
        // Accuracy should take maximum
        self.aggregation_strategies.insert("accuracy".to_string(), AggregationStrategy::Max);
        self.aggregation_strategies.insert("dodge_rate".to_string(), AggregationStrategy::Max);
        
        // Level should take maximum
        self.aggregation_strategies.insert("level".to_string(), AggregationStrategy::Max);
    }
    
    /// Set aggregation strategy for a stat
    pub fn set_aggregation_strategy(&mut self, stat_name: String, strategy: AggregationStrategy) {
        self.aggregation_strategies.insert(stat_name, strategy);
    }
    
    /// Get aggregation strategy for a stat
    pub fn get_aggregation_strategy(&self, stat_name: &str) -> AggregationStrategy {
        self.aggregation_strategies
            .get(stat_name)
            .cloned()
            .unwrap_or(AggregationStrategy::Sum) // Default to Sum
    }
    
    /// Aggregate all system contributions for an actor
    pub fn aggregate_actor_stats(&mut self, actor: &HierarchicalActor) -> HashMap<String, f64> {
        let actor_id = actor.get_id();
        
        // Check cache first
        if let Some(cached_stats) = self.aggregation_cache.get(actor_id) {
            if self.is_cache_valid(actor) {
                return cached_stats.clone();
            }
        }
        
        // Aggregate stats from all systems
        let mut aggregated_stats = HashMap::new();
        
        // Collect all contributions by stat name
        let mut stat_contributions: HashMap<String, Vec<f64>> = HashMap::new();
        
        for (system_name, contributions) in &actor.system_contributions {
            for contribution in contributions {
                stat_contributions
                    .entry(contribution.stat_name.clone())
                    .or_insert_with(Vec::new)
                    .push(contribution.value);
            }
        }
        
        // Apply aggregation strategy to each stat
        for (stat_name, contributions) in stat_contributions {
            let strategy = self.get_aggregation_strategy(&stat_name);
            let aggregated_value = self.apply_aggregation_strategy(&strategy, &contributions);
            aggregated_stats.insert(stat_name, aggregated_value);
        }
        
        // Update cache
        self.aggregation_cache.insert(actor_id.to_string(), aggregated_stats.clone());
        self.last_cache_update = Utc::now();
        
        aggregated_stats
    }
    
    /// Apply aggregation strategy to contributions
    pub fn apply_aggregation_strategy(&self, strategy: &AggregationStrategy, contributions: &[f64]) -> f64 {
        if contributions.is_empty() {
            return 0.0;
        }
        
        match strategy {
            AggregationStrategy::Sum => contributions.iter().sum(),
            AggregationStrategy::Max => contributions.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            AggregationStrategy::Min => contributions.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
            AggregationStrategy::Average => contributions.iter().sum::<f64>() / contributions.len() as f64,
            AggregationStrategy::Multiply => contributions.iter().product(),
            AggregationStrategy::Custom(func) => func(contributions),
        }
    }
    
    /// Check if cache is valid
    fn is_cache_valid(&self, actor: &HierarchicalActor) -> bool {
        // Cache is valid if it was updated after the actor's last update
        self.last_cache_update > actor.get_updated_at()
    }
    
    /// Invalidate cache for an actor
    pub fn invalidate_actor_cache(&mut self, actor_id: &str) {
        self.aggregation_cache.remove(actor_id);
    }
    
    /// Clear all cache
    pub fn clear_cache(&mut self) {
        self.aggregation_cache.clear();
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        CacheStats {
            cached_actors: self.aggregation_cache.len(),
            last_update: self.last_cache_update,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub cached_actors: usize,
    pub last_update: chrono::DateTime<Utc>,
}

