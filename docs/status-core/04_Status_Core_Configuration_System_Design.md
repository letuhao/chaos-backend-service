# Status Core Configuration System Design

## 📋 **Tổng Quan**

Status Core Configuration System thiết kế hệ thống configuration linh hoạt và mạnh mẽ cho Status Core, cho phép dynamic configuration, hot-reload, và easy management của status effects, categories, và interactions.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Configuration-Driven Architecture**
- **YAML-Based**: Sử dụng YAML cho human-readable configuration
- **Schema Validation**: Validate configuration với JSON Schema
- **Hot Reload**: Reload configuration mà không restart server
- **Version Control**: Support versioning cho configuration changes

### **2. Hierarchical Configuration**
- **Global Settings**: Global configuration cho toàn bộ system
- **Plugin Settings**: Plugin-specific configuration
- **Category Settings**: Category-specific configuration
- **Effect Settings**: Effect-specific configuration

### **3. Dynamic Configuration**
- **Runtime Changes**: Thay đổi configuration tại runtime
- **Conditional Configuration**: Configuration dựa trên conditions
- **Environment-Specific**: Different configuration cho different environments
- **User-Specific**: User-specific configuration overrides

## 🏗️ **Configuration Architecture**

### **1. Configuration Hierarchy**

```yaml
# Configuration Hierarchy
status_core_config/
├── global/
│   ├── system_settings.yaml
│   ├── performance_settings.yaml
│   └── security_settings.yaml
├── plugins/
│   ├── elemental_status_plugin.yaml
│   ├── combat_status_plugin.yaml
│   └── movement_status_plugin.yaml
├── categories/
│   ├── elemental_categories.yaml
│   ├── combat_categories.yaml
│   └── movement_categories.yaml
├── effects/
│   ├── elemental_effects.yaml
│   ├── combat_effects.yaml
│   └── movement_effects.yaml
├── interactions/
│   ├── elemental_interactions.yaml
│   ├── combat_interactions.yaml
│   └── movement_interactions.yaml
└── overrides/
    ├── development.yaml
    ├── staging.yaml
    └── production.yaml
```

### **2. Configuration Manager**

```rust
/// Status Core Configuration Manager
pub struct StatusCoreConfigurationManager {
    // Configuration sources
    file_loader: ConfigurationFileLoader,
    database_loader: ConfigurationDatabaseLoader,
    environment_loader: EnvironmentConfigurationLoader,
    
    // Configuration cache
    configuration_cache: ConfigurationCache,
    
    // Configuration validators
    schema_validator: ConfigurationSchemaValidator,
    business_validator: ConfigurationBusinessValidator,
    
    // Configuration watchers
    file_watcher: ConfigurationFileWatcher,
    database_watcher: ConfigurationDatabaseWatcher,
    
    // Configuration processors
    configuration_processor: ConfigurationProcessor,
    hot_reload_processor: HotReloadProcessor,
}

impl StatusCoreConfigurationManager {
    /// Load configuration from all sources
    pub async fn load_configuration(&mut self) -> Result<StatusCoreConfiguration, ConfigurationError> {
        let mut configuration = StatusCoreConfiguration::new();
        
        // Load global configuration
        let global_config = self.load_global_configuration().await?;
        configuration.merge_global(global_config);
        
        // Load plugin configurations
        let plugin_configs = self.load_plugin_configurations().await?;
        configuration.merge_plugins(plugin_configs);
        
        // Load category configurations
        let category_configs = self.load_category_configurations().await?;
        configuration.merge_categories(category_configs);
        
        // Load effect configurations
        let effect_configs = self.load_effect_configurations().await?;
        configuration.merge_effects(effect_configs);
        
        // Load interaction configurations
        let interaction_configs = self.load_interaction_configurations().await?;
        configuration.merge_interactions(interaction_configs);
        
        // Load environment overrides
        let environment_overrides = self.load_environment_overrides().await?;
        configuration.apply_overrides(environment_overrides);
        
        // Validate configuration
        self.validate_configuration(&configuration).await?;
        
        // Cache configuration
        self.cache_configuration(configuration.clone()).await?;
        
        Ok(configuration)
    }
    
    /// Hot reload configuration
    pub async fn hot_reload_configuration(&mut self) -> Result<(), ConfigurationError> {
        // Check for configuration changes
        let changes = self.detect_configuration_changes().await?;
        
        if changes.is_empty() {
            return Ok(());
        }
        
        // Load changed configurations
        let new_configuration = self.load_configuration().await?;
        
        // Validate new configuration
        self.validate_configuration(&new_configuration).await?;
        
        // Apply new configuration
        self.apply_configuration(new_configuration).await?;
        
        // Notify subscribers
        self.notify_configuration_changes(changes).await?;
        
        Ok(())
    }
    
    /// Get configuration value
    pub fn get_configuration_value<T>(&self, path: &str) -> Result<T, ConfigurationError>
    where
        T: DeserializeOwned,
    {
        self.configuration_cache.get_value(path)
    }
    
    /// Set configuration value
    pub async fn set_configuration_value<T>(&mut self, path: &str, value: T) -> Result<(), ConfigurationError>
    where
        T: Serialize,
    {
        // Update configuration
        self.configuration_cache.set_value(path, value).await?;
        
        // Persist changes
        self.persist_configuration_changes().await?;
        
        // Notify subscribers
        self.notify_configuration_changes(vec![path.to_string()]).await?;
        
        Ok(())
    }
}
```

### **3. Configuration Schema System**

```rust
/// Configuration Schema Validator
pub struct ConfigurationSchemaValidator {
    schemas: HashMap<String, JsonSchema>,
    schema_registry: SchemaRegistry,
}

impl ConfigurationSchemaValidator {
    /// Validate configuration against schema
    pub fn validate_configuration(
        &self,
        configuration: &StatusCoreConfiguration
    ) -> Result<ValidationResult, ConfigurationError> {
        let mut validation_result = ValidationResult::new();
        
        // Validate global configuration
        self.validate_global_configuration(configuration, &mut validation_result)?;
        
        // Validate plugin configurations
        self.validate_plugin_configurations(configuration, &mut validation_result)?;
        
        // Validate category configurations
        self.validate_category_configurations(configuration, &mut validation_result)?;
        
        // Validate effect configurations
        self.validate_effect_configurations(configuration, &mut validation_result)?;
        
        // Validate interaction configurations
        self.validate_interaction_configurations(configuration, &mut validation_result)?;
        
        Ok(validation_result)
    }
    
    /// Validate global configuration
    fn validate_global_configuration(
        &self,
        configuration: &StatusCoreConfiguration,
        validation_result: &mut ValidationResult
    ) -> Result<(), ConfigurationError> {
        let schema = self.schemas.get("global_configuration")
            .ok_or(ConfigurationError::SchemaNotFound("global_configuration".to_string()))?;
        
        let global_config_json = serde_json::to_value(&configuration.global)?;
        let validation_result = schema.validate(&global_config_json);
        
        if !validation_result.is_valid() {
            validation_result.add_error("global_configuration", validation_result.errors);
        }
        
        Ok(())
    }
}

/// Configuration Schema Registry
pub struct SchemaRegistry {
    schemas: HashMap<String, JsonSchema>,
    schema_loader: SchemaLoader,
}

impl SchemaRegistry {
    /// Register schema
    pub fn register_schema(&mut self, name: String, schema: JsonSchema) {
        self.schemas.insert(name, schema);
    }
    
    /// Load schema from file
    pub async fn load_schema_from_file(&mut self, name: String, file_path: &str) -> Result<(), ConfigurationError> {
        let schema_content = tokio::fs::read_to_string(file_path).await?;
        let schema: JsonSchema = serde_json::from_str(&schema_content)?;
        self.register_schema(name, schema);
        Ok(())
    }
    
    /// Get schema
    pub fn get_schema(&self, name: &str) -> Option<&JsonSchema> {
        self.schemas.get(name)
    }
}
```

