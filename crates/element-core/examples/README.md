# Element-Core Integration Examples

This directory contains comprehensive examples demonstrating how to integrate `element-core` with various game systems.

## 📁 Examples Overview

### 1. **Race-Core Integration** (`race_core_integration.rs`)
Demonstrates how to integrate `element-core` with a race system.

**Key Features:**
- Racial elemental affinities
- Racial traits affecting elemental abilities
- Base racial stats contribution
- Event handling for element changes
- Priority-based contribution processing

**Usage:**
```bash
cargo run --example race_core_integration
```

### 2. **Item-Core Integration** (`item_core_integration.rs`)
Demonstrates how to integrate `element-core` with an item system.

**Key Features:**
- Item elemental properties
- Enchantment effects
- Rarity-based power scaling
- Equipment bonuses
- Item management operations

**Usage:**
```bash
cargo run --example item_core_integration
```

### 3. **Skill-Core Integration** (`skill_core_integration.rs`)
Demonstrates how to integrate `element-core` with a skill system.

**Key Features:**
- Skill elemental effects
- Level-based power scaling
- Mastery bonuses
- Skill type classification
- Experience-based progression

**Usage:**
```bash
cargo run --example skill_core_integration
```

### 4. **Actor-Core Integration** (`actor_core_integration.rs`)
Demonstrates how to integrate `element-core` with an actor system.

**Key Features:**
- Actor class bonuses
- Level-based bonuses
- Status effect contributions
- Equipment bonuses
- Base stat contributions

**Usage:**
```bash
cargo run --example actor_core_integration
```

### 5. **Comprehensive Integration** (`comprehensive_integration.rs`)
Demonstrates a complete integration of all systems working together.

**Key Features:**
- Multi-system integration
- Element interaction testing
- Event broadcasting
- Comprehensive metrics
- Real-world usage scenarios

**Usage:**
```bash
cargo run --example comprehensive_integration
```

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+
- Element-Core crate
- Actor-Core crate

### Running Examples

1. **Run a specific example:**
   ```bash
   cargo run --example <example_name>
   ```

2. **Run all examples:**
   ```bash
   cargo run --example race_core_integration
   cargo run --example item_core_integration
   cargo run --example skill_core_integration
   cargo run --example actor_core_integration
   cargo run --example comprehensive_integration
   ```

3. **Run with debug output:**
   ```bash
   RUST_LOG=debug cargo run --example comprehensive_integration
   ```

## 📊 Example Output

Each example provides detailed output showing:
- System initialization
- Element registration
- Contributor registration
- Contribution collection
- Stat aggregation
- Event handling
- Performance metrics

## 🔧 Customization

### Adding New Contributors

To add a new contributor system:

1. **Implement the `ElementContributor` trait:**
   ```rust
   use element_core::ElementContributor;
   
   pub struct MySystemContributor {
       // Your system data
   }
   
   #[async_trait]
   impl ElementContributor for MySystemContributor {
       // Implement required methods
   }
   ```

2. **Register with the contributor registry:**
   ```rust
   let contributor = Arc::new(MySystemContributor::new());
   contributor_registry.register_contributor(contributor).await?;
   ```

3. **Collect contributions:**
   ```rust
   let contributions = contributor_registry.collect_contributions(&actor, "fire").await?;
   ```

### Custom Element Types

To add custom element types:

1. **Define your element:**
   ```rust
   let element = ElementDefinition::new(
       "custom_element".to_string(),
       "Custom Element".to_string(),
       "Description".to_string(),
       ElementCategory::Custom,
   );
   ```

2. **Register with the registry:**
   ```rust
   registry.register_element(element).await?;
   ```

### Custom Interactions

To add custom element interactions:

1. **Define the interaction:**
   ```rust
   let interaction = ElementInteraction::new(
       "custom_interaction".to_string(),
       "source_element".to_string(),
       "target_element".to_string(),
       InteractionType::Custom,
   );
   ```

2. **Register with the registry:**
   ```rust
   registry.register_interaction(interaction).await?;
   ```

## 🧪 Testing

Each example includes comprehensive tests:

```bash
# Run tests for a specific example
cargo test --example race_core_integration

# Run all example tests
cargo test --examples
```

## 📈 Performance Considerations

### Optimization Tips

1. **Use Arc for shared data:**
   ```rust
   let contributor = Arc::new(MyContributor::new());
   ```

2. **Batch operations when possible:**
   ```rust
   let contributions = contributor_registry.collect_contributions(&actor, element_type).await?;
   ```

3. **Cache frequently accessed data:**
   ```rust
   let cached_stats = aggregator.get_cached_stats(element_type).await?;
   ```

### Memory Management

- Contributors are stored in `Arc<dyn ElementContributor>`
- Use `DashMap` for thread-safe concurrent access
- Implement proper cleanup in `shutdown()` methods

## 🐛 Troubleshooting

### Common Issues

1. **Compilation errors:**
   - Ensure all dependencies are properly imported
   - Check trait implementations are complete

2. **Runtime errors:**
   - Verify element registration before use
   - Check contributor registration order

3. **Performance issues:**
   - Use appropriate aggregation strategies
   - Consider caching for frequently accessed data

### Debug Mode

Enable debug logging:
```bash
RUST_LOG=debug cargo run --example comprehensive_integration
```

## 📚 Further Reading

- [Element-Core Documentation](../docs/)
- [API Reference](../docs/api/)
- [Integration Guide](../docs/integration/)
- [Performance Guide](../docs/performance/)

## 🤝 Contributing

To add new examples:

1. Create a new file in this directory
2. Follow the existing pattern
3. Include comprehensive tests
4. Update this README
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
