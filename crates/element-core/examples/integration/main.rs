//! # Element-Core Integration Examples
//! 
//! This is the main entry point for running integration examples.

use std::error::Error;

mod simple_integration;
use simple_integration::demonstrate_simple_integration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Element-Core Integration Examples");
    println!("=====================================\n");
    
    // Run simple integration example
    println!("1️⃣  Running Simple Integration Example...");
    demonstrate_simple_integration().await?;
    println!("\n{}\n", "=".repeat(80));
    
    println!("🎉 All integration examples completed successfully!");
    println!("\n📋 Summary of Integration Examples:");
    println!("   ✅ Simple Integration: Basic Element-Core usage and API demonstration");
    
    println!("\n🏗️  Architecture Benefits Demonstrated:");
    println!("   • Data Hub Pattern: Element-Core aggregates data from multiple sources");
    println!("   • Loose Coupling: Systems can be developed independently");
    println!("   • External Contributors: Easy integration of new systems");
    println!("   • Thread Safety: Concurrent access to elemental data");
    println!("   • Performance: Array-based structures for high-frequency access");
    println!("   • Validation: Comprehensive data integrity checking");
    println!("   • Extensibility: Easy to add new elements and interaction types");
    
    Ok(())
}
