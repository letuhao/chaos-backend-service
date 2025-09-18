# Condition Core - Updated Documentation with Builder

## 📋 **Tổng Quan**

Condition Core là hệ thống đơn giản và hiệu quả để resolve conditions trong game, được thiết kế với **Dependency Injection** architecture để hỗ trợ plugin-based system. Hệ thống hỗ trợ cả **YAML configuration** và **Condition Builder** cho programmatic condition creation.

## 🎯 **Tại Sao Cần Condition Core?**

### **Vấn Đề Hiện Tại**
- 🔴 **Condition Duplication**: Cùng một condition được implement ở nhiều nơi
- 🔴 **Inconsistent Logic**: Logic condition không nhất quán
- 🔴 **Hard to Maintain**: Khó maintain và update conditions
- 🔴 **Plugin System Violation**: Hard-coded data vi phạm plugin design

### **Lợi Ích của Condition Core**
- ✅ **Unified Condition System**: Single source of truth
- ✅ **Dependency Injection**: No hard-coded data
- ✅ **Plugin Ready**: Easy to add new elements/resources/categories
- ✅ **Clean Error Handling**: Proper exceptions when no provider
- ✅ **Multiple Configuration Methods**: YAML + Builder

## 🏗️ **Architecture (Actual Implementation)**

```
Condition Core
├── src/
│   ├── lib.rs              # Main exports
│   ├── error.rs            # Error types
│   ├── types.rs            # Core types & traits
│   ├── resolver.rs         # Main resolver
│   ├── functions.rs        # Condition functions
│   ├── config.rs           # YAML config loading
│   ├── data_provider.rs    # Data provider interfaces
│   └── builder.rs          # Condition Builder
├── examples/
│   ├── basic_usage.rs      # Basic usage
│   ├── yaml_config.rs      # YAML config example
│   ├── advanced_usage.rs   # Advanced usage
│   ├── dependency_injection.rs # DI example
│   ├── architecture_review.rs # Architecture demo
│   └── condition_builder.rs # Builder example
└── tests/
    ├── unit_tests.rs       # Unit tests
    └── integration_tests.rs # Integration tests
```

## 🔌 **Core Components**

### **1. Condition Resolver**

```rust
pub struct ConditionResolver {
    function_registry: FunctionRegistry,
    data_registry: DataProviderRegistry,
}

impl ConditionResolver {
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        let function_registry = create_function_registry_with_providers(&data_registry);
        Self {
            function_registry,
            data_registry,
        }
    }
}
```

### **2. Data Provider System**

```rust
// Data provider interfaces
#[async_trait::async_trait]
pub trait ElementDataProvider: Send + Sync {
    async fn get_element_mastery(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn get_element_resistance(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn has_element_affinity(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn is_element_weakness(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String>;
    async fn list_elements(&self) -> ConditionResult<Vec<String>>;
}

// Data provider registry
pub struct DataProviderRegistry {
    element_provider: Option<Arc<dyn ElementDataProvider>>,
    resource_provider: Option<Arc<dyn ResourceDataProvider>>,
    category_provider: Option<Arc<dyn CategoryDataProvider>>,
    // ... other providers
}
```

### **3. Condition Functions**

```rust
// Generic condition functions
pub struct GetActorResourceFunction {
    data_provider: Option<Arc<dyn ResourceDataProvider>>,
}

pub struct GetElementMasteryFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

pub struct HasCategoryItemFunction {
    data_provider: Option<Arc<dyn CategoryDataProvider>>,
}
```

## 🔧 **Configuration Methods**

### **1. YAML Configuration**

```yaml
# Basic condition configuration
condition_id: "check_health"
function_name: "get_actor_resource"
operator: GreaterThan
value: !Float 75.0
parameters:
  - !String "health"
```

### **2. Condition Builder**

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

### **3. Factory Methods**

```rust
// Factory methods for common patterns
let health_condition = ConditionBuilderFactory::health_check(75.0).build()?;
let mana_condition = ConditionBuilderFactory::mana_check(50.0).build()?;
let fire_mastery_condition = ConditionBuilderFactory::element_mastery_check("fire", 100.0).build()?;
let weapon_condition = ConditionBuilderFactory::has_category_item("weapon").build()?;
```

### **4. Condition Chains**

```rust
// Chain condition building
let chain = ConditionChainBuilder::new()
    .id("complex_condition")
    .logic(ChainLogic::And)
    .condition(health_condition)
    .condition(mana_condition)
    .condition(fire_mastery_condition)
    .condition(weapon_condition)
    .build()?;
```

## 🚀 **Usage Examples**

### **1. YAML Configuration Usage**

