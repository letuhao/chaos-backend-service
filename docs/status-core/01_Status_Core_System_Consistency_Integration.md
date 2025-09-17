# Status Core System Consistency & Integration

## 📋 **Tổng Quan**

Document này mô tả cách Status Core kế thừa và tích hợp với các hệ thống status/buff/debuff hiện có từ Element Core, Combat Core, và Resource Exhaustion Systems để đảm bảo tính nhất quán và tránh trùng lặp.

## 🔗 **System Inheritance & Integration**

### **1. Element Core Status Integration**

#### **A. Elemental Status Effects**
```rust
// Element Core provides elemental status effects
pub struct ElementalStatusEffect {
    pub element_id: String,
    pub effect_name: String,
    pub effect_type: ElementalStatusEffectType,
    pub magnitude: f64,
    pub duration: Duration,
    pub target: String,
    pub source: String,
    pub element_mastery_requirement: f64,
    pub element_interaction_bonus: f64,
}

pub enum ElementalStatusEffectType {
    // Fire effects
    Burning,                // Cháy
    FireRegeneration,       // Hồi phục hỏa
    FireResistance,         // Kháng hỏa
    FireImmunity,           // Miễn nhiễm hỏa
    
    // Water effects
    Freezing,               // Đóng băng
    WaterFlow,              // Chảy nước
    WaterResistance,        // Kháng thủy
    WaterImmunity,          // Miễn nhiễm thủy
    
    // Earth effects
    Rooted,                 // Bị rễ
    EarthShield,            // Khiên đất
    EarthResistance,        // Kháng thổ
    EarthImmunity,          // Miễn nhiễm thổ
    
    // Wood effects
    Entangled,              // Bị quấn
    WoodGrowth,             // Mọc gỗ
    WoodResistance,         // Kháng mộc
    WoodImmunity,           // Miễn nhiễm mộc
    
    // Metal effects
    Sharpened,              // Sắc bén
    MetalArmor,             // Giáp kim
    MetalResistance,        // Kháng kim
    MetalImmunity,          // Miễn nhiễm kim
    
    // Air effects
    WindBoost,              // Tăng tốc gió
    AirResistance,          // Kháng phong
    AirImmunity,            // Miễn nhiễm phong
    
    // Lightning effects
    Electrified,            // Bị điện giật
    LightningSpeed,         // Tốc độ sét
    LightningResistance,    // Kháng lôi
    LightningImmunity,      // Miễn nhiễm lôi
}
```

#### **B. Element Core Status Integration**
```rust
pub struct ElementStatusIntegration {
    element_core_client: ElementCoreClient,
    status_core_client: StatusCoreClient,
}

impl ElementStatusIntegration {
    /// Convert Element Core status effects to Status Core format
    pub async fn convert_elemental_status_effects(
        &self,
        element_effects: &[ElementalStatusEffect]
    ) -> Result<Vec<StatusEffect>, StatusError> {
        let mut status_effects = Vec::new();
        
        for element_effect in element_effects {
            let status_effect = self.convert_elemental_to_status_effect(element_effect).await?;
            status_effects.push(status_effect);
        }
        
        Ok(status_effects)
    }
    
    async fn convert_elemental_to_status_effect(
        &self,
        element_effect: &ElementalStatusEffect
    ) -> Result<StatusEffect, StatusError> {
        let status_effect = StatusEffect {
            effect_id: format!("elemental_{}", element_effect.effect_name),
            effect_name: element_effect.effect_name.clone(),
            effect_name_vi: self.translate_effect_name(&element_effect.effect_name),
            effect_type: self.convert_elemental_effect_type(element_effect.effect_type),
            magnitude: element_effect.magnitude,
            duration: element_effect.duration,
            target: element_effect.target.clone(),
            source: format!("element_{}", element_effect.element_id),
            conditions: vec![
                StatusCondition {
                    condition_type: StatusConditionType::ElementMastery,
                    condition_value: element_effect.element_mastery_requirement,
                    condition_operator: StatusConditionOperator::GreaterThanOrEqual,
                    condition_target: element_effect.element_id.clone(),
                }
            ],
            interactions: vec![
                StatusInteraction {
                    interaction_type: StatusInteractionType::ElementBonus,
                    target_stat: "element_mastery".to_string(),
                    multiplier: element_effect.element_interaction_bonus,
                }
            ],
            immunity_list: vec![],
            movement_restrictions: vec![],
        };
        
        Ok(status_effect)
    }
}
```

