//! Performance configuration for Actor Core.
//!
//! This module provides centralized configuration for all performance-related
//! settings, thresholds, and benchmarks.

use std::time::Duration;
use serde::{Deserialize, Serialize};
// use crate::config::manager::ConfigurationManager; // Unused import

/// Centralized performance configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// General performance settings
    pub general: GeneralPerformanceConfig,
    /// Aggregation performance settings
    pub aggregation: AggregationPerformanceConfig,
    /// Cache performance settings
    pub cache: CachePerformanceConfig,
    /// Memory performance settings
    pub memory: MemoryPerformanceConfig,
    /// System performance settings
    pub system: SystemPerformanceConfig,
    /// Monitoring settings
    pub monitoring: MonitoringConfig,
    /// Benchmarking settings
    pub benchmarking: BenchmarkingConfig,
    /// Alerting settings
    pub alerting: AlertingConfig,
}

/// General performance configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralPerformanceConfig {
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Enable performance profiling
    pub enable_profiling: bool,
    /// Enable performance benchmarking
    pub enable_benchmarking: bool,
    /// Performance monitoring interval
    pub monitoring_interval: Duration,
    /// Maximum performance degradation (percentage)
    pub max_degradation: f64,
    /// Performance optimization level (0-3)
    pub optimization_level: u8,
}

/// Aggregation performance configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationPerformanceConfig {
    /// Maximum aggregation time (microseconds)
    pub max_aggregation_time: u64,
    /// Maximum contributions per aggregation
    pub max_contributions_per_aggregation: usize,
    /// Maximum subsystems per aggregation
    pub max_subsystems_per_aggregation: usize,
    /// Minimum throughput (operations per second)
    pub min_throughput: u64,
    /// Maximum latency (microseconds)
    pub max_latency: u64,
    /// Enable parallel aggregation
    pub enable_parallel_aggregation: bool,
    /// Parallel aggregation threshold (number of contributions)
    pub parallel_threshold: usize,
    /// Enable aggregation caching
    pub enable_aggregation_caching: bool,
    /// Aggregation cache TTL (seconds)
    pub aggregation_cache_ttl: u64,
}

/// Cache performance configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePerformanceConfig {
    /// Maximum cache operation time (microseconds)
    pub max_operation_time: u64,
    /// Minimum cache hit rate (percentage)
    pub min_hit_rate: f64,
    /// Maximum cache size (bytes)
    pub max_cache_size: u64,
    /// Cache eviction threshold (percentage)
    pub eviction_threshold: f64,
    /// Enable cache warming
    pub enable_cache_warming: bool,
    /// Cache warming interval
    pub warming_interval: Duration,
    /// Enable cache compression
    pub enable_compression: bool,
    /// Compression threshold (bytes)
    pub compression_threshold: usize,
}

/// Memory performance configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPerformanceConfig {
    /// Maximum memory usage per actor (bytes)
    pub max_memory_per_actor: u64,
    /// Maximum total memory usage (bytes)
    pub max_total_memory: u64,
    /// Memory usage warning threshold (percentage)
    pub warning_threshold: f64,
    /// Memory usage critical threshold (percentage)
    pub critical_threshold: f64,
    /// Enable memory pooling
    pub enable_memory_pooling: bool,
    /// Memory pool size (bytes)
    pub memory_pool_size: u64,
    /// Enable memory compaction
    pub enable_memory_compaction: bool,
    /// Compaction threshold (percentage)
    pub compaction_threshold: f64,
}

/// System performance configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPerformanceConfig {
    /// Maximum CPU usage (percentage)
    pub max_cpu_usage: f64,
    /// CPU usage warning threshold (percentage)
    pub cpu_warning_threshold: f64,
    /// Maximum thread count
    pub max_thread_count: usize,
    /// Thread pool size
    pub thread_pool_size: usize,
    /// Enable CPU affinity
    pub enable_cpu_affinity: bool,
    /// Enable NUMA awareness
    pub enable_numa_awareness: bool,
    /// System load warning threshold
    pub load_warning_threshold: f64,
    /// System load critical threshold
    pub load_critical_threshold: f64,
}

