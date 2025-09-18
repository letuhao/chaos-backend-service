# Condition Core - Actor Core Integration Design

## 📋 **Tổng Quan**

Tài liệu này thiết kế chi tiết cách Condition Core tích hợp với Actor Core để cung cấp conditional logic cho actor stat aggregation, subsystem activation, và resource management.

## 🎯 **Integration Goals**

### **1. Primary Objectives**
- ✅ **Conditional Subsystem Activation** - Enable/disable subsystems based on conditions
- ✅ **Resource Validation** - Validate actor resources before stat aggregation
- ✅ **Dynamic Stat Modifiers** - Apply conditional stat modifications
- ✅ **Event-Driven Conditions** - React to actor state changes
- ✅ **Performance Optimization** - Cache condition results for performance

### **2. Secondary Objectives**
- ✅ **Plugin Architecture** - Seamless integration with existing Actor Core plugins
- ✅ **Backward Compatibility** - No breaking changes to existing Actor Core API
- ✅ **Type Safety** - Strong typing for condition evaluation
- ✅ **Error Handling** - Proper error propagation and handling

## 🏗️ **Integration Architecture**

### **1. High-Level Integration Flow**

```
Actor Core + Condition Core Integration
├── Actor Creation
│   ├── Load Actor Data
│   ├── Initialize Condition Resolver
│   └── Register Data Providers
├── Stat Aggregation
│   ├── Pre-Aggregation Conditions
│   │   ├── Resource Validation
│   │   ├── Subsystem Activation
│   │   └── Stat Modifiers
│   ├── Core Aggregation
│   │   ├── Subsystem Contributions
│   │   ├── Cap Application
│   │   └── Final Stats
│   └── Post-Aggregation Conditions
│       ├── Stat Validation
│       ├── Event Triggers
│       └── Cache Updates
└── Event Handling
    ├── Actor State Changes
    ├── Condition Re-evaluation
    └── Cache Invalidation
```

### **2. Component Integration**

```rust
// Actor Core + Condition Core Integration
pub struct ActorCoreWithConditions {
    // Actor Core components
    aggregator: Arc<dyn Aggregator>,
    cache: Arc<dyn Cache>,
    plugin_registry: Arc<PluginRegistry>,
    
    // Condition Core components
    condition_resolver: Arc<ConditionResolver>,
    data_provider_registry: Arc<DataProviderRegistry>,
    
    // Integration components
    condition_cache: Arc<ConditionCache>,
    event_handler: Arc<ConditionEventHandler>,
}
```

## 🔧 **Data Provider Integration**

### **1. Actor Data Provider Implementation**

