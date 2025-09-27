# Element Core Overview

## 📋 **Overview**

Element Core is the central data hub for managing all elemental systems in the Chaos World MMORPG. It aggregates and caches elemental data from multiple sources (Race-Core, Item-Core, Skill-Core, etc.) while maintaining high performance and flexibility.

**Version**: 2.0  
**Last Updated**: 2024-12-19  
**Status**: Active

### **Key Features**
- **Data Hub Pattern**: Central aggregation and caching of elemental data
- **External Contributor Pattern**: Other systems contribute data through standardized interfaces
- **High Performance**: Optimized for high-frequency game scenarios
- **Unified Architecture**: Single, consistent approach across all elemental systems

## 🎯 **Design Principles**

### **1. Data Hub Pattern**
Element-Core acts as a **central data hub** that aggregates and caches elemental data from multiple sources, without containing business logic.

### **2. External Contributor Pattern**
Other systems (Race-Core, Item-Core, Skill-Core) contribute elemental data to Element-Core through standardized interfaces.

### **3. Single Responsibility**
Element-Core focuses solely on:
- Data aggregation and caching
- Registry management
- Performance optimization
- Basic element operations

### **4. Performance First**
All operations optimized for high-frequency game scenarios with minimal latency.

### **5. Unified Architecture**
Single, consistent approach across all elemental systems, eliminating multiple conflicting patterns.

## 🏗️ **Element Core Architecture**

### **Core Structure**

```rust
/// Element-Core: Central data hub
pub struct ElementCore {
    /// Unified registry for all element data
    registry: UnifiedElementRegistry,
    
    /// Aggregator for combining contributions
    aggregator: ElementAggregator,
    
    /// Cache for performance optimization
    cache: ElementCache,
    
    /// Configuration management
    config: ElementConfig,
}
```

### **Data Flow**

```
External Systems → Element-Core → Unified Registry
     ↓
Race-Core, Item-Core, Skill-Core contribute elemental data
     ↓
Element-Core aggregates and caches data
     ↓
Other systems consume aggregated data
### **External Contributor Pattern**

```rust
/// External system integration trait
pub trait ElementContributor: Send + Sync {
    /// System identifier
    fn system_id(&self) -> &str;
    
    /// Priority (higher = more important)
    fn priority(&self) -> i64;
    
    /// Contribute to element stats
    async fn contribute_element_stats(
        &self, 
        actor: &Actor, 
        element_type: &str
    ) -> ElementCoreResult<ElementContribution>;
    
    /// Handle element events
    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()>;
}
```

## 📊 **Element Types & Categories**

### **1. Basic Element Categories**

```rust
// Element Categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementCategory {
    // Physical Elements
    Physical,
    Slashing,
    Piercing,
    Blunt,
    Crushing,
    
    // Magical Elements
    Magical,
    Arcane,
    Mystical,
    Spiritual,
    Mental,
    
    // Natural Elements
    Fire,
    Water,
    Ice,
    Earth,
    Air,
    Lightning,
    Poison,
    Dark,
    Light,
    
    // Cultivation Elements
    Qi,
    Dao,
    Profound,
    Karma,
    Fate,
    
    // Special Elements
    True,
    Healing,
    Drain,
    Reflect,
    Absorb,
    Chaos,
    Reality,
    Conceptual,
}
```

### **2. Element Type Registry**

```rust
// Element Type Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementType {
    pub id: String,                    // Unique identifier
    pub name: String,                  // Display name
    pub category: ElementCategory,     // Element category
    pub description: String,           // Description
    pub derived_stats: Vec<DerivedStatType>, // Available derived stats
    pub is_active: bool,               // Is this element active
    pub created_at: i64,               // Creation timestamp
    pub updated_at: i64,               // Last update timestamp
}

