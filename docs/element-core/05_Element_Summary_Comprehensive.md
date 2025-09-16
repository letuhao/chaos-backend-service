# Element Summary - Comprehensive Guide

## 📋 **Overview**

This document provides a comprehensive summary of all element types in the Element Core system, including their properties, status effects, interactions, and game mechanics. This serves as a quick reference for developers, game designers, and system architects.

## 🆔 **Engine IDs & Aliases**

- Engine sử dụng English snake_case IDs thống nhất (vd: `fire, water, wood, metal, earth`).
- Aliases dùng cho hiển thị/ngôn ngữ (vd: `vi`, `zh_pinyin`) và không tham gia vào logic.
- Ví dụ cấu hình:

```yaml
# element_registry.yaml (trích)
elements:
  - id: "fire"
    name: "Fire"
    aliases:
      vi: "hỏa"
      zh_pinyin: "huo"
    category: "five_elements"
    is_active: true
```

## 🎯 **Element Categories**

### **1. Universal Elements**

| Element | ID | Category | Status Effect | Damage Type | Defense Type | Special |
|---------|----|---------|---------------|-------------|--------------|---------|
| **Omni** | `omni` | Universal | None | Universal | Balanced | Baseline |

### **2. Five Elements (Ngũ Hành)**

| Element | ID | Category | Status Effect | Damage Type | Defense Type | Special |
|---------|----|---------|---------------|-------------|--------------|---------|
| **Kim (Metal)** | `metal` | Five Elements | Bleeding | Physical | High Defense | Penetration |
| **Mộc (Wood)** | `wood` | Five Elements | Poison | Nature | Medium Defense | Growth |
| **Thủy (Water)** | `water` | Five Elements | Slow | Water | Medium Defense | Flexibility |
| **Hỏa (Fire)** | `fire` | Five Elements | Burning | Fire | Low Defense | Destruction |
| **Thổ (Earth)** | `earth` | Five Elements | Petrification | Earth | High Defense | Stability |

### **2. Yin-Yang Elements**

| Element | ID | Category | Status Effect | Damage Type | Defense Type | Special |
|---------|----|---------|---------------|-------------|--------------|---------|
| **Âm (Yin)** | `yin` | Yin-Yang | Chill | Cold | High Defense | Passive |
| **Dương (Yang)** | `yang` | Yin-Yang | Frenzy | Heat | Low Defense | Active |

### **3. Light & Dark Elements**

| Element | ID | Category | Status Effect | Damage Type | Defense Type | Special |
|---------|----|---------|---------------|-------------|--------------|---------|
| **Light** | `light` | Light/Dark | Purification | Holy | High Defense | Healing |
| **Dark** | `dark` | Light/Dark | Corruption | Shadow | Low Defense | Destruction |
| **Holy** | `holy` | Light/Dark | Blessing | Divine | High Defense | Protection |
| **Unholy** | `unholy` | Light/Dark | Curse | Demonic | Low Defense | Corruption |

### **4. Life & Death Elements**

| Element | ID | Category | Status Effect | Damage Type | Defense Type | Special |
|---------|----|---------|---------------|-------------|--------------|---------|
| **Life** | `life` | Life/Death | Regeneration | Nature | Medium Defense | Healing |
| **Death** | `death` | Life/Death | Decay | Shadow | Low Defense | Destruction |
| **Nature** | `nature` | Life/Death | Growth | Plant | Medium Defense | Healing |
| **Necromancy** | `necromancy` | Life/Death | Undeath | Dark | Low Defense | Undead |

### **5. Time & Space Elements**

| Element | ID | Category | Status Effect | Damage Type | Defense Type | Special |
|---------|----|---------|---------------|-------------|--------------|---------|
| **Time** | `time` | Time/Space | Temporal Distortion | Arcane | Medium Defense | Speed |
| **Space** | `space` | Time/Space | Spatial Lock | Arcane | High Defense | Control |
| **Void** | `void` | Time/Space | Entropy | Dark | Low Defense | Absorption |
| **Chaos** | `chaos` | Time/Space | Randomization | Primal | Low Defense | Unpredictable |

### **6. Physical Elements**

| Element | ID | Category | Status Effect | Damage Type | Defense Type | Special |
|---------|----|---------|---------------|-------------|--------------|---------|
| **Physical** | `physical` | Physical | Stun | Blunt | High Defense | Raw Power |
| **Slashing** | `slashing` | Physical | Bleeding | Sharp | Medium Defense | Precision |
| **Piercing** | `piercing` | Physical | Armor Penetration | Sharp | Low Defense | Penetration |
| **Blunt** | `blunt` | Physical | Concussion | Crushing | High Defense | Impact |
| **Crushing** | `crushing` | Physical | Fracture | Crushing | High Defense | Devastation |

