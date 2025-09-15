# Damage System Design

## 📋 **Tổng Quan**

Damage System là trung tâm của Combat Core, xử lý tất cả các loại sát thương trong game. Hệ thống được thiết kế để hỗ trợ nhiều loại damage khác nhau, từ vật lý cơ bản đến các loại damage phức tạp trong cultivation systems.

**Integration với Element-Core**: Damage System sử dụng hybrid approach, trong đó Element-Core cung cấp element stats (bao gồm Omni stats) và Combat-Core thực hiện damage calculation dựa trên những stats này.

**⚠️ Critical Implementation Notes**: Xem [Element Core Implementation Notes](../element-core/06_Implementation_Notes.md) để biết các yêu cầu implementation quan trọng, bao gồm damage composition law, Omni additive-only rule, và status hit dependency.

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
│   ├── Element Interaction Calculator
│   └── Status Effect Calculator
├── Damage Calculation Engine
│   ├── Base Damage Calculation
│   ├── Power Points Calculation (from Element-Core)
│   ├── Defense Calculation (from Element-Core)
│   ├── Multiplier Application
│   └── Critical Hit Processing (from Element-Core)
├── Damage Types & Categories
│   ├── Physical Damage
│   ├── Magical Damage
│   ├── Elemental Damage (Element-Core)
│   ├── True Damage
│   └── Special Damage
├── DoT (Damage over Time) System
│   ├── DoT Manager
│   ├── Tick Processing
│   └── DoT Effects (Element-Core)
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

## 🔗 **Element-Core Integration**

### **1. Hybrid Architecture**

Combat-Core sử dụng hybrid approach để tích hợp với Element-Core:

```rust
// Element-Core provides stats
pub struct ElementStatsProvider {
    element_core: Arc<ElementCore>,
}

impl ElementStatsProvider {
    pub fn get_combat_stats(&self, attacker: &Actor, target: &Actor, element_type: &str) -> CombatElementStats {
        // Get Omni + Element stats from Element-Core
        let attacker_omni = self.element_core.get_omni_stats(attacker);
        let attacker_element = self.element_core.get_element_stats(attacker, element_type);
        let target_omni = self.element_core.get_omni_stats(target);
        let target_element = self.element_core.get_element_stats(target, element_type);
        
        CombatElementStats {
            // Power stats
            attacker_power: attacker_omni.power + attacker_element.power,
            target_defense: target_omni.defense + target_element.defense,
            
            // Critical stats
            attacker_crit_rate: attacker_omni.crit_rate + attacker_element.crit_rate,
            attacker_crit_damage: attacker_omni.crit_damage + attacker_element.crit_damage,
            target_resist_crit: target_omni.resist_crit + target_element.resist_crit,
            target_resist_crit_damage: target_omni.resist_crit_damage + target_element.resist_crit_damage,
            
            // Accuracy stats
            attacker_accuracy: attacker_omni.accuracy + attacker_element.accuracy,
            target_dodge: target_omni.dodge + target_element.dodge,
            
            // Status effect stats
            attacker_status_prob: attacker_omni.status_prob + attacker_element.status_prob,
            target_status_resist: target_omni.status_resist + target_element.status_resist,
            
            // Element interactions
            damage_multiplier: self.element_core.get_damage_multiplier(element_type, target.get_primary_element()),
        }
    }
}
```

### **2. Combat-Core Integration**

