# Resource Manager System Overview

## 📋 **Tổng Quan**

Resource Manager là một **Subsystem** của Actor Core v3, được thiết kế để quản lý tất cả các loại tài nguyên (resources) trong game thông qua hệ thống Contributions và Snapshot. Hệ thống này tuân thủ nguyên tắc "metadata-only aggregator" của Actor Core, không lưu trữ state mà chỉ cung cấp logic tính toán resources.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Tuân Thủ Actor Core Architecture**
- **Subsystem Pattern**: Resource Manager là một Subsystem, không phải Core component
- **Contribution-Based**: Sử dụng Contribution system để output resource values
- **Snapshot Integration**: Resources được expose qua Snapshot của Actor Core
- **No State Storage**: Không lưu trữ state, chỉ tính toán dựa trên Actor metadata

### **2. Flexible & Extensible**
- Hỗ trợ nhiều loại resources khác nhau
- Dễ dàng thêm mới resource types
- Tương thích với các cultivation systems

### **3. Performance Optimized**
- Sử dụng caching của Actor Core
- Batch processing cho multiple actors
- Lazy calculation khi cần thiết

## 🏗️ **Kiến Trúc Resource Manager**

### **Core Components**

```
Resource Manager Subsystem
├── Resource Registry
│   ├── Resource Definitions
│   ├── Resource Categories
│   └── Resource Validation
├── Resource Calculator
│   ├── Base Resource Calculation
│   ├── Resource Modifiers
│   └── Resource Caps
├── Resource Aggregator
│   ├── Multi-System Aggregation
│   ├── Resource Conflicts Resolution
│   └── Resource Priority Handling
└── Resource Events
    ├── Resource Change Events
    ├── Resource Depletion Events
    └── Resource Regeneration Events
```

### **Integration với Actor Core**

```
Actor Core v3
├── Actor (metadata)
├── Subsystems[]
│   └── ResourceManagerSubsystem
├── Aggregator
│   └── Resource Contributions
└── Snapshot
    ├── Primary Stats
    │   ├── hp_current
    │   ├── hp_max
    │   ├── mana_current
    │   ├── mana_max
    │   └── ...
    └── Derived Stats
        ├── hp_percentage
        ├── mana_percentage
        └── ...
```

## 📊 **Resource Categories**

### **1. Health Resources**
```go
// Health Points
"hp_current"     // Sinh mệnh hiện tại
"hp_max"         // Sinh mệnh tối đa
"hp_regen"       // Tốc độ hồi máu
"hp_regen_rate"  // Tỷ lệ hồi máu

// Life Span
"lifespan_years"     // Tuổi thọ (năm)
"age_years"          // Tuổi hiện tại (năm)
"lifespan_remaining" // Tuổi thọ còn lại
```

### **2. Energy Resources**
```go
// Mana
"mana_current"     // Mana hiện tại
"mana_max"         // Mana tối đa
"mana_regen"       // Tốc độ hồi mana
"mana_regen_rate"  // Tỷ lệ hồi mana

// Spiritual Energy
"spiritual_energy_current"     // Linh lực hiện tại
"spiritual_energy_max"         // Linh lực tối đa
"spiritual_energy_regen"       // Tốc độ hồi linh lực

// Qi Energy
"qi_current"       // Khí hiện tại
"qi_max"           // Khí tối đa
"qi_regen"         // Tốc độ hồi khí
```

### **3. Physical Resources**
```go
// Stamina
"stamina_current"     // Thể lực hiện tại
"stamina_max"         // Thể lực tối đa
"stamina_regen"       // Tốc độ hồi thể lực

// Vitality
"vitality_current"    // Sinh lực hiện tại
"vitality_max"        // Sinh lực tối đa
"vitality_regen"      // Tốc độ hồi sinh lực
```

### **4. Cultivation Resources**
```go
// Cultivation-specific resources
"cultivation_energy_current"   // Tu luyện năng lượng
"cultivation_energy_max"       // Tu luyện năng lượng tối đa
"realm_energy_current"         // Cảnh giới năng lượng
"realm_energy_max"             // Cảnh giới năng lượng tối đa
"dao_energy_current"           // Đạo năng lượng
"dao_energy_max"               // Đạo năng lượng tối đa
```

## 🔧 **Resource Manager Subsystem**

### **1. Subsystem Interface**

