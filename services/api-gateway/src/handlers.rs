//! Request handlers for API Gateway

use crate::errors::{ApiGatewayError, Result};
use crate::types::{ApiResponse, RequestContext};
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use serde_json::json;
use std::sync::Arc;
use tracing::{info, debug, warn, error};

/// Health check handler
pub async fn health_check() -> &'static str {
    "OK"
}

/// Status handler
pub async fn status() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": env!("CARGO_PKG_VERSION"),
        "service": "api-gateway"
    }))
}

/// Metrics handler
pub async fn metrics() -> String {
    "# HELP api_gateway_requests_total Total number of requests\n\
     # TYPE api_gateway_requests_total counter\n\
     api_gateway_requests_total 0\n\
     # HELP api_gateway_request_duration_seconds Average request duration in seconds\n\
     # TYPE api_gateway_request_duration_seconds histogram\n\
     api_gateway_request_duration_seconds 0.0\n\
     # HELP api_gateway_errors_total Total number of errors\n\
     # TYPE api_gateway_errors_total counter\n\
     api_gateway_errors_total 0\n\
     # HELP api_gateway_active_connections Current number of active connections\n\
     # TYPE api_gateway_active_connections gauge\n\
     api_gateway_active_connections 0\n".to_string()
}

/// Not found handler
pub async fn not_found() -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        "Not Found".to_string(),
        "not_found".to_string(),
    );
    (StatusCode::NOT_FOUND, Json(response))
}

/// Method not allowed handler
pub async fn method_not_allowed() -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        "Method Not Allowed".to_string(),
        "method_not_allowed".to_string(),
    );
    (StatusCode::METHOD_NOT_ALLOWED, Json(response))
}

/// Internal server error handler
pub async fn internal_server_error() -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        "Internal Server Error".to_string(),
        "internal_server_error".to_string(),
    );
    (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
}

/// Bad request handler
pub async fn bad_request(message: String) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(message, "bad_request".to_string());
    (StatusCode::BAD_REQUEST, Json(response))
}

/// Unauthorized handler
pub async fn unauthorized(message: String) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(message, "unauthorized".to_string());
    (StatusCode::UNAUTHORIZED, Json(response))
}

/// Forbidden handler
pub async fn forbidden(message: String) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(message, "forbidden".to_string());
    (StatusCode::FORBIDDEN, Json(response))
}

/// Too many requests handler
pub async fn too_many_requests(message: String) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(message, "too_many_requests".to_string());
    (StatusCode::TOO_MANY_REQUESTS, Json(response))
}

/// Service unavailable handler
pub async fn service_unavailable(message: String) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(message, "service_unavailable".to_string());
    (StatusCode::SERVICE_UNAVAILABLE, Json(response))
}

/// Gateway timeout handler
pub async fn gateway_timeout(message: String) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(message, "gateway_timeout".to_string());
    (StatusCode::GATEWAY_TIMEOUT, Json(response))
}

/// Create success response
pub fn success_response<T>(data: T, request_id: String) -> (StatusCode, Json<ApiResponse<T>>) {
    let response = ApiResponse::success(data, request_id);
    (StatusCode::OK, Json(response))
}

/// Create error response
pub fn error_response(message: String, request_id: String, status_code: StatusCode) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(message, request_id);
    (status_code, Json(response))
}

/// Extract request ID from headers
pub fn extract_request_id(headers: &HeaderMap) -> String {
    headers
        .get("X-Request-ID")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_else(|| "unknown")
        .to_string()
}

/// Extract user ID from headers
pub fn extract_user_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get("X-User-ID")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}

/// Extract session ID from headers
pub fn extract_session_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get("X-Session-ID")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}

