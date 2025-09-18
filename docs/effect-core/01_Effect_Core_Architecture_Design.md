# Effect Core Architecture Design

## 📋 **Tổng Quan**

Tài liệu này thiết kế kiến trúc chi tiết cho Effect Core, dựa trên Skyrim's Magic Effects system và các best practices từ game industry.

## 🏗️ **Core Architecture**

### **1. Effect Core Components**

```rust
// Effect Core Main Structure
pub struct EffectCore {
    // Core components
    effect_registry: EffectRegistry,
    condition_system: ConditionSystem,
    effect_engine: EffectEngine,
    effect_interfaces: EffectInterfaces,
    effect_integration: EffectIntegration,
    
    // Configuration
    config_manager: EffectConfigManager,
    
    // Performance
    effect_cache: EffectCache,
    condition_cache: ConditionCache,
    
    // Monitoring
    effect_monitor: EffectMonitor,
    performance_monitor: PerformanceMonitor,
}

// Effect Registry
pub struct EffectRegistry {
    effect_types: HashMap<String, EffectType>,
    effect_categories: HashMap<String, EffectCategory>,
    effect_definitions: HashMap<String, EffectDefinition>,
    effect_validation_rules: Vec<ValidationRule>,
}

// Condition System
pub struct ConditionSystem {
    condition_functions: HashMap<String, ConditionFunction>,
    condition_evaluator: ConditionEvaluator,
    condition_cache: ConditionCache,
    condition_validator: ConditionValidator,
}

// Effect Engine
pub struct EffectEngine {
    effect_calculator: EffectCalculator,
    effect_processor: EffectProcessor,
    effect_scheduler: EffectScheduler,
    effect_monitor: EffectMonitor,
}
```

### **2. Effect Definition Structure**

```rust
// Effect Definition (Skyrim-inspired)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectDefinition {
    // Basic Info (Skyrim-inspired)
    pub effect_guid: String,           // GUID for uniqueness
    pub effect_id: String,             // Editor ID like Skyrim
    pub effect_name: String,           // Display name
    pub effect_name_vi: String,        // Vietnamese name
    pub world_id: String,              // World identifier
    
    // Effect Properties
    pub effect_type: EffectType,       // Type of effect
    pub effect_archetype: EffectArchetype, // Effect archetype
    pub effect_delivery: EffectDelivery,   // How effect is delivered
    pub effect_duration: EffectDuration,   // Duration type
    pub effect_magnitude: EffectMagnitude, // Magnitude type
    
    // Categories
    pub categories: Vec<String>,       // Effect categories
    
    // Complex Conditions (Skyrim-inspired)
    pub conditions: Vec<EffectCondition>, // Complex conditions
    
    // Effect Data
    pub effect_data: EffectData,       // Effect-specific data
    
    // Visual/Audio Effects
    pub visual_effects: Vec<VisualEffect>,
    pub audio_effects: Vec<AudioEffect>,
    
    // Metadata
    pub priority: u32,                 // Effect priority
    pub stackable: bool,               // Can stack with same effect
    pub max_stacks: u32,               // Maximum stacks
    pub stack_behavior: StackBehavior, // How stacks behave
    
    // Timestamps
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

// Effect Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Damage(DamageEffectType),
    Healing(HealingEffectType),
    Buff(BuffEffectType),
    Debuff(DebuffEffectType),
    Status(StatusEffectType),
    Movement(MovementEffectType),
    Environmental(EnvironmentalEffectType),
    Custom(CustomEffectType),
}

// Effect Archetypes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectArchetype {
    ElementalDamage,
    PhysicalDamage,
    MagicalDamage,
    TrueDamage,
    HealthHealing,
    StaminaHealing,
    ManaHealing,
    StatBuff,
    SpeedBuff,
    DefenseBuff,
    StatDebuff,
    SpeedDebuff,
    DefenseDebuff,
    StatusEffect,
    MovementEffect,
    EnvironmentalEffect,
}

// Effect Delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectDelivery {
    TargetActor,        // Target specific actor
    TargetLocation,     // Target specific location
    TargetArea,         // Target area of effect
    Self,              // Apply to self
    Caster,            // Apply to caster
    Custom(CustomDelivery),
}
```

