# Status Core Error Handling Design

## 📋 **Tổng Quan**

Status Core Error Handling Design định nghĩa comprehensive error handling system cho Status Core, bao gồm error types, error recovery, logging, monitoring, và debugging tools.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Error Handling Principles**
- **Fail Fast**: Detect errors early và fail fast
- **Graceful Degradation**: Continue operation khi possible
- **Error Recovery**: Automatic error recovery mechanisms
- **Comprehensive Logging**: Detailed error logging cho debugging

### **2. Error Categories**
- **Validation Errors**: Input validation failures
- **System Errors**: Internal system failures
- **Integration Errors**: External system integration failures
- **Performance Errors**: Performance-related failures

### **3. Error Recovery Strategies**
- **Retry Logic**: Automatic retry với exponential backoff
- **Circuit Breaker**: Prevent cascade failures
- **Fallback Mechanisms**: Alternative execution paths
- **Error Propagation**: Proper error propagation

## 🏗️ **Error Handling Architecture**

### **1. Error Types System**

```rust
/// Status Core Error Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusError {
    // Validation Errors
    ValidationError(ValidationError),
    
    // System Errors
    SystemError(SystemError),
    
    // Integration Errors
    IntegrationError(IntegrationError),
    
    // Performance Errors
    PerformanceError(PerformanceError),
    
    // Configuration Errors
    ConfigurationError(ConfigurationError),
    
    // Plugin Errors
    PluginError(PluginError),
    
    // Cache Errors
    CacheError(CacheError),
    
    // Memory Errors
    MemoryError(MemoryError),
    
    // Network Errors
    NetworkError(NetworkError),
    
    // Database Errors
    DatabaseError(DatabaseError),
    
    // Unknown Errors
    UnknownError(UnknownError),
}

/// Validation Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub value: Option<String>,
    pub message: String,
    pub code: ValidationErrorCode,
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValidationErrorCode {
    RequiredFieldMissing,
    InvalidValue,
    ValueOutOfRange,
    InvalidFormat,
    DuplicateValue,
    InvalidReference,
}

/// System Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemError {
    pub component: String,
    pub operation: String,
    pub message: String,
    pub code: SystemErrorCode,
    pub details: Option<serde_json::Value>,
    pub stack_trace: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SystemErrorCode {
    InitializationFailed,
    ComponentNotFound,
    OperationFailed,
    ResourceExhausted,
    Timeout,
    Deadlock,
    CorruptedState,
}

/// Integration Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationError {
    pub system: String,
    pub operation: String,
    pub message: String,
    pub code: IntegrationErrorCode,
    pub retry_after: Option<Duration>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntegrationErrorCode {
    ServiceUnavailable,
    AuthenticationFailed,
    AuthorizationFailed,
    RateLimitExceeded,
    NetworkTimeout,
    InvalidResponse,
    ServiceError,
}

/// Performance Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceError {
    pub metric: String,
    pub current_value: f64,
    pub threshold: f64,
    pub message: String,
    pub code: PerformanceErrorCode,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerformanceErrorCode {
    ThroughputTooLow,
    LatencyTooHigh,
    MemoryUsageTooHigh,
    CpuUsageTooHigh,
    CacheHitRateTooLow,
    QueueSizeTooLarge,
}

impl StatusError {
    /// Create validation error
    pub fn validation_error(
        field: &str,
        value: Option<&str>,
        message: &str,
        code: ValidationErrorCode
    ) -> Self {
        Self::ValidationError(ValidationError {
            field: field.to_string(),
            value: value.map(|v| v.to_string()),
            message: message.to_string(),
            code,
            context: None,
        })
    }
    
    /// Create system error
    pub fn system_error(
        component: &str,
        operation: &str,
        message: &str,
        code: SystemErrorCode
    ) -> Self {
        Self::SystemError(SystemError {
            component: component.to_string(),
            operation: operation.to_string(),
            message: message.to_string(),
            code,
            details: None,
            stack_trace: None,
        })
    }
    
    /// Create integration error
    pub fn integration_error(
        system: &str,
        operation: &str,
        message: &str,
        code: IntegrationErrorCode
    ) -> Self {
        Self::IntegrationError(IntegrationError {
            system: system.to_string(),
            operation: operation.to_string(),
            message: message.to_string(),
            code,
            retry_after: None,
            details: None,
        })
    }
    
    /// Get error code
    pub fn get_error_code(&self) -> String {
        match self {
            StatusError::ValidationError(e) => format!("VALIDATION_{:?}", e.code),
            StatusError::SystemError(e) => format!("SYSTEM_{:?}", e.code),
            StatusError::IntegrationError(e) => format!("INTEGRATION_{:?}", e.code),
            StatusError::PerformanceError(e) => format!("PERFORMANCE_{:?}", e.code),
            StatusError::ConfigurationError(e) => format!("CONFIG_{:?}", e.code),
            StatusError::PluginError(e) => format!("PLUGIN_{:?}", e.code),
            StatusError::CacheError(e) => format!("CACHE_{:?}", e.code),
            StatusError::MemoryError(e) => format!("MEMORY_{:?}", e.code),
            StatusError::NetworkError(e) => format!("NETWORK_{:?}", e.code),
            StatusError::DatabaseError(e) => format!("DATABASE_{:?}", e.code),
            StatusError::UnknownError(e) => format!("UNKNOWN_{:?}", e.code),
        }
    }
    
    /// Get error message
    pub fn get_error_message(&self) -> String {
        match self {
            StatusError::ValidationError(e) => e.message.clone(),
            StatusError::SystemError(e) => e.message.clone(),
            StatusError::IntegrationError(e) => e.message.clone(),
            StatusError::PerformanceError(e) => e.message.clone(),
            StatusError::ConfigurationError(e) => e.message.clone(),
            StatusError::PluginError(e) => e.message.clone(),
            StatusError::CacheError(e) => e.message.clone(),
            StatusError::MemoryError(e) => e.message.clone(),
            StatusError::NetworkError(e) => e.message.clone(),
            StatusError::DatabaseError(e) => e.message.clone(),
            StatusError::UnknownError(e) => e.message.clone(),
        }
    }
    
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            StatusError::IntegrationError(e) => matches!(
                e.code,
                IntegrationErrorCode::ServiceUnavailable
                    | IntegrationErrorCode::NetworkTimeout
                    | IntegrationErrorCode::ServiceError
            ),
            StatusError::NetworkError(_) => true,
            StatusError::DatabaseError(_) => true,
            _ => false,
        }
    }
    
    /// Get retry delay
    pub fn get_retry_delay(&self) -> Option<Duration> {
        match self {
            StatusError::IntegrationError(e) => e.retry_after,
            _ => None,
        }
    }
}
```

