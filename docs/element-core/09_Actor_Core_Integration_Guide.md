# Actor Core Integration Guide - Elemental Mastery System

## Tổng Quan (Overview)

Tài liệu này hướng dẫn chi tiết cách tích hợp Elemental Mastery System vào Actor Core framework. Elemental Mastery System được thiết kế như một cultivation system độc lập, kế thừa từ `SystemResourceCalculator` trait của Actor Core.

**🔄 Updated Integration**: Elemental Mastery System hiện tích hợp với Damage Manager để cung cấp elemental damage data và modifiers. Xem [Damage Manager Overview](../combat-core/damage-management/00_Damage_Manager_Overview.md) để hiểu integration mới.

## 🏗️ **Kiến Trúc Tích Hợp (Integration Architecture)**

### 1. Actor Core Framework Pattern

Elemental Mastery System tuân theo pattern chuẩn của Actor Core:

```rust
// Actor Core SystemResourceCalculator trait
#[async_trait]
pub trait SystemResourceCalculator: Send + Sync {
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>>;
    fn system_id(&self) -> &str;
    fn affects_resource(&self, resource_id: &str) -> bool;
    async fn notify_stat_change(&self, actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()>;
    fn get_resource_dependencies(&self) -> Vec<String>;
    fn get_resource_categories(&self) -> Vec<ResourceCategory>;
    async fn is_active(&self, actor: &Actor) -> ActorCoreResult<bool>;
}
```

### 2. Elemental Mastery System Implementation

```rust
/// Plugin-Based Element Mastery System Resource Calculator
#[derive(Debug)]
pub struct PluginBasedElementMasterySystem {
    /// Base calculator (inherits from Actor Core)
    base: BaseSystemResourceCalculator,
    /// Element plugin registry
    plugin_registry: ElementPluginRegistry,
    /// Decay configuration
    decay_config: MasteryDecayConfig,
}

#[async_trait]
impl SystemResourceCalculator for PluginBasedElementMasterySystem {
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut resources = HashMap::new();
        
        // Calculate mastery for each element
        for (element_id, _) in &self.plugin_registry.plugins {
            let mastery = self.calculate_element_mastery(actor, element_id);
            let decay = self.calculate_mastery_decay(actor, element_id);
            let final_mastery = (mastery - decay).max(0.0);
            
            resources.insert(format!("{}_mastery_current", element_id), final_mastery);
            resources.insert(format!("{}_mastery_max", element_id), 100.0);
            resources.insert(format!("{}_mastery_decay", element_id), decay);
        }
        
        Ok(resources)
    }
    
    fn system_id(&self) -> &str {
        "element_mastery_system"
    }
    
    fn affects_resource(&self, resource_id: &str) -> bool {
        resource_id.starts_with("element_mastery_") || 
        resource_id.ends_with("_mastery_current") ||
        resource_id.ends_with("_mastery_max") ||
        resource_id.ends_with("_mastery_decay")
    }
    
    async fn notify_stat_change(&self, actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()> {
        for stat_id in changed_stats {
            if stat_id.ends_with("_experience") || stat_id.ends_with("_training_time") {
                let element_id = stat_id.trim_end_matches("_experience").trim_end_matches("_training_time");
                if self.plugin_registry.plugins.contains_key(element_id) {
                    info!(element = %element_id, stat = %stat_id, "Element mastery affected by stat change");
                }
            }
        }
        Ok(())
    }
    
    fn get_resource_dependencies(&self) -> Vec<String> {
        let mut dependencies = Vec::new();
        for plugin in self.plugin_registry.plugins.values() {
            dependencies.extend(plugin.get_element_definition().dependencies.clone());
        }
        dependencies
    }
    
    fn get_resource_categories(&self) -> Vec<ResourceCategory> {
        vec![ResourceCategory::Cultivation, ResourceCategory::Special]
    }
    
    async fn is_active(&self, actor: &Actor) -> ActorCoreResult<bool> {
        // Check if actor has any element mastery
        for (element_id, _) in &self.plugin_registry.plugins {
            let mastery = actor.get_data().get(&format!("{}_mastery_current", element_id))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            if mastery > 0.0 {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
```

## 🔧 **Implementation Steps**

### Step 1: Tạo Elemental Mastery System

```rust
// 1. Tạo Elemental Mastery System
let mut element_mastery_system = PluginBasedElementMasterySystem::new();

// 2. Register element plugins
element_mastery_system.register_element_plugin(Box::new(FireElementPlugin));
element_mastery_system.register_element_plugin(Box::new(WaterElementPlugin));
element_mastery_system.register_element_plugin(Box::new(EarthElementPlugin));
// ... register other elements

// 3. Load from configuration
element_mastery_system.load_plugins_from_config("config/element_plugins.yaml").await?;
```

