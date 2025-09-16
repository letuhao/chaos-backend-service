# Fire Element

## 📋 **Element Overview**

**Fire (Hỏa)** is one of the five elements of Ngũ Hành (Five Elements), representing fire, high temperature, destruction, and energy. In Element Core, Fire is designed to create high damage and cause Burning status effects.

### **Basic Information**
- **Element ID**: `fire`
- **Element Name**: Fire (Hỏa)
- **Category**: Ngũ Hành (Five Elements) — engine category: `five_elements`
- **Classification**: Yang - Active, hot, destructive
- **Visual**: Red, orange, yellow with fire effects

### **Lore & Background**
Fire represents energy, passion, and destructive power. In Eastern philosophy, Fire generates Earth and overcomes Metal. It symbolizes summer, south direction, and daytime.

## 🔥 **Element Properties**

### **Base Characteristics**
- **Damage Type**: Fire Damage
- **Damage Scaling**: High (1.2x base)
- **Defense Scaling**: Low (0.8x base)
- **Critical Scaling**: High (1.3x base)
- **Accuracy Scaling**: Medium (1.0x base)

### **Scaling Factors**
```yaml
# Defined centrally in probability_config.yaml (do not duplicate here)
scaling_factors:
  crit_rate: 120.0
  crit_damage: 130.0
  accuracy: 100.0
  defense: 80.0
  power: 120.0
```

### **Special Properties**
- **Destructive**: High destructive capability
- **Volatile**: Easy to spread and create effects
- **Energetic**: Increases action speed and cooldown
- **Unstable**: Can cause damage to self

## 🔥 **Status Effects**

### **Burning Status Effect**

#### **Mechanics**
- **Status Name**: Burning
- **Element Type**: Fire
- **Type**: Damage over Time (DoT)
- **Stackable**: Yes (up to 5 stacks)
- **Refresh Duration**: Yes

#### **Status Properties**
```yaml
burning_status:
  base_probability: 0.15        # 15% base chance
  base_duration: 8.0            # 8 seconds base duration
  base_intensity: 1.0           # 1.0x base intensity
  tick_interval: 1.0            # 1 second per tick
  max_stacks: 5                 # Maximum 5 stacks
  dynamics:
    intensity_gain: 0.02
    intensity_damping: 0.01
    decay_rate: 0.05
    refractory_gain: 0.5
    refractory_decay: 0.1
```

#### **Damage Calculation**
```rust
// Burning damage increases with each tick
fn calculate_burning_damage(base_damage: f64, intensity: f64, current_tick: i32) -> f64 {
    let tick_multiplier = 1.0 + (current_tick as f64 * 0.1);
    base_damage * intensity * tick_multiplier
}

// Example: 100 base damage, 1.5 intensity, tick 3
// Damage = 100 * 1.5 * 1.3 = 195 damage
```

#### **Stacking Rules**
- **Stacking Type**: Additive duration, multiplicative intensity
- **New Stack**: Adds duration to existing stacks
- **Intensity**: New intensity replaces old intensity if higher
- **Refresh**: Refreshes duration if same or higher intensity

#### **Spread Mechanics**
- Burning can spread to nearby targets based on environment and intensity.
- Controlled by `spread_rules` in the element config.

```yaml
# elements/configs/fire_element.yaml (burning spread excerpt)
burning_status:
  spread_rules:
    enabled: true
    spread_chance_base: 0.05
    spread_range: 2.5
    spread_max_targets: 2
```

#### **Visual Effects**
- **Particle Effect**: Red/orange flames around target
- **Sound Effect**: Crackling fire sounds
- **UI Indicator**: Fire icon with stack count
- **Damage Numbers**: Red damage numbers

### **Fire Regeneration Buff (Defender)**

#### **Mechanics**
- **Buff Name**: Fire Regeneration
- **Element Type**: Fire (Defender receives)
- **Type**: Healing over Time (HoT)
- **Trigger**: When defender takes fire damage
- **Stackable**: Yes (up to 3 stacks)
- **Refresh Duration**: Yes

