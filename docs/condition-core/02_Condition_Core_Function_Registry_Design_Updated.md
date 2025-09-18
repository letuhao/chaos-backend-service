# Condition Core Function Registry Design - Updated

## 📋 **Tổng Quan**

Tài liệu này mô tả thiết kế Function Registry thực tế của Condition Core, dựa trên **Dependency Injection** pattern để hỗ trợ plugin-based system.

## 🏗️ **Function Registry Architecture (Actual)**

### **1. Core Structure**

```rust
// Function Registry (Actual Implementation)
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
```

### **2. Condition Function Trait**

```rust
// Condition Function Trait (Actual)
#[async_trait::async_trait]
pub trait ConditionFunction: Send + Sync {
    fn name(&self) -> &str;
    
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue>;
}
```

## 🔌 **Available Functions (Current Implementation)**

### **1. Resource Functions**

```rust
// Get Actor Resource Function
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

**Usage Example:**
```yaml
condition_id: "check_health"
function_name: "get_actor_resource"
operator: GreaterThan
value: !Float 75.0
parameters:
  - !String "health"
```

### **2. Element Functions**

```rust
// Get Element Mastery Function
pub struct GetElementMasteryFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl GetElementMasteryFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetElementMasteryFunction {
    fn name(&self) -> &str {
        "get_element_mastery"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(element_type)) = parameters.first() {
            let mastery = provider.get_element_mastery(element_type, &context.target.id).await?;
            Ok(ConditionValue::Float(mastery))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_type".to_string(),
            })
        }
    }
}
```

**Usage Example:**
```yaml
condition_id: "check_fire_mastery"
function_name: "get_element_mastery"
operator: GreaterThan
value: !Float 100.0
parameters:
  - !String "fire"
```

### **3. Category Functions**

```rust
// Has Category Item Function
pub struct HasCategoryItemFunction {
    data_provider: Option<Arc<dyn CategoryDataProvider>>,
}

impl HasCategoryItemFunction {
    pub fn new(data_provider: Option<Arc<dyn CategoryDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasCategoryItemFunction {
    fn name(&self) -> &str {
        "has_category_item"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Category data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(category)) = parameters.first() {
            let has_item = provider.has_category_item(category, &context.target.id).await?;
            Ok(ConditionValue::Boolean(has_item))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "category".to_string(),
            })
        }
    }
}
```

**Usage Example:**
```yaml
condition_id: "has_weapon"
function_name: "has_category_item"
operator: Equal
value: !Boolean true
parameters:
  - !String "weapon"
```

## 🔧 **Function Registry Management**

### **1. Function Registration**

```rust
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

### **2. Function Discovery**

```rust
// Function discovery and listing
impl FunctionRegistry {
    // List all registered functions
    pub fn list(&self) -> Vec<&str> {
        self.functions.keys().map(|k| k.as_str()).collect()
    }
    
    // Check if function exists
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
    
    // Get function count
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }
}
```

### **3. Function Execution**

```rust
// Function execution through registry
impl FunctionRegistry {
    // Execute function by name
    pub async fn execute_function(
        &self,
        name: &str,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let function = self.get(name)
            .ok_or_else(|| ConditionError::FunctionNotFound {
                function_name: name.to_string(),
            })?;
        
        function.evaluate(parameters, context).await
    }
}
```

## 🚀 **Adding New Functions**

### **1. Create New Function**

```rust
// Example: Add new function for checking status effects
pub struct HasStatusEffectFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl HasStatusEffectFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasStatusEffectFunction {
    fn name(&self) -> &str {
        "has_status_effect"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(status_id)) = parameters.first() {
            let has_status = provider.has_status_effect(status_id, &context.target.id).await?;
            Ok(ConditionValue::Boolean(has_status))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "status_id".to_string(),
            })
        }
    }
}
```

### **2. Register New Function**

