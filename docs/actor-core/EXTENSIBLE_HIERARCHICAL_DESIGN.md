# Extensible Hierarchical Design for Actor Core

## 🎯 **Problem Statement**

Current `actor-core-hierarchical` structure is not scalable for multiple systems:

```
actor-core-hierarchical/src/
├── types/elemental_data.rs    # ❌ Hard-coded for elemental only
├── systems/elemental.rs       # ❌ Hard-coded for elemental only
├── adapters/elemental_adapter.rs  # ❌ Hard-coded for elemental only
└── aggregation/elemental_aggregator.rs  # ❌ Hard-coded for elemental only
```

**Issues:**
- Hard-coded elemental-specific implementations
- Cannot easily add new systems (cultivation, magic, race, etc.)
- Duplicate code for each system
- No generic system management

## 🏗️ **New Extensible Architecture (Revised)**

### **📁 New File Structure (Specialized Approach):**

```
actor-core-hierarchical/src/
├── core/                          # Core hierarchical functionality
│   ├── mod.rs
│   ├── hierarchical_actor.rs      # Main hierarchical actor
│   ├── global_aggregator.rs       # Global stats aggregation
│   └── actor_factory.rs           # Actor creation factory
│
├── systems/                       # System implementations
│   ├── mod.rs
│   ├── base/                      # Base system traits & implementations
│   │   ├── mod.rs
│   │   ├── system_trait.rs        # Generic system trait
│   │   └── system_base.rs         # Base system implementation
│   │
│   ├── elemental/                 # Elemental system (Complete)
│   │   ├── mod.rs
│   │   ├── elemental_system.rs    # Elemental system implementation
│   │   ├── elemental_data.rs      # Elemental data structures
│   │   ├── elemental_registry.rs  # Elemental-specific registry
│   │   ├── elemental_factory.rs   # Elemental system factory
│   │   ├── elemental_aggregator.rs # Elemental aggregation logic
│   │   └── elemental_adapter.rs   # Elemental adapter
│   │
│   ├── cultivation/               # Cultivation system (Future)
│   │   ├── mod.rs
│   │   ├── cultivation_system.rs
│   │   ├── cultivation_data.rs
│   │   ├── cultivation_registry.rs # Cultivation-specific registry
│   │   ├── cultivation_factory.rs
│   │   ├── cultivation_aggregator.rs
│   │   └── cultivation_adapter.rs
│   │
│   ├── magic/                     # Magic system (Future)
│   │   ├── mod.rs
│   │   ├── magic_system.rs
│   │   ├── magic_data.rs
│   │   ├── magic_registry.rs      # Magic-specific registry
│   │   ├── magic_factory.rs
│   │   ├── magic_aggregator.rs
│   │   └── magic_adapter.rs
│   │
│   └── race/                      # Race system (Future)
│       ├── mod.rs
│       ├── race_system.rs
│       ├── race_data.rs
│       ├── race_registry.rs       # Race-specific registry
│       ├── race_factory.rs
│       ├── race_aggregator.rs
│       └── race_adapter.rs
│
├── adapters/                      # System-specific adapters
│   ├── mod.rs
│   ├── base_adapter.rs            # Base adapter trait
│   ├── actor_adapter.rs           # Actor conversion adapter
│   ├── elemental_adapter.rs       # Elemental adapter (moved from systems)
│   ├── cultivation_adapter.rs     # Cultivation adapter (future)
│   ├── magic_adapter.rs           # Magic adapter (future)
│   └── race_adapter.rs            # Race adapter (future)
│
├── aggregation/                   # System-specific aggregation
│   ├── mod.rs
│   ├── base_aggregator.rs         # Base aggregation logic
│   ├── elemental_aggregator.rs    # Elemental aggregation (moved from systems)
│   ├── cultivation_aggregator.rs  # Cultivation aggregation (future)
│   ├── magic_aggregator.rs        # Magic aggregation (future)
│   ├── race_aggregator.rs         # Race aggregation (future)
│   └── global_aggregator.rs       # Global stats aggregation
│
├── registries/                    # System-specific registries
│   ├── mod.rs
│   ├── base_registry.rs           # Base registry trait
│   ├── elemental_registry.rs      # Elemental registry (moved from systems)
│   ├── cultivation_registry.rs    # Cultivation registry (future)
│   ├── magic_registry.rs          # Magic registry (future)
│   ├── race_registry.rs           # Race registry (future)
│   └── registry_manager.rs        # Registry coordination manager
│
├── factories/                     # System-specific factories
│   ├── mod.rs
│   ├── base_factory.rs            # Base factory trait
│   ├── elemental_factory.rs       # Elemental factory (moved from systems)
│   ├── cultivation_factory.rs     # Cultivation factory (future)
│   ├── magic_factory.rs           # Magic factory (future)
│   ├── race_factory.rs            # Race factory (future)
│   └── factory_manager.rs         # Factory coordination manager
│
└── utils/                         # Utilities
    ├── mod.rs
    ├── system_utils.rs            # System utilities
    ├── performance.rs             # Performance monitoring
    └── validation.rs              # Validation utilities
```

