//! # Registry Configuration
//! 
//! This module defines the configuration structure for the unified element registry.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Registry configuration
/// 
/// This struct contains all configuration options for the unified element registry,
/// including performance settings, caching options, and feature flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Maximum number of elements supported
    pub max_elements: usize,
    
    /// Maximum number of contributors
    pub max_contributors: usize,
    
    /// Maximum number of plugins
    pub max_plugins: usize,
    
    /// Cache configuration
    pub cache_config: CacheConfig,
    
    /// Performance configuration
    pub performance_config: PerformanceConfig,
    
    /// Feature flags
    pub feature_flags: HashMap<String, bool>,
    
    /// Validation settings
    pub validation_config: ValidationConfig,
    
    /// Logging configuration
    pub logging_config: LoggingConfig,
    
    /// Registry version
    pub version: String,
    
    /// Registry name
    pub name: String,
    
    /// Registry description
    pub description: String,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    
    /// Cache size limit
    pub size_limit: usize,
    
    /// Cache TTL in seconds
    pub ttl_seconds: u64,
    
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
    
    /// Enable cache metrics
    pub enable_metrics: bool,
    
    /// Cache hit rate threshold
    pub hit_rate_threshold: f64,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    
    /// Performance sampling rate
    pub sampling_rate: f64,
    
    /// Maximum response time in milliseconds
    pub max_response_time_ms: f64,
    
    /// Enable parallel processing
    pub enable_parallel_processing: bool,
    
    /// Number of worker threads
    pub worker_threads: usize,
    
    /// Enable SIMD operations
    pub enable_simd: bool,
    
    /// Batch size for operations
    pub batch_size: usize,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Enable validation
    pub enabled: bool,
    
    /// Validate on registration
    pub validate_on_registration: bool,
    
    /// Validate on access
    pub validate_on_access: bool,
    
    /// Strict validation mode
    pub strict_mode: bool,
    
    /// Validation timeout in milliseconds
    pub timeout_ms: u64,
    
    /// Enable validation caching
    pub enable_validation_cache: bool,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Enable logging
    pub enabled: bool,
    
    /// Log level
    pub log_level: LogLevel,
    
    /// Log format
    pub log_format: LogFormat,
    
    /// Enable structured logging
    pub enable_structured_logging: bool,
    
    /// Log file path
    pub log_file_path: Option<String>,
    
    /// Enable console logging
    pub enable_console_logging: bool,
    
    /// Enable performance logging
    pub enable_performance_logging: bool,
}

/// Cache eviction policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvictionPolicy {
    /// Least Recently Used
    LRU,
    
    /// Least Frequently Used
    LFU,
    
    /// First In First Out
    FIFO,
    
    /// Random eviction
    Random,
    
    /// Time-based eviction
    TTL,
}

/// Log levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    /// Error level
    Error,
    
    /// Warning level
    Warning,
    
    /// Info level
    Info,
    
    /// Debug level
    Debug,
    
    /// Trace level
    Trace,
}

/// Log formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    /// Plain text format
    Plain,
    
    /// JSON format
    Json,
    
    /// Compact format
    Compact,
    
    /// Pretty format
    Pretty,
}

impl RegistryConfig {
    /// Create default registry configuration
    pub fn default() -> Self {
        Self {
            max_elements: 50,
            max_contributors: 100,
            max_plugins: 50,
            cache_config: CacheConfig::default(),
            performance_config: PerformanceConfig::default(),
            feature_flags: HashMap::new(),
            validation_config: ValidationConfig::default(),
            logging_config: LoggingConfig::default(),
            version: "1.0.0".to_string(),
            name: "UnifiedElementRegistry".to_string(),
            description: "Unified Element Registry for Element-Core".to_string(),
        }
    }
    
    /// Create custom registry configuration
    pub fn new(
        max_elements: usize,
        max_contributors: usize,
        max_plugins: usize,
    ) -> Self {
        Self {
            max_elements,
            max_contributors,
            max_plugins,
            cache_config: CacheConfig::default(),
            performance_config: PerformanceConfig::default(),
            feature_flags: HashMap::new(),
            validation_config: ValidationConfig::default(),
            logging_config: LoggingConfig::default(),
            version: "1.0.0".to_string(),
            name: "UnifiedElementRegistry".to_string(),
            description: "Unified Element Registry for Element-Core".to_string(),
        }
    }
    
