//! Prelude module for Actor Core.
//!
//! This module re-exports the most commonly used types and traits,
//! providing a clean and minimal API surface for users of the library.
//!
//! # Usage
//!
//! ```rust
//! use actor_core::prelude::*;
//!
//! // Now you have access to all the core types and traits
//! let actor = Actor::new("player1".to_string(), "human".to_string());
//! let contribution = Contribution::new("strength".to_string(), Bucket::Flat, 10.0, "equipment".to_string());
//! ```
//!
//! # What's Included
//!
//! - **Core Types**: `Actor`, `Contribution`, `CapContribution`, `Snapshot`, `Caps`
//! - **Enums**: `Bucket`, `CapMode`, `AcrossLayerPolicy`, `Operator`
//! - **Traits**: `Subsystem`, `Aggregator`, `CapsProvider`, `PluginRegistry`, `CombinerRegistry`, `Cache`
//! - **Error Types**: `ActorCoreError`, `ActorCoreResult`
//! - **Service Factory**: `ServiceFactory` for easy service creation
//! - **Common Utilities**: `ObservabilityManager`, `PerformanceProfiler`

// Core types - the essential data structures
pub use crate::types::{
    Actor,
    Contribution,
    CapContribution,
    SubsystemOutput,
    Snapshot,
    Caps,
    ModifierPack,
    EffectiveCaps,
    SubsystemMeta,
};

// Enums - the behavioral definitions
pub use crate::enums::{
    Bucket,
    CapMode,
    AcrossLayerPolicy,
    Operator,
};

// Traits - the behavioral contracts
pub use crate::interfaces::{
    Subsystem,
    Aggregator,
    CapsProvider,
    PluginRegistry,
    CombinerRegistry,
    CapLayerRegistry,
    Cache,
    CombinerRegistryAsync,
    CapLayerRegistryAsync,
};

// Error handling
pub use crate::error::{
    ActorCoreError,
    ActorCoreResult,
};

// Validation
pub use crate::validation::{
    Validator,
    ValidationRules,
    ValidationResult,
    ValidationError,
    ValidationWarning,
    validators,
    ValidationMiddlewareFactory,
};

// Deprecation management
pub use crate::deprecation::{
    DeprecationManager,
    DeprecationEntry,
    DeprecationStatus,
    DeprecationSeverity,
    DeprecationConfig,
    DeprecationReport,
    RollbackPlan,
    RollbackStep,
    RiskLevel,
    default_deprecations,
    default_rollback_plans,
    MigrationGuideManager,
    MigrationGuide,
    MigrationComplexity,
    MigrationTime,
    BreakingChange,
    BreakingChangeType,
    ChangeImpact,
    MigrationStep,
    CodeExample,
    MigrationPitfall,
    MigrationResource,
    ResourceType,
    MigrationChecklistItem,
    ChecklistCategory,
    MigrationGuideTemplate,
    TemplateSection,
    MigrationGuideData,
    default_migration_guides,
};

// Service factory for easy setup
pub use crate::service_factory::{
    ServiceFactory,
};

// Observability and performance monitoring
pub use crate::observability::{
    ObservabilityManager,
    ObservabilityConfig,
    // New observability components
    SLOManager,
    SLO,
    SLOStatus,
    SLOMetricType,
    SLOSeverity,
    SLOViolation,
    default_slos,
    MetricsCollector,
    MetricsSnapshot,
    MetricValue,
    MetricType,
    default_metrics,
    ObservabilityDashboard,
    DashboardConfig,
    DashboardStatus,
    SystemHealthStatus,
    DashboardBuilder,
};

pub use crate::performance::{
    PerformanceProfiler,
    PerformanceTestSuite,
    PerformanceWorkflow,
};

// Metrics for monitoring
pub use crate::metrics::{
    SubsystemMetrics,
    AggregatorMetrics,
    CapStatistics,
    CacheStats,
};

// Constants for common values
pub use crate::constants::system_ids::*;

// Common async utilities
pub use async_trait::async_trait;

// Re-export serde for convenience in user code
pub use serde::{Serialize, Deserialize};

// Re-export common std types that are frequently used
pub use std::collections::HashMap;
pub use std::sync::Arc;

/// Version information for the Actor Core library.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The minimum supported Rust version (MSRV).
pub const MSRV: &str = "1.89.0";

/// Feature flags available in this build.
pub const AVAILABLE_FEATURES: &[&str] = &[
    #[cfg(feature = "moka-cache")]
    "moka-cache",
    #[cfg(feature = "memory-mapped")]
    "memory-mapped",
    #[cfg(feature = "redis-cache")]
    "redis-cache",
    #[cfg(feature = "mongodb-storage")]
    "mongodb-storage",
    #[cfg(feature = "sqlx-database")]
    "sqlx-database",
    #[cfg(feature = "cli-tools")]
    "cli-tools",
    #[cfg(feature = "heavy-deps")]
    "heavy-deps",
];

