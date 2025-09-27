//! # Inheritable Types
//! 
//! This module provides traits and base implementations that allow for inheritance-like
//! behavior in Rust, enabling actor-core-hierarchical to extend and override functionality.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use shared::{EntityId, Timestamp, Version};
use crate::enums::{Bucket, CapMode};

/// Trait for base actor functionality that can be overridden
pub trait ActorBase {
    /// Get the actor's unique identifier
    fn get_id(&self) -> &EntityId;
    
    /// Get the actor's name
    fn get_name(&self) -> &str;
    
    /// Get the actor's race
    fn get_race(&self) -> &str;
    
    /// Get the actor's lifespan
    fn get_lifespan(&self) -> i64;
    
    /// Get the actor's age
    fn get_age(&self) -> i64;
    
    /// Get the actor's version
    fn get_version(&self) -> Version;
    
    /// Check if the actor is valid (can be overridden)
    fn is_valid(&self) -> bool {
        !self.get_name().is_empty() && !self.get_race().is_empty() && self.get_version() > 0
    }
    
    /// Update the actor's version and timestamp (can be overridden)
    fn touch(&mut self) {
        // Default implementation - can be overridden
    }
    
    /// Get additional data (can be overridden)
    fn get_data(&self) -> &HashMap<String, serde_json::Value>;
    
    /// Set additional data (can be overridden)
    fn set_data(&mut self, data: HashMap<String, serde_json::Value>);
    
    /// Check if actor has a specific subsystem (can be overridden)
    fn has_subsystem(&self, system_id: &str) -> bool {
        // Default implementation - can be overridden
        false
    }
    
    /// Get subsystem count (can be overridden)
    fn get_subsystem_count(&self) -> usize {
        // Default implementation - can be overridden
        0
    }
}

/// Trait for subsystem functionality that can be overridden
pub trait SubsystemBase {
    /// Get the subsystem's system ID
    fn get_system_id(&self) -> &str;
    
    /// Get the subsystem's priority
    fn get_priority(&self) -> i64;
    
    /// Check if the subsystem is enabled
    fn is_enabled(&self) -> bool;
    
    /// Get the subsystem's configuration
    fn get_config(&self) -> &HashMap<String, serde_json::Value>;
    
    /// Set the subsystem's configuration
    fn set_config(&mut self, config: HashMap<String, serde_json::Value>);
    
    /// Get the subsystem's data
    fn get_data(&self) -> &HashMap<String, serde_json::Value>;
    
    /// Set the subsystem's data
    fn set_data(&mut self, data: HashMap<String, serde_json::Value>);
    
    /// Check if the subsystem is valid (can be overridden)
    fn is_valid(&self) -> bool {
        !self.get_system_id().is_empty() && self.get_priority() >= 0
    }
}

/// Trait for contribution functionality that can be overridden
pub trait ContributionBase {
    /// Get the contribution's name
    fn get_name(&self) -> &str;
    
    /// Get the contribution's value
    fn get_value(&self) -> f64;
    
    /// Get the contribution's bucket type
    fn get_bucket(&self) -> Bucket;
    
    /// Get the contribution's source
    fn get_source(&self) -> &str;
    
    /// Check if the contribution is valid (can be overridden)
    fn is_valid(&self) -> bool {
        !self.get_name().is_empty() && !self.get_source().is_empty()
    }
}

/// Trait for caps functionality that can be overridden
pub trait CapsBase {
    /// Get the caps' name
    fn get_name(&self) -> &str;
    
    /// Get the minimum value
    fn get_min(&self) -> Option<f64>;
    
    /// Get the maximum value
    fn get_max(&self) -> Option<f64>;
    
    /// Get the cap mode
    fn get_mode(&self) -> CapMode;
    
    /// Check if the caps are valid (can be overridden)
    fn is_valid(&self) -> bool {
        !self.get_name().is_empty()
    }
}

/// Base actor implementation that can be extended
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseActor {
    /// Unique identifier for the actor
    pub id: EntityId,
    /// Display name of the actor
    pub name: String,
    /// Race of the actor
    pub race: String,
    /// Lifespan of the actor in seconds
    pub lifespan: i64,
    /// Current age of the actor in seconds
    pub age: i64,
    /// When the actor was created
    pub created_at: Timestamp,
    /// When the actor was last updated
    pub updated_at: Timestamp,
    /// Version for optimistic concurrency control
    pub version: Version,
    /// Additional actor data
    pub data: HashMap<String, serde_json::Value>,
}

impl Default for BaseActor {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseActor {
    /// Create a new base actor with default values
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            race: String::new(),
            lifespan: 0,
            age: 0,
            created_at: now,
            updated_at: now,
            version: 1,
            data: HashMap::new(),
        }
    }
    
    /// Create a new base actor with specified name and race
    pub fn with_name_and_race(name: String, race: String) -> Self {
        let mut actor = Self::new();
        actor.name = name;
        actor.race = race;
        actor
    }
    
    /// Update the actor's version and timestamp
    pub fn touch(&mut self) {
        self.version += 1;
        self.updated_at = Utc::now();
    }
}

