# Resource Manager Integration Design

## 📋 **Tổng Quan**

Tài liệu này mô tả chi tiết cách tích hợp Resource Manager với Element-Core, bao gồm cách Resource Manager quản lý primary stats và Element-Core tính toán derived stats từ những primary stats đó.

**⚠️ Critical Implementation Notes**: Xem [Implementation Notes](06_Implementation_Notes.md) để biết các yêu cầu implementation quan trọng, bao gồm Omni additive-only rule và damage composition law.

## 🎯 **Integration Architecture**

### **1. Resource Manager → Element-Core Flow**

```
Resource Manager (Primary Stats)
    ↓
Element-Core (Derived Stats Calculation)
    ↓
Combat-Core (Damage/Status Application)
    ↓
Actor Core (Final Stats Aggregation)
```

### **2. Key Integration Points**

#### **A. Primary Stats Management**
- **Resource Manager**: Quản lý primary stats (vitality, intelligence, strength, etc.)
- **Element-Core**: Tính toán derived stats từ primary stats
- **Actor Core**: Aggregates tất cả stats từ các systems

#### **B. Stats Change Propagation**
- **Resource Manager**: Trigger stats change events
- **Element-Core**: Listen to stats change events và recalculate derived stats
- **Combat-Core**: Use updated derived stats for calculations

## 🏗️ **Resource Manager Integration**

### **1. Primary Stats to Derived Stats Mapping**

#### **RPG Resource Manager Integration**
```rust
// RPG Resource Manager provides primary stats
pub struct RpgResourceManager {
    // Primary stats từ cultivation systems
    pub primary_stats: HashMap<String, f64>,
    // Resource definitions với formulas
    pub resource_definitions: HashMap<String, RpgResourceDefinition>,
}

// Element-Core maps primary stats to derived stats
impl ElementCore {
    pub fn calculate_derived_stats_from_primary(
        &self,
        primary_stats: &HashMap<String, f64>,
        element_type: &str,
    ) -> HashMap<DerivedStatType, f64> {
        let mut derived_stats = HashMap::new();
        
        // Map primary stats to element-specific derived stats
        if let Some(vitality) = primary_stats.get("vitality") {
            // Vitality contributes to defense and health
            derived_stats.insert(DerivedStatType::DefensePoint, vitality * 2.0);
            derived_stats.insert(DerivedStatType::StatusResistance, vitality * 0.5);
        }
        
        if let Some(intelligence) = primary_stats.get("intelligence") {
            // Intelligence contributes to magic power and status probability
            derived_stats.insert(DerivedStatType::PowerPoint, intelligence * 1.5);
            derived_stats.insert(DerivedStatType::StatusProbability, intelligence * 0.3);
        }
        
        if let Some(strength) = primary_stats.get("strength") {
            // Strength contributes to physical power and crit damage
            derived_stats.insert(DerivedStatType::PowerPoint, strength * 1.2);
            derived_stats.insert(DerivedStatType::CritDamage, strength * 0.4);
        }
        
        if let Some(agility) = primary_stats.get("agility") {
            // Agility contributes to accuracy and dodge
            derived_stats.insert(DerivedStatType::AccurateRate, agility * 1.0);
            derived_stats.insert(DerivedStatType::DodgeRate, agility * 0.8);
        }
        
        if let Some(wisdom) = primary_stats.get("wisdom") {
            // Wisdom contributes to crit rate and status duration
            derived_stats.insert(DerivedStatType::CritRate, wisdom * 0.6);
            derived_stats.insert(DerivedStatType::StatusDuration, wisdom * 0.2);
        }
        
        // Apply element-specific scaling
        self.apply_element_scaling(&mut derived_stats, element_type);
        
        derived_stats
    }
}
```

#### **Magic Resource Manager Integration**
```rust
// Magic Resource Manager provides magical primary stats
pub struct MagicResourceManager {
    // Magical primary stats
    pub magical_stats: HashMap<String, f64>,
    // Magic school affinities
    pub school_affinities: HashMap<MagicSchool, f64>,
}

// Element-Core maps magical stats to derived stats
impl ElementCore {
    pub fn calculate_magical_derived_stats(
        &self,
        magical_stats: &HashMap<String, f64>,
        school_affinities: &HashMap<MagicSchool, f64>,
        element_type: &str,
    ) -> HashMap<DerivedStatType, f64> {
        let mut derived_stats = HashMap::new();
        
        // Map magical stats to element-specific derived stats
        if let Some(mana) = magical_stats.get("mana") {
            // Mana contributes to magic power and status intensity
            derived_stats.insert(DerivedStatType::PowerPoint, mana * 1.0);
            derived_stats.insert(DerivedStatType::StatusIntensity, mana * 0.4);
        }
        
        if let Some(arcane_focus) = magical_stats.get("arcane_focus") {
            // Arcane focus contributes to accuracy and crit rate
            derived_stats.insert(DerivedStatType::AccurateRate, arcane_focus * 0.8);
            derived_stats.insert(DerivedStatType::CritRate, arcane_focus * 0.5);
        }
        
        if let Some(magical_knowledge) = magical_stats.get("magical_knowledge") {
            // Knowledge contributes to status duration and resistance
            derived_stats.insert(DerivedStatType::StatusDuration, magical_knowledge * 0.3);
            derived_stats.insert(DerivedStatType::StatusResistance, magical_knowledge * 0.6);
        }
        
        // Apply school affinity bonuses
        if let Some(affinity) = school_affinities.get(&self.get_element_school(element_type)) {
            let affinity_bonus = *affinity * 0.2;
            for (_, value) in derived_stats.iter_mut() {
                *value += affinity_bonus;
            }
        }
        
        derived_stats
    }
}
```