## 🎯 **Why Specialized Approach is Better**

### **❌ Problems with Generic "God Registry" Approach:**

1. **God Registry Problem**: 
   - One registry trying to handle all systems
   - Complex generic logic becomes hard to maintain
   - Performance issues with too many abstractions

2. **Tight Coupling**: 
   - All systems depend on generic registry
   - Changes in one system affect others
   - Hard to optimize individual systems

3. **Code Complexity**: 
   - Generic traits become bloated
   - Hard to understand system-specific logic
   - Difficult to debug issues

### **✅ Benefits of Specialized Approach:**

1. **Separation of Concerns**:
   - Each system has its own registry, factory, aggregator
   - Clear boundaries between systems
   - Easy to understand and maintain

2. **Performance Optimization**:
   - System-specific optimizations
   - No generic overhead
   - Direct access to system data

3. **Extensibility**:
   - Easy to add new systems
   - Copy existing system structure
   - No need to modify generic code

4. **Maintainability**:
   - Each system is self-contained
   - Changes in one system don't affect others
   - Easier testing and debugging

### **🏗️ Architecture Principles:**

1. **System Isolation**: Each system manages its own components
2. **Coordination Layer**: Registry/Factory managers coordinate between systems
3. **Base Traits**: Common functionality through base traits
4. **Specialized Implementation**: System-specific optimizations

## 🔧 **Core Implementation**

### **1. Generic System Trait**

```rust
// src/systems/base/system_trait.rs

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Generic system trait that all systems must implement
#[async_trait]
pub trait HierarchicalSystem: Send + Sync {
    /// System identifier
    fn system_id(&self) -> &str;
    
    /// System name for display
    fn system_name(&self) -> &str;
    
    /// System priority (higher = more important)
    fn system_priority(&self) -> i32;
    
    /// Check if system is enabled for actor
    async fn is_enabled(&self, actor_id: &str) -> bool;
    
    /// Get system-specific data for actor
    async fn get_system_data(&self, actor_id: &str) -> ActorCoreResult<SystemData>;
    
    /// Update system-specific data for actor
    async fn update_system_data(&self, actor_id: &str, data: SystemData) -> ActorCoreResult<()>;
    
    /// Calculate system contributions to global stats
    async fn calculate_contributions(&self, actor_id: &str) -> ActorCoreResult<Vec<Contribution>>;
    
    /// Get system dependencies
    fn get_dependencies(&self) -> Vec<String>;
    
    /// Validate system data
    async fn validate_data(&self, data: &SystemData) -> ActorCoreResult<()>;
    
    /// Initialize system
    async fn initialize(&mut self) -> ActorCoreResult<()>;
    
    /// Shutdown system
    async fn shutdown(&mut self) -> ActorCoreResult<()>;
}

/// Generic system data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemData {
    pub system_id: String,
    pub actor_id: String,
    pub data: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
    pub version: u32,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// System contribution to global stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
    pub dimension: String,
    pub value: f64,
    pub bucket: String,
    pub operator: String,
    pub metadata: HashMap<String, String>,
}
```

