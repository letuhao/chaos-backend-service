# Condition Core Configuration System Design

## 📋 **Tổng Quan**

Tài liệu này thiết kế hệ thống configuration cho Condition Core, hỗ trợ cả YAML string-based và class/interface-based configuration, tương tự Skyrim's plugin system.

## 🏗️ **Configuration Architecture**

### **1. Configuration Hierarchy**

```
Condition Core Configuration
├── Global Configuration
│   ├── condition_core_config.yaml
│   ├── function_registry_config.yaml
│   └── performance_config.yaml
├── Function Categories
│   ├── actor_functions.yaml
│   ├── item_functions.yaml
│   ├── location_functions.yaml
│   ├── time_functions.yaml
│   ├── weather_functions.yaml
│   ├── magic_functions.yaml
│   ├── relationship_functions.yaml
│   └── custom_functions.yaml
├── Condition Definitions
│   ├── conditions/
│   │   ├── actor_conditions.yaml
│   │   ├── item_conditions.yaml
│   │   ├── location_conditions.yaml
│   │   ├── time_conditions.yaml
│   │   ├── weather_conditions.yaml
│   │   ├── magic_conditions.yaml
│   │   ├── relationship_conditions.yaml
│   │   └── custom_conditions.yaml
│   └── samples/
│       ├── health_condition.yaml
│       ├── combat_condition.yaml
│       ├── inventory_condition.yaml
│       └── location_condition.yaml
├── Templates
│   ├── condition_templates.yaml
│   ├── function_templates.yaml
│   └── validation_templates.yaml
└── Plugins
    ├── master/
    ├── plugins/
    ├── mods/
    └── user/
```

### **2. Configuration Types**

```rust
// Configuration Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigurationType {
    YAML,           // YAML string-based
    Interface,      // Class/interface-based
    Hybrid,         // Combined approach
    Plugin,         // Plugin-based
}

// Configuration Manager
pub struct ConditionConfigManager {
    config_type: ConfigurationType,
    yaml_parser: YamlConfigParser,
    interface_parser: InterfaceConfigParser,
    hybrid_parser: HybridConfigParser,
    plugin_parser: PluginConfigParser,
    config_cache: ConfigCache,
    config_validator: ConfigValidator,
}
```

## 🔧 **YAML Configuration System**

### **1. Global Configuration**

```yaml
# condition_core_config.yaml
condition_core:
  version: "1.0.0"
  world_id: "chaos_world"
  
  # Performance Settings
  performance:
    max_concurrent_evaluations: 1000
    cache_size: 10000
    cache_ttl: 300
    batch_size: 100
    evaluation_timeout: 5000
    
  # Function Registry Settings
  function_registry:
    enable_caching: true
    enable_validation: true
    enable_monitoring: true
    max_functions: 1000
    
  # Condition Settings
  conditions:
    enable_caching: true
    enable_validation: true
    enable_monitoring: true
    max_conditions: 10000
    
  # Integration Settings
  integration:
    action_core: true
    status_core: true
    element_core: true
    effect_core: true
    talent_core: false
    perk_core: false
```

### **2. Function Registry Configuration**

