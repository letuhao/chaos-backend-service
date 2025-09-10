# Cultivation System Integration với Combat Core

## 📋 **Tổng Quan**

Combat Core được thiết kế như một **hub trung tâm** để xử lý logic combat cuối cùng, trong khi các hệ thống tu luyện (jindan-system, rpg-leveling-system, v.v.) sẽ tự implement logic riêng của chúng. Để tính damage và các chỉ số combat từ nhiều hệ thống khác nhau, chúng ta cần định nghĩa **endpoint interfaces** để các hệ thống tu luyện có thể cung cấp dữ liệu cho combat-core.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Combat Core là Hub, không phải Implementation**
- **Combat Core**: Chỉ xử lý logic combat cuối cùng
- **Cultivation Systems**: Tự implement logic riêng, cung cấp dữ liệu qua interfaces
- **Separation of Concerns**: Mỗi hệ thống có trách nhiệm riêng

### **2. Interface-Based Integration**
- **StatProvider Interface**: Cung cấp stats từ cultivation systems
- **DamageCalculator Interface**: Tính toán damage từ cultivation systems
- **ResourceManager Interface**: Quản lý tài nguyên từ cultivation systems

### **3. Flexible & Extensible**
- **Plugin Architecture**: Dễ dàng thêm mới cultivation systems
- **Backward Compatible**: Không phá vỡ existing systems
- **Performance Optimized**: Cache và optimize calculations

## 🏗️ **Kiến Trúc Tích Hợp**

### **Core Integration Flow**

```
┌─────────────────────────────────────────────────────────────┐
│                    Combat Core Hub                         │
├─────────────────────────────────────────────────────────────┤
│  Combat Engine     │  Damage Calculator  │  Status Manager  │
│  - Action System   │  - Power/Defense    │  - Buff/Debuff   │
│  - Event System    │  - Damage Types     │  - Interactions  │
│  - Multi-Target    │  - Calculations     │  - Stacking      │
├─────────────────────────────────────────────────────────────┤
│                    Interface Layer                          │
│  StatProvider      │  DamageCalculator   │  ResourceManager │
│  - GetStats()      │  - Calculate()      │  - Consume()     │
│  - GetPower()      │  - GetDefense()     │  - Regenerate()  │
│  - GetDefense()    │  - GetMultipliers() │  - GetCapacity() │
├─────────────────────────────────────────────────────────────┤
│                Cultivation Systems                          │
│  Jindan System     │  RPG System        │  Other Systems   │
│  - Implement       │  - Implement       │  - Implement     │
│  - StatProvider    │  - StatProvider    │  - StatProvider  │
│  - DamageCalc      │  - DamageCalc      │  - DamageCalc    │
│  - ResourceMgr     │  - ResourceMgr     │  - ResourceMgr   │
└─────────────────────────────────────────────────────────────┘
```

## 📊 **Derived Stats cho Combat**

### **Kế Thừa từ Actor Core v2**

Dựa trên review `actor-core-v2`, chúng ta có thể kế thừa các derived stats sau:

#### **Core Combat Stats (Từ actor-core-v2)**
```go
// Core Derived Stats
STAT_HP_MAX      = "hpMax"           // Sinh mệnh tối đa
STAT_STAMINA     = "stamina"         // Thể lực
STAT_SPEED       = "speed"           // Tốc độ
STAT_HASTE       = "haste"           // Tốc độ tấn công
STAT_CRIT_CHANCE = "critChance"      // Tỷ lệ chí mạng
STAT_CRIT_MULTI  = "critMulti"       // Hệ số chí mạng
STAT_MOVE_SPEED  = "moveSpeed"       // Tốc độ di chuyển
STAT_REGEN_HP    = "regenHP"         // Hồi máu

// Combat Stats
STAT_ACCURACY     = "accuracy"       // Độ chính xác
STAT_PENETRATION  = "penetration"    // Xuyên thủng
STAT_LETHALITY    = "lethality"      // Sát thương
STAT_BRUTALITY    = "brutality"      // Tàn bạo
STAT_ARMOR_CLASS  = "armorClass"     // Giáp
STAT_EVASION      = "evasion"        // Né tránh
STAT_BLOCK_CHANCE = "blockChance"    // Tỷ lệ chặn
STAT_PARRY_CHANCE = "parryChance"    // Tỷ lệ phản đòn
STAT_DODGE_CHANCE = "dodgeChance"    // Tỷ lệ né

// Energy Stats
STAT_ENERGY_EFFICIENCY = "energyEfficiency"  // Hiệu suất năng lượng
STAT_ENERGY_CAPACITY   = "energyCapacity"    // Dung lượng năng lượng
STAT_ENERGY_DRAIN      = "energyDrain"       // Hút năng lượng
STAT_RESOURCE_REGEN    = "resourceRegen"     // Hồi tài nguyên
STAT_RESOURCE_DECAY    = "resourceDecay"     // Suy giảm tài nguyên
```

