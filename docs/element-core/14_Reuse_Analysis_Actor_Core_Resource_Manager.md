# Reuse Analysis: Actor Core & Resource Manager

## 📋 **Tổng Quan**

Tài liệu này phân tích khả năng tái sử dụng Actor Core và Resource Manager cho Element Core, xác định những gì có thể tận dụng và những gì cần implement mới.

## 🔍 **Phân Tích Actor Core**

### **✅ Có Thể Tái Sử Dụng**

#### **1. SystemResourceCalculator Trait**
```rust
// Actor Core đã có sẵn
#[async_trait]
pub trait SystemResourceCalculator: Send + Sync {
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>>;
    fn system_id(&self) -> &str;
    fn affects_resource(&self, resource_id: &str) -> bool;
    async fn notify_stat_change(&self, actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()>;
    fn get_resource_dependencies(&self) -> Vec<String>;
    fn get_resource_categories(&self) -> Vec<ResourceCategory>;
    async fn is_active(&self, actor: &Actor) -> ActorCoreResult<bool>;
}
```

**Element Core có thể tận dụng:**
- ✅ **Elemental Mastery System**: Implement `SystemResourceCalculator`
- ✅ **Element Stats Provider**: Implement `SystemResourceCalculator`
- ✅ **Resource Dependencies**: Track dependencies between elements
- ✅ **Resource Categories**: Map elements to `ResourceCategory`

#### **2. ResourceCategory Enum**
```rust
// Actor Core đã có sẵn
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceCategory {
    Health,      // HP, lifespan
    Energy,      // Mana, spiritual energy
    Physical,    // Stamina, vitality
    Cultivation, // Qi, dao energy
    Special,     // Shield, temporary effects
}
```

**Element Core có thể tận dụng:**
- ✅ **Element Mastery**: `Cultivation` category
- ✅ **Element Stats**: `Special` category
- ✅ **Element Resources**: `Energy` category cho mana/qi

#### **3. BaseSystemResourceCalculator**
```rust
// Actor Core đã có sẵn
pub struct BaseSystemResourceCalculator {
    system_id: String,
    resource_categories: Vec<ResourceCategory>,
    resource_dependencies: Vec<String>,
    resource_definitions: HashMap<String, SystemResourceDefinition>,
}
```

**Element Core có thể tận dụng:**
- ✅ **Element System Base**: Inherit từ `BaseSystemResourceCalculator`
- ✅ **Resource Definitions**: Define element resources
- ✅ **Dependency Management**: Track element dependencies

### **🔄 Cần Mở Rộng**

#### **1. ResourceCategory Enum**
```rust
// Cần thêm categories mới
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceCategory {
    Health,      // HP, lifespan
    Energy,      // Mana, spiritual energy
    Physical,    // Stamina, vitality
    Cultivation, // Qi, dao energy
    Special,     // Shield, temporary effects
    
    // NEW: Element-specific categories
    Elemental,   // Element mastery, element stats
    Combat,      // Combat stats, damage, defense
    Social,      // Leadership, teaching, crafting
    Perception,  // Detection, sensitivity
}
```

#### **2. SystemResourceDefinition**
```rust
// Cần mở rộng cho element-specific resources
pub struct SystemResourceDefinition {
    pub id: String,
    pub name: String,
    pub category: ResourceCategory,
    pub base_value_calculator: fn(&Actor) -> f64,
    pub regen_rate_calculator: fn(&Actor) -> f64,
    pub dependencies: Vec<String>,
    pub tags: HashMap<String, String>,
    
    // NEW: Element-specific fields
    pub element_type: Option<String>,        // For element-specific resources
    pub stat_type: Option<DerivedStatType>,  // For derived stats
    pub calculation_formula: Option<String>, // For complex calculations
}
```

## 🔍 **Phân Tích Resource Manager**

### **✅ Có Thể Tái Sử Dụng**

#### **1. String-Based Formula Evaluation**
```rust
// RPG Resource Manager đã có sẵn
pub struct RpgResourceDefinition {
    pub base_formula: String,    // "strength * 2 + level * 10"
    pub max_formula: String,     // "vitality * 5 + level * 20"
    pub regen_rate: f64,
}
```

**Element Core có thể tận dụng:**
- ✅ **Derived Stats Calculation**: Sử dụng string formulas
- ✅ **Element Mastery Calculation**: Sử dụng string formulas
- ✅ **Dynamic Configuration**: Runtime formula evaluation

#### **2. Resource Dependencies**
```rust
// RPG Resource Manager đã có sẵn
pub struct RpgResourceDefinition {
    pub dependencies: Vec<String>, // ["strength", "vitality"]
}
```

**Element Core có thể tận dụng:**
- ✅ **Element Dependencies**: Track dependencies between elements
- ✅ **Stat Dependencies**: Track dependencies between stats
- ✅ **Cascade Updates**: Update dependent stats when primary stats change

