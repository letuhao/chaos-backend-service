//! Configuration management for API Gateway

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

/// API Gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub metrics: MetricsConfig,
    pub health: HealthConfig,
    pub service_discovery: ServiceDiscoveryConfig,
    pub routing: RoutingConfig,
    pub auth: AuthConfig,
    pub rate_limiting: RateLimitingConfig,
    pub security: SecurityConfig,
    pub load_balancing: LoadBalancingConfig,
    pub caching: CachingConfig,
    pub monitoring: MonitoringConfig,
    pub development: DevelopmentConfig,
    pub production: ProductionConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub max_connections: usize,
    pub keep_alive: u64,
    pub timeout: u64,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub file: String,
    pub max_size: String,
    pub max_files: usize,
    pub compress: bool,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub port: u16,
    pub path: String,
    pub interval: u64,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    pub enabled: bool,
    pub path: String,
    pub interval: u64,
    pub timeout: u64,
}

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    #[serde(rename = "type")]
    pub discovery_type: String,
    pub consul: Option<ConsulConfig>,
    pub kubernetes: Option<KubernetesConfig>,
    pub static_config: Option<StaticConfig>,
}

/// Consul configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsulConfig {
    pub host: String,
    pub port: u16,
    pub datacenter: String,
    pub service_name: String,
    pub check_interval: String,
    pub check_timeout: String,
}

/// Kubernetes configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesConfig {
    pub namespace: String,
    pub label_selector: String,
}

/// Static service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticConfig {
    pub services: Vec<ServiceConfig>,
}

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub health_check: String,
}

/// Routing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    pub default_timeout: u64,
    pub retry_attempts: u32,
    pub retry_delay: u64,
    pub circuit_breaker: CircuitBreakerConfig,
    pub routes: Vec<RouteConfig>,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub enabled: bool,
    pub failure_threshold: u32,
    pub recovery_timeout: u64,
    pub half_open_max_calls: u32,
}

/// Route configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    pub path: String,
    pub methods: Vec<String>,
    pub handler: Option<String>,
    pub service: Option<String>,
    pub timeout: Option<u64>,
    pub auth_required: Option<bool>,
    pub roles: Option<Vec<String>>,
    pub priority: Option<String>,
    pub websocket: Option<bool>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt: JwtConfig,
    pub oauth2: OAuth2Config,
    pub api_keys: ApiKeysConfig,
}

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub algorithm: String,
    pub expiration: u64,
    pub refresh_expiration: u64,
    pub issuer: String,
    pub audience: String,
}

/// OAuth2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    pub enabled: bool,
    pub providers: HashMap<String, OAuth2Provider>,
}

/// OAuth2 provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Provider {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

/// API keys configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeysConfig {
    pub enabled: bool,
    pub header_name: String,
    pub keys: Vec<ApiKey>,
}

/// API key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub expires_at: String,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    pub enabled: bool,
    pub redis: RedisConfig,
    pub rules: Vec<RateLimitRule>,
}

/// Rate limit rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitRule {
    pub name: String,
    pub limit: u32,
    pub window: u64,
    pub burst: u32,
    pub paths: Option<Vec<String>>,
    pub roles: Option<Vec<String>>,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub cors: CorsConfig,
    pub headers: Vec<SecurityHeader>,
    pub ip_whitelist: IpWhitelistConfig,
    pub ip_blacklist: IpBlacklistConfig,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub enabled: bool,
    pub origins: Vec<String>,
    pub methods: Vec<String>,
    pub headers: Vec<String>,
    pub credentials: bool,
    pub max_age: u64,
}

/// Security header configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeader {
    pub name: String,
    pub value: String,
}

/// IP whitelist configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpWhitelistConfig {
    pub enabled: bool,
    pub ips: Vec<String>,
}

/// IP blacklist configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpBlacklistConfig {
    pub enabled: bool,
    pub ips: Vec<String>,
    pub auto_ban: bool,
    pub ban_duration: u64,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    pub algorithm: String,
    pub health_check: HealthCheckConfig,
    pub sticky_sessions: StickySessionsConfig,
}

