# Resource System Design

## 📋 **Tổng Quan**

Resource System Design mô tả chi tiết cách thức thiết kế và implement hệ thống quản lý tài nguyên trong game, tuân thủ nguyên tắc "metadata-only aggregator" của Actor Core v3.

## 🎯 **Design Principles**

### **1. Actor Core Compliance**
- **Subsystem Pattern**: Resource Manager là một Subsystem, không phải Core component
- **Contribution-Based**: Sử dụng Contribution system để output resource values
- **No State Storage**: Không lưu trữ state, chỉ tính toán dựa trên Actor metadata
- **Snapshot Integration**: Resources được expose qua Snapshot của Actor Core

### **2. Flexible Resource Management**
- **Multi-System Support**: Hỗ trợ nhiều cultivation systems
- **Dynamic Resource Types**: Có thể thêm mới resource types runtime
- **Resource Dependencies**: Hỗ trợ resource dependencies và interactions
- **Resource Conflicts Resolution**: Xử lý conflicts giữa các systems

### **3. Performance First**
- **Caching Strategy**: Sử dụng multi-layer caching
- **Batch Processing**: Xử lý nhiều actors cùng lúc
- **Lazy Calculation**: Chỉ tính toán khi cần thiết
- **Memory Optimization**: Tối ưu memory usage

## 🏗️ **System Architecture**

### **1. Resource Manager Subsystem**

```go
// ResourceManagerSubsystem - Main subsystem interface
type ResourceManagerSubsystem struct {
    systemID    string
    priority    int64
    registry    *ResourceRegistry
    calculator  *ResourceCalculator
    aggregator  *ResourceAggregator
    eventBus    *ResourceEventBus
    cache       *ResourceCache
}

// Implements Subsystem interface
func (r *ResourceManagerSubsystem) SystemID() string {
    return r.systemID
}

func (r *ResourceManagerSubsystem) Priority() int64 {
    return r.priority
}

func (r *ResourceManagerSubsystem) Contribute(ctx context.Context, actor *Actor) (*SubsystemOutput, error) {
    // 1. Check cache first
    if cached, exists := r.cache.Get(actor.ID); exists {
        return cached, nil
    }
    
    // 2. Calculate resources
    resources := r.calculator.CalculateResources(actor)
    
    // 3. Create contributions
    contributions := r.createContributions(resources)
    
    // 4. Cache result
    r.cache.Set(actor.ID, contributions, "5m")
    
    return contributions, nil
}
```

### **2. Resource Registry**

```go
// ResourceRegistry - Manages resource definitions
type ResourceRegistry struct {
    resources  map[string]*ResourceDefinition
    categories map[string][]string
    dependencies map[string][]string
    mutex      sync.RWMutex
}

// ResourceDefinition - Defines a resource
type ResourceDefinition struct {
    ID            string                 `json:"id"`
    Name          string                 `json:"name"`
    Category      ResourceCategory       `json:"category"`
    Type          ResourceType           `json:"type"`
    BaseValue     float64                `json:"base_value"`
    MinValue      float64                `json:"min_value"`
    MaxValue      float64                `json:"max_value"`
    RegenRate     float64                `json:"regen_rate"`
    RegenType     RegenType              `json:"regen_type"`
    Dependencies  []string               `json:"dependencies"`
    Conflicts     []string               `json:"conflicts"`
    Tags          map[string]string      `json:"tags"`
    Validation    *ResourceValidation    `json:"validation"`
    Calculation   *ResourceCalculation   `json:"calculation"`
}

// ResourceCategory - Categories of resources
type ResourceCategory string
const (
    HealthCategory      ResourceCategory = "health"
    EnergyCategory      ResourceCategory = "energy"
    PhysicalCategory    ResourceCategory = "physical"
    CultivationCategory ResourceCategory = "cultivation"
    SpecialCategory     ResourceCategory = "special"
    SocialCategory      ResourceCategory = "social"
    EconomicCategory    ResourceCategory = "economic"
)

// ResourceType - Types of resources
type ResourceType string
const (
    CurrentType  ResourceType = "current"  // Giá trị hiện tại
    MaxType      ResourceType = "max"      // Giá trị tối đa
    RegenType    ResourceType = "regen"    // Tốc độ hồi
    PercentageType ResourceType = "percentage" // Tỷ lệ phần trăm
)
```

