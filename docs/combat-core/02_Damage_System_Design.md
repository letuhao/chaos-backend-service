# Damage System Design

## 📋 **Tổng Quan**

Damage System là trung tâm của Combat Core, xử lý tất cả các loại sát thương trong game. Hệ thống được thiết kế để hỗ trợ nhiều loại damage khác nhau, từ vật lý cơ bản đến các loại damage phức tạp trong cultivation systems.

**Integration với Element-Core**: Damage System sử dụng hybrid approach, trong đó Element-Core cung cấp element stats (bao gồm Omni stats và Elemental Mastery stats) và Combat-Core thực hiện damage calculation dựa trên những stats này.

**⚠️ Critical Implementation Notes**: Xem [Element Core Implementation Notes](../element-core/06_Implementation_Notes.md) để biết các yêu cầu implementation quan trọng, bao gồm damage composition law, Omni additive-only rule, và status hit dependency.

**🎯 Elemental Mastery Integration**: Xem [Elemental Mastery System Design](../element-core/08_Elemental_Mastery_System_Design.md) và [Actor Core Integration Guide](../element-core/09_Actor_Core_Integration_Guide.md) để hiểu cách Elemental Mastery System tích hợp vào damage calculation.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Flexible & Extensible**
- Hỗ trợ nhiều loại damage khác nhau
- Dễ dàng thêm mới damage types
- Tương thích với các cultivation systems

### **2. Realistic & Balanced**
- Defense có thể chặn được damage
- Absolute damage đảm bảo vẫn có damage
- Cân bằng giữa offense và defense

### **3. Performance Optimized**
- Tính toán nhanh và hiệu quả
- Caching cho các calculations phức tạp
- Batch processing cho multiple targets

## 🏗️ **Kiến Trúc Damage System**

### **Core Components**

```
Damage System
├── Element-Core Integration
│   ├── Element Stats Provider
│   ├── Omni Stats Integration
│   ├── Elemental Mastery Stats Integration
│   ├── Element Interaction Calculator
│   └── Status Effect Calculator
├── Damage Calculation Engine
│   ├── Base Damage Calculation
│   ├── Power Points Calculation (from Element-Core + Mastery)
│   ├── Defense Calculation (from Element-Core + Mastery)
│   ├── Multiplier Application
│   └── Critical Hit Processing (from Element-Core + Mastery)
├── Damage Types & Categories
│   ├── Physical Damage
│   ├── Magical Damage
│   ├── Elemental Damage (Element-Core + Mastery)
│   ├── True Damage
│   └── Special Damage
├── DoT (Damage over Time) System
│   ├── DoT Manager
│   ├── Tick Processing
│   └── DoT Effects (Element-Core + Mastery)
├── Damage Events & Logging
│   ├── Event System
│   ├── Damage Tracking
│   └── Analytics
└── Damage Validation & Anti-cheat
    ├── Range Validation
    ├── Type Validation
    └── Limit Enforcement
```

## ⚠️ **Critical Implementation Requirements**

### **1. Damage Composition Law**

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

#### **Omni Additive-Only Rule**
```rust
// Omni stats chỉ cộng, không nhân
let total_power = omni_power + element_power;  // ✅ Correct
let total_power = omni_power * element_power;  // ❌ Wrong - causes snowball

// Tương tự cho tất cả stats
let total_defense = omni_defense + element_defense;
let total_crit_rate = omni_crit_rate + element_crit_rate;
let total_accuracy = omni_accuracy + element_accuracy;
```

#### **Status Hit Dependency**
```rust
// Status chỉ apply khi hit thành công
if !hit_success && status_config.requires_hit {
    return; // Không apply status nếu miss
}

// Calculate status probability
let status_prob = calculate_status_probability(attacker_stats, defender_stats);

// Apply status if probability check passes
if status_prob > random_threshold {
    apply_status_effect(status_effect, duration, intensity);
}
```

### Parry/Block Placement (Passive, pre-mitigation)

- Resolve in this order for each hit: `HitCheck → Parry → Block → Penetration/Defense → Reflection → Shields → Resources`.
- Parry trigger:

```rust
let p_parry = sigmoid(scale * (parry_rate_def - parry_break_att));
if rng.next() < p_parry { return apply_parry_outcome(attacker, target); }
```

- Block trigger and magnitude:

```rust
let p_block = sigmoid(scale * (block_rate_def - block_break_att));
if rng.next() < p_block {
    let block_value = compute_block_value(block_strength_def - block_shred_att);
    damage.final_damage = (damage.final_damage - block_value).max(0.0);
}
```

- Do not apply any `skill_*_effectiveness` to these checks. They are passive.

## 🔗 **Element-Core Integration**

### **1. Hybrid Architecture với Elemental Mastery**

Combat-Core sử dụng hybrid approach để tích hợp với Element-Core và Elemental Mastery System:

```rust
// Element-Core provides stats including mastery
pub struct ElementStatsProvider {
    element_core: Arc<ElementCore>,
    mastery_provider: Arc<ElementMasteryStatsProvider>,
}

impl ElementStatsProvider {
    pub fn get_combat_stats(&self, attacker: &Actor, target: &Actor, element_type: &str) -> CombatElementStats {
        // Get Omni + Element + Mastery stats from Element-Core
        let attacker_omni = self.element_core.get_omni_stats(attacker);
        let attacker_element = self.element_core.get_element_stats(attacker, element_type);
        let attacker_mastery = self.mastery_provider.get_element_derived_stats(attacker, element_type).await?;
        
        let target_omni = self.element_core.get_omni_stats(target);
        let target_element = self.element_core.get_element_stats(target, element_type);
        let target_mastery = self.mastery_provider.get_element_derived_stats(target, element_type).await?;
        
        CombatElementStats {
            // Power stats (Omni + Element + Mastery)
            attacker_power: attacker_omni.power + attacker_element.power + attacker_mastery.get("attack_power", 0.0),
            target_defense: target_omni.defense + target_element.defense + target_mastery.get("defense", 0.0),
            
            // Critical stats (Omni + Element + Mastery)
            attacker_crit_rate: attacker_omni.crit_rate + attacker_element.crit_rate + attacker_mastery.get("crit_rate", 0.0),
            attacker_crit_damage: attacker_omni.crit_damage + attacker_element.crit_damage + attacker_mastery.get("crit_damage", 0.0),
            target_resist_crit: target_omni.resist_crit + target_element.resist_crit + target_mastery.get("resist_crit", 0.0),
            target_resist_crit_damage: target_omni.resist_crit_damage + target_element.resist_crit_damage + target_mastery.get("resist_crit_damage", 0.0),
            
            // Accuracy stats (Omni + Element + Mastery)
            attacker_accuracy: attacker_omni.accuracy + attacker_element.accuracy + attacker_mastery.get("accuracy", 0.0),
            target_dodge: target_omni.dodge + target_element.dodge + target_mastery.get("dodge", 0.0),
            
            // Status effect stats (Omni + Element + Mastery)
            attacker_status_prob: attacker_omni.status_prob + attacker_element.status_prob + attacker_mastery.get("status_prob", 0.0),
            target_status_resist: target_omni.status_resist + target_element.status_resist + target_mastery.get("status_resist", 0.0),
            
            // Element interactions (affected by mastery)
            damage_multiplier: self.element_core.get_damage_multiplier(element_type, target.get_primary_element()),
            
            // Mastery-specific stats
            mastery_bonus: self.calculate_mastery_bonus(attacker_mastery, target_mastery),
        }
    }
    
    /// Calculate mastery-based damage bonus
    fn calculate_mastery_bonus(&self, attacker_mastery: &HashMap<String, f64>, target_mastery: &HashMap<String, f64>) -> f64 {
        let attacker_mastery_level = attacker_mastery.get("mastery_level").unwrap_or(&0.0);
        let target_mastery_level = target_mastery.get("mastery_level").unwrap_or(&0.0);
        
        // Mastery difference affects damage bonus
        let mastery_difference = attacker_mastery_level - target_mastery_level;
        let mastery_bonus = mastery_difference * 0.01; // 1% per mastery point difference
        
        // No cap - mastery can provide unlimited bonus/penalty
        mastery_bonus
    }
}
```

### **2. Combat-Core Integration với Mastery**

```rust
// Combat-Core uses Element-Core stats including mastery
impl CombatCore {
    pub async fn calculate_damage(&self, action: &Action, attacker: &Actor, target: &Actor) -> DamageResult {
        // 1. Get element stats from Element-Core (including mastery)
        let element_stats = self.element_stats_provider.get_combat_stats(
            attacker, 
            target, 
            action.element_type
        ).await?;
        
        // 2. Create damage input with element stats + mastery
        let damage_input = DamageInput {
            base_damage: action.base_damage,
            power_points: vec![element_stats.attacker_power],
            target_defense: element_stats.target_defense,
            critical_chance: element_stats.attacker_crit_rate,
            critical_multiplier: element_stats.attacker_crit_damage,
            accuracy: element_stats.attacker_accuracy,
            target_dodge: element_stats.target_dodge,
            element_multiplier: element_stats.damage_multiplier,
            mastery_bonus: element_stats.mastery_bonus,
            // ... other fields
        };
        
        // 3. Calculate damage using existing formula
        let mut damage_result = self.damage_calculator.calculate_final_damage(damage_input, target);
        
        // 4. Apply element-specific effects
        damage_result.final_damage *= element_stats.damage_multiplier;
        
        // 5. Apply mastery bonus
        damage_result.final_damage *= (1.0 + element_stats.mastery_bonus);
        
        // 6. Apply status effects if applicable (affected by mastery)
        if self.should_apply_status_effects(action, element_stats) {
            let status_effects = self.element_core.calculate_status_effects(
                attacker, target, action.element_type
            ).await?;
            damage_result.status_effects = status_effects;
        }
        
        // 7. Log mastery-based damage
        if element_stats.mastery_bonus != 0.0 {
            info!(
                attacker = %attacker.id,
                target = %target.id,
                element = %action.element_type,
                mastery_bonus = %element_stats.mastery_bonus,
                final_damage = %damage_result.final_damage,
                "Mastery-based damage calculation"
            );
        }
        
        damage_result
    }
}
```

### **3. Benefits of Hybrid Approach với Mastery**

#### **Separation of Concerns**
- **Element-Core**: Quản lý element stats, interactions, status effects
- **Elemental Mastery System**: Quản lý mastery progression, decay, training
- **Combat-Core**: Quản lý combat mechanics, action processing, event handling

#### **Performance**
- **Element-Core**: Có thể cache element calculations
- **Elemental Mastery System**: Có thể cache mastery calculations
- **Combat-Core**: Có thể cache combat calculations
- **Minimal overhead**: Chỉ pass data, không duplicate calculations

#### **Flexibility**
- **Element-Core**: Có thể được sử dụng bởi systems khác (Shield, Item, Race)
- **Elemental Mastery System**: Có thể được sử dụng bởi systems khác (Skills, Items, Locations)
- **Combat-Core**: Có thể sử dụng element stats từ nhiều sources
- **Easy testing**: Có thể test từng component riêng biệt

#### **Mastery Integration Benefits**
- **Progressive Power**: Mastery tăng dần theo thời gian tu luyện
- **Decay System**: Tạo động lực tu luyện liên tục
- **Element Specialization**: Players có thể chuyên sâu vào elements cụ thể
- **Balanced Meta**: Mastery difference tạo ra meta game cân bằng

## ⚔️ **Damage Categories Chuẩn**

### **Tham Khảo từ Các Game Nổi Tiếng**

#### **1. World of Warcraft**
- **Physical**: Melee, Ranged
- **Magical**: Arcane, Fire, Frost, Nature, Shadow, Holy
- **True**: Damage không thể chặn

#### **2. Final Fantasy XIV**
- **Physical**: Slashing, Piercing, Blunt
- **Magical**: Fire, Ice, Lightning, Earth, Wind, Water, Astral, Umbral
- **True**: Unaspected damage

#### **3. League of Legends**
- **Physical**: Attack damage
- **Magical**: Ability power
- **True**: Damage không thể giảm

#### **4. Genshin Impact**
- **Physical**: Normal attacks
- **Elemental**: Pyro, Hydro, Electro, Cryo, Anemo, Geo, Dendro
- **True**: Damage không thể chặn

