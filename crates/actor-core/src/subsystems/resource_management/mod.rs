//! Resource Management Subsystems
//!
//! This module contains all subsystems related to resource management,
//! including databases, caches, and regeneration.
//!
//! NOTE: Legacy resource managers with hardcoded resources have been moved to
//! examples/legacy_resource_managers/ to maintain the pure hub architecture.

use async_trait::async_trait;
use std::collections::HashMap;
use crate::types::Actor;
use crate::ActorCoreResult;

// Legacy system_resource_manager moved to examples/legacy_subsystems/
pub mod resource_regeneration;
pub mod resource_database;
pub mod resource_cache;

/// Trait for subsystems that can calculate system resources
#[async_trait]
pub trait SystemResourceCalculator {
    /// Get the system ID for this calculator
    fn system_id(&self) -> &str;
    
    /// Check if this calculator affects the given resource
    fn affects_resource(&self, resource_id: &str) -> bool;
    
    /// Calculate resources for the given actor
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>>;
    
    /// Notify about stat changes
    async fn notify_stat_change(&self, actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()>;
    
    /// Get resource dependencies
    fn get_resource_dependencies(&self) -> Vec<String>;
    
    /// Get resource categories
    fn get_resource_categories(&self) -> Vec<String>;
    
    /// Check if the calculator is active for the given actor
    async fn is_active(&self, actor: &Actor) -> ActorCoreResult<bool>;
}

// Re-export commonly used resource management subsystems
pub use resource_database::InMemoryResourceDatabase;
#[cfg(feature = "mongodb-storage")]
pub use resource_database::MongoResourceDatabase;
// Legacy system resource managers moved to examples/legacy_subsystems/
pub use resource_cache::{ResourceCache, CacheConfig, CacheStats};
pub use resource_regeneration::{ResourceRegenerationManager, RegenerationConfig, RegenerationStats};