#### **Mystical Stats (Phù hợp với Cultivation)**
```go
// Mystical Stats
STAT_MANA_EFFICIENCY  = "manaEfficiency"     // Hiệu suất mana
STAT_SPELL_POWER      = "spellPower"         // Sức mạnh pháp thuật
STAT_MYSTIC_RESONANCE = "mysticResonance"    // Cộng hưởng thần bí
STAT_REALITY_BEND     = "realityBend"        // Bẻ cong thực tại
STAT_TIME_SENSE       = "timeSense"          // Cảm giác thời gian
STAT_SPACE_SENSE      = "spaceSense"         // Cảm giác không gian
```

#### **Movement Stats (Quan trọng cho Combat)**
```go
// Movement Stats
STAT_JUMP_HEIGHT    = "jumpHeight"      // Độ cao nhảy
STAT_CLIMB_SPEED    = "climbSpeed"      // Tốc độ leo
STAT_SWIM_SPEED     = "swimSpeed"       // Tốc độ bơi
STAT_FLIGHT_SPEED   = "flightSpeed"     // Tốc độ bay
STAT_TELEPORT_RANGE = "teleportRange"   // Tầm dịch chuyển
STAT_STEALTH        = "stealth"         // Tàng hình
```

### **Combat-Specific Derived Stats (Mới)**

```go
// Combat-Specific Stats
STAT_POWER_POINTS     = "powerPoints"      // Điểm sức mạnh tấn công
STAT_DEFENSE_POINTS   = "defensePoints"    // Điểm phòng thủ
STAT_SHIELD_STRENGTH  = "shieldStrength"   // Sức mạnh shield
STAT_SHIELD_CAPACITY  = "shieldCapacity"   // Dung lượng shield
STAT_STATUS_RESIST    = "statusResist"     // Kháng trạng thái
STAT_STATUS_POWER     = "statusPower"      // Sức mạnh trạng thái
STAT_AOE_RADIUS       = "aoeRadius"        // Bán kính AOE
STAT_RANGE_BONUS      = "rangeBonus"       // Bonus tầm xa
STAT_COOLDOWN_REDUCE  = "cooldownReduce"   // Giảm thời gian hồi
STAT_RESOURCE_EFF     = "resourceEff"      // Hiệu suất tài nguyên
```

## 🔌 **Interface Definitions**

### **1. StatProvider Interface**

```go
// StatProvider defines the interface for providing stats from cultivation systems
type StatProvider interface {
    // GetCombatStats returns combat-relevant stats
    GetCombatStats() map[string]float64
    
    // GetPowerPoints returns power points for damage calculation
    GetPowerPoints() map[string]float64
    
    // GetDefensePoints returns defense points for damage reduction
    GetDefensePoints() map[string]float64
    
    // GetResourceStats returns resource-related stats
    GetResourceStats() map[string]float64
    
    // GetStatusStats returns status effect related stats
    GetStatusStats() map[string]float64
    
    // GetMovementStats returns movement related stats
    GetMovementStats() map[string]float64
    
    // GetSystemName returns the name of the cultivation system
    GetSystemName() string
    
    // GetSystemVersion returns the version of the cultivation system
    GetSystemVersion() string
    
    // IsActive checks if the system is active
    IsActive() bool
}
```