#### **5. Diablo Series**
- **Physical**: Weapon damage
- **Elemental**: Fire, Cold, Lightning, Poison
- **Arcane**: Magic damage
- **True**: Damage không thể giảm

### **Damage Categories Chuẩn cho Cultivation Game**

```go
// Damage Categories
type DamageCategory string
const (
    // Physical Categories
    PhysicalCategory    DamageCategory = "physical"     // Vật lý cơ bản
    SlashingCategory    DamageCategory = "slashing"     // Chém
    PiercingCategory    DamageCategory = "piercing"     // Đâm
    BluntCategory       DamageCategory = "blunt"        // Đập
    CrushingCategory    DamageCategory = "crushing"     // Nghiền
    
    // Magical Categories
    MagicalCategory     DamageCategory = "magical"      // Ma pháp
    ArcaneCategory      DamageCategory = "arcane"       // Huyền bí
    MysticalCategory    DamageCategory = "mystical"     // Thần bí
    SpiritualCategory   DamageCategory = "spiritual"    // Tinh thần (category name; not to be confused with cultivation system naming)
    MentalCategory      DamageCategory = "mental"       // Tâm trí
    
    // Elemental Categories
    FireCategory        DamageCategory = "fire"         // Hỏa
    WaterCategory       DamageCategory = "water"        // Thủy
    EarthCategory       DamageCategory = "earth"        // Thổ
    AirCategory         DamageCategory = "air"          // Phong
    LightningCategory   DamageCategory = "lightning"    // Lôi
    IceCategory         DamageCategory = "ice"          // Băng
    PoisonCategory      DamageCategory = "poison"       // Độc
    DarkCategory        DamageCategory = "dark"         // Ám
    LightCategory       DamageCategory = "light"        // Quang
    
    // Cultivation Categories
    QiCategory          DamageCategory = "qi"           // Khí
    SpiritualCategory   DamageCategory = "spiritual"    // Tinh thần
    DaoCategory         DamageCategory = "dao"          // Đạo
    ProfoundCategory    DamageCategory = "profound"     // Áo nghĩa
    KarmaCategory       DamageCategory = "karma"        // Nghiệp
    FateCategory        DamageCategory = "fate"         // Số mệnh
    
    // Special Categories
    TrueCategory        DamageCategory = "true"         // Sát thương thật
    HealingCategory     DamageCategory = "healing"      // Hồi máu
    DrainCategory       DamageCategory = "drain"        // Hút máu/mana
    ReflectCategory     DamageCategory = "reflect"      // Phản đòn
    AbsorbCategory      DamageCategory = "absorb"       // Hấp thụ
    ChaosCategory       DamageCategory = "chaos"        // Hỗn mang
    RealityCategory     DamageCategory = "reality"      // Thực tại
    ConceptualCategory  DamageCategory = "conceptual"   // Khái niệm
)

// Damage Types (chi tiết hơn)
type DamageType string
const (
    // Physical Types
    SwordDamage         DamageType = "sword"           // Kiếm
    SpearDamage         DamageType = "spear"           // Thương
    AxeDamage           DamageType = "axe"             // Rìu
    BowDamage           DamageType = "bow"             // Cung
    FistDamage          DamageType = "fist"            // Quyền
    KickDamage          DamageType = "kick"            // Cước
    BodyDamage          DamageType = "body"            // Thân thể
    
    // Magical Types
    SpellDamage         DamageType = "spell"           // Phép thuật
    TechniqueDamage     DamageType = "technique"       // Kỹ thuật
    SkillDamage         DamageType = "skill"           // Kỹ năng
    TalentDamage        DamageType = "talent"          // Tài năng
    
    // Elemental Types
    FireballDamage      DamageType = "fireball"        // Cầu lửa
    IceShardDamage      DamageType = "ice_shard"       // Mảnh băng
    LightningBoltDamage DamageType = "lightning_bolt"  // Tia sét
    EarthSpikeDamage    DamageType = "earth_spike"     // Gai đất
    WindBladeDamage     DamageType = "wind_blade"      // Lưỡi gió
    WaterJetDamage      DamageType = "water_jet"       // Tia nước
    
    // Cultivation Types
    QiBlastDamage       DamageType = "qi_blast"        // Phóng khí
    SpiritualStrikeDamage DamageType = "spiritual_strike" // Đánh tinh thần (matches Spiritual category)
    DaoSwordDamage      DamageType = "dao_sword"       // Đạo kiếm
    ProfoundMeaningDamage DamageType = "profound_meaning" // Áo nghĩa
    
    // Special Types
    TrueStrikeDamage    DamageType = "true_strike"     // Đánh thật
    LifeDrainDamage     DamageType = "life_drain"      // Hút máu
    ManaDrainDamage     DamageType = "mana_drain"      // Hút mana
    ReflectDamage       DamageType = "reflect"         // Phản đòn
    AbsorbDamage        DamageType = "absorb"          // Hấp thụ
    ChaosDamage         DamageType = "chaos"           // Hỗn mang
    RealityBendDamage   DamageType = "reality_bend"    // Bẻ cong thực tại
)
```

## 🔢 **Công Thức Tính Damage**

### **1. Công Thức Chính với Mastery**

```
Final Damage = (BaseDamage + (PowerPoints - TargetDefense) + FlatAdditions) × TotalMultiplier × CriticalMultiplier × (1 + MasteryBonus) + AbsoluteDamage
```

**Trong đó:**
- **PowerPoints**: Omni Power + Element Power + Mastery Power
- **TargetDefense**: Omni Defense + Element Defense + Mastery Defense
- **MasteryBonus**: (Attacker Mastery - Target Mastery) × 0.01 (không có cap)

### **2. Derived Stats Calculation Flow**

#### **A. Primary Stats → Derived Stats Mapping**

**Từ Actor Core Resource Manager:**
```rust
// Primary Stats (từ equipment, level, cultivation)
let primary_stats = actor.get_primary_stats();
// Ví dụ: strength: 100, intelligence: 150, agility: 80, vitality: 120
```

**Element-Core tính Derived Stats:**
```rust
// Omni Stats (baseline cho tất cả elements)
let omni_stats = element_core.calculate_omni_derived_stats(primary_stats);
// Formula: omni_attack = (strength + intelligence) * 0.2
// = (100 + 150) * 0.2 = 50

// Fire Element Stats (specific cho fire)
let fire_element_stats = element_core.calculate_element_derived_stats(primary_stats, "fire");
// Formula: fire_attack = intelligence * 0.8 + strength * 0.4
// = 150 * 0.8 + 100 * 0.4 = 120 + 40 = 160
```