```rust
// Combat-Core uses Element-Core stats
impl CombatCore {
    pub fn calculate_damage(&self, action: &Action, attacker: &Actor, target: &Actor) -> DamageResult {
        // 1. Get element stats from Element-Core
        let element_stats = self.element_stats_provider.get_combat_stats(
            attacker, 
            target, 
            action.element_type
        );
        
        // 2. Create damage input with element stats
        let damage_input = DamageInput {
            base_damage: action.base_damage,
            power_points: vec![element_stats.attacker_power],
            target_defense: element_stats.target_defense,
            critical_chance: element_stats.attacker_crit_rate,
            critical_multiplier: element_stats.attacker_crit_damage,
            accuracy: element_stats.attacker_accuracy,
            target_dodge: element_stats.target_dodge,
            element_multiplier: element_stats.damage_multiplier,
            // ... other fields
        };
        
        // 3. Calculate damage using existing formula
        let mut damage_result = self.damage_calculator.calculate_final_damage(damage_input, target);
        
        // 4. Apply element-specific effects
        damage_result.final_damage *= element_stats.damage_multiplier;
        
        // 5. Apply status effects if applicable
        if self.should_apply_status_effects(action, element_stats) {
            let status_effects = self.element_core.calculate_status_effects(
                attacker, target, action.element_type
            );
            damage_result.status_effects = status_effects;
        }
        
        damage_result
    }
}
```

### **3. Benefits of Hybrid Approach**

#### **Separation of Concerns**
- **Element-Core**: Quản lý element stats, interactions, status effects
- **Combat-Core**: Quản lý combat mechanics, action processing, event handling

#### **Performance**
- **Element-Core**: Có thể cache element calculations
- **Combat-Core**: Có thể cache combat calculations
- **Minimal overhead**: Chỉ pass data, không duplicate calculations

#### **Flexibility**
- **Element-Core**: Có thể được sử dụng bởi systems khác (Shield, Item, Race)
- **Combat-Core**: Có thể sử dụng element stats từ nhiều sources
- **Easy testing**: Có thể test từng component riêng biệt

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

### **1. Công Thức Chính**

```
Final Damage = (BaseDamage + (PowerPoints - TargetDefense) + FlatAdditions) × TotalMultiplier × CriticalMultiplier + AbsoluteDamage
```

### **2. Công Thức Chi Tiết**

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
    
    // 10. Tính damage cuối cùng
    postMultiplierDamage := preMultiplierDamage * totalMultiplier * criticalMultiplier
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
        AbsoluteDamage: absoluteDamage,
        IsBlocked: false,
    }
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
// Fireball Spell
fireballSpell := &DamageInput{
    BaseDamage: DamageRange{Min: 400, Max: 600},
    PowerPoints: map[string]DamageRange{
        "magical_attack": {Min: 200, Max: 400},
        "fire_attack": {Min: 300, Max: 500},
    },
    Multipliers: map[string]float64{
        "spell_multiplier": 1.3,
        "fire_multiplier": 1.2,
        "intelligence_multiplier": 1.15,
    },
    Additions: map[string]DamageRange{
        "spell_bonus": {Min: 100, Max: 150},
    },
    CriticalChance: 0.20,
    CriticalMulti: DamageRange{Min: 2.2, Max: 2.8},
    Penetration: map[string]float64{
        "magic_penetration": 60,
        "fire_penetration": 40,
    },
    DamageTypes: []string{"magical", "fire"},
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

### **Phase 2: Advanced Features**
1. **DoT System**: Damage over time
2. **Damage Interactions**: Elemental interactions
3. **Special Damage**: True, Healing, Drain
4. **Damage Events**: Logging và tracking

### **Phase 3: Cultivation Integration**
1. **Cultivation Damage**: Qi, Spiritual, Dao
2. **Complex Interactions**: Multi-system damage
3. **Advanced DoT**: Cultivation-specific DoTs
4. **Performance Optimization**: Caching và optimization

## ❓ **Questions for Discussion**

1. **Damage Scaling**: Làm thế nào để scale damage theo level?
2. **Elemental Interactions**: Có nên có elemental rock-paper-scissors?
3. **DoT Stacking**: Có nên cho phép stack nhiều DoT cùng loại?
4. **Damage Reflection**: Làm thế nào để xử lý damage reflection?
5. **Performance**: Làm thế nào để optimize damage calculation cho nhiều targets?

## 🎯 **Next Steps**

1. **Implement Core Damage System**: Basic damage calculation
2. **Create Damage Types**: Define all damage types
3. **Implement DoT System**: Damage over time
4. **Create Damage Events**: Logging và tracking
5. **Performance Testing**: Test với nhiều targets

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