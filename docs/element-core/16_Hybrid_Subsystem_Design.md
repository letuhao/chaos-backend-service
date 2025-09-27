# Hybrid Subsystem Design (Hệ Lai)

## 📋 **Tổng Quan**

Hybrid Subsystem hỗ trợ các nguyên tố lai (kết hợp hai hay nhiều yếu tố/khía cạnh), ví dụ: "Mộc Thần Lôi" (Lightning + Wood + Holy/Exorcism). Hệ thống giữ nguyên tắc no hard caps + yin–yang counterbalance, sử dụng dynamics + refractory, và nhất quán với Actor Core ModifierPack system.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Consistency với Actor Core**
- **Reuse ModifierPack**: Sử dụng existing ModifierPack system từ Actor Core
- **Extend, Don't Replace**: Mở rộng thay vì tạo mới
- **Same Patterns**: Follow existing design patterns

### **2. Configuration-Driven**
- **Tag Detection**: Load từ config thay vì hard-code
- **Modifier Rules**: Define trong YAML configs
- **Flexible Activation**: Conditional activation based on items/talents

### **3. No Hard Caps**
- **Dynamics-based**: Sử dụng dynamics thay vì hard limits
- **Refractory System**: Cooldown mechanisms
- **Yin-Yang Balance**: Counterbalance effects

## 🏗️ **Kiến Trúc Hệ Thống**

### **1. Core Components**

```rust
// Hybrid Element Structure
pub struct HybridElement {
    pub id: String,                    // "lightning_divine_wood"
    pub name: String,                  // "Mộc Thần Lôi"
    pub parents: Vec<String>,          // ["lightning", "wood"]
    pub tags: Vec<String>,             // ["holy", "exorcism"]
    pub activation: HybridActivation,  // Điều kiện kích hoạt
    pub modifiers: ModifierPack,       // ✅ Reuse Actor Core ModifierPack
}

// Activation Requirements
pub struct HybridActivation {
    pub requires_any: Vec<ActivationRequirement>,
}

pub enum ActivationRequirement {
    Item { item_id: String },
    CultivationTalent { talent_id: String },
    Skill { skill_id: String },
    Level { min_level: u32 },
    // Có thể thêm conditions khác
}
```

### **2. Extended ModifierPack**

```rust
// Extend existing Actor Core ModifierPack
pub struct ModifierPack {
    /// Additive percentage modifiers (existing)
    pub additive_percent: HashMap<String, f64>,
    /// Multiplicative modifiers (existing)
    pub multipliers: HashMap<String, f64>,
    /// Post-additive modifiers (existing)
    pub post_add: HashMap<String, f64>,
    /// NEW: Conditional modifiers
    pub conditional: HashMap<String, ConditionalModifier>,
    /// NEW: Tag-based modifiers
    pub tag_based: HashMap<String, TagBasedModifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalModifier {
    pub condition: String,             // "target.hp < 0.5"
    pub then: ModifierPack,            // Apply if condition is true
    pub else: Option<ModifierPack>,    // Apply if condition is false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagBasedModifier {
    pub tag_pattern: String,           // "undead|ghost|corpse"
    pub modifiers: ModifierPack,       // Modifiers to apply
    pub priority: u32,                 // Higher priority overrides lower
}
```

### **3. Tag Detection System**

```rust
// Configuration-driven tag detection
pub struct TagDetector {
    tag_definitions: HashMap<String, TagDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDefinition {
    pub conditions: Vec<TagCondition>,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCondition {
    pub field: String,                 // "race", "equipment.weapon"
    pub operator: TagOperator,         // "in", "equals", "contains"
    pub values: Vec<String>,           // ["undead", "zombie"]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagOperator {
    Equals,
    NotEquals,
    In,
    NotIn,
    Contains,
    NotContains,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}
```

## 🔧 **Configuration System**

### **1. Tag Detection Configuration**