```go
// ResourceManagerSubsystem implements Subsystem interface
type ResourceManagerSubsystem struct {
    systemID    string
    priority    int64
    registry    *ResourceRegistry
    calculator  *ResourceCalculator
    aggregator  *ResourceAggregator
}

// SystemID returns the unique identifier
func (r *ResourceManagerSubsystem) SystemID() string {
    return r.systemID
}

// Priority returns the processing priority
func (r *ResourceManagerSubsystem) Priority() int64 {
    return r.priority
}

// Contribute calculates and returns resource contributions
func (r *ResourceManagerSubsystem) Contribute(ctx context.Context, actor *Actor) (*SubsystemOutput, error) {
    // 1. Calculate base resources
    baseResources := r.calculator.CalculateBaseResources(actor)
    
    // 2. Apply modifiers from other systems
    modifiedResources := r.calculator.ApplyModifiers(baseResources, actor)
    
    // 3. Apply caps
    cappedResources := r.calculator.ApplyCaps(modifiedResources, actor)
    
    // 4. Create contributions
    contributions := r.createContributions(cappedResources)
    
    // 5. Return subsystem output
    return &SubsystemOutput{
        Primary: contributions.Primary,
        Derived: contributions.Derived,
        Caps:    contributions.Caps,
        Meta: SubsystemMeta{
            System:    r.systemID,
            Version:   1,
            Timestamp: time.Now(),
        },
    }, nil
}
```

### **2. Resource Registry**

```go
// ResourceRegistry manages resource definitions
type ResourceRegistry struct {
    resources map[string]*ResourceDefinition
    categories map[string][]string
    mutex     sync.RWMutex
}

// ResourceDefinition defines a resource
type ResourceDefinition struct {
    ID          string            `json:"id"`
    Name        string            `json:"name"`
    Category    string            `json:"category"`
    Type        ResourceType      `json:"type"`
    BaseValue   float64           `json:"base_value"`
    MinValue    float64           `json:"min_value"`
    MaxValue    float64           `json:"max_value"`
    RegenRate   float64           `json:"regen_rate"`
    RegenType   RegenType         `json:"regen_type"`
    Dependencies []string         `json:"dependencies"`
    Tags        map[string]string `json:"tags"`
}

// ResourceType defines the type of resource
type ResourceType string
const (
    HealthType    ResourceType = "health"
    EnergyType    ResourceType = "energy"
    PhysicalType  ResourceType = "physical"
    CultivationType ResourceType = "cultivation"
    SpecialType   ResourceType = "special"
)

// RegenType defines how resource regenerates
type RegenType string
const (
    ContinuousRegen RegenType = "continuous"  // Hồi liên tục
    TickRegen       RegenType = "tick"        // Hồi theo tick
    ConditionalRegen RegenType = "conditional" // Hồi có điều kiện
    NoRegen         RegenType = "none"        // Không hồi
)
```

### **3. Resource Calculator**

```go
// ResourceCalculator calculates resource values
type ResourceCalculator struct {
    registry *ResourceRegistry
    cache    *ResourceCache
}

// CalculateBaseResources calculates base resource values
func (rc *ResourceCalculator) CalculateBaseResources(actor *Actor) map[string]float64 {
    resources := make(map[string]float64)
    
    // Calculate based on actor metadata
    for _, resourceDef := range rc.registry.GetAllResources() {
        baseValue := rc.calculateBaseValue(resourceDef, actor)
        resources[resourceDef.ID+"_current"] = baseValue
        resources[resourceDef.ID+"_max"] = baseValue
    }
    
    return resources
}

// ApplyModifiers applies modifiers from other systems
func (rc *ResourceCalculator) ApplyModifiers(baseResources map[string]float64, actor *Actor) map[string]float64 {
    modified := make(map[string]float64)
    
    for resourceID, value := range baseResources {
        // Apply modifiers from cultivation systems
        modified[resourceID] = rc.applySystemModifiers(resourceID, value, actor)
    }
    
    return modified
}

// ApplyCaps applies resource caps
func (rc *ResourceCalculator) ApplyCaps(resources map[string]float64, actor *Actor) map[string]float64 {
    capped := make(map[string]float64)
    
    for resourceID, value := range resources {
        resourceDef := rc.registry.GetResource(resourceID)
        if resourceDef != nil {
            // Apply min/max caps
            if value < resourceDef.MinValue {
                value = resourceDef.MinValue
            }
            if value > resourceDef.MaxValue {
                value = resourceDef.MaxValue
            }
        }
        capped[resourceID] = value
    }
    
    return capped
}
```

