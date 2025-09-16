# Advanced Derived Stats Design

## 📋 **Tổng Quan**

Tài liệu này mô tả các derived stats nâng cao cho Element Core, bao gồm skill execution, mastery bonuses, advanced combat mechanics, và resource management. Các stats này tạo ra depth và strategic choices cho players.

## ⚙️ **Phạm Vi & Cờ Tính Năng (Scope & Feature Flags)**

- MVP (Enabled by default):
  - SkillExecutionSpeed, SkillCooldownReduction
  - ParryRate, BlockRate
  - ElementPenetration, ElementAbsorption, ElementReflection
  - ResourceRegeneration, ResourceEfficiency
- Future (Disabled by default; bật qua config khi cần):
  - ElementMovementSpeed, ElementTeleportation
  - ElementSelfHealing, ElementGroupHealing
  - ElementLeadershipBonus, ElementTeachingEfficiency, ElementCraftingEfficiency, ElementResourceDiscovery
  - ElementSensitivity, MasterySynergyBonus

Lưu ý triển khai: Các tính năng Future nên được bảo vệ bởi feature flags để tránh chi phí runtime không cần thiết và giữ ổn định cân bằng game trong giai đoạn đầu.

## 🧩 **Dynamics (No Hard Caps)**

- Không dùng cap gameplay. Các đại lượng tiến hóa bằng đối trọng và suy giảm:
  - Form chung: dX/dt = α·Δ − β·X, với Δ là chênh lệch phù hợp (vd mastery/power/defense) và β>0 dập tắt tự nhiên.
  - Refractory dynamics cho trigger/tần suất: R tăng khi kích hoạt và giảm dần theo thời gian.
- Xác suất vẫn ràng buộc trong [0,1] (tính chất toán học của sigmoid), không phải cap gameplay.

## 🔧 **Cấu Hình Tính Năng (advanced_stats.yaml)**

```yaml
version: 1
features:
  skill_execution_speed: true
  skill_cooldown_reduction: true
  parry_rate: true
  block_rate: true
  element_penetration: true
  element_absorption: true
  element_reflection: true
  resource_regeneration: true
  resource_efficiency: true

  element_movement_speed: false
  element_teleportation: false
  element_self_healing: false
  element_group_healing: false
  element_leadership_bonus: false
  element_teaching_efficiency: false
  element_crafting_efficiency: false
  element_resource_discovery: false
  element_sensitivity: false
  mastery_synergy_bonus: false

dynamics:
  intensity_gain: 0.02
  intensity_damping: 0.01
  decay_rate: 0.05
  refractory_gain: 0.5
  refractory_decay: 0.1

integration:
  use_probability_engine: true
  probability_config_path: "probability_config.yaml" # xem 01_Probability_Mechanics_Design.md
```

## 🔗 **Nhất Quán Hệ Thống**

- Công thức chênh lệch stat và xác suất phải dùng Probability Engine (sigmoid/custom_sigmoid) được mô tả ở 01_Probability_Mechanics_Design.md.
- Thứ tự tính damage, áp dụng status, và quy tắc Omni additive-only tuân theo 06_Implementation_Notes.md.
- Các tham số (steepness/scaling) cấu hình tập trung trong `probability_config.yaml`; tài liệu này không lặp lại công thức mà chỉ tham chiếu.
- Checklist cấu hình phần tử: xem `README.md` → “Element Config Validation Checklist”.
- Cấu hình trung tâm liên quan:
  - `docs/element-core/configs/probability_config.yaml`
  - `docs/element-core/configs/interaction_config.yaml`
  - `docs/element-core/configs/status_pool.yaml`

## 🧪 **Testing Checklist (MVP)**

- Unit tests cho clamp và cap của từng stat chính (cooldown, penetration, absorption…).
- Property tests đảm bảo tăng mastery không vượt cap toàn cục và vẫn đơn điệu.
- Integration tests bảo đảm bật/tắt feature flags không ảnh hưởng stats khác.

## 🎯 **Advanced Derived Stats Categories**

### **1. Skill Execution & Performance**

