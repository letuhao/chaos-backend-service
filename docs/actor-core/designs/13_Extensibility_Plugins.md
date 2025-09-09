# 13 — Extensibility & Plugins

**Updated:** 2025-09-08 00:45

This document outlines how to extend Actor Core v3 with new layers, buckets, and subsystems while maintaining system stability.

## Plugin Architecture

### Subsystem Plugin Interface
```go
type Subsystem interface {
    SystemID() string
    Priority() int
    Contribute(ctx context.Context, actor *Actor) (SubsystemOutput, error)
}

// Optional interfaces for advanced functionality
type ConfigurableSubsystem interface {
    Configure(config map[string]interface{}) error
}

type ValidatingSubsystem interface {
    Validate(actor *Actor) error
}

type CachingSubsystem interface {
    GetCacheKey(actor *Actor) string
    ShouldCache() bool
}
```

### Plugin Registration
```go
type PluginRegistry struct {
    subsystems map[string]Subsystem
    mu         sync.RWMutex
}

func (pr *PluginRegistry) Register(subsystem Subsystem) error {
    pr.mu.Lock()
    defer pr.mu.Unlock()
    
    if _, exists := pr.subsystems[subsystem.SystemID()]; exists {
        return fmt.Errorf("subsystem %s already registered", subsystem.SystemID())
    }
    
    pr.subsystems[subsystem.SystemID()] = subsystem
    return nil
}

func (pr *PluginRegistry) Get(systemID string) (Subsystem, bool) {
    pr.mu.RLock()
    defer pr.mu.RUnlock()
    
    subsystem, exists := pr.subsystems[systemID]
    return subsystem, exists
}
```

## Extending Layers

### Adding New Layers
```yaml
# CapLayerRegistry.yaml
order: [REALM, WORLD, EVENT, GUILD, TOTAL]
across_policy: INTERSECT

# New layer: GUILD
# Guild-specific caps that apply to all guild members
```

### Layer Implementation
```go
type GuildLayerProvider struct {
    guildCaps map[string]EffectiveCaps
}

func (glp *GuildLayerProvider) GetCapsForGuild(guildID string) EffectiveCaps {
    return glp.guildCaps[guildID]
}

func (glp *GuildLayerProvider) EffectiveCapsWithinLayer(ctx context.Context, actor *Actor, outputs []SubsystemOutput, layer string) (EffectiveCaps, error) {
    if layer != "GUILD" {
        return nil, fmt.Errorf("unsupported layer: %s", layer)
    }
    
    // Get guild ID from actor context
    guildID := actor.GetGuildID()
    if guildID == "" {
        return make(EffectiveCaps), nil
    }
    
    return glp.GetCapsForGuild(guildID), nil
}
```

## Extending Buckets

### Adding New Bucket Types
```go
type Bucket string

const (
    BucketFlat     Bucket = "FLAT"
    BucketMult     Bucket = "MULT"
    BucketPostAdd  Bucket = "POST_ADD"
    BucketOverride Bucket = "OVERRIDE"
    
    // New bucket types
    BucketExponential Bucket = "EXPONENTIAL"  // value^exponent
    BucketLogarithmic Bucket = "LOGARITHMIC"  // log(value)
    BucketConditional Bucket = "CONDITIONAL"  // conditional application
)
```

### Bucket Processing
```go
func (a *Aggregator) processBucket(bucket Bucket, value float64, baseValue float64) float64 {
    switch bucket {
    case BucketFlat:
        return baseValue + value
    case BucketMult:
        return baseValue * (1 + value)
    case BucketPostAdd:
        return baseValue + value
    case BucketOverride:
        return value
    case BucketExponential:
        return baseValue * math.Pow(value, 2) // Example: value^2
    case BucketLogarithmic:
        return baseValue * math.Log(value)
    case BucketConditional:
        // Apply only if condition is met
        return baseValue
    default:
        return baseValue
    }
}
```

## Subsystem Development Guide

### Basic Subsystem Implementation
```go
type MySubsystem struct {
    systemID string
    priority int
    config   map[string]interface{}
}

func NewMySubsystem(config map[string]interface{}) *MySubsystem {
    return &MySubsystem{
        systemID: "my_subsystem",
        priority: 100,
        config:   config,
    }
}

func (ms *MySubsystem) SystemID() string {
    return ms.systemID
}

func (ms *MySubsystem) Priority() int {
    return ms.priority
}

func (ms *MySubsystem) Contribute(ctx context.Context, actor *Actor) (SubsystemOutput, error) {
    // Calculate contributions based on actor state
    primary := []Contribution{
        {Dimension: "strength", Bucket: "FLAT", Value: 50, System: ms.systemID},
    }
    
    derived := []Contribution{
        {Dimension: "hp_max", Bucket: "MULT", Value: 0.1, System: ms.systemID},
    }
    
    caps := []CapContribution{
        {System: ms.systemID, Dimension: "strength", Mode: "ADDITIVE", Kind: "max", Value: 100, Scope: "TOTAL"},
    }
    
    return SubsystemOutput{
        Primary: primary,
        Derived: derived,
        Caps:    caps,
        Context: make(map[string]ModifierPack),
        Meta:    SubsystemMeta{System: ms.systemID, Version: 1},
    }, nil
}
```

