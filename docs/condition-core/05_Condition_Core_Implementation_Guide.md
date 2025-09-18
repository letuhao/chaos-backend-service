# Condition Core Implementation Guide

## 📋 **Tổng Quan**

Tài liệu này cung cấp hướng dẫn step-by-step để implement Condition Core system, bao gồm code examples, best practices, và integration guidelines.

## 🚀 **Implementation Phases**

### **Phase 1: Foundation Setup (Week 1)**

#### **1.1 Project Structure Setup**

```bash
# Create Condition Core project structure
mkdir chaos-condition-core
cd chaos-condition-core

# Initialize Rust project
cargo init --lib

# Create directory structure
mkdir -p src/{core,config,functions,evaluation,caching,utils}
mkdir -p examples/{basic,advanced,integration}
mkdir -p tests/{unit,integration,performance}
mkdir -p docs/{api,guides,examples}
```

#### **1.2 Cargo.toml Configuration**

```toml
[package]
name = "chaos-condition-core"
version = "0.1.0"
edition = "2021"
authors = ["Chaos World Team"]
description = "Unified condition system library for Chaos World"
license = "MIT"
repository = "https://github.com/chaos-world/chaos-condition-core"

[dependencies]
# Core dependencies
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"

# Caching
dashmap = "5.0"
lru = "0.12"
redis = { version = "0.23", features = ["tokio-comp"] }

# Performance
rayon = "1.7"
criterion = "0.5"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Configuration
config = "0.13"
toml = "0.8"

# Hashing
xxhash-rust = "0.8"

# Time
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tokio-test = "0.4"
criterion = "0.5"
proptest = "1.0"
mockall = "0.11"

[features]
default = ["yaml", "interface", "caching", "redis"]
yaml = ["serde_yaml"]
interface = ["async-trait"]
caching = ["dashmap", "lru"]
redis = ["redis"]
performance = ["rayon"]
```

#### **1.3 Core Module Structure**

```rust
// src/lib.rs
pub mod core;
pub mod config;
pub mod functions;
pub mod evaluation;
pub mod caching;
pub mod utils;

pub use core::*;
pub use config::*;
pub use functions::*;
pub use evaluation::*;
pub use caching::*;
pub use utils::*;

// Re-exports for convenience
pub use async_trait::async_trait;
pub use serde::{Deserialize, Serialize};
pub use tokio;
pub use tracing;
```

### **Phase 2: Core Implementation (Week 2)**

#### **2.1 Condition Definition Implementation**

```rust
// src/core/condition_definition.rs
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionDefinition {
    // Basic Info
    pub condition_guid: String,
    pub condition_id: String,
    pub condition_name: String,
    pub condition_name_vi: String,
    pub world_id: String,
    
    // Condition Properties
    pub condition_function: String,
    pub condition_parameters: Vec<ConditionParameter>,
    pub condition_operator: ConditionOperator,
    pub condition_value: ConditionValue,
    pub condition_logic: ConditionLogic,
    
    // Categories
    pub categories: Vec<String>,
    
    // Metadata
    pub priority: u32,
    pub cacheable: bool,
    pub cache_ttl: Option<Duration>,
    pub performance_impact: PerformanceImpact,
    
    // Timestamps
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl ConditionDefinition {
    pub fn new(
        condition_id: String,
        condition_function: String,
        condition_operator: ConditionOperator,
        condition_value: ConditionValue,
    ) -> Self {
        Self {
            condition_guid: uuid::Uuid::new_v4().to_string(),
            condition_id,
            condition_name: condition_id.clone(),
            condition_name_vi: condition_id.clone(),
            world_id: "default".to_string(),
            condition_function,
            condition_parameters: Vec::new(),
            condition_operator,
            condition_value,
            condition_logic: ConditionLogic::And,
            categories: Vec::new(),
            priority: 100,
            cacheable: true,
            cache_ttl: Some(Duration::from_secs(60)),
            performance_impact: PerformanceImpact::Low,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
    
    pub fn with_parameters(mut self, parameters: Vec<ConditionParameter>) -> Self {
        self.condition_parameters = parameters;
        self
    }
    
    pub fn with_categories(mut self, categories: Vec<String>) -> Self {
        self.categories = categories;
        self
    }
    
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_cache_settings(mut self, cacheable: bool, ttl: Option<Duration>) -> Self {
        self.cacheable = cacheable;
        self.cache_ttl = ttl;
        self
    }
}
```