#### **SkillExecutionSpeed**
- **Mục đích**: Tăng tốc độ thi triển skill dựa trên element mastery
- **Formula**: `base_speed * (1 + mastery_level * 0.001)`
- **Max Value**: 3.0x (300% speed)
- **Application**: Tất cả skills của element đó

```rust
// Skill execution speed calculation
fn calculate_skill_execution_speed(
    base_speed: f64,
    element_mastery: f64,
    skill_complexity: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.001; // 0.1% per mastery point
    let complexity_penalty = skill_complexity * 0.1; // 10% penalty per complexity
    base_speed * (1.0 + mastery_bonus - complexity_penalty)
}

// Example: 1000 fire mastery, skill complexity 2.0
// Speed = base_speed * (1 + 1.0 - 0.2) = base_speed * 1.8 (80% faster)
```

#### **SkillCooldownReduction**
- **Mục đích**: Giảm cooldown của skills
- **Formula**: `cooldown * (1 - mastery_level * 0.0005)`
- **Max Value**: 0.5x (50% cooldown reduction)
- **Application**: Tất cả skills của element đó

```rust
// Skill cooldown reduction
fn calculate_skill_cooldown_reduction(
    base_cooldown: f64,
    element_mastery: f64
) -> f64 {
    let reduction = element_mastery * 0.0005; // 0.05% per mastery point
    base_cooldown * (1.0 - reduction.min(0.5)) // Max 50% reduction
}
```

#### **SkillResourceEfficiency**
- **Mục đích**: Giảm resource cost của skills
- **Formula**: `resource_cost * (1 - mastery_level * 0.0003)`
- **Max Value**: 0.7x (30% resource reduction)
- **Application**: Tất cả skills của element đó

```rust
// Skill resource efficiency
fn calculate_skill_resource_efficiency(
    base_resource_cost: f64,
    element_mastery: f64
) -> f64 {
    let efficiency = element_mastery * 0.0003; // 0.03% per mastery point
    base_resource_cost * (1.0 - efficiency.min(0.3)) // Max 30% reduction
}
```

#### **SkillCastTimeReduction**
- **Mục đích**: Giảm thời gian cast của skills
- **Formula**: `cast_time * (1 - mastery_level * 0.0008)`
- **Max Value**: 0.6x (40% cast time reduction)
- **Application**: Chỉ skills có cast time

### **2. Combat Defense Mechanics**

#### **ParryRate**
- **Mục đích**: Tỷ lệ parry attacks (lấy cảm hứng từ Diablo)
- **Formula**: `parry_rate = base_parry + mastery_level * 0.0002`
- **Max Value**: 0.75 (75% parry chance)
- **Application**: Khi bị tấn công bởi element đó

```rust
// Parry rate calculation
fn calculate_parry_rate(
    base_parry: f64,
    element_mastery: f64,
    attacker_element_mastery: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.0002; // 0.02% per mastery point
    let attacker_penalty = attacker_element_mastery * 0.0001; // 0.01% per attacker mastery
    (base_parry + mastery_bonus - attacker_penalty).min(0.75).max(0.0)
}
```

#### **BlockRate**
- **Mục đích**: Tỷ lệ block attacks (lấy cảm hứng từ Diablo)
- **Formula**: `block_rate = base_block + mastery_level * 0.0003`
- **Max Value**: 0.8 (80% block chance)
- **Application**: Khi bị tấn công bởi element đó

```rust
// Block rate calculation
fn calculate_block_rate(
    base_block: f64,
    element_mastery: f64,
    attacker_element_mastery: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.0003; // 0.03% per mastery point
    let attacker_penalty = attacker_element_mastery * 0.0001; // 0.01% per attacker mastery
    (base_block + mastery_bonus - attacker_penalty).min(0.8).max(0.0)
}
```

#### Parry/Block Counter-Stats and Strength/Shred

- Parry/Block checks are passive and do not scale with any `skill_*_effectiveness`.
- Use yin–yang deltas with sigmoid for triggers; use deltas for mitigation magnitude.

