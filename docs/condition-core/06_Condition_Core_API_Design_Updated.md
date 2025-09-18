# Condition Core API Design - Updated

## 📋 **Tổng Quan**

Tài liệu này mô tả API thực tế của Condition Core, dựa trên **Dependency Injection** architecture và **simple, clean** design.

## 🏗️ **API Architecture (Actual)**

### **1. Core API Structure**

```rust
// Main Condition Resolver API
pub struct ConditionResolver {
    function_registry: FunctionRegistry,
    data_registry: DataProviderRegistry,
}

impl ConditionResolver {
    // Create new resolver with data providers
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        let function_registry = create_function_registry_with_providers(&data_registry);
        Self {
            function_registry,
            data_registry,
        }
    }
    
    // Get data provider registry
    pub fn get_data_registry(&self) -> &DataProviderRegistry {
        &self.data_registry
    }
    
    // Get mutable data provider registry
    pub fn get_data_registry_mut(&mut self) -> &mut DataProviderRegistry {
        &mut self.data_registry
    }
}
```

### **2. Condition Resolution API**

```rust
// Condition Resolution API
#[async_trait::async_trait]
impl ConditionResolverTrait for ConditionResolver {
    // Resolve single condition
    async fn resolve_condition(
        &self,
        condition_config: &ConditionConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool> {
        // Get the function
        let function = self.function_registry
            .get(&condition_config.function_name)
            .ok_or_else(|| ConditionError::FunctionNotFound {
                function_name: condition_config.function_name.clone(),
            })?;

        // Evaluate the function
        let result_value = function
            .evaluate(&condition_config.parameters, context)
            .await?;

        // Compare with expected value using operator
        self.compare_values(&result_value, &condition_config.value, &condition_config.operator)
    }
    
    // Resolve multiple conditions
    async fn resolve_conditions(
        &self,
        condition_configs: &[ConditionConfig],
        context: &ConditionContext,
    ) -> ConditionResult<Vec<bool>> {
        let mut results = Vec::new();
        
        for condition_config in condition_configs {
            let result = self.resolve_condition(condition_config, context).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    // Resolve condition chain
    async fn resolve_condition_chain(
        &self,
        chain_config: &ConditionChainConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool> {
        if chain_config.conditions.is_empty() {
            return Err(ConditionError::ChainLogicError {
                message: "Empty condition chain".to_string(),
            });
        }

        // Evaluate all conditions in the chain
        let mut results = Vec::new();
        for condition in &chain_config.conditions {
            let result = self.resolve_condition(condition, context).await?;
            results.push(result);
        }

        // Apply chain logic
        match chain_config.logic {
            ChainLogic::And => Ok(results.iter().all(|&b| b)),
            ChainLogic::Or => Ok(results.iter().any(|&b| b)),
            ChainLogic::Not => {
                if results.len() != 1 {
                    return Err(ConditionError::ChainLogicError {
                        message: "Not operator requires exactly one condition".to_string(),
                    });
                }
                Ok(!results[0])
            }
            ChainLogic::Xor => {
                let true_count = results.iter().filter(|&&b| b).count();
                Ok(true_count == 1)
            }
        }
    }
}
```

## 🔧 **Configuration API**

### **1. YAML Configuration Loading**

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