```yaml
# configs/tag_detection.yaml
tag_definitions:
  undead:
    conditions:
      - field: "race"
        operator: "in"
        values: ["undead", "zombie", "skeleton"]
      - field: "cultivation.realm"
        operator: "contains"
        values: ["corruption", "necromancy"]
    priority: 100
    
  ghost:
    conditions:
      - field: "race"
        operator: "equals"
        values: ["ghost", "spirit", "wraith"]
      - field: "state"
        operator: "equals"
        values: ["ethereal", "incorporeal"]
    priority: 90
    
  holy:
    conditions:
      - field: "equipment.weapon"
        operator: "contains"
        values: ["holy", "blessed", "divine"]
      - field: "cultivation.realm"
        operator: "contains"
        values: ["divine", "light", "purification"]
    priority: 80
    
  life:
    conditions:
      - field: "race"
        operator: "in"
        values: ["human", "elf", "dwarf", "halfling"]
      - field: "state"
        operator: "not_equals"
        values: ["undead", "ghost"]
    priority: 70
```

### **2. Hybrid Element Configuration**

```yaml
# hybrid/configs/lightning_divine_wood.yaml
hybrid:
  id: "lightning_divine_wood"
  name: "Mộc Thần Lôi"
  parents: ["lightning", "wood"]
  tags: ["holy", "exorcism"]
  description: "Divine wood-aspected lightning for exorcism and banishment"
  
  activation:
    requires_any:
      - item: "kim_lei_truc"
      - cultivation_talent: "divine_thunder_wood"
      - skill: "divine_lightning_mastery"
      - level: 50
  
  modifiers:
    # Base modifiers (always applied)
    additive_percent:
      "divine_damage": 15.0
      "exorcism_power": 25.0
      "nature_harmony": 10.0
    multipliers:
      "lightning_damage": 1.3
      "wood_damage": 1.2
      "holy_effectiveness": 1.5
    post_add:
      "holy_bonus": 50.0
      "purification_power": 25.0
    
    # Tag-based modifiers
    tag_based:
      "undead":
        additive_percent:
          "damage_amplification": 60.0
          "purge_effectiveness": 40.0
        multipliers:
          "execution_damage": 2.0
          "corruption_removal": 3.0
        post_add:
          "undead_slayer": 100.0
      "ghost":
        additive_percent:
          "spirit_damage": 40.0
          "banishment_power": 35.0
        multipliers:
          "ethereal_penetration": 1.8
          "spirit_binding": 2.2
        post_add:
          "ghost_banisher": 75.0
      "corpse":
        additive_percent:
          "necromancy_damage": 50.0
        multipliers:
          "decay_acceleration": 1.6
        post_add:
          "corpse_purifier": 50.0
      "life":
        additive_percent:
          "healing_bonus": 20.0
          "nature_blessing": 15.0
        multipliers:
          "life_enhancement": 1.5
          "growth_acceleration": 1.3
        post_add:
          "life_guardian": 30.0
    
    # Conditional modifiers
    conditional:
      "low_health_target":
        condition: "target.hp < 0.3"
        then:
          multipliers:
            "execution_damage": 2.0
          post_add:
            "finishing_blow": 100.0
        else:
          multipliers:
            "normal_damage": 1.0
      "holy_weapon_equipped":
        condition: "attacker.equipment.weapon.contains('holy')"
        then:
          additive_percent:
            "weapon_synergy": 30.0
          multipliers:
            "divine_channeling": 1.4
        else:
          additive_percent:
            "basic_holy": 10.0
      "corrupted_environment":
        condition: "environment.corruption_level > 0.7"
        then:
          multipliers:
            "purification_power": 1.8
          post_add:
            "environment_cleansing": 50.0
        else:
          multipliers:
            "normal_purification": 1.0

references:
  tag_detection_config: "../../configs/tag_detection.yaml"
  probability_config_path: "../../configs/probability_config.yaml"
  interaction_config_path: "../../configs/interaction_config.yaml"
  status_pool_path: "../../configs/status_pool.yaml"
  dynamics_design: "../../11_Advanced_Derived_Stats_Design.md"
```

