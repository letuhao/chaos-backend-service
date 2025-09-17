# Status Core Integration System Design

## 📋 **Tổng Quan**

Status Core Integration System thiết kế cách tích hợp Status Core với các hệ thống khác (Element Core, Action Core, Combat Core) một cách seamless và hiệu quả, đảm bảo tính nhất quán và performance tối ưu.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Seamless Integration**
- **Zero Breaking Changes**: Không phá vỡ existing systems
- **Backward Compatibility**: Tương thích với code cũ
- **Gradual Migration**: Migration từng bước, không cần rewrite toàn bộ
- **Performance Neutral**: Không ảnh hưởng performance của existing systems

### **2. Unified Status Management**
- **Single Source of Truth**: Status Core là trung tâm quản lý status
- **Centralized Processing**: Tất cả status effects được xử lý tập trung
- **Consistent API**: Unified API cho tất cả systems
- **Event-Driven**: Event-driven architecture cho real-time updates

### **3. Performance Optimization**
- **Minimal Overhead**: Tối thiểu overhead cho integration
- **Smart Caching**: Intelligent caching cho frequently used data
- **Batch Processing**: Process multiple status effects efficiently
- **Async Operations**: Non-blocking operations cho better performance

## 🏗️ **Integration Architecture**

### **1. Core Integration Components**

```rust
/// Status Core Integration Manager
pub struct StatusCoreIntegrationManager {
    // Core systems
    status_core: Arc<StatusCore>,
    element_core_client: ElementCoreClient,
    action_core_client: ActionCoreClient,
    combat_core_client: CombatCoreClient,
    
    // Integration bridges
    element_status_bridge: ElementStatusBridge,
    action_status_bridge: ActionStatusBridge,
    combat_status_bridge: CombatStatusBridge,
    
    // Performance optimization
    integration_cache: IntegrationCache,
    batch_processor: StatusBatchProcessor,
    event_dispatcher: StatusEventDispatcher,
}

/// Element Core Integration Bridge
pub struct ElementStatusBridge {
    element_core_client: ElementCoreClient,
    status_core_client: StatusCoreClient,
    element_status_mapper: ElementStatusMapper,
    status_effect_converter: StatusEffectConverter,
}

/// Action Core Integration Bridge
pub struct ActionStatusBridge {
    action_core_client: ActionCoreClient,
    status_core_client: StatusCoreClient,
    action_status_mapper: ActionStatusMapper,
    status_condition_evaluator: StatusConditionEvaluator,
}

/// Combat Core Integration Bridge
pub struct CombatStatusBridge {
    combat_core_client: CombatCoreClient,
    status_core_client: StatusCoreClient,
    combat_status_mapper: CombatStatusMapper,
    status_damage_calculator: StatusDamageCalculator,
}
```

### **2. Element Core Integration**

```rust
impl ElementStatusBridge {
    /// Convert Element Core status effects to Status Core format
    pub async fn convert_elemental_status_effects(
        &self,
        element_effects: &[ElementalStatusEffect]
    ) -> Result<Vec<StatusEffect>, StatusError> {
        let mut status_effects = Vec::new();
        
        for element_effect in element_effects {
            let status_effect = self.convert_elemental_to_status_effect(element_effect).await?;
            status_effects.push(status_effect);
        }
        
        Ok(status_effects)
    }
    
    /// Apply elemental status effects
    pub async fn apply_elemental_status_effects(
        &self,
        actor: &mut Actor,
        element_interaction: &ElementInteraction
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        let mut results = Vec::new();
        
        // Get elemental status effects from Element Core
        let element_effects = self.element_core_client.get_elemental_status_effects(
            &element_interaction.element_id
        ).await?;
        
        // Convert to Status Core format
        let status_effects = self.convert_elemental_status_effects(&element_effects).await?;
        
        // Apply status effects
        for status_effect in status_effects {
            let result = self.status_core_client.apply_status_effect(
                actor.get_id(),
                status_effect
            ).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Process elemental status interactions
    pub async fn process_elemental_status_interactions(
        &self,
        actor: &Actor,
        status_effects: &[StatusEffect]
    ) -> Result<Vec<StatusInteractionResult>, StatusError> {
        let mut results = Vec::new();
        
        for status_effect in status_effects {
            if let StatusCategory::Elemental(elemental_category) = &status_effect.category {
                let interaction_result = self.process_elemental_interaction(
                    actor,
                    status_effect,
                    elemental_category
                ).await?;
                results.push(interaction_result);
            }
        }
        
        Ok(results)
    }
    
    /// Calculate elemental status magnitude
    fn calculate_elemental_magnitude(
        &self,
        effect: &StatusEffect,
        actor: &Actor,
        element_mastery: f64
    ) -> Result<f64, StatusError> {
        let base_magnitude = effect.magnitude.base_value;
        let scaling_factor = effect.magnitude.scaling_factor;
        let scaling_stat = &effect.magnitude.scaling_stat;
        
        // Get scaling stat value from Element Core
        let scaling_value = self.element_core_client.get_element_mastery(
            actor.get_id(),
            scaling_stat
        ).await?;
        
        // Calculate final magnitude
        let final_magnitude = base_magnitude + (scaling_value * scaling_factor);
        
        // Apply min/max constraints
        let final_magnitude = final_magnitude
            .max(effect.magnitude.min_value)
            .min(effect.magnitude.max_value);
        
        Ok(final_magnitude)
    }
}
```