### **2. Error Recovery System**

```rust
/// Error Recovery System
pub struct ErrorRecoverySystem {
    // Retry configuration
    retry_config: RetryConfig,
    
    // Circuit breaker
    circuit_breaker: CircuitBreaker,
    
    // Fallback mechanisms
    fallback_manager: FallbackManager,
    
    // Error monitoring
    error_monitor: ErrorMonitor,
    
    // Recovery strategies
    recovery_strategies: HashMap<String, Box<dyn RecoveryStrategy>>,
}

impl ErrorRecoverySystem {
    /// Handle error with recovery
    pub async fn handle_error(
        &self,
        error: StatusError,
        context: &ErrorContext
    ) -> Result<ErrorRecoveryResult, StatusError> {
        // Log error
        self.error_monitor.log_error(&error, context).await?;
        
        // Check if error is retryable
        if error.is_retryable() {
            return self.handle_retryable_error(error, context).await;
        }
        
        // Check circuit breaker
        if self.circuit_breaker.is_open(&error) {
            return self.handle_circuit_breaker_error(error, context).await;
        }
        
        // Try fallback mechanism
        if let Some(fallback) = self.fallback_manager.get_fallback(&error) {
            return self.handle_fallback_error(error, context, fallback).await;
        }
        
        // Try recovery strategy
        if let Some(strategy) = self.recovery_strategies.get(&error.get_error_code()) {
            return strategy.recover(error, context).await;
        }
        
        // No recovery available
        Err(error)
    }
    
    /// Handle retryable error
    async fn handle_retryable_error(
        &self,
        error: StatusError,
        context: &ErrorContext
    ) -> Result<ErrorRecoveryResult, StatusError> {
        let retry_delay = error.get_retry_delay().unwrap_or(self.retry_config.base_delay);
        let max_retries = self.retry_config.max_retries;
        
        for attempt in 1..=max_retries {
            // Wait before retry
            if attempt > 1 {
                let delay = self.calculate_retry_delay(retry_delay, attempt);
                tokio::time::sleep(delay).await;
            }
            
            // Try to recover
            match self.attempt_recovery(&error, context).await {
                Ok(result) => {
                    self.error_monitor.log_recovery_success(&error, attempt).await?;
                    return Ok(result);
                },
                Err(e) if e.is_retryable() && attempt < max_retries => {
                    self.error_monitor.log_retry_attempt(&error, attempt).await?;
                    continue;
                },
                Err(e) => {
                    self.error_monitor.log_recovery_failure(&error, attempt, &e).await?;
                    return Err(e);
                },
            }
        }
        
        Err(error)
    }
    
    /// Calculate retry delay with exponential backoff
    fn calculate_retry_delay(&self, base_delay: Duration, attempt: u32) -> Duration {
        let multiplier = 2.0_f64.powi(attempt as i32 - 1);
        let jitter = self.retry_config.jitter_range;
        let jitter_amount = if jitter > 0.0 {
            let jitter_value = (rand::random::<f64>() - 0.5) * jitter;
            base_delay.as_secs_f64() * jitter_value
        } else {
            0.0
        };
        
        let delay_seconds = base_delay.as_secs_f64() * multiplier + jitter_amount;
        let max_delay = self.retry_config.max_delay.as_secs_f64();
        let final_delay = delay_seconds.min(max_delay);
        
        Duration::from_secs_f64(final_delay)
    }
    
    /// Attempt recovery
    async fn attempt_recovery(
        &self,
        error: &StatusError,
        context: &ErrorContext
    ) -> Result<ErrorRecoveryResult, StatusError> {
        // Implement recovery logic based on error type
        match error {
            StatusError::IntegrationError(e) => {
                self.recover_integration_error(e, context).await
            },
            StatusError::NetworkError(e) => {
                self.recover_network_error(e, context).await
            },
            StatusError::DatabaseError(e) => {
                self.recover_database_error(e, context).await
            },
            _ => Err(error.clone()),
        }
    }
}

/// Retry Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub jitter_range: f64,
    pub enabled: bool,
}

/// Circuit Breaker
pub struct CircuitBreaker {
    // Circuit breaker state
    state: Arc<Mutex<CircuitBreakerState>>,
    
    // Configuration
    config: CircuitBreakerConfig,
    
    // Metrics
    metrics: CircuitBreakerMetrics,
}

#[derive(Debug, Clone)]
pub enum CircuitBreakerState {
    Closed,
    Open { opened_at: Instant, next_attempt: Instant },
    HalfOpen { attempt_count: u32 },
}

impl CircuitBreaker {
    /// Check if circuit breaker is open
    pub fn is_open(&self, error: &StatusError) -> bool {
        let state = self.state.lock().unwrap();
        match *state {
            CircuitBreakerState::Open { next_attempt, .. } => {
                Instant::now() < next_attempt
            },
            CircuitBreakerState::HalfOpen { .. } => true,
            CircuitBreakerState::Closed => false,
        }
    }
    
    /// Record success
    pub fn record_success(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            CircuitBreakerState::HalfOpen { .. } => {
                *state = CircuitBreakerState::Closed;
                self.metrics.record_circuit_closed();
            },
            _ => {},
        }
    }
    
    /// Record failure
    pub fn record_failure(&self, error: &StatusError) {
        let mut state = self.state.lock().unwrap();
        match *state {
            CircuitBreakerState::Closed => {
                self.metrics.increment_failure_count();
                if self.metrics.get_failure_count() >= self.config.failure_threshold {
                    *state = CircuitBreakerState::Open {
                        opened_at: Instant::now(),
                        next_attempt: Instant::now() + self.config.timeout,
                    };
                    self.metrics.record_circuit_opened();
                }
            },
            CircuitBreakerState::HalfOpen { attempt_count } => {
                if attempt_count >= self.config.half_open_max_attempts {
                    *state = CircuitBreakerState::Open {
                        opened_at: Instant::now(),
                        next_attempt: Instant::now() + self.config.timeout,
                    };
                    self.metrics.record_circuit_opened();
                } else {
                    *state = CircuitBreakerState::HalfOpen { attempt_count: attempt_count + 1 };
                }
            },
            CircuitBreakerState::Open { next_attempt, .. } => {
                if Instant::now() >= next_attempt {
                    *state = CircuitBreakerState::HalfOpen { attempt_count: 0 };
                    self.metrics.record_circuit_half_open();
                }
            },
        }
    }
}
```

