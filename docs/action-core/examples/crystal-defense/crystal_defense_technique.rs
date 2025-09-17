// Crystal Defense Technique Implementation
// A powerful defensive technique that crystallizes the user and provides massive defense bonuses

use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};

use crate::action_core::{
    Action, ActionCategory, ActionType, ActionMetadata, ActionContext, ActionResult,
    ValidationResult, ActionError, ResourceRequirement, ExecutionCondition,
    InterruptCondition, TargetRequirements, ActionEffect, CooldownConfig, DurationRange
};
use crate::element_core::ElementDerivedStats;
use crate::combat_core::CombatDerivedStats;
use crate::resource_manager::ResourceStats;

/// Crystal Defense Technique Action
/// 
/// This action crystallizes the user, making them immobile but providing
/// massive defense bonuses for all physical and elemental attributes.
/// 
/// Base Stats (before derived stats):
/// - Defense Multiplier: 20x
/// - Defense Bonus: +100,000
/// - Duration: 5 seconds
/// - Immobilized during effect
/// - High elemental resistance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystalDefenseTechnique {
    metadata: ActionMetadata,
    defense_properties: CrystalDefenseProperties,
    resource_requirements: Vec<ResourceRequirement>,
    execution_conditions: Vec<ExecutionCondition>,
    interrupt_conditions: Vec<InterruptCondition>,
    target_requirements: TargetRequirements,
    effects: Vec<ActionEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystalDefenseProperties {
    /// Base defense multiplier (20x)
    pub defense_multiplier: f64,
    /// Absolute defense bonus (+100,000)
    pub defense_bonus: f64,
    /// Base duration in seconds (5.0s)
    pub base_duration: f64,
    /// Elemental resistance bonus (80%)
    pub elemental_resistance_bonus: f64,
    /// Resource costs
    pub mana_cost: f64,
    pub qi_cost: f64,
    pub stamina_cost: f64,
}

impl CrystalDefenseTechnique {
    pub fn new() -> Self {
        Self {
            metadata: ActionMetadata {
                id: "crystal_defense_technique".to_string(),
                name: "Crystal Defense Technique".to_string(),
                name_vi: "Kỹ Thuật Phòng Thủ Tinh Thể".to_string(),
                description: "The user crystallizes into a crystal form, becoming immobile but gaining massive defense bonuses for all physical and elemental attributes.".to_string(),
                description_vi: "Người thi triển kết tinh thành tinh thể, không thể di chuyển, tăng defense point của các thuộc tính thuộc elemental category vật lý và nguyên tố lên 20 lần chỉ số của người thi triển, cộng thêm tuyệt đối 100000 điểm.".to_string(),
                category: ActionCategory::Combat,
                type_: ActionType::Defense,
                level: 1,
                max_level: 10,
                rarity: "Epic".to_string(),
            },
            defense_properties: CrystalDefenseProperties {
                defense_multiplier: 20.0,
                defense_bonus: 100000.0,
                base_duration: 5.0,
                elemental_resistance_bonus: 0.8,
                mana_cost: 500.0,
                qi_cost: 300.0,
                stamina_cost: 200.0,
            },
            resource_requirements: vec![
                ResourceRequirement {
                    resource_type: "Mana".to_string(),
                    min_value: 500.0,
                    max_value: 1000.0,
                    scaling_factor: 1.1,
                },
                ResourceRequirement {
                    resource_type: "Qi".to_string(),
                    min_value: 300.0,
                    max_value: 600.0,
                    scaling_factor: 1.05,
                },
                ResourceRequirement {
                    resource_type: "Stamina".to_string(),
                    min_value: 200.0,
                    max_value: 400.0,
                    scaling_factor: 1.0,
                },
            ],
            execution_conditions: vec![
                ExecutionCondition {
                    condition: "self.health_percentage > 0.1".to_string(),
                    description: "Must have at least 10% health".to_string(),
                },
                ExecutionCondition {
                    condition: "self.mana_percentage > 0.3".to_string(),
                    description: "Must have at least 30% mana".to_string(),
                },
                ExecutionCondition {
                    condition: "self.qi_percentage > 0.2".to_string(),
                    description: "Must have at least 20% qi".to_string(),
                },
                ExecutionCondition {
                    condition: "self.status_effects.crystallized == false".to_string(),
                    description: "Cannot be already crystallized".to_string(),
                },
                ExecutionCondition {
                    condition: "self.movement_state == stationary".to_string(),
                    description: "Must be stationary to crystallize".to_string(),
                },
            ],
            interrupt_conditions: vec![],
            target_requirements: TargetRequirements {
                target_type: "Self".to_string(),
                target_count: 1,
                target_selection: "Self".to_string(),
                area_size: Some(0.0),
            },
            effects: vec![],
        }
    }