### **2. Base System Implementation**

```rust
// src/systems/base/system_base.rs

use async_trait::async_trait;
use std::collections::HashMap;
use crate::systems::base::{HierarchicalSystem, SystemData, Contribution};

/// Base system implementation with common functionality
pub struct BaseHierarchicalSystem {
    pub system_id: String,
    pub system_name: String,
    pub system_priority: i32,
    pub dependencies: Vec<String>,
    pub enabled: bool,
}

impl BaseHierarchicalSystem {
    pub fn new(system_id: String, system_name: String, priority: i32) -> Self {
        Self {
            system_id,
            system_name,
            system_priority: priority,
            dependencies: Vec::new(),
            enabled: true,
        }
    }
    
    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }
    
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

#[async_trait]
impl HierarchicalSystem for BaseHierarchicalSystem {
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    fn system_name(&self) -> &str {
        &self.system_name
    }
    
    fn system_priority(&self) -> i32 {
        self.system_priority
    }
    
    async fn is_enabled(&self, _actor_id: &str) -> bool {
        self.enabled
    }
    
    async fn get_system_data(&self, actor_id: &str) -> ActorCoreResult<SystemData> {
        // Default implementation - override in concrete systems
        Ok(SystemData {
            system_id: self.system_id.clone(),
            actor_id: actor_id.to_string(),
            data: HashMap::new(),
            metadata: HashMap::new(),
            version: 1,
            last_updated: chrono::Utc::now(),
        })
    }
    
    async fn update_system_data(&self, _actor_id: &str, _data: SystemData) -> ActorCoreResult<()> {
        // Default implementation - override in concrete systems
        Ok(())
    }
    
    async fn calculate_contributions(&self, _actor_id: &str) -> ActorCoreResult<Vec<Contribution>> {
        // Default implementation - override in concrete systems
        Ok(Vec::new())
    }
    
    fn get_dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
    
    async fn validate_data(&self, _data: &SystemData) -> ActorCoreResult<()> {
        // Default validation - override in concrete systems
        Ok(())
    }
    
    async fn initialize(&mut self) -> ActorCoreResult<()> {
        // Default initialization - override in concrete systems
        Ok(())
    }
    
    async fn shutdown(&mut self) -> ActorCoreResult<()> {
        // Default shutdown - override in concrete systems
        Ok(())
    }
}
```

### **3. System Factory Pattern**

```rust
// src/systems/base/system_factory.rs

use async_trait::async_trait;
use std::collections::HashMap;
use crate::systems::base::HierarchicalSystem;

/// Factory trait for creating system instances
#[async_trait]
pub trait SystemFactory: Send + Sync {
    type SystemType: HierarchicalSystem;
    
    /// Create a new system instance
    async fn create_system(&self, config: HashMap<String, serde_json::Value>) -> ActorCoreResult<Self::SystemType>;
    
    /// Get system configuration schema
    fn get_config_schema(&self) -> serde_json::Value;
    
    /// Validate system configuration
    fn validate_config(&self, config: &HashMap<String, serde_json::Value>) -> ActorCoreResult<()>;
    
    /// Get system dependencies
    fn get_system_dependencies(&self) -> Vec<String>;
}

/// Default system factory implementation
pub struct DefaultSystemFactory;

#[async_trait]
impl SystemFactory for DefaultSystemFactory {
    type SystemType = BaseHierarchicalSystem;
    
    async fn create_system(&self, config: HashMap<String, serde_json::Value>) -> ActorCoreResult<Self::SystemType> {
        let system_id = config.get("system_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ActorCoreError::ConfigurationError("Missing system_id".to_string()))?;
        
        let system_name = config.get("system_name")
            .and_then(|v| v.as_str())
            .unwrap_or(system_id);
        
        let priority = config.get("priority")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;
        
        Ok(BaseHierarchicalSystem::new(
            system_id.to_string(),
            system_name.to_string(),
            priority,
        ))
    }
    
    fn get_config_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "system_id": {"type": "string"},
                "system_name": {"type": "string"},
                "priority": {"type": "integer"}
            },
            "required": ["system_id"]
        })
    }
    
    fn validate_config(&self, config: &HashMap<String, serde_json::Value>) -> ActorCoreResult<()> {
        if !config.contains_key("system_id") {
            return Err(ActorCoreError::ConfigurationError("Missing system_id".to_string()));
        }
        Ok(())
    }
    
    fn get_system_dependencies(&self) -> Vec<String> {
        Vec::new()
    }
}
```