**Elemental Mastery System tính Mastery Stats:**
```rust
// Mastery Stats (từ mastery level và training)
let fire_mastery_stats = mastery_system.calculate_mastery_derived_stats(actor, "fire");
// Formula: mastery_attack = mastery_level * 1.5 + training_hours * 0.1
// = 150 * 1.5 + 500 * 0.1 = 225 + 50 = 275
```

#### **B. Stats Combination**

**Attacker Stats:**
```rust
// Tổng hợp tất cả derived stats cho attacker
let total_attack = omni_stats.attack + fire_element_stats.attack + fire_mastery_stats.attack;
// = 50 + 160 + 275 = 485

let total_defense = omni_stats.defense + fire_element_stats.defense + fire_mastery_stats.defense;
// = 30 + 80 + 200 = 310

let total_crit_rate = omni_stats.crit_rate + fire_element_stats.crit_rate + fire_mastery_stats.crit_rate;
// = 5 + 8 + 15 = 28
```

**Target Stats:**
```rust
// Tổng hợp tất cả derived stats cho target
let target_omni_defense = target.get_omni_defense(); // 40
let target_fire_defense = target.get_fire_element_defense(); // 120
let target_fire_mastery_defense = target.get_fire_mastery_defense(); // 200
let target_total_defense = target_omni_defense + target_fire_defense + target_fire_mastery_defense;
// = 40 + 120 + 200 = 360
```

### **3. Step-by-Step Damage Calculation**

#### **A. Tổng Quan Quy Trình**

```
Step 1: Thu thập Base Stats
Step 2: Tính Derived Stats (Element-Core)
Step 3: Tính Mastery Stats (Elemental Mastery System)
Step 4: Kết hợp tất cả Stats
Step 5: Tính Damage theo công thức
Step 6: Áp dụng Mastery Bonus
Step 7: Tính Final Damage
```

#### **B. Visual Flow Diagram**

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Actor Core    │    │   Element-Core  │    │ Elemental Mastery│
│ Resource Manager│    │                 │    │     System      │
│                 │    │                 │    │                 │
│ Primary Stats:  │───▶│ Derived Stats:  │    │ Mastery Stats:  │
│ • strength: 100 │    │ • omni_attack:  │    │ • mastery_attack│
│ • intel: 150    │    │   50            │    │   275           │
│ • agility: 80   │    │ • fire_attack:  │    │ • mastery_defense│
│ • vitality: 120 │    │   160           │    │   200           │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │                        │
                                ▼                        ▼
                       ┌─────────────────────────────────────────┐
                       │      Attacker Stats Combination        │
                       │                                         │
                       │ total_attack = 50 + 160 + 275 = 485    │
                       │ total_defense = 30 + 80 + 200 = 310    │
                       │ total_crit_rate = 5 + 8 + 15 = 28      │
                       └─────────────────────────────────────────┘
                                                │
                                                ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Actor Core    │    │   Element-Core  │    │ Elemental Mastery│
│ Resource Manager│    │                 │    │     System      │
│                 │    │                 │    │                 │
│ Primary Stats:  │───▶│ Derived Stats:  │    │ Mastery Stats:  │
│ • strength: 120 │    │ • omni_defense: │    │ • mastery_defense│
│ • intel: 80     │    │   40            │    │   200           │
│ • agility: 100  │    │ • fire_defense: │    │ • mastery_level │
│ • vitality: 140 │    │   120           │    │   100           │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │                        │
                                ▼                        ▼
                       ┌─────────────────────────────────────────┐
                       │       Target Stats Combination         │
                       │                                         │
                       │ target_omni_defense = 40               │
                       │ target_fire_defense = 120              │
                       │ target_mastery_defense = 200           │
                       │ target_total_defense = 40+120+200=360  │
                       └─────────────────────────────────────────┘
                                                │
                                                ▼
                       ┌─────────────────────────────────────────┐
                       │         Damage Calculation             │
                       │                                         │
                       │ base_damage = 500                      │
                       │ power_points = total_attack = 485      │
                       │ target_defense = target_total_defense  │
                       │ power_diff = 485 - 360 = 125           │
                       │ pre_multiplier = 500 + 125 = 625       │
                       │                                         │
                       │ mastery_diff = 150 - 100 = 50          │
                       │ mastery_bonus = 50 * 0.01 = 0.5        │
                       │                                         │
                       │ final = 625 * 1.2 * 2.0 * 1.5 = 2250   │
                       └─────────────────────────────────────────┘
```

#### **C. Chi Tiết Từng Step**

**Step 1: Thu thập Base Stats**
```rust
// Từ Actor Core Resource Manager
let primary_stats = actor.get_primary_stats();
// Ví dụ: strength: 100, intelligence: 150, agility: 80, vitality: 120
```

**Step 2: Tính Derived Stats (Element-Core)**
```rust
// Element-Core tính derived stats từ primary stats
let omni_stats = element_core.calculate_omni_derived_stats(primary_stats);
// Formula: omni_attack = (strength + intelligence) * 0.2
// = (100 + 150) * 0.2 = 50

let fire_element_stats = element_core.calculate_element_derived_stats(primary_stats, "fire");
// Formula: fire_attack = intelligence * 0.8 + strength * 0.4
// = 150 * 0.8 + 100 * 0.4 = 120 + 40 = 160
```

**Step 3: Tính Mastery Stats (Elemental Mastery System)**
```rust
// Elemental Mastery System tính mastery stats
let fire_mastery_stats = mastery_system.calculate_mastery_derived_stats(actor, "fire");
// Formula: mastery_attack = mastery_level * 1.5 + training_hours * 0.1
// = 150 * 1.5 + 500 * 0.1 = 225 + 50 = 275
```

**Step 4: Kết hợp tất cả Stats**
```rust
// Tổng hợp tất cả derived stats
let total_attack = omni_stats.attack + fire_element_stats.attack + fire_mastery_stats.attack;
// = 50 + 160 + 275 = 485

