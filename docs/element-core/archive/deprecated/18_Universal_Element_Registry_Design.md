# Universal Element Registry Design

## ⚠️ **DEPRECATION NOTICE**

**This document is deprecated and has been merged into [Element Registry Design](04_Element_Registry_Design.md).**

For new implementations, please use the unified registry approach described in the Element Registry Design document. Migration guide: [Migration Guide](21_Migration_Guide.md).

---

## 📋 **Tổng Quan** (DEPRECATED)

Universal Element Registry là một hệ thống trung tâm trong Element-Core cho phép nhiều hệ thống (Race-Core, Item-Core, Skill-Core, etc.) đăng ký và sử dụng element interactions mà không phụ thuộc vào một hệ thống cụ thể nào. Hệ thống này tạo ra một single source of truth cho tất cả element interactions và damage calculations.

## 🎯 **Mục Tiêu Thiết Kế**

### **1. Decoupled Architecture**
- **Race-Core**: Chỉ cần register racial elements
- **Item-Core**: Chỉ cần register equipment elements  
- **Skill-Core**: Chỉ cần register skill elements
- **Element-Core**: Central registry cho tất cả

### **2. Consistent Logic**
- **Single Source of Truth**: Tất cả interactions đi qua registry
- **Unified Calculation**: Cùng một logic tính damage
- **Centralized Balance**: Dễ dàng balance tất cả interactions
- **Easy Debugging**: Dễ debug interaction issues

### **3. Performance & Scalability**
- **Caching**: Registry có thể cache interactions
- **Lazy Loading**: Load interactions chỉ khi cần
- **Batch Operations**: Process multiple interactions cùng lúc
- **Memory Efficient**: Shared interaction matrix

## 🏗️ **Core Architecture**

### **Universal Element Registry Structure**
```rust
/// Universal Element Registry for all systems
/// 通用元素注册表 - 供所有系统使用
pub struct UniversalElementRegistry {
    /// Element definitions
    pub elements: HashMap<String, ElementDefinition>,
    /// Element interaction matrix
    pub interaction_matrix: HashMap<(String, String), ElementInteraction>,
    /// System registrations
    pub system_registrations: HashMap<String, SystemRegistration>,
    /// Element categories
    pub element_categories: HashMap<String, ElementCategory>,
    /// Interaction cache for performance
    pub interaction_cache: HashMap<(String, String), ElementInteraction>,
    /// Registry configuration
    pub config: RegistryConfig,
}

/// Element definition
/// 元素定义
pub struct ElementDefinition {
    pub element_id: String,                    // Element identifier
    pub name: String,                          // Element name
    pub aliases: HashMap<String, String>,      // Language aliases
    pub category: String,                      // Element category
    pub description: String,                   // Element description
    pub base_properties: ElementBaseProperties, // Base properties
    pub derived_stats: Vec<String>,            // Derived stats
    pub status_effects: Vec<StatusEffectDefinition>, // Status effects
    pub system_origin: String,                 // Which system registered this element
}

/// Element base properties
/// 元素基础属性
pub struct ElementBaseProperties {
    pub base_damage: f64,                      // Base damage
    pub base_defense: f64,                     // Base defense
    pub base_crit_rate: f64,                   // Base crit rate
    pub base_crit_damage: f64,                 // Base crit damage
    pub base_accuracy: f64,                    // Base accuracy
    pub base_status_probability: f64,          // Base status probability
    pub base_status_resistance: f64,           // Base status resistance
}

/// System registration
/// 系统注册
pub struct SystemRegistration {
    pub system_id: String,                     // System identifier
    pub system_name: String,                   // System name
    pub registered_elements: Vec<String>,      // Registered elements
    pub custom_interactions: HashMap<(String, String), ElementInteraction>, // Custom interactions
    pub element_overrides: HashMap<String, ElementOverride>, // Element overrides
    pub registration_time: SystemTime,         // Registration time
    pub system_priority: u32,                  // System priority (higher = more important)
}

/// Element override for specific systems
/// 系统特定元素覆盖
pub struct ElementOverride {
    pub system_id: String,                     // System identifier
    pub element_id: String,                    // Element identifier
    pub overrides: HashMap<String, serde_json::Value>, // Override values
    pub override_type: OverrideType,           // Override type
}

/// Override types
/// 覆盖类型
pub enum OverrideType {
    Additive,      // Add to base value
    Multiplicative, // Multiply base value
    Replacement,   // Replace base value
    Conditional,   // Conditional override
}
```

