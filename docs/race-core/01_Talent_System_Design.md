# Talent System Design

## 📋 **Tổng Quan**

Thiên Phú (Talent) là hệ thống đặc biệt của từng chủng tộc, cung cấp các bonus đặc thù cho các element và derived stats. Hệ thống này tạo ra sự đa dạng và đặc trưng riêng biệt cho mỗi chủng tộc trong Chaos World.

## 🎯 **Bản Chất của Thiên Phú**

### **Đặc Điểm Chính:**
- **Chủng tộc đặc thù**: Mỗi chủng tộc có thiên phú riêng
- **Element-focused**: Tập trung vào một hoặc nhiều element cụ thể
- **Derived stats enhancement**: Tăng cường các derived stats của element
- **Cố định**: Không thay đổi trong suốt cuộc đời nhân vật
- **Multiplicative**: Nhân với base stats thay vì cộng

### **Vai Trò trong Game:**
- **Racial Identity**: Tạo bản sắc riêng cho chủng tộc
- **Build Diversity**: Khuyến khích các build khác nhau
- **Element Mastery**: Tăng cường hiệu quả element
- **Strategic Depth**: Tạo chiều sâu chiến thuật

## 🏗️ **Talent System Architecture**

### **Core Structure**
```rust
/// Talent system for racial element affinities
/// 种族元素天赋系统
pub struct TalentSystem {
    pub race_id: String,                    // Race identifier
    pub talent_name: String,                // Talent name (天赋名称)
    pub talent_description: String,         // Talent description (天赋描述)
    pub element_affinities: Vec<ElementAffinity>, // Element affinities (元素亲和)
    pub talent_bonuses: Vec<TalentBonus>,   // Talent bonuses (天赋加成)
    pub unlock_conditions: Vec<UnlockCondition>, // Unlock conditions (解锁条件)
}

/// Element affinity definition
/// 元素亲和定义
pub struct ElementAffinity {
    pub element_id: String,                 // Element ID (元素ID)
    pub affinity_level: f64,                // Affinity level (亲和等级)
    pub affinity_type: AffinityType,        // Affinity type (亲和类型)
    pub derived_stat_bonuses: Vec<DerivedStatBonus>, // Derived stat bonuses (衍生属性加成)
}

/// Affinity types
/// 亲和类型
pub enum AffinityType {
    Natural,        // Natural affinity (自然亲和)
    Innate,         // Innate affinity (天生亲和)
    Awakened,       // Awakened affinity (觉醒亲和)
    Transcendent,   // Transcendent affinity (超越亲和)
}

/// Derived stat bonus
/// 衍生属性加成
pub struct DerivedStatBonus {
    pub stat_name: String,                  // Stat name (属性名称)
    pub bonus_type: BonusType,              // Bonus type (加成类型)
    pub bonus_value: f64,                   // Bonus value (加成值)
    pub scaling_factor: f64,                // Scaling factor (缩放因子)
}

/// Bonus types
/// 加成类型
pub enum BonusType {
    Multiplicative, // Multiplicative bonus (乘法加成)
    Additive,       // Additive bonus (加法加成)
    Percentage,     // Percentage bonus (百分比加成)
    Flat,           // Flat bonus (固定加成)
}
```

## 🔥 **Ví Dụ: Hỏa Tinh Linh (Fire Spirit)**