### **3. Condition System Architecture**

```rust
// Condition System (Skyrim-inspired)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectCondition {
    pub condition_id: String,
    pub condition_function: String,    // Function name
    pub condition_parameter: Option<String>, // Parameter for function
    pub condition_operator: ConditionOperator,
    pub condition_value: ConditionValue,
    pub condition_logic: ConditionLogic, // AND, OR, NOT
    pub condition_priority: u32,       // Evaluation priority
}

// Condition Functions (Skyrim-inspired)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionFunction {
    // Actor Conditions
    GetActorValue(String),             // Get actor stat value
    GetLevel,                          // Get actor level
    GetRace,                           // Get actor race
    GetSex,                            // Get actor sex
    IsInCombat,                        // Is actor in combat
    IsDead,                            // Is actor dead
    IsUnconscious,                     // Is actor unconscious
    
    // Item Conditions
    HasItem(String),                   // Has specific item
    GetItemCount(String),              // Get item count
    IsEquipped(String),                // Is item equipped
    GetItemCharge(String),             // Get item charge
    
    // Location Conditions
    GetInCurrentLocation(String),      // In specific location
    GetInCurrentLocType(String),       // In location type
    IsInInterior,                      // Is in interior
    IsInWater,                         // Is in water
    GetDistanceFromPlayer,             // Distance from player
    
    // Time Conditions
    GetCurrentTime,                    // Current time
    GetDayOfWeek,                      // Day of week
    GetSeason,                         // Current season
    IsDay,                             // Is day time
    IsNight,                           // Is night time
    
    // Weather Conditions
    GetCurrentWeather,                 // Current weather
    IsRaining,                         // Is raining
    IsSnowing,                         // Is snowing
    IsStorming,                        // Is storming
    
    // Magic Conditions
    HasEffect(String),                 // Has specific effect
    GetEffectMagnitude(String),        // Get effect magnitude
    HasSpell(String),                  // Has specific spell
    GetSpellCount(String),             // Get spell count
    
    // Relationship Conditions
    GetRelationshipRank(String),       // Get relationship rank
    IsHostileToActor(String),          // Is hostile to actor
    IsFriendlyToActor(String),         // Is friendly to actor
    IsNeutralToActor(String),          // Is neutral to actor
    
    // Custom Conditions
    GetGlobalValue(String),            // Get global value
    GetQuestCompleted(String),         // Is quest completed
    GetQuestStage(String),             // Get quest stage
    GetFactionRank(String),            // Get faction rank
    IsInFaction(String),               // Is in faction
}

// Condition Operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equal,                             // ==
    NotEqual,                          // !=
    GreaterThan,                       // >
    GreaterThanOrEqual,                // >=
    LessThan,                          // <
    LessThanOrEqual,                   // <=
    Contains,                          // Contains
    NotContains,                       // Not contains
    StartsWith,                        // Starts with
    EndsWith,                          // Ends with
    Regex,                             // Regex match
}

// Condition Logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionLogic {
    And,                               // AND
    Or,                                // OR
    Not,                               // NOT
    Xor,                               // XOR
    Nand,                              // NAND
    Nor,                               // NOR
}

// Condition Value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Vector3(Vector3),
    Color(Color),
    Time(Time),
    Date(Date),
    Custom(serde_json::Value),
}
```

### **4. Effect Processing Pipeline**

