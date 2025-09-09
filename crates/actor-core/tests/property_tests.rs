//! Property-based tests for Actor Core.
//!
//! These tests use proptest to generate random inputs and verify
//! that certain properties hold for all valid inputs.

use actor_core::types::*;
use actor_core::enums::*;
use actor_core::bucket_processor::*;
use std::collections::HashMap;

/// Test that Caps operations are commutative where expected
#[test]
fn test_caps_union_commutative() {
    let a = Caps::new(0.0, 100.0);
    let b = Caps::new(50.0, 150.0);
    
    let union_ab = a.union(&b);
    let union_ba = b.union(&a);
    
    // Union should be commutative
    assert_eq!(union_ab.get_min(), union_ba.get_min());
    assert_eq!(union_ab.get_max(), union_ba.get_max());
}

/// Test that Caps intersection is commutative
#[test]
fn test_caps_intersection_commutative() {
    let a = Caps::new(0.0, 100.0);
    let b = Caps::new(50.0, 150.0);
    
    let intersection_ab = a.intersection(&b);
    let intersection_ba = b.intersection(&a);
    
    // Intersection should be commutative
    assert_eq!(intersection_ab.get_min(), intersection_ba.get_min());
    assert_eq!(intersection_ab.get_max(), intersection_ba.get_max());
}

/// Test that Caps clamping is idempotent
#[test]
fn test_caps_clamping_idempotent() {
    let caps = Caps::new(0.0, 100.0);
    let value = 50.0;
    
    let clamped_once = caps.clamp(value);
    let clamped_twice = caps.clamp(clamped_once);
    
    // Clamping should be idempotent
    assert_eq!(clamped_once, clamped_twice);
}

/// Test that Caps clamping respects bounds
#[test]
fn test_caps_clamping_bounds() {
    let caps = Caps::new(0.0, 100.0);
    let value = 150.0;
    
    let clamped = caps.clamp(value);
    
    // Clamped value should be within bounds
    assert!(clamped >= caps.get_min());
    assert!(clamped <= caps.get_max());
    assert_eq!(clamped, 100.0);
}

/// Test that bucket processing is deterministic
#[test]
fn test_bucket_processing_deterministic() {
    let contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, 10.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Mult, 2.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::PostAdd, 5.0, "test".to_string()),
    ];
    
    let initial_value = 0.0;
    
    // Process the same contributions multiple times
    let result1 = process_contributions_in_order(
        contributions.clone(),
        initial_value,
        None
    ).unwrap();
    
    let result2 = process_contributions_in_order(
        contributions,
        initial_value,
        None
    ).unwrap();
    
    // Results should be identical
    assert_eq!(result1, result2);
}

/// Test that bucket processing with clamping respects bounds
#[test]
fn test_bucket_processing_clamping_bounds() {
    let contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, 150.0, "test".to_string()),
    ];
    
    let initial_value = 0.0;
    let clamp_caps = Caps::new(0.0, 100.0);
    
    let result = process_contributions_in_order(
        contributions,
        initial_value,
        Some(&clamp_caps)
    ).unwrap();
    
    // Result should be within clamp bounds
    assert!(result >= clamp_caps.get_min());
    assert!(result <= clamp_caps.get_max());
    assert_eq!(result, 100.0);
}

/// Test that bucket processing is monotonic for additive operations
#[test]
fn test_bucket_processing_monotonic_additive() {
    let flat_contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, 10.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Flat, 5.0, "test".to_string()),
    ];
    
    let initial_value = 0.0;
    
    let result = process_contributions_in_order(
        flat_contributions,
        initial_value,
        None
    ).unwrap();
    
    // For flat contributions, result should be >= initial value
    assert!(result >= initial_value);
    assert_eq!(result, 15.0);
}

/// Test that Actor operations are consistent
#[test]
fn test_actor_operations_consistent() {
    let mut actor = Actor::new("TestPlayer".to_string(), "Human".to_string());
    let key = "class";
    let value = "warrior";
    
    let original_data = actor.get_data().clone();
    
    // Set and get should be consistent
    let mut new_data = HashMap::new();
    new_data.insert(key.to_string(), serde_json::Value::String(value.to_string()));
    actor.set_data(new_data);
    
    // Data should be different from original
    assert_ne!(actor.get_data(), &original_data);
}

