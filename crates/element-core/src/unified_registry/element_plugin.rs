//! # Element Plugin
//! 
//! This module defines the element plugin system for extensible element functionality.

use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::{ElementCoreResult, ElementCoreError};
use crate::unified_registry::ElementDefinition;
use actor_core::Actor;

/// Element plugin trait for extensible element functionality
/// 
/// This trait allows external systems to provide plugin-based functionality
/// for elements, such as custom training methods, derived stats calculations,
/// and element interactions.
#[async_trait]
pub trait ElementPlugin: Send + Sync {
    /// Get element identifier
    fn get_element_id(&self) -> String;
    
    /// Get element definition
    fn get_element_definition(&self) -> ElementDefinition;
    
    /// Calculate base mastery for an actor
    fn calculate_base_mastery(&self, actor: &Actor) -> f64;
    
    /// Calculate decay rate for an actor
    fn calculate_decay_rate(&self, actor: &Actor) -> f64;
    
    /// Get opposite elements
    fn get_opposite_elements(&self) -> Vec<String>;
    
    /// Handle training for an actor
    async fn handle_training(&self, actor: &mut Actor, training_amount: f64) -> ElementCoreResult<()>;
    
    /// Get derived stats for this element
    fn get_derived_stats(&self, actor: &Actor) -> HashMap<String, f64>;
    
    /// Get training methods for this element
    fn get_training_methods(&self) -> Vec<TrainingMethod>;
    
    /// Get element interactions
    fn get_element_interactions(&self) -> HashMap<String, PluginElementInteraction>;
    
    /// Get plugin metadata
    fn get_metadata(&self) -> PluginMetadata;
    
    /// Initialize the plugin
    async fn initialize(&self) -> ElementCoreResult<()>;
    
    /// Shutdown the plugin
    async fn shutdown(&self) -> ElementCoreResult<()>;
    
    /// Check if plugin is healthy
    fn is_healthy(&self) -> bool;
}

/// Training method for element mastery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMethod {
    /// Method identifier
    pub id: String,
    
    /// Method name
    pub name: String,
    
    /// Method description
    pub description: String,
    
    /// Base experience gain
    pub base_experience_gain: f64,
    
    /// Experience multiplier
    pub experience_multiplier: f64,
    
    /// Required conditions
    pub required_conditions: Vec<String>,
    
    /// Method type
    pub method_type: TrainingMethodType,
    
    /// Cooldown time in seconds
    pub cooldown_seconds: u64,
    
    /// Resource cost
    pub resource_cost: HashMap<String, f64>,
    
    /// Success probability
    pub success_probability: f64,
    
    /// Failure consequences
    pub failure_consequences: Vec<String>,
}

/// Training method types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrainingMethodType {
    /// Meditation-based training
    Meditation,
    
    /// Combat-based training
    Combat,
    
    /// Crafting-based training
    Crafting,
    
    /// Exploration-based training
    Exploration,
    
    /// Social-based training
    Social,
    
    /// Ritual-based training
    Ritual,
    
    /// Study-based training
    Study,
    
    /// Practice-based training
    Practice,
}

/// Element interaction definition (Plugin version)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginElementInteraction {
    /// Interaction identifier
    pub id: String,
    
    /// Target element
    pub target_element: String,
    
    /// Interaction type
    pub interaction_type: PluginInteractionType,
    
    /// Base multiplier
    pub base_multiplier: f64,
    
    /// Scaling factor
    pub scaling_factor: f64,
    
    /// Maximum multiplier
    pub max_multiplier: f64,
    
    /// Minimum multiplier
    pub min_multiplier: f64,
    
    /// Special effects
    pub special_effects: Vec<String>,
    
    /// Conditions
    pub conditions: Vec<String>,
}

/// Element interaction types (Plugin version)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginInteractionType {
    /// Generating relationship (tương sinh)
    Generating,
    
    /// Overcoming relationship (tương khắc)
    Overcoming,
    
    /// Same element
    Same,
    
    /// Neutral relationship
    Neutral,
    
    /// Opposite relationship
    Opposite,
    
    /// Special interaction
    Special,
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin identifier
    pub plugin_id: String,
    
    /// Plugin name
    pub plugin_name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Plugin description
    pub description: String,
    
    /// Plugin author
    pub author: String,
    
    /// Plugin dependencies
    pub dependencies: Vec<String>,
    
    /// Plugin capabilities
    pub capabilities: Vec<PluginCapability>,
    
    /// Plugin configuration
    pub config: HashMap<String, serde_json::Value>,
    
    /// Plugin status
    pub status: PluginStatus,
    
    /// Plugin health
    pub health: PluginHealth,
}

