//! Registry implementations for the Actor Core system.
//!
//! This module contains the concrete implementations of the registry traits
//! including plugin registry, combiner registry, and cap layer registry.

pub mod loader;
pub mod optimized;
pub mod runtime_registries;
// Legacy subsystem_registration moved to examples/legacy_subsystems/

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, warn};

use crate::interfaces::{PluginRegistry, CombinerRegistry, CapLayerRegistry, CombinerRegistryAsync, CapLayerRegistryAsync, Subsystem as SubsystemTrait, MergeRule};
use crate::enums::AcrossLayerPolicy;
// use crate::types::*; // Unused import
use crate::ActorCoreResult;



/// PluginRegistryImpl is the implementation of the PluginRegistry trait.
pub struct PluginRegistryImpl {
    /// Map of system ID to subsystem
    subsystems: Arc<RwLock<HashMap<String, Arc<dyn SubsystemTrait>>>>,
    /// Metrics for performance monitoring
    #[allow(dead_code)]
    metrics: Arc<RwLock<RegistryMetrics>>,
}

impl PluginRegistryImpl {
    /// Create a new plugin registry instance.
    pub fn new() -> Self {
        Self {
            subsystems: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(RegistryMetrics::default())),
        }
    }

    /// Get all subsystems sorted by priority.
    fn get_subsystems_by_priority(&self) -> Vec<Arc<dyn SubsystemTrait>> {
        let subsystems = self.subsystems.read();
        let mut subsystem_list: Vec<Arc<dyn SubsystemTrait>> = subsystems.values().cloned().collect();
        
        // Sort by priority (higher priority first)
        subsystem_list.sort_by(|a, b| b.priority().cmp(&a.priority()));
        subsystem_list
    }
}

impl Default for PluginRegistryImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PluginRegistry for PluginRegistryImpl {
    fn register(&self, subsystem: Arc<dyn SubsystemTrait>) -> ActorCoreResult<()> {
        let system_id = subsystem.system_id().to_string();
        
        if system_id.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "System ID cannot be empty".to_string()
            ));
        }

        let mut subsystems = self.subsystems.write();
        
        if subsystems.contains_key(&system_id) {
            warn!("Overwriting existing subsystem: {}", system_id);
        }
        
        subsystems.insert(system_id.clone(), subsystem);
        
        info!("Registered subsystem: {}", system_id);
        Ok(())
    }

    fn unregister(&self, system_id: &str) -> ActorCoreResult<()> {
        let mut subsystems = self.subsystems.write();
        
        if subsystems.remove(system_id).is_some() {
            info!("Unregistered subsystem: {}", system_id);
            Ok(())
        } else {
            Err(crate::ActorCoreError::RegistryError(
                format!("Subsystem not found: {}", system_id)
            ))
        }
    }

    fn get_by_id(&self, system_id: &str) -> Option<Arc<dyn SubsystemTrait>> {
        let subsystems = self.subsystems.read();
        subsystems.get(system_id).cloned()
    }

    fn get_by_priority(&self) -> Vec<Arc<dyn SubsystemTrait>> {
        self.get_subsystems_by_priority()
    }

    fn get_by_priority_range(&self, min_priority: i64, max_priority: i64) -> Vec<Arc<dyn SubsystemTrait>> {
        let subsystems = self.subsystems.read();
        let mut subsystem_list: Vec<Arc<dyn SubsystemTrait>> = Vec::new();
        
        // Filter subsystems by priority range
        for (_, subsystem) in subsystems.iter() {
            let priority = subsystem.priority();
            if priority >= min_priority && priority <= max_priority {
                subsystem_list.push(subsystem.clone());
            }
        }
        
        // Sort by priority (higher priority first)
        subsystem_list.sort_by(|a, b| b.priority().cmp(&a.priority()));
        subsystem_list
    }

    fn is_registered(&self, system_id: &str) -> bool {
        let subsystems = self.subsystems.read();
        subsystems.contains_key(system_id)
    }

    fn count(&self) -> usize {
        let subsystems = self.subsystems.read();
        subsystems.len()
    }

    fn validate_all(&self) -> ActorCoreResult<()> {
        let subsystems = self.subsystems.read();
        
        for (system_id, subsystem) in subsystems.iter() {
            if system_id.is_empty() {
                return Err(crate::ActorCoreError::ConfigurationError(
                    "Empty system ID found".to_string()
                ));
            }
            
            if subsystem.priority() < 0 {
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Invalid priority for subsystem {}: {}", system_id, subsystem.priority())
                ));
            }
        }
        
        Ok(())
    }
}

