# Status Core Core System Design

## 📋 **Tổng Quan**

Status Core Core System là trung tâm xử lý tất cả status effects, buffs, debuffs, và immunity effects trong game. Hệ thống này cung cấp core engine, status effect manager, immunity manager, và các thành phần cốt lõi khác.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Core Engine Architecture**
- **Single Responsibility**: Mỗi component có trách nhiệm rõ ràng
- **High Performance**: Tối ưu cho high-load scenarios
- **Thread Safety**: Thread-safe operations cho concurrency
- **Memory Efficient**: Efficient memory usage và management

### **2. Status Effect Processing**
- **Real-time Processing**: Process status effects trong real-time
- **Batch Processing**: Process multiple effects efficiently
- **Priority System**: Priority-based effect resolution
- **Stacking Rules**: Complex stacking rules cho effects

### **3. Immunity System**
- **Comprehensive Immunity**: Support multiple immunity types
- **Immunity Stacking**: Stack immunity effects
- **Immunity Break**: Break immunity conditions
- **Immunity Override**: Override immunity rules

## 🏗️ **Core System Architecture**

### **1. Status Core Engine**

```rust
/// Status Core Engine - Main orchestrator
pub struct StatusCoreEngine {
    // Core components
    status_effect_manager: StatusEffectManager,
    immunity_manager: ImmunityManager,
    status_calculator: StatusCalculator,
    status_validator: StatusValidator,
    status_processor: StatusProcessor,
    
    // Integration components
    plugin_registry: Arc<StatusPluginRegistry>,
    configuration_manager: Arc<StatusCoreConfigurationManager>,
    
    // Performance optimization
    status_cache: StatusCache,
    batch_processor: StatusBatchProcessor,
    memory_pool: StatusMemoryPool,
    
    // Event system
    event_dispatcher: StatusEventDispatcher,
    event_subscribers: HashMap<String, Vec<Box<dyn StatusEventSubscriber>>>,
    
    // Configuration
    config: StatusCoreConfig,
}

impl StatusCoreEngine {
    /// Initialize Status Core Engine
    pub async fn new(config: StatusCoreConfig) -> Result<Self, StatusError> {
        let plugin_registry = Arc::new(StatusPluginRegistry::new());
        let configuration_manager = Arc::new(StatusCoreConfigurationManager::new());
        
        let status_effect_manager = StatusEffectManager::new(
            plugin_registry.clone(),
            configuration_manager.clone()
        ).await?;
        
        let immunity_manager = ImmunityManager::new(
            plugin_registry.clone(),
            configuration_manager.clone()
        ).await?;
        
        let status_calculator = StatusCalculator::new(
            plugin_registry.clone(),
            configuration_manager.clone()
        );
        
        let status_validator = StatusValidator::new(
            plugin_registry.clone(),
            configuration_manager.clone()
        );
        
        let status_processor = StatusProcessor::new(
            plugin_registry.clone(),
            configuration_manager.clone()
        );
        
        let status_cache = StatusCache::new(config.cache_config.clone());
        let batch_processor = StatusBatchProcessor::new(
            plugin_registry.clone(),
            status_cache.clone()
        );
        
        let memory_pool = StatusMemoryPool::new(config.memory_config.clone());
        
        let event_dispatcher = StatusEventDispatcher::new();
        
        Ok(Self {
            status_effect_manager,
            immunity_manager,
            status_calculator,
            status_validator,
            status_processor,
            plugin_registry,
            configuration_manager,
            status_cache,
            batch_processor,
            memory_pool,
            event_dispatcher,
            event_subscribers: HashMap::new(),
            config,
        })
    }
    
    /// Process status effects for an actor
    pub async fn process_actor_status_effects(
        &mut self,
        actor_id: &str,
        context: &StatusContext
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        // Get active status effects
        let active_effects = self.status_effect_manager.get_actor_status_effects(actor_id).await?;
        
        // Process each effect
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
    
    /// Apply status effect to actor
    pub async fn apply_status_effect(
        &mut self,
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
    
    /// Remove status effect from actor
    pub async fn remove_status_effect(
        &mut self,
        actor_id: &str,
        effect_id: &str
    ) -> Result<StatusRemovalResult, StatusError> {
        // Remove status effect
        let result = self.status_effect_manager.remove_status_effect(actor_id, effect_id).await?;
        
        // Update cache
        self.status_cache.remove_actor_status_effect(actor_id, effect_id).await?;
        
        // Dispatch events
        self.event_dispatcher.dispatch_status_effect_removed(actor_id, effect_id).await?;
        
        Ok(result)
    }
}
```

