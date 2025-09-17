# Status Core Implementation Guide

## 📋 **Tổng Quan**

Status Core Implementation Guide cung cấp step-by-step hướng dẫn implement Status Core system, bao gồm project setup, core components, integration, testing, và deployment.

## 🎯 **Implementation Phases**

### **Phase 1: Project Setup**
- Create Status Core crate
- Setup dependencies
- Configure build system
- Setup testing framework

### **Phase 2: Core Components**
- Implement Status Core Engine
- Implement Status Effect Manager
- Implement Immunity Manager
- Implement Status Calculator

### **Phase 3: Integration**
- Integrate with Element Core
- Integrate with Action Core
- Integrate with Combat Core
- Integrate with Actor Core

### **Phase 4: Testing & Deployment**
- Implement comprehensive tests
- Performance testing
- Integration testing
- Deployment configuration

## 🏗️ **Implementation Steps**

### **Step 1: Project Setup**

```toml
# Cargo.toml
[package]
name = "status-core"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
dashmap = "5.0"
arc-swap = "1.0"
tracing = "0.1"
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
tokio-test = "0.4"
mockall = "0.11"
criterion = "0.5"
```

### **Step 2: Core Types**

```rust
// src/types.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect_id: String,
    pub effect_name: String,
    pub effect_name_vi: String,
    pub category: StatusCategory,
    pub effect_type: StatusEffectType,
    pub magnitude: StatusMagnitude,
    pub duration: StatusDuration,
    pub target: StatusTarget,
    pub source: StatusSource,
    pub conditions: Vec<StatusCondition>,
    pub interactions: Vec<StatusEffectInteraction>,
    pub immunity_list: Vec<String>,
    pub movement_restrictions: Vec<MovementRestriction>,
    pub visual_effects: Vec<VisualEffect>,
    pub audio_effects: Vec<AudioEffect>,
    pub properties: HashMap<String, serde_json::Value>,
    pub priority: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusCategory {
    Debuff,
    Buff,
    Control,
    Transformation,
    Environmental,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEffectType {
    DamageOverTime,
    HealOverTime,
    StatModifier,
    MovementRestriction,
    Control,
    Immunity,
    Transformation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMagnitude {
    pub base_value: f64,
    pub scaling_factor: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub scaling_stat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDuration {
    pub base_duration: Duration,
    pub scaling_factor: f64,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub scaling_stat: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusTarget {
    Self,
    Ally,
    Enemy,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusSource {
    Player,
    NPC,
    Environment,
    Item,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCondition {
    pub condition_type: StatusConditionType,
    pub value: f64,
    pub operator: StatusConditionOperator,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusConditionType {
    HealthBelow,
    HealthAbove,
    ManaBelow,
    ManaAbove,
    LevelBelow,
    LevelAbove,
    HasEffect,
    HasImmunity,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusConditionOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectInteraction {
    pub target_effect_id: String,
    pub interaction_type: StatusInteractionType,
    pub probability: f64,
    pub magnitude_modifier: f64,
    pub duration_modifier: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusInteractionType {
    Replace,
    Stack,
    Extend,
    Reduce,
    Cancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementRestriction {
    pub restriction_type: MovementRestrictionType,
    pub magnitude: f64,
    pub duration: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MovementRestrictionType {
    CannotMove,
    ReducedSpeed,
    CannotJump,
    CannotTeleport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualEffect {
    pub effect_type: VisualEffectType,
    pub color: Color,
    pub intensity: f64,
    pub duration: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisualEffectType {
    Particle,
    Glow,
    Aura,
    Overlay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEffect {
    pub sound_id: String,
    pub volume: f64,
    pub pitch: f64,
    pub duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectInstance {
    pub effect_id: String,
    pub effect_name: String,
    pub effect_name_vi: String,
    pub category: StatusCategory,
    pub effect_type: StatusEffectType,
    pub magnitude: f64,
    pub duration: Duration,
    pub target: StatusTarget,
    pub source: StatusSource,
    pub conditions: Vec<StatusCondition>,
    pub interactions: Vec<StatusEffectInteraction>,
    pub immunity_list: Vec<String>,
    pub movement_restrictions: Vec<MovementRestriction>,
    pub visual_effects: Vec<VisualEffect>,
    pub audio_effects: Vec<AudioEffect>,
    pub properties: HashMap<String, serde_json::Value>,
    pub priority: u32,
    pub applied_at: SystemTime,
    pub expires_at: SystemTime,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectResult {
    pub effect_id: String,
    pub success: bool,
    pub reason: StatusEffectFailureReason,
    pub magnitude: f64,
    pub duration: Duration,
    pub applied_at: Option<SystemTime>,
    pub expires_at: Option<SystemTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEffectFailureReason {
    None,
    Validation,
    Immunity,
    Stacking,
    Condition,
    Resource,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusContext {
    pub actor_id: String,
    pub target_id: Option<String>,
    pub source_id: Option<String>,
    pub environment: String,
    pub time: SystemTime,
    pub additional_data: HashMap<String, serde_json::Value>,
}

impl StatusContext {
    pub fn new(actor_id: String) -> Self {
        Self {
            actor_id,
            target_id: None,
            source_id: None,
            environment: "default".to_string(),
            time: SystemTime::now(),
            additional_data: HashMap::new(),
        }
    }
    
    pub fn with_target(mut self, target_id: String) -> Self {
        self.target_id = Some(target_id);
        self
    }
    
    pub fn with_source(mut self, source_id: String) -> Self {
        self.source_id = Some(source_id);
        self
    }
    
    pub fn with_environment(mut self, environment: String) -> Self {
        self.environment = environment;
        self
    }
    
    pub fn add_data(mut self, key: String, value: serde_json::Value) -> Self {
        self.additional_data.insert(key, value);
        self
    }
}
```