```rust
// Register new function in registry
pub fn create_extended_function_registry(
    data_registry: &DataProviderRegistry,
) -> FunctionRegistry {
    let mut registry = create_function_registry_with_providers(data_registry);
    
    // Add new function
    registry.register(Box::new(HasStatusEffectFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry
}
```

### **3. Usage Example**

```yaml
# Use new function in YAML
condition_id: "has_burning_status"
function_name: "has_status_effect"
operator: Equal
value: !Boolean true
parameters:
  - !String "burning"
```

## 📊 **Function Performance**

### **1. No Fallback Mockup**

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

        // Direct data access - no performance overhead
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
pub struct FunctionRegistry {
    functions: HashMap<String, Box<dyn ConditionFunction>>, // Fast lookup
}

impl FunctionRegistry {
    // O(1) function lookup
    pub fn get(&self, name: &str) -> Option<&dyn ConditionFunction> {
        self.functions.get(name).map(|f| f.as_ref())
    }
}
```

## 🔄 **Function Lifecycle**

### **1. Function Creation**

```rust
// Function creation with dependency injection
pub fn create_function_with_provider<T: ConditionFunction + 'static>(
    function: T,
    data_provider: Option<Arc<dyn DataProvider>>
) -> Box<dyn ConditionFunction> {
    Box::new(function)
}
```

### **2. Function Registration**

```rust
// Function registration in registry
pub fn register_function(
    registry: &mut FunctionRegistry,
    function: Box<dyn ConditionFunction>
) -> Result<(), ConditionError> {
    let name = function.name().to_string();
    
    if registry.has_function(&name) {
        return Err(ConditionError::ConfigError {
            message: format!("Function '{}' already registered", name),
        });
    }
    
    registry.register(function);
    Ok(())
}
```

### **3. Function Execution**

```rust
// Function execution with error handling
pub async fn execute_function_safely(
    registry: &FunctionRegistry,
    name: &str,
    parameters: &[ConditionParameter],
    context: &ConditionContext,
) -> ConditionResult<ConditionValue> {
    let function = registry.get(name)
        .ok_or_else(|| ConditionError::FunctionNotFound {
            function_name: name.to_string(),
        })?;
    
    function.evaluate(parameters, context).await
}
```

## 🎯 **Best Practices**

### **1. Function Design**

```rust
// Good function design
pub struct GoodFunction {
    data_provider: Option<Arc<dyn DataProvider>>,
}

impl GoodFunction {
    // Clear constructor
    pub fn new(data_provider: Option<Arc<dyn DataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GoodFunction {
    // Clear function name
    fn name(&self) -> &str {
        "clear_function_name"
    }

    // Proper error handling
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        // Check provider availability
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Data provider not available".to_string(),
            })?;

        // Validate parameters
        if parameters.is_empty() {
            return Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "parameters".to_string(),
            });
        }

        // Execute logic
        // ... implementation
    }
}
```

### **2. Error Handling**

```rust
// Comprehensive error handling
impl ConditionFunction for ExampleFunction {
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        // Provider check
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Data provider not available".to_string(),
            })?;

        // Parameter validation
        let param = parameters.first()
            .ok_or_else(|| ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "first_parameter".to_string(),
            })?;

        // Type checking
        let string_param = match param {
            ConditionParameter::String(s) => s,
            _ => return Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "string_parameter".to_string(),
            }),
        };

        // Execute with error propagation
        let result = provider.get_data(string_param, &context.target.id).await?;
        Ok(ConditionValue::String(result))
    }
}
```

## 📝 **Implementation Status**

### **✅ Completed Functions**
1. **GetActorResourceFunction**: Generic resource access
2. **GetElementMasteryFunction**: Element mastery access
3. **HasCategoryItemFunction**: Category item checking

### **🔄 Future Functions**
1. **Status Functions**: Status effect checking
2. **Action Functions**: Action availability checking
3. **Location Functions**: Location-based conditions
4. **Time Functions**: Time-based conditions
5. **Weather Functions**: Weather-based conditions

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Function Registry Complete  
**Maintainer**: Chaos World Team