```yaml
# function_registry_config.yaml
function_registry:
  version: "1.0.0"
  
  # Function Categories
  categories:
    actor:
      enabled: true
      functions: 25
      cache_ttl: 30
      performance_impact: "low"
      
    item:
      enabled: true
      functions: 15
      cache_ttl: 60
      performance_impact: "low"
      
    location:
      enabled: true
      functions: 20
      cache_ttl: 60
      performance_impact: "medium"
      
    time:
      enabled: true
      functions: 10
      cache_ttl: 300
      performance_impact: "low"
      
    weather:
      enabled: true
      functions: 8
      cache_ttl: 120
      performance_impact: "low"
      
    magic:
      enabled: true
      functions: 15
      cache_ttl: 30
      performance_impact: "medium"
      
    relationship:
      enabled: true
      functions: 12
      cache_ttl: 60
      performance_impact: "low"
      
    custom:
      enabled: true
      functions: 10
      cache_ttl: 60
      performance_impact: "medium"
  
  # Function Definitions
  functions:
    # Actor Functions
    get_actor_value:
      category: "actor"
      description: "Get actor's stat value"
      description_vi: "Lấy giá trị thống kê của diễn viên"
      parameter_types: ["string"]
      return_type: "float"
      cacheable: true
      cache_ttl: 30
      performance_impact: "low"
      
    is_in_combat:
      category: "actor"
      description: "Check if actor is in combat"
      description_vi: "Kiểm tra diễn viên có đang trong chiến đấu"
      parameter_types: []
      return_type: "boolean"
      cacheable: true
      cache_ttl: 10
      performance_impact: "low"
      
    # Item Functions
    has_item:
      category: "item"
      description: "Check if actor has item"
      description_vi: "Kiểm tra diễn viên có vật phẩm"
      parameter_types: ["string"]
      return_type: "boolean"
      cacheable: true
      cache_ttl: 60
      performance_impact: "low"
      
    # Location Functions
    get_in_current_location:
      category: "location"
      description: "Check if actor is in location"
      description_vi: "Kiểm tra diễn viên có trong vị trí"
      parameter_types: ["string"]
      return_type: "boolean"
      cacheable: true
      cache_ttl: 60
      performance_impact: "low"
```

### **3. Condition Definition Configuration**

```yaml
# conditions/actor_conditions.yaml
actor_conditions:
  version: "1.0.0"
  
  # Health Conditions
  health_condition:
    condition_guid: "health_condition_001"
    condition_id: "health_condition"
    condition_name: "Health Condition"
    condition_name_vi: "Điều kiện máu"
    world_id: "chaos_world"
    
    condition_function: "get_actor_value"
    condition_parameters:
      - parameter_type: "string"
        parameter_value: "health"
    condition_operator: "less_than"
    condition_value:
      value_type: "float"
      value: 0.5
    condition_logic: "AND"
    
    categories: ["actor", "health", "combat"]
    priority: 100
    cacheable: true
    cache_ttl: 30
    performance_impact: "low"
    
  # Combat Conditions
  combat_condition:
    condition_guid: "combat_condition_001"
    condition_id: "combat_condition"
    condition_name: "Combat Condition"
    condition_name_vi: "Điều kiện chiến đấu"
    world_id: "chaos_world"
    
    condition_function: "is_in_combat"
    condition_parameters: []
    condition_operator: "equal"
    condition_value:
      value_type: "boolean"
      value: true
    condition_logic: "AND"
    
    categories: ["actor", "combat"]
    priority: 90
    cacheable: true
    cache_ttl: 10
    performance_impact: "low"
    
  # Level Conditions
  level_condition:
    condition_guid: "level_condition_001"
    condition_id: "level_condition"
    condition_name: "Level Condition"
    condition_name_vi: "Điều kiện cấp độ"
    world_id: "chaos_world"
    
    condition_function: "get_level"
    condition_parameters: []
    condition_operator: "greater_than_or_equal"
    condition_value:
      value_type: "integer"
      value: 10
    condition_logic: "AND"
    
    categories: ["actor", "level"]
    priority: 80
    cacheable: true
    cache_ttl: 300
    performance_impact: "low"
```

### **4. Sample Condition Configurations**

