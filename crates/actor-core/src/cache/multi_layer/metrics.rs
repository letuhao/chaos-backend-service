//! Metrics and statistics for the multi-layer cache system.
//!
//! This module provides comprehensive metrics collection and statistics
//! for monitoring cache performance across all layers.

use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Custom serialization for Instant
fn serialize_instant<S>(_instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Convert to duration since UNIX_EPOCH for serialization
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    duration.serialize(serializer)
}

/// Custom deserialization for Instant
fn deserialize_instant<'de, D>(deserializer: D) -> Result<Instant, D::Error>
where
    D: Deserializer<'de>,
{
    let duration = Duration::deserialize(deserializer)?;
    let system_time = UNIX_EPOCH + duration;
    Ok(Instant::now() - system_time.elapsed().unwrap_or_default())
}

/// Statistics for the multi-layer cache system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiLayerStats {
    /// L1 cache statistics
    pub l1_stats: L1CacheStats,
    /// L2 cache statistics
    pub l2_stats: L2CacheStats,
    /// L3 cache statistics
    pub l3_stats: L3CacheStats,
    /// Total operations across all layers
    pub total_operations: u64,
    /// Total cache hits across all layers
    pub total_hits: u64,
    /// Total cache misses across all layers
    pub total_misses: u64,
    /// Total memory usage across all layers
    pub total_memory_usage: u64,
    /// Average response time across all layers
    pub avg_response_time: Duration,
    /// Maximum response time across all layers
    pub max_response_time: Duration,
    /// Cache hit ratio (hits / total operations)
    pub hit_ratio: f64,
    /// Last updated timestamp
    #[serde(serialize_with = "serialize_instant", deserialize_with = "deserialize_instant")]
    pub last_updated: Instant,
}

impl MultiLayerStats {
    /// Create new multi-layer statistics.
    pub fn new() -> Self {
        Self {
            l1_stats: L1CacheStats::new(),
            l2_stats: L2CacheStats::new(),
            l3_stats: L3CacheStats::new(),
            total_operations: 0,
            total_hits: 0,
            total_misses: 0,
            total_memory_usage: 0,
            avg_response_time: Duration::from_micros(0),
            max_response_time: Duration::from_micros(0),
            hit_ratio: 0.0,
            last_updated: Instant::now(),
        }
    }

    /// Update statistics with new operation data.
    pub fn update_operation(&mut self, layer: CacheLayer, hit: bool, response_time: Duration) {
        self.total_operations += 1;
        if hit {
            self.total_hits += 1;
        } else {
            self.total_misses += 1;
        }

        // Update layer-specific stats
        match layer {
            CacheLayer::L1 => {
                self.l1_stats.update_operation(hit, response_time);
            }
            CacheLayer::L2 => {
                self.l2_stats.update_operation(hit, response_time);
            }
            CacheLayer::L3 => {
                self.l3_stats.update_operation(hit, response_time);
            }
        }

        // Update global stats
        self.update_global_stats();
    }

    /// Update global statistics.
    fn update_global_stats(&mut self) {
        self.total_memory_usage = self.l1_stats.memory_usage
            + self.l2_stats.memory_usage
            + self.l3_stats.disk_usage;

        if self.total_operations > 0 {
            self.hit_ratio = self.total_hits as f64 / self.total_operations as f64;
        }

        // Update average response time
        let total_response_time = self.l1_stats.avg_response_time
            + self.l2_stats.avg_response_time
            + self.l3_stats.avg_response_time;
        self.avg_response_time = total_response_time / 3;

        // Update max response time
        self.max_response_time = self.l1_stats.max_response_time
            .max(self.l2_stats.max_response_time)
            .max(self.l3_stats.max_response_time);

        self.last_updated = Instant::now();
    }

    /// Reset all statistics.
    pub fn reset(&mut self) {
        self.l1_stats.reset();
        self.l2_stats.reset();
        self.l3_stats.reset();
        self.total_operations = 0;
        self.total_hits = 0;
        self.total_misses = 0;
        self.total_memory_usage = 0;
        self.avg_response_time = Duration::from_micros(0);
        self.max_response_time = Duration::from_micros(0);
        self.hit_ratio = 0.0;
        self.last_updated = Instant::now();
    }

    /// Get cache efficiency score (0.0 to 1.0).
    pub fn efficiency_score(&self) -> f64 {
        let hit_ratio_weight = 0.4;
        let response_time_weight = 0.3;
        let memory_efficiency_weight = 0.3;

        let hit_ratio_score = self.hit_ratio;
        let response_time_score = 1.0 - (self.avg_response_time.as_micros() as f64 / 1000.0).min(1.0);
        let memory_efficiency_score = if self.total_memory_usage > 0 {
            (self.l1_stats.memory_usage as f64 / self.total_memory_usage as f64).min(1.0)
        } else {
            1.0
        };

        hit_ratio_score * hit_ratio_weight
            + response_time_score * response_time_weight
            + memory_efficiency_score * memory_efficiency_weight
    }
}

