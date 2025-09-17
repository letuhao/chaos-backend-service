# Status Core Design Notes

## 📋 **Tổng Quan**

Status Core là một hệ thống chuyên biệt để quản lý các status effects, buffs, debuffs, và immunity effects trong game. Hệ thống này tích hợp chặt chẽ với Action Core, Element Core, và Movement System để tạo ra các hiệu ứng phức tạp và tương tác.

## 🎯 **Vấn Đề Cần Giải Quyết**

### **1. Status Effect Management**
- **Status Effect Types**: Buffs, debuffs, neutral effects
- **Status Effect Stacking**: Cách các status effects tương tác với nhau
- **Status Effect Duration**: Quản lý thời gian hiệu lực
- **Status Effect Magnitude**: Quản lý cường độ hiệu ứng

### **2. Status Immunity System**
- **Immunity Types**: Immunity to specific status effects
- **Immunity Duration**: Thời gian miễn nhiễm
- **Immunity Break Conditions**: Điều kiện phá vỡ immunity
- **Immunity Interactions**: Tương tác giữa các loại immunity

### **3. Complex Status Effects**
- **Multi-layered Effects**: Status effects có nhiều lớp
- **Conditional Effects**: Status effects có điều kiện
- **Dynamic Effects**: Status effects thay đổi theo thời gian
- **Interaction Effects**: Status effects tương tác với nhau

### **4. System Integration**
- **Action Core Integration**: Status effects từ actions
- **Element Core Integration**: Status effects từ elements
- **Movement System Integration**: Status effects ảnh hưởng movement
- **Combat Core Integration**: Status effects trong combat

## 🏗️ **Architecture Overview**

### **Core Components**

```rust
pub struct StatusCore {
    // Core components
    status_effect_manager: StatusEffectManager,
    immunity_manager: ImmunityManager,
    status_calculator: StatusCalculator,
    status_validator: StatusValidator,
    
    // Integration components
    action_core_client: ActionCoreClient,
    element_core_client: ElementCoreClient,
    movement_system_client: MovementSystemClient,
    combat_core_client: CombatCoreClient,
    
    // Performance optimization
    status_cache: StatusCache,
    batch_processor: StatusBatchProcessor,
    memory_pool: StatusMemoryPool,
    
    // Configuration
    config: StatusConfig,
}

pub struct StatusEffectManager {
    active_effects: HashMap<ActorId, Vec<StatusEffect>>,
    effect_registry: StatusEffectRegistry,
    effect_calculator: StatusEffectCalculator,
    effect_applier: StatusEffectApplier,
}

pub struct ImmunityManager {
    immunity_registry: ImmunityRegistry,
    immunity_calculator: ImmunityCalculator,
    immunity_validator: ImmunityValidator,
}
```

## 🔧 **Status Effect System**

### **1. Status Effect Types**

```rust
pub enum StatusEffectType {
    // Buffs
    Buff,                    // Buff cơ bản
    Enhancement,             // Tăng cường
    Regeneration,            // Hồi phục
    Protection,              // Bảo vệ
    Empowerment,             // Tăng sức mạnh
    
    // Debuffs
    Debuff,                  // Debuff cơ bản
    Weakness,                // Yếu đuối
    Damage,                  // Gây sát thương
    Impairment,              // Hạn chế
    Curse,                   // Lời nguyền
    
    // Neutral Effects
    Neutral,                 // Trung tính
    Information,             // Thông tin
    Visual,                  // Hiệu ứng thị giác
    Audio,                   // Hiệu ứng âm thanh
    
    // Special Effects
    Transformation,          // Biến đổi
    Crystallization,         // Kết tinh
    Immunity,                // Miễn nhiễm
    ImmunityBreak,           // Phá vỡ miễn nhiễm
}

pub struct StatusEffect {
    pub effect_id: String,
    pub effect_name: String,
    pub effect_name_vi: String,
    pub effect_type: StatusEffectType,
    pub magnitude: f64,
    pub duration: Duration,
    pub target: String,
    pub source: String,
    pub conditions: Vec<StatusCondition>,
    pub interactions: Vec<StatusInteraction>,
    pub immunity_list: Vec<String>,
    pub movement_restrictions: Vec<MovementRestriction>,
}
```

### **2. Status Effect Examples**

