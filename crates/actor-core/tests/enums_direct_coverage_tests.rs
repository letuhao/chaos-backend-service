//! Direct coverage tests for enums.rs module.
//! This file provides direct tests that exercise the actual enums module code.

use actor_core::enums::*;

// ============================================================================
// ACROSS LAYER POLICY TESTS
// ============================================================================

#[test]
fn test_across_layer_policy_variants() {
    let intersect = AcrossLayerPolicy::Intersect;
    let union = AcrossLayerPolicy::Union;
    let prioritized_override = AcrossLayerPolicy::PrioritizedOverride;
    
    assert_eq!(intersect, AcrossLayerPolicy::Intersect);
    assert_eq!(union, AcrossLayerPolicy::Union);
    assert_eq!(prioritized_override, AcrossLayerPolicy::PrioritizedOverride);
}

#[test]
fn test_across_layer_policy_debug() {
    let intersect = AcrossLayerPolicy::Intersect;
    let debug_string = format!("{:?}", intersect);
    assert!(debug_string.contains("Intersect"));
}

#[test]
fn test_across_layer_policy_clone() {
    let intersect = AcrossLayerPolicy::Intersect;
    let cloned = intersect.clone();
    assert_eq!(intersect, cloned);
}

#[test]
fn test_across_layer_policy_copy() {
    let intersect = AcrossLayerPolicy::Intersect;
    let copied = intersect; // This should work because it implements Copy
    assert_eq!(intersect, copied);
}

#[test]
fn test_across_layer_policy_serialization() {
    let intersect = AcrossLayerPolicy::Intersect;
    let json = serde_json::to_string(&intersect).unwrap();
    assert!(json.contains("Intersect"));
    
    let deserialized: AcrossLayerPolicy = serde_json::from_str(&json).unwrap();
    assert_eq!(intersect, deserialized);
}

#[test]
fn test_across_layer_policy_all_variants() {
    let policies = vec![
        AcrossLayerPolicy::Intersect,
        AcrossLayerPolicy::Union,
        AcrossLayerPolicy::PrioritizedOverride,
    ];
    
    assert_eq!(policies.len(), 3);
    
    for policy in policies {
        let debug_string = format!("{:?}", policy);
        assert!(!debug_string.is_empty());
    }
}

// ============================================================================
// OPERATOR TESTS
// ============================================================================

#[test]
fn test_operator_variants() {
    let sum = Operator::Sum;
    let max = Operator::Max;
    let min = Operator::Min;
    let multiply = Operator::Multiply;
    let average = Operator::Average;
    let intersect = Operator::Intersect;
    
    assert_eq!(sum, Operator::Sum);
    assert_eq!(max, Operator::Max);
    assert_eq!(min, Operator::Min);
    assert_eq!(multiply, Operator::Multiply);
    assert_eq!(average, Operator::Average);
    assert_eq!(intersect, Operator::Intersect);
}

#[test]
fn test_operator_debug() {
    let sum = Operator::Sum;
    let debug_string = format!("{:?}", sum);
    assert!(debug_string.contains("Sum"));
}

#[test]
fn test_operator_clone() {
    let sum = Operator::Sum;
    let cloned = sum.clone();
    assert_eq!(sum, cloned);
}

#[test]
fn test_operator_serialization() {
    let sum = Operator::Sum;
    let json = serde_json::to_string(&sum).unwrap();
    assert!(json.contains("Sum"));
    
    let deserialized: Operator = serde_json::from_str(&json).unwrap();
    assert_eq!(sum, deserialized);
}

#[test]
fn test_operator_all_variants() {
    let operators = vec![
        Operator::Sum,
        Operator::Max,
        Operator::Min,
        Operator::Multiply,
        Operator::Average,
        Operator::Intersect,
    ];
    
    assert_eq!(operators.len(), 6);
    
    for operator in operators {
        let debug_string = format!("{:?}", operator);
        assert!(!debug_string.is_empty());
    }
}

// ============================================================================
// BUCKET TESTS
// ============================================================================

