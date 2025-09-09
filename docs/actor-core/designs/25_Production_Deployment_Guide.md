# 25 — Production Deployment Guide

**Updated:** 2025-09-08 01:00

This guide covers deploying Actor Core v3 in production environments with high availability, performance, and reliability.

## Overview

Production deployment of Actor Core v3 requires careful consideration of:
- **Scalability**: Handle thousands of concurrent actors
- **Performance**: Sub-millisecond response times
- **Reliability**: 99.9% uptime
- **Monitoring**: Comprehensive observability
- **Security**: Data protection and access control

## Architecture Overview

### Production Architecture
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Load Balancer │────│  Actor Core v3  │────│   Database      │
│   (HAProxy)     │    │   (Multiple     │    │   (PostgreSQL)  │
│                 │    │    Instances)   │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Monitoring    │    │   Cache Layer   │    │   Backup        │
│   (Prometheus)  │    │   (Redis)       │    │   (S3/MinIO)    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Component Responsibilities
- **Load Balancer**: Distributes requests across instances
- **Actor Core v3**: Processes actor stat calculations
- **Database**: Stores persistent actor data
- **Cache Layer**: Provides fast access to calculated stats
- **Monitoring**: Tracks performance and health metrics
- **Backup**: Ensures data durability

## Deployment Strategies

### 1. Blue-Green Deployment
```yaml
# docker-compose.blue-green.yml
version: '3.8'
services:
  actor-core-blue:
    image: actor-core:v1.0.0
    ports:
      - "8080:8080"
    environment:
      - ENV=production
      - VERSION=blue
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  actor-core-green:
    image: actor-core:v1.1.0
    ports:
      - "8081:8080"
    environment:
      - ENV=production
      - VERSION=green
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8081/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  load-balancer:
    image: haproxy:2.4
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./haproxy.cfg:/usr/local/etc/haproxy/haproxy.cfg
```

### 2. Rolling Deployment
```yaml
# kubernetes-deployment.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: actor-core
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 1
      maxSurge: 1
  selector:
    matchLabels:
      app: actor-core
  template:
    metadata:
      labels:
        app: actor-core
    spec:
      containers:
      - name: actor-core
        image: actor-core:latest
        ports:
        - containerPort: 8080
        env:
        - name: ENV
          value: "production"
        - name: REDIS_URL
          value: "redis://redis:6379"
        - name: DATABASE_URL
          value: "postgres://user:pass@postgres:5432/actorcore"
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
```

## Configuration Management

### Environment-Specific Configuration
```yaml
# config/production.yaml
server:
  port: 8080
  host: "0.0.0.0"
  read_timeout: "30s"
  write_timeout: "30s"
  idle_timeout: "120s"

database:
  host: "postgres.example.com"
  port: 5432
  name: "actorcore_prod"
  user: "actorcore_user"
  password: "${DB_PASSWORD}"
  ssl_mode: "require"
  max_connections: 100
  max_idle_connections: 10
  connection_max_lifetime: "1h"

cache:
  redis:
    host: "redis.example.com"
    port: 6379
    password: "${REDIS_PASSWORD}"
    db: 0
    max_retries: 3
    dial_timeout: "5s"
    read_timeout: "3s"
    write_timeout: "3s"
    pool_size: 100
    min_idle_connections: 10

logging:
  level: "info"
  format: "json"
  output: "stdout"
  fields:
    service: "actor-core"
    version: "v3.0.0"
    environment: "production"

monitoring:
  prometheus:
    enabled: true
    port: 9090
    path: "/metrics"
  jaeger:
    enabled: true
    endpoint: "http://jaeger:14268/api/traces"
    service_name: "actor-core"
```

### Configuration Loading
```go
type ProductionConfig struct {
    Server     ServerConfig     `yaml:"server"`
    Database   DatabaseConfig   `yaml:"database"`
    Cache      CacheConfig      `yaml:"cache"`
    Logging    LoggingConfig    `yaml:"logging"`
    Monitoring MonitoringConfig `yaml:"monitoring"`
}

func LoadProductionConfig() (*ProductionConfig, error) {
    config := &ProductionConfig{}
    
    // Load base configuration
    if err := loadYAMLConfig("config/production.yaml", config); err != nil {
        return nil, err
    }
    
    // Override with environment variables
    if err := overrideWithEnv(config); err != nil {
        return nil, err
    }
    
    // Validate configuration
    if err := validateConfig(config); err != nil {
        return nil, err
    }
    
    return config, nil
}
```

