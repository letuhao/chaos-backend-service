//! Configuration validation tests for CI.
//!
//! These tests ensure that the sample configuration files are valid
//! and can be loaded correctly by the registry loader.

use actor_core::registry::loader::*;
use actor_core::interfaces::{CapLayerRegistry, CombinerRegistry};
use actor_core::enums::Bucket;
use std::path::Path;

/// Test that the sample cap_layers.yaml configuration is valid.
#[test]
fn test_sample_cap_layers_yaml_valid() {
    let config_path = Path::new("configs/cap_layers.yaml");
    
    // Test that the file exists
    assert!(config_path.exists(), "cap_layers.yaml should exist");
    
    // Test that it can be loaded
    let result = load_cap_layers(config_path);
    assert!(result.is_ok(), "cap_layers.yaml should load successfully: {:?}", result.err());
    
    let registry = result.unwrap();
    
    // Test basic properties
    let layer_order = registry.get_layer_order();
    assert!(!layer_order.is_empty(), "Should have at least one layer");
    
    // Verify expected layers are present
    assert!(layer_order.contains(&"base".to_string()), "Should contain 'base' layer");
    assert!(layer_order.contains(&"equipment".to_string()), "Should contain 'equipment' layer");
    assert!(layer_order.contains(&"buffs".to_string()), "Should contain 'buffs' layer");
    
    // Test validation
    let validation_result = registry.validate();
    assert!(validation_result.is_ok(), "Registry should validate successfully: {:?}", validation_result.err());
}

/// Test that the sample combiner.yaml configuration is valid.
#[test]
fn test_sample_combiner_yaml_valid() {
    let config_path = Path::new("configs/combiner.yaml");
    
    // Test that the file exists
    assert!(config_path.exists(), "combiner.yaml should exist");
    
    // Test that it can be loaded
    let result = load_combiner(config_path);
    assert!(result.is_ok(), "combiner.yaml should load successfully: {:?}", result.err());
    
    let registry = result.unwrap();
    
    // Test basic properties
    let attack_rule = registry.get_rule("attack");
    assert!(attack_rule.is_some(), "Should have attack rule");
    
    let defense_rule = registry.get_rule("defense");
    assert!(defense_rule.is_some(), "Should have defense rule");
    
    let speed_rule = registry.get_rule("speed");
    assert!(speed_rule.is_some(), "Should have speed rule");
    
    // Test validation
    let validation_result = registry.validate();
    assert!(validation_result.is_ok(), "Registry should validate successfully: {:?}", validation_result.err());
}

/// Test that both configurations can be loaded together.
#[test]
fn test_load_all_configs() {
    let config_dir = Path::new("configs");
    
    // Test that the directory exists
    assert!(config_dir.exists(), "configs directory should exist");
    assert!(config_dir.is_dir(), "configs should be a directory");
    
    // Test loading both configurations
    let result = load_all(config_dir);
    assert!(result.is_ok(), "Should load all configs successfully: {:?}", result.err());
    
    let (cap_layers, combiner) = result.unwrap();
    
    // Test that both registries are valid
    assert!(cap_layers.validate().is_ok(), "Cap layers should validate");
    assert!(combiner.validate().is_ok(), "Combiner should validate");
    
    // Test that they have expected content
    assert!(!cap_layers.get_layer_order().is_empty(), "Cap layers should have layers");
    assert!(combiner.get_rule("attack").is_some(), "Combiner should have attack rule");
}

/// Test that the configurations work with the bucket processor.
#[test]
fn test_configs_work_with_bucket_processor() {
    use actor_core::bucket_processor::*;
    
    // Get the processing order
    let processing_order = get_bucket_processing_order();
    assert!(!processing_order.is_empty(), "Should have processing order");
    
    // Verify core buckets are in the correct order
    let flat_pos = processing_order.iter().position(|&b| b == Bucket::Flat).unwrap();
    let mult_pos = processing_order.iter().position(|&b| b == Bucket::Mult).unwrap();
    let post_add_pos = processing_order.iter().position(|&b| b == Bucket::PostAdd).unwrap();
    let override_pos = processing_order.iter().position(|&b| b == Bucket::Override).unwrap();
    
    assert!(flat_pos < mult_pos, "Flat should come before Mult");
    assert!(mult_pos < post_add_pos, "Mult should come before PostAdd");
    assert!(post_add_pos < override_pos, "PostAdd should come before Override");
}

