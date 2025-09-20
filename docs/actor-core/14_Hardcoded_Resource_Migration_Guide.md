# Hardcoded Resource Migration Guide

## Overview

This guide explains how to migrate from hardcoded resource definitions to the Runtime Registry System in Actor Core. The goal is to make Actor Core a pure hub without any hardcoded game-specific data.

## Problem Statement

Currently, several subsystems have hardcoded resource definitions:

1. **RPG Resource Manager** - Hardcoded HP, MP, Stamina, Experience, Special Points
2. **Magic Resource Manager** - Hardcoded Mana, Arcane Focus, Magical Knowledge, Divine Energy, Nature Essence
3. **Resource Exhaustion System** - References hardcoded resource names
4. **Enhanced Hybrid Resource Manager** - Hardcoded resource categories and mappings

## Migration Strategy

### Phase 1: Identify Hardcoded Resources

#### RPG Resource Manager (`rpg_resource_manager.rs`)
**Before (Hardcoded):**
```rust
// Hardcoded in initialize_default_resources()
self.add_resource_definition(RpgResourceDefinition {
    name: "hp_current".to_string(),
    base_formula: "vitality * 10 + level * 5".to_string(),
    regen_rate: 0.1,
    max_formula: "vitality * 10 + level * 5 + equipment_bonus".to_string(),
    category: RpgResourceCategory::Health,
    dependencies: vec!["vitality".to_string(), "level".to_string()],
});
```

**After (Runtime Registry):**
```rust
// Register resources at runtime
let hp_resource = ResourceDefinition {
    id: "hp_current".to_string(),
    name: "Health Points".to_string(),
    description: Some("Character's current health points".to_string()),
    category: "health".to_string(),
    resource_type: ResourceType::Health,
    base_value: 0.0,
    min_value: 0.0,
    max_value: 1000.0,
    regen_rate: 0.1,
    regen_type: RegenType::Passive,
    dependencies: vec!["vitality".to_string(), "level".to_string()],
    tags: vec!["vital".to_string(), "health".to_string(), "rpg".to_string()],
    subsystem_id: self.system_id.clone(),
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
};
resource_registry.register_resource(hp_resource).await?;
```

#### Magic Resource Manager (`magic_resource_manager.rs`)
**Before (Hardcoded):**
```rust
// Hardcoded in initialize_default_resources()
self.add_resource_definition(MagicResourceDefinition {
    name: "mana_current".to_string(),
    base_formula: "intelligence * 12 + wisdom * 8 + level * 4".to_string(),
    regen_rate: 0.3,
    max_formula: "intelligence * 12 + wisdom * 8 + level * 4 + equipment_bonus".to_string(),
    category: MagicResourceCategory::Mana,
    dependencies: vec!["intelligence".to_string(), "wisdom".to_string(), "level".to_string()],
    school: MagicSchool::Universal,
});
```

**After (Runtime Registry):**
```rust
// Register resources at runtime
let mana_resource = ResourceDefinition {
    id: "mana_current".to_string(),
    name: "Mana Points".to_string(),
    description: Some("Character's current mana points".to_string()),
    category: "mana".to_string(),
    resource_type: ResourceType::Mana,
    base_value: 0.0,
    min_value: 0.0,
    max_value: 1000.0,
    regen_rate: 0.3,
    regen_type: RegenType::Passive,
    dependencies: vec!["intelligence".to_string(), "wisdom".to_string(), "level".to_string()],
    tags: vec!["magic".to_string(), "mana".to_string(), "universal".to_string()],
    subsystem_id: self.system_id.clone(),
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
};
resource_registry.register_resource(mana_resource).await?;
```

### Phase 2: Create Configuration Files

#### RPG Resource Configuration (`rpg_resource_config.yaml`)
```yaml
rpg_resources:
  hp_current:
    name: "Health Points"
    description: "Character's current health points"
    category: "health"
    resource_type: "health"
    base_formula: "vitality * 10 + level * 5"
    max_formula: "vitality * 10 + level * 5 + equipment_bonus"
    regen_rate: 0.1
    regen_type: "passive"
    dependencies: ["vitality", "level"]
    tags: ["vital", "health", "rpg"]
    subsystem_id: "rpg_system"
```

#### Magic Resource Configuration (`magic_resource_config.yaml`)
```yaml
magic_resources:
  mana_current:
    name: "Mana Points"
    description: "Character's current mana points"
    category: "mana"
    resource_type: "mana"
    base_formula: "intelligence * 12 + wisdom * 8 + level * 4"
    max_formula: "intelligence * 12 + wisdom * 8 + level * 4 + equipment_bonus"
    regen_rate: 0.3
    regen_type: "passive"
    dependencies: ["intelligence", "wisdom", "level"]
    tags: ["magic", "mana", "universal"]
    subsystem_id: "magic_system"
```

### Phase 3: Refactor Subsystems

#### 1. Update Constructor Pattern
**Before:**
```rust
impl RpgResourceManager {
    pub fn new() -> Self {
        let mut manager = Self {
            system_id: "rpg_system".to_string(),
            resource_definitions: HashMap::new(),
        };
        manager.initialize_default_resources(); // Hardcoded
        manager
    }
}
```