### **2. Status Effect Manager**

```rust
/// Status Effect Manager - Manages status effects
pub struct StatusEffectManager {
    // Core data
    active_effects: HashMap<String, Vec<StatusEffectInstance>>,
    effect_definitions: HashMap<String, StatusEffectDefinition>,
    
    // Dependencies
    plugin_registry: Arc<StatusPluginRegistry>,
    configuration_manager: Arc<StatusCoreConfigurationManager>,
    
    // Performance optimization
    effect_cache: EffectCache,
    batch_processor: EffectBatchProcessor,
    
    // Configuration
    config: StatusEffectManagerConfig,
}

impl StatusEffectManager {
    /// Apply status effect to actor
    pub async fn apply_status_effect(
        &mut self,
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
        self.active_effects.entry(actor_id.to_string())
            .or_insert_with(Vec::new)
            .push(effect_instance.clone());
        
        // Update cache
        self.effect_cache.update_actor_effects(actor_id, &self.active_effects[actor_id]).await?;
        
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
    
    /// Get actor status effects
    pub async fn get_actor_status_effects(&self, actor_id: &str) -> Result<Vec<StatusEffectInstance>, StatusError> {
        if let Some(effects) = self.active_effects.get(actor_id) {
            Ok(effects.clone())
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Remove status effect from actor
    pub async fn remove_status_effect(
        &mut self,
        actor_id: &str,
        effect_id: &str
    ) -> Result<StatusRemovalResult, StatusError> {
        if let Some(effects) = self.active_effects.get_mut(actor_id) {
            if let Some(index) = effects.iter().position(|e| e.effect_id == effect_id) {
                let removed_effect = effects.remove(index);
                
                // Update cache
                self.effect_cache.remove_actor_effect(actor_id, effect_id).await?;
                
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
    
    /// Check stacking rules
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
}

/// Status Effect Instance
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

/// Stacking Result
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
```

### **3. Immunity Manager**

