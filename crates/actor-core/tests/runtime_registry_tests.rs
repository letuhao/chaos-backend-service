//! Runtime registry system tests

use actor_core::runtime_registry::*;
use std::sync::Arc;

#[tokio::test]
async fn test_resource_registry() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Arc::new(ResourceRegistryImpl::new());
    
    // Test registering a resource
    let resource = ResourceDefinition {
        id: "test_health".to_string(),
        name: "Test Health".to_string(),
        description: Some("Test health resource".to_string()),
        category: "vital".to_string(),
        resource_type: ResourceType::Health,
        base_value: 100.0,
        min_value: 0.0,
        max_value: 1000.0,
        regen_rate: 1.0,
        regen_type: RegenType::Passive,
        dependencies: vec![],
        tags: vec!["vital".to_string(), "health".to_string()],
        subsystem_id: "test_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    registry.register_resource(resource).await?;
    
    // Test getting resource
    let retrieved_resource = registry.get_resource("test_health").await?;
    assert!(retrieved_resource.is_some());
    assert_eq!(retrieved_resource.unwrap().name, "Test Health");
    
    // Test checking if resource exists
    assert!(registry.has_resource("test_health").await?);
    assert!(!registry.has_resource("nonexistent").await?);
    
    // Test getting resources by category
    let vital_resources = registry.get_resources_by_category("vital").await?;
    assert_eq!(vital_resources.len(), 1);
    
    // Test getting resources by subsystem
    let subsystem_resources = registry.get_resources_by_subsystem("test_subsystem").await?;
    assert_eq!(subsystem_resources.len(), 1);
    
    // Test unregistering resource
    registry.unregister_resource("test_health").await?;
    assert!(!registry.has_resource("test_health").await?);
    
    Ok(())
}

#[tokio::test]
async fn test_category_registry() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Arc::new(CategoryRegistryImpl::new());
    
    // Test registering a category
    let category = CategoryDefinition {
        id: "test_category".to_string(),
        name: "Test Category".to_string(),
        description: Some("Test category description".to_string()),
        parent_category: None,
        tags: vec!["test".to_string(), "category".to_string()],
        subsystem_id: "test_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    registry.register_category(category).await?;
    
    // Test getting category
    let retrieved_category = registry.get_category("test_category").await?;
    assert!(retrieved_category.is_some());
    assert_eq!(retrieved_category.unwrap().name, "Test Category");
    
    // Test checking if category exists
    assert!(registry.has_category("test_category").await?);
    assert!(!registry.has_category("nonexistent").await?);
    
    // Test getting category tags
    let tags = registry.get_category_tags("test_category").await?;
    assert_eq!(tags.len(), 2);
    assert!(tags.contains(&"test".to_string()));
    assert!(tags.contains(&"category".to_string()));
    
    // Test getting categories by subsystem
    let subsystem_categories = registry.get_categories_by_subsystem("test_subsystem").await?;
    assert_eq!(subsystem_categories.len(), 1);
    
    // Test unregistering category
    registry.unregister_category("test_category").await?;
    assert!(!registry.has_category("test_category").await?);
    
    Ok(())
}

#[tokio::test]
async fn test_tag_registry() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Arc::new(TagRegistryImpl::new());
    
    // Test registering a tag
    let tag = TagDefinition {
        id: "test_tag".to_string(),
        name: "Test Tag".to_string(),
        description: Some("Test tag description".to_string()),
        tag_type: TagType::Resource,
        subsystem_id: "test_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    registry.register_tag(tag).await?;
    
    // Test getting tag
    let retrieved_tag = registry.get_tag("test_tag").await?;
    assert!(retrieved_tag.is_some());
    assert_eq!(retrieved_tag.unwrap().name, "Test Tag");
    
    // Test checking if tag exists
    assert!(registry.has_tag("test_tag").await?);
    assert!(!registry.has_tag("nonexistent").await?);
    
    // Test getting tags by type
    let resource_tags = registry.get_tags_by_type(&TagType::Resource).await?;
    assert_eq!(resource_tags.len(), 1);
    
    // Test getting tags by subsystem
    let subsystem_tags = registry.get_tags_by_subsystem("test_subsystem").await?;
    assert_eq!(subsystem_tags.len(), 1);
    
    // Test unregistering tag
    registry.unregister_tag("test_tag").await?;
    assert!(!registry.has_tag("test_tag").await?);
    
    Ok(())
}