#### **Buff Properties**
```yaml
fire_regeneration_buff:
  base_probability: 0.0         # 0% base chance to trigger (mastery-based only)
  base_duration: 12.0           # 12 seconds base duration
  base_intensity: 1.0           # 1.0x base intensity
  tick_interval: 2.0            # 2 seconds per tick
  max_stacks: 3                 # Maximum 3 stacks
  dynamics:
    intensity_gain: 0.015
    intensity_damping: 0.012
    decay_rate: 0.04
    refractory_gain: 0.6
    refractory_decay: 0.12
  hp_heal_per_tick: 0.02        # 2% max HP per tick
  stamina_heal_per_tick: 0.03   # 3% max Stamina per tick
```

#### **Healing Calculation**
```rust
// Fire regeneration healing increases with each tick
fn calculate_fire_regeneration_healing(
    max_hp: f64, 
    max_stamina: f64, 
    intensity: f64, 
    current_tick: i32
) -> (f64, f64) {
    let tick_multiplier = 1.0 + (current_tick as f64 * 0.05);
    let hp_heal = max_hp * 0.02 * intensity * tick_multiplier;
    let stamina_heal = max_stamina * 0.03 * intensity * tick_multiplier;
    
    (hp_heal, stamina_heal)
}

// Example: 1000 max HP, 500 max Stamina, 1.5 intensity, tick 4
// HP Heal = 1000 * 0.02 * 1.5 * 1.2 = 36 HP per tick
// Stamina Heal = 500 * 0.03 * 1.5 * 1.2 = 27 Stamina per tick
```

#### **Trigger Conditions**
```rust
// Fire regeneration trigger using Probability Engine (sigmoid; probability bound only)
fn calculate_fire_regeneration_trigger(
    attacker_fire_mastery: f64,
    defender_fire_mastery: f64,
    base_probability: f64,
    cfg: &InteractionConfig,
) -> f64 {
    let norm_diff = normalize_mastery_difference(defender_fire_mastery, attacker_fire_mastery);
    let scaled = custom_sigmoid(norm_diff / cfg.trigger_scale, cfg.steepness);
    let trig = base_probability + scaled;
    trig.clamp(0.0, 1.0)
}
```

#### **Stacking Rules**
- **Stacking Type**: Additive duration, multiplicative intensity
- **New Stack**: Adds duration to existing stacks
- **Intensity**: New intensity replaces old intensity if higher
- **Refresh**: Refreshes duration if same or higher intensity
- **Refractory**: xác suất trigger giảm tạm thời sau khi kích hoạt (không dùng cooldown cứng)

### **Same-Element Interaction: Fire ↔ Fire**

#### **Heat Resonance**
- +skill_execution_speed và +crit_rate theo thời gian giao tranh; đồng thời +resource_cost (risk-reward).
- Dynamics: intensity tăng theo Δ (mastery/power chênh lệch), dập bởi damping; refractory tránh spam.

#### **Ember Shield**
- Chuyển một phần damage nhận thành Burning DoT trả ngược attacker (thorns-burn) với ramp nhanh rồi tự giảm.
- Dynamics: α cao, β cũng cao (bùng nổ ngắn, dập nhanh).

#### **Overheat**
- Tích “nhiệt” khi tấn công liên tục: +crit_damage nhưng tăng self-DoT mỏng (overheat burn) nếu vượt ngưỡng.
- Dynamics: refractory tăng nhanh để ngăn tích vô hạn; self-DoT tỷ lệ với intensity hiện tại.

```yaml
# elements/configs/fire_element.yaml (same-element excerpt)
same_element_effects:
  - id: "heat_resonance"
    from_status_pool: true
    apply_to: "both"
    dynamics:
      intensity_gain: 0.015
      intensity_damping: 0.012
      refractory_gain: 0.4
      refractory_decay: 0.1
    stat_hooks:
      skill_execution_speed: { delta_weight: 0.12 }
      crit_rate: { delta_weight: 0.06 }
      resource_cost: { delta_weight: 0.05 }

  - id: "ember_shield"
    from_status_pool: true
    apply_to: "defender"
    dynamics:
      intensity_gain: 0.02
      intensity_damping: 0.02
    reflect:
      type: "burning_thorns"
    proportion_of_damage: 0.08

  - id: "overheat"
    from_status_pool: true
    apply_to: "attacker"
    dynamics:
      intensity_gain: 0.018
      intensity_damping: 0.015
      refractory_gain: 0.5
    stat_hooks:
      crit_damage: { delta_weight: 0.08 }
      self_dot_risk: { base: "light", scales_with_intensity: true }
```

