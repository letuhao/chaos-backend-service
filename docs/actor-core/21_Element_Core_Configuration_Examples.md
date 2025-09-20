# Element Core Configuration Examples

## 🎯 **OBJECTIVE**

Create simple example configurations for element-core stats and elemental mastery system to demonstrate how the configuration hub would work in practice.

## 🔥 **ELEMENT CORE STATS CONFIGURATION**

### **1. Element Core Subsystem Configuration Provider**
Create `src/config/providers/element_core_provider.rs`:

```rust
use std::collections::HashMap;
use async_trait::async_trait;
use crate::config::provider::ConfigurationProvider;
use crate::config::value::{ConfigurationValue, ConfigurationValueType, ConfigurationMergeRule, ConfigurationMergeStrategy};
use crate::ActorCoreResult;

pub struct ElementCoreConfigurationProvider {
    provider_id: String,
    priority: i64,
    config_data: HashMap<String, HashMap<String, ConfigurationValue>>,
}

impl ElementCoreConfigurationProvider {
    pub fn new() -> Self {
        let mut config_data = HashMap::new();
        
        // Element affinities configuration (based on actual element-core structure)
        let mut element_affinities = HashMap::new();
        // Five Elements (Ngũ Hành)
        element_affinities.insert("fire_affinity".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        element_affinities.insert("water_affinity".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        element_affinities.insert("wood_affinity".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        element_affinities.insert("metal_affinity".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        element_affinities.insert("earth_affinity".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        // Light/Dark elements
        element_affinities.insert("light_affinity".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        element_affinities.insert("dark_affinity".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        
        config_data.insert("element_affinities".to_string(), element_affinities);
        
        // Element interactions configuration (based on actual tương sinh tương khắc)
        let mut element_interactions = HashMap::new();
        // Generating (Tương Sinh) - Fire generates Earth
        element_interactions.insert("fire_earth_generating".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.6.into()), // 0.6x multiplier
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: false, // Interactions are fixed
            can_merge: false,
        });
        // Overcoming (Tương Khắc) - Fire overcomes Metal
        element_interactions.insert("fire_metal_overcoming".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.5.into()), // 1.5x multiplier
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        // Water overcomes Fire
        element_interactions.insert("water_fire_overcoming".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.5.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        // Light vs Dark
        element_interactions.insert("light_dark_overcoming".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(2.0.into()), // 2.0x multiplier
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        
        config_data.insert("element_interactions".to_string(), element_interactions);
        
        // Element resistances configuration
        let mut element_resistances = HashMap::new();
        element_resistances.insert("fire_resistance".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        element_resistances.insert("water_resistance".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        element_resistances.insert("wood_resistance".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        element_resistances.insert("metal_resistance".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        element_resistances.insert("earth_resistance".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        element_resistances.insert("light_resistance".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        element_resistances.insert("dark_resistance".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_core".to_string(),
            priority: 150,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        
        config_data.insert("element_resistances".to_string(), element_resistances);
        
        Self {
            provider_id: "element_core".to_string(),
            priority: 150,
            config_data,
        }
    }
}

#[async_trait]
impl ConfigurationProvider for ElementCoreConfigurationProvider {
    fn provider_id(&self) -> &str { &self.provider_id }
    fn priority(&self) -> i64 { self.priority }
    
    fn get_supported_categories(&self) -> Vec<String> {
        vec!["element_affinities".to_string(), "element_interactions".to_string(), "element_resistances".to_string()]
    }
    
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>> {
        Ok(self.config_data.get(category)?.get(key).cloned())
    }
    
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        Ok(self.config_data.get(category).cloned().unwrap_or_default())
    }
    
    fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule> {
        match (category, key) {
            ("element_affinities", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Sum, // Sum all element affinities
                use_pipeline: true,
                default_value: Some(serde_json::Value::Number(0.0.into())),
                validation_rules: vec![],
            }),
            ("element_interactions", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override, // Interactions are fixed
                use_pipeline: false,
                default_value: None,
                validation_rules: vec![],
            }),
            ("element_resistances", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Max, // Use maximum resistance for safety
                use_pipeline: true,
                default_value: Some(serde_json::Value::Number(0.0.into())),
                validation_rules: vec![],
            }),
            _ => None,
        }
    }
    
    async fn validate_config(&self) -> ActorCoreResult<()> {
        // Validate element affinities are between 0.0 and 1.0
        if let Some(element_affinities) = self.config_data.get("element_affinities") {
            for (key, value) in element_affinities {
                if let Some(affinity) = value.value.as_f64() {
                    if affinity < 0.0 || affinity > 1.0 {
                        return Err(crate::ActorCoreError::ConfigurationError(
                            format!("Element affinity {} must be between 0.0 and 1.0, got {}", key, affinity)
                        ));
                    }
                }
            }
        }
        
        // Validate element resistances are between 0.0 and 1.0
        if let Some(element_resistances) = self.config_data.get("element_resistances") {
            for (key, value) in element_resistances {
                if let Some(resistance) = value.value.as_f64() {
                    if resistance < 0.0 || resistance > 1.0 {
                        return Err(crate::ActorCoreError::ConfigurationError(
                            format!("Element resistance {} must be between 0.0 and 1.0, got {}", key, resistance)
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
}
```