### **2. Combat Core Status Integration**

#### **A. Combat Status Effects**
```rust
// Combat Core provides combat-related status effects
pub struct CombatStatusEffect {
    pub effect_name: String,
    pub effect_type: CombatStatusEffectType,
    pub magnitude: f64,
    pub duration: Duration,
    pub target: String,
    pub source: String,
    pub combat_requirement: CombatRequirement,
    pub combat_bonus: f64,
}

pub enum CombatStatusEffectType {
    // Damage effects
    Bleeding,               // Chảy máu
    Poison,                 // Độc
    Burn,                   // Bỏng
    Freeze,                 // Đóng băng
    Shock,                  // Sốc điện
    
    // Defense effects
    Shield,                 // Khiên
    Armor,                  // Giáp
    Resistance,             // Kháng
    Immunity,               // Miễn nhiễm
    
    // Movement effects
    Slowed,                 // Chậm
    Rooted,                 // Bị rễ
    Stunned,                // Choáng
    Paralyzed,              // Tê liệt
    
    // Buff effects
    Strength,               // Sức mạnh
    Agility,                // Nhanh nhẹn
    Endurance,              // Bền bỉ
    Intelligence,           // Thông minh
    Wisdom,                 // Khôn ngoan
}
```

#### **B. Combat Core Status Integration**
```rust
pub struct CombatStatusIntegration {
    combat_core_client: CombatCoreClient,
    status_core_client: StatusCoreClient,
}

impl CombatStatusIntegration {
    /// Convert Combat Core status effects to Status Core format
    pub async fn convert_combat_status_effects(
        &self,
        combat_effects: &[CombatStatusEffect]
    ) -> Result<Vec<StatusEffect>, StatusError> {
        let mut status_effects = Vec::new();
        
        for combat_effect in combat_effects {
            let status_effect = self.convert_combat_to_status_effect(combat_effect).await?;
            status_effects.push(status_effect);
        }
        
        Ok(status_effects)
    }
    
    async fn convert_combat_to_status_effect(
        &self,
        combat_effect: &CombatStatusEffect
    ) -> Result<StatusEffect, StatusError> {
        let status_effect = StatusEffect {
            effect_id: format!("combat_{}", combat_effect.effect_name),
            effect_name: combat_effect.effect_name.clone(),
            effect_name_vi: self.translate_effect_name(&combat_effect.effect_name),
            effect_type: self.convert_combat_effect_type(combat_effect.effect_type),
            magnitude: combat_effect.magnitude,
            duration: combat_effect.duration,
            target: combat_effect.target.clone(),
            source: "combat".to_string(),
            conditions: vec![
                StatusCondition {
                    condition_type: StatusConditionType::CombatLevel,
                    condition_value: combat_effect.combat_requirement.min_level as f64,
                    condition_operator: StatusConditionOperator::GreaterThanOrEqual,
                    condition_target: "combat_level".to_string(),
                }
            ],
            interactions: vec![
                StatusInteraction {
                    interaction_type: StatusInteractionType::CombatBonus,
                    target_stat: "combat_power".to_string(),
                    multiplier: combat_effect.combat_bonus,
                }
            ],
            immunity_list: vec![],
            movement_restrictions: vec![],
        };
        
        Ok(status_effect)
    }
}
```

### **3. Resource Exhaustion System Integration**

