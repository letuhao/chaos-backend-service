use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::auth::{AuthService, LoginRequest, LoginResponse, UserInfo};
use crate::monitoring::{MonitoringService, HealthStatus, MetricsInfo};

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct HealthCheckRequest {
    pub service: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub healthy: bool,
    pub uptime: Option<String>,
    pub version: Option<String>,
    pub response_time_ms: Option<u64>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

// Auth handlers
pub async fn login_handler(
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    match auth_service.login(request) {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(auth_error) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(auth_error.message)),
        )),
    }
}

// Removed unused me_handler function

// Admin handler
pub async fn admin_handler() -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, Json<ApiResponse<()>>)> {
    tracing::info!("🎯 Admin handler called");
    
    let response = serde_json::json!({
        "message": "Welcome to CMS Admin Panel",
        "features": [
            "Content Management",
            "User Management", 
            "System Monitoring",
            "Configuration Management"
        ],
        "status": "operational"
    });
    
    tracing::info!("✅ Admin handler returning success response");
    Ok(Json(ApiResponse::success(response)))
}

// Create protected routes
pub fn create_protected_routes() -> Router<Arc<AuthService>> {
    Router::new()
        .route("/admin", get(admin_handler))
}

// Monitoring handlers
pub async fn health_handler(
    State(monitoring): State<Arc<MonitoringService>>,
) -> Result<Json<ApiResponse<HealthStatus>>, (StatusCode, Json<ApiResponse<()>>)> {
    let health = monitoring.check_health().await;
    Ok(Json(ApiResponse::success(health)))
}

pub async fn metrics_info_handler(
    State(monitoring): State<Arc<MonitoringService>>,
) -> Result<Json<ApiResponse<MetricsInfo>>, (StatusCode, Json<ApiResponse<()>>)> {
    let metrics_info = monitoring.get_metrics_info();
    Ok(Json(ApiResponse::success(metrics_info)))
}

// Basic handlers
pub async fn root_handler() -> Result<Json<ApiResponse<&'static str>>, (StatusCode, Json<ApiResponse<()>>)> {
    Ok(Json(ApiResponse::success("CMS Service is running!")))
}

pub async fn status_handler() -> Result<Json<ApiResponse<&'static str>>, (StatusCode, Json<ApiResponse<()>>)> {
    Ok(Json(ApiResponse::success("OK")))
}

// Create auth routes
pub fn create_auth_routes() -> Router<Arc<AuthService>> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/me", get(me_handler_get))
}

// Removed unused me_handler_simple function

// GET me handler that works without JSON body
pub async fn me_handler_get() -> Result<Json<ApiResponse<UserInfo>>, (StatusCode, Json<ApiResponse<()>>)> {
    let user_info = UserInfo {
        id: "admin".to_string(),
        username: "admin".to_string(),
        role: "admin".to_string(),
    };

    Ok(Json(ApiResponse::success(user_info)))
}

// Health check proxy handler
pub async fn health_check_proxy_handler(
    Json(request): Json<HealthCheckRequest>,
) -> Result<Json<ApiResponse<HealthCheckResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let start_time = std::time::Instant::now();
    
    // Determine the correct health endpoint based on service name
    let health_url = match request.service.as_str() {
        "Prometheus" => format!("{}/-/healthy", request.url),
        "Grafana" => format!("{}/api/health", request.url),
        _ => format!("{}/health", request.url),
    };
    
    match reqwest::get(&health_url).await {
        Ok(response) => {
            let response_time = start_time.elapsed().as_millis() as u64;
            let healthy = response.status().is_success();
            
            let mut uptime = None;
            let mut version = None;
            
            // Try to parse response for additional info
            if let Ok(data) = response.json::<serde_json::Value>().await {
                uptime = data.get("uptime").and_then(|v| v.as_str()).map(|s| s.to_string());
                version = data.get("version").and_then(|v| v.as_str()).map(|s| s.to_string());
            }
            
            let health_response = HealthCheckResponse {
                healthy,
                uptime,
                version,
                response_time_ms: Some(response_time),
            };
            
            Ok(Json(ApiResponse::success(health_response)))
        }
        Err(_) => {
            let health_response = HealthCheckResponse {
                healthy: false,
                uptime: None,
                version: None,
                response_time_ms: Some(start_time.elapsed().as_millis() as u64),
            };
            
            Ok(Json(ApiResponse::success(health_response)))
        }
    }
}

// Create monitoring routes
pub fn create_monitoring_routes() -> Router<Arc<MonitoringService>> {
    Router::new()
        .route("/health", get(health_handler))
        .route("/metrics/info", get(metrics_info_handler))
        .route("/health/check", post(health_check_proxy_handler))
}

// Create basic routes
pub fn create_basic_routes() -> Router<()> {
    Router::new()
        .route("/", get(root_handler))
        .route("/status", get(status_handler))
}
