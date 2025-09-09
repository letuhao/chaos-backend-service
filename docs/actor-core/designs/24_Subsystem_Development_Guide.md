# 24 — Subsystem Development Guide

**Updated:** 2025-09-08 00:55

This comprehensive guide helps developers create robust, performant subsystems for Actor Core v3.

## Overview

Subsystems are the core building blocks of Actor Core v3. They implement domain-specific logic and contribute to actor stats through the standardized `SubsystemOutput` interface.

## Subsystem Interface

### Core Interface
```go
type Subsystem interface {
    SystemID() string
    Priority() int
    Contribute(ctx context.Context, actor *Actor) (SubsystemOutput, error)
}
```

### Optional Interfaces
```go
// Configuration support
type ConfigurableSubsystem interface {
    Configure(config map[string]interface{}) error
}

// Validation support
type ValidatingSubsystem interface {
    Validate(actor *Actor) error
}

// Caching support
type CachingSubsystem interface {
    GetCacheKey(actor *Actor) string
    ShouldCache() bool
}

// Lifecycle support
type LifecycleSubsystem interface {
    Initialize() error
    Shutdown() error
}
```

## Basic Subsystem Implementation

### Step 1: Define the Subsystem Structure
```go
type CombatSubsystem struct {
    systemID string
    priority int
    config   CombatConfig
    cache    map[string]*Snapshot
    mu       sync.RWMutex
}

type CombatConfig struct {
    BaseDamage      float64 `yaml:"base_damage"`
    CritMultiplier  float64 `yaml:"crit_multiplier"`
    WeaponBonus     float64 `yaml:"weapon_bonus"`
    ArmorPenetration float64 `yaml:"armor_penetration"`
}
```

### Step 2: Implement Core Interface
```go
func NewCombatSubsystem(config CombatConfig) *CombatSubsystem {
    return &CombatSubsystem{
        systemID: "combat",
        priority: 100,
        config:   config,
        cache:    make(map[string]*Snapshot),
    }
}

func (cs *CombatSubsystem) SystemID() string {
    return cs.systemID
}

func (cs *CombatSubsystem) Priority() int {
    return cs.priority
}

func (cs *CombatSubsystem) Contribute(ctx context.Context, actor *Actor) (SubsystemOutput, error) {
    // Check for cancellation
    select {
    case <-ctx.Done():
        return SubsystemOutput{}, ctx.Err()
    default:
    }
    
    // Calculate contributions based on actor state
    primary, derived, caps, context := cs.calculateContributions(actor)
    
    return SubsystemOutput{
        Primary: primary,
        Derived: derived,
        Caps:    caps,
        Context: context,
        Meta: SubsystemMeta{
            System:  cs.systemID,
            Version: 1,
        },
    }, nil
}
```

### Step 3: Implement Contribution Logic
```go
func (cs *CombatSubsystem) calculateContributions(actor *Actor) ([]Contribution, []Contribution, []CapContribution, map[string]ModifierPack) {
    // Primary contributions
    primary := []Contribution{
        {Dimension: "strength", Bucket: "FLAT", Value: cs.config.BaseDamage, System: cs.systemID},
    }
    
    // Derived contributions
    derived := []Contribution{
        {Dimension: "attack_power", Bucket: "MULT", Value: cs.config.WeaponBonus, System: cs.systemID},
        {Dimension: "crit_damage", Bucket: "FLAT", Value: cs.config.CritMultiplier - 1.0, System: cs.systemID},
    }
    
    // Cap contributions
    caps := []CapContribution{
        {System: cs.systemID, Dimension: "attack_power", Mode: "ADDITIVE", Kind: "max", Value: 1000, Scope: "TOTAL"},
        {System: cs.systemID, Dimension: "crit_rate", Mode: "HARD_MAX", Kind: "max", Value: 0.5, Scope: "TOTAL"},
    }
    
    // Context modifiers
    context := map[string]ModifierPack{
        "damage": {
            AdditivePercent: 0.1, // +10% damage
            Multipliers:     []float64{1.2}, // 1.2x multiplier
            PostAdd:         5.0, // +5 flat damage
        },
    }
    
    return primary, derived, caps, context
}
```

## Advanced Subsystem Features

### Configuration Support
```go
func (cs *CombatSubsystem) Configure(config map[string]interface{}) error {
    if baseDamage, ok := config["base_damage"].(float64); ok {
        cs.config.BaseDamage = baseDamage
    }
    
    if critMultiplier, ok := config["crit_multiplier"].(float64); ok {
        cs.config.CritMultiplier = critMultiplier
    }
    
    if priority, ok := config["priority"].(int); ok {
        cs.priority = priority
    }
    
    return nil
}
```

