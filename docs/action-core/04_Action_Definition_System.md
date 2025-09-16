# Action Definition System

## 📋 **Tổng Quan**

Action Definition System là core component của Action Core, cung cấp unified interface để định nghĩa và quản lý tất cả các actions trong game. Hệ thống này đảm bảo type safety, flexibility và performance cho action definitions.

## 🎯 **Core Principles**

### **1. Unified Interface**
- **Single Interface**: Tất cả actions implement cùng một interface
- **Type Safety**: Strong typing cho action parameters
- **Flexibility**: Dễ dàng extend cho action types mới
- **Validation**: Built-in validation cho action definitions

### **2. Resource Management Integration**
- **Resource Requirements**: Actions định nghĩa resource requirements
- **Resource Consumption**: Real-time resource consumption
- **Resource Validation**: Validate resource availability
- **Resource Regeneration**: Handle resource regeneration

### **3. Timing Management**
- **Execution Duration**: Actions có thời gian thực hiện
- **Interrupt Mechanism**: Có thể bị interrupt theo điều kiện
- **Cooldown Management**: Quản lý cooldown với điều kiện phức tạp
- **Timing Validation**: Validate timing constraints

## 🏗️ **Architecture**

### **Core Components**

```rust
pub struct ActionDefinitionSystem {
    // Core components
    action_interface: ActionInterface,
    action_builder: ActionBuilder,
    action_validator: ActionValidator,
    action_registry: ActionRegistry,
    
    // Resource management
    resource_validator: ResourceValidator,
    resource_consumer: ResourceConsumer,
    resource_regenerator: ResourceRegenerator,
    
    // Timing management
    execution_duration_manager: ExecutionDurationManager,
    interrupt_manager: InterruptManager,
    cooldown_manager: CooldownManager,
    
    // Multi-element support (NEW)
    multi_element_validator: MultiElementActionValidator,
    element_synergy_calculator: MultiElementSynergyCalculator,
    domain_activation_validator: DomainActivationValidator,
    
    // Life force management (NEW)
    life_force_validator: LifeForceValidator,
    life_force_consumer: LifeForceConsumer,
    life_force_regenerator: LifeForceRegenerator,
    
    // World area spawning (NEW)
    world_area_spawner: WorldAreaSpawner,
    world_core_integration: WorldCoreIntegration,
    
    // Forbidden spell support (NEW)
    forbidden_spell_validator: ForbiddenSpellValidator,
    forbidden_spell_effects: ForbiddenSpellEffectManager,
    
    // Configuration
    config: ActionDefinitionConfig,
}
```

## 🔧 **Core Components**

### **1. Action Interface**

```rust
/// Unified interface cho tất cả actions
pub trait Action {
    /// Action metadata
    fn get_metadata(&self) -> &ActionMetadata;
    
    /// Action category
    fn get_category(&self) -> ActionCategory;
    
    /// Action type
    fn get_type(&self) -> ActionType;
    
    /// Resource requirements
    fn get_resource_requirements(&self) -> &[ResourceRequirement];
    
    /// Execution duration
    fn get_execution_duration(&self) -> DurationRange;
    
    /// Cooldown duration
    fn get_cooldown_duration(&self) -> CooldownConfig;
    
    /// Interrupt conditions
    fn get_interrupt_conditions(&self) -> &[InterruptCondition];
    
    /// Execution conditions
    fn get_execution_conditions(&self) -> &[ExecutionCondition];
    
    /// Target requirements
    fn get_target_requirements(&self) -> &TargetRequirements;
    
    /// Effects
    fn get_effects(&self) -> &[ActionEffect];
    
    /// Validation
    fn validate(&self, context: &ActionContext) -> ValidationResult;
    
    /// Execution
    fn execute(&self, context: &mut ActionContext) -> ActionResult;
}

/// Action metadata
pub struct ActionMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_id: String,
    pub animation_id: String,
    pub sound_effect_id: String,
    pub visual_effects: Vec<String>,
    pub level_requirement: u32,
    pub class_requirements: Vec<String>,
    pub race_requirements: Vec<String>,
}

/// Action category
pub enum ActionCategory {
    Combat,
    Lifestyle,
    Social,
    Crafting,
    Movement,
    Special,
}

/// Action type
pub enum ActionType {
    Attack,
    Skill,
    Movement,
    Interaction,
    Special,
}
```

### **2. Resource Requirements**

```rust
/// Resource requirement cho action
pub struct ResourceRequirement {
    pub resource_type: ResourceType,
    pub consumption_type: ConsumptionType,
    pub base_value: f64,
    pub scaling_factor: f64,
    pub scaling_stat: Option<String>,
    pub conditional_modifiers: Vec<ConditionalModifier>,
}

/// Resource consumption type
pub enum ConsumptionType {
    Fixed(f64),                    // Fixed amount
    Percentage(f64),               // Percentage of max resource
    Scaling(String, f64),          // Scales with stat
    Conditional(Vec<Condition>),   // Conditional consumption
}

/// Resource type
pub enum ResourceType {
    Health,
    Mana,
    Stamina,
    Qi,
    Custom(String),
}

/// Conditional modifier
pub struct ConditionalModifier {
    pub condition: String,         // "target.hp < 0.5"
    pub modifier: f64,             // 0.5 = 50% reduction
    pub modifier_type: ModifierType,
}

pub enum ModifierType {
    Multiply,      // Multiply base value
    Add,           // Add to base value
    Set,           // Set to specific value
}
```

### **3. Timing Management**

```rust
/// Execution duration configuration
pub struct DurationRange {
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub base_duration: Duration,
    pub scaling_stats: Vec<String>,
    pub scaling_factor: f64,
}

/// Cooldown configuration
pub struct CooldownConfig {
    pub base_cooldown: Duration,
    pub min_cooldown: Duration,
    pub max_cooldown: Duration,
    pub scaling_stats: Vec<String>,
    pub scaling_factor: f64,
    pub cooldown_conditions: Vec<CooldownCondition>,
    pub interrupt_affects_cooldown: bool,
}

/// Cooldown condition
pub struct CooldownCondition {
    pub condition: String,         // "target.hp < 0.3"
    pub cooldown_modifier: f64,    // 0.5 = 50% cooldown reduction
    pub condition_type: CooldownConditionType,
}

pub enum CooldownConditionType {
    TargetHealth,
    TargetStatus,
    SelfStatus,
    Environment,
    Custom(String),
}

/// Interrupt condition
pub struct InterruptCondition {
    pub condition: String,         // "target.hp < 0.1"
    pub interrupt_chance: f64,     // 0.8 = 80% chance to interrupt
    pub interrupt_type: InterruptType,
    pub condition_type: InterruptConditionType,
}

pub enum InterruptType {
    Hard,           // Immediate interrupt
    Soft,           // Graceful interrupt
    Conditional,    // Conditional interrupt
}

pub enum InterruptConditionType {
    TargetHealth,
    TargetStatus,
    SelfStatus,
    Environment,
    Custom(String),
}

/// Execution condition - điều kiện để action có thể được thực hiện
pub struct ExecutionCondition {
    pub condition: String,         // "self.hp < 0.5" (dưới 50% HP)
    pub condition_type: ExecutionConditionType,
    pub required: bool,            // true = bắt buộc phải thỏa mãn, false = optional
    pub error_message: String,     // "Action requires HP below 50%"
    pub condition_value: Option<f64>, // Giá trị cụ thể nếu cần
}

pub enum ExecutionConditionType {
    SelfHealth,         // "self.hp < 0.5"
    SelfMana,           // "self.mana < 0.3"
    SelfStamina,        // "self.stamina > 0.8"
    SelfStatus,         // "self.has_status('berserk')"
    TargetHealth,       // "target.hp > 0.2"
    TargetStatus,       // "target.has_status('stunned')"
    Environment,        // "environment.is_night"
    ResourceAvailable,  // "resource.available('mana') > 100"
    CooldownReady,      // "cooldown.ready('fireball')"
    LevelRequirement,   // "self.level >= 10"
    ClassRequirement,   // "self.class == 'warrior'"
    RaceRequirement,    // "self.race == 'human'"
    EquipmentRequirement, // "self.equipped('sword')"
    LocationRequirement,  // "self.location == 'dungeon'"
    TimeRequirement,    // "time.hour >= 18" (sau 6 PM)
    WeatherRequirement, // "weather.is_raining"
    Custom(String),     // Custom condition
}
```

