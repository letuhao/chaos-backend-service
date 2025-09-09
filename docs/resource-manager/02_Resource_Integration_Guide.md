# Resource Integration Guide

## 📋 **Tổng Quan**

Resource Integration Guide hướng dẫn cách tích hợp Resource Manager Subsystem với Actor Core v3 và các hệ thống khác trong game, đảm bảo tuân thủ nguyên tắc "metadata-only aggregator".

## 🎯 **Integration Principles**

### **1. Actor Core Compliance**
- **Subsystem Pattern**: Resource Manager là một Subsystem, không phải Core component
- **Contribution-Based**: Sử dụng Contribution system để output resource values
- **No State Storage**: Không lưu trữ state, chỉ tính toán dựa trên Actor metadata
- **Snapshot Integration**: Resources được expose qua Snapshot của Actor Core

### **2. System Integration**
- **Cultivation Systems**: Tích hợp với các hệ thống tu luyện
- **Combat Systems**: Tích hợp với combat core
- **Item Systems**: Tích hợp với item management
- **Event Systems**: Tích hợp với event handling

## 🏗️ **Integration Architecture**

### **1. Resource Manager Subsystem Integration**

```go
// ResourceManagerSubsystem - Main integration point
type ResourceManagerSubsystem struct {
    systemID    string
    priority    int64
    registry    *ResourceRegistry
    calculator  *ResourceCalculator
    aggregator  *ResourceAggregator
    eventBus    *ResourceEventBus
    cache       *ResourceCache
    mutex       sync.RWMutex
}

// NewResourceManagerSubsystem - Constructor
func NewResourceManagerSubsystem(config *ResourceConfig) *ResourceManagerSubsystem {
    return &ResourceManagerSubsystem{
        systemID:    "resource_manager",
        priority:    100, // High priority
        registry:    NewResourceRegistry(config.Registry),
        calculator:  NewResourceCalculator(config.Calculator),
        aggregator:  NewResourceAggregator(config.Aggregator),
        eventBus:    NewResourceEventBus(config.EventBus),
        cache:       NewResourceCache(config.Cache),
    }
}

// Contribute - Main contribution method
func (r *ResourceManagerSubsystem) Contribute(ctx context.Context, actor *Actor) (*SubsystemOutput, error) {
    r.mutex.RLock()
    defer r.mutex.RUnlock()
    
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

### **2. Resource Configuration**

```go
// ResourceConfig - Configuration for Resource Manager
type ResourceConfig struct {
    Registry    *RegistryConfig    `json:"registry"`
    Calculator  *CalculatorConfig  `json:"calculator"`
    Aggregator  *AggregatorConfig  `json:"aggregator"`
    EventBus    *EventBusConfig    `json:"event_bus"`
    Cache       *CacheConfig       `json:"cache"`
}

// RegistryConfig - Registry configuration
type RegistryConfig struct {
    Resources      []*ResourceDefinition `json:"resources"`
    Categories     map[string][]string   `json:"categories"`
    Dependencies   map[string][]string   `json:"dependencies"`
    Conflicts      map[string][]string   `json:"conflicts"`
    Validation     *ValidationConfig     `json:"validation"`
}

// CalculatorConfig - Calculator configuration
type CalculatorConfig struct {
    BaseCalculation *BaseCalculationConfig `json:"base_calculation"`
    Modifiers       *ModifierConfig        `json:"modifiers"`
    Dependencies    *DependencyConfig      `json:"dependencies"`
    Conflicts       *ConflictConfig        `json:"conflicts"`
    Caps            *CapsConfig            `json:"caps"`
}

// AggregatorConfig - Aggregator configuration
type AggregatorConfig struct {
    Method          string                 `json:"method"`
    Priority        *PriorityConfig        `json:"priority"`
    Weight          *WeightConfig          `json:"weight"`
    Conflicts       *ConflictResolutionConfig `json:"conflicts"`
}

// EventBusConfig - Event bus configuration
type EventBusConfig struct {
    Enabled         bool                   `json:"enabled"`
    Channels        []string               `json:"channels"`
    BufferSize      int                    `json:"buffer_size"`
    Workers         int                    `json:"workers"`
}

