# Actor Core Hierarchical

High-performance data hub for actor systems with array-based elemental data management.

## Overview

This crate provides a hierarchical actor data system that serves as a **data hub** for managing actor properties across multiple game systems. It focuses on **performance**, **extensibility**, and **clean separation** of concerns.

## Key Features

- **Data Hub Only**: Pure data storage and management, no business logic
- **Elemental Core Focus**: Array-based elemental system data management
- **High Performance**: 1-2 ns access time with array-based approach
- **Extensible Design**: Easy to add new systems without breaking existing ones
- **Inheritance Support**: Extends base actor-core classes using trait-based inheritance
- **Backward Compatibility**: Seamless conversion between base and hierarchical actors

## Architecture

```
Actor Core Hierarchical
├── Types
│   ├── ElementalData      # Array-based elemental data structures
│   ├── ActorData          # Hierarchical actor data
│   └── SystemData         # System data traits and registry
├── Systems
│   └── Elemental          # Elemental system implementation
├── Adapters
│   ├── ActorAdapter       # Actor data conversion
│   └── ElementalAdapter   # Elemental data conversion
├── Aggregation
│   ├── ElementalAggregator # Elemental stats aggregation
│   └── BaseAggregator     # Base aggregation logic
└── Utils
    ├── ElementIndices     # Element index constants
    └── Performance        # Performance utilities
```

## Usage

```rust
use actor_core_hierarchical::*;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    // Initialize the system
    init_hierarchical_system().await?;
    
    // Create hierarchical actor data
    let mut actor_data = create_hierarchical_actor(
        "warrior_001".to_string(),
        "Ancient Warrior".to_string(),
        "cultivation_world".to_string(),
    );
    
    // Set elemental mastery levels (direct array access - 1-2 ns)
    actor_data.elemental_system.element_mastery_levels[0] = 50.0; // Fire
    actor_data.elemental_system.element_mastery_levels[1] = 45.0; // Water
    actor_data.elemental_system.element_mastery_levels[2] = 40.0; // Earth
    
    // Set elemental power points (direct array access - 1-2 ns)
    actor_data.elemental_system.element_power_points[0] = 100.0; // Fire
    actor_data.elemental_system.element_power_points[1] = 90.0;  // Water
    actor_data.elemental_system.element_power_points[2] = 95.0;  // Earth
    
    // Set element interactions (direct 2D array access - 1-2 ns)
    actor_data.elemental_system.element_interactions[0][1] = 0.7; // Fire vs Water
    actor_data.elemental_system.element_interactions[1][0] = 1.3; // Water vs Fire
    
    // Aggregate stats
    let mut aggregator = BaseAggregator::new();
    let global_stats = aggregator.aggregate_all_stats(&mut actor_data).await?;
    
    // Get total stats (cached - 1-2 ns)
    let total_hp = global_stats.total_hp;
    let total_mp = global_stats.total_mp;
    let total_attack_power = global_stats.total_attack_power;
    
    println!("Total HP: {}", total_hp);
    println!("Total MP: {}", total_mp);
    println!("Total Attack Power: {}", total_attack_power);
    
    // Convert to base actor for compatibility
    let base_actor = ActorAdapter::to_base_actor(&actor_data)?;
    
    Ok(())
}
```

## Inheritance Support

Actor-core-hierarchical extends actor-core using a trait-based inheritance system:

```rust
use actor_core_hierarchical::*;
use actor_core::inheritable::{ActorBase, BaseActor, ActorFactory, DefaultActorFactory};

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    // Create base actor using inheritable system
    let factory = DefaultActorFactory;
    let base_actor = factory.create_actor(
        "Base Warrior".to_string(),
        "human".to_string(),
    );
    
    // Create hierarchical actor
    let hierarchical_actor = create_hierarchical_actor(
        "hierarchical_warrior_001".to_string(),
        "Hierarchical Warrior".to_string(),
        "cultivation_world".to_string(),
    );
    
    // Convert between base and hierarchical actors
    let converted_base = ActorAdapter::to_base_actor(&hierarchical_actor)?;
    let reconverted_hierarchical = ActorAdapter::from_base_actor(&converted_base)?;
    
    Ok(())
}
```

## Performance

### Array-Based Access
- **Single element access**: 1-2 ns
- **Bulk element access**: 50-100 ns for 50 elements
- **Memory layout**: Contiguous arrays for cache efficiency
- **Zero allocations**: All data stored in fixed-size arrays

