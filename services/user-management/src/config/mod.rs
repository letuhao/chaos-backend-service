use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

/// User Management Service Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserServiceConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub password: PasswordConfig,
    pub rate_limiting: RateLimitingConfig,
    pub email: EmailConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub workers: usize,
    pub max_connections: u32,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
    pub timeout_seconds: u64,
    pub ssl_mode: String,
}

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub password: Option<String>,
    pub db: u8,
    pub pool_size: u32,
}

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub access_expiry_seconds: u64,
    pub refresh_expiry_seconds: u64,
    pub issuer: String,
    pub audience: String,
}

/// Password configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordConfig {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special: bool,
    pub max_length: usize,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    pub enabled: bool,
    pub window_seconds: u64,
    pub max_requests: u32,
    pub login_attempts: u32,
    pub password_reset: u32,
}

/// Email configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
}

impl Default for UserServiceConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            redis: RedisConfig::default(),
            jwt: JwtConfig::default(),
            password: PasswordConfig::default(),
            rate_limiting: RateLimitingConfig::default(),
            email: EmailConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8082,
            host: "0.0.0.0".to_string(),
            workers: 4,
            max_connections: 10000,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "mongodb://localhost:27017".to_string(),
            pool_size: 10,
            timeout_seconds: 30,
            ssl_mode: "prefer".to_string(),
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            password: None,
            db: 0,
            pool_size: 100,
        }
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "your-secret-key-change-in-production".to_string(),
            access_expiry_seconds: 3600, // 1 hour
            refresh_expiry_seconds: 604800, // 7 days
            issuer: "chaos-world".to_string(),
            audience: "chaos-world-api".to_string(),
        }
    }
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: false,
            max_length: 128,
        }
    }
}

impl Default for RateLimitingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            window_seconds: 3600, // 1 hour
            max_requests: 1000,
            login_attempts: 10,
            password_reset: 3,
        }
    }
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            smtp_host: "smtp.gmail.com".to_string(),
            smtp_port: 587,
            smtp_username: "your-email@gmail.com".to_string(),
            smtp_password: "your-app-password".to_string(),
            from_email: "noreply@chaosworld.com".to_string(),
            from_name: "Chaos World".to_string(),
        }
    }
}

impl UserServiceConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = UserServiceConfig {
            server: ServerConfig {
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8082".to_string())
                    .parse()?,
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                workers: env::var("SERVER_WORKERS")
                    .unwrap_or_else(|_| "4".to_string())
                    .parse()?,
                max_connections: env::var("SERVER_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10000".to_string())
                    .parse()?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
                pool_size: env::var("DATABASE_POOL_SIZE")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()?,
                timeout_seconds: env::var("DATABASE_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()?,
                ssl_mode: env::var("DATABASE_SSL_MODE")
                    .unwrap_or_else(|_| "prefer".to_string()),
            },
            redis: RedisConfig {
                url: env::var("REDIS_URL")
                    .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
                password: env::var("REDIS_PASSWORD").ok(),
                db: env::var("REDIS_DB")
                    .unwrap_or_else(|_| "0".to_string())
                    .parse()?,
                pool_size: env::var("REDIS_POOL_SIZE")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()?,
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
                access_expiry_seconds: env::var("JWT_ACCESS_EXPIRY")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()?,
                refresh_expiry_seconds: env::var("JWT_REFRESH_EXPIRY")
                    .unwrap_or_else(|_| "604800".to_string())
                    .parse()?,
                issuer: env::var("JWT_ISSUER")
                    .unwrap_or_else(|_| "chaos-world".to_string()),
                audience: env::var("JWT_AUDIENCE")
                    .unwrap_or_else(|_| "chaos-world-api".to_string()),
            },
            password: PasswordConfig {
                min_length: env::var("PASSWORD_MIN_LENGTH")
                    .unwrap_or_else(|_| "8".to_string())
                    .parse()?,
                require_uppercase: env::var("PASSWORD_REQUIRE_UPPERCASE")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()?,
                require_lowercase: env::var("PASSWORD_REQUIRE_LOWERCASE")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()?,
                require_numbers: env::var("PASSWORD_REQUIRE_NUMBERS")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()?,
                require_special: env::var("PASSWORD_REQUIRE_SPECIAL")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()?,
                max_length: env::var("PASSWORD_MAX_LENGTH")
                    .unwrap_or_else(|_| "128".to_string())
                    .parse()?,
            },
            rate_limiting: RateLimitingConfig {
                enabled: env::var("RATE_LIMIT_ENABLED")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()?,
                window_seconds: env::var("RATE_LIMIT_WINDOW")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()?,
                max_requests: env::var("RATE_LIMIT_MAX_REQUESTS")
                    .unwrap_or_else(|_| "1000".to_string())
                    .parse()?,
                login_attempts: env::var("RATE_LIMIT_LOGIN_ATTEMPTS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()?,
                password_reset: env::var("RATE_LIMIT_PASSWORD_RESET")
                    .unwrap_or_else(|_| "3".to_string())
                    .parse()?,
            },
            email: EmailConfig {
                smtp_host: env::var("SMTP_HOST")
                    .unwrap_or_else(|_| "smtp.gmail.com".to_string()),
                smtp_port: env::var("SMTP_PORT")
                    .unwrap_or_else(|_| "587".to_string())
                    .parse()?,
                smtp_username: env::var("SMTP_USERNAME")
                    .unwrap_or_else(|_| "your-email@gmail.com".to_string()),
                smtp_password: env::var("SMTP_PASSWORD")
                    .unwrap_or_else(|_| "your-app-password".to_string()),
                from_email: env::var("EMAIL_FROM")
                    .unwrap_or_else(|_| "noreply@chaosworld.com".to_string()),
                from_name: env::var("EMAIL_FROM_NAME")
                    .unwrap_or_else(|_| "Chaos World".to_string()),
            },
        };

        Ok(config)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate server config
        if self.server.port == 0 {
            errors.push("Server port must be greater than 0".to_string());
        }
        if self.server.workers == 0 {
            errors.push("Server workers must be greater than 0".to_string());
        }

        // Validate database config
        if self.database.pool_size == 0 {
            errors.push("Database pool size must be greater than 0".to_string());
        }

        // Validate JWT config
        if self.jwt.secret.len() < 32 {
            errors.push("JWT secret must be at least 32 characters long".to_string());
        }
        if self.jwt.access_expiry_seconds == 0 {
            errors.push("JWT access expiry must be greater than 0".to_string());
        }
        if self.jwt.refresh_expiry_seconds <= self.jwt.access_expiry_seconds {
            errors.push("JWT refresh expiry must be greater than access expiry".to_string());
        }

        // Validate password config
        if self.password.min_length < 6 {
            errors.push("Password minimum length must be at least 6".to_string());
        }
        if self.password.max_length < self.password.min_length {
            errors.push("Password maximum length must be greater than minimum length".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Load configuration from YAML file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: UserServiceConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration from file or environment variables
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "configs/user-management.yaml".to_string());
        
        if std::path::Path::new(&config_path).exists() {
            tracing::info!("Loading configuration from file: {}", config_path);
            Self::from_file(&config_path)
        } else {
            tracing::warn!("Config file not found at {}, using environment variables", config_path);
            Self::from_env()
        }
    }
}
