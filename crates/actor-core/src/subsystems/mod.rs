//! Subsystems for the Actor Core system.
//!
//! This module contains various subsystems that contribute to actor stats
//! and provide specialized functionality within the Actor Core framework.

pub mod resource_manager;

// Re-export commonly used subsystems
pub use resource_manager::ResourceManagerSubsystem;
