//! Resource Regeneration System
//!
//! This module provides a comprehensive resource regeneration system
//! for the Enhanced Hybrid Resource Manager, handling automatic
//! regeneration of various resources over time.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::types::Actor;
use crate::ActorCoreResult;
use super::system_resource_manager::SystemResourceCalculator;
use tracing::{info};

/// Convert string error to ActorCoreError
fn to_actor_core_error(msg: String) -> crate::ActorCoreError {
    crate::ActorCoreError::SubsystemError(msg)
}

/// Resource Regeneration Manager
pub struct ResourceRegenerationManager {
    /// System identifier
    system_id: String,
    /// Regeneration rules
    regeneration_rules: HashMap<String, RegenerationRule>,
    /// Active regeneration tasks
    active_tasks: Arc<RwLock<HashMap<String, RegenerationTask>>>,
    /// Configuration
    #[allow(dead_code)]
    config: RegenerationConfig,
}

/// Regeneration Rule
#[derive(Debug, Clone)]
pub struct RegenerationRule {
    /// Resource name
    pub resource_name: String,
    /// Base regeneration rate per second
    pub base_rate: f64,
    /// Regeneration formula
    pub formula: String,
    /// Regeneration conditions
    pub conditions: Vec<RegenerationCondition>,
    /// Regeneration modifiers
    pub modifiers: Vec<RegenerationModifier>,
}

/// Regeneration Condition
#[derive(Debug, Clone)]
pub enum RegenerationCondition {
    /// Only regenerate when not in combat
    NotInCombat,
    /// Only regenerate when resting
    Resting,
    /// Only regenerate when not moving
    NotMoving,
    /// Only regenerate when above certain health percentage
    HealthAbove(f64),
    /// Only regenerate when below certain health percentage
    HealthBelow(f64),
    /// Only regenerate when certain stat is above threshold
    StatAbove(String, f64),
    /// Only regenerate when certain stat is below threshold
    StatBelow(String, f64),
}

/// Regeneration Modifier
#[derive(Debug, Clone)]
pub enum RegenerationModifier {
    /// Multiply by a factor
    Multiply(f64),
    /// Add a fixed amount
    Add(f64),
    /// Apply based on stat value
    StatBased(String, f64),
    /// Apply based on equipment
    EquipmentBased(String, f64),
    /// Apply based on environment
    EnvironmentBased(String, f64),
}

/// Regeneration Task
#[derive(Debug, Clone)]
pub struct RegenerationTask {
    /// Actor ID
    pub actor_id: String,
    /// Resource name
    pub resource_name: String,
    /// Last update time
    pub last_update: u64,
    /// Current regeneration rate
    pub current_rate: f64,
    /// Total regenerated amount
    pub total_regenerated: f64,
}

/// Regeneration Configuration
#[derive(Debug, Clone)]
pub struct RegenerationConfig {
    /// Update interval in seconds
    pub update_interval: f64,
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: usize,
    /// Enable batch processing
    pub enable_batch_processing: bool,
    /// Batch size
    pub batch_size: usize,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
}

impl Default for RegenerationConfig {
    fn default() -> Self {
        Self {
            update_interval: 1.0, // 1 second
            max_concurrent_tasks: 1000,
            enable_batch_processing: true,
            batch_size: 100,
            enable_monitoring: true,
        }
    }
}

impl ResourceRegenerationManager {
    /// Create a new Resource Regeneration Manager
    pub fn new(config: RegenerationConfig) -> Self {
        let mut manager = Self {
            system_id: "regeneration_system".to_string(),
            regeneration_rules: HashMap::new(),
            active_tasks: Arc::new(RwLock::new(HashMap::new())),
            config,
        };
        
        // Initialize default regeneration rules
        manager.initialize_default_rules();
        manager
    }
    
