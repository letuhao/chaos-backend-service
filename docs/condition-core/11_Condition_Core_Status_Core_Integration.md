# Condition Core - Status Core Integration

## 📋 **Tổng Quan**

Tài liệu này mô tả cách tích hợp Status Core với Condition Core để cung cấp các status condition functions cho tất cả các hệ thống trong Chaos World. Tích hợp này cho phép các hệ thống khác sử dụng condition-core để đánh giá các điều kiện liên quan đến status effects một cách thống nhất và hiệu quả.

## 🎯 **Mục Đích Tích Hợp**

### **1. Standardized Status Conditions**
- **Unified Logic**: Tất cả status conditions được xử lý thống nhất qua Condition Core
- **Cross-System Reuse**: Các hệ thống khác có thể tái sử dụng status conditions
- **Consistent Behavior**: Hành vi nhất quán across tất cả systems
- **Performance Optimization**: Tối ưu performance thông qua centralized caching

### **2. Status Core Integration Benefits**
- **Centralized Management**: Quản lý tập trung tất cả status conditions
- **Plugin Architecture**: Hỗ trợ plugin-based architecture
- **Configuration-Driven**: Configuration-driven approach
- **Real-time Processing**: Real-time status condition evaluation

## 🏗️ **Kiến Trúc Tích Hợp**

### **1. Status Core Integration Architecture**

```
Status Core Integration
├── Status Data Provider
│   ├── StatusEffectDataProvider
│   ├── StatusImmunityDataProvider
│   ├── StatusCategoryDataProvider
│   └── StatusInteractionDataProvider
├── Status Condition Functions
│   ├── Basic Status Functions (10+ functions)
│   ├── Status Effect Functions (8+ functions)
│   ├── Status Immunity Functions (5+ functions)
│   ├── Status Category Functions (3+ functions)
│   └── Status Interaction Functions (4+ functions)
├── Status Condition Registry
│   ├── Status Function Registry
│   ├── Status Template Registry
│   └── Status Metadata Registry
└── Status Condition Evaluation
    ├── Status Condition Evaluator
    ├── Status Condition Parser
    └── Status Condition Optimizer
```

### **2. Status Data Provider Interface**

```rust
/// Status Data Provider Interface
#[async_trait]
pub trait StatusDataProvider: Send + Sync {
    // Basic Status Functions
    async fn has_status_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_effect_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32>;
    async fn get_status_effect_magnitude(&self, actor_id: &str, effect_id: &str) -> ConditionResult<f64>;
    async fn get_status_effect_duration(&self, actor_id: &str, effect_id: &str) -> ConditionResult<Duration>;
    async fn is_status_effect_active(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn is_status_effect_expired(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_effect_stack(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32>;
    async fn get_status_effect_priority(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32>;
    async fn get_status_effect_source(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String>;
    async fn get_status_effect_target(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String>;

    // Status Immunity Functions
    async fn has_status_immunity(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_immunity_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32>;
    async fn is_status_immunity_active(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_immunity_duration(&self, actor_id: &str, effect_id: &str) -> ConditionResult<Duration>;
    async fn get_status_immunity_source(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String>;

    // Status Category Functions
    async fn has_status_category(&self, actor_id: &str, category: &str) -> ConditionResult<bool>;
    async fn get_status_category_count(&self, actor_id: &str, category: &str) -> ConditionResult<u32>;
    async fn list_status_categories(&self, actor_id: &str) -> ConditionResult<Vec<String>>;

    // Status Interaction Functions
    async fn is_status_effect_stackable(&self, effect_id: &str) -> ConditionResult<bool>;
    async fn can_status_effect_stack(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_effect_interaction(&self, effect_id: &str, target_effect_id: &str) -> ConditionResult<String>;
    async fn get_status_effect_modifier(&self, actor_id: &str, effect_id: &str, modifier_type: &str) -> ConditionResult<f64>;

    // Status Movement Functions
    async fn has_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<bool>;
    async fn get_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<f64>;

    // Status Visual/Audio Functions
    async fn has_status_visual_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_visual_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String>;
    async fn has_status_audio_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_audio_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String>;

    // Status Properties Functions
    async fn get_status_effect_properties(&self, actor_id: &str, effect_id: &str) -> ConditionResult<HashMap<String, serde_json::Value>>;
    async fn has_status_effect_property(&self, actor_id: &str, effect_id: &str, property: &str) -> ConditionResult<bool>;
    async fn get_status_effect_property(&self, actor_id: &str, effect_id: &str, property: &str) -> ConditionResult<serde_json::Value>;

    // Status History Functions
    async fn get_status_effect_history(&self, actor_id: &str, effect_id: &str) -> ConditionResult<Vec<StatusEffectHistory>>;
    async fn get_status_effect_timeline(&self, actor_id: &str, effect_id: &str) -> ConditionResult<Vec<StatusEffectTimeline>>;
}
```

