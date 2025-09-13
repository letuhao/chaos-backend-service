//! Core types for the Actor Core system.
//!
//! This module contains the fundamental data structures for character stat
//! aggregation, including Actor, Contribution, Snapshot, and related types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use shared::{EntityId, Timestamp, Version, GameEntity};
use crate::enums::{Bucket, CapMode};

/// Type alias for effective caps mapping.
/// Maps dimension names to their effective cap values.
pub type EffectiveCaps = HashMap<String, Caps>;

/// Actor represents a game character with stats and subsystems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
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
    /// List of active subsystems
    pub subsystems: Vec<Subsystem>,
    /// Additional actor data
    pub data: HashMap<String, serde_json::Value>,
}

impl Actor {
    /// Create a new actor with default values.
    pub fn new(name: String, race: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            race,
            lifespan: 0,
            age: 0,
            created_at: now,
            updated_at: now,
            version: 1,
            subsystems: Vec::new(),
            data: HashMap::new(),
        }
    }

    /// Check if the actor is valid.
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.race.is_empty() && self.version > 0
    }

    /// Update the actor's version and timestamp.
    pub fn touch(&mut self) {
        self.version += 1;
        self.updated_at = Utc::now();
    }

    /// Add a subsystem to the actor.
    pub fn add_subsystem(&mut self, subsystem: Subsystem) {
        self.subsystems.push(subsystem);
        self.touch();
    }

    /// Remove a subsystem by system ID.
    pub fn remove_subsystem(&mut self, system_id: &str) -> bool {
        if let Some(pos) = self.subsystems.iter().position(|s| s.system_id == system_id) {
            self.subsystems.remove(pos);
            self.touch();
            true
        } else {
            false
        }
    }

    /// Check if the actor is in combat.
    pub fn is_in_combat(&self) -> bool {
        self.data.get("in_combat")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Check if the actor has active buffs.
    pub fn has_buffs(&self) -> bool {
        self.data.get("has_buffs")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Get a subsystem by system ID
    pub fn get_subsystem(&self, system_id: &str) -> Option<&Subsystem> {
        self.subsystems.iter().find(|s| s.system_id == system_id)
    }

    /// Check if actor has a specific subsystem
    pub fn has_subsystem(&self, system_id: &str) -> bool {
        self.subsystems.iter().any(|s| s.system_id == system_id)
    }

    /// Get guild ID from actor data
    pub fn get_guild_id(&self) -> Option<&str> {
        self.data.get("guild_id")
            .and_then(|v| v.as_str())
    }

    /// Set guild ID in actor data
    pub fn set_guild_id(&mut self, guild_id: String) {
        self.data.insert("guild_id".to_string(), serde_json::Value::String(guild_id));
        self.touch();
    }

    /// Set combat status
    pub fn set_in_combat(&mut self, in_combat: bool) {
        self.data.insert("in_combat".to_string(), serde_json::Value::Bool(in_combat));
        if in_combat {
            self.data.insert("combat_start_time".to_string(), 
                serde_json::Value::Number(serde_json::Number::from(Utc::now().timestamp())));
        }
        self.touch();
    }

    /// Check if actor has a specific buff
    pub fn has_buff(&self, buff_id: &str) -> bool {
        self.data.get("buffs")
            .and_then(|v| v.as_array())
            .map(|buffs| buffs.iter().any(|b| b.as_str() == Some(buff_id)))
            .unwrap_or(false)
    }

    /// Add a buff to the actor
    pub fn add_buff(&mut self, buff_id: String) {
        let buffs = self.data.entry("buffs".to_string())
            .or_insert_with(|| serde_json::Value::Array(Vec::new()));
        if let Some(buffs_array) = buffs.as_array_mut() {
            if !buffs_array.iter().any(|b| b.as_str() == Some(&buff_id)) {
                buffs_array.push(serde_json::Value::String(buff_id));
            }
        }
        self.touch();
    }

    /// Remove a buff from the actor
    pub fn remove_buff(&mut self, buff_id: &str) -> bool {
        if let Some(buffs) = self.data.get_mut("buffs").and_then(|v| v.as_array_mut()) {
            if let Some(pos) = buffs.iter().position(|b| b.as_str() == Some(buff_id)) {
                buffs.remove(pos);
                self.touch();
                return true;
            }
        }
        false
    }

    /// Update the actor's version (different from touch)
    pub fn update_version(&mut self) {
        self.version += 1;
    }

    /// Get subsystems sorted by priority
    pub fn get_subsystem_by_priority(&self) -> Vec<&Subsystem> {
        let mut subsystems: Vec<&Subsystem> = self.subsystems.iter().collect();
        subsystems.sort_by(|a, b| b.priority.cmp(&a.priority));
        subsystems
    }

    /// Get the number of subsystems
    pub fn get_subsystem_count(&self) -> usize {
        self.subsystems.len()
    }

    /// Check if actor is a guild member
    pub fn is_guild_member(&self) -> bool {
        self.get_guild_id().is_some()
    }

    /// Get active buffs as a vector of strings
    pub fn get_active_buffs(&self) -> Vec<&str> {
        self.data.get("buffs")
            .and_then(|v| v.as_array())
            .map(|buffs| buffs.iter().filter_map(|b| b.as_str()).collect())
            .unwrap_or_default()
    }

    /// Clear all buffs
    pub fn clear_buffs(&mut self) {
        self.data.remove("buffs");
        self.touch();
    }

    /// Get combat duration in seconds
    pub fn get_combat_duration(&self) -> Option<i64> {
        if self.is_in_combat() {
            self.data.get("combat_start_time")
                .and_then(|v| v.as_i64())
                .map(|start_time| Utc::now().timestamp() - start_time)
        } else {
            None
        }
    }

    /// Set combat duration (for testing/debugging)
    pub fn set_combat_duration(&mut self, duration: i64) {
        let start_time = Utc::now().timestamp() - duration;
        self.data.insert("combat_start_time".to_string(), 
            serde_json::Value::Number(serde_json::Number::from(start_time)));
        self.touch();
    }

    /// Get last combat time
    pub fn get_last_combat_time(&self) -> Option<Timestamp> {
        self.data.get("last_combat_time")
            .and_then(|v| v.as_i64())
            .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
    }

    /// Check if actor is online
    pub fn is_online(&self) -> bool {
        self.data.get("online")
            .and_then(|v| v.as_bool())
            .unwrap_or(true) // Default to online
    }

    /// Set online status
    pub fn set_online(&mut self, online: bool) {
        self.data.insert("online".to_string(), serde_json::Value::Bool(online));
        if !online {
            self.data.insert("last_online_time".to_string(), 
                serde_json::Value::Number(serde_json::Number::from(Utc::now().timestamp())));
        }
        self.touch();
    }

    // === Missing Getter Methods (matching Go interface) ===
    
    /// Get the actor's ID
    pub fn get_id(&self) -> &EntityId {
        &self.id
    }

    /// Get the actor's name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the actor's race
    pub fn get_race(&self) -> &str {
        &self.race
    }

    /// Get the actor's lifespan
    pub fn get_lifespan(&self) -> i64 {
        self.lifespan
    }

    /// Get the actor's age
    pub fn get_age(&self) -> i64 {
        self.age
    }

    /// Get the creation timestamp
    pub fn get_created_at(&self) -> Timestamp {
        self.created_at
    }

    /// Get the last update timestamp
    pub fn get_updated_at(&self) -> Timestamp {
        self.updated_at
    }

    /// Get the actor's version
    pub fn get_version(&self) -> Version {
        self.version
    }

    /// Get the actor's subsystems
    pub fn get_subsystems(&self) -> &[Subsystem] {
        &self.subsystems
    }

    /// Get the actor's data
    pub fn get_data(&self) -> &HashMap<String, serde_json::Value> {
        &self.data
    }

    // === Missing Setter Methods (matching Go interface) ===

    /// Set the actor's name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.touch();
    }

    /// Set the actor's race
    pub fn set_race(&mut self, race: String) {
        self.race = race;
        self.touch();
    }

    /// Set the actor's lifespan
    pub fn set_lifespan(&mut self, lifespan: i64) {
        self.lifespan = lifespan;
        self.touch();
    }

    /// Set the actor's age
    pub fn set_age(&mut self, age: i64) {
        self.age = age;
        self.touch();
    }

    /// Set the last update timestamp
    pub fn set_updated_at(&mut self, updated_at: Timestamp) {
        self.updated_at = updated_at;
    }

    /// Set the actor's version
    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    /// Set the actor's subsystems
    pub fn set_subsystems(&mut self, subsystems: Vec<Subsystem>) {
        self.subsystems = subsystems;
        self.touch();
    }

    /// Set the actor's data
    pub fn set_data(&mut self, data: HashMap<String, serde_json::Value>) {
        self.data = data;
        self.touch();
    }
}

