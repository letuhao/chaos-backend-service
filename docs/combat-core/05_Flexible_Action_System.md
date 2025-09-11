# Flexible Action System Design

## Overview

The Flexible Action System is a comprehensive, data-driven approach to defining combat actions in the Chaos MMORPG. Unlike traditional hardcoded combat systems, this system allows for dynamic action creation, modification, and execution through structured data definitions.

## Core Principles

1. **Data-Driven**: Actions are defined as data structures, not hardcoded logic
2. **Modular**: Each aspect of an action is defined in separate, reusable components
3. **Extensible**: New action types can be added without modifying core code
4. **Performance-Oriented**: Hot path optimization for combat calculations
5. **Actor-Based**: Status effects and projectiles are treated as actors for consistency

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Flexible Action System                   │
├─────────────────────────────────────────────────────────────┤
│  Action Definition  │  Event Triggers  │  Status Effects    │
│  ├─ Resource Cost   │  ├─ Conditions   │  ├─ Status Actors  │
│  ├─ Damage Types    │  ├─ Logic Gates  │  ├─ Interactions   │
│  ├─ Cooldowns       │  ├─ Chains       │  └─ Behaviors      │
│  └─ Prerequisites   │  └─ Throttling   │                    │
├─────────────────────────────────────────────────────────────┤
│  Attack Range       │  Projectiles     │  Environment       │
│  ├─ Single Target   │  ├─ Projectile   │  ├─ Interactions   │
│  ├─ Multi Target    │  ├─ Actors       │  ├─ Weather        │
│  ├─ AOE Shapes      │  ├─ Behaviors    │  └─ Terrain        │
│  └─ Line of Sight   │  └─ Collisions   │                    │
├─────────────────────────────────────────────────────────────┤
│  Damage System      │  Power Level     │  Combat Calculator │
│  ├─ Type Generation │  ├─ Components   │  ├─ Hot Path       │
│  ├─ Keywords        │  ├─ Aggregation  │  ├─ Batch Process  │
│  └─ Interactions    │  └─ AI Decisions │  └─ Validation     │
└─────────────────────────────────────────────────────────────┘
```

## File Structure

### Core Action Files
- **action_definition.rs** - Core action structure and metadata
- **event_trigger.rs** - Event trigger conditions and logic
- **status_definition.rs** - Status effects (treated as actors)
- **attack_range.rs** - Targeting, range, and AOE definitions
- **projectile_definition.rs** - Projectiles (treated as actors)
- **environment_interactions.rs** - Environmental effects and interactions

### Supporting Systems
- **damage_system.rs** - Damage type generation with unique keywords
- **power_level_system.rs** - Actor power calculation for AI decisions
- **combat_calculator.rs** - Core damage application logic
- **combat_validator.rs** - Action validation subsystem
- **combat_events.rs** - Event system subsystem

## Key Components

### 1. Action Definition Structure

```rust
pub struct Action {
    // Core identification
    pub action_id: String,
    pub action_name: String,
    pub action_type: ActionType,
    
    // Resource management
    pub resource_consumption: Vec<ResourceConsumption>,
    pub resource_regeneration: Vec<ResourceRegeneration>,
    
    // Damage system
    pub damage_types: Vec<DamageType>,
    pub damage_scaling: HashMap<String, f64>,
    
    // Timing and cooldowns
    pub duration: Duration,
    pub cooldown: ActionCooldown,
    pub channeling: Option<ChannelingConfig>,
    
    // Targeting and range
    pub attack_range: AttackRange,
    pub target_selection: TargetSelection,
    
    // Triggers and conditions
    pub trigger_conditions: Vec<TriggerCondition>,
    pub prerequisites: ActionPrerequisites,
    
    // Effects and interactions
    pub status_effects: Vec<StatusEffectApplication>,
    pub environmental_effects: Vec<EnvironmentalEffect>,
    
    // Advanced mechanics
    pub critical_hit: Option<CriticalHitConfig>,
    pub penetration: Option<PenetrationConfig>,
    pub reflection: Option<ReflectionConfig>,
    pub absorption: Option<AbsorptionConfig>,
    
    // Metadata and scaling
    pub metadata: ActionMetadata,
    pub scaling: ActionScaling,
}
```

### 2. Resource Consumption System

```rust
pub struct ResourceConsumption {
    pub resource_type: String,
    pub consumption_type: ConsumptionType,
    pub base_value: f64,
    pub scaling_factor: f64,
    pub scaling_stat: Option<String>,
    pub conditional_modifiers: Vec<ConditionalModifier>,
}

