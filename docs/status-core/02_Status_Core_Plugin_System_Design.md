# Status Core Plugin System Design

## 📋 **Tổng Quan**

Status Core Plugin System cho phép dynamic loading và management của status effects, categories, và interactions thông qua plugin architecture. Hệ thống này đảm bảo tính linh hoạt và extensibility mà không cần sửa source code.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Plugin-Based Architecture**
- **Dynamic Loading**: Load plugins tại runtime
- **Hot Reload**: Reload plugins mà không restart server
- **Isolation**: Plugins hoạt động độc lập, không ảnh hưởng lẫn nhau
- **Configuration-Driven**: Plugin discovery và loading qua configuration

### **2. Flexible Status Management**
- **Status Categories**: Hierarchical category system
- **Status Tags**: Flexible tagging system
- **Status Interactions**: Complex interaction rules
- **Status Effects**: Extensible effect system

### **3. Performance Optimization**
- **Lazy Loading**: Load plugins chỉ khi cần
- **Caching**: Cache plugin data và calculations
- **Batch Processing**: Process multiple status effects efficiently
- **Memory Management**: Efficient memory usage

## 🏗️ **Plugin System Architecture**

### **1. Core Plugin Interface**

```rust
/// Status Plugin trait for dynamic loading
#[async_trait]
pub trait StatusPlugin: Send + Sync {
    /// Plugin metadata
    fn get_plugin_info(&self) -> StatusPluginInfo;
    
    /// Initialize plugin with configuration
    async fn initialize(&mut self, config: &StatusPluginConfig) -> Result<(), StatusError>;
    
    /// Get status categories defined by this plugin
    fn get_status_categories(&self) -> Vec<StatusCategory>;
    
    /// Get status effects defined by this plugin
    fn get_status_effects(&self) -> Vec<StatusEffectDefinition>;
    
    /// Get status interactions defined by this plugin
    fn get_status_interactions(&self) -> Vec<StatusInteractionDefinition>;
    
    /// Process status effect application
    async fn process_status_effect(
        &self,
        effect: &StatusEffect,
        actor: &Actor,
        context: &StatusContext
    ) -> Result<StatusEffectResult, StatusError>;
    
    /// Process status effect removal
    async fn process_status_removal(
        &self,
        effect: &StatusEffect,
        actor: &Actor,
        context: &StatusContext
    ) -> Result<StatusRemovalResult, StatusError>;
    
    /// Calculate status effect magnitude
    fn calculate_magnitude(
        &self,
        effect: &StatusEffect,
        actor: &Actor,
        context: &StatusContext
    ) -> Result<f64, StatusError>;
    
    /// Calculate status effect duration
    fn calculate_duration(
        &self,
        effect: &StatusEffect,
        actor: &Actor,
        context: &StatusContext
    ) -> Result<Duration, StatusError>;
    
    /// Validate status effect compatibility
    fn validate_compatibility(
        &self,
        effect: &StatusEffect,
        actor: &Actor,
        context: &StatusContext
    ) -> Result<bool, StatusError>;
    
    /// Cleanup plugin resources
    async fn cleanup(&mut self) -> Result<(), StatusError>;
}

/// Plugin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPluginInfo {
    pub plugin_id: String,
    pub plugin_name: String,
    pub plugin_name_vi: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub description_vi: String,
    pub dependencies: Vec<String>,
    pub status_categories: Vec<String>,
    pub status_effects: Vec<String>,
    pub status_interactions: Vec<String>,
    pub config_schema: Option<serde_json::Value>,
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPluginConfig {
    pub plugin_id: String,
    pub enabled: bool,
    pub priority: u32,
    pub config: serde_json::Value,
    pub dependencies: Vec<String>,
    pub load_order: u32,
}
```

### **2. Plugin Registry System**

