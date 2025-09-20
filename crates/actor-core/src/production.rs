//! Production readiness helpers for Actor Core.
//!
//! Provides a simple readiness check that validates registries and performs
//! a cache round-trip to ensure core components are operational.

use crate::interfaces::{PluginRegistry, CombinerRegistry, CapsProvider, Cache};
use crate::ActorCoreResult;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// Perform a production readiness check.
/// - Validates plugin, combiner, and cap layer registries
/// - Exercises cache set/get/delete round-trip
pub fn check_readiness(
    plugin_registry: &dyn PluginRegistry,
    combiner_registry: &dyn CombinerRegistry,
    caps_provider: &dyn CapsProvider,
    cache: &dyn Cache,
) -> ActorCoreResult<()> {
    let config = ProductionConfig::load_config().unwrap_or_else(|_| {
        warn!("Failed to load production config, using hardcoded defaults");
        ProductionConfig::get_default_config()
    });

    // Validate plugin registry
    plugin_registry.validate_all()?;

    // Validate combiner and cap layer registries
    combiner_registry.validate()?;
    caps_provider.validate()?;

    // Cache round-trip
    let key = &config.readiness_probe_key;
    let val = serde_json::json!({ "ok": true });
    cache.set(key.to_string(), val.clone(), Some(config.readiness_probe_ttl))?;
    let got = cache.get(key).ok_or_else(|| {
        crate::ActorCoreError::CacheError("Readiness cache get returned None".to_string())
    })?;
    if got != val {
        return Err(crate::ActorCoreError::CacheError(
            "Readiness cache value mismatch".to_string(),
        ));
    }
    let _ = cache.delete(key)?;

    info!("Production readiness check completed successfully");
    Ok(())
}

/// Production configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {
    pub readiness_probe_key: String,
    pub readiness_probe_ttl: u64,
    pub enable_readiness_check: bool,
    pub enable_health_check: bool,
    pub health_check_interval_seconds: u64,
    pub enable_metrics: bool,
    pub enable_logging: bool,
    pub log_level: String,
}

impl ProductionConfig {
    /// Load production configuration from config file
    pub fn load_config() -> ActorCoreResult<Self> {
        // Try to load from production_config.yaml first
        let config_path = std::path::Path::new("configs/production_config.yaml");
            
        if config_path.exists() {
            match Self::load_config_from_file(config_path) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    warn!("Failed to load production config from file: {}. Using hardcoded defaults.", e);
                }
            }
        }
        
        // Fallback to hardcoded defaults
        Ok(Self::get_default_config())
    }

    /// Load configuration from file
    fn load_config_from_file(path: &std::path::Path) -> ActorCoreResult<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: ProductionConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Get default configuration
    fn get_default_config() -> Self {
        Self {
            readiness_probe_key: "actor_core:readiness_probe".to_string(),
            readiness_probe_ttl: 30,
            enable_readiness_check: true,
            enable_health_check: true,
            health_check_interval_seconds: 30,
            enable_metrics: true,
            enable_logging: true,
            log_level: "info".to_string(),
        }
    }
}