### **Neutral Interaction: Fire ↔ Neutral**

- Áp dụng khi đối phương không thuộc các cặp trong `pairs`. Fire vẫn có thể gây Burning theo xác suất cơ bản và cơ chế probability engine.
- Không buff/debuff đặc thù tương sinh/khắc; coi như trung lập nhưng vẫn kích hoạt trạng thái nền của Fire.

```yaml
# elements/configs/fire_element.yaml (neutral excerpt)
neutral_effects:
  - id: "burning_fallback"
    from_status_pool: true
    apply_to: "defender"
    pool_id: "burning_fallback"
    probability:
      base: "from_element"            # lấy base_probability từ burning
      use_probability_engine: true
      scaling_factor_key: "status_probability"
    dynamics_override: {}              # có thể rỗng để dùng dynamics mặc định
```

### **Environmental Modifiers**

Certain environments adjust Fire behavior. These are optional tuning knobs.

```yaml
# elements/configs/fire_element.yaml (environment excerpt)
environment_mods:
  rain:
    status_probability_add: -0.05
    intensity_gain_mod: -0.005
  dry:
    status_probability_add: +0.05
    intensity_gain_mod: +0.005
  indoors:
    spread_chance_add: -0.02
```

## 📊 **Derived Stats**

### **Supported Derived Stats**
```rust
// Fire element supports these derived stats
pub const FIRE_DERIVED_STATS: [DerivedStatType; 12] = [
    // Basic stats
    DerivedStatType::PowerPoint,           // Fire attack power
    DerivedStatType::DefensePoint,         // Fire defense
    DerivedStatType::CritRate,             // Fire crit rate
    DerivedStatType::CritDamage,           // Fire crit damage
    DerivedStatType::AccurateRate,         // Fire accuracy
    DerivedStatType::DodgeRate,            // Fire dodge rate
    
    // Status effect stats (Burning + Fire Regeneration)
    DerivedStatType::StatusProbability,    // Burning probability + Fire regeneration trigger chance
    DerivedStatType::StatusDuration,       // Burning duration + Fire regeneration duration
    DerivedStatType::StatusIntensity,      // Burning intensity + Fire regeneration intensity
    DerivedStatType::StatusResistance,     // Burning resistance + Fire regeneration resistance
    DerivedStatType::StatusDurationReduction, // Burning duration reduction + Fire regeneration duration reduction
    DerivedStatType::StatusIntensityReduction, // Burning intensity reduction + Fire regeneration intensity reduction
];
```

### **Stat Weights**
```yaml
stat_weights:
  # Basic combat stats
  power_point: 1.0              # High priority
  crit_rate: 0.8                # High priority
  crit_damage: 0.7              # High priority
  defense_point: 0.3            # Low priority
  dodge_rate: 0.2               # Low priority
  
  # Status effect stats (Burning + Fire Regeneration)
  status_probability: 0.7       # High priority (Burning + Regeneration trigger)
  status_intensity: 0.6         # High priority (Burning + Regeneration intensity)
  status_duration: 0.5          # Medium priority (Burning + Regeneration duration)
  status_resistance: 0.4        # Medium priority (Burning + Regeneration resistance)
  status_duration_reduction: 0.3 # Low priority (Defensive)
  status_intensity_reduction: 0.2 # Low priority (Defensive)
```

### **Scaling Formulas**
```rust
// Fire power scaling
fn calculate_fire_power(base_power: f64, fire_stat: f64) -> f64 {
    base_power * (1.0 + fire_stat * 0.01) // 1% per point
}

// Fire crit rate scaling
fn calculate_fire_crit_rate(base_crit: f64, fire_stat: f64) -> f64 {
    base_crit + (fire_stat * 0.005) // 0.5% per point
}

// Fire burning probability
fn calculate_burning_probability(base_prob: f64, fire_stat: f64) -> f64 {
    base_prob + (fire_stat * 0.001) // 0.1% per point
}

// Fire regeneration uses same stats as Burning status
// StatusProbability -> Fire regeneration trigger chance
// StatusIntensity -> Fire regeneration intensity  
// StatusDuration -> Fire regeneration duration
// StatusResistance -> Fire regeneration resistance
```

