//! Error types for API Gateway

use std::fmt;
use std::error::Error as StdError;

/// API Gateway result type
pub type Result<T> = std::result::Result<T, ApiGatewayError>;

/// API Gateway error types
#[derive(Debug)]
pub enum ApiGatewayError {
    /// Configuration error
    Config(String),
    
    /// Server error
    Server(String),
    
    /// Routing error
    Routing(String),
    
    /// Authentication error
    Auth(String),
    
    /// Authorization error
    Authorization(String),
    
    /// Rate limiting error
    RateLimit(String),
    
    /// Service discovery error
    ServiceDiscovery(String),
    
    /// Load balancing error
    LoadBalancing(String),
    
    /// Caching error
    Caching(String),
    
    /// Monitoring error
    Monitoring(String),
    
    /// Security error
    Security(String),
    
    /// Network error
    Network(String),
    
    /// Timeout error
    Timeout(String),
    
    /// Circuit breaker error
    CircuitBreaker(String),
    
    /// Validation error
    Validation(String),
    
    /// Serialization error
    Serialization(String),
    
    /// IO error
    Io(std::io::Error),
    
    /// HTTP error
    Http(String),
    
    /// Database error
    Database(String),
    
    /// External service error
    ExternalService(String),
    
    /// Internal error
    Internal(String),
}

impl fmt::Display for ApiGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiGatewayError::Config(msg) => write!(f, "Configuration error: {}", msg),
            ApiGatewayError::Server(msg) => write!(f, "Server error: {}", msg),
            ApiGatewayError::Routing(msg) => write!(f, "Routing error: {}", msg),
            ApiGatewayError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            ApiGatewayError::Authorization(msg) => write!(f, "Authorization error: {}", msg),
            ApiGatewayError::RateLimit(msg) => write!(f, "Rate limiting error: {}", msg),
            ApiGatewayError::ServiceDiscovery(msg) => write!(f, "Service discovery error: {}", msg),
            ApiGatewayError::LoadBalancing(msg) => write!(f, "Load balancing error: {}", msg),
            ApiGatewayError::Caching(msg) => write!(f, "Caching error: {}", msg),
            ApiGatewayError::Monitoring(msg) => write!(f, "Monitoring error: {}", msg),
            ApiGatewayError::Security(msg) => write!(f, "Security error: {}", msg),
            ApiGatewayError::Network(msg) => write!(f, "Network error: {}", msg),
            ApiGatewayError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
            ApiGatewayError::CircuitBreaker(msg) => write!(f, "Circuit breaker error: {}", msg),
            ApiGatewayError::Validation(msg) => write!(f, "Validation error: {}", msg),
            ApiGatewayError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            ApiGatewayError::Io(err) => write!(f, "IO error: {}", err),
            ApiGatewayError::Http(msg) => write!(f, "HTTP error: {}", msg),
            ApiGatewayError::Database(msg) => write!(f, "Database error: {}", msg),
            ApiGatewayError::ExternalService(msg) => write!(f, "External service error: {}", msg),
            ApiGatewayError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl StdError for ApiGatewayError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ApiGatewayError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ApiGatewayError {
    fn from(err: std::io::Error) -> Self {
        ApiGatewayError::Io(err)
    }
}

impl From<serde_yaml::Error> for ApiGatewayError {
    fn from(err: serde_yaml::Error) -> Self {
        ApiGatewayError::Serialization(format!("YAML serialization error: {}", err))
    }
}

impl From<serde_json::Error> for ApiGatewayError {
    fn from(err: serde_json::Error) -> Self {
        ApiGatewayError::Serialization(format!("JSON serialization error: {}", err))
    }
}

impl From<tokio::time::error::Elapsed> for ApiGatewayError {
    fn from(err: tokio::time::error::Elapsed) -> Self {
        ApiGatewayError::Timeout(format!("Operation timed out: {}", err))
    }
}

impl From<reqwest::Error> for ApiGatewayError {
    fn from(err: reqwest::Error) -> Self {
        ApiGatewayError::Http(format!("HTTP client error: {}", err))
    }
}

impl From<redis::RedisError> for ApiGatewayError {
    fn from(err: redis::RedisError) -> Self {
        ApiGatewayError::Database(format!("Redis error: {}", err))
    }
}

impl From<jsonwebtoken::errors::Error> for ApiGatewayError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        ApiGatewayError::Auth(format!("JWT error: {}", err))
    }
}

impl From<axum::Error> for ApiGatewayError {
    fn from(err: axum::Error) -> Self {
        ApiGatewayError::Server(format!("Axum error: {}", err))
    }
}

