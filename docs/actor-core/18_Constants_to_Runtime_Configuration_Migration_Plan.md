# Constants to Runtime Configuration Migration Plan

## 🎯 **OBJECTIVE**

Migrate all hardcoded constants to a **Configuration Hub Architecture** that follows the Actor Core pattern, allowing multiple subsystems to register configurations with merge/override/aggregate logic, similar to how stats and resources are handled.

## 🏗️ **ARCHITECTURE APPROACH**

### **Configuration Hub Pattern**
- **ConfigurationRegistry**: Manage multiple configuration providers with priority
- **ConfigurationCombiner**: Handle merge rules for configuration values (Override, Sum, Max, Min, Average, Merge)
- **ConfigurationAggregator**: Process configurations from all providers and apply merge rules
- **ConfigurationProvider**: Trait for subsystems to provide configuration data

### **Benefits**
- **Consistent with Actor Core**: Same patterns as PluginRegistry, CombinerRegistry, Aggregator
- **Subsystem Independence**: Each subsystem manages its own configuration
- **Flexible Resolution**: Priority-based merge/override/aggregate logic
- **Runtime Updates**: No restart required for configuration changes

## 📋 **MIGRATION TARGETS**

### **1. REGISTRY SYSTEM - `src/registry.rs`**

#### **Primary Dimensions (Lines 203-213)**
**Current:**
```rust
crate::constants::primary_dimensions::STRENGTH,
crate::constants::primary_dimensions::AGILITY,
crate::constants::primary_dimensions::INTELLIGENCE,
crate::constants::primary_dimensions::VITALITY,
crate::constants::primary_dimensions::SPIRIT,
crate::constants::primary_dimensions::LUCK,
crate::constants::primary_dimensions::HEALTH,
crate::constants::primary_dimensions::MANA,
crate::constants::primary_dimensions::STAMINA,
crate::constants::primary_dimensions::EXPERIENCE,
crate::constants::primary_dimensions::LEVEL,
```

**Migration:** Load from configuration at runtime
**Action:** Replace with `config_manager.get_primary_dimensions().await?`

#### **Clamp Ranges (Lines 220, 245)**
**Current:**
```rust
clamp_default: crate::constants::clamp_ranges::get_range(dimension)
```

**Migration:** Load from configuration at runtime
**Action:** Replace with `config_manager.get_dimension_range(dimension).await?`

#### **Derived Dimensions (Lines 227-238)**
**Current:**
```rust
crate::constants::derived_dimensions::ATTACK_POWER,
crate::constants::derived_dimensions::DEFENSE_POWER,
crate::constants::derived_dimensions::CRITICAL_HIT_CHANCE,
crate::constants::derived_dimensions::CRITICAL_HIT_DAMAGE,
crate::constants::derived_dimensions::ATTACK_SPEED,
crate::constants::derived_dimensions::MOVEMENT_SPEED,
crate::constants::derived_dimensions::CASTING_SPEED,
crate::constants::derived_dimensions::COOLDOWN_REDUCTION,
crate::constants::derived_dimensions::LIFE_STEAL,
crate::constants::derived_dimensions::MANA_STEAL,
crate::constants::derived_dimensions::DAMAGE_REDUCTION,
crate::constants::derived_dimensions::ELEMENTAL_RESISTANCE,
```

**Migration:** Load from configuration at runtime
**Action:** Replace with `config_manager.get_derived_dimensions().await?`

### **2. AGGREGATOR SYSTEM - `src/aggregator/mod.rs`**

#### **Clamp Ranges (Line 321)**
**Current:**
```rust
if let Some((min, max)) = crate::constants::clamp_ranges::get_range(&dimension) {
```

**Migration:** Load from configuration at runtime
**Action:** Replace with `if let Some((min, max)) = config_manager.get_dimension_range(&dimension).await? {`

### **3. PERFORMANCE SYSTEM - `src/performance/profiler.rs`**

#### **Performance Thresholds Import (Line 12)**
**Current:**
```rust
use crate::constants::performance_thresholds;
```

**Migration:** Use configuration manager
**Action:** Remove import, use `config_manager.get_performance_thresholds().await?`

#### **Performance Thresholds Usage (Lines 93-97)**
**Current:**
```rust
max_aggregation_time: performance_thresholds::MAX_AGGREGATION_TIME,
max_cache_time: performance_thresholds::MAX_CACHE_TIME,
max_subsystem_time: performance_thresholds::MAX_SUBSYSTEM_TIME,
max_memory_per_actor: performance_thresholds::MAX_MEMORY_PER_ACTOR,
max_cache_size: performance_thresholds::MAX_CACHE_SIZE,
```