### Step 2: Tích Hợp Vào Enhanced Hybrid Resource Manager

```rust
impl EnhancedHybridResourceManager {
    /// Register Elemental Mastery System
    pub fn register_element_mastery_system(&mut self) -> ActorCoreResult<()> {
        let element_mastery_system = PluginBasedElementMasterySystem::new();
        self.register_system(Box::new(element_mastery_system));
        Ok(())
    }
    
    /// Initialize with Elemental Mastery System
    pub async fn new_with_element_mastery() -> ActorCoreResult<Self> {
        let mut manager = Self::new();
        
        // Register default systems
        manager.register_rpg_system()?;
        manager.register_magic_system()?;
        
        // Register Elemental Mastery System
        manager.register_element_mastery_system()?;
        
        Ok(manager)
    }
}
```

### Step 3: Cấu Hình Actor Data

```rust
// Actor data structure for Elemental Mastery
pub struct ActorElementMasteryData {
    // Fire element data
    pub fire_experience: f64,
    pub fire_training_time: f64,
    pub fire_last_used: f64,
    
    // Water element data
    pub water_experience: f64,
    pub water_training_time: f64,
    pub water_last_used: f64,
    
    // ... other elements
}

impl Actor {
    /// Initialize element mastery data
    pub fn initialize_element_mastery(&mut self) {
        let elements = vec!["fire", "water", "earth", "metal", "wood", "light", "dark"];
        
        for element in elements {
            self.set_data(&format!("{}_experience", element), 0.0.into());
            self.set_data(&format!("{}_training_time", element), 0.0.into());
            self.set_data(&format!("{}_last_used", element), 0.0.into());
        }
    }
    
    /// Get element mastery
    pub fn get_element_mastery(&self, element: &str) -> f64 {
        self.get_data().get(&format!("{}_mastery_current", element))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0)
    }
    
    /// Set element mastery
    pub fn set_element_mastery(&mut self, element: &str, mastery: f64) {
        self.set_data(&format!("{}_mastery_current", element), mastery.into());
    }
}
```

## 📊 **Resource Management Integration**

### 1. Resource Categories

Elemental Mastery System sử dụng các resource categories của Actor Core:

```rust
pub enum ResourceCategory {
    Health,      // HP, lifespan
    Energy,      // Mana, spiritual energy
    Physical,    // Stamina, vitality
    Cultivation, // Qi, dao energy, element mastery
    Special,     // Shield, temporary effects
}
```

### 2. Resource Dependencies

```rust
impl PluginBasedElementMasterySystem {
    fn get_resource_dependencies(&self) -> Vec<String> {
        let mut dependencies = Vec::new();
        
        // Add element-specific dependencies
        for plugin in self.plugin_registry.plugins.values() {
            let element_def = plugin.get_element_definition();
            dependencies.extend(element_def.dependencies);
        }
        
        // Add common dependencies
        dependencies.extend(vec![
            "intelligence".to_string(),
            "wisdom".to_string(),
            "dexterity".to_string(),
            "constitution".to_string(),
            "charisma".to_string(),
        ]);
        
        dependencies.sort();
        dependencies.dedup();
        dependencies
    }
}
```

### 3. Resource Calculation

```rust
impl PluginBasedElementMasterySystem {
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut resources = HashMap::new();
        
        // Calculate mastery for each registered element
        for (element_id, plugin) in &self.plugin_registry.plugins {
            // Calculate base mastery
            let base_mastery = plugin.calculate_base_mastery(actor);
            
            // Calculate decay
            let decay = self.calculate_mastery_decay(actor, element_id);
            
            // Final mastery (with decay applied)
            let final_mastery = (base_mastery - decay).max(0.0);
            
            // Store resources
            resources.insert(format!("{}_mastery_current", element_id), final_mastery);
            resources.insert(format!("{}_mastery_max", element_id), 100.0);
            resources.insert(format!("{}_mastery_decay", element_id), decay);
            
            // Store derived stats
            let derived_stats = plugin.get_derived_stats(actor);
            for (stat_name, stat_value) in derived_stats {
                resources.insert(format!("{}_{}", element_id, stat_name), stat_value);
            }
        }
        
        Ok(resources)
    }
}
```

## 🔄 **Event System Integration**

### 1. Element Mastery Events

