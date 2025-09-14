//! Observability module for standardized tracing, metrics, SLOs, and monitoring.
//!
//! This module provides comprehensive observability capabilities including
//! structured logging, metrics collection, Service Level Objectives (SLOs),
//! and dashboard monitoring for the Actor Core system.

pub mod slos;
pub mod metrics_collector;
pub mod dashboard;

// Re-export key observability components
pub use slos::{SLOManager, SLO, SLOStatus, SLOMetricType, SLOSeverity, SLOViolation, SLOViolationHandler, ConsoleSLOViolationHandler, default_slos};
pub use metrics_collector::{MetricsCollector, MetricsSnapshot, MetricValue, MetricType, default_metrics};
pub use dashboard::{ObservabilityDashboard, DashboardConfig, DashboardStatus, SystemHealthStatus, DashboardBuilder};

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Standardized tracing fields for consistent logging across components.
pub mod tracing_fields {
    /// Actor ID field name
    pub const ACTOR_ID: &str = "actor_id";
    /// System ID field name
    pub const SYSTEM_ID: &str = "system_id";
    /// Dimension field name
    pub const DIMENSION: &str = "dimension";
    /// Operation field name
    pub const OPERATION: &str = "operation";
    /// Duration field name (in microseconds)
    pub const DURATION_US: &str = "duration_us";
    /// Cache hit/miss field name
    pub const CACHE_HIT: &str = "cache_hit";
    /// Cache layer field name
    pub const CACHE_LAYER: &str = "cache_layer";
    /// Error field name
    pub const ERROR: &str = "error";
    /// Memory usage field name (in bytes)
    pub const MEMORY_USAGE_BYTES: &str = "memory_usage_bytes";
    /// Queue size field name
    pub const QUEUE_SIZE: &str = "queue_size";
    /// Processing time field name (in microseconds)
    pub const PROCESSING_TIME_US: &str = "processing_time_us";
    /// Batch size field name
    pub const BATCH_SIZE: &str = "batch_size";
    /// Version field name
    pub const VERSION: &str = "version";
    /// Priority field name
    pub const PRIORITY: &str = "priority";
    /// Subsystem count field name
    pub const SUBSYSTEM_COUNT: &str = "subsystem_count";
    /// Contribution count field name
    pub const CONTRIBUTION_COUNT: &str = "contribution_count";
}

/// Standardized metrics for consistent performance monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardMetrics {
    /// Operation counters
    pub operations: HashMap<String, u64>,
    /// Timing measurements (in microseconds)
    pub timings: HashMap<String, u64>,
    /// Memory usage (in bytes)
    pub memory_usage: u64,
    /// Error counts by type
    pub errors: HashMap<String, u64>,
    /// Cache statistics
    pub cache_stats: CacheMetrics,
    /// Subsystem statistics
    pub subsystem_stats: SubsystemMetrics,
    /// Last updated timestamp
    pub last_updated: u64,
}

/// Cache-specific metrics following a consistent structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    /// Cache hits
    pub hits: u64,
    /// Cache misses
    pub misses: u64,
    /// Cache sets
    pub sets: u64,
    /// Cache deletes
    pub deletes: u64,
    /// Cache clears
    pub clears: u64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Hit rate (0.0 to 1.0)
    pub hit_rate: f64,
    /// Average access time in microseconds
    pub avg_access_time_us: u64,
}

/// Subsystem-specific metrics following a consistent structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemMetrics {
    /// Total subsystem executions
    pub total_executions: u64,
    /// Successful executions
    pub successful_executions: u64,
    /// Failed executions
    pub failed_executions: u64,
    /// Average execution time in microseconds
    pub avg_execution_time_us: u64,
    /// Maximum execution time in microseconds
    pub max_execution_time_us: u64,
    /// Active subsystems
    pub active_subsystems: usize,
}

impl Default for StandardMetrics {
    fn default() -> Self {
        Self {
            operations: HashMap::new(),
            timings: HashMap::new(),
            memory_usage: 0,
            errors: HashMap::new(),
            cache_stats: CacheMetrics::default(),
            subsystem_stats: SubsystemMetrics::default(),
            last_updated: current_timestamp_ms(),
        }
    }
}

impl Default for CacheMetrics {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            sets: 0,
            deletes: 0,
            clears: 0,
            memory_usage: 0,
            hit_rate: 0.0,
            avg_access_time_us: 0,
        }
    }
}