```text
Parry trigger (defender vs attacker):
  p_parry = sigmoid( s × (parry_rate_defender − parry_break_attacker) )

Block trigger (defender vs attacker):
  p_block = sigmoid( s × (block_rate_defender − block_break_attacker) )

Block mitigation value (applied on pre-shield damage):
  block_value = f(block_strength_defender − block_shred_attacker)
  // f can be linear or bounded-sigmoid per balance; recommended linear→clamped at engine bounds

Parry outcome scaling (optional, engine-defined):
  parry_outcome_scale ∝ max(0, parry_strength_defender − parry_shred_attacker)
  // e.g., affects counter-window length, stagger time, or converted damage share
```

Implementation notes:
- Order in Damage Composition: resolve Hit→Parry→Block before penetration/defense. If Parry succeeds, short-circuit damage (engine-defined outcome). If Block succeeds, reduce incoming damage by `block_value` before further mitigation and before shields.
- Telemetry: log `(parry_rate_def, parry_break_att, p_parry)` and `(block_rate_def, block_break_att, p_block, block_value)` alongside existing `(Δ, I, R, p)`.

### **3. Element Mastery Bonuses**

#### **MasteryExperienceGain**
- **Mục đích**: Tăng experience gain khi training element
- **Formula**: `base_exp * (1 + mastery_level * 0.0002)`
- **Max Value**: 2.0x (100% exp bonus)
- **Application**: Khi training element mastery

```rust
// Mastery experience gain
fn calculate_mastery_exp_gain(
    base_exp: f64,
    element_mastery: f64,
    training_intensity: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.0002; // 0.02% per mastery point
    let intensity_multiplier = 1.0 + training_intensity * 0.5; // 50% per intensity
    base_exp * (1.0 + mastery_bonus) * intensity_multiplier
}
```

#### **MasteryDecayResistance**
- **Mục đích**: Giảm decay rate của element mastery
- **Formula**: `decay_rate * (1 - mastery_level * 0.0001)`
- **Max Value**: 0.5x (50% decay reduction)
- **Application**: Khi tính decay của mastery

```rust
// Mastery decay resistance
fn calculate_mastery_decay_resistance(
    base_decay_rate: f64,
    element_mastery: f64
) -> f64 {
    let resistance = element_mastery * 0.0001; // 0.01% per mastery point
    base_decay_rate * (1.0 - resistance.min(0.5)) // Max 50% reduction
}
```

#### **MasteryTrainingSpeed**
- **Mục đích**: Tăng tốc độ training element mastery
- **Formula**: `training_time * (1 - mastery_level * 0.0003)`
- **Max Value**: 0.7x (30% training time reduction)
- **Application**: Khi training element mastery

#### **MasterySynergyBonus**
- **Mục đích**: Bonus khi sử dụng nhiều elements cùng lúc
- **Formula**: `synergy_bonus = min(elements_count * 0.1, 0.5)`
- **Max Value**: 0.5x (50% synergy bonus)
- **Application**: Khi sử dụng multiple elements

### **4. Skill Effects & Mobility**

#### **ElementMovementSpeed**
- **Mục đích**: Tăng tốc độ di chuyển trong môi trường phù hợp
- **Formula**: `movement_speed = base_speed * (1 + mastery_level * 0.0001)`
- **Max Value**: 2.0x (100% speed bonus)
- **Application**: Khi di chuyển trong element-appropriate terrain

```rust
// Element movement speed calculation
fn calculate_element_movement_speed(
    base_speed: f64,
    element_mastery: f64,
    terrain_type: &str,
    element_type: &str
) -> f64 {
    let mastery_bonus = element_mastery * 0.0001; // 0.01% per mastery point
    
    // Terrain compatibility check
    let terrain_multiplier = match (terrain_type, element_type) {
        ("water", "water") => 1.5,      // Water mastery in water
        ("fire", "fire") => 1.3,        // Fire mastery in lava
        ("earth", "earth") => 1.4,      // Earth mastery on ground
        ("air", "air") => 1.6,          // Air mastery in sky
        _ => 1.0,                       // No bonus
    };
    
    base_speed * (1.0 + mastery_bonus) * terrain_multiplier
}
```

