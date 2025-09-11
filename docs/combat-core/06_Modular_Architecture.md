# Modular Architecture Design

## Overview

The Modular Architecture document outlines the file structure and organization of the Flexible Action System. This architecture emphasizes separation of concerns, modularity, and extensibility while maintaining high performance for MMORPG combat scenarios.

## File Structure Overview

```
combat-core/
├── action_definition.rs          # Core action structure and metadata
├── event_trigger.rs              # Event trigger conditions and logic
├── status_definition.rs          # Status effects (treated as actors)
├── attack_range.rs               # Targeting, range, and AOE definitions
├── projectile_definition.rs      # Projectiles (treated as actors)
├── environment_interactions.rs   # Environmental effects and interactions
├── damage_system.rs              # Damage type generation with unique keywords
├── power_level_system.rs         # Actor power calculation for AI decisions
├── combat_calculator.rs          # Core damage application logic
├── combat_validator.rs           # Action validation subsystem
├── combat_events.rs              # Event system subsystem
└── constants.rs                  # Global constants (MAX_* bounds)
```

## Core Files (Original 6)

### 1. action_definition.rs
**Purpose**: Core action structure and metadata definitions

**Key Components**:
- `Action` struct with all action properties
- `ActionMetadata` for UI and display information
- `ActionScaling` for level and stat scaling
- `ResourceConsumption` and `ResourceRegeneration`
- `ActionCooldown` and `ActionPrerequisites`

**Responsibilities**:
- Define the complete structure of combat actions
- Handle resource consumption and regeneration
- Manage action metadata and scaling
- Define prerequisites and cooldowns

### 2. event_trigger.rs
**Purpose**: Event trigger conditions and logic

**Key Components**:
- `TriggerCondition` for individual conditions
- `EventChain` for complex trigger sequences
- `ConditionalLogic` with AND/OR/XOR operations
- `TriggerEvent` for event definitions

**Responsibilities**:
- Define when actions can be triggered
- Handle complex conditional logic
- Manage event chains and sequences
- Provide flexible trigger conditions

### 3. status_definition.rs
**Purpose**: Status effects treated as actors

**Key Components**:
- `StatusActor` inheriting from Actor
- `StatusBehavior` for status effect behavior
- `StatusInteraction` for status effect interactions
- `StatusData` for status-specific information

**Responsibilities**:
- Define status effects as actors
- Handle status effect behavior and interactions
- Manage status effect stacking and decay
- Provide status effect lifecycle management

### 4. attack_range.rs
**Purpose**: Targeting, range, and AOE definitions

**Key Components**:
- `AttackRange` for basic range definitions
- `AreaOfEffect` for AOE configurations
- `MultiTarget` for multi-target selection
- `TargetSelection` and `TargetPriority`

**Responsibilities**:
- Define targeting mechanics
- Handle AOE and multi-target configurations
- Manage line of sight and range calculations
- Provide target selection algorithms

### 5. projectile_definition.rs
**Purpose**: Projectiles treated as actors

**Key Components**:
- `ProjectileActor` inheriting from Actor
- `ProjectileData` for projectile properties
- `ProjectileBehavior` for movement and collision
- `CollisionRule` for collision handling

**Responsibilities**:
- Define projectiles as actors
- Handle projectile movement and physics
- Manage collision detection and resolution
- Provide projectile lifecycle management

### 6. environment_interactions.rs
**Purpose**: Environmental effects and interactions

**Key Components**:
- `EnvironmentActor` inheriting from Actor
- `EnvironmentData` for environmental properties
- `EnvironmentBehavior` for interaction rules
- `WeatherInteraction` and `TerrainModification`

**Responsibilities**:
- Define environmental effects as actors
- Handle weather and terrain interactions
- Manage environmental modification
- Provide environmental effect lifecycle

## Supporting Systems (Additional 5)

### 7. damage_system.rs
**Purpose**: Damage type generation with unique keywords

**Key Components**:
- `DamageTypeGenerator` for generating damage types
- `DamageType` with unique keywords
- `DamageCategory` for damage classification
- `DamageProperties` for damage behavior

**Responsibilities**:
- Generate damage types with unique keywords
- Handle damage type interactions
- Manage damage categories and properties
- Provide damage type subsystems

**Integration with Other Systems**:
- Used by `action_definition.rs` for damage types
- Used by `combat_calculator.rs` for damage calculations
- Used by `power_level_system.rs` for power calculations

### 8. power_level_system.rs
**Purpose**: Actor power calculation for AI decisions

**Key Components**:
- `PowerLevelSystem` for power calculation
- `PowerComponent` for individual power sources
- `PowerAggregation` for combining power sources
- `CombatAI` for AI decision making

**Responsibilities**:
- Calculate actor power levels
- Aggregate power from multiple sources
- Provide AI decision making based on power
- Handle personality traits and relationships

**Integration with Other Systems**:
- Used by `combat_ai.rs` for targeting decisions
- Used by `combat_calculator.rs` for damage scaling
- Used by `action_definition.rs` for action prerequisites

