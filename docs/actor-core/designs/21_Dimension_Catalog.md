# 21 — Dimension Catalog

**Updated:** 2025-09-08 00:50

This document provides a comprehensive catalog of all dimensions supported by Actor Core v3, including their types, ranges, and usage guidelines.

## Primary Dimensions

### Physical Attributes
- **`strength`**: Physical power and damage capability
  - Range: 0 - 999,999
  - Units: Points
  - Affects: Attack power, carrying capacity, physical skills

- **`vitality`**: Health and endurance
  - Range: 0 - 999,999
  - Units: Points
  - Affects: HP max, stamina max, physical resistance

- **`dexterity`**: Speed and agility
  - Range: 0 - 999,999
  - Units: Points
  - Affects: Attack speed, dodge chance, critical hit chance

- **`intelligence`**: Mental capacity and magical power
  - Range: 0 - 999,999
  - Units: Points
  - Affects: MP max, magic power, learning rate

- **`spirit`**: Spiritual energy and cultivation
  - Range: 0 - 999,999
  - Units: Points
  - Affects: Spiritual energy, cultivation speed, breakthrough chance

## Derived Dimensions

### Health & Resources
- **`hp_max`**: Maximum health points
  - Range: 1 - 2,000,000
  - Units: Points
  - Formula: `vitality * 10 + strength * 2`

- **`mp_max`**: Maximum mana points
  - Range: 1 - 1,000,000
  - Units: Points
  - Formula: `intelligence * 8 + spirit * 4`

- **`stamina_max`**: Maximum stamina points
  - Range: 1 - 500,000
  - Units: Points
  - Formula: `vitality * 5 + dexterity * 3`

### Combat Attributes
- **`attack_power`**: Physical damage output
  - Range: 0 - 999,999
  - Units: Points
  - Formula: `strength * 2 + dexterity * 0.5`

- **`defense`**: Physical damage reduction
  - Range: 0 - 999,999
  - Units: Points
  - Formula: `vitality * 1.5 + dexterity * 0.8`

- **`magic_power`**: Magical damage output
  - Range: 0 - 999,999
  - Units: Points
  - Formula: `intelligence * 2 + spirit * 1.5`

- **`magic_resistance`**: Magical damage reduction
  - Range: 0 - 999,999
  - Units: Points
  - Formula: `spirit * 1.2 + intelligence * 0.8`

### Critical & Accuracy
- **`crit_rate`**: Critical hit chance
  - Range: 0.0 - 1.0
  - Units: Percentage (0.0 = 0%, 1.0 = 100%)
  - Formula: `dexterity * 0.001 + base_crit`

- **`crit_damage`**: Critical hit damage multiplier
  - Range: 1.0 - 10.0
  - Units: Multiplier
  - Formula: `1.5 + strength * 0.0001`

- **`accuracy`**: Hit chance
  - Range: 0.0 - 1.0
  - Units: Percentage
  - Formula: `dexterity * 0.0008 + base_accuracy`

### Speed & Movement
- **`move_speed`**: Movement speed
  - Range: 0.0 - 12.0
  - Units: Meters per second
  - Formula: `dexterity * 0.01 + base_speed`

- **`attack_speed`**: Attack speed
  - Range: 0.1 - 10.0
  - Units: Attacks per second
  - Formula: `dexterity * 0.005 + base_attack_speed`

- **`cast_speed`**: Spell casting speed
  - Range: 0.1 - 10.0
  - Units: Spells per second
  - Formula: `intelligence * 0.003 + spirit * 0.002`

### Resource Management
- **`cooldown_reduction`**: Ability cooldown reduction
  - Range: 0.0 - 0.5
  - Units: Percentage (0.0 = 0%, 0.5 = 50%)
  - Formula: `intelligence * 0.0001 + spirit * 0.0001`

- **`mana_efficiency`**: Mana cost reduction
  - Range: 0.0 - 0.8
  - Units: Percentage
  - Formula: `spirit * 0.0002 + intelligence * 0.0001`

- **`energy_efficiency`**: Energy cost reduction
  - Range: 0.0 - 0.8
  - Units: Percentage
  - Formula: `vitality * 0.0001 + spirit * 0.0001`

### Learning & Progression
- **`learning_rate`**: Experience gain multiplier
  - Range: 0.1 - 5.0
  - Units: Multiplier
  - Formula: `intelligence * 0.0001 + spirit * 0.0001`

- **`cultivation_speed`**: Cultivation progress rate
  - Range: 0.1 - 10.0
  - Units: Multiplier
  - Formula: `spirit * 0.0002 + intelligence * 0.0001`