```rust
/// Status Plugin Registry for managing plugins
pub struct StatusPluginRegistry {
    plugins: HashMap<String, Box<dyn StatusPlugin>>,
    plugin_configs: HashMap<String, StatusPluginConfig>,
    plugin_dependencies: HashMap<String, Vec<String>>,
    plugin_load_order: Vec<String>,
    status_categories: HashMap<String, StatusCategory>,
    status_effects: HashMap<String, StatusEffectDefinition>,
    status_interactions: HashMap<String, StatusInteractionDefinition>,
    plugin_cache: StatusPluginCache,
}

impl StatusPluginRegistry {
    /// Register a new plugin
    pub async fn register_plugin(
        &mut self,
        plugin: Box<dyn StatusPlugin>,
        config: StatusPluginConfig
    ) -> Result<(), StatusError> {
        let plugin_info = plugin.get_plugin_info();
        let plugin_id = plugin_info.plugin_id.clone();
        
        // Validate plugin
        self.validate_plugin(&plugin_info, &config).await?;
        
        // Check dependencies
        self.check_dependencies(&plugin_info).await?;
        
        // Initialize plugin
        plugin.initialize(&config).await?;
        
        // Register plugin
        self.plugins.insert(plugin_id.clone(), plugin);
        self.plugin_configs.insert(plugin_id.clone(), config);
        
        // Update registries
        self.update_status_registries(&plugin_id).await?;
        
        // Update load order
        self.update_load_order().await?;
        
        Ok(())
    }
    
    /// Load plugin from configuration
    pub async fn load_plugin_from_config(
        &mut self,
        config_path: &str
    ) -> Result<(), StatusError> {
        let config = self.load_plugin_config(config_path).await?;
        let plugin = self.create_plugin_instance(&config).await?;
        
        self.register_plugin(plugin, config).await?;
        
        Ok(())
    }
    
    /// Unload plugin
    pub async fn unload_plugin(
        &mut self,
        plugin_id: &str
    ) -> Result<(), StatusError> {
        if let Some(mut plugin) = self.plugins.remove(plugin_id) {
            plugin.cleanup().await?;
        }
        
        self.plugin_configs.remove(plugin_id);
        self.remove_plugin_from_registries(plugin_id).await?;
        
        Ok(())
    }
    
    /// Reload plugin
    pub async fn reload_plugin(
        &mut self,
        plugin_id: &str,
        config_path: &str
    ) -> Result<(), StatusError> {
        self.unload_plugin(plugin_id).await?;
        self.load_plugin_from_config(config_path).await?;
        
        Ok(())
    }
    
    /// Get plugin by ID
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&dyn StatusPlugin> {
        self.plugins.get(plugin_id).map(|p| p.as_ref())
    }
    
    /// Get all plugins
    pub fn get_all_plugins(&self) -> &HashMap<String, Box<dyn StatusPlugin>> {
        &self.plugins
    }
    
    /// Get plugins by category
    pub fn get_plugins_by_category(&self, category: &str) -> Vec<&dyn StatusPlugin> {
        self.plugins.values()
            .filter(|plugin| {
                plugin.get_plugin_info()
                    .status_categories
                    .contains(&category.to_string())
            })
            .map(|plugin| plugin.as_ref())
            .collect()
    }
}
```

### **3. Status Category System**

