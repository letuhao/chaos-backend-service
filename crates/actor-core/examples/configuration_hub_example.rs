//! Configuration Hub Example
//! 
//! This example demonstrates how to use the Configuration Hub system
//! to manage configurations from multiple providers with merge/override logic.

use actor_core::config::*;
use actor_core::config::providers::*;
use std::sync::Arc;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 Configuration Hub Example");
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
    
    // Add example provider
    let example_provider = Arc::new(ExampleConfigurationProvider::new());
    loader.add_provider(example_provider);
    
    // Add environment provider (with higher priority)
    let mut env_provider = EnvironmentConfigurationProvider::new(
        "env_provider".to_string(),
        200, // Higher priority than example provider
        "ACTOR_CORE".to_string(),
    );
    env_provider.load_from_environment()?;
    loader.add_provider(Arc::new(env_provider));
    
    // Add database provider (with highest priority)
    let mut db_provider = DatabaseConfigurationProvider::new(
        "db_provider".to_string(),
        300, // Highest priority
    );
    db_provider.load_from_database().await?;
    loader.add_provider(Arc::new(db_provider));
    
    // Create configuration manager
    let manager = ConfigurationManager::new(registry, combiner, aggregator, Arc::new(loader));
    
    // Initialize the configuration system
    manager.initialize().await?;
    
    println!("✅ Configuration system initialized");
    
    // Get configuration values
    println!("\n📊 Configuration Values:");
    println!("========================");
    
    // Get element affinities
    if let Some(fire_affinity) = manager.get_config("element_affinities", "fire_affinity").await? {
        println!("Fire Affinity: {}", fire_affinity.value);
    }
    
    if let Some(water_affinity) = manager.get_config("element_affinities", "water_affinity").await? {
        println!("Water Affinity: {}", water_affinity.value);
    }
    
    // Get primary stats
    if let Some(strength) = manager.get_config("primary_stats", "strength").await? {
        println!("Strength: {}", strength.value);
    }
    
    if let Some(agility) = manager.get_config("primary_stats", "agility").await? {
        println!("Agility: {}", agility.value);
    }
    
    // Get all element affinities
    println!("\n🔥 Element Affinities:");
    let element_affinities = manager.get_category_config("element_affinities").await?;
    for (key, value) in element_affinities {
        println!("  {}: {}", key, value.value);
    }
    
    // Get all primary stats
    println!("\n💪 Primary Stats:");
    let primary_stats = manager.get_category_config("primary_stats").await?;
    for (key, value) in primary_stats {
        println!("  {}: {}", key, value.value);
    }
    
    // Get database configuration
    println!("\n🗄️ Database Configuration:");
    let database_config = manager.get_category_config("database").await?;
    for (key, value) in database_config {
        println!("  {}: {}", key, value.value);
    }
    
    // Get system configuration
    println!("\n⚙️ System Configuration:");
    let system_config = manager.get_category_config("system").await?;
    for (key, value) in system_config {
        println!("  {}: {}", key, value.value);
    }
    
    // Get system health status
    println!("\n🏥 System Health:");
    let health = manager.get_health_status().await;
    println!("  Registry Health: {}", health.registry_health);
    println!("  Combiner Health: {}", health.combiner_health);
    println!("  Aggregator Health: {}", health.aggregator_health);
    println!("  Total Providers: {}", health.total_providers);
    println!("  Total Merge Rules: {}", health.total_merge_rules);
    println!("  Total Requests: {}", health.total_requests);
    println!("  Cache Hit Ratio: {:.2}%", health.cache_hit_ratio * 100.0);
    
    println!("\n✅ Configuration Hub example completed successfully!");
    
    Ok(())
}