```yaml
# samples/health_condition.yaml
health_condition:
  condition_guid: "health_condition_sample_001"
  condition_id: "health_condition_sample"
  condition_name: "Health Condition Sample"
  condition_name_vi: "Mẫu điều kiện máu"
  world_id: "chaos_world"
  
  condition_function: "get_actor_value"
  condition_parameters:
    - parameter_type: "string"
      parameter_value: "health"
  condition_operator: "less_than"
  condition_value:
    value_type: "float"
    value: 0.5
  condition_logic: "AND"
  
  categories: ["actor", "health", "combat"]
  priority: 100
  cacheable: true
  cache_ttl: 30
  performance_impact: "low"
  
  # Metadata
  metadata:
    description: "Check if actor health is below 50%"
    description_vi: "Kiểm tra máu diễn viên dưới 50%"
    author: "Chaos World Team"
    version: "1.0.0"
    created_at: "2025-01-27T00:00:00Z"
    updated_at: "2025-01-27T00:00:00Z"
    
  # Validation Rules
  validation:
    required_parameters: ["stat_name"]
    parameter_types: ["string"]
    return_type: "float"
    operator_types: ["less_than", "less_than_or_equal", "greater_than", "greater_than_or_equal"]
    
  # Performance Settings
  performance:
    cacheable: true
    cache_ttl: 30
    performance_impact: "low"
    evaluation_timeout: 1000
```

## 🔧 **Interface Configuration System**

### **1. Interface Configuration Trait**

```rust
// Interface Configuration Trait
pub trait ConditionInterface {
    fn get_condition_guid(&self) -> String;
    fn get_condition_id(&self) -> String;
    fn get_condition_name(&self) -> String;
    fn get_condition_name_vi(&self) -> String;
    fn get_world_id(&self) -> String;
    
    fn get_condition_function(&self) -> String;
    fn get_condition_parameters(&self) -> Vec<ConditionParameter>;
    fn get_condition_operator(&self) -> ConditionOperator;
    fn get_condition_value(&self) -> ConditionValue;
    fn get_condition_logic(&self) -> ConditionLogic;
    
    fn get_categories(&self) -> Vec<String>;
    fn get_priority(&self) -> u32;
    fn is_cacheable(&self) -> bool;
    fn get_cache_ttl(&self) -> Option<Duration>;
    fn get_performance_impact(&self) -> PerformanceImpact;
    
    fn evaluate(&self, context: &ConditionContext) -> Result<bool, ConditionError>;
}

// Interface Configuration Parser
pub struct InterfaceConfigParser {
    interface_registry: HashMap<String, Box<dyn ConditionInterface>>,
    interface_metadata: HashMap<String, InterfaceMetadata>,
}

impl InterfaceConfigParser {
    // Register interface
    pub fn register_interface<T: ConditionInterface + 'static>(
        &mut self,
        interface: T
    ) -> Result<(), ConditionError> {
        let condition_id = interface.get_condition_id();
        let metadata = InterfaceMetadata {
            condition_guid: interface.get_condition_guid(),
            condition_id: condition_id.clone(),
            condition_name: interface.get_condition_name(),
            condition_name_vi: interface.get_condition_name_vi(),
            world_id: interface.get_world_id(),
            categories: interface.get_categories(),
            priority: interface.get_priority(),
            cacheable: interface.is_cacheable(),
            cache_ttl: interface.get_cache_ttl(),
            performance_impact: interface.get_performance_impact(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        
        self.interface_registry.insert(condition_id.clone(), Box::new(interface));
        self.interface_metadata.insert(condition_id, metadata);
        
        Ok(())
    }
    
    // Get interface
    pub fn get_interface(&self, condition_id: &str) -> Option<&dyn ConditionInterface> {
        self.interface_registry.get(condition_id).map(|i| i.as_ref())
    }
    
    // List interfaces
    pub fn list_interfaces(&self) -> Vec<String> {
        self.interface_registry.keys().cloned().collect()
    }
}
```

### **2. Interface Implementation Examples**

