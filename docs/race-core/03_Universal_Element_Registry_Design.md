# Universal Element Registry Design

## 📋 **Tổng Quan**

Thiết kế này tạo ra một Universal Element Registry trong Element-Core, cho phép nhiều hệ thống (Race-Core, Item-Core, Skill-Core, etc.) đăng ký và sử dụng element interactions mà không phụ thuộc vào một hệ thống cụ thể nào.

## 🎯 **So Sánh với Pokemon System**

### **1. Pokemon System Design**

#### **A. Core Mechanics**
- **18 Element Types**: Normal, Fire, Water, Electric, Grass, Ice, Fighting, Poison, Ground, Flying, Psychic, Bug, Rock, Ghost, Dragon, Dark, Steel, Fairy
- **Type Effectiveness Matrix**: 18x18 matrix với 4 values (0x, 0.5x, 1x, 2x)
- **Dual Types**: Pokemon có thể có 1-2 types, damage multiplier = type1_multiplier * type2_multiplier
- **Simple Calculation**: `final_damage = base_damage * type_effectiveness`

#### **B. Example: Charizard (Fire/Flying)**
```
Water attack vs Charizard:
- Water vs Fire = 2x (super effective)
- Water vs Flying = 1x (normal)
- Final = 2x * 1x = 2x damage

Electric attack vs Charizard:
- Electric vs Fire = 1x (normal)
- Electric vs Flying = 2x (super effective)
- Final = 1x * 2x = 2x damage

Rock attack vs Charizard:
- Rock vs Fire = 0.5x (not very effective)
- Rock vs Flying = 2x (super effective)
- Final = 0.5x * 2x = 1x damage
```

### **2. Our System vs Pokemon**

#### **A. Similarities**
- **Element Types**: Both have multiple element types
- **Effectiveness Matrix**: Both use multiplier-based interactions
- **Dual Elements**: Both support multiple elements per entity
- **Damage Calculation**: Both multiply base damage by effectiveness

#### **B. Differences**
- **Complexity**: Our system has more complex interactions (generating, overcoming, neutral, same, opposite)
- **Mastery System**: Our system includes mastery levels affecting damage
- **Racial Elements**: Our system has racial-specific element affinities
- **Status Effects**: Our system includes status effect interactions
- **Probability Engine**: Our system uses sigmoid functions for complex calculations

## 🏗️ **Universal Element Registry Architecture**

### **Core Structure**
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
}

/// Element base properties
/// 元素基础属性
pub struct ElementBaseProperties {
    pub base_damage: f64,                      // Base damage
    pub base_defense: f64,                     // Base defense
    pub base_crit_rate: f64,                   // Base crit rate
    pub base_crit_damage: f64,                 // Base crit damage
    pub base_accuracy: f64,                    // Base accuracy
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
}

/// Element override for specific systems
/// 系统特定元素覆盖
pub struct ElementOverride {
    pub system_id: String,                     // System identifier
    pub element_id: String,                    // Element identifier
    pub overrides: HashMap<String, serde_json::Value>, // Override values
}
```

### **Element Interaction Matrix**
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
        }
    }
}
```

## 🔧 **System Registration Examples**

### **1. Race-Core Registration**
```rust
/// Race-Core registration with Universal Element Registry
/// Race-Core注册到通用元素注册表
impl RaceCore {
    pub fn register_with_element_registry(&self, registry: &mut UniversalElementRegistry) -> Result<(), ElementCoreError> {
        // Register race-specific elements
        let race_elements = vec![
            "fire_spirit".to_string(),
            "water_spirit".to_string(),
            "ice_fire_dragon".to_string(),
            "water_fire_spirit".to_string(),
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
        };
        
        registry.register_system(system_registration)?;
        Ok(())
    }
}
```

### **2. Item-Core Registration**
```rust
/// Item-Core registration with Universal Element Registry
/// Item-Core注册到通用元素注册表
impl ItemCore {
    pub fn register_with_element_registry(&self, registry: &mut UniversalElementRegistry) -> Result<(), ElementCoreError> {
        // Register item-specific elements
        let item_elements = vec![
            "fire_sword".to_string(),
            "ice_armor".to_string(),
            "lightning_staff".to_string(),
            "earth_shield".to_string(),
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
        };
        
        registry.register_system(system_registration)?;
        Ok(())
    }
}
```

### **3. Skill-Core Registration**
```rust
/// Skill-Core registration with Universal Element Registry
/// Skill-Core注册到通用元素注册表
impl SkillCore {
    pub fn register_with_element_registry(&self, registry: &mut UniversalElementRegistry) -> Result<(), ElementCoreError> {
        // Register skill-specific elements
        let skill_elements = vec![
            "fireball".to_string(),
            "ice_shard".to_string(),
            "lightning_bolt".to_string(),
            "earth_spike".to_string(),
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
        };
        
        registry.register_system(system_registration)?;
        Ok(())
    }
}
```

## 🎯 **Universal Element Registry Implementation**