```rust
/// Status Category for organizing status effects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusCategory {
    // Basic Categories
    Buff,                    // Tăng cường
    Debuff,                  // Giảm cường
    Neutral,                 // Trung tính
    
    // Elemental Categories
    Elemental(ElementalCategory),
    
    // Combat Categories
    Combat(CombatCategory),
    
    // Movement Categories
    Movement(MovementCategory),
    
    // Resource Categories
    Resource(ResourceCategory),
    
    // Special Categories
    Transformation,          // Biến đổi
    Immunity,                // Miễn nhiễm
    Environmental,           // Môi trường
    Social,                  // Xã hội
    Economic,                // Kinh tế
    
    // Custom Categories
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementalCategory {
    Fire,                    // Hỏa
    Water,                   // Thủy
    Earth,                   // Thổ
    Wood,                    // Mộc
    Metal,                   // Kim
    Air,                     // Phong
    Lightning,               // Lôi
    Ice,                     // Băng
    Light,                   // Quang
    Dark,                    // Ám
    Life,                    // Sinh
    Death,                   // Tử
    Time,                    // Thời
    Space,                   // Không
    Chaos,                   // Hỗn loạn
    Order,                   // Trật tự
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatCategory {
    Attack,                  // Tấn công
    Defense,                 // Phòng thủ
    Critical,                // Chí mạng
    Accuracy,                // Chính xác
    Dodge,                   // Né tránh
    Block,                   // Chặn
    Parry,                   // Đỡ
    Counter,                 // Phản công
    Damage,                  // Sát thương
    Healing,                 // Hồi phục
    Regeneration,            // Tái sinh
    Absorption,              // Hấp thụ
    Reflection,              // Phản xạ
    Penetration,             // Xuyên thủng
    Resistance,              // Kháng
    Immunity,                // Miễn nhiễm
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MovementCategory {
    Speed,                   // Tốc độ
    Teleportation,           // Dịch chuyển
    Flight,                  // Bay
    Swimming,                // Bơi
    Climbing,                // Leo trèo
    Jumping,                 // Nhảy
    Slowing,                 // Chậm
    Rooting,                 // Bị rễ
    Stunning,                // Choáng
    Paralyzing,              // Tê liệt
    Immobilizing,            // Bất động
    Restricting,             // Hạn chế
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceCategory {
    Health,                  // Máu
    Mana,                    // Mana
    Stamina,                 // Thể lực
    Qi,                      // Khí
    LifeForce,               // Thọ nguyên
    Vitality,                // Sinh mệnh
    Energy,                  // Sinh lực
    Exhaustion,              // Kiệt sức
    Regeneration,            // Tái sinh
    Drain,                   // Hút
    Conversion,              // Chuyển đổi
    Amplification,           // Khuếch đại
    Reduction,               // Giảm
}

/// Status Category Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCategoryDefinition {
    pub category: StatusCategory,
    pub name: String,
    pub name_vi: String,
    pub description: String,
    pub description_vi: String,
    pub parent_category: Option<StatusCategory>,
    pub child_categories: Vec<StatusCategory>,
    pub tags: Vec<String>,
    pub properties: HashMap<String, serde_json::Value>,
    pub interactions: Vec<StatusCategoryInteraction>,
    pub priority: u32,
    pub is_active: bool,
}

/// Status Category Interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCategoryInteraction {
    pub target_category: StatusCategory,
    pub interaction_type: StatusInteractionType,
    pub multiplier: f64,
    pub conditions: Vec<StatusCondition>,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusInteractionType {
    Amplify,                 // Khuếch đại
    Suppress,                // Áp chế
    Transform,               // Biến đổi
    Merge,                   // Hợp nhất
    Split,                   // Tách ra
    Replace,                 // Thay thế
    Stack,                   // Chồng lên
    Ignore,                  // Bỏ qua
    Custom(String),          // Tùy chỉnh
}
```

### **4. Status Effect System**