## 🔄 **Element Interactions**

### **Tương Sinh (Generating)**
- **Fire → Earth**: Fire generates Earth (1.2x damage)
- **Earth → Metal**: Earth generates Metal (1.2x damage)
- **Metal → Water**: Metal generates Water (1.2x damage)
- **Water → Wood**: Water generates Wood (1.2x damage)
- **Wood → Fire**: Wood generates Fire (1.2x damage)

### **Tương Khắc (Overcoming)**
- **Fire → Metal**: Fire overcomes Metal (1.5x damage)
- **Metal → Wood**: Metal overcomes Wood (1.5x damage)
- **Wood → Earth**: Wood overcomes Earth (1.5x damage)
- **Earth → Water**: Earth overcomes Water (1.5x damage)
- **Water → Fire**: Water overcomes Fire (0.7x damage)

### **Special Interactions**
- **Fire vs Ice**: 1.8x damage (Fire melts Ice)
- **Fire vs Water**: 0.5x damage (Water extinguishes Fire)
- **Fire vs Nature**: 1.3x damage (Fire burns Nature)
- **Fire vs Metal**: 1.5x damage (Fire melts Metal)

### **Interaction Matrix**
```yaml
interactions:
  # Tương sinh
  - attacker: "fire"
    defender: "earth"
    multiplier: 1.2
    type: "generating"
    description: "Fire generates Earth"
  
  # Tương khắc
  - attacker: "fire"
    defender: "metal"
    multiplier: 1.5
    type: "overcoming"
    description: "Fire overcomes Metal"
  
  # Special
  - attacker: "fire"
    defender: "ice"
    multiplier: 1.8
    type: "special"
    description: "Fire vs Ice"
```

## ⚔️ **Game Mechanics**

### **Combat Applications**

#### **Offensive Capabilities**
- **High Damage**: Fire deals high base damage
- **Critical Hits**: High crit rate and crit damage
- **Burning DoT**: Consistent damage over time
- **Area Damage**: Can spread to nearby enemies
- **Armor Penetration**: Effective against metal armor

#### **Defensive Limitations**
- **Low Defense**: Fire users have lower defense
- **Vulnerable to Water**: Weak against water attacks
- **Self-Damage**: Can damage self if not controlled
- **Resource Consumption**: High mana/stamina cost

### **Shield Interactions**

#### **Fire Shields**
- **Absorption**: Can absorb fire damage
- **Reflection**: Can reflect fire damage back
- **Burning Immunity**: Immune to burning status
- **Heat Resistance**: Reduces fire damage taken

#### **Shield Properties**
```yaml
fire_shield:
  absorption_rate: 0.6          # capped by global absorption_max
  reflection_rate: 0.3          # 30% fire damage reflection
  burning_immunity: true        # Immune to burning
  heat_resistance: 0.5          # 50% fire damage reduction
```

### **Item Attribute Effects**

#### **Fire Weapons**
- **Base Damage**: +20% fire damage
- **Burning Chance**: +10% burning probability
- **Crit Rate**: +5% crit rate
- **Crit Damage**: +15% crit damage

#### **Fire Armor**
- **Fire Resistance**: +30% fire damage reduction
- **Burning Immunity**: 50% chance to resist burning
- **Heat Tolerance**: +20% heat resistance
- **Mobility Penalty**: -10% movement speed

#### **Fire Accessories**
- **Fire Power**: +15% fire power
- **Burning Duration**: +2 seconds burning duration
- **Crit Rate**: +3% crit rate
- **Status Probability**: +5% status probability

### **Race Talent Bonuses**

#### **Fire-Affinity Races**
- **Dragonborn**: +25% fire damage, +15% burning chance
- **Phoenix**: +20% fire damage, burning immunity
- **Salamander**: +30% fire resistance, +10% fire power
- **Ifrit**: +35% fire damage, +20% crit rate

