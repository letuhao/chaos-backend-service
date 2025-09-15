//! Direct coverage tests for config.rs module.
//! This file provides direct tests that exercise the actual config module code.

use actor_core::config::*;

// ============================================================================
// CONFIG STRUCT CREATION TESTS
// ============================================================================

#[test]
fn test_actor_core_config_creation() {
    let config = ActorCoreConfig {
        redis: RedisConfig::default(),
        cache: CacheConfig::default(),
        logging: LoggingConfig::default(),
    };
    
    assert_eq!(config.redis.url, "redis://localhost:6379");
    assert_eq!(config.cache.default_ttl, 1800);
    assert_eq!(config.logging.level, "info");
}

#[test]
fn test_redis_config_creation() {
    let redis_config = RedisConfig {
        url: "redis://test:6379".to_string(),
        connection_timeout: 10,
        command_timeout: 5,
        max_connections: 20,
        use_tls: true,
    };
    
    assert_eq!(redis_config.url, "redis://test:6379");
    assert_eq!(redis_config.connection_timeout, 10);
    assert_eq!(redis_config.command_timeout, 5);
    assert_eq!(redis_config.max_connections, 20);
    assert!(redis_config.use_tls);
}

#[test]
fn test_cache_config_creation() {
    let cache_config = CacheConfig {
        default_ttl: 3600,
        max_entries: 2_000_000,
        enable_redis: true,
        l1_size: 100_000,
        l2_size: 400_000,
        l3_size: 1_000_000,
    };
    
    assert_eq!(cache_config.default_ttl, 3600);
    assert_eq!(cache_config.max_entries, 2_000_000);
    assert!(cache_config.enable_redis);
    assert_eq!(cache_config.l1_size, 100_000);
    assert_eq!(cache_config.l2_size, 400_000);
    assert_eq!(cache_config.l3_size, 1_000_000);
}

#[test]
fn test_logging_config_creation() {
    let logging_config = LoggingConfig {
        level: "debug".to_string(),
        structured: false,
        json: true,
    };
    
    assert_eq!(logging_config.level, "debug");
    assert!(!logging_config.structured);
    assert!(logging_config.json);
}

// ============================================================================
// DEFAULT IMPLEMENTATION TESTS
// ============================================================================

#[test]
fn test_actor_core_config_default() {
    let config = ActorCoreConfig::default();
    
    assert_eq!(config.redis.url, "redis://localhost:6379");
    assert_eq!(config.redis.connection_timeout, 5);
    assert_eq!(config.redis.command_timeout, 3);
    assert_eq!(config.redis.max_connections, 10);
    assert!(!config.redis.use_tls);
    
    assert_eq!(config.cache.default_ttl, 1800);
    assert_eq!(config.cache.max_entries, 1_000_000);
    assert!(!config.cache.enable_redis);
    assert_eq!(config.cache.l1_size, 50_000);
    assert_eq!(config.cache.l2_size, 200_000);
    assert_eq!(config.cache.l3_size, 500_000);
    
    assert_eq!(config.logging.level, "info");
    assert!(config.logging.structured);
    assert!(!config.logging.json);
}

#[test]
fn test_redis_config_default() {
    let config = RedisConfig::default();
    
    assert_eq!(config.url, "redis://localhost:6379");
    assert_eq!(config.connection_timeout, 5);
    assert_eq!(config.command_timeout, 3);
    assert_eq!(config.max_connections, 10);
    assert!(!config.use_tls);
}

#[test]
fn test_cache_config_default() {
    let config = CacheConfig::default();
    
    assert_eq!(config.default_ttl, 1800);
    assert_eq!(config.max_entries, 1_000_000);
    assert!(!config.enable_redis);
    assert_eq!(config.l1_size, 50_000);
    assert_eq!(config.l2_size, 200_000);
    assert_eq!(config.l3_size, 500_000);
}

#[test]
fn test_logging_config_default() {
    let config = LoggingConfig::default();
    
    assert_eq!(config.level, "info");
    assert!(config.structured);
    assert!(!config.json);
}

// ============================================================================
// SERIALIZATION TESTS
// ============================================================================

#[test]
fn test_actor_core_config_serialization() {
    let config = ActorCoreConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(!json.is_empty());
    
    let deserialized: ActorCoreConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config.redis.url, deserialized.redis.url);
    assert_eq!(config.cache.default_ttl, deserialized.cache.default_ttl);
    assert_eq!(config.logging.level, deserialized.logging.level);
}

#[test]
fn test_redis_config_serialization() {
    let config = RedisConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(!json.is_empty());
    
    let deserialized: RedisConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config.url, deserialized.url);
    assert_eq!(config.connection_timeout, deserialized.connection_timeout);
}

#[test]
fn test_cache_config_serialization() {
    let config = CacheConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(!json.is_empty());
    
    let deserialized: CacheConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config.default_ttl, deserialized.default_ttl);
    assert_eq!(config.max_entries, deserialized.max_entries);
}

#[test]
fn test_logging_config_serialization() {
    let config = LoggingConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    assert!(!json.is_empty());
    
    let deserialized: LoggingConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config.level, deserialized.level);
    assert_eq!(config.structured, deserialized.structured);
}

// ============================================================================
// CLONE TESTS
// ============================================================================

#[test]
fn test_actor_core_config_clone() {
    let config = ActorCoreConfig::default();
    let cloned = config.clone();
    
    assert_eq!(config.redis.url, cloned.redis.url);
    assert_eq!(config.cache.default_ttl, cloned.cache.default_ttl);
    assert_eq!(config.logging.level, cloned.logging.level);
}