```rust
/// Status Effect Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectDefinition {
    pub effect_id: String,
    pub effect_name: String,
    pub effect_name_vi: String,
    pub category: StatusCategory,
    pub effect_type: StatusEffectType,
    pub magnitude: StatusMagnitude,
    pub duration: StatusDuration,
    pub target: StatusTarget,
    pub source: StatusSource,
    pub conditions: Vec<StatusCondition>,
    pub interactions: Vec<StatusEffectInteraction>,
    pub immunity_list: Vec<String>,
    pub movement_restrictions: Vec<MovementRestriction>,
    pub visual_effects: Vec<VisualEffect>,
    pub audio_effects: Vec<AudioEffect>,
    pub properties: HashMap<String, serde_json::Value>,
    pub priority: u32,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Status Effect Type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEffectType {
    // Basic Types
    Buff,                    // Tăng cường
    Debuff,                  // Giảm cường
    Neutral,                 // Trung tính
    
    // Elemental Types
    Elemental(ElementalStatusType),
    
    // Combat Types
    Combat(CombatStatusType),
    
    // Movement Types
    Movement(MovementStatusType),
    
    // Resource Types
    Resource(ResourceStatusType),
    
    // Special Types
    Transformation,          // Biến đổi
    Immunity,                // Miễn nhiễm
    Environmental,           // Môi trường
    Social,                  // Xã hội
    Economic,                // Kinh tế
    
    // Custom Types
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementalStatusType {
    Burning,                 // Cháy
    Freezing,                // Đóng băng
    Electrified,             // Bị điện giật
    Poisoned,                // Bị độc
    Crystallized,            // Kết tinh
    Regeneration,            // Tái sinh
    Resistance,              // Kháng
    Immunity,                // Miễn nhiễm
    Amplification,           // Khuếch đại
    Suppression,             // Áp chế
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatStatusType {
    Bleeding,                // Chảy máu
    Stunned,                 // Choáng
    Paralyzed,               // Tê liệt
    Rooted,                  // Bị rễ
    Slowed,                  // Chậm
    Hasted,                  // Tăng tốc
    Strengthened,            // Tăng sức mạnh
    Weakened,                // Yếu đi
    Shielded,                // Có khiên
    Armored,                 // Có giáp
    Critical,                // Chí mạng
    Accurate,                // Chính xác
    Dodging,                 // Né tránh
    Blocking,                // Chặn
    Parrying,                // Đỡ
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MovementStatusType {
    Immobilized,             // Bất động
    Rooted,                  // Bị rễ
    Slowed,                  // Chậm
    Hasted,                  // Tăng tốc
    Teleporting,             // Dịch chuyển
    Flying,                  // Bay
    Swimming,                // Bơi
    Climbing,                // Leo trèo
    Jumping,                 // Nhảy
    Falling,                 // Rơi
    Floating,                // Lơ lửng
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceStatusType {
    Exhausted,               // Kiệt sức
    Regenerating,            // Tái sinh
    Draining,                // Hút
    Converting,              // Chuyển đổi
    Amplifying,              // Khuếch đại
    Reducing,                // Giảm
    Blocking,                // Chặn
    Boosting,                // Tăng cường
}

/// Status Magnitude
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMagnitude {
    pub base_value: f64,
    pub scaling_factor: f64,
    pub scaling_stat: String,
    pub min_value: f64,
    pub max_value: f64,
    pub calculation_formula: String,
}

/// Status Duration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDuration {
    pub base_duration: Duration,
    pub scaling_factor: f64,
    pub scaling_stat: String,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub calculation_formula: String,
}

/// Status Target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusTarget {
    Self,                    // Bản thân
    Target,                  // Mục tiêu
    Ally,                    // Đồng minh
    Enemy,                   // Kẻ thù
    Area,                    // Khu vực
    All,                     // Tất cả
    Custom(String),          // Tùy chỉnh
}

/// Status Source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusSource {
    Action(String),          // Hành động
    Element(String),         // Nguyên tố
    Item(String),            // Vật phẩm
    Skill(String),           // Kỹ năng
    Environment(String),     // Môi trường
    Custom(String),          // Tùy chỉnh
}
```

### **5. Status Interaction System**

```rust
/// Status Effect Interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectInteraction {
    pub interaction_id: String,
    pub interaction_name: String,
    pub interaction_name_vi: String,
    pub target_effect: String,
    pub interaction_type: StatusInteractionType,
    pub multiplier: f64,
    pub conditions: Vec<StatusCondition>,
    pub priority: u32,
    pub is_active: bool,
}

/// Status Condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCondition {
    pub condition_type: StatusConditionType,
    pub condition_value: f64,
    pub condition_operator: StatusConditionOperator,
    pub condition_target: String,
    pub condition_duration: Option<Duration>,
    pub is_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusConditionType {
    HealthPercentage,
    ManaPercentage,
    StaminaPercentage,
    QiPercentage,
    LifeForcePercentage,
    ElementMastery,
    StatusEffect,
    TerrainType,
    TimeOfDay,
    Weather,
    Season,
    MoonPhase,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusConditionOperator {
    GreaterThan,
    LessThan,
    EqualTo,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqualTo,
    Contains,
    NotContains,
    Custom(String),
}
```

## 🔧 **Configuration System**

### **1. Plugin Configuration**

