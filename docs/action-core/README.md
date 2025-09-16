# Action Core Documentation

## 📋 **Tổng Quan**

Action Core là hệ thống trung tâm xử lý tất cả các actions trong game, đặc biệt tập trung vào việc tích hợp derived stats từ Element Core vào damage calculation trong Combat Core. Hệ thống này đóng vai trò cầu nối giữa Element Core (stats) và Combat Core (damage calculation).

## 🎯 **Vấn Đề Cần Giải Quyết**

### **1. Derived Stats Complexity**
- Element Core có **hàng trăm derived stats** ảnh hưởng đến damage calculation
- Combat Core cần access đến các stats này một cách hiệu quả
- Cần hệ thống để **aggregate và optimize** stat access

### **2. Action Execution Integration**
- Actions cần **real-time access** đến derived stats
- Cần **caching và optimization** cho performance
- Cần **unified interface** cho tất cả action types

### **3. Multi-System Coordination**
- Element Core: Provides derived stats
- Combat Core: Uses stats for damage calculation
- Action Core: **Orchestrates** the integration

## 📚 **Danh Sách Tài Liệu**

### **1. [00_Action_Core_Overview.md](./00_Action_Core_Overview.md)**
- **Mục đích**: Tổng quan hệ thống Action Core
- **Nội dung**:
  - Kiến trúc hệ thống
  - Core components
  - Integration points
  - Performance goals
  - Implementation phases

### **2. [01_Action_Execution_Engine.md](./01_Action_Execution_Engine.md)**
- **Mục đích**: Action execution engine chi tiết
- **Nội dung**:
  - Action scheduler và validator
  - Action executor và result processor
  - Stats aggregator và cache manager
  - Hot path optimizer
  - Performance optimization

### **3. [02_Derived_Stats_Integration.md](./02_Derived_Stats_Integration.md)**
- **Mục đích**: Tích hợp derived stats từ Element Core
- **Nội dung**:
  - Stats aggregator và calculator
  - Multi-level caching system
  - Stats categories (combat, skill, resource)
  - Performance optimization
  - Memory management

### **4. [03_Damage_Calculation_Bridge.md](./03_Damage_Calculation_Bridge.md)**
- **Mục đích**: Bridge giữa Element Core và Combat Core
- **Nội dung**:
  - Stats formatter và damage input builder
  - Damage output processor
  - Hot path cache cho damage calculation
  - Batch processing
  - Performance monitoring

### **5. [04_Action_Definition_System.md](./04_Action_Definition_System.md)**
- **Mục đích**: Hệ thống định nghĩa actions với unified interface
- **Nội dung**:
  - Action interface và metadata
  - Resource requirements và consumption
  - Timing management (duration, interrupt, cooldown)
  - Targeting system và effect system
  - Action builder và registry

### **6. [05_Resource_Management_Bridge.md](./05_Resource_Management_Bridge.md)**
- **Mục đích**: Bridge với Resource Manager để quản lý tài nguyên
- **Nội dung**:
  - Resource validation và consumption
  - Resource regeneration
  - Resource Manager integration
  - Performance optimization
  - Error handling và rollback

### **7. [06_Action_Timing_System.md](./06_Action_Timing_System.md)**
- **Mục đích**: Quản lý thời gian thực hiện và cooldown cho actions
- **Nội dung**:
  - Execution duration management
  - Interrupt mechanism
  - Cooldown management
  - Timing validation
  - Performance optimization

### **8. [07_Combat_Action_System.md](./07_Combat_Action_System.md)**
- **Mục đích**: Hệ thống xử lý combat actions
- **Nội dung**:
  - Attack action handling
  - Targeting system (single, multiple, AOE, projectile)
  - Effect system và projectile system
  - Combat effectiveness calculation
  - Derived stats integration

### **9. [08_Execution_Conditions_Integration.md](./08_Execution_Conditions_Integration.md)**
- **Mục đích**: Tích hợp execution conditions vào Action Core
- **Nội dung**:
  - Execution condition validation
  - Condition evaluator và parser
  - Integration với Action Execution Engine
  - Performance optimization
  - Caching và batch processing

