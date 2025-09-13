//! Performance optimization modules for the Actor Core system.
//!
//! This module provides SIMD optimizations, benchmarks, and real-time analytics
//! for high-performance actor stat aggregation.

pub mod simd;
pub mod benchmarks;
pub mod analytics;
pub mod profiler;
pub mod test_suite;
pub mod workflow;
pub mod config;

pub use simd::*;
pub use benchmarks::*;
pub use analytics::*;
pub use profiler::*;
pub use test_suite::*;
pub use workflow::*;
pub use config::*;