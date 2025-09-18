# Condition Core Architecture Design - Updated

## 📋 **Tổng Quan**

Tài liệu này mô tả kiến trúc thực tế của Condition Core, dựa trên **Dependency Injection** pattern và **plugin-based** design để hỗ trợ extensibility mà không vi phạm nguyên tắc SOLID.

## 🏗️ **Actual Architecture**

### **1. Core Components (Implementation)**

```rust
// Condition Core Main Structure (Actual)
pub struct ConditionResolver {
    function_registry: FunctionRegistry,
    data_registry: DataProviderRegistry,
}

// Function Registry
pub struct FunctionRegistry {
    functions: HashMap<String, Box<dyn ConditionFunction>>,
}

// Data Provider Registry
pub struct DataProviderRegistry {
    element_provider: Option<Arc<dyn ElementDataProvider>>,
    resource_provider: Option<Arc<dyn ResourceDataProvider>>,
    category_provider: Option<Arc<dyn CategoryDataProvider>>,
    status_provider: Option<Arc<dyn StatusDataProvider>>,
    action_provider: Option<Arc<dyn ActionDataProvider>>,
    location_provider: Option<Arc<dyn LocationDataProvider>>,
    event_provider: Option<Arc<dyn EventDataProvider>>,
    quest_provider: Option<Arc<dyn QuestDataProvider>>,
    actor_provider: Option<Arc<dyn ActorDataProvider>>,
    item_provider: Option<Arc<dyn ItemDataProvider>>,
    shield_provider: Option<Arc<dyn ShieldDataProvider>>,
}
```

### **2. Condition Definition Structure (Actual)**

```rust
// Condition Configuration (Actual Implementation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionConfig {
    pub condition_id: String,
    pub function_name: String,
    pub operator: ConditionOperator,
    pub value: ConditionValue,
    pub parameters: Vec<ConditionParameter>,
}

// Condition Parameters (Actual)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionParameter {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

// Condition Operators (Actual)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    NotContains,
    In,
    NotIn,
}

// Condition Values (Actual)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<ConditionValue>),
}
```

### **3. Condition Context (Actual)**

```rust
// Condition Context (Actual Implementation)
#[derive(Debug, Clone)]
pub struct ConditionContext {
    pub target: ActorTarget,
    pub world_id: String,
    pub current_time: SystemTime,
    pub current_weather: WeatherType,
    pub world_state: WorldState,
}

// Actor Target
#[derive(Debug, Clone)]
pub struct ActorTarget {
    pub id: String,
}

// Weather Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherType {
    Clear,
    Rain,
    Snow,
    Storm,
    Fog,
}

// World State
#[derive(Debug, Clone)]
pub struct WorldState {
    pub time_of_day: f64,
    pub season: String,
    pub temperature: f64,
    pub humidity: f64,
}
```

## 🔌 **Dependency Injection Architecture**

### **1. Data Provider Interfaces**

```rust
// Element Data Provider
#[async_trait::async_trait]
pub trait ElementDataProvider: Send + Sync {
    async fn get_element_mastery(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn get_element_resistance(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn has_element_affinity(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn is_element_weakness(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String>;
    async fn list_elements(&self) -> ConditionResult<Vec<String>>;
}

// Resource Data Provider
#[async_trait::async_trait]
pub trait ResourceDataProvider: Send + Sync {
    async fn get_resource_value(&self, resource_id: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn get_resource_max(&self, resource_id: &str, actor_id: &str) -> ConditionResult<f64>;
    async fn is_resource_low(&self, resource_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn list_resources(&self) -> ConditionResult<Vec<String>>;
}

// Category Data Provider
#[async_trait::async_trait]
pub trait CategoryDataProvider: Send + Sync {
    async fn has_category_item(&self, category_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn get_category_item_count(&self, category_id: &str, actor_id: &str) -> ConditionResult<i64>;
    async fn is_category_available(&self, category_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn is_category_blocked(&self, category_id: &str, actor_id: &str) -> ConditionResult<bool>;
    async fn list_categories(&self) -> ConditionResult<Vec<String>>;
}
```

