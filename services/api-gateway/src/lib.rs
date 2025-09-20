//! API Gateway Service
//! 
//! A high-performance, production-ready API Gateway for the Chaos World game backend.
//! Provides routing, load balancing, authentication, rate limiting, and monitoring.

/// API Gateway version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// API Gateway service name
pub const SERVICE_NAME: &str = "api-gateway";

/// Default configuration file path
pub const DEFAULT_CONFIG_PATH: &str = "configs/api-gateway.yaml";

/// Development configuration file path
pub const DEV_CONFIG_PATH: &str = "configs/api-gateway-dev.yaml";

/// Production configuration file path
pub const PROD_CONFIG_PATH: &str = "configs/api-gateway-prod.yaml";