// Derived Stat Types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DerivedStatType {
    // Power & Defense
    PowerPoint,           // Attack power
    DefensePoint,         // Defense value
    
    // Critical Stats
    CritRate,            // Critical hit rate
    CritDamage,          // Critical damage multiplier
    ResistCritRate,      // Resist critical hit rate
    ResistCritDamage,    // Resist critical damage
    
    // Accuracy Stats
    AccurateRate,        // Hit accuracy rate
    DodgeRate,           // Dodge rate
    
    // Custom Stats (extensible)
    Penetration,         // Armor penetration
    Absorption,          // Damage absorption
    Reflection,          // Damage reflection
    Conversion,          // Damage conversion
    Amplification,       // Damage amplification
    Reduction,           // Damage reduction
}
```

## 🔢 **Derived Stats System**

### **1. Stat Calculation Formula**

```rust
// Base calculation formula including Omni stats: (total_attacker_stat - total_defender_stat)
pub fn calculate_stat_difference(
    attacker_omni_stat: f64,
    attacker_element_stat: f64,
    defender_omni_stat: f64,
    defender_element_stat: f64,
) -> f64 {
    let total_attacker_stat = attacker_omni_stat + attacker_element_stat;
    let total_defender_stat = defender_omni_stat + defender_element_stat;
    total_attacker_stat - total_defender_stat
}

// Probability calculation for critical hits including Omni stats
pub fn calculate_crit_probability(
    attacker_omni_crit: f64,
    attacker_element_crit: f64,
    defender_omni_resist_crit: f64,
    defender_element_resist_crit: f64,
) -> f64 {
    let difference = calculate_stat_difference(
        attacker_omni_crit,
        attacker_element_crit,
        defender_omni_resist_crit,
        defender_element_resist_crit,
    );
    
    // Sigmoid function for smooth probability curve
    // Allows 100% crit when attacker >> defender
    // Allows 0% crit when defender >> attacker
    sigmoid(difference / CRIT_SCALING_FACTOR)
}

// Probability calculation for accuracy including Omni stats
pub fn calculate_accuracy_probability(
    attacker_omni_accuracy: f64,
    attacker_element_accuracy: f64,
    defender_omni_dodge: f64,
    defender_element_dodge: f64,
) -> f64 {
    let difference = calculate_stat_difference(
        attacker_omni_accuracy,
        attacker_element_accuracy,
        defender_omni_dodge,
        defender_element_dodge,
    );
    
    // Sigmoid function for smooth probability curve
    sigmoid(difference / ACCURACY_SCALING_FACTOR)
}

