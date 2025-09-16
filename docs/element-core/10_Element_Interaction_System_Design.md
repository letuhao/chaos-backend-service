# Element Interaction System Design (Tương Sinh Tương Khắc)

## 📋 **Tổng Quan**

Element Interaction System (Hệ thống Tương Sinh Tương Khắc) là một hệ thống phức tạp cho phép defender nhận buff hoặc attacker nhận debuff dựa trên mối quan hệ giữa các element types và mastery levels. Hệ thống này tích hợp chặt chẽ với Elemental Mastery System để tạo ra meta game cân bằng và thú vị.

**🎯 Elemental Mastery Integration**: Xem [Elemental Mastery System Design](08_Elemental_Mastery_System_Design.md) và [Actor Core Integration Guide](09_Actor_Core_Integration_Guide.md) để hiểu cách Elemental Mastery System tích hợp vào element interaction system.

## ⚙️ **Khung Chuẩn (Counterbalance – No Hard Caps)**

- Base trigger theo quan hệ element được đọc từ config (same/generating/overcoming/neutral).
- Xác suất và cường độ hiệu ứng dùng Probability Engine (sigmoid/custom_sigmoid) từ `01_Probability_Mechanics_Design.md`; KHÔNG dùng công thức tuyến tính không clamp.
- Không sử dụng “cap” gameplay. Chỉ ràng buộc xác suất trong [0,1] (tính chất toán học). Cường độ/độ dài được điều tiết bằng đối trọng + suy giảm (dynamics) và refractory.

### Công Thức Chuẩn (không lặp công thức xác suất)

```rust
// Trigger probability (counterbalance)
let base_trigger = cfg.base_trigger_for(relationship); // 0.0/0.1/0.3/0.8
let norm_diff = normalize_mastery_difference(attacker_mastery, defender_mastery);
let trig = base_trigger + sigmoid(norm_diff / cfg.trigger_scale);
let final_trigger = trig.clamp(0.0, 1.0); // ràng buộc xác suất

// Intensity dynamics (no hard caps): dI/dt = α·Δ − β·I
let delta = compute_delta(&attacker_stats, &defender_stats, relationship);
let intensity_next = evolve_intensity(intensity_current, delta, cfg.dynamics.intensity_gain, cfg.dynamics.intensity_damping, dt);

// Refractory: p_next = σ((Δ − θ − R)/s), dR/dt = −ρ·R
let p_next = refractory_p(base_trigger, delta, refractory, cfg.theta, cfg.trigger_scale);
```

Ghi chú: Các bảng minh họa bên dưới chỉ mang tính ví dụ; engine thực thi theo công thức chuẩn + caps ở trên.

## 🧩 **Cấu Hình (interaction_config.yaml)**

```yaml
version: 1
relationships:
  same: 0.0
  generating: 0.3
  overcoming: 0.8
  neutral: 0.1

dynamics:
  trigger_scale: 50.0
  steepness: 1.0
  intensity_gain: 0.02
  intensity_damping: 0.01
  decay_rate: 0.05
  refractory_gain: 0.5
  refractory_decay: 0.1

pairs:
  # Five Elements (English IDs)
  fire:
    generating: ["wood"]
    overcoming: ["metal"]
    neutral: ["water", "earth"]
  water:
    generating: ["metal"]
    overcoming: ["fire"]
    neutral: ["wood", "earth"]
  # ... các element khác
```

## 🔗 **Nhất Quán Hệ Thống**

- Dùng English snake_case IDs cho engine (`fire, water, wood, metal, earth, light, dark, time, space, void, chaos`). Alias ngôn ngữ chỉ dùng cho hiển thị (xem `05_Element_Summary_Comprehensive.md`).
- Probability/steepness/scaling: tham chiếu duy nhất `01_Probability_Mechanics_Design.md`.
- Caps/cờ tính năng: tham chiếu `11_Advanced_Derived_Stats_Design.md`.
- Checklist cấu hình phần tử: xem `README.md` → “Element Config Validation Checklist”.
- Cấu hình trung tâm:
  - `docs/element-core/configs/interaction_config.yaml`
  - `docs/element-core/configs/status_pool.yaml`
  - `docs/element-core/configs/probability_config.yaml`

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Tương Sinh Tương Khắc Concept**
- **Defender Buff**: Khi attacker dùng element "sai", defender nhận buff
- **Attacker Debuff**: Khi attacker dùng element "sai", attacker nhận debuff
- **Mastery-Based**: Trigger probability dựa trên mastery difference
- **Element Interaction**: Dựa trên tương sinh tương khắc và element relationships

