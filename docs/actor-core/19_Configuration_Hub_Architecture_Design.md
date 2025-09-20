# Configuration Hub Architecture Design

## 🎯 **OBJECTIVE**

Design a configuration system that follows the Actor Core hub pattern, allowing multiple subsystems to register configurations with merge/override/aggregate logic, similar to how stats and resources are handled.

## 🏗️ **CURRENT ARCHITECTURE ANALYSIS**

### **Actor Core Hub Pattern**
- **PluginRegistry**: Manages multiple subsystems with priority-based ordering
- **CombinerRegistry**: Handles merge rules for stat aggregation (Sum, Max, Min, Average, Multiply, Intersect)
- **CapLayerRegistry**: Manages cap layers with across-layer policies
- **Aggregator**: Processes contributions from all subsystems and applies merge rules

### **Configuration System Should Follow Same Pattern**
- **ConfigurationRegistry**: Manage multiple configuration providers with priority
- **ConfigurationCombiner**: Handle merge rules for configuration values
- **ConfigurationAggregator**: Process configurations from all providers and apply merge rules

## 🔧 **PROPOSED CONFIGURATION HUB ARCHITECTURE**

### **1. Configuration Provider Trait**
```rust
/// Trait for subsystems to provide configuration data
#[async_trait]
pub trait ConfigurationProvider {
    /// Unique identifier for this configuration provider
    fn provider_id(&self) -> &str;
    
    /// Priority for this provider (higher = more important)
    fn priority(&self) -> i64;
    
    /// Get configuration categories this provider supports
    fn get_supported_categories(&self) -> Vec<String>;
    
    /// Get configuration value for a specific key
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>>;
    
    /// Get all configuration values for a category
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>>;
    
    /// Get merge rule for a specific configuration key
    fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule>;
    
    /// Validate configuration data
    async fn validate_config(&self) -> ActorCoreResult<()>;
}
```

### **2. Configuration Value Types**
```rust
/// Configuration value with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationValue {
    /// The actual value
    pub value: serde_json::Value,
    /// Data type of the value
    pub value_type: ConfigurationValueType,
    /// Source provider ID
    pub source_provider: String,
    /// Priority of this value
    pub priority: i64,
    /// Timestamp when this value was set
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Whether this value can be overridden
    pub can_override: bool,
    /// Whether this value can be merged
    pub can_merge: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigurationValueType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
    Duration,
    Size,
    Percentage,
}
```

### **3. Configuration Merge Rules**
```rust
/// Merge rule for configuration values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationMergeRule {
    /// Merge strategy to use
    pub strategy: ConfigurationMergeStrategy,
    /// Whether to use pipeline processing
    pub use_pipeline: bool,
    /// Default value if no providers have this config
    pub default_value: Option<serde_json::Value>,
    /// Validation rules for the merged value
    pub validation_rules: Vec<ConfigurationValidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigurationMergeStrategy {
    /// Use the highest priority value (override)
    Override,
    /// Use the lowest priority value (baseline)
    Baseline,
    /// Sum all values (for numeric values)
    Sum,
    /// Take the maximum value
    Max,
    /// Take the minimum value
    Min,
    /// Calculate average (for numeric values)
    Average,
    /// Multiply all values (for numeric values)
    Multiply,
    /// Intersect ranges (for range values)
    Intersect,
    /// Merge objects/arrays
    Merge,
    /// Concatenate strings
    Concat,
}
```

### **4. Configuration Registry**
```rust
/// Registry for managing configuration providers
#[async_trait]
pub trait ConfigurationRegistry {
    /// Register a configuration provider
    async fn register_provider(&self, provider: Arc<dyn ConfigurationProvider>) -> ActorCoreResult<()>;
    
    /// Unregister a configuration provider
    async fn unregister_provider(&self, provider_id: &str) -> ActorCoreResult<()>;
    
    /// Get configuration provider by ID
    async fn get_provider(&self, provider_id: &str) -> Option<Arc<dyn ConfigurationProvider>>;
    
    /// Get all providers sorted by priority
    async fn get_providers_by_priority(&self) -> Vec<Arc<dyn ConfigurationProvider>>;
    
    /// Get providers for a specific category
    async fn get_providers_for_category(&self, category: &str) -> Vec<Arc<dyn ConfigurationProvider>>;
    
    /// Validate all registered providers
    async fn validate_all_providers(&self) -> ActorCoreResult<()>;
}
```

### **5. Configuration Combiner**
```rust
/// Combiner for merging configuration values
#[async_trait]
pub trait ConfigurationCombiner {
    /// Get merge rule for a specific configuration key
    async fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule>;
    
    /// Set merge rule for a specific configuration key
    async fn set_merge_rule(&self, category: &str, key: &str, rule: ConfigurationMergeRule) -> ActorCoreResult<()>;
    
    /// Merge configuration values from multiple providers
    async fn merge_values(
        &self,
        category: &str,
        key: &str,
        values: Vec<ConfigurationValue>,
    ) -> ActorCoreResult<ConfigurationValue>;
    
    /// Validate merged configuration
    async fn validate_merged_config(&self, config: &ConfigurationValue) -> ActorCoreResult<()>;
}
```

### **6. Configuration Aggregator**
```rust
/// Aggregator for processing configurations from all providers
#[async_trait]
pub trait ConfigurationAggregator {
    /// Get configuration value for a specific key
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>>;
    
    /// Get all configuration values for a category
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>>;
    
    /// Get all configuration values
    async fn get_all_config(&self) -> ActorCoreResult<HashMap<String, HashMap<String, ConfigurationValue>>>;
    
    /// Refresh configuration from all providers
    async fn refresh_config(&self) -> ActorCoreResult<()>;
    
    /// Invalidate configuration cache
    async fn invalidate_cache(&self, category: Option<&str>, key: Option<&str>);
}
```

