# Movement Restrictions Configuration

This directory contains configuration files for movement restrictions in the Status Core system.

## Overview

Movement restrictions define how status effects can limit or modify actor movement and actions. They provide a flexible system for implementing various movement-related status effects.

## Configuration Files

### Core Movement Restrictions

- **`movement_slowdown.yaml`** - Basic movement slowdown restriction
- **`movement_speedup.yaml`** - Basic movement speedup restriction
- **`movement_block.yaml`** - Basic movement block restriction
- **`partial_block.yaml`** - Partial movement block restriction
- **`complete_block.yaml`** - Complete movement block restriction

## Movement Restriction Types

### 1. Movement Slowdown
- **Type**: `slowdown`
- **Effect**: Reduces movement speed and action execution speed
- **Stackable**: Yes (additive)
- **Max Stacks**: 10
- **Priority**: 100

### 2. Movement Speedup
- **Type**: `speedup`
- **Effect**: Increases movement speed and action execution speed
- **Stackable**: Yes (additive)
- **Max Stacks**: 10
- **Priority**: 100

### 3. Movement Block
- **Type**: `block`
- **Effect**: Blocks movement and actions
- **Stackable**: Yes (replace)
- **Max Stacks**: 1
- **Priority**: 150

### 4. Partial Block
- **Type**: `partial`
- **Effect**: Partially blocks movement and actions
- **Stackable**: Yes (additive)
- **Max Stacks**: 5
- **Priority**: 150

### 5. Complete Block
- **Type**: `complete`
- **Effect**: Completely blocks movement and actions
- **Stackable**: No (replace)
- **Max Stacks**: 1
- **Priority**: 200

## Movement Restriction Properties

### Basic Properties
- **`restriction_id`**: Unique identifier for the restriction
- **`restriction_name`**: Human-readable name
- **`restriction_name_vi`**: Vietnamese name
- **`restriction_type`**: Type of movement restriction
- **`priority`**: Priority for stacking and application
- **`stackable`**: Whether the restriction can stack
- **`max_stacks`**: Maximum number of stacks
- **`stack_behavior`**: How stacks are handled

### Restriction Properties
- **`movement_type`**: Type of movement affected (all, ground, air, water, etc.)
- **`restriction_value`**: Base restriction value (0.0 to 1.0)
- **`restriction_duration`**: Duration of the restriction
- **`restriction_condition`**: Condition for the restriction
- **`visual_effect`**: Visual effect for the restriction
- **`audio_effect`**: Audio effect for the restriction

## Movement Restriction Rules

### Calculation Rules
- **`restriction_calculation`**: Calculate restriction value
- **`restriction_application`**: Apply restriction to actor
- **`restriction_break_processing`**: Process restriction break conditions

### Application Rules
- **`movement_modification`**: Modify movement speed
- **`action_modification`**: Modify action execution
- **`duration_management`**: Manage restriction duration

## Movement Restriction Modifiers

### Value Modifiers
- **`restriction_value_multiplier`**: Multiplier for restriction value
- **`restriction_condition_modifier`**: Modifier for restriction conditions
- **`restriction_duration_modifier`**: Modifier for restriction duration

### Scaling Modifiers
- **`constitution_scaling`**: Constitution-based scaling
- **`willpower_scaling`**: Willpower-based scaling
- **`element_mastery_scaling`**: Element mastery-based scaling

## Movement Restriction Interactions

### Conflict Interactions
- **`with_healing`**: Interaction with healing effects
- **`with_resistance`**: Interaction with resistance effects
- **`with_immunity`**: Interaction with immunity effects

### Synergy Interactions
- **`with_speed_boost`**: Interaction with speed boost effects
- **`with_movement_buff`**: Interaction with movement buff effects
- **`with_action_buff`**: Interaction with action buff effects

## Movement Restriction Immunity

### Immunity Types
- **`complete_immunity`**: Complete immunity to restriction
- **`partial_immunity`**: Partial immunity to restriction
- **`conditional_immunity`**: Conditional immunity to restriction

### Immunity Conditions
- **`element_mastery_above_threshold`**: Element mastery above threshold
- **`constitution_above_threshold`**: Constitution above threshold
- **`willpower_above_threshold`**: Willpower above threshold

## Movement Restriction Visual Effects

### Indicator Effects
- **`movement_indicator`**: Visual indicator for movement restriction
- **`action_indicator`**: Visual indicator for action restriction
- **`duration_indicator`**: Visual indicator for restriction duration

### Animation Effects
- **`movement_animation`**: Animation for movement restriction
- **`action_animation`**: Animation for action restriction
- **`break_animation`**: Animation for restriction break

## Movement Restriction Audio Effects

### Sound Effects
- **`movement_sound`**: Sound for movement restriction
- **`action_sound`**: Sound for action restriction
- **`break_sound`**: Sound for restriction break

### Voice Effects
- **`movement_voice`**: Voice for movement restriction
- **`action_voice`**: Voice for action restriction
- **`break_voice`**: Voice for restriction break

## Usage Examples

### Basic Movement Slowdown
```yaml
movement_restriction_definition:
  restriction_id: "movement_slowdown"
  restriction_type: "slowdown"
  restriction_value: 0.5
  movement_type: "all"
  stackable: true
  max_stacks: 10
```

### Complete Movement Block
```yaml
movement_restriction_definition:
  restriction_id: "complete_block"
  restriction_type: "complete"
  restriction_value: 1.0
  movement_type: "all"
  stackable: false
  max_stacks: 1
```

## Integration with Status Effects

Movement restrictions are integrated with status effects through the Status Core system:

1. **Status Effect Application**: Movement restrictions are applied when status effects are applied
2. **Status Effect Processing**: Movement restrictions are processed during status effect processing
3. **Status Effect Removal**: Movement restrictions are removed when status effects are removed
4. **Status Effect Stacking**: Movement restrictions follow status effect stacking rules

## Performance Considerations

- **Restriction Calculation**: Movement restrictions are calculated efficiently using cached values
- **Restriction Application**: Movement restrictions are applied in batches for performance
- **Restriction Processing**: Movement restrictions are processed asynchronously
- **Restriction Caching**: Movement restrictions are cached for quick access

## Security Considerations

- **Restriction Validation**: Movement restrictions are validated before application
- **Restriction Authorization**: Movement restrictions require proper authorization
- **Restriction Logging**: Movement restrictions are logged for audit purposes
- **Restriction Rate Limiting**: Movement restrictions are rate-limited to prevent abuse

## Future Enhancements

- **Advanced Movement Types**: Support for more complex movement types
- **Dynamic Restriction Values**: Support for dynamic restriction values
- **Restriction Combinations**: Support for combining multiple restrictions
- **Restriction Templates**: Support for restriction templates
- **Restriction Inheritance**: Support for restriction inheritance
- **Restriction Overrides**: Support for restriction overrides
