# Element-Core Migration Guide

## 📋 **Overview**

This document provides a comprehensive migration guide for transitioning from the old Element-Core architecture to the new unified architecture. It includes step-by-step instructions, code examples, and best practices.

**Version**: 1.0  
**Last Updated**: 2024-12-19  
**Status**: Active

---

## 🎯 **Migration Goals**

### **1. Architecture Consolidation**
- Consolidate multiple registry implementations into UnifiedElementRegistry
- Standardize integration patterns to External Contributor pattern
- Implement data hub principle

### **2. Performance Improvement**
- Optimize data aggregation and caching
- Reduce code duplication
- Improve system scalability

### **3. Maintainability Enhancement**
- Simplify system architecture
- Standardize interfaces
- Improve documentation

---

## 🏗️ **Migration Phases**

### **Phase 1: Preparation (Week 1)**
- Audit existing implementations
- Create migration plan
- Set up development environment

### **Phase 2: Core Migration (Week 2-3)**
- Implement UnifiedElementRegistry
- Create ElementContributor trait
- Migrate core systems

### **Phase 3: System Integration (Week 4-5)**
- Migrate external systems
- Update integration points
- Test system interactions

### **Phase 4: Optimization (Week 6)**
- Performance optimization
- Final testing
- Documentation updates

---

## 🔧 **Migration Steps**

### **Step 1: Create Unified Architecture**

#### **1.1 Create UnifiedElementRegistry**

```rust
// OLD: Multiple registry implementations
// ElementRegistry, UniversalElementRegistry, ElementContributorRegistry, etc.

// NEW: Single unified registry
pub struct UnifiedElementRegistry {
    elements: HashMap<String, ElementDefinition>,
    system_registrations: HashMap<String, SystemRegistration>,
    contributors: HashMap<String, Arc<dyn ElementContributor>>,
    interaction_matrix: HashMap<(String, String), ElementInteraction>,
    config: RegistryConfig,
}
```

#### **1.2 Create ElementContributor Trait**

```rust
// OLD: Various integration patterns
// Hybrid approach, system registration, external contributor

// NEW: Standardized contributor pattern
pub trait ElementContributor: Send + Sync {
    fn system_id(&self) -> &str;
    fn priority(&self) -> i64;
    async fn contribute_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementContribution>;
    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()>;
}
```

### **Step 2: Migrate Core Systems**

#### **2.1 Race-Core Migration**

```rust
// OLD: Direct integration with Element-Core
impl RaceCore {
    pub fn get_element_affinity(&self, element_type: &str) -> f64 {
        // Direct calculation
    }
}

// NEW: ElementContributor implementation
pub struct RaceCoreElementContributor {
    system_id: String,
    priority: i64,
    race_data: Arc<RaceData>,
}

impl ElementContributor for RaceCoreElementContributor {
    fn system_id(&self) -> &str { "race-core" }
    
    async fn contribute_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementContribution> {
        let race = self.race_data.get_race(&actor.race)?;
        let affinity = race.get_element_affinity(element_type);
        let resistance = race.get_element_resistance(element_type);
        
        Ok(ElementContribution {
            system_id: self.system_id.clone(),
            element_type: element_type.to_string(),
            contributions: HashMap::from([
                ("affinity".to_string(), affinity),
                ("resistance".to_string(), resistance),
            ]),
            modifiers: HashMap::new(),
            metadata: ContributionMetadata::default(),
        })
    }
}
```

#### **2.2 Item-Core Migration**

```rust
// OLD: Direct item bonus calculation
impl ItemCore {
    pub fn calculate_element_bonus(&self, item: &Item, element_type: &str) -> ElementBonus {
        // Direct calculation
    }
}

// NEW: ElementContributor implementation
pub struct ItemCoreElementContributor {
    system_id: String,
    priority: i64,
    item_data: Arc<ItemData>,
}

impl ElementContributor for ItemCoreElementContributor {
    fn system_id(&self) -> &str { "item-core" }
    
    async fn contribute_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementContribution> {
        let equipped_items = actor.get_equipped_items();
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
            metadata: ContributionMetadata::default(),
        })
    }
}
```

### **Step 3: Update Element-Core**

#### **3.1 Implement Element Aggregator**

```rust
// NEW: Element aggregator for combining contributions
pub struct ElementAggregator {
    contributors: HashMap<String, Arc<dyn ElementContributor>>,
    aggregation_rules: HashMap<String, AggregationRule>,
    cache: AggregationCache,
}

impl ElementAggregator {
    pub async fn aggregate_contributions(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        let mut contributions = Vec::new();
        
        // Collect contributions from all registered systems
        for contributor in self.contributors.values() {
            let contribution = contributor.contribute_element_stats(actor, element_type).await?;
            contributions.push(contribution);
        }
        
        // Sort by priority
        contributions.sort_by_key(|c| c.metadata.priority);
        
        // Apply aggregation rules
        let final_stats = self.apply_aggregation_rules(contributions).await?;
        
        Ok(final_stats)
    }
}
```