impl ActorBase for BaseActor {
    fn get_id(&self) -> &EntityId {
        &self.id
    }
    
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_race(&self) -> &str {
        &self.race
    }
    
    fn get_lifespan(&self) -> i64 {
        self.lifespan
    }
    
    fn get_age(&self) -> i64 {
        self.age
    }
    
    fn get_version(&self) -> Version {
        self.version
    }
    
    fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.race.is_empty() && self.version > 0
    }
    
    fn touch(&mut self) {
        self.version += 1;
        self.updated_at = Utc::now();
    }
    
    fn get_data(&self) -> &HashMap<String, serde_json::Value> {
        &self.data
    }
    
    fn set_data(&mut self, data: HashMap<String, serde_json::Value>) {
        self.data = data;
        self.touch();
    }
    
    fn has_subsystem(&self, _system_id: &str) -> bool {
        // Base implementation returns false - can be overridden
        false
    }
    
    fn get_subsystem_count(&self) -> usize {
        // Base implementation returns 0 - can be overridden
        0
    }
}

/// Base subsystem implementation that can be extended
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseSubsystem {
    /// Unique identifier for the subsystem
    pub system_id: String,
    /// Priority of the subsystem (higher = more important)
    pub priority: i64,
    /// Whether the subsystem is enabled
    pub enabled: bool,
    /// Configuration for the subsystem
    pub config: HashMap<String, serde_json::Value>,
    /// Additional subsystem data
    pub data: HashMap<String, serde_json::Value>,
}

impl Default for BaseSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseSubsystem {
    /// Create a new base subsystem
    pub fn new() -> Self {
        Self {
            system_id: String::new(),
            priority: 0,
            enabled: true,
            config: HashMap::new(),
            data: HashMap::new(),
        }
    }
    
    /// Create a new base subsystem with specified system ID and priority
    pub fn with_id_and_priority(system_id: String, priority: i64) -> Self {
        Self {
            system_id,
            priority,
            enabled: true,
            config: HashMap::new(),
            data: HashMap::new(),
        }
    }
}

impl SubsystemBase for BaseSubsystem {
    fn get_system_id(&self) -> &str {
        &self.system_id
    }
    
    fn get_priority(&self) -> i64 {
        self.priority
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn get_config(&self) -> &HashMap<String, serde_json::Value> {
        &self.config
    }
    
    fn set_config(&mut self, config: HashMap<String, serde_json::Value>) {
        self.config = config;
    }
    
    fn get_data(&self) -> &HashMap<String, serde_json::Value> {
        &self.data
    }
    
    fn set_data(&mut self, data: HashMap<String, serde_json::Value>) {
        self.data = data;
    }
    
    fn is_valid(&self) -> bool {
        !self.system_id.is_empty() && self.priority >= 0
    }
}

/// Base contribution implementation that can be extended
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseContribution {
    /// Name of the stat being contributed to
    pub name: String,
    /// Value of the contribution
    pub value: f64,
    /// Bucket type for processing order
    pub bucket: Bucket,
    /// Source of the contribution
    pub source: String,
    /// Additional contribution data
    pub data: HashMap<String, serde_json::Value>,
}

impl Default for BaseContribution {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseContribution {
    /// Create a new base contribution
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: 0.0,
            bucket: Bucket::Flat,
            source: String::new(),
            data: HashMap::new(),
        }
    }
    
    /// Create a new base contribution with specified values
    pub fn with_values(name: String, value: f64, bucket: Bucket, source: String) -> Self {
        Self {
            name,
            value,
            bucket,
            source,
            data: HashMap::new(),
        }
    }
}

impl ContributionBase for BaseContribution {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_value(&self) -> f64 {
        self.value
    }
    
    fn get_bucket(&self) -> Bucket {
        self.bucket
    }
    
    fn get_source(&self) -> &str {
        &self.source
    }
    
    fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.source.is_empty()
    }
}

/// Base caps implementation that can be extended
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCaps {
    /// Name of the stat being capped
    pub name: String,
    /// Minimum value (None means no minimum)
    pub min: Option<f64>,
    /// Maximum value (None means no maximum)
    pub max: Option<f64>,
    /// Cap mode for processing
    pub mode: CapMode,
    /// Additional caps data
    pub data: HashMap<String, serde_json::Value>,
}

impl Default for BaseCaps {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseCaps {
    /// Create a new base caps
    pub fn new() -> Self {
        Self {
            name: String::new(),
            min: None,
            max: None,
            mode: CapMode::Baseline,
            data: HashMap::new(),
        }
    }
    
    /// Create a new base caps with specified values
    pub fn with_values(name: String, min: Option<f64>, max: Option<f64>, mode: CapMode) -> Self {
        Self {
            name,
            min,
            max,
            mode,
            data: HashMap::new(),
        }
    }
}

impl CapsBase for BaseCaps {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_min(&self) -> Option<f64> {
        self.min
    }
    
    fn get_max(&self) -> Option<f64> {
        self.max
    }
    
    fn get_mode(&self) -> CapMode {
        self.mode
    }
    