### **4. Resource Aggregator**

```go
// ResourceAggregator aggregates resources from multiple systems
type ResourceAggregator struct {
    registry *ResourceRegistry
    cache    *ResourceCache
}

// AggregateResources aggregates resources from multiple sources
func (ra *ResourceAggregator) AggregateResources(sources []ResourceSource) map[string]float64 {
    aggregated := make(map[string]float64)
    
    // Group by resource ID
    resourceGroups := make(map[string][]ResourceSource)
    for _, source := range sources {
        resourceGroups[source.ResourceID] = append(resourceGroups[source.ResourceID], source)
    }
    
    // Aggregate each resource
    for resourceID, sources := range resourceGroups {
        aggregated[resourceID] = ra.aggregateResource(resourceID, sources)
    }
    
    return aggregated
}

// ResourceSource represents a resource value from a source
type ResourceSource struct {
    ResourceID string
    Value      float64
    System     string
    Priority   int64
    Weight     float64
}
```

## 📈 **Resource Calculation Examples**

### **1. Health Points Calculation**

```go
// HP calculation example
func (rc *ResourceCalculator) calculateHP(actor *Actor) map[string]float64 {
    // Base HP from actor metadata
    baseHP := float64(actor.LifeSpan) * 10.0 // 10 HP per year of lifespan
    
    // Apply race modifier
    raceModifier := rc.getRaceModifier(actor.Race)
    modifiedHP := baseHP * raceModifier
    
    // Apply age modifier (older = more HP)
    ageModifier := 1.0 + (float64(actor.Age) / 100.0)
    finalHP := modifiedHP * ageModifier
    
    return map[string]float64{
        "hp_current": finalHP,
        "hp_max":     finalHP,
        "hp_regen":   finalHP * 0.01, // 1% per second
    }
}
```

### **2. Mana Calculation**

```go
// Mana calculation example
func (rc *ResourceCalculator) calculateMana(actor *Actor) map[string]float64 {
    // Base mana from age (older = more mana)
    baseMana := float64(actor.Age) * 5.0
    
    // Apply race modifier
    raceModifier := rc.getRaceManaModifier(actor.Race)
    modifiedMana := baseMana * raceModifier
    
    // Apply cultivation modifier (if applicable)
    cultivationModifier := rc.getCultivationModifier(actor)
    finalMana := modifiedMana * cultivationModifier
    
    return map[string]float64{
        "mana_current": finalMana,
        "mana_max":     finalMana,
        "mana_regen":   finalMana * 0.02, // 2% per second
    }
}
```

### **3. Cultivation Energy Calculation**

```go
// Cultivation energy calculation example
func (rc *ResourceCalculator) calculateCultivationEnergy(actor *Actor) map[string]float64 {
    // Base energy from lifespan
    baseEnergy := float64(actor.LifeSpan) * 2.0
    
    // Apply cultivation system modifiers
    cultivationModifier := rc.getCultivationSystemModifier(actor)
    modifiedEnergy := baseEnergy * cultivationModifier
    
    // Apply realm modifier (if applicable)
    realmModifier := rc.getRealmModifier(actor)
    finalEnergy := modifiedEnergy * realmModifier
    
    return map[string]float64{
        "cultivation_energy_current": finalEnergy,
        "cultivation_energy_max":     finalEnergy,
        "cultivation_energy_regen":   finalEnergy * 0.005, // 0.5% per second
    }
}
```

## 🔄 **Resource Regeneration System**

### **1. Regeneration Types**

```go
// Continuous regeneration (every frame/tick)
func (rc *ResourceCalculator) processContinuousRegen(actor *Actor, deltaTime float64) map[string]float64 {
    regenValues := make(map[string]float64)
    
    for _, resourceDef := range rc.registry.GetResourcesByType(EnergyType) {
        if resourceDef.RegenType == ContinuousRegen {
            regenAmount := resourceDef.RegenRate * deltaTime
            regenValues[resourceDef.ID+"_regen"] = regenAmount
        }
    }
    
    return regenValues
}

// Tick-based regeneration (every second)
func (rc *ResourceCalculator) processTickRegen(actor *Actor) map[string]float64 {
    regenValues := make(map[string]float64)
    
    for _, resourceDef := range rc.registry.GetResourcesByType(HealthType) {
        if resourceDef.RegenType == TickRegen {
            regenAmount := resourceDef.RegenRate
            regenValues[resourceDef.ID+"_regen"] = regenAmount
        }
    }
    
    return regenValues
}
```