#### **3.2 Update Element-Core Main Structure**

```rust
// OLD: Complex Element-Core with multiple responsibilities
pub struct ElementCore {
    // Multiple registries
    // Complex business logic
    // Direct system integrations
}

// NEW: Simplified Element-Core as data hub
pub struct ElementCore {
    registry: UnifiedElementRegistry,
    aggregator: ElementAggregator,
    cache: ElementCache,
    config: ElementConfig,
}

impl ElementCore {
    pub async fn get_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        // 1. Check cache
        if let Some(cached_stats) = self.cache.get(&actor.id, element_type).await? {
            return Ok(cached_stats);
        }
        
        // 2. Aggregate contributions
        let stats = self.aggregator.aggregate_contributions(actor, element_type).await?;
        
        // 3. Cache result
        self.cache.store(&actor.id, element_type, &stats).await?;
        
        Ok(stats)
    }
}
```

---

## 🔄 **Backward Compatibility**

### **Legacy API Support**

```rust
/// Legacy API wrapper for backward compatibility
pub struct LegacyElementCore {
    new_core: ElementCore,
    legacy_registry: LegacyElementRegistry,
}

impl LegacyElementCore {
    /// Legacy method for getting element
    pub fn get_element(&self, element_type: &str) -> Option<ElementDefinition> {
        self.new_core.registry.get_element(element_type)
    }
    
    /// Legacy method for element interaction
    pub fn get_element_interaction(&self, attacker: &str, defender: &str) -> f64 {
        self.new_core.registry.get_interaction_factor(attacker, defender)
    }
}
```

### **Data Migration Tools**

```rust
/// Data migration utility
pub struct ElementDataMigrator {
    old_registry: LegacyElementRegistry,
    new_registry: UnifiedElementRegistry,
}

impl ElementDataMigrator {
    /// Migrate element definitions
    pub async fn migrate_elements(&mut self) -> ElementCoreResult<()> {
        for (id, element) in self.old_registry.get_all_elements() {
            let new_element = self.convert_element_definition(element)?;
            self.new_registry.register_element(new_element).await?;
        }
        Ok(())
    }
    
    /// Convert old element definition to new format
    fn convert_element_definition(&self, old_element: &LegacyElementDefinition) -> ElementCoreResult<ElementDefinition> {
        Ok(ElementDefinition {
            id: old_element.id.clone(),
            name: old_element.name.clone(),
            description: old_element.description.clone(),
            category: self.convert_category(&old_element.category)?,
            base_properties: self.convert_properties(&old_element.properties)?,
            interaction_rules: self.convert_interaction_rules(&old_element.interactions)?,
            derived_stats_config: DerivedStatsConfig::default(),
            metadata: ElementMetadata::default(),
        })
    }
}
```

---

## 🚀 **Migration Examples**

### **Complete System Migration**

```rust
// 1. Create new Element-Core
let mut element_core = ElementCore::new();

// 2. Register external contributors
let race_contributor = Arc::new(RaceCoreElementContributor::new());
element_core.register_contributor(race_contributor).await?;

let item_contributor = Arc::new(ItemCoreElementContributor::new());
element_core.register_contributor(item_contributor).await?;

let skill_contributor = Arc::new(SkillCoreElementContributor::new());
element_core.register_contributor(skill_contributor).await?;

// 3. Migrate existing data
let mut migrator = ElementDataMigrator::new(old_registry, element_core.registry.clone());
migrator.migrate_elements().await?;

// 4. Test new system
let actor = Actor::new("test_actor".to_string(), "human".to_string());
let fire_stats = element_core.get_element_stats(&actor, "fire").await?;

// 5. Verify results
assert!(fire_stats.power > 0.0);
assert!(fire_stats.defense > 0.0);
```

### **Gradual Migration Strategy**

```rust
/// Gradual migration with feature flags
pub struct GradualMigrationManager {
    element_core: ElementCore,
    legacy_core: LegacyElementCore,
    migration_config: MigrationConfig,
}

impl GradualMigrationManager {
    /// Get element stats with migration support
    pub async fn get_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        if self.migration_config.use_new_architecture {
            // Use new architecture
            self.element_core.get_element_stats(actor, element_type).await
        } else {
            // Use legacy architecture
            self.legacy_core.get_element_stats(actor, element_type).await
        }
    }
    
    /// Migrate specific actor
    pub async fn migrate_actor(&self, actor_id: &str) -> ElementCoreResult<()> {
        // Migrate actor data to new format
        // Update actor references
        // Verify migration success
        Ok(())
    }
}
```

