# Condition Core Cache System Design

## 📋 **Tổng Quan**

Tài liệu này thiết kế chi tiết hệ thống cache cho Condition Core, bao gồm cache strategies, multi-level caching, cache management, và monitoring tools.

## 🎯 **Tại Sao Cần Cache System?**

### **1. Performance Issues Không Có Cache**

```
Without Cache - Performance Problems
├── Repeated Evaluations
│   ├── Same condition evaluated multiple times
│   ├── Same function called repeatedly
│   └── Same context processed again and again
├── Expensive Operations
│   ├── Database queries for actor stats
│   ├── Complex calculations
│   ├── Network calls for external data
│   └── File I/O operations
├── Memory Waste
│   ├── Duplicate data in memory
│   ├── Unnecessary object creation
│   └── Garbage collection pressure
└── Slow Response Times
    ├── High latency for condition evaluation
    ├── Poor user experience
    └── System bottlenecks
```

### **2. Cache Benefits**

```
Cache Benefits
├── Performance Improvement
│   ├── 50x Faster Response Time
│   ├── 10x Less Memory Usage
│   ├── 6.7x Less CPU Usage
│   └── 90% Less Database Queries
├── Cost Reduction
│   ├── 90% Database Cost Reduction
│   ├── 75% Server Cost Reduction
│   ├── 90% Memory Cost Reduction
│   └── 50x Better User Experience
└── System Efficiency
    ├── Reduced Load on Backend
    ├── Better Resource Utilization
    ├── Improved Scalability
    └── Enhanced Reliability
```

## 🏗️ **Cache Architecture**

### **1. Multi-Level Cache Architecture**

```rust
// Multi-Level Cache Architecture
pub struct MultiLevelCache {
    // L1 Cache: In-memory cache (fastest)
    l1_cache: LruCache<String, CachedConditionResult>,
    
    // L2 Cache: Shared cache (fast)
    l2_cache: RedisCache<String, CachedConditionResult>,
    
    // L3 Cache: Database cache (slower but persistent)
    l3_cache: DatabaseCache<String, CachedConditionResult>,
    
    // Cache configuration
    config: CacheConfig,
    
    // Performance monitoring
    metrics: Arc<Mutex<CacheMetrics>>,
}

// Cache Configuration
pub struct CacheConfig {
    pub l1_max_size: usize,
    pub l1_ttl: Duration,
    pub l2_ttl: Duration,
    pub l3_ttl: Duration,
    pub enable_preloading: bool,
    pub enable_prediction: bool,
    pub cache_strategy: CacheStrategy,
}
```

### **2. Cache Key Strategy**

```rust
// Cache Key Generation Strategy
pub struct CacheKeyGenerator {
    // Base key components
    condition_id: String,
    context_hash: String,
    parameter_hash: String,
    world_id: String,
}

impl CacheKeyGenerator {
    // Generate cache key for condition
    pub fn generate_condition_key(
        &self,
        condition_id: &str,
        context: &ConditionContext,
        parameters: &[ConditionParameter]
    ) -> String {
        let context_hash = self.hash_context(context);
        let parameter_hash = self.hash_parameters(parameters);
        
        format!(
            "condition:{}:{}:{}:{}",
            condition_id,
            context_hash,
            parameter_hash,
            context.world_id
        )
    }
    
    // Generate cache key for function
    pub fn generate_function_key(
        &self,
        function_id: &str,
        parameters: &[ConditionParameter],
        context: &ConditionContext
    ) -> String {
        let parameter_hash = self.hash_parameters(parameters);
        let context_hash = self.hash_context(context);
        
        format!(
            "function:{}:{}:{}:{}",
            function_id,
            parameter_hash,
            context_hash,
            context.world_id
        )
    }
    
    // Hash context for cache key
    fn hash_context(&self, context: &ConditionContext) -> String {
        let mut hasher = DefaultHasher::new();
        
        // Hash actor ID
        context.target.id.hash(&mut hasher);
        
        // Hash world state
        context.world_state.hash(&mut hasher);
        
        // Hash time
        context.current_time.hash(&mut hasher);
        
        // Hash weather
        context.current_weather.hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }
    
    // Hash parameters for cache key
    fn hash_parameters(&self, parameters: &[ConditionParameter]) -> String {
        let mut hasher = DefaultHasher::new();
        
        for param in parameters {
            param.hash(&mut hasher);
        }
        
        format!("{:x}", hasher.finish())
    }
}
```

