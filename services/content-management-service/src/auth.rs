use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub username: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthError {
    pub error: String,
    pub message: String,
}

pub struct AuthService {
    jwt_secret: String,
    jwt_expiry: i64,
    admin_username: String,
    admin_password_hash: String,
}

impl AuthService {
    pub fn new(jwt_secret: String, jwt_expiry: i64, admin_username: String, admin_password: String) -> Self {
        // Hash the admin password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let admin_password_hash = argon2
            .hash_password(admin_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        Self {
            jwt_secret,
            jwt_expiry,
            admin_username,
            admin_password_hash,
        }
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| AuthError {
                error: "INVALID_HASH".to_string(),
                message: "Invalid password hash".to_string(),
            })?;

        let argon2 = Argon2::default();
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn login(&self, request: LoginRequest) -> Result<LoginResponse, AuthError> {
        // Verify credentials
        if request.username != self.admin_username {
            return Err(AuthError {
                error: "INVALID_CREDENTIALS".to_string(),
                message: "Invalid username or password".to_string(),
            });
        }

        if !self.verify_password(&request.password, &self.admin_password_hash)? {
            return Err(AuthError {
                error: "INVALID_CREDENTIALS".to_string(),
                message: "Invalid username or password".to_string(),
            });
        }

        // Generate JWT token
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            user_id: "admin".to_string(),
            username: self.admin_username.clone(),
            role: "admin".to_string(),
            exp: now + self.jwt_expiry as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AuthError {
            error: "TOKEN_GENERATION_FAILED".to_string(),
            message: "Failed to generate token".to_string(),
        })?;

        Ok(LoginResponse {
            token,
            expires_in: self.jwt_expiry,
            user: UserInfo {
                id: "admin".to_string(),
                username: self.admin_username.clone(),
                role: "admin".to_string(),
            },
        })
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AuthError {
            error: "INVALID_TOKEN".to_string(),
            message: "Invalid or expired token".to_string(),
        })?;

        Ok(token_data.claims)
    }
}

pub async fn auth_middleware(
    State(auth_service): State<Arc<AuthService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<AuthError>)> {
    tracing::debug!("🔐 Auth middleware called for: {}", request.uri());
    
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok());

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];
            tracing::debug!("🔑 Token received: {}...", &token[..20.min(token.len())]);

            match auth_service.validate_token(token) {
                Ok(claims) => {
                    // Add user info to request extensions
                    request.extensions_mut().insert(claims);
                    Ok(next.run(request).await)
                }
                Err(auth_error) => Err((
                    StatusCode::UNAUTHORIZED,
                    Json(auth_error),
                )),
            }
        }
        _ => Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                error: "MISSING_TOKEN".to_string(),
                message: "Authorization header with Bearer token required".to_string(),
            }),
        )),
    }
}