### **3. Resource Calculator**

```go
// ResourceCalculator - Calculates resource values
type ResourceCalculator struct {
    registry    *ResourceRegistry
    modifiers   *ResourceModifierRegistry
    cache       *ResourceCache
    mutex       sync.RWMutex
}

// CalculateResources - Main calculation method
func (rc *ResourceCalculator) CalculateResources(actor *Actor) map[string]float64 {
    resources := make(map[string]float64)
    
    // 1. Calculate base resources
    baseResources := rc.calculateBaseResources(actor)
    
    // 2. Apply system modifiers
    modifiedResources := rc.applySystemModifiers(baseResources, actor)
    
    // 3. Apply resource dependencies
    dependencyResources := rc.applyDependencies(modifiedResources, actor)
    
    // 4. Apply resource conflicts
    conflictResources := rc.resolveConflicts(dependencyResources, actor)
    
    // 5. Apply caps
    finalResources := rc.applyCaps(conflictResources, actor)
    
    return finalResources
}

// calculateBaseResources - Calculate base resource values
func (rc *ResourceCalculator) calculateBaseResources(actor *Actor) map[string]float64 {
    resources := make(map[string]float64)
    
    for _, resourceDef := range rc.registry.GetAllResources() {
        // Calculate based on actor metadata
        baseValue := rc.calculateBaseValue(resourceDef, actor)
        
        // Set current and max values
        resources[resourceDef.ID+"_current"] = baseValue
        resources[resourceDef.ID+"_max"] = baseValue
        
        // Calculate regeneration rate
        if resourceDef.RegenRate > 0 {
            resources[resourceDef.ID+"_regen"] = resourceDef.RegenRate
        }
        
        // Calculate percentage
        resources[resourceDef.ID+"_percentage"] = 100.0 // 100% initially
    }
    
    return resources
}
```

### **4. Resource Modifier System**

```go
// ResourceModifierRegistry - Manages resource modifiers
type ResourceModifierRegistry struct {
    modifiers map[string][]*ResourceModifier
    mutex     sync.RWMutex
}

// ResourceModifier - Modifies resource values
type ResourceModifier struct {
    ID          string                 `json:"id"`
    ResourceID  string                 `json:"resource_id"`
    Type        ModifierType           `json:"type"`
    Value       float64                `json:"value"`
    Condition   *ModifierCondition     `json:"condition"`
    Priority    int64                  `json:"priority"`
    System      string                 `json:"system"`
    Tags        map[string]string      `json:"tags"`
}

// ModifierType - Types of modifiers
type ModifierType string
const (
    AdditiveModifier    ModifierType = "additive"     // Cộng thêm
    MultiplicativeModifier ModifierType = "multiplicative" // Nhân với
    PercentageModifier  ModifierType = "percentage"   // Phần trăm
    OverrideModifier    ModifierType = "override"     // Ghi đè
    ConditionalModifier ModifierType = "conditional"  // Có điều kiện
)

// ModifierCondition - Condition for applying modifier
type ModifierCondition struct {
    Type        string                 `json:"type"`
    Operator    string                 `json:"operator"`
    Value       interface{}            `json:"value"`
    ResourceID  string                 `json:"resource_id,omitempty"`
    System      string                 `json:"system,omitempty"`
    Tags        map[string]string      `json:"tags,omitempty"`
}
```

### **5. Resource Aggregator**