```rust
// Effect Processing Pipeline
pub struct EffectProcessor {
    effect_calculator: EffectCalculator,
    effect_validator: EffectValidator,
    effect_applier: EffectApplier,
    effect_monitor: EffectMonitor,
}

impl EffectProcessor {
    // Process effect request
    pub async fn process_effect(
        &self,
        effect_request: EffectRequest
    ) -> Result<EffectResult, EffectError> {
        // 1. Validate effect request
        self.validate_effect_request(&effect_request).await?;
        
        // 2. Evaluate conditions
        let condition_result = self.evaluate_conditions(&effect_request).await?;
        if !condition_result.passed {
            return Ok(EffectResult::ConditionFailed(condition_result));
        }
        
        // 3. Calculate effect values
        let calculated_effect = self.calculate_effect(&effect_request).await?;
        
        // 4. Apply effect
        let application_result = self.apply_effect(&calculated_effect).await?;
        
        // 5. Monitor effect
        self.monitor_effect(&application_result).await?;
        
        Ok(EffectResult::Success(application_result))
    }
    
    // Evaluate conditions
    async fn evaluate_conditions(
        &self,
        effect_request: &EffectRequest
    ) -> Result<ConditionResult, EffectError> {
        let mut condition_results = Vec::new();
        
        for condition in &effect_request.conditions {
            let result = self.evaluate_condition(condition, effect_request).await?;
            condition_results.push(result);
        }
        
        // Apply condition logic
        let final_result = self.apply_condition_logic(condition_results).await?;
        
        Ok(final_result)
    }
    
    // Calculate effect values
    async fn calculate_effect(
        &self,
        effect_request: &EffectRequest
    ) -> Result<CalculatedEffect, EffectError> {
        let base_effect = effect_request.effect_definition.clone();
        
        // Apply scaling
        let scaled_effect = self.apply_scaling(&base_effect, effect_request).await?;
        
        // Apply modifiers
        let modified_effect = self.apply_modifiers(&scaled_effect, effect_request).await?;
        
        // Apply resistances
        let final_effect = self.apply_resistances(&modified_effect, effect_request).await?;
        
        Ok(final_effect)
    }
}
```

## 🔧 **Effect Interfaces**

### **1. Action Effect Interface**

```rust
// Action Effect Interface
pub struct ActionEffectInterface {
    effect_core: Arc<EffectCore>,
    action_core_client: ActionCoreClient,
}

impl ActionEffectInterface {
    // Apply effects from action
    pub async fn apply_action_effects(
        &self,
        action: &Action,
        target: &Target
    ) -> Result<Vec<EffectResult>, EffectError> {
        let effects = self.extract_effects_from_action(action);
        let mut results = Vec::new();
        
        for effect in effects {
            let effect_request = EffectRequest {
                effect_definition: effect,
                target: target.clone(),
                caster: action.caster.clone(),
                context: action.context.clone(),
                conditions: effect.conditions.clone(),
            };
            
            let result = self.effect_core.process_effect(effect_request).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    // Extract effects from action
    fn extract_effects_from_action(&self, action: &Action) -> Vec<EffectDefinition> {
        let mut effects = Vec::new();
        
        // Extract direct effects
        for effect_id in &action.effect_ids {
            if let Some(effect) = self.effect_core.get_effect(effect_id) {
                effects.push(effect);
            }
        }
        
        // Extract effects from action type
        if let Some(action_type_effects) = self.get_action_type_effects(&action.action_type) {
            effects.extend(action_type_effects);
        }
        
        effects
    }
}
```

### **2. Status Effect Interface**

```rust
// Status Effect Interface
pub struct StatusEffectInterface {
    effect_core: Arc<EffectCore>,
    status_core_client: StatusCoreClient,
}

impl StatusEffectInterface {
    // Apply status effects
    pub async fn apply_status_effects(
        &self,
        status_effects: &[StatusEffect],
        target: &Target
    ) -> Result<Vec<EffectResult>, EffectError> {
        let mut results = Vec::new();
        
        for status_effect in status_effects {
            let effect_request = EffectRequest {
                effect_definition: status_effect.effect_definition.clone(),
                target: target.clone(),
                caster: status_effect.caster.clone(),
                context: status_effect.context.clone(),
                conditions: status_effect.conditions.clone(),
            };
            
            let result = self.effect_core.process_effect(effect_request).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    // Process status effect tick
    pub async fn process_status_tick(
        &self,
        status_effect: &StatusEffect
    ) -> Result<EffectResult, EffectError> {
        let effect_request = EffectRequest {
            effect_definition: status_effect.effect_definition.clone(),
            target: status_effect.target.clone(),
            caster: status_effect.caster.clone(),
            context: status_effect.context.clone(),
            conditions: status_effect.conditions.clone(),
        };
        
        self.effect_core.process_effect(effect_request).await
    }
}
```