// Sigmoid function implementation
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
```

### **2. Scaling Factors**

```rust
// Scaling factors for different stat types
pub const CRIT_SCALING_FACTOR: f64 = 100.0;      // Critical hit scaling
pub const ACCURACY_SCALING_FACTOR: f64 = 80.0;    // Accuracy scaling
pub const DAMAGE_SCALING_FACTOR: f64 = 1.0;       // Damage scaling (1:1)
pub const PENETRATION_SCALING_FACTOR: f64 = 1.0;  // Penetration scaling (1:1)
```

## ⚔️ **Combat-Core Integration**

### **1. Hybrid Architecture**

Element-Core tích hợp với Combat-Core thông qua hybrid approach:

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

### **2. Element-Core Responsibilities**

#### **Stats Management**
- **Omni Stats**: Cung cấp baseline protection cho tất cả elements
- **Element Stats**: Cung cấp specialized stats cho từng element
- **Combined Stats**: Kết hợp Omni + Element stats cho Combat-Core

#### **Element Interactions**
- **Tương Sinh Tương Khắc**: Tính damage multipliers
- **Special Interactions**: Xử lý các tương tác đặc biệt
- **Element Fusion**: Hỗ trợ element combination

#### **Status Effects**
- **Status Probability**: Tính xác suất gây status effect
- **Status Resistance**: Tính kháng status effect
- **Status Application**: Áp dụng status effects

### **3. Combat-Core Responsibilities**

#### **Damage Calculation**
- **Base Damage**: Tính base damage từ action
- **Power vs Defense**: Sử dụng stats từ Element-Core
- **Multipliers**: Áp dụng các multipliers
- **Critical Hits**: Xử lý critical hit mechanics

#### **Combat Mechanics**
- **Action Processing**: Xử lý các action trong combat
- **Event Handling**: Quản lý combat events
- **Resource Management**: Quản lý tài nguyên combat

### **4. Integration Benefits**

#### **Performance**
- **Element-Core Caching**: Cache element calculations
- **Combat-Core Caching**: Cache combat calculations
- **Minimal Overhead**: Chỉ pass data, không duplicate calculations

#### **Maintainability**
- **Clear Separation**: Mỗi core có trách nhiệm rõ ràng
- **Independent Evolution**: Có thể phát triển độc lập
- **Easy Testing**: Test từng component riêng biệt

#### **Flexibility**
- **Multi-System Support**: Element-Core có thể được sử dụng bởi nhiều systems
- **Easy Extension**: Dễ dàng thêm element types mới
- **Configuration-Driven**: Dễ dàng điều chỉnh thông qua config

## 🎮 **Multi-System Integration**

### **1. System-Specific Element Usage**

```rust
// System-specific element configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemElementConfig {
    pub system_id: String,                    // System identifier
    pub element_type: String,                 // Element type
    pub primary_stats: Vec<DerivedStatType>,  // Primary stats for this system
    pub power_scale: f64,                     // Power scaling factor
    pub stat_weights: HashMap<DerivedStatType, f64>, // Stat weights
    pub is_enabled: bool,                     // Is this system enabled
}

// Example: Combat Core using Fire element
let combat_fire_config = SystemElementConfig {
    system_id: "combat_core".to_string(),
    element_type: "fire".to_string(),
    primary_stats: vec![
        DerivedStatType::PowerPoint,
        DerivedStatType::DefensePoint,
        DerivedStatType::CritRate,
        DerivedStatType::CritDamage,
    ],
    power_scale: 1.0,
    stat_weights: HashMap::from([
        (DerivedStatType::PowerPoint, 1.0),
        (DerivedStatType::DefensePoint, 0.8),
        (DerivedStatType::CritRate, 0.3),
        (DerivedStatType::CritDamage, 0.5),
    ]),
    is_enabled: true,
};

// Example: Shield System using Fire element
let shield_fire_config = SystemElementConfig {
    system_id: "shield_system".to_string(),
    element_type: "fire".to_string(),
    primary_stats: vec![
        DerivedStatType::DefensePoint,
        DerivedStatType::Absorption,
        DerivedStatType::Reflection,
    ],
    power_scale: 0.7, // Shields are 70% as effective as offense
    stat_weights: HashMap::from([
        (DerivedStatType::DefensePoint, 1.0),
        (DerivedStatType::Absorption, 0.6),
        (DerivedStatType::Reflection, 0.4),
    ]),
    is_enabled: true,
};
```

### **2. Element Registry Management**

```rust
// Element Registry
pub struct ElementRegistry {
    elements: HashMap<String, ElementType>,
    system_configs: HashMap<String, Vec<SystemElementConfig>>,
    derived_stats_cache: HashMap<String, Vec<DerivedStatType>>,
}

impl ElementRegistry {
    // Register new element type
    pub fn register_element(&mut self, element: ElementType) -> Result<(), ElementError> {
        // Validate element
        self.validate_element(&element)?;
        
        // Add to registry
        self.elements.insert(element.id.clone(), element);
        
        // Update cache
        self.update_derived_stats_cache();
        
        Ok(())
    }
    
    // Get element by ID
    pub fn get_element(&self, element_id: &str) -> Option<&ElementType> {
        self.elements.get(element_id)
    }
    
    // Get all elements in category
    pub fn get_elements_by_category(&self, category: &ElementCategory) -> Vec<&ElementType> {
        self.elements.values()
            .filter(|e| e.category == *category)
            .collect()
    }
    