```go
// ResourceAggregator - Aggregates resources from multiple systems
type ResourceAggregator struct {
    registry    *ResourceRegistry
    calculator  *ResourceCalculator
    cache       *ResourceCache
    mutex       sync.RWMutex
}

// AggregateResources - Aggregate resources from multiple sources
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

// ResourceSource - Resource value from a source
type ResourceSource struct {
    ResourceID  string                 `json:"resource_id"`
    Value       float64                `json:"value"`
    System      string                 `json:"system"`
    Priority    int64                  `json:"priority"`
    Weight      float64                `json:"weight"`
    Tags        map[string]string      `json:"tags"`
}

// aggregateResource - Aggregate a single resource
func (ra *ResourceAggregator) aggregateResource(resourceID string, sources []ResourceSource) float64 {
    if len(sources) == 0 {
        return 0.0
    }
    
    // Sort by priority (higher priority first)
    sort.Slice(sources, func(i, j int) bool {
        return sources[i].Priority > sources[j].Priority
    })
    
    // Group by modifier type
    additiveSources := make([]ResourceSource, 0)
    multiplicativeSources := make([]ResourceSource, 0)
    overrideSources := make([]ResourceSource, 0)
    
    for _, source := range sources {
        switch source.Tags["modifier_type"] {
        case "additive":
            additiveSources = append(additiveSources, source)
        case "multiplicative":
            multiplicativeSources = append(multiplicativeSources, source)
        case "override":
            overrideSources = append(overrideSources, source)
        }
    }
    
    // Start with base value
    result := 0.0
    
    // Apply additive modifiers
    for _, source := range additiveSources {
        result += source.Value * source.Weight
    }
    
    // Apply multiplicative modifiers
    for _, source := range multiplicativeSources {
        result *= (1.0 + source.Value * source.Weight)
    }
    
    // Apply override modifiers (highest priority wins)
    if len(overrideSources) > 0 {
        result = overrideSources[0].Value
    }
    
    return result
}
```

## 🔄 **Resource Regeneration System**

### **1. Regeneration Types**

```go
// RegenerationType - Types of regeneration
type RegenerationType string
const (
    ContinuousRegen  RegenerationType = "continuous"  // Hồi liên tục
    TickRegen        RegenerationType = "tick"        // Hồi theo tick
    ConditionalRegen RegenerationType = "conditional" // Hồi có điều kiện
    EventRegen       RegenerationType = "event"       // Hồi theo event
    NoRegen          RegenerationType = "none"        // Không hồi
)

// ResourceRegenerator - Handles resource regeneration
type ResourceRegenerator struct {
    registry    *ResourceRegistry
    calculator  *ResourceCalculator
    eventBus    *ResourceEventBus
    mutex       sync.RWMutex
}

// ProcessRegeneration - Process resource regeneration
func (rr *ResourceRegenerator) ProcessRegeneration(actor *Actor, deltaTime float64) map[string]float64 {
    regenValues := make(map[string]float64)
    
    for _, resourceDef := range rr.registry.GetAllResources() {
        if resourceDef.RegenRate <= 0 {
            continue
        }
        
        switch resourceDef.RegenType {
        case ContinuousRegen:
            regenValues[resourceDef.ID+"_regen"] = rr.processContinuousRegen(resourceDef, actor, deltaTime)
        case TickRegen:
            regenValues[resourceDef.ID+"_regen"] = rr.processTickRegen(resourceDef, actor)
        case ConditionalRegen:
            regenValues[resourceDef.ID+"_regen"] = rr.processConditionalRegen(resourceDef, actor)
        case EventRegen:
            regenValues[resourceDef.ID+"_regen"] = rr.processEventRegen(resourceDef, actor)
        }
    }
    
    return regenValues
}
```

### **2. Regeneration Examples**