### **4. Specialized System Registries**

```rust
// src/registries/elemental_registry.rs

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::systems::elemental::{ElementalSystem, ElementalData};

/// Elemental-specific registry for managing elemental systems
pub struct ElementalRegistry {
    elemental_systems: Arc<RwLock<HashMap<String, Arc<ElementalSystem>>>>,
    elemental_data: Arc<RwLock<HashMap<String, ElementalData>>>,
    system_metrics: Arc<RwLock<ElementalRegistryMetrics>>,
}

impl ElementalRegistry {
    pub fn new() -> Self {
        Self {
            elemental_systems: Arc::new(RwLock::new(HashMap::new())),
            elemental_data: Arc::new(RwLock::new(HashMap::new())),
            system_metrics: Arc::new(RwLock::new(ElementalRegistryMetrics::default())),
        }
    }
    
    /// Register an elemental system for an actor
    pub async fn register_elemental_system(
        &self,
        actor_id: String,
        elemental_data: ElementalData,
    ) -> ActorCoreResult<()> {
        let elemental_system = ElementalSystem::new(actor_id.clone(), elemental_data.clone());
        
        {
            let mut systems = self.elemental_systems.write().await;
            systems.insert(actor_id.clone(), Arc::new(elemental_system));
        }
        
        {
            let mut data = self.elemental_data.write().await;
            data.insert(actor_id, elemental_data);
        }
        
        Ok(())
    }
    
    /// Get elemental system for actor
    pub async fn get_elemental_system(&self, actor_id: &str) -> Option<Arc<ElementalSystem>> {
        let systems = self.elemental_systems.read().await;
        systems.get(actor_id).cloned()
    }
    
    /// Get elemental data for actor
    pub async fn get_elemental_data(&self, actor_id: &str) -> Option<ElementalData> {
        let data = self.elemental_data.read().await;
        data.get(actor_id).cloned()
    }
}

#[derive(Debug, Default)]
pub struct ElementalRegistryMetrics {
    pub registered_systems: usize,
    pub total_elements: usize,
    pub cache_hits: u64,
    pub cache_misses: u64,
}
```