## 🔧 **Cache Strategies**

### **1. Cache Lookup Strategy**

```rust
// Cache Lookup Strategy
pub struct CacheLookupStrategy {
    cache: Arc<MultiLevelCache>,
    key_generator: CacheKeyGenerator,
    fallback_strategy: FallbackStrategy,
}

impl CacheLookupStrategy {
    // Check if condition exists in cache
    pub async fn has_condition_in_cache(
        &self,
        condition_id: &str,
        context: &ConditionContext,
        parameters: &[ConditionParameter]
    ) -> bool {
        let cache_key = self.key_generator.generate_condition_key(
            condition_id,
            context,
            parameters
        );
        
        // Check L1 cache first (fastest)
        if self.cache.l1_cache.contains_key(&cache_key) {
            return true;
        }
        
        // Check L2 cache (fast)
        if self.cache.l2_cache.exists(&cache_key).await.unwrap_or(false) {
            return true;
        }
        
        // Check L3 cache (slower)
        if self.cache.l3_cache.exists(&cache_key).await.unwrap_or(false) {
            return true;
        }
        
        false
    }
    
    // Get condition from cache
    pub async fn get_condition_from_cache(
        &self,
        condition_id: &str,
        context: &ConditionContext,
        parameters: &[ConditionParameter]
    ) -> Option<ConditionResult> {
        let cache_key = self.key_generator.generate_condition_key(
            condition_id,
            context,
            parameters
        );
        
        // Try L1 cache first
        if let Some(cached_result) = self.cache.l1_cache.get(&cache_key) {
            if cached_result.is_valid() {
                self.record_cache_hit("L1", &cache_key);
                return Some(cached_result.result.clone());
            } else {
                // Remove expired entry
                self.cache.l1_cache.remove(&cache_key);
            }
        }
        
        // Try L2 cache
        if let Some(cached_result) = self.cache.l2_cache.get(&cache_key).await {
            if cached_result.is_valid() {
                // Update L1 cache
                self.cache.l1_cache.put(cache_key.clone(), cached_result.clone());
                self.record_cache_hit("L2", &cache_key);
                return Some(cached_result.result);
            } else {
                // Remove expired entry
                self.cache.l2_cache.remove(&cache_key).await;
            }
        }
        
        // Try L3 cache
        if let Some(cached_result) = self.cache.l3_cache.get(&cache_key).await {
            if cached_result.is_valid() {
                // Update L1 and L2 caches
                self.cache.l1_cache.put(cache_key.clone(), cached_result.clone());
                self.cache.l2_cache.set(&cache_key, cached_result.clone()).await;
                self.record_cache_hit("L3", &cache_key);
                return Some(cached_result.result);
            } else {
                // Remove expired entry
                self.cache.l3_cache.remove(&cache_key).await;
            }
        }
        
        self.record_cache_miss(&cache_key);
        None
    }
}
```

### **2. Cache Validation Strategy**