### **2. Element Core Configuration File**
Create `configs/subsystems/element_core.yaml`:

```yaml
# Element Core Subsystem Configuration
subsystem_id: "element_core"
priority: 150
description: "Element Core subsystem for elemental stats and interactions"

categories:
  element_affinities:
    # Five Elements (Ngũ Hành)
    fire_affinity: 0.0
    water_affinity: 0.0
    wood_affinity: 0.0
    metal_affinity: 0.0
    earth_affinity: 0.0
    # Light/Dark elements
    light_affinity: 0.0
    dark_affinity: 0.0
    
  element_interactions:
    # Generating (Tương Sinh) - 0.6x multiplier
    fire_earth_generating: 0.6
    earth_metal_generating: 0.6
    metal_water_generating: 0.6
    water_wood_generating: 0.6
    wood_fire_generating: 0.6
    
    # Overcoming (Tương Khắc) - 1.5x multiplier
    fire_metal_overcoming: 1.5
    metal_wood_overcoming: 1.5
    wood_earth_overcoming: 1.5
    earth_water_overcoming: 1.5
    water_fire_overcoming: 1.5
    
    # Light vs Dark - 2.0x multiplier
    light_dark_overcoming: 2.0
    dark_light_overcoming: 2.0
    
  element_resistances:
    fire_resistance: 0.0
    water_resistance: 0.0
    wood_resistance: 0.0
    metal_resistance: 0.0
    earth_resistance: 0.0
    light_resistance: 0.0
    dark_resistance: 0.0
```

## ⚡ **ELEMENTAL MASTERY SYSTEM CONFIGURATION**

### **1. Elemental Mastery Subsystem Configuration Provider**
Create `src/config/providers/elemental_mastery_provider.rs`:

