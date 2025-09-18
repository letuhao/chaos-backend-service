# Condition Core Documentation

## 📋 **Tổng Quan**

Condition Core là hệ thống trung tâm quản lý tất cả các conditions trong game, được thiết kế dựa trên Skyrim's Condition Functions system với hơn 100 condition functions và logic phức tạp. Hệ thống này đóng vai trò cầu nối thống nhất giữa tất cả các systems trong Chaos World.

## 🎯 **Tại Sao Cần Condition Core?**

### **Vấn Đề Hiện Tại**
- 🔴 **Condition Duplication**: Cùng một condition được implement ở nhiều nơi
- 🔴 **Inconsistent Logic**: Logic condition không nhất quán
- 🔴 **Hard to Maintain**: Khó maintain và update conditions
- 🔴 **Performance Issues**: Mỗi system phải implement riêng

### **Lợi Ích của Condition Core**
- ✅ **Unified System**: Single source of truth cho tất cả conditions
- ✅ **Cross-System Sharing**: Tái sử dụng conditions across systems
- ✅ **Multiple Config Methods**: YAML + Class/Interface support
- ✅ **Performance Optimization**: Centralized caching và batch evaluation

## 🏗️ **Kiến Trúc Condition Core**

```
Condition Core
├── Condition Registry
│   ├── Condition Function Registry
│   ├── Condition Template Registry
│   ├── Condition Category Registry
│   └── Condition Metadata Registry
├── Condition Engine
│   ├── Condition Evaluator
│   ├── Condition Parser
│   ├── Condition Optimizer
│   └── Condition Scheduler
├── Configuration System
│   ├── YAML Configuration
│   ├── Interface Configuration
│   ├── Hybrid Configuration
│   └── Plugin Configuration
├── Condition Functions
│   ├── Actor Functions (25+ functions)
│   ├── Item Functions (15+ functions)
│   ├── Location Functions (20+ functions)
│   ├── Time Functions (10+ functions)
│   ├── Weather Functions (8+ functions)
│   ├── Magic Functions (15+ functions)
│   ├── Relationship Functions (12+ functions)
│   └── Custom Functions (10+ functions)
└── Integration Bridges
    ├── Action Core Bridge
    ├── Status Core Bridge
    ├── Element Core Bridge
    ├── Effect Core Bridge
    ├── Talent Core Bridge
    └── Perk Core Bridge
```

## 📚 **Tài Liệu Thiết Kế**

### **1. Core Design Documents**

- **[00_Condition_Core_Overview.md](00_Condition_Core_Overview.md)**
  - Tổng quan về Condition Core
  - Tại sao cần Condition Core
  - Kiến trúc tổng thể
  - Skyrim Condition System Analysis
  - Multiple Configuration Support
  - Integration với Existing Systems
  - Performance Benefits

- **[01_Condition_Core_Architecture_Design.md](01_Condition_Core_Architecture_Design.md)**
  - Kiến trúc chi tiết Condition Core
  - Core Components
  - Condition Definition Structure
  - Condition Function Registry
  - Condition Functions Implementation
  - Condition Evaluation Engine
  - Performance Monitoring

- **[02_Condition_Core_Function_Registry_Design.md](02_Condition_Core_Function_Registry_Design.md)**
  - Function Registry Architecture
  - Skyrim-Inspired Function Categories
  - Function Implementation Examples
  - Function Registry Management
  - Function Validation

- **[03_Condition_Core_Configuration_System_Design.md](03_Condition_Core_Configuration_System_Design.md)**
  - Configuration Architecture
  - YAML Configuration System
  - Interface Configuration System
  - Hybrid Configuration System
  - Plugin Configuration System
  - Configuration Validation

- **[04_Condition_Core_Cache_System_Design.md](04_Condition_Core_Cache_System_Design.md)**
  - Multi-Level Cache Architecture
  - Cache Key Generation Strategy
  - Cache Lookup Strategy
  - Cache Validation Strategy
  - Cache TTL Strategy
  - Cache Preloading Strategy
  - Cache Eviction Strategy
  - Cache Monitoring & Debugging

