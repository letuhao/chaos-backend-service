//! Main Integration Module for Actor Core + Condition Core
//! 
//! This module provides the main integration functionality between
//! Actor Core and Condition Core.

use condition_core::*;
use std::sync::Arc;

// Use explicit cache type to avoid ambiguity
type ActorCoreCache = dyn crate::interfaces::Cache;

// Error conversion implementations
impl From<Box<dyn std::error::Error + Send + Sync>> for ActorCoreError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        ActorCoreError::ConditionError(err.to_string())
    }
}

impl From<ConditionError> for ActorCoreError {
    fn from(err: ConditionError) -> Self {
        ActorCoreError::ConditionError(err.to_string())
    }
}

// Type aliases to avoid conflicts
pub type ActorCoreActor = crate::condition_integration::data_providers::Actor;
pub type ActorCoreSnapshot = crate::condition_integration::data_providers::Snapshot;
pub type ActorCoreResource = crate::condition_integration::data_providers::Resource;

/// Main integration struct for Actor Core + Condition Core
pub struct ActorCoreWithConditions {
    // Actor Core components
    #[allow(dead_code)]
    aggregator: Arc<dyn Aggregator>,
    #[allow(dead_code)]
    cache: Arc<ActorCoreCache>,
    #[allow(dead_code)]
    plugin_registry: Arc<PluginRegistry>,
    
    // Condition Core components
    #[allow(dead_code)]
    condition_resolver: Arc<ConditionResolver>,
    #[allow(dead_code)]
    data_provider_registry: Arc<DataProviderRegistry>,
    
    // Integration components
    #[allow(dead_code)]
    condition_cache: Arc<ConditionCache>,
    event_handler: Arc<ConditionEventHandler>,
}

impl ActorCoreWithConditions {
    /// Create a new integrated system
    pub fn new(
        aggregator: Arc<dyn Aggregator>,
        condition_resolver: Arc<ConditionResolver>,
        cache: Arc<ActorCoreCache>,
    ) -> Self {
        Self {
            aggregator,
            cache: cache.clone(),
            plugin_registry: Arc::new(PluginRegistry::new()),
            condition_resolver,
            data_provider_registry: Arc::new(DataProviderRegistry::new()),
            condition_cache: Arc::new(ConditionCache::new(cache)),
            event_handler: Arc::new(ConditionEventHandler::new()),
        }
    }
    
    /// Resolve actor stats with conditions
    pub async fn resolve_with_conditions(&self, actor: &crate::Actor) -> ActorCoreResult<Snapshot> {
        // Pre-aggregation conditions
        self.validate_pre_conditions(actor).await?;
        
        // Apply conditional subsystems
        let mut snapshot = self.apply_conditional_subsystems(actor).await?;
        
        // Apply conditional modifiers - convert types
        let modifier_system = ConditionalModifierSystem::new(
            self.condition_resolver.clone(),
            Arc::new(ModifierRegistryImpl::new()),
            Arc::new(MockCache::new()),
        );
        
        // Convert integration types to modifier types
        let modifier_actor = ModifierActor {
            id: actor.id.to_string(),
            race: actor.race.clone(),
            data: serde_json::Value::Object(actor.data.clone().into_iter().collect()),
        };
        let mut modifier_snapshot = ModifierSnapshot {
            primary: snapshot.primary.clone(),
            derived: snapshot.derived.clone(),
        };
        
        modifier_system.apply_modifiers(&modifier_actor, &mut modifier_snapshot).await
            .map_err(|e| ActorCoreError::ConditionError(e.to_string()))?;
        
        // Convert back to integration types
        snapshot.primary = modifier_snapshot.primary;
        snapshot.derived = modifier_snapshot.derived;
        
        // Post-aggregation conditions
        self.validate_post_conditions(actor, &snapshot).await?;
        
        // Trigger events
        self.event_handler.handle_actor_update(actor, &snapshot).await?;
        
        Ok(snapshot)
    }
    