### **Element Interaction System**
```rust
/// Element interaction definition
/// 元素交互定义
pub struct ElementInteraction {
    pub source_element: String,                // Source element
    pub target_element: String,                // Target element
    pub interaction_type: InteractionType,     // Interaction type
    pub damage_multiplier: f64,                // Damage multiplier
    pub resistance_modifier: f64,              // Resistance modifier
    pub status_effect_modifier: f64,           // Status effect modifier
    pub description: String,                   // Description
    pub category: String,                      // Interaction category
    pub system_origin: String,                 // Which system defined this interaction
    pub priority: u32,                         // Interaction priority
}

/// Interaction types
/// 交互类型
pub enum InteractionType {
    Same,          // 相同 (0.8x damage, 1.5x resistance)
    Generating,    // 相生 (0.6x damage, 1.2x resistance)
    Overcoming,    // 相克 (1.5x damage, 0.7x resistance)
    Neutral,       // 中性 (1.0x damage, 1.0x resistance)
    Opposite,      // 对立 (2.0x damage, 0.5x resistance)
    Immune,        // 免疫 (0.0x damage, 1.0x resistance)
    Synergy,       // 协同 (1.2x damage, 0.9x resistance)
    Conflict,      // 冲突 (0.7x damage, 1.3x resistance)
}

impl InteractionType {
    pub fn get_damage_multiplier(&self) -> f64 {
        match self {
            InteractionType::Same => 0.8,
            InteractionType::Generating => 0.6,
            InteractionType::Overcoming => 1.5,
            InteractionType::Neutral => 1.0,
            InteractionType::Opposite => 2.0,
            InteractionType::Immune => 0.0,
            InteractionType::Synergy => 1.2,
            InteractionType::Conflict => 0.7,
        }
    }
    
    pub fn get_resistance_modifier(&self) -> f64 {
        match self {
            InteractionType::Same => 1.5,
            InteractionType::Generating => 1.2,
            InteractionType::Overcoming => 0.7,
            InteractionType::Neutral => 1.0,
            InteractionType::Opposite => 0.5,
            InteractionType::Immune => 1.0,
            InteractionType::Synergy => 0.9,
            InteractionType::Conflict => 1.3,
        }
    }
    
    pub fn get_status_effect_modifier(&self) -> f64 {
        match self {
            InteractionType::Same => 1.0,
            InteractionType::Generating => 0.8,
            InteractionType::Overcoming => 1.2,
            InteractionType::Neutral => 1.0,
            InteractionType::Opposite => 1.5,
            InteractionType::Immune => 0.0,
            InteractionType::Synergy => 1.1,
            InteractionType::Conflict => 0.9,
        }
    }
}
```

## 🔧 **Registry Implementation**

