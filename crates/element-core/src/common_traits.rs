//! # Common Traits
//! 
//! This module defines common traits used across the element-core system
//! to ensure consistent API patterns and behavior. These traits provide
//! a standardized interface for all elemental system components.
//! 
//! ## Design Principles
//! 
//! 1. **Consistency**: All components follow the same API patterns
//! 2. **Composability**: Traits can be combined for complex behaviors
//! 3. **Validation**: All data structures can be validated
//! 4. **Performance**: Traits are designed for high-frequency access
//! 5. **Thread Safety**: All operations are safe for concurrent access
//! 
//! ## Core Traits
//! 
//! ### `ElementGetter<T>` & `ElementSetter<T>`
//! - **Purpose**: Standardized element retrieval and modification
//! - **Usage**: All element containers implement these traits
//! - **Benefits**: Consistent API across different storage backends
//! 
//! ### `Validatable`
//! - **Purpose**: Data integrity validation
//! - **Features**: Comprehensive validation with detailed error messages
//! - **Usage**: All data structures implement validation
//! 
//! ### `Cacheable`
//! - **Purpose**: Cache management and statistics
//! - **Features**: Cache clearing, statistics, and eviction policies
//! - **Usage**: Performance-critical components
//! 
//! ### `MetricsProvider`
//! - **Purpose**: Performance monitoring and metrics collection
//! - **Features**: Metrics retrieval, reset, and display
//! - **Usage**: System monitoring and optimization
//! 
//! ### `Configurable`
//! - **Purpose**: Configuration management
//! - **Features**: Config retrieval, updates, and validation
//! - **Usage**: All configurable components
//! 
//! ### `Serializable`
//! - **Purpose**: Data persistence and serialization
//! - **Features**: JSON/YAML serialization support
//! - **Usage**: Data persistence and inter-system communication
//! 
//! ### `ElementHelper`
//! - **Purpose**: Utility functions for element operations
//! - **Features**: Identifier validation, cache key generation
//! - **Usage**: Common operations across all components
//! 
//! ## Usage Examples
//! 
//! ### Basic Element Operations
//! ```rust
//! // Get an element
//! let element = registry.get_element("fire")?;
//! 
//! // Set an element
//! registry.set_element("fire", new_element)?;
//! 
//! // Check if element exists
//! if registry.has_element("fire") {
//!     println!("Fire element is available");
//! }
//! ```
//! 
//! ### Validation
//! ```rust
//! // Validate a component
//! component.validate()?;
//! 
//! // Get validation errors
//! let errors = component.get_validation_errors();
//! if !errors.is_empty() {
//!     println!("Validation errors: {:?}", errors);
//! }
//! ```
//! 
//! ### Caching and Metrics
//! ```rust
//! // Clear cache
//! component.clear_cache();
//! 
//! // Get cache statistics
//! let stats = component.get_cache_stats();
//! 
//! // Get performance metrics
//! let metrics = component.get_metrics();
//! println!("{}", component.display_metrics());
//! ```

use crate::{ElementCoreResult, ElementCoreError};

/// Common trait for all element getters
/// 
/// This trait provides a consistent interface for retrieving elements
/// across different components in the element-core system.
pub trait ElementGetter<T> {
    /// Get an element by its identifier
    /// 
    /// # Arguments
    /// * `identifier` - The unique identifier of the element
    /// 
    /// # Returns
    /// * `Ok(T)` if the element is found
    /// * `Err(ElementCoreError::ElementNotFound)` if the element doesn't exist
    /// * `Err(ElementCoreError::Validation)` if the identifier is invalid
    fn get_element(&self, identifier: &str) -> ElementCoreResult<T>;
    
    /// Check if an element exists
    /// 
    /// # Arguments
    /// * `identifier` - The unique identifier of the element
    /// 
    /// # Returns
    /// * `true` if the element exists
    /// * `false` if the element doesn't exist
    fn has_element(&self, identifier: &str) -> bool;
    
    /// Get all element identifiers
    /// 
    /// # Returns
    /// * `Vec<String>` containing all element identifiers
    fn get_all_element_ids(&self) -> ElementCoreResult<Vec<String>>;
    
    /// Get the count of elements
    /// 
    /// # Returns
    /// * `usize` representing the number of elements
    fn element_count(&self) -> usize;
}

/// Common trait for all element setters
/// 
/// This trait provides a consistent interface for setting elements
/// across different components in the element-core system.
pub trait ElementSetter<T> {
    /// Set an element by its identifier
    /// 
    /// # Arguments
    /// * `identifier` - The unique identifier of the element
    /// * `element` - The element to set
    /// 
    /// # Returns
    /// * `Ok(())` if the element was set successfully
    /// * `Err(ElementCoreError::Validation)` if the identifier or element is invalid
    /// * `Err(ElementCoreError::Registry)` if there was a registry error
    fn set_element(&self, identifier: &str, element: T) -> ElementCoreResult<()>;
    
    /// Remove an element by its identifier
    /// 
    /// # Arguments
    /// * `identifier` - The unique identifier of the element
    /// 
    /// # Returns
    /// * `Ok(())` if the element was removed successfully
    /// * `Err(ElementCoreError::ElementNotFound)` if the element doesn't exist
    fn remove_element(&self, identifier: &str) -> ElementCoreResult<()>;
}

/// Common trait for all validatable objects
/// 
/// This trait provides a consistent interface for validation
/// across different components in the element-core system.
pub trait Validatable {
    /// Validate the object
    /// 
    /// # Returns
    /// * `Ok(())` if the object is valid
    /// * `Err(ElementCoreError::Validation)` if the object is invalid
    fn validate(&self) -> ElementCoreResult<()>;
    
