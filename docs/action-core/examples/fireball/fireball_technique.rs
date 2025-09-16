// Hỏa Cầu Thuật - Fireball Technique Implementation
// Kim Đan hệ thống - Golden Pill System
// Combat Action Implementation

use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};

/// Hỏa Cầu Thuật - Fireball Technique Action
/// Kim Đan hệ thống - Golden Pill System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireballTechnique {
    metadata: ActionMetadata,
    properties: ActionProperties,
    resource_requirements: Vec<ResourceRequirement>,
    execution_conditions: Vec<ExecutionCondition>,
    target_requirements: TargetRequirements,
    effects: Vec<ActionEffect>,
    damage_calculation: DamageCalculation,
    status_effects: Vec<StatusEffectConfig>,
    element_interactions: Vec<ElementInteraction>,
    cultivation_integration: CultivationIntegration,
}

impl Action for FireballTechnique {
    fn get_metadata(&self) -> &ActionMetadata {
        &self.metadata
    }
    
    fn get_category(&self) -> ActionCategory {
        ActionCategory::Combat
    }
    
    fn get_type(&self) -> ActionType {
        ActionType::Attack
    }
    
    fn get_resource_requirements(&self) -> &[ResourceRequirement] {
        &self.resource_requirements
    }
    
    fn get_execution_duration(&self) -> DurationRange {
        DurationRange {
            min: Duration::from_millis(1500),
            max: Duration::from_millis(2500),
            base: Duration::from_millis(2000),
        }
    }
    
    fn get_cooldown_duration(&self) -> CooldownConfig {
        CooldownConfig {
            min: Duration::from_millis(3000),
            max: Duration::from_millis(5000),
            base: Duration::from_millis(4000),
            interrupt_conditions: vec![
                InterruptCondition {
                    condition: "self.hp < 0.1".to_string(),
                    interrupt_type: InterruptType::HealthCritical,
                    message: "Sức khỏe quá yếu, không thể thi triển kỹ thuật".to_string(),
                },
                InterruptCondition {
                    condition: "self.qi < 20".to_string(),
                    interrupt_type: InterruptType::InsufficientQi,
                    message: "Linh khí không đủ, không thể thi triển kỹ thuật".to_string(),
                },
            ],
        }
    }
    
    fn get_interrupt_conditions(&self) -> &[InterruptCondition] {
        &self.cooldown_duration.interrupt_conditions
    }
    
    fn get_execution_conditions(&self) -> &[ExecutionCondition] {
        &self.execution_conditions
    }
    
    fn get_target_requirements(&self) -> &TargetRequirements {
        &self.target_requirements
    }
    
    fn get_effects(&self) -> &[ActionEffect] {
        &self.effects
    }
    
    fn validate(&self, context: &ActionContext) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Validate execution conditions
        for condition in &self.execution_conditions {
            if !self.evaluate_condition(condition, context) {
                result.add_error(ActionError::ExecutionConditionNotMet(
                    condition.error_message.clone()
                ));
            }
        }
        
        // Validate resource requirements
        for requirement in &self.resource_requirements {
            if !self.validate_resource_requirement(requirement, context) {
                result.add_error(ActionError::InsufficientResource(
                    requirement.resource_type.clone()
                ));
            }
        }
        
        // Validate target requirements
        if let Some(target) = &context.target {
            if !self.validate_target_requirements(target, context) {
                result.add_error(ActionError::InvalidTarget);
            }
        }
        
        result
    }
    
    fn execute(&self, context: &mut ActionContext) -> ActionResult {
        let mut result = ActionResult::new();
        
        // Consume resources
        self.consume_resources(context)?;
        
        // Calculate damage
        let damage = self.calculate_damage(context)?;
        
        // Apply effects
        self.apply_effects(context, &damage)?;
        
        // Apply status effects
        self.apply_status_effects(context)?;
        
        // Apply element interactions
        self.apply_element_interactions(context, &damage)?;
        
        result.success = true;
        result.damage = Some(damage);
        result.status_effects = self.get_applied_status_effects(context);
        
        Ok(result)
    }
}

