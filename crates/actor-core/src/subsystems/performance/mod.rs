//! Performance Monitoring and Optimization
//!
//! This module contains subsystems related to performance monitoring,
//! benchmarking, and optimization tools.

pub mod performance_monitor;

// Re-export commonly used performance components
pub use performance_monitor::{PerformanceMonitor, PerformanceConfig, PerformanceStats, LoadTestingSuite};