### **2. Strategic Depth**
- **Element Choice**: Tạo ra lý do để chọn element phù hợp
- **Mastery Investment**: Khuyến khích đầu tư vào mastery
- **Counter-Play**: Tạo ra counter-play opportunities
- **Meta Evolution**: Meta game thay đổi theo mastery distribution

## 📊 **Bảng Overview - Cùng Hệ (Same Element)**

### **Ví Dụ: Fire vs Fire**

| Attacker Mastery | Defender Mastery | Mastery Difference | Trigger Probability | Defender Buff | Attacker Debuff |
|------------------|------------------|-------------------|-------------------|---------------|-----------------|
| 1,000,000        | 10,000          | +990,000         | 0% (Same Element) | None          | None            |
| 10,000           | 1,000,000       | -990,000         | 0% (Same Element) | None          | None            |
| 500,000          | 500,000         | 0                | 0% (Same Element) | None          | None            |

**Kết luận**: Cùng hệ không có trigger, không có buff/debuff.

### **Ví Dụ: Water vs Water**

| Attacker Mastery | Defender Mastery | Mastery Difference | Trigger Probability | Defender Buff | Attacker Debuff |
|------------------|------------------|-------------------|-------------------|---------------|-----------------|
| 2,000,000        | 50,000           | +1,950,000       | 0% (Same Element) | None          | None            |
| 50,000           | 2,000,000        | -1,950,000       | 0% (Same Element) | None          | None            |
| 1,000,000        | 1,000,000        | 0                | 0% (Same Element) | None          | None            |

**Kết luận**: Cùng hệ không có trigger, không có buff/debuff.

## 📊 **Bảng Overview - Tương Khắc (Overcoming)**

### **Ví Dụ: Fire vs Metal (Fire khắc Metal)**

| Attacker Mastery | Defender Mastery | Mastery Difference | Trigger Probability | Defender Buff | Attacker Debuff |
|------------------|------------------|-------------------|-------------------|---------------|-----------------|
| 1,000,000        | 10,000           | +990,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |
| 10,000           | 1,000,000        | -990,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |
| 500,000          | 500,000          | 0                | 80% (Base)        | small (cfg)   | small (cfg)     |
| 100,000          | 200,000          | -100,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |
| 200,000          | 100,000          | +100,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |

**Công thức** (chuẩn hóa): dùng công thức ở phần Framework (sigmoid + caps), không dùng tuyến tính không clamp.

### **Ví Dụ: Water vs Fire (Water khắc Fire)**

| Attacker Mastery | Defender Mastery | Mastery Difference | Trigger Probability | Defender Buff | Attacker Debuff |
|------------------|------------------|-------------------|-------------------|---------------|-----------------|
| 2,000,000        | 50,000           | +1,950,000       | 100% (Capped)     | +3900% Defense| -2925% Attack   |
| 50,000           | 2,000,000        | -1,950,000       | 100% (Capped)     | +3900% Defense| -2925% Attack   |
| 1,000,000        | 1,000,000        | 0                | 80% (Base)        | +10% Defense  | -5% Attack      |

## 📊 **Bảng Overview - Tương Sinh (Generating)**

### **Ví Dụ: Wood vs Fire (Wood sinh Fire)**

| Attacker Mastery | Defender Mastery | Mastery Difference | Trigger Probability | Defender Buff | Attacker Debuff |
|------------------|------------------|-------------------|-------------------|---------------|-----------------|
| 1,000,000        | 10,000           | +990,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |
| 10,000           | 1,000,000        | -990,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |
| 500,000          | 500,000          | 0                | 30% (Base)        | small (cfg)   | small (cfg)     |
| 100,000          | 200,000          | -100,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |
| 200,000          | 100,000          | +100,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |

**Công thức** (chuẩn hóa): tham chiếu Framework ở trên.

## 📊 **Bảng Overview - Trung Tính (Neutral)**

### **Ví Dụ: Fire vs Earth (Neutral)**

