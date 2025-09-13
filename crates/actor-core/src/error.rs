//! Error types specific to the actor-core module.

use thiserror::Error;
use shared::ChaosError;

/// Actor core specific errors.
#[derive(Error, Debug)]
pub enum ActorCoreError {
    /// Invalid actor data
    #[error("Invalid actor: {0}")]
    InvalidActor(String),

    /// Invalid contribution data
    #[error("Invalid contribution: {0}")]
    InvalidContribution(String),

    /// Invalid cap data
    #[error("Invalid cap: {0}")]
    InvalidCap(String),

    /// Subsystem error
    #[error("Subsystem error: {0}")]
    SubsystemError(String),

    /// Cache error
    #[error("Cache error: {0}")]
    CacheError(String),

    /// Registry error
    #[error("Registry error: {0}")]
    RegistryError(String),

    /// Aggregation error
    #[error("Aggregation error: {0}")]
    AggregationError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Invalid input error
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Wrapper for shared errors
    #[error(transparent)]
    Shared(#[from] ChaosError),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Result type for actor core operations.
pub type ActorCoreResult<T> = Result<T, ActorCoreError>;