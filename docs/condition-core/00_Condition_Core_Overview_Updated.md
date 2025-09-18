# Condition Core Overview - Updated

## 📋 **Tổng Quan**

Condition Core là hệ thống đơn giản và hiệu quả để resolve conditions trong game, được thiết kế với **Dependency Injection** architecture để hỗ trợ plugin-based system. Hệ thống này cung cấp interface thống nhất cho các systems khác sử dụng trong YAML configurations.

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
- 🔴 **Plugin System Violation**: Hard-coded data vi phạm plugin design

### **2. Lợi Ích của Condition Core**

```
Condition Core Benefits
├── Unified Condition System
│   ├── Single source of truth
│   ├── Consistent logic
│   └── Centralized validation
├── Dependency Injection Architecture
│   ├── No hard-coded data
│   ├── Plugin-ready design
│   └── Easy to extend
├── YAML Configuration Support
│   ├── String-based configuration
│   ├── Easy to modify
│   └── Hot reload support
└── Performance Optimization
    ├── Efficient evaluation
    ├── No fallback mockup
    └── Clean error handling
```

## 🏗️ **Kiến Trúc Condition Core (Actual Implementation)**

### **Core Components**

```
Condition Core (Actual)
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

### **1. Core Types**

```rust
// Main trait for condition resolution
#[async_trait::async_trait]
pub trait ConditionResolverTrait {
    async fn resolve_condition(
        &self,
        condition_config: &ConditionConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool>;
    
    async fn resolve_conditions(
        &self,
        condition_configs: &[ConditionConfig],
        context: &ConditionContext,
    ) -> ConditionResult<Vec<bool>>;
    
    async fn resolve_condition_chain(
        &self,
        chain_config: &ConditionChainConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool>;
}

// Condition configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionConfig {
    pub condition_id: String,
    pub function_name: String,
    pub operator: ConditionOperator,
    pub value: ConditionValue,
    pub parameters: Vec<ConditionParameter>,
}

// Condition context
#[derive(Debug, Clone)]
pub struct ConditionContext {
    pub target: ActorTarget,
    pub world_id: String,
    pub current_time: SystemTime,
    pub current_weather: WeatherType,
    pub world_state: WorldState,
}
```

### **2. Data Provider Architecture**

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

// Function registry
pub struct FunctionRegistry {
    functions: HashMap<String, Box<dyn ConditionFunction>>,
}
```

## 🔧 **YAML Configuration Support**

### **1. Basic YAML Configuration**

```yaml
# Basic condition configuration
condition_id: "check_health"
function_name: "get_actor_resource"
operator: GreaterThan
value: !Float 75.0
parameters:
  - !String "health"
```

### **2. Condition Chain Configuration**

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

## 🚀 **Integration với Existing Systems**

### **1. Action Core Integration**

```rust
// Action Core using Condition Core
use condition_core::{ConditionResolver, DataProviderRegistry, ConditionConfig};

pub struct ActionCore {
    condition_resolver: ConditionResolver,
    // ... other fields
}

impl ActionCore {
    // Create condition resolver with data providers
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        let condition_resolver = ConditionResolver::new(data_registry);
        Self { condition_resolver }
    }
    
    // Evaluate action conditions
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

### **2. Status Core Integration**

```rust
// Status Core using Condition Core
use condition_core::{ConditionResolver, DataProviderRegistry};

pub struct StatusCore {
    condition_resolver: ConditionResolver,
    // ... other fields
}

impl StatusCore {
    // Create condition resolver with data providers
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        let condition_resolver = ConditionResolver::new(data_registry);
        Self { condition_resolver }
    }
    
    // Evaluate status conditions
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

### **3. Element Core Integration**

```rust
// Element Core using Condition Core
use condition_core::{ConditionResolver, DataProviderRegistry};

pub struct ElementCore {
    condition_resolver: ConditionResolver,
    // ... other fields
}

impl ElementCore {
    // Create condition resolver with data providers
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        let condition_resolver = ConditionResolver::new(data_registry);
        Self { condition_resolver }
    }
    
    // Evaluate element conditions
    pub async fn evaluate_element_conditions(
        &self,
        element_id: &str,
        context: &ElementContext
    ) -> Result<bool, ElementError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition_config in &self.get_element_condition_configs(element_id) {
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

## 🎯 **Key Features (Actual Implementation)**

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

## 🚀 **Usage Example**

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

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Implementation Complete  
**Maintainer**: Chaos World Team