```rust
use condition_core::*;
use actor_core::*;

/// Actor data provider for Condition Core
pub struct ActorDataProvider {
    actor_repository: Arc<dyn ActorRepository>,
    stat_cache: Arc<dyn StatCache>,
}

#[async_trait::async_trait]
impl ActorDataProvider for ActorDataProvider {
    // Generic resource value - không hard-code specific resources
    async fn get_actor_resource(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64> {
        let actor = self.actor_repository.get_actor(actor_id).await?;
        let snapshot = self.stat_cache.get_snapshot(actor_id).await?;
        
        // Lấy resource value từ snapshot, fallback về 0.0 nếu không có
        Ok(snapshot.primary.get(resource_type).unwrap_or(&0.0).clone())
    }
    
    // Generic stat value - không hard-code specific stats
    async fn get_actor_stat(&self, stat_name: &str, actor_id: &str) -> ConditionResult<f64> {
        let actor = self.actor_repository.get_actor(actor_id).await?;
        let snapshot = self.stat_cache.get_snapshot(actor_id).await?;
        
        // Lấy stat value từ snapshot, fallback về 0.0 nếu không có
        Ok(snapshot.primary.get(stat_name).unwrap_or(&0.0).clone())
    }
    
    // Generic derived stat value
    async fn get_actor_derived_stat(&self, stat_name: &str, actor_id: &str) -> ConditionResult<f64> {
        let actor = self.actor_repository.get_actor(actor_id).await?;
        let snapshot = self.stat_cache.get_snapshot(actor_id).await?;
        
        // Lấy derived stat value từ snapshot, fallback về 0.0 nếu không có
        Ok(snapshot.derived.get(stat_name).unwrap_or(&0.0).clone())
    }
    
    // Actor metadata - không phụ thuộc vào specific cultivation system
    async fn get_actor_race(&self, actor_id: &str) -> ConditionResult<String> {
        let actor = self.actor_repository.get_actor(actor_id).await?;
        Ok(actor.race.clone())
    }
    
    // Combat state - generic state check
    async fn is_actor_in_combat(&self, actor_id: &str) -> ConditionResult<bool> {
        let actor = self.actor_repository.get_actor(actor_id).await?;
        Ok(actor.is_in_combat())
    }
    
    // Generic status effects - không hard-code "buffs"
    async fn has_actor_status_effects(&self, status_type: &str, actor_id: &str) -> ConditionResult<bool> {
        let actor = self.actor_repository.get_actor(actor_id).await?;
        
        // Check trong actor.data với key tương ứng với status_type
        let has_status = actor.data.get(status_type)
            .and_then(|v| v.as_array())
            .map(|statuses| !statuses.is_empty())
            .unwrap_or(false);
        
        Ok(has_status)
    }
    
    // Generic status effect count
    async fn get_actor_status_effect_count(&self, status_type: &str, actor_id: &str) -> ConditionResult<i64> {
        let actor = self.actor_repository.get_actor(actor_id).await?;
        
        let count = actor.data.get(status_type)
            .and_then(|v| v.as_array())
            .map(|statuses| statuses.len())
            .unwrap_or(0);
        
        Ok(count as i64)
    }
    
    // Generic status effect count by category/tag
    async fn get_actor_status_effect_count_by_category(&self, status_type: &str, category: &str, actor_id: &str) -> ConditionResult<i64> {
        let actor = self.actor_repository.get_actor(actor_id).await?;
        
        let count = actor.data.get(status_type)
            .and_then(|v| v.as_array())
            .map(|statuses| {
                statuses.iter()
                    .filter(|status| {
                        status.get("category")
                            .and_then(|c| c.as_str())
                            .map(|c| c == category)
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0);
        
        Ok(count as i64)
    }
}
```

### **2. Resource Data Provider Implementation**

```rust
/// Resource data provider for Condition Core
pub struct ResourceDataProvider {
    resource_manager: Arc<dyn ResourceManager>,
    stat_cache: Arc<dyn StatCache>,
}

#[async_trait::async_trait]
impl ResourceDataProvider for ResourceDataProvider {
    async fn get_resource_value(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await?;
        Ok(resource.current_value)
    }
    
    async fn get_resource_max(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await?;
        Ok(resource.max_value)
    }
    
    async fn get_resource_percentage(&self, resource_type: &str, actor_id: &str) -> ConditionResult<f64> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await?;
        if resource.max_value > 0.0 {
            Ok(resource.current_value / resource.max_value)
        } else {
            Ok(0.0)
        }
    }
    
    
    async fn is_resource_empty(&self, resource_type: &str, actor_id: &str) -> ConditionResult<bool> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await?;
        Ok(resource.current_value <= 0.0)
    }
    
    // Generic resource state check với custom threshold
    async fn is_resource_below_threshold(&self, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await?;
        Ok(resource.current_value < threshold)
    }
    
    // Generic resource state check với custom threshold (above)
    async fn is_resource_above_threshold(&self, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await?;
        Ok(resource.current_value > threshold)
    }
    
    // Generic resource state check với percentage threshold (below)
    async fn is_resource_below_percentage(&self, resource_type: &str, percentage: f64, actor_id: &str) -> ConditionResult<bool> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await?;
        if resource.max_value > 0.0 {
            let threshold = resource.max_value * (percentage / 100.0);
            Ok(resource.current_value < threshold)
        } else {
            Ok(false)
        }
    }
    
    // Generic resource state check với percentage threshold (above)
    async fn is_resource_above_percentage(&self, resource_type: &str, percentage: f64, actor_id: &str) -> ConditionResult<bool> {
        let resource = self.resource_manager.get_resource(actor_id, resource_type).await?;
        if resource.max_value > 0.0 {
            let threshold = resource.max_value * (percentage / 100.0);
            Ok(resource.current_value > threshold)
        } else {
            Ok(false)
        }
    }
}
```

