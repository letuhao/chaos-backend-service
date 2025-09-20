//! Resource Database implementations
//!
//! This module provides database implementations for storing and retrieving
//! actor resource data, including both MongoDB and in-memory implementations.
//!
//! NOTE: Legacy ResourceDatabase trait implementations have been removed
//! to maintain the pure hub architecture. Use Runtime Registry System instead.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Legacy ResourceDatabase trait implementations removed - use Runtime Registry System instead

/// In-memory Resource Database for testing
#[derive(Debug, Clone)]
pub struct InMemoryResourceDatabase {
    /// In-memory storage
    #[allow(dead_code)]
    storage: Arc<RwLock<HashMap<String, HashMap<String, f64>>>>,
}

impl InMemoryResourceDatabase {
    /// Create a new in-memory resource database
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryResourceDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// MongoDB Resource Database implementation
#[cfg(feature = "mongodb-storage")]
pub struct MongoResourceDatabase {
    /// MongoDB client
    #[allow(dead_code)]
    client: mongodb::Client,
    /// Database name
    #[allow(dead_code)]
    database_name: String,
    /// Collection name for actor resources
    #[allow(dead_code)]
    collection_name: String,
}

#[cfg(feature = "mongodb-storage")]
impl MongoResourceDatabase {
    /// Create a new MongoDB resource database
    pub fn new(client: mongodb::Client, database_name: String, collection_name: String) -> Self {
        Self {
            client,
            database_name,
            collection_name,
        }
    }
}