### **Step 3: Error Handling**

```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StatusError {
    #[error("Validation error: {field} - {message}")]
    ValidationError { field: String, message: String },
    
    #[error("System error: {component} - {message}")]
    SystemError { component: String, message: String },
    
    #[error("Integration error: {system} - {message}")]
    IntegrationError { system: String, message: String },
    
    #[error("Performance error: {metric} - {message}")]
    PerformanceError { metric: String, message: String },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    #[error("Plugin error: {plugin_id} - {message}")]
    PluginError { plugin_id: String, message: String },
    
    #[error("Cache error: {message}")]
    CacheError { message: String },
    
    #[error("Memory error: {message}")]
    MemoryError { message: String },
    
    #[error("Network error: {message}")]
    NetworkError { message: String },
    
    #[error("Database error: {message}")]
    DatabaseError { message: String },
    
    #[error("Unknown error: {message}")]
    UnknownError { message: String },
}

impl StatusError {
    pub fn validation_error(field: &str, message: &str) -> Self {
        Self::ValidationError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
    
    pub fn system_error(component: &str, message: &str) -> Self {
        Self::SystemError {
            component: component.to_string(),
            message: message.to_string(),
        }
    }
    
    pub fn integration_error(system: &str, message: &str) -> Self {
        Self::IntegrationError {
            system: system.to_string(),
            message: message.to_string(),
        }
    }
    
    pub fn performance_error(metric: &str, message: &str) -> Self {
        Self::PerformanceError {
            metric: metric.to_string(),
            message: message.to_string(),
        }
    }
    
    pub fn configuration_error(message: &str) -> Self {
        Self::ConfigurationError {
            message: message.to_string(),
        }
    }
    
    pub fn plugin_error(plugin_id: &str, message: &str) -> Self {
        Self::PluginError {
            plugin_id: plugin_id.to_string(),
            message: message.to_string(),
        }
    }
    
    pub fn cache_error(message: &str) -> Self {
        Self::CacheError {
            message: message.to_string(),
        }
    }
    
    pub fn memory_error(message: &str) -> Self {
        Self::MemoryError {
            message: message.to_string(),
        }
    }
    
    pub fn network_error(message: &str) -> Self {
        Self::NetworkError {
            message: message.to_string(),
        }
    }
    
    pub fn database_error(message: &str) -> Self {
        Self::DatabaseError {
            message: message.to_string(),
        }
    }
    
    pub fn unknown_error(message: &str) -> Self {
        Self::UnknownError {
            message: message.to_string(),
        }
    }
}
```

