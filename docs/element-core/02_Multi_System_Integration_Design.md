# Multi-System Integration Design

## 📋 **Tổng Quan**

Tài liệu này mô tả chi tiết cách Element Core tích hợp với các hệ thống khác trong game, bao gồm Combat Core, Shield System, Race Talents, Item Attributes, và các custom systems.

**Hybrid Integration Approach**: Element Core sử dụng hybrid approach để tích hợp với các systems khác, trong đó Element Core cung cấp element stats (bao gồm Omni stats) và các systems khác sử dụng những stats này để thực hiện calculations của riêng mình.

**Condition Core Integration**: Element Core tích hợp với Condition Core để cung cấp standardized condition functions cho tất cả systems, đảm bảo condition logic được quản lý tập trung và có thể tái sử dụng across systems.

## 🎯 **Nguyên Tắc Tích Hợp**

### **1. Loose Coupling**
- Element Core không phụ thuộc trực tiếp vào các systems khác
- Sử dụng interface-based integration
- Dễ dàng thêm/sửa/xóa systems mà không ảnh hưởng Element Core

### **2. Consistent Interface**
- Tất cả systems sử dụng cùng một interface
- Standardized data structures
- Unified configuration system

### **3. Performance First**
- Minimize overhead khi integrate
- Efficient data passing
- Caching và optimization

> Consistency: Các công thức xác suất/steepness/scaling được định nghĩa ở `01_Probability_Mechanics_Design.md`. Tài liệu này không lặp công thức; chỉ sử dụng interfaces/adapters và liên kết tới Probability Engine.

## 🏗️ **Integration Architecture**

```
Element Core Integration Layer
├── System Interface
│   ├── ElementSystemInterface
│   ├── ElementDataProvider
│   └── ElementEventHandler
├── System Adapters
│   ├── CombatCoreAdapter
│   ├── ShieldSystemAdapter
│   ├── RaceTalentAdapter
│   ├── ItemAttributeAdapter
│   └── CustomSystemAdapter
├── Data Aggregation
│   ├── MultiSystemAggregator
│   ├── ConflictResolver
│   └── PriorityManager
└── Event System
    ├── ElementEventDispatcher
    ├── ElementEventSubscriber
    └── ElementEventProcessor
```

## 🎯 **Condition Core Integration**

### **1. Element Data Provider**

Element Core implements `ElementDataProvider` trait để cung cấp data cho Condition Core:

```rust
// Element Core as Condition Core data provider
impl ElementDataProvider for ElementCore {
    async fn get_element_mastery(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64> {
        // Get element mastery from Element Core
        self.get_actor_element_mastery(actor_id, element_id).await
    }
    
    async fn has_element_affinity(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool> {
        // Check element affinity from Element Core
        self.check_actor_element_affinity(actor_id, element_id).await
    }
    
    async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String> {
        // Get element interaction from Element Core
        self.get_element_interaction_type(source_element, target_element).await
    }
    
    // ... implement other ElementDataProvider methods
}
```

### **2. Standardized Element Conditions**

Tất cả element conditions được chuẩn hóa thông qua Condition Core:

```yaml
# Element mastery condition
element_mastery_condition:
  condition_id: "has_fire_mastery"
  function_name: "get_element_mastery"
  operator: "GreaterThanOrEqual"
  value:
    value_type: "float"
    value: 100.0
  parameters:
    - parameter_type: "string"
      parameter_value: "fire"

# Element affinity condition
element_affinity_condition:
  condition_id: "has_water_affinity"
  function_name: "has_element_affinity"
  operator: "Equal"
  value:
    value_type: "boolean"
    value: true
  parameters:
    - parameter_type: "string"
      parameter_value: "water"
```

### **3. Cross-System Condition Reuse**

Các systems khác có thể sử dụng element conditions thông qua Condition Core:

```rust
// Combat Core using Element Core conditions
impl CombatCore {
    pub async fn can_cast_fire_spell(&self, actor_id: &str) -> Result<bool, CombatError> {
        let condition = ConditionConfig {
            condition_id: "can_cast_fire_spell".to_string(),
            function_name: "get_element_mastery".to_string(),
            operator: ConditionOperator::GreaterThanOrEqual,
            value: ConditionValue::Float(100.0),
            parameters: vec![ConditionParameter::String("fire".to_string())],
        };
        
        let context = self.create_condition_context(actor_id).await?;
        self.condition_resolver.resolve_condition(&condition, &context).await
    }
}
```

## 🔌 **System Interface**

### **1. Element System Interface**