impl Default for MultiLayerStats {
    fn default() -> Self {
        Self::new()
    }
}

/// L1 cache statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1CacheStats {
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
    /// Average response time
    pub avg_response_time: Duration,
    /// Maximum response time
    pub max_response_time: Duration,
    /// Number of evictions
    pub evictions: u64,
}

impl L1CacheStats {
    /// Create new L1 cache statistics.
    pub fn new() -> Self {
        Self {
            hits: 0,
            misses: 0,
            sets: 0,
            deletes: 0,
            memory_usage: 0,
            max_memory_usage: 0,
            avg_response_time: Duration::from_micros(0),
            max_response_time: Duration::from_micros(0),
            evictions: 0,
        }
    }

    /// Update statistics with new operation data.
    pub fn update_operation(&mut self, hit: bool, response_time: Duration) {
        if hit {
            self.hits += 1;
        } else {
            self.misses += 1;
        }

        // Update response time statistics
        let total_operations = self.hits + self.misses;
        if total_operations > 0 {
            self.avg_response_time = Duration::from_micros(
                (self.avg_response_time.as_micros() as u64 * (total_operations - 1)
                    + response_time.as_micros() as u64)
                    / total_operations,
            );
        }
        self.max_response_time = self.max_response_time.max(response_time);
    }

    /// Record a cache set operation.
    pub fn record_set(&mut self) {
        self.sets += 1;
    }

    /// Record a cache delete operation.
    pub fn record_delete(&mut self) {
        self.deletes += 1;
    }

    /// Record an eviction.
    pub fn record_eviction(&mut self) {
        self.evictions += 1;
    }

    /// Update memory usage.
    pub fn update_memory_usage(&mut self, usage: u64) {
        self.memory_usage = usage;
        self.max_memory_usage = self.max_memory_usage.max(usage);
    }

    /// Reset statistics.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for L1CacheStats {
    fn default() -> Self {
        Self::new()
    }
}

/// L2 cache statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2CacheStats {
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
    /// Average response time
    pub avg_response_time: Duration,
    /// Maximum response time
    pub max_response_time: Duration,
    /// Number of file operations
    pub file_operations: u64,
}

impl L2CacheStats {
    /// Create new L2 cache statistics.
    pub fn new() -> Self {
        Self {
            hits: 0,
            misses: 0,
            sets: 0,
            deletes: 0,
            memory_usage: 0,
            max_memory_usage: 0,
            avg_response_time: Duration::from_micros(0),
            max_response_time: Duration::from_micros(0),
            file_operations: 0,
        }
    }

    /// Update statistics with new operation data.
    pub fn update_operation(&mut self, hit: bool, response_time: Duration) {
        if hit {
            self.hits += 1;
        } else {
            self.misses += 1;
        }

        // Update response time statistics
        let total_operations = self.hits + self.misses;
        if total_operations > 0 {
            self.avg_response_time = Duration::from_micros(
                (self.avg_response_time.as_micros() as u64 * (total_operations - 1)
                    + response_time.as_micros() as u64)
                    / total_operations,
            );
        }
        self.max_response_time = self.max_response_time.max(response_time);
    }

    /// Record a cache set operation.
    pub fn record_set(&mut self) {
        self.sets += 1;
    }

    /// Record a cache delete operation.
    pub fn record_delete(&mut self) {
        self.deletes += 1;
    }

    /// Record a file operation.
    pub fn record_file_operation(&mut self) {
        self.file_operations += 1;
    }

    /// Update memory usage.
    pub fn update_memory_usage(&mut self, usage: u64) {
        self.memory_usage = usage;
        self.max_memory_usage = self.max_memory_usage.max(usage);
    }

    /// Reset statistics.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for L2CacheStats {
    fn default() -> Self {
        Self::new()
    }
}

/// L3 cache statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L3CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of cache sets
    pub sets: u64,
    /// Number of cache deletes
    pub deletes: u64,
    /// Current disk usage in bytes
    pub disk_usage: u64,
    /// Maximum disk usage in bytes
    pub max_disk_usage: u64,
    /// Average response time
    pub avg_response_time: Duration,
    /// Maximum response time
    pub max_response_time: Duration,
    /// Number of disk operations
    pub disk_operations: u64,
}

impl L3CacheStats {
    /// Create new L3 cache statistics.
    pub fn new() -> Self {
        Self {
            hits: 0,
            misses: 0,
            sets: 0,
            deletes: 0,
            disk_usage: 0,
            max_disk_usage: 0,
            avg_response_time: Duration::from_micros(0),
            max_response_time: Duration::from_micros(0),
            disk_operations: 0,
        }
    }

    /// Update statistics with new operation data.
    pub fn update_operation(&mut self, hit: bool, response_time: Duration) {
        if hit {
            self.hits += 1;
        } else {
            self.misses += 1;
        }

        // Update response time statistics
        let total_operations = self.hits + self.misses;
        if total_operations > 0 {
            self.avg_response_time = Duration::from_micros(
                (self.avg_response_time.as_micros() as u64 * (total_operations - 1)
                    + response_time.as_micros() as u64)
                    / total_operations,
            );
        }
        self.max_response_time = self.max_response_time.max(response_time);
    }