```rust
/// Element Mastery Events
#[derive(Debug, Clone)]
pub enum ElementMasteryEvent {
    /// Mastery level changed
    MasteryChanged {
        element_id: String,
        old_mastery: f64,
        new_mastery: f64,
    },
    /// Mastery decay occurred
    MasteryDecay {
        element_id: String,
        decay_amount: f64,
        new_mastery: f64,
    },
    /// Mastery gained from training
    MasteryGained {
        element_id: String,
        gain_amount: f64,
        new_mastery: f64,
    },
    /// Element training completed
    ElementTrainingCompleted {
        element_id: String,
        training_amount: f64,
        new_experience: f64,
    },
}
```

### 2. Event Handler Integration

```rust
impl PluginBasedElementMasterySystem {
    /// Handle element training
    pub async fn handle_element_training(
        &self, 
        actor: &mut Actor, 
        element_id: &str, 
        training_amount: f64
    ) -> ActorCoreResult<()> {
        if let Some(plugin) = self.plugin_registry.get_plugin(element_id) {
            // Get old mastery for event
            let old_mastery = self.calculate_element_mastery(actor, element_id);
            
            // Handle training
            plugin.handle_training(actor, training_amount)?;
            
            // Calculate new mastery
            let new_mastery = self.calculate_element_mastery(actor, element_id);
            let gain_amount = new_mastery - old_mastery;
            
            // Emit events
            if gain_amount > 0.0 {
                self.emit_event(ElementMasteryEvent::MasteryGained {
                    element_id: element_id.to_string(),
                    gain_amount,
                    new_mastery,
                });
            }
            
            self.emit_event(ElementMasteryEvent::ElementTrainingCompleted {
                element_id: element_id.to_string(),
                training_amount,
                new_experience: actor.get_data().get(&format!("{}_experience", element_id))
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
            });
        }
        
        Ok(())
    }
}
```

## ⚙️ **Configuration Integration**

### 1. YAML Configuration

```yaml
# element_mastery_config.yaml
element_mastery_system:
  enabled: true
  decay_config:
    time_decay_rate: 0.5
    opposite_element_decay_multiplier: 2.0
    long_absence_threshold: 7.0
    long_absence_decay_multiplier: 1.5
  
  plugins:
    fire:
      plugin_type: "fire"
      enabled: true
      config:
        name: "Fire Mastery"
        category: "ngu_hanh"
        opposite_elements: ["water"]
        training_methods:
          - id: "fire_meditation"
            name: "Fire Meditation"
            efficiency_multiplier: 1.0
            requirements: []
            rewards:
              - stat_name: "fire_experience"
                amount: 10.0
    
    water:
      plugin_type: "water"
      enabled: true
      config:
        name: "Water Mastery"
        category: "ngu_hanh"
        opposite_elements: ["fire"]
        training_methods:
          - id: "water_meditation"
            name: "Water Meditation"
            efficiency_multiplier: 1.0
            requirements: []
            rewards:
              - stat_name: "water_experience"
                amount: 10.0
```

### 2. Configuration Loading

```rust
impl PluginBasedElementMasterySystem {
    /// Load configuration
    pub async fn load_configuration(&mut self, config_path: &str) -> ActorCoreResult<()> {
        let config = load_yaml_config(config_path).await?;
        
        // Load decay configuration
        if let Some(decay_config) = config.element_mastery_system.decay_config {
            self.decay_config = MasteryDecayConfig {
                time_decay_rate: decay_config.time_decay_rate,
                opposite_element_decay_multiplier: decay_config.opposite_element_decay_multiplier,
                long_absence_threshold: decay_config.long_absence_threshold,
                long_absence_decay_multiplier: decay_config.long_absence_decay_multiplier,
            };
        }
        
        // Load plugins
        if let Some(plugins) = config.element_mastery_system.plugins {
            for (element_id, plugin_config) in plugins {
                if plugin_config.enabled {
                    let plugin = self.create_plugin_from_config(element_id, plugin_config)?;
                    self.register_element_plugin(Box::new(plugin));
                }
            }
        }
        
        Ok(())
    }
}
```

## 🔗 **Element Core Integration**

### 1. Element Mastery Stats Provider

