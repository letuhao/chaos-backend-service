# Racial Element Integration Design

## 📋 **Tổng Quan**

Thiết kế này tích hợp racial elements vào Element-Core hiện có thay vì tạo hệ thống mới. Chúng ta sẽ đăng ký thuộc tính racial vào Element-Core và sử dụng hệ thống tương tác đã có sẵn.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Tận Dụng Element-Core Hiện Có**
- **Element Interaction System**: Đã có sẵn trong `element-core/10_Element_Interaction_System_Design.md`
- **Damage Calculation**: Đã có sẵn trong `element-core/02_Multi_System_Integration_Design.md`
- **Probability Engine**: Đã có sẵn với sigmoid functions
- **Status Effects**: Đã có sẵn với dynamics system

### **2. Integration Points**
- **Race Core**: Định nghĩa racial elements và resistances
- **Element Core**: Xử lý element interactions và damage calculations
- **Combat Core**: Sử dụng Element-Core stats cho damage calculation
- **Resource Core**: Apply final damage

## 🏗️ **Racial Element Registration**

### **Core Structure**
```rust
/// Racial element definition for Element-Core integration
/// 种族元素定义 - 集成到Element-Core
pub struct RacialElementDefinition {
    pub race_id: String,                    // Race identifier
    pub primary_elements: Vec<String>,      // Primary elements (主要元素)
    pub element_affinities: HashMap<String, f64>, // Element affinities (元素亲和)
    pub element_resistances: HashMap<String, f64>, // Element resistances (元素抗性)
    pub element_interactions: HashMap<String, ElementInteractionOverride>, // Custom interactions (自定义交互)
}

/// Element interaction override for racial elements
/// 种族元素交互覆盖
pub struct ElementInteractionOverride {
    pub source_element: String,             // Source element (攻击元素)
    pub target_element: String,             // Target element (目标元素)
    pub interaction_type: InteractionType,  // Interaction type (交互类型)
    pub damage_multiplier: f64,             // Damage multiplier (伤害倍数)
    pub resistance_modifier: f64,           // Resistance modifier (抗性修正)
    pub description: String,                // Description (描述)
}

/// Interaction types (reuse from Element-Core)
/// 交互类型 (重用Element-Core)
pub enum InteractionType {
    Same,          // 相同 (Tương Đồng)
    Generating,    // 相生 (Tương Sinh)
    Overcoming,    // 相克 (Tương Khắc)
    Neutral,       // 中性 (Trung Tính)
    Opposite,      // 对立 (Đối Lập)
}
```

### **Element-Core Integration**
```rust
/// Element-Core extension for racial elements
/// Element-Core扩展 - 种族元素
impl ElementCore {
    /// Register racial element definition
    /// 注册种族元素定义
    pub fn register_racial_element(&mut self, racial_def: RacialElementDefinition) -> Result<(), ElementCoreError> {
        // Store racial definition
        self.racial_elements.insert(racial_def.race_id.clone(), racial_def);
        
        // Update element interaction matrix with racial overrides
        self.update_interaction_matrix_with_racial_elements()?;
        
        Ok(())
    }
    
    /// Get racial element resistances
    /// 获取种族元素抗性
    pub fn get_racial_resistance(&self, race_id: &str, element: &str) -> f64 {
        self.racial_elements
            .get(race_id)
            .and_then(|racial_def| racial_def.element_resistances.get(element))
            .copied()
            .unwrap_or(1.0) // Default 100% damage if no resistance defined
    }
    
    /// Get racial element affinities
    /// 获取种族元素亲和
    pub fn get_racial_affinity(&self, race_id: &str, element: &str) -> f64 {
        self.racial_elements
            .get(race_id)
            .and_then(|racial_def| racial_def.element_affinities.get(element))
            .copied()
            .unwrap_or(1.0) // Default 100% affinity if no affinity defined
    }
    
    /// Calculate damage with racial elements
    /// 计算种族元素伤害
    pub fn calculate_damage_with_racial_elements(
        &self,
        attacker: &Actor,
        target: &Actor,
        element: &str,
        base_damage: f64,
    ) -> f64 {
        // Get target's race
        let target_race = target.race.clone();
        
        // Get element interaction (using existing Element-Core system)
        let interaction = self.get_element_interaction(element, &target_race);
        let damage_multiplier = interaction.damage_multiplier;
        
        // Get racial resistance
        let racial_resistance = self.get_racial_resistance(&target_race, element);
        
        // Get racial affinity for attacker
        let attacker_race = attacker.race.clone();
        let racial_affinity = self.get_racial_affinity(&attacker_race, element);
        
        // Calculate final damage
        let final_damage = base_damage * damage_multiplier * racial_resistance * racial_affinity;
        
        // Apply minimum damage (10% of base)
        final_damage.max(base_damage * 0.1)
    }
}
```