## Database Setup

### PostgreSQL Configuration
```sql
-- Create database
CREATE DATABASE actorcore_prod;

-- Create user
CREATE USER actorcore_user WITH PASSWORD 'secure_password';

-- Grant permissions
GRANT ALL PRIVILEGES ON DATABASE actorcore_prod TO actorcore_user;

-- Create tables
CREATE TABLE actors (
    id VARCHAR(255) PRIMARY KEY,
    version BIGINT NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE subsystems (
    id VARCHAR(255) PRIMARY KEY,
    actor_id VARCHAR(255) NOT NULL,
    system_id VARCHAR(255) NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    FOREIGN KEY (actor_id) REFERENCES actors(id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX idx_actors_version ON actors(version);
CREATE INDEX idx_subsystems_actor_id ON subsystems(actor_id);
CREATE INDEX idx_subsystems_system_id ON subsystems(system_id);
CREATE INDEX idx_actors_data_gin ON actors USING GIN(data);
CREATE INDEX idx_subsystems_data_gin ON subsystems USING GIN(data);

-- Create updated_at trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_actors_updated_at BEFORE UPDATE ON actors
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_subsystems_updated_at BEFORE UPDATE ON subsystems
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

### Database Connection Pool
```go
type DatabasePool struct {
    db *sql.DB
    mu sync.RWMutex
}

func NewDatabasePool(config DatabaseConfig) (*DatabasePool, error) {
    db, err := sql.Open("postgres", config.ConnectionString())
    if err != nil {
        return nil, err
    }
    
    // Configure connection pool
    db.SetMaxOpenConns(config.MaxConnections)
    db.SetMaxIdleConns(config.MaxIdleConnections)
    db.SetConnMaxLifetime(config.ConnectionMaxLifetime)
    
    // Test connection
    if err := db.Ping(); err != nil {
        return nil, err
    }
    
    return &DatabasePool{db: db}, nil
}

func (dp *DatabasePool) GetActor(id string) (*Actor, error) {
    row := dp.db.QueryRow(`
        SELECT id, version, data 
        FROM actors 
        WHERE id = $1
    `, id)
    
    var actor Actor
    var dataJSON []byte
    
    err := row.Scan(&actor.ID, &actor.Version, &dataJSON)
    if err != nil {
        return nil, err
    }
    
    if err := json.Unmarshal(dataJSON, &actor.Data); err != nil {
        return nil, err
    }
    
    return &actor, nil
}
```

## Caching Strategy

### Redis Configuration
```yaml
# redis.conf
maxmemory 2gb
maxmemory-policy allkeys-lru
save 900 1
save 300 10
save 60 10000
appendonly yes
appendfsync everysec
```

### Cache Implementation
```go
type RedisCache struct {
    client *redis.Client
    mu     sync.RWMutex
}

func NewRedisCache(config RedisConfig) (*RedisCache, error) {
    client := redis.NewClient(&redis.Options{
        Addr:         fmt.Sprintf("%s:%d", config.Host, config.Port),
        Password:     config.Password,
        DB:           config.DB,
        MaxRetries:   config.MaxRetries,
        DialTimeout:  config.DialTimeout,
        ReadTimeout:  config.ReadTimeout,
        WriteTimeout: config.WriteTimeout,
        PoolSize:     config.PoolSize,
        MinIdleConns: config.MinIdleConnections,
    })
    
    // Test connection
    if err := client.Ping().Err(); err != nil {
        return nil, err
    }
    
    return &RedisCache{client: client}, nil
}

func (rc *RedisCache) Get(key string) (*Snapshot, error) {
    data, err := rc.client.Get(key).Result()
    if err != nil {
        if err == redis.Nil {
            return nil, nil // Cache miss
        }
        return nil, err
    }
    
    var snapshot Snapshot
    if err := json.Unmarshal([]byte(data), &snapshot); err != nil {
        return nil, err
    }
    
    return &snapshot, nil
}