// CacheConfig - Cache configuration
type CacheConfig struct {
    L1Enabled       bool                   `json:"l1_enabled"`
    L2Enabled       bool                   `json:"l2_enabled"`
    L3Enabled       bool                   `json:"l3_enabled"`
    TTL             time.Duration          `json:"ttl"`
    MaxSize         int64                  `json:"max_size"`
    CleanupInterval time.Duration          `json:"cleanup_interval"`
}
```

## 🔗 **Integration với Actor Core**

### **1. Subsystem Registration**

```go
// Register Resource Manager Subsystem
func RegisterResourceManagerSubsystem(pluginRegistry interfaces.PluginRegistry, config *ResourceConfig) error {
    // Create Resource Manager Subsystem
    resourceSubsystem := NewResourceManagerSubsystem(config)
    
    // Register with plugin registry
    err := pluginRegistry.Register(resourceSubsystem)
    if err != nil {
        return fmt.Errorf("failed to register resource manager subsystem: %w", err)
    }
    
    return nil
}

// Example usage
func main() {
    // Create plugin registry
    pluginRegistry := registry.NewPluginRegistry()
    
    // Create resource config
    resourceConfig := &ResourceConfig{
        Registry: &RegistryConfig{
            Resources: []*ResourceDefinition{
                // HP resource
                {
                    ID:        "hp",
                    Name:      "Health Points",
                    Category:  "health",
                    Type:      "current",
                    BaseValue: 100.0,
                    MinValue:  0.0,
                    MaxValue:  10000.0,
                    RegenRate: 1.0,
                    RegenType: "continuous",
                },
                // Mana resource
                {
                    ID:        "mana",
                    Name:      "Mana",
                    Category:  "energy",
                    Type:      "current",
                    BaseValue: 50.0,
                    MinValue:  0.0,
                    MaxValue:  5000.0,
                    RegenRate: 2.0,
                    RegenType: "continuous",
                },
            },
        },
        Calculator: &CalculatorConfig{
            BaseCalculation: &BaseCalculationConfig{
                Method: "actor_metadata",
            },
        },
        Cache: &CacheConfig{
            L1Enabled: true,
            L2Enabled: true,
            L3Enabled: false,
            TTL:       time.Minute * 5,
            MaxSize:   10000,
        },
    }
    
    // Register Resource Manager Subsystem
    err := RegisterResourceManagerSubsystem(pluginRegistry, resourceConfig)
    if err != nil {
        log.Fatal(err)
    }
}
```

### **2. Resource Access từ Snapshot**

```go
// Access resources from Actor Core Snapshot
func (cc *CombatCore) getActorResources(actor *Actor) (*ActorResources, error) {
    // Get snapshot from Actor Core
    snapshot, err := cc.aggregator.Resolve(ctx, actor)
    if err != nil {
        return nil, fmt.Errorf("failed to resolve actor snapshot: %w", err)
    }
    
    // Extract resources from snapshot
    resources := &ActorResources{
        HP: &Resource{
            Current:    snapshot.Primary["hp_current"],
            Max:        snapshot.Primary["hp_max"],
            Regen:      snapshot.Primary["hp_regen"],
            Percentage: snapshot.Derived["hp_percentage"],
        },
        Mana: &Resource{
            Current:    snapshot.Primary["mana_current"],
            Max:        snapshot.Primary["mana_max"],
            Regen:      snapshot.Primary["mana_regen"],
            Percentage: snapshot.Derived["mana_percentage"],
        },
        Stamina: &Resource{
            Current:    snapshot.Primary["stamina_current"],
            Max:        snapshot.Primary["stamina_max"],
            Regen:      snapshot.Primary["stamina_regen"],
            Percentage: snapshot.Derived["stamina_percentage"],
        },
    }
    
    return resources, nil
}

// ActorResources - Resource container
type ActorResources struct {
    HP      *Resource `json:"hp"`
    Mana    *Resource `json:"mana"`
    Stamina *Resource `json:"stamina"`
    // ... other resources
}

// Resource - Individual resource
type Resource struct {
    Current    float64 `json:"current"`
    Max        float64 `json:"max"`
    Regen      float64 `json:"regen"`
    Percentage float64 `json:"percentage"`
}
```

## 🔗 **Integration với Cultivation Systems**

### **1. Cultivation System Modifiers**

```go
// CultivationSystemModifier - Modifier from cultivation system
type CultivationSystemModifier struct {
    SystemID    string                 `json:"system_id"`
    ResourceID  string                 `json:"resource_id"`
    Modifier    *ResourceModifier      `json:"modifier"`
    Condition   *ModifierCondition     `json:"condition"`
    Priority    int64                  `json:"priority"`
}