```rust
// src/registries/registry_manager.rs

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::registries::elemental_registry::ElementalRegistry;
// Future: use crate::registries::cultivation_registry::CultivationRegistry;
// Future: use crate::registries::magic_registry::MagicRegistry;
// Future: use crate::registries::race_registry::RaceRegistry;

/// Registry manager for coordinating all system registries
pub struct RegistryManager {
    elemental_registry: Arc<ElementalRegistry>,
    // Future registries
    // cultivation_registry: Arc<CultivationRegistry>,
    // magic_registry: Arc<MagicRegistry>,
    // race_registry: Arc<RaceRegistry>,
    
    registry_configs: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

impl RegistryManager {
    pub fn new() -> Self {
        Self {
            elemental_registry: Arc::new(ElementalRegistry::new()),
            registry_configs: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Get elemental registry
    pub fn elemental(&self) -> &Arc<ElementalRegistry> {
        &self.elemental_registry
    }
    
    /// Get all registered actor IDs across all systems
    pub async fn get_all_actor_ids(&self) -> Vec<String> {
        let mut actor_ids = Vec::new();
        
        // Get from elemental registry
        {
            let systems = self.elemental_registry.elemental_systems.read().await;
            actor_ids.extend(systems.keys().cloned());
        }
        
        // Future: Get from other registries
        
        actor_ids.sort();
        actor_ids.dedup();
        actor_ids
    }
    
    /// Check if actor exists in any system
    pub async fn actor_exists(&self, actor_id: &str) -> bool {
        // Check elemental registry
        {
            let systems = self.elemental_registry.elemental_systems.read().await;
            if systems.contains_key(actor_id) {
                return true;
            }
        }
        
        // Future: Check other registries
        
        false
    }
}

## 🚀 **Benefits of Specialized Architecture**

### **1. Extensibility:**
- **Easy System Addition**: Copy elemental system structure to create new systems
- **Independent Development**: Each system can be developed independently
- **No Generic Overhead**: No complex generic traits to maintain

### **2. Performance:**
- **System-Specific Optimization**: Each system can optimize for its specific needs
- **Direct Data Access**: No generic abstraction overhead
- **Efficient Memory Usage**: System-specific data structures

### **3. Maintainability:**
- **Clear Boundaries**: Each system is self-contained
- **Easy Debugging**: System-specific issues are isolated
- **Simple Testing**: Each system can be tested independently

### **4. Flexibility:**
- **System-Specific Features**: Each system can have unique features
- **Independent Configuration**: Each system manages its own config
- **Custom Validation**: System-specific validation logic

## 📋 **Implementation Steps**

### **Step 1: Create Base Infrastructure (1 week)**
1. Create base system traits
2. Create specialized registries structure
3. Create factory managers

### **Step 2: Implement Elemental System (1 week)**
1. Move existing elemental code to new structure
2. Create elemental-specific registry, factory, aggregator
3. Test elemental system functionality

### **Step 3: Add New Systems (2 weeks each)**
1. **Cultivation System**: Copy elemental structure, implement cultivation logic
2. **Magic System**: Copy elemental structure, implement magic logic
3. **Race System**: Copy elemental structure, implement race logic

### **Step 4: Integration & Testing (1 week)**
1. Integration testing with all systems
2. Performance testing
3. Documentation update

## 🎯 **Key Advantages Over Generic Approach**

### **❌ Generic Approach Problems:**
- God Registry trying to handle everything
- Complex generic traits become unmaintainable
- Performance overhead from abstractions
- Hard to optimize individual systems

### **✅ Specialized Approach Benefits:**
- Each system has its own components
- Clear separation of concerns
- System-specific optimizations
- Easy to understand and maintain
- Simple to extend with new systems

---

**This specialized architecture provides the perfect balance between extensibility and maintainability while avoiding the "God Registry" problem!** 🎯

### **5. Elemental System Implementation (Example)**

```rust
// src/systems/elemental/elemental_system.rs

use async_trait::async_trait;
use std::collections::HashMap;
use crate::systems::base::{HierarchicalSystem, SystemData, Contribution};
use crate::systems::elemental::elemental_data::ElementalData;

pub struct ElementalSystem {
    base: BaseHierarchicalSystem,
    elemental_data: HashMap<String, ElementalData>,
}

impl ElementalSystem {
    pub fn new() -> Self {
        Self {
            base: BaseHierarchicalSystem::new(
                "elemental".to_string(),
                "Elemental System".to_string(),
                100, // High priority
            ).with_dependencies(vec![]),
            elemental_data: HashMap::new(),
        }
    }
}

#[async_trait]
impl HierarchicalSystem for ElementalSystem {
    fn system_id(&self) -> &str {
        self.base.system_id()
    }
    
    fn system_name(&self) -> &str {
        self.base.system_name()
    }
    