### **Step 4: Core Engine**

```rust
// src/engine.rs
use crate::types::*;
use crate::error::StatusError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct StatusCoreEngine {
    status_effect_manager: Arc<StatusEffectManager>,
    immunity_manager: Arc<ImmunityManager>,
    status_calculator: Arc<StatusCalculator>,
    status_validator: Arc<StatusValidator>,
    status_processor: Arc<StatusProcessor>,
    plugin_registry: Arc<StatusPluginRegistry>,
    configuration_manager: Arc<StatusCoreConfigurationManager>,
    status_cache: Arc<StatusCache>,
    memory_pool: Arc<StatusMemoryPool>,
    event_dispatcher: Arc<StatusEventDispatcher>,
    config: StatusCoreConfig,
}

impl StatusCoreEngine {
    pub async fn new(config: StatusCoreConfig) -> Result<Self, StatusError> {
        let plugin_registry = Arc::new(StatusPluginRegistry::new());
        let configuration_manager = Arc::new(StatusCoreConfigurationManager::new(config.clone()).await?);
        
        let status_effect_manager = Arc::new(StatusEffectManager::new(
            plugin_registry.clone(),
            configuration_manager.clone()
        ).await?);
        
        let immunity_manager = Arc::new(ImmunityManager::new(
            plugin_registry.clone(),
            configuration_manager.clone()
        ).await?);
        
        let status_calculator = Arc::new(StatusCalculator::new(
            plugin_registry.clone(),
            configuration_manager.clone()
        ));
        
        let status_validator = Arc::new(StatusValidator::new(
            plugin_registry.clone(),
            configuration_manager.clone()
        ));
        
        let status_processor = Arc::new(StatusProcessor::new(
            plugin_registry.clone(),
            configuration_manager.clone()
        ));
        
        let status_cache = Arc::new(StatusCache::new(config.cache_config.clone()));
        let memory_pool = Arc::new(StatusMemoryPool::new(config.memory_config.clone()));
        let event_dispatcher = Arc::new(StatusEventDispatcher::new());
        
        Ok(Self {
            status_effect_manager,
            immunity_manager,
            status_calculator,
            status_validator,
            status_processor,
            plugin_registry,
            configuration_manager,
            status_cache,
            memory_pool,
            event_dispatcher,
            config,
        })
    }
    
    pub async fn apply_status_effect(
        &self,
        actor_id: &str,
        status_effect: StatusEffect,
        context: &StatusContext
    ) -> Result<StatusEffectResult, StatusError> {
        // Validate status effect
        self.status_validator.validate_status_effect(&status_effect, actor_id, context).await?;
        
        // Check immunity
        if self.immunity_manager.is_immune_to(actor_id, &status_effect.effect_id).await? {
            return Ok(StatusEffectResult {
                effect_id: status_effect.effect_id.clone(),
                success: false,
                reason: StatusEffectFailureReason::Immunity,
                magnitude: 0.0,
                duration: Duration::ZERO,
                applied_at: None,
                expires_at: None,
            });
        }
        
        // Calculate final magnitude and duration
        let final_magnitude = self.status_calculator.calculate_magnitude(
            &status_effect,
            actor_id,
            context
        ).await?;
        
        let final_duration = self.status_calculator.calculate_duration(
            &status_effect,
            actor_id,
            context
        ).await?;
        
        // Apply status effect
        let result = self.status_effect_manager.apply_status_effect(
            actor_id,
            status_effect,
            final_magnitude,
            final_duration,
            context
        ).await?;
        
        // Update cache
        self.status_cache.update_actor_status_effect(actor_id, &result).await?;
        
        // Dispatch events
        self.event_dispatcher.dispatch_status_effect_applied(actor_id, &result).await?;
        
        Ok(result)
    }
    
    pub async fn get_actor_status_effects(&self, actor_id: &str) -> Result<Vec<StatusEffectInstance>, StatusError> {
        self.status_effect_manager.get_actor_status_effects(actor_id).await
    }
    
    pub async fn remove_status_effect(
        &self,
        actor_id: &str,
        effect_id: &str
    ) -> Result<StatusRemovalResult, StatusError> {
        let result = self.status_effect_manager.remove_status_effect(actor_id, effect_id).await?;
        
        // Update cache
        self.status_cache.remove_actor_status_effect(actor_id, effect_id).await?;
        
        // Dispatch events
        self.event_dispatcher.dispatch_status_effect_removed(actor_id, effect_id).await?;
        
        Ok(result)
    }
    
    pub async fn process_actor_status_effects(
        &self,
        actor_id: &str,
        context: &StatusContext
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        let active_effects = self.status_effect_manager.get_actor_status_effects(actor_id).await?;
        
        let mut results = Vec::new();
        for effect in active_effects {
            let result = self.status_processor.process_status_effect(
                &effect,
                actor_id,
                context
            ).await?;
            results.push(result);
        }
        
        // Update cache
        self.status_cache.update_actor_status_effects(actor_id, &results).await?;
        
        // Dispatch events
        self.event_dispatcher.dispatch_status_effects_processed(actor_id, &results).await?;
        
        Ok(results)
    }
}

#[derive(Debug, Clone)]
pub struct StatusCoreConfig {
    pub cache_config: CacheConfig,
    pub memory_config: MemoryConfig,
    pub processing_config: ProcessingConfig,
    pub plugin_config: PluginConfig,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub l1_cache_size: usize,
    pub l1_cache_ttl: Duration,
    pub l2_cache_enabled: bool,
    pub l2_cache_ttl: Duration,
    pub l2_cache_host: String,
    pub l2_cache_port: u16,
}

#[derive(Debug, Clone)]
pub struct MemoryConfig {
    pub effect_pool_size: usize,
    pub immunity_pool_size: usize,
    pub result_pool_size: usize,
    pub context_pool_size: usize,
    pub initial_size: usize,
    pub max_size: usize,
    pub grow_size: usize,
}

#[derive(Debug, Clone)]
pub struct ProcessingConfig {
    pub core_threads: usize,
    pub max_threads: usize,
    pub queue_capacity: usize,
    pub enable_work_stealing: bool,
    pub enable_priority_queues: bool,
}

#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub plugin_directory: String,
    pub hot_reload: bool,
    pub plugin_timeout: Duration,
}
```

