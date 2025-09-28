# Comprehensive Implementation Plan

## 📋 **Overview**

This document provides a comprehensive implementation plan for Element-Core based on deep analysis of all documentation in the element-core folder. It consolidates requirements, specifications, and implementation details from all design documents.

**Version**: 1.0  
**Last Updated**: 2024-12-19  
**Status**: Active

---

## 🎯 **Implementation Strategy**

### **Phase-Based Approach**
1. **Phase 1**: Core Infrastructure (Weeks 1-2)
2. **Phase 2**: Advanced Features (Weeks 3-4)  
3. **Phase 3**: Integration & Testing (Weeks 5-6)
4. **Phase 4**: Performance & Optimization (Week 7)

---

## 📚 **Document Analysis Summary**

### **Core Architecture Documents**
- **00_Element_Core_Overview.md**: Data hub pattern, external contributor pattern
- **01_Element_System_Architecture.md**: Core components, system integration
- **04_Element_Registry_Design.md**: Unified registry, element definitions
- **20_Unified_Architecture_Design.md**: Consolidated architecture approach

### **Feature-Specific Documents**
- **08_Elemental_Mastery_System_Design.md**: Plugin-based mastery system
- **10_Element_Interaction_System_Design.md**: Tương sinh tương khắc mechanics
- **11_Advanced_Derived_Stats_Design.md**: 50+ derived stats system
- **12_Performance_Optimization_Design.md**: Array-based performance optimization

### **Integration Documents**
- **06_Implementation_Notes.md**: Omni element, status effects, combat integration
- **09_Actor_Core_Integration_Guide.md**: Actor-Core integration patterns
- **13_Integration_Impact_Analysis.md**: SystemContribution removal impact

