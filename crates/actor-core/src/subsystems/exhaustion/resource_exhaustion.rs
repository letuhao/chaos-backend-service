//! Resource Exhaustion System
//!
//! This subsystem implements the Resource Exhaustion System that defines breakpoints
//! and effects that apply when an actor's resources fall to critical thresholds.
//! It operates globally (outside of combat) and inside combat, with deterministic
//! evaluation and event publication.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::interfaces::Subsystem;
use crate::types::{Actor, SubsystemOutput, Snapshot};
use crate::ActorCoreResult;

/// Resource Exhaustion Subsystem
pub struct ResourceExhaustionSubsystem {
    /// Unique system identifier
    system_id: String,
    /// Processing priority
    priority: i64,
    /// Exhaustion engine for evaluation
    engine: Arc<ExhaustionEngine>,
    /// Event publisher for exhaustion events
    event_publisher: Arc<dyn ExhaustionEventPublisher + Send + Sync>,
    /// State tracker for active effects
    active_effects: Arc<RwLock<HashMap<String, HashMap<String, ExhaustionState>>>>,
}

/// Exhaustion configuration loaded from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExhaustionConfig {
    /// Configuration version
    pub version: u32,
    /// Default hysteresis value (0.0 to 1.0)
    pub hysteresis_default: f64,
    /// Event configuration
    pub events: EventConfig,
    /// Resource category priorities for simultaneous exhaustion
    pub priorities: Option<PriorityConfig>,
    /// Archetype-specific configurations
    pub archetypes: HashMap<String, ArchetypeConfig>,
}

/// Event configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConfig {
    /// Coalescing window in milliseconds
    pub coalesce_window_ms: u64,
}

/// Priority configuration for resource categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityConfig {
    /// Ordered list of resource categories by priority (highest first)
    pub categories: Vec<String>,
}

/// Archetype configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypeConfig {
    /// Resource-specific configurations
    #[serde(flatten)]
    pub resources: HashMap<String, ResourceConfig>,
}

/// Resource configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    /// Thresholds for this resource
    pub thresholds: Vec<ThresholdConfig>,
}

/// Threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThresholdConfig {
    /// Unique threshold identifier
    pub id: String,
    /// Processing order (lower numbers processed first)
    pub order: Option<u32>,
    /// Enter condition: percentage less than or equal
    pub enter_percent_lte: Option<f64>,
    /// Exit condition: percentage greater than or equal
    pub exit_percent_gte: Option<f64>,
    /// Enter condition: exact value
    pub enter_value_eq: Option<f64>,
    /// Exit condition: value greater than or equal
    pub exit_value_ge: Option<f64>,
    /// Effects to apply when threshold is entered
    pub effects: Vec<EffectConfig>,
}

/// Effect configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EffectConfig {
    /// Effect type
    #[serde(rename = "type")]
    pub effect_type: String,
    /// Effect values (for list-based effects)
    pub values: Option<Vec<String>>,
    /// Effect categories (for multiplier effects)
    pub categories: Option<Vec<String>>,
    /// Effect modifier value
    pub modifier: Option<f64>,
    /// Effect name (for flag effects)
    pub name: Option<String>,
    /// Effect value (for flag effects)
    pub value: Option<bool>,
    /// Effect level (for stagger effects)
    pub level: Option<String>,
    /// Effect resource (for regen effects)
    pub resource: Option<String>,
}

/// Exhaustion state for an actor
#[derive(Debug, Clone)]
pub struct ExhaustionState {
    /// Currently active thresholds
    pub active_thresholds: HashMap<String, ThresholdState>,
    /// Last evaluation timestamp
    pub last_evaluation: DateTime<Utc>,
    /// Coalescing window state
    pub coalescing_state: CoalescingState,
}

/// Threshold state
#[derive(Debug, Clone)]
pub struct ThresholdState {
    /// Threshold ID
    pub threshold_id: String,
    /// Whether currently active
    pub is_active: bool,
    /// When it was activated
    pub activated_at: DateTime<Utc>,
    /// Applied effects
    pub applied_effects: Vec<EffectConfig>,
}

