# Element Registry Design

## 📋 **Overview**

This document defines the design for the Unified Element Registry, the central data storage and management component of the Element-Core system. This registry consolidates all previous registry implementations into a single, unified approach that serves as the single source of truth for all elemental data.

**Version**: 2.0  
**Last Updated**: 2024-12-19  
**Status**: Active

### **Consolidation Summary**
This document consolidates features from:
- **ElementRegistry** (basic functionality)
- **UniversalElementRegistry** (advanced features)
- **ElementContributorRegistry** (external integration)
- **ElementalCategoryRegistry** (category management)
- **ElementPluginRegistry** (plugin system)

---

## 🎯 **Design Goals**

### **1. Unified Architecture**
- **Single Source of Truth**: All element data in one registry
- **Consistent Patterns**: Standardized interfaces and operations
- **Eliminate Duplication**: Remove multiple conflicting implementations

### **2. High Performance**
- **Fast Lookup**: Optimized element retrieval
- **Efficient Caching**: Smart caching strategies
- **Memory Optimization**: Minimal memory footprint

### **3. Comprehensive Features**
- **Element Management**: CRUD operations for elements
- **System Integration**: External contributor pattern
- **Category Management**: Element classification and organization
- **Plugin Support**: Extensible plugin architecture
- **Interaction Matrix**: Element interaction calculations

### **4. Developer Experience**
- **Simple API**: Easy-to-use interfaces
- **Clear Documentation**: Comprehensive examples
- **Error Handling**: Robust error management
- **Testing Support**: Built-in testing utilities

---

## 🏗️ **Registry Architecture**

### **Core Registry Structure**

```rust
/// Unified Element Registry - Single source of truth
pub struct UnifiedElementRegistry {
    /// Core element definitions
    elements: HashMap<String, ElementDefinition>,
    
    /// System registrations
    system_registrations: HashMap<String, SystemRegistration>,
    
    /// External contributors
    contributors: HashMap<String, Arc<dyn ElementContributor>>,
    
    /// Category management
    categories: HashMap<String, ElementalCategory>,
    
    /// Plugin management
    plugins: HashMap<String, Arc<dyn ElementPlugin>>,
    
    /// Interaction matrix
    interaction_matrix: HashMap<(String, String), ElementInteraction>,
    
    /// Configuration
    config: RegistryConfig,
    
    /// Performance metrics
    metrics: RegistryMetrics,
}
```

### **Element Definition**

```rust
/// Core element definition
pub struct ElementDefinition {
    /// Unique identifier
    pub id: String,
    
    /// Display name
    pub name: String,
    
    /// Element description
    pub description: String,
    
    /// Element category
    pub category: ElementCategory,
    
    /// Base properties
    pub base_properties: ElementProperties,
    
    /// Interaction rules
    pub interaction_rules: Vec<ElementInteractionRule>,
    
    /// Derived stats configuration
    pub derived_stats_config: DerivedStatsConfig,
    
    /// Metadata
    pub metadata: ElementMetadata,
}
```

---

## 📊 **Data Management**

### **Element Storage**

```rust
impl UnifiedElementRegistry {
    /// Register a new element
    pub async fn register_element(&mut self, element: ElementDefinition) -> ElementCoreResult<()> {
        // Validate element definition
        self.validate_element(&element)?;
        
        // Store element
        self.elements.insert(element.id.clone(), element);
        
        // Update interaction matrix
        self.update_interaction_matrix(&element.id).await?;
        
        // Notify registered systems
        self.notify_element_registered(&element.id).await?;
        
        Ok(())
    }
    
    /// Get element by ID
    pub fn get_element(&self, element_id: &str) -> Option<&ElementDefinition> {
        self.elements.get(element_id)
    }
    
    /// Get all elements
    pub fn get_all_elements(&self) -> &HashMap<String, ElementDefinition> {
        &self.elements
    }
}
```

### **System Registration**