    /// Set feature flag
    pub fn set_feature_flag(&mut self, feature: String, enabled: bool) {
        self.feature_flags.insert(feature, enabled);
    }
    
    /// Get feature flag
    pub fn get_feature_flag(&self, feature: &str) -> bool {
        self.feature_flags.get(feature).copied().unwrap_or(false)
    }
    
    /// Check if feature is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.get_feature_flag(feature)
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.max_elements == 0 {
            return Err("Max elements must be greater than 0".to_string());
        }
        
        if self.max_contributors == 0 {
            return Err("Max contributors must be greater than 0".to_string());
        }
        
        if self.max_plugins == 0 {
            return Err("Max plugins must be greater than 0".to_string());
        }
        
        if self.version.is_empty() {
            return Err("Version cannot be empty".to_string());
        }
        
        if self.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        // Validate cache config
        self.cache_config.validate()?;
        
        // Validate performance config
        self.performance_config.validate()?;
        
        // Validate validation config
        self.validation_config.validate()?;
        
        // Validate logging config
        self.logging_config.validate()?;
        
        Ok(())
    }
}

impl CacheConfig {
    /// Create default cache configuration
    pub fn default() -> Self {
        Self {
            enabled: true,
            size_limit: 1000,
            ttl_seconds: 3600, // 1 hour
            eviction_policy: EvictionPolicy::LRU,
            enable_metrics: true,
            hit_rate_threshold: 0.8,
        }
    }
    
    /// Validate cache configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.size_limit == 0 {
            return Err("Cache size limit must be greater than 0".to_string());
        }
        
        if self.ttl_seconds == 0 {
            return Err("Cache TTL must be greater than 0".to_string());
        }
        
        if self.hit_rate_threshold < 0.0 || self.hit_rate_threshold > 1.0 {
            return Err("Hit rate threshold must be between 0.0 and 1.0".to_string());
        }
        
        Ok(())
    }
}

impl PerformanceConfig {
    /// Create default performance configuration
    pub fn default() -> Self {
        Self {
            enable_monitoring: true,
            sampling_rate: 0.1, // 10% sampling
            max_response_time_ms: 1000.0,
            enable_parallel_processing: true,
            worker_threads: 4, // Default to 4 threads
            enable_simd: true,
            batch_size: 100,
        }
    }
    
    /// Validate performance configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.sampling_rate < 0.0 || self.sampling_rate > 1.0 {
            return Err("Sampling rate must be between 0.0 and 1.0".to_string());
        }
        
        if self.max_response_time_ms <= 0.0 {
            return Err("Max response time must be greater than 0".to_string());
        }
        
        if self.worker_threads == 0 {
            return Err("Worker threads must be greater than 0".to_string());
        }
        
        if self.batch_size == 0 {
            return Err("Batch size must be greater than 0".to_string());
        }
        
        Ok(())
    }
}

impl ValidationConfig {
    /// Create default validation configuration
    pub fn default() -> Self {
        Self {
            enabled: true,
            validate_on_registration: true,
            validate_on_access: false,
            strict_mode: false,
            timeout_ms: 5000, // 5 seconds
            enable_validation_cache: true,
        }
    }
    
    /// Validate validation configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.timeout_ms == 0 {
            return Err("Validation timeout must be greater than 0".to_string());
        }
        
        Ok(())
    }
}

impl LoggingConfig {
    /// Create default logging configuration
    pub fn default() -> Self {
        Self {
            enabled: true,
            log_level: LogLevel::Info,
            log_format: LogFormat::Plain,
            enable_structured_logging: false,
            log_file_path: None,
            enable_console_logging: true,
            enable_performance_logging: false,
        }
    }
    
    /// Validate logging configuration
    pub fn validate(&self) -> Result<(), String> {
        // No validation needed for logging config
        Ok(())
    }
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for EvictionPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvictionPolicy::LRU => write!(f, "LRU"),
            EvictionPolicy::LFU => write!(f, "LFU"),
            EvictionPolicy::FIFO => write!(f, "FIFO"),
            EvictionPolicy::Random => write!(f, "Random"),
            EvictionPolicy::TTL => write!(f, "TTL"),
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warning => write!(f, "WARN"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Trace => write!(f, "TRACE"),
        }
    }
}

impl std::fmt::Display for LogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogFormat::Plain => write!(f, "Plain"),
            LogFormat::Json => write!(f, "JSON"),
            LogFormat::Compact => write!(f, "Compact"),
            LogFormat::Pretty => write!(f, "Pretty"),
        }
    }
}
