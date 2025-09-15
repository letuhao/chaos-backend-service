# Element Core Documentation

## 📋 **Tổng Quan**

Element Core là hệ thống trung tâm quản lý tất cả các loại element trong game, từ combat damage/defense đến shield, race talent, item attributes và nhiều hơn nữa. Hệ thống được thiết kế để:

- **Centralized Management**: Quản lý tập trung tất cả element types
- **Maximum Flexibility**: Dễ dàng thêm element types mới
- **Multi-System Support**: Hỗ trợ nhiều leveling systems khác nhau
- **Unified Mechanics**: Cơ chế thống nhất cho tất cả derived stats

## 📚 **Danh Sách Tài Liệu**

### **1. [00_Element_Core_Overview.md](./00_Element_Core_Overview.md)**
- **Mục đích**: Tổng quan hệ thống Element Core
- **Nội dung**: 
  - Kiến trúc hệ thống
  - Element types và categories
  - Derived stats system
  - Multi-system integration
  - Performance considerations
  - Testing strategy

### **2. [01_Probability_Mechanics_Design.md](./01_Probability_Mechanics_Design.md)**
- **Mục đích**: Thiết kế chi tiết probability mechanics
- **Nội dung**:
  - Sigmoid functions cho smooth probability curves
  - Critical hit mechanics
  - Accuracy mechanics
  - Defense mechanics
  - Element interaction mechanics
  - Advanced probability features
  - Testing và validation

### **3. [02_Multi_System_Integration_Design.md](./02_Multi_System_Integration_Design.md)**
- **Mục đích**: Thiết kế tích hợp multi-system
- **Nội dung**:
  - System interface design
  - Combat Core integration
  - Shield System integration
  - Race Talent integration
  - Item Attribute integration
  - Custom system integration
  - Multi-system aggregation
  - Conflict resolution

## 🎯 **Hướng Dẫn Đọc**

### **Cho Developers**
1. Bắt đầu với [00_Element_Core_Overview.md](./00_Element_Core_Overview.md) để hiểu tổng quan
2. Đọc [01_Probability_Mechanics_Design.md](./01_Probability_Mechanics_Design.md) để hiểu probability system
3. Đọc [02_Multi_System_Integration_Design.md](./02_Multi_System_Integration_Design.md) để hiểu integration
4. Tham khảo code examples và configuration examples

### **Cho System Architects**
1. [00_Element_Core_Overview.md](./00_Element_Core_Overview.md) - Kiến trúc tổng thể
2. [02_Multi_System_Integration_Design.md](./02_Multi_System_Integration_Design.md) - Integration patterns
3. [01_Probability_Mechanics_Design.md](./01_Probability_Mechanics_Design.md) - Advanced mechanics

### **Cho Game Designers**
1. [00_Element_Core_Overview.md](./00_Element_Core_Overview.md) - Game mechanics overview
2. [01_Probability_Mechanics_Design.md](./01_Probability_Mechanics_Design.md) - Probability system details
3. Element interaction examples và configuration

## 🚀 **Key Features**

### **Centralized Element Management**
- **Unified Registry**: Tất cả element types trong một registry
- **Flexible Configuration**: YAML-based configuration system
- **Easy Extension**: Dễ dàng thêm element types mới
- **Type Safety**: Strong typing với Rust enums

### **Advanced Probability System**
- **Smooth Curves**: Sigmoid functions cho natural probability
- **Element-Specific Scaling**: Mỗi element có scaling factors khác nhau
- **Balanced Mechanics**: 100% chance khi attacker >> defender, 0% khi ngược lại
- **Configurable Parameters**: Dễ dàng balance và fine-tune

### **Multi-System Integration**
- **Loose Coupling**: Systems không phụ thuộc trực tiếp
- **Consistent Interface**: Unified interface cho tất cả systems
- **Conflict Resolution**: Advanced conflict handling
- **Event System**: Real-time event handling

### **Performance Optimized**
- **Efficient Calculations**: Optimized probability calculations
- **Caching System**: Multi-layer caching
- **Batch Processing**: Process multiple elements
- **Memory Management**: Efficient memory usage

## 📊 **Element Types & Categories**

### **Basic Categories**
- **Physical**: Physical damage/defense
- **Magical**: Magical damage/defense
- **Natural Elements**: Fire, Water, Ice, Earth, Air, Lightning, Poison, Dark, Light
- **Cultivation Elements**: Qi, Dao, Profound, Karma, Fate
- **Special Elements**: True, Healing, Drain, Reflect, Absorb, Chaos, Reality, Conceptual

### **Derived Stats**
- **Power & Defense**: PowerPoint, DefensePoint
- **Critical Stats**: CritRate, CritDamage, ResistCritRate, ResistCritDamage
- **Accuracy Stats**: AccurateRate, DodgeRate
- **Custom Stats**: Penetration, Absorption, Reflection, Conversion, Amplification, Reduction