/// RegistryMetrics contains performance metrics for the registry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegistryMetrics {
    /// Number of registered subsystems
    pub registered_count: usize,
    /// Number of registration attempts
    pub registration_attempts: u64,
    /// Number of unregistration attempts
    pub unregistration_attempts: u64,
    /// Number of lookup attempts
    pub lookup_attempts: u64,
    /// Number of validation attempts
    pub validation_attempts: u64,
}

impl Default for RegistryMetrics {
    fn default() -> Self {
        Self {
            registered_count: 0,
            registration_attempts: 0,
            unregistration_attempts: 0,
            lookup_attempts: 0,
            validation_attempts: 0,
        }
    }
}

/// CombinerRegistryImpl is the implementation of the CombinerRegistry trait.
pub struct CombinerRegistryImpl {
    /// Map of dimension to merge rule
    rules: Arc<RwLock<HashMap<String, MergeRule>>>,
    /// Metrics for performance monitoring
    #[allow(dead_code)]
    metrics: Arc<RwLock<CombinerMetrics>>,
}

impl CombinerRegistryImpl {
    /// Create a new combiner registry instance.
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(CombinerMetrics::default())),
        }
    }

    /// Load default rules for common dimensions.
    /// 
    /// NOTE: This method is deprecated. Merge rules should now be registered
    /// dynamically by subsystems through the Runtime Registry system.
    /// 
    /// Subsystems should register their dimensions and merge rules via:
    /// - RegistryManager::register_resource()
    /// - RegistryManager::register_category() 
    /// - RegistryManager::register_tag()
    pub fn load_default_rules(&self) -> ActorCoreResult<()> {
        // No hardcoded rules - all rules are now registered by subsystems
        // through the Runtime Registry system at startup
        Ok(())
    }
}

impl Default for CombinerRegistryImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CombinerRegistry for CombinerRegistryImpl {
    fn get_rule(&self, dimension: &str) -> Option<MergeRule> {
        let rules = self.rules.read();
        rules.get(dimension).cloned()
    }

