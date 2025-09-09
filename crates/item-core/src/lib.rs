//! Item Core - Item generation, properties, and inventory management.
//!
//! This crate provides the core functionality for item management,
//! item generation, inventory systems, and item properties in the Chaos World MMORPG.

pub mod types;
pub mod enums;
pub mod interfaces;
pub mod services;
pub mod inventory;
pub mod generation;
pub mod properties;
pub mod error;

// Re-export commonly used types
pub use types::*;
pub use enums::*;
pub use interfaces::*;
pub use services::*;
pub use error::*;