### **3. Status Pool Integration**

```yaml
# configs/status_pool.yaml (schema khớp codebase hiện tại)
effects:
  divine_conduit_aspect:
    category: "aspect"
    applies_to: ["attacker"]
    description: "Grants divine lightning aspect with wood bias"
    grants_tags: ["lightning_divine", "holy", "wood_bias"]
    dynamics: { intensity_gain: 0.0, intensity_damping: 0.0, decay_rate: 0.0, refractory_gain: 0.0, refractory_decay: 0.0 }

  divine_shock:
    category: "hybrid_lightning"
    applies_to: ["defender"]
    description: "Shocked variant with exorcism scaling vs undead/ghost/corpse"
    status_attach:
      element: "lightning"
      element_status_id: "shocked"
      probability:
        base: "from_element"
        use_probability_engine: true
        scaling_factor_key: "status_probability"
    dynamics: { intensity_gain: 0.018, intensity_damping: 0.013, decay_rate: 0.05, refractory_gain: 0.5, refractory_decay: 0.14 }
    category_mods:
      defender:
        undead|ghost|corpse:
          intensity_gain_mod: 0.012
          damage_taken_amp_add: 0.06
          purge_corruption: true
        life:
          intensity_gain_mod: -0.004
```

### **4. Telemetry**
- Log cho mỗi lần áp dụng hybrid: `(hybrid_id, parents, target_tags, Δ, I, R, p, dt)`
- Đánh dấu event type: `HybridEffectApplied` để tiện truy vết cân bằng.

## 🔄 **Implementation Details**

### **1. Hybrid Element Manager**

```rust
pub struct HybridElementManager {
    hybrid_elements: HashMap<String, HybridElement>,
    tag_detector: TagDetector,
    activation_cache: HashMap<String, bool>, // Cache activation status
}

impl HybridElementManager {
    // Load hybrid configuration
    pub async fn load_hybrid_config(&mut self, config_path: &Path) -> Result<(), ConfigError> {
        let content = tokio::fs::read_to_string(config_path).await?;
        let config: HybridConfig = serde_yaml::from_str(&content)?;
        
        let hybrid_element = HybridElement {
            id: config.hybrid.id,
            name: config.hybrid.name,
            parents: config.hybrid.parents,
            tags: config.hybrid.tags,
            activation: config.hybrid.activation,
            modifiers: config.hybrid.modifiers,
        };
        
        self.hybrid_elements.insert(hybrid_element.id.clone(), hybrid_element);
        Ok(())
    }
    
    // Check if hybrid is activated for actor
    pub async fn is_hybrid_activated(&self, actor: &Actor, hybrid_id: &str) -> bool {
        if let Some(hybrid) = self.hybrid_elements.get(hybrid_id) {
            for requirement in &hybrid.activation.requires_any {
                match requirement {
                    ActivationRequirement::Item { item_id } => {
                        if actor.has_item(item_id) {
                            return true;
                        }
                    },
                    ActivationRequirement::CultivationTalent { talent_id } => {
                        if actor.has_cultivation_talent(talent_id) {
                            return true;
                        }
                    },
                    ActivationRequirement::Skill { skill_id } => {
                        if actor.has_skill(skill_id) {
                            return true;
                        }
                    },
                    ActivationRequirement::Level { min_level } => {
                        if actor.level >= *min_level {
                            return true;
                        }
                    },
                }
            }
        }
        false
    }
    
    // Get active hybrids for actor
    pub async fn get_active_hybrids(&self, actor: &Actor) -> Vec<&HybridElement> {
        let mut active_hybrids = Vec::new();
        
        for hybrid in self.hybrid_elements.values() {
            if self.is_hybrid_activated(actor, &hybrid.id).await {
                active_hybrids.push(hybrid);
            }
        }
        
        active_hybrids
    }
    
    // Calculate hybrid effects
    pub async fn calculate_hybrid_effects(&self, actor: &Actor, target: &Actor) -> HashMap<String, f64> {
        let mut effects = HashMap::new();
        
        // Get active hybrids
        let active_hybrids = self.get_active_hybrids(actor).await;
        
        // Detect target tags
        let target_tags = self.tag_detector.detect_tags(target);
        
        for hybrid in active_hybrids {
            // Calculate base effects
            let base_effects = self.calculate_base_effects(hybrid, actor, target).await;
            effects.extend(base_effects);
            
            // Apply tag-based modifiers
            let tag_effects = self.apply_tag_modifiers(hybrid, &target_tags, actor, target).await;
            effects.extend(tag_effects);
            
            // Apply conditional modifiers
            let conditional_effects = self.apply_conditional_modifiers(hybrid, actor, target).await;
            effects.extend(conditional_effects);
        }
        
        effects
    }
}
```