## 🔥❄️ **Ví Dụ: Băng Hỏa Long (Ice Fire Dragon)**

### **Racial Element Definition**
```rust
/// Ice Fire Dragon racial element definition
/// 冰火龙族种族元素定义
pub fn create_ice_fire_dragon_definition() -> RacialElementDefinition {
    RacialElementDefinition {
        race_id: "ice_fire_dragon".to_string(),
        primary_elements: vec!["fire".to_string(), "ice".to_string()],
        element_affinities: HashMap::from([
            ("fire".to_string(), 1.3),    // 130% fire affinity
            ("ice".to_string(), 1.3),     // 130% ice affinity
        ]),
        element_resistances: HashMap::from([
            // Fire interactions
            ("fire".to_string(), 0.8),      // 80% fire damage (same element resistance)
            ("ice".to_string(), 0.5),       // 50% ice damage (opposite element weakness)
            ("water".to_string(), 0.7),     // 70% water damage (overcoming weakness)
            ("earth".to_string(), 1.2),     // 120% earth damage (generating resistance)
            ("air".to_string(), 1.0),       // 100% air damage (neutral)
            
            // Ice interactions
            ("fire".to_string(), 0.5),      // 50% fire damage (opposite element weakness)
            ("ice".to_string(), 0.8),       // 80% ice damage (same element resistance)
            ("water".to_string(), 1.2),     // 120% water damage (generating resistance)
            ("earth".to_string(), 0.7),     // 70% earth damage (overcoming weakness)
            ("air".to_string(), 1.0),       // 100% air damage (neutral)
        ]),
        element_interactions: HashMap::from([
            // Custom interactions for Ice Fire Dragon
            ("fire_ice".to_string(), ElementInteractionOverride {
                source_element: "fire".to_string(),
                target_element: "ice".to_string(),
                interaction_type: InteractionType::Opposite,
                damage_multiplier: 2.0,     // 200% damage (opposite elements)
                resistance_modifier: 0.5,   // -50% resistance
                description: "Fire vs Ice - Opposite elements, maximum damage".to_string(),
            }),
            ("ice_fire".to_string(), ElementInteractionOverride {
                source_element: "ice".to_string(),
                target_element: "fire".to_string(),
                interaction_type: InteractionType::Opposite,
                damage_multiplier: 2.0,     // 200% damage (opposite elements)
                resistance_modifier: 0.5,   // -50% resistance
                description: "Ice vs Fire - Opposite elements, maximum damage".to_string(),
            }),
        ]),
    }
}
```

### **Damage Calculation Examples**

#### **Scenario 1: Fire Dragon vs Ice Fire Dragon**
```rust
// Fire Dragon attacks Ice Fire Dragon with Fire element
let attacker_race = "fire_dragon";
let target_race = "ice_fire_dragon";
let element = "fire";
let base_damage = 1000.0;

// Element-Core calculates damage
let final_damage = element_core.calculate_damage_with_racial_elements(
    attacker, target, element, base_damage
);

// Breakdown:
// 1. Element interaction: Fire vs Fire = Same element (0.8x multiplier)
// 2. Racial resistance: Ice Fire Dragon vs Fire = 0.8 (80% damage)
// 3. Racial affinity: Fire Dragon vs Fire = 1.0 (100% affinity)
// 4. Final: 1000 * 0.8 * 0.8 * 1.0 = 640 damage (36% reduction)
```

#### **Scenario 2: Water Mage vs Ice Fire Dragon**
```rust
// Water Mage attacks Ice Fire Dragon with Water element
let attacker_race = "water_spirit";
let target_race = "ice_fire_dragon";
let element = "water";
let base_damage = 1000.0;

// Element-Core calculates damage
let final_damage = element_core.calculate_damage_with_racial_elements(
    attacker, target, element, base_damage
);

// Breakdown:
// 1. Element interaction: Water vs Fire = Overcoming (1.5x multiplier)
// 2. Racial resistance: Ice Fire Dragon vs Water = 0.7 (70% damage)
// 3. Racial affinity: Water Spirit vs Water = 1.4 (140% affinity)
// 4. Final: 1000 * 1.5 * 0.7 * 1.4 = 1470 damage (47% increase)
```

