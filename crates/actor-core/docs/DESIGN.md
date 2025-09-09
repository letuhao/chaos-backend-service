# Actor Core Design Document

## Overview

Actor Core is a high-performance character stat aggregation and management system designed for the Chaos World MMORPG. It provides a flexible, extensible framework for managing character statistics, stat modifications, and character progression.

## Design Principles

### 1. Performance First
- Zero-copy operations where possible
- Efficient memory management
- SIMD optimizations for mathematical operations
- Comprehensive caching system

### 2. Flexibility
- Modular subsystem architecture
- Configurable processing pipelines
- Extensible bucket system
- Feature flags for optional functionality

### 3. Reliability
- Comprehensive error handling
- Input validation
- Deterministic processing order
- Extensive testing coverage

### 4. Maintainability
- Clear separation of concerns
- Well-documented APIs
- Consistent coding patterns
- Modular design

## Architecture

### Core Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│     Actor       │    │   Contribution  │    │      Caps       │
│                 │    │                 │    │                 │
│ - Character ID  │    │ - Dimension     │    │ - Min/Max       │
│ - Stats         │    │ - Bucket Type   │    │ - Constraints   │
│ - Buffs         │    │ - Value         │    │ - Validation    │
│ - Subsystems    │    │ - Source        │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Aggregator    │    │ Bucket Processor│    │  Caps Provider  │
│                 │    │                 │    │                 │
│ - Stat Resolution│    │ - Ordering      │    │ - Layer Management│
│ - Caching       │    │ - Processing    │    │ - Policy        │
│ - Metrics       │    │ - Clamping      │    │ - Validation    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│     Snapshot    │    │   Subsystem     │    │   Registry      │
│                 │    │                 │    │                 │
│ - Final State   │    │ - Processing    │    │ - Registration  │
│ - Metadata      │    │ - Contributions │    │ - Management    │
│ - Timestamps    │    │ - Priority      │    │ - Validation    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Data Flow

1. **Input**: Actor with stats, buffs, and subsystems
2. **Processing**: Contributions are collected from subsystems
3. **Ordering**: Contributions are ordered by bucket type
4. **Aggregation**: Contributions are processed in order
5. **Capping**: Values are clamped to caps constraints
6. **Output**: Final snapshot with aggregated stats

## Bucket System

The bucket system determines how contributions are processed:

### Core Buckets

- **Flat**: Additive contributions (equipment bonuses)
- **Mult**: Multiplicative contributions (percentage bonuses)
- **PostAdd**: Post-addition contributions (final adjustments)
- **Override**: Override contributions (replaces previous values)

### Processing Order

1. Flat contributions (additive)
2. Mult contributions (multiplicative)
3. PostAdd contributions (final adjustments)
4. Override contributions (replacements)

### Example

```rust
// Initial value: 100
// Flat contribution: +10 (equipment)
// Mult contribution: *1.2 (buff)
// PostAdd contribution: +5 (talent)
// Override contribution: 150 (special ability)

// Processing:
// 1. Flat: 100 + 10 = 110
// 2. Mult: 110 * 1.2 = 132
// 3. PostAdd: 132 + 5 = 137
// 4. Override: 150 (replaces previous)
// Final value: 150
```

## Caps System

The caps system provides constraints on stat values:

### Cap Modes

- **Baseline**: Base caps for the stat
- **Additive**: Additional caps that stack
- **HardMax**: Hard maximum that cannot be exceeded
- **SoftMax**: Soft maximum that can be exceeded with penalties

### Layer System

Caps are organized into layers with priorities:

1. **Base Layer**: Fundamental caps
2. **Equipment Layer**: Equipment-based caps
3. **Buff Layer**: Buff-based caps
4. **Talent Layer**: Talent-based caps

### Across-Layer Policy

- **Strict**: All layers must be satisfied
- **Lenient**: Only the highest priority layer must be satisfied
- **Custom**: Custom policy implementation

## Subsystem Architecture

Subsystems are modular components that contribute to stat processing:

### Subsystem Interface

```rust
pub trait Subsystem: Send + Sync {
    fn system_id(&self) -> &str;
    fn priority(&self) -> i64;
    fn contribute(&self, actor: &Actor) -> Pin<Box<dyn Future<Output = Result<SubsystemOutput, ActorCoreError>> + Send + '_>>;
}
```

### Built-in Subsystems

- **CombatSubsystem**: Handles combat-related stats
- **MagicSubsystem**: Handles magic-related stats
- **SocialSubsystem**: Handles social-related stats
- **CraftingSubsystem**: Handles crafting-related stats

### Custom Subsystems

Developers can create custom subsystems by implementing the `Subsystem` trait:

```rust
struct CustomSubsystem {
    system_id: String,
    priority: i64,
}

#[async_trait]
impl Subsystem for CustomSubsystem {
    // Implementation
}
```

## Caching System

The caching system provides performance optimization:

### Cache Types

- **InMemoryCache**: Fast in-memory caching
- **MultiLayerCache**: Layered caching with different TTLs
- **PersistentCache**: Persistent caching to disk

