# Actor Core Documentation

**Actor Core v3** - High-performance character stat aggregation system for Chaos World MMORPG

## 📁 Documentation Structure

### 🎯 [Designs](designs/)
Complete design documentation migrated from the original Go implementation:
- **Core Design**: Architecture, domain models, algorithms (26 documents)
- **Implementation Guides**: Step-by-step implementation instructions
- **Test Vectors**: Golden test cases and validation data (10 test cases)
- **Schemas**: JSON schemas for data validation (8 schemas)
- **Examples**: Usage examples and configuration templates
- **Status**: ✅ Cleaned and optimized for Rust implementation

### 🚀 [Migrations](migrations/)
Go to Rust migration documentation:
- **Migration Plan**: Comprehensive 18-week migration strategy
- **Implementation Guide**: Rust-specific technical details
- **Performance Comparison**: Go vs Rust performance analysis
- **Migration Checklist**: Detailed task breakdown

## 🏗️ Architecture Overview

Actor Core v3 is a metadata-only aggregator system that:

- **Collects Contributions**: Gathers stat modifications from various subsystems
- **Applies Caps**: Manages min/max limits across multiple layers
- **Aggregates Stats**: Combines contributions using configurable merge rules
- **Produces Snapshots**: Generates final character stat states

### Key Components

| Component | Description | Status |
|-----------|-------------|--------|
| **Aggregator** | Core stat aggregation engine | ✅ Implemented |
| **Caps Provider** | Cap calculation and layer management | ✅ Implemented |
| **Registry System** | Configuration and rule management | ✅ Implemented |
| **Cache System** | In-memory cache implemented; advanced multi-layer optional | ✅ Implemented (basic) |
| **Subsystem Interface** | Plugin system for game modules | ✅ Implemented |

## 🚀 Quick Start

### For Designers
1. Read [Designs/01_Executive_Summary.md](designs/01_Executive_Summary.md)
2. Study [Designs/03_Domain_Model.md](designs/03_Domain_Model.md)
3. Review [Designs/06_Aggregation_Algorithm.md](designs/06_Aggregation_Algorithm.md)
4. See examples: [appendix/Combiner.operator_examples.yaml](designs/appendix/Combiner.operator_examples.yaml), [appendix/CapLayerRegistry.layered_examples.yaml](designs/appendix/CapLayerRegistry.layered_examples.yaml)

### For Developers
1. Follow [Migrations/GO_TO_RUST_MIGRATION_PLAN.md](migrations/GO_TO_RUST_MIGRATION_PLAN.md)
2. Study [Migrations/RUST_IMPLEMENTATION_GUIDE.md](migrations/RUST_IMPLEMENTATION_GUIDE.md)
3. Use [Migrations/MIGRATION_CHECKLIST.md](migrations/MIGRATION_CHECKLIST.md)

### For Testers
1. Review [Designs/golden_vectors/](designs/golden_vectors/)
2. Study [Designs/schemas/](designs/schemas/)
3. Follow [Designs/14_Testing_Strategy.md](designs/14_Testing_Strategy.md)

## 📊 Performance Targets

| Metric | Go Implementation | Rust Target | Improvement |
|--------|------------------|-------------|-------------|
| **Throughput** | 50K ops/sec | 150K ops/sec | 3x |
| **Latency** | 15ms p99 | 3ms p99 | 80% |
| **Memory** | 1GB | 600MB | 40% |
| **Concurrency** | 1K users | 5K users | 5x |

## 🔧 Implementation Status

### Phase 1: Foundation (Weeks 1-2) ✅
- [x] Project structure setup
- [x] Documentation migration
- [x] Type system design
- [x] Basic trait definitions

### Phase 2: Core Services (Weeks 3-6) 🚧
- [x] Aggregator implementation
- [x] Caps provider implementation
- [x] Registry system implementation
- [x] Basic testing framework

