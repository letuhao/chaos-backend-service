# Enhanced Hybrid Resource Manager - Examples

This directory contains comprehensive examples demonstrating the usage of the Enhanced Hybrid Resource Manager system.

## 📚 **Available Examples**

### 1. Basic Usage (`01_basic_usage.rs`)
**Difficulty**: Beginner  
**Duration**: 5-10 minutes  
**Description**: Introduction to the Enhanced Hybrid Resource Manager with basic resource calculation and management.

**What you'll learn**:
- Setting up the resource manager
- Creating player characters
- Calculating resources
- Simulating stat changes
- Basic resource regeneration

**Key Features Demonstrated**:
- Resource manager initialization
- Actor creation and data setup
- Resource calculation
- Stat change simulation
- Regeneration system

### 2. Advanced Usage (`02_advanced_usage.rs`)
**Difficulty**: Intermediate  
**Duration**: 15-20 minutes  
**Description**: Advanced features including multiple resource systems, event handling, and performance monitoring.

**What you'll learn**:
- Multi-system resource management
- Event-driven architecture
- Performance monitoring
- Combat simulation
- Resource regeneration over time

**Key Features Demonstrated**:
- RPG, Magic, and Jindan resource systems
- Event listeners and filtering
- Combat scenario simulation
- Performance testing
- Time-based regeneration

### 3. Performance Testing (`03_performance_testing.rs`)
**Difficulty**: Advanced  
**Duration**: 20-30 minutes  
**Description**: Comprehensive performance testing and monitoring capabilities.

**What you'll learn**:
- Performance monitoring setup
- Load testing
- Stress testing
- Memory usage analysis
- Performance reporting

**Key Features Demonstrated**:
- Performance metrics collection
- Load testing with 1000+ actors
- Stress testing system limits
- Memory usage profiling
- Comprehensive performance reports

## 🚀 **Quick Start**

### Prerequisites

1. **Rust Environment**: Ensure you have Rust 1.70+ installed
2. **Dependencies**: The examples use the `chaos-backend-service` crate
3. **MongoDB** (optional): For database persistence examples

### Running Examples

1. **Basic Usage**:
   ```bash
   cd chaos-backend-service
   cargo run --example basic_usage
   ```

2. **Advanced Usage**:
   ```bash
   cd chaos-backend-service
   cargo run --example advanced_usage
   ```

3. **Performance Testing**:
   ```bash
   cd chaos-backend-service
   cargo run --example performance_testing
   ```

## 📖 **Example Walkthrough**

### Basic Usage Example

```rust
use chaos_backend_service::crates::actor_core::subsystems::*;
use chaos_backend_service::crates::actor_core::types::Actor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the resource manager
    let resource_manager = setup_resource_manager().await?;
    
    // 2. Create a player character
    let mut player = create_player_character("Hero", "Human", 15, 20, 18, 16, 14);
    
    // 3. Calculate resources
    let result = resource_manager.contribute(&player).await?;
    
    // 4. Display results
    display_resource_results(&result);
    
    Ok(())
}
```

### Advanced Usage Example

```rust
// Set up comprehensive resource management system
let (resource_manager, event_manager, performance_monitor) = setup_advanced_system().await?;

// Create multiple characters with different builds
let characters = create_diverse_characters();

// Test resource calculation for all characters
for character in &characters {
    test_character_resources(&resource_manager, character).await?;
}

// Set up event handling
setup_event_handling(&event_manager).await?;

// Simulate combat scenario
simulate_combat_scenario(&resource_manager, &event_manager, &characters).await?;
```

### Performance Testing Example

```rust
// Set up performance monitoring
let performance_monitor = setup_performance_monitoring().await?;

// Create load testing suite
let mut load_test_suite = LoadTestingSuite::new(performance_monitor.clone());

// Generate test actors
load_test_suite.generate_test_actors(1000)?;

// Run comprehensive tests
let test_results = load_test_suite.run_comprehensive_test_suite().await?;

// Generate performance report
generate_performance_report(&performance_monitor).await?;
```

## 🎯 **Learning Path**

