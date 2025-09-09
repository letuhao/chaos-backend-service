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

// Re-export commonly used types
pub use types::*;
pub use enums::*;
pub use interfaces::*;
pub use services::*;
pub use registry::*;
pub use cache::*;
pub use error::*;