    // Get derived stats for element
    pub fn get_derived_stats(&self, element_id: &str) -> Option<&Vec<DerivedStatType>> {
        self.derived_stats_cache.get(element_id)
    }
}
```

## 🎯 **Condition Core Integration**

### **Element Data Provider**

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

### **Standardized Element Conditions**

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

### **Cross-System Condition Reuse**

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

// Shield System using Element Core conditions
impl ShieldSystem {
    pub async fn can_activate_water_shield(&self, actor_id: &str) -> Result<bool, ShieldError> {
        let condition = ConditionConfig {
            condition_id: "can_activate_water_shield".to_string(),
            function_name: "has_element_affinity".to_string(),
            operator: ConditionOperator::Equal,
            value: ConditionValue::Boolean(true),
            parameters: vec![ConditionParameter::String("water".to_string())],
        };
        
        let context = self.create_condition_context(actor_id).await?;
        self.condition_resolver.resolve_condition(&condition, &context).await
    }
}
```

### **Benefits of Condition Core Integration**

#### **1. Standardized Condition Logic**
- **Unified Functions**: Tất cả systems sử dụng cùng element condition functions
- **Consistent Behavior**: Hành vi nhất quán cho element conditions
- **Centralized Management**: Condition logic được quản lý tập trung

#### **2. Cross-System Reuse**
- **Shared Conditions**: Element conditions có thể được tái sử dụng across systems
- **Easy Integration**: Systems dễ dàng tích hợp element conditions
- **Reduced Duplication**: Giảm code duplication cho element conditions

#### **3. Performance Benefits**
- **Centralized Caching**: Element conditions được cache tập trung
- **Batch Evaluation**: Có thể evaluate nhiều element conditions cùng lúc
- **Optimized Queries**: Tối ưu queries cho element data

## 🧮 **Calculation Engine**

### **1. Stat Comparison Engine**

```rust
// Stat comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatComparisonResult {
    pub stat_type: DerivedStatType,
    pub attacker_value: f64,
    pub defender_value: f64,
    pub difference: f64,
    pub probability: Option<f64>, // Only for probability-based stats
    pub final_value: f64,
}

// Stat comparison engine
pub struct StatComparisonEngine {
    scaling_factors: HashMap<DerivedStatType, f64>,
    probability_formulas: HashMap<DerivedStatType, ProbabilityFormula>,
}

impl StatComparisonEngine {
    // Compare stats between attacker and defender
    pub fn compare_stats(
        &self,
        attacker_stats: &HashMap<DerivedStatType, f64>,
        defender_stats: &HashMap<DerivedStatType, f64>,
        stat_type: &DerivedStatType,
    ) -> StatComparisonResult {
        let attacker_value = attacker_stats.get(stat_type).copied().unwrap_or(0.0);
        let defender_value = defender_stats.get(stat_type).copied().unwrap_or(0.0);
        let difference = calculate_stat_difference(attacker_value, defender_value);
        
        // Calculate probability if this is a probability-based stat
        let probability = if self.is_probability_stat(stat_type) {
            Some(self.calculate_probability(stat_type, difference))
        } else {
            None
        };
        
        // Calculate final value
        let final_value = if let Some(prob) = probability {
            // For probability stats, return the probability
            prob
        } else {
            // For flat stats, return the difference
            difference
        };
        
        StatComparisonResult {
            stat_type: stat_type.clone(),
            attacker_value,
            defender_value,
            difference,
            probability,
            final_value,
        }
    }
    
    // Check if stat is probability-based
    fn is_probability_stat(&self, stat_type: &DerivedStatType) -> bool {
        matches!(
            stat_type,
            DerivedStatType::CritRate
                | DerivedStatType::ResistCritRate
                | DerivedStatType::AccurateRate
                | DerivedStatType::DodgeRate
        )
    }
    
    // Calculate probability using appropriate formula
    fn calculate_probability(&self, stat_type: &DerivedStatType, difference: f64) -> f64 {
        let scaling_factor = self.scaling_factors.get(stat_type).copied().unwrap_or(1.0);
        sigmoid(difference / scaling_factor)
    }
}
```