### **3. Element Effect Interface**

```rust
// Element Effect Interface
pub struct ElementEffectInterface {
    effect_core: Arc<EffectCore>,
    element_core_client: ElementCoreClient,
}

impl ElementEffectInterface {
    // Apply element effects
    pub async fn apply_element_effects(
        &self,
        element_id: &str,
        target: &Target,
        context: &EffectContext
    ) -> Result<Vec<EffectResult>, EffectError> {
        let element_effects = self.get_element_effects(element_id);
        let mut results = Vec::new();
        
        for effect in element_effects {
            let effect_request = EffectRequest {
                effect_definition: effect,
                target: target.clone(),
                caster: context.caster.clone(),
                context: context.clone(),
                conditions: effect.conditions.clone(),
            };
            
            let result = self.effect_core.process_effect(effect_request).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    // Get element effects
    fn get_element_effects(&self, element_id: &str) -> Vec<EffectDefinition> {
        let mut effects = Vec::new();
        
        // Get effects by element category
        if let Some(element_effects) = self.effect_core.get_effects_by_category(element_id) {
            effects.extend(element_effects);
        }
        
        // Get effects by element type
        if let Some(element_type_effects) = self.effect_core.get_effects_by_type(element_id) {
            effects.extend(element_type_effects);
        }
        
        effects
    }
}
```

## 🚀 **Integration Bridges**

### **1. Action Core Bridge**

```rust
// Action Core Bridge
pub struct ActionCoreBridge {
    effect_core: Arc<EffectCore>,
    action_effect_interface: ActionEffectInterface,
}

impl ActionCoreBridge {
    // Process action effects
    pub async fn process_action_effects(
        &self,
        action: &Action,
        target: &Target
    ) -> Result<Vec<EffectResult>, EffectError> {
        // Get effects from action
        let effects = self.action_effect_interface.extract_effects_from_action(action);
        
        // Process each effect
        let mut results = Vec::new();
        for effect in effects {
            let result = self.process_single_effect(effect, target, action).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    // Process single effect
    async fn process_single_effect(
        &self,
        effect: EffectDefinition,
        target: &Target,
        action: &Action
    ) -> Result<EffectResult, EffectError> {
        let effect_request = EffectRequest {
            effect_definition: effect,
            target: target.clone(),
            caster: action.caster.clone(),
            context: action.context.clone(),
            conditions: effect.conditions.clone(),
        };
        
        self.effect_core.process_effect(effect_request).await
    }
}
```

### **2. Status Core Bridge**

```rust
// Status Core Bridge
pub struct StatusCoreBridge {
    effect_core: Arc<EffectCore>,
    status_effect_interface: StatusEffectInterface,
}

impl StatusCoreBridge {
    // Process status effects
    pub async fn process_status_effects(
        &self,
        status_effects: &[StatusEffect],
        target: &Target
    ) -> Result<Vec<EffectResult>, EffectError> {
        self.status_effect_interface.apply_status_effects(status_effects, target).await
    }
    
    // Process status tick
    pub async fn process_status_tick(
        &self,
        status_effect: &StatusEffect
    ) -> Result<EffectResult, EffectError> {
        self.status_effect_interface.process_status_tick(status_effect).await
    }
}
```

### **3. Element Core Bridge**

