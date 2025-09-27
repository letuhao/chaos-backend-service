# Element-Core Terminology Glossary

## 📋 **Overview**

This document defines the standard terminology used across all Element-Core documentation. All documents should use these terms consistently to avoid confusion and ensure clear communication.

**Version**: 1.0  
**Last Updated**: 2024-12-19  
**Status**: Active

---

## 🎯 **Core Terms**

### **Element vs Elemental**

#### **Element** (Nguyên tố / 元素)
- **Definition**: Individual element type (Fire, Water, Earth, Air, etc.)
- **Usage**: "Fire element", "Water element", "Element definition"
- **Examples**: 
  - `ElementDefinition` struct
  - `get_element("fire")` function
  - `element_type: String` field

#### **Elemental** (Thuộc về nguyên tố / 元素的)
- **Definition**: Adjective describing something related to elements
- **Usage**: "Elemental system", "Elemental mastery", "Elemental interaction"
- **Examples**:
  - `ElementalSystem` struct
  - `ElementalMastery` trait
  - `ElementalInteraction` enum

### **Registry vs System**

#### **Registry** (Sổ đăng ký / 注册表)
- **Definition**: Central data storage and management component
- **Usage**: "Element registry", "Registry implementation", "Registry operations"
- **Examples**:
  - `ElementRegistry` struct
  - `register_element()` function
  - `registry.get_element()` method

#### **System** (Hệ thống / 系统)
- **Definition**: Complete functional unit with multiple components
- **Usage**: "Element system", "System architecture", "System integration"
- **Examples**:
  - `ElementSystem` struct
  - `SystemIntegration` trait
  - `system.initialize()` method

### **Integration vs Contribution**

#### **Integration** (Tích hợp / 集成)
- **Definition**: Process of connecting systems together
- **Usage**: "System integration", "Integration pattern", "Integration layer"
- **Examples**:
  - `SystemIntegration` trait
  - `integration.connect()` method
  - `IntegrationPattern` enum

#### **Contribution** (Đóng góp / 贡献)
- **Definition**: Data or functionality provided by one system to another
- **Usage**: "Element contribution", "Contribution data", "Contribute stats"
- **Examples**:
  - `ElementContribution` struct
  - `contribute_stats()` function
  - `contribution.value` field

### **Stats vs Attributes**

#### **Stats** (Chỉ số / 属性)
- **Definition**: Numerical values that can be calculated and modified
- **Usage**: "Element stats", "Power stats", "Defense stats"
- **Examples**:
  - `fire_power_point: f64`
  - `water_defense_point: f64`
  - `earth_mastery_level: f64`

#### **Attributes** (Thuộc tính / 属性)
- **Definition**: Qualitative properties or characteristics
- **Usage**: "Element attributes", "Racial attributes", "Attribute modifiers"
- **Examples**:
  - `affinity: AffinityType`
  - `resistance: ResistanceLevel`
  - `category: ElementCategory`

---

## 🏗️ **Architecture Terms**

### **Data Hub** (Trung tâm dữ liệu / 数据中心)
- **Definition**: Central component that aggregates and caches data from multiple sources
- **Usage**: "Element-Core as data hub", "Data hub pattern", "Hub architecture"
- **Examples**:
  - `ElementCore` acts as data hub
  - `DataHub` trait
  - `hub.aggregate()` method

### **External Contributor** (Đóng góp bên ngoài / 外部贡献者)
- **Definition**: System that provides data to Element-Core without being part of it
- **Usage**: "External contributor pattern", "Contributor system", "External integration"
- **Examples**:
  - `ElementContributor` trait
  - `external_contributor.contribute()` method
  - `ContributorRegistry` struct

### **Aggregation** (Tổng hợp / 聚合)
- **Definition**: Process of combining data from multiple sources
- **Usage**: "Stat aggregation", "Aggregation rules", "Aggregate contributions"
- **Examples**:
  - `ElementAggregator` struct
  - `aggregate_stats()` function
  - `AggregationRule` enum

### **Caching** (Bộ nhớ đệm / 缓存)
- **Definition**: Storing computed results for faster access
- **Usage**: "Element cache", "Cache strategy", "Cache invalidation"
- **Examples**:
  - `ElementCache` struct
  - `cache.get()` method
  - `CacheStrategy` enum

---

## 🔧 **Implementation Terms**

### **Plugin** (Phần mở rộng / 插件)
- **Definition**: Extensible component that adds functionality
- **Usage**: "Element plugin", "Plugin system", "Plugin architecture"
- **Examples**:
  - `ElementPlugin` trait
  - `plugin.initialize()` method
  - `PluginRegistry` struct

### **Configuration** (Cấu hình / 配置)
- **Definition**: Settings and parameters that control behavior
- **Usage**: "Element configuration", "Config file", "Configuration management"
- **Examples**:
  - `ElementConfig` struct
  - `config.yaml` file
  - `ConfigurationManager` struct

### **Validation** (Xác thực / 验证)
- **Definition**: Process of checking data correctness
- **Usage**: "Element validation", "Validate input", "Validation rules"
- **Examples**:
  - `ElementValidator` trait
  - `validate_element()` function
  - `ValidationError` enum

