# Error Handling & Logging Design

## 📋 **Tổng Quan**

Tài liệu này mô tả chiến lược error handling và logging cho Element Core, bao gồm error recovery, logging strategy, debugging tools, và error reporting.

## 🚨 **Error Handling Strategy**

### **1. Error Categories**

#### **Calculation Errors**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ElementCalculationError {
    // Invalid input parameters
    InvalidInput {
        parameter: String,
        value: String,
        expected: String,
    },
    
    // Division by zero
    DivisionByZero {
        operation: String,
        context: String,
    },
    
    // Overflow/underflow
    NumericOverflow {
        operation: String,
        value: f64,
        max_value: f64,
    },
    
    // Invalid element type
    InvalidElementType {
        element_type: String,
        available_types: Vec<String>,
    },
    
    // Missing required stats
    MissingRequiredStats {
        element_type: String,
        missing_stats: Vec<String>,
    },
}
```

#### **Configuration Errors**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ElementConfigError {
    // Invalid configuration file
    InvalidConfigFile {
        file_path: String,
        error: String,
    },
    
    // Missing required configuration
    MissingRequiredConfig {
        section: String,
        key: String,
    },
    
    // Invalid configuration value
    InvalidConfigValue {
        section: String,
        key: String,
        value: String,
        expected: String,
    },
    
    // Configuration validation failed
    ValidationFailed {
        section: String,
        errors: Vec<String>,
    },
}
```

#### **System Errors**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ElementSystemError {
    // Resource exhaustion
    ResourceExhausted {
        resource_type: String,
        requested: u64,
        available: u64,
    },
    
    // Concurrency errors
    ConcurrencyError {
        operation: String,
        context: String,
    },
    
    // Cache errors
    CacheError {
        operation: String,
        key: String,
        error: String,
    },
    
    // Integration errors
    IntegrationError {
        system: String,
        operation: String,
        error: String,
    },
}
```

### **2. Error Recovery Strategies**

#### **Graceful Degradation**
```rust
// Error recovery with graceful degradation
pub struct ElementCoreErrorHandler {
    // Fallback configurations
    fallback_configs: HashMap<String, ElementConfig>,
    
    // Error recovery strategies
    recovery_strategies: HashMap<ElementCalculationError, RecoveryStrategy>,
    
    // Circuit breaker for failing operations
    circuit_breaker: CircuitBreaker,
}

impl ElementCoreErrorHandler {
    // Handle calculation errors with fallback
    fn handle_calculation_error(&self, error: ElementCalculationError, context: &CalculationContext) -> Result<f64, ElementCoreError> {
        match error {
            ElementCalculationError::DivisionByZero { operation, context } => {
                // Use fallback value
                self.log_warning(&format!("Division by zero in {}: {}, using fallback", operation, context));
                Ok(self.get_fallback_value(&operation))
            },
            
            ElementCalculationError::NumericOverflow { operation, value, max_value } => {
                // Clamp to maximum value
                self.log_warning(&format!("Numeric overflow in {}: {} > {}, clamping to {}", operation, value, max_value, max_value));
                Ok(max_value)
            },
            
            ElementCalculationError::InvalidElementType { element_type, available_types } => {
                // Use default element type
                self.log_error(&format!("Invalid element type: {}, using default. Available: {:?}", element_type, available_types));
                Ok(self.calculate_with_default_element(context))
            },
            
            _ => Err(ElementCoreError::CalculationError(error))
        }
    }
}
```

#### **Circuit Breaker Pattern**
```rust
// Circuit breaker for failing operations
pub struct CircuitBreaker {
    // Failure threshold
    failure_threshold: u32,
    
    // Recovery timeout
    recovery_timeout: Duration,
    
    // Current state
    state: CircuitBreakerState,
    
    // Failure count
    failure_count: AtomicU32,
    
    // Last failure time
    last_failure_time: AtomicU64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    Closed,    // Normal operation
    Open,      // Failing, reject requests
    HalfOpen,  // Testing recovery
}

