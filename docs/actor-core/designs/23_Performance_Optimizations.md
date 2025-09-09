# 23 — Performance Optimizations for Actor Core v3

**Updated:** 2025-09-08 00:15

This document outlines performance optimization strategies for Actor Core v3, specifically designed for games with many subsystems where performance is critical.

## Overview

Actor Core v3 can be optimized to perform **better than v2** in most scenarios, especially when dealing with multiple subsystems. The key is implementing the right caching, lazy evaluation, and parallel processing strategies.

## 1. Caching Strategy (Critical)

### 1.1 L1 Cache - In-Memory
```go
type AggregatorCache struct {
    // Cache snapshots by actor ID + version
    snapshots map[string]*Snapshot
    
    // Cache effective caps by layer + dimension
    capsCache map[string]map[string]EffectiveCaps
    
    // Cache merge rules
    rulesCache map[string]MergeRule
    
    mu sync.RWMutex
}
```

### 1.2 L2 Cache - Redis/Distributed
```go
// Cache cross-server for multiplayer
type DistributedCache struct {
    redis *redis.Client
    ttl   time.Duration
}
```

### 1.3 Cache Invalidation Strategy
- **Version-based**: Invalidate when actor version changes
- **Subsystem-based**: Invalidate when specific subsystems change
- **TTL-based**: Automatic expiration for memory management

## 2. Lazy Evaluation & Incremental Updates

### 2.1 Dirty Tracking
```go
type Actor struct {
    // ... existing fields
    DirtySubsystems map[string]bool  // Track which subsystems changed
    LastSnapshot    *Snapshot
    LastVersion     int64
}

// Only re-compute when changes detected
func (a *Actor) IsDirty() bool {
    return a.Version > a.LastVersion
}
```

### 2.2 Incremental Caps Computation
```go
// Only re-compute caps when affected subsystems change
func (cp *CapsProvider) GetEffectiveCaps(actor *Actor, dirtySubsystems []string) EffectiveCaps {
    if len(dirtySubsystems) == 0 {
        return cp.cachedCaps[actor.ID] // Return cached
    }
    // Re-compute only affected dimensions
}
```

## 3. Pre-computation & Batching

### 3.1 Pre-computed Lookups
```go
type OptimizedAggregator struct {
    // Pre-sorted contribution buckets
    flatContributions   map[string][]Contribution
    multContributions   map[string][]Contribution
    postContributions   map[string][]Contribution
    overrideContributions map[string][]Contribution
    
    // Pre-computed layer orders
    layerOrder []string
}
```

### 3.2 Batch Processing
```go
// Process multiple actors simultaneously
func (a *Aggregator) ResolveBatch(actors []*Actor) []*Snapshot {
    // Group by similar subsystems
    // Process in parallel
    // Reuse computed caps
}
```

## 4. Memory Pool & Zero Allocations

### 4.1 Object Pooling
```go
var (
    contributionPool = sync.Pool{
        New: func() interface{} {
            return make([]Contribution, 0, 100)
        },
    }
    
    snapshotPool = sync.Pool{
        New: func() interface{} {
            return &Snapshot{
                Primary:  make(map[string]float64),
                Derived:  make(map[string]float64),
                CapsUsed: make(map[string]Caps),
            }
        },
    }
)
```

### 4.2 Zero-Copy Operations
```go
// Reuse slices, avoid allocations
func (a *Aggregator) resolveDimension(dimension string, contributions []Contribution) float64 {
    // Reuse pre-allocated slices
    flat := a.tempFlat[:0]
    mult := a.tempMult[:0]
    // ... process without allocations
}
```

## 5. Parallel Processing

### 5.1 Goroutine Pools
```go
type ParallelAggregator struct {
    workerPool chan struct{} // Limit concurrent workers
    wg         sync.WaitGroup
}

func (pa *ParallelAggregator) ProcessSubsystems(actor *Actor) {
    // Process subsystems in parallel
    for _, subsystem := range actor.Subsystems {
        pa.wg.Add(1)
        go func(s Subsystem) {
            defer pa.wg.Done()
            // Process subsystem
        }(subsystem)
    }
    pa.wg.Wait()
}
```

### 5.2 Dimension-Level Parallelism
```go
// Process different dimensions in parallel
func (a *Aggregator) resolveDimensionsParallel(dimensions []string) map[string]float64 {
    results := make(map[string]float64)
    var mu sync.Mutex
    var wg sync.WaitGroup
    
    for _, dim := range dimensions {
        wg.Add(1)
        go func(dimension string) {
            defer wg.Done()
            result := a.resolveDimension(dimension, contributions)
            mu.Lock()
            results[dimension] = result
            mu.Unlock()
        }(dim)
    }
    wg.Wait()
    return results
}
```

## 6. Smart Registry Caching

### 6.1 Compiled Rules
```go
type CompiledRegistry struct {
    // Pre-compile rules to avoid YAML parsing
    rules map[string]CompiledRule
    
    // Pre-sort dimensions by frequency
    dimensionOrder []string
}

type CompiledRule struct {
    UsePipeline    bool
    Operator       string
    ClampDefault   Caps
    // Pre-computed values
}
```

### 6.2 Hot Path Optimization
```go
// Cache frequently accessed dimensions
type HotPathCache struct {
    hotDimensions map[string]bool
    precomputed   map[string]CompiledRule
}
```

