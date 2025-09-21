use prometheus::{
    HistogramVec, IntCounter, IntCounterVec, IntGauge,
    Opts, Registry, HistogramOpts,
};
use std::sync::Arc;

/// Metrics for the user-management service
#[allow(dead_code)]
pub struct UserManagementMetrics {
    /// Total HTTP requests
    pub http_requests_total: IntCounterVec,
    
    /// HTTP request duration
    pub http_request_duration_seconds: HistogramVec,
    
    /// Authentication attempts
    pub auth_attempts_total: IntCounterVec,
    
    /// User registrations
    pub registrations_total: IntCounter,
    
    /// User logins
    pub logins_total: IntCounter,
    
    /// User logouts
    pub logouts_total: IntCounter,
    
    /// Active sessions
    pub active_sessions: IntGauge,
    
    /// Database operations
    pub database_operations_total: IntCounterVec,
    
    /// JWT token operations
    pub jwt_operations_total: IntCounterVec,
    
    /// Password hash operations
    pub password_operations_total: IntCounterVec,
    
    /// Registry for all metrics
    pub registry: Registry,
}

#[allow(dead_code)]
impl UserManagementMetrics {
    /// Create a new metrics instance
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let registry = Registry::new();

        // HTTP requests metrics
        let http_requests_total = IntCounterVec::new(
            Opts::new("user_management_http_requests_total", "Total HTTP requests")
                .namespace("user_management"),
            &["method", "endpoint", "status_code"],
        )?;

        let http_request_duration_seconds = HistogramVec::new(
            HistogramOpts::new("user_management_http_request_duration_seconds", "HTTP request duration in seconds")
                .namespace("user_management")
                .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]),
            &["method", "endpoint"],
        )?;

        // Authentication metrics
        let auth_attempts_total = IntCounterVec::new(
            Opts::new("user_management_auth_attempts_total", "Total authentication attempts")
                .namespace("user_management"),
            &["type", "result"], // type: login, register, refresh, logout; result: success, failure
        )?;

        let registrations_total = IntCounter::new(
            "user_management_registrations_total",
            "Total user registrations",
        )?;

        let logins_total = IntCounter::new(
            "user_management_logins_total",
            "Total user logins",
        )?;

        let logouts_total = IntCounter::new(
            "user_management_logouts_total",
            "Total user logouts",
        )?;

        let active_sessions = IntGauge::new(
            "user_management_active_sessions",
            "Number of active user sessions",
        )?;

        // Database metrics
        let database_operations_total = IntCounterVec::new(
            Opts::new("user_management_database_operations_total", "Total database operations")
                .namespace("user_management"),
            &["operation", "collection", "result"], // operation: create, read, update, delete; result: success, error
        )?;

        // JWT metrics
        let jwt_operations_total = IntCounterVec::new(
            Opts::new("user_management_jwt_operations_total", "Total JWT operations")
                .namespace("user_management"),
            &["operation", "result"], // operation: create, validate, refresh; result: success, failure
        )?;

        // Password metrics
        let password_operations_total = IntCounterVec::new(
            Opts::new("user_management_password_operations_total", "Total password operations")
                .namespace("user_management"),
            &["operation", "result"], // operation: hash, verify; result: success, failure
        )?;

        // Register all metrics
        registry.register(Box::new(http_requests_total.clone()))?;
        registry.register(Box::new(http_request_duration_seconds.clone()))?;
        registry.register(Box::new(auth_attempts_total.clone()))?;
        registry.register(Box::new(registrations_total.clone()))?;
        registry.register(Box::new(logins_total.clone()))?;
        registry.register(Box::new(logouts_total.clone()))?;
        registry.register(Box::new(active_sessions.clone()))?;
        registry.register(Box::new(database_operations_total.clone()))?;
        registry.register(Box::new(jwt_operations_total.clone()))?;
        registry.register(Box::new(password_operations_total.clone()))?;

        Ok(Self {
            http_requests_total,
            http_request_duration_seconds,
            auth_attempts_total,
            registrations_total,
            logins_total,
            logouts_total,
            active_sessions,
            database_operations_total,
            jwt_operations_total,
            password_operations_total,
            registry,
        })
    }

    /// Record an HTTP request
    pub fn record_http_request(&self, method: &str, endpoint: &str, status_code: u16) {
        self.http_requests_total
            .with_label_values(&[method, endpoint, &status_code.to_string()])
            .inc();
    }

    /// Record HTTP request duration
    pub fn record_http_duration(&self, method: &str, endpoint: &str, duration: f64) {
        self.http_request_duration_seconds
            .with_label_values(&[method, endpoint])
            .observe(duration);
    }

    /// Record authentication attempt
    pub fn record_auth_attempt(&self, auth_type: &str, result: &str) {
        self.auth_attempts_total
            .with_label_values(&[auth_type, result])
            .inc();
    }

    /// Record user registration
    pub fn record_registration(&self) {
        self.registrations_total.inc();
    }

    /// Record user login
    pub fn record_login(&self) {
        self.logins_total.inc();
    }

    /// Record user logout
    pub fn record_logout(&self) {
        self.logouts_total.inc();
    }

    /// Set active sessions count
    pub fn set_active_sessions(&self, count: i64) {
        self.active_sessions.set(count);
    }

    /// Record database operation
    pub fn record_database_operation(&self, operation: &str, collection: &str, result: &str) {
        self.database_operations_total
            .with_label_values(&[operation, collection, result])
            .inc();
    }

    /// Record JWT operation
    pub fn record_jwt_operation(&self, operation: &str, result: &str) {
        self.jwt_operations_total
            .with_label_values(&[operation, result])
            .inc();
    }

    /// Record password operation
    pub fn record_password_operation(&self, operation: &str, result: &str) {
        self.password_operations_total
            .with_label_values(&[operation, result])
            .inc();
    }
}

lazy_static::lazy_static! {
    pub static ref METRICS: Arc<UserManagementMetrics> = Arc::new(
        UserManagementMetrics::new().expect("Failed to create metrics")
    );
}
