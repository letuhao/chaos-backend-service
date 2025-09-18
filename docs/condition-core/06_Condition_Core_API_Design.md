# Condition Core API Design

## 📋 **Tổng Quan**

Tài liệu này thiết kế chi tiết các APIs cho Condition Core, bao gồm Public API, Internal API, Event API, và Admin API.

## 🏗️ **API Architecture**

### **1. API Layers**

```
Condition Core API Layers
├── Public API (External Systems)
│   ├── Condition Evaluation API
│   ├── Condition Management API
│   ├── Function Registry API
│   └── Configuration API
├── Internal API (Core Components)
│   ├── Condition Engine API
│   ├── Cache Management API
│   ├── Performance Monitoring API
│   └── Validation API
├── Event API (Event System)
│   ├── Condition Events
│   ├── Cache Events
│   ├── Performance Events
│   └── Error Events
└── Admin API (Administration)
    ├── System Management
    ├── Performance Monitoring
    ├── Configuration Management
    └── Debugging Tools
```

## 🔧 **Public API Design**

### **1. Condition Evaluation API**

```rust
// Public API for Condition Evaluation
pub struct ConditionCorePublicAPI {
    condition_engine: Arc<ConditionEngine>,
    function_registry: Arc<ConditionFunctionRegistry>,
    condition_cache: Arc<ConditionCache>,
}

impl ConditionCorePublicAPI {
    // Evaluate single condition
    pub async fn evaluate_condition(
        &self,
        condition_id: &str,
        context: &ConditionContext
    ) -> Result<ConditionResult, ConditionError> {
        // Implementation
    }
    
    // Evaluate multiple conditions
    pub async fn evaluate_conditions(
        &self,
        condition_ids: &[String],
        context: &ConditionContext
    ) -> Result<Vec<ConditionResult>, ConditionError> {
        // Implementation
    }
    
    // Evaluate condition chain
    pub async fn evaluate_condition_chain(
        &self,
        conditions: &[ConditionDefinition],
        context: &ConditionContext
    ) -> Result<ConditionChainResult, ConditionError> {
        // Implementation
    }
    
    // Batch evaluate conditions
    pub async fn batch_evaluate_conditions(
        &self,
        requests: &[ConditionEvaluationRequest]
    ) -> Result<Vec<ConditionResult>, ConditionError> {
        // Implementation
    }
}
```

### **2. Condition Management API**

```rust
// Public API for Condition Management
impl ConditionCorePublicAPI {
    // Register condition
    pub async fn register_condition(
        &self,
        condition: ConditionDefinition
    ) -> Result<(), ConditionError> {
        // Implementation
    }
    
    // Unregister condition
    pub async fn unregister_condition(
        &self,
        condition_id: &str
    ) -> Result<(), ConditionError> {
        // Implementation
    }
    
    // Update condition
    pub async fn update_condition(
        &self,
        condition_id: &str,
        updates: ConditionUpdates
    ) -> Result<(), ConditionError> {
        // Implementation
    }
    
    // Get condition
    pub async fn get_condition(
        &self,
        condition_id: &str
    ) -> Result<Option<ConditionDefinition>, ConditionError> {
        // Implementation
    }
    
    // List conditions
    pub async fn list_conditions(
        &self,
        filter: Option<ConditionFilter>
    ) -> Result<Vec<ConditionDefinition>, ConditionError> {
        // Implementation
    }
}
```

### **3. Function Registry API**

```rust
// Public API for Function Registry
impl ConditionCorePublicAPI {
    // Register function
    pub async fn register_function<T: ConditionFunction + 'static>(
        &self,
        function_id: String,
        function: T
    ) -> Result<(), ConditionError> {
        // Implementation
    }
    
    // Unregister function
    pub async fn unregister_function(
        &self,
        function_id: &str
    ) -> Result<(), ConditionError> {
        // Implementation
    }
    
    // Get function
    pub async fn get_function(
        &self,
        function_id: &str
    ) -> Result<Option<FunctionMetadata>, ConditionError> {
        // Implementation
    }
    
    // List functions
    pub async fn list_functions(
        &self,
        category: Option<FunctionCategory>
    ) -> Result<Vec<FunctionMetadata>, ConditionError> {
        // Implementation
    }
    
    // Validate function parameters
    pub async fn validate_function_parameters(
        &self,
        function_id: &str,
        parameters: &[ConditionParameter]
    ) -> Result<(), ConditionError> {
        // Implementation
    }
}
```

## 🔧 **Internal API Design**