### **3. Category Data Provider Implementation**

```rust
/// Category data provider for Condition Core
pub struct CategoryDataProvider {
    category_registry: Arc<dyn CategoryRegistry>,
    actor_inventory: Arc<dyn ActorInventory>,
}

#[async_trait::async_trait]
impl CategoryDataProvider for CategoryDataProvider {
    async fn has_category_item(&self, category: &str, actor_id: &str) -> ConditionResult<bool> {
        let items = self.actor_inventory.get_items_by_category(actor_id, category).await?;
        Ok(!items.is_empty())
    }
    
    async fn get_category_item_count(&self, category: &str, actor_id: &str) -> ConditionResult<i64> {
        let items = self.actor_inventory.get_items_by_category(actor_id, category).await?;
        Ok(items.len() as i64)
    }
    
    async fn is_category_available(&self, category: &str, actor_id: &str) -> ConditionResult<bool> {
        let actor = self.actor_repository.get_actor(actor_id).await?;
        let snapshot = self.stat_cache.get_snapshot(actor_id).await?;
        
        // Check if actor has required stats for category
        let required_stats = self.category_registry.get_required_stats(category)?;
        for (stat_name, min_value) in required_stats {
            let actor_value = snapshot.primary.get(stat_name).unwrap_or(&0.0);
            if *actor_value < min_value {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}
```

## 🔌 **Conditional Subsystem Integration**

### **1. Conditional Subsystem Trait**

```rust
use condition_core::*;

/// Trait for subsystems that can be conditionally activated
pub trait ConditionalSubsystem: Subsystem {
    /// Get the condition configuration for this subsystem
    fn get_activation_condition(&self) -> Option<ConditionConfig>;
    
    /// Get the condition resolver for this subsystem
    fn get_condition_resolver(&self) -> Option<Arc<ConditionResolver>>;
    
    /// Check if the subsystem should be activated for the given actor
    async fn should_activate(&self, actor: &Actor) -> ActorCoreResult<bool> {
        if let Some(condition) = self.get_activation_condition() {
            if let Some(resolver) = self.get_condition_resolver() {
                let context = self.create_condition_context(actor).await?;
                let result = resolver.resolve_condition(&condition, &context).await?;
                Ok(result)
            } else {
                Ok(true) // No resolver means always activate
            }
        } else {
            Ok(true) // No condition means always activate
        }
    }
    
    /// Create condition context from actor
    async fn create_condition_context(&self, actor: &Actor) -> ActorCoreResult<ConditionContext> {
        Ok(ConditionContext {
            target: ActorTarget {
                id: actor.id.to_string(),
            },
            world_id: "default".to_string(),
            current_time: std::time::SystemTime::now(),
            current_weather: WeatherType::Clear,
            world_state: WorldState {
                time_of_day: 12.0,
                season: "spring".to_string(),
                temperature: 20.0,
                humidity: 0.5,
            },
        })
    }
}
```

### **2. Conditional Subsystem Implementation**