    fn is_valid(&self) -> bool {
        !self.name.is_empty()
    }
}

/// Trait for factory pattern to create inheritable types
pub trait ActorFactory {
    type ActorType: ActorBase;
    
    /// Create a new actor instance
    fn create_actor(&self, name: String, race: String) -> Self::ActorType;
    
    /// Create a new actor with custom data
    fn create_actor_with_data(&self, name: String, race: String, data: HashMap<String, serde_json::Value>) -> Self::ActorType;
}

/// Trait for factory pattern to create inheritable subsystems
pub trait SubsystemFactory {
    type SubsystemType: SubsystemBase;
    
    /// Create a new subsystem instance
    fn create_subsystem(&self, system_id: String, priority: i64) -> Self::SubsystemType;
    
    /// Create a new subsystem with custom data
    fn create_subsystem_with_data(&self, system_id: String, priority: i64, data: HashMap<String, serde_json::Value>) -> Self::SubsystemType;
}

/// Default actor factory implementation
pub struct DefaultActorFactory;

impl ActorFactory for DefaultActorFactory {
    type ActorType = BaseActor;
    
    fn create_actor(&self, name: String, race: String) -> Self::ActorType {
        BaseActor::with_name_and_race(name, race)
    }
    
    fn create_actor_with_data(&self, name: String, race: String, data: HashMap<String, serde_json::Value>) -> Self::ActorType {
        let mut actor = BaseActor::with_name_and_race(name, race);
        actor.set_data(data);
        actor
    }
}

/// Default subsystem factory implementation
pub struct DefaultSubsystemFactory;

impl SubsystemFactory for DefaultSubsystemFactory {
    type SubsystemType = BaseSubsystem;
    
    fn create_subsystem(&self, system_id: String, priority: i64) -> Self::SubsystemType {
        BaseSubsystem::with_id_and_priority(system_id, priority)
    }
    
    fn create_subsystem_with_data(&self, system_id: String, priority: i64, data: HashMap<String, serde_json::Value>) -> Self::SubsystemType {
        let mut subsystem = BaseSubsystem::with_id_and_priority(system_id, priority);
        subsystem.set_data(data);
        subsystem
    }
}

/// Utility functions for inheritance support
pub mod inheritance_utils {
    use super::*;
    
    /// Convert a base actor to a hierarchical actor (for actor-core-hierarchical)
    pub fn convert_base_to_hierarchical(base_actor: &BaseActor) -> Result<HashMap<String, serde_json::Value>, String> {
        let mut data = base_actor.get_data().clone();
        
        // Add basic actor information
        data.insert("id".to_string(), serde_json::Value::String(base_actor.get_id().to_string()));
        data.insert("name".to_string(), serde_json::Value::String(base_actor.get_name().to_string()));
        data.insert("race".to_string(), serde_json::Value::String(base_actor.get_race().to_string()));
        data.insert("lifespan".to_string(), serde_json::Value::Number(serde_json::Number::from(base_actor.get_lifespan())));
        data.insert("age".to_string(), serde_json::Value::Number(serde_json::Number::from(base_actor.get_age())));
        data.insert("version".to_string(), serde_json::Value::Number(serde_json::Number::from(base_actor.get_version())));
        
        Ok(data)
    }
    
    /// Convert hierarchical actor data to a base actor
    pub fn convert_hierarchical_to_base(data: &HashMap<String, serde_json::Value>) -> Result<BaseActor, String> {
        let name = data.get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing name field")?
            .to_string();
        
        let race = data.get("race")
            .and_then(|v| v.as_str())
            .ok_or("Missing race field")?
            .to_string();
        
        let mut actor = BaseActor::with_name_and_race(name, race);
        
        // Set additional fields if present
        if let Some(id) = data.get("id").and_then(|v| v.as_str()) {
            if let Ok(uuid) = Uuid::parse_str(id) {
                actor.id = uuid;
            }
        }
        
        if let Some(lifespan) = data.get("lifespan").and_then(|v| v.as_i64()) {
            actor.lifespan = lifespan;
        }
        
        if let Some(age) = data.get("age").and_then(|v| v.as_i64()) {
            actor.age = age;
        }
        
        if let Some(version) = data.get("version").and_then(|v| v.as_i64()) {
            actor.version = version;
        }
        
        Ok(actor)
    }
    
    /// Validate that a hierarchical actor is compatible with base actor
    pub fn validate_compatibility(base_actor: &BaseActor, hierarchical_data: &HashMap<String, serde_json::Value>) -> Result<(), String> {
        // Check if names match
        if let Some(name) = hierarchical_data.get("name").and_then(|v| v.as_str()) {
            if name != base_actor.get_name() {
                return Err("Name mismatch between base and hierarchical actor".to_string());
            }
        }
        
        // Check if races match
        if let Some(race) = hierarchical_data.get("race").and_then(|v| v.as_str()) {
            if race != base_actor.get_race() {
                return Err("Race mismatch between base and hierarchical actor".to_string());
            }
        }
        
        Ok(())
    }
}
