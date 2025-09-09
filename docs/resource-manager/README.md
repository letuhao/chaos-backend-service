# Resource Manager System

## 📋 **Tổng Quan**

Resource Manager System là một **Subsystem** của Actor Core v3, được thiết kế để quản lý tất cả các loại tài nguyên (resources) trong game thông qua hệ thống Contributions và Snapshot. Hệ thống này tuân thủ nguyên tắc "metadata-only aggregator" của Actor Core, không lưu trữ state mà chỉ cung cấp logic tính toán resources.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Actor Core Compliance**
- **Subsystem Pattern**: Resource Manager là một Subsystem, không phải Core component
- **Contribution-Based**: Sử dụng Contribution system để output resource values
- **No State Storage**: Không lưu trữ state, chỉ tính toán dựa trên Actor metadata
- **Snapshot Integration**: Resources được expose qua Snapshot của Actor Core

### **2. Flexible & Extensible**
- Hỗ trợ nhiều loại resources khác nhau
- Dễ dàng thêm mới resource types
- Tương thích với các cultivation systems

### **3. Performance Optimized**
- Sử dụng caching của Actor Core
- Batch processing cho multiple actors
- Lazy calculation khi cần thiết

## 🏗️ **Kiến Trúc**

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

## 📊 **Resource Categories**

### **1. Health Resources**
- `hp_current` - Sinh mệnh hiện tại
- `hp_max` - Sinh mệnh tối đa
- `hp_regen` - Tốc độ hồi máu
- `lifespan_years` - Tuổi thọ (năm)
- `age_years` - Tuổi hiện tại (năm)

### **2. Energy Resources**
- `mana_current` - Mana hiện tại
- `mana_max` - Mana tối đa
- `mana_regen` - Tốc độ hồi mana
- `spiritual_energy_current` - Linh lực hiện tại
- `spiritual_energy_max` - Linh lực tối đa

### **3. Physical Resources**
- `stamina_current` - Thể lực hiện tại
- `stamina_max` - Thể lực tối đa
- `stamina_regen` - Tốc độ hồi thể lực
- `vitality_current` - Sinh lực hiện tại
- `vitality_max` - Sinh lực tối đa

### **4. Cultivation Resources**
- `qi_current` - Khí hiện tại
- `qi_max` - Khí tối đa
- `cultivation_energy_current` - Tu luyện năng lượng
- `realm_energy_current` - Cảnh giới năng lượng
- `dao_energy_current` - Đạo năng lượng

## 🔧 **Usage Examples**

### **1. Basic Resource Calculation**

```go
// Create Resource Manager Subsystem
resourceSubsystem := NewResourceManagerSubsystem(config)

// Register with Actor Core
pluginRegistry.Register(resourceSubsystem)

// Get actor resources from Snapshot
snapshot := aggregator.Resolve(ctx, actor)
hpCurrent := snapshot.Primary["hp_current"]
hpMax := snapshot.Primary["hp_max"]
```

### **2. Resource Consumption**

```go
// Consume HP during combat
err := resourceConsumer.ConsumeResource(actor, "hp", 100.0)
if err != nil {
    log.Printf("Failed to consume HP: %v", err)
}

// Restore HP with potion
err := resourceConsumer.RestoreResource(actor, "hp", 50.0)
if err != nil {
    log.Printf("Failed to restore HP: %v", err)
}
```

### **3. Resource Modifiers**

```go
// Add cultivation modifier
modifier := &CultivationSystemModifier{
    SystemID:   "jindan_system",
    ResourceID: "hp",
    Modifier: &ResourceModifier{
        Type:  "multiplicative",
        Value: 1.5, // 50% increase
    },
}

err := resourceManager.RegisterCultivationModifier(modifier)
if err != nil {
    log.Printf("Failed to register modifier: %v", err)
}
```

## 📚 **Documentation**

- [00_Resource_Manager_Overview.md](00_Resource_Manager_Overview.md) - Tổng quan hệ thống
- [01_Resource_System_Design.md](01_Resource_System_Design.md) - Thiết kế chi tiết
- [02_Resource_Integration_Guide.md](02_Resource_Integration_Guide.md) - Hướng dẫn tích hợp

## 🚀 **Quick Start**

### **1. Installation**

```bash
# Clone repository
git clone <repository-url>
cd chaos-backend-service/docs/resource-manager

# Install dependencies
go mod tidy
```

### **2. Basic Setup**

```go
package main

import (
    "chaos-backend-service/docs/resource-manager"
    "chaos-backend-service/crates/actor-core/interfaces"
)

func main() {
    // Create resource config
    config := &ResourceConfig{
        Registry: &RegistryConfig{
            Resources: []*ResourceDefinition{
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
            },
        },
    }
    
    // Create Resource Manager Subsystem
    resourceSubsystem := NewResourceManagerSubsystem(config)
    
    // Register with Actor Core
    pluginRegistry := registry.NewPluginRegistry()
    err := pluginRegistry.Register(resourceSubsystem)
    if err != nil {
        log.Fatal(err)
    }
}
```

### **3. Resource Access**