```go
// Continuous regeneration (every frame/tick)
func (rr *ResourceRegenerator) processContinuousRegen(resourceDef *ResourceDefinition, actor *Actor, deltaTime float64) float64 {
    // Base regeneration rate
    baseRegen := resourceDef.RegenRate * deltaTime
    
    // Apply modifiers
    modifiedRegen := rr.applyRegenModifiers(resourceDef, baseRegen, actor)
    
    // Apply conditions
    if !rr.checkRegenConditions(resourceDef, actor) {
        return 0.0
    }
    
    return modifiedRegen
}

// Tick-based regeneration (every second)
func (rr *ResourceRegenerator) processTickRegen(resourceDef *ResourceDefinition, actor *Actor) float64 {
    // Base regeneration rate
    baseRegen := resourceDef.RegenRate
    
    // Apply modifiers
    modifiedRegen := rr.applyRegenModifiers(resourceDef, baseRegen, actor)
    
    // Apply conditions
    if !rr.checkRegenConditions(resourceDef, actor) {
        return 0.0
    }
    
    return modifiedRegen
}

// Conditional regeneration (based on conditions)
func (rr *ResourceRegenerator) processConditionalRegen(resourceDef *ResourceDefinition, actor *Actor) float64 {
    // Check conditions
    if !rr.checkRegenConditions(resourceDef, actor) {
        return 0.0
    }
    
    // Base regeneration rate
    baseRegen := resourceDef.RegenRate
    
    // Apply modifiers
    modifiedRegen := rr.applyRegenModifiers(resourceDef, baseRegen, actor)
    
    return modifiedRegen
}
```

## 🎯 **Resource Categories & Examples**

### **1. Health Resources**

```go
// Health resource definitions
var HealthResources = []*ResourceDefinition{
    {
        ID:        "hp",
        Name:      "Health Points",
        Category:  HealthCategory,
        Type:      CurrentType,
        BaseValue: 100.0,
        MinValue:  0.0,
        MaxValue:  10000.0,
        RegenRate: 1.0,
        RegenType: ContinuousRegen,
        Dependencies: []string{"vitality"},
        Tags: map[string]string{
            "combat_related": "true",
            "critical": "true",
        },
    },
    {
        ID:        "lifespan",
        Name:      "Lifespan",
        Category:  HealthCategory,
        Type:      MaxType,
        BaseValue: 100.0,
        MinValue:  1.0,
        MaxValue:  1000.0,
        RegenRate: 0.0,
        RegenType: NoRegen,
        Tags: map[string]string{
            "immutable": "true",
            "age_related": "true",
        },
    },
}
```

### **2. Energy Resources**

```go
// Energy resource definitions
var EnergyResources = []*ResourceDefinition{
    {
        ID:        "mana",
        Name:      "Mana",
        Category:  EnergyCategory,
        Type:      CurrentType,
        BaseValue: 50.0,
        MinValue:  0.0,
        MaxValue:  5000.0,
        RegenRate: 2.0,
        RegenType: ContinuousRegen,
        Dependencies: []string{"intelligence"},
        Tags: map[string]string{
            "magic_related": "true",
            "castable": "true",
        },
    },
    {
        ID:        "spiritual_energy",
        Name:      "Spiritual Energy",
        Category:  EnergyCategory,
        Type:      CurrentType,
        BaseValue: 25.0,
        MinValue:  0.0,
        MaxValue:  2500.0,
        RegenRate: 1.5,
        RegenType: ConditionalRegen,
        Dependencies: []string{"spirituality"},
        Tags: map[string]string{
            "cultivation_related": "true",
            "meditation_required": "true",
        },
    },
}
```

### **3. Cultivation Resources**

```go
// Cultivation resource definitions
var CultivationResources = []*ResourceDefinition{
    {
        ID:        "qi",
        Name:      "Qi Energy",
        Category:  CultivationCategory,
        Type:      CurrentType,
        BaseValue: 10.0,
        MinValue:  0.0,
        MaxValue:  1000.0,
        RegenRate: 0.5,
        RegenType: ConditionalRegen,
        Dependencies: []string{"cultivation_level"},
        Tags: map[string]string{
            "cultivation_related": "true",
            "realm_dependent": "true",
        },
    },
    {
        ID:        "dao_energy",
        Name:      "Dao Energy",
        Category:  CultivationCategory,
        Type:      CurrentType,
        BaseValue: 5.0,
        MinValue:  0.0,
        MaxValue:  500.0,
        RegenRate: 0.1,
        RegenType: EventRegen,
        Dependencies: []string{"dao_comprehension"},
        Tags: map[string]string{
            "cultivation_related": "true",
            "dao_related": "true",
            "rare": "true",
        },
    },
}
```