impl Default for SubsystemMetrics {
    fn default() -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            avg_execution_time_us: 0,
            max_execution_time_us: 0,
            active_subsystems: 0,
        }
    }
}

impl StandardMetrics {
    /// Record an operation with timing.
    pub fn record_operation(&mut self, operation: &str, duration_us: u64) {
        *self.operations.entry(operation.to_string()).or_insert(0) += 1;
        *self.timings.entry(format!("{}_duration_us", operation)).or_insert(0) = duration_us;
        self.last_updated = current_timestamp_ms();
    }

    /// Record a cache operation.
    pub fn record_cache_operation(&mut self, operation: &str, hit: bool, duration_us: u64) {
        match operation {
            "get" => {
                if hit {
                    self.cache_stats.hits += 1;
                } else {
                    self.cache_stats.misses += 1;
                }
            }
            "set" => self.cache_stats.sets += 1,
            "delete" => self.cache_stats.deletes += 1,
            "clear" => self.cache_stats.clears += 1,
            _ => {}
        }
        
        // Update hit rate
        let total_operations = self.cache_stats.hits + self.cache_stats.misses;
        if total_operations > 0 {
            self.cache_stats.hit_rate = self.cache_stats.hits as f64 / total_operations as f64;
        }
        
        // Update average access time
        self.cache_stats.avg_access_time_us = duration_us;
        
        self.record_operation(&format!("cache_{}", operation), duration_us);
    }

    /// Record a subsystem execution.
    pub fn record_subsystem_execution(&mut self, success: bool, duration_us: u64) {
        self.subsystem_stats.total_executions += 1;
        if success {
            self.subsystem_stats.successful_executions += 1;
        } else {
            self.subsystem_stats.failed_executions += 1;
        }
        
        // Update timing statistics
        self.subsystem_stats.avg_execution_time_us = duration_us;
        if duration_us > self.subsystem_stats.max_execution_time_us {
            self.subsystem_stats.max_execution_time_us = duration_us;
        }
        
        self.record_operation("subsystem_execution", duration_us);
    }

    /// Record an error.
    pub fn record_error(&mut self, error_type: &str) {
        *self.errors.entry(error_type.to_string()).or_insert(0) += 1;
        self.last_updated = current_timestamp_ms();
    }

    /// Update memory usage.
    pub fn update_memory_usage(&mut self, memory_bytes: u64) {
        self.memory_usage = memory_bytes;
        self.cache_stats.memory_usage = memory_bytes;
        self.last_updated = current_timestamp_ms();
    }

    /// Get the total number of operations.
    pub fn total_operations(&self) -> u64 {
        self.operations.values().sum()
    }

    /// Get the total number of errors.
    pub fn total_errors(&self) -> u64 {
        self.errors.values().sum()
    }

    /// Get error rate (0.0 to 1.0).
    pub fn error_rate(&self) -> f64 {
        let total_ops = self.total_operations();
        if total_ops == 0 {
            0.0
        } else {
            self.total_errors() as f64 / total_ops as f64
        }
    }
}

/// Centralized observability manager for actor-core components.
pub struct ObservabilityManager {
    /// Global metrics
    metrics: Arc<std::sync::RwLock<StandardMetrics>>,
    /// Component-specific metrics
    component_metrics: Arc<std::sync::RwLock<HashMap<String, StandardMetrics>>>,
    /// Configuration
    config: ObservabilityConfig,
}

/// Configuration for observability.
#[derive(Debug, Clone)]
pub struct ObservabilityConfig {
    /// Enable detailed tracing
    pub enable_detailed_tracing: bool,
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Enable performance monitoring
    pub enable_performance_monitoring: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Maximum number of component metrics to keep
    pub max_component_metrics: usize,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            enable_detailed_tracing: true,
            enable_metrics: true,
            enable_performance_monitoring: true,
            metrics_interval: Duration::from_secs(5),
            max_component_metrics: 100,
        }
    }
}

