# Status Core System Design

## 📋 **Tổng Quan**

Status Core là hệ thống trung tâm quản lý tất cả status effects, buffs, debuffs, và immunity effects trong game. Hệ thống được thiết kế với plugin architecture, configuration-driven approach, và seamless integration với các hệ thống khác.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Plugin-Based Architecture**
- **Dynamic Loading**: Load plugins tại runtime
- **Hot Reload**: Reload plugins mà không restart server
- **Isolation**: Plugins hoạt động độc lập, không ảnh hưởng lẫn nhau
- **Configuration-Driven**: Plugin discovery và loading qua configuration

### **2. Flexible Status Management**
- **Status Categories**: Hierarchical category system
- **Status Tags**: Flexible tagging system
- **Status Interactions**: Complex interaction rules
- **Status Effects**: Extensible effect system

### **3. Seamless Integration**
- **Zero Breaking Changes**: Không phá vỡ existing systems
- **Backward Compatibility**: Tương thích với code cũ
- **Gradual Migration**: Migration từng bước, không cần rewrite toàn bộ
- **Performance Neutral**: Không ảnh hưởng performance của existing systems

### **4. Condition Core Integration**
- **Standardized Condition Logic**: Sử dụng Condition Core cho tất cả status conditions
- **Unified Condition Functions**: Tái sử dụng condition functions across systems
- **Centralized Condition Management**: Quản lý tập trung tất cả conditions
- **Cross-System Condition Reuse**: Các hệ thống khác có thể tái sử dụng status conditions

## 🏗️ **Kiến Trúc Status Core**

```
Status Core
├── Plugin System
│   ├── StatusPlugin Trait
│   ├── Plugin Registry
│   ├── Plugin Loader
│   └── Plugin Cache
├── Category System
│   ├── Status Categories
│   ├── Category Hierarchy
│   ├── Category Interactions
│   └── Category Tags
├── Effect System
│   ├── Status Effects
│   ├── Effect Types
│   ├── Effect Magnitude
│   └── Effect Duration
├── Interaction System
│   ├── Status Interactions
│   ├── Interaction Types
│   ├── Interaction Rules
│   └── Interaction Priorities
├── Integration System
│   ├── Condition Core Integration
│   │   ├── Status Data Provider
│   │   ├── Status Condition Functions
│   │   ├── Status Condition Registry
│   │   └── Status Condition Evaluation
│   ├── Element Core Bridge
│   ├── Action Core Bridge
│   ├── Combat Core Bridge
│   └── Integration Cache
├── Configuration System
│   ├── YAML Configuration
│   ├── Schema Validation
│   ├── Hot Reload
│   └── Environment Overrides
└── Performance Optimization
    ├── Caching System
    ├── Batch Processing
    ├── Memory Management
    └── Concurrency Control
```

## 📚 **Documentation Structure**

### **Core Design Documents**
- [00_Status_Core_Design_Notes.md](00_Status_Core_Design_Notes.md) - Initial design notes và requirements
- [01_Status_Core_System_Consistency_Integration.md](01_Status_Core_System_Consistency_Integration.md) - System consistency và integration analysis
- [02_Status_Core_Plugin_System_Design.md](02_Status_Core_Plugin_System_Design.md) - Plugin-based architecture design
- [03_Status_Core_Integration_System_Design.md](03_Status_Core_Integration_System_Design.md) - Integration system design
- [04_Status_Core_Configuration_System_Design.md](04_Status_Core_Configuration_System_Design.md) - Configuration system design

### **Implementation Documents**
- [05_Status_Core_Core_System_Design.md](05_Status_Core_Core_System_Design.md) - Core system components design
- [06_Status_Core_API_Design.md](06_Status_Core_API_Design.md) - API design và interfaces
- [07_Status_Core_Performance_Design.md](07_Status_Core_Performance_Design.md) - Performance optimization design
- [08_Status_Core_Error_Handling_Design.md](08_Status_Core_Error_Handling_Design.md) - Error handling design
- [09_Status_Core_Testing_Strategy.md](09_Status_Core_Testing_Strategy.md) - Testing strategy và framework
- [10_Status_Core_Implementation_Guide.md](10_Status_Core_Implementation_Guide.md) - Step-by-step implementation guide

### **Integration & Flow Documents**
- [11_Burning_Status_Combat_Flow_Diagram.md](11_Burning_Status_Combat_Flow_Diagram.md) - Visual flow diagram cho burning status trong combat
- [12_Status_Core_Combat_Integration_Design.md](12_Status_Core_Combat_Integration_Design.md) - Optimized integration design với CombatCore
- [13_Condition_Core_Status_Core_Integration.md](13_Condition_Core_Status_Core_Integration.md) - Integration design với Condition Core