### **2. Multi-System Aggregation**

```rust
// Multi-system aggregation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSystemAggregationResult {
    pub element_type: String,
    pub system_results: HashMap<String, HashMap<DerivedStatType, f64>>,
    pub final_stats: HashMap<DerivedStatType, f64>,
    pub total_power: f64,
    pub total_defense: f64,
}

// Multi-system aggregation engine
pub struct MultiSystemAggregationEngine {
    registry: Arc<ElementRegistry>,
    comparison_engine: StatComparisonEngine,
}

impl MultiSystemAggregationEngine {
    // Aggregate stats from multiple systems
    pub fn aggregate_systems(
        &self,
        element_type: &str,
        system_stats: &HashMap<String, HashMap<DerivedStatType, f64>>,
    ) -> Result<MultiSystemAggregationResult, ElementError> {
        let element = self.registry.get_element(element_type)
            .ok_or(ElementError::ElementNotFound(element_type.to_string()))?;
        
        let mut system_results = HashMap::new();
        let mut final_stats = HashMap::new();
        
        // Process each system
        for (system_id, stats) in system_stats {
            let system_config = self.get_system_config(system_id, element_type)?;
            let mut system_result = HashMap::new();
            
            // Calculate stats for this system
            for stat_type in &system_config.primary_stats {
                if let Some(&value) = stats.get(stat_type) {
                    let scaled_value = value * system_config.power_scale;
                    let weighted_value = scaled_value * system_config.stat_weights.get(stat_type).copied().unwrap_or(1.0);
                    
                    system_result.insert(stat_type.clone(), weighted_value);
                    
                    // Aggregate into final stats
                    let current_value = final_stats.get(stat_type).copied().unwrap_or(0.0);
                    final_stats.insert(stat_type.clone(), current_value + weighted_value);
                }
            }
            
            system_results.insert(system_id.clone(), system_result);
        }
        
        // Calculate totals
        let total_power = final_stats.get(&DerivedStatType::PowerPoint).copied().unwrap_or(0.0);
        let total_defense = final_stats.get(&DerivedStatType::DefensePoint).copied().unwrap_or(0.0);
        
        Ok(MultiSystemAggregationResult {
            element_type: element_type.to_string(),
            system_results,
            final_stats,
            total_power,
            total_defense,
        })
    }
}
```

## 🔧 **Configuration System**

### **1. Element Configuration**

```yaml
# element_types.yaml
version: 1
elements:
  - id: "fire"
    name: "Fire"
    category: "fire"
    description: "Element of flame and heat"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "crit_rate"
      - "crit_damage"
      - "resist_crit_rate"
      - "resist_crit_damage"
      - "accurate_rate"
      - "dodge_rate"
      - "penetration"
      - "absorption"
    is_active: true
    
  - id: "water"
    name: "Water"
    category: "water"
    description: "Element of fluidity and healing"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "crit_rate"
      - "crit_damage"
      - "resist_crit_rate"
      - "resist_crit_damage"
      - "accurate_rate"
      - "dodge_rate"
      - "penetration"
      - "absorption"
      - "conversion"
    is_active: true
```

### **2. System Configuration**

```yaml
# system_configs.yaml
version: 1
systems:
  - system_id: "combat_core"
    elements:
      - element_type: "fire"
        primary_stats:
          - "power_point"
          - "defense_point"
          - "crit_rate"
          - "crit_damage"
        power_scale: 1.0
        stat_weights:
          power_point: 1.0
          defense_point: 0.8
          crit_rate: 0.3
          crit_damage: 0.5
        is_enabled: true
        
  - system_id: "shield_system"
    elements:
      - element_type: "fire"
        primary_stats:
          - "defense_point"
          - "absorption"
          - "reflection"
        power_scale: 0.7
        stat_weights:
          defense_point: 1.0
          absorption: 0.6
          reflection: 0.4
        is_enabled: true
```

