# Damage Manager Configuration System

## 📋 **Tổng Quan**

Configuration System là core component của Damage Manager, quản lý tất cả configuration files và cung cấp hot-reload capabilities. Hệ thống được thiết kế để hỗ trợ dynamic configuration updates mà không cần restart service.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Configuration-Driven**
- **YAML-Based**: Sử dụng YAML cho tất cả configuration files
- **Schema Validation**: Validate configuration schema trước khi load
- **Type Safety**: Strong typing cho configuration values
- **Hot Reload**: Dynamic configuration updates

### **2. Extensibility**
- **Plugin Support**: Hỗ trợ dynamic loading của configuration plugins
- **Custom Types**: Dễ dàng thêm custom configuration types
- **Validation Rules**: Flexible validation rules
- **Default Values**: Sensible default values

### **3. Performance**
- **Lazy Loading**: Load configuration khi cần thiết
- **Caching**: Cache configuration values
- **Validation Caching**: Cache validation results
- **Memory Efficient**: Efficient memory usage

## 🏗️ **Configuration Architecture**

### **Core Components**

```rust
/// Configuration Manager
pub struct ConfigurationManager {
    // Configuration loaders
    damage_type_loader: DamageTypeLoader,
    modifier_loader: ModifierLoader,
    source_loader: SourceLoader,
    condition_loader: ConditionLoader,
    calculation_loader: CalculationLoader,
    settings_loader: SettingsLoader,
    
    // Configuration validators
    schema_validator: SchemaValidator,
    type_validator: TypeValidator,
    value_validator: ValueValidator,
    
    // Configuration cache
    configuration_cache: ConfigurationCache,
    
    // Hot reload system
    hot_reload_watcher: HotReloadWatcher,
    configuration_updater: ConfigurationUpdater,
    
    // Configuration registry
    configuration_registry: ConfigurationRegistry,
}

impl ConfigurationManager {
    /// Load all configurations
    pub async fn load_all_configurations(&self) -> Result<(), ConfigurationError> {
        // Load damage types
        let damage_types = self.damage_type_loader.load_damage_types().await?;
        self.configuration_registry.register_damage_types(damage_types).await?;
        
        // Load modifiers
        let modifiers = self.modifier_loader.load_modifiers().await?;
        self.configuration_registry.register_modifiers(modifiers).await?;
        
        // Load sources
        let sources = self.source_loader.load_sources().await?;
        self.configuration_registry.register_sources(sources).await?;
        
        // Load conditions
        let conditions = self.condition_loader.load_conditions().await?;
        self.configuration_registry.register_conditions(conditions).await?;
        
        // Load calculations
        let calculations = self.calculation_loader.load_calculations().await?;
        self.configuration_registry.register_calculations(calculations).await?;
        
        // Load settings
        let settings = self.settings_loader.load_settings().await?;
        self.configuration_registry.register_settings(settings).await?;
        
        Ok(())
    }
    
    /// Get damage type configuration
    pub async fn get_damage_type(&self, damage_type_id: &str) -> Result<DamageTypeConfig, ConfigurationError> {
        // Check cache first
        if let Some(cached_config) = self.configuration_cache.get_damage_type(damage_type_id).await? {
            return Ok(cached_config);
        }
        
        // Load from registry
        let config = self.configuration_registry.get_damage_type(damage_type_id).await?;
        
        // Cache result
        self.configuration_cache.cache_damage_type(damage_type_id, &config).await?;
        
        Ok(config)
    }
    
    /// Hot reload configuration
    pub async fn hot_reload_configuration(&self, config_type: ConfigurationType) -> Result<(), ConfigurationError> {
        match config_type {
            ConfigurationType::DamageTypes => {
                let damage_types = self.damage_type_loader.load_damage_types().await?;
                self.configuration_registry.update_damage_types(damage_types).await?;
                self.configuration_cache.invalidate_damage_types().await?;
            },
            ConfigurationType::Modifiers => {
                let modifiers = self.modifier_loader.load_modifiers().await?;
                self.configuration_registry.update_modifiers(modifiers).await?;
                self.configuration_cache.invalidate_modifiers().await?;
            },
            ConfigurationType::Sources => {
                let sources = self.source_loader.load_sources().await?;
                self.configuration_registry.update_sources(sources).await?;
                self.configuration_cache.invalidate_sources().await?;
            },
            ConfigurationType::Conditions => {
                let conditions = self.condition_loader.load_conditions().await?;
                self.configuration_registry.update_conditions(conditions).await?;
                self.configuration_cache.invalidate_conditions().await?;
            },
            ConfigurationType::Calculations => {
                let calculations = self.calculation_loader.load_calculations().await?;
                self.configuration_registry.update_calculations(calculations).await?;
                self.configuration_cache.invalidate_calculations().await?;
            },
            ConfigurationType::Settings => {
                let settings = self.settings_loader.load_settings().await?;
                self.configuration_registry.update_settings(settings).await?;
                self.configuration_cache.invalidate_settings().await?;
            },
        }
        
        Ok(())
    }
}
```