```rust
// Health Condition Interface
pub struct HealthConditionInterface {
    threshold: f64,
    operator: ConditionOperator,
}

impl HealthConditionInterface {
    pub fn new(threshold: f64, operator: ConditionOperator) -> Self {
        Self { threshold, operator }
    }
}

impl ConditionInterface for HealthConditionInterface {
    fn get_condition_guid(&self) -> String {
        "health_condition_interface_001".to_string()
    }
    
    fn get_condition_id(&self) -> String {
        "health_condition_interface".to_string()
    }
    
    fn get_condition_name(&self) -> String {
        "Health Condition Interface".to_string()
    }
    
    fn get_condition_name_vi(&self) -> String {
        "Giao diện điều kiện máu".to_string()
    }
    
    fn get_world_id(&self) -> String {
        "chaos_world".to_string()
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
    
    fn get_categories(&self) -> Vec<String> {
        vec!["actor".to_string(), "health".to_string(), "combat".to_string()]
    }
    
    fn get_priority(&self) -> u32 {
        100
    }
    
    fn is_cacheable(&self) -> bool {
        true
    }
    
    fn get_cache_ttl(&self) -> Option<Duration> {
        Some(Duration::from_secs(30))
    }
    
    fn get_performance_impact(&self) -> PerformanceImpact {
        PerformanceImpact::Low
    }
    
    fn evaluate(&self, context: &ConditionContext) -> Result<bool, ConditionError> {
        let health_value = context.get_actor_value("health")?;
        Ok(self.operator.compare(health_value, self.threshold))
    }
}

// Combat Condition Interface
pub struct CombatConditionInterface {
    in_combat: bool,
}

impl CombatConditionInterface {
    pub fn new(in_combat: bool) -> Self {
        Self { in_combat }
    }
}

impl ConditionInterface for CombatConditionInterface {
    fn get_condition_guid(&self) -> String {
        "combat_condition_interface_001".to_string()
    }
    
    fn get_condition_id(&self) -> String {
        "combat_condition_interface".to_string()
    }
    
    fn get_condition_name(&self) -> String {
        "Combat Condition Interface".to_string()
    }
    
    fn get_condition_name_vi(&self) -> String {
        "Giao diện điều kiện chiến đấu".to_string()
    }
    
    fn get_world_id(&self) -> String {
        "chaos_world".to_string()
    }
    
    fn get_condition_function(&self) -> String {
        "is_in_combat".to_string()
    }
    
    fn get_condition_parameters(&self) -> Vec<ConditionParameter> {
        vec![]
    }
    
    fn get_condition_operator(&self) -> ConditionOperator {
        ConditionOperator::Equal
    }
    
    fn get_condition_value(&self) -> ConditionValue {
        ConditionValue::Boolean(self.in_combat)
    }
    
    fn get_condition_logic(&self) -> ConditionLogic {
        ConditionLogic::And
    }
    
    fn get_categories(&self) -> Vec<String> {
        vec!["actor".to_string(), "combat".to_string()]
    }
    
    fn get_priority(&self) -> u32 {
        90
    }
    
    fn is_cacheable(&self) -> bool {
        true
    }
    
    fn get_cache_ttl(&self) -> Option<Duration> {
        Some(Duration::from_secs(10))
    }
    
    fn get_performance_impact(&self) -> PerformanceImpact {
        PerformanceImpact::Low
    }
    
    fn evaluate(&self, context: &ConditionContext) -> Result<bool, ConditionError> {
        let is_in_combat = context.is_actor_in_combat()?;
        Ok(is_in_combat == self.in_combat)
    }
}
```

## 🔧 **Hybrid Configuration System**

### **1. Hybrid Configuration Structure**