    /// Validate pre-aggregation conditions
    async fn validate_pre_conditions(&self, actor: &crate::Actor) -> ActorCoreResult<()> {
        let context = self.create_condition_context(actor).await?;
        
        // Check if actor has minimum health
        let health_condition = ConditionConfig {
            condition_id: "min_health".to_string(),
            function_name: "get_actor_resource".to_string(),
            operator: ConditionOperator::GreaterThan,
            value: ConditionValue::Float(1.0),
            parameters: vec![ConditionParameter::String("health".to_string())],
        };
        
        let has_min_health = self.condition_resolver
            .resolve_condition(&health_condition, &context)
            .await?;
        
        if !has_min_health {
            return Err(ActorCoreError::ValidationError(
                "Actor does not have minimum health".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Apply conditional subsystems
    async fn apply_conditional_subsystems(&self, _actor: &crate::Actor) -> ActorCoreResult<Snapshot> {
        let snapshot = Snapshot::default();
        
        // For now, just return an empty snapshot since subsystems are structs, not trait objects
        // TODO: Implement proper subsystem contribution logic
        // This would require either:
        // 1. Converting Actor to use trait objects for subsystems, or
        // 2. Implementing a subsystem registry that maps system_id to trait implementations
        
        Ok(snapshot)
    }
    
    /// Validate post-aggregation conditions
    async fn validate_post_conditions(&self, _actor: &crate::Actor, _snapshot: &Snapshot) -> ActorCoreResult<()> {
        // Add post-aggregation validation logic here
        Ok(())
    }
    
    /// Create condition context from actor
    async fn create_condition_context(&self, actor: &crate::Actor) -> ActorCoreResult<ConditionContext> {
        Ok(ConditionContext {
            target: ActorTarget {
                id: actor.id.to_string(),
            },
            world_id: "default".to_string(),
            current_time: std::time::SystemTime::now(),
            current_weather: WeatherType::Clear,
            world_state: WorldState {
                time_of_day: 12.0,
                season: "spring".to_string(),
                temperature: 20.0,
                humidity: 0.5,
            },
        })
    }
    
    /// Merge subsystem output into snapshot
    #[allow(dead_code)]
    fn merge_subsystem_output(&self, snapshot: &mut Snapshot, output: &SubsystemOutput) -> ActorCoreResult<()> {
        // Merge primary stats
        for contribution in &output.primary {
            let current = snapshot.primary.get(&contribution.dimension).unwrap_or(&0.0);
            let new_value = match contribution.bucket {
                Bucket::FLAT => current + contribution.value,
                Bucket::PERCENTAGE => current * (1.0 + contribution.value / 100.0),
                Bucket::MULTIPLIER => current * contribution.value,
            };
            snapshot.primary.insert(contribution.dimension.clone(), new_value);
        }
        
        // Merge derived stats
        for contribution in &output.derived {
            let current = snapshot.derived.get(&contribution.dimension).unwrap_or(&0.0);
            let new_value = match contribution.bucket {
                Bucket::FLAT => current + contribution.value,
                Bucket::PERCENTAGE => current * (1.0 + contribution.value / 100.0),
                Bucket::MULTIPLIER => current * contribution.value,
            };
            snapshot.derived.insert(contribution.dimension.clone(), new_value);
        }
        
        Ok(())
    }
}

/// Condition cache for performance optimization
pub struct ConditionCache {
    cache: Arc<ActorCoreCache>,
    #[allow(dead_code)]
    ttl: std::time::Duration,
}

impl ConditionCache {
    /// Create a new condition cache
    pub fn new(cache: Arc<ActorCoreCache>) -> Self {
        Self {
            cache,
            ttl: std::time::Duration::from_secs(300), // 5 minutes
        }
    }
    
    /// Get cached condition result
    pub async fn get_condition_result(
        &self,
        condition_id: &str,
        actor_id: &str,
    ) -> Option<bool> {
        let key = format!("condition:{}:{}", condition_id, actor_id);
        // Use Actor Core cache interface
        match self.cache.get(&key) {
            Some(value) => value.as_bool(),
            None => None,
        }
    }
    
    /// Cache condition result
    pub async fn cache_condition_result(
        &self,
        condition_id: &str,
        actor_id: &str,
        result: bool,
    ) -> ActorCoreResult<()> {
        let key = format!("condition:{}:{}", condition_id, actor_id);
        self.cache.set(key, serde_json::Value::Bool(result), Some(300))
            .map_err(|e| ActorCoreError::ConditionError(e.to_string()))?;
        Ok(())
    }
    
    /// Invalidate condition cache for actor
    pub async fn invalidate_actor_cache(&self, actor_id: &str) -> ActorCoreResult<()> {
        let pattern = format!("condition:*:{}", actor_id);
        self.cache.delete(&pattern)
            .map_err(|e| ActorCoreError::ConditionError(e.to_string()))?;
        Ok(())
    }
}

/// Condition event handler
pub struct ConditionEventHandler;

impl ConditionEventHandler {
    /// Create a new condition event handler
    pub fn new() -> Self {
        Self
    }
    
    /// Handle actor update events
    pub async fn handle_actor_update(&self, _actor: &crate::Actor, _snapshot: &Snapshot) -> ActorCoreResult<()> {
        // Handle actor update events here
        Ok(())
    }
}

// TODO: Replace with actual traits and types from Actor Core
#[async_trait::async_trait]
pub trait Aggregator: Send + Sync {
    async fn aggregate(&self, actor: &crate::Actor) -> Result<Snapshot, Box<dyn std::error::Error + Send + Sync>>;
}

#[derive(Debug, Clone)]
pub struct PluginRegistry;

impl PluginRegistry {
    pub fn new() -> Self {
        Self
    }
}




// These traits are now imported from other modules to avoid conflicts

// These structs are now imported from other modules to avoid conflicts

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub primary: std::collections::HashMap<String, f64>,
    pub derived: std::collections::HashMap<String, f64>,
}

impl Default for Snapshot {
    fn default() -> Self {
        Self {
            primary: std::collections::HashMap::new(),
            derived: std::collections::HashMap::new(),
        }
    }
}

impl Snapshot {
    pub fn get_mut(&mut self, key: &str) -> Option<&mut f64> {
        self.primary.get_mut(key)
    }
    
    pub fn insert(&mut self, key: String, value: f64) {
        self.primary.insert(key, value);
    }
}

// TODO: Replace with actual error type from Actor Core
#[derive(Debug)]
pub enum ActorCoreError {
    ValidationError(String),
    ResourceError(String),
    ConditionError(String),
}

impl std::fmt::Display for ActorCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorCoreError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            ActorCoreError::ResourceError(msg) => write!(f, "Resource Error: {}", msg),
            ActorCoreError::ConditionError(msg) => write!(f, "Condition Error: {}", msg),
        }
    }
}

