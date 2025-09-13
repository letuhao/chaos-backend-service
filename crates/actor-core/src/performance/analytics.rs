//! Real-time analytics and monitoring for the Actor Core system.
//!
//! This module provides real-time performance monitoring, metrics collection,
//! and analytics for the actor stat aggregation system.

use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Real-time analytics collector.
pub struct AnalyticsCollector {
    /// Metrics storage
    metrics: Arc<std::sync::RwLock<HashMap<String, MetricValue>>>,
    /// Performance counters
    counters: Arc<std::sync::RwLock<HashMap<String, u64>>>,
    /// Time series data
    time_series: Arc<std::sync::RwLock<Vec<TimeSeriesPoint>>>,
    /// Configuration
    config: AnalyticsConfig,
}

/// Configuration for analytics collection.
#[derive(Debug, Clone)]
pub struct AnalyticsConfig {
    /// Enable real-time analytics
    pub enable_analytics: bool,
    /// Metrics collection interval
    pub collection_interval: Duration,
    /// Maximum number of time series points to keep
    pub max_time_series_points: usize,
    /// Enable performance counters
    pub enable_counters: bool,
    /// Enable memory tracking
    pub enable_memory_tracking: bool,
    /// Enable latency tracking
    pub enable_latency_tracking: bool,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            enable_analytics: true,
            collection_interval: Duration::from_secs(1),
            max_time_series_points: 10000,
            enable_counters: true,
            enable_memory_tracking: true,
            enable_latency_tracking: true,
        }
    }
}

/// A metric value that can be stored in analytics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
    String(String),
}

/// A time series data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    /// Timestamp
    pub timestamp: u64,
    /// Metric name
    pub metric: String,
    /// Metric value
    pub value: MetricValue,
    /// Tags
    pub tags: HashMap<String, String>,
}

/// Performance metrics for the actor core system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// System metrics
    pub system: SystemMetrics,
    /// Cache metrics
    pub cache: CacheMetrics,
    /// Aggregation metrics
    pub aggregation: AggregationMetrics,
    /// Memory metrics
    pub memory: MemoryMetrics,
    /// Error metrics
    pub errors: ErrorMetrics,
}

/// System-level performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Current CPU usage percentage
    pub cpu_usage: f64,
    /// Current memory usage in bytes
    pub memory_usage: u64,
    /// Number of active threads
    pub thread_count: usize,
    /// Uptime in seconds
    pub uptime: u64,
    /// Number of active connections
    pub active_connections: u64,
}

/// Cache performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    /// Cache hit rate
    pub hit_rate: f64,
    /// Cache miss rate
    pub miss_rate: f64,
    /// Average cache access time
    pub avg_access_time: Duration,
    /// Cache size in bytes
    pub cache_size: u64,
    /// Number of cache evictions
    pub evictions: u64,
    /// L1 cache hit rate
    pub l1_hit_rate: f64,
    /// L2 cache hit rate
    pub l2_hit_rate: f64,
    /// L3 cache hit rate
    pub l3_hit_rate: f64,
}

/// Aggregation performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationMetrics {
    /// Number of aggregations performed
    pub total_aggregations: u64,
    /// Average aggregation time
    pub avg_aggregation_time: Duration,
    /// Number of active actors
    pub active_actors: u64,
    /// Number of active subsystems
    pub active_subsystems: u64,
    /// Average contributions per aggregation
    pub avg_contributions: f64,
    /// Average caps per aggregation
    pub avg_caps: f64,
}

/// Memory usage metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// Total memory usage in bytes
    pub total_memory: u64,
    /// Memory pool usage in bytes
    pub pool_memory: u64,
    /// Cache memory usage in bytes
    pub cache_memory: u64,
    /// Allocated memory in bytes
    pub allocated_memory: u64,
    /// Peak memory usage in bytes
    pub peak_memory: u64,
    /// Memory fragmentation percentage
    pub fragmentation: f64,
}

/// Error metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    /// Total number of errors
    pub total_errors: u64,
    /// Error rate (errors per second)
    pub error_rate: f64,
    /// Number of cache errors
    pub cache_errors: u64,
    /// Number of aggregation errors
    pub aggregation_errors: u64,
    /// Number of memory errors
    pub memory_errors: u64,
    /// Number of validation errors
    pub validation_errors: u64,
}