## 🔧 **Status Condition Functions**

### **1. Basic Status Functions**

```rust
/// Has Status Effect Function
pub struct HasStatusEffectFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for HasStatusEffectFunction {
    fn name(&self) -> &str { "has_status_effect" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        data_accessor.has_status_effect(&context.target.id, effect_id).await
    }
}

/// Get Status Effect Count Function
pub struct GetStatusEffectCountFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetStatusEffectCountFunction {
    fn name(&self) -> &str { "get_status_effect_count" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 2 {
            return Err(ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        let expected_count = params[1].as_integer()? as u32;
        
        let actual_count = data_accessor.get_status_effect_count(&context.target.id, effect_id).await?;
        Ok(actual_count >= expected_count)
    }
}

/// Get Status Effect Magnitude Function
pub struct GetStatusEffectMagnitudeFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetStatusEffectMagnitudeFunction {
    fn name(&self) -> &str { "get_status_effect_magnitude" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 2 {
            return Err(ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        let expected_magnitude = params[1].as_float()?;
        
        let actual_magnitude = data_accessor.get_status_effect_magnitude(&context.target.id, effect_id).await?;
        Ok(actual_magnitude >= expected_magnitude)
    }
}

/// Is Status Effect Active Function
pub struct IsStatusEffectActiveFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsStatusEffectActiveFunction {
    fn name(&self) -> &str { "is_status_effect_active" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        data_accessor.is_status_effect_active(&context.target.id, effect_id).await
    }
}

/// Is Status Effect Expired Function
pub struct IsStatusEffectExpiredFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsStatusEffectExpiredFunction {
    fn name(&self) -> &str { "is_status_effect_expired" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        data_accessor.is_status_effect_expired(&context.target.id, effect_id).await
    }
}
```

### **2. Status Immunity Functions**

```rust
/// Has Status Immunity Function
pub struct HasStatusImmunityFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for HasStatusImmunityFunction {
    fn name(&self) -> &str { "has_status_immunity" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        data_accessor.has_status_immunity(&context.target.id, effect_id).await
    }
}

/// Get Status Immunity Count Function
pub struct GetStatusImmunityCountFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetStatusImmunityCountFunction {
    fn name(&self) -> &str { "get_status_immunity_count" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 2 {
            return Err(ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        let expected_count = params[1].as_integer()? as u32;
        
        let actual_count = data_accessor.get_status_immunity_count(&context.target.id, effect_id).await?;
        Ok(actual_count >= expected_count)
    }
}

/// Is Status Immunity Active Function
pub struct IsStatusImmunityActiveFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsStatusImmunityActiveFunction {
    fn name(&self) -> &str { "is_status_immunity_active" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        data_accessor.is_status_immunity_active(&context.target.id, effect_id).await
    }
}
```

### **3. Status Category Functions**