### **2. Stats Change Event System**

#### **Event-Driven Integration**
```rust
// Resource Manager triggers stats change events
pub struct StatsChangeEvent {
    pub actor_id: String,
    pub system_id: String,
    pub primary_stats: HashMap<String, f64>,
    pub timestamp: u64,
}

// Element-Core listens to stats change events
impl ElementCore {
    pub fn handle_stats_change_event(&self, event: &StatsChangeEvent) -> Result<(), ElementError> {
        // Recalculate derived stats for all elements
        for element_type in self.get_all_element_types() {
            let derived_stats = self.calculate_derived_stats_from_primary(
                &event.primary_stats,
                element_type,
            );
            
            // Update actor's element stats
            self.update_actor_element_stats(
                &event.actor_id,
                element_type,
                derived_stats,
            )?;
        }
        
        // Recalculate Omni stats
        let omni_stats = self.calculate_omni_stats_from_primary(&event.primary_stats);
        self.update_actor_omni_stats(&event.actor_id, omni_stats)?;
        
        // Dispatch element stats change event
        self.dispatch_element_stats_change_event(&event.actor_id)?;
        
        Ok(())
    }
}
```

### **3. Multi-System Stats Aggregation**

#### **Actor Core Integration**
```rust
// Actor Core aggregates stats from all systems
pub struct ActorCore {
    // Resource managers
    pub rpg_resource_manager: Arc<RpgResourceManager>,
    pub magic_resource_manager: Arc<MagicResourceManager>,
    // Element core
    pub element_core: Arc<ElementCore>,
    // Stats aggregation engine
    pub stats_aggregator: Arc<StatsAggregator>,
}

impl ActorCore {
    pub fn get_actor_total_stats(&self, actor_id: &str) -> Result<ActorTotalStats, ActorCoreError> {
        // Get primary stats from resource managers
        let rpg_stats = self.rpg_resource_manager.get_actor_primary_stats(actor_id)?;
        let magic_stats = self.magic_resource_manager.get_actor_primary_stats(actor_id)?;
        
        // Combine primary stats
        let mut combined_primary_stats = rpg_stats;
        combined_primary_stats.extend(magic_stats);
        
        // Get derived stats from element core
        let element_stats = self.element_core.get_actor_all_element_stats(actor_id)?;
        let omni_stats = self.element_core.get_actor_omni_stats(actor_id)?;
        
        // Aggregate all stats
        let total_stats = self.stats_aggregator.aggregate_stats(
            combined_primary_stats,
            element_stats,
            omni_stats,
        )?;
        
        Ok(total_stats)
    }
}
```

## 🔄 **Stats Change Propagation Flow**

### **1. Primary Stats Change**

```rust
// 1. Resource Manager detects primary stats change
pub fn update_primary_stats(&mut self, actor_id: &str, new_stats: HashMap<String, f64>) {
    // Update internal stats
    self.actor_primary_stats.insert(actor_id.to_string(), new_stats.clone());
    
    // Dispatch stats change event
    let event = StatsChangeEvent {
        actor_id: actor_id.to_string(),
        system_id: self.system_id.clone(),
        primary_stats: new_stats,
        timestamp: current_timestamp(),
    };
    
    self.event_dispatcher.dispatch_event(&event);
}
```

### **2. Element-Core Response**

```rust
// 2. Element-Core receives stats change event
impl ElementCore {
    pub fn on_primary_stats_changed(&self, event: &StatsChangeEvent) -> Result<(), ElementError> {
        // Recalculate all element stats
        for element_type in self.get_all_element_types() {
            let derived_stats = self.calculate_derived_stats_from_primary(
                &event.primary_stats,
                element_type,
            );
            
            // Update cached stats
            self.update_cached_element_stats(
                &event.actor_id,
                element_type,
                derived_stats,
            )?;
        }
        
        // Recalculate Omni stats
        let omni_stats = self.calculate_omni_stats_from_primary(&event.primary_stats);
        self.update_cached_omni_stats(&event.actor_id, omni_stats)?;
        
        // Notify dependent systems
        self.notify_stats_dependents(&event.actor_id)?;
        
        Ok(())
    }
}
```

### **3. Combat-Core Update**