```rust
/// Element Mastery Stats Provider for Element Core
pub struct ElementMasteryStatsProvider {
    /// Element mastery system
    mastery_system: PluginBasedElementMasterySystem,
}

impl ElementMasteryStatsProvider {
    /// Get element mastery stats for Element Core
    pub async fn get_element_mastery_stats(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        self.mastery_system.calculate_resources(actor).await
    }
    
    /// Get specific element mastery
    pub async fn get_element_mastery(&self, actor: &Actor, element: ElementType) -> ActorCoreResult<f64> {
        let element_id = element.to_string();
        let resources = self.get_element_mastery_stats(actor).await?;
        Ok(resources.get(&format!("{}_mastery_current", element_id)).copied().unwrap_or(0.0))
    }
    
    /// Get element derived stats
    pub async fn get_element_derived_stats(&self, actor: &Actor, element: ElementType) -> ActorCoreResult<HashMap<String, f64>> {
        let element_id = element.to_string();
        let resources = self.get_element_mastery_stats(actor).await?;
        
        let mut derived_stats = HashMap::new();
        for (key, value) in resources {
            if key.starts_with(&format!("{}_", element_id)) && !key.ends_with("_mastery_current") {
                let stat_name = key.trim_start_matches(&format!("{}_", element_id));
                derived_stats.insert(stat_name.to_string(), value);
            }
        }
        
        Ok(derived_stats)
    }
}
```

### 2. Element Core Integration

```rust
impl ElementCore {
    /// Initialize with Element Mastery System
    pub fn new_with_mastery_system(mastery_provider: ElementMasteryStatsProvider) -> Self {
        Self {
            element_registry: ElementRegistry::new(),
            probability_calculator: ProbabilityCalculator::new(),
            mastery_provider: Some(mastery_provider),
            // ... other fields
        }
    }
    
    /// Calculate element stats with mastery
    pub async fn calculate_element_stats(&self, actor: &Actor, element: ElementType) -> ActorCoreResult<ElementStats> {
        let mut stats = ElementStats::new();
        
        // Get base element stats
        let base_stats = self.calculate_base_element_stats(actor, element).await?;
        stats.merge(base_stats);
        
        // Get mastery stats if available
        if let Some(ref mastery_provider) = self.mastery_provider {
            let mastery_stats = mastery_provider.get_element_derived_stats(actor, element).await?;
            stats.merge_from_map(mastery_stats);
        }
        
        Ok(stats)
    }
}
```

## 🧪 **Testing Integration**

### 1. Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_element_mastery_system_integration() {
        // Create actor
        let mut actor = Actor::new();
        actor.initialize_element_mastery();
        
        // Create element mastery system
        let mut system = PluginBasedElementMasterySystem::new();
        system.register_element_plugin(Box::new(FireElementPlugin));
        
        // Test resource calculation
        let resources = system.calculate_resources(&actor).await.unwrap();
        assert!(resources.contains_key("fire_mastery_current"));
        assert!(resources.contains_key("fire_mastery_max"));
        
        // Test training
        system.handle_element_training(&mut actor, "fire", 100.0).await.unwrap();
        
        let new_resources = system.calculate_resources(&actor).await.unwrap();
        let fire_mastery = new_resources.get("fire_mastery_current").unwrap();
        assert!(*fire_mastery > 0.0);
    }
    
    #[tokio::test]
    async fn test_actor_core_integration() {
        // Create enhanced hybrid resource manager
        let mut manager = EnhancedHybridResourceManager::new_with_element_mastery().await.unwrap();
        
        // Create actor
        let mut actor = Actor::new();
        actor.initialize_element_mastery();
        
        // Test resource calculation
        let resources = manager.calculate_all_resources(&actor).await.unwrap();
        
        // Check that element mastery resources are present
        assert!(resources.contains_key("fire_mastery_current"));
        assert!(resources.contains_key("water_mastery_current"));
    }
}
```

### 2. Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_full_system_integration() {
        // Initialize all systems
        let mut resource_manager = EnhancedHybridResourceManager::new_with_element_mastery().await.unwrap();
        let element_core = ElementCore::new_with_mastery_system(
            ElementMasteryStatsProvider::new(resource_manager.get_element_mastery_system())
        );
        
        // Create actor
        let mut actor = Actor::new();
        actor.initialize_element_mastery();
        
        // Train fire element
        resource_manager.handle_element_training(&mut actor, "fire", 1000.0).await.unwrap();
        
        // Calculate element stats
        let fire_stats = element_core.calculate_element_stats(&actor, ElementType::Fire).await.unwrap();
        
        // Verify mastery integration
        assert!(fire_stats.get_stat("fire_attack_power").unwrap() > 0.0);
        assert!(fire_stats.get_stat("fire_defense").unwrap() > 0.0);
    }
}
```

## 📈 **Performance Considerations**

### 1. Caching Strategy

```rust
impl PluginBasedElementMasterySystem {
    /// Cached mastery calculation
    pub async fn calculate_element_mastery_cached(&self, actor: &Actor, element_id: &str) -> f64 {
        // Check cache first
        if let Some(cached_value) = self.cache.get(&format!("{}_mastery_{}", actor.id, element_id)) {
            return cached_value;
        }
        
        // Calculate and cache
        let mastery = self.calculate_element_mastery(actor, element_id);
        self.cache.insert(format!("{}_mastery_{}", actor.id, element_id), mastery);
        
        mastery
    }
}
```