```rust
/// Has Status Category Function
pub struct HasStatusCategoryFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for HasStatusCategoryFunction {
    fn name(&self) -> &str { "has_status_category" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        
        let category = params[0].as_string()?;
        data_accessor.has_status_category(&context.target.id, category).await
    }
}

/// Get Status Category Count Function
pub struct GetStatusCategoryCountFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetStatusCategoryCountFunction {
    fn name(&self) -> &str { "get_status_category_count" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 2 {
            return Err(ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }
        
        let category = params[0].as_string()?;
        let expected_count = params[1].as_integer()? as u32;
        
        let actual_count = data_accessor.get_status_category_count(&context.target.id, category).await?;
        Ok(actual_count >= expected_count)
    }
}
```

### **4. Status Interaction Functions**

```rust
/// Is Status Effect Stackable Function
pub struct IsStatusEffectStackableFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsStatusEffectStackableFunction {
    fn name(&self) -> &str { "is_status_effect_stackable" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        data_accessor.is_status_effect_stackable(effect_id).await
    }
}

/// Can Status Effect Stack Function
pub struct CanStatusEffectStackFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for CanStatusEffectStackFunction {
    fn name(&self) -> &str { "can_status_effect_stack" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        data_accessor.can_status_effect_stack(&context.target.id, effect_id).await
    }
}

/// Get Status Effect Interaction Function
pub struct GetStatusEffectInteractionFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetStatusEffectInteractionFunction {
    fn name(&self) -> &str { "get_status_effect_interaction" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 2 {
            return Err(ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        let target_effect_id = params[1].as_string()?;
        
        let interaction = data_accessor.get_status_effect_interaction(effect_id, target_effect_id).await?;
        Ok(interaction == "compatible" || interaction == "synergy")
    }
}
```

### **5. Status Movement Functions**

```rust
/// Has Status Movement Restriction Function
pub struct HasStatusMovementRestrictionFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for HasStatusMovementRestrictionFunction {
    fn name(&self) -> &str { "has_status_movement_restriction" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        
        let restriction_type = params[0].as_string()?;
        data_accessor.has_status_movement_restriction(&context.target.id, restriction_type).await
    }
}

/// Get Status Movement Restriction Function
pub struct GetStatusMovementRestrictionFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetStatusMovementRestrictionFunction {
    fn name(&self) -> &str { "get_status_movement_restriction" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 2 {
            return Err(ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }
        
        let restriction_type = params[0].as_string()?;
        let expected_magnitude = params[1].as_float()?;
        
        let actual_magnitude = data_accessor.get_status_movement_restriction(&context.target.id, restriction_type).await?;
        Ok(actual_magnitude >= expected_magnitude)
    }
}
```

### **6. Status Visual/Audio Functions**

```rust
/// Has Status Visual Effect Function
pub struct HasStatusVisualEffectFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for HasStatusVisualEffectFunction {
    fn name(&self) -> &str { "has_status_visual_effect" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        data_accessor.has_status_visual_effect(&context.target.id, effect_id).await
    }
}

/// Has Status Audio Effect Function
pub struct HasStatusAudioEffectFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for HasStatusAudioEffectFunction {
    fn name(&self) -> &str { "has_status_audio_effect" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        data_accessor.has_status_audio_effect(&context.target.id, effect_id).await
    }
}
```

### **7. Status Properties Functions**

```rust
/// Has Status Effect Property Function
pub struct HasStatusEffectPropertyFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for HasStatusEffectPropertyFunction {
    fn name(&self) -> &str { "has_status_effect_property" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 2 {
            return Err(ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        let property = params[1].as_string()?;
        data_accessor.has_status_effect_property(&context.target.id, effect_id, property).await
    }
}

/// Get Status Effect Property Function
pub struct GetStatusEffectPropertyFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetStatusEffectPropertyFunction {
    fn name(&self) -> &str { "get_status_effect_property" }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 3 {
            return Err(ConditionError::InvalidParameterCount { expected: 3, actual: params.len() });
        }
        
        let effect_id = params[0].as_string()?;
        let property = params[1].as_string()?;
        let expected_value = params[2].as_string()?;
        
        let actual_value = data_accessor.get_status_effect_property(&context.target.id, effect_id, property).await?;
        Ok(actual_value.as_str().unwrap_or("") == expected_value)
    }
}
```