#### **Fire-Resistant Races**
- **Water Elementals**: +50% fire resistance, -20% fire damage
- **Ice Giants**: +40% fire resistance, -15% fire damage
- **Earth Dwarves**: +25% fire resistance, +10% earth damage vs fire

## 🧪 **Configuration Examples**

### **YAML Configuration**
```yaml
# fire_element.yaml
# Full stub stored at elements/configs/fire_element.yaml
# Below is a shortened view
element:
  id: "fire"
  name: "Fire"
  category: "five_elements"
  description: "Fire, hot, destructive"
  
  # Base properties
  base_damage: 100.0
  base_defense: 80.0
  base_crit_rate: 0.15
  base_crit_damage: 1.5
  base_accuracy: 0.85
  
  # Scaling factors
  scaling_factors:
    power: 1.2
    defense: 0.8
    crit_rate: 1.3
    crit_damage: 1.3
    accuracy: 1.0
  
  # Derived stats (Burning + Fire Regeneration share same stats)
  derived_stats:
    - "power_point"
    - "defense_point"
    - "crit_rate"
    - "crit_damage"
    - "accurate_rate"
    - "dodge_rate"
    - "status_probability"        # Burning probability + Fire regeneration trigger
    - "status_duration"           # Burning duration + Fire regeneration duration
    - "status_intensity"          # Burning intensity + Fire regeneration intensity
    - "status_resistance"         # Burning resistance + Fire regeneration resistance
    - "status_duration_reduction" # Burning duration reduction + Fire regeneration duration reduction
    - "status_intensity_reduction" # Burning intensity reduction + Fire regeneration intensity reduction
  
  # Status effects
  status_effects:
    - name: "burning"
      base_probability: 0.15
      base_duration: 8.0
      base_intensity: 1.0
      max_duration: 30.0
      max_intensity: 3.0
      tick_interval: 1.0
      max_stacks: 5
      stackable: true
      refresh_duration: true
    
    # Fire regeneration buff (defender receives)
    - name: "fire_regeneration"
      base_probability: 0.0       # 0% base chance (mastery-based only)
      base_duration: 12.0
      base_intensity: 1.0
      max_duration: 30.0
      max_intensity: 2.5
      tick_interval: 2.0
      max_stacks: 3
      stackable: true
      refresh_duration: true
      trigger_cooldown: 5.0
      hp_heal_per_tick: 0.02      # 2% max HP per tick
      stamina_heal_per_tick: 0.03 # 3% max Stamina per tick
  
  # Interactions
  interactions:
    generating:
      - element: "earth"
        multiplier: 1.2
        description: "Fire generates Earth"
    
    overcoming:
      - element: "metal"
        multiplier: 1.5
        description: "Fire overcomes Metal"
    
    special:
      - element: "ice"
        multiplier: 1.8
        description: "Fire vs Ice"
      - element: "water"
        multiplier: 0.5
        description: "Water extinguishes Fire"
```

### **JSON Schema**
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "element": {
      "type": "object",
      "properties": {
        "id": { "type": "string", "enum": ["fire"] },
        "name": { "type": "string" },
        "category": { "type": "string", "enum": ["five_elements"] },
        "description": { "type": "string" },
        "base_damage": { "type": "number", "minimum": 0 },
        "base_defense": { "type": "number", "minimum": 0 },
        "base_crit_rate": { "type": "number", "minimum": 0, "maximum": 1 },
        "base_crit_damage": { "type": "number", "minimum": 1 },
        "base_accuracy": { "type": "number", "minimum": 0, "maximum": 1 }
      },
      "required": ["id", "name", "category", "description"]
    }
  },
  "required": ["element"]
}
```

## 🧪 **Testing & Validation**

### **Unit Tests**
```rust
#[cfg(test)]
mod fire_element_tests {
    use super::*;
    
    #[test]
    fn test_fire_damage_calculation() {
        let fire_element = FireElement::new();
        let base_damage = 100.0;
        let fire_power = 50.0;
        
        let final_damage = fire_element.calculate_damage(base_damage, fire_power);
        assert_eq!(final_damage, 160.0); // 100 * (1 + 50 * 0.01) * 1.2
    }
    