### **2. Implementation Documents**

- **[05_Condition_Core_Implementation_Guide.md](05_Condition_Core_Implementation_Guide.md)**
  - Step-by-step implementation guide
  - Code examples và best practices
  - Testing strategies
  - Performance optimization
  - Integration guidelines

- **[06_Condition_Core_API_Design.md](06_Condition_Core_API_Design.md)**
  - Public API design
  - Internal API design
  - Event API design
  - Admin API design
  - API documentation

### **3. Integration Documents**

- **[07_Condition_Core_Integration_Design.md](07_Condition_Core_Integration_Design.md)**
  - Integration với Action Core
  - Integration với Status Core
  - Integration với Element Core
  - Integration với Effect Core
  - Integration với Talent Core
  - Integration với Perk Core

- **[08_Condition_Core_Performance_Design.md](08_Condition_Core_Performance_Design.md)**
  - Performance optimization strategies
  - Caching mechanisms
  - Batch processing
  - Memory optimization
  - Performance monitoring

### **4. Testing Documents**

- **[09_Condition_Core_Testing_Strategy.md](09_Condition_Core_Testing_Strategy.md)**
  - Unit testing strategy
  - Integration testing strategy
  - Performance testing strategy
  - Load testing strategy
  - Test automation

## 🎮 **Skyrim-Inspired Design**

### **Condition Functions Categories**

```
Skyrim Condition Functions
├── Actor Conditions (25+ functions)
│   ├── GetActorValue (Health, Magicka, Stamina, etc.)
│   ├── GetLevel, GetRace, GetSex
│   ├── IsInCombat, IsDead, IsUnconscious
│   ├── GetActorValuePercentage
│   └── GetActorValueMax
├── Item Conditions (15+ functions)
│   ├── HasItem, GetItemCount, IsEquipped
│   ├── GetItemCharge, GetItemHealth
│   ├── IsWorn, IsWornHasKeyword
│   └── GetEquippedItemType
├── Location Conditions (20+ functions)
│   ├── GetInCurrentLocation, GetInCurrentLocType
│   ├── IsInInterior, IsInWater, IsInAir
│   ├── GetDistanceFromPlayer, GetDistanceFromRef
│   └── GetLocationCleared
├── Time Conditions (10+ functions)
│   ├── GetCurrentTime, GetDayOfWeek, GetSeason
│   ├── IsDay, IsNight, IsSunrise, IsSunset
│   └── GetGameHour
├── Weather Conditions (8+ functions)
│   ├── GetCurrentWeather, IsRaining, IsSnowing
│   ├── IsStorming, IsFoggy, IsCloudy
│   └── GetWeatherTransition
├── Magic Conditions (15+ functions)
│   ├── HasMagicEffect, GetMagicEffectMagnitude
│   ├── HasSpell, GetSpellCount, IsSpellTarget
│   ├── GetMagicEffectDuration, GetMagicEffectTimeLeft
│   └── HasPerk, GetPerkCount
├── Relationship Conditions (12+ functions)
│   ├── GetRelationshipRank, IsHostileToActor
│   ├── IsFriendlyToActor, IsNeutralToActor
│   ├── GetFactionRank, IsInFaction
│   └── GetCrimeGold, GetCrimeGoldViolent
└── Custom Conditions (10+ functions)
    ├── GetGlobalValue, SetGlobalValue
    ├── GetQuestCompleted, GetQuestStage
    ├── GetEventData, GetEventDataString
    └── GetRandomPercent
```

### **Complex Condition Logic**

```javascript
// Skyrim Condition Examples
// Example 1: Complex Fire Damage Condition
if (GetActorValue Health < 0.5) AND 
   (IsInCombat == 1) AND 
   (HasMagicEffect FireResist == 0) AND
   (GetCurrentWeather == 0) AND
   (GetDistanceFromPlayer < 1000)
then
   ApplyEffect FireDamage

// Example 2: Healing Potion Condition
if (GetActorValue Health < 0.8) AND
   (HasItem HealthPotion > 0) AND
   (IsInCombat == 0) AND
   (GetCurrentTime > 6.0) AND
   (GetCurrentTime < 18.0)
then
   ApplyEffect HealthHealing

// Example 3: Weather-based Effect
if (IsRaining == 1) AND
   (HasMagicEffect WaterMastery > 0) AND
   (GetInCurrentLocation == "Forest")
then
   ApplyEffect WaterAmplification
```

