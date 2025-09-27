# Actor Core Hierarchical - Implementation Guide

## 📋 **Overview**

This guide provides a systematic approach to implementing the corrected actor-core-hierarchical architecture with proper system boundaries and component placement.

## 🎯 **Goals**

1. **Move elemental components** from `actor-core-hierarchical` to `element-core`
2. **Create integration layer** in `actor-core-hierarchical` for system coordination
3. **Establish proper system boundaries** between different cores
4. **Enable extensibility** for future systems (cultivation, magic, race)

## 📚 **Required Reading (Before Implementation)**

### **1. Core Design Documents:**
- `chaos-backend-service/docs/actor-core/SYSTEM_BOUNDARY_ANALYSIS.md` - System boundary analysis
- `chaos-backend-service/docs/actor-core/EXTENSIBLE_HIERARCHICAL_DESIGN.md` - Extensible design
- `chaos-backend-service/docs/actor-core/ACTOR_CORE_COMPLETENESS_ANALYSIS.md` - Completeness analysis

### **2. Source Code to Understand:**
- `chaos-backend-service/crates/actor-core/src/types/inheritable.rs` - Inheritance traits
- `chaos-backend-service/crates/actor-core-hierarchical/src/types/elemental_data.rs` - Current elemental data
- `chaos-backend-service/crates/actor-core-hierarchical/src/systems/elemental.rs` - Current elemental system
- `chaos-backend-service/crates/actor-core-hierarchical/src/adapters/elemental_adapter.rs` - Current adapter

### **3. Element Core Structure:**
- `chaos-backend-service/crates/element-core/src/` - Current element-core structure
- `chaos-backend-service/docs/element-core/` - Element-core documentation

## 🏗️ **Target Architecture**

### **Final Structure:**
```
actor-core-hierarchical/src/        # Actor Management Layer
├── core/
│   ├── hierarchical_actor.rs       # Main hierarchical actor
│   ├── global_aggregator.rs        # Global stats aggregation
│   └── actor_factory.rs            # Actor creation factory
├── systems/
│   ├── elemental/
│   │   ├── elemental_actor_integration.rs  # Actor-elemental bridge
│   │   └── elemental_adapter.rs    # Actor-elemental conversion
│   ├── cultivation/                # Future systems
│   ├── magic/
│   └── race/
├── adapters/
│   ├── base_adapter.rs             # Base adapter trait
│   └── actor_adapter.rs            # Actor conversion adapter
└── aggregation/
    ├── base_aggregator.rs          # Base aggregation logic
    └── global_aggregator.rs        # Global stats aggregation

element-core/src/                   # Elemental System Layer
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

## 📋 **Implementation Steps**

### **Phase 1: Preparation & Analysis (Day 1)**

#### **Step 1.1: Read Required Documents**
```bash
# Read these documents in order:
1. chaos-backend-service/docs/actor-core/SYSTEM_BOUNDARY_ANALYSIS.md
2. chaos-backend-service/docs/actor-core/EXTENSIBLE_HIERARCHICAL_DESIGN.md
3. chaos-backend-service/docs/actor-core/ACTOR_CORE_COMPLETENESS_ANALYSIS.md
```

#### **Step 1.2: Analyze Current Code**
```bash
# Review current implementations:
1. chaos-backend-service/crates/actor-core-hierarchical/src/types/elemental_data.rs
2. chaos-backend-service/crates/actor-core-hierarchical/src/systems/elemental.rs
3. chaos-backend-service/crates/actor-core-hierarchical/src/adapters/elemental_adapter.rs
4. chaos-backend-service/crates/actor-core-hierarchical/src/aggregation/elemental_aggregator.rs
```

#### **Step 1.3: Understand Element Core Structure**
```bash
# Review element-core current structure:
1. chaos-backend-service/crates/element-core/src/
2. chaos-backend-service/crates/element-core/Cargo.toml
3. chaos-backend-service/docs/element-core/
```

#### **Step 1.4: Create Backup**
```bash
# Create backup of current implementation:
cp -r chaos-backend-service/crates/actor-core-hierarchical chaos-backend-service/crates/actor-core-hierarchical-backup
cp -r chaos-backend-service/crates/element-core chaos-backend-service/crates/element-core-backup
```

### **Phase 2: Element Core Enhancement (Days 2-3)**

#### **Step 2.1: Update Element Core Structure**
```bash
# Navigate to element-core
cd chaos-backend-service/crates/element-core/src

# Create new directories
mkdir -p core registry factory aggregation config adapters
```

#### **Step 2.2: Move Elemental Data Structures**
```bash
# Move from actor-core-hierarchical to element-core:
# Source: chaos-backend-service/crates/actor-core-hierarchical/src/types/elemental_data.rs
# Target: chaos-backend-service/crates/element-core/src/core/elemental_data.rs