## 🌊🔥 **Ví Dụ: Thủy Hỏa Tinh Linh (Water Fire Spirit)**

### **Racial Element Definition**
```rust
/// Water Fire Spirit racial element definition
/// 水火精灵种族元素定义
pub fn create_water_fire_spirit_definition() -> RacialElementDefinition {
    RacialElementDefinition {
        race_id: "water_fire_spirit".to_string(),
        primary_elements: vec!["water".to_string(), "fire".to_string()],
        element_affinities: HashMap::from([
            ("water".to_string(), 1.4),   // 140% water affinity
            ("fire".to_string(), 1.4),    // 140% fire affinity
        ]),
        element_resistances: HashMap::from([
            // Water interactions
            ("fire".to_string(), 0.5),      // 50% fire damage (opposite element weakness)
            ("water".to_string(), 0.8),     // 80% water damage (same element resistance)
            ("ice".to_string(), 1.2),       // 120% ice damage (generating resistance)
            ("earth".to_string(), 0.7),     // 70% earth damage (overcoming weakness)
            ("air".to_string(), 1.0),       // 100% air damage (neutral)
            
            // Fire interactions
            ("water".to_string(), 0.5),     // 50% water damage (opposite element weakness)
            ("fire".to_string(), 0.8),      // 80% fire damage (same element resistance)
            ("ice".to_string(), 0.7),       // 70% ice damage (overcoming weakness)
            ("earth".to_string(), 1.2),     // 120% earth damage (generating resistance)
            ("air".to_string(), 1.0),       // 100% air damage (neutral)
        ]),
        element_interactions: HashMap::from([
            // Custom interactions for Water Fire Spirit
            ("water_fire".to_string(), ElementInteractionOverride {
                source_element: "water".to_string(),
                target_element: "fire".to_string(),
                interaction_type: InteractionType::Opposite,
                damage_multiplier: 2.0,     // 200% damage (opposite elements)
                resistance_modifier: 0.5,   // -50% resistance
                description: "Water vs Fire - Opposite elements, maximum damage".to_string(),
            }),
            ("fire_water".to_string(), ElementInteractionOverride {
                source_element: "fire".to_string(),
                target_element: "water".to_string(),
                interaction_type: InteractionType::Opposite,
                damage_multiplier: 2.0,     // 200% damage (opposite elements)
                resistance_modifier: 0.5,   // -50% resistance
                description: "Fire vs Water - Opposite elements, maximum damage".to_string(),
            }),
        ]),
    }
}
```

## 🔧 **Integration với Combat System**

### **Combat Core Integration**
```rust
/// Combat Core extension for racial elements
/// Combat Core扩展 - 种族元素
impl CombatCore {
    /// Calculate damage with racial element integration
    /// 计算伤害 - 集成种族元素
    pub fn calculate_damage_with_racial_elements(
        &self,
        action: &Action,
        attacker: &Actor,
        target: &Actor,
    ) -> DamageResult {
        // 1. Get base damage
        let base_damage = action.base_damage;
        let element = &action.element_type;
        
        // 2. Use Element-Core to calculate damage with racial elements
        let element_damage = self.element_core.calculate_damage_with_racial_elements(
            attacker, target, element, base_damage
        );
        
        // 3. Apply other combat modifiers (crit, accuracy, etc.)
        let combat_damage = self.apply_combat_modifiers(element_damage, attacker, target);
        
        // 4. Create damage result
        DamageResult {
            base_damage,
            element_damage,
            final_damage: combat_damage,
            element_type: element.clone(),
            racial_modifiers: self.get_racial_modifiers(attacker, target, element),
        }
    }
    
    /// Get racial modifiers for display
    /// 获取种族修正器用于显示
    fn get_racial_modifiers(&self, attacker: &Actor, target: &Actor, element: &str) -> RacialModifiers {
        let attacker_race = attacker.race.clone();
        let target_race = target.race.clone();
        
        RacialModifiers {
            attacker_affinity: self.element_core.get_racial_affinity(&attacker_race, element),
            target_resistance: self.element_core.get_racial_resistance(&target_race, element),
            interaction_type: self.element_core.get_element_interaction(element, &target_race).interaction_type,
        }
    }
}
```