```rust
// Core interface for all systems that use elements
pub trait ElementSystemInterface {
    // Get system identifier
    fn get_system_id(&self) -> &str;
    
    // Get supported element types
    fn get_supported_elements(&self) -> Vec<String>;
    
    // Get element stats for an actor
    fn get_actor_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
    ) -> Result<HashMap<DerivedStatType, f64>, ElementError>;
    
    // Update element stats for an actor
    fn update_actor_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
        stats: HashMap<DerivedStatType, f64>,
    ) -> Result<(), ElementError>;
    
    // Handle element events
    fn handle_element_event(
        &self,
        event: &ElementEvent,
    ) -> Result<(), ElementError>;
    
    // Get system priority (higher = more important)
    fn get_system_priority(&self) -> i32;
    
    // Check if system is active
    fn is_system_active(&self) -> bool;
}
```

### **2. Element Data Provider**

```rust
// Data provider interface for element stats
pub trait ElementDataProvider {
    // Get base element stats
    fn get_base_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
    ) -> Result<HashMap<DerivedStatType, f64>, ElementError>;
    
    // Get element modifiers
    fn get_element_modifiers(
        &self,
        actor_id: &str,
        element_type: &str,
    ) -> Result<Vec<ElementModifier>, ElementError>;
    
    // Get element bonuses
    fn get_element_bonuses(
        &self,
        actor_id: &str,
        element_type: &str,
    ) -> Result<Vec<ElementBonus>, ElementError>;
    
    // Get element penalties
    fn get_element_penalties(
        &self,
        actor_id: &str,
        element_type: &str,
    ) -> Result<Vec<ElementPenalty>, ElementError>;
}

// Element modifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementModifier {
    pub id: String,
    pub stat_type: DerivedStatType,
    pub value: f64,
    pub multiplier: f64,
    pub duration: Option<Duration>,
    pub source: String,
    pub is_active: bool,
}

// Element bonus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementBonus {
    pub id: String,
    pub stat_type: DerivedStatType,
    pub value: f64,
    pub condition: Option<ElementCondition>,
    pub source: String,
    pub is_active: bool,
}

// Element penalty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementPenalty {
    pub id: String,
    pub stat_type: DerivedStatType,
    pub value: f64,
    pub condition: Option<ElementCondition>,
    pub source: String,
    pub is_active: bool,
}
```

### **3. Element Event System**

```rust
// Element event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementEventType {
    StatChanged,
    ElementActivated,
    ElementDeactivated,
    ElementModified,
    ElementBonusApplied,
    ElementPenaltyApplied,
    ElementConflict,
    ElementResolved,
}

// Element event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementEvent {
    pub id: String,
    pub event_type: ElementEventType,
    pub actor_id: String,
    pub element_type: String,
    pub system_id: String,
    pub data: HashMap<String, serde_json::Value>,
    pub timestamp: i64,
}

// Element event handler
pub trait ElementEventHandler {
    fn handle_event(&self, event: &ElementEvent) -> Result<(), ElementError>;
    fn can_handle_event(&self, event_type: &ElementEventType) -> bool;
}
```

## ⚔️ **Combat Core Integration (Hybrid Approach)**

### **1. Element Stats Provider**

```rust
// Element-Core provides stats for Combat-Core
pub struct ElementStatsProvider {
    element_core: Arc<ElementCore>,
}

impl ElementStatsProvider {
    pub fn get_combat_stats(&self, attacker: &Actor, target: &Actor, element_type: &str) -> CombatElementStats {
        // Get Omni stats (baseline protection)
        let attacker_omni = self.element_core.get_omni_stats(attacker);
        let target_omni = self.element_core.get_omni_stats(target);
        
        // Get element-specific stats
        let attacker_element = self.element_core.get_element_stats(attacker, element_type);
        let target_element = self.element_core.get_element_stats(target, element_type);
        
        // Combine Omni + Element stats
        CombatElementStats {
            // Power calculation: Omni + Element
            attacker_power: attacker_omni.power + attacker_element.power,
            target_defense: target_omni.defense + target_element.defense,
            
            // Critical calculation: Omni + Element
            attacker_crit_rate: attacker_omni.crit_rate + attacker_element.crit_rate,
            attacker_crit_damage: attacker_omni.crit_damage + attacker_element.crit_damage,
            target_resist_crit: target_omni.resist_crit + target_element.resist_crit,
            target_resist_crit_damage: target_omni.resist_crit_damage + target_element.resist_crit_damage,
            
            // Accuracy calculation: Omni + Element
            attacker_accuracy: attacker_omni.accuracy + attacker_element.accuracy,
            target_dodge: target_omni.dodge + target_element.dodge,
            
            // Status effect calculation: Omni + Element
            attacker_status_prob: attacker_omni.status_prob + attacker_element.status_prob,
            target_status_resist: target_omni.status_resist + target_element.status_resist,
            
            // Element interactions
            damage_multiplier: self.element_core.get_damage_multiplier(element_type, target.get_primary_element()),
        }
    }
}
```

### **2. Combat Core Integration**