### **2. Tag Detection Implementation**

```rust
impl TagDetector {
    // Load tag definitions from config
    pub async fn load_from_config(&mut self, config_path: &Path) -> Result<(), ConfigError> {
        let content = tokio::fs::read_to_string(config_path).await?;
        let config: TagDetectionConfig = serde_yaml::from_str(&content)?;
        
        for (tag_id, definition) in config.tag_definitions {
            self.tag_definitions.insert(tag_id, definition);
        }
        
        Ok(())
    }
    
    // Detect tags for an actor
    pub fn detect_tags(&self, actor: &Actor) -> Vec<String> {
        let mut detected_tags = Vec::new();
        
        for (tag_id, definition) in &self.tag_definitions {
            if self.check_tag_conditions(actor, definition) {
                detected_tags.push(tag_id.clone());
            }
        }
        
        // Sort by priority (higher first)
        detected_tags.sort_by_key(|tag| {
            self.tag_definitions.get(tag).map(|d| d.priority).unwrap_or(0)
        });
        
        detected_tags
    }
    
    // Check if actor matches tag conditions
    fn check_tag_conditions(&self, actor: &Actor, definition: &TagDefinition) -> bool {
        definition.conditions.iter().all(|condition| {
            self.evaluate_condition(actor, condition)
        })
    }
    
    // Evaluate single condition
    fn evaluate_condition(&self, actor: &Actor, condition: &TagCondition) -> bool {
        let field_value = self.get_field_value(actor, &condition.field);
        
        match condition.operator {
            TagOperator::Equals => condition.values.contains(&field_value),
            TagOperator::NotEquals => !condition.values.contains(&field_value),
            TagOperator::In => condition.values.contains(&field_value),
            TagOperator::NotIn => !condition.values.contains(&field_value),
            TagOperator::Contains => condition.values.iter().any(|v| field_value.contains(v)),
            TagOperator::NotContains => !condition.values.iter().any(|v| field_value.contains(v)),
            TagOperator::GreaterThan => {
                if let (Ok(field_num), Ok(condition_num)) = (field_value.parse::<f64>(), condition.values[0].parse::<f64>()) {
                    field_num > condition_num
                } else {
                    false
                }
            },
            TagOperator::LessThan => {
                if let (Ok(field_num), Ok(condition_num)) = (field_value.parse::<f64>(), condition.values[0].parse::<f64>()) {
                    field_num < condition_num
                } else {
                    false
                }
            },
            TagOperator::GreaterThanOrEqual => {
                if let (Ok(field_num), Ok(condition_num)) = (field_value.parse::<f64>(), condition.values[0].parse::<f64>()) {
                    field_num >= condition_num
                } else {
                    false
                }
            },
            TagOperator::LessThanOrEqual => {
                if let (Ok(field_num), Ok(condition_num)) = (field_value.parse::<f64>(), condition.values[0].parse::<f64>()) {
                    field_num <= condition_num
                } else {
                    false
                }
            },
        }
    }
    
    // Get field value from actor
    fn get_field_value(&self, actor: &Actor, field_path: &str) -> String {
        let parts: Vec<&str> = field_path.split('.').collect();
        match parts[0] {
            "race" => actor.race.clone(),
            "level" => actor.level.to_string(),
            "equipment" => {
                if parts.len() > 1 {
                    match parts[1] {
                        "weapon" => actor.get_equipment("weapon").unwrap_or_default(),
                        "armor" => actor.get_equipment("armor").unwrap_or_default(),
                        _ => String::new(),
                    }
                } else {
                    String::new()
                }
            },
            "cultivation" => {
                if parts.len() > 1 {
                    match parts[1] {
                        "realm" => actor.get_cultivation_realm().unwrap_or_default(),
                        "level" => actor.get_cultivation_level().to_string(),
                        _ => String::new(),
                    }
                } else {
                    String::new()
                }
            },
            "state" => actor.get_state().unwrap_or_default(),
            _ => String::new(),
        }
    }
}
```