```rust
/// Immunity Manager - Manages immunity effects
pub struct ImmunityManager {
    // Core data
    active_immunities: HashMap<String, Vec<ImmunityInstance>>,
    immunity_definitions: HashMap<String, ImmunityDefinition>,
    
    // Dependencies
    plugin_registry: Arc<StatusPluginRegistry>,
    configuration_manager: Arc<StatusCoreConfigurationManager>,
    
    // Performance optimization
    immunity_cache: ImmunityCache,
    batch_processor: ImmunityBatchProcessor,
    
    // Configuration
    config: ImmunityManagerConfig,
}

impl ImmunityManager {
    /// Apply immunity to actor
    pub async fn apply_immunity(
        &mut self,
        actor_id: &str,
        immunity: Immunity,
        context: &StatusContext
    ) -> Result<ImmunityResult, StatusError> {
        // Get immunity definition
        let immunity_definition = self.get_immunity_definition(&immunity.immunity_id).await?;
        
        // Calculate final magnitude and duration
        let final_magnitude = self.calculate_immunity_magnitude(&immunity, actor_id, context).await?;
        let final_duration = self.calculate_immunity_duration(&immunity, actor_id, context).await?;
        
        // Create immunity instance
        let immunity_instance = ImmunityInstance {
            immunity_id: immunity.immunity_id.clone(),
            immunity_name: immunity.immunity_name.clone(),
            immunity_name_vi: immunity.immunity_name_vi.clone(),
            immunity_type: immunity.immunity_type.clone(),
            target_effects: immunity.target_effects.clone(),
            magnitude: final_magnitude,
            duration: final_duration,
            break_conditions: immunity.break_conditions.clone(),
            source: immunity.source.clone(),
            applied_at: SystemTime::now(),
            expires_at: SystemTime::now() + final_duration,
            is_active: true,
        };
        
        // Store immunity instance
        self.active_immunities.entry(actor_id.to_string())
            .or_insert_with(Vec::new)
            .push(immunity_instance.clone());
        
        // Update cache
        self.immunity_cache.update_actor_immunities(actor_id, &self.active_immunities[actor_id]).await?;
        
        Ok(ImmunityResult {
            immunity_id: immunity.immunity_id.clone(),
            success: true,
            magnitude: final_magnitude,
            duration: final_duration,
            applied_at: immunity_instance.applied_at,
            expires_at: immunity_instance.expires_at,
        })
    }
    
    /// Check if actor is immune to effect
    pub async fn is_immune_to(&self, actor_id: &str, effect_id: &str) -> Result<bool, StatusError> {
        if let Some(immunities) = self.active_immunities.get(actor_id) {
            for immunity in immunities {
                if immunity.is_active && immunity.target_effects.contains(&effect_id.to_string()) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
    
    /// Check immunity break conditions
    pub async fn check_immunity_break_conditions(
        &self,
        actor_id: &str,
        immunity_id: &str,
        context: &StatusContext
    ) -> Result<bool, StatusError> {
        if let Some(immunities) = self.active_immunities.get(actor_id) {
            if let Some(immunity) = immunities.iter().find(|i| i.immunity_id == immunity_id) {
                for condition in &immunity.break_conditions {
                    if self.evaluate_break_condition(condition, actor_id, context).await? {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }
    
    /// Break immunity
    pub async fn break_immunity(
        &mut self,
        actor_id: &str,
        immunity_id: &str
    ) -> Result<ImmunityBreakResult, StatusError> {
        if let Some(immunities) = self.active_immunities.get_mut(actor_id) {
            if let Some(immunity) = immunities.iter_mut().find(|i| i.immunity_id == immunity_id) {
                immunity.is_active = false;
                immunity.expires_at = SystemTime::now();
                
                // Update cache
                self.immunity_cache.update_actor_immunities(actor_id, &self.active_immunities[actor_id]).await?;
                
                Ok(ImmunityBreakResult {
                    immunity_id: immunity_id.to_string(),
                    success: true,
                    broken_at: SystemTime::now(),
                    reason: ImmunityBreakReason::Condition,
                })
            } else {
                Ok(ImmunityBreakResult {
                    immunity_id: immunity_id.to_string(),
                    success: false,
                    broken_at: SystemTime::now(),
                    reason: ImmunityBreakReason::NotFound,
                })
            }
        } else {
            Ok(ImmunityBreakResult {
                immunity_id: immunity_id.to_string(),
                success: false,
                broken_at: SystemTime::now(),
                reason: ImmunityBreakReason::NotFound,
            })
        }
    }
}

/// Immunity Instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunityInstance {
    pub immunity_id: String,
    pub immunity_name: String,
    pub immunity_name_vi: String,
    pub immunity_type: ImmunityType,
    pub target_effects: Vec<String>,
    pub magnitude: f64,
    pub duration: Duration,
    pub break_conditions: Vec<ImmunityBreakCondition>,
    pub source: String,
    pub applied_at: SystemTime,
    pub expires_at: SystemTime,
    pub is_active: bool,
}
```

### **4. Status Calculator**

