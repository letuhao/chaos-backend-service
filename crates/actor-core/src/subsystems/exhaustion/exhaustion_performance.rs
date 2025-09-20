//! Performance Optimizations for Resource Exhaustion System
//!
//! This module provides performance optimizations including caching,
//! pre-computed thresholds, and fast-path evaluation.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use dashmap::DashMap;

use super::resource_exhaustion::{
    ExhaustionConfig, ThresholdConfig, ExhaustionTransition, ExhaustionEngine
};
use crate::types::{Actor, Snapshot};

/// Performance-optimized exhaustion engine
pub struct OptimizedExhaustionEngine {
    /// Base engine
    base_engine: ExhaustionEngine,
    /// Pre-computed threshold data for fast evaluation
    threshold_cache: Arc<DashMap<String, Vec<CachedThreshold>>>,
    /// Actor state cache to avoid redundant evaluations
    actor_state_cache: Arc<DashMap<String, CachedActorState>>,
    /// Performance statistics
    stats: Arc<RwLock<PerformanceStats>>,
}

/// Cached threshold data for fast evaluation
#[derive(Debug, Clone)]
struct CachedThreshold {
    /// Original threshold config
    #[allow(dead_code)]
    pub config: ThresholdConfig,
    /// Pre-computed enter condition checker
    #[allow(dead_code)]
    pub enter_checker: ThresholdChecker,
    /// Pre-computed exit condition checker
    #[allow(dead_code)]
    pub exit_checker: ThresholdChecker,
    /// Resource name this threshold applies to
    #[allow(dead_code)]
    pub resource_name: String,
}

/// Threshold condition checker
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum ThresholdChecker {
    /// Percentage-based checker
    PercentLte(f64),
    /// Value-based checker
    ValueEq(f64),
    /// Value-based checker (greater than or equal)
    ValueGe(f64),
}

/// Cached actor state
#[derive(Debug, Clone)]
struct CachedActorState {
    /// Last known resource values
    pub resource_values: HashMap<String, f64>,
    /// Last known resource max values
    pub resource_max_values: HashMap<String, f64>,
    /// Last evaluation timestamp
    #[allow(dead_code)]
    pub last_evaluation: Instant,
    /// Last known active thresholds
    #[allow(dead_code)]
    pub active_thresholds: HashMap<String, bool>,
}

/// Performance statistics
#[derive(Debug, Default, Clone)]
pub struct PerformanceStats {
    /// Total evaluations performed
    pub total_evaluations: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,
    /// Average evaluation time in microseconds
    pub avg_evaluation_time_us: f64,
    /// Total evaluation time
    pub total_evaluation_time: Duration,
    /// Fast path evaluations (no changes detected)
    pub fast_path_evaluations: u64,
}

impl OptimizedExhaustionEngine {
    /// Create a new optimized exhaustion engine
    pub fn new(config: Arc<RwLock<ExhaustionConfig>>) -> Self {
        let base_engine = ExhaustionEngine::new(config.clone());
        let mut optimized = Self {
            base_engine,
            threshold_cache: Arc::new(DashMap::new()),
            actor_state_cache: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(PerformanceStats::default())),
        };
        
        // Pre-compute threshold cache
        optimized.build_threshold_cache();
        
