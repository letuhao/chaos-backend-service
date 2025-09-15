# Element Core Implementation Notes

## 📋 **Tổng Quan**

Tài liệu này chứa các điểm lưu ý quan trọng cho việc implement Element Core system, bao gồm integration với Actor Core, Combat Core, và các systems khác.

## 🎯 **Key Implementation Points**

### **1. Omni Element Foundation**

#### **Anti-One-Trick Design**
- **Omni Stats**: Cộng dồn "baseline" cho mọi phép tính (offense/defense/status)
- **Additive Only**: Không nhân chồng để tránh build đơn nguyên tố bị "lụt" khi thiếu stat nền
- **Baseline Protection**: Đảm bảo mọi character đều có khả năng chống chịu cơ bản

#### **Implementation Notes**
```rust
// Omni stats chỉ cộng, không nhân
let total_power = omni_power + element_power;  // ✅ Correct
let total_power = omni_power * element_power;  // ❌ Wrong - causes snowball
```

### **2. Status Effect System Integration**

#### **6 Derived Stats for Status**
- **StatusProbability**: Xác suất gây status effect
- **StatusResistance**: Kháng status effect
- **StatusDuration**: Thời gian kéo dài status effect
- **StatusDurationReduction**: Giảm thời gian status effect
- **StatusIntensity**: Cường độ status effect
- **StatusIntensityReduction**: Giảm cường độ status effect

#### **Actor Core Integration Requirements**
```rust
// Cần map vào Actor Core Derived system
pub enum DerivedStatType {
    // ... existing stats
    StatusProbability,
    StatusResistance,
    StatusDuration,
    StatusDurationReduction,
    StatusIntensity,
    StatusIntensityReduction,
}

// Cần thêm clamping
pub struct StatusClamping {
    pub max_duration: f64,
    pub max_intensity: f64,
    pub min_duration: f64,
    pub min_intensity: f64,
}
```

### **3. Combat Core Integration**

#### **Hybrid Architecture Flow**
```rust
// 1. Get Omni + Element stats
let omni_stats = element_core.get_omni_stats(actor);
let element_stats = element_core.get_element_stats(actor, element_type);

// 2. Combine stats
let total_power = omni_stats.power + element_stats.power;
let total_defense = omni_stats.defense + element_stats.defense;

// 3. Calculate damage with element multiplier
let base_damage = calculate_base_damage(action);
let element_multiplier = element_core.get_damage_multiplier(element_type, target_element);
let final_damage = (base_damage * element_multiplier) * (1.0 - resist_after_pen);

// 4. Apply status effects after hit success
if hit_success && status_probability > threshold {
    let status_effects = element_core.calculate_status_effects(attacker, target, element_type);
    apply_status_effects(status_effects);
}
```

#### **Status Application Flow**
```rust
// Status chỉ apply khi hit thành công
pub struct StatusEffectConfig {
    pub requires_hit: bool,  // Mới thêm: status chỉ apply khi hit
    pub base_probability: f64,
    pub max_duration: f64,
    pub max_intensity: f64,
    pub stackable: bool,
    pub max_stacks: u32,
    pub refresh_duration: bool,
}
```

### **4. Element-Specific Sigmoid Parameters**

#### **Per-Element Scaling**
```rust
// Mỗi element có scaling parameters riêng
pub struct ElementSigmoidConfig {
    pub element_type: String,
    pub crit_scaling_factor: f64,
    pub accuracy_scaling_factor: f64,
    pub status_scaling_factor: f64,
    pub steepness: f64,
}

// Ví dụ: Fire có crit scaling cao
let fire_config = ElementSigmoidConfig {
    element_type: "fire".to_string(),
    crit_scaling_factor: 120.0,  // Cao hơn default
    accuracy_scaling_factor: 80.0,
    status_scaling_factor: 100.0,
    steepness: 1.2,
};
```

### **5. Damage Composition Law**