```rust
/// Status Calculator - Calculates status effect values
pub struct StatusCalculator {
    // Dependencies
    plugin_registry: Arc<StatusPluginRegistry>,
    configuration_manager: Arc<StatusCoreConfigurationManager>,
    
    // Performance optimization
    calculation_cache: CalculationCache,
    
    // Configuration
    config: StatusCalculatorConfig,
}

impl StatusCalculator {
    /// Calculate status effect magnitude
    pub async fn calculate_magnitude(
        &self,
        status_effect: &StatusEffect,
        actor_id: &str,
        context: &StatusContext
    ) -> Result<f64, StatusError> {
        // Check cache first
        if let Some(cached_magnitude) = self.calculation_cache.get_magnitude(
            &status_effect.effect_id,
            actor_id
        ).await? {
            return Ok(cached_magnitude);
        }
        
        // Calculate base magnitude
        let base_magnitude = status_effect.magnitude.base_value;
        
        // Get scaling stat value
        let scaling_value = self.get_scaling_stat_value(
            &status_effect.magnitude.scaling_stat,
            actor_id,
            context
        ).await?;
        
        // Apply scaling factor
        let scaled_magnitude = base_magnitude + (scaling_value * status_effect.magnitude.scaling_factor);
        
        // Apply min/max constraints
        let final_magnitude = scaled_magnitude
            .max(status_effect.magnitude.min_value)
            .min(status_effect.magnitude.max_value);
        
        // Cache result
        self.calculation_cache.cache_magnitude(
            &status_effect.effect_id,
            actor_id,
            final_magnitude
        ).await?;
        
        Ok(final_magnitude)
    }
    
    /// Calculate status effect duration
    pub async fn calculate_duration(
        &self,
        status_effect: &StatusEffect,
        actor_id: &str,
        context: &StatusContext
    ) -> Result<Duration, StatusError> {
        // Check cache first
        if let Some(cached_duration) = self.calculation_cache.get_duration(
            &status_effect.effect_id,
            actor_id
        ).await? {
            return Ok(cached_duration);
        }
        
        // Calculate base duration
        let base_duration = status_effect.duration.base_duration;
        
        // Get scaling stat value
        let scaling_value = self.get_scaling_stat_value(
            &status_effect.duration.scaling_stat,
            actor_id,
            context
        ).await?;
        
        // Apply scaling factor
        let scaled_duration = base_duration + Duration::from_secs_f64(
            scaling_value * status_effect.duration.scaling_factor
        );
        
        // Apply min/max constraints
        let final_duration = scaled_duration
            .max(status_effect.duration.min_duration)
            .min(status_effect.duration.max_duration);
        
        // Cache result
        self.calculation_cache.cache_duration(
            &status_effect.effect_id,
            actor_id,
            final_duration
        ).await?;
        
        Ok(final_duration)
    }
    
    /// Get scaling stat value
    async fn get_scaling_stat_value(
        &self,
        stat_name: &str,
        actor_id: &str,
        context: &StatusContext
    ) -> Result<f64, StatusError> {
        // Try to get from context first
        if let Some(value) = context.get_stat_value(stat_name) {
            return Ok(value);
        }
        
        // Try to get from plugin registry
        if let Some(plugin) = self.plugin_registry.get_plugin_for_stat(stat_name) {
            return plugin.get_stat_value(actor_id, stat_name).await;
        }
        
        // Default to 0
        Ok(0.0)
    }
}
```

### **5. Status Processor**

