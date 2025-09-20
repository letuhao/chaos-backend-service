//! Metrics collection system for Actor Core.
//!
//! This module provides comprehensive metrics collection, aggregation,
//! and reporting capabilities for monitoring Actor Core performance.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn};

use crate::ActorCoreResult;
use crate::ActorCoreError;

/// Metrics collector for gathering and aggregating performance metrics.
pub struct MetricsCollector {
    /// Atomic counters for thread-safe metrics collection
    counters: HashMap<String, Arc<AtomicU64>>,
    /// Histograms for latency measurements
    histograms: HashMap<String, Arc<Histogram>>,
    /// Gauges for current values
    gauges: HashMap<String, Arc<AtomicU64>>,
    /// Metrics metadata
    metadata: HashMap<String, MetricMetadata>,
    /// Collection start time
    start_time: Instant,
}

/// Histogram for latency and distribution metrics.
#[derive(Debug)]
pub struct Histogram {
    /// Buckets for histogram data
    buckets: Vec<(f64, AtomicU64)>, // (upper_bound, count)
    /// Total count of observations
    count: AtomicU64,
    /// Sum of all observations
    sum: AtomicU64,
}

/// Metadata for a metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricMetadata {
    /// Metric name
    pub name: String,
    /// Metric type
    pub metric_type: MetricType,
    /// Description of what this metric measures
    pub description: String,
    /// Labels for this metric
    pub labels: HashMap<String, String>,
    /// Unit of measurement
    pub unit: Option<String>,
    /// Whether this metric is enabled
    pub enabled: bool,
}

/// Types of metrics supported.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetricType {
    /// Counter - monotonically increasing value
    Counter,
    /// Gauge - value that can go up or down
    Gauge,
    /// Histogram - distribution of values
    Histogram,
    /// Summary - quantiles and counts
    Summary,
}

/// Collected metric value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    /// Metric name
    pub name: String,
    /// Metric type
    pub metric_type: MetricType,
    /// Current value
    pub value: f64,
    /// Labels
    pub labels: HashMap<String, String>,
    /// Timestamp when this metric was collected
    pub timestamp: SystemTime,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Histogram metric value with bucket information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramValue {
    /// Metric name
    pub name: String,
    /// Total count of observations
    pub count: u64,
    /// Sum of all observations
    pub sum: f64,
    /// Buckets with their counts
    pub buckets: Vec<(f64, u64)>, // (upper_bound, count)
    /// Labels
    pub labels: HashMap<String, String>,
    /// Timestamp when this metric was collected
    pub timestamp: SystemTime,
}

