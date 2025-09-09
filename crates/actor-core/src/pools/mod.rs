//! Memory pool system for high-performance actor core operations.
//!
//! This module provides memory pools for frequently allocated objects
//! to reduce garbage collection pressure and improve performance.

pub mod memory_pools;

pub use memory_pools::*;
