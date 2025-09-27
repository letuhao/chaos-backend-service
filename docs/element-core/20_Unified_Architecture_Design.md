# Unified Architecture Design

## 📋 **Overview**

This document defines the unified architecture for Element-Core, consolidating all previous design approaches into a single, consistent pattern. This architecture follows the data hub principle and external contributor pattern, similar to actor-core.

**Version**: 1.0  
**Last Updated**: 2024-12-19  
**Status**: Active

---

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

---

## 🏗️ **Unified Architecture**

### **Core Element-Core Structure**

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
    
    /// Performance metrics
    metrics: ElementCoreMetrics,
}

impl ElementCore {
    /// Get element stats for an actor
    pub async fn get_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        // 1. Check cache first
        if let Some(cached_stats) = self.cache.get(&actor.id, element_type).await? {
            return Ok(cached_stats);
        }
        
        // 2. Aggregate contributions from all registered systems
        let contributions = self.aggregator.aggregate_contributions(actor, element_type).await?;
        
        // 3. Calculate final stats
        let final_stats = self.calculate_final_stats(contributions).await?;
        
        // 4. Cache result
        self.cache.store(&actor.id, element_type, &final_stats).await?;
        
        Ok(final_stats)
    }
}
```

### **Unified Element Registry**

```rust
/// Unified Element Registry - Single source of truth
pub struct UnifiedElementRegistry {
    /// Core element definitions
    elements: HashMap<String, ElementDefinition>,
    
    /// System registrations
    system_registrations: HashMap<String, SystemRegistration>,
    
    /// External contributors
    contributors: HashMap<String, Arc<dyn ElementContributor>>,
    
    /// Interaction matrix
    interaction_matrix: HashMap<(String, String), ElementInteraction>,
    
    /// Configuration
    config: RegistryConfig,
}

impl UnifiedElementRegistry {
    /// Register external system
    pub async fn register_system(&mut self, system: SystemRegistration) -> ElementCoreResult<()> {
        // Validate system registration
        self.validate_system(&system)?;
        
        // Store registration
        self.system_registrations.insert(system.system_id.clone(), system);
        
        // Initialize system integration
        self.initialize_system_integration(&system.system_id).await?;
        
        Ok(())
    }
    
    /// Get element interaction factor
    pub fn get_interaction_factor(&self, attacker_element: &str, defender_element: &str) -> f64 {
        self.interaction_matrix
            .get(&(attacker_element.to_string(), defender_element.to_string()))
            .map(|interaction| interaction.factor)
            .unwrap_or(1.0)
    }
}
```

---

## 🔗 **External Contributor Pattern**

### **Element Contributor Trait**

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
    
    /// Validate contributor output
    fn validate_output(&self, output: &ElementContribution) -> ElementCoreResult<()>;
}
```

### **Element Contribution Structure**

```rust
/// Standardized element contribution
pub struct ElementContribution {
    /// System identifier
    pub system_id: String,
    
    /// Element type
    pub element_type: String,
    
    /// Stat contributions
    pub contributions: HashMap<String, f64>,
    
    /// Modifiers
    pub modifiers: HashMap<String, f64>,
    
    /// Metadata
    pub metadata: ContributionMetadata,
}

/// Contribution metadata
pub struct ContributionMetadata {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Priority
    pub priority: i64,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// Context
    pub context: HashMap<String, serde_json::Value>,
}
```

---

## 📊 **Data Flow Architecture**

### **1. System Registration Phase**
```
External System → Element-Core → Unified Registry
     ↓
Race-Core, Item-Core, Skill-Core register with Element-Core
```

### **2. Data Contribution Phase**
```
External System → Element Contributor → Element-Core → Aggregator
     ↓
Each system contributes elemental data based on actor state
```

### **3. Data Aggregation Phase**
```
Aggregator → Combine Contributions → Final Stats → Cache
     ↓
Apply aggregation rules, handle conflicts, optimize performance
```

### **4. Data Consumption Phase**
```
Element-Core → Other Systems → Game Logic
     ↓
Combat-Core, Resource-Core consume aggregated data
```

---

## 🔧 **Implementation Examples**

### **Race-Core Integration**

```rust
/// Race-Core Element Contributor
pub struct RaceCoreElementContributor {
    system_id: String,
    priority: i64,
    race_data: Arc<RaceData>,
}

impl ElementContributor for RaceCoreElementContributor {
    fn system_id(&self) -> &str { "race-core" }
    
    fn priority(&self) -> i64 { 100 }
    
    async fn contribute_element_stats(
        &self, 
        actor: &Actor, 
        element_type: &str
    ) -> ElementCoreResult<ElementContribution> {
        // Get race data for actor
        let race = self.race_data.get_race(&actor.race)?;
        
        // Calculate racial bonuses
        let fire_affinity = race.get_element_affinity(element_type);
        let fire_resistance = race.get_element_resistance(element_type);
        
        Ok(ElementContribution {
            system_id: self.system_id.clone(),
            element_type: element_type.to_string(),
            contributions: HashMap::from([
                ("affinity".to_string(), fire_affinity),
                ("resistance".to_string(), fire_resistance),
            ]),
            modifiers: HashMap::new(),
            metadata: ContributionMetadata {
                timestamp: Utc::now(),
                priority: self.priority,
                tags: vec!["racial".to_string()],
                context: HashMap::new(),
            },
        })
    }
}
```

### **Item-Core Integration**