### **4. Targeting System**

```rust
/// Target requirements
pub struct TargetRequirements {
    pub target_type: TargetType,
    pub target_count: TargetCount,
    pub target_filters: Vec<TargetFilter>,
    pub range_requirements: RangeRequirements,
    pub line_of_sight: bool,
    pub target_validation: TargetValidation,
}

/// Target type
pub enum TargetType {
    None,           // No target required
    Self,           // Self target
    Single,         // Single target
    Multiple,       // Multiple targets
    Random,         // Random target
    AOE,            // Area of Effect
    MultipleAOE,    // Multiple AOE
    Projectile,     // Projectile
    MultipleProjectile, // Multiple projectiles
}

/// Target count
pub struct TargetCount {
    pub min_count: u32,
    pub max_count: u32,
    pub base_count: u32,
    pub scaling_stats: Vec<String>,
    pub scaling_factor: f64,
}

/// Range requirements
pub struct RangeRequirements {
    pub min_range: f64,
    pub max_range: f64,
    pub base_range: f64,
    pub scaling_stats: Vec<String>,
    pub scaling_factor: f64,
}

/// Target filter
pub struct TargetFilter {
    pub filter_type: TargetFilterType,
    pub filter_value: String,
    pub filter_operator: FilterOperator,
}

pub enum TargetFilterType {
    Race,
    Class,
    Level,
    Status,
    Faction,
    Custom(String),
}

pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    NotContains,
}
```

### **5. Effect System**

```rust
/// Action effect
pub struct ActionEffect {
    pub effect_id: String,
    pub effect_type: EffectType,
    pub magnitude: MagnitudeRange,
    pub trigger_chance: f64,
    pub conditions: Vec<EffectCondition>,
    pub duration: DurationRange,
    pub target_type: EffectTargetType,
    pub effect_data: EffectData,
}

/// Effect type
pub enum EffectType {
    Damage,
    Heal,
    StatusEffect,
    SpawnProjectile,
    SpawnDamageArea,
    ResourceModification,
    StatModification,
    Custom(String),
}

/// Magnitude range
pub struct MagnitudeRange {
    pub min_magnitude: f64,
    pub max_magnitude: f64,
    pub base_magnitude: f64,
    pub scaling_stats: Vec<String>,
    pub scaling_factor: f64,
}

/// Effect condition
pub struct EffectCondition {
    pub condition: String,         // "target.hp < 0.5"
    pub condition_type: EffectConditionType,
    pub condition_value: f64,
}

pub enum EffectConditionType {
    TargetHealth,
    TargetStatus,
    SelfStatus,
    Environment,
    Random,
    Custom(String),
}

/// Effect target type
pub enum EffectTargetType {
    Self,
    Target,
    AllTargets,
    AllAllies,
    AllEnemies,
    Area,
    Custom(String),
}

/// Effect data
pub enum EffectData {
    Damage {
        damage_type: String,
        damage_element: String,
        can_crit: bool,
        can_penetrate: bool,
    },
    Heal {
        heal_type: String,
        heal_element: String,
    },
    StatusEffect {
        status_id: String,
        status_type: String,
        status_duration: Duration,
        status_stacks: u32,
    },
    SpawnProjectile {
        projectile_id: String,
        projectile_count: u32,
        projectile_speed: f64,
        projectile_lifetime: Duration,
    },
    SpawnDamageArea {
        area_id: String,
        area_size: f64,
        area_duration: Duration,
        area_damage: f64,
    },
    ResourceModification {
        resource_type: ResourceType,
        modification_type: ResourceModificationType,
        modification_value: f64,
    },
    StatModification {
        stat_name: String,
        modification_type: StatModificationType,
        modification_value: f64,
        modification_duration: Duration,
    },
}
```

## 🚀 **Action Builder**

