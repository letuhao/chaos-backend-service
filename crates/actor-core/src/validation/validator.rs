//! Centralized validation for Actor Core.
//!
//! This module provides comprehensive validation for contributions, caps,
//! configuration data, and other core types to ensure data integrity.

use std::collections::HashMap;
use serde_json::Value;
use tracing::{warn, error, debug};

use crate::types::*;
use crate::enums::*;
use crate::ActorCoreError;
use crate::ActorCoreResult;

/// Validation rules and constraints for the Actor Core system.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationRules {
    /// Maximum length for dimension names
    pub max_dimension_length: usize,
    /// Maximum length for system names
    pub max_system_length: usize,
    /// Minimum value for contributions
    pub min_contribution_value: f64,
    /// Maximum value for contributions
    pub max_contribution_value: f64,
    /// Maximum number of tags per contribution
    pub max_tags_per_contribution: usize,
    /// Maximum tag key length
    pub max_tag_key_length: usize,
    /// Maximum tag value length
    pub max_tag_value_length: usize,
    /// Maximum priority value
    pub max_priority: i64,
    /// Minimum priority value
    pub min_priority: i64,
    /// Allowed dimension names (if empty, all are allowed)
    pub allowed_dimensions: Vec<String>,
    /// Allowed system names (if empty, all are allowed)
    pub allowed_systems: Vec<String>,
    /// Allowed cap kinds
    pub allowed_cap_kinds: Vec<String>,
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self {
            max_dimension_length: 64,
            max_system_length: 64,
            min_contribution_value: -1_000_000.0,
            max_contribution_value: 1_000_000.0,
            max_tags_per_contribution: 10,
            max_tag_key_length: 32,
            max_tag_value_length: 128,
            max_priority: 1000,
            min_priority: -1000,
            allowed_dimensions: vec![
                "strength".to_string(),
                "agility".to_string(),
                "intelligence".to_string(),
                "health".to_string(),
                "mana".to_string(),
                "stamina".to_string(),
                "defense".to_string(),
                "attack".to_string(),
                "speed".to_string(),
                "luck".to_string(),
            ],
            allowed_systems: vec![
                "equipment".to_string(),
                "buff".to_string(),
                "talent".to_string(),
                "skill".to_string(),
                "item".to_string(),
                "guild".to_string(),
                "event".to_string(),
                "world".to_string(),
            ],
            allowed_cap_kinds: vec![
                "min".to_string(),
                "max".to_string(),
            ],
        }
    }
}

/// Validation result with detailed error information.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    /// Whether the validation passed
    pub is_valid: bool,
    /// List of validation errors
    pub errors: Vec<ValidationError>,
    /// List of validation warnings
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationResult {
    /// Create a new validation result.
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Add an error to the validation result.
    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
        self.is_valid = false;
    }

    /// Add a warning to the validation result.
    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }

    /// Check if there are any errors.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Check if there are any warnings.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Get the first error message.
    pub fn first_error(&self) -> Option<String> {
        self.errors.first().map(|e| e.message.clone())
    }

    /// Get all error messages.
    pub fn error_messages(&self) -> Vec<String> {
        self.errors.iter().map(|e| e.message.clone()).collect()
    }

    /// Convert to ActorCoreResult.
    pub fn to_result(self) -> ActorCoreResult<()> {
        if self.is_valid {
            Ok(())
        } else {
            Err(ActorCoreError::InvalidInput(self.first_error().unwrap_or_else(|| "Validation failed".to_string())))
        }
    }
}

/// Validation error with context.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    /// Error code for programmatic handling
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Field that caused the error
    pub field: Option<String>,
    /// Additional context
    pub context: Option<String>,
}

impl ValidationError {
    /// Create a new validation error.
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            field: None,
            context: None,
        }
    }

    /// Create a new validation error with field context.
    pub fn with_field(code: &str, message: &str, field: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            field: Some(field.to_string()),
            context: None,
        }
    }

    /// Create a new validation error with full context.
    pub fn with_context(code: &str, message: &str, field: &str, context: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            field: Some(field.to_string()),
            context: Some(context.to_string()),
        }
    }
}