#### **ElementTeleportation**
- **Mục đích**: Teleportation abilities với high mastery
- **Formula**: `teleport_chance = mastery_level * 0.00001`
- **Max Value**: 0.3 (30% teleport chance)
- **Application**: Short-range teleportation (max 10 meters)

```rust
// Element teleportation calculation
fn calculate_element_teleportation(
    element_mastery: f64,
    element_type: &str
) -> (f64, f64) {
    let teleport_chance = element_mastery * 0.00001; // 0.001% per mastery point
    
    // Different elements have different teleport ranges
    let max_range = match element_type {
        "space" => 20.0,        // Space mastery: 20m range
        "time" => 15.0,         // Time mastery: 15m range
        "void" => 25.0,         // Void mastery: 25m range
        "reality" => 10.0,      // Reality mastery: 10m range
        _ => 5.0,               // Other elements: 5m range
    };
    
    (teleport_chance.min(0.3), max_range)
}
```

#### **ElementSelfHealing**
- **Mục đích**: Self-heal khi sử dụng element skills
- **Formula**: `heal_amount = skill_cost * mastery_level * 0.0001`
- **Max Value**: 0.2x (20% of skill cost as healing)
- **Application**: Khi sử dụng element skills

```rust
// Element self-healing calculation
fn calculate_element_self_healing(
    skill_resource_cost: f64,
    element_mastery: f64,
    element_type: &str
) -> f64 {
    let base_heal = skill_resource_cost * element_mastery * 0.0001;
    
    // Different elements have different healing efficiency
    let element_multiplier = match element_type {
        "life" => 2.0,          // Life mastery: 2x healing
        "water" => 1.5,         // Water mastery: 1.5x healing
        "earth" => 1.2,         // Earth mastery: 1.2x healing
        "fire" => 0.8,          // Fire mastery: 0.8x healing
        _ => 1.0,               // Default
    };
    
    (base_heal * element_multiplier).min(skill_resource_cost * 0.2)
}
```

#### **ElementGroupHealing**
- **Mục đích**: Heal team members khi sử dụng element skills
- **Formula**: `group_heal = skill_cost * mastery_level * 0.00005`
- **Max Value**: 0.1x (10% of skill cost as group healing)
- **Application**: Khi sử dụng element skills trong team

```rust
// Element group healing calculation
fn calculate_element_group_healing(
    skill_resource_cost: f64,
    element_mastery: f64,
    team_size: u32,
    element_type: &str
) -> f64 {
    let base_heal = skill_resource_cost * element_mastery * 0.00005;
    let team_multiplier = 1.0 + (team_size as f64 - 1.0) * 0.1; // 10% per team member
    
    // Different elements have different group healing efficiency
    let element_multiplier = match element_type {
        "life" => 3.0,          // Life mastery: 3x group healing
        "water" => 2.0,         // Water mastery: 2x group healing
        "light" => 2.5,         // Light mastery: 2.5x group healing
        _ => 1.0,               // Default
    };
    
    (base_heal * team_multiplier * element_multiplier).min(skill_resource_cost * 0.1)
}
```

### **5. Social & Economy**

#### **ElementLeadershipBonus**
- **Mục đích**: Bonus cho team members khi có high mastery
- **Formula**: `team_bonus = mastery_level * 0.0001`
- **Max Value**: 0.2x (20% team bonus)
- **Application**: Passive bonus cho team members

```rust
// Element leadership bonus calculation
fn calculate_element_leadership_bonus(
    element_mastery: f64,
    team_member_element_mastery: f64,
    element_type: &str
) -> f64 {
    let base_bonus = element_mastery * 0.0001; // 0.01% per mastery point
    let member_penalty = team_member_element_mastery * 0.00005; // 0.005% per member mastery
    
    // Different elements have different leadership effectiveness
    let element_multiplier = match element_type {
        "fire" => 1.5,          // Fire mastery: 1.5x leadership
        "water" => 1.2,         // Water mastery: 1.2x leadership
        "earth" => 1.3,         // Earth mastery: 1.3x leadership
        _ => 1.0,               // Default
    };
    
    ((base_bonus - member_penalty) * element_multiplier).min(0.2).max(0.0)
}
```

