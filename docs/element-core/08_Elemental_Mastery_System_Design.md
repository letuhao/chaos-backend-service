# Elemental Mastery System Design

## Tổng Quan (Overview)

Elemental Mastery System là một hệ thống tu luyện độc lập trong Chaos World MMORPG, được thiết kế như một cultivation system kế thừa từ Actor Core. Hệ thống này cho phép nhân vật phát triển tinh thông (mastery) với các nguyên tố khác nhau thông qua việc tu luyện, training và sử dụng kỹ năng.

### Mục Tiêu Chính (Main Goals)

1. **Hệ Thống Tu Luyện Độc Lập**: Elemental Mastery hoạt động song song với các hệ thống tu luyện khác (hỏa pháp sư, luyện khí, ma pháp, etc.)
2. **Plugin-Based Architecture**: Mỗi element được implement như một plugin độc lập, dễ dàng thêm mới và tùy chỉnh
3. **Decay System**: Mastery sẽ giảm dần theo thời gian nếu không được sử dụng, tạo ra động lực tu luyện liên tục
4. **Element Interactions**: Hệ thống tương sinh tương khắc ảnh hưởng đến việc tu luyện và sử dụng elements
5. **Integration với Element Core**: Cung cấp derived stats cho Element Core system

## Kiến Trúc Hệ Thống (System Architecture)

### 1. Plugin-Based Design

```rust
/// Element Plugin Trait
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
    fn handle_training(&self, actor: &mut Actor, training_amount: f64) -> ActorCoreResult<()>;
    
    /// Get derived stats for this element
    fn get_derived_stats(&self, actor: &Actor) -> HashMap<String, f64>;
    
    /// Get training methods for this element
    fn get_training_methods(&self) -> Vec<TrainingMethod>;
    
    /// Get element interactions
    fn get_element_interactions(&self) -> HashMap<String, ElementInteraction>;
}
```

### 2. Element Plugin Registry

```rust
/// Element Plugin Registry
pub struct ElementPluginRegistry {
    /// Registered element plugins
    plugins: HashMap<String, Box<dyn ElementPlugin>>,
    /// Element definitions cache
    element_definitions: HashMap<String, ElementDefinition>,
    /// Element interactions matrix
    element_interactions: HashMap<(String, String), ElementInteraction>,
}
```