### Validation Support
```go
func (cs *CombatSubsystem) Validate(actor *Actor) error {
    // Validate actor has required attributes
    if actor.Strength < 0 {
        return fmt.Errorf("actor strength cannot be negative")
    }
    
    // Validate configuration
    if cs.config.BaseDamage < 0 {
        return fmt.Errorf("base damage cannot be negative")
    }
    
    return nil
}
```

### Caching Support
```go
func (cs *CombatSubsystem) GetCacheKey(actor *Actor) string {
    return fmt.Sprintf("%s:%d:%d", actor.ID, actor.Version, cs.config.Version)
}

func (cs *CombatSubsystem) ShouldCache() bool {
    return true
}

func (cs *CombatSubsystem) getCachedOutput(actor *Actor) (SubsystemOutput, bool) {
    cs.mu.RLock()
    defer cs.mu.RUnlock()
    
    key := cs.GetCacheKey(actor)
    output, exists := cs.cache[key]
    return output, exists
}

func (cs *CombatSubsystem) setCachedOutput(actor *Actor, output SubsystemOutput) {
    cs.mu.Lock()
    defer cs.mu.Unlock()
    
    key := cs.GetCacheKey(actor)
    cs.cache[key] = &output
}
```

### Lifecycle Support
```go
func (cs *CombatSubsystem) Initialize() error {
    // Initialize any required resources
    cs.cache = make(map[string]*Snapshot)
    
    // Load configuration
    if err := cs.loadConfig(); err != nil {
        return fmt.Errorf("failed to load combat config: %w", err)
    }
    
    return nil
}

func (cs *CombatSubsystem) Shutdown() error {
    // Clean up resources
    cs.mu.Lock()
    defer cs.mu.Unlock()
    
    cs.cache = nil
    return nil
}
```

## Subsystem Types

### 1. Combat Subsystems
```go
// Physical combat
type PhysicalCombatSubsystem struct {
    *CombatSubsystem
    weaponType string
}

// Magical combat
type MagicalCombatSubsystem struct {
    *CombatSubsystem
    spellSchool string
}

// Ranged combat
type RangedCombatSubsystem struct {
    *CombatSubsystem
    weaponRange float64
}
```

### 2. Progression Subsystems
```go
// Experience and leveling
type ExperienceSubsystem struct {
    systemID string
    priority int
    config   ExperienceConfig
}

// Cultivation system
type CultivationSubsystem struct {
    systemID string
    priority int
    config   CultivationConfig
}

// Skill development
type SkillSubsystem struct {
    systemID string
    priority int
    skills   map[string]Skill
}
```

### 3. Social Subsystems
```go
// Reputation system
type ReputationSubsystem struct {
    systemID string
    priority int
    factions map[string]Faction
}

// Guild system
type GuildSubsystem struct {
    systemID string
    priority int
    guild    *Guild
}

// Trading system
type TradingSubsystem struct {
    systemID string
    priority int
    market   *Market
}
```

### 4. Environmental Subsystems
```go
// Weather effects
type WeatherSubsystem struct {
    systemID string
    priority int
    weather  Weather
}

// Location effects
type LocationSubsystem struct {
    systemID string
    priority int
    location *Location
}

// Time effects
type TimeSubsystem struct {
    systemID string
    priority int
    time     TimeOfDay
}
```

## Best Practices

### 1. Performance Optimization
```go
// Use efficient data structures
type OptimizedSubsystem struct {
    // Pre-allocate slices
    primaryContributions []Contribution
    derivedContributions []Contribution
    capContributions     []CapContribution
    
    // Use object pooling
    pool sync.Pool
}

func (os *OptimizedSubsystem) getContributions() []Contribution {
    contributions := os.pool.Get().([]Contribution)
    return contributions[:0] // Reset length but keep capacity
}

func (os *OptimizedSubsystem) putContributions(contributions []Contribution) {
    os.pool.Put(contributions)
}
```

### 2. Error Handling
```go
func (cs *CombatSubsystem) Contribute(ctx context.Context, actor *Actor) (SubsystemOutput, error) {
    // Validate input
    if actor == nil {
        return SubsystemOutput{}, fmt.Errorf("actor cannot be nil")
    }
    
    // Check for cancellation
    select {
    case <-ctx.Done():
        return SubsystemOutput{}, ctx.Err()
    default:
    }
    
    // Defer panic recovery
    defer func() {
        if r := recover(); r != nil {
            log.Error("Panic in combat subsystem", "panic", r, "actor", actor.ID)
        }
    }()
    
    // Calculate contributions
    return cs.calculateContributions(actor), nil
}
```

### 3. Logging and Monitoring
```go
func (cs *CombatSubsystem) Contribute(ctx context.Context, actor *Actor) (SubsystemOutput, error) {
    start := time.Now()
    defer func() {
        duration := time.Since(start)
        log.Debug("Combat subsystem contribution completed",
            "actor", actor.ID,
            "duration", duration,
            "system", cs.systemID)
    }()
    
    // ... contribution logic
}
```