pub enum ConsumptionType {
    Fixed(f64),                    // Fixed amount
    Percentage(f64),               // Percentage of max resource
    Scaling(String, f64),          // Scales with stat
    Conditional(Vec<Condition>),   // Conditional consumption
}
```

### 3. Damage Type Generation

```rust
pub struct DamageType {
    pub damage_id: String,
    pub damage_keyword: String,        // Unique identifier (e.g., "fire", "ice")
    pub damage_category: DamageCategory,
    pub base_damage: f64,
    pub scaling_stats: Vec<String>,
    pub damage_properties: DamageProperties,
}

pub enum DamageCategory {
    Physical,    // sword, arrow, punch
    Magical,     // fire, ice, lightning
    Elemental,   // earth, water, air
    Spiritual,   // holy, dark, void
    Poison,      // toxin, acid, disease
    Custom(String),
}
```

### 4. Event Trigger System

```rust
pub struct TriggerCondition {
    pub condition_type: ConditionType,
    pub target: ConditionTarget,
    pub value: f64,
    pub operator: ComparisonOperator,
    pub duration: Option<Duration>,
}

pub enum ConditionType {
    Health,           // Health percentage
    Mana,             // Mana percentage
    Distance,         // Distance to target
    Status,           // Status effect present
    Environment,      // Environmental condition
    Time,             // Time-based trigger
    Random,           // Random chance
    Custom(String),   // Custom condition
}

pub enum ConditionTarget {
    Self,
    Target,
    Ally,
    Enemy,
    Environment,
    Custom(String),
}
```

### 5. Status Effects as Actors

```rust
pub struct StatusActor {
    // Inherits from Actor
    pub actor_id: EntityId,
    pub status_type: StatusType,
    pub caster_id: EntityId,
    pub target_id: EntityId,
    
    // Status-specific data
    pub status_data: StatusData,
    pub status_behavior: StatusBehavior,
    pub interaction_rules: Vec<StatusInteraction>,
}

pub struct StatusBehavior {
    pub tick_interval: Duration,
    pub tick_actions: Vec<String>,
    pub on_apply_actions: Vec<String>,
    pub on_remove_actions: Vec<String>,
    pub on_stack_actions: Vec<String>,
    pub decay_behavior: DecayBehavior,
}
```

### 6. Projectiles as Actors

```rust
pub struct ProjectileActor {
    // Inherits from Actor
    pub actor_id: EntityId,
    pub projectile_type: ProjectileType,
    pub caster_id: EntityId,
    pub target_id: Option<EntityId>,
    
    // Projectile-specific data
    pub projectile_data: ProjectileData,
    pub projectile_behavior: ProjectileBehavior,
    pub collision_rules: Vec<CollisionRule>,
}

pub struct ProjectileData {
    pub speed: f64,
    pub acceleration: f64,
    pub max_speed: f64,
    pub lifetime: Duration,
    pub collision_radius: f64,
    pub penetration_count: u32,
    pub bounce_count: u32,
    pub homing_strength: f64,
}
```

### 7. Power Level System

```rust
pub struct PowerLevelSystem {
    pub power_components: HashMap<String, PowerComponent>,
    pub power_aggregation: PowerAggregation,
    pub ai_decision_matrix: DecisionMatrix,
}

pub struct PowerComponent {
    pub component_id: String,
    pub component_type: PowerComponentType,
    pub base_power: i64,
    pub scaling_factor: f64,
    pub dependencies: Vec<String>,
}

pub enum PowerComponentType {
    Equipment,      // Armor, weapon power
    Stats,          // Primary/secondary stats
    Skills,         // Skill levels
    Cultivation,    // Cultivation realm
    Talents,        // Talent points
    Buffs,          // Active buffs
    Resources,      // Resource levels
    Custom(String),
}
```

### 8. Combat Calculator

```rust
pub struct CombatCalculator {
    pub damage_calculator: DamageCalculator,
    pub shield_calculator: ShieldCalculator,
    pub resistance_calculator: ResistanceCalculator,
    pub critical_calculator: CriticalCalculator,
    pub hot_path_optimizer: HotPathCalculator,
}