impl From<tower::BoxError> for ApiGatewayError {
    fn from(err: tower::BoxError) -> Self {
        ApiGatewayError::Server(format!("Tower error: {}", err))
    }
}

impl From<hyper::Error> for ApiGatewayError {
    fn from(err: hyper::Error) -> Self {
        ApiGatewayError::Network(format!("Hyper error: {}", err))
    }
}

impl From<tonic::transport::Error> for ApiGatewayError {
    fn from(err: tonic::transport::Error) -> Self {
        ApiGatewayError::Network(format!("gRPC transport error: {}", err))
    }
}

impl From<tonic::Status> for ApiGatewayError {
    fn from(err: tonic::Status) -> Self {
        ApiGatewayError::ExternalService(format!("gRPC service error: {}", err))
    }
}

/// HTTP status code for API Gateway errors
impl ApiGatewayError {
    pub fn status_code(&self) -> u16 {
        match self {
            ApiGatewayError::Config(_) => 500,
            ApiGatewayError::Server(_) => 500,
            ApiGatewayError::Routing(_) => 404,
            ApiGatewayError::Auth(_) => 401,
            ApiGatewayError::Authorization(_) => 403,
            ApiGatewayError::RateLimit(_) => 429,
            ApiGatewayError::ServiceDiscovery(_) => 503,
            ApiGatewayError::LoadBalancing(_) => 503,
            ApiGatewayError::Caching(_) => 500,
            ApiGatewayError::Monitoring(_) => 500,
            ApiGatewayError::Security(_) => 403,
            ApiGatewayError::Network(_) => 503,
            ApiGatewayError::Timeout(_) => 504,
            ApiGatewayError::CircuitBreaker(_) => 503,
            ApiGatewayError::Validation(_) => 400,
            ApiGatewayError::Serialization(_) => 400,
            ApiGatewayError::Io(_) => 500,
            ApiGatewayError::Http(_) => 502,
            ApiGatewayError::Database(_) => 500,
            ApiGatewayError::ExternalService(_) => 502,
            ApiGatewayError::Internal(_) => 500,
        }
    }

    /// Get error message for client
    pub fn client_message(&self) -> String {
        match self {
            ApiGatewayError::Config(_) => "Configuration error".to_string(),
            ApiGatewayError::Server(_) => "Internal server error".to_string(),
            ApiGatewayError::Routing(_) => "Route not found".to_string(),
            ApiGatewayError::Auth(_) => "Authentication required".to_string(),
            ApiGatewayError::Authorization(_) => "Access denied".to_string(),
            ApiGatewayError::RateLimit(_) => "Rate limit exceeded".to_string(),
            ApiGatewayError::ServiceDiscovery(_) => "Service unavailable".to_string(),
            ApiGatewayError::LoadBalancing(_) => "Service unavailable".to_string(),
            ApiGatewayError::Caching(_) => "Internal server error".to_string(),
            ApiGatewayError::Monitoring(_) => "Internal server error".to_string(),
            ApiGatewayError::Security(_) => "Access denied".to_string(),
            ApiGatewayError::Network(_) => "Service unavailable".to_string(),
            ApiGatewayError::Timeout(_) => "Request timeout".to_string(),
            ApiGatewayError::CircuitBreaker(_) => "Service temporarily unavailable".to_string(),
            ApiGatewayError::Validation(_) => "Invalid request".to_string(),
            ApiGatewayError::Serialization(_) => "Invalid request format".to_string(),
            ApiGatewayError::Io(_) => "Internal server error".to_string(),
            ApiGatewayError::Http(_) => "Service unavailable".to_string(),
            ApiGatewayError::Database(_) => "Internal server error".to_string(),
            ApiGatewayError::ExternalService(_) => "Service unavailable".to_string(),
            ApiGatewayError::Internal(_) => "Internal server error".to_string(),
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            ApiGatewayError::Network(_) => true,
            ApiGatewayError::Timeout(_) => true,
            ApiGatewayError::CircuitBreaker(_) => true,
            ApiGatewayError::ServiceDiscovery(_) => true,
            ApiGatewayError::LoadBalancing(_) => true,
            ApiGatewayError::ExternalService(_) => true,
            ApiGatewayError::Http(_) => true,
            _ => false,
        }
    }

    /// Check if error should be logged
    pub fn should_log(&self) -> bool {
        match self {
            ApiGatewayError::Config(_) => true,
            ApiGatewayError::Server(_) => true,
            ApiGatewayError::Caching(_) => true,
            ApiGatewayError::Monitoring(_) => true,
            ApiGatewayError::Io(_) => true,
            ApiGatewayError::Database(_) => true,
            ApiGatewayError::Internal(_) => true,
            _ => false,
        }
    }
}
