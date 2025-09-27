# Quick Reference Guide - Actor Core Hierarchical Migration

## 🚀 **Quick Start Checklist**

### **Before Starting:**
- [ ] Read `SYSTEM_BOUNDARY_ANALYSIS.md`
- [ ] Read `EXTENSIBLE_HIERARCHICAL_DESIGN.md`
- [ ] Create backup of current implementation
- [ ] Understand current elemental components

### **Phase 1: Element Core Enhancement (2 days)**
- [ ] Create element-core directory structure
- [ ] Move elemental_data.rs to element-core/src/core/
- [ ] Create elemental_system.rs
- [ ] Create elemental_registry.rs
- [ ] Create elemental_factory.rs
- [ ] Create elemental_aggregator.rs
- [ ] Create elemental_config_loader.rs
- [ ] Update element-core Cargo.toml
- [ ] Test element-core independently

### **Phase 2: Actor Core Hierarchical Refactoring (2 days)**
- [ ] Remove moved components from actor-core-hierarchical
- [ ] Create elemental_actor_integration.rs
- [ ] Create new elemental_adapter.rs
- [ ] Update global_aggregator.rs
- [ ] Update actor-core-hierarchical Cargo.toml
- [ ] Test integration layer

### **Phase 3: Integration & Testing (2 days)**
- [ ] Update integration tests
- [ ] Create new tests
- [ ] Update examples
- [ ] Run full test suite
- [ ] Performance testing

## 📁 **File Structure Quick Reference**

### **Element Core Structure:**
```
element-core/src/
├── core/
│   ├── elemental_system.rs         # Elemental system logic
│   ├── elemental_data.rs           # Elemental data structures
│   └── elemental_config.rs         # Elemental configuration
├── registry/
│   ├── elemental_registry.rs       # Elemental registry
│   └── elemental_registry_manager.rs
├── factory/
│   ├── elemental_factory.rs        # Elemental factory
│   └── elemental_builder.rs        # Elemental builder
├── aggregation/
│   ├── elemental_aggregator.rs     # Elemental aggregation
│   └── elemental_stats_calculator.rs
├── config/
│   ├── elemental_config_loader.rs  # Config loading
│   └── elemental_config_validator.rs
└── adapters/
    ├── elemental_serializer.rs     # Serialization
    └── elemental_validator.rs      # Validation
```

### **Actor Core Hierarchical Structure:**
```
actor-core-hierarchical/src/
├── core/
│   ├── hierarchical_actor.rs       # Main hierarchical actor
│   ├── global_aggregator.rs        # Global stats aggregation
│   └── actor_factory.rs            # Actor creation factory
├── systems/
│   └── elemental/
│       ├── elemental_actor_integration.rs  # Actor-elemental bridge
│       └── elemental_adapter.rs    # Actor-elemental conversion
├── adapters/
│   └── actor_adapter.rs            # Actor conversion adapter
└── aggregation/
    └── global_aggregator.rs        # Global stats aggregation
```

## 🔧 **Key Commands**

### **Backup Commands:**
```bash
# Create backup
cp -r chaos-backend-service/crates/actor-core-hierarchical chaos-backend-service/crates/actor-core-hierarchical-backup
cp -r chaos-backend-service/crates/element-core chaos-backend-service/crates/element-core-backup
```

### **Testing Commands:**
```bash
# Test element-core independently
cd chaos-backend-service/crates/element-core
cargo test

# Test actor-core-hierarchical
cd chaos-backend-service/crates/actor-core-hierarchical
cargo test

# Test integration
cargo test --all-features
```

### **Build Commands:**
```bash
# Build element-core
cd chaos-backend-service/crates/element-core
cargo build

# Build actor-core-hierarchical
cd chaos-backend-service/crates/actor-core-hierarchical
cargo build
```

## 📋 **Migration Checklist**

### **Files to Move:**
- [ ] `actor-core-hierarchical/src/types/elemental_data.rs` → `element-core/src/core/elemental_data.rs`
- [ ] `actor-core-hierarchical/src/systems/elemental.rs` → `element-core/src/core/elemental_system.rs`
- [ ] `actor-core-hierarchical/src/adapters/elemental_adapter.rs` → `element-core/src/adapters/elemental_serializer.rs`
- [ ] `actor-core-hierarchical/src/aggregation/elemental_aggregator.rs` → `element-core/src/aggregation/elemental_aggregator.rs`

### **Files to Create:**
- [ ] `element-core/src/registry/elemental_registry.rs`
- [ ] `element-core/src/factory/elemental_factory.rs`
- [ ] `element-core/src/config/elemental_config_loader.rs`
- [ ] `actor-core-hierarchical/src/systems/elemental/elemental_actor_integration.rs`
- [ ] `actor-core-hierarchical/src/systems/elemental/elemental_adapter.rs`

### **Files to Update:**
- [ ] `element-core/Cargo.toml`
- [ ] `element-core/src/lib.rs`
- [ ] `actor-core-hierarchical/Cargo.toml`
- [ ] `actor-core-hierarchical/src/lib.rs`

## 🚨 **Critical Points**

### **1. Dependency Direction:**
- Element-core should NOT depend on actor-core-hierarchical
- Actor-core-hierarchical should depend on element-core
- Use trait-based interfaces for loose coupling

### **2. Error Handling:**
- Maintain consistent error types
- Use ActorCoreResult for compatibility
- Handle integration errors gracefully

### **3. Performance:**
- Minimize data copying between layers
- Use Arc<Mutex<>> for shared data
- Optimize hot paths

## 🎯 **Success Criteria**

### **Element Core Complete When:**
- [ ] Compiles independently
- [ ] All tests pass
- [ ] No dependencies on actor-core-hierarchical

### **Actor Core Hierarchical Complete When:**
- [ ] Compiles with element-core dependency
- [ ] Integration tests pass
- [ ] Performance benchmarks meet requirements

### **Full Integration Complete When:**
- [ ] All tests pass
- [ ] Documentation updated
- [ ] Examples working
- [ ] Performance acceptable

## 📞 **Troubleshooting**

### **Common Issues:**
1. **Circular Dependencies**: Check Cargo.toml dependencies
2. **Import Errors**: Verify module exports in lib.rs
3. **Test Failures**: Check trait implementations
4. **Performance Issues**: Profile integration layer

### **Debug Commands:**
```bash
# Check for compilation errors
cargo check

# Run tests with verbose output
cargo test --verbose

# Check dependencies
cargo tree

# Check features
cargo check --features
```

---

**Use this quick reference alongside the detailed implementation guide for efficient migration.** 🎯
