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

### **4. [06_Implementation_Notes.md](./06_Implementation_Notes.md)** ⚠️ **CRITICAL**
- **Mục đích**: Guidelines và requirements quan trọng cho implementation
- **Nội dung**:
  - Omni additive-only rule enforcement
  - Status hit dependency requirements
  - Element-specific sigmoid parameters
  - Damage composition law
  - Clamping & validation requirements
  - Testing requirements
  - Critical implementation notes

### **5. [07_Resource_Manager_Integration_Design.md](./07_Resource_Manager_Integration_Design.md)**
- **Mục đích**: Tích hợp với Resource Manager systems
- **Nội dung**:
  - Primary stats to derived stats mapping
  - Event-driven stats change propagation
  - Multi-system stats aggregation
  - RPG Resource Manager integration
  - Magic Resource Manager integration
  - Stats change event system
  - Performance optimization

### **6. [08_Elemental_Mastery_System_Design.md](./08_Elemental_Mastery_System_Design.md)**
- **Mục đích**: Thiết kế hệ thống Elemental Mastery (cultivation system)
- **Nội dung**:
  - Plugin-based architecture cho element mastery
  - Decay system và training mechanics
  - Element interactions và tương sinh tương khắc
  - Integration với Element Core và Actor Core
  - Configuration system và event handling
  - Performance optimization và testing strategy

### **7. [09_Actor_Core_Integration_Guide.md](./09_Actor_Core_Integration_Guide.md)**
- **Mục đích**: Hướng dẫn tích hợp Elemental Mastery System vào Actor Core
- **Nội dung**:
  - Step-by-step integration guide
  - Actor Core framework pattern implementation
  - Resource management integration
  - Event system integration
  - Configuration và deployment guide
  - Testing và debugging strategies

### **8. [10_Element_Interaction_System_Design.md](./10_Element_Interaction_System_Design.md)**
- **Mục đích**: Thiết kế hệ thống tương sinh tương khắc với Elemental Mastery integration
- **Nội dung**:
  - Tương sinh tương khắc concept và strategic depth
  - Bảng overview cho các trường hợp cụ thể (cùng hệ, tương khắc, tương sinh, trung tính)
  - Mastery-based trigger calculation với công thức chi tiết
  - Buff/debuff effects system
  - Integration với Elemental Mastery System
  - Combat integration và event system
  - Configuration examples và implementation priority

### **9. [11_Advanced_Derived_Stats_Design.md](./11_Advanced_Derived_Stats_Design.md)**
- **Mục đích**: Các derived stats nâng cao cho Element Core
- **Nội dung**:
  - Skill execution speed dựa trên element mastery
  - Mastery bonuses (experience gain, decay resistance, training speed)
  - Advanced combat mechanics (penetration, absorption, reflection)
  - Resource management (mana/stamina/health regeneration)
  - Implementation strategy với 4 phases
  - Stat weights và priorities
  - Game impact và meta game considerations

### **10. [12_Performance_Optimization_Design.md](./12_Performance_Optimization_Design.md)**
- **Mục đích**: Tối ưu performance cho Element Core
- **Nội dung**:
  - Caching strategy với multi-level cache
  - Memory management và efficient storage
  - Calculation optimization với batch processing
  - Concurrency handling với thread-safe operations
  - Performance metrics và monitoring
  - Configuration tuning và deployment considerations

### **11. [13_Error_Handling_Logging_Design.md](./13_Error_Handling_Logging_Design.md)**
- **Mục đích**: Error handling và logging strategy cho Element Core
- **Nội dung**:
  - Error categories và recovery strategies
  - Graceful degradation và circuit breaker pattern
  - Structured logging với context tracking
  - Performance logging và memory monitoring
  - Debugging tools và calculation visualizer
  - Error reporting và user-friendly messages

### **12. [14_Reuse_Analysis_Actor_Core_Resource_Manager.md](./14_Reuse_Analysis_Actor_Core_Resource_Manager.md)**
- **Mục đích**: Phân tích khả năng tái sử dụng Actor Core và Resource Manager
- **Nội dung**:
  - Mapping Element Core features với existing systems
  - Tận dụng SystemResourceCalculator trait
  - Tận dụng string-based formula evaluation
  - Extensions cần thiết cho Actor Core và Resource Manager
  - Implementation strategy với 4 phases
  - Lợi ích tái sử dụng và development speed

