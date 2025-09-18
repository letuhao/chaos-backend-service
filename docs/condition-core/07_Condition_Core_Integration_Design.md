# Condition Core Integration Design

## 📋 **Tổng Quan**

Tài liệu này thiết kế chi tiết cách Condition Core tích hợp với các systems khác trong Chaos World, bao gồm Action Core, Status Core, Element Core, Effect Core, và các systems tương lai.

## 🏗️ **Integration Architecture**

### **1. Integration Layers**

```
Condition Core Integration
├── Action Core Integration
│   ├── Action Condition Evaluation
│   ├── Action Resource Validation
│   ├── Action Timing Validation
│   └── Action Target Validation
├── Status Core Integration
│   ├── Status Application Conditions
│   ├── Status Duration Conditions
│   ├── Status Interaction Conditions
│   └── Status Immunity Conditions
├── Element Core Integration
│   ├── Element Mastery Conditions
│   ├── Element Resistance Conditions
│   ├── Element Interaction Conditions
│   └── Element Derived Stat Conditions
├── Effect Core Integration
│   ├── Effect Application Conditions
│   ├── Effect Duration Conditions
│   ├── Effect Interaction Conditions
│   └── Effect Chain Conditions
└── Future Systems Integration
    ├── Talent Core Integration
    ├── Perk Core Integration
    └── Skill Core Integration
```

## 🔧 **Action Core Integration**

### **1. Action Condition Evaluation**

```rust
// Action Core Integration
pub struct ActionCoreIntegration {
    condition_core: Arc<ConditionCore>,
    action_engine: Arc<ActionEngine>,
    action_cache: Arc<ActionCache>,
}

impl ActionCoreIntegration {
    // Evaluate action execution conditions
    pub async fn evaluate_action_execution_conditions(
        &self,
        action: &Action,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition_id in &action.execution_conditions {
            let result = self.condition_core
                .evaluate_condition(condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Evaluate action resource conditions
    pub async fn evaluate_action_resource_conditions(
        &self,
        action: &Action,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition_id in &action.resource_conditions {
            let result = self.condition_core
                .evaluate_condition(condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Evaluate action timing conditions
    pub async fn evaluate_action_timing_conditions(
        &self,
        action: &Action,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition_id in &action.timing_conditions {
            let result = self.condition_core
                .evaluate_condition(condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Evaluate action target conditions
    pub async fn evaluate_action_target_conditions(
        &self,
        action: &Action,
        target: &ActorTarget,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        let mut condition_context = self.convert_to_condition_context(context);
        condition_context.target = target.clone();
        
        for condition_id in &action.target_conditions {
            let result = self.condition_core
                .evaluate_condition(condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Convert ActionContext to ConditionContext
    fn convert_to_condition_context(&self, action_context: &ActionContext) -> ConditionContext {
        ConditionContext {
            target: ActorTarget { id: action_context.actor_id.clone() },
            world_id: action_context.world_id.clone(),
            current_time: action_context.current_time,
            current_weather: action_context.current_weather,
            world_state: action_context.world_state.clone(),
        }
    }
}
```

### **2. Action Condition Examples**

```rust
// Action Condition Examples
pub struct ActionConditionExamples;

impl ActionConditionExamples {
    // Health condition for action execution
    pub fn create_health_condition() -> ConditionDefinition {
        ConditionDefinition::new(
            "action_health_condition".to_string(),
            "get_actor_value".to_string(),
            ConditionOperator::GreaterThan,
            ConditionValue::Float(0.1), // At least 10% health
        )
        .with_parameters(vec![ConditionParameter::String("health".to_string())])
        .with_categories(vec!["action".to_string(), "health".to_string()])
        .with_priority(100)
    }
    
    // Mana condition for spell casting
    pub fn create_mana_condition() -> ConditionDefinition {
        ConditionDefinition::new(
            "action_mana_condition".to_string(),
            "get_actor_value".to_string(),
            ConditionOperator::GreaterThanOrEqual,
            ConditionValue::Float(50.0), // At least 50 mana
        )
        .with_parameters(vec![ConditionParameter::String("mana".to_string())])
        .with_categories(vec!["action".to_string(), "mana".to_string()])
        .with_priority(90)
    }
    
    // Level condition for action availability
    pub fn create_level_condition() -> ConditionDefinition {
        ConditionDefinition::new(
            "action_level_condition".to_string(),
            "get_level".to_string(),
            ConditionOperator::GreaterThanOrEqual,
            ConditionValue::Integer(10), // At least level 10
        )
        .with_categories(vec!["action".to_string(), "level".to_string()])
        .with_priority(80)
    }
    
    // Combat condition for action execution
    pub fn create_combat_condition() -> ConditionDefinition {
        ConditionDefinition::new(
            "action_combat_condition".to_string(),
            "is_in_combat".to_string(),
            ConditionOperator::Equal,
            ConditionValue::Boolean(true), // Must be in combat
        )
        .with_categories(vec!["action".to_string(), "combat".to_string()])
        .with_priority(70)
    }
}
```

