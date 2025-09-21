# Condition Core - Element Core Integration

## 📋 **Tổng Quan**

Tài liệu này mô tả chi tiết cách Condition Core tích hợp với Element Core để cung cấp các condition functions chuẩn hóa cho tất cả các systems trong Chaos World. Thông qua integration này, tất cả các systems có thể sử dụng cùng một bộ condition functions để đánh giá các điều kiện liên quan đến elements.

## 🎯 **Mục Đích Integration**

### **Vấn Đề Hiện Tại**
- 🔴 **Scattered Element Conditions**: Các condition liên quan đến element được implement rải rác trong nhiều systems
- 🔴 **Inconsistent Logic**: Logic đánh giá element conditions không nhất quán
- 🔴 **Code Duplication**: Cùng một condition được implement nhiều lần
- 🔴 **Hard to Maintain**: Khó maintain và update element conditions

### **Lợi Ích của Integration**
- ✅ **Centralized Element Conditions**: Tất cả element conditions được quản lý tập trung
- ✅ **Standardized Functions**: Các function chuẩn hóa cho element conditions
- ✅ **Cross-System Reuse**: Tái sử dụng element conditions across systems
- ✅ **Consistent Behavior**: Hành vi nhất quán cho tất cả systems

## 🏗️ **Element Core Condition Functions**

### **1. Element Mastery Conditions**

```rust
/// Get element mastery level
pub struct GetElementMasteryFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Check if actor has element affinity
pub struct HasElementAffinityFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Check if actor has element weakness
pub struct HasElementWeaknessFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Get element resistance value
pub struct GetElementResistanceFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}
```

### **2. Element Interaction Conditions**

```rust
/// Get element interaction type
pub struct GetElementInteractionFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Check if elements are in same category
pub struct IsElementSameCategoryFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Check if elements are in generating relationship
pub struct IsElementGeneratingFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Check if elements are in overcoming relationship
pub struct IsElementOvercomingFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}
```

### **3. Element Status Conditions**

```rust
/// Check if actor has element status effect
pub struct HasElementStatusEffectFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Get element status effect count
pub struct GetElementStatusEffectCountFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Check if element status effect is active
pub struct IsElementStatusEffectActiveFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}
```

### **4. Element Resource Conditions**

```rust
/// Check if actor has element resource
pub struct HasElementResourceFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Get element resource value
pub struct GetElementResourceValueFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Check if element resource is below threshold
pub struct IsElementResourceBelowThresholdFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Check if element resource is above threshold
pub struct IsElementResourceAboveThresholdFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}
```

### **5. Element Hybrid Conditions**

```rust
/// Check if actor has hybrid element
pub struct HasHybridElementFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Check if hybrid element is activated
pub struct IsHybridElementActivatedFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

/// Get hybrid element parents
pub struct GetHybridElementParentsFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}
```

## 🔧 **Element Data Provider Interface**

### **Extended Element Data Provider**

```rust
/// Extended trait for providing element data to Condition Core
#[async_trait::async_trait]
pub trait ElementDataProvider: Send + Sync {
    // Basic element functions
    async fn get_element_mastery(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn get_element_resistance(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn has_element_affinity(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn is_element_weakness(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String>;
    async fn list_elements(&self) -> ConditionResult<Vec<String>>;
    
    // Element status functions
    async fn has_element_status_effect(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_element_status_effect_count(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<i64>;
    async fn is_element_status_effect_active(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<bool>;
    
    // Element resource functions
    async fn has_element_resource(&self, element_id: &str, resource_type: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_element_resource_value(&self, element_id: &str, resource_type: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn is_element_resource_below_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool>;
    async fn is_element_resource_above_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool>;
    
    // Element interaction functions
    async fn is_element_same_category(&self, element1: &str, element2: &str) -> ConditionResult<bool>;
    async fn is_element_generating(&self, source_element: &str, target_element: &str) -> ConditionResult<bool>;
    async fn is_element_overcoming(&self, source_element: &str, target_element: &str) -> ConditionResult<bool>;
    async fn is_element_neutral(&self, source_element: &str, target_element: &str) -> ConditionResult<bool>;
    
    // Hybrid element functions
    async fn has_hybrid_element(&self, hybrid_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn is_hybrid_element_activated(&self, hybrid_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_hybrid_element_parents(&self, hybrid_id: &str) -> ConditionResult<Vec<String>>;
    async fn list_hybrid_elements(&self) -> ConditionResult<Vec<String>>;
    
    // Element derived stats functions
    async fn get_element_derived_stat(&self, element_id: &str, stat_name: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn has_element_derived_stat(&self, element_id: &str, stat_name: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn list_element_derived_stats(&self, element_id: &str) -> ConditionResult<Vec<String>>;
}
```