### **Core Registry Methods**
```rust
impl UniversalElementRegistry {
    /// Initialize registry with base elements
    /// 初始化注册表与基础元素
    pub fn new() -> Self {
        let mut registry = Self {
            elements: HashMap::new(),
            interaction_matrix: HashMap::new(),
            system_registrations: HashMap::new(),
            element_categories: HashMap::new(),
            interaction_cache: HashMap::new(),
            config: RegistryConfig::default(),
        };
        
        // Load base elements
        registry.load_base_elements().unwrap();
        
        // Load base interaction matrix
        registry.load_base_interaction_matrix().unwrap();
        
        registry
    }
    
    /// Register a system with the registry
    /// 注册系统到注册表
    pub fn register_system(&mut self, registration: SystemRegistration) -> Result<(), ElementCoreError> {
        let system_id = registration.system_id.clone();
        
        // Validate system registration
        self.validate_system_registration(&registration)?;
        
        // Check for conflicts with existing systems
        self.check_system_conflicts(&registration)?;
        
        // Register system
        self.system_registrations.insert(system_id.clone(), registration);
        
        // Update interaction matrix with custom interactions
        self.update_interaction_matrix()?;
        
        // Clear cache to force recalculation
        self.interaction_cache.clear();
        
        Ok(())
    }
    
    /// Get element interaction between two elements
    /// 获取两个元素之间的交互
    pub fn get_element_interaction(&self, source: &str, target: &str) -> ElementInteraction {
        // Check cache first
        if let Some(cached) = self.interaction_cache.get(&(source.to_string(), target.to_string())) {
            return cached.clone();
        }
        
        // Find interaction with highest priority
        let mut best_interaction = None;
        let mut highest_priority = 0;
        
        // Check system-specific interactions first
        for (_, system_reg) in &self.system_registrations {
            if let Some(interaction) = system_reg.custom_interactions.get(&(source.to_string(), target.to_string())) {
                if interaction.priority > highest_priority {
                    best_interaction = Some(interaction);
                    highest_priority = interaction.priority;
                }
            }
        }
        
        // Fall back to base interaction matrix
        if best_interaction.is_none() {
            best_interaction = self.interaction_matrix
                .get(&(source.to_string(), target.to_string()))
                .map(|i| i);
        }
        
        // Return best interaction or default
        best_interaction
            .cloned()
            .unwrap_or_else(|| ElementInteraction {
                source_element: source.to_string(),
                target_element: target.to_string(),
                interaction_type: InteractionType::Neutral,
                damage_multiplier: 1.0,
                resistance_modifier: 1.0,
                status_effect_modifier: 1.0,
                description: "Default neutral interaction".to_string(),
                category: "base".to_string(),
                system_origin: "element_core".to_string(),
                priority: 0,
            })
    }
    
    /// Calculate element interaction factor for Combat-Core
    /// 计算元素交互因子供Combat-Core使用
    pub fn calculate_element_interaction_factor(
        &self,
        source_element: &str,
        target_element: &str,
        system_modifiers: &HashMap<String, f64>,
    ) -> ElementInteractionResult {
        // Get base matrix interaction
        let matrix_interaction = self.get_element_interaction(source_element, target_element);
        
        // Apply all system modifiers
        let mut total_multiplier = matrix_interaction.damage_multiplier;
        for (system, modifier) in system_modifiers {
            total_multiplier *= modifier;
        }
        
        ElementInteractionResult {
            matrix_interaction,
            system_modifiers: system_modifiers.clone(),
            total_multiplier,
            resistance_modifier: matrix_interaction.resistance_modifier,
            status_effect_modifier: matrix_interaction.status_effect_modifier,
        }
    }
    
    /// Get system-specific modifier
    /// 获取系统特定修正器
    fn get_system_modifier(&self, system_id: &str, element_id: &str) -> f64 {
        self.system_registrations
            .get(system_id)
            .and_then(|reg| reg.element_overrides.get(element_id))
            .and_then(|override_val| override_val.overrides.get("damage_modifier"))
            .and_then(|val| val.as_f64())
            .unwrap_or(1.0)
    }
    
    /// Calculate mastery bonus
    /// 计算精通加成
    fn calculate_mastery_bonus(&self, attacker_mastery: f64, target_mastery: f64) -> f64 {
        let mastery_ratio = attacker_mastery / target_mastery.max(1.0);
        
        // Mastery bonus caps at 2.0x and minimum at 0.5x
        mastery_ratio.clamp(0.5, 2.0)
    }
    
    /// Create damage breakdown for debugging
    /// 创建伤害分解用于调试
    fn create_damage_breakdown(
        &self,
        base_damage: f64,
        interaction: &ElementInteraction,
        source_modifier: f64,
        target_modifier: f64,
        mastery_bonus: f64,
    ) -> DamageBreakdown {
        DamageBreakdown {
            base_damage,
            interaction_multiplier: interaction.damage_multiplier,
            resistance_modifier: interaction.resistance_modifier,
            source_modifier,
            target_modifier,
            mastery_bonus,
            status_effect_modifier: interaction.status_effect_modifier,
            total_multiplier: interaction.damage_multiplier * 
                             interaction.resistance_modifier * 
                             source_modifier * 
                             target_modifier * 
                             mastery_bonus,
        }
    }
}
```

