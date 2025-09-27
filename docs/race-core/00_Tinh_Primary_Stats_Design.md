# Tinh Primary Stats Design

## 📋 **Tổng Quan**

Tinh (精) là nền tảng của sự sống, đại diện cho căn cơ, tinh huyết, nguyên dương và di truyền. Trong hệ thống Chaos World, Tinh được thiết kế như primary stats cơ bản, tương tự như thọ nguyên/sinh lực/thể lực trong các hệ thống tu luyện truyền thống.

## 🎯 **Bản Chất của Tinh**

### **Đặc Điểm Chính:**
- **Cố định từ khi sinh**: Quyết định bởi huyết mạch, thiên phú
- **Ít thay đổi**: Chỉ tăng khi đột phá cảnh giới hoặc kỳ ngộ
- **Nền tảng**: Quyết định giới hạn của Khí và Thần
- **Meta-stat**: Định hình build và tiềm năng phát triển

### **Vai Trò trong Game:**
- **Growth Cap**: Giới hạn tăng trưởng của Khí và Thần
- **Survival Stats**: Ảnh hưởng HP, lifespan, regen
- **Foundation**: Nền tảng cho tất cả stats khác

## 🏗️ **Primary Stats của Tinh**

### **1. Lifespan (Thọ Nguyên / 寿元)**
- **Định nghĩa**: Tuổi thọ cơ bản của nhân vật
- **Tính chất**: Cố định, chỉ tăng khi đột phá
- **Ảnh hưởng**: 
  - Tuổi thọ tối đa
  - Tốc độ lão hóa
  - Khả năng sống sót

### **2. Vitality (Sinh Lực / 生命力)**
- **Định nghĩa**: Sức sống cơ bản, khả năng phục hồi
- **Tính chất**: Cố định, ảnh hưởng đến regen
- **Ảnh hưởng**:
  - HP regeneration rate
  - Wound healing speed
  - Disease resistance

### **3. Physical Foundation (Thể Lực / 体力)**
- **Định nghĩa**: Nền tảng thể chất, sức mạnh cơ bản
- **Tính chất**: Cố định, quyết định physical stats
- **Ảnh hưởng**:
  - Base HP
  - Physical damage resistance
  - Carrying capacity

### **4. Talent Foundation (Căn Cơ / 根基)**
- **Định nghĩa**: Thiên phú tu luyện, khả năng học tập
- **Tính chất**: Cố định, quyết định learning speed
- **Ảnh hưởng**:
  - Skill learning speed
  - Cultivation efficiency
  - Breakthrough success rate

### **5. Bloodline (Huyết Mạch / 血脉)**
- **Định nghĩa**: Đặc tính di truyền, chủng tộc
- **Tính chất**: Cố định, quyết định racial bonuses
- **Ảnh hưởng**:
  - Racial abilities
  - Elemental affinities
  - Special traits

## 📊 **Stats Structure**

### **Base Structure**
```rust
/// Tinh (精) Primary Stats - Foundation of life essence
/// 精元基础属性 - 生命精华的基础
pub struct TinhStats {
    // Primary Stats - 基础属性
    pub lifespan: f64,        // Lifespan (Thọ Nguyên / 寿元)
    pub vitality: f64,        // Vitality (Sinh Lực / 生命力)
    pub physical_foundation: f64, // Physical Foundation (Thể Lực / 体力)
    pub talent_foundation: f64,   // Talent Foundation (Căn Cơ / 根基)
    pub bloodline: f64,       // Bloodline (Huyết Mạch / 血脉)
    
    // Derived Stats - 衍生属性
    pub max_lifespan: f64,    // Calculated from lifespan
    pub base_hp: f64,         // Calculated from physical_foundation
    pub regen_rate: f64,      // Calculated from vitality
    pub learning_speed: f64,  // Calculated from talent_foundation
    pub racial_bonus: f64,    // Calculated from bloodline
}
```

### **Calculation Formulas**
```rust
impl TinhStats {
    /// Calculate derived stats from primary Tinh stats
    /// 从精元基础属性计算衍生属性
    pub fn calculate_derived_stats(&self) -> DerivedStats {
        DerivedStats {
            max_lifespan: self.lifespan * 100.0,           // 寿元 * 100
            base_hp: self.physical_foundation * 50.0,      // 体力 * 50
            regen_rate: self.vitality * 0.1,               // 生命力 * 0.1
            learning_speed: self.talent_foundation * 0.05, // 根基 * 0.05
            racial_bonus: self.bloodline * 0.2,            // 血脉 * 0.2
        }
    }
}
```