/// Plugin capabilities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PluginCapability {
    /// Can provide training methods
    ProvideTrainingMethods,
    
    /// Can calculate derived stats
    CalculateDerivedStats,
    
    /// Can handle element interactions
    HandleElementInteractions,
    
    /// Can provide status effects
    ProvideStatusEffects,
    
    /// Can manage element mastery
    ManageElementMastery,
    
    /// Can provide custom formulas
    ProvideCustomFormulas,
    
    /// Can handle element events
    HandleElementEvents,
    
    /// Can provide element definitions
    ProvideElementDefinitions,
}

/// Plugin status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginStatus {
    /// Plugin is active
    Active,
    
    /// Plugin is inactive
    Inactive,
    
    /// Plugin is loading
    Loading,
    
    /// Plugin has errors
    Error,
    
    /// Plugin is deprecated
    Deprecated,
}

/// Plugin health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginHealth {
    /// Overall health score (0.0 - 1.0)
    pub health_score: f64,
    
    /// Last health check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
    
    /// Health check failures
    pub failure_count: u32,
    
    /// Health check successes
    pub success_count: u32,
    
    /// Plugin errors
    pub errors: Vec<String>,
    
    /// Plugin warnings
    pub warnings: Vec<String>,
}

impl TrainingMethod {
    /// Create a new training method
    pub fn new(
        id: String,
        name: String,
        description: String,
        method_type: TrainingMethodType,
    ) -> Self {
        Self {
            id,
            name,
            description,
            base_experience_gain: 100.0,
            experience_multiplier: 1.0,
            required_conditions: Vec::new(),
            method_type,
            cooldown_seconds: 0,
            resource_cost: HashMap::new(),
            success_probability: 1.0,
            failure_consequences: Vec::new(),
        }
    }
    
    /// Add a required condition
    pub fn add_required_condition(&mut self, condition: String) {
        if !self.required_conditions.contains(&condition) {
            self.required_conditions.push(condition);
        }
    }
    
    /// Add a resource cost
    pub fn add_resource_cost(&mut self, resource: String, cost: f64) {
        self.resource_cost.insert(resource, cost);
    }
    
    /// Add a failure consequence
    pub fn add_failure_consequence(&mut self, consequence: String) {
        if !self.failure_consequences.contains(&consequence) {
            self.failure_consequences.push(consequence);
        }
    }
    
    /// Calculate total experience gain
    pub fn calculate_experience_gain(&self, actor: &Actor) -> f64 {
        self.base_experience_gain * self.experience_multiplier
    }
    
    /// Check if all required conditions are met
    pub fn check_conditions(&self, actor: &Actor) -> bool {
        // TODO: Implement condition checking logic
        true
    }
    
    /// Check if actor has required resources
    pub fn check_resources(&self, actor: &Actor) -> bool {
        // TODO: Implement resource checking logic
        true
    }
}

impl PluginElementInteraction {
    /// Create a new element interaction
    pub fn new(
        id: String,
        target_element: String,
        interaction_type: PluginInteractionType,
    ) -> Self {
        Self {
            id,
            target_element,
            interaction_type,
            base_multiplier: 1.0,
            scaling_factor: 1.0,
            max_multiplier: 2.0,
            min_multiplier: 0.5,
            special_effects: Vec::new(),
            conditions: Vec::new(),
        }
    }
    
    /// Add a special effect
    pub fn add_special_effect(&mut self, effect: String) {
        if !self.special_effects.contains(&effect) {
            self.special_effects.push(effect);
        }
    }
    
    /// Add a condition
    pub fn add_condition(&mut self, condition: String) {
        if !self.conditions.contains(&condition) {
            self.conditions.push(condition);
        }
    }
    
    /// Calculate interaction multiplier
    pub fn calculate_multiplier(&self, mastery_level: f64) -> f64 {
        let multiplier = self.base_multiplier + (mastery_level * self.scaling_factor);
        multiplier.max(self.min_multiplier).min(self.max_multiplier)
    }
    
    /// Check if conditions are met
    pub fn check_conditions(&self, actor: &Actor) -> bool {
        // TODO: Implement condition checking logic
        true
    }
}

impl PluginMetadata {
    /// Create new plugin metadata
    pub fn new(
        plugin_id: String,
        plugin_name: String,
        version: String,
        description: String,
        author: String,
    ) -> Self {
        Self {
            plugin_id,
            plugin_name,
            version,
            description,
            author,
            dependencies: Vec::new(),
            capabilities: Vec::new(),
            config: HashMap::new(),
            status: PluginStatus::Active,
            health: PluginHealth::new(),
        }
    }
    
