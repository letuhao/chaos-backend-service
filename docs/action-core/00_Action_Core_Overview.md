# Action Core Overview

## 📋 **Tổng Quan**

Action Core là hệ thống trung tâm xử lý tất cả các actions trong game, đặc biệt tập trung vào việc tích hợp derived stats từ Element Core vào damage calculation trong Combat Core. Hệ thống này đóng vai trò cầu nối giữa Element Core (stats) và Combat Core (damage calculation).

## 🎯 **Vấn Đề Cần Giải Quyết**

### **1. Action Definition Interface**
- Cần **unified interface** để định nghĩa actions
- Actions cần **flexible configuration** cho các properties
- Cần **type-safe** action definitions

### **2. Resource Management Integration**
- Actions tiêu tốn **multiple resources** (HP, Mana, Stamina, Qi)
- Cần **bridge với Resource Manager** để quản lý tài nguyên
- Cần **validation** cho resource requirements

### **3. Action Timing Management**
- Actions có **execution duration** cần được quản lý
- Cần **interrupt mechanism** theo điều kiện
- Cần **cooldown management** với các điều kiện phức tạp

### **4. Action Categories & Types**
- **Combat Actions**: Attack, skill, movement trong combat
- **Lifestyle Actions**: Sinh hoạt, crafting, social (sẽ bàn trong modules khác)
- **Combat Actions** cần **detailed targeting system** và **effect management**

### **5. Derived Stats Integration**
- Element Core có **hàng trăm derived stats** ảnh hưởng đến action execution
- Cần **bridge với Resource Manager** để quản lý tài nguyên
- Cần **unified interface** cho tất cả action types

## 🏗️ **Kiến Trúc Action Core**

### **Core Components**

```
Action Core
├── Action Definition System
│   ├── Action Interface
│   ├── Action Builder
│   ├── Action Validator
│   └── Action Registry
├── Resource Management Bridge
│   ├── Resource Validator
│   ├── Resource Consumer
│   ├── Resource Regenerator
│   └── Resource Manager Client
├── Action Timing System
│   ├── Execution Duration Manager
│   ├── Interrupt Manager
│   ├── Cooldown Manager
│   └── Timing Validator
├── Combat Action System
│   ├── Attack Action Handler
│   ├── Targeting System
│   ├── Effect System
│   └── Projectile System
├── Derived Stats Integration
│   ├── Stats Aggregator
│   ├── Stats Cache Manager
│   ├── Stats Calculator
│   └── Stats Validator
└── Performance Optimization
    ├── Hot Path Optimizer
    ├── Batch Processor
    ├── Cache Manager
    └── Memory Pool
```

## 🔧 **Key Features**

### **1. Unified Action Interface**
```rust
pub trait Action {
    fn execute(&self, context: &ActionContext) -> ActionResult;
    fn validate(&self, context: &ActionContext) -> ValidationResult;
    fn get_required_stats(&self) -> Vec<String>;
    fn get_affected_stats(&self) -> Vec<String>;
}
```

### **2. Derived Stats Integration**
```rust
pub struct ActionContext {
    pub actor: Actor,
    pub target: Option<Actor>,
    pub derived_stats: DerivedStatsSnapshot,
    pub element_stats: ElementStatsSnapshot,
    pub combat_stats: CombatStatsSnapshot,
    pub environment: EnvironmentContext,
}
```

### **3. Performance Optimization**
- **Hot Path Optimization**: Critical path cho damage calculation
- **Batch Processing**: Multiple actions cùng lúc
- **Smart Caching**: Cache derived stats với intelligent invalidation
- **Memory Pool**: Reuse objects để giảm allocation

## 📊 **Integration Points**

### **Element Core Integration**
- **Stats Provider**: Access derived stats từ Element Core
- **Stats Calculator**: Calculate complex stat interactions
- **Stats Cache**: Cache frequently used stats
- **Stats Validator**: Validate stat consistency

### **Combat Core Integration**
- **Damage Input**: Provide formatted stats cho damage calculation
- **Damage Output**: Process damage results
- **Status Effects**: Handle status effect application
- **Resource Management**: Handle resource consumption/regeneration

