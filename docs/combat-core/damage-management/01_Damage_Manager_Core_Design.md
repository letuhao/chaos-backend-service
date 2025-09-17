# Damage Manager Core Design

## 📋 **Tổng Quan**

Damage Manager Core Design định nghĩa core components và architecture cho Damage Manager system, bao gồm damage calculation engine, modifier processing, validation, và event handling.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Generic Design**
- **No Hard Coding**: Tránh hard code resource/element/status/action/category/tag
- **Configuration-Driven**: Sử dụng configuration files cho tất cả definitions
- **Extensible Architecture**: Dễ dàng mở rộng cho damage types mới
- **Type Safety**: Strong typing cho tất cả damage operations

### **2. Performance First**
- **High Throughput**: 50,000+ damage calculations/second
- **Low Latency**: < 0.1ms cho single damage calculation
- **Memory Efficient**: < 1KB per damage request
- **Cache Optimized**: Intelligent caching system

### **3. Separation of Concerns**
- **DamageCalculator**: Chỉ tính toán damage
- **ModifierProcessor**: Chỉ xử lý modifiers
- **Validator**: Chỉ validate damage requests
- **EventDispatcher**: Chỉ dispatch events

## 🏗️ **Core Architecture**

### **1. Damage Manager Core**

```rust
/// Damage Manager Core
pub struct DamageManager {
    // Core components
    damage_calculator: Arc<DamageCalculator>,
    modifier_processor: Arc<DamageModifierProcessor>,
    damage_validator: Arc<DamageValidator>,
    event_dispatcher: Arc<DamageEventDispatcher>,
    damage_cache: Arc<DamageCache>,
    
    // Configuration system
    damage_type_registry: Arc<DamageTypeRegistry>,
    modifier_registry: Arc<DamageModifierRegistry>,
    source_registry: Arc<DamageSourceRegistry>,
    condition_registry: Arc<DamageConditionRegistry>,
    
    // Integration bridges
    resource_manager_bridge: Arc<ResourceManagerBridge>,
    element_core_bridge: Arc<ElementCoreBridge>,
    status_core_bridge: Arc<StatusCoreBridge>,
    action_core_bridge: Arc<ActionCoreBridge>,
    
    // Performance components
    batch_processor: Arc<DamageBatchProcessor>,
    metrics_collector: Arc<DamageMetricsCollector>,
    
    // Configuration
    config: DamageManagerConfig,
}

impl DamageManager {
    /// Apply damage to actor
    pub async fn apply_damage(
        &self,
        request: DamageRequest
    ) -> Result<DamageResult, DamageError> {
        // 1. Validate damage request
        self.damage_validator.validate_damage_request(&request).await?;
        
        // 2. Check damage immunity
        if self.check_damage_immunity(&request).await? {
            return Ok(self.create_immunity_result(&request));
        }
        
        // 3. Calculate base damage
        let base_damage = self.damage_calculator.calculate_base_damage(&request).await?;
        
        // 4. Process damage modifiers
        let modified_damage = self.modifier_processor.process_modifiers(
            &request,
            base_damage
        ).await?;
        
        // 5. Apply damage to resources
        let damage_result = self.apply_damage_to_resources(&request, modified_damage).await?;
        
        // 6. Dispatch damage events
        self.event_dispatcher.dispatch_damage_applied(&damage_result).await?;
        
        // 7. Update cache
        self.damage_cache.update_damage_result(&damage_result).await?;
        
        // 8. Collect metrics
        self.metrics_collector.record_damage_application(&damage_result).await?;
        
        Ok(damage_result)
    }
    
    /// Apply damage to resources
    async fn apply_damage_to_resources(
        &self,
        request: &DamageRequest,
        damage: f64
    ) -> Result<DamageResult, DamageError> {
        // Get damage type configuration
        let damage_type_config = self.damage_type_registry.get_damage_type(&request.damage_type).await?;
        
        // Apply damage based on damage type
        let damage_result = match damage_type_config.resource_type {
            ResourceType::HP => {
                self.resource_manager_bridge.apply_hp_damage(
                    &request.actor_id,
                    damage,
                    &request.properties
                ).await?
            },
            ResourceType::MP => {
                self.resource_manager_bridge.apply_mp_damage(
                    &request.actor_id,
                    damage,
                    &request.properties
                ).await?
            },
            ResourceType::Stamina => {
                self.resource_manager_bridge.apply_stamina_damage(
                    &request.actor_id,
                    damage,
                    &request.properties
                ).await?
            },
            ResourceType::Qi => {
                self.resource_manager_bridge.apply_qi_damage(
                    &request.actor_id,
                    damage,
                    &request.properties
                ).await?
            },
            ResourceType::Custom(ref resource_id) => {
                self.resource_manager_bridge.apply_custom_damage(
                    &request.actor_id,
                    resource_id,
                    damage,
                    &request.properties
                ).await?
            },
        };
        
        Ok(damage_result)
    }
    
    /// Check damage immunity
    async fn check_damage_immunity(
        &self,
        request: &DamageRequest
    ) -> Result<bool, DamageError> {
        // Check status core for immunity
        if let Some(status_immunity) = self.status_core_bridge.check_damage_immunity(
            &request.actor_id,
            &request.damage_type,
            &request.damage_source
        ).await? {
            return Ok(status_immunity);
        }
        
        // Check element core for immunity
        if let Some(element_immunity) = self.element_core_bridge.check_damage_immunity(
            &request.actor_id,
            &request.damage_type,
            &request.element_id
        ).await? {
            return Ok(element_immunity);
        }
        
        // Check action core for immunity
        if let Some(action_immunity) = self.action_core_bridge.check_damage_immunity(
            &request.actor_id,
            &request.damage_type,
            &request.source_id
        ).await? {
            return Ok(action_immunity);
        }
        
        Ok(false)
    }
}
```