### Phase 3: Performance (Weeks 7-10) 📋
- [x] Lock-free cache implementation (optional advanced)
- [x] Memory-mapped cache (L2) (optional advanced)
- [x] Persistent cache (L3) (optional advanced)
- [x] Performance optimization

### Phase 4: Advanced Features (Weeks 11-14) 📋
- [x] Async/await integration
- [x] Error handling enhancement
- [x] Comprehensive testing
- [x] Property-based testing

### Phase 5: Integration (Weeks 15-18) 📋
- [x] API compatibility layer
- [x] Deployment and monitoring
- [x] Documentation completion
- [ ] Production readiness

### Production readiness checklist
- Add health/readiness endpoints in your service to call `actor_core::production::check_readiness(...)`
- Validate registries at startup; fail fast on invalid configuration
- Configure cache via env (basic/lock_free/multi; Redis URL optional)
- Enable structured logging (tracing) and error aggregation
- Set alerts on cache miss spikes and aggregation latency

#### Example: readiness probe in a service

```rust
use actor_core::{RegistryFactory, ServiceFactory, CacheFactory};
use actor_core::production::check_readiness;

fn readiness_probe() -> Result<(), String> {
    // Build registries and services
    let plugin = RegistryFactory::create_plugin_registry();
    let combiner = RegistryFactory::create_combiner_registry();
    let cap_layers = RegistryFactory::create_cap_layer_registry();
    let caps = ServiceFactory::create_caps_provider(cap_layers);

    // Pick a cache (basic/lock_free/multi)
    let cache = CacheFactory::create_default_multi_layer_cache();

    // Perform readiness checks
    check_readiness(plugin.as_ref(), combiner.as_ref(), caps.as_ref(), cache.as_ref())
        .map_err(|e| e.to_string())
}

// In your web framework handler:
// if readiness_probe().is_ok() { return 200 OK } else { return 503 Service Unavailable }
```

## 🧪 Testing Strategy

### Test Types
- **Unit Tests**: Individual component testing
- **Integration Tests**: Component interaction testing
- **Property Tests**: Automated property validation
- **Golden Tests**: Reference implementation validation
- **Performance Tests**: Benchmark and load testing

### Test Data
- **Golden Vectors**: 10 comprehensive test cases
- **JSON Schemas**: Data validation schemas
- **Example Configs**: Configuration templates
- **Test Vectors**: Edge case and stress testing

## 🔧 Configuration Examples

- Combiner rules (pipeline and operator-mode):
  - `designs/appendix/Combiner.operator_examples.yaml`
- Cap layer registry (REALM/WORLD/EVENT/TOTAL with INTERSECT policy):
  - `designs/appendix/CapLayerRegistry.layered_examples.yaml`

To load examples at runtime:

```powershell
$env:ACTOR_CORE_CONFIG_DIR = (Resolve-Path ./docs/actor-core/designs/appendix)
cargo test -p actor-core
```

## ⚙️ Cache Options: How to switch caches

```rust
use actor_core::{RegistryFactory, ServiceFactory, CacheFactory};

// Registries and services
let plugin_registry = RegistryFactory::create_plugin_registry();
let combiner_registry = RegistryFactory::create_combiner_registry();
let cap_layer_registry = RegistryFactory::create_cap_layer_registry();
let caps_provider = ServiceFactory::create_caps_provider(cap_layer_registry);

// 1) Basic in-memory cache
let cache = CacheFactory::create_in_memory_cache(100_000, 600);

// 2) Lock-free in-memory cache (DashMap-based)
// let cache = CacheFactory::create_lock_free_in_memory_cache(100_000, 300);

// 3) Default multi-layer cache (L1 lock-free, L2 in-memory, L3 optional Redis)
// std::env::set_var("ACTOR_CORE_REDIS_URL", "redis://127.0.0.1:6379"); // optional
// let cache = CacheFactory::create_default_multi_layer_cache();

// Aggregator with selected cache
let aggregator = ServiceFactory::create_aggregator(
    plugin_registry,
    combiner_registry,
    caps_provider,
    cache,
);
```