impl FireballTechnique {
    /// Create new Fireball Technique action
    pub fn new() -> Self {
        Self {
            metadata: ActionMetadata {
                id: "fireball_technique".to_string(),
                name: "Hỏa Cầu Thuật".to_string(),
                name_en: "Fireball Technique".to_string(),
                description: "Tạo ra một quả cầu lửa mạnh mẽ, tấn công kẻ thù với sát thương hỏa nguyên tố".to_string(),
                description_en: "Creates a powerful fireball that attacks enemies with fire elemental damage".to_string(),
                icon_id: "fireball_technique_icon".to_string(),
                animation_id: "fireball_technique_cast".to_string(),
                sound_effect_id: "fireball_technique_sound".to_string(),
                visual_effects: vec!["fire_particles".to_string(), "explosion".to_string(), "heat_wave".to_string()],
                level_requirement: 5,
                class_requirements: vec!["golden_pill_cultivator".to_string(), "fire_mage".to_string()],
                race_requirements: vec![],
                cultivation_requirements: CultivationRequirements {
                    golden_pill_realm: 1,
                    fire_affinity: 30.0,
                },
            },
            properties: ActionProperties {
                target_count: 1,
                area_size: 0.0,
                projectile_speed: 15.0,
                projectile_range: 20.0,
                min_execution_duration: Duration::from_millis(1500),
                max_execution_duration: Duration::from_millis(2500),
                base_execution_duration: Duration::from_millis(2000),
                min_cooldown_duration: Duration::from_millis(3000),
                max_cooldown_duration: Duration::from_millis(5000),
                base_cooldown_duration: Duration::from_millis(4000),
            },
            resource_requirements: vec![
                ResourceRequirement {
                    resource_type: "qi".to_string(),
                    base_consumption: 50.0,
                    min_consumption: 40.0,
                    max_consumption: 60.0,
                    consumption_formula: "base_consumption * (1 + mastery_bonus * 0.1)".to_string(),
                    required: true,
                },
                ResourceRequirement {
                    resource_type: "stamina".to_string(),
                    base_consumption: 20.0,
                    min_consumption: 15.0,
                    max_consumption: 25.0,
                    consumption_formula: "base_consumption * (1 - efficiency_bonus * 0.2)".to_string(),
                    required: true,
                },
            ],
            execution_conditions: vec![
                ExecutionCondition {
                    condition: "self.qi >= 50".to_string(),
                    condition_type: ExecutionConditionType::ResourceAvailable,
                    required: true,
                    error_message: "Cần ít nhất 50 linh khí để thi triển Hỏa Cầu Thuật".to_string(),
                    condition_value: Some(50.0),
                },
                ExecutionCondition {
                    condition: "self.hp > 0.2".to_string(),
                    condition_type: ExecutionConditionType::SelfHealth,
                    required: true,
                    error_message: "Sức khỏe quá yếu, không thể thi triển kỹ thuật".to_string(),
                    condition_value: Some(0.2),
                },
                ExecutionCondition {
                    condition: "self.cultivation_realm >= 1".to_string(),
                    condition_type: ExecutionConditionType::CultivationRequirement,
                    required: true,
                    error_message: "Cần đạt Kim Đan kỳ 1 để thi triển kỹ thuật này".to_string(),
                    condition_value: Some(1.0),
                },
                ExecutionCondition {
                    condition: "self.fire_affinity >= 30".to_string(),
                    condition_type: ExecutionConditionType::ElementAffinity,
                    required: true,
                    error_message: "Cần độ tương thích hỏa nguyên tố ít nhất 30".to_string(),
                    condition_value: Some(30.0),
                },
            ],
            target_requirements: TargetRequirements {
                target_type: TargetType::Enemy,
                max_range: 20.0,
                line_of_sight: true,
                target_count: 1,
                target_selection: TargetSelection::Single,
                target_conditions: vec![
                    TargetCondition {
                        condition: "target.hp > 0".to_string(),
                        condition_type: TargetConditionType::TargetAlive,
                        required: true,
                        error_message: "Mục tiêu đã chết".to_string(),
                    },
                    TargetCondition {
                        condition: "distance(self, target) <= 20.0".to_string(),
                        condition_type: TargetConditionType::TargetRange,
                        required: true,
                        error_message: "Mục tiêu quá xa".to_string(),
                    },
                ],
            },
            effects: vec![
                ActionEffect {
                    effect_id: "fire_damage".to_string(),
                    effect_type: EffectType::Damage,
                    element: Some("fire".to_string()),
                    base_magnitude: 120.0,
                    min_magnitude: 100.0,
                    max_magnitude: 140.0,
                    magnitude_formula: "base_magnitude * (1 + fire_mastery * 0.02) * (1 + cultivation_bonus * 0.1)".to_string(),
                    trigger_probability: 1.0,
                    duration: 0.0,
                },
                ActionEffect {
                    effect_id: "burning_status".to_string(),
                    effect_type: EffectType::StatusEffect,
                    element: Some("fire".to_string()),
                    base_magnitude: 1.0,
                    min_magnitude: 0.8,
                    max_magnitude: 1.2,
                    magnitude_formula: "base_magnitude * (1 + fire_mastery * 0.01)".to_string(),
                    trigger_probability: 0.3,
                    duration: 8.0,
                },
                ActionEffect {
                    effect_id: "heat_wave".to_string(),
                    effect_type: EffectType::AreaEffect,
                    element: Some("fire".to_string()),
                    base_magnitude: 30.0,
                    min_magnitude: 25.0,
                    max_magnitude: 35.0,
                    magnitude_formula: "base_magnitude * (1 + fire_mastery * 0.01)".to_string(),
                    trigger_probability: 0.8,
                    duration: 2.0,
                },
            ],
            damage_calculation: DamageCalculation {
                base_damage: 120.0,
                damage_type: "fire".to_string(),
                critical_chance: 0.15,
                critical_damage: 1.5,
                damage_formula: "base_damage * (1 + fire_mastery * 0.02) * (1 + cultivation_bonus * 0.1) * (1 + element_amplification * 0.05)".to_string(),
                critical_chance_formula: "base_critical_chance + fire_mastery * 0.001".to_string(),
                critical_damage_formula: "base_critical_damage + fire_mastery * 0.002".to_string(),
            },
            status_effects: vec![
                StatusEffectConfig {
                    status_name: "burning".to_string(),
                    base_probability: 0.3,
                    base_duration: 8.0,
                    base_intensity: 1.0,
                    max_stacks: 3,
                    stackable: true,
                    refresh_duration: true,
                    probability_formula: "base_probability + fire_mastery * 0.005".to_string(),
                    duration_formula: "base_duration * (1 + fire_mastery * 0.05)".to_string(),
                    intensity_formula: "base_intensity * (1 + fire_mastery * 0.02)".to_string(),
                },
            ],
            element_interactions: vec![
                ElementInteraction {
                    interaction_type: "overcoming".to_string(),
                    target_element: "wood".to_string(),
                    damage_multiplier: 1.5,
                    status_probability_bonus: 0.2,
                },
                ElementInteraction {
                    interaction_type: "generating".to_string(),
                    target_element: "earth".to_string(),
                    damage_multiplier: 1.2,
                    status_probability_bonus: 0.1,
                },
                ElementInteraction {
                    interaction_type: "overcome_by".to_string(),
                    target_element: "water".to_string(),
                    damage_multiplier: 0.7,
                    status_probability_bonus: -0.3,
                },
            ],
            cultivation_integration: CultivationIntegration {
                system: "golden_pill".to_string(),
                realm_requirement: 1,
                realm_bonus: 0.1,
                cultivation_stats: vec![
                    CultivationStat {
                        stat_name: "fire_affinity".to_string(),
                        base_value: 30.0,
                        required: true,
                    },
                    CultivationStat {
                        stat_name: "qi_control".to_string(),
                        base_value: 20.0,
                        required: true,
                    },
                    CultivationStat {
                        stat_name: "spiritual_power".to_string(),
                        base_value: 15.0,
                        required: true,
                    },
                ],
            },
        }
    }
    