#### **3. Resource Categories**
```rust
// Magic Resource Manager đã có sẵn
pub enum MagicResourceCategory {
    Mana,
    SpellSlots,
    ArcaneEnergy,
    MagicalFocus,
}
```

**Element Core có thể tận dụng:**
- ✅ **Element Resources**: Map elements to resource categories
- ✅ **Resource Management**: Use existing resource management logic

### **🔄 Cần Mở Rộng**

#### **1. Element-Specific Resource Categories**
```rust
// Cần thêm element-specific categories
pub enum ElementResourceCategory {
    // Existing categories
    Mana,
    SpellSlots,
    ArcaneEnergy,
    MagicalFocus,
    
    // NEW: Element-specific categories
    ElementMastery,        // Fire mastery, Water mastery, etc.
    ElementStats,          // Fire attack, Water defense, etc.
    ElementResources,      // Fire mana, Water qi, etc.
    ElementEffects,        // Fire regeneration, Water healing, etc.
}
```

#### **2. Element-Specific Resource Definitions**
```rust
// Cần thêm element-specific resource definitions
pub struct ElementResourceDefinition {
    pub id: String,
    pub name: String,
    pub category: ElementResourceCategory,
    pub element_type: String,              // "fire", "water", etc.
    pub base_formula: String,              // "fire_mastery * 0.01"
    pub max_formula: String,               // "fire_mastery * 0.1"
    pub regen_rate: f64,
    pub dependencies: Vec<String>,
    
    // NEW: Element-specific fields
    pub stat_type: DerivedStatType,        // For derived stats
    pub calculation_context: String,       // For complex calculations
    pub element_interactions: Vec<String>, // For element interactions
}
```

## 🎯 **Mapping Element Core Features**

### **1. Elemental Mastery System**

#### **Tận Dụng Actor Core**
```rust
// Elemental Mastery System implements SystemResourceCalculator
pub struct ElementalMasterySystem {
    // Inherit from BaseSystemResourceCalculator
    base: BaseSystemResourceCalculator,
    
    // Element-specific fields
    element_definitions: HashMap<String, ElementDefinition>,
    mastery_calculator: MasteryCalculator,
}

impl SystemResourceCalculator for ElementalMasterySystem {
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut resources = HashMap::new();
        
        // Calculate mastery for each element
        for (element_id, definition) in &self.element_definitions {
            let mastery_level = self.mastery_calculator.calculate_mastery(actor, element_id).await?;
            resources.insert(format!("{}_mastery", element_id), mastery_level);
        }
        
        Ok(resources)
    }
    
    fn get_resource_categories(&self) -> Vec<ResourceCategory> {
        vec![ResourceCategory::Cultivation, ResourceCategory::Elemental]
    }
}
```

#### **Tận Dụng Resource Manager**
```rust
// Element Mastery Resources
pub struct ElementMasteryResourceDefinition {
    pub id: String,                    // "fire_mastery"
    pub name: String,                  // "Fire Mastery"
    pub category: ElementResourceCategory::ElementMastery,
    pub element_type: String,          // "fire"
    pub base_formula: String,          // "fire_training_hours * 0.1 + fire_experience * 0.001"
    pub max_formula: String,           // "fire_mastery_cap"
    pub regen_rate: f64,               // 0.0 (no regeneration)
    pub dependencies: Vec<String>,     // ["fire_training_hours", "fire_experience"]
}
```

### **2. Element Stats System**

#### **Tận Dụng Actor Core**
```rust
// Element Stats System implements SystemResourceCalculator
pub struct ElementStatsSystem {
    base: BaseSystemResourceCalculator,
    element_calculator: ElementStatsCalculator,
}

impl SystemResourceCalculator for ElementStatsSystem {
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut resources = HashMap::new();
        
        // Calculate derived stats for each element
        for element_id in self.get_supported_elements() {
            let element_stats = self.element_calculator.calculate_element_stats(actor, element_id).await?;
            
            for (stat_name, stat_value) in element_stats {
                resources.insert(format!("{}_{}", element_id, stat_name), stat_value);
            }
        }
        
        Ok(resources)
    }
    
    fn get_resource_categories(&self) -> Vec<ResourceCategory> {
        vec![ResourceCategory::Elemental, ResourceCategory::Combat]
    }
}
```

#### **Tận Dụng Resource Manager**
```rust
// Element Stats Resources
pub struct ElementStatsResourceDefinition {
    pub id: String,                    // "fire_attack"
    pub name: String,                  // "Fire Attack"
    pub category: ElementResourceCategory::ElementStats,
    pub element_type: String,          // "fire"
    pub stat_type: DerivedStatType::PowerPoint,
    pub base_formula: String,          // "intelligence * 0.8 + strength * 0.4"
    pub max_formula: String,           // "fire_attack_cap"
    pub regen_rate: f64,               // 0.0 (no regeneration)
    pub dependencies: Vec<String>,     // ["intelligence", "strength", "fire_mastery"]
}
```

