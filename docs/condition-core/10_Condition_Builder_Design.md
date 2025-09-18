# Condition Builder Design

## 📋 **Tổng Quan**

Condition Builder là một API programmatic để tạo conditions một cách type-safe và hiệu quả, bổ sung cho YAML configuration system. Nó cung cấp IDE support, compile-time validation, và performance tốt hơn cho dynamic conditions.

## 🎯 **Tại Sao Cần Condition Builder?**

### **1. Complementary to YAML**

```
Configuration Methods
├── YAML Configuration
│   ├── Static conditions
│   ├── Configuration files
│   ├── Hot reload support
│   └── Non-technical users
└── Condition Builder
    ├── Dynamic conditions
    ├── Programmatic logic
    ├── Type safety
    └── IDE support
```

### **2. Use Cases**

#### **YAML - Good For:**
- ✅ **Static Configuration**: Pre-defined conditions
- ✅ **Configuration Files**: Easy to modify without recompilation
- ✅ **Hot Reload**: Dynamic configuration updates
- ✅ **Non-technical Users**: Easy to understand and modify

#### **Condition Builder - Good For:**
- ✅ **Dynamic Conditions**: Runtime-generated conditions
- ✅ **Programmatic Logic**: Complex condition building
- ✅ **Type Safety**: Compile-time validation
- ✅ **IDE Support**: Auto-completion and refactoring
- ✅ **Performance**: No parsing overhead

## 🏗️ **Architecture Design**

### **1. Core Components**

```rust
// Condition Builder
pub struct ConditionBuilder {
    condition_id: Option<String>,
    function_name: Option<String>,
    parameters: Vec<ConditionParameter>,
    operator: Option<ConditionOperator>,
    value: Option<ConditionValue>,
}

// Condition Chain Builder
pub struct ConditionChainBuilder {
    chain_id: Option<String>,
    logic: Option<ChainLogic>,
    conditions: Vec<ConditionConfig>,
}

// Builder Factory
pub struct ConditionBuilderFactory {
    // Factory methods for common patterns
}
```

### **2. Builder Pattern Implementation**

```rust
impl ConditionBuilder {
    // Create new builder
    pub fn new() -> Self {
        Self {
            condition_id: None,
            function_name: None,
            parameters: Vec::new(),
            operator: None,
            value: None,
        }
    }
    
    // Set condition ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.condition_id = Some(id.into());
        self
    }
    
    // Set function name
    pub fn function(mut self, function: impl Into<String>) -> Self {
        self.function_name = Some(function.into());
        self
    }
    
    // Add parameter
    pub fn parameter(mut self, param: impl Into<ConditionParameter>) -> Self {
        self.parameters.push(param.into());
        self
    }
    
    // Set operator
    pub fn operator(mut self, op: ConditionOperator) -> Self {
        self.operator = Some(op);
        self
    }
    
    // Set value
    pub fn value(mut self, value: impl Into<ConditionValue>) -> Self {
        self.value = Some(value.into());
        self
    }
    
    // Build condition
    pub fn build(self) -> ConditionResult<ConditionConfig> {
        // Validation and construction
    }
}
```

## 🔧 **API Design**

### **1. Basic Condition Building**

```rust
// Basic condition building
let condition = ConditionBuilder::new()
    .id("check_health")
    .function("get_actor_resource")
    .parameter("health")
    .operator(ConditionOperator::GreaterThan)
    .value(ConditionValue::Float(75.0))
    .build()?;
```

### **2. Chain Condition Building**

```rust
// Chain condition building
let chain = ConditionChainBuilder::new()
    .id("complex_condition")
    .logic(ChainLogic::And)
    .condition(
        ConditionBuilder::new()
            .id("check_health")
            .function("get_actor_resource")
            .parameter("health")
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(50.0))
            .build()?
    )
    .condition(
        ConditionBuilder::new()
            .id("check_mana")
            .function("get_actor_resource")
            .parameter("mana")
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(25.0))
            .build()?
    )
    .build()?;
```

### **3. Factory Methods**

```rust
// Factory methods for common patterns
impl ConditionBuilderFactory {
    // Health check factory
    pub fn health_check(threshold: f64) -> ConditionBuilder {
        ConditionBuilder::new()
            .id("health_check")
            .function("get_actor_resource")
            .parameter("health")
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(threshold))
    }
    
    // Mana check factory
    pub fn mana_check(threshold: f64) -> ConditionBuilder {
        ConditionBuilder::new()
            .id("mana_check")
            .function("get_actor_resource")
            .parameter("mana")
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(threshold))
    }
    
    // Element mastery check factory
    pub fn element_mastery_check(element: &str, threshold: f64) -> ConditionBuilder {
        ConditionBuilder::new()
            .id("element_mastery_check")
            .function("get_element_mastery")
            .parameter(element)
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(threshold))
    }
}
```

