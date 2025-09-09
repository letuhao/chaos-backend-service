//! Performance optimization modules for the Actor Core system.
//!
//! This module provides SIMD optimizations, benchmarks, and real-time analytics
//! for high-performance actor stat aggregation.

pub mod simd;
pub mod benchmarks;
pub mod analytics;

pub use simd::*;
pub use benchmarks::*;
pub use analytics::*;