/// Quick setup function for common use cases.
///
/// This function creates a basic Actor Core setup with default configurations.
/// Perfect for getting started quickly or for simple use cases.
///
/// # Example
///
/// ```rust
/// use actor_core::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> ActorCoreResult<()> {
///     let (aggregator, cache) = quick_setup().await?;
///     
///     let actor = Actor::new("player1".to_string(), "human".to_string());
///     let snapshot = aggregator.resolve(&actor).await?;
///     
///     println!("Actor stats: {:?}", snapshot);
///     Ok(())
/// }
/// ```
pub async fn quick_setup() -> ActorCoreResult<(Arc<dyn Aggregator>, Arc<dyn Cache>)> {
    let cache = ServiceFactory::create_cache()?;
    let plugin_registry = ServiceFactory::create_plugin_registry();
    let combiner_registry = ServiceFactory::create_combiner_registry();
    let caps_provider = ServiceFactory::create_caps_provider();
    
    let aggregator = ServiceFactory::create_aggregator(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache.clone(),
    )?;
    
    Ok((aggregator, cache))
}

/// Create a simple actor with basic data.
///
/// This is a convenience function for creating actors with common initial data.
///
/// # Example
///
/// ```rust
/// use actor_core::prelude::*;
///
/// let actor = create_simple_actor("player1", "human", 10);
/// assert_eq!(actor.data.get("level").unwrap().as_i64().unwrap(), 10);
/// ```
pub fn create_simple_actor(id: &str, race: &str, level: i64) -> Actor {
    let mut actor = Actor::new(id.to_string(), race.to_string());
    
    let mut data = HashMap::new();
    data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(level)));
    data.insert("race".to_string(), serde_json::Value::String(race.to_string()));
    
    actor.set_data(data);
    actor
}

/// Create a basic contribution for testing or simple use cases.
///
/// # Example
///
/// ```rust
/// use actor_core::prelude::*;
///
/// let contribution = create_basic_contribution("strength", 10.0, "equipment");
/// assert_eq!(contribution.dimension, "strength");
/// assert_eq!(contribution.value, 10.0);
/// ```
pub fn create_basic_contribution(dimension: &str, value: f64, system: &str) -> Contribution {
    Contribution::new(
        dimension.to_string(),
        Bucket::Flat,
        value,
        system.to_string(),
    )
}

/// Create a basic caps configuration.
///
/// # Example
///
/// ```rust
/// use actor_core::prelude::*;
///
/// let caps = create_basic_caps(0.0, 100.0);
/// assert_eq!(caps.min, Some(0.0));
/// assert_eq!(caps.max, Some(100.0));
/// ```
pub fn create_basic_caps(min: f64, max: f64) -> Caps {
    Caps {
        min: Some(min),
        max: Some(max),
    }
}

/// Validate a contribution with default rules.
///
/// # Example
///
/// ```rust
/// use actor_core::prelude::*;
///
/// let contribution = create_basic_contribution("strength", 10.0, "equipment");
/// let result = validate_contribution(&contribution);
/// assert!(result.is_valid);
/// ```
pub fn validate_contribution(contribution: &Contribution) -> ValidationResult {
    validators::validate_contribution(contribution)
}

/// Validate a cap contribution with default rules.
///
/// # Example
///
/// ```rust
/// use actor_core::prelude::*;
///
/// let cap_contrib = CapContribution::new(
///     "equipment".to_string(),
///     "strength".to_string(),
///     CapMode::Baseline,
///     "max".to_string(),
///     100.0,
/// );
/// let result = validate_cap_contribution(&cap_contrib);
/// assert!(result.is_valid);
/// ```
pub fn validate_cap_contribution(cap_contrib: &CapContribution) -> ValidationResult {
    validators::validate_cap_contribution(cap_contrib)
}

/// Validate an actor with default rules.
///
/// # Example
///
/// ```rust
/// use actor_core::prelude::*;
///
/// let actor = create_simple_actor("player1", "human", 10);
/// let result = validate_actor(&actor);
/// assert!(result.is_valid);
/// ```
pub fn validate_actor(actor: &Actor) -> ValidationResult {
    validators::validate_actor(actor)
}

/// Validate a snapshot with default rules.
///
/// # Example
///
/// ```rust
/// use actor_core::prelude::*;
///
/// let snapshot = Snapshot::new("player1".parse().unwrap(), 1);
/// let result = validate_snapshot(&snapshot);
/// assert!(result.is_valid);
/// ```
pub fn validate_snapshot(snapshot: &Snapshot) -> ValidationResult {
    validators::validate_snapshot(snapshot)
}

/// Validate that all required features are available.
///
/// This function checks if the required features are enabled at compile time.
/// Useful for ensuring your application has the necessary functionality.
///
/// # Example
///
/// ```rust
/// use actor_core::prelude::*;
///
/// fn main() {
///     if !has_feature("redis-cache") {
///         eprintln!("Warning: Redis cache support not available");
///     }
/// }
/// ```
pub fn has_feature(feature: &str) -> bool {
    AVAILABLE_FEATURES.contains(&feature)
}

/// Get the library version and build information.
///
/// # Example
///
/// ```rust
/// use actor_core::prelude::*;
///
/// let info = get_build_info();
/// println!("Actor Core version: {}", info.version);
/// println!("MSRV: {}", info.msrv);
/// ```
pub fn get_build_info() -> BuildInfo {
    BuildInfo {
        version: VERSION.to_string(),
        msrv: MSRV.to_string(),
        features: AVAILABLE_FEATURES.to_vec(),
        build_target: std::env::consts::ARCH.to_string(),
        build_os: std::env::consts::OS.to_string(),
    }
}

/// Build information structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildInfo {
    /// Library version
    pub version: String,
    /// Minimum supported Rust version
    pub msrv: String,
    /// Available feature flags
    pub features: Vec<String>,
    /// Build target architecture
    pub build_target: String,
    /// Build operating system
    pub build_os: String,
}