```rust
/// Example conditional subsystem
pub struct ConditionalResourceSubsystem {
    system_id: String,
    priority: i64,
    activation_condition: Option<ConditionConfig>,
    condition_resolver: Option<Arc<ConditionResolver>>,
    resource_manager: Arc<dyn ResourceManager>,
}

impl ConditionalSubsystem for ConditionalResourceSubsystem {
    fn get_activation_condition(&self) -> Option<ConditionConfig> {
        self.activation_condition.clone()
    }
    
    fn get_condition_resolver(&self) -> Option<Arc<ConditionResolver>> {
        self.condition_resolver.clone()
    }
}

#[async_trait::async_trait]
impl Subsystem for ConditionalResourceSubsystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn priority(&self) -> i64 {
        self.priority
    }
    
    async fn contribute(&self, actor: &Actor) -> ActorCoreResult<SubsystemOutput> {
        // Check if subsystem should be activated
        if !self.should_activate(actor).await? {
            return Ok(SubsystemOutput::empty());
        }
        
        // Normal subsystem contribution logic
        let resources = self.resource_manager.get_actor_resources(&actor.id).await?;
        let mut primary = Vec::new();
        let mut derived = Vec::new();
        let mut caps = Vec::new();
        
        for resource in resources {
            primary.push(Contribution {
                dimension: resource.name.clone(),
                bucket: Bucket::FLAT,
                value: resource.current_value,
                system: self.system_id.clone(),
                priority: Some(self.priority),
            });
            
            caps.push(CapContribution {
                system: self.system_id.clone(),
                dimension: resource.name.clone(),
                mode: CapMode::HARD_MAX,
                kind: CapKind::Max,
                value: resource.max_value,
                priority: Some(self.priority),
                scope: Some("TOTAL".to_string()),
                realm: None,
                tags: None,
            });
        }
        
        Ok(SubsystemOutput {
            primary,
            derived,
            caps,
            context: None,
            meta: SubsystemMeta {
                system: self.system_id.clone(),
                stage: None,
                version: Some(1),
            },
        })
    }
}
```

## 🎯 **Conditional Stat Modifiers**

### **1. Conditional Modifier System**

```rust
/// Conditional stat modifier system
pub struct ConditionalModifierSystem {
    condition_resolver: Arc<ConditionResolver>,
    modifier_registry: Arc<ModifierRegistry>,
    cache: Arc<dyn Cache>,
}

impl ConditionalModifierSystem {
    /// Apply conditional modifiers to actor stats
    pub async fn apply_modifiers(
        &self,
        actor: &Actor,
        snapshot: &mut Snapshot,
    ) -> ActorCoreResult<()> {
        let context = self.create_condition_context(actor).await?;
        let modifiers = self.modifier_registry.get_modifiers_for_actor(actor).await?;
        
        for modifier in modifiers {
            if let Some(condition) = &modifier.condition {
                let should_apply = self.condition_resolver
                    .resolve_condition(condition, &context)
                    .await?;
                
                if should_apply {
                    self.apply_modifier(snapshot, &modifier).await?;
                }
            } else {
                // No condition means always apply
                self.apply_modifier(snapshot, &modifier).await?;
            }
        }
        
        Ok(())
    }
    
    /// Apply a single modifier to snapshot
    async fn apply_modifier(
        &self,
        snapshot: &mut Snapshot,
        modifier: &ConditionalModifier,
    ) -> ActorCoreResult<()> {
        match modifier.modifier_type {
            ModifierType::Additive => {
                if let Some(value) = snapshot.primary.get_mut(&modifier.dimension) {
                    *value += modifier.value;
                }
            }
            ModifierType::Multiplicative => {
                if let Some(value) = snapshot.primary.get_mut(&modifier.dimension) {
                    *value *= modifier.value;
                }
            }
            ModifierType::Override => {
                snapshot.primary.insert(modifier.dimension.clone(), modifier.value);
            }
        }
        Ok(())
    }
}
```

### **2. Conditional Modifier Configuration**

```yaml
# Conditional modifier configuration
modifiers:
  - id: "low_health_boost"
    dimension: "strength"
    modifier_type: "additive"
    value: 10.0
    condition:
      condition_id: "low_health_check"
      function_name: "get_actor_resource"
      operator: "LessThan"
      value: !Float 25.0
      parameters:
        - !String "health"
  
  - id: "combat_damage_boost"
    dimension: "damage"
    modifier_type: "multiplicative"
    value: 1.5
    condition:
      condition_id: "in_combat_check"
      function_name: "is_actor_in_combat"
      operator: "Equal"
      value: !Boolean true
      parameters: []
  
  - id: "buff_stack_bonus"
    dimension: "crit_rate"
    modifier_type: "additive"
    value: 5.0
    condition:
      condition_id: "multiple_buffs_check"
      function_name: "get_actor_buff_count"
      operator: "GreaterThan"
      value: !Integer 3
      parameters: []
```