```rust
// Hybrid Configuration
pub struct HybridCondition {
    pub yaml_config: Option<YamlCondition>,
    pub interface_config: Option<Box<dyn ConditionInterface>>,
    pub evaluation_strategy: EvaluationStrategy,
    pub fallback_strategy: FallbackStrategy,
}

// Evaluation Strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvaluationStrategy {
    YamlFirst,      // Try YAML first, fallback to interface
    InterfaceFirst, // Try interface first, fallback to YAML
    Hybrid,         // Use both and combine results
}

// Fallback Strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FallbackStrategy {
    Interface,      // Fallback to interface
    Yaml,           // Fallback to YAML
    Error,          // Return error
}

// Hybrid Configuration Parser
pub struct HybridConfigParser {
    yaml_parser: YamlConfigParser,
    interface_parser: InterfaceConfigParser,
    hybrid_conditions: HashMap<String, HybridCondition>,
}

impl HybridConfigParser {
    // Create from YAML with interface fallback
    pub fn from_yaml_with_fallback(
        &mut self,
        yaml: &str,
        fallback_interface: Box<dyn ConditionInterface>
    ) -> Result<HybridCondition, ConditionError> {
        let yaml_config = match serde_yaml::from_str::<YamlCondition>(yaml) {
            Ok(config) => Some(config),
            Err(_) => None,
        };
        
        Ok(HybridCondition {
            yaml_config,
            interface_config: Some(fallback_interface),
            evaluation_strategy: EvaluationStrategy::YamlFirst,
            fallback_strategy: FallbackStrategy::Interface,
        })
    }
    
    // Create from interface with YAML fallback
    pub fn from_interface_with_fallback(
        &mut self,
        interface: Box<dyn ConditionInterface>,
        yaml_fallback: &str
    ) -> Result<HybridCondition, ConditionError> {
        let yaml_config = match serde_yaml::from_str::<YamlCondition>(yaml_fallback) {
            Ok(config) => Some(config),
            Err(_) => None,
        };
        
        Ok(HybridCondition {
            yaml_config,
            interface_config: Some(interface),
            evaluation_strategy: EvaluationStrategy::InterfaceFirst,
            fallback_strategy: FallbackStrategy::Yaml,
        })
    }
    
    // Evaluate hybrid condition
    pub async fn evaluate_hybrid_condition(
        &self,
        condition: &HybridCondition,
        context: &ConditionContext
    ) -> Result<bool, ConditionError> {
        match condition.evaluation_strategy {
            EvaluationStrategy::YamlFirst => {
                if let Some(yaml_config) = &condition.yaml_config {
                    return self.evaluate_yaml_condition(yaml_config, context).await;
                }
                if let Some(interface_config) = &condition.interface_config {
                    return interface_config.evaluate(context);
                }
                Err(ConditionError::NoConfiguration)
            },
            EvaluationStrategy::InterfaceFirst => {
                if let Some(interface_config) = &condition.interface_config {
                    return interface_config.evaluate(context);
                }
                if let Some(yaml_config) = &condition.yaml_config {
                    return self.evaluate_yaml_condition(yaml_config, context).await;
                }
                Err(ConditionError::NoConfiguration)
            },
            EvaluationStrategy::Hybrid => {
                // Try both and combine results
                let yaml_result = if let Some(yaml_config) = &condition.yaml_config {
                    Some(self.evaluate_yaml_condition(yaml_config, context).await?)
                } else {
                    None
                };
                
                let interface_result = if let Some(interface_config) = &condition.interface_config {
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

## 🔧 **Plugin Configuration System**

### **1. Plugin System Structure**

```rust
// Plugin System (Skyrim-inspired)
pub struct PluginConfigParser {
    master_files: Vec<MasterFile>,
    plugin_files: Vec<PluginFile>,
    mod_files: Vec<ModFile>,
    user_files: Vec<UserFile>,
    load_order: Vec<String>,
    conflict_resolver: ConflictResolver,
}