#[test]
fn test_bucket_variants() {
    let flat = Bucket::Flat;
    let mult = Bucket::Mult;
    let post_add = Bucket::PostAdd;
    let override_bucket = Bucket::Override;
    
    assert_eq!(flat, Bucket::Flat);
    assert_eq!(mult, Bucket::Mult);
    assert_eq!(post_add, Bucket::PostAdd);
    assert_eq!(override_bucket, Bucket::Override);
}

#[test]
fn test_bucket_debug() {
    let flat = Bucket::Flat;
    let debug_string = format!("{:?}", flat);
    assert!(debug_string.contains("Flat"));
}

#[test]
fn test_bucket_clone() {
    let flat = Bucket::Flat;
    let cloned = flat.clone();
    assert_eq!(flat, cloned);
}

#[test]
fn test_bucket_hash() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert(Bucket::Flat, "flat_value");
    map.insert(Bucket::Mult, "mult_value");
    
    assert_eq!(map.get(&Bucket::Flat), Some(&"flat_value"));
    assert_eq!(map.get(&Bucket::Mult), Some(&"mult_value"));
}

#[test]
fn test_bucket_priority() {
    assert_eq!(Bucket::Flat.priority(), 1);
    assert_eq!(Bucket::Mult.priority(), 2);
    assert_eq!(Bucket::PostAdd.priority(), 3);
    assert_eq!(Bucket::Override.priority(), 4);
}

#[test]
fn test_bucket_is_valid() {
    assert!(Bucket::Flat.is_valid());
    assert!(Bucket::Mult.is_valid());
    assert!(Bucket::PostAdd.is_valid());
    assert!(Bucket::Override.is_valid());
}

#[test]
fn test_bucket_display_name() {
    assert_eq!(Bucket::Flat.display_name(), "Flat");
    assert_eq!(Bucket::Mult.display_name(), "Mult");
    assert_eq!(Bucket::PostAdd.display_name(), "PostAdd");
    assert_eq!(Bucket::Override.display_name(), "Override");
}

#[test]
fn test_bucket_serialization() {
    let flat = Bucket::Flat;
    let json = serde_json::to_string(&flat).unwrap();
    assert!(json.contains("Flat"));
    
    let deserialized: Bucket = serde_json::from_str(&json).unwrap();
    assert_eq!(flat, deserialized);
}

#[test]
fn test_bucket_all_variants() {
    let buckets = vec![
        Bucket::Flat,
        Bucket::Mult,
        Bucket::PostAdd,
        Bucket::Override,
    ];
    
    assert_eq!(buckets.len(), 4);
    
    for bucket in buckets {
        let debug_string = format!("{:?}", bucket);
        assert!(!debug_string.is_empty());
        assert!(bucket.is_valid());
        assert!(!bucket.display_name().is_empty());
    }
}

// ============================================================================
// CAP MODE TESTS
// ============================================================================

#[test]
fn test_cap_mode_variants() {
    let baseline = CapMode::Baseline;
    let additive = CapMode::Additive;
    let hard_max = CapMode::HardMax;
    let hard_min = CapMode::HardMin;
    let override_mode = CapMode::Override;
    let soft_max = CapMode::SoftMax;
    
    assert_eq!(baseline, CapMode::Baseline);
    assert_eq!(additive, CapMode::Additive);
    assert_eq!(hard_max, CapMode::HardMax);
    assert_eq!(hard_min, CapMode::HardMin);
    assert_eq!(override_mode, CapMode::Override);
    assert_eq!(soft_max, CapMode::SoftMax);
}

#[test]
fn test_cap_mode_debug() {
    let baseline = CapMode::Baseline;
    let debug_string = format!("{:?}", baseline);
    assert!(debug_string.contains("Baseline"));
}

#[test]
fn test_cap_mode_clone() {
    let baseline = CapMode::Baseline;
    let cloned = baseline.clone();
    assert_eq!(baseline, cloned);
}

#[test]
fn test_cap_mode_is_valid() {
    assert!(CapMode::Baseline.is_valid());
    assert!(CapMode::Additive.is_valid());
    assert!(CapMode::HardMax.is_valid());
    assert!(CapMode::HardMin.is_valid());
    assert!(CapMode::Override.is_valid());
    assert!(CapMode::SoftMax.is_valid());
}

