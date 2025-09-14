//! Bucket processing and ordering utilities.
//!
//! This module provides centralized functions for processing contributions
//! in the correct bucket order and applying proper clamping.

pub mod optimized;

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
    let bucket_order = [
        Bucket::Flat,
        Bucket::Mult,
        Bucket::PostAdd,
        Bucket::Override,
    ];
    
    for bucket in bucket_order {
        if let Some(mut bucket_contribs) = contributions_by_bucket.remove(&bucket) {
            // Sort contributions deterministically within this bucket
            sort_contributions_deterministic(&mut bucket_contribs);
            
            // Apply bucket-specific processing
            value = apply_bucket_processing(value, bucket, &bucket_contribs);
        }
    }
    
    // Process extra buckets if feature is enabled
    #[cfg(feature = "extra_buckets")]
    {
        let extra_buckets = [
            Bucket::Exponential,
            Bucket::Logarithmic,
            Bucket::Conditional,
        ];
        
        for bucket in extra_buckets {
            if let Some(mut bucket_contribs) = contributions_by_bucket.remove(&bucket) {
                sort_contributions_deterministic(&mut bucket_contribs);
                value = apply_bucket_processing(value, bucket, &bucket_contribs);
            }
        }
    }
    
    // Apply caps if provided
    if let Some(caps) = clamp_caps {
        value = apply_caps(value, caps);
    }
    
    Ok(value)
}

/// Group contributions by bucket type for efficient processing.
fn group_contributions_by_bucket(
    contributions: Vec<Contribution>,
) -> HashMap<Bucket, Vec<Contribution>> {
    let mut groups: HashMap<Bucket, Vec<Contribution>> = HashMap::new();
    
    for contrib in contributions {
        groups.entry(contrib.bucket).or_insert_with(Vec::new).push(contrib);
    }
    
    groups
}

/// Apply bucket-specific processing logic.
fn apply_bucket_processing(
    mut value: f64,
    bucket: Bucket,
    contribs: &[Contribution],
) -> f64 {
    match bucket {
        Bucket::Flat => {
            // Flat: simple addition
            let sum: f64 = contribs.iter().map(|c| c.value).sum();
            value += sum;
        }
        Bucket::Mult => {
            // Mult: multiplication
            for contrib in contribs {
                value *= contrib.value;
            }
        }
        Bucket::PostAdd => {
            // PostAdd: addition after multiplication
            let sum: f64 = contribs.iter().map(|c| c.value).sum();
            value += sum;
        }
        Bucket::Override => {
            // Override: use the last (highest priority) contribution
            if let Some(last_contrib) = contribs.last() {
                value = last_contrib.value;
            }
        }
        #[cfg(feature = "extra_buckets")]
        Bucket::Exponential => {
            // Exponential: value = value^contrib
            for contrib in contribs {
                value = value.powf(contrib.value);
            }
        }
        #[cfg(feature = "extra_buckets")]
        Bucket::Logarithmic => {
            // Logarithmic: value = value * log(contrib + 1)
            for contrib in contribs {
                value *= (contrib.value + 1.0).ln();
            }
        }
        #[cfg(feature = "extra_buckets")]
        Bucket::Conditional => {
            // Conditional: only apply if condition is met
            for contrib in contribs {
                // This would need context to evaluate conditions
                // For now, treat as flat addition
                value += contrib.value;
            }
        }
    }
    value
}

/// Apply caps (min/max constraints) to a value.
fn apply_caps(value: f64, caps: &Caps) -> f64 {
    let mut result = value;
    
    // Apply minimum cap
    if caps.min > result {
        result = caps.min;
    }
    
    // Apply maximum cap
    if caps.max < result {
        result = caps.max;
    }
    
    result
}

/// Get the standard bucket processing order.
/// 
/// Returns the buckets in the order they should be processed:
/// FLAT → MULT → POST_ADD → OVERRIDE
/// 
/// # Returns
/// * `Vec<Bucket>` - The processing order
pub fn get_bucket_processing_order() -> Vec<Bucket> {
    vec![
        Bucket::Flat,
        Bucket::Mult,
        Bucket::PostAdd,
        Bucket::Override,
    ]
}