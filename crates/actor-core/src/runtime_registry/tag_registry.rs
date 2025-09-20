//! Tag registry for dynamic tag management

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tracing::{info, warn};

use crate::ActorCoreResult;

/// Tag definition for runtime registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub tag_type: TagType,
    pub subsystem_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tag type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TagType {
    Resource,
    Category,
    Action,
    Element,
    Status,
    Buff,
    Debuff,
    Behavior,
    Type,
    Custom(String),
}

/// Tag registry trait
#[async_trait]
pub trait TagRegistry: Send + Sync {
    /// Register a tag definition
    async fn register_tag(&self, tag: TagDefinition) -> ActorCoreResult<()>;
    
    /// Get all registered tags
    async fn get_all_tags(&self) -> ActorCoreResult<Vec<TagDefinition>>;
    
    /// Get tag by ID
    async fn get_tag(&self, id: &str) -> ActorCoreResult<Option<TagDefinition>>;
    
    /// Check if tag exists
    async fn has_tag(&self, id: &str) -> ActorCoreResult<bool>;
    
    /// Get tags by type
    async fn get_tags_by_type(&self, tag_type: &TagType) -> ActorCoreResult<Vec<TagDefinition>>;
    
    /// Unregister tag
    async fn unregister_tag(&self, id: &str) -> ActorCoreResult<()>;
    
    /// Get tags by subsystem
    async fn get_tags_by_subsystem(&self, subsystem_id: &str) -> ActorCoreResult<Vec<TagDefinition>>;
}

/// Tag registry implementation
pub struct TagRegistryImpl {
    tags: Arc<RwLock<HashMap<String, TagDefinition>>>,
    metrics: Arc<RwLock<TagRegistryMetrics>>,
}

impl TagRegistryImpl {
    pub fn new() -> Self {
        Self {
            tags: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(TagRegistryMetrics::default())),
        }
    }
}

#[async_trait]
impl TagRegistry for TagRegistryImpl {
    async fn register_tag(&self, tag: TagDefinition) -> ActorCoreResult<()> {
        let tag_id = tag.id.clone();
        
        if tag_id.is_empty() {
            return Err(crate::ActorCoreError::RegistryError(
                "Tag ID cannot be empty".to_string()
            ));
        }

        let mut tags = self.tags.write();
        
        if tags.contains_key(&tag_id) {
            warn!("Overwriting existing tag: {}", tag_id);
        }
        
        tags.insert(tag_id.clone(), tag);
        
        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.registered_count = tags.len();
        metrics.registration_attempts += 1;
        
        info!("Registered tag: {}", tag_id);
        Ok(())
    }

    async fn get_all_tags(&self) -> ActorCoreResult<Vec<TagDefinition>> {
        let tags = self.tags.read();
        Ok(tags.values().cloned().collect())
    }

    async fn get_tag(&self, id: &str) -> ActorCoreResult<Option<TagDefinition>> {
        let tags = self.tags.read();
        Ok(tags.get(id).cloned())
    }

    async fn has_tag(&self, id: &str) -> ActorCoreResult<bool> {
        let tags = self.tags.read();
        Ok(tags.contains_key(id))
    }

    async fn get_tags_by_type(&self, tag_type: &TagType) -> ActorCoreResult<Vec<TagDefinition>> {
        let tags = self.tags.read();
        Ok(tags
            .values()
            .filter(|tag| &tag.tag_type == tag_type)
            .cloned()
            .collect())
    }

    async fn unregister_tag(&self, id: &str) -> ActorCoreResult<()> {
        let mut tags = self.tags.write();
        
        if tags.remove(id).is_some() {
            // Update metrics
            let mut metrics = self.metrics.write();
            metrics.registered_count = tags.len();
            metrics.unregistration_attempts += 1;
            
            info!("Unregistered tag: {}", id);
            Ok(())
        } else {
            Err(crate::ActorCoreError::RegistryError(
                format!("Tag not found: {}", id)
            ))
        }
    }

    async fn get_tags_by_subsystem(&self, subsystem_id: &str) -> ActorCoreResult<Vec<TagDefinition>> {
        let tags = self.tags.read();
        Ok(tags
            .values()
            .filter(|tag| tag.subsystem_id == subsystem_id)
            .cloned()
            .collect())
    }
}

/// Tag registry metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagRegistryMetrics {
    pub registered_count: usize,
    pub registration_attempts: u64,
    pub unregistration_attempts: u64,
    pub lookup_attempts: u64,
}

impl Default for TagRegistryMetrics {
    fn default() -> Self {
        Self {
            registered_count: 0,
            registration_attempts: 0,
            unregistration_attempts: 0,
            lookup_attempts: 0,
        }
    }
}