## 📊 **Element Condition Examples**

### **1. Combat Core Integration**

```yaml
# Fire spell condition
fire_spell_condition:
  condition_id: "can_cast_fire_spell"
  function_name: "get_element_mastery"
  operator: "GreaterThanOrEqual"
  value:
    value_type: "float"
    value: 100.0
  parameters:
    - parameter_type: "string"
      parameter_value: "fire"
  condition_logic: "AND"
  condition_priority: 100

# Water resistance condition
water_resistance_condition:
  condition_id: "has_water_resistance"
  function_name: "get_element_resistance"
  operator: "GreaterThan"
  value:
    value_type: "float"
    value: 50.0
  parameters:
    - parameter_type: "string"
      parameter_value: "water"
  condition_logic: "AND"
  condition_priority: 80
```

### **2. Shield System Integration**

```yaml
# Fire shield activation condition
fire_shield_activation:
  condition_id: "can_activate_fire_shield"
  function_name: "has_element_affinity"
  operator: "Equal"
  value:
    value_type: "boolean"
    value: true
  parameters:
    - parameter_type: "string"
      parameter_value: "fire"
  condition_logic: "AND"
  condition_priority: 90

# Element interaction condition
element_interaction_condition:
  condition_id: "fire_vs_water_interaction"
  function_name: "is_element_overcoming"
  operator: "Equal"
  value:
    value_type: "boolean"
    value: true
  parameters:
    - parameter_type: "string"
      parameter_value: "fire"
    - parameter_type: "string"
      parameter_value: "water"
  condition_logic: "AND"
  condition_priority: 70
```

### **3. Status Effect Integration**

```yaml
# Element status effect condition
element_status_effect_condition:
  condition_id: "has_fire_burning_effect"
  function_name: "has_element_status_effect"
  operator: "Equal"
  value:
    value_type: "boolean"
    value: true
  parameters:
    - parameter_type: "string"
      parameter_value: "fire"
    - parameter_type: "string"
      parameter_value: "burning"
  condition_logic: "AND"
  condition_priority: 60
```

## 🔄 **Integration Flow**

### **1. Element Core as Data Provider**

```rust
// Element Core implements ElementDataProvider
impl ElementDataProvider for ElementCore {
    async fn get_element_mastery(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64> {
        // Get element mastery from Element Core
        self.get_actor_element_mastery(actor_id, element_id).await
    }
    
    async fn has_element_affinity(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool> {
        // Check element affinity from Element Core
        self.check_actor_element_affinity(actor_id, element_id).await
    }
    
    // ... implement other methods
}
```

### **2. Condition Core Registration**

```rust
// Register Element Core as data provider
let mut data_registry = DataProviderRegistry::new();
data_registry.register_element_provider(Box::new(element_core));

// Create function registry with element functions
let function_registry = create_function_registry_with_providers(&data_registry);
```

### **3. System Integration**

```rust
// Combat Core using Element Core conditions
impl CombatCore {
    pub async fn can_cast_element_spell(
        &self,
        actor_id: &str,
        element_type: &str,
        spell_id: &str,
    ) -> Result<bool, CombatError> {
        let condition = ConditionConfig {
            condition_id: format!("can_cast_{}_spell", element_type),
            function_name: "get_element_mastery".to_string(),
            operator: ConditionOperator::GreaterThanOrEqual,
            value: ConditionValue::Float(100.0),
            parameters: vec![ConditionParameter::String(element_type.to_string())],
        };
        
        let context = ConditionContext {
            target: ActorTarget { id: actor_id.to_string() },
            world_id: self.world_id.clone(),
            current_time: SystemTime::now(),
            current_weather: WeatherType::Clear,
            world_state: self.get_world_state().await?,
        };
        
        self.condition_resolver.resolve_condition(&condition, &context).await
    }
}
```

