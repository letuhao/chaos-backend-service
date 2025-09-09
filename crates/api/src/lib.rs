//! API - REST, gRPC, and WebSocket API endpoints.
//!
//! This crate provides the API layer for the Chaos World MMORPG backend,
//! including REST endpoints, gRPC services, and WebSocket connections.

pub mod rest;
pub mod grpc;
pub mod websocket;
pub mod auth;
pub mod middleware;
pub mod error;
pub mod types;

// Re-export commonly used types
pub use error::*;
pub use types::*;