| Attacker Mastery | Defender Mastery | Mastery Difference | Trigger Probability | Defender Buff | Attacker Debuff |
|------------------|------------------|-------------------|-------------------|---------------|-----------------|
| 1,000,000        | 10,000           | +990,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |
| 10,000           | 1,000,000        | -990,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |
| 500,000          | 500,000          | 0                | 10% (Base)        | small (cfg)   | small (cfg)     |
| 100,000          | 200,000          | -100,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |
| 200,000          | 100,000          | +100,000         | ~clamped (<=100%) | capped by cfg | capped by cfg   |

**Công thức** (chuẩn hóa): tham chiếu Framework ở trên.

## 📊 **Bảng Overview - Tất Cả Hệ Ngũ Hành**

### **1. Fire (Hỏa) Interactions**

| Target Element | Relationship | Base Trigger | Defender Buff Type | Attacker Debuff Type |
|----------------|--------------|--------------|-------------------|---------------------|
| Fire           | Same         | 0%           | None              | None                |
| Water          | Overcoming   | 80%          | Defense Boost     | Attack Reduction    |
| Metal          | Overcoming   | 80%          | Defense Boost     | Attack Reduction    |
| Wood           | Generating   | 30%          | Attack Boost      | Defense Reduction   |
| Earth          | Neutral      | 10%          | Defense Boost     | Attack Reduction    |

### **2. Water (Thủy) Interactions**

| Target Element | Relationship | Base Trigger | Defender Buff Type | Attacker Debuff Type |
|----------------|--------------|--------------|-------------------|---------------------|
| Water          | Same         | 0%           | None              | None                |
| Fire           | Overcoming   | 80%          | Defense Boost     | Attack Reduction    |
| Earth          | Overcoming   | 80%          | Defense Boost     | Attack Reduction    |
| Metal          | Generating   | 30%          | Attack Boost      | Defense Reduction   |
| Wood           | Neutral      | 10%          | Defense Boost     | Attack Reduction    |

### **3. Wood (Mộc) Interactions**

| Target Element | Relationship | Base Trigger | Defender Buff Type | Attacker Debuff Type |
|----------------|--------------|--------------|-------------------|---------------------|
| Wood           | Same         | 0%           | None              | None                |
| Metal          | Overcoming   | 80%          | Defense Boost     | Attack Reduction    |
| Earth          | Overcoming   | 80%          | Defense Boost     | Attack Reduction    |
| Fire           | Generating   | 30%          | Attack Boost      | Defense Reduction   |
| Water          | Neutral      | 10%          | Defense Boost     | Attack Reduction    |

### **4. Metal (Kim) Interactions**

| Target Element | Relationship | Base Trigger | Defender Buff Type | Attacker Debuff Type |
|----------------|--------------|--------------|-------------------|---------------------|
| Metal          | Same         | 0%           | None              | None                |
| Wood           | Overcoming   | 80%          | Defense Boost     | Attack Reduction    |
| Fire           | Overcoming   | 80%          | Defense Boost     | Attack Reduction    |
| Water          | Generating   | 30%          | Attack Boost      | Defense Reduction   |
| Earth          | Neutral      | 10%          | Defense Boost     | Attack Reduction    |

### **5. Earth (Thổ) Interactions**

| Target Element | Relationship | Base Trigger | Defender Buff Type | Attacker Debuff Type |
|----------------|--------------|--------------|-------------------|---------------------|
| Earth          | Same         | 0%           | None              | None                |
| Water          | Overcoming   | 80%          | Defense Boost     | Attack Reduction    |
| Wood           | Overcoming   | 80%          | Defense Boost     | Attack Reduction    |
| Fire           | Generating   | 30%          | Attack Boost      | Defense Reduction   |
| Metal          | Neutral      | 10%          | Defense Boost     | Attack Reduction    |

## 📊 **Bảng Overview - Advanced Elements**

### **1. Light vs Dark**

| Attacker Mastery | Defender Mastery | Mastery Difference | Trigger Probability | Defender Buff | Attacker Debuff |
|------------------|------------------|-------------------|-------------------|---------------|-----------------|
| 1,000,000        | 10,000           | +990,000         | 100% (Capped)     | +1980% Defense| -1485% Attack   |
| 10,000           | 1,000,000        | -990,000         | 100% (Capped)     | +1980% Defense| -1485% Attack   |
| 500,000          | 500,000          | 0                | 90% (Base)        | +10% Defense  | -5% Attack      |