```yaml
# Crystallization Status Effect
crystallization_effect:
  effect_id: "crystallization"
  effect_name: "Crystallization"
  effect_name_vi: "Kết Tinh"
  effect_type: "Transformation"
  magnitude: 1.0
  duration: "5.0s"
  target: "Self"
  source: "Crystal Defense Technique"
  conditions:
    - condition_type: "HealthPercentage"
      condition_value: 0.1
      condition_operator: "GreaterThan"
  interactions:
    - interaction_type: "DefenseMultiplier"
      target_stat: "defense_point"
      multiplier: 20.0
    - interaction_type: "DefenseBonus"
      target_stat: "defense_point"
      bonus: 100000.0
    - interaction_type: "ElementalResistance"
      target_stat: "elemental_resistance"
      bonus: 0.8
  immunity_list:
    - "stun"
    - "knockback"
    - "movement_impairment"
  movement_restrictions:
    - restriction_type: "Immobilized"
      magnitude: 1.0
      duration: "5.0s"

# Fire Regeneration Buff
fire_regeneration_buff:
  effect_id: "fire_regeneration"
  effect_name: "Fire Regeneration"
  effect_name_vi: "Hồi Phục Hỏa"
  effect_type: "Regeneration"
  magnitude: 0.05
  duration: "10.0s"
  target: "Self"
  source: "Fire Element Interaction"
  conditions:
    - condition_type: "ElementMastery"
      condition_value: 1000
      condition_operator: "GreaterThan"
  interactions:
    - interaction_type: "HealthRegeneration"
      target_stat: "health"
      multiplier: 0.05
    - interaction_type: "ManaRegeneration"
      target_stat: "mana"
      multiplier: 0.03
  immunity_list: []
  movement_restrictions: []
```

## 🔧 **Immunity System**

### **1. Immunity Types**

```rust
pub enum ImmunityType {
    // Status Immunity
    StatusImmunity,          // Miễn nhiễm status effects
    DebuffImmunity,          // Miễn nhiễm debuffs
    BuffImmunity,            // Miễn nhiễm buffs
    CurseImmunity,           // Miễn nhiễm lời nguyền
    
    // Element Immunity
    ElementImmunity,         // Miễn nhiễm nguyên tố
    PhysicalImmunity,        // Miễn nhiễm vật lý
    MagicalImmunity,         // Miễn nhiễm phép thuật
    
    // Damage Immunity
    DamageImmunity,          // Miễn nhiễm sát thương
    PhysicalDamageImmunity,  // Miễn nhiễm sát thương vật lý
    MagicalDamageImmunity,   // Miễn nhiễm sát thương phép thuật
    
    // Special Immunity
    MovementImmunity,        // Miễn nhiễm hạn chế di chuyển
    StatusEffectImmunity,    // Miễn nhiễm tất cả status effects
    DeathImmunity,           // Miễn nhiễm tử vong
}

pub struct Immunity {
    pub immunity_id: String,
    pub immunity_name: String,
    pub immunity_name_vi: String,
    pub immunity_type: ImmunityType,
    pub target_effects: Vec<String>,
    pub magnitude: f64,
    pub duration: Duration,
    pub break_conditions: Vec<ImmunityBreakCondition>,
    pub source: String,
}
```

### **2. Immunity Examples**

```yaml
# Crystallization Immunity
crystallization_immunity:
  immunity_id: "crystallization_immunity"
  immunity_name: "Crystallization Immunity"
  immunity_name_vi: "Miễn Nhiễm Kết Tinh"
  immunity_type: "StatusImmunity"
  target_effects:
    - "stun"
    - "knockback"
    - "movement_impairment"
    - "paralysis"
    - "root"
  magnitude: 1.0
  duration: "5.0s"
  break_conditions:
    - condition_type: "HealthPercentage"
      condition_value: 0.05
      condition_operator: "LessThan"
    - condition_type: "StatusEffect"
      condition_value: 1.0
      condition_operator: "EqualTo"
      condition_target: "crystallization_break"
  source: "Crystal Defense Technique"

# Element Immunity
element_immunity:
  immunity_id: "element_immunity"
  immunity_name: "Element Immunity"
  immunity_name_vi: "Miễn Nhiễm Nguyên Tố"
  immunity_type: "ElementImmunity"
  target_effects:
    - "fire_damage"
    - "water_damage"
    - "earth_damage"
    - "wood_damage"
    - "metal_damage"
    - "air_damage"
    - "lightning_damage"
  magnitude: 0.8
  duration: "15.0s"
  break_conditions:
    - condition_type: "ElementMastery"
      condition_value: 2000
      condition_operator: "GreaterThan"
      condition_target: "attacker"
  source: "Element Mastery"
```

## 🔧 **Complex Status Effects**

### **1. Multi-layered Effects**

```rust
pub struct MultiLayeredStatusEffect {
    pub base_effect: StatusEffect,
    pub layers: Vec<StatusEffectLayer>,
    pub layer_interactions: Vec<LayerInteraction>,
}

pub struct StatusEffectLayer {
    pub layer_id: String,
    pub layer_name: String,
    pub effect_type: StatusEffectType,
    pub magnitude: f64,
    pub duration: Duration,
    pub conditions: Vec<StatusCondition>,
    pub dependencies: Vec<String>,
}

pub struct LayerInteraction {
    pub source_layer: String,
    pub target_layer: String,
    pub interaction_type: LayerInteractionType,
    pub multiplier: f64,
    pub conditions: Vec<StatusCondition>,
}

pub enum LayerInteractionType {
    Amplify,        // Khuếch đại
    Suppress,       // Áp chế
    Transform,      // Biến đổi
    Merge,          // Hợp nhất
    Split,          // Tách ra
}
```