### **Element Interaction Result Structures**
```rust
/// Element interaction calculation result for Combat-Core
/// 元素交互计算结果供Combat-Core使用
pub struct ElementInteractionResult {
    pub matrix_interaction: ElementInteraction, // Base matrix interaction
    pub system_modifiers: HashMap<String, f64>, // System-specific modifiers
    pub total_multiplier: f64,                  // Total damage multiplier
    pub resistance_modifier: f64,               // Resistance modifier
    pub status_effect_modifier: f64,            // Status effect modifier
}

/// Element power and interaction result
/// 元素力量和交互结果
pub struct ElementPowerResult {
    pub power_point: f64,                       // Total power point from derived stats
    pub interaction_factor: f64,                // Element interaction factor
    pub derived_stats: HashMap<String, f64>,    // All derived stats
    pub mastery_bonus: f64,                     // Mastery bonus
}
```

## 📊 **Base Element Matrix**

### **Five Elements Interaction Matrix**
```yaml
# element-core/configs/universal_element_matrix.yaml
universal_element_matrix:
  # Fire interactions
  fire:
    fire: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0, priority: 100 }
    water: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    earth: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    metal: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    wood: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    ice: { type: "opposite", damage: 2.0, resistance: 0.5, status: 1.5, priority: 100 }
    lightning: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
    air: { type: "synergy", damage: 1.2, resistance: 0.9, status: 1.1, priority: 100 }
  
  # Water interactions
  water:
    fire: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    water: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0, priority: 100 }
    earth: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    metal: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    wood: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    ice: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    lightning: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    air: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
  
  # Earth interactions
  earth:
    fire: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    water: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    earth: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0, priority: 100 }
    metal: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    wood: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    ice: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
    lightning: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    air: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
  
  # Metal interactions
  metal:
    fire: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    water: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    earth: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    metal: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0, priority: 100 }
    wood: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    ice: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    lightning: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    air: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
  
  # Wood interactions
  wood:
    fire: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    water: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    earth: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    metal: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    wood: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0, priority: 100 }
    ice: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
    lightning: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    air: { type: "synergy", damage: 1.2, resistance: 0.9, status: 1.1, priority: 100 }
  
  # Ice interactions
  ice:
    fire: { type: "opposite", damage: 2.0, resistance: 0.5, status: 1.5, priority: 100 }
    water: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    earth: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
    metal: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    wood: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
    ice: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0, priority: 100 }
    lightning: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
    air: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
  
  # Lightning interactions
  lightning:
    fire: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
    water: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    earth: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    metal: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8, priority: 100 }
    wood: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    ice: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
    lightning: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0, priority: 100 }
    air: { type: "synergy", damage: 1.2, resistance: 0.9, status: 1.1, priority: 100 }
  
  # Air interactions
  air:
    fire: { type: "synergy", damage: 1.2, resistance: 0.9, status: 1.1, priority: 100 }
    water: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
    earth: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    metal: { type: "neutral", damage: 1.0, resistance: 1.0, status: 1.0, priority: 100 }
    wood: { type: "synergy", damage: 1.2, resistance: 0.9, status: 1.1, priority: 100 }
    ice: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2, priority: 100 }
    lightning: { type: "synergy", damage: 1.2, resistance: 0.9, status: 1.1, priority: 100 }
    air: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0, priority: 100 }
```

