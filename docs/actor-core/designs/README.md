# Actor Core v3 Design Documentation

**Source**: Migrated from `chaos-actor-module/packages/actor-core/docs`  
**Target**: Rust implementation in `chaos-backend-service/crates/actor-core`  
**Status**: Design phase - Ready for Rust implementation  

## 📚 Documentation Overview

This directory contains the complete design documentation for Actor Core v3, a high-performance character stat aggregation system for the Chaos World MMORPG. The documentation has been migrated from the original Go implementation and is now ready to guide the Rust implementation.

## 🗂️ Documentation Structure

### Core Design Documents
- **[00_README.md](00_README.md)** - Main documentation index and overview
- **[01_Executive_Summary.md](01_Executive_Summary.md)** - High-level system overview and key concepts
- **[02_Architecture_and_Source_Structure.md](02_Architecture_and_Source_Structure.md)** - System architecture and code organization
- **[03_Domain_Model.md](03_Domain_Model.md)** - Core domain models and data structures
- **[04_Constants_Enums_Interfaces.md](04_Constants_Enums_Interfaces.md)** - Type definitions and interfaces

### Implementation Guides
- **[06_Aggregation_Algorithm.md](06_Aggregation_Algorithm.md)** - Core stat aggregation algorithm
- **[07_Caps_and_Layers.md](07_Caps_and_Layers.md)** - Cap system and layer management
- **[08_Combiner_Registry.md](08_Combiner_Registry.md)** - Registry system for merge rules
- **[09_Context_Modifiers.md](09_Context_Modifiers.md)** - Context-based stat modifications
- **[10_Implementation_Guide.md](10_Implementation_Guide.md)** - Step-by-step implementation guide

### Advanced Features
- **[11_Error_Handling_Observability.md](11_Error_Handling_Observability.md)** - Error handling and monitoring
- **[12_Performance_Concurrency.md](12_Performance_Concurrency.md)** - Performance optimization strategies
- **[13_Extensibility_Plugins.md](13_Extensibility_Plugins.md)** - Plugin system and extensibility
- **[23_Performance_Optimizations.md](23_Performance_Optimizations.md)** - Detailed performance optimizations
- **[24_Subsystem_Development_Guide.md](24_Subsystem_Development_Guide.md)** - Subsystem development guide

### Testing & Quality
- **[05_Data_Schemas.md](05_Data_Schemas.md)** - JSON schemas and data validation
- **[14_Testing_Strategy.md](14_Testing_Strategy.md)** - Comprehensive testing approach
- **[18_Examples_and_Sequences.md](18_Examples_and_Sequences.md)** - Usage examples and data flows
- **[22_Human_Reviewer_Checklist.md](22_Human_Reviewer_Checklist.md)** - Code review checklist

### Production & Deployment
- **[15_Release_Versioning.md](15_Release_Versioning.md)** - Version management and releases
- **[16_Security_Trust.md](16_Security_Trust.md)** - Security considerations and trust model
- **[17_Governance_Change_Control.md](17_Governance_Change_Control.md)** - Change management process
- **[25_Production_Deployment_Guide.md](25_Production_Deployment_Guide.md)** - Production deployment guide

### Reference Materials
- **[19_FAQ.md](19_FAQ.md)** - Frequently asked questions
- **[20_Glossary.md](20_Glossary.md)** - Terminology and definitions
- **[21_Dimension_Catalog.md](21_Dimension_Catalog.md)** - Complete dimension catalog
- **[26_Real_World_Pattern_Comparison.md](26_Real_World_Pattern_Comparison.md)** - Industry pattern comparisons

## 🧪 Test Vectors & Schemas

### Golden Test Vectors
- **[golden_vectors/](golden_vectors/)** - Comprehensive test cases for validation
  - `case01_total_cap_sum/` - Basic cap aggregation
  - `case02_realm_world_total/` - Multi-layer cap testing
  - `case03_override_priority/` - Priority-based overrides
  - `case04_mult_stacking/` - Multiplicative stacking
  - `case05_cdr_cap/` - Cooldown reduction caps
  - `case06_lifespan_operator_max/` - Lifespan operator testing
  - `case07_move_speed_floor/` - Movement speed floor testing
  - `case08_min_gt_max_conflict/` - Min/max conflict handling
  - `case09_prioritized_override/` - Prioritized override testing
  - `case10_primary_derived_realm_min/` - Primary/derived realm testing

### JSON Schemas
- **[schemas/](schemas/)** - JSON Schema definitions for data validation
  - `Contribution.schema.json` - Stat contribution schema
  - `CapContribution.schema.json` - Cap contribution schema
  - `SubsystemOutput.schema.json` - Subsystem output schema
  - `Snapshot.schema.json` - Final snapshot schema
  - `EffectiveCaps.schema.json` - Effective caps schema
  - `MergeRule.schema.json` - Merge rule schema
  - `Registry.schema.json` - Registry configuration schema
  - `CapLayerRegistry.schema.json` - Cap layer registry schema

### Example Configurations
- **[appendix/](appendix/)** - Example configuration files
  - `Registry.example.yml` - Registry configuration example
  - `CapLayerRegistry.example.yml` - Cap layer registry example

## 📋 Development Resources

### Documentation
- **README.md** - This comprehensive documentation index
- **Migration Plans** - See [../migrations/](../migrations/) for Go to Rust migration guides

## 🚀 Quick Start for Rust Implementation

### 1. Read the Core Design
Start with these essential documents:
1. [01_Executive_Summary.md](01_Executive_Summary.md) - Understand the system
2. [03_Domain_Model.md](03_Domain_Model.md) - Learn the data models
3. [06_Aggregation_Algorithm.md](06_Aggregation_Algorithm.md) - Understand the core algorithm

### 2. Study the Implementation Guide
Follow the step-by-step guide:
1. [10_Implementation_Guide.md](10_Implementation_Guide.md) - Implementation roadmap
2. [24_Subsystem_Development_Guide.md](24_Subsystem_Development_Guide.md) - Subsystem development
3. [23_Performance_Optimizations.md](23_Performance_Optimizations.md) - Performance considerations

### 3. Validate with Test Vectors
Use the golden test vectors for validation:
1. Review [golden_vectors/README.md](golden_vectors/README.md)
2. Implement test cases from the golden vectors
3. Validate against JSON schemas in [schemas/](schemas/)

### 4. Reference the Migration Plan
For Go to Rust migration specifics:
- See [../migrations/](../migrations/) for detailed migration plans
- Review [RUST_IMPLEMENTATION_GUIDE.md](../migrations/RUST_IMPLEMENTATION_GUIDE.md)
- Follow [MIGRATION_CHECKLIST.md](../migrations/MIGRATION_CHECKLIST.md)

## 🔗 Related Documentation

- **[../migrations/](../migrations/)** - Go to Rust migration documentation
- **[../../crates/actor-core/](../../crates/actor-core/)** - Rust implementation source code
- **[../../README.md](../../README.md)** - Main project documentation

## 📝 Notes

- This documentation was migrated from the original Go implementation
- All design principles and algorithms remain valid for the Rust implementation
- The migration plan in `../migrations/` provides Rust-specific implementation details
- Test vectors and schemas are ready for immediate use in Rust testing

---

**Last Updated**: 2025-01-27  
**Migration Status**: Complete  
**Next Steps**: Begin Rust implementation following the migration plan