#[tokio::test]
async fn test_registry_manager() -> Result<(), Box<dyn std::error::Error>> {
    let resource_registry = Arc::new(ResourceRegistryImpl::new());
    let category_registry = Arc::new(CategoryRegistryImpl::new());
    let tag_registry = Arc::new(TagRegistryImpl::new());
    
    let manager = RegistryManager::new(
        resource_registry.clone(),
        category_registry.clone(),
        tag_registry.clone(),
    );
    
    // Test initialization
    manager.initialize().await?;
    
    // Test getting all resources
    let resources = resource_registry.get_all_resources().await?;
    assert!(!resources.is_empty());
    
    // Test getting all categories
    let categories = category_registry.get_all_categories().await?;
    assert!(!categories.is_empty());
    
    // Test getting all tags
    let tags = tag_registry.get_all_tags().await?;
    assert!(!tags.is_empty());
    
    // Test health status
    let health = manager.get_health_status().await;
    assert!(health.resource_count > 0);
    assert!(health.category_count > 0);
    assert!(health.tag_count > 0);
    assert!(health.total_definitions > 0);
    
    Ok(())
}

#[tokio::test]
async fn test_registry_hierarchy() -> Result<(), Box<dyn std::error::Error>> {
    let category_registry = Arc::new(CategoryRegistryImpl::new());
    
    // Register parent category
    let parent_category = CategoryDefinition {
        id: "parent_category".to_string(),
        name: "Parent Category".to_string(),
        description: Some("Parent category description".to_string()),
        parent_category: None,
        tags: vec!["parent".to_string()],
        subsystem_id: "test_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    category_registry.register_category(parent_category).await?;
    
    // Register child category
    let child_category = CategoryDefinition {
        id: "child_category".to_string(),
        name: "Child Category".to_string(),
        description: Some("Child category description".to_string()),
        parent_category: Some("parent_category".to_string()),
        tags: vec!["child".to_string()],
        subsystem_id: "test_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    category_registry.register_category(child_category).await?;
    
    // Test getting child categories
    let child_categories = category_registry.get_child_categories("parent_category").await?;
    assert_eq!(child_categories.len(), 1);
    assert_eq!(child_categories[0].id, "child_category");
    
    Ok(())
}

#[tokio::test]
async fn test_registry_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let resource_registry = Arc::new(ResourceRegistryImpl::new());
    
    // Register some resources
    for i in 0..5 {
        let resource = ResourceDefinition {
            id: format!("resource_{}", i),
            name: format!("Resource {}", i),
            description: Some(format!("Resource {} description", i)),
            category: "test".to_string(),
            resource_type: ResourceType::Custom(format!("type_{}", i)),
            base_value: 100.0,
            min_value: 0.0,
            max_value: 1000.0,
            regen_rate: 1.0,
            regen_type: RegenType::Passive,
            dependencies: vec![],
            tags: vec![format!("tag_{}", i)],
            subsystem_id: "test_subsystem".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        resource_registry.register_resource(resource).await?;
    }
    
    // Test metrics
    let metrics = resource_registry.get_metrics().await;
    assert_eq!(metrics.registered_count, 5);
    assert_eq!(metrics.registration_attempts, 5);
    assert_eq!(metrics.unregistration_attempts, 0);
    
    // Unregister some resources
    resource_registry.unregister_resource("resource_0").await?;
    resource_registry.unregister_resource("resource_1").await?;
    
    let metrics = resource_registry.get_metrics().await;
    assert_eq!(metrics.registered_count, 3);
    assert_eq!(metrics.registration_attempts, 5);
    assert_eq!(metrics.unregistration_attempts, 2);
    
    Ok(())
}