let total_defense = omni_stats.defense + fire_element_stats.defense + fire_mastery_stats.defense;
// = 30 + 80 + 200 = 310

let total_crit_rate = omni_stats.crit_rate + fire_element_stats.crit_rate + fire_mastery_stats.crit_rate;
// = 5 + 8 + 15 = 28
```

**Step 5: Tính Damage theo công thức**
```rust
// Base damage từ skill/action
let base_damage = 500.0;

// Power points calculation - sử dụng total_attack từ Step 4
let power_points = total_attack; // 485 (từ Step 4)

// Target defense calculation - tương tự như attacker
let target_omni_defense = target.get_omni_defense(); // 40
let target_fire_defense = target.get_fire_element_defense(); // 120
let target_fire_mastery_defense = target.get_fire_mastery_defense(); // 200
let target_total_defense = target_omni_defense + target_fire_defense + target_fire_mastery_defense;
// = 40 + 120 + 200 = 360

// Pre-multiplier damage
let pre_multiplier_damage = base_damage + (power_points - target_total_defense);
// = 500 + (485 - 360) = 500 + 125 = 625
```

**Step 6: Áp dụng Mastery Bonus**
```rust
// Tính mastery difference
let attacker_mastery = fire_mastery_stats.mastery_level; // 150
let target_mastery = target.get_fire_mastery_level(); // 100
let mastery_difference = attacker_mastery - target_mastery; // 50
let mastery_bonus = mastery_difference * 0.01; // 0.5 (50%)

// Áp dụng multipliers
let total_multiplier = 1.2; // từ skill/equipment
let critical_multiplier = 2.0; // nếu crit

// Damage với mastery bonus
let final_damage = pre_multiplier_damage * total_multiplier * critical_multiplier * (1.0 + mastery_bonus);
// = 675 * 1.2 * 2.0 * (1.0 + 0.5) = 675 * 1.2 * 2.0 * 1.5 = 2430
```

**Step 7: Tính Final Damage**
```rust
let absolute_damage = 100.0; // từ skill
let final_damage = final_damage + absolute_damage;
// = 2430 + 100 = 2530
```

#### **D. Ví Dụ Cụ Thể**

**Scenario**: Fire Mage (Level 50) tấn công Fire Warrior (Level 45)

**Attacker Stats:**
```
Primary Stats: strength=100, intelligence=150, agility=80, vitality=120
Omni Stats: attack=50, defense=30, crit_rate=5
Fire Element Stats: attack=160, defense=80, crit_rate=8
Fire Mastery Stats: attack=275, defense=200, crit_rate=15, mastery_level=150
```

**Target Stats:**
```
Primary Stats: strength=120, intelligence=80, agility=100, vitality=140
Omni Stats: attack=60, defense=40, crit_rate=6
Fire Element Stats: attack=100, defense=120, crit_rate=6
Fire Mastery Stats: attack=150, defense=200, crit_rate=10, mastery_level=100
```

**Damage Calculation:**
```
Step 1: Primary Stats ✓
Step 2: Derived Stats ✓
Step 3: Mastery Stats ✓
Step 4: Attacker Stats Combination
  - Attacker Total Attack: 50 + 160 + 275 = 485
  - Attacker Total Defense: 30 + 80 + 200 = 310
Step 4b: Target Stats Combination
  - Target Omni Defense: 40
  - Target Fire Defense: 120
  - Target Fire Mastery Defense: 200
  - Target Total Defense: 40 + 120 + 200 = 360
Step 5: Pre-multiplier Damage
  - Base Damage: 500
  - Power Points: 485 (total_attack từ Step 4)
  - Target Defense: 360 (target_total_defense từ Step 4b)
  - Power Difference: 485 - 360 = 125
  - Pre-multiplier: 500 + 125 = 625
Step 6: Mastery Bonus
  - Mastery Difference: 150 - 100 = 50
  - Mastery Bonus: 50 * 0.01 = 0.5 (50%)
Step 7: Final Damage
  - Multipliers: 1.2 * 2.0 = 2.4
  - With Mastery: 625 * 2.4 * 1.5 = 2250
  - Absolute: 2250 + 100 = 2350
```

### **4. Tóm Tắt Cách Thức Hoạt Động**

#### **A. Luồng Dữ Liệu (Data Flow)**

```
Primary Stats (Actor Core)
    ↓
Derived Stats (Element-Core)
    ↓
Mastery Stats (Elemental Mastery System)
    ↓
Combined Stats (Combat-Core)
    ↓
Damage Calculation
    ↓
Mastery Bonus Application
    ↓
Final Damage
```

#### **B. Các Thành Phần Chính**

1. **Primary Stats**: Từ equipment, level, cultivation (strength, intelligence, etc.)
2. **Omni Stats**: Baseline stats cho tất cả elements (từ Element-Core)
3. **Element Stats**: Specific stats cho từng element (từ Element-Core)
4. **Mastery Stats**: Stats từ mastery level và training (từ Elemental Mastery System)
5. **Combined Stats**: Tổng hợp tất cả stats (Omni + Element + Mastery)
6. **Mastery Bonus**: Bonus/penalty dựa trên mastery difference

#### **C. Công Thức Tổng Quan**

```
Final Damage = (BaseDamage + (TotalAttack - TotalDefense)) × Multipliers × (1 + MasteryBonus) + AbsoluteDamage