impl AnalyticsCollector {
    /// Create a new analytics collector.
    pub fn new(config: AnalyticsConfig) -> Self {
        Self {
            metrics: Arc::new(std::sync::RwLock::new(HashMap::new())),
            counters: Arc::new(std::sync::RwLock::new(HashMap::new())),
            time_series: Arc::new(std::sync::RwLock::new(Vec::new())),
            config,
        }
    }

    /// Create a new analytics collector with default configuration.
    pub fn new_default() -> Self {
        Self::new(AnalyticsConfig::default())
    }

    /// Record a metric value.
    pub fn record_metric(&self, name: &str, value: MetricValue) {
        if !self.config.enable_analytics {
            return;
        }

        let mut metrics = self.metrics.write().unwrap();
        metrics.insert(name.to_string(), value);
    }

    /// Increment a counter.
    pub fn increment_counter(&self, name: &str, value: u64) {
        if !self.config.enable_counters {
            return;
        }

        let mut counters = self.counters.write().unwrap();
        *counters.entry(name.to_string()).or_insert(0) += value;
    }

    /// Record a time series point.
    pub fn record_time_series(&self, metric: &str, value: MetricValue, tags: HashMap<String, String>) {
        if !self.config.enable_analytics {
            return;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let point = TimeSeriesPoint {
            timestamp,
            metric: metric.to_string(),
            value,
            tags,
        };

        let mut time_series = self.time_series.write().unwrap();
        time_series.push(point);

        // Keep only the most recent points
        if time_series.len() > self.config.max_time_series_points {
            let excess = time_series.len() - self.config.max_time_series_points;
            time_series.drain(0..excess);
        }
    }

    /// Record a latency measurement.
    pub fn record_latency(&self, operation: &str, latency: Duration) {
        if !self.config.enable_latency_tracking {
            return;
        }

        let mut tags = HashMap::new();
        tags.insert("operation".to_string(), operation.to_string());
        
        self.record_time_series(
            "latency",
            MetricValue::Gauge(latency.as_micros() as f64),
            tags,
        );
    }

    /// Record memory usage.
    pub fn record_memory_usage(&self, component: &str, usage: u64) {
        if !self.config.enable_memory_tracking {
            return;
        }

        let mut tags = HashMap::new();
        tags.insert("component".to_string(), component.to_string());
        
        self.record_time_series(
            "memory_usage",
            MetricValue::Gauge(usage as f64),
            tags,
        );
    }

    /// Get current performance metrics.
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        let metrics = self.metrics.read().unwrap();
        let counters = self.counters.read().unwrap();

        PerformanceMetrics {
            system: SystemMetrics {
                cpu_usage: self.get_gauge_value(&metrics, "cpu_usage", 0.0),
                memory_usage: self.get_counter_value(&metrics, "memory_usage", 0),
                thread_count: self.get_counter_value(&metrics, "thread_count", 0) as usize,
                uptime: self.get_counter_value(&metrics, "uptime", 0),
                active_connections: self.get_counter_value(&metrics, "active_connections", 0),
            },
            cache: CacheMetrics {
                hit_rate: self.get_gauge_value(&metrics, "cache_hit_rate", 0.0),
                miss_rate: self.get_gauge_value(&metrics, "cache_miss_rate", 0.0),
                avg_access_time: Duration::from_micros(self.get_counter_value(&metrics, "avg_cache_access_time", 0)),
                cache_size: self.get_counter_value(&metrics, "cache_size", 0),
                evictions: *counters.get("cache_evictions").unwrap_or(&0),
                l1_hit_rate: self.get_gauge_value(&metrics, "l1_hit_rate", 0.0),
                l2_hit_rate: self.get_gauge_value(&metrics, "l2_hit_rate", 0.0),
                l3_hit_rate: self.get_gauge_value(&metrics, "l3_hit_rate", 0.0),
            },
            aggregation: AggregationMetrics {
                total_aggregations: *counters.get("total_aggregations").unwrap_or(&0),
                avg_aggregation_time: Duration::from_micros(self.get_counter_value(&metrics, "avg_aggregation_time", 0)),
                active_actors: self.get_counter_value(&metrics, "active_actors", 0),
                active_subsystems: self.get_counter_value(&metrics, "active_subsystems", 0),
                avg_contributions: self.get_gauge_value(&metrics, "avg_contributions", 0.0),
                avg_caps: self.get_gauge_value(&metrics, "avg_caps", 0.0),
            },
            memory: MemoryMetrics {
                total_memory: self.get_counter_value(&metrics, "total_memory", 0),
                pool_memory: self.get_counter_value(&metrics, "pool_memory", 0),
                cache_memory: self.get_counter_value(&metrics, "cache_memory", 0),
                allocated_memory: self.get_counter_value(&metrics, "allocated_memory", 0),
                peak_memory: self.get_counter_value(&metrics, "peak_memory", 0),
                fragmentation: self.get_gauge_value(&metrics, "fragmentation", 0.0),
            },
            errors: ErrorMetrics {
                total_errors: *counters.get("total_errors").unwrap_or(&0),
                error_rate: self.get_gauge_value(&metrics, "error_rate", 0.0),
                cache_errors: *counters.get("cache_errors").unwrap_or(&0),
                aggregation_errors: *counters.get("aggregation_errors").unwrap_or(&0),
                memory_errors: *counters.get("memory_errors").unwrap_or(&0),
                validation_errors: *counters.get("validation_errors").unwrap_or(&0),
            },
        }
    }