## 🚀 **Integration Examples**

### **1. Basic Integration Setup**

```rust
use condition_core::*;
use actor_core::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Actor Core components
    let cache = ServiceFactory::create_cache()?;
    let plugin_registry = ServiceFactory::create_plugin_registry();
    let combiner_registry = ServiceFactory::create_combiner_registry();
    let cap_layer_registry = ServiceFactory::create_cap_layer_registry();
    let caps_provider = ServiceFactory::create_caps_provider(cap_layer_registry);
    
    // Create Condition Core data providers
    let mut data_registry = DataProviderRegistry::new();
    data_registry.register_actor_provider(Box::new(ActorDataProvider::new(
        actor_repository.clone(),
        stat_cache.clone(),
    )));
    data_registry.register_resource_provider(Box::new(ResourceDataProvider::new(
        resource_manager.clone(),
        stat_cache.clone(),
    )));
    data_registry.register_category_provider(Box::new(CategoryDataProvider::new(
        category_registry.clone(),
        actor_inventory.clone(),
    )));
    
    // Create condition resolver
    let condition_resolver = ConditionResolver::new(data_registry);
    
    // Create Actor Core aggregator
    let aggregator = ServiceFactory::create_aggregator(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    
    // Create integrated system
    let integrated_system = ActorCoreWithConditions::new(
        aggregator,
        condition_resolver,
        cache,
    );
    
    // Create actor with conditions
    let mut actor = Actor::new("player1".to_string(), "human".to_string());
    
    // Add conditional subsystem
    let conditional_subsystem = ConditionalResourceSubsystem::new(
        "resource_system".to_string(),
        100,
        Some(create_health_condition()),
        Some(Arc::new(condition_resolver)),
        resource_manager,
    );
    actor.add_subsystem(conditional_subsystem);
    
    // Resolve actor stats with conditions
    let snapshot = integrated_system.resolve_with_conditions(&actor).await?;
    
    println!("Actor stats with conditions: {:?}", snapshot);
    Ok(())
}

fn create_health_condition() -> ConditionConfig {
    ConditionConfig {
        condition_id: "health_check".to_string(),
        function_name: "get_actor_resource".to_string(),
        operator: ConditionOperator::GreaterThan,
        value: ConditionValue::Float(50.0),
        parameters: vec![ConditionParameter::String("health".to_string())],
    }
}
```

### **2. Advanced Conditional Logic**