## 🎮 **Status Condition Examples**

### **1. Basic Status Conditions**

```yaml
# Has Burning Status Effect
burning_status_condition:
  condition_id: "burning_status_condition"
  condition_function: "has_status_effect"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "burning"
  condition_operator: "equal"
  condition_value:
    value_type: "boolean"
    value: true

# Status Effect Count Condition
multiple_burning_condition:
  condition_id: "multiple_burning_condition"
  condition_function: "get_status_effect_count"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "burning"
    - parameter_type: "integer"
      parameter_value: 3
  condition_operator: "greater_than_or_equal"
  condition_value:
    value_type: "boolean"
    value: true

# Status Effect Magnitude Condition
high_magnitude_burning_condition:
  condition_id: "high_magnitude_burning_condition"
  condition_function: "get_status_effect_magnitude"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "burning"
    - parameter_type: "float"
      parameter_value: 50.0
  condition_operator: "greater_than_or_equal"
  condition_value:
    value_type: "boolean"
    value: true
```

### **2. Status Immunity Conditions**

```yaml
# Has Fire Immunity
fire_immunity_condition:
  condition_id: "fire_immunity_condition"
  condition_function: "has_status_immunity"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "fire"
  condition_operator: "equal"
  condition_value:
    value_type: "boolean"
    value: true

# Multiple Immunities
multiple_immunities_condition:
  condition_id: "multiple_immunities_condition"
  condition_function: "get_status_immunity_count"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "stun"
    - parameter_type: "integer"
      parameter_value: 2
  condition_operator: "greater_than_or_equal"
  condition_value:
    value_type: "boolean"
    value: true
```

### **3. Status Category Conditions**

```yaml
# Has Debuff Category
debuff_category_condition:
  condition_id: "debuff_category_condition"
  condition_function: "has_status_category"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "debuff"
  condition_operator: "equal"
  condition_value:
    value_type: "boolean"
    value: true

# Multiple Debuffs
multiple_debuffs_condition:
  condition_id: "multiple_debuffs_condition"
  condition_function: "get_status_category_count"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "debuff"
    - parameter_type: "integer"
      parameter_value: 5
  condition_operator: "greater_than_or_equal"
  condition_value:
    value_type: "boolean"
    value: true
```

### **4. Status Interaction Conditions**

```yaml
# Stackable Status Effect
stackable_effect_condition:
  condition_id: "stackable_effect_condition"
  condition_function: "is_status_effect_stackable"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "poison"
  condition_operator: "equal"
  condition_value:
    value_type: "boolean"
    value: true

# Status Effect Interaction
status_interaction_condition:
  condition_id: "status_interaction_condition"
  condition_function: "get_status_effect_interaction"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "fire"
    - parameter_type: "string"
      parameter_value: "water"
  condition_operator: "equal"
  condition_value:
    value_type: "string"
    value: "conflict"
```

### **5. Status Movement Conditions**

```yaml
# Movement Restriction
movement_restriction_condition:
  condition_id: "movement_restriction_condition"
  condition_function: "has_status_movement_restriction"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "cannot_move"
  condition_operator: "equal"
  condition_value:
    value_type: "boolean"
    value: true

# Speed Reduction
speed_reduction_condition:
  condition_id: "speed_reduction_condition"
  condition_function: "get_status_movement_restriction"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "reduced_speed"
    - parameter_type: "float"
      parameter_value: 0.5
  condition_operator: "less_than_or_equal"
  condition_value:
    value_type: "boolean"
    value: true
```

### **6. Complex Status Conditions**