```rust
use std::collections::HashMap;
use async_trait::async_trait;
use crate::config::provider::ConfigurationProvider;
use crate::config::value::{ConfigurationValue, ConfigurationValueType, ConfigurationMergeRule, ConfigurationMergeStrategy};
use crate::ActorCoreResult;

pub struct ElementalMasteryConfigurationProvider {
    provider_id: String,
    priority: i64,
    config_data: HashMap<String, HashMap<String, ConfigurationValue>>,
}

impl ElementalMasteryConfigurationProvider {
    pub fn new() -> Self {
        let mut config_data = HashMap::new();
        
        // Mastery experience configuration (based on actual elemental mastery system)
        let mut mastery_experience = HashMap::new();
        // Five Elements (Ngũ Hành) - using correct element names
        mastery_experience.insert("fire_mastery_xp".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        mastery_experience.insert("water_mastery_xp".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        mastery_experience.insert("wood_mastery_xp".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        mastery_experience.insert("metal_mastery_xp".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        mastery_experience.insert("earth_mastery_xp".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        // Light/Dark elements
        mastery_experience.insert("light_mastery_xp".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        mastery_experience.insert("dark_mastery_xp".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        
        config_data.insert("mastery_experience".to_string(), mastery_experience);
        
        // Mastery levels configuration (24 levels across 6 tiers)
        let mut mastery_levels = HashMap::new();
        mastery_levels.insert("fire_mastery_level".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: false, // Levels are calculated, not merged
        });
        mastery_levels.insert("water_mastery_level".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: false,
        });
        mastery_levels.insert("wood_mastery_level".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: false,
        });
        mastery_levels.insert("metal_mastery_level".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: false,
        });
        mastery_levels.insert("earth_mastery_level".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: false,
        });
        mastery_levels.insert("light_mastery_level".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: false,
        });
        mastery_levels.insert("dark_mastery_level".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.into()),
            value_type: ConfigurationValueType::Integer,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: false,
        });
        
        config_data.insert("mastery_levels".to_string(), mastery_levels);
        
        // Power scale configuration (based on actual power scale first system)
        let mut power_scale_config = HashMap::new();
        power_scale_config.insert("base_power_scale_formula".to_string(), ConfigurationValue {
            value: serde_json::Value::String("log10(experience / 1000000.0) * 1000.0".to_string()),
            value_type: ConfigurationValueType::String,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: false, // Formula is fixed
            can_merge: false,
        });
        power_scale_config.insert("min_power_scale".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        power_scale_config.insert("max_power_scale".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1000000.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        power_scale_config.insert("level_progression_rate".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.1.into()), // 10% increase per level
            value_type: ConfigurationValueType::Float,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        
        config_data.insert("power_scale_config".to_string(), power_scale_config);
        
        // Realm multipliers configuration (exponential scaling based on breakthroughs)
        let mut realm_multipliers = HashMap::new();
        realm_multipliers.insert("0_breakthroughs".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        realm_multipliers.insert("1_breakthrough".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(2.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        realm_multipliers.insert("2_breakthroughs".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(4.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        realm_multipliers.insert("3_breakthroughs".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(8.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        realm_multipliers.insert("4_breakthroughs".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(16.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        realm_multipliers.insert("5_breakthroughs".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(32.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "elemental_mastery".to_string(),
            priority: 200,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        
        config_data.insert("realm_multipliers".to_string(), realm_multipliers);
        
        Self {
            provider_id: "elemental_mastery".to_string(),
            priority: 200,
            config_data,
        }
    }
}

#[async_trait]
impl ConfigurationProvider for ElementalMasteryConfigurationProvider {
    fn provider_id(&self) -> &str { &self.provider_id }
    fn priority(&self) -> i64 { self.priority }
    
    fn get_supported_categories(&self) -> Vec<String> {
        vec!["mastery_experience".to_string(), "mastery_levels".to_string(), "power_scale_config".to_string(), "realm_multipliers".to_string()]
    }
    
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>> {
        Ok(self.config_data.get(category)?.get(key).cloned())
    }
    
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        Ok(self.config_data.get(category).cloned().unwrap_or_default())
    }
    
    fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule> {
        match (category, key) {
            ("mastery_experience", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Sum, // Sum all mastery experience
                use_pipeline: true,
                default_value: Some(serde_json::Value::Number(0.into())),
                validation_rules: vec![],
            }),
            ("mastery_levels", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override, // Levels are calculated
                use_pipeline: false,
                default_value: Some(serde_json::Value::Number(1.into())),
                validation_rules: vec![],
            }),
            ("power_scale_config", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override, // Power scale config is fixed
                use_pipeline: false,
                default_value: None,
                validation_rules: vec![],
            }),
            ("realm_multipliers", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override, // Realm multipliers are fixed
                use_pipeline: false,
                default_value: None,
                validation_rules: vec![],
            }),
            _ => None,
        }
    }
    
    async fn validate_config(&self) -> ActorCoreResult<()> {
        // Validate mastery experience is non-negative
        if let Some(mastery_experience) = self.config_data.get("mastery_experience") {
            for (key, value) in mastery_experience {
                if let Some(xp) = value.value.as_i64() {
                    if xp < 0 {
                        return Err(crate::ActorCoreError::ConfigurationError(
                            format!("Mastery experience {} must be non-negative, got {}", key, xp)
                        ));
                    }
                }
            }
        }
        
        // Validate mastery levels are between 1 and 24 (based on actual system)
        if let Some(mastery_levels) = self.config_data.get("mastery_levels") {
            for (key, value) in mastery_levels {
                if let Some(level) = value.value.as_i64() {
                    if level < 1 || level > 24 {
                        return Err(crate::ActorCoreError::ConfigurationError(
                            format!("Mastery level {} must be between 1 and 24, got {}", key, level)
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
}
```