### **7. Magical Elements**

| Element | ID | Category | Status Effect | Damage Type | Defense Type | Special |
|---------|----|---------|---------------|-------------|--------------|---------|
| **Arcane** | `arcane` | Magical | Mana Burn | Mystical | Medium Defense | Magic |
| **Mystical** | `mystical` | Magical | Confusion | Psychic | Low Defense | Illusion |
| **Spiritual** | `spiritual` | Magical | Soul Drain | Ethereal | Medium Defense | Soul |
| **Mental** | `mental` | Magical | Mind Control | Psychic | Low Defense | Control |
| **Psychic** | `psychic` | Magical | Charm | Mental | Low Defense | Influence |

### **8. Cultivation Elements**

| Element | ID | Category | Status Effect | Damage Type | Defense Type | Special |
|---------|----|---------|---------------|-------------|--------------|---------|
| **Qi** | `qi` | Cultivation | Qi Disruption | Energy | Medium Defense | Internal |
| **Dao** | `dao` | Cultivation | Dao Suppression | Conceptual | High Defense | Philosophy |
| **Profound** | `profound` | Cultivation | Profound Seal | Mystical | High Defense | Wisdom |
| **Karma** | `karma` | Cultivation | Karma Binding | Fate | Medium Defense | Destiny |
| **Fate** | `fate` | Cultivation | Fate Lock | Destiny | High Defense | Predestination |

## 🔥 **Status Effects Summary**

### **Damage over Time (DoT) Effects**

| Status Effect | Element | Damage Type | Duration | Intensity | Stackable |
|---------------|---------|-------------|----------|-----------|-----------|
| **Burning** | Fire | Fire | 8s | 1.0x | Yes (5) |
| **Poison** | Wood | Nature | 10s | 1.0x | Yes (3) |
| **Bleeding** | Metal | Physical | 6s | 1.0x | Yes (5) |
| **Corruption** | Dark | Shadow | 12s | 1.2x | Yes (3) |
| **Decay** | Death | Shadow | 15s | 1.5x | Yes (2) |
| **Entropy** | Void | Dark | 20s | 2.0x | No |

### **Movement Effects**

| Status Effect | Element | Effect | Duration | Intensity | Stackable |
|---------------|---------|--------|----------|-----------|-----------|
| **Slow** | Water | -50% Movement | 8s | 1.0x | Yes (3) |
| **Petrification** | Earth | -80% Movement | 12s | 1.5x | No |
| **Spatial Lock** | Space | -100% Movement | 6s | 1.0x | No |
| **Temporal Distortion** | Time | +100% Movement | 10s | 1.0x | No |

### **Control Effects**

| Status Effect | Element | Effect | Duration | Intensity | Stackable |
|---------------|---------|--------|----------|-----------|-----------|
| **Confusion** | Mental | Wrong Target | 5s | 1.0x | No |
| **Charm** | Psychic | Control | 8s | 1.0x | No |
| **Mind Control** | Mental | Full Control | 3s | 1.0x | No |
| **Soul Drain** | Spiritual | Stat Drain | 15s | 1.2x | Yes (2) |

### **Healing Effects**

| Status Effect | Element | Effect | Duration | Intensity | Stackable |
|---------------|---------|--------|----------|-----------|-----------|
| **Regeneration** | Life | +HP/tick | 20s | 1.0x | Yes (3) |
| **Purification** | Light | +HP/tick + Cleanse | 15s | 1.2x | Yes (2) |
| **Blessing** | Holy | +Stats + Cleanse | 30s | 1.5x | No |
| **Growth** | Nature | +HP/tick + Stats | 25s | 1.3x | Yes (2) |

## 🔄 **Element Interactions Matrix**

### **Five Elements Cycle**

#### **Generating Cycle (Tương Sinh)**
```
Kim → Thủy → Mộc → Hỏa → Thổ → Kim
1.2x damage multiplier
```

#### **Overcoming Cycle (Tương Khắc)**
```
Kim → Mộc → Thổ → Thủy → Hỏa → Kim
1.5x damage multiplier
```

### **Light vs Dark Interactions**