func (rc *RedisCache) Set(key string, snapshot *Snapshot, ttl time.Duration) error {
    data, err := json.Marshal(snapshot)
    if err != nil {
        return err
    }
    
    return rc.client.Set(key, data, ttl).Err()
}
```

## Monitoring and Observability

### Prometheus Metrics
```go
type MetricsCollector struct {
    requestsTotal     prometheus.Counter
    requestDuration   prometheus.Histogram
    activeActors      prometheus.Gauge
    cacheHits         prometheus.Counter
    cacheMisses       prometheus.Counter
    errorsTotal       prometheus.Counter
}

func NewMetricsCollector() *MetricsCollector {
    return &MetricsCollector{
        requestsTotal: prometheus.NewCounter(prometheus.CounterOpts{
            Name: "actor_core_requests_total",
            Help: "Total number of requests processed",
        }),
        requestDuration: prometheus.NewHistogram(prometheus.HistogramOpts{
            Name:    "actor_core_request_duration_seconds",
            Help:    "Request duration in seconds",
            Buckets: prometheus.DefBuckets,
        }),
        activeActors: prometheus.NewGauge(prometheus.GaugeOpts{
            Name: "actor_core_active_actors",
            Help: "Number of active actors",
        }),
        cacheHits: prometheus.NewCounter(prometheus.CounterOpts{
            Name: "actor_core_cache_hits_total",
            Help: "Total number of cache hits",
        }),
        cacheMisses: prometheus.NewCounter(prometheus.CounterOpts{
            Name: "actor_core_cache_misses_total",
            Help: "Total number of cache misses",
        }),
        errorsTotal: prometheus.NewCounter(prometheus.CounterOpts{
            Name: "actor_core_errors_total",
            Help: "Total number of errors",
        }),
    }
}

func (mc *MetricsCollector) RecordRequest(duration time.Duration) {
    mc.requestsTotal.Inc()
    mc.requestDuration.Observe(duration.Seconds())
}

func (mc *MetricsCollector) RecordCacheHit() {
    mc.cacheHits.Inc()
}

func (mc *MetricsCollector) RecordCacheMiss() {
    mc.cacheMisses.Inc()
}

func (mc *MetricsCollector) RecordError() {
    mc.errorsTotal.Inc()
}
```

### Health Checks
```go
type HealthChecker struct {
    db    *DatabasePool
    cache *RedisCache
}

func (hc *HealthChecker) CheckHealth() HealthStatus {
    status := HealthStatus{
        Overall: true,
        Checks:  make(map[string]bool),
    }
    
    // Check database
    if err := hc.db.Ping(); err != nil {
        status.Checks["database"] = false
        status.Overall = false
    } else {
        status.Checks["database"] = true
    }
    
    // Check cache
    if err := hc.cache.Ping(); err != nil {
        status.Checks["cache"] = false
        status.Overall = false
    } else {
        status.Checks["cache"] = true
    }
    
    return status
}
```

## Security Considerations

### Authentication and Authorization
```go
type SecurityMiddleware struct {
    jwtSecret []byte
    rateLimiter *rate.Limiter
}

func (sm *SecurityMiddleware) Authenticate(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        token := r.Header.Get("Authorization")
        if token == "" {
            http.Error(w, "Missing authorization token", http.StatusUnauthorized)
            return
        }
        
        // Validate JWT token
        claims, err := sm.validateJWT(token)
        if err != nil {
            http.Error(w, "Invalid token", http.StatusUnauthorized)
            return
        }
        
        // Add claims to context
        ctx := context.WithValue(r.Context(), "claims", claims)
        next.ServeHTTP(w, r.WithContext(ctx))
    })
}

func (sm *SecurityMiddleware) RateLimit(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        if !sm.rateLimiter.Allow() {
            http.Error(w, "Rate limit exceeded", http.StatusTooManyRequests)
            return
        }
        
        next.ServeHTTP(w, r)
    })
}
```

### Data Encryption
```go
type EncryptedStorage struct {
    encryptionKey []byte
    db           *DatabasePool
}