### For Beginners
1. Start with **Basic Usage** to understand the fundamentals
2. Experiment with different character builds and stats
3. Try modifying the resource calculation formulas
4. Test different regeneration scenarios

### For Intermediate Users
1. Run **Advanced Usage** to see multi-system integration
2. Implement custom event listeners
3. Experiment with different resource systems
4. Try different combat scenarios

### For Advanced Users
1. Use **Performance Testing** to understand system limits
2. Implement custom performance metrics
3. Optimize resource calculations
4. Build custom load testing scenarios

## 🔧 **Customization Examples**

### Custom Resource System

```rust
use chaos_backend_service::crates::actor_core::subsystems::*;
use async_trait::async_trait;

// Create a custom resource system
struct CustomResourceSystem {
    system_id: String,
}

#[async_trait]
impl SystemResourceCalculator for CustomResourceSystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn affects_resource(&self, resource_id: &str) -> bool {
        resource_id.starts_with("custom_")
    }
    
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut resources = HashMap::new();
        
        // Custom resource calculation logic
        let custom_energy = calculate_custom_energy(actor).await?;
        resources.insert("custom_energy_current".to_string(), custom_energy);
        
        Ok(resources)
    }
    
    // ... implement other required methods
}
```

### Custom Event Listener

```rust
use chaos_backend_service::crates::actor_core::subsystems::*;
use async_trait::async_trait;

struct CustomEventListener {
    listener_id: String,
}

#[async_trait]
impl ResourceEventListener for CustomEventListener {
    async fn handle_event(&self, event: &ResourceEvent) -> ActorCoreResult<()> {
        match event.event_type {
            ResourceEventType::ResourceDepleted => {
                println!("Custom handling for resource depletion: {}", event.resource_name);
                // Custom logic here
            }
            _ => {
                // Handle other events
            }
        }
        Ok(())
    }
    
    fn listener_id(&self) -> &str {
        &self.listener_id
    }
    
    fn interested_event_types(&self) -> Vec<ResourceEventType> {
        vec![ResourceEventType::ResourceDepleted]
    }
    
    fn event_priority(&self) -> EventPriority {
        EventPriority::High
    }
}
```

### Custom Performance Metric

```rust
use chaos_backend_service::crates::actor_core::subsystems::*;

// Record custom performance metrics
let custom_metric = PerformanceMetric {
    name: "custom_operation_time".to_string(),
    value: 150.0,
    unit: "ms".to_string(),
    timestamp: std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(),
    category: MetricCategory::ResourceCalculation,
};

performance_monitor.record_metric(custom_metric).await?;
```

## 🐛 **Troubleshooting**

### Common Issues

1. **Compilation Errors**:
   - Ensure all dependencies are properly imported
   - Check that the `chaos-backend-service` crate is available
   - Verify Rust version compatibility

2. **Runtime Errors**:
   - Check that all required data is set on actors
   - Verify resource manager initialization
   - Ensure proper error handling

3. **Performance Issues**:
   - Use performance monitoring to identify bottlenecks
   - Check memory usage and cache configuration
   - Consider adjusting batch sizes and timeouts

### Debug Tips

1. **Enable Debug Logging**:
   ```rust
   env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
   ```

2. **Check Resource Manager State**:
   ```rust
   let stats = resource_cache.get_stats().await?;
   println!("Cache stats: {:?}", stats);
   ```

3. **Monitor Event System**:
   ```rust
   let event_stats = event_manager.get_event_stats().await?;
   println!("Event stats: {:?}", event_stats);
   ```

## 📚 **Additional Resources**

- [API Documentation](../07_API_Documentation.md)
- [Implementation Plan](../06_Enhanced_Hybrid_Implementation_Plan.md)
- [Resource Manager Overview](../00_Resource_Manager_Overview.md)
- [System Design](../01_Resource_System_Design.md)

## 🤝 **Contributing**

To add new examples:

1. Create a new file following the naming convention: `XX_description.rs`
2. Include comprehensive documentation and comments
3. Add the example to this README
4. Ensure the example compiles and runs successfully
5. Test with different configurations and scenarios

## 📄 **License**

These examples are part of the Enhanced Hybrid Resource Manager system and follow the same license terms.