pub async fn load_condition_chain_config<P: AsRef<Path>>(path: P) -> ConditionResult<ConditionChainConfig> {
    let content = fs::read_to_string(path)?;
    let config: ConditionChainConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

pub fn parse_condition_chain_config(yaml: &str) -> ConditionResult<ConditionChainConfig> {
    let config: ConditionChainConfig = serde_yaml::from_str(yaml)?;
    Ok(config)
}
```

### **2. Configuration Validation**

```rust
// Configuration validation functions
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

pub fn validate_condition_chain_config(config: &ConditionChainConfig) -> ConditionResult<()> {
    if config.chain_id.is_empty() {
        return Err(ConditionError::ConfigError {
            message: "Chain ID cannot be empty".to_string(),
        });
    }

    if config.conditions.is_empty() {
        return Err(ConditionError::ConfigError {
            message: "Chain must have at least one condition".to_string(),
        });
    }

    for condition in &config.conditions {
        validate_condition_config(condition)?;
    }

    Ok(())
}
```

## 🔌 **Data Provider API**

### **1. Data Provider Registration**

```rust
// Data provider registration API
impl DataProviderRegistry {
    // Register element data provider
    pub fn register_element_provider(&mut self, provider: Box<dyn ElementDataProvider>) {
        self.element_provider = Some(Arc::from(provider));
    }

    // Register resource data provider
    pub fn register_resource_provider(&mut self, provider: Box<dyn ResourceDataProvider>) {
        self.resource_provider = Some(Arc::from(provider));
    }

    // Register category data provider
    pub fn register_category_provider(&mut self, provider: Box<dyn CategoryDataProvider>) {
        self.category_provider = Some(Arc::from(provider));
    }
    
    // ... other provider registration methods
}
```

### **2. Data Provider Access**

```rust
// Data provider access API
impl DataProviderRegistry {
    // Get element data provider
    pub fn get_element_provider(&self) -> Option<Arc<dyn ElementDataProvider>> {
        self.element_provider.clone()
    }

    // Get resource data provider
    pub fn get_resource_provider(&self) -> Option<Arc<dyn ResourceDataProvider>> {
        self.resource_provider.clone()
    }

    // Get category data provider
    pub fn get_category_provider(&self) -> Option<Arc<dyn CategoryDataProvider>> {
        self.category_provider.clone()
    }
    
    // ... other provider access methods
}
```

## 🚀 **Usage Examples**

### **1. Basic Usage**

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
        condition_id: "check_health".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(75.0),
        parameters: vec![ConditionParameter::String("health".to_string())],
    };

    // Evaluate condition
    let result = resolver.resolve_condition(&condition, &context).await?;
    println!("Health > 75: {}", result);

    Ok(())
}
```

### **2. YAML Configuration Usage**

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

### **3. Condition Chain Usage**

```rust
use condition_core::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load condition chain from YAML
    let yaml = r#"
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
    "#;

    let chain_config = parse_condition_chain_config(yaml)?;
    
    // Create resolver and context
    let resolver = create_test_resolver();
    let context = create_test_context();
    
    // Evaluate condition chain
    let result = resolver.resolve_condition_chain(&chain_config, &context).await?;
    println!("Complex condition result: {}", result);

    Ok(())
}
```

## 🔧 **Error Handling API**

### **1. Error Types**

```rust
// Error types
#[derive(Error, Debug)]
pub enum ConditionError {
    #[error("Function not found: {function_name}")]
    FunctionNotFound { function_name: String },

    #[error("Invalid parameter: {parameter} for function: {function_name}")]
    InvalidParameter { 
        function_name: String, 
        parameter: String 
    },

    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    #[error("Context error: {message}")]
    ContextError { message: String },

    #[error("Chain logic error: {message}")]
    ChainLogicError { message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Unknown error: {message}")]
    Unknown { message: String },
}

// Result type
pub type ConditionResult<T> = Result<T, ConditionError>;
```

### **2. Error Handling Examples**

```rust
// Error handling in condition evaluation
async fn evaluate_condition_safely(
    resolver: &ConditionResolver,
    condition: &ConditionConfig,
    context: &ConditionContext,
) -> Result<bool, String> {
    match resolver.resolve_condition(condition, context).await {
        Ok(result) => Ok(result),
        Err(ConditionError::FunctionNotFound { function_name }) => {
            Err(format!("Function '{}' not found", function_name))
        }
        Err(ConditionError::InvalidParameter { function_name, parameter }) => {
            Err(format!("Invalid parameter '{}' for function '{}'", parameter, function_name))
        }
        Err(ConditionError::ConfigError { message }) => {
            Err(format!("Configuration error: {}", message))
        }
        Err(e) => Err(format!("Unknown error: {}", e)),
    }
}
```

## 📊 **Performance API**

### **1. Efficient Evaluation**

```rust
// Efficient condition evaluation
impl ConditionResolver {
    // Direct function lookup - O(1)
    async fn evaluate_single_condition(
        &self,
        condition_config: &ConditionConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool> {
        let function = self.function_registry
            .get(&condition_config.function_name)
            .ok_or_else(|| ConditionError::FunctionNotFound {
                function_name: condition_config.function_name.clone(),
            })?;

        let result_value = function
            .evaluate(&condition_config.parameters, context)
            .await?;

        self.compare_values(&result_value, &condition_config.value, &condition_config.operator)
    }
}
```

### **2. Batch Evaluation**

```rust
// Batch condition evaluation
impl ConditionResolver {
    // Evaluate multiple conditions efficiently
    async fn resolve_conditions(
        &self,
        condition_configs: &[ConditionConfig],
        context: &ConditionContext,
    ) -> ConditionResult<Vec<bool>> {
        let mut results = Vec::with_capacity(condition_configs.len());
        
        for condition_config in condition_configs {
            let result = self.evaluate_single_condition(condition_config, context).await?;
            results.push(result);
        }
        
        Ok(results)
    }
}
```

## 🎯 **API Best Practices**

### **1. Clean API Design**

```rust
// Clean API design
impl ConditionResolver {
    // Clear method names
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        // Implementation
    }
    
    // Clear return types
    pub fn get_data_registry(&self) -> &DataProviderRegistry {
        &self.data_registry
    }
    
    // Clear error handling
    pub async fn resolve_condition(
        &self,
        condition_config: &ConditionConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool> {
        // Implementation with proper error handling
    }
}
```

### **2. Consistent Error Handling**

```rust
// Consistent error handling
impl ConditionResolver {
    async fn evaluate_single_condition(
        &self,
        condition_config: &ConditionConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool> {
        // Always check for function existence
        let function = self.function_registry
            .get(&condition_config.function_name)
            .ok_or_else(|| ConditionError::FunctionNotFound {
                function_name: condition_config.function_name.clone(),
            })?;

        // Always propagate errors
        let result_value = function
            .evaluate(&condition_config.parameters, context)
            .await?;

        // Always return consistent result
        self.compare_values(&result_value, &condition_config.value, &condition_config.operator)
    }
}
```

## 📝 **API Status**

### **✅ Completed APIs**
1. **Condition Resolution API**: Single and multiple condition evaluation
2. **Configuration API**: YAML loading and validation
3. **Data Provider API**: Registration and access
4. **Error Handling API**: Comprehensive error types
5. **Performance API**: Efficient evaluation

### **🔄 Future APIs**
1. **Caching API**: Performance optimization
2. **Monitoring API**: Metrics and monitoring
3. **Hot Reload API**: Dynamic configuration updates
4. **Advanced Logic API**: Complex condition chains

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: API Design Complete  
**Maintainer**: Chaos World Team