func (es *EncryptedStorage) StoreActor(actor *Actor) error {
    // Encrypt sensitive data
    encryptedData, err := es.encrypt(actor.Data)
    if err != nil {
        return err
    }
    
    // Store encrypted data
    return es.db.StoreActor(actor.ID, actor.Version, encryptedData)
}

func (es *EncryptedStorage) GetActor(id string) (*Actor, error) {
    // Get encrypted data
    encryptedData, err := es.db.GetActorData(id)
    if err != nil {
        return nil, err
    }
    
    // Decrypt data
    decryptedData, err := es.decrypt(encryptedData)
    if err != nil {
        return nil, err
    }
    
    // Create actor
    actor := &Actor{
        ID:   id,
        Data: decryptedData,
    }
    
    return actor, nil
}
```

## Performance Optimization

### Connection Pooling
```go
type ConnectionPool struct {
    db    *sql.DB
    cache *redis.Client
    mu    sync.RWMutex
}

func (cp *ConnectionPool) GetDB() *sql.DB {
    cp.mu.RLock()
    defer cp.mu.RUnlock()
    return cp.db
}

func (cp *ConnectionPool) GetCache() *redis.Client {
    cp.mu.RLock()
    defer cp.mu.RUnlock()
    return cp.cache
}
```

### Batch Processing
```go
func (a *Aggregator) ProcessBatch(actors []*Actor) ([]*Snapshot, error) {
    // Process actors in parallel
    var wg sync.WaitGroup
    snapshots := make([]*Snapshot, len(actors))
    errors := make([]error, len(actors))
    
    for i, actor := range actors {
        wg.Add(1)
        go func(idx int, act *Actor) {
            defer wg.Done()
            snapshot, err := a.Resolve(act)
            snapshots[idx] = snapshot
            errors[idx] = err
        }(i, actor)
    }
    
    wg.Wait()
    
    // Check for errors
    for _, err := range errors {
        if err != nil {
            return nil, err
        }
    }
    
    return snapshots, nil
}
```

## Backup and Recovery

### Automated Backups
```bash
#!/bin/bash
# backup.sh

# Database backup
pg_dump -h postgres.example.com -U actorcore_user actorcore_prod > backup_$(date +%Y%m%d_%H%M%S).sql

# Upload to S3
aws s3 cp backup_$(date +%Y%m%d_%H%M%S).sql s3://actor-core-backups/

# Cleanup old backups (keep last 30 days)
find /backups -name "backup_*.sql" -mtime +30 -delete
```

### Disaster Recovery
```go
type DisasterRecovery struct {
    db    *DatabasePool
    cache *RedisCache
    s3    *s3.Client
}

func (dr *DisasterRecovery) RestoreFromBackup(backupKey string) error {
    // Download backup from S3
    backupData, err := dr.downloadBackup(backupKey)
    if err != nil {
        return err
    }
    
    // Restore database
    if err := dr.restoreDatabase(backupData); err != nil {
        return err
    }
    
    // Clear cache
    if err := dr.cache.FlushAll(); err != nil {
        return err
    }
    
    return nil
}
```

## Deployment Checklist

### Pre-Deployment
- [ ] Configuration files updated
- [ ] Database migrations applied
- [ ] Cache cleared
- [ ] Health checks passing
- [ ] Performance tests completed
- [ ] Security scan passed

### Deployment
- [ ] Blue-green deployment executed
- [ ] Load balancer updated
- [ ] Monitoring alerts configured
- [ ] Logs being collected
- [ ] Metrics being recorded

### Post-Deployment
- [ ] Health checks passing
- [ ] Performance metrics normal
- [ ] Error rates within limits
- [ ] Cache hit rates optimal
- [ ] Database performance normal

## Conclusion

This production deployment guide provides a comprehensive foundation for deploying Actor Core v3 in production environments. By following these guidelines, you can ensure:

1. **High Availability**: Multiple instances with load balancing
2. **Performance**: Optimized caching and connection pooling
3. **Reliability**: Health checks and monitoring
4. **Security**: Authentication, authorization, and encryption
5. **Scalability**: Horizontal scaling capabilities
6. **Recovery**: Backup and disaster recovery procedures

Remember to adapt these guidelines to your specific infrastructure and requirements.
