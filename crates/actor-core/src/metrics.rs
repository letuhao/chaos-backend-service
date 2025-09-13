//! Metrics and statistics structures for the Actor Core system.
//!
//! This module defines various metrics and statistics structures used
//! throughout the system for performance monitoring and observability.

use serde::{Deserialize, Serialize};

/// SubsystemMetrics contains performance metrics for a subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemMetrics {
    /// Number of contributions made
    pub contributions_count: u64,
    /// Average processing time in microseconds
    pub avg_processing_time: u64,
    /// Maximum processing time in microseconds
    pub max_processing_time: u64,
    /// Number of errors encountered
    pub error_count: u64,
    /// Last contribution timestamp
    pub last_contribution: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for SubsystemMetrics {
    fn default() -> Self {
        Self {
            contributions_count: 0,
            avg_processing_time: 0,
            max_processing_time: 0,
            error_count: 0,
            last_contribution: None,
        }
    }
}

/// AggregatorMetrics contains performance metrics for the aggregator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatorMetrics {
    /// Total number of resolutions performed
    pub total_resolutions: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Average resolution time in microseconds
    pub avg_resolution_time: u64,
    /// Maximum resolution time in microseconds
    pub max_resolution_time: u64,
    /// Number of errors encountered
    pub error_count: u64,
    /// Number of active subsystems
    pub active_subsystems: usize,
}

impl Default for AggregatorMetrics {
    fn default() -> Self {
        Self {
            total_resolutions: 0,
            cache_hits: 0,
            cache_misses: 0,
            avg_resolution_time: 0,
            max_resolution_time: 0,
            error_count: 0,
            active_subsystems: 0,
        }
    }
}

/// CapStatistics contains statistics about cap usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapStatistics {
    /// Total number of cap calculations
    pub total_calculations: u64,
    /// Number of dimensions with caps
    pub dimensions_with_caps: usize,
    /// Average cap calculation time in microseconds
    pub avg_calculation_time: u64,
    /// Maximum cap calculation time in microseconds
    pub max_calculation_time: u64,
}

impl Default for CapStatistics {
    fn default() -> Self {
        Self {
            total_calculations: 0,
            dimensions_with_caps: 0,
            avg_calculation_time: 0,
            max_calculation_time: 0,
        }
    }
}

/// CacheStats contains statistics about cache usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of cache sets
    pub sets: u64,
    /// Number of cache deletes
    pub deletes: u64,
    /// Current memory usage in bytes
    pub memory_usage: u64,
    /// Maximum memory usage in bytes
    pub max_memory_usage: u64,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            sets: 0,
            deletes: 0,
            memory_usage: 0,
            max_memory_usage: 0,
        }
    }
}