## 🔗 **Integration với Race Core**

### **Race Base Values**
```rust
/// Race base values for Tinh stats
/// 种族精元基础值
pub struct RaceTinhBase {
    pub race_id: String,              // Race identifier
    pub lifespan_base: f64,           // Base lifespan (寿元基础值)
    pub vitality_base: f64,           // Base vitality (生命力基础值)
    pub physical_foundation_base: f64, // Base physical foundation (体力基础值)
    pub talent_foundation_base: f64,   // Base talent foundation (根基基础值)
    pub bloodline_base: f64,          // Base bloodline (血脉基础值)
}
```

### **Racial Examples**
```rust
// Human - 人类
RaceTinhBase {
    race_id: "human".to_string(),
    lifespan_base: 100.0,              // Standard lifespan (标准寿元)
    vitality_base: 100.0,              // Standard vitality (标准生命力)
    physical_foundation_base: 100.0,   // Standard physical (标准体力)
    talent_foundation_base: 100.0,     // Standard talent (标准根基)
    bloodline_base: 100.0,             // Standard bloodline (标准血脉)
}

// Dragon - 龙族
RaceTinhBase {
    race_id: "dragon".to_string(),
    lifespan_base: 1000.0,             // Long lifespan (长寿元)
    vitality_base: 200.0,              // High vitality (高生命力)
    physical_foundation_base: 300.0,   // Strong physical (强体力)
    talent_foundation_base: 150.0,     // Good talent (好根基)
    bloodline_base: 500.0,             // Strong bloodline (强血脉)
}

// Demon - 魔族
RaceTinhBase {
    race_id: "demon".to_string(),
    lifespan_base: 200.0,              // Medium lifespan (中等寿元)
    vitality_base: 150.0,              // High vitality (高生命力)
    physical_foundation_base: 200.0,   // Strong physical (强体力)
    talent_foundation_base: 120.0,     // Good talent (好根基)
    bloodline_base: 300.0,             // Strong bloodline (强血脉)
}
```

## 🚀 **Breakthrough Bonuses**

### **Breakthrough Structure**
```rust
/// Breakthrough bonuses for Tinh stats
/// 突破境界的精元加成
pub struct BreakthroughBonus {
    pub realm: String,                 // Realm name (境界名称)
    pub lifespan_bonus: f64,           // Lifespan bonus (寿元加成)
    pub vitality_bonus: f64,           // Vitality bonus (生命力加成)
    pub physical_foundation_bonus: f64, // Physical foundation bonus (体力加成)
    pub talent_foundation_bonus: f64,   // Talent foundation bonus (根基加成)
    pub bloodline_bonus: f64,          // Bloodline bonus (血脉加成)
}
```

### **Realm Examples**
```rust
// Trúc Cơ (Foundation Building) - 筑基
BreakthroughBonus {
    realm: "zhu_ji".to_string(),
    lifespan_bonus: 50.0,              // +50 years (增加50年)
    vitality_bonus: 20.0,              // +20% vitality (增加20%生命力)
    physical_foundation_bonus: 30.0,   // +30% physical (增加30%体力)
    talent_foundation_bonus: 10.0,     // +10% talent (增加10%根基)
    bloodline_bonus: 0.0,              // No bloodline change (血脉不变)
}

// Kim Đan (Golden Core) - 金丹
BreakthroughBonus {
    realm: "jin_dan".to_string(),
    lifespan_bonus: 200.0,             // +200 years (增加200年)
    vitality_bonus: 50.0,              // +50% vitality (增加50%生命力)
    physical_foundation_bonus: 80.0,   // +80% physical (增加80%体力)
    talent_foundation_bonus: 30.0,     // +30% talent (增加30%根基)
    bloodline_bonus: 10.0,             // +10% bloodline (增加10%血脉)
}

// Nguyên Anh (Nascent Soul) - 元婴
BreakthroughBonus {
    realm: "yuan_ying".to_string(),
    lifespan_bonus: 1000.0,            // +1000 years (增加1000年)
    vitality_bonus: 100.0,             // +100% vitality (增加100%生命力)
    physical_foundation_bonus: 150.0,  // +150% physical (增加150%体力)
    talent_foundation_bonus: 60.0,     // +60% talent (增加60%根基)
    bloodline_bonus: 30.0,             // +30% bloodline (增加30%血脉)
}
```

## 🎯 **Special Events**

