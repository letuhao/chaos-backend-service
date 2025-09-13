//! Resource Management Subsystems
//!
//! This module contains all subsystems related to resource management,
//! including different types of resource managers, databases, caches, and regeneration.

pub mod resource_manager;
pub mod enhanced_hybrid_resource_manager;
pub mod system_resource_manager;
pub mod rpg_resource_manager;
pub mod magic_resource_manager;
pub mod resource_regeneration;
pub mod resource_database;
pub mod resource_cache;

// Re-export commonly used resource management subsystems
pub use resource_manager::ResourceManagerSubsystem;
pub use enhanced_hybrid_resource_manager::EnhancedHybridResourceManager;
pub use resource_database::{MongoResourceDatabase, InMemoryResourceDatabase};
pub use system_resource_manager::{
    SystemResourceCalculator, JindanSystemResourceCalculator, 
    RpgSystemResourceCalculator, MagicSystemResourceCalculator
};
pub use resource_cache::{ResourceCache, CacheConfig, CacheStats};
pub use rpg_resource_manager::{RpgResourceManager, RpgResourceCategory, RpgResourceEvent};
pub use magic_resource_manager::{MagicResourceManager, MagicResourceCategory, MagicSchool, MagicResourceEvent};
pub use resource_regeneration::{ResourceRegenerationManager, RegenerationConfig, RegenerationStats};