| Attacker | Defender | Multiplier | Description |
|----------|----------|------------|-------------|
| Light | Dark | 1.8x | Light overcomes Dark |
| Dark | Light | 0.3x | Dark weak against Light |
| Holy | Unholy | 2.0x | Holy destroys Unholy |
| Unholy | Holy | 0.2x | Unholy weak against Holy |

### **Life vs Death Interactions**

| Attacker | Defender | Multiplier | Description |
|----------|----------|------------|-------------|
| Life | Death | 1.4x | Life overcomes Death |
| Death | Life | 0.6x | Death weak against Life |
| Nature | Necromancy | 1.6x | Nature purifies Undead |
| Necromancy | Nature | 0.4x | Undead weak against Nature |

### **Time vs Space Interactions**

| Attacker | Defender | Multiplier | Description |
|----------|----------|------------|-------------|
| Time | Space | 1.0x | Balanced interaction |
| Space | Time | 1.0x | Balanced interaction |
| Void | Chaos | 1.2x | Void absorbs Chaos |
| Chaos | Void | 0.8x | Chaos weak against Void |

## 📊 **Derived Stats Summary**

### **Core Combat Stats**

| Stat Type | Description | Scaling | Priority |
|-----------|-------------|---------|----------|
| **PowerPoint** | Attack power | 1.0x | High |
| **DefensePoint** | Defense value | 1.0x | High |
| **CritRate** | Critical hit chance | 0.5% per point | High |
| **CritDamage** | Critical damage multiplier | 1% per point | High |
| **AccurateRate** | Hit accuracy | 1% per point | Medium |
| **DodgeRate** | Dodge chance | 1% per point | Medium |

### **Status Effect Stats**

| Stat Type | Description | Scaling | Priority |
|-----------|-------------|---------|----------|
| **StatusProbability** | Chance to apply status | 0.1% per point | High |
| **StatusDuration** | Status effect duration | 0.1s per point | Medium |
| **StatusIntensity** | Status effect strength | 1% per point | High |
| **StatusResistance** | Resist status effects | 0.1% per point | High |
| **StatusDurationReduction** | Reduce status duration | 0.1s per point | Medium |
| **StatusIntensityReduction** | Reduce status intensity | 1% per point | Medium |

## 🎮 **Game Mechanics Summary**

### **Combat Applications**

#### **Offensive Elements**
- **Fire**: High damage, burning DoT, low defense
- **Lightning**: High crit rate, chain damage, medium defense
- **Dark**: High damage, corruption DoT, low defense
- **Death**: High damage, decay DoT, low defense
- **Chaos**: Random effects, unpredictable, low defense

#### **Defensive Elements**
- **Earth**: High defense, petrification, low mobility
- **Water**: Medium defense, slow effects, high mobility
- **Light**: High defense, healing, purification
- **Space**: High defense, spatial control, low damage
- **Time**: Medium defense, speed control, temporal effects

#### **Support Elements**
- **Life**: Healing, regeneration, stat boosts
- **Nature**: Growth, healing, environmental effects
- **Holy**: Blessing, purification, protection
- **Spiritual**: Soul effects, ethereal damage
- **Mental**: Control effects, psychic damage

### **Shield Interactions**

#### **Element-Specific Shields**
- **Fire Shield**: Absorbs fire damage, reflects burning
- **Water Shield**: Absorbs water damage, reflects slow
- **Earth Shield**: Absorbs earth damage, reflects petrification
- **Light Shield**: Absorbs light damage, reflects purification
- **Dark Shield**: Absorbs dark damage, reflects corruption

#### **Universal Shields**
- **Physical Shield**: Absorbs all physical damage
- **Magical Shield**: Absorbs all magical damage
- **Elemental Shield**: Absorbs all elemental damage
- **Status Shield**: Prevents all status effects

### **Item Attribute Effects**

#### **Weapon Attributes**
- **Elemental Damage**: +20% damage of specific element
- **Status Chance**: +10% chance to apply status effects
- **Crit Rate**: +5% critical hit chance
- **Crit Damage**: +15% critical damage multiplier
- **Penetration**: Ignores defense of specific element

#### **Armor Attributes**
- **Elemental Resistance**: +30% resistance to specific element
- **Status Immunity**: 50% chance to resist status effects
- **Defense Boost**: +20% defense against specific element
- **Mobility Penalty**: -10% movement speed for heavy armor

#### **Accessory Attributes**
- **Elemental Power**: +15% power of specific element
- **Status Duration**: +2 seconds status effect duration
- **Crit Rate**: +3% critical hit chance
- **Status Probability**: +5% status effect probability