## 🔧 **Configuration Files**

### **1. Global Configuration**

```yaml
# global/system_settings.yaml
version: 1.0

system:
  name: "Status Core"
  version: "1.0.0"
  environment: "development"
  debug_mode: true
  log_level: "info"
  
performance:
  max_status_effects_per_actor: 50
  status_effect_cleanup_interval: 60.0  # seconds
  cache_ttl: 300.0  # seconds
  batch_processing_size: 100
  max_concurrent_operations: 1000
  
security:
  enable_plugin_validation: true
  enable_sandboxing: true
  max_plugin_memory: 100  # MB
  max_plugin_cpu_time: 10.0  # seconds
  allowed_plugin_operations: ["read", "write", "execute"]
  
plugins:
  enable_hot_reload: true
  plugin_directory: "plugins/"
  plugin_config_directory: "configs/plugins/"
  plugin_cache_directory: "cache/plugins/"
  
categories:
  enable_dynamic_categories: true
  max_categories_per_plugin: 100
  category_hierarchy_depth: 10
  
effects:
  enable_dynamic_effects: true
  max_effects_per_plugin: 1000
  effect_property_limit: 100
  
interactions:
  enable_dynamic_interactions: true
  max_interactions_per_effect: 50
  interaction_priority_range: 1000

# New configuration sections based on discussions
status_categories:
  enable_dynamic_categories: true
  max_categories_per_plugin: 100
  category_hierarchy_depth: 10
  
status_effects:
  enable_dynamic_effects: true
  max_effects_per_plugin: 1000
  effect_property_limit: 100
  
status_conditions:
  enable_dynamic_conditions: true
  max_conditions_per_effect: 50
  condition_priority_range: 1000
  
status_triggers:
  enable_dynamic_triggers: true
  max_triggers_per_effect: 20
  trigger_priority_range: 1000
  
status_modifiers:
  enable_dynamic_modifiers: true
  max_modifiers_per_effect: 100
  modifier_priority_range: 1000
  
status_interactions:
  enable_dynamic_interactions: true
  max_interactions_per_effect: 50
  interaction_priority_range: 1000
  
status_immunity:
  enable_dynamic_immunity: true
  max_immunity_types_per_effect: 20
  immunity_priority_range: 1000
  
status_visual_effects:
  enable_dynamic_visual_effects: true
  max_visual_effects_per_effect: 50
  visual_effect_priority_range: 1000
  
status_audio_effects:
  enable_dynamic_audio_effects: true
  max_audio_effects_per_effect: 50
  audio_effect_priority_range: 1000
  
status_movement_restrictions:
  enable_dynamic_movement_restrictions: true
  max_movement_restrictions_per_effect: 20
  movement_restriction_priority_range: 1000
```

### **2. Plugin Configuration**

```yaml
# plugins/elemental_status_plugin.yaml
version: 1.0

plugin:
  plugin_id: "elemental_status_plugin"
  plugin_name: "Elemental Status Plugin"
  plugin_name_vi: "Plugin Trạng Thái Nguyên Tố"
  version: "1.0.0"
  author: "Chaos World Team"
  description: "Plugin for elemental status effects"
  description_vi: "Plugin cho hiệu ứng trạng thái nguyên tố"
  enabled: true
  priority: 100
  dependencies: []
  load_order: 1
  
configuration:
  element_mastery_scaling: 0.01
  status_duration_base: 10.0
  status_magnitude_base: 1.0
  enable_elemental_interactions: true
  enable_status_combinations: true
  enable_elemental_mastery_bonuses: true
  enable_elemental_resistance: true
  
performance:
  cache_element_mastery: true
  cache_status_effects: true
  cache_interactions: true
  batch_process_effects: true
  max_concurrent_effects: 100
  
security:
  allowed_element_types: ["fire", "water", "earth", "wood", "metal", "air", "lightning", "ice"]
  max_magnitude_multiplier: 10.0
  max_duration_multiplier: 10.0
  require_element_mastery: true
  min_element_mastery: 0.0
  max_element_mastery: 10000.0
  
categories:
  - category_id: "elemental"
    name: "Elemental"
    name_vi: "Nguyên Tố"
    description: "Elemental status effects"
    description_vi: "Hiệu ứng trạng thái nguyên tố"
    parent_category: null
    child_categories: ["fire", "water", "earth", "wood", "metal"]
    tags: ["elemental", "magic", "nature"]
    properties:
      element_mastery_required: true
      element_interaction_bonus: 0.2
      status_duration_multiplier: 1.0
    interactions:
      - target_category: "combat"
        interaction_type: "Amplify"
        multiplier: 1.5
        conditions: []
        priority: 100
    priority: 100
    is_active: true
    
effects:
  - effect_id: "burning"
    effect_name: "Burning"
    effect_name_vi: "Cháy"
    category: "Elemental(Fire)"
    effect_type: "Elemental(Burning)"
    magnitude:
      base_value: 0.05
      scaling_factor: 0.01
      scaling_stat: "fire_mastery"
      min_value: 0.01
      max_value: 0.2
      calculation_formula: "base_value + (scaling_stat * scaling_factor)"
    duration:
      base_duration: "10.0s"
      scaling_factor: 0.1
      scaling_stat: "fire_mastery"
      min_duration: "5.0s"
      max_duration: "30.0s"
      calculation_formula: "base_duration + (scaling_stat * scaling_factor)"
    target: "Target"
    source: "Element(fire)"
    conditions:
      - condition_type: "ElementMastery"
        condition_value: 100
        condition_operator: "GreaterThanOrEqual"
        condition_target: "fire"
        condition_duration: null
        is_required: true
    interactions:
      - interaction_id: "burning_damage"
        interaction_name: "Burning Damage"
        interaction_name_vi: "Sát Thương Cháy"
        target_effect: "health"
        interaction_type: "Amplify"
        multiplier: 0.05
        conditions: []
        priority: 100
        is_active: true
    immunity_list: ["fire_immunity", "burning_immunity"]
    movement_restrictions: []
    visual_effects:
      - effect_name: "fire_particles"
        intensity: "medium"
        duration: "10.0s"
    audio_effects:
      - effect_name: "burning_sound"
        volume: 0.7
        duration: "10.0s"
    properties:
      burning_damage_per_second: 0.05
      fire_resistance_reduction: 0.1
      stackable: true
      max_stacks: 5
    priority: 100
    is_active: true
    created_at: 1640995200
    updated_at: 1640995200
```

### **3. Status Configuration Structure**