```rust
pub struct ActionBuilder {
    action_registry: ActionRegistry,
    validation_rules: HashMap<String, ValidationRule>,
    resource_validator: ResourceValidator,
    timing_validator: TimingValidator,
    execution_condition_validator: ExecutionConditionValidator,
}

impl ActionBuilder {
    pub fn create_action<T: Action + 'static>(
        &mut self,
        action_data: ActionData
    ) -> Result<Box<dyn Action>, ActionError> {
        // Validate action data
        self.validate_action_data(&action_data)?;
        
        // Create action instance
        let action = T::from_data(action_data)?;
        
        // Register action
        self.action_registry.register(action.id(), Box::new(action));
        
        Ok(action)
    }
    
    fn validate_action_data(&self, action_data: &ActionData) -> Result<(), ActionError> {
        // Validate execution conditions
        for condition in &action_data.execution_conditions {
            self.execution_condition_validator.validate_condition(condition)?;
        }
        
        // Validate other action data...
        Ok(())
    }
}

/// Execution Condition Validator
pub struct ExecutionConditionValidator {
    condition_evaluator: ConditionEvaluator,
    stats_integration: DerivedStatsIntegration,
}

impl ExecutionConditionValidator {
    pub fn new() -> Self {
        Self {
            condition_evaluator: ConditionEvaluator::new(),
            stats_integration: DerivedStatsIntegration::new(),
        }
    }
    
    pub fn validate_condition(&self, condition: &ExecutionCondition) -> Result<(), ActionError> {
        // Validate condition syntax
        self.validate_condition_syntax(condition)?;
        
        // Validate condition type
        self.validate_condition_type(condition)?;
        
        Ok(())
    }
    
    pub async fn check_execution_conditions(
        &self,
        conditions: &[ExecutionCondition],
        actor: &Actor,
        context: &ActionContext
    ) -> Result<ExecutionConditionResult, ActionError> {
        let mut result = ExecutionConditionResult::new();
        
        for condition in conditions {
            let condition_met = self.evaluate_execution_condition(condition, actor, context).await?;
            
            if condition.required && !condition_met {
                result.add_failed_condition(condition.clone(), condition.error_message.clone());
            } else if condition_met {
                result.add_passed_condition(condition.clone());
            }
        }
        
        Ok(result)
    }
    
    async fn evaluate_execution_condition(
        &self,
        condition: &ExecutionCondition,
        actor: &Actor,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        match condition.condition_type {
            ExecutionConditionType::SelfHealth => {
                let current_hp = actor.get_health_percentage();
                self.evaluate_health_condition(&condition.condition, current_hp).await
            },
            ExecutionConditionType::SelfMana => {
                let current_mana = actor.get_mana_percentage();
                self.evaluate_mana_condition(&condition.condition, current_mana).await
            },
            ExecutionConditionType::SelfStamina => {
                let current_stamina = actor.get_stamina_percentage();
                self.evaluate_stamina_condition(&condition.condition, current_stamina).await
            },
            ExecutionConditionType::SelfStatus => {
                self.evaluate_status_condition(&condition.condition, actor, true).await
            },
            ExecutionConditionType::TargetHealth => {
                if let Some(target) = context.get_primary_target() {
                    let target_hp = target.get_health_percentage();
                    self.evaluate_health_condition(&condition.condition, target_hp).await
                } else {
                    Ok(false)
                }
            },
            ExecutionConditionType::TargetStatus => {
                if let Some(target) = context.get_primary_target() {
                    self.evaluate_status_condition(&condition.condition, target, false).await
                } else {
                    Ok(false)
                }
            },
            ExecutionConditionType::Environment => {
                self.evaluate_environment_condition(&condition.condition, context).await
            },
            ExecutionConditionType::ResourceAvailable => {
                self.evaluate_resource_condition(&condition.condition, actor).await
            },
            ExecutionConditionType::CooldownReady => {
                self.evaluate_cooldown_condition(&condition.condition, actor).await
            },
            ExecutionConditionType::LevelRequirement => {
                self.evaluate_level_condition(&condition.condition, actor).await
            },
            ExecutionConditionType::ClassRequirement => {
                self.evaluate_class_condition(&condition.condition, actor).await
            },
            ExecutionConditionType::RaceRequirement => {
                self.evaluate_race_condition(&condition.condition, actor).await
            },
            ExecutionConditionType::EquipmentRequirement => {
                self.evaluate_equipment_condition(&condition.condition, actor).await
            },
            ExecutionConditionType::LocationRequirement => {
                self.evaluate_location_condition(&condition.condition, actor).await
            },
            ExecutionConditionType::TimeRequirement => {
                self.evaluate_time_condition(&condition.condition, context).await
            },
            ExecutionConditionType::WeatherRequirement => {
                self.evaluate_weather_condition(&condition.condition, context).await
            },
            ExecutionConditionType::Custom(ref custom_type) => {
                self.evaluate_custom_condition(&condition.condition, custom_type, actor, context).await
            },
        }
    }
    
    async fn evaluate_health_condition(
        &self,
        condition: &str,
        current_hp: f64
    ) -> Result<bool, ActionError> {
        // Parse condition like "self.hp < 0.5"
        if let Some(threshold) = self.extract_threshold(condition) {
            Ok(current_hp < threshold)
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_mana_condition(
        &self,
        condition: &str,
        current_mana: f64
    ) -> Result<bool, ActionError> {
        if let Some(threshold) = self.extract_threshold(condition) {
            Ok(current_mana < threshold)
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_stamina_condition(
        &self,
        condition: &str,
        current_stamina: f64
    ) -> Result<bool, ActionError> {
        if let Some(threshold) = self.extract_threshold(condition) {
            Ok(current_stamina > threshold)
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_status_condition(
        &self,
        condition: &str,
        actor: &Actor,
        is_self: bool
    ) -> Result<bool, ActionError> {
        // Parse condition like "self.has_status('berserk')"
        if let Some(status_name) = self.extract_status_name(condition) {
            Ok(actor.has_status(&status_name))
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_environment_condition(
        &self,
        condition: &str,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        // Parse condition like "environment.is_night"
        match condition {
            "environment.is_night" => Ok(context.environment.is_night()),
            "environment.is_day" => Ok(context.environment.is_day()),
            "environment.is_indoors" => Ok(context.environment.is_indoors()),
            "environment.is_outdoors" => Ok(context.environment.is_outdoors()),
            _ => Err(ActionError::InvalidCondition(condition.to_string())),
        }
    }
    
    async fn evaluate_resource_condition(
        &self,
        condition: &str,
        actor: &Actor
    ) -> Result<bool, ActionError> {
        // Parse condition like "resource.available('mana') > 100"
        if let Some((resource_type, threshold)) = self.extract_resource_threshold(condition) {
            let available = actor.get_resource_amount(&resource_type);
            Ok(available > threshold)
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_cooldown_condition(
        &self,
        condition: &str,
        actor: &Actor
    ) -> Result<bool, ActionError> {
        // Parse condition like "cooldown.ready('fireball')"
        if let Some(action_id) = self.extract_action_id(condition) {
            Ok(actor.is_cooldown_ready(&action_id))
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_level_condition(
        &self,
        condition: &str,
        actor: &Actor
    ) -> Result<bool, ActionError> {
        // Parse condition like "self.level >= 10"
        if let Some(threshold) = self.extract_threshold(condition) {
            Ok(actor.level >= threshold as u32)
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_class_condition(
        &self,
        condition: &str,
        actor: &Actor
    ) -> Result<bool, ActionError> {
        // Parse condition like "self.class == 'warrior'"
        if let Some(expected_class) = self.extract_equality_value(condition) {
            Ok(actor.class == expected_class)
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_race_condition(
        &self,
        condition: &str,
        actor: &Actor
    ) -> Result<bool, ActionError> {
        // Parse condition like "self.race == 'human'"
        if let Some(expected_race) = self.extract_equality_value(condition) {
            Ok(actor.race == expected_race)
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_equipment_condition(
        &self,
        condition: &str,
        actor: &Actor
    ) -> Result<bool, ActionError> {
        // Parse condition like "self.equipped('sword')"
        if let Some(equipment_name) = self.extract_equipment_name(condition) {
            Ok(actor.has_equipment(&equipment_name))
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_location_condition(
        &self,
        condition: &str,
        actor: &Actor
    ) -> Result<bool, ActionError> {
        // Parse condition like "self.location == 'dungeon'"
        if let Some(expected_location) = self.extract_equality_value(condition) {
            Ok(actor.location == expected_location)
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_time_condition(
        &self,
        condition: &str,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        // Parse condition like "time.hour >= 18"
        if let Some(threshold) = self.extract_threshold(condition) {
            Ok(context.time.hour >= threshold as u32)
        } else {
            Err(ActionError::InvalidCondition(condition.to_string()))
        }
    }
    
    async fn evaluate_weather_condition(
        &self,
        condition: &str,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        // Parse condition like "weather.is_raining"
        match condition {
            "weather.is_raining" => Ok(context.weather.is_raining()),
            "weather.is_snowing" => Ok(context.weather.is_snowing()),
            "weather.is_clear" => Ok(context.weather.is_clear()),
            _ => Err(ActionError::InvalidCondition(condition.to_string())),
        }
    }
    
    async fn evaluate_custom_condition(
        &self,
        condition: &str,
        custom_type: &str,
        actor: &Actor,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        // Delegate to custom condition evaluator
        self.condition_evaluator.evaluate_custom_condition(
            condition, custom_type, actor, context
        ).await
    }
    
    // Helper methods for parsing conditions
    fn extract_threshold(&self, condition: &str) -> Option<f64> {
        // Extract threshold from conditions like "self.hp < 0.5"
        if let Some(captures) = regex::Regex::new(r"< (\d+\.?\d*)").unwrap().captures(condition) {
            captures.get(1)?.as_str().parse().ok()
        } else if let Some(captures) = regex::Regex::new(r"> (\d+\.?\d*)").unwrap().captures(condition) {
            captures.get(1)?.as_str().parse().ok()
        } else if let Some(captures) = regex::Regex::new(r">= (\d+\.?\d*)").unwrap().captures(condition) {
            captures.get(1)?.as_str().parse().ok()
        } else if let Some(captures) = regex::Regex::new(r"<= (\d+\.?\d*)").unwrap().captures(condition) {
            captures.get(1)?.as_str().parse().ok()
        } else {
            None
        }
    }
    
    fn extract_status_name(&self, condition: &str) -> Option<String> {
        // Extract status name from conditions like "self.has_status('berserk')"
        if let Some(captures) = regex::Regex::new(r"has_status\('([^']+)'\)").unwrap().captures(condition) {
            Some(captures.get(1)?.as_str().to_string())
        } else {
            None
        }
    }
    
    fn extract_resource_threshold(&self, condition: &str) -> Option<(String, f64)> {
        // Extract resource type and threshold from conditions like "resource.available('mana') > 100"
        if let Some(captures) = regex::Regex::new(r"resource\.available\('([^']+)'\) > (\d+\.?\d*)").unwrap().captures(condition) {
            let resource_type = captures.get(1)?.as_str().to_string();
            let threshold = captures.get(2)?.as_str().parse().ok()?;
            Some((resource_type, threshold))
        } else {
            None
        }
    }
    
    fn extract_action_id(&self, condition: &str) -> Option<String> {
        // Extract action ID from conditions like "cooldown.ready('fireball')"
        if let Some(captures) = regex::Regex::new(r"cooldown\.ready\('([^']+)'\)").unwrap().captures(condition) {
            Some(captures.get(1)?.as_str().to_string())
        } else {
            None
        }
    }
    
    fn extract_equality_value(&self, condition: &str) -> Option<String> {
        // Extract equality value from conditions like "self.class == 'warrior'"
        if let Some(captures) = regex::Regex::new(r"== '([^']+)'").unwrap().captures(condition) {
            Some(captures.get(1)?.as_str().to_string())
        } else {
            None
        }
    }
    
    fn extract_equipment_name(&self, condition: &str) -> Option<String> {
        // Extract equipment name from conditions like "self.equipped('sword')"
        if let Some(captures) = regex::Regex::new(r"equipped\('([^']+)'\)").unwrap().captures(condition) {
            Some(captures.get(1)?.as_str().to_string())
        } else {
            None
        }
    }
    
    fn validate_condition_syntax(&self, condition: &ExecutionCondition) -> Result<(), ActionError> {
        // Basic syntax validation
        if condition.condition.is_empty() {
            return Err(ActionError::InvalidCondition("Empty condition".to_string()));
        }
        
        // Validate based on condition type
        match condition.condition_type {
            ExecutionConditionType::SelfHealth | ExecutionConditionType::SelfMana | 
            ExecutionConditionType::SelfStamina | ExecutionConditionType::TargetHealth => {
                if !self.is_valid_comparison_condition(&condition.condition) {
                    return Err(ActionError::InvalidCondition(format!("Invalid comparison condition: {}", condition.condition)));
                }
            },
            ExecutionConditionType::SelfStatus | ExecutionConditionType::TargetStatus => {
                if !self.is_valid_status_condition(&condition.condition) {
                    return Err(ActionError::InvalidCondition(format!("Invalid status condition: {}", condition.condition)));
                }
            },
            _ => {
                // Custom validation for other types
            }
        }
        
        Ok(())
    }
    
    fn validate_condition_type(&self, condition: &ExecutionCondition) -> Result<(), ActionError> {
        // Validate that condition type matches condition string
        match condition.condition_type {
            ExecutionConditionType::SelfHealth => {
                if !condition.condition.contains("self.hp") {
                    return Err(ActionError::InvalidCondition("SelfHealth condition must contain 'self.hp'".to_string()));
                }
            },
            ExecutionConditionType::SelfMana => {
                if !condition.condition.contains("self.mana") {
                    return Err(ActionError::InvalidCondition("SelfMana condition must contain 'self.mana'".to_string()));
                }
            },
            ExecutionConditionType::SelfStamina => {
                if !condition.condition.contains("self.stamina") {
                    return Err(ActionError::InvalidCondition("SelfStamina condition must contain 'self.stamina'".to_string()));
                }
            },
            _ => {
                // Custom validation for other types
            }
        }
        
        Ok(())
    }
    
    fn is_valid_comparison_condition(&self, condition: &str) -> bool {
        // Check if condition is a valid comparison (e.g., "self.hp < 0.5")
        regex::Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_.]*\s*[<>=!]+\s*\d+\.?\d*$").unwrap().is_match(condition)
    }
    
    fn is_valid_status_condition(&self, condition: &str) -> bool {
        // Check if condition is a valid status condition (e.g., "self.has_status('berserk')")
        regex::Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_.]*\.has_status\('[^']+'\)$").unwrap().is_match(condition)
    }
}

/// Execution Condition Result
pub struct ExecutionConditionResult {
    pub passed_conditions: Vec<ExecutionCondition>,
    pub failed_conditions: Vec<(ExecutionCondition, String)>,
    pub can_execute: bool,
}

impl ExecutionConditionResult {
    pub fn new() -> Self {
        Self {
            passed_conditions: Vec::new(),
            failed_conditions: Vec::new(),
            can_execute: true,
        }
    }
    
    pub fn add_passed_condition(&mut self, condition: ExecutionCondition) {
        self.passed_conditions.push(condition);
    }
    
    pub fn add_failed_condition(&mut self, condition: ExecutionCondition, error_message: String) {
        self.failed_conditions.push((condition, error_message));
        self.can_execute = false;
    }
    
    pub fn get_error_messages(&self) -> Vec<String> {
        self.failed_conditions.iter().map(|(_, msg)| msg.clone()).collect()
    }
}
    
    pub fn create_attack_action(
        &mut self,
        attack_data: AttackActionData
    ) -> Result<Box<dyn Action>, ActionError> {
        let action = AttackAction::new(attack_data)?;
        self.action_registry.register(action.id(), Box::new(action));
        Ok(Box::new(action))
    }
    
    pub fn create_skill_action(
        &mut self,
        skill_data: SkillActionData
    ) -> Result<Box<dyn Action>, ActionError> {
        let action = SkillAction::new(skill_data)?;
        self.action_registry.register(action.id(), Box::new(action));
        Ok(Box::new(action))
    }
}

/// Action Data structure
pub struct ActionData {
    pub metadata: ActionMetadata,
    pub category: ActionCategory,
    pub action_type: ActionType,
    pub resource_requirements: Vec<ResourceRequirement>,
    pub execution_duration: DurationRange,
    pub cooldown_duration: CooldownConfig,
    pub interrupt_conditions: Vec<InterruptCondition>,
    pub execution_conditions: Vec<ExecutionCondition>,
    pub target_requirements: TargetRequirements,
    pub effects: Vec<ActionEffect>,
}

/// Attack Action Data
pub struct AttackActionData {
    pub action_data: ActionData,
    pub projectile_config: Option<ProjectileConfig>,
    pub aoe_config: Option<AOEConfig>,
    pub combat_effectiveness_scaling: Vec<String>,
}

/// Skill Action Data
pub struct SkillActionData {
    pub action_data: ActionData,
    pub skill_type: SkillType,
    pub skill_level: u32,
    pub skill_requirements: Vec<SkillRequirement>,
    pub skill_effectiveness_scaling: Vec<String>,
}

pub enum SkillType {
    Active,
    Passive,
    Toggle,
    Channeled,
    Instant,
}

pub struct SkillRequirement {
    pub skill_id: String,
    pub min_level: u32,
    pub required: bool,
}
```

