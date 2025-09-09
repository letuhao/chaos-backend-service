//! Production readiness helpers for Actor Core.
//!
//! Provides a simple readiness check that validates registries and performs
//! a cache round-trip to ensure core components are operational.

use crate::interfaces::{PluginRegistry, CombinerRegistry, CapsProvider, Cache};
use crate::ActorCoreResult;

/// Perform a production readiness check.
/// - Validates plugin, combiner, and cap layer registries
/// - Exercises cache set/get/delete round-trip
pub fn check_readiness(
    plugin_registry: &dyn PluginRegistry,
    combiner_registry: &dyn CombinerRegistry,
    caps_provider: &dyn CapsProvider,
    cache: &dyn Cache,
) -> ActorCoreResult<()> {
    // Validate plugin registry
    plugin_registry.validate_all()?;

    // Validate combiner and cap layer registries
    combiner_registry.validate()?;
    caps_provider.validate()?;

    // Cache round-trip
    let key = "actor_core:readiness_probe";
    let val = serde_json::json!({ "ok": true });
    cache.set(key.to_string(), val.clone(), Some(30))?;
    let got = cache.get(key).ok_or_else(|| {
        crate::ActorCoreError::CacheError("Readiness cache get returned None".to_string())
    })?;
    if got != val {
        return Err(crate::ActorCoreError::CacheError(
            "Readiness cache value mismatch".to_string(),
        ));
    }
    let _ = cache.delete(key)?;

    Ok(())
}


