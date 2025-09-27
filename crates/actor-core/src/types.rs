//! Core types for the Actor Core system.
//!
//! This module defines the fundamental data structures used throughout
//! the actor stat aggregation system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::enums::{Bucket, CapMode, AcrossLayerPolicy, Operator};
use crate::ActorCoreResult;

/// Actor represents a character with stats, buffs, and subsystems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    /// Unique identifier for the actor
    pub id: String,
    /// Actor name
    pub name: String,
    /// Actor race/class
    pub race: String,
    /// Current level
    pub level: i64,
    /// Core resources (health, mana, stamina, etc.)
    pub core_resources: [f64; 9],
    /// Custom resources (flexible HashMap for game-specific resources)
    pub custom_resources: HashMap<String, f64>,
    /// Subsystems attached to this actor
    pub subsystems: Vec<String>,
    /// Data (for compatibility)
    pub data: HashMap<String, serde_json::Value>,
    /// Version (for compatibility)
    pub version: i64,
    /// Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Actor {
    /// Create a new actor with basic information
    pub fn new(id: String, race: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name: String::new(),
            race,
            level: 1,
            core_resources: [100.0, 100.0, 100.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0], // Default values
            custom_resources: HashMap::new(),
            subsystems: Vec::new(),
            data: HashMap::new(),
            version: 1,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set data
    pub fn set_data(&mut self, data: HashMap<String, serde_json::Value>) {
        self.data = data;
    }

    /// Get data
    pub fn get_data(&self) -> &HashMap<String, serde_json::Value> {
        &self.data
    }

    /// Create a simple actor for testing
    pub fn simple(id: &str, race: &str, level: i64) -> Self {
        let mut actor = Self::new(id.to_string(), race.to_string());
        actor.level = level;
        actor
    }
}

/// Contribution represents a stat modification from a subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
    /// Stat name being modified
    pub stat_name: String,
    /// Bucket type for processing
    pub bucket: Bucket,
    /// Value to contribute
    pub value: f64,
    /// Source subsystem
    pub source: String,
    /// Priority for ordering
    pub priority: Option<i64>,
    /// Dimension (for compatibility)
    pub dimension: String,
    /// System (for compatibility)
    pub system: String,
    /// Tags (for compatibility)
    pub tags: Option<Vec<String>>,
    /// Metadata
    pub created_at: DateTime<Utc>,
}

impl Contribution {
    /// Create a new contribution
    pub fn new(stat_name: String, bucket: Bucket, value: f64, source: String) -> Self {
        Self {
            stat_name: stat_name.clone(),
            bucket,
            value,
            source: source.clone(),
            priority: None,
            dimension: stat_name,
            system: source,
            tags: None,
            created_at: Utc::now(),
        }
    }

    /// Create a contribution with priority
    pub fn with_priority(stat_name: String, bucket: Bucket, value: f64, source: String, priority: i64) -> Self {
        Self {
            stat_name: stat_name.clone(),
            bucket,
            value,
            source: source.clone(),
            priority: Some(priority),
            dimension: stat_name,
            system: source,
            tags: None,
            created_at: Utc::now(),
        }
    }
}

/// CapContribution represents a cap constraint from a subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapContribution {
    /// Stat name being capped
    pub stat_name: String,
    /// Cap mode
    pub cap_mode: CapMode,
    /// Minimum value
    pub min_value: Option<f64>,
    /// Maximum value
    pub max_value: Option<f64>,
    /// Source subsystem
    pub source: String,
    /// Layer name
    pub layer: String,
    /// Priority for ordering
    pub priority: i64,
    /// Kind (for compatibility)
    pub kind: String,
    /// Value (for compatibility)
    pub value: f64,
    /// Mode (for compatibility)
    pub mode: CapMode,
    /// Dimension (for compatibility)
    pub dimension: String,
    /// System (for compatibility)
    pub system: String,
    /// Scope (for compatibility)
    pub scope: Option<String>,
    /// Metadata
    pub created_at: DateTime<Utc>,
}