## 📊 **Action Registry**

```rust
pub struct ActionRegistry {
    actions: HashMap<String, Box<dyn Action>>,
    action_categories: HashMap<ActionCategory, Vec<String>>,
    action_types: HashMap<ActionType, Vec<String>>,
    action_tags: HashMap<String, Vec<String>>,
}

impl ActionRegistry {
    pub fn register(&mut self, action_id: String, action: Box<dyn Action>) {
        let category = action.get_category();
        let action_type = action.get_type();
        
        // Register action
        self.actions.insert(action_id.clone(), action);
        
        // Register by category
        self.action_categories.entry(category)
            .or_insert_with(Vec::new)
            .push(action_id.clone());
        
        // Register by type
        self.action_types.entry(action_type)
            .or_insert_with(Vec::new)
            .push(action_id.clone());
    }
    
    pub fn get_action(&self, action_id: &str) -> Option<&dyn Action> {
        self.actions.get(action_id).map(|a| a.as_ref())
    }
    
    pub fn get_actions_by_category(&self, category: ActionCategory) -> Vec<&dyn Action> {
        self.action_categories.get(&category)
            .map(|ids| ids.iter().filter_map(|id| self.actions.get(id)).map(|a| a.as_ref()).collect())
            .unwrap_or_default()
    }
    
    pub fn get_actions_by_type(&self, action_type: ActionType) -> Vec<&dyn Action> {
        self.action_types.get(&action_type)
            .map(|ids| ids.iter().filter_map(|id| self.actions.get(id)).map(|a| a.as_ref()).collect())
            .unwrap_or_default()
    }
}
```

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_action_creation() {
        let mut builder = ActionBuilder::new();
        let action_data = create_test_action_data();
        
        let action = builder.create_action::<TestAction>(action_data).unwrap();
        assert_eq!(action.get_metadata().id, "test_action");
    }
    
    #[test]
    fn test_resource_validation() {
        let validator = ResourceValidator::new();
        let requirement = create_test_resource_requirement();
        let actor = create_test_actor();
        
        let result = validator.validate_requirement(&requirement, &actor);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_timing_validation() {
        let validator = TimingValidator::new();
        let duration = create_test_duration_range();
        let cooldown = create_test_cooldown_config();
        
        let result = validator.validate_timing(&duration, &cooldown);
        assert!(result.is_ok());
    }
}
```

## 🔗 **Integration Points**

### **Resource Manager Integration**
- **Resource Validation**: Validate resource availability
- **Resource Consumption**: Consume resources during execution
- **Resource Regeneration**: Handle resource regeneration
- **Resource Events**: Trigger resource change events

### **Element Core Integration**
- **Derived Stats**: Access derived stats for scaling
- **Stat Validation**: Validate stat requirements
- **Stat Scaling**: Apply stat scaling to action properties

### **Combat Core Integration**
- **Damage Calculation**: Provide damage input
- **Status Effects**: Handle status effect application
- **Targeting**: Provide target information

## 🆕 **Multi-Element Support Components**

### **1. Multi-Element Action Validator**

```rust
/// Multi-element action validator
pub struct MultiElementActionValidator {
    synergy_calculator: MultiElementSynergyCalculator,
    domain_validator: DomainActivationValidator,
    life_force_validator: LifeForceValidator,
    element_core_client: ElementCoreClient,
}