impl std::error::Error for ActorCoreError {}

pub type ActorCoreResult<T> = Result<T, ActorCoreError>;

// TODO: Replace with actual modifier registry implementation from Actor Core
pub struct ModifierRegistryImpl;

impl ModifierRegistryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ModifierRegistry for ModifierRegistryImpl {
    async fn get_modifiers_for_actor(&self, _actor: &ModifierActor) -> Result<Vec<ConditionalModifier>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Vec::new())
    }
}

// Mock cache implementation
pub struct MockCache;

impl MockCache {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl crate::condition_integration::conditional_modifiers::Cache for MockCache {
    async fn get(&self, _key: &str) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }
    
    async fn set(&self, _key: &str, _value: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn delete(&self, _key: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

// Re-export specific types from other modules to avoid conflicts
pub use crate::condition_integration::data_providers::{
    ActorDataProvider, ResourceDataProvider, CategoryDataProvider,
    ActorRepository, StatCache, ResourceManager, ActorInventory, CategoryRegistry,
    Resource, Item
};
pub use crate::condition_integration::conditional_subsystems::{
    ConditionalSubsystem, SubsystemOutput, Contribution, CapContribution, SubsystemMeta, CapMode, CapKind, Bucket
};
pub use crate::condition_integration::conditional_modifiers::{
    ConditionalModifierSystem, ConditionalModifier, ModifierRegistry,
    ModifierActor, ModifierSnapshot, ModifierType
};