### **2. DamageCalculator Interface**

```go
// DamageCalculator defines the interface for calculating damage from cultivation systems
type DamageCalculator interface {
    // CalculatePowerPoints calculates power points for a specific damage type
    CalculatePowerPoints(damageType string, baseStats map[string]float64) (float64, error)
    
    // CalculateDefensePoints calculates defense points for a specific damage type
    CalculateDefensePoints(damageType string, baseStats map[string]float64) (float64, error)
    
    // CalculateDamageMultipliers calculates damage multipliers
    CalculateDamageMultipliers(damageType string, baseStats map[string]float64) (map[string]float64, error)
    
    // CalculateCriticalChance calculates critical hit chance
    CalculateCriticalChance(damageType string, baseStats map[string]float64) (float64, error)
    
    // CalculateCriticalMultiplier calculates critical hit multiplier
    CalculateCriticalMultiplier(damageType string, baseStats map[string]float64) (float64, error)
    
    // GetSupportedDamageTypes returns supported damage types
    GetSupportedDamageTypes() []string
    
    // GetSupportedDefenseTypes returns supported defense types
    GetSupportedDefenseTypes() []string
}
```

### **3. ResourceManager Interface**

```go
// ResourceManager defines the interface for managing resources from cultivation systems
type ResourceManager interface {
    // GetResourceCapacity returns resource capacity
    GetResourceCapacity(resourceType string) (float64, error)
    
    // GetResourceCurrent returns current resource amount
    GetResourceCurrent(resourceType string) (float64, error)
    
    // ConsumeResource consumes resource
    ConsumeResource(resourceType string, amount float64) error
    
    // RegenerateResource regenerates resource
    RegenerateResource(resourceType string, amount float64) error
    
    // GetResourceRegenRate returns resource regeneration rate
    GetResourceRegenRate(resourceType string) (float64, error)
    
    // GetSupportedResources returns supported resource types
    GetSupportedResources() []string
    
    // CanConsume checks if resource can be consumed
    CanConsume(resourceType string, amount float64) bool
}
```

### **4. StatusEffectProvider Interface**

```go
// StatusEffectProvider defines the interface for providing status effects from cultivation systems
type StatusEffectProvider interface {
    // GetActiveStatusEffects returns active status effects
    GetActiveStatusEffects() []StatusEffect
    
    // GetStatusEffectPower returns power of a status effect
    GetStatusEffectPower(effectType string) (float64, error)
    
    // GetStatusEffectResistance returns resistance to status effects
    GetStatusEffectResistance(effectType string) (float64, error)
    
    // GetStatusEffectDuration returns duration of a status effect
    GetStatusEffectDuration(effectType string) (int64, error)
    
    // GetSupportedStatusEffects returns supported status effect types
    GetSupportedStatusEffects() []string
}
```

## 🔄 **Integration Workflow**

### **1. System Registration**

```go
// Combat Core registers cultivation systems
type CombatCore struct {
    statProviders        map[string]StatProvider
    damageCalculators    map[string]DamageCalculator
    resourceManagers     map[string]ResourceManager
    statusEffectProviders map[string]StatusEffectProvider
    aggregationConfig    StatAggregationConfig
}

// Stat Aggregation Configuration
type StatAggregationConfig struct {
    Method      string  `json:"method"`      // "weighted_sum_decay"
    BaseValue   float64 `json:"base_value"`  // Giá trị cơ bản
    Weight      float64 `json:"weight"`      // Trọng số (0.8)
    Decay       float64 `json:"decay"`       // Hệ số suy giảm (0.3)
    MaxSystems  int     `json:"max_systems"` // Số hệ thống tối đa
}

// Default aggregation config
func NewDefaultAggregationConfig() StatAggregationConfig {
    return StatAggregationConfig{
        Method:     "weighted_sum_decay",
        BaseValue:  100,  // Giá trị cơ bản
        Weight:     0.8,  // Trọng số 80%
        Decay:      0.3,  // Suy giảm 30%
        MaxSystems: 5,    // Tối đa 5 hệ thống
    }
}

func (cc *CombatCore) RegisterCultivationSystem(systemName string, provider StatProvider) {
    cc.statProviders[systemName] = provider
}

func (cc *CombatCore) RegisterDamageCalculator(systemName string, calculator DamageCalculator) {
    cc.damageCalculators[systemName] = calculator
}
```

