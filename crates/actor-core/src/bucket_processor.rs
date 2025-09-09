//! Bucket processing and ordering utilities.
//!
//! This module provides centralized functions for processing contributions
//! in the correct bucket order and applying proper clamping.

use std::collections::HashMap;
use crate::enums::Bucket;
use crate::types::{Contribution, Caps};
use crate::ActorCoreResult;

/// Sort contributions deterministically within a bucket.
/// Order: priority DESC (None treated as 0), then system ASC, then value ASC for stability.
fn sort_contributions_deterministic(contribs: &mut Vec<Contribution>) {
    contribs.sort_by(|a, b| {
        let pa = a.priority.unwrap_or(0);
        let pb = b.priority.unwrap_or(0);
        match pb.cmp(&pa) { // DESC by priority
            std::cmp::Ordering::Equal => match a.system.cmp(&b.system) { // ASC by system
                std::cmp::Ordering::Equal => a.value.partial_cmp(&b.value).unwrap_or(std::cmp::Ordering::Equal),
                other => other,
            },
            other => other,
        }
    });
}

/// Process contributions in the correct bucket order.
/// 
/// The order is: FLAT → MULT → POST_ADD → OVERRIDE
/// Extra buckets (if enabled) are processed after core buckets.
/// 
/// # Arguments
/// * `contributions` - Vector of contributions to process
/// * `initial_value` - Starting value for aggregation
/// * `clamp_caps` - Optional caps to apply after processing
/// 
/// # Returns
/// * `ActorCoreResult<f64>` - Final aggregated value
pub fn process_contributions_in_order(
    contributions: Vec<Contribution>,
    initial_value: f64,
    clamp_caps: Option<&Caps>,
) -> ActorCoreResult<f64> {
    let mut value = initial_value;
    
    // Group contributions by bucket type
    let mut contributions_by_bucket = group_contributions_by_bucket(contributions);
    
    // Process in strict order: FLAT → MULT → POST_ADD → OVERRIDE
    // Then extra buckets if feature is enabled
    
    // 1. FLAT contributions
    if let Some(mut flat_contribs) = contributions_by_bucket.remove(&Bucket::Flat) {
        sort_contributions_deterministic(&mut flat_contribs);
        for contrib in flat_contribs {
            value += contrib.value;
        }
    }
    
    // 2. MULT contributions
    if let Some(mut mult_contribs) = contributions_by_bucket.remove(&Bucket::Mult) {
        sort_contributions_deterministic(&mut mult_contribs);
        for contrib in mult_contribs {
            value *= contrib.value;
        }
    }
    
    // 3. POST_ADD contributions
    if let Some(mut post_add_contribs) = contributions_by_bucket.remove(&Bucket::PostAdd) {
        sort_contributions_deterministic(&mut post_add_contribs);
        for contrib in post_add_contribs {
            value += contrib.value;
        }
    }
    
    // 4. OVERRIDE contributions (last core bucket)
    if let Some(mut override_contribs) = contributions_by_bucket.remove(&Bucket::Override) {
        // Deterministic tie-break: priority DESC, system ASC
        sort_contributions_deterministic(&mut override_contribs);
        // Override replaces the value, so pick the highest-priority first element
        if let Some(top_override) = override_contribs.first() {
            value = top_override.value;
        }
    }
    
    // 5. Extra buckets (if feature is enabled)
    #[cfg(feature = "extra_buckets")]
    {
        // Process remaining extra buckets in priority order
        let mut extra_contributions: Vec<_> = contributions_by_bucket.into_iter().collect();
        extra_contributions.sort_by(|a, b| a.0.priority().cmp(&b.0.priority()));
        
        for (bucket, contribs) in extra_contributions {
            for contrib in contribs {
                value = apply_extra_bucket_contribution(value, contrib, bucket)?;
            }
        }
    }
    
    // 6. Apply final clamping
    if let Some(caps) = clamp_caps {
        value = caps.clamp(value);
    }
    
    Ok(value)
}

/// Group contributions by their bucket type.
fn group_contributions_by_bucket(contributions: Vec<Contribution>) -> HashMap<Bucket, Vec<Contribution>> {
    let mut groups = HashMap::new();
    
    for contrib in contributions {
        groups.entry(contrib.bucket).or_insert_with(Vec::new).push(contrib);
    }
    
    groups
}

/// Apply an extra bucket contribution (only available with extra_buckets feature).
#[cfg(feature = "extra_buckets")]
fn apply_extra_bucket_contribution(
    current_value: f64,
    contribution: Contribution,
    bucket: Bucket,
) -> ActorCoreResult<f64> {
    match bucket {
        Bucket::Exponential => {
            Ok(current_value.powf(contribution.value))
        }
        Bucket::Logarithmic => {
            Ok(current_value.log(contribution.value))
        }
        Bucket::Conditional => {
            // For now, treat conditional as flat
            // In a real implementation, this would check conditions
            Ok(current_value + contribution.value)
        }
        _ => {
            // This shouldn't happen as we only call this for extra buckets
            Err(crate::ActorCoreError::ConfigurationError(
                format!("Unexpected bucket type in extra bucket processing: {:?}", bucket)
            ))
        }
    }
}

/// Get the processing order for all available bucket types.
/// 
/// Returns buckets in the order they should be processed.
pub fn get_bucket_processing_order() -> Vec<Bucket> {
    #[allow(unused_mut)] // mut is needed when extra_buckets feature is enabled
    let mut order = vec![
        Bucket::Flat,
        Bucket::Mult,
        Bucket::PostAdd,
        Bucket::Override,
    ];
    
    #[cfg(feature = "extra_buckets")]
    {
        order.extend_from_slice(&[
            Bucket::Exponential,
            Bucket::Logarithmic,
            Bucket::Conditional,
        ]);
    }
    
    order
}