## 🔧 **Status Core Integration**

### **1. Status Condition Evaluation**

```rust
// Status Core Integration
pub struct StatusCoreIntegration {
    condition_core: Arc<ConditionCore>,
    status_engine: Arc<StatusEngine>,
    status_cache: Arc<StatusCache>,
}

impl StatusCoreIntegration {
    // Evaluate status application conditions
    pub async fn evaluate_status_application_conditions(
        &self,
        status_effect: &StatusEffect,
        context: &StatusContext
    ) -> Result<bool, StatusError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition in &status_effect.application_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Evaluate status duration conditions
    pub async fn evaluate_status_duration_conditions(
        &self,
        status_effect: &StatusEffect,
        context: &StatusContext
    ) -> Result<bool, StatusError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition in &status_effect.duration_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Evaluate status interaction conditions
    pub async fn evaluate_status_interaction_conditions(
        &self,
        status_effect: &StatusEffect,
        other_status: &StatusEffect,
        context: &StatusContext
    ) -> Result<bool, StatusError> {
        let mut condition_context = self.convert_to_condition_context(context);
        condition_context.other_status = Some(other_status.clone());
        
        for condition in &status_effect.interaction_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Evaluate status immunity conditions
    pub async fn evaluate_status_immunity_conditions(
        &self,
        status_effect: &StatusEffect,
        target: &ActorTarget,
        context: &StatusContext
    ) -> Result<bool, StatusError> {
        let mut condition_context = self.convert_to_condition_context(context);
        condition_context.target = target.clone();
        
        for condition in &status_effect.immunity_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if result.passed {
                return Ok(true); // Target is immune
            }
        }
        
        Ok(false) // Target is not immune
    }
    
    // Convert StatusContext to ConditionContext
    fn convert_to_condition_context(&self, status_context: &StatusContext) -> ConditionContext {
        ConditionContext {
            target: ActorTarget { id: status_context.target_id.clone() },
            world_id: status_context.world_id.clone(),
            current_time: status_context.current_time,
            current_weather: status_context.current_weather,
            world_state: status_context.world_state.clone(),
            other_status: None,
        }
    }
}
```

### **2. Status Condition Examples**

```rust
// Status Condition Examples
pub struct StatusConditionExamples;

impl StatusConditionExamples {
    // Health condition for status application
    pub fn create_health_status_condition() -> ConditionDefinition {
        ConditionDefinition::new(
            "status_health_condition".to_string(),
            "get_actor_value".to_string(),
            ConditionOperator::LessThan,
            ConditionValue::Float(0.8), // Below 80% health
        )
        .with_parameters(vec![ConditionParameter::String("health".to_string())])
        .with_categories(vec!["status".to_string(), "health".to_string()])
        .with_priority(100)
    }
    
    // Resistance condition for status immunity
    pub fn create_resistance_condition() -> ConditionDefinition {
        ConditionDefinition::new(
            "status_resistance_condition".to_string(),
            "get_actor_value".to_string(),
            ConditionOperator::GreaterThan,
            ConditionValue::Float(0.5), // Above 50% resistance
        )
        .with_parameters(vec![ConditionParameter::String("status_resistance".to_string())])
        .with_categories(vec!["status".to_string(), "resistance".to_string()])
        .with_priority(90)
    }
    
    // Time condition for status duration
    pub fn create_time_condition() -> ConditionDefinition {
        ConditionDefinition::new(
            "status_time_condition".to_string(),
            "get_current_time".to_string(),
            ConditionOperator::Between,
            ConditionValue::List(vec![
                ConditionValue::Float(6.0), // 6 AM
                ConditionValue::Float(18.0), // 6 PM
            ]),
        )
        .with_categories(vec!["status".to_string(), "time".to_string()])
        .with_priority(80)
    }
}
```

## 🔧 **Element Core Integration**

### **1. Element Condition Evaluation**