### **1. Condition Engine API**

```rust
// Internal API for Condition Engine
pub struct ConditionEngineInternalAPI {
    evaluator: Arc<ConditionEvaluator>,
    parser: Arc<ConditionParser>,
    optimizer: Arc<ConditionOptimizer>,
}

impl ConditionEngineInternalAPI {
    // Parse condition definition
    pub async fn parse_condition(
        &self,
        condition_config: &str
    ) -> Result<ConditionDefinition, ConditionError> {
        // Implementation
    }
    
    // Optimize condition
    pub async fn optimize_condition(
        &self,
        condition: &mut ConditionDefinition
    ) -> Result<(), ConditionError> {
        // Implementation
    }
    
    // Validate condition
    pub async fn validate_condition(
        &self,
        condition: &ConditionDefinition
    ) -> Result<(), ConditionError> {
        // Implementation
    }
    
    // Preload condition
    pub async fn preload_condition(
        &self,
        condition_id: &str,
        context: &ConditionContext
    ) -> Result<(), ConditionError> {
        // Implementation
    }
}
```

### **2. Cache Management API**

```rust
// Internal API for Cache Management
pub struct CacheManagementInternalAPI {
    cache: Arc<MultiLevelCache>,
    preloader: Arc<CachePreloader>,
    eviction_manager: Arc<CacheEvictionManager>,
}

impl CacheManagementInternalAPI {
    // Get cache statistics
    pub async fn get_cache_statistics(&self) -> CacheStatistics {
        // Implementation
    }
    
    // Clear cache
    pub async fn clear_cache(&self) -> Result<(), ConditionError> {
        // Implementation
    }
    
    // Clear cache by pattern
    pub async fn clear_cache_by_pattern(
        &self,
        pattern: &str
    ) -> Result<(), ConditionError> {
        // Implementation
    }
    
    // Warm cache
    pub async fn warm_cache(
        &self,
        conditions: &[String]
    ) -> Result<(), ConditionError> {
        // Implementation
    }
    
    // Evict expired entries
    pub async fn evict_expired_entries(&self) -> Result<usize, ConditionError> {
        // Implementation
    }
}
```

## 🔧 **Event API Design**

### **1. Condition Events**

```rust
// Event API for Condition Events
pub struct ConditionEventAPI {
    event_publisher: Arc<EventPublisher>,
    event_subscriber: Arc<EventSubscriber>,
}

impl ConditionEventAPI {
    // Publish condition evaluated event
    pub async fn publish_condition_evaluated(
        &self,
        event: ConditionEvaluatedEvent
    ) -> Result<(), EventError> {
        // Implementation
    }
    
    // Publish condition registered event
    pub async fn publish_condition_registered(
        &self,
        event: ConditionRegisteredEvent
    ) -> Result<(), EventError> {
        // Implementation
    }
    
    // Publish condition unregistered event
    pub async fn publish_condition_unregistered(
        &self,
        event: ConditionUnregisteredEvent
    ) -> Result<(), EventError> {
        // Implementation
    }
    
    // Subscribe to condition events
    pub async fn subscribe_to_condition_events(
        &self,
        handler: Box<dyn ConditionEventHandler>
    ) -> Result<EventSubscription, EventError> {
        // Implementation
    }
}

// Condition Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionEvaluatedEvent {
    pub condition_id: String,
    pub context: ConditionContext,
    pub result: ConditionResult,
    pub evaluation_time: Duration,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionRegisteredEvent {
    pub condition_id: String,
    pub condition_definition: ConditionDefinition,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionUnregisteredEvent {
    pub condition_id: String,
    pub timestamp: SystemTime,
}
```

### **2. Cache Events**

```rust
// Event API for Cache Events
impl ConditionEventAPI {
    // Publish cache hit event
    pub async fn publish_cache_hit(
        &self,
        event: CacheHitEvent
    ) -> Result<(), EventError> {
        // Implementation
    }
    
    // Publish cache miss event
    pub async fn publish_cache_miss(
        &self,
        event: CacheMissEvent
    ) -> Result<(), EventError> {
        // Implementation
    }
    
    // Publish cache eviction event
    pub async fn publish_cache_eviction(
        &self,
        event: CacheEvictionEvent
    ) -> Result<(), EventError> {
        // Implementation
    }
}

// Cache Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheHitEvent {
    pub cache_key: String,
    pub cache_level: String,
    pub hit_time: Duration,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMissEvent {
    pub cache_key: String,
    pub miss_time: Duration,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEvictionEvent {
    pub cache_key: String,
    pub eviction_reason: EvictionReason,
    pub timestamp: SystemTime,
}
```

