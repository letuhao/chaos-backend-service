use actor_core::subsystems::resource_management::resource_database::{ActorResourceDocument, ActorStatus, InMemoryResourceDatabase};
use actor_core::subsystems::resource_management::enhanced_hybrid_resource_manager::ResourceDatabase;
use std::collections::HashMap;

#[tokio::test]
async fn test_actor_status_variants() {
    let active = ActorStatus::Active;
    let inactive = ActorStatus::Inactive;
    
    assert!(matches!(active, ActorStatus::Active));
    assert!(matches!(inactive, ActorStatus::Inactive));
}

#[tokio::test]
async fn test_actor_status_debug_clone() {
    let status = ActorStatus::Active;
    let cloned_status = status.clone();
    assert!(matches!(cloned_status, ActorStatus::Active));
    
    println!("{:?}", status); // Check Debug impl
}

#[tokio::test]
async fn test_actor_resource_document_creation() {
    let mut resources = HashMap::new();
    resources.insert("health".to_string(), 100.0);
    resources.insert("mana".to_string(), 50.0);
    
    let document = ActorResourceDocument {
        actor_id: "test_actor".to_string(),
        resources: resources.clone(),
        last_updated: 1234567890,
        status: ActorStatus::Active,
        version: 1,
    };
    
    assert_eq!(document.actor_id, "test_actor");
    assert_eq!(document.resources.get("health"), Some(&100.0));
    assert_eq!(document.resources.get("mana"), Some(&50.0));
    assert_eq!(document.last_updated, 1234567890);
    assert!(matches!(document.status, ActorStatus::Active));
    assert_eq!(document.version, 1);
}

#[tokio::test]
async fn test_actor_resource_document_debug_clone() {
    let document = ActorResourceDocument {
        actor_id: "test_actor".to_string(),
        resources: HashMap::new(),
        last_updated: 1234567890,
        status: ActorStatus::Inactive,
        version: 2,
    };
    
    let cloned_document = document.clone();
    assert_eq!(document.actor_id, cloned_document.actor_id);
    assert_eq!(document.resources, cloned_document.resources);
    assert_eq!(document.last_updated, cloned_document.last_updated);
    assert!(matches!(cloned_document.status, ActorStatus::Inactive));
    assert_eq!(document.version, cloned_document.version);
    
    println!("{:?}", document); // Check Debug impl
}

#[tokio::test]
async fn test_in_memory_resource_database_new() {
    let _db = InMemoryResourceDatabase::new();
    assert!(true); // Basic creation test
}

#[tokio::test]
async fn test_in_memory_resource_database_debug() {
    let db = InMemoryResourceDatabase::new();
    println!("{:?}", db); // Check Debug impl
    assert!(true);
}

#[tokio::test]
async fn test_in_memory_resource_database_store_actor_resources() {
    let db = InMemoryResourceDatabase::new();
    let mut resources = HashMap::new();
    resources.insert("health".to_string(), 100.0);
    resources.insert("mana".to_string(), 50.0);
    
    let result = db.store_actor_resources("test_actor", &resources).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_in_memory_resource_database_load_actor_resources_empty() {
    let db = InMemoryResourceDatabase::new();
    let resources = db.load_actor_resources("nonexistent_actor").await;
    assert!(resources.is_ok());
    let resources = resources.unwrap();
    assert!(resources.is_empty());
}

#[tokio::test]
async fn test_in_memory_resource_database_load_actor_resources_existing() {
    let db = InMemoryResourceDatabase::new();
    let mut resources = HashMap::new();
    resources.insert("health".to_string(), 100.0);
    resources.insert("mana".to_string(), 50.0);
    
    db.store_actor_resources("test_actor", &resources).await.unwrap();
    let loaded_resources = db.load_actor_resources("test_actor").await.unwrap();
    
    assert_eq!(loaded_resources.get("health"), Some(&100.0));
    assert_eq!(loaded_resources.get("mana"), Some(&50.0));
}

#[tokio::test]
async fn test_in_memory_resource_database_is_actor_active_new() {
    let db = InMemoryResourceDatabase::new();
    let is_active = db.is_actor_active("new_actor").await;
    assert!(is_active.is_ok());
    assert!(is_active.unwrap()); // New actors are considered active
}

#[tokio::test]
async fn test_in_memory_resource_database_is_actor_active_existing() {
    let db = InMemoryResourceDatabase::new();
    let mut resources = HashMap::new();
    resources.insert("health".to_string(), 100.0);
    
    db.store_actor_resources("test_actor", &resources).await.unwrap();
    let is_active = db.is_actor_active("test_actor").await;
    assert!(is_active.is_ok());
    assert!(is_active.unwrap()); // Stored actors are active by default
}

#[tokio::test]
async fn test_in_memory_resource_database_mark_actor_inactive() {
    let db = InMemoryResourceDatabase::new();
    let mut resources = HashMap::new();
    resources.insert("health".to_string(), 100.0);
    
    db.store_actor_resources("test_actor", &resources).await.unwrap();
    let result = db.mark_actor_inactive("test_actor").await;
    assert!(result.is_ok());
    
    let is_active = db.is_actor_active("test_actor").await.unwrap();
    assert!(!is_active);
}

#[tokio::test]
async fn test_in_memory_resource_database_mark_actor_active() {
    let db = InMemoryResourceDatabase::new();
    let mut resources = HashMap::new();
    resources.insert("health".to_string(), 100.0);
    
    db.store_actor_resources("test_actor", &resources).await.unwrap();
    db.mark_actor_inactive("test_actor").await.unwrap();
    let result = db.mark_actor_active("test_actor").await;
    assert!(result.is_ok());
    
    let is_active = db.is_actor_active("test_actor").await.unwrap();
    assert!(is_active);
}

#[tokio::test]
async fn test_in_memory_resource_database_mark_nonexistent_actor_inactive() {
    let db = InMemoryResourceDatabase::new();
    let result = db.mark_actor_inactive("nonexistent_actor").await;
    assert!(result.is_ok());
    
    let is_active = db.is_actor_active("nonexistent_actor").await.unwrap();
    assert!(!is_active);
}

#[tokio::test]
async fn test_in_memory_resource_database_mark_nonexistent_actor_active() {
    let db = InMemoryResourceDatabase::new();
    let result = db.mark_actor_active("nonexistent_actor").await;
    assert!(result.is_ok());
    
    let is_active = db.is_actor_active("nonexistent_actor").await.unwrap();
    assert!(is_active);
}