**Migration:** Load from configuration at runtime
**Action:** Replace with values from `config_manager.get_performance_thresholds().await?`

### **4. PERFORMANCE CONFIG - `src/performance/config.rs`**

#### **Performance Thresholds Import (Line 8)**
**Current:**
```rust
use crate::constants::performance_thresholds;
```

**Migration:** Use configuration manager
**Action:** Remove import, use `config_manager.get_performance_thresholds().await?`

#### **Performance Thresholds Usage (Lines 280, 296, 298, 311)**
**Current:**
```rust
max_aggregation_time: performance_thresholds::MAX_AGGREGATION_TIME,
max_operation_time: performance_thresholds::MAX_CACHE_TIME,
max_cache_size: performance_thresholds::MAX_CACHE_SIZE,
max_memory_per_actor: performance_thresholds::MAX_MEMORY_PER_ACTOR,
```

**Migration:** Load from configuration at runtime
**Action:** Replace with values from `config_manager.get_performance_thresholds().await?`

### **5. CAPS PROVIDER - `src/caps_provider.rs`**

#### **All Dimensions (Line 188)**
**Current:**
```rust
crate::constants::all_dimensions().iter().map(|s| s.to_string()).collect()
```

**Migration:** Load from configuration at runtime
**Action:** Replace with `config_manager.get_all_dimensions().await?`

### **6. PRELUDE - `src/prelude.rs`**

#### **System IDs Export (Line 203)**
**Current:**
```rust
pub use crate::constants::system_ids::*;
```

**Migration:** Keep for backward compatibility
**Action:** No change needed - system_ids are still valid constants

## 🔧 **IMPLEMENTATION STRATEGY**

### **Phase 1: Create Configuration Hub Infrastructure**

#### **1. Create Configuration Provider Trait**
Create `src/config/provider.rs`:
```rust
/// Trait for subsystems to provide configuration data
#[async_trait]
pub trait ConfigurationProvider {
    fn provider_id(&self) -> &str;
    fn priority(&self) -> i64;
    fn get_supported_categories(&self) -> Vec<String>;
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>>;
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>>;
    fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule>;
    async fn validate_config(&self) -> ActorCoreResult<()>;
}
```

#### **2. Create Configuration Value Types**
Create `src/config/value.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationValue {
    pub value: serde_json::Value,
    pub value_type: ConfigurationValueType,
    pub source_provider: String,
    pub priority: i64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub can_override: bool,
    pub can_merge: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigurationValueType {
    String, Integer, Float, Boolean, Array, Object, Duration, Size, Percentage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationMergeRule {
    pub strategy: ConfigurationMergeStrategy,
    pub use_pipeline: bool,
    pub default_value: Option<serde_json::Value>,
    pub validation_rules: Vec<ConfigurationValidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigurationMergeStrategy {
    Override, Baseline, Sum, Max, Min, Average, Multiply, Intersect, Merge, Concat,
}
```

#### **3. Create Configuration Registry**
Create `src/config/registry.rs`:
```rust
#[async_trait]
pub trait ConfigurationRegistry {
    async fn register_provider(&self, provider: Arc<dyn ConfigurationProvider>) -> ActorCoreResult<()>;
    async fn unregister_provider(&self, provider_id: &str) -> ActorCoreResult<()>;
    async fn get_provider(&self, provider_id: &str) -> Option<Arc<dyn ConfigurationProvider>>;
    async fn get_providers_by_priority(&self) -> Vec<Arc<dyn ConfigurationProvider>>;
    async fn get_providers_for_category(&self, category: &str) -> Vec<Arc<dyn ConfigurationProvider>>;
    async fn validate_all_providers(&self) -> ActorCoreResult<()>;
}
```

#### **4. Create Configuration Combiner**
Create `src/config/combiner.rs`:
```rust
#[async_trait]
pub trait ConfigurationCombiner {
    async fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule>;
    async fn set_merge_rule(&self, category: &str, key: &str, rule: ConfigurationMergeRule) -> ActorCoreResult<()>;
    async fn merge_values(&self, category: &str, key: &str, values: Vec<ConfigurationValue>) -> ActorCoreResult<ConfigurationValue>;
    async fn validate_merged_config(&self, config: &ConfigurationValue) -> ActorCoreResult<()>;
}
```

#### **5. Create Configuration Aggregator**
Create `src/config/aggregator.rs`:
```rust
#[async_trait]
pub trait ConfigurationAggregator {
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>>;
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>>;
    async fn get_all_config(&self) -> ActorCoreResult<HashMap<String, HashMap<String, ConfigurationValue>>>;
    async fn refresh_config(&self) -> ActorCoreResult<()>;
    async fn invalidate_cache(&self, category: Option<&str>, key: Option<&str>);
}
```