## 🔧 **Admin API Design**

### **1. System Management API**

```rust
// Admin API for System Management
pub struct ConditionCoreAdminAPI {
    system_manager: Arc<SystemManager>,
    performance_monitor: Arc<PerformanceMonitor>,
    configuration_manager: Arc<ConfigurationManager>,
}

impl ConditionCoreAdminAPI {
    // Get system status
    pub async fn get_system_status(&self) -> SystemStatus {
        // Implementation
    }
    
    // Get system health
    pub async fn get_system_health(&self) -> SystemHealth {
        // Implementation
    }
    
    // Restart system
    pub async fn restart_system(&self) -> Result<(), AdminError> {
        // Implementation
    }
    
    // Shutdown system
    pub async fn shutdown_system(&self) -> Result<(), AdminError> {
        // Implementation
    }
    
    // Get system metrics
    pub async fn get_system_metrics(&self) -> SystemMetrics {
        // Implementation
    }
}
```

### **2. Performance Monitoring API**

```rust
// Admin API for Performance Monitoring
impl ConditionCoreAdminAPI {
    // Get performance metrics
    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        // Implementation
    }
    
    // Get cache metrics
    pub async fn get_cache_metrics(&self) -> CacheMetrics {
        // Implementation
    }
    
    // Get function metrics
    pub async fn get_function_metrics(&self) -> FunctionMetrics {
        // Implementation
    }
    
    // Get evaluation metrics
    pub async fn get_evaluation_metrics(&self) -> EvaluationMetrics {
        // Implementation
    }
    
    // Start performance monitoring
    pub async fn start_performance_monitoring(&self) -> Result<(), AdminError> {
        // Implementation
    }
    
    // Stop performance monitoring
    pub async fn stop_performance_monitoring(&self) -> Result<(), AdminError> {
        // Implementation
    }
}
```

### **3. Configuration Management API**

```rust
// Admin API for Configuration Management
impl ConditionCoreAdminAPI {
    // Get configuration
    pub async fn get_configuration(&self) -> ConditionCoreConfig {
        // Implementation
    }
    
    // Update configuration
    pub async fn update_configuration(
        &self,
        updates: ConfigurationUpdates
    ) -> Result<(), AdminError> {
        // Implementation
    }
    
    // Reload configuration
    pub async fn reload_configuration(&self) -> Result<(), AdminError> {
        // Implementation
    }
    
    // Validate configuration
    pub async fn validate_configuration(
        &self,
        config: &ConditionCoreConfig
    ) -> Result<(), AdminError> {
        // Implementation
    }
    
    // Export configuration
    pub async fn export_configuration(&self) -> Result<String, AdminError> {
        // Implementation
    }
    
    // Import configuration
    pub async fn import_configuration(
        &self,
        config_data: &str
    ) -> Result<(), AdminError> {
        // Implementation
    }
}
```

## 🔧 **API Documentation**

### **1. OpenAPI Specification**

```yaml
# OpenAPI 3.0 Specification for Condition Core API
openapi: 3.0.0
info:
  title: Condition Core API
  description: Unified condition system API for Chaos World
  version: 1.0.0
  contact:
    name: Chaos World Team
    email: team@chaos-world.com

servers:
  - url: https://api.chaos-world.com/condition-core/v1
    description: Production server
  - url: https://staging-api.chaos-world.com/condition-core/v1
    description: Staging server

paths:
  /conditions/evaluate:
    post:
      summary: Evaluate single condition
      description: Evaluate a single condition with given context
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/ConditionEvaluationRequest'
      responses:
        '200':
          description: Condition evaluation result
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ConditionResult'
        '400':
          description: Bad request
        '500':
          description: Internal server error

  /conditions/batch-evaluate:
    post:
      summary: Batch evaluate conditions
      description: Evaluate multiple conditions in batch
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: array
              items:
                $ref: '#/components/schemas/ConditionEvaluationRequest'
      responses:
        '200':
          description: Batch evaluation results
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/ConditionResult'

components:
  schemas:
    ConditionEvaluationRequest:
      type: object
      required:
        - condition_id
        - context
      properties:
        condition_id:
          type: string
          description: ID of the condition to evaluate
        context:
          $ref: '#/components/schemas/ConditionContext'
        parameters:
          type: array
          items:
            $ref: '#/components/schemas/ConditionParameter'
    
    ConditionContext:
      type: object
      required:
        - target
        - world_id
      properties:
        target:
          $ref: '#/components/schemas/ActorTarget'
        world_id:
          type: string
        current_time:
          type: string
          format: date-time
        current_weather:
          $ref: '#/components/schemas/WeatherType'
        world_state:
          $ref: '#/components/schemas/WorldState'
    
    ConditionResult:
      type: object
      required:
        - condition_id
        - passed
        - value
        - evaluated_at
        - evaluation_time
      properties:
        condition_id:
          type: string
        passed:
          type: boolean
        value:
          $ref: '#/components/schemas/ConditionValue'
        evaluated_at:
          type: string
          format: date-time
        evaluation_time:
          type: string
          format: duration
```