// Register cultivation system modifier
func (rm *ResourceManagerSubsystem) RegisterCultivationModifier(modifier *CultivationSystemModifier) error {
    rm.mutex.Lock()
    defer rm.mutex.Unlock()
    
    // Add modifier to calculator
    err := rm.calculator.AddModifier(modifier.Modifier)
    if err != nil {
        return fmt.Errorf("failed to add cultivation modifier: %w", err)
    }
    
    return nil
}

// Example: Jindan System Modifier
func (js *JindanSystem) RegisterResourceModifiers(resourceManager *ResourceManagerSubsystem) error {
    // HP modifier from Jindan System
    hpModifier := &CultivationSystemModifier{
        SystemID:   "jindan_system",
        ResourceID: "hp",
        Modifier: &ResourceModifier{
            ID:         "jindan_hp_modifier",
            ResourceID: "hp",
            Type:       "multiplicative",
            Value:      1.5, // 50% increase
            Condition: &ModifierCondition{
                Type:     "realm",
                Operator: ">=",
                Value:    "foundation",
            },
            Priority: 100,
            System:   "jindan_system",
        },
    }
    
    // Mana modifier from Jindan System
    manaModifier := &CultivationSystemModifier{
        SystemID:   "jindan_system",
        ResourceID: "mana",
        Modifier: &ResourceModifier{
            ID:         "jindan_mana_modifier",
            ResourceID: "mana",
            Type:       "multiplicative",
            Value:      2.0, // 100% increase
            Condition: &ModifierCondition{
                Type:     "realm",
                Operator: ">=",
                Value:    "foundation",
            },
            Priority: 100,
            System:   "jindan_system",
        },
    }
    
    // Register modifiers
    err := resourceManager.RegisterCultivationModifier(hpModifier)
    if err != nil {
        return err
    }
    
    err = resourceManager.RegisterCultivationModifier(manaModifier)
    if err != nil {
        return err
    }
    
    return nil
}
```

### **2. Cultivation Resource Dependencies**

```go
// CultivationResourceDependency - Resource dependency from cultivation
type CultivationResourceDependency struct {
    SystemID      string                 `json:"system_id"`
    ResourceID    string                 `json:"resource_id"`
    Dependencies  []string               `json:"dependencies"`
    Calculation   *DependencyCalculation `json:"calculation"`
}

// DependencyCalculation - How to calculate dependency
type DependencyCalculation struct {
    Method        string                 `json:"method"`
    Formula       string                 `json:"formula"`
    Variables     map[string]string      `json:"variables"`
    Multipliers   map[string]float64     `json:"multipliers"`
}

// Example: Qi Energy dependency on cultivation level
func (cs *CultivationSystem) RegisterResourceDependencies(resourceManager *ResourceManagerSubsystem) error {
    // Qi Energy depends on cultivation level
    qiDependency := &CultivationResourceDependency{
        SystemID:     "cultivation_system",
        ResourceID:   "qi",
        Dependencies: []string{"cultivation_level", "realm"},
        Calculation: &DependencyCalculation{
            Method: "formula",
            Formula: "base_value * cultivation_level * realm_multiplier",
            Variables: map[string]string{
                "base_value": "10.0",
                "cultivation_level": "actor.cultivation_level",
                "realm_multiplier": "actor.realm_multiplier",
            },
            Multipliers: map[string]float64{
                "foundation": 1.0,
                "core_formation": 2.0,
                "golden_core": 5.0,
                "nascent_soul": 10.0,
            },
        },
    }
    
    // Register dependency
    err := resourceManager.RegisterResourceDependency(qiDependency)
    if err != nil {
        return fmt.Errorf("failed to register qi dependency: %w", err)
    }
    
    return nil
}
```

## 🔗 **Integration với Combat Core**

### **1. Resource Consumption trong Combat**

```go
// CombatResourceConsumer - Consumes resources during combat
type CombatResourceConsumer struct {
    resourceManager *ResourceManagerSubsystem
    eventBus        *ResourceEventBus
    mutex           sync.RWMutex
}

