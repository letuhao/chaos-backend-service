# Status Core API Design

## 📋 **Tổng Quan**

Status Core API Design định nghĩa các API interfaces cho Status Core system, bao gồm public API cho external systems, internal API cho internal components, event system, và comprehensive error handling.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. API Design Principles**
- **RESTful Design**: RESTful API patterns
- **Type Safety**: Strong typing cho all API endpoints
- **Async Operations**: Non-blocking operations
- **Error Handling**: Comprehensive error handling

### **2. API Categories**
- **Public API**: API cho external systems
- **Internal API**: API cho internal components
- **Event API**: Event-driven API
- **Admin API**: Administrative API

### **3. API Documentation**
- **OpenAPI Specification**: OpenAPI 3.0 specification
- **Code Examples**: Comprehensive code examples
- **Error Codes**: Detailed error codes và messages
- **Rate Limiting**: Rate limiting documentation

## 🏗️ **API Architecture**

### **1. Public API**

```rust
/// Status Core Public API
pub struct StatusCorePublicAPI {
    // Core engine
    engine: Arc<StatusCoreEngine>,
    
    // API configuration
    config: StatusCoreAPIConfig,
    
    // Rate limiting
    rate_limiter: RateLimiter,
    
    // Authentication
    auth_service: AuthService,
    
    // Logging
    logger: Logger,
}

impl StatusCorePublicAPI {
    /// Apply status effect to actor
    pub async fn apply_status_effect(
        &self,
        request: ApplyStatusEffectRequest
    ) -> Result<ApplyStatusEffectResponse, StatusAPIError> {
        // Validate request
        self.validate_apply_status_effect_request(&request)?;
        
        // Check rate limiting
        self.rate_limiter.check_rate_limit(&request.actor_id).await?;
        
        // Authenticate request
        self.auth_service.authenticate_request(&request).await?;
        
        // Apply status effect
        let result = self.engine.apply_status_effect(
            &request.actor_id,
            request.status_effect,
            &request.context
        ).await?;
        
        // Log operation
        self.logger.log_status_effect_applied(&request.actor_id, &result).await?;
        
        Ok(ApplyStatusEffectResponse {
            success: result.success,
            effect_id: result.effect_id,
            magnitude: result.magnitude,
            duration: result.duration,
            applied_at: result.applied_at,
            expires_at: result.expires_at,
            error_message: if result.success { None } else { Some(result.reason.to_string()) },
        })
    }
    
    /// Get actor status effects
    pub async fn get_actor_status_effects(
        &self,
        request: GetActorStatusEffectsRequest
    ) -> Result<GetActorStatusEffectsResponse, StatusAPIError> {
        // Validate request
        self.validate_get_actor_status_effects_request(&request)?;
        
        // Check rate limiting
        self.rate_limiter.check_rate_limit(&request.actor_id).await?;
        
        // Authenticate request
        self.auth_service.authenticate_request(&request).await?;
        
        // Get status effects
        let effects = self.engine.get_actor_status_effects(&request.actor_id).await?;
        
        // Log operation
        self.logger.log_status_effects_retrieved(&request.actor_id, effects.len()).await?;
        
        Ok(GetActorStatusEffectsResponse {
            actor_id: request.actor_id,
            effects: effects.into_iter().map(|e| StatusEffectResponse::from(e)).collect(),
            total_count: effects.len(),
        })
    }
    
    /// Remove status effect from actor
    pub async fn remove_status_effect(
        &self,
        request: RemoveStatusEffectRequest
    ) -> Result<RemoveStatusEffectResponse, StatusAPIError> {
        // Validate request
        self.validate_remove_status_effect_request(&request)?;
        
        // Check rate limiting
        self.rate_limiter.check_rate_limit(&request.actor_id).await?;
        
        // Authenticate request
        self.auth_service.authenticate_request(&request).await?;
        
        // Remove status effect
        let result = self.engine.remove_status_effect(
            &request.actor_id,
            &request.effect_id
        ).await?;
        
        // Log operation
        self.logger.log_status_effect_removed(&request.actor_id, &request.effect_id).await?;
        
        Ok(RemoveStatusEffectResponse {
            success: result.success,
            effect_id: result.effect_id,
            removed_at: result.removed_at,
            error_message: if result.success { None } else { Some(result.reason.to_string()) },
        })
    }
    
    /// Process actor status effects
    pub async fn process_actor_status_effects(
        &self,
        request: ProcessActorStatusEffectsRequest
    ) -> Result<ProcessActorStatusEffectsResponse, StatusAPIError> {
        // Validate request
        self.validate_process_actor_status_effects_request(&request)?;
        
        // Check rate limiting
        self.rate_limiter.check_rate_limit(&request.actor_id).await?;
        
        // Authenticate request
        self.auth_service.authenticate_request(&request).await?;
        
        // Process status effects
        let results = self.engine.process_actor_status_effects(
            &request.actor_id,
            &request.context
        ).await?;
        
        // Log operation
        self.logger.log_status_effects_processed(&request.actor_id, results.len()).await?;
        
        Ok(ProcessActorStatusEffectsResponse {
            actor_id: request.actor_id,
            results: results.into_iter().map(|r| StatusEffectResultResponse::from(r)).collect(),
            total_count: results.len(),
        })
    }
}

/// Apply Status Effect Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyStatusEffectRequest {
    pub actor_id: String,
    pub status_effect: StatusEffect,
    pub context: StatusContext,
    pub request_id: Option<String>,
    pub timestamp: Option<SystemTime>,
}

/// Apply Status Effect Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyStatusEffectResponse {
    pub success: bool,
    pub effect_id: String,
    pub magnitude: f64,
    pub duration: Duration,
    pub applied_at: Option<SystemTime>,
    pub expires_at: Option<SystemTime>,
    pub error_message: Option<String>,
}

/// Get Actor Status Effects Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActorStatusEffectsRequest {
    pub actor_id: String,
    pub include_expired: Option<bool>,
    pub category_filter: Option<StatusCategory>,
    pub effect_type_filter: Option<StatusEffectType>,
    pub request_id: Option<String>,
    pub timestamp: Option<SystemTime>,
}

/// Get Actor Status Effects Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActorStatusEffectsResponse {
    pub actor_id: String,
    pub effects: Vec<StatusEffectResponse>,
    pub total_count: usize,
}

/// Status Effect Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectResponse {
    pub effect_id: String,
    pub effect_name: String,
    pub effect_name_vi: String,
    pub category: StatusCategory,
    pub effect_type: StatusEffectType,
    pub magnitude: f64,
    pub duration: Duration,
    pub target: StatusTarget,
    pub source: StatusSource,
    pub applied_at: SystemTime,
    pub expires_at: SystemTime,
    pub is_active: bool,
    pub properties: HashMap<String, serde_json::Value>,
}
```