## 🔧 **Multiple Configuration Support**

### **1. YAML String-based Configuration**

```yaml
# YAML Condition Configuration
condition_definition:
  condition_id: "health_condition"
  condition_function: "get_actor_value"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "health"
  condition_operator: "less_than"
  condition_value:
    value_type: "float"
    value: 0.5
  condition_logic: "AND"
  condition_priority: 100
```

### **2. Class/Interface-based Configuration**

```rust
// Class-based Condition Implementation
pub struct HealthCondition {
    threshold: f64,
    operator: ConditionOperator,
}

impl ConditionInterface for HealthCondition {
    fn evaluate(&self, context: &ConditionContext) -> Result<bool, ConditionError> {
        let health_value = context.get_actor_value("health")?;
        Ok(self.operator.compare(health_value, self.threshold))
    }
}
```

### **3. Hybrid Configuration Support**

```rust
// Hybrid Condition Configuration
pub struct HybridCondition {
    pub yaml_config: Option<YamlCondition>,
    pub interface_config: Option<Box<dyn ConditionInterface>>,
    pub evaluation_strategy: EvaluationStrategy,
}
```

## 🚀 **Integration với Existing Systems**

### **Action Core Integration**

```rust
// Action Core using Condition Core
use chaos_condition_core::{ConditionCore, YamlCondition, ConditionInterface};

pub struct ActionCore {
    condition_core: Arc<ConditionCore>,
    // ... other fields
}

impl ActionCore {
    // Load conditions from YAML
    pub async fn load_conditions_from_yaml(&self, yaml: &str) -> Result<(), ActionError> {
        let conditions = self.condition_core
            .parse_yaml_conditions(yaml)
            .await?;
        
        for condition in conditions {
            self.register_action_condition(condition).await?;
        }
        
        Ok(())
    }
    
    // Evaluate action conditions
    pub async fn evaluate_action_conditions(
        &self,
        action: &Action,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition_id in &action.condition_ids {
            let result = self.condition_core
                .evaluate_condition(condition_id, &condition_context)
                .await?;
            
            if !result {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}
```

### **Status Core Integration**

```rust
// Status Core using Condition Core
use chaos_condition_core::{ConditionCore, HybridCondition};

pub struct StatusCore {
    condition_core: Arc<ConditionCore>,
    // ... other fields
}

impl StatusCore {
    // Load hybrid conditions
    pub async fn load_hybrid_conditions(&self, config: &StatusConfig) -> Result<(), StatusError> {
        for condition_config in &config.conditions {
            let hybrid_condition = HybridCondition::from_yaml_with_fallback(
                &condition_config.yaml_config,
                condition_config.interface_config.clone()
            )?;
            
            self.condition_core
                .register_hybrid_condition(condition_config.id.clone(), hybrid_condition)
                .await?;
        }
        
        Ok(())
    }
}
```

## 📊 **Performance Benefits**

### **1. Centralized Caching**

```rust
// Centralized Condition Cache
pub struct CentralizedConditionCache {
    condition_cache: DashMap<String, CachedConditionResult>,
    function_cache: DashMap<String, CachedFunctionResult>,
    evaluation_cache: DashMap<String, CachedEvaluationResult>,
    cache_metrics: CacheMetrics,
}
```

### **2. Batch Evaluation**

```rust
// Batch Condition Evaluation
pub struct BatchConditionEvaluator {
    condition_core: Arc<ConditionCore>,
    batch_size: usize,
    evaluation_strategies: HashMap<String, EvaluationStrategy>,
}

impl BatchConditionEvaluator {
    // Evaluate multiple conditions in batch
    pub async fn evaluate_conditions_batch(
        &self,
        conditions: &[String],
        context: &ConditionContext
    ) -> Result<Vec<ConditionResult>, ConditionError> {
        // Batch evaluation logic
    }
}
```