### **2. Stat Aggregation với Weighted Sum**

```go
// Combat Core aggregates stats from all systems using Weighted Sum
func (cc *CombatCore) GetAggregatedStats() map[string]float64 {
    aggregatedStats := make(map[string]float64)
    
    // Lấy tất cả stat names từ tất cả systems
    allStatNames := cc.getAllStatNames()
    
    for _, statName := range allStatNames {
        var statValues []float64
        
        // Thu thập values từ tất cả active systems
        for systemName, provider := range cc.statProviders {
            if !provider.IsActive() {
                continue
            }
            
            systemStats := provider.GetCombatStats()
            if value, exists := systemStats[statName]; exists {
                statValues = append(statValues, value)
            }
        }
        
        if len(statValues) > 0 {
            // Sử dụng Weighted Sum với Diminishing Returns
            aggregatedStats[statName] = cc.aggregateStatValues(statValues)
        }
    }
    
    return aggregatedStats
}

// Aggregate individual stat values using Weighted Sum
func (cc *CombatCore) aggregateStatValues(values []float64) float64 {
    config := cc.aggregationConfig
    
    if len(values) == 0 {
        return config.BaseValue
    }
    
    // Giới hạn số hệ thống
    if len(values) > config.MaxSystems {
        values = values[:config.MaxSystems]
    }
    
    sum := 0.0
    for _, value := range values {
        sum += value
    }
    
    // Diminishing factor: 1 - decay^number_of_systems
    diminishingFactor := 1 - math.Pow(config.Decay, float64(len(values)))
    
    return config.BaseValue + (sum * config.Weight * diminishingFactor)
}

// Lấy tất cả stat names từ tất cả systems
func (cc *CombatCore) getAllStatNames() []string {
    statNamesSet := make(map[string]bool)
    
    for _, provider := range cc.statProviders {
        if !provider.IsActive() {
            continue
        }
        
        systemStats := provider.GetCombatStats()
        for statName := range systemStats {
            statNamesSet[statName] = true
        }
    }
    
    var statNames []string
    for statName := range statNamesSet {
        statNames = append(statNames, statName)
    }
    
    return statNames
}
```

### **3. Damage Calculation**

```go
// Combat Core calculates damage using all systems
func (cc *CombatCore) CalculateDamage(attacker, target *Actor, damageType string) (*DamageResult, error) {
    var totalPower float64
    var totalDefense float64
    
    // Calculate power from all systems
    for systemName, calculator := range cc.damageCalculators {
        if !calculator.GetSupportedDamageTypes().Contains(damageType) {
            continue
        }
        
        attackerStats := cc.GetActorStats(attacker, systemName)
        power, err := calculator.CalculatePowerPoints(damageType, attackerStats)
        if err != nil {
            return nil, err
        }
        totalPower += power
    }
    
    // Calculate defense from all systems
    for systemName, calculator := range cc.damageCalculators {
        if !calculator.GetSupportedDefenseTypes().Contains(damageType) {
            continue
        }
        
        targetStats := cc.GetActorStats(target, systemName)
        defense, err := calculator.CalculateDefensePoints(damageType, targetStats)
        if err != nil {
            return nil, err
        }
        totalDefense += defense
    }
    
    // Calculate final damage
    finalDamage := totalPower - totalDefense
    if finalDamage < 0 {
        finalDamage = 0
    }
    
    return &DamageResult{
        Power:       totalPower,
        Defense:     totalDefense,
        FinalDamage: finalDamage,
        DamageType:  damageType,
    }, nil
}
```

## 🎯 **Cultivation System Implementation Examples**

### **1. Jindan System Integration**

