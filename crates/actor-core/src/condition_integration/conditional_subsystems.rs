//! Conditional Subsystems for Condition Core Integration
//! 
//! This module provides conditional subsystem functionality that allows
//! subsystems to be activated or deactivated based on conditions.

use condition_core::*;
use std::sync::Arc;
use crate::Actor;

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

/// Trait for subsystems that can be conditionally activated
pub trait ConditionalSubsystem: Subsystem {
    /// Get the condition configuration for this subsystem
    fn get_activation_condition(&self) -> Option<ConditionConfig>;
    
    /// Get the condition resolver for this subsystem
    fn get_condition_resolver(&self) -> Option<Arc<ConditionResolver>>;
    
    /// Check if the subsystem should be activated for the given actor
    #[allow(async_fn_in_trait)]
    async fn should_activate(&self, actor: &Actor) -> ActorCoreResult<bool> {
        if let Some(condition) = self.get_activation_condition() {
            if let Some(resolver) = self.get_condition_resolver() {
                let context = self.create_condition_context(actor).await?;
                let result = resolver.resolve_condition(&condition, &context).await?;
                Ok(result)
            } else {
                Ok(true) // No resolver means always activate
            }
        } else {
            Ok(true) // No condition means always activate
        }
    }
    
    /// Create condition context from actor
    #[allow(async_fn_in_trait)]
    async fn create_condition_context(&self, actor: &Actor) -> ActorCoreResult<ConditionContext> {
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
}

/// Example conditional subsystem
pub struct ConditionalResourceSubsystem {
    system_id: String,
    priority: i64,
    activation_condition: Option<ConditionConfig>,
    condition_resolver: Option<Arc<ConditionResolver>>,
    resource_manager: Arc<dyn ResourceManager>,
}

impl ConditionalResourceSubsystem {
    /// Create a new conditional resource subsystem
    pub fn new(
        system_id: String,
        priority: i64,
        activation_condition: Option<ConditionConfig>,
        condition_resolver: Option<Arc<ConditionResolver>>,
        resource_manager: Arc<dyn ResourceManager>,
    ) -> Self {
        Self {
            system_id,
            priority,
            activation_condition,
            condition_resolver,
            resource_manager,
        }
    }
}

impl ConditionalSubsystem for ConditionalResourceSubsystem {
    fn get_activation_condition(&self) -> Option<ConditionConfig> {
        self.activation_condition.clone()
    }
    
    fn get_condition_resolver(&self) -> Option<Arc<ConditionResolver>> {
        self.condition_resolver.clone()
    }
}

#[async_trait::async_trait]
impl Subsystem for ConditionalResourceSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute(&self, actor: &Actor) -> ActorCoreResult<SubsystemOutput> {
        // Check if subsystem should be activated
        if !self.should_activate(actor).await? {
            return Ok(SubsystemOutput::empty());
        }
        
        // Normal subsystem contribution logic
        let resources = self.resource_manager.get_actor_resources(&actor.id.to_string()).await?;
        let mut primary = Vec::new();
        let derived = Vec::new();
        let mut caps = Vec::new();
        
        for resource in resources {
            primary.push(Contribution {
                dimension: resource.name.clone(),
                bucket: Bucket::FLAT,
                value: resource.current_value,
                system: self.system_id.clone(),
                priority: Some(self.priority),
            });
            
            caps.push(CapContribution {
                system: self.system_id.clone(),
                dimension: resource.name.clone(),
                mode: CapMode::HardMax,
                kind: CapKind::Max,
                value: resource.max_value,
                priority: Some(self.priority),
                scope: Some("TOTAL".to_string()),
                realm: None,
                tags: None,
            });
        }
        
        Ok(SubsystemOutput {
            primary,
            derived,
            caps,
            context: None,
            meta: SubsystemMeta {
                system: self.system_id.clone(),
                stage: None,
                version: Some(1),
            },
        })
    }
}

// TODO: Replace with actual traits and types from actor-core
#[async_trait::async_trait]
pub trait Subsystem: Send + Sync {
    fn system_id(&self) -> &str;
    fn priority(&self) -> i64;
    async fn contribute(&self, actor: &Actor) -> ActorCoreResult<SubsystemOutput>;
}

#[derive(Debug, Clone)]
pub struct SubsystemOutput {
    pub primary: Vec<Contribution>,
    pub derived: Vec<Contribution>,
    pub caps: Vec<CapContribution>,
    pub context: Option<serde_json::Value>,
    pub meta: SubsystemMeta,
}

impl SubsystemOutput {
    pub fn empty() -> Self {
        Self {
            primary: Vec::new(),
            derived: Vec::new(),
            caps: Vec::new(),
            context: None,
            meta: SubsystemMeta {
                system: "empty".to_string(),
                stage: None,
                version: Some(1),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Contribution {
    pub dimension: String,
    pub bucket: Bucket,
    pub value: f64,
    pub system: String,
    pub priority: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct CapContribution {
    pub system: String,
    pub dimension: String,
    pub mode: CapMode,
    pub kind: CapKind,
    pub value: f64,
    pub priority: Option<i64>,
    pub scope: Option<String>,
    pub realm: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct SubsystemMeta {
    pub system: String,
    pub stage: Option<String>,
    pub version: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum Bucket {
    FLAT,
    PERCENTAGE,
    MULTIPLIER,
}

#[derive(Debug, Clone)]
pub enum CapMode {
    HardMax,
    SoftMax,
    HardMin,
    SoftMin,
}

#[derive(Debug, Clone)]
pub enum CapKind {
    Max,
    Min,
}

// TODO: Replace with actual error type from actor-core
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

// TODO: Replace with actual ResourceManager trait from actor-core
#[async_trait::async_trait]
pub trait ResourceManager: Send + Sync {
    async fn get_actor_resources(&self, actor_id: &str) -> Result<Vec<Resource>, Box<dyn std::error::Error + Send + Sync>>;
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub name: String,
    pub current_value: f64,
    pub max_value: f64,
}
