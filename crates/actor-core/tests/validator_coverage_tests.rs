//! Comprehensive tests for validation/validator.rs to improve code coverage
//! 
//! This module tests all the validation functionality including:
//! - ValidationRules
//! - ValidationResult
//! - ValidationError
//! - ValidationWarning
//! - Validator
//! - validators module functions

use actor_core::{
    validation::validator::{
        ValidationRules, ValidationResult, ValidationError, ValidationWarning, Validator, validators
    },
    types::{Actor, Contribution, CapContribution, Snapshot},
    enums::{CapMode, Bucket},
    ActorCoreError,
};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

#[cfg(test)]
mod tests {
    use super::*;

    // ValidationRules tests
    #[test]
    fn test_validation_rules_default() {
        let rules = ValidationRules::default();
        assert_eq!(rules.max_dimension_length, 64);
        assert_eq!(rules.max_system_length, 64);
        assert_eq!(rules.min_contribution_value, -1_000_000.0);
        assert_eq!(rules.max_contribution_value, 1_000_000.0);
        assert_eq!(rules.max_tags_per_contribution, 10);
        assert_eq!(rules.max_tag_key_length, 32);
        assert_eq!(rules.max_tag_value_length, 128);
        assert_eq!(rules.max_priority, 1000);
        assert_eq!(rules.min_priority, -1000);
        assert!(!rules.allowed_dimensions.is_empty());
        assert!(!rules.allowed_systems.is_empty());
        assert!(!rules.allowed_cap_kinds.is_empty());
    }

    #[test]
    fn test_validation_rules_custom() {
        let rules = ValidationRules {
            max_dimension_length: 32,
            max_system_length: 32,
            min_contribution_value: -100.0,
            max_contribution_value: 100.0,
            max_tags_per_contribution: 5,
            max_tag_key_length: 16,
            max_tag_value_length: 64,
            max_priority: 100,
            min_priority: -100,
            allowed_dimensions: vec!["test_dim".to_string()],
            allowed_systems: vec!["test_sys".to_string()],
            allowed_cap_kinds: vec!["test_kind".to_string()],
        };
        
        assert_eq!(rules.max_dimension_length, 32);
        assert_eq!(rules.allowed_dimensions.len(), 1);
    }