### **Configuration Loaders**

```rust
/// Damage Type Loader
pub struct DamageTypeLoader {
    file_path: PathBuf,
    yaml_parser: YamlParser,
    schema_validator: SchemaValidator,
}

impl DamageTypeLoader {
    /// Load damage types from YAML file
    pub async fn load_damage_types(&self) -> Result<Vec<DamageTypeConfig>, ConfigurationError> {
        // Read YAML file
        let yaml_content = tokio::fs::read_to_string(&self.file_path).await?;
        
        // Parse YAML
        let yaml_data: serde_yaml::Value = serde_yaml::from_str(&yaml_content)?;
        
        // Validate schema
        self.schema_validator.validate_damage_types_schema(&yaml_data)?;
        
        // Parse damage types
        let damage_types: Vec<DamageTypeConfig> = serde_yaml::from_value(yaml_data["damage_types"].clone())?;
        
        // Validate each damage type
        for damage_type in &damage_types {
            self.validate_damage_type(damage_type)?;
        }
        
        Ok(damage_types)
    }
    
    /// Validate damage type configuration
    fn validate_damage_type(&self, damage_type: &DamageTypeConfig) -> Result<(), ConfigurationError> {
        // Validate required fields
        if damage_type.id.is_empty() {
            return Err(ConfigurationError::InvalidField("id cannot be empty"));
        }
        
        if damage_type.name.is_empty() {
            return Err(ConfigurationError::InvalidField("name cannot be empty"));
        }
        
        if damage_type.resource_type.is_empty() {
            return Err(ConfigurationError::InvalidField("resource_type cannot be empty"));
        }
        
        // Validate scaling values
        if damage_type.scaling.base_multiplier < 0.0 {
            return Err(ConfigurationError::InvalidValue("base_multiplier must be >= 0.0"));
        }
        
        if damage_type.scaling.level_scaling < 0.0 {
            return Err(ConfigurationError::InvalidValue("level_scaling must be >= 0.0"));
        }
        
        if damage_type.scaling.mastery_scaling < 0.0 {
            return Err(ConfigurationError::InvalidValue("mastery_scaling must be >= 0.0"));
        }
        
        Ok(())
    }
}

/// Modifier Loader
pub struct ModifierLoader {
    file_path: PathBuf,
    yaml_parser: YamlParser,
    schema_validator: SchemaValidator,
}

impl ModifierLoader {
    /// Load modifiers from YAML file
    pub async fn load_modifiers(&self) -> Result<Vec<ModifierConfig>, ConfigurationError> {
        // Read YAML file
        let yaml_content = tokio::fs::read_to_string(&self.file_path).await?;
        
        // Parse YAML
        let yaml_data: serde_yaml::Value = serde_yaml::from_str(&yaml_content)?;
        
        // Validate schema
        self.schema_validator.validate_modifiers_schema(&yaml_data)?;
        
        // Parse modifiers
        let modifiers: Vec<ModifierConfig> = serde_yaml::from_value(yaml_data["damage_modifiers"].clone())?;
        
        // Validate each modifier
        for modifier in &modifiers {
            self.validate_modifier(modifier)?;
        }
        
        Ok(modifiers)
    }
    
    /// Validate modifier configuration
    fn validate_modifier(&self, modifier: &ModifierConfig) -> Result<(), ConfigurationError> {
        // Validate required fields
        if modifier.id.is_empty() {
            return Err(ConfigurationError::InvalidField("id cannot be empty"));
        }
        
        if modifier.name.is_empty() {
            return Err(ConfigurationError::InvalidField("name cannot be empty"));
        }
        
        if modifier.formula.is_empty() {
            return Err(ConfigurationError::InvalidField("formula cannot be empty"));
        }
        
        // Validate formula syntax
        self.validate_formula_syntax(&modifier.formula)?;
        
        // Validate value ranges
        if modifier.validation.value_range.len() != 2 {
            return Err(ConfigurationError::InvalidValue("value_range must have exactly 2 elements"));
        }
        
        let min_value = modifier.validation.value_range[0];
        let max_value = modifier.validation.value_range[1];
        
        if min_value >= max_value {
            return Err(ConfigurationError::InvalidValue("min_value must be < max_value"));
        }
        
        Ok(())
    }
}
```