## 🔧 **Resource Calculation Examples**

### **1. HP Calculation**

```go
// HP calculation example
func (rc *ResourceCalculator) calculateHP(actor *Actor) map[string]float64 {
    // Base HP from lifespan
    baseHP := float64(actor.LifeSpan) * 10.0
    
    // Apply race modifier
    raceModifier := rc.getRaceModifier(actor.Race, "hp")
    modifiedHP := baseHP * raceModifier
    
    // Apply age modifier (older = more HP)
    ageModifier := 1.0 + (float64(actor.Age) / 100.0)
    finalHP := modifiedHP * ageModifier
    
    // Apply cultivation modifiers
    cultivationModifier := rc.getCultivationModifier(actor, "hp")
    finalHP *= cultivationModifier
    
    return map[string]float64{
        "hp_current":     finalHP,
        "hp_max":         finalHP,
        "hp_regen":       finalHP * 0.01, // 1% per second
        "hp_percentage":  100.0,          // 100% initially
    }
}
```

### **2. Mana Calculation**

```go
// Mana calculation example
func (rc *ResourceCalculator) calculateMana(actor *Actor) map[string]float64 {
    // Base mana from age
    baseMana := float64(actor.Age) * 5.0
    
    // Apply race modifier
    raceModifier := rc.getRaceModifier(actor.Race, "mana")
    modifiedMana := baseMana * raceModifier
    
    // Apply cultivation modifier
    cultivationModifier := rc.getCultivationModifier(actor, "mana")
    finalMana := modifiedMana * cultivationModifier
    
    // Apply intelligence modifier
    intelligenceModifier := rc.getIntelligenceModifier(actor)
    finalMana *= intelligenceModifier
    
    return map[string]float64{
        "mana_current":     finalMana,
        "mana_max":         finalMana,
        "mana_regen":       finalMana * 0.02, // 2% per second
        "mana_percentage":  100.0,            // 100% initially
    }
}
```

### **3. Cultivation Energy Calculation**

```go
// Cultivation energy calculation example
func (rc *ResourceCalculator) calculateCultivationEnergy(actor *Actor) map[string]float64 {
    // Base energy from lifespan
    baseEnergy := float64(actor.LifeSpan) * 2.0
    
    // Apply cultivation system modifier
    cultivationModifier := rc.getCultivationSystemModifier(actor)
    modifiedEnergy := baseEnergy * cultivationModifier
    
    // Apply realm modifier
    realmModifier := rc.getRealmModifier(actor)
    finalEnergy := modifiedEnergy * realmModifier
    
    // Apply dao comprehension modifier
    daoModifier := rc.getDaoComprehensionModifier(actor)
    finalEnergy *= daoModifier
    
    return map[string]float64{
        "cultivation_energy_current": finalEnergy,
        "cultivation_energy_max":     finalEnergy,
        "cultivation_energy_regen":   finalEnergy * 0.005, // 0.5% per second
        "cultivation_energy_percentage": 100.0,            // 100% initially
    }
}
```

## 📊 **Performance Optimization**

### **1. Caching Strategy**

