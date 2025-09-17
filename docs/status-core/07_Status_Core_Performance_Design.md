# Status Core Performance Design

## 📋 **Tổng Quan**

Status Core Performance Design tập trung vào tối ưu hóa hiệu suất cho Status Core system, bao gồm caching strategies, memory management, concurrency optimization, và performance monitoring.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Performance Principles**
- **High Throughput**: Xử lý hàng nghìn status effects per second
- **Low Latency**: Response time < 1ms cho critical operations
- **Memory Efficiency**: Efficient memory usage và management
- **Scalability**: Horizontal và vertical scaling

### **2. Performance Targets**
- **Throughput**: 10,000+ status effects/second
- **Latency**: < 1ms cho single effect processing
- **Memory Usage**: < 100MB cho 10,000 active effects
- **CPU Usage**: < 50% cho normal operations

### **3. Performance Monitoring**
- **Real-time Metrics**: Real-time performance monitoring
- **Performance Profiling**: Detailed performance profiling
- **Resource Usage**: CPU, memory, và network usage
- **Bottleneck Detection**: Automatic bottleneck detection

## 🏗️ **Performance Architecture**

### **1. Caching System**

```rust
/// High-Performance Status Cache
pub struct StatusCache {
    // Multi-level cache
    l1_cache: L1StatusCache,      // In-memory cache
    l2_cache: L2StatusCache,      // Redis cache
    l3_cache: L3StatusCache,      // Database cache
    
    // Cache configuration
    config: CacheConfig,
    
    // Performance metrics
    metrics: CacheMetrics,
    
    // Cache eviction
    eviction_policy: EvictionPolicy,
}

impl StatusCache {
    /// Get status effect with multi-level caching
    pub async fn get_status_effect(
        &self,
        actor_id: &str,
        effect_id: &str
    ) -> Result<Option<StatusEffectInstance>, StatusError> {
        // Try L1 cache first (fastest)
        if let Some(effect) = self.l1_cache.get(actor_id, effect_id).await? {
            self.metrics.increment_l1_hit();
            return Ok(Some(effect));
        }
        
        // Try L2 cache (Redis)
        if let Some(effect) = self.l2_cache.get(actor_id, effect_id).await? {
            self.metrics.increment_l2_hit();
            // Populate L1 cache
            self.l1_cache.set(actor_id, effect_id, &effect).await?;
            return Ok(Some(effect));
        }
        
        // Try L3 cache (Database)
        if let Some(effect) = self.l3_cache.get(actor_id, effect_id).await? {
            self.metrics.increment_l3_hit();
            // Populate L1 and L2 caches
            self.l1_cache.set(actor_id, effect_id, &effect).await?;
            self.l2_cache.set(actor_id, effect_id, &effect).await?;
            return Ok(Some(effect));
        }
        
        self.metrics.increment_miss();
        Ok(None)
    }
    
    /// Set status effect with multi-level caching
    pub async fn set_status_effect(
        &self,
        actor_id: &str,
        effect_id: &str,
        effect: &StatusEffectInstance
    ) -> Result<(), StatusError> {
        // Set in all cache levels
        self.l1_cache.set(actor_id, effect_id, effect).await?;
        self.l2_cache.set(actor_id, effect_id, effect).await?;
        self.l3_cache.set(actor_id, effect_id, effect).await?;
        
        self.metrics.increment_set();
        Ok(())
    }
    
    /// Batch get status effects
    pub async fn batch_get_status_effects(
        &self,
        requests: Vec<(String, String)>
    ) -> Result<HashMap<String, Option<StatusEffectInstance>>, StatusError> {
        let mut results = HashMap::new();
        
        // Group requests by cache level
        let mut l1_requests = Vec::new();
        let mut l2_requests = Vec::new();
        let mut l3_requests = Vec::new();
        
        for (actor_id, effect_id) in requests {
            // Check L1 cache first
            if let Some(effect) = self.l1_cache.get(&actor_id, &effect_id).await? {
                results.insert(format!("{}:{}", actor_id, effect_id), Some(effect));
                continue;
            }
            
            l2_requests.push((actor_id, effect_id));
        }
        
        // Batch get from L2 cache
        if !l2_requests.is_empty() {
            let l2_results = self.l2_cache.batch_get(&l2_requests).await?;
            for ((actor_id, effect_id), result) in l2_requests.into_iter().zip(l2_results) {
                if let Some(effect) = result {
                    // Populate L1 cache
                    self.l1_cache.set(&actor_id, &effect_id, &effect).await?;
                    results.insert(format!("{}:{}", actor_id, effect_id), Some(effect));
                } else {
                    l3_requests.push((actor_id, effect_id));
                }
            }
        }
        
        // Batch get from L3 cache
        if !l3_requests.is_empty() {
            let l3_results = self.l3_cache.batch_get(&l3_requests).await?;
            for ((actor_id, effect_id), result) in l3_requests.into_iter().zip(l3_results) {
                if let Some(effect) = result {
                    // Populate L1 and L2 caches
                    self.l1_cache.set(&actor_id, &effect_id, &effect).await?;
                    self.l2_cache.set(&actor_id, &effect_id, &effect).await?;
                    results.insert(format!("{}:{}", actor_id, effect_id), Some(effect));
                } else {
                    results.insert(format!("{}:{}", actor_id, effect_id), None);
                }
            }
        }
        
        Ok(results)
    }
}

/// L1 Cache (In-Memory)
pub struct L1StatusCache {
    // Fast hash map for O(1) access
    cache: DashMap<String, StatusEffectInstance>,
    
    // Cache configuration
    max_size: usize,
    ttl: Duration,
    
    // Performance metrics
    hits: AtomicU64,
    misses: AtomicU64,
}

impl L1StatusCache {
    /// Get from L1 cache
    pub async fn get(
        &self,
        actor_id: &str,
        effect_id: &str
    ) -> Result<Option<StatusEffectInstance>, StatusError> {
        let key = format!("{}:{}", actor_id, effect_id);
        
        if let Some(effect) = self.cache.get(&key) {
            // Check TTL
            if effect.applied_at.elapsed().unwrap_or(Duration::MAX) < self.ttl {
                self.hits.fetch_add(1, Ordering::Relaxed);
                return Ok(Some(effect.clone()));
            } else {
                // Remove expired entry
                self.cache.remove(&key);
            }
        }
        
        self.misses.fetch_add(1, Ordering::Relaxed);
        Ok(None)
    }
    
    /// Set in L1 cache
    pub async fn set(
        &self,
        actor_id: &str,
        effect_id: &str,
        effect: &StatusEffectInstance
    ) -> Result<(), StatusError> {
        let key = format!("{}:{}", actor_id, effect_id);
        
        // Check cache size limit
        if self.cache.len() >= self.max_size {
            self.evict_oldest_entries().await?;
        }
        
        self.cache.insert(key, effect.clone());
        Ok(())
    }
    
    /// Evict oldest entries
    async fn evict_oldest_entries(&self) -> Result<(), StatusError> {
        let mut entries: Vec<_> = self.cache.iter().collect();
        entries.sort_by_key(|entry| entry.applied_at);
        
        // Remove oldest 10% of entries
        let to_remove = (entries.len() / 10).max(1);
        for entry in entries.into_iter().take(to_remove) {
            self.cache.remove(entry.key());
        }
        
        Ok(())
    }
}
```