impl GameEntity for Actor {
    fn id(&self) -> EntityId {
        self.id
    }

    fn version(&self) -> Version {
        self.version
    }

    fn created_at(&self) -> Timestamp {
        self.created_at
    }

    fn updated_at(&self) -> Timestamp {
        self.updated_at
    }
}

/// Subsystem represents a game system that contributes to actor stats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subsystem {
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

impl Subsystem {
    /// Create a new subsystem.
    pub fn new(system_id: String, priority: i64) -> Self {
        Self {
            system_id,
            priority,
            enabled: true,
            config: HashMap::new(),
            data: HashMap::new(),
        }
    }

    /// Check if the subsystem is valid.
    pub fn is_valid(&self) -> bool {
        !self.system_id.is_empty() && self.priority >= 0
    }

    // === Missing Subsystem Methods (matching Go interface) ===

    /// Get the system ID
    pub fn get_system_id(&self) -> &str {
        &self.system_id
    }

    /// Get the priority
    pub fn get_priority(&self) -> i64 {
        self.priority
    }

    /// Check if the subsystem is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the configuration
    pub fn get_config(&self) -> &HashMap<String, serde_json::Value> {
        &self.config
    }

    /// Get the data
    pub fn get_data(&self) -> &HashMap<String, serde_json::Value> {
        &self.data
    }

