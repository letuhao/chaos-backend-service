//! Runtime Registry Example
//! 
//! This example demonstrates how to use the Runtime Registry system
//! to manage resources, categories, and tags dynamically.

use actor_core::runtime_registry::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 Runtime Registry Example");
    println!("===========================");

    // Create registries
    let resource_registry = Arc::new(ResourceRegistryImpl::new());
    let category_registry = Arc::new(CategoryRegistryImpl::new());
    let tag_registry = Arc::new(TagRegistryImpl::new());
    
    // Create registry manager
    let manager = RegistryManager::new(
        resource_registry.clone(),
        category_registry.clone(),
        tag_registry.clone(),
    );
    
    // Initialize the registry system
    manager.initialize().await?;
    
    println!("✅ Registry system initialized");
    
    // Display all resources
    println!("\n📊 Registered Resources:");
    println!("========================");
    let resources = resource_registry.get_all_resources().await?;
    for resource in resources {
        println!("  {}: {} ({} - {})", 
                 resource.id, 
                 resource.name, 
                 resource.category, 
                 resource.resource_type);
        println!("    Base: {}, Min: {}, Max: {}", 
                 resource.base_value, 
                 resource.min_value, 
                 resource.max_value);
        println!("    Regen: {} ({})", 
                 resource.regen_rate, 
                 resource.regen_type);
        println!("    Tags: {:?}", resource.tags);
        println!();
    }
    
    // Display all categories
    println!("\n📁 Registered Categories:");
    println!("=========================");
    let categories = category_registry.get_all_categories().await?;
    for category in categories {
        println!("  {}: {} ({})", 
                 category.id, 
                 category.name, 
                 category.description.unwrap_or_default());
        println!("    Parent: {:?}", category.parent_category);
        println!("    Tags: {:?}", category.tags);
        println!();
    }
    
    // Display all tags
    println!("\n🏷️ Registered Tags:");
    println!("===================");
    let tags = tag_registry.get_all_tags().await?;
    for tag in tags {
        println!("  {}: {} ({})", 
                 tag.id, 
                 tag.name, 
                 tag.description.unwrap_or_default());
        println!("    Type: {:?}", tag.tag_type);
        println!();
    }
    
    // Test resource lookups
    println!("\n🔍 Resource Lookups:");
    println!("===================");
    
    if let Some(health) = resource_registry.get_resource("health").await? {
        println!("Health resource found: {}", health.name);
    }
    
    if let Some(mana) = resource_registry.get_resource("mana").await? {
        println!("Mana resource found: {}", mana.name);
    }
    
    // Test category lookups
    println!("\n🔍 Category Lookups:");
    println!("===================");
    
    if let Some(vital) = category_registry.get_category("vital").await? {
        println!("Vital category found: {}", vital.name);
    }
    
    if let Some(combat) = category_registry.get_category("combat").await? {
        println!("Combat category found: {}", combat.name);
    }
    
    // Test tag lookups
    println!("\n🔍 Tag Lookups:");
    println!("===============");
    
    if let Some(vital_tag) = tag_registry.get_tag("vital").await? {
        println!("Vital tag found: {}", vital_tag.name);
    }
    
    if let Some(combat_tag) = tag_registry.get_tag("combat").await? {
        println!("Combat tag found: {}", combat_tag.name);
    }
    
    // Get system health status
    println!("\n🏥 System Health:");
    println!("=================");
    let health = manager.get_health_status().await;
    println!("  Resource Count: {}", health.resource_count);
    println!("  Category Count: {}", health.category_count);
    println!("  Tag Count: {}", health.tag_count);
    println!("  Total Definitions: {}", health.total_definitions);
    
    println!("\n✅ Runtime Registry example completed successfully!");
    
    Ok(())
}