Based on our discussions, the Status Core configuration system now supports the following structure:

```yaml
# Configuration Structure
status_core_config/
├── global/
│   ├── system_settings.yaml
│   ├── performance_settings.yaml
│   └── security_settings.yaml
├── plugins/
│   ├── elemental_status_plugin.yaml
│   ├── combat_status_plugin.yaml
│   └── movement_status_plugin.yaml
├── categories/
│   ├── elemental_categories.yaml
│   ├── combat_categories.yaml
│   └── movement_categories.yaml
├── effects/
│   ├── burning.yaml
│   ├── stun.yaml
│   ├── life_spirits_gift.yaml
│   ├── curse.yaml
│   └── bone_broken.yaml
├── conditions/
│   ├── elemental_conditions.yaml
│   ├── combat_conditions.yaml
│   └── movement_conditions.yaml
├── triggers/
│   ├── elemental_triggers.yaml
│   ├── combat_triggers.yaml
│   └── movement_triggers.yaml
├── modifiers/
│   ├── elemental_modifiers.yaml
│   ├── combat_modifiers.yaml
│   └── movement_modifiers.yaml
├── interactions/
│   ├── elemental_interactions.yaml
│   ├── combat_interactions.yaml
│   └── movement_interactions.yaml
├── immunity/
│   ├── elemental_immunity.yaml
│   ├── combat_immunity.yaml
│   └── movement_immunity.yaml
├── visual_effects/
│   ├── elemental_visual_effects.yaml
│   ├── combat_visual_effects.yaml
│   └── movement_visual_effects.yaml
├── audio_effects/
│   ├── elemental_audio_effects.yaml
│   ├── combat_audio_effects.yaml
│   └── movement_audio_effects.yaml
├── movement_restrictions/
│   ├── movement_slowdown.yaml
│   ├── movement_speedup.yaml
│   ├── movement_block.yaml
│   ├── partial_block.yaml
│   └── complete_block.yaml
└── overrides/
    ├── development.yaml
    ├── staging.yaml
    └── production.yaml
```

### **4. Status Effect Configuration Examples**

Based on our discussions, here are examples of the five status types we designed:

#### **4.1. Burning Status (Dot Damage)**

```yaml
# effects/burning.yaml
status_effect_definition:
  # Basic Info
  effect_id: "burning"
  effect_name: "Burning"
  effect_name_vi: "Cháy"
  world_id: "chaos_world"
  guid: "550e8400-e29b-41d4-a716-446655440000"
  
  # Effect Properties
  categories: ["Elemental(Fire)", "Damage Over Time"]
  effect_type: "Elemental(Burning)"
  priority: 100
  stackable: true
  max_stacks: 5
  stack_behavior: "stack_additive"
  
  # Effect Description
  description: "Burns the target, dealing fire damage over time"
  description_vi: "Đốt cháy mục tiêu, gây sát thương hỏa theo thời gian"
  
  # Effect Properties
  properties:
    damage_per_second: 0.05
    fire_resistance_reduction: 0.1
    element_id: "fire"
    damage_type: "fire"
    duration_type: "temporary"
    target_type: "enemy"
    
  # Effect Rules
  rules:
    - rule_id: "burning_damage_calculation"
      rule_name: "Burning Damage Calculation"
      rule_name_vi: "Tính Toán Sát Thương Cháy"
      rule_type: "damage_calculation"
      rule_description: "Calculate burning damage based on fire mastery and target resistance"
      rule_formula: "damage = base_damage * fire_mastery_multiplier * (1 - target_fire_resistance)"
      
    - rule_id: "burning_duration_calculation"
      rule_name: "Burning Duration Calculation"
      rule_name_vi: "Tính Toán Thời Gian Cháy"
      rule_type: "duration_calculation"
      rule_description: "Calculate burning duration based on fire mastery and target resistance"
      rule_formula: "duration = base_duration * fire_mastery_multiplier * (1 - target_fire_resistance)"
      
  # Effect Modifiers
  modifiers:
    - modifier_id: "fire_mastery_multiplier"
      modifier_name: "Fire Mastery Multiplier"
      modifier_name_vi: "Hệ Số Nhân Thành Thạo Hỏa"
      modifier_type: "multiplier"
      base_value: 1.0
      scaling_factor: 0.01
      scaling_stat: "fire_mastery"
      
    - modifier_id: "fire_resistance_reduction"
      modifier_name: "Fire Resistance Reduction"
      modifier_name_vi: "Giảm Kháng Hỏa"
      modifier_type: "reduction"
      base_value: 0.1
      scaling_factor: 0.001
      scaling_stat: "fire_mastery"
      
  # Effect Interactions
  interactions:
    - interaction_id: "burning_with_water"
      interaction_name: "Burning with Water"
      interaction_name_vi: "Cháy với Thủy"
      interaction_type: "suppress"
      target_effect: "water_status"
      interaction_behavior: "reduce_effectiveness"
      interaction_modifier: 0.5
      
    - interaction_id: "burning_with_wood"
      interaction_name: "Burning with Wood"
      interaction_name_vi: "Cháy với Mộc"
      interaction_type: "amplify"
      target_effect: "wood_status"
      interaction_behavior: "increase_effectiveness"
      interaction_modifier: 2.0
      
  # Effect Immunity
  immunity:
    - immunity_id: "fire_immunity"
      immunity_name: "Fire Immunity"
      immunity_name_vi: "Miễn Nhiễm Hỏa"
      immunity_type: "complete"
      immunity_condition: "element_mastery_above_1000"
      immunity_duration: "permanent"
      
  # Effect Visual Effects
  visual_effects:
    - effect_id: "fire_particles"
      effect_name: "Fire Particles"
      effect_name_vi: "Hạt Lửa"
      effect_type: "particle_effect"
      particle_type: "fire"
      particle_color: "#ff4500"
      particle_intensity: 0.8
      
  # Effect Audio Effects
  audio_effects:
    - effect_id: "burning_sound"
      effect_name: "Burning Sound"
      effect_name_vi: "Âm Thanh Cháy"
      effect_type: "loop_sound"
      sound_type: "fire"
      volume: 0.7
      loop: true
```

#### **4.2. Stun Status (Hard Control)**