## 7. Performance Monitoring

### 7.1 Metrics Collection
```go
type PerformanceMonitor struct {
    // Track hot paths
    aggregationTime time.Duration
    capsTime        time.Duration
    cacheHitRate    float64
    
    // Alert on performance degradation
    thresholds map[string]time.Duration
}
```

### 7.2 Real-time Profiling
```go
// Built-in profiling hooks
func (a *Aggregator) resolveWithProfiling(actor *Actor) *Snapshot {
    start := time.Now()
    defer func() {
        a.metrics.RecordAggregationTime(time.Since(start))
    }()
    
    return a.resolve(actor)
}
```

## 8. Configuration Tuning

### 8.1 Performance-First Config
```yaml
# registry.yaml
performance:
  cache_ttl: 300s
  max_workers: 8
  batch_size: 100
  enable_profiling: true
  
dimensions:
  # High-frequency dimensions first
  strength: { use_pipeline: true, clamp_default: { min: 0, max: 999999 } }
  hp_max: { use_pipeline: true, clamp_default: { min: 1, max: 2000000 } }
  # ... other dimensions
```

### 8.2 Memory Management
```yaml
memory:
  max_cache_size: 10000
  gc_threshold: 0.8
  pool_size: 1000
```

## 9. Benchmarking & Testing

### 9.1 Performance Test Suite
```go
func BenchmarkAggregator_Resolve(b *testing.B) {
    // Test with 1000+ subsystems
    // Test with 10000+ actors
    // Test concurrent access
}

func BenchmarkCapsProvider_EffectiveCaps(b *testing.B) {
    // Test layered caps performance
}

func BenchmarkCache_HitRate(b *testing.B) {
    // Test cache effectiveness
}
```

### 9.2 Load Testing
```go
// Simulate real game load
func TestLoadPerformance(t *testing.T) {
    actors := generateTestActors(10000)
    subsystems := generateTestSubsystems(100)
    
    start := time.Now()
    for _, actor := range actors {
        aggregator.Resolve(actor)
    }
    duration := time.Since(start)
    
    assert.Less(t, duration, 100*time.Millisecond)
}
```

## 10. Implementation Priority

### Phase 1: Core Optimizations (Immediate Impact)
1. ✅ **Caching** - L1 cache for snapshots
2. ✅ **Dirty tracking** - Only re-compute when needed
3. ✅ **Object pooling** - Reduce allocations

### Phase 2: Advanced Optimizations (2-5x Performance)
4. ✅ **Pre-computation** - Pre-sort, pre-compile
5. ✅ **Parallel processing** - Goroutine pools
6. ✅ **Memory optimization** - Zero-copy operations

### Phase 3: Production Optimizations (10x+ Performance)
7. ✅ **Distributed caching** - Redis L2 cache
8. ✅ **Batch processing** - Multiple actors
9. ✅ **Performance monitoring** - Real-time metrics

## 11. Expected Performance Gains

| Optimization | Performance Gain | Memory Impact |
|-------------|------------------|---------------|
| **Caching** | 10-100x faster | +20% memory |
| **Dirty tracking** | 5-20x faster | +10% memory |
| **Object pooling** | 2-5x faster | -50% GC pressure |
| **Parallel processing** | 2-8x faster | +30% memory |
| **Pre-computation** | 2-3x faster | +15% memory |

## 12. Performance vs Flexibility Trade-offs

### High Performance Mode
```yaml
performance_mode: "high"
caching: "aggressive"
parallelism: "maximum"
memory_pooling: true
```

### Balanced Mode (Recommended)
```yaml
performance_mode: "balanced"
caching: "moderate"
parallelism: "adaptive"
memory_pooling: true
```

### Maximum Flexibility Mode
```yaml
performance_mode: "flexible"
caching: "minimal"
parallelism: "sequential"
memory_pooling: false
```

## 13. Common Performance Pitfalls

### ❌ Avoid These
- Computing caps for every dimension on every update
- Not using dirty tracking for incremental updates
- Allocating new objects in hot paths
- Processing subsystems sequentially when they're independent
- Not caching frequently accessed data

### ✅ Best Practices
- Cache everything that can be cached
- Use dirty tracking for incremental updates
- Pool objects to reduce GC pressure
- Process independent operations in parallel
- Profile and measure before optimizing

## 14. Monitoring & Alerting

### Key Metrics to Track
- **Aggregation time per actor**
- **Cache hit rate**
- **Memory usage and GC frequency**
- **Number of active subsystems**
- **Caps computation time**

### Alert Thresholds
```yaml
alerts:
  aggregation_time: "> 10ms"
  cache_hit_rate: "< 80%"
  memory_usage: "> 80%"
  gc_frequency: "> 1/second"
```

## 15. Conclusion

Actor Core v3 can be optimized to **outperform v2** in most scenarios, especially with multiple subsystems. The key is implementing the right combination of:

1. **Intelligent caching** (biggest impact)
2. **Lazy evaluation** (incremental updates)
3. **Parallel processing** (multi-core utilization)
4. **Memory optimization** (reduced GC pressure)

Start with Phase 1 optimizations for immediate impact, then gradually implement Phase 2 and 3 for maximum performance.

---

**Next Steps:**
1. Implement basic caching and dirty tracking
2. Add performance benchmarks
3. Profile real workloads
4. Iterate based on measurements