### **Phase 2: Create Subsystem Configuration Providers**

#### **1. Create RPG Subsystem Configuration Provider**
Create `src/config/providers/rpg_provider.rs`:
```rust
pub struct RpgConfigurationProvider {
    provider_id: String,
    priority: i64,
    config_data: HashMap<String, HashMap<String, ConfigurationValue>>,
}

impl ConfigurationProvider for RpgConfigurationProvider {
    fn provider_id(&self) -> &str { &self.provider_id }
    fn priority(&self) -> i64 { self.priority }
    
    fn get_supported_categories(&self) -> Vec<String> {
        vec!["dimensions".to_string(), "performance".to_string()]
    }
    
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>> {
        Ok(self.config_data.get(category)?.get(key).cloned())
    }
    
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        Ok(self.config_data.get(category).cloned().unwrap_or_default())
    }
    
    fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule> {
        match (category, key) {
            ("dimensions", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override,
                use_pipeline: false,
                default_value: None,
                validation_rules: vec![],
            }),
            ("performance", "max_memory_per_actor") => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Min, // Use minimum for safety
                use_pipeline: false,
                default_value: Some(serde_json::Value::Number(500000.into())),
                validation_rules: vec![],
            }),
            _ => None,
        }
    }
}
```

#### **2. Create Magic Subsystem Configuration Provider**
Create `src/config/providers/magic_provider.rs`:
```rust
pub struct MagicConfigurationProvider {
    provider_id: String,
    priority: i64, // Higher priority than RPG
    config_data: HashMap<String, HashMap<String, ConfigurationValue>>,
}

impl ConfigurationProvider for MagicConfigurationProvider {
    // Similar implementation but with higher priority and magic-specific configs
    fn priority(&self) -> i64 { 200 } // Higher than RPG (100)
    
    fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule> {
        match (category, key) {
            ("dimensions", "mana") => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override, // Override RPG mana
                use_pipeline: false,
                default_value: None,
                validation_rules: vec![],
            }),
            ("performance", "max_memory_per_actor") => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Max, // Use maximum for magic
                use_pipeline: false,
                default_value: Some(serde_json::Value::Number(1000000.into())),
                validation_rules: vec![],
            }),
            _ => None,
        }
    }
}
```

#### **3. Create Configuration File Providers**
Create `src/config/providers/file_provider.rs`:
```rust
pub struct FileConfigurationProvider {
    provider_id: String,
    priority: i64,
    file_path: String,
    config_data: HashMap<String, HashMap<String, ConfigurationValue>>,
}

impl FileConfigurationProvider {
    pub async fn load_from_file(&mut self, path: &str) -> ActorCoreResult<()> {
        // Load YAML configuration file
        let content = tokio::fs::read_to_string(path).await?;
        let config: serde_yaml::Value = serde_yaml::from_str(&content)?;
        
        // Parse configuration into internal format
        self.parse_config(config)?;
        Ok(())
    }
}
```

### **Phase 3: Update Target Files to Use Configuration Hub**

#### **1. Update Registry System**
**File:** `src/registry.rs`
**Lines:** 203-213, 220, 227-238, 245

**Before:**
```rust
crate::constants::primary_dimensions::STRENGTH,
```

**After:**
```rust
let config_value = config_aggregator.get_config_value("dimensions", "strength").await?;
config_value.unwrap().value.as_str().unwrap().to_string(),
```

#### **2. Update Aggregator System**
**File:** `src/aggregator/mod.rs`
**Line:** 321

**Before:**
```rust
if let Some((min, max)) = crate::constants::clamp_ranges::get_range(&dimension) {
```

**After:**
```rust
if let Some(config_value) = config_aggregator.get_config_value("dimension_ranges", &dimension).await? {
    if let (Some(min), Some(max)) = (config_value.value.get("min"), config_value.value.get("max")) {
        if let (Some(min_val), Some(max_val)) = (min.as_f64(), max.as_f64()) {
            // Use min_val and max_val
        }
    }
}
```

#### **3. Update Performance System**
**File:** `src/performance/profiler.rs`
**Lines:** 12, 93-97

**Before:**
```rust
use crate::constants::performance_thresholds;
max_aggregation_time: performance_thresholds::MAX_AGGREGATION_TIME,
```

**After:**
```rust
let config_value = config_aggregator.get_config_value("performance", "max_aggregation_time").await?;
max_aggregation_time: config_value.unwrap().value.as_f64().unwrap(),
```

#### **4. Update Performance Config**
**File:** `src/performance/config.rs`
**Lines:** 8, 280, 296, 298, 311

**Before:**
```rust
use crate::constants::performance_thresholds;
max_aggregation_time: performance_thresholds::MAX_AGGREGATION_TIME,
```