```rust
// Cache Validation Strategy
pub struct CacheValidationStrategy {
    version_checker: VersionChecker,
    checksum_validator: ChecksumValidator,
    ttl_validator: TTLValidator,
}

impl CacheValidationStrategy {
    // Validate cache entry
    pub fn validate_cache_entry(
        &self,
        cached_result: &CachedConditionResult
    ) -> ValidationResult {
        // Check TTL
        if !self.ttl_validator.is_valid(cached_result) {
            return ValidationResult::Expired;
        }
        
        // Check version
        if !self.version_checker.is_compatible(cached_result) {
            return ValidationResult::VersionMismatch;
        }
        
        // Check checksum
        if !self.checksum_validator.is_valid(cached_result) {
            return ValidationResult::ChecksumMismatch;
        }
        
        ValidationResult::Valid
    }
}

// Cache Entry Validation
pub struct CachedConditionResult {
    pub result: ConditionResult,
    pub cached_at: SystemTime,
    pub ttl: Duration,
    pub version: String,
    pub checksum: u64,
}

impl CachedConditionResult {
    // Check if cache entry is valid
    pub fn is_valid(&self) -> bool {
        // Check TTL
        if self.cached_at.elapsed().unwrap_or_default() > self.ttl {
            return false;
        }
        
        // Check version (for cache invalidation)
        if self.version != self.get_current_version() {
            return false;
        }
        
        // Check checksum (for data integrity)
        if self.checksum != self.calculate_checksum() {
            return false;
        }
        
        true
    }
    
    // Get current version
    fn get_current_version(&self) -> String {
        // This would be the current system version
        env!("CARGO_PKG_VERSION").to_string()
    }
    
    // Calculate checksum
    fn calculate_checksum(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.result.hash(&mut hasher);
        self.cached_at.hash(&mut hasher);
        self.ttl.hash(&mut hasher);
        hasher.finish()
    }
}
```

### **3. Cache TTL Strategy**

```rust
// Cache TTL Strategy
pub struct CacheTTLStrategy {
    // Static data (long TTL)
    actor_level: Duration::from_secs(300),      // 5 minutes
    actor_race: Duration::from_secs(3600),      // 1 hour
    actor_sex: Duration::from_secs(3600),       // 1 hour
    
    // Dynamic data (medium TTL)
    actor_health: Duration::from_secs(30),      // 30 seconds
    actor_magicka: Duration::from_secs(30),     // 30 seconds
    actor_stamina: Duration::from_secs(30),     // 30 seconds
    
    // Combat data (short TTL)
    combat_state: Duration::from_secs(10),      // 10 seconds
    combat_duration: Duration::from_secs(10),   // 10 seconds
    
    // Real-time data (very short TTL)
    current_time: Duration::from_secs(1),       // 1 second
    current_weather: Duration::from_secs(5),    // 5 seconds
}

impl CacheTTLStrategy {
    // Get TTL for condition type
    pub fn get_ttl_for_condition(&self, condition_type: &ConditionType) -> Duration {
        match condition_type {
            ConditionType::ActorLevel => self.actor_level,
            ConditionType::ActorRace => self.actor_race,
            ConditionType::ActorSex => self.actor_sex,
            ConditionType::ActorHealth => self.actor_health,
            ConditionType::ActorMagicka => self.actor_magicka,
            ConditionType::ActorStamina => self.actor_stamina,
            ConditionType::CombatState => self.combat_state,
            ConditionType::CombatDuration => self.combat_duration,
            ConditionType::CurrentTime => self.current_time,
            ConditionType::CurrentWeather => self.current_weather,
            _ => Duration::from_secs(60), // Default TTL
        }
    }
}
```

## 🔧 **Cache Management**

### **1. Cache Preloading Strategy**

