# CMS Service Deployment Guide

## Overview

This guide covers the deployment of the CMS service in various environments, from local development to production clusters.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Local Development](#local-development)
- [Docker Deployment](#docker-deployment)
- [Kubernetes Deployment](#kubernetes-deployment)
- [Production Deployment](#production-deployment)
- [Configuration Management](#configuration-management)
- [Monitoring and Logging](#monitoring-and-logging)
- [Security Considerations](#security-considerations)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### System Requirements

**Minimum Requirements:**
- CPU: 2 cores
- RAM: 4GB
- Storage: 20GB SSD
- Network: 100 Mbps

**Recommended Requirements:**
- CPU: 4+ cores
- RAM: 8GB+
- Storage: 100GB+ SSD
- Network: 1 Gbps

### Software Dependencies

- **Rust**: 1.70+ (for building from source)
- **MongoDB**: 6.0+
- **Redis**: 7.0+
- **Docker**: 20.10+ (for containerized deployment)
- **Kubernetes**: 1.24+ (for K8s deployment)

## Local Development

### 1. Clone and Build

```bash
# Clone the repository
git clone <repository-url>
cd chaos-backend-service/services/content-management-service

# Build the service
cargo build --release
```

### 2. Start Dependencies

```bash
# Using Docker Compose
docker-compose up -d mongodb redis

# Or start manually
# MongoDB
mongod --dbpath /data/db

# Redis
redis-server
```

### 3. Configure Environment

```bash
# Copy example environment file
cp .env.example .env

# Edit configuration
nano .env
```

**Example .env file:**
```bash
# Server Configuration
CMS_PORT=8080
CMS_HOST=0.0.0.0

# Database Configuration
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=chaos_cms

# Cache Configuration
REDIS_URL=redis://localhost:6379
CACHE_TTL=3600

# Authentication
JWT_SECRET=your_jwt_secret_here

# File Storage
FILE_STORAGE_PATH=./storage
MAX_FILE_SIZE=10485760

# Logging
RUST_LOG=info
```

### 4. Run the Service

```bash
# Development mode
cargo run

# Production mode
cargo run --release
```

### 5. Verify Deployment

```bash
# Health check
curl http://localhost:8080/health

# API test
curl http://localhost:8080/api/v1/quests
```

## Docker Deployment

### 1. Build Docker Image

```dockerfile
# Dockerfile
FROM rust:1.70-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Copy binary
COPY --from=builder /app/target/release/content-management-service /usr/local/bin/

# Create directories
RUN mkdir -p /app/storage && chown appuser:appuser /app/storage

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["content-management-service"]
```

### 2. Build and Push Image

```bash
# Build image
docker build -t chaos-world/cms-service:latest .

# Tag for registry
docker tag chaos-world/cms-service:latest your-registry.com/chaos-world/cms-service:latest

# Push to registry
docker push your-registry.com/chaos-world/cms-service:latest
```

### 3. Docker Compose Deployment

```yaml
# docker-compose.yml
version: '3.8'

services:
  cms-service:
    image: chaos-world/cms-service:latest
    ports:
      - "8080:8080"
    environment:
      - MONGODB_URI=mongodb://mongodb:27017
      - REDIS_URL=redis://redis:6379
      - JWT_SECRET=your_jwt_secret_here
      - FILE_STORAGE_PATH=/app/storage
    volumes:
      - cms_storage:/app/storage
    depends_on:
      - mongodb
      - redis
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  mongodb:
    image: mongo:7.0
    ports:
      - "27017:27017"
    environment:
      - MONGO_INITDB_ROOT_USERNAME=admin
      - MONGO_INITDB_ROOT_PASSWORD=password
      - MONGO_INITDB_DATABASE=chaos_cms
    volumes:
      - mongodb_data:/data/db
      - ./init-mongo.js:/docker-entrypoint-initdb.d/init-mongo.js:ro
    restart: unless-stopped

  redis:
    image: redis:7.0-alpine
    ports:
      - "6379:6379"
    command: redis-server --appendonly yes
    volumes:
      - redis_data:/data
    restart: unless-stopped

volumes:
  mongodb_data:
  redis_data:
  cms_storage:
```

### 4. Deploy with Docker Compose

```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f cms-service

# Scale service
docker-compose up -d --scale cms-service=3

# Stop services
docker-compose down
```

## Kubernetes Deployment

### 1. Namespace

```yaml
# namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: chaos-world
  labels:
    name: chaos-world
```

### 2. ConfigMap

```yaml
# configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: cms-config
  namespace: chaos-world
data:
  MONGODB_URI: "mongodb://mongodb-service:27017"
  REDIS_URL: "redis://redis-service:6379"
  FILE_STORAGE_PATH: "/app/storage"
  LOG_LEVEL: "info"
  CACHE_TTL: "3600"
```

### 3. Secret

```yaml
# secret.yaml
apiVersion: v1
kind: Secret
metadata:
  name: cms-secrets
  namespace: chaos-world
type: Opaque
data:
  JWT_SECRET: <base64-encoded-jwt-secret>
  MONGODB_PASSWORD: <base64-encoded-password>
```

### 4. PersistentVolumeClaim

```yaml
# pvc.yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: cms-storage
  namespace: chaos-world
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 10Gi
  storageClassName: fast-ssd
```

### 5. Deployment

```yaml
# deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cms-service
  namespace: chaos-world
  labels:
    app: cms-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: cms-service
  template:
    metadata:
      labels:
        app: cms-service
    spec:
      containers:
      - name: cms-service
        image: chaos-world/cms-service:latest
        ports:
        - containerPort: 8080
          name: http
        env:
        - name: MONGODB_URI
          valueFrom:
            configMapKeyRef:
              name: cms-config
              key: MONGODB_URI
        - name: REDIS_URL
          valueFrom:
            configMapKeyRef:
              name: cms-config
              key: REDIS_URL
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: cms-secrets
              key: JWT_SECRET
        volumeMounts:
        - name: storage
          mountPath: /app/storage
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: storage
        persistentVolumeClaim:
          claimName: cms-storage
```

### 6. Service

```yaml
# service.yaml
apiVersion: v1
kind: Service
metadata:
  name: cms-service
  namespace: chaos-world
  labels:
    app: cms-service
spec:
  selector:
    app: cms-service
  ports:
  - port: 80
    targetPort: 8080
    protocol: TCP
    name: http
  type: ClusterIP
```

### 7. Ingress

```yaml
# ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: cms-ingress
  namespace: chaos-world
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
spec:
  tls:
  - hosts:
    - cms.chaos-world.com
    secretName: cms-tls
  rules:
  - host: cms.chaos-world.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: cms-service
            port:
              number: 80
```

### 8. Deploy to Kubernetes

```bash
# Apply all manifests
kubectl apply -f namespace.yaml
kubectl apply -f configmap.yaml
kubectl apply -f secret.yaml
kubectl apply -f pvc.yaml
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
kubectl apply -f ingress.yaml

# Check deployment status
kubectl get pods -n chaos-world
kubectl get services -n chaos-world
kubectl get ingress -n chaos-world

# View logs
kubectl logs -f deployment/cms-service -n chaos-world
```

## Production Deployment

### 1. High Availability Setup

```yaml
# ha-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cms-service
  namespace: chaos-world
spec:
  replicas: 5
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 1
      maxSurge: 2
  selector:
    matchLabels:
      app: cms-service
  template:
    metadata:
      labels:
        app: cms-service
    spec:
      affinity:
        podAntiAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
          - weight: 100
            podAffinityTerm:
              labelSelector:
                matchExpressions:
                - key: app
                  operator: In
                  values:
                  - cms-service
              topologyKey: kubernetes.io/hostname
      containers:
      - name: cms-service
        image: chaos-world/cms-service:latest
        ports:
        - containerPort: 8080
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3
```

### 2. Horizontal Pod Autoscaler

```yaml
# hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: cms-hpa
  namespace: chaos-world
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: cms-service
  minReplicas: 3
  maxReplicas: 20
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### 3. Database High Availability

```yaml
# mongodb-ha.yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: mongodb
  namespace: chaos-world
spec:
  serviceName: mongodb
  replicas: 3
  selector:
    matchLabels:
      app: mongodb
  template:
    metadata:
      labels:
        app: mongodb
    spec:
      containers:
      - name: mongodb
        image: mongo:7.0
        ports:
        - containerPort: 27017
        env:
        - name: MONGO_INITDB_ROOT_USERNAME
          value: "admin"
        - name: MONGO_INITDB_ROOT_PASSWORD
          valueFrom:
            secretKeyRef:
              name: mongodb-secrets
              key: password
        volumeMounts:
        - name: mongodb-data
          mountPath: /data/db
        - name: mongodb-config
          mountPath: /etc/mongod.conf
          subPath: mongod.conf
  volumeClaimTemplates:
  - metadata:
      name: mongodb-data
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 20Gi
```

## Configuration Management

### 1. Environment-Specific Configs

**Development:**
```yaml
# config/dev.yaml
server:
  port: 8080
  host: "0.0.0.0"
  workers: 2

database:
  mongodb:
    uri: "mongodb://localhost:27017"
    database: "chaos_cms_dev"

cache:
  redis:
    url: "redis://localhost:6379"
    ttl: 1800

logging:
  level: "debug"
```

**Production:**
```yaml
# config/prod.yaml
server:
  port: 8080
  host: "0.0.0.0"
  workers: 8

database:
  mongodb:
    uri: "mongodb://mongodb-cluster:27017"
    database: "chaos_cms"
    options:
      maxPoolSize: 100
      minPoolSize: 10

cache:
  redis:
    url: "redis://redis-cluster:6379"
    ttl: 3600
    max_connections: 200

logging:
  level: "info"
  format: "json"
```

### 2. Configuration Loading

```rust
// src/config/mod.rs
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub logging: LoggingConfig,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let env = env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".to_string());
        let config_path = format!("config/{}.yaml", env);
        
        let config_content = std::fs::read_to_string(&config_path)?;
        let mut config: Config = serde_yaml::from_str(&config_content)?;
        
        // Override with environment variables
        if let Ok(port) = env::var("CMS_PORT") {
            config.server.port = port.parse()?;
        }
        
        if let Ok(mongodb_uri) = env::var("MONGODB_URI") {
            config.database.mongodb.uri = mongodb_uri;
        }
        
        Ok(config)
    }
}
```

## Monitoring and Logging

### 1. Prometheus Metrics

```rust
// src/metrics.rs
use prometheus::{Counter, Histogram, Registry};

pub struct Metrics {
    pub requests_total: Counter,
    pub request_duration: Histogram,
    pub active_connections: Counter,
}

impl Metrics {
    pub fn new(registry: &Registry) -> Self {
        let requests_total = Counter::new(
            "cms_requests_total",
            "Total number of requests"
        ).unwrap();
        
        let request_duration = Histogram::new(
            "cms_request_duration_seconds",
            "Request duration in seconds"
        ).unwrap();
        
        let active_connections = Counter::new(
            "cms_active_connections",
            "Number of active connections"
        ).unwrap();
        
        registry.register(Box::new(requests_total.clone())).unwrap();
        registry.register(Box::new(request_duration.clone())).unwrap();
        registry.register(Box::new(active_connections.clone())).unwrap();
        
        Self {
            requests_total,
            request_duration,
            active_connections,
        }
    }
}
```

### 2. Grafana Dashboard

```json
{
  "dashboard": {
    "title": "CMS Service Dashboard",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(cms_requests_total[5m])",
            "legendFormat": "Requests/sec"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(cms_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          }
        ]
      }
    ]
  }
}
```

### 3. Log Aggregation

```yaml
# fluentd-config.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: fluentd-config
  namespace: chaos-world
data:
  fluent.conf: |
    <source>
      @type tail
      path /var/log/containers/cms-service*.log
      pos_file /var/log/fluentd-cms.log.pos
      tag cms.*
      format json
    </source>
    
    <match cms.**>
      @type elasticsearch
      host elasticsearch.logging.svc.cluster.local
      port 9200
      index_name cms-logs
    </match>
```

## Security Considerations

### 1. Network Security

```yaml
# network-policy.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: cms-network-policy
  namespace: chaos-world
spec:
  podSelector:
    matchLabels:
      app: cms-service
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: api-gateway
    ports:
    - protocol: TCP
      port: 8080
  egress:
  - to:
    - namespaceSelector:
        matchLabels:
          name: database
    ports:
    - protocol: TCP
      port: 27017
  - to:
    - namespaceSelector:
        matchLabels:
          name: cache
    ports:
    - protocol: TCP
      port: 6379
```

### 2. Pod Security Policy

```yaml
# psp.yaml
apiVersion: policy/v1beta1
kind: PodSecurityPolicy
metadata:
  name: cms-psp
  namespace: chaos-world
spec:
  privileged: false
  allowPrivilegeEscalation: false
  requiredDropCapabilities:
    - ALL
  volumes:
    - 'configMap'
    - 'emptyDir'
    - 'projected'
    - 'secret'
    - 'downwardAPI'
    - 'persistentVolumeClaim'
  runAsUser:
    rule: 'MustRunAsNonRoot'
  seLinux:
    rule: 'RunAsAny'
  fsGroup:
    rule: 'RunAsAny'
```

### 3. RBAC

```yaml
# rbac.yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: cms-service
  namespace: chaos-world
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: cms-role
  namespace: chaos-world
rules:
- apiGroups: [""]
  resources: ["configmaps", "secrets"]
  verbs: ["get", "list"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: cms-rolebinding
  namespace: chaos-world
subjects:
- kind: ServiceAccount
  name: cms-service
  namespace: chaos-world
roleRef:
  kind: Role
  name: cms-role
  apiGroup: rbac.authorization.k8s.io
```

## Troubleshooting

### 1. Common Issues

**Service Won't Start:**
```bash
# Check logs
kubectl logs -f deployment/cms-service -n chaos-world

# Check configuration
kubectl describe configmap cms-config -n chaos-world

# Check secrets
kubectl describe secret cms-secrets -n chaos-world
```

**Database Connection Issues:**
```bash
# Test MongoDB connection
kubectl exec -it deployment/cms-service -n chaos-world -- \
  mongosh "mongodb://mongodb-service:27017/chaos_cms"

# Check MongoDB status
kubectl get pods -l app=mongodb -n chaos-world
```

**High Memory Usage:**
```bash
# Check resource usage
kubectl top pods -n chaos-world

# Check memory limits
kubectl describe pod <pod-name> -n chaos-world
```

### 2. Debug Commands

```bash
# Get pod details
kubectl describe pod <pod-name> -n chaos-world

# Check events
kubectl get events -n chaos-world --sort-by='.lastTimestamp'

# Port forward for local testing
kubectl port-forward svc/cms-service 8080:80 -n chaos-world

# Execute commands in pod
kubectl exec -it <pod-name> -n chaos-world -- /bin/bash
```

### 3. Performance Tuning

**Database Optimization:**
```javascript
// MongoDB indexes
db.quests.createIndex({ "status": 1, "type": 1 })
db.quests.createIndex({ "level_required": 1 })
db.quests.createIndex({ "title": "text", "description": "text" })

// Connection pooling
db.adminCommand({setParameter: 1, maxConnections: 1000})
```

**Redis Optimization:**
```bash
# Redis configuration
redis-cli CONFIG SET maxmemory 2gb
redis-cli CONFIG SET maxmemory-policy allkeys-lru
redis-cli CONFIG SET save "900 1 300 10 60 10000"
```

### 4. Backup and Recovery

**Database Backup:**
```bash
# MongoDB backup
mongodump --uri="mongodb://mongodb-service:27017/chaos_cms" \
  --out=/backup/$(date +%Y%m%d)

# Restore from backup
mongorestore --uri="mongodb://mongodb-service:27017/chaos_cms" \
  /backup/20240101
```

**File Storage Backup:**
```bash
# Backup file storage
kubectl exec -it <pod-name> -n chaos-world -- \
  tar -czf /backup/storage-$(date +%Y%m%d).tar.gz /app/storage

# Restore file storage
kubectl exec -it <pod-name> -n chaos-world -- \
  tar -xzf /backup/storage-20240101.tar.gz -C /
```
