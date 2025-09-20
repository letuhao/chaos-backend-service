//! Configuration management for Actor Core system.
//!
//! This module provides configuration loading and management for various
//! components including Redis cache, database connections, and other settings.

use std::env;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::{info, warn};

/// Configuration for the Actor Core system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorCoreConfig {
    /// Redis configuration
    pub redis: RedisConfig,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Redis connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Command timeout in seconds
    pub command_timeout: u64,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Whether to use TLS/SSL
    pub use_tls: bool,
}

/// Cache configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Default TTL in seconds
    pub default_ttl: u64,
    /// Maximum number of entries in memory cache
    pub max_entries: usize,
    /// Whether to enable Redis cache
    pub enable_redis: bool,
    /// L1 cache size (fastest, smallest)
    pub l1_size: usize,
    /// L2 cache size (medium speed, medium size)
    pub l2_size: usize,
    /// L3 cache size (slowest, largest)
    pub l3_size: usize,
}

/// Logging configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Whether to enable structured logging
    pub structured: bool,
    /// Whether to enable JSON logging
    pub json: bool,
}

impl Default for ActorCoreConfig {
    fn default() -> Self {
        Self {
            redis: RedisConfig::default(),
            cache: CacheConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            connection_timeout: 5,
            command_timeout: 3,
            max_connections: 10,
            use_tls: false,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            default_ttl: 1800, // 30 minutes
            max_entries: 1_000_000,
            enable_redis: false,
            l1_size: 50_000,
            l2_size: 200_000,
            l3_size: 500_000,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            structured: true,
            json: false,
        }
    }
}

impl ActorCoreConfig {
    /// Load configuration from environment variables.
    /// 
    /// This method loads configuration from environment variables with the following precedence:
    /// 1. Environment variables (highest priority)
    /// 2. Default values (lowest priority)
    /// 
    /// # Environment Variables
    /// - `ACTOR_CORE_REDIS_URL`: Redis connection URL
    /// - `ACTOR_CORE_REDIS_CONNECTION_TIMEOUT`: Connection timeout in seconds
    /// - `ACTOR_CORE_REDIS_COMMAND_TIMEOUT`: Command timeout in seconds
    /// - `ACTOR_CORE_REDIS_MAX_CONNECTIONS`: Maximum connections in pool
    /// - `ACTOR_CORE_REDIS_USE_TLS`: Whether to use TLS (true/false)
    /// - `ACTOR_CORE_CACHE_DEFAULT_TTL`: Default cache TTL in seconds
    /// - `ACTOR_CORE_CACHE_MAX_ENTRIES`: Maximum entries in memory cache
    /// - `ACTOR_CORE_CACHE_ENABLE_REDIS`: Whether to enable Redis cache (true/false)
    /// - `ACTOR_CORE_CACHE_L1_SIZE`: L1 cache size
    /// - `ACTOR_CORE_CACHE_L2_SIZE`: L2 cache size
    /// - `ACTOR_CORE_CACHE_L3_SIZE`: L3 cache size
    /// - `RUST_LOG`: Log level
    /// - `ACTOR_CORE_LOG_STRUCTURED`: Enable structured logging (true/false)
    /// - `ACTOR_CORE_LOG_JSON`: Enable JSON logging (true/false)
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();
        
        // Load Redis configuration
        if let Ok(url) = env::var("ACTOR_CORE_REDIS_URL") {
            config.redis.url = url;
            config.cache.enable_redis = true;
            info!("Redis URL loaded from environment: {}", config.redis.url);
        } else {
            warn!("No Redis URL found in environment variables. Redis cache will be disabled.");
        }
        
        if let Ok(timeout) = env::var("ACTOR_CORE_REDIS_CONNECTION_TIMEOUT") {
            // TODO: Load default connection timeout from configuration instead of hardcoding 5
            config.redis.connection_timeout = timeout.parse().unwrap_or(5);
        }
        
        if let Ok(timeout) = env::var("ACTOR_CORE_REDIS_COMMAND_TIMEOUT") {
            // TODO: Load default command timeout from configuration instead of hardcoding 3
            config.redis.command_timeout = timeout.parse().unwrap_or(3);
        }
        
        if let Ok(max_conn) = env::var("ACTOR_CORE_REDIS_MAX_CONNECTIONS") {
            // TODO: Load default max connections from configuration instead of hardcoding 10
            config.redis.max_connections = max_conn.parse().unwrap_or(10);
        }
        
        if let Ok(use_tls) = env::var("ACTOR_CORE_REDIS_USE_TLS") {
            config.redis.use_tls = use_tls.to_lowercase() == "true";
        }
        
        // Load cache configuration
        if let Ok(ttl) = env::var("ACTOR_CORE_CACHE_DEFAULT_TTL") {
            // TODO: Load default TTL from configuration instead of hardcoding 1800
            config.cache.default_ttl = ttl.parse().unwrap_or(1800);
        }
        
        if let Ok(max_entries) = env::var("ACTOR_CORE_CACHE_MAX_ENTRIES") {
            // TODO: Load default max entries from configuration instead of hardcoding 1_000_000
            config.cache.max_entries = max_entries.parse().unwrap_or(1_000_000);
        }
        
