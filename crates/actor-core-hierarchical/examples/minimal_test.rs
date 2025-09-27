//! # Minimal Test Example
//!
//! This example demonstrates the most basic functionality.

use actor_core_hierarchical::ActorFactory;
use element_core::ElementalRegistry;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Minimal Test");
    
    // 1. Create empty registry
    let registry = ElementalRegistry::new();
    println!("✅ Created registry");
    
    // 2. Create factory
    let factory = ActorFactory::new(Arc::new(registry));
    println!("✅ Created factory");
    
    // 3. Create basic actor
    let actor = factory.create_actor("test")?;
    println!("✅ Created actor: {}", actor.get_name());
    
    Ok(())
}
