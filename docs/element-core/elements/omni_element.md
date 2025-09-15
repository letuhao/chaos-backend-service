# Omni Element

## 📋 **Element Overview**

**Omni Element** is a universal element that represents the fundamental resistance and power that all characters possess across all element types. It serves as a baseline protection and capability that prevents characters from being completely vulnerable to elements they haven't specialized in.

### **Basic Information**
- **Element ID**: `omni`
- **Element Name**: Omni (Toàn Năng)
- **Category**: Universal
- **Classification**: Neutral - Universal, fundamental, baseline
- **Visual**: Translucent white/silver with subtle shimmer effects

### **Lore & Background**
Omni Element represents the fundamental essence that all living beings possess - a basic understanding and resistance to all elemental forces. It's the foundation that prevents complete vulnerability and ensures that even non-specialists have some degree of protection and capability.

## 🌟 **Element Properties**

### **Base Characteristics**
- **Damage Type**: Universal Damage (converts to target element)
- **Damage Scaling**: Low (0.3x base) - Intentionally low
- **Defense Scaling**: Medium (1.0x base) - Balanced baseline
- **Critical Scaling**: Low (0.5x base) - Lower than specialized elements
- **Accuracy Scaling**: Medium (1.0x base) - Balanced baseline

### **Scaling Factors**
```yaml
scaling_factors:
  crit_rate: 50.0       # Lower crit chance than specialized
  crit_damage: 80.0     # Lower crit damage than specialized
  accuracy: 100.0       # Normal accuracy
  defense: 100.0        # Normal defense
  power: 30.0           # Much lower power than specialized
```

### **Special Properties**
- **Universal**: Applies to all element types
- **Baseline**: Provides minimum protection/capability
- **Additive**: Adds to specialized element stats
- **Non-Stacking**: Doesn't multiply with specialized elements
- **Progressive**: Grows with character level and cultivation

## 📊 **Derived Stats**

### **Supported Derived Stats**
```rust
// Omni element supports all derived stats
pub const OMNI_DERIVED_STATS: [DerivedStatType; 12] = [
    // Basic stats
    DerivedStatType::PowerPoint,           // Universal attack power
    DerivedStatType::DefensePoint,         // Universal defense
    DerivedStatType::CritRate,             // Universal crit rate
    DerivedStatType::CritDamage,           // Universal crit damage
    DerivedStatType::AccurateRate,         // Universal accuracy
    DerivedStatType::DodgeRate,            // Universal dodge rate
    
    // Status effect stats
    DerivedStatType::StatusProbability,    // Universal status probability
    DerivedStatType::StatusDuration,       // Universal status duration
    DerivedStatType::StatusIntensity,      // Universal status intensity
    DerivedStatType::StatusResistance,     // Universal status resistance
    DerivedStatType::StatusDurationReduction, // Universal status duration reduction
    DerivedStatType::StatusIntensityReduction, // Universal status intensity reduction
];
```

### **Stat Weights**
```yaml
stat_weights:
  defense_point: 1.0              # Highest priority - protection
  status_resistance: 0.8          # High priority - status protection
  dodge_rate: 0.6                 # Medium priority - avoidance
  power_point: 0.3                # Low priority - minimal offense
  crit_rate: 0.2                  # Very low priority - minimal crit
  status_probability: 0.1         # Minimal status application
```

### **Scaling Formulas**
```rust
// Omni power scaling (very low)
fn calculate_omni_power(base_power: f64, omni_stat: f64, level: f64) -> f64 {
    base_power * (1.0 + omni_stat * 0.005) * (1.0 + level * 0.01) // 0.5% per point + 1% per level
}

// Omni defense scaling (balanced)
fn calculate_omni_defense(base_defense: f64, omni_stat: f64, level: f64) -> f64 {
    base_defense * (1.0 + omni_stat * 0.01) * (1.0 + level * 0.02) // 1% per point + 2% per level
}

// Omni status resistance (high)
fn calculate_omni_status_resistance(base_resistance: f64, omni_stat: f64, level: f64) -> f64 {
    base_resistance + (omni_stat * 0.02) + (level * 0.01) // 2% per point + 1% per level
}
```

