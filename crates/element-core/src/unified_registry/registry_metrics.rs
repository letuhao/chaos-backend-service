//! # Registry Metrics
//! 
//! This module defines the metrics collection system for the unified element registry.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Registry metrics for performance monitoring
/// 
/// This struct collects various metrics about the registry's performance,
/// including access times, hit rates, and error counts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryMetrics {
    /// Overall registry metrics
    pub overall: OverallMetrics,
    
    /// Element-specific metrics
    pub element_metrics: HashMap<String, ElementMetrics>,
    
    /// Contributor-specific metrics
    pub contributor_metrics: HashMap<String, ContributorMetrics>,
    
    /// Plugin-specific metrics
    pub plugin_metrics: HashMap<String, PluginMetrics>,
    
    /// Cache metrics
    pub cache_metrics: CacheMetrics,
    
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    
    /// Error metrics
    pub error_metrics: ErrorMetrics,
    
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

/// Overall registry metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallMetrics {
    /// Total number of elements
    pub total_elements: usize,
    
    /// Total number of contributors
    pub total_contributors: usize,
    
    /// Total number of plugins
    pub total_plugins: usize,
    
    /// Total number of interactions
    pub total_interactions: usize,
    
    /// Registry uptime in seconds
    pub uptime_seconds: u64,
    
    /// Total operations performed
    pub total_operations: u64,
    
    /// Successful operations
    pub successful_operations: u64,
    
    /// Failed operations
    pub failed_operations: u64,
}

/// Element-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementMetrics {
    /// Element ID
    pub element_id: String,
    
    /// Number of accesses
    pub access_count: u64,
    
    /// Average access time in milliseconds
    pub average_access_time_ms: f64,
    
    /// Last access timestamp
    pub last_access: DateTime<Utc>,
    
    /// Cache hit rate
    pub cache_hit_rate: f64,
    
    /// Error count
    pub error_count: u64,
    
    /// Success rate
    pub success_rate: f64,
}

/// Contributor-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributorMetrics {
    /// Contributor system ID
    pub system_id: String,
    
    /// Number of contributions
    pub contribution_count: u64,
    
    /// Average contribution time in milliseconds
    pub average_contribution_time_ms: f64,
    
    /// Last contribution timestamp
    pub last_contribution: DateTime<Utc>,
    
    /// Error count
    pub error_count: u64,
    
    /// Success rate
    pub success_rate: f64,
    
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
}

/// Plugin-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetrics {
    /// Plugin ID
    pub plugin_id: String,
    
    /// Number of operations
    pub operation_count: u64,
    
    /// Average operation time in milliseconds
    pub average_operation_time_ms: f64,
    
    /// Last operation timestamp
    pub last_operation: DateTime<Utc>,
    
    /// Error count
    pub error_count: u64,
    
    /// Success rate
    pub success_rate: f64,
    
    /// Plugin health score
    pub health_score: f64,
}

/// Cache metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    /// Total cache size
    pub total_size: usize,
    
    /// Cache hit count
    pub hit_count: u64,
    
    /// Cache miss count
    pub miss_count: u64,
    
    /// Cache hit rate
    pub hit_rate: f64,
    
    /// Cache eviction count
    pub eviction_count: u64,
    
    /// Average cache access time in milliseconds
    pub average_access_time_ms: f64,
    
    /// Cache memory usage in bytes
    pub memory_usage_bytes: usize,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
    
    /// 95th percentile response time in milliseconds
    pub p95_response_time_ms: f64,
    
    /// 99th percentile response time in milliseconds
    pub p99_response_time_ms: f64,
    
    /// Maximum response time in milliseconds
    pub max_response_time_ms: f64,
    
    /// Minimum response time in milliseconds
    pub min_response_time_ms: f64,
    
    /// Operations per second
    pub operations_per_second: f64,
    
    /// Peak operations per second
    pub peak_operations_per_second: f64,
    
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
    
    /// CPU usage percentage
    pub cpu_usage_percentage: f64,
}