### **Core Registry Methods**
```rust
impl UniversalElementRegistry {
    /// Register a system with the registry
    /// 注册系统到注册表
    pub fn register_system(&mut self, registration: SystemRegistration) -> Result<(), ElementCoreError> {
        let system_id = registration.system_id.clone();
        
        // Validate system registration
        self.validate_system_registration(&registration)?;
        
        // Register system
        self.system_registrations.insert(system_id, registration);
        
        // Update interaction matrix with custom interactions
        self.update_interaction_matrix()?;
        
        Ok(())
    }
    
    /// Get element interaction between two elements
    /// 获取两个元素之间的交互
    pub fn get_element_interaction(&self, source: &str, target: &str) -> ElementInteraction {
        // Check for custom interactions first
        for (_, system_reg) in &self.system_registrations {
            if let Some(interaction) = system_reg.custom_interactions.get(&(source.to_string(), target.to_string())) {
                return interaction.clone();
            }
        }
        
        // Fall back to base interaction matrix
        self.interaction_matrix
            .get(&(source.to_string(), target.to_string()))
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
            })
    }
    
    /// Calculate damage with all system interactions
    /// 计算所有系统交互的伤害
    pub fn calculate_universal_damage(
        &self,
        source_element: &str,
        target_element: &str,
        base_damage: f64,
        source_system: &str,
        target_system: &str,
    ) -> f64 {
        // Get base interaction
        let interaction = self.get_element_interaction(source_element, target_element);
        
        // Apply system-specific modifiers
        let source_modifier = self.get_system_modifier(source_system, source_element);
        let target_modifier = self.get_system_modifier(target_system, target_element);
        
        // Calculate final damage
        let final_damage = base_damage * 
                          interaction.damage_multiplier * 
                          interaction.resistance_modifier * 
                          source_modifier * 
                          target_modifier;
        
        // Apply minimum damage (10% of base)
        final_damage.max(base_damage * 0.1)
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
}
```

## 📊 **Element Interaction Matrix Table**

### **Base Five Elements Matrix**
```yaml
# element-core/configs/universal_element_matrix.yaml
universal_element_matrix:
  # Fire interactions
  fire:
    fire: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0 }
    water: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2 }
    earth: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8 }
    metal: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2 }
    wood: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8 }
  
  # Water interactions
  water:
    fire: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2 }
    water: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0 }
    earth: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2 }
    metal: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8 }
    wood: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8 }
  
  # Earth interactions
  earth:
    fire: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8 }
    water: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2 }
    earth: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0 }
    metal: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8 }
    wood: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2 }
  
  # Metal interactions
  metal:
    fire: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2 }
    water: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8 }
    earth: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8 }
    metal: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0 }
    wood: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2 }
  
  # Wood interactions
  wood:
    fire: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8 }
    water: { type: "generating", damage: 0.6, resistance: 1.2, status: 0.8 }
    earth: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2 }
    metal: { type: "overcoming", damage: 1.5, resistance: 0.7, status: 1.2 }
    wood: { type: "same", damage: 0.8, resistance: 1.5, status: 1.0 }
```

## 🚀 **Benefits của Universal Registry**

### **1. Decoupled Systems**
- **Race-Core**: Chỉ cần register racial elements
- **Item-Core**: Chỉ cần register equipment elements
- **Skill-Core**: Chỉ cần register skill elements
- **Element-Core**: Central registry cho tất cả

### **2. Easy Extension**
- **New Systems**: Easy to add new systems
- **New Elements**: Easy to add new elements
- **New Interactions**: Easy to add custom interactions
- **New Categories**: Easy to add new interaction categories

### **3. Consistent Logic**
- **Single Source**: All interactions go through registry
- **Unified Calculation**: Same damage calculation logic
- **Centralized Balance**: Easy to balance all interactions
- **Debugging**: Easy to debug interaction issues

### **4. Performance**
- **Caching**: Registry can cache interactions
- **Lazy Loading**: Load interactions only when needed
- **Batch Operations**: Process multiple interactions at once
- **Memory Efficient**: Shared interaction matrix

## 🎯 **Implementation Strategy**

### **Phase 1: Core Registry**
1. **Create Universal Element Registry** trong Element-Core
2. **Implement base interaction matrix** với Five Elements
3. **Add system registration** mechanism
4. **Test với simple examples**

### **Phase 2: System Integration**
1. **Register Race-Core** với racial elements
2. **Register Item-Core** với equipment elements
3. **Register Skill-Core** với skill elements
4. **Update existing systems** để use registry

### **Phase 3: Advanced Features**
1. **Add complex interactions** (multi-element, conditional)
2. **Implement interaction categories** (racial, equipment, skill)
3. **Add UI display** cho interaction matrix
4. **Balance testing** và fine-tuning

## 🎯 **Next Steps**

1. **Review và feedback** trên design này
2. **Implement Universal Element Registry** trong Element-Core
3. **Create base interaction matrix** với Five Elements
4. **Register existing systems** (Race-Core, Item-Core, Skill-Core)
5. **Test integration** và performance

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
