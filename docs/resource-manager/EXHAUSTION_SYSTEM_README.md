# Resource Exhaustion System

## Overview

The Resource Exhaustion System is a comprehensive subsystem for the Chaos World MMORPG that defines breakpoints and effects that apply when an actor's resources fall to critical thresholds. It operates both globally (outside of combat) and inside combat, with deterministic evaluation and event publication.

## Features

- **Deterministic Evaluation**: Resources are evaluated in a consistent order to ensure reproducible results
- **Hysteresis Support**: Prevents rapid flapping around thresholds with configurable enter/exit conditions
- **Event Coalescing**: Reduces event spam by coalescing repeated transitions within a configurable window
- **Deep Configuration Merging**: Supports global, area, and PvP template overrides with proper precedence
- **Performance Optimized**: Includes caching, fast-path evaluation, and performance monitoring
- **Comprehensive Testing**: Golden vector tests and comprehensive test suites

## Architecture

### Core Components

1. **ResourceExhaustionSubsystem**: Main subsystem that handles exhaustion evaluation and effect application
2. **ExhaustionEngine**: Core evaluation engine with deterministic ordering
3. **ExhaustionConfigLoader**: Handles configuration loading and deep merging
4. **ExhaustionEventPublisher**: Publishes exhaustion events with idempotency
5. **OptimizedExhaustionEngine**: Performance-optimized version with caching

### Configuration System

The system supports a hierarchical configuration system:

```
Global Config (base)
    ↓
Area Overrides (area-specific)
    ↓
PvP Template Overrides (PvP-specific)
    ↓
Final Merged Config
```

## Configuration

### Basic Configuration Structure

```yaml
version: 1
hysteresis_default: 0.02
events:
  coalesce_window_ms: 200
priorities:
  categories: ["health", "lifeforce", "lifespan", "qi", "spiritual", "mana", "stamina", "other"]
archetypes:
  mage:
    mana:
      thresholds:
        - id: low_mana
          order: 10
          enter_percent_lte: 0.10
          exit_percent_gte: 0.12
          effects:
            - type: disable_tags
              values: ["shield_activation", "buff_activation"]
            - type: damage_multiplier
              categories: ["magical", "elemental"]
              modifier: -0.40
```

### Threshold Configuration

Each threshold can have:

- **Enter Conditions**: `enter_percent_lte` or `enter_value_eq`
- **Exit Conditions**: `exit_percent_gte` or `exit_value_ge` (optional, defaults to enter + hysteresis)
- **Order**: Processing order (lower numbers processed first)
- **Effects**: List of effects to apply when threshold is entered

### Effect Types

The system supports various effect types:

- `disable_tags`: Disable action/system tags
- `disable_cost_type`: Prevent actions using specified resource types
- `damage_multiplier`: Modify outgoing damage by category
- `incoming_multiplier`: Modify incoming damage by category
- `cast_time_modifier`: Modify cast time
- `move_speed_modifier`: Modify movement speed
- `break_active_shields`: Break specific shield types
- `set_flag`: Set actor flags
- `action_lockout`: Lock specific action families
- `stagger_susceptibility`: Set stagger susceptibility level
- `taunt_effectiveness_modifier`: Modify taunt effectiveness
- `regen_modifier`: Modify resource regeneration

## Usage

### Basic Usage

```rust
use actor_core::subsystems::{
    ResourceExhaustionSubsystem, ExhaustionConfig, InMemoryEventPublisher
};

// Load configuration
let config = ExhaustionConfig::load_from_file("config.yaml").await?;

// Create event publisher
let event_publisher = Arc::new(InMemoryEventPublisher::new());

// Create subsystem
let subsystem = ResourceExhaustionSubsystem::new(config, event_publisher);

// Evaluate exhaustion for an actor
let transitions = subsystem.evaluate(&actor, &snapshot).await?;

// Apply effects
subsystem.apply_effects(&actor.id.to_string(), &transitions).await?;
```

### Configuration Loading with Overrides

```rust
use actor_core::subsystems::ExhaustionConfigLoader;

let mut loader = ExhaustionConfigLoader::new();

// Load global configuration
loader.load_global_config("global_config.yaml").await?;

// Load area override
loader.load_area_override("dungeon_01", "area_overrides/dungeon_01.yaml").await?;

// Load PvP template override
loader.load_pvp_override("arena_template", "pvp_templates/arena.yaml").await?;

// Resolve final configuration
let merged_config = loader.resolve_config(Some("dungeon_01"), Some("arena_template"))?;
```