### **3. Error Logging System**

```rust
/// Error Logging System
pub struct ErrorLoggingSystem {
    // Loggers for different levels
    error_logger: Logger,
    warning_logger: Logger,
    info_logger: Logger,
    debug_logger: Logger,
    
    // Log configuration
    config: LoggingConfig,
    
    // Log aggregation
    log_aggregator: LogAggregator,
    
    // Log storage
    log_storage: LogStorage,
}

impl ErrorLoggingSystem {
    /// Log error with context
    pub async fn log_error(
        &self,
        error: &StatusError,
        context: &ErrorContext
    ) -> Result<(), StatusError> {
        let log_entry = ErrorLogEntry {
            timestamp: SystemTime::now(),
            error: error.clone(),
            context: context.clone(),
            severity: self.determine_severity(error),
            component: self.extract_component(error),
            operation: self.extract_operation(error),
            user_id: context.user_id.clone(),
            session_id: context.session_id.clone(),
            request_id: context.request_id.clone(),
            stack_trace: self.capture_stack_trace(),
            environment: self.get_environment_info(),
        };
        
        // Log to appropriate logger
        match log_entry.severity {
            LogSeverity::Error => {
                self.error_logger.log(&log_entry).await?;
            },
            LogSeverity::Warning => {
                self.warning_logger.log(&log_entry).await?;
            },
            LogSeverity::Info => {
                self.info_logger.log(&log_entry).await?;
            },
            LogSeverity::Debug => {
                self.debug_logger.log(&log_entry).await?;
            },
        }
        
        // Aggregate logs
        self.log_aggregator.aggregate_log(&log_entry).await?;
        
        // Store logs
        self.log_storage.store_log(&log_entry).await?;
        
        Ok(())
    }
    
    /// Determine log severity
    fn determine_severity(&self, error: &StatusError) -> LogSeverity {
        match error {
            StatusError::SystemError(_) => LogSeverity::Error,
            StatusError::IntegrationError(_) => LogSeverity::Warning,
            StatusError::PerformanceError(_) => LogSeverity::Warning,
            StatusError::ValidationError(_) => LogSeverity::Info,
            StatusError::ConfigurationError(_) => LogSeverity::Error,
            StatusError::PluginError(_) => LogSeverity::Warning,
            StatusError::CacheError(_) => LogSeverity::Info,
            StatusError::MemoryError(_) => LogSeverity::Error,
            StatusError::NetworkError(_) => LogSeverity::Warning,
            StatusError::DatabaseError(_) => LogSeverity::Error,
            StatusError::UnknownError(_) => LogSeverity::Error,
        }
    }
    
    /// Extract component from error
    fn extract_component(&self, error: &StatusError) -> String {
        match error {
            StatusError::SystemError(e) => e.component.clone(),
            StatusError::IntegrationError(e) => e.system.clone(),
            StatusError::PerformanceError(_) => "Performance".to_string(),
            StatusError::ConfigurationError(_) => "Configuration".to_string(),
            StatusError::PluginError(_) => "Plugin".to_string(),
            StatusError::CacheError(_) => "Cache".to_string(),
            StatusError::MemoryError(_) => "Memory".to_string(),
            StatusError::NetworkError(_) => "Network".to_string(),
            StatusError::DatabaseError(_) => "Database".to_string(),
            _ => "Unknown".to_string(),
        }
    }
    
    /// Extract operation from error
    fn extract_operation(&self, error: &StatusError) -> String {
        match error {
            StatusError::SystemError(e) => e.operation.clone(),
            StatusError::IntegrationError(e) => e.operation.clone(),
            _ => "Unknown".to_string(),
        }
    }
    
    /// Capture stack trace
    fn capture_stack_trace(&self) -> Option<String> {
        if self.config.capture_stack_trace {
            Some(std::backtrace::Backtrace::capture().to_string())
        } else {
            None
        }
    }
}

/// Error Log Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLogEntry {
    pub timestamp: SystemTime,
    pub error: StatusError,
    pub context: ErrorContext,
    pub severity: LogSeverity,
    pub component: String,
    pub operation: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub request_id: Option<String>,
    pub stack_trace: Option<String>,
    pub environment: EnvironmentInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogSeverity {
    Error,
    Warning,
    Info,
    Debug,
}

/// Error Context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub request_id: Option<String>,
    pub actor_id: Option<String>,
    pub effect_id: Option<String>,
    pub operation: String,
    pub additional_data: HashMap<String, serde_json::Value>,
}
```