```rust
/// Advanced conditional logic example
pub struct AdvancedConditionalSystem {
    condition_resolver: Arc<ConditionResolver>,
    modifier_system: Arc<ConditionalModifierSystem>,
    event_handler: Arc<ConditionEventHandler>,
}

impl AdvancedConditionalSystem {
    /// Process actor with complex conditional logic
    pub async fn process_actor(&self, actor: &Actor) -> ActorCoreResult<Snapshot> {
        // Pre-aggregation conditions
        self.validate_pre_conditions(actor).await?;
        
        // Apply conditional subsystems
        let mut snapshot = self.apply_conditional_subsystems(actor).await?;
        
        // Apply conditional modifiers
        self.modifier_system.apply_modifiers(actor, &mut snapshot).await?;
        
        // Post-aggregation conditions
        self.validate_post_conditions(actor, &snapshot).await?;
        
        // Trigger events
        self.event_handler.handle_actor_update(actor, &snapshot).await?;
        
        Ok(snapshot)
    }
    
    /// Validate pre-aggregation conditions
    async fn validate_pre_conditions(&self, actor: &Actor) -> ActorCoreResult<()> {
        let context = self.create_condition_context(actor).await?;
        
        // Check if actor has minimum health
        let health_condition = ConditionConfig {
            condition_id: "min_health".to_string(),
            function_name: "get_actor_resource".to_string(),
            operator: ConditionOperator::GreaterThan,
            value: ConditionValue::Float(1.0),
            parameters: vec![ConditionParameter::String("health".to_string())],
        };
        
        let has_min_health = self.condition_resolver
            .resolve_condition(&health_condition, &context)
            .await?;
        
        if !has_min_health {
            return Err(ActorCoreError::ValidationError(
                "Actor does not have minimum health".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Apply conditional subsystems
    async fn apply_conditional_subsystems(&self, actor: &Actor) -> ActorCoreResult<Snapshot> {
        let mut snapshot = Snapshot::default();
        
        for subsystem in &actor.subsystems {
            if let Some(conditional) = subsystem.as_conditional() {
                if conditional.should_activate(actor).await? {
                    let output = conditional.contribute(actor).await?;
                    self.merge_subsystem_output(&mut snapshot, &output)?;
                }
            } else {
                // Non-conditional subsystem
                let output = subsystem.contribute(actor).await?;
                self.merge_subsystem_output(&mut snapshot, &output)?;
            }
        }
        
        Ok(snapshot)
    }
}
```

## 📊 **Performance Considerations**

### **1. Caching Strategy**

```rust
/// Condition cache for performance optimization
pub struct ConditionCache {
    cache: Arc<dyn Cache>,
    ttl: Duration,
}

impl ConditionCache {
    /// Get cached condition result
    pub async fn get_condition_result(
        &self,
        condition_id: &str,
        actor_id: &str,
    ) -> Option<bool> {
        let key = format!("condition:{}:{}", condition_id, actor_id);
        self.cache.get(&key).await.ok().flatten()
    }
    
    /// Cache condition result
    pub async fn cache_condition_result(
        &self,
        condition_id: &str,
        actor_id: &str,
        result: bool,
    ) -> ActorCoreResult<()> {
        let key = format!("condition:{}:{}", condition_id, actor_id);
        self.cache.set_with_ttl(&key, result, self.ttl).await?;
        Ok(())
    }
    
    /// Invalidate condition cache for actor
    pub async fn invalidate_actor_cache(&self, actor_id: &str) -> ActorCoreResult<()> {
        let pattern = format!("condition:*:{}", actor_id);
        self.cache.delete_pattern(&pattern).await?;
        Ok(())
    }
}
```

### **2. Batch Processing**

```rust
/// Batch condition evaluation for performance
pub struct BatchConditionEvaluator {
    condition_resolver: Arc<ConditionResolver>,
    cache: Arc<ConditionCache>,
}

impl BatchConditionEvaluator {
    /// Evaluate multiple conditions in batch
    pub async fn evaluate_batch(
        &self,
        conditions: &[ConditionConfig],
        actors: &[Actor],
    ) -> ActorCoreResult<HashMap<String, HashMap<String, bool>>> {
        let mut results = HashMap::new();
        
        for actor in actors {
            let mut actor_results = HashMap::new();
            
            for condition in conditions {
                let cache_key = format!("{}:{}", condition.condition_id, actor.id);
                
                if let Some(cached_result) = self.cache.get_condition_result(
                    &condition.condition_id,
                    &actor.id.to_string(),
                ).await {
                    actor_results.insert(condition.condition_id.clone(), cached_result);
                } else {
                    let context = self.create_condition_context(actor).await?;
                    let result = self.condition_resolver
                        .resolve_condition(condition, &context)
                        .await?;
                    
                    self.cache.cache_condition_result(
                        &condition.condition_id,
                        &actor.id.to_string(),
                        result,
                    ).await?;
                    
                    actor_results.insert(condition.condition_id.clone(), result);
                }
            }
            
            results.insert(actor.id.to_string(), actor_results);
        }
        
        Ok(results)
    }
}
```