### **3. SystemResourceCalculator Integration**

```rust
pub struct HybridElementResourceManager {
    hybrid_manager: HybridElementManager,
    system_id: String,
}

#[async_trait]
impl SystemResourceCalculator for HybridElementResourceManager {
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut resources = HashMap::new();
        
        // Get active hybrids
        let active_hybrids = self.hybrid_manager.get_active_hybrids(actor).await;
        
        for hybrid in active_hybrids {
            // Calculate hybrid-specific resources
            let hybrid_resources = self.calculate_hybrid_resources(actor, hybrid).await?;
            resources.extend(hybrid_resources);
        }
        
        Ok(resources)
    }
    
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn affects_resource(&self, resource_id: &str) -> bool {
        resource_id.starts_with("hybrid_")
    }
    
    async fn notify_stat_change(&self, actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()> {
        // Check if stat changes affect hybrid activation
        for stat in changed_stats {
            if self.affects_hybrid_activation(stat) {
                // Trigger hybrid recalculation
                self.hybrid_manager.invalidate_activation_cache(actor.id.to_string()).await;
            }
        }
        Ok(())
    }
    
    fn get_resource_dependencies(&self) -> Vec<String> {
        vec![
            "level".to_string(),
            "equipment".to_string(),
            "cultivation".to_string(),
            "skills".to_string(),
        ]
    }
    
    fn get_resource_categories(&self) -> Vec<ResourceCategory> {
        vec![
            ResourceCategory::Special,
            ResourceCategory::Cultivation,
        ]
    }
    
    async fn is_active(&self, actor: &Actor) -> ActorCoreResult<bool> {
        // Check if actor has any active hybrids
        let active_hybrids = self.hybrid_manager.get_active_hybrids(actor).await;
        Ok(!active_hybrids.is_empty())
    }
}
```

## 🧪 **Testing & Validation**