impl MultiElementActionValidator {
    pub async fn validate_multi_element_action(
        &self,
        action: &dyn Action,
        actor: &Actor,
        context: &ActionContext
    ) -> Result<ValidationResult, ActionError> {
        let mut result = ValidationResult::new();
        
        // Validate multi-element requirements
        if let Some(multi_element_req) = action.get_multi_element_requirement() {
            self.validate_multi_element_requirements(multi_element_req, actor, &mut result).await?;
        }
        
        // Validate domain activation
        if let Some(domain_req) = action.get_domain_activation_requirement() {
            self.validate_domain_activation(domain_req, actor, &mut result).await?;
        }
        
        // Validate life force consumption
        if let Some(life_force_req) = action.get_life_force_consumption() {
            self.validate_life_force_consumption(life_force_req, actor, &mut result).await?;
        }
        
        Ok(result)
    }
    
    async fn validate_multi_element_requirements(
        &self,
        multi_element_req: &MultiElementRequirement,
        actor: &Actor,
        result: &mut ValidationResult
    ) -> Result<(), ActionError> {
        let mut valid_elements = 0;
        let mut total_mastery = 0.0;
        
        for element_req in &multi_element_req.required_elements {
            let mastery = actor.get_element_mastery(&element_req.element_name)?;
            
            if mastery >= element_req.min_mastery && mastery <= element_req.max_mastery {
                valid_elements += 1;
                total_mastery += mastery * element_req.weight;
            } else if element_req.required {
                result.add_error(ActionError::ElementMasteryInsufficient(
                    element_req.element_name.clone(),
                    element_req.min_mastery,
                    mastery
                ));
            }
        }
        
        if valid_elements < multi_element_req.min_elements {
            result.add_error(ActionError::InsufficientElements(
                multi_element_req.min_elements,
                valid_elements
            ));
        }
        
        if valid_elements > multi_element_req.max_elements {
            result.add_error(ActionError::TooManyElements(
                multi_element_req.max_elements,
                valid_elements
            ));
        }
        
        Ok(())
    }
}
```

### **2. Multi-Element Synergy Calculator**

```rust
/// Multi-element synergy calculator
pub struct MultiElementSynergyCalculator {
    element_core_client: ElementCoreClient,
    synergy_configs: HashMap<String, ElementSynergyConfig>,
    element_interactions: HashMap<String, ElementInteraction>,
}

impl MultiElementSynergyCalculator {
    pub async fn calculate_multi_element_synergy(
        &self,
        elements: &[ElementRequirement],
        actor: &Actor
    ) -> Result<MultiElementSynergyResult, ActionError> {
        let mut synergy_result = MultiElementSynergyResult::new();
        
        // Calculate individual element masteries
        for element_req in elements {
            let mastery = actor.get_element_mastery(&element_req.element_name)?;
            synergy_result.add_element_mastery(element_req.element_name.clone(), mastery);
        }
        
        // Calculate synergy based on type
        match self.get_synergy_type(elements) {
            ElementSynergyType::Additive => {
                self.calculate_additive_synergy(elements, &mut synergy_result).await?;
            },
            ElementSynergyType::Multiplicative => {
                self.calculate_multiplicative_synergy(elements, &mut synergy_result).await?;
            },
            ElementSynergyType::Harmonic => {
                self.calculate_harmonic_synergy(elements, &mut synergy_result).await?;
            },
            ElementSynergyType::Geometric => {
                self.calculate_geometric_synergy(elements, &mut synergy_result).await?;
            },
            ElementSynergyType::Custom(ref formula) => {
                self.calculate_custom_synergy(formula, elements, &mut synergy_result).await?;
            },
        }
        
        // Apply element interactions
        self.apply_element_interactions(elements, &mut synergy_result).await?;
        
        Ok(synergy_result)
    }
    
