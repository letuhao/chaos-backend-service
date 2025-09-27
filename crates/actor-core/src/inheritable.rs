//! Inheritable types and traits for extending actor-core functionality.
//!
//! This module provides the foundation for creating extensible actor systems
//! that can be inherited and extended by other modules.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Base trait for all inheritable actor components
pub trait InheritableComponent {
    /// Get the component type identifier
    fn component_type(&self) -> &str;
    
    /// Get the component version
    fn version(&self) -> &str;
    
    /// Check if this component is compatible with another version
    fn is_compatible_with(&self, other_version: &str) -> bool;
}

/// Inheritable actor extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritableActorExtension {
    /// Extension ID
    pub id: String,
    /// Extension type
    pub extension_type: String,
    /// Extension data
    pub data: HashMap<String, serde_json::Value>,
    /// Version
    pub version: String,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl InheritableActorExtension {
    /// Create a new inheritable actor extension
    pub fn new(id: String, extension_type: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            extension_type,
            data: HashMap::new(),
            version: "1.0.0".to_string(),
            dependencies: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Add extension data
    pub fn add_data(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
        self.updated_at = Utc::now();
    }
    
    /// Get extension data
    pub fn get_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }
}

impl InheritableComponent for InheritableActorExtension {
    fn component_type(&self) -> &str {
        &self.extension_type
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn is_compatible_with(&self, other_version: &str) -> bool {
        // Simple version compatibility check
        // In a real implementation, this would be more sophisticated
        self.version == other_version
    }
}

/// Inheritable subsystem extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritableSubsystemExtension {
    /// Extension ID
    pub id: String,
    /// Base subsystem ID
    pub base_subsystem_id: String,
    /// Extension configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Extension methods
    pub methods: Vec<String>,
    /// Version
    pub version: String,
    /// Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl InheritableSubsystemExtension {
    /// Create a new inheritable subsystem extension
    pub fn new(id: String, base_subsystem_id: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            base_subsystem_id,
            config: HashMap::new(),
            methods: Vec::new(),
            version: "1.0.0".to_string(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Add extension method
    pub fn add_method(&mut self, method: String) {
        self.methods.push(method);
        self.updated_at = Utc::now();
    }
    
    /// Add configuration
    pub fn add_config(&mut self, key: String, value: serde_json::Value) {
        self.config.insert(key, value);
        self.updated_at = Utc::now();
    }
}

impl InheritableComponent for InheritableSubsystemExtension {
    fn component_type(&self) -> &str {
        "subsystem_extension"
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn is_compatible_with(&self, other_version: &str) -> bool {
        self.version == other_version
    }
}

/// Registry for inheritable components
pub struct InheritableComponentRegistry {
    /// Registered components
    pub components: HashMap<String, Box<dyn InheritableComponent + Send + Sync>>,
    /// Component metadata
    pub metadata: HashMap<String, ComponentMetadata>,
}

/// Component metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetadata {
    /// Component ID
    pub id: String,
    /// Component type
    pub component_type: String,
    /// Version
    pub version: String,
    /// Description
    pub description: String,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
}

impl InheritableComponentRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Register a component
    pub fn register_component(&mut self, id: String, component: Box<dyn InheritableComponent + Send + Sync>, metadata: ComponentMetadata) {
        self.components.insert(id.clone(), component);
        self.metadata.insert(id, metadata);
    }
    
    /// Get a component
    pub fn get_component(&self, id: &str) -> Option<&Box<dyn InheritableComponent + Send + Sync>> {
        self.components.get(id)
    }
    
    /// Get component metadata
    pub fn get_metadata(&self, id: &str) -> Option<&ComponentMetadata> {
        self.metadata.get(id)
    }
}

impl Default for InheritableComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