```rust
// Combat-Core uses Element-Core stats
impl CombatCore {
    pub fn calculate_damage(&self, action: &Action, attacker: &Actor, target: &Actor) -> DamageResult {
        // 1. Get element stats from Element-Core
        let element_stats = self.element_stats_provider.get_combat_stats(
            attacker, 
            target, 
            action.element_type
        );
        
        // 2. Create damage input with element stats
        let damage_input = DamageInput {
            base_damage: action.base_damage,
            power_points: vec![element_stats.attacker_power],
            target_defense: element_stats.target_defense,
            critical_chance: element_stats.attacker_crit_rate,
            critical_multiplier: element_stats.attacker_crit_damage,
            accuracy: element_stats.attacker_accuracy,
            target_dodge: element_stats.target_dodge,
            element_multiplier: element_stats.damage_multiplier,
            // ... other fields
        };
        
        // 3. Calculate damage using existing formula
        let mut damage_result = self.damage_calculator.calculate_final_damage(damage_input, target);
        
        // 4. Apply element-specific effects
        damage_result.final_damage *= element_stats.damage_multiplier;
        
        // 5. Apply status effects if applicable
        if self.should_apply_status_effects(action, element_stats) {
            let status_effects = self.element_core.calculate_status_effects(
                attacker, target, action.element_type
            );
            damage_result.status_effects = status_effects;
        }
        
        damage_result
    }
}
```

### **3. Integration Benefits**

#### **Separation of Concerns**
- **Element-Core**: Quản lý element stats, interactions, status effects
- **Combat-Core**: Quản lý combat mechanics, action processing, event handling

#### **Performance**
- **Element-Core Caching**: Cache element calculations
- **Combat-Core Caching**: Cache combat calculations
- **Minimal Overhead**: Chỉ pass data, không duplicate calculations

#### **Flexibility**
- **Element-Core**: Có thể được sử dụng bởi systems khác (Shield, Item, Race)
- **Combat-Core**: Có thể sử dụng element stats từ nhiều sources
- **Easy Testing**: Có thể test từng component riêng biệt

## 🛡️ **Shield System Integration**
```

### **2. Combat Element Calculator**

```rust
// Combat element calculator
pub struct CombatElementCalculator {
    element_registry: Arc<ElementRegistry>,
    probability_engine: Arc<ProbabilityEngine>,
    combat_adapter: Arc<CombatCoreAdapter>,
}

impl CombatElementCalculator {
    // Calculate damage using element system including Omni stats
    pub fn calculate_element_damage(
        &self,
        attacker_id: &str,
        target_id: &str,
        element_type: &str,
        base_damage: f64,
    ) -> Result<ElementDamageResult, ElementError> {
        // Get Omni stats for both actors
        let attacker_omni_stats = self.combat_adapter.get_actor_omni_stats(attacker_id)?;
        let target_omni_stats = self.combat_adapter.get_actor_omni_stats(target_id)?;
        
        // Get element stats for both actors
        let attacker_element_stats = self.combat_adapter.get_actor_element_stats(attacker_id, element_type)?;
        let target_element_stats = self.combat_adapter.get_actor_element_stats(target_id, element_type)?;
        
        // Calculate total power vs defense (Omni + Element)
        let attacker_total_power = attacker_omni_stats.get(&DerivedStatType::PowerPoint).copied().unwrap_or(0.0) +
                                 attacker_element_stats.get(&DerivedStatType::PowerPoint).copied().unwrap_or(0.0);
        let target_total_defense = target_omni_stats.get(&DerivedStatType::DefensePoint).copied().unwrap_or(0.0) +
                                 target_element_stats.get(&DerivedStatType::DefensePoint).copied().unwrap_or(0.0);
        
        // Calculate final damage
        let power_damage = attacker_total_power - target_total_defense;
        let final_damage = base_damage + power_damage.max(0.0);
        
        // Check critical hit using Omni + Element stats
        let attacker_total_crit_rate = attacker_omni_stats.get(&DerivedStatType::CritRate).copied().unwrap_or(0.0) +
                                     attacker_element_stats.get(&DerivedStatType::CritRate).copied().unwrap_or(0.0);
        let target_total_resist_crit = target_omni_stats.get(&DerivedStatType::ResistCritRate).copied().unwrap_or(0.0) +
                                     target_element_stats.get(&DerivedStatType::ResistCritRate).copied().unwrap_or(0.0);
        
        let crit_difference = attacker_total_crit_rate - target_total_resist_crit;
        let crit_probability = self.probability_engine.calculate_crit_probability(
            attacker_omni_stats.get(&DerivedStatType::CritRate).copied().unwrap_or(0.0),
            attacker_element_stats.get(&DerivedStatType::CritRate).copied().unwrap_or(0.0),
            target_omni_stats.get(&DerivedStatType::ResistCritRate).copied().unwrap_or(0.0),
            target_element_stats.get(&DerivedStatType::ResistCritRate).copied().unwrap_or(0.0),
            element_type,
        );
        
        let is_critical = self.probability_engine.check_probability(crit_probability);
        
        // Calculate critical damage if applicable
        let final_damage = if is_critical {
            let attacker_total_crit_damage = attacker_omni_stats.get(&DerivedStatType::CritDamage).copied().unwrap_or(0.0) +
                                           attacker_element_stats.get(&DerivedStatType::CritDamage).copied().unwrap_or(0.0);
            let target_total_resist_crit_damage = target_omni_stats.get(&DerivedStatType::ResistCritDamage).copied().unwrap_or(0.0) +
                                                target_element_stats.get(&DerivedStatType::ResistCritDamage).copied().unwrap_or(0.0);
            
            let crit_damage_multiplier = 1.0 + (attacker_total_crit_damage - target_total_resist_crit_damage).max(0.0);
            final_damage * crit_damage_multiplier
        } else {
            final_damage
        };
        
        Ok(ElementDamageResult {
            final_damage,
            base_damage,
            power_damage,
            defense_reduction: defense_result.final_value,
            is_critical,
            critical_multiplier: if is_critical { 1.0 + crit_result.final_value } else { 1.0 },
            element_type: element_type.to_string(),
        })
    }
    
