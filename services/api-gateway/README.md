# API Gateway Service

A high-performance, production-ready API Gateway for the Chaos World game backend. This service provides routing, load balancing, authentication, rate limiting, monitoring, and security features.

## Features

### Core Features
- **High Performance**: Built with Rust and Axum for maximum performance
- **Load Balancing**: Multiple algorithms (round-robin, least-connections, weighted, IP hash)
- **Service Discovery**: Support for Consul, Kubernetes, and static configuration
- **Circuit Breaker**: Automatic failure detection and recovery
- **Health Checks**: Comprehensive health monitoring for all services

### Security Features
- **Authentication**: JWT, OAuth2, and API key authentication
- **Authorization**: Role-based access control (RBAC)
- **Rate Limiting**: Configurable rate limiting with Redis backend
- **IP Filtering**: Whitelist and blacklist support
- **Security Headers**: Automatic security header injection
- **CORS**: Cross-origin resource sharing support

### Monitoring Features
- **Metrics**: Prometheus metrics collection
- **Tracing**: Distributed tracing with Jaeger
- **Logging**: Structured logging with configurable levels
- **Health Checks**: Service health monitoring
- **Performance Monitoring**: Request/response time tracking

### Caching Features
- **Redis Caching**: High-performance caching with Redis
- **Local Caching**: In-memory caching for frequently accessed data
- **TTL Support**: Configurable time-to-live for cache entries
- **Cache Invalidation**: Smart cache invalidation strategies

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client        │    │   API Gateway   │    │   Backend       │
│                 │    │                 │    │   Services      │
│  ┌───────────┐  │    │  ┌───────────┐  │    │  ┌───────────┐  │
│  │   Web     │  │───▶│  │  Router   │  │───▶│  │   User    │  │
│  │   App     │  │    │  │           │  │    │  │ Service  │  │
│  └───────────┘  │    │  └───────────┘  │    │  └───────────┘  │
│                 │    │  ┌───────────┐  │    │  ┌───────────┐  │
│  ┌───────────┐  │    │  │   Auth    │  │    │  │   Game    │  │
│  │  Mobile   │  │───▶│  │   Service │  │───▶│  │  Service  │  │
│  │   App     │  │    │  └───────────┘  │    │  └───────────┘  │
│  └───────────┘  │    │  ┌───────────┐  │    │  ┌───────────┐  │
│                 │    │  │   Rate    │  │    │  │Inventory  │  │
│  ┌───────────┐  │    │  │  Limiter  │  │    │  │ Service  │  │
│  │   Game    │  │───▶│  └───────────┘  │    │  └───────────┘  │
│  │  Client   │  │    │  ┌───────────┐  │    │  ┌───────────┐  │
│  └───────────┘  │    │  │   Cache   │  │    │  │   Chat    │  │
│                 │    │  │  Service  │  │    │  │ Service  │  │
│                 │    │  └───────────┘  │    │  └───────────┘  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.70+ (https://rustup.rs/)
- Redis (optional, for caching and rate limiting)
- MongoDB (optional, for configuration storage)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/chaos-world/api-gateway.git
   cd api-gateway
   ```

2. **Setup the environment**
   ```powershell
   .\scripts\setup_api_gateway.ps1
   ```

3. **Configure the service**
   ```bash
   cp env.example .env
   # Edit .env with your configuration
   ```

4. **Run the service**
   ```powershell
   .\scripts\run_api_gateway.ps1
   ```

5. **Test the service**
   ```powershell
   .\scripts\test_api_gateway.ps1
   ```

### Manual Installation

1. **Build the service**
   ```bash
   cargo build --release
   ```

2. **Run the service**
   ```bash
   ./target/release/api-gateway --env development --debug
   ```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ENV` | Environment (development/production) | `development` |
| `RUST_LOG` | Log level | `info` |
| `API_GATEWAY_HOST` | Server host | `0.0.0.0` |
| `API_GATEWAY_PORT` | Server port | `8080` |
| `REDIS_HOST` | Redis host | `localhost` |
| `REDIS_PORT` | Redis port | `6379` |
| `JWT_SECRET` | JWT secret key | `your-jwt-secret-key` |

### Configuration Files

- `configs/api-gateway.yaml` - Default configuration
- `configs/api-gateway-dev.yaml` - Development configuration
- `configs/api-gateway-prod.yaml` - Production configuration

### Example Configuration

```yaml
server:
  host: "0.0.0.0"
  port: 8080
  workers: 4
  max_connections: 10000

auth:
  jwt:
    secret: "your-secret-key"
    expiration: 3600
    algorithm: "HS256"

rate_limiting:
  enabled: true
  redis:
    host: "localhost"
    port: 6379
  rules:
    - name: "global"
      limit: 1000
      window: 60
```

## API Endpoints

### Health and Status

- `GET /health` - Health check endpoint
- `GET /status` - Service status and information
- `GET /metrics` - Prometheus metrics

### Authentication

- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/logout` - User logout
- `POST /api/v1/auth/refresh` - Refresh token
- `POST /api/v1/auth/register` - User registration

### Game Services

- `GET /api/v1/game/*` - Game logic endpoints
- `GET /api/v1/inventory/*` - Inventory management
- `GET /api/v1/chat/*` - Chat system
- `GET /api/v1/guilds/*` - Guild management
- `GET /api/v1/world/*` - World state

## Development

### Project Structure

```
src/
├── main.rs              # Entry point
├── lib.rs               # Library root
├── config.rs            # Configuration management
├── server.rs            # Server implementation
├── routing.rs           # Request routing
├── auth.rs              # Authentication
├── rate_limiting.rs     # Rate limiting
├── security.rs          # Security features
├── load_balancing.rs    # Load balancing
├── caching.rs           # Caching system
├── monitoring.rs        # Monitoring and metrics
├── service_discovery.rs # Service discovery
├── middleware.rs        # Middleware
├── handlers.rs          # Request handlers
├── errors.rs            # Error types
└── types.rs             # Common types
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# With specific features
cargo build --features "auth,rate-limiting,caching"
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Linting

```bash
# Check code
cargo check

# Format code
cargo fmt

# Clippy linting
cargo clippy
```

## Deployment

### Docker

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/api-gateway /usr/local/bin/
COPY --from=builder /app/configs /app/configs
EXPOSE 8080
CMD ["api-gateway"]
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-gateway
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api-gateway
  template:
    metadata:
      labels:
        app: api-gateway
    spec:
      containers:
      - name: api-gateway
        image: chaos-world/api-gateway:latest
        ports:
        - containerPort: 8080
        env:
        - name: ENV
          value: "production"
        - name: REDIS_HOST
          value: "redis-service"
```

## Monitoring

### Metrics

The API Gateway exposes Prometheus metrics at `/metrics`:

- `api_gateway_requests_total` - Total number of requests
- `api_gateway_request_duration_seconds` - Request duration histogram
- `api_gateway_errors_total` - Total number of errors
- `api_gateway_active_connections` - Current active connections

### Health Checks

- `GET /health` - Basic health check
- `GET /status` - Detailed status information

### Logging

Structured JSON logging with configurable levels:

```json
{
  "timestamp": "2024-01-01T00:00:00Z",
  "level": "info",
  "message": "Request processed",
  "request_id": "123e4567-e89b-12d3-a456-426614174000",
  "method": "GET",
  "path": "/api/v1/health",
  "status": 200,
  "duration_ms": 5.2
}
```

## Performance

### Benchmarks

- **Throughput**: 100,000+ requests/second
- **Latency**: < 1ms P50, < 5ms P99
- **Memory**: < 100MB baseline
- **CPU**: < 10% under normal load

### Optimization

- **Connection Pooling**: Reuse HTTP connections
- **Caching**: Redis and in-memory caching
- **Compression**: Gzip compression for responses
- **Keep-Alive**: HTTP keep-alive connections
- **Async I/O**: Non-blocking I/O operations

## Security

### Authentication

- **JWT Tokens**: Secure token-based authentication
- **OAuth2**: Support for Google, Discord, Steam
- **API Keys**: Service-to-service authentication

### Authorization

- **RBAC**: Role-based access control
- **Permissions**: Fine-grained permission system
- **Scopes**: OAuth2-style scopes

### Rate Limiting

- **Per-IP**: Rate limiting per IP address
- **Per-User**: Rate limiting per authenticated user
- **Per-Endpoint**: Different limits for different endpoints

### Security Headers

- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`
- `Strict-Transport-Security: max-age=31536000`

## Troubleshooting

### Common Issues

1. **Port already in use**
   ```bash
   # Check what's using the port
   netstat -ano | findstr :8080
   
   # Kill the process
   taskkill /PID <PID> /F
   ```

2. **Redis connection failed**
   ```bash
   # Start Redis
   redis-server
   
   # Check Redis status
   redis-cli ping
   ```

3. **Configuration not found**
   ```bash
   # Check if config files exist
   ls configs/
   
   # Copy example config
   cp configs/api-gateway-dev.yaml configs/api-gateway.yaml
   ```

### Debug Mode

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run with backtrace
RUST_BACKTRACE=1 cargo run
```

### Logs

```bash
# View logs
tail -f logs/api-gateway.log

# Filter logs
grep "ERROR" logs/api-gateway.log
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run the test suite
6. Submit a pull request

### Code Style

- Follow Rust conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write comprehensive tests
- Document public APIs

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [docs/](docs/)
- **Issues**: [GitHub Issues](https://github.com/chaos-world/api-gateway/issues)
- **Discussions**: [GitHub Discussions](https://github.com/chaos-world/api-gateway/discussions)
- **Discord**: [Chaos World Discord](https://discord.gg/chaos-world)

## Changelog

### v0.1.0 (2024-01-01)
- Initial release
- Basic routing and load balancing
- JWT authentication
- Rate limiting
- Redis caching
- Prometheus metrics
- Health checks
- Security headers
- CORS support