    fn set_rule(&self, dimension: &str, rule: MergeRule) -> ActorCoreResult<()> {
        if dimension.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Dimension cannot be empty".to_string()
            ));
        }
        
        let mut rules = self.rules.write();
        rules.insert(dimension.to_string(), rule);
        
        info!("Set merge rule for dimension: {}", dimension);
        Ok(())
    }

    fn validate(&self) -> ActorCoreResult<()> {
        let rules = self.rules.read();
        
        for (dimension, rule) in rules.iter() {
            if dimension.is_empty() {
                return Err(crate::ActorCoreError::ConfigurationError(
                    "Empty dimension found".to_string()
                ));
            }
            
            if let Some(caps) = &rule.clamp_default {
                if !caps.is_valid() {
                    return Err(crate::ActorCoreError::ConfigurationError(
                        format!("Invalid clamp range for dimension {}: {:?}", dimension, caps)
                    ));
                }
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl CombinerRegistryAsync for CombinerRegistryImpl {
    async fn load_from_file(&self, path: &str) -> ActorCoreResult<()> {
        // This is a simplified implementation
        // In a real implementation, this would load from a JSON or TOML file
        warn!("Loading from file not implemented: {}", path);
        Ok(())
    }

    async fn save_to_file(&self, path: &str) -> ActorCoreResult<()> {
        // This is a simplified implementation
        // In a real implementation, this would save to a JSON or TOML file
        warn!("Saving to file not implemented: {}", path);
        Ok(())
    }
}

/// CombinerMetrics contains performance metrics for the combiner registry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CombinerMetrics {
    /// Number of rules
    pub rule_count: usize,
    /// Number of rule lookups
    pub lookup_count: u64,
    /// Number of rule sets
    pub set_count: u64,
    /// Number of validation attempts
    pub validation_count: u64,
}

impl Default for CombinerMetrics {
    fn default() -> Self {
        Self {
            rule_count: 0,
            lookup_count: 0,
            set_count: 0,
            validation_count: 0,
        }
    }
}

/// CapLayerRegistryImpl is the implementation of the CapLayerRegistry trait.
pub struct CapLayerRegistryImpl {
    /// Order of layers for cap processing
    layer_order: Arc<RwLock<Vec<String>>>,
    /// Policy for combining caps across layers
    across_layer_policy: Arc<RwLock<AcrossLayerPolicy>>,
    /// Metrics for performance monitoring
    #[allow(dead_code)]
    metrics: Arc<RwLock<CapLayerMetrics>>,
}

impl CapLayerRegistryImpl {
    /// Create a new cap layer registry instance.
    pub fn new() -> Self {
        Self {
            layer_order: Arc::new(RwLock::new(vec![
                "realm".to_string(),
                "world".to_string(),
                "event".to_string(),
                "guild".to_string(),
                "total".to_string(),
            ])),
            across_layer_policy: Arc::new(RwLock::new(AcrossLayerPolicy::Intersect)),
            metrics: Arc::new(RwLock::new(CapLayerMetrics::default())),
        }
    }

    /// Load default layer configuration.
    /// TODO: Load layer order from configuration instead of hardcoded values
    pub fn load_default_config(&self) -> ActorCoreResult<()> {
        let mut layer_order = self.layer_order.write();
        // TODO: Load these layer names from configuration
        *layer_order = vec![
            "realm".to_string(),
            "world".to_string(),
            "event".to_string(),
            "guild".to_string(),
            "total".to_string(),
        ];
        
        Ok(())
    }
}

impl Default for CapLayerRegistryImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CapLayerRegistry for CapLayerRegistryImpl {
    fn get_layer_order(&self) -> Vec<String> {
        let layer_order = self.layer_order.read();
        layer_order.clone()
    }

    fn set_layer_order(&self, order: Vec<String>) -> ActorCoreResult<()> {
        if order.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Layer order cannot be empty".to_string()
            ));
        }
        
        let mut layer_order = self.layer_order.write();
        *layer_order = order;
        
        info!("Set layer order: {:?}", layer_order);
        Ok(())
    }

    fn get_across_layer_policy(&self) -> AcrossLayerPolicy {
        let policy = self.across_layer_policy.read();
        *policy
    }

    fn set_across_layer_policy(&self, policy: AcrossLayerPolicy) {
        let mut current_policy = self.across_layer_policy.write();
        *current_policy = policy;
        
        info!("Set across layer policy: {:?}", policy);
    }

    fn validate(&self) -> ActorCoreResult<()> {
        let layer_order = self.layer_order.read();
        
        if layer_order.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Layer order cannot be empty".to_string()
            ));
        }
        
        for layer in layer_order.iter() {
            if layer.is_empty() {
                return Err(crate::ActorCoreError::ConfigurationError(
                    "Layer name cannot be empty".to_string()
                ));
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl CapLayerRegistryAsync for CapLayerRegistryImpl {
    async fn load_from_file(&self, path: &str) -> ActorCoreResult<()> {
        // This is a simplified implementation
        // In a real implementation, this would load from a JSON or TOML file
        warn!("Loading from file not implemented: {}", path);
        Ok(())
    }

    async fn save_to_file(&self, path: &str) -> ActorCoreResult<()> {
        // This is a simplified implementation
        // In a real implementation, this would save to a JSON or TOML file
        warn!("Saving to file not implemented: {}", path);
        Ok(())
    }
}

/// CapLayerMetrics contains performance metrics for the cap layer registry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CapLayerMetrics {
    /// Number of layers
    pub layer_count: usize,
    /// Number of policy changes
    pub policy_changes: u64,
    /// Number of order changes
    pub order_changes: u64,
    /// Number of validation attempts
    pub validation_count: u64,
}

impl Default for CapLayerMetrics {
    fn default() -> Self {
        Self {
            layer_count: 0,
            policy_changes: 0,
            order_changes: 0,
            validation_count: 0,
        }
    }
}

/// Factory for creating registry instances.
pub struct RegistryFactory;

impl RegistryFactory {
    /// Create a new plugin registry instance.
    pub fn create_plugin_registry() -> Arc<dyn PluginRegistry> {
        let registry: Arc<dyn PluginRegistry> = Arc::new(PluginRegistryImpl::new());
        // Register default subsystems
        // Legacy resource manager registration removed - use Runtime Registry System instead
        registry
    }

    /// Create a new combiner registry instance.
    pub fn create_combiner_registry() -> Arc<dyn CombinerRegistry> {
        Arc::new(CombinerRegistryImpl::new())
    }

    /// Create a new cap layer registry instance.
    pub fn create_cap_layer_registry() -> Arc<dyn CapLayerRegistry> {
        Arc::new(CapLayerRegistryImpl::new())
    }
    
    /// Create a new registry manager instance.
    pub fn create_registry_manager() -> runtime_registries::RegistryManager {
        runtime_registries::RegistryManager::new()
    }
}

// Re-export runtime registry types for convenience
pub use runtime_registries::{
    ResourceRegistry, CategoryRegistry, TagRegistry,
    ResourceRegistryImpl, CategoryRegistryImpl, TagRegistryImpl,
    ResourceDefinition, CategoryDefinition, TagDefinition,
    ResourceType, RegenType, RegistryManager,
};

// Legacy subsystem_registration moved to examples/legacy_subsystems/
// Use Runtime Registry System for dynamic registration instead