/// Validation warning with context.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationWarning {
    /// Warning code for programmatic handling
    pub code: String,
    /// Human-readable warning message
    pub message: String,
    /// Field that caused the warning
    pub field: Option<String>,
    /// Additional context
    pub context: Option<String>,
}

impl ValidationWarning {
    /// Create a new validation warning.
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            field: None,
            context: None,
        }
    }

    /// Create a new validation warning with field context.
    pub fn with_field(code: &str, message: &str, field: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            field: Some(field.to_string()),
            context: None,
        }
    }
}

/// Central validation service for Actor Core.
pub struct Validator {
    /// Validation rules
    rules: ValidationRules,
    /// Enable strict validation (treat warnings as errors)
    strict_mode: bool,
}

impl Validator {
    /// Create a new validator with default rules.
    pub fn new() -> Self {
        Self {
            rules: ValidationRules::default(),
            strict_mode: false,
        }
    }

    /// Create a new validator with custom rules.
    pub fn with_rules(rules: ValidationRules) -> Self {
        Self {
            rules,
            strict_mode: false,
        }
    }

    /// Create a new validator in strict mode.
    pub fn strict() -> Self {
        Self {
            rules: ValidationRules::default(),
            strict_mode: true,
        }
    }

    /// Set strict mode.
    pub fn set_strict_mode(&mut self, strict: bool) {
        self.strict_mode = strict;
    }

    /// Get the current validation rules.
    pub fn rules(&self) -> &ValidationRules {
        &self.rules
    }

    /// Update validation rules.
    pub fn set_rules(&mut self, rules: ValidationRules) {
        self.rules = rules;
    }

    /// Validate a contribution.
    pub fn validate_contribution(&self, contribution: &Contribution) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate dimension
        self.validate_dimension(&contribution.dimension, "dimension", &mut result);

        // Validate system
        self.validate_system(&contribution.system, "system", &mut result);

        // Validate value
        self.validate_contribution_value(contribution.value, "value", &mut result);

        // Validate priority
        if let Some(priority) = contribution.priority {
            self.validate_priority(priority, "priority", &mut result);
        }

        // Validate tags
        if let Some(tags) = &contribution.tags {
            self.validate_tags(tags, "tags", &mut result);
        }

        // Validate bucket
        self.validate_bucket(contribution.bucket, &mut result);