/// Coalescing state for event deduplication
#[derive(Debug, Clone)]
pub struct CoalescingState {
    /// Pending events within coalescing window
    pub pending_events: Vec<ExhaustionEvent>,
    /// Last event timestamp
    pub last_event_time: DateTime<Utc>,
}

/// Exhaustion event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExhaustionEvent {
    /// Event type
    pub event_type: ExhaustionEventType,
    /// Actor ID
    pub actor_id: String,
    /// Resource type
    pub resource_type: String,
    /// Threshold ID
    pub threshold_id: String,
    /// Effects applied/removed
    pub effects: Vec<EffectConfig>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Idempotency key
    pub idempotency_key: String,
    /// Whether this event was coalesced
    pub coalesced: bool,
}

/// Exhaustion event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExhaustionEventType {
    /// Resource exhausted (threshold entered)
    ResourceExhausted,
    /// Resource recovered (threshold exited)
    ResourceRecovered,
}

/// Exhaustion transition
#[derive(Debug, Clone)]
pub struct ExhaustionTransition {
    /// Resource name
    pub resource: String,
    /// Threshold ID
    pub threshold_id: String,
    /// Whether entering the threshold
    pub entering: bool,
    /// Effects to apply/remove
    pub effects: Vec<EffectConfig>,
}

/// Exhaustion engine for evaluation
#[derive(Debug)]
pub struct ExhaustionEngine {
    /// Configuration
    config: Arc<RwLock<ExhaustionConfig>>,
    /// Actor state cache to track previous threshold states
    actor_states: Arc<RwLock<HashMap<String, HashMap<String, bool>>>>,
}

/// Exhaustion event publisher trait
#[async_trait]
pub trait ExhaustionEventPublisher: Send + Sync {
    /// Publish an exhaustion event
    async fn publish_event(&self, event: ExhaustionEvent) -> ActorCoreResult<()>;
}

/// Exhaustion error types
#[derive(Debug, thiserror::Error)]
pub enum ExhaustionError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Unknown resource: {0}")]
    UnknownResource(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Evaluation error: {0}")]
    EvaluationError(String),
    #[error("Effect application error: {0}")]
    EffectApplicationError(String),
}

impl From<ExhaustionError> for crate::ActorCoreError {
    fn from(err: ExhaustionError) -> Self {
        crate::ActorCoreError::ConfigurationError(err.to_string())
    }
}

