# 12 — Performance & Concurrency

**Updated:** 2025-09-08 00:40

This document outlines performance considerations and concurrency patterns for Actor Core v3.

## Performance Considerations

### Algorithmic Complexity
- **Sorting**: O(N log N) for contribution ordering
- **Caps Merging**: O(M) where M is number of cap contributions
- **Aggregation**: O(D) where D is number of dimensions
- **Overall**: O(N log N + M + D) per actor resolution

### Optimization Strategies
- **Local Reduction**: Process caps within layers before across-layer reduction
- **Caching**: Cache by (actor version, caps version, registry version)
- **Pre-computation**: Pre-sort contributions, pre-compile rules
- **Parallel Processing**: Process independent subsystems concurrently

## Concurrency Patterns

### Thread Safety
```go
type ThreadSafeAggregator struct {
    mu       sync.RWMutex
    cache    map[string]*Snapshot
    registry *CombinerRegistry
}

func (a *ThreadSafeAggregator) Resolve(actor *Actor) (*Snapshot, error) {
    a.mu.RLock()
    if cached := a.cache[actor.ID]; cached != nil {
        a.mu.RUnlock()
        return cached, nil
    }
    a.mu.RUnlock()
    
    // Compute snapshot
    snapshot, err := a.computeSnapshot(actor)
    if err != nil {
        return nil, err
    }
    
    a.mu.Lock()
    a.cache[actor.ID] = snapshot
    a.mu.Unlock()
    
    return snapshot, nil
}
```

### Parallel Subsystem Processing
```go
func (a *Aggregator) processSubsystemsParallel(actor *Actor) ([]SubsystemOutput, error) {
    var wg sync.WaitGroup
    outputs := make([]SubsystemOutput, len(actor.Subsystems))
    errors := make([]error, len(actor.Subsystems))
    
    for i, subsystem := range actor.Subsystems {
        wg.Add(1)
        go func(idx int, sub Subsystem) {
            defer wg.Done()
            output, err := sub.Contribute(context.Background(), actor)
            outputs[idx] = output
            errors[idx] = err
        }(i, subsystem)
    }
    
    wg.Wait()
    
    // Check for errors
    for _, err := range errors {
        if err != nil {
            return nil, err
        }
    }
    
    return outputs, nil
}
```

### Cancellation Support
```go
func (a *Aggregator) ResolveWithContext(ctx context.Context, actor *Actor) (*Snapshot, error) {
    // Check for cancellation
    select {
    case <-ctx.Done():
        return nil, ctx.Err()
    default:
    }
    
    // Process with cancellation propagation
    return a.resolve(ctx, actor)
}
```

## Memory Management

### Object Pooling
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

func (a *Aggregator) getSnapshot() *Snapshot {
    snapshot := snapshotPool.Get().(*Snapshot)
    // Reset snapshot
    for k := range snapshot.Primary {
        delete(snapshot.Primary, k)
    }
    return snapshot
}
```

### Cache Management
```go
type CacheManager struct {
    cache      map[string]*Snapshot
    maxSize    int
    evictionPolicy string
    mu         sync.RWMutex
}

func (cm *CacheManager) Get(key string) (*Snapshot, bool) {
    cm.mu.RLock()
    defer cm.mu.RUnlock()
    
    snapshot, exists := cm.cache[key]
    return snapshot, exists
}

func (cm *CacheManager) Set(key string, snapshot *Snapshot) {
    cm.mu.Lock()
    defer cm.mu.Unlock()
    
    if len(cm.cache) >= cm.maxSize {
        cm.evict()
    }
    
    cm.cache[key] = snapshot
}
```

## Performance Monitoring

### Metrics Collection
```go
type PerformanceMetrics struct {
    AggregationTime    time.Duration
    CapsTime          time.Duration
    SubsystemTime     time.Duration
    CacheHitRate      float64
    MemoryUsage       uint64
    GoroutineCount    int
}

func (pm *PerformanceMetrics) RecordAggregation(duration time.Duration) {
    pm.AggregationTime = duration
    // Record to monitoring system
}
```

### Profiling Integration
```go
func (a *Aggregator) resolveWithProfiling(actor *Actor) (*Snapshot, error) {
    if a.enableProfiling {
        defer func() {
            if r := recover(); r != nil {
                // Log panic with stack trace
                log.Error("Panic in aggregation", "panic", r)
            }
        }()
    }
    
    return a.resolve(actor)
}
```

## Best Practices

### 1. Minimize Lock Contention
- Use read locks for cache lookups
- Batch write operations
- Use lock-free data structures where possible

### 2. Optimize Hot Paths
- Cache frequently accessed data
- Avoid allocations in hot paths
- Use efficient data structures

### 3. Handle Backpressure
- Implement rate limiting
- Use bounded channels
- Monitor resource usage

### 4. Graceful Shutdown
```go
func (a *Aggregator) Shutdown(ctx context.Context) error {
    // Stop accepting new requests
    a.stopAccepting = true
    
    // Wait for ongoing operations to complete
    select {
    case <-a.done:
        return nil
    case <-ctx.Done():
        return ctx.Err()
    }
}
```

## Testing Performance

### Benchmark Tests
```go
func BenchmarkAggregator_Resolve(b *testing.B) {
    aggregator := NewAggregator()
    actor := createTestActor()
    
    b.ResetTimer()
    for i := 0; i < b.N; i++ {
        _, err := aggregator.Resolve(actor)
        if err != nil {
            b.Fatal(err)
        }
    }
}
```

### Load Tests
```go
func TestConcurrentAccess(t *testing.T) {
    aggregator := NewAggregator()
    actors := createTestActors(1000)
    
    var wg sync.WaitGroup
    for i := 0; i < 100; i++ {
        wg.Add(1)
        go func() {
            defer wg.Done()
            for _, actor := range actors {
                _, err := aggregator.Resolve(actor)
                assert.NoError(t, err)
            }
        }()
    }
    
    wg.Wait()
}
```

## Conclusion

Proper performance and concurrency handling ensures Actor Core v3 can scale to handle many actors and subsystems efficiently while maintaining thread safety and resource efficiency.