    /// Set the configuration
    pub fn set_config(&mut self, config: HashMap<String, serde_json::Value>) {
        self.config = config;
    }

    /// Set the data
    pub fn set_data(&mut self, data: HashMap<String, serde_json::Value>) {
        self.data = data;
    }

    /// Set the enabled status
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Contribution represents a stat modification from a subsystem.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contribution {
    /// The dimension being modified (e.g., "strength", "health")
    pub dimension: String,
    /// The bucket type for this contribution
    pub bucket: Bucket,
    /// The value of the contribution
    pub value: f64,
    /// The system that made this contribution
    pub system: String,
    /// Priority of this contribution (optional)
    pub priority: Option<i64>,
    /// Additional tags for this contribution
    pub tags: Option<HashMap<String, String>>,
}

impl Contribution {
    /// Create a new contribution.
    pub fn new(dimension: String, bucket: Bucket, value: f64, system: String) -> Self {
        Self {
            dimension,
            bucket,
            value,
            system,
            priority: None,
            tags: None,
        }
    }

    /// Check if the contribution is valid.
    pub fn is_valid(&self) -> bool {
        !self.dimension.is_empty() 
            && !self.system.is_empty() 
            && self.priority.unwrap_or(0) >= 0
            && self.value.is_finite()
    }
}

/// CapContribution represents a cap modification from a subsystem.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapContribution {
    /// The system that made this contribution
    pub system: String,
    /// The dimension being capped
    pub dimension: String,
    /// The cap mode
    pub mode: CapMode,
    /// The kind of cap (min, max)
    pub kind: String,
    /// The value of the cap
    pub value: f64,
    /// Priority of this cap contribution
    pub priority: Option<i64>,
    /// The scope of the cap
    pub scope: Option<String>,
    /// The realm for realm-scoped caps
    pub realm: Option<String>,
    /// Additional tags for this cap contribution
    pub tags: Option<HashMap<String, String>>,
}

impl CapContribution {
    /// Create a new cap contribution.
    pub fn new(
        system: String,
        dimension: String,
        mode: CapMode,
        kind: String,
        value: f64,
    ) -> Self {
        Self {
            system,
            dimension,
            mode,
            kind,
            value,
            priority: None,
            scope: None,
            realm: None,
            tags: None,
        }
    }

    /// Check if the cap contribution is valid.
    pub fn is_valid(&self) -> bool {
        !self.system.is_empty() 
            && !self.dimension.is_empty() 
            && !self.kind.is_empty()
            && self.priority.unwrap_or(0) >= 0
            && self.value.is_finite()
    }
}

/// SubsystemOutput represents the output from a subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemOutput {
    /// Primary stat contributions
    pub primary: Vec<Contribution>,
    /// Derived stat contributions
    pub derived: Vec<Contribution>,
    /// Cap contributions
    pub caps: Vec<CapContribution>,
    /// Context modifiers for temporary effects
    pub context: Option<ModifierPack>,
    /// Metadata about the subsystem
    pub meta: SubsystemMeta,
}

impl SubsystemOutput {
    /// Create a new subsystem output.
    pub fn new(system_id: String) -> Self {
        Self {
            primary: Vec::new(),
            derived: Vec::new(),
            caps: Vec::new(),
            context: None,
            meta: SubsystemMeta::new(system_id),
        }
    }