    /// Calculate damage for fireball technique
    fn calculate_damage(&self, context: &ActionContext) -> Result<DamageResult, ActionError> {
        let mut damage = self.damage_calculation.base_damage;
        
        // Apply fire mastery bonus
        let fire_mastery = context.attacker.get_stat("fire_mastery").unwrap_or(0.0);
        damage *= 1.0 + fire_mastery * 0.02;
        
        // Apply cultivation bonus
        let cultivation_bonus = context.attacker.get_stat("cultivation_bonus").unwrap_or(0.0);
        damage *= 1.0 + cultivation_bonus * 0.1;
        
        // Apply element amplification
        let element_amplification = context.attacker.get_stat("element_amplification").unwrap_or(0.0);
        damage *= 1.0 + element_amplification * 0.05;
        
        // Apply element interactions
        if let Some(target) = &context.target {
            let target_element = target.get_primary_element();
            if let Some(interaction) = self.get_element_interaction("fire", &target_element) {
                damage *= interaction.damage_multiplier;
            }
        }
        
        // Calculate critical hit
        let critical_chance = self.calculate_critical_chance(context)?;
        let is_critical = context.rng.gen::<f64>() < critical_chance;
        
        if is_critical {
            let critical_damage = self.calculate_critical_damage(context)?;
            damage *= critical_damage;
        }
        
        Ok(DamageResult {
            total_damage: damage,
            critical_hit: is_critical,
            critical_damage: if is_critical { self.calculate_critical_damage(context)? } else { 1.0 },
            element_damage: HashMap::from([("fire".to_string(), damage)]),
            status_effects: None,
            resource_damage: None,
        })
    }
    