## 🚀 **Integration Examples**

### **1. Combat Core Integration**

```rust
// Combat Core using Element Core
pub struct CombatElementIntegration {
    element_registry: Arc<ElementRegistry>,
    aggregation_engine: MultiSystemAggregationEngine,
}

impl CombatElementIntegration {
    // Calculate damage using element system
    pub fn calculate_element_damage(
        &self,
        attacker: &Actor,
        target: &Actor,
        element_type: &str,
        base_damage: f64,
    ) -> Result<f64, ElementError> {
        // Get element stats from both actors
        let attacker_stats = self.get_actor_element_stats(attacker, element_type)?;
        let target_stats = self.get_actor_element_stats(target, element_type)?;
        
        // Compare power vs defense
        let power_result = self.aggregation_engine.comparison_engine.compare_stats(
            &attacker_stats,
            &target_stats,
            &DerivedStatType::PowerPoint,
        );
        
        let defense_result = self.aggregation_engine.comparison_engine.compare_stats(
            &attacker_stats,
            &target_stats,
            &DerivedStatType::DefensePoint,
        );
        
        // Calculate final damage
        let power_damage = power_result.final_value - defense_result.final_value;
        let final_damage = base_damage + power_damage.max(0.0);
        
        Ok(final_damage)
    }
    
    // Check critical hit
    pub fn check_critical_hit(
        &self,
        attacker: &Actor,
        target: &Actor,
        element_type: &str,
    ) -> Result<bool, ElementError> {
        let attacker_stats = self.get_actor_element_stats(attacker, element_type)?;
        let target_stats = self.get_actor_element_stats(target, element_type)?;
        
        let crit_result = self.aggregation_engine.comparison_engine.compare_stats(
            &attacker_stats,
            &target_stats,
            &DerivedStatType::CritRate,
        );
        
        // Use probability to determine if critical hit occurs
        let random_value = rand::thread_rng().gen::<f64>();
        Ok(random_value < crit_result.final_value)
    }
}
```

### **2. Shield System Integration**

```rust
// Shield System using Element Core
pub struct ShieldElementIntegration {
    element_registry: Arc<ElementRegistry>,
    aggregation_engine: MultiSystemAggregationEngine,
}

impl ShieldElementIntegration {
    // Calculate shield absorption
    pub fn calculate_shield_absorption(
        &self,
        shield: &Shield,
        incoming_damage: f64,
        element_type: &str,
    ) -> Result<f64, ElementError> {
        let shield_stats = self.get_shield_element_stats(shield, element_type)?;
        
        // Get absorption value
        let absorption = shield_stats.get(&DerivedStatType::Absorption).copied().unwrap_or(0.0);
        
        // Calculate absorbed damage
        let absorbed_damage = (incoming_damage * absorption).min(incoming_damage);
        
        Ok(absorbed_damage)
    }
    
    // Calculate shield reflection
    pub fn calculate_shield_reflection(
        &self,
        shield: &Shield,
        incoming_damage: f64,
        element_type: &str,
    ) -> Result<f64, ElementError> {
        let shield_stats = self.get_shield_element_stats(shield, element_type)?;
        
        // Get reflection value
        let reflection = shield_stats.get(&DerivedStatType::Reflection).copied().unwrap_or(0.0);
        
        // Calculate reflected damage
        let reflected_damage = incoming_damage * reflection;
        
        Ok(reflected_damage)
    }
}
```

## 📊 **Performance Considerations**

### **1. Caching Strategy**

