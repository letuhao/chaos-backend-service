//! Configuration Hub System for Actor Core
//! 
//! This module provides a comprehensive configuration system that follows the Actor Core hub pattern,
//! allowing multiple subsystems to register configurations with merge/override/aggregate logic.

pub mod types;
pub mod provider;
pub mod registry;
pub mod combiner;
pub mod aggregator;
pub mod manager;
pub mod loader;
pub mod providers;
pub mod loaders;
pub mod mongodb;
pub mod mongodb_manager;

// Re-export main types for convenience
pub use types::*;
pub use provider::ConfigurationProvider;
pub use registry::ConfigurationRegistry;
pub use combiner::ConfigurationCombiner;
pub use aggregator::ConfigurationAggregator;
pub use manager::ConfigurationManager;
pub use loader::ConfigurationLoader;