### **3. Action Core Integration**

```rust
impl ActionStatusBridge {
    /// Apply status effects from action execution
    pub async fn apply_action_status_effects(
        &self,
        action: &dyn Action,
        actor: &mut Actor,
        context: &ActionContext
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        let mut results = Vec::new();
        
        // Get status effects from action
        let action_effects = action.get_status_effects();
        
        // Convert to Status Core format
        let status_effects = self.convert_action_effects_to_status_effects(action_effects).await?;
        
        // Apply status effects
        for status_effect in status_effects {
            let result = self.status_core_client.apply_status_effect(
                actor.get_id(),
                status_effect
            ).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Evaluate status conditions for action execution
    pub async fn evaluate_status_conditions(
        &self,
        actor: &Actor,
        conditions: &[StatusCondition]
    ) -> Result<bool, StatusError> {
        for condition in conditions {
            let condition_result = self.evaluate_status_condition(actor, condition).await?;
            if !condition_result {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Process status effects during action execution
    pub async fn process_action_status_effects(
        &self,
        actor: &mut Actor,
        action: &dyn Action,
        context: &ActionContext
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        let mut results = Vec::new();
        
        // Get active status effects
        let active_effects = self.status_core_client.get_actor_status_effects(
            actor.get_id()
        ).await?;
        
        // Process each active effect
        for status_effect in active_effects {
            let effect_result = self.process_status_effect_during_action(
                actor,
                &status_effect,
                action,
                context
            ).await?;
            results.push(effect_result);
        }
        
        Ok(results)
    }
    
    /// Calculate action status bonuses
    fn calculate_action_status_bonuses(
        &self,
        actor: &Actor,
        action: &dyn Action,
        status_effects: &[StatusEffect]
    ) -> Result<ActionStatusBonuses, StatusError> {
        let mut bonuses = ActionStatusBonuses::new();
        
        for status_effect in status_effects {
            let effect_bonuses = self.calculate_status_effect_bonuses(
                actor,
                action,
                status_effect
            )?;
            bonuses.merge(effect_bonuses);
        }
        
        Ok(bonuses)
    }
}
```

### **4. Combat Core Integration**