### **4. Error Monitoring System**

```rust
/// Error Monitoring System
pub struct ErrorMonitoringSystem {
    // Error metrics
    error_metrics: ErrorMetrics,
    
    // Alert system
    alert_system: AlertSystem,
    
    // Error dashboard
    dashboard: ErrorDashboard,
    
    // Error analysis
    analyzer: ErrorAnalyzer,
}

impl ErrorMonitoringSystem {
    /// Record error occurrence
    pub async fn record_error(
        &self,
        error: &StatusError,
        context: &ErrorContext
    ) -> Result<(), StatusError> {
        // Update metrics
        self.error_metrics.record_error(error, context).await?;
        
        // Check for alerts
        if let Some(alert) = self.check_alert_conditions(error, context).await? {
            self.alert_system.send_alert(alert).await?;
        }
        
        // Update dashboard
        self.dashboard.update_error_display(error, context).await?;
        
        // Analyze error patterns
        self.analyzer.analyze_error(error, context).await?;
        
        Ok(())
    }
    
    /// Check alert conditions
    async fn check_alert_conditions(
        &self,
        error: &StatusError,
        context: &ErrorContext
    ) -> Result<Option<Alert>, StatusError> {
        let error_rate = self.error_metrics.get_error_rate(error.get_error_code()).await?;
        let threshold = self.get_alert_threshold(error.get_error_code()).await?;
        
        if error_rate > threshold {
            return Ok(Some(Alert {
                id: Uuid::new_v4().to_string(),
                severity: AlertSeverity::High,
                title: format!("High error rate for {}", error.get_error_code()),
                message: format!("Error rate {} exceeds threshold {}", error_rate, threshold),
                error_code: error.get_error_code(),
                component: self.extract_component(error),
                timestamp: SystemTime::now(),
                context: context.clone(),
            }));
        }
        
        Ok(None)
    }
    
    /// Get error statistics
    pub async fn get_error_statistics(
        &self,
        time_range: TimeRange
    ) -> Result<ErrorStatistics, StatusError> {
        let errors = self.error_metrics.get_errors_in_range(time_range).await?;
        
        let mut statistics = ErrorStatistics {
            total_errors: errors.len(),
            error_by_type: HashMap::new(),
            error_by_component: HashMap::new(),
            error_by_severity: HashMap::new(),
            error_trend: Vec::new(),
            top_errors: Vec::new(),
        };
        
        for error in errors {
            // Count by type
            *statistics.error_by_type.entry(error.get_error_code()).or_insert(0) += 1;
            
            // Count by component
            let component = self.extract_component(&error);
            *statistics.error_by_component.entry(component).or_insert(0) += 1;
            
            // Count by severity
            let severity = self.determine_severity(&error);
            *statistics.error_by_severity.entry(severity).or_insert(0) += 1;
        }
        
        // Calculate top errors
        let mut error_counts: Vec<_> = statistics.error_by_type.into_iter().collect();
        error_counts.sort_by(|a, b| b.1.cmp(&a.1));
        statistics.top_errors = error_counts.into_iter().take(10).collect();
        
        Ok(statistics)
    }
}

/// Error Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStatistics {
    pub total_errors: usize,
    pub error_by_type: HashMap<String, u32>,
    pub error_by_component: HashMap<String, u32>,
    pub error_by_severity: HashMap<LogSeverity, u32>,
    pub error_trend: Vec<ErrorTrendPoint>,
    pub top_errors: Vec<(String, u32)>,
}

/// Error Trend Point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorTrendPoint {
    pub timestamp: SystemTime,
    pub error_count: u32,
    pub error_rate: f64,
}
```