#### **2.2 Condition Function Registry Implementation**

```rust
// src/functions/function_registry.rs
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

pub struct ConditionFunctionRegistry {
    functions: HashMap<String, Box<dyn ConditionFunction>>,
    function_categories: HashMap<String, Vec<String>>,
    function_metadata: HashMap<String, FunctionMetadata>,
}

impl ConditionFunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            function_categories: HashMap::new(),
            function_metadata: HashMap::new(),
        }
    }
    
    pub fn register_function<T: ConditionFunction + 'static>(
        &mut self,
        function_id: String,
        function: T
    ) -> Result<(), ConditionError> {
        let metadata = FunctionMetadata {
            function_id: function_id.clone(),
            function_name: function.get_description(),
            function_name_vi: function.get_description_vi(),
            category: self.determine_category(&function_id),
            description: function.get_description(),
            description_vi: function.get_description_vi(),
            parameter_types: function.get_parameter_types(),
            return_type: function.get_return_type(),
            is_async: true,
            cacheable: function.is_cacheable(),
            cache_ttl: function.get_cache_ttl(),
            performance_impact: function.get_performance_impact(),
            version: "1.0.0".to_string(),
            author: "Chaos World Team".to_string(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        
        self.functions.insert(function_id.clone(), Box::new(function));
        self.function_metadata.insert(function_id.clone(), metadata);
        
        // Add to category
        if let Some(category) = self.function_metadata.get(&function_id) {
            self.function_categories
                .entry(category.category.to_string())
                .or_insert_with(Vec::new)
                .push(function_id);
        }
        
        Ok(())
    }
    
    pub fn get_function(&self, function_id: &str) -> Option<&dyn ConditionFunction> {
        self.functions.get(function_id).map(|f| f.as_ref())
    }
    
    pub fn list_functions(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
    
    fn determine_category(&self, function_id: &str) -> FunctionCategory {
        if function_id.starts_with("get_actor_") || function_id.starts_with("is_") {
            FunctionCategory::Actor
        } else if function_id.starts_with("has_item") || function_id.starts_with("get_item") {
            FunctionCategory::Item
        } else if function_id.starts_with("get_in_") || function_id.starts_with("is_in_") {
            FunctionCategory::Location
        } else if function_id.starts_with("get_current_time") || function_id.starts_with("is_day") {
            FunctionCategory::Time
        } else if function_id.starts_with("get_current_weather") || function_id.starts_with("is_raining") {
            FunctionCategory::Weather
        } else if function_id.starts_with("has_magic_") || function_id.starts_with("get_spell") {
            FunctionCategory::Magic
        } else if function_id.starts_with("get_relationship_") || function_id.starts_with("is_hostile") {
            FunctionCategory::Relationship
        } else {
            FunctionCategory::Custom
        }
    }
}
```

#### **2.3 Condition Evaluator Implementation**