        optimized
    }

    /// Build threshold cache for fast evaluation
    fn build_threshold_cache(&mut self) {
        // This would be called when configuration changes
        // TODO: Implement actual threshold cache building instead of placeholder
        // In a real implementation, this would parse the config and pre-compute checkers
    }

    /// Evaluate exhaustion with performance optimizations
    pub async fn evaluate_optimized(&self, actor: &Actor, snapshot: &Snapshot) -> Result<Vec<ExhaustionTransition>, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        
        // Get actor archetype (for future use)
        // TODO: Load default archetype from configuration instead of hardcoding "default"
        let _archetype = actor.data.get("archetype")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        // Check if we can use fast path (no changes detected)
        if let Some(cached_state) = self.actor_state_cache.get(&actor.id.to_string()) {
            if self.can_use_fast_path(&cached_state, snapshot) {
                self.update_stats_fast_path().await;
                return Ok(Vec::new());
            }
        }

        // Perform full evaluation
        let transitions = self.base_engine.evaluate(actor, snapshot).await?;
        
        // Update cache
        self.update_actor_cache(actor, snapshot).await;
        
        // Update statistics
        let evaluation_time = start_time.elapsed();
        self.update_stats(evaluation_time, false).await;
        
        Ok(transitions)
    }

    /// Check if we can use fast path (no resource changes)
    fn can_use_fast_path(&self, cached_state: &CachedActorState, snapshot: &Snapshot) -> bool {
        // Check if resource values have changed significantly
        for (resource_name, current_value) in &snapshot.primary {
            if resource_name.ends_with("_current") {
                if let Some(base_name) = resource_name.strip_suffix("_current") {
                    if let Some(cached_value) = cached_state.resource_values.get(base_name) {
                        if (current_value - cached_value).abs() > f64::EPSILON {
                            return false;
                        }
                    }
                }
            }
        }
        
        // Check if max values have changed
        for (resource_name, max_value) in &snapshot.primary {
            if resource_name.ends_with("_max") {
                if let Some(base_name) = resource_name.strip_suffix("_max") {
                    if let Some(cached_max) = cached_state.resource_max_values.get(base_name) {
                        if (max_value - cached_max).abs() > f64::EPSILON {
                            return false;
                        }
                    }
                }
            }
        }
        
        true
    }

    /// Update actor cache
    async fn update_actor_cache(&self, actor: &Actor, snapshot: &Snapshot) {
        let mut resource_values = HashMap::new();
        let mut resource_max_values = HashMap::new();
        
        // Extract resource values
        for (key, value) in &snapshot.primary {
            if key.ends_with("_current") {
                if let Some(base_name) = key.strip_suffix("_current") {
                    resource_values.insert(base_name.to_string(), *value);
                }
            } else if key.ends_with("_max") {
                if let Some(base_name) = key.strip_suffix("_max") {
                    resource_max_values.insert(base_name.to_string(), *value);
                }
            }
        }
        
        let cached_state = CachedActorState {
            resource_values,
            resource_max_values,
            last_evaluation: Instant::now(),
            active_thresholds: HashMap::new(),
        };
        
        self.actor_state_cache.insert(actor.id.to_string(), cached_state);
    }

    /// Update performance statistics
    async fn update_stats(&self, evaluation_time: Duration, fast_path: bool) {
        let mut stats = self.stats.write().await;
        stats.total_evaluations += 1;
        stats.total_evaluation_time += evaluation_time;
        
        if fast_path {
            stats.fast_path_evaluations += 1;
        } else {
            stats.cache_misses += 1;
        }
        
        // Update average evaluation time
        stats.avg_evaluation_time_us = stats.total_evaluation_time.as_micros() as f64 / stats.total_evaluations as f64;
    }

    /// Update statistics for fast path
    async fn update_stats_fast_path(&self) {
        let mut stats = self.stats.write().await;
        stats.total_evaluations += 1;
        stats.cache_hits += 1;
        stats.fast_path_evaluations += 1;
    }

    /// Get performance statistics
    pub async fn get_stats(&self) -> PerformanceStats {
        let stats = self.stats.read().await;
        PerformanceStats {
            total_evaluations: stats.total_evaluations,
            cache_hits: stats.cache_hits,
            cache_misses: stats.cache_misses,
            avg_evaluation_time_us: stats.avg_evaluation_time_us,
            total_evaluation_time: stats.total_evaluation_time,
            fast_path_evaluations: stats.fast_path_evaluations,
        }
    }

    /// Clear caches
    pub fn clear_caches(&self) {
        self.actor_state_cache.clear();
        self.threshold_cache.clear();
    }

    /// Reset performance statistics
    pub async fn reset_stats(&self) {
        let mut stats = self.stats.write().await;
        *stats = PerformanceStats::default();
    }
}

/// Performance monitoring utilities
pub struct PerformanceMonitor {
    /// Evaluation time thresholds
    pub warning_threshold_us: u64,
    pub error_threshold_us: u64,
    /// Statistics
    stats: Arc<RwLock<PerformanceStats>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(warning_threshold_us: u64, error_threshold_us: u64) -> Self {
        Self {
            warning_threshold_us,
            error_threshold_us,
            stats: Arc::new(RwLock::new(PerformanceStats::default())),
        }
    }

    /// Record an evaluation
    pub async fn record_evaluation(&self, duration: Duration, fast_path: bool) {
        let mut stats = self.stats.write().await;
        stats.total_evaluations += 1;
        stats.total_evaluation_time += duration;
        
        if fast_path {
            stats.fast_path_evaluations += 1;
        }
        
        // Check for performance issues
        let duration_us = duration.as_micros() as u64;
        if duration_us > self.error_threshold_us {
            tracing::error!(
                "Exhaustion evaluation exceeded error threshold: {}μs (threshold: {}μs)",
                duration_us,
                self.error_threshold_us
            );
        } else if duration_us > self.warning_threshold_us {
            tracing::warn!(
                "Exhaustion evaluation exceeded warning threshold: {}μs (threshold: {}μs)",
                duration_us,
                self.warning_threshold_us
            );
        }
    }