### **Resource Core Integration**
```rust
/// Resource Core extension for applying racial damage
/// Resource Core扩展 - 应用种族伤害
impl ResourceCore {
    /// Apply damage with racial element considerations
    /// 应用伤害 - 考虑种族元素
    pub fn apply_damage_with_racial_elements(
        &self,
        target: &mut Actor,
        damage_result: &DamageResult,
    ) -> Result<(), ResourceCoreError> {
        // Apply final damage to target's HP
        let current_hp = target.get_hp();
        let new_hp = (current_hp - damage_result.final_damage).max(0.0);
        target.set_hp(new_hp);
        
        // Log racial modifiers for debugging/display
        if let Some(racial_modifiers) = &damage_result.racial_modifiers {
            self.log_racial_damage_modifiers(racial_modifiers, damage_result);
        }
        
        Ok(())
    }
    
    /// Log racial damage modifiers
    /// 记录种族伤害修正器
    fn log_racial_damage_modifiers(&self, modifiers: &RacialModifiers, damage_result: &DamageResult) {
        println!(
            "Racial Damage Modifiers: Affinity={:.2}x, Resistance={:.2}x, Interaction={:?}",
            modifiers.attacker_affinity,
            modifiers.target_resistance,
            modifiers.interaction_type
        );
    }
}
```

## 🎯 **Configuration Integration**

### **Element-Core Configuration Extension**
```yaml
# element-core/configs/racial_elements.yaml
racial_elements:
  ice_fire_dragon:
    race_id: "ice_fire_dragon"
    primary_elements: ["fire", "ice"]
    element_affinities:
      fire: 1.3
      ice: 1.3
    element_resistances:
      fire: 0.8
      ice: 0.5
      water: 0.7
      earth: 1.2
      air: 1.0
    element_interactions:
      fire_ice:
        source_element: "fire"
        target_element: "ice"
        interaction_type: "opposite"
        damage_multiplier: 2.0
        resistance_modifier: 0.5
  
  water_fire_spirit:
    race_id: "water_fire_spirit"
    primary_elements: ["water", "fire"]
    element_affinities:
      water: 1.4
      fire: 1.4
    element_resistances:
      fire: 0.5
      water: 0.8
      ice: 1.2
      earth: 0.7
      air: 1.0
    element_interactions:
      water_fire:
        source_element: "water"
        target_element: "fire"
        interaction_type: "opposite"
        damage_multiplier: 2.0
        resistance_modifier: 0.5
```

## 🚀 **Implementation Strategy**

### **Phase 1: Core Integration**
1. **Extend Element-Core** với racial element support
2. **Add racial element registration** system
3. **Update damage calculation** để include racial modifiers
4. **Test với simple examples** (Fire Dragon vs Ice Fire Dragon)

### **Phase 2: Combat Integration**
1. **Update Combat Core** để use Element-Core racial calculations
2. **Add racial modifiers** to damage results
3. **Update Resource Core** để apply racial damage
4. **Add logging** cho racial modifiers

### **Phase 3: Advanced Features**
1. **Add complex multi-element interactions**
2. **Implement racial element abilities**
3. **Add UI display** cho racial modifiers
4. **Balance testing** và fine-tuning

## 📈 **Benefits của Approach Này**

### **1. Tận Dụng Existing Systems**
- **Element-Core**: Đã có sẵn interaction system
- **Probability Engine**: Đã có sẵn sigmoid functions
- **Status Effects**: Đã có sẵn dynamics system
- **Damage Calculation**: Đã có sẵn combat integration

### **2. No System Conflicts**
- **Single Source of Truth**: Element-Core handles all element interactions
- **Consistent Logic**: Same interaction logic cho all elements
- **Maintainable**: Chỉ cần maintain một system

### **3. Easy Integration**
- **Race Core**: Chỉ cần define racial elements
- **Combat Core**: Sử dụng Element-Core calculations
- **Resource Core**: Apply final damage
- **UI**: Display racial modifiers

### **4. Extensible**
- **New Races**: Easy to add new racial elements
- **New Interactions**: Easy to add custom interactions
- **New Elements**: Easy to add new elements
- **Balance Changes**: Easy to adjust values

## 🎯 **Next Steps**

1. **Review và feedback** trên design này
2. **Extend Element-Core** với racial element support
3. **Create racial element definitions** cho các chủng tộc
4. **Update Combat Core** để use Element-Core calculations
5. **Test integration** và balance

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
