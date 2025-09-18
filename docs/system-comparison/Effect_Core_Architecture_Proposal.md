# Effect-Core Architecture Proposal

## 📋 **Tổng Quan**

Tài liệu này đề xuất kiến trúc Effect-Core để giải quyết vấn đề effect duplication và confusion giữa Action-Core, Status-Core, và Element-Core.

## 🎯 **Vấn Đề Hiện Tại**

### **1. Effect Duplication**

```
Current Problem: Effects Scattered Across Systems
├── Action-Core
│   ├── Fireball Damage Effect
│   ├── Healing Potion Effect
│   └── Speed Boost Effect
├── Status-Core
│   ├── Burning Status Effect
│   ├── Regeneration Effect
│   └── Slow Movement Effect
└── Element-Core
    ├── Fire Mastery Effect
    ├── Ice Resistance Effect
    └── Lightning Interaction Effect
```

**Vấn đề:**
- Cùng một effect được định nghĩa ở nhiều nơi
- Interface không nhất quán
- Khó maintain và extend
- Performance không tối ưu

### **2. Future Complexity**

```
Future Systems Will Add More Effects
├── Talent-Core
│   ├── Passive Effects
│   ├── Active Effects
│   └── Conditional Effects
├── Perk-Core
│   ├── Skill Effects
│   ├── Mastery Effects
│   └── Special Effects
└── Skill-Core
    ├── Learning Effects
    ├── Practice Effects
    └── Mastery Effects
```

## 🏗️ **Proposed Effect-Core Architecture**

### **1. Core Components**

```
Effect-Core
├── Effect Registry
│   ├── Effect Type Definitions
│   ├── Effect Category Management
│   ├── Effect Validation Rules
│   └── Effect Dependency Tracking
├── Effect Engine
│   ├── Effect Calculator
│   ├── Effect Processor
│   ├── Effect Scheduler
│   └── Effect Validator
├── Effect Interfaces
│   ├── Action Effect Interface
│   ├── Status Effect Interface
│   ├── Element Effect Interface
│   ├── Talent Effect Interface
│   └── Perk Effect Interface
└── Effect Integration
    ├── Action-Core Bridge
    ├── Status-Core Bridge
    ├── Element-Core Bridge
    ├── Talent-Core Bridge
    └── Perk-Core Bridge
```

### **2. Effect Type Hierarchy**

```
Effect Types
├── Damage Effects
│   ├── Physical Damage
│   ├── Elemental Damage
│   ├── Magical Damage
│   └── True Damage
├── Healing Effects
│   ├── Health Healing
│   ├── Stamina Healing
│   ├── Mana Healing
│   └── Lifespan Healing
├── Buff Effects
│   ├── Stat Buffs
│   ├── Speed Buffs
│   ├── Defense Buffs
│   └── Special Buffs
├── Debuff Effects
│   ├── Stat Debuffs
│   ├── Speed Debuffs
│   ├── Defense Debuffs
│   └── Special Debuffs
├── Status Effects
│   ├── Burning Status
│   ├── Freezing Status
│   ├── Stunned Status
│   └── Charmed Status
├── Movement Effects
│   ├── Speed Boost
│   ├── Jump Boost
│   ├── Flight Effect
│   └── Teleport Effect
└── Environmental Effects
    ├── Weather Effects
    ├── Terrain Effects
    ├── Time Effects
    └── Location Effects
```

### **3. Effect Processing Pipeline**

```
Effect Processing Pipeline
├── Effect Request
│   ├── Source System (Action/Status/Element/Talent)
│   ├── Effect Type
│   ├── Target Information
│   └── Context Data
├── Effect Validation
│   ├── Effect Existence Check
│   ├── Target Validation
│   ├── Permission Check
│   └── Resource Check
├── Effect Calculation
│   ├── Base Effect Value
│   ├── Modifier Application
│   ├── Scaling Calculation
│   └── Final Effect Value
├── Effect Application
│   ├── Target System Notification
│   ├── Effect State Update
│   ├── Visual/Audio Effects
│   └── Event Dispatch
└── Effect Monitoring
    ├── Duration Tracking
    ├── Effect Stacking
    ├── Effect Interactions
    └── Effect Cleanup
```

## 🔧 **Implementation Strategy**

### **Phase 1: Foundation (Weeks 1-2)**

```rust
// Effect-Core Foundation
pub struct EffectCore {
    effect_registry: EffectRegistry,
    effect_engine: EffectEngine,
    effect_interfaces: EffectInterfaces,
    effect_integration: EffectIntegration,
}

pub struct EffectRegistry {
    effect_types: HashMap<String, EffectType>,
    effect_categories: HashMap<String, EffectCategory>,
    effect_validation_rules: Vec<ValidationRule>,
}

pub struct EffectEngine {
    calculator: EffectCalculator,
    processor: EffectProcessor,
    scheduler: EffectScheduler,
    validator: EffectValidator,
}
```

### **Phase 2: Core Interfaces (Weeks 3-4)**