### **Step 5: Status Effect Manager**

```rust
// src/status_effect_manager.rs
use crate::types::*;
use crate::error::StatusError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct StatusEffectManager {
    active_effects: Arc<RwLock<HashMap<String, Vec<StatusEffectInstance>>>>,
    effect_definitions: Arc<RwLock<HashMap<String, StatusEffectDefinition>>>,
    plugin_registry: Arc<StatusPluginRegistry>,
    configuration_manager: Arc<StatusCoreConfigurationManager>,
    config: StatusEffectManagerConfig,
}

impl StatusEffectManager {
    pub async fn new(
        plugin_registry: Arc<StatusPluginRegistry>,
        configuration_manager: Arc<StatusCoreConfigurationManager>
    ) -> Result<Self, StatusError> {
        Ok(Self {
            active_effects: Arc::new(RwLock::new(HashMap::new())),
            effect_definitions: Arc::new(RwLock::new(HashMap::new())),
            plugin_registry,
            configuration_manager,
            config: StatusEffectManagerConfig::default(),
        })
    }
    
    pub async fn apply_status_effect(
        &self,
        actor_id: &str,
        status_effect: StatusEffect,
        magnitude: f64,
        duration: Duration,
        context: &StatusContext
    ) -> Result<StatusEffectResult, StatusError> {
        // Get effect definition
        let effect_definition = self.get_effect_definition(&status_effect.effect_id).await?;
        
        // Check stacking rules
        let stacking_result = self.check_stacking_rules(actor_id, &status_effect, &effect_definition).await?;
        
        // Create effect instance
        let effect_instance = StatusEffectInstance {
            effect_id: status_effect.effect_id.clone(),
            effect_name: status_effect.effect_name.clone(),
            effect_name_vi: status_effect.effect_name_vi.clone(),
            category: status_effect.category.clone(),
            effect_type: status_effect.effect_type.clone(),
            magnitude,
            duration,
            target: status_effect.target.clone(),
            source: status_effect.source.clone(),
            conditions: status_effect.conditions.clone(),
            interactions: status_effect.interactions.clone(),
            immunity_list: status_effect.immunity_list.clone(),
            movement_restrictions: status_effect.movement_restrictions.clone(),
            visual_effects: status_effect.visual_effects.clone(),
            audio_effects: status_effect.audio_effects.clone(),
            properties: status_effect.properties.clone(),
            priority: status_effect.priority,
            applied_at: SystemTime::now(),
            expires_at: SystemTime::now() + duration,
            is_active: true,
        };
        
        // Apply stacking rules
        match stacking_result.action {
            StackingAction::Replace => {
                self.replace_effect(actor_id, &effect_instance).await?;
            },
            StackingAction::Stack => {
                self.stack_effect(actor_id, &effect_instance).await?;
            },
            StackingAction::Ignore => {
                return Ok(StatusEffectResult {
                    effect_id: status_effect.effect_id.clone(),
                    success: false,
                    reason: StatusEffectFailureReason::Stacking,
                    magnitude: 0.0,
                    duration: Duration::ZERO,
                    applied_at: None,
                    expires_at: None,
                });
            },
        }
        
        // Store effect instance
        let mut active_effects = self.active_effects.write().await;
        active_effects.entry(actor_id.to_string())
            .or_insert_with(Vec::new)
            .push(effect_instance.clone());
        
        Ok(StatusEffectResult {
            effect_id: status_effect.effect_id.clone(),
            success: true,
            reason: StatusEffectFailureReason::None,
            magnitude,
            duration,
            applied_at: Some(effect_instance.applied_at),
            expires_at: Some(effect_instance.expires_at),
        })
    }
    
    pub async fn get_actor_status_effects(&self, actor_id: &str) -> Result<Vec<StatusEffectInstance>, StatusError> {
        let active_effects = self.active_effects.read().await;
        Ok(active_effects.get(actor_id).cloned().unwrap_or_default())
    }
    
    pub async fn remove_status_effect(
        &self,
        actor_id: &str,
        effect_id: &str
    ) -> Result<StatusRemovalResult, StatusError> {
        let mut active_effects = self.active_effects.write().await;
        
        if let Some(effects) = active_effects.get_mut(actor_id) {
            if let Some(index) = effects.iter().position(|e| e.effect_id == effect_id) {
                effects.remove(index);
                
                Ok(StatusRemovalResult {
                    effect_id: effect_id.to_string(),
                    success: true,
                    removed_at: SystemTime::now(),
                    reason: StatusRemovalReason::Manual,
                })
            } else {
                Ok(StatusRemovalResult {
                    effect_id: effect_id.to_string(),
                    success: false,
                    removed_at: SystemTime::now(),
                    reason: StatusRemovalReason::NotFound,
                })
            }
        } else {
            Ok(StatusRemovalResult {
                effect_id: effect_id.to_string(),
                success: false,
                removed_at: SystemTime::now(),
                reason: StatusRemovalReason::NotFound,
            })
        }
    }
    
    async fn get_effect_definition(&self, effect_id: &str) -> Result<StatusEffectDefinition, StatusError> {
        let effect_definitions = self.effect_definitions.read().await;
        effect_definitions.get(effect_id)
            .cloned()
            .ok_or_else(|| StatusError::system_error(
                "StatusEffectManager",
                "get_effect_definition",
                &format!("Effect definition not found: {}", effect_id)
            ))
    }
    
    async fn check_stacking_rules(
        &self,
        actor_id: &str,
        new_effect: &StatusEffect,
        effect_definition: &StatusEffectDefinition
    ) -> Result<StackingResult, StatusError> {
        let existing_effects = self.get_actor_status_effects(actor_id).await?;
        
        // Find conflicting effects
        let conflicting_effects: Vec<&StatusEffectInstance> = existing_effects
            .iter()
            .filter(|effect| self.is_conflicting_effect(effect, new_effect))
            .collect();
        
        if conflicting_effects.is_empty() {
            return Ok(StackingResult {
                action: StackingAction::Stack,
                conflicting_effects: Vec::new(),
                reason: "No conflicts".to_string(),
            });
        }
        
        // Determine stacking action based on priority
        let new_priority = new_effect.priority;
        let max_existing_priority = conflicting_effects
            .iter()
            .map(|e| e.priority)
            .max()
            .unwrap_or(0);
        
        if new_priority > max_existing_priority {
            Ok(StackingResult {
                action: StackingAction::Replace,
                conflicting_effects: conflicting_effects.into_iter().cloned().collect(),
                reason: "Higher priority".to_string(),
            })
        } else if new_priority == max_existing_priority {
            // Check if effect allows stacking
            if effect_definition.properties.get("stackable").and_then(|v| v.as_bool()).unwrap_or(false) {
                Ok(StackingResult {
                    action: StackingAction::Stack,
                    conflicting_effects: Vec::new(),
                    reason: "Stackable effect".to_string(),
                })
            } else {
                Ok(StackingResult {
                    action: StackingAction::Ignore,
                    conflicting_effects: Vec::new(),
                    reason: "Non-stackable effect".to_string(),
                })
            }
        } else {
            Ok(StackingResult {
                action: StackingAction::Ignore,
                conflicting_effects: Vec::new(),
                reason: "Lower priority".to_string(),
            })
        }
    }
    
    fn is_conflicting_effect(&self, existing: &StatusEffectInstance, new: &StatusEffect) -> bool {
        // Check if effects have the same ID
        if existing.effect_id == new.effect_id {
            return true;
        }
        
        // Check if effects are in the same category and conflict
        if existing.category == new.category {
            match existing.category {
                StatusCategory::Control => true,
                StatusCategory::Transformation => true,
                _ => false,
            }
        } else {
            false
        }
    }
    
    async fn replace_effect(&self, actor_id: &str, new_effect: &StatusEffectInstance) -> Result<(), StatusError> {
        let mut active_effects = self.active_effects.write().await;
        if let Some(effects) = active_effects.get_mut(actor_id) {
            // Remove conflicting effects
            effects.retain(|effect| effect.effect_id != new_effect.effect_id);
        }
        Ok(())
    }
    
    async fn stack_effect(&self, actor_id: &str, new_effect: &StatusEffectInstance) -> Result<(), StatusError> {
        // Stacking is handled in apply_status_effect
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StatusEffectDefinition {
    pub id: String,
    pub name: String,
    pub name_vi: String,
    pub category: StatusCategory,
    pub effect_type: StatusEffectType,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct StackingResult {
    pub action: StackingAction,
    pub conflicting_effects: Vec<StatusEffectInstance>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackingAction {
    Replace,
    Stack,
    Ignore,
}

#[derive(Debug, Clone)]
pub struct StatusRemovalResult {
    pub effect_id: String,
    pub success: bool,
    pub removed_at: SystemTime,
    pub reason: StatusRemovalReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusRemovalReason {
    Manual,
    Expired,
    Replaced,
    NotFound,
}

#[derive(Debug, Clone)]
pub struct StatusEffectManagerConfig {
    pub max_effects_per_actor: usize,
    pub effect_cleanup_interval: Duration,
    pub enable_stacking: bool,
    pub enable_priority_system: bool,
}

impl Default for StatusEffectManagerConfig {
    fn default() -> Self {
        Self {
            max_effects_per_actor: 100,
            effect_cleanup_interval: Duration::from_secs(60),
            enable_stacking: true,
            enable_priority_system: true,
        }
    }
}
```

