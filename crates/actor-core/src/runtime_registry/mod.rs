//! Runtime Registry System for Actor Core
//! 
//! This module provides runtime registries for resources, categories, and tags,
//! allowing subsystems to register their data dynamically without hardcoding.

pub mod resource_registry;
pub mod category_registry;
pub mod tag_registry;
pub mod registry_manager;

// Re-export main types for convenience
pub use resource_registry::*;
pub use category_registry::*;
pub use tag_registry::*;
pub use registry_manager::*;
