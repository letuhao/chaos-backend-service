//! # Factory Module
//! 
//! This module provides factory functionality for creating elemental system instances
//! with various configuration options and builder patterns.
//! 
//! ## Components
//! 
//! ### `ElementalFactory`
//! - **Main Factory**: Creates elemental system instances
//! - **Registry Integration**: Uses unified registry for element definitions
//! - **Configuration Loading**: Loads elements from YAML configurations
//! - **Parameter Support**: Creates systems with specific element parameters
//! 
//! ### `ElementalSystemBuilder`
//! - **Builder Pattern**: Step-by-step system construction
//! - **Flexible Configuration**: Add elements and properties incrementally
//! - **Validation**: Ensures system integrity during construction
//! - **Thread Safety**: Safe for concurrent builder operations
//! 
//! ## Usage Patterns
//! 
//! ### Basic Factory Usage
//! ```rust
//! let factory = ElementalFactory::new(registry);
//! let system = factory.create_elemental_system();
//! ```
//! 
//! ### Builder Pattern Usage
//! ```rust
//! let builder = factory.create_builder();
//! let system = builder
//!     .with_element("fire")
//!     .with_mastery_level("fire", 10.0)
//!     .build()?;
//! ```
//! 
//! ### Parameter-based Creation
//! ```rust
//! let params = ElementalParams {
//!     primary_element: "fire".to_string(),
//!     initial_mastery_levels: [("fire".to_string(), 5.0)].into(),
//!     // ... other parameters
//! };
//! let system = factory.create_elemental_system_with_params(params)?;
//! ```

pub mod elemental_factory;

pub use elemental_factory::*;