### **Error Handling** (Xử lý lỗi / 错误处理)
- **Definition**: Managing and reporting errors
- **Usage**: "Error handling", "Handle errors", "Error recovery"
- **Examples**:
  - `ElementCoreError` enum
  - `handle_error()` function
  - `ErrorRecovery` trait

---

## 🎮 **Game-Specific Terms**

### **Mastery** (Thành thạo / 精通)
- **Definition**: Level of proficiency with an element
- **Usage**: "Element mastery", "Mastery level", "Mastery system"
- **Examples**:
  - `fire_mastery_level: f64`
  - `MasterySystem` struct
  - `mastery.calculate_bonus()` method

### **Affinity** (Thiên phú / 亲和)
- **Definition**: Natural connection or compatibility with an element
- **Usage**: "Element affinity", "Affinity level", "Affinity bonus"
- **Examples**:
  - `fire_affinity: f64`
  - `AffinityType` enum
  - `affinity.get_multiplier()` method

### **Resistance** (Kháng cự / 抗性)
- **Definition**: Ability to resist or reduce element effects
- **Usage**: "Element resistance", "Resistance value", "Resistance calculation"
- **Examples**:
  - `fire_resistance: f64`
  - `ResistanceLevel` enum
  - `resistance.calculate_reduction()` method

### **Interaction** (Tương tác / 交互)
- **Definition**: How elements affect each other
- **Usage**: "Element interaction", "Interaction matrix", "Interaction factor"
- **Examples**:
  - `ElementInteraction` struct
  - `interaction.calculate_damage()` method
  - `InteractionMatrix` struct

---

## 📚 **Documentation Terms**

### **Design Document** (Tài liệu thiết kế / 设计文档)
- **Definition**: Document describing system design and architecture
- **Usage**: "Design document", "Architecture design", "System design"
- **Examples**:
  - `Element_System_Architecture.md`
  - `Registry_Design.md`
  - `Integration_Design.md`

### **Implementation Guide** (Hướng dẫn triển khai / 实施指南)
- **Definition**: Document explaining how to implement features
- **Usage**: "Implementation guide", "Implementation steps", "Implementation examples"
- **Examples**:
  - `Implementation_Guide.md`
  - `Code_Examples.md`
  - `Best_Practices.md`

### **Migration Guide** (Hướng dẫn di chuyển / 迁移指南)
- **Definition**: Document explaining how to migrate from old to new versions
- **Usage**: "Migration guide", "Migration steps", "Migration examples"
- **Examples**:
  - `Migration_Guide.md`
  - `Version_Migration.md`
  - `Breaking_Changes.md`

---

## 🔗 **Cross-Reference Terms**

### **Related Document** (Tài liệu liên quan / 相关文档)
- **Definition**: Document that provides additional information
- **Usage**: "Related documents", "See also", "Additional reading"
- **Examples**:
  - `[Element System Architecture](01_Element_System_Architecture.md)`
  - `[Element Registry Design](04_Element_Registry_Design.md)`
  - `[Integration Patterns](02_Multi_System_Integration_Design.md)`

### **Reference** (Tham chiếu / 参考)
- **Definition**: Link to specific section or information
- **Usage**: "See reference", "Reference implementation", "Reference documentation"
- **Examples**:
  - `[Element Definition](../elements/configs/fire_element.yaml)`
  - `[Code Examples](../examples/element_usage.rs)`

---

## ⚠️ **Deprecated Terms**

### **Terms to Avoid**
- **"Elemental Registry"** → Use **"Element Registry"**
- **"Element System"** → Use **"Elemental System"** (when referring to the complete system)
- **"Element Integration"** → Use **"System Integration"**
- **"Element Contribution"** → Use **"Element Contribution"** (correct)
- **"Element Stats"** → Use **"Element Stats"** (correct)
- **"Element Attributes"** → Use **"Element Attributes"** (correct)

### **Common Mistakes**
- ❌ "Elemental Registry" (should be "Element Registry")
- ❌ "Element System" (should be "Elemental System" for complete system)
- ❌ "Element Integration" (should be "System Integration")
- ❌ "Elemental Stats" (should be "Element Stats")
- ❌ "Elemental Attributes" (should be "Element Attributes")

---

## 📝 **Usage Guidelines**

### **1. Consistency Rules**
- Always use the same term for the same concept
- Use "Element" for individual elements, "Elemental" for systems
- Use "Registry" for data storage, "System" for complete functionality
- Use "Integration" for connections, "Contribution" for data provided

### **2. Context Matters**
- Consider the context when choosing between similar terms
- Use specific terms when precision is important
- Use general terms when discussing concepts broadly

### **3. Documentation Standards**
- Include Vietnamese and Chinese translations for key terms
- Use consistent formatting for code examples
- Provide clear examples for each term
- Update this glossary when new terms are introduced

---

## 🔄 **Maintenance**

### **Version Control**
- Update version number when making changes
- Document changes in the change log
- Review and update terms regularly

### **Review Process**
- Review terminology during document updates
- Validate consistency across all documents
- Get feedback from developers and stakeholders

### **Expansion**
- Add new terms as the system evolves
- Remove deprecated terms
- Update examples and usage guidelines

---

**Last Updated**: 2024-12-19  
**Version**: 1.0  
**Status**: Active  
**Next Review**: 2024-12-26
