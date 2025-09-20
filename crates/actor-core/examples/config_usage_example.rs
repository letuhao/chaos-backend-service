//! Example demonstrating the usage of the Actor Core configuration system.
//!
//! This example shows how to load and use configuration values
//! instead of hardcoded constants.

use actor_core::config::{ConfigManager, GlobalConfigManager};
use actor_core::constants::ConfigConstants;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Actor Core Configuration System Example");
    println!("==========================================\n");

    // Create a global configuration manager
    let global_config = GlobalConfigManager::new("configs/actor_core_config.yaml".to_string());
    
    // Initialize the configuration
    global_config.initialize().await?;
    println!("✅ Configuration loaded successfully\n");

    // Get the configuration manager
    let config_manager = global_config.get_manager();

    // Create a configuration constants loader
    let config_constants = ConfigConstants::new(config_manager.clone());

    // Load and display configuration values
    println!("📋 Configuration Values:");
    println!("========================");

    // Default values
    let defaults = config_constants.get_defaults().await?;
    println!("🔧 Defaults:");
    println!("  • Actor Lifespan: {} seconds", defaults.actor_lifespan);
    println!("  • Actor Age: {} seconds", defaults.actor_age);
    println!("  • Subsystem Priority: {}", defaults.subsystem_priority);
    println!("  • Contribution Priority: {}", defaults.contribution_priority);
    println!("  • Cap Priority: {}", defaults.cap_priority);
    println!("  • Cache TTL: {} seconds", defaults.cache_ttl);
    println!("  • Batch Size: {}", defaults.batch_size);
    println!("  • Max Retries: {}", defaults.max_retries);

    // Timeouts
    let timeouts = config_constants.get_timeouts().await?;
    println!("\n⏱️  Timeouts:");
    println!("  • Aggregation Timeout: {} ms", timeouts.aggregation_timeout);
    println!("  • Cache Timeout: {} ms", timeouts.cache_timeout);
    println!("  • Database Timeout: {} ms", timeouts.database_timeout);
    println!("  • Network Timeout: {} ms", timeouts.network_timeout);
    println!("  • Subsystem Timeout: {} ms", timeouts.subsystem_timeout);
    println!("  • Batch Interval: {} ms", timeouts.batch_interval);
    println!("  • Cache Cleanup Interval: {} ms", timeouts.cache_cleanup_interval);

    // Performance thresholds
    let performance = config_constants.get_performance_thresholds().await?;
    println!("\n📊 Performance Thresholds:");
    println!("  • Max Aggregation Time: {} μs", performance.max_aggregation_time);
    println!("  • Max Cache Time: {} μs", performance.max_cache_time);
    println!("  • Max Subsystem Time: {} μs", performance.max_subsystem_time);
    println!("  • Max Memory Per Actor: {} bytes", performance.max_memory_per_actor);
    println!("  • Max Cache Size: {} bytes", performance.max_cache_size);

    // Validation rules
    let validation = config_constants.get_validation_rules().await?;
    println!("\n✅ Validation Rules:");
    println!("  • Min Actor Name Length: {}", validation.min_actor_name_length);
    println!("  • Max Actor Name Length: {}", validation.max_actor_name_length);
    println!("  • Min Dimension Name Length: {}", validation.min_dimension_name_length);
    println!("  • Max Dimension Name Length: {}", validation.max_dimension_name_length);
    println!("  • Min System ID Length: {}", validation.min_system_id_length);
    println!("  • Max System ID Length: {}", validation.max_system_id_length);
    println!("  • Max Subsystems Per Actor: {}", validation.max_subsystems_per_actor);
    println!("  • Max Contributions Per Subsystem: {}", validation.max_contributions_per_subsystem);

    // Cache keys
    let cache_keys = config_constants.get_cache_keys().await?;
    println!("\n🗄️  Cache Keys:");
    println!("  • Actor Snapshot Prefix: \"{}\"", cache_keys.actor_snapshot_prefix);
    println!("  • Subsystem Output Prefix: \"{}\"", cache_keys.subsystem_output_prefix);
    println!("  • Effective Caps Prefix: \"{}\"", cache_keys.effective_caps_prefix);
    println!("  • Registry Prefix: \"{}\"", cache_keys.registry_prefix);
    println!("  • Config Prefix: \"{}\"", cache_keys.config_prefix);

    // Log levels
    let log_levels = config_constants.get_log_levels().await?;
    println!("\n📝 Log Levels:");
    println!("  • Trace: \"{}\"", log_levels.trace);
    println!("  • Debug: \"{}\"", log_levels.debug);
    println!("  • Info: \"{}\"", log_levels.info);
    println!("  • Warn: \"{}\"", log_levels.warn);
    println!("  • Error: \"{}\"", log_levels.error);

    // Cache policies
    let cache_policies = config_constants.get_cache_policies().await?;
    println!("\n🔄 Cache Policies:");
    println!("  • LRU: \"{}\"", cache_policies.lru);
    println!("  • LFU: \"{}\"", cache_policies.lfu);
    println!("  • TTL: \"{}\"", cache_policies.ttl);
    println!("  • FIFO: \"{}\"", cache_policies.fifo);

    // System IDs
    let system_ids = config_constants.get_system_ids().await?;
    println!("\n🎮 System IDs ({} total):", system_ids.len());
    for (i, system_id) in system_ids.iter().enumerate() {
        println!("  {}. {}", i + 1, system_id);
    }

    // Context types
    let context_types = config_constants.get_context_types().await?;
    println!("\n🌍 Context Types ({} total):", context_types.len());
    for (i, context_type) in context_types.iter().enumerate() {
        println!("  {}. {}", i + 1, context_type);
    }

    // Demonstrate configuration reloading
    println!("\n🔄 Configuration Management:");
    println!("==========================");
    
    let is_loaded = config_manager.is_loaded().await;
    println!("  • Configuration Loaded: {}", is_loaded);
    
    // Reload configuration
    config_manager.reload_config().await?;
    println!("  • Configuration Reloaded: ✅");

    // Demonstrate runtime configuration update
    println!("\n🔧 Runtime Configuration Update:");
    println!("===============================");
    
    let mut current_config = config_manager.get_config().await;
    println!("  • Current Actor Lifespan: {} seconds", current_config.defaults.actor_lifespan);
    
    // Update a configuration value
    current_config.defaults.actor_lifespan = 63072000; // 2 years
    config_manager.update_config(current_config).await?;
    
    let updated_config = config_manager.get_config().await;
    println!("  • Updated Actor Lifespan: {} seconds", updated_config.defaults.actor_lifespan);

    println!("\n✅ Configuration System Example Complete!");
    println!("\n💡 Benefits of Configuration System:");
    println!("  • ✅ No hardcoded values in code");
    println!("  • ✅ Runtime configuration loading");
    println!("  • ✅ Easy configuration updates");
    println!("  • ✅ Validation of configuration values");
    println!("  • ✅ Fallback to defaults if config fails");
    println!("  • ✅ Support for multiple configuration files");
    println!("  • ✅ Type-safe configuration access");

    Ok(())
}