## 🧪 **Configuration Examples**

### **Element Registry Configuration**

```yaml
# element_registry.yaml
version: 1
elements:
  # Five Elements
  - id: "metal"
    name: "Metal"
    aliases:
      vi: "kim"
      zh_pinyin: "jin"
    category: "five_elements"
    description: "Sharp, hard, penetrating"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "crit_rate"
      - "penetration"
    status_effects:
      - "bleeding"
    is_active: true
    
  - id: "wood"
    name: "Wood"
    aliases:
      vi: "mộc"
      zh_pinyin: "mu"
    category: "five_elements"
    description: "Growing, flexible, nurturing"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "healing_rate"
      - "growth_rate"
    status_effects:
      - "poison"
    is_active: true
    
  # Light & Dark
  - id: "light"
    name: "Light"
    category: "light_dark"
    description: "Holy, healing, purifying"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "healing_rate"
      - "purification_rate"
    status_effects:
      - "purification"
    is_active: true
    
  - id: "dark"
    name: "Dark"
    category: "light_dark"
    description: "Evil, corrupting, destructive"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "corruption_rate"
      - "decay_rate"
    status_effects:
      - "corruption"
    is_active: true
```

### **Status Effect Configuration**

```yaml
# status_effects.yaml
version: 1
status_effects:
  # Fire - Burning
  - element: "fire"
    name: "burning"
    type: "dot"
    base_probability: 0.15
    base_duration: 8.0
    base_intensity: 1.0
    max_duration: 30.0
    max_intensity: 3.0
    tick_interval: 1.0
    max_stacks: 5
    stackable: true
    refresh_duration: true
    
  # Water - Slow
  - element: "water"
    name: "slow"
    type: "movement"
    base_probability: 0.20
    base_duration: 8.0
    base_intensity: 0.5
    max_duration: 20.0
    max_intensity: 1.0
    tick_interval: 0.0
    max_stacks: 3
    stackable: true
    refresh_duration: true
    
  # Earth - Petrification
  - element: "earth"
    name: "petrification"
    type: "control"
    base_probability: 0.10
    base_duration: 12.0
    base_intensity: 0.8
    max_duration: 30.0
    max_intensity: 1.0
    tick_interval: 0.0
    max_stacks: 1
    stackable: false
    refresh_duration: false
```

### **Element Interactions Configuration**

```yaml
# element_interactions.yaml
version: 1
interactions:
  # Five Elements - Generating Cycle
  - attacker: "metal"
    defender: "water"
    multiplier: 1.2
    type: "generating"
    description: "Metal generates Water"
    
  - attacker: "water"
    defender: "wood"
    multiplier: 1.2
    type: "generating"
    description: "Water generates Wood"
    
  # Five Elements - Overcoming Cycle
  - attacker: "metal"
    defender: "wood"
    multiplier: 1.5
    type: "overcoming"
    description: "Metal overcomes Wood"
    
  - attacker: "wood"
    defender: "earth"
    multiplier: 1.5
    type: "overcoming"
    description: "Wood overcomes Earth"
    
  # Light vs Dark
  - attacker: "light"
    defender: "dark"
    multiplier: 1.8
    type: "special"
    description: "Light overcomes Dark"
    
  - attacker: "dark"
    defender: "light"
    multiplier: 0.3
    type: "special"
    description: "Dark weak against Light"
    
  # Life vs Death
  - attacker: "life"
    defender: "death"
    multiplier: 1.4
    type: "special"
    description: "Life overcomes Death"
    
  - attacker: "death"
    defender: "life"
    multiplier: 0.6
    type: "special"
    description: "Death weak against Life"
```

## 🎯 **Implementation Guidelines**

### **For Developers**

#### **Element System Integration**
1. **Element Registry**: Register all elements with their properties
2. **Status Effects**: Implement status effect calculation and processing
3. **Interactions**: Implement element interaction matrix
4. **Derived Stats**: Calculate derived stats based on element properties
5. **Performance**: Optimize calculations for real-time gameplay

