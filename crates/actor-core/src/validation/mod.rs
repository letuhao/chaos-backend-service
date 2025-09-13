//! Validation module for Actor Core.
//!
//! This module provides comprehensive validation for contributions, caps,
//! configuration data, and other core types to ensure data integrity.

pub mod validator;
pub mod middleware;

// Re-export the main validation types and functions
pub use validator::{
    Validator,
    ValidationRules,
    ValidationResult,
    ValidationError,
    ValidationWarning,
    validators,
};

// Re-export middleware types
pub use middleware::{
    ValidationMiddleware,
    AggregatorValidationMiddleware,
    CacheValidationMiddleware,
    RegistryValidationMiddleware,
    ValidationMiddlewareFactory,
    ValidationStats,
};