## 🔄 **CONFIGURATION RESOLUTION FLOW**

### **1. Provider Registration**
```rust
// Subsystem registers its configuration provider
let config_provider = MySubsystemConfigProvider::new();
config_registry.register_provider(Arc::new(config_provider)).await?;
```

### **2. Configuration Resolution**
```rust
// Get configuration value (automatically merges from all providers)
let config_value = config_aggregator.get_config_value("performance", "max_memory_per_actor").await?;
```

### **3. Merge Process**
1. **Collect**: Get all providers that support the requested category
2. **Gather**: Collect configuration values from all providers
3. **Sort**: Sort values by priority (highest first)
4. **Merge**: Apply merge rule to combine values
5. **Validate**: Validate the merged result
6. **Cache**: Cache the result for future use

## 📁 **CONFIGURATION FILE STRUCTURE**

### **Base Configuration (`configs/actor_core_config.yaml`)**
```yaml
# Base configuration for Actor Core
actor_core:
  # System-wide settings
  system:
    max_actors: 10000
    cache_ttl: 3600
    log_level: "info"
  
  # Default merge rules
  merge_rules:
    performance:
      max_memory_per_actor:
        strategy: "min"  # Use minimum value for safety
        use_pipeline: false
        default_value: 1000000
      max_cache_size:
        strategy: "sum"  # Sum all cache sizes
        use_pipeline: true
        default_value: 1000
    dimensions:
      strength:
        strategy: "override"  # Last provider wins
        use_pipeline: false
        default_value: 0.0
      health:
        strategy: "max"  # Use maximum health
        use_pipeline: true
        default_value: 100.0
```

### **Subsystem Configuration (`configs/subsystems/rpg_subsystem.yaml`)**
```yaml
# RPG Subsystem Configuration
subsystem_id: "rpg_subsystem"
priority: 100
categories:
  performance:
    max_memory_per_actor: 500000
    max_cache_size: 500
  dimensions:
    strength: 10.0
    agility: 10.0
    intelligence: 10.0
    vitality: 10.0
    spirit: 10.0
    luck: 10.0
    health: 100.0
    mana: 100.0
    stamina: 100.0
    experience: 0.0
    level: 1.0
  derived_dimensions:
    attack_power: 0.0
    defense_power: 0.0
    critical_hit_chance: 0.0
    critical_hit_damage: 0.0
    attack_speed: 1.0
    movement_speed: 1.0
    casting_speed: 1.0
    cooldown_reduction: 0.0
    life_steal: 0.0
    mana_steal: 0.0
    damage_reduction: 0.0
    elemental_resistance: 0.0
```

### **Subsystem Configuration (`configs/subsystems/magic_subsystem.yaml`)**
```yaml
# Magic Subsystem Configuration
subsystem_id: "magic_subsystem"
priority: 200  # Higher priority than RPG
categories:
  performance:
    max_memory_per_actor: 1000000  # Higher memory for magic
    max_cache_size: 1000
  dimensions:
    mana: 200.0  # Override RPG mana
    intelligence: 15.0  # Override RPG intelligence
  derived_dimensions:
    casting_speed: 1.5  # Override RPG casting speed
    elemental_resistance: 10.0  # Add elemental resistance
```

## 🔧 **IMPLEMENTATION STRATEGY**

### **Phase 1: Core Infrastructure**
1. **Create Configuration Provider Trait**
2. **Implement Configuration Value Types**
3. **Create Configuration Merge Rules**
4. **Implement Configuration Registry**

### **Phase 2: Combiner and Aggregator**
1. **Implement Configuration Combiner**
2. **Implement Configuration Aggregator**
3. **Add caching and validation**

### **Phase 3: Subsystem Integration**
1. **Create example configuration providers**
2. **Update existing subsystems to use configuration providers**
3. **Migrate hardcoded constants to configuration providers**

### **Phase 4: Migration**
1. **Update all hardcoded constants to use configuration aggregator**
2. **Remove constants.rs hardcoded modules**
3. **Test and validate the new system**

## ✅ **BENEFITS**

### **1. Hub Architecture Consistency**
- **Same pattern as stats/resources**: Multiple providers, priority-based resolution
- **Familiar interface**: Similar to existing Actor Core patterns
- **Consistent behavior**: Merge rules, validation, caching

### **2. Subsystem Independence**
- **Each subsystem manages its own configuration**
- **No hardcoded dependencies**
- **Easy to add/remove subsystems**

### **3. Flexible Configuration**
- **Multiple configuration sources**: Files, databases, environment variables
- **Runtime configuration updates**: No restart required
- **Priority-based resolution**: Higher priority subsystems can override lower priority ones

### **4. Merge/Override/Aggregate Logic**
- **Override**: Last provider wins (for settings)
- **Sum**: Add all values (for limits)
- **Max/Min**: Use extreme values (for safety)
- **Merge**: Combine objects/arrays (for complex configs)

### **5. Validation and Safety**
- **Type validation**: Ensure correct data types
- **Range validation**: Ensure values are within acceptable ranges
- **Dependency validation**: Ensure required configurations are present

## 🎯 **EXPECTED OUTCOME**

After implementation:
- **✅ Zero hardcoded constants** in Actor Core
- **✅ Subsystem-driven configuration** with priority-based resolution
- **✅ Flexible merge/override/aggregate logic** for different configuration types
- **✅ Runtime configuration updates** without code changes
- **✅ Consistent hub architecture** following Actor Core patterns
- **✅ Easy subsystem integration** for configuration management

This design follows the Actor Core hub pattern perfectly, allowing multiple subsystems to register configurations with the same merge/override/aggregate logic used for stats and resources.
