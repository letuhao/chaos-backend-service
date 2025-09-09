//! Shared types, utilities, and common functionality for Chaos World MMORPG backend.
//!
//! This crate provides common types, error definitions, and utility functions
//! that are used across multiple modules in the Chaos World backend.

pub mod error;
pub mod types;
pub mod utils;
pub mod constants;

// Re-export commonly used types
pub use error::{ChaosError, ChaosResult};
pub use types::*;
pub use utils::*;
pub use constants::*;
