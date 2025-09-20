# CMS Service Configuration Guide

## Overview

This guide covers all configuration options for the CMS service, including environment variables, configuration files, and deployment-specific settings.

## Table of Contents

- [Environment Variables](#environment-variables)
- [Configuration Files](#configuration-files)
- [Database Configuration](#database-configuration)
- [Cache Configuration](#cache-configuration)
- [Security Configuration](#security-configuration)
- [Performance Configuration](#performance-configuration)
- [Logging Configuration](#logging-configuration)

## Environment Variables

### Server Configuration

```bash
# Server settings
CMS_PORT=8080                    # Port to listen on
CMS_HOST=0.0.0.0                # Host to bind to
CMS_WORKERS=4                    # Number of worker threads
CMS_MAX_CONNECTIONS=1000         # Maximum concurrent connections
```

### Database Configuration

```bash
# MongoDB settings
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=chaos_cms
MONGODB_MAX_POOL_SIZE=100
MONGODB_MIN_POOL_SIZE=10
MONGODB_CONNECT_TIMEOUT=5000
MONGODB_SERVER_SELECTION_TIMEOUT=5000

# Redis settings
REDIS_URL=redis://localhost:6379
REDIS_MAX_CONNECTIONS=200
REDIS_CONNECTION_TIMEOUT=5000
REDIS_COMMAND_TIMEOUT=3000
```

### Authentication Configuration

```bash
# JWT settings
JWT_SECRET=your_jwt_secret_here
JWT_EXPIRY=3600
JWT_ISSUER=chaos-world
JWT_AUDIENCE=cms-service

# OAuth settings (optional)
OAUTH_CLIENT_ID=your_client_id
OAUTH_CLIENT_SECRET=your_client_secret
OAUTH_AUTHORIZATION_URL=https://auth.chaos-world.com/oauth/authorize
OAUTH_TOKEN_URL=https://auth.chaos-world.com/oauth/token
```

### File Storage Configuration

```bash
# File storage settings
FILE_STORAGE_PATH=/app/storage
FILE_MAX_SIZE=10485760          # 10MB
FILE_ALLOWED_TYPES=png,jpg,jpeg,gif,mp3,wav
FILE_CLEANUP_INTERVAL=3600      # 1 hour
FILE_RETENTION_DAYS=30
```

## Configuration Files

### Main Configuration (config.yaml)

```yaml
# config/config.yaml
server:
  port: 8080
  host: "0.0.0.0"
  workers: 4
  max_connections: 1000
  timeout: 30s
  keep_alive: 75s

database:
  mongodb:
    uri: "mongodb://localhost:27017"
    database: "chaos_cms"
    options:
      max_pool_size: 100
      min_pool_size: 10
      connect_timeout: 5000
      server_selection_timeout: 5000
    collections:
      quests: "quests"
      npcs: "npcs"
      items: "items"
      locations: "locations"
      files: "files"
      versions: "versions"

cache:
  redis:
    url: "redis://localhost:6379"
    max_connections: 200
    connection_timeout: 5000
    command_timeout: 3000
    ttl: 3600
  memory:
    max_size: 1000
    ttl: 300

storage:
  file_path: "/app/storage"
  max_file_size: 10485760
  allowed_types: ["png", "jpg", "jpeg", "gif", "mp3", "wav"]
  cleanup_interval: 3600
  retention_days: 30

security:
  jwt:
    secret: "your_jwt_secret_here"
    expiry: 3600
    issuer: "chaos-world"
    audience: "cms-service"
  cors:
    allowed_origins: ["http://localhost:3000", "https://admin.chaos-world.com"]
    allowed_methods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
    allowed_headers: ["Authorization", "Content-Type"]
    max_age: 86400

logging:
  level: "info"
  format: "json"
  file: "/app/logs/cms.log"
  max_size: 100MB
  max_files: 10
  compress: true

monitoring:
  metrics:
    enabled: true
    path: "/metrics"
    port: 9090
  health:
    enabled: true
    path: "/health"
    detailed: true
  tracing:
    enabled: true
    jaeger_endpoint: "http://jaeger:14268/api/traces"
```

### Environment-Specific Configs

**Development (config/dev.yaml):**
```yaml
server:
  port: 8080
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
  format: "pretty"
```

**Production (config/prod.yaml):**
```yaml
server:
  port: 8080
  workers: 8
  max_connections: 5000

database:
  mongodb:
    uri: "mongodb://mongodb-cluster:27017"
    database: "chaos_cms"
    options:
      max_pool_size: 200
      min_pool_size: 20

cache:
  redis:
    url: "redis://redis-cluster:6379"
    max_connections: 500
    ttl: 3600

logging:
  level: "info"
  format: "json"
  file: "/var/log/cms/cms.log"
```

## Database Configuration

### MongoDB Settings

```yaml
database:
  mongodb:
    # Connection settings
    uri: "mongodb://username:password@host:port/database"
    database: "chaos_cms"
    
    # Connection pool settings
    options:
      max_pool_size: 100
      min_pool_size: 10
      max_idle_time: 600000      # 10 minutes
      connect_timeout: 5000      # 5 seconds
      server_selection_timeout: 5000
      heartbeat_frequency: 10000 # 10 seconds
      
    # Collection settings
    collections:
      quests: "quests"
      npcs: "npcs"
      items: "items"
      locations: "locations"
      files: "files"
      versions: "versions"
      
    # Index settings
    indexes:
      auto_create: true
      background: true
      
    # Replica set settings
    replica_set:
      name: "rs0"
      read_preference: "primary"
      write_concern: "majority"
```

### Redis Settings

```yaml
cache:
  redis:
    # Connection settings
    url: "redis://username:password@host:port/database"
    max_connections: 200
    connection_timeout: 5000
    command_timeout: 3000
    
    # Pool settings
    pool:
      max_idle: 20
      min_idle: 5
      max_lifetime: 1800000  # 30 minutes
      
    # Cache settings
    ttl: 3600
    key_prefix: "cms:"
    
    # Cluster settings (if using Redis Cluster)
    cluster:
      enabled: false
      nodes: ["redis1:6379", "redis2:6379", "redis3:6379"]
      
    # Sentinel settings (if using Redis Sentinel)
    sentinel:
      enabled: false
      master_name: "mymaster"
      sentinels: ["sentinel1:26379", "sentinel2:26379"]
```

## Cache Configuration

### Multi-Level Caching

```yaml
cache:
  # L1: Memory cache (fastest, smallest)
  memory:
    enabled: true
    max_size: 1000
    ttl: 300
    eviction_policy: "lru"
    
  # L2: Redis cache (fast, medium size)
  redis:
    enabled: true
    url: "redis://localhost:6379"
    ttl: 3600
    max_connections: 200
    
  # L3: Database (slowest, largest)
  database:
    enabled: true
    query_cache: true
    result_cache_ttl: 1800

# Cache strategies
cache_strategies:
  quests:
    memory_ttl: 300
    redis_ttl: 1800
    database_ttl: 3600
  npcs:
    memory_ttl: 600
    redis_ttl: 3600
    database_ttl: 7200
  items:
    memory_ttl: 1800
    redis_ttl: 7200
    database_ttl: 86400
```

## Security Configuration

### Authentication

```yaml
security:
  # JWT configuration
  jwt:
    secret: "your_jwt_secret_here"
    algorithm: "HS256"
    expiry: 3600
    issuer: "chaos-world"
    audience: "cms-service"
    clock_skew: 60
    
  # OAuth configuration (optional)
  oauth:
    enabled: false
    client_id: "your_client_id"
    client_secret: "your_client_secret"
    authorization_url: "https://auth.chaos-world.com/oauth/authorize"
    token_url: "https://auth.chaos-world.com/oauth/token"
    userinfo_url: "https://auth.chaos-world.com/oauth/userinfo"
    scopes: ["read", "write"]
    
  # API Key authentication (optional)
  api_keys:
    enabled: false
    keys:
      - name: "admin"
        key: "admin_api_key_here"
        permissions: ["*"]
      - name: "readonly"
        key: "readonly_api_key_here"
        permissions: ["content.read"]
```

### CORS Configuration

```yaml
security:
  cors:
    enabled: true
    allowed_origins:
      - "http://localhost:3000"
      - "https://admin.chaos-world.com"
      - "https://game.chaos-world.com"
    allowed_methods:
      - "GET"
      - "POST"
      - "PUT"
      - "DELETE"
      - "PATCH"
      - "OPTIONS"
    allowed_headers:
      - "Authorization"
      - "Content-Type"
      - "X-Requested-With"
    exposed_headers:
      - "X-Total-Count"
      - "X-Page-Count"
    max_age: 86400
    credentials: true
```

### Rate Limiting

```yaml
security:
  rate_limiting:
    enabled: true
    rules:
      - name: "authenticated_users"
        key: "user_id"
        limit: 1000
        window: "1h"
        conditions:
          - "auth.authenticated == true"
      - name: "anonymous_users"
        key: "ip"
        limit: 100
        window: "1h"
        conditions:
          - "auth.authenticated == false"
      - name: "bulk_operations"
        key: "user_id"
        limit: 10
        window: "1h"
        conditions:
          - "request.path matches '/bulk/'"
```

## Performance Configuration

### Server Performance

```yaml
server:
  # Threading
  workers: 4
  max_connections: 1000
  
  # Timeouts
  timeout: 30s
  keep_alive: 75s
  read_timeout: 30s
  write_timeout: 30s
  
  # Compression
  compression:
    enabled: true
    level: 6
    types: ["text/plain", "text/html", "application/json"]
    
  # Request limits
  max_request_size: 10485760  # 10MB
  max_headers: 100
  max_header_size: 8192
```

### Database Performance

```yaml
database:
  mongodb:
    # Connection pool
    max_pool_size: 100
    min_pool_size: 10
    
    # Query optimization
    query_timeout: 30000
    max_await_time: 5000
    
    # Index optimization
    indexes:
      auto_create: true
      background: true
      sparse: true
      
    # Read preferences
    read_preference: "primary"
    read_concern: "majority"
    write_concern: "majority"
```

### Cache Performance

```yaml
cache:
  # Memory cache
  memory:
    max_size: 1000
    ttl: 300
    eviction_policy: "lru"
    
  # Redis cache
  redis:
    max_connections: 200
    connection_timeout: 5000
    command_timeout: 3000
    ttl: 3600
    
  # Cache warming
  warming:
    enabled: true
    interval: 300  # 5 minutes
    strategies:
      - "popular_quests"
      - "recent_npcs"
      - "common_items"
```

## Logging Configuration

### Log Levels and Formats

```yaml
logging:
  # Global settings
  level: "info"
  format: "json"  # json, pretty, compact
  
  # File logging
  file:
    enabled: true
    path: "/app/logs/cms.log"
    max_size: "100MB"
    max_files: 10
    compress: true
    
  # Console logging
  console:
    enabled: true
    color: true
    
  # Structured logging
  structured:
    enabled: true
    fields:
      - "timestamp"
      - "level"
      - "message"
      - "request_id"
      - "user_id"
      - "duration"
      
  # Log filtering
  filters:
    - level: "debug"
      target: "cms::database"
    - level: "warn"
      target: "cms::cache"
```

### Application-Specific Logging

```yaml
logging:
  modules:
    # Database logging
    database:
      level: "info"
      target: "cms::database"
      
    # Cache logging
    cache:
      level: "warn"
      target: "cms::cache"
      
    # API logging
    api:
      level: "info"
      target: "cms::api"
      
    # Business logic logging
    business:
      level: "debug"
      target: "cms::business"
```

## Configuration Loading

### Environment Variable Override

```rust
// src/config/mod.rs
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub security: SecurityConfig,
    pub logging: LoggingConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let env = env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".to_string());
        let config_path = format!("config/{}.yaml", env);
        
        // Load base configuration
        let mut config: Config = if std::path::Path::new(&config_path).exists() {
            let content = std::fs::read_to_string(&config_path)?;
            serde_yaml::from_str(&content)?
        } else {
            return Err(ConfigError::FileNotFound(config_path));
        };
        
        // Override with environment variables
        config.override_with_env();
        
        Ok(config)
    }
    
    fn override_with_env(&mut self) {
        if let Ok(port) = env::var("CMS_PORT") {
            self.server.port = port.parse().unwrap_or(self.server.port);
        }
        
        if let Ok(mongodb_uri) = env::var("MONGODB_URI") {
            self.database.mongodb.uri = mongodb_uri;
        }
        
        if let Ok(redis_url) = env::var("REDIS_URL") {
            self.cache.redis.url = redis_url;
        }
        
        if let Ok(jwt_secret) = env::var("JWT_SECRET") {
            self.security.jwt.secret = jwt_secret;
        }
        
        if let Ok(log_level) = env::var("LOG_LEVEL") {
            self.logging.level = log_level;
        }
    }
}
```

This configuration guide provides comprehensive coverage of all CMS service configuration options, from basic environment variables to advanced performance and security settings.
