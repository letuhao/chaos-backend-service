# 09 — Context Modifiers

**Updated:** 2025-09-08 00:30

Context Modifiers provide a way for subsystems to apply temporary effects during specific game events (damage, healing, experience gain, item drops, etc.) without permanently modifying actor stats.

## Overview

Context Modifiers are applied during specific game contexts and can modify the final calculated values through:
- **Additive Percent**: Percentage-based additions
- **Multipliers**: Array of multiplicative factors
- **Post Add**: Flat additions after all calculations

## ModifierPack Structure

```go
type ModifierPack struct {
    AdditivePercent float64   // Percentage addition (e.g., 0.1 = +10%)
    Multipliers     []float64 // Array of multipliers (e.g., [1.2, 1.5] = 1.2 * 1.5 = 1.8x)
    PostAdd         float64   // Flat addition after all calculations
}
```

## Usage Examples

### Damage Context
```go
// Subsystem outputs damage modifier
output := SubsystemOutput{
    Context: map[string]ModifierPack{
        "damage": {
            AdditivePercent: 0.2,    // +20% damage
            Multipliers:     []float64{1.5, 1.2}, // 1.5 * 1.2 = 1.8x multiplier
            PostAdd:         50.0,   // +50 flat damage
        },
    },
}
```

### Healing Context
```go
// Healing effectiveness modifier
output := SubsystemOutput{
    Context: map[string]ModifierPack{
        "healing": {
            AdditivePercent: 0.3,    // +30% healing
            Multipliers:     []float64{1.2}, // 1.2x multiplier
            PostAdd:         25.0,   // +25 flat healing
        },
    },
}
```

### Experience Gain Context
```go
// Experience bonus modifier
output := SubsystemOutput{
    Context: map[string]ModifierPack{
        "experience": {
            AdditivePercent: 0.5,    // +50% experience
            Multipliers:     []float64{1.1, 1.2}, // 1.32x multiplier
            PostAdd:         0.0,    // No flat bonus
        },
    },
}
```

### Item Drop Context
```go
// Drop rate modifier
output := SubsystemOutput{
    Context: map[string]ModifierPack{
        "drop_rate": {
            AdditivePercent: 0.0,    // No percentage bonus
            Multipliers:     []float64{2.0}, // 2x drop rate
            PostAdd:         0.0,    // No flat bonus
        },
    },
}
```

## Application Algorithm

When applying context modifiers to a base value:

```go
func ApplyContextModifier(baseValue float64, modifier ModifierPack) float64 {
    // Step 1: Apply additive percentage
    additiveValue := baseValue * (1.0 + modifier.AdditivePercent)
    
    // Step 2: Apply all multipliers
    result := additiveValue
    for _, multiplier := range modifier.Multipliers {
        result *= multiplier
    }
    
    // Step 3: Apply post-add
    result += modifier.PostAdd
    
    return result
}
```

## Context Types

### Standard Contexts
- **`damage`**: Damage dealt to targets
- **`healing`**: Healing received
- **`experience`**: Experience points gained
- **`drop_rate`**: Item drop probability
- **`gold`**: Gold/currency gained
- **`reputation`**: Reputation points gained

### Custom Contexts
Subsystems can define custom context types for their specific needs:
- **`cultivation_speed`**: Cultivation progress rate
- **`breakthrough_chance`**: Breakthrough success rate
- **`skill_learning`**: Skill learning speed
- **`crafting_quality`**: Crafting item quality

## Integration with SubsystemOutput

Context modifiers are included in the `SubsystemOutput` structure:

```go
type SubsystemOutput struct {
    Primary []Contribution
    Derived []Contribution
    Caps    []CapContribution
    Context map[string]ModifierPack  // Context modifiers
    Meta    SubsystemMeta
}
```

## Best Practices

### 1. Use Appropriate Context Types
- Choose context types that match the game event
- Avoid generic contexts when specific ones exist
- Document custom context types

### 2. Reasonable Modifier Values
- **AdditivePercent**: Typically 0.0 to 2.0 (0% to 200%)
- **Multipliers**: Typically 0.1 to 10.0 (0.1x to 10x)
- **PostAdd**: Can be any value depending on context

### 3. Performance Considerations
- Limit the number of active context modifiers
- Use efficient data structures for modifier lookup
- Consider caching frequently used modifiers

### 4. Validation
- Validate modifier values are within reasonable ranges
- Ensure multipliers are positive
- Check for potential overflow conditions

## Example: Complete Subsystem with Context Modifiers

```go
type CombatSubsystem struct {
    systemID string
    priority int
}

func (cs *CombatSubsystem) Contribute(ctx context.Context, actor *Actor) (SubsystemOutput, error) {
    // Calculate base contributions
    primary := []Contribution{
        {Dimension: "strength", Bucket: "FLAT", Value: 100, System: cs.systemID},
    }
    
    // Calculate context modifiers based on actor state
    contextModifiers := make(map[string]ModifierPack)
    
    // Apply damage modifier if actor is in combat
    if actor.IsInCombat() {
        contextModifiers["damage"] = ModifierPack{
            AdditivePercent: 0.2,    // +20% damage
            Multipliers:     []float64{1.1}, // 1.1x multiplier
            PostAdd:         10.0,   // +10 flat damage
        }
    }
    
    // Apply healing modifier if actor has healing buff
    if actor.HasBuff("healing_boost") {
        contextModifiers["healing"] = ModifierPack{
            AdditivePercent: 0.5,    // +50% healing
            Multipliers:     []float64{1.2}, // 1.2x multiplier
            PostAdd:         0.0,    // No flat bonus
        }
    }
    
    return SubsystemOutput{
        Primary: primary,
        Derived: []Contribution{},
        Caps:    []CapContribution{},
        Context: contextModifiers,
        Meta:    SubsystemMeta{System: cs.systemID, Version: 1},
    }, nil
}
```

## Testing Context Modifiers

```go
func TestContextModifiers(t *testing.T) {
    baseValue := 100.0
    modifier := ModifierPack{
        AdditivePercent: 0.2,    // +20%
        Multipliers:     []float64{1.5, 1.2}, // 1.8x
        PostAdd:         50.0,   // +50
    }
    
    // Expected: (100 * 1.2) * 1.8 + 50 = 120 * 1.8 + 50 = 216 + 50 = 266
    result := ApplyContextModifier(baseValue, modifier)
    assert.Equal(t, 266.0, result)
}
```

## Conclusion

Context Modifiers provide a flexible way to apply temporary effects during specific game events. They complement the core contribution system by allowing subsystems to modify values in context-specific ways without permanently changing actor stats.

When implementing context modifiers:
1. Choose appropriate context types
2. Use reasonable modifier values
3. Consider performance implications
4. Validate input values
5. Test thoroughly with various scenarios