```rust
// src/evaluation/condition_evaluator.rs
use std::time::SystemTime;
use std::sync::Arc;

pub struct ConditionEvaluator {
    function_registry: Arc<ConditionFunctionRegistry>,
    condition_cache: Arc<ConditionCache>,
    performance_monitor: Arc<PerformanceMonitor>,
}

impl ConditionEvaluator {
    pub fn new(
        function_registry: Arc<ConditionFunctionRegistry>,
        condition_cache: Arc<ConditionCache>,
        performance_monitor: Arc<PerformanceMonitor>,
    ) -> Self {
        Self {
            function_registry,
            condition_cache,
            performance_monitor,
        }
    }
    
    pub async fn evaluate_condition(
        &self,
        condition: &ConditionDefinition,
        context: &ConditionContext
    ) -> Result<ConditionResult, ConditionError> {
        let start_time = SystemTime::now();
        
        // Check cache first
        let cache_key = self.generate_cache_key(condition, context);
        if let Some(cached_result) = self.condition_cache.get(&cache_key) {
            self.performance_monitor.record_cache_hit(&condition.condition_id);
            return Ok(cached_result.clone());
        }
        
        // Get condition function
        let function = self.function_registry
            .get_function(&condition.condition_function)
            .ok_or(ConditionError::FunctionNotFound(condition.condition_function.clone()))?;
        
        // Evaluate condition
        let condition_value = function.evaluate(&condition.condition_parameters, context).await?;
        
        // Apply operator
        let result = self.apply_operator(
            &condition_value,
            &condition.condition_operator,
            &condition.condition_value
        )?;
        
        // Create result
        let condition_result = ConditionResult {
            condition_id: condition.condition_id.clone(),
            passed: result,
            value: condition_value,
            evaluated_at: SystemTime::now(),
            evaluation_time: start_time.elapsed().unwrap_or_default(),
        };
        
        // Cache result if cacheable
        if condition.cacheable {
            self.condition_cache.cache(cache_key, condition_result.clone());
        }
        
        // Record performance metrics
        self.performance_monitor.record_evaluation(&condition.condition_id, start_time.elapsed().unwrap_or_default());
        
        Ok(condition_result)
    }
    
    fn generate_cache_key(&self, condition: &ConditionDefinition, context: &ConditionContext) -> String {
        let mut hasher = xxhash_rust::xxh3::Xxh3::new();
        hasher.update(condition.condition_id.as_bytes());
        hasher.update(condition.condition_function.as_bytes());
        hasher.update(context.target.id.as_bytes());
        hasher.update(context.world_id.as_bytes());
        
        format!("condition:{}:{}", condition.condition_id, hasher.finish())
    }
    
    fn apply_operator(
        &self,
        value: &ConditionValue,
        operator: &ConditionOperator,
        target_value: &ConditionValue
    ) -> Result<bool, ConditionError> {
        match operator {
            ConditionOperator::Equal => Ok(value == target_value),
            ConditionOperator::NotEqual => Ok(value != target_value),
            ConditionOperator::GreaterThan => Ok(value > target_value),
            ConditionOperator::GreaterThanOrEqual => Ok(value >= target_value),
            ConditionOperator::LessThan => Ok(value < target_value),
            ConditionOperator::LessThanOrEqual => Ok(value <= target_value),
            // ... other operators
            _ => Err(ConditionError::UnsupportedOperator(operator.clone())),
        }
    }
}
```

### **Phase 3: Configuration System (Week 3)**

#### **3.1 YAML Configuration Implementation**

```rust
// src/config/yaml_config.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlConditionConfig {
    pub condition_core: ConditionCoreConfig,
    pub function_registry: FunctionRegistryConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionCoreConfig {
    pub version: String,
    pub world_id: String,
    pub performance: PerformanceSettings,
    pub function_registry: FunctionRegistrySettings,
    pub conditions: ConditionSettings,
    pub integration: IntegrationSettings,
}

impl YamlConditionConfig {
    pub fn load_from_file(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: YamlConditionConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save_to_file(&self, path: &str) -> Result<(), ConfigError> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validate configuration
        if self.condition_core.version.is_empty() {
            return Err(ValidationError::MissingField("version"));
        }
        
        if self.condition_core.world_id.is_empty() {
            return Err(ValidationError::MissingField("world_id"));
        }
        
        // Validate performance settings
        self.condition_core.performance.validate()?;
        
        // Validate function registry settings
        self.condition_core.function_registry.validate()?;
        
        // Validate condition settings
        self.condition_core.conditions.validate()?;
        
        Ok(())
    }
}
```

#### **3.2 Interface Configuration Implementation**

```rust
// src/config/interface_config.rs
use async_trait::async_trait;

pub struct InterfaceConfigParser {
    interface_registry: HashMap<String, Box<dyn ConditionInterface>>,
    interface_metadata: HashMap<String, InterfaceMetadata>,
}

impl InterfaceConfigParser {
    pub fn new() -> Self {
        Self {
            interface_registry: HashMap::new(),
            interface_metadata: HashMap::new(),
        }
    }
    
    pub fn register_interface<T: ConditionInterface + 'static>(
        &mut self,
        interface: T
    ) -> Result<(), ConditionError> {
        let condition_id = interface.get_condition_id();
        let metadata = InterfaceMetadata {
            condition_guid: interface.get_condition_guid(),
            condition_id: condition_id.clone(),
            condition_name: interface.get_condition_name(),
            condition_name_vi: interface.get_condition_name_vi(),
            world_id: interface.get_world_id(),
            categories: interface.get_categories(),
            priority: interface.get_priority(),
            cacheable: interface.is_cacheable(),
            cache_ttl: interface.get_cache_ttl(),
            performance_impact: interface.get_performance_impact(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        
        self.interface_registry.insert(condition_id.clone(), Box::new(interface));
        self.interface_metadata.insert(condition_id, metadata);
        
        Ok(())
    }
    
    pub fn get_interface(&self, condition_id: &str) -> Option<&dyn ConditionInterface> {
        self.interface_registry.get(condition_id).map(|i| i.as_ref())
    }
    
    pub fn list_interfaces(&self) -> Vec<String> {
        self.interface_registry.keys().cloned().collect()
    }
}
```