### **13. [16_Hybrid_Subsystem_Design.md](./16_Hybrid_Subsystem_Design.md)**
- **Mục đích**: Thiết kế Hybrid Subsystem cho các nguyên tố lai
- **Nội dung**:
  - Consistency với Actor Core ModifierPack system
  - Configuration-driven tag detection và modifier rules
  - Hybrid Element structure với conditional activation
  - Extended ModifierPack với conditional và tag-based modifiers
  - Tag detection system với flexible operators
  - YAML configuration examples và implementation details
  - Integration với SystemResourceCalculator và Status Pool

## 🔗 **System Consistency**
- Công thức xác suất/steepness/scaling: tham chiếu duy nhất `01_Probability_Mechanics_Design.md`.
- Caps/cờ tính năng: tham chiếu `11_Advanced_Derived_Stats_Design.md`.
- Engine IDs dùng English snake_case; alias dùng cho hiển thị (xem `05_Element_Summary_Comprehensive.md`).


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
    category: "five_elements"
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

## 📈 **Telemetry & Tests Checklist**

- Probability engine: log (Δ, p) distributions; ensure p ∈ [0,1].
- Dynamics: log (I, Δ, R) over time; detect oscillation/runaway; verify damping.
- Golden vectors: validate interactions and probability ranges (`elements/golden_vectors`).
- Config references:
  - Probability: `docs/element-core/configs/probability_config.yaml`
  - Interactions: `docs/element-core/configs/interaction_config.yaml`
  - Element example: `docs/element-core/elements/configs/fire_element.yaml`
  - Five Elements Overview: `docs/element-core/elements/overview/five_elements_overview.md`
  - Extended Elements Overview: `docs/element-core/elements/overview/five_elements_extensions_overview.md`
  - Hybrid Subsystem: `docs/element-core/hybrid/README.md`

## ✅ Element Config Validation Checklist

Use this list when adding or reviewing an element config:

- IDs & Aliases
  - [ ] `element.id` matches engine IDs (english snake_case)
  - [ ] `aliases.vi` and `aliases.zh_pinyin` present if needed
- References
  - [ ] `probability_config_path` points to `configs/probability_config.yaml`
  - [ ] `interaction_config_path` points to `configs/interaction_config.yaml`
  - [ ] `status_pool_path` points to `configs/status_pool.yaml`
  - [ ] `golden_vectors_path` exists if vectors are provided
- Status Effects
  - [ ] No hard caps (`max_*`); use `dynamics` (gain, damping, decay, refractory)
  - [ ] Optional `spread_rules` documented if spread is supported
  - [ ] Probability fields align with Probability Mechanics doc
- Interactions
  - [ ] `same_element_effects` defined in element (from status pool)
  - [ ] `neutral_effects` defined in element (when not in pairs)
  - [ ] Cross-element effects live in `configs/interaction_config.yaml` and reference pool by `pool_id`
- Testing & Telemetry
  - [ ] Golden vectors updated/added under `elements/golden_vectors`
  - [ ] Engine logs `(Δ, I, R, p)` for tuning per Implementation Notes

## 🗂️ Where to put what

Use this map to avoid duplication and keep responsibilities clear:

- configs/status_pool.yaml
  - Reusable, multi-element effect templates (e.g., `heat_resonance`, `ember_shield`, `burning_fallback`).
  - Contains effect dynamics and stat hooks that many elements can reference by `pool_id`.

- configs/interaction_config.yaml
  - Relationships and pairs (same/generating/overcoming/neutral) across elements.
  - Cross-element effects (Fire→Metal, Water→Fire, etc.) that reference `status_pool.yaml` via `pool_id`.
  - Do not place element-owned same-element or neutral-by-element rules here.

- elements/configs/<element>.yaml (e.g., Fire)
  - Element-owned statuses (e.g., `burning`, `fire_regeneration`) with dynamics.
  - same_element_effects (e.g., Fire↔Fire) and neutral_effects (e.g., Fire↔Neutral) referencing the pool.
  - Optional environment modifiers and spread rules.
  - References to central configs and golden vectors.

- elements/<element>.md (e.g., `elements/fire_element.md`)
  - Human-readable spec: intent, mechanics, and YAML excerpts for same-element and neutral sections.

- elements/elemental_interactions.md
  - Aggregate overview for cross-element interactions; process and examples only (no duplication of numbers).

- configs/probability_config.yaml
  - Central scaling/steepness for the probability engine; do not duplicate per element.