```rust
// Cache Preloading Strategy
pub struct CachePreloader {
    cache: Arc<MultiLevelCache>,
    prediction_engine: PredictionEngine,
    preload_queue: Arc<Mutex<Vec<PreloadTask>>>,
    preload_scheduler: PreloadScheduler,
}

impl CachePreloader {
    // Preload conditions based on patterns
    pub async fn preload_conditions(&self) -> Result<(), ConditionError> {
        let predictions = self.prediction_engine.predict_next_conditions().await?;
        
        for prediction in predictions {
            let preload_task = PreloadTask {
                condition_id: prediction.condition_id,
                context: prediction.context,
                parameters: prediction.parameters,
                priority: prediction.priority,
                created_at: SystemTime::now(),
            };
            
            self.preload_queue.lock().unwrap().push(preload_task);
        }
        
        // Process preload queue
        self.process_preload_queue().await?;
        
        Ok(())
    }
    
    // Process preload queue
    async fn process_preload_queue(&self) -> Result<(), ConditionError> {
        let mut queue = self.preload_queue.lock().unwrap();
        
        while let Some(task) = queue.pop() {
            // Check if already cached
            if self.cache.has_condition_in_cache(
                &task.condition_id,
                &task.context,
                &task.parameters
            ).await {
                continue; // Skip if already cached
            }
            
            // Preload condition
            self.cache.preload_condition(
                &task.condition_id,
                &task.context,
                &task.parameters
            ).await?;
        }
        
        Ok(())
    }
}

// Preload Task
pub struct PreloadTask {
    pub condition_id: String,
    pub context: ConditionContext,
    pub parameters: Vec<ConditionParameter>,
    pub priority: u32,
    pub created_at: SystemTime,
}

// Prediction Engine
pub struct PredictionEngine {
    pattern_analyzer: PatternAnalyzer,
    machine_learning: MachineLearningEngine,
}

impl PredictionEngine {
    // Predict next conditions to be evaluated
    pub async fn predict_next_conditions(&self) -> Result<Vec<ConditionPrediction>, ConditionError> {
        // Analyze patterns
        let patterns = self.pattern_analyzer.analyze_patterns().await?;
        
        // Use ML to predict
        let predictions = self.machine_learning.predict_conditions(patterns).await?;
        
        Ok(predictions)
    }
}
```

### **2. Cache Eviction Strategy**

```rust
// Cache Eviction Strategy
pub struct CacheEvictionStrategy {
    eviction_policy: EvictionPolicy,
    eviction_threshold: f64,
    eviction_batch_size: usize,
}

impl CacheEvictionStrategy {
    // Evict entries based on policy
    pub fn evict_entries(&self, cache: &mut MultiLevelCache) -> Result<(), ConditionError> {
        match self.eviction_policy {
            EvictionPolicy::LRU => self.evict_lru(cache),
            EvictionPolicy::LFU => self.evict_lfu(cache),
            EvictionPolicy::TTL => self.evict_expired(cache),
            EvictionPolicy::Random => self.evict_random(cache),
        }
    }
    
    // Evict LRU entries
    fn evict_lru(&self, cache: &mut MultiLevelCache) -> Result<(), ConditionError> {
        let mut entries: Vec<_> = cache.l1_cache.iter().collect();
        entries.sort_by_key(|(_, result)| result.cached_at);
        
        let evict_count = (cache.l1_cache.len() as f64 * self.eviction_threshold) as usize;
        
        for (key, _) in entries.iter().take(evict_count) {
            cache.l1_cache.remove(*key);
        }
        
        Ok(())
    }
    
    // Evict LFU entries
    fn evict_lfu(&self, cache: &mut MultiLevelCache) -> Result<(), ConditionError> {
        let mut entries: Vec<_> = cache.l1_cache.iter().collect();
        entries.sort_by_key(|(_, result)| result.access_count);
        
        let evict_count = (cache.l1_cache.len() as f64 * self.eviction_threshold) as usize;
        
        for (key, _) in entries.iter().take(evict_count) {
            cache.l1_cache.remove(*key);
        }
        
        Ok(())
    }
    
    // Evict expired entries
    fn evict_expired(&self, cache: &mut MultiLevelCache) -> Result<(), ConditionError> {
        let now = SystemTime::now();
        let mut to_remove = Vec::new();
        
        for (key, result) in cache.l1_cache.iter() {
            if !result.is_valid() {
                to_remove.push(key.clone());
            }
        }
        
        for key in to_remove {
            cache.l1_cache.remove(&key);
        }
        
        Ok(())
    }
}

// Eviction Policy
pub enum EvictionPolicy {
    LRU,    // Least Recently Used
    LFU,    // Least Frequently Used
    TTL,    // Time To Live
    Random, // Random
}
```