### **2. Damage Calculator**

```rust
/// Damage Calculator
pub struct DamageCalculator {
    // Configuration
    calculation_config: DamageCalculationConfig,
    
    // Integration bridges
    element_core_bridge: Arc<ElementCoreBridge>,
    status_core_bridge: Arc<StatusCoreBridge>,
    action_core_bridge: Arc<ActionCoreBridge>,
    actor_core_bridge: Arc<ActorCoreBridge>,
    
    // Cache
    calculation_cache: Arc<DamageCalculationCache>,
}

impl DamageCalculator {
    /// Calculate base damage
    pub async fn calculate_base_damage(
        &self,
        request: &DamageRequest
    ) -> Result<f64, DamageError> {
        // Check cache first
        if let Some(cached_damage) = self.calculation_cache.get_damage(request).await? {
            return Ok(cached_damage);
        }
        
        // Calculate damage based on source
        let base_damage = match request.damage_source {
            DamageSource::Direct => {
                self.calculate_direct_damage(request).await?
            },
            DamageSource::Status => {
                self.calculate_status_damage(request).await?
            },
            DamageSource::Elemental => {
                self.calculate_elemental_damage(request).await?
            },
            DamageSource::Action => {
                self.calculate_action_damage(request).await?
            },
            DamageSource::Environmental => {
                self.calculate_environmental_damage(request).await?
            },
            DamageSource::Custom(ref source_id) => {
                self.calculate_custom_damage(request, source_id).await?
            },
        };
        
        // Cache result
        self.calculation_cache.cache_damage(request, base_damage).await?;
        
        Ok(base_damage)
    }
    
    /// Calculate direct damage
    async fn calculate_direct_damage(
        &self,
        request: &DamageRequest
    ) -> Result<f64, DamageError> {
        // Get actor stats
        let actor_stats = self.actor_core_bridge.get_derived_stats(&request.actor_id).await?;
        
        // Calculate base damage from actor stats
        let base_damage = self.calculate_damage_from_stats(
            &actor_stats,
            &request.damage_type,
            &request.properties
        ).await?;
        
        Ok(base_damage)
    }
    
    /// Calculate status damage
    async fn calculate_status_damage(
        &self,
        request: &DamageRequest
    ) -> Result<f64, DamageError> {
        // Get status effect data
        let status_data = self.status_core_bridge.get_status_damage_data(
            &request.actor_id,
            &request.damage_type,
            &request.properties
        ).await?;
        
        // Calculate damage from status data
        let base_damage = self.calculate_damage_from_status_data(
            &status_data,
            &request.damage_type,
            &request.properties
        ).await?;
        
        Ok(base_damage)
    }
    
    /// Calculate elemental damage
    async fn calculate_elemental_damage(
        &self,
        request: &DamageRequest
    ) -> Result<f64, DamageError> {
        // Get element data
        let element_data = self.element_core_bridge.get_element_damage_data(
            &request.actor_id,
            &request.element_id.as_ref().unwrap(),
            &request.damage_type,
            &request.properties
        ).await?;
        
        // Calculate damage from element data
        let base_damage = self.calculate_damage_from_element_data(
            &element_data,
            &request.damage_type,
            &request.properties
        ).await?;
        
        Ok(base_damage)
    }
    
    /// Calculate action damage
    async fn calculate_action_damage(
        &self,
        request: &DamageRequest
    ) -> Result<f64, DamageError> {
        // Get action data
        let action_data = self.action_core_bridge.get_action_damage_data(
            &request.actor_id,
            &request.source_id.as_ref().unwrap(),
            &request.damage_type,
            &request.properties
        ).await?;
        
        // Calculate damage from action data
        let base_damage = self.calculate_damage_from_action_data(
            &action_data,
            &request.damage_type,
            &request.properties
        ).await?;
        
        Ok(base_damage)
    }
}
```