### **Configuration Validators**

```rust
/// Schema Validator
pub struct SchemaValidator {
    damage_types_schema: JsonSchema,
    modifiers_schema: JsonSchema,
    sources_schema: JsonSchema,
    conditions_schema: JsonSchema,
    calculations_schema: JsonSchema,
    settings_schema: JsonSchema,
}

impl SchemaValidator {
    /// Validate damage types schema
    pub fn validate_damage_types_schema(&self, yaml_data: &serde_yaml::Value) -> Result<(), ConfigurationError> {
        // Convert YAML to JSON for schema validation
        let json_data = serde_json::to_value(yaml_data)?;
        
        // Validate against schema
        let validation_result = self.damage_types_schema.validate(&json_data);
        
        if let Err(validation_errors) = validation_result {
            let error_messages: Vec<String> = validation_errors
                .map(|e| format!("{}: {}", e.instance_path, e.to_string()))
                .collect();
            
            return Err(ConfigurationError::SchemaValidationFailed(error_messages.join(", ")));
        }
        
        Ok(())
    }
    
    /// Validate modifiers schema
    pub fn validate_modifiers_schema(&self, yaml_data: &serde_yaml::Value) -> Result<(), ConfigurationError> {
        // Convert YAML to JSON for schema validation
        let json_data = serde_json::to_value(yaml_data)?;
        
        // Validate against schema
        let validation_result = self.modifiers_schema.validate(&json_data);
        
        if let Err(validation_errors) = validation_result {
            let error_messages: Vec<String> = validation_errors
                .map(|e| format!("{}: {}", e.instance_path, e.to_string()))
                .collect();
            
            return Err(ConfigurationError::SchemaValidationFailed(error_messages.join(", ")));
        }
        
        Ok(())
    }
}

/// Type Validator
pub struct TypeValidator {
    type_registry: TypeRegistry,
}

impl TypeValidator {
    /// Validate damage type
    pub fn validate_damage_type(&self, damage_type: &DamageTypeConfig) -> Result<(), ConfigurationError> {
        // Validate resource type
        if !self.type_registry.is_valid_resource_type(&damage_type.resource_type) {
            return Err(ConfigurationError::InvalidResourceType(damage_type.resource_type.clone()));
        }
        
        // Validate category
        if !self.type_registry.is_valid_damage_category(&damage_type.category) {
            return Err(ConfigurationError::InvalidDamageCategory(damage_type.category.clone()));
        }
        
        // Validate element ID if present
        if let Some(element_id) = &damage_type.element_id {
            if !self.type_registry.is_valid_element_id(element_id) {
                return Err(ConfigurationError::InvalidElementId(element_id.clone()));
            }
        }
        
        Ok(())
    }
    
    /// Validate modifier type
    pub fn validate_modifier_type(&self, modifier: &ModifierConfig) -> Result<(), ConfigurationError> {
        // Validate modifier type
        if !self.type_registry.is_valid_modifier_type(&modifier.id) {
            return Err(ConfigurationError::InvalidModifierType(modifier.id.clone()));
        }
        
        // Validate formula variables
        for variable in &modifier.variables {
            if !self.type_registry.is_valid_variable_type(variable) {
                return Err(ConfigurationError::InvalidVariableType(variable.clone()));
            }
        }
        
        Ok(())
    }
}
```