---

## 📊 **Migration Testing**

### **Unit Tests**

```rust
#[cfg(test)]
mod migration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_element_contributor_migration() {
        // Test Race-Core contributor
        let race_contributor = RaceCoreElementContributor::new();
        let actor = Actor::new("test".to_string(), "human".to_string());
        
        let contribution = race_contributor.contribute_element_stats(&actor, "fire").await.unwrap();
        
        assert_eq!(contribution.system_id, "race-core");
        assert!(contribution.contributions.contains_key("affinity"));
        assert!(contribution.contributions.contains_key("resistance"));
    }
    
    #[tokio::test]
    async fn test_element_aggregation() {
        // Test element aggregation
        let mut aggregator = ElementAggregator::new();
        
        // Add contributors
        aggregator.add_contributor(Arc::new(RaceCoreElementContributor::new()));
        aggregator.add_contributor(Arc::new(ItemCoreElementContributor::new()));
        
        let actor = Actor::new("test".to_string(), "human".to_string());
        let stats = aggregator.aggregate_contributions(&actor, "fire").await.unwrap();
        
        assert!(stats.power > 0.0);
        assert!(stats.defense > 0.0);
    }
}
```

### **Integration Tests**

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_complete_migration() {
        // Create old and new systems
        let old_core = LegacyElementCore::new();
        let mut new_core = ElementCore::new();
        
        // Register contributors
        new_core.register_contributor(Arc::new(RaceCoreElementContributor::new())).await.unwrap();
        new_core.register_contributor(Arc::new(ItemCoreElementContributor::new())).await.unwrap();
        
        // Test same actor with both systems
        let actor = Actor::new("test".to_string(), "human".to_string());
        
        let old_stats = old_core.get_element_stats(&actor, "fire").await.unwrap();
        let new_stats = new_core.get_element_stats(&actor, "fire").await.unwrap();
        
        // Results should be similar (within tolerance)
        assert!((old_stats.power - new_stats.power).abs() < 0.01);
        assert!((old_stats.defense - new_stats.defense).abs() < 0.01);
    }
}
```

---

## 📚 **Related Documents**

- [Unified Architecture Design](20_Unified_Architecture_Design.md) - Target architecture
- [Element System Architecture](01_Element_System_Architecture.md) - Basic architecture
- [Element Registry Design](04_Element_Registry_Design.md) - Registry implementation
- [Stats Distribution Design](19_Stats_Distribution_Design.md) - External system integration

---

## ⚖️ **Migration Considerations**

### **Performance Impact**
- **Initial**: Slight performance impact during migration
- **Long-term**: Significant performance improvement
- **Mitigation**: Gradual migration, performance monitoring

### **Data Consistency**
- **Risk**: Data inconsistency during migration
- **Mitigation**: Validation tools, rollback procedures
- **Monitoring**: Continuous data integrity checks

### **System Downtime**
- **Risk**: System downtime during migration
- **Mitigation**: Blue-green deployment, feature flags
- **Monitoring**: Real-time system health monitoring

---

## 🔄 **Rollback Strategy**

### **Rollback Procedures**

```rust
/// Rollback manager for migration
pub struct MigrationRollbackManager {
    backup_registry: UnifiedElementRegistry,
    rollback_config: RollbackConfig,
}

impl MigrationRollbackManager {
    /// Rollback to previous version
    pub async fn rollback(&mut self) -> ElementCoreResult<()> {
        // 1. Stop new system
        // 2. Restore backup data
        // 3. Restart old system
        // 4. Verify rollback success
        Ok(())
    }
    
    /// Create backup before migration
    pub async fn create_backup(&mut self) -> ElementCoreResult<()> {
        // Create backup of current system state
        Ok(())
    }
}
```

### **Emergency Procedures**
1. **Immediate rollback** if critical issues detected
2. **Data recovery** from backup systems
3. **System restoration** to previous stable state
4. **Issue analysis** and resolution

---

## 🎯 **Success Criteria**

### **Technical Success**
- [ ] All systems migrated successfully
- [ ] Performance improved by 20%+
- [ ] Code duplication reduced by 50%+
- [ ] Test coverage maintained at 90%+

### **Business Success**
- [ ] Zero data loss during migration
- [ ] Minimal system downtime
- [ ] Developer productivity improved
- [ ] Maintenance effort reduced

---

**Last Updated**: 2024-12-19  
**Version**: 1.0  
**Status**: Active  
**Next Review**: 2024-12-26