    /// Add a primary contribution.
    pub fn add_primary(&mut self, contribution: Contribution) {
        self.primary.push(contribution);
    }

    /// Add a derived contribution.
    pub fn add_derived(&mut self, contribution: Contribution) {
        self.derived.push(contribution);
    }

    /// Add a cap contribution.
    pub fn add_cap(&mut self, cap: CapContribution) {
        self.caps.push(cap);
    }
}

/// SubsystemMeta contains metadata about a subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemMeta {
    /// The system ID
    pub system: String,
    /// Additional metadata
    pub data: HashMap<String, serde_json::Value>,
}

impl SubsystemMeta {
    /// Create new subsystem metadata.
    pub fn new(system: String) -> Self {
        Self {
            system,
            data: HashMap::new(),
        }
    }
}

/// ModifierPack contains context modifiers for temporary effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierPack {
    /// Additive percentage modifiers
    pub additive_percent: HashMap<String, f64>,
    /// Multiplicative modifiers
    pub multipliers: HashMap<String, f64>,
    /// Post-additive modifiers
    pub post_add: HashMap<String, f64>,
}

impl ModifierPack {
    /// Create a new modifier pack.
    pub fn new() -> Self {
        Self {
            additive_percent: HashMap::new(),
            multipliers: HashMap::new(),
            post_add: HashMap::new(),
        }
    }

    /// Apply modifiers to a value.
    pub fn apply(&self, dimension: &str, base_value: f64) -> f64 {
        let mut value = base_value;

        // Apply additive percentage
        if let Some(percent) = self.additive_percent.get(dimension) {
            value += base_value * (percent / 100.0);
        }

        // Apply multipliers
        if let Some(multiplier) = self.multipliers.get(dimension) {
            value *= multiplier;
        }

        // Apply post-add
        if let Some(add) = self.post_add.get(dimension) {
            value += add;
        }

        value
    }
}

impl Default for ModifierPack {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot represents the final aggregated state of an actor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// The actor ID this snapshot belongs to
    pub actor_id: EntityId,
    /// Primary stats after aggregation
    pub primary: HashMap<String, f64>,
    /// Derived stats after aggregation
    pub derived: HashMap<String, f64>,
    /// Effective caps used
    pub caps_used: HashMap<String, Caps>,
    /// Version of the actor when snapshot was created
    pub version: Version,
    /// When the snapshot was created
    pub created_at: Timestamp,
    /// Which subsystems were processed
    pub subsystems_processed: Vec<String>,
    /// Processing time in microseconds
    pub processing_time: Option<u64>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Snapshot {
    /// Create a new snapshot.
    pub fn new(actor_id: EntityId, version: Version) -> Self {
        Self {
            actor_id,
            primary: HashMap::new(),
            derived: HashMap::new(),
            caps_used: HashMap::new(),
            version,
            created_at: Utc::now(),
            subsystems_processed: Vec::new(),
            processing_time: None,
            metadata: HashMap::new(),
        }
    }

    /// Check if the snapshot is valid.
    pub fn is_valid(&self) -> bool {
        self.version > 0
    }

    /// Get a primary stat value.
    pub fn get_primary(&self, dimension: &str) -> Option<f64> {
        self.primary.get(dimension).copied()
    }

    /// Get a derived stat value.
    pub fn get_derived(&self, dimension: &str) -> Option<f64> {
        self.derived.get(dimension).copied()
    }

    /// Get effective caps for a dimension.
    pub fn get_caps(&self, dimension: &str) -> Option<&Caps> {
        self.caps_used.get(dimension)
    }

    /// Clone the snapshot with a new timestamp.
    pub fn clone_with_new_timestamp(&self) -> Self {
        let mut snapshot = self.clone();
        snapshot.created_at = Utc::now();
        snapshot
    }

    /// Add a primary contribution to the snapshot.
    pub fn add_primary(&mut self, contribution: Contribution) {
        let current_value = self.primary.get(&contribution.dimension).unwrap_or(&0.0);
        let new_value = match contribution.bucket {
            Bucket::Flat => current_value + contribution.value,
            Bucket::Mult => current_value * contribution.value,
            Bucket::PostAdd => current_value + contribution.value,
            Bucket::Override => contribution.value,
            #[cfg(feature = "extra_buckets")]
            Bucket::Exponential => current_value * (1.0 + contribution.value),
            #[cfg(feature = "extra_buckets")]
            Bucket::Logarithmic => current_value + (contribution.value * current_value.ln()),
            #[cfg(feature = "extra_buckets")]
            Bucket::Conditional => if contribution.value > 0.0 { contribution.value } else { *current_value },
        };
        self.primary.insert(contribution.dimension, new_value);
    }

    /// Add a derived contribution to the snapshot.
    pub fn add_derived(&mut self, contribution: Contribution) {
        let current_value = self.derived.get(&contribution.dimension).unwrap_or(&0.0);
        let new_value = match contribution.bucket {
            Bucket::Flat => current_value + contribution.value,
            Bucket::Mult => current_value * contribution.value,
            Bucket::PostAdd => current_value + contribution.value,
            Bucket::Override => contribution.value,
            #[cfg(feature = "extra_buckets")]
            Bucket::Exponential => current_value * (1.0 + contribution.value),
            #[cfg(feature = "extra_buckets")]
            Bucket::Logarithmic => current_value + (contribution.value * current_value.ln()),
            #[cfg(feature = "extra_buckets")]
            Bucket::Conditional => if contribution.value > 0.0 { contribution.value } else { *current_value },
        };
        self.derived.insert(contribution.dimension, new_value);
    }

    /// Add a cap contribution to the snapshot.
    pub fn add_cap(&mut self, cap_contribution: CapContribution) {
        let caps = self.caps_used.entry(cap_contribution.dimension.clone())
            .or_insert_with(|| Caps::new(0.0, 1000.0));
        
        match cap_contribution.mode {
            CapMode::Baseline => {
                caps.set_min(cap_contribution.value);
                caps.set_max(cap_contribution.value);
            },
            CapMode::Additive => {
                caps.expand(cap_contribution.value);
            },
            CapMode::HardMax => {
                caps.set_max(cap_contribution.value);
            },
            CapMode::HardMin => {
                caps.set_min(cap_contribution.value);
            },
            CapMode::Override => {
                caps.set_min(cap_contribution.value);
                caps.set_max(cap_contribution.value);
            },
            CapMode::SoftMax => {
                // SoftMax allows exceeding the cap but applies a penalty
                // For now, treat it the same as HardMax
                caps.set_max(cap_contribution.value);
            },
        }
    }
}

/// Caps represents min/max limits for a dimension.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Caps {
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
}