**After:**
```rust
let config_value = config_aggregator.get_config_value("performance", "max_aggregation_time").await?;
max_aggregation_time: config_value.unwrap().value.as_f64().unwrap(),
```

#### **5. Update Caps Provider**
**File:** `src/caps_provider.rs`
**Line:** 188

**Before:**
```rust
crate::constants::all_dimensions().iter().map(|s| s.to_string()).collect()
```

**After:**
```rust
let dimensions_config = config_aggregator.get_category_config("dimensions").await?;
dimensions_config.keys().cloned().collect()
```

### **Phase 4: Update Constants.rs**

#### **Remove Hardcoded Modules**
**File:** `src/constants.rs`
**Action:** Remove these modules:
- `primary_dimensions`
- `derived_dimensions`
- `clamp_ranges`
- `performance_thresholds`
- `all_dimensions()` function

#### **Keep System Constants**
**File:** `src/constants.rs`
**Action:** Keep these modules:
- `system_ids` (still valid)
- `error_codes` (still valid)
- `error_types` (still valid)

## ✅ **MIGRATION CHECKLIST**

### **Configuration Hub Infrastructure**
- [ ] Create `ConfigurationProvider` trait in `src/config/provider.rs`
- [ ] Create `ConfigurationValue` types in `src/config/value.rs`
- [ ] Create `ConfigurationRegistry` trait in `src/config/registry.rs`
- [ ] Create `ConfigurationCombiner` trait in `src/config/combiner.rs`
- [ ] Create `ConfigurationAggregator` trait in `src/config/aggregator.rs`
- [ ] Implement concrete implementations for all traits

### **Subsystem Configuration Providers**
- [ ] Create `RpgConfigurationProvider` in `src/config/providers/rpg_provider.rs`
- [ ] Create `MagicConfigurationProvider` in `src/config/providers/magic_provider.rs`
- [ ] Create `FileConfigurationProvider` in `src/config/providers/file_provider.rs`
- [ ] Create example configuration files for each subsystem

### **Target Files Migration**
- [ ] Update `src/registry.rs` (lines 203-213, 220, 227-238, 245)
- [ ] Update `src/aggregator/mod.rs` (line 321)
- [ ] Update `src/performance/profiler.rs` (lines 12, 93-97)
- [ ] Update `src/performance/config.rs` (lines 8, 280, 296, 298, 311)
- [ ] Update `src/caps_provider.rs` (line 188)

### **Constants Cleanup**
- [ ] Remove hardcoded modules from `src/constants.rs`
- [ ] Keep system constants (system_ids, error_codes, error_types)
- [ ] Update imports in target files

### **Testing**
- [ ] Test configuration provider registration
- [ ] Test configuration value resolution
- [ ] Test merge rule application
- [ ] Test priority-based resolution
- [ ] Test runtime configuration updates
- [ ] Verify all compilation errors resolved

## 🎯 **EXPECTED OUTCOME**

After migration:
- **✅ Zero hardcoded dimensions** in code
- **✅ Configuration hub architecture** following Actor Core patterns
- **✅ Multiple subsystem configuration providers** with priority-based resolution
- **✅ Merge/override/aggregate logic** for configuration values
- **✅ Runtime configuration updates** without code changes
- **✅ Subsystem independence** for configuration management
- **✅ Type-safe configuration access** throughout

## 📊 **IMPACT ASSESSMENT**

- **🔥 EFFORT:** Medium-High (2-3 days focused work)
- **🚨 RISK:** Low-Medium (new architecture, but follows existing patterns)
- **✅ VALUE:** Very High (true hub architecture with runtime configuration)

## 🏗️ **ARCHITECTURE BENEFITS**

### **1. Consistency with Actor Core**
- **Same patterns**: ConfigurationRegistry, ConfigurationCombiner, ConfigurationAggregator
- **Familiar interface**: Similar to PluginRegistry, CombinerRegistry, Aggregator
- **Consistent behavior**: Merge rules, validation, caching

### **2. Subsystem Independence**
- **Each subsystem manages its own configuration**
- **No hardcoded dependencies**
- **Easy to add/remove subsystems**

### **3. Flexible Configuration Resolution**
- **Priority-based resolution**: Higher priority subsystems can override lower priority ones
- **Multiple merge strategies**: Override, Sum, Max, Min, Average, Merge, Concat
- **Runtime updates**: No restart required for configuration changes

### **4. Hub Architecture**
- **Centralized configuration management**
- **Consistent with stats and resources handling**
- **Easy to extend and maintain**

This migration plan provides a comprehensive approach that creates a true configuration hub architecture, following the Actor Core patterns while achieving runtime configuration loading with subsystem independence.