## 🔧 **System Integration Examples**

### **1. Race-Core Integration**
```rust
/// Race-Core integration with Universal Element Registry
/// Race-Core集成通用元素注册表
impl RaceCore {
    pub fn register_with_element_registry(&self, registry: &mut UniversalElementRegistry) -> Result<(), ElementCoreError> {
        // Register race-specific elements
        let race_elements = vec![
            "fire_spirit".to_string(),
            "water_spirit".to_string(),
            "ice_fire_dragon".to_string(),
            "water_fire_spirit".to_string(),
            "earth_dragon".to_string(),
            "lightning_tiger".to_string(),
        ];
        
        // Register custom interactions
        let mut custom_interactions = HashMap::new();
        
        // Ice Fire Dragon interactions
        custom_interactions.insert(
            ("fire".to_string(), "ice".to_string()),
            ElementInteraction {
                source_element: "fire".to_string(),
                target_element: "ice".to_string(),
                interaction_type: InteractionType::Opposite,
                damage_multiplier: 2.0,
                resistance_modifier: 0.5,
                status_effect_modifier: 1.5,
                description: "Fire vs Ice - Opposite elements".to_string(),
                category: "racial".to_string(),
                system_origin: "race_core".to_string(),
                priority: 200, // Higher priority than base interactions
            }
        );
        
        // Register system
        let system_registration = SystemRegistration {
            system_id: "race_core".to_string(),
            system_name: "Race Core System".to_string(),
            registered_elements: race_elements,
            custom_interactions,
            element_overrides: HashMap::new(),
            registration_time: SystemTime::now(),
            system_priority: 100,
        };
        
        registry.register_system(system_registration)?;
        Ok(())
    }
}
```

### **2. Item-Core Integration**
```rust
/// Item-Core integration with Universal Element Registry
/// Item-Core集成通用元素注册表
impl ItemCore {
    pub fn register_with_element_registry(&self, registry: &mut UniversalElementRegistry) -> Result<(), ElementCoreError> {
        // Register item-specific elements
        let item_elements = vec![
            "fire_sword".to_string(),
            "ice_armor".to_string(),
            "lightning_staff".to_string(),
            "earth_shield".to_string(),
            "water_amulet".to_string(),
            "air_boots".to_string(),
        ];
        
        // Register custom interactions
        let mut custom_interactions = HashMap::new();
        
        // Fire Sword vs Ice Armor
        custom_interactions.insert(
            ("fire_sword".to_string(), "ice_armor".to_string()),
            ElementInteraction {
                source_element: "fire_sword".to_string(),
                target_element: "ice_armor".to_string(),
                interaction_type: InteractionType::Overcoming,
                damage_multiplier: 1.8,
                resistance_modifier: 0.6,
                status_effect_modifier: 1.2,
                description: "Fire Sword vs Ice Armor - Equipment interaction".to_string(),
                category: "equipment".to_string(),
                system_origin: "item_core".to_string(),
                priority: 150,
            }
        );
        
        // Register system
        let system_registration = SystemRegistration {
            system_id: "item_core".to_string(),
            system_name: "Item Core System".to_string(),
            registered_elements: item_elements,
            custom_interactions,
            element_overrides: HashMap::new(),
            registration_time: SystemTime::now(),
            system_priority: 80,
        };
        
        registry.register_system(system_registration)?;
        Ok(())
    }
}
```