    /// Record a cache set operation.
    pub fn record_set(&mut self) {
        self.sets += 1;
    }

    /// Record a cache delete operation.
    pub fn record_delete(&mut self) {
        self.deletes += 1;
    }

    /// Record a disk operation.
    pub fn record_disk_operation(&mut self) {
        self.disk_operations += 1;
    }

    /// Update disk usage.
    pub fn update_disk_usage(&mut self, usage: u64) {
        self.disk_usage = usage;
        self.max_disk_usage = self.max_disk_usage.max(usage);
    }

    /// Reset statistics.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for L3CacheStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache layer enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CacheLayer {
    /// L1 cache (lock-free in-memory)
    L1,
    /// L2 cache (memory-mapped file)
    L2,
    /// L3 cache (persistent disk)
    L3,
}

impl CacheLayer {
    /// Get the display name of this cache layer.
    pub fn display_name(&self) -> &'static str {
        match self {
            CacheLayer::L1 => "L1",
            CacheLayer::L2 => "L2",
            CacheLayer::L3 => "L3",
        }
    }

    /// Get the description of this cache layer.
    pub fn description(&self) -> &'static str {
        match self {
            CacheLayer::L1 => "Lock-free in-memory cache (fastest access)",
            CacheLayer::L2 => "Memory-mapped file cache (persistent, medium speed)",
            CacheLayer::L3 => "Persistent disk cache (slowest, largest capacity)",
        }
    }
}

/// Statistics for cache warming operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheWarmingStats {
    /// Total warming operations performed
    pub total_warming_ops: u64,
    /// Total warming operations that succeeded
    pub successful_warming_ops: u64,
    /// Total warming operations that failed
    pub failed_warming_ops: u64,
    /// Total items warmed
    pub total_items_warmed: u64,
    /// Total warming time
    pub total_warming_time: Duration,
    /// Average warming time per operation
    pub avg_warming_time: Duration,
    /// Last warming operation timestamp
    #[serde(serialize_with = "serialize_instant", deserialize_with = "deserialize_instant")]
    pub last_warming_time: Instant,
}

impl CacheWarmingStats {
    /// Create new cache warming statistics.
    pub fn new() -> Self {
        Self {
            total_warming_ops: 0,
            successful_warming_ops: 0,
            failed_warming_ops: 0,
            total_items_warmed: 0,
            total_warming_time: Duration::ZERO,
            avg_warming_time: Duration::ZERO,
            last_warming_time: Instant::now(),
        }
    }

    /// Record a successful warming operation.
    pub fn record_success(&mut self, items_warmed: u64, duration: Duration) {
        self.total_warming_ops += 1;
        self.successful_warming_ops += 1;
        self.total_items_warmed += items_warmed;
        self.total_warming_time += duration;
        self.last_warming_time = Instant::now();
        self.update_avg_warming_time();
    }

    /// Record a failed warming operation.
    pub fn record_failure(&mut self, duration: Duration) {
        self.total_warming_ops += 1;
        self.failed_warming_ops += 1;
        self.total_warming_time += duration;
        self.last_warming_time = Instant::now();
        self.update_avg_warming_time();
    }

    /// Update the average warming time.
    fn update_avg_warming_time(&mut self) {
        if self.total_warming_ops > 0 {
            let total_nanos = self.total_warming_time.as_nanos() as u64;
            let avg_nanos = total_nanos / self.total_warming_ops;
            self.avg_warming_time = Duration::from_nanos(avg_nanos);
        }
    }

    /// Get the success rate as a percentage.
    pub fn success_rate(&self) -> f64 {
        if self.total_warming_ops == 0 {
            0.0
        } else {
            (self.successful_warming_ops as f64 / self.total_warming_ops as f64) * 100.0
        }
    }

    /// Get the failure rate as a percentage.
    pub fn failure_rate(&self) -> f64 {
        if self.total_warming_ops == 0 {
            0.0
        } else {
            (self.failed_warming_ops as f64 / self.total_warming_ops as f64) * 100.0
        }
    }

    /// Record a warming operation with success and error counts.
    pub fn record_warming_operation(&mut self, success_count: u64, error_count: u64, duration: Duration) {
        self.total_warming_ops += 1;
        if error_count == 0 {
            self.successful_warming_ops += 1;
            self.total_items_warmed += success_count;
        } else {
            self.failed_warming_ops += 1;
        }
        self.total_warming_time += duration;
        self.last_warming_time = Instant::now();
        self.update_avg_warming_time();
    }

    /// Record a scheduled warming operation.
    pub fn record_scheduled_warming(&mut self, duration: Duration) {
        self.total_warming_ops += 1;
        self.successful_warming_ops += 1;
        self.total_warming_time += duration;
        self.last_warming_time = Instant::now();
        self.update_avg_warming_time();
    }
}

impl Default for CacheWarmingStats {
    fn default() -> Self {
        Self::new()
    }
}