    /// Initialize default regeneration rules
    fn initialize_default_rules(&mut self) {
        // HP Regeneration
        self.add_regeneration_rule(RegenerationRule {
            resource_name: "hp_current".to_string(),
            base_rate: 0.1, // 0.1 HP per second
            formula: "base_rate * vitality_modifier * rest_modifier".to_string(),
            conditions: vec![
                RegenerationCondition::NotInCombat,
                RegenerationCondition::HealthAbove(0.1), // Only regenerate when above 10% health
            ],
            modifiers: vec![
                RegenerationModifier::StatBased("vitality".to_string(), 0.1),
                RegenerationModifier::EquipmentBased("regeneration_bonus".to_string(), 1.0),
            ],
        });
        
        // MP Regeneration
        self.add_regeneration_rule(RegenerationRule {
            resource_name: "mp_current".to_string(),
            base_rate: 0.2, // 0.2 MP per second
            formula: "base_rate * intelligence_modifier * meditation_modifier".to_string(),
            conditions: vec![
                RegenerationCondition::NotInCombat,
                RegenerationCondition::Resting,
            ],
            modifiers: vec![
                RegenerationModifier::StatBased("intelligence".to_string(), 0.05),
                RegenerationModifier::EquipmentBased("mana_regeneration".to_string(), 1.0),
            ],
        });
        
        // Stamina Regeneration
        self.add_regeneration_rule(RegenerationRule {
            resource_name: "stamina_current".to_string(),
            base_rate: 0.5, // 0.5 stamina per second
            formula: "base_rate * constitution_modifier * rest_modifier".to_string(),
            conditions: vec![
                RegenerationCondition::NotMoving,
            ],
            modifiers: vec![
                RegenerationModifier::StatBased("constitution".to_string(), 0.1),
                RegenerationModifier::EquipmentBased("stamina_regeneration".to_string(), 1.0),
            ],
        });
        
        // Mana Regeneration
        self.add_regeneration_rule(RegenerationRule {
            resource_name: "mana_current".to_string(),
            base_rate: 0.3, // 0.3 mana per second
            formula: "base_rate * wisdom_modifier * meditation_modifier".to_string(),
            conditions: vec![
                RegenerationCondition::NotInCombat,
                RegenerationCondition::Resting,
            ],
            modifiers: vec![
                RegenerationModifier::StatBased("wisdom".to_string(), 0.05),
                RegenerationModifier::EquipmentBased("mana_regeneration".to_string(), 1.0),
            ],
        });
    }
    
    /// Add a regeneration rule
    pub fn add_regeneration_rule(&mut self, rule: RegenerationRule) {
        self.regeneration_rules.insert(rule.resource_name.clone(), rule);
    }
    
    /// Start regeneration for an actor
    pub async fn start_regeneration(&self, actor: &Actor, resource_name: &str) -> ActorCoreResult<()> {
        let task_key = format!("{}:{}", actor.id.to_string(), resource_name);
        
        // Check if task already exists
        {
            let tasks = self.active_tasks.read().await;
            if tasks.contains_key(&task_key) {
                return Ok(()); // Already regenerating
            }
        }
        
        // Check if we have a rule for this resource
        if !self.regeneration_rules.contains_key(resource_name) {
            return Err(to_actor_core_error(format!("No regeneration rule found for resource: {}", resource_name)));
        }
        
        // Create new regeneration task
        let task = RegenerationTask {
            actor_id: actor.id.to_string(),
            resource_name: resource_name.to_string(),
            last_update: chrono::Utc::now().timestamp() as u64,
            current_rate: 0.0,
            total_regenerated: 0.0,
        };
        
        // Add task to active tasks
        {
            let mut tasks = self.active_tasks.write().await;
            tasks.insert(task_key, task);
        }
        
        Ok(())
    }
    
    /// Stop regeneration for an actor
    pub async fn stop_regeneration(&self, actor: &Actor, resource_name: &str) -> ActorCoreResult<()> {
        let task_key = format!("{}:{}", actor.id.to_string(), resource_name);
        
        let mut tasks = self.active_tasks.write().await;
        tasks.remove(&task_key);
        
        Ok(())
    }
    
    /// Update regeneration for all active tasks
    pub async fn update_regeneration(&self, actors: &HashMap<String, Actor>) -> ActorCoreResult<()> {
        let current_time = chrono::Utc::now().timestamp() as u64;
        let mut tasks_to_remove = Vec::new();
        let mut tasks_to_update = Vec::new();
        
        // Get current tasks
        {
            let tasks = self.active_tasks.read().await;
            for (task_key, task) in tasks.iter() {
                let time_delta = current_time - task.last_update;
                if time_delta > 0 {
                    tasks_to_update.push((task_key.clone(), task.clone()));
                }
            }
        }
        
        // Update tasks
        for (task_key, mut task) in tasks_to_update {
            if let Some(actor) = actors.get(&task.actor_id) {
                // Check if regeneration should continue
                if self.should_continue_regeneration(actor, &task.resource_name).await? {
                    // Calculate regeneration amount
                    let time_delta = current_time - task.last_update;
                    let regen_amount = self.calculate_regeneration_amount(actor, &task.resource_name, time_delta as f64).await?;
                    
                    // Update task
                    task.last_update = current_time;
                    task.total_regenerated += regen_amount;
                    
                    // Update actor's resource
                    self.update_actor_resource(actor, &task.resource_name, regen_amount).await?;
                    
                    // Update task in active tasks
                    {
                        let mut tasks = self.active_tasks.write().await;
                        tasks.insert(task_key, task);
                    }
                } else {
                    // Stop regeneration
                    tasks_to_remove.push(task_key);
                }
            } else {
                // Actor not found, remove task
                tasks_to_remove.push(task_key);
            }
        }
        
        // Remove stopped tasks
        {
            let mut tasks = self.active_tasks.write().await;
            for task_key in tasks_to_remove {
                tasks.remove(&task_key);
            }
        }
        
        Ok(())
    }
    
