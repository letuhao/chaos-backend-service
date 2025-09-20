//! Metrics and statistics structures for the Actor Core system.
//!
//! This module defines various metrics and statistics structures used
//! throughout the system for performance monitoring and observability.

use serde::{Deserialize, Serialize};
use tracing;
use crate::ActorCoreResult;

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
        Self::load_default_metrics().unwrap_or_else(|_| {
            tracing::warn!("Failed to load subsystem metrics config, using hardcoded defaults");
            Self {
                contributions_count: 0,
                avg_processing_time: 0,
                max_processing_time: 0,
                error_count: 0,
                last_contribution: None,
            }
        })
    }
}

impl SubsystemMetrics {
    /// Load default subsystem metrics from configuration
    pub fn load_default_metrics() -> ActorCoreResult<Self> {
        // Try to load from metrics_config.yaml first
        let config_path = std::path::Path::new("configs/metrics_config.yaml");
            
        if config_path.exists() {
            match Self::load_metrics_from_file(config_path) {
                Ok(metrics) => return Ok(metrics),
                Err(e) => {
                    tracing::warn!("Failed to load subsystem metrics from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self {
            contributions_count: 0,
            avg_processing_time: 0,
            max_processing_time: 0,
            error_count: 0,
            last_contribution: None,
        })
    }

    /// Load subsystem metrics from file
    fn load_metrics_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: MetricsConfig = serde_yaml::from_str(&content)?;
        Ok(Self {
            contributions_count: config.subsystem.default_contributions_count,
            avg_processing_time: config.subsystem.default_avg_processing_time,
            max_processing_time: config.subsystem.default_max_processing_time,
            error_count: config.subsystem.default_error_count,
            last_contribution: None,
        })
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
        Self::load_default_metrics().unwrap_or_else(|_| {
            tracing::warn!("Failed to load aggregator metrics config, using hardcoded defaults");
            Self {
                total_resolutions: 0,
                cache_hits: 0,
                cache_misses: 0,
                avg_resolution_time: 0,
                max_resolution_time: 0,
                error_count: 0,
                active_subsystems: 0,
            }
        })
    }
}

impl AggregatorMetrics {
    /// Load default aggregator metrics from configuration
    pub fn load_default_metrics() -> ActorCoreResult<Self> {
        // Try to load from metrics_config.yaml first
        let config_path = std::path::Path::new("configs/metrics_config.yaml");
            
        if config_path.exists() {
            match Self::load_metrics_from_file(config_path) {
                Ok(metrics) => return Ok(metrics),
                Err(e) => {
                    tracing::warn!("Failed to load aggregator metrics from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self {
            total_resolutions: 0,
            cache_hits: 0,
            cache_misses: 0,
            avg_resolution_time: 0,
            max_resolution_time: 0,
            error_count: 0,
            active_subsystems: 0,
        })
    }

    /// Load aggregator metrics from file
    fn load_metrics_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: MetricsConfig = serde_yaml::from_str(&content)?;
        Ok(Self {
            total_resolutions: config.aggregator.default_total_resolutions,
            cache_hits: config.aggregator.default_cache_hits,
            cache_misses: config.aggregator.default_cache_misses,
            avg_resolution_time: config.aggregator.default_avg_resolution_time,
            max_resolution_time: config.aggregator.default_max_resolution_time,
            error_count: config.aggregator.default_error_count,
            active_subsystems: config.aggregator.default_active_subsystems,
        })
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
        Self::load_default_metrics().unwrap_or_else(|_| {
            tracing::warn!("Failed to load cap statistics config, using hardcoded defaults");
            Self {
                total_calculations: 0,
                dimensions_with_caps: 0,
                avg_calculation_time: 0,
                max_calculation_time: 0,
            }
        })
    }
}

impl CapStatistics {
    /// Load default cap statistics from configuration
    pub fn load_default_metrics() -> ActorCoreResult<Self> {
        // Try to load from metrics_config.yaml first
        let config_path = std::path::Path::new("configs/metrics_config.yaml");
            
        if config_path.exists() {
            match Self::load_metrics_from_file(config_path) {
                Ok(metrics) => return Ok(metrics),
                Err(e) => {
                    tracing::warn!("Failed to load cap statistics from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self {
            total_calculations: 0,
            dimensions_with_caps: 0,
            avg_calculation_time: 0,
            max_calculation_time: 0,
        })
    }

    /// Load cap statistics from file
    fn load_metrics_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: MetricsConfig = serde_yaml::from_str(&content)?;
        Ok(Self {
            total_calculations: config.cap_statistics.default_total_calculations,
            dimensions_with_caps: config.cap_statistics.default_dimensions_with_caps,
            avg_calculation_time: config.cap_statistics.default_avg_calculation_time,
            max_calculation_time: config.cap_statistics.default_max_calculation_time,
        })
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
        Self::load_default_metrics().unwrap_or_else(|_| {
            tracing::warn!("Failed to load cache stats config, using hardcoded defaults");
            Self {
                hits: 0,
                misses: 0,
                sets: 0,
                deletes: 0,
                memory_usage: 0,
                max_memory_usage: 0,
            }
        })
    }
}

impl CacheStats {
    /// Load default cache stats from configuration
    pub fn load_default_metrics() -> ActorCoreResult<Self> {
        // Try to load from metrics_config.yaml first
        let config_path = std::path::Path::new("configs/metrics_config.yaml");
            
        if config_path.exists() {
            match Self::load_metrics_from_file(config_path) {
                Ok(metrics) => return Ok(metrics),
                Err(e) => {
                    tracing::warn!("Failed to load cache stats from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self {
            hits: 0,
            misses: 0,
            sets: 0,
            deletes: 0,
            memory_usage: 0,
            max_memory_usage: 0,
        })
    }

    /// Load cache stats from file
    fn load_metrics_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: MetricsConfig = serde_yaml::from_str(&content)?;
        Ok(Self {
            hits: config.cache_stats.default_hits,
            misses: config.cache_stats.default_misses,
            sets: config.cache_stats.default_sets,
            deletes: config.cache_stats.default_deletes,
            memory_usage: config.cache_stats.default_memory_usage,
            max_memory_usage: config.cache_stats.default_max_memory_usage,
        })
    }
}

/// Metrics configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub subsystem: SubsystemMetricsConfig,
    pub aggregator: AggregatorMetricsConfig,
    pub cap_statistics: CapStatisticsConfig,
    pub cache_stats: CacheStatsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemMetricsConfig {
    pub default_contributions_count: u64,
    pub default_avg_processing_time: u64,
    pub default_max_processing_time: u64,
    pub default_error_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatorMetricsConfig {
    pub default_total_resolutions: u64,
    pub default_cache_hits: u64,
    pub default_cache_misses: u64,
    pub default_avg_resolution_time: u64,
    pub default_max_resolution_time: u64,
    pub default_error_count: u64,
    pub default_active_subsystems: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapStatisticsConfig {
    pub default_total_calculations: u64,
    pub default_dimensions_with_caps: usize,
    pub default_avg_calculation_time: u64,
    pub default_max_calculation_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatsConfig {
    pub default_hits: u64,
    pub default_misses: u64,
    pub default_sets: u64,
    pub default_deletes: u64,
    pub default_memory_usage: u64,
    pub default_max_memory_usage: u64,
}