```go
// Jindan System implements StatProvider
type JindanStatProvider struct {
    actor *Actor
    jindanSystem *JindanSystem
}

func (jsp *JindanStatProvider) GetCombatStats() map[string]float64 {
    stats := make(map[string]float64)
    
    // Convert jindan stats to combat stats
    stats[STAT_POWER_POINTS] = jsp.calculatePowerPoints()
    stats[STAT_DEFENSE_POINTS] = jsp.calculateDefensePoints()
    stats[STAT_SHIELD_STRENGTH] = jsp.calculateShieldStrength()
    
    return stats
}

func (jsp *JindanStatProvider) GetPowerPoints() map[string]float64 {
    powerPoints := make(map[string]float64)
    
    // Luyện khí system power points
    powerPoints["qi_attack"] = jsp.calculateQiAttack()
    powerPoints["spiritual_attack"] = jsp.calculateSpiritualAttack()
    powerPoints["technique_attack"] = jsp.calculateTechniqueAttack()
    
    return powerPoints
}
```

### **2. RPG System Integration**

```go
// RPG System implements StatProvider
type RPGStatProvider struct {
    actor *Actor
    rpgSystem *RPGSystem
}

func (rsp *RPGStatProvider) GetCombatStats() map[string]float64 {
    stats := make(map[string]float64)
    
    // Convert RPG stats to combat stats
    stats[STAT_POWER_POINTS] = rsp.calculatePowerPoints()
    stats[STAT_DEFENSE_POINTS] = rsp.calculateDefensePoints()
    stats[STAT_CRIT_CHANCE] = rsp.calculateCritChance()
    
    return stats
}

func (rsp *RPGStatProvider) GetPowerPoints() map[string]float64 {
    powerPoints := make(map[string]float64)
    
    // RPG system power points
    powerPoints["physical_attack"] = rsp.calculatePhysicalAttack()
    powerPoints["magical_attack"] = rsp.calculateMagicalAttack()
    powerPoints["skill_attack"] = rsp.calculateSkillAttack()
    
    return powerPoints
}
```

## 📊 **Performance Considerations**

### **1. Caching Strategy**

```go
// Cache combat stats to avoid recalculation
type CombatStatsCache struct {
    cache map[string]*CachedStats
    mutex sync.RWMutex
    ttl   time.Duration
}

type CachedStats struct {
    Stats     map[string]float64
    Timestamp int64
    ExpiresAt int64
}
```

### **2. Batch Processing**

```go
// Process multiple actors in batch
func (cc *CombatCore) ProcessBatch(actors []*Actor) error {
    // Batch get stats from all systems
    statBatches := make(map[string][]*Actor)
    for _, actor := range actors {
        for systemName := range cc.statProviders {
            statBatches[systemName] = append(statBatches[systemName], actor)
        }
    }
    
    // Process each system batch
    for systemName, batch := range statBatches {
        provider := cc.statProviders[systemName]
        // Process batch...
    }
    
    return nil
}
```

## 🧪 **Testing Strategy**

### **1. Unit Tests**
- **Interface Tests**: Test từng interface implementation
- **Integration Tests**: Test tích hợp giữa combat-core và cultivation systems
- **Performance Tests**: Test performance với nhiều systems

### **2. Mock Systems**
- **Mock StatProvider**: Test combat-core với mock data
- **Mock DamageCalculator**: Test damage calculation logic
- **Mock ResourceManager**: Test resource management

## 📊 **Stat Aggregation Strategy**

### **Weighted Sum với Diminishing Returns**

Sau khi thảo luận, chúng ta đã chọn **Weighted Sum với Diminishing Returns** làm phương pháp aggregation chính vì:

- ✅ **Khuyến khích đa hệ thống**: Tu luyện nhiều hệ thống = mạnh hơn
- ✅ **Diminishing Returns**: Không quá mạnh, vẫn có giới hạn
- ✅ **Realistic**: Phản ánh đúng logic tu luyện
- ✅ **Configurable**: Dễ dàng điều chỉnh parameters
- ✅ **Balanced**: Cân bằng giữa power và balance

### **Công Thức Aggregation**