#### **Code Structure**
```rust
// Element system structure
pub struct ElementSystem {
    registry: ElementRegistry,
    status_engine: StatusEffectEngine,
    interaction_matrix: InteractionMatrix,
    derived_stats_calculator: DerivedStatsCalculator,
}

impl ElementSystem {
    // Calculate element damage including Omni stats
    pub fn calculate_damage(
        &self, 
        attacker: &Actor, 
        target: &Actor, 
        element: &str
    ) -> f64 {
        // Get Omni stats
        let attacker_omni_power = attacker.get_omni_stat(DerivedStatType::PowerPoint);
        let target_omni_defense = target.get_omni_stat(DerivedStatType::DefensePoint);
        
        // Get element stats
        let attacker_element_power = attacker.get_element_stat(element, DerivedStatType::PowerPoint);
        let target_element_defense = target.get_element_stat(element, DerivedStatType::DefensePoint);
        
        // Calculate total stats
        let total_attacker_power = attacker_omni_power + attacker_element_power;
        let total_target_defense = target_omni_defense + target_element_defense;
        
        // Calculate damage
        let stat_difference = total_attacker_power - total_target_defense;
        base_damage + stat_difference.max(0.0)
    }
    
    // Apply status effects including Omni stats
    pub fn apply_status_effects(
        &self, 
        attacker: &Actor, 
        target: &Actor, 
        element: &str
    ) -> Vec<StatusEffect> {
        // Get Omni status stats
        let attacker_omni_prob = attacker.get_omni_stat(DerivedStatType::StatusProbability);
        let target_omni_resistance = target.get_omni_stat(DerivedStatType::StatusResistance);
        
        // Get element status stats
        let attacker_element_prob = attacker.get_element_stat(element, DerivedStatType::StatusProbability);
        let target_element_resistance = target.get_element_stat(element, DerivedStatType::StatusResistance);
        
        // Calculate total probability and resistance
        let total_probability = attacker_omni_prob + attacker_element_prob;
        let total_resistance = target_omni_resistance + target_element_resistance;
        
        // Apply status effects based on probability
        self.apply_status_effects_with_probability(total_probability, total_resistance, element)
    }
    
    // Get element interactions
    pub fn get_interaction_multiplier(&self, attacker_element: &str, defender_element: &str) -> f64;
}
```

### **For Game Designers**

#### **Balance Considerations**
1. **Element Diversity**: Ensure all elements have unique roles
2. **Status Effects**: Balance status effect power and duration
3. **Interactions**: Create meaningful element interactions
4. **Progression**: Design element mastery progression
5. **Player Choice**: Provide meaningful element choices

#### **Design Principles**
- **Rock-Paper-Scissors**: Create counter-play opportunities
- **Risk vs Reward**: Balance power with vulnerability
- **Player Expression**: Allow different playstyles
- **Strategic Depth**: Create complex decision-making
- **Visual Clarity**: Make elements visually distinct

### **For System Architects**

#### **Architecture Considerations**
1. **Modularity**: Design for easy element addition
2. **Performance**: Optimize for high-frequency calculations
3. **Scalability**: Support large numbers of elements
4. **Extensibility**: Allow custom element types
5. **Maintainability**: Keep code clean and documented

#### **Integration Points**
- **Combat System**: Element damage calculation
- **Shield System**: Element resistance and absorption
- **Item System**: Element-based item attributes
- **Race System**: Element-based racial bonuses
- **Talent System**: Element-based talent trees

## 🚀 **Future Enhancements**

### **Planned Features**
1. **Element Fusion**: Combine elements for new effects
2. **Element Mastery**: Advanced element techniques
3. **Environmental Effects**: Element-based world interactions
4. **Element Evolution**: Element progression and transformation
5. **Element Resonance**: Synergy between multiple elements

### **Potential Expansions**
1. **New Element Categories**: Additional element types
2. **Advanced Status Effects**: More complex status mechanics
3. **Element Crafting**: Create custom elements
4. **Element PvP**: Element-based player vs player
5. **Element Events**: Dynamic element-based events

## 📚 **References**

### **Game References**
- **Grim Dawn**: Elemental damage types and interactions
- **Diablo Series**: Status effects and elemental resistances
- **Path of Exile**: Complex elemental interactions
- **World of Warcraft**: Elemental damage and resistances
- **Final Fantasy XIV**: Elemental system and interactions

### **Philosophical References**
- **Five Elements Theory**: Traditional Chinese philosophy
- **Yin-Yang Theory**: Balance and harmony concepts
- **Alchemy**: Western elemental philosophy
- **Taoism**: Natural balance and flow
- **Buddhism**: Karma and fate concepts

### **Technical References**
- **Rust Programming**: System implementation
- **Game Design Patterns**: Common game mechanics
- **Performance Optimization**: Real-time calculation techniques
- **Data Structures**: Efficient element storage and lookup
- **Testing Strategies**: Comprehensive testing approaches

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Complete  
**Maintainer**: Chaos World Team