**Công thức**:
- Trigger Probability: `min(0.9 + (mastery_difference.abs() * 0.0001), 1.0)`
- Defender Buff: `max(0.1, mastery_difference.abs() * 0.002)` (Defense Boost)
- Attacker Debuff: `max(0.05, mastery_difference * 0.0015)` (Attack Reduction)

### **2. Life vs Death**

| Attacker Mastery | Defender Mastery | Mastery Difference | Trigger Probability | Defender Buff | Attacker Debuff |
|------------------|------------------|-------------------|-------------------|---------------|-----------------|
| 1,000,000        | 10,000           | +990,000         | 100% (Capped)     | +1980% Defense| -1485% Attack   |
| 10,000           | 1,000,000        | -990,000         | 100% (Capped)     | +1980% Defense| -1485% Attack   |
| 500,000          | 500,000          | 0                | 90% (Base)        | +10% Defense  | -5% Attack      |

**Công thức**:
- Trigger Probability: `min(0.9 + (mastery_difference.abs() * 0.0001), 1.0)`
- Defender Buff: `max(0.1, mastery_difference.abs() * 0.002)` (Defense Boost)
- Attacker Debuff: `max(0.05, mastery_difference * 0.0015)` (Attack Reduction)

### **3. Time vs Space**

| Attacker Mastery | Defender Mastery | Mastery Difference | Trigger Probability | Defender Buff | Attacker Debuff |
|------------------|------------------|-------------------|-------------------|---------------|-----------------|
| 1,000,000        | 10,000           | +990,000         | 100% (Capped)     | +1980% Defense| -1485% Attack   |
| 10,000           | 1,000,000        | -990,000         | 100% (Capped)     | +1980% Defense| -1485% Attack   |
| 500,000          | 500,000          | 0                | 90% (Base)        | +10% Defense  | -5% Attack      |

**Công thức**:
- Trigger Probability: `min(0.9 + (mastery_difference.abs() * 0.0001), 1.0)`
- Defender Buff: `max(0.1, mastery_difference.abs() * 0.002)` (Defense Boost)
- Attacker Debuff: `max(0.05, mastery_difference * 0.0015)` (Attack Reduction)

## 🏗️ **Kiến Trúc Debuff Trigger System**

### **Core Components**

```
Debuff Trigger System
├── Element Interaction Matrix
│   ├── Tương Sinh Tương Khắc (Ngũ Hành)
│   ├── Light <-> Dark
│   ├── Life <-> Death
│   ├── Time <-> Space
│   ├── Void <-> Chaos
│   └── Phong <-> Lôi
├── Mastery-Based Trigger Calculation
│   ├── Mastery Difference Analysis
│   ├── Trigger Probability Calculation
│   ├── Buff/Debuff Intensity Scaling
│   └── Duration Calculation
├── Buff/Debuff Effects
│   ├── Defender Buffs
│   ├── Attacker Debuffs
│   ├── Stacking Rules
│   └── Duration Management
├── Integration Points
│   ├── Element-Core Integration
│   ├── Elemental Mastery Integration
│   ├── Combat-Core Integration
│   └── Actor Core Integration
└── Event System
    ├── Trigger Events
    ├── Effect Application
    ├── Duration Tracking
    └── Cleanup Events
```

## ⚡ **Element Interaction Matrix**

### **1. Tương Sinh Tương Khắc (Ngũ Hành)**

```rust
// Element interaction relationships
pub struct ElementInteraction {
    pub element_type: String,
    pub generating_elements: Vec<String>,    // Tương sinh
    pub overcoming_elements: Vec<String>,    // Tương khắc
    pub neutral_elements: Vec<String>,       // Trung tính
}

// Ngũ Hành interactions
pub const FIRE_INTERACTIONS: ElementInteraction = ElementInteraction {
    element_type: "fire".to_string(),
    generating_elements: vec!["wood".to_string()],      // Mộc sinh Hỏa
    overcoming_elements: vec!["metal".to_string()],     // Hỏa khắc Kim
    neutral_elements: vec!["water".to_string(), "earth".to_string()],
};

pub const WATER_INTERACTIONS: ElementInteraction = ElementInteraction {
    element_type: "water".to_string(),
    generating_elements: vec!["metal".to_string()],     // Kim sinh Thủy
    overcoming_elements: vec!["fire".to_string()],      // Thủy khắc Hỏa
    neutral_elements: vec!["wood".to_string(), "earth".to_string()],
};

// ... other elements
```