### **2. Condition Functions (Actual)**

```rust
// Generic Resource Function
pub struct GetActorResourceFunction {
    data_provider: Option<Arc<dyn ResourceDataProvider>>,
}

impl GetActorResourceFunction {
    pub fn new(data_provider: Option<Arc<dyn ResourceDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetActorResourceFunction {
    fn name(&self) -> &str {
        "get_actor_resource"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Resource data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(resource_type)) = parameters.first() {
            let value = provider.get_resource_value(resource_type, &context.target.id).await?;
            Ok(ConditionValue::Float(value))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "resource_type".to_string(),
            })
        }
    }
}
```

### **3. Function Registry (Actual)**

```rust
// Function Registry Implementation
pub struct FunctionRegistry {
    functions: HashMap<String, Box<dyn ConditionFunction>>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn register(&mut self, function: Box<dyn ConditionFunction>) {
        self.functions.insert(function.name().to_string(), function);
    }

    pub fn get(&self, name: &str) -> Option<&dyn ConditionFunction> {
        self.functions.get(name).map(|f| f.as_ref())
    }

    pub fn list(&self) -> Vec<&str> {
        self.functions.keys().map(|k| k.as_str()).collect()
    }
}

// Create function registry with data providers
pub fn create_function_registry_with_providers(
    data_registry: &DataProviderRegistry,
) -> FunctionRegistry {
    let mut registry = FunctionRegistry::new();
    
    // Register functions with data providers
    registry.register(Box::new(GetActorResourceFunction::new(
        data_registry.get_resource_provider()
    )));
    
    registry.register(Box::new(GetElementMasteryFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(HasCategoryItemFunction::new(
        data_registry.get_category_provider()
    )));
    
    registry
}
```

## 🔧 **YAML Configuration System**

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

### **3. YAML Configuration Loading**

```rust
// Configuration loading functions
pub async fn load_condition_config<P: AsRef<Path>>(path: P) -> ConditionResult<ConditionConfig> {
    let content = fs::read_to_string(path)?;
    let config: ConditionConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

pub fn parse_condition_config(yaml: &str) -> ConditionResult<ConditionConfig> {
    let config: ConditionConfig = serde_yaml::from_str(yaml)?;
    Ok(config)
}

pub fn validate_condition_config(config: &ConditionConfig) -> ConditionResult<()> {
    if config.condition_id.is_empty() {
        return Err(ConditionError::ConfigError {
            message: "Condition ID cannot be empty".to_string(),
        });
    }

    if config.function_name.is_empty() {
        return Err(ConditionError::ConfigError {
            message: "Function name cannot be empty".to_string(),
        });
    }

    Ok(())
}
```

## 🚀 **Integration Architecture**

### **1. System Integration Pattern**

```rust
// Integration pattern for other systems
pub struct SystemIntegration {
    condition_resolver: ConditionResolver,
    data_registry: DataProviderRegistry,
}

impl SystemIntegration {
    // Create integration with data providers
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        let condition_resolver = ConditionResolver::new(data_registry.clone());
        Self {
            condition_resolver,
            data_registry,
        }
    }
    
    // Evaluate conditions
    pub async fn evaluate_conditions(
        &self,
        condition_configs: &[ConditionConfig],
        context: &ConditionContext
    ) -> Result<Vec<bool>, ConditionError> {
        self.condition_resolver.resolve_conditions(condition_configs, context).await
    }
}
```

### **2. Data Provider Registration**