    /// Calculate defense bonus based on actor's stats and derived stats
    pub fn calculate_defense_bonus(
        &self,
        actor_defense_point: f64,
        actor_physical_defense: f64,
        actor_elemental_defense: f64,
        earth_mastery: f64,
        metal_mastery: f64,
        derived_stats: &ElementDerivedStats
    ) -> f64 {
        // Base defense calculation
        let base_defense = actor_defense_point + actor_physical_defense + actor_elemental_defense;
        
        // Elemental mastery bonus
        let elemental_bonus = (earth_mastery + metal_mastery) * 0.0001; // 0.01% per mastery point
        
        // Apply defense multiplier and bonus
        let final_defense = (base_defense * self.defense_properties.defense_multiplier + self.defense_properties.defense_bonus) 
            * (1.0 + elemental_bonus);
        
        // Apply derived stats bonuses
        let derived_bonus = derived_stats.defense_point * 0.1; // 10% of derived defense point
        
        final_defense + derived_bonus
    }

    /// Calculate duration based on level and mastery
    pub fn calculate_duration(
        &self,
        action_level: u32,
        earth_mastery: f64,
        metal_mastery: f64,
        derived_stats: &ElementDerivedStats
    ) -> Duration {
        let base_duration = self.defense_properties.base_duration;
        
        // Level bonus
        let level_bonus = action_level as f64 * 0.1; // 10% per level
        
        // Mastery bonus
        let mastery_bonus = (earth_mastery + metal_mastery) * 0.0001; // 0.01% per mastery point
        
        // Derived stats bonus
        let derived_bonus = derived_stats.skill_execution_speed * 0.05; // 5% per execution speed
        
        let final_duration = base_duration * (1.0 + level_bonus + mastery_bonus + derived_bonus);
        
        Duration::from_secs_f64(final_duration)
    }

    /// Calculate resource cost with efficiency
    pub fn calculate_resource_cost(
        &self,
        derived_stats: &ElementDerivedStats
    ) -> HashMap<String, f64> {
        let efficiency_bonus = derived_stats.resource_efficiency * 0.1; // 10% per efficiency
        
        let mut costs = HashMap::new();
        costs.insert("Mana".to_string(), self.defense_properties.mana_cost * (1.0 - efficiency_bonus));
        costs.insert("Qi".to_string(), self.defense_properties.qi_cost * (1.0 - efficiency_bonus));
        costs.insert("Stamina".to_string(), self.defense_properties.stamina_cost * (1.0 - efficiency_bonus));
        
        costs
    }

    /// Calculate elemental resistance bonus
    pub fn calculate_elemental_resistance(
        &self,
        base_resistance: f64,
        earth_mastery: f64,
        metal_mastery: f64
    ) -> f64 {
        let mastery_bonus = (earth_mastery + metal_mastery) * 0.0002; // 0.02% per mastery point
        base_resistance + self.defense_properties.elemental_resistance_bonus + mastery_bonus
    }
}