/// Validate that contributions are in a valid state for processing.
pub fn validate_contributions(contributions: &[Contribution]) -> ActorCoreResult<()> {
    for contrib in contributions {
        if contrib.value.is_nan() {
            return Err(crate::ActorCoreError::ConfigurationError(
                format!("Contribution has NaN value: {:?}", contrib)
            ));
        }
        
        if contrib.value.is_infinite() {
            return Err(crate::ActorCoreError::ConfigurationError(
                format!("Contribution has infinite value: {:?}", contrib)
            ));
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Contribution;
    use crate::enums::Bucket;

    #[test]
    fn test_bucket_processing_order() {
        let order = get_bucket_processing_order();
        
        // Core buckets should always be first
        assert_eq!(order[0], Bucket::Flat);
        assert_eq!(order[1], Bucket::Mult);
        assert_eq!(order[2], Bucket::PostAdd);
        assert_eq!(order[3], Bucket::Override);
        
        #[cfg(feature = "extra_buckets")]
        {
            // Extra buckets should be after core buckets
            assert_eq!(order[4], Bucket::Exponential);
            assert_eq!(order[5], Bucket::Logarithmic);
            assert_eq!(order[6], Bucket::Conditional);
        }
    }
    
    #[test]
    fn test_process_contributions_deterministic() {
        let contributions = vec![
            Contribution::new("stat".to_string(), Bucket::Override, 100.0, "test".to_string()),
            Contribution::new("stat".to_string(), Bucket::Flat, 10.0, "test".to_string()),
            Contribution::new("stat".to_string(), Bucket::Mult, 2.0, "test".to_string()),
        ];
        
        let result = process_contributions_in_order(contributions, 0.0, None).unwrap();
        
        // Should process in order: FLAT (0+10=10) → MULT (10*2=20) → OVERRIDE (100)
        assert_eq!(result, 100.0);
    }
    
    #[test]
    fn test_process_contributions_with_clamping() {
        let contributions = vec![
            Contribution::new("stat".to_string(), Bucket::Flat, 150.0, "test".to_string()),
        ];
        
        let caps = Caps::new(0.0, 100.0);
        let result = process_contributions_in_order(contributions, 0.0, Some(&caps)).unwrap();
        
        // Should be clamped to 100.0
        assert_eq!(result, 100.0);
    }
    
    #[test]
    fn test_validation_rejects_nan() {
        let contributions = vec![
            Contribution::new("stat".to_string(), Bucket::Flat, f64::NAN, "test".to_string()),
        ];
        
        let result = validate_contributions(&contributions);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validation_rejects_infinite() {
        let contributions = vec![
            Contribution::new("stat".to_string(), Bucket::Flat, f64::INFINITY, "test".to_string()),
        ];
        
        let result = validate_contributions(&contributions);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validation_accepts_valid() {
        let contributions = vec![
            Contribution::new("stat".to_string(), Bucket::Flat, 10.0, "test".to_string()),
            Contribution::new("stat".to_string(), Bucket::Mult, 2.0, "test".to_string()),
        ];
        
        let result = validate_contributions(&contributions);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_deterministic_ordering_regardless_of_input_order() {
        // Test that the same contributions in different orders produce the same result
        let contributions1 = vec![
            Contribution::new("stat".to_string(), Bucket::Override, 100.0, "test".to_string()),
            Contribution::new("stat".to_string(), Bucket::Flat, 10.0, "test".to_string()),
            Contribution::new("stat".to_string(), Bucket::Mult, 2.0, "test".to_string()),
            Contribution::new("stat".to_string(), Bucket::PostAdd, 5.0, "test".to_string()),
        ];
        
        let contributions2 = vec![
            Contribution::new("stat".to_string(), Bucket::Mult, 2.0, "test".to_string()),
            Contribution::new("stat".to_string(), Bucket::Override, 100.0, "test".to_string()),
            Contribution::new("stat".to_string(), Bucket::PostAdd, 5.0, "test".to_string()),
            Contribution::new("stat".to_string(), Bucket::Flat, 10.0, "test".to_string()),
        ];
        
        let result1 = process_contributions_in_order(contributions1, 0.0, None).unwrap();
        let result2 = process_contributions_in_order(contributions2, 0.0, None).unwrap();
        
        // Both should produce the same result: FLAT(0+10=10) → MULT(10*2=20) → POST_ADD(20+5=25) → OVERRIDE(100)
        assert_eq!(result1, result2);
        assert_eq!(result1, 100.0);
    }
    
    #[test]
    fn test_clamping_applied_after_all_buckets() {
        let contributions = vec![
            Contribution::new("stat".to_string(), Bucket::Flat, 50.0, "test".to_string()),
            Contribution::new("stat".to_string(), Bucket::Mult, 2.0, "test".to_string()),
            Contribution::new("stat".to_string(), Bucket::PostAdd, 10.0, "test".to_string()),
        ];
        
        let caps = Caps::new(0.0, 100.0);
        let result = process_contributions_in_order(contributions, 0.0, Some(&caps)).unwrap();
        
        // Should process: FLAT(0+50=50) → MULT(50*2=100) → POST_ADD(100+10=110) → CLAMP(100)
        assert_eq!(result, 100.0);
    }
}