#### **A. Resource Exhaustion Effects**
```rust
// Resource Exhaustion System provides resource-related status effects
pub struct ResourceExhaustionEffect {
    pub resource_type: String,
    pub effect_name: String,
    pub effect_type: ResourceExhaustionEffectType,
    pub magnitude: f64,
    pub duration: Duration,
    pub target: String,
    pub source: String,
    pub resource_threshold: f64,
    pub resource_penalty: f64,
}

pub enum ResourceExhaustionEffectType {
    // Health exhaustion
    HealthExhaustion,       // Kiệt sức máu
    HealthRegeneration,     // Hồi phục máu
    HealthDrain,            // Hút máu
    
    // Mana exhaustion
    ManaExhaustion,         // Kiệt sức mana
    ManaRegeneration,       // Hồi phục mana
    ManaDrain,              // Hút mana
    
    // Stamina exhaustion
    StaminaExhaustion,      // Kiệt sức thể lực
    StaminaRegeneration,    // Hồi phục thể lực
    StaminaDrain,           // Hút thể lực
    
    // Qi exhaustion
    QiExhaustion,           // Kiệt sức khí
    QiRegeneration,         // Hồi phục khí
    QiDrain,                // Hút khí
    
    // Life Force exhaustion
    LifeForceExhaustion,    // Kiệt sức thọ nguyên
    LifeForceRegeneration,  // Hồi phục thọ nguyên
    LifeForceDrain,         // Hút thọ nguyên
}
```

#### **B. Resource Exhaustion Integration**
```rust
pub struct ResourceExhaustionIntegration {
    resource_manager_client: ResourceManagerClient,
    status_core_client: StatusCoreClient,
}

impl ResourceExhaustionIntegration {
    /// Convert Resource Exhaustion effects to Status Core format
    pub async fn convert_resource_exhaustion_effects(
        &self,
        resource_effects: &[ResourceExhaustionEffect]
    ) -> Result<Vec<StatusEffect>, StatusError> {
        let mut status_effects = Vec::new();
        
        for resource_effect in resource_effects {
            let status_effect = self.convert_resource_to_status_effect(resource_effect).await?;
            status_effects.push(status_effect);
        }
        
        Ok(status_effects)
    }
    
    async fn convert_resource_to_status_effect(
        &self,
        resource_effect: &ResourceExhaustionEffect
    ) -> Result<StatusEffect, StatusError> {
        let status_effect = StatusEffect {
            effect_id: format!("resource_{}", resource_effect.effect_name),
            effect_name: resource_effect.effect_name.clone(),
            effect_name_vi: self.translate_effect_name(&resource_effect.effect_name),
            effect_type: self.convert_resource_effect_type(resource_effect.effect_type),
            magnitude: resource_effect.magnitude,
            duration: resource_effect.duration,
            target: resource_effect.target.clone(),
            source: format!("resource_{}", resource_effect.resource_type),
            conditions: vec![
                StatusCondition {
                    condition_type: StatusConditionType::ResourcePercentage,
                    condition_value: resource_effect.resource_threshold,
                    condition_operator: StatusConditionOperator::LessThan,
                    condition_target: resource_effect.resource_type.clone(),
                }
            ],
            interactions: vec![
                StatusInteraction {
                    interaction_type: StatusInteractionType::ResourcePenalty,
                    target_stat: resource_effect.resource_type.clone(),
                    multiplier: resource_effect.resource_penalty,
                }
            ],
            immunity_list: vec![],
            movement_restrictions: vec![],
        };
        
        Ok(status_effect)
    }
}
```

## 🏗️ **Unified Status Core Architecture**

### **1. Centralized Status Management**