    /// Calculate critical chance
    fn calculate_critical_chance(&self, context: &ActionContext) -> Result<f64, ActionError> {
        let base_critical = self.damage_calculation.critical_chance;
        let fire_mastery = context.attacker.get_stat("fire_mastery").unwrap_or(0.0);
        let critical_chance = base_critical + fire_mastery * 0.001;
        Ok(critical_chance.min(1.0))
    }
    
    /// Calculate critical damage
    fn calculate_critical_damage(&self, context: &ActionContext) -> Result<f64, ActionError> {
        let base_critical = self.damage_calculation.critical_damage;
        let fire_mastery = context.attacker.get_stat("fire_mastery").unwrap_or(0.0);
        let critical_damage = base_critical + fire_mastery * 0.002;
        Ok(critical_damage)
    }
    
    /// Apply status effects
    fn apply_status_effects(&self, context: &mut ActionContext) -> Result<(), ActionError> {
        for status_config in &self.status_effects {
            let probability = self.calculate_status_probability(status_config, context)?;
            
            if context.rng.gen::<f64>() < probability {
                let duration = self.calculate_status_duration(status_config, context)?;
                let intensity = self.calculate_status_intensity(status_config, context)?;
                
                let status_effect = StatusEffect {
                    name: status_config.status_name.clone(),
                    duration,
                    intensity,
                    max_stacks: status_config.max_stacks,
                    stackable: status_config.stackable,
                    refresh_duration: status_config.refresh_duration,
                };
                
                context.target.as_mut().unwrap().apply_status_effect(status_effect);
            }
        }
        
        Ok(())
    }
    
    /// Calculate status probability
    fn calculate_status_probability(&self, status_config: &StatusEffectConfig, context: &ActionContext) -> Result<f64, ActionError> {
        let base_probability = status_config.base_probability;
        let fire_mastery = context.attacker.get_stat("fire_mastery").unwrap_or(0.0);
        let probability = base_probability + fire_mastery * 0.005;
        Ok(probability.min(1.0))
    }
    
    /// Calculate status duration
    fn calculate_status_duration(&self, status_config: &StatusEffectConfig, context: &ActionContext) -> Result<f64, ActionError> {
        let base_duration = status_config.base_duration;
        let fire_mastery = context.attacker.get_stat("fire_mastery").unwrap_or(0.0);
        let duration = base_duration * (1.0 + fire_mastery * 0.05);
        Ok(duration)
    }
    
    /// Calculate status intensity
    fn calculate_status_intensity(&self, status_config: &StatusEffectConfig, context: &ActionContext) -> Result<f64, ActionError> {
        let base_intensity = status_config.base_intensity;
        let fire_mastery = context.attacker.get_stat("fire_mastery").unwrap_or(0.0);
        let intensity = base_intensity * (1.0 + fire_mastery * 0.02);
        Ok(intensity)
    }
    
    /// Get element interaction
    fn get_element_interaction(&self, attacker_element: &str, target_element: &str) -> Option<&ElementInteraction> {
        self.element_interactions.iter()
            .find(|interaction| interaction.target_element == target_element)
    }
    
    /// Consume resources
    fn consume_resources(&self, context: &mut ActionContext) -> Result<(), ActionError> {
        for requirement in &self.resource_requirements {
            let consumption = self.calculate_resource_consumption(requirement, context)?;
            context.attacker.consume_resource(&requirement.resource_type, consumption)?;
        }
        Ok(())
    }
    
