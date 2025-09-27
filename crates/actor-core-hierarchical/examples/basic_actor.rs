//! # Basic Actor Example
//!
//! This example demonstrates how to create a basic actor without elemental parameters.

use actor_core_hierarchical::ActorFactory;
use element_core::ElementalRegistry;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Creating Basic Actor Example");
    
    // 1. Create Elemental Registry (empty for now)
    let registry = ElementalRegistry::new();
    
    // 2. Create Actor Factory
    let factory = ActorFactory::new(Arc::new(registry));
    
    // 3. Create Basic Actor
    let actor = factory.create_actor("warrior")?;
    
    println!("✅ Created Basic Actor");
    println!("Actor ID: {}", actor.get_id());
    println!("Actor Name: {}", actor.get_name());
    
    // 4. Display Basic Info
    let elemental_data = actor.get_elemental_system().get_data();
    println!("Elemental System Initialized: {}", 
             actor.get_metadata("elemental_system_initialized").unwrap_or(&"false".to_string()));
    
    println!("Fire Mastery Level: {}", elemental_data.element_mastery_levels[0]);
    println!("Water Mastery Level: {}", elemental_data.element_mastery_levels[1]);
    
    println!("\n🎉 Basic Actor created successfully!");
    
    Ok(())
}
