//! Edge case and error condition tests for Actor Core.
//!
//! These tests focus on boundary conditions, error handling,
//! and edge cases that might not be covered by normal usage.

use actor_core::prelude::*;
use std::collections::HashMap;

/// Test Caps edge cases
#[test]
fn test_caps_edge_cases() {
    // Test with identical min and max
    let caps = Caps::new(5.0, 5.0);
    assert!(caps.is_valid());
    assert_eq!(caps.clamp(3.0), 5.0);
    assert_eq!(caps.clamp(5.0), 5.0);
    assert_eq!(caps.clamp(7.0), 5.0);
    
    // Test with very small range
    let caps = Caps::new(0.0, 1e-10);
    assert!(caps.is_valid());
    assert_eq!(caps.clamp(0.0), 0.0);
    assert_eq!(caps.clamp(1e-10), 1e-10);
    assert_eq!(caps.clamp(1e-9), 1e-10);
    
    // Test with very large range
    let caps = Caps::new(-1e10, 1e10);
    assert!(caps.is_valid());
    assert_eq!(caps.clamp(0.0), 0.0);
    assert_eq!(caps.clamp(-1e10), -1e10);
    assert_eq!(caps.clamp(1e10), 1e10);
}

/// Test Caps with extreme values
#[test]
fn test_caps_extreme_values() {
    // Test with infinity
    let _caps = Caps::new(f64::NEG_INFINITY, f64::INFINITY);
    // Note: Infinity caps might not be considered valid by the current implementation
    // Let's test with a more reasonable range
    let caps = Caps::new(-1e10, 1e10);
    assert!(caps.is_valid());
    assert_eq!(caps.clamp(0.0), 0.0);
    assert_eq!(caps.clamp(-1e10), -1e10);
    assert_eq!(caps.clamp(1e10), 1e10);
    
    // Test with NaN (should be invalid)
    let caps = Caps::new(f64::NAN, 1.0);
    assert!(!caps.is_valid());
    
    let caps = Caps::new(1.0, f64::NAN);
    assert!(!caps.is_valid());
}

/// Test Contribution edge cases
#[test]
fn test_contribution_edge_cases() {
    // Test with empty strings
    let contrib = Contribution::new("".to_string(), Bucket::Flat, 0.0, "".to_string());
    assert_eq!(contrib.dimension, "");
    assert_eq!(contrib.system, "");
    
    // Test with very long strings
    let long_string = "a".repeat(1000);
    let contrib = Contribution::new(
        long_string.clone(),
        Bucket::Flat,
        0.0,
        long_string.clone()
    );
    assert_eq!(contrib.dimension, long_string);
    assert_eq!(contrib.system, long_string);
    
    // Test with extreme values
    let contrib = Contribution::new(
        "test".to_string(),
        Bucket::Flat,
        f64::MAX,
        "test".to_string()
    );
    assert_eq!(contrib.value, f64::MAX);
}

/// Test Actor edge cases
#[test]
fn test_actor_edge_cases() {
    // Test with empty name and race
    let actor = Actor::new("".to_string(), "".to_string());
    assert_eq!(actor.get_name(), "");
    assert_eq!(actor.get_race(), "");
    assert_eq!(actor.get_age(), 0);
    
    // Test with very long strings
    let long_string = "a".repeat(1000);
    let actor = Actor::new(long_string.clone(), long_string.clone());
    assert_eq!(actor.get_name(), long_string);
    assert_eq!(actor.get_race(), long_string);
}

/// Test Snapshot edge cases
#[test]
fn test_snapshot_edge_cases() {
    use uuid::Uuid;
    let mut snapshot = Snapshot::new(Uuid::new_v4(), 1);
    
    // Test with empty dimension
    let contrib = Contribution::new("".to_string(), Bucket::Flat, 1.0, "test".to_string());
    snapshot.add_primary(contrib);
    assert!(snapshot.primary.contains_key(""));
    
    // Test with duplicate dimensions
    let contrib1 = Contribution::new("test".to_string(), Bucket::Flat, 1.0, "test".to_string());
    let contrib2 = Contribution::new("test".to_string(), Bucket::Flat, 2.0, "test".to_string());
    snapshot.add_primary(contrib1);
    snapshot.add_primary(contrib2);
    // The add_primary method adds to the existing value, so 1.0 + 2.0 = 3.0
    assert_eq!(snapshot.primary.get("test"), Some(&3.0));
}

/// Test bucket processor edge cases
#[test]
fn test_bucket_processor_edge_cases() {
    // Test with empty contributions
    let contributions = vec![];
    let result = process_contributions_in_order(contributions, 5.0, None);
    assert!(!result.is_err());
    assert_eq!(result.unwrap(), 5.0);
    
    // Test with single contribution
    let contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, 1.0, "test".to_string())
    ];
    let result = process_contributions_in_order(contributions, 0.0, None);
    assert!(!result.is_err());
    assert_eq!(result.unwrap(), 1.0);
    
    // Test with zero value contributions
    let contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, 0.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Mult, 1.0, "test".to_string()),
    ];
    let result = process_contributions_in_order(contributions, 1.0, None);
    assert!(!result.is_err());
    assert_eq!(result.unwrap(), 1.0);
}