### **3. Skill-Core Integration**
```rust
/// Skill-Core integration with Universal Element Registry
/// Skill-Core集成通用元素注册表
impl SkillCore {
    pub fn register_with_element_registry(&self, registry: &mut UniversalElementRegistry) -> Result<(), ElementCoreError> {
        // Register skill-specific elements
        let skill_elements = vec![
            "fireball".to_string(),
            "ice_shard".to_string(),
            "lightning_bolt".to_string(),
            "earth_spike".to_string(),
            "water_blast".to_string(),
            "air_slash".to_string(),
        ];
        
        // Register custom interactions
        let mut custom_interactions = HashMap::new();
        
        // Fireball vs Ice Shard
        custom_interactions.insert(
            ("fireball".to_string(), "ice_shard".to_string()),
            ElementInteraction {
                source_element: "fireball".to_string(),
                target_element: "ice_shard".to_string(),
                interaction_type: InteractionType::Opposite,
                damage_multiplier: 2.2,
                resistance_modifier: 0.4,
                status_effect_modifier: 1.8,
                description: "Fireball vs Ice Shard - Skill interaction".to_string(),
                category: "skill".to_string(),
                system_origin: "skill_core".to_string(),
                priority: 120,
            }
        );
        
        // Register system
        let system_registration = SystemRegistration {
            system_id: "skill_core".to_string(),
            system_name: "Skill Core System".to_string(),
            registered_elements: skill_elements,
            custom_interactions,
            element_overrides: HashMap::new(),
            registration_time: SystemTime::now(),
            system_priority: 90,
        };
        
        registry.register_system(system_registration)?;
        Ok(())
    }
}
```

## 🎯 **Combat Integration**

### **Element-Core to Combat-Core Flow**
```rust
/// Element-Core provides power and interaction data to Combat-Core
/// Element-Core提供力量和交互数据给Combat-Core
impl ElementCoreAggregator {
    pub async fn calculate_element_power_and_interaction(
        &self,
        actor: &Actor,
        source_element: &str,
        target_element: &str,
        system_modifiers: HashMap<String, f64>,
    ) -> ElementCoreResult<ElementPowerResult> {
        // Step 1: Calculate derived stats from all external systems
        let derived_stats = self.calculate_derived_stats(actor, source_element).await?;
        let power_point = derived_stats.get("power_point").unwrap_or(&0.0);
        
        // Step 2: Calculate element interaction factor
        let interaction_result = self.registry.calculate_element_interaction_factor(
            source_element,
            target_element,
            &system_modifiers
        );
        
        // Step 3: Calculate mastery bonus
        let attacker_mastery = self.get_element_mastery(actor, source_element);
        let target_mastery = self.get_target_mastery(target_element);
        let mastery_bonus = (attacker_mastery - target_mastery) * 0.01;
        
        Ok(ElementPowerResult {
            power_point: *power_point,
            interaction_factor: interaction_result.total_multiplier,
            derived_stats,
            mastery_bonus,
        })
    }
}

/// Combat Core integration with Element-Core
/// Combat Core集成Element-Core
impl CombatCore {
    pub async fn calculate_damage_with_element_core(
        &self,
        action: &Action,
        attacker: &Actor,
        target: &Actor,
        element_core: &ElementCoreAggregator,
    ) -> CombatCoreResult<f64> {
        // Get system modifiers from external systems
        let system_modifiers = self.collect_system_modifiers(attacker, target).await?;
        
        // Get element power and interaction from Element-Core
        let element_result = element_core.calculate_element_power_and_interaction(
            attacker,
            &action.element_type,
            &target.element_type,
            system_modifiers,
        ).await?;
        
        // Calculate final damage using Combat-Core formula
        let base_damage = action.base_damage;
        let power_damage = element_result.power_point - self.get_target_defense(target).await?;
        let pre_multiplier_damage = base_damage + power_damage.max(0.0);
        
        // Apply element interaction factor and mastery bonus
        let final_damage = pre_multiplier_damage * 
                          element_result.interaction_factor * 
                          (1.0 + element_result.mastery_bonus);
        
        Ok(final_damage)
    }
    
    async fn collect_system_modifiers(&self, attacker: &Actor, target: &Actor) -> CombatCoreResult<HashMap<String, f64>> {
        let mut modifiers = HashMap::new();
        
        // Race-Core modifiers
        if let Some(race_modifier) = self.get_race_modifier(attacker, target).await? {
            modifiers.insert("race_core".to_string(), race_modifier);
        }
        
        // Status-Core modifiers
        if let Some(status_modifier) = self.get_status_modifier(attacker, target).await? {
            modifiers.insert("status_core".to_string(), status_modifier);
        }
        
        // Item-Core modifiers
        if let Some(item_modifier) = self.get_item_modifier(attacker, target).await? {
            modifiers.insert("item_core".to_string(), item_modifier);
        }
        
        Ok(modifiers)
    }
}
```