```rust
use condition_core::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load condition from YAML
    let yaml = r#"
    condition_id: "check_mana"
    function_name: "get_actor_resource"
    operator: GreaterThan
    value: !Float 50.0
    parameters:
      - !String "mana"
    "#;

    let condition = parse_condition_config(yaml)?;
    
    // Create resolver and context
    let resolver = create_test_resolver();
    let context = create_test_context();
    
    // Evaluate condition
    let result = resolver.resolve_condition(&condition, &context).await?;
    println!("Mana > 50: {}", result);

    Ok(())
}
```

### **2. Condition Builder Usage**

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

### **3. Dynamic Condition Building**

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

### **4. Factory Usage**

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

## 🔌 **Integration với Other Systems**

### **1. Actor Core Integration** ⭐ **IMPLEMENTED**

Condition Core đã được tích hợp với Actor Core để cung cấp conditional logic cho actor stat aggregation:

```rust
use condition_core::{ConditionResolver, DataProviderRegistry, ConditionBuilder};
use actor_core::*;

pub struct ActorCoreWithConditions {
    condition_resolver: ConditionResolver,
    aggregator: Arc<dyn Aggregator>,
    // ... other fields
}

impl ActorCoreWithConditions {
    pub fn new(data_registry: DataProviderRegistry, aggregator: Arc<dyn Aggregator>) -> Self {
        let condition_resolver = ConditionResolver::new(data_registry);
        Self { condition_resolver, aggregator }
    }
    
    // Conditional subsystem activation
    pub async fn should_activate_subsystem(
        &self,
        subsystem: &dyn ConditionalSubsystem,
        actor: &Actor
    ) -> Result<bool, ActorCoreError> {
        if let Some(condition) = subsystem.get_activation_condition() {
            let context = self.create_condition_context(actor).await?;
            let result = self.condition_resolver
                .resolve_condition(&condition, &context)
                .await?;
            Ok(result)
        } else {
            Ok(true) // No condition means always activate
        }
    }
    
    // Resource validation
    pub async fn validate_actor_resources(&self, actor: &Actor) -> Result<bool, ActorCoreError> {
        let health_condition = ConditionBuilder::new()
            .id("health_validation")
            .function("get_actor_resource")
            .parameter("health")
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(1.0))
            .build()?;
        
        let context = self.create_condition_context(actor).await?;
        let result = self.condition_resolver
            .resolve_condition(&health_condition, &context)
            .await?;
        
        Ok(result)
    }
}
```

**Tính năng chính:**
- ✅ **Conditional Subsystems** - Enable/disable subsystems based on conditions
- ✅ **Resource Validation** - Validate actor resources before stat aggregation
- ✅ **Dynamic Stat Modifiers** - Apply conditional stat modifications
- ✅ **Event-Driven Conditions** - React to actor state changes

**Documentation**: [11_Condition_Core_Actor_Core_Integration.md](11_Condition_Core_Actor_Core_Integration.md)

### **2. Action Core Integration** 🚧 **PLANNED**

```rust
use condition_core::{ConditionResolver, DataProviderRegistry, ConditionConfig, ConditionBuilder};

pub struct ActionCore {
    condition_resolver: ConditionResolver,
    // ... other fields
}

impl ActionCore {
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        let condition_resolver = ConditionResolver::new(data_registry);
        Self { condition_resolver }
    }
    
    // Create conditions using builder
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
        
        Ok(conditions)
    }
}
```

### **3. Status Core Integration** 🚧 **PLANNED**