### **2. Memory Management**

```rust
/// High-Performance Memory Pool
pub struct StatusMemoryPool {
    // Object pools for different types
    effect_pool: ObjectPool<StatusEffectInstance>,
    immunity_pool: ObjectPool<ImmunityInstance>,
    result_pool: ObjectPool<StatusEffectResult>,
    context_pool: ObjectPool<StatusContext>,
    
    // Memory configuration
    config: MemoryPoolConfig,
    
    // Memory metrics
    metrics: MemoryMetrics,
    
    // Garbage collection
    gc_scheduler: GarbageCollector,
}

impl StatusMemoryPool {
    /// Get effect instance from pool
    pub fn get_effect_instance(&self) -> PooledObject<StatusEffectInstance> {
        self.effect_pool.get()
    }
    
    /// Return effect instance to pool
    pub fn return_effect_instance(&self, instance: PooledObject<StatusEffectInstance>) {
        // Reset instance before returning
        instance.reset();
        self.effect_pool.return_object(instance);
    }
    
    /// Batch get effect instances
    pub fn batch_get_effect_instances(
        &self,
        count: usize
    ) -> Vec<PooledObject<StatusEffectInstance>> {
        (0..count)
            .map(|_| self.get_effect_instance())
            .collect()
    }
    
    /// Batch return effect instances
    pub fn batch_return_effect_instances(
        &self,
        instances: Vec<PooledObject<StatusEffectInstance>>
    ) {
        for instance in instances {
            self.return_effect_instance(instance);
        }
    }
    
    /// Get memory usage statistics
    pub fn get_memory_usage(&self) -> MemoryUsage {
        MemoryUsage {
            total_allocated: self.effect_pool.get_total_allocated(),
            total_used: self.effect_pool.get_total_used(),
            total_free: self.effect_pool.get_total_free(),
            allocation_count: self.effect_pool.get_allocation_count(),
            deallocation_count: self.effect_pool.get_deallocation_count(),
        }
    }
}

/// Object Pool for efficient memory management
pub struct ObjectPool<T> {
    // Pool of objects
    pool: Arc<Mutex<Vec<T>>>,
    
    // Factory function
    factory: Box<dyn Fn() -> T + Send + Sync>,
    
    // Pool configuration
    initial_size: usize,
    max_size: usize,
    grow_size: usize,
    
    // Statistics
    total_allocated: AtomicUsize,
    total_used: AtomicUsize,
    allocation_count: AtomicU64,
    deallocation_count: AtomicU64,
}

impl<T> ObjectPool<T> {
    /// Get object from pool
    pub fn get(&self) -> PooledObject<T> {
        let mut pool = self.pool.lock().unwrap();
        
        if let Some(obj) = pool.pop() {
            self.total_used.fetch_add(1, Ordering::Relaxed);
            self.allocation_count.fetch_add(1, Ordering::Relaxed);
            PooledObject::new(obj, self.clone())
        } else {
            // Pool is empty, create new object
            let obj = (self.factory)();
            self.total_allocated.fetch_add(1, Ordering::Relaxed);
            self.total_used.fetch_add(1, Ordering::Relaxed);
            self.allocation_count.fetch_add(1, Ordering::Relaxed);
            PooledObject::new(obj, self.clone())
        }
    }
    
    /// Return object to pool
    pub fn return_object(&self, mut pooled_obj: PooledObject<T>) {
        let obj = pooled_obj.take();
        let mut pool = self.pool.lock().unwrap();
        
        if pool.len() < self.max_size {
            pool.push(obj);
            self.total_used.fetch_sub(1, Ordering::Relaxed);
            self.deallocation_count.fetch_add(1, Ordering::Relaxed);
        } else {
            // Pool is full, drop object
            drop(obj);
            self.total_allocated.fetch_sub(1, Ordering::Relaxed);
            self.total_used.fetch_sub(1, Ordering::Relaxed);
            self.deallocation_count.fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// Pooled Object wrapper
pub struct PooledObject<T> {
    obj: Option<T>,
    pool: ObjectPool<T>,
}

impl<T> PooledObject<T> {
    fn new(obj: T, pool: ObjectPool<T>) -> Self {
        Self {
            obj: Some(obj),
            pool,
        }
    }
    
    fn take(&mut self) -> T {
        self.obj.take().unwrap()
    }
    
    fn reset(&mut self) {
        // Reset object to initial state
        if let Some(ref mut obj) = self.obj {
            // Implement reset logic for specific types
            // This would be implemented for each specific type
        }
    }
}

impl<T> Drop for PooledObject<T> {
    fn drop(&mut self) {
        if let Some(obj) = self.obj.take() {
            self.pool.return_object(PooledObject::new(obj, self.pool.clone()));
        }
    }
}
```

