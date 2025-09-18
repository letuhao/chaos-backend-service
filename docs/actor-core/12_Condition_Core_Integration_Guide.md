# Actor Core + Condition Core Integration Guide

## Overview

This document provides a comprehensive guide for integrating Actor Core with Condition Core, enabling powerful conditional logic for character stats, subsystems, and behaviors.

## Integration Levels

### 1. Simple Integration (Recommended for most use cases)

Simple integration provides basic condition evaluation capabilities with minimal complexity.

#### Features
- ✅ Basic condition evaluation (health, mana, resources, status effects)
- ✅ Condition chains (AND/OR logic)
- ✅ Data providers for Actor, Resource, and Category data
- ✅ Easy-to-use API
- ✅ Fast compilation and runtime performance

#### Usage Example

```rust
use actor_core::prelude::*;
use condition_core::prelude::*;

// Create mock data providers
let mut data_registry = DataProviderRegistry::new();
data_registry.register_actor_provider(Box::new(MockActorDataProvider));
data_registry.register_resource_provider(Box::new(MockResourceDataProvider));
data_registry.register_category_provider(Box::new(MockCategoryDataProvider));

// Create condition resolver
let resolver = ConditionResolver::new(data_registry);

// Create condition context
let context = ConditionContext {
    target: ActorTarget { id: "player_1".to_string() },
    world_id: "test_world".to_string(),
    current_time: SystemTime::now(),
    current_weather: WeatherType::Clear,
    world_state: WorldState {
        time_of_day: 12.0,
        season: "spring".to_string(),
        temperature: 20.0,
        humidity: 0.5,
    },
};

// Evaluate conditions
let health_condition = ConditionConfig {
    condition_id: "has_sufficient_health".to_string(),
    function_name: "get_actor_resource".to_string(),
    operator: ConditionOperator::GreaterThanOrEqual,
    value: ConditionValue::Float(50.0),
    parameters: vec![ConditionParameter::String("health".to_string())],
};

let result = resolver.resolve_condition(&health_condition, &context).await?;
println!("Has sufficient health: {}", result);
```

### 2. Complex Integration (Advanced features)

Complex integration provides full-featured conditional systems with advanced capabilities.

#### Features
- ✅ Conditional subsystems (dynamic activation based on conditions)
- ✅ Conditional modifiers (stat modifications based on conditions)
- ✅ Advanced caching (performance optimization)
- ✅ Event handling (condition-based event triggers)
- ✅ Full Actor Core integration

#### Usage Example

```rust
use actor_core::condition_integration::*;

// Create complex integration
let integration = ActorCoreWithConditions::new(
    aggregator,
    condition_resolver,
    cache,
);

// Resolve actor stats with conditions
let snapshot = integration.resolve_with_conditions(&actor).await?;

// Apply conditional subsystems
let subsystem_output = integration.apply_conditional_subsystems(&actor).await?;

// Apply conditional modifiers
let modifier_system = ConditionalModifierSystem::new(
    condition_resolver.clone(),
    modifier_registry,
    cache,
);
modifier_system.apply_modifiers(&actor, &mut snapshot).await?;
```

## Data Providers

### ActorDataProvider

Provides access to actor-specific data for condition evaluation.

```rust
pub trait ActorDataProvider: Send + Sync {
    async fn get_actor_resource(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn get_actor_stat(&self, stat_name: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn get_actor_derived_stat(&self, stat_name: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn get_actor_race(&self, actor_id: &str) -> ConditionResult<String>;
    async fn is_actor_in_combat(&self, actor_id: &str) -> ConditionResult<bool>;
    async fn has_actor_status_effects(&self, status_type: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_actor_status_effect_count(&self, status_type: &str, actor_id: &str) -> ConditionResult<i64>;
    async fn get_actor_status_effect_count_by_category(&self, status_type: &str, category: &str, actor_id: &str) -> ConditionResult<i64>;
}
```

### ResourceDataProvider

Provides access to resource data for condition evaluation.

```rust
pub trait ResourceDataProvider: Send + Sync {
    async fn get_resource_value(&self, resource_id: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn get_resource_max(&self, resource_id: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn get_resource_percentage(&self, resource_id: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn is_resource_empty(&self, resource_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn is_resource_below_threshold(&self, resource_id: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool>;
    async fn is_resource_above_threshold(&self, resource_id: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool>;
    async fn is_resource_below_percentage(&self, resource_id: &str, percentage: f64, actor_id: &str) -> ConditionResult<bool>;
    async fn is_resource_above_percentage(&self, resource_id: &str, percentage: f64, actor_id: &str) -> ConditionResult<bool>;
    async fn list_resources(&self) -> ConditionResult<Vec<String>>;
}
```

### CategoryDataProvider

Provides access to category data for condition evaluation.

```rust
pub trait CategoryDataProvider: Send + Sync {
    async fn has_category_item(&self, category: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_category_item_count(&self, category: &str, actor_id: &str) -> ConditionResult<i64>;
    async fn is_category_available(&self, category: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn is_category_blocked(&self, category: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn list_categories(&self) -> ConditionResult<Vec<String>>;
}
```

## Condition Types

### Resource Conditions

```yaml
# Health condition
condition_id: "has_sufficient_health"
function_name: "get_actor_resource"
operator: ">="
value: 50.0
parameters:
  - "health"

# Mana percentage condition
condition_id: "has_low_mana"
function_name: "is_resource_below_percentage"
operator: "=="
value: true
parameters:
  - "mana"
  - 25.0
```