```rust
// Element Core Integration
pub struct ElementCoreIntegration {
    condition_core: Arc<ConditionCore>,
    element_engine: Arc<ElementEngine>,
    element_cache: Arc<ElementCache>,
}

impl ElementCoreIntegration {
    // Evaluate element mastery conditions
    pub async fn evaluate_element_mastery_conditions(
        &self,
        element_id: &str,
        context: &ElementContext
    ) -> Result<bool, ElementError> {
        let condition_context = self.convert_to_condition_context(context);
        
        let mastery_conditions = self.element_engine
            .get_mastery_conditions(element_id)
            .await?;
        
        for condition in mastery_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Evaluate element resistance conditions
    pub async fn evaluate_element_resistance_conditions(
        &self,
        element_id: &str,
        context: &ElementContext
    ) -> Result<bool, ElementError> {
        let condition_context = self.convert_to_condition_context(context);
        
        let resistance_conditions = self.element_engine
            .get_resistance_conditions(element_id)
            .await?;
        
        for condition in resistance_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Evaluate element interaction conditions
    pub async fn evaluate_element_interaction_conditions(
        &self,
        source_element: &str,
        target_element: &str,
        context: &ElementContext
    ) -> Result<bool, ElementError> {
        let mut condition_context = self.convert_to_condition_context(context);
        condition_context.source_element = Some(source_element.to_string());
        condition_context.target_element = Some(target_element.to_string());
        
        let interaction_conditions = self.element_engine
            .get_interaction_conditions(source_element, target_element)
            .await?;
        
        for condition in interaction_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Convert ElementContext to ConditionContext
    fn convert_to_condition_context(&self, element_context: &ElementContext) -> ConditionContext {
        ConditionContext {
            target: ActorTarget { id: element_context.actor_id.clone() },
            world_id: element_context.world_id.clone(),
            current_time: element_context.current_time,
            current_weather: element_context.current_weather,
            world_state: element_context.world_state.clone(),
            source_element: None,
            target_element: None,
        }
    }
}
```

### **2. Element Condition Examples**

```rust
// Element Condition Examples
pub struct ElementConditionExamples;

impl ElementConditionExamples {
    // Fire mastery condition
    pub fn create_fire_mastery_condition() -> ConditionDefinition {
        ConditionDefinition::new(
            "fire_mastery_condition".to_string(),
            "get_actor_value".to_string(),
            ConditionOperator::GreaterThanOrEqual,
            ConditionValue::Float(100.0), // At least 100 fire mastery
        )
        .with_parameters(vec![ConditionParameter::String("fire_mastery".to_string())])
        .with_categories(vec!["element".to_string(), "fire".to_string(), "mastery".to_string()])
        .with_priority(100)
    }
    
    // Water resistance condition
    pub fn create_water_resistance_condition() -> ConditionDefinition {
        ConditionDefinition::new(
            "water_resistance_condition".to_string(),
            "get_actor_value".to_string(),
            ConditionOperator::LessThan,
            ConditionValue::Float(0.3), // Below 30% water resistance
        )
        .with_parameters(vec![ConditionParameter::String("water_resistance".to_string())])
        .with_categories(vec!["element".to_string(), "water".to_string(), "resistance".to_string()])
        .with_priority(90)
    }
    
    // Element interaction condition
    pub fn create_element_interaction_condition() -> ConditionDefinition {
        ConditionDefinition::new(
            "element_interaction_condition".to_string(),
            "get_element_interaction".to_string(),
            ConditionOperator::Equal,
            ConditionValue::String("suppress".to_string()), // Fire suppresses Water
        )
        .with_parameters(vec![
            ConditionParameter::String("fire".to_string()),
            ConditionParameter::String("water".to_string()),
        ])
        .with_categories(vec!["element".to_string(), "interaction".to_string()])
        .with_priority(80)
    }
}
```

## 🔧 **Effect Core Integration**

### **1. Effect Condition Evaluation**

```rust
// Effect Core Integration
pub struct EffectCoreIntegration {
    condition_core: Arc<ConditionCore>,
    effect_engine: Arc<EffectEngine>,
    effect_cache: Arc<EffectCache>,
}

impl EffectCoreIntegration {
    // Evaluate effect application conditions
    pub async fn evaluate_effect_application_conditions(
        &self,
        effect: &Effect,
        context: &EffectContext
    ) -> Result<bool, EffectError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition in &effect.application_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Evaluate effect duration conditions
    pub async fn evaluate_effect_duration_conditions(
        &self,
        effect: &Effect,
        context: &EffectContext
    ) -> Result<bool, EffectError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition in &effect.duration_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Evaluate effect chain conditions
    pub async fn evaluate_effect_chain_conditions(
        &self,
        effect_chain: &EffectChain,
        context: &EffectContext
    ) -> Result<bool, EffectError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition in &effect_chain.chain_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Convert EffectContext to ConditionContext
    fn convert_to_condition_context(&self, effect_context: &EffectContext) -> ConditionContext {
        ConditionContext {
            target: ActorTarget { id: effect_context.target_id.clone() },
            world_id: effect_context.world_id.clone(),
            current_time: effect_context.current_time,
            current_weather: effect_context.current_weather,
            world_state: effect_context.world_state.clone(),
            effect_chain: Some(effect_context.effect_chain.clone()),
        }
    }
}
```