```yaml
# status_plugins.yaml
version: 1.0

plugins:
  elemental_status_plugin:
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
    config:
      element_mastery_scaling: 0.01
      status_duration_base: 10.0
      status_magnitude_base: 1.0
      enable_elemental_interactions: true
      enable_status_combinations: true
      
  combat_status_plugin:
    plugin_id: "combat_status_plugin"
    plugin_name: "Combat Status Plugin"
    plugin_name_vi: "Plugin Trạng Thái Chiến Đấu"
    version: "1.0.0"
    author: "Chaos World Team"
    description: "Plugin for combat status effects"
    description_vi: "Plugin cho hiệu ứng trạng thái chiến đấu"
    enabled: true
    priority: 90
    dependencies: []
    load_order: 2
    config:
      combat_power_scaling: 0.02
      status_duration_base: 15.0
      status_magnitude_base: 1.5
      enable_combat_interactions: true
      enable_critical_status: true
      
  movement_status_plugin:
    plugin_id: "movement_status_plugin"
    plugin_name: "Movement Status Plugin"
    plugin_name_vi: "Plugin Trạng Thái Di Chuyển"
    version: "1.0.0"
    author: "Chaos World Team"
    description: "Plugin for movement status effects"
    description_vi: "Plugin cho hiệu ứng trạng thái di chuyển"
    enabled: true
    priority: 80
    dependencies: []
    load_order: 3
    config:
      movement_speed_scaling: 0.05
      status_duration_base: 8.0
      status_magnitude_base: 0.8
      enable_movement_interactions: true
      enable_terrain_effects: true
```

### **2. Status Categories Configuration**

```yaml
# status_categories.yaml
version: 1.0

categories:
  elemental:
    category: "Elemental"
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
    
  fire:
    category: "Elemental(Fire)"
    name: "Fire"
    name_vi: "Hỏa"
    description: "Fire elemental status effects"
    description_vi: "Hiệu ứng trạng thái hỏa"
    parent_category: "Elemental"
    child_categories: []
    tags: ["fire", "elemental", "heat", "burning"]
    properties:
      element_mastery_required: true
      element_interaction_bonus: 0.3
      status_duration_multiplier: 1.2
      burning_damage: 0.05
    interactions:
      - target_category: "Elemental(Water)"
        interaction_type: "Suppress"
        multiplier: 0.5
        conditions: []
        priority: 100
      - target_category: "Elemental(Wood)"
        interaction_type: "Amplify"
        multiplier: 2.0
        conditions: []
        priority: 100
    priority: 100
    is_active: true
```

### **3. Status Effects Configuration**

```yaml
# status_effects.yaml
version: 1.0

effects:
  burning:
    effect_id: "burning"
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

## 🚀 **Performance Optimization**

### **1. Plugin Caching**

```rust
/// Status Plugin Cache for performance optimization
pub struct StatusPluginCache {
    plugin_data_cache: HashMap<String, CachedPluginData>,
    status_effect_cache: HashMap<String, CachedStatusEffect>,
    status_category_cache: HashMap<String, CachedStatusCategory>,
    cache_ttl: Duration,
    last_update: HashMap<String, Instant>,
}

#[derive(Debug, Clone)]
pub struct CachedPluginData {
    pub plugin_info: StatusPluginInfo,
    pub status_categories: Vec<StatusCategory>,
    pub status_effects: Vec<StatusEffectDefinition>,
    pub status_interactions: Vec<StatusInteractionDefinition>,
    pub last_accessed: Instant,
}

#[derive(Debug, Clone)]
pub struct CachedStatusEffect {
    pub effect_definition: StatusEffectDefinition,
    pub plugin_id: String,
    pub last_accessed: Instant,
}

#[derive(Debug, Clone)]
pub struct CachedStatusCategory {
    pub category_definition: StatusCategoryDefinition,
    pub plugin_id: String,
    pub last_accessed: Instant,
}

impl StatusPluginCache {
    /// Get cached plugin data
    pub fn get_plugin_data(&self, plugin_id: &str) -> Option<&CachedPluginData> {
        self.plugin_data_cache.get(plugin_id)
    }
    
    /// Cache plugin data
    pub fn cache_plugin_data(&mut self, plugin_id: String, data: CachedPluginData) {
        self.plugin_data_cache.insert(plugin_id, data);
    }
    
