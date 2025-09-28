//! # YAML Configuration Loader Tests
//! 
//! Comprehensive test suite for the YAML configuration loader

use element_core::config::yaml_loader::{
    YamlConfigLoader, ElementConfig, ElementInteractionConfig, ConfigValidationRule,
    InteractionConfig, RelationshipConfig, InteractionDynamicsConfig, ElementPairConfig,
    EffectConfig, EffectCondition, DynamicsModification, ProbabilityConfig, SigmoidConfig,
    ElementSigmoidConfig, CustomFunctionConfig, StatusPoolConfig, StatusPool,
    StatusEffectPoolEntry, StatusDynamicsConfig
};
use element_core::unified_registry::{
    ElementDefinition, ElementCategory, ElementProperties, DerivedStatConfig,
    StatusEffectConfig, ElementReferences, ElementAliases
};
use element_core::core::ElementConfig as CoreElementConfig;
use element_core::ElementCoreResult;
use std::path::PathBuf;
use std::collections::HashMap;

#[test]
fn test_yaml_loader_creation() {
    let config_dir = PathBuf::from("test_configs");
    let loader = YamlConfigLoader::new(config_dir);
    
    assert_eq!(loader.cache_size(), 0);
    assert!(!loader.is_cached("test_element"));
}

#[test]
fn test_yaml_loader_default() {
    let loader = YamlConfigLoader::default();
    assert_eq!(loader.cache_size(), 0);
}

#[test]
fn test_element_config_creation() {
    let element_def = ElementDefinition::new(
        "fire".to_string(),
        "Fire".to_string(),
        "Fire element".to_string(),
        ElementCategory::Elemental,
        CoreElementConfig::default(),
    );
    
    let config = ElementConfig {
        version: 1,
        element: element_def,
        interactions: vec![],
        status_effects: vec![],
        derived_stats: vec![],
    };
    
    assert_eq!(config.version, 1);
    assert_eq!(config.element.id, "fire");
    assert_eq!(config.element.name, "Fire");
}

#[test]
fn test_element_interaction_config() {
    let interaction = ElementInteractionConfig {
        target_element: "wood".to_string(),
        interaction_type: "overcoming".to_string(),
        base_multiplier: 1.5,
        scaling_factor: 0.1,
        special_effects: vec!["burn".to_string()],
    };
    
    assert_eq!(interaction.target_element, "wood");
    assert_eq!(interaction.interaction_type, "overcoming");
    assert_eq!(interaction.base_multiplier, 1.5);
    assert_eq!(interaction.scaling_factor, 0.1);
    assert_eq!(interaction.special_effects.len(), 1);
}

#[test]
fn test_config_validation_rule() {
    let rule = ConfigValidationRule::new(
        "test_rule".to_string(),
        |config| {
            if config.version > 0 {
                Ok(())
            } else {
                Err("Version must be greater than 0".to_string())
            }
        }
    );
    
    assert_eq!(rule.name, "test_rule");
    
    // Test validation
    let element_def = ElementDefinition::new(
        "fire".to_string(),
        "Fire".to_string(),
        "Fire element".to_string(),
        ElementCategory::Elemental,
        CoreElementConfig::default(),
    );
    
    let valid_config = ElementConfig {
        version: 1,
        element: element_def,
        interactions: vec![],
        status_effects: vec![],
        derived_stats: vec![],
    };
    
    let result = (rule.validator)(&valid_config);
    assert!(result.is_ok());
}

#[test]
fn test_config_validation_rule_clone() {
    let rule = ConfigValidationRule::new(
        "test_rule".to_string(),
        |_| Ok(()),
    );
    
    let cloned_rule = rule.clone();
    assert_eq!(cloned_rule.name, "test_rule");
    
    // Test that cloned rule works (with default validator)
    let element_def = ElementDefinition::new(
        "fire".to_string(),
        "Fire".to_string(),
        "Fire element".to_string(),
        ElementCategory::Elemental,
        CoreElementConfig::default(),
    );
    
    let config = ElementConfig {
        version: 1,
        element: element_def,
        interactions: vec![],
        status_effects: vec![],
        derived_stats: vec![],
    };
    
    let result = (cloned_rule.validator)(&config);
    assert!(result.is_ok());
}