- **`breakthrough_success`**: Breakthrough success chance
  - Range: 0.0 - 1.0
  - Units: Percentage
  - Formula: `spirit * 0.0001 + base_breakthrough`

## Meta/World Dimensions

### Life & Longevity
- **`lifespan_years`**: Character lifespan
  - Range: 1 - 10,000
  - Units: Years
  - Operator: MAX
  - Formula: `vitality * 0.1 + spirit * 0.05 + base_lifespan`

### Social & Reputation
- **`poise_rank`**: Social standing rank
  - Range: 0 - 10
  - Units: Rank
  - Operator: MAX
  - Formula: `charisma * 0.0001 + reputation_bonus`

- **`charisma`**: Social influence
  - Range: 0 - 999,999
  - Units: Points
  - Affects: Social interactions, reputation gain

### Specialized Dimensions
- **`stealth`**: Stealth capability
  - Range: 0.0 - 1.0
  - Units: Percentage
  - Formula: `dexterity * 0.0005 + intelligence * 0.0003`

- **`perception`**: Awareness and detection
  - Range: 0.0 - 1.0
  - Units: Percentage
  - Formula: `intelligence * 0.0004 + dexterity * 0.0002`

- **`luck`**: Fortune and chance
  - Range: 0 - 999,999
  - Units: Points
  - Affects: Random events, rare drops, critical success

## Dimension Categories

### By Type
- **Primary**: Core character attributes
- **Derived**: Calculated from primary attributes
- **Meta**: World-level or system-level attributes
- **Specialized**: Specific to certain game mechanics

### By Usage
- **Combat**: Directly affects combat effectiveness
- **Social**: Affects social interactions and reputation
- **Progression**: Affects character advancement
- **Utility**: Affects general gameplay mechanics

### By Calculation Method
- **Pipeline**: Uses FLAT → MULT → POST_ADD pipeline
- **Operator**: Uses SUM/MAX/MIN operators
- **Hybrid**: Combines both methods

## Clamp Ranges

### Standard Ranges
```yaml
dimensions:
  # Primary attributes
  strength: { min: 0, max: 999999 }
  vitality: { min: 0, max: 999999 }
  dexterity: { min: 0, max: 999999 }
  intelligence: { min: 0, max: 999999 }
  spirit: { min: 0, max: 999999 }
  
  # Health & resources
  hp_max: { min: 1, max: 2000000 }
  mp_max: { min: 1, max: 1000000 }
  stamina_max: { min: 1, max: 500000 }
  
  # Combat attributes
  attack_power: { min: 0, max: 999999 }
  defense: { min: 0, max: 999999 }
  magic_power: { min: 0, max: 999999 }
  magic_resistance: { min: 0, max: 999999 }
  
  # Critical & accuracy
  crit_rate: { min: 0.0, max: 1.0 }
  crit_damage: { min: 1.0, max: 10.0 }
  accuracy: { min: 0.0, max: 1.0 }
  
  # Speed & movement
  move_speed: { min: 0.0, max: 12.0 }
  attack_speed: { min: 0.1, max: 10.0 }
  cast_speed: { min: 0.1, max: 10.0 }
  
  # Resource management
  cooldown_reduction: { min: 0.0, max: 0.5 }
  mana_efficiency: { min: 0.0, max: 0.8 }
  energy_efficiency: { min: 0.0, max: 0.8 }
  
  # Learning & progression
  learning_rate: { min: 0.1, max: 5.0 }
  cultivation_speed: { min: 0.1, max: 10.0 }
  breakthrough_success: { min: 0.0, max: 1.0 }
  
  # Meta/World
  lifespan_years: { min: 1, max: 10000 }
  poise_rank: { min: 0, max: 10 }
  charisma: { min: 0, max: 999999 }
  stealth: { min: 0.0, max: 1.0 }
  perception: { min: 0.0, max: 1.0 }
  luck: { min: 0, max: 999999 }
```

## Usage Guidelines

### 1. Dimension Selection
- Choose dimensions that fit your game's mechanics
- Consider the relationship between primary and derived dimensions
- Plan for future expansion and new dimensions

### 2. Range Setting
- Set realistic ranges based on game balance
- Consider the impact of extreme values
- Allow for growth and progression

### 3. Formula Design
- Keep formulas simple and understandable
- Ensure balanced progression curves
- Test with various input values

### 4. Performance Considerations
- Cache frequently calculated dimensions
- Use efficient calculation methods
- Consider the impact on aggregation performance

## Conclusion

This dimension catalog provides a comprehensive foundation for Actor Core v3. Developers can extend this catalog with additional dimensions as needed for their specific game requirements, while maintaining consistency with the established patterns and guidelines.