```rust
pub struct UnifiedStatusCore {
    // Core components
    status_effect_manager: StatusEffectManager,
    immunity_manager: ImmunityManager,
    status_calculator: StatusCalculator,
    status_validator: StatusValidator,
    
    // System integrations
    element_status_integration: ElementStatusIntegration,
    combat_status_integration: CombatStatusIntegration,
    resource_exhaustion_integration: ResourceExhaustionIntegration,
    
    // Unified registry
    unified_status_registry: UnifiedStatusRegistry,
    
    // Performance optimization
    status_cache: StatusCache,
    batch_processor: StatusBatchProcessor,
    memory_pool: StatusMemoryPool,
}

pub struct UnifiedStatusRegistry {
    // Elemental status effects
    elemental_effects: HashMap<String, ElementalStatusEffect>,
    
    // Combat status effects
    combat_effects: HashMap<String, CombatStatusEffect>,
    
    // Resource exhaustion effects
    resource_effects: HashMap<String, ResourceExhaustionEffect>,
    
    // Custom status effects
    custom_effects: HashMap<String, StatusEffect>,
    
    // Effect mappings
    effect_mappings: HashMap<String, Vec<String>>, // effect_id -> source_systems
}
```

### **2. Unified Status Effect**

```rust
pub struct UnifiedStatusEffect {
    // Basic properties
    pub effect_id: String,
    pub effect_name: String,
    pub effect_name_vi: String,
    pub effect_type: UnifiedStatusEffectType,
    pub magnitude: f64,
    pub duration: Duration,
    pub target: String,
    pub source: String,
    
    // Source system information
    pub source_systems: Vec<SourceSystem>,
    pub source_effects: Vec<SourceEffect>,
    
    // Unified properties
    pub conditions: Vec<StatusCondition>,
    pub interactions: Vec<StatusInteraction>,
    pub immunity_list: Vec<String>,
    pub movement_restrictions: Vec<MovementRestriction>,
    
    // System-specific properties
    pub element_properties: Option<ElementalProperties>,
    pub combat_properties: Option<CombatProperties>,
    pub resource_properties: Option<ResourceProperties>,
}

pub enum UnifiedStatusEffectType {
    // Elemental types
    Elemental(ElementalStatusEffectType),
    
    // Combat types
    Combat(CombatStatusEffectType),
    
    // Resource types
    Resource(ResourceExhaustionEffectType),
    
    // Custom types
    Custom(String),
}

pub enum SourceSystem {
    ElementCore,
    CombatCore,
    ResourceManager,
    StatusCore,
    Custom(String),
}

pub struct SourceEffect {
    pub system: SourceSystem,
    pub effect_id: String,
    pub weight: f64, // Weight of this source in the unified effect
}
```

### **3. Status Effect Resolution**

```rust
impl UnifiedStatusCore {
    /// Resolve conflicts between status effects from different systems
    pub async fn resolve_status_conflicts(
        &self,
        actor_id: ActorId,
        new_effects: Vec<UnifiedStatusEffect>
    ) -> Result<Vec<UnifiedStatusEffect>, StatusError> {
        let mut resolved_effects = Vec::new();
        let existing_effects = self.get_actor_status_effects(actor_id).await?;
        
        for new_effect in new_effects {
            let conflicts = self.find_conflicts(&new_effect, &existing_effects).await?;
            
            if conflicts.is_empty() {
                resolved_effects.push(new_effect);
            } else {
                let resolved_effect = self.resolve_conflict(new_effect, conflicts).await?;
                resolved_effects.push(resolved_effect);
            }
        }
        
        Ok(resolved_effects)
    }
    
    async fn resolve_conflict(
        &self,
        new_effect: UnifiedStatusEffect,
        conflicts: Vec<UnifiedStatusEffect>
    ) -> Result<UnifiedStatusEffect, StatusError> {
        // Priority resolution: StatusCore > ElementCore > CombatCore > ResourceManager
        let mut resolved_effect = new_effect;
        
        for conflict in conflicts {
            if self.has_higher_priority(&resolved_effect, &conflict) {
                resolved_effect = self.merge_effects(resolved_effect, conflict).await?;
            }
        }
        
        Ok(resolved_effect)
    }
    
    fn has_higher_priority(
        &self,
        effect1: &UnifiedStatusEffect,
        effect2: &UnifiedStatusEffect
    ) -> bool {
        let priority1 = self.get_system_priority(&effect1.source_systems);
        let priority2 = self.get_system_priority(&effect2.source_systems);
        
        priority1 > priority2
    }
    
    fn get_system_priority(&self, systems: &[SourceSystem]) -> u32 {
        systems.iter().map(|system| {
            match system {
                SourceSystem::StatusCore => 4,
                SourceSystem::ElementCore => 3,
                SourceSystem::CombatCore => 2,
                SourceSystem::ResourceManager => 1,
                SourceSystem::Custom(_) => 0,
            }
        }).max().unwrap_or(0)
    }
}
```

