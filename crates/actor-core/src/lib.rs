//! Actor Core - Character stat aggregation and management system.
//!
//! This crate provides the core functionality for managing character stats,
//! stat aggregation, and character progression in the Chaos World MMORPG.
//!
//! # Quick Start
//!
//! ```rust
//! use actor_core::*;
//! use std::collections::HashMap;
//!
//! // Create an actor
//! let mut actor = Actor::new("Player1".to_string(), "Human".to_string());
//!
//! // Add some data
//! let mut data = HashMap::new();
//! data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(10)));
//! actor.set_data(data);
//!
//! // Add a buff
//! actor.add_buff("strength_boost".to_string());
//!
//! // Create contributions
//! let contribution = Contribution::new(
//!     "strength".to_string(),
//!     Bucket::Flat,
//!     10.0,
//!     "equipment".to_string()
//! );
//!
//! // Process contributions
//! let result = bucket_processor::process_contributions_in_order(
//!     vec![contribution],
//!     0.0,
//!     None
//! );
//! ```
//!
//! # Features
//!
//! - **High-Performance Stat Aggregation**: Efficient processing of character stats with deterministic ordering
//! - **Flexible Bucket System**: Support for different stat processing modes (Flat, Mult, PostAdd, Override)
//! - **Caps Management**: Comprehensive min/max value constraints with layer-based policies
//! - **Async Support**: Full async/await support for non-blocking operations
//! - **Caching**: Built-in caching system for performance optimization
//! - **Configuration Loading**: YAML/JSON configuration support for game rules
//! - **Comprehensive Testing**: Unit tests, integration tests, property-based tests, and benchmarks
//! - **CI/CD Ready**: Full CI pipeline with formatting, linting, security auditing, and performance monitoring
//!
//! # Architecture
//!
//! The system is built around several core components:
//!
//! - **Actor**: Represents a character with stats, buffs, and subsystems
//! - **Contribution**: Individual stat modifications from various sources
//! - **Caps**: Min/max constraints for stat values
//! - **Snapshot**: Aggregated stat state at a point in time
//! - **Subsystem**: Modular stat processing components
//!
//! # Bucket System
//!
//! The bucket system determines how contributions are processed:
//!
//! - **Flat**: Additive contributions (equipment bonuses)
//! - **Mult**: Multiplicative contributions (percentage bonuses)
//! - **PostAdd**: Post-addition contributions (final adjustments)
//! - **Override**: Override contributions (replaces previous values)
//!
//! # Examples
//!
//! See the `examples/` directory for comprehensive usage examples:
//!
//! - `basic_usage.rs`: Basic actor operations and contribution processing
//! - `subsystem_example.rs`: Custom subsystem implementation
//! - `configuration_example.rs`: Configuration file usage
//!
//! # Performance
//!
//! The system is designed for high performance with:
//!
//! - Zero-copy operations where possible
//! - Efficient memory management with custom allocators
//! - SIMD optimizations for mathematical operations
//! - Comprehensive caching to avoid redundant calculations
//! - Async processing for non-blocking operations
//!
//! # Testing
//!
//! Run the comprehensive test suite:
//!
//! ```bash
//! # Run all tests
//! cargo test
//!
//! # Run with extra features
//! cargo test --features extra_buckets
//!
//! # Run specific test suites
//! cargo test --test actor_tests
//! cargo test --test caps_tests
//! cargo test --test property_tests
//! cargo test --test edge_case_tests
//! ```
//!
//! # Benchmarking
//!
//! Run performance benchmarks:
//!
//! ```bash
//! # Run all benchmarks
//! cargo bench
//!
//! # Run quick benchmarks
//! make bench-quick
//!
//! # Run comprehensive benchmarks
//! make bench-comprehensive
//! ```
//!
//! # Configuration
//!
//! The system supports YAML and JSON configuration files:
//!
//! ```yaml
//! # configs/cap_layers.yaml
//! cap_layers:
//!   - name: base
//!     priority: 100
//!     cap_mode: BASELINE
//!   - name: equipment
//!     priority: 200
//!     cap_mode: ADDITIVE
//!   - name: buffs
//!     priority: 300
//!     cap_mode: HARD_MAX
//!
//! across_layer_policy: STRICT
//! ```
//!
//! # License
//!
//! This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
//!
//! # Contributing
//!
//! Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.
//!
//! # Support
//!
//! For questions, issues, or contributions:
//!
//! - Open an issue on GitHub
//! - Join our Discord community
//! - Check the documentation

pub mod types;
pub mod enums;
pub mod interfaces;
pub mod services;
pub mod registry;
pub mod cache;
pub mod constants;
pub mod error;
pub mod pools;
pub mod performance;
pub mod bucket_processor;

#[cfg(test)]
mod integration_tests;

// Re-export commonly used types
pub use enums::*;
pub use services::*;
pub use registry::*;
pub use cache::*;
pub use error::*;

// Re-export specific types to avoid conflicts
pub use types::{Actor, Contribution, CapContribution, Subsystem as SubsystemStruct, SubsystemOutput, Snapshot, Caps, ModifierPack, EffectiveCaps};
pub use interfaces::{Subsystem as SubsystemTrait, Aggregator, CapsProvider, PluginRegistry, CombinerRegistry, CapLayerRegistry, Cache};