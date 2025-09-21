use crate::config::UserServiceConfig;
use crate::models::{User, UserSession, TokenClaims, TokenResponse, UserStatus};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;
use chrono::{Duration, Utc};
use std::sync::Arc;

/// Authentication service for handling user authentication and JWT tokens
pub struct AuthService {
    config: Arc<UserServiceConfig>,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    argon2: Argon2<'static>,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(config: Arc<UserServiceConfig>) -> Result<Self, Box<dyn std::error::Error>> {
        let encoding_key = EncodingKey::from_secret(config.jwt.secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.jwt.secret.as_bytes());
        let argon2 = Argon2::default();

        Ok(Self {
            config,
            encoding_key,
            decoding_key,
            argon2,
        })
    }

    /// Hash a password using Argon2id
    pub fn hash_password(&self, password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Failed to hash password: {}", e))?
            .to_string();
        Ok(password_hash)
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| format!("Failed to parse password hash: {}", e))?;
        let result = self.argon2.verify_password(password.as_bytes(), &parsed_hash);
        Ok(result.is_ok())
    }

    /// Generate access and refresh tokens for a user
    pub fn generate_tokens(&self, user: &User, session_id: Uuid) -> Result<TokenResponse, Box<dyn std::error::Error>> {
        let now = Utc::now();
        let access_exp = now + Duration::seconds(self.config.jwt.access_expiry_seconds as i64);
        let refresh_exp = now + Duration::seconds(self.config.jwt.refresh_expiry_seconds as i64);

        // Create access token claims
        let access_claims = TokenClaims {
            user_id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            roles: vec!["player".to_string()], // Default role
            permissions: vec![
                "game:play".to_string(),
                "shop:purchase".to_string(),
                "profile:edit".to_string(),
                "guild:join".to_string(),
            ],
            session_id,
            iat: now.timestamp(),
            exp: access_exp.timestamp(),
            iss: self.config.jwt.issuer.clone(),
            aud: self.config.jwt.audience.clone(),
        };

        // Create refresh token claims
        let refresh_claims = TokenClaims {
            user_id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            roles: vec!["player".to_string()],
            permissions: vec!["auth:refresh".to_string()],
            session_id,
            iat: now.timestamp(),
            exp: refresh_exp.timestamp(),
            iss: self.config.jwt.issuer.clone(),
            aud: self.config.jwt.audience.clone(),
        };

        // Generate tokens
        let access_token = encode(&Header::default(), &access_claims, &self.encoding_key)?;
        let refresh_token = encode(&Header::default(), &refresh_claims, &self.encoding_key)?;

        Ok(TokenResponse {
            access_token,
            refresh_token,
            expires_at: access_exp,
        })
    }

    /// Validate and decode a JWT token
    pub fn validate_token(&self, token: &str) -> Result<TokenClaims, Box<dyn std::error::Error>> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.config.jwt.issuer]);
        validation.set_audience(&[&self.config.jwt.audience]);

        let token_data = decode::<TokenClaims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }

    /// Validate refresh token
    pub fn validate_refresh_token(&self, token: &str) -> Result<TokenClaims, Box<dyn std::error::Error>> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.config.jwt.issuer]);
        validation.set_audience(&[&self.config.jwt.audience]);
        validation.set_required_spec_claims(&["exp", "iat", "iss", "aud"]);

        let token_data = decode::<TokenClaims>(token, &self.decoding_key, &validation)?;
        
        // Check if token has refresh permission
        if !token_data.claims.permissions.contains(&"auth:refresh".to_string()) {
            return Err("Invalid refresh token".into());
        }

        Ok(token_data.claims)
    }

    /// Create a new user session
    pub fn create_session(
        &self,
        user_id: Uuid,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<UserSession, Box<dyn std::error::Error>> {
        let now = Utc::now();
        let expires_at = now + Duration::seconds(self.config.jwt.refresh_expiry_seconds as i64);

        // Generate user fingerprint for better tracking
        let user_fingerprint = self.generate_user_fingerprint(&ip_address, &user_agent);

        Ok(UserSession {
            id: Uuid::new_v4(),
            user_id,
            session_token: Uuid::new_v4().to_string(),
            refresh_token: Uuid::new_v4().to_string(),
            expires_at,
            created_at: now,
            last_accessed: now,
            ip_address,
            user_agent,
            user_fingerprint,
            is_active: true,
        })
    }

    /// Generate a user fingerprint based on IP address and user agent
    fn generate_user_fingerprint(&self, ip_address: &Option<String>, user_agent: &Option<String>) -> Option<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        
        // Hash IP address and user agent together
        if let Some(ip) = ip_address {
            ip.hash(&mut hasher);
        }
        if let Some(ua) = user_agent {
            ua.hash(&mut hasher);
        }
        
        // Add some randomness to make it harder to reverse
        let timestamp = Utc::now().timestamp();
        timestamp.hash(&mut hasher);
        
        Some(format!("{:x}", hasher.finish()))
    }

    /// Validate password strength
    pub fn validate_password_strength(&self, password: &str) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if password.len() < self.config.password.min_length {
            errors.push(format!(
                "Password must be at least {} characters long",
                self.config.password.min_length
            ));
        }

        if password.len() > self.config.password.max_length {
            errors.push(format!(
                "Password must be no more than {} characters long",
                self.config.password.max_length
            ));
        }

        if self.config.password.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            errors.push("Password must contain at least one uppercase letter".to_string());
        }

        if self.config.password.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            errors.push("Password must contain at least one lowercase letter".to_string());
        }

        if self.config.password.require_numbers && !password.chars().any(|c| c.is_numeric()) {
            errors.push("Password must contain at least one number".to_string());
        }

        if self.config.password.require_special {
            let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
            if !password.chars().any(|c| special_chars.contains(c)) {
                errors.push("Password must contain at least one special character".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Check if user is active
    pub fn is_user_active(&self, user: &User) -> bool {
        matches!(user.status, UserStatus::Active)
    }

    // Unused methods removed for cleaner code
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::UserServiceConfig;

    fn create_test_config() -> UserServiceConfig {
        UserServiceConfig {
            jwt: crate::config::JwtConfig {
                secret: "test-secret-key-that-is-long-enough-for-testing".to_string(),
                access_expiry_seconds: 3600,
                refresh_expiry_seconds: 604800,
                issuer: "test".to_string(),
                audience: "test-api".to_string(),
            },
            password: crate::config::PasswordConfig {
                min_length: 8,
                require_uppercase: true,
                require_lowercase: true,
                require_numbers: true,
                require_special: false,
                max_length: 128,
            },
            ..Default::default()
        }
    }

    #[test]
    fn test_password_hashing() {
        let config = Arc::new(create_test_config());
        let auth_service = AuthService::new(config).unwrap();
        
        let password = "TestPassword123";
        let hash = auth_service.hash_password(password).unwrap();
        
        assert!(auth_service.verify_password(password, &hash).unwrap());
        assert!(!auth_service.verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_password_validation() {
        let config = Arc::new(create_test_config());
        let auth_service = AuthService::new(config).unwrap();
        
        // Valid password
        assert!(auth_service.validate_password_strength("TestPassword123").is_ok());
        
        // Too short
        assert!(auth_service.validate_password_strength("Test1").is_err());
        
        // No uppercase
        assert!(auth_service.validate_password_strength("testpassword123").is_err());
        
        // No lowercase
        assert!(auth_service.validate_password_strength("TESTPASSWORD123").is_err());
        
        // No numbers
        assert!(auth_service.validate_password_strength("TestPassword").is_err());
    }

    #[test]
    fn test_token_generation() {
        let config = Arc::new(create_test_config());
        let auth_service = AuthService::new(config).unwrap();
        
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            display_name: Some("Test User".to_string()),
            avatar_url: None,
            status: UserStatus::Active,
            email_verified: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: None,
            login_count: 0,
        };
        
        let session_id = Uuid::new_v4();
        let tokens = auth_service.generate_tokens(&user, session_id).unwrap();
        
        assert!(!tokens.access_token.is_empty());
        assert!(!tokens.refresh_token.is_empty());
        assert!(tokens.expires_at > Utc::now());
    }

    #[test]
    fn test_token_validation() {
        let config = Arc::new(create_test_config());
        let auth_service = AuthService::new(config).unwrap();
        
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            display_name: Some("Test User".to_string()),
            avatar_url: None,
            status: UserStatus::Active,
            email_verified: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: None,
            login_count: 0,
        };
        
        let session_id = Uuid::new_v4();
        let tokens = auth_service.generate_tokens(&user, session_id).unwrap();
        
        let claims = auth_service.validate_token(&tokens.access_token).unwrap();
        assert_eq!(claims.user_id, user.id);
        assert_eq!(claims.username, user.username);
        assert_eq!(claims.email, user.email);
    }
}