### **3. Damage Modifier Processor**

```rust
/// Damage Modifier Processor
pub struct DamageModifierProcessor {
    // Modifier registry
    modifier_registry: Arc<DamageModifierRegistry>,
    
    // Integration bridges
    element_core_bridge: Arc<ElementCoreBridge>,
    status_core_bridge: Arc<StatusCoreBridge>,
    action_core_bridge: Arc<ActionCoreBridge>,
    actor_core_bridge: Arc<ActorCoreBridge>,
    
    // Cache
    modifier_cache: Arc<DamageModifierCache>,
}

impl DamageModifierProcessor {
    /// Process damage modifiers
    pub async fn process_modifiers(
        &self,
        request: &DamageRequest,
        base_damage: f64
    ) -> Result<f64, DamageError> {
        // Check cache first
        if let Some(cached_damage) = self.modifier_cache.get_modified_damage(request, base_damage).await? {
            return Ok(cached_damage);
        }
        
        let mut final_damage = base_damage;
        let mut applied_modifiers = Vec::new();
        
        // Process request modifiers
        for modifier in &request.modifiers {
            if self.should_apply_modifier(modifier, request).await? {
                let modified_damage = self.apply_modifier(final_damage, modifier).await?;
                final_damage = modified_damage;
                applied_modifiers.push(modifier.clone());
            }
        }
        
        // Process system modifiers
        let system_modifiers = self.get_system_modifiers(request).await?;
        for modifier in system_modifiers {
            if self.should_apply_modifier(&modifier, request).await? {
                let modified_damage = self.apply_modifier(final_damage, &modifier).await?;
                final_damage = modified_damage;
                applied_modifiers.push(modifier);
            }
        }
        
        // Cache result
        self.modifier_cache.cache_modified_damage(request, base_damage, final_damage).await?;
        
        Ok(final_damage)
    }
    
    /// Apply individual modifier
    async fn apply_modifier(
        &self,
        damage: f64,
        modifier: &DamageModifier
    ) -> Result<f64, DamageError> {
        match modifier.modifier_type {
            DamageModifierType::Multiplier => Ok(damage * modifier.value),
            DamageModifierType::Addition => Ok(damage + modifier.value),
            DamageModifierType::Reduction => Ok((damage - modifier.value).max(0.0)),
            DamageModifierType::Resistance => {
                let resistance = modifier.value.min(1.0); // Cap at 100%
                Ok(damage * (1.0 - resistance))
            },
            DamageModifierType::Immunity => Ok(0.0), // No damage
            DamageModifierType::Absorption => Ok(-damage), // Convert to healing
            DamageModifierType::Reflection => Ok(damage), // Keep original damage
            DamageModifierType::Custom(ref modifier_id) => {
                self.apply_custom_modifier(damage, modifier_id, modifier).await?
            },
        }
    }
    
    /// Get system modifiers
    async fn get_system_modifiers(
        &self,
        request: &DamageRequest
    ) -> Result<Vec<DamageModifier>, DamageError> {
        let mut system_modifiers = Vec::new();
        
        // Get element modifiers
        if let Some(element_id) = &request.element_id {
            let element_modifiers = self.element_core_bridge.get_damage_modifiers(
                &request.actor_id,
                element_id,
                &request.damage_type
            ).await?;
            system_modifiers.extend(element_modifiers);
        }
        
        // Get status modifiers
        let status_modifiers = self.status_core_bridge.get_damage_modifiers(
            &request.actor_id,
            &request.damage_type
        ).await?;
        system_modifiers.extend(status_modifiers);
        
        // Get action modifiers
        if let Some(source_id) = &request.source_id {
            let action_modifiers = self.action_core_bridge.get_damage_modifiers(
                &request.actor_id,
                source_id,
                &request.damage_type
            ).await?;
            system_modifiers.extend(action_modifiers);
        }
        
        Ok(system_modifiers)
    }
}
```

