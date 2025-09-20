# Legacy Resource Managers

This directory contains legacy resource management implementations that have been moved from the main Actor Core codebase because they contain hardcoded resource definitions, which violates the pure hub architecture principle.

## Files Moved

### 1. `rpg_resource_manager.rs`
- **Original Location**: `src/subsystems/resource_management/rpg_resource_manager.rs`
- **Reason for Move**: Contains hardcoded RPG resource definitions (HP, MP, Stamina, Experience, Special Points)
- **Replacement**: Use `examples/rpg_resource_manager_refactored.rs` with Runtime Registry System

### 2. `magic_resource_manager.rs`
- **Original Location**: `src/subsystems/resource_management/magic_resource_manager.rs`
- **Reason for Move**: Contains hardcoded Magic resource definitions (Mana, Arcane Focus, Magical Knowledge, Divine Energy, Nature Essence)
- **Replacement**: Use `examples/magic_resource_config.yaml` with Runtime Registry System

### 3. `resource_manager.rs`
- **Original Location**: `src/subsystems/resource_management/resource_manager.rs`
- **Reason for Move**: Contains hardcoded core resource definitions (HP, Mana, Stamina)
- **Replacement**: Use Runtime Registry System for dynamic resource management

### 4. `enhanced_hybrid_resource_manager.rs`
- **Original Location**: `src/subsystems/resource_management/enhanced_hybrid_resource_manager.rs`
- **Reason for Move**: Contains hardcoded resource calculations and mappings
- **Replacement**: Use Runtime Registry System with configurable resource definitions

## Why These Files Were Moved

Actor Core follows a **pure hub architecture** where:
- ✅ **No hardcoded game data** - All resources, categories, and tags are defined at runtime
- ✅ **Plugin-based** - Subsystems register their resources dynamically
- ✅ **Configuration-driven** - Resources defined in YAML configuration files
- ✅ **Extensible** - New resources can be added without code changes

The legacy files violated these principles by:
- ❌ **Hardcoded resource definitions** - Resources like "hp", "mana", "stamina" were hardcoded
- ❌ **Hardcoded categories** - Resource categories were defined in enums
- ❌ **Hardcoded formulas** - Calculation formulas were hardcoded in the implementation
- ❌ **Tight coupling** - Game-specific logic was mixed with core Actor functionality

## Migration Path

### For RPG Resources
1. **Use**: `examples/rpg_resource_config.yaml` for configuration
2. **Use**: `examples/rpg_resource_manager_refactored.rs` for implementation
3. **Register**: Resources at runtime using `RegistryManager`

### For Magic Resources
1. **Use**: `examples/magic_resource_config.yaml` for configuration
2. **Create**: Similar refactored implementation using Runtime Registry System
3. **Register**: Resources at runtime using `RegistryManager`

### For Core Resources
1. **Use**: Runtime Registry System for all resource management
2. **Define**: Resources in configuration files
3. **Register**: Resources dynamically at subsystem startup

## Current Actor Core Architecture

The current Actor Core now uses:

### Runtime Registry System
- **`ResourceRegistry`** - Manages resource definitions dynamically
- **`CategoryRegistry`** - Manages category definitions dynamically
- **`TagRegistry`** - Manages tag definitions dynamically
- **`RegistryManager`** - Coordinates all registries

### Configuration Files
- **YAML-based** - Easy to modify without recompilation
- **Version controlled** - Changes tracked in git
- **Environment-specific** - Different configs for different game modes

### Dynamic Registration
- **Subsystem startup** - Resources registered when subsystems initialize
- **Runtime discovery** - Other systems can query available resources
- **Hot reloading** - Resources can be updated without restart

## Benefits of the New Architecture

1. **✅ Pure Hub** - Actor Core has zero hardcoded game data
2. **✅ Extensible** - New resources can be added without code changes
3. **✅ Configurable** - Resources defined in external configuration files
4. **✅ Maintainable** - Clear separation between core and game logic
5. **✅ Testable** - Easy to test with different resource configurations
6. **✅ Scalable** - Supports multiple game modes and subsystems

## Usage Examples

See the following files for examples of the new architecture:
- `examples/rpg_resource_config.yaml` - RPG resource configuration
- `examples/magic_resource_config.yaml` - Magic resource configuration
- `examples/rpg_resource_manager_refactored.rs` - Refactored RPG manager
- `examples/runtime_registry_example.rs` - Runtime registry usage

## Deprecation Notice

These legacy files are kept for reference and migration purposes but should not be used in new implementations. They will be removed in a future version of Actor Core.

**Use the Runtime Registry System for all new resource management implementations.**