### **Event Structure**
```rust
/// Special events that affect Tinh stats
/// 影响精元属性的特殊事件
pub struct SpecialEvent {
    pub event_id: String,               // Event identifier
    pub event_type: EventType,          // Event type
    pub lifespan_bonus: f64,            // Lifespan bonus (寿元加成)
    pub vitality_bonus: f64,            // Vitality bonus (生命力加成)
    pub physical_foundation_bonus: f64, // Physical foundation bonus (体力加成)
    pub talent_foundation_bonus: f64,   // Talent foundation bonus (根基加成)
    pub bloodline_bonus: f64,           // Bloodline bonus (血脉加成)
    pub duration: Option<Duration>,     // None = permanent (None = 永久)
}

/// Types of special events
/// 特殊事件类型
pub enum EventType {
    HeavenlyTreasure,    // Thiên tài địa bảo (天材地宝)
    BloodlineAwakening,  // Đột phá huyết mạch (突破血脉)
    BodyTempering,       // Luyện thể (炼体)
    CultivationMethod,   // Công pháp tu luyện (功法修炼)
    DivineIntervention,  // Thiên can thiên địa (天干天地)
}
```

### **Event Examples**
```rust
// Thiên tài địa bảo - Tăng căn cơ (天材地宝 - 增加根基)
SpecialEvent {
    event_id: "heavenly_treasure_001".to_string(),
    event_type: EventType::HeavenlyTreasure,
    lifespan_bonus: 0.0,              // No lifespan change (寿元不变)
    vitality_bonus: 0.0,              // No vitality change (生命力不变)
    physical_foundation_bonus: 0.0,   // No physical change (体力不变)
    talent_foundation_bonus: 100.0,   // +100% talent (增加100%根基)
    bloodline_bonus: 0.0,             // No bloodline change (血脉不变)
    duration: None,                   // Permanent (永久)
}

// Đột phá huyết mạch - Tăng huyết mạch (突破血脉 - 增加血脉)
SpecialEvent {
    event_id: "bloodline_awakening_001".to_string(),
    event_type: EventType::BloodlineAwakening,
    lifespan_bonus: 0.0,              // No lifespan change (寿元不变)
    vitality_bonus: 0.0,              // No vitality change (生命力不变)
    physical_foundation_bonus: 0.0,   // No physical change (体力不变)
    talent_foundation_bonus: 0.0,     // No talent change (根基不变)
    bloodline_bonus: 200.0,           // +200% bloodline (增加200%血脉)
    duration: None,                   // Permanent (永久)
}

// Luyện thể - Tăng thể lực (炼体 - 增加体力)
SpecialEvent {
    event_id: "body_tempering_001".to_string(),
    event_type: EventType::BodyTempering,
    lifespan_bonus: 0.0,              // No lifespan change (寿元不变)
    vitality_bonus: 50.0,             // +50% vitality (增加50%生命力)
    physical_foundation_bonus: 100.0, // +100% physical (增加100%体力)
    talent_foundation_bonus: 0.0,     // No talent change (根基不变)
    bloodline_bonus: 0.0,             // No bloodline change (血脉不变)
    duration: Some(Duration::from_secs(3600)), // 1 hour (1小时)
}
```

## 🔧 **Implementation Strategy**

### **Phase 1: Core Structure**
1. **Define TinhStats struct**
2. **Implement base calculations**
3. **Create race base values**
4. **Add breakthrough bonuses**

### **Phase 2: Integration**
1. **Integrate with Actor Core**
2. **Add event system**
3. **Implement derived stats**
4. **Add validation logic**

### **Phase 3: Advanced Features**
1. **Add special events**
2. **Implement temporary effects**
3. **Add racial interactions**
4. **Create balance testing**

## 📈 **Balance Considerations**

### **Base Values Range**
- **Lifespan (Thọ Nguyên / 寿元)**: 50-2000 (years)
- **Vitality (Sinh Lực / 生命力)**: 50-500 (vitality points)
- **Physical Foundation (Thể Lực / 体力)**: 50-500 (physical points)
- **Talent Foundation (Căn Cơ / 根基)**: 50-500 (talent points)
- **Bloodline (Huyết Mạch / 血脉)**: 50-1000 (bloodline points)

### **Breakthrough Scaling**
- **Linear scaling** for early realms
- **Exponential scaling** for high realms
- **Diminishing returns** for repeated breakthroughs

### **Event Balance**
- **Permanent events**: Small bonuses
- **Temporary events**: Large bonuses
- **Rare events**: Medium permanent bonuses
- **Common events**: Small temporary bonuses

## 🎯 **Next Steps**

1. **Review và feedback** trên design này
2. **Implement core structure** trong Race Core
3. **Create integration** với Actor Core
4. **Add event system** support
5. **Test balance** và fine-tune values

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