### **2. Internal API**

```rust
/// Status Core Internal API
pub struct StatusCoreInternalAPI {
    // Core engine
    engine: Arc<StatusCoreEngine>,
    
    // Internal configuration
    config: StatusCoreInternalConfig,
    
    // Internal logging
    logger: InternalLogger,
}

impl StatusCoreInternalAPI {
    /// Initialize status effect manager
    pub async fn initialize_status_effect_manager(
        &self,
        config: StatusEffectManagerConfig
    ) -> Result<(), StatusAPIError> {
        self.engine.status_effect_manager.initialize(config).await?;
        self.logger.log_component_initialized("status_effect_manager").await?;
        Ok(())
    }
    
    /// Initialize immunity manager
    pub async fn initialize_immunity_manager(
        &self,
        config: ImmunityManagerConfig
    ) -> Result<(), StatusAPIError> {
        self.engine.immunity_manager.initialize(config).await?;
        self.logger.log_component_initialized("immunity_manager").await?;
        Ok(())
    }
    
    /// Load plugin
    pub async fn load_plugin(
        &self,
        plugin_config: StatusPluginConfig
    ) -> Result<(), StatusAPIError> {
        self.engine.plugin_registry.register_plugin(
            plugin_config.plugin,
            plugin_config.config
        ).await?;
        self.logger.log_plugin_loaded(&plugin_config.plugin_id).await?;
        Ok(())
    }
    
    /// Unload plugin
    pub async fn unload_plugin(
        &self,
        plugin_id: &str
    ) -> Result<(), StatusAPIError> {
        self.engine.plugin_registry.unload_plugin(plugin_id).await?;
        self.logger.log_plugin_unloaded(plugin_id).await?;
        Ok(())
    }
    
    /// Reload configuration
    pub async fn reload_configuration(
        &self
    ) -> Result<(), StatusAPIError> {
        self.engine.configuration_manager.hot_reload_configuration().await?;
        self.logger.log_configuration_reloaded().await?;
        Ok(())
    }
    
    /// Get system status
    pub async fn get_system_status(
        &self
    ) -> Result<SystemStatusResponse, StatusAPIError> {
        let status = SystemStatusResponse {
            engine_status: self.engine.get_status().await?,
            plugin_count: self.engine.plugin_registry.get_plugin_count(),
            active_effects_count: self.engine.status_effect_manager.get_active_effects_count().await?,
            active_immunities_count: self.engine.immunity_manager.get_active_immunities_count().await?,
            cache_hit_rate: self.engine.status_cache.get_hit_rate(),
            memory_usage: self.engine.memory_pool.get_memory_usage(),
            uptime: self.engine.get_uptime(),
        };
        
        Ok(status)
    }
}

/// System Status Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusResponse {
    pub engine_status: EngineStatus,
    pub plugin_count: usize,
    pub active_effects_count: usize,
    pub active_immunities_count: usize,
    pub cache_hit_rate: f64,
    pub memory_usage: MemoryUsage,
    pub uptime: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineStatus {
    Initializing,
    Running,
    Stopping,
    Stopped,
    Error(String),
}
```