```rust
// Effect Interfaces
pub trait EffectInterface {
    async fn apply_effect(&self, effect: &Effect, target: &Target) -> Result<EffectResult, EffectError>;
    async fn remove_effect(&self, effect_id: &str, target: &Target) -> Result<(), EffectError>;
    async fn modify_effect(&self, effect_id: &str, modification: &EffectModification) -> Result<(), EffectError>;
}

pub struct ActionEffectInterface {
    effect_core: Arc<EffectCore>,
    action_core_client: ActionCoreClient,
}

pub struct StatusEffectInterface {
    effect_core: Arc<EffectCore>,
    status_core_client: StatusCoreClient,
}

pub struct ElementEffectInterface {
    effect_core: Arc<EffectCore>,
    element_core_client: ElementCoreClient,
}
```

### **Phase 3: Integration (Weeks 5-6)**

```rust
// Effect Integration Bridges
pub struct ActionCoreBridge {
    effect_core: Arc<EffectCore>,
    action_effect_interface: ActionEffectInterface,
}

impl ActionCoreBridge {
    pub async fn process_action_effects(
        &self,
        action: &Action,
        target: &Target
    ) -> Result<Vec<EffectResult>, EffectError> {
        let effects = self.extract_effects_from_action(action);
        let mut results = Vec::new();
        
        for effect in effects {
            let result = self.effect_core.apply_effect(&effect, target).await?;
            results.push(result);
        }
        
        Ok(results)
    }
}
```

### **Phase 4: Migration (Weeks 7-8)**

```yaml
# Migration Strategy
migration_plan:
  phase_1_foundation:
    - Create Effect-Core system
    - Implement basic effect types
    - Create effect registry
    - Add effect validation
  
  phase_2_interfaces:
    - Create effect interfaces
    - Implement action effect interface
    - Implement status effect interface
    - Implement element effect interface
  
  phase_3_integration:
    - Create integration bridges
    - Update Action-Core to use Effect-Core
    - Update Status-Core to use Effect-Core
    - Update Element-Core to use Effect-Core
  
  phase_4_migration:
    - Migrate existing effects
    - Update configuration files
    - Test and validate
    - Performance optimization
```

## 📁 **Configuration File Organization**

### **1. Effect-Core Config Structure**

```
effect-core/
├── core/
│   ├── effect_types.yaml
│   ├── effect_categories.yaml
│   ├── effect_validation_rules.yaml
│   └── effect_processing_rules.yaml
├── interfaces/
│   ├── action_effect_interface.yaml
│   ├── status_effect_interface.yaml
│   ├── element_effect_interface.yaml
│   └── talent_effect_interface.yaml
├── integrations/
│   ├── action_core_bridge.yaml
│   ├── status_core_bridge.yaml
│   ├── element_core_bridge.yaml
│   └── talent_core_bridge.yaml
└── effects/
    ├── damage_effects/
    ├── healing_effects/
    ├── buff_effects/
    ├── debuff_effects/
    ├── status_effects/
    ├── movement_effects/
    └── environmental_effects/
```

### **2. Effect Type Configuration**

```yaml
# effect-core/core/effect_types.yaml
effect_types:
  damage:
    type_id: "damage"
    type_name: "Damage"
    type_name_vi: "Sát Thương"
    category: "combat"
    base_interface: "damage_effect_interface"
    validation_rules:
      - "target_must_be_valid"
      - "damage_value_must_be_positive"
      - "damage_type_must_be_valid"
    processing_rules:
      - "calculate_base_damage"
      - "apply_damage_modifiers"
      - "apply_resistance"
      - "apply_armor"
    properties:
      damage_type: "required"
      base_damage: "required"
      damage_modifiers: "optional"
      damage_conditions: "optional"
  
  healing:
    type_id: "healing"
    type_name: "Healing"
    type_name_vi: "Hồi Máu"
    category: "support"
    base_interface: "healing_effect_interface"
    validation_rules:
      - "target_must_be_valid"
      - "healing_value_must_be_positive"
      - "healing_type_must_be_valid"
    processing_rules:
      - "calculate_base_healing"
      - "apply_healing_modifiers"
      - "apply_healing_bonuses"
      - "apply_healing_penalties"
    properties:
      healing_type: "required"
      base_healing: "required"
      healing_modifiers: "optional"
      healing_conditions: "optional"
```

### **3. Effect Interface Configuration**

```yaml
# effect-core/interfaces/action_effect_interface.yaml
action_effect_interface:
  interface_id: "action_effect_interface"
  interface_name: "Action Effect Interface"
  interface_name_vi: "Giao Diện Hiệu Ứng Hành Động"
  
  # Supported Effect Types
  supported_effect_types:
    - "damage"
    - "healing"
    - "movement"
    - "resource_consumption"
    - "timing"
  
  # Effect Processing Rules
  effect_processing_rules:
    - rule_id: "action_damage_processing"
      rule_name: "Action Damage Processing"
      rule_description: "Process damage effects from actions"
      rule_steps:
        - "validate_action_context"
        - "calculate_damage_value"
        - "apply_action_modifiers"
        - "apply_target_resistance"
        - "apply_damage_to_target"
    
    - rule_id: "action_healing_processing"
      rule_name: "Action Healing Processing"
      rule_description: "Process healing effects from actions"
      rule_steps:
        - "validate_action_context"
        - "calculate_healing_value"
        - "apply_healing_modifiers"
        - "apply_healing_bonuses"
        - "apply_healing_to_target"
  
  # Integration Points
  integration_points:
    - point_id: "action_execution"
      point_name: "Action Execution"
      point_description: "Apply effects during action execution"
      point_trigger: "action_executed"
      point_effects: ["damage", "healing", "movement"]
    
    - point_id: "action_completion"
      point_name: "Action Completion"
      point_description: "Apply effects after action completion"
      point_trigger: "action_completed"
      point_effects: ["resource_consumption", "timing"]
```