### **2. Advanced Element Interactions**

```rust
// Light <-> Dark
pub const LIGHT_DARK_INTERACTION: ElementInteraction = ElementInteraction {
    element_type: "light".to_string(),
    generating_elements: vec![],
    overcoming_elements: vec!["dark".to_string()],
    neutral_elements: vec![],
};

// Life <-> Death
pub const LIFE_DEATH_INTERACTION: ElementInteraction = ElementInteraction {
    element_type: "life".to_string(),
    generating_elements: vec![],
    overcoming_elements: vec!["death".to_string()],
    neutral_elements: vec![],
};

// Time <-> Space
pub const TIME_SPACE_INTERACTION: ElementInteraction = ElementInteraction {
    element_type: "time".to_string(),
    generating_elements: vec![],
    overcoming_elements: vec!["space".to_string()],
    neutral_elements: vec![],
};
```

## 🎲 **Mastery-Based Trigger Calculation**

### **1. Trigger Probability Formula**

```rust
pub struct DebuffTriggerCalculator {
    element_core: Arc<ElementCore>,
    mastery_system: Arc<ElementalMasterySystem>,
}

impl DebuffTriggerCalculator {
    /// Calculate debuff trigger probability
    pub async fn calculate_trigger_probability(
        &self,
        attacker: &Actor,
        target: &Actor,
        attacker_element: &str,
        target_primary_element: &str,
    ) -> DebuffTriggerResult {
        // 1. Get mastery levels
        let attacker_mastery = self.mastery_system.get_element_mastery(attacker, attacker_element).await?;
        let target_mastery = self.mastery_system.get_element_mastery(target, target_primary_element).await?;
        
        // 2. Calculate mastery difference
        let mastery_difference = attacker_mastery.mastery_level - target_mastery.mastery_level;
        
        // 3. Get element interaction
        let interaction = self.element_core.get_element_interaction(attacker_element, target_primary_element);
        
        // 4. Calculate base probability
        let base_probability = self.calculate_base_trigger_probability(interaction, mastery_difference);
        
        // 5. Apply mastery scaling
        let mastery_scaling = self.calculate_mastery_scaling(mastery_difference);
        
        // 6. Final probability
        let final_probability = base_probability * mastery_scaling;
        
        DebuffTriggerResult {
            trigger_probability: final_probability.clamp(0.0, 1.0),
            mastery_difference,
            interaction_type: interaction.interaction_type,
            buff_intensity: self.calculate_buff_intensity(mastery_difference),
            debuff_intensity: self.calculate_debuff_intensity(mastery_difference),
        }
    }
    
    /// Calculate base trigger probability based on element interaction
    fn calculate_base_trigger_probability(&self, interaction: &ElementInteraction, mastery_difference: f64) -> f64 {
        match interaction.interaction_type {
            InteractionType::Overcoming => 0.8,  // High chance when using overcoming element
            InteractionType::Generating => 0.3,  // Low chance when using generating element
            InteractionType::Neutral => 0.1,     // Very low chance for neutral
            InteractionType::Same => 0.0,        // No trigger for same element
        }
    }
    
    /// Calculate mastery scaling factor
    fn calculate_mastery_scaling(&self, mastery_difference: f64) -> f64 {
        // Higher mastery difference = higher trigger chance
        // But cap at reasonable levels
        let scaling = 1.0 + (mastery_difference.abs() * 0.01);
        scaling.clamp(0.5, 3.0)
    }
}
```

### **2. Buff/Debuff Intensity Calculation**

```rust
impl DebuffTriggerCalculator {
    /// Calculate buff intensity for defender
    fn calculate_buff_intensity(&self, mastery_difference: f64) -> f64 {
        // Negative mastery difference = defender is stronger = higher buff
        if mastery_difference < 0.0 {
            let intensity = mastery_difference.abs() * 0.02; // 2% per mastery point
            intensity.clamp(0.1, 1.0) // Cap at 100% buff
        } else {
            0.1 // Minimum buff even when attacker is stronger
        }
    }
    
    /// Calculate debuff intensity for attacker
    fn calculate_debuff_intensity(&self, mastery_difference: f64) -> f64 {
        // Positive mastery difference = attacker is stronger = higher debuff
        if mastery_difference > 0.0 {
            let intensity = mastery_difference * 0.015; // 1.5% per mastery point
            intensity.clamp(0.05, 0.8) // Cap at 80% debuff
        } else {
            0.05 // Minimum debuff even when defender is stronger
        }
    }
}
```