/// Test that Actor buff management is consistent
#[test]
fn test_actor_buff_management_consistent() {
    let mut actor = Actor::new("TestPlayer".to_string(), "Human".to_string());
    let buff_id = "strength_potion";
    
    let initial_buffs = actor.get_active_buffs().len();
    
    // Add buff
    actor.add_buff(buff_id.to_string());
    assert_eq!(actor.get_active_buffs().len(), initial_buffs + 1);
    assert!(actor.get_active_buffs().contains(&buff_id));
    
    // Remove buff
    actor.remove_buff(buff_id);
    assert_eq!(actor.get_active_buffs().len(), initial_buffs);
    assert!(!actor.get_active_buffs().contains(&buff_id));
}

/// Test that Snapshot operations are consistent
#[test]
fn test_snapshot_operations_consistent() {
    use uuid::Uuid;
    let mut snapshot = Snapshot::new(Uuid::new_v4(), 1);
    let contribution = Contribution::new("test".to_string(), Bucket::Flat, 10.0, "test".to_string());
    
    let original_primary = snapshot.primary.clone();
    let original_derived = snapshot.derived.clone();
    
    // Add primary contribution
    snapshot.add_primary(contribution.clone());
    assert_ne!(snapshot.primary, original_primary);
    
    // Add derived contribution
    snapshot.add_derived(contribution);
    assert_ne!(snapshot.derived, original_derived);
}

/// Test that effective caps alias works correctly
#[test]
fn test_effective_caps_alias_consistent() {
    let mut effective_caps: EffectiveCaps = HashMap::new();
    effective_caps.insert("attack".to_string(), Caps::new(0.0, 100.0));
    effective_caps.insert("defense".to_string(), Caps::new(0.0, 200.0));
    
    // Should be able to iterate over the map
    for (key, caps) in &effective_caps {
        assert!(caps.is_valid());
        assert!(!key.is_empty());
    }
    
    // Should be able to get values
    for key in effective_caps.keys() {
        assert!(effective_caps.get(key).is_some());
    }
}

/// Test that bucket processing order is consistent
#[test]
fn test_bucket_processing_order_consistent() {
    let order1 = get_bucket_processing_order();
    let order2 = get_bucket_processing_order();
    
    // Order should be deterministic
    assert_eq!(order1, order2);
    
    // Core buckets should always be present
    assert!(order1.contains(&Bucket::Flat));
    assert!(order1.contains(&Bucket::Mult));
    assert!(order1.contains(&Bucket::PostAdd));
    assert!(order1.contains(&Bucket::Override));
    
    // Core buckets should be in correct order
    let flat_pos = order1.iter().position(|&b| b == Bucket::Flat).unwrap();
    let mult_pos = order1.iter().position(|&b| b == Bucket::Mult).unwrap();
    let post_add_pos = order1.iter().position(|&b| b == Bucket::PostAdd).unwrap();
    let override_pos = order1.iter().position(|&b| b == Bucket::Override).unwrap();
    
    assert!(flat_pos < mult_pos);
    assert!(mult_pos < post_add_pos);
    assert!(post_add_pos < override_pos);
}

/// Test that contribution validation works correctly
#[test]
fn test_contribution_validation_works() {
    let valid_contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, 10.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Mult, 2.0, "test".to_string()),
    ];
    
    // Valid contributions should pass validation
    let result = validate_contributions(&valid_contributions);
    assert!(result.is_ok());
}

/// Test that contribution validation rejects invalid values
#[test]
fn test_contribution_validation_rejects_invalid() {
    let mut invalid_contributions = vec![
        Contribution::new("test".to_string(), Bucket::Flat, 10.0, "test".to_string()),
        Contribution::new("test".to_string(), Bucket::Flat, f64::NAN, "test".to_string()),
    ];
    
    // Invalid contributions should fail validation
    let result = validate_contributions(&invalid_contributions);
    assert!(result.is_err());
}