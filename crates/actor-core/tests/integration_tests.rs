//! Integration tests for the complete Actor Core refactor

use actor_core::builder::*;
use actor_core::config::*;
use actor_core::runtime_registry::*;
use std::path::PathBuf;

#[tokio::test]
async fn test_complete_system_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Build the complete Actor Core system
    let actor_core = ActorCoreBuilder::new()
        .with_config_path(PathBuf::from("configs/actor_core_defaults.yaml"))
        .with_hot_reload(true)
        .with_metrics(true)
        .with_caching(true)
        .with_cache_size(200)
        .with_log_level("debug".to_string())
        .build()
        .await?;
    
    // Test configuration system
    let config_manager = actor_core.get_config_manager();
    
    // Test getting default resource values
    let health_defaults = config_manager.get_config("defaults", "resources").await?;
    assert!(health_defaults.is_some());
    
    // Test getting timeout values
    let cache_ttl = config_manager.get_config("timeouts", "cache_ttl").await?;
    assert!(cache_ttl.is_some());
    assert_eq!(cache_ttl.unwrap().value.as_u64().unwrap(), 3600);
    
    // Test getting performance thresholds
    let max_actors = config_manager.get_config("performance_thresholds", "max_actors").await?;
    assert!(max_actors.is_some());
    assert_eq!(max_actors.unwrap().value.as_u64().unwrap(), 10000);
    
    // Test getting validation rules
    let resource_validation = config_manager.get_config("validation_rules", "resource_values").await?;
    assert!(resource_validation.is_some());
    
    // Test registry system
    let registry_manager = actor_core.get_registry_manager();
    
    // Test getting all resources
    let resources = registry_manager.get_resource_registry().get_all_resources().await?;
    assert!(!resources.is_empty());
    
    // Test getting all categories
    let categories = registry_manager.get_category_registry().get_all_categories().await?;
    assert!(!categories.is_empty());
    
    // Test getting all tags
    let tags = registry_manager.get_tag_registry().get_all_tags().await?;
    assert!(!tags.is_empty());
    
    // Test dynamic resource registration
    let new_resource = ResourceDefinition {
        id: "test_energy".to_string(),
        name: "Test Energy".to_string(),
        description: Some("Test energy resource".to_string()),
        category: "vital".to_string(),
        resource_type: ResourceType::Custom("energy".to_string()),
        base_value: 100.0,
        min_value: 0.0,
        max_value: 1000.0,
        regen_rate: 1.5,
        regen_type: RegenType::Passive,
        dependencies: vec![],
        tags: vec!["vital".to_string(), "energy".to_string()],
        subsystem_id: "test_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    registry_manager.get_resource_registry().register_resource(new_resource).await?;
    
    // Verify the resource was registered
    let test_energy = registry_manager.get_resource_registry().get_resource("test_energy").await?;
    assert!(test_energy.is_some());
    assert_eq!(test_energy.unwrap().name, "Test Energy");
    
    // Test system health
    let health = actor_core.get_health_status().await;
    assert!(health.config_health.registry_health);
    assert!(health.config_health.combiner_health);
    assert!(health.config_health.aggregator_health);
    assert!(health.registry_health.resource_count > 0);
    assert!(health.registry_health.category_count > 0);
    assert!(health.registry_health.tag_count > 0);
    
    // Test shutdown
    actor_core.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_configuration_provider_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test that multiple configuration providers work together
    let config_hub = ConfigurationHubBuilder::new()
        .with_config_path(PathBuf::from("configs/actor_core_defaults.yaml"))
        .build()
        .await?;
    
    let config_manager = config_hub.get_config_manager();
    
    // Test that we can get configuration from multiple sources
    let defaults = config_manager.get_config("defaults", "resources").await?;
    assert!(defaults.is_some());
    
    let timeouts = config_manager.get_config("timeouts", "cache_ttl").await?;
    assert!(timeouts.is_some());
    
    let performance = config_manager.get_config("performance_thresholds", "max_actors").await?;
    assert!(performance.is_some());
    
    // Test getting all configuration
    let all_config = config_manager.get_all_config().await?;
    assert!(!all_config.is_empty());
    assert!(all_config.contains_key("defaults"));
    assert!(all_config.contains_key("timeouts"));
    assert!(all_config.contains_key("performance_thresholds"));
    
    config_hub.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_registry_hierarchy_integration() -> Result<(), Box<dyn std::error::Error>> {
    let registry_system = RegistryBuilder::new().build().await?;
    
    // Test parent-child category relationships
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
    
    registry_system.get_category_registry().register_category(parent_category).await?;
    registry_system.get_category_registry().register_category(child_category).await?;
    
    // Test getting child categories
    let child_categories = registry_system.get_category_registry().get_child_categories("parent_category").await?;
    assert_eq!(child_categories.len(), 1);
    assert_eq!(child_categories[0].id, "child_category");
    
    // Test getting categories by subsystem
    let subsystem_categories = registry_system.get_category_registry().get_categories_by_subsystem("test_subsystem").await?;
    assert_eq!(subsystem_categories.len(), 2);
    
    registry_system.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_error_handling_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test that the system handles errors gracefully
    
    // Test invalid resource registration
    let registry_system = RegistryBuilder::new().build().await?;
    
    let invalid_resource = ResourceDefinition {
        id: "".to_string(), // Empty ID should fail
        name: "Invalid Resource".to_string(),
        description: Some("Invalid resource description".to_string()),
        category: "vital".to_string(),
        resource_type: ResourceType::Health,
        base_value: 100.0,
        min_value: 0.0,
        max_value: 1000.0,
        regen_rate: 1.0,
        regen_type: RegenType::Passive,
        dependencies: vec![],
        tags: vec![],
        subsystem_id: "test_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    let result = registry_system.get_resource_registry().register_resource(invalid_resource).await;
    assert!(result.is_err());
    
    // Test getting non-existent resource
    let result = registry_system.get_resource_registry().get_resource("nonexistent").await?;
    assert!(result.is_none());
    
    // Test unregistering non-existent resource
    let result = registry_system.get_resource_registry().unregister_resource("nonexistent").await;
    assert!(result.is_err());
    
    registry_system.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_performance_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test that the system performs well with many resources
    let mut registry_system = RegistryBuilder::new().build().await?;
    
    // Register many resources
    for i in 0..100 {
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
        
        registry_system.get_resource_registry().register_resource(resource).await?;
    }
    
    // Test that we can still get all resources quickly
    let start = std::time::Instant::now();
    let resources = registry_system.get_resource_registry().get_all_resources().await?;
    let duration = start.elapsed();
    
    assert_eq!(resources.len(), 100);
    assert!(duration.as_millis() < 100); // Should be fast
    
    // Test that we can get specific resources quickly
    let start = std::time::Instant::now();
    let resource = registry_system.get_resource_registry().get_resource("resource_50").await?;
    let duration = start.elapsed();
    
    assert!(resource.is_some());
    assert!(duration.as_millis() < 10); // Should be very fast
    
    registry_system.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_concurrent_access_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test that the system handles concurrent access correctly
    let registry_system = Arc::new(RegistryBuilder::new().build().await?);
    
    let mut handles = vec![];
    
    // Spawn multiple tasks that access the registry concurrently
    for i in 0..10 {
        let registry = registry_system.clone();
        let handle = tokio::spawn(async move {
            for j in 0..10 {
                let resource = ResourceDefinition {
                    id: format!("resource_{}_{}", i, j),
                    name: format!("Resource {}_{}", i, j),
                    description: Some(format!("Resource {}_{} description", i, j)),
                    category: "test".to_string(),
                    resource_type: ResourceType::Custom(format!("type_{}_{}", i, j)),
                    base_value: 100.0,
                    min_value: 0.0,
                    max_value: 1000.0,
                    regen_rate: 1.0,
                    regen_type: RegenType::Passive,
                    dependencies: vec![],
                    tags: vec![format!("tag_{}_{}", i, j)],
                    subsystem_id: "test_subsystem".to_string(),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };
                
                registry.get_resource_registry().register_resource(resource).await?;
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await??;
    }
    
    // Verify that all resources were registered
    let resources = registry_system.get_resource_registry().get_all_resources().await?;
    assert_eq!(resources.len(), 100);
    
    registry_system.shutdown().await?;
    
    Ok(())
}