```rust
// Element Core Bridge
pub struct ElementCoreBridge {
    effect_core: Arc<EffectCore>,
    element_effect_interface: ElementEffectInterface,
}

impl ElementCoreBridge {
    // Process element effects
    pub async fn process_element_effects(
        &self,
        element_id: &str,
        target: &Target,
        context: &EffectContext
    ) -> Result<Vec<EffectResult>, EffectError> {
        self.element_effect_interface.apply_element_effects(element_id, target, context).await
    }
    
    // Get element mastery effects
    pub async fn get_element_mastery_effects(
        &self,
        element_id: &str,
        mastery_level: u32
    ) -> Result<Vec<EffectDefinition>, EffectError> {
        let effects = self.element_effect_interface.get_element_effects(element_id);
        
        // Filter by mastery level
        let filtered_effects: Vec<EffectDefinition> = effects
            .into_iter()
            .filter(|effect| {
                self.check_mastery_requirement(effect, mastery_level)
            })
            .collect();
        
        Ok(filtered_effects)
    }
}
```

## 📊 **Performance Considerations**

### **1. Effect Caching**

```rust
// Effect Cache
pub struct EffectCache {
    effect_cache: HashMap<String, EffectDefinition>,
    condition_cache: HashMap<String, ConditionResult>,
    calculation_cache: HashMap<String, CalculatedEffect>,
    cache_ttl: Duration,
}

impl EffectCache {
    // Get cached effect
    pub fn get_effect(&self, effect_id: &str) -> Option<&EffectDefinition> {
        self.effect_cache.get(effect_id)
    }
    
    // Cache effect
    pub fn cache_effect(&mut self, effect_id: String, effect: EffectDefinition) {
        self.effect_cache.insert(effect_id, effect);
    }
    
    // Get cached condition result
    pub fn get_condition_result(&self, condition_key: &str) -> Option<&ConditionResult> {
        self.condition_cache.get(condition_key)
    }
    
    // Cache condition result
    pub fn cache_condition_result(&mut self, condition_key: String, result: ConditionResult) {
        self.condition_cache.insert(condition_key, result);
    }
}
```

### **2. Condition Evaluation Optimization**

```rust
// Condition Evaluator
pub struct ConditionEvaluator {
    condition_cache: ConditionCache,
    evaluation_strategies: HashMap<String, EvaluationStrategy>,
}

impl ConditionEvaluator {
    // Evaluate condition with caching
    pub async fn evaluate_condition(
        &self,
        condition: &EffectCondition,
        context: &EffectContext
    ) -> Result<ConditionResult, EffectError> {
        // Check cache first
        let cache_key = self.generate_cache_key(condition, context);
        if let Some(cached_result) = self.condition_cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }
        
        // Evaluate condition
        let result = self.evaluate_condition_internal(condition, context).await?;
        
        // Cache result
        self.condition_cache.cache(cache_key, result.clone());
        
        Ok(result)
    }
    
    // Generate cache key
    fn generate_cache_key(&self, condition: &EffectCondition, context: &EffectContext) -> String {
        format!("{}:{}:{}:{}", 
            condition.condition_function,
            condition.condition_parameter.as_deref().unwrap_or(""),
            condition.condition_value,
            context.target.id
        )
    }
}
```

## 🎯 **Key Features**

### **1. Skyrim-Inspired Design**
- **Complex Condition System**: Hơn 100 condition functions
- **Editor ID System**: GUID + Editor ID như Skyrim
- **Magic Effect Structure**: Tương tự Skyrim's Magic Effects
- **Plugin Architecture**: Modular plugin system

### **2. Advanced Architecture**
- **Unified Effect Management**: Quản lý tập trung
- **Consistent Interfaces**: Interface thống nhất
- **Centralized Processing**: Xử lý tập trung
- **Cross-System Integration**: Tích hợp với tất cả systems

### **3. Performance Optimization**
- **Effect Caching**: Cache effects và conditions
- **Condition Optimization**: Tối ưu condition evaluation
- **Batch Processing**: Xử lý batch effects
- **Async Processing**: Xử lý async

### **4. Future-Proof Design**
- **Extensible Architecture**: Dễ dàng extend
- **Plugin Support**: Hỗ trợ plugin system
- **Version Control**: Hỗ trợ versioning
- **Migration Support**: Hỗ trợ migration

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Architecture Design Complete  
**Maintainer**: Chaos World Team
