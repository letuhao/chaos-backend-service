//! # Element Core
//! 
//! A high-performance elemental system for the Chaos World MMORPG backend service.
//! 
//! This crate provides:
//! - Array-based elemental data structures for high performance
//! - Element configuration loading from YAML files
//! - Element registry for managing element configurations
//! - Factory patterns for creating elemental system instances
//! - Configurable element properties and derived stats

pub mod core;
pub mod registry;
pub mod factory;
pub mod config;
pub mod aggregation;
pub mod adapters;

// Re-export commonly used types
pub use core::*;
pub use registry::*;
pub use factory::*;
pub use config::*;

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
#[derive(Debug, thiserror::Error)]
pub enum ElementCoreError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Registry error: {0}")]
    Registry(String),
    
    #[error("Factory error: {0}")]
    Factory(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Index out of bounds: {index} (max: {max})")]
    IndexOutOfBounds { index: usize, max: usize },
    
    #[error("Element not found: {element_id}")]
    ElementNotFound { element_id: String },
    
    #[error("Invalid element configuration: {0}")]
    InvalidElementConfig(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<serde_yaml::Error> for ElementCoreError {
    fn from(error: serde_yaml::Error) -> Self {
        ElementCoreError::Config(error.to_string())
    }
}

impl From<String> for ElementCoreError {
    fn from(error: String) -> Self {
        ElementCoreError::Config(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_core_creation() {
        let registry = ElementalRegistry::new();
        assert_eq!(registry.element_count().unwrap(), 0);
    }

    #[test]
    fn test_factory_creation() {
        let registry = Arc::new(ElementalRegistry::new());
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
