use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API Gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGatewayConfig {
    pub server: ServerConfig,
    pub routing: RoutingConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// Routing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    /// Service discovery configuration
    pub service_discovery: ServiceDiscoveryConfig,
    /// Route definitions
    pub routes: Vec<RouteConfig>,
}

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    /// Static service discovery
    pub static_services: HashMap<String, ServiceConfig>,
}

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub host: String,
    pub port: u16,
    pub health_check: Option<String>,
}

/// Route configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    /// Route path pattern (e.g., "/auth/*", "/api/v1/*")
    pub path: String,
    /// Target service name
    pub service: String,
    /// HTTP methods allowed for this route
    pub methods: Vec<String>,
    /// Whether to strip the path prefix when forwarding
    pub strip_prefix: bool,
    /// Custom headers to add to the request
    pub add_headers: Option<HashMap<String, String>>,
    /// Rate limiting configuration
    pub rate_limit: Option<RateLimitConfig>,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: Option<u32>,
}

impl Default for ApiGatewayConfig {
    fn default() -> Self {
        let mut static_services = HashMap::new();
        static_services.insert(
            "user-management".to_string(),
            ServiceConfig {
                host: "localhost".to_string(),
                port: 8082,
                health_check: Some("/health".to_string()),
            },
        );
        static_services.insert(
            "chaos-backend".to_string(),
            ServiceConfig {
                host: "localhost".to_string(),
                port: 8081,
                health_check: Some("/health".to_string()),
            },
        );

        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
            },
            routing: RoutingConfig {
                service_discovery: ServiceDiscoveryConfig {
                    static_services,
                },
                routes: vec![
                    RouteConfig {
                        path: "/auth/*".to_string(),
                        service: "user-management".to_string(),
                        methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
                        strip_prefix: false,
                        add_headers: None,
                        rate_limit: Some(RateLimitConfig {
                            requests_per_minute: 100,
                            burst_size: Some(10),
                        }),
                    },
                    RouteConfig {
                        path: "/api/*".to_string(),
                        service: "chaos-backend".to_string(),
                        methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
                        strip_prefix: false,
                        add_headers: None,
                        rate_limit: None,
                    },
                ],
            },
        }
    }
}

impl ApiGatewayConfig {
    /// Load configuration from file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: ApiGatewayConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = std::env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "configs/api-gateway.yaml".to_string());
        
        tracing::info!("🔍 Config::from_env() - Looking for config at: {}", config_path);
        tracing::info!("🔍 Config::from_env() - File exists: {}", std::path::Path::new(&config_path).exists());
        
        if std::path::Path::new(&config_path).exists() {
            tracing::info!("🔍 Config::from_env() - Loading config from file: {}", config_path);
            match Self::from_file(&config_path) {
                Ok(config) => {
                    tracing::info!("✅ Config::from_env() - Successfully loaded config from file");
                    Ok(config)
                }
                Err(e) => {
                    tracing::error!("❌ Config::from_env() - Failed to parse config file: {}", e);
                    tracing::warn!("Config::from_env() - Falling back to default configuration");
                    Ok(Self::default())
                }
            }
        } else {
            tracing::warn!("Config file not found at {}, using default configuration", config_path);
            Ok(Self::default())
        }
    }

    /// Get service configuration by name
    pub fn get_service(&self, service_name: &str) -> Option<&ServiceConfig> {
        self.routing.service_discovery.static_services.get(service_name)
    }

    /// Find route configuration that matches the given path
    pub fn find_route(&self, path: &str) -> Option<&RouteConfig> {
        self.routing.routes.iter().find(|route| {
            // Simple wildcard matching for now
            if route.path.ends_with("/*") {
                let prefix = &route.path[..route.path.len() - 2];
                path.starts_with(prefix)
            } else {
                path == route.path
            }
        })
    }

    /// Check if a method is allowed for a route
    pub fn is_method_allowed(&self, route: &RouteConfig, method: &str) -> bool {
        route.methods.iter().any(|m| m.eq_ignore_ascii_case(method))
    }
}