```yaml
# effects/stun.yaml
status_effect_definition:
  # Basic Info
  effect_id: "stun"
  effect_name: "Stun"
  effect_name_vi: "Choáng"
  world_id: "chaos_world"
  guid: "550e8400-e29b-41d4-a716-446655440001"
  
  # Effect Properties
  categories: ["Hard Control", "Combat"]
  effect_type: "HardControl(Stun)"
  priority: 200
  stackable: false
  max_stacks: 1
  stack_behavior: "stack_replace"
  
  # Effect Description
  description: "Stuns the target, preventing all actions"
  description_vi: "Làm choáng mục tiêu, ngăn chặn tất cả hành động"
  
  # Effect Properties
  properties:
    action_restriction: "all"
    movement_restriction: "all"
    element_id: "lightning"
    damage_type: "lightning"
    duration_type: "temporary"
    target_type: "enemy"
    
  # Effect Rules
  rules:
    - rule_id: "stun_duration_calculation"
      rule_name: "Stun Duration Calculation"
      rule_name_vi: "Tính Toán Thời Gian Choáng"
      rule_type: "duration_calculation"
      rule_description: "Calculate stun duration based on lightning mastery and target resistance"
      rule_formula: "duration = base_duration * lightning_mastery_multiplier * (1 - target_lightning_resistance)"
      
    - rule_id: "stun_break_processing"
      rule_name: "Stun Break Processing"
      rule_name_vi: "Xử Lý Phá Vỡ Choáng"
      rule_type: "break_processing"
      rule_description: "Process stun break conditions"
      rule_formula: "stun_broken = (constitution + willpower) / 100.0 > break_threshold"
      
  # Effect Modifiers
  modifiers:
    - modifier_id: "lightning_mastery_multiplier"
      modifier_name: "Lightning Mastery Multiplier"
      modifier_name_vi: "Hệ Số Nhân Thành Thạo Lôi"
      modifier_type: "multiplier"
      base_value: 1.0
      scaling_factor: 0.01
      scaling_stat: "lightning_mastery"
      
    - modifier_id: "stun_resistance_reduction"
      modifier_name: "Stun Resistance Reduction"
      modifier_name_vi: "Giảm Kháng Choáng"
      modifier_type: "reduction"
      base_value: 0.1
      scaling_factor: 0.001
      scaling_stat: "lightning_mastery"
      
  # Effect Interactions
  interactions:
    - interaction_id: "stun_with_earth"
      interaction_name: "Stun with Earth"
      interaction_name_vi: "Choáng với Thổ"
      interaction_type: "amplify"
      target_effect: "earth_status"
      interaction_behavior: "increase_effectiveness"
      interaction_modifier: 1.5
      
    - interaction_id: "stun_with_water"
      interaction_name: "Stun with Water"
      interaction_name_vi: "Choáng với Thủy"
      interaction_type: "suppress"
      target_effect: "water_status"
      interaction_behavior: "reduce_effectiveness"
      interaction_modifier: 0.5
      
  # Effect Immunity
  immunity:
    - immunity_id: "stun_immunity"
      immunity_name: "Stun Immunity"
      immunity_name_vi: "Miễn Nhiễm Choáng"
      immunity_type: "complete"
      immunity_condition: "element_mastery_above_2000"
      immunity_duration: "permanent"
      
  # Effect Visual Effects
  visual_effects:
    - effect_id: "lightning_aura"
      effect_name: "Lightning Aura"
      effect_name_vi: "Hào Quang Lôi"
      effect_type: "aura_effect"
      aura_type: "lightning"
      aura_color: "#ffff00"
      aura_intensity: 1.0
      
  # Effect Audio Effects
  audio_effects:
    - effect_id: "stun_sound"
      effect_name: "Stun Sound"
      effect_name_vi: "Âm Thanh Choáng"
      effect_type: "instant_sound"
      sound_type: "lightning"
      volume: 1.0
      loop: false
```

#### **4.3. Life Spirit's Gift Status (Healing + Buff)**

```yaml
# effects/life_spirits_gift.yaml
status_effect_definition:
  # Basic Info
  effect_id: "life_spirits_gift"
  effect_name: "Life Spirit's Gift"
  effect_name_vi: "Quà Tặng Linh Hồn Sự Sống"
  world_id: "chaos_world"
  guid: "550e8400-e29b-41d4-a716-446655440002"
  
  # Effect Properties
  categories: ["Healing", "Buff", "Spiritual"]
  effect_type: "Healing(LifeSpiritGift)"
  priority: 150
  stackable: true
  max_stacks: 3
  stack_behavior: "stack_additive"
  
  # Effect Description
  description: "Heals the target and increases physical speed action execution"
  description_vi: "Hồi máu mục tiêu và tăng tốc độ thực hiện hành động thể chất"
  
  # Effect Properties
  properties:
    healing_per_second: 0.03
    physical_speed_bonus: 0.2
    element_id: "wood"
    damage_type: "healing"
    duration_type: "temporary"
    target_type: "ally"
    
  # Effect Rules
  rules:
    - rule_id: "healing_calculation"
      rule_name: "Healing Calculation"
      rule_name_vi: "Tính Toán Hồi Máu"
      rule_type: "healing_calculation"
      rule_description: "Calculate healing amount based on wood mastery and target health"
      rule_formula: "healing = base_healing * wood_mastery_multiplier * (1 + target_healing_bonus)"
      
    - rule_id: "physical_speed_calculation"
      rule_name: "Physical Speed Calculation"
      rule_name_vi: "Tính Toán Tốc Độ Thể Chất"
      rule_type: "speed_calculation"
      rule_description: "Calculate physical speed bonus based on wood mastery"
      rule_formula: "speed_bonus = base_speed_bonus * wood_mastery_multiplier"
      
  # Effect Modifiers
  modifiers:
    - modifier_id: "wood_mastery_multiplier"
      modifier_name: "Wood Mastery Multiplier"
      modifier_name_vi: "Hệ Số Nhân Thành Thạo Mộc"
      modifier_type: "multiplier"
      base_value: 1.0
      scaling_factor: 0.01
      scaling_stat: "wood_mastery"
      
    - modifier_id: "healing_bonus"
      modifier_name: "Healing Bonus"
      modifier_name_vi: "Thưởng Hồi Máu"
      modifier_type: "bonus"
      base_value: 0.0
      scaling_factor: 0.001
      scaling_stat: "wood_mastery"
      
  # Effect Interactions
  interactions:
    - interaction_id: "life_gift_with_water"
      interaction_name: "Life Gift with Water"
      interaction_name_vi: "Quà Sống với Thủy"
      interaction_type: "amplify"
      target_effect: "water_status"
      interaction_behavior: "increase_effectiveness"
      interaction_modifier: 1.5
      
    - interaction_id: "life_gift_with_fire"
      interaction_name: "Life Gift with Fire"
      interaction_name_vi: "Quà Sống với Hỏa"
      interaction_type: "suppress"
      target_effect: "fire_status"
      interaction_behavior: "reduce_effectiveness"
      interaction_modifier: 0.5
      
  # Effect Immunity
  immunity:
    - immunity_id: "life_gift_immunity"
      immunity_name: "Life Gift Immunity"
      immunity_name_vi: "Miễn Nhiễm Quà Sống"
      immunity_type: "complete"
      immunity_condition: "element_mastery_above_5000"
      immunity_duration: "permanent"
      
  # Effect Visual Effects
  visual_effects:
    - effect_id: "healing_aura"
      effect_name: "Healing Aura"
      effect_name_vi: "Hào Quang Hồi Máu"
      effect_type: "aura_effect"
      aura_type: "healing"
      aura_color: "#00ff00"
      aura_intensity: 0.8
      
  # Effect Audio Effects
  audio_effects:
    - effect_id: "healing_sound"
      effect_name: "Healing Sound"
      effect_name_vi: "Âm Thanh Hồi Máu"
      effect_type: "loop_sound"
      sound_type: "healing"
      volume: 0.6
      loop: true
```

#### **4.4. Curse Status (Debuff)**

