//! Registry implementations for the Actor Core system.
//!
//! This module contains the concrete implementations of the registry traits
//! including plugin registry, combiner registry, and cap layer registry.

pub mod loader;

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

use crate::interfaces::{PluginRegistry, CombinerRegistry, CapLayerRegistry, CombinerRegistryAsync, CapLayerRegistryAsync, Subsystem as SubsystemTrait, MergeRule, AcrossLayerPolicy};
use crate::types::*;
use crate::ActorCoreResult;


/// Wrapper to convert Arc<dyn SubsystemTrait> to Box<dyn SubsystemTrait>
struct SubsystemWrapper {
    inner: Arc<dyn SubsystemTrait>,
}

#[async_trait::async_trait]
impl SubsystemTrait for SubsystemWrapper {
    fn system_id(&self) -> &str {
        self.inner.system_id()
    }
    
    fn priority(&self) -> i64 {
        self.inner.priority()
    }
    
    async fn contribute(&self, actor: &Actor) -> ActorCoreResult<SubsystemOutput> {
        self.inner.contribute(actor).await
    }
}

/// PluginRegistryImpl is the implementation of the PluginRegistry trait.
pub struct PluginRegistryImpl {
    /// Map of system ID to subsystem
    subsystems: Arc<std::sync::RwLock<HashMap<String, Arc<dyn SubsystemTrait>>>>,
    /// Metrics for performance monitoring
    #[allow(dead_code)]
    metrics: Arc<std::sync::RwLock<RegistryMetrics>>,
}

impl PluginRegistryImpl {
    /// Create a new plugin registry instance.
    pub fn new() -> Self {
        Self {
            subsystems: Arc::new(std::sync::RwLock::new(HashMap::new())),
            metrics: Arc::new(std::sync::RwLock::new(RegistryMetrics::default())),
        }
    }

    /// Get all subsystems sorted by priority.
    fn get_subsystems_by_priority(&self) -> Vec<Box<dyn SubsystemTrait>> {
        let subsystems = self.subsystems.read().unwrap();
        let mut subsystem_list: Vec<Box<dyn SubsystemTrait>> = Vec::new();
        
        // Convert Arc<dyn SubsystemTrait> to Box<dyn SubsystemTrait>
        // This is a limitation of the current design, but we can work around it
        // by cloning the Arc and then converting to Box
        for (_, subsystem) in subsystems.iter() {
            // We need to create a wrapper that implements the trait
            // Since we can't directly convert Arc to Box, we'll create a wrapper
            let wrapper = SubsystemWrapper {
                inner: subsystem.clone(),
            };
            subsystem_list.push(Box::new(wrapper));
        }
        
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
    fn register(&self, subsystem: Box<dyn SubsystemTrait>) -> ActorCoreResult<()> {
        let system_id = subsystem.system_id().to_string();
        
        if system_id.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "System ID cannot be empty".to_string()
            ));
        }

        let mut subsystems = self.subsystems.write().unwrap();
        // Convert Box to Arc
        let subsystem_arc = Arc::from(subsystem);
        
        if subsystems.contains_key(&system_id) {
            warn!("Overwriting existing subsystem: {}", system_id);
        }
        
        subsystems.insert(system_id.clone(), subsystem_arc);
        
        info!("Registered subsystem: {}", system_id);
        Ok(())
    }

    fn unregister(&self, system_id: &str) -> ActorCoreResult<()> {
        let mut subsystems = self.subsystems.write().unwrap();
        
        if subsystems.remove(system_id).is_some() {
            info!("Unregistered subsystem: {}", system_id);
            Ok(())
        } else {
            Err(crate::ActorCoreError::RegistryError(
                format!("Subsystem not found: {}", system_id)
            ))
        }
    }

    fn get_by_id(&self, system_id: &str) -> Option<Box<dyn SubsystemTrait>> {
        let subsystems = self.subsystems.read().unwrap();
        
        if let Some(subsystem) = subsystems.get(system_id) {
            // Create a wrapper to convert Arc<dyn SubsystemTrait> to Box<dyn SubsystemTrait>
            let wrapper = SubsystemWrapper {
                inner: subsystem.clone(),
            };
            Some(Box::new(wrapper))
        } else {
            None
        }
    }

    fn get_by_priority(&self) -> Vec<Box<dyn SubsystemTrait>> {
        self.get_subsystems_by_priority()
    }

    fn get_by_priority_range(&self, min_priority: i64, max_priority: i64) -> Vec<Box<dyn SubsystemTrait>> {
        let subsystems = self.subsystems.read().unwrap();
        let mut subsystem_list: Vec<Box<dyn SubsystemTrait>> = Vec::new();
        
        // Filter subsystems by priority range
        for (_, subsystem) in subsystems.iter() {
            let priority = subsystem.priority();
            if priority >= min_priority && priority <= max_priority {
                let wrapper = SubsystemWrapper {
                    inner: subsystem.clone(),
                };
                subsystem_list.push(Box::new(wrapper));
            }
        }
        
        // Sort by priority (higher priority first)
        subsystem_list.sort_by(|a, b| b.priority().cmp(&a.priority()));
        subsystem_list
    }

    fn is_registered(&self, system_id: &str) -> bool {
        let subsystems = self.subsystems.read().unwrap();
        subsystems.contains_key(system_id)
    }

    fn count(&self) -> usize {
        let subsystems = self.subsystems.read().unwrap();
        subsystems.len()
    }

    fn validate_all(&self) -> ActorCoreResult<()> {
        let subsystems = self.subsystems.read().unwrap();
        
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
    rules: Arc<std::sync::RwLock<HashMap<String, MergeRule>>>,
    /// Metrics for performance monitoring
    #[allow(dead_code)]
    metrics: Arc<std::sync::RwLock<CombinerMetrics>>,
}

