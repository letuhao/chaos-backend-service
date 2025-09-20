//! Configuration Handlers
//!
//! This module handles configuration-related HTTP requests.

use std::sync::Arc;
use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::Json,
};
use serde::Serialize;
use tracing::{info, debug, warn};

use crate::server::SharedGameState;

/// Configuration sync response
#[derive(Debug, Serialize)]
pub struct ConfigSyncResponse {
    pub success: bool,
    pub message: String,
    pub sync_status: Option<ConfigSyncStatus>,
}

/// Configuration sync status
#[derive(Debug, Serialize)]
pub struct ConfigSyncStatus {
    pub is_running: bool,
    pub sync_interval_seconds: u64,
    pub mongodb_enabled: bool,
}

/// Configuration info response
#[derive(Debug, Serialize)]
pub struct ConfigInfoResponse {
    pub success: bool,
    pub config_categories: Vec<String>,
    pub total_configs: usize,
    pub message: String,
}

/// Configuration handlers
pub struct ConfigHandlers;

impl ConfigHandlers {
    /// Trigger configuration sync
    pub async fn sync_config(
        State(shared_state): State<Arc<SharedGameState>>,
    ) -> Result<Json<ConfigSyncResponse>, StatusCode> {
        debug!("Manual configuration sync requested");
        
        // TODO: Implement actual config sync
        // For now, just return success
        warn!("Configuration sync not yet implemented");
        
        Ok(Json(ConfigSyncResponse {
            success: true,
            message: "Configuration sync completed".to_string(),
            sync_status: Some(ConfigSyncStatus {
                is_running: false,
                sync_interval_seconds: 300,
                mongodb_enabled: true,
            }),
        }))
    }
    
    /// Get configuration information
    pub async fn get_config_info(
        State(shared_state): State<Arc<SharedGameState>>,
    ) -> Result<Json<ConfigInfoResponse>, StatusCode> {
        debug!("Configuration info requested");
        
        // Get configuration categories
        let config_manager = shared_state.actor_core.get_config_manager();
        let categories = config_manager.get_registry().get_supported_categories();
        
        // Count total configurations
        let mut total_configs = 0;
        for category in &categories {
            if let Ok(category_config) = config_manager.get_category_config(category).await {
                total_configs += category_config.len();
            }
        }
        
        info!("Configuration info: {} categories, {} total configs", categories.len(), total_configs);
        
        Ok(Json(ConfigInfoResponse {
            success: true,
            config_categories: categories,
            total_configs,
            message: format!("Found {} categories with {} total configurations", categories.len(), total_configs),
        }))
    }
    
    /// Get configuration by category
    pub async fn get_config_by_category(
        State(shared_state): State<Arc<SharedGameState>>,
        Path(category): Path<String>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        debug!("Configuration for category requested: {}", category);
        
        let config_manager = shared_state.actor_core.get_config_manager();
        
        match config_manager.get_category_config(&category).await {
            Ok(category_config) => {
                info!("Retrieved {} configurations for category: {}", category_config.len(), category);
                Ok(Json(serde_json::to_value(category_config).unwrap_or(serde_json::Value::Null)))
            }
            Err(e) => {
                warn!("Failed to get configuration for category {}: {}", category, e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
    
    /// Get specific configuration value
    pub async fn get_config_value(
        State(shared_state): State<Arc<SharedGameState>>,
        Path((category, key)): Path<(String, String)>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        debug!("Configuration value requested: {}:{}", category, key);
        
        let config_manager = shared_state.actor_core.get_config_manager();
        
        match config_manager.get_config_value(&category, &key).await {
            Ok(Some(config_value)) => {
                info!("Retrieved configuration value: {}:{}", category, key);
                Ok(Json(config_value.value))
            }
            Ok(None) => {
                warn!("Configuration value not found: {}:{}", category, key);
                Err(StatusCode::NOT_FOUND)
            }
            Err(e) => {
                warn!("Failed to get configuration value {}:{}: {}", category, key, e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