impl CircuitBreaker {
    // Execute operation with circuit breaker protection
    fn execute<F, R>(&self, operation: F) -> Result<R, ElementCoreError>
    where
        F: FnOnce() -> Result<R, ElementCoreError>
    {
        match self.state {
            CircuitBreakerState::Closed => {
                match operation() {
                    Ok(result) => {
                        self.reset_failure_count();
                        Ok(result)
                    },
                    Err(error) => {
                        self.record_failure();
                        Err(error)
                    }
                }
            },
            
            CircuitBreakerState::Open => {
                if self.should_attempt_reset() {
                    self.state = CircuitBreakerState::HalfOpen;
                    self.execute(operation)
                } else {
                    Err(ElementCoreError::SystemError(ElementSystemError::ResourceExhausted {
                        resource_type: "CircuitBreaker".to_string(),
                        requested: 1,
                        available: 0,
                    }))
                }
            },
            
            CircuitBreakerState::HalfOpen => {
                match operation() {
                    Ok(result) => {
                        self.state = CircuitBreakerState::Closed;
                        self.reset_failure_count();
                        Ok(result)
                    },
                    Err(error) => {
                        self.state = CircuitBreakerState::Open;
                        self.record_failure();
                        Err(error)
                    }
                }
            }
        }
    }
}
```

### **3. Error Context & Tracing**

#### **Error Context**
```rust
// Rich error context for debugging
pub struct ErrorContext {
    // Operation being performed
    operation: String,
    
    // Input parameters
    input_params: HashMap<String, String>,
    
    // System state
    system_state: SystemState,
    
    // Call stack
    call_stack: Vec<String>,
    
    // Timestamp
    timestamp: DateTime<Utc>,
    
    // Request ID for tracing
    request_id: String,
}

impl ErrorContext {
    // Create error context for operation
    fn new(operation: String) -> Self {
        Self {
            operation,
            input_params: HashMap::new(),
            system_state: SystemState::default(),
            call_stack: Vec::new(),
            timestamp: Utc::now(),
            request_id: generate_request_id(),
        }
    }
    
    // Add parameter to context
    fn add_param(&mut self, key: String, value: String) {
        self.input_params.insert(key, value);
    }
    
    // Add call stack entry
    fn add_call_stack(&mut self, entry: String) {
        self.call_stack.push(entry);
    }
}
```

#### **Error Tracing**
```rust
// Error tracing and correlation
pub struct ErrorTracer {
    // Error correlation ID
    correlation_id: String,
    
    // Error chain
    error_chain: Vec<ElementCoreError>,
    
    // Context history
    context_history: Vec<ErrorContext>,
    
    // Performance metrics
    performance_metrics: PerformanceMetrics,
}

impl ErrorTracer {
    // Trace error through system
    fn trace_error(&mut self, error: ElementCoreError, context: ErrorContext) {
        self.error_chain.push(error);
        self.context_history.push(context);
        
        // Log error with full context
        self.log_error_with_context();
        
        // Update performance metrics
        self.update_error_metrics();
    }
    
    // Get error correlation ID
    fn get_correlation_id(&self) -> &str {
        &self.correlation_id
    }
}
```

## 📝 **Logging Strategy**

### **1. Log Levels & Categories**

#### **Log Levels**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Trace,    // Detailed debugging information
    Debug,    // General debugging information
    Info,     // General information
    Warn,     // Warning messages
    Error,    // Error messages
    Fatal,    // Fatal errors
}

// Log categories for different components
#[derive(Debug, Clone, PartialEq)]
pub enum LogCategory {
    // Core system logs
    ElementCore,
    StatCalculation,
    CacheManagement,
    
    // Integration logs
    CombatIntegration,
    ResourceManagerIntegration,
    ActorCoreIntegration,
    
    // Performance logs
    Performance,
    Memory,
    Concurrency,
    
    // Error logs
    Error,
    Warning,
    Recovery,
}
```