impl CapContribution {
    /// Create a new cap contribution
    pub fn new(stat_name: String, cap_mode: CapMode, source: String, layer: String) -> Self {
        Self {
            stat_name: stat_name.clone(),
            cap_mode,
            min_value: None,
            max_value: None,
            source: source.clone(),
            layer,
            priority: 0,
            kind: "min".to_string(),
            value: 0.0,
            mode: cap_mode,
            dimension: stat_name,
            system: source,
            scope: None,
            created_at: Utc::now(),
        }
    }

    /// Create a cap contribution with min/max values
    pub fn with_values(stat_name: String, cap_mode: CapMode, min_value: Option<f64>, max_value: Option<f64>, source: String, layer: String) -> Self {
        Self {
            stat_name: stat_name.clone(),
            cap_mode,
            min_value,
            max_value,
            source: source.clone(),
            layer,
            priority: 0,
            kind: "min".to_string(),
            value: 0.0,
            mode: cap_mode,
            dimension: stat_name,
            system: source,
            scope: None,
            created_at: Utc::now(),
        }
    }
}

/// SubsystemOutput represents the output from a subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemOutput {
    /// System ID that generated this output
    pub system_id: String,
    /// Primary contributions from this subsystem
    pub primary: Vec<Contribution>,
    /// Derived contributions from this subsystem
    pub derived: Vec<Contribution>,
    /// Cap contributions from this subsystem
    pub caps: Vec<CapContribution>,
    /// Processing time in microseconds
    pub processing_time: u64,
    /// Context (for compatibility)
    pub context: Option<HashMap<String, serde_json::Value>>,
    /// Metadata
    pub meta: SubsystemMeta,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

impl SubsystemOutput {
    /// Create a new subsystem output
    pub fn new(system_id: String) -> Self {
        Self {
            system_id: system_id.clone(),
            primary: Vec::new(),
            derived: Vec::new(),
            caps: Vec::new(),
            processing_time: 0,
            context: None,
            meta: SubsystemMeta::new(system_id, 0),
            created_at: Utc::now(),
        }
    }

    /// Add a contribution
    pub fn add_contribution(&mut self, contribution: Contribution) {
        self.primary.push(contribution);
    }

    /// Add a cap contribution
    pub fn add_cap_contribution(&mut self, cap_contribution: CapContribution) {
        self.caps.push(cap_contribution);
    }
}

/// Snapshot represents the final aggregated stat state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Actor ID this snapshot belongs to
    pub actor_id: String,
    /// Primary stat values
    pub primary: HashMap<String, f64>,
    /// Derived stat values
    pub derived: HashMap<String, f64>,
    /// Effective caps for each stat
    pub caps_used: HashMap<String, Caps>,
    /// Version
    pub version: i64,
    /// Processing metadata
    pub processing_time: Option<u64>,
    /// Number of subsystems processed
    pub subsystems_processed: Vec<String>,
    /// Cache hit/miss information
    pub cache_hit: bool,
    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

impl Snapshot {
    /// Create a new snapshot
    pub fn new(actor_id: String) -> Self {
        Self {
            actor_id,
            primary: HashMap::new(),
            derived: HashMap::new(),
            caps_used: HashMap::new(),
            version: 1,
            processing_time: None,
            subsystems_processed: Vec::new(),
            cache_hit: false,
            metadata: HashMap::new(),
            created_at: Utc::now(),
        }
    }

    /// Get a stat value
    pub fn get_stat(&self, stat_name: &str) -> Option<f64> {
        self.primary.get(stat_name).copied()
    }

    /// Set a stat value
    pub fn set_stat(&mut self, stat_name: String, value: f64) {
        self.primary.insert(stat_name, value);
    }
}

/// Caps represents the effective min/max constraints for a stat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Caps {
    /// Stat name
    pub stat_name: String,
    /// Effective minimum value
    pub min: f64,
    /// Effective maximum value
    pub max: f64,
    /// Across layer policy
    pub across_layer_policy: AcrossLayerPolicy,
    /// Metadata
    pub created_at: DateTime<Utc>,
}