### 9. combat_calculator.rs
**Purpose**: Core damage application logic

**Key Components**:
- `CombatCalculator` for main calculation engine
- `DamageCalculator` for damage calculations
- `ShieldCalculator` for shield processing
- `HotPathCalculator` for performance optimization

**Responsibilities**:
- Calculate damage application
- Handle shield and resistance calculations
- Provide performance optimization
- Manage batch processing

**Integration with Other Systems**:
- Uses `damage_system.rs` for damage types
- Uses `power_level_system.rs` for power calculations
- Used by `combat_validator.rs` for validation
- Used by `combat_events.rs` for event generation

### 10. combat_validator.rs
**Purpose**: Action validation subsystem

**Key Components**:
- `CombatValidator` for validation engine
- `ValidationRule` for individual rules
- `PrerequisiteChecker` for prerequisite validation
- `ResourceValidator` for resource validation

**Responsibilities**:
- Validate actions before execution
- Check prerequisites and requirements
- Validate resource availability
- Provide validation error messages

**Integration with Other Systems**:
- Uses `action_definition.rs` for action structure
- Uses `combat_calculator.rs` for calculations
- Used by `combat_events.rs` for validation events
- Used by `power_level_system.rs` for power requirements

### 11. combat_events.rs
**Purpose**: Event system subsystem

**Key Components**:
- `CombatEventSystem` for event management
- `EventType` for event definitions
- `EventQueue` for event processing
- `EventThrottler` for performance management

**Responsibilities**:
- Manage combat event system
- Handle event queuing and processing
- Provide event throttling and coalescing
- Manage event handlers and subscriptions

**Integration with Other Systems**:
- Used by all other systems for event communication
- Uses `combat_validator.rs` for validation events
- Uses `combat_calculator.rs` for calculation events
- Provides event system for the entire combat core

## Architecture Benefits

### 1. **Separation of Concerns**
Each file has a clear, focused responsibility:
- Action definition is separate from validation
- Damage calculation is separate from event handling
- Status effects are separate from projectiles

### 2. **Modularity**
- Files can be developed independently
- Easy to add new features without affecting others
- Clear interfaces between modules

### 3. **Extensibility**
- New action types can be added through data
- New damage types can be added through subsystems
- New status effects can be added through actor definitions

### 4. **Performance**
- Hot path optimization in combat calculator
- Efficient data structures for each concern
- Minimal cross-module dependencies
- Centralized constants: `constants.rs` defines `MAX_SHIELDS_PER_ACTOR`, `MAX_RESOURCES_PER_ACTOR`, `MAX_DAMAGE_TYPES`, `MAX_RESOURCE_TYPES` referenced by all modules

### 5. **Maintainability**
- Clear file organization
- Focused responsibilities
- Easy to test and debug

## Integration Patterns

### 1. **Actor-Based Design**
- Status effects, projectiles, and environmental effects are all actors
- Consistent interface and behavior
- Easy to extend and modify

### 2. **Event-Driven Architecture**
- All systems communicate through events
- Loose coupling between modules
- Easy to add new event handlers

### 3. **Data-Driven Configuration**
- Actions defined as data structures
- Easy to modify without code changes
- Support for runtime configuration

### 4. **Subsystem Pattern**
- Damage types, power levels, and other systems as subsystems
- Easy to add new subsystems
- Consistent interface and behavior

## Performance Considerations

### 1. **Hot Path Optimization**
- Combat calculator optimized for performance
- Minimal allocations in hot paths
- Efficient data structures

### 2. **Batch Processing**
- Multiple actions processed together
- Reduced overhead per action
- Better cache utilization

### 3. **Caching Strategy**
- Frequently used data cached
- Smart cache invalidation
- Multi-level caching

### 4. **Memory Management**
- Object pooling for frequently created objects
- Efficient data structures
- Minimal memory allocations

## Testing Strategy

### 1. **Unit Testing**
- Each file tested independently
- Mock dependencies for isolation
- Comprehensive test coverage

### 2. **Integration Testing**
- Test interactions between modules
- End-to-end combat scenarios
- Performance testing

### 3. **Load Testing**
- High-load combat scenarios
- Memory usage testing
- Performance benchmarking

## Future Extensions

### 1. **New Action Types**
- Easy to add through data definitions
- No code changes required
- Runtime configuration support

### 2. **New Damage Types**
- Add through damage subsystems
- Consistent interface
- Easy integration

### 3. **New Status Effects**
- Add through status actor definitions
- Consistent behavior
- Easy to extend

### 4. **New Environmental Effects**
- Add through environment actor definitions
- Consistent interface
- Easy integration

## Conclusion

The modular architecture provides a solid foundation for a flexible, extensible combat system. The clear separation of concerns, actor-based design, and event-driven architecture make it easy to develop, test, and maintain while providing high performance for MMORPG scenarios.

The architecture supports both current requirements and future extensions, making it a robust solution for complex combat systems.
