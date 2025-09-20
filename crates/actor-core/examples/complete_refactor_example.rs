//! Complete Actor Core Refactor Example
//! 
//! This example demonstrates the complete refactored Actor Core system
//! with Configuration Hub, Runtime Registry, and dynamic configuration loading.

use actor_core::config::*;
use actor_core::config::providers::*;
use actor_core::config::loaders::*;
use actor_core::runtime_registry::*;
use std::sync::Arc;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 Complete Actor Core Refactor Example");
    println!("======================================");

    // Phase 1: Initialize Configuration Hub
    println!("\n📋 Phase 1: Configuration Hub");
    println!("=============================");
    
    // Create configuration registry
    let registry = Arc::new(ConfigurationRegistryImpl::new());
    
    // Create configuration combiner
    let combiner = Arc::new(ConfigurationCombinerImpl::new());
    combiner.load_default_rules()?;
    
    // Create configuration aggregator
    let aggregator = Arc::new(ConfigurationAggregatorImpl::new(registry.clone(), combiner.clone()));
    
    // Create configuration loader
    let mut loader = ConfigurationLoader::new(registry.clone(), combiner.clone(), aggregator.clone());
    
    // Add default configuration provider
    let default_config_path = PathBuf::from("configs/actor_core_defaults.yaml");
    let default_provider = Arc::new(DefaultConfigProvider::new(default_config_path)?);
    loader.add_provider(default_provider);
    
    // Add example provider
    let example_provider = Arc::new(ExampleConfigurationProvider::new());
    loader.add_provider(example_provider);
    
    // Add environment provider
    let mut env_provider = EnvironmentConfigurationProvider::new(
        "env_provider".to_string(),
        200,
        "ACTOR_CORE".to_string(),
    );
    env_provider.load_from_environment()?;
    loader.add_provider(Arc::new(env_provider));
    
    // Add database provider
    let mut db_provider = DatabaseConfigurationProvider::new(
        "db_provider".to_string(),
        300,
    );
    db_provider.load_from_database().await?;
    loader.add_provider(Arc::new(db_provider));
    
    // Create configuration manager
    let config_manager = ConfigurationManager::new(registry, combiner, aggregator, Arc::new(loader));
    
    // Initialize the configuration system
    config_manager.initialize().await?;
    
    println!("✅ Configuration Hub initialized");
    
    // Phase 2: Initialize Runtime Registry
    println!("\n📊 Phase 2: Runtime Registry");
    println!("============================");
    
    // Create registries
    let resource_registry = Arc::new(ResourceRegistryImpl::new());
    let category_registry = Arc::new(CategoryRegistryImpl::new());
    let tag_registry = Arc::new(TagRegistryImpl::new());
    
    // Create registry manager
    let registry_manager = RegistryManager::new(
        resource_registry.clone(),
        category_registry.clone(),
        tag_registry.clone(),
    );
    
    // Initialize the registry system
    registry_manager.initialize().await?;
    
    println!("✅ Runtime Registry initialized");
    
    // Phase 3: Demonstrate Configuration Access
    println!("\n⚙️ Phase 3: Configuration Access");
    println!("===============================");
    
    // Get default resource values
    if let Some(health_defaults) = config_manager.get_config("defaults", "resources").await? {
        println!("Health defaults: {}", health_defaults.value);
    }
    
    // Get timeout values
    if let Some(cache_ttl) = config_manager.get_config("timeouts", "cache_ttl").await? {
        println!("Cache TTL: {}", cache_ttl.value);
    }
    
    // Get performance thresholds
    if let Some(max_actors) = config_manager.get_config("performance_thresholds", "max_actors").await? {
        println!("Max actors: {}", max_actors.value);
    }
    
    // Get validation rules
    if let Some(resource_validation) = config_manager.get_config("validation_rules", "resource_values").await? {
        println!("Resource validation rules: {}", resource_validation.value);
    }
    
    // Phase 4: Demonstrate Runtime Registry Access
    println!("\n📋 Phase 4: Runtime Registry Access");
    println!("===================================");
    
    // Get all resources
    let resources = resource_registry.get_all_resources().await?;
    println!("Registered resources: {}", resources.len());
    for resource in resources {
        println!("  {}: {} ({} - {})", 
                 resource.id, 
                 resource.name, 
                 resource.min_value, 
                 resource.max_value);
    }
    
    // Get all categories
    let categories = category_registry.get_all_categories().await?;
    println!("Registered categories: {}", categories.len());
    for category in categories {
        println!("  {}: {}", category.id, category.name);
    }
    
    // Get all tags
    let tags = tag_registry.get_all_tags().await?;
    println!("Registered tags: {}", tags.len());
    for tag in tags {
        println!("  {}: {} ({:?})", tag.id, tag.name, tag.tag_type);
    }
    
    // Phase 5: Demonstrate Dynamic Registration
    println!("\n🔄 Phase 5: Dynamic Registration");
    println!("===============================");
    
    // Register a new resource dynamically
    let new_resource = ResourceDefinition {
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
        subsystem_id: "example_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    resource_registry.register_resource(new_resource).await?;
    println!("✅ Registered new resource: energy");
    
    // Register a new category dynamically
    let new_category = CategoryDefinition {
        id: "magic".to_string(),
        name: "Magic".to_string(),
        description: Some("Magic-related resources and stats".to_string()),
        parent_category: None,
        tags: vec!["magic".to_string(), "elemental".to_string()],
        subsystem_id: "example_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    category_registry.register_category(new_category).await?;
    println!("✅ Registered new category: magic");
    
    // Register a new tag dynamically
    let new_tag = TagDefinition {
        id: "elemental".to_string(),
        name: "Elemental".to_string(),
        description: Some("Elemental-related resources and stats".to_string()),
        tag_type: TagType::Element,
        subsystem_id: "example_subsystem".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    tag_registry.register_tag(new_tag).await?;
    println!("✅ Registered new tag: elemental");
    
    // Phase 6: System Health Status
    println!("\n🏥 Phase 6: System Health Status");
    println!("================================");
    
    // Configuration system health
    let config_health = config_manager.get_health_status().await;
    println!("Configuration System:");
    println!("  Registry Health: {}", config_health.registry_health);
    println!("  Combiner Health: {}", config_health.combiner_health);
    println!("  Aggregator Health: {}", config_health.aggregator_health);
    println!("  Total Providers: {}", config_health.total_providers);
    println!("  Total Merge Rules: {}", config_health.total_merge_rules);
    println!("  Total Requests: {}", config_health.total_requests);
    println!("  Cache Hit Ratio: {:.2}%", config_health.cache_hit_ratio * 100.0);
    
    // Registry system health
    let registry_health = registry_manager.get_health_status().await;
    println!("Registry System:");
    println!("  Resource Count: {}", registry_health.resource_count);
    println!("  Category Count: {}", registry_health.category_count);
    println!("  Tag Count: {}", registry_health.tag_count);
    println!("  Total Definitions: {}", registry_health.total_definitions);
    
    println!("\n✅ Complete Actor Core refactor example completed successfully!");
    println!("\n🎉 All phases completed:");
    println!("  ✅ Configuration Hub Implementation");
    println!("  ✅ Configuration Providers");
    println!("  ✅ Runtime Registry System");
    println!("  ✅ Hardcoded Data Elimination");
    println!("  ✅ Dynamic Configuration Loading");
    println!("  ✅ System Health Monitoring");
    
    Ok(())
}