```yaml
# effects/curse.yaml
status_effect_definition:
  # Basic Info
  effect_id: "curse"
  effect_name: "Curse"
  effect_name_vi: "Lời Nguyền"
  world_id: "chaos_world"
  guid: "550e8400-e29b-41d4-a716-446655440003"
  
  # Effect Properties
  categories: ["Debuff", "Dark Magic"]
  effect_type: "Debuff(Curse)"
  priority: 120
  stackable: true
  max_stacks: 3
  stack_behavior: "stack_additive"
  
  # Effect Description
  description: "Increases life category resource consumption: hp/stamina/lifespan"
  description_vi: "Tăng tiêu thụ tài nguyên danh mục sự sống: hp/stamina/lifespan"
  
  # Effect Properties
  properties:
    hp_consumption_increase: 0.2
    stamina_consumption_increase: 0.2
    lifespan_consumption_increase: 0.1
    element_id: "dark"
    damage_type: "dark"
    duration_type: "temporary"
    target_type: "enemy"
    
  # Effect Rules
  rules:
    - rule_id: "curse_consumption_calculation"
      rule_name: "Curse Consumption Calculation"
      rule_name_vi: "Tính Toán Tiêu Thụ Lời Nguyền"
      rule_type: "consumption_calculation"
      rule_description: "Calculate resource consumption increase based on dark mastery"
      rule_formula: "consumption_increase = base_increase * dark_mastery_multiplier"
      
    - rule_id: "curse_duration_calculation"
      rule_name: "Curse Duration Calculation"
      rule_name_vi: "Tính Toán Thời Gian Lời Nguyền"
      rule_type: "duration_calculation"
      rule_description: "Calculate curse duration based on dark mastery and target resistance"
      rule_formula: "duration = base_duration * dark_mastery_multiplier * (1 - target_dark_resistance)"
      
  # Effect Modifiers
  modifiers:
    - modifier_id: "dark_mastery_multiplier"
      modifier_name: "Dark Mastery Multiplier"
      modifier_name_vi: "Hệ Số Nhân Thành Thạo Tối"
      modifier_type: "multiplier"
      base_value: 1.0
      scaling_factor: 0.01
      scaling_stat: "dark_mastery"
      
    - modifier_id: "curse_resistance_reduction"
      modifier_name: "Curse Resistance Reduction"
      modifier_name_vi: "Giảm Kháng Lời Nguyền"
      modifier_type: "reduction"
      base_value: 0.1
      scaling_factor: 0.001
      scaling_stat: "dark_mastery"
      
  # Effect Interactions
  interactions:
    - interaction_id: "curse_with_light"
      interaction_name: "Curse with Light"
      interaction_name_vi: "Lời Nguyền với Ánh Sáng"
      interaction_type: "suppress"
      target_effect: "light_status"
      interaction_behavior: "reduce_effectiveness"
      interaction_modifier: 0.5
      
    - interaction_id: "curse_with_dark"
      interaction_name: "Curse with Dark"
      interaction_name_vi: "Lời Nguyền với Tối"
      interaction_type: "amplify"
      target_effect: "dark_status"
      interaction_behavior: "increase_effectiveness"
      interaction_modifier: 1.5
      
  # Effect Immunity
  immunity:
    - immunity_id: "curse_immunity"
      immunity_name: "Curse Immunity"
      immunity_name_vi: "Miễn Nhiễm Lời Nguyền"
      immunity_type: "complete"
      immunity_condition: "element_mastery_above_3000"
      immunity_duration: "permanent"
      
  # Effect Visual Effects
  visual_effects:
    - effect_id: "dark_aura"
      effect_name: "Dark Aura"
      effect_name_vi: "Hào Quang Tối"
      effect_type: "aura_effect"
      aura_type: "dark"
      aura_color: "#800080"
      aura_intensity: 0.9
      
  # Effect Audio Effects
  audio_effects:
    - effect_id: "curse_sound"
      effect_name: "Curse Sound"
      effect_name_vi: "Âm Thanh Lời Nguyền"
      effect_type: "loop_sound"
      sound_type: "dark"
      volume: 0.8
      loop: true
```

#### **4.5. Bone Broken Status (Negate)**

```yaml
# effects/bone_broken.yaml
status_effect_definition:
  # Basic Info
  effect_id: "bone_broken"
  effect_name: "Bone Broken"
  effect_name_vi: "Gãy Xương"
  world_id: "chaos_world"
  guid: "550e8400-e29b-41d4-a716-446655440004"
  
  # Effect Properties
  categories: ["Negate", "Physical", "Injury"]
  effect_type: "Negate(BoneBroken)"
  priority: 180
  stackable: false
  max_stacks: 1
  stack_behavior: "stack_replace"
  
  # Effect Description
  description: "Slows down physical category action stats and causes physical damage on movement/attack/defense actions"
  description_vi: "Làm chậm thống kê hành động danh mục thể chất và gây sát thương thể chất khi di chuyển/tấn công/phòng thủ"
  
  # Effect Properties
  properties:
    action_execution_duration_increase: 0.5
    action_cooldown_duration_increase: 0.5
    physical_damage_on_action: 0.1
    element_id: "earth"
    damage_type: "physical"
    duration_type: "temporary"
    target_type: "enemy"
    
  # Effect Rules
  rules:
    - rule_id: "bone_broken_slowdown_calculation"
      rule_name: "Bone Broken Slowdown Calculation"
      rule_name_vi: "Tính Toán Chậm Gãy Xương"
      rule_type: "slowdown_calculation"
      rule_description: "Calculate action slowdown based on earth mastery and target resistance"
      rule_formula: "slowdown = base_slowdown * earth_mastery_multiplier * (1 - target_earth_resistance)"
      
    - rule_id: "bone_broken_damage_calculation"
      rule_name: "Bone Broken Damage Calculation"
      rule_name_vi: "Tính Toán Sát Thương Gãy Xương"
      rule_type: "damage_calculation"
      rule_description: "Calculate physical damage on action based on earth mastery"
      rule_formula: "damage = base_damage * earth_mastery_multiplier"
      
  # Effect Modifiers
  modifiers:
    - modifier_id: "earth_mastery_multiplier"
      modifier_name: "Earth Mastery Multiplier"
      modifier_name_vi: "Hệ Số Nhân Thành Thạo Thổ"
      modifier_type: "multiplier"
      base_value: 1.0
      scaling_factor: 0.01
      scaling_stat: "earth_mastery"
      
    - modifier_id: "bone_broken_resistance_reduction"
      modifier_name: "Bone Broken Resistance Reduction"
      modifier_name_vi: "Giảm Kháng Gãy Xương"
      modifier_type: "reduction"
      base_value: 0.1
      scaling_factor: 0.001
      scaling_stat: "earth_mastery"
      
  # Effect Interactions
  interactions:
    - interaction_id: "bone_broken_with_healing"
      interaction_name: "Bone Broken with Healing"
      interaction_name_vi: "Gãy Xương với Hồi Máu"
      interaction_type: "suppress"
      target_effect: "healing_status"
      interaction_behavior: "reduce_effectiveness"
      interaction_modifier: 0.3
      
    - interaction_id: "bone_broken_with_earth"
      interaction_name: "Bone Broken with Earth"
      interaction_name_vi: "Gãy Xương với Thổ"
      interaction_type: "amplify"
      target_effect: "earth_status"
      interaction_behavior: "increase_effectiveness"
      interaction_modifier: 1.5
      
  # Effect Immunity
  immunity:
    - immunity_id: "bone_broken_immunity"
      immunity_name: "Bone Broken Immunity"
      immunity_name_vi: "Miễn Nhiễm Gãy Xương"
      immunity_type: "complete"
      immunity_condition: "element_mastery_above_4000"
      immunity_duration: "permanent"
      
  # Effect Visual Effects
  visual_effects:
    - effect_id: "bone_crack_effect"
      effect_name: "Bone Crack Effect"
      effect_name_vi: "Hiệu Ứng Nứt Xương"
      effect_type: "particle_effect"
      particle_type: "bone"
      particle_color: "#8b4513"
      particle_intensity: 0.7
      
  # Effect Audio Effects
  audio_effects:
    - effect_id: "bone_crack_sound"
      effect_name: "Bone Crack Sound"
      effect_name_vi: "Âm Thanh Nứt Xương"
      effect_type: "instant_sound"
      sound_type: "bone"
      volume: 0.9
      loop: false
```