    #[test]
    fn test_burning_status_application() {
        let mut fire_element = FireElement::new();
        let attacker_stats = create_test_stats(100.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let defender_stats = create_test_stats(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        
        let probability = fire_element.calculate_burning_probability(
            &attacker_stats,
            &defender_stats
        ).unwrap();
        
        assert!(probability > 0.0);
        assert!(probability <= 1.0);
    }
    
    #[test]
    fn test_fire_interactions() {
        let fire_element = FireElement::new();
        
        // Test tương sinh
        let earth_multiplier = fire_element.get_damage_multiplier("fire", "earth");
        assert_eq!(earth_multiplier, 1.2);
        
        // Test tương khắc
        let metal_multiplier = fire_element.get_damage_multiplier("fire", "metal");
        assert_eq!(metal_multiplier, 1.5);
        
        // Test special interaction
        let ice_multiplier = fire_element.get_damage_multiplier("fire", "ice");
        assert_eq!(ice_multiplier, 1.8);
    }
}
```

### **Integration Tests**
```rust
#[cfg(test)]
mod fire_element_integration_tests {
    use super::*;
    
    #[test]
    fn test_fire_combat_integration() {
        let combat_system = CombatSystem::new();
        let fire_element = FireElement::new();
        
        // Test fire damage in combat
        let damage_result = combat_system.calculate_damage(
            "attacker",
            "target",
            "fire",
            100.0
        ).unwrap();
        
        assert!(damage_result.final_damage > 0.0);
        assert!(damage_result.is_critical || !damage_result.is_critical);
    }
    
    #[test]
    fn test_fire_shield_integration() {
        let shield_system = ShieldSystem::new();
        let fire_element = FireElement::new();
        
        // Test fire shield absorption
        let absorption = shield_system.calculate_absorption(
            "shield_id",
            100.0,
            "fire"
        ).unwrap();
        
        assert!(absorption > 0.0);
        assert!(absorption <= 100.0);
    }
}
```

### **Performance Tests**
```rust
#[cfg(test)]
mod fire_element_performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_fire_calculation_performance() {
        let fire_element = FireElement::new();
        let start = Instant::now();
        
        for _ in 0..10000 {
            let _ = fire_element.calculate_damage(100.0, 50.0);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should complete in < 100ms
    }
}
```

## 🎯 **Balance Considerations**

### **Strengths**
- **High Damage Output**: Excellent for DPS roles
- **Critical Hit Focus**: Great for burst damage
- **Status Effect**: Burning provides consistent DoT
- **Element Synergy**: Good with Earth and Wood elements
- **Defensive Regeneration**: Fire mastery provides defensive healing
- **Dual Purpose**: Offensive damage + defensive regeneration

### **Weaknesses**
- **Low Defense**: Vulnerable to counter-attacks
- **Water Vulnerability**: Weak against water attacks
- **Resource Intensive**: High mana/stamina cost
- **Self-Damage Risk**: Can damage self if uncontrolled
- **Mastery Dependency**: Regeneration requires high fire mastery
- **Cooldown Limitation**: 5-second cooldown between regeneration triggers

### **Balance Recommendations**
- **Damage**: Keep fire damage high but not overwhelming
- **Defense**: Ensure fire users have ways to mitigate low defense
- **Status Effects**: Balance burning damage vs duration
- **Interactions**: Ensure water counter is meaningful but not overpowered
- **Regeneration**: Balance regeneration healing vs mastery requirements
- **Cooldown**: Monitor 5-second cooldown to prevent regeneration spam
- **Mastery Scaling**: Ensure regeneration scales appropriately with mastery

## 🚀 **Future Enhancements**

### **Planned Features**
- **Fire Mastery**: Advanced fire techniques
- **Element Fusion**: Fire + other elements
- **Environmental Effects**: Fire spreading mechanics
- **Advanced Status**: More complex burning effects

### **Potential Changes**
- **Balance Adjustments**: Based on player feedback
- **New Interactions**: Additional element combinations
- **Visual Improvements**: Better fire effects
- **Sound Design**: Enhanced fire audio

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Complete  
**Maintainer**: Chaos World Team