#### **Correct Order of Operations**
```rust
// 1. Base damage calculation
let base_damage = calculate_base_damage(action);

// 2. Apply element multiplier
let element_multiplier = get_element_multiplier(attacker_element, target_element);
let element_damage = base_damage * element_multiplier;

// 3. Apply resistance (after penetration)
let resistance = calculate_resistance_after_penetration(target, element_type);
let final_damage = element_damage * (1.0 - resistance);

// 4. Apply DoT/CC after damage calculation
if should_apply_status {
    apply_status_effects(attacker, target, element_type);
}
```

### **6. Configuration & Clamping**

#### **Status Effect Caps**
```yaml
# status_effects.yaml
fire:
  burning:
    max_duration: 30.0
    max_intensity: 100.0
    stackable: true
    max_stacks: 5
    refresh_duration: true
    requires_hit: true  # Mới thêm
```

#### **Element Interaction Caps**
```yaml
# element_interactions.yaml
fire:
  vs_wood:
    damage_multiplier: 1.5
    status_multiplier: 1.2
  vs_water:
    damage_multiplier: 0.7
    status_multiplier: 0.8
```

### **7. Testing Requirements**

#### **Golden Tests for Intransitive Meta**
```rust
// Test 5 Ngũ Hành pairs
#[test]
fn test_ngu_hanh_interactions() {
    // Fire > Wood > Earth > Water > Fire
    assert!(fire_vs_wood_damage > 1.0);
    assert!(wood_vs_earth_damage > 1.0);
    assert!(earth_vs_water_damage > 1.0);
    assert!(water_vs_fire_damage > 1.0);
}

// Test 4 Light/Dark pairs
#[test]
fn test_light_dark_interactions() {
    // Light > Dark > Light
    assert!(light_vs_dark_damage > 1.0);
    assert!(dark_vs_light_damage > 1.0);
}
```

#### **Property Tests**
```rust
// Status resistance always decreases apply probability
#[test]
fn test_status_resistance_property() {
    let low_resistance = 10.0;
    let high_resistance = 50.0;
    
    let low_prob = calculate_status_probability(attacker_prob, low_resistance);
    let high_prob = calculate_status_probability(attacker_prob, high_resistance);
    
    assert!(high_prob < low_prob);
}
```

## 🚨 **Critical Implementation Notes**

### **1. Omni Additive-Only Rule**
- **NEVER** multiply Omni stats with other stats
- **ALWAYS** add Omni stats to element stats
- **PREVENT** snowball builds when combining with amplifiers

### **2. Status Hit Dependency**
- **ALWAYS** check `requires_hit: true` before applying status
- **NEVER** apply status on miss (trừ khi config cho phép)
- **LOG** status application events for debugging

### **3. Element Interaction Matrix**
- **ENSURE** intransitive meta (no element dominates all)
- **BALANCE** interaction multipliers to prevent one-trick builds
- **TEST** all interaction pairs for proper scaling

### **4. Clamping & Validation**
- **CLAMP** all duration/intensity values within min/max bounds
- **VALIDATE** all input values before calculations
- **LOG** clamping events for balance analysis

### **5. Resource Manager Integration**

#### **Primary Stats to Derived Stats Mapping**
```rust
// Map primary stats to element-specific derived stats
pub fn calculate_derived_stats_from_primary(
    &self,
    primary_stats: &HashMap<String, f64>,
    element_type: &str,
) -> HashMap<DerivedStatType, f64> {
    let mut derived_stats = HashMap::new();
    
    // Vitality → Defense + Status Resistance
    if let Some(vitality) = primary_stats.get("vitality") {
        derived_stats.insert(DerivedStatType::DefensePoint, vitality * 2.0);
        derived_stats.insert(DerivedStatType::StatusResistance, vitality * 0.5);
    }
    
    // Intelligence → Power + Status Probability
    if let Some(intelligence) = primary_stats.get("intelligence") {
        derived_stats.insert(DerivedStatType::PowerPoint, intelligence * 1.5);
        derived_stats.insert(DerivedStatType::StatusProbability, intelligence * 0.3);
    }
    
    // Apply element-specific scaling
    self.apply_element_scaling(&mut derived_stats, element_type);
    
    derived_stats
}
```

