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
| **Aggregator** | Core stat aggregation engine | 🚧 In Development |
| **Caps Provider** | Cap calculation and layer management | 📋 Planned |
| **Registry System** | Configuration and rule management | 📋 Planned |
| **Cache System** | Multi-layer performance optimization | 📋 Planned |
| **Subsystem Interface** | Plugin system for game modules | 📋 Planned |

## 🚀 Quick Start

### For Designers
1. Read [Designs/01_Executive_Summary.md](designs/01_Executive_Summary.md)
2. Study [Designs/03_Domain_Model.md](designs/03_Domain_Model.md)
3. Review [Designs/06_Aggregation_Algorithm.md](designs/06_Aggregation_Algorithm.md)

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
- [ ] Aggregator implementation
- [ ] Caps provider implementation
- [ ] Registry system implementation
- [ ] Basic testing framework

### Phase 3: Performance (Weeks 7-10) 📋
- [ ] Lock-free cache implementation
- [ ] Memory-mapped cache (L2)
- [ ] Persistent cache (L3)
- [ ] Performance optimization

### Phase 4: Advanced Features (Weeks 11-14) 📋
- [ ] Async/await integration
- [ ] Error handling enhancement
- [ ] Comprehensive testing
- [ ] Property-based testing

### Phase 5: Integration (Weeks 15-18) 📋
- [ ] API compatibility layer
- [ ] Deployment and monitoring
- [ ] Documentation completion
- [ ] Production readiness

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

**Last Updated**: 2025-01-27  
**Status**: Design Complete, Implementation In Progress  
**Next Milestone**: Core Services Implementation (Weeks 3-6)