    async fn calculate_additive_synergy(
        &self,
        elements: &[ElementRequirement],
        result: &mut MultiElementSynergyResult
    ) -> Result<(), ActionError> {
        let mut total_synergy = 0.0;
        let mut total_weight = 0.0;
        
        for element_req in elements {
            let mastery = result.get_element_mastery(&element_req.element_name);
            let weighted_mastery = mastery * element_req.weight;
            total_synergy += weighted_mastery;
            total_weight += element_req.weight;
        }
        
        let average_synergy = if total_weight > 0.0 {
            total_synergy / total_weight
        } else {
            0.0
        };
        
        result.set_synergy_bonus(average_synergy);
        Ok(())
    }
    
    async fn calculate_multiplicative_synergy(
        &self,
        elements: &[ElementRequirement],
        result: &mut MultiElementSynergyResult
    ) -> Result<(), ActionError> {
        let mut total_synergy = 1.0;
        
        for element_req in elements {
            let mastery = result.get_element_mastery(&element_req.element_name);
            let normalized_mastery = (mastery / 100.0).max(0.1); // Normalize to 0.1-1.0
            total_synergy *= normalized_mastery;
        }
        
        result.set_synergy_bonus(total_synergy);
        Ok(())
    }
    
    async fn calculate_harmonic_synergy(
        &self,
        elements: &[ElementRequirement],
        result: &mut MultiElementSynergyResult
    ) -> Result<(), ActionError> {
        let mut harmonic_sum = 0.0;
        let mut count = 0;
        
        for element_req in elements {
            let mastery = result.get_element_mastery(&element_req.element_name);
            if mastery > 0.0 {
                harmonic_sum += 1.0 / mastery;
                count += 1;
            }
        }
        
        let harmonic_mean = if count > 0 {
            count as f64 / harmonic_sum
        } else {
            0.0
        };
        
        result.set_synergy_bonus(harmonic_mean);
        Ok(())
    }
    
    async fn calculate_geometric_synergy(
        &self,
        elements: &[ElementRequirement],
        result: &mut MultiElementSynergyResult
    ) -> Result<(), ActionError> {
        let mut geometric_product = 1.0;
        let mut count = 0;
        
        for element_req in elements {
            let mastery = result.get_element_mastery(&element_req.element_name);
            if mastery > 0.0 {
                geometric_product *= mastery;
                count += 1;
            }
        }
        
        let geometric_mean = if count > 0 {
            geometric_product.powf(1.0 / count as f64)
        } else {
            0.0
        };
        
        result.set_synergy_bonus(geometric_mean);
        Ok(())
    }
}

/// Multi-element synergy result
pub struct MultiElementSynergyResult {
    pub element_masteries: HashMap<String, f64>,
    pub synergy_bonus: f64,
    pub synergy_penalty: f64,
    pub total_synergy: f64,
    pub element_interactions: Vec<ElementInteractionResult>,
}

impl MultiElementSynergyResult {
    pub fn new() -> Self {
        Self {
            element_masteries: HashMap::new(),
            synergy_bonus: 0.0,
            synergy_penalty: 0.0,
            total_synergy: 0.0,
            element_interactions: Vec::new(),
        }
    }
    
    pub fn add_element_mastery(&mut self, element_name: String, mastery: f64) {
        self.element_masteries.insert(element_name, mastery);
    }
    
    pub fn get_element_mastery(&self, element_name: &str) -> f64 {
        self.element_masteries.get(element_name).copied().unwrap_or(0.0)
    }
    
    pub fn set_synergy_bonus(&mut self, bonus: f64) {
        self.synergy_bonus = bonus;
        self.total_synergy = bonus - self.synergy_penalty;
    }
    
    pub fn add_synergy_penalty(&mut self, penalty: f64) {
        self.synergy_penalty += penalty;
        self.total_synergy = self.synergy_bonus - self.synergy_penalty;
    }
}
```

### **3. Domain Activation Validator**

```rust
/// Domain activation validator
pub struct DomainActivationValidator {
    domain_registry: DomainRegistry,
    element_core_client: ElementCoreClient,
}

impl DomainActivationValidator {
    pub async fn validate_domain_activation(
        &self,
        requirement: &DomainActivationRequirement,
        actor: &Actor
    ) -> Result<bool, ActionError> {
        // Check if actor has active domain
        let active_domain = actor.get_active_domain(&requirement.domain_element)?;
        
        if active_domain.is_none() {
            return Err(ActionError::DomainNotActive(requirement.domain_element.clone()));
        }
        
        let domain = active_domain.unwrap();
        
        // Check domain level
        if domain.level < requirement.domain_level {
            return Err(ActionError::DomainLevelInsufficient(
                requirement.domain_level,
                domain.level
            ));
        }
        
        // Check domain duration
        if domain.remaining_duration < requirement.domain_duration {
            return Err(ActionError::DomainDurationInsufficient(
                requirement.domain_duration,
                domain.remaining_duration
            ));
        }
        
        Ok(true)
    }
}

/// Domain activation requirement
pub struct DomainActivationRequirement {
    pub domain_element: String,
    pub domain_level: u32,
    pub domain_duration: Duration,
    pub domain_effects: Vec<DomainEffect>,
    pub domain_radius: f64,
    pub domain_power: f64,
}

/// Domain effect
pub struct DomainEffect {
    pub effect_type: DomainEffectType,
    pub magnitude: f64,
    pub duration: Duration,
    pub target_type: DomainTargetType,
}

pub enum DomainEffectType {
    Buff,
    Debuff,
    Damage,
    Heal,
    StatusEffect,
    ResourceModification,
}

pub enum DomainTargetType {
    Self,
    Allies,
    Enemies,
    All,
    Custom(String),
}
```

### **4. Life Force Management System**

```rust
/// Life force validator
pub struct LifeForceValidator {
    life_force_registry: LifeForceRegistry,
    recovery_calculator: LifeForceRecoveryCalculator,
}

impl LifeForceValidator {
    pub async fn validate_life_force_consumption(
        &self,
        consumption: &LifeForceConsumption,
        actor: &Actor
    ) -> Result<bool, ActionError> {
        // Check life force availability
        let current_life_force = actor.get_life_force()?;
        if current_life_force < consumption.life_force_amount {
            return Err(ActionError::InsufficientLifeForce(
                consumption.life_force_amount,
                current_life_force
            ));
        }
        
        // Check vitality availability
        let current_vitality = actor.get_vitality()?;
        if current_vitality < consumption.vitality_amount {
            return Err(ActionError::InsufficientVitality(
                consumption.vitality_amount,
                current_vitality
            ));
        }
        
        // Check energy availability
        let current_energy = actor.get_energy()?;
        if current_energy < consumption.energy_amount {
            return Err(ActionError::InsufficientEnergy(
                consumption.energy_amount,
                current_energy
            ));
        }
        
        Ok(true)
    }
}

/// Life force consumer
pub struct LifeForceConsumer {
    life_force_registry: LifeForceRegistry,
    consumption_tracker: ConsumptionTracker,
}