    /// Add a dependency
    pub fn add_dependency(&mut self, dependency: String) {
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }
    
    /// Add a capability
    pub fn add_capability(&mut self, capability: PluginCapability) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }
    
    /// Set configuration
    pub fn set_config(&mut self, key: String, value: serde_json::Value) {
        self.config.insert(key, value);
    }
    
    /// Get configuration
    pub fn get_config(&self, key: &str) -> Option<&serde_json::Value> {
        self.config.get(key)
    }
    
    /// Check if plugin has a capability
    pub fn has_capability(&self, capability: &PluginCapability) -> bool {
        self.capabilities.contains(capability)
    }
    
    /// Set plugin status
    pub fn set_status(&mut self, status: PluginStatus) {
        self.status = status;
    }
    
    /// Check if plugin is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, PluginStatus::Active)
    }
    
    /// Check if plugin is healthy
    pub fn is_healthy(&self) -> bool {
        self.health.health_score > 0.5 && self.health.errors.is_empty()
    }
}

impl PluginHealth {
    /// Create new plugin health
    pub fn new() -> Self {
        Self {
            health_score: 1.0,
            last_check: chrono::Utc::now(),
            failure_count: 0,
            success_count: 0,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    /// Record a successful operation
    pub fn record_success(&mut self) {
        self.success_count += 1;
        self.last_check = chrono::Utc::now();
        
        let total_operations = self.success_count + self.failure_count;
        if total_operations > 0 {
            self.health_score = self.success_count as f64 / total_operations as f64;
        }
    }
    
    /// Record a failed operation
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_check = chrono::Utc::now();
        
        let total_operations = self.success_count + self.failure_count;
        if total_operations > 0 {
            self.health_score = self.success_count as f64 / total_operations as f64;
        }
    }
    
    /// Add an error
    pub fn add_error(&mut self, error: String) {
        if !self.errors.contains(&error) {
            self.errors.push(error);
        }
        self.record_failure();
    }
    
    /// Add a warning
    pub fn add_warning(&mut self, warning: String) {
        if !self.warnings.contains(&warning) {
            self.warnings.push(warning);
        }
    }
    
    /// Clear errors
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }
    
    /// Clear warnings
    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }
    
    /// Get total operations
    pub fn total_operations(&self) -> u32 {
        self.success_count + self.failure_count
    }
    
    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        let total = self.total_operations();
        if total > 0 {
            self.success_count as f64 / total as f64
        } else {
            0.0
        }
    }
}

impl Default for PluginHealth {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TrainingMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrainingMethodType::Meditation => write!(f, "Meditation"),
            TrainingMethodType::Combat => write!(f, "Combat"),
            TrainingMethodType::Crafting => write!(f, "Crafting"),
            TrainingMethodType::Exploration => write!(f, "Exploration"),
            TrainingMethodType::Social => write!(f, "Social"),
            TrainingMethodType::Ritual => write!(f, "Ritual"),
            TrainingMethodType::Study => write!(f, "Study"),
            TrainingMethodType::Practice => write!(f, "Practice"),
        }
    }
}

impl std::fmt::Display for PluginInteractionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginInteractionType::Generating => write!(f, "Generating"),
            PluginInteractionType::Overcoming => write!(f, "Overcoming"),
            PluginInteractionType::Same => write!(f, "Same"),
            PluginInteractionType::Neutral => write!(f, "Neutral"),
            PluginInteractionType::Opposite => write!(f, "Opposite"),
            PluginInteractionType::Special => write!(f, "Special"),
        }
    }
}

impl std::fmt::Display for PluginStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginStatus::Active => write!(f, "Active"),
            PluginStatus::Inactive => write!(f, "Inactive"),
            PluginStatus::Loading => write!(f, "Loading"),
            PluginStatus::Error => write!(f, "Error"),
            PluginStatus::Deprecated => write!(f, "Deprecated"),
        }
    }
}

impl std::fmt::Display for PluginCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginCapability::ProvideTrainingMethods => write!(f, "ProvideTrainingMethods"),
            PluginCapability::CalculateDerivedStats => write!(f, "CalculateDerivedStats"),
            PluginCapability::HandleElementInteractions => write!(f, "HandleElementInteractions"),
            PluginCapability::ProvideStatusEffects => write!(f, "ProvideStatusEffects"),
            PluginCapability::ManageElementMastery => write!(f, "ManageElementMastery"),
            PluginCapability::ProvideCustomFormulas => write!(f, "ProvideCustomFormulas"),
            PluginCapability::HandleElementEvents => write!(f, "HandleElementEvents"),
            PluginCapability::ProvideElementDefinitions => write!(f, "ProvideElementDefinitions"),
        }
    }
}
