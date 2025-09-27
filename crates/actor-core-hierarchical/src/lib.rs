//! # Actor Core Hierarchical
//! 
//! High-performance hierarchical actor management system with proper separation of concerns.
//!
//! ## Overview
//!
//! This crate provides a hierarchical actor data system that serves as a **data hub** for
//! managing actor properties across multiple game systems. It focuses on **performance**,
//! **extensibility**, and **clean separation** of concerns.
//!
//! ## Key Features
//!
//! - **Data Hub Only**: Pure data storage and management, no business logic
//! - **Elemental Core Integration**: Array-based elemental system data management
//! - **High Performance**: 1-2 ns access time with array-based approach
//! - **Extensible Design**: Easy to add new systems without breaking existing ones
//! - **Clean Architecture**: Proper separation between actor management and system logic
//!
//! ## Architecture
//!
//! ```text
//! Actor Core Hierarchical
//! +-- Core
//! |   +-- HierarchicalActor      # Main actor data structure
//! |   +-- GlobalAggregator       # Global stats aggregation
//! |   +-- ActorFactory           # Actor creation factory
//! +-- Systems
//! |   +-- Elemental              # Elemental system integration (element-core)
//! +-- Adapters
//! |   +-- BaseAdapter            # Base adapter trait
//! |   +-- ActorAdapter           # Actor data conversion
//! +-- Aggregation
//!     +-- (Future: Custom aggregators)
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use actor_core_hierarchical::{
//!     HierarchicalActor, GlobalAggregator, ActorFactory
//! };
//! use element_core::ElementalRegistry;
//! use std::sync::Arc;
//! 
//! // Create actor factory with elemental registry
//! let elemental_registry = Arc::new(ElementalRegistry::new());
//! let factory = ActorFactory::new(elemental_registry);
//! 
//! // Create a basic warrior actor
//! let actor = factory.create_actor("warrior").unwrap();
//! 
//! // Create global aggregator
//! let mut aggregator = GlobalAggregator::new();
//! 
//! // Aggregate stats from all systems
//! let stats = aggregator.aggregate_actor_stats(&actor);
//! ```

pub mod core;
pub mod systems;
pub mod adapters;
pub mod aggregation;

// Re-export commonly used types
pub use core::*;
pub use systems::*;
pub use adapters::*;
pub use aggregation::*;