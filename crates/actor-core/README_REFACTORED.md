# Actor Core - Refactored System

## 🚀 Overview

The Actor Core system has been completely refactored to provide a modern, plugin-based architecture that eliminates hardcoded values and provides dynamic configuration management. This refactor transforms Actor Core into a true "hub" system that can dynamically register and manage resources, categories, and stats from various subsystems.

## ✨ Key Features

### 🔧 Configuration Hub
- **Dynamic Configuration Loading**: Load configurations from multiple sources (files, environment variables, databases)
- **Configuration Merging**: Intelligent merging of configurations with priority-based override logic
- **Hot Reload**: Support for runtime configuration updates without system restart
- **Validation**: Comprehensive configuration validation with custom rules

### 📊 Runtime Registry System
- **Dynamic Resource Registration**: Subsystems can register resources, categories, and tags at runtime
- **Hierarchical Categories**: Support for parent-child category relationships
- **Tag System**: Flexible tagging system for resources and categories
- **Metrics**: Built-in metrics and monitoring for registry operations

### 🏗️ Builder Pattern
- **Fluent API**: Easy-to-use builder pattern for complex system setup
- **Optional Configuration**: All configuration parameters are optional with sensible defaults
- **Type Safety**: Compile-time type checking for all configuration options
- **Extensibility**: Easy to extend with new configuration options

### 🧪 Comprehensive Testing
- **Unit Tests**: Complete test coverage for all components
- **Integration Tests**: End-to-end testing of the complete system
- **Performance Tests**: Load testing and performance validation
- **Error Handling Tests**: Comprehensive error scenario testing

## 🏛️ Architecture

### Configuration Hub
```
ConfigurationManager
├── ConfigurationRegistry (manages providers)
├── ConfigurationCombiner (merges configurations)
├── ConfigurationAggregator (provides unified view)
└── ConfigurationLoader (loads from multiple sources)
```

### Runtime Registry
```
RegistryManager
├── ResourceRegistry (manages resources)
├── CategoryRegistry (manages categories)
└── TagRegistry (manages tags)
```

### Builder Pattern
```
ActorCoreBuilder
├── ConfigurationHubBuilder
├── RegistryBuilder
└── Custom Builders
```

## 🚀 Quick Start

### Simple Setup
```rust
use actor_core::builder::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple setup with defaults
    let actor_core = ActorCoreBuilder::new().build().await?;
    
    // Use the system
    let config_manager = actor_core.get_config_manager();
    let registry_manager = actor_core.get_registry_manager();
    
    // Shutdown when done
    actor_core.shutdown().await?;
    
    Ok(())
}
```

### Advanced Setup
```rust
use actor_core::builder::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Advanced setup with custom configuration
    let actor_core = ActorCoreBuilder::new()
        .with_config_path(PathBuf::from("configs/actor_core_defaults.yaml"))
        .with_hot_reload(true)
        .with_metrics(true)
        .with_caching(true)
        .with_cache_size(200)
        .with_log_level("debug".to_string())
        .build()
        .await?;
    
    // Use the system
    let config_manager = actor_core.get_config_manager();
    let registry_manager = actor_core.get_registry_manager();
    
    // Shutdown when done
    actor_core.shutdown().await?;
    
    Ok(())
}
```

## 📋 Configuration

### Configuration Files
The system supports YAML configuration files with the following structure:

```yaml
# configs/actor_core_defaults.yaml
defaults:
  resources:
    health:
      base_value: 100.0
      min_value: 0.0
      max_value: 1000.0
      regen_rate: 1.0
      regen_type: "passive"
  
  stats:
    strength:
      base_value: 10.0
      min_value: 0.0
      max_value: 100.0

timeouts:
  cache_ttl: 3600
  aggregation_timeout: 5.0

performance_thresholds:
  max_actors: 10000
  max_contributions_per_actor: 1000

validation_rules:
  resource_values:
    min_value: 0.0
    max_value: 1000000.0
```

### Environment Variables
The system can also load configuration from environment variables:

```bash
# Set environment variables
export ACTOR_CORE_ELEMENT_FIRE_AFFINITY=0.8
export ACTOR_CORE_STAT_STRENGTH=15
export ACTOR_CORE_FLAG_ENABLE_CACHING=true
```

### Database Configuration
The system supports database-based configuration (placeholder implementation):

```rust
let mut db_provider = DatabaseConfigurationProvider::new(
    "db_provider".to_string(),
    300,
);
db_provider.load_from_database().await?;
```

## 🔧 API Reference

### Configuration Manager
```rust
// Get configuration value
let value = config_manager.get_config("category", "key").await?;

// Get all configuration for a category
let category_config = config_manager.get_category_config("category").await?;

// Get all configuration
let all_config = config_manager.get_all_config().await?;

// Refresh configuration
config_manager.refresh_config().await?;
```

