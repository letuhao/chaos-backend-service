use axum::{
    extract::{Json, State, Request, ConnectInfo},
    http::{StatusCode, HeaderMap},
    response::Json as ResponseJson,
};
use std::net::SocketAddr;
use serde_json::{json, Value};
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use validator::Validate;

use crate::config::UserServiceConfig;
use crate::models::{
    RegisterRequest, LoginRequest, RefreshTokenRequest, 
    AuthResponse, ErrorResponse, SuccessResponse, UserProfileResponse,
    User, PublicUser, UserStatus, TokenClaims
};
use crate::services::AuthService;
use crate::database::DatabaseManager;
use crate::metrics::METRICS;
use crate::utils::request::ClientInfo;

/// User registration handler
pub async fn register(
    State((config, db_manager)): State<(Arc<UserServiceConfig>, Arc<DatabaseManager>)>,
    headers: HeaderMap,
    connect_info: Option<ConnectInfo<SocketAddr>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<ResponseJson<Value>, (StatusCode, ResponseJson<Value>)> {
    // Record HTTP request
    METRICS.record_http_request("POST", "/auth/register", 200);
    
    // Extract client information
    let client_info = ClientInfo::from_request(&headers, connect_info);
    tracing::info!("Registration request from IP: {:?}, User-Agent: {:?}", 
                   client_info.ip_address, client_info.user_agent);
    
    // Validate request
    if let Err(validation_errors) = payload.validate() {
        let error_messages: Vec<String> = validation_errors
            .field_errors()
            .values()
            .flat_map(|errors| errors.iter().map(|e| e.message.clone().unwrap_or_else(|| "Invalid field".into()).to_string()))
            .collect();
        let error_response = ErrorResponse::with_details(
            "Validation failed",
            &error_messages.join(", ")
        );
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(json!(error_response))
        ));
    }

    // Check if user agrees to terms
    if !payload.agree_to_terms {
        let error_response = ErrorResponse::new("You must agree to the terms and conditions");
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(json!(error_response))
        ));
    }

    // Create auth service
    let auth_service = match AuthService::new(config.clone()) {
        Ok(service) => service,
        Err(e) => {
            tracing::error!("Failed to create auth service: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // Validate password strength
    if let Err(password_errors) = auth_service.validate_password_strength(&payload.password) {
        let error_response = ErrorResponse::with_details(
            "Password does not meet requirements",
            &password_errors.join(", ")
        );
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(json!(error_response))
        ));
    }

    // Check if username already exists
    if db_manager.user_repo.username_exists(&payload.username).await.unwrap_or(false) {
        let error_response = ErrorResponse::new("Username already exists");
        return Err((
            StatusCode::CONFLICT,
            ResponseJson(json!(error_response))
        ));
    }

    // Check if email already exists
    if db_manager.user_repo.email_exists(&payload.email).await.unwrap_or(false) {
        let error_response = ErrorResponse::new("Email already exists");
        return Err((
            StatusCode::CONFLICT,
            ResponseJson(json!(error_response))
        ));
    }

    // Hash password
    let password_hash = match auth_service.hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Failed to hash password: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // Create user
    let user = User {
        id: Uuid::new_v4(),
        username: payload.username.clone(),
        email: payload.email.clone(),
        password_hash,
        display_name: payload.display_name.clone(),
        avatar_url: None,
        status: UserStatus::Active,
        email_verified: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        last_login: None,
        login_count: 0,
    };

    // Create session
    let session = match auth_service.create_session(user.id, client_info.ip_address, client_info.user_agent) {
        Ok(session) => session,
        Err(e) => {
            tracing::error!("Failed to create session: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // Generate tokens
    let tokens = match auth_service.generate_tokens(&user, session.id) {
        Ok(tokens) => tokens,
        Err(e) => {
            tracing::error!("Failed to generate tokens: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // Save user to database
    tracing::info!("Saving user to database: {}", user.username);
    let saved_user = match db_manager.user_repo.create_user(&user).await {
        Ok(user) => {
            tracing::info!("User saved successfully: {}", user.username);
            user
        },
        Err(e) => {
            tracing::error!("Failed to save user to database: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // Save session to database
    tracing::info!("Saving session to database: {:?}", session);
    match db_manager.session_repo.create_session(&session).await {
        Ok(saved_session) => {
            tracing::info!("Session saved successfully to database: {:?}", saved_session);
        }
        Err(e) => {
            tracing::error!("Failed to save session to database: {}", e);
            tracing::error!("Session data that failed to save: {:?}", session);
            // Continue anyway, user is created
        }
    }

    // Record successful registration
    METRICS.record_registration();
    METRICS.record_auth_attempt("register", "success");

    let response = AuthResponse {
        success: true,
        user: PublicUser::from(saved_user),
        tokens,
    };

    Ok(ResponseJson(json!(response)))
}

/// User login handler
pub async fn login(
    State((config, db_manager)): State<(Arc<UserServiceConfig>, Arc<DatabaseManager>)>,
    headers: HeaderMap,
    connect_info: Option<ConnectInfo<SocketAddr>>,
    Json(payload): Json<LoginRequest>,
) -> Result<ResponseJson<Value>, (StatusCode, ResponseJson<Value>)> {
    // Record HTTP request
    METRICS.record_http_request("POST", "/auth/login", 200);
    
    // Extract client information
    let client_info = ClientInfo::from_request(&headers, connect_info);
    tracing::info!("Login request from IP: {:?}, User-Agent: {:?}", 
                   client_info.ip_address, client_info.user_agent);
    
    // Validate request
    if let Err(validation_errors) = payload.validate() {
        let error_messages: Vec<String> = validation_errors
            .field_errors()
            .values()
            .flat_map(|errors| errors.iter().map(|e| e.message.clone().unwrap_or_else(|| "Invalid field".into()).to_string()))
            .collect();
        let error_response = ErrorResponse::with_details(
            "Validation failed",
            &error_messages.join(", ")
        );
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(json!(error_response))
        ));
    }

    // Create auth service
    let auth_service = match AuthService::new(config.clone()) {
        Ok(service) => service,
        Err(e) => {
            tracing::error!("Failed to create auth service: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // Find user by username or email in database
    tracing::info!("Looking for user: {}", payload.username_or_email);
    let user = match db_manager.user_repo.find_by_username_or_email(&payload.username_or_email).await {
        Ok(Some(user)) => {
            tracing::info!("Found user: {}", user.username);
            user
        },
        Ok(None) => {
            tracing::warn!("User not found: {}", payload.username_or_email);
            let error_response = ErrorResponse::new("Invalid username or password");
            return Err((
                StatusCode::UNAUTHORIZED,
                ResponseJson(json!(error_response))
            ));
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // Verify password
    if !auth_service.verify_password(&payload.password, &user.password_hash).unwrap_or(false) {
        let error_response = ErrorResponse::new("Invalid username or password");
        return Err((
            StatusCode::UNAUTHORIZED,
            ResponseJson(json!(error_response))
        ));
    }

    // Check if user is active
    if !auth_service.is_user_active(&user) {
        let error_response = ErrorResponse::new("Account is not active");
        return Err((
            StatusCode::UNAUTHORIZED,
            ResponseJson(json!(error_response))
        ));
    }

    // Create session
    let session = match auth_service.create_session(user.id, client_info.ip_address, client_info.user_agent) {
        Ok(session) => session,
        Err(e) => {
            tracing::error!("Failed to create session: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // Generate tokens
    let tokens = match auth_service.generate_tokens(&user, session.id) {
        Ok(tokens) => tokens,
        Err(e) => {
            tracing::error!("Failed to generate tokens: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // Update user login info
    let mut updated_user = user.clone();
    updated_user.last_login = Some(Utc::now());
    updated_user.login_count += 1;
    updated_user.updated_at = Utc::now();

    if let Err(e) = db_manager.user_repo.update_user(&updated_user).await {
        tracing::error!("Failed to update user login info: {}", e);
        // Continue anyway
    }

    // Save session to database
    tracing::info!("Saving session to database: {:?}", session);
    if let Err(e) = db_manager.session_repo.create_session(&session).await {
        tracing::error!("Failed to save session to database: {}", e);
        // Continue anyway
    } else {
        tracing::info!("Session saved successfully to database");
    }

    // Record successful login
    METRICS.record_login();
    METRICS.record_auth_attempt("login", "success");

    let response = AuthResponse {
        success: true,
        user: PublicUser::from(updated_user),
        tokens,
    };

    Ok(ResponseJson(json!(response)))
}

/// Get current user profile handler
pub async fn me(
    State((_config, db_manager)): State<(Arc<UserServiceConfig>, Arc<DatabaseManager>)>,
    request: Request,
    // User claims will be extracted by the auth middleware
) -> Result<ResponseJson<Value>, (StatusCode, ResponseJson<Value>)> {
    // Extract user claims from request extensions (set by auth middleware)
    let claims = request.extensions().get::<TokenClaims>()
        .ok_or_else(|| {
            tracing::error!("No token claims found in request extensions");
            let error_response = ErrorResponse::new("User not authenticated");
            (StatusCode::UNAUTHORIZED, ResponseJson(json!(error_response)))
        })?;

    tracing::info!("Token claims found - Looking for user with ID: {}", claims.user_id);

    // Find user in database by ID from token claims
    let user = match db_manager.user_repo.find_by_id(claims.user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            let error_response = ErrorResponse::new("User not found");
            return Err((StatusCode::NOT_FOUND, ResponseJson(json!(error_response))));
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(json!(error_response))));
        }
    };

    let response = UserProfileResponse {
        success: true,
        user: PublicUser::from(user),
    };

    Ok(ResponseJson(json!(response)))
}

/// Refresh token handler
pub async fn refresh_token(
    State((config, _db_manager)): State<(Arc<UserServiceConfig>, Arc<DatabaseManager>)>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<ResponseJson<Value>, (StatusCode, ResponseJson<Value>)> {
    // Validate request
    if let Err(validation_errors) = payload.validate() {
        let error_messages: Vec<String> = validation_errors
            .field_errors()
            .values()
            .flat_map(|errors| errors.iter().map(|e| e.message.clone().unwrap_or_else(|| "Invalid field".into()).to_string()))
            .collect();
        let error_response = ErrorResponse::with_details(
            "Validation failed",
            &error_messages.join(", ")
        );
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(json!(error_response))
        ));
    }

    // Create auth service
    let auth_service = match AuthService::new(config.clone()) {
        Ok(service) => service,
        Err(e) => {
            tracing::error!("Failed to create auth service: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // Validate refresh token
    let claims = match auth_service.validate_refresh_token(&payload.refresh_token) {
        Ok(claims) => claims,
        Err(_) => {
            let error_response = ErrorResponse::new("Invalid refresh token");
            return Err((
                StatusCode::UNAUTHORIZED,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // TODO: Find user by ID in database
    // TODO: Check if session is still active
    // For now, we'll create a mock user

    let user = User {
        id: claims.user_id,
        username: claims.username.clone(),
        email: claims.email.clone(),
        password_hash: "hashed_password".to_string(),
        display_name: Some("Test User".to_string()),
        avatar_url: None,
        status: UserStatus::Active,
        email_verified: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        last_login: Some(Utc::now()),
        login_count: 1,
    };

    // Generate new tokens
    let tokens = match auth_service.generate_tokens(&user, claims.session_id) {
        Ok(tokens) => tokens,
        Err(e) => {
            tracing::error!("Failed to generate tokens: {}", e);
            let error_response = ErrorResponse::new("Internal server error");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(json!(error_response))
            ));
        }
    };

    // TODO: Update session in database

    let response = json!({
        "success": true,
        "tokens": tokens
    });

    Ok(ResponseJson(response))
}

/// User logout handler
pub async fn logout(
    State((_config, _db_manager)): State<(Arc<UserServiceConfig>, Arc<DatabaseManager>)>,
    // TODO: Add authentication middleware to extract user from token
) -> Result<ResponseJson<Value>, (StatusCode, ResponseJson<Value>)> {
    // TODO: Extract user from JWT token
    // TODO: Invalidate session in database

    let response = SuccessResponse::new("Logged out successfully");
    Ok(ResponseJson(json!(response)))
}

/// User logout from all devices handler
pub async fn logout_all(
    State((_config, _db_manager)): State<(Arc<UserServiceConfig>, Arc<DatabaseManager>)>,
    // TODO: Add authentication middleware to extract user from token
) -> Result<ResponseJson<Value>, (StatusCode, ResponseJson<Value>)> {
    // TODO: Extract user from JWT token
    // TODO: Invalidate all sessions for user in database

    let response = SuccessResponse::new("Logged out from all devices");
    Ok(ResponseJson(json!(response)))
}

// Debug functions removed for production security

/// Health check handler
#[allow(dead_code)]
pub async fn health_check() -> &'static str {
    "OK"
}

/// Root handler
#[allow(dead_code)]
pub async fn root() -> &'static str {
    "User Management Service"
}