    /// Get time series data for a specific metric.
    pub fn get_time_series(&self, metric: &str, start_time: Option<u64>, end_time: Option<u64>) -> Vec<TimeSeriesPoint> {
        let time_series = self.time_series.read().unwrap();
        
        time_series
            .iter()
            .filter(|point| {
                point.metric == metric
                    && start_time.map_or(true, |start| point.timestamp >= start)
                    && end_time.map_or(true, |end| point.timestamp <= end)
            })
            .cloned()
            .collect()
    }

    /// Get counter value.
    pub fn get_counter(&self, name: &str) -> u64 {
        let counters = self.counters.read().unwrap();
        counters.get(name).copied().unwrap_or(0)
    }

    /// Reset all metrics.
    pub fn reset_metrics(&self) {
        let mut metrics = self.metrics.write().unwrap();
        metrics.clear();
        
        let mut counters = self.counters.write().unwrap();
        counters.clear();
        
        let mut time_series = self.time_series.write().unwrap();
        time_series.clear();
    }

    /// Generate analytics report.
    pub fn generate_report(&self) -> String {
        let metrics = self.get_performance_metrics();
        
        format!(
            r#"
# Actor Core Analytics Report

## System Performance
- **CPU Usage**: {:.1}%
- **Memory Usage**: {} bytes
- **Thread Count**: {}
- **Uptime**: {} seconds
- **Active Connections**: {}

## Cache Performance
- **Hit Rate**: {:.2}%
- **Miss Rate**: {:.2}%
- **Average Access Time**: {:?}
- **Cache Size**: {} bytes
- **Evictions**: {}
- **L1 Hit Rate**: {:.2}%
- **L2 Hit Rate**: {:.2}%
- **L3 Hit Rate**: {:.2}%

## Aggregation Performance
- **Total Aggregations**: {}
- **Average Aggregation Time**: {:?}
- **Active Actors**: {}
- **Active Subsystems**: {}
- **Average Contributions**: {:.1}
- **Average Caps**: {:.1}

## Memory Usage
- **Total Memory**: {} bytes
- **Pool Memory**: {} bytes
- **Cache Memory**: {} bytes
- **Allocated Memory**: {} bytes
- **Peak Memory**: {} bytes
- **Fragmentation**: {:.1}%

## Error Statistics
- **Total Errors**: {}
- **Error Rate**: {:.2} errors/sec
- **Cache Errors**: {}
- **Aggregation Errors**: {}
- **Memory Errors**: {}
- **Validation Errors**: {}
"#,
            metrics.system.cpu_usage,
            metrics.system.memory_usage,
            metrics.system.thread_count,
            metrics.system.uptime,
            metrics.system.active_connections,
            metrics.cache.hit_rate * 100.0,
            metrics.cache.miss_rate * 100.0,
            metrics.cache.avg_access_time,
            metrics.cache.cache_size,
            metrics.cache.evictions,
            metrics.cache.l1_hit_rate * 100.0,
            metrics.cache.l2_hit_rate * 100.0,
            metrics.cache.l3_hit_rate * 100.0,
            metrics.aggregation.total_aggregations,
            metrics.aggregation.avg_aggregation_time,
            metrics.aggregation.active_actors,
            metrics.aggregation.active_subsystems,
            metrics.aggregation.avg_contributions,
            metrics.aggregation.avg_caps,
            metrics.memory.total_memory,
            metrics.memory.pool_memory,
            metrics.memory.cache_memory,
            metrics.memory.allocated_memory,
            metrics.memory.peak_memory,
            metrics.memory.fragmentation,
            metrics.errors.total_errors,
            metrics.errors.error_rate,
            metrics.errors.cache_errors,
            metrics.errors.aggregation_errors,
            metrics.errors.memory_errors,
            metrics.errors.validation_errors,
        )
    }