### Status Effect Conditions

```yaml
# Status effect condition
condition_id: "has_buffs"
function_name: "has_actor_status_effects"
operator: "=="
value: true
parameters:
  - "buffs"

# Status effect count condition
condition_id: "has_multiple_debuffs"
function_name: "get_actor_status_effect_count"
operator: ">"
value: 2
parameters:
  - "debuffs"
```

### Category Conditions

```yaml
# Item category condition
condition_id: "has_weapon"
function_name: "has_category_item"
operator: "=="
value: true
parameters:
  - "weapon"

# Category availability condition
condition_id: "can_use_magic"
function_name: "is_category_available"
operator: "=="
value: true
parameters:
  - "magic"
```

## Condition Chains

### AND Logic

```yaml
chain_id: "can_cast_fireball"
logic: "And"
conditions:
  - condition_id: "has_mana"
    function_name: "get_actor_resource"
    operator: ">="
    value: 25.0
    parameters: ["mana"]
  - condition_id: "has_fireball_spell"
    function_name: "get_element_mastery"
    operator: ">"
    value: 100.0
    parameters: ["fire"]
```

### OR Logic

```yaml
chain_id: "can_heal"
logic: "Or"
conditions:
  - condition_id: "has_healing_potion"
    function_name: "has_category_item"
    operator: "=="
    value: true
    parameters: ["potion"]
  - condition_id: "has_healing_spell"
    function_name: "get_element_mastery"
    operator: ">"
    value: 50.0
    parameters: ["life"]
```

## Performance Considerations

### Caching

Both simple and complex integrations support caching for performance optimization:

```rust
// Simple integration - automatic caching
let resolver = ConditionResolver::new(data_registry);

// Complex integration - advanced caching
let condition_cache = ConditionCache::new(cache);
let cached_result = condition_cache.get_condition_result("condition_id", "actor_id").await;
```

### Error Handling

All integrations use proper error handling with `ConditionResult<T>`:

```rust
match resolver.resolve_condition(&condition, &context).await {
    Ok(result) => println!("Condition result: {}", result),
    Err(ConditionError::ConfigError { message }) => {
        eprintln!("Configuration error: {}", message);
    }
    Err(ConditionError::InvalidParameter { function_name, parameter }) => {
        eprintln!("Invalid parameter '{}' for function '{}'", parameter, function_name);
    }
    Err(e) => eprintln!("Other error: {:?}", e),
}
```

## Migration Guide

### From Simple to Complex Integration

1. **Add Complex Integration Dependencies**
   ```rust
   use actor_core::condition_integration::*;
   ```

2. **Create Complex Integration Instance**
   ```rust
   let integration = ActorCoreWithConditions::new(
       aggregator,
       condition_resolver,
       cache,
   );
   ```

3. **Use Advanced Features**
   ```rust
   // Conditional subsystems
   let subsystem_output = integration.apply_conditional_subsystems(&actor).await?;
   
   // Conditional modifiers
   let modifier_system = ConditionalModifierSystem::new(
       condition_resolver.clone(),
       modifier_registry,
       cache,
   );
   ```

## Best Practices

### 1. Use Simple Integration for Basic Needs
- Basic condition evaluation
- Simple condition chains
- Performance-critical applications

### 2. Use Complex Integration for Advanced Features
- Dynamic subsystem activation
- Complex stat modifications
- Event-driven systems

### 3. Proper Error Handling
- Always handle `ConditionResult<T>` properly
- Use specific error types for different scenarios
- Log errors appropriately

### 4. Performance Optimization
- Use caching for frequently evaluated conditions
- Batch condition evaluations when possible
- Monitor performance metrics

### 5. Configuration Management
- Use YAML configuration files for conditions
- Version control condition configurations
- Test condition configurations thoroughly

## Examples

See the following example files:
- `examples/simple_condition_integration.rs` - Simple integration example
- `examples/condition_integration_example.rs` - Complex integration example
- `examples/simple_condition_integration.rs` - Basic usage example

## Troubleshooting

### Common Issues

1. **Compilation Errors**
   - Ensure all dependencies are properly configured
   - Check trait implementations
   - Verify error handling

2. **Runtime Errors**
   - Check data provider implementations
   - Verify condition configurations
   - Monitor error logs

3. **Performance Issues**
   - Enable caching
   - Optimize condition evaluation order
   - Monitor memory usage

### Debug Tips

1. **Enable Debug Logging**
   ```rust
   env_logger::init();
   ```

2. **Use Condition Builder for Dynamic Conditions**
   ```rust
   let condition = ConditionBuilder::new()
       .function("get_actor_resource")
       .operator(ConditionOperator::GreaterThanOrEqual)
       .value(50.0)
       .parameter("health")
       .build()?;
   ```

3. **Test Conditions Individually**
   ```rust
   let result = resolver.resolve_condition(&condition, &context).await?;
   println!("Condition result: {}", result);
   ```

## Conclusion

Actor Core + Condition Core integration provides powerful conditional logic capabilities for game systems. Choose the appropriate integration level based on your needs:

- **Simple Integration** for basic condition evaluation
- **Complex Integration** for advanced features

Both integrations are production-ready and can be used together for maximum flexibility.