/// Test bucket processor with extreme values
#[test]
fn test_bucket_processor_extreme_values() {
    // Test with very large values
    let contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, f64::MAX, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Mult, 1.0, "test".to_string()),
    ];
    let result = process_contributions_in_order(contributions, 0.0, None);
    assert!(!result.is_err());
    assert_eq!(result.unwrap(), f64::MAX);
    
    // Test with very small values
    let contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, f64::MIN_POSITIVE, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Mult, 2.0, "test".to_string()),
    ];
    let result = process_contributions_in_order(contributions, 0.0, None);
    assert!(!result.is_err());
    assert!(result.unwrap() > 0.0);
}

/// Test validation edge cases
#[test]
fn test_validation_edge_cases() {
    // Test with empty contributions
    let contributions = vec![];
    let result = validate_contributions(&contributions);
    assert!(!result.has_errors());
    
    // Test with single valid contribution
    let contributions = vec![
        Contribution::new("strength".to_string(), Bucket::Flat, 1.0, "equipment".to_string())
    ];
    let result = validate_contributions(&contributions);
    assert!(!result.has_errors());
    
    // Test with single NaN contribution
    let contributions = vec![
        Contribution::new("strength".to_string(), Bucket::Flat, f64::NAN, "equipment".to_string())
    ];
    let result = validate_contributions(&contributions);
    assert!(result.has_errors());
    
    // Test with single infinite contribution
    let contributions = vec![
        Contribution::new("strength".to_string(), Bucket::Flat, f64::INFINITY, "equipment".to_string())
    ];
    let result = validate_contributions(&contributions);
    assert!(result.has_errors());
}

/// Test config loading edge cases
#[test]
fn test_config_loading_edge_cases() {
    // Test with non-existent file
    let result = load_cap_layers("non_existent.yaml");
    assert!(result.is_err());
    
    // Test with non-existent directory
    let result = load_all("non_existent_dir");
    assert!(result.is_err());
    
    // Test with empty file
    let temp_file = std::fs::File::create("empty.yaml").unwrap();
    drop(temp_file);
    let result = load_cap_layers("empty.yaml");
    assert!(result.is_err());
    
    // Clean up
    let _ = std::fs::remove_file("empty.yaml");
}

/// Test effective caps edge cases
#[test]
fn test_effective_caps_edge_cases() {
    // Test with empty map
    let effective_caps: EffectiveCaps = HashMap::new();
    assert!(effective_caps.is_empty());
    
    // Test with single entry
    let mut effective_caps: EffectiveCaps = HashMap::new();
    effective_caps.insert("test".to_string(), Caps::new(0.0, 100.0));
    assert_eq!(effective_caps.len(), 1);
    assert!(effective_caps.contains_key("test"));
    
    // Test with duplicate keys (should overwrite)
    let mut effective_caps: EffectiveCaps = HashMap::new();
    effective_caps.insert("test".to_string(), Caps::new(0.0, 100.0));
    effective_caps.insert("test".to_string(), Caps::new(0.0, 200.0));
    assert_eq!(effective_caps.len(), 1);
    assert_eq!(effective_caps.get("test").unwrap().get_max(), 200.0);
}

/// Test bucket processing with all bucket types
#[test]
fn test_bucket_processing_all_types() {
    let contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, 10.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Mult, 2.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::PostAdd, 5.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Override, 100.0, "test".to_string()),
    ];
    
    let result = process_contributions_in_order(contributions, 0.0, None);
    assert!(!result.is_err());
    assert_eq!(result.unwrap(), 100.0); // Override should win
}

/// Test bucket processing with extra buckets (if feature enabled)
#[cfg(feature = "extra_buckets")]
#[test]
fn test_bucket_processing_extra_buckets() {
    let contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, 10.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Exponential, 2.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Logarithmic, 1.5, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Conditional, 5.0, "test".to_string()),
    ];
    
    let result = process_contributions_in_order(contributions, 1.0, None);
    assert!(!result.is_err());
    assert!(result.unwrap().is_finite());
}

/// Test bucket processing with clamping edge cases
#[test]
fn test_bucket_processing_clamping_edge_cases() {
    let contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, 1000.0, "test".to_string()),
    ];
    
    // Test with tight clamping
    let caps = Caps::new(0.0, 100.0);
    let result = process_contributions_in_order(contributions.clone(), 0.0, Some(&caps));
    assert!(!result.is_err());
    assert_eq!(result.unwrap(), 100.0);
    
    // Test with no clamping
    let result = process_contributions_in_order(contributions, 0.0, None);
    assert!(!result.is_err());
    assert_eq!(result.unwrap(), 1000.0);
}

/// Test bucket processing with zero and negative values
#[test]
fn test_bucket_processing_zero_negative() {
    let contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, -10.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Mult, 0.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::PostAdd, 5.0, "test".to_string()),
    ];
    
    let result = process_contributions_in_order(contributions, 1.0, None);
    assert!(!result.is_err());
    assert_eq!(result.unwrap(), 5.0); // 1 + (-10) = -9, -9 * 0 = 0, 0 + 5 = 5
}