    fn system_priority(&self) -> i32 {
        self.base.system_priority()
    }
    
    async fn is_enabled(&self, actor_id: &str) -> bool {
        self.base.is_enabled(actor_id).await
    }
    
    async fn get_system_data(&self, actor_id: &str) -> ActorCoreResult<SystemData> {
        if let Some(elemental_data) = self.elemental_data.get(actor_id) {
            Ok(SystemData {
                system_id: self.system_id().to_string(),
                actor_id: actor_id.to_string(),
                data: elemental_data.to_serializable_data(),
                metadata: HashMap::new(),
                version: 1,
                last_updated: chrono::Utc::now(),
            })
        } else {
            // Return default elemental data
            let default_data = ElementalData::default();
            Ok(SystemData {
                system_id: self.system_id().to_string(),
                actor_id: actor_id.to_string(),
                data: default_data.to_serializable_data(),
                metadata: HashMap::new(),
                version: 1,
                last_updated: chrono::Utc::now(),
            })
        }
    }
    
    async fn update_system_data(&self, actor_id: &str, data: SystemData) -> ActorCoreResult<()> {
        let elemental_data = ElementalData::from_serializable_data(&data.data)?;
        // Update the data (this would need to be mutable, so we'd need Arc<Mutex<>>)
        Ok(())
    }
    
    async fn calculate_contributions(&self, actor_id: &str) -> ActorCoreResult<Vec<Contribution>> {
        if let Some(elemental_data) = self.elemental_data.get(actor_id) {
            let mut contributions = Vec::new();
            
            // Add elemental mastery contributions
            for (element_id, mastery_level) in elemental_data.element_mastery_levels.iter().enumerate() {
                contributions.push(Contribution {
                    dimension: format!("element_{}_mastery", element_id),
                    value: *mastery_level,
                    bucket: "additive".to_string(),
                    operator: "sum".to_string(),
                    metadata: HashMap::new(),
                });
            }
            
            Ok(contributions)
        } else {
            Ok(Vec::new())
        }
    }
    
    fn get_dependencies(&self) -> Vec<String> {
        self.base.get_dependencies()
    }
    
    async fn validate_data(&self, data: &SystemData) -> ActorCoreResult<()> {
        // Validate elemental data
        ElementalData::from_serializable_data(&data.data)?;
        Ok(())
    }
    
    async fn initialize(&mut self) -> ActorCoreResult<()> {
        // Initialize elemental system
        Ok(())
    }
    
    async fn shutdown(&mut self) -> ActorCoreResult<()> {
        // Shutdown elemental system
        Ok(())
    }
}
```

## 🚀 **Benefits of New Architecture**

### **1. Extensibility:**
- Easy to add new systems (cultivation, magic, race, etc.)
- Generic system trait allows uniform management
- Factory pattern for dynamic system creation

### **2. Maintainability:**
- Clear separation of concerns
- Each system is self-contained
- Common functionality in base classes

### **3. Performance:**
- Dynamic loading of only needed systems
- Efficient system registry management
- Generic aggregation engine

### **4. Flexibility:**
- Systems can be enabled/disabled per actor
- Dependency management between systems
- Configuration-driven system setup

## 📋 **Migration Plan**

### **Phase 1: Core Infrastructure (1 week)**
1. Create base system traits and implementations
2. Implement dynamic system registry
3. Create system factory pattern

### **Phase 2: Elemental System Migration (1 week)**
1. Migrate existing elemental code to new structure
2. Implement elemental system using new traits
3. Test elemental system functionality

### **Phase 3: Additional Systems (2 weeks)**
1. Implement cultivation system
2. Implement magic system
3. Implement race system

### **Phase 4: Integration & Testing (1 week)**
1. Integration testing with all systems
2. Performance testing
3. Documentation update

---

**This new architecture provides a solid foundation for extensible hierarchical systems while maintaining performance and maintainability!** 🎯