#### **ElementTeachingEfficiency**
- **Mục đích**: Tăng exp gain cho players khác khi training cùng
- **Formula**: `teaching_bonus = mastery_level * 0.0002`
- **Max Value**: 0.5x (50% teaching bonus)
- **Application**: Khi training cùng players khác

```rust
// Element teaching efficiency calculation
fn calculate_element_teaching_efficiency(
    teacher_mastery: f64,
    student_mastery: f64,
    element_type: &str
) -> f64 {
    let base_bonus = teacher_mastery * 0.0002; // 0.02% per mastery point
    let student_penalty = student_mastery * 0.0001; // 0.01% per student mastery
    
    // Different elements have different teaching effectiveness
    let element_multiplier = match element_type {
        "water" => 1.5,         // Water mastery: 1.5x teaching
        "earth" => 1.3,         // Earth mastery: 1.3x teaching
        "fire" => 1.1,          // Fire mastery: 1.1x teaching
        _ => 1.0,               // Default
    };
    
    ((base_bonus - student_penalty) * element_multiplier).min(0.5).max(0.0)
}
```

#### **ElementCraftingEfficiency**
- **Mục đích**: Better crafting results với element mastery
- **Formula**: `crafting_bonus = mastery_level * 0.0001`
- **Max Value**: 0.3x (30% crafting bonus)
- **Application**: Khi crafting element-related items

```rust
// Element crafting efficiency calculation
fn calculate_element_crafting_efficiency(
    element_mastery: f64,
    item_element_type: &str,
    crafting_skill_level: f64
) -> f64 {
    let base_bonus = element_mastery * 0.0001; // 0.01% per mastery point
    let skill_bonus = crafting_skill_level * 0.01; // 1% per skill level
    
    // Different elements have different crafting effectiveness
    let element_multiplier = match item_element_type {
        "fire" => 1.5,          // Fire mastery: 1.5x fire item crafting
        "water" => 1.3,         // Water mastery: 1.3x water item crafting
        "earth" => 1.4,         // Earth mastery: 1.4x earth item crafting
        _ => 1.0,               // Default
    };
    
    ((base_bonus + skill_bonus) * element_multiplier).min(0.3)
}
```

#### **ElementResourceDiscovery**
- **Mục đích**: Higher chance to find element-related resources
- **Formula**: `discovery_chance = mastery_level * 0.00005`
- **Max Value**: 0.2 (20% discovery chance)
- **Application**: Khi exploring/gathering resources

```rust
// Element resource discovery calculation
fn calculate_element_resource_discovery(
    element_mastery: f64,
    resource_element_type: &str,
    exploration_skill_level: f64
) -> f64 {
    let base_chance = element_mastery * 0.00005; // 0.005% per mastery point
    let skill_bonus = exploration_skill_level * 0.001; // 0.1% per skill level
    
    // Different elements have different discovery effectiveness
    let element_multiplier = match resource_element_type {
        "earth" => 2.0,         // Earth mastery: 2x earth resource discovery
        "water" => 1.5,         // Water mastery: 1.5x water resource discovery
        "fire" => 1.3,          // Fire mastery: 1.3x fire resource discovery
        _ => 1.0,               // Default
    };
    
    ((base_chance + skill_bonus) * element_multiplier).min(0.2)
}
```

### **6. Perception & Detection**

#### **ElementSensitivity**
- **Mục đích**: Detect enemies và resources với same element
- **Formula**: `detection_range = base_range * (1 + mastery_level * 0.0001)`
- **Max Value**: 3.0x (200% range bonus)
- **Application**: Detect enemies/resources với same element

```rust
// Element sensitivity calculation
fn calculate_element_sensitivity(
    base_detection_range: f64,
    element_mastery: f64,
    target_element_type: &str,
    current_element_type: &str
) -> f64 {
    let mastery_bonus = element_mastery * 0.0001; // 0.01% per mastery point
    
    // Same element detection bonus
    let element_match_bonus = if target_element_type == current_element_type {
        1.5 // 50% bonus for same element
    } else {
        1.0 // No bonus for different element
    };
    
    base_detection_range * (1.0 + mastery_bonus) * element_match_bonus
}
```

