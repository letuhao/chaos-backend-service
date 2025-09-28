//! # Element Core
//! 
//! A high-performance, comprehensive elemental system for the Chaos World MMORPG backend service.
//! 
//! ## Overview
//! 
//! The Element Core provides a complete elemental system that manages element definitions,
//! interactions, derived stats, and system integrations. It follows a data hub pattern where
//! other systems can register and contribute elemental data without direct dependencies.
//! 
//! ## Key Features
//! 
//! ### 🚀 **High Performance**
//! - Array-based data structures for O(1) access times
//! - Fixed-size arrays (`MAX_ELEMENTS = 50`) for predictable memory usage
//! - 2D interaction matrices for fast element interaction lookups
//! - Thread-safe concurrent access with `Arc<RwLock<T>>`
//! 
//! ### 🏗️ **Modular Architecture**
//! - **Core Module**: Primary data structures and elemental system logic
//! - **Unified Registry**: Central registry for all element definitions and interactions
//! - **Factory Module**: Builder patterns for creating elemental system instances
//! - **Config Module**: YAML configuration loading and validation
//! - **Aggregation Module**: Stats aggregation and caching
//! - **Contributor Module**: External system integration
//! - **Common Traits**: Standardized API patterns across all components
//! 
//! ### 📊 **Comprehensive Stats System**
//! - **Primary Stats**: Directly stored values (mastery levels, qi amounts, etc.)
//! - **Derived Stats**: Calculated from primary stats + base properties
//! - **50+ Derived Stats**: Power points, defense points, crit rates, status effects, etc.
//! - **Element Mastery Ranks**: 7-tier ranking system (Novice to Transcendent)
//! 
//! ### 🔄 **Element Interactions**
//! - **Tương Sinh Tương Khắc**: Traditional Chinese elemental relationships
//! - **Generating, Overcoming, Neutral, Same, Opposite** interaction types
//! - **Matrix-based Lookups**: O(1) interaction factor retrieval
//! - **Configurable Bonuses**: Custom interaction multipliers
//! 
//! ### 🛠️ **Developer Experience**
//! - **Comprehensive Validation**: All data structures validate their integrity
//! - **Detailed Error Messages**: Context-rich error reporting with helpful descriptions
//! - **Common Traits**: Consistent API patterns across all components
//! - **Thread Safety**: Safe concurrent access to all data structures
//! - **Serialization Support**: JSON/YAML serialization for persistence
//! 
//! ## Quick Start
//! 
//! ```rust
//! use element_core::{ElementalFactory, UnifiedElementRegistry, ElementalSystemData};
//! use std::sync::Arc;
//! 
//! // Create a registry
//! let registry = Arc::new(UnifiedElementRegistry::new());
//! 
//! // Create a factory
//! let factory = ElementalFactory::new(registry);
//! 
//! // Create an elemental system
//! let system = factory.create_elemental_system();
//! 
//! // Access elemental data
//! let data = system.get_data();
//! println!("Total mastery: {}", data.get_total_elemental_mastery());
//! ```
//! 
//! ## Architecture Principles
//! 
//! 1. **Data Hub Pattern**: Element-Core acts as a central data aggregator
//! 2. **External Contributors**: Other systems register and contribute data
//! 3. **Single Responsibility**: Each module has a clear, focused purpose
//! 4. **Performance First**: Optimized for game loop performance requirements
//! 5. **Thread Safety**: All operations are safe for concurrent access
//! 6. **Validation**: Comprehensive data integrity checking
//! 7. **Extensibility**: Easy to add new elements and interaction types

pub mod core;
pub mod factory;
pub mod config;
pub mod aggregation;
pub mod adapters;
pub mod contributor;
pub mod unified_registry;
pub mod common_traits;

// Re-export core types
pub use core::elemental_data::{
    ElementMasteryLevel, MasteryLevelTier, ElementMasteryRealm, 
    ElementMasteryStage, ElementalPowerTier, ExperienceTier,
    ElementalSystemData, MAX_ELEMENTS
};

// Re-export commonly used types from core module
pub use core::{
    ElementalSystem, ElementConfig, ElementRegistry,
    ElementDefinition, ElementAliases, BaseProperties, ElementReferences
};

// Note: registry module removed - using unified_registry instead

// Re-export from factory module
pub use factory::{
    ElementalFactory, ElementalSystemBuilder
};

// Re-export from config module
pub use config::{
    ElementConfigLoader, YamlConfigLoader, ConfigValidationRule,
    InteractionConfig, ProbabilityConfig, StatusPoolConfig
};

// Re-export from contributor module
pub use contributor::{
    ElementContributor, ElementContribution, ElementContributorRegistry,
    ElementEvent, ContributorMetadata
};

// Re-export from unified_registry module
pub use unified_registry::{
    UnifiedElementRegistry, ElementCategory, SystemRegistration,
    SystemCapability, SystemHealth, ElementPlugin, ElementInteraction,
    RegistryConfig, RegistryMetrics, ElementProperties, DerivedStatConfig,
    StatusEffectConfig, SpreadRules, EnvironmentMod
};

