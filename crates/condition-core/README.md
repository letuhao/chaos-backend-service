# Condition Core

A simple and efficient condition resolution system for Chaos World that provides a unified interface for evaluating conditions based on configuration files.

## Overview

Condition Core is designed to be a lightweight, high-performance condition resolver that can be used by other systems in the Chaos World ecosystem. It provides a clean interface for evaluating conditions defined in YAML configuration files.

## Features

- **Simple Interface**: Easy-to-use trait-based API for condition resolution
- **YAML Configuration**: Define conditions using human-readable YAML files
- **Multiple Condition Types**: Support for single conditions, multiple conditions, and condition chains
- **Logical Operators**: Support for AND, OR, NOT, and XOR logic in condition chains
- **Built-in Functions**: 20+ built-in condition functions for common game logic
- **Element Core Integration**: 20+ element condition functions for standardized element logic
- **Status Core Integration**: 20+ status condition functions for standardized status logic
- **Type Safety**: Strong typing with compile-time safety
- **Async Support**: Full async/await support for high-performance applications
- **Comprehensive Testing**: Unit tests, integration tests, and benchmarks included

## Quick Start

### Basic Usage

```rust
use condition_core::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a condition resolver
    let resolver = DefaultConditionResolver::new();

    // Create a test context
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

    // Define a condition
    let condition = ConditionConfig {
        condition_id: "has_sufficient_health".to_string(),
        function_name: "get_actor_health".to_string(),
        operator: ConditionOperator::GreaterThanOrEqual,
        value: ConditionValue::Float(50.0),
        parameters: vec![],
    };

    // Resolve the condition
    let result = resolver.resolve_condition(&condition, &context).await?;
    println!("Has sufficient health: {}", result);

    Ok(())
}
```

### YAML Configuration

```yaml
# Single condition
condition_id: "check_health"
function_name: "get_actor_health"
operator: GreaterThan
value: 75.0
parameters: []

# Multiple conditions
- condition_id: "has_mana"
  function_name: "get_actor_mana"
  operator: GreaterThanOrEqual
  value: 30.0
  parameters: []

- condition_id: "is_raining"
  function_name: "is_raining"
  operator: Equal
  value: true
  parameters: []

# Condition chain
chain_id: "weather_spell_conditions"
logic: And
conditions:
  - condition_id: "is_raining"
    function_name: "is_raining"
    operator: Equal
    value: true
    parameters: []

  - condition_id: "has_water_spell"
    function_name: "has_spell"
    operator: Equal
    value: true
    parameters:
      - String: "water_bolt"
```

## Built-in Functions

### Actor Functions
- `get_actor_health()` - Get actor's current health
- `get_actor_mana()` - Get actor's current mana
- `get_actor_stamina()` - Get actor's current stamina
- `get_actor_level()` - Get actor's level
- `is_actor_in_combat()` - Check if actor is in combat

### Item Functions
- `has_item(item_id)` - Check if actor has specific item
- `get_item_count(item_id)` - Get count of specific item

### Location Functions
- `is_in_area(area_id)` - Check if actor is in specific area
- `get_distance_to(target_id)` - Get distance to target
- `is_indoors()` - Check if actor is indoors

### Time Functions
- `is_day()` - Check if it's day time
- `is_night()` - Check if it's night time

### Weather Functions
- `is_raining()` - Check if it's raining
- `is_snowing()` - Check if it's snowing

### Magic Functions
- `has_spell(spell_id)` - Check if actor has specific spell
- `get_spell_level(spell_id)` - Get level of specific spell
- `get_mana_cost(spell_id)` - Get mana cost of specific spell

### Relationship Functions
- `is_ally(target_id)` - Check if target is ally
- `is_enemy(target_id)` - Check if target is enemy

## Logical Operators

- `Equal` - Values are equal
- `NotEqual` - Values are not equal
- `GreaterThan` - First value is greater than second
- `LessThan` - First value is less than second
- `GreaterThanOrEqual` - First value is greater than or equal to second
- `LessThanOrEqual` - First value is less than or equal to second
- `Contains` - First value contains second value
- `NotContains` - First value does not contain second value
- `In` - First value is in second value (list)
- `NotIn` - First value is not in second value (list)