impl CombinerRegistryImpl {
    /// Create a new combiner registry instance.
    pub fn new() -> Self {
        Self {
            rules: Arc::new(std::sync::RwLock::new(HashMap::new())),
            metrics: Arc::new(std::sync::RwLock::new(CombinerMetrics::default())),
        }
    }

    /// Load default rules for common dimensions.
    pub fn load_default_rules(&self) -> ActorCoreResult<()> {
        let mut rules = self.rules.write().unwrap();
        
        // Add default rules for primary dimensions
        let primary_dims = [
            crate::constants::primary_dimensions::STRENGTH,
            crate::constants::primary_dimensions::AGILITY,
            crate::constants::primary_dimensions::INTELLIGENCE,
            crate::constants::primary_dimensions::VITALITY,
            crate::constants::primary_dimensions::SPIRIT,
            crate::constants::primary_dimensions::LUCK,
            crate::constants::primary_dimensions::HEALTH,
            crate::constants::primary_dimensions::MANA,
            crate::constants::primary_dimensions::STAMINA,
            crate::constants::primary_dimensions::EXPERIENCE,
            crate::constants::primary_dimensions::LEVEL,
        ];
        
        for dimension in primary_dims {
            rules.insert(dimension.to_string(), MergeRule {
                use_pipeline: true,
                operator: crate::enums::Operator::Sum,
                clamp_default: crate::constants::clamp_ranges::get_range(dimension)
                    .map(|(min, max)| Caps::new(min, max)),
            });
        }
        
        // Add default rules for derived dimensions
        let derived_dims = [
            crate::constants::derived_dimensions::ATTACK_POWER,
            crate::constants::derived_dimensions::DEFENSE_POWER,
            crate::constants::derived_dimensions::CRITICAL_HIT_CHANCE,
            crate::constants::derived_dimensions::CRITICAL_HIT_DAMAGE,
            crate::constants::derived_dimensions::ATTACK_SPEED,
            crate::constants::derived_dimensions::MOVEMENT_SPEED,
            crate::constants::derived_dimensions::CASTING_SPEED,
            crate::constants::derived_dimensions::COOLDOWN_REDUCTION,
            crate::constants::derived_dimensions::LIFE_STEAL,
            crate::constants::derived_dimensions::MANA_STEAL,
            crate::constants::derived_dimensions::DAMAGE_REDUCTION,
            crate::constants::derived_dimensions::ELEMENTAL_RESISTANCE,
        ];
        
        for dimension in derived_dims {
            rules.insert(dimension.to_string(), MergeRule {
                use_pipeline: true,
                operator: crate::enums::Operator::Sum,
                clamp_default: crate::constants::clamp_ranges::get_range(dimension)
                    .map(|(min, max)| Caps::new(min, max)),
            });
        }
        
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
        let rules = self.rules.read().unwrap();
        rules.get(dimension).cloned()
    }