    // Check hit accuracy
    pub fn check_hit_accuracy(
        &self,
        attacker_id: &str,
        target_id: &str,
        element_type: &str,
    ) -> Result<bool, ElementError> {
        let attacker_stats = self.combat_adapter.get_actor_element_stats(attacker_id, element_type)?;
        let target_stats = self.combat_adapter.get_actor_element_stats(target_id, element_type)?;
        
        let accuracy_result = self.probability_engine.compare_stats(
            &attacker_stats,
            &target_stats,
            &DerivedStatType::AccurateRate,
        );
        
        Ok(self.probability_engine.check_probability(accuracy_result.final_value))
    }
}
```

## 🛡️ **Shield System Integration**

### **1. Shield System Adapter**

```rust
// Shield system adapter for Element Core
pub struct ShieldSystemAdapter {
    shield_system: Arc<ShieldSystem>,
    element_registry: Arc<ElementRegistry>,
    event_dispatcher: Arc<ElementEventDispatcher>,
}

impl ElementSystemInterface for ShieldSystemAdapter {
    fn get_system_id(&self) -> &str {
        "shield_system"
    }
    
    fn get_supported_elements(&self) -> Vec<String> {
        vec![
            "fire".to_string(),
            "water".to_string(),
            "ice".to_string(),
            "lightning".to_string(),
            "earth".to_string(),
            "physical".to_string(),
            "magical".to_string(),
        ]
    }
    
    fn get_actor_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
    ) -> Result<HashMap<DerivedStatType, f64>, ElementError> {
        // Get actor's shields
        let shields = self.shield_system.get_actor_shields(actor_id)?;
        
        let mut stats = HashMap::new();
        
        // Calculate total defense for element
        let mut total_defense = 0.0;
        let mut total_absorption = 0.0;
        let mut total_reflection = 0.0;
        
        for shield in shields {
            if shield.element_type == element_type {
                total_defense += shield.defense_value;
                total_absorption += shield.absorption_value;
                total_reflection += shield.reflection_value;
            }
        }
        
        stats.insert(DerivedStatType::DefensePoint, total_defense);
        stats.insert(DerivedStatType::Absorption, total_absorption);
        stats.insert(DerivedStatType::Reflection, total_reflection);
        
        Ok(stats)
    }
    
    fn update_actor_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
        stats: HashMap<DerivedStatType, f64>,
    ) -> Result<(), ElementError> {
        // Update shield stats
        if let Some(defense) = stats.get(&DerivedStatType::DefensePoint) {
            self.shield_system.update_shield_defense(actor_id, element_type, *defense)?;
        }
        
        if let Some(absorption) = stats.get(&DerivedStatType::Absorption) {
            self.shield_system.update_shield_absorption(actor_id, element_type, *absorption)?;
        }
        
        if let Some(reflection) = stats.get(&DerivedStatType::Reflection) {
            self.shield_system.update_shield_reflection(actor_id, element_type, *reflection)?;
        }
        
        // Dispatch event
        let event = ElementEvent {
            id: generate_id(),
            event_type: ElementEventType::StatChanged,
            actor_id: actor_id.to_string(),
            element_type: element_type.to_string(),
            system_id: self.get_system_id().to_string(),
            data: HashMap::new(),
            timestamp: current_timestamp(),
        };
        
        self.event_dispatcher.dispatch_event(&event)?;
        
        Ok(())
    }
    
    fn handle_element_event(&self, event: &ElementEvent) -> Result<(), ElementError> {
        match event.event_type {
            ElementEventType::StatChanged => {
                // Update shield calculations
                self.shield_system.invalidate_actor_cache(&event.actor_id)?;
            }
            ElementEventType::ElementActivated => {
                // Activate element shields
                self.shield_system.activate_element_shields(&event.actor_id, &event.element_type)?;
            }
            ElementEventType::ElementDeactivated => {
                // Deactivate element shields
                self.shield_system.deactivate_element_shields(&event.actor_id, &event.element_type)?;
            }
            _ => {
                // Handle other event types
            }
        }
        
        Ok(())
    }
    
    fn get_system_priority(&self) -> i32 {
        80 // Medium-high priority for shields
    }
    
    fn is_system_active(&self) -> bool {
        self.shield_system.is_active()
    }
}
```

### **2. Shield Element Calculator**

```rust
// Shield element calculator
pub struct ShieldElementCalculator {
    element_registry: Arc<ElementRegistry>,
    probability_engine: Arc<ProbabilityEngine>,
    shield_adapter: Arc<ShieldSystemAdapter>,
}