## 🔧 **Configuration Examples**

### **1. Actor Core Configuration with Conditions**

```yaml
# Actor Core configuration with Condition Core integration
actor_core:
  condition_integration:
    enabled: true
    cache_ttl: 300  # 5 minutes
    batch_size: 100
    
  conditional_subsystems:
    - id: "resource_system"
      condition:
        condition_id: "health_check"
        function_name: "get_actor_resource"
        operator: "GreaterThan"
        value: !Float 50.0
        parameters:
          - !String "health"
    
    - id: "combat_system"
      condition:
        condition_id: "in_combat_check"
        function_name: "is_actor_in_combat"
        operator: "Equal"
        value: !Boolean true
        parameters: []
  
  conditional_modifiers:
    - id: "low_health_boost"
      dimension: "strength"
      modifier_type: "additive"
      value: 10.0
      condition:
        condition_id: "low_health_check"
        function_name: "is_resource_below_percentage"
        operator: "Equal"
        value: !Boolean true
        parameters:
          - !String "health"
          - !Float 25.0
```

### **2. Condition Core Configuration**

```yaml
# Condition Core configuration for Actor Core integration
condition_core:
  data_providers:
    actor:
      enabled: true
      cache_ttl: 60
    resource:
      enabled: true
      cache_ttl: 30
    category:
      enabled: true
      cache_ttl: 120
  
  functions:
    - name: "get_actor_resource"
      provider: "actor"
      cache_ttl: 30
    - name: "is_actor_in_combat"
      provider: "actor"
      cache_ttl: 10
    - name: "is_resource_below_percentage"
      provider: "resource"
      cache_ttl: 30
    - name: "is_resource_above_percentage"
      provider: "resource"
      cache_ttl: 30
    - name: "is_resource_below_threshold"
      provider: "resource"
      cache_ttl: 30
    - name: "is_resource_above_threshold"
      provider: "resource"
      cache_ttl: 30
    - name: "has_category_item"
      provider: "category"
      cache_ttl: 60
```

## 🎯 **Integration Benefits**

### **1. Enhanced Actor Core Capabilities**
- ✅ **Conditional Logic** - Rich conditional logic for actor behavior
- ✅ **Dynamic Subsystems** - Enable/disable subsystems based on conditions
- ✅ **Resource Validation** - Validate resources before stat aggregation
- ✅ **Event-Driven Updates** - React to actor state changes

### **2. Performance Benefits**
- ✅ **Caching** - Cache condition results for performance
- ✅ **Batch Processing** - Evaluate multiple conditions efficiently
- ✅ **Lazy Evaluation** - Only evaluate conditions when needed
- ✅ **Memory Optimization** - Efficient memory usage

### **3. Developer Experience**
- ✅ **Type Safety** - Strong typing for condition evaluation
- ✅ **IDE Support** - Auto-completion and error checking
- ✅ **Debugging** - Clear error messages and debugging tools
- ✅ **Testing** - Easy to test conditional logic

## 📝 **Implementation Checklist**

### **Phase 1: Basic Integration**
- [ ] Implement Actor Data Provider
- [ ] Implement Resource Data Provider
- [ ] Implement Category Data Provider
- [ ] Create Condition Resolver integration
- [ ] Add basic conditional subsystem support

### **Phase 2: Advanced Features**
- [ ] Implement conditional modifiers
- [ ] Add event-driven condition updates
- [ ] Implement caching system
- [ ] Add batch processing
- [ ] Create configuration system

### **Phase 3: Optimization**
- [ ] Performance optimization
- [ ] Memory optimization
- [ ] Cache optimization
- [ ] Monitoring and metrics
- [ ] Documentation and examples

## 🚀 **Next Steps**

1. **Implement Data Providers** - Create actual data providers for Actor Core
2. **Add Conditional Subsystems** - Implement conditional subsystem support
3. **Create Integration Examples** - Build comprehensive examples
4. **Performance Testing** - Test performance with real data
5. **Documentation** - Complete integration documentation

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
