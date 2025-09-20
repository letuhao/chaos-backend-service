//! Metrics Handlers
//!
//! This module handles metrics and performance-related HTTP requests.

use std::sync::Arc;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use tracing::{info, debug};

use crate::server::SharedGameState;

/// Performance metrics response
#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub success: bool,
    pub metrics: PerformanceMetricsData,
    pub message: String,
}

/// Performance metrics data
#[derive(Debug, Serialize)]
pub struct PerformanceMetricsData {
    pub fps: f64,
    pub avg_frame_time_ms: f64,
    pub memory_usage_mb: f64,
    pub active_actors: u32,
    pub packets_per_second: u32,
    pub db_queries_per_second: u32,
    pub last_update: u64,
}

/// Actor statistics response
#[derive(Debug, Serialize)]
pub struct ActorStatsResponse {
    pub success: bool,
    pub stats: ActorStatsData,
    pub message: String,
}

/// Actor statistics data
#[derive(Debug, Serialize)]
pub struct ActorStatsData {
    pub total_created: u64,
    pub total_destroyed: u64,
    pub active_count: u32,
    pub creation_rate: f64,
    pub last_update: u64,
}

/// Server status response
#[derive(Debug, Serialize)]
pub struct ServerStatusResponse {
    pub success: bool,
    pub status: ServerStatusData,
    pub message: String,
}

/// Server status data
#[derive(Debug, Serialize)]
pub struct ServerStatusData {
    pub uptime_seconds: u64,
    pub active_actors: u32,
    pub performance_acceptable: bool,
    pub config_sync_enabled: bool,
    pub mongodb_enabled: bool,
}

/// Metrics handlers
pub struct MetricsHandlers;

impl MetricsHandlers {
    
    /// Get performance metrics
    pub async fn get_metrics(
        State(shared_state): State<Arc<SharedGameState>>,
    ) -> Result<Json<MetricsResponse>, StatusCode> {
        debug!("Performance metrics requested");
        
        let metrics = shared_state.get_metrics();
        
        let metrics_data = PerformanceMetricsData {
            fps: metrics.fps,
            avg_frame_time_ms: metrics.avg_frame_time_ms,
            memory_usage_mb: metrics.memory_usage_mb,
            active_actors: metrics.active_actors,
            packets_per_second: metrics.packets_per_second,
            db_queries_per_second: metrics.db_queries_per_second,
            last_update: metrics.last_update,
        };
        
        info!("Retrieved performance metrics: FPS={:.1}, Actors={}", metrics.fps, metrics.active_actors);
        
        Ok(Json(MetricsResponse {
            success: true,
            metrics: metrics_data,
            message: "Performance metrics retrieved successfully".to_string(),
        }))
    }
    
    /// Get actor statistics
    pub async fn get_actor_stats(
        State(shared_state): State<Arc<SharedGameState>>,
    ) -> Result<Json<ActorStatsResponse>, StatusCode> {
        debug!("Actor statistics requested");
        
        let stats = shared_state.actor_manager.get_stats();
        
        let stats_data = ActorStatsData {
            total_created: stats.total_created,
            total_destroyed: stats.total_destroyed,
            active_count: stats.active_count,
            creation_rate: stats.creation_rate,
            last_update: stats.last_update,
        };
        
        info!("Retrieved actor statistics: Created={}, Active={}", stats.total_created, stats.active_count);
        
        Ok(Json(ActorStatsResponse {
            success: true,
            stats: stats_data,
            message: "Actor statistics retrieved successfully".to_string(),
        }))
    }
    
    /// Get server status
    pub async fn get_server_status(
        State(shared_state): State<Arc<SharedGameState>>,
    ) -> Result<Json<ServerStatusResponse>, StatusCode> {
        debug!("Server status requested");
        
        let uptime = 0; // TODO: Calculate actual uptime
        let active_actors = shared_state.actor_manager.get_active_count();
        let metrics = shared_state.get_metrics();
        
        // Check if performance is acceptable
        let performance_acceptable = metrics.fps >= 30.0 && metrics.avg_frame_time_ms <= 33.0;
        
        // Check configuration sync status
        let config_sync_enabled = true; // TODO: Get actual status
        let mongodb_enabled = true; // TODO: Get actual status
        
        let status_data = ServerStatusData {
            uptime_seconds: uptime,
            active_actors,
            performance_acceptable,
            config_sync_enabled,
            mongodb_enabled,
        };
        
        info!("Retrieved server status: Uptime={}s, Actors={}, Performance={}", 
              uptime, active_actors, if performance_acceptable { "OK" } else { "POOR" });
        
        Ok(Json(ServerStatusResponse {
            success: true,
            status: status_data,
            message: "Server status retrieved successfully".to_string(),
        }))
    }
    
    /// Get detailed performance report
    pub async fn get_performance_report(
        State(shared_state): State<Arc<SharedGameState>>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        debug!("Detailed performance report requested");
        
        let metrics = shared_state.get_metrics();
        let actor_stats = shared_state.actor_manager.get_stats();
        let uptime = 0; // TODO: Calculate actual uptime
        
        let report = serde_json::json!({
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            "uptime_seconds": uptime,
            "performance": {
                "fps": metrics.fps,
                "avg_frame_time_ms": metrics.avg_frame_time_ms,
                "memory_usage_mb": metrics.memory_usage_mb,
                "packets_per_second": metrics.packets_per_second,
                "db_queries_per_second": metrics.db_queries_per_second,
            },
            "actors": {
                "total_created": actor_stats.total_created,
                "total_destroyed": actor_stats.total_destroyed,
                "active_count": actor_stats.active_count,
                "creation_rate": actor_stats.creation_rate,
            },
            "system": {
                "config_sync_enabled": true,
                "mongodb_enabled": true,
                "performance_acceptable": metrics.fps >= 30.0 && metrics.avg_frame_time_ms <= 33.0,
            }
        });
        
        info!("Generated detailed performance report");
        
        Ok(Json(report))
    }
}