```go
type StatAggregationConfig struct {
    Method      string  `json:"method"`      // "weighted_sum_decay"
    BaseValue   float64 `json:"base_value"`  // Giá trị cơ bản
    Weight      float64 `json:"weight"`      // Trọng số (0.8)
    Decay       float64 `json:"decay"`       // Hệ số suy giảm (0.3)
    MaxSystems  int     `json:"max_systems"` // Số hệ thống tối đa
}

func (cc *CombatCore) AggregateStats(stats []float64, config StatAggregationConfig) float64 {
    if len(stats) == 0 {
        return config.BaseValue
    }
    
    // Giới hạn số hệ thống
    if len(stats) > config.MaxSystems {
        stats = stats[:config.MaxSystems]
    }
    
    sum := 0.0
    for _, stat := range stats {
        sum += stat
    }
    
    // Diminishing factor: 1 - decay^number_of_systems
    diminishingFactor := 1 - math.Pow(config.Decay, float64(len(stats)))
    
    return config.BaseValue + (sum * config.Weight * diminishingFactor)
}
```

### **Ví Dụ Aggregation**

```go
// Config mặc định
config := StatAggregationConfig{
    Method:     "weighted_sum_decay",
    BaseValue:  100,  // Giá trị cơ bản
    Weight:     0.8,  // Trọng số 80%
    Decay:      0.3,  // Suy giảm 30%
    MaxSystems: 5,    // Tối đa 5 hệ thống
}

// Kết quả với các scenarios khác nhau:
// 1 system: 100 + (1000 * 0.8 * 0.7) = 100 + 560 = 660
// 2 systems: 100 + (1800 * 0.8 * 0.91) = 100 + 1310.4 = 1410.4
// 3 systems: 100 + (2400 * 0.8 * 0.973) = 100 + 1868.16 = 1968.16
// 4 systems: 100 + (3000 * 0.8 * 0.9919) = 100 + 2380.56 = 2480.56
// 5 systems: 100 + (3600 * 0.8 * 0.9973) = 100 + 2870.24 = 2970.24
```

### **So Sánh với Các Phương Pháp Khác**

| Method | 1 System | 2 Systems | 3 Systems | **Khuyến khích đa hệ thống?** |
|--------|----------|-----------|-----------|-------------------------------|
| **Simple Sum** | 1000 | 1800 | 2400 | ✅ Mạnh hơn (nhưng quá mạnh) |
| **Simple Average** | 1000 | 900 | 800 | ❌ Yếu hơn |
| **Weighted Sum** | 660 | 1410 | 1968 | ✅ Mạnh hơn (cân bằng) |
| **Square Root** | 1000 | 1281 | 1414 | ✅ Mạnh hơn (ít hơn) |
| **Logarithmic** | 1000 | 1099 | 1220 | ✅ Mạnh hơn (ít hơn) |

## 🔧 Rust Alignment (Actor Core v3)

- StatProvider/DamageCalculator/ResourceManager ở đây là khái niệm. Trong Rust:
  - Hệ thống tu luyện nên phát Contribution vào Actor Core (Subsystem hoặc upstream pipeline)
  - Combat Core đọc `Snapshot` đã tổng hợp để tính damage

Trait phác thảo (Rust, minh họa):
```rust
pub trait CombatStatProvider {
    fn get_combat_stats(&self, snapshot: &actor_core::types::Snapshot) -> std::collections::HashMap<String, f64>;
}

pub trait DamageCalculator {
    fn calculate(&self, attacker: &actor_core::types::Snapshot, target: &actor_core::types::Snapshot, damage_type: &str) -> f64;
}
```

Cấu hình aggregation (ví dụ):
```yaml
# docs/combat-core/configs/aggregation.yaml
method: weighted_sum_decay
base_value: 100.0
weight: 0.8
decay: 0.3
max_systems: 5
```

Env: `COMBAT_CORE_CONFIG_DIR` → nạp `aggregation.yaml`.

## ❓ **Questions for Discussion**

1. **Damage Calculation**: Có nên có damage calculation chung hay mỗi system tự tính?
2. **Resource Management**: Làm thế nào để quản lý resources từ nhiều systems?
3. **Performance**: Làm thế nào để optimize performance với nhiều systems?
4. **Backward Compatibility**: Làm thế nào để đảm bảo backward compatibility?
5. **Aggregation Parameters**: Làm thế nào để điều chỉnh Weight, Decay, BaseValue cho phù hợp?