### **1. Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    async fn test_hybrid_activation() {
        let mut manager = HybridElementManager::new();
        manager.load_hybrid_config("configs/lightning_divine_wood.yaml").await.unwrap();
        
        let mut actor = Actor::new("Test Actor".to_string(), "Human".to_string());
        actor.add_item("kim_lei_truc");
        
        assert!(manager.is_hybrid_activated(&actor, "lightning_divine_wood").await);
    }
    
    #[test]
    async fn test_tag_detection() {
        let mut detector = TagDetector::new();
        detector.load_from_config("configs/tag_detection.yaml").await.unwrap();
        
        let actor = Actor::new("Undead Warrior".to_string(), "undead".to_string());
        let tags = detector.detect_tags(&actor);
        
        assert!(tags.contains(&"undead".to_string()));
    }
    
    #[test]
    async fn test_modifier_application() {
        let hybrid = HybridElement {
            id: "test_hybrid".to_string(),
            name: "Test Hybrid".to_string(),
            parents: vec!["fire".to_string(), "water".to_string()],
            tags: vec!["test".to_string()],
            activation: HybridActivation { requires_any: vec![] },
            modifiers: ModifierPack {
                additive_percent: HashMap::from([("damage".to_string(), 20.0)]),
                multipliers: HashMap::from([("power".to_string(), 1.5)]),
                post_add: HashMap::from([("bonus".to_string(), 100.0)]),
                conditional: HashMap::new(),
                tag_based: HashMap::new(),
            },
        };
        
        let base_value = 100.0;
        let modified_value = hybrid.modifiers.apply("damage", base_value);
        
        // 100 + (100 * 0.2) = 120
        // 120 * 1.5 = 180
        // 180 + 100 = 280
        assert_eq!(modified_value, 280.0);
    }
}
```

### **2. Integration Tests**

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_hybrid_system_integration() {
        let mut manager = HybridElementManager::new();
        manager.load_hybrid_config("configs/lightning_divine_wood.yaml").await.unwrap();
        
        let mut actor = Actor::new("Divine Warrior".to_string(), "Human".to_string());
        actor.add_item("kim_lei_truc");
        actor.level = 60;
        
        let target = Actor::new("Undead Enemy".to_string(), "undead".to_string());
        
        let effects = manager.calculate_hybrid_effects(&actor, &target).await;
        
        // Verify hybrid effects are applied
        assert!(effects.contains_key("divine_damage"));
        assert!(effects.contains_key("exorcism_power"));
        assert!(effects.contains_key("undead_slayer"));
    }
}
```

## 🚀 **Implementation Strategy**

### **Phase 1: Core Infrastructure (2-3 weeks)**
1. **Extend ModifierPack** trong Actor Core
2. **TagDetector** system với config loading
3. **HybridElement** struct và basic parsing

### **Phase 2: Hybrid System (2-3 weeks)**
1. **HybridElementManager** với activation logic
2. **Modifier application** system
3. **YAML configuration** parsing

### **Phase 3: Integration (1-2 weeks)**
1. **SystemResourceCalculator** implementation
2. **Actor Core integration**
3. **Status Pool integration**

### **Phase 4: Testing & Polish (1-2 weeks)**
1. **Unit tests** cho hybrid logic
2. **Integration tests** với Element Core
3. **Performance optimization**

## 💡 **Lợi Ích của Hybrid System**

### **1. Consistency**
- **Reuse Actor Core**: Sử dụng existing ModifierPack
- **Same Patterns**: Follow established patterns
- **Unified API**: Consistent interface

### **2. Flexibility**
- **Configuration-driven**: Easy to modify without code changes
- **Tag-based Effects**: Dynamic effects based on target properties
- **Conditional Logic**: Complex conditional modifiers

### **3. Extensibility**
- **Easy to Add**: Chỉ cần thêm YAML config
- **Plugin System**: Modifier types có thể được extend
- **Parent Elements**: Reuse existing element logic

### **4. Performance**
- **Conditional Loading**: Chỉ load khi cần
- **Caching**: Cache activation status và calculations
- **Lazy Evaluation**: Chỉ calculate khi cần

## 📚 **Related Documents**

- [00_Element_Core_Overview.md](./00_Element_Core_Overview.md) - Element Core overview
- [11_Advanced_Derived_Stats_Design.md](./11_Advanced_Derived_Stats_Design.md) - Dynamics system
- [14_Reuse_Analysis_Actor_Core_Resource_Manager.md](./14_Reuse_Analysis_Actor_Core_Resource_Manager.md) - Actor Core integration
- [17_Elemental_Category_System_Design.md](./17_Elemental_Category_System_Design.md) - Element categorization system
- [18_Universal_Element_Registry_Design.md](./18_Universal_Element_Registry_Design.md) - Universal element registry
- [19_Stats_Distribution_Design.md](./19_Stats_Distribution_Design.md) - External system integration
- [configs/tag_detection.yaml](./configs/tag_detection.yaml) - Tag detection configuration
- [hybrid/configs/lightning_divine_wood.yaml](./hybrid/configs/lightning_divine_wood.yaml) - Example hybrid config

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