### **Configuration Cache**

```rust
/// Configuration Cache
pub struct ConfigurationCache {
    // Caches for different configuration types
    damage_types_cache: HashMap<String, DamageTypeConfig>,
    modifiers_cache: HashMap<String, ModifierConfig>,
    sources_cache: HashMap<String, SourceConfig>,
    conditions_cache: HashMap<String, ConditionConfig>,
    calculations_cache: HashMap<String, CalculationConfig>,
    settings_cache: HashMap<String, SettingsConfig>,
    
    // Cache configuration
    cache_config: CacheConfig,
    
    // Cache statistics
    cache_stats: CacheStatistics,
}

impl ConfigurationCache {
    /// Get damage type from cache
    pub async fn get_damage_type(&self, damage_type_id: &str) -> Result<Option<DamageTypeConfig>, ConfigurationError> {
        // Check if cache is enabled
        if !self.cache_config.enable_damage_types_cache {
            return Ok(None);
        }
        
        // Check cache
        if let Some(cached_config) = self.damage_types_cache.get(damage_type_id) {
            // Update cache statistics
            self.cache_stats.increment_hit("damage_types");
            return Ok(Some(cached_config.clone()));
        }
        
        // Update cache statistics
        self.cache_stats.increment_miss("damage_types");
        Ok(None)
    }
    
    /// Cache damage type
    pub async fn cache_damage_type(&mut self, damage_type_id: &str, config: &DamageTypeConfig) -> Result<(), ConfigurationError> {
        // Check if cache is enabled
        if !self.cache_config.enable_damage_types_cache {
            return Ok(());
        }
        
        // Check cache size limit
        if self.damage_types_cache.len() >= self.cache_config.max_damage_types_cache_size {
            self.evict_oldest_damage_type().await?;
        }
        
        // Cache the configuration
        self.damage_types_cache.insert(damage_type_id.to_string(), config.clone());
        
        // Update cache statistics
        self.cache_stats.increment_set("damage_types");
        
        Ok(())
    }
    
    /// Invalidate damage types cache
    pub async fn invalidate_damage_types(&mut self) -> Result<(), ConfigurationError> {
        self.damage_types_cache.clear();
        self.cache_stats.increment_invalidation("damage_types");
        Ok(())
    }
}
```

### **Hot Reload System**