/// Monitoring configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable real-time monitoring
    pub enable_realtime_monitoring: bool,
    /// Enable historical monitoring
    pub enable_historical_monitoring: bool,
    /// Monitoring data retention (days)
    pub data_retention_days: u32,
    /// Enable performance metrics export
    pub enable_metrics_export: bool,
    /// Metrics export interval
    pub export_interval: Duration,
    /// Enable performance dashboards
    pub enable_dashboards: bool,
    /// Dashboard refresh interval
    pub dashboard_refresh_interval: Duration,
}

/// Benchmarking configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkingConfig {
    /// Enable automated benchmarking
    pub enable_automated_benchmarking: bool,
    /// Benchmarking interval
    pub benchmarking_interval: Duration,
    /// Benchmark duration
    pub benchmark_duration: Duration,
    /// Number of benchmark iterations
    pub benchmark_iterations: usize,
    /// Enable stress testing
    pub enable_stress_testing: bool,
    /// Stress test duration
    pub stress_test_duration: Duration,
    /// Stress test load multiplier
    pub stress_load_multiplier: f64,
    /// Enable regression testing
    pub enable_regression_testing: bool,
    /// Regression test threshold (percentage)
    pub regression_threshold: f64,
}

/// Alerting configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Enable performance alerts
    pub enable_alerts: bool,
    /// Alert severity levels
    pub severity_levels: SeverityLevels,
    /// Alert channels
    pub channels: AlertChannels,
    /// Alert throttling
    pub throttling: AlertThrottling,
    /// Alert escalation
    pub escalation: AlertEscalation,
}

/// Alert severity levels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityLevels {
    /// Warning level threshold (performance score)
    pub warning_threshold: f64,
    /// Critical level threshold (performance score)
    pub critical_threshold: f64,
    /// Fatal level threshold (performance score)
    pub fatal_threshold: f64,
}

/// Alert channels configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannels {
    /// Enable console alerts
    pub enable_console: bool,
    /// Enable file alerts
    pub enable_file: bool,
    /// Alert file path
    pub file_path: String,
    /// Enable email alerts
    pub enable_email: bool,
    /// Email recipients
    pub email_recipients: Vec<String>,
    /// Enable webhook alerts
    pub enable_webhook: bool,
    /// Webhook URL
    pub webhook_url: String,
}

/// Alert throttling configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThrottling {
    /// Enable alert throttling
    pub enable_throttling: bool,
    /// Maximum alerts per minute
    pub max_alerts_per_minute: u32,
    /// Alert cooldown period
    pub cooldown_period: Duration,
    /// Enable alert grouping
    pub enable_grouping: bool,
    /// Group timeout
    pub group_timeout: Duration,
}

/// Alert escalation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEscalation {
    /// Enable alert escalation
    pub enable_escalation: bool,
    /// Escalation delay
    pub escalation_delay: Duration,
    /// Maximum escalation levels
    pub max_escalation_levels: u8,
    /// Escalation recipients per level
    pub escalation_recipients: Vec<Vec<String>>,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            general: GeneralPerformanceConfig::default(),
            aggregation: AggregationPerformanceConfig::default(),
            cache: CachePerformanceConfig::default(),
            memory: MemoryPerformanceConfig::default(),
            system: SystemPerformanceConfig::default(),
            monitoring: MonitoringConfig::default(),
            benchmarking: BenchmarkingConfig::default(),
            alerting: AlertingConfig::default(),
        }
    }
}

impl Default for GeneralPerformanceConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            enable_profiling: true,
            enable_benchmarking: true,
            monitoring_interval: Duration::from_secs(5),
            max_degradation: 10.0,
            optimization_level: 2,
        }
    }
}

impl Default for AggregationPerformanceConfig {
    fn default() -> Self {
        Self {
            max_aggregation_time: 100, // Default 100ms - loaded from config at runtime
            max_contributions_per_aggregation: 1000,
            max_subsystems_per_aggregation: 100,
            min_throughput: 1000,
            max_latency: 10_000, // 10ms
            enable_parallel_aggregation: true,
            parallel_threshold: 50,
            enable_aggregation_caching: true,
            aggregation_cache_ttl: 300, // 5 minutes
        }
    }
}

impl Default for CachePerformanceConfig {
    fn default() -> Self {
        Self {
            max_operation_time: 1000, // Default 1s - loaded from config at runtime
            min_hit_rate: 90.0,
            max_cache_size: 100, // Default 100MB - loaded from config at runtime
            eviction_threshold: 80.0,
            enable_cache_warming: true,
            warming_interval: Duration::from_secs(60),
            enable_compression: true,
            compression_threshold: 1024, // 1KB
        }
    }
}