### **Phase 4: Caching System (Week 4)**

#### **4.1 Multi-Level Cache Implementation**

```rust
// src/caching/multi_level_cache.rs
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use dashmap::DashMap;
use lru::LruCache;

pub struct MultiLevelCache {
    // L1 Cache: In-memory cache (fastest)
    l1_cache: Arc<DashMap<String, CachedConditionResult>>,
    
    // L2 Cache: Shared cache (fast)
    l2_cache: Arc<RedisCache>,
    
    // L3 Cache: Database cache (slower but persistent)
    l3_cache: Arc<DatabaseCache>,
    
    // Cache configuration
    config: CacheConfig,
    
    // Performance monitoring
    metrics: Arc<Mutex<CacheMetrics>>,
}

impl MultiLevelCache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            l1_cache: Arc::new(DashMap::new()),
            l2_cache: Arc::new(RedisCache::new(config.redis_config.clone())),
            l3_cache: Arc::new(DatabaseCache::new(config.database_config.clone())),
            config,
            metrics: Arc::new(Mutex::new(CacheMetrics::new())),
        }
    }
    
    pub async fn get_condition_result(
        &self,
        cache_key: &str
    ) -> Option<ConditionResult> {
        // Try L1 cache first
        if let Some(cached_result) = self.l1_cache.get(cache_key) {
            if cached_result.is_valid() {
                self.record_cache_hit("L1");
                return Some(cached_result.result.clone());
            } else {
                self.l1_cache.remove(cache_key);
            }
        }
        
        // Try L2 cache
        if let Some(cached_result) = self.l2_cache.get(cache_key).await {
            if cached_result.is_valid() {
                // Update L1 cache
                self.l1_cache.insert(cache_key.to_string(), cached_result.clone());
                self.record_cache_hit("L2");
                return Some(cached_result.result);
            } else {
                self.l2_cache.remove(cache_key).await;
            }
        }
        
        // Try L3 cache
        if let Some(cached_result) = self.l3_cache.get(cache_key).await {
            if cached_result.is_valid() {
                // Update L1 and L2 caches
                self.l1_cache.insert(cache_key.to_string(), cached_result.clone());
                self.l2_cache.set(cache_key, cached_result.clone()).await;
                self.record_cache_hit("L3");
                return Some(cached_result.result);
            } else {
                self.l3_cache.remove(cache_key).await;
            }
        }
        
        self.record_cache_miss();
        None
    }
    
    pub async fn cache_condition_result(
        &self,
        cache_key: String,
        result: ConditionResult,
        ttl: Option<Duration>
    ) {
        let ttl = ttl.unwrap_or(self.config.default_ttl);
        let cached_result = CachedConditionResult {
            result,
            cached_at: SystemTime::now(),
            ttl,
            version: env!("CARGO_PKG_VERSION").to_string(),
            checksum: 0, // Will be calculated
        };
        
        // Cache in all levels
        self.l1_cache.insert(cache_key.clone(), cached_result.clone());
        self.l2_cache.set(&cache_key, cached_result.clone()).await;
        self.l3_cache.set(&cache_key, cached_result).await;
    }
    
    fn record_cache_hit(&self, level: &str) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.hits += 1;
            metrics.total_requests += 1;
        }
    }
    
    fn record_cache_miss(&self) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.misses += 1;
            metrics.total_requests += 1;
        }
    }
}
```

### **Phase 5: Testing Implementation (Week 5)**

#### **5.1 Unit Tests**