### **4. Damage Validator**

```rust
/// Damage Validator
pub struct DamageValidator {
    // Configuration
    validation_config: DamageValidationConfig,
    
    // Integration bridges
    actor_core_bridge: Arc<ActorCoreBridge>,
    resource_manager_bridge: Arc<ResourceManagerBridge>,
}

impl DamageValidator {
    /// Validate damage request
    pub async fn validate_damage_request(
        &self,
        request: &DamageRequest
    ) -> Result<(), DamageError> {
        // Validate actor exists
        if !self.actor_core_bridge.actor_exists(&request.actor_id).await? {
            return Err(DamageError::ActorNotFound(request.actor_id.clone()));
        }
        
        // Validate damage type
        if !self.is_valid_damage_type(&request.damage_type).await? {
            return Err(DamageError::InvalidDamageType(request.damage_type.clone()));
        }
        
        // Validate damage source
        if !self.is_valid_damage_source(&request.damage_source).await? {
            return Err(DamageError::InvalidDamageSource(request.damage_source.clone()));
        }
        
        // Validate damage value
        if request.base_damage < 0.0 {
            return Err(DamageError::InvalidDamageValue(request.base_damage));
        }
        
        // Validate modifiers
        for modifier in &request.modifiers {
            self.validate_modifier(modifier).await?;
        }
        
        // Validate properties
        self.validate_properties(&request.properties).await?;
        
        Ok(())
    }
    
    /// Validate modifier
    async fn validate_modifier(
        &self,
        modifier: &DamageModifier
    ) -> Result<(), DamageError> {
        // Validate modifier type
        if !self.is_valid_modifier_type(&modifier.modifier_type).await? {
            return Err(DamageError::InvalidModifierType(modifier.modifier_type.clone()));
        }
        
        // Validate modifier value
        if modifier.value < 0.0 {
            return Err(DamageError::InvalidModifierValue(modifier.value));
        }
        
        // Validate modifier condition
        if let Some(condition) = &modifier.condition {
            self.validate_modifier_condition(condition).await?;
        }
        
        Ok(())
    }
}
```

### **5. Damage Event Dispatcher**