// Master File
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterFile {
    pub file_name: String,
    pub file_path: String,
    pub file_size: u64,
    pub file_hash: String,
    pub conditions: Vec<ConditionDefinition>,
    pub functions: Vec<FunctionDefinition>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

// Plugin File
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginFile {
    pub file_name: String,
    pub file_path: String,
    pub file_size: u64,
    pub file_hash: String,
    pub master_dependencies: Vec<String>,
    pub conditions: Vec<ConditionDefinition>,
    pub functions: Vec<FunctionDefinition>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

// Mod File
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModFile {
    pub file_name: String,
    pub file_path: String,
    pub file_size: u64,
    pub file_hash: String,
    pub plugin_dependencies: Vec<String>,
    pub conditions: Vec<ConditionDefinition>,
    pub functions: Vec<FunctionDefinition>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

// User File
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFile {
    pub file_name: String,
    pub file_path: String,
    pub file_size: u64,
    pub file_hash: String,
    pub mod_dependencies: Vec<String>,
    pub conditions: Vec<ConditionDefinition>,
    pub functions: Vec<FunctionDefinition>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
```

### **2. Plugin Load Order Management**

```rust
impl PluginConfigParser {
    // Load plugin files in order
    pub async fn load_plugins(&mut self) -> Result<(), ConditionError> {
        // Load master files first
        for master_file in &self.master_files {
            self.load_master_file(master_file).await?;
        }
        
        // Load plugin files in dependency order
        let sorted_plugins = self.sort_plugins_by_dependencies()?;
        for plugin_file in sorted_plugins {
            self.load_plugin_file(&plugin_file).await?;
        }
        
        // Load mod files
        for mod_file in &self.mod_files {
            self.load_mod_file(mod_file).await?;
        }
        
        // Load user files last
        for user_file in &self.user_files {
            self.load_user_file(user_file).await?;
        }
        
        Ok(())
    }
    
    // Sort plugins by dependencies
    fn sort_plugins_by_dependencies(&self) -> Result<Vec<PluginFile>, ConditionError> {
        let mut sorted = Vec::new();
        let mut remaining = self.plugin_files.clone();
        
        while !remaining.is_empty() {
            let mut found_dependency = false;
            
            for (i, plugin) in remaining.iter().enumerate() {
                if self.can_load_plugin(plugin, &sorted) {
                    sorted.push(plugin.clone());
                    remaining.remove(i);
                    found_dependency = true;
                    break;
                }
            }
            
            if !found_dependency {
                return Err(ConditionError::CircularDependency);
            }
        }
        
        Ok(sorted)
    }
    
    // Check if plugin can be loaded
    fn can_load_plugin(&self, plugin: &PluginFile, loaded: &[PluginFile]) -> bool {
        for dependency in &plugin.master_dependencies {
            if !self.is_dependency_loaded(dependency, loaded) {
                return false;
            }
        }
        true
    }
    
    // Check if dependency is loaded
    fn is_dependency_loaded(&self, dependency: &str, loaded: &[PluginFile]) -> bool {
        loaded.iter().any(|p| p.file_name == dependency)
    }
}
```

## 📊 **Configuration Validation**

### **1. Configuration Validator**

```rust
// Configuration Validator
pub struct ConfigValidator {
    validation_rules: HashMap<String, ValidationRule>,
    schema_validator: SchemaValidator,
    function_validator: FunctionValidator,
    condition_validator: ConditionValidator,
}

impl ConfigValidator {
    // Validate YAML configuration
    pub fn validate_yaml_config(&self, config: &YamlCondition) -> Result<(), ValidationError> {
        // Validate required fields
        if config.condition_id.is_empty() {
            return Err(ValidationError::MissingField("condition_id"));
        }
        
        if config.condition_function.is_empty() {
            return Err(ValidationError::MissingField("condition_function"));
        }
        
        // Validate function exists
        if !self.function_validator.function_exists(&config.condition_function) {
            return Err(ValidationError::FunctionNotFound(config.condition_function.clone()));
        }
        
        // Validate parameters
        self.validate_parameters(&config.condition_parameters)?;
        
        // Validate operator
        self.validate_operator(&config.condition_operator)?;
        
        // Validate value
        self.validate_value(&config.condition_value)?;
        
        // Validate logic
        self.validate_logic(&config.condition_logic)?;
        
        Ok(())
    }
    
    // Validate interface configuration
    pub fn validate_interface_config(&self, config: &dyn ConditionInterface) -> Result<(), ValidationError> {
        // Validate required fields
        if config.get_condition_id().is_empty() {
            return Err(ValidationError::MissingField("condition_id"));
        }
        
        if config.get_condition_function().is_empty() {
            return Err(ValidationError::MissingField("condition_function"));
        }
        
        // Validate function exists
        if !self.function_validator.function_exists(&config.get_condition_function()) {
            return Err(ValidationError::FunctionNotFound(config.get_condition_function()));
        }
        
        // Validate parameters
        self.validate_parameters(&config.get_condition_parameters())?;
        
        // Validate operator
        self.validate_operator(&config.get_condition_operator())?;
        
        // Validate value
        self.validate_value(&config.get_condition_value())?;
        
        // Validate logic
        self.validate_logic(&config.get_condition_logic())?;
        
        Ok(())
    }
    
    // Validate parameters
    fn validate_parameters(&self, parameters: &[ConditionParameter]) -> Result<(), ValidationError> {
        for (i, parameter) in parameters.iter().enumerate() {
            if !parameter.is_valid() {
                return Err(ValidationError::InvalidParameter {
                    index: i,
                    parameter: parameter.clone(),
                });
            }
        }
        Ok(())
    }
    
    // Validate operator
    fn validate_operator(&self, operator: &ConditionOperator) -> Result<(), ValidationError> {
        match operator {
            ConditionOperator::Equal | ConditionOperator::NotEqual => Ok(()),
            ConditionOperator::GreaterThan | ConditionOperator::GreaterThanOrEqual => Ok(()),
            ConditionOperator::LessThan | ConditionOperator::LessThanOrEqual => Ok(()),
            ConditionOperator::Contains | ConditionOperator::NotContains => Ok(()),
            ConditionOperator::StartsWith | ConditionOperator::EndsWith => Ok(()),
            ConditionOperator::Regex => Ok(()),
            ConditionOperator::In | ConditionOperator::NotIn => Ok(()),
            ConditionOperator::Between | ConditionOperator::NotBetween => Ok(()),
            _ => Err(ValidationError::InvalidOperator(operator.clone())),
        }
    }
    
    // Validate value
    fn validate_value(&self, value: &ConditionValue) -> Result<(), ValidationError> {
        match value {
            ConditionValue::Boolean(_) => Ok(()),
            ConditionValue::Integer(_) => Ok(()),
            ConditionValue::Float(_) => Ok(()),
            ConditionValue::String(_) => Ok(()),
            ConditionValue::Vector3(_) => Ok(()),
            ConditionValue::Color(_) => Ok(()),
            ConditionValue::Time(_) => Ok(()),
            ConditionValue::Date(_) => Ok(()),
            ConditionValue::List(_) => Ok(()),
            ConditionValue::Custom(_) => Ok(()),
        }
    }
    
    // Validate logic
    fn validate_logic(&self, logic: &ConditionLogic) -> Result<(), ValidationError> {
        match logic {
            ConditionLogic::And | ConditionLogic::Or => Ok(()),
            ConditionLogic::Not | ConditionLogic::Xor => Ok(()),
            ConditionLogic::Nand | ConditionLogic::Nor => Ok(()),
        }
    }
}
```

## 🎯 **Key Features**

### **1. Multiple Configuration Methods**
- **YAML String-based**: Configuration linh hoạt
- **Class/Interface-based**: Type-safe configuration
- **Hybrid Approach**: Kết hợp cả hai methods
- **Plugin System**: Skyrim-inspired plugin system

### **2. Comprehensive Configuration Coverage**
- **Global Configuration**: System-wide settings
- **Function Registry**: Function definitions và metadata
- **Condition Definitions**: Condition definitions và templates
- **Sample Configurations**: Example configurations

### **3. Advanced Configuration Management**
- **Configuration Validation**: Comprehensive validation
- **Plugin Load Order**: Dependency management
- **Conflict Resolution**: Handle configuration conflicts
- **Hot Reload**: Dynamic configuration updates

### **4. Performance Optimization**
- **Configuration Caching**: Cache configuration data
- **Lazy Loading**: Load configurations on demand
- **Batch Processing**: Process multiple configurations
- **Memory Optimization**: Optimize memory usage

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Configuration System Design Complete  
**Maintainer**: Chaos World Team