## 🎯 **Ví Dụ Cụ Thể với 5 Hệ Thống Tu Luyện**

### **Scenario: Actor có 5 cultivation systems**

```go
// Tạo actor với 5 systems
actor := &Actor{
    ID: "player_001",
    Systems: []PowerPointProvider{
        &JindanSystem{
            Realm: "Kim Đan", Stage: "Hậu Kỳ", 
            PowerLevel: 1000, LinhLuc: 5000,
        },
        &RPGSystem{
            Level: 50, Class: "Warrior", 
            Experience: 100000, SkillPoints: 25,
        },
        &MagicSystem{
            ManaLevel: 25, SpellLevel: 80, 
            ManaPool: 3000, SpellPower: 2500,
        },
        &BodyCultivationSystem{
            BodyLevel: 15, MusclePower: 4000, 
            BoneStrength: 3500, BloodPower: 3000,
        },
        &SuccubusSystem{
            LustLevel: 10, Seduction: 2000, 
            Charm: 1800, Temptation: 1600,
        },
    },
}

// Config aggregation
config := StatAggregationConfig{
    Method:     "weighted_sum_decay",
    BaseValue:  100,
    Weight:     0.8,
    Decay:      0.3,
    MaxSystems: 5,
}

// Tính aggregation cho từng damage type
powerPoints := map[string]float64{
    "qi_attack":        950,   // (1000 + 900) / 2 = 950
    "physical_attack":  1050,  // (900 + 1200) / 2 = 1050
    "fire_attack":      800,   // Magic only
    "fist_attack":      1200,  // Body only
    "seduction_attack": 600,   // Succubus only
}

// Kết quả aggregation với Weighted Sum:
// qi_attack: 100 + (950 * 0.8 * 0.7) = 100 + 532 = 632
// physical_attack: 100 + (1050 * 0.8 * 0.7) = 100 + 588 = 688
// fire_attack: 100 + (800 * 0.8 * 0.7) = 100 + 448 = 548
// fist_attack: 100 + (1200 * 0.8 * 0.7) = 100 + 672 = 772
// seduction_attack: 100 + (600 * 0.8 * 0.7) = 100 + 336 = 436
```

### **So Sánh Kết Quả**

| Damage Type | Single System | Multiple Systems | **Aggregated Result** | **Improvement** |
|-------------|---------------|------------------|----------------------|-----------------|
| **qi_attack** | 1000 (Jindan) | 950 (Jindan + Body) | 632 | -37% (nhưng có 2 systems) |
| **physical_attack** | 900 (RPG) | 1050 (RPG + Body) | 688 | -24% (nhưng có 2 systems) |
| **fire_attack** | 800 (Magic) | 800 (Magic only) | 548 | -32% (single system) |
| **fist_attack** | 1200 (Body) | 1200 (Body only) | 772 | -36% (single system) |
| **seduction_attack** | 600 (Succubus) | 600 (Succubus only) | 436 | -27% (single system) |

### **Phân Tích Kết Quả**

- ✅ **Khuyến khích đa hệ thống**: Có 2 systems cho qi_attack và physical_attack
- ✅ **Diminishing Returns**: Không quá mạnh, vẫn có giới hạn
- ✅ **Balanced**: Kết quả hợp lý, không quá cao
- ✅ **Realistic**: Phản ánh đúng logic tu luyện

## 🎯 **Next Steps**

1. **Define Interfaces**: Hoàn thiện interface definitions
2. **Implement Mock Systems**: Tạo mock implementations để test
3. **Create Integration Tests**: Test tích hợp giữa combat-core và cultivation systems
4. **Performance Optimization**: Optimize performance cho multiple systems
5. **Documentation**: Tạo documentation chi tiết cho từng interface
6. **Aggregation Tuning**: Điều chỉnh Weight, Decay, BaseValue cho phù hợp với game balance

---

*Tài liệu này sẽ được cập nhật khi có thêm yêu cầu và feedback từ team.*