## 🔧 **Crystal Defense Technique Status Integration**

### **1. Crystal Defense Status Effects**

```yaml
# Crystal Defense Technique status effects
crystal_defense_status_effects:
  # Primary crystallization effect
  crystallization:
    effect_id: "crystallization"
    effect_name: "Crystallization"
    effect_name_vi: "Kết Tinh"
    effect_type: "Transformation"
    magnitude: 1.0
    duration: "5.0s"
    target: "Self"
    source: "Crystal Defense Technique"
    
    # Source systems
    source_systems:
      - "StatusCore"      # Primary system
      - "ElementCore"     # Elemental properties
      - "CombatCore"      # Combat properties
    
    # Elemental properties (from Element Core)
    element_properties:
      primary_elements: ["earth", "metal"]
      target_categories: ["physical", "elemental"]
      element_mastery_requirement:
        earth: 1000
        metal: 1000
      element_interaction_bonus: 0.2
    
    # Combat properties (from Combat Core)
    combat_properties:
      defense_multiplier: 20.0
      defense_bonus: 100000
      elemental_resistance: 0.8
      status_resistance: 0.5
    
    # Status properties (from Status Core)
    status_properties:
      immunity_list: ["stun", "knockback", "movement_impairment"]
      movement_restrictions:
        - restriction_type: "Immobilized"
          magnitude: 1.0
          duration: "5.0s"
      visual_effects:
        - effect_name: "crystal_formation"
          intensity: "high"
          duration: "5.0s"
```

### **2. Crystal Defense Status Resolution**

```rust
impl CrystalDefenseTechnique {
    /// Apply crystallization status effect with system integration
    pub async fn apply_crystallization_status(
        &self,
        actor: &mut Actor,
        unified_status_core: &mut UnifiedStatusCore
    ) -> Result<(), ActionError> {
        // Create elemental status effect
        let elemental_effect = ElementalStatusEffect {
            element_id: "earth_metal".to_string(),
            effect_name: "crystallization".to_string(),
            effect_type: ElementalStatusEffectType::EarthShield,
            magnitude: 1.0,
            duration: Duration::from_secs_f64(5.0),
            target: "Self".to_string(),
            source: "Crystal Defense Technique".to_string(),
            element_mastery_requirement: 1000.0,
            element_interaction_bonus: 0.2,
        };
        
        // Create combat status effect
        let combat_effect = CombatStatusEffect {
            effect_name: "crystal_defense".to_string(),
            effect_type: CombatStatusEffectType::Shield,
            magnitude: 20.0,
            duration: Duration::from_secs_f64(5.0),
            target: "Self".to_string(),
            source: "Crystal Defense Technique".to_string(),
            combat_requirement: CombatRequirement { min_level: 50 },
            combat_bonus: 100000.0,
        };
        
        // Create unified status effect
        let unified_effect = UnifiedStatusEffect {
            effect_id: "crystallization".to_string(),
            effect_name: "Crystallization".to_string(),
            effect_name_vi: "Kết Tinh".to_string(),
            effect_type: UnifiedStatusEffectType::Custom("Transformation".to_string()),
            magnitude: 1.0,
            duration: Duration::from_secs_f64(5.0),
            target: "Self".to_string(),
            source: "Crystal Defense Technique".to_string(),
            source_systems: vec![
                SourceSystem::StatusCore,
                SourceSystem::ElementCore,
                SourceSystem::CombatCore,
            ],
            source_effects: vec![
                SourceEffect {
                    system: SourceSystem::ElementCore,
                    effect_id: "elemental_crystallization".to_string(),
                    weight: 0.4,
                },
                SourceEffect {
                    system: SourceSystem::CombatCore,
                    effect_id: "combat_crystal_defense".to_string(),
                    weight: 0.4,
                },
                SourceEffect {
                    system: SourceSystem::StatusCore,
                    effect_id: "status_crystallization".to_string(),
                    weight: 0.2,
                },
            ],
            conditions: vec![],
            interactions: vec![],
            immunity_list: vec![
                "stun".to_string(),
                "knockback".to_string(),
                "movement_impairment".to_string(),
            ],
            movement_restrictions: vec![
                MovementRestriction {
                    restriction_type: MovementRestrictionType::Immobilized,
                    magnitude: 1.0,
                    duration: Duration::from_secs_f64(5.0),
                    source: RestrictionSource::StatusEffect("crystallized".to_string()),
                    conditions: vec![],
                }
            ],
            element_properties: Some(ElementalProperties {
                primary_elements: vec!["earth".to_string(), "metal".to_string()],
                target_categories: vec!["physical".to_string(), "elemental".to_string()],
                element_mastery_requirement: 1000.0,
                element_interaction_bonus: 0.2,
            }),
            combat_properties: Some(CombatProperties {
                defense_multiplier: 20.0,
                defense_bonus: 100000.0,
                elemental_resistance: 0.8,
                status_resistance: 0.5,
            }),
            resource_properties: None,
        };
        
        // Apply unified status effect
        unified_status_core.apply_unified_effect(actor.get_id(), unified_effect).await?;
        
        Ok(())
    }
}
```