#### **Event-Driven Stats Change Propagation**
```rust
// Resource Manager triggers stats change events
pub struct StatsChangeEvent {
    pub actor_id: String,
    pub system_id: String,
    pub primary_stats: HashMap<String, f64>,
    pub timestamp: u64,
}

// Element-Core responds to stats change events
impl ElementCore {
    pub fn handle_stats_change_event(&self, event: &StatsChangeEvent) -> Result<(), ElementError> {
        // Recalculate derived stats for all elements
        for element_type in self.get_all_element_types() {
            let derived_stats = self.calculate_derived_stats_from_primary(
                &event.primary_stats,
                element_type,
            );
            
            // Update actor's element stats
            self.update_actor_element_stats(
                &event.actor_id,
                element_type,
                derived_stats,
            )?;
        }
        
        // Recalculate Omni stats
        let omni_stats = self.calculate_omni_stats_from_primary(&event.primary_stats);
        self.update_actor_omni_stats(&event.actor_id, omni_stats)?;
        
        Ok(())
    }
}
```

#### **Critical Implementation Requirements**
- **EVENT ORDER**: Stats change events must be processed in correct order
- **CACHE INVALIDATION**: Proper cache invalidation to ensure consistency
- **BATCH UPDATES**: Batch stats updates for performance
- **PRIMARY STATS MAPPING**: Correct mapping from primary stats to derived stats
- **OMNI STATS CALCULATION**: Omni stats must be recalculated when primary stats change

### **6. Loop Prevention (CRITICAL)**

#### **Stat Categories & Immutability**
```rust
// Separate stats into categories to prevent loops
pub enum ElementStatCategory {
    Base,        // Immutable during combat
    Equipment,   // Can change with equipment
    BuffDebuff,  // Can change with effects
    Derived,     // Calculated from above
}

// Base stats are immutable during combat
pub struct BaseElementStats {
    pub vitality: f64,
    pub intelligence: f64,
    pub strength: f64,
    pub agility: f64,
    pub wisdom: f64,
    pub constitution: f64,
}
```

#### **Event Source Validation**
```rust
// Validate event sources to prevent loops
pub enum BaseStatsChangeSource {
    LevelUp,
    EquipmentChange,
    TalentChange,
    // NOT from effects or derived stats
}

pub enum EffectSource {
    BuffApplied,
    DebuffApplied,
    StatusEffectApplied,
    // NOT from derived stats
}
```

#### **Calculation Phase Locking**
```rust
// Lock calculation phases to prevent concurrent modifications
pub enum ElementCalculationPhase {
    BaseStats,
    EquipmentStats,
    BuffDebuffStats,
    DerivedStats,
}

impl ElementCore {
    pub fn calculate_actor_stats(&self, actor_id: &str) -> Result<(), ElementError> {
        // Lock phases in order
        let mut calculation_lock = self.calculation_lock.lock().unwrap();
        
        // Phase 1: Base stats
        calculation_lock.lock_phase(ElementCalculationPhase::BaseStats)?;
        let base_stats = self.calculate_base_stats(actor_id)?;
        
        // Phase 2: Equipment stats
        calculation_lock.lock_phase(ElementCalculationPhase::EquipmentStats)?;
        let equipment_stats = self.calculate_equipment_stats(actor_id)?;
        
        // Phase 3: Buff/Debuff stats
        calculation_lock.lock_phase(ElementCalculationPhase::BuffDebuffStats)?;
        let buff_debuff_stats = self.calculate_buff_debuff_stats(actor_id)?;
        
        // Phase 4: Derived stats
        calculation_lock.lock_phase(ElementCalculationPhase::DerivedStats)?;
        let derived_stats = self.calculate_derived_stats(
            &base_stats,
            &equipment_stats,
            &buff_debuff_stats,
        )?;
        
        Ok(())
    }
}
```