impl Action for CrystalDefenseTechnique {
    fn get_metadata(&self) -> &ActionMetadata {
        &self.metadata
    }

    fn get_category(&self) -> ActionCategory {
        ActionCategory::Combat
    }

    fn get_type(&self) -> ActionType {
        ActionType::Defense
    }

    fn get_resource_requirements(&self) -> &[ResourceRequirement] {
        &self.resource_requirements
    }

    fn get_execution_duration(&self) -> DurationRange {
        DurationRange {
            min: Duration::from_secs_f64(2.0),
            max: Duration::from_secs_f64(3.0),
            base: Duration::from_secs_f64(2.5),
        }
    }

    fn get_cooldown_duration(&self) -> CooldownConfig {
        CooldownConfig {
            min: Duration::from_secs_f64(60.0),
            max: Duration::from_secs_f64(120.0),
            base: Duration::from_secs_f64(90.0),
            cooldown_conditions: vec![],
            interrupt_affects_cooldown: false,
        }
    }

    fn get_interrupt_conditions(&self) -> &[InterruptCondition] {
        &self.interrupt_conditions
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

        // Validate health requirement
        if context.actor.get_health_percentage() < 0.1 {
            result.add_error(ActionError::HealthTooLow);
        }

        // Validate mana requirement
        if context.actor.get_mana_percentage() < 0.3 {
            result.add_error(ActionError::InsufficientResource("Mana".to_string()));
        }

        // Validate qi requirement
        if context.actor.get_qi_percentage() < 0.2 {
            result.add_error(ActionError::InsufficientResource("Qi".to_string()));
        }

        // Validate not already crystallized
        if context.actor.has_status_effect("crystallized") {
            result.add_error(ActionError::StatusEffectAlreadyActive("crystallized".to_string()));
        }

        // Validate stationary state
        if !context.actor.is_stationary() {
            result.add_error(ActionError::MovementRequirementNotMet);
        }

        result
    }

    fn execute(&self, context: &mut ActionContext) -> ActionResult {
        let mut result = ActionResult::new();

        // Get derived stats
        let derived_stats = context.derived_stats.as_ref()
            .ok_or_else(|| ActionError::DerivedStatsNotAvailable)?;

        // Calculate resource costs
        let resource_costs = self.calculate_resource_cost(&derived_stats.element_derived_stats);

        // Consume resources
        for (resource_type, cost) in &resource_costs {
            context.actor.consume_resource(resource_type, *cost)?;
        }

        // Calculate defense bonus
        let defense_bonus = self.calculate_defense_bonus(
            context.actor.get_defense_point(),
            context.actor.get_physical_defense(),
            context.actor.get_elemental_defense(),
            context.actor.get_element_mastery("earth"),
            context.actor.get_element_mastery("metal"),
            &derived_stats.element_derived_stats
        );

        // Calculate duration
        let duration = self.calculate_duration(
            self.metadata.level,
            context.actor.get_element_mastery("earth"),
            context.actor.get_element_mastery("metal"),
            &derived_stats.element_derived_stats
        );

        // Apply crystallization status effect
        let crystallization_effect = CrystallizationEffect {
            defense_bonus,
            duration,
            elemental_resistance: self.calculate_elemental_resistance(
                0.0, // base resistance
                context.actor.get_element_mastery("earth"),
                context.actor.get_element_mastery("metal")
            ),
        };

        context.actor.apply_status_effect("crystallized", Box::new(crystallization_effect))?;

        // Set result
        result.success = true;
        result.defense_result = Some(DefenseResult {
            defense_type: DefenseType::Crystallization,
            defense_success: true,
            damage_mitigation: 1.0, // Complete damage mitigation
            counter_attack: None,
            defense_failure_reason: None,
        });

        result.resource_consumed = resource_costs;
        result.status_effects_applied = vec![
            StatusEffect {
                effect_name: "crystallized".to_string(),
                duration,
                magnitude: 1.0,
                target: "Self".to_string(),
            }
        ];

        Ok(result)
    }
}