impl ShieldElementCalculator {
    // Calculate shield absorption
    pub fn calculate_shield_absorption(
        &self,
        shield_id: &str,
        incoming_damage: f64,
        element_type: &str,
    ) -> Result<ShieldAbsorptionResult, ElementError> {
        // Get shield element stats
        let shield_stats = self.shield_adapter.get_actor_element_stats(shield_id, element_type)?;
        
        // Calculate absorption
        let absorption = shield_stats.get(&DerivedStatType::Absorption).copied().unwrap_or(0.0);
        let absorbed_damage = (incoming_damage * absorption).min(incoming_damage);
        let remaining_damage = incoming_damage - absorbed_damage;
        
        Ok(ShieldAbsorptionResult {
            total_damage: incoming_damage,
            absorbed_damage,
            remaining_damage,
            absorption_rate: absorption,
            element_type: element_type.to_string(),
        })
    }
    
    // Calculate shield reflection
    pub fn calculate_shield_reflection(
        &self,
        shield_id: &str,
        incoming_damage: f64,
        element_type: &str,
    ) -> Result<ShieldReflectionResult, ElementError> {
        // Get shield element stats
        let shield_stats = self.shield_adapter.get_actor_element_stats(shield_id, element_type)?;
        
        // Calculate reflection
        let reflection = shield_stats.get(&DerivedStatType::Reflection).copied().unwrap_or(0.0);
        let reflected_damage = incoming_damage * reflection;
        
        Ok(ShieldReflectionResult {
            total_damage: incoming_damage,
            reflected_damage,
            reflection_rate: reflection,
            element_type: element_type.to_string(),
        })
    }
}
```

## 🏛️ **Race Talent Integration**

### **1. Race Talent Adapter**

```rust
// Race talent adapter for Element Core
pub struct RaceTalentAdapter {
    race_system: Arc<RaceSystem>,
    element_registry: Arc<ElementRegistry>,
    event_dispatcher: Arc<ElementEventDispatcher>,
}

impl ElementSystemInterface for RaceTalentAdapter {
    fn get_system_id(&self) -> &str {
        "race_talent_system"
    }
    
    fn get_supported_elements(&self) -> Vec<String> {
        // Get all elements from race talents
        self.race_system.get_all_talent_elements()
    }
    
    fn get_actor_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
    ) -> Result<HashMap<DerivedStatType, f64>, ElementError> {
        // Get actor's race
        let actor = self.race_system.get_actor(actor_id)?;
        let race = self.race_system.get_race(&actor.race_id)?;
        
        // Get race talents for element
        let talents = race.get_talents_for_element(element_type)?;
        
        let mut stats = HashMap::new();
        
        // Calculate stats from talents
        for talent in talents {
            if talent.is_active {
                for (stat_type, value) in &talent.element_stats {
                    let current_value = stats.get(stat_type).copied().unwrap_or(0.0);
                    stats.insert(stat_type.clone(), current_value + value);
                }
            }
        }
        
        Ok(stats)
    }
    
    fn update_actor_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
        stats: HashMap<DerivedStatType, f64>,
    ) -> Result<(), ElementError> {
        // Update race talent stats
        self.race_system.update_actor_talent_stats(actor_id, element_type, stats)?;
        
        // Dispatch event
        let event = ElementEvent {
            id: generate_id(),
            event_type: ElementEventType::StatChanged,
            actor_id: actor_id.to_string(),
            element_type: element_type.to_string(),
            system_id: self.get_system_id().to_string(),
            data: HashMap::new(),
            timestamp: current_timestamp(),
        };
        
        self.event_dispatcher.dispatch_event(&event)?;
        
        Ok(())
    }
    
    fn handle_element_event(&self, event: &ElementEvent) -> Result<(), ElementError> {
        match event.event_type {
            ElementEventType::ElementActivated => {
                // Activate race talents for element
                self.race_system.activate_element_talents(&event.actor_id, &event.element_type)?;
            }
            ElementEventType::ElementDeactivated => {
                // Deactivate race talents for element
                self.race_system.deactivate_element_talents(&event.actor_id, &event.element_type)?;
            }
            _ => {
                // Handle other event types
            }
        }
        
        Ok(())
    }
    
    fn get_system_priority(&self) -> i32 {
        60 // Medium priority for race talents
    }
    
    fn is_system_active(&self) -> bool {
        self.race_system.is_active()
    }
}
```

## 🎒 **Item Attribute Integration**

### **1. Item Attribute Adapter**

```rust
// Item attribute adapter for Element Core
pub struct ItemAttributeAdapter {
    item_system: Arc<ItemSystem>,
    element_registry: Arc<ElementRegistry>,
    event_dispatcher: Arc<ElementEventDispatcher>,
}