        result
    }

    /// Validate a cap contribution.
    pub fn validate_cap_contribution(&self, cap_contrib: &CapContribution) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate dimension
        self.validate_dimension(&cap_contrib.dimension, "dimension", &mut result);

        // Validate system
        self.validate_system(&cap_contrib.system, "system", &mut result);

        // Validate cap kind
        self.validate_cap_kind(&cap_contrib.kind, "kind", &mut result);

        // Validate value
        self.validate_contribution_value(cap_contrib.value, "value", &mut result);

        // Validate priority
        if let Some(priority) = cap_contrib.priority {
            self.validate_priority(priority, "priority", &mut result);
        }

        // Validate tags
        if let Some(tags) = &cap_contrib.tags {
            self.validate_tags(tags, "tags", &mut result);
        }

        // Validate cap mode
        self.validate_cap_mode(cap_contrib.mode, &mut result);

        // Validate scope
        if let Some(scope) = &cap_contrib.scope {
            self.validate_string_length(scope, self.rules.max_system_length, "scope", &mut result);
        }

        // Validate realm
        if let Some(realm) = &cap_contrib.realm {
            self.validate_string_length(realm, self.rules.max_system_length, "realm", &mut result);
        }

        result
    }

    /// Validate an actor.
    pub fn validate_actor(&self, actor: &Actor) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate actor ID
        if actor.id.to_string().is_empty() {
            result.add_error(ValidationError::with_field(
                "EMPTY_ACTOR_ID",
                "Actor ID cannot be empty",
                "id",
            ));
        }

        // Validate race
        if actor.race.is_empty() {
            result.add_error(ValidationError::with_field(
                "EMPTY_RACE",
                "Actor race cannot be empty",
                "race",
            ));
        }

        // Validate data
        if let Some(data) = &actor.data {
            self.validate_actor_data(data, &mut result);
        }

        // Validate subsystems
        for (i, subsystem) in actor.subsystems.iter().enumerate() {
            self.validate_subsystem_meta(subsystem, &format!("subsystems[{}]", i), &mut result);
        }

        result
    }

    /// Validate a snapshot.
    pub fn validate_snapshot(&self, snapshot: &Snapshot) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate actor ID
        if snapshot.actor_id.to_string().is_empty() {
            result.add_error(ValidationError::with_field(
                "EMPTY_ACTOR_ID",
                "Snapshot actor ID cannot be empty",
                "actor_id",
            ));
        }

        // Validate version
        if snapshot.version < 0 {
            result.add_error(ValidationError::with_field(
                "INVALID_VERSION",
                "Snapshot version must be non-negative",
                "version",
            ));
        }

        // Validate primary stats
        for (dimension, value) in &snapshot.primary {
            self.validate_dimension(dimension, &format!("primary.{}", dimension), &mut result);
            self.validate_contribution_value(*value, &format!("primary.{}", dimension), &mut result);
        }

        // Validate derived stats
        for (dimension, value) in &snapshot.derived {
            self.validate_dimension(dimension, &format!("derived.{}", dimension), &mut result);
            self.validate_contribution_value(*value, &format!("derived.{}", dimension), &mut result);
        }

        // Validate caps
        for (dimension, caps) in &snapshot.caps_used {
            self.validate_dimension(dimension, &format!("caps_used.{}", dimension), &mut result);
            self.validate_caps(caps, &format!("caps_used.{}", dimension), &mut result);
        }

        result
    }

    /// Validate caps.
    pub fn validate_caps(&self, caps: &Caps, field_name: &str) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate min cap
        if let Some(min) = caps.min {
            if !min.is_finite() {
                result.add_error(ValidationError::with_field(
                    "INVALID_MIN_CAP",
                    "Min cap must be finite",
                    &format!("{}.min", field_name),
                ));
            }
        }

        // Validate max cap
        if let Some(max) = caps.max {
            if !max.is_finite() {
                result.add_error(ValidationError::with_field(
                    "INVALID_MAX_CAP",
                    "Max cap must be finite",
                    &format!("{}.max", field_name),
                ));
            }
        }

        // Validate min <= max
        if let (Some(min), Some(max)) = (caps.min, caps.max) {
            if min > max {
                result.add_error(ValidationError::with_field(
                    "INVALID_CAP_RANGE",
                    "Min cap cannot be greater than max cap",
                    field_name,
                ));
            }
        }

        result
    }

    /// Validate configuration data.
    pub fn validate_config(&self, config: &HashMap<String, Value>) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Check for required fields
        let required_fields = ["version", "name"];
        for field in required_fields {
            if !config.contains_key(field) {
                result.add_error(ValidationError::with_field(
                    "MISSING_REQUIRED_FIELD",
                    &format!("Required field '{}' is missing", field),
                    field,
                ));
            }
        }

        // Validate version
        if let Some(version) = config.get("version") {
            if let Some(version_num) = version.as_f64() {
                if version_num < 1.0 {
                    result.add_error(ValidationError::with_field(
                        "INVALID_VERSION",
                        "Version must be >= 1.0",
                        "version",
                    ));
                }
            } else {
                result.add_error(ValidationError::with_field(
                    "INVALID_VERSION_TYPE",
                    "Version must be a number",
                    "version",
                ));
            }
        }

        // Validate name
        if let Some(name) = config.get("name") {
            if let Some(name_str) = name.as_str() {
                if name_str.is_empty() {
                    result.add_error(ValidationError::with_field(
                        "EMPTY_NAME",
                        "Configuration name cannot be empty",
                        "name",
                    ));
                }
            } else {
                result.add_error(ValidationError::with_field(
                    "INVALID_NAME_TYPE",
                    "Configuration name must be a string",
                    "name",
                ));
            }
        }

        result
    }

    // Private validation helper methods

    fn validate_dimension(&self, dimension: &str, field_name: &str, result: &mut ValidationResult) {
        if dimension.is_empty() {
            result.add_error(ValidationError::with_field(
                "EMPTY_DIMENSION",
                "Dimension name cannot be empty",
                field_name,
            ));
            return;
        }

        if dimension.len() > self.rules.max_dimension_length {
            result.add_error(ValidationError::with_field(
                "DIMENSION_TOO_LONG",
                &format!("Dimension name cannot exceed {} characters", self.rules.max_dimension_length),
                field_name,
            ));
        }

        if !self.rules.allowed_dimensions.is_empty() && !self.rules.allowed_dimensions.contains(&dimension.to_string()) {
            result.add_error(ValidationError::with_field(
                "INVALID_DIMENSION",
                &format!("Dimension '{}' is not allowed", dimension),
                field_name,
            ));
        }

        // Check for valid characters (alphanumeric and underscore only)
        if !dimension.chars().all(|c| c.is_alphanumeric() || c == '_') {
            result.add_error(ValidationError::with_field(
                "INVALID_DIMENSION_CHARS",
                "Dimension name can only contain alphanumeric characters and underscores",
                field_name,
            ));
        }
    }

    fn validate_system(&self, system: &str, field_name: &str, result: &mut ValidationResult) {
        if system.is_empty() {
            result.add_error(ValidationError::with_field(
                "EMPTY_SYSTEM",
                "System name cannot be empty",
                field_name,
            ));
            return;
        }

        if system.len() > self.rules.max_system_length {
            result.add_error(ValidationError::with_field(
                "SYSTEM_TOO_LONG",
                &format!("System name cannot exceed {} characters", self.rules.max_system_length),
                field_name,
            ));
        }

        if !self.rules.allowed_systems.is_empty() && !self.rules.allowed_systems.contains(&system.to_string()) {
            result.add_error(ValidationError::with_field(
                "INVALID_SYSTEM",
                &format!("System '{}' is not allowed", system),
                field_name,
            ));
        }

        // Check for valid characters (alphanumeric and underscore only)
        if !system.chars().all(|c| c.is_alphanumeric() || c == '_') {
            result.add_error(ValidationError::with_field(
                "INVALID_SYSTEM_CHARS",
                "System name can only contain alphanumeric characters and underscores",
                field_name,
            ));
        }
    }

    fn validate_contribution_value(&self, value: f64, field_name: &str, result: &mut ValidationResult) {
        if !value.is_finite() {
            result.add_error(ValidationError::with_field(
                "INVALID_VALUE",
                "Contribution value must be finite",
                field_name,
            ));
            return;
        }

        if value < self.rules.min_contribution_value {
            result.add_error(ValidationError::with_field(
                "VALUE_TOO_SMALL",
                &format!("Contribution value cannot be less than {}", self.rules.min_contribution_value),
                field_name,
            ));
        }

        if value > self.rules.max_contribution_value {
            result.add_error(ValidationError::with_field(
                "VALUE_TOO_LARGE",
                &format!("Contribution value cannot be greater than {}", self.rules.max_contribution_value),
                field_name,
            ));
        }
    }

    fn validate_priority(&self, priority: i64, field_name: &str, result: &mut ValidationResult) {
        if priority < self.rules.min_priority {
            result.add_error(ValidationError::with_field(
                "PRIORITY_TOO_SMALL",
                &format!("Priority cannot be less than {}", self.rules.min_priority),
                field_name,
            ));
        }

        if priority > self.rules.max_priority {
            result.add_error(ValidationError::with_field(
                "PRIORITY_TOO_LARGE",
                &format!("Priority cannot be greater than {}", self.rules.max_priority),
                field_name,
            ));
        }
    }

    fn validate_tags(&self, tags: &HashMap<String, String>, field_name: &str, result: &mut ValidationResult) {
        if tags.len() > self.rules.max_tags_per_contribution {
            result.add_error(ValidationError::with_field(
                "TOO_MANY_TAGS",
                &format!("Cannot have more than {} tags", self.rules.max_tags_per_contribution),
                field_name,
            ));
        }

        for (key, value) in tags {
            if key.len() > self.rules.max_tag_key_length {
                result.add_error(ValidationError::with_field(
                    "TAG_KEY_TOO_LONG",
                    &format!("Tag key cannot exceed {} characters", self.rules.max_tag_key_length),
                    &format!("{}.{}", field_name, key),
                ));
            }

            if value.len() > self.rules.max_tag_value_length {
                result.add_error(ValidationError::with_field(
                    "TAG_VALUE_TOO_LONG",
                    &format!("Tag value cannot exceed {} characters", self.rules.max_tag_value_length),
                    &format!("{}.{}", field_name, key),
                ));
            }
        }
    }

    fn validate_bucket(&self, bucket: Bucket, result: &mut ValidationResult) {
        // Bucket validation is currently just a placeholder
        // In the future, we might want to validate against allowed buckets
        match bucket {
            Bucket::Flat | Bucket::Mult | Bucket::PostAdd | Bucket::Override => {
                // These are always valid
            }
            #[cfg(feature = "extra_buckets")]
            Bucket::Exponential | Bucket::Logarithmic | Bucket::Conditional => {
                // These are valid when extra_buckets feature is enabled
            }
        }
    }

    fn validate_cap_kind(&self, kind: &str, field_name: &str, result: &mut ValidationResult) {
        if !self.rules.allowed_cap_kinds.contains(&kind.to_string()) {
            result.add_error(ValidationError::with_field(
                "INVALID_CAP_KIND",
                &format!("Cap kind '{}' is not allowed. Allowed kinds: {:?}", kind, self.rules.allowed_cap_kinds),
                field_name,
            ));
        }
    }

    fn validate_cap_mode(&self, mode: CapMode, result: &mut ValidationResult) {
        // CapMode validation is currently just a placeholder
        // All enum variants are considered valid
        match mode {
            CapMode::Baseline | CapMode::Additive | CapMode::HardMax | 
            CapMode::HardMin | CapMode::Override | CapMode::SoftMax => {
                // These are always valid
            }
        }
    }

    fn validate_string_length(&self, s: &str, max_length: usize, field_name: &str, result: &mut ValidationResult) {
        if s.len() > max_length {
            result.add_error(ValidationError::with_field(
                "STRING_TOO_LONG",
                &format!("String cannot exceed {} characters", max_length),
                field_name,
            ));
        }
    }

    fn validate_actor_data(&self, data: &HashMap<String, Value>, result: &mut ValidationResult) {
        for (key, value) in data {
            if key.is_empty() {
                result.add_error(ValidationError::with_field(
                    "EMPTY_DATA_KEY",
                    "Actor data key cannot be empty",
                    &format!("data.{}", key),
                ));
            }

            if key.len() > self.rules.max_dimension_length {
                result.add_error(ValidationError::with_field(
                    "DATA_KEY_TOO_LONG",
                    &format!("Actor data key cannot exceed {} characters", self.rules.max_dimension_length),
                    &format!("data.{}", key),
                ));
            }

            // Validate value types
            match value {
                Value::String(s) => {
                    if s.len() > 1000 { // Arbitrary limit for string values
                        result.add_error(ValidationError::with_field(
                            "DATA_VALUE_TOO_LONG",
                            "Actor data string value cannot exceed 1000 characters",
                            &format!("data.{}", key),
                        ));
                    }
                }
                Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        if !f.is_finite() {
                            result.add_error(ValidationError::with_field(
                                "INVALID_DATA_VALUE",
                                "Actor data number value must be finite",
                                &format!("data.{}", key),
                            ));
                        }
                    }
                }
                _ => {
                    // Other types are allowed but might generate warnings
                    if self.strict_mode {
                        result.add_error(ValidationError::with_field(
                            "UNSUPPORTED_DATA_TYPE",
                            &format!("Actor data type {:?} is not recommended", value),
                            &format!("data.{}", key),
                        ));
                    } else {
                        result.add_warning(ValidationWarning::with_field(
                            "UNSUPPORTED_DATA_TYPE",
                            &format!("Actor data type {:?} is not recommended", value),
                            &format!("data.{}", key),
                        ));
                    }
                }
            }
        }
    }

    fn validate_subsystem_meta(&self, meta: &SubsystemMeta, field_name: &str, result: &mut ValidationResult) {
        self.validate_system(&meta.system, &format!("{}.system", field_name), result);
        
        // Validate data if present
        if let Some(data) = &meta.data {
            for (key, value) in data {
                if key.is_empty() {
                    result.add_error(ValidationError::with_field(
                        "EMPTY_SUBSYSTEM_DATA_KEY",
                        "Subsystem data key cannot be empty",
                        &format!("{}.data.{}", field_name, key),
                    ));
                }
            }
        }
    }
}