/// Crystallization status effect
#[derive(Debug, Clone)]
pub struct CrystallizationEffect {
    pub defense_bonus: f64,
    pub duration: Duration,
    pub elemental_resistance: f64,
}

impl CrystallizationEffect {
    pub fn apply_effects(&self, actor: &mut Actor) -> Result<(), ActionError> {
        // Apply defense bonus
        actor.add_defense_bonus(self.defense_bonus);
        
        // Apply elemental resistance
        actor.add_elemental_resistance(self.elemental_resistance);
        
        // Immobilize actor
        actor.set_movement_restricted(true);
        
        // Apply status immunity
        actor.add_status_immunity(vec![
            "stun".to_string(),
            "knockback".to_string(),
            "movement_impairment".to_string(),
        ]);
        
        Ok(())
    }

    pub fn remove_effects(&self, actor: &mut Actor) -> Result<(), ActionError> {
        // Remove defense bonus
        actor.remove_defense_bonus(self.defense_bonus);
        
        // Remove elemental resistance
        actor.remove_elemental_resistance(self.elemental_resistance);
        
        // Restore movement
        actor.set_movement_restricted(false);
        
        // Remove status immunity
        actor.remove_status_immunity(vec![
            "stun".to_string(),
            "knockback".to_string(),
            "movement_impairment".to_string(),
        ]);
        
        // Apply fatigue effect
        if rand::random::<f64>() < 0.3 {
            actor.apply_status_effect("crystal_fatigue", Box::new(CrystalFatigueEffect::new()))?;
        }
        
        Ok(())
    }
}

/// Crystal fatigue effect after crystallization ends
#[derive(Debug, Clone)]
pub struct CrystalFatigueEffect {
    pub duration: Duration,
}

impl CrystalFatigueEffect {
    pub fn new() -> Self {
        Self {
            duration: Duration::from_secs_f64(10.0),
        }
    }
}

// Supporting types
#[derive(Debug, Clone)]
pub struct DefenseResult {
    pub defense_type: DefenseType,
    pub defense_success: bool,
    pub damage_mitigation: f64,
    pub counter_attack: Option<CounterAttack>,
    pub defense_failure_reason: Option<DefenseFailureReason>,
}

#[derive(Debug, Clone)]
pub enum DefenseType {
    Block,
    Parry,
    Dodge,
    Shield,
    Crystallization,
}

#[derive(Debug, Clone)]
pub struct CounterAttack {
    pub attack_id: String,
    pub damage: f64,
    pub element_type: String,
}

#[derive(Debug, Clone)]
pub enum DefenseFailureReason {
    HealthTooLow,
    InsufficientResource(String),
    StatusEffectAlreadyActive(String),
    MovementRequirementNotMet,
    TimingRequirementNotMet,
}

#[derive(Debug, Clone)]
pub struct StatusEffect {
    pub effect_name: String,
    pub duration: Duration,
    pub magnitude: f64,
    pub target: String,
}

// Mock Actor trait for demonstration
pub trait Actor {
    fn get_health_percentage(&self) -> f64;
    fn get_mana_percentage(&self) -> f64;
    fn get_qi_percentage(&self) -> f64;
    fn get_defense_point(&self) -> f64;
    fn get_physical_defense(&self) -> f64;
    fn get_elemental_defense(&self) -> f64;
    fn get_element_mastery(&self, element: &str) -> f64;
    fn has_status_effect(&self, effect: &str) -> bool;
    fn is_stationary(&self) -> bool;
    fn consume_resource(&mut self, resource_type: &str, amount: f64) -> Result<(), ActionError>;
    fn apply_status_effect(&mut self, effect_name: &str, effect: Box<dyn StatusEffect>) -> Result<(), ActionError>;
    fn add_defense_bonus(&mut self, bonus: f64);
    fn remove_defense_bonus(&mut self, bonus: f64);
    fn add_elemental_resistance(&mut self, resistance: f64);
    fn remove_elemental_resistance(&mut self, resistance: f64);
    fn set_movement_restricted(&mut self, restricted: bool);
    fn add_status_immunity(&mut self, statuses: Vec<String>);
    fn remove_status_immunity(&mut self, statuses: Vec<String>);
}