```rust
// Element stats cache
pub struct ElementStatsCache {
    actor_stats_cache: HashMap<String, HashMap<String, HashMap<DerivedStatType, f64>>>,
    system_aggregation_cache: HashMap<String, MultiSystemAggregationResult>,
    cache_ttl: Duration,
    last_update: HashMap<String, Instant>,
}

impl ElementStatsCache {
    // Get cached actor stats
    pub fn get_actor_stats(
        &self,
        actor_id: &str,
        element_type: &str,
    ) -> Option<&HashMap<DerivedStatType, f64>> {
        self.actor_stats_cache
            .get(actor_id)?
            .get(element_type)
    }
    
    // Cache actor stats
    pub fn cache_actor_stats(
        &mut self,
        actor_id: String,
        element_type: String,
        stats: HashMap<DerivedStatType, f64>,
    ) {
        self.actor_stats_cache
            .entry(actor_id)
            .or_insert_with(HashMap::new)
            .insert(element_type, stats);
    }
    
    // Check if cache is valid
    pub fn is_cache_valid(&self, key: &str) -> bool {
        if let Some(last_update) = self.last_update.get(key) {
            last_update.elapsed() < self.cache_ttl
        } else {
            false
        }
    }
}
```

### **2. Batch Processing**

```rust
// Batch processing for multiple elements
pub struct ElementBatchProcessor {
    registry: Arc<ElementRegistry>,
    aggregation_engine: MultiSystemAggregationEngine,
    cache: ElementStatsCache,
}

impl ElementBatchProcessor {
    // Process multiple elements at once
    pub fn process_elements_batch(
        &mut self,
        requests: Vec<ElementProcessingRequest>,
    ) -> Result<Vec<ElementProcessingResult>, ElementError> {
        let mut results = Vec::new();
        
        // Group by element type for efficient processing
        let mut grouped_requests: HashMap<String, Vec<ElementProcessingRequest>> = HashMap::new();
        for request in requests {
            grouped_requests.entry(request.element_type.clone()).or_insert_with(Vec::new).push(request);
        }
        
        // Process each element type
        for (element_type, requests) in grouped_requests {
            let element_results = self.process_element_type_batch(&element_type, requests)?;
            results.extend(element_results);
        }
        
        Ok(results)
    }
}
```

## 🧪 **Testing Strategy**

### **1. Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_element_registration() {
        let mut registry = ElementRegistry::new();
        
        let fire_element = ElementType {
            id: "fire".to_string(),
            name: "Fire".to_string(),
            category: ElementCategory::Fire,
            description: "Element of flame".to_string(),
            derived_stats: vec![
                DerivedStatType::PowerPoint,
                DerivedStatType::DefensePoint,
            ],
            is_active: true,
            created_at: 0,
            updated_at: 0,
        };
        
        assert!(registry.register_element(fire_element).is_ok());
        assert!(registry.get_element("fire").is_some());
    }
    
    #[test]
    fn test_stat_comparison() {
        let engine = StatComparisonEngine::new();
        
        let mut attacker_stats = HashMap::new();
        attacker_stats.insert(DerivedStatType::PowerPoint, 100.0);
        
        let mut defender_stats = HashMap::new();
        defender_stats.insert(DerivedStatType::DefensePoint, 80.0);
        
        let result = engine.compare_stats(
            &attacker_stats,
            &defender_stats,
            &DerivedStatType::PowerPoint,
        );
        
        assert_eq!(result.attacker_value, 100.0);
        assert_eq!(result.defender_value, 0.0);
        assert_eq!(result.difference, 100.0);
        assert_eq!(result.final_value, 100.0);
    }
    
    #[test]
    fn test_probability_calculation() {
        let engine = StatComparisonEngine::new();
        
        let mut attacker_stats = HashMap::new();
        attacker_stats.insert(DerivedStatType::CritRate, 150.0);
        
        let mut defender_stats = HashMap::new();
        defender_stats.insert(DerivedStatType::ResistCritRate, 50.0);
        
        let result = engine.compare_stats(
            &attacker_stats,
            &defender_stats,
            &DerivedStatType::CritRate,
        );
        
        assert!(result.probability.is_some());
        let prob = result.probability.unwrap();
        assert!(prob > 0.5); // Should be high probability
        assert!(prob < 1.0); // But not 100%
    }
}
```

### **2. Integration Tests**

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_combat_integration() {
        let registry = Arc::new(ElementRegistry::new());
        let aggregation_engine = MultiSystemAggregationEngine::new(registry.clone());
        let combat_integration = CombatElementIntegration::new(registry, aggregation_engine);
        
        // Test damage calculation
        let attacker = create_test_actor();
        let target = create_test_target();
        
        let damage = combat_integration.calculate_element_damage(
            &attacker,
            &target,
            "fire",
            100.0,
        ).unwrap();
        
        assert!(damage > 0.0);
    }
    
    #[test]
    fn test_multi_system_aggregation() {
        let registry = Arc::new(ElementRegistry::new());
        let aggregation_engine = MultiSystemAggregationEngine::new(registry);
        
        let mut system_stats = HashMap::new();
        let mut combat_stats = HashMap::new();
        combat_stats.insert(DerivedStatType::PowerPoint, 100.0);
        system_stats.insert("combat_core".to_string(), combat_stats);
        
        let result = aggregation_engine.aggregate_systems("fire", &system_stats).unwrap();
        
        assert_eq!(result.total_power, 100.0);
    }
}
```