### **3. Concurrency Optimization**

```rust
/// High-Performance Status Processor
pub struct StatusProcessor {
    // Thread pool for parallel processing
    thread_pool: ThreadPool,
    
    // Work queues for different priorities
    high_priority_queue: Arc<Mutex<VecDeque<StatusEffectTask>>>,
    normal_priority_queue: Arc<Mutex<VecDeque<StatusEffectTask>>>,
    low_priority_queue: Arc<Mutex<VecDeque<StatusEffectTask>>>,
    
    // Batch processor for efficient processing
    batch_processor: BatchProcessor,
    
    // Performance metrics
    metrics: ProcessingMetrics,
    
    // Configuration
    config: ProcessingConfig,
}

impl StatusProcessor {
    /// Process status effect with priority
    pub async fn process_status_effect(
        &self,
        effect: StatusEffectInstance,
        priority: ProcessingPriority,
        context: &StatusContext
    ) -> Result<StatusEffectResult, StatusError> {
        let task = StatusEffectTask {
            effect,
            context: context.clone(),
            priority,
            created_at: Instant::now(),
        };
        
        // Add to appropriate queue
        match priority {
            ProcessingPriority::High => {
                self.high_priority_queue.lock().unwrap().push_back(task);
            },
            ProcessingPriority::Normal => {
                self.normal_priority_queue.lock().unwrap().push_back(task);
            },
            ProcessingPriority::Low => {
                self.low_priority_queue.lock().unwrap().push_back(task);
            },
        }
        
        // Process task
        self.process_next_task().await
    }
    
    /// Process next task from queues
    async fn process_next_task(&self) -> Result<StatusEffectResult, StatusError> {
        // Try high priority queue first
        if let Some(task) = self.high_priority_queue.lock().unwrap().pop_front() {
            return self.process_task(task).await;
        }
        
        // Try normal priority queue
        if let Some(task) = self.normal_priority_queue.lock().unwrap().pop_front() {
            return self.process_task(task).await;
        }
        
        // Try low priority queue
        if let Some(task) = self.low_priority_queue.lock().unwrap().pop_front() {
            return self.process_task(task).await;
        }
        
        Err(StatusError::NoTasksAvailable)
    }
    
    /// Process individual task
    async fn process_task(
        &self,
        task: StatusEffectTask
    ) -> Result<StatusEffectResult, StatusError> {
        let start_time = Instant::now();
        
        // Process effect
        let result = self.batch_processor.process_effect(&task.effect, &task.context).await?;
        
        // Update metrics
        let processing_time = start_time.elapsed();
        self.metrics.record_processing_time(processing_time);
        self.metrics.increment_processed_effects();
        
        Ok(result)
    }
    
    /// Batch process multiple effects
    pub async fn batch_process_effects(
        &self,
        effects: Vec<StatusEffectInstance>,
        context: &StatusContext
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        let start_time = Instant::now();
        
        // Group effects by type for efficient processing
        let mut grouped_effects: HashMap<String, Vec<StatusEffectInstance>> = HashMap::new();
        for effect in effects {
            grouped_effects
                .entry(effect.effect_id.clone())
                .or_insert_with(Vec::new)
                .push(effect);
        }
        
        // Process each group in parallel
        let mut handles = Vec::new();
        for (effect_id, group_effects) in grouped_effects {
            let processor = self.batch_processor.clone();
            let context = context.clone();
            let handle = tokio::spawn(async move {
                processor.process_effect_group(effect_id, group_effects, &context).await
            });
            handles.push(handle);
        }
        
        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            let group_results = handle.await??;
            results.extend(group_results);
        }
        
        // Update metrics
        let processing_time = start_time.elapsed();
        self.metrics.record_batch_processing_time(processing_time);
        self.metrics.increment_processed_batches();
        
        Ok(results)
    }
}

/// Batch Processor for efficient processing
pub struct BatchProcessor {
    // Effect processors by type
    effect_processors: HashMap<String, Box<dyn StatusEffectProcessor>>,
    
    // Batch configuration
    config: BatchConfig,
    
    // Performance metrics
    metrics: BatchMetrics,
}

impl BatchProcessor {
    /// Process effect group
    pub async fn process_effect_group(
        &self,
        effect_id: String,
        effects: Vec<StatusEffectInstance>,
        context: &StatusContext
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        if let Some(processor) = self.effect_processors.get(&effect_id) {
            processor.process_batch(effects, context).await
        } else {
            // Use default processor
            self.process_with_default_processor(effects, context).await
        }
    }
    
    /// Process with default processor
    async fn process_with_default_processor(
        &self,
        effects: Vec<StatusEffectInstance>,
        context: &StatusContext
    ) -> Result<Vec<StatusEffectResult>, StatusError> {
        let mut results = Vec::new();
        
        for effect in effects {
            let result = self.process_effect(&effect, context).await?;
            results.push(result);
        }
        
        Ok(results)
    }
}

/// Status Effect Task
#[derive(Debug, Clone)]
pub struct StatusEffectTask {
    pub effect: StatusEffectInstance,
    pub context: StatusContext,
    pub priority: ProcessingPriority,
    pub created_at: Instant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessingPriority {
    High,
    Normal,
    Low,
}
```

