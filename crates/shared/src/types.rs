//! Common types used across the Chaos World backend.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Unique identifier for game entities.
pub type EntityId = Uuid;

/// Timestamp type for all time-based operations.
pub type Timestamp = DateTime<Utc>;

/// Version number for optimistic concurrency control.
pub type Version = u64;

/// Priority level for various operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    /// Lowest priority
    Low = 0,
    /// Normal priority
    Normal = 1,
    /// High priority
    High = 2,
    /// Critical priority
    Critical = 3,
}

/// Base trait for all game entities.
pub trait GameEntity {
    /// Get the unique identifier of this entity.
    fn id(&self) -> EntityId;
    
    /// Get the version of this entity.
    fn version(&self) -> Version;
    
    /// Get when this entity was created.
    fn created_at(&self) -> Timestamp;
    
    /// Get when this entity was last updated.
    fn updated_at(&self) -> Timestamp;
}

/// Metadata for game entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMetadata {
    /// Unique identifier
    pub id: EntityId,
    /// Version for optimistic concurrency control
    pub version: Version,
    /// Creation timestamp
    pub created_at: Timestamp,
    /// Last update timestamp
    pub updated_at: Timestamp,
    /// Additional metadata
    pub tags: std::collections::HashMap<String, String>,
}

impl EntityMetadata {
    /// Create new entity metadata.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            version: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags: std::collections::HashMap::new(),
        }
    }
    
    /// Update the version and timestamp.
    pub fn touch(&mut self) {
        self.version += 1;
        self.updated_at = Utc::now();
    }
}

impl Default for EntityMetadata {
    fn default() -> Self {
        Self::new()
    }
}