# Update imports and dependencies
```

#### **Step 2.3: Create Elemental System Core**
```rust
// File: chaos-backend-service/crates/element-core/src/core/elemental_system.rs
// Create elemental system logic
// Implement elemental-specific functionality
// Handle elemental data management
```

#### **Step 2.4: Create Elemental Registry**
```rust
// File: chaos-backend-service/crates/element-core/src/registry/elemental_registry.rs
// Move registry logic from actor-core-hierarchical
// Implement elemental-specific registry management
// Handle elemental system registration
```

#### **Step 2.5: Create Elemental Factory & Builder**
```rust
// File: chaos-backend-service/crates/element-core/src/factory/elemental_factory.rs
// File: chaos-backend-service/crates/element-core/src/factory/elemental_builder.rs
// Move factory/builder logic from actor-core-hierarchical
// Implement elemental object creation
// Handle elemental configuration
```

#### **Step 2.6: Create Elemental Aggregator**
```rust
// File: chaos-backend-service/crates/element-core/src/aggregation/elemental_aggregator.rs
// Move aggregation logic from actor-core-hierarchical
// Implement elemental stats calculation
// Handle elemental contribution processing
```

#### **Step 2.7: Create Elemental Config System**
```rust
// File: chaos-backend-service/crates/element-core/src/config/elemental_config_loader.rs
// File: chaos-backend-service/crates/element-core/src/config/elemental_config_validator.rs
// Create elemental configuration management
// Handle config loading and validation
```

#### **Step 2.8: Create Elemental Adapters**
```rust
// File: chaos-backend-service/crates/element-core/src/adapters/elemental_serializer.rs
// File: chaos-backend-service/crates/element-core/src/adapters/elemental_validator.rs
// Create elemental serialization/validation
// Handle data conversion
```

#### **Step 2.9: Update Element Core Cargo.toml**
```toml
# Add new dependencies if needed
# Update feature flags
# Add new modules
```

#### **Step 2.10: Update Element Core lib.rs**
```rust
// Export new modules
// Update public API
// Add new re-exports
```

### **Phase 3: Actor Core Hierarchical Refactoring (Days 4-5)**

#### **Step 3.1: Remove Moved Components**
```bash
# Remove from actor-core-hierarchical:
rm chaos-backend-service/crates/actor-core-hierarchical/src/types/elemental_data.rs
rm chaos-backend-service/crates/actor-core-hierarchical/src/systems/elemental.rs
rm chaos-backend-service/crates/actor-core-hierarchical/src/adapters/elemental_adapter.rs
rm chaos-backend-service/crates/actor-core-hierarchical/src/aggregation/elemental_aggregator.rs
```

#### **Step 3.2: Create Integration Layer**
```rust
// File: chaos-backend-service/crates/actor-core-hierarchical/src/systems/elemental/elemental_actor_integration.rs
// Create bridge between actor and elemental system
// Handle actor-elemental coordination
// Implement integration logic
```

#### **Step 3.3: Create New Elemental Adapter**
```rust
// File: chaos-backend-service/crates/actor-core-hierarchical/src/systems/elemental/elemental_adapter.rs
// Create actor-elemental conversion adapter
// Handle data transformation between layers
// Implement adapter pattern
```

#### **Step 3.4: Update Global Aggregator**
```rust
// File: chaos-backend-service/crates/actor-core-hierarchical/src/aggregation/global_aggregator.rs
// Update to use elemental integration
// Handle multi-system aggregation
// Implement global stats calculation
```

#### **Step 3.5: Update Actor Core Hierarchical Cargo.toml**
```toml
# Add element-core as dependency
# Update feature flags
# Add new modules
```

#### **Step 3.6: Update Actor Core Hierarchical lib.rs**
```rust
// Update imports
// Export new modules
// Update public API
```

### **Phase 4: Integration & Testing (Days 6-7)**

#### **Step 4.1: Update Integration Tests**
```rust
// Update existing tests to use new architecture
// Test elemental integration
// Test global aggregation
```

#### **Step 4.2: Create New Tests**
```rust
// Test elemental actor integration
// Test elemental adapter functionality
// Test global aggregator with elemental system
```

#### **Step 4.3: Update Examples**
```rust
// Update examples to use new architecture
// Demonstrate elemental integration
// Show multi-system usage
```

#### **Step 4.4: Update Benchmarks**
```rust
// Update benchmarks for new architecture
// Test performance of integration layer
// Compare with old implementation
```

#### **Step 4.5: Integration Testing**
```bash
# Run full integration tests
cargo test --all-features
cargo test --package actor-core-hierarchical
cargo test --package element-core
```

### **Phase 5: Documentation & Cleanup (Day 8)**

#### **Step 5.1: Update Documentation**
```bash
# Update README files
# Update API documentation
# Update examples
```

#### **Step 5.2: Cleanup**
```bash
# Remove unused code
# Clean up imports
# Remove deprecated functions
```

#### **Step 5.3: Final Testing**
```bash
# Run complete test suite
# Performance testing
# Integration testing
```

## 🔧 **Detailed Implementation Files**

### **Element Core Files to Create:**

#### **1. Core System Files:**
```rust
// chaos-backend-service/crates/element-core/src/core/elemental_system.rs
// chaos-backend-service/crates/element-core/src/core/elemental_data.rs
// chaos-backend-service/crates/element-core/src/core/elemental_config.rs
```

#### **2. Registry Files:**
```rust
// chaos-backend-service/crates/element-core/src/registry/elemental_registry.rs
// chaos-backend-service/crates/element-core/src/registry/elemental_registry_manager.rs
// chaos-backend-service/crates/element-core/src/registry/mod.rs
```

#### **3. Factory Files:**
```rust
// chaos-backend-service/crates/element-core/src/factory/elemental_factory.rs
// chaos-backend-service/crates/element-core/src/factory/elemental_builder.rs
// chaos-backend-service/crates/element-core/src/factory/mod.rs
```

#### **4. Aggregation Files:**
```rust
// chaos-backend-service/crates/element-core/src/aggregation/elemental_aggregator.rs
// chaos-backend-service/crates/element-core/src/aggregation/elemental_stats_calculator.rs
// chaos-backend-service/crates/element-core/src/aggregation/mod.rs
```

#### **5. Config Files:**
```rust
// chaos-backend-service/crates/element-core/src/config/elemental_config_loader.rs
// chaos-backend-service/crates/element-core/src/config/elemental_config_validator.rs
// chaos-backend-service/crates/element-core/src/config/mod.rs
```

#### **6. Adapter Files:**
```rust
// chaos-backend-service/crates/element-core/src/adapters/elemental_serializer.rs
// chaos-backend-service/crates/element-core/src/adapters/elemental_validator.rs
// chaos-backend-service/crates/element-core/src/adapters/mod.rs
```

### **Actor Core Hierarchical Files to Create:**

#### **1. Integration Files:**
```rust
// chaos-backend-service/crates/actor-core-hierarchical/src/systems/elemental/elemental_actor_integration.rs
// chaos-backend-service/crates/actor-core-hierarchical/src/systems/elemental/elemental_adapter.rs
// chaos-backend-service/crates/actor-core-hierarchical/src/systems/elemental/mod.rs
```

#### **2. Core Files:**
```rust
// chaos-backend-service/crates/actor-core-hierarchical/src/core/hierarchical_actor.rs
// chaos-backend-service/crates/actor-core-hierarchical/src/core/global_aggregator.rs
// chaos-backend-service/crates/actor-core-hierarchical/src/core/actor_factory.rs
// chaos-backend-service/crates/actor-core-hierarchical/src/core/mod.rs
```

## 📊 **Migration Checklist**

### **Element Core Migration:**
- [ ] Create new directory structure
- [ ] Move elemental_data.rs to core/
- [ ] Create elemental_system.rs
- [ ] Create elemental_registry.rs
- [ ] Create elemental_factory.rs
- [ ] Create elemental_aggregator.rs
- [ ] Create elemental_config_loader.rs
- [ ] Create elemental_serializer.rs
- [ ] Update Cargo.toml
- [ ] Update lib.rs
- [ ] Test element-core independently

### **Actor Core Hierarchical Migration:**
- [ ] Remove moved components
- [ ] Create elemental_actor_integration.rs
- [ ] Create new elemental_adapter.rs
- [ ] Update global_aggregator.rs
- [ ] Update Cargo.toml
- [ ] Update lib.rs
- [ ] Test integration layer

### **Integration Testing:**
- [ ] Test elemental system independently
- [ ] Test actor-elemental integration
- [ ] Test global aggregation
- [ ] Test performance
- [ ] Update documentation

## 🚨 **Important Notes**

### **1. Dependency Management:**
- Element-core should NOT depend on actor-core-hierarchical
- Actor-core-hierarchical should depend on element-core
- Use trait-based interfaces for loose coupling

### **2. Error Handling:**
- Maintain consistent error types
- Use ActorCoreResult for compatibility
- Handle integration errors gracefully

### **3. Performance Considerations:**
- Minimize data copying between layers
- Use Arc<Mutex<>> for shared data
- Optimize hot paths

### **4. Backward Compatibility:**
- Maintain existing public APIs where possible
- Use deprecation warnings for old APIs
- Provide migration guides

## 🎯 **Success Criteria**

### **Phase 1 Complete When:**
- [ ] All elemental components moved to element-core
- [ ] Element-core compiles independently
- [ ] Element-core tests pass

### **Phase 2 Complete When:**
- [ ] Integration layer created in actor-core-hierarchical
- [ ] Actor-core-hierarchical compiles with element-core dependency
- [ ] Integration tests pass

### **Phase 3 Complete When:**
- [ ] Full system integration working
- [ ] Performance benchmarks meet requirements
- [ ] Documentation updated
- [ ] All tests pass

## 📞 **Support & Troubleshooting**

### **Common Issues:**
1. **Circular Dependencies**: Ensure element-core doesn't depend on actor-core-hierarchical
2. **Import Errors**: Check Cargo.toml dependencies and feature flags
3. **Test Failures**: Verify trait implementations and async/await usage
4. **Performance Issues**: Profile integration layer and optimize hot paths

### **Debugging Steps:**
1. Check compilation errors first
2. Run tests individually to isolate issues
3. Use `cargo check` for quick syntax validation
4. Use `cargo test --verbose` for detailed test output

---

**This guide provides a systematic approach to implementing the corrected architecture. Follow the steps in order and check off items as you complete them.** 🎯
