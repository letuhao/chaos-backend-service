# Condition Core - Updated Documentation

## 📋 **Tổng Quan**

Condition Core là hệ thống đơn giản và hiệu quả để resolve conditions trong game, được thiết kế với **Dependency Injection** architecture để hỗ trợ plugin-based system.

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
│   └── data_provider.rs    # Data provider interfaces
├── examples/
│   ├── basic_usage.rs      # Basic usage
│   ├── yaml_config.rs      # YAML config example
│   ├── advanced_usage.rs   # Advanced usage
│   ├── dependency_injection.rs # DI example
│   └── architecture_review.rs # Architecture demo
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

## 🔧 **YAML Configuration**

### **Basic Configuration**

```yaml
# Basic condition configuration
condition_id: "check_health"
function_name: "get_actor_resource"
operator: GreaterThan
value: !Float 75.0
parameters:
  - !String "health"
```

### **Condition Chain**

```yaml
# Condition chain configuration
chain_id: "complex_condition"
logic: And
conditions:
  - condition_id: "check_health"
    function_name: "get_actor_resource"
    operator: GreaterThan
    value: !Float 50.0
    parameters:
      - !String "health"
  - condition_id: "check_mana"
    function_name: "get_actor_resource"
    operator: GreaterThan
    value: !Float 25.0
    parameters:
      - !String "mana"
```

## 🚀 **Usage Examples**

### **Basic Usage**

```rust
use condition_core::*;
use std::time::SystemTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create data provider registry
    let mut data_registry = DataProviderRegistry::new();
    data_registry.register_element_provider(Box::new(MyElementCore));
    data_registry.register_resource_provider(Box::new(MyResourceCore));
    data_registry.register_category_provider(Box::new(MyCategoryCore));

    // Create condition resolver
    let resolver = ConditionResolver::new(data_registry);

    // Create condition context
    let context = ConditionContext {
        target: ActorTarget { id: "player_1".to_string() },
        world_id: "test_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Clear,
        world_state: WorldState {
            time_of_day: 12.0,
            season: "summer".to_string(),
            temperature: 25.0,
            humidity: 0.5,
        },
    };

    // Create condition configuration
    let condition = ConditionConfig {
        condition_id: "check_fire_mastery".to_string(),
        function_name: "get_element_mastery".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(100.0),
        parameters: vec![ConditionParameter::String("fire".to_string())],
    };

    // Evaluate condition
    let result = resolver.resolve_condition(&condition, &context).await?;
    println!("Fire mastery > 100: {}", result);

    Ok(())
}
```

### **YAML Configuration Usage**

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

## 🔌 **Integration với Other Systems**

### **Action Core Integration**

```rust
use condition_core::{ConditionResolver, DataProviderRegistry, ConditionConfig};

pub struct ActionCore {
    condition_resolver: ConditionResolver,
    // ... other fields
}

impl ActionCore {
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        let condition_resolver = ConditionResolver::new(data_registry);
        Self { condition_resolver }
    }
    
    pub async fn evaluate_action_conditions(
        &self,
        action: &Action,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition_config in &action.condition_configs {
            let result = self.condition_resolver
                .resolve_condition(condition_config, &condition_context)
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
use condition_core::{ConditionResolver, DataProviderRegistry};

pub struct StatusCore {
    condition_resolver: ConditionResolver,
    // ... other fields
}

impl StatusCore {
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        let condition_resolver = ConditionResolver::new(data_registry);
        Self { condition_resolver }
    }
    
    pub async fn evaluate_status_conditions(
        &self,
        status_effect: &StatusEffect,
        context: &StatusContext
    ) -> Result<bool, StatusError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition_config in &status_effect.condition_configs {
            let result = self.condition_resolver
                .resolve_condition(condition_config, &condition_context)
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

### **1. No Fallback Mockup**
- ✅ **Clean Error Handling**: Functions throw proper exceptions when no provider
- ✅ **No Performance Overhead**: No fallback logic to slow down evaluation
- ✅ **Clear Dependencies**: Explicit dependency on data providers

### **2. Dependency Injection Architecture**
- ✅ **Plugin Ready**: Easy to add new elements/resources/categories via YAML
- ✅ **Testable**: Easy to mock data providers for testing
- ✅ **Maintainable**: Changes to data don't affect condition logic
- ✅ **Scalable**: Easy to add new data providers for new systems

### **3. Efficient Evaluation**
- ✅ **Direct Data Access**: Functions directly access data providers
- ✅ **No Hard-coded Data**: All data comes from external systems
- ✅ **Type Safety**: Strong typing with proper error handling

## 🎯 **Key Features**

### **1. Simple and Clean Architecture**
- **3 Core Functions**: Resource, Element, Category functions
- **Dependency Injection**: Data providers inject real data
- **No Hard-coded Data**: All data comes from external systems
- **Clean Error Handling**: Proper exceptions when no provider

### **2. YAML Configuration Support**
- **String-based Configuration**: Easy to modify
- **Condition Chains**: Support for complex logic
- **Type Safety**: Proper YAML tag support
- **Validation**: Built-in configuration validation

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
4. **Basic Functions**: Resource, Element, Category functions
5. **Error Handling**: Proper error handling without fallback
6. **Examples**: Comprehensive examples and demos
7. **Tests**: Unit and integration tests

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
- `README_Updated.md` - This file

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

4. **Use in YAML configurations**:
```yaml
condition_id: "check_fire_mastery"
function_name: "get_element_mastery"
operator: GreaterThan
value: !Float 100.0
parameters:
  - !String "fire"
```

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Implementation Complete  
**Maintainer**: Chaos World Team