## 🚀 **Usage Examples**

### **1. Basic Usage**

```rust
use condition_core::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create condition using builder
    let condition = ConditionBuilder::new()
        .id("check_health")
        .function("get_actor_resource")
        .parameter("health")
        .operator(ConditionOperator::GreaterThan)
        .value(ConditionValue::Float(75.0))
        .build()?;
    
    // Create resolver and context
    let resolver = create_test_resolver();
    let context = create_test_context();
    
    // Evaluate condition
    let result = resolver.resolve_condition(&condition, &context).await?;
    println!("Health > 75: {}", result);
    
    Ok(())
}
```

### **2. Dynamic Condition Building**

```rust
// Dynamic condition building based on runtime data
fn create_dynamic_condition(health_threshold: f64, mana_threshold: f64) -> ConditionResult<ConditionChainConfig> {
    let chain = ConditionChainBuilder::new()
        .id("dynamic_condition")
        .logic(ChainLogic::And)
        .condition(
            ConditionBuilder::new()
                .id("check_health")
                .function("get_actor_resource")
                .parameter("health")
                .operator(ConditionOperator::GreaterThan)
                .value(ConditionValue::Float(health_threshold))
                .build()?
        )
        .condition(
            ConditionBuilder::new()
                .id("check_mana")
                .function("get_actor_resource")
                .parameter("mana")
                .operator(ConditionOperator::GreaterThan)
                .value(ConditionValue::Float(mana_threshold))
                .build()?
        )
        .build()?;
    
    Ok(chain)
}
```

### **3. Factory Usage**

```rust
// Using factory methods
let health_condition = ConditionBuilderFactory::health_check(75.0).build()?;
let mana_condition = ConditionBuilderFactory::mana_check(50.0).build()?;
let fire_mastery_condition = ConditionBuilderFactory::element_mastery_check("fire", 100.0).build()?;

// Create chain using factory methods
let chain = ConditionChainBuilder::new()
    .id("factory_chain")
    .logic(ChainLogic::And)
    .condition(health_condition)
    .condition(mana_condition)
    .condition(fire_mastery_condition)
    .build()?;
```

### **4. Complex Logic Building**

```rust
// Complex logic building
fn create_complex_condition() -> ConditionResult<ConditionChainConfig> {
    let chain = ConditionChainBuilder::new()
        .id("complex_condition")
        .logic(ChainLogic::And)
        .condition(
            // Health check
            ConditionBuilder::new()
                .id("check_health")
                .function("get_actor_resource")
                .parameter("health")
                .operator(ConditionOperator::GreaterThan)
                .value(ConditionValue::Float(50.0))
                .build()?
        )
        .condition(
            // Mana check
            ConditionBuilder::new()
                .id("check_mana")
                .function("get_actor_resource")
                .parameter("mana")
                .operator(ConditionOperator::GreaterThan)
                .value(ConditionValue::Float(25.0))
                .build()?
        )
        .condition(
            // Fire mastery check
            ConditionBuilder::new()
                .id("check_fire_mastery")
                .function("get_element_mastery")
                .parameter("fire")
                .operator(ConditionOperator::GreaterThan)
                .value(ConditionValue::Float(100.0))
                .build()?
        )
        .condition(
            // Has weapon check
            ConditionBuilder::new()
                .id("check_weapon")
                .function("has_category_item")
                .parameter("weapon")
                .operator(ConditionOperator::Equal)
                .value(ConditionValue::Boolean(true))
                .build()?
        )
        .build()?;
    
    Ok(chain)
}
```

## 📊 **Performance Benefits**

### **1. No Parsing Overhead**

```rust
// YAML - requires parsing
let yaml = r#"
condition_id: "check_health"
function_name: "get_actor_resource"
operator: GreaterThan
value: !Float 75.0
parameters:
  - !String "health"
"#;
let condition = parse_condition_config(yaml)?; // Parsing overhead

// Builder - direct construction
let condition = ConditionBuilder::new()
    .id("check_health")
    .function("get_actor_resource")
    .parameter("health")
    .operator(ConditionOperator::GreaterThan)
    .value(ConditionValue::Float(75.0))
    .build()?; // No parsing overhead
```

### **2. Compile-time Validation**

```rust
// Compile-time validation
let condition = ConditionBuilder::new()
    .id("check_health")
    .function("get_actor_resource")
    .parameter("health")
    .operator(ConditionOperator::GreaterThan) // Type checked
    .value(ConditionValue::Float(75.0)) // Type checked
    .build()?; // Compile-time validation
```

### **3. IDE Support**