## 🎯 **Key Features**

### **1. Skyrim-Inspired Design**
- **100+ Condition Functions**: Tương tự Skyrim's Condition Functions
- **Complex Logic**: AND, OR, NOT, XOR, NAND, NOR logic
- **Multiple Categories**: Actor, Item, Location, Time, Weather, Magic, Relationship
- **Performance Optimization**: Caching và async evaluation

### **2. Multiple Configuration Methods**
- **YAML String-based**: Configuration linh hoạt
- **Class/Interface-based**: Type-safe configuration
- **Hybrid Approach**: Kết hợp cả hai methods
- **Plugin System**: Skyrim-inspired plugin system

### **3. Cross-System Integration**
- **Unified API**: Single API cho tất cả systems
- **Consistent Behavior**: Hành vi nhất quán
- **Easy Integration**: Dễ dàng tích hợp
- **Performance Optimization**: Tối ưu performance

### **4. Future-Proof Design**
- **Extensible Architecture**: Dễ dàng extend
- **Plugin Support**: Hỗ trợ plugin system
- **Version Control**: Hỗ trợ versioning
- **Migration Support**: Hỗ trợ migration

## 📝 **Implementation Plan**

### **Phase 1: Foundation (2 weeks)**
1. **Create Condition Core Structure**
   - Condition Registry
   - Condition Engine
   - Basic Condition Functions

2. **Implement Configuration System**
   - YAML Configuration
   - Interface Configuration
   - Hybrid Configuration

### **Phase 2: Core Functions (2 weeks)**
1. **Implement Condition Functions**
   - Actor Functions (25+ functions)
   - Item Functions (15+ functions)
   - Location Functions (20+ functions)
   - Time Functions (10+ functions)
   - Weather Functions (8+ functions)
   - Magic Functions (15+ functions)
   - Relationship Functions (12+ functions)
   - Custom Functions (10+ functions)

### **Phase 3: Integration (2 weeks)**
1. **Integration Bridges**
   - Action Core Bridge
   - Status Core Bridge
   - Element Core Bridge
   - Effect Core Bridge

2. **System Integration**
   - Update existing systems
   - Migrate existing conditions
   - Test integration

### **Phase 4: Advanced Features (2 weeks)**
1. **Performance Optimization**
   - Centralized caching
   - Batch evaluation
   - Performance monitoring

2. **Advanced Features**
   - Complex condition logic
   - Plugin system
   - Hot reload support

## 🎯 **Documentation Status**

### **✅ Complete (9/9 Documents):**

1. **✅ 00_Condition_Core_Overview.md** - Complete
2. **✅ 01_Condition_Core_Architecture_Design.md** - Complete  
3. **✅ 02_Condition_Core_Function_Registry_Design.md** - Complete
4. **✅ 03_Condition_Core_Configuration_System_Design.md** - Complete
5. **✅ 04_Condition_Core_Cache_System_Design.md** - Complete
6. **✅ 05_Condition_Core_Implementation_Guide.md** - Complete
7. **✅ 06_Condition_Core_API_Design.md** - Complete
8. **✅ 07_Condition_Core_Integration_Design.md** - Complete
9. **✅ 08_Condition_Core_Performance_Design.md** - Complete
10. **✅ 09_Condition_Core_Testing_Strategy.md** - Complete

### **📊 Progress: 100% Complete (9/9)**

**🎉 Condition Core Documentation is COMPLETE!**

## 🔗 **Related Systems**

- **[Action Core](../action-core/README.md)**: Action system integration
- **[Status Core](../status-core/README.md)**: Status effect system integration
- **[Element Core](../element-core/README.md)**: Elemental system integration
- **[Effect Core](../effect-core/README.md)**: Effect system integration
- **[Talent Core](../talent-core/README.md)**: Talent system integration
- **[Perk Core](../perk-core/README.md)**: Perk system integration

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Documentation Complete  
**Maintainer**: Chaos World Team