```rust
// Data provider registration example
pub fn setup_condition_core() -> ConditionResolver {
    let mut data_registry = DataProviderRegistry::new();
    
    // Register data providers from other systems
    data_registry.register_element_provider(Box::new(ElementCore::new()));
    data_registry.register_resource_provider(Box::new(ResourceCore::new()));
    data_registry.register_category_provider(Box::new(CategoryCore::new()));
    
    // Create condition resolver
    ConditionResolver::new(data_registry)
}
```

## 📊 **Performance Architecture**

### **1. No Fallback Mockup Design**

```rust
// Clean error handling without fallback
impl ConditionFunction for GetActorResourceFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        // No fallback - throw error if no provider
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Resource data provider not available".to_string(),
            })?;

        // Direct data access
        if let Some(ConditionParameter::String(resource_type)) = parameters.first() {
            let value = provider.get_resource_value(resource_type, &context.target.id).await?;
            Ok(ConditionValue::Float(value))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "resource_type".to_string(),
            })
        }
    }
}
```

### **2. Efficient Data Access**

```rust
// Direct data access through providers
pub struct ConditionResolver {
    function_registry: FunctionRegistry,
    data_registry: DataProviderRegistry, // Arc for efficient cloning
}

impl ConditionResolver {
    // Get data registry for external access
    pub fn get_data_registry(&self) -> &DataProviderRegistry {
        &self.data_registry
    }
    
    // Get mutable data registry for updates
    pub fn get_data_registry_mut(&mut self) -> &mut DataProviderRegistry {
        &mut self.data_registry
    }
}
```

## 🎯 **Key Architectural Principles**

### **1. Single Responsibility Principle (SRP)**
- **ConditionResolver**: Only resolves conditions
- **FunctionRegistry**: Only manages functions
- **DataProviderRegistry**: Only manages data providers
- **ConditionFunction**: Only evaluates specific condition

### **2. Open/Closed Principle (OCP)**
- **Open for Extension**: Easy to add new functions and providers
- **Closed for Modification**: Core logic doesn't change

### **3. Dependency Inversion Principle (DIP)**
- **High-level modules**: Don't depend on low-level modules
- **Abstractions**: Don't depend on details
- **Details**: Depend on abstractions

### **4. Interface Segregation Principle (ISP)**
- **Specific interfaces**: Each data provider has specific interface
- **No fat interfaces**: Interfaces are focused and minimal

### **5. Liskov Substitution Principle (LSP)**
- **Substitutability**: Any data provider can be substituted
- **Behavioral compatibility**: All providers behave consistently

## 🔄 **Extension Points**

### **1. Adding New Functions**

```rust
// Add new condition function
pub struct NewConditionFunction {
    data_provider: Option<Arc<dyn NewDataProvider>>,
}

impl NewConditionFunction {
    pub fn new(data_provider: Option<Arc<dyn NewDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for NewConditionFunction {
    fn name(&self) -> &str {
        "new_condition_function"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        // Implementation
    }
}
```

### **2. Adding New Data Providers**

```rust
// Add new data provider interface
#[async_trait::async_trait]
pub trait NewDataProvider: Send + Sync {
    async fn get_new_data(&self, id: &str) -> ConditionResult<String>;
}

// Add to DataProviderRegistry
pub struct DataProviderRegistry {
    // ... existing providers
    new_provider: Option<Arc<dyn NewDataProvider>>,
}
```

## 📝 **Implementation Status**

### **✅ Completed Components**
1. **Core Types**: All basic types and traits
2. **Data Provider System**: Complete interface system
3. **Function Registry**: Basic function management
4. **YAML Configuration**: String-based configuration
5. **Error Handling**: Comprehensive error system
6. **Examples**: Complete examples and demos
7. **Tests**: Unit and integration tests

### **🔄 Future Enhancements**
1. **More Functions**: Additional condition functions
2. **Caching System**: Performance optimization
3. **Advanced Logic**: Complex condition chains
4. **Performance Monitoring**: Metrics and monitoring
5. **Hot Reload**: Dynamic configuration updates

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Architecture Complete  
**Maintainer**: Chaos World Team