## 🎭 **Buff/Debuff Effects**

### **1. Defender Buffs**

```rust
pub struct DefenderBuff {
    pub buff_type: BuffType,
    pub intensity: f64,
    pub duration: f64,
    pub stackable: bool,
    pub max_stacks: u32,
    pub source_element: String,
    pub trigger_reason: String,
}

pub enum BuffType {
    // Offensive buffs
    AttackPowerBoost(f64),
    CritRateBoost(f64),
    CritDamageBoost(f64),
    AccuracyBoost(f64),
    
    // Defensive buffs
    DefenseBoost(f64),
    ResistCritBoost(f64),
    ResistCritDamageBoost(f64),
    DodgeBoost(f64),
    
    // Status effect buffs
    StatusResistanceBoost(f64),
    StatusDurationReduction(f64),
    StatusIntensityReduction(f64),
    
    // Special buffs
    ElementMasteryBoost(f64),  // Temporary mastery boost
    ElementResistanceBoost(f64), // Resistance to specific element
}

// Example: Fire vs Water (Water overcomes Fire)
// Defender (Water) gets buff when attacked by Fire
let water_defender_buff = DefenderBuff {
    buff_type: BuffType::DefenseBoost(0.3), // 30% defense boost
    intensity: 0.3,
    duration: 10.0, // 10 seconds
    stackable: true,
    max_stacks: 3,
    source_element: "fire".to_string(),
    trigger_reason: "water_overcomes_fire".to_string(),
};
```

### **2. Attacker Debuffs**

```rust
pub struct AttackerDebuff {
    pub debuff_type: DebuffType,
    pub intensity: f64,
    pub duration: f64,
    pub stackable: bool,
    pub max_stacks: u32,
    pub target_element: String,
    pub trigger_reason: String,
}

pub enum DebuffType {
    // Offensive debuffs
    AttackPowerReduction(f64),
    CritRateReduction(f64),
    CritDamageReduction(f64),
    AccuracyReduction(f64),
    
    // Defensive debuffs
    DefenseReduction(f64),
    ResistCritReduction(f64),
    ResistCritDamageReduction(f64),
    DodgeReduction(f64),
    
    // Status effect debuffs
    StatusProbabilityReduction(f64),
    StatusDurationIncrease(f64),
    StatusIntensityIncrease(f64),
    
    // Special debuffs
    ElementMasteryReduction(f64),  // Temporary mastery reduction
    ElementVulnerabilityIncrease(f64), // More vulnerable to specific element
}

// Example: Fire vs Water (Water overcomes Fire)
// Attacker (Fire) gets debuff when attacking Water
let fire_attacker_debuff = AttackerDebuff {
    debuff_type: DebuffType::AttackPowerReduction(0.2), // 20% attack reduction
    intensity: 0.2,
    duration: 8.0, // 8 seconds
    stackable: true,
    max_stacks: 2,
    target_element: "water".to_string(),
    trigger_reason: "fire_weak_against_water".to_string(),
};
```

## 🔗 **Integration với Elemental Mastery System**

### **1. Mastery-Based Trigger Conditions**

```rust
pub struct MasteryTriggerConditions {
    pub min_mastery_difference: f64,  // Minimum difference to trigger
    pub max_mastery_difference: f64,  // Maximum difference for full effect
    pub mastery_scaling_factor: f64,  // How mastery affects intensity
    pub decay_rate: f64,              // How fast effects decay
}

impl DebuffTriggerCalculator {
    /// Check if conditions are met for debuff trigger
    pub async fn check_trigger_conditions(
        &self,
        attacker: &Actor,
        target: &Actor,
        attacker_element: &str,
        target_primary_element: &str,
    ) -> bool {
        // 1. Get mastery levels
        let attacker_mastery = self.mastery_system.get_element_mastery(attacker, attacker_element).await?;
        let target_mastery = self.mastery_system.get_element_mastery(target, target_primary_element).await?;
        
        // 2. Check mastery difference
        let mastery_difference = attacker_mastery.mastery_level - target_mastery.mastery_level;
        
        // 3. Check if difference is significant enough
        if mastery_difference.abs() < self.conditions.min_mastery_difference {
            return false;
        }
        
        // 4. Check element interaction
        let interaction = self.element_core.get_element_interaction(attacker_element, target_primary_element);
        if interaction.interaction_type == InteractionType::Same {
            return false; // No trigger for same element
        }
        
        // 5. Check if elements are in opposite relationship
        match interaction.interaction_type {
            InteractionType::Overcoming | InteractionType::Generating => true,
            _ => false,
        }
    }
}
```