### 3. Integration với Actor Core

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
```

## Cơ Chế Tu Luyện (Cultivation Mechanics)

### 1. Mastery Progression

#### A. Experience-Based System
- **Fire Experience**: Tăng từ việc sử dụng fire skills, training, meditation
- **Training Time**: Thời gian thực tế dành cho việc tu luyện element
- **Formula**: `Mastery = (Experience / 1000) + (Training Time / 100)`

#### B. Mastery Calculation
```rust
fn calculate_base_mastery(&self, actor: &Actor) -> f64 {
    let experience = actor.get_data().get("fire_experience")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let training_time = actor.get_data().get("fire_training_time")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    (experience / 1000.0 + training_time / 100.0).min(100.0)
}
```

### 2. Decay System

#### A. Time-Based Decay
- **Base Decay Rate**: 0.5% per day
- **Long Absence Threshold**: 7 days
- **Long Absence Multiplier**: 1.5x decay after 7 days

#### B. Opposite Element Decay
- **Fire vs Water**: Sử dụng Water element sẽ tăng decay rate của Fire mastery
- **Light vs Dark**: Sử dụng Dark element sẽ tăng decay rate của Light mastery
- **Decay Multiplier**: 2.0x khi sử dụng opposite element

#### C. Decay Calculation
```rust
fn calculate_mastery_decay(&self, actor: &Actor, element_id: &str) -> f64 {
    let base_decay = self.decay_config.time_decay_rate;
    let decay_multiplier = plugin.calculate_decay_rate(actor);
    
    // Check for opposite element usage
    let opposite_elements = plugin.get_opposite_elements();
    let opposite_element_usage = self.calculate_opposite_element_usage(actor, &opposite_elements);
    let opposite_decay_multiplier = if opposite_element_usage > 0.0 {
        self.decay_config.opposite_element_decay_multiplier
    } else {
        1.0
    };
    
    base_decay * decay_multiplier * opposite_decay_multiplier
}
```

### 3. Training Methods

#### A. Fire Element Training Methods
```rust
fn get_training_methods(&self) -> Vec<TrainingMethod> {
    vec![
        TrainingMethod {
            id: "fire_meditation".to_string(),
            name: "Fire Meditation".to_string(),
            description: "Meditate with fire element to increase mastery".to_string(),
            efficiency_multiplier: 1.0,
            requirements: vec![
                TrainingRequirement {
                    stat_name: "fire_mastery".to_string(),
                    min_value: 0.0,
                    description: "No minimum mastery required".to_string(),
                },
            ],
            rewards: vec![
                TrainingReward {
                    stat_name: "fire_experience".to_string(),
                    amount: 10.0,
                    description: "Gain fire experience".to_string(),
                },
            ],
        },
        TrainingMethod {
            id: "fire_combat_training".to_string(),
            name: "Fire Combat Training".to_string(),
            description: "Practice fire-based combat techniques".to_string(),
            efficiency_multiplier: 1.5,
            requirements: vec![
                TrainingRequirement {
                    stat_name: "fire_mastery".to_string(),
                    min_value: 10.0,
                    description: "Requires 10% fire mastery".to_string(),
                },
            ],
            rewards: vec![
                TrainingReward {
                    stat_name: "fire_experience".to_string(),
                    amount: 15.0,
                    description: "Gain fire experience".to_string(),
                },
            ],
        },
    ]
}
```

#### B. Training Efficiency
- **Base Efficiency**: 1.0x
- **Method Bonuses**: 1.5x for combat training, 2.0x for advanced methods
- **Location Bonuses**: Volcano (+2.0x), Fire Temple (+1.5x)
- **Equipment Bonuses**: Fire Staff (+1.2x), Fire Robes (+1.1x)

## Element Interactions

### 1. Ngũ Hành (Five Elements) Interactions

#### A. Tương Sinh (Generating)
- **Fire → Earth**: Fire generates Earth (0.6x multiplier)
- **Earth → Metal**: Earth generates Metal (0.6x multiplier)
- **Metal → Water**: Metal generates Water (0.6x multiplier)
- **Water → Wood**: Water generates Wood (0.6x multiplier)
- **Wood → Fire**: Wood generates Fire (0.6x multiplier)

#### B. Tương Khắc (Overcoming)
- **Fire → Water**: Fire is overcome by Water (1.5x multiplier)
- **Water → Fire**: Water is overcome by Fire (1.5x multiplier)
- **Earth → Wood**: Earth is overcome by Wood (1.5x multiplier)
- **Wood → Earth**: Wood is overcome by Earth (1.5x multiplier)
- **Metal → Fire**: Metal is overcome by Fire (1.5x multiplier)

### 2. Light/Dark Interactions

#### A. Light vs Dark
- **Light → Dark**: Light is overcome by Dark (2.0x multiplier)
- **Dark → Light**: Dark is overcome by Light (2.0x multiplier)

### 3. Advanced Element Interactions

#### A. Life/Death
- **Life → Death**: Life is overcome by Death (1.8x multiplier)
- **Death → Life**: Death is overcome by Life (1.8x multiplier)

#### B. Time/Space
- **Time → Space**: Time is overcome by Space (1.7x multiplier)
- **Space → Time**: Space is overcome by Time (1.7x multiplier)

## Derived Stats Integration

### 1. Element-Specific Derived Stats

#### A. Fire Element Stats
```rust
fn get_derived_stats(&self, actor: &Actor) -> HashMap<String, f64> {
    let mastery = self.calculate_base_mastery(actor);
    let intelligence = actor.get_data().get("intelligence")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    let wisdom = actor.get_data().get("wisdom")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    let dexterity = actor.get_data().get("dexterity")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    
    HashMap::from([
        ("fire_attack_power".to_string(), mastery * 0.1 + intelligence * 0.05),
        ("fire_defense".to_string(), mastery * 0.08 + wisdom * 0.03),
        ("fire_crit_rate".to_string(), mastery * 0.02 + dexterity * 0.01),
        ("fire_accuracy".to_string(), mastery * 0.015 + dexterity * 0.02),
        ("fire_buff_resistance".to_string(), mastery * 0.01),
        ("fire_buff_reception".to_string(), mastery * 0.01),
    ])
}
```

#### B. Water Element Stats
```rust
fn get_derived_stats(&self, actor: &Actor) -> HashMap<String, f64> {
    let mastery = self.calculate_base_mastery(actor);
    let intelligence = actor.get_data().get("intelligence")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    let wisdom = actor.get_data().get("wisdom")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    let constitution = actor.get_data().get("constitution")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    
    HashMap::from([
        ("water_attack_power".to_string(), mastery * 0.1 + intelligence * 0.05),
        ("water_defense".to_string(), mastery * 0.08 + wisdom * 0.03),
        ("water_crit_rate".to_string(), mastery * 0.02 + constitution * 0.01),
        ("water_accuracy".to_string(), mastery * 0.015 + constitution * 0.02),
        ("water_buff_resistance".to_string(), mastery * 0.01),
        ("water_buff_reception".to_string(), mastery * 0.01),
    ])
}
```

### 2. Integration với Element Core

#### A. Element Mastery Stats Provider
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
        let element_id = format!("{}_mastery", element.to_string());
        let resources = self.get_element_mastery_stats(actor).await?;
        Ok(resources.get(&format!("{}_current", element_id)).copied().unwrap_or(0.0))
    }
}
```