impl Default for MemoryPerformanceConfig {
    fn default() -> Self {
        Self {
            max_memory_per_actor: 1024, // Default 1KB - loaded from config at runtime
            max_total_memory: 1024 * 1024 * 1024, // 1GB
            warning_threshold: 75.0,
            critical_threshold: 90.0,
            enable_memory_pooling: true,
            memory_pool_size: 100 * 1024 * 1024, // 100MB
            enable_memory_compaction: true,
            compaction_threshold: 70.0,
        }
    }
}

impl Default for SystemPerformanceConfig {
    fn default() -> Self {
        Self {
            max_cpu_usage: 80.0,
            cpu_warning_threshold: 70.0,
            max_thread_count: 32,
            thread_pool_size: 8,
            enable_cpu_affinity: false,
            enable_numa_awareness: false,
            load_warning_threshold: 2.0,
            load_critical_threshold: 4.0,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_realtime_monitoring: true,
            enable_historical_monitoring: true,
            data_retention_days: 30,
            enable_metrics_export: true,
            export_interval: Duration::from_secs(60),
            enable_dashboards: true,
            dashboard_refresh_interval: Duration::from_secs(5),
        }
    }
}

impl Default for BenchmarkingConfig {
    fn default() -> Self {
        Self {
            enable_automated_benchmarking: true,
            benchmarking_interval: Duration::from_secs(3600), // 1 hour
            benchmark_duration: Duration::from_secs(300), // 5 minutes
            benchmark_iterations: 10,
            enable_stress_testing: true,
            stress_test_duration: Duration::from_secs(600), // 10 minutes
            stress_load_multiplier: 2.0,
            enable_regression_testing: true,
            regression_threshold: 10.0,
        }
    }
}

impl Default for AlertingConfig {
    fn default() -> Self {
        Self {
            enable_alerts: true,
            severity_levels: SeverityLevels::default(),
            channels: AlertChannels::default(),
            throttling: AlertThrottling::default(),
            escalation: AlertEscalation::default(),
        }
    }
}

impl Default for SeverityLevels {
    fn default() -> Self {
        Self {
            warning_threshold: 80.0,
            critical_threshold: 60.0,
            fatal_threshold: 40.0,
        }
    }
}

impl Default for AlertChannels {
    fn default() -> Self {
        Self {
            enable_console: true,
            enable_file: true,
            file_path: "/tmp/actor-core-alerts.log".to_string(),
            enable_email: false,
            email_recipients: Vec::new(),
            enable_webhook: false,
            webhook_url: String::new(),
        }
    }
}

impl Default for AlertThrottling {
    fn default() -> Self {
        Self {
            enable_throttling: true,
            max_alerts_per_minute: 10,
            cooldown_period: Duration::from_secs(300), // 5 minutes
            enable_grouping: true,
            group_timeout: Duration::from_secs(60), // 1 minute
        }
    }
}

impl Default for AlertEscalation {
    fn default() -> Self {
        Self {
            enable_escalation: true,
            escalation_delay: Duration::from_secs(900), // 15 minutes
            max_escalation_levels: 3,
            escalation_recipients: vec![
                vec!["dev-team@example.com".to_string()],
                vec!["ops-team@example.com".to_string()],
                vec!["management@example.com".to_string()],
            ],
        }
    }
}

/// Performance configuration loader.
pub struct PerformanceConfigLoader;

