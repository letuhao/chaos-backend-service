//! # Unified Element Registry Module
//! 
//! This module provides the UnifiedElementRegistry as the single source of truth for all element data,
//! following the data hub pattern where external systems can register and contribute elemental data
//! without direct dependencies.
//! 
//! ## Architecture
//! 
//! The Unified Element Registry acts as a central data hub that:
//! - **Aggregates** element definitions from multiple sources
//! - **Manages** element interactions and relationships
//! - **Provides** thread-safe access to all elemental data
//! - **Validates** data integrity across all registered elements
//! - **Caches** frequently accessed data for performance
//! 
//! ## Components
//! 
//! ### `UnifiedElementRegistry`
//! - **Central Registry**: Main registry for all element data
//! - **Thread Safety**: `Arc<RwLock<T>>` for concurrent access
//! - **System Integration**: Manages external system registrations
//! - **Performance**: Optimized for high-frequency access patterns
//! 
//! ### `ElementDefinition`
//! - **Element Properties**: Core element characteristics
//! - **Derived Stats**: Configuration for calculated statistics
//! - **Status Effects**: Element-specific status effect definitions
//! - **Environment Mods**: Environmental interaction modifiers
//! 
//! ### `SystemRegistration`
//! - **External Systems**: Registration of contributing systems
//! - **Capabilities**: System-specific elemental capabilities
//! - **Health Monitoring**: System status and health tracking
//! - **Metadata**: System identification and versioning
//! 
//! ### `ElementInteraction`
//! - **Interaction Matrix**: 2D array for O(1) interaction lookups
//! - **Tương Sinh Tương Khắc**: Traditional Chinese elemental relationships
//! - **Configurable Bonuses**: Custom interaction multipliers
//! - **Performance**: Optimized for game loop access patterns
//! 
//! ### `RegistryConfig`
//! - **Configuration Management**: Registry-wide settings
//! - **Performance Tuning**: Cache and performance configurations
//! - **Validation Rules**: Data integrity validation settings
//! - **Logging**: Registry operation logging configuration
//! 
//! ## Usage Patterns
//! 
//! ### Basic Registry Usage
//! ```rust
//! let registry = UnifiedElementRegistry::new();
//! registry.register_element("fire", element_definition)?;
//! let element = registry.get_element("fire")?;
//! ```
//! 
//! ### System Registration
//! ```rust
//! let system_reg = SystemRegistration::new("race-core", capabilities);
//! registry.register_system(system_reg)?;
//! ```
//! 
//! ### Element Interactions
//! ```rust
//! let interaction = ElementInteraction::new("fire", "water", 1.5);
//! registry.register_interaction(interaction)?;
//! let bonus = registry.get_interaction_bonus("fire", "water")?;
//! ```

pub mod unified_element_registry;
pub mod element_definition;
pub mod system_registration;
pub mod element_category;
pub mod element_plugin;
pub mod element_interaction;
pub mod registry_config;
pub mod registry_metrics;

pub use unified_element_registry::*;
pub use element_definition::*;
pub use system_registration::*;
pub use element_category::*;
pub use element_plugin::*;
pub use element_interaction::*;
pub use registry_config::*;
pub use registry_metrics::*;
