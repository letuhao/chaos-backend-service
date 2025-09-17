# Combat Skill Categories Design

## 📋 **Tổng Quan**

Document này mô tả các loại combat skills và cách chúng được phân loại trong hệ thống Chaos World MMORPG.

## 🎯 **Combat Skill Categories**

### **1. Attack Skills (Kỹ Năng Tấn Công)**

**Mô tả**: Skills chuyên về tấn công, gây sát thương trực tiếp cho đối phương.

**Ví dụ**:
- **Fireball** (Hỏa Cầu) - Fire element attack
- **Lightning Strike** (Sét Đánh) - Lightning element attack
- **Ice Shard** (Băng Tinh) - Ice element attack
- **Earth Spike** (Đất Nhọn) - Earth element attack
- **Wind Blade** (Gió Lưỡi) - Wind element attack

**Derived Stats**:
- `attack_skill_effectiveness` - Hiệu quả kỹ năng tấn công
- `skill_execution_speed` - Tốc độ thực thi kỹ năng
- `skill_cooldown_reduction` - Giảm thời gian cooldown

**Scaling Factors**:
- Element mastery
- Power point
- Crit rate
- Element penetration

### **2. Defense Skills (Kỹ Năng Phòng Thủ)**

**Mô tả**: Skills chuyên về phòng thủ, giảm sát thương nhận vào hoặc tăng khả năng phòng thủ.

**Ví dụ**:
- **Crystal Defense** (Kết Tinh Phòng Thủ) - Earth/Metal element defense
- **Water Shield** (Thủy Khiên) - Water element shield
- **Wind Barrier** (Gió Rào) - Wind element barrier
- **Fire Ward** (Hỏa Hộ) - Fire element ward
- **Ice Armor** (Băng Giáp) - Ice element armor

**Derived Stats**:
- `defense_skill_effectiveness` - Hiệu quả kỹ năng phòng thủ
- `skill_execution_speed` - Tốc độ thực thi kỹ năng
- `skill_cooldown_reduction` - Giảm thời gian cooldown

**Scaling Factors**:
- Element mastery
- Defense point
- Element absorption
- Block rate

### **3. Status Skills (Kỹ Năng Trạng Thái)**

**Mô tả**: Skills gây ra hoặc loại bỏ status effects (buff/debuff).

**Ví dụ**:
- **Burning Touch** (Chạm Cháy) - Fire element burning debuff
- **Freeze Ray** (Tia Đóng Băng) - Ice element freeze debuff
- **Healing Aura** (Hào Quang Hồi Phục) - Light element healing buff
- **Poison Cloud** (Mây Độc) - Dark element poison debuff
- **Speed Boost** (Tăng Tốc) - Wind element speed buff

**Derived Stats**:
- `status_skill_effectiveness` - Hiệu quả kỹ năng trạng thái
- `status_probability` - Xác suất gây status effect
- `status_duration` - Thời gian duy trì status effect
- `status_intensity` - Cường độ status effect

**Scaling Factors**:
- Element mastery
- Status probability
- Status intensity
- Element interaction bonuses

### **4. Movement Skills (Kỹ Năng Di Chuyển)**

**Mô tả**: Skills liên quan đến di chuyển, thân pháp, và positioning.

**Ví dụ**:
- **Fire Dash** (Thân Pháp Hỏa) - Fire element movement
- **Water Flow** (Thân Pháp Thủy) - Water element movement
- **Earth Stride** (Thân Pháp Thổ) - Earth element movement
- **Wind Step** (Thân Pháp Phong) - Wind element movement
- **Lightning Flash** (Thân Pháp Lôi) - Lightning element movement

**Derived Stats**:
- `movement_technique_effectiveness` - Hiệu quả kỹ năng di chuyển
- `skill_execution_speed` - Tốc độ thực thi kỹ năng
- `skill_cooldown_reduction` - Giảm thời gian cooldown

**Scaling Factors**:
- Element mastery
- Movement technique mastery
- Elemental movement bonuses
- Terrain adaptation

### **5. Healing Skills (Kỹ Năng Hồi Phục)**

**Mô tả**: Skills chuyên về hồi phục HP, MP, Stamina, và các tài nguyên khác.

**Ví dụ**:
- **Healing Light** (Ánh Sáng Hồi Phục) - Light element healing
- **Water Restoration** (Thủy Hồi Phục) - Water element healing
- **Life Force** (Sinh Lực) - Life element healing
- **Regeneration** (Tái Tạo) - Wood element healing
- **Vitality Boost** (Tăng Sinh Mệnh) - Life element buff

**Derived Stats**:
- `healing_skill_effectiveness` - Hiệu quả kỹ năng hồi phục
- `skill_execution_speed` - Tốc độ thực thi kỹ năng
- `skill_cooldown_reduction` - Giảm thời gian cooldown

**Scaling Factors**:
- Element mastery
- Healing effectiveness
- Resource regeneration
- Element interaction bonuses

### **6. Support Skills (Kỹ Năng Hỗ Trợ)**

**Mô tả**: Skills hỗ trợ đồng đội, tăng cường stats, hoặc cung cấp utility.