impl ObservabilityManager {
    /// Create a new observability manager.
    pub fn new(config: ObservabilityConfig) -> Self {
        Self {
            metrics: Arc::new(std::sync::RwLock::new(StandardMetrics::default())),
            component_metrics: Arc::new(std::sync::RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Get global metrics.
    pub fn get_global_metrics(&self) -> StandardMetrics {
        self.metrics.read().unwrap().clone()
    }

    /// Get component metrics.
    pub fn get_component_metrics(&self, component: &str) -> Option<StandardMetrics> {
        self.component_metrics.read().unwrap().get(component).cloned()
    }

    /// Record an operation for a specific component.
    pub fn record_component_operation(&self, component: &str, operation: &str, duration_us: u64) {
        if !self.config.enable_metrics {
            return;
        }

        let mut component_metrics = self.component_metrics.write().unwrap();
        let metrics = component_metrics.entry(component.to_string()).or_insert_with(StandardMetrics::default);
        metrics.record_operation(operation, duration_us);
        
        // Also update global metrics
        let mut global_metrics = self.metrics.write().unwrap();
        global_metrics.record_operation(&format!("{}:{}", component, operation), duration_us);
    }

    /// Record a cache operation for a specific component.
    pub fn record_component_cache_operation(&self, component: &str, operation: &str, hit: bool, duration_us: u64) {
        if !self.config.enable_metrics {
            return;
        }

        let mut component_metrics = self.component_metrics.write().unwrap();
        let metrics = component_metrics.entry(component.to_string()).or_insert_with(StandardMetrics::default);
        metrics.record_cache_operation(operation, hit, duration_us);
        
        // Also update global metrics
        let mut global_metrics = self.metrics.write().unwrap();
        global_metrics.record_cache_operation(operation, hit, duration_us);
    }

    /// Record a subsystem execution.
    pub fn record_subsystem_execution(&self, subsystem_id: &str, success: bool, duration_us: u64) {
        if !self.config.enable_metrics {
            return;
        }

        let mut component_metrics = self.component_metrics.write().unwrap();
        let metrics = component_metrics.entry(subsystem_id.to_string()).or_insert_with(StandardMetrics::default);
        metrics.record_subsystem_execution(success, duration_us);
        
        // Also update global metrics
        let mut global_metrics = self.metrics.write().unwrap();
        global_metrics.record_subsystem_execution(success, duration_us);
    }

    /// Record an error for a specific component.
    pub fn record_component_error(&self, component: &str, error_type: &str) {
        if !self.config.enable_metrics {
            return;
        }

        let mut component_metrics = self.component_metrics.write().unwrap();
        let metrics = component_metrics.entry(component.to_string()).or_insert_with(StandardMetrics::default);
        metrics.record_error(error_type);
        
        // Also update global metrics
        let mut global_metrics = self.metrics.write().unwrap();
        global_metrics.record_error(&format!("{}:{}", component, error_type));
    }
}

/// Utility function to get current timestamp in milliseconds.
fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Macro for standardized structured logging with consistent fields.
#[macro_export]
macro_rules! trace_operation {
    ($level:expr, $msg:expr, $($field:ident = $value:expr),* $(,)?) => {
        tracing::event!($level, $($field = $value,)* $msg);
    };
}

/// Macro for recording operations with timing.
#[macro_export]
macro_rules! trace_with_timing {
    ($manager:expr, $component:expr, $operation:expr, $duration_us:expr, $($field:ident = $value:expr),* $(,)?) => {
        $manager.record_component_operation($component, $operation, $duration_us);
        $crate::trace_operation!(tracing::Level::DEBUG, 
            concat!($operation, " completed"), 
            component = $component,
            operation = $operation,
            duration_us = $duration_us,
            $($field = $value,)*
        );
    };
}

/// Macro for recording cache operations with timing.
#[macro_export]
macro_rules! trace_cache_operation {
    ($manager:expr, $component:expr, $operation:expr, $hit:expr, $duration_us:expr, $($field:ident = $value:expr),* $(,)?) => {
        $manager.record_component_cache_operation($component, $operation, $hit, $duration_us);
        $crate::trace_operation!(tracing::Level::DEBUG, 
            concat!("cache ", $operation, " completed"), 
            component = $component,
            operation = $operation,
            cache_hit = $hit,
            duration_us = $duration_us,
            $($field = $value,)*
        );
    };
}

/// Macro for recording errors with context.
#[macro_export]
macro_rules! trace_error {
    ($manager:expr, $component:expr, $error_type:expr, $error:expr, $($field:ident = $value:expr),* $(,)?) => {
        $manager.record_component_error($component, $error_type);
        tracing::error!(
            component = $component,
            error_type = $error_type,
            error = %$error,
            $($field = $value,)*
            concat!($error_type, " error occurred")
        );
    };
}