### **2. Conditional Regeneration**

```go
// Conditional regeneration (based on conditions)
func (rc *ResourceCalculator) processConditionalRegen(actor *Actor) map[string]float64 {
    regenValues := make(map[string]float64)
    
    for _, resourceDef := range rc.registry.GetResourcesByType(CultivationType) {
        if resourceDef.RegenType == ConditionalRegen {
            // Check conditions (e.g., not in combat, meditation, etc.)
            if rc.checkRegenConditions(resourceDef, actor) {
                regenAmount := resourceDef.RegenRate
                regenValues[resourceDef.ID+"_regen"] = regenAmount
            }
        }
    }
    
    return regenValues
}
```

## 🎯 **Integration với Combat Core**

### **1. Resource Consumption**

```go
// Combat Core consumes resources from Snapshot
func (cc *CombatCore) consumeResources(actor *Actor, skill *Skill) error {
    snapshot := cc.aggregator.Resolve(ctx, actor)
    
    // Get current resources
    hpCurrent := snapshot.Primary["hp_current"]
    manaCurrent := snapshot.Primary["mana_current"]
    
    // Check if enough resources
    if hpCurrent < skill.HPCost {
        return errors.New("insufficient HP")
    }
    if manaCurrent < skill.ManaCost {
        return errors.New("insufficient Mana")
    }
    
    // Consume resources (this would trigger resource update)
    return cc.consumeResource(actor, "hp_current", skill.HPCost)
}
```

### **2. Resource Damage**

```go
// Apply damage to resources
func (cc *CombatCore) applyResourceDamage(actor *Actor, damage *DamageResult) error {
    snapshot := cc.aggregator.Resolve(ctx, actor)
    
    // Get current HP
    hpCurrent := snapshot.Primary["hp_current"]
    
    // Apply damage
    newHP := hpCurrent - damage.FinalDamage
    if newHP < 0 {
        newHP = 0
    }
    
    // Update resource (this would trigger resource update)
    return cc.updateResource(actor, "hp_current", newHP)
}
```

## 📊 **Performance Considerations**

### **1. Caching Strategy**

```go
// Resource cache for performance
type ResourceCache struct {
    cache map[string]*CachedResource
    mutex sync.RWMutex
    ttl   time.Duration
}

// CachedResource represents a cached resource value
type CachedResource struct {
    Value     float64
    Timestamp time.Time
    TTL       time.Duration
}

// Get cached resource value
func (rc *ResourceCache) Get(resourceID string) (float64, bool) {
    rc.mutex.RLock()
    defer rc.mutex.RUnlock()
    
    cached, exists := rc.cache[resourceID]
    if !exists {
        return 0, false
    }
    
    // Check if expired
    if time.Since(cached.Timestamp) > cached.TTL {
        return 0, false
    }
    
    return cached.Value, true
}
```

### **2. Batch Processing**

```go
// Batch process multiple actors
func (rm *ResourceManagerSubsystem) ProcessBatch(ctx context.Context, actors []*Actor) ([]*SubsystemOutput, error) {
    outputs := make([]*SubsystemOutput, 0, len(actors))
    
    // Process in parallel
    var wg sync.WaitGroup
    var mutex sync.Mutex
    
    for _, actor := range actors {
        wg.Add(1)
        go func(a *Actor) {
            defer wg.Done()
            
            output, err := rm.Contribute(ctx, a)
            if err != nil {
                return
            }
            
            mutex.Lock()
            outputs = append(outputs, output)
            mutex.Unlock()
        }(actor)
    }
    
    wg.Wait()
    return outputs, nil
}
```

## 🚀 **Implementation Priority**

### **Phase 1: Core Resource System**
1. **Resource Registry**: Define resource types and categories
2. **Resource Calculator**: Basic resource calculation
3. **Resource Subsystem**: Integration with Actor Core
4. **Basic Resources**: HP, Mana, Stamina

### **Phase 2: Advanced Features**
1. **Resource Regeneration**: Continuous and tick-based
2. **Resource Modifiers**: From cultivation systems
3. **Resource Caps**: Min/max limits
4. **Resource Events**: Change and depletion events

### **Phase 3: Cultivation Integration**
1. **Cultivation Resources**: Qi, Spiritual Energy, Dao Energy
2. **Realm-based Resources**: Different resources per realm
3. **Advanced Regeneration**: Conditional and complex regeneration
4. **Resource Interactions**: Resource dependencies and conflicts

