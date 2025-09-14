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

// Re-export specific types to avoid naming conflicts
pub use simd::{SimdConfig, SimdOptimizer, SimdStats, SimdResult};
pub use benchmarks::{BenchmarkConfig, BenchmarkRunner, BenchmarkResults, ComprehensiveBenchmarkResults};
pub use analytics::{
    AnalyticsCollector, AnalyticsConfig,
    PerformanceMetrics as AnalyticsPerformanceMetrics,
    SystemMetrics as AnalyticsSystemMetrics,
    CacheMetrics as AnalyticsCacheMetrics,
    AggregationMetrics as AnalyticsAggregationMetrics,
    MemoryMetrics,
    ErrorMetrics as AnalyticsErrorMetrics
};
pub use profiler::{
    PerformanceProfiler,
    PerformanceMetrics as ProfilerPerformanceMetrics,
    SystemMetrics as ProfilerSystemMetrics,
    CacheMetrics as ProfilerCacheMetrics,
    AggregationMetrics as ProfilerAggregationMetrics,
    ErrorMetrics as ProfilerErrorMetrics,
    PerformanceTestResult as ProfilerPerformanceTestResult
};
pub use test_suite::{
    PerformanceTestSuite,
    PerformanceTestResult as TestSuitePerformanceTestResult,
    TestSuiteConfig,
    TestSuiteResults,
    OverallTestResults
};
pub use workflow::{PerformanceWorkflow, WorkflowConfig, BaselineConfig, WorkflowState, PerformanceTrends, WorkflowExecution, WorkflowReport};
pub use config::{PerformanceConfig as PerfConfig};