### **2. Mastery Decay Integration**

```rust
impl DebuffTriggerCalculator {
    /// Apply mastery decay to buff/debuff effects
    pub async fn apply_mastery_decay(
        &self,
        effect: &mut BuffDebuffEffect,
        attacker: &Actor,
        target: &Actor,
        time_elapsed: f64,
    ) -> Result<(), Error> {
        // 1. Calculate decay based on mastery difference
        let attacker_mastery = self.mastery_system.get_element_mastery(attacker, &effect.source_element).await?;
        let target_mastery = self.mastery_system.get_element_mastery(target, &effect.target_element).await?;
        let mastery_difference = attacker_mastery.mastery_level - target_mastery.mastery_level;
        
        // 2. Calculate decay rate
        let decay_rate = self.conditions.decay_rate * (1.0 + mastery_difference.abs() * 0.01);
        
        // 3. Apply decay to intensity
        effect.intensity *= (1.0 - decay_rate * time_elapsed).max(0.1);
        
        // 4. Apply decay to duration
        effect.remaining_duration -= time_elapsed * decay_rate;
        
        Ok(())
    }
}
```

## ⚔️ **Combat Integration**

### **1. Combat-Core Integration**

```rust
impl CombatCore {
    /// Process debuff trigger during combat
    pub async fn process_debuff_trigger(
        &self,
        action: &Action,
        attacker: &Actor,
        target: &Actor,
    ) -> CombatResult {
        // 1. Calculate damage normally
        let damage_result = self.calculate_damage(action, attacker, target).await?;
        
        // 2. Check for debuff trigger
        if self.debuff_trigger_calculator.check_trigger_conditions(
            attacker, target, action.element_type, target.get_primary_element()
        ).await? {
            // 3. Calculate trigger probability
            let trigger_result = self.debuff_trigger_calculator.calculate_trigger_probability(
                attacker, target, action.element_type, target.get_primary_element()
            ).await?;
            
            // 4. Roll for trigger
            if self.rng.gen::<f64>() < trigger_result.trigger_probability {
                // 5. Apply buffs/debuffs
                self.apply_debuff_effects(attacker, target, &trigger_result).await?;
                
                // 6. Log trigger event
                self.log_debuff_trigger(attacker, target, &trigger_result).await?;
            }
        }
        
        Ok(damage_result)
    }
    
    /// Apply debuff effects to actors
    async fn apply_debuff_effects(
        &self,
        attacker: &Actor,
        target: &Actor,
        trigger_result: &DebuffTriggerResult,
    ) -> Result<(), Error> {
        // 1. Apply defender buff
        let defender_buff = self.create_defender_buff(trigger_result);
        self.buff_system.apply_buff(target, defender_buff).await?;
        
        // 2. Apply attacker debuff
        let attacker_debuff = self.create_attacker_debuff(trigger_result);
        self.debuff_system.apply_debuff(attacker, attacker_debuff).await?;
        
        // 3. Update mastery systems
        self.mastery_system.notify_element_usage(attacker, trigger_result.source_element).await?;
        self.mastery_system.notify_element_resistance(target, trigger_result.target_element).await?;
        
        Ok(())
    }
}
```

### **2. Event System Integration**