    fn set_rule(&self, dimension: &str, rule: MergeRule) -> ActorCoreResult<()> {
        if dimension.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Dimension cannot be empty".to_string()
            ));
        }
        
        let mut rules = self.rules.write().unwrap();
        rules.insert(dimension.to_string(), rule);
        
        info!("Set merge rule for dimension: {}", dimension);
        Ok(())
    }

    fn validate(&self) -> ActorCoreResult<()> {
        let rules = self.rules.read().unwrap();
        
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
    layer_order: Arc<std::sync::RwLock<Vec<String>>>,
    /// Policy for combining caps across layers
    across_layer_policy: Arc<std::sync::RwLock<AcrossLayerPolicy>>,
    /// Metrics for performance monitoring
    #[allow(dead_code)]
    metrics: Arc<std::sync::RwLock<CapLayerMetrics>>,
}

impl CapLayerRegistryImpl {
    /// Create a new cap layer registry instance.
    pub fn new() -> Self {
        Self {
            layer_order: Arc::new(std::sync::RwLock::new(vec![
                "realm".to_string(),
                "world".to_string(),
                "event".to_string(),
                "guild".to_string(),
                "total".to_string(),
            ])),
            across_layer_policy: Arc::new(std::sync::RwLock::new(AcrossLayerPolicy::Intersect)),
            metrics: Arc::new(std::sync::RwLock::new(CapLayerMetrics::default())),
        }
    }

    /// Load default layer configuration.
    pub fn load_default_config(&self) -> ActorCoreResult<()> {
        let mut layer_order = self.layer_order.write().unwrap();
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
        let layer_order = self.layer_order.read().unwrap();
        layer_order.clone()
    }

    fn set_layer_order(&self, order: Vec<String>) -> ActorCoreResult<()> {
        if order.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Layer order cannot be empty".to_string()
            ));
        }
        
        let mut layer_order = self.layer_order.write().unwrap();
        *layer_order = order;
        
        info!("Set layer order: {:?}", layer_order);
        Ok(())
    }

    fn get_across_layer_policy(&self) -> AcrossLayerPolicy {
        let policy = self.across_layer_policy.read().unwrap();
        *policy
    }

    fn set_across_layer_policy(&self, policy: AcrossLayerPolicy) {
        let mut current_policy = self.across_layer_policy.write().unwrap();
        *current_policy = policy;
        
        info!("Set across layer policy: {:?}", policy);
    }

    fn validate(&self) -> ActorCoreResult<()> {
        let layer_order = self.layer_order.read().unwrap();
        
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
        Arc::new(PluginRegistryImpl::new())
    }

    /// Create a new combiner registry instance.
    pub fn create_combiner_registry() -> Arc<dyn CombinerRegistry> {
        Arc::new(CombinerRegistryImpl::new())
    }

    /// Create a new cap layer registry instance.
    pub fn create_cap_layer_registry() -> Arc<dyn CapLayerRegistry> {
        Arc::new(CapLayerRegistryImpl::new())
    }
}
