//! Resource Exhaustion System
//!
//! This module contains all subsystems related to the Resource Exhaustion System,
//! including the main subsystem, event publishing, configuration loading, and performance optimizations.

pub mod resource_exhaustion;
pub mod exhaustion_event_publisher;
pub mod exhaustion_config_loader;
pub mod exhaustion_performance;

// Re-export commonly used exhaustion system components
pub use resource_exhaustion::{
    ResourceExhaustionSubsystem, ExhaustionConfig, ExhaustionEvent, ExhaustionEventType,
    ExhaustionTransition, ExhaustionEngine, ExhaustionEventPublisher, ExhaustionError
};
pub use exhaustion_event_publisher::{
    InMemoryEventPublisher, LoggingEventPublisher, NoOpEventPublisher, EventStats as ExhaustionEventStats
};
pub use exhaustion_config_loader::{
    ExhaustionConfigLoader, MergedConfig, ConfigSource, ConfigLoaderError
};
pub use exhaustion_performance::{
    OptimizedExhaustionEngine, ExhaustionBenchmark,
    BenchmarkConfig, BenchmarkResult, PerformanceStats as ExhaustionPerformanceStats
};
