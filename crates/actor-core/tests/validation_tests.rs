//! Validation tests for Actor Core.
//!
//! This module contains comprehensive tests for the validation system,
//! including contribution validation, cap validation, actor validation,
//! and configuration validation.

use actor_core::prelude::*;
use std::collections::HashMap;
use serde_json::Value;

#[test]
fn test_validate_contribution_valid() {
    let contribution = Contribution::new(
        "strength".to_string(),
        Bucket::Flat,
        10.0,
        "equipment".to_string(),
    );

    let result = validate_contribution(&contribution);
    assert!(result.is_valid);
    assert!(!result.has_errors());
}

#[test]
fn test_validate_contribution_invalid_dimension() {
    let contribution = Contribution::new(
        "".to_string(), // Empty dimension
        Bucket::Flat,
        10.0,
        "equipment".to_string(),
    );

    let result = validate_contribution(&contribution);
    assert!(!result.is_valid);
    assert!(result.has_errors());
    assert!(result.first_error().unwrap().contains("empty"));
}

#[test]
fn test_validate_contribution_invalid_value() {
    let contribution = Contribution::new(
        "strength".to_string(),
        Bucket::Flat,
        f64::INFINITY, // Invalid value
        "equipment".to_string(),
    );

    let result = validate_contribution(&contribution);
    assert!(!result.is_valid);
    assert!(result.has_errors());
    assert!(result.first_error().unwrap().contains("finite"));
}

#[test]
fn test_validate_cap_contribution_valid() {
    let cap_contrib = CapContribution::new(
        "equipment".to_string(),
        "strength".to_string(),
        CapMode::Baseline,
        "max".to_string(),
        100.0,
    );

    let result = validate_cap_contribution(&cap_contrib);
    assert!(result.is_valid);
    assert!(!result.has_errors());
}

#[test]
fn test_validate_cap_contribution_invalid_kind() {
    let cap_contrib = CapContribution::new(
        "equipment".to_string(),
        "strength".to_string(),
        CapMode::Baseline,
        "invalid_kind".to_string(), // Invalid kind
        100.0,
    );

    let result = validate_cap_contribution(&cap_contrib);
    assert!(!result.is_valid);
    assert!(result.has_errors());
    assert!(result.first_error().unwrap().contains("not allowed"));
}

#[test]
fn test_validate_actor_valid() {
    let actor = Actor::new("test".to_string(), "human".to_string());
    let result = validate_actor(&actor);
    assert!(result.is_valid);
    assert!(!result.has_errors());
}

#[test]
fn test_validate_config_valid() {
    let mut config = HashMap::new();
    config.insert("version".to_string(), Value::Number(serde_json::Number::from_f64(1.0).unwrap()));
    config.insert("name".to_string(), Value::String("test_config".to_string()));

    let result = validators::validate_config(&config);
    assert!(result.is_valid);
    assert!(!result.has_errors());
}

#[test]
fn test_validate_config_missing_required() {
    let mut config = HashMap::new();
    config.insert("version".to_string(), Value::Number(serde_json::Number::from_f64(1.0).unwrap()));
    // Missing required "name" field

    let result = validators::validate_config(&config);
    assert!(!result.is_valid);
    assert!(result.has_errors());
    assert!(result.first_error().unwrap().contains("missing"));
}

#[test]
fn test_validate_contributions_multiple() {
    let contributions = vec![
        Contribution::new("strength".to_string(), Bucket::Flat, 10.0, "equipment".to_string()),
        Contribution::new("".to_string(), Bucket::Flat, 20.0, "equipment".to_string()), // Invalid
    ];

    let result = validators::validate_contributions(&contributions);
    assert!(!result.is_valid);
    assert!(result.has_errors());
    assert!(result.first_error().unwrap().contains("contributions[1]"));
}