### **Configuration Files**
- **configs/status_plugins.yaml** - Plugin configurations
- **configs/status_categories.yaml** - Category configurations
- **configs/status_effects.yaml** - Effect configurations
- **configs/status_interactions.yaml** - Interaction configurations
- **configs/global_settings.yaml** - Global system settings

## 🔧 **Key Features**

### **1. Plugin System**
- **Dynamic Loading**: Load plugins tại runtime
- **Hot Reload**: Reload plugins mà không restart server
- **Plugin Registry**: Centralized plugin management
- **Plugin Cache**: Performance optimization cho plugins

### **2. Category System**
- **Hierarchical Categories**: Nested category structure
- **Category Tags**: Flexible tagging system
- **Category Interactions**: Complex interaction rules
- **Dynamic Categories**: Create categories tại runtime

### **3. Effect System**
- **Status Effects**: Comprehensive status effect system
- **Effect Types**: Multiple effect types (Elemental, Combat, Movement, Resource)
- **Effect Magnitude**: Configurable magnitude calculation
- **Effect Duration**: Configurable duration calculation

### **4. Interaction System**
- **Status Interactions**: Complex interaction rules
- **Interaction Types**: Multiple interaction types (Amplify, Suppress, Transform, etc.)
- **Interaction Priorities**: Priority-based interaction resolution
- **Conditional Interactions**: Condition-based interactions

### **5. Integration System**
- **Element Core Bridge**: Integration với Element Core
- **Action Core Bridge**: Integration với Action Core
- **Combat Core Bridge**: Integration với Combat Core
- **Seamless Integration**: Zero breaking changes

### **6. Configuration System**
- **YAML Configuration**: Human-readable configuration
- **Schema Validation**: JSON Schema validation
- **Hot Reload**: Runtime configuration changes
- **Environment Overrides**: Environment-specific configuration

### **7. Condition Core Integration**
- **Status Data Provider**: Interface để cung cấp status data cho Condition Core
- **Status Condition Functions**: 25+ status condition functions
- **Status Condition Registry**: Registry để quản lý status condition functions
- **Status Condition Evaluation**: Engine để evaluate status conditions

## 🚀 **Performance Optimization**

### **1. Caching Strategy**
- **Plugin Cache**: Cache plugin data và calculations
- **Effect Cache**: Cache status effect definitions
- **Category Cache**: Cache category definitions
- **Integration Cache**: Cache integration data

### **2. Batch Processing**
- **Effect Processing**: Process multiple effects efficiently
- **Integration Processing**: Process multiple integrations efficiently
- **Configuration Processing**: Process configuration changes efficiently
- **Memory Management**: Efficient memory usage

### **3. Concurrency Control**
- **Thread-Safe Operations**: Thread-safe plugin operations
- **Async Processing**: Non-blocking operations
- **Resource Management**: Efficient resource usage
- **Error Handling**: Graceful error handling

## 🎯 **Condition Core Integration**

### **1. Status Data Provider**

Status Core implements `StatusDataProvider` trait để cung cấp status data cho Condition Core:

```rust
/// Status Data Provider Interface
#[async_trait]
pub trait StatusDataProvider: Send + Sync {
    // Basic Status Functions
    async fn has_status_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_effect_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32>;
    async fn get_status_effect_magnitude(&self, actor_id: &str, effect_id: &str) -> ConditionResult<f64>;
    async fn is_status_effect_active(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    
    // Status Immunity Functions
    async fn has_status_immunity(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_immunity_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32>;
    
    // Status Category Functions
    async fn has_status_category(&self, actor_id: &str, category: &str) -> ConditionResult<bool>;
    async fn get_status_category_count(&self, actor_id: &str, category: &str) -> ConditionResult<u32>;
    
    // Status Interaction Functions
    async fn is_status_effect_stackable(&self, effect_id: &str) -> ConditionResult<bool>;
    async fn can_status_effect_stack(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn get_status_effect_interaction(&self, effect_id: &str, target_effect_id: &str) -> ConditionResult<String>;
    
    // Status Movement Functions
    async fn has_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<bool>;
    async fn get_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<f64>;
    
    // Status Visual/Audio Functions
    async fn has_status_visual_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    async fn has_status_audio_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool>;
    
    // Status Properties Functions
    async fn get_status_effect_properties(&self, actor_id: &str, effect_id: &str) -> ConditionResult<HashMap<String, serde_json::Value>>;
    async fn has_status_effect_property(&self, actor_id: &str, effect_id: &str, property: &str) -> ConditionResult<bool>;
    
    // Status History Functions
    async fn get_status_effect_history(&self, actor_id: &str, effect_id: &str) -> ConditionResult<Vec<StatusEffectHistory>>;
    async fn get_status_effect_timeline(&self, actor_id: &str, effect_id: &str) -> ConditionResult<Vec<StatusEffectTimeline>>;
}
```