```rust
// tests/unit/condition_evaluator_test.rs
use chaos_condition_core::*;
use tokio_test;

#[tokio::test]
async fn test_evaluate_health_condition() {
    // Setup
    let function_registry = Arc::new(ConditionFunctionRegistry::new());
    let condition_cache = Arc::new(ConditionCache::new());
    let performance_monitor = Arc::new(PerformanceMonitor::new());
    
    let evaluator = ConditionEvaluator::new(
        function_registry,
        condition_cache,
        performance_monitor,
    );
    
    // Register health function
    let health_function = GetActorValueFunction;
    evaluator.function_registry.register_function(
        "get_actor_value".to_string(),
        health_function
    ).unwrap();
    
    // Create condition
    let condition = ConditionDefinition::new(
        "health_condition".to_string(),
        "get_actor_value".to_string(),
        ConditionOperator::LessThan,
        ConditionValue::Float(0.5),
    ).with_parameters(vec![ConditionParameter::String("health".to_string())]);
    
    // Create context
    let context = ConditionContext {
        target: ActorTarget { id: "player_1".to_string() },
        world_id: "test_world".to_string(),
        current_time: SystemTime::now(),
        current_weather: WeatherType::Clear,
        world_state: WorldState::default(),
    };
    
    // Test evaluation
    let result = evaluator.evaluate_condition(&condition, &context).await;
    assert!(result.is_ok());
    
    let condition_result = result.unwrap();
    assert_eq!(condition_result.condition_id, "health_condition");
    assert!(condition_result.evaluation_time.as_millis() < 100); // Should be fast
}
```

#### **5.2 Integration Tests**

```rust
// tests/integration/condition_core_integration_test.rs
use chaos_condition_core::*;

#[tokio::test]
async fn test_condition_core_integration() {
    // Setup Condition Core
    let condition_core = ConditionCore::new(ConditionCoreConfig::default()).await;
    
    // Load configuration
    let config = YamlConditionConfig::load_from_file("tests/fixtures/test_config.yaml").unwrap();
    condition_core.load_configuration(config).await.unwrap();
    
    // Register test conditions
    let health_condition = ConditionDefinition::new(
        "health_condition".to_string(),
        "get_actor_value".to_string(),
        ConditionOperator::LessThan,
        ConditionValue::Float(0.5),
    );
    
    condition_core.register_condition(health_condition).await.unwrap();
    
    // Test condition evaluation
    let context = create_test_context();
    let result = condition_core.evaluate_condition("health_condition", &context).await;
    
    assert!(result.is_ok());
    let condition_result = result.unwrap();
    assert_eq!(condition_result.condition_id, "health_condition");
}
```

#### **5.3 Performance Tests**

```rust
// tests/performance/condition_core_performance_test.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use chaos_condition_core::*;

fn benchmark_condition_evaluation(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let condition_core = rt.block_on(async {
        ConditionCore::new(ConditionCoreConfig::default()).await
    });
    
    let context = create_test_context();
    
    c.bench_function("evaluate_condition", |b| {
        b.iter(|| {
            rt.block_on(async {
                condition_core.evaluate_condition("health_condition", &context).await
            })
        })
    });
}

fn benchmark_cache_performance(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let condition_core = rt.block_on(async {
        ConditionCore::new(ConditionCoreConfig::default()).await
    });
    
    let context = create_test_context();
    
    c.bench_function("cached_evaluation", |b| {
        b.iter(|| {
            rt.block_on(async {
                // First call - cache miss
                let _ = condition_core.evaluate_condition("health_condition", &context).await;
                // Second call - cache hit
                condition_core.evaluate_condition("health_condition", &context).await
            })
        })
    });
}

criterion_group!(benches, benchmark_condition_evaluation, benchmark_cache_performance);
criterion_main!(benches);
```

## 🎯 **Best Practices**

### **1. Error Handling**

```rust
// Always use Result for fallible operations
pub async fn evaluate_condition(
    &self,
    condition: &ConditionDefinition,
    context: &ConditionContext
) -> Result<ConditionResult, ConditionError> {
    // Use ? operator for error propagation
    let function = self.function_registry
        .get_function(&condition.condition_function)
        .ok_or(ConditionError::FunctionNotFound(condition.condition_function.clone()))?;
    
    // Handle specific error cases
    match function.evaluate(&condition.condition_parameters, context).await {
        Ok(value) => Ok(value),
        Err(ConditionError::Timeout) => {
            tracing::warn!("Condition evaluation timeout: {}", condition.condition_id);
            Err(ConditionError::Timeout)
        },
        Err(e) => {
            tracing::error!("Condition evaluation failed: {:?}", e);
            Err(e)
        }
    }
}
```

### **2. Performance Optimization**

