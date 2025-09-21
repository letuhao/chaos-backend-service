use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub mongodb_uri: String,
    pub mongodb_database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiry: i64,
    pub admin_username: String,
    pub admin_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub metrics_port: u16,
    pub health_check_enabled: bool,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "configs/content-management-service.yaml".to_string());
        
        if std::path::Path::new(&config_path).exists() {
            tracing::info!("Loading configuration from file: {}", config_path);
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_yaml::from_str(&content)?;
            return Ok(config);
        }
        
        tracing::warn!("Config file not found at {}, using environment variables", config_path);
        let server = ServerConfig {
            port: env::var("CMS_PORT")
                .unwrap_or_else(|_| "8083".to_string())
                .parse()
                .unwrap_or(8083),
            host: env::var("CMS_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            workers: env::var("CMS_WORKERS")
                .unwrap_or_else(|_| "4".to_string())
                .parse()
                .unwrap_or(4),
        };

        let database = DatabaseConfig {
            mongodb_uri: env::var("MONGODB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            mongodb_database: env::var("MONGODB_DATABASE")
                .unwrap_or_else(|_| "chaos_cms".to_string()),
        };

        let auth = AuthConfig {
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your_jwt_secret_here_change_in_production".to_string()),
            jwt_expiry: env::var("JWT_EXPIRY")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
            admin_username: env::var("ADMIN_USERNAME")
                .unwrap_or_else(|_| "admin".to_string()),
            admin_password: env::var("ADMIN_PASSWORD")
                .unwrap_or_else(|_| "admin123".to_string()),
        };

        let monitoring = MonitoringConfig {
            metrics_enabled: env::var("METRICS_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            metrics_port: env::var("METRICS_PORT")
                .unwrap_or_else(|_| "9090".to_string())
                .parse()
                .unwrap_or(9090),
            health_check_enabled: env::var("HEALTH_CHECK_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
        };

        Ok(Config {
            server,
            database,
            auth,
            monitoring,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Environment variable not found: {0}")]
    MissingEnvVar(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

// Allow unused variants for future use
#[allow(dead_code)]
impl ConfigError {
    pub fn invalid_config(msg: impl Into<String>) -> Self {
        Self::InvalidConfig(msg.into())
    }
    
    pub fn missing_env_var(var: impl Into<String>) -> Self {
        Self::MissingEnvVar(var.into())
    }
}