pub struct HotPathCalculator {
    pub precomputed_values: HashMap<String, f64>,
    pub cached_results: HashMap<String, f64>,
    pub branch_optimization: bool,
    pub latency_budget: Duration,
}
```

## Example: Fire Punch Action

```rust
let fire_punch = Action {
    action_id: "fire_punch".to_string(),
    action_name: "Fire Punch".to_string(),
    action_type: ActionType::Attack,
    
    resource_consumption: vec![
        ResourceConsumption {
            resource_type: "mana".to_string(),
            consumption_type: ConsumptionType::Fixed(10.0),
            base_value: 10.0,
            scaling_factor: 0.0,
            scaling_stat: None,
            conditional_modifiers: vec![],
        }
    ],
    
    damage_types: vec![
        DamageType {
            damage_id: "fire_damage".to_string(),
            damage_keyword: "fire".to_string(),
            damage_category: DamageCategory::Magical,
            base_damage: 50.0,
            scaling_stats: vec!["strength".to_string(), "fire_attack".to_string()],
            damage_properties: DamageProperties {
                can_crit: true,
                can_penetrate: false,
                can_reflect: true,
                can_absorb: true,
            },
        }
    ],
    
    duration: Duration::from_millis(500),
    cooldown: ActionCooldown {
        global_cooldown: Duration::from_millis(1000),
        category_cooldown: None,
        specific_cooldown: None,
        resource_cooldown: None,
    },
    
    attack_range: AttackRange {
        min_range: 0.0,
        max_range: 2.0,
        requires_line_of_sight: true,
        aoe_config: None,
    },
    
    target_selection: TargetSelection {
        target_type: TargetType::Enemy,
        target_count: 1,
        target_filters: vec![],
    },
    
    trigger_conditions: vec![
        TriggerCondition {
            condition_type: ConditionType::Health,
            target: ConditionTarget::Self,
            value: 50.0,
            operator: ComparisonOperator::GreaterThan,
            duration: None,
        }
    ],
    
    status_effects: vec![
        StatusEffectApplication {
            effect_id: "burning".to_string(),
            application_chance: 0.3,
            effect_duration: Duration::from_secs(5),
            effect_stacks: Some(1),
            application_conditions: vec![],
        }
    ],
    
    critical_hit: Some(CriticalHitConfig {
        base_chance: 0.1,
        chance_scaling: vec!["crit_rate".to_string()],
        damage_multiplier: 2.0,
        damage_scaling: vec!["crit_damage".to_string()],
    }),
    
    metadata: ActionMetadata {
        description: "A powerful fire-imbued punch".to_string(),
        icon_id: "fire_punch_icon".to_string(),
        animation_id: "fire_punch_anim".to_string(),
        sound_effect_id: "fire_punch_sound".to_string(),
        visual_effects: vec!["fire_trail".to_string(), "impact_spark".to_string()],
        level_requirement: 5,
        class_requirement: vec!["warrior".to_string(), "mage".to_string()],
        race_requirement: vec![],
    },
    
    scaling: ActionScaling {
        level_scaling: 1.2,
        stat_scaling: HashMap::from([
            ("strength".to_string(), 1.5),
            ("fire_attack".to_string(), 2.0),
        ]),
        equipment_scaling: HashMap::from([
            ("fire_weapon".to_string(), 1.3),
        ]),
        talent_scaling: HashMap::from([
            ("fire_mastery".to_string(), 1.1),
        ]),
    },
};
```

## Benefits

### 1. **Flexibility**
- Actions can be modified without code changes
- New action types can be added through data
- Complex interactions can be defined declaratively

### 2. **Performance**
- Hot path optimization for combat calculations
- Batch processing for multiple actions
- Caching of frequently used values

### 3. **Maintainability**
- Clear separation of concerns
- Modular architecture
- Easy to test and debug

### 4. **Extensibility**
- Plugin system for new damage types
- Custom status effects and projectiles
- Environmental interaction system

### 5. **Consistency**
- Status effects and projectiles as actors
- Unified event system
- Consistent power level calculation

## Integration Points

### Resource Manager Integration
- Actions consume and regenerate resources
- Resource changes trigger recalculation
- Caching for performance

### Precomputed Inputs (Combat Resources)
- Actions and calculators should use pre-calculated Power/Defense points from the Enhanced Resource Manager when available.
- Deterministic lookup by `(actor_id, damage_type)` from the combat resource cache.
- True Damage: use policy defined in Damage Application (bypasses shields and secondary; absolute immunities only).

### Combat Core Integration
- Damage application system
- Shield and resistance calculations
- Event system integration

### AI System Integration
- Power level-based decisions
- Personality trait influence
- Situational awareness

## Next Steps

1. **Implementation**: Create the core file structure
2. **Testing**: Develop comprehensive test cases
3. **Performance**: Optimize hot path calculations
4. **Integration**: Connect with existing systems
5. **Documentation**: Complete API documentation

This flexible action system provides a solid foundation for a dynamic, extensible combat system that can grow with the game's needs while maintaining high performance.
