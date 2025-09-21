use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
// JWT validation is handled by AuthService
use serde_json::json;
use std::sync::Arc;

use crate::config::UserServiceConfig;
// TokenClaims import removed as it's not used in this module
use crate::services::AuthService;
use crate::database::DatabaseManager;

/// Authentication middleware to verify JWT tokens
pub async fn auth_middleware(
    State((config, _db_manager)): State<(Arc<UserServiceConfig>, Arc<DatabaseManager>)>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, axum::Json<serde_json::Value>)> {
    // Extract Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(json!({
                    "success": false,
                    "error": "Missing Authorization header"
                })),
            )
        })?;

    // Check if it's a Bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(json!({
                    "success": false,
                    "error": "Invalid Authorization header format"
                })),
            )
        })?;

    // Create auth service
    let auth_service = AuthService::new(config.clone())
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(json!({
                    "success": false,
                    "error": "Internal server error"
                })),
            )
        })?;

    // Validate token
    let claims = auth_service
        .validate_token(token)
        .map_err(|e| {
            tracing::error!("Token validation failed: {}", e);
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(json!({
                    "success": false,
                    "error": "Invalid or expired token"
                })),
            )
        })?;

    tracing::info!("Token validated successfully for user: {}", claims.user_id);
    tracing::info!("Claims: user_id={}, username={}, email={}", claims.user_id, claims.username, claims.email);

    // Add user info to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

// Unused middleware functions removed for cleaner code
