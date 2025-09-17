# Derived Stats Consistency Check

## 📋 **Tổng Quan**

Document này kiểm tra tính nhất quán giữa derived stats trong các element config files và ElementDerivedStats struct từ Action Core.

## ✅ **Derived Stats Mapping**

### **1. Core Element Mastery**
- **ElementDerivedStats**: `element_mastery: HashMap<String, f64>`
- **Config Files**: `"element_mastery"`
- **Status**: ✅ **Consistent**

### **2. Counterbalance Pairs**
- **ElementDerivedStats**: 
  - `power_point: f64`
  - `defense_point: f64`
  - `crit_rate: f64`
  - `resist_crit_rate: f64`
  - `crit_damage: f64`
  - `resist_crit_damage: f64`
  - `accurate_rate: f64`
  - `dodge_rate: f64`
  - `status_probability: f64`
  - `status_resistance: f64`
  - `status_duration: f64`
  - `status_duration_reduction: f64`
  - `status_intensity: f64`
  - `status_intensity_reduction: f64`
  - `element_penetration: f64`
  - `element_absorption: f64`
  - `element_amplification: f64`
  - `element_reduction: f64`
  - `reflection_rate: f64`
  - `resist_reflection_rate: f64`
  - `reflection_damage: f64`
  - `resist_reflection_damage: f64`

- **Config Files**: Tất cả các stats trên đều có trong configs
- **Status**: ✅ **Consistent**

### **3. Parry System**
- **ElementDerivedStats**:
  - `parry_rate: f64`
  - `parry_break: f64`
  - `parry_strength: f64`
  - `parry_shred: f64`

- **Config Files**: Tất cả các stats trên đều có trong configs
- **Status**: ✅ **Consistent**

### **4. Block System**
- **ElementDerivedStats**:
  - `block_rate: f64`
  - `block_break: f64`
  - `block_strength: f64`
  - `block_shred: f64`

- **Config Files**: Tất cả các stats trên đều có trong configs
- **Status**: ✅ **Consistent**

### **5. Skill Execution & Performance**
- **ElementDerivedStats**:
  - `skill_execution_speed: f64`
  - `skill_cooldown_reduction: f64`
  - `attack_skill_effectiveness: f64`
  - `defense_skill_effectiveness: f64`
  - `status_skill_effectiveness: f64`
  - `movement_technique_effectiveness: f64`
  - `healing_skill_effectiveness: f64`
  - `support_skill_effectiveness: f64`
  - `utility_skill_effectiveness: f64`
  - `skill_effectiveness: f64`

- **Config Files**: Tất cả các stats trên đều có trong configs
- **Status**: ✅ **Consistent** (Updated: Split combat_skill_effectiveness into attack/defense)

### **6. Resource Management**
- **ElementDerivedStats**:
  - `resource_regeneration: f64`
  - `resource_efficiency: f64`

- **Config Files**: Tất cả các stats trên đều có trong configs
- **Status**: ✅ **Consistent**

### **7. Social & Economy (Future features)**
- **ElementDerivedStats**:
  - `element_leadership_bonus: f64`
  - `element_teaching_efficiency: f64`
  - `element_crafting_efficiency: f64`
  - `element_resource_discovery: f64`

- **Config Files**: Tất cả các stats trên đều có trong configs
- **Status**: ✅ **Consistent**

### **8. Perception & Detection**
- **ElementDerivedStats**:
  - `element_sensitivity: f64`

- **Config Files**: `"element_sensitivity"`
- **Status**: ✅ **Consistent**

### **9. Advanced Combat Mechanics**
- **ElementDerivedStats**:
  - `mastery_synergy_bonus: f64`

- **Config Files**: `"mastery_synergy_bonus"`
- **Status**: ✅ **Consistent**

### **10. Element Interaction Bonuses**
- **ElementDerivedStats**:
  - `element_interaction_bonuses: HashMap<String, f64>`

- **Config Files**: `"element_interaction_bonuses"`
- **Status**: ✅ **Consistent**

### **11. Feature Flags**
- **ElementDerivedStats**:
  - `feature_flags: HashMap<String, bool>`

- **Config Files**: `"feature_flags"`
- **Status**: ✅ **Consistent**

## 📊 **Summary**

### **Total Derived Stats Count**
- **ElementDerivedStats**: 38+ individual stats + 3 HashMap fields
- **Config Files**: 38+ individual stats + 3 HashMap fields
- **Status**: ✅ **Fully Consistent** (Updated: Added 3 new skill effectiveness stats)

### **Coverage by Category**
1. **Core Element Mastery**: ✅ 100% coverage
2. **Counterbalance Pairs**: ✅ 100% coverage (22 stats)
3. **Parry System**: ✅ 100% coverage (4 stats)
4. **Block System**: ✅ 100% coverage (4 stats)
5. **Skill Execution & Performance**: ✅ 100% coverage (10 stats)
6. **Resource Management**: ✅ 100% coverage (2 stats)
7. **Social & Economy**: ✅ 100% coverage (4 stats)
8. **Perception & Detection**: ✅ 100% coverage (1 stat)
9. **Advanced Combat Mechanics**: ✅ 100% coverage (1 stat)
10. **Element Interaction Bonuses**: ✅ 100% coverage (1 HashMap)
11. **Feature Flags**: ✅ 100% coverage (1 HashMap)

## 🔧 **Updated Config Files**

Tất cả các element config files đã được cập nhật:

1. ✅ **fire_element.yaml** - Updated with all derived stats
2. ✅ **water_element.yaml** - Updated with all derived stats
3. ✅ **earth_element.yaml** - Updated with all derived stats
4. ✅ **wood_element.yaml** - Updated with all derived stats
5. ✅ **metal_element.yaml** - Updated with all derived stats
6. ✅ **wind_element.yaml** - Updated with all derived stats
7. ✅ **lightning_element.yaml** - Updated with all derived stats
8. ✅ **ice_element.yaml** - Updated with all derived stats

## 🎯 **Key Improvements**

### **1. Added Missing Stats**
- **element_mastery**: Core element mastery system
- **element_interaction_bonuses**: Element interaction system
- **feature_flags**: Feature flag system

### **2. Organized by Categories**
- **Core Element Mastery**: Element mastery system
- **Counterbalance Pairs**: Basic combat stats
- **Parry System**: Parry mechanics
- **Block System**: Block mechanics
- **Skill Execution & Performance**: Skill-related stats
- **Resource Management**: Resource-related stats
- **Social & Economy**: Future features
- **Perception & Detection**: Detection stats
- **Advanced Combat Mechanics**: Advanced combat stats

### **3. Consistent Naming**
- Tất cả derived stats đều có tên nhất quán giữa ElementDerivedStats và config files
- Comments rõ ràng cho từng category
- References đến source files (e.g., "from fire_element.yaml")

## ✅ **Verification Complete**

**Status**: ✅ **All derived stats are now consistent between ElementDerivedStats and element config files**

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Complete  
**Maintainer**: Chaos World Team