```rust
impl CombatStatusBridge {
    /// Apply status effects during combat
    pub async fn apply_combat_status_effects(
        &self,
        attacker: &mut Actor,
        target: &mut Actor,
        combat_action: &CombatAction
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        let mut results = Vec::new();
        
        // Get combat status effects from Combat Core
        let combat_effects = self.combat_core_client.get_combat_status_effects(
            combat_action
        ).await?;
        
        // Convert to Status Core format
        let status_effects = self.convert_combat_effects_to_status_effects(combat_effects).await?;
        
        // Apply status effects to target
        for status_effect in status_effects {
            let result = self.status_core_client.apply_status_effect(
                target.get_id(),
                status_effect
            ).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Calculate status damage
    pub async fn calculate_status_damage(
        &self,
        attacker: &Actor,
        target: &Actor,
        status_effect: &StatusEffect
    ) -> Result<f64, StatusError> {
        // Get combat stats from Combat Core
        let attacker_stats = self.combat_core_client.get_actor_combat_stats(attacker).await?;
        let target_stats = self.combat_core_client.get_actor_combat_stats(target).await?;
        
        // Calculate status damage
        let base_damage = status_effect.magnitude.base_value;
        let scaling_factor = status_effect.magnitude.scaling_factor;
        let scaling_stat = &status_effect.magnitude.scaling_stat;
        
        // Get scaling stat value
        let scaling_value = self.get_scaling_stat_value(attacker, scaling_stat).await?;
        
        // Calculate final damage
        let final_damage = base_damage + (scaling_value * scaling_factor);
        
        // Apply combat modifiers
        let modified_damage = self.apply_combat_modifiers(
            final_damage,
            attacker_stats,
            target_stats,
            status_effect
        ).await?;
        
        Ok(modified_damage)
    }
    
    /// Process status effects during damage calculation
    pub async fn process_status_effects_during_damage(
        &self,
        attacker: &Actor,
        target: &Actor,
        base_damage: f64,
        damage_type: &str
    ) -> Result<f64, StatusError> {
        let mut final_damage = base_damage;
        
        // Get attacker status effects
        let attacker_effects = self.status_core_client.get_actor_status_effects(
            attacker.get_id()
        ).await?;
        
        // Get target status effects
        let target_effects = self.status_core_client.get_actor_status_effects(
            target.get_id()
        ).await?;
        
        // Apply attacker status bonuses
        for status_effect in attacker_effects {
            if self.is_damage_amplifying_effect(&status_effect, damage_type) {
                let bonus = self.calculate_damage_amplification_bonus(
                    &status_effect,
                    attacker
                ).await?;
                final_damage *= (1.0 + bonus);
            }
        }
        
        // Apply target status resistances
        for status_effect in target_effects {
            if self.is_damage_resistance_effect(&status_effect, damage_type) {
                let resistance = self.calculate_damage_resistance(
                    &status_effect,
                    target
                ).await?;
                final_damage *= (1.0 - resistance);
            }
        }
        
        Ok(final_damage)
    }
}
```

## 🔧 **Status Effect Conversion System**

### **1. Elemental Status Effect Conversion**

```rust
/// Elemental Status Effect Converter
pub struct ElementalStatusEffectConverter {
    element_core_client: ElementCoreClient,
    status_core_client: StatusCoreClient,
    conversion_rules: HashMap<String, ConversionRule>,
}

impl ElementalStatusEffectConverter {
    /// Convert Elemental Status Effect to Status Core format
    pub async fn convert_elemental_to_status_effect(
        &self,
        element_effect: &ElementalStatusEffect
    ) -> Result<StatusEffect, StatusError> {
        let conversion_rule = self.get_conversion_rule(&element_effect.effect_type)?;
        
        let status_effect = StatusEffect {
            effect_id: format!("elemental_{}", element_effect.effect_name),
            effect_name: element_effect.effect_name.clone(),
            effect_name_vi: self.translate_effect_name(&element_effect.effect_name),
            category: StatusCategory::Elemental(element_effect.element_category),
            effect_type: self.convert_elemental_effect_type(element_effect.effect_type),
            magnitude: StatusMagnitude {
                base_value: element_effect.magnitude,
                scaling_factor: conversion_rule.scaling_factor,
                scaling_stat: format!("{}_mastery", element_effect.element_id),
                min_value: conversion_rule.min_magnitude,
                max_value: conversion_rule.max_magnitude,
                calculation_formula: conversion_rule.magnitude_formula.clone(),
            },
            duration: StatusDuration {
                base_duration: element_effect.duration,
                scaling_factor: conversion_rule.duration_scaling_factor,
                scaling_stat: format!("{}_mastery", element_effect.element_id),
                min_duration: conversion_rule.min_duration,
                max_duration: conversion_rule.max_duration,
                calculation_formula: conversion_rule.duration_formula.clone(),
            },
            target: self.convert_target(element_effect.target),
            source: StatusSource::Element(element_effect.element_id.clone()),
            conditions: self.convert_conditions(element_effect),
            interactions: self.convert_interactions(element_effect),
            immunity_list: self.convert_immunity_list(element_effect),
            movement_restrictions: self.convert_movement_restrictions(element_effect),
            visual_effects: self.convert_visual_effects(element_effect),
            audio_effects: self.convert_audio_effects(element_effect),
            properties: self.convert_properties(element_effect),
            priority: conversion_rule.priority,
            is_active: true,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
        };
        
        Ok(status_effect)
    }
    
    /// Get conversion rule for effect type
    fn get_conversion_rule(&self, effect_type: &ElementalStatusEffectType) -> Result<&ConversionRule, StatusError> {
        let rule_key = format!("{:?}", effect_type);
        self.conversion_rules.get(&rule_key)
            .ok_or_else(|| StatusError::ConversionRuleNotFound(rule_key))
    }
    
    /// Convert elemental effect type to status effect type
    fn convert_elemental_effect_type(&self, effect_type: ElementalStatusEffectType) -> StatusEffectType {
        match effect_type {
            ElementalStatusEffectType::Burning => StatusEffectType::Elemental(ElementalStatusType::Burning),
            ElementalStatusEffectType::Freezing => StatusEffectType::Elemental(ElementalStatusType::Freezing),
            ElementalStatusEffectType::Electrified => StatusEffectType::Elemental(ElementalStatusType::Electrified),
            ElementalStatusEffectType::Poisoned => StatusEffectType::Elemental(ElementalStatusType::Poisoned),
            ElementalStatusEffectType::Crystallized => StatusEffectType::Elemental(ElementalStatusType::Crystallized),
            ElementalStatusEffectType::Regeneration => StatusEffectType::Elemental(ElementalStatusType::Regeneration),
            ElementalStatusEffectType::Resistance => StatusEffectType::Elemental(ElementalStatusType::Resistance),
            ElementalStatusEffectType::Immunity => StatusEffectType::Elemental(ElementalStatusType::Immunity),
            ElementalStatusEffectType::Amplification => StatusEffectType::Elemental(ElementalStatusType::Amplification),
            ElementalStatusEffectType::Suppression => StatusEffectType::Elemental(ElementalStatusType::Suppression),
        }
    }
}

/// Conversion Rule for Elemental Status Effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRule {
    pub effect_type: ElementalStatusEffectType,
    pub scaling_factor: f64,
    pub min_magnitude: f64,
    pub max_magnitude: f64,
    pub magnitude_formula: String,
    pub duration_scaling_factor: f64,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub duration_formula: String,
    pub priority: u32,
}
```