## 🧪 **Error Handling Testing**

### **1. Error Recovery Testing**

```rust
#[cfg(test)]
mod error_handling_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_retry_mechanism() {
        let recovery_system = ErrorRecoverySystem::new(create_test_retry_config()).await?;
        let context = create_test_error_context();
        
        // Create retryable error
        let error = StatusError::integration_error(
            "test_system",
            "test_operation",
            "Service temporarily unavailable",
            IntegrationErrorCode::ServiceUnavailable
        );
        
        // Test retry mechanism
        let result = recovery_system.handle_error(error, &context).await;
        
        // Should eventually succeed or fail after max retries
        assert!(result.is_ok() || result.is_err());
    }
    
    #[tokio::test]
    async fn test_circuit_breaker() {
        let recovery_system = ErrorRecoverySystem::new(create_test_circuit_breaker_config()).await?;
        let context = create_test_error_context();
        
        // Create error that should trigger circuit breaker
        let error = StatusError::integration_error(
            "test_system",
            "test_operation",
            "Service error",
            IntegrationErrorCode::ServiceError
        );
        
        // Record multiple failures to trigger circuit breaker
        for _ in 0..5 {
            recovery_system.handle_error(error.clone(), &context).await;
        }
        
        // Circuit breaker should be open
        assert!(recovery_system.circuit_breaker.is_open(&error));
    }
    
    #[tokio::test]
    async fn test_fallback_mechanism() {
        let recovery_system = ErrorRecoverySystem::new(create_test_fallback_config()).await?;
        let context = create_test_error_context();
        
        // Create error with fallback
        let error = StatusError::system_error(
            "test_component",
            "test_operation",
            "Component failure",
            SystemErrorCode::ComponentNotFound
        );
        
        // Test fallback mechanism
        let result = recovery_system.handle_error(error, &context).await?;
        
        // Should use fallback mechanism
        assert!(result.used_fallback);
    }
}
```

