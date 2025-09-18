# Condition Core Overview

## 📋 **Tổng Quan**

Condition Core là hệ thống trung tâm quản lý tất cả các conditions trong game, được thiết kế dựa trên Skyrim's Condition Functions system với hơn 100 condition functions và logic phức tạp. Hệ thống này đóng vai trò cầu nối thống nhất giữa tất cả các systems trong Chaos World.

## 🎯 **Tại Sao Cần Condition Core?**

### **1. Vấn Đề Hiện Tại**

```
Current Condition Usage Across Systems
├── Action-Core
│   ├── Action Conditions (execution, resource, timing)
│   └── Target Conditions (validity, range, type)
├── Status-Core
│   ├── Status Conditions (application, duration, interaction)
│   └── Immunity Conditions (resistance, immunity, break)
├── Element-Core
│   ├── Element Conditions (mastery, resistance, interaction)
│   └── Derived Stat Conditions (calculation, scaling)
└── Future Systems
    ├── Talent-Core (talent conditions)
    ├── Perk-Core (perk conditions)
    └── Skill-Core (skill conditions)
```

**Vấn đề:**
- 🔴 **Condition Duplication**: Cùng một condition được implement ở nhiều nơi
- 🔴 **Inconsistent Logic**: Logic condition không nhất quán
- 🔴 **Hard to Maintain**: Khó maintain và update conditions
- 🔴 **Performance Issues**: Mỗi system phải implement riêng

### **2. Lợi Ích của Condition Core**

```
Condition Core Benefits
├── Unified Condition System
│   ├── Single source of truth
│   ├── Consistent logic
│   └── Centralized validation
├── Cross-System Sharing
│   ├── Reuse conditions across systems
│   ├── Consistent behavior
│   └── Easy maintenance
├── Multiple Configuration Methods
│   ├── YAML string-based
│   ├── Class/interface-based
│   └── Hybrid approach
└── Performance Optimization
    ├── Centralized caching
    ├── Optimized evaluation
    └── Batch processing
```

## 🏗️ **Kiến Trúc Condition Core**

### **Core Components**

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
│   └── Condition Cache
├── Configuration System
│   ├── YAML Configuration
│   ├── Interface Configuration
│   ├── Hybrid Configuration
│   └── Configuration Manager
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

## 🎮 **Skyrim Condition System Analysis**

### **1. Skyrim Condition Functions Categories**

Skyrim có hơn 100 condition functions được chia thành các categories:

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

### **2. Skyrim Condition Logic Examples**

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
  
  # Metadata
  metadata:
    description: "Check if actor health is below 50%"
    description_vi: "Kiểm tra máu diễn viên dưới 50%"
    category: "actor"
    cacheable: true
    cache_ttl: 30.0
    performance_impact: "low"
```

### **2. Class/Interface-based Configuration**

```rust
// Class-based Condition Implementation
pub struct HealthCondition {
    threshold: f64,
    operator: ConditionOperator,
}

impl HealthCondition {
    pub fn new(threshold: f64, operator: ConditionOperator) -> Self {
        Self { threshold, operator }
    }
}

impl ConditionInterface for HealthCondition {
    fn get_condition_id(&self) -> String {
        "health_condition".to_string()
    }
    
    fn get_condition_function(&self) -> String {
        "get_actor_value".to_string()
    }
    
    fn get_condition_parameters(&self) -> Vec<ConditionParameter> {
        vec![ConditionParameter::String("health".to_string())]
    }
    
    fn get_condition_operator(&self) -> ConditionOperator {
        self.operator.clone()
    }
    
    fn get_condition_value(&self) -> ConditionValue {
        ConditionValue::Float(self.threshold)
    }
    
    fn get_condition_logic(&self) -> ConditionLogic {
        ConditionLogic::And
    }
    
    fn get_condition_priority(&self) -> u32 {
        100
    }
    
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
    pub fallback_strategy: FallbackStrategy,
}

impl HybridCondition {
    // Create from YAML with interface fallback
    pub fn from_yaml_with_fallback(
        yaml: &str,
        fallback_interface: Box<dyn ConditionInterface>
    ) -> Result<Self, ConditionError> {
        let yaml_config = match serde_yaml::from_str::<YamlCondition>(yaml) {
            Ok(config) => Some(config),
            Err(_) => None,
        };
        
        Ok(Self {
            yaml_config,
            interface_config: Some(fallback_interface),
            evaluation_strategy: EvaluationStrategy::YamlFirst,
            fallback_strategy: FallbackStrategy::Interface,
        })
    }
    