## 🔢 **Probability Mechanics**

### **Core Formulas**
```rust
// Base calculation: (attacker_stat - defender_stat)
let difference = attacker_stat - defender_stat;

// Probability calculation using sigmoid
let probability = sigmoid(difference / scaling_factor);

// Sigmoid function
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
```

### **Element-Specific Scaling**
- **Fire**: Higher crit scaling, moderate accuracy
- **Water**: Higher accuracy scaling, better defense
- **Ice**: Higher accuracy, better defense scaling
- **Lightning**: Highest crit scaling, lower accuracy
- **Earth**: Best defense scaling, lower crit scaling

## 🔄 **Multi-System Support**

### **Supported Systems**
- **Combat Core**: Damage calculation và combat mechanics
- **Shield System**: Shield absorption và reflection
- **Race Talents**: Race-based element bonuses
- **Item Attributes**: Item-based element stats
- **Custom Systems**: Extensible cho custom systems

### **Integration Pattern**
```rust
// System interface
pub trait ElementSystemInterface {
    fn get_system_id(&self) -> &str;
    fn get_supported_elements(&self) -> Vec<String>;
    fn get_actor_element_stats(&self, actor_id: &str, element_type: &str) -> Result<HashMap<DerivedStatType, f64>, ElementError>;
    // ... more methods
}
```

## 🧪 **Testing Strategy**

### **Test Types**
- **Unit Tests**: Individual component testing
- **Integration Tests**: Multi-system testing
- **Performance Tests**: Load và stress testing
- **Probability Tests**: Validate probability calculations

### **Test Data**
- **Element Definitions**: Test element types
- **Probability Vectors**: Test probability calculations
- **Multi-System Scenarios**: Test integration scenarios
- **Performance Benchmarks**: Test performance

## 🔧 **Configuration Examples**

### **Element Types**
```yaml
# element_types.yaml
elements:
  - id: "fire"
    name: "Fire"
    category: "fire"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "crit_rate"
      - "crit_damage"
    is_active: true
```

### **System Configuration**
```yaml
# system_configs.yaml
systems:
  - system_id: "combat_core"
    elements:
      - element_type: "fire"
        primary_stats:
          - "power_point"
          - "defense_point"
        power_scale: 1.0
        is_enabled: true
```

### **Probability Configuration**
```yaml
# probability_config.yaml
scaling_factors:
  crit_rate:
    fire: 120.0
    water: 100.0
    ice: 110.0
  accuracy:
    fire: 100.0
    water: 120.0
    ice: 110.0
```

## 🚀 **Implementation Roadmap**

### **Phase 1: Core Element System** (Weeks 1-2)
- [ ] Element Registry implementation
- [ ] Basic derived stats system
- [ ] Core calculation engine
- [ ] YAML configuration system

### **Phase 2: Probability Mechanics** (Weeks 3-4)
- [ ] Sigmoid function implementation
- [ ] Element-specific scaling
- [ ] Critical hit mechanics
- [ ] Accuracy mechanics

### **Phase 3: Multi-System Integration** (Weeks 5-6)
- [ ] System interface implementation
- [ ] Combat Core integration
- [ ] Shield System integration
- [ ] Race Talent integration

### **Phase 4: Advanced Features** (Weeks 7-8)
- [ ] Item Attribute integration
- [ ] Custom system support
- [ ] Conflict resolution
- [ ] Event system

### **Phase 5: Optimization & Testing** (Weeks 9-10)
- [ ] Performance optimization
- [ ] Comprehensive testing
- [ ] Documentation completion
- [ ] Production readiness

## ❓ **Questions for Discussion**

1. **Element Interactions**: Có nên có element rock-paper-scissors system?
2. **Stat Scaling**: Làm thế nào để scale stats theo level?
3. **Custom Stats**: Có nên cho phép custom derived stats?
4. **Performance**: Làm thế nào để optimize cho nhiều elements?
5. **Configuration**: Có nên có runtime configuration changes?
6. **Multi-Element**: Có nên hỗ trợ multiple elements per actor?
7. **Element Evolution**: Có nên có element evolution/upgrade system?

## 🎯 **Next Steps**

1. **Review Design**: Review và feedback trên design documents
2. **Implement Core**: Bắt đầu implement core element system
3. **Create Tests**: Tạo comprehensive test suite
4. **Integration**: Integrate với existing systems
5. **Performance**: Optimize và fine-tune performance

## 📞 **Support & Contributing**

### **Getting Help**
- **Documentation**: Check this directory first
- **Issues**: Use GitHub issues for bugs
- **Discussions**: Use GitHub discussions for questions
- **Code Review**: Follow the review process

### **Contributing**
- **Fork Repository**: Create your own fork
- **Create Branch**: Use feature branches
- **Write Tests**: Include comprehensive tests
- **Update Docs**: Keep documentation current
- **Submit PR**: Follow the PR template

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