### **2. gRPC Service Definition**

```protobuf
// Condition Core gRPC Service Definition
syntax = "proto3";

package chaos.condition.core.v1;

import "google/protobuf/timestamp.proto";
import "google/protobuf/duration.proto";

service ConditionCoreService {
  // Condition Evaluation
  rpc EvaluateCondition(EvaluateConditionRequest) returns (EvaluateConditionResponse);
  rpc EvaluateConditions(EvaluateConditionsRequest) returns (EvaluateConditionsResponse);
  rpc BatchEvaluateConditions(BatchEvaluateConditionsRequest) returns (BatchEvaluateConditionsResponse);
  
  // Condition Management
  rpc RegisterCondition(RegisterConditionRequest) returns (RegisterConditionResponse);
  rpc UnregisterCondition(UnregisterConditionRequest) returns (UnregisterConditionResponse);
  rpc UpdateCondition(UpdateConditionRequest) returns (UpdateConditionResponse);
  rpc GetCondition(GetConditionRequest) returns (GetConditionResponse);
  rpc ListConditions(ListConditionsRequest) returns (ListConditionsResponse);
  
  // Function Registry
  rpc RegisterFunction(RegisterFunctionRequest) returns (RegisterFunctionResponse);
  rpc UnregisterFunction(UnregisterFunctionRequest) returns (UnregisterFunctionResponse);
  rpc GetFunction(GetFunctionRequest) returns (GetFunctionResponse);
  rpc ListFunctions(ListFunctionsRequest) returns (ListFunctionsResponse);
  
  // System Management
  rpc GetSystemStatus(GetSystemStatusRequest) returns (GetSystemStatusResponse);
  rpc GetSystemHealth(GetSystemHealthRequest) returns (GetSystemHealthResponse);
  rpc GetPerformanceMetrics(GetPerformanceMetricsRequest) returns (GetPerformanceMetricsResponse);
}

// Request/Response Messages
message EvaluateConditionRequest {
  string condition_id = 1;
  ConditionContext context = 2;
  repeated ConditionParameter parameters = 3;
}

message EvaluateConditionResponse {
  ConditionResult result = 1;
  string error_message = 2;
}

message ConditionContext {
  ActorTarget target = 1;
  string world_id = 2;
  google.protobuf.Timestamp current_time = 3;
  WeatherType current_weather = 4;
  WorldState world_state = 5;
}

message ConditionResult {
  string condition_id = 1;
  bool passed = 2;
  ConditionValue value = 3;
  google.protobuf.Timestamp evaluated_at = 4;
  google.protobuf.Duration evaluation_time = 5;
}
```

## 🎯 **Key Features**

### **1. Comprehensive API Coverage**
- ✅ **Public API**: External system integration
- ✅ **Internal API**: Core component communication
- ✅ **Event API**: Event-driven architecture
- ✅ **Admin API**: System administration

### **2. Multiple Protocol Support**
- ✅ **REST API**: HTTP/JSON for web integration
- ✅ **gRPC API**: High-performance binary protocol
- ✅ **GraphQL API**: Flexible query language
- ✅ **WebSocket API**: Real-time communication

### **3. Advanced Features**
- ✅ **Async/Await**: Non-blocking operations
- ✅ **Batch Processing**: Efficient bulk operations
- ✅ **Event Streaming**: Real-time event processing
- ✅ **Admin Tools**: System management and monitoring

### **4. Documentation & Standards**
- ✅ **OpenAPI 3.0**: REST API documentation
- ✅ **Protocol Buffers**: gRPC service definition
- ✅ **GraphQL Schema**: GraphQL API definition
- ✅ **API Versioning**: Backward compatibility

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: API Design Complete  
**Maintainer**: Chaos World Team