## Configuration System

### 1. YAML Configuration

#### A. Element Plugins Configuration
```yaml
# element_plugins.yaml
elements:
  fire:
    plugin_type: "fire"
    enabled: true
    
  water:
    plugin_type: "water"
    enabled: true
    
  earth:
    plugin_type: "custom"
    enabled: true
    config:
      name: "Earth Mastery"
      category: "ngu_hanh"
      opposite_elements: ["wood"]
      training_methods:
        - id: "earth_meditation"
          name: "Earth Meditation"
          efficiency_multiplier: 1.0
          requirements: []
          rewards:
            - stat_name: "earth_experience"
              amount: 10.0
      interactions:
        - target_element: "wood"
          interaction_type: "overcoming"
          multiplier: 1.5
        - target_element: "fire"
          interaction_type: "generating"
          multiplier: 0.6
```

#### B. Decay Configuration
```yaml
# mastery_decay.yaml
decay_config:
  time_decay_rate: 0.5  # 0.5% per day
  opposite_element_decay_multiplier: 2.0  # 2x decay when using opposite
  long_absence_threshold: 7.0  # 7 days
  long_absence_decay_multiplier: 1.5  # 1.5x decay after long absence
```

### 2. Runtime Configuration Loading

```rust
impl PluginBasedElementMasterySystem {
    /// Load plugins from configuration
    pub fn load_plugins_from_config(&mut self, config_path: &str) -> ActorCoreResult<()> {
        let config = load_yaml_config(config_path)?;
        
        for (element_id, element_config) in config.elements {
            let plugin = self.create_plugin_from_config(element_id, element_config)?;
            self.register_element_plugin(Box::new(plugin));
        }
        
        Ok(())
    }
}
```

## Event System

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
    /// Opposite element usage detected
    OppositeElementUsage {
        element_id: String,
        opposite_element_id: String,
        decay_multiplier: f64,
    },
}
```

### 2. Event Handler

```rust
/// Element Mastery Event Handler
pub struct ElementMasteryEventHandler {
    /// Event listeners
    listeners: Vec<Box<dyn Fn(ElementMasteryEvent) + Send + Sync>>,
}

impl ElementMasteryEventHandler {
    /// Create a new event handler
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
    
