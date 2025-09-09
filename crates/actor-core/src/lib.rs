//! Actor Core - Character stat aggregation and management system.
//!
//! This crate provides the core functionality for managing character stats,
//! stat aggregation, and character progression in the Chaos World MMORPG.

pub mod types;
pub mod enums;
pub mod interfaces;
pub mod services;
pub mod registry;
pub mod cache;
pub mod constants;
pub mod error;
pub mod pools;
pub mod performance;

#[cfg(test)]
mod integration_tests;

// Re-export commonly used types
pub use enums::*;
pub use services::*;
pub use registry::*;
pub use cache::*;
pub use error::*;

// Re-export specific types to avoid conflicts
pub use types::{Actor, Contribution, CapContribution, Subsystem as SubsystemStruct, SubsystemOutput, Snapshot, Caps, ModifierPack};
pub use interfaces::{Subsystem as SubsystemTrait, Aggregator, CapsProvider, PluginRegistry, CombinerRegistry, CapLayerRegistry, Cache};