```rust
/// Status Processor - Processes status effects
pub struct StatusProcessor {
    // Dependencies
    plugin_registry: Arc<StatusPluginRegistry>,
    configuration_manager: Arc<StatusCoreConfigurationManager>,
    
    // Performance optimization
    processing_cache: ProcessingCache,
    batch_processor: StatusBatchProcessor,
    
    // Configuration
    config: StatusProcessorConfig,
}

impl StatusProcessor {
    /// Process status effect
    pub async fn process_status_effect(
        &self,
        effect: &StatusEffectInstance,
        actor_id: &str,
        context: &StatusContext
    ) -> Result<StatusEffectProcessingResult, StatusError> {
        // Get plugin for this effect
        let plugin = self.plugin_registry.get_plugin_for_effect(&effect.effect_id).await?;
        
        // Process effect through plugin (only status logic, no damage calculation)
        let processing_result = plugin.process_status_effect(effect, actor_id, context).await?;
        
        // Create damage request for CombatCore if effect deals damage
        let damage_request = if processing_result.requires_damage_calculation {
            Some(StatusDamageRequest {
                effect_id: effect.effect_id.clone(),
                effect_type: effect.effect_type.clone(),
                base_magnitude: effect.magnitude,
                element_id: self.extract_element_id(effect),
                actor_id: actor_id.to_string(),
                damage_type: self.map_to_damage_type(&effect.effect_type),
                properties: effect.properties.clone(),
            })
        } else {
            None
        };
        
        // Update processing cache
        self.processing_cache.update_effect_processing(
            &effect.effect_id,
            actor_id,
            &processing_result
        ).await?;
        
        Ok(StatusEffectProcessingResult {
            effect_id: effect.effect_id.clone(),
            success: processing_result.success,
            requires_damage_calculation: processing_result.requires_damage_calculation,
            damage_request,
            status_changes: processing_result.status_changes,
            visual_effects: processing_result.visual_effects,
            audio_effects: processing_result.audio_effects,
        })
    }
    
    /// Process multiple status effects in batch
    pub async fn process_status_effects_batch(
        &self,
        effects: Vec<StatusEffectInstance>,
        actor_id: &str,
        context: &StatusContext
    ) -> Result<Vec<StatusEffectProcessingResult>, StatusError> {
        let mut results = Vec::new();
        
        // Group effects by plugin for efficient processing
        let mut grouped_effects: HashMap<String, Vec<StatusEffectInstance>> = HashMap::new();
        for effect in effects {
            if let Some(plugin_id) = self.get_effect_plugin_id(&effect.effect_id).await? {
                grouped_effects.entry(plugin_id).or_insert_with(Vec::new).push(effect);
            }
        }
        
        // Process each plugin's effects
        for (plugin_id, plugin_effects) in grouped_effects {
            if let Some(plugin) = self.plugin_registry.get_plugin(&plugin_id) {
                let plugin_results = self.process_plugin_effects_batch(
                    plugin,
                    plugin_effects,
                    actor_id,
                    context
                ).await?;
                results.extend(plugin_results);
            }
        }
        
        Ok(results)
    }
    
    /// Extract element ID from status effect
    fn extract_element_id(&self, effect: &StatusEffectInstance) -> String {
        // Extract element ID from effect properties or effect ID
        effect.properties
            .get("element_id")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                // Fallback: extract from effect_id (e.g., "fire_burning" -> "fire")
                effect.effect_id.split('_').next().unwrap_or("unknown").to_string()
            })
            .to_string()
    }
    
    /// Map status effect type to damage type
    fn map_to_damage_type(&self, effect_type: &StatusEffectType) -> StatusDamageType {
        match effect_type {
            StatusEffectType::DamageOverTime => StatusDamageType::DamageOverTime,
            StatusEffectType::HealOverTime => StatusDamageType::HealOverTime,
            StatusEffectType::StatModifier => StatusDamageType::StatModifier,
            StatusEffectType::MovementRestriction => StatusDamageType::MovementRestriction,
            StatusEffectType::Control => StatusDamageType::Control,
            StatusEffectType::Immunity => StatusDamageType::Immunity,
            StatusEffectType::Transformation => StatusDamageType::Transformation,
        }
    }
}
```

## 🚀 **Performance Optimization**

### **1. Status Cache**

```rust
/// Status Cache for performance optimization
pub struct StatusCache {
    // Effect cache
    effect_cache: HashMap<String, HashMap<String, StatusEffectInstance>>,
    
    // Immunity cache
    immunity_cache: HashMap<String, HashMap<String, ImmunityInstance>>,
    
    // Calculation cache
    calculation_cache: HashMap<String, HashMap<String, CachedCalculation>>,
    
    // Cache configuration
    cache_ttl: Duration,
    max_cache_size: usize,
    
    // Performance metrics
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
}

#[derive(Debug, Clone)]
pub struct CachedCalculation {
    pub magnitude: Option<f64>,
    pub duration: Option<Duration>,
    pub calculated_at: Instant,
}

impl StatusCache {
    /// Get cached actor status effects
    pub async fn get_actor_status_effects(&self, actor_id: &str) -> Option<&HashMap<String, StatusEffectInstance>> {
        self.effect_cache.get(actor_id)
    }
    
    /// Update actor status effects
    pub async fn update_actor_status_effects(
        &mut self,
        actor_id: &str,
        effects: &[StatusEffectInstance]
    ) -> Result<(), StatusError> {
        let mut effect_map = HashMap::new();
        for effect in effects {
            effect_map.insert(effect.effect_id.clone(), effect.clone());
        }
        self.effect_cache.insert(actor_id.to_string(), effect_map);
        Ok(())
    }
    
    /// Get cached calculation
    pub async fn get_calculation(
        &self,
        effect_id: &str,
        actor_id: &str
    ) -> Option<&CachedCalculation> {
        self.calculation_cache.get(effect_id)?.get(actor_id)
    }
    
    /// Cache calculation
    pub async fn cache_calculation(
        &mut self,
        effect_id: &str,
        actor_id: &str,
        calculation: CachedCalculation
    ) -> Result<(), StatusError> {
        self.calculation_cache
            .entry(effect_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(actor_id.to_string(), calculation);
        Ok(())
    }
    
    /// Cleanup expired cache entries
    pub async fn cleanup_expired_cache(&mut self) {
        let now = Instant::now();
        let mut to_remove = Vec::new();
        
        for (effect_id, actor_calculations) in &self.calculation_cache {
            for (actor_id, calculation) in actor_calculations {
                if now.duration_since(calculation.calculated_at) > self.cache_ttl {
                    to_remove.push((effect_id.clone(), actor_id.clone()));
                }
            }
        }
        
        for (effect_id, actor_id) in to_remove {
            if let Some(actor_calculations) = self.calculation_cache.get_mut(&effect_id) {
                actor_calculations.remove(&actor_id);
            }
        }
    }
}
```

