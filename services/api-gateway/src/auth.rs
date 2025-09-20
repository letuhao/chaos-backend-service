//! Authentication and authorization for API Gateway

use crate::config::{Config, JwtConfig, OAuth2Config, ApiKeysConfig};
use crate::errors::{ApiGatewayError, Result};
use crate::types::{User, RequestContext};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, debug, error, warn};

/// Authentication service
#[derive(Clone)]
pub struct AuthService {
    config: Config,
    jwt_config: JwtConfig,
    oauth2_config: OAuth2Config,
    api_keys_config: ApiKeysConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub iss: String,        // Issuer
    pub aud: String,        // Audience
    pub exp: u64,           // Expiration time
    pub iat: u64,           // Issued at
    pub roles: Vec<String>, // User roles
    pub permissions: Vec<String>, // User permissions
}

/// API key information
#[derive(Debug, Clone)]
pub struct ApiKeyInfo {
    pub name: String,
    pub permissions: Vec<String>,
    pub expires_at: Option<SystemTime>,
}

/// Initialize auth service
pub async fn init(config: &Config) -> Result<AuthService> {
    AuthService::new(config.clone())
}

impl std::fmt::Debug for AuthService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthService")
            .field("config", &self.config)
            .field("jwt_config", &self.jwt_config)
            .field("oauth2_config", &self.oauth2_config)
            .field("api_keys_config", &self.api_keys_config)
            .field("validation", &self.validation)
            .finish()
    }
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(config: Config) -> Result<Self> {
        let jwt_config = config.auth.jwt.clone();
        let oauth2_config = config.auth.oauth2.clone();
        let api_keys_config = config.auth.api_keys.clone();

        // Create JWT keys
        let encoding_key = EncodingKey::from_secret(jwt_config.secret.as_ref());
        let decoding_key = DecodingKey::from_secret(jwt_config.secret.as_ref());

        // Create JWT validation
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&jwt_config.issuer]);
        validation.set_audience(&[&jwt_config.audience]);

        Ok(Self {
            config,
            jwt_config,
            oauth2_config,
            api_keys_config,
            encoding_key,
            decoding_key,
            validation,
        })
    }

    /// Authenticate a request using JWT token
    pub async fn authenticate_jwt(&self, token: &str) -> Result<User> {
        debug!("Authenticating JWT token");

        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map_err(|e| ApiGatewayError::Auth(format!("Invalid JWT token: {}", e)))?;

        let claims = token_data.claims;

        // Check if token is expired
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if claims.exp < now {
            return Err(ApiGatewayError::Auth("Token expired".to_string()));
        }

        // Create user from claims
        let user = User {
            id: claims.sub,
            username: "".to_string(), // TODO: Get from database
            email: "".to_string(),    // TODO: Get from database
            roles: claims.roles,
            permissions: claims.permissions,
            created_at: SystemTime::now(), // TODO: Get from database
            last_login: Some(SystemTime::now()),
            is_active: true,
        };

        debug!("JWT authentication successful for user: {}", user.id);
        Ok(user)
    }

    /// Authenticate a request using API key
    pub async fn authenticate_api_key(&self, api_key: &str) -> Result<ApiKeyInfo> {
        debug!("Authenticating API key");

        if !self.api_keys_config.enabled {
            return Err(ApiGatewayError::Auth("API key authentication disabled".to_string()));
        }

        for key in &self.api_keys_config.keys {
            if key.key == api_key {
                // Check if key is expired
                if let Ok(expires_at) = chrono::DateTime::parse_from_rfc3339(&key.expires_at) {
                    if expires_at.timestamp() < SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64 {
                        return Err(ApiGatewayError::Auth("API key expired".to_string()));
                    }
                }

                let api_key_info = ApiKeyInfo {
                    name: key.name.clone(),
                    permissions: key.permissions.clone(),
                    expires_at: None, // TODO: Parse expires_at
                };

                debug!("API key authentication successful: {}", api_key_info.name);
                return Ok(api_key_info);
            }
        }

        Err(ApiGatewayError::Auth("Invalid API key".to_string()))
    }

    /// Authenticate a request using OAuth2
    pub async fn authenticate_oauth2(&self, provider: &str, code: &str) -> Result<User> {
        debug!("Authenticating OAuth2 with provider: {}", provider);

        if !self.oauth2_config.enabled {
            return Err(ApiGatewayError::Auth("OAuth2 authentication disabled".to_string()));
        }

        let oauth_provider = self.oauth2_config.providers.get(provider)
            .ok_or_else(|| ApiGatewayError::Auth(format!("Unknown OAuth2 provider: {}", provider)))?;

        // TODO: Implement OAuth2 flow
        // This is a placeholder implementation
        let user = User {
            id: "oauth_user_123".to_string(),
            username: "oauth_user".to_string(),
            email: "oauth@example.com".to_string(),
            roles: vec!["user".to_string()],
            permissions: vec!["read".to_string(), "write".to_string()],
            created_at: SystemTime::now(),
            last_login: Some(SystemTime::now()),
            is_active: true,
        };

        debug!("OAuth2 authentication successful for user: {}", user.id);
        Ok(user)
    }

    /// Generate a JWT token for a user
    pub async fn generate_token(&self, user: &User) -> Result<String> {
        debug!("Generating JWT token for user: {}", user.id);

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let exp = now + self.jwt_config.expiration;

        let claims = Claims {
            sub: user.id.clone(),
            iss: self.jwt_config.issuer.clone(),
            aud: self.jwt_config.audience.clone(),
            exp,
            iat: now,
            roles: user.roles.clone(),
            permissions: user.permissions.clone(),
        };

        let token = encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(|e| ApiGatewayError::Auth(format!("Failed to generate JWT token: {}", e)))?;

        debug!("JWT token generated successfully");
        Ok(token)
    }

    /// Generate a refresh token
    pub async fn generate_refresh_token(&self, user: &User) -> Result<String> {
        debug!("Generating refresh token for user: {}", user.id);

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let exp = now + self.jwt_config.refresh_expiration;

        let claims = Claims {
            sub: user.id.clone(),
            iss: self.jwt_config.issuer.clone(),
            aud: self.jwt_config.audience.clone(),
            exp,
            iat: now,
            roles: user.roles.clone(),
            permissions: user.permissions.clone(),
        };

        let token = encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(|e| ApiGatewayError::Auth(format!("Failed to generate refresh token: {}", e)))?;

        debug!("Refresh token generated successfully");
        Ok(token)
    }

    /// Authorize a user for a specific action
    pub async fn authorize(&self, user: &User, required_roles: &[String], required_permissions: &[String]) -> Result<bool> {
        debug!("Authorizing user: {} for roles: {:?}, permissions: {:?}", 
               user.id, required_roles, required_permissions);

        // Check roles
        if !required_roles.is_empty() {
            let has_role = required_roles.iter().any(|role| user.roles.contains(role));
            if !has_role {
                warn!("User {} lacks required roles: {:?}", user.id, required_roles);
                return Ok(false);
            }
        }

        // Check permissions
        if !required_permissions.is_empty() {
            let has_permission = required_permissions.iter().any(|permission| user.permissions.contains(permission));
            if !has_permission {
                warn!("User {} lacks required permissions: {:?}", user.id, required_permissions);
                return Ok(false);
            }
        }

        debug!("Authorization successful for user: {}", user.id);
        Ok(true)
    }

    /// Validate a JWT token
    pub async fn validate_token(&self, token: &str) -> Result<Claims> {
        debug!("Validating JWT token");

        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map_err(|e| ApiGatewayError::Auth(format!("Invalid JWT token: {}", e)))?;

        let claims = token_data.claims;

        // Check if token is expired
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if claims.exp < now {
            return Err(ApiGatewayError::Auth("Token expired".to_string()));
        }

        debug!("JWT token validation successful");
        Ok(claims)
    }

    /// Extract user from request context
    pub async fn extract_user_from_context(&self, context: &RequestContext) -> Result<Option<User>> {
        // Try to extract from Authorization header
        if let Some(auth_header) = context.headers.get("Authorization") {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                return Ok(Some(self.authenticate_jwt(token).await?));
            }
        }

        // Try to extract from API key header
        if let Some(api_key) = context.headers.get(&self.api_keys_config.header_name) {
            let _api_key_info = self.authenticate_api_key(api_key).await?;
            // TODO: Convert API key info to User
            return Ok(None);
        }

        Ok(None)
    }

    /// Check if user has required role
    pub fn has_role(&self, user: &User, role: &str) -> bool {
        user.roles.contains(&role.to_string())
    }

    /// Check if user has required permission
    pub fn has_permission(&self, user: &User, permission: &str) -> bool {
        user.permissions.contains(&permission.to_string())
    }

    /// Get user roles
    pub fn get_user_roles<'a>(&self, user: &'a User) -> &'a [String] {
        &user.roles
    }

    /// Get user permissions
    pub fn get_user_permissions<'a>(&self, user: &'a User) -> &'a [String] {
        &user.permissions
    }
}