/// Convenience functions for common validation tasks.
pub mod validators {
    use super::*;

    /// Validate a single contribution.
    pub fn validate_contribution(contribution: &Contribution) -> ValidationResult {
        let validator = Validator::new();
        validator.validate_contribution(contribution)
    }

    /// Validate a single cap contribution.
    pub fn validate_cap_contribution(cap_contrib: &CapContribution) -> ValidationResult {
        let validator = Validator::new();
        validator.validate_cap_contribution(cap_contrib)
    }

    /// Validate an actor.
    pub fn validate_actor(actor: &Actor) -> ValidationResult {
        let validator = Validator::new();
        validator.validate_actor(actor)
    }

    /// Validate a snapshot.
    pub fn validate_snapshot(snapshot: &Snapshot) -> ValidationResult {
        let validator = Validator::new();
        validator.validate_snapshot(snapshot)
    }

    /// Validate configuration data.
    pub fn validate_config(config: &HashMap<String, Value>) -> ValidationResult {
        let validator = Validator::new();
        validator.validate_config(config)
    }

    /// Validate multiple contributions.
    pub fn validate_contributions(contributions: &[Contribution]) -> ValidationResult {
        let mut result = ValidationResult::new();
        let validator = Validator::new();

        for (i, contribution) in contributions.iter().enumerate() {
            let mut contrib_result = validator.validate_contribution(contribution);
            if !contrib_result.is_valid {
                for error in contrib_result.errors {
                    result.add_error(ValidationError::with_context(
                        &error.code,
                        &error.message,
                        &format!("contributions[{}].{}", i, error.field.unwrap_or_default()),
                        &error.context.unwrap_or_default(),
                    ));
                }
            }
            result.warnings.extend(contrib_result.warnings);
        }

        result
    }

    /// Validate multiple cap contributions.
    pub fn validate_cap_contributions(cap_contribs: &[CapContribution]) -> ValidationResult {
        let mut result = ValidationResult::new();
        let validator = Validator::new();

        for (i, cap_contrib) in cap_contribs.iter().enumerate() {
            let mut contrib_result = validator.validate_cap_contribution(cap_contrib);
            if !contrib_result.is_valid {
                for error in contrib_result.errors {
                    result.add_error(ValidationError::with_context(
                        &error.code,
                        &error.message,
                        &format!("cap_contributions[{}].{}", i, error.field.unwrap_or_default()),
                        &error.context.unwrap_or_default(),
                    ));
                }
            }
            result.warnings.extend(contrib_result.warnings);
        }

        result
    }
}