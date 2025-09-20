//! Health Check Handlers
//!
//! This module handles health check and status endpoints.

use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use tracing::{info, debug};

use crate::server::SharedGameState;

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: u64,
    pub uptime_seconds: u64,
    pub version: String,
}

/// Detailed health response
#[derive(Debug, Serialize)]
pub struct DetailedHealthResponse {
    pub status: String,
    pub timestamp: u64,
    pub uptime_seconds: u64,
    pub version: String,
    pub components: ComponentHealth,
}

/// Component health status
#[derive(Debug, Serialize)]
pub struct ComponentHealth {
    pub actor_core: String,
    pub config_sync: String,
    pub actor_manager: String,
    pub performance_monitor: String,
    pub mongodb: String,
}

/// Health check handlers
pub struct HealthHandlers;

impl HealthHandlers {
    
    /// Basic health check
    pub async fn health_check(
        State(_shared_state): State<Arc<SharedGameState>>,
    ) -> Result<Json<HealthResponse>, StatusCode> {
        debug!("Health check requested");
        
        let uptime = 0; // TODO: Calculate actual uptime
        
        Ok(Json(HealthResponse {
            status: "healthy".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            uptime_seconds: uptime,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }))
    }
    
    /// Detailed health check
    pub async fn detailed_health_check(
        State(shared_state): State<Arc<SharedGameState>>,
    ) -> Result<Json<DetailedHealthResponse>, StatusCode> {
        debug!("Detailed health check requested");
        
        let uptime = 0; // TODO: Calculate actual uptime
        
        // Check component health
        let components = ComponentHealth {
            actor_core: "healthy".to_string(), // TODO: Add actual health checks
            config_sync: "healthy".to_string(),
            actor_manager: "healthy".to_string(),
            performance_monitor: "healthy".to_string(),
            mongodb: "healthy".to_string(),
        };
        
        Ok(Json(DetailedHealthResponse {
            status: "healthy".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            uptime_seconds: uptime,
            version: env!("CARGO_PKG_VERSION").to_string(),
            components,
        }))
    }
    
    /// Readiness check
    pub async fn readiness_check(
        State(shared_state): State<Arc<SharedGameState>>,
    ) -> Result<Json<HealthResponse>, StatusCode> {
        debug!("Readiness check requested");
        
        // Check if all critical components are ready
        let is_ready = Self::check_readiness(&shared_state).await;
        
        let status = if is_ready {
            "ready"
        } else {
            "not_ready"
        };
        
        let uptime = 0; // TODO: Calculate actual uptime
        
        Ok(Json(HealthResponse {
            status: status.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            uptime_seconds: uptime,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }))
    }
    
    /// Liveness check
    pub async fn liveness_check(
        State(_shared_state): State<Arc<SharedGameState>>,
    ) -> Result<Json<HealthResponse>, StatusCode> {
        debug!("Liveness check requested");
        
        let uptime = 0; // TODO: Calculate actual uptime
        
        Ok(Json(HealthResponse {
            status: "alive".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            uptime_seconds: uptime,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }))
    }
    
    /// Check if all components are ready
    async fn check_readiness(shared_state: &SharedGameState) -> bool {
        // Check Actor Core
        if shared_state.actor_core.get_config_manager().is_none() {
            return false;
        }
        
        // Check Actor Manager
        if shared_state.actor_manager.get_active_count() == 0 {
            // This is actually OK, just means no actors yet
        }
        
        // Check Performance Monitor
        let metrics = shared_state.get_metrics();
        if metrics.fps < 1.0 {
            // Performance monitor might not be running yet
            return false;
        }
        
        true
    }
}