### **3. Element Effects System**

#### **Tận Dụng Actor Core**
```rust
// Element Effects System implements SystemResourceCalculator
pub struct ElementEffectsSystem {
    base: BaseSystemResourceCalculator,
    effects_calculator: ElementEffectsCalculator,
}

impl SystemResourceCalculator for ElementEffectsSystem {
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut resources = HashMap::new();
        
        // Calculate element effects
        let effects = self.effects_calculator.calculate_effects(actor).await?;
        
        for (effect_id, effect_value) in effects {
            resources.insert(effect_id, effect_value);
        }
        
        Ok(resources)
    }
    
    fn get_resource_categories(&self) -> Vec<ResourceCategory> {
        vec![ResourceCategory::Special, ResourceCategory::Elemental]
    }
}
```

## 📊 **Tổng Kết Tái Sử Dụng**

### **✅ Tận Dụng Hoàn Toàn (100%)**

#### **Actor Core**
- **SystemResourceCalculator Trait**: ✅ Hoàn toàn tái sử dụng
- **BaseSystemResourceCalculator**: ✅ Hoàn toàn tái sử dụng
- **ResourceCategory Enum**: ✅ Hoàn toàn tái sử dụng (cần mở rộng)
- **Resource Dependencies**: ✅ Hoàn toàn tái sử dụng
- **Event System**: ✅ Hoàn toàn tái sử dụng

#### **Resource Manager**
- **String-Based Formula Evaluation**: ✅ Hoàn toàn tái sử dụng
- **Resource Dependencies**: ✅ Hoàn toàn tái sử dụng
- **Resource Categories**: ✅ Hoàn toàn tái sử dụng (cần mở rộng)
- **Resource Management Logic**: ✅ Hoàn toàn tái sử dụng

### **🔄 Cần Mở Rộng (50%)**

#### **Actor Core Extensions**
- **ResourceCategory**: Thêm `Elemental`, `Combat`, `Social`, `Perception`
- **SystemResourceDefinition**: Thêm element-specific fields
- **Resource Dependencies**: Thêm element-specific dependencies

#### **Resource Manager Extensions**
- **ElementResourceCategory**: Thêm element-specific categories
- **ElementResourceDefinition**: Thêm element-specific fields
- **Formula Evaluation**: Thêm element-specific formulas

### **🆕 Cần Implement Mới (0%)**

#### **Element Core Specific**
- **Element Definitions**: Element types, properties, interactions
- **Element Calculations**: Derived stats calculations
- **Element Interactions**: Tương sinh tương khắc
- **Element Effects**: Status effects, buffs, debuffs
- **Element Mastery**: Mastery progression, decay, training

## 🚀 **Implementation Strategy**

### **Phase 1: Extend Actor Core (1-2 weeks)**
1. **Extend ResourceCategory**: Thêm element-specific categories
2. **Extend SystemResourceDefinition**: Thêm element-specific fields
3. **Update Resource Dependencies**: Thêm element-specific dependencies

### **Phase 2: Extend Resource Manager (1-2 weeks)**
1. **Extend Resource Categories**: Thêm element-specific categories
2. **Extend Resource Definitions**: Thêm element-specific fields
3. **Update Formula Evaluation**: Thêm element-specific formulas

### **Phase 3: Implement Element Core (2-3 weeks)**
1. **Elemental Mastery System**: Implement `SystemResourceCalculator`
2. **Element Stats System**: Implement `SystemResourceCalculator`
3. **Element Effects System**: Implement `SystemResourceCalculator`

### **Phase 4: Integration (1 week)**
1. **Actor Core Integration**: Register Element Core systems
2. **Resource Manager Integration**: Register Element Core resources
3. **Testing & Validation**: End-to-end testing

## 💡 **Lợi Ích Tái Sử Dụng**

### **1. Development Speed**
- **Faster Implementation**: Tận dụng existing code
- **Proven Architecture**: Sử dụng tested patterns
- **Consistent API**: Unified interface across systems

### **2. Maintenance**
- **Single Source of Truth**: Centralized resource management
- **Consistent Updates**: Changes propagate automatically
- **Reduced Complexity**: Less code to maintain

### **3. Performance**
- **Optimized Code**: Sử dụng optimized existing code
- **Proven Scalability**: Tested at scale
- **Memory Efficiency**: Shared resource management

### **4. Integration**
- **Seamless Integration**: Natural fit with existing systems
- **Event System**: Automatic event propagation
- **Resource Dependencies**: Automatic dependency resolution

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Analysis Complete  
**Maintainer**: Chaos World Team