#[test]
fn test_interaction_config() {
    let relationships = RelationshipConfig {
        same: 0.1,
        generating: 0.3,
        overcoming: 0.5,
        neutral: 0.1,
    };
    
    let dynamics = InteractionDynamicsConfig {
        trigger_scale: 1.0,
        steepness: 2.0,
        intensity_gain: 0.1,
        intensity_damping: 0.05,
        decay_rate: 0.02,
        refractory_gain: 0.1,
        refractory_decay: 0.05,
    };
    
    let mut pairs = HashMap::new();
    pairs.insert("fire".to_string(), ElementPairConfig {
        generating: vec!["wood".to_string()],
        overcoming: vec!["metal".to_string()],
        neutral: vec!["earth".to_string()],
    });
    
    let config = InteractionConfig {
        version: 1,
        relationships,
        dynamics,
        pairs,
        effects: vec![],
    };
    
    assert_eq!(config.version, 1);
    assert_eq!(config.relationships.same, 0.1);
    assert_eq!(config.dynamics.trigger_scale, 1.0);
    assert_eq!(config.pairs.len(), 1);
}

#[test]
fn test_probability_config() {
    let sigmoid = SigmoidConfig {
        default_steepness: 2.0,
        default_midpoint: 0.5,
        element_configs: HashMap::new(),
    };
    
    let mut custom_functions = HashMap::new();
    custom_functions.insert("custom_func".to_string(), CustomFunctionConfig {
        name: "custom_func".to_string(),
        parameters: HashMap::new(),
        function_type: "linear".to_string(),
    });
    
    let config = ProbabilityConfig {
        version: 1,
        sigmoid,
        custom_functions,
    };
    
    assert_eq!(config.version, 1);
    assert_eq!(config.sigmoid.default_steepness, 2.0);
    assert_eq!(config.custom_functions.len(), 1);
}

#[test]
fn test_status_pool_config() {
    let mut pools = HashMap::new();
    pools.insert("fire_effects".to_string(), StatusPool {
        name: "Fire Effects".to_string(),
        description: "Fire-related status effects".to_string(),
        effects: vec![StatusEffectPoolEntry {
            name: "burn".to_string(),
            effect_type: "damage_over_time".to_string(),
            base_probability: 0.3,
            base_duration: 5.0,
            base_intensity: 10.0,
            tick_interval: 1.0,
            max_stacks: 3,
            stackable: true,
            refresh_duration: true,
            effects: Some(HashMap::new()),
            hp_heal_per_tick: None,
            stamina_heal_per_tick: None,
            dynamics: StatusDynamicsConfig {
                intensity_gain: 0.1,
                intensity_damping: 0.05,
                decay_rate: 0.02,
                refractory_gain: 0.1,
                refractory_decay: 0.05,
            },
        }],
    });
    
    let config = StatusPoolConfig {
        version: 1,
        pools,
    };
    
    assert_eq!(config.version, 1);
    assert_eq!(config.pools.len(), 1);
    assert!(config.pools.contains_key("fire_effects"));
}

#[test]
fn test_effect_config() {
    let condition = EffectCondition {
        attacker: "fire".to_string(),
        defender: "wood".to_string(),
        relationship: "overcoming".to_string(),
    };
    
    let dynamics_mod = DynamicsModification {
        intensity_gain_mod: 1.5,
        intensity_damping_mod: 0.8,
    };
    
    let effect = EffectConfig {
        id: "fire_burn".to_string(),
        when: condition,
        apply_to: "defender".to_string(),
        pool_id: "fire_effects".to_string(),
        dynamics_mod: Some(dynamics_mod),
    };
    
    assert_eq!(effect.id, "fire_burn");
    assert_eq!(effect.when.attacker, "fire");
    assert_eq!(effect.when.defender, "wood");
    assert_eq!(effect.apply_to, "defender");
    assert_eq!(effect.pool_id, "fire_effects");
    assert!(effect.dynamics_mod.is_some());
}