```rust
/// Damage Event Dispatcher
pub struct DamageEventDispatcher {
    // Event subscribers
    event_subscribers: HashMap<String, Vec<Box<dyn DamageEventSubscriber>>>,
    
    // Event queue
    event_queue: Arc<Mutex<VecDeque<DamageEvent>>>,
    
    // Configuration
    config: DamageEventConfig,
}

impl DamageEventDispatcher {
    /// Dispatch damage applied event
    pub async fn dispatch_damage_applied(
        &self,
        result: &DamageResult
    ) -> Result<(), DamageError> {
        let event = DamageEvent {
            event_type: DamageEventType::DamageApplied,
            actor_id: result.actor_id.clone(),
            damage_type: result.damage_type.clone(),
            damage_amount: result.damage_applied,
            timestamp: SystemTime::now(),
            data: serde_json::to_value(result).unwrap_or_default(),
        };
        
        // Add to event queue
        {
            let mut queue = self.event_queue.lock().unwrap();
            queue.push_back(event.clone());
        }
        
        // Notify subscribers
        self.notify_subscribers(&event).await?;
        
        Ok(())
    }
    
    /// Notify subscribers
    async fn notify_subscribers(
        &self,
        event: &DamageEvent
    ) -> Result<(), DamageError> {
        let subscribers = self.event_subscribers.get(&event.event_type.to_string())
            .cloned()
            .unwrap_or_default();
        
        for subscriber in subscribers {
            if let Err(e) = subscriber.handle_damage_event(event).await {
                // Log error but don't fail the entire operation
                tracing::error!("Error notifying damage event subscriber: {}", e);
            }
        }
        
        Ok(())
    }
}
```

## 🔧 **Core Types**

### **1. Damage Request**

```rust
/// Damage Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageRequest {
    pub actor_id: String,
    pub damage_type: DamageType,
    pub base_damage: f64,
    pub damage_source: DamageSource,
    pub element_id: Option<String>,
    pub source_id: Option<String>,
    pub modifiers: Vec<DamageModifier>,
    pub properties: HashMap<String, serde_json::Value>,
    pub context: DamageContext,
}

/// Damage Type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    HP,           // Health Points
    MP,           // Mana Points
    Stamina,      // Stamina
    Qi,           // Qi/Energy
    Armor,        // Armor durability
    Weapon,       // Weapon durability
    Custom(String), // Custom damage type
}

/// Damage Source
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageSource {
    Direct,       // Direct damage (weapon, spell)
    Status,       // Status effect damage
    Elemental,    // Elemental damage
    Action,       // Action-based damage
    Environmental, // Environmental damage
    Fall,         // Fall damage
    Poison,       // Poison damage
    Custom(String), // Custom damage source
}

/// Damage Modifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageModifier {
    pub modifier_type: DamageModifierType,
    pub value: f64,
    pub source: String,
    pub condition: Option<DamageCondition>,
    pub properties: HashMap<String, serde_json::Value>,
}

/// Damage Modifier Type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageModifierType {
    Multiplier,   // Multiply damage by value
    Addition,     // Add value to damage
    Reduction,    // Reduce damage by value
    Resistance,   // Resistance to damage type
    Immunity,     // Immunity to damage type
    Absorption,   // Absorb damage as healing
    Reflection,   // Reflect damage back to source
    Custom(String), // Custom modifier type
}

/// Damage Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageResult {
    pub actor_id: String,
    pub damage_type: DamageType,
    pub base_damage: f64,
    pub final_damage: f64,
    pub damage_applied: f64,
    pub damage_blocked: f64,
    pub immunity_applied: bool,
    pub modifiers_applied: Vec<DamageModifier>,
    pub events_triggered: Vec<DamageEvent>,
    pub timestamp: SystemTime,
}

/// Damage Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageEvent {
    pub event_type: DamageEventType,
    pub actor_id: String,
    pub damage_type: DamageType,
    pub damage_amount: f64,
    pub timestamp: SystemTime,
    pub data: serde_json::Value,
}

/// Damage Event Type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageEventType {
    DamageApplied,
    DamageBlocked,
    DamageAbsorbed,
    DamageReflected,
    DamageImmunity,
    DamageCritical,
    DamageMiss,
}

/// Damage Context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageContext {
    pub combat_id: Option<String>,
    pub attacker_id: Option<String>,
    pub target_id: Option<String>,
    pub environment: String,
    pub time: SystemTime,
    pub additional_data: HashMap<String, serde_json::Value>,
}
```