## 🚀 **Usage Examples**

### **Basic Element Stats Retrieval**

```rust
// Create Element-Core
let mut element_core = ElementCore::new();

// Register external contributors
element_core.register_contributor(Arc::new(RaceCoreElementContributor::new())).await?;
element_core.register_contributor(Arc::new(ItemCoreElementContributor::new())).await?;
element_core.register_contributor(Arc::new(SkillCoreElementContributor::new())).await?;

// Get element stats for actor
let actor = Actor::new("player_1".to_string(), "human".to_string());
let fire_stats = element_core.get_element_stats(&actor, "fire").await?;

println!("Fire Power: {}", fire_stats.power);
println!("Fire Defense: {}", fire_stats.defense);
println!("Fire Affinity: {}", fire_stats.affinity);
```

### **Element Interaction Calculation**

```rust
// Calculate element interaction factor
let interaction_factor = element_core.get_interaction_factor("fire", "water");

// Use in combat calculation
let base_damage = 100.0;
let final_damage = base_damage * interaction_factor;
```

## ⚖️ **Balance Considerations**

### **Performance vs Features**
- **Performance**: Optimized for high-frequency operations
- **Features**: Comprehensive elemental system capabilities

### **Simplicity vs Power**
- **Simplicity**: Clear, understandable architecture
- **Power**: Rich elemental interactions and mechanics

### **Maintenance vs Extensibility**
- **Maintenance**: Easy to update and extend
- **Extensibility**: Support for new systems and elements

## 📚 **Related Documents**

- [Unified Architecture Design](20_Unified_Architecture_Design.md) - Target architecture
- [Migration Guide](21_Migration_Guide.md) - Migration from old architecture
- [Element System Architecture](01_Element_System_Architecture.md) - Basic architecture
- [Element Registry Design](04_Element_Registry_Design.md) - Registry implementation
- [Universal Element Registry Design](18_Universal_Element_Registry_Design.md) - Advanced registry features
- [Stats Distribution Design](19_Stats_Distribution_Design.md) - External system integration

## 🔄 **Evolution Strategy**

### **Version 2.0 (Current)**
- Unified architecture implementation
- External contributor pattern
- Performance optimization

### **Version 3.0 (Future)**
- Advanced caching strategies
- Machine learning integration
- Enhanced performance monitoring

### **Version 4.0 (Future)**
- AI-powered optimization
- Predictive caching
- Advanced analytics

---

**Last Updated**: 2024-12-19  
**Version**: 2.0  
**Status**: Active  
**Next Review**: 2024-12-26