/// Error metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    /// Total error count
    pub total_errors: u64,
    
    /// Error rate
    pub error_rate: f64,
    
    /// Error types
    pub error_types: HashMap<String, u64>,
    
    /// Recent errors
    pub recent_errors: Vec<ErrorRecord>,
    
    /// Error recovery count
    pub recovery_count: u64,
    
    /// Average recovery time in milliseconds
    pub average_recovery_time_ms: f64,
}

/// Error record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRecord {
    /// Error timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Error type
    pub error_type: String,
    
    /// Error message
    pub message: String,
    
    /// Error context
    pub context: HashMap<String, String>,
    
    /// Recovery time in milliseconds
    pub recovery_time_ms: Option<f64>,
}

impl RegistryMetrics {
    /// Create new registry metrics
    pub fn new() -> Self {
        Self {
            overall: OverallMetrics::new(),
            element_metrics: HashMap::new(),
            contributor_metrics: HashMap::new(),
            plugin_metrics: HashMap::new(),
            cache_metrics: CacheMetrics::new(),
            performance_metrics: PerformanceMetrics::new(),
            error_metrics: ErrorMetrics::new(),
            last_updated: Utc::now(),
        }
    }
    
    /// Update element metrics
    pub fn update_element_metrics(&mut self, element_id: String, access_time_ms: f64, success: bool) {
        let metrics = self.element_metrics.entry(element_id.clone()).or_insert_with(|| {
            ElementMetrics::new(element_id)
        });
        
        metrics.update(access_time_ms, success);
        self.overall.update_operation(success);
        self.last_updated = Utc::now();
    }
    
    /// Update contributor metrics
    pub fn update_contributor_metrics(&mut self, system_id: String, response_time_ms: f64, success: bool) {
        let metrics = self.contributor_metrics.entry(system_id.clone()).or_insert_with(|| {
            ContributorMetrics::new(system_id)
        });
        
        metrics.update(response_time_ms, success);
        self.last_updated = Utc::now();
    }
    
    /// Update plugin metrics
    pub fn update_plugin_metrics(&mut self, plugin_id: String, operation_time_ms: f64, success: bool) {
        let metrics = self.plugin_metrics.entry(plugin_id.clone()).or_insert_with(|| {
            PluginMetrics::new(plugin_id)
        });
        
        metrics.update(operation_time_ms, success);
        self.last_updated = Utc::now();
    }
    
    /// Update cache metrics
    pub fn update_cache_metrics(&mut self, hit: bool, access_time_ms: f64) {
        self.cache_metrics.update(hit, access_time_ms);
        self.last_updated = Utc::now();
    }
    
    /// Update performance metrics
    pub fn update_performance_metrics(&mut self, response_time_ms: f64) {
        self.performance_metrics.update(response_time_ms);
        self.last_updated = Utc::now();
    }
    
    /// Record an error
    pub fn record_error(&mut self, error_type: String, message: String, context: HashMap<String, String>) {
        self.error_metrics.record_error(error_type, message, context);
        self.last_updated = Utc::now();
    }
    
    /// Get overall success rate
    pub fn get_overall_success_rate(&self) -> f64 {
        if self.overall.total_operations > 0 {
            self.overall.successful_operations as f64 / self.overall.total_operations as f64
        } else {
            0.0
        }
    }
    
    /// Get element success rate
    pub fn get_element_success_rate(&self, element_id: &str) -> f64 {
        self.element_metrics.get(element_id)
            .map(|m| m.success_rate)
            .unwrap_or(0.0)
    }
    
    /// Get contributor success rate
    pub fn get_contributor_success_rate(&self, system_id: &str) -> f64 {
        self.contributor_metrics.get(system_id)
            .map(|m| m.success_rate)
            .unwrap_or(0.0)
    }
    
    /// Get plugin success rate
    pub fn get_plugin_success_rate(&self, plugin_id: &str) -> f64 {
        self.plugin_metrics.get(plugin_id)
            .map(|m| m.success_rate)
            .unwrap_or(0.0)
    }
    