```rust
/// Item-Core Element Contributor
pub struct ItemCoreElementContributor {
    system_id: String,
    priority: i64,
    item_data: Arc<ItemData>,
}

impl ElementContributor for ItemCoreElementContributor {
    fn system_id(&self) -> &str { "item-core" }
    
    fn priority(&self) -> i64 { 200 }
    
    async fn contribute_element_stats(
        &self, 
        actor: &Actor, 
        element_type: &str
    ) -> ElementCoreResult<ElementContribution> {
        // Get equipped items
        let equipped_items = actor.get_equipped_items();
        
        // Calculate item bonuses
        let mut total_power_bonus = 0.0;
        let mut total_defense_bonus = 0.0;
        
        for item in equipped_items {
            if let Some(element_bonus) = item.get_element_bonus(element_type) {
                total_power_bonus += element_bonus.power;
                total_defense_bonus += element_bonus.defense;
            }
        }
        
        Ok(ElementContribution {
            system_id: self.system_id.clone(),
            element_type: element_type.to_string(),
            contributions: HashMap::from([
                ("power_bonus".to_string(), total_power_bonus),
                ("defense_bonus".to_string(), total_defense_bonus),
            ]),
            modifiers: HashMap::new(),
            metadata: ContributionMetadata {
                timestamp: Utc::now(),
                priority: self.priority,
                tags: vec!["item".to_string()],
                context: HashMap::new(),
            },
        })
    }
}
```

### **Skill-Core Integration**

```rust
/// Skill-Core Element Contributor
pub struct SkillCoreElementContributor {
    system_id: String,
    priority: i64,
    skill_data: Arc<SkillData>,
}

impl ElementContributor for SkillCoreElementContributor {
    fn system_id(&self) -> &str { "skill-core" }
    
    fn priority(&self) -> i64 { 300 }
    
    async fn contribute_element_stats(
        &self, 
        actor: &Actor, 
        element_type: &str
    ) -> ElementCoreResult<ElementContribution> {
        // Get active skills
        let active_skills = actor.get_active_skills();
        
        // Calculate skill bonuses
        let mut total_mastery_bonus = 0.0;
        let mut total_efficiency_bonus = 0.0;
        
        for skill in active_skills {
            if let Some(element_skill) = skill.get_element_skill(element_type) {
                total_mastery_bonus += element_skill.mastery_bonus;
                total_efficiency_bonus += element_skill.efficiency_bonus;
            }
        }
        
        Ok(ElementContribution {
            system_id: self.system_id.clone(),
            element_type: element_type.to_string(),
            contributions: HashMap::from([
                ("mastery_bonus".to_string(), total_mastery_bonus),
                ("efficiency_bonus".to_string(), total_efficiency_bonus),
            ]),
            modifiers: HashMap::new(),
            metadata: ContributionMetadata {
                timestamp: Utc::now(),
                priority: self.priority,
                tags: vec!["skill".to_string()],
                context: HashMap::new(),
            },
        })
    }
}
```

---

## 🚀 **Usage Examples**

### **System Registration**

```rust
// Create Element-Core
let mut element_core = ElementCore::new();

// Register Race-Core
let race_contributor = Arc::new(RaceCoreElementContributor::new());
element_core.register_contributor(race_contributor).await?;

// Register Item-Core
let item_contributor = Arc::new(ItemCoreElementContributor::new());
element_core.register_contributor(item_contributor).await?;

// Register Skill-Core
let skill_contributor = Arc::new(SkillCoreElementContributor::new());
element_core.register_contributor(skill_contributor).await?;
```

### **Element Stats Calculation**

```rust
// Get element stats for actor
let fire_stats = element_core.get_element_stats(&actor, "fire").await?;

// Stats include contributions from all systems:
// - Race-Core: racial affinity and resistance
// - Item-Core: equipment bonuses
// - Skill-Core: active skill bonuses
// - Other systems: additional contributions

println!("Fire Power: {}", fire_stats.power);
println!("Fire Defense: {}", fire_stats.defense);
println!("Fire Affinity: {}", fire_stats.affinity);
```

### **Element Interaction**

```rust
// Calculate element interaction factor
let interaction_factor = element_core.get_interaction_factor("fire", "water");

// Use in combat calculation
let base_damage = 100.0;
let final_damage = base_damage * interaction_factor;
```

---

## 📚 **Related Documents**

- [Element System Architecture](01_Element_System_Architecture.md) - Basic architecture
- [Element Registry Design](04_Element_Registry_Design.md) - Registry implementation
- [Universal Element Registry Design](18_Universal_Element_Registry_Design.md) - Advanced registry features
- [Stats Distribution Design](19_Stats_Distribution_Design.md) - External system integration

---

## ⚖️ **Balance Considerations**

### **Performance vs Flexibility**
- **Performance**: Optimized for high-frequency operations
- **Flexibility**: Extensible for new systems and elements

### **Simplicity vs Power**
- **Simplicity**: Clear, understandable architecture
- **Power**: Comprehensive elemental system capabilities

### **Maintenance vs Features**
- **Maintenance**: Easy to update and extend
- **Features**: Rich elemental interactions and mechanics

---

## 🔄 **Migration Strategy**

### **From Old Architecture**
1. **Replace multiple registries** with UnifiedElementRegistry
2. **Convert system integrations** to ElementContributor pattern
3. **Update data flow** to use aggregation approach
4. **Migrate existing data** to new format

### **Backward Compatibility**
- **Legacy API support** during transition
- **Data migration tools** for existing systems
- **Gradual rollout** to minimize disruption

---

## 🔄 **Evolution Strategy**

### **Version 1.0 (Current)**
- Unified architecture implementation
- External contributor pattern
- Performance optimization

### **Version 2.0 (Future)**
- Advanced caching strategies
- Machine learning integration
- Enhanced performance monitoring

### **Version 3.0 (Future)**
- AI-powered optimization
- Predictive caching
- Advanced analytics

---

**Last Updated**: 2024-12-19  
**Version**: 1.0  
**Status**: Active  
**Next Review**: 2024-12-26