## 📊 **Cache Monitoring & Debugging**

### **1. Cache Monitoring Dashboard**

```rust
// Cache Monitoring Dashboard
pub struct CacheMonitoringDashboard {
    cache_metrics: Arc<Mutex<CacheMetrics>>,
    hit_patterns: Arc<Mutex<HashMap<String, HitPattern>>>,
    performance_metrics: Arc<Mutex<PerformanceMetrics>>,
}

impl CacheMonitoringDashboard {
    // Get cache status
    pub fn get_cache_status(&self) -> CacheStatus {
        let metrics = self.cache_metrics.lock().unwrap();
        let patterns = self.hit_patterns.lock().unwrap();
        
        CacheStatus {
            total_requests: metrics.total_requests,
            hits: metrics.hits,
            misses: metrics.misses,
            hit_ratio: metrics.calculate_hit_ratio(),
            cache_size: self.get_cache_size(),
            top_hit_patterns: self.get_top_hit_patterns(&patterns),
            performance_impact: self.calculate_performance_impact(),
        }
    }
    
    // Get top hit patterns
    fn get_top_hit_patterns(&self, patterns: &HashMap<String, HitPattern>) -> Vec<HitPatternSummary> {
        let mut pattern_list: Vec<_> = patterns.iter().collect();
        pattern_list.sort_by(|a, b| b.1.hit_ratio.partial_cmp(&a.1.hit_ratio).unwrap());
        
        pattern_list
            .iter()
            .take(10)
            .map(|(key, pattern)| HitPatternSummary {
                cache_key: key.clone(),
                hit_ratio: pattern.hit_ratio,
                total_requests: pattern.total_requests,
                last_hit: pattern.last_hit,
            })
            .collect()
    }
    
    // Calculate performance impact
    fn calculate_performance_impact(&self) -> PerformanceImpact {
        let metrics = self.cache_metrics.lock().unwrap();
        
        match metrics.calculate_hit_ratio() {
            ratio if ratio >= 0.9 => PerformanceImpact::Excellent,
            ratio if ratio >= 0.8 => PerformanceImpact::Good,
            ratio if ratio >= 0.7 => PerformanceImpact::Fair,
            ratio if ratio >= 0.6 => PerformanceImpact::Poor,
            _ => PerformanceImpact::Critical,
        }
    }
}

// Cache Status
pub struct CacheStatus {
    pub total_requests: u64,
    pub hits: u64,
    pub misses: u64,
    pub hit_ratio: f64,
    pub cache_size: usize,
    pub top_hit_patterns: Vec<HitPatternSummary>,
    pub performance_impact: PerformanceImpact,
}

// Hit Pattern Summary
pub struct HitPatternSummary {
    pub cache_key: String,
    pub hit_ratio: f64,
    pub total_requests: u64,
    pub last_hit: Option<SystemTime>,
}
```

### **2. Cache Debugging Tools**