### **Actor Core Integration**
- **Actor State**: Access actor state và properties
- **Resource Management**: Handle resource changes
- **Event System**: Trigger events cho action execution
- **Validation**: Validate actor capabilities

## 🚀 **Performance Goals**

### **Target Metrics**
- **Action Execution**: < 0.1ms per action
- **Stats Access**: < 0.01ms per stat lookup
- **Cache Hit Rate**: > 95%
- **Memory Usage**: < 50MB for 1000 actors
- **Throughput**: > 10,000 actions/second

### **Optimization Strategies**
1. **Pre-calculation**: Calculate stats trước khi cần
2. **Caching**: Cache frequently used values
3. **Batch Processing**: Process multiple actions together
4. **Memory Pool**: Reuse objects
5. **Hot Path**: Optimize critical execution path

## 📚 **Documentation Structure**

### **Core Documents**
- **00_Action_Core_Overview.md** - Tổng quan hệ thống
- **01_Action_Execution_Engine.md** - Action execution engine
- **02_Derived_Stats_Integration.md** - Derived stats integration
- **03_Damage_Calculation_Bridge.md** - Bridge với Combat Core
- **04_Action_Types_System.md** - Action types và implementations
- **05_Performance_Optimization.md** - Performance optimization
- **06_Integration_Guide.md** - Integration với other cores

### **Configuration Files**
- **action_config.yaml** - Action system configuration
- **stats_integration.yaml** - Stats integration configuration
- **performance_config.yaml** - Performance optimization config
- **action_schemas.yaml** - Action type schemas

## 🎯 **Implementation Phases**

### **Phase 1: Core Infrastructure (2-3 weeks)**
1. **Action Execution Engine** - Basic action execution
2. **Derived Stats Integration** - Stats access và caching
3. **Basic Action Types** - Attack, skill, movement actions

### **Phase 2: Advanced Features (2-3 weeks)**
1. **Damage Calculation Bridge** - Integration với Combat Core
2. **Performance Optimization** - Caching và optimization
3. **Advanced Action Types** - Complex actions

### **Phase 3: Integration & Testing (1-2 weeks)**
1. **Element Core Integration** - Full integration
2. **Combat Core Integration** - Damage calculation bridge
3. **Testing & Validation** - Comprehensive testing

### **Phase 4: Polish & Optimization (1-2 weeks)**
1. **Performance Tuning** - Fine-tune performance
2. **Memory Optimization** - Optimize memory usage
3. **Documentation** - Complete documentation

## 💡 **Benefits**

### **1. Centralized Action Management**
- **Single Source of Truth**: Tất cả actions được quản lý tập trung
- **Consistent Interface**: Unified interface cho tất cả action types
- **Easy Extension**: Dễ dàng thêm action types mới

### **2. Optimized Performance**
- **Hot Path Optimization**: Critical path được tối ưu
- **Smart Caching**: Intelligent caching cho derived stats
- **Batch Processing**: Process multiple actions efficiently

### **3. Seamless Integration**
- **Element Core**: Seamless access đến derived stats
- **Combat Core**: Optimized damage calculation
- **Actor Core**: Unified actor state management

### **4. Developer Experience**
- **Clear API**: Easy to use API cho developers
- **Comprehensive Documentation**: Detailed documentation
- **Testing Support**: Built-in testing utilities

## 🔗 **Related Systems**

- **Element Core**: Provides derived stats
- **Combat Core**: Uses stats for damage calculation
- **Actor Core**: Provides actor state và capabilities
- **Resource Manager**: Handles resource management
- **Event System**: Handles action events

## 📈 **Success Metrics**

### **Technical Metrics**
- **Performance**: < 0.1ms action execution
- **Memory**: < 50MB for 1000 actors
- **Cache Hit Rate**: > 95%
- **Throughput**: > 10,000 actions/second

### **Developer Metrics**
- **API Usability**: Easy to use API
- **Documentation Quality**: Comprehensive docs
- **Testing Coverage**: > 90% test coverage
- **Bug Rate**: < 1% bug rate

### **Game Metrics**
- **Action Responsiveness**: < 100ms end-to-end
- **Stat Accuracy**: 100% accurate stat calculations
- **System Stability**: 99.9% uptime
- **Player Experience**: Smooth action execution

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
