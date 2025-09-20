//! Validation module for Actor Core.
//!
//! This module provides comprehensive validation for contributions, caps,
//! configuration data, and other core types to ensure data integrity.
//!
//! NOTE: Legacy hardcoded validator has been moved to examples/legacy_subsystems/
//! Use DynamicValidator for configuration-based validation.

pub mod dynamic_validator;
pub mod middleware;

// Re-export the main validation types and functions
pub use dynamic_validator::{
    DynamicValidator,
    ValidationRules,
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