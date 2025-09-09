//! Combat Core - Combat system, damage calculation, and battle mechanics.
//!
//! This crate provides the core functionality for combat systems,
//! damage calculation, skill effects, and battle mechanics in the Chaos World MMORPG.

pub mod types;
pub mod enums;
pub mod interfaces;
pub mod services;
pub mod skills;
pub mod effects;
pub mod damage;
pub mod error;

// Re-export commonly used types
pub use types::*;
pub use enums::*;
pub use interfaces::*;
pub use services::*;
pub use error::*;