### **2. Conditional Effects**

```rust
pub struct ConditionalStatusEffect {
    pub base_effect: StatusEffect,
    pub conditions: Vec<StatusCondition>,
    pub conditional_effects: Vec<ConditionalEffect>,
}

pub struct StatusCondition {
    pub condition_type: StatusConditionType,
    pub condition_value: f64,
    pub condition_operator: StatusConditionOperator,
    pub condition_target: String,
    pub condition_duration: Option<Duration>,
}

pub enum StatusConditionType {
    HealthPercentage,
    ManaPercentage,
    StaminaPercentage,
    ElementMastery,
    StatusEffect,
    TerrainType,
    TimeOfDay,
    Weather,
    Custom(String),
}

pub enum StatusConditionOperator {
    GreaterThan,
    LessThan,
    EqualTo,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqualTo,
    Contains,
    NotContains,
}
```

## 🔧 **System Integration**

### **1. Action Core Integration**

```rust
pub struct ActionStatusIntegration {
    action_core_client: ActionCoreClient,
    status_effect_manager: StatusEffectManager,
}

impl ActionStatusIntegration {
    /// Apply status effects from action execution
    pub async fn apply_action_status_effects(
        &self,
        action: &dyn Action,
        actor: &mut Actor,
        context: &ActionContext
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        let mut results = Vec::new();
        
        for effect in action.get_effects() {
            let status_effect = self.convert_action_effect_to_status_effect(effect)?;
            let result = self.status_effect_manager.apply_effect(
                actor.get_id(),
                status_effect
            ).await?;
            
            results.push(result);
        }
        
        Ok(results)
    }
}
```

### **2. Element Core Integration**

```rust
pub struct ElementStatusIntegration {
    element_core_client: ElementCoreClient,
    status_effect_manager: StatusEffectManager,
}

impl ElementStatusIntegration {
    /// Apply status effects from element interactions
    pub async fn apply_element_status_effects(
        &self,
        element_interaction: &ElementInteraction,
        actor: &mut Actor
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        let mut results = Vec::new();
        
        for effect in &element_interaction.status_effects {
            let status_effect = self.convert_element_effect_to_status_effect(effect)?;
            let result = self.status_effect_manager.apply_effect(
                actor.get_id(),
                status_effect
            ).await?;
            
            results.push(result);
        }
        
        Ok(results)
    }
}
```

### **3. Movement System Integration**

```rust
pub struct MovementStatusIntegration {
    movement_system_client: MovementSystemClient,
    status_effect_manager: StatusEffectManager,
}

impl MovementStatusIntegration {
    /// Apply movement restrictions from status effects
    pub async fn apply_movement_restrictions(
        &self,
        actor_id: ActorId,
        status_effects: &[StatusEffect]
    ) -> Result<Vec<MovementRestriction>, StatusError> {
        let mut restrictions = Vec::new();
        
        for effect in status_effects {
            for restriction in &effect.movement_restrictions {
                restrictions.push(restriction.clone());
            }
        }
        
        Ok(restrictions)
    }
}
```

## 📊 **Configuration Files**

### **1. Status Effect Configuration**