```rust
// Use async/await for I/O operations
pub async fn evaluate_condition_async(
    &self,
    condition: &ConditionDefinition,
    context: &ConditionContext
) -> Result<ConditionResult, ConditionError> {
    // Check cache first (fast)
    if let Some(cached_result) = self.condition_cache.get(&cache_key).await {
        return Ok(cached_result);
    }
    
    // Evaluate condition (potentially slow)
    let result = self.evaluate_condition_uncached(condition, context).await?;
    
    // Cache result (async)
    self.condition_cache.cache(cache_key, result.clone()).await;
    
    Ok(result)
}

// Use batch processing for multiple conditions
pub async fn evaluate_conditions_batch(
    &self,
    conditions: &[ConditionDefinition],
    context: &ConditionContext
) -> Result<Vec<ConditionResult>, ConditionError> {
    let mut tasks = Vec::new();
    
    for condition in conditions {
        let task = self.evaluate_condition(condition, context);
        tasks.push(task);
    }
    
    // Execute all conditions in parallel
    let results = futures::future::join_all(tasks).await;
    
    // Collect results
    let mut condition_results = Vec::new();
    for result in results {
        condition_results.push(result?);
    }
    
    Ok(condition_results)
}
```

### **3. Configuration Management**

```rust
// Use builder pattern for complex configurations
pub struct ConditionCoreBuilder {
    config: ConditionCoreConfig,
}

impl ConditionCoreBuilder {
    pub fn new() -> Self {
        Self {
            config: ConditionCoreConfig::default(),
        }
    }
    
    pub fn with_cache_settings(mut self, cache_config: CacheConfig) -> Self {
        self.config.cache = cache_config;
        self
    }
    
    pub fn with_performance_settings(mut self, performance_config: PerformanceConfig) -> Self {
        self.config.performance = performance_config;
        self
    }
    
    pub fn with_function_registry(mut self, function_registry: ConditionFunctionRegistry) -> Self {
        self.config.function_registry = function_registry;
        self
    }
    
    pub async fn build(self) -> Result<ConditionCore, ConditionError> {
        let condition_core = ConditionCore::new(self.config).await?;
        Ok(condition_core)
    }
}

// Usage
let condition_core = ConditionCoreBuilder::new()
    .with_cache_settings(CacheConfig::high_performance())
    .with_performance_settings(PerformanceConfig::optimized())
    .with_function_registry(function_registry)
    .build()
    .await?;
```

## 📝 **Integration Guidelines**

### **1. Action Core Integration**

```rust
// Action Core integration example
pub struct ActionCore {
    condition_core: Arc<ConditionCore>,
    // ... other fields
}

impl ActionCore {
    pub async fn evaluate_action_conditions(
        &self,
        action: &Action,
        context: &ActionContext
    ) -> Result<bool, ActionError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition_id in &action.condition_ids {
            let result = self.condition_core
                .evaluate_condition(condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    fn convert_to_condition_context(&self, action_context: &ActionContext) -> ConditionContext {
        ConditionContext {
            target: ActorTarget { id: action_context.actor_id.clone() },
            world_id: action_context.world_id.clone(),
            current_time: action_context.current_time,
            current_weather: action_context.current_weather,
            world_state: action_context.world_state.clone(),
        }
    }
}
```

### **2. Status Core Integration**

```rust
// Status Core integration example
pub struct StatusCore {
    condition_core: Arc<ConditionCore>,
    // ... other fields
}

impl StatusCore {
    pub async fn evaluate_status_conditions(
        &self,
        status_effect: &StatusEffect,
        context: &StatusContext
    ) -> Result<bool, StatusError> {
        let condition_context = self.convert_to_condition_context(context);
        
        for condition in &status_effect.conditions {
            let result = self.condition_core
                .evaluate_condition(&condition.condition_id, &condition_context)
                .await?;
            
            if !result.passed {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}
```

## 🎯 **Key Implementation Points**

### **1. Async/Await Usage**
- ✅ Use async/await for all I/O operations
- ✅ Use tokio for async runtime
- ✅ Use futures for parallel execution

### **2. Error Handling**
- ✅ Use Result<T, E> for fallible operations
- ✅ Use thiserror for custom error types
- ✅ Use anyhow for error context

### **3. Performance**
- ✅ Use Arc for shared ownership
- ✅ Use DashMap for concurrent access
- ✅ Use LRU cache for memory efficiency
- ✅ Use batch processing for multiple operations

### **4. Configuration**
- ✅ Use serde for serialization
- ✅ Use builder pattern for complex configs
- ✅ Use validation for config integrity

### **5. Testing**
- ✅ Use tokio-test for async testing
- ✅ Use mockall for mocking
- ✅ Use criterion for performance testing
- ✅ Use proptest for property testing

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Implementation Guide Complete  
**Maintainer**: Chaos World Team