    /// Get cached status effect
    pub fn get_status_effect(&self, effect_id: &str) -> Option<&CachedStatusEffect> {
        self.status_effect_cache.get(effect_id)
    }
    
    /// Cache status effect
    pub fn cache_status_effect(&mut self, effect_id: String, effect: CachedStatusEffect) {
        self.status_effect_cache.insert(effect_id, effect);
    }
    
    /// Check if cache is valid
    pub fn is_cache_valid(&self, key: &str) -> bool {
        if let Some(last_update) = self.last_update.get(key) {
            last_update.elapsed() < self.cache_ttl
        } else {
            false
        }
    }
    
    /// Invalidate cache
    pub fn invalidate_cache(&mut self, key: &str) {
        self.last_update.remove(key);
    }
    
    /// Cleanup expired cache
    pub fn cleanup_expired_cache(&mut self) {
        let now = Instant::now();
        let mut to_remove = Vec::new();
        
        for (key, last_update) in &self.last_update {
            if now.duration_since(*last_update) > self.cache_ttl {
                to_remove.push(key.clone());
            }
        }
        
        for key in to_remove {
            self.last_update.remove(&key);
        }
    }
}
```

### **2. Batch Processing**

```rust
/// Status Effect Batch Processor
pub struct StatusEffectBatchProcessor {
    plugin_registry: Arc<StatusPluginRegistry>,
    cache: StatusPluginCache,
    batch_size: usize,
}

impl StatusEffectBatchProcessor {
    /// Process multiple status effects in batch
    pub async fn process_status_effects_batch(
        &mut self,
        effects: Vec<StatusEffect>,
        actor: &Actor,
        context: &StatusContext
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        let mut results = Vec::new();
        
        // Group effects by plugin for efficient processing
        let mut grouped_effects: HashMap<String, Vec<StatusEffect>> = HashMap::new();
        for effect in effects {
            if let Some(plugin_id) = self.get_effect_plugin_id(&effect.effect_id) {
                grouped_effects.entry(plugin_id).or_insert_with(Vec::new).push(effect);
            }
        }
        
        // Process each plugin's effects
        for (plugin_id, plugin_effects) in grouped_effects {
            if let Some(plugin) = self.plugin_registry.get_plugin(&plugin_id) {
                let plugin_results = self.process_plugin_effects_batch(
                    plugin,
                    plugin_effects,
                    actor,
                    context
                ).await?;
                results.extend(plugin_results);
            }
        }
        
        Ok(results)
    }
    
