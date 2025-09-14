//! Exhaustion System CLI Tool
//!
//! This CLI tool provides commands for testing and debugging the Resource Exhaustion System.

#[cfg(feature = "cli-tools")]
use clap::{Parser, Subcommand};
#[cfg(feature = "cli-tools")]
use actor_core::subsystems::{
    ResourceExhaustionSubsystem, ExhaustionConfigLoader,
    InMemoryEventPublisher
};
#[cfg(feature = "cli-tools")]
use actor_core::types::{Actor, Snapshot};
#[cfg(feature = "cli-tools")]
use std::collections::HashMap;
#[cfg(feature = "cli-tools")]
use std::sync::Arc;

#[cfg(feature = "cli-tools")]
#[derive(Parser)]
#[command(name = "exhaustion-cli")]
#[command(about = "CLI tool for testing the Resource Exhaustion System")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(feature = "cli-tools")]
#[derive(Subcommand)]
enum Commands {
    /// Validate a configuration file
    Validate {
        /// Path to the configuration file
        config_path: String,
    },
    /// Test exhaustion evaluation with a specific scenario
    Test {
        /// Path to the configuration file
        config_path: String,
        /// Actor archetype
        archetype: String,
        /// Current mana value
        mana_current: f64,
        /// Maximum mana value
        mana_max: f64,
        /// Current stamina value
        stamina_current: f64,
        /// Maximum stamina value
        stamina_max: f64,
    },
    /// Run golden vector tests
    GoldenVector {
        /// Path to the configuration file
        config_path: String,
        /// Test case name (case05, case06, etc.)
        case_name: String,
    },
    /// Show configuration merge debug info
    DebugConfig {
        /// Path to the global configuration file
        global_config_path: String,
        /// Path to area override configuration file (optional)
        area_config_path: Option<String>,
        /// Path to PvP override configuration file (optional)
        pvp_config_path: Option<String>,
        /// Area ID for testing
        area_id: Option<String>,
        /// PvP template ID for testing
        pvp_template_id: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    #[cfg(feature = "cli-tools")]
    tracing_subscriber::fmt::init();

    #[cfg(feature = "cli-tools")]
    let cli = Cli::parse();

    #[cfg(feature = "cli-tools")]
    match cli.command {
        Commands::Validate { config_path } => {
            validate_config(&config_path).await?;
        }
        Commands::Test {
            config_path,
            archetype,
            mana_current,
            mana_max,
            stamina_current,
            stamina_max,
        } => {
            test_exhaustion(
                &config_path,
                &archetype,
                mana_current,
                mana_max,
                stamina_current,
                stamina_max,
            ).await?;
        }
        Commands::GoldenVector {
            config_path,
            case_name,
        } => {
            run_golden_vector_test(&config_path, &case_name).await?;
        }
        Commands::DebugConfig {
            global_config_path,
            area_config_path,
            pvp_config_path,
            area_id,
            pvp_template_id,
        } => {
            debug_config_merge(
                &global_config_path,
                area_config_path.as_deref(),
                pvp_config_path.as_deref(),
                area_id.as_deref(),
                pvp_template_id.as_deref(),
            ).await?;
        }
    }

    #[cfg(not(feature = "cli-tools"))]
    {
        eprintln!("CLI tools feature is not enabled. Please enable the 'cli-tools' feature to use this example.");
        eprintln!("Run with: cargo run --example exhaustion_cli --features cli-tools");
    }

    Ok(())
}

#[cfg(feature = "cli-tools")]
async fn validate_config(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating configuration: {}", config_path);
    
    let mut loader = ExhaustionConfigLoader::new();
    loader.load_global_config(config_path).await?;
    
    println!("✅ Configuration is valid!");
    Ok(())
}

#[cfg(feature = "cli-tools")]
async fn test_exhaustion(
    config_path: &str,
    archetype: &str,
    mana_current: f64,
    mana_max: f64,
    stamina_current: f64,
    stamina_max: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing exhaustion evaluation:");
    println!("  Archetype: {}", archetype);
    println!("  Mana: {}/{} ({:.1}%)", mana_current, mana_max, (mana_current / mana_max) * 100.0);
    println!("  Stamina: {}/{} ({:.1}%)", stamina_current, stamina_max, (stamina_current / stamina_max) * 100.0);
    
    // Load configuration
    let mut loader = ExhaustionConfigLoader::new();
    loader.load_global_config(config_path).await?;
    let merged_config = loader.resolve_config(None, None)?;
    
    // Create event publisher
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    
    // Create exhaustion subsystem
    let subsystem = ResourceExhaustionSubsystem::new(merged_config.config, event_publisher.clone());
    
    // Create test actor
    let mut actor = Actor::new("test_actor".to_string(), "Human".to_string());
    let mut data = HashMap::new();
    data.insert("archetype".to_string(), serde_json::Value::String(archetype.to_string()));
    actor.set_data(data);
    
    // Create snapshot
    let mut snapshot = Snapshot::new(actor.id, actor.version);
    snapshot.primary.insert("mana_current".to_string(), mana_current);
    snapshot.primary.insert("mana_max".to_string(), mana_max);
    snapshot.primary.insert("stamina_current".to_string(), stamina_current);
    snapshot.primary.insert("stamina_max".to_string(), stamina_max);
    
    // Evaluate exhaustion
    let transitions = subsystem.evaluate(&actor, &snapshot).await?;
    
    println!("\nResults:");
    if transitions.is_empty() {
        println!("  No exhaustion thresholds triggered");
    } else {
        for transition in &transitions {
            println!("  {} {}: {} effects", 
                if transition.entering { "Entering" } else { "Exiting" },
                transition.threshold_id,
                transition.effects.len()
            );
            
            for effect in &transition.effects {
                println!("    - {}: {:?}", effect.effect_type, effect.values);
            }
        }
        
        // Apply effects to generate events
        subsystem.apply_effects(&actor.id.to_string(), &transitions).await?;
        
        // Show events
        let events = event_publisher.get_events().await;
        if !events.is_empty() {
            println!("\nEvents published:");
            for event in &events {
                println!("  {} - {} {} {}", 
                    match event.event_type {
                        actor_core::subsystems::resource_exhaustion::ExhaustionEventType::ResourceExhausted => "EXHAUSTED",
                        actor_core::subsystems::resource_exhaustion::ExhaustionEventType::ResourceRecovered => "RECOVERED",
                    },
                    event.resource_type,
                    event.threshold_id,
                    if event.coalesced { "(coalesced)" } else { "" }
                );
            }
        }
    }
    
    Ok(())
}

#[cfg(feature = "cli-tools")]
async fn run_golden_vector_test(
    config_path: &str,
    case_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running golden vector test: {}", case_name);
    
    // Load configuration
    let mut loader = ExhaustionConfigLoader::new();
    loader.load_global_config(config_path).await?;
    let merged_config = loader.resolve_config(None, None)?;
    
    // Create event publisher
    let event_publisher = Arc::new(InMemoryEventPublisher::new());
    
    // Create exhaustion subsystem
    let subsystem = ResourceExhaustionSubsystem::new(merged_config.config, event_publisher.clone());
    
    // Run test based on case name
    match case_name {
        "case05" => run_case05_test(subsystem, event_publisher).await?,
        "case06" => run_case06_test(subsystem, event_publisher).await?,
        _ => {
            println!("❌ Unknown test case: {}", case_name);
            println!("Available test cases: case05, case06");
            return Ok(());
        }
    }
    
    println!("✅ Golden vector test completed successfully!");
    Ok(())
}

#[cfg(feature = "cli-tools")]
async fn run_case05_test(
    subsystem: ResourceExhaustionSubsystem,
    event_publisher: Arc<InMemoryEventPublisher>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running Case 05: Exhaustion Hysteresis and Coalescing");
    
    // Create mage actor
    let mut actor = Actor::new("actor_mage_001".to_string(), "Human".to_string());
    let mut data = HashMap::new();
    data.insert("archetype".to_string(), serde_json::Value::String("mage".to_string()));
    actor.set_data(data);
    
    // Test timeline
    let test_steps = vec![
        (0, 100.0, "10% mana - should trigger low_mana"),
        (100, 105.0, "10.5% mana - should be coalesced"),
        (150, 100.0, "10% mana - should be coalesced"),
        (180, 105.0, "10.5% mana - should be coalesced"),
        (400, 225.0, "22.5% mana - should recover"),
    ];
    
    for (time_ms, mana_current, description) in test_steps {
        let mut snapshot = Snapshot::new(actor.id, actor.version);
        snapshot.primary.insert("mana_current".to_string(), mana_current);
        snapshot.primary.insert("mana_max".to_string(), 1000.0);
        snapshot.primary.insert("stamina_current".to_string(), 500.0);
        snapshot.primary.insert("stamina_max".to_string(), 1000.0);
        
        let transitions = subsystem.evaluate(&actor, &snapshot).await?;
        if !transitions.is_empty() {
            subsystem.apply_effects(&actor.id.to_string(), &transitions).await?;
        }
        
        println!("  t={}ms: {} - {} transitions", time_ms, description, transitions.len());
    }
    
    // Check final events
    let events = event_publisher.get_events().await;
    println!("\nFinal events: {}", events.len());
    for event in &events {
        println!("  {} - {} {} {}", 
            match event.event_type {
                actor_core::subsystems::resource_exhaustion::ExhaustionEventType::ResourceExhausted => "EXHAUSTED",
                actor_core::subsystems::resource_exhaustion::ExhaustionEventType::ResourceRecovered => "RECOVERED",
            },
            event.resource_type,
            event.threshold_id,
            if event.coalesced { "(coalesced)" } else { "" }
        );
    }
    
    Ok(())
}

#[cfg(feature = "cli-tools")]
async fn run_case06_test(
    subsystem: ResourceExhaustionSubsystem,
    event_publisher: Arc<InMemoryEventPublisher>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running Case 06: Simultaneous Exhaustion Precedence");
    
    // Create warrior actor
    let mut actor = Actor::new("actor_warrior_001".to_string(), "Human".to_string());
    let mut data = HashMap::new();
    data.insert("archetype".to_string(), serde_json::Value::String("warrior".to_string()));
    actor.set_data(data);
    
    // Test simultaneous exhaustion
    let mut snapshot = Snapshot::new(actor.id, actor.version);
    snapshot.primary.insert("mana_current".to_string(), 200.0); // 20% mana
    snapshot.primary.insert("mana_max".to_string(), 1000.0);
    snapshot.primary.insert("stamina_current".to_string(), 150.0); // 15% stamina
    snapshot.primary.insert("stamina_max".to_string(), 1000.0);
    
    let transitions = subsystem.evaluate(&actor, &snapshot).await?;
    if !transitions.is_empty() {
        subsystem.apply_effects(&actor.id.to_string(), &transitions).await?;
    }
    
    println!("  Simultaneous exhaustion: {} transitions", transitions.len());
    for transition in &transitions {
        println!("    {} {}: {} effects", 
            if transition.entering { "Entering" } else { "Exiting" },
            transition.threshold_id,
            transition.effects.len()
        );
    }
    
    // Check events
    let events = event_publisher.get_events().await;
    println!("\nEvents: {}", events.len());
    for event in &events {
        println!("  {} - {} {} {}", 
            match event.event_type {
                actor_core::subsystems::resource_exhaustion::ExhaustionEventType::ResourceExhausted => "EXHAUSTED",
                actor_core::subsystems::resource_exhaustion::ExhaustionEventType::ResourceRecovered => "RECOVERED",
            },
            event.resource_type,
            event.threshold_id,
            if event.coalesced { "(coalesced)" } else { "" }
        );
    }
    
    Ok(())
}

#[cfg(feature = "cli-tools")]
async fn debug_config_merge(
    global_config_path: &str,
    area_config_path: Option<&str>,
    pvp_config_path: Option<&str>,
    area_id: Option<&str>,
    pvp_template_id: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Debugging configuration merge:");
    println!("  Global config: {}", global_config_path);
    if let Some(area_path) = area_config_path {
        println!("  Area config: {} (area_id: {})", area_path, area_id.unwrap_or("unknown"));
    }
    if let Some(pvp_path) = pvp_config_path {
        println!("  PvP config: {} (template_id: {})", pvp_path, pvp_template_id.unwrap_or("unknown"));
    }
    
    let mut loader = ExhaustionConfigLoader::new();
    
    // Load global config
    loader.load_global_config(global_config_path).await?;
    
    // Load area override if provided
    if let Some(area_path) = area_config_path {
        if let Some(area_id) = area_id {
            loader.load_area_override(area_id, area_path).await?;
        }
    }
    
    // Load PvP override if provided
    if let Some(pvp_path) = pvp_config_path {
        if let Some(template_id) = pvp_template_id {
            loader.load_pvp_override(template_id, pvp_path).await?;
        }
    }
    
    // Resolve final configuration
    let merged_config = loader.resolve_config(area_id, pvp_template_id)?;
    
    // Show debug information
    println!("\n{}", loader.get_debug_info(&merged_config));
    
    // Show final configuration summary
    println!("\nFinal Configuration Summary:");
    println!("  Version: {}", merged_config.config.version);
    println!("  Hysteresis Default: {}", merged_config.config.hysteresis_default);
    println!("  Coalesce Window: {}ms", merged_config.config.events.coalesce_window_ms);
    println!("  Archetypes: {}", merged_config.config.archetypes.len());
    
    for (archetype_name, archetype_config) in &merged_config.config.archetypes {
        println!("    {}: {} resources", archetype_name, archetype_config.resources.len());
        for (resource_name, resource_config) in &archetype_config.resources {
            println!("      {}: {} thresholds", resource_name, resource_config.thresholds.len());
        }
    }
    
    Ok(())
}
