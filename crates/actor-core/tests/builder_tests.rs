//! Builder pattern tests

use actor_core::builder::*;
use actor_core::runtime_registry::*;
use std::path::PathBuf;

#[tokio::test]
async fn test_actor_core_builder() -> Result<(), Box<dyn std::error::Error>> {
    let actor_core = ActorCoreBuilder::new()
        .with_config_path(PathBuf::from("configs/actor_core_defaults.yaml"))
        .with_hot_reload(true)
        .with_metrics(true)
        .with_caching(true)
        .with_cache_size(200)
        .with_log_level("debug".to_string())
        .build()
        .await?;
    
    // Test system health
    let health = actor_core.get_health_status().await;
    assert!(health.config_health.registry_health);
    assert!(health.registry_health.resource_count > 0);
    assert!(health.enable_hot_reload);
    assert!(health.enable_metrics);
    assert!(health.enable_caching);
    assert_eq!(health.cache_size_mb, 200);
    assert_eq!(health.log_level, "debug");
    
    // Test configuration access
    let config_manager = actor_core.get_config_manager();
    let value = config_manager.get_config("defaults", "resources").await?;
    assert!(value.is_some());
    
    // Test registry access
    let registry_manager = actor_core.get_registry_manager();
    let resources = registry_manager.get_resource_registry().get_all_resources().await?;
    assert!(!resources.is_empty());
    
    // Test shutdown
    actor_core.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_configuration_hub_builder() -> Result<(), Box<dyn std::error::Error>> {
    let config_hub = ConfigurationHubBuilder::new()
        .with_config_path(PathBuf::from("configs/actor_core_defaults.yaml"))
        .with_hot_reload(true)
        .with_metrics(true)
        .with_caching(true)
        .with_cache_size(150)
        .with_log_level("info".to_string())
        .build()
        .await?;
    
    // Test system health
    let health = config_hub.get_health_status().await;
    assert!(health.config_health.registry_health);
    assert!(health.config_health.total_providers > 0);
    assert!(health.enable_hot_reload);
    assert!(health.enable_metrics);
    assert!(health.enable_caching);
    assert_eq!(health.cache_size_mb, 150);
    assert_eq!(health.log_level, "info");
    
    // Test configuration access
    let config_manager = config_hub.get_config_manager();
    let value = config_manager.get_config("defaults", "resources").await?;
    assert!(value.is_some());
    
    // Test shutdown
    config_hub.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_registry_builder() -> Result<(), Box<dyn std::error::Error>> {
    // Create custom resource definitions
    let custom_resources = vec![
        ResourceDefinition {
            id: "energy".to_string(),
            name: "Energy".to_string(),
            description: Some("Character energy points".to_string()),
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
        },
        ResourceDefinition {
            id: "focus".to_string(),
            name: "Focus".to_string(),
            description: Some("Character focus points".to_string()),
            category: "mental".to_string(),
            resource_type: ResourceType::Custom("focus".to_string()),
            base_value: 50.0,
            min_value: 0.0,
            max_value: 500.0,
            regen_rate: 0.8,
            regen_type: RegenType::Passive,
            dependencies: vec![],
            tags: vec!["mental".to_string(), "focus".to_string()],
            subsystem_id: "test_subsystem".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ];
    
    // Create custom category definitions
    let custom_categories = vec![
        CategoryDefinition {
            id: "mental".to_string(),
            name: "Mental".to_string(),
            description: Some("Mental resources and stats".to_string()),
            parent_category: None,
            tags: vec!["mental".to_string(), "cognitive".to_string()],
            subsystem_id: "test_subsystem".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        CategoryDefinition {
            id: "spiritual".to_string(),
            name: "Spiritual".to_string(),
            description: Some("Spiritual resources and stats".to_string()),
            parent_category: None,
            tags: vec!["spiritual".to_string(), "divine".to_string()],
            subsystem_id: "test_subsystem".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ];
    
    // Create custom tag definitions
    let custom_tags = vec![
        TagDefinition {
            id: "mental".to_string(),
            name: "Mental".to_string(),
            description: Some("Mental-related resources and stats".to_string()),
            tag_type: TagType::Custom("mental".to_string()),
            subsystem_id: "test_subsystem".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        TagDefinition {
            id: "spiritual".to_string(),
            name: "Spiritual".to_string(),
            description: Some("Spiritual-related resources and stats".to_string()),
            tag_type: TagType::Custom("spiritual".to_string()),
            subsystem_id: "test_subsystem".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ];
    
    let registry_system = RegistryBuilder::new()
        .with_resource(custom_resources[0].clone())
        .with_resource(custom_resources[1].clone())
        .with_category(custom_categories[0].clone())
        .with_category(custom_categories[1].clone())
        .with_tag(custom_tags[0].clone())
        .with_tag(custom_tags[1].clone())
        .with_metrics(true)
        .with_caching(true)
        .with_cache_size(75)
        .with_log_level("debug".to_string())
        .build()
        .await?;
    
    // Test system health
    let health = registry_system.get_health_status().await;
    assert!(health.registry_health.resource_count >= 2);
    assert!(health.registry_health.category_count >= 2);
    assert!(health.registry_health.tag_count >= 2);
    assert!(health.enable_metrics);
    assert!(health.enable_caching);
    assert_eq!(health.cache_size_mb, 75);
    assert_eq!(health.log_level, "debug");
    
    // Test registry access
    let resources = registry_system.get_resource_registry().get_all_resources().await?;
    assert!(resources.len() >= 2);
    
    let categories = registry_system.get_category_registry().get_all_categories().await?;
    assert!(categories.len() >= 2);
    
    let tags = registry_system.get_tag_registry().get_all_tags().await?;
    assert!(tags.len() >= 2);
    
    // Test specific resource lookup
    let energy_resource = registry_system.get_resource_registry().get_resource("energy").await?;
    assert!(energy_resource.is_some());
    assert_eq!(energy_resource.unwrap().name, "Energy");
    
    // Test specific category lookup
    let mental_category = registry_system.get_category_registry().get_category("mental").await?;
    assert!(mental_category.is_some());
    assert_eq!(mental_category.unwrap().name, "Mental");
    
    // Test specific tag lookup
    let mental_tag = registry_system.get_tag_registry().get_tag("mental").await?;
    assert!(mental_tag.is_some());
    assert_eq!(mental_tag.unwrap().name, "Mental");
    
    // Test shutdown
    registry_system.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_builder_fluent_api() -> Result<(), Box<dyn std::error::Error>> {
    // Test that the builder pattern allows chaining methods
    let actor_core = ActorCoreBuilder::new()
        .with_config_path(PathBuf::from("configs/actor_core_defaults.yaml"))
        .with_hot_reload(true)
        .with_metrics(false)
        .with_caching(true)
        .with_cache_size(100)
        .with_log_level("warn".to_string())
        .build()
        .await?;
    
    let health = actor_core.get_health_status().await;
    assert!(health.enable_hot_reload);
    assert!(!health.enable_metrics);
    assert!(health.enable_caching);
    assert_eq!(health.cache_size_mb, 100);
    assert_eq!(health.log_level, "warn");
    
    actor_core.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_builder_default_values() -> Result<(), Box<dyn std::error::Error>> {
    // Test that the builder uses sensible defaults
    let actor_core = ActorCoreBuilder::new().build().await?;
    
    let health = actor_core.get_health_status().await;
    assert!(!health.enable_hot_reload);
    assert!(health.enable_metrics);
    assert!(health.enable_caching);
    assert_eq!(health.cache_size_mb, 100);
    assert_eq!(health.log_level, "info");
    
    actor_core.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_builder_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test that the builder handles errors gracefully
    let result = ActorCoreBuilder::new()
        .with_config_path(PathBuf::from("nonexistent_config.yaml"))
        .build()
        .await;
    
    // This should fail because the config file doesn't exist
    assert!(result.is_err());
    
    Ok(())
}