### **2. Standardized Status Conditions**

Status Core cung cấp 25+ status condition functions cho Condition Core:

```yaml
# Status Effect Conditions
has_status_effect: "Check if actor has specific status effect"
get_status_effect_count: "Get count of specific status effect"
get_status_effect_magnitude: "Get magnitude of specific status effect"
is_status_effect_active: "Check if status effect is active"
is_status_effect_expired: "Check if status effect is expired"

# Status Immunity Conditions
has_status_immunity: "Check if actor has immunity to specific effect"
get_status_immunity_count: "Get count of specific immunity"
is_status_immunity_active: "Check if immunity is active"

# Status Category Conditions
has_status_category: "Check if actor has effects in specific category"
get_status_category_count: "Get count of effects in specific category"
list_status_categories: "List all status categories for actor"

# Status Interaction Conditions
is_status_effect_stackable: "Check if effect can be stacked"
can_status_effect_stack: "Check if effect can stack on actor"
get_status_effect_interaction: "Get interaction between two effects"

# Status Movement Conditions
has_status_movement_restriction: "Check if actor has movement restriction"
get_status_movement_restriction: "Get magnitude of movement restriction"

# Status Visual/Audio Conditions
has_status_visual_effect: "Check if actor has visual effect"
has_status_audio_effect: "Check if actor has audio effect"

# Status Properties Conditions
get_status_effect_properties: "Get all properties of status effect"
has_status_effect_property: "Check if effect has specific property"
get_status_effect_property: "Get specific property value"

# Status History Conditions
get_status_effect_history: "Get history of status effect"
get_status_effect_timeline: "Get timeline of status effect"
```

### **3. Cross-System Condition Reuse**

Các hệ thống khác có thể sử dụng status conditions thông qua Condition Core:

```rust
// Combat Core sử dụng status conditions
use chaos_condition_core::{ConditionCore, StatusDataProvider};

pub struct CombatCore {
    condition_core: Arc<ConditionCore>,
    status_data_provider: Arc<dyn StatusDataProvider>,
}

impl CombatCore {
    // Check if target can be stunned
    pub async fn can_stun_target(&self, target_id: &str) -> Result<bool, CombatError> {
        let context = ConditionContext::new(target_id);
        let result = self.condition_core.evaluate_condition(
            "has_status_immunity",
            &[ConditionParameter::String("stun".to_string())],
            &context
        ).await?;
        Ok(!result) // Can stun if not immune
    }
    
    // Check if attacker has damage bonus
    pub async fn has_damage_bonus(&self, attacker_id: &str) -> Result<bool, CombatError> {
        let context = ConditionContext::new(attacker_id);
        let result = self.condition_core.evaluate_condition(
            "has_status_effect",
            &[ConditionParameter::String("damage_bonus".to_string())],
            &context
        ).await?;
        Ok(result)
    }
}

// Action Core sử dụng status conditions
pub struct ActionCore {
    condition_core: Arc<ConditionCore>,
    status_data_provider: Arc<dyn StatusDataProvider>,
}

impl ActionCore {
    // Check if action can be executed
    pub async fn can_execute_action(&self, actor_id: &str, action_id: &str) -> Result<bool, ActionError> {
        let context = ConditionContext::new(actor_id);
        
        // Check if actor is stunned
        let is_stunned = self.condition_core.evaluate_condition(
            "has_status_effect",
            &[ConditionParameter::String("stun".to_string())],
            &context
        ).await?;
        
        if is_stunned {
            return Ok(false);
        }
        
        // Check if actor has required status effects
        let has_required_effect = self.condition_core.evaluate_condition(
            "has_status_effect",
            &[ConditionParameter::String("action_ready".to_string())],
            &context
        ).await?;
        
        Ok(has_required_effect)
    }
}
```

### **4. Benefits of Condition Core Integration**

#### **Standardized Logic**
- **Consistent Behavior**: Tất cả systems sử dụng cùng logic cho status conditions
- **Unified API**: Single API cho tất cả status condition evaluation
- **Centralized Management**: Quản lý tập trung tất cả status conditions