    // ValidationResult tests
    #[test]
    fn test_validation_result_new() {
        let result = ValidationResult::new();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_validation_result_add_error() {
        let mut result = ValidationResult::new();
        let error = ValidationError::new("TEST_ERROR", "Test error message");
        
        result.add_error(error);
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].code, "TEST_ERROR");
    }

    #[test]
    fn test_validation_result_add_warning() {
        let mut result = ValidationResult::new();
        let warning = ValidationWarning::new("TEST_WARNING", "Test warning message");
        
        result.add_warning(warning);
        assert!(result.is_valid); // Warnings don't make result invalid
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.warnings[0].code, "TEST_WARNING");
    }

    #[test]
    fn test_validation_result_merge() {
        let mut result1 = ValidationResult::new();
        let mut result2 = ValidationResult::new();
        
        result1.add_error(ValidationError::new("ERROR1", "Error 1"));
        result2.add_error(ValidationError::new("ERROR2", "Error 2"));
        result2.add_warning(ValidationWarning::new("WARNING1", "Warning 1"));
        
        result1.merge(result2);
        assert!(!result1.is_valid);
        assert_eq!(result1.errors.len(), 2);
        assert_eq!(result1.warnings.len(), 1);
    }

    #[test]
    fn test_validation_result_has_errors() {
        let mut result = ValidationResult::new();
        assert!(!result.has_errors());
        
        result.add_error(ValidationError::new("ERROR", "Error"));
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_result_has_warnings() {
        let mut result = ValidationResult::new();
        assert!(!result.has_warnings());
        
        result.add_warning(ValidationWarning::new("WARNING", "Warning"));
        assert!(result.has_warnings());
    }

    #[test]
    fn test_validation_result_first_error() {
        let mut result = ValidationResult::new();
        assert!(result.first_error().is_none());
        
        result.add_error(ValidationError::new("ERROR1", "First error"));
        result.add_error(ValidationError::new("ERROR2", "Second error"));
        assert_eq!(result.first_error(), Some("First error".to_string()));
    }

    #[test]
    fn test_validation_result_error_messages() {
        let mut result = ValidationResult::new();
        result.add_error(ValidationError::new("ERROR1", "First error"));
        result.add_error(ValidationError::new("ERROR2", "Second error"));
        
        let messages = result.error_messages();
        assert_eq!(messages.len(), 2);
        assert!(messages.contains(&"First error".to_string()));
        assert!(messages.contains(&"Second error".to_string()));
    }

    #[test]
    fn test_validation_result_to_result_success() {
        let result = ValidationResult::new();
        assert!(result.to_result().is_ok());
    }

    #[test]
    fn test_validation_result_to_result_failure() {
        let mut result = ValidationResult::new();
        result.add_error(ValidationError::new("ERROR", "Test error"));
        
        let actor_result = result.to_result();
        assert!(actor_result.is_err());
        if let Err(ActorCoreError::InvalidInput(msg)) = actor_result {
            assert_eq!(msg, "Test error");
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    // ValidationError tests
    #[test]
    fn test_validation_error_new() {
        let error = ValidationError::new("TEST_CODE", "Test message");
        assert_eq!(error.code, "TEST_CODE");
        assert_eq!(error.message, "Test message");
        assert!(error.field.is_none());
        assert!(error.context.is_none());
    }

    #[test]
    fn test_validation_error_with_field() {
        let error = ValidationError::with_field("TEST_CODE", "Test message", "test_field");
        assert_eq!(error.code, "TEST_CODE");
        assert_eq!(error.message, "Test message");
        assert_eq!(error.field, Some("test_field".to_string()));
        assert!(error.context.is_none());
    }

    #[test]
    fn test_validation_error_with_context() {
        let error = ValidationError::with_context("TEST_CODE", "Test message", "test_field", "test_context");
        assert_eq!(error.code, "TEST_CODE");
        assert_eq!(error.message, "Test message");
        assert_eq!(error.field, Some("test_field".to_string()));
        assert_eq!(error.context, Some("test_context".to_string()));
    }

    // ValidationWarning tests
    #[test]
    fn test_validation_warning_new() {
        let warning = ValidationWarning::new("TEST_CODE", "Test message");
        assert_eq!(warning.code, "TEST_CODE");
        assert_eq!(warning.message, "Test message");
        assert!(warning.field.is_none());
        assert!(warning.context.is_none());
    }

    #[test]
    fn test_validation_warning_with_field() {
        let warning = ValidationWarning::with_field("TEST_CODE", "Test message", "test_field");
        assert_eq!(warning.code, "TEST_CODE");
        assert_eq!(warning.message, "Test message");
        assert_eq!(warning.field, Some("test_field".to_string()));
        assert!(warning.context.is_none());
    }

    // Validator tests
    #[test]
    fn test_validator_new() {
        let validator = Validator::new();
        assert_eq!(validator.rules().max_dimension_length, 64);
    }

    #[test]
    fn test_validator_with_rules() {
        let custom_rules = ValidationRules {
            max_dimension_length: 32,
            max_system_length: 32,
            min_contribution_value: -100.0,
            max_contribution_value: 100.0,
            max_tags_per_contribution: 5,
            max_tag_key_length: 16,
            max_tag_value_length: 64,
            max_priority: 100,
            min_priority: -100,
            allowed_dimensions: vec!["test_dim".to_string()],
            allowed_systems: vec!["test_sys".to_string()],
            allowed_cap_kinds: vec!["test_kind".to_string()],
        };
        
        let validator = Validator::with_rules(custom_rules);
        assert_eq!(validator.rules().max_dimension_length, 32);
    }

    #[test]
    fn test_validator_strict() {
        let validator = Validator::strict();
        // Test that strict validator can be created
        assert_eq!(validator.rules().max_dimension_length, 64);
    }

    #[test]
    fn test_validator_set_strict_mode() {
        let mut validator = Validator::new();
        validator.set_strict_mode(true);
        // Test that strict mode can be set
        assert_eq!(validator.rules().max_dimension_length, 64);
    }

    #[test]
    fn test_validator_set_rules() {
        let mut validator = Validator::new();
        let custom_rules = ValidationRules {
            max_dimension_length: 16,
            max_system_length: 16,
            min_contribution_value: -50.0,
            max_contribution_value: 50.0,
            max_tags_per_contribution: 3,
            max_tag_key_length: 8,
            max_tag_value_length: 32,
            max_priority: 50,
            min_priority: -50,
            allowed_dimensions: vec!["custom_dim".to_string()],
            allowed_systems: vec!["custom_sys".to_string()],
            allowed_cap_kinds: vec!["custom_kind".to_string()],
        };
        
        validator.set_rules(custom_rules);
        assert_eq!(validator.rules().max_dimension_length, 16);
    }

    // Contribution validation tests
    #[test]
    fn test_validate_contribution_valid() {
        let validator = Validator::new();
        let contribution = Contribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            value: 10.0,
            priority: Some(100),
            tags: Some(HashMap::new()),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_contribution_invalid_dimension() {
        let validator = Validator::new();
        let contribution = Contribution {
            dimension: "".to_string(), // Empty dimension
            system: "equipment".to_string(),
            value: 10.0,
            priority: Some(100),
            tags: Some(HashMap::new()),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validate_contribution_invalid_value() {
        let validator = Validator::new();
        let contribution = Contribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            value: 2_000_000.0, // Too high
            priority: Some(100),
            tags: Some(HashMap::new()),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validate_contribution_invalid_priority() {
        let validator = Validator::new();
        let contribution = Contribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            value: 10.0,
            priority: Some(2000), // Too high
            tags: Some(HashMap::new()),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    // CapContribution validation tests
    #[test]
    fn test_validate_cap_contribution_valid() {
        let validator = Validator::new();
        let cap_contrib = CapContribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            kind: "max".to_string(),
            value: 100.0,
            priority: Some(100),
            tags: Some(HashMap::new()),
            mode: CapMode::Override,
            scope: Some("test_scope".to_string()),
            realm: Some("test_realm".to_string()),
        };
        
        let result = validator.validate_cap_contribution(&cap_contrib);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_cap_contribution_invalid_kind() {
        let validator = Validator::new();
        let cap_contrib = CapContribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            kind: "invalid_kind".to_string(), // Not in allowed list
            value: 100.0,
            priority: Some(100),
            tags: Some(HashMap::new()),
            mode: CapMode::Override,
            scope: Some("test_scope".to_string()),
            realm: Some("test_realm".to_string()),
        };
        
        let result = validator.validate_cap_contribution(&cap_contrib);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    // Actor validation tests
    #[test]
    fn test_validate_actor_valid() {
        let validator = Validator::new();
        let actor = Actor {
            id: Uuid::new_v4(),
            name: "Test Actor".to_string(),
            race: "human".to_string(),
            lifespan: 3600,
            age: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 1,
            subsystems: vec![],
            data: HashMap::new(),
        };
        
        let result = validator.validate_actor(&actor);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_actor_empty_id() {
        let validator = Validator::new();
        let actor = Actor {
            id: Uuid::nil(), // Nil UUID (not actually empty string)
            name: "Test Actor".to_string(),
            race: "human".to_string(),
            lifespan: 3600,
            age: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 1,
            subsystems: vec![],
            data: HashMap::new(),
        };
        
        let result = validator.validate_actor(&actor);
        // The nil UUID is not considered empty by the validator
        // since it converts to a string representation
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_actor_empty_race() {
        let validator = Validator::new();
        let actor = Actor {
            id: Uuid::new_v4(),
            name: "Test Actor".to_string(),
            race: "".to_string(), // Empty race
            lifespan: 3600,
            age: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 1,
            subsystems: vec![],
            data: HashMap::new(),
        };
        
        let result = validator.validate_actor(&actor);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    // Snapshot validation tests
    #[test]
    fn test_validate_snapshot_valid() {
        let validator = Validator::new();
        let snapshot = Snapshot::new(Uuid::new_v4(), 1);
        
        let result = validator.validate_snapshot(&snapshot);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_snapshot_empty_actor_id() {
        let validator = Validator::new();
        let snapshot = Snapshot::new(Uuid::nil(), 1); // Nil UUID (not actually empty string)
        
        let result = validator.validate_snapshot(&snapshot);
        // The nil UUID is not considered empty by the validator
        // since it converts to a string representation
        assert!(result.is_valid);
    }

    // Config validation tests
    #[test]
    fn test_validate_config_valid() {
        let validator = Validator::new();
        let mut config = HashMap::new();
        config.insert("version".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
        config.insert("name".to_string(), serde_json::Value::String("test_config".to_string()));
        
        let result = validator.validate_config(&config);
        if !result.is_valid {
            println!("Config validation errors: {:?}", result.error_messages());
        }
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_config_empty_key() {
        let validator = Validator::new();
        let mut config = HashMap::new();
        config.insert("".to_string(), serde_json::Value::String("test_value".to_string()));
        
        let result = validator.validate_config(&config);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    // validators module tests
    #[test]
    fn test_validators_validate_contribution() {
        let contribution = Contribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            value: 10.0,
            priority: Some(100),
            tags: Some(HashMap::new()),
            bucket: Bucket::Flat,
        };
        
        let result = validators::validate_contribution(&contribution);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validators_validate_cap_contribution() {
        let cap_contrib = CapContribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            kind: "max".to_string(),
            value: 100.0,
            priority: Some(100),
            tags: Some(HashMap::new()),
            mode: CapMode::Override,
            scope: Some("test_scope".to_string()),
            realm: Some("test_realm".to_string()),
        };
        
        let result = validators::validate_cap_contribution(&cap_contrib);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validators_validate_actor() {
        let actor = Actor {
            id: Uuid::new_v4(),
            name: "Test Actor".to_string(),
            race: "human".to_string(),
            lifespan: 3600,
            age: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 1,
            subsystems: vec![],
            data: HashMap::new(),
        };
        
        let result = validators::validate_actor(&actor);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validators_validate_snapshot() {
        let snapshot = Snapshot::new(Uuid::new_v4(), 1);
        
        let result = validators::validate_snapshot(&snapshot);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validators_validate_config() {
        let mut config = HashMap::new();
        config.insert("version".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
        config.insert("name".to_string(), serde_json::Value::String("test_config".to_string()));
        
        let result = validators::validate_config(&config);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validators_validate_contributions() {
        let contributions = vec![
            Contribution {
                dimension: "strength".to_string(),
                system: "equipment".to_string(),
                value: 10.0,
                priority: Some(100),
                tags: Some(HashMap::new()),
                bucket: Bucket::Flat,
            },
            Contribution {
                dimension: "agility".to_string(),
                system: "equipment".to_string(),
                value: 15.0,
                priority: Some(200),
                tags: Some(HashMap::new()),
                bucket: Bucket::Flat,
            },
        ];
        
        let result = validators::validate_contributions(&contributions);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validators_validate_contributions_with_errors() {
        let contributions = vec![
            Contribution {
                dimension: "".to_string(), // Invalid: empty dimension
                system: "equipment".to_string(),
                value: 10.0,
                priority: Some(100),
                tags: Some(HashMap::new()),
                bucket: Bucket::Flat,
            },
            Contribution {
                dimension: "agility".to_string(),
                system: "equipment".to_string(),
                value: 15.0,
                priority: Some(200),
                tags: Some(HashMap::new()),
                bucket: Bucket::Flat,
            },
        ];
        
        let result = validators::validate_contributions(&contributions);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validators_validate_cap_contributions() {
        let cap_contribs = vec![
            CapContribution {
                dimension: "strength".to_string(),
                system: "equipment".to_string(),
                kind: "max".to_string(),
                value: 100.0,
                priority: Some(100),
                tags: Some(HashMap::new()),
                mode: CapMode::Override,
                scope: Some("test_scope".to_string()),
                realm: Some("test_realm".to_string()),
            },
            CapContribution {
                dimension: "agility".to_string(),
                system: "equipment".to_string(),
                kind: "min".to_string(),
                value: 50.0,
                priority: Some(200),
                tags: Some(HashMap::new()),
                mode: CapMode::Additive,
                scope: Some("test_scope2".to_string()),
                realm: Some("test_realm2".to_string()),
            },
        ];
        
        let result = validators::validate_cap_contributions(&cap_contribs);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validators_validate_cap_contributions_with_errors() {
        let cap_contribs = vec![
            CapContribution {
                dimension: "".to_string(), // Invalid: empty dimension
                system: "equipment".to_string(),
                kind: "max".to_string(),
                value: 100.0,
                priority: Some(100),
                tags: Some(HashMap::new()),
                mode: CapMode::Override,
                scope: Some("test_scope".to_string()),
                realm: Some("test_realm".to_string()),
            },
            CapContribution {
                dimension: "agility".to_string(),
                system: "equipment".to_string(),
                kind: "min".to_string(),
                value: 50.0,
                priority: Some(200),
                tags: Some(HashMap::new()),
                mode: CapMode::Additive,
                scope: Some("test_scope2".to_string()),
                realm: Some("test_realm2".to_string()),
            },
        ];
        
        let result = validators::validate_cap_contributions(&cap_contribs);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    // Edge cases and error handling
    #[test]
    fn test_validation_with_extreme_values() {
        let validator = Validator::new();
        let contribution = Contribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            value: f64::MAX, // Extreme value
            priority: Some(i64::MAX),
            tags: Some(HashMap::new()),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_with_negative_values() {
        let validator = Validator::new();
        let contribution = Contribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            value: -1_000_001.0, // Below minimum
            priority: Some(-1001), // Below minimum
            tags: Some(HashMap::new()),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_with_long_strings() {
        let validator = Validator::new();
        let long_string = "a".repeat(100);
        let contribution = Contribution {
            dimension: long_string.clone(),
            system: long_string,
            value: 10.0,
            priority: Some(100),
            tags: Some(HashMap::new()),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_with_many_tags() {
        let validator = Validator::new();
        let mut tags = HashMap::new();
        for i in 0..20 { // More than max_tags_per_contribution
            tags.insert(format!("key_{}", i), format!("value_{}", i));
        }
        
        let contribution = Contribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            value: 10.0,
            priority: Some(100),
            tags: Some(tags),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_with_long_tag_values() {
        let validator = Validator::new();
        let mut tags = HashMap::new();
        let long_value = "a".repeat(200); // Longer than max_tag_value_length
        tags.insert("test_key".to_string(), long_value);
        
        let contribution = Contribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            value: 10.0,
            priority: Some(100),
            tags: Some(tags),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_with_long_tag_keys() {
        let validator = Validator::new();
        let mut tags = HashMap::new();
        let long_key = "a".repeat(50); // Longer than max_tag_key_length
        tags.insert(long_key, "test_value".to_string());
        
        let contribution = Contribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            value: 10.0,
            priority: Some(100),
            tags: Some(tags),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_with_disallowed_dimensions() {
        let mut rules = ValidationRules::default();
        rules.allowed_dimensions = vec!["allowed_dim".to_string()];
        let validator = Validator::with_rules(rules);
        
        let contribution = Contribution {
            dimension: "disallowed_dim".to_string(),
            system: "equipment".to_string(),
            value: 10.0,
            priority: Some(100),
            tags: Some(HashMap::new()),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_with_disallowed_systems() {
        let mut rules = ValidationRules::default();
        rules.allowed_systems = vec!["allowed_sys".to_string()];
        let validator = Validator::with_rules(rules);
        
        let contribution = Contribution {
            dimension: "strength".to_string(),
            system: "disallowed_sys".to_string(),
            value: 10.0,
            priority: Some(100),
            tags: Some(HashMap::new()),
            bucket: Bucket::Flat,
        };
        
        let result = validator.validate_contribution(&contribution);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_with_disallowed_cap_kinds() {
        let mut rules = ValidationRules::default();
        rules.allowed_cap_kinds = vec!["allowed_kind".to_string()];
        let validator = Validator::with_rules(rules);
        
        let cap_contrib = CapContribution {
            dimension: "strength".to_string(),
            system: "equipment".to_string(),
            kind: "disallowed_kind".to_string(),
            value: 100.0,
            priority: Some(100),
            tags: Some(HashMap::new()),
            mode: CapMode::Override,
            scope: Some("test_scope".to_string()),
            realm: Some("test_realm".to_string()),
        };
        
        let result = validator.validate_cap_contribution(&cap_contrib);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }
}