impl Caps {
    /// Create new caps.
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Check if the caps are valid.
    pub fn is_valid(&self) -> bool {
        self.min <= self.max && self.min.is_finite() && self.max.is_finite()
    }

    /// Clamp a value to these caps.
    pub fn clamp(&self, value: f64) -> f64 {
        value.max(self.min).min(self.max)
    }

    /// Get the intersection of two caps.
    pub fn intersection(&self, other: &Caps) -> Caps {
        Caps::new(
            self.min.max(other.min),
            self.max.min(other.max),
        )
    }

    /// Get the union of two caps.
    pub fn union(&self, other: &Caps) -> Caps {
        Caps::new(
            self.min.min(other.min),
            self.max.max(other.max),
        )
    }

    /// Check if a value is within the caps range
    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }

    /// Check if the caps range is empty (min > max)
    pub fn is_empty(&self) -> bool {
        self.min > self.max
    }

    /// Get the range size (max - min)
    pub fn get_range(&self) -> f64 {
        self.max - self.min
    }

    /// Get the center point of the range
    pub fn get_center(&self) -> f64 {
        (self.min + self.max) / 2.0
    }

    /// Expand the range by the given amount
    pub fn expand(&mut self, amount: f64) {
        self.min -= amount;
        self.max += amount;
    }

    /// Shrink the range by the given amount
    pub fn shrink(&mut self, amount: f64) {
        self.min += amount;
        self.max -= amount;
        // Ensure min doesn't exceed max
        if self.min > self.max {
            let center = (self.min + self.max) / 2.0;
            self.min = center;
            self.max = center;
        }
    }

    /// Set both min and max values
    pub fn set(&mut self, min: f64, max: f64) {
        self.min = min;
        self.max = max;
    }

    /// Get the minimum value
    pub fn get_min(&self) -> f64 {
        self.min
    }

    /// Get the maximum value
    pub fn get_max(&self) -> f64 {
        self.max
    }

    /// Set the minimum value
    pub fn set_min(&mut self, min: f64) {
        self.min = min;
    }

    /// Set the maximum value
    pub fn set_max(&mut self, max: f64) {
        self.max = max;
    }
}