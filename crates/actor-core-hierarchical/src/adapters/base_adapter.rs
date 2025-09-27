//! # Base Adapter Trait
//! 
//! Base trait for all system-actor adapters.

use crate::core::HierarchicalActor;
use std::collections::HashMap;

/// Result type for adapter operations
pub type AdapterResult<T> = Result<T, String>;

/// Base trait for all system-actor adapters
pub trait BaseAdapter {
    /// Convert system data to hierarchical actor format
    fn to_hierarchical_actor(&self, system_data: &dyn SystemData) -> AdapterResult<HierarchicalActor>;
    
    /// Convert hierarchical actor to system data format
    fn from_hierarchical_actor(&self, actor: &HierarchicalActor) -> AdapterResult<Box<dyn SystemData>>;
    
    /// Validate system data
    fn validate_system_data(&self, system_data: &dyn SystemData) -> AdapterResult<()>;
    
    /// Validate hierarchical actor data
    fn validate_hierarchical_actor(&self, actor: &HierarchicalActor) -> AdapterResult<()>;
    
    /// Get adapter name
    fn get_adapter_name(&self) -> &str;
    
    /// Get supported system types
    fn get_supported_system_types(&self) -> Vec<String>;
}

/// Trait for system data that can be adapted
pub trait SystemData {
    /// Get system name
    fn get_system_name(&self) -> &str;
    
    /// Get system data as key-value pairs
    fn get_data_map(&self) -> HashMap<String, String>;
    
    /// Get system metadata
    fn get_metadata(&self) -> &HashMap<String, String>;
    
    /// Validate system data
    fn validate(&self) -> AdapterResult<()>;
    
    /// Clone the system data
    fn clone_system_data(&self) -> Box<dyn SystemData>;
}

/// Base adapter implementation with common functionality
#[derive(Debug)]
pub struct BaseAdapterImpl {
    adapter_name: String,
    supported_systems: Vec<String>,
}

impl BaseAdapterImpl {
    /// Create a new base adapter
    pub fn new(adapter_name: String, supported_systems: Vec<String>) -> Self {
        Self {
            adapter_name,
            supported_systems,
        }
    }
    
    /// Common validation logic
    pub fn validate_common(&self, system_data: &dyn SystemData) -> AdapterResult<()> {
        // Check if system type is supported
        if !self.supported_systems.contains(&system_data.get_system_name().to_string()) {
            return Err(format!(
                "System type '{}' is not supported by adapter '{}'",
                system_data.get_system_name(),
                self.adapter_name
            ));
        }
        
        // Validate system data
        system_data.validate()?;
        
        // Check if data map is not empty
        let data_map = system_data.get_data_map();
        if data_map.is_empty() {
            return Err("System data cannot be empty".to_string());
        }
        
        Ok(())
    }
    
    /// Common actor validation logic
    pub fn validate_actor_common(&self, actor: &HierarchicalActor) -> AdapterResult<()> {
        // Check if actor ID is not empty
        if actor.get_id().is_empty() {
            return Err("Actor ID cannot be empty".to_string());
        }
        
        // Check if actor name is not empty
        if actor.get_name().is_empty() {
            return Err("Actor name cannot be empty".to_string());
        }
        
        Ok(())
    }
}

impl BaseAdapter for BaseAdapterImpl {
    fn to_hierarchical_actor(&self, system_data: &dyn SystemData) -> AdapterResult<HierarchicalActor> {
        // Validate system data
        self.validate_common(system_data)?;
        
        // Create new hierarchical actor
        let mut actor = HierarchicalActor::with_id_and_name(
            uuid::Uuid::new_v4().to_string(),
            format!("{} Actor", system_data.get_system_name()),
        );
        
        // Copy metadata
        for (key, value) in system_data.get_metadata() {
            actor.set_metadata(key.clone(), value.clone());
        }
        
        Ok(actor)
    }
    
    fn from_hierarchical_actor(&self, actor: &HierarchicalActor) -> AdapterResult<Box<dyn SystemData>> {
        // Validate actor
        self.validate_actor_common(actor)?;
        
        // Create system data from actor
        let system_data = Box::new(SimpleSystemData {
            system_name: "hierarchical_actor".to_string(),
            data_map: actor.get_all_metadata().clone(),
            metadata: actor.get_all_metadata().clone(),
        });
        
        Ok(system_data)
    }
    
    fn validate_system_data(&self, system_data: &dyn SystemData) -> AdapterResult<()> {
        self.validate_common(system_data)
    }
    
    fn validate_hierarchical_actor(&self, actor: &HierarchicalActor) -> AdapterResult<()> {
        self.validate_actor_common(actor)
    }
    
    fn get_adapter_name(&self) -> &str {
        &self.adapter_name
    }
    
    fn get_supported_system_types(&self) -> Vec<String> {
        self.supported_systems.clone()
    }
}

/// Simple system data implementation for testing
#[derive(Debug, Clone)]
pub struct SimpleSystemData {
    pub system_name: String,
    pub data_map: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
}

impl SystemData for SimpleSystemData {
    fn get_system_name(&self) -> &str {
        &self.system_name
    }
    
    fn get_data_map(&self) -> HashMap<String, String> {
        self.data_map.clone()
    }
    
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
    
    fn validate(&self) -> AdapterResult<()> {
        if self.system_name.is_empty() {
            return Err("System name cannot be empty".to_string());
        }
        
        Ok(())
    }
    
    fn clone_system_data(&self) -> Box<dyn SystemData> {
        Box::new(self.clone())
    }
}