### **4. Performance Monitoring**

```rust
/// Performance Monitor
pub struct PerformanceMonitor {
    // Performance metrics
    metrics: Arc<Mutex<PerformanceMetrics>>,
    
    // Real-time monitoring
    real_time_monitor: RealTimeMonitor,
    
    // Performance profiler
    profiler: PerformanceProfiler,
    
    // Alert system
    alert_system: AlertSystem,
    
    // Configuration
    config: MonitoringConfig,
}

impl PerformanceMonitor {
    /// Record performance metric
    pub fn record_metric(&self, metric: PerformanceMetric) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.record_metric(metric);
        
        // Check for alerts
        if let Some(alert) = self.check_alert_conditions(&metric) {
            self.alert_system.send_alert(alert).await;
        }
    }
    
    /// Get performance summary
    pub fn get_performance_summary(&self) -> PerformanceSummary {
        let metrics = self.metrics.lock().unwrap();
        PerformanceSummary {
            throughput: metrics.get_throughput(),
            latency: metrics.get_latency(),
            memory_usage: metrics.get_memory_usage(),
            cpu_usage: metrics.get_cpu_usage(),
            error_rate: metrics.get_error_rate(),
            cache_hit_rate: metrics.get_cache_hit_rate(),
        }
    }
    
    /// Start performance profiling
    pub fn start_profiling(&self, session_id: &str) -> Result<(), StatusError> {
        self.profiler.start_session(session_id)?;
        Ok(())
    }
    
    /// Stop performance profiling
    pub fn stop_profiling(&self, session_id: &str) -> Result<ProfileReport, StatusError> {
        self.profiler.stop_session(session_id)
    }
}

/// Performance Metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    // Throughput metrics
    requests_per_second: AtomicU64,
    effects_processed_per_second: AtomicU64,
    
    // Latency metrics
    average_latency: AtomicU64,
    p95_latency: AtomicU64,
    p99_latency: AtomicU64,
    
    // Memory metrics
    memory_usage: AtomicU64,
    memory_peak: AtomicU64,
    
    // CPU metrics
    cpu_usage: AtomicU64,
    
    // Error metrics
    error_count: AtomicU64,
    total_requests: AtomicU64,
    
    // Cache metrics
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
}

impl PerformanceMetrics {
    /// Record metric
    pub fn record_metric(&mut self, metric: PerformanceMetric) {
        match metric {
            PerformanceMetric::RequestProcessed => {
                self.requests_per_second.fetch_add(1, Ordering::Relaxed);
                self.total_requests.fetch_add(1, Ordering::Relaxed);
            },
            PerformanceMetric::EffectProcessed => {
                self.effects_processed_per_second.fetch_add(1, Ordering::Relaxed);
            },
            PerformanceMetric::Latency(latency) => {
                self.update_latency_metrics(latency);
            },
            PerformanceMetric::MemoryUsage(usage) => {
                self.memory_usage.store(usage, Ordering::Relaxed);
                if usage > self.memory_peak.load(Ordering::Relaxed) {
                    self.memory_peak.store(usage, Ordering::Relaxed);
                }
            },
            PerformanceMetric::CpuUsage(usage) => {
                self.cpu_usage.store(usage, Ordering::Relaxed);
            },
            PerformanceMetric::Error => {
                self.error_count.fetch_add(1, Ordering::Relaxed);
            },
            PerformanceMetric::CacheHit => {
                self.cache_hits.fetch_add(1, Ordering::Relaxed);
            },
            PerformanceMetric::CacheMiss => {
                self.cache_misses.fetch_add(1, Ordering::Relaxed);
            },
        }
    }
    
    /// Update latency metrics
    fn update_latency_metrics(&self, latency: Duration) {
        let latency_ms = latency.as_millis() as u64;
        
        // Update average latency (simple moving average)
        let current_avg = self.average_latency.load(Ordering::Relaxed);
        let new_avg = (current_avg + latency_ms) / 2;
        self.average_latency.store(new_avg, Ordering::Relaxed);
        
        // Update percentiles (simplified)
        if latency_ms > self.p95_latency.load(Ordering::Relaxed) {
            self.p95_latency.store(latency_ms, Ordering::Relaxed);
        }
        if latency_ms > self.p99_latency.load(Ordering::Relaxed) {
            self.p99_latency.store(latency_ms, Ordering::Relaxed);
        }
    }
}

#[derive(Debug, Clone)]
pub enum PerformanceMetric {
    RequestProcessed,
    EffectProcessed,
    Latency(Duration),
    MemoryUsage(u64),
    CpuUsage(u64),
    Error,
    CacheHit,
    CacheMiss,
}
```