```rust
/// Hot Reload Watcher
pub struct HotReloadWatcher {
    file_watchers: HashMap<ConfigurationType, FileWatcher>,
    configuration_manager: Arc<ConfigurationManager>,
    hot_reload_config: HotReloadConfig,
}

impl HotReloadWatcher {
    /// Start watching configuration files
    pub async fn start_watching(&self) -> Result<(), ConfigurationError> {
        // Watch damage types file
        if self.hot_reload_config.watch_damage_types {
            let damage_types_watcher = FileWatcher::new(
                self.hot_reload_config.damage_types_file_path.clone(),
                self.create_file_change_handler(ConfigurationType::DamageTypes),
            ).await?;
            self.file_watchers.insert(ConfigurationType::DamageTypes, damage_types_watcher);
        }
        
        // Watch modifiers file
        if self.hot_reload_config.watch_modifiers {
            let modifiers_watcher = FileWatcher::new(
                self.hot_reload_config.modifiers_file_path.clone(),
                self.create_file_change_handler(ConfigurationType::Modifiers),
            ).await?;
            self.file_watchers.insert(ConfigurationType::Modifiers, modifiers_watcher);
        }
        
        // Watch sources file
        if self.hot_reload_config.watch_sources {
            let sources_watcher = FileWatcher::new(
                self.hot_reload_config.sources_file_path.clone(),
                self.create_file_change_handler(ConfigurationType::Sources),
            ).await?;
            self.file_watchers.insert(ConfigurationType::Sources, sources_watcher);
        }
        
        // Watch conditions file
        if self.hot_reload_config.watch_conditions {
            let conditions_watcher = FileWatcher::new(
                self.hot_reload_config.conditions_file_path.clone(),
                self.create_file_change_handler(ConfigurationType::Conditions),
            ).await?;
            self.file_watchers.insert(ConfigurationType::Conditions, conditions_watcher);
        }
        
        // Watch calculations file
        if self.hot_reload_config.watch_calculations {
            let calculations_watcher = FileWatcher::new(
                self.hot_reload_config.calculations_file_path.clone(),
                self.create_file_change_handler(ConfigurationType::Calculations),
            ).await?;
            self.file_watchers.insert(ConfigurationType::Calculations, calculations_watcher);
        }
        
        // Watch settings file
        if self.hot_reload_config.watch_settings {
            let settings_watcher = FileWatcher::new(
                self.hot_reload_config.settings_file_path.clone(),
                self.create_file_change_handler(ConfigurationType::Settings),
            ).await?;
            self.file_watchers.insert(ConfigurationType::Settings, settings_watcher);
        }
        
        Ok(())
    }
    
    /// Create file change handler
    fn create_file_change_handler(&self, config_type: ConfigurationType) -> impl Fn(FileChangeEvent) -> Result<(), ConfigurationError> + Send + Sync {
        let configuration_manager = self.configuration_manager.clone();
        let hot_reload_config = self.hot_reload_config.clone();
        
        move |event| {
            let configuration_manager = configuration_manager.clone();
            let hot_reload_config = hot_reload_config.clone();
            
            tokio::spawn(async move {
                match event {
                    FileChangeEvent::Modified(path) => {
                        tracing::info!("Configuration file modified: {:?}", path);
                        
                        // Wait for file to stabilize
                        tokio::time::sleep(Duration::from_millis(hot_reload_config.file_stabilization_delay_ms)).await;
                        
                        // Reload configuration
                        if let Err(e) = configuration_manager.hot_reload_configuration(config_type).await {
                            tracing::error!("Failed to reload configuration: {}", e);
                        } else {
                            tracing::info!("Configuration reloaded successfully: {:?}", config_type);
                        }
                    },
                    FileChangeEvent::Deleted(path) => {
                        tracing::warn!("Configuration file deleted: {:?}", path);
                    },
                    FileChangeEvent::Created(path) => {
                        tracing::info!("Configuration file created: {:?}", path);
                    },
                }
            });
            
            Ok(())
        }
    }
}
```

## 🔧 **Configuration Types**

### **Damage Type Configuration**

```rust
/// Damage Type Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageTypeConfig {
    pub id: String,
    pub name: String,
    pub resource_type: String,
    pub category: String,
    pub description: String,
    pub element_id: Option<String>,
    pub properties: HashMap<String, serde_json::Value>,
    pub scaling: DamageScalingConfig,
    pub visual_effects: VisualEffectsConfig,
}

/// Damage Scaling Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageScalingConfig {
    pub base_multiplier: f64,
    pub level_scaling: f64,
    pub mastery_scaling: f64,
}

/// Visual Effects Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualEffectsConfig {
    pub color: String,
    pub particle_effect: String,
    pub sound_effect: String,
}
```

