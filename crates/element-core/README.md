# Element-Core

A high-performance, thread-safe elemental system for game backends, designed to manage elemental data, interactions, and character progression with optimal performance and scalability.

## 🚀 Features

- **High Performance**: Array-based data structures for optimal cache efficiency
- **Thread Safety**: Safe concurrent access with `Arc<RwLock<T>>` patterns
- **Extensible**: Plugin system for custom elements and interactions
- **Comprehensive**: Full elemental mastery, interaction, and contribution system
- **Well Documented**: Extensive documentation and examples
- **Type Safe**: Rust's type system ensures data integrity
- **Async/Await**: Modern async patterns for I/O operations

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [Core Concepts](#core-concepts)
- [API Reference](#api-reference)
- [Integration Guide](#integration-guide)
- [Performance Guide](#performance-guide)
- [Troubleshooting](#troubleshooting)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## 🏃 Quick Start

### Installation

Add Element-Core to your `Cargo.toml`:

```toml
[dependencies]
element-core = { path = "../element-core" }
tokio = { version = "1.0", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
```

### Basic Usage

```rust
use element_core::{UnifiedElementRegistry, ElementDefinition, ElementProperties};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create registry
    let registry = Arc::new(UnifiedElementRegistry::new());
    
    // Create fire element
    let fire_element = ElementDefinition {
        id: "fire".to_string(),
        name: "Fire".to_string(),
        description: "Element of flame and heat".to_string(),
        category: ElementCategory::Elemental,
        base_properties: ElementProperties {
            power_multiplier: 1.2,
            defense_multiplier: 0.8,
            mastery_gain_rate: 1.0,
            qi_efficiency: 0.9,
            interaction_bonus: 0.0,
            status_effect_resistance: 0.5,
            environmental_adaptation: 0.7,
            synergy_bonus: 0.0,
        },
        derived_stats: vec![],
        status_effects: vec![],
        environment_mods: vec![],
        references: ElementReferences::default(),
        aliases: ElementAliases::default(),
    };
    
    // Register element
    registry.register_element(fire_element).await?;
    
    // Get element
    let element = registry.get_element("fire").await?;
    println!("Element: {}", element.name);
    
    Ok(())
}
```

### Run Examples

```bash
cd element-core
cargo run --example integration_examples
```

## 🏗️ Architecture

Element-Core follows a **Data Hub Pattern** where it aggregates elemental data from multiple sources without containing business logic.

### Core Components

- **UnifiedElementRegistry**: Central registry for all elements and interactions
- **ElementDefinition**: Defines element properties and configurations
- **ElementalSystemData**: Stores actor's elemental mastery and stats
- **ElementContributor**: Trait for external systems to contribute data
- **ElementAggregator**: Aggregates contributions from multiple sources

### Design Principles

1. **Data Hub**: Aggregates data without business logic
2. **External Contributors**: Other systems contribute via standardized interfaces
3. **Loose Coupling**: Systems can be developed independently
4. **Thread Safety**: All operations are safe for concurrent access
5. **High Performance**: Array-based structures for optimal performance

## 🧠 Core Concepts

### Elemental System

The elemental system is based on the concept of "Tinh", "Khí", and "Thần" (Essence, Qi, and Spirit) from cultivation novels:

- **Tinh (Essence)**: Physical body and material aspects
- **Khí (Qi)**: Energy and power systems
- **Thần (Spirit)**: Mental and spiritual aspects

### Element Categories

- **Physical**: Earth, Metal, Wood
- **Elemental**: Fire, Water, Ice, Lightning
- **Spiritual**: Light, Dark, Void
- **Dimensional**: Space, Time, Reality
- **Hybrid**: Combinations of multiple elements

### Mastery System

Elements have mastery levels that affect their effectiveness:

```rust
pub enum ElementMasteryRank {
    Novice,      // 0-10
    Apprentice,  // 11-25
    Adept,       // 26-50
    Expert,      // 51-75
    Master,      // 76-90
    Grandmaster, // 91-100
}
```

### Element Interactions

Elements interact with each other based on "tương sinh tương khắc" (mutual generation and mutual overcoming):

- **Fire** overcomes **Water** (1.5x damage)
- **Water** overcomes **Fire** (1.3x damage)
- **Earth** overcomes **Wood** (1.2x damage)
- **Wood** overcomes **Earth** (1.1x damage)

## 📚 API Reference

### Common Traits

Element-Core provides a set of common traits for consistent API patterns:

- **ElementGetter<T>**: Standardized element retrieval
- **ElementSetter<T>**: Standardized element modification
- **Validatable**: Data integrity validation
- **Cacheable**: Cache management and statistics
- **MetricsProvider**: Performance monitoring
- **Configurable**: Configuration management
- **Serializable**: Data persistence
- **ElementHelper**: Utility functions

### Error Handling

All operations return `ElementCoreResult<T>`, which is a `Result<T, ElementCoreError>`:

```rust
pub type ElementCoreResult<T> = Result<T, ElementCoreError>;

#[derive(Error, Debug)]
pub enum ElementCoreError {
    #[error("Validation error: {message}")]
    Validation { message: String },
    #[error("Element not found: {element_id}")]
    ElementNotFound { element_id: String },
    #[error("Configuration error: {message}")]
    Config { message: String },
    // ... other error variants
}
```

### Registry Operations

```rust
// Register element
registry.register_element(element).await?;

// Get element
let element = registry.get_element("fire").await?;

// Check if element exists
if registry.has_element("fire") {
    // Element exists
}

// Get all elements
let elements = registry.get_all_elements().await?;
```

### Contributor System

External systems can contribute elemental data:

```rust
#[async_trait]
impl ElementContributor for MySystem {
    fn system_id(&self) -> &str { "my-system" }
    fn priority(&self) -> i64 { 500 }
    
    async fn contribute_element_stats(
        &self, 
        actor: &Actor, 
        element_type: &str
    ) -> ElementCoreResult<ElementContribution> {
        // Provide elemental contributions
    }
    
    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()> {
        // Handle elemental events
    }
}
```

## 🔗 Integration Guide

### Step 1: Add Dependencies

```toml
[dependencies]
element-core = { path = "../element-core" }
actor-core = { path = "../actor-core" }
tokio = { version = "1.0", features = ["full"] }
```

### Step 2: Create Contributors

```rust
use element_core::{ElementContributor, ElementContribution};
use async_trait::async_trait;

pub struct RaceContributor {
    system_id: String,
    priority: i64,
}

#[async_trait]
impl ElementContributor for RaceContributor {
    // Implementation
}
```

### Step 3: Register Contributors

```rust
let registry = Arc::new(UnifiedElementRegistry::new());
let contributor = Arc::new(RaceContributor::new());
registry.register_contributor(contributor).await?;
```

### Step 4: Aggregate Contributions

```rust
let contribution = registry.aggregate_contributions(&actor, "fire").await?;
```

For detailed integration instructions, see [Integration Guide](docs/Integration_Guide.md).

## ⚡ Performance Guide

Element-Core is optimized for high-performance game backends:

### Performance Targets

- **Element Lookup**: < 1μs
- **Element Registration**: < 10μs
- **Contribution Aggregation**: < 100μs
- **Cache Hit Rate**: > 95%
- **Memory Usage**: < 1MB per 1000 elements

### Optimization Strategies

1. **Use Fixed-Size Arrays**: For known maximums
2. **Minimize String Allocations**: Use string interning
3. **Batch Operations**: Group multiple operations
4. **Use Appropriate Data Structures**: HashMap for O(1) lookups
5. **Optimize Hot Paths**: Focus on frequently executed code

For detailed performance optimization, see [Performance Guide](docs/Performance_Guide.md).

## 🐛 Troubleshooting

### Common Issues

1. **Compilation Errors**: Check ElementCoreError variants and imports
2. **Runtime Errors**: Validate data before use
3. **Performance Issues**: Monitor cache hit rates and memory usage
4. **Integration Issues**: Ensure proper async/await usage

### Debug Mode

Enable debug logging:

```rust
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
```

For detailed troubleshooting, see [Troubleshooting Guide](docs/Troubleshooting_Guide.md).

## 📖 Examples

### Simple Integration

```rust
use element_core::{UnifiedElementRegistry, ElementContributor, ElementContribution};
use std::collections::HashMap;

pub struct SimpleContributor {
    system_id: String,
    priority: i64,
}

#[async_trait]
impl ElementContributor for SimpleContributor {
    fn system_id(&self) -> &str { &self.system_id }
    fn priority(&self) -> i64 { self.priority }
    
    async fn contribute_element_stats(
        &self,
        _actor: &Actor,
        element_type: &str,
    ) -> ElementCoreResult<ElementContribution> {
        let mut stats = HashMap::new();
        stats.insert("power".to_string(), 100.0);
        Ok(self.create_contribution(element_type, stats))
    }
    
    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()> {
        println!("Event: {:?}", event);
        Ok(())
    }
}
```

### Configuration

```yaml
# element_config.yaml
registry:
  cache:
    max_size: 1000
    ttl_seconds: 3600
    eviction_policy: "lru"
  
  performance:
    max_elements: 50
    thread_pool_size: 4
    enable_parallel_processing: true

elements:
  fire:
    name: "Fire"
    description: "Element of flame and heat"
    category: "elemental"
    base_properties:
      power_multiplier: 1.2
      defense_multiplier: 0.8
      mastery_gain_rate: 1.0
      qi_efficiency: 0.9
```

### Running Examples

```bash
# Run integration examples
cargo run --example integration_examples

# Run specific example
cargo run --example simple_integration
```

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

### Development Setup

```bash
# Clone repository
git clone <repository-url>
cd element-core

# Install dependencies
cargo build

# Run tests
cargo test

# Run examples
cargo run --examples
```

### Code Style

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Add documentation for public APIs
- Write tests for new features

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by cultivation novels and elemental systems
- Built with Rust for performance and safety
- Uses modern async/await patterns
- Designed for game backend services

## 📞 Support

- **Documentation**: [API Reference](docs/API_Reference.md)
- **Integration**: [Integration Guide](docs/Integration_Guide.md)
- **Performance**: [Performance Guide](docs/Performance_Guide.md)
- **Troubleshooting**: [Troubleshooting Guide](docs/Troubleshooting_Guide.md)
- **Issues**: [GitHub Issues](https://github.com/your-org/element-core/issues)

---

**Element-Core** - High-performance elemental system for game backends 🚀