## 🏗️ **Kiến Trúc Hệ Thống**

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
├── Damage Calculation Bridge
│   ├── Stats Formatter
│   ├── Damage Input Builder
│   ├── Damage Output Processor
│   └── Combat Core Client
└── Performance Optimization
    ├── Hot Path Optimizer
    ├── Batch Processor
    ├── Cache Manager
    └── Memory Pool
```

## 🚀 **Key Features**

### **1. Unified Action Interface**
- **Consistent API**: Unified interface cho tất cả action types
- **Type Safety**: Strong typing cho action parameters
- **Error Handling**: Comprehensive error handling
- **Validation**: Built-in validation cho actions

### **2. Derived Stats Integration**
- **Real-time Access**: Access derived stats trong real-time
- **Intelligent Caching**: Cache stats với smart invalidation
- **Stat Aggregation**: Aggregate stats từ multiple sources
- **Performance Optimization**: Optimize stat access patterns

### **3. Damage Calculation Bridge**
- **Stats Formatting**: Format derived stats cho damage calculation
- **Damage Input Building**: Build formatted damage input
- **Damage Output Processing**: Process damage results
- **Performance Optimization**: Optimize damage calculation path

### **4. Performance Optimization**
- **Hot Path Optimization**: Critical path cho action execution
- **Multi-level Caching**: L1, L2, L3 cache system
- **Batch Processing**: Process multiple actions cùng lúc
- **Memory Pool**: Reuse objects để giảm allocation

## 📊 **Performance Goals**

### **Target Metrics**
- **Action Execution**: < 0.1ms per action
- **Stats Access**: < 0.01ms per stat lookup
- **Damage Calculation**: < 0.05ms per calculation
- **Cache Hit Rate**: > 95%
- **Memory Usage**: < 50MB for 1000 actors
- **Throughput**: > 10,000 actions/second

### **Optimization Strategies**
1. **Pre-calculation**: Calculate stats trước khi cần
2. **Multi-level Caching**: L1 (hot), L2 (warm), L3 (cold) cache
3. **Batch Processing**: Process multiple actions together
4. **Memory Pool**: Reuse objects
5. **Hot Path**: Optimize critical execution path

## 🔗 **Integration Points**

### **Element Core Integration**
- **Stats Provider**: Access derived stats từ Element Core
- **Stats Calculator**: Calculate complex stat interactions
- **Stats Validator**: Validate stat consistency
- **Stats Cache**: Cache stats với smart invalidation

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

## 🧪 **Testing Strategy**

### **Unit Tests**
- Action execution tests
- Stats integration tests
- Damage calculation bridge tests
- Performance optimization tests

### **Integration Tests**
- Element Core integration tests
- Combat Core integration tests
- Actor Core integration tests
- End-to-end action flow tests

### **Performance Tests**
- Action execution benchmarks
- Stats access benchmarks
- Damage calculation benchmarks
- Memory usage tests
- Throughput tests

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

## 🔧 **Configuration**

### **Action Engine Config**
```yaml
action_engine:
  enabled: true
  batch_size: 100
  max_concurrent: 1000
  cache_ttl: 300000
  performance_monitoring: true
```

### **Stats Integration Config**
```yaml
stats_integration:
  enabled: true
  cache_levels:
    l1_size: 1000
    l2_size: 5000
    l3_size: 20000
  cache_ttl: 300000
  batch_processing: true
```

### **Damage Bridge Config**
```yaml
damage_bridge:
  enabled: true
  hot_path_cache: true
  batch_processing: true
  performance_monitoring: true
```

## 🎯 **Next Steps**

1. **Complete Core Documents** - Finish remaining design documents
2. **Create Configuration Files** - Create YAML config files
3. **Start Implementation** - Begin Phase 1 implementation
4. **Integration Testing** - Test integration với other cores
5. **Performance Optimization** - Optimize performance
6. **Documentation** - Complete documentation

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