```rust
pub enum DebuffTriggerEvent {
    TriggerActivated {
        attacker_id: String,
        target_id: String,
        attacker_element: String,
        target_element: String,
        mastery_difference: f64,
        trigger_probability: f64,
        buff_intensity: f64,
        debuff_intensity: f64,
    },
    EffectApplied {
        actor_id: String,
        effect_type: String,
        intensity: f64,
        duration: f64,
    },
    EffectExpired {
        actor_id: String,
        effect_type: String,
    },
    MasteryDecayApplied {
        actor_id: String,
        element: String,
        decay_amount: f64,
    },
}

impl EventSystem {
    /// Handle debuff trigger events
    pub async fn handle_debuff_trigger_event(&self, event: DebuffTriggerEvent) -> Result<(), Error> {
        match event {
            DebuffTriggerEvent::TriggerActivated { .. } => {
                // Log trigger activation
                self.logger.info("Debuff trigger activated", &event);
            },
            DebuffTriggerEvent::EffectApplied { .. } => {
                // Notify UI and other systems
                self.notify_effect_applied(&event).await?;
            },
            DebuffTriggerEvent::EffectExpired { .. } => {
                // Clean up expired effects
                self.cleanup_expired_effect(&event).await?;
            },
            DebuffTriggerEvent::MasteryDecayApplied { .. } => {
                // Update mastery systems
                self.update_mastery_decay(&event).await?;
            },
        }
        Ok(())
    }
}
```

## 📊 **Configuration Examples**

### **1. Element Interaction Configuration**

```yaml
# element_interactions.yaml
version: 1
interactions:
  - element: "fire"
    generating: ["wood"]
    overcoming: ["metal"]
    neutral: ["water", "earth"]
    trigger_probability:
      generating: 0.3
      overcoming: 0.8
      neutral: 0.1
      same: 0.0
  
  - element: "water"
    generating: ["metal"]
    overcoming: ["fire"]
    neutral: ["wood", "earth"]
    trigger_probability:
      generating: 0.3
      overcoming: 0.8
      neutral: 0.1
      same: 0.0

  - element: "light"
    generating: []
    overcoming: ["dark"]
    neutral: []
    trigger_probability:
      generating: 0.0
      overcoming: 0.9
      neutral: 0.0
      same: 0.0
```

### **2. Mastery Trigger Configuration**

```yaml
# mastery_trigger_config.yaml
version: 1
trigger_conditions:
  min_mastery_difference: 10.0
  max_mastery_difference: 100.0
  mastery_scaling_factor: 0.01
  decay_rate: 0.1

buff_effects:
  defender:
    base_intensity: 0.1
    max_intensity: 1.0
    base_duration: 10.0
    max_duration: 30.0
    stackable: true
    max_stacks: 3

debuff_effects:
  attacker:
    base_intensity: 0.05
    max_intensity: 0.8
    base_duration: 8.0
    max_duration: 20.0
    stackable: true
    max_stacks: 2
```

## 🎯 **Implementation Priority**

### **Phase 1: Core Debuff System**
1. **Element Interaction Matrix**: Implement tương sinh tương khắc
2. **Basic Trigger Calculation**: Simple probability calculation
3. **Buff/Debuff Effects**: Basic offensive/defensive effects
4. **Event System**: Basic event handling

### **Phase 2: Mastery Integration**
1. **Mastery-Based Triggers**: Integrate with Elemental Mastery System
2. **Mastery Scaling**: Intensity based on mastery difference
3. **Mastery Decay**: Time-based effect decay
4. **Advanced Effects**: Mastery-specific buffs/debuffs

### **Phase 3: Advanced Features**
1. **Stacking Rules**: Complex stacking and refresh mechanics
2. **Element Fusion**: Multi-element interactions
3. **Cultivation Integration**: Integration with cultivation systems
4. **Performance Optimization**: Caching and optimization

## ❓ **Questions for Discussion**

1. **Trigger Balance**: Làm thế nào để balance trigger probability?
2. **Effect Intensity**: Có nên có cap cho buff/debuff intensity?
3. **Stacking Rules**: Có nên cho phép stack nhiều effects cùng loại?
4. **Mastery Decay**: Có nên có mastery decay trong combat?
5. **Element Fusion**: Có nên có interactions cho multi-element attacks?

## 🎯 **Next Steps**

1. **Implement Element Interaction Matrix**: Define all element relationships
2. **Create Mastery Integration**: Integrate with Elemental Mastery System
3. **Design Buff/Debuff Effects**: Define all possible effects
4. **Build Event System**: Create event handling system
5. **Performance Testing**: Test với nhiều actors và effects

---

*Tài liệu này sẽ được cập nhật khi có thêm yêu cầu và feedback từ team.*