### **7. Advanced Combat Mechanics**

#### **ElementPenetration**
- **Mục đích**: Xuyên thủng defense của target
- **Formula**: `penetration = mastery_level * 0.0001`
- **Max Value**: 0.8x (80% penetration)
- **Application**: Khi tính damage vs defense

```rust
// Element penetration calculation
fn calculate_element_penetration(
    target_defense: f64,
    element_mastery: f64
) -> f64 {
    let penetration = element_mastery * 0.0001; // 0.01% per mastery point
    target_defense * (1.0 - penetration.min(0.8)) // Max 80% penetration
}
```

#### **ElementAbsorption**
- **Mục đích**: Hấp thụ damage từ element đó
- **Formula**: `absorption = mastery_level * 0.0002`
- **Max Value**: 0.6x (60% absorption)
- **Application**: Khi nhận damage từ element đó

#### **ElementReflection**
- **Mục đích**: Phản xạ damage về attacker
- **Formula**: `reflection = mastery_level * 0.0001`
- **Max Value**: 0.4x (40% reflection)
- **Application**: Khi nhận damage từ element đó

#### **ElementConversion**
- **Mục đích**: Chuyển đổi damage type
- **Formula**: `conversion_chance = mastery_level * 0.00005`
- **Max Value**: 0.3x (30% conversion chance)
- **Application**: Khi gây damage

### **4. Resource Management**

#### **ResourceRegeneration**
- **Mục đích**: Tái tạo tất cả resources dựa trên element mastery
- **Formula**: `resource_regen = base_regen * (1 + mastery_level * 0.0001)`
- **Max Value**: 3.0x (200% regen bonus)
- **Application**: Passive regeneration cho tất cả resources (HP, Stamina, Mana, etc.)

```rust
// Resource regeneration bonus
fn calculate_resource_regeneration(
    base_regen: f64,
    element_mastery: f64,
    element_count: u32,
    resource_type: &str
) -> f64 {
    let mastery_bonus = element_mastery * 0.0001; // 0.01% per mastery point
    let multi_element_bonus = element_count as f64 * 0.1; // 10% per element
    
    // Different resources have different scaling
    let resource_multiplier = match resource_type {
        "hp" => 1.0,           // Health regeneration
        "stamina" => 1.2,      // Stamina regeneration (higher)
        "mana" => 0.8,         // Mana regeneration (lower)
        "qi" => 1.5,           // Qi regeneration (highest)
        _ => 1.0,              // Default
    };
    
    base_regen * (1.0 + mastery_bonus + multi_element_bonus) * resource_multiplier
}
```

#### **ResourceEfficiency**
- **Mục đích**: Giảm resource cost của tất cả actions
- **Formula**: `efficiency = mastery_level * 0.0001`
- **Max Value**: 0.5x (50% cost reduction)
- **Application**: Tất cả resource-consuming actions

```rust
// Resource efficiency calculation
fn calculate_resource_efficiency(
    base_cost: f64,
    element_mastery: f64,
    resource_type: &str
) -> f64 {
    let efficiency = element_mastery * 0.0001; // 0.01% per mastery point
    
    // Different resources have different efficiency scaling
    let resource_multiplier = match resource_type {
        "hp" => 0.8,           // Health cost reduction (lower)
        "stamina" => 1.0,      // Stamina cost reduction (normal)
        "mana" => 1.2,         // Mana cost reduction (higher)
        "qi" => 1.5,           // Qi cost reduction (highest)
        _ => 1.0,              // Default
    };
    
    let total_efficiency = efficiency * resource_multiplier;
    base_cost * (1.0 - total_efficiency.min(0.5)) // Max 50% reduction
}
```

## 🔧 **Implementation Strategy**