// ConsumeResource - Consume resource during combat
func (crc *CombatResourceConsumer) ConsumeResource(actor *Actor, resourceID string, amount float64) error {
    crc.mutex.Lock()
    defer crc.mutex.Unlock()
    
    // Get current resource value
    snapshot, err := crc.resourceManager.GetActorSnapshot(actor)
    if err != nil {
        return fmt.Errorf("failed to get actor snapshot: %w", err)
    }
    
    currentValue := snapshot.Primary[resourceID+"_current"]
    maxValue := snapshot.Primary[resourceID+"_max"]
    
    // Check if enough resource
    if currentValue < amount {
        return fmt.Errorf("insufficient %s: %f < %f", resourceID, currentValue, amount)
    }
    
    // Calculate new value
    newValue := currentValue - amount
    if newValue < 0 {
        newValue = 0
    }
    
    // Update resource
    err = crc.updateResource(actor, resourceID+"_current", newValue)
    if err != nil {
        return fmt.Errorf("failed to update resource: %w", err)
    }
    
    // Emit resource consumption event
    event := &ResourceEvent{
        Type:       "resource_consumed",
        ActorID:    actor.ID,
        ResourceID: resourceID,
        Amount:     amount,
        NewValue:   newValue,
        MaxValue:   maxValue,
        Timestamp:  time.Now(),
    }
    
    crc.eventBus.Emit(event)
    
    return nil
}

// RestoreResource - Restore resource (healing, mana potion, etc.)
func (crc *CombatResourceConsumer) RestoreResource(actor *Actor, resourceID string, amount float64) error {
    crc.mutex.Lock()
    defer crc.mutex.Unlock()
    
    // Get current resource value
    snapshot, err := crc.resourceManager.GetActorSnapshot(actor)
    if err != nil {
        return fmt.Errorf("failed to get actor snapshot: %w", err)
    }
    
    currentValue := snapshot.Primary[resourceID+"_current"]
    maxValue := snapshot.Primary[resourceID+"_max"]
    
    // Calculate new value
    newValue := currentValue + amount
    if newValue > maxValue {
        newValue = maxValue
    }
    
    // Update resource
    err = crc.updateResource(actor, resourceID+"_current", newValue)
    if err != nil {
        return fmt.Errorf("failed to update resource: %w", err)
    }
    
    // Emit resource restoration event
    event := &ResourceEvent{
        Type:       "resource_restored",
        ActorID:    actor.ID,
        ResourceID: resourceID,
        Amount:     amount,
        NewValue:   newValue,
        MaxValue:   maxValue,
        Timestamp:  time.Now(),
    }
    
    crc.eventBus.Emit(event)
    
    return nil
}
```

### **2. Resource Damage trong Combat**

```go
// CombatResourceDamager - Applies damage to resources
type CombatResourceDamager struct {
    resourceManager *ResourceManagerSubsystem
    eventBus        *ResourceEventBus
    mutex           sync.RWMutex
}

// ApplyResourceDamage - Apply damage to resource
func (crd *CombatResourceDamager) ApplyResourceDamage(actor *Actor, damage *ResourceDamage) error {
    crd.mutex.Lock()
    defer crd.mutex.Unlock()
    
    // Get current resource value
    snapshot, err := crd.resourceManager.GetActorSnapshot(actor)
    if err != nil {
        return fmt.Errorf("failed to get actor snapshot: %w", err)
    }
    
    currentValue := snapshot.Primary[damage.ResourceID+"_current"]
    maxValue := snapshot.Primary[damage.ResourceID+"_max"]
    
    // Calculate new value
    newValue := currentValue - damage.Amount
    if newValue < 0 {
        newValue = 0
    }
    
    // Update resource
    err = crd.updateResource(actor, damage.ResourceID+"_current", newValue)
    if err != nil {
        return fmt.Errorf("failed to update resource: %w", err)
    }
    
    // Emit resource damage event
    event := &ResourceEvent{
        Type:       "resource_damaged",
        ActorID:    actor.ID,
        ResourceID: damage.ResourceID,
        Amount:     damage.Amount,
        NewValue:   newValue,
        MaxValue:   maxValue,
        Timestamp:  time.Now(),
    }
    
    crd.eventBus.Emit(event)
    
    return nil
}