## Chain Logic

- `And` - All conditions must be true
- `Or` - At least one condition must be true
- `Not` - Condition must be false (single condition only)
- `Xor` - Exactly one condition must be true

## Element Core Integration

Condition Core now includes comprehensive integration with Element Core, providing 20+ standardized element condition functions:

### Element Mastery Functions
- `get_element_mastery` - Get element mastery level
- `get_element_resistance` - Get element resistance value
- `has_element_affinity` - Check if actor has element affinity
- `has_element_weakness` - Check if actor has element weakness

### Element Interaction Functions
- `get_element_interaction` - Get element interaction type
- `is_element_same_category` - Check if elements are in same category
- `is_element_generating` - Check if elements are in generating relationship
- `is_element_overcoming` - Check if elements are in overcoming relationship
- `is_element_neutral` - Check if elements are neutral

### Element Status Functions
- `has_element_status_effect` - Check if actor has element status effect
- `get_element_status_effect_count` - Get element status effect count
- `is_element_status_effect_active` - Check if element status effect is active

### Element Resource Functions
- `has_element_resource` - Check if actor has element resource
- `get_element_resource_value` - Get element resource value
- `is_element_resource_below_threshold` - Check if element resource is below threshold
- `is_element_resource_above_threshold` - Check if element resource is above threshold

### Hybrid Element Functions
- `has_hybrid_element` - Check if actor has hybrid element
- `is_hybrid_element_activated` - Check if hybrid element is activated
- `get_hybrid_element_parents` - Get hybrid element parents

### Element Derived Stats Functions
- `get_element_derived_stat` - Get element derived stat value
- `has_element_derived_stat` - Check if actor has element derived stat

## Status Core Integration

Condition Core now includes comprehensive integration with Status Core, providing 20+ standardized status condition functions:

### Status Effect Functions
- `has_status_effect` - Check if actor has specific status effect
- `get_status_effect_count` - Get count of specific status effect
- `get_status_effect_magnitude` - Get magnitude of specific status effect
- `is_status_effect_active` - Check if status effect is active
- `is_status_effect_expired` - Check if status effect is expired

### Status Immunity Functions
- `has_status_immunity` - Check if actor has immunity to specific effect
- `get_status_immunity_count` - Get count of specific immunity
- `is_status_immunity_active` - Check if immunity is active

### Status Category Functions
- `has_status_category` - Check if actor has effects in specific category
- `get_status_category_count` - Get count of effects in specific category

### Status Interaction Functions
- `is_status_effect_stackable` - Check if effect can be stacked
- `can_status_effect_stack` - Check if effect can stack on actor
- `get_status_effect_interaction` - Get interaction between two effects
- `get_status_effect_priority` - Get priority of status effect

### Status Movement Functions
- `has_status_movement_restriction` - Check if actor has movement restriction
- `get_status_movement_restriction` - Get magnitude of movement restriction

### Status Visual/Audio Functions
- `has_status_visual_effect` - Check if actor has visual effect
- `has_status_audio_effect` - Check if actor has audio effect

### Status Properties Functions
- `has_status_effect_property` - Check if effect has specific property

## Examples

See the `examples/` directory for more detailed usage examples:

- `basic_usage.rs` - Basic condition resolution
- `yaml_config.rs` - Loading conditions from YAML files
- `element_conditions.rs` - Element Core integration examples
- `status_conditions.rs` - Status Core integration examples

## Testing

Run the test suite:

```bash
cargo test
```

Run benchmarks:

```bash
cargo bench
```

## Integration with Other Systems

Condition Core is designed to integrate seamlessly with other Chaos World systems:

- **Action Core**: Validate action execution conditions
- **Status Core**: Check status application conditions
- **Element Core**: Evaluate element interaction conditions
- **Resource Exhaustion**: Check resource availability conditions

## Performance

Condition Core is optimized for high-performance applications:

- Async/await support for non-blocking operations
- Efficient condition evaluation with minimal allocations
- Built-in benchmarking for performance monitoring
- Type-safe operations with compile-time optimizations

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
