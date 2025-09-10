# Enhanced Hybrid Resource Manager - API Documentation

## 📚 **Table of Contents**

1. [Overview](#overview)
2. [Quick Start](#quick-start)
3. [Core Components](#core-components)
4. [Resource Managers](#resource-managers)
5. [Event System](#event-system)
6. [Performance Monitoring](#performance-monitoring)
7. [Integration Examples](#integration-examples)
8. [Best Practices](#best-practices)
9. [Troubleshooting](#troubleshooting)

## 🎯 **Overview**

The Enhanced Hybrid Resource Manager is a comprehensive, high-performance resource management system designed for MMORPGs. It provides:

- **Multi-layer caching** (L1, L2, L3)
- **Smart resource regeneration** with conditional logic
- **Comprehensive event system** with filtering and batching
- **Performance monitoring** and load testing
- **Multiple resource systems** (RPG, Magic, Jindan)
- **Database persistence** for inactive actors
- **Event-driven architecture** with real-time notifications

## 🚀 **Quick Start**

### Basic Setup

```rust
use chaos_backend_service::crates::actor_core::subsystems::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create cache configuration
    let cache_config = CacheConfig::default();
    let resource_cache = Arc::new(ResourceCache::new(cache_config));
    
    // 2. Create stat change notifier
    let notifier_config = NotifierConfig::default();
    let stat_notifier = Arc::new(StatChangeNotifier::new(notifier_config));
    
    // 3. Create event manager
    let event_config = EventConfig::default();
    let event_manager = Arc::new(ResourceEventManager::new(event_config));
    
    // 4. Create enhanced hybrid resource manager
    let resource_manager = Arc::new(EnhancedHybridResourceManager::new(
        "enhanced_hybrid".to_string(),
        100,
        resource_cache,
        stat_notifier,
        event_manager,
    ));
    
    // 5. Create an actor
    let mut actor = Actor::new("player_1".to_string(), "Human".to_string());
    
    // 6. Set actor data
    let mut data = std::collections::HashMap::new();
    data.insert("level".to_string(), serde_json::json!(10));
    data.insert("vitality".to_string(), serde_json::json!(15));
    data.insert("intelligence".to_string(), serde_json::json!(12));
    actor.set_data(data);
    
    // 7. Calculate resources
    let result = resource_manager.contribute(&actor).await?;
    
    println!("Resource calculation result: {:?}", result);
    Ok(())
}
```

## 🏗️ **Core Components**

### EnhancedHybridResourceManager

The main resource manager that coordinates all resource operations.

```rust
pub struct EnhancedHybridResourceManager {
    system_id: String,
    priority: i64,
    shared_resource_cache: Arc<RwLock<HashMap<String, SharedResource>>>,
    system_managers: HashMap<String, Box<dyn SystemResourceCalculator + Send + Sync>>,
    database: Option<Arc<dyn ResourceDatabase + Send + Sync>>,
    stat_change_notifier: Arc<dyn StatChangeNotifier + Send + Sync>,
    resource_cache: Arc<dyn ResourceCache + Send + Sync>,
}
```

#### Key Methods

```rust
impl EnhancedHybridResourceManager {
    /// Create a new enhanced hybrid resource manager
    pub fn new(
        system_id: String,
        priority: i64,
        resource_cache: Arc<dyn ResourceCache + Send + Sync>,
        stat_change_notifier: Arc<dyn StatChangeNotifier + Send + Sync>,
        event_manager: Arc<ResourceEventManager>,
    ) -> Self;
    
    /// Calculate resources for an actor
    pub async fn contribute(&self, actor: &Actor) -> ActorCoreResult<SubsystemOutput>;
    
    /// Add a system resource manager
    pub fn add_system_manager(&mut self, manager: Box<dyn SystemResourceCalculator + Send + Sync>);
    
    /// Calculate shared resources (HP, Mana, Lifespan)
    pub async fn calculate_shared_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>>;
}
```

### ResourceCache

Multi-layer caching system for optimal performance.

```rust
pub struct ResourceCache {
    l1_cache: Arc<RwLock<HashMap<String, CachedResource>>>,
    l2_cache: Option<Arc<dyn L2Cache + Send + Sync>>,
    l3_cache: Option<Arc<dyn L3Cache + Send + Sync>>,
    config: CacheConfig,
}
```

#### Key Methods

```rust
impl ResourceCache {
    /// Get resource value from cache
    pub async fn get(&self, actor_id: &str, resource_id: &str) -> ActorCoreResult<Option<f64>>;
    
    /// Set resource value in cache
    pub async fn set(&self, actor_id: &str, resource_id: &str, value: f64, metadata: ResourceMetadata) -> ActorCoreResult<()>;
    
    /// Invalidate cache for actor
    pub async fn invalidate_actor(&self, actor_id: &str) -> ActorCoreResult<()>;
    
    /// Get cache statistics
    pub async fn get_stats(&self) -> ActorCoreResult<CacheStats>;
}
```

## 🎮 **Resource Managers**

### RPG Resource Manager

Handles traditional RPG resources like HP, MP, and stamina.

```rust
pub struct RpgResourceManager {
    system_id: String,
    resource_definitions: HashMap<String, RpgResourceDefinition>,
}
```

#### Example Usage

```rust
use chaos_backend_service::crates::actor_core::subsystems::RpgResourceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create RPG resource manager
    let mut rpg_manager = RpgResourceManager::new();
    
    // Create an actor
    let mut actor = Actor::new("warrior".to_string(), "Human".to_string());
    
    // Set actor stats
    let mut data = std::collections::HashMap::new();
    data.insert("level".to_string(), serde_json::json!(15));
    data.insert("vitality".to_string(), serde_json::json!(20));
    data.insert("intelligence".to_string(), serde_json::json!(10));
    data.insert("constitution".to_string(), serde_json::json!(18));
    actor.set_data(data);
    
    // Calculate RPG resources
    let resources = rpg_manager.calculate_resources(&actor).await?;
    
    println!("HP: {}", resources.get("hp_current").unwrap_or(&0.0));
    println!("MP: {}", resources.get("mp_current").unwrap_or(&0.0));
    println!("Stamina: {}", resources.get("stamina_current").unwrap_or(&0.0));
    
    Ok(())
}
```

### Magic Resource Manager

Handles magical resources like mana, arcane focus, and spell slots.

```rust
pub struct MagicResourceManager {
    system_id: String,
    resource_definitions: HashMap<String, MagicResourceDefinition>,
    spell_slot_definitions: HashMap<i32, SpellSlotDefinition>,
}
```

#### Example Usage

```rust
use chaos_backend_service::crates::actor_core::subsystems::MagicResourceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create magic resource manager
    let mut magic_manager = MagicResourceManager::new();
    
    // Create a mage actor
    let mut actor = Actor::new("mage".to_string(), "Elf".to_string());
    
    // Set actor stats
    let mut data = std::collections::HashMap::new();
    data.insert("level".to_string(), serde_json::json!(12));
    data.insert("intelligence".to_string(), serde_json::json!(18));
    data.insert("wisdom".to_string(), serde_json::json!(16));
    data.insert("charisma".to_string(), serde_json::json!(14));
    actor.set_data(data);
    
    // Calculate magic resources
    let resources = magic_manager.calculate_resources(&actor).await?;
    
    println!("Mana: {}", resources.get("mana_current").unwrap_or(&0.0));
    println!("Arcane Focus: {}", resources.get("arcane_focus_current").unwrap_or(&0.0));
    println!("Level 1 Spell Slots: {}", resources.get("spell_slots_level_1").unwrap_or(&0.0));
    
    Ok(())
}
```

## 📡 **Event System**

### ResourceEventManager

Comprehensive event system for resource changes and notifications.

```rust
pub struct ResourceEventManager {
    listeners: Arc<RwLock<HashMap<String, Vec<Box<dyn ResourceEventListener + Send + Sync>>>>>,
    event_history: Arc<RwLock<Vec<ResourceEvent>>>,
    event_filters: Arc<RwLock<HashMap<String, EventFilter>>>,
    config: EventConfig,
}
```

#### Example Usage

```rust
use chaos_backend_service::crates::actor_core::subsystems::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create event manager
    let event_config = EventConfig::default();
    let event_manager = Arc::new(ResourceEventManager::new(event_config));
    
    // Create a custom event listener
    struct CustomEventListener {
        listener_id: String,
    }
    
    #[async_trait]
    impl ResourceEventListener for CustomEventListener {
        async fn handle_event(&self, event: &ResourceEvent) -> ActorCoreResult<()> {
            println!("Event received: {:?} for actor {}", event.event_type, event.actor_id);
            Ok(())
        }
        
        fn listener_id(&self) -> &str {
            &self.listener_id
        }
        
        fn interested_event_types(&self) -> Vec<ResourceEventType> {
            vec![ResourceEventType::ResourceChanged, ResourceEventType::ResourceDepleted]
        }
        
        fn event_priority(&self) -> EventPriority {
            EventPriority::High
        }
    }
    
    // Add event listener
    let listener = Box::new(CustomEventListener {
        listener_id: "custom_listener".to_string(),
    });
    event_manager.add_listener(listener).await?;
    
    // Emit a resource changed event
    let event = event_manager.create_resource_changed_event(
        "player_1",
        "hp_current",
        100.0,
        90.0,
        "combat_system",
    );
    
    event_manager.emit_event(event).await?;
    
    Ok(())
}
```

## 📊 **Performance Monitoring**

### PerformanceMonitor

Real-time performance monitoring and testing capabilities.

```rust
pub struct PerformanceMonitor {
    metrics: Arc<RwLock<HashMap<String, PerformanceMetric>>>,
    tests: Arc<RwLock<HashMap<String, PerformanceTest>>>,
    config: PerformanceConfig,
}
```

#### Example Usage

```rust
use chaos_backend_service::crates::actor_core::subsystems::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create performance monitor
    let performance_config = PerformanceConfig::default();
    let monitor = Arc::new(PerformanceMonitor::new(performance_config));
    
    // Record a performance metric
    let metric = PerformanceMetric {
        name: "resource_calculation_time".to_string(),
        value: 150.0,
        unit: "ms".to_string(),
        timestamp: 1234567890,
        category: MetricCategory::ResourceCalculation,
    };
    
    monitor.record_metric(metric).await?;
    
    // Run a performance test
    let result = monitor.run_test("resource_calculation_test", || {
        // Simulate resource calculation
        std::thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }).await?;
    
    println!("Test result: {:?}", result);
    
    // Run a benchmark test
    let config = TestConfig {
        iterations: 100,
        warmup_iterations: 10,
        timeout_ms: 5000,
        memory_limit_bytes: Some(100 * 1024 * 1024), // 100MB
        enable_memory_profiling: true,
        enable_cpu_profiling: true,
    };
    
    let results = monitor.run_benchmark("resource_calculation_benchmark", config, || {
        // Simulate resource calculation
        std::thread::sleep(std::time::Duration::from_millis(50));
        Ok(())
    }).await?;
    
    println!("Benchmark results: {} tests completed", results.len());
    
    Ok(())
}
```

## 🔧 **Integration Examples**

### Complete MMORPG Resource System

```rust
use chaos_backend_service::crates::actor_core::subsystems::*;
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize all components
    let cache_config = CacheConfig::default();
    let resource_cache = Arc::new(ResourceCache::new(cache_config));
    
    let notifier_config = NotifierConfig::default();
    let stat_notifier = Arc::new(StatChangeNotifier::new(notifier_config));
    
    let event_config = EventConfig::default();
    let event_manager = Arc::new(ResourceEventManager::new(event_config));
    
    // 2. Create resource managers
    let mut rpg_manager = RpgResourceManager::new();
    let mut magic_manager = MagicResourceManager::new();
    let mut jindan_manager = JindanSystemResourceCalculator::new();
    
    // 3. Create enhanced hybrid resource manager
    let mut resource_manager = EnhancedHybridResourceManager::new(
        "mmorpg_resources".to_string(),
        100,
        resource_cache.clone(),
        stat_notifier.clone(),
        event_manager.clone(),
    );
    
    // 4. Add system managers
    resource_manager.add_system_manager(Box::new(rpg_manager));
    resource_manager.add_system_manager(Box::new(magic_manager));
    resource_manager.add_system_manager(Box::new(jindan_manager));
    
    // 5. Create performance monitor
    let performance_config = PerformanceConfig::default();
    let performance_monitor = Arc::new(PerformanceMonitor::new(performance_config));
    
    // 6. Create load testing suite
    let mut load_test_suite = LoadTestingSuite::new(performance_monitor);
    load_test_suite.generate_test_actors(1000)?;
    
    // 7. Run comprehensive tests
    let test_results = load_test_suite.run_comprehensive_test_suite().await?;
    
    println!("Load test results:");
    println!("Total tests: {}", test_results.total_tests);
    println!("Success rate: {:.2}%", test_results.success_rate * 100.0);
    println!("Average execution time: {:.2}ms", test_results.average_execution_time);
    
    // 8. Create and test with real actors
    let mut player = Actor::new("player_1".to_string(), "Human".to_string());
    
    let mut data = HashMap::new();
    data.insert("level".to_string(), serde_json::json!(25));
    data.insert("vitality".to_string(), serde_json::json!(30));
    data.insert("intelligence".to_string(), serde_json::json!(25));
    data.insert("wisdom".to_string(), serde_json::json!(20));
    data.insert("constitution".to_string(), serde_json::json!(35));
    data.insert("charisma".to_string(), serde_json::json!(15));
    data.insert("equipment_bonus".to_string(), serde_json::json!(10));
    player.set_data(data);
    
    // 9. Calculate resources
    let result = resource_manager.contribute(&player).await?;
    
    println!("Player resources calculated successfully!");
    println!("Result: {:?}", result);
    
    Ok(())
}
```

### Real-time Resource Monitoring

```rust
use chaos_backend_service::crates::actor_core::subsystems::*;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize components
    let cache_config = CacheConfig::default();
    let resource_cache = Arc::new(ResourceCache::new(cache_config));
    
    let notifier_config = NotifierConfig::default();
    let stat_notifier = Arc::new(StatChangeNotifier::new(notifier_config));
    
    let event_config = EventConfig::default();
    let event_manager = Arc::new(ResourceEventManager::new(event_config));
    
    let resource_manager = Arc::new(EnhancedHybridResourceManager::new(
        "monitoring_system".to_string(),
        100,
        resource_cache,
        stat_notifier,
        event_manager.clone(),
    ));
    
    // Create monitoring actor
    let mut actor = Actor::new("monitored_player".to_string(), "Human".to_string());
    
    let mut data = std::collections::HashMap::new();
    data.insert("level".to_string(), serde_json::json!(20));
    data.insert("vitality".to_string(), serde_json::json!(25));
    data.insert("intelligence".to_string(), serde_json::json!(22));
    actor.set_data(data);
    
    // Start monitoring loop
    for i in 0..100 {
        // Simulate stat changes
        let mut data = actor.get_data();
        data.insert("vitality".to_string(), serde_json::json!(25 + i));
        actor.set_data(data);
        
        // Calculate resources
        let result = resource_manager.contribute(&actor).await?;
        
        // Emit events
        let event = event_manager.create_resource_changed_event(
            &actor.id.to_string(),
            "hp_current",
            100.0,
            100.0 + i as f64,
            "monitoring_system",
        );
        event_manager.emit_event(event).await?;
        
        // Get event statistics
        let stats = event_manager.get_event_stats().await?;
        println!("Event stats: {} total events", stats.total_events);
        
        // Wait before next iteration
        sleep(Duration::from_millis(100)).await;
    }
    
    Ok(())
}
```

## 🎯 **Best Practices**

### 1. Resource Calculation

```rust
// ✅ Good: Use caching for frequently accessed resources
let cached_value = resource_cache.get(&actor.id.to_string(), "hp_current").await?;
if let Some(value) = cached_value {
    return Ok(value);
}

// Calculate and cache
let calculated_value = calculate_hp(actor).await?;
resource_cache.set(&actor.id.to_string(), "hp_current", calculated_value, metadata).await?;
```

### 2. Event Handling

```rust
// ✅ Good: Use event filtering for performance
let filter = EventFilter {
    name: "high_priority_only".to_string(),
    event_types: vec![],
    actor_ids: None,
    resource_names: None,
    priorities: vec![EventPriority::High, EventPriority::Critical],
    min_value_change: Some(10.0),
    max_value_change: None,
};
event_manager.add_event_filter(filter).await?;
```

### 3. Performance Monitoring

```rust
// ✅ Good: Monitor performance in production
let performance_config = PerformanceConfig {
    enable_monitoring: true,
    collection_interval: 60, // 1 minute
    max_metrics_history: 1000,
    enable_automatic_testing: true,
    test_interval: 300, // 5 minutes
};
let monitor = PerformanceMonitor::new(performance_config);
```

### 4. Error Handling

```rust
// ✅ Good: Proper error handling
match resource_manager.contribute(&actor).await {
    Ok(result) => {
        println!("Resources calculated successfully: {:?}", result);
    }
    Err(e) => {
        eprintln!("Error calculating resources: {}", e);
        // Handle error appropriately
    }
}
```

## 🔧 **Troubleshooting**

### Common Issues

#### 1. Resource Calculation Errors

**Problem**: Resources not calculating correctly

**Solution**: Check actor data and resource definitions

```rust
// Debug actor data
let data = actor.get_data();
println!("Actor data: {:?}", data);

// Check resource definitions
let rpg_manager = RpgResourceManager::new();
let dependencies = rpg_manager.get_resource_dependencies();
println!("Resource dependencies: {:?}", dependencies);
```

#### 2. Cache Issues

**Problem**: Cache not working properly

**Solution**: Check cache configuration and invalidation

```rust
// Check cache stats
let stats = resource_cache.get_stats().await?;
println!("Cache stats: {:?}", stats);

// Invalidate cache if needed
resource_cache.invalidate_actor(&actor.id.to_string()).await?;
```

#### 3. Event System Issues

**Problem**: Events not being received

**Solution**: Check event filters and listener registration

```rust
// Check event history
let history = event_manager.get_event_history(None, None, Some(10)).await?;
println!("Recent events: {:?}", history);

// Check event statistics
let stats = event_manager.get_event_stats().await?;
println!("Event stats: {:?}", stats);
```

#### 4. Performance Issues

**Problem**: Slow resource calculation

**Solution**: Use performance monitoring to identify bottlenecks

```rust
// Run performance tests
let monitor = PerformanceMonitor::new(PerformanceConfig::default());
let result = monitor.run_test("resource_calculation", || {
    resource_manager.contribute(&actor).await
}).await?;

println!("Performance test result: {:?}", result);
```

### Debug Mode

Enable debug logging for troubleshooting:

```rust
// Set up debug logging
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

// Run with debug output
let result = resource_manager.contribute(&actor).await?;
```

## 📈 **Performance Tuning**

### 1. Cache Configuration

```rust
let cache_config = CacheConfig {
    l1_ttl: 300,        // 5 minutes
    l2_ttl: 3600,       // 1 hour
    l3_ttl: 86400,      // 24 hours
    max_l1_size: 10000, // 10,000 entries
    max_l2_size: 100000, // 100,000 entries
    warming_enabled: true,
    batch_enabled: true,
};
```

### 2. Event Configuration

```rust
let event_config = EventConfig {
    max_history_size: 10000,
    enable_batching: true,
    batch_size: 100,
    batch_timeout_ms: 100,
    enable_filtering: true,
    enable_persistence: true,
    enable_monitoring: true,
};
```

### 3. Performance Monitoring

```rust
let performance_config = PerformanceConfig {
    enable_monitoring: true,
    collection_interval: 60,
    max_metrics_history: 1000,
    enable_automatic_testing: true,
    test_interval: 300,
};
```

## 🎉 **Conclusion**

The Enhanced Hybrid Resource Manager provides a comprehensive, high-performance solution for MMORPG resource management. With its multi-layer caching, smart regeneration, event system, and performance monitoring, it can handle thousands of actors efficiently while maintaining reliability and performance.

For more examples and advanced usage, see the integration tests and performance monitoring documentation.