### 2. Lazy Loading

```rust
impl ElementPluginRegistry {
    /// Lazy load plugin
    pub async fn get_plugin_lazy(&mut self, element_id: &str) -> ActorCoreResult<&dyn ElementPlugin> {
        if !self.plugins.contains_key(element_id) {
            self.load_plugin_from_config(element_id).await?;
        }
        Ok(self.plugins.get(element_id).unwrap().as_ref())
    }
}
```

## 🚀 **Deployment Guide**

### 1. Dependencies

```toml
# Cargo.toml
[dependencies]
actor-core = { path = "../actor-core" }
element-core = { path = "../element-core" }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
tokio = { version = "1.0", features = ["full"] }
```

### 2. Initialization

```rust
// main.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Actor Core
    let mut resource_manager = EnhancedHybridResourceManager::new_with_element_mastery().await?;
    
    // Load element mastery configuration
    resource_manager.load_element_mastery_config("config/element_mastery_config.yaml").await?;
    
    // Initialize Element Core with mastery integration
    let element_core = ElementCore::new_with_mastery_system(
        ElementMasteryStatsProvider::new(resource_manager.get_element_mastery_system())
    );
    
    // Start game server
    start_game_server(resource_manager, element_core).await?;
    
    Ok(())
}
```

### 3. Configuration Files

```yaml
# config/element_mastery_config.yaml
element_mastery_system:
  enabled: true
  decay_config:
    time_decay_rate: 0.5
    opposite_element_decay_multiplier: 2.0
    long_absence_threshold: 7.0
    long_absence_decay_multiplier: 1.5
  
  plugins:
    fire:
      plugin_type: "fire"
      enabled: true
    water:
      plugin_type: "water"
      enabled: true
    # ... other elements
```

## 🔍 **Debugging & Monitoring**

### 1. Logging

```rust
impl PluginBasedElementMasterySystem {
    /// Debug logging for mastery calculations
    pub fn debug_mastery_calculation(&self, actor: &Actor, element_id: &str) {
        let mastery = self.calculate_element_mastery(actor, element_id);
        let decay = self.calculate_mastery_decay(actor, element_id);
        
        info!(
            element = %element_id,
            mastery = %mastery,
            decay = %decay,
            final_mastery = %(mastery - decay),
            "Element mastery calculation"
        );
    }
}
```

### 2. Metrics

```rust
impl PluginBasedElementMasterySystem {
    /// Get system metrics
    pub fn get_metrics(&self) -> ElementMasteryMetrics {
        ElementMasteryMetrics {
            total_plugins: self.plugin_registry.plugins.len(),
            active_elements: self.get_active_elements().len(),
            cache_hit_rate: self.cache.get_hit_rate(),
            average_calculation_time: self.cache.get_average_calculation_time(),
        }
    }
}
```

## 📋 **Checklist Integration**

### ✅ **Pre-Integration Checklist**

- [ ] Actor Core framework đã được setup
- [ ] Enhanced Hybrid Resource Manager đã được implement
- [ ] Element Core system đã được setup
- [ ] Configuration files đã được tạo
- [ ] Plugin system đã được implement

### ✅ **Integration Checklist**

- [ ] Elemental Mastery System implements SystemResourceCalculator
- [ ] System được register vào Enhanced Hybrid Resource Manager
- [ ] Configuration loading được implement
- [ ] Event system được integrate
- [ ] Element Core integration được setup
- [ ] Testing được implement

### ✅ **Post-Integration Checklist**

- [ ] All tests pass
- [ ] Performance benchmarks meet requirements
- [ ] Configuration validation works
- [ ] Event handling works correctly
- [ ] Documentation is complete
- [ ] Monitoring and debugging tools work

## 🎯 **Next Steps**

1. **Implement Core System**: Bắt đầu implement PluginBasedElementMasterySystem
2. **Create Element Plugins**: Implement FireElementPlugin, WaterElementPlugin, etc.
3. **Integration Testing**: Test integration với Actor Core và Element Core
4. **Performance Optimization**: Optimize caching và calculation performance
5. **Documentation**: Complete documentation và examples

---

**Related Documents:**
- [Elemental Mastery System Design](08_Elemental_Mastery_System_Design.md)
- [Element Core Overview](00_Element_Core_Overview.md)
- [Actor Core Documentation](../../actor-core/README.md)
- [Resource Manager Integration Design](07_Resource_Manager_Integration_Design.md)