    /// Check if regeneration should continue
    async fn should_continue_regeneration(&self, actor: &Actor, resource_name: &str) -> ActorCoreResult<bool> {
        let rule = self.regeneration_rules.get(resource_name)
            .ok_or_else(|| to_actor_core_error(format!("No regeneration rule found for resource: {}", resource_name)))?;
        
        // Check all conditions
        for condition in &rule.conditions {
            if !self.check_condition(actor, condition).await? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Check a regeneration condition
    async fn check_condition(&self, actor: &Actor, condition: &RegenerationCondition) -> ActorCoreResult<bool> {
        match condition {
            RegenerationCondition::NotInCombat => {
                // Check if actor is in combat
                let data = actor.get_data();
                let in_combat = data.get("in_combat").and_then(|v| v.as_bool()).unwrap_or(false);
                Ok(!in_combat)
            },
            RegenerationCondition::Resting => {
                // Check if actor is resting
                let data = actor.get_data();
                let resting = data.get("resting").and_then(|v| v.as_bool()).unwrap_or(false);
                Ok(resting)
            },
            RegenerationCondition::NotMoving => {
                // Check if actor is moving
                let data = actor.get_data();
                let moving = data.get("moving").and_then(|v| v.as_bool()).unwrap_or(false);
                Ok(!moving)
            },
            RegenerationCondition::HealthAbove(threshold) => {
                // Check if health is above threshold
                let data = actor.get_data();
                let current_hp = data.get("hp_current").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let max_hp = data.get("hp_max").and_then(|v| v.as_f64()).unwrap_or(100.0);
                let health_percentage = current_hp / max_hp;
                Ok(health_percentage > *threshold)
            },
            RegenerationCondition::HealthBelow(threshold) => {
                // Check if health is below threshold
                let data = actor.get_data();
                let current_hp = data.get("hp_current").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let max_hp = data.get("hp_max").and_then(|v| v.as_f64()).unwrap_or(100.0);
                let health_percentage = current_hp / max_hp;
                Ok(health_percentage < *threshold)
            },
            RegenerationCondition::StatAbove(stat_name, threshold) => {
                // Check if stat is above threshold
                let data = actor.get_data();
                let stat_value = data.get(stat_name).and_then(|v| v.as_f64()).unwrap_or(0.0);
                Ok(stat_value > *threshold)
            },
            RegenerationCondition::StatBelow(stat_name, threshold) => {
                // Check if stat is below threshold
                let data = actor.get_data();
                let stat_value = data.get(stat_name).and_then(|v| v.as_f64()).unwrap_or(0.0);
                Ok(stat_value < *threshold)
            },
        }
    }
    
    /// Calculate regeneration amount
    async fn calculate_regeneration_amount(&self, actor: &Actor, resource_name: &str, time_delta: f64) -> ActorCoreResult<f64> {
        let rule = self.regeneration_rules.get(resource_name)
            .ok_or_else(|| to_actor_core_error(format!("No regeneration rule found for resource: {}", resource_name)))?;
        
        let mut regen_amount = rule.base_rate * time_delta;
        
        // Apply modifiers
        for modifier in &rule.modifiers {
            regen_amount = self.apply_modifier(actor, modifier, regen_amount).await?;
        }
        
        // Ensure we don't exceed maximum
        let data = actor.get_data();
        let current_value = data.get(resource_name).and_then(|v| v.as_f64()).unwrap_or(0.0);
        let max_value = data.get(&format!("{}_max", resource_name)).and_then(|v| v.as_f64()).unwrap_or(100.0);
        
        let new_value = (current_value + regen_amount).min(max_value);
        regen_amount = new_value - current_value;
        
        Ok(regen_amount)
    }
    
    /// Apply a regeneration modifier
    async fn apply_modifier(&self, actor: &Actor, modifier: &RegenerationModifier, current_amount: f64) -> ActorCoreResult<f64> {
        match modifier {
            RegenerationModifier::Multiply(factor) => Ok(current_amount * factor),
            RegenerationModifier::Add(amount) => Ok(current_amount + amount),
            RegenerationModifier::StatBased(stat_name, multiplier) => {
                let data = actor.get_data();
                let stat_value = data.get(stat_name).and_then(|v| v.as_f64()).unwrap_or(0.0);
                Ok(current_amount * (1.0 + stat_value * multiplier))
            },
            RegenerationModifier::EquipmentBased(equipment_stat, multiplier) => {
                let data = actor.get_data();
                let equipment_value = data.get(equipment_stat).and_then(|v| v.as_f64()).unwrap_or(0.0);
                Ok(current_amount * (1.0 + equipment_value * multiplier))
            },
            RegenerationModifier::EnvironmentBased(environment_stat, multiplier) => {
                let data = actor.get_data();
                let environment_value = data.get(environment_stat).and_then(|v| v.as_f64()).unwrap_or(0.0);
                Ok(current_amount * (1.0 + environment_value * multiplier))
            },
        }
    }
    
    /// Update actor's resource value
    async fn update_actor_resource(&self, actor: &Actor, resource_name: &str, amount: f64) -> ActorCoreResult<()> {
        // This would update the actor's resource value
        // In practice, you'd call the appropriate system to update the resource
        info!(resource = %resource_name, actor_id = %actor.id, amount = amount, "Updating resource by amount");
        Ok(())
    }
    
    /// Get regeneration statistics
    pub async fn get_regeneration_stats(&self) -> ActorCoreResult<RegenerationStats> {
        let tasks = self.active_tasks.read().await;
        
        let mut total_tasks = 0;
        let mut total_regenerated = 0.0;
        let mut resource_stats = HashMap::new();
        
        for task in tasks.values() {
            total_tasks += 1;
            total_regenerated += task.total_regenerated;
            
            let entry = resource_stats.entry(task.resource_name.clone()).or_insert(0.0);
            *entry += task.total_regenerated;
        }
        
        Ok(RegenerationStats {
            total_tasks,
            total_regenerated,
            resource_stats,
        })
    }
}

/// Regeneration Statistics
#[derive(Debug, Clone)]
pub struct RegenerationStats {
    /// Total number of active tasks
    pub total_tasks: usize,
    /// Total amount regenerated across all resources
    pub total_regenerated: f64,
    /// Statistics per resource
    pub resource_stats: HashMap<String, f64>,
}

#[async_trait]
impl SystemResourceCalculator for ResourceRegenerationManager {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn affects_resource(&self, resource_id: &str) -> bool {
        self.regeneration_rules.contains_key(resource_id)
    }
    
    async fn calculate_resources(&self, _actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        // Regeneration manager doesn't calculate resources directly
        // It manages the regeneration process
        Ok(HashMap::new())
    }
    
    async fn notify_stat_change(&self, _actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()> {
        // Check if this stat change affects any regeneration rules
        for stat_id in changed_stats {
            for (resource_name, rule) in &self.regeneration_rules {
                for modifier in &rule.modifiers {
                    if let RegenerationModifier::StatBased(stat_name, _) = modifier {
                        if stat_name == stat_id {
                            // This stat change affects regeneration for this resource
                            info!(resource = %resource_name, stat = %stat_id, "Regeneration affected by stat change");
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn get_resource_dependencies(&self) -> Vec<String> {
        let mut dependencies = Vec::new();
        for rule in self.regeneration_rules.values() {
            for modifier in &rule.modifiers {
                if let RegenerationModifier::StatBased(stat_name, _) = modifier {
                    dependencies.push(stat_name.clone());
                }
            }
        }
        dependencies.sort();
        dependencies.dedup();
        dependencies
    }
    
    fn get_resource_categories(&self) -> Vec<super::system_resource_manager::ResourceCategory> {
        use super::system_resource_manager::ResourceCategory;
        vec![
            ResourceCategory::Health,
            ResourceCategory::Energy,
            ResourceCategory::Physical,
            ResourceCategory::Cultivation,
        ]
    }
    
    async fn is_active(&self, _actor: &Actor) -> ActorCoreResult<bool> {
        // Regeneration system is always active
        Ok(true)
    }
}