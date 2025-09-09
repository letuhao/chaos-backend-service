//! Error types and result definitions for the Chaos World backend.

use thiserror::Error;

/// Main error type for the Chaos World backend.
#[derive(Error, Debug)]
pub enum ChaosError {
    /// Database related errors
    #[error("Database error: {0}")]
    Database(String),

    /// Network related errors
    #[error("Network error: {0}")]
    Network(String),

    /// Authentication/authorization errors
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Internal server errors
    #[error("Internal error: {0}")]
    Internal(String),

    /// External service errors
    #[error("External service error: {0}")]
    ExternalService(String),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic error wrapper
    #[error("Generic error: {0}")]
    Generic(#[from] anyhow::Error),
}

/// Result type alias for the Chaos World backend.
pub type ChaosResult<T> = Result<T, ChaosError>;

impl From<sqlx::Error> for ChaosError {
    fn from(err: sqlx::Error) -> Self {
        ChaosError::Database(err.to_string())
    }
}

impl From<serde_json::Error> for ChaosError {
    fn from(err: serde_json::Error) -> Self {
        ChaosError::Serialization(err.to_string())
    }
}