impl LifeForceConsumer {
    pub async fn consume_life_force(
        &self,
        consumption: &LifeForceConsumption,
        actor: &mut Actor
    ) -> Result<(), ActionError> {
        // Consume life force
        actor.consume_life_force(consumption.life_force_amount)?;
        
        // Consume vitality
        actor.consume_vitality(consumption.vitality_amount)?;
        
        // Consume energy
        actor.consume_energy(consumption.energy_amount)?;
        
        // Track consumption
        self.consumption_tracker.track_consumption(actor.id(), consumption)?;
        
        Ok(())
    }
}

/// Life force regenerator
pub struct LifeForceRegenerator {
    recovery_calculator: LifeForceRecoveryCalculator,
    recovery_conditions: Vec<RecoveryCondition>,
}

impl LifeForceRegenerator {
    pub async fn regenerate_life_force(
        &self,
        actor: &mut Actor,
        delta_time: Duration
    ) -> Result<(), ActionError> {
        let recovery_rate = self.calculate_recovery_rate(actor).await?;
        
        // Regenerate life force
        let life_force_recovery = recovery_rate * delta_time.as_secs_f64();
        actor.regenerate_life_force(life_force_recovery)?;
        
        // Regenerate vitality
        let vitality_recovery = recovery_rate * 0.8 * delta_time.as_secs_f64();
        actor.regenerate_vitality(vitality_recovery)?;
        
        // Regenerate energy
        let energy_recovery = recovery_rate * 1.2 * delta_time.as_secs_f64();
        actor.regenerate_energy(energy_recovery)?;
        
        Ok(())
    }
    
    async fn calculate_recovery_rate(&self, actor: &Actor) -> Result<f64, ActionError> {
        let base_rate = 0.01; // 1% per second
        let mut total_rate = base_rate;
        
        for condition in &self.recovery_conditions {
            if self.evaluate_recovery_condition(condition, actor).await? {
                total_rate += condition.recovery_rate;
            }
        }
        
        Ok(total_rate)
    }
}

/// Life force consumption
pub struct LifeForceConsumption {
    pub life_force_amount: f64,
    pub vitality_amount: f64,
    pub energy_amount: f64,
    pub consumption_formula: String,
    pub recovery_rate: f64,
    pub recovery_conditions: Vec<RecoveryCondition>,
}

/// Recovery condition for life force
pub struct RecoveryCondition {
    pub condition: String,
    pub recovery_rate: f64,
    pub condition_type: RecoveryConditionType,
}

pub enum RecoveryConditionType {
    Time,
    Meditation,
    Rest,
    Consumable,
    Custom(String),
}
```

### **5. World Area Spawning System**

```rust
/// World area spawner
pub struct WorldAreaSpawner {
    world_core_integration: WorldCoreIntegration,
    area_registry: AreaRegistry,
    area_manager: AreaManager,
}

impl WorldAreaSpawner {
    pub async fn spawn_world_area(
        &self,
        effect: &WorldAreaSpawningEffect,
        position: Position,
        caster: &Actor
    ) -> Result<WorldAreaId, ActionError> {
        let world_area = WorldArea {
            id: effect.area_id.clone(),
            area_type: effect.area_type.clone(),
            position,
            size: effect.area_size,
            duration: effect.area_duration,
            effects: effect.area_effects.clone(),
            caster_id: caster.id.clone(),
            spawn_time: Instant::now(),
        };
        
        let area_id = self.world_core_integration.spawn_area(world_area).await?;
        Ok(area_id)
    }
}

/// World area spawning effect
pub struct WorldAreaSpawningEffect {
    pub area_id: String,
    pub area_type: WorldAreaType,
    pub area_size: f64,
    pub area_duration: Duration,
    pub area_effects: Vec<WorldAreaEffect>,
    pub world_core_integration: WorldCoreIntegration,
    pub spawn_conditions: Vec<SpawnCondition>,
}

pub enum WorldAreaType {
    DestructionZone,      // Khu vực tàn phá
    ElementalField,       // Trường nguyên tố
    ForbiddenZone,        // Vùng cấm
    SacredGround,         // Đất thánh
    Custom(String),
}

pub struct WorldAreaEffect {
    pub effect_type: WorldAreaEffectType,
    pub magnitude: f64,
    pub duration: Duration,
    pub target_type: WorldAreaTargetType,
    pub area_modifier: f64,
}

pub enum WorldAreaEffectType {
    ContinuousDamage,
    StatusEffect,
    ResourceDrain,
    StatModification,
    MovementRestriction,
    Custom(String),
}

pub enum WorldAreaTargetType {
    All,
    Enemies,
    Allies,
    Self,
    Custom(String),
}

/// World Core integration
pub struct WorldCoreIntegration {
    pub world_core_client: WorldCoreClient,
    pub area_registry: AreaRegistry,
    pub area_manager: AreaManager,
}

impl WorldCoreIntegration {
    pub async fn spawn_area(&self, area: WorldArea) -> Result<WorldAreaId, ActionError> {
        let area_id = self.world_core_client.spawn_area(area).await?;
        Ok(area_id)
    }
}
```

### **6. Forbidden Spell Support**

```rust
/// Forbidden spell validator
pub struct ForbiddenSpellValidator {
    forbidden_spell_registry: ForbiddenSpellRegistry,
    spell_level_validator: SpellLevelValidator,
    multi_element_validator: MultiElementActionValidator,
}

impl ForbiddenSpellValidator {
    pub async fn validate_forbidden_spell(
        &self,
        spell: &ForbiddenSpellMetadata,
        actor: &Actor
    ) -> Result<bool, ActionError> {
        // Validate spell level
        let actor_spell_level = actor.get_forbidden_spell_level()?;
        if actor_spell_level < spell.spell_level as u32 {
            return Err(ActionError::ForbiddenSpellLevelInsufficient(
                spell.spell_level as u32,
                actor_spell_level
            ));
        }
        
        // Validate multi-element requirements
        self.multi_element_validator.validate_multi_element_requirements(
            &spell.multi_element_requirement,
            actor
        ).await?;
        
        // Validate domain activation
        if let Some(domain_req) = &spell.domain_activation_requirement {
            self.validate_domain_activation(domain_req, actor).await?;
        }
        
        // Validate life force consumption
        if let Some(life_force_req) = &spell.life_force_consumption {
            self.validate_life_force_consumption(life_force_req, actor).await?;
        }
        
        Ok(true)
    }
}

/// Forbidden spell effect manager
pub struct ForbiddenSpellEffectManager {
    effect_registry: ForbiddenSpellEffectRegistry,
    effect_calculator: ForbiddenSpellEffectCalculator,
}