    /// Add an event listener
    pub fn add_listener<F>(&mut self, listener: F)
    where
        F: Fn(ElementMasteryEvent) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }
    
    /// Emit an event
    pub fn emit(&self, event: ElementMasteryEvent) {
        for listener in &self.listeners {
            listener(event.clone());
        }
    }
}
```

## Performance Considerations

### 1. Caching Strategy

#### A. Mastery Calculation Cache
```rust
pub struct MasteryCalculationCache {
    /// Cached mastery values
    mastery_cache: HashMap<String, f64>,
    /// Cache timestamps
    cache_timestamps: HashMap<String, SystemTime>,
    /// Cache TTL
    cache_ttl: Duration,
}
```

#### B. Derived Stats Cache
```rust
pub struct DerivedStatsCache {
    /// Cached derived stats
    stats_cache: HashMap<String, HashMap<String, f64>>,
    /// Cache invalidation triggers
    invalidation_triggers: HashMap<String, Vec<String>>,
}
```

### 2. Lazy Loading

#### A. Plugin Lazy Loading
```rust
impl ElementPluginRegistry {
    /// Load plugin only when needed
    pub fn get_plugin_lazy(&mut self, element_id: &str) -> ActorCoreResult<&dyn ElementPlugin> {
        if !self.plugins.contains_key(element_id) {
            self.load_plugin_from_config(element_id)?;
        }
        Ok(self.plugins.get(element_id).unwrap().as_ref())
    }
}
```

## Testing Strategy

### 1. Unit Tests

#### A. Plugin Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fire_plugin_mastery_calculation() {
        let plugin = FireElementPlugin;
        let mut actor = Actor::new();
        actor.set_data("fire_experience", 1000.0.into());
        actor.set_data("fire_training_time", 100.0.into());
        
        let mastery = plugin.calculate_base_mastery(&actor);
        assert_eq!(mastery, 2.0); // (1000/1000) + (100/100) = 2.0
    }
    
    #[test]
    fn test_fire_plugin_decay_calculation() {
        let plugin = FireElementPlugin;
        let mut actor = Actor::new();
        actor.set_data("fire_last_used", (SystemTime::now() - Duration::from_secs(86400 * 8)).into());
        
        let decay_rate = plugin.calculate_decay_rate(&actor);
        assert_eq!(decay_rate, 1.5); // Long absence multiplier
    }
}
```

### 2. Integration Tests

#### A. System Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_element_mastery_system_integration() {
        let mut system = PluginBasedElementMasterySystem::new();
        system.register_element_plugin(Box::new(FireElementPlugin));
        
        let mut actor = Actor::new();
        system.handle_element_training(&mut actor, "fire", 100.0).await.unwrap();
        
        let mastery = system.calculate_element_mastery(&actor, "fire");
        assert!(mastery > 0.0);
    }
}
```

## Future Enhancements

### 1. Advanced Features

#### A. Element Fusion
- **Fire + Water = Steam**: Tạo ra element mới từ việc kết hợp
- **Light + Dark = Balance**: Element cân bằng từ opposite elements
- **Time + Space = Void**: Element không gian-thời gian

#### B. Element Mastery Trees
- **Specialization Paths**: Fire Mage, Fire Warrior, Fire Healer
- **Cross-Element Synergies**: Fire + Earth = Lava, Water + Air = Storm
- **Mastery Bonuses**: Unlock special abilities at high mastery levels

#### C. Element Locations
- **Elemental Shrines**: Special locations for training specific elements
- **Elemental Dungeons**: Dungeons with element-specific challenges
- **Elemental Events**: World events that boost specific element training

### 2. Social Features

#### A. Element Guilds
- **Fire Guild**: Guild focused on fire element mastery
- **Elemental Competitions**: PvP competitions based on element mastery
- **Mentorship System**: High mastery players can mentor others

#### B. Element Trading
- **Elemental Essences**: Tradeable items that boost element mastery
- **Mastery Transfer**: Temporary transfer of mastery between players
- **Elemental Artifacts**: Rare items that provide permanent mastery bonuses

## Conclusion

Elemental Mastery System cung cấp một framework linh hoạt và mở rộng cho việc phát triển tinh thông nguyên tố trong Chaos World MMORPG. Với kiến trúc plugin-based, hệ thống có thể dễ dàng mở rộng với các element mới và cơ chế tu luyện phức tạp hơn.

Hệ thống tích hợp chặt chẽ với Element Core để cung cấp derived stats cho combat system, đồng thời duy trì tính độc lập như một cultivation system riêng biệt trong Actor Core framework.

---

**Related Documents:**
- [Element Core Overview](00_Element_Core_Overview.md)
- [Element Types Comprehensive Design](03_Element_Types_Comprehensive_Design.md)
- [Status Effect System Design](04_Status_Effect_System_Design.md)
- [Implementation Notes](06_Implementation_Notes.md)
- [Resource Manager Integration Design](07_Resource_Manager_Integration_Design.md)
