//! Subsystems for the Actor Core system.
//!
//! This module contains various subsystems that contribute to actor stats
//! and provide specialized functionality within the Actor Core framework.

pub mod resource_manager;
pub mod enhanced_hybrid_resource_manager;
pub mod resource_database;
pub mod system_resource_manager;
pub mod resource_cache;
pub mod stat_change_notifier;
pub mod rpg_resource_manager;
pub mod magic_resource_manager;
pub mod resource_regeneration;
pub mod resource_events;
pub mod performance_monitor;
pub mod integration_tests;

// Re-export commonly used subsystems
pub use resource_manager::ResourceManagerSubsystem;
pub use enhanced_hybrid_resource_manager::EnhancedHybridResourceManager;
pub use resource_database::{MongoResourceDatabase, InMemoryResourceDatabase};
pub use system_resource_manager::{
    SystemResourceCalculator, JindanSystemResourceCalculator, 
    RpgSystemResourceCalculator, MagicSystemResourceCalculator
};
pub use resource_cache::{ResourceCache, CacheConfig, CacheStats};
pub use stat_change_notifier::{StatChangeNotifier, StatChangeEvent, StatChangeListener};
pub use rpg_resource_manager::{RpgResourceManager, RpgResourceCategory, RpgResourceEvent};
pub use magic_resource_manager::{MagicResourceManager, MagicResourceCategory, MagicSchool, MagicResourceEvent};
pub use resource_regeneration::{ResourceRegenerationManager, RegenerationConfig, RegenerationStats};
pub use resource_events::{ResourceEventManager, ResourceEvent, ResourceEventType, EventConfig, EventStats};
pub use performance_monitor::{PerformanceMonitor, PerformanceConfig, PerformanceStats, LoadTestingSuite};
pub use integration_tests::{IntegrationTestSuite, ComprehensiveTestResults};
