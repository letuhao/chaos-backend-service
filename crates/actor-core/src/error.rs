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
}

/// Result type for actor core operations.
pub type ActorCoreResult<T> = Result<T, ActorCoreError>;

#[cfg(test)]
mod tests {
    use super::*;
    use shared::ChaosError;

    #[test]
    fn test_actor_core_error_display() {
        let error = ActorCoreError::InvalidActor("test actor".to_string());
        assert!(error.to_string().contains("Invalid actor: test actor"));

        let error = ActorCoreError::InvalidContribution("test contribution".to_string());
        assert!(error.to_string().contains("Invalid contribution: test contribution"));

        let error = ActorCoreError::InvalidCap("test cap".to_string());
        assert!(error.to_string().contains("Invalid cap: test cap"));

        let error = ActorCoreError::SubsystemError("test subsystem".to_string());
        assert!(error.to_string().contains("Subsystem error: test subsystem"));

        let error = ActorCoreError::CacheError("test cache".to_string());
        assert!(error.to_string().contains("Cache error: test cache"));

        let error = ActorCoreError::RegistryError("test registry".to_string());
        assert!(error.to_string().contains("Registry error: test registry"));

        let error = ActorCoreError::AggregationError("test aggregation".to_string());
        assert!(error.to_string().contains("Aggregation error: test aggregation"));

        let error = ActorCoreError::ConfigurationError("test config".to_string());
        assert!(error.to_string().contains("Configuration error: test config"));

        let error = ActorCoreError::InvalidInput("test input".to_string());
        assert!(error.to_string().contains("Invalid input: test input"));
    }

    #[test]
    fn test_actor_core_error_from_shared() {
        let shared_error = ChaosError::Validation("test".to_string());
        let actor_error: ActorCoreError = shared_error.into();
        
        match actor_error {
            ActorCoreError::Shared(_) => {},
            _ => panic!("Expected Shared variant"),
        }
    }

    #[test]
    fn test_actor_core_error_debug() {
        let error = ActorCoreError::InvalidActor("test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("InvalidActor"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_actor_core_result_ok() {
        let result: ActorCoreResult<i32> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_actor_core_result_err() {
        let result: ActorCoreResult<i32> = Err(ActorCoreError::InvalidActor("test".to_string()));
        assert!(result.is_err());
        match result {
            Err(ActorCoreError::InvalidActor(msg)) => assert_eq!(msg, "test"),
            _ => panic!("Expected InvalidActor error"),
        }
    }

    #[test]
    fn test_actor_core_error_equality() {
        let error1 = ActorCoreError::InvalidActor("test".to_string());
        let error2 = ActorCoreError::InvalidActor("test".to_string());
        let error3 = ActorCoreError::InvalidActor("different".to_string());
        
        // Note: Error types don't implement PartialEq by default
        // This test just ensures they can be created and compared via string representation
        assert_eq!(error1.to_string(), error2.to_string());
        assert_ne!(error1.to_string(), error3.to_string());
    }

    #[test]
    fn test_all_error_variants() {
        let errors = vec![
            ActorCoreError::InvalidActor("test".to_string()),
            ActorCoreError::InvalidContribution("test".to_string()),
            ActorCoreError::InvalidCap("test".to_string()),
            ActorCoreError::SubsystemError("test".to_string()),
            ActorCoreError::CacheError("test".to_string()),
            ActorCoreError::RegistryError("test".to_string()),
            ActorCoreError::AggregationError("test".to_string()),
            ActorCoreError::ConfigurationError("test".to_string()),
            ActorCoreError::InvalidInput("test".to_string()),
        ];

        for error in errors {
            assert!(!error.to_string().is_empty());
        }
    }
}