### **2. Elemental Mastery Configuration File**
Create `configs/subsystems/elemental_mastery.yaml`:

```yaml
# Elemental Mastery Subsystem Configuration
subsystem_id: "elemental_mastery"
priority: 200
description: "Elemental Mastery subsystem for mastery experience and levels"

categories:
  mastery_experience:
    # Five Elements (Ngũ Hành) - using correct element names
    fire_mastery_xp: 0
    water_mastery_xp: 0
    wood_mastery_xp: 0
    metal_mastery_xp: 0
    earth_mastery_xp: 0
    # Light/Dark elements
    light_mastery_xp: 0
    dark_mastery_xp: 0
    
  mastery_levels:
    fire_mastery_level: 1
    water_mastery_level: 1
    wood_mastery_level: 1
    metal_mastery_level: 1
    earth_mastery_level: 1
    light_mastery_level: 1
    dark_mastery_level: 1
    
  power_scale_config:
    base_power_scale_formula: "log10(experience / 1000000.0) * 1000.0"
    min_power_scale: 1.0
    max_power_scale: 1000000.0
    level_progression_rate: 0.1  # 10% increase per level
    
  realm_multipliers:
    0_breakthroughs: 1.0
    1_breakthrough: 2.0
    2_breakthroughs: 4.0
    3_breakthroughs: 8.0
    4_breakthroughs: 16.0
    5_breakthroughs: 32.0
    6_breakthroughs: 64.0
    7_breakthroughs: 128.0
    8_breakthroughs: 256.0
    9_breakthroughs: 512.0
    10_breakthroughs: 1024.0
    11_breakthroughs: 2048.0
    12_breakthroughs: 4096.0  # Cap at 12 breakthroughs
```

## 📊 **DERIVED STATS REGISTRATION**

### **1. Element Core Derived Stats Provider**
Create `src/config/providers/element_derived_stats_provider.rs`:

```rust
use std::collections::HashMap;
use async_trait::async_trait;
use crate::config::provider::ConfigurationProvider;
use crate::config::value::{ConfigurationValue, ConfigurationValueType, ConfigurationMergeRule, ConfigurationMergeStrategy};
use crate::ActorCoreResult;

pub struct ElementDerivedStatsConfigurationProvider {
    provider_id: String,
    priority: i64,
    config_data: HashMap<String, HashMap<String, ConfigurationValue>>,
}

impl ElementDerivedStatsConfigurationProvider {
    pub fn new() -> Self {
        let mut config_data = HashMap::new();
        
        // Core Combat Stats
        let mut core_combat_stats = HashMap::new();
        core_combat_stats.insert("power_point".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        core_combat_stats.insert("defense_point".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        core_combat_stats.insert("crit_rate".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        core_combat_stats.insert("crit_damage".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        core_combat_stats.insert("accurate_rate".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        
        config_data.insert("core_combat_stats".to_string(), core_combat_stats);
        
        // Advanced Derived Stats (MVP)
        let mut advanced_derived_stats = HashMap::new();
        // Skill Performance
        advanced_derived_stats.insert("skill_execution_speed".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        advanced_derived_stats.insert("skill_cooldown_reduction".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        advanced_derived_stats.insert("skill_resource_efficiency".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        advanced_derived_stats.insert("skill_cast_time_reduction".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        
        // Combat Defense
        advanced_derived_stats.insert("parry_rate".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        advanced_derived_stats.insert("block_rate".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        
        // Element Mastery Bonuses
        advanced_derived_stats.insert("mastery_experience_gain".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        advanced_derived_stats.insert("mastery_decay_resistance".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        advanced_derived_stats.insert("mastery_training_speed".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        advanced_derived_stats.insert("mastery_synergy_bonus".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        
        // Advanced Combat Mechanics
        advanced_derived_stats.insert("element_penetration".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        advanced_derived_stats.insert("element_absorption".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        advanced_derived_stats.insert("element_reflection".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        advanced_derived_stats.insert("element_conversion".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        
        // Resource Management
        advanced_derived_stats.insert("resource_regeneration".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(1.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        advanced_derived_stats.insert("resource_efficiency".to_string(), ConfigurationValue {
            value: serde_json::Value::Number(0.0.into()),
            value_type: ConfigurationValueType::Float,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: true,
            can_merge: true,
        });
        
        config_data.insert("advanced_derived_stats".to_string(), advanced_derived_stats);
        
        // Element Categories
        let mut element_categories = HashMap::new();
        element_categories.insert("physical_category".to_string(), ConfigurationValue {
            value: serde_json::Value::String("physical,earth,metal".to_string()),
            value_type: ConfigurationValueType::String,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: false, // Categories are fixed
            can_merge: false,
        });
        element_categories.insert("elemental_category".to_string(), ConfigurationValue {
            value: serde_json::Value::String("fire,water,wood,air,lightning".to_string()),
            value_type: ConfigurationValueType::String,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        element_categories.insert("spiritual_category".to_string(), ConfigurationValue {
            value: serde_json::Value::String("light,dark,life,death".to_string()),
            value_type: ConfigurationValueType::String,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        element_categories.insert("dimensional_category".to_string(), ConfigurationValue {
            value: serde_json::Value::String("time,space,void,chaos".to_string()),
            value_type: ConfigurationValueType::String,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        
        config_data.insert("element_categories".to_string(), element_categories);
        
        // Element Tags
        let mut element_tags = HashMap::new();
        element_tags.insert("damage_over_time".to_string(), ConfigurationValue {
            value: serde_json::Value::String("burning,poison,bleeding,corruption,decay,entropy".to_string()),
            value_type: ConfigurationValueType::String,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        element_tags.insert("movement_effects".to_string(), ConfigurationValue {
            value: serde_json::Value::String("slow,petrification,spatial_lock,temporal_distortion".to_string()),
            value_type: ConfigurationValueType::String,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        element_tags.insert("control_effects".to_string(), ConfigurationValue {
            value: serde_json::Value::String("confusion,charm,mind_control,soul_drain".to_string()),
            value_type: ConfigurationValueType::String,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        element_tags.insert("healing_effects".to_string(), ConfigurationValue {
            value: serde_json::Value::String("regeneration,purification,blessing,growth".to_string()),
            value_type: ConfigurationValueType::String,
            source_provider: "element_derived_stats".to_string(),
            priority: 160,
            timestamp: chrono::Utc::now(),
            can_override: false,
            can_merge: false,
        });
        
        config_data.insert("element_tags".to_string(), element_tags);
        
        Self {
            provider_id: "element_derived_stats".to_string(),
            priority: 160,
            config_data,
        }
    }
}

#[async_trait]
impl ConfigurationProvider for ElementDerivedStatsConfigurationProvider {
    fn provider_id(&self) -> &str { &self.provider_id }
    fn priority(&self) -> i64 { self.priority }
    
    fn get_supported_categories(&self) -> Vec<String> {
        vec![
            "core_combat_stats".to_string(),
            "advanced_derived_stats".to_string(),
            "element_categories".to_string(),
            "element_tags".to_string()
        ]
    }
    
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>> {
        Ok(self.config_data.get(category)?.get(key).cloned())
    }
    
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        Ok(self.config_data.get(category).cloned().unwrap_or_default())
    }
    
    fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule> {
        match (category, key) {
            ("core_combat_stats", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Sum, // Sum all combat stats
                use_pipeline: true,
                default_value: Some(serde_json::Value::Number(0.0.into())),
                validation_rules: vec![],
            }),
            ("advanced_derived_stats", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Sum, // Sum all derived stats
                use_pipeline: true,
                default_value: Some(serde_json::Value::Number(0.0.into())),
                validation_rules: vec![],
            }),
            ("element_categories", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override, // Categories are fixed
                use_pipeline: false,
                default_value: None,
                validation_rules: vec![],
            }),
            ("element_tags", _) => Some(ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override, // Tags are fixed
                use_pipeline: false,
                default_value: None,
                validation_rules: vec![],
            }),
            _ => None,
        }
    }
    
    async fn validate_config(&self) -> ActorCoreResult<()> {
        // Validate core combat stats are non-negative
        if let Some(core_combat_stats) = self.config_data.get("core_combat_stats") {
            for (key, value) in core_combat_stats {
                if let Some(stat_value) = value.value.as_f64() {
                    if stat_value < 0.0 {
                        return Err(crate::ActorCoreError::ConfigurationError(
                            format!("Core combat stat {} must be non-negative, got {}", key, stat_value)
                        ));
                    }
                }
            }
        }
        
        // Validate advanced derived stats are within reasonable ranges
        if let Some(advanced_derived_stats) = self.config_data.get("advanced_derived_stats") {
            for (key, value) in advanced_derived_stats {
                if let Some(stat_value) = value.value.as_f64() {
                    match key.as_str() {
                        "skill_execution_speed" | "skill_cooldown_reduction" | "skill_resource_efficiency" | "skill_cast_time_reduction" => {
                            if stat_value < 0.0 || stat_value > 3.0 {
                                return Err(crate::ActorCoreError::ConfigurationError(
                                    format!("Skill stat {} must be between 0.0 and 3.0, got {}", key, stat_value)
                                ));
                            }
                        },
                        "parry_rate" | "block_rate" => {
                            if stat_value < 0.0 || stat_value > 1.0 {
                                return Err(crate::ActorCoreError::ConfigurationError(
                                    format!("Defense stat {} must be between 0.0 and 1.0, got {}", key, stat_value)
                                ));
                            }
                        },
                        _ => {
                            if stat_value < 0.0 {
                                return Err(crate::ActorCoreError::ConfigurationError(
                                    format!("Derived stat {} must be non-negative, got {}", key, stat_value)
                                ));
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}
```