### Cache Keys

Cache keys are generated based on:
- Actor ID
- Actor version
- Subsystem IDs
- Configuration hash

### Cache Invalidation

- **Time-based**: TTL expiration
- **Version-based**: Actor version changes
- **Manual**: Explicit invalidation

## Configuration System

The configuration system supports YAML and JSON formats:

### Cap Layers Configuration

```yaml
cap_layers:
  - name: base
    priority: 100
    cap_mode: BASELINE
  - name: equipment
    priority: 200
    cap_mode: ADDITIVE
  - name: buffs
    priority: 300
    cap_mode: HARD_MAX

across_layer_policy: STRICT
```

### Combiner Configuration

```yaml
combiner_rules:
  - name: attack
    bucket_order: [Flat, Mult, PostAdd, Override]
    clamp: true
  - name: defense
    bucket_order: [Flat, Mult, PostAdd, Override]
    clamp: true
```

## Error Handling

The system uses a comprehensive error handling strategy:

### Error Types

- **ValidationError**: Input validation failures
- **ProcessingError**: Processing failures
- **ConfigurationError**: Configuration errors
- **CacheError**: Cache operation failures
- **RegistryError**: Registry operation failures
- **IoError**: I/O operation failures
- **SerdeError**: Serialization/deserialization failures

### Error Propagation

Errors are propagated through the `ActorCoreResult<T>` type:

```rust
pub type ActorCoreResult<T> = Result<T, ActorCoreError>;
```

### Error Recovery

The system provides error recovery mechanisms:

- **Graceful degradation**: Continue processing with partial results
- **Fallback values**: Use default values when possible
- **Retry mechanisms**: Retry failed operations
- **Logging**: Comprehensive error logging

## Performance Considerations

### Memory Management

- **Arc<T>**: Shared ownership for large objects
- **Box<T>**: Owned heap allocation for trait objects
- **Vec<T>**: Dynamic arrays for collections
- **HashMap<K, V>**: Hash maps for key-value storage

### Async Operations

- **async_trait**: Async trait methods
- **tokio**: Async runtime
- **Pin<Box<dyn Future>>**: Pinned futures for trait methods

### SIMD Optimizations

The system includes SIMD optimizations for mathematical operations:

- **Vectorized operations**: Process multiple values simultaneously
- **CPU feature detection**: Use available SIMD instructions
- **Fallback implementations**: Non-SIMD fallbacks for compatibility

### Caching Strategy

- **L1 Cache**: Hot data in memory
- **L2 Cache**: Warm data in memory
- **L3 Cache**: Cold data on disk
- **Cache warming**: Preload frequently accessed data

## Testing Strategy

### Test Categories

1. **Unit Tests**: Individual component testing
2. **Integration Tests**: End-to-end functionality testing
3. **Property Tests**: Mathematical property validation
4. **Edge Case Tests**: Boundary condition testing
5. **Performance Tests**: Performance benchmark validation

### Test Coverage

- **Code Coverage**: 100% line coverage
- **Branch Coverage**: 100% branch coverage
- **Function Coverage**: 100% function coverage
- **Integration Coverage**: All integration paths tested

### Test Data

- **Synthetic Data**: Generated test data
- **Real Data**: Production-like data
- **Edge Cases**: Boundary conditions
- **Stress Tests**: High-load scenarios

## Security Considerations

### Input Validation

- **Type Validation**: Ensure correct types
- **Range Validation**: Ensure values are within expected ranges
- **Format Validation**: Ensure correct formats
- **Sanitization**: Clean and sanitize inputs

### Access Control

- **Permission Checks**: Verify access permissions
- **Rate Limiting**: Prevent abuse
- **Audit Logging**: Log all operations
- **Encryption**: Encrypt sensitive data

### Error Information

- **Error Messages**: Avoid exposing sensitive information
- **Stack Traces**: Limit stack trace exposure
- **Logging**: Secure logging practices

## Future Enhancements

### Planned Features

1. **Distributed Caching**: Redis/Memcached support
2. **Database Integration**: PostgreSQL/MongoDB support
3. **GraphQL API**: GraphQL endpoint
4. **WebSocket Support**: Real-time updates
5. **Metrics Dashboard**: Performance monitoring

### Performance Improvements

1. **JIT Compilation**: Just-in-time compilation for hot paths
2. **GPU Acceleration**: GPU-based processing
3. **Streaming Processing**: Stream-based processing
4. **Parallel Processing**: Multi-threaded processing

### Extensibility

1. **Plugin System**: Dynamic plugin loading
2. **Scripting Support**: Lua/JavaScript scripting
3. **Custom Buckets**: User-defined bucket types
4. **Custom Operators**: User-defined operators

## Conclusion

Actor Core provides a robust, high-performance foundation for character stat management in the Chaos World MMORPG. Its modular design, comprehensive testing, and performance optimizations make it suitable for production use while maintaining flexibility for future enhancements.

The system's design principles of performance, flexibility, reliability, and maintainability ensure that it can scale with the game's growth while providing a solid foundation for character progression systems.