### Performance Optimization

```rust
use actor_core::subsystems::OptimizedExhaustionEngine;

// Create optimized engine
let optimized_engine = OptimizedExhaustionEngine::new(config);

// Evaluate with performance optimizations
let transitions = optimized_engine.evaluate_optimized(&actor, &snapshot).await?;

// Get performance statistics
let stats = optimized_engine.get_stats().await;
println!("Average evaluation time: {:.2}μs", stats.avg_evaluation_time_us);
```

## Testing

### Unit Tests

The system includes comprehensive unit tests:

```bash
cargo test exhaustion_system_tests
cargo test exhaustion_golden_vector_tests
```

### Golden Vector Tests

Golden vector tests validate deterministic behavior:

```bash
cargo test test_case05_exhaustion_hysteresis_and_coalescing
cargo test test_case06_simultaneous_exhaustion_precedence
```

### CLI Testing Tool

A CLI tool is provided for testing and debugging:

```bash
# Validate configuration
cargo run --example exhaustion_cli -- validate config.yaml

# Test specific scenario
cargo run --example exhaustion_cli -- test config.yaml mage 50.0 1000.0 500.0 1000.0

# Run golden vector test
cargo run --example exhaustion_cli -- golden-vector config.yaml case05

# Debug configuration merging
cargo run --example exhaustion_cli -- debug-config global.yaml area.yaml pvp.yaml
```

## Performance

### Performance Targets

- **Evaluation Time**: ≤ 1-3 μs per actor per evaluation
- **Memory Usage**: Minimal memory footprint with efficient caching
- **Throughput**: High throughput for large numbers of actors

### Performance Monitoring

The system includes built-in performance monitoring:

```rust
use actor_core::subsystems::PerformanceMonitor;

let monitor = PerformanceMonitor::new(1000, 5000); // 1ms warning, 5ms error
monitor.record_evaluation(duration, fast_path).await;
```

### Benchmarking

Performance benchmarks are available:

```rust
use actor_core::subsystems::ExhaustionBenchmark;

let mut benchmark = ExhaustionBenchmark::new();
benchmark.add_config(BenchmarkConfig {
    name: "high_load".to_string(),
    actor_count: 1000,
    evaluations_per_actor: 100,
    resource_ranges: HashMap::new(),
});

benchmark.run_all(&engine).await?;
```

## Events

### Event Types

- `ResourceExhaustedEvent`: Published when a threshold is entered
- `ResourceRecoveredEvent`: Published when a threshold is exited

### Event Properties

- **Idempotency**: Events include idempotency keys to prevent duplicate processing
- **Coalescing**: Events within the coalescing window are merged
- **Timestamping**: All events include precise timestamps

### Event Publishers

Multiple event publisher implementations are available:

- `InMemoryEventPublisher`: For testing and debugging
- `LoggingEventPublisher`: For development and logging
- `NoOpEventPublisher`: For performance testing

## Integration

### Combat System Integration

The exhaustion system integrates with the combat system:

```rust
// In combat tick
let transitions = exhaustion_subsystem.evaluate(&actor, &snapshot).await?;
exhaustion_subsystem.apply_effects(&actor.id.to_string(), &transitions).await?;
```

### Resource Manager Integration

The system works with the resource manager to track resource changes:

```rust
// After resource consumption/regeneration
let transitions = exhaustion_subsystem.evaluate(&actor, &snapshot).await?;
```

## Error Handling

The system provides comprehensive error handling:

- `ExhaustionError`: Main error type for exhaustion-specific errors
- `ConfigLoaderError`: Configuration loading and validation errors
- Graceful degradation when configuration is invalid

## Future Enhancements

- **Real-time Configuration Updates**: Hot-reload configuration changes
- **Advanced Caching**: More sophisticated caching strategies
- **Metrics Integration**: Integration with metrics collection systems
- **Distributed Events**: Support for distributed event publishing

## Contributing

When contributing to the exhaustion system:

1. Follow the existing code style and patterns
2. Add comprehensive tests for new features
3. Update documentation for API changes
4. Consider performance implications
5. Ensure deterministic behavior

## License

This system is part of the Chaos World MMORPG project and follows the project's licensing terms.