### **2. Action Status Effect Conversion**

```rust
/// Action Status Effect Converter
pub struct ActionStatusEffectConverter {
    action_core_client: ActionCoreClient,
    status_core_client: StatusCoreClient,
    conversion_rules: HashMap<String, ActionConversionRule>,
}

impl ActionStatusEffectConverter {
    /// Convert Action Status Effect to Status Core format
    pub async fn convert_action_to_status_effect(
        &self,
        action_effect: &ActionStatusEffect
    ) -> Result<StatusEffect, StatusError> {
        let conversion_rule = self.get_action_conversion_rule(&action_effect.effect_type)?;
        
        let status_effect = StatusEffect {
            effect_id: format!("action_{}", action_effect.effect_name),
            effect_name: action_effect.effect_name.clone(),
            effect_name_vi: self.translate_effect_name(&action_effect.effect_name),
            category: self.convert_action_category(&action_effect.effect_type),
            effect_type: self.convert_action_effect_type(action_effect.effect_type),
            magnitude: StatusMagnitude {
                base_value: action_effect.magnitude,
                scaling_factor: conversion_rule.scaling_factor,
                scaling_stat: conversion_rule.scaling_stat.clone(),
                min_value: conversion_rule.min_magnitude,
                max_value: conversion_rule.max_magnitude,
                calculation_formula: conversion_rule.magnitude_formula.clone(),
            },
            duration: StatusDuration {
                base_duration: action_effect.duration,
                scaling_factor: conversion_rule.duration_scaling_factor,
                scaling_stat: conversion_rule.duration_scaling_stat.clone(),
                min_duration: conversion_rule.min_duration,
                max_duration: conversion_rule.max_duration,
                calculation_formula: conversion_rule.duration_formula.clone(),
            },
            target: self.convert_action_target(action_effect.target),
            source: StatusSource::Action(action_effect.action_id.clone()),
            conditions: self.convert_action_conditions(action_effect),
            interactions: self.convert_action_interactions(action_effect),
            immunity_list: self.convert_action_immunity_list(action_effect),
            movement_restrictions: self.convert_action_movement_restrictions(action_effect),
            visual_effects: self.convert_action_visual_effects(action_effect),
            audio_effects: self.convert_action_audio_effects(action_effect),
            properties: self.convert_action_properties(action_effect),
            priority: conversion_rule.priority,
            is_active: true,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
        };
        
        Ok(status_effect)
    }
}

/// Action Conversion Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConversionRule {
    pub effect_type: ActionStatusEffectType,
    pub scaling_factor: f64,
    pub scaling_stat: String,
    pub min_magnitude: f64,
    pub max_magnitude: f64,
    pub magnitude_formula: String,
    pub duration_scaling_factor: f64,
    pub duration_scaling_stat: String,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub duration_formula: String,
    pub priority: u32,
}
```