impl ElementSystemInterface for ItemAttributeAdapter {
    fn get_system_id(&self) -> &str {
        "item_attribute_system"
    }
    
    fn get_supported_elements(&self) -> Vec<String> {
        // Get all elements from item attributes
        self.item_system.get_all_attribute_elements()
    }
    
    fn get_actor_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
    ) -> Result<HashMap<DerivedStatType, f64>, ElementError> {
        // Get actor's equipped items
        let equipped_items = self.item_system.get_actor_equipped_items(actor_id)?;
        
        let mut stats = HashMap::new();
        
        // Calculate stats from item attributes
        for item in equipped_items {
            let attributes = item.get_element_attributes(element_type)?;
            
            for (stat_type, value) in attributes {
                let current_value = stats.get(&stat_type).copied().unwrap_or(0.0);
                stats.insert(stat_type, current_value + value);
            }
        }
        
        Ok(stats)
    }
    
    fn update_actor_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
        stats: HashMap<DerivedStatType, f64>,
    ) -> Result<(), ElementError> {
        // Update item attribute stats
        self.item_system.update_actor_item_stats(actor_id, element_type, stats)?;
        
        // Dispatch event
        let event = ElementEvent {
            id: generate_id(),
            event_type: ElementEventType::StatChanged,
            actor_id: actor_id.to_string(),
            element_type: element_type.to_string(),
            system_id: self.get_system_id().to_string(),
            data: HashMap::new(),
            timestamp: current_timestamp(),
        };
        
        self.event_dispatcher.dispatch_event(&event)?;
        
        Ok(())
    }
    
    fn handle_element_event(&self, event: &ElementEvent) -> Result<(), ElementError> {
        match event.event_type {
            ElementEventType::ElementActivated => {
                // Activate item attributes for element
                self.item_system.activate_element_attributes(&event.actor_id, &event.element_type)?;
            }
            ElementEventType::ElementDeactivated => {
                // Deactivate item attributes for element
                self.item_system.deactivate_element_attributes(&event.actor_id, &event.element_type)?;
            }
            _ => {
                // Handle other event types
            }
        }
        
        Ok(())
    }
    
    fn get_system_priority(&self) -> i32 {
        40 // Medium priority for item attributes
    }
    
    fn is_system_active(&self) -> bool {
        self.item_system.is_active()
    }
}
```

## 🔄 **Multi-System Aggregation**

### **1. Multi-System Aggregator**

```rust
// Multi-system aggregator
pub struct MultiSystemAggregator {
    systems: HashMap<String, Arc<dyn ElementSystemInterface>>,
    element_registry: Arc<ElementRegistry>,
    conflict_resolver: Arc<ConflictResolver>,
    priority_manager: Arc<PriorityManager>,
}

impl MultiSystemAggregator {
    // Register a system
    pub fn register_system(
        &mut self,
        system: Arc<dyn ElementSystemInterface>,
    ) -> Result<(), ElementError> {
        let system_id = system.get_system_id().to_string();
        
        if self.systems.contains_key(&system_id) {
            return Err(ElementError::SystemAlreadyRegistered(system_id));
        }
        
        self.systems.insert(system_id, system);
        Ok(())
    }
    
    // Get aggregated element stats for an actor
    pub fn get_aggregated_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
    ) -> Result<HashMap<DerivedStatType, f64>, ElementError> {
        let mut aggregated_stats = HashMap::new();
        let mut system_stats = HashMap::new();
        
        // Collect stats from all systems
        for (system_id, system) in &self.systems {
            if system.is_system_active() && system.get_supported_elements().contains(&element_type.to_string()) {
                match system.get_actor_element_stats(actor_id, element_type) {
                    Ok(stats) => {
                        system_stats.insert(system_id.clone(), stats);
                    }
                    Err(e) => {
                        // Log error but continue with other systems
                        eprintln!("Error getting stats from system {}: {:?}", system_id, e);
                    }
                }
            }
        }
        
        // Aggregate stats from all systems
        for (system_id, stats) in system_stats {
            for (stat_type, value) in stats {
                let current_value = aggregated_stats.get(&stat_type).copied().unwrap_or(0.0);
                aggregated_stats.insert(stat_type, current_value + value);
            }
        }
        
        // Resolve conflicts
        let resolved_stats = self.conflict_resolver.resolve_conflicts(
            &aggregated_stats,
            &system_stats,
        )?;
        
        Ok(resolved_stats)
    }
    
    // Update element stats across all systems
    pub fn update_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
        stats: HashMap<DerivedStatType, f64>,
    ) -> Result<(), ElementError> {
        // Update all systems
        for (system_id, system) in &self.systems {
            if system.is_system_active() && system.get_supported_elements().contains(&element_type.to_string()) {
                if let Err(e) = system.update_actor_element_stats(actor_id, element_type, stats.clone()) {
                    eprintln!("Error updating stats in system {}: {:?}", system_id, e);
                }
            }
        }
        
        Ok(())
    }
}
```

### **2. Conflict Resolver**

```rust
// Conflict resolver for handling stat conflicts
pub struct ConflictResolver {
    resolution_rules: HashMap<DerivedStatType, ConflictResolutionRule>,
}