/// Metrics snapshot containing all collected metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    /// Timestamp when this snapshot was taken
    pub timestamp: SystemTime,
    /// Counter metrics
    pub counters: HashMap<String, MetricValue>,
    /// Gauge metrics
    pub gauges: HashMap<String, MetricValue>,
    /// Histogram metrics
    pub histograms: HashMap<String, HistogramValue>,
    /// Snapshot metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl MetricsCollector {
    /// Create a new metrics collector.
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
            histograms: HashMap::new(),
            gauges: HashMap::new(),
            metadata: HashMap::new(),
            start_time: Instant::now(),
        }
    }

    /// Register a counter metric.
    pub fn register_counter(
        &mut self,
        name: String,
        description: String,
        labels: HashMap<String, String>,
    ) -> ActorCoreResult<()> {
        let metadata = MetricMetadata {
            name: name.clone(),
            metric_type: MetricType::Counter,
            description,
            labels,
            unit: None,
            enabled: true,
        };

        self.metadata.insert(name.clone(), metadata);
        self.counters.insert(name.clone(), Arc::new(AtomicU64::new(0)));

        debug!(metric_name = %name, "Registered counter metric");
        Ok(())
    }

    /// Register a gauge metric.
    pub fn register_gauge(
        &mut self,
        name: String,
        description: String,
        labels: HashMap<String, String>,
        initial_value: Option<u64>,
    ) -> ActorCoreResult<()> {
        let metadata = MetricMetadata {
            name: name.clone(),
            metric_type: MetricType::Gauge,
            description,
            labels,
            unit: None,
            enabled: true,
        };

        self.metadata.insert(name.clone(), metadata);
        self.gauges.insert(name.clone(), Arc::new(AtomicU64::new(initial_value.unwrap_or(0))));

        debug!(metric_name = %name, "Registered gauge metric");
        Ok(())
    }

    /// Register a histogram metric.
    pub fn register_histogram(
        &mut self,
        name: String,
        description: String,
        labels: HashMap<String, String>,
        buckets: Vec<f64>,
    ) -> ActorCoreResult<()> {
        let metadata = MetricMetadata {
            name: name.clone(),
            metric_type: MetricType::Histogram,
            description,
            labels,
            unit: None,
            enabled: true,
        };

        let histogram = Histogram::new(buckets);
        self.metadata.insert(name.clone(), metadata);
        self.histograms.insert(name.clone(), Arc::new(histogram));

        debug!(metric_name = %name, "Registered histogram metric");
        Ok(())
    }

    /// Increment a counter metric.
    pub fn increment_counter(&self, name: &str, value: u64) -> ActorCoreResult<()> {
        if let Some(counter) = self.counters.get(name) {
            counter.fetch_add(value, Ordering::Relaxed);
            debug!(metric_name = %name, increment = value, "Incremented counter");
            Ok(())
        } else {
            Err(ActorCoreError::InvalidInput(format!("Counter metric '{}' not found", name)))
        }
    }

    /// Set a gauge metric value.
    pub fn set_gauge(&self, name: &str, value: u64) -> ActorCoreResult<()> {
        if let Some(gauge) = self.gauges.get(name) {
            gauge.store(value, Ordering::Relaxed);
            debug!(metric_name = %name, value = value, "Set gauge value");
            Ok(())
        } else {
            Err(ActorCoreError::InvalidInput(format!("Gauge metric '{}' not found", name)))
        }
    }

    /// Observe a value in a histogram.
    pub fn observe_histogram(&self, name: &str, value: f64) -> ActorCoreResult<()> {
        if let Some(histogram) = self.histograms.get(name) {
            histogram.observe(value);
            debug!(metric_name = %name, value = value, "Observed histogram value");
            Ok(())
        } else {
            Err(ActorCoreError::InvalidInput(format!("Histogram metric '{}' not found", name)))
        }
    }

    /// Record a duration in a histogram (convenience method).
    pub fn record_duration(&self, name: &str, duration: Duration) -> ActorCoreResult<()> {
        let duration_ms = duration.as_millis() as f64;
        self.observe_histogram(name, duration_ms)
    }

    /// Record a duration from an instant (convenience method).
    pub fn record_duration_since(&self, name: &str, start: Instant) -> ActorCoreResult<()> {
        let duration = start.elapsed();
        self.record_duration(name, duration)
    }

    /// Get current metric values.
    pub fn get_metric(&self, name: &str) -> Option<MetricValue> {
        if let Some(metadata) = self.metadata.get(name) {
            if !metadata.enabled {
                return None;
            }

            let value = match metadata.metric_type {
                MetricType::Counter => {
                    self.counters.get(name)?.load(Ordering::Relaxed) as f64
                }
                MetricType::Gauge => {
                    self.gauges.get(name)?.load(Ordering::Relaxed) as f64
                }
                MetricType::Histogram => {
                    // For histograms, return the count as the primary value
                    self.histograms.get(name)?.count() as f64
                }
                MetricType::Summary => {
                    // Summaries not implemented yet
                    0.0
                }
            };

            Some(MetricValue {
                name: name.to_string(),
                metric_type: metadata.metric_type.clone(),
                value,
                labels: metadata.labels.clone(),
                timestamp: SystemTime::now(),
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }

    /// Get histogram value with bucket information.
    pub fn get_histogram(&self, name: &str) -> Option<HistogramValue> {
        if let Some(metadata) = self.metadata.get(name) {
            if !metadata.enabled || metadata.metric_type != MetricType::Histogram {
                return None;
            }

            if let Some(histogram) = self.histograms.get(name) {
                Some(histogram.value())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Take a snapshot of all metrics.
    pub fn snapshot(&self) -> MetricsSnapshot {
        let mut counters = HashMap::new();
        let mut gauges = HashMap::new();
        let mut histograms = HashMap::new();

        for (name, _) in &self.counters {
            if let Some(value) = self.get_metric(name) {
                counters.insert(name.clone(), value);
            }
        }

        for (name, _) in &self.gauges {
            if let Some(value) = self.get_metric(name) {
                gauges.insert(name.clone(), value);
            }
        }

        for (name, _) in &self.histograms {
            if let Some(value) = self.get_histogram(name) {
                histograms.insert(name.clone(), value);
            }
        }

        let mut metadata = HashMap::new();
        metadata.insert("collection_duration_seconds".to_string(), 
            serde_json::Value::Number(serde_json::Number::from_f64(
                self.start_time.elapsed().as_secs_f64()
            ).unwrap()));

        MetricsSnapshot {
            timestamp: SystemTime::now(),
            counters,
            gauges,
            histograms,
            metadata,
        }
    }

    /// Get metrics metadata.
    pub fn get_metadata(&self) -> &HashMap<String, MetricMetadata> {
        &self.metadata
    }

    /// Enable or disable a metric.
    pub fn set_metric_enabled(&mut self, name: &str, enabled: bool) -> ActorCoreResult<()> {
        if let Some(metadata) = self.metadata.get_mut(name) {
            metadata.enabled = enabled;
            debug!(metric_name = %name, enabled = enabled, "Set metric enabled state");
            Ok(())
        } else {
            Err(ActorCoreError::InvalidInput(format!("Metric '{}' not found", name)))
        }
    }

    /// List all registered metrics.
    pub fn list_metrics(&self) -> Vec<&MetricMetadata> {
        self.metadata.values().collect()
    }
}

impl Histogram {
    /// Create a new histogram with the given buckets.
    pub fn new(buckets: Vec<f64>) -> Self {
        let mut histogram_buckets = buckets.into_iter()
            .map(|bound| (bound, AtomicU64::new(0)))
            .collect::<Vec<_>>();
        
        // Add +Inf bucket
        histogram_buckets.push((f64::INFINITY, AtomicU64::new(0)));

        Self {
            buckets: histogram_buckets,
            count: AtomicU64::new(0),
            sum: AtomicU64::new(0),
        }
    }

    /// Observe a value in the histogram.
    pub fn observe(&self, value: f64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.sum.fetch_add(value as u64, Ordering::Relaxed);

        // Find the appropriate bucket
        for (bound, bucket_count) in &self.buckets {
            if value <= *bound {
                bucket_count.fetch_add(1, Ordering::Relaxed);
                break;
            }
        }
    }

    /// Get the current count of observations.
    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    /// Get the sum of all observations.
    pub fn sum(&self) -> f64 {
        self.sum.load(Ordering::Relaxed) as f64
    }

    /// Get the histogram value with bucket information.
    pub fn value(&self) -> HistogramValue {
        let buckets: Vec<(f64, u64)> = self.buckets.iter()
            .map(|(bound, count)| (*bound, count.load(Ordering::Relaxed)))
            .collect();

        HistogramValue {
            name: String::new(), // Will be set by caller
            count: self.count(),
            sum: self.sum(),
            buckets,
            labels: HashMap::new(), // Will be set by caller
            timestamp: SystemTime::now(),
        }
    }
}

/// Default metrics for Actor Core system.
pub mod default_metrics {
    use super::*;

    /// Register default metrics for Actor Core.
    pub fn register_default_metrics(collector: &mut MetricsCollector) -> ActorCoreResult<()> {
        // Load metrics configuration
        let config = MetricsCollectorConfig::load_config().unwrap_or_else(|_| {
            warn!("Failed to load metrics collector config, using hardcoded defaults");
            MetricsCollectorConfig::get_default_config()
        });

        // Register metrics from configuration
        let metrics_count = config.metrics.len();
        for metric_config in config.metrics {
            match metric_config.metric_type {
                MetricType::Counter => {
                    collector.register_counter(
                        metric_config.name,
                        metric_config.description,
                        metric_config.labels,
                    )?;
                }
                MetricType::Gauge => {
                    collector.register_gauge(
                        metric_config.name,
                        metric_config.description,
                        metric_config.labels,
                        metric_config.initial_value,
                    )?;
                }
                MetricType::Histogram => {
                    collector.register_histogram(
                        metric_config.name,
                        metric_config.description,
                        metric_config.labels,
                        metric_config.buckets.unwrap_or_else(|| vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0]),
                    )?;
                }
                MetricType::Summary => {
                    // Summaries not implemented yet
                    warn!("Summary metrics not implemented yet, skipping: {}", metric_config.name);
                }
            }
        }

        info!("Registered {} Actor Core metrics from configuration", metrics_count);
        Ok(())
    }
}

/// Metrics collector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsCollectorConfig {
    pub metrics: Vec<MetricConfig>,
}

/// Individual metric configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricConfig {
    pub name: String,
    pub description: String,
    pub metric_type: MetricType,
    pub labels: HashMap<String, String>,
    pub unit: Option<String>,
    pub enabled: bool,
    pub initial_value: Option<u64>,
    pub buckets: Option<Vec<f64>>,
}

impl MetricsCollectorConfig {
    /// Load metrics collector configuration from config file
    pub fn load_config() -> ActorCoreResult<Self> {
        // Try to load from metrics_collector_config.yaml first
        let config_path = std::path::Path::new("configs/metrics_collector_config.yaml");
            
        if config_path.exists() {
            match Self::load_config_from_file(config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    warn!("Failed to load metrics collector config from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_config())
    }

    /// Load configuration from file
    fn load_config_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: MetricsCollectorConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default configuration
    fn get_default_config() -> Self {
        let mut metrics = Vec::new();

        // Actor resolution metrics
        metrics.push(MetricConfig {
            name: "actor_resolutions_total".to_string(),
            description: "Total number of actor stat resolutions".to_string(),
            metric_type: MetricType::Counter,
            labels: HashMap::new(),
            unit: Some("count".to_string()),
            enabled: true,
            initial_value: None,
            buckets: None,
        });

        metrics.push(MetricConfig {
            name: "actor_resolution_errors_total".to_string(),
            description: "Total number of actor resolution errors".to_string(),
            metric_type: MetricType::Counter,
            labels: HashMap::new(),
            unit: Some("count".to_string()),
            enabled: true,
            initial_value: None,
            buckets: None,
        });

        metrics.push(MetricConfig {
            name: "actor_resolution_duration_ms".to_string(),
            description: "Duration of actor stat resolutions in milliseconds".to_string(),
            metric_type: MetricType::Histogram,
            labels: HashMap::new(),
            unit: Some("milliseconds".to_string()),
            enabled: true,
            initial_value: None,
            buckets: Some(vec![1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0]),
        });

        // Cache metrics
        metrics.push(MetricConfig {
            name: "cache_operations_total".to_string(),
            description: "Total number of cache operations".to_string(),
            metric_type: MetricType::Counter,
            labels: HashMap::new(),
            unit: Some("count".to_string()),
            enabled: true,
            initial_value: None,
            buckets: None,
        });

        metrics.push(MetricConfig {
            name: "cache_hits_total".to_string(),
            description: "Total number of cache hits".to_string(),
            metric_type: MetricType::Counter,
            labels: HashMap::new(),
            unit: Some("count".to_string()),
            enabled: true,
            initial_value: None,
            buckets: None,
        });

        metrics.push(MetricConfig {
            name: "cache_misses_total".to_string(),
            description: "Total number of cache misses".to_string(),
            metric_type: MetricType::Counter,
            labels: HashMap::new(),
            unit: Some("count".to_string()),
            enabled: true,
            initial_value: None,
            buckets: None,
        });

        metrics.push(MetricConfig {
            name: "cache_operation_duration_ms".to_string(),
            description: "Duration of cache operations in milliseconds".to_string(),
            metric_type: MetricType::Histogram,
            labels: HashMap::new(),
            unit: Some("milliseconds".to_string()),
            enabled: true,
            initial_value: None,
            buckets: Some(vec![0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 25.0, 50.0]),
        });

        // Subsystem metrics
        metrics.push(MetricConfig {
            name: "subsystem_contributions_total".to_string(),
            description: "Total number of subsystem contributions".to_string(),
            metric_type: MetricType::Counter,
            labels: HashMap::new(),
            unit: Some("count".to_string()),
            enabled: true,
            initial_value: None,
            buckets: None,
        });

        metrics.push(MetricConfig {
            name: "subsystem_errors_total".to_string(),
            description: "Total number of subsystem errors".to_string(),
            metric_type: MetricType::Counter,
            labels: HashMap::new(),
            unit: Some("count".to_string()),
            enabled: true,
            initial_value: None,
            buckets: None,
        });

        metrics.push(MetricConfig {
            name: "subsystem_processing_duration_ms".to_string(),
            description: "Duration of subsystem processing in milliseconds".to_string(),
            metric_type: MetricType::Histogram,
            labels: HashMap::new(),
            unit: Some("milliseconds".to_string()),
            enabled: true,
            initial_value: None,
            buckets: Some(vec![0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 25.0, 50.0]),
        });

        // Memory metrics
        metrics.push(MetricConfig {
            name: "memory_usage_bytes".to_string(),
            description: "Current memory usage in bytes".to_string(),
            metric_type: MetricType::Gauge,
            labels: HashMap::new(),
            unit: Some("bytes".to_string()),
            enabled: true,
            initial_value: Some(0),
            buckets: None,
        });

        metrics.push(MetricConfig {
            name: "active_actors_count".to_string(),
            description: "Current number of active actors".to_string(),
            metric_type: MetricType::Gauge,
            labels: HashMap::new(),
            unit: Some("count".to_string()),
            enabled: true,
            initial_value: Some(0),
            buckets: None,
        });

        // Validation metrics
        metrics.push(MetricConfig {
            name: "validation_checks_total".to_string(),
            description: "Total number of validation checks performed".to_string(),
            metric_type: MetricType::Counter,
            labels: HashMap::new(),
            unit: Some("count".to_string()),
            enabled: true,
            initial_value: None,
            buckets: None,
        });

        metrics.push(MetricConfig {
            name: "validation_failures_total".to_string(),
            description: "Total number of validation failures".to_string(),
            metric_type: MetricType::Counter,
            labels: HashMap::new(),
            unit: Some("count".to_string()),
            enabled: true,
            initial_value: None,
            buckets: None,
        });

        metrics.push(MetricConfig {
            name: "validation_duration_ms".to_string(),
            description: "Duration of validation operations in milliseconds".to_string(),
            metric_type: MetricType::Histogram,
            labels: HashMap::new(),
            unit: Some("milliseconds".to_string()),
            enabled: true,
            initial_value: None,
            buckets: Some(vec![0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 25.0, 50.0]),
        });

        Self { metrics }
    }
}