### **Talent Definition**
```rust
/// Fire Spirit race talent
/// 火精灵种族天赋
pub struct FireSpiritTalent {
    pub race_id: "fire_spirit".to_string(),
    pub talent_name: "Fire Mastery".to_string(),
    pub talent_description: "Natural affinity with fire element, enhanced fire-derived stats".to_string(),
    pub element_affinities: vec![
        ElementAffinity {
            element_id: "fire".to_string(),
            affinity_level: 1.5,                    // 150% base affinity
            affinity_type: AffinityType::Natural,
            derived_stat_bonuses: vec![
                // Core Element Mastery
                DerivedStatBonus {
                    stat_name: "element_mastery".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.2,               // +20% element mastery
                    scaling_factor: 1.0,
                },
                
                // Power & Defense
                DerivedStatBonus {
                    stat_name: "power_point".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.15,              // +15% power point
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "defense_point".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.1,               // +10% defense point
                    scaling_factor: 1.0,
                },
                
                // Critical Stats
                DerivedStatBonus {
                    stat_name: "crit_rate".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.25,              // +25% crit rate
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "crit_damage".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.3,               // +30% crit damage
                    scaling_factor: 1.0,
                },
                
                // Accuracy Stats
                DerivedStatBonus {
                    stat_name: "accurate_rate".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.1,               // +10% accuracy
                    scaling_factor: 1.0,
                },
                
                // Status Effects
                DerivedStatBonus {
                    stat_name: "status_probability".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.2,               // +20% status probability
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "status_intensity".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.25,              // +25% status intensity
                    scaling_factor: 1.0,
                },
                
                // Element Interaction
                DerivedStatBonus {
                    stat_name: "element_penetration".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.15,              // +15% penetration
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "element_amplification".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.2,               // +20% amplification
                    scaling_factor: 1.0,
                },
                
                // Skill Effectiveness
                DerivedStatBonus {
                    stat_name: "attack_skill_effectiveness".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.15,              // +15% attack skill effectiveness
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "status_skill_effectiveness".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.2,               // +20% status skill effectiveness
                    scaling_factor: 1.0,
                },
                
                // Resource Management
                DerivedStatBonus {
                    stat_name: "resource_efficiency".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.1,               // +10% resource efficiency
                    scaling_factor: 1.0,
                },
            ],
        },
    ],
    pub unlock_conditions: vec![
        UnlockCondition {
            condition_type: "race_selection".to_string(),
            condition_value: "fire_spirit".to_string(),
        },
    ],
}
```

## 🌊 **Ví Dụ: Thủy Tinh Linh (Water Spirit)**

### **Talent Definition**
```rust
/// Water Spirit race talent
/// 水精灵种族天赋
pub struct WaterSpiritTalent {
    pub race_id: "water_spirit".to_string(),
    pub talent_name: "Water Mastery".to_string(),
    pub talent_description: "Natural affinity with water element, enhanced water-derived stats".to_string(),
    pub element_affinities: vec![
        ElementAffinity {
            element_id: "water".to_string(),
            affinity_level: 1.5,                    // 150% base affinity
            affinity_type: AffinityType::Natural,
            derived_stat_bonuses: vec![
                // Core Element Mastery
                DerivedStatBonus {
                    stat_name: "element_mastery".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.2,               // +20% element mastery
                    scaling_factor: 1.0,
                },
                
                // Power & Defense (Water focuses on defense)
                DerivedStatBonus {
                    stat_name: "power_point".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.1,               // +10% power point
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "defense_point".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.25,              // +25% defense point
                    scaling_factor: 1.0,
                },
                
                // Critical Stats (Water focuses on resistance)
                DerivedStatBonus {
                    stat_name: "resist_crit_rate".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.3,               // +30% crit resistance
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "resist_crit_damage".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.25,              // +25% crit damage resistance
                    scaling_factor: 1.0,
                },
                
                // Accuracy Stats (Water focuses on accuracy)
                DerivedStatBonus {
                    stat_name: "accurate_rate".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.2,               // +20% accuracy
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "dodge_rate".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.15,              // +15% dodge rate
                    scaling_factor: 1.0,
                },
                
                // Status Effects (Water focuses on healing)
                DerivedStatBonus {
                    stat_name: "status_resistance".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.25,              // +25% status resistance
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "status_duration_reduction".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.2,               // +20% duration reduction
                    scaling_factor: 1.0,
                },
                
                // Element Interaction (Water focuses on absorption)
                DerivedStatBonus {
                    stat_name: "element_absorption".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.3,               // +30% absorption
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "element_reduction".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.2,               // +20% reduction
                    scaling_factor: 1.0,
                },
                
                // Skill Effectiveness (Water focuses on healing and support)
                DerivedStatBonus {
                    stat_name: "healing_skill_effectiveness".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.25,              // +25% healing effectiveness
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "support_skill_effectiveness".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.2,               // +20% support effectiveness
                    scaling_factor: 1.0,
                },
                
                // Resource Management (Water focuses on regeneration)
                DerivedStatBonus {
                    stat_name: "resource_regeneration".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.3,               // +30% resource regeneration
                    scaling_factor: 1.0,
                },
            ],
        },
    ],
    pub unlock_conditions: vec![
        UnlockCondition {
            condition_type: "race_selection".to_string(),
            condition_value: "water_spirit".to_string(),
        },
    ],
}
```

## 🐉 **Ví Dụ: Long Tộc (Dragon Race)**