Trong đó:
- TotalAttack = OmniAttack + ElementAttack + MasteryAttack
- TotalDefense = OmniDefense + ElementDefense + MasteryDefense
- MasteryBonus = (AttackerMastery - TargetMastery) × 0.01
```

#### **D. Lợi Ích của Hệ Thống**

1. **Flexibility**: Dễ dàng thêm elements và mastery systems mới
2. **Balance**: Mastery difference tạo ra meta game cân bằng
3. **Progression**: Mastery tăng dần theo thời gian tu luyện
4. **Specialization**: Players có thể chuyên sâu vào elements cụ thể
5. **No Cap**: Mastery bonus không có giới hạn, tạo ra infinite progression

### **5. Công Thức Chi Tiết**

```go
// Công thức tính damage cuối cùng
func CalculateFinalDamage(input *DamageInput, target *Actor) *DamageResult {
    // 1. Random factor cho toàn bộ calculation
    randomFactor := rand.Float64()
    
    // 2. Tính Base Damage
    baseDamage := input.BaseDamage.Min + (input.BaseDamage.Max - input.BaseDamage.Min) * randomFactor
    
    // 3. Tính Power Points
    totalPowerPoints := 0.0
    for _, powerRange := range input.PowerPoints {
        powerValue := powerRange.Min + (powerRange.Max - powerRange.Min) * randomFactor
        totalPowerPoints += powerValue
    }
    
    // 4. Tính Flat Additions
    totalFlatAdditions := 0.0
    for _, additionRange := range input.Additions {
        additionValue := additionRange.Min + (additionRange.Max - additionRange.Min) * randomFactor
        totalFlatAdditions += additionValue
    }
    
    // 5. Tính Target Defense
    targetDefense := calculateTargetDefense(target, input.DamageTypes)
    
    // 6. Tính damage trước multipliers
    preMultiplierDamage := baseDamage + (totalPowerPoints - targetDefense) + totalFlatAdditions
    
    // 7. Kiểm tra nếu damage <= 0 thì chỉ còn Absolute Damage
    if preMultiplierDamage <= 0 {
        absoluteDamage := calculateAbsoluteDamage(input)
        return &DamageResult{
            FinalDamage: absoluteDamage,
            IsBlocked: true,
            AbsoluteDamage: absoluteDamage,
        }
    }
    
    // 8. Tính Multipliers
    totalMultiplier := 1.0
    for _, multiplier := range input.Multipliers {
        totalMultiplier *= multiplier
    }
    
    // 9. Tính Critical Multiplier
    criticalMultiplier := 1.0
    if rand.Float64() < input.CriticalChance {
        criticalMultiplier = input.CriticalMulti.Min + (input.CriticalMulti.Max - input.CriticalMulti.Min) * randomFactor
    }
    
    // 10. Tính Mastery Bonus
    masteryBonus := calculateMasteryBonus(attacker, target, input.ElementType)
    
    // 11. Tính damage cuối cùng
    postMultiplierDamage := preMultiplierDamage * totalMultiplier * criticalMultiplier * (1.0 + masteryBonus)
    absoluteDamage := calculateAbsoluteDamage(input)
    finalDamage := postMultiplierDamage + absoluteDamage
    
    return &DamageResult{
        FinalDamage: finalDamage,
        BaseDamage: baseDamage,
        PowerPoints: totalPowerPoints,
        FlatAdditions: totalFlatAdditions,
        TargetDefense: targetDefense,
        Multipliers: totalMultiplier,
        CriticalMultiplier: criticalMultiplier,
        MasteryBonus: masteryBonus,
        AbsoluteDamage: absoluteDamage,
        IsBlocked: false,
    }
}

// Tính Mastery Bonus
func calculateMasteryBonus(attacker *Actor, target *Actor, elementType string) float64 {
    // Get mastery levels
    attackerMastery := attacker.getElementMastery(elementType)
    targetMastery := target.getElementMastery(elementType)
    
    // Calculate mastery difference
    masteryDifference := attackerMastery - targetMastery
    
    // Calculate bonus (1% per mastery point difference)
    masteryBonus := masteryDifference * 0.01
    
    // No cap - mastery can provide unlimited bonus/penalty
    
    return masteryBonus
}
```

### **3. Công Thức Cho Từng Loại Impact**

```go
// Công thức tính damage cho từng loại impact
func CalculateImpactDamage(totalDamage float64, impact DamageImpact, target *Actor) map[string]float64 {
    result := make(map[string]float64)
    randomFactor := rand.Float64()
    
    for impactType, impactRange := range impact.ImpactDetails {
        // 1. Tính weight
        weight := impactRange.Min + (impactRange.Max - impactRange.Min) * randomFactor
        
        // 2. Tính damage cho loại này
        impactDamage := totalDamage * weight
        
        // 3. Tính defense cho loại này
        impactDefense := calculateDefenseForType(target, impactType)
        
        // 4. Áp dụng defense
        impactDamage = impactDamage - impactDefense
        
        // 5. Kiểm tra nếu damage <= 0 thì chỉ còn Absolute Damage
        if impactDamage <= 0 {
            absoluteDamage := calculateAbsoluteDamageForType(input, impactType)
            result[impactType] = absoluteDamage
            continue
        }
        
        // 6. Áp dụng multiplier
        if multiplier, exists := impact.Multipliers[impactType+"_multiplier"]; exists {
            impactDamage *= multiplier
        }
        
        // 7. Áp dụng penetration
        if penetration, exists := impact.Penetration[impactType+"_penetration"]; exists {
            impactDamage += penetration
        }
        
        // 8. Đảm bảo damage không âm
        if impactDamage < 0 {
            impactDamage = 0
        }
        
        result[impactType] = impactDamage
    }
    
    return result
}
```

## 🎭 **DoT (Damage over Time) System**

### **1. DoT Structure**

```go
// DoT Effect
type DoTEffect struct {
    ID              string            `json:"id"`
    Name            string            `json:"name"`
    DamageType      string            `json:"damage_type"`
    BaseDamage      DamageRange       `json:"base_damage"`
    TickInterval    int64             `json:"tick_interval"`    // ms
    TotalDuration   int64             `json:"total_duration"`   // ms
    MaxTicks        int               `json:"max_ticks"`
    CurrentTicks    int               `json:"current_ticks"`
    LastTickTime    int64             `json:"last_tick_time"`
    IsActive        bool              `json:"is_active"`
    SourceID        string            `json:"source_id"`
    TargetID        string            `json:"target_id"`
    Multipliers     map[string]float64 `json:"multipliers"`
    Penetration     map[string]float64 `json:"penetration"`
    Conditions      []DoTCondition    `json:"conditions"`
}

