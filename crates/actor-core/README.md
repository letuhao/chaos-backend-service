# Actor Core

[![Rust](https://img.shields.io/badge/rust-1.89+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/chaos-world/actor-core/workflows/CI/badge.svg)](https://github.com/chaos-world/actor-core/actions)

A high-performance character stat aggregation and management system for the Chaos World MMORPG, built in Rust.

## 🚀 Features

- **High-Performance Stat Aggregation**: Efficient processing of character stats with deterministic ordering
- **Flexible Bucket System**: Support for different stat processing modes (Flat, Mult, PostAdd, Override)
- **Caps Management**: Comprehensive min/max value constraints with layer-based policies
- **Async Support**: Full async/await support for non-blocking operations
- **Caching**: Built-in caching system for performance optimization
- **Configuration Loading**: YAML/JSON configuration support for game rules
- **Comprehensive Testing**: Unit tests, integration tests, property-based tests, and benchmarks
- **CI/CD Ready**: Full CI pipeline with formatting, linting, security auditing, and performance monitoring

## 📦 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
actor-core = "0.1.0"
```

### Basic Usage

```rust
use actor_core::*;
use std::collections::HashMap;

// Create an actor
let mut actor = Actor::new("Player1".to_string(), "Human".to_string());

// Add some data
let mut data = HashMap::new();
data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(10)));
actor.set_data(data);

// Add a buff
actor.add_buff("strength_boost".to_string());

// Create contributions
let contribution = Contribution::new(
    "strength".to_string(),
    Bucket::Flat,
    10.0,
    "equipment".to_string()
);

// Process contributions
let result = bucket_processor::process_contributions_in_order(
    vec![contribution],
    0.0,
    None
);
```

## 🏗️ Architecture

### Core Components

- **Actor**: Represents a character with stats, buffs, and subsystems
- **Contribution**: Individual stat modifications from various sources
- **Caps**: Min/max constraints for stat values
- **Snapshot**: Aggregated stat state at a point in time
- **Subsystem**: Modular stat processing components

### Bucket System

The bucket system determines how contributions are processed:

- **Flat**: Additive contributions (equipment bonuses)
- **Mult**: Multiplicative contributions (percentage bonuses)
- **PostAdd**: Post-addition contributions (final adjustments)
- **Override**: Override contributions (replaces previous values)

### Feature Flags

- `extra_buckets`: Enables additional bucket types (Exponential, Logarithmic, Conditional)

## 🔧 Configuration

### Cap Layers Configuration

```yaml
# configs/cap_layers.yaml
cap_layers:
  - name: base
    priority: 100
    cap_mode: BASELINE
  - name: equipment
    priority: 200
    cap_mode: ADDITIVE
  - name: buffs
    priority: 300
    cap_mode: HARD_MAX

across_layer_policy: STRICT
```

### Combiner Configuration

```yaml
# configs/combiner.yaml
combiner_rules:
  - name: attack
    bucket_order: [Flat, Mult, PostAdd, Override]
    clamp: true
  - name: defense
    bucket_order: [Flat, Mult, PostAdd, Override]
    clamp: true
```

## 🧪 Testing

The crate includes comprehensive testing:

```bash
# Run all tests
cargo test

# Run with extra features
cargo test --features extra_buckets

# Run specific test suites
cargo test --test actor_tests
cargo test --test caps_tests
cargo test --test property_tests
cargo test --test edge_case_tests
```

### Test Categories

- **Unit Tests**: Individual component testing
- **Integration Tests**: End-to-end functionality testing
- **Property Tests**: Mathematical property validation
- **Edge Case Tests**: Boundary condition testing
- **Performance Tests**: Performance benchmark validation

## 📊 Benchmarking

Run performance benchmarks:

```bash
# Run all benchmarks
cargo bench

# Run quick benchmarks
make bench-quick

# Run comprehensive benchmarks
make bench-comprehensive

# Run specific benchmark suites
cargo bench --bench simple_benchmarks
cargo bench --bench actor_benchmarks
```

## 🛠️ Development

### Prerequisites

- Rust 1.89+
- Cargo
- Make (optional, for convenience commands)

### Building

```bash
# Build the project
cargo build

# Build with extra features
cargo build --features extra_buckets

# Build for release
cargo build --release
```

### Code Quality

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy

# Run all CI checks
make ci
```

### Available Make Commands

```bash
make help              # Show all available commands
make test              # Run all tests
make bench             # Run all benchmarks
make format            # Format code
make clippy            # Run clippy lints
make docs              # Generate documentation
make ci                # Run all CI checks
```

## 📚 API Documentation

### Core Types

- [`Actor`](https://docs.rs/actor-core/latest/actor_core/struct.Actor.html) - Character representation
- [`Contribution`](https://docs.rs/actor-core/latest/actor_core/struct.Contribution.html) - Stat modification
- [`Caps`](https://docs.rs/actor-core/latest/actor_core/struct.Caps.html) - Value constraints
- [`Snapshot`](https://docs.rs/actor-core/latest/actor_core/struct.Snapshot.html) - Aggregated state

### Traits

- [`Subsystem`](https://docs.rs/actor-core/latest/actor_core/trait.Subsystem.html) - Stat processing component
- [`Aggregator`](https://docs.rs/actor-core/latest/actor_core/trait.Aggregator.html) - Stat aggregation
- [`CapsProvider`](https://docs.rs/actor-core/latest/actor_core/trait.CapsProvider.html) - Caps management

### Enums

- [`Bucket`](https://docs.rs/actor-core/latest/actor_core/enum.Bucket.html) - Processing modes
- [`CapMode`](https://docs.rs/actor-core/latest/actor_core/enum.CapMode.html) - Constraint types
- [`Operator`](https://docs.rs/actor-core/latest/actor_core/enum.Operator.html) - Mathematical operations

## 🔒 Security

The crate includes security auditing:

```bash
# Run security audit
cargo audit

# Check for outdated dependencies
cargo outdated
```

## 📈 Performance

The system is designed for high performance:

- **Zero-copy operations** where possible
- **Efficient memory management** with custom allocators
- **SIMD optimizations** for mathematical operations
- **Comprehensive caching** to avoid redundant calculations
- **Async processing** for non-blocking operations

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust naming conventions
- Add tests for new functionality
- Update documentation for API changes
- Run `make ci` before submitting PRs
- Use conventional commit messages

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built for the Chaos World MMORPG project
- Inspired by modern game development practices
- Powered by the Rust programming language

## 📞 Support

For questions, issues, or contributions:

- Open an issue on GitHub
- Join our Discord community
- Check the documentation

---

**Made with ❤️ for the Chaos World MMORPG**
