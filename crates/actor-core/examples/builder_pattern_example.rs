//! Builder Pattern Example
//! 
//! This example demonstrates how to use the Builder pattern for complex Actor Core setup scenarios.

use actor_core::builder::*;
use actor_core::runtime_registry::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 Builder Pattern Example");
    println!("==========================");

    // Example 1: Simple Actor Core Builder
    println!("\n📋 Example 1: Simple Actor Core Builder");
    println!("=======================================");
    
    let actor_core = ActorCoreBuilder::new()
        .with_config_path(PathBuf::from("configs/actor_core_defaults.yaml"))
        .with_hot_reload(true)
        .with_metrics(true)
        .with_caching(true)
        .with_cache_size(200)
        .with_log_level("debug".to_string())
        .build()
        .await?;
    
    println!("✅ Simple Actor Core built successfully");
    
    // Get system health
    let health = actor_core.get_health_status().await;
    println!("System Health:");
    println!("  Config Registry Health: {}", health.config_health.registry_health);
    println!("  Registry Health: {}", health.registry_health.resource_count);
    println!("  Hot Reload: {}", health.enable_hot_reload);
    println!("  Metrics: {}", health.enable_metrics);
    println!("  Caching: {}", health.enable_caching);
    println!("  Cache Size: {} MB", health.cache_size_mb);
    println!("  Log Level: {}", health.log_level);
    
    // Example 2: Complex Configuration Hub Builder
    println!("\n⚙️ Example 2: Complex Configuration Hub Builder");
    println!("===============================================");
    
    let config_hub = ConfigurationHubBuilder::new()
        .with_config_path(PathBuf::from("configs/actor_core_defaults.yaml"))
        .with_config_path(PathBuf::from("configs/custom_config.yaml"))
        .with_hot_reload(true)
        .with_metrics(true)
        .with_caching(true)
        .with_cache_size(150)
        .with_log_level("info".to_string())
        .build()
        .await?;
    
    println!("✅ Complex Configuration Hub built successfully");
    
    // Get configuration hub health
    let config_health = config_hub.get_health_status().await;
    println!("Configuration Hub Health:");
    println!("  Registry Health: {}", config_health.config_health.registry_health);
    println!("  Total Providers: {}", config_health.config_health.total_providers);
    println!("  Total Merge Rules: {}", config_health.config_health.total_merge_rules);
    println!("  Hot Reload: {}", config_health.enable_hot_reload);
    println!("  Metrics: {}", config_health.enable_metrics);
    println!("  Caching: {}", config_health.enable_caching);
    println!("  Cache Size: {} MB", config_health.cache_size_mb);
    println!("  Log Level: {}", config_health.log_level);
    
    // Example 3: Complex Registry Builder
    println!("\n📊 Example 3: Complex Registry Builder");
    println!("=====================================");
    
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
            subsystem_id: "custom_subsystem".to_string(),
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
            subsystem_id: "custom_subsystem".to_string(),
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
            subsystem_id: "custom_subsystem".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        CategoryDefinition {
            id: "spiritual".to_string(),
            name: "Spiritual".to_string(),
            description: Some("Spiritual resources and stats".to_string()),
            parent_category: None,
            tags: vec!["spiritual".to_string(), "divine".to_string()],
            subsystem_id: "custom_subsystem".to_string(),
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
            subsystem_id: "custom_subsystem".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        TagDefinition {
            id: "spiritual".to_string(),
            name: "Spiritual".to_string(),
            description: Some("Spiritual-related resources and stats".to_string()),
            tag_type: TagType::Custom("spiritual".to_string()),
            subsystem_id: "custom_subsystem".to_string(),
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
    
    println!("✅ Complex Registry System built successfully");
    
    // Get registry system health
    let registry_health = registry_system.get_health_status().await;
    println!("Registry System Health:");
    println!("  Resource Count: {}", registry_health.registry_health.resource_count);
    println!("  Category Count: {}", registry_health.registry_health.category_count);
    println!("  Tag Count: {}", registry_health.registry_health.tag_count);
    println!("  Total Definitions: {}", registry_health.registry_health.total_definitions);
    println!("  Metrics: {}", registry_health.enable_metrics);
    println!("  Caching: {}", registry_health.enable_caching);
    println!("  Cache Size: {} MB", registry_health.cache_size_mb);
    println!("  Log Level: {}", registry_health.log_level);
    
    // Example 4: Demonstrate Registry Access
    println!("\n🔍 Example 4: Registry Access");
    println!("============================");
    
    // Get all resources
    let resources = registry_system.get_resource_registry().get_all_resources().await?;
    println!("All Resources:");
    for resource in resources {
        println!("  {}: {} ({} - {})", 
                 resource.id, 
                 resource.name, 
                 resource.min_value, 
                 resource.max_value);
    }
    
    // Get all categories
    let categories = registry_system.get_category_registry().get_all_categories().await?;
    println!("All Categories:");
    for category in categories {
        println!("  {}: {}", category.id, category.name);
    }
    
    // Get all tags
    let tags = registry_system.get_tag_registry().get_all_tags().await?;
    println!("All Tags:");
    for tag in tags {
        println!("  {}: {} ({:?})", tag.id, tag.name, tag.tag_type);
    }
    
    // Example 5: System Shutdown
    println!("\n🔄 Example 5: System Shutdown");
    println!("=============================");
    
    // Shutdown all systems
    actor_core.shutdown().await?;
    config_hub.shutdown().await?;
    registry_system.shutdown().await?;
    
    println!("✅ All systems shutdown successfully");
    
    println!("\n🎉 Builder Pattern example completed successfully!");
    println!("\n📋 Summary of Builder Pattern Benefits:");
    println!("  ✅ Fluent API for complex setup");
    println!("  ✅ Optional configuration parameters");
    println!("  ✅ Type-safe configuration");
    println!("  ✅ Easy to extend and modify");
    println!("  ✅ Clear separation of concerns");
    println!("  ✅ Reusable builder components");
    
    Ok(())
}