## 🚀 **Performance Optimization**

### **1. Integration Caching**

```rust
/// Integration Cache for performance optimization
pub struct IntegrationCache {
    element_status_cache: HashMap<String, CachedElementStatus>,
    action_status_cache: HashMap<String, CachedActionStatus>,
    combat_status_cache: HashMap<String, CachedCombatStatus>,
    conversion_cache: HashMap<String, CachedConversion>,
    cache_ttl: Duration,
    last_update: HashMap<String, Instant>,
}

#[derive(Debug, Clone)]
pub struct CachedElementStatus {
    pub element_effects: Vec<ElementalStatusEffect>,
    pub status_effects: Vec<StatusEffect>,
    pub last_accessed: Instant,
}

#[derive(Debug, Clone)]
pub struct CachedActionStatus {
    pub action_effects: Vec<ActionStatusEffect>,
    pub status_effects: Vec<StatusEffect>,
    pub last_accessed: Instant,
}

#[derive(Debug, Clone)]
pub struct CachedCombatStatus {
    pub combat_effects: Vec<CombatStatusEffect>,
    pub status_effects: Vec<StatusEffect>,
    pub last_accessed: Instant,
}

#[derive(Debug, Clone)]
pub struct CachedConversion {
    pub source_effect: serde_json::Value,
    pub converted_effect: StatusEffect,
    pub last_accessed: Instant,
}

impl IntegrationCache {
    /// Get cached element status
    pub fn get_element_status(&self, element_id: &str) -> Option<&CachedElementStatus> {
        self.element_status_cache.get(element_id)
    }
    
    /// Cache element status
    pub fn cache_element_status(&mut self, element_id: String, status: CachedElementStatus) {
        self.element_status_cache.insert(element_id, status);
    }
    
    /// Get cached action status
    pub fn get_action_status(&self, action_id: &str) -> Option<&CachedActionStatus> {
        self.action_status_cache.get(action_id)
    }
    
    /// Cache action status
    pub fn cache_action_status(&mut self, action_id: String, status: CachedActionStatus) {
        self.action_status_cache.insert(action_id, status);
    }
    
    /// Get cached conversion
    pub fn get_conversion(&self, conversion_key: &str) -> Option<&CachedConversion> {
        self.conversion_cache.get(conversion_key)
    }
    
    /// Cache conversion
    pub fn cache_conversion(&mut self, conversion_key: String, conversion: CachedConversion) {
        self.conversion_cache.insert(conversion_key, conversion);
    }
    
    /// Check if cache is valid
    pub fn is_cache_valid(&self, key: &str) -> bool {
        if let Some(last_update) = self.last_update.get(key) {
            last_update.elapsed() < self.cache_ttl
        } else {
            false
        }
    }
    
    /// Invalidate cache
    pub fn invalidate_cache(&mut self, key: &str) {
        self.last_update.remove(key);
    }
}
```

### **2. Batch Processing**

```rust
/// Status Integration Batch Processor
pub struct StatusIntegrationBatchProcessor {
    integration_manager: Arc<StatusCoreIntegrationManager>,
    cache: IntegrationCache,
    batch_size: usize,
}

impl StatusIntegrationBatchProcessor {
    /// Process multiple status integrations in batch
    pub async fn process_status_integrations_batch(
        &mut self,
        integrations: Vec<StatusIntegrationRequest>
    ) -> Result<Vec<StatusIntegrationResult>, StatusError> {
        let mut results = Vec::new();
        
        // Group integrations by type for efficient processing
        let mut grouped_integrations: HashMap<IntegrationType, Vec<StatusIntegrationRequest>> = HashMap::new();
        for integration in integrations {
            grouped_integrations.entry(integration.integration_type)
                .or_insert_with(Vec::new)
                .push(integration);
        }
        
        // Process each integration type
        for (integration_type, integration_requests) in grouped_integrations {
            let type_results = self.process_integration_type_batch(
                integration_type,
                integration_requests
            ).await?;
            results.extend(type_results);
        }
        
        Ok(results)
    }
    
    /// Process integrations for a specific type
    async fn process_integration_type_batch(
        &self,
        integration_type: IntegrationType,
        requests: Vec<StatusIntegrationRequest>
    ) -> Result<Vec<StatusIntegrationResult>, StatusError> {
        let mut results = Vec::new();
        
        match integration_type {
            IntegrationType::Elemental => {
                for request in requests {
                    let result = self.process_elemental_integration(request).await?;
                    results.push(result);
                }
            },
            IntegrationType::Action => {
                for request in requests {
                    let result = self.process_action_integration(request).await?;
                    results.push(result);
                }
            },
            IntegrationType::Combat => {
                for request in requests {
                    let result = self.process_combat_integration(request).await?;
                    results.push(result);
                }
            },
        }
        
        Ok(results)
    }
}

/// Status Integration Request
#[derive(Debug, Clone)]
pub struct StatusIntegrationRequest {
    pub integration_type: IntegrationType,
    pub actor_id: String,
    pub source_data: serde_json::Value,
    pub target_system: String,
    pub priority: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntegrationType {
    Elemental,
    Action,
    Combat,
}

/// Status Integration Result
#[derive(Debug, Clone)]
pub struct StatusIntegrationResult {
    pub request_id: String,
    pub success: bool,
    pub status_effects: Vec<StatusEffect>,
    pub errors: Vec<StatusError>,
    pub processing_time: Duration,
}
```