// ResourceDamage - Resource damage information
type ResourceDamage struct {
    ResourceID string  `json:"resource_id"`
    Amount     float64 `json:"amount"`
    Type       string  `json:"type"`
    Source     string  `json:"source"`
}
```

## 🔗 **Integration với Item Systems**

### **1. Item Resource Modifiers**

```go
// ItemResourceModifier - Resource modifier from item
type ItemResourceModifier struct {
    ItemID      string                 `json:"item_id"`
    ResourceID  string                 `json:"resource_id"`
    Modifier    *ResourceModifier      `json:"modifier"`
    Condition   *ModifierCondition     `json:"condition"`
    Priority    int64                  `json:"priority"`
}

// Register item resource modifier
func (rm *ResourceManagerSubsystem) RegisterItemModifier(modifier *ItemResourceModifier) error {
    rm.mutex.Lock()
    defer rm.mutex.Unlock()
    
    // Add modifier to calculator
    err := rm.calculator.AddModifier(modifier.Modifier)
    if err != nil {
        return fmt.Errorf("failed to add item modifier: %w", err)
    }
    
    return nil
}

// Example: Sword with HP bonus
func (is *ItemSystem) RegisterSwordModifiers(resourceManager *ResourceManagerSubsystem) error {
    // HP bonus from sword
    hpModifier := &ItemResourceModifier{
        ItemID:     "iron_sword",
        ResourceID: "hp",
        Modifier: &ResourceModifier{
            ID:         "iron_sword_hp_bonus",
            ResourceID: "hp",
            Type:       "additive",
            Value:      50.0, // +50 HP
            Condition: &ModifierCondition{
                Type:     "item_equipped",
                Operator: "==",
                Value:    "iron_sword",
            },
            Priority: 50,
            System:   "item_system",
        },
    }
    
    // Register modifier
    err := resourceManager.RegisterItemModifier(hpModifier)
    if err != nil {
        return fmt.Errorf("failed to register sword modifier: %w", err)
    }
    
    return nil
}
```

### **2. Consumable Items**

```go
// ConsumableItem - Item that consumes resources
type ConsumableItem struct {
    ItemID      string                 `json:"item_id"`
    Name        string                 `json:"name"`
    Consumables []*ResourceConsumable  `json:"consumables"`
    Effects     []*ResourceEffect      `json:"effects"`
}

// ResourceConsumable - Resource consumed by item
type ResourceConsumable struct {
    ResourceID string  `json:"resource_id"`
    Amount     float64 `json:"amount"`
    Type       string  `json:"type"` // "consume", "restore", "modify"
}

// ResourceEffect - Resource effect from item
type ResourceEffect struct {
    ResourceID string  `json:"resource_id"`
    Amount     float64 `json:"amount"`
    Type       string  `json:"type"` // "add", "multiply", "set"
    Duration   int64   `json:"duration"` // in seconds
}

// Use consumable item
func (is *ItemSystem) UseConsumableItem(actor *Actor, item *ConsumableItem) error {
    // Check if actor has enough resources
    for _, consumable := range item.Consumables {
        if consumable.Type == "consume" {
            err := is.resourceConsumer.ConsumeResource(actor, consumable.ResourceID, consumable.Amount)
            if err != nil {
                return fmt.Errorf("failed to consume resource: %w", err)
            }
        }
    }
    
    // Apply effects
    for _, effect := range item.Effects {
        err := is.applyResourceEffect(actor, effect)
        if err != nil {
            return fmt.Errorf("failed to apply resource effect: %w", err)
        }
    }
    
    return nil
}
```

## 🔗 **Integration với Event Systems**

### **1. Resource Events**

```go
// ResourceEvent - Resource-related event
type ResourceEvent struct {
    Type       string                 `json:"type"`
    ActorID    string                 `json:"actor_id"`
    ResourceID string                 `json:"resource_id"`
    Amount     float64                `json:"amount"`
    NewValue   float64                `json:"new_value"`
    MaxValue   float64                `json:"max_value"`
    Timestamp  time.Time              `json:"timestamp"`
    Metadata   map[string]interface{} `json:"metadata"`
}

// ResourceEventBus - Event bus for resource events
type ResourceEventBus struct {
    channels  map[string]chan *ResourceEvent
    workers   int
    mutex     sync.RWMutex
}

// Emit resource event
func (reb *ResourceEventBus) Emit(event *ResourceEvent) {
    reb.mutex.RLock()
    defer reb.mutex.RUnlock()
    
    // Send to all channels
    for _, channel := range reb.channels {
        select {
        case channel <- event:
        default:
            // Channel is full, skip
        }
    }
}