## 🚀 **Performance Optimization Strategies**

### **1. Caching Strategies**

```rust
/// Cache Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    // L1 Cache (In-Memory)
    l1_cache_size: usize,
    l1_cache_ttl: Duration,
    
    // L2 Cache (Redis)
    l2_cache_enabled: bool,
    l2_cache_ttl: Duration,
    l2_cache_host: String,
    l2_cache_port: u16,
    
    // L3 Cache (Database)
    l3_cache_enabled: bool,
    l3_cache_ttl: Duration,
    
    // Eviction policy
    eviction_policy: EvictionPolicy,
    max_memory_usage: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvictionPolicy {
    LRU,        // Least Recently Used
    LFU,        // Least Frequently Used
    TTL,        // Time To Live
    Random,     // Random
}
```

### **2. Memory Optimization**

```rust
/// Memory Pool Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPoolConfig {
    // Pool sizes
    effect_pool_size: usize,
    immunity_pool_size: usize,
    result_pool_size: usize,
    context_pool_size: usize,
    
    // Growth settings
    initial_size: usize,
    max_size: usize,
    grow_size: usize,
    
    // Garbage collection
    gc_interval: Duration,
    gc_threshold: f64,
}

/// Garbage Collector
pub struct GarbageCollector {
    // GC configuration
    config: GCConfig,
    
    // GC metrics
    metrics: GCMetrics,
    
    // GC scheduler
    scheduler: GCScheduler,
}

impl GarbageCollector {
    /// Run garbage collection
    pub async fn run_gc(&self) -> Result<GCResult, StatusError> {
        let start_time = Instant::now();
        
        // Collect garbage from all pools
        let effect_gc = self.collect_effect_pool().await?;
        let immunity_gc = self.collect_immunity_pool().await?;
        let result_gc = self.collect_result_pool().await?;
        let context_gc = self.collect_context_pool().await?;
        
        let total_objects_collected = effect_gc.objects_collected
            + immunity_gc.objects_collected
            + result_gc.objects_collected
            + context_gc.objects_collected;
        
        let total_memory_freed = effect_gc.memory_freed
            + immunity_gc.memory_freed
            + result_gc.memory_freed
            + context_gc.memory_freed;
        
        let gc_time = start_time.elapsed();
        
        // Update metrics
        self.metrics.record_gc_run(gc_time, total_objects_collected, total_memory_freed);
        
        Ok(GCResult {
            objects_collected: total_objects_collected,
            memory_freed: total_memory_freed,
            gc_time,
        })
    }
}
```

