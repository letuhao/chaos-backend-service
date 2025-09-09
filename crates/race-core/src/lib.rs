//! Race Core - Race definitions, bonuses, and racial abilities.
//!
//! This crate provides the core functionality for character races,
//! racial bonuses, abilities, and racial progression in the Chaos World MMORPG.

pub mod types;
pub mod enums;
pub mod interfaces;
pub mod services;
pub mod abilities;
pub mod bonuses;
pub mod error;

// Re-export commonly used types
pub use types::*;
pub use enums::*;
pub use interfaces::*;
pub use services::*;
pub use error::*;
