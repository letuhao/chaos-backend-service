//! Actor Core - Character stat aggregation and management system.
//!
//! This crate provides the core functionality for managing character stats,
//! stat aggregation, and character progression in the Chaos World MMORPG.
//!
//! # Quick Start
//!
//! ```rust
//! use actor_core::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> ActorCoreResult<()> {
//!     // Quick setup with default configurations
//!     let (aggregator, _cache) = quick_setup().await?;
//!     
//!     // Create a simple actor
//!     let actor = create_simple_actor("player1", "human", 10);
//!     
//!     // Resolve actor stats
//!     let snapshot = aggregator.resolve(&actor).await?;
//!     
//!     println!("Actor stats: {:?}", snapshot);
//!     Ok(())
//! }
//! ```
//!
//! # Alternative: Manual Setup
//!
//! For more control over the configuration:
//!
//! ```rust
//! use actor_core::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> ActorCoreResult<()> {
//!     // Create services manually
//!     let cache = ServiceFactory::create_cache()?;
//!     let plugin_registry = ServiceFactory::create_plugin_registry();
//!     let combiner_registry = ServiceFactory::create_combiner_registry();
//!     let caps_provider = ServiceFactory::create_caps_provider();
//!     
//!     let aggregator = ServiceFactory::create_aggregator(
//!         plugin_registry,
//!         combiner_registry,
//!         caps_provider,
//!         cache,
//!     )?;
//!     
//!     // Create actor and process
//!     let actor = Actor::new("player1".to_string(), "human".to_string());
//!     let snapshot = aggregator.resolve(&actor).await?;
//!     
//!     Ok(())
//! }
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

// Core modules - essential functionality
pub mod types;
pub mod enums;
pub mod interfaces;
pub mod error;
pub mod service_factory;
pub mod validation;
pub mod deprecation;

// Prelude module - clean API surface
pub mod prelude;

// Internal modules - advanced functionality
#[doc(hidden)]
pub mod metrics;
#[doc(hidden)]
pub mod aggregator;
#[doc(hidden)]
pub mod caps_provider;
#[doc(hidden)]
pub mod registry;
#[doc(hidden)]
pub mod cache;
#[doc(hidden)]
pub mod constants;
#[doc(hidden)]
pub mod pools;
#[doc(hidden)]
pub mod performance;
#[doc(hidden)]
pub mod bucket_processor;
#[doc(hidden)]
pub mod production;
#[doc(hidden)]
pub mod subsystems;
#[doc(hidden)]
pub mod observability;


// Re-export prelude as the main API
pub use prelude::*;