#[test]
fn test_cap_mode_display_name() {
    assert_eq!(CapMode::Baseline.display_name(), "Baseline");
    assert_eq!(CapMode::Additive.display_name(), "Additive");
    assert_eq!(CapMode::HardMax.display_name(), "HardMax");
    assert_eq!(CapMode::HardMin.display_name(), "HardMin");
    assert_eq!(CapMode::Override.display_name(), "Override");
    assert_eq!(CapMode::SoftMax.display_name(), "SoftMax");
}

#[test]
fn test_cap_mode_serialization() {
    let baseline = CapMode::Baseline;
    let json = serde_json::to_string(&baseline).unwrap();
    assert!(json.contains("Baseline"));
    
    let deserialized: CapMode = serde_json::from_str(&json).unwrap();
    assert_eq!(baseline, deserialized);
}

#[test]
fn test_cap_mode_all_variants() {
    let cap_modes = vec![
        CapMode::Baseline,
        CapMode::Additive,
        CapMode::HardMax,
        CapMode::HardMin,
        CapMode::Override,
        CapMode::SoftMax,
    ];
    
    assert_eq!(cap_modes.len(), 6);
    
    for cap_mode in cap_modes {
        let debug_string = format!("{:?}", cap_mode);
        assert!(!debug_string.is_empty());
        assert!(cap_mode.is_valid());
        assert!(!cap_mode.display_name().is_empty());
    }
}

// ============================================================================
// COMPREHENSIVE SERIALIZATION TESTS
// ============================================================================

#[test]
fn test_all_enums_serialization_roundtrip() {
    // Test AcrossLayerPolicy
    let intersect = AcrossLayerPolicy::Intersect;
    let json = serde_json::to_string(&intersect).unwrap();
    let deserialized: AcrossLayerPolicy = serde_json::from_str(&json).unwrap();
    assert_eq!(intersect, deserialized);
    
    // Test Operator
    let sum = Operator::Sum;
    let json = serde_json::to_string(&sum).unwrap();
    let deserialized: Operator = serde_json::from_str(&json).unwrap();
    assert_eq!(sum, deserialized);
    
    // Test Bucket
    let flat = Bucket::Flat;
    let json = serde_json::to_string(&flat).unwrap();
    let deserialized: Bucket = serde_json::from_str(&json).unwrap();
    assert_eq!(flat, deserialized);
    
    // Test CapMode
    let baseline = CapMode::Baseline;
    let json = serde_json::to_string(&baseline).unwrap();
    let deserialized: CapMode = serde_json::from_str(&json).unwrap();
    assert_eq!(baseline, deserialized);
}

#[test]
fn test_enum_comparisons() {
    // Test equality
    assert_eq!(AcrossLayerPolicy::Intersect, AcrossLayerPolicy::Intersect);
    assert_ne!(AcrossLayerPolicy::Intersect, AcrossLayerPolicy::Union);
    
    assert_eq!(Operator::Sum, Operator::Sum);
    assert_ne!(Operator::Sum, Operator::Max);
    
    assert_eq!(Bucket::Flat, Bucket::Flat);
    assert_ne!(Bucket::Flat, Bucket::Mult);
    
    assert_eq!(CapMode::Baseline, CapMode::Baseline);
    assert_ne!(CapMode::Baseline, CapMode::Additive);
}

#[test]
fn test_enum_pattern_matching() {
    // Test pattern matching for AcrossLayerPolicy
    match AcrossLayerPolicy::Intersect {
        AcrossLayerPolicy::Intersect => assert!(true),
        _ => assert!(false),
    }
    
    // Test pattern matching for Operator
    match Operator::Sum {
        Operator::Sum => assert!(true),
        _ => assert!(false),
    }
    
    // Test pattern matching for Bucket
    match Bucket::Flat {
        Bucket::Flat => assert!(true),
        _ => assert!(false),
    }
    
    // Test pattern matching for CapMode
    match CapMode::Baseline {
        CapMode::Baseline => assert!(true),
        _ => assert!(false),
    }
}