```yaml
# chaos-backend-service/docs/status-core/configs/status_effects.yaml
version: 1.0

status_effects:
  crystallization:
    effect_id: "crystallization"
    effect_name: "Crystallization"
    effect_name_vi: "Kết Tinh"
    effect_type: "Transformation"
    magnitude: 1.0
    duration: "5.0s"
    target: "Self"
    source: "Crystal Defense Technique"
    conditions:
      - condition_type: "HealthPercentage"
        condition_value: 0.1
        condition_operator: "GreaterThan"
    interactions:
      - interaction_type: "DefenseMultiplier"
        target_stat: "defense_point"
        multiplier: 20.0
      - interaction_type: "DefenseBonus"
        target_stat: "defense_point"
        bonus: 100000.0
      - interaction_type: "ElementalResistance"
        target_stat: "elemental_resistance"
        bonus: 0.8
    immunity_list:
      - "stun"
      - "knockback"
      - "movement_impairment"
    movement_restrictions:
      - restriction_type: "Immobilized"
        magnitude: 1.0
        duration: "5.0s"

  fire_regeneration:
    effect_id: "fire_regeneration"
    effect_name: "Fire Regeneration"
    effect_name_vi: "Hồi Phục Hỏa"
    effect_type: "Regeneration"
    magnitude: 0.05
    duration: "10.0s"
    target: "Self"
    source: "Fire Element Interaction"
    conditions:
      - condition_type: "ElementMastery"
        condition_value: 1000
        condition_operator: "GreaterThan"
    interactions:
      - interaction_type: "HealthRegeneration"
        target_stat: "health"
        multiplier: 0.05
      - interaction_type: "ManaRegeneration"
        target_stat: "mana"
        multiplier: 0.03
    immunity_list: []
    movement_restrictions: []

# Immunity configuration
immunities:
  crystallization_immunity:
    immunity_id: "crystallization_immunity"
    immunity_name: "Crystallization Immunity"
    immunity_name_vi: "Miễn Nhiễm Kết Tinh"
    immunity_type: "StatusImmunity"
    target_effects:
      - "stun"
      - "knockback"
      - "movement_impairment"
      - "paralysis"
      - "root"
    magnitude: 1.0
    duration: "5.0s"
    break_conditions:
      - condition_type: "HealthPercentage"
        condition_value: 0.05
        condition_operator: "LessThan"
      - condition_type: "StatusEffect"
        condition_value: 1.0
        condition_operator: "EqualTo"
        condition_target: "crystallization_break"
    source: "Crystal Defense Technique"

# Global status settings
global_settings:
  max_status_effects_per_actor: 20
  status_effect_duration_cap: 3600.0  # 1 hour
  immunity_duration_cap: 1800.0       # 30 minutes
  status_effect_cleanup_interval: 60.0  # 1 minute
```

## 🧪 **Testing Strategy**

### **1. Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_status_effect_application() {
        let status_manager = StatusEffectManager::new();
        let actor_id = ActorId::new();
        let effect = StatusEffect::new("test_effect", StatusEffectType::Buff, 1.0, Duration::from_secs_f64(10.0));
        
        let result = status_manager.apply_effect(actor_id, effect).await?;
        assert!(result.success);
    }
    
    #[test]
    fn test_immunity_system() {
        let immunity_manager = ImmunityManager::new();
        let actor_id = ActorId::new();
        let immunity = Immunity::new("test_immunity", ImmunityType::StatusImmunity, vec!["stun".to_string()]);
        
        immunity_manager.apply_immunity(actor_id, immunity).await?;
        
        let is_immune = immunity_manager.is_immune_to(actor_id, "stun").await?;
        assert!(is_immune);
    }
}
```

### **2. Integration Tests**

```rust
#[tokio::test]
async fn test_crystal_defense_status_integration() {
    let status_core = StatusCore::new();
    let crystal_defense = CrystalDefenseTechnique::new();
    let mut actor = create_test_actor();
    
    // Apply crystallization
    crystal_defense.apply_crystallization_effects(&mut actor, &status_core).await?;
    
    // Test status effects
    let effects = status_core.get_actor_status_effects(actor.get_id()).await?;
    assert!(effects.iter().any(|e| e.effect_id == "crystallization"));
    
    // Test immunity
    let is_immune = status_core.is_immune_to(actor.get_id(), "stun").await?;
    assert!(is_immune);
    
    // Test movement restrictions
    let restrictions = status_core.get_movement_restrictions(actor.get_id()).await?;
    assert!(restrictions.iter().any(|r| r.restriction_type == MovementRestrictionType::Immobilized));
}
```

## 📝 **Implementation Notes**

### **1. Performance Considerations**
- **Status Effect Caching**: Cache active status effects for performance
- **Batch Processing**: Process status effects in batches
- **Lazy Evaluation**: Calculate status effects only when needed

### **2. Scalability Considerations**
- **Actor-based Partitioning**: Partition status effects by actor
- **Time-based Cleanup**: Clean up expired status effects
- **Memory Management**: Efficient memory usage for status effects

### **3. Integration Considerations**
- **Action Core**: Status effects from actions
- **Element Core**: Status effects from elements
- **Movement System**: Movement restrictions from status effects
- **Combat Core**: Status effects in combat

## 🔄 **Next Steps**

### **1. Phase 1: Basic Status Effects (2-3 tuần)**
- Implement basic status effect system
- Implement immunity system
- Implement status effect application and removal

### **2. Phase 2: Complex Status Effects (2-3 tuần)**
- Implement multi-layered effects
- Implement conditional effects
- Implement dynamic effects

### **3. Phase 3: System Integration (2-3 tuần)**
- Integrate with Action Core
- Integrate with Element Core
- Integrate with Movement System

### **4. Phase 4: Advanced Features (2-3 tuần)**
- Implement status effect interactions
- Implement advanced immunity system
- Implement performance optimizations

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Notes  
**Maintainer**: Chaos World Team
