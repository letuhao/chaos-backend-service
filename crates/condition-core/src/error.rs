//! Error types for Condition Core

use thiserror::Error;

/// Errors that can occur during condition resolution
#[derive(Error, Debug)]
pub enum ConditionError {
    #[error("Function not found: {function_name}")]
    FunctionNotFound { function_name: String },

    #[error("Invalid parameter: {parameter} for function: {function_name}")]
    InvalidParameter { 
        function_name: String, 
        parameter: String 
    },

    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    #[error("Context error: {message}")]
    ContextError { message: String },

    #[error("Chain logic error: {message}")]
    ChainLogicError { message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Unknown error: {message}")]
    Unknown { message: String },
}

/// Result type for condition operations
pub type ConditionResult<T> = Result<T, ConditionError>;