### Advanced Subsystem Features
```go
type AdvancedSubsystem struct {
    *MySubsystem
    cache    map[string]*Snapshot
    validator func(*Actor) error
}

func (as *AdvancedSubsystem) Configure(config map[string]interface{}) error {
    // Configure subsystem based on config
    if priority, ok := config["priority"].(int); ok {
        as.priority = priority
    }
    return nil
}

func (as *AdvancedSubsystem) Validate(actor *Actor) error {
    if as.validator != nil {
        return as.validator(actor)
    }
    return nil
}

func (as *AdvancedSubsystem) GetCacheKey(actor *Actor) string {
    return fmt.Sprintf("%s:%d", actor.ID, actor.Version)
}

func (as *AdvancedSubsystem) ShouldCache() bool {
    return true
}
```

## Plugin Configuration

### Configuration Schema
```yaml
# subsystem-config.yaml
subsystems:
  - id: "combat"
    priority: 100
    enabled: true
    config:
      base_damage: 10
      crit_multiplier: 2.0
      
  - id: "magic"
    priority: 200
    enabled: true
    config:
      mana_cost: 5
      spell_power: 1.5
      
  - id: "cultivation"
    priority: 300
    enabled: false
    config:
      cultivation_speed: 1.0
      breakthrough_chance: 0.1
```

### Configuration Loading
```go
type SubsystemConfig struct {
    ID       string                 `yaml:"id"`
    Priority int                    `yaml:"priority"`
    Enabled  bool                   `yaml:"enabled"`
    Config   map[string]interface{} `yaml:"config"`
}

func LoadSubsystemConfigs(filename string) ([]SubsystemConfig, error) {
    data, err := ioutil.ReadFile(filename)
    if err != nil {
        return nil, err
    }
    
    var config struct {
        Subsystems []SubsystemConfig `yaml:"subsystems"`
    }
    
    err = yaml.Unmarshal(data, &config)
    if err != nil {
        return nil, err
    }
    
    return config.Subsystems, nil
}
```

## Versioning & Compatibility

### Subsystem Versioning
```go
type SubsystemMeta struct {
    System    string `json:"system"`
    Version   int64  `json:"version"`
    APILevel  int    `json:"api_level"`
    Compatible bool  `json:"compatible"`
}

func (sm *SubsystemMeta) IsCompatible(requiredAPILevel int) bool {
    return sm.APILevel >= requiredAPILevel
}
```

### Backward Compatibility
```go
type CompatibilityChecker struct {
    minAPILevel int
    supportedVersions []int64
}

func (cc *CompatibilityChecker) CheckCompatibility(meta SubsystemMeta) error {
    if !meta.IsCompatible(cc.minAPILevel) {
        return fmt.Errorf("subsystem %s requires API level %d, but minimum is %d", 
            meta.System, meta.APILevel, cc.minAPILevel)
    }
    
    if !cc.isVersionSupported(meta.Version) {
        return fmt.Errorf("subsystem %s version %d is not supported", 
            meta.System, meta.Version)
    }
    
    return nil
}
```

## Testing Plugins

### Unit Testing
```go
func TestMySubsystem(t *testing.T) {
    subsystem := NewMySubsystem(map[string]interface{}{
        "priority": 100,
    })
    
    actor := &Actor{
        ID: "test-actor",
        Subsystems: []Subsystem{subsystem},
    }
    
    output, err := subsystem.Contribute(context.Background(), actor)
    assert.NoError(t, err)
    assert.Equal(t, "my_subsystem", output.Meta.System)
    assert.Len(t, output.Primary, 1)
}
```

### Integration Testing
```go
func TestSubsystemIntegration(t *testing.T) {
    registry := NewPluginRegistry()
    
    // Register multiple subsystems
    registry.Register(NewCombatSubsystem())
    registry.Register(NewMagicSubsystem())
    registry.Register(NewCultivationSubsystem())
    
    aggregator := NewAggregator(registry)
    actor := createTestActor()
    
    snapshot, err := aggregator.Resolve(actor)
    assert.NoError(t, err)
    assert.NotNil(t, snapshot)
}
```

## Best Practices

### 1. Subsystem Design
- Keep subsystems focused on single responsibility
- Use consistent naming conventions
- Implement proper error handling
- Support configuration and validation

### 2. Performance Considerations
- Implement caching where appropriate
- Avoid expensive operations in hot paths
- Use efficient data structures
- Consider memory usage

### 3. Testing
- Write comprehensive unit tests
- Test error conditions
- Test performance characteristics
- Test integration with other subsystems

### 4. Documentation
- Document subsystem behavior
- Provide usage examples
- Document configuration options
- Maintain API documentation

## Conclusion

The plugin architecture of Actor Core v3 provides a flexible foundation for extending functionality while maintaining system stability. By following these guidelines, developers can create robust, performant subsystems that integrate seamlessly with the core system.