## 🔄 **Element Interactions**

### **Universal Application**
- **Applies to All Elements**: Omni stats are added to all element calculations
- **No Special Interactions**: Omni doesn't have generating/overcoming relationships
- **Additive Only**: Always adds, never multiplies
- **Baseline Protection**: Provides minimum protection against all elements

### **Calculation Examples**
```rust
// Final damage calculation
fn calculate_final_damage(
    attacker_omni_power: f64,
    attacker_fire_power: f64,
    defender_omni_defense: f64,
    defender_fire_defense: f64,
    base_damage: f64
) -> f64 {
    let total_attacker_power = attacker_omni_power + attacker_fire_power;
    let total_defender_defense = defender_omni_defense + defender_fire_defense;
    
    let damage = (total_attacker_power - total_defender_defense).max(0.0);
    base_damage + damage
}

// Final status resistance calculation
fn calculate_final_status_resistance(
    defender_omni_resistance: f64,
    defender_fire_resistance: f64,
    attacker_omni_probability: f64,
    attacker_fire_probability: f64
) -> f64 {
    let total_resistance = defender_omni_resistance + defender_fire_resistance;
    let total_probability = attacker_omni_probability + attacker_fire_probability;
    
    (total_probability - total_resistance).max(0.0).min(1.0)
}
```

## ⚔️ **Game Mechanics**

### **Combat Applications**

#### **Offensive Capabilities**
- **Minimal Damage**: Very low damage contribution
- **Universal Application**: Applies to all element types
- **Level Scaling**: Grows with character progression
- **Cultivation Bonus**: Benefits from all cultivation systems

#### **Defensive Capabilities**
- **Baseline Protection**: Provides minimum protection against all elements
- **Status Resistance**: Reduces status effect probability
- **Universal Defense**: Works against all element types
- **Progressive Growth**: Improves with character development

### **Cultivation System Integration**

#### **Qi Cultivation**
```yaml
qi_cultivation_omni_bonus:
  qi_refining: 0.1      # 10% omni stats per qi refining level
  qi_condensation: 0.15  # 15% omni stats per qi condensation level
  qi_core: 0.2          # 20% omni stats per qi core level
  qi_nascent: 0.25      # 25% omni stats per qi nascent level
```

#### **Body Cultivation**
```yaml
body_cultivation_omni_bonus:
  muscle_tempering: 0.08   # 8% omni stats per muscle tempering level
  bone_forging: 0.12       # 12% omni stats per bone forging level
  organ_refining: 0.15     # 15% omni stats per organ refining level
  blood_transformation: 0.18 # 18% omni stats per blood transformation level
```

#### **Spiritual Cultivation**
```yaml
spiritual_cultivation_omni_bonus:
  soul_nurturing: 0.1      # 10% omni stats per soul nurturing level
  soul_condensation: 0.15  # 15% omni stats per soul condensation level
  soul_core: 0.2          # 20% omni stats per soul core level
  soul_nascent: 0.25      # 25% omni stats per soul nascent level
```

### **Level Scaling**
```rust
// Omni stats scale with character level
fn calculate_omni_level_bonus(level: f64, cultivation_bonus: f64) -> f64 {
    let base_bonus = level * 0.01; // 1% per level
    let cultivation_multiplier = 1.0 + cultivation_bonus;
    base_bonus * cultivation_multiplier
}

// Example: Level 100 character with 50% cultivation bonus
// Level bonus = 100 * 0.01 * 1.5 = 1.5x omni stats
```

## 🧪 **Configuration Examples**