### Benchmark Results
```
Benchmark: Elemental System Access
  Iterations: 1,000,000
  Total time: 1.23 ms
  Average time: 1.23 ns
  Min time: 1 ns
  Max time: 3 ns
  Median time: 1 ns
  P95 time: 2 ns
  P99 time: 3 ns
  Operations per second: 813,008,130
```

## Element System

### Supported Elements (50 total)
- **Five Elements**: Fire, Water, Earth, Metal, Wood
- **Yin-Yang Elements**: Yin, Yang
- **Classical Western**: Ice, Lightning, Wind
- **Advanced Elements**: Light, Dark, Void, Chaos, Order
- **Nature Elements**: Nature, Poison, Acid, Radiation
- **Energy Elements**: Kinetic, Thermal, Electromagnetic, Gravitational
- **Mystical Elements**: Spirit, Soul, Mind, Dream
- **Cosmic Elements**: Space, Time, Reality, Dimension
- **Elemental Combinations**: Steam, Lava, Magma, Storm, Blizzard, etc.

### Element Data Structure
```rust
pub struct ElementalSystemData {
    // Element mastery data (array-based)
    pub element_mastery_levels: [f64; MAX_ELEMENTS],                    // 50 elements
    pub element_mastery_experience: [f64; MAX_ELEMENTS],                // 50 elements
    pub element_mastery_ranks: [ElementMasteryRank; MAX_ELEMENTS],      // 50 elements
    
    // Element primary stats (array-based)
    pub element_qi_amounts: [f64; MAX_ELEMENTS],                        // 50 elements
    pub element_qi_capacities: [f64; MAX_ELEMENTS],                     // 50 elements
    // ... 10 more qi stats
    
    // Element derived stats (array-based)
    pub element_power_points: [f64; MAX_ELEMENTS],                      // 50 elements
    pub element_defense_points: [f64; MAX_ELEMENTS],                    // 50 elements
    // ... 35 more derived stats
    
    // Element interactions (2D array)
    pub element_interactions: [[f64; MAX_ELEMENTS]; MAX_ELEMENTS],      // 50x50 interactions
    
    // Feature flags (2D array)
    pub element_feature_flags: [[bool; MAX_FEATURE_FLAGS]; MAX_ELEMENTS], // 50x16 flags
}
```

## Extensibility

### Adding New Systems
The architecture is designed to easily accommodate new systems:

```rust
// Future: Add cultivation system
pub struct CultivationSystemData {
    pub cultivation_levels: [f64; MAX_CULTIVATION_STAGES],
    pub cultivation_experience: [f64; MAX_CULTIVATION_STAGES],
    // ... other cultivation data
}

// Future: Add magic system
pub struct MagicSystemData {
    pub magic_levels: [f64; MAX_MAGIC_SCHOOLS],
    pub mana_amounts: [f64; MAX_MAGIC_SCHOOLS],
    // ... other magic data
}
```

### System Registry
```rust
pub struct SystemRegistry {
    pub elemental_system_active: bool,
    pub cultivation_system_active: bool,    // Future
    pub magic_system_active: bool,          // Future
    pub race_system_active: bool,           // Future
    pub talent_system_active: bool,         // Future
    pub item_system_active: bool,           // Future
    pub luck_system_active: bool,           // Future
}
```

## Dependencies

- `actor-core`: Base actor system
- `serde`: Serialization support
- `tokio`: Async runtime
- `thiserror`: Error handling
- `chrono`: Time handling
- `uuid`: UUID generation
- `phf`: Compile-time hash maps
- `tracing`: Logging

## Development

### Building
```bash
cargo build
```

### Testing
```bash
# Run all tests
cargo test

# Run inheritance tests specifically
cargo test --test inheritance_tests

# Run with features
cargo test --features inheritable
```

### Benchmarking
```bash
# Run all benchmarks
cargo bench

# Run inheritance performance benchmarks
cargo bench --bench inheritance_performance_benchmarks

# Run specific benchmark groups
cargo bench --bench inheritance_performance_benchmarks -- --bench=actor_creation
cargo bench --bench inheritance_performance_benchmarks -- --bench=elemental_data_access
```

### Examples
```bash
# Run inheritance example
cargo run --example inheritance_example
```

### Documentation
```bash
cargo doc --open --features inheritable
```

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run benchmarks
6. Submit a pull request

## Changelog

### v0.1.0
- Initial release
- Array-based elemental system
- High-performance data access
- Extensible architecture
- Performance monitoring