### **2. Element Derived Stats Configuration File**
Create `configs/subsystems/element_derived_stats.yaml`:

```yaml
# Element Derived Stats Subsystem Configuration
subsystem_id: "element_derived_stats"
priority: 160
description: "Element Core derived stats, categories, and tags for other systems"

categories:
  core_combat_stats:
    power_point: 0.0
    defense_point: 0.0
    crit_rate: 0.0
    crit_damage: 0.0
    accurate_rate: 0.0
    
  advanced_derived_stats:
    # Skill Performance (MVP)
    skill_execution_speed: 1.0
    skill_cooldown_reduction: 0.0
    skill_resource_efficiency: 0.0
    skill_cast_time_reduction: 0.0
    
    # Combat Defense (MVP)
    parry_rate: 0.0
    block_rate: 0.0
    
    # Element Mastery Bonuses (MVP)
    mastery_experience_gain: 1.0
    mastery_decay_resistance: 0.0
    mastery_training_speed: 1.0
    mastery_synergy_bonus: 0.0
    
    # Advanced Combat Mechanics (MVP)
    element_penetration: 0.0
    element_absorption: 0.0
    element_reflection: 0.0
    element_conversion: 0.0
    
    # Resource Management (MVP)
    resource_regeneration: 1.0
    resource_efficiency: 0.0
    
  element_categories:
    physical_category: "physical,earth,metal"
    elemental_category: "fire,water,wood,air,lightning"
    spiritual_category: "light,dark,life,death"
    dimensional_category: "time,space,void,chaos"
    
  element_tags:
    damage_over_time: "burning,poison,bleeding,corruption,decay,entropy"
    movement_effects: "slow,petrification,spatial_lock,temporal_distortion"
    control_effects: "confusion,charm,mind_control,soul_drain"
    healing_effects: "regeneration,purification,blessing,growth"
```