// DoT Condition
type DoTCondition struct {
    Type        string      `json:"type"`
    Operator    string      `json:"operator"`
    Value       float64     `json:"value"`
    Description string      `json:"description"`
}
```

### **2. DoT Calculation**

```go
// Tính DoT damage cho mỗi tick
func (cc *CombatCore) CalculateDoTDamage(dot *DoTEffect, target *Actor) *DamageResult {
    // 1. Kiểm tra điều kiện
    if !cc.checkDoTConditions(dot.Conditions, target) {
        dot.IsActive = false
        return nil
    }
    
    // 2. Tính base damage
    randomFactor := rand.Float64()
    baseDamage := dot.BaseDamage.Min + (dot.BaseDamage.Max - dot.BaseDamage.Min) * randomFactor
    
    // 3. Áp dụng multipliers
    totalMultiplier := 1.0
    for _, multiplier := range dot.Multipliers {
        totalMultiplier *= multiplier
    }
    
    // 4. Tính defense
    targetDefense := cc.calculateDefenseForDoT(target, dot.DamageType)
    
    // 5. Áp dụng penetration
    totalPenetration := 0.0
    for _, penetration := range dot.Penetration {
        totalPenetration += penetration
    }
    
    // 6. Tính damage cuối cùng
    preDefenseDamage := baseDamage * totalMultiplier
    finalDamage := preDefenseDamage - targetDefense + totalPenetration
    
    if finalDamage < 0 {
        finalDamage = 0
    }
    
    return &DamageResult{
        FinalDamage: finalDamage,
        BaseDamage: baseDamage,
        Multipliers: totalMultiplier,
        TargetDefense: targetDefense,
        Penetration: totalPenetration,
        DamageTypes: []string{dot.DamageType},
        IsDoT: true,
        DoTID: dot.ID,
    }
}
```

## 📊 **Damage Events & Logging**

### **1. Damage Event Structure**

```go
// Damage Event
type DamageEvent struct {
    ID          string    `json:"id"`
    Timestamp   int64     `json:"timestamp"`
    AttackerID  string    `json:"attacker_id"`
    TargetID    string    `json:"target_id"`
    SkillID     string    `json:"skill_id"`
    Damage      float64   `json:"damage"`
    DamageTypes []string  `json:"damage_types"`
    IsCritical  bool      `json:"is_critical"`
    IsBlocked   bool      `json:"is_blocked"`
    IsHealing   bool      `json:"is_healing"`
    IsDrain     bool      `json:"is_drain"`
    IsReflect   bool      `json:"is_reflect"`
    IsAbsorb    bool      `json:"is_absorb"`
    IsDoT       bool      `json:"is_dot"`
    DoTID       string    `json:"dot_id"`
    Metadata    map[string]interface{} `json:"metadata"`
}
```

### **2. Damage Validation**

```go
// Damage validation
func ValidateDamage(damage *DamageResult) error {
    // 1. Kiểm tra damage không âm (trừ healing)
    if damage.FinalDamage < 0 && !damage.IsHealing {
        return fmt.Errorf("damage cannot be negative: %f", damage.FinalDamage)
    }
    
    // 2. Kiểm tra damage không quá cao
    if damage.FinalDamage > MaxAllowedDamage {
        return fmt.Errorf("damage exceeds maximum allowed: %f > %f", damage.FinalDamage, MaxAllowedDamage)
    }
    
    // 3. Kiểm tra critical multiplier
    if damage.CriticalMultiplier < 1.0 {
        return fmt.Errorf("critical multiplier cannot be less than 1.0: %f", damage.CriticalMultiplier)
    }
    
    // 4. Kiểm tra damage types
    for _, damageType := range damage.DamageTypes {
        if !IsValidDamageType(damageType) {
            return fmt.Errorf("invalid damage type: %s", damageType)
        }
    }
    
    return nil
}

// Damage limits
const (
    MaxAllowedDamage = 1000000.0  // 1M damage tối đa
    MinAllowedDamage = 0.0        // 0 damage tối thiểu
    MaxCriticalMultiplier = 10.0  // 10x critical tối đa
    MinCriticalMultiplier = 1.0   // 1x critical tối thiểu
)
```

## 🎯 **Damage Examples**

### **1. Physical Damage Example**

```go
// Sword Attack
swordAttack := &DamageInput{
    BaseDamage: DamageRange{Min: 500, Max: 800},
    PowerPoints: map[string]DamageRange{
        "physical_attack": {Min: 300, Max: 500},
    },
    Multipliers: map[string]float64{
        "weapon_multiplier": 1.2,
        "strength_multiplier": 1.1,
    },
    Additions: map[string]DamageRange{
        "weapon_bonus": {Min: 50, Max: 100},
    },
    CriticalChance: 0.15,
    CriticalMulti: DamageRange{Min: 2.0, Max: 2.5},
    Penetration: map[string]float64{
        "armor_penetration": 50,
    },
    DamageTypes: []string{"physical", "slashing"},
    Impacts: []DamageImpact{
        {
            TargetType: "mixed",
            ImpactDetails: map[string]DamageRange{
                "health": {Min: 1.0, Max: 1.0}, // 100% health
            },
            Multipliers: map[string]float64{
                "health_multiplier": 1.0,
            },
            Penetration: map[string]float64{
                "health_penetration": 30,
            },
        },
    },
}
```

### **2. Magical Damage Example**

```go
// Fireball Spell với Fire Mastery
fireballSpell := &DamageInput{
    BaseDamage: DamageRange{Min: 400, Max: 600},
    PowerPoints: map[string]DamageRange{
        "magical_attack": {Min: 200, Max: 400},
        "fire_attack": {Min: 300, Max: 500},
        "fire_mastery_attack": {Min: 0, Max: 0}, // Sẽ được tính từ mastery
    },
    Multipliers: map[string]float64{
        "spell_multiplier": 1.3,
        "fire_multiplier": 1.2,
        "intelligence_multiplier": 1.15,
        "fire_mastery_multiplier": 1.0, // Sẽ được tính từ mastery
    },
    Additions: map[string]DamageRange{
        "spell_bonus": {Min: 100, Max: 150},
    },
    CriticalChance: 0.20, // Sẽ được tăng bởi fire mastery
    CriticalMulti: DamageRange{Min: 2.2, Max: 2.8}, // Sẽ được tăng bởi fire mastery
    Penetration: map[string]float64{
        "magic_penetration": 60,
        "fire_penetration": 40,
    },
    DamageTypes: []string{"magical", "fire"},
    ElementType: "fire", // Để tính mastery bonus
    Impacts: []DamageImpact{
        {
            TargetType: "mixed",
            ImpactDetails: map[string]DamageRange{
                "health": {Min: 0.8, Max: 0.8}, // 80% health
                "mana": {Min: 0.2, Max: 0.2},   // 20% mana
            },
            Multipliers: map[string]float64{
                "health_multiplier": 1.0,
                "mana_multiplier": 1.5,
            },
            Penetration: map[string]float64{
                "health_penetration": 40,
                "mana_penetration": 60,
            },
        },
    },
}

