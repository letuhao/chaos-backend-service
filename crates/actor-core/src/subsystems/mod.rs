//! Subsystems for the Actor Core system.
//!
//! This module contains various subsystems that contribute to actor stats
//! and provide specialized functionality within the Actor Core framework.
//!
//! ## Organization
//!
//! The subsystems are organized into logical groups:
//! - `resource_management/` - All resource management related subsystems
//! - `exhaustion/` - Resource exhaustion system components
//! - `performance/` - Performance monitoring and optimization tools
//! - `core/` - Core system functionality
//! - `testing/` - Testing utilities and integration tests

pub mod resource_management;
pub mod exhaustion;
pub mod performance;
pub mod core;
pub mod testing;

// Re-export commonly used subsystems for backward compatibility
pub use resource_management::*;
pub use exhaustion::*;
pub use performance::*;
pub use core::*;
pub use testing::*;