**After:**
```rust
impl RpgResourceManagerRefactored {
    pub fn new(registry_manager: Arc<RegistryManager>) -> Self {
        Self {
            system_id: "rpg_system".to_string(),
            registry_manager,
            calculator: SystemResourceCalculator::new(),
        }
    }
    
    pub async fn initialize_from_config(&self) -> ActorCoreResult<()> {
        // Register resources from configuration
        self.register_rpg_resources().await?;
        Ok(())
    }
}
```

#### 2. Update Resource Access Pattern
**Before:**
```rust
async fn calculate_resource_value(&self, actor: &Actor, resource_name: &str) -> ActorCoreResult<f64> {
    let definition = self.resource_definitions.get(resource_name)
        .ok_or_else(|| to_actor_core_error(format!("Resource definition not found: {}", resource_name)))?;
    // Use hardcoded definition
}
```

**After:**
```rust
async fn calculate_resource_value(&self, actor: &Actor, resource_name: &str) -> ActorCoreResult<f64> {
    let resource_registry = self.registry_manager.get_resource_registry();
    let resource_def = resource_registry.get_resource(resource_name).await?
        .ok_or_else(|| to_actor_core_error(format!("Resource not found: {}", resource_name)))?;
    // Use dynamic definition from registry
}
```

#### 3. Update Resource Discovery Pattern
**Before:**
```rust
fn get_available_resources(&self) -> Vec<String> {
    self.resource_definitions.keys().cloned().collect()
}
```

**After:**
```rust
async fn get_available_resources(&self) -> ActorCoreResult<Vec<String>> {
    let resource_registry = self.registry_manager.get_resource_registry();
    let resources = resource_registry.get_resources_by_subsystem(&self.system_id).await?;
    Ok(resources.into_iter().map(|r| r.id).collect())
}
```

### Phase 4: Update Resource Exhaustion System

#### Before (Hardcoded References):
```rust
// Hardcoded resource name references
if resource_name == "mana" && threshold.id == "low_mana" {
    // Handle low mana
}
```

#### After (Dynamic References):
```rust
// Query available resources from registry
let resource_registry = self.registry_manager.get_resource_registry();
let available_resources = resource_registry.get_all_resources().await?;

for resource in available_resources {
    if resource.id == resource_name && threshold.id == "low_mana" {
        // Handle low mana dynamically
    }
}
```

## Implementation Steps

### Step 1: Create Example Configurations
1. Create `rpg_resource_config.yaml` with all RPG resources
2. Create `magic_resource_config.yaml` with all Magic resources
3. Create `exhaustion_config.yaml` with exhaustion thresholds

### Step 2: Refactor Resource Managers
1. Update constructors to accept `RegistryManager`
2. Replace hardcoded resource definitions with registry registration
3. Update resource access methods to use registry
4. Add configuration loading methods

### Step 3: Update Resource Exhaustion System
1. Replace hardcoded resource references with registry queries
2. Make exhaustion thresholds configurable
3. Update event publishing to use dynamic resource names

### Step 4: Create Migration Examples
1. Create `rpg_resource_manager_refactored.rs` example
2. Create `magic_resource_manager_refactored.rs` example
3. Create `exhaustion_system_refactored.rs` example

### Step 5: Update Tests
1. Update unit tests to use registry system
2. Create integration tests with configuration loading
3. Add performance tests for registry queries

## Benefits of Migration

### 1. **Pure Hub Architecture**
- Actor Core has zero hardcoded game data
- All resources are defined by subsystems at runtime

### 2. **Runtime Extensibility**
- New resources can be added without code changes
- Different game modes can have different resource sets
- Subsystems can be loaded/unloaded dynamically

### 3. **Configuration-Driven**
- Resources defined in YAML configuration files
- Easy to modify without recompilation
- Version control friendly

### 4. **Maintainability**
- No hardcoded values to maintain
- Clear separation between core and game-specific logic
- Easier to test and debug

### 5. **Plugin Architecture**
- Subsystems can register their resources independently
- No coupling between different resource systems
- Easy to add new resource types

## Migration Checklist

- [ ] Identify all hardcoded resource definitions
- [ ] Create configuration files for each subsystem
- [ ] Refactor resource managers to use registry system
- [ ] Update resource exhaustion system
- [ ] Create migration examples
- [ ] Update tests
- [ ] Update documentation
- [ ] Performance testing
- [ ] Integration testing
- [ ] Production deployment

## Example Migration

See the following files for complete migration examples:
- `examples/rpg_resource_config.yaml` - RPG resource configuration
- `examples/magic_resource_config.yaml` - Magic resource configuration
- `examples/rpg_resource_manager_refactored.rs` - Refactored RPG manager
- `examples/magic_resource_manager_refactored.rs` - Refactored Magic manager

This migration ensures Actor Core remains a pure hub while providing the flexibility needed for dynamic, extensible game systems.