    /// Get validation errors
    /// 
    /// # Returns
    /// * `Vec<String>` containing all validation error messages
    fn get_validation_errors(&self) -> Vec<String>;
}

/// Common trait for all cacheable objects
/// 
/// This trait provides a consistent interface for caching
/// across different components in the element-core system.
pub trait Cacheable {
    /// Get cache key for this object
    /// 
    /// # Returns
    /// * `String` representing the cache key
    fn cache_key(&self) -> String;
    
    /// Get cache TTL (time to live) in seconds
    /// 
    /// # Returns
    /// * `u64` representing the TTL in seconds
    fn cache_ttl(&self) -> u64;
    
    /// Check if this object should be cached
    /// 
    /// # Returns
    /// * `true` if the object should be cached
    /// * `false` if the object should not be cached
    fn should_cache(&self) -> bool;
}

/// Common trait for all metrics providers
/// 
/// This trait provides a consistent interface for metrics
/// across different components in the element-core system.
pub trait MetricsProvider {
    /// Get current metrics
    /// 
    /// # Returns
    /// * `HashMap<String, f64>` containing metric name -> value pairs
    fn get_metrics(&self) -> std::collections::HashMap<String, f64>;
    
    /// Reset metrics
    fn reset_metrics(&self);
    
    /// Get metrics summary
    /// 
    /// # Returns
    /// * `String` containing a human-readable metrics summary
    fn get_metrics_summary(&self) -> String;
}

/// Common trait for all configurable objects
/// 
/// This trait provides a consistent interface for configuration
/// across different components in the element-core system.
pub trait Configurable {
    /// Get configuration
    /// 
    /// # Returns
    /// * `serde_json::Value` containing the configuration
    fn get_config(&self) -> serde_json::Value;
    
    /// Update configuration
    /// 
    /// # Arguments
    /// * `config` - The new configuration
    /// 
    /// # Returns
    /// * `Ok(())` if the configuration was updated successfully
    /// * `Err(ElementCoreError::Config)` if the configuration is invalid
    fn update_config(&self, config: serde_json::Value) -> ElementCoreResult<()>;
    
    /// Validate configuration
    /// 
    /// # Arguments
    /// * `config` - The configuration to validate
    /// 
    /// # Returns
    /// * `Ok(())` if the configuration is valid
    /// * `Err(ElementCoreError::Config)` if the configuration is invalid
    fn validate_config(&self, config: &serde_json::Value) -> ElementCoreResult<()>;
}

/// Common trait for all serializable objects
/// 
/// This trait provides a consistent interface for serialization
/// across different components in the element-core system.
pub trait Serializable {
    /// Serialize to JSON
    /// 
    /// # Returns
    /// * `Ok(String)` if serialization was successful
    /// * `Err(ElementCoreError::Serialization)` if serialization failed
    fn to_json(&self) -> ElementCoreResult<String>;
    
    /// Serialize to YAML
    /// 
    /// # Returns
    /// * `Ok(String)` if serialization was successful
    /// * `Err(ElementCoreError::YamlParsing)` if serialization failed
    fn to_yaml(&self) -> ElementCoreResult<String>;
    
    /// Deserialize from JSON
    /// 
    /// # Arguments
    /// * `json` - The JSON string to deserialize
    /// 
    /// # Returns
    /// * `Ok(Self)` if deserialization was successful
    /// * `Err(ElementCoreError::Serialization)` if deserialization failed
    fn from_json(json: &str) -> ElementCoreResult<Self> where Self: Sized;
    
    /// Deserialize from YAML
    /// 
    /// # Arguments
    /// * `yaml` - The YAML string to deserialize
    /// 
    /// # Returns
    /// * `Ok(Self)` if deserialization was successful
    /// * `Err(ElementCoreError::YamlParsing)` if deserialization failed
    fn from_yaml(yaml: &str) -> ElementCoreResult<Self> where Self: Sized;
}

/// Helper trait for common operations
pub trait ElementHelper {
    /// Normalize element identifier
    /// 
    /// # Arguments
    /// * `identifier` - The identifier to normalize
    /// 
    /// # Returns
    /// * `String` containing the normalized identifier
    fn normalize_identifier(identifier: &str) -> String {
        identifier.to_lowercase().trim().to_string()
    }
    
    /// Validate element identifier
    /// 
    /// # Arguments
    /// * `identifier` - The identifier to validate
    /// 
    /// # Returns
    /// * `Ok(())` if the identifier is valid
    /// * `Err(ElementCoreError::Validation)` if the identifier is invalid
    fn validate_identifier(identifier: &str) -> ElementCoreResult<()> {
        if identifier.is_empty() {
            return Err(ElementCoreError::Validation { message: "Identifier cannot be empty".to_string() });
        }
        
        if identifier.len() > 100 {
            return Err(ElementCoreError::Validation { message: 
                format!("Identifier too long (max 100 chars), got {} chars", identifier.len())
            });
        }
        
        if !identifier.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(ElementCoreError::Validation { message: 
                "Identifier contains invalid characters (only alphanumeric, underscore, and dash allowed)".to_string()
            });
        }
        
        Ok(())
    }
    
    /// Generate element cache key
    /// 
    /// # Arguments
    /// * `prefix` - The prefix for the cache key
    /// * `identifier` - The element identifier
    /// 
    /// # Returns
    /// * `String` containing the cache key
    fn generate_cache_key(prefix: &str, identifier: &str) -> String {
        format!("{}:{}", prefix, Self::normalize_identifier(identifier))
    }
}