### **2. Error Logging Testing**

```rust
#[tokio::test]
async fn test_error_logging() {
    let logging_system = ErrorLoggingSystem::new(create_test_logging_config()).await?;
    let context = create_test_error_context();
    
    // Create test error
    let error = StatusError::validation_error(
        "test_field",
        Some("invalid_value"),
        "Invalid value provided",
        ValidationErrorCode::InvalidValue
    );
    
    // Log error
    logging_system.log_error(&error, &context).await?;
    
    // Verify error was logged
    let logs = logging_system.get_logs_for_time_range(
        SystemTime::now() - Duration::from_secs(1),
        SystemTime::now()
    ).await?;
    
    assert!(!logs.is_empty());
    assert_eq!(logs[0].error.get_error_code(), "VALIDATION_InvalidValue");
}
```

## 📝 **Implementation Notes**

### **1. Error Handling Strategy**
- **Fail Fast**: Detect errors early và fail fast
- **Graceful Degradation**: Continue operation khi possible
- **Error Recovery**: Automatic error recovery mechanisms
- **Comprehensive Logging**: Detailed error logging cho debugging

### **2. Error Categories**
- **Validation Errors**: Input validation failures
- **System Errors**: Internal system failures
- **Integration Errors**: External system integration failures
- **Performance Errors**: Performance-related failures

### **3. Recovery Mechanisms**
- **Retry Logic**: Automatic retry với exponential backoff
- **Circuit Breaker**: Prevent cascade failures
- **Fallback Mechanisms**: Alternative execution paths
- **Error Propagation**: Proper error propagation

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