/// Health check configuration for load balancing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub interval: u64,
    pub timeout: u64,
    pub path: String,
    pub success_threshold: u32,
    pub failure_threshold: u32,
}

/// Sticky sessions configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickySessionsConfig {
    pub enabled: bool,
    pub cookie_name: String,
    pub max_age: u64,
}

/// Caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    pub enabled: bool,
    pub redis: RedisConfig,
    pub ttl: TtlConfig,
}

/// TTL configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtlConfig {
    pub default: u64,
    pub auth: u64,
    pub user: u64,
    pub game: u64,
    pub inventory: u64,
    pub chat: u64,
    pub guild: u64,
    pub world: u64,
    pub matchmaking: u64,
    pub events: u64,
    pub content: u64,
    pub notifications: u64,
    pub payments: u64,
    pub anti_cheat: u64,
    pub analytics: u64,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub tracing: TracingConfig,
    pub metrics: MetricsMonitoringConfig,
}

/// Tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    pub enabled: bool,
    pub jaeger: JaegerConfig,
}

/// Jaeger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JaegerConfig {
    pub endpoint: String,
    pub service_name: String,
    pub sample_rate: f64,
}

/// Metrics monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsMonitoringConfig {
    pub prometheus: PrometheusConfig,
    pub custom_metrics: Vec<CustomMetric>,
}

/// Prometheus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    pub enabled: bool,
    pub port: u16,
    pub path: String,
}

/// Custom metric configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    pub name: String,
    #[serde(rename = "type")]
    pub metric_type: String,
    pub labels: Vec<String>,
}

/// Development configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentConfig {
    pub hot_reload: bool,
    pub debug: bool,
    pub mock_services: bool,
    pub log_requests: bool,
    pub log_responses: bool,
    pub log_headers: bool,
    pub log_body: bool,
}

/// Production configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {
    pub hot_reload: bool,
    pub debug: bool,
    pub mock_services: bool,
    pub log_requests: bool,
    pub log_responses: bool,
    pub log_headers: bool,
    pub log_body: bool,
    pub minify_responses: bool,
    pub compress_responses: bool,
}

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub database: u8,
    pub password: String,
    pub pool_size: usize,
}