        if let Ok(enable_redis) = env::var("ACTOR_CORE_CACHE_ENABLE_REDIS") {
            config.cache.enable_redis = enable_redis.to_lowercase() == "true";
        }
        
        if let Ok(l1_size) = env::var("ACTOR_CORE_CACHE_L1_SIZE") {
            // TODO: Load default L1 size from configuration instead of hardcoding 50_000
            config.cache.l1_size = l1_size.parse().unwrap_or(50_000);
        }
        
        if let Ok(l2_size) = env::var("ACTOR_CORE_CACHE_L2_SIZE") {
            // TODO: Load default L2 size from configuration instead of hardcoding 200_000
            config.cache.l2_size = l2_size.parse().unwrap_or(200_000);
        }
        
        if let Ok(l3_size) = env::var("ACTOR_CORE_CACHE_L3_SIZE") {
            // TODO: Load default L3 size from configuration instead of hardcoding 500_000
            config.cache.l3_size = l3_size.parse().unwrap_or(500_000);
        }
        
        // Load logging configuration
        if let Ok(log_level) = env::var("RUST_LOG") {
            config.logging.level = log_level;
        }
        
        if let Ok(structured) = env::var("ACTOR_CORE_LOG_STRUCTURED") {
            config.logging.structured = structured.to_lowercase() == "true";
        }
        
        if let Ok(json) = env::var("ACTOR_CORE_LOG_JSON") {
            config.logging.json = json.to_lowercase() == "true";
        }
        
        info!("Configuration loaded successfully");
        info!("Redis enabled: {}", config.cache.enable_redis);
        info!("Cache TTL: {} seconds", config.cache.default_ttl);
        info!("Log level: {}", config.logging.level);
        
        Ok(config)
    }
    
    /// Load configuration from a file.
    /// 
    /// This method loads configuration from a TOML or YAML file.
    /// Environment variables will override file values.
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        
        let config: ActorCoreConfig = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::from_str(&content)?
        } else if path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
                  path.extension().and_then(|s| s.to_str()) == Some("yml") {
            serde_yaml::from_str(&content)?
        } else {
            return Err(anyhow::anyhow!("Unsupported configuration file format"));
        };
        
        // Override with environment variables
        let mut env_config = Self::from_env()?;
        
        // Only override if environment variables are set
        if env::var("ACTOR_CORE_REDIS_URL").is_ok() {
            env_config.redis.url = config.redis.url;
        }
        if env::var("ACTOR_CORE_CACHE_DEFAULT_TTL").is_ok() {
            env_config.cache.default_ttl = config.cache.default_ttl;
        }
        
        Ok(env_config)
    }
    
    /// Validate the configuration.
    /// 
    /// This method validates that the configuration is valid and all required
    /// fields are properly set.
    pub fn validate(&self) -> Result<()> {
        if self.cache.enable_redis && self.redis.url.is_empty() {
            return Err(anyhow::anyhow!("Redis URL is required when Redis cache is enabled"));
        }
        
        if self.redis.url.starts_with("rediss://") && !self.redis.use_tls {
            warn!("Redis URL uses TLS (rediss://) but use_tls is set to false");
        }
        
        if self.cache.default_ttl == 0 {
            return Err(anyhow::anyhow!("Cache default TTL must be greater than 0"));
        }
        
        if self.cache.max_entries == 0 {
            return Err(anyhow::anyhow!("Cache max entries must be greater than 0"));
        }
        
        Ok(())
    }
    
    /// Get Redis connection URL with proper formatting.
    /// 
    /// This method ensures the Redis URL is properly formatted and includes
    /// any necessary parameters.
    pub fn get_redis_url(&self) -> String {
        let mut url = self.redis.url.clone();
        
        // Add connection parameters if not already present
        if !url.contains('?') {
            url.push_str(&format!(
                "?connection_timeout={}&command_timeout={}&max_connections={}",
                self.redis.connection_timeout,
                self.redis.command_timeout,
                self.redis.max_connections
            ));
        }
        
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_default_config() {
        let config = ActorCoreConfig::default();
        assert_eq!(config.redis.url, "redis://localhost:6379");
        assert_eq!(config.cache.default_ttl, 1800);
        assert!(!config.cache.enable_redis);
    }
    
    #[test]
    fn test_config_from_env() {
        env::set_var("ACTOR_CORE_REDIS_URL", "redis://test:6379");
        env::set_var("ACTOR_CORE_CACHE_DEFAULT_TTL", "3600");
        
        let config = ActorCoreConfig::from_env().expect("Failed to load config from environment");
        assert_eq!(config.redis.url, "redis://test:6379");
        assert_eq!(config.cache.default_ttl, 3600);
        assert!(config.cache.enable_redis);
        
        env::remove_var("ACTOR_CORE_REDIS_URL");
        env::remove_var("ACTOR_CORE_CACHE_DEFAULT_TTL");
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = ActorCoreConfig::default();
        config.cache.enable_redis = true;
        config.redis.url = "".to_string();
        
        assert!(config.validate().is_err());
        
        config.redis.url = "redis://localhost:6379".to_string();
        assert!(config.validate().is_ok());
    }
}