### **Modifier Configuration**

```rust
/// Modifier Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub formula: String,
    pub variables: Vec<String>,
    pub properties: HashMap<String, serde_json::Value>,
    pub validation: ModifierValidationConfig,
}

/// Modifier Validation Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierValidationConfig {
    pub value_range: Vec<f64>,
    pub required_properties: Vec<String>,
}
```

### **Source Configuration**

```rust
/// Source Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub properties: HashMap<String, serde_json::Value>,
    pub modifiers: Vec<String>,
    pub visual_effects: VisualEffectsConfig,
}
```

### **Condition Configuration**

```rust
/// Condition Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub formula: String,
    pub variables: Vec<String>,
    pub properties: HashMap<String, serde_json::Value>,
    pub validation: ConditionValidationConfig,
}

/// Condition Validation Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionValidationConfig {
    pub value_range: Vec<f64>,
    pub required_properties: Vec<String>,
}
```

### **Calculation Configuration**

```rust
/// Calculation Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub formula: String,
    pub variables: Vec<String>,
    pub properties: HashMap<String, serde_json::Value>,
}
```

### **Settings Configuration**

```rust
/// Settings Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsConfig {
    pub performance: PerformanceSettings,
    pub validation: ValidationSettings,
    pub calculations: CalculationSettings,
    pub integration: IntegrationSettings,
    pub events: EventSettings,
    pub logging: LoggingSettings,
    pub security: SecuritySettings,
    pub debug: DebugSettings,
    pub features: FeatureSettings,
    pub resource_limits: ResourceLimitSettings,
    pub error_handling: ErrorHandlingSettings,
    pub monitoring: MonitoringSettings,
    pub configuration: ConfigurationSettings,
    pub development: DevelopmentSettings,
    pub production: ProductionSettings,
    pub backup: BackupSettings,
    pub scaling: ScalingSettings,
    pub load_balancing: LoadBalancingSettings,
    pub database: DatabaseSettings,
    pub cache: CacheSettings,
    pub message_queue: MessageQueueSettings,
    pub api: ApiSettings,
    pub documentation: DocumentationSettings,
}
```

## 🚀 **Usage Examples**

### **Load Configuration**

```rust
// Create configuration manager
let config_manager = ConfigurationManager::new(config_path).await?;

// Load all configurations
config_manager.load_all_configurations().await?;

// Get damage type configuration
let fire_damage_config = config_manager.get_damage_type("fire_damage").await?;

// Get modifier configuration
let multiplier_config = config_manager.get_modifier("multiplier").await?;

// Get source configuration
let direct_source_config = config_manager.get_source("direct").await?;
```

### **Hot Reload Configuration**

```rust
// Enable hot reload
config_manager.enable_hot_reload().await?;

// Manually reload specific configuration
config_manager.hot_reload_configuration(ConfigurationType::DamageTypes).await?;

// Reload all configurations
config_manager.hot_reload_all_configurations().await?;
```

### **Validate Configuration**

```rust
// Validate damage type
let validation_result = config_manager.validate_damage_type(&fire_damage_config).await?;

// Validate modifier
let validation_result = config_manager.validate_modifier(&multiplier_config).await?;

// Validate all configurations
let validation_result = config_manager.validate_all_configurations().await?;
```

## 📊 **Performance Metrics**

### **Configuration Loading**
- **Load Time**: < 100ms cho tất cả configurations
- **Memory Usage**: < 10MB cho configuration cache
- **Validation Time**: < 50ms cho configuration validation

### **Hot Reload**
- **Reload Time**: < 200ms cho single configuration
- **File Detection**: < 1s cho file change detection
- **Update Time**: < 100ms cho configuration update

### **Cache Performance**
- **Hit Rate**: > 95% cho configuration cache
- **Miss Rate**: < 5% cho configuration cache
- **Eviction Time**: < 10ms cho cache eviction

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