#### **Structured Logging**
```rust
// Structured logging with context
pub struct ElementCoreLogger {
    // Log level configuration
    log_levels: HashMap<LogCategory, LogLevel>,
    
    // Log formatters
    formatters: HashMap<LogCategory, Box<dyn LogFormatter>>,
    
    // Log outputs
    outputs: Vec<Box<dyn LogOutput>>,
    
    // Log filtering
    filters: Vec<Box<dyn LogFilter>>,
}

// Structured log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub category: LogCategory,
    pub message: String,
    pub context: HashMap<String, String>,
    pub correlation_id: Option<String>,
    pub request_id: Option<String>,
}

impl ElementCoreLogger {
    // Log with structured context
    fn log(&self, level: LogLevel, category: LogCategory, message: &str, context: HashMap<String, String>) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level,
            category,
            message: message.to_string(),
            context,
            correlation_id: self.get_correlation_id(),
            request_id: self.get_request_id(),
        };
        
        // Apply filters
        if self.should_log(&entry) {
            // Format and output
            for output in &self.outputs {
                output.write(&entry);
            }
        }
    }
}
```

### **2. Performance Logging**

#### **Performance Metrics Logging**
```rust
// Performance metrics logging
pub struct PerformanceLogger {
    // Metrics collection
    metrics_collector: MetricsCollector,
    
    // Performance thresholds
    thresholds: PerformanceThresholds,
    
    // Alerting
    alert_manager: AlertManager,
}

impl PerformanceLogger {
    // Log performance metrics
    fn log_performance_metrics(&self, operation: &str, duration: Duration, context: &HashMap<String, String>) {
        let mut log_context = context.clone();
        log_context.insert("operation".to_string(), operation.to_string());
        log_context.insert("duration_ms".to_string(), duration.as_millis().to_string());
        
        // Check against thresholds
        if duration > self.thresholds.get_threshold(operation) {
            self.logger.log(LogLevel::Warn, LogCategory::Performance, 
                &format!("Slow operation: {} took {}ms", operation, duration.as_millis()), 
                log_context);
            
            // Trigger alert if needed
            self.alert_manager.check_performance_alert(operation, duration);
        } else {
            self.logger.log(LogLevel::Debug, LogCategory::Performance, 
                &format!("Operation completed: {} in {}ms", operation, duration.as_millis()), 
                log_context);
        }
    }
}
```

#### **Memory Usage Logging**
```rust
// Memory usage logging
pub struct MemoryLogger {
    // Memory tracking
    memory_tracker: MemoryTracker,
    
    // Memory thresholds
    memory_thresholds: MemoryThresholds,
    
    // Garbage collection logging
    gc_logger: GarbageCollectionLogger,
}

impl MemoryLogger {
    // Log memory usage
    fn log_memory_usage(&self, context: &HashMap<String, String>) {
        let memory_usage = self.memory_tracker.get_current_usage();
        let mut log_context = context.clone();
        log_context.insert("memory_usage_mb".to_string(), memory_usage.total_mb.to_string());
        log_context.insert("memory_usage_percent".to_string(), memory_usage.usage_percent.to_string());
        
        if memory_usage.usage_percent > self.memory_thresholds.warning_threshold {
            self.logger.log(LogLevel::Warn, LogCategory::Memory, 
                &format!("High memory usage: {}%", memory_usage.usage_percent), 
                log_context);
        }
    }
}
```

### **3. Debug Logging**

#### **Debug Information Logging**
```rust
// Debug information logging
pub struct DebugLogger {
    // Debug configuration
    debug_config: DebugConfig,
    
    // Debug outputs
    debug_outputs: Vec<Box<dyn DebugOutput>>,
    
    // Debug filters
    debug_filters: Vec<Box<dyn DebugFilter>>,
}

impl DebugLogger {
    // Log debug information
    fn log_debug(&self, component: &str, message: &str, data: &DebugData) {
        if self.debug_config.is_enabled(component) {
            let debug_entry = DebugEntry {
                timestamp: Utc::now(),
                component: component.to_string(),
                message: message.to_string(),
                data: data.clone(),
                call_stack: self.get_call_stack(),
            };
            
            // Apply debug filters
            if self.should_log_debug(&debug_entry) {
                for output in &self.debug_outputs {
                    output.write_debug(&debug_entry);
                }
            }
        }
    }
}
```