impl ForbiddenSpellEffectManager {
    pub async fn apply_forbidden_spell_effects(
        &self,
        effects: &[ForbiddenSpellEffect],
        context: &ActionContext
    ) -> Result<Vec<ForbiddenSpellEffectResult>, ActionError> {
        let mut results = Vec::new();
        
        for effect in effects {
            let result = self.apply_forbidden_spell_effect(effect, context).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    async fn apply_forbidden_spell_effect(
        &self,
        effect: &ForbiddenSpellEffect,
        context: &ActionContext
    ) -> Result<ForbiddenSpellEffectResult, ActionError> {
        match effect.effect_type {
            ForbiddenSpellEffectType::MassiveDamage => {
                self.apply_massive_damage_effect(effect, context).await
            },
            ForbiddenSpellEffectType::ElementalStorm => {
                self.apply_elemental_storm_effect(effect, context).await
            },
            ForbiddenSpellEffectType::RealityDistortion => {
                self.apply_reality_distortion_effect(effect, context).await
            },
            ForbiddenSpellEffectType::TimeManipulation => {
                self.apply_time_manipulation_effect(effect, context).await
            },
            ForbiddenSpellEffectType::SpaceManipulation => {
                self.apply_space_manipulation_effect(effect, context).await
            },
            ForbiddenSpellEffectType::Custom(ref custom_type) => {
                self.apply_custom_effect(effect, custom_type, context).await
            },
        }
    }
}

/// Forbidden spell metadata
pub struct ForbiddenSpellMetadata {
    pub spell_level: ForbiddenSpellLevel,
    pub spell_type: ForbiddenSpellType,
    pub multi_element_requirement: MultiElementRequirement,
    pub domain_activation_requirement: Option<DomainActivationRequirement>,
    pub life_force_consumption: Option<LifeForceConsumption>,
    pub world_area_spawning: Option<WorldAreaSpawningEffect>,
    pub forbidden_spell_effects: Vec<ForbiddenSpellEffect>,
}

pub enum ForbiddenSpellLevel {
    Level1,  // Cấm chú cấp 1
    Level2,  // Cấm chú cấp 2
    Level3,  // Cấm chú cấp 3
    Level4,  // Cấm chú cấp 4
    Level5,  // Cấm chú cấp 5
}

pub enum ForbiddenSpellType {
    Destruction,     // Hủy diệt
    Control,         // Khống chế
    Summoning,       // Triệu hồi
    Transformation,  // Biến hóa
    Custom(String),
}

/// Forbidden spell effect
pub struct ForbiddenSpellEffect {
    pub effect_type: ForbiddenSpellEffectType,
    pub magnitude: f64,
    pub duration: Duration,
    pub target_type: ForbiddenSpellTargetType,
    pub element_combination: Vec<String>,
    pub synergy_multiplier: f64,
}

pub enum ForbiddenSpellEffectType {
    MassiveDamage,      // Sát thương diện rộng
    ElementalStorm,     // Bão nguyên tố
    RealityDistortion,  // Bóp méo thực tại
    TimeManipulation,   // Thao tác thời gian
    SpaceManipulation,  // Thao tác không gian
    Custom(String),
}

pub enum ForbiddenSpellTargetType {
    AllEnemies,         // Tất cả kẻ thù
    AllAllies,          // Tất cả đồng minh
    All,                // Tất cả
    Area,               // Khu vực
    World,              // Toàn thế giới
    Custom(String),
}
```

### **7. Enhanced Action Interface**

```rust
/// Enhanced action interface with multi-element support
pub trait Action {
    /// Action metadata
    fn get_metadata(&self) -> &ActionMetadata;
    
    /// Action category
    fn get_category(&self) -> ActionCategory;
    
    /// Action type
    fn get_type(&self) -> ActionType;
    
    /// Resource requirements
    fn get_resource_requirements(&self) -> &[ResourceRequirement];
    
    /// Execution duration
    fn get_execution_duration(&self) -> DurationRange;
    
    /// Cooldown duration
    fn get_cooldown_duration(&self) -> CooldownConfig;
    
    /// Interrupt conditions
    fn get_interrupt_conditions(&self) -> &[InterruptCondition];
    
    /// Execution conditions
    fn get_execution_conditions(&self) -> &[ExecutionCondition];
    
    /// Target requirements
    fn get_target_requirements(&self) -> &TargetRequirements;
    
    /// Effects
    fn get_effects(&self) -> &[ActionEffect];
    
    /// Multi-element requirements (NEW)
    fn get_multi_element_requirement(&self) -> Option<&MultiElementRequirement>;
    
    /// Domain activation requirement (NEW)
    fn get_domain_activation_requirement(&self) -> Option<&DomainActivationRequirement>;
    
    /// Life force consumption (NEW)
    fn get_life_force_consumption(&self) -> Option<&LifeForceConsumption>;
    
    /// World area spawning (NEW)
    fn get_world_area_spawning(&self) -> Option<&WorldAreaSpawningEffect>;
    
    /// Forbidden spell metadata (NEW)
    fn get_forbidden_spell_metadata(&self) -> Option<&ForbiddenSpellMetadata>;
    
    /// Validation
    fn validate(&self, context: &ActionContext) -> ValidationResult;
    
    /// Execution
    fn execute(&self, context: &mut ActionContext) -> ActionResult;
}

/// Enhanced action category with forbidden spells
pub enum ActionCategory {
    Combat,
    Lifestyle,
    Social,
    Crafting,
    Movement,
    Special,
    ForbiddenSpell,  // Cấm chú ma pháp
}

/// Enhanced action type with forbidden spells
pub enum ActionType {
    Attack,
    Skill,
    Movement,
    Interaction,
    Special,
    ForbiddenSpell,  // Cấm chú ma pháp
}
```

## 🔧 **Implementation Strategy**

### **Phase 1: Core Multi-Element Support (2-3 tuần)**
1. **MultiElementRequirement** struct và validation
2. **MultiElementSynergyCalculator** với các loại synergy
3. **DomainActivationRequirement** và validator
4. **LifeForceConsumption** và management system

### **Phase 2: World Integration (2-3 tuần)**
1. **WorldAreaSpawningEffect** và spawner
2. **WorldCoreIntegration** interface
3. **ForbiddenSpellMetadata** và validator
4. **ForbiddenSpellEffectManager**

### **Phase 3: Performance & Testing (1-2 tuần)**
1. **Caching system** cho synergy calculations
2. **Batch processing** cho multi-element actions
3. **Comprehensive testing** cho tất cả components
4. **Documentation** và examples

### **Phase 4: Integration & Deployment (1 tuần)**
1. **Element Core integration**
2. **World Core integration**
3. **Combat Core integration**
4. **Production deployment**

## 📊 **Performance Considerations**

### **Caching Strategy**
- **Synergy Calculations**: Cache kết quả synergy cho các element combinations
- **Domain Validation**: Cache domain status và validation results
- **Life Force Calculations**: Cache recovery rates và consumption calculations

### **Batch Processing**
- **Multi-Element Validation**: Validate multiple elements cùng lúc
- **Effect Application**: Apply multiple effects trong một batch
- **World Area Spawning**: Spawn multiple areas cùng lúc

### **Memory Management**
- **Object Pooling**: Reuse action instances và context objects
- **Lazy Loading**: Load components chỉ khi cần thiết
- **Garbage Collection**: Optimize memory usage cho large-scale operations

## 🧪 **Testing Strategy**

### **Unit Tests**
- **Multi-Element Validation**: Test các loại synergy calculations
- **Domain Activation**: Test domain requirements và validation
- **Life Force Management**: Test consumption và recovery
- **World Area Spawning**: Test area creation và management

### **Integration Tests**
- **Element Core Integration**: Test integration với Element Core
- **World Core Integration**: Test integration với World Core
- **Combat Core Integration**: Test integration với Combat Core

### **Performance Tests**
- **Load Testing**: Test với hàng nghìn actions cùng lúc
- **Memory Testing**: Test memory usage và garbage collection
- **Latency Testing**: Test response times cho critical paths

---

**Last Updated**: 2025-01-27  
**Version**: 2.0  
**Status**: Enhanced Design Phase  
**Maintainer**: Chaos World Team