    /// Calculate resource consumption
    fn calculate_resource_consumption(&self, requirement: &ResourceRequirement, context: &ActionContext) -> Result<f64, ActionError> {
        let base_consumption = requirement.base_consumption;
        
        match requirement.resource_type.as_str() {
            "qi" => {
                let mastery_bonus = context.attacker.get_stat("mastery_bonus").unwrap_or(0.0);
                let consumption = base_consumption * (1.0 + mastery_bonus * 0.1);
                Ok(consumption.clamp(requirement.min_consumption, requirement.max_consumption))
            },
            "stamina" => {
                let efficiency_bonus = context.attacker.get_stat("efficiency_bonus").unwrap_or(0.0);
                let consumption = base_consumption * (1.0 - efficiency_bonus * 0.2);
                Ok(consumption.clamp(requirement.min_consumption, requirement.max_consumption))
            },
            _ => Ok(base_consumption),
        }
    }
}

// Supporting types and enums
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMetadata {
    pub id: String,
    pub name: String,
    pub name_en: String,
    pub description: String,
    pub description_en: String,
    pub icon_id: String,
    pub animation_id: String,
    pub sound_effect_id: String,
    pub visual_effects: Vec<String>,
    pub level_requirement: u32,
    pub class_requirements: Vec<String>,
    pub race_requirements: Vec<String>,
    pub cultivation_requirements: CultivationRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CultivationRequirements {
    pub golden_pill_realm: u32,
    pub fire_affinity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionProperties {
    pub target_count: u32,
    pub area_size: f64,
    pub projectile_speed: f64,
    pub projectile_range: f64,
    pub min_execution_duration: Duration,
    pub max_execution_duration: Duration,
    pub base_execution_duration: Duration,
    pub min_cooldown_duration: Duration,
    pub max_cooldown_duration: Duration,
    pub base_cooldown_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirement {
    pub resource_type: String,
    pub base_consumption: f64,
    pub min_consumption: f64,
    pub max_consumption: f64,
    pub consumption_formula: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCondition {
    pub condition: String,
    pub condition_type: ExecutionConditionType,
    pub required: bool,
    pub error_message: String,
    pub condition_value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionConditionType {
    ResourceAvailable,
    SelfHealth,
    CultivationRequirement,
    ElementAffinity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetRequirements {
    pub target_type: TargetType,
    pub max_range: f64,
    pub line_of_sight: bool,
    pub target_count: u32,
    pub target_selection: TargetSelection,
    pub target_conditions: Vec<TargetCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetType {
    Enemy,
    Ally,
    Self,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetSelection {
    Single,
    Multiple,
    Area,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetCondition {
    pub condition: String,
    pub condition_type: TargetConditionType,
    pub required: bool,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetConditionType {
    TargetAlive,
    TargetRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEffect {
    pub effect_id: String,
    pub effect_type: EffectType,
    pub element: Option<String>,
    pub base_magnitude: f64,
    pub min_magnitude: f64,
    pub max_magnitude: f64,
    pub magnitude_formula: String,
    pub trigger_probability: f64,
    pub duration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Damage,
    StatusEffect,
    AreaEffect,
    Buff,
    Debuff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageCalculation {
    pub base_damage: f64,
    pub damage_type: String,
    pub critical_chance: f64,
    pub critical_damage: f64,
    pub damage_formula: String,
    pub critical_chance_formula: String,
    pub critical_damage_formula: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectConfig {
    pub status_name: String,
    pub base_probability: f64,
    pub base_duration: f64,
    pub base_intensity: f64,
    pub max_stacks: u32,
    pub stackable: bool,
    pub refresh_duration: bool,
    pub probability_formula: String,
    pub duration_formula: String,
    pub intensity_formula: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementInteraction {
    pub interaction_type: String,
    pub target_element: String,
    pub damage_multiplier: f64,
    pub status_probability_bonus: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CultivationIntegration {
    pub system: String,
    pub realm_requirement: u32,
    pub realm_bonus: f64,
    pub cultivation_stats: Vec<CultivationStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CultivationStat {
    pub stat_name: String,
    pub base_value: f64,
    pub required: bool,
}

// Action trait and supporting types
pub trait Action {
    fn get_metadata(&self) -> &ActionMetadata;
    fn get_category(&self) -> ActionCategory;
    fn get_type(&self) -> ActionType;
    fn get_resource_requirements(&self) -> &[ResourceRequirement];
    fn get_execution_duration(&self) -> DurationRange;
    fn get_cooldown_duration(&self) -> CooldownConfig;
    fn get_interrupt_conditions(&self) -> &[InterruptCondition];
    fn get_execution_conditions(&self) -> &[ExecutionCondition];
    fn get_target_requirements(&self) -> &TargetRequirements;
    fn get_effects(&self) -> &[ActionEffect];
    fn validate(&self, context: &ActionContext) -> ValidationResult;
    fn execute(&self, context: &mut ActionContext) -> ActionResult;
}

#[derive(Debug, Clone)]
pub enum ActionCategory {
    Combat,
    Cultivation,
    Social,
    Crafting,
}

#[derive(Debug, Clone)]
pub enum ActionType {
    Attack,
    Defense,
    Support,
    Movement,
}

#[derive(Debug, Clone)]
pub struct DurationRange {
    pub min: Duration,
    pub max: Duration,
    pub base: Duration,
}

#[derive(Debug, Clone)]
pub struct CooldownConfig {
    pub min: Duration,
    pub max: Duration,
    pub base: Duration,
    pub interrupt_conditions: Vec<InterruptCondition>,
}

#[derive(Debug, Clone)]
pub struct InterruptCondition {
    pub condition: String,
    pub interrupt_type: InterruptType,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum InterruptType {
    HealthCritical,
    InsufficientQi,
    InsufficientStamina,
    TargetLost,
}

#[derive(Debug, Clone)]
pub struct ActionContext {
    pub attacker: Actor,
    pub target: Option<Actor>,
    pub action_id: String,
    pub rng: Rng,
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub id: String,
    pub stats: HashMap<String, f64>,
    pub resources: HashMap<String, f64>,
    pub status_effects: Vec<StatusEffect>,
}

impl Actor {
    pub fn get_stat(&self, stat_name: &str) -> Option<f64> {
        self.stats.get(stat_name).copied()
    }
    
    pub fn consume_resource(&mut self, resource_type: &str, amount: f64) -> Result<(), ActionError> {
        if let Some(current) = self.resources.get_mut(resource_type) {
            if *current >= amount {
                *current -= amount;
                Ok(())
            } else {
                Err(ActionError::InsufficientResource(resource_type.to_string()))
            }
        } else {
            Err(ActionError::InsufficientResource(resource_type.to_string()))
        }
    }
    
    pub fn apply_status_effect(&mut self, status_effect: StatusEffect) {
        self.status_effects.push(status_effect);
    }
    
    pub fn get_primary_element(&self) -> String {
        // This would be determined by the actor's cultivation path
        "fire".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct StatusEffect {
    pub name: String,
    pub duration: f64,
    pub intensity: f64,
    pub max_stacks: u32,
    pub stackable: bool,
    pub refresh_duration: bool,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub success: bool,
    pub errors: Vec<ActionError>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            success: true,
            errors: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, error: ActionError) {
        self.success = false;
        self.errors.push(error);
    }
}

#[derive(Debug, Clone)]
pub struct ActionResult {
    pub success: bool,
    pub damage: Option<DamageResult>,
    pub status_effects: Option<Vec<StatusEffect>>,
    pub errors: Vec<ActionError>,
}

impl ActionResult {
    pub fn new() -> Self {
        Self {
            success: false,
            damage: None,
            status_effects: None,
            errors: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DamageResult {
    pub total_damage: f64,
    pub critical_hit: bool,
    pub critical_damage: f64,
    pub element_damage: HashMap<String, f64>,
    pub status_effects: Option<Vec<StatusEffect>>,
    pub resource_damage: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone)]
pub enum ActionError {
    ExecutionConditionNotMet(String),
    InsufficientResource(String),
    InvalidTarget,
    InsufficientQi,
    HealthTooLow,
    CultivationInsufficient,
    FireAffinityLow,
    TargetTooFar,
    TargetDead,
}

// RNG trait for random number generation
pub trait Rng {
    fn gen<T>(&mut self) -> T where T: From<f64>;
}

// Mock RNG implementation
pub struct MockRng {
    pub seed: u64,
}

impl Rng for MockRng {
    fn gen<T>(&mut self) -> T where T: From<f64> {
        // Simple mock implementation
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        let normalized = (self.seed >> 16) as f64 / 65536.0;
        T::from(normalized)
    }
}