/// Test that the configurations are compatible with feature flags.
#[test]
fn test_configs_compatible_with_feature_flags() {
    // Test without extra_buckets feature
    let combiner_path = Path::new("configs/combiner.yaml");
    let result = load_combiner(combiner_path);
    assert!(result.is_ok(), "Should load without extra_buckets feature");
    
    // Test with extra_buckets feature (if available)
    #[cfg(feature = "extra_buckets")]
    {
        let result = load_combiner(combiner_path);
        assert!(result.is_ok(), "Should load with extra_buckets feature");
    }
}

/// Test that the configurations have reasonable values.
#[test]
fn test_configs_have_reasonable_values() {
    let cap_layers = load_cap_layers("configs/cap_layers.yaml").unwrap();
    let combiner = load_combiner("configs/combiner.yaml").unwrap();
    
    // Test cap layers have reasonable structure
    let layer_order = cap_layers.get_layer_order();
    assert!(!layer_order.is_empty(), "Should have at least one layer");
    
    // Test combiner rules have reasonable clamp values
    let attack_rule = combiner.get_rule("attack").unwrap();
    if let Some(clamp) = attack_rule.clamp_default {
        assert!(clamp.get_min() >= 0.0, "Attack min should be non-negative");
        assert!(clamp.get_max() > clamp.get_min(), "Attack max should be greater than min");
        assert!(clamp.get_max() <= 100000.0, "Attack max should be reasonable");
    }
}

/// Test that the configurations can be used in a real aggregation scenario.
#[test]
fn test_configs_integration_with_aggregation() {
    use actor_core::types::*;
    use actor_core::enums::*;
    use actor_core::bucket_processor::*;
    
    // Load configurations
    let _cap_layers = load_cap_layers("configs/cap_layers.yaml").unwrap();
    let combiner = load_combiner("configs/combiner.yaml").unwrap();
    
    // Create test contributions
    let contributions = vec![
        Contribution::new("attack".to_string(), Bucket::Flat, 10.0, "test".to_string()),
        Contribution::new("attack".to_string(), Bucket::Mult, 1.5, "test".to_string()),
        Contribution::new("attack".to_string(), Bucket::PostAdd, 5.0, "test".to_string()),
    ];
    
    // Get clamp caps from combiner
    let attack_rule = combiner.get_rule("attack").unwrap();
    let clamp_caps = attack_rule.clamp_default;
    
    // Process contributions
    let result = process_contributions_in_order(contributions, 0.0, clamp_caps.as_ref());
    assert!(result.is_ok(), "Should process contributions successfully");
    
    let final_value = result.unwrap();
    assert!(final_value > 0.0, "Final value should be positive");
    assert!(final_value.is_finite(), "Final value should be finite");
}

/// Test that the configurations are well-formed YAML.
#[test]
fn test_configs_are_valid_yaml() {
    use std::fs;
    
    // Test cap_layers.yaml
    let cap_layers_content = fs::read_to_string("configs/cap_layers.yaml").unwrap();
    let yaml_result: Result<serde_yaml::Value, _> = serde_yaml::from_str(&cap_layers_content);
    assert!(yaml_result.is_ok(), "cap_layers.yaml should be valid YAML");
    
    // Test combiner.yaml
    let combiner_content = fs::read_to_string("configs/combiner.yaml").unwrap();
    let yaml_result: Result<serde_yaml::Value, _> = serde_yaml::from_str(&combiner_content);
    assert!(yaml_result.is_ok(), "combiner.yaml should be valid YAML");
}

/// Test that the configurations have consistent structure.
#[test]
fn test_configs_consistent_structure() {
    let cap_layers = load_cap_layers("configs/cap_layers.yaml").unwrap();
    let combiner = load_combiner("configs/combiner.yaml").unwrap();
    
    // Test that all layers have reasonable names
    let layer_order = cap_layers.get_layer_order();
    for layer_name in &layer_order {
        assert!(!layer_name.is_empty(), "Layer name should not be empty");
        assert!(layer_name.len() <= 50, "Layer name should be reasonable length");
        assert!(layer_name.chars().all(|c| c.is_alphanumeric() || c == '_'), 
                "Layer name '{}' should be alphanumeric or underscore", layer_name);
    }
    
    // Test that all rules have reasonable names
    let attack_rule = combiner.get_rule("attack");
    let defense_rule = combiner.get_rule("defense");
    let speed_rule = combiner.get_rule("speed");
    
    assert!(attack_rule.is_some(), "Should have attack rule");
    assert!(defense_rule.is_some(), "Should have defense rule");
    assert!(speed_rule.is_some(), "Should have speed rule");
}