### **3. Event API**

```rust
/// Status Core Event API
pub struct StatusCoreEventAPI {
    // Event dispatcher
    event_dispatcher: Arc<StatusEventDispatcher>,
    
    // Event configuration
    config: StatusCoreEventConfig,
    
    // Event logging
    logger: EventLogger,
}

impl StatusCoreEventAPI {
    /// Subscribe to status effect events
    pub async fn subscribe_to_status_effect_events(
        &self,
        subscriber: Box<dyn StatusEventSubscriber>
    ) -> Result<String, StatusAPIError> {
        let subscription_id = Uuid::new_v4().to_string();
        self.event_dispatcher.subscribe(
            StatusEventType::StatusEffectApplied,
            subscription_id.clone(),
            subscriber
        ).await?;
        self.logger.log_event_subscription_created(&subscription_id).await?;
        Ok(subscription_id)
    }
    
    /// Unsubscribe from events
    pub async fn unsubscribe_from_events(
        &self,
        subscription_id: &str
    ) -> Result<(), StatusAPIError> {
        self.event_dispatcher.unsubscribe(subscription_id).await?;
        self.logger.log_event_subscription_removed(subscription_id).await?;
        Ok(())
    }
    
    /// Publish status effect event
    pub async fn publish_status_effect_event(
        &self,
        event: StatusEffectEvent
    ) -> Result<(), StatusAPIError> {
        self.event_dispatcher.publish(event).await?;
        self.logger.log_event_published(&event.event_type).await?;
        Ok(())
    }
    
    /// Get event history
    pub async fn get_event_history(
        &self,
        request: GetEventHistoryRequest
    ) -> Result<GetEventHistoryResponse, StatusAPIError> {
        let events = self.event_dispatcher.get_event_history(
            &request.actor_id,
            request.event_type,
            request.start_time,
            request.end_time,
            request.limit
        ).await?;
        
        Ok(GetEventHistoryResponse {
            events: events.into_iter().map(|e| EventResponse::from(e)).collect(),
            total_count: events.len(),
        })
    }
}

/// Status Effect Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectEvent {
    pub event_id: String,
    pub event_type: StatusEventType,
    pub actor_id: String,
    pub effect_id: String,
    pub timestamp: SystemTime,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEventType {
    StatusEffectApplied,
    StatusEffectRemoved,
    StatusEffectExpired,
    StatusEffectProcessed,
    ImmunityApplied,
    ImmunityRemoved,
    ImmunityBroken,
    PluginLoaded,
    PluginUnloaded,
    ConfigurationReloaded,
}

/// Status Event Subscriber
#[async_trait]
pub trait StatusEventSubscriber: Send + Sync {
    async fn handle_event(&self, event: &StatusEffectEvent) -> Result<(), StatusError>;
    fn get_subscription_id(&self) -> &str;
    fn get_event_types(&self) -> &[StatusEventType];
}
```

### **4. Admin API**