#[test]
fn test_redis_config_clone() {
    let config = RedisConfig::default();
    let cloned = config.clone();
    
    assert_eq!(config.url, cloned.url);
    assert_eq!(config.connection_timeout, cloned.connection_timeout);
}

#[test]
fn test_cache_config_clone() {
    let config = CacheConfig::default();
    let cloned = config.clone();
    
    assert_eq!(config.default_ttl, cloned.default_ttl);
    assert_eq!(config.max_entries, cloned.max_entries);
}

#[test]
fn test_logging_config_clone() {
    let config = LoggingConfig::default();
    let cloned = config.clone();
    
    assert_eq!(config.level, cloned.level);
    assert_eq!(config.structured, cloned.structured);
}

// ============================================================================
// DEBUG TESTS
// ============================================================================

#[test]
fn test_actor_core_config_debug() {
    let config = ActorCoreConfig::default();
    let debug_string = format!("{:?}", config);
    assert!(debug_string.contains("ActorCoreConfig"));
    assert!(debug_string.contains("redis"));
    assert!(debug_string.contains("cache"));
    assert!(debug_string.contains("logging"));
}

#[test]
fn test_redis_config_debug() {
    let config = RedisConfig::default();
    let debug_string = format!("{:?}", config);
    assert!(debug_string.contains("RedisConfig"));
    assert!(debug_string.contains("url"));
}

#[test]
fn test_cache_config_debug() {
    let config = CacheConfig::default();
    let debug_string = format!("{:?}", config);
    assert!(debug_string.contains("CacheConfig"));
    assert!(debug_string.contains("default_ttl"));
}

#[test]
fn test_logging_config_debug() {
    let config = LoggingConfig::default();
    let debug_string = format!("{:?}", config);
    assert!(debug_string.contains("LoggingConfig"));
    assert!(debug_string.contains("level"));
}

// ============================================================================
// CONFIG VALIDATION TESTS
// ============================================================================

#[test]
fn test_config_validation_success() {
    let config = ActorCoreConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_redis_url_required() {
    let mut config = ActorCoreConfig::default();
    config.cache.enable_redis = true;
    config.redis.url = "".to_string();
    
    assert!(config.validate().is_err());
}

#[test]
fn test_config_validation_redis_url_provided() {
    let mut config = ActorCoreConfig::default();
    config.cache.enable_redis = true;
    config.redis.url = "redis://localhost:6379".to_string();
    
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validation_cache_ttl_zero() {
    let mut config = ActorCoreConfig::default();
    config.cache.default_ttl = 0;
    
    assert!(config.validate().is_err());
}

#[test]
fn test_config_validation_cache_max_entries_zero() {
    let mut config = ActorCoreConfig::default();
    config.cache.max_entries = 0;
    
    assert!(config.validate().is_err());
}

// ============================================================================
// REDIS URL TESTS
// ============================================================================

#[test]
fn test_get_redis_url_basic() {
    let config = ActorCoreConfig::default();
    let url = config.get_redis_url();
    
    assert!(url.contains("redis://localhost:6379"));
    assert!(url.contains("connection_timeout=5"));
    assert!(url.contains("command_timeout=3"));
    assert!(url.contains("max_connections=10"));
}

#[test]
fn test_get_redis_url_with_existing_params() {
    let mut config = ActorCoreConfig::default();
    config.redis.url = "redis://localhost:6379?existing=param".to_string();
    let url = config.get_redis_url();
    
    assert!(url.contains("redis://localhost:6379"));
    assert!(url.contains("existing=param"));
}

// ============================================================================
// ENVIRONMENT LOADING TESTS (Simplified to avoid env var interference)
// ============================================================================

#[test]
fn test_from_env_basic() {
    // Test that from_env doesn't panic and returns a valid config
    let config = ActorCoreConfig::from_env().unwrap();
    assert!(!config.redis.url.is_empty());
    assert!(config.cache.default_ttl > 0);
    assert!(!config.logging.level.is_empty());
}

// ============================================================================
// COMPREHENSIVE CONFIG TESTS
// ============================================================================

#[test]
fn test_config_roundtrip() {
    let original = ActorCoreConfig::default();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: ActorCoreConfig = serde_json::from_str(&json).unwrap();
    
    assert_eq!(original.redis.url, deserialized.redis.url);
    assert_eq!(original.cache.default_ttl, deserialized.cache.default_ttl);
    assert_eq!(original.logging.level, deserialized.logging.level);
}

#[test]
fn test_config_field_access() {
    let mut config = ActorCoreConfig::default();
    
    // Test field access
    assert_eq!(config.redis.url, "redis://localhost:6379");
    assert_eq!(config.cache.default_ttl, 1800);
    assert_eq!(config.logging.level, "info");
    
    // Test field modification
    config.redis.url = "redis://new:6379".to_string();
    config.cache.default_ttl = 3600;
    config.logging.level = "debug".to_string();
    
    assert_eq!(config.redis.url, "redis://new:6379");
    assert_eq!(config.cache.default_ttl, 3600);
    assert_eq!(config.logging.level, "debug");
}

#[test]
fn test_config_nested_field_access() {
    let config = ActorCoreConfig::default();
    
    // Test nested field access
    assert_eq!(config.redis.connection_timeout, 5);
    assert_eq!(config.redis.command_timeout, 3);
    assert_eq!(config.redis.max_connections, 10);
    assert!(!config.redis.use_tls);
    
    assert_eq!(config.cache.max_entries, 1_000_000);
    assert!(!config.cache.enable_redis);
    assert_eq!(config.cache.l1_size, 50_000);
    assert_eq!(config.cache.l2_size, 200_000);
    assert_eq!(config.cache.l3_size, 500_000);
    
    assert!(config.logging.structured);
    assert!(!config.logging.json);
}