## 🧪 **Testing Strategy**

### **1. Integration Testing**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_element_status_integration() {
        let integration_manager = StatusCoreIntegrationManager::new();
        let element_bridge = integration_manager.element_status_bridge;
        
        // Test elemental status effect conversion
        let element_effect = create_test_elemental_effect();
        let status_effect = element_bridge.convert_elemental_to_status_effect(&element_effect).await?;
        
        assert_eq!(status_effect.effect_id, "elemental_burning");
        assert_eq!(status_effect.category, StatusCategory::Elemental(ElementalCategory::Fire));
        assert_eq!(status_effect.source, StatusSource::Element("fire".to_string()));
    }
    
    #[tokio::test]
    async fn test_action_status_integration() {
        let integration_manager = StatusCoreIntegrationManager::new();
        let action_bridge = integration_manager.action_status_bridge;
        
        // Test action status effect application
        let action = create_test_action();
        let mut actor = create_test_actor();
        let context = create_test_action_context();
        
        let results = action_bridge.apply_action_status_effects(
            &action,
            &mut actor,
            &context
        ).await?;
        
        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.success));
    }
    
    #[tokio::test]
    async fn test_combat_status_integration() {
        let integration_manager = StatusCoreIntegrationManager::new();
        let combat_bridge = integration_manager.combat_status_bridge;
        
        // Test combat status damage calculation
        let attacker = create_test_actor();
        let target = create_test_actor();
        let status_effect = create_test_status_effect();
        
        let damage = combat_bridge.calculate_status_damage(
            &attacker,
            &target,
            &status_effect
        ).await?;
        
        assert!(damage > 0.0);
    }
}
```

### **2. Performance Testing**

```rust
#[tokio::test]
async fn test_integration_performance() {
    let integration_manager = StatusCoreIntegrationManager::new();
    let batch_processor = StatusIntegrationBatchProcessor::new(integration_manager);
    
    // Create test requests
    let mut requests = Vec::new();
    for i in 0..1000 {
        requests.push(StatusIntegrationRequest {
            integration_type: IntegrationType::Elemental,
            actor_id: format!("actor_{}", i),
            source_data: serde_json::json!({"element_id": "fire"}),
            target_system: "status_core".to_string(),
            priority: 100,
        });
    }
    
    // Process batch
    let start_time = Instant::now();
    let results = batch_processor.process_status_integrations_batch(requests).await?;
    let processing_time = start_time.elapsed();
    
    // Verify performance
    assert!(processing_time.as_millis() < 100); // Should complete in < 100ms
    assert_eq!(results.len(), 1000);
    assert!(results.iter().all(|r| r.success));
}
```

## 📝 **Implementation Notes**

### **1. Migration Strategy**
- **Phase 1**: Implement Status Core với basic functionality
- **Phase 2**: Implement integration bridges cho existing systems
- **Phase 3**: Migrate existing status effects từ other systems
- **Phase 4**: Optimize performance và add advanced features

### **2. Backward Compatibility**
- **Legacy Support**: Maintain support cho existing status systems
- **Gradual Migration**: Allow gradual migration từ old to new system
- **API Compatibility**: Maintain API compatibility during migration
- **Data Migration**: Provide tools cho data migration

### **3. Performance Considerations**
- **Minimal Overhead**: Keep integration overhead minimal
- **Smart Caching**: Use intelligent caching strategies
- **Batch Processing**: Process multiple operations efficiently
- **Async Operations**: Use async operations cho better performance

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