**Ví dụ**:
- **Elemental Blessing** (Phước Lành Nguyên Tố) - Multi-element support
- **Group Shield** (Khiên Nhóm) - Earth element group defense
- **Speed Aura** (Hào Quang Tốc Độ) - Wind element group speed
- **Mana Regeneration** (Hồi Phục Mana) - Light element group mana
- **Elemental Resonance** (Cộng Hưởng Nguyên Tố) - Multi-element group buff

**Derived Stats**:
- `support_skill_effectiveness` - Hiệu quả kỹ năng hỗ trợ
- `skill_execution_speed` - Tốc độ thực thi kỹ năng
- `skill_cooldown_reduction` - Giảm thời gian cooldown

**Scaling Factors**:
- Element mastery
- Support effectiveness
- Group size
- Element synergy bonuses

### **7. Utility Skills (Kỹ Năng Tiện Ích)**

**Mô tả**: Skills cung cấp utility, không trực tiếp combat nhưng hữu ích trong gameplay.

**Ví dụ**:
- **Elemental Detection** (Phát Hiện Nguyên Tố) - Multi-element detection
- **Resource Gathering** (Thu Thập Tài Nguyên) - Earth element gathering
- **Teleportation** (Dịch Chuyển) - Space element movement
- **Invisibility** (Tàng Hình) - Shadow element stealth
- **Elemental Communication** (Giao Tiếp Nguyên Tố) - Multi-element communication

**Derived Stats**:
- `utility_skill_effectiveness` - Hiệu quả kỹ năng tiện ích
- `skill_execution_speed` - Tốc độ thực thi kỹ năng
- `skill_cooldown_reduction` - Giảm thời gian cooldown

**Scaling Factors**:
- Element mastery
- Utility effectiveness
- Element sensitivity
- Resource efficiency

## 🔧 **Skill Effectiveness Calculation**

### **Base Formula**
```rust
fn calculate_skill_effectiveness(
    base_effectiveness: f64,
    skill_category: SkillCategory,
    element_mastery: f64,
    derived_stats: &ElementDerivedStats
) -> f64 {
    let category_effectiveness = match skill_category {
        SkillCategory::Attack => derived_stats.attack_skill_effectiveness,
        SkillCategory::Defense => derived_stats.defense_skill_effectiveness,
        SkillCategory::Status => derived_stats.status_skill_effectiveness,
        SkillCategory::Movement => derived_stats.movement_technique_effectiveness,
        SkillCategory::Healing => derived_stats.healing_skill_effectiveness,
        SkillCategory::Support => derived_stats.support_skill_effectiveness,
        SkillCategory::Utility => derived_stats.utility_skill_effectiveness,
    };
    
    let mastery_bonus = element_mastery * 0.001; // 0.1% per mastery point
    let execution_speed_bonus = derived_stats.skill_execution_speed - 1.0;
    
    base_effectiveness * category_effectiveness * (1.0 + mastery_bonus + execution_speed_bonus)
}
```

### **Category-Specific Modifiers**
```rust
fn apply_category_modifiers(
    effectiveness: f64,
    skill_category: SkillCategory,
    derived_stats: &ElementDerivedStats
) -> f64 {
    match skill_category {
        SkillCategory::Attack => {
            effectiveness * (1.0 + derived_stats.element_penetration * 0.01)
        },
        SkillCategory::Defense => {
            effectiveness * (1.0 + derived_stats.element_absorption * 0.01)
        },
        SkillCategory::Status => {
            effectiveness * (1.0 + derived_stats.status_probability * 0.01)
        },
        SkillCategory::Movement => {
            effectiveness * (1.0 + derived_stats.movement_technique_effectiveness - 1.0)
        },
        SkillCategory::Healing => {
            effectiveness * (1.0 + derived_stats.resource_regeneration * 0.01)
        },
        SkillCategory::Support => {
            effectiveness * (1.0 + derived_stats.element_leadership_bonus * 0.01)
        },
        SkillCategory::Utility => {
            effectiveness * (1.0 + derived_stats.element_sensitivity * 0.01)
        },
    }
}
```

## 📊 **Skill Category Distribution**

### **By Element Type**
```yaml
element_skill_distribution:
  fire:
    attack: 0.4      # 40% attack skills
    defense: 0.1     # 10% defense skills
    status: 0.3      # 30% status skills
    movement: 0.1    # 10% movement skills
    healing: 0.05    # 5% healing skills
    support: 0.03    # 3% support skills
    utility: 0.02    # 2% utility skills
  
  water:
    attack: 0.2      # 20% attack skills
    defense: 0.2     # 20% defense skills
    status: 0.2      # 20% status skills
    movement: 0.15   # 15% movement skills
    healing: 0.15    # 15% healing skills
    support: 0.08    # 8% support skills
    utility: 0.02    # 2% utility skills
  
  earth:
    attack: 0.25     # 25% attack skills
    defense: 0.35    # 35% defense skills
    status: 0.15     # 15% status skills
    movement: 0.1    # 10% movement skills
    healing: 0.05    # 5% healing skills
    support: 0.08    # 8% support skills
    utility: 0.02    # 2% utility skills
```