## 📊 **System Consistency Rules**

### **1. Naming Conventions**

```rust
// Consistent naming across all systems
pub struct StatusNamingConventions {
    // Effect naming
    pub effect_id_format: String,        // "system_effect_name"
    pub effect_name_format: String,      // "Effect Name"
    pub effect_name_vi_format: String,   // "Tên Hiệu Ứng"
    
    // Source naming
    pub source_format: String,           // "System Name"
    pub source_id_format: String,        // "system_name"
    
    // Condition naming
    pub condition_format: String,        // "condition_type_condition_value"
    pub condition_operator_format: String, // "operator_name"
}

// Examples
let naming = StatusNamingConventions {
    effect_id_format: "elemental_burning".to_string(),
    effect_name_format: "Burning".to_string(),
    effect_name_vi_format: "Cháy".to_string(),
    source_format: "Element Core".to_string(),
    source_id_format: "element_core".to_string(),
    condition_format: "element_mastery_1000".to_string(),
    condition_operator_format: "greater_than_or_equal".to_string(),
};
```

### **2. Data Type Consistency**

```rust
// Consistent data types across all systems
pub struct StatusDataTypeConsistency {
    // Magnitude: f64 (0.0 to 1.0 for percentages, unlimited for absolute values)
    pub magnitude_type: String,
    
    // Duration: Duration (always in seconds)
    pub duration_type: String,
    
    // Target: String (actor_id, "Self", "Enemy", "Ally", etc.)
    pub target_type: String,
    
    // Source: String (system_name, skill_name, item_name, etc.)
    pub source_type: String,
    
    // Conditions: Vec<StatusCondition> (consistent condition structure)
    pub condition_type: String,
    
    // Interactions: Vec<StatusInteraction> (consistent interaction structure)
    pub interaction_type: String,
}
```

### **3. Priority Resolution**

```rust
// Consistent priority resolution across all systems
pub struct StatusPriorityResolution {
    // System priority (higher number = higher priority)
    pub system_priorities: HashMap<SourceSystem, u32>,
    
    // Effect type priority (higher number = higher priority)
    pub effect_type_priorities: HashMap<UnifiedStatusEffectType, u32>,
    
    // Resolution rules
    pub resolution_rules: Vec<ResolutionRule>,
}

pub struct ResolutionRule {
    pub rule_name: String,
    pub condition: ResolutionCondition,
    pub action: ResolutionAction,
}

pub enum ResolutionCondition {
    SameEffectType,
    SameTarget,
    SameSource,
    ConflictingEffects,
    Custom(String),
}

pub enum ResolutionAction {
    Replace,
    Merge,
    Stack,
    Ignore,
    Custom(String),
}
```