/// Extract IP address from headers
pub fn extract_ip_address(headers: &HeaderMap) -> String {
    // Try to get IP from X-Forwarded-For header
    if let Some(forwarded_for) = headers.get("X-Forwarded-For") {
        if let Ok(forwarded_for) = forwarded_for.to_str() {
            if let Some(ip) = forwarded_for.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }
    
    // Try to get IP from X-Real-IP header
    if let Some(real_ip) = headers.get("X-Real-IP") {
        if let Ok(real_ip) = real_ip.to_str() {
            return real_ip.to_string();
        }
    }
    
    // Fallback to localhost
    "127.0.0.1".to_string()
}

/// Create CORS response
pub fn cors_response() -> (StatusCode, Json<serde_json::Value>) {
    let response = json!({
        "message": "CORS preflight request handled",
        "timestamp": chrono::Utc::now()
    });
    (StatusCode::OK, Json(response))
}

/// Create maintenance response
pub fn maintenance_response() -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        "Service temporarily unavailable for maintenance".to_string(),
        "maintenance".to_string(),
    );
    (StatusCode::SERVICE_UNAVAILABLE, Json(response))
}

/// Create rate limit response
pub fn rate_limit_response(limit: u32, remaining: u32, reset_time: u64) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Rate limit exceeded. Limit: {}, Remaining: {}, Reset time: {}", limit, remaining, reset_time),
        "rate_limit_exceeded".to_string(),
    );
    (StatusCode::TOO_MANY_REQUESTS, Json(response))
}

/// Create circuit breaker response
pub fn circuit_breaker_response(service_name: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Service {} is temporarily unavailable due to circuit breaker", service_name),
        "circuit_breaker_open".to_string(),
    );
    (StatusCode::SERVICE_UNAVAILABLE, Json(response))
}

/// Create timeout response
pub fn timeout_response(service_name: &str, timeout: u64) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Service {} request timed out after {} seconds", service_name, timeout),
        "request_timeout".to_string(),
    );
    (StatusCode::GATEWAY_TIMEOUT, Json(response))
}

/// Create validation error response
pub fn validation_error_response(errors: Vec<String>) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Validation failed: {}", errors.join(", ")),
        "validation_error".to_string(),
    );
    (StatusCode::BAD_REQUEST, Json(response))
}

/// Create authentication error response
pub fn authentication_error_response(message: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        message.to_string(),
        "authentication_error".to_string(),
    );
    (StatusCode::UNAUTHORIZED, Json(response))
}

/// Create authorization error response
pub fn authorization_error_response(message: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        message.to_string(),
        "authorization_error".to_string(),
    );
    (StatusCode::FORBIDDEN, Json(response))
}

/// Create service error response
pub fn service_error_response(service_name: &str, error: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Service {} error: {}", service_name, error),
        "service_error".to_string(),
    );
    (StatusCode::BAD_GATEWAY, Json(response))
}

/// Create network error response
pub fn network_error_response(service_name: &str, error: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Network error connecting to {}: {}", service_name, error),
        "network_error".to_string(),
    );
    (StatusCode::BAD_GATEWAY, Json(response))
}

/// Create database error response
pub fn database_error_response(error: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Database error: {}", error),
        "database_error".to_string(),
    );
    (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
}

/// Create cache error response
pub fn cache_error_response(error: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Cache error: {}", error),
        "cache_error".to_string(),
    );
    (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
}

/// Create monitoring error response
pub fn monitoring_error_response(error: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Monitoring error: {}", error),
        "monitoring_error".to_string(),
    );
    (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
}

/// Create security error response
pub fn security_error_response(error: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Security error: {}", error),
        "security_error".to_string(),
    );
    (StatusCode::FORBIDDEN, Json(response))
}

/// Create load balancing error response
pub fn load_balancing_error_response(error: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Load balancing error: {}", error),
        "load_balancing_error".to_string(),
    );
    (StatusCode::SERVICE_UNAVAILABLE, Json(response))
}

/// Create service discovery error response
pub fn service_discovery_error_response(error: &str) -> (StatusCode, Json<ApiResponse<()>>) {
    let response = ApiResponse::error(
        format!("Service discovery error: {}", error),
        "service_discovery_error".to_string(),
    );
    (StatusCode::SERVICE_UNAVAILABLE, Json(response))
}