### **YAML Configuration**
```yaml
# omni_element.yaml
element:
  id: "omni"
  name: "Omni"
  category: "universal"
  description: "Universal element providing baseline protection and capability"
  
  # Base properties (intentionally low)
  base_damage: 20.0
  base_defense: 50.0
  base_crit_rate: 0.05
  base_crit_damage: 1.2
  base_accuracy: 0.7
  
  # Scaling factors (balanced for baseline)
  scaling_factors:
    power: 0.3
    defense: 1.0
    crit_rate: 0.5
    crit_damage: 0.8
    accuracy: 1.0
  
  # Derived stats (all supported)
  derived_stats:
    - "power_point"
    - "defense_point"
    - "crit_rate"
    - "crit_damage"
    - "accurate_rate"
    - "dodge_rate"
    - "status_probability"
    - "status_duration"
    - "status_intensity"
    - "status_resistance"
    - "status_duration_reduction"
    - "status_intensity_reduction"
  
  # Level scaling
  level_scaling:
    power: 0.01          # 1% per level
    defense: 0.02        # 2% per level
    crit_rate: 0.001     # 0.1% per level
    crit_damage: 0.005   # 0.5% per level
    status_resistance: 0.01 # 1% per level
  
  # Cultivation bonuses
  cultivation_bonuses:
    qi_cultivation: 0.1      # 10% per cultivation level
    body_cultivation: 0.08   # 8% per cultivation level
    spiritual_cultivation: 0.1 # 10% per cultivation level
  
  # No status effects (omni doesn't cause status)
  status_effects: []
  
  # No special interactions (omni is additive only)
  interactions: []
```

### **Integration Example**
```rust
// Example: Fire damage calculation with Omni
fn calculate_fire_damage(
    attacker: &Actor,
    defender: &Actor,
    base_damage: f64
) -> f64 {
    // Get omni stats
    let attacker_omni_power = attacker.get_omni_stat(DerivedStatType::PowerPoint);
    let defender_omni_defense = defender.get_omni_stat(DerivedStatType::DefensePoint);
    
    // Get fire stats
    let attacker_fire_power = attacker.get_element_stat("fire", DerivedStatType::PowerPoint);
    let defender_fire_defense = defender.get_element_stat("fire", DerivedStatType::DefensePoint);
    
    // Calculate total stats
    let total_attacker_power = attacker_omni_power + attacker_fire_power;
    let total_defender_defense = defender_omni_defense + defender_fire_defense;
    
    // Calculate final damage
    let stat_difference = total_attacker_power - total_defender_defense;
    let final_damage = base_damage + stat_difference.max(0.0);
    
    final_damage
}
```

## 🎯 **Balance Considerations**

### **Strengths**
- **Universal Protection**: Provides baseline protection against all elements
- **Progressive Growth**: Scales with character level and cultivation
- **Prevents One-Shot**: Ensures characters aren't completely vulnerable
- **Cultivation Integration**: Benefits from all cultivation systems

### **Weaknesses**
- **Low Offensive Power**: Minimal damage contribution
- **No Specialization**: Can't replace specialized element training
- **Diminishing Returns**: Less effective at high levels
- **Resource Cost**: Still requires investment to be effective

### **Balance Recommendations**
- **Keep Omni Stats Low**: Ensure specialized elements remain superior
- **Progressive Scaling**: Make omni stats grow with character progression
- **Cultivation Integration**: Reward all cultivation paths equally
- **Baseline Protection**: Ensure minimum viable protection

## 🧪 **Testing & Validation**