## 🧪 **Testing Strategy**

### **1. Consistency Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_status_naming_consistency() {
        let naming = StatusNamingConventions::new();
        
        // Test effect ID format
        let effect_id = naming.format_effect_id("elemental", "burning");
        assert_eq!(effect_id, "elemental_burning");
        
        // Test effect name format
        let effect_name = naming.format_effect_name("Burning");
        assert_eq!(effect_name, "Burning");
        
        // Test effect name VI format
        let effect_name_vi = naming.format_effect_name_vi("Cháy");
        assert_eq!(effect_name_vi, "Cháy");
    }
    
    #[test]
    fn test_status_data_type_consistency() {
        let consistency = StatusDataTypeConsistency::new();
        
        // Test magnitude type
        assert_eq!(consistency.magnitude_type, "f64");
        
        // Test duration type
        assert_eq!(consistency.duration_type, "Duration");
        
        // Test target type
        assert_eq!(consistency.target_type, "String");
    }
    
    #[test]
    fn test_status_priority_resolution() {
        let priority = StatusPriorityResolution::new();
        
        // Test system priorities
        assert!(priority.system_priorities[&SourceSystem::StatusCore] > 
                priority.system_priorities[&SourceSystem::ElementCore]);
        
        // Test effect type priorities
        assert!(priority.effect_type_priorities[&UnifiedStatusEffectType::Custom("Transformation".to_string())] >
                priority.effect_type_priorities[&UnifiedStatusEffectType::Elemental(ElementalStatusEffectType::Burning)]);
    }
}
```

### **2. Integration Tests**

```rust
#[tokio::test]
async fn test_crystal_defense_status_integration() {
    let unified_status_core = UnifiedStatusCore::new();
    let crystal_defense = CrystalDefenseTechnique::new();
    let mut actor = create_test_actor();
    
    // Apply crystallization with system integration
    crystal_defense.apply_crystallization_status(&mut actor, &mut unified_status_core).await?;
    
    // Test unified status effect
    let effects = unified_status_core.get_actor_status_effects(actor.get_id()).await?;
    assert!(effects.iter().any(|e| e.effect_id == "crystallization"));
    
    // Test system integration
    let crystallization_effect = effects.iter()
        .find(|e| e.effect_id == "crystallization")
        .unwrap();
    
    assert!(crystallization_effect.source_systems.contains(&SourceSystem::StatusCore));
    assert!(crystallization_effect.source_systems.contains(&SourceSystem::ElementCore));
    assert!(crystallization_effect.source_systems.contains(&SourceSystem::CombatCore));
    
    // Test priority resolution
    let priority = unified_status_core.get_effect_priority(crystallization_effect).await?;
    assert!(priority > 0);
}
```

## 📝 **Implementation Recommendations**

### **1. Centralized Management**
- **Status Core quản lý tất cả**: Status Core nên là hệ thống trung tâm quản lý tất cả status effects
- **System Integration**: Các hệ thống khác tích hợp với Status Core thông qua integration layers
- **Unified Registry**: Sử dụng unified registry để quản lý tất cả status effects

### **2. Consistency Enforcement**
- **Naming Conventions**: Áp dụng naming conventions nhất quán
- **Data Type Consistency**: Sử dụng data types nhất quán
- **Priority Resolution**: Áp dụng priority resolution rules nhất quán

### **3. Performance Optimization**
- **Caching**: Cache status effects để tối ưu performance
- **Batch Processing**: Xử lý status effects theo batch
- **Lazy Evaluation**: Tính toán status effects chỉ khi cần thiết

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