### **Step 6: Testing**

```rust
// tests/integration_tests.rs
use status_core::*;
use tokio_test;

#[tokio::test]
async fn test_status_effect_application() {
    let config = StatusCoreConfig {
        cache_config: CacheConfig::default(),
        memory_config: MemoryConfig::default(),
        processing_config: ProcessingConfig::default(),
        plugin_config: PluginConfig::default(),
    };
    
    let engine = StatusCoreEngine::new(config).await.unwrap();
    let actor_id = "test_actor";
    let status_effect = create_test_status_effect();
    let context = StatusContext::new(actor_id.to_string());
    
    let result = engine.apply_status_effect(actor_id, status_effect, &context).await.unwrap();
    
    assert!(result.success);
    assert_eq!(result.effect_id, "test_effect");
}

#[tokio::test]
async fn test_status_effect_processing() {
    let config = StatusCoreConfig::default();
    let engine = StatusCoreEngine::new(config).await.unwrap();
    let actor_id = "test_actor";
    let context = StatusContext::new(actor_id.to_string());
    
    // Apply multiple status effects
    let effects = vec![
        create_test_status_effect_with_id("effect1"),
        create_test_status_effect_with_id("effect2"),
        create_test_status_effect_with_id("effect3"),
    ];
    
    for effect in effects {
        engine.apply_status_effect(actor_id, effect, &context).await.unwrap();
    }
    
    // Process status effects
    let results = engine.process_actor_status_effects(actor_id, &context).await.unwrap();
    
    assert_eq!(results.len(), 3);
    for result in results {
        assert!(result.success);
    }
}

fn create_test_status_effect() -> StatusEffect {
    StatusEffect {
        effect_id: "test_effect".to_string(),
        effect_name: "Test Effect".to_string(),
        effect_name_vi: "Hiệu Ứng Test".to_string(),
        category: StatusCategory::Debuff,
        effect_type: StatusEffectType::DamageOverTime,
        magnitude: StatusMagnitude {
            base_value: 10.0,
            scaling_factor: 1.0,
            min_value: 1.0,
            max_value: 100.0,
            scaling_stat: "intelligence".to_string(),
        },
        duration: StatusDuration {
            base_duration: Duration::from_secs(10),
            scaling_factor: 1.0,
            min_duration: Duration::from_secs(1),
            max_duration: Duration::from_secs(60),
            scaling_stat: "wisdom".to_string(),
        },
        target: StatusTarget::Enemy,
        source: StatusSource::Player,
        conditions: Vec::new(),
        interactions: Vec::new(),
        immunity_list: Vec::new(),
        movement_restrictions: Vec::new(),
        visual_effects: Vec::new(),
        audio_effects: Vec::new(),
        properties: HashMap::new(),
        priority: 1,
    }
}

fn create_test_status_effect_with_id(effect_id: &str) -> StatusEffect {
    let mut effect = create_test_status_effect();
    effect.effect_id = effect_id.to_string();
    effect
}
```

## 📝 **Implementation Notes**

### **1. Project Structure**
```
status-core/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── types.rs
│   ├── error.rs
│   ├── engine.rs
│   ├── status_effect_manager.rs
│   ├── immunity_manager.rs
│   ├── status_calculator.rs
│   ├── status_validator.rs
│   ├── status_processor.rs
│   ├── plugin_registry.rs
│   ├── configuration_manager.rs
│   ├── status_cache.rs
│   ├── memory_pool.rs
│   ├── event_dispatcher.rs
│   └── utils.rs
├── tests/
│   ├── integration_tests.rs
│   └── unit_tests.rs
├── examples/
│   └── basic_usage.rs
└── README.md
```

### **2. Dependencies**
- **tokio**: Async runtime
- **serde**: Serialization
- **uuid**: Unique identifiers
- **dashmap**: Concurrent HashMap
- **arc-swap**: Atomic reference counting
- **tracing**: Logging
- **anyhow**: Error handling
- **thiserror**: Error types

### **3. Testing Strategy**
- **Unit Tests**: Test individual components
- **Integration Tests**: Test component interactions
- **Performance Tests**: Test performance characteristics
- **End-to-End Tests**: Test complete workflows

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
