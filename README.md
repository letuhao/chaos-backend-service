# Chaos World MMORPG Backend Service

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/chaos-world/chaos-backend-service/workflows/CI/badge.svg)](https://github.com/chaos-world/chaos-backend-service/actions)
[![Coverage](https://codecov.io/gh/chaos-world/chaos-backend-service/branch/main/graph/badge.svg)](https://codecov.io/gh/chaos-world/chaos-backend-service)

A high-performance, scalable backend service for the Chaos World MMORPG built in Rust. This repository contains the core modules that power the game's backend infrastructure, providing a robust foundation for character progression, combat systems, world management, and more.

## 🎮 Overview

Chaos World is an immersive MMORPG featuring:
- **Complex Character Systems**: Multi-layered stat aggregation with realm-based progression
- **Dynamic Combat**: Real-time combat with skill-based mechanics
- **Persistent World**: Massive open world with dynamic events and guild systems
- **Advanced Progression**: Multiple leveling systems including cultivation and job specialization
- **Rich Item Economy**: Complex item generation and trading systems

## 🏗️ Architecture

This backend service is built as a modular Rust workspace, with each core module handling specific game systems:

### Core Modules

| Module | Description | Status |
|--------|-------------|--------|
| **actor-core** | Character stat aggregation and management | 🚧 In Development |
| **combat-core** | Combat system, damage calculation, and battle mechanics | 📋 Planned |
| **leveling-core** | Character progression and experience systems | 📋 Planned |
| **race-core** | Race definitions, bonuses, and racial abilities | 📋 Planned |
| **world-core** | World state, zones, and environmental systems | 📋 Planned |
| **event-core** | Event system, quests, and dynamic content | 📋 Planned |
| **item-core** | Item generation, properties, and inventory management | 📋 Planned |
| **job-core** | Job classes, skills, and specialization systems | 📋 Planned |
| **generator-core** | Procedural content generation and world building | 📋 Planned |

### Supporting Modules

| Module | Description | Status |
|--------|-------------|--------|
| **shared** | Common types, utilities, and shared functionality | 📋 Planned |
| **api** | REST, gRPC, and WebSocket API endpoints | 📋 Planned |
| **services** | Business logic and service orchestration | 📋 Planned |
| **tools** | Development tools, migrations, and utilities | 📋 Planned |

## 🚀 Features

### Performance
- **High Throughput**: Designed to handle 100K+ concurrent players
- **Low Latency**: Sub-millisecond response times for critical operations
- **Memory Efficient**: Optimized memory usage with zero-copy operations
- **Scalable**: Horizontal scaling with distributed caching

### Safety & Reliability
- **Memory Safe**: Rust's ownership system prevents memory leaks and data races
- **Type Safe**: Compile-time guarantees for data integrity
- **Fault Tolerant**: Graceful error handling and recovery mechanisms
- **Observable**: Comprehensive logging, metrics, and distributed tracing

### Developer Experience
- **Modular Design**: Clean separation of concerns with trait-based architecture
- **Comprehensive Testing**: Unit, integration, property-based, and fuzz testing
- **Documentation**: Extensive API documentation and examples
- **Hot Reloading**: Development mode with live code reloading

## 🛠️ Technology Stack

### Core Technologies
- **Rust**: Systems programming language for performance and safety
- **Tokio**: Async runtime for high-concurrency applications
- **Serde**: Serialization framework for data interchange
- **SQLx**: Type-safe SQL database access
- **Redis**: High-performance caching and session storage

### Networking & APIs
- **Axum**: Modern web framework for REST APIs
- **Tonic**: gRPC implementation for high-performance RPC
- **WebSocket**: Real-time communication for game clients
- **OpenAPI**: API documentation and client generation

### Monitoring & Observability
- **Tracing**: Structured logging and distributed tracing
- **Prometheus**: Metrics collection and monitoring
- **Jaeger**: Distributed tracing for request flow analysis
- **Grafana**: Visualization and alerting dashboards

## 📦 Installation

### Prerequisites
- Rust 1.75+ (stable)
- PostgreSQL 14+
- Redis 6+
- Docker & Docker Compose (optional)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/chaos-world/chaos-backend-service.git
cd chaos-backend-service

# Install dependencies
cargo build

# Set up environment
cp .env.example .env
# Edit .env with your configuration

# Run database migrations
cargo run --bin migrate

# Start the main backend service
cargo run --bin chaos-backend

# Or start individual microservices
cargo run --bin actor-service
cargo run --bin combat-service
cargo run --bin world-service
cargo run --bin event-service
```

### Docker Development

```bash
# Start all services with Docker Compose
docker-compose up -d

# Run tests
docker-compose exec chaos-backend cargo test

# View logs
docker-compose logs -f chaos-backend
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo tarpaulin --out Html

# Run property-based tests
cargo test --features proptest

# Run fuzz testing
cargo fuzz run <target>

# Run benchmarks
cargo bench
```

## 📚 Documentation

### API Documentation
- [REST API](docs/api/rest.md) - RESTful API endpoints
- [gRPC API](docs/api/grpc.md) - High-performance RPC interface
- [WebSocket API](docs/api/websocket.md) - Real-time communication

### Module Documentation
- [Actor Core](docs/actor-core/) - Character stat system
- [Combat Core](docs/combat-core/) - Combat mechanics
- [Leveling Core](docs/leveling-core/) - Progression systems
- [Migration Guide](docs/actor-core/migrations/) - Go to Rust migration

### Development Guides
- [Contributing](CONTRIBUTING.md) - How to contribute
- [Architecture](docs/architecture.md) - System design overview
- [Performance](docs/performance.md) - Optimization guidelines
- [Deployment](docs/deployment.md) - Production deployment

## 🔧 Development

### Project Structure
```
chaos-backend-service/
├── Cargo.toml           # Workspace configuration
├── crates/              # Library crates
│   ├── shared/          # Shared utilities and types
│   ├── actor-core/      # Character stat aggregation
│   ├── combat-core/     # Combat system
│   ├── leveling-core/   # Progression systems
│   ├── race-core/       # Race definitions
│   ├── world-core/      # World management
│   ├── event-core/      # Event system
│   ├── item-core/       # Item management
│   ├── job-core/        # Job classes
│   ├── generator-core/  # Content generation
│   └── api/             # API layer
├── services/            # Binary services
│   ├── chaos-backend/   # Main backend service
│   ├── actor-service/   # Actor microservice
│   ├── combat-service/  # Combat microservice
│   ├── world-service/   # World microservice
│   └── event-service/   # Event microservice
├── tools/               # Development tools
│   ├── migrate/         # Database migration tool
│   ├── load-test/       # Load testing tool
│   ├── data-gen/        # Data generation tool
│   └── admin-cli/       # Administrative CLI
├── docs/                # Documentation
└── tests/               # Integration tests
```

### Code Quality
- **Clippy**: Rust linter for code quality
- **Rustfmt**: Code formatting
- **Cargo Audit**: Security vulnerability scanning
- **Cargo Deny**: License compliance checking

### Performance
- **Criterion**: Benchmarking framework
- **Flamegraph**: CPU profiling
- **Heaptrack**: Memory profiling
- **Perf**: System-level profiling

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run the test suite
6. Submit a pull request

### Code Standards
- Follow Rust naming conventions
- Write comprehensive tests
- Document public APIs
- Use meaningful commit messages
- Keep PRs focused and atomic

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Rust Community**: For the amazing ecosystem and tools
- **Tokio Team**: For the excellent async runtime
- **Game Development Community**: For inspiration and best practices
- **Open Source Contributors**: For the libraries that make this possible

## 📞 Support

- **Documentation**: [docs.chaos-world.com](https://docs.chaos-world.com)
- **Discord**: [discord.gg/chaos-world](https://discord.gg/chaos-world)
- **Issues**: [GitHub Issues](https://github.com/chaos-world/chaos-backend-service/issues)
- **Discussions**: [GitHub Discussions](https://github.com/chaos-world/chaos-backend-service/discussions)

---

**Built with ❤️ by the Chaos World Team**