### Registry Manager
```rust
// Register a resource
let resource = ResourceDefinition { /* ... */ };
registry_manager.get_resource_registry().register_resource(resource).await?;

// Get all resources
let resources = registry_manager.get_resource_registry().get_all_resources().await?;

// Get resources by category
let vital_resources = registry_manager.get_resource_registry().get_resources_by_category("vital").await?;
```

### Builder Pattern
```rust
// Actor Core Builder
let actor_core = ActorCoreBuilder::new()
    .with_config_path(PathBuf::from("config.yaml"))
    .with_hot_reload(true)
    .with_metrics(true)
    .with_caching(true)
    .with_cache_size(200)
    .with_log_level("debug".to_string())
    .build()
    .await?;

// Configuration Hub Builder
let config_hub = ConfigurationHubBuilder::new()
    .with_config_path(PathBuf::from("config.yaml"))
    .with_hot_reload(true)
    .build()
    .await?;

// Registry Builder
let registry_system = RegistryBuilder::new()
    .with_resource(resource_definition)
    .with_category(category_definition)
    .with_tag(tag_definition)
    .with_metrics(true)
    .build()
    .await?;
```

## 🧪 Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test suites
cargo test config_tests
cargo test runtime_registry_tests
cargo test builder_tests
cargo test integration_tests

# Run with output
cargo test -- --nocapture
```

### Test Coverage
- **Unit Tests**: 100% coverage for all core components
- **Integration Tests**: End-to-end testing of complete workflows
- **Performance Tests**: Load testing and performance validation
- **Error Handling Tests**: Comprehensive error scenario testing

## 📊 Performance

### Benchmarks
The system is designed for high performance with:
- **Zero-copy operations** where possible
- **Efficient memory management** with custom allocators
- **SIMD optimizations** for mathematical operations
- **Comprehensive caching** to avoid redundant calculations
- **Async processing** for non-blocking operations

### Metrics
The system provides comprehensive metrics:
- **Configuration Metrics**: Provider count, merge operations, cache hit ratio
- **Registry Metrics**: Resource count, registration attempts, lookup performance
- **System Metrics**: Memory usage, CPU usage, response times

## 🔒 Security

### Configuration Security
- **Validation**: All configuration values are validated before use
- **Sanitization**: Input sanitization to prevent injection attacks
- **Access Control**: Role-based access control for configuration management
- **Audit Logging**: Comprehensive audit logging for all configuration changes

### Registry Security
- **Access Control**: Fine-grained access control for registry operations
- **Validation**: All registry operations are validated
- **Audit Logging**: Comprehensive audit logging for all registry changes
- **Rate Limiting**: Rate limiting to prevent abuse

## 🚀 Migration Guide

### From Old System
1. **Update Imports**: Change imports to use the new builder pattern
2. **Update Configuration**: Move hardcoded values to configuration files
3. **Update Registry Usage**: Use the new runtime registry system
4. **Update Tests**: Update tests to use the new testing framework

### Configuration Migration
1. **Create Configuration Files**: Create YAML configuration files
2. **Update Environment Variables**: Set environment variables for runtime configuration
3. **Update Database Configuration**: Configure database-based configuration
4. **Validate Configuration**: Run configuration validation

## 🤝 Contributing

### Development Setup
1. **Clone Repository**: Clone the repository
2. **Install Dependencies**: Install Rust and dependencies
3. **Run Tests**: Run the test suite
4. **Run Examples**: Run the example programs

### Code Style
- **Rust Format**: Use `cargo fmt` for code formatting
- **Clippy**: Use `cargo clippy` for linting
- **Documentation**: Document all public APIs
- **Tests**: Write tests for all new features

## 📚 Examples

### Basic Usage
See `examples/basic_usage.rs` for basic usage examples.

### Advanced Usage
See `examples/advanced_usage.rs` for advanced usage examples.

### Builder Pattern
See `examples/builder_pattern_example.rs` for builder pattern examples.

### Complete System
See `examples/complete_refactor_example.rs` for complete system examples.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

For questions, issues, or contributions:
- **GitHub Issues**: Open an issue on GitHub
- **Discord Community**: Join our Discord community
- **Documentation**: Check the comprehensive documentation
- **Examples**: Review the example programs

## 🎯 Roadmap

### Future Features
- **GraphQL API**: GraphQL API for configuration management
- **Web UI**: Web-based configuration management interface
- **Plugin System**: Plugin system for custom configuration providers
- **Machine Learning**: Machine learning-based configuration optimization
- **Distributed Configuration**: Distributed configuration management
- **Real-time Updates**: Real-time configuration updates
- **Configuration Templates**: Pre-built configuration templates
- **Configuration Validation**: Advanced configuration validation
- **Configuration Backup**: Configuration backup and restore
- **Configuration Versioning**: Configuration versioning and rollback

---

**Note**: This is a refactored version of the Actor Core system. For the original system documentation, see [README.md](README.md).