impl ConflictResolver {
    // Resolve conflicts between systems
    pub fn resolve_conflicts(
        &self,
        aggregated_stats: &HashMap<DerivedStatType, f64>,
        system_stats: &HashMap<String, HashMap<DerivedStatType, f64>>,
    ) -> Result<HashMap<DerivedStatType, f64>, ElementError> {
        let mut resolved_stats = aggregated_stats.clone();
        
        // Apply resolution rules for each stat type
        for (stat_type, value) in aggregated_stats {
            if let Some(rule) = self.resolution_rules.get(stat_type) {
                let resolved_value = match rule {
                    ConflictResolutionRule::Sum => *value,
                    ConflictResolutionRule::Max => {
                        system_stats.values()
                            .map(|stats| stats.get(stat_type).copied().unwrap_or(0.0))
                            .fold(0.0, f64::max)
                    }
                    ConflictResolutionRule::Min => {
                        system_stats.values()
                            .map(|stats| stats.get(stat_type).copied().unwrap_or(0.0))
                            .fold(f64::INFINITY, f64::min)
                    }
                    ConflictResolutionRule::Average => {
                        let count = system_stats.values()
                            .filter(|stats| stats.contains_key(stat_type))
                            .count();
                        if count > 0 { *value / count as f64 } else { *value }
                    }
                    ConflictResolutionRule::Priority(priority_systems) => {
                        self.resolve_by_priority(stat_type, system_stats, priority_systems)?
                    }
                };
                
                resolved_stats.insert(stat_type.clone(), resolved_value);
            }
        }
        
        Ok(resolved_stats)
    }
    
    // Resolve by system priority
    fn resolve_by_priority(
        &self,
        stat_type: &DerivedStatType,
        system_stats: &HashMap<String, HashMap<DerivedStatType, f64>>,
        priority_systems: &[String],
    ) -> Result<f64, ElementError> {
        for system_id in priority_systems {
            if let Some(stats) = system_stats.get(system_id) {
                if let Some(value) = stats.get(stat_type) {
                    return Ok(*value);
                }
            }
        }
        
        // Fallback to sum if no priority system has the stat
        Ok(system_stats.values()
            .map(|stats| stats.get(stat_type).copied().unwrap_or(0.0))
            .sum())
    }
}

// Conflict resolution rules
#[derive(Debug, Clone)]
pub enum ConflictResolutionRule {
    Sum,                    // Sum all values
    Max,                    // Take maximum value
    Min,                    // Take minimum value
    Average,                // Take average value
    Priority(Vec<String>),  // Use priority system order
}
```

## 🎯 **Custom System Integration**

### **1. Custom System Adapter**

```rust
// Custom system adapter for Element Core
pub struct CustomSystemAdapter {
    system_id: String,
    supported_elements: Vec<String>,
    data_provider: Arc<dyn ElementDataProvider>,
    event_handler: Arc<dyn ElementEventHandler>,
    priority: i32,
    is_active: bool,
}

impl ElementSystemInterface for CustomSystemAdapter {
    fn get_system_id(&self) -> &str {
        &self.system_id
    }
    
    fn get_supported_elements(&self) -> Vec<String> {
        self.supported_elements.clone()
    }
    