```rust
/// System registration for external contributors
pub struct SystemRegistration {
    /// System identifier
    pub system_id: String,
    
    /// System name
    pub system_name: String,
    
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    
    /// System capabilities
    pub capabilities: Vec<SystemCapability>,
    
    /// Configuration
    pub config: SystemConfig,
}

impl UnifiedElementRegistry {
    /// Register external system
    pub async fn register_system(&mut self, system: SystemRegistration) -> ElementCoreResult<()> {
        // Validate system registration
        self.validate_system(&system)?;
        
        // Store registration
        self.system_registrations.insert(system.system_id.clone(), system);
        
        // Initialize system integration
        self.initialize_system_integration(&system.system_id).await?;
        
        Ok(())
    }
}
```

---

## 🔧 **Performance Optimization**

### **Caching Strategy**

```rust
/// Registry cache for performance optimization
pub struct RegistryCache {
    /// Element lookup cache
    element_cache: LruCache<String, ElementDefinition>,
    
    /// Interaction matrix cache
    interaction_cache: LruCache<(String, String), ElementInteraction>,
    
    /// System registration cache
    system_cache: LruCache<String, SystemRegistration>,
    
    /// Cache configuration
    config: CacheConfig,
}

impl RegistryCache {
    /// Get element with caching
    pub fn get_element_cached(&mut self, element_id: &str) -> Option<&ElementDefinition> {
        if let Some(element) = self.element_cache.get(element_id) {
            return Some(element);
        }
        
        // Cache miss - load from registry
        if let Some(element) = self.registry.get_element(element_id) {
            self.element_cache.put(element_id.to_string(), element.clone());
            Some(element)
        } else {
            None
        }
    }
}
```

### **Memory Management**

```rust
/// Memory-efficient element storage
pub struct ElementStorage {
    /// Compact element data
    elements: Vec<ElementData>,
    
    /// Index for fast lookup
    element_index: HashMap<String, usize>,
    
    /// Memory pool for allocations
    memory_pool: MemoryPool<ElementData>,
}

impl ElementStorage {
    /// Get element by ID with memory optimization
    pub fn get_element(&self, element_id: &str) -> Option<&ElementData> {
        if let Some(&index) = self.element_index.get(element_id) {
            self.elements.get(index)
        } else {
            None
        }
    }
}
```

---

## 🔗 **Integration Patterns**

### **External System Integration**

```rust
/// External system integration trait
pub trait ElementContributor: Send + Sync {
    /// Get system identifier
    fn system_id(&self) -> &str;
    
    /// Contribute element data
    async fn contribute_element_data(&self, element_id: &str) -> ElementCoreResult<ElementContribution>;
    
    /// Handle element events
    async fn handle_element_event(&self, event: &ElementEvent) -> ElementCoreResult<()>;
}

impl UnifiedElementRegistry {
    /// Register external contributor
    pub async fn register_contributor(&mut self, contributor: Arc<dyn ElementContributor>) -> ElementCoreResult<()> {
        let system_id = contributor.system_id().to_string();
        self.contributors.insert(system_id, contributor);
        Ok(())
    }
}
```

### **Event System**

```rust
/// Element registry events
pub enum ElementRegistryEvent {
    /// Element registered
    ElementRegistered { element_id: String },
    
    /// Element updated
    ElementUpdated { element_id: String },
    
    /// Element removed
    ElementRemoved { element_id: String },
    
    /// System registered
    SystemRegistered { system_id: String },
    
    /// System unregistered
    SystemUnregistered { system_id: String },
}

impl UnifiedElementRegistry {
    /// Notify event subscribers
    async fn notify_event(&self, event: ElementRegistryEvent) -> ElementCoreResult<()> {
        for contributor in self.contributors.values() {
            contributor.handle_element_event(&ElementEvent::RegistryEvent(event.clone())).await?;
        }
        Ok(())
    }
}
```

---

## 📚 **Configuration Management**

### **Registry Configuration**

```rust
/// Registry configuration
pub struct RegistryConfig {
    /// Maximum number of elements
    pub max_elements: usize,
    
    /// Cache size limits
    pub cache_limits: CacheLimits,
    
    /// Performance settings
    pub performance: PerformanceConfig,
    
    /// Security settings
    pub security: SecurityConfig,
    
    /// Logging settings
    pub logging: LoggingConfig,
}

/// Cache configuration
pub struct CacheLimits {
    /// Element cache size
    pub element_cache_size: usize,
    
    /// Interaction cache size
    pub interaction_cache_size: usize,
    
    /// System cache size
    pub system_cache_size: usize,
}
```