    // Evaluate condition
    pub async fn evaluate(&self, context: &ConditionContext) -> Result<bool, ConditionError> {
        match self.evaluation_strategy {
            EvaluationStrategy::YamlFirst => {
                if let Some(yaml_config) = &self.yaml_config {
                    return self.evaluate_yaml(yaml_config, context).await;
                }
                if let Some(interface_config) = &self.interface_config {
                    return interface_config.evaluate(context);
                }
                Err(ConditionError::NoConfiguration)
            },
            EvaluationStrategy::InterfaceFirst => {
                if let Some(interface_config) = &self.interface_config {
                    return interface_config.evaluate(context);
                }
                if let Some(yaml_config) = &self.yaml_config {
                    return self.evaluate_yaml(yaml_config, context).await;
                }
                Err(ConditionError::NoConfiguration)
            },
            EvaluationStrategy::Hybrid => {
                // Try both and combine results
                let yaml_result = if let Some(yaml_config) = &self.yaml_config {
                    Some(self.evaluate_yaml(yaml_config, context).await?)
                } else {
                    None
                };
                
                let interface_result = if let Some(interface_config) = &self.interface_config {
                    Some(interface_config.evaluate(context)?)
                } else {
                    None
                };
                
                match (yaml_result, interface_result) {
                    (Some(yaml), Some(interface)) => {
                        // Both available - use AND logic
                        Ok(yaml && interface)
                    },
                    (Some(yaml), None) => Ok(yaml),
                    (None, Some(interface)) => Ok(interface),
                    (None, None) => Err(ConditionError::NoConfiguration),
                }
            },
        }
    }
}
```

## 🚀 **Integration với Existing Systems**

### **1. Action Core Integration**

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
    
    // Register interface-based condition
    pub fn register_interface_condition<T: ConditionInterface + 'static>(
        &self,
        condition: T
    ) -> Result<(), ActionError> {
        self.condition_core
            .register_interface_condition(condition)?;
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

### **2. Status Core Integration**

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
    
    // Evaluate status conditions
    pub async fn evaluate_status_conditions(
        &self,
        status_effect: &StatusEffect,
        context: &StatusContext
    ) -> Result<bool, StatusError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition in &status_effect.conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}
```

### **3. Element Core Integration**

```rust
// Element Core using Condition Core
use chaos_condition_core::{ConditionCore, ConditionInterface};

pub struct ElementCore {
    condition_core: Arc<ConditionCore>,
    // ... other fields
}

impl ElementCore {
    // Register element-specific conditions
    pub fn register_element_conditions(&self) -> Result<(), ElementError> {
        // Register mastery conditions
        self.condition_core.register_interface_condition(
            MasteryLevelCondition::new(100, ConditionOperator::GreaterThanOrEqual)
        )?;
        
        // Register resistance conditions
        self.condition_core.register_interface_condition(
            ResistanceLevelCondition::new(0.5, ConditionOperator::LessThan)
        )?;
        
        // Register interaction conditions
        self.condition_core.register_interface_condition(
            ElementInteractionCondition::new("fire", "water", InteractionType::Suppress)
        )?;
        
        Ok(())
    }
    
    // Evaluate element conditions
    pub async fn evaluate_element_conditions(
        &self,
        element_id: &str,
        context: &ElementContext
    ) -> Result<bool, ElementError> {
        let condition_context = self.convert_to_condition_context(context);
        
        let condition_ids = self.get_element_condition_ids(element_id);
        
        for condition_id in condition_ids {
            let result = self.condition_core
                .evaluate_condition(&condition_id, &condition_context)
                .await?;
            
            if !result {
                return Ok(false);
            }
        }
        
        Ok(true)
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

impl CentralizedConditionCache {
    // Cache condition result
    pub fn cache_condition_result(
        &self,
        condition_id: &str,
        context_key: &str,
        result: ConditionResult,
        ttl: Duration
    ) {
        let cache_key = format!("{}:{}", condition_id, context_key);
        let cached_result = CachedConditionResult {
            result,
            cached_at: SystemTime::now(),
            ttl,
        };
        
        self.condition_cache.insert(cache_key, cached_result);
    }
    
    // Get cached condition result
    pub fn get_condition_result(
        &self,
        condition_id: &str,
        context_key: &str
    ) -> Option<ConditionResult> {
        let cache_key = format!("{}:{}", condition_id, context_key);
        
        if let Some(cached_result) = self.condition_cache.get(&cache_key) {
            if cached_result.is_valid() {
                return Some(cached_result.result.clone());
            } else {
                self.condition_cache.remove(&cache_key);
            }
        }
        
        None
    }
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
        let mut results = Vec::new();
        let mut batch = Vec::new();
        
        for condition_id in conditions {
            batch.push(condition_id.clone());
            
            if batch.len() >= self.batch_size {
                let batch_results = self.evaluate_batch(&batch, context).await?;
                results.extend(batch_results);
                batch.clear();
            }
        }
        
        if !batch.is_empty() {
            let batch_results = self.evaluate_batch(&batch, context).await?;
            results.extend(batch_results);
        }
        
        Ok(results)
    }
    
    // Evaluate single batch
    async fn evaluate_batch(
        &self,
        condition_ids: &[String],
        context: &ConditionContext
    ) -> Result<Vec<ConditionResult>, ConditionError> {
        let mut tasks = Vec::new();
        
        for condition_id in condition_ids {
            let task = self.condition_core.evaluate_condition(condition_id, context);
            tasks.push(task);
        }
        
        let results = futures::future::join_all(tasks).await;
        let mut condition_results = Vec::new();
        
        for result in results {
            condition_results.push(result?);
        }
        
        Ok(condition_results)
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
- **Easy Migration**: Dễ dàng migrate giữa methods

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

## 📝 **Implementation Strategy**

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

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Overview Complete  
**Maintainer**: Chaos World Team