impl Caps {
    /// Create new caps
    pub fn new(stat_name: String, across_layer_policy: AcrossLayerPolicy) -> Self {
        Self {
            stat_name,
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            across_layer_policy,
            created_at: Utc::now(),
        }
    }

    /// Create caps with min/max values
    pub fn with_values(stat_name: String, min: f64, max: f64, across_layer_policy: AcrossLayerPolicy) -> Self {
        Self {
            stat_name,
            min,
            max,
            across_layer_policy,
            created_at: Utc::now(),
        }
    }

    /// Set minimum value
    pub fn set_min(&mut self, value: f64) {
        self.min = value;
    }

    /// Set maximum value
    pub fn set_max(&mut self, value: f64) {
        self.max = value;
    }

    /// Clamp a value to the caps
    pub fn clamp(&self, value: f64) -> f64 {
        value.max(self.min).min(self.max)
    }

    /// Check if caps are valid
    pub fn is_valid(&self) -> bool {
        self.min <= self.max
    }

    /// Intersect with another caps
    pub fn intersection(&self, other: &Caps) -> Caps {
        Caps {
            stat_name: self.stat_name.clone(),
            min: self.min.max(other.min),
            max: self.max.min(other.max),
            across_layer_policy: self.across_layer_policy,
            created_at: Utc::now(),
        }
    }

    /// Union with another caps
    pub fn union(&self, other: &Caps) -> Caps {
        Caps {
            stat_name: self.stat_name.clone(),
            min: self.min.min(other.min),
            max: self.max.max(other.max),
            across_layer_policy: self.across_layer_policy,
            created_at: Utc::now(),
        }
    }
}

/// ModifierPack represents a collection of modifiers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierPack {
    /// Pack ID
    pub id: String,
    /// Modifiers in this pack
    pub modifiers: HashMap<String, f64>,
    /// Source
    pub source: String,
    /// Metadata
    pub created_at: DateTime<Utc>,
}

impl ModifierPack {
    /// Create a new modifier pack
    pub fn new(id: String, source: String) -> Self {
        Self {
            id,
            modifiers: HashMap::new(),
            source,
            created_at: Utc::now(),
        }
    }

    /// Add a modifier
    pub fn add_modifier(&mut self, name: String, value: f64) {
        self.modifiers.insert(name, value);
    }
}

/// EffectiveCaps represents the final effective caps after all processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectiveCaps {
    /// Stat name
    pub stat_name: String,
    /// Final minimum value
    pub min_value: Option<f64>,
    /// Final maximum value
    pub max_value: Option<f64>,
    /// Metadata
    pub created_at: DateTime<Utc>,
}

impl EffectiveCaps {
    /// Create new effective caps
    pub fn new(stat_name: String) -> Self {
        Self {
            stat_name,
            min_value: None,
            max_value: None,
            created_at: Utc::now(),
        }
    }
}

/// SubsystemMeta represents metadata about a subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemMeta {
    /// System ID
    pub system_id: String,
    /// Priority
    pub priority: i64,
    /// Version
    pub version: String,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// System (for compatibility)
    pub system: String,
    /// Data (for compatibility)
    pub data: HashMap<String, serde_json::Value>,
    /// Metadata
    pub created_at: DateTime<Utc>,
}

impl SubsystemMeta {
    /// Create new subsystem metadata
    pub fn new(system_id: String, priority: i64) -> Self {
        Self {
            system_id: system_id.clone(),
            priority,
            version: "1.0.0".to_string(),
            dependencies: Vec::new(),
            system: system_id,
            data: HashMap::new(),
            created_at: Utc::now(),
        }
    }
}

/// Subsystem represents a subsystem implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subsystem {
    /// Metadata
    pub meta: SubsystemMeta,
    /// Configuration
    pub config: HashMap<String, serde_json::Value>,
}

impl Subsystem {
    /// Create a new subsystem
    pub fn new(meta: SubsystemMeta) -> Self {
        Self {
            meta,
            config: HashMap::new(),
        }
    }
}