### **5. Configuration Integration with Element Core**

Based on our discussions, the Status Core configuration system integrates with Element Core's resistance system:

```yaml
# Integration with Element Core Resistance System
element_core_integration:
  resistance_system:
    use_element_core_resistance: true
    resistance_calculation_method: "element_mastery_based"
    resistance_scaling_factor: 0.01
    resistance_cap: 0.95
    
  element_binding:
    enable_element_binding: true
    element_mastery_required: true
    element_mastery_scaling: 0.01
    element_mastery_cap: 10000.0
    
  status_element_relationship:
    fire_statuses: ["burning", "ignite", "combustion"]
    water_statuses: ["freezing", "drowning", "wet"]
    earth_statuses: ["bone_broken", "petrify", "sink"]
    wood_statuses: ["life_spirits_gift", "growth", "entangle"]
    metal_statuses: ["rust", "corrosion", "magnetic"]
    air_statuses: ["wind_slow", "levitate", "tornado"]
    lightning_statuses: ["stun", "shock", "electrify"]
    ice_statuses: ["freeze", "frostbite", "slippery"]
    dark_statuses: ["curse", "shadow", "fear"]
    light_statuses: ["blessing", "illuminate", "purify"]
```

### **6. Configuration Validation and Schema**

```yaml
# Configuration Schema for Status Effects
status_effect_schema:
  type: "object"
  required: ["effect_id", "effect_name", "effect_name_vi", "world_id", "guid", "categories", "effect_type"]
  properties:
    effect_id:
      type: "string"
      pattern: "^[a-z_]+$"
      minLength: 1
      maxLength: 50
    effect_name:
      type: "string"
      minLength: 1
      maxLength: 100
    effect_name_vi:
      type: "string"
      minLength: 1
      maxLength: 100
    world_id:
      type: "string"
      pattern: "^[a-z_]+$"
      minLength: 1
      maxLength: 50
    guid:
      type: "string"
      pattern: "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
    categories:
      type: "array"
      items:
        type: "string"
      minItems: 1
      maxItems: 10
    effect_type:
      type: "string"
      pattern: "^[A-Za-z]+\\([A-Za-z]+\\)$"
    priority:
      type: "integer"
      minimum: 1
      maximum: 1000
    stackable:
      type: "boolean"
    max_stacks:
      type: "integer"
      minimum: 1
      maximum: 100
    stack_behavior:
      type: "string"
      enum: ["stack_additive", "stack_replace", "stack_multiply"]
    description:
      type: "string"
      minLength: 1
      maxLength: 500
    description_vi:
      type: "string"
      minLength: 1
      maxLength: 500
    properties:
      type: "object"
      additionalProperties: true
    rules:
      type: "array"
      items:
        type: "object"
        required: ["rule_id", "rule_name", "rule_name_vi", "rule_type", "rule_description", "rule_formula"]
        properties:
          rule_id:
            type: "string"
            pattern: "^[a-z_]+$"
          rule_name:
            type: "string"
          rule_name_vi:
            type: "string"
          rule_type:
            type: "string"
            enum: ["damage_calculation", "duration_calculation", "healing_calculation", "speed_calculation", "consumption_calculation", "slowdown_calculation", "break_processing"]
          rule_description:
            type: "string"
          rule_formula:
            type: "string"
    modifiers:
      type: "array"
      items:
        type: "object"
        required: ["modifier_id", "modifier_name", "modifier_name_vi", "modifier_type", "base_value", "scaling_factor", "scaling_stat"]
        properties:
          modifier_id:
            type: "string"
            pattern: "^[a-z_]+$"
          modifier_name:
            type: "string"
          modifier_name_vi:
            type: "string"
          modifier_type:
            type: "string"
            enum: ["multiplier", "bonus", "reduction", "penalty"]
          base_value:
            type: "number"
          scaling_factor:
            type: "number"
          scaling_stat:
            type: "string"
            pattern: "^[a-z_]+_mastery$"
    interactions:
      type: "array"
      items:
        type: "object"
        required: ["interaction_id", "interaction_name", "interaction_name_vi", "interaction_type", "target_effect", "interaction_behavior", "interaction_modifier"]
        properties:
          interaction_id:
            type: "string"
            pattern: "^[a-z_]+$"
          interaction_name:
            type: "string"
          interaction_name_vi:
            type: "string"
          interaction_type:
            type: "string"
            enum: ["amplify", "suppress", "conflict", "synergy"]
          target_effect:
            type: "string"
          interaction_behavior:
            type: "string"
            enum: ["increase_effectiveness", "reduce_effectiveness", "replace_effect", "combine_effect"]
          interaction_modifier:
            type: "number"
    immunity:
      type: "array"
      items:
        type: "object"
        required: ["immunity_id", "immunity_name", "immunity_name_vi", "immunity_type", "immunity_condition", "immunity_duration"]
        properties:
          immunity_id:
            type: "string"
            pattern: "^[a-z_]+$"
          immunity_name:
            type: "string"
          immunity_name_vi:
            type: "string"
          immunity_type:
            type: "string"
            enum: ["complete", "partial", "conditional"]
          immunity_condition:
            type: "string"
          immunity_duration:
            type: "string"
            enum: ["permanent", "temporary", "conditional"]
    visual_effects:
      type: "array"
      items:
        type: "object"
        required: ["effect_id", "effect_name", "effect_name_vi", "effect_type"]
        properties:
          effect_id:
            type: "string"
            pattern: "^[a-z_]+$"
          effect_name:
            type: "string"
          effect_name_vi:
            type: "string"
          effect_type:
            type: "string"
            enum: ["particle_effect", "aura_effect", "indicator_effect", "animation_effect"]
          particle_type:
            type: "string"
          aura_type:
            type: "string"
          indicator_type:
            type: "string"
          particle_color:
            type: "string"
            pattern: "^#[0-9a-fA-F]{6}$"
          aura_color:
            type: "string"
            pattern: "^#[0-9a-fA-F]{6}$"
          indicator_color:
            type: "string"
            pattern: "^#[0-9a-fA-F]{6}$"
          particle_intensity:
            type: "number"
            minimum: 0.0
            maximum: 1.0
          aura_intensity:
            type: "number"
            minimum: 0.0
            maximum: 1.0
          indicator_intensity:
            type: "number"
            minimum: 0.0
            maximum: 1.0
    audio_effects:
      type: "array"
      items:
        type: "object"
        required: ["effect_id", "effect_name", "effect_name_vi", "effect_type"]
        properties:
          effect_id:
            type: "string"
            pattern: "^[a-z_]+$"
          effect_name:
            type: "string"
          effect_name_vi:
            type: "string"
          effect_type:
            type: "string"
            enum: ["instant_sound", "loop_sound", "voice_effect", "ambient_sound"]
          sound_type:
            type: "string"
          volume:
            type: "number"
            minimum: 0.0
            maximum: 1.0
          loop:
            type: "boolean"
```