### Benchmarking cache options

```powershell
# Basic benches
cargo bench -p actor-core

# Quick compare: run with basic in-memory
$env:ACTOR_CORE_CACHE_KIND = "basic"
cargo bench -p actor-core -- bench_cache

# Compare lock-free
$env:ACTOR_CORE_CACHE_KIND = "lock_free"
cargo bench -p actor-core -- bench_cache

# Compare multi-layer (optionally set Redis URL)
$env:ACTOR_CORE_CACHE_KIND = "multi"
$env:ACTOR_CORE_REDIS_URL = "redis://127.0.0.1:6379"
cargo bench -p actor-core -- bench_cache
```

Notes:
- The environment variable `ACTOR_CORE_CACHE_KIND` is a suggested toggle; wire it in your app when constructing the cache (pick the factory accordingly).
- Use the existing `performance/benchmarks.rs` or add a small `bench_cache` that batters get/set patterns for the chosen cache.

## 🧯 Error taxonomy

Actor Core centralizes errors via `ActorCoreError` and returns `ActorCoreResult<T>` across APIs.

- InvalidActor: bad or incomplete actor data (empty name/race, invalid version)
- InvalidContribution: dimension empty, non-finite value, negative priority
- InvalidCap: malformed cap contribution (min>max, invalid mode/kind)
- SubsystemError: a subsystem’s `contribute` failed; aggregator logs and continues
- CacheError: cache set/get/delete/sync failures (e.g., filesystem/Redis issues)
- RegistryError: duplicate/missing subsystem, invalid priority, bad rules
- AggregationError: unexpected aggregation state or configuration mismatch
- ConfigurationError: invalid loader config, bad paths, invalid YAML/JSON
- InvalidInput: generic validation failure for API inputs
- Shared(ChaosError): wrapped errors bubbling from `shared` crate

Recommended action: fix inputs or config; for operational errors (Cache/Subsystem), check logs and environment. All errors are non-panicking and surfaced as Results.

## 📚 Key Concepts

### Stat Aggregation
- **Contributions**: Stat modifications from subsystems
- **Buckets**: FLAT, MULT, POST_ADD, OVERRIDE, etc.
- **Merge Rules**: Pipeline vs Operator-based aggregation
- **Priority**: Contribution ordering and precedence

### Cap System
- **Layers**: REALM, WORLD, EVENT, GUILD, TOTAL
- **Modes**: BASELINE, ADDITIVE, HARD_MAX, HARD_MIN, OVERRIDE
- **Policies**: INTERSECT, UNION, PRIORITIZED_OVERRIDE
- **Effective Caps**: Final calculated limits

### Context Modifiers
- **Additive Percent**: Percentage-based additions
- **Multipliers**: Stat multiplication factors
- **Post Add**: Final additive modifications
- **Temporary Effects**: Event-based stat changes

## 🔗 Related Systems

- **[Combat Core](../../crates/combat-core/)** - Combat mechanics and damage calculation
- **[Leveling Core](../../crates/leveling-core/)** - Character progression systems
- **[Race Core](../../crates/race-core/)** - Race definitions and bonuses
- **[World Core](../../crates/world-core/)** - World state and zone management
- **[Event Core](../../crates/event-core/)** - Event system and quests

## 📞 Support

- **Documentation**: This directory contains all design and implementation docs
- **Issues**: Use GitHub issues for bug reports and feature requests
- **Discussions**: Use GitHub discussions for questions and design discussions
- **Migration**: Follow the migration plan for Go to Rust transition

---

**Last Updated**: 2025-09-09  
**Status**: Design Complete, Core Services Implemented  
**Next Milestone**: Optional Performance (Advanced Cache) & Production Readiness