### **3. Concurrency Optimization**

```rust
/// Thread Pool Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolConfig {
    // Thread pool sizes
    core_threads: usize,
    max_threads: usize,
    keep_alive_time: Duration,
    
    // Queue settings
    queue_capacity: usize,
    queue_type: QueueType,
    
    // Performance settings
    enable_work_stealing: bool,
    enable_priority_queues: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueueType {
    Bounded,
    Unbounded,
    Priority,
}
```

## 🧪 **Performance Testing**

### **1. Load Testing**

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_high_throughput() {
        let processor = StatusProcessor::new(create_high_performance_config()).await?;
        let context = create_test_status_context();
        
        // Test 10,000 effects per second
        let start_time = Instant::now();
        let mut handles = Vec::new();
        
        for i in 0..10000 {
            let effect = create_test_status_effect_with_id(&format!("effect_{}", i));
            let processor = processor.clone();
            let context = context.clone();
            
            let handle = tokio::spawn(async move {
                processor.process_status_effect(effect, ProcessingPriority::Normal, &context).await
            });
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        for handle in handles {
            handle.await??;
        }
        
        let total_time = start_time.elapsed();
        let throughput = 10000.0 / total_time.as_secs_f64();
        
        assert!(throughput >= 10000.0, "Throughput {} is below target 10000", throughput);
    }
    
    #[tokio::test]
    async fn test_low_latency() {
        let processor = StatusProcessor::new(create_low_latency_config()).await?;
        let context = create_test_status_context();
        let effect = create_test_status_effect();
        
        let start_time = Instant::now();
        let result = processor.process_status_effect(effect, ProcessingPriority::High, &context).await?;
        let latency = start_time.elapsed();
        
        assert!(latency.as_millis() < 1, "Latency {}ms is above target 1ms", latency.as_millis());
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_memory_efficiency() {
        let memory_pool = StatusMemoryPool::new(create_memory_efficient_config()).await?;
        
        // Allocate 10,000 effect instances
        let mut instances = Vec::new();
        for _ in 0..10000 {
            instances.push(memory_pool.get_effect_instance());
        }
        
        let memory_usage = memory_pool.get_memory_usage();
        assert!(memory_usage.total_used <= 100000000, "Memory usage {} is above target 100MB", memory_usage.total_used);
        
        // Return instances to pool
        memory_pool.batch_return_effect_instances(instances);
        
        // Check that memory was freed
        let memory_usage_after = memory_pool.get_memory_usage();
        assert!(memory_usage_after.total_used < memory_usage.total_used);
    }
}
```

### **2. Benchmark Testing**

```rust
#[tokio::test]
async fn benchmark_status_effect_processing() {
    let processor = StatusProcessor::new(create_benchmark_config()).await?;
    let context = create_test_status_context();
    
    // Benchmark single effect processing
    let mut latencies = Vec::new();
    for _ in 0..1000 {
        let effect = create_test_status_effect();
        let start_time = Instant::now();
        let _result = processor.process_status_effect(effect, ProcessingPriority::Normal, &context).await?;
        let latency = start_time.elapsed();
        latencies.push(latency);
    }
    
    // Calculate statistics
    latencies.sort();
    let p50 = latencies[latencies.len() / 2];
    let p95 = latencies[(latencies.len() * 95) / 100];
    let p99 = latencies[(latencies.len() * 99) / 100];
    
    println!("Latency Statistics:");
    println!("  P50: {:?}", p50);
    println!("  P95: {:?}", p95);
    println!("  P99: {:?}", p99);
    
    // Verify performance targets
    assert!(p50.as_micros() < 1000, "P50 latency {}μs is above target 1000μs", p50.as_micros());
    assert!(p95.as_micros() < 2000, "P95 latency {}μs is above target 2000μs", p95.as_micros());
    assert!(p99.as_micros() < 5000, "P99 latency {}μs is above target 5000μs", p99.as_micros());
}
```

## 📝 **Implementation Notes**

### **1. Performance Targets**
- **Throughput**: 10,000+ status effects/second
- **Latency**: < 1ms cho single effect processing
- **Memory Usage**: < 100MB cho 10,000 active effects
- **CPU Usage**: < 50% cho normal operations

### **2. Optimization Strategies**
- **Multi-level Caching**: L1 (memory), L2 (Redis), L3 (database)
- **Object Pooling**: Efficient memory management
- **Batch Processing**: Process multiple effects efficiently
- **Priority Queues**: Process high-priority effects first

### **3. Monitoring & Alerting**
- **Real-time Metrics**: Monitor performance in real-time
- **Performance Profiling**: Detailed performance analysis
- **Alert System**: Automatic alerts for performance issues
- **Resource Usage**: Monitor CPU, memory, và network usage

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