```yaml
# Complex Burning Condition
complex_burning_condition:
  condition_id: "complex_burning_condition"
  condition_logic: "AND"
  condition_conditions:
    - condition_id: "has_burning_effect"
      condition_function: "has_status_effect"
      condition_parameters:
        - parameter_type: "string"
          parameter_value: "burning"
      condition_operator: "equal"
      condition_value:
        value_type: "boolean"
        value: true
    - condition_id: "burning_magnitude"
      condition_function: "get_status_effect_magnitude"
      condition_parameters:
        - parameter_type: "string"
          parameter_value: "burning"
        - parameter_type: "float"
          parameter_value: 25.0
      condition_operator: "greater_than_or_equal"
      condition_value:
        value_type: "boolean"
        value: true
    - condition_id: "no_fire_immunity"
      condition_function: "has_status_immunity"
      condition_parameters:
        - parameter_type: "string"
          parameter_value: "fire"
      condition_operator: "equal"
      condition_value:
        value_type: "boolean"
        value: false

# Status Stacking Condition
status_stacking_condition:
  condition_id: "status_stacking_condition"
  condition_logic: "AND"
  condition_conditions:
    - condition_id: "has_poison_effect"
      condition_function: "has_status_effect"
      condition_parameters:
        - parameter_type: "string"
          parameter_value: "poison"
      condition_operator: "equal"
      condition_value:
        value_type: "boolean"
        value: true
    - condition_id: "poison_is_stackable"
      condition_function: "is_status_effect_stackable"
      condition_parameters:
        - parameter_type: "string"
          parameter_value: "poison"
      condition_operator: "equal"
      condition_value:
        value_type: "boolean"
        value: true
    - condition_id: "can_poison_stack"
      condition_function: "can_status_effect_stack"
      condition_parameters:
        - parameter_type: "string"
          parameter_value: "poison"
      condition_operator: "equal"
      condition_value:
        value_type: "boolean"
        value: true
```

## 🚀 **Performance Benefits**

### **1. Centralized Caching**
- **Status Effect Cache**: Cache status effect data để tránh repeated lookups
- **Status Immunity Cache**: Cache immunity data cho performance
- **Status Category Cache**: Cache category data cho fast access
- **Status Interaction Cache**: Cache interaction data cho complex conditions

### **2. Batch Processing**
- **Batch Status Evaluation**: Evaluate multiple status conditions cùng lúc
- **Batch Status Updates**: Update multiple status effects efficiently
- **Batch Status Queries**: Query multiple status data cùng lúc

### **3. Memory Optimization**
- **Object Pooling**: Reuse status effect objects
- **Lazy Loading**: Load status data only when needed
- **Memory Management**: Efficient memory usage cho status data

## 🧪 **Testing Strategy**

### **1. Unit Testing**
- **Status Function Tests**: Test individual status condition functions
- **Status Provider Tests**: Test status data provider implementations
- **Status Integration Tests**: Test status core integration
- **Status Performance Tests**: Test status condition performance

### **2. Integration Testing**
- **Status Core Integration**: Test integration với Status Core
- **Cross-System Integration**: Test integration với other systems
- **Status Condition Workflow**: Test complete status condition workflows
- **Status Condition Performance**: Test performance under load

### **3. End-to-End Testing**
- **Status Condition Scenarios**: Test real-world status condition scenarios
- **Status Effect Lifecycle**: Test complete status effect lifecycle
- **Status Condition Edge Cases**: Test edge cases và error conditions
- **Status Condition Stress Testing**: Test under high load

## 📝 **Implementation Notes**

### **1. Status Core Integration**
- **Status Data Provider**: Implement StatusDataProvider trait
- **Status Condition Functions**: Implement 25+ status condition functions
- **Status Condition Registry**: Register status condition functions
- **Status Condition Evaluation**: Evaluate status conditions efficiently

### **2. Performance Considerations**
- **Caching Strategy**: Implement multi-level caching
- **Batch Processing**: Process multiple status conditions efficiently
- **Memory Management**: Optimize memory usage
- **Concurrency**: Handle concurrent status condition evaluation

### **3. Error Handling**
- **Status Errors**: Handle status-related errors gracefully
- **Validation Errors**: Validate status condition parameters
- **Integration Errors**: Handle integration errors
- **Performance Errors**: Handle performance-related errors

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
