# Complete Hardcoded Data Elimination Plan

## Overview

After a comprehensive review of the Actor Core codebase, I found extensive hardcoded game data that violates the pure hub architecture principle. This document outlines the complete elimination of all hardcoded data.

## 🚨 **CRITICAL HARDCODED PROBLEMS FOUND**

### 1. **`constants.rs` - MASSIVE HARDCODED PROBLEM**
**Current State:**
- Hardcoded resources: `HEALTH`, `MANA`, `STAMINA`, `EXPERIENCE`, `LEVEL`
- Hardcoded stats: `STRENGTH`, `AGILITY`, `INTELLIGENCE`, `VITALITY`, `SPIRIT`, `LUCK`
- Hardcoded derived stats: `ATTACK_POWER`, `DEFENSE_POWER`, `CRITICAL_HIT_CHANCE`, etc.
- Hardcoded system IDs: `LUYEN_THE`, `KIM_DAN`, `COMBAT`, `EQUIPMENT`, `MAGIC`, etc.
- Hardcoded ranges: Min/max values for all stats
- Hardcoded context types: `DAMAGE`, `HEALING`, `EXPERIENCE_GAIN`, etc.

**Solution:**
- Move all game-specific constants to configuration files
- Keep only system-level constants (timeouts, cache keys, error codes)
- Create dynamic constant loading from Runtime Registry System

### 2. **`validation/validator.rs` - HARDCODED VALIDATION**
**Current State:**
- Hardcoded dimensions: `"strength"`, `"agility"`, `"intelligence"`, `"health"`, `"mana"`, `"stamina"`
- Hardcoded validation rules

**Solution:**
- Load validation rules from Runtime Registry System
- Make validation configurable per subsystem
- Remove hardcoded dimension lists

### 3. **`subsystems/resource_management/system_resource_manager.rs` - HARDCODED RESOURCES**
**Current State:**
- Hardcoded resource categories: `ResourceCategory::Health`, `ResourceCategory::Physical`
- Hardcoded dependencies: `"level"`, `"vitality"`
- Hardcoded resource definitions

**Solution:**
- Move to examples/legacy_resource_managers/
- Use Runtime Registry System for all resource management
- Make resource calculations configurable

### 4. **`registry/subsystem_registration.rs` - HARDCODED BUILDERS**
**Current State:**
- Hardcoded resource builders: `create_health_resource`, `create_mana_resource`, `create_stamina_resource`
- Hardcoded values: Base values, max values, regen rates, categories, tags

**Solution:**
- Move to examples/legacy_resource_managers/
- Create generic builders that load from configuration
- Remove hardcoded resource definitions

### 5. **`condition_integration/integration.rs` - HARDCODED CONDITIONS**
**Current State:**
- Hardcoded resource names: `"health"` in condition parameters

**Solution:**
- Use Runtime Registry System to get available resources
- Make conditions configurable
- Remove hardcoded resource references

## 🔧 **ELIMINATION STRATEGY**

### Phase 1: Move Hardcoded Constants to Configuration
1. **Create Configuration Files:**
   - `configs/game_constants.yaml` - All game-specific constants
   - `configs/validation_rules.yaml` - Validation rules
   - `configs/resource_definitions.yaml` - Resource definitions
   - `configs/system_definitions.yaml` - System definitions

2. **Update Constants Module:**
   - Keep only system-level constants (timeouts, cache keys, error codes)
   - Remove all game-specific constants
   - Add dynamic loading from configuration

### Phase 2: Move Hardcoded Subsystems to Examples
1. **Move to `examples/legacy_subsystems/`:**
   - `system_resource_manager.rs` - Hardcoded resource calculations
   - `subsystem_registration.rs` - Hardcoded builders
   - Any other hardcoded subsystems

2. **Update Module References:**
   - Remove hardcoded subsystem references
   - Update imports and exports
   - Fix compilation errors

### Phase 3: Create Dynamic Configuration System
1. **Configuration Loader:**
   - Load game constants from YAML files
   - Load validation rules from configuration
   - Load resource definitions from Runtime Registry

2. **Dynamic Validation:**
   - Validate against configured rules
   - Support per-subsystem validation
   - Remove hardcoded dimension lists

### Phase 4: Update All References
1. **Replace Hardcoded References:**
   - Use Runtime Registry System for resource discovery
   - Use configuration for validation rules
   - Use dynamic constants for calculations

2. **Update Tests:**
   - Remove hardcoded test data
   - Use configuration-based test data
   - Test with different configurations

## 📁 **NEW FILE STRUCTURE**

### Configuration Files
```
configs/
├── game_constants.yaml          # All game-specific constants
├── validation_rules.yaml        # Validation rules
├── resource_definitions.yaml    # Resource definitions
├── system_definitions.yaml      # System definitions
└── examples/
    ├── rpg_constants.yaml       # RPG-specific constants
    ├── magic_constants.yaml     # Magic-specific constants
    └── cultivation_constants.yaml # Cultivation-specific constants
```

### Legacy Examples
```
examples/legacy_subsystems/
├── system_resource_manager.rs   # Hardcoded resource manager
├── subsystem_registration.rs    # Hardcoded builders
├── validation_rules.rs          # Hardcoded validation
└── README.md                    # Migration guide
```

### Updated Core
```
src/
├── constants.rs                 # Only system-level constants
├── validation/
│   └── dynamic_validator.rs     # Configuration-based validation
├── config/
│   ├── loader.rs                # Configuration loader
│   └── manager.rs               # Configuration manager
└── subsystems/
    └── resource_management/
        └── dynamic_resource_manager.rs # Runtime Registry-based manager
```

## 🎯 **IMPLEMENTATION STEPS**

### Step 1: Create Configuration Files
1. Extract all hardcoded constants to YAML files
2. Create configuration loader system
3. Test configuration loading

### Step 2: Move Hardcoded Subsystems
1. Move hardcoded subsystems to examples
2. Update module references
3. Fix compilation errors

### Step 3: Create Dynamic Systems
1. Create dynamic validation system
2. Create dynamic resource manager
3. Create dynamic constant loader

### Step 4: Update All References
1. Replace hardcoded references with dynamic ones
2. Update tests to use configuration
3. Test with different configurations

### Step 5: Documentation and Migration
1. Create migration guide
2. Update all documentation
3. Create examples for different game types

## ✅ **EXPECTED OUTCOME**

After complete elimination:
- **Zero hardcoded game data** in Actor Core
- **Pure hub architecture** maintained
- **Fully configurable** system
- **Runtime extensibility** for all game data
- **Plugin architecture** for all subsystems
- **Configuration-driven** validation and resources

## 🚀 **BENEFITS**

1. **✅ Pure Hub** - Actor Core has zero game-specific data
2. **✅ Configurable** - All game data defined in configuration files
3. **✅ Extensible** - New game types can be added without code changes
4. **✅ Maintainable** - No hardcoded values to maintain
5. **✅ Testable** - Easy to test with different configurations
6. **✅ Scalable** - Supports multiple game modes and subsystems

This plan ensures Actor Core becomes a true pure hub that can support any game type through configuration, without any hardcoded game-specific data.