### **Unit Tests**
```rust
#[cfg(test)]
mod omni_element_tests {
    use super::*;
    
    #[test]
    fn test_omni_damage_calculation() {
        let omni_element = OmniElement::new();
        let base_damage = 100.0;
        let omni_power = 50.0;
        let level = 10.0;
        
        let final_damage = omni_element.calculate_damage(base_damage, omni_power, level);
        // Should be: 100 + (50 * 0.005) * (1 + 10 * 0.01) = 100 + 0.25 * 1.1 = 100.275
        assert_eq!(final_damage, 100.275);
    }
    
    #[test]
    fn test_omni_defense_calculation() {
        let omni_element = OmniElement::new();
        let base_defense = 100.0;
        let omni_defense = 50.0;
        let level = 10.0;
        
        let final_defense = omni_element.calculate_defense(base_defense, omni_defense, level);
        // Should be: 100 * (1 + 50 * 0.01) * (1 + 10 * 0.02) = 100 * 1.5 * 1.2 = 180
        assert_eq!(final_defense, 180.0);
    }
    
    #[test]
    fn test_omni_status_resistance() {
        let omni_element = OmniElement::new();
        let base_resistance = 0.1;
        let omni_resistance = 50.0;
        let level = 10.0;
        
        let final_resistance = omni_element.calculate_status_resistance(
            base_resistance, 
            omni_resistance, 
            level
        );
        // Should be: 0.1 + (50 * 0.02) + (10 * 0.01) = 0.1 + 1.0 + 0.1 = 1.2
        assert_eq!(final_resistance, 1.2);
    }
}
```

### **Integration Tests**
```rust
#[cfg(test)]
mod omni_element_integration_tests {
    use super::*;
    
    #[test]
    fn test_omni_fire_damage_integration() {
        let combat_system = CombatSystem::new();
        
        // Create attacker with omni + fire stats
        let mut attacker = Actor::new();
        attacker.set_omni_stat(DerivedStatType::PowerPoint, 30.0);
        attacker.set_element_stat("fire", DerivedStatType::PowerPoint, 70.0);
        
        // Create defender with omni + fire stats
        let mut defender = Actor::new();
        defender.set_omni_stat(DerivedStatType::DefensePoint, 20.0);
        defender.set_element_stat("fire", DerivedStatType::DefensePoint, 80.0);
        
        // Calculate fire damage
        let damage_result = combat_system.calculate_fire_damage(
            &attacker,
            &defender,
            100.0
        ).unwrap();
        
        // Total attacker power: 30 + 70 = 100
        // Total defender defense: 20 + 80 = 100
        // Final damage: 100 + (100 - 100) = 100
        assert_eq!(damage_result.final_damage, 100.0);
    }
    
    #[test]
    fn test_omni_status_resistance_integration() {
        let status_system = StatusEffectSystem::new();
        
        // Create attacker with omni + fire status probability
        let mut attacker = Actor::new();
        attacker.set_omni_stat(DerivedStatType::StatusProbability, 0.05);
        attacker.set_element_stat("fire", DerivedStatType::StatusProbability, 0.15);
        
        // Create defender with omni + fire status resistance
        let mut defender = Actor::new();
        defender.set_omni_stat(DerivedStatType::StatusResistance, 0.1);
        defender.set_element_stat("fire", DerivedStatType::StatusResistance, 0.2);
        
        // Calculate burning probability
        let probability = status_system.calculate_burning_probability(
            &attacker,
            &defender
        ).unwrap();
        
        // Total probability: 0.05 + 0.15 = 0.2
        // Total resistance: 0.1 + 0.2 = 0.3
        // Final probability: max(0.2 - 0.3, 0) = 0
        assert_eq!(probability, 0.0);
    }
}
```

## 🚀 **Future Enhancements**

### **Planned Features**
- **Omni Mastery**: Advanced omni techniques
- **Element Harmony**: Omni-based element fusion
- **Universal Resistance**: Advanced omni resistance mechanics
- **Cultivation Synergy**: Omni-based cultivation paths

### **Potential Changes**
- **Balance Adjustments**: Based on gameplay data
- **Scaling Improvements**: Better level scaling formulas
- **Cultivation Integration**: Deeper cultivation system integration
- **Visual Effects**: Better omni visual representation

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Complete  
**Maintainer**: Chaos World Team