```rust
use condition_core::{ConditionResolver, DataProviderRegistry, ConditionBuilder};

pub struct StatusCore {
    condition_resolver: ConditionResolver,
    // ... other fields
}

impl StatusCore {
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        let condition_resolver = ConditionResolver::new(data_registry);
        Self { condition_resolver }
    }
    
    // Create conditions using builder
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

## 📊 **Performance Benefits**

### **1. No Fallback Mockup**
- ✅ **Clean Error Handling**: Functions throw proper exceptions when no provider
- ✅ **No Performance Overhead**: No fallback logic to slow down evaluation
- ✅ **Clear Dependencies**: Explicit dependency on data providers

### **2. Dependency Injection Architecture**
- ✅ **Plugin Ready**: Easy to add new elements/resources/categories via YAML
- ✅ **Testable**: Easy to mock data providers for testing
- ✅ **Maintainable**: Changes to data don't affect condition logic
- ✅ **Scalable**: Easy to add new data providers for new systems

### **3. Multiple Configuration Methods**
- ✅ **YAML Configuration**: Easy to modify without recompilation
- ✅ **Condition Builder**: Type-safe programmatic creation
- ✅ **Factory Methods**: Common pattern factories
- ✅ **Performance**: Builder is 270x faster than YAML parsing

### **4. Efficient Evaluation**
- ✅ **Direct Data Access**: Functions directly access data providers
- ✅ **No Hard-coded Data**: All data comes from external systems
- ✅ **Type Safety**: Strong typing with proper error handling

## 🎯 **Key Features**

### **1. Simple and Clean Architecture**
- **3 Core Functions**: Resource, Element, Category functions
- **Dependency Injection**: Data providers inject real data
- **No Hard-coded Data**: All data comes from external systems
- **Clean Error Handling**: Proper exceptions when no provider

### **2. Multiple Configuration Methods**
- **YAML Configuration**: String-based configuration
- **Condition Builder**: Programmatic condition creation
- **Factory Methods**: Common pattern factories
- **Condition Chains**: Support for complex logic

### **3. Cross-System Integration**
- **Unified API**: Single API for all systems
- **Consistent Behavior**: Hành vi nhất quán
- **Easy Integration**: Dễ dàng tích hợp
- **Plugin Support**: Hỗ trợ plugin system

### **4. Future-Proof Design**
- **Extensible Architecture**: Dễ dàng extend
- **Plugin Support**: Hỗ trợ plugin system
- **Dependency Injection**: Loose coupling
- **Clean Separation**: Clear separation of concerns

## 📝 **Implementation Status**

### **✅ Completed Features**
1. **Core Architecture**: Condition resolver with dependency injection
2. **Data Provider System**: Interfaces for external systems
3. **YAML Configuration**: String-based configuration support
4. **Condition Builder**: Programmatic condition creation
5. **Factory Methods**: Common pattern factories
6. **Basic Functions**: Resource, Element, Category functions
7. **Error Handling**: Proper error handling without fallback
8. **Examples**: Comprehensive examples and demos
9. **Tests**: Unit and integration tests

### **🔄 Future Enhancements**
1. **More Functions**: Additional condition functions as needed
2. **Caching System**: Performance optimization with caching
3. **Advanced Logic**: More complex condition logic
4. **Performance Monitoring**: Metrics and monitoring
5. **Hot Reload**: Dynamic configuration reloading

## 📚 **Documentation Structure**

### **Updated Documentation**
- `00_Condition_Core_Overview_Updated.md` - Tổng quan về hệ thống
- `01_Condition_Core_Architecture_Design_Updated.md` - Kiến trúc chi tiết
- `02_Condition_Core_Function_Registry_Design_Updated.md` - Function registry
- `06_Condition_Core_API_Design_Updated.md` - API design
- `10_Condition_Builder_Design.md` - Condition Builder design
- `README_Updated_With_Builder.md` - This file

### **Legacy Documentation (Outdated)**
- `00_Condition_Core_Overview.md` - Old overview (complex architecture)
- `01_Condition_Core_Architecture_Design.md` - Old architecture (complex)
- `02_Condition_Core_Function_Registry_Design.md` - Old function registry
- `06_Condition_Core_API_Design.md` - Old API design (complex)

## 🚀 **Quick Start**

1. **Add to Cargo.toml**:
```toml
[dependencies]
condition-core = { path = "crates/condition-core" }
```

2. **Create data providers**:
```rust
use condition_core::*;

// Implement data provider traits
struct MyElementCore;
impl ElementDataProvider for MyElementCore {
    // ... implementation
}
```

3. **Register providers and create resolver**:
```rust
let mut data_registry = DataProviderRegistry::new();
data_registry.register_element_provider(Box::new(MyElementCore));
let resolver = ConditionResolver::new(data_registry);
```

4. **Use YAML configuration**:
```yaml
condition_id: "check_fire_mastery"
function_name: "get_element_mastery"
operator: GreaterThan
value: !Float 100.0
parameters:
  - !String "fire"
```

5. **Use Condition Builder**:
```rust
let condition = ConditionBuilder::new()
    .id("check_fire_mastery")
    .function("get_element_mastery")
    .parameter("fire")
    .operator(ConditionOperator::GreaterThan)
    .value(ConditionValue::Float(100.0))
    .build()?;
```

## 🎉 **Performance Results**

### **Builder vs YAML Performance**
- **YAML Parse Time**: 189.1µs
- **Builder Construct Time**: 700ns
- **Builder is 270x faster** than YAML parsing

### **Key Benefits**
- ✅ **Type Safety**: Compile-time validation
- ✅ **IDE Support**: Auto-completion and error checking
- ✅ **Performance**: No parsing overhead
- ✅ **Flexibility**: Both static and dynamic conditions

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Implementation Complete with Builder  
**Maintainer**: Chaos World Team