    /// Get performance statistics
    pub async fn get_stats(&self) -> PerformanceStats {
        let stats = self.stats.read().await;
        PerformanceStats {
            total_evaluations: stats.total_evaluations,
            cache_hits: stats.cache_hits,
            cache_misses: stats.cache_misses,
            avg_evaluation_time_us: stats.avg_evaluation_time_us,
            total_evaluation_time: stats.total_evaluation_time,
            fast_path_evaluations: stats.fast_path_evaluations,
        }
    }
}

/// Benchmark utilities for exhaustion system
pub struct ExhaustionBenchmark {
    /// Test configurations
    test_configs: Vec<BenchmarkConfig>,
    /// Results
    results: Vec<BenchmarkResult>,
}

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Configuration name
    pub name: String,
    /// Number of actors to test
    pub actor_count: usize,
    /// Number of evaluations per actor
    pub evaluations_per_actor: usize,
    /// Resource value ranges
    pub resource_ranges: HashMap<String, (f64, f64)>,
}

/// Benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Configuration name
    pub config_name: String,
    /// Total time
    pub total_time: Duration,
    /// Average time per evaluation
    pub avg_time_per_evaluation: Duration,
    /// Evaluations per second
    pub evaluations_per_second: f64,
    /// Memory usage (if available)
    pub memory_usage_bytes: Option<usize>,
}

impl ExhaustionBenchmark {
    /// Create a new benchmark
    pub fn new() -> Self {
        Self {
            test_configs: Vec::new(),
            results: Vec::new(),
        }
    }

    /// Add a test configuration
    pub fn add_config(&mut self, config: BenchmarkConfig) {
        self.test_configs.push(config);
    }

    /// Run all benchmarks
    pub async fn run_all(&mut self, engine: &OptimizedExhaustionEngine) -> Result<(), Box<dyn std::error::Error>> {
        for config in &self.test_configs.clone() {
            let result = self.run_single_benchmark(engine, config).await?;
            self.results.push(result);
        }
        Ok(())
    }

    /// Run a single benchmark
    async fn run_single_benchmark(&self, engine: &OptimizedExhaustionEngine, config: &BenchmarkConfig) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        
        // Create test actors
        let actors = self.create_test_actors(config.actor_count);
        
        // Run evaluations
        for _ in 0..config.evaluations_per_actor {
            for actor in &actors {
                let snapshot = self.create_test_snapshot(&config.resource_ranges);
                let _transitions = engine.evaluate_optimized(actor, &snapshot).await?;
            }
        }
        
        let total_time = start_time.elapsed();
        let total_evaluations = config.actor_count * config.evaluations_per_actor;
        let avg_time_per_evaluation = total_time / total_evaluations as u32;
        let evaluations_per_second = total_evaluations as f64 / total_time.as_secs_f64();
        
        Ok(BenchmarkResult {
            config_name: config.name.clone(),
            total_time,
            avg_time_per_evaluation,
            evaluations_per_second,
            memory_usage_bytes: None, // Would need memory profiling tools
        })
    }

    /// Create test actors
    fn create_test_actors(&self, count: usize) -> Vec<Actor> {
        let mut actors = Vec::new();
        for i in 0..count {
            let mut actor = Actor::new(format!("benchmark_actor_{}", i), "Human".to_string());
            let mut data = HashMap::new();
            data.insert("archetype".to_string(), serde_json::Value::String("mage".to_string()));
            actor.set_data(data);
            actors.push(actor);
        }
        actors
    }

    /// Create test snapshot
    fn create_test_snapshot(&self, resource_ranges: &HashMap<String, (f64, f64)>) -> Snapshot {
        let mut snapshot = Snapshot::new(uuid::Uuid::new_v4(), 1);
        
        for (resource_name, (min, max)) in resource_ranges {
            let current = min + (max - min) * 0.5; // Use middle value
            snapshot.primary.insert(format!("{}_current", resource_name), current);
            snapshot.primary.insert(format!("{}_max", resource_name), *max);
        }
        
        snapshot
    }

    /// Get benchmark results
    pub fn get_results(&self) -> &[BenchmarkResult] {
        &self.results
    }
}