## ❓ **Questions for Discussion**

1. **Resource Dependencies**: Làm thế nào để xử lý resource dependencies?
2. **Resource Conflicts**: Làm thế nào để resolve conflicts giữa các systems?
3. **Resource Persistence**: Làm thế nào để persist resource changes?
4. **Resource Events**: Làm thế nào để handle resource events?
5. **Performance**: Làm thế nào để optimize resource calculations?

### ✅ Proposed Answers / 建议 / Gợi ý

- **Resource Dependencies** (依赖关系 / Phụ thuộc)
  - Model derived stats explicitly (e.g., `hp_percentage` via operator mode) and keep a DAG of dependencies; forbid cycles at load-time.
  - Use Combiner rules to separate pipeline dimensions (e.g., `hp_current`, `hp_max`, `*_regen`) from operator-mode ratios to avoid accidental reordering effects.

- **Resource Conflicts** (冲突处理 / Xung đột)
  - Deterministic resolution within a bucket: sort by `priority DESC, system ASC, value ASC`; `OVERRIDE` selects last after sort.
  - Across layers use `CapLayerRegistry` policy (recommend `INTERSECT`) to ensure conservative caps; document any prioritized-override exceptions.

- **Resource Persistence** (事件溯源 + 快照 / Ghi nhật ký sự kiện + snapshot)
  - Event-sourcing of deltas with idempotency key; periodic snapshots of `*_current` for fast warm start.
  - Write-ahead log (WAL) before apply; batch operations are transactional (all-or-nothing).
  - Tables: `resource_events(actor_id, ts, dimension, delta, cause, idem_key)` and `resource_snapshots(actor_id, version, map)`.

- **Resource Events** (事件与可观测性 / Sự kiện & Quan sát)
  - Emit `resource_change`, `resource_cap_change`, `resource_conflict` with `actor_id`, `dimension`, `delta`, `cause`, `correlation_id`.
  - Consumers: UI state, combat log, analytics, and cache invalidation; support sampling to reduce load.

- **Performance** (性能建议 / Hiệu năng)
  - Batch contributions; reuse Aggregator/registries; avoid per-request I/O.
  - Cache strategy: L1 lock-free in-memory, L2 memory-mapped, optional L3 persistent/Redis; warm critical actors.
  - Prefer operator-mode for simple aggregates; clamp early using precedence: EffectiveCaps → Combiner `clamp_default` → constants clamp ranges.

## 🛠️ Implementation Plan & Overview Checklist / 实施计划与清单 / Kế hoạch & Checklist

### Milestones
- Phase A: Configuration wiring (Combiner & Cap layers) ready and documented.
- Phase B: Subsystem skeleton (`ResourceManagerSubsystem`) created and registered.
- Phase C: Tick/decay/offline semantics implemented.
- Phase D: Golden vectors + property tests passing and stable.
- Phase E: Production readiness (readiness check, logging, metrics).

### Checklist (高层清单 / Danh sách kiểm)
- [ ] Configs published: `configs/combiner.resources.yaml`, `configs/cap_layers.resources.yaml` loaded via `ACTOR_CORE_CONFIG_DIR`.
- [ ] Subsystem file added: `crates/actor-core/src/subsystems/resource_manager.rs`.
- [ ] Module exposed: `crates/actor-core/src/subsystems/mod.rs` + `lib.rs` exports.
- [ ] Implements `interfaces::Subsystem` with `system_id`, `priority`, `contribute`.
- [ ] Emits contributions: `hp_max`, `hp_current`, `hp_regen`, `mana_*`, `stamina_*`, `shield_*`.
- [ ] Derived via operator-mode: `hp_percentage`.
- [ ] Tick pipeline: regen to current, shield decay, clamped by caps/combiner/constants.
- [ ] Offline catch-up bounded by `offline_regen_max_seconds`.
- [ ] Golden vectors pass in harness; order invariance tested.
- [ ] Proptests for clamp invariants, idempotency, monotonicity.
- [ ] Readiness probe validates registries and cache round-trip.

## 🎯 **Next Steps**

1. **Implement Resource Registry**: Define all resource types
2. **Create Resource Calculator**: Basic calculation logic
3. **Integrate with Actor Core**: As a Subsystem
4. **Add Resource Regeneration**: Continuous and tick-based
5. **Performance Testing**: Test with multiple actors

---

*Tài liệu này sẽ được cập nhật khi có thêm yêu cầu và feedback từ team.*