### **By Cultivation Level**
```yaml
cultivation_skill_unlocks:
  foundation_building:
    attack: 0.6      # 60% attack skills available
    defense: 0.4     # 40% defense skills available
    status: 0.3      # 30% status skills available
    movement: 0.2    # 20% movement skills available
    healing: 0.1     # 10% healing skills available
    support: 0.05    # 5% support skills available
    utility: 0.02    # 2% utility skills available
  
  golden_core:
    attack: 0.8      # 80% attack skills available
    defense: 0.7     # 70% defense skills available
    status: 0.6      # 60% status skills available
    movement: 0.5    # 50% movement skills available
    healing: 0.4     # 40% healing skills available
    support: 0.3     # 30% support skills available
    utility: 0.2     # 20% utility skills available
  
  nascent_soul:
    attack: 0.95     # 95% attack skills available
    defense: 0.9     # 90% defense skills available
    status: 0.85     # 85% status skills available
    movement: 0.8    # 80% movement skills available
    healing: 0.7     # 70% healing skills available
    support: 0.6     # 60% support skills available
    utility: 0.5     # 50% utility skills available
```

## 🎯 **Implementation Examples**

### **1. Fire Element Attack Skill**
```yaml
fireball_technique:
  skill_id: "fireball_technique"
  skill_name: "Fireball Technique"
  skill_name_vi: "Hỏa Cầu Thuật"
  skill_category: "attack"
  element_id: "fire"
  base_damage: 100.0
  base_effectiveness: 1.0
  scaling_stats:
    - "attack_skill_effectiveness"
    - "power_point"
    - "element_penetration"
  derived_stats_required:
    - "attack_skill_effectiveness"
    - "skill_execution_speed"
    - "skill_cooldown_reduction"
```

### **2. Earth Element Defense Skill**
```yaml
crystal_defense_technique:
  skill_id: "crystal_defense_technique"
  skill_name: "Crystal Defense Technique"
  skill_name_vi: "Kết Tinh Phòng Thủ"
  skill_category: "defense"
  element_id: "earth"
  base_defense_multiplier: 20.0
  base_defense_bonus: 100000.0
  scaling_stats:
    - "defense_skill_effectiveness"
    - "defense_point"
    - "element_absorption"
  derived_stats_required:
    - "defense_skill_effectiveness"
    - "skill_execution_speed"
    - "skill_cooldown_reduction"
```

### **3. Water Element Status Skill**
```yaml
freeze_ray:
  skill_id: "freeze_ray"
  skill_name: "Freeze Ray"
  skill_name_vi: "Tia Đóng Băng"
  skill_category: "status"
  element_id: "water"
  base_probability: 0.15
  base_duration: 5.0
  scaling_stats:
    - "status_skill_effectiveness"
    - "status_probability"
    - "status_duration"
  derived_stats_required:
    - "status_skill_effectiveness"
    - "status_probability"
    - "status_duration"
```

## 📈 **Performance Considerations**

### **1. Skill Category Caching**
```rust
pub struct SkillCategoryCache {
    attack_skills: HashMap<String, f64>,
    defense_skills: HashMap<String, f64>,
    status_skills: HashMap<String, f64>,
    movement_skills: HashMap<String, f64>,
    healing_skills: HashMap<String, f64>,
    support_skills: HashMap<String, f64>,
    utility_skills: HashMap<String, f64>,
}
```

### **2. Batch Processing**
```rust
impl SkillCategoryCache {
    pub fn batch_calculate_effectiveness(
        &self,
        skills: &[Skill],
        derived_stats: &ElementDerivedStats
    ) -> HashMap<String, f64> {
        let mut results = HashMap::new();
        
        for skill in skills {
            let effectiveness = self.calculate_skill_effectiveness(skill, derived_stats);
            results.insert(skill.skill_id.clone(), effectiveness);
        }
        
        results
    }
}
```

## ✅ **Summary**

### **Combat Skill Categories**
1. **Attack Skills** - Tấn công trực tiếp
2. **Defense Skills** - Phòng thủ và bảo vệ
3. **Status Skills** - Gây status effects
4. **Movement Skills** - Di chuyển và thân pháp
5. **Healing Skills** - Hồi phục tài nguyên
6. **Support Skills** - Hỗ trợ đồng đội
7. **Utility Skills** - Tiện ích và utility

### **Derived Stats Mapping**
- `attack_skill_effectiveness` → Attack Skills
- `defense_skill_effectiveness` → Defense Skills
- `status_skill_effectiveness` → Status Skills
- `movement_technique_effectiveness` → Movement Skills
- `healing_skill_effectiveness` → Healing Skills
- `support_skill_effectiveness` → Support Skills
- `utility_skill_effectiveness` → Utility Skills

### **Benefits**
- **Granular Control**: Kiểm soát chi tiết từng loại skill
- **Balanced Scaling**: Scaling khác nhau cho từng category
- **Element Specialization**: Elements có thể specialize vào categories khác nhau
- **Performance Optimization**: Cache và batch processing hiệu quả

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