### **7. Configuration Management and Updates**

Based on our discussions, the Status Core configuration system now supports:

#### **7.1. Dynamic Configuration Loading**

```yaml
# Configuration Loading Strategy
configuration_loading:
  load_strategy: "individual_files"
  file_naming_convention: "{effect_id}.yaml"
  directory_structure: "hierarchical"
  validation_level: "strict"
  
  # File Loading Order
  loading_order:
    1: "global/system_settings.yaml"
    2: "categories/*.yaml"
    3: "effects/*.yaml"
    4: "conditions/*.yaml"
    5: "triggers/*.yaml"
    6: "modifiers/*.yaml"
    7: "interactions/*.yaml"
    8: "immunity/*.yaml"
    9: "visual_effects/*.yaml"
    10: "audio_effects/*.yaml"
    11: "movement_restrictions/*.yaml"
    12: "overrides/*.yaml"
```

#### **7.2. Configuration Validation**

```yaml
# Configuration Validation Rules
validation_rules:
  schema_validation: true
  business_validation: true
  cross_reference_validation: true
  
  # Validation Levels
  validation_levels:
    strict: "All validation rules must pass"
    moderate: "Critical validation rules must pass"
    lenient: "Basic validation rules must pass"
    
  # Validation Checks
  validation_checks:
    - check_id: "effect_id_uniqueness"
      check_name: "Effect ID Uniqueness"
      check_type: "uniqueness"
      check_target: "effect_id"
      check_scope: "global"
      is_required: true
      
    - check_id: "guid_uniqueness"
      check_name: "GUID Uniqueness"
      check_type: "uniqueness"
      check_target: "guid"
      check_scope: "global"
      is_required: true
      
    - check_id: "category_reference_validity"
      check_name: "Category Reference Validity"
      check_type: "reference"
      check_target: "categories"
      check_scope: "cross_file"
      is_required: true
      
    - check_id: "element_reference_validity"
      check_name: "Element Reference Validity"
      check_type: "reference"
      check_target: "element_id"
      check_scope: "cross_system"
      is_required: true
```

#### **7.3. Configuration Hot Reload**

```yaml
# Hot Reload Configuration
hot_reload:
  enable_hot_reload: true
  reload_strategy: "incremental"
  reload_validation: true
  reload_rollback: true
  
  # Reload Triggers
  reload_triggers:
    - trigger_type: "file_change"
      trigger_path: "configs/effects/*.yaml"
      reload_scope: "effect"
      validation_required: true
      
    - trigger_type: "file_change"
      trigger_path: "configs/categories/*.yaml"
      reload_scope: "category"
      validation_required: true
      
    - trigger_type: "file_change"
      trigger_path: "configs/global/*.yaml"
      reload_scope: "global"
      validation_required: true
      
  # Reload Processing
  reload_processing:
    batch_size: 10
    max_concurrent_reloads: 5
    reload_timeout: 30.0
    rollback_timeout: 10.0
```

#### **7.4. Configuration Caching**

```yaml
# Configuration Caching Strategy
caching_strategy:
  enable_caching: true
  cache_type: "in_memory"
  cache_ttl: 300.0
  cache_size_limit: 1000
  cache_eviction_policy: "lru"
  
  # Cache Invalidation
  cache_invalidation:
    - invalidation_trigger: "file_change"
      invalidation_scope: "affected_files"
      invalidation_strategy: "immediate"
      
    - invalidation_trigger: "manual_reload"
      invalidation_scope: "all"
      invalidation_strategy: "immediate"
      
  # Cache Warming
  cache_warming:
    enable_warmup: true
    warmup_strategy: "eager"
    warmup_priority: "high_usage_first"
```

#### **7.5. Configuration Monitoring**

```yaml
# Configuration Monitoring
monitoring:
  enable_monitoring: true
  monitoring_level: "detailed"
  
  # Metrics to Track
  metrics:
    - metric_name: "configuration_load_time"
      metric_type: "duration"
      metric_unit: "milliseconds"
      
    - metric_name: "configuration_validation_time"
      metric_type: "duration"
      metric_unit: "milliseconds"
      
    - metric_name: "configuration_cache_hit_rate"
      metric_type: "ratio"
      metric_unit: "percentage"
      
    - metric_name: "configuration_reload_frequency"
      metric_type: "counter"
      metric_unit: "count_per_minute"
      
  # Alerts
  alerts:
    - alert_name: "configuration_load_failure"
      alert_condition: "load_time > 5000ms"
      alert_severity: "error"
      alert_action: "notify_admin"
      
    - alert_name: "configuration_validation_failure"
      alert_condition: "validation_errors > 0"
      alert_severity: "warning"
      alert_action: "log_error"
```

#### **7.6. Configuration Backup and Recovery**

```yaml
# Configuration Backup and Recovery
backup_recovery:
  enable_backup: true
  backup_strategy: "versioned"
  backup_frequency: "daily"
  backup_retention: 30
  
  # Backup Storage
  backup_storage:
    storage_type: "file_system"
    storage_path: "backups/configs/"
    compression: true
    encryption: true
    
  # Recovery Strategy
  recovery_strategy:
    recovery_method: "rollback"
    recovery_validation: true
    recovery_testing: true
    
  # Version Control
  version_control:
    enable_versioning: true
    version_format: "semantic"
    version_auto_increment: true
    version_metadata: true
```

## 🚀 **Hot Reload System**

### **1. Configuration Watcher**

```rust
/// Configuration File Watcher
pub struct ConfigurationFileWatcher {
    watcher: notify::RecommendedWatcher,
    watch_paths: Vec<PathBuf>,
    change_handlers: HashMap<String, Box<dyn Fn(PathBuf) -> Result<(), ConfigurationError> + Send + Sync>>,
}

impl ConfigurationFileWatcher {
    /// Start watching configuration files
    pub async fn start_watching(&mut self) -> Result<(), ConfigurationError> {
        for path in &self.watch_paths {
            self.watcher.watch(path, notify::RecursiveMode::Recursive)?;
        }
        
        // Start watching loop
        tokio::spawn(async move {
            loop {
                match self.watcher.next().await {
                    Some(Ok(event)) => {
                        self.handle_file_change(event).await?;
                    },
                    Some(Err(e)) => {
                        eprintln!("Configuration watcher error: {}", e);
                    },
                    None => break,
                }
            }
        });
        
        Ok(())
    }
    
    /// Handle file change
    async fn handle_file_change(&self, event: notify::Event) -> Result<(), ConfigurationError> {
        for path in event.paths {
            if let Some(handler) = self.change_handlers.get(&path.to_string_lossy()) {
                handler(path)?;
            }
        }
        Ok(())
    }
}
```