### **2. Memory Pool**

```rust
/// Status Memory Pool for efficient memory management
pub struct StatusMemoryPool {
    // Object pools
    effect_pool: ObjectPool<StatusEffectInstance>,
    immunity_pool: ObjectPool<ImmunityInstance>,
    result_pool: ObjectPool<StatusEffectResult>,
    
    // Pool configuration
    pool_size: usize,
    max_pool_size: usize,
    
    // Performance metrics
    allocations: AtomicU64,
    deallocations: AtomicU64,
    pool_hits: AtomicU64,
}

impl StatusMemoryPool {
    /// Get effect instance from pool
    pub fn get_effect_instance(&self) -> PooledObject<StatusEffectInstance> {
        self.effect_pool.get()
    }
    
    /// Return effect instance to pool
    pub fn return_effect_instance(&self, instance: PooledObject<StatusEffectInstance>) {
        self.effect_pool.return_object(instance);
    }
    
    /// Get immunity instance from pool
    pub fn get_immunity_instance(&self) -> PooledObject<ImmunityInstance> {
        self.immunity_pool.get()
    }
    
    /// Return immunity instance to pool
    pub fn return_immunity_instance(&self, instance: PooledObject<ImmunityInstance>) {
        self.immunity_pool.return_object(instance);
    }
    
    /// Get result from pool
    pub fn get_result(&self) -> PooledObject<StatusEffectResult> {
        self.result_pool.get()
    }
    
    /// Return result to pool
    pub fn return_result(&self, result: PooledObject<StatusEffectResult>) {
        self.result_pool.return_object(result);
    }
}
```

## 🧪 **Testing Strategy**