impl Config {
    /// Load configuration from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration from environment
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let env = std::env::var("ENV").unwrap_or_else(|_| "development".to_string());
        let config_path = match env.as_str() {
            "development" => "configs/api-gateway-dev.yaml",
            "production" => "configs/api-gateway-prod.yaml",
            _ => "configs/api-gateway.yaml",
        };
        Self::from_file(config_path)
    }

    /// Get server address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Get metrics address
    pub fn metrics_address(&self) -> String {
        format!("{}:{}", self.server.host, self.metrics.port)
    }

    /// Check if development mode
    pub fn is_development(&self) -> bool {
        std::env::var("ENV").unwrap_or_else(|_| "development".to_string()) == "development"
    }

    /// Check if production mode
    pub fn is_production(&self) -> bool {
        std::env::var("ENV").unwrap_or_else(|_| "development".to_string()) == "production"
    }

    /// Get timeout duration
    pub fn timeout_duration(&self) -> Duration {
        Duration::from_secs(self.server.timeout)
    }

    /// Get keep alive duration
    pub fn keep_alive_duration(&self) -> Duration {
        Duration::from_secs(self.server.keep_alive)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: 4,
                max_connections: 10000,
                keep_alive: 30,
                timeout: 30,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
                file: "logs/api-gateway.log".to_string(),
                max_size: "100MB".to_string(),
                max_files: 10,
                compress: true,
            },
            metrics: MetricsConfig {
                enabled: true,
                port: 9090,
                path: "/metrics".to_string(),
                interval: 30,
            },
            health: HealthConfig {
                enabled: true,
                path: "/health".to_string(),
                interval: 10,
                timeout: 5,
            },
            service_discovery: ServiceDiscoveryConfig {
                discovery_type: "static".to_string(),
                consul: None,
                kubernetes: None,
                static_config: Some(StaticConfig {
                    services: vec![],
                }),
            },
            routing: RoutingConfig {
                default_timeout: 30,
                retry_attempts: 3,
                retry_delay: 100,
                circuit_breaker: CircuitBreakerConfig {
                    enabled: true,
                    failure_threshold: 5,
                    recovery_timeout: 30,
                    half_open_max_calls: 3,
                },
                routes: vec![],
            },
            auth: AuthConfig {
                jwt: JwtConfig {
                    secret: "default-secret".to_string(),
                    algorithm: "HS256".to_string(),
                    expiration: 3600,
                    refresh_expiration: 86400,
                    issuer: "api-gateway".to_string(),
                    audience: "client".to_string(),
                },
                oauth2: OAuth2Config {
                    enabled: false,
                    providers: HashMap::new(),
                },
                api_keys: ApiKeysConfig {
                    enabled: true,
                    header_name: "X-API-Key".to_string(),
                    keys: vec![],
                },
            },
            rate_limiting: RateLimitingConfig {
                enabled: true,
                redis: RedisConfig {
                    host: "localhost".to_string(),
                    port: 6379,
                    database: 0,
                    password: String::new(),
                    pool_size: 10,
                },
                rules: vec![],
            },
            security: SecurityConfig {
                cors: CorsConfig {
                    enabled: true,
                    origins: vec!["*".to_string()],
                    methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
                    headers: vec!["*".to_string()],
                    credentials: false,
                    max_age: 86400,
                },
                headers: vec![],
                ip_whitelist: IpWhitelistConfig {
                    enabled: false,
                    ips: vec![],
                },
                ip_blacklist: IpBlacklistConfig {
                    enabled: true,
                    ips: vec![],
                    auto_ban: true,
                    ban_duration: 3600,
                },
            },
            load_balancing: LoadBalancingConfig {
                algorithm: "round_robin".to_string(),
                health_check: HealthCheckConfig {
                    enabled: true,
                    interval: 30,
                    timeout: 5,
                    path: "/health".to_string(),
                    success_threshold: 2,
                    failure_threshold: 3,
                },
                sticky_sessions: StickySessionsConfig {
                    enabled: false,
                    cookie_name: "session_id".to_string(),
                    max_age: 3600,
                },
            },
            caching: CachingConfig {
                enabled: true,
                redis: RedisConfig {
                    host: "localhost".to_string(),
                    port: 6379,
                    database: 1,
                    password: String::new(),
                    pool_size: 10,
                },
                ttl: TtlConfig {
                    default: 300,
                    auth: 60,
                    user: 600,
                    game: 30,
                    inventory: 300,
                    chat: 60,
                    guild: 600,
                    world: 30,
                    matchmaking: 60,
                    events: 300,
                    content: 1800,
                    notifications: 60,
                    payments: 0,
                    anti_cheat: 0,
                    analytics: 0,
                },
            },
            monitoring: MonitoringConfig {
                tracing: TracingConfig {
                    enabled: true,
                    jaeger: JaegerConfig {
                        endpoint: "http://localhost:14268/api/traces".to_string(),
                        service_name: "api-gateway".to_string(),
                        sample_rate: 0.1,
                    },
                },
                metrics: MetricsMonitoringConfig {
                    prometheus: PrometheusConfig {
                        enabled: true,
                        port: 9090,
                        path: "/metrics".to_string(),
                    },
                    custom_metrics: vec![],
                },
            },
            development: DevelopmentConfig {
                hot_reload: true,
                debug: true,
                mock_services: false,
                log_requests: true,
                log_responses: true,
                log_headers: true,
                log_body: true,
            },
            production: ProductionConfig {
                hot_reload: false,
                debug: false,
                mock_services: false,
                log_requests: false,
                log_responses: false,
                log_headers: false,
                log_body: false,
                minify_responses: true,
                compress_responses: true,
            },
        }
    }
}