## 📈 **Damage Calculation Flow**

### **Complete Flow Example**
```rust
/// Example: Fire vs Wood Spirit with Fire Curse
/// 示例：火 vs 木精灵 with 火诅咒
async fn example_damage_calculation() {
    // Step 1: Element-Core calculates derived stats
    let fire_power_point = 1850.0; // From RPG + Cultivation + Items + Talents + Destiny
    
    // Step 2: Collect system modifiers
    let system_modifiers = HashMap::from([
        ("race_core".to_string(), 2.0),    // Wood Spirit takes 2x fire damage
        ("status_core".to_string(), 1.5),  // Fire Curse adds 1.5x fire damage
    ]);
    
    // Step 3: Element-Core calculates interaction factor
    let matrix_interaction = 1.5; // Fire vs Wood base interaction
    let total_interaction_factor = matrix_interaction * 2.0 * 1.5; // = 4.5
    
    // Step 4: Combat-Core calculates final damage
    let base_damage = 500.0;
    let target_defense = 360.0;
    let power_damage = fire_power_point - target_defense; // 1850 - 360 = 1490
    let pre_multiplier_damage = base_damage + power_damage; // 500 + 1490 = 1990
    
    // Step 5: Apply interaction factor and mastery bonus
    let mastery_bonus = 0.5; // 50% from mastery difference
    let final_damage = pre_multiplier_damage * total_interaction_factor * (1.0 + mastery_bonus);
    // = 1990 * 4.5 * 1.5 = 13,432.5
    
    println!("Final Damage: {}", final_damage);
}
```

### **Performance Optimizations**
```rust
/// Performance optimizations for Universal Element Registry
/// 通用元素注册表性能优化
impl UniversalElementRegistry {
    /// Cache interaction results for performance
    /// 缓存交互结果以提高性能
    pub fn get_cached_interaction(&self, source: &str, target: &str) -> Option<ElementInteraction> {
        self.interaction_cache.get(&(source.to_string(), target.to_string())).cloned()
    }
    
    /// Pre-calculate common interactions
    /// 预计算常见交互
    pub fn precalculate_common_interactions(&mut self) {
        let common_elements = vec!["fire", "water", "earth", "metal", "wood", "ice", "lightning", "air"];
        
        for source in &common_elements {
            for target in &common_elements {
                let interaction = self.get_element_interaction(source, target);
                self.interaction_cache.insert(
                    (source.to_string(), target.to_string()),
                    interaction
                );
            }
        }
    }
    
    /// Batch calculate multiple interaction factors
    /// 批量计算多个交互因子
    pub fn batch_calculate_interaction_factors(
        &self,
        interactions: Vec<(String, String, HashMap<String, f64>)>,
    ) -> Vec<ElementInteractionResult> {
        interactions
            .into_iter()
            .map(|(source, target, system_modifiers)| {
                self.calculate_element_interaction_factor(&source, &target, &system_modifiers)
            })
            .collect()
    }
}
```

## 🚀 **Implementation Strategy**

### **Phase 1: Core Registry (2 weeks)**
1. **Create Universal Element Registry** structure
2. **Implement base interaction matrix** với Five Elements
3. **Add system registration** mechanism
4. **Create element interaction factor calculation** logic
5. **Add caching** for performance