impl ResourceExhaustionSubsystem {
    /// Create a new Resource Exhaustion Subsystem
    pub fn new(
        config: ExhaustionConfig,
        event_publisher: Arc<dyn ExhaustionEventPublisher + Send + Sync>,
    ) -> Self {
        let config = Arc::new(RwLock::new(config));
        let engine = Arc::new(ExhaustionEngine::new(config.clone()));
        
        Self {
            system_id: "resource_exhaustion".to_string(),
            priority: 200, // High priority to process after resource calculations
            engine,
            event_publisher,
            active_effects: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Load configuration from file
    pub async fn load_from_file(path: &str) -> ActorCoreResult<ExhaustionConfig> {
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| ExhaustionError::StorageError(format!("Failed to read config file: {}", e)))?;
        
        let config: ExhaustionConfig = serde_yaml::from_str(&content)
            .map_err(|e| ExhaustionError::InvalidConfig(format!("Failed to parse YAML: {}", e)))?;
        
        // Validate configuration
        Self::validate_config(&config)?;
        
        Ok(config)
    }

    /// Validate configuration
    pub fn validate_config(config: &ExhaustionConfig) -> ActorCoreResult<()> {
        // Validate version
        if config.version == 0 {
            return Err(ExhaustionError::InvalidConfig("Version must be >= 1".to_string()).into());
        }

        // Validate hysteresis default
        if config.hysteresis_default < 0.0 || config.hysteresis_default > 1.0 {
            return Err(ExhaustionError::InvalidConfig(
                "Hysteresis default must be between 0.0 and 1.0".to_string()
            ).into());
        }

        // Validate archetypes and thresholds
        for (archetype_name, archetype_config) in &config.archetypes {
            for (resource_name, resource_config) in &archetype_config.resources {
                let mut threshold_ids = std::collections::HashSet::new();
                
                for threshold in &resource_config.thresholds {
                    // Validate threshold ID uniqueness
                    if !threshold_ids.insert(&threshold.id) {
                        return Err(ExhaustionError::InvalidConfig(
                            format!("Duplicate threshold ID '{}' in {}.{}", 
                                threshold.id, archetype_name, resource_name)
                        ).into());
                    }

                    // Validate enter condition
                    let has_enter_percent = threshold.enter_percent_lte.is_some();
                    let has_enter_value = threshold.enter_value_eq.is_some();
                    
                    if !has_enter_percent && !has_enter_value {
                        return Err(ExhaustionError::InvalidConfig(
                            format!("Threshold '{}' must have either enter_percent_lte or enter_value_eq", 
                                threshold.id)
                        ).into());
                    }

                    if has_enter_percent && has_enter_value {
                        return Err(ExhaustionError::InvalidConfig(
                            format!("Threshold '{}' cannot have both enter_percent_lte and enter_value_eq", 
                                threshold.id)
                        ).into());
                    }

                    // Validate exit condition
                    if let Some(enter_percent) = threshold.enter_percent_lte {
                        if let Some(exit_percent) = threshold.exit_percent_gte {
                            if exit_percent < enter_percent {
                                return Err(ExhaustionError::InvalidConfig(
                                    format!("Threshold '{}' exit_percent_gte ({}) must be >= enter_percent_lte ({})", 
                                        threshold.id, exit_percent, enter_percent)
                                ).into());
                            }
                        }
                    }

                    if let Some(enter_value) = threshold.enter_value_eq {
                        if let Some(exit_value) = threshold.exit_value_ge {
                            if exit_value < enter_value {
                                return Err(ExhaustionError::InvalidConfig(
                                    format!("Threshold '{}' exit_value_ge ({}) must be >= enter_value_eq ({})", 
                                        threshold.id, exit_value, enter_value)
                                ).into());
                            }
                        }
                    }

                    // Validate effects
                    if threshold.effects.is_empty() {
                        return Err(ExhaustionError::InvalidConfig(
                            format!("Threshold '{}' must have at least one effect", threshold.id)
                        ).into());
                    }

                    for effect in &threshold.effects {
                        Self::validate_effect(effect)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate effect configuration
    fn validate_effect(effect: &EffectConfig) -> ActorCoreResult<()> {
        match effect.effect_type.as_str() {
            "disable_tags" | "disable_cost_type" | "break_active_shields" | "action_lockout" => {
                if effect.values.is_none() || effect.values.as_ref().unwrap().is_empty() {
                    return Err(ExhaustionError::InvalidConfig(
                        format!("Effect '{}' requires non-empty values", effect.effect_type)
                    ).into());
                }
            }
            "damage_multiplier" | "incoming_multiplier" => {
                if effect.categories.is_none() || effect.categories.as_ref().unwrap().is_empty() {
                    return Err(ExhaustionError::InvalidConfig(
                        format!("Effect '{}' requires non-empty categories", effect.effect_type)
                    ).into());
                }
                if effect.modifier.is_none() {
                    return Err(ExhaustionError::InvalidConfig(
                        format!("Effect '{}' requires modifier", effect.effect_type)
                    ).into());
                }
            }
            "cast_time_modifier" | "move_speed_modifier" | "taunt_effectiveness_modifier" => {
                if effect.modifier.is_none() {
                    return Err(ExhaustionError::InvalidConfig(
                        format!("Effect '{}' requires modifier", effect.effect_type)
                    ).into());
                }
            }
            "set_flag" => {
                if effect.name.is_none() || effect.value.is_none() {
                    return Err(ExhaustionError::InvalidConfig(
                        "Effect 'set_flag' requires name and value".to_string()
                    ).into());
                }
            }
            "stagger_susceptibility" => {
                if effect.level.is_none() {
                    return Err(ExhaustionError::InvalidConfig(
                        "Effect 'stagger_susceptibility' requires level".to_string()
                    ).into());
                }
                let level = effect.level.as_ref().unwrap();
                if !["light", "medium", "heavy"].contains(&level.as_str()) {
                    return Err(ExhaustionError::InvalidConfig(
                        format!("Effect 'stagger_susceptibility' level must be light, medium, or heavy, got: {}", level)
                    ).into());
                }
            }
            "regen_modifier" => {
                if effect.resource.is_none() || effect.modifier.is_none() {
                    return Err(ExhaustionError::InvalidConfig(
                        "Effect 'regen_modifier' requires resource and modifier".to_string()
                    ).into());
                }
            }
            _ => {
                return Err(ExhaustionError::InvalidConfig(
                    format!("Unknown effect type: {}", effect.effect_type)
                ).into());
            }
        }

        Ok(())
    }

    /// Evaluate exhaustion for an actor
    pub async fn evaluate(&self, actor: &Actor, snapshot: &Snapshot) -> ActorCoreResult<Vec<ExhaustionTransition>> {
        self.engine.evaluate(actor, snapshot).await
    }

    /// Apply exhaustion effects
    pub async fn apply_effects(&self, actor_id: &str, transitions: &[ExhaustionTransition]) -> ActorCoreResult<()> {
        let mut active_effects = self.active_effects.write().await;
        let actor_effects = active_effects.entry(actor_id.to_string()).or_insert_with(HashMap::new);

        for transition in transitions {
            if transition.entering {
                // Apply effects
                for effect in &transition.effects {
                    self.apply_effect(actor_id, &transition.resource, &transition.threshold_id, effect).await?;
                }

                // Track active threshold
                actor_effects.insert(
                    format!("{}:{}", transition.resource, transition.threshold_id),
                    ExhaustionState {
                        active_thresholds: {
                            let mut thresholds = HashMap::new();
                            thresholds.insert(
                                transition.threshold_id.clone(),
                                ThresholdState {
                                    threshold_id: transition.threshold_id.clone(),
                                    is_active: true,
                                    activated_at: Utc::now(),
                                    applied_effects: transition.effects.clone(),
                                }
                            );
                            thresholds
                        },
                        last_evaluation: Utc::now(),
                        coalescing_state: CoalescingState {
                            pending_events: Vec::new(),
                            last_event_time: Utc::now(),
                        },
                    }
                );
            } else {
                // Clear effects
                for effect in &transition.effects {
                    self.clear_effect(actor_id, &transition.resource, &transition.threshold_id, effect).await?;
                }

                // Remove from active thresholds
                actor_effects.remove(&format!("{}:{}", transition.resource, transition.threshold_id));
            }
        }

        Ok(())
    }

    /// Apply a single effect
    async fn apply_effect(
        &self,
        actor_id: &str,
        resource: &str,
        threshold_id: &str,
        effect: &EffectConfig,
    ) -> ActorCoreResult<()> {
        // Create idempotency key
        let idempotency_key = self.generate_idempotency_key(actor_id, resource, threshold_id, true);
        
        // Publish event
        let event = ExhaustionEvent {
            event_type: ExhaustionEventType::ResourceExhausted,
            actor_id: actor_id.to_string(),
            resource_type: resource.to_string(),
            threshold_id: threshold_id.to_string(),
            effects: vec![effect.clone()],
            timestamp: Utc::now(),
            idempotency_key,
            coalesced: false,
        };

        // Debug logging for case05 (disabled for performance)
        // println!("DEBUG: Published apply_effect event - type={:?}, resource_type={}, threshold_id={}", 
        //     event.event_type, event.resource_type, event.threshold_id);

        self.event_publisher.publish_event(event).await?;
        
        Ok(())
    }

    /// Clear a single effect
    async fn clear_effect(
        &self,
        actor_id: &str,
        resource: &str,
        threshold_id: &str,
        effect: &EffectConfig,
    ) -> ActorCoreResult<()> {
        // Create idempotency key
        let idempotency_key = self.generate_idempotency_key(actor_id, resource, threshold_id, false);
        
        // Publish event
        let event = ExhaustionEvent {
            event_type: ExhaustionEventType::ResourceRecovered,
            actor_id: actor_id.to_string(),
            resource_type: resource.to_string(),
            threshold_id: threshold_id.to_string(),
            effects: vec![effect.clone()],
            timestamp: Utc::now(),
            idempotency_key,
            coalesced: false,
        };

        // Debug logging for case05 (disabled for performance)
        // println!("DEBUG: Published clear_effect event - type={:?}, resource_type={}, threshold_id={}", 
        //     event.event_type, event.resource_type, event.threshold_id);

        self.event_publisher.publish_event(event).await?;
        
        Ok(())
    }

    /// Generate idempotency key
    fn generate_idempotency_key(&self, actor_id: &str, resource: &str, threshold_id: &str, entering: bool) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        actor_id.hash(&mut hasher);
        resource.hash(&mut hasher);
        threshold_id.hash(&mut hasher);
        entering.hash(&mut hasher);
        
        format!("exhaustion_{:x}", hasher.finish())
    }
}

impl ExhaustionEngine {
    /// Create a new exhaustion engine
    pub fn new(config: Arc<RwLock<ExhaustionConfig>>) -> Self {
        Self { 
            config,
            actor_states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Evaluate exhaustion for an actor
    pub async fn evaluate(&self, actor: &Actor, snapshot: &Snapshot) -> ActorCoreResult<Vec<ExhaustionTransition>> {
        let config = self.config.read().await;
        
        // Get actor archetype (simplified - in real implementation, this would come from actor data)
        let archetype = actor.data.get("archetype")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        // Get archetype config
        let archetype_config = config.archetypes.get(archetype)
            .ok_or_else(|| ExhaustionError::UnknownResource(format!("Unknown archetype: {}", archetype)))?;

        let mut transitions = Vec::new();
        let actor_id = actor.id.to_string();
        
        // Debug logging for case05 (disabled for performance)
        // println!("DEBUG: Actor ID: {}", actor_id);

        // Get current actor state
        let mut actor_states = self.actor_states.write().await;
        let current_states = actor_states.entry(actor_id.clone()).or_insert_with(HashMap::new);

        // Process each resource in deterministic order
        let mut resource_names: Vec<_> = archetype_config.resources.keys().collect();
        resource_names.sort();

        for resource_name in resource_names {
            let resource_config = &archetype_config.resources[resource_name];
            
            // Get current resource value
            let current_value = snapshot.primary.get(&format!("{}_current", resource_name))
                .copied()
                .unwrap_or(0.0);
            let max_value = snapshot.primary.get(&format!("{}_max", resource_name))
                .copied()
                .unwrap_or(1.0);
            
            // Calculate percentage
            let percentage = if max_value > 0.0 { current_value / max_value } else { 0.0 };

            // Sort thresholds by order
            let mut thresholds = resource_config.thresholds.clone();
            thresholds.sort_by_key(|t| t.order.unwrap_or(0));

            // Check each threshold
            for threshold in thresholds {
                let threshold_key = format!("{}:{}", resource_name, threshold.id);
                let was_active = current_states.get(&threshold_key).copied().unwrap_or(false);
                
                let should_enter = self.check_threshold_entering(&threshold, current_value, percentage)?;
                let should_exit = self.check_threshold_exiting(&threshold, current_value, percentage)?;
                
                // Debug logging for case05 (disabled for performance)
                // if resource_name == "mana" && threshold.id == "low_mana" {
                //     println!("DEBUG: {} - should_enter={}, should_exit={}, was_active={}, current_value={}, percentage={}, threshold_key={}", 
                //         threshold.id, should_enter, should_exit, was_active, current_value, percentage, threshold_key);
                // }
                
                // Determine if threshold should be active now
                // A threshold is active if it's entering OR if it was active and not exiting
                let should_be_active = should_enter || (was_active && !should_exit);
                
                // Always update the current state first
                current_states.insert(threshold_key, should_be_active);
                
                // Debug logging for case05 (disabled for performance)
                // if resource_name == "mana" && threshold.id == "low_mana" {
                //     println!("DEBUG: {} - should_be_active={}, was_active={}, creating_transition={}", 
                //         threshold.id, should_be_active, was_active, should_be_active != was_active);
                // }
                
                // Only create transitions for actual state changes
                if should_be_active && !was_active {
                    // Entering threshold
                    transitions.push(ExhaustionTransition {
                        resource: resource_name.clone(),
                        threshold_id: threshold.id.clone(),
                        entering: true,
                        effects: threshold.effects.clone(),
                    });
                } else if !should_be_active && was_active {
                    // Exiting threshold
                    transitions.push(ExhaustionTransition {
                        resource: resource_name.clone(),
                        threshold_id: threshold.id.clone(),
                        entering: false,
                        effects: threshold.effects.clone(),
                    });
                }
            }
        }

        // Debug logging for case05 (disabled for performance)
        // if !transitions.is_empty() {
        //     println!("DEBUG: Created {} transitions", transitions.len());
        //     for transition in &transitions {
        //         println!("DEBUG: Transition - resource={}, threshold_id={}, entering={}", 
        //             transition.resource, transition.threshold_id, transition.entering);
        //     }
        // }

        Ok(transitions)
    }

    /// Check if threshold is being entered
    fn check_threshold_entering(&self, threshold: &ThresholdConfig, current_value: f64, percentage: f64) -> ActorCoreResult<bool> {
        if let Some(enter_percent) = threshold.enter_percent_lte {
            return Ok(percentage <= enter_percent);
        }
        
        if let Some(enter_value) = threshold.enter_value_eq {
            return Ok((current_value - enter_value).abs() < f64::EPSILON);
        }

        Ok(false)
    }

    /// Check if threshold is being exited
    fn check_threshold_exiting(&self, threshold: &ThresholdConfig, current_value: f64, percentage: f64) -> ActorCoreResult<bool> {
        if let Some(exit_percent) = threshold.exit_percent_gte {
            return Ok(percentage >= exit_percent);
        }
        
        if let Some(exit_value) = threshold.exit_value_ge {
            return Ok(current_value >= exit_value);
        }

        Ok(false)
    }
}

#[async_trait]
impl Subsystem for ResourceExhaustionSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }

    fn priority(&self) -> i64 {
        self.priority
    }

    async fn contribute(&self, _actor: &Actor) -> ActorCoreResult<SubsystemOutput> {
        // Resource Exhaustion Subsystem doesn't contribute to stats directly
        // It only processes exhaustion effects based on current resource values
        // This would typically be called by the combat system or resource manager
        
        let output = SubsystemOutput::new(self.system_id.clone());
        
        // Add any context modifiers that might affect resource calculations
        // (e.g., regen modifiers from exhaustion effects)
        
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_exhaustion_config_validation() {
        let config = ExhaustionConfig {
            version: 1,
            hysteresis_default: 0.02,
            events: EventConfig {
                coalesce_window_ms: 200,
            },
            priorities: None,
            archetypes: HashMap::new(),
        };

        assert!(ResourceExhaustionSubsystem::validate_config(&config).is_ok());
    }

    #[tokio::test]
    async fn test_invalid_config_version() {
        let config = ExhaustionConfig {
            version: 0,
            hysteresis_default: 0.02,
            events: EventConfig {
                coalesce_window_ms: 200,
            },
            priorities: None,
            archetypes: HashMap::new(),
        };

        assert!(ResourceExhaustionSubsystem::validate_config(&config).is_err());
    }

    #[tokio::test]
    async fn test_invalid_hysteresis() {
        let config = ExhaustionConfig {
            version: 1,
            hysteresis_default: 1.5,
            events: EventConfig {
                coalesce_window_ms: 200,
            },
            priorities: None,
            archetypes: HashMap::new(),
        };

        assert!(ResourceExhaustionSubsystem::validate_config(&config).is_err());
    }
}