```rust
// 3. Combat-Core receives element stats change notification
impl CombatCore {
    pub fn on_element_stats_changed(&self, actor_id: &str) -> Result<(), CombatError> {
        // Invalidate combat calculations cache
        self.invalidate_actor_combat_cache(actor_id)?;
        
        // Recalculate combat stats if needed
        if self.is_actor_in_combat(actor_id)? {
            self.recalculate_combat_stats(actor_id)?;
        }
        
        Ok(())
    }
}
```

## ⚙️ **Configuration & Scaling**

### **1. Element-Specific Primary Stats Mapping**

```yaml
# element_primary_stats_mapping.yaml
fire:
  primary_stats:
    strength: 1.5      # Strength contributes 1.5x to fire power
    intelligence: 1.0  # Intelligence contributes 1.0x to fire power
    vitality: 0.8      # Vitality contributes 0.8x to fire defense
  derived_stats:
    power_point: "strength * 1.5 + intelligence * 1.0"
    defense_point: "vitality * 0.8 + constitution * 1.2"
    crit_rate: "wisdom * 0.6 + agility * 0.4"
    status_probability: "intelligence * 0.3 + wisdom * 0.2"

water:
  primary_stats:
    intelligence: 1.2
    wisdom: 1.3
    vitality: 1.0
  derived_stats:
    power_point: "intelligence * 1.2 + wisdom * 1.0"
    defense_point: "vitality * 1.0 + constitution * 1.0"
    status_duration: "wisdom * 0.4 + intelligence * 0.2"
    status_resistance: "wisdom * 0.8 + vitality * 0.5"
```

### **2. Omni Stats Calculation**

```yaml
# omni_stats_calculation.yaml
omni:
  primary_stats:
    level: 2.0         # Level contributes 2x to all omni stats
    constitution: 1.0  # Constitution contributes 1x to all omni stats
  derived_stats:
    power_point: "level * 2.0 + constitution * 1.0"
    defense_point: "level * 2.0 + constitution * 1.0"
    crit_rate: "level * 0.1 + constitution * 0.05"
    status_resistance: "level * 0.2 + constitution * 0.1"
```

## 🚨 **Critical Implementation Requirements**

### **1. Stats Change Event Order**

```rust
// Critical: Stats change events must be processed in correct order
pub fn process_stats_change_events(&self, events: Vec<StatsChangeEvent>) -> Result<(), Error> {
    // 1. Sort events by timestamp
    let mut sorted_events = events;
    sorted_events.sort_by_key(|e| e.timestamp);
    
    // 2. Process events in order
    for event in sorted_events {
        // 3. Update primary stats first
        self.update_primary_stats(&event.actor_id, &event.primary_stats)?;
        
        // 4. Recalculate derived stats
        self.recalculate_derived_stats(&event.actor_id)?;
        
        // 5. Notify dependent systems
        self.notify_dependent_systems(&event.actor_id)?;
    }
    
    Ok(())
}
```

### **2. Cache Invalidation**

```rust
// Critical: Proper cache invalidation to ensure consistency
impl ElementCore {
    pub fn invalidate_actor_stats_cache(&self, actor_id: &str) -> Result<(), ElementError> {
        // Invalidate element stats cache
        self.element_stats_cache.remove(actor_id);
        
        // Invalidate omni stats cache
        self.omni_stats_cache.remove(actor_id);
        
        // Invalidate derived stats cache
        self.derived_stats_cache.remove(actor_id);
        
        // Notify dependent systems
        self.notify_cache_invalidation(actor_id)?;
        
        Ok(())
    }
}
```

### **3. Performance Optimization**

```rust
// Critical: Batch stats updates for performance
impl ElementCore {
    pub fn batch_update_actor_stats(
        &self,
        actor_id: &str,
        primary_stats: &HashMap<String, f64>,
    ) -> Result<(), ElementError> {
        // Calculate all derived stats in one go
        let mut all_derived_stats = HashMap::new();
        
        for element_type in self.get_all_element_types() {
            let derived_stats = self.calculate_derived_stats_from_primary(
                primary_stats,
                element_type,
            );
            all_derived_stats.insert(element_type.clone(), derived_stats);
        }
        
        // Update all caches at once
        self.batch_update_caches(actor_id, &all_derived_stats)?;
        
        // Dispatch single event for all changes
        self.dispatch_batch_stats_change_event(actor_id)?;
        
        Ok(())
    }
}
```

## 📚 **Related Documents**

- [Element Core Overview](00_Element_Core_Overview.md)
- [Implementation Notes](06_Implementation_Notes.md)
- [Multi-System Integration Design](02_Multi_System_Integration_Design.md)
- [Status Effect System Design](04_Status_Effect_System_Design.md)

## 🔄 **Update History**

- **2024-01-XX**: Initial resource manager integration design
- **2024-01-XX**: Added primary stats to derived stats mapping
- **2024-01-XX**: Added event-driven integration system
- **2024-01-XX**: Added multi-system stats aggregation
- **2024-01-XX**: Added critical implementation requirements