### **Phase 2: Element-Core Integration (2 weeks)**
1. **Update Element-Core Aggregator** để use registry
2. **Implement calculate_element_power_and_interaction** method
3. **Add system modifier collection** mechanism
4. **Test derived stats calculation** với external systems
5. **Validate interaction factor calculation**

### **Phase 3: Combat-Core Integration (2 weeks)**
1. **Update Combat-Core** để use Element-Core results
2. **Implement collect_system_modifiers** method
3. **Add final damage calculation** với interaction factors
4. **Test complete damage flow** với examples
5. **Validate performance** và caching

### **Phase 4: System Registration (2 weeks)**
1. **Register Race-Core** với racial elements và modifiers
2. **Register Status-Core** với status effect modifiers
3. **Register Item-Core** với equipment modifiers
4. **Register Skill-Core** với skill modifiers
5. **Test all system integrations**

### **Phase 5: Production Ready (1 week)**
1. **Add comprehensive logging** cho debugging
2. **Create configuration files** cho all interactions
3. **Add validation** cho system registrations
4. **Create documentation** và examples
5. **Final testing** và deployment

## 📊 **Benefits Summary**

### **1. Decoupled Architecture**
- **Race-Core**: Chỉ cần register racial elements và modifiers
- **Status-Core**: Chỉ cần register status effect modifiers
- **Item-Core**: Chỉ cần register equipment modifiers
- **Skill-Core**: Chỉ cần register skill modifiers
- **Element-Core**: Central registry cho tất cả interactions

### **2. Consistent Logic**
- **Single Source of Truth**: Tất cả interactions đi qua registry
- **Unified Calculation**: Cùng một logic tính interaction factors
- **Centralized Balance**: Dễ dàng balance tất cả interactions
- **Easy Debugging**: Dễ debug interaction issues

### **3. Performance & Scalability**
- **Caching**: Registry có thể cache interactions
- **Lazy Loading**: Load interactions chỉ khi cần
- **Batch Operations**: Process multiple interactions cùng lúc
- **Memory Efficient**: Shared interaction matrix

### **4. Extensibility**
- **New Systems**: Easy to add new systems với modifiers
- **New Elements**: Easy to add new elements
- **New Interactions**: Easy to add custom interactions
- **New Categories**: Easy to add new interaction categories

### **5. Integration Benefits**
- **Element-Core**: Tính derived stats và interaction factors
- **Combat-Core**: Sử dụng factors để tính final damage
- **Clear Separation**: Element logic vs Combat logic
- **Performance**: Caching và batch processing

## 🎯 **Next Steps**

1. **Review và feedback** trên design này
2. **Implement Universal Element Registry** trong Element-Core
3. **Update Element-Core Aggregator** để use registry
4. **Update Combat-Core** để use Element-Core results
5. **Register existing systems** (Race-Core, Status-Core, Item-Core, Skill-Core)
6. **Test complete damage flow** với examples
7. **Add advanced features** và optimizations

## 📚 **Related Documents**

- [00_Element_Core_Overview.md](./00_Element_Core_Overview.md) - Element Core overview
- [02_Multi_System_Integration_Design.md](./02_Multi_System_Integration_Design.md) - Multi-system integration
- [10_Element_Interaction_System_Design.md](./10_Element_Interaction_System_Design.md) - Element interactions
- [16_Hybrid_Subsystem_Design.md](./16_Hybrid_Subsystem_Design.md) - Hybrid elements system
- [17_Elemental_Category_System_Design.md](./17_Elemental_Category_System_Design.md) - Element categorization
- [19_Stats_Distribution_Design.md](./19_Stats_Distribution_Design.md) - External system integration

---

**Last Updated**: 2024-12-19  
**Version**: 2.0  
**Status**: Updated to match Combat-Core flow  
**Maintainer**: Chaos World Team