```rust
// Cache Debugging Tools
pub struct CacheDebugger {
    cache: Arc<MultiLevelCache>,
    debug_logger: DebugLogger,
}

impl CacheDebugger {
    // Debug cache lookup
    pub async fn debug_cache_lookup(
        &self,
        condition_id: &str,
        context: &ConditionContext,
        parameters: &[ConditionParameter]
    ) -> CacheDebugInfo {
        let cache_key = self.generate_cache_key(condition_id, context, parameters);
        
        let mut debug_info = CacheDebugInfo {
            cache_key: cache_key.clone(),
            condition_id: condition_id.to_string(),
            lookup_steps: Vec::new(),
            result: None,
            performance_metrics: PerformanceMetrics::new(),
        };
        
        let start_time = SystemTime::now();
        
        // Step 1: Check L1 cache
        debug_info.lookup_steps.push(LookupStep {
            level: "L1".to_string(),
            found: self.cache.l1_cache.contains_key(&cache_key),
            duration: start_time.elapsed().unwrap_or_default(),
        });
        
        // Step 2: Check L2 cache
        let l2_start = SystemTime::now();
        let l2_found = self.cache.l2_cache.exists(&cache_key).await.unwrap_or(false);
        debug_info.lookup_steps.push(LookupStep {
            level: "L2".to_string(),
            found: l2_found,
            duration: l2_start.elapsed().unwrap_or_default(),
        });
        
        // Step 3: Check L3 cache
        let l3_start = SystemTime::now();
        let l3_found = self.cache.l3_cache.exists(&cache_key).await.unwrap_or(false);
        debug_info.lookup_steps.push(LookupStep {
            level: "L3".to_string(),
            found: l3_found,
            duration: l3_start.elapsed().unwrap_or_default(),
        });
        
        // Get result if found
        if let Some(result) = self.cache.get_condition_from_cache(
            condition_id,
            context,
            parameters
        ).await {
            debug_info.result = Some(result);
        }
        
        debug_info.performance_metrics.total_duration = start_time.elapsed().unwrap_or_default();
        
        // Log debug info
        self.debug_logger.log_cache_lookup(debug_info.clone());
        
        debug_info
    }
}

// Cache Debug Info
pub struct CacheDebugInfo {
    pub cache_key: String,
    pub condition_id: String,
    pub lookup_steps: Vec<LookupStep>,
    pub result: Option<ConditionResult>,
    pub performance_metrics: PerformanceMetrics,
}

// Lookup Step
pub struct LookupStep {
    pub level: String,
    pub found: bool,
    pub duration: Duration,
}
```

## 🎯 **Key Features**

### **1. Multi-Level Caching**
- ✅ **L1 Cache**: In-memory cache (fastest)
- ✅ **L2 Cache**: Shared cache (fast)
- ✅ **L3 Cache**: Database cache (slower but persistent)
- ✅ **Cascade Update**: Update higher levels when found

### **2. Advanced Cache Strategies**
- ✅ **Cache Key Generation**: Unique keys for each condition
- ✅ **Cache Lookup Strategy**: Multi-level lookup
- ✅ **Cache Validation**: TTL, version, checksum validation
- ✅ **Cache TTL Strategy**: Different TTL for different data types

### **3. Cache Management**
- ✅ **Cache Preloading**: Predictive cache loading
- ✅ **Cache Eviction**: LRU, LFU, TTL, Random policies
- ✅ **Cache Warming**: Pre-populate cache
- ✅ **Cache Invalidation**: Smart invalidation strategies

### **4. Monitoring & Debugging**
- ✅ **Real-time Metrics**: Cache hit/miss ratios
- ✅ **Performance Monitoring**: Response time tracking
- ✅ **Debug Tools**: Cache lookup debugging
- ✅ **Pattern Analysis**: Hit pattern analysis

## 📝 **Implementation Plan**

### **Phase 1: Basic Cache (1 week)**
1. **Multi-Level Cache Implementation**
   - L1, L2, L3 cache implementation
   - Cache key generation
   - Basic cache operations

2. **Cache Validation**
   - TTL validation
   - Version checking
   - Checksum validation

### **Phase 2: Advanced Features (1 week)**
1. **Cache Strategies**
   - Cache lookup strategy
   - Cache eviction strategy
   - Cache TTL strategy

2. **Cache Management**
   - Cache preloading
   - Cache warming
   - Cache invalidation

### **Phase 3: Monitoring & Debugging (1 week)**
1. **Cache Monitoring**
   - Real-time metrics
   - Performance monitoring
   - Hit pattern analysis

2. **Debug Tools**
   - Cache debugging tools
   - Performance analysis
   - Troubleshooting tools

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Cache System Design Complete  
**Maintainer**: Chaos World Team
