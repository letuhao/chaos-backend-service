//! Common types for API Gateway

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Request context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestContext {
    pub request_id: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub timestamp: SystemTime,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub path_params: HashMap<String, String>,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub created_at: SystemTime,
    pub last_login: Option<SystemTime>,
    pub is_active: bool,
}

/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub health_check: String,
    pub status: ServiceStatus,
    pub last_health_check: Option<SystemTime>,
    pub response_time: Option<u64>,
    pub error_count: u32,
    pub success_count: u32,
}

/// Service status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Healthy,
    Unhealthy,
    Unknown,
    Maintenance,
}

/// Route information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub path: String,
    pub methods: Vec<String>,
    pub service: Option<String>,
    pub handler: Option<String>,
    pub timeout: u64,
    pub auth_required: bool,
    pub roles: Vec<String>,
    pub priority: RoutePriority,
    pub websocket: bool,
}

/// Route priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoutePriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Circuit breaker state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

/// Circuit breaker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreaker {
    pub service_name: String,
    pub state: CircuitBreakerState,
    pub failure_count: u32,
    pub success_count: u32,
    pub last_failure: Option<SystemTime>,
    pub last_success: Option<SystemTime>,
    pub next_attempt: Option<SystemTime>,
}

/// Rate limit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub rule_name: String,
    pub limit: u32,
    pub window: u64,
    pub current: u32,
    pub remaining: u32,
    pub reset_time: SystemTime,
    pub burst: u32,
}

/// Cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub key: String,
    pub value: T,
    pub ttl: u64,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub service: String,
    pub status: ServiceStatus,
    pub response_time: Option<u64>,
    pub error: Option<String>,
    pub timestamp: SystemTime,
}

/// Metrics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub request_count: u64,
    pub request_duration: f64,
    pub error_count: u64,
    pub active_connections: u32,
    pub rate_limit_hits: u64,
    pub circuit_breaker_state: HashMap<String, CircuitBreakerState>,
    pub service_health: HashMap<String, ServiceStatus>,
}

/// API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
    pub timestamp: SystemTime,
    pub request_id: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, request_id: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
            timestamp: SystemTime::now(),
            request_id,
        }
    }

    pub fn error(error: String, request_id: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            message: None,
            timestamp: SystemTime::now(),
            request_id,
        }
    }

    pub fn message(message: String, request_id: String) -> Self {
        Self {
            success: true,
            data: None,
            error: None,
            message: Some(message),
            timestamp: SystemTime::now(),
            request_id,
        }
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
    pub total: Option<u64>,
    pub total_pages: Option<u32>,
}

/// Sort parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sort {
    pub field: String,
    pub order: SortOrder,
}

/// Sort order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Filter parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: String,
}

/// Filter operator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FilterOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Like,
    In,
    NotIn,
    Between,
    IsNull,
    IsNotNull,
}

/// WebSocket message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
    pub timestamp: SystemTime,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
}

/// WebSocket connection info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConnection {
    pub connection_id: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub connected_at: SystemTime,
    pub last_activity: SystemTime,
    pub is_authenticated: bool,
}

/// Load balancer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub active_connections: u32,
    pub service_stats: HashMap<String, ServiceStats>,
}

/// Service statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStats {
    pub requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub error_rate: f64,
    pub last_request: Option<SystemTime>,
}

/// Security event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_type: SecurityEventType,
    pub severity: SecuritySeverity,
    pub description: String,
    pub ip_address: String,
    pub user_id: Option<String>,
    pub timestamp: SystemTime,
    pub metadata: HashMap<String, String>,
}

/// Security event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityEventType {
    AuthenticationFailure,
    AuthorizationFailure,
    RateLimitExceeded,
    SuspiciousActivity,
    MaliciousRequest,
    DDoS,
    BruteForce,
    UnauthorizedAccess,
    DataBreach,
    SystemCompromise,
}

/// Security severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Configuration validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValidation {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Service discovery event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryEvent {
    pub event_type: ServiceDiscoveryEventType,
    pub service_name: String,
    pub service_info: Service,
    pub timestamp: SystemTime,
}

/// Service discovery event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceDiscoveryEventType {
    ServiceRegistered,
    ServiceDeregistered,
    ServiceHealthChanged,
    ServiceUpdated,
}

/// Tracing context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingContext {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub baggage: HashMap<String, String>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub disk_usage: u64,
    pub network_io: NetworkIO,
    pub request_metrics: RequestMetrics,
}

/// Network IO metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIO {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
}

/// Request metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetrics {
    pub total_requests: u64,
    pub requests_per_second: f64,
    pub average_response_time: f64,
    pub p50_response_time: f64,
    pub p95_response_time: f64,
    pub p99_response_time: f64,
    pub error_rate: f64,
}