## 🔧 **Debugging Tools**

### **1. Stat Calculation Debugger**

#### **Calculation Debugger**
```rust
// Debug tool for stat calculations
pub struct StatCalculationDebugger {
    // Calculation history
    calculation_history: Vec<CalculationStep>,
    
    // Debug configuration
    debug_config: CalculationDebugConfig,
    
    // Step-by-step logging
    step_logger: StepLogger,
}

impl StatCalculationDebugger {
    // Debug stat calculation step by step
    fn debug_calculation(&mut self, calculation: &StatCalculation) -> Result<f64, ElementCoreError> {
        let mut result = 0.0;
        
        for step in &calculation.steps {
            // Log step details
            self.step_logger.log_step(step);
            
            // Execute step
            match self.execute_step(step) {
                Ok(step_result) => {
                    result = step_result;
                    self.calculation_history.push(CalculationStep {
                        step: step.clone(),
                        result: step_result,
                        success: true,
                        error: None,
                    });
                },
                Err(error) => {
                    self.calculation_history.push(CalculationStep {
                        step: step.clone(),
                        result: 0.0,
                        success: false,
                        error: Some(error.clone()),
                    });
                    
                    // Log error and continue or abort
                    if self.debug_config.abort_on_error {
                        return Err(error);
                    }
                }
            }
        }
        
        Ok(result)
    }
}
```

#### **Calculation Visualizer**
```rust
// Visual representation of calculations
pub struct CalculationVisualizer {
    // Calculation tree
    calculation_tree: CalculationTree,
    
    // Visualization options
    viz_options: VisualizationOptions,
    
    // Output formats
    output_formats: Vec<Box<dyn CalculationOutput>>,
}

impl CalculationVisualizer {
    // Generate calculation visualization
    fn visualize_calculation(&self, calculation: &StatCalculation) -> String {
        let tree = self.build_calculation_tree(calculation);
        
        match self.viz_options.format {
            VisualizationFormat::Text => self.generate_text_visualization(&tree),
            VisualizationFormat::Json => self.generate_json_visualization(&tree),
            VisualizationFormat::Graphviz => self.generate_graphviz_visualization(&tree),
        }
    }
}
```

### **2. Performance Profiler**

#### **Performance Profiler**
```rust
// Performance profiler for Element Core
pub struct ElementCoreProfiler {
    // Profiling configuration
    profiler_config: ProfilerConfig,
    
    // Performance data
    performance_data: PerformanceData,
    
    // Profiling outputs
    profiling_outputs: Vec<Box<dyn ProfilingOutput>>,
}

impl ElementCoreProfiler {
    // Profile operation
    fn profile_operation<F, R>(&mut self, operation_name: &str, operation: F) -> R
    where
        F: FnOnce() -> R
    {
        let start_time = Instant::now();
        let start_memory = self.get_memory_usage();
        
        let result = operation();
        
        let end_time = Instant::now();
        let end_memory = self.get_memory_usage();
        
        // Record performance data
        let performance_entry = PerformanceEntry {
            operation: operation_name.to_string(),
            duration: end_time - start_time,
            memory_delta: end_memory - start_memory,
            timestamp: Utc::now(),
        };
        
        self.performance_data.add_entry(performance_entry);
        
        result
    }
}
```

## 📊 **Error Reporting**

### **1. User-Friendly Error Messages**

#### **Error Message Generator**
```rust
// Generate user-friendly error messages
pub struct ErrorMessageGenerator {
    // Error message templates
    message_templates: HashMap<ElementCoreError, String>,
    
    // Localization support
    localization: LocalizationManager,
    
    // Error context formatter
    context_formatter: ErrorContextFormatter,
}

impl ErrorMessageGenerator {
    // Generate user-friendly error message
    fn generate_error_message(&self, error: &ElementCoreError, context: &ErrorContext) -> String {
        let template = self.message_templates.get(error).unwrap_or(&"Unknown error".to_string());
        
        // Format template with context
        self.format_error_template(template, context)
    }
    
    // Format error template
    fn format_error_template(&self, template: &str, context: &ErrorContext) -> String {
        let mut message = template.to_string();
        
        // Replace placeholders with context values
        for (key, value) in &context.input_params {
            message = message.replace(&format!("{{{}}}", key), value);
        }
        
        message
    }
}
```

### **2. Error Reporting System**

#### **Error Reporting**
```rust
// Error reporting system
pub struct ErrorReportingSystem {
    // Error reporters
    error_reporters: Vec<Box<dyn ErrorReporter>>,
    
    // Error aggregation
    error_aggregator: ErrorAggregator,
    
    // Error analytics
    error_analytics: ErrorAnalytics,
}

impl ErrorReportingSystem {
    // Report error
    fn report_error(&self, error: &ElementCoreError, context: &ErrorContext) {
        // Aggregate error data
        self.error_aggregator.aggregate_error(error, context);
        
        // Generate error report
        let error_report = self.generate_error_report(error, context);
        
        // Send to reporters
        for reporter in &self.error_reporters {
            reporter.report_error(&error_report);
        }
        
        // Update analytics
        self.error_analytics.update_error_statistics(error, context);
    }
}
```

## 🧪 **Testing & Validation**

### **1. Error Handling Tests**

#### **Error Recovery Tests**
```rust
#[cfg(test)]
mod error_handling_tests {
    use super::*;
    
    #[test]
    fn test_division_by_zero_recovery() {
        let error_handler = ElementCoreErrorHandler::new();
        let context = CalculationContext::new();
        
        let error = ElementCalculationError::DivisionByZero {
            operation: "crit_rate_calculation".to_string(),
            context: "attacker_crit_rate / defender_resist_crit_rate".to_string(),
        };
        
        let result = error_handler.handle_calculation_error(error, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0); // Fallback value
    }
    
    #[test]
    fn test_circuit_breaker_functionality() {
        let circuit_breaker = CircuitBreaker::new(3, Duration::from_secs(5));
        
        // Simulate failures
        for _ in 0..3 {
            let result = circuit_breaker.execute(|| Err(ElementCoreError::SystemError(ElementSystemError::ResourceExhausted {
                resource_type: "test".to_string(),
                requested: 1,
                available: 0,
            })));
            assert!(result.is_err());
        }
        
        // Circuit should be open now
        let result = circuit_breaker.execute(|| Ok(42));
        assert!(result.is_err());
    }
}
```

### **2. Logging Tests**

#### **Logging Tests**
```rust
#[cfg(test)]
mod logging_tests {
    use super::*;
    
    #[test]
    fn test_structured_logging() {
        let logger = ElementCoreLogger::new();
        let mut context = HashMap::new();
        context.insert("actor_id".to_string(), "actor_123".to_string());
        context.insert("element_type".to_string(), "fire".to_string());
        
        logger.log(LogLevel::Info, LogCategory::StatCalculation, 
            "Calculating fire stats for actor", context);
        
        // Verify log was written
        assert!(logger.has_log_entry("Calculating fire stats for actor"));
    }
    
    #[test]
    fn test_performance_logging() {
        let perf_logger = PerformanceLogger::new();
        let mut context = HashMap::new();
        context.insert("operation".to_string(), "stat_calculation".to_string());
        
        perf_logger.log_performance_metrics("stat_calculation", Duration::from_millis(150), &context);
        
        // Verify performance log was written
        assert!(perf_logger.has_performance_log("stat_calculation"));
    }
}
```

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