```go
// ResourceCache - Multi-layer caching for resources
type ResourceCache struct {
    l1Cache    *sync.Map                    // L1: In-memory cache
    l2Cache    *ResourceL2Cache             // L2: Memory-mapped cache
    l3Cache    *ResourceL3Cache             // L3: Persistent cache
    mutex      sync.RWMutex
    ttl        time.Duration
}

// Get cached resource value
func (rc *ResourceCache) Get(actorID string, resourceID string) (float64, bool) {
    // Try L1 cache first
    if value, exists := rc.l1Cache.Load(actorID + ":" + resourceID); exists {
        return value.(float64), true
    }
    
    // Try L2 cache
    if value, exists := rc.l2Cache.Get(actorID, resourceID); exists {
        // Store in L1 cache
        rc.l1Cache.Store(actorID + ":" + resourceID, value)
        return value, true
    }
    
    // Try L3 cache
    if value, exists := rc.l3Cache.Get(actorID, resourceID); exists {
        // Store in L2 and L1 cache
        rc.l2Cache.Set(actorID, resourceID, value)
        rc.l1Cache.Store(actorID + ":" + resourceID, value)
        return value, true
    }
    
    return 0, false
}

// Set cached resource value
func (rc *ResourceCache) Set(actorID string, resourceID string, value float64) {
    // Store in all cache layers
    rc.l1Cache.Store(actorID + ":" + resourceID, value)
    rc.l2Cache.Set(actorID, resourceID, value)
    rc.l3Cache.Set(actorID, resourceID, value)
}
```

### **2. Batch Processing**

```go
// BatchProcessResources - Process multiple actors
func (rm *ResourceManagerSubsystem) BatchProcessResources(ctx context.Context, actors []*Actor) ([]*SubsystemOutput, error) {
    // Group actors by priority
    priorityGroups := make(map[int64][]*Actor)
    for _, actor := range actors {
        priority := rm.getActorPriority(actor)
        priorityGroups[priority] = append(priorityGroups[priority], actor)
    }
    
    // Process each priority group
    var wg sync.WaitGroup
    var mutex sync.Mutex
    outputs := make([]*SubsystemOutput, 0, len(actors))
    
    for priority, group := range priorityGroups {
        wg.Add(1)
        go func(p int64, actors []*Actor) {
            defer wg.Done()
            
            groupOutputs := rm.processActorGroup(ctx, actors)
            
            mutex.Lock()
            outputs = append(outputs, groupOutputs...)
            mutex.Unlock()
        }(priority, group)
    }
    
    wg.Wait()
    return outputs, nil
}
```

## 🚀 **Implementation Plan**

### **Phase 1: Core Resource System**
1. **Resource Registry**: Define resource types and categories
2. **Resource Calculator**: Basic resource calculation
3. **Resource Subsystem**: Integration with Actor Core
4. **Basic Resources**: HP, Mana, Stamina

### **Phase 2: Advanced Features**
1. **Resource Modifiers**: From cultivation systems
2. **Resource Dependencies**: Resource interactions
3. **Resource Conflicts**: Conflict resolution
4. **Resource Regeneration**: Continuous and tick-based

### **Phase 3: Cultivation Integration**
1. **Cultivation Resources**: Qi, Spiritual Energy, Dao Energy
2. **Realm-based Resources**: Different resources per realm
3. **Advanced Regeneration**: Conditional and complex regeneration
4. **Resource Events**: Change and depletion events

### **Phase 4: Performance & Optimization**
1. **Multi-layer Caching**: L1, L2, L3 cache
2. **Batch Processing**: Multiple actors processing
3. **Memory Optimization**: Memory pooling
4. **Performance Monitoring**: Metrics and analytics

## ❓ **Questions for Discussion**

1. **Resource Dependencies**: Làm thế nào để xử lý resource dependencies?
2. **Resource Conflicts**: Làm thế nào để resolve conflicts giữa các systems?
3. **Resource Persistence**: Làm thế nào để persist resource changes?
4. **Resource Events**: Làm thế nào để handle resource events?
5. **Performance**: Làm thế nào để optimize resource calculations?

## 🎯 **Next Steps**

1. **Implement Resource Registry**: Define all resource types
2. **Create Resource Calculator**: Basic calculation logic
3. **Integrate with Actor Core**: As a Subsystem
4. **Add Resource Regeneration**: Continuous and tick-based
5. **Performance Testing**: Test with multiple actors

---

*Tài liệu này sẽ được cập nhật khi có thêm yêu cầu và feedback từ team.*