### **Configuration Documents**
- **configs/**: YAML configuration files for elements, interactions, probabilities
- **elements/configs/**: Individual element YAML configurations

---

## 🏗️ **Phase 1: Core Infrastructure (Weeks 1-2)**

### **1.1 Element Contributor System** - 🔴 **CRITICAL**

#### **ElementContributor Trait**
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

#### **ElementContribution Struct**
```rust
/// Standardized contribution format
pub struct ElementContribution {
    /// System that contributed
    pub system_id: String,
    
    /// Element type
    pub element_type: String,
    
    /// Stat contributions
    pub stat_contributions: HashMap<String, f64>,
    
    /// Priority weight
    pub priority: i64,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}
```

#### **ElementContributorRegistry**
```rust
/// Registry for external contributors
pub struct ElementContributorRegistry {
    /// Registered contributors
    contributors: HashMap<String, Arc<dyn ElementContributor>>,
    
    /// Contributor metadata
    metadata: HashMap<String, ContributorMetadata>,
    
    /// Registration order
    registration_order: Vec<String>,
}
```

### **1.2 Unified Element Registry** - 🔴 **CRITICAL**

#### **Core Registry Structure**
```rust
/// Unified Element Registry - Single source of truth
pub struct UnifiedElementRegistry {
    /// Core element definitions
    elements: HashMap<String, ElementDefinition>,
    
    /// System registrations
    system_registrations: HashMap<String, SystemRegistration>,
    
    /// External contributors
    contributors: HashMap<String, Arc<dyn ElementContributor>>,
    
    /// Category management
    categories: HashMap<String, ElementalCategory>,
    
    /// Plugin management
    plugins: HashMap<String, Arc<dyn ElementPlugin>>,
    
    /// Interaction matrix
    interaction_matrix: HashMap<(String, String), ElementInteraction>,
    
    /// Configuration
    config: RegistryConfig,
    
    /// Performance metrics
    metrics: RegistryMetrics,
}
```

#### **Element Definition**
```rust
/// Core element definition
pub struct ElementDefinition {
    /// Unique identifier
    pub id: String,
    
    /// Display name
    pub name: String,
    
    /// Element description
    pub description: String,
    
    /// Element category
    pub category: ElementCategory,
    
    /// Base properties
    pub base_properties: ElementProperties,
    
    /// Derived stats configuration
    pub derived_stats: Vec<DerivedStatConfig>,
    
    /// Status effects
    pub status_effects: Vec<StatusEffectConfig>,
    
    /// Environment modifications
    pub environment_mods: HashMap<String, EnvironmentMod>,
    
    /// References to other configs
    pub references: ElementReferences,
}
```

### **1.3 YAML Configuration Loading** - 🟡 **PARTIAL**

#### **Configuration Loader**
```rust
/// Element configuration loader
pub struct ElementConfigLoader {
    /// Base configuration directory
    config_dir: PathBuf,
    
    /// Loaded configurations cache
    config_cache: HashMap<String, ElementConfig>,
    
    /// Validation rules
    validation_rules: Vec<ConfigValidationRule>,
}

impl ElementConfigLoader {
    /// Load element configuration from YAML
    pub fn load_element_config(&self, element_id: &str) -> ElementCoreResult<ElementConfig> {
        let file_path = self.config_dir.join(format!("{}_element.yaml", element_id));
        let content = fs::read_to_string(&file_path)?;
        let config: ElementConfig = serde_yaml::from_str(&content)?;
        self.validate_config(&config)?;
        Ok(config)
    }
    
    /// Load all element configurations
    pub fn load_all_configs(&self) -> ElementCoreResult<HashMap<String, ElementConfig>> {
        let mut configs = HashMap::new();
        for element_id in self.get_available_elements()? {
            let config = self.load_element_config(&element_id)?;
            configs.insert(element_id, config);
        }
        Ok(configs)
    }
}
```

### **1.4 Element Aggregator** - 🔴 **CRITICAL**

#### **ElementAggregator**
```rust
/// Aggregator for combining contributions from multiple systems
pub struct ElementAggregator {
    /// Aggregation strategies
    strategies: HashMap<String, AggregationStrategy>,
    
    /// Cache for aggregated results
    cache: ElementCache,
    
    /// Performance metrics
    metrics: AggregatorMetrics,
}

impl ElementAggregator {
    /// Aggregate contributions from all registered systems
    pub async fn aggregate_contributions(
        &self,
        actor: &Actor,
        element_type: &str
    ) -> ElementCoreResult<ElementStats> {
        // 1. Collect contributions from all systems
        let mut contributions = Vec::new();
        for contributor in self.get_contributors() {
            let contribution = contributor.contribute_element_stats(actor, element_type).await?;
            contributions.push(contribution);
        }
        
        // 2. Sort by priority
        contributions.sort_by_key(|c| c.priority);
        
        // 3. Apply aggregation strategies
        let final_stats = self.apply_aggregation_strategies(contributions).await?;
        
        Ok(final_stats)
    }
}
```

---

## 🚀 **Phase 2: Advanced Features (Weeks 3-4)**

### **2.1 Derived Stats Calculation** - 🟡 **PARTIAL**

#### **50+ Derived Stats Implementation**
```rust
/// Derived stats calculator
pub struct DerivedStatsCalculator {
    /// Calculation formulas
    formulas: HashMap<String, CalculationFormula>,
    
    /// Performance cache
    cache: DerivedStatsCache,
    
    /// Validation rules
    validation_rules: Vec<ValidationRule>,
}

impl DerivedStatsCalculator {
    /// Calculate all derived stats for an element
    pub fn calculate_derived_stats(
        &self,
        element_data: &ElementalSystemData,
        element_config: &ElementConfig
    ) -> ElementCoreResult<DerivedStats> {
        let mut derived_stats = DerivedStats::new();
        
        // Core Element Mastery
        derived_stats.element_mastery = self.calculate_element_mastery(element_data)?;
        
        // Counterbalance Pairs
        derived_stats.power_point = self.calculate_power_point(element_data, element_config)?;
        derived_stats.defense_point = self.calculate_defense_point(element_data, element_config)?;
        derived_stats.crit_rate = self.calculate_crit_rate(element_data, element_config)?;
        derived_stats.resist_crit_rate = self.calculate_resist_crit_rate(element_data, element_config)?;
        
        // Status Effect Stats
        derived_stats.status_probability = self.calculate_status_probability(element_data, element_config)?;
        derived_stats.status_resistance = self.calculate_status_resistance(element_data, element_config)?;
        derived_stats.status_duration = self.calculate_status_duration(element_data, element_config)?;
        derived_stats.status_duration_reduction = self.calculate_status_duration_reduction(element_data, element_config)?;
        derived_stats.status_intensity = self.calculate_status_intensity(element_data, element_config)?;
        derived_stats.status_intensity_reduction = self.calculate_status_intensity_reduction(element_data, element_config)?;
        
        // Element Interaction Stats
        derived_stats.element_penetration = self.calculate_element_penetration(element_data, element_config)?;
        derived_stats.element_absorption = self.calculate_element_absorption(element_data, element_config)?;
        derived_stats.element_amplification = self.calculate_element_amplification(element_data, element_config)?;
        derived_stats.element_reduction = self.calculate_element_reduction(element_data, element_config)?;
        
        // Reflection Stats
        derived_stats.reflection_rate = self.calculate_reflection_rate(element_data, element_config)?;
        derived_stats.resist_reflection_rate = self.calculate_resist_reflection_rate(element_data, element_config)?;
        derived_stats.reflection_damage = self.calculate_reflection_damage(element_data, element_config)?;
        derived_stats.resist_reflection_damage = self.calculate_resist_reflection_damage(element_data, element_config)?;
        
        // Advanced Combat Mechanics
        derived_stats.parry_rate = self.calculate_parry_rate(element_data, element_config)?;
        derived_stats.parry_break = self.calculate_parry_break(element_data, element_config)?;
        derived_stats.parry_strength = self.calculate_parry_strength(element_data, element_config)?;
        derived_stats.parry_shred = self.calculate_parry_shred(element_data, element_config)?;
        
        derived_stats.block_rate = self.calculate_block_rate(element_data, element_config)?;
        derived_stats.block_break = self.calculate_block_break(element_data, element_config)?;
        derived_stats.block_strength = self.calculate_block_strength(element_data, element_config)?;
        derived_stats.block_shred = self.calculate_block_shred(element_data, element_config)?;
        
        // Skill Effectiveness
        derived_stats.attack_skill_effectiveness = self.calculate_attack_skill_effectiveness(element_data, element_config)?;
        derived_stats.defense_skill_effectiveness = self.calculate_defense_skill_effectiveness(element_data, element_config)?;
        derived_stats.status_skill_effectiveness = self.calculate_status_skill_effectiveness(element_data, element_config)?;
        derived_stats.movement_technique_effectiveness = self.calculate_movement_technique_effectiveness(element_data, element_config)?;
        derived_stats.healing_skill_effectiveness = self.calculate_healing_skill_effectiveness(element_data, element_config)?;
        derived_stats.support_skill_effectiveness = self.calculate_support_skill_effectiveness(element_data, element_config)?;
        derived_stats.utility_skill_effectiveness = self.calculate_utility_skill_effectiveness(element_data, element_config)?;
        derived_stats.skill_effectiveness = self.calculate_skill_effectiveness(element_data, element_config)?;
        
        // Social & Economy
        derived_stats.element_leadership_bonus = self.calculate_element_leadership_bonus(element_data, element_config)?;
        derived_stats.element_teaching_efficiency = self.calculate_element_teaching_efficiency(element_data, element_config)?;
        derived_stats.element_crafting_efficiency = self.calculate_element_crafting_efficiency(element_data, element_config)?;
        derived_stats.element_resource_discovery = self.calculate_element_resource_discovery(element_data, element_config)?;
        
        // Perception & Detection
        derived_stats.element_sensitivity = self.calculate_element_sensitivity(element_data, element_config)?;
        
        // Advanced Combat Mechanics
        derived_stats.mastery_synergy_bonus = self.calculate_mastery_synergy_bonus(element_data, element_config)?;
        
        Ok(derived_stats)
    }
}
```

### **2.2 Element Interaction Matrix** - 🟡 **PARTIAL**

#### **Interaction Matrix Implementation**
```rust
/// Element interaction matrix
pub struct ElementInteractionMatrix {
    /// Interaction relationships
    relationships: HashMap<(String, String), ElementRelationship>,
    
    /// Interaction effects
    effects: HashMap<String, InteractionEffect>,
    
    /// Dynamics configuration
    dynamics: InteractionDynamics,
    
    /// Performance cache
    cache: InteractionCache,
}

impl ElementInteractionMatrix {
    /// Calculate interaction factor between two elements
    pub fn calculate_interaction_factor(
        &self,
        attacker_element: &str,
        defender_element: &str,
        attacker_mastery: f64,
        defender_mastery: f64
    ) -> ElementCoreResult<f64> {
        // 1. Get base relationship
        let relationship = self.get_relationship(attacker_element, defender_element)?;
        
        // 2. Calculate mastery difference
        let mastery_diff = attacker_mastery - defender_mastery;
        
        // 3. Apply sigmoid function
        let base_trigger = relationship.base_trigger;
        let normalized_diff = mastery_diff / self.dynamics.trigger_scale;
        let trigger = base_trigger + sigmoid(normalized_diff, self.dynamics.steepness);
        
        // 4. Clamp to valid range
        Ok(trigger.clamp(0.0, 1.0))
    }
    
    /// Get element relationship
    pub fn get_relationship(&self, element1: &str, element2: &str) -> ElementCoreResult<ElementRelationship> {
        let key = (element1.to_string(), element2.to_string());
        self.relationships.get(&key)
            .cloned()
            .ok_or_else(|| ElementCoreError::ElementNotFound { element_id: format!("{}-{}", element1, element2) })
    }
}
```

### **2.3 Elemental Mastery System** - 🟡 **PARTIAL**

#### **Plugin-Based Mastery System**
```rust
/// Element plugin trait
pub trait ElementPlugin: Send + Sync {
    /// Get element identifier
    fn get_element_id(&self) -> String;
    
    /// Get element definition
    fn get_element_definition(&self) -> ElementDefinition;
    
    /// Calculate base mastery
    fn calculate_base_mastery(&self, actor: &Actor) -> f64;
    
    /// Calculate decay rate
    fn calculate_decay_rate(&self, actor: &Actor) -> f64;
    
    /// Get opposite elements
    fn get_opposite_elements(&self) -> Vec<String>;
    
    /// Handle training
    fn handle_training(&self, actor: &mut Actor, training_amount: f64) -> ElementCoreResult<()>;
    
    /// Get derived stats for this element
    fn get_derived_stats(&self, actor: &Actor) -> HashMap<String, f64>;
    
    /// Get training methods for this element
    fn get_training_methods(&self) -> Vec<TrainingMethod>;
    
    /// Get element interactions
    fn get_element_interactions(&self) -> HashMap<String, ElementInteraction>;
}
```

#### **Element Plugin Registry**
```rust
/// Element plugin registry
pub struct ElementPluginRegistry {
    /// Registered element plugins
    plugins: HashMap<String, Arc<dyn ElementPlugin>>,
    
    /// Element definitions cache
    element_definitions: HashMap<String, ElementDefinition>,
    
    /// Element interactions matrix
    element_interactions: HashMap<(String, String), ElementInteraction>,
    
    /// Plugin metadata
    plugin_metadata: HashMap<String, PluginMetadata>,
}
```

### **2.4 Status Effect System** - 🟡 **PARTIAL**

#### **Status Effect Configuration**
```rust
/// Status effect configuration
pub struct StatusEffectConfig {
    /// Effect name
    pub name: String,
    
    /// Effect type
    pub effect_type: String,
    
    /// Base probability
    pub base_probability: f64,
    
    /// Base duration
    pub base_duration: f64,
    
    /// Base intensity
    pub base_intensity: f64,
    
    /// Tick interval
    pub tick_interval: f64,
    
    /// Maximum stacks
    pub max_stacks: u32,
    
    /// Stackable
    pub stackable: bool,
    
    /// Refresh duration
    pub refresh_duration: bool,
    
    /// Spread rules
    pub spread_rules: Option<SpreadRules>,
    
    /// Effects
    pub effects: Option<HashMap<String, f64>>,
    
    /// HP heal per tick
    pub hp_heal_per_tick: Option<f64>,
    
    /// Stamina heal per tick
    pub stamina_heal_per_tick: Option<f64>,
    
    /// Dynamics
    pub dynamics: StatusDynamics,
}
```

#### **Status Dynamics**
```rust
/// Status effect dynamics
pub struct StatusDynamics {
    /// Intensity gain rate
    pub intensity_gain: f64,
    
    /// Intensity damping rate
    pub intensity_damping: f64,
    
    /// Decay rate
    pub decay_rate: f64,
    
    /// Refractory gain
    pub refractory_gain: f64,
    
    /// Refractory decay
    pub refractory_decay: f64,
}
```

---

## 🔧 **Phase 3: Integration & Testing (Weeks 5-6)**

### **3.1 Actor-Core-Hierarchical Integration**

#### **Integration Points**
```rust
/// Actor-Core-Hierarchical integration
pub struct ActorCoreIntegration {
    /// Element core reference
    element_core: Arc<ElementCore>,
    
    /// Integration cache
    integration_cache: HashMap<String, IntegrationData>,
    
    /// Performance metrics
    metrics: IntegrationMetrics,
}

impl ActorCoreIntegration {
    /// Get element stats for actor
    pub async fn get_element_stats(
        &self,
        actor: &HierarchicalActor,
        element_type: &str
    ) -> ElementCoreResult<ElementStats> {
        // 1. Convert HierarchicalActor to Actor
        let actor_data = self.convert_hierarchical_actor(actor)?;
        
        // 2. Get element stats from Element-Core
        let element_stats = self.element_core.get_element_stats(&actor_data, element_type).await?;
        
        // 3. Cache result
        self.cache_result(actor.get_id(), element_type, &element_stats).await?;
        
        Ok(element_stats)
    }
}
```

### **3.2 External System Integration**

#### **Race-Core Integration Example**
```rust
/// Race-Core element contributor
pub struct RaceCoreElementContributor {
    /// System identifier
    system_id: String,
    
    /// Priority
    priority: i64,
    
    /// Race data
    race_data: Arc<RaceData>,
}

impl ElementContributor for RaceCoreElementContributor {
    async fn contribute_element_stats(
        &self,
        actor: &Actor,
        element_type: &str
    ) -> ElementCoreResult<ElementContribution> {
        // 1. Get race data
        let race = actor.get_race()?;
        let race_config = self.race_data.get_race_config(&race)?;
        
        // 2. Calculate race-based elemental bonuses
        let mut stat_contributions = HashMap::new();
        
        // Fire element bonus for Fire Spirit race
        if element_type == "fire" && race == "fire_spirit" {
            stat_contributions.insert("fire_power".to_string(), 200.0);
            stat_contributions.insert("fire_mastery".to_string(), 1.5);
        }
        
        // 3. Create contribution
        Ok(ElementContribution {
            system_id: self.system_id.clone(),
            element_type: element_type.to_string(),
            stat_contributions,
            priority: self.priority,
            timestamp: Utc::now(),
        })
    }
}
```

#### **Item-Core Integration Example**
```rust
/// Item-Core element contributor
pub struct ItemCoreElementContributor {
    /// System identifier
    system_id: String,
    
    /// Priority
    priority: i64,
    
    /// Item data
    item_data: Arc<ItemData>,
}

impl ElementContributor for ItemCoreElementContributor {
    async fn contribute_element_stats(
        &self,
        actor: &Actor,
        element_type: &str
    ) -> ElementCoreResult<ElementContribution> {
        // 1. Get equipped items
        let equipped_items = actor.get_equipped_items()?;
        
        // 2. Calculate item-based elemental bonuses
        let mut stat_contributions = HashMap::new();
        
        for item in equipped_items {
            let item_config = self.item_data.get_item_config(&item.id)?;
            
            // Fire sword bonus
            if item_config.element_type == Some(element_type.to_string()) {
                if let Some(power_bonus) = item_config.element_power_bonus {
                    stat_contributions.insert(
                        format!("{}_power", element_type),
                        power_bonus
                    );
                }
            }
        }
        
        // 3. Create contribution
        Ok(ElementContribution {
            system_id: self.system_id.clone(),
            element_type: element_type.to_string(),
            stat_contributions,
            priority: self.priority,
            timestamp: Utc::now(),
        })
    }
}
```

### **3.3 Testing Framework**

#### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_element_contributor_system() {
        // Test ElementContributor trait implementation
        let contributor = RaceCoreElementContributor::new();
        let actor = create_test_actor();
        
        let contribution = contributor.contribute_element_stats(&actor, "fire").await.unwrap();
        assert_eq!(contribution.system_id, "race_core");
        assert_eq!(contribution.element_type, "fire");
    }
    
    #[tokio::test]
    async fn test_unified_element_registry() {
        // Test UnifiedElementRegistry functionality
        let mut registry = UnifiedElementRegistry::new();
        let element_config = load_test_element_config("fire");
        
        registry.register_element(element_config).unwrap();
        let element = registry.get_element("fire").unwrap();
        assert_eq!(element.id, "fire");
    }
    
    #[tokio::test]
    async fn test_derived_stats_calculation() {
        // Test derived stats calculation
        let calculator = DerivedStatsCalculator::new();
        let element_data = create_test_element_data();
        let element_config = load_test_element_config("fire");
        
        let derived_stats = calculator.calculate_derived_stats(&element_data, &element_config).unwrap();
        assert!(derived_stats.power_point > 0.0);
        assert!(derived_stats.defense_point > 0.0);
    }
}
```

#### **Integration Tests**
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_actor_core_integration() {
        // Test Actor-Core-Hierarchical integration
        let element_core = ElementCore::new();
        let integration = ActorCoreIntegration::new(element_core);
        let actor = create_test_hierarchical_actor();
        
        let element_stats = integration.get_element_stats(&actor, "fire").await.unwrap();
        assert!(element_stats.power > 0.0);
    }
    
    #[tokio::test]
    async fn test_external_system_integration() {
        // Test external system integration
        let mut element_core = ElementCore::new();
        let race_contributor = RaceCoreElementContributor::new();
        let item_contributor = ItemCoreElementContributor::new();
        
        element_core.register_contributor(race_contributor).await.unwrap();
        element_core.register_contributor(item_contributor).await.unwrap();
        
        let actor = create_test_actor();
        let element_stats = element_core.get_element_stats(&actor, "fire").await.unwrap();
        
        // Should include contributions from both systems
        assert!(element_stats.power > 0.0);
    }
}
```

---

## ⚡ **Phase 4: Performance & Optimization (Week 7)**

### **4.1 Performance Optimization**

#### **Array-Based Data Structures**
```rust
/// Optimized elemental system data
pub struct ElementalSystemData {
    /// Primary stats - direct array access (1-2 ns)
    pub element_mastery_levels: [f64; MAX_ELEMENTS],
    pub element_qi_amounts: [f64; MAX_ELEMENTS],
    
    /// Derived stats - direct array access (1-2 ns)
    pub power_point: [f64; MAX_ELEMENTS],
    pub defense_point: [f64; MAX_ELEMENTS],
    
    /// Interaction matrix - 2D array for O(1) access
    pub element_interaction_bonuses: [[f64; MAX_ELEMENTS]; MAX_ELEMENTS],
    
    /// Feature flags - 2D boolean array
    pub feature_flags: [[bool; 16]; MAX_ELEMENTS],
}
```

#### **Caching Strategy**
```rust
/// Multi-level cache for performance
pub struct ElementCache {
    /// L1 Cache - Hot data (frequently accessed)
    l1_cache: HashMap<String, CachedElementData>,
    
    /// L2 Cache - Warm data (occasionally accessed)
    l2_cache: HashMap<String, CachedElementData>,
    
    /// L3 Cache - Cold data (rarely accessed)
    l3_cache: HashMap<String, CachedElementData>,
    
    /// Cache statistics
    hit_rates: CacheHitRates,
    eviction_policy: EvictionPolicy,
}
```

#### **SIMD Operations**
```rust
/// SIMD-optimized batch calculation
impl ElementalSystem {
    pub fn simd_calculate_power_points(&self, elements: &[usize]) -> Vec<f64> {
        let mut results = Vec::with_capacity(elements.len());
        
        // Process 4 elements at a time using SIMD
        for chunk in elements.chunks(4) {
            if chunk.len() == 4 {
                // Load 4 mastery levels
                let mastery_levels = unsafe {
                    _mm256_loadu_pd(&self.data.element_mastery_levels[chunk[0]] as *const f64)
                };
                
                // Load 4 base damages
                let base_damages = unsafe {
                    _mm256_loadu_pd(&self.data.base_damage[chunk[0]] as *const f64)
                };
                
                // Calculate power points: mastery * base_damage
                let power_points = unsafe {
                    _mm256_mul_pd(mastery_levels, base_damages)
                };
                
                // Store results
                let mut result_array = [0.0f64; 4];
                unsafe {
                    _mm256_storeu_pd(result_array.as_mut_ptr(), power_points);
                }
                
                results.extend_from_slice(&result_array);
            }
        }
        
        results
    }
}
```

### **4.2 Performance Monitoring**

#### **Performance Metrics**
```rust
/// Performance metrics collection
pub struct PerformanceMetrics {
    /// Access time metrics
    pub average_access_time: f64,
    pub p99_access_time: f64,
    pub p999_access_time: f64,
    
    /// Throughput metrics
    pub operations_per_second: f64,
    pub peak_throughput: f64,
    
    /// Memory metrics
    pub memory_usage: usize,
    pub cache_hit_rate: f64,
    
    /// Error metrics
    pub error_rate: f64,
    pub timeout_rate: f64,
}
```

---

## 📋 **Implementation Checklist**

### **Phase 1: Core Infrastructure**
- [ ] **ElementContributor trait** - Define external system interface
- [ ] **ElementContribution struct** - Standardized contribution format
- [ ] **ElementContributorRegistry** - Manage external contributors
- [ ] **UnifiedElementRegistry** - Single source of truth for element data
- [ ] **ElementDefinition struct** - Core element definition
- [ ] **YAML configuration loading** - Load real element configs
- [ ] **ElementAggregator** - Combine contributions from multiple systems
- [ ] **Basic error handling** - ElementCoreError types

### **Phase 2: Advanced Features**
- [ ] **DerivedStatsCalculator** - Implement 50+ derived stats
- [ ] **ElementInteractionMatrix** - Tương sinh tương khắc logic
- [ ] **ElementPlugin trait** - Plugin-based mastery system
- [ ] **ElementPluginRegistry** - Manage element plugins
- [ ] **StatusEffectConfig** - Status effect configuration
- [ ] **StatusDynamics** - Status effect dynamics
- [ ] **Adapters module** - Serialization and validation
- [ ] **Feature flags system** - Conditional compilation

### **Phase 3: Integration & Testing**
- [ ] **ActorCoreIntegration** - Actor-Core-Hierarchical integration
- [ ] **RaceCoreElementContributor** - Race-Core integration example
- [ ] **ItemCoreElementContributor** - Item-Core integration example
- [ ] **Unit tests** - Core functionality testing
- [ ] **Integration tests** - System integration testing
- [ ] **Performance tests** - Benchmark validation
- [ ] **Documentation updates** - Implementation guides

### **Phase 4: Performance & Optimization**
- [ ] **Array-based data structures** - Optimized for 1-2 ns access
- [ ] **Multi-level caching** - L1/L2/L3 cache strategy
- [ ] **SIMD operations** - Vectorized calculations
- [ ] **Performance monitoring** - Metrics collection
- [ ] **Memory optimization** - Minimal memory footprint
- [ ] **Hot path optimization** - Game performance targets

---

## 🎯 **Success Criteria**

### **Functional Requirements**
- [ ] All 50+ derived stats implemented and tested
- [ ] Element interaction matrix functional
- [ ] External contributor pattern working
- [ ] YAML configuration loading operational
- [ ] Actor-Core-Hierarchical integration complete
- [ ] Race-Core and Item-Core integration examples

### **Performance Requirements**
- [ ] Access time < 2 nanoseconds for hot paths
- [ ] Memory usage < 25KB per system instance
- [ ] Throughput > 1,000,000 operations per second
- [ ] Latency < 1 microsecond for 99.9% of operations
- [ ] Cache hit rate > 95% for frequently accessed data

### **Quality Requirements**
- [ ] Code coverage > 90%
- [ ] All tests passing
- [ ] Documentation complete
- [ ] No memory leaks
- [ ] Error handling comprehensive
- [ ] Performance benchmarks met

---

## 📚 **Dependencies**

### **Cargo.toml Updates**
```toml
[dependencies]
# Core dependencies
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
thiserror = "1.0"
anyhow = "1.0"

# Performance dependencies
rayon = "1.7"  # Parallel processing
dashmap = "5.4"  # Concurrent HashMap
arc-swap = "1.5"  # Atomic reference counting

# Testing dependencies
[dev-dependencies]
tokio-test = "0.4"
criterion = "0.5"
proptest = "1.0"
```

### **Feature Flags**
```toml
[features]
default = ["yaml", "async", "performance"]
yaml = ["serde", "serde_yaml"]
async = ["tokio", "async-trait"]
performance = ["rayon", "dashmap"]
simd = ["packed_simd"]
```

---

**Last Updated**: 2024-12-19  
**Version**: 1.0  
**Status**: Ready for Implementation  
**Maintainer**: Chaos World Team