## 🚀 **Benefits of Effect-Core**

### **1. Unified Effect Management**

- **Single Source of Truth**: Tất cả effects được quản lý ở một nơi
- **Consistent Interfaces**: Interface thống nhất cho tất cả effects
- **Centralized Validation**: Validation tập trung và nhất quán
- **Unified Processing**: Xử lý effects thống nhất

### **2. Better Organization**

- **Clear Separation of Concerns**: Tách biệt rõ ràng giữa các concerns
- **Easier Maintenance**: Dễ dàng maintain và update
- **Better Performance**: Performance tối ưu với centralized processing
- **Easier Testing**: Dễ dàng test và debug

### **3. Future-Proof Design**

- **Easy Extension**: Dễ dàng thêm effect types mới
- **System Integration**: Dễ dàng tích hợp với systems mới
- **Complex Interactions**: Hỗ trợ complex effect interactions
- **Effect Combinations**: Hỗ trợ effect combinations

### **4. Configuration Management**

- **Modular Configuration**: Configuration modular và flexible
- **Hot Reload**: Hỗ trợ hot reload effects
- **Version Control**: Hỗ trợ version control
- **Conflict Resolution**: Giải quyết conflicts giữa effects

## 📝 **Migration Plan**

### **1. Pre-Migration Analysis**

```yaml
# Analyze current effect usage
analysis_plan:
  action_core_effects:
    - "List all effects in Action-Core"
    - "Identify effect types and categories"
    - "Document effect interfaces"
    - "Identify dependencies"
  
  status_core_effects:
    - "List all effects in Status-Core"
    - "Identify effect types and categories"
    - "Document effect interfaces"
    - "Identify dependencies"
  
  element_core_effects:
    - "List all effects in Element-Core"
    - "Identify effect types and categories"
    - "Document effect interfaces"
    - "Identify dependencies"
```

### **2. Migration Steps**

```yaml
# Step-by-step migration
migration_steps:
  step_1_create_effect_core:
    - "Create Effect-Core system"
    - "Implement basic effect types"
    - "Create effect registry"
    - "Add effect validation"
  
  step_2_create_interfaces:
    - "Create effect interfaces"
    - "Implement action effect interface"
    - "Implement status effect interface"
    - "Implement element effect interface"
  
  step_3_create_bridges:
    - "Create integration bridges"
    - "Update Action-Core to use Effect-Core"
    - "Update Status-Core to use Effect-Core"
    - "Update Element-Core to use Effect-Core"
  
  step_4_migrate_effects:
    - "Migrate existing effects"
    - "Update configuration files"
    - "Test and validate"
    - "Performance optimization"
```

### **3. Testing Strategy**

```yaml
# Testing plan
testing_strategy:
  unit_tests:
    - "Test effect registry"
    - "Test effect interfaces"
    - "Test effect processing"
    - "Test effect validation"
  
  integration_tests:
    - "Test Action-Core integration"
    - "Test Status-Core integration"
    - "Test Element-Core integration"
    - "Test cross-system effects"
  
  performance_tests:
    - "Test effect processing performance"
    - "Test memory usage"
    - "Test scalability"
    - "Test concurrent effects"
  
  regression_tests:
    - "Test existing functionality"
    - "Test effect interactions"
    - "Test edge cases"
    - "Test error handling"
```

## 🎯 **Recommendations**

### **1. Immediate Actions**

1. **Create Effect-Core Foundation**: Bắt đầu với basic effect types
2. **Design Effect Interfaces**: Thiết kế interfaces cho tất cả systems
3. **Plan Migration Strategy**: Lập kế hoạch migration chi tiết
4. **Create Configuration Structure**: Tạo cấu trúc config files

### **2. Long-term Goals**

1. **Unified Effect System**: Hệ thống effect thống nhất
2. **Better Performance**: Performance tối ưu
3. **Easier Maintenance**: Dễ dàng maintain
4. **Future-Proof Design**: Thiết kế cho tương lai

### **3. Success Metrics**

1. **Effect Duplication**: Giảm 90% effect duplication
2. **Performance**: Cải thiện 50% effect processing performance
3. **Maintainability**: Giảm 70% thời gian maintain effects
4. **Extensibility**: Tăng 200% khả năng extend effects

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Proposal Complete  
**Maintainer**: Chaos World Team