### **2. Hot Reload Processor**

```rust
/// Hot Reload Processor
pub struct HotReloadProcessor {
    configuration_manager: Arc<StatusCoreConfigurationManager>,
    reload_queue: Arc<Mutex<Vec<ReloadRequest>>>,
    reload_processor: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl HotReloadProcessor {
    /// Process hot reload
    pub async fn process_hot_reload(&self) -> Result<(), ConfigurationError> {
        let mut reload_queue = self.reload_queue.lock().await;
        
        while let Some(reload_request) = reload_queue.pop() {
            match reload_request.reload_type {
                ReloadType::Plugin => {
                    self.reload_plugin(reload_request.plugin_id).await?;
                },
                ReloadType::Category => {
                    self.reload_category(reload_request.category_id).await?;
                },
                ReloadType::Effect => {
                    self.reload_effect(reload_request.effect_id).await?;
                },
                ReloadType::Interaction => {
                    self.reload_interaction(reload_request.interaction_id).await?;
                },
                ReloadType::Global => {
                    self.reload_global_configuration().await?;
                },
            }
        }
        
        Ok(())
    }
    
    /// Reload plugin
    async fn reload_plugin(&self, plugin_id: String) -> Result<(), ConfigurationError> {
        // Unload plugin
        self.configuration_manager.unload_plugin(&plugin_id).await?;
        
        // Reload plugin configuration
        let plugin_config = self.configuration_manager.load_plugin_configuration(&plugin_id).await?;
        
        // Load plugin
        self.configuration_manager.load_plugin(plugin_config).await?;
        
        Ok(())
    }
}

/// Reload Request
#[derive(Debug, Clone)]
pub struct ReloadRequest {
    pub reload_type: ReloadType,
    pub plugin_id: Option<String>,
    pub category_id: Option<String>,
    pub effect_id: Option<String>,
    pub interaction_id: Option<String>,
    pub priority: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReloadType {
    Plugin,
    Category,
    Effect,
    Interaction,
    Global,
}
```

## 🧪 **Testing Strategy**

### **1. Configuration Testing**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_configuration_loading() {
        let mut config_manager = StatusCoreConfigurationManager::new();
        let config = config_manager.load_configuration().await?;
        
        assert!(config.global.is_some());
        assert!(!config.plugins.is_empty());
        assert!(!config.categories.is_empty());
        assert!(!config.effects.is_empty());
    }
    
    #[test]
    fn test_configuration_validation() {
        let config_manager = StatusCoreConfigurationManager::new();
        let config = create_test_configuration();
        
        let validation_result = config_manager.validate_configuration(&config).await?;
        assert!(validation_result.is_valid());
    }
    
    #[test]
    fn test_hot_reload() {
        let hot_reload_processor = HotReloadProcessor::new();
        let reload_request = ReloadRequest {
            reload_type: ReloadType::Plugin,
            plugin_id: Some("test_plugin".to_string()),
            category_id: None,
            effect_id: None,
            interaction_id: None,
            priority: 100,
        };
        
        hot_reload_processor.queue_reload(reload_request).await?;
        hot_reload_processor.process_hot_reload().await?;
    }
}
```

### **2. Performance Testing**

```rust
#[tokio::test]
async fn test_configuration_performance() {
    let mut config_manager = StatusCoreConfigurationManager::new();
    
    // Test configuration loading performance
    let start_time = Instant::now();
    let config = config_manager.load_configuration().await?;
    let loading_time = start_time.elapsed();
    
    assert!(loading_time.as_millis() < 1000); // Should load in < 1 second
    
    // Test configuration access performance
    let start_time = Instant::now();
    for _ in 0..1000 {
        let _: String = config_manager.get_configuration_value("global.system.name")?;
    }
    let access_time = start_time.elapsed();
    
    assert!(access_time.as_millis() < 100); // Should access in < 100ms
}
```

## 📝 **Implementation Notes**

### **1. Configuration Management**
- **File-based**: Use YAML files cho configuration
- **Database Integration**: Support database storage cho dynamic configuration
- **Environment Variables**: Support environment variable overrides
- **Validation**: Comprehensive validation với JSON Schema

### **2. Hot Reload Strategy**
- **File Watching**: Watch configuration files cho changes
- **Incremental Reload**: Reload only changed components
- **Rollback Support**: Support rollback to previous configuration
- **Error Handling**: Graceful error handling cho reload failures

### **3. Performance Considerations**
- **Caching**: Cache configuration values cho fast access
- **Lazy Loading**: Load configuration components on demand
- **Batch Processing**: Process multiple configuration changes efficiently
- **Memory Management**: Efficient memory usage cho configuration data

---

**Last Updated**: 2025-01-27  
**Version**: 2.0  
**Status**: Updated with Configuration Examples and Integration  
**Maintainer**: Chaos World Team

## 📝 **Summary of Updates**

### **Version 2.0 Changes (2025-01-27)**

1. **Enhanced Configuration Structure**:
   - Added support for individual status effect files (e.g., `burning.yaml`, `stun.yaml`)
   - Added GUID and world_id support for cross-world compatibility
   - Added support for multiple categories per status effect
   - Added comprehensive configuration validation schema

2. **Status Effect Examples**:
   - Added complete configuration examples for all five status types discussed:
     - Burning (Dot Damage)
     - Stun (Hard Control)
     - Life Spirit's Gift (Healing + Buff)
     - Curse (Debuff)
     - Bone Broken (Negate)

3. **Element Core Integration**:
   - Added configuration for integration with Element Core's resistance system
   - Added element binding configuration
   - Added status-element relationship mapping

4. **Configuration Management**:
   - Added dynamic configuration loading strategy
   - Added comprehensive validation rules
   - Added hot reload configuration
   - Added caching strategy
   - Added monitoring and alerting
   - Added backup and recovery strategy

5. **Schema Validation**:
   - Added complete JSON Schema for status effect validation
   - Added validation rules for all configuration components
   - Added cross-reference validation support

### **Key Features**

- **Plugin-Based Architecture**: Support for dynamic loading and management of status effects
- **Configuration-Driven Design**: YAML-based configuration for maximum flexibility
- **Hot Reload Support**: Real-time configuration updates without server restart
- **Comprehensive Validation**: Multi-level validation with schema and business rules
- **Element Core Integration**: Seamless integration with existing elemental systems
- **Cross-World Compatibility**: GUID-based identification for multi-world support
- **Performance Optimization**: Caching and batch processing for optimal performance
- **Monitoring and Alerting**: Comprehensive monitoring and alerting system
- **Backup and Recovery**: Versioned backup and recovery system

### **Next Steps**

1. **Implementation**: Begin implementing the Status Core system based on this design
2. **Testing**: Create comprehensive test suites for all configuration components
3. **Documentation**: Create user guides and API documentation
4. **Integration**: Integrate with existing systems (Element Core, Combat Core, etc.)
5. **Performance Tuning**: Optimize performance based on real-world usage
6. **Monitoring**: Implement monitoring and alerting systems
7. **Backup Strategy**: Implement backup and recovery systems