// Re-export from aggregation module
pub use aggregation::{
    ElementAggregator, AggregationStrategy, ElementCache, CacheStats,
    AggregatorMetrics, CacheConfig, EvictionPolicy
};

// Re-export common traits
pub use common_traits::{
    ElementGetter, ElementSetter, Validatable, Cacheable,
    MetricsProvider, Configurable, Serializable, ElementHelper
};

/// Parameters for configuring elemental system when creating an actor
#[derive(Debug, Clone)]
pub struct ElementalParams {
    /// Primary element specialization
    pub primary_element: String,
    /// Initial mastery levels for each element
    pub initial_mastery_levels: std::collections::HashMap<String, f64>,
    /// Initial experience for each element
    pub initial_experience: std::collections::HashMap<String, f64>,
    /// Initial qi amounts for each element
    pub initial_qi_amounts: std::collections::HashMap<String, f64>,
    /// Elemental preferences (order of preference)
    pub elemental_preferences: Vec<String>,
}

use std::sync::Arc;

/// Element Core Result type
pub type ElementCoreResult<T> = Result<T, ElementCoreError>;

/// Element Core Error type
/// 
/// This enum provides comprehensive error handling for the element-core system
/// with detailed context and helpful error messages.
#[derive(Debug, thiserror::Error)]
pub enum ElementCoreError {
    /// Configuration-related errors
    /// 
    /// This error occurs when there are issues with configuration loading,
    /// parsing, or validation. It includes context about what configuration
    /// was being processed and what went wrong.
    #[error("Configuration error: {message}")]
    Config { message: String },
    
    /// Registry-related errors
    /// 
    /// This error occurs when there are issues with the element registry,
    /// such as failed registrations, missing elements, or registry corruption.
    #[error("Registry error: {message}")]
    Registry { message: String },
    
    /// Factory-related errors
    /// 
    /// This error occurs when there are issues with the elemental factory,
    /// such as failed element creation or invalid factory configuration.
    #[error("Factory error: {message}")]
    Factory { message: String },
    
    /// Validation-related errors
    /// 
    /// This error occurs when data validation fails. It includes detailed
    /// information about what validation rule was violated and why.
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    /// Index out of bounds errors
    /// 
    /// This error occurs when trying to access an array or collection
    /// with an index that is outside the valid range.
    #[error("Index out of bounds: attempted to access index {index} but maximum allowed index is {max}")]
    IndexOutOfBounds { 
        /// The index that was attempted
        index: usize, 
        /// The maximum allowed index
        max: usize 
    },
    
    /// Element not found errors
    /// 
    /// This error occurs when trying to access an element that doesn't exist
    /// in the registry or when an element ID is invalid.
    #[error("Element not found: '{element_id}' - this element does not exist in the registry or the ID is invalid")]
    ElementNotFound { 
        /// The element ID that was not found
        element_id: String 
    },
    
    /// Invalid element configuration errors
    /// 
    /// This error occurs when an element configuration is malformed,
    /// missing required fields, or contains invalid values.
    #[error("Invalid element configuration: {message}")]
    InvalidElementConfig { message: String },
    
    /// IO-related errors
    /// 
    /// This error occurs when there are file system or network I/O issues,
    /// such as file not found, permission denied, or network timeouts.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Serialization errors
    /// 
    /// This error occurs when there are issues with JSON serialization
    /// or deserialization, such as malformed JSON or type mismatches.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// YAML parsing errors
    /// 
    /// This error occurs when there are issues with YAML parsing,
    /// such as malformed YAML syntax or invalid YAML structure.
    #[error("YAML parsing error: {0}")]
    YamlParsing(#[from] serde_yaml::Error),
}

// Note: From<serde_yaml::Error> is automatically implemented by thiserror

impl From<String> for ElementCoreError {
    fn from(error: String) -> Self {
        ElementCoreError::Config { message: error }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_core_creation() {
        let registry = ElementRegistry::new();
        assert_eq!(registry.element_count(), 0);
    }

    #[test]
    fn test_factory_creation() {
        let registry = Arc::new(UnifiedElementRegistry::new());
        let factory = ElementalFactory::new(registry);
        let system = factory.create_elemental_system();
        assert!(system.get_data().element_mastery_levels[0] == 0.0);
    }

    #[test]
    fn test_elemental_params_creation() {
        let mut mastery_levels = std::collections::HashMap::new();
        mastery_levels.insert("fire".to_string(), 10.0);
        mastery_levels.insert("water".to_string(), 5.0);
        
        let mut experience = std::collections::HashMap::new();
        experience.insert("fire".to_string(), 100.0);
        
        let mut qi_amounts = std::collections::HashMap::new();
        qi_amounts.insert("fire".to_string(), 500.0);
        
        let params = ElementalParams {
            primary_element: "fire".to_string(),
            initial_mastery_levels: mastery_levels,
            initial_experience: experience,
            initial_qi_amounts: qi_amounts,
            elemental_preferences: vec!["fire".to_string(), "water".to_string()],
        };
        
        assert_eq!(params.primary_element, "fire");
        assert_eq!(params.initial_mastery_levels.get("fire"), Some(&10.0));
        assert_eq!(params.elemental_preferences.len(), 2);
    }
}