#### **Cross-System Reuse**
- **Code Reuse**: Tái sử dụng status condition logic across systems
- **Maintenance**: Dễ dàng maintain và update status conditions
- **Testing**: Test status conditions một lần, sử dụng everywhere

#### **Performance**
- **Centralized Caching**: Cache status condition results
- **Batch Processing**: Process multiple status conditions efficiently
- **Memory Optimization**: Optimize memory usage cho status data

## 🧪 **Testing Strategy**

### **1. Unit Testing**
- **Plugin Testing**: Test individual plugins
- **Category Testing**: Test category system
- **Effect Testing**: Test effect system
- **Interaction Testing**: Test interaction system

### **2. Integration Testing**
- **System Integration**: Test integration với other systems
- **Plugin Integration**: Test plugin integration
- **Configuration Integration**: Test configuration system
- **Performance Testing**: Test performance characteristics

### **3. End-to-End Testing**
- **Complete Workflow**: Test complete status effect workflow
- **Real-world Scenarios**: Test real-world usage scenarios
- **Stress Testing**: Test under high load
- **Regression Testing**: Test for regressions

## 📝 **Implementation Phases**

### **Phase 1: Core Infrastructure (2-3 weeks)**
1. **Plugin System**: Basic plugin architecture
2. **Category System**: Basic category system
3. **Effect System**: Basic effect system
4. **Configuration System**: Basic configuration system

### **Phase 2: Integration System (2-3 weeks)**
1. **Element Core Integration**: Integration với Element Core
2. **Action Core Integration**: Integration với Action Core
3. **Combat Core Integration**: Integration với Combat Core
4. **Performance Optimization**: Optimize performance

### **Phase 3: Advanced Features (2-3 weeks)**
1. **Advanced Interactions**: Complex interaction rules
2. **Hot Reload**: Runtime configuration changes
3. **Plugin Management**: Advanced plugin management
4. **Monitoring**: System monitoring và logging

### **Phase 4: Polish & Optimization (1-2 weeks)**
1. **Performance Tuning**: Fine-tune performance
2. **Memory Optimization**: Optimize memory usage
3. **Documentation**: Complete documentation
4. **Testing**: Comprehensive testing

## 💡 **Benefits**

### **1. Flexibility**
- **Plugin Architecture**: Easy to extend với new plugins
- **Configuration-Driven**: Easy to configure và customize
- **Dynamic Loading**: Load/unload features tại runtime
- **Hot Reload**: Update configuration mà không restart

### **2. Performance**
- **Smart Caching**: Intelligent caching cho performance
- **Batch Processing**: Process multiple operations efficiently
- **Memory Management**: Efficient memory usage
- **Concurrency**: Thread-safe operations

### **3. Integration**
- **Seamless Integration**: Zero breaking changes
- **Backward Compatibility**: Tương thích với existing systems
- **Gradual Migration**: Migration từng bước
- **Performance Neutral**: Không ảnh hưởng performance

### **4. Developer Experience**
- **Clear API**: Easy to use API cho developers
- **Comprehensive Documentation**: Detailed documentation
- **Testing Support**: Built-in testing utilities
- **Debugging Tools**: Tools cho debugging và monitoring

## 🔗 **Related Systems**

- **Element Core**: Provides elemental status effects
- **Action Core**: Provides action-based status effects
- **Combat Core**: Provides combat status effects
- **Actor Core**: Provides actor state management
- **Resource Manager**: Handles resource management

## 📈 **Success Metrics**

### **Technical Metrics**
- **Performance**: < 0.1ms status effect processing
- **Memory**: < 100MB for 1000 actors
- **Cache Hit Rate**: > 95%
- **Throughput**: > 10,000 status effects/second

### **Developer Metrics**
- **API Usability**: Easy to use API
- **Documentation Quality**: Comprehensive docs
- **Testing Coverage**: > 90% test coverage
- **Bug Rate**: < 1% bug rate

### **Game Metrics**
- **Status Responsiveness**: < 100ms end-to-end
- **Effect Accuracy**: 100% accurate effect calculations
- **System Stability**: 99.9% uptime
- **Player Experience**: Smooth status effect execution

## ❓ **Questions for Discussion**

1. **Status Effect Complexity**: Có nên có complex status effects với multiple layers?
2. **Performance Requirements**: Làm thế nào để optimize cho high-load scenarios?
3. **Plugin Security**: Làm thế nào để ensure plugin security?
4. **Configuration Management**: Có nên có advanced configuration management features?
5. **Integration Strategy**: Làm thế nào để migrate existing status systems?

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
