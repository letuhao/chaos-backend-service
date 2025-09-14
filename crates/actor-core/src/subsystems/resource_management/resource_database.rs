//! Resource Database Integration
//!
//! This module provides MongoDB integration for storing inactive actor resources
//! to reduce memory overhead in the Enhanced Hybrid Resource Manager.

use async_trait::async_trait;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ActorCoreResult;

/// Convert string error to ActorCoreError
#[allow(dead_code)]
fn to_actor_core_error(msg: String) -> crate::ActorCoreError {
    crate::ActorCoreError::SubsystemError(msg)
}

/// MongoDB Resource Database implementation
#[cfg(feature = "mongodb-storage")]
#[derive(Debug)]
pub struct MongoResourceDatabase {
    /// MongoDB client
    client: mongodb::Client,
    /// Database name
    database_name: String,
    /// Collection name for actor resources
    collection_name: String,
}

/// Actor resource document stored in MongoDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorResourceDocument {
    /// Actor ID
    pub actor_id: String,
    /// Resource values
    pub resources: HashMap<String, f64>,
    /// Last updated timestamp
    pub last_updated: u64,
    /// Actor status (active/inactive)
    pub status: ActorStatus,
    /// Resource version for conflict resolution
    pub version: u64,
}

/// Actor status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorStatus {
    Active,
    Inactive,
}

#[cfg(feature = "mongodb-storage")]
impl MongoResourceDatabase {
    /// Create a new MongoDB Resource Database
    pub async fn new(connection_string: &str, database_name: &str) -> ActorCoreResult<Self> {
        let client = mongodb::Client::with_uri_str(connection_string)
            .await
            .map_err(|e| to_actor_core_error(format!("Failed to connect to MongoDB: {}", e)))?;
        
        Ok(Self {
            client,
            database_name: database_name.to_string(),
            collection_name: "actor_resources".to_string(),
        })
    }
    
    /// Get the database reference
    fn get_database(&self) -> mongodb::Database {
        self.client.database(&self.database_name)
    }
    
    /// Get the collection reference
    fn get_collection(&self) -> mongodb::Collection<ActorResourceDocument> {
        self.get_database().collection(&self.collection_name)
    }
    
    /// Create indexes for better performance
    pub async fn create_indexes(&self) -> ActorCoreResult<()> {
        let collection = self.get_collection();
        
        // Create index on actor_id for fast lookups
        let index_model = mongodb::IndexModel::builder()
            .keys(bson::doc! { "actor_id": 1 })
            .build();
        
        collection.create_index(index_model, None)
            .await
            .map_err(|e| to_actor_core_error(format!("Failed to create index: {}", e)))?;
        
        // Create index on status for filtering active/inactive actors
        let index_model = mongodb::IndexModel::builder()
            .keys(bson::doc! { "status": 1 })
            .build();
        
        collection.create_index(index_model, None)
            .await
            .map_err(|e| to_actor_core_error(format!("Failed to create status index: {}", e)))?;
        
        Ok(())
    }
    
    /// Get actor resource document
    async fn get_actor_document(&self, actor_id: &str) -> ActorCoreResult<Option<ActorResourceDocument>> {
        let collection = self.get_collection();
        let filter = bson::doc! { "actor_id": actor_id };
        
        let document = collection.find_one(filter, None)
            .await
            .map_err(|e| to_actor_core_error(format!("Failed to find actor document: {}", e)))?;
        
        Ok(document)
    }
    
    /// Update actor resource document
    async fn update_actor_document(&self, document: &ActorResourceDocument) -> ActorCoreResult<()> {
        let collection = self.get_collection();
        let filter = bson::doc! { 
            "actor_id": &document.actor_id,
            "version": { "$lt": document.version as i64 }
        };
        
        let update = bson::doc! {
            "$set": {
                "resources": bson::to_bson(&document.resources)
                    .map_err(|e| to_actor_core_error(format!("Failed to serialize resources: {}", e)))?,
                "last_updated": document.last_updated as i64,
                "status": bson::to_bson(&document.status)
                    .map_err(|e| to_actor_core_error(format!("Failed to serialize status: {}", e)))?,
                "version": document.version as i64,
            }
        };
        
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
        
        let result = collection.update_one(filter, update, options)
            .await
            .map_err(|e| to_actor_core_error(format!("Failed to update actor document: {}", e)))?;
        
        if result.upserted_id.is_none() && result.modified_count == 0 {
            return Err(to_actor_core_error("Failed to update actor document - version conflict".to_string()));
        }
        
        Ok(())
    }
}

#[cfg(feature = "mongodb-storage")]
#[async_trait]
impl super::enhanced_hybrid_resource_manager::ResourceDatabase for MongoResourceDatabase {
    /// Store actor resources
    async fn store_actor_resources(&self, actor_id: &str, resources: &HashMap<String, f64>) -> ActorCoreResult<()> {
        let existing_doc = self.get_actor_document(actor_id).await?;
        let version = existing_doc.map(|doc| doc.version + 1).unwrap_or(1);
        
        let document = ActorResourceDocument {
            actor_id: actor_id.to_string(),
            resources: resources.clone(),
            last_updated: chrono::Utc::now().timestamp() as u64,
            status: ActorStatus::Active,
            version,
        };
        
        self.update_actor_document(&document).await?;
        Ok(())
    }
    