// Mastery Integration Example
func calculateFireballDamageWithMastery(attacker *Actor, target *Actor, spell *DamageInput) *DamageResult {
    // Get fire mastery stats
    fireMastery := attacker.getFireMasteryStats()
    
    // Apply mastery bonuses
    spell.PowerPoints["fire_mastery_attack"] = DamageRange{
        Min: fireMastery.AttackPower * 0.1,
        Max: fireMastery.AttackPower * 0.1,
    }
    
    spell.Multipliers["fire_mastery_multiplier"] = 1.0 + (fireMastery.AttackPower * 0.001)
    spell.CriticalChance += fireMastery.CritRate * 0.01
    spell.CriticalMulti.Min += fireMastery.CritDamage * 0.01
    spell.CriticalMulti.Max += fireMastery.CritDamage * 0.01
    
    // Calculate damage
    return calculateFinalDamage(spell, target)
}
```

### **3. DoT Example**

```go
// Poison DoT
poisonDoT := &DoTEffect{
    ID: "poison_" + generateID(),
    Name: "Poison",
    DamageType: "poison",
    BaseDamage: DamageRange{Min: 50, Max: 100},
    TickInterval: 1000, // 1 giây
    TotalDuration: 10000, // 10 giây
    MaxTicks: 10,
    CurrentTicks: 0,
    LastTickTime: time.Now().UnixMilli(),
    IsActive: true,
    SourceID: "attacker_001",
    TargetID: "target_001",
    Multipliers: map[string]float64{
        "poison_multiplier": 1.0,
    },
    Penetration: map[string]float64{
        "poison_penetration": 20,
    },
    Conditions: []DoTCondition{
        {
            Type: "health_threshold",
            Operator: ">",
            Value: 0.1,
            Description: "Chỉ hoạt động khi target > 10% máu",
        },
    },
}
```

## 🚀 **Implementation Priority**

### **Phase 1: Core Damage System**
1. **Basic Damage Calculation**: Công thức cơ bản
2. **Damage Types**: Physical, Magical, Elemental
3. **Defense System**: Basic defense calculation
4. **Critical Hits**: Critical chance và multiplier

### **Phase 2: Elemental Mastery Integration**
1. **Mastery Stats Integration**: Tích hợp mastery stats vào damage calculation
2. **Mastery Bonus System**: Mastery difference bonus calculation
3. **Element-Core Integration**: Tích hợp với Element-Core system
4. **Actor Core Integration**: Tích hợp với Actor Core framework

### **Phase 3: Advanced Features**
1. **DoT System**: Damage over time với mastery integration
2. **Damage Interactions**: Elemental interactions với mastery
3. **Special Damage**: True, Healing, Drain
4. **Damage Events**: Logging và tracking với mastery data

### **Phase 4: Cultivation Integration**
1. **Cultivation Damage**: Qi, Spiritual, Dao với mastery
2. **Complex Interactions**: Multi-system damage với mastery
3. **Advanced DoT**: Cultivation-specific DoTs với mastery
4. **Performance Optimization**: Caching và optimization cho mastery calculations

## ❓ **Questions for Discussion**

1. **Damage Scaling**: Làm thế nào để scale damage theo level và mastery?
2. **Elemental Interactions**: Có nên có elemental rock-paper-scissors với mastery?
3. **DoT Stacking**: Có nên cho phép stack nhiều DoT cùng loại với mastery?
4. **Damage Reflection**: Làm thế nào để xử lý damage reflection với mastery?
5. **Performance**: Làm thế nào để optimize damage calculation cho nhiều targets với mastery?
6. **Mastery Balance**: Làm thế nào để balance mastery system để tránh overpowered?
7. **Mastery Decay**: Có nên có mastery decay trong combat để tạo động lực tu luyện?
8. **Mastery Interactions**: Có nên có mastery interactions giữa các elements?

## 🎯 **Next Steps**

1. **Implement Core Damage System**: Basic damage calculation
2. **Integrate Elemental Mastery**: Tích hợp mastery system vào damage calculation
3. **Create Damage Types**: Define all damage types với mastery support
4. **Implement DoT System**: Damage over time với mastery integration
5. **Create Damage Events**: Logging và tracking với mastery data
6. **Performance Testing**: Test với nhiều targets và mastery calculations

---

*Tài liệu này sẽ được cập nhật khi có thêm yêu cầu và feedback từ team.*

## 📐 Formal Variables & Order (Rust)

- BaseDamage (BD): từ input hoặc skill
- PowerPoints (PP): tổng hợp từ `Snapshot` attacker theo damage type
- TargetDefense (DEF): từ `Snapshot` target theo damage type
- FlatAdditions (FA): phụ trội cố định
- TotalMultiplier (M): nhân các multiplier theo thứ tự deterministic (tên-ASC)
- CriticalChance (CC), CriticalMulti (CM): RNG có seed
- AbsoluteDamage (AD): bỏ qua DEF

Thứ tự:
1) BD, PP, FA, DEF → tiền multiplier
2) Áp dụng M
3) Áp dụng crit (CM) nếu trúng CC
4) Cộng AD

## 🎲 RNG Determinism

- Dùng `StdRng` + seed (test/golden) để tái lập
- Seed nguồn: input vector hoặc combat tick id

## 🗂️ YAML Schemas (rút gọn)

`docs/combat-core/configs/damage_types.yaml`:
```yaml
version: 1
categories:
  - id: physical
  - id: magical
  - id: elemental
  - id: true
types:
  - id: sword
    category: physical
  - id: fireball
    category: elemental
```

`docs/combat-core/configs/interactions.yaml`:
```yaml
version: 1
pairs:
  - types: [fire, ice]
    modifier: 1.10
```

## 🧪 Golden Vectors (Rust harness)

- Input: attacker `Snapshot`, target `Snapshot`, damage input JSON, RNG seed
- Output: breakdown per impact (health, mana, shield), final values
- Harness: tương tự resource manager golden harness, đọc từ `docs/combat-core/golden_vectors/*`