// Subscribe to resource events
func (reb *ResourceEventBus) Subscribe(channelName string) <-chan *ResourceEvent {
    reb.mutex.Lock()
    defer reb.mutex.Unlock()
    
    channel := make(chan *ResourceEvent, 100)
    reb.channels[channelName] = channel
    
    return channel
}
```

### **2. Event Handlers**

```go
// ResourceEventHandler - Handles resource events
type ResourceEventHandler struct {
    eventBus        *ResourceEventBus
    resourceManager *ResourceManagerSubsystem
    mutex           sync.RWMutex
}

// Handle resource events
func (reh *ResourceEventHandler) HandleEvents() {
    // Subscribe to resource events
    eventChannel := reh.eventBus.Subscribe("resource_events")
    
    for event := range eventChannel {
        reh.processEvent(event)
    }
}

// Process individual event
func (reh *ResourceEventHandler) processEvent(event *ResourceEvent) {
    switch event.Type {
    case "resource_consumed":
        reh.handleResourceConsumed(event)
    case "resource_restored":
        reh.handleResourceRestored(event)
    case "resource_damaged":
        reh.handleResourceDamaged(event)
    case "resource_depleted":
        reh.handleResourceDepleted(event)
    case "resource_regenerated":
        reh.handleResourceRegenerated(event)
    }
}

// Handle resource consumed event
func (reh *ResourceEventHandler) handleResourceConsumed(event *ResourceEvent) {
    // Log resource consumption
    log.Printf("Actor %s consumed %f %s", event.ActorID, event.Amount, event.ResourceID)
    
    // Check if resource is depleted
    if event.NewValue <= 0 {
        // Emit resource depleted event
        depletedEvent := &ResourceEvent{
            Type:       "resource_depleted",
            ActorID:    event.ActorID,
            ResourceID: event.ResourceID,
            Amount:     0,
            NewValue:   0,
            MaxValue:   event.MaxValue,
            Timestamp:  time.Now(),
        }
        
        reh.eventBus.Emit(depletedEvent)
    }
}
```

## 📊 **Performance Considerations**

### **1. Caching Strategy**

```go
// ResourceCache - Multi-layer caching
type ResourceCache struct {
    l1Cache    *sync.Map
    l2Cache    *ResourceL2Cache
    l3Cache    *ResourceL3Cache
    mutex      sync.RWMutex
    ttl        time.Duration
}

// Get cached resource
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

## 🚀 **Implementation Steps**

### **Step 1: Setup Resource Manager Subsystem**
1. Create Resource Manager Subsystem
2. Register with Actor Core Plugin Registry
3. Configure resource definitions
4. Test basic resource calculation

### **Step 2: Integrate với Cultivation Systems**
1. Register cultivation modifiers
2. Add resource dependencies
3. Test cultivation resource calculations
4. Validate resource conflicts

### **Step 3: Integrate với Combat Core**
1. Add resource consumption
2. Add resource damage
3. Add resource restoration
4. Test combat resource interactions

### **Step 4: Integrate với Item Systems**
1. Add item resource modifiers
2. Add consumable items
3. Test item resource effects
4. Validate item resource interactions

### **Step 5: Add Event System Integration**
1. Add resource events
2. Add event handlers
3. Test event processing
4. Validate event performance

## ❓ **Questions for Discussion**

1. **Resource Persistence**: Làm thế nào để persist resource changes?
2. **Resource Synchronization**: Làm thế nào để sync resources across multiple systems?
3. **Resource Validation**: Làm thế nào để validate resource values?
4. **Resource Performance**: Làm thế nào để optimize resource calculations?
5. **Resource Testing**: Làm thế nào để test resource integrations?

### ✅ Proposed Answers / 建议 / Gợi ý

- **Resource Persistence** (事件溯源 + 快照 / Ghi nhật ký sự kiện + snapshot)
  - Event-sourcing deltas with idempotency keys; periodic snapshots of `*_current` for fast loads.
  - Atomic batches (transactional apply) with WAL before commit. Include `correlation_id` for auditing.
  - Storage model: `resource_events(actor_id, ts, dimension, delta, cause, idem_key)` + `resource_snapshots(actor_id, version, map)`.

- **Resource Synchronization** (单写者分片 + 乐观并发 / Một writer theo shard + ETag)
  - Single-writer-per-actor via shard routing (e.g., consistent hash). All writes go through that shard.
  - Optimistic concurrency: include `actor_version`/ETag; on mismatch, re-read and retry.
  - Async fan-out to caches and subscribers via `resource_change` events (eventual consistency for reads).