### **1. Unit Testing**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_status_effect_application() {
        let mut engine = StatusCoreEngine::new(create_test_config()).await?;
        let actor_id = "test_actor";
        let status_effect = create_test_status_effect();
        let context = create_test_status_context();
        
        let result = engine.apply_status_effect(actor_id, status_effect, &context).await?;
        assert!(result.success);
        assert_eq!(result.effect_id, "test_effect");
    }
    
    #[tokio::test]
    async fn test_status_effect_stacking() {
        let mut engine = StatusCoreEngine::new(create_test_config()).await?;
        let actor_id = "test_actor";
        let context = create_test_status_context();
        
        // Apply first effect
        let effect1 = create_test_status_effect_with_id("effect1");
        let result1 = engine.apply_status_effect(actor_id, effect1, &context).await?;
        assert!(result1.success);
        
        // Apply second effect with same ID
        let effect2 = create_test_status_effect_with_id("effect1");
        let result2 = engine.apply_status_effect(actor_id, effect2, &context).await?;
        assert!(result2.success);
        
        // Check that only one effect is active
        let active_effects = engine.get_actor_status_effects(actor_id).await?;
        assert_eq!(active_effects.len(), 1);
    }
    
    #[tokio::test]
    async fn test_immunity_system() {
        let mut engine = StatusCoreEngine::new(create_test_config()).await?;
        let actor_id = "test_actor";
        let context = create_test_status_context();
        
        // Apply immunity
        let immunity = create_test_immunity();
        let immunity_result = engine.immunity_manager.apply_immunity(actor_id, immunity, &context).await?;
        assert!(immunity_result.success);
        
        // Try to apply effect that should be blocked
        let status_effect = create_test_status_effect();
        let result = engine.apply_status_effect(actor_id, status_effect, &context).await?;
        assert!(!result.success);
        assert_eq!(result.reason, StatusEffectFailureReason::Immunity);
    }
}
```

### **2. Performance Testing**

```rust
#[tokio::test]
async fn test_status_effect_performance() {
    let mut engine = StatusCoreEngine::new(create_test_config()).await?;
    let context = create_test_status_context();
    
    // Test single effect application
    let start_time = Instant::now();
    let result = engine.apply_status_effect("test_actor", create_test_status_effect(), &context).await?;
    let single_effect_time = start_time.elapsed();
    
    assert!(single_effect_time.as_millis() < 1); // Should complete in < 1ms
    
    // Test batch effect processing
    let effects = (0..1000).map(|i| create_test_status_effect_with_id(&format!("effect_{}", i))).collect();
    let start_time = Instant::now();
    let results = engine.process_actor_status_effects("test_actor", &context).await?;
    let batch_processing_time = start_time.elapsed();
    
    assert!(batch_processing_time.as_millis() < 100); // Should complete in < 100ms
    assert_eq!(results.len(), 1000);
}
```

## 📝 **Implementation Notes**

### **1. Core System Design**
- **Single Responsibility**: Mỗi component có trách nhiệm rõ ràng
- **High Performance**: Tối ưu cho high-load scenarios
- **Thread Safety**: Thread-safe operations cho concurrency
- **Memory Efficient**: Efficient memory usage và management

### **2. Status Effect Processing**
- **Real-time Processing**: Process status effects trong real-time
- **Batch Processing**: Process multiple effects efficiently
- **Priority System**: Priority-based effect resolution
- **Stacking Rules**: Complex stacking rules cho effects

### **3. Immunity System**
- **Comprehensive Immunity**: Support multiple immunity types
- **Immunity Stacking**: Stack immunity effects
- **Immunity Break**: Break immunity conditions
- **Immunity Override**: Override immunity rules

## 🔄 **New Types for Optimized Design**

### **Status Effect Processing Result**

```rust
/// Status Effect Processing Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectProcessingResult {
    pub effect_id: String,
    pub success: bool,
    pub requires_damage_calculation: bool,
    pub damage_request: Option<StatusDamageRequest>,
    pub status_changes: Vec<StatusChange>,
    pub visual_effects: Vec<VisualEffect>,
    pub audio_effects: Vec<AudioEffect>,
}

/// Status Damage Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDamageRequest {
    pub effect_id: String,
    pub effect_type: StatusEffectType,
    pub base_magnitude: f64,
    pub element_id: String,
    pub actor_id: String,
    pub damage_type: StatusDamageType,
    pub properties: HashMap<String, serde_json::Value>,
}

/// Status Damage Type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusDamageType {
    DamageOverTime,
    HealOverTime,
    StatModifier,
    MovementRestriction,
    Control,
    Immunity,
    Transformation,
}

/// Status Change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusChange {
    pub change_type: StatusChangeType,
    pub stat_name: String,
    pub old_value: f64,
    pub new_value: f64,
    pub change_amount: f64,
}

/// Status Change Type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusChangeType {
    StatModifier,
    MovementRestriction,
    Control,
    Immunity,
    Transformation,
}
```

### **Status Core Interface for Combat Core**

```rust
/// Status Core Interface for Combat Core Integration
#[async_trait]
pub trait StatusCoreInterface: Send + Sync {
    /// Process status effects and return damage requests
    async fn process_actor_status_effects(
        &self,
        actor_id: &str,
        context: &StatusContext
    ) -> Result<Vec<StatusEffectProcessingResult>, StatusError>;
    
    /// Get active status effects for an actor
    async fn get_actor_status_effects(
        &self,
        actor_id: &str
    ) -> Result<Vec<StatusEffectInstance>, StatusError>;
    
    /// Apply status effect to actor
    async fn apply_status_effect(
        &self,
        actor_id: &str,
        status_effect: StatusEffect,
        context: &StatusContext
    ) -> Result<StatusEffectResult, StatusError>;
    
    /// Remove status effect from actor
    async fn remove_status_effect(
        &self,
        actor_id: &str,
        effect_id: &str
    ) -> Result<StatusRemovalResult, StatusError>;
}
```

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
