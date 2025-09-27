//! Core Resource Accessors
//! 
//! This module provides direct accessors for core resources,
//! enabling optimal performance for commonly accessed resources
//! while maintaining the existing HashMap system for custom resources.

use crate::ActorCoreResult;
use crate::ActorCoreError;
use std::collections::HashMap;

/// Core resource accessor trait
/// 
/// This trait provides a unified interface for accessing
/// both core and custom resources with appropriate performance
/// characteristics for each type.
pub trait CoreResourceAccessors {
    // Core resource accessors (direct access for performance)
    fn get_health(&self) -> f64;
    fn set_health(&mut self, value: f64);
    fn get_mana(&self) -> f64;
    fn set_mana(&mut self, value: f64);
    fn get_stamina(&self) -> f64;
    fn set_stamina(&mut self, value: f64);
    fn get_qi(&self) -> f64;
    fn set_qi(&mut self, value: f64);
    fn get_experience(&self) -> f64;
    fn set_experience(&mut self, value: f64);
    fn get_level(&self) -> i64;
    fn set_level(&mut self, value: i64);
    
    // Generic access for core resources by index
    fn get_core_resource_by_index(&self, index: usize) -> Option<f64>;
    fn set_core_resource_by_index(&mut self, index: usize, value: f64) -> ActorCoreResult<()>;
    
    // Custom resource accessors (HashMap access for flexibility)
    fn get_custom_resource(&self, name: &str) -> Option<f64>;
    fn set_custom_resource(&mut self, name: &str, value: f64);
    fn remove_custom_resource(&mut self, name: &str) -> Option<f64>;
}

/// Core resource accessor implementation
/// 
/// This struct provides direct access to core resources
/// while maintaining compatibility with the existing system.
pub struct CoreResourceAccessor<'a> {
    /// Core resources array for fast access
    core_resources: &'a mut [f64; 9], // 9 core resources
    /// Custom resources HashMap for flexible access
    custom_resources: &'a mut HashMap<String, f64>,
}

impl<'a> CoreResourceAccessor<'a> {
    /// Create a new core resource accessor
    pub fn new(
        core_resources: &'a mut [f64; 9],
        custom_resources: &'a mut HashMap<String, f64>,
    ) -> Self {
        Self {
            core_resources,
            custom_resources,
        }
    }
    
    /// Get health value
    pub fn get_health(&self) -> f64 {
        self.core_resources[0]
    }
    
    /// Set health value
    pub fn set_health(&mut self, value: f64) {
        self.core_resources[0] = value;
    }
    
    /// Get mana value
    pub fn get_mana(&self) -> f64 {
        self.core_resources[1]
    }
    
    /// Set mana value
    pub fn set_mana(&mut self, value: f64) {
        self.core_resources[1] = value;
    }
    
    /// Get stamina value
    pub fn get_stamina(&self) -> f64 {
        self.core_resources[2]
    }
    
    /// Set stamina value
    pub fn set_stamina(&mut self, value: f64) {
        self.core_resources[2] = value;
    }
    
    /// Get qi value
    pub fn get_qi(&self) -> f64 {
        self.core_resources[3]
    }
    
    /// Set qi value
    pub fn set_qi(&mut self, value: f64) {
        self.core_resources[3] = value;
    }
    
    /// Get experience value
    pub fn get_experience(&self) -> f64 {
        self.core_resources[4]
    }
    
    /// Set experience value
    pub fn set_experience(&mut self, value: f64) {
        self.core_resources[4] = value;
    }
    
    /// Get level value
    pub fn get_level(&self) -> i64 {
        self.core_resources[5] as i64
    }
    
    /// Set level value
    pub fn set_level(&mut self, value: i64) {
        self.core_resources[5] = value as f64;
    }
    
    /// Get vitality value
    pub fn get_vitality(&self) -> f64 {
        self.core_resources[6]
    }
    
    /// Set vitality value
    pub fn set_vitality(&mut self, value: f64) {
        self.core_resources[6] = value;
    }
    
    /// Get spirit value
    pub fn get_spirit(&self) -> f64 {
        self.core_resources[7]
    }
    
    /// Set spirit value
    pub fn set_spirit(&mut self, value: f64) {
        self.core_resources[7] = value;
    }
    
    /// Get chi value
    pub fn get_chi(&self) -> f64 {
        self.core_resources[8]
    }
    
    /// Set chi value
    pub fn set_chi(&mut self, value: f64) {
        self.core_resources[8] = value;
    }
    
    /// Get core resource by index
    pub fn get_core_resource_by_index(&self, index: usize) -> Option<f64> {
        if index < self.core_resources.len() {
            Some(self.core_resources[index])
        } else {
            None
        }
    }
    
    /// Set core resource by index
    pub fn set_core_resource_by_index(&mut self, index: usize, value: f64) -> ActorCoreResult<()> {
        if index < self.core_resources.len() {
            self.core_resources[index] = value;
            Ok(())
        } else {
            Err(ActorCoreError::ConfigurationError(
                format!("Invalid resource index: {}", index)
            ))
        }
    }
    
    /// Get custom resource
    pub fn get_custom_resource(&self, name: &str) -> Option<f64> {
        self.custom_resources.get(name).copied()
    }
    
    /// Set custom resource
    pub fn set_custom_resource(&mut self, name: &str, value: f64) {
        self.custom_resources.insert(name.to_string(), value);
    }
    
    /// Remove custom resource
    pub fn remove_custom_resource(&mut self, name: &str) -> Option<f64> {
        self.custom_resources.remove(name)
    }
}