## 🔄 **CONFIGURATION RESOLUTION EXAMPLE**

### **1. Multiple Subsystems with Different Priorities**
```rust
// Register configuration providers
let element_core_provider = Arc::new(ElementCoreConfigurationProvider::new());
let elemental_mastery_provider = Arc::new(ElementalMasteryConfigurationProvider::new());
let element_derived_stats_provider = Arc::new(ElementDerivedStatsConfigurationProvider::new());

config_registry.register_provider(element_core_provider).await?;
config_registry.register_provider(elemental_mastery_provider).await?;
config_registry.register_provider(element_derived_stats_provider).await?;

// Get configuration value (automatically merges from all providers)
let fire_affinity = config_aggregator.get_config_value("element_affinities", "fire_affinity").await?;
let fire_mastery_xp = config_aggregator.get_config_value("mastery_experience", "fire_mastery_xp").await?;
let skill_execution_speed = config_aggregator.get_config_value("advanced_derived_stats", "skill_execution_speed").await?;
let physical_category = config_aggregator.get_config_value("element_categories", "physical_category").await?;
```

### **2. Priority-Based Resolution**
```rust
// Element Core (priority 150) provides base fire_affinity: 0.0
// Elemental Mastery (priority 200) can override with higher priority
// Element Derived Stats (priority 160) provides derived stats
// Result: Uses Elemental Mastery value due to highest priority
```

### **3. Merge Strategy Examples**
```rust
// Sum strategy: Add all mastery experience from different sources
// Override strategy: Use highest priority value for interactions
// Max strategy: Use maximum value for safety limits
// Min strategy: Use minimum value for safety limits
```

## 🎯 **BENEFITS OF THIS APPROACH**

### **1. Subsystem Independence**
- **Element Core** manages its own elemental stats and interactions
- **Elemental Mastery** manages its own mastery experience and levels
- **Element Derived Stats** provides derived stats, categories, and tags for other systems
- **No hardcoded dependencies** between subsystems

### **2. Priority-Based Resolution**
- **Elemental Mastery** (priority 200) can override **Element Core** (priority 150)
- **Element Derived Stats** (priority 160) provides derived stats
- **Higher priority subsystems** can modify lower priority configurations
- **Flexible configuration management** without code changes

### **3. Merge Strategy Flexibility**
- **Sum**: Add all mastery experience from different sources
- **Override**: Use highest priority value for interactions
- **Max/Min**: Use extreme values for safety limits
- **Custom strategies** for different configuration types

### **4. Runtime Configuration Updates**
- **No restart required** for configuration changes
- **Dynamic subsystem registration** and unregistration
- **Real-time configuration updates** without code changes

### **5. Complete Element System Integration**
- **All derived stats** are available for other systems to query
- **Element categories** provide classification for skills and effects
- **Element tags** enable flexible effect targeting and filtering
- **Comprehensive element data** accessible through configuration hub

This example demonstrates how the configuration hub architecture would work in practice, allowing multiple subsystems to register their configurations with priority-based resolution and flexible merge strategies, while providing complete access to all element-related data for other systems!