- **Resource Validation** (分层夹紧 + 业务不变量 / Kiểm tra clamp + bất biến)
  - Enforce clamp pipeline: EffectiveCaps → Combiner `clamp_default` → constants clamp ranges.
  - Validate invariants before persist: `0 ≤ *_current ≤ *_max`, `regen ≥ 0` (unless designed negative), shield-first absorption, etc.
  - Schema validation on API (types/ranges), and per-dimension business rules.

- **Resource Performance** (批量化 + 无锁L1缓存 / Gộp batch + cache L1 không khóa)
  - Batch contributions; avoid per-request file I/O; reuse Aggregator and registries.
  - Cache: L1 lock-free, L2 memory-mapped, optional L3 persistent/Redis; warm critical keys.
  - Prefer operator-mode for aggregate ratios where pipeline is unnecessary; short-circuit no-op deltas.

- **Resource Testing** (Golden + Property-based / Bộ mẫu vàng + Proptest)
  - Golden vectors for common MMO flows (DPS + heal same tick, OOC regen, shield decay, offline catch-up).
  - Property tests for determinism (order invariance), clamp invariants, idempotency, and monotonicity.
  - Harness executes vectors via a temporary subsystem; asserts numeric equality on `primary` and derived.

## 🎯 **Next Steps**

1. **Implement Resource Manager Subsystem**: Basic structure
2. **Add Resource Registry**: Define all resource types
3. **Integrate với Actor Core**: As a Subsystem
4. **Add Resource Modifiers**: From cultivation systems
5. **Test Resource Calculations**: Validate accuracy

---

*Tài liệu này sẽ được cập nhật khi có thêm yêu cầu và feedback từ team.*

## API Contracts

### Transactional Consumption

#### POST /resource/consume
- Description: Consume an amount from a resource with version precondition and idempotency.
- Request
```json
{
  "actor_id": "uuid",
  "resource_id": "hp_current",
  "amount": 150.0,
  "snapshot_version": 42,
  "idempotency_key": "skill_12345_cast_67890"
}
```
- Responses
  - 200 OK
  ```json
  { "ok": true, "new_value": 850.0, "version": 43 }
  ```
  - 409 Conflict (stale version)
  ```json
  { "ok": false, "error": "version_conflict", "latest_version": 44 }
  ```
  - 422 Unprocessable
  ```json
  { "ok": false, "error": "insufficient_resource", "current": 50.0 }
  ```

#### POST /resource/restore
- Same schema as consume; adds amount to the resource (clamped to max).

#### POST /resource/batch_consume
- Description: Atomic multi-resource consumption (all-or-none).
- Request
```json
{
  "actor_id": "uuid",
  "costs": [
    { "resource_id": "mana_current", "amount": 80.0 },
    { "resource_id": "stamina_current", "amount": 25.0 }
  ],
  "snapshot_version": 42,
  "idempotency_key": "skill_abc_combo_1"
}
```
- Responses
  - 200 OK with new values per resource
  - 409 Conflict with `latest_version`
  - 422 Unprocessable with `insufficient_resource` and offending resource list

### Error Codes
- `version_conflict`: Provided `snapshot_version` differs from latest.
- `insufficient_resource`: Cannot cover the requested amount.
- `invalid_resource`: Unknown or immutable resource.
- `invalid_input`: Schema/validation errors.
- `internal_error`: Unexpected failure.

### Notes
- All mutations are server-authoritative; client may predict UI but must reconcile.
- Idempotency keys should be unique per actor-action; TTL aligns with server retry window.
- Prefer using `actor_core` snapshot immediately before consumption to minimize conflicts.

## 🔧 Implement Guide: Integrate Resource Manager with Actor Core / 实施指南 / Hướng dẫn tích hợp

This section provides exact steps, file paths, Rust types, and commands to integrate Resource Manager as an Actor Core subsystem.

### 1) Module scaffolding (模块骨架/Khung module)
- Create files:
  - `crates/actor-core/src/subsystems/mod.rs`
  - `crates/actor-core/src/subsystems/resource_manager.rs`

Add module index:
```rust
// crates/actor-core/src/subsystems/mod.rs
pub mod resource_manager;
```