#### **Critical Loop Prevention Rules**
- **PRIMARY STATS**: Only changed by level up, equipment change, talent change (IMMUTABLE during combat)
- **EQUIPMENT STATS**: Only changed by equipment change
- **BUFF/DEBUFF EFFECTS**: Only affect derived stats, NEVER primary stats
- **DERIVED STATS**: Calculated from primary + equipment + effects, never modified directly
- **STATUS EFFECTS**: Only affect derived stats, NEVER primary stats
- **EVENT SOURCES**: Validate event sources to prevent invalid modifications
- **CALCULATION PHASES**: Lock phases to prevent concurrent modifications
- **MAX DEPTH**: Limit processing depth to prevent infinite loops

#### **Buff/Debuff → Derived Stats Only Design**
```rust
// Buff/Debuff chỉ tác động vào Derived Stats, không phải Primary Stats
pub struct BuffDebuffEffect {
    pub effect_id: String,
    pub effect_type: EffectType,
    pub derived_stat_modifiers: HashMap<DerivedStatType, f64>,
    pub duration: f64,
    pub intensity: f64,
}

pub enum EffectType {
    // Buff effects - tăng derived stats
    Buff {
        power_bonus: f64,
        defense_bonus: f64,
        crit_rate_bonus: f64,
        crit_damage_bonus: f64,
        accuracy_bonus: f64,
        dodge_bonus: f64,
        status_probability_bonus: f64,
        status_resistance_bonus: f64,
    },
    // Debuff effects - giảm derived stats
    Debuff {
        power_penalty: f64,
        defense_penalty: f64,
        crit_rate_penalty: f64,
        crit_damage_penalty: f64,
        accuracy_penalty: f64,
        dodge_penalty: f64,
        status_probability_penalty: f64,
        status_resistance_penalty: f64,
    },
}

// Primary Stats là immutable, không thể thay đổi bởi effects
pub struct PrimaryStats {
    pub vitality: f64,      // Chỉ thay đổi bởi level up, equipment, talents
    pub intelligence: f64,  // Chỉ thay đổi bởi level up, equipment, talents
    pub strength: f64,      // Chỉ thay đổi bởi level up, equipment, talents
    pub agility: f64,       // Chỉ thay đổi bởi level up, equipment, talents
    pub wisdom: f64,        // Chỉ thay đổi bởi level up, equipment, talents
    pub constitution: f64,  // Chỉ thay đổi bởi level up, equipment, talents
}
```

#### **Status Effects → Derived Stats Only**
```rust
// Status effects chỉ tác động vào derived stats
pub struct StatusEffect {
    pub effect_id: String,
    pub status_type: StatusType,
    pub derived_stat_modifiers: HashMap<DerivedStatType, f64>,
    pub duration: f64,
    pub intensity: f64,
}

pub enum StatusType {
    // Burning - tăng status probability, giảm defense
    Burning { status_probability_bonus: f64, defense_penalty: f64 },
    // Slow - giảm accuracy, giảm dodge
    Slow { accuracy_penalty: f64, dodge_penalty: f64 },
    // Petrification - tăng defense, giảm dodge
    Petrification { defense_bonus: f64, dodge_penalty: f64 },
    // Bleeding - giảm defense, tăng status probability
    Bleeding { defense_penalty: f64, status_probability_bonus: f64 },
    // Poison - giảm status resistance, tăng status intensity
    Poison { status_resistance_penalty: f64, status_intensity_bonus: f64 },
}
```

## 📚 **Related Documents**

- [Element Core Overview](00_Element_Core_Overview.md)
- [Probability Mechanics Design](01_Probability_Mechanics_Design.md)
- [Multi-System Integration Design](02_Multi_System_Integration_Design.md)
- [Status Effect System Design](04_Status_Effect_System_Design.md)
- [Element Summary Comprehensive](05_Element_Summary_Comprehensive.md)
- [Resource Manager Integration Design](07_Resource_Manager_Integration_Design.md)

## 🔄 **Update History**

- **2024-01-XX**: Initial implementation notes based on design review
- **2024-01-XX**: Added Omni additive-only rule enforcement
- **2024-01-XX**: Added status hit dependency requirements
- **2024-01-XX**: Added element-specific sigmoid parameters
- **2024-01-XX**: Added damage composition law requirements