pub trait StatusEffect {
    fn apply_effects(&self, actor: &mut Actor) -> Result<(), ActionError>;
    fn remove_effects(&self, actor: &mut Actor) -> Result<(), ActionError>;
    fn get_duration(&self) -> Duration;
    fn is_expired(&self) -> bool;
}

impl StatusEffect for CrystallizationEffect {
    fn apply_effects(&self, actor: &mut Actor) -> Result<(), ActionError> {
        self.apply_effects(actor)
    }

    fn remove_effects(&self, actor: &mut Actor) -> Result<(), ActionError> {
        self.remove_effects(actor)
    }

    fn get_duration(&self) -> Duration {
        self.duration
    }

    fn is_expired(&self) -> bool {
        // This would be implemented with a timer in real code
        false
    }
}

impl StatusEffect for CrystalFatigueEffect {
    fn apply_effects(&self, actor: &mut Actor) -> Result<(), ActionError> {
        // Apply fatigue effects (reduced movement speed, etc.)
        Ok(())
    }

    fn remove_effects(&self, actor: &mut Actor) -> Result<(), ActionError> {
        // Remove fatigue effects
        Ok(())
    }

    fn get_duration(&self) -> Duration {
        self.duration
    }

    fn is_expired(&self) -> bool {
        // This would be implemented with a timer in real code
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_defense_technique_creation() {
        let action = CrystalDefenseTechnique::new();
        assert_eq!(action.metadata.id, "crystal_defense_technique");
        assert_eq!(action.defense_properties.defense_multiplier, 20.0);
        assert_eq!(action.defense_properties.defense_bonus, 100000.0);
    }

    #[test]
    fn test_defense_bonus_calculation() {
        let action = CrystalDefenseTechnique::new();
        let derived_stats = ElementDerivedStats::new();
        
        let defense_bonus = action.calculate_defense_bonus(
            1000.0, // defense_point
            500.0,  // physical_defense
            300.0,  // elemental_defense
            1000.0, // earth_mastery
            1000.0, // metal_mastery
            &derived_stats
        );
        
        // Expected: (1000 + 500 + 300) * 20 + 100000 = 136000
        assert!(defense_bonus > 130000.0);
        assert!(defense_bonus < 140000.0);
    }

    #[test]
    fn test_duration_calculation() {
        let action = CrystalDefenseTechnique::new();
        let derived_stats = ElementDerivedStats::new();
        
        let duration = action.calculate_duration(
            5,      // action_level
            1000.0, // earth_mastery
            1000.0, // metal_mastery
            &derived_stats
        );
        
        // Expected: 5.0 * (1 + 0.5 + 0.2) = 8.5 seconds
        assert!(duration.as_secs_f64() > 8.0);
        assert!(duration.as_secs_f64() < 9.0);
    }

    #[test]
    fn test_resource_cost_calculation() {
        let action = CrystalDefenseTechnique::new();
        let mut derived_stats = ElementDerivedStats::new();
        derived_stats.resource_efficiency = 0.5; // 50% efficiency
        
        let costs = action.calculate_resource_cost(&derived_stats);
        
        // Expected: base_cost * (1 - 0.5) = base_cost * 0.5
        assert_eq!(costs["Mana"], 250.0); // 500 * 0.5
        assert_eq!(costs["Qi"], 150.0);   // 300 * 0.5
        assert_eq!(costs["Stamina"], 100.0); // 200 * 0.5
    }
}