    // Helper methods to get metric values with defaults
    fn get_gauge_value(&self, metrics: &HashMap<String, MetricValue>, key: &str, default: f64) -> f64 {
        match metrics.get(key) {
            Some(MetricValue::Gauge(v)) => *v,
            _ => default,
        }
    }

    fn get_counter_value(&self, metrics: &HashMap<String, MetricValue>, key: &str, default: u64) -> u64 {
        match metrics.get(key) {
            Some(MetricValue::Counter(v)) => *v,
            _ => default,
        }
    }
}

/// Real-time performance monitor.
pub struct PerformanceMonitor {
    collector: Arc<AnalyticsCollector>,
    #[allow(dead_code)]
    start_time: Instant,
}

impl PerformanceMonitor {
    /// Create a new performance monitor.
    pub fn new(collector: Arc<AnalyticsCollector>) -> Self {
        Self {
            collector,
            start_time: Instant::now(),
        }
    }

    /// Start monitoring a operation.
    pub fn start_operation(&self, operation: &str) -> OperationTimer {
        OperationTimer::new(operation.to_string(), self.collector.clone())
    }

    /// Record a cache operation.
    pub fn record_cache_operation(&self, operation: &str, hit: bool, latency: Duration) {
        self.collector.record_latency(&format!("cache_{}", operation), latency);
        
        if hit {
            self.collector.increment_counter("cache_hits", 1);
        } else {
            self.collector.increment_counter("cache_misses", 1);
        }
    }

    /// Record an aggregation operation.
    pub fn record_aggregation(&self, actor_count: usize, contribution_count: usize, latency: Duration) {
        self.collector.record_latency("aggregation", latency);
        self.collector.increment_counter("total_aggregations", 1);
        self.collector.record_metric("active_actors", MetricValue::Gauge(actor_count as f64));
        self.collector.record_metric("avg_contributions", MetricValue::Gauge(contribution_count as f64));
    }

    /// Record memory usage.
    pub fn record_memory_usage(&self, component: &str, usage: u64) {
        self.collector.record_memory_usage(component, usage);
    }

    /// Record an error.
    pub fn record_error(&self, error_type: &str) {
        self.collector.increment_counter("total_errors", 1);
        self.collector.increment_counter(&format!("{}_errors", error_type), 1);
    }
}

/// Timer for measuring operation duration.
pub struct OperationTimer {
    operation: String,
    collector: Arc<AnalyticsCollector>,
    start_time: Instant,
}

impl OperationTimer {
    fn new(operation: String, collector: Arc<AnalyticsCollector>) -> Self {
        Self {
            operation,
            collector,
            start_time: Instant::now(),
        }
    }

    /// Finish timing the operation.
    pub fn finish(self) {
        let duration = self.start_time.elapsed();
        self.collector.record_latency(&self.operation, duration);
    }
}

impl Drop for OperationTimer {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        self.collector.record_latency(&self.operation, duration);
    }
}