    /// Process effects for a specific plugin
    async fn process_plugin_effects_batch(
        &self,
        plugin: &dyn StatusPlugin,
        effects: Vec<StatusEffect>,
        actor: &Actor,
        context: &StatusContext
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        let mut results = Vec::new();
        
        for effect in effects {
            let result = plugin.process_status_effect(effect, actor, context).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Get plugin ID for effect
    fn get_effect_plugin_id(&self, effect_id: &str) -> Option<String> {
        if let Some(cached_effect) = self.cache.get_status_effect(effect_id) {
            Some(cached_effect.plugin_id.clone())
        } else {
            None
        }
    }
}
```

## 🧪 **Testing Strategy**

### **1. Plugin Testing**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_registration() {
        let mut registry = StatusPluginRegistry::new();
        let plugin = create_test_plugin();
        let config = create_test_config();
        
        let result = registry.register_plugin(plugin, config).await;
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_status_category_creation() {
        let category = StatusCategory::Elemental(ElementalCategory::Fire);
        let definition = StatusCategoryDefinition {
            category: category.clone(),
            name: "Fire".to_string(),
            name_vi: "Hỏa".to_string(),
            description: "Fire elemental category".to_string(),
            description_vi: "Danh mục nguyên tố hỏa".to_string(),
            parent_category: Some(StatusCategory::Elemental(ElementalCategory::Elemental)),
            child_categories: vec![],
            tags: vec!["fire".to_string(), "elemental".to_string()],
            properties: HashMap::new(),
            interactions: vec![],
            priority: 100,
            is_active: true,
        };
        
        assert_eq!(definition.category, category);
        assert_eq!(definition.name, "Fire");
        assert_eq!(definition.name_vi, "Hỏa");
    }
    
    #[test]
    fn test_status_effect_definition() {
        let effect = StatusEffectDefinition {
            effect_id: "burning".to_string(),
            effect_name: "Burning".to_string(),
            effect_name_vi: "Cháy".to_string(),
            category: StatusCategory::Elemental(ElementalCategory::Fire),
            effect_type: StatusEffectType::Elemental(ElementalStatusType::Burning),
            magnitude: StatusMagnitude {
                base_value: 0.05,
                scaling_factor: 0.01,
                scaling_stat: "fire_mastery".to_string(),
                min_value: 0.01,
                max_value: 0.2,
                calculation_formula: "base_value + (scaling_stat * scaling_factor)".to_string(),
            },
            duration: StatusDuration {
                base_duration: Duration::from_secs_f64(10.0),
                scaling_factor: 0.1,
                scaling_stat: "fire_mastery".to_string(),
                min_duration: Duration::from_secs_f64(5.0),
                max_duration: Duration::from_secs_f64(30.0),
                calculation_formula: "base_duration + (scaling_stat * scaling_factor)".to_string(),
            },
            target: StatusTarget::Target,
            source: StatusSource::Element("fire".to_string()),
            conditions: vec![],
            interactions: vec![],
            immunity_list: vec![],
            movement_restrictions: vec![],
            visual_effects: vec![],
            audio_effects: vec![],
            properties: HashMap::new(),
            priority: 100,
            is_active: true,
            created_at: 1640995200,
            updated_at: 1640995200,
        };
        
        assert_eq!(effect.effect_id, "burning");
        assert_eq!(effect.effect_name, "Burning");
        assert_eq!(effect.effect_name_vi, "Cháy");
    }
}
```

### **2. Integration Testing**

```rust
#[tokio::test]
async fn test_plugin_system_integration() {
    let mut registry = StatusPluginRegistry::new();
    
    // Load test plugins
    registry.load_plugin_from_config("test_plugins/elemental_status_plugin.yaml").await?;
    registry.load_plugin_from_config("test_plugins/combat_status_plugin.yaml").await?;
    
    // Test plugin discovery
    let plugins = registry.get_all_plugins();
    assert_eq!(plugins.len(), 2);
    
    // Test status effect processing
    let effect = create_test_status_effect();
    let actor = create_test_actor();
    let context = create_test_status_context();
    
    let result = registry.process_status_effect(effect, &actor, &context).await?;
    assert!(result.success);
}

#[tokio::test]
async fn test_status_category_interactions() {
    let registry = StatusPluginRegistry::new();
    
    // Test category interactions
    let fire_category = StatusCategory::Elemental(ElementalCategory::Fire);
    let water_category = StatusCategory::Elemental(ElementalCategory::Water);
    
    let interaction = StatusCategoryInteraction {
        target_category: water_category,
        interaction_type: StatusInteractionType::Suppress,
        multiplier: 0.5,
        conditions: vec![],
        priority: 100,
    };
    
    assert_eq!(interaction.target_category, StatusCategory::Elemental(ElementalCategory::Water));
    assert_eq!(interaction.interaction_type, StatusInteractionType::Suppress);
    assert_eq!(interaction.multiplier, 0.5);
}
```

## 📝 **Implementation Notes**

### **1. Plugin Loading Strategy**
- **Lazy Loading**: Load plugins chỉ khi cần
- **Dependency Resolution**: Resolve dependencies trước khi load
- **Hot Reload**: Reload plugins mà không restart server
- **Error Handling**: Graceful error handling cho plugin failures

### **2. Performance Considerations**
- **Caching**: Cache plugin data và calculations
- **Batch Processing**: Process multiple effects efficiently
- **Memory Management**: Efficient memory usage cho plugins
- **Concurrency**: Thread-safe plugin operations

### **3. Security Considerations**
- **Plugin Validation**: Validate plugins trước khi load
- **Sandboxing**: Isolate plugin execution
- **Resource Limits**: Limit plugin resource usage
- **Audit Logging**: Log plugin operations

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