    /// Load actor resources
    async fn load_actor_resources(&self, actor_id: &str) -> ActorCoreResult<HashMap<String, f64>> {
        let document = self.get_actor_document(actor_id).await?;
        
        match document {
            Some(doc) => Ok(doc.resources),
            None => Ok(HashMap::new()),
        }
    }
    
    /// Check if actor is active
    async fn is_actor_active(&self, actor_id: &str) -> ActorCoreResult<bool> {
        let document = self.get_actor_document(actor_id).await?;
        
        match document {
            Some(doc) => Ok(matches!(doc.status, ActorStatus::Active)),
            None => Ok(true), // New actors are considered active
        }
    }
    
    /// Mark actor as inactive
    async fn mark_actor_inactive(&self, actor_id: &str) -> ActorCoreResult<()> {
        let existing_doc = self.get_actor_document(actor_id).await?;
        let version = existing_doc.as_ref().map(|doc| doc.version + 1).unwrap_or(1);
        
        let document = ActorResourceDocument {
            actor_id: actor_id.to_string(),
            resources: existing_doc.map(|doc| doc.resources).unwrap_or_default(),
            last_updated: chrono::Utc::now().timestamp() as u64,
            status: ActorStatus::Inactive,
            version,
        };
        
        self.update_actor_document(&document).await?;
        Ok(())
    }
    
    /// Mark actor as active
    async fn mark_actor_active(&self, actor_id: &str) -> ActorCoreResult<()> {
        let existing_doc = self.get_actor_document(actor_id).await?;
        let version = existing_doc.as_ref().map(|doc| doc.version + 1).unwrap_or(1);
        
        let document = ActorResourceDocument {
            actor_id: actor_id.to_string(),
            resources: existing_doc.map(|doc| doc.resources).unwrap_or_default(),
            last_updated: chrono::Utc::now().timestamp() as u64,
            status: ActorStatus::Active,
            version,
        };
        
        self.update_actor_document(&document).await?;
        Ok(())
    }
}

/// In-memory Resource Database for testing
#[derive(Debug)]
pub struct InMemoryResourceDatabase {
    /// In-memory storage
    storage: std::sync::Arc<tokio::sync::RwLock<HashMap<String, ActorResourceDocument>>>,
}

impl InMemoryResourceDatabase {
    /// Create a new in-memory database
    pub fn new() -> Self {
        Self {
            storage: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl super::enhanced_hybrid_resource_manager::ResourceDatabase for InMemoryResourceDatabase {
    /// Store actor resources
    async fn store_actor_resources(&self, actor_id: &str, resources: &HashMap<String, f64>) -> ActorCoreResult<()> {
        let mut storage = self.storage.write().await;
        let existing_doc = storage.get(actor_id);
        let version = existing_doc.map(|doc| doc.version + 1).unwrap_or(1);
        
        let document = ActorResourceDocument {
            actor_id: actor_id.to_string(),
            resources: resources.clone(),
            last_updated: chrono::Utc::now().timestamp() as u64,
            status: ActorStatus::Active,
            version,
        };
        
        storage.insert(actor_id.to_string(), document);
        Ok(())
    }
    
    /// Load actor resources
    async fn load_actor_resources(&self, actor_id: &str) -> ActorCoreResult<HashMap<String, f64>> {
        let storage = self.storage.read().await;
        
        match storage.get(actor_id) {
            Some(doc) => Ok(doc.resources.clone()),
            None => Ok(HashMap::new()),
        }
    }
    
    /// Check if actor is active
    async fn is_actor_active(&self, actor_id: &str) -> ActorCoreResult<bool> {
        let storage = self.storage.read().await;
        
        match storage.get(actor_id) {
            Some(doc) => Ok(matches!(doc.status, ActorStatus::Active)),
            None => Ok(true), // New actors are considered active
        }
    }
    
    /// Mark actor as inactive
    async fn mark_actor_inactive(&self, actor_id: &str) -> ActorCoreResult<()> {
        let mut storage = self.storage.write().await;
        
        if let Some(doc) = storage.get(actor_id) {
            let mut updated_doc = doc.clone();
            updated_doc.status = ActorStatus::Inactive;
            updated_doc.version += 1;
            updated_doc.last_updated = chrono::Utc::now().timestamp() as u64;
            storage.insert(actor_id.to_string(), updated_doc);
        } else {
            let document = ActorResourceDocument {
                actor_id: actor_id.to_string(),
                resources: HashMap::new(),
                last_updated: chrono::Utc::now().timestamp() as u64,
                status: ActorStatus::Inactive,
                version: 1,
            };
            storage.insert(actor_id.to_string(), document);
        }
        
        Ok(())
    }
    
    /// Mark actor as active
    async fn mark_actor_active(&self, actor_id: &str) -> ActorCoreResult<()> {
        let mut storage = self.storage.write().await;
        
        if let Some(doc) = storage.get(actor_id) {
            let mut updated_doc = doc.clone();
            updated_doc.status = ActorStatus::Active;
            updated_doc.version += 1;
            updated_doc.last_updated = chrono::Utc::now().timestamp() as u64;
            storage.insert(actor_id.to_string(), updated_doc);
        } else {
            let document = ActorResourceDocument {
                actor_id: actor_id.to_string(),
                resources: HashMap::new(),
                last_updated: chrono::Utc::now().timestamp() as u64,
                status: ActorStatus::Active,
                version: 1,
            };
            storage.insert(actor_id.to_string(), document);
        }
        
        Ok(())
    }
}