    fn get_actor_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
    ) -> Result<HashMap<DerivedStatType, f64>, ElementError> {
        // Get base stats
        let base_stats = self.data_provider.get_base_element_stats(actor_id, element_type)?;
        
        // Get modifiers
        let modifiers = self.data_provider.get_element_modifiers(actor_id, element_type)?;
        
        // Get bonuses
        let bonuses = self.data_provider.get_element_bonuses(actor_id, element_type)?;
        
        // Get penalties
        let penalties = self.data_provider.get_element_penalties(actor_id, element_type)?;
        
        // Calculate final stats
        let mut final_stats = base_stats;
        
        // Apply modifiers
        for modifier in modifiers {
            if modifier.is_active {
                if let Some(current_value) = final_stats.get(&modifier.stat_type) {
                    let new_value = (current_value + modifier.value) * modifier.multiplier;
                    final_stats.insert(modifier.stat_type, new_value);
                }
            }
        }
        
        // Apply bonuses
        for bonus in bonuses {
            if bonus.is_active {
                if let Some(condition) = &bonus.condition {
                    if self.check_condition(condition, actor_id)? {
                        if let Some(current_value) = final_stats.get(&bonus.stat_type) {
                            let new_value = current_value + bonus.value;
                            final_stats.insert(bonus.stat_type, new_value);
                        }
                    }
                } else {
                    if let Some(current_value) = final_stats.get(&bonus.stat_type) {
                        let new_value = current_value + bonus.value;
                        final_stats.insert(bonus.stat_type, new_value);
                    }
                }
            }
        }
        
        // Apply penalties
        for penalty in penalties {
            if penalty.is_active {
                if let Some(condition) = &penalty.condition {
                    if self.check_condition(condition, actor_id)? {
                        if let Some(current_value) = final_stats.get(&penalty.stat_type) {
                            let new_value = current_value - penalty.value;
                            final_stats.insert(penalty.stat_type, new_value);
                        }
                    }
                } else {
                    if let Some(current_value) = final_stats.get(&penalty.stat_type) {
                        let new_value = current_value - penalty.value;
                        final_stats.insert(penalty.stat_type, new_value);
                    }
                }
            }
        }
        
        Ok(final_stats)
    }
    
    fn update_actor_element_stats(
        &self,
        actor_id: &str,
        element_type: &str,
        stats: HashMap<DerivedStatType, f64>,
    ) -> Result<(), ElementError> {
        // Update stats in custom system
        // Implementation depends on custom system
        Ok(())
    }
    
    fn handle_element_event(&self, event: &ElementEvent) -> Result<(), ElementError> {
        self.event_handler.handle_event(event)
    }
    
    fn get_system_priority(&self) -> i32 {
        self.priority
    }
    
    fn is_system_active(&self) -> bool {
        self.is_active
    }
}

impl CustomSystemAdapter {
    // Check condition for bonuses/penalties
    fn check_condition(
        &self,
        condition: &ElementCondition,
        actor_id: &str,
    ) -> Result<bool, ElementError> {
        // Implementation depends on condition type
        // This is a placeholder
        Ok(true)
    }
}
```

## 🧪 **Testing & Validation**

### **1. Integration Tests**

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_multi_system_aggregation() {
        let mut aggregator = MultiSystemAggregator::new();
        
        // Register systems
        let combat_adapter = Arc::new(CombatCoreAdapter::new());
        let shield_adapter = Arc::new(ShieldSystemAdapter::new());
        
        aggregator.register_system(combat_adapter).unwrap();
        aggregator.register_system(shield_adapter).unwrap();
        
        // Test aggregation
        let stats = aggregator.get_aggregated_element_stats("actor_1", "fire").unwrap();
        
        assert!(stats.contains_key(&DerivedStatType::PowerPoint));
        assert!(stats.contains_key(&DerivedStatType::DefensePoint));
    }
    
    #[test]
    fn test_conflict_resolution() {
        let resolver = ConflictResolver::new();
        
        let mut aggregated_stats = HashMap::new();
        aggregated_stats.insert(DerivedStatType::PowerPoint, 200.0);
        
        let mut system_stats = HashMap::new();
        let mut combat_stats = HashMap::new();
        combat_stats.insert(DerivedStatType::PowerPoint, 100.0);
        system_stats.insert("combat_core".to_string(), combat_stats);
        
        let mut shield_stats = HashMap::new();
        shield_stats.insert(DerivedStatType::PowerPoint, 50.0);
        system_stats.insert("shield_system".to_string(), shield_stats);
        
        let resolved = resolver.resolve_conflicts(&aggregated_stats, &system_stats).unwrap();
        
        assert_eq!(resolved.get(&DerivedStatType::PowerPoint), Some(&200.0));
    }
}
```

## 🎯 **Next Steps**

### **Phase 1: Core Integration**
1. **System Interface**: Implement core interface
2. **Basic Adapters**: Combat, Shield, Race, Item adapters
3. **Event System**: Basic event handling
4. **Configuration**: YAML-based configuration

### **Phase 2: Advanced Features**
1. **Multi-System Aggregation**: Full aggregation system
2. **Conflict Resolution**: Advanced conflict handling
3. **Custom Systems**: Support for custom systems
4. **Performance Optimization**: Caching and optimization

### **Phase 3: Integration Testing**
1. **Unit Tests**: Individual adapter testing
2. **Integration Tests**: Multi-system testing
3. **Performance Tests**: Load and stress testing
4. **End-to-End Tests**: Full system testing

### **Phase 4: Production**
1. **Monitoring**: Performance monitoring
2. **Logging**: Comprehensive logging
3. **Documentation**: User guides and API docs
4. **Support**: Maintenance and support

---

*Tài liệu này sẽ được cập nhật khi có thêm yêu cầu và feedback từ team.*