## 📈 **Performance Benefits**

### **1. Centralized Caching**

```rust
// Element conditions are cached centrally
pub struct ElementConditionCache {
    mastery_cache: DashMap<String, CachedElementMastery>,
    affinity_cache: DashMap<String, CachedElementAffinity>,
    interaction_cache: DashMap<String, CachedElementInteraction>,
}
```

### **2. Batch Evaluation**

```rust
// Batch evaluate multiple element conditions
pub struct ElementConditionBatchEvaluator {
    condition_core: Arc<ConditionCore>,
    element_core: Arc<ElementCore>,
}

impl ElementConditionBatchEvaluator {
    pub async fn evaluate_element_conditions_batch(
        &self,
        conditions: &[String],
        actor_id: &str,
    ) -> Result<Vec<ConditionResult>, ConditionError> {
        // Batch evaluation logic
    }
}
```

## 🧪 **Testing Strategy**

### **1. Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_element_mastery() {
        let element_core = create_test_element_core();
        let function = GetElementMasteryFunction::new(Some(Arc::new(element_core)));
        
        let parameters = vec![ConditionParameter::String("fire".to_string())];
        let context = create_test_context();
        
        let result = function.evaluate(&parameters, &context).await.unwrap();
        assert!(matches!(result, ConditionValue::Float(_)));
    }
    
    #[tokio::test]
    async fn test_has_element_affinity() {
        let element_core = create_test_element_core();
        let function = HasElementAffinityFunction::new(Some(Arc::new(element_core)));
        
        let parameters = vec![ConditionParameter::String("water".to_string())];
        let context = create_test_context();
        
        let result = function.evaluate(&parameters, &context).await.unwrap();
        assert!(matches!(result, ConditionValue::Boolean(_)));
    }
}
```

### **2. Integration Tests**

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_combat_core_element_integration() {
        let combat_core = create_test_combat_core_with_element_conditions();
        
        let can_cast = combat_core.can_cast_element_spell("actor_1", "fire", "fireball").await.unwrap();
        assert!(can_cast);
    }
}
```

## 🎯 **Implementation Priority**

### **Phase 1: Core Element Functions (1 week)**
1. **Element Mastery Functions**: get_element_mastery, has_element_affinity
2. **Element Interaction Functions**: get_element_interaction, is_element_overcoming
3. **Basic Element Status Functions**: has_element_status_effect

### **Phase 2: Advanced Element Functions (1 week)**
1. **Element Resource Functions**: get_element_resource_value, is_element_resource_below_threshold
2. **Element Derived Stats Functions**: get_element_derived_stat, has_element_derived_stat
3. **Element Resistance Functions**: get_element_resistance, is_element_weakness

### **Phase 3: Hybrid Element Functions (1 week)**
1. **Hybrid Element Functions**: has_hybrid_element, is_hybrid_element_activated
2. **Hybrid Parent Functions**: get_hybrid_element_parents
3. **Hybrid Activation Functions**: check_hybrid_element_activation_conditions

### **Phase 4: Integration & Testing (1 week)**
1. **System Integration**: Update Combat Core, Shield System, Status Core
2. **Performance Testing**: Test caching and batch evaluation
3. **End-to-End Testing**: Test complete element condition workflows

## 📚 **Related Documents**

- [00_Condition_Core_Overview.md](00_Condition_Core_Overview.md) - Condition Core overview
- [07_Condition_Core_Integration_Design.md](07_Condition_Core_Integration_Design.md) - General integration design
- [Element Core Documentation](../element-core/README.md) - Element Core system
- [Element Core Multi-System Integration](../element-core/02_Multi_System_Integration_Design.md) - Element Core integration patterns

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