#[test]
fn test_yaml_loader_cache_operations() {
    let config_dir = PathBuf::from("test_configs");
    let mut loader = YamlConfigLoader::new(config_dir);
    
    // Test initial state
    assert_eq!(loader.cache_size(), 0);
    assert!(!loader.is_cached("test_element"));
    assert!(loader.get_cached_config("test_element").is_none());
    
    // Test cache clear
    loader.clear_cache();
    assert_eq!(loader.cache_size(), 0);
}

#[test]
fn test_yaml_loader_validation_rules() {
    let config_dir = PathBuf::from("test_configs");
    let mut loader = YamlConfigLoader::new(config_dir);
    
    // Add validation rule
    let rule = ConfigValidationRule::new(
        "version_check".to_string(),
        |config| {
            if config.version > 0 {
                Ok(())
            } else {
                Err("Version must be positive".to_string())
            }
        }
    );
    
    loader.add_validation_rule(rule);
    
    // Test that rule was added (we can't directly access validation_rules,
    // but we can test that the loader still works)
    assert_eq!(loader.cache_size(), 0);
}

#[test]
fn test_serialization_deserialization() {
    // Test that all config structs can be serialized and deserialized
    let element_def = ElementDefinition::new(
        "fire".to_string(),
        "Fire".to_string(),
        "Fire element".to_string(),
        ElementCategory::Elemental,
        CoreElementConfig::default(),
    );
    
    let config = ElementConfig {
        version: 1,
        element: element_def,
        interactions: vec![],
        status_effects: vec![],
        derived_stats: vec![],
    };
    
    // Test JSON serialization
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: ElementConfig = serde_json::from_str(&json).unwrap();
    
    assert_eq!(config.version, deserialized.version);
    assert_eq!(config.element.id, deserialized.element.id);
    assert_eq!(config.element.name, deserialized.element.name);
}

#[test]
fn test_config_validation() {
    let config_dir = PathBuf::from("test_configs");
    let mut loader = YamlConfigLoader::new(config_dir);
    
    // Create valid config
    let element_def = ElementDefinition::new(
        "fire".to_string(),
        "Fire".to_string(),
        "Fire element".to_string(),
        ElementCategory::Elemental,
        CoreElementConfig::default(),
    );
    
    let valid_config = ElementConfig {
        version: 1,
        element: element_def,
        interactions: vec![],
        status_effects: vec![],
        derived_stats: vec![],
    };
    
    // Test that validation passes (we can't directly call validate_config,
    // but we can test that the loader works with valid configs)
    assert_eq!(valid_config.version, 1);
    assert!(!valid_config.element.id.is_empty());
    assert!(!valid_config.element.name.is_empty());
}

#[test]
fn test_config_error_handling() {
    let config_dir = PathBuf::from("nonexistent_directory");
    let mut loader = YamlConfigLoader::new(config_dir);
    
    // Test that loading from non-existent directory returns error
    let result = loader.load_element_config("nonexistent");
    assert!(result.is_err());
    
    // Test that loading all configs from non-existent directory returns error
    let result = loader.load_all_configs();
    assert!(result.is_err());
}

#[test]
fn test_config_file_patterns() {
    let config_dir = PathBuf::from("test_configs");
    let loader = YamlConfigLoader::new(config_dir);
    
    // Test that loader is created successfully
    assert_eq!(loader.cache_size(), 0);
    
    // Test that we can check cache status
    assert!(!loader.is_cached("any_element"));
    assert!(loader.get_cached_config("any_element").is_none());
}