### **Talent Definition**
```rust
/// Dragon race talent
/// 龙族天赋
pub struct DragonTalent {
    pub race_id: "dragon".to_string(),
    pub talent_name: "Dragon Bloodline".to_string(),
    pub talent_description: "Ancient dragon bloodline grants mastery over multiple elements".to_string(),
    pub element_affinities: vec![
        // Fire Affinity
        ElementAffinity {
            element_id: "fire".to_string(),
            affinity_level: 1.3,                    // 130% base affinity
            affinity_type: AffinityType::Innate,
            derived_stat_bonuses: vec![
                DerivedStatBonus {
                    stat_name: "power_point".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.2,               // +20% power point
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "crit_damage".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.25,              // +25% crit damage
                    scaling_factor: 1.0,
                },
            ],
        },
        // Lightning Affinity
        ElementAffinity {
            element_id: "lightning".to_string(),
            affinity_level: 1.2,                    // 120% base affinity
            affinity_type: AffinityType::Innate,
            derived_stat_bonuses: vec![
                DerivedStatBonus {
                    stat_name: "skill_execution_speed".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.15,              // +15% execution speed
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "crit_rate".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.2,               // +20% crit rate
                    scaling_factor: 1.0,
                },
            ],
        },
        // Earth Affinity
        ElementAffinity {
            element_id: "earth".to_string(),
            affinity_level: 1.1,                    // 110% base affinity
            affinity_type: AffinityType::Innate,
            derived_stat_bonuses: vec![
                DerivedStatBonus {
                    stat_name: "defense_point".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.3,               // +30% defense point
                    scaling_factor: 1.0,
                },
                DerivedStatBonus {
                    stat_name: "status_resistance".to_string(),
                    bonus_type: BonusType::Multiplicative,
                    bonus_value: 1.25,              // +25% status resistance
                    scaling_factor: 1.0,
                },
            ],
        },
    ],
    pub unlock_conditions: vec![
        UnlockCondition {
            condition_type: "race_selection".to_string(),
            condition_value: "dragon".to_string(),
        },
    ],
}
```

## 🔧 **Implementation Strategy**

### **Phase 1: Core Structure**
1. **Define TalentSystem struct**
2. **Implement ElementAffinity system**
3. **Create DerivedStatBonus calculations**
4. **Add race-specific talent definitions**

### **Phase 2: Integration**
1. **Integrate with Race Core**
2. **Connect with Element Core**
3. **Add talent application logic**
4. **Implement bonus calculations**

### **Phase 3: Advanced Features**
1. **Add unlock conditions**
2. **Implement talent evolution**
3. **Create talent combinations**
4. **Add balance testing**

## 📈 **Balance Considerations**

### **Bonus Scaling**
- **Multiplicative bonuses**: 1.1x - 1.5x (10% - 50%)
- **Additive bonuses**: 5% - 25% of base value
- **Percentage bonuses**: 5% - 30% of current value
- **Flat bonuses**: 10 - 100 points

### **Element Focus**
- **Single Element**: Higher bonuses (1.2x - 1.5x)
- **Dual Elements**: Medium bonuses (1.1x - 1.3x)
- **Multiple Elements**: Lower bonuses (1.05x - 1.2x)

### **Racial Balance**
- **Common Races**: Standard bonuses
- **Rare Races**: Higher bonuses but with drawbacks
- **Legendary Races**: Very high bonuses but significant limitations

## 🎯 **Integration Points**

### **Race Core Integration**
```rust
pub struct Race {
    pub race_id: String,
    pub race_name: String,
    pub base_tinh_stats: TinhStats,
    pub talent_system: TalentSystem,  // Talent system integration
    pub racial_abilities: Vec<RacialAbility>,
}
```

### **Element Core Integration**
```rust
pub struct ElementalMastery {
    pub element_id: String,
    pub base_mastery: f64,
    pub talent_bonus: f64,           // Talent system bonus
    pub total_mastery: f64,          // Base + talent bonus
}
```

### **Actor Core Integration**
```rust
pub struct Actor {
    pub id: String,
    pub race: String,
    pub tinh_stats: TinhStats,
    pub elemental_masteries: HashMap<String, ElementalMastery>,
    pub talent_bonuses: HashMap<String, f64>, // Applied talent bonuses
}
```

## 🎯 **Next Steps**

1. **Review và feedback** trên design này
2. **Implement core structure** trong Race Core
3. **Create integration** với Element Core
4. **Add talent application** logic
5. **Test balance** và fine-tune values

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