```rust
// IDE auto-completion and validation
let condition = ConditionBuilder::new()
    .id("check_health")
    .function("get_actor_resource") // IDE suggests valid functions
    .parameter("health") // IDE suggests valid parameters
    .operator(ConditionOperator::GreaterThan) // IDE suggests valid operators
    .value(ConditionValue::Float(75.0)) // IDE suggests valid value types
    .build()?; // IDE shows validation errors
```

## 🔄 **Integration với Existing Systems**

### **1. Action Core Integration**

```rust
// Action Core using Condition Builder
impl ActionCore {
    pub fn create_action_conditions(&self, action: &Action) -> Result<Vec<ConditionConfig>, ActionError> {
        let mut conditions = Vec::new();
        
        // Health check
        if let Some(health_threshold) = action.health_requirement {
            let condition = ConditionBuilder::new()
                .id("action_health_check")
                .function("get_actor_resource")
                .parameter("health")
                .operator(ConditionOperator::GreaterThan)
                .value(ConditionValue::Float(health_threshold))
                .build()?;
            conditions.push(condition);
        }
        
        // Mana check
        if let Some(mana_threshold) = action.mana_requirement {
            let condition = ConditionBuilder::new()
                .id("action_mana_check")
                .function("get_actor_resource")
                .parameter("mana")
                .operator(ConditionOperator::GreaterThan)
                .value(ConditionValue::Float(mana_threshold))
                .build()?;
            conditions.push(condition);
        }
        
        // Element mastery check
        if let Some(element_requirement) = &action.element_requirement {
            let condition = ConditionBuilder::new()
                .id("action_element_check")
                .function("get_element_mastery")
                .parameter(&element_requirement.element)
                .operator(ConditionOperator::GreaterThan)
                .value(ConditionValue::Float(element_requirement.threshold))
                .build()?;
            conditions.push(condition);
        }
        
        Ok(conditions)
    }
}
```

### **2. Status Core Integration**

```rust
// Status Core using Condition Builder
impl StatusCore {
    pub fn create_status_conditions(&self, status: &StatusEffect) -> Result<Vec<ConditionConfig>, StatusError> {
        let mut conditions = Vec::new();
        
        // Health requirement
        if let Some(health_threshold) = status.health_requirement {
            let condition = ConditionBuilder::new()
                .id("status_health_check")
                .function("get_actor_resource")
                .parameter("health")
                .operator(ConditionOperator::GreaterThan)
                .value(ConditionValue::Float(health_threshold))
                .build()?;
            conditions.push(condition);
        }
        
        // Element affinity check
        if let Some(element_affinity) = &status.element_affinity {
            let condition = ConditionBuilder::new()
                .id("status_element_affinity")
                .function("has_element_affinity")
                .parameter(&element_affinity.element)
                .operator(ConditionOperator::Equal)
                .value(ConditionValue::Boolean(true))
                .build()?;
            conditions.push(condition);
        }
        
        Ok(conditions)
    }
}
```

## 🎯 **Best Practices**

### **1. Builder Pattern Usage**

```rust
// Good: Fluent interface
let condition = ConditionBuilder::new()
    .id("check_health")
    .function("get_actor_resource")
    .parameter("health")
    .operator(ConditionOperator::GreaterThan)
    .value(ConditionValue::Float(75.0))
    .build()?;

// Bad: Multiple assignments
let mut builder = ConditionBuilder::new();
builder = builder.id("check_health");
builder = builder.function("get_actor_resource");
// ... more assignments
let condition = builder.build()?;
```

### **2. Error Handling**

```rust
// Good: Proper error handling
let condition = ConditionBuilder::new()
    .id("check_health")
    .function("get_actor_resource")
    .parameter("health")
    .operator(ConditionOperator::GreaterThan)
    .value(ConditionValue::Float(75.0))
    .build()
    .map_err(|e| {
        eprintln!("Failed to build condition: {}", e);
        e
    })?;
```

### **3. Factory Methods**

```rust
// Good: Use factory methods for common patterns
let health_condition = ConditionBuilderFactory::health_check(75.0).build()?;
let mana_condition = ConditionBuilderFactory::mana_check(50.0).build()?;

// Bad: Repetitive builder calls
let health_condition = ConditionBuilder::new()
    .id("health_check")
    .function("get_actor_resource")
    .parameter("health")
    .operator(ConditionOperator::GreaterThan)
    .value(ConditionValue::Float(75.0))
    .build()?;
```

## 📝 **Implementation Status**

### **✅ Planned Features**
1. **Basic Condition Builder**: Core builder functionality
2. **Chain Builder**: Complex condition chains
3. **Factory Methods**: Common pattern factories
4. **Validation**: Compile-time and runtime validation
5. **IDE Support**: Auto-completion and error checking

### **🔄 Future Enhancements**
1. **Advanced Validation**: Custom validation rules
2. **Optimization**: Builder optimization
3. **Serialization**: YAML/JSON export
4. **Testing**: Comprehensive test coverage

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
