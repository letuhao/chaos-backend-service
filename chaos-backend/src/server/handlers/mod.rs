//! Request Handlers
//!
//! This module contains HTTP request handlers for the game server API.

pub mod actor_handlers;
pub mod config_handlers;
pub mod health_handlers;
pub mod metrics_handlers;

// Re-export main handlers
pub use actor_handlers::ActorHandlers;
pub use config_handlers::ConfigHandlers;
pub use health_handlers::HealthHandlers;
pub use metrics_handlers::MetricsHandlers;