## 🚀 **Performance Optimization**

### **1. Damage Cache**

```rust
/// Damage Cache
pub struct DamageCache {
    // Calculation cache
    calculation_cache: HashMap<String, f64>,
    
    // Modifier cache
    modifier_cache: HashMap<String, f64>,
    
    // Result cache
    result_cache: HashMap<String, DamageResult>,
    
    // Cache configuration
    config: DamageCacheConfig,
}

impl DamageCache {
    /// Get cached damage calculation
    pub async fn get_damage_calculation(
        &self,
        request: &DamageRequest
    ) -> Result<Option<f64>, DamageError> {
        let cache_key = self.generate_cache_key(request);
        Ok(self.calculation_cache.get(&cache_key).copied())
    }
    
    /// Cache damage calculation
    pub async fn cache_damage_calculation(
        &mut self,
        request: &DamageRequest,
        damage: f64
    ) -> Result<(), DamageError> {
        let cache_key = self.generate_cache_key(request);
        self.calculation_cache.insert(cache_key, damage);
        Ok(())
    }
    
    /// Generate cache key
    fn generate_cache_key(&self, request: &DamageRequest) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            request.actor_id,
            request.damage_type,
            request.damage_source,
            request.base_damage,
            request.element_id.as_deref().unwrap_or("none")
        )
    }
}
```

### **2. Batch Processing**

```rust
/// Damage Batch Processor
pub struct DamageBatchProcessor {
    // Batch configuration
    config: DamageBatchConfig,
    
    // Processing queue
    processing_queue: Arc<Mutex<VecDeque<DamageRequest>>>,
    
    // Results queue
    results_queue: Arc<Mutex<VecDeque<DamageResult>>>,
}

impl DamageBatchProcessor {
    /// Process damage batch
    pub async fn process_damage_batch(
        &self,
        requests: Vec<DamageRequest>
    ) -> Result<Vec<DamageResult>, DamageError> {
        // Group requests by actor for efficient processing
        let mut grouped_requests: HashMap<String, Vec<DamageRequest>> = HashMap::new();
        for request in requests {
            grouped_requests
                .entry(request.actor_id.clone())
                .or_insert_with(Vec::new)
                .push(request);
        }
        
        // Process each actor's requests
        let mut results = Vec::new();
        for (actor_id, actor_requests) in grouped_requests {
            let actor_results = self.process_actor_damage_batch(actor_id, actor_requests).await?;
            results.extend(actor_results);
        }
        
        Ok(results)
    }
    
    /// Process actor damage batch
    async fn process_actor_damage_batch(
        &self,
        actor_id: String,
        requests: Vec<DamageRequest>
    ) -> Result<Vec<DamageResult>, DamageError> {
        // Process requests in parallel
        let mut handles = Vec::new();
        for request in requests {
            let processor = self.clone();
            let handle = tokio::spawn(async move {
                processor.process_single_damage(request).await
            });
            handles.push(handle);
        }
        
        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            let result = handle.await??;
            results.push(result);
        }
        
        Ok(results)
    }
}
```

## 📝 **Implementation Notes**

### **1. Key Benefits**
- **Generic Design**: No hard coding, highly extensible
- **High Performance**: 50,000+ calculations/second
- **Memory Efficient**: < 1KB per request
- **Easy Integration**: Seamless integration with other systems

### **2. Configuration-Driven**
- **Damage Types**: Defined in configuration files
- **Modifiers**: Defined in configuration files
- **Sources**: Defined in configuration files
- **Conditions**: Defined in configuration files

### **3. Extensibility**
- **Custom Damage Types**: Easy to add new types
- **Custom Modifiers**: Easy to add new modifiers
- **Custom Sources**: Easy to add new sources
- **Plugin Architecture**: Dynamic loading support

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