Export from library:
```rust
// crates/actor-core/src/lib.rs
pub mod subsystems; // ensure this line exists
```

### 2) Implement `ResourceManagerSubsystem` (实现子系统/Cài đặt subsystem)
- File: `crates/actor-core/src/subsystems/resource_manager.rs`
- Struct: `ResourceManagerSubsystem`
- Implements: `crate::interfaces::Subsystem`

Template:
```rust
use crate::interfaces::Subsystem as SubsystemTrait;
use crate::types::{SubsystemOutput, Contribution};
use crate::enums::Bucket;

pub struct ResourceManagerSubsystem {
    system_id: String,
    priority: i64,
}

impl ResourceManagerSubsystem {
    pub fn new() -> Self { Self { system_id: "resource_manager".into(), priority: 100 } }
}

#[async_trait::async_trait]
impl SubsystemTrait for ResourceManagerSubsystem {
    fn system_id(&self) -> &str { &self.system_id }
    fn priority(&self) -> i64 { self.priority }
    async fn contribute(&self, _actor: &crate::types::Actor) -> crate::ActorCoreResult<SubsystemOutput> {
        let mut out = SubsystemOutput::new(self.system_id.clone());
        // Emit baseline/example contributions. Replace with real actor state.
        out.add_primary(Contribution{ dimension: "hp_max".into(), bucket: Bucket::Flat, value: 1000.0, system: self.system_id.clone(), priority: Some(100), tags: None });
        out.add_primary(Contribution{ dimension: "hp_current".into(), bucket: Bucket::Flat, value: 960.0, system: self.system_id.clone(), priority: Some(100), tags: None });
        Ok(out)
    }
}
```

### 3) Register subsystem (注册/Cấu hình đăng ký)
- File: `crates/actor-core/src/registry.rs`
- In the default registration path, add:
```rust
self.register(Box::new(crate::subsystems::resource_manager::ResourceManagerSubsystem::new()))?;
```

Place the line where other subsystems are registered (e.g., in `PluginRegistryImpl` bootstrap).

### 4) Configuration wiring (配置接入/Cấu hình)
- Ensure loader reads from env var:
  - `ACTOR_CORE_CONFIG_DIR = docs/resource-manager/configs`
- Files provided:
  - `docs/resource-manager/configs/combiner.resources.yaml`
  - `docs/resource-manager/configs/cap_layers.resources.yaml`
- Combiner rules specify pipeline vs operator mode and clamp defaults per dimension.

PowerShell example:
```powershell
$Env:ACTOR_CORE_CONFIG_DIR = "docs/resource-manager/configs"
```

### 5) Tick/decay/offline semantics (逻辑/Ngữ nghĩa)
- Tick: add `*_regen * delta_seconds` to `*_current` using `Bucket::Flat`.
- Shield decay: add `-shield_per_second * delta_seconds` to `shield_current`.
- Offline catch-up: add `min(offline_seconds, offline_regen_max_seconds) * regen` to current.
- Clamp precedence: EffectiveCaps → Combiner `clamp_default` → constants clamp ranges.
- Derived: `hp_percentage` via operator-mode configured in combiner.

### 6) End-to-end usage (端到端/Đầu-cuối)
Minimal example to resolve an actor snapshot:
```rust
use actor_core::{RegistryFactory, ServiceFactory, CacheFactory};

let plugin = RegistryFactory::create_plugin_registry();
let combiner = RegistryFactory::create_combiner_registry();
let cap_layers = RegistryFactory::create_cap_layer_registry();
let caps = ServiceFactory::create_caps_provider(cap_layers);
let cache = CacheFactory::create_in_memory_cache(10_000, 600);
let aggregator = ServiceFactory::create_aggregator(plugin, combiner, caps, cache);

let rt = tokio::runtime::Runtime::new().unwrap();
let actor = actor_core::types::Actor::new("ActorA".into(), "Human".into());
let snapshot = rt.block_on(aggregator.resolve(&actor)).unwrap();
assert!(snapshot.primary.get("hp_current").is_some());
```

### 7) Golden vectors & tests (用例/Kiểm thử)
- Harness: `crates/actor-core/tests/golden_vector_harness.rs`
- Set env and run:
```powershell
$Env:ACTOR_CORE_CONFIG_DIR = "docs/resource-manager/configs"
cargo test golden_vector_harness -- --nocapture
```
- Run property and operator tests; use `--features extra_buckets` if needed.