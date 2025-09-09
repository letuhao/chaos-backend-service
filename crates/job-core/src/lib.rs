//! Job Core - Job classes, skills, and specialization systems.
//!
//! This crate provides the core functionality for job classes,
//! skill systems, specialization trees, and job progression in the Chaos World MMORPG.

pub mod types;
pub mod enums;
pub mod interfaces;
pub mod services;
pub mod classes;
pub mod skills;
pub mod specializations;
pub mod error;

// Re-export commonly used types
pub use types::*;
pub use enums::*;
pub use interfaces::*;
pub use services::*;
pub use error::*;