### 4. Testing
```go
func TestCombatSubsystem(t *testing.T) {
    config := CombatConfig{
        BaseDamage:     10.0,
        CritMultiplier: 2.0,
        WeaponBonus:    0.1,
    }
    
    subsystem := NewCombatSubsystem(config)
    actor := &Actor{
        ID:       "test-actor",
        Strength: 100,
    }
    
    output, err := subsystem.Contribute(context.Background(), actor)
    assert.NoError(t, err)
    assert.Equal(t, "combat", output.Meta.System)
    assert.Len(t, output.Primary, 1)
    assert.Len(t, output.Derived, 2)
    assert.Len(t, output.Caps, 2)
}
```

## Configuration Management

### YAML Configuration
```yaml
# combat-subsystem.yaml
combat:
  base_damage: 10.0
  crit_multiplier: 2.0
  weapon_bonus: 0.1
  armor_penetration: 0.05
  priority: 100
  enabled: true
```

### Configuration Loading
```go
func LoadCombatConfig(filename string) (CombatConfig, error) {
    data, err := ioutil.ReadFile(filename)
    if err != nil {
        return CombatConfig{}, err
    }
    
    var config struct {
        Combat CombatConfig `yaml:"combat"`
    }
    
    err = yaml.Unmarshal(data, &config)
    if err != nil {
        return CombatConfig{}, err
    }
    
    return config.Combat, nil
}
```

## Integration with Actor Core

### Registration
```go
func RegisterSubsystems(registry *PluginRegistry) error {
    // Load configurations
    combatConfig, err := LoadCombatConfig("config/combat.yaml")
    if err != nil {
        return err
    }
    
    // Create and register subsystems
    combatSubsystem := NewCombatSubsystem(combatConfig)
    if err := registry.Register(combatSubsystem); err != nil {
        return err
    }
    
    return nil
}
```

### Usage in Aggregator
```go
func (a *Aggregator) Resolve(actor *Actor) (*Snapshot, error) {
    // Get all subsystems
    subsystems := a.registry.GetSubsystems()
    
    // Process each subsystem
    var outputs []SubsystemOutput
    for _, subsystem := range subsystems {
        output, err := subsystem.Contribute(context.Background(), actor)
        if err != nil {
            return nil, fmt.Errorf("subsystem %s failed: %w", subsystem.SystemID(), err)
        }
        outputs = append(outputs, output)
    }
    
    // Aggregate outputs
    return a.aggregateOutputs(outputs)
}
```

## Common Patterns

### 1. Stateful Subsystems
```go
type StatefulSubsystem struct {
    state map[string]interface{}
    mu    sync.RWMutex
}

func (ss *StatefulSubsystem) GetState(key string) (interface{}, bool) {
    ss.mu.RLock()
    defer ss.mu.RUnlock()
    
    value, exists := ss.state[key]
    return value, exists
}

func (ss *StatefulSubsystem) SetState(key string, value interface{}) {
    ss.mu.Lock()
    defer ss.mu.Unlock()
    
    ss.state[key] = value
}
```

### 2. Event-Driven Subsystems
```go
type EventDrivenSubsystem struct {
    eventHandlers map[string]func(*Actor, interface{}) error
    mu            sync.RWMutex
}

func (eds *EventDrivenSubsystem) RegisterHandler(eventType string, handler func(*Actor, interface{}) error) {
    eds.mu.Lock()
    defer eds.mu.Unlock()
    
    eds.eventHandlers[eventType] = handler
}

func (eds *EventDrivenSubsystem) HandleEvent(actor *Actor, eventType string, data interface{}) error {
    eds.mu.RLock()
    handler, exists := eds.eventHandlers[eventType]
    eds.mu.RUnlock()
    
    if !exists {
        return fmt.Errorf("no handler for event type: %s", eventType)
    }
    
    return handler(actor, data)
}
```

### 3. Conditional Subsystems
```go
type ConditionalSubsystem struct {
    condition func(*Actor) bool
    subsystem Subsystem
}

func (cs *ConditionalSubsystem) Contribute(ctx context.Context, actor *Actor) (SubsystemOutput, error) {
    if !cs.condition(actor) {
        return SubsystemOutput{}, nil
    }
    
    return cs.subsystem.Contribute(ctx, actor)
}
```

## Conclusion

This guide provides a comprehensive foundation for developing subsystems for Actor Core v3. By following these patterns and best practices, developers can create robust, performant, and maintainable subsystems that integrate seamlessly with the core system.

Key takeaways:
1. **Implement the core interface correctly**
2. **Use optional interfaces for advanced functionality**
3. **Follow performance best practices**
4. **Implement proper error handling and logging**
5. **Write comprehensive tests**
6. **Use configuration management**
7. **Follow common patterns for reusability**
