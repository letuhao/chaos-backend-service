# 11 — Error Handling & Observability

**Updated:** 2025-09-08 00:35

This document outlines the error handling strategy and observability requirements for Actor Core v3.

## Error Handling Strategy

### Error Categories
- **Validation Errors**: Invalid input data, schema violations, range violations
- **System Errors**: Subsystem failures, registry errors, caps conflicts  
- **Performance Errors**: Timeouts, resource exhaustion, cache failures

### Error Structure
```go
type ActorCoreError struct {
    Type        string            `json:"type"`        // Error category
    Code        string            `json:"code"`        // Specific error code
    Message     string            `json:"message"`     // Human-readable message
    System      string            `json:"system"`      // Originating system
    Dimension   string            `json:"dimension"`   // Affected dimension
    Layer       string            `json:"layer"`       // Affected layer
    Context     map[string]interface{} `json:"context"` // Additional context
    Timestamp   time.Time         `json:"timestamp"`   // When error occurred
}
```

### Error Recovery Mechanisms
- **Caps Conflict Resolution**: Auto-correct when min > max
- **Fallback Values**: Use cached snapshots when resolution fails
- **Graceful Degradation**: Continue with available subsystems

## Observability Requirements

### Logging Strategy
- **Structured Logging**: Include system, dimension, layer in all logs
- **Log Levels**: DEBUG, INFO, WARN, ERROR, FATAL
- **Key Events**: Subsystem contributions, caps calculations, aggregation steps

### Metrics Collection
- **Performance Metrics**: Timing, throughput, error rates
- **Business Metrics**: Active actors, dimension usage, layer distribution
- **Cache Metrics**: Hit rates, miss rates, eviction rates

### Monitoring & Alerting
- **Health Checks**: Registry, cache, subsystem health
- **Alert Conditions**: High error rates, slow aggregation, cache misses
- **Tracing**: Distributed tracing for debugging

## Implementation Guidelines

### Error Handling Best Practices
1. Always include context (system, dimension, layer)
2. Use appropriate error types for different scenarios
3. Implement automatic recovery where possible
4. Log errors with full context for debugging

### Logging Best Practices
1. Use structured logging with consistent fields
2. Include performance data in logs
3. Set appropriate log levels
4. Monitor log volume and performance impact

### Monitoring Best Practices
1. Set appropriate thresholds based on expected performance
2. Monitor key indicators continuously
3. Implement alerting for critical conditions
4. Use metrics for capacity planning

## Testing Error Handling

### Unit Tests
- Test validation errors with invalid inputs
- Test recovery mechanisms with conflicting data
- Test error propagation through the system

### Integration Tests
- Test subsystem failure isolation
- Test fallback mechanisms
- Test error recovery in real scenarios

## Conclusion

Proper error handling and observability ensure Actor Core v3 operates reliably in production while providing the visibility needed to diagnose and resolve issues quickly.