### **Dynamic Configuration**

```rust
impl UnifiedElementRegistry {
    /// Update configuration at runtime
    pub async fn update_config(&mut self, config: RegistryConfig) -> ElementCoreResult<()> {
        // Validate new configuration
        self.validate_config(&config)?;
        
        // Apply configuration changes
        self.apply_config_changes(&config).await?;
        
        // Update internal state
        self.config = config;
        
        Ok(())
    }
}
```

---

## 🚀 **Usage Examples**

### **Basic Element Registration**

```rust
// Create element definition
let fire_element = ElementDefinition {
    id: "fire".to_string(),
    name: "Fire".to_string(),
    description: "Element of flame and heat".to_string(),
    category: ElementCategory::Elemental,
    base_properties: ElementProperties {
        power: 100.0,
        defense: 80.0,
        affinity: 1.0,
        resistance: 0.8,
    },
    interaction_rules: vec![
        ElementInteractionRule {
            target_element: "water".to_string(),
            interaction_type: InteractionType::Overcome,
            multiplier: 1.5,
        },
    ],
    derived_stats_config: DerivedStatsConfig::default(),
    metadata: ElementMetadata::default(),
};

// Register element
registry.register_element(fire_element).await?;
```

### **System Integration**

```rust
// Create system registration
let race_system = SystemRegistration {
    system_id: "race-core".to_string(),
    system_name: "Race Core System".to_string(),
    registered_at: Utc::now(),
    capabilities: vec![
        SystemCapability::ElementAffinity,
        SystemCapability::ElementResistance,
    ],
    config: SystemConfig::default(),
};

// Register system
registry.register_system(race_system).await?;
```

### **Element Lookup**

```rust
// Get element by ID
if let Some(element) = registry.get_element("fire") {
    println!("Fire element: {}", element.name);
    println!("Power: {}", element.base_properties.power);
}

// Get all elements
for (id, element) in registry.get_all_elements() {
    println!("Element {}: {}", id, element.name);
}
```

---

## 📚 **Related Documents**

- [Unified Architecture Design](20_Unified_Architecture_Design.md) - Target architecture
- [Migration Guide](21_Migration_Guide.md) - Migration from old registries
- [Element System Architecture](01_Element_System_Architecture.md) - Overall system architecture
- [Element Core Overview](00_Element_Core_Overview.md) - Main overview

### **Deprecated Documents (Merged into this document)**
- ~~[Universal Element Registry Design](18_Universal_Element_Registry_Design.md)~~ - **DEPRECATED**
- ~~[Stats Distribution Design](19_Stats_Distribution_Design.md)~~ - **DEPRECATED**
- ~~[Elemental Category System Design](17_Elemental_Category_System_Design.md)~~ - **DEPRECATED**
- ~~[Elemental Plugin System Design](16_Elemental_Plugin_System_Design.md)~~ - **DEPRECATED**

---

## ⚖️ **Balance Considerations**

### **Performance vs Memory**
- **Performance**: Fast lookup and retrieval
- **Memory**: Efficient storage and caching

### **Simplicity vs Features**
- **Simplicity**: Clear, understandable design
- **Features**: Rich functionality and extensibility

### **Consistency vs Flexibility**
- **Consistency**: Standardized data formats
- **Flexibility**: Support for custom elements

---

## 🔄 **Evolution Strategy**

### **Version 2.0 (Current)**
- Unified registry architecture
- Consolidated features from all previous registries
- External contributor pattern
- Performance optimization

### **Version 3.0 (Future)**
- Advanced caching strategies
- Machine learning integration
- Enhanced performance monitoring

### **Version 4.0 (Future)**
- AI-powered optimization
- Predictive caching
- Advanced analytics

---

**Last Updated**: 2024-12-19  
**Version**: 2.0  
**Status**: Active  
**Next Review**: 2024-12-26