impl PerformanceConfigLoader {
    /// Load performance configuration from file.
    pub fn load_from_file(path: &str) -> Result<PerformanceConfig, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: PerformanceConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Save performance configuration to file.
    pub fn save_to_file(config: &PerformanceConfig, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_yaml::to_string(config)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load performance configuration from environment variables.
    pub fn load_from_env() -> PerformanceConfig {
        let mut config = PerformanceConfig::default();

        // Load general settings
        if let Ok(enable_monitoring) = std::env::var("ACTOR_CORE_ENABLE_MONITORING") {
            config.general.enable_monitoring = enable_monitoring.parse().unwrap_or(true);
        }

        if let Ok(monitoring_interval) = std::env::var("ACTOR_CORE_MONITORING_INTERVAL") {
            if let Ok(seconds) = monitoring_interval.parse::<u64>() {
                config.general.monitoring_interval = Duration::from_secs(seconds);
            }
        }

        // Load aggregation settings
        if let Ok(max_aggregation_time) = std::env::var("ACTOR_CORE_MAX_AGGREGATION_TIME") {
            if let Ok(time) = max_aggregation_time.parse::<u64>() {
                config.aggregation.max_aggregation_time = time;
            }
        }

        if let Ok(min_throughput) = std::env::var("ACTOR_CORE_MIN_THROUGHPUT") {
            if let Ok(throughput) = min_throughput.parse::<u64>() {
                config.aggregation.min_throughput = throughput;
            }
        }

        // Load cache settings
        if let Ok(max_cache_time) = std::env::var("ACTOR_CORE_MAX_CACHE_TIME") {
            if let Ok(time) = max_cache_time.parse::<u64>() {
                config.cache.max_operation_time = time;
            }
        }

        if let Ok(min_hit_rate) = std::env::var("ACTOR_CORE_MIN_CACHE_HIT_RATE") {
            if let Ok(rate) = min_hit_rate.parse::<f64>() {
                config.cache.min_hit_rate = rate;
            }
        }

        // Load memory settings
        if let Ok(max_memory) = std::env::var("ACTOR_CORE_MAX_MEMORY_PER_ACTOR") {
            if let Ok(memory) = max_memory.parse::<u64>() {
                config.memory.max_memory_per_actor = memory;
            }
        }

        // Load system settings
        if let Ok(max_cpu) = std::env::var("ACTOR_CORE_MAX_CPU_USAGE") {
            if let Ok(cpu) = max_cpu.parse::<f64>() {
                config.system.max_cpu_usage = cpu;
            }
        }

        config
    }

    /// Validate performance configuration.
    pub fn validate(config: &PerformanceConfig) -> Result<(), String> {
        // Validate general settings
        if config.general.max_degradation < 0.0 || config.general.max_degradation > 100.0 {
            return Err("max_degradation must be between 0 and 100".to_string());
        }

        if config.general.optimization_level > 3 {
            return Err("optimization_level must be between 0 and 3".to_string());
        }

        // Validate aggregation settings
        if config.aggregation.max_contributions_per_aggregation == 0 {
            return Err("max_contributions_per_aggregation must be greater than 0".to_string());
        }

        if config.aggregation.max_subsystems_per_aggregation == 0 {
            return Err("max_subsystems_per_aggregation must be greater than 0".to_string());
        }

        if config.aggregation.min_throughput == 0 {
            return Err("min_throughput must be greater than 0".to_string());
        }

        // Validate cache settings
        if config.cache.min_hit_rate < 0.0 || config.cache.min_hit_rate > 100.0 {
            return Err("min_hit_rate must be between 0 and 100".to_string());
        }

        if config.cache.eviction_threshold < 0.0 || config.cache.eviction_threshold > 100.0 {
            return Err("eviction_threshold must be between 0 and 100".to_string());
        }

        // Validate memory settings
        if config.memory.warning_threshold < 0.0 || config.memory.warning_threshold > 100.0 {
            return Err("warning_threshold must be between 0 and 100".to_string());
        }

        if config.memory.critical_threshold < config.memory.warning_threshold {
            return Err("critical_threshold must be greater than or equal to warning_threshold".to_string());
        }

        // Validate system settings
        if config.system.max_cpu_usage < 0.0 || config.system.max_cpu_usage > 100.0 {
            return Err("max_cpu_usage must be between 0 and 100".to_string());
        }

        if config.system.cpu_warning_threshold > config.system.max_cpu_usage {
            return Err("cpu_warning_threshold must be less than or equal to max_cpu_usage".to_string());
        }

        // Validate alerting settings
        if config.alerting.severity_levels.warning_threshold < 0.0 || config.alerting.severity_levels.warning_threshold > 100.0 {
            return Err("warning_threshold must be between 0 and 100".to_string());
        }

        if config.alerting.severity_levels.critical_threshold > config.alerting.severity_levels.warning_threshold {
            return Err("critical_threshold must be less than or equal to warning_threshold".to_string());
        }

        if config.alerting.severity_levels.fatal_threshold > config.alerting.severity_levels.critical_threshold {
            return Err("fatal_threshold must be less than or equal to critical_threshold".to_string());
        }

        Ok(())
    }
}