    /// Reset all metrics
    pub fn reset(&mut self) {
        self.overall = OverallMetrics::new();
        self.element_metrics.clear();
        self.contributor_metrics.clear();
        self.plugin_metrics.clear();
        self.cache_metrics = CacheMetrics::new();
        self.performance_metrics = PerformanceMetrics::new();
        self.error_metrics = ErrorMetrics::new();
        self.last_updated = Utc::now();
    }
}

impl OverallMetrics {
    /// Create new overall metrics
    pub fn new() -> Self {
        Self {
            total_elements: 0,
            total_contributors: 0,
            total_plugins: 0,
            total_interactions: 0,
            uptime_seconds: 0,
            total_operations: 0,
            successful_operations: 0,
            failed_operations: 0,
        }
    }
    
    /// Update operation count
    pub fn update_operation(&mut self, success: bool) {
        self.total_operations += 1;
        if success {
            self.successful_operations += 1;
        } else {
            self.failed_operations += 1;
        }
    }
    
    /// Update element count
    pub fn update_element_count(&mut self, count: usize) {
        self.total_elements = count;
    }
    
    /// Update contributor count
    pub fn update_contributor_count(&mut self, count: usize) {
        self.total_contributors = count;
    }
    
    /// Update plugin count
    pub fn update_plugin_count(&mut self, count: usize) {
        self.total_plugins = count;
    }
    
    /// Update interaction count
    pub fn update_interaction_count(&mut self, count: usize) {
        self.total_interactions = count;
    }
    
    /// Update uptime
    pub fn update_uptime(&mut self, uptime_seconds: u64) {
        self.uptime_seconds = uptime_seconds;
    }
}

impl ElementMetrics {
    /// Create new element metrics
    pub fn new(element_id: String) -> Self {
        Self {
            element_id,
            access_count: 0,
            average_access_time_ms: 0.0,
            last_access: Utc::now(),
            cache_hit_rate: 0.0,
            error_count: 0,
            success_rate: 0.0,
        }
    }
    
    /// Update element metrics
    pub fn update(&mut self, access_time_ms: f64, success: bool) {
        self.access_count += 1;
        self.last_access = Utc::now();
        
        // Update average access time
        self.average_access_time_ms = (self.average_access_time_ms * (self.access_count - 1) as f64 + access_time_ms) / self.access_count as f64;
        
        // Update success rate
        if success {
            self.success_rate = (self.success_rate * (self.access_count - 1) as f64 + 1.0) / self.access_count as f64;
        } else {
            self.error_count += 1;
            self.success_rate = (self.success_rate * (self.access_count - 1) as f64) / self.access_count as f64;
        }
    }
}

impl ContributorMetrics {
    /// Create new contributor metrics
    pub fn new(system_id: String) -> Self {
        Self {
            system_id,
            contribution_count: 0,
            average_contribution_time_ms: 0.0,
            last_contribution: Utc::now(),
            error_count: 0,
            success_rate: 0.0,
            average_response_time_ms: 0.0,
        }
    }
    
    /// Update contributor metrics
    pub fn update(&mut self, response_time_ms: f64, success: bool) {
        self.contribution_count += 1;
        self.last_contribution = Utc::now();
        
        // Update average response time
        self.average_response_time_ms = (self.average_response_time_ms * (self.contribution_count - 1) as f64 + response_time_ms) / self.contribution_count as f64;
        
        // Update success rate
        if success {
            self.success_rate = (self.success_rate * (self.contribution_count - 1) as f64 + 1.0) / self.contribution_count as f64;
        } else {
            self.error_count += 1;
            self.success_rate = (self.success_rate * (self.contribution_count - 1) as f64) / self.contribution_count as f64;
        }
    }
}

impl PluginMetrics {
    /// Create new plugin metrics
    pub fn new(plugin_id: String) -> Self {
        Self {
            plugin_id,
            operation_count: 0,
            average_operation_time_ms: 0.0,
            last_operation: Utc::now(),
            error_count: 0,
            success_rate: 0.0,
            health_score: 1.0,
        }
    }
    