```go
// Get actor resources
snapshot, err := aggregator.Resolve(ctx, actor)
if err != nil {
    log.Printf("Failed to resolve actor: %v", err)
    return
}

// Access resources
hpCurrent := snapshot.Primary["hp_current"]
hpMax := snapshot.Primary["hp_max"]
hpPercentage := snapshot.Derived["hp_percentage"]

log.Printf("HP: %.1f/%.1f (%.1f%%)", hpCurrent, hpMax, hpPercentage)
```

## 🔄 **Resource Regeneration**

### **1. Continuous Regeneration**

```go
// Process continuous regeneration
func (rm *ResourceManagerSubsystem) ProcessRegeneration(actor *Actor, deltaTime float64) {
    // HP regenerates continuously
    hpRegen := snapshot.Primary["hp_regen"] * deltaTime
    newHP := snapshot.Primary["hp_current"] + hpRegen
    
    // Cap at maximum
    if newHP > snapshot.Primary["hp_max"] {
        newHP = snapshot.Primary["hp_max"]
    }
    
    // Update HP
    rm.updateResource(actor, "hp_current", newHP)
}
```

### **2. Conditional Regeneration**

```go
// Process conditional regeneration
func (rm *ResourceManagerSubsystem) ProcessConditionalRegen(actor *Actor) {
    // Mana only regenerates when not in combat
    if !actor.IsInCombat() {
        manaRegen := snapshot.Primary["mana_regen"]
        newMana := snapshot.Primary["mana_current"] + manaRegen
        
        // Cap at maximum
        if newMana > snapshot.Primary["mana_max"] {
            newMana = snapshot.Primary["mana_max"]
        }
        
        // Update Mana
        rm.updateResource(actor, "mana_current", newMana)
    }
}
```

## 📊 **Performance**

### **1. Caching**
- **L1 Cache**: In-memory cache for hot data
- **L2 Cache**: Memory-mapped cache for warm data
- **L3 Cache**: Persistent cache for cold data

### **2. Batch Processing**
- Process multiple actors in parallel
- Group by priority for optimal performance
- Use worker pools for concurrent processing

### **3. Memory Optimization**
- Memory pooling for resource objects
- Lazy loading for resource calculations
- Garbage collection optimization

## 🧪 **Testing**

### **1. Unit Tests**

```bash
# Run unit tests
go test ./...

# Run with coverage
go test -cover ./...
```

### **2. Integration Tests**

```bash
# Run integration tests
go test -tags=integration ./...

# Run performance tests
go test -tags=performance ./...
```

### **3. Benchmark Tests**

```bash
# Run benchmark tests
go test -bench=. ./...

# Run specific benchmark
go test -bench=BenchmarkResourceCalculation ./...
```

## 🔧 **Configuration**

### **1. Resource Definitions**

```yaml
# resources.yaml
resources:
  - id: "hp"
    name: "Health Points"
    category: "health"
    type: "current"
    base_value: 100.0
    min_value: 0.0
    max_value: 10000.0
    regen_rate: 1.0
    regen_type: "continuous"
    
  - id: "mana"
    name: "Mana"
    category: "energy"
    type: "current"
    base_value: 50.0
    min_value: 0.0
    max_value: 5000.0
    regen_rate: 2.0
    regen_type: "continuous"
```

### **2. Modifier Configuration**

```yaml
# modifiers.yaml
modifiers:
  - id: "jindan_hp_modifier"
    resource_id: "hp"
    type: "multiplicative"
    value: 1.5
    condition:
      type: "realm"
      operator: ">="
      value: "foundation"
    priority: 100
    system: "jindan_system"
```

## 🚀 **Roadmap**

### **Phase 1: Core System** ✅
- [x] Resource Registry
- [x] Resource Calculator
- [x] Resource Subsystem
- [x] Basic Resources (HP, Mana, Stamina)

### **Phase 2: Advanced Features** 🚧
- [ ] Resource Modifiers
- [ ] Resource Dependencies
- [ ] Resource Conflicts
- [ ] Resource Regeneration

### **Phase 3: Cultivation Integration** 📋
- [ ] Cultivation Resources
- [ ] Realm-based Resources
- [ ] Advanced Regeneration
- [ ] Resource Events

### **Phase 4: Performance & Optimization** 📋
- [ ] Multi-layer Caching
- [ ] Batch Processing
- [ ] Memory Optimization
- [ ] Performance Monitoring

## 🤝 **Contributing**

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 **Support**

- **Documentation**: [docs/](docs/)
- **Issues**: [GitHub Issues](https://github.com/your-org/resource-manager/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/resource-manager/discussions)

---

*Resource Manager System - A flexible and performant resource management subsystem for Actor Core v3.*

## 📦 Examples & Golden Vectors

- Combiner/Cap configs (for Actor Core integration via ACTOR_CORE_CONFIG_DIR):
  - `docs/resource-manager/configs/combiner.resources.yaml`
  - `docs/resource-manager/configs/cap_layers.resources.yaml`
- Golden vectors (core scenarios):
  - `golden_vectors/case01_damage_and_heal_same_tick/`
  - `golden_vectors/case02_ooc_regen/`
  - `golden_vectors/case03_shield_decay/`
  - `golden_vectors/case04_offline_catchup/`