```rust
/// Status Core Admin API
pub struct StatusCoreAdminAPI {
    // Core engine
    engine: Arc<StatusCoreEngine>,
    
    // Admin configuration
    config: StatusCoreAdminConfig,
    
    // Admin authentication
    admin_auth: AdminAuthService,
    
    // Admin logging
    logger: AdminLogger,
}

impl StatusCoreAdminAPI {
    /// Get system metrics
    pub async fn get_system_metrics(
        &self,
        request: GetSystemMetricsRequest
    ) -> Result<GetSystemMetricsResponse, StatusAPIError> {
        // Authenticate admin request
        self.admin_auth.authenticate_admin_request(&request).await?;
        
        let metrics = SystemMetrics {
            performance_metrics: self.engine.get_performance_metrics().await?,
            memory_metrics: self.engine.memory_pool.get_memory_metrics(),
            cache_metrics: self.engine.status_cache.get_cache_metrics(),
            plugin_metrics: self.engine.plugin_registry.get_plugin_metrics(),
            error_metrics: self.engine.get_error_metrics().await?,
        };
        
        Ok(GetSystemMetricsResponse {
            metrics,
            timestamp: SystemTime::now(),
        })
    }
    
    /// Update system configuration
    pub async fn update_system_configuration(
        &self,
        request: UpdateSystemConfigurationRequest
    ) -> Result<UpdateSystemConfigurationResponse, StatusAPIError> {
        // Authenticate admin request
        self.admin_auth.authenticate_admin_request(&request).await?;
        
        // Validate configuration
        self.engine.configuration_manager.validate_configuration(&request.config).await?;
        
        // Update configuration
        self.engine.configuration_manager.update_configuration(request.config).await?;
        
        // Log configuration update
        self.logger.log_configuration_updated(&request.admin_id).await?;
        
        Ok(UpdateSystemConfigurationResponse {
            success: true,
            updated_at: SystemTime::now(),
        })
    }
    
    /// Clear system cache
    pub async fn clear_system_cache(
        &self,
        request: ClearSystemCacheRequest
    ) -> Result<ClearSystemCacheResponse, StatusAPIError> {
        // Authenticate admin request
        self.admin_auth.authenticate_admin_request(&request).await?;
        
        // Clear cache
        self.engine.status_cache.clear_all_caches().await?;
        
        // Log cache clear
        self.logger.log_cache_cleared(&request.admin_id).await?;
        
        Ok(ClearSystemCacheResponse {
            success: true,
            cleared_at: SystemTime::now(),
        })
    }
    
    /// Get system logs
    pub async fn get_system_logs(
        &self,
        request: GetSystemLogsRequest
    ) -> Result<GetSystemLogsResponse, StatusAPIError> {
        // Authenticate admin request
        self.admin_auth.authenticate_admin_request(&request).await?;
        
        let logs = self.engine.get_system_logs(
            request.log_level,
            request.start_time,
            request.end_time,
            request.limit
        ).await?;
        
        Ok(GetSystemLogsResponse {
            logs: logs.into_iter().map(|l| LogEntryResponse::from(l)).collect(),
            total_count: logs.len(),
        })
    }
}

/// System Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub performance_metrics: PerformanceMetrics,
    pub memory_metrics: MemoryMetrics,
    pub cache_metrics: CacheMetrics,
    pub plugin_metrics: PluginMetrics,
    pub error_metrics: ErrorMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub requests_per_second: f64,
    pub average_response_time: Duration,
    pub p95_response_time: Duration,
    pub p99_response_time: Duration,
    pub error_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
    pub memory_usage_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    pub cache_hit_rate: f64,
    pub cache_miss_rate: f64,
    pub cache_size: usize,
    pub cache_evictions: u64,
}
```

## 🔧 **Error Handling**

### **1. API Error Types**

```rust
/// Status API Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusAPIError {
    pub error_code: String,
    pub error_message: String,
    pub error_details: Option<serde_json::Value>,
    pub timestamp: SystemTime,
    pub request_id: Option<String>,
}

impl StatusAPIError {
    /// Create validation error
    pub fn validation_error(message: &str, details: Option<serde_json::Value>) -> Self {
        Self {
            error_code: "VALIDATION_ERROR".to_string(),
            error_message: message.to_string(),
            error_details: details,
            timestamp: SystemTime::now(),
            request_id: None,
        }
    }
    
    /// Create authentication error
    pub fn authentication_error(message: &str) -> Self {
        Self {
            error_code: "AUTHENTICATION_ERROR".to_string(),
            error_message: message.to_string(),
            error_details: None,
            timestamp: SystemTime::now(),
            request_id: None,
        }
    }
    
    /// Create rate limit error
    pub fn rate_limit_error(message: &str, retry_after: Option<Duration>) -> Self {
        Self {
            error_code: "RATE_LIMIT_ERROR".to_string(),
            error_message: message.to_string(),
            error_details: retry_after.map(|d| serde_json::json!({
                "retry_after_seconds": d.as_secs()
            })),
            timestamp: SystemTime::now(),
            request_id: None,
        }
    }
    
    /// Create internal error
    pub fn internal_error(message: &str, details: Option<serde_json::Value>) -> Self {
        Self {
            error_code: "INTERNAL_ERROR".to_string(),
            error_message: message.to_string(),
            error_details: details,
            timestamp: SystemTime::now(),
            request_id: None,
        }
    }
}

/// Error Codes
pub mod error_codes {
    pub const VALIDATION_ERROR: &str = "VALIDATION_ERROR";
    pub const AUTHENTICATION_ERROR: &str = "AUTHENTICATION_ERROR";
    pub const AUTHORIZATION_ERROR: &str = "AUTHORIZATION_ERROR";
    pub const RATE_LIMIT_ERROR: &str = "RATE_LIMIT_ERROR";
    pub const NOT_FOUND_ERROR: &str = "NOT_FOUND_ERROR";
    pub const CONFLICT_ERROR: &str = "CONFLICT_ERROR";
    pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
    pub const SERVICE_UNAVAILABLE_ERROR: &str = "SERVICE_UNAVAILABLE_ERROR";
}
```

### **2. Request Validation**

```rust
/// Request Validator
pub struct RequestValidator {
    // Validation configuration
    config: ValidationConfig,
    
    // Validation rules
    rules: HashMap<String, ValidationRule>,
}

impl RequestValidator {
    /// Validate apply status effect request
    pub fn validate_apply_status_effect_request(
        &self,
        request: &ApplyStatusEffectRequest
    ) -> Result<(), StatusAPIError> {
        // Validate actor ID
        if request.actor_id.is_empty() {
            return Err(StatusAPIError::validation_error(
                "Actor ID cannot be empty",
                None
            ));
        }
        
        // Validate status effect
        self.validate_status_effect(&request.status_effect)?;
        
        // Validate context
        self.validate_status_context(&request.context)?;
        
        Ok(())
    }
    
    /// Validate status effect
    fn validate_status_effect(
        &self,
        status_effect: &StatusEffect
    ) -> Result<(), StatusAPIError> {
        if status_effect.effect_id.is_empty() {
            return Err(StatusAPIError::validation_error(
                "Effect ID cannot be empty",
                None
            ));
        }
        
        if status_effect.magnitude.base_value < 0.0 {
            return Err(StatusAPIError::validation_error(
                "Magnitude base value cannot be negative",
                None
            ));
        }
        
        if status_effect.duration.base_duration.as_secs() == 0 {
            return Err(StatusAPIError::validation_error(
                "Duration base value cannot be zero",
                None
            ));
        }
        
        Ok(())
    }
    
    /// Validate status context
    fn validate_status_context(
        &self,
        context: &StatusContext
    ) -> Result<(), StatusAPIError> {
        // Validate context based on configuration
        if let Some(required_stats) = &self.config.required_stats {
            for stat in required_stats {
                if !context.has_stat(stat) {
                    return Err(StatusAPIError::validation_error(
                        &format!("Required stat '{}' not found in context", stat),
                        None
                    ));
                }
            }
        }
        
        Ok(())
    }
}
```

## 🚀 **Rate Limiting**

### **1. Rate Limiter**

```rust
/// Rate Limiter
pub struct RateLimiter {
    // Rate limiting configuration
    config: RateLimiterConfig,
    
    // Rate limiting storage
    storage: RateLimiterStorage,
    
    // Rate limiting metrics
    metrics: RateLimiterMetrics,
}

impl RateLimiter {
    /// Check rate limit for actor
    pub async fn check_rate_limit(
        &self,
        actor_id: &str
    ) -> Result<(), StatusAPIError> {
        let current_time = SystemTime::now();
        let window_start = current_time - self.config.window_duration;
        
        // Get current request count
        let request_count = self.storage.get_request_count(actor_id, window_start, current_time).await?;
        
        if request_count >= self.config.max_requests_per_window {
            let retry_after = self.calculate_retry_after(actor_id, current_time).await?;
            return Err(StatusAPIError::rate_limit_error(
                "Rate limit exceeded",
                Some(retry_after)
            ));
        }
        
        // Increment request count
        self.storage.increment_request_count(actor_id, current_time).await?;
        
        // Update metrics
        self.metrics.increment_requests(actor_id).await?;
        
        Ok(())
    }
    
    /// Calculate retry after duration
    async fn calculate_retry_after(
        &self,
        actor_id: &str,
        current_time: SystemTime
    ) -> Result<Duration, StatusAPIError> {
        let window_start = current_time - self.config.window_duration;
        let oldest_request = self.storage.get_oldest_request(actor_id, window_start).await?;
        
        if let Some(oldest) = oldest_request {
            let retry_after = oldest + self.config.window_duration - current_time;
            Ok(retry_after.max(Duration::ZERO))
        } else {
            Ok(Duration::ZERO)
        }
    }
}

/// Rate Limiter Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimiterConfig {
    pub max_requests_per_window: u32,
    pub window_duration: Duration,
    pub burst_limit: Option<u32>,
    pub enabled: bool,
}
```

## 🧪 **Testing Strategy**

### **1. API Testing**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_apply_status_effect_api() {
        let api = StatusCorePublicAPI::new(create_test_config()).await?;
        let request = ApplyStatusEffectRequest {
            actor_id: "test_actor".to_string(),
            status_effect: create_test_status_effect(),
            context: create_test_status_context(),
            request_id: Some("test_request".to_string()),
            timestamp: Some(SystemTime::now()),
        };
        
        let response = api.apply_status_effect(request).await?;
        assert!(response.success);
        assert_eq!(response.effect_id, "test_effect");
    }
    
    #[tokio::test]
    async fn test_rate_limiting() {
        let api = StatusCorePublicAPI::new(create_test_config_with_rate_limiting()).await?;
        let request = create_test_apply_status_effect_request();
        
        // Should succeed first few times
        for _ in 0..5 {
            let response = api.apply_status_effect(request.clone()).await?;
            assert!(response.success);
        }
        
        // Should fail due to rate limiting
        let response = api.apply_status_effect(request).await;
        assert!(response.is_err());
        assert_eq!(response.unwrap_err().error_code, "RATE_LIMIT_ERROR");
    }
    
    #[tokio::test]
    async fn test_authentication() {
        let api = StatusCorePublicAPI::new(create_test_config_with_auth()).await?;
        let request = create_test_apply_status_effect_request_without_auth();
        
        let response = api.apply_status_effect(request).await;
        assert!(response.is_err());
        assert_eq!(response.unwrap_err().error_code, "AUTHENTICATION_ERROR");
    }
}
```

### **2. Integration Testing**

```rust
#[tokio::test]
async fn test_api_integration() {
    let api = StatusCorePublicAPI::new(create_test_config()).await?;
    
    // Test complete workflow
    let apply_request = create_test_apply_status_effect_request();
    let apply_response = api.apply_status_effect(apply_request).await?;
    assert!(apply_response.success);
    
    let get_request = GetActorStatusEffectsRequest {
        actor_id: "test_actor".to_string(),
        include_expired: Some(false),
        category_filter: None,
        effect_type_filter: None,
        request_id: Some("test_request".to_string()),
        timestamp: Some(SystemTime::now()),
    };
    let get_response = api.get_actor_status_effects(get_request).await?;
    assert_eq!(get_response.effects.len(), 1);
    
    let remove_request = RemoveStatusEffectRequest {
        actor_id: "test_actor".to_string(),
        effect_id: "test_effect".to_string(),
        request_id: Some("test_request".to_string()),
        timestamp: Some(SystemTime::now()),
    };
    let remove_response = api.remove_status_effect(remove_request).await?;
    assert!(remove_response.success);
}
```

## 📝 **Implementation Notes**

### **1. API Design**
- **RESTful Design**: Follow RESTful API patterns
- **Type Safety**: Use strong typing cho all API endpoints
- **Async Operations**: Use async operations cho better performance
- **Error Handling**: Comprehensive error handling với detailed error codes

### **2. Security**
- **Authentication**: Implement proper authentication
- **Authorization**: Implement authorization checks
- **Rate Limiting**: Implement rate limiting để prevent abuse
- **Input Validation**: Validate all input parameters

### **3. Performance**
- **Caching**: Use caching cho frequently accessed data
- **Rate Limiting**: Implement rate limiting để prevent overload
- **Async Operations**: Use async operations cho better concurrency
- **Monitoring**: Implement monitoring cho API performance

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