    /// Update plugin metrics
    pub fn update(&mut self, operation_time_ms: f64, success: bool) {
        self.operation_count += 1;
        self.last_operation = Utc::now();
        
        // Update average operation time
        self.average_operation_time_ms = (self.average_operation_time_ms * (self.operation_count - 1) as f64 + operation_time_ms) / self.operation_count as f64;
        
        // Update success rate and health score
        if success {
            self.success_rate = (self.success_rate * (self.operation_count - 1) as f64 + 1.0) / self.operation_count as f64;
        } else {
            self.error_count += 1;
            self.success_rate = (self.success_rate * (self.operation_count - 1) as f64) / self.operation_count as f64;
        }
        
        // Update health score based on success rate
        self.health_score = self.success_rate;
    }
}

impl CacheMetrics {
    /// Create new cache metrics
    pub fn new() -> Self {
        Self {
            total_size: 0,
            hit_count: 0,
            miss_count: 0,
            hit_rate: 0.0,
            eviction_count: 0,
            average_access_time_ms: 0.0,
            memory_usage_bytes: 0,
        }
    }
    
    /// Update cache metrics
    pub fn update(&mut self, hit: bool, access_time_ms: f64) {
        if hit {
            self.hit_count += 1;
        } else {
            self.miss_count += 1;
        }
        
        let total_accesses = self.hit_count + self.miss_count;
        self.hit_rate = if total_accesses > 0 {
            self.hit_count as f64 / total_accesses as f64
        } else {
            0.0
        };
        
        // Update average access time
        self.average_access_time_ms = (self.average_access_time_ms * (total_accesses - 1) as f64 + access_time_ms) / total_accesses as f64;
    }
    
    /// Record eviction
    pub fn record_eviction(&mut self) {
        self.eviction_count += 1;
    }
}

impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new() -> Self {
        Self {
            average_response_time_ms: 0.0,
            p95_response_time_ms: 0.0,
            p99_response_time_ms: 0.0,
            max_response_time_ms: 0.0,
            min_response_time_ms: f64::MAX,
            operations_per_second: 0.0,
            peak_operations_per_second: 0.0,
            memory_usage_bytes: 0,
            cpu_usage_percentage: 0.0,
        }
    }
    
    /// Update performance metrics
    pub fn update(&mut self, response_time_ms: f64) {
        // Update min/max
        self.min_response_time_ms = self.min_response_time_ms.min(response_time_ms);
        self.max_response_time_ms = self.max_response_time_ms.max(response_time_ms);
        
        // TODO: Implement percentile calculations
        // For now, just update average
        self.average_response_time_ms = (self.average_response_time_ms + response_time_ms) / 2.0;
    }
}

impl ErrorMetrics {
    /// Create new error metrics
    pub fn new() -> Self {
        Self {
            total_errors: 0,
            error_rate: 0.0,
            error_types: HashMap::new(),
            recent_errors: Vec::new(),
            recovery_count: 0,
            average_recovery_time_ms: 0.0,
        }
    }
    
    /// Record an error
    pub fn record_error(&mut self, error_type: String, message: String, context: HashMap<String, String>) {
        self.total_errors += 1;
        
        // Update error type count
        *self.error_types.entry(error_type.clone()).or_insert(0) += 1;
        
        // Add to recent errors
        let error_record = ErrorRecord {
            timestamp: Utc::now(),
            error_type,
            message,
            context,
            recovery_time_ms: None,
        };
        
        self.recent_errors.push(error_record);
        
        // Keep only last 100 errors
        if self.recent_errors.len() > 100 {
            self.recent_errors.remove(0);
        }
    }
    
    /// Record recovery
    pub fn record_recovery(&mut self, recovery_time_ms: f64) {
        self.recovery_count += 1;
        self.average_recovery_time_ms = (self.average_recovery_time_ms * (self.recovery_count - 1) as f64 + recovery_time_ms) / self.recovery_count as f64;
    }
}

impl Default for RegistryMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for OverallMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CacheMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ErrorMetrics {
    fn default() -> Self {
        Self::new()
    }
}