### **Phase 1: Core Stats**
1. **SkillExecutionSpeed** - Tốc độ thi triển skill
2. **SkillCooldownReduction** - Giảm cooldown
3. **ParryRate** - Tỷ lệ parry attacks
4. **BlockRate** - Tỷ lệ block attacks

### **Phase 2: Skill Effects & Mobility**
1. **ElementMovementSpeed** - Tốc độ di chuyển
2. **ElementTeleportation** - Teleportation abilities
3. **ElementSelfHealing** - Self-healing
4. **ElementGroupHealing** - Group healing

### **Phase 3: Social & Economy**
1. **ElementLeadershipBonus** - Team bonuses
2. **ElementTeachingEfficiency** - Teaching bonuses
3. **ElementCraftingEfficiency** - Crafting bonuses
4. **ElementResourceDiscovery** - Resource discovery

### **Phase 4: Advanced Features**
1. **ElementSensitivity** - Detection abilities
2. **ElementPenetration** - Defense penetration
3. **ElementAbsorption** - Damage absorption
4. **ElementReflection** - Damage reflection

### **Phase 5: Mastery & Resources**
1. **MasteryExperienceGain** - Experience bonuses
2. **MasterySynergyBonus** - Multi-element bonuses
3. **ResourceRegeneration** - Resource regeneration
4. **ResourceEfficiency** - Resource efficiency

## 📊 **Stat Weights & Priorities**

```yaml
stat_weights:
  # Skill Performance (High Priority)
  skill_execution_speed: 1.0      # Most important
  skill_cooldown_reduction: 0.9   # Very important
  skill_resource_efficiency: 0.8  # Important
  skill_cast_time_reduction: 0.7  # Important
  
  # Combat Defense (High Priority)
  parry_rate: 0.9                 # Very important
  block_rate: 0.8                 # Important
  
  # Skill Effects & Mobility (Medium Priority)
  element_movement_speed: 0.7     # Important
  element_teleportation: 0.6      # Medium priority
  element_self_healing: 0.8       # Important
  element_group_healing: 0.6      # Medium priority
  
  # Social & Economy (Medium Priority)
  element_leadership_bonus: 0.7   # Important
  element_teaching_efficiency: 0.6 # Medium priority
  element_crafting_efficiency: 0.8 # Important
  element_resource_discovery: 0.7  # Important
  
  # Perception & Detection (Medium Priority)
  element_sensitivity: 0.6        # Medium priority
  
  # Advanced Combat (Medium Priority)
  element_penetration: 0.7        # Important
  element_absorption: 0.6         # Medium priority
  element_reflection: 0.5         # Medium priority
  
  # Mastery & Resources (High Priority)
  mastery_experience_gain: 0.9    # Very important
  mastery_synergy_bonus: 0.6      # Medium priority
  resource_regeneration: 0.8      # Important
  resource_efficiency: 0.7        # Important
```

## 🎮 **Game Impact**

### **Strategic Depth**
- **Mastery Investment**: Players phải cân nhắc invest vào element nào
- **Skill Optimization**: Tối ưu hóa skill execution dựa trên mastery
- **Resource Management**: Efficient resource usage với mastery bonuses

### **Progression Rewards**
- **Early Game**: Basic stats, low mastery
- **Mid Game**: Moderate bonuses, balanced mastery
- **Late Game**: High bonuses, specialized mastery
- **End Game**: Maximum bonuses, multi-element synergy

### **Meta Game**
- **Element Specialization**: Focus vào 1-2 elements
- **Element Diversification**: Spread mastery across elements
- **Hybrid Builds**: Balance giữa specialization và diversification

## 🚀 **Future Enhancements**

### **Planned Features**
- **Element Fusion**: Kết hợp multiple elements
- **Mastery Prestige**: Reset mastery với permanent bonuses
- **Element Mastery Trees**: Skill trees cho từng element
- **Dynamic Scaling**: Stats scale với game difficulty

### **Advanced Mechanics**
- **Element Resonance**: Synergy giữa compatible elements
- **Element Conflict**: Penalty cho conflicting elements
- **Element Evolution**: Elements evolve với high mastery
- **Element Mastery Quests**: Special quests cho mastery progression

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