## 🔧 **Future Systems Integration**

### **1. Talent Core Integration**

```rust
// Talent Core Integration (Future)
pub struct TalentCoreIntegration {
    condition_core: Arc<ConditionCore>,
    talent_engine: Arc<TalentEngine>,
}

impl TalentCoreIntegration {
    // Evaluate talent activation conditions
    pub async fn evaluate_talent_activation_conditions(
        &self,
        talent: &Talent,
        context: &TalentContext
    ) -> Result<bool, TalentError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition in &talent.activation_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Convert TalentContext to ConditionContext
    fn convert_to_condition_context(&self, talent_context: &TalentContext) -> ConditionContext {
        ConditionContext {
            target: ActorTarget { id: talent_context.actor_id.clone() },
            world_id: talent_context.world_id.clone(),
            current_time: talent_context.current_time,
            current_weather: talent_context.current_weather,
            world_state: talent_context.world_state.clone(),
            talent_level: Some(talent_context.talent_level),
        }
    }
}
```

### **2. Perk Core Integration**

```rust
// Perk Core Integration (Future)
pub struct PerkCoreIntegration {
    condition_core: Arc<ConditionCore>,
    perk_engine: Arc<PerkEngine>,
}

impl PerkCoreIntegration {
    // Evaluate perk activation conditions
    pub async fn evaluate_perk_activation_conditions(
        &self,
        perk: &Perk,
        context: &PerkContext
    ) -> Result<bool, PerkError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition in &perk.activation_conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    // Convert PerkContext to ConditionContext
    fn convert_to_condition_context(&self, perk_context: &PerkContext) -> ConditionContext {
        ConditionContext {
            target: ActorTarget { id: perk_context.actor_id.clone() },
            world_id: perk_context.world_id.clone(),
            current_time: perk_context.current_time,
            current_weather: perk_context.current_weather,
            world_state: perk_context.world_state.clone(),
            perk_level: Some(perk_context.perk_level),
        }
    }
}
```

## 🔧 **Integration Patterns**

### **1. Bridge Pattern**

```rust
// Integration Bridge Pattern
pub trait IntegrationBridge {
    type SourceContext;
    type TargetContext;
    
    fn convert_context(&self, source: &Self::SourceContext) -> TargetContext;
    fn convert_result(&self, result: &ConditionResult) -> Self::SourceContext;
}

// Action Core Bridge
pub struct ActionCoreBridge;

impl IntegrationBridge for ActionCoreBridge {
    type SourceContext = ActionContext;
    type TargetContext = ConditionContext;
    
    fn convert_context(&self, action_context: &ActionContext) -> ConditionContext {
        ConditionContext {
            target: ActorTarget { id: action_context.actor_id.clone() },
            world_id: action_context.world_id.clone(),
            current_time: action_context.current_time,
            current_weather: action_context.current_weather,
            world_state: action_context.world_state.clone(),
        }
    }
    
    fn convert_result(&self, result: &ConditionResult) -> ActionContext {
        // Convert condition result back to action context
        ActionContext::default()
    }
}
```

### **2. Adapter Pattern**

```rust
// Integration Adapter Pattern
pub struct ConditionCoreAdapter<T> {
    condition_core: Arc<ConditionCore>,
    bridge: T,
}

impl<T: IntegrationBridge> ConditionCoreAdapter<T> {
    pub fn new(condition_core: Arc<ConditionCore>, bridge: T) -> Self {
        Self { condition_core, bridge }
    }
    
    pub async fn evaluate_condition_with_context(
        &self,
        condition_id: &str,
        source_context: &T::SourceContext
    ) -> Result<ConditionResult, ConditionError> {
        let condition_context = self.bridge.convert_context(source_context);
        self.condition_core.evaluate_condition(condition_id, &condition_context).await
    }
}
```

## 🎯 **Key Features**

### **1. Seamless Integration**
- ✅ **Context Conversion**: Automatic context conversion between systems
- ✅ **Error Handling**: Proper error propagation and handling
- ✅ **Performance**: Optimized integration with minimal overhead
- ✅ **Caching**: Shared caching across integrated systems

### **2. Flexible Architecture**
- ✅ **Bridge Pattern**: Clean separation between systems
- ✅ **Adapter Pattern**: Easy integration with new systems
- ✅ **Event-Driven**: Event-based communication
- ✅ **Async/Await**: Non-blocking integration

### **3. Future-Proof Design**
- ✅ **Extensible**: Easy to add new system integrations
- ✅ **Backward Compatible**: Maintains compatibility with existing systems
- ✅ **Version Control**: Support for different API versions
- ✅ **Migration Support**: Easy migration between versions

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Integration Design Complete  
**Maintainer**: Chaos World Team
