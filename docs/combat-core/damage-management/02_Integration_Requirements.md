# Integration Requirements

## 📋 **Tổng Quan**

Document này liệt kê tất cả các thiết kế cần chỉnh sửa để tích hợp Damage Manager vào hệ thống hiện tại.

## 🎯 **Systems Cần Chỉnh Sửa**

### **1. Combat Core**
- **02_Damage_System_Design.md** - Cần cập nhật để sử dụng Damage Manager
- **03_Combat_System_Design.md** - Cần tích hợp Damage Manager
- **04_Combat_Action_System_Design.md** - Cần cập nhật damage handling
- **05_Combat_Status_Integration_Design.md** - Cần cập nhật status damage handling

### **2. Status Core**
- **12_Status_Core_Combat_Integration_Design.md** - Cần cập nhật để sử dụng Damage Manager
- **05_Status_Core_Core_System_Design.md** - Cần cập nhật damage request structure

### **3. Element Core**
- **08_Elemental_Mastery_System_Design.md** - Cần cập nhật damage calculation integration
- **10_Element_Interaction_System_Design.md** - Cần cập nhật elemental damage handling

### **4. Action Core**
- **04_Action_Definition_System.md** - Cần cập nhật action damage handling
- **05_Action_Execution_Engine_Design.md** - Cần cập nhật damage application

### **5. Actor Core**
- **Resource Management System** - Cần cập nhật để tích hợp với Damage Manager
- **Derived Stats System** - Cần cập nhật damage-related stats

## 🔄 **Chi Tiết Cập Nhật**

### **1. Combat Core Updates**

#### **02_Damage_System_Design.md**
- **Thay đổi**: Thay thế direct damage calculation bằng Damage Manager
- **Cập nhật**: Damage calculation pipeline
- **Thêm**: Damage Manager integration
- **Xóa**: Hard-coded damage calculations

#### **03_Combat_System_Design.md**
- **Thay đổi**: Tích hợp Damage Manager vào combat flow
- **Cập nhật**: Combat tick processing
- **Thêm**: Damage Manager configuration
- **Xóa**: Direct resource management

#### **04_Combat_Action_System_Design.md**
- **Thay đổi**: Action damage handling sử dụng Damage Manager
- **Cập nhật**: Action execution flow
- **Thêm**: Damage request creation
- **Xóa**: Direct damage application

#### **05_Combat_Status_Integration_Design.md**
- **Thay đổi**: Status damage handling sử dụng Damage Manager
- **Cập nhật**: Status effect processing
- **Thêm**: Status damage request creation
- **Xóa**: Direct status damage calculation

### **2. Status Core Updates**

#### **12_Status_Core_Combat_Integration_Design.md**
- **Thay đổi**: Status damage handling sử dụng Damage Manager
- **Cập nhật**: Status effect processing flow
- **Thêm**: Damage request creation từ status effects
- **Xóa**: Direct damage calculation trong StatusCore

#### **05_Status_Core_Core_System_Design.md**
- **Thay đổi**: StatusEffectProcessingResult structure
- **Cập nhật**: Status damage request creation
- **Thêm**: Damage Manager integration interface
- **Xóa**: Direct damage calculation methods

### **3. Element Core Updates**

#### **08_Elemental_Mastery_System_Design.md**
- **Thay đổi**: Elemental damage calculation integration
- **Cập nhật**: Mastery bonus calculation
- **Thêm**: Damage Manager integration
- **Xóa**: Direct damage calculation

#### **10_Element_Interaction_System_Design.md**
- **Thay đổi**: Elemental damage handling
- **Cập nhật**: Element interaction damage
- **Thêm**: Damage Manager integration
- **Xóa**: Direct elemental damage calculation

### **4. Action Core Updates**

#### **04_Action_Definition_System.md**
- **Thay đổi**: Action damage handling
- **Cập nhật**: Action execution flow
- **Thêm**: Damage request creation từ actions
- **Xóa**: Direct damage application

#### **05_Action_Execution_Engine_Design.md**
- **Thay đổi**: Action execution sử dụng Damage Manager
- **Cập nhật**: Action result processing
- **Thêm**: Damage Manager integration
- **Xóa**: Direct damage calculation

### **5. Actor Core Updates**

#### **Resource Management System**
- **Thay đổi**: Resource updates sử dụng Damage Manager
- **Cập nhật**: Resource modification flow
- **Thêm**: Damage Manager integration interface
- **Xóa**: Direct resource damage application

#### **Derived Stats System**
- **Thay đổi**: Damage-related stats calculation
- **Cập nhật**: Stats integration với Damage Manager
- **Thêm**: Damage modifier stats
- **Xóa**: Direct damage stats calculation

## 🔧 **Integration Points**

### **1. Damage Manager Interfaces**

```rust
/// Damage Manager Interface
#[async_trait]
pub trait DamageManagerInterface: Send + Sync {
    /// Apply damage to actor
    async fn apply_damage(
        &self,
        request: DamageRequest
    ) -> Result<DamageResult, DamageError>;
    
    /// Apply damage batch
    async fn apply_damage_batch(
        &self,
        requests: Vec<DamageRequest>
    ) -> Result<Vec<DamageResult>, DamageError>;
    
    /// Get damage modifiers
    async fn get_damage_modifiers(
        &self,
        actor_id: &str,
        damage_type: &DamageType
    ) -> Result<Vec<DamageModifier>, DamageError>;
    
    /// Check damage immunity
    async fn check_damage_immunity(
        &self,
        actor_id: &str,
        damage_type: &DamageType
    ) -> Result<bool, DamageError>;
}
```

### **2. Resource Manager Bridge**

```rust
/// Resource Manager Bridge
pub struct ResourceManagerBridge {
    resource_manager: Arc<dyn ResourceManagerInterface>,
    damage_manager: Arc<dyn DamageManagerInterface>,
}

impl ResourceManagerBridge {
    /// Apply damage to resources
    pub async fn apply_damage_to_resources(
        &self,
        actor_id: &str,
        damage: f64,
        damage_type: &DamageType
    ) -> Result<ResourceUpdateResult, ResourceError> {
        // Create damage request
        let damage_request = DamageRequest {
            actor_id: actor_id.to_string(),
            damage_type: damage_type.clone(),
            base_damage: damage,
            damage_source: DamageSource::Direct,
            element_id: None,
            source_id: None,
            modifiers: Vec::new(),
            properties: HashMap::new(),
            context: DamageContext::default(),
        };
        
        // Apply damage through Damage Manager
        let damage_result = self.damage_manager.apply_damage(damage_request).await?;
        
        // Convert to resource update result
        Ok(ResourceUpdateResult {
            resource_id: damage_type.to_string(),
            old_value: damage_result.base_damage,
            new_value: damage_result.final_damage,
            change_amount: damage_result.damage_applied,
        })
    }
}
```

### **3. Element Core Bridge**

```rust
/// Element Core Bridge
pub struct ElementCoreBridge {
    element_core: Arc<dyn ElementCoreInterface>,
    damage_manager: Arc<dyn DamageManagerInterface>,
}

impl ElementCoreBridge {
    /// Get elemental damage modifiers
    pub async fn get_elemental_damage_modifiers(
        &self,
        actor_id: &str,
        element_id: &str,
        damage_type: &DamageType
    ) -> Result<Vec<DamageModifier>, DamageError> {
        // Get element mastery data
        let mastery_data = self.element_core.get_element_mastery_data(actor_id, element_id).await?;
        
        // Create damage modifiers from mastery data
        let mut modifiers = Vec::new();
        
        // Mastery bonus modifier
        if mastery_data.mastery_level > 0 {
            modifiers.push(DamageModifier {
                modifier_type: DamageModifierType::Multiplier,
                value: 1.0 + (mastery_data.mastery_level as f64 * 0.1),
                source: format!("element_mastery_{}", element_id),
                condition: None,
                properties: HashMap::new(),
            });
        }
        
        // Element resistance modifier
        if mastery_data.resistance > 0.0 {
            modifiers.push(DamageModifier {
                modifier_type: DamageModifierType::Resistance,
                value: mastery_data.resistance,
                source: format!("element_resistance_{}", element_id),
                condition: None,
                properties: HashMap::new(),
            });
        }
        
        Ok(modifiers)
    }
}
```

### **4. Status Core Bridge**

```rust
/// Status Core Bridge
pub struct StatusCoreBridge {
    status_core: Arc<dyn StatusCoreInterface>,
    damage_manager: Arc<dyn DamageManagerInterface>,
}

impl StatusCoreBridge {
    /// Get status damage modifiers
    pub async fn get_status_damage_modifiers(
        &self,
        actor_id: &str,
        damage_type: &DamageType
    ) -> Result<Vec<DamageModifier>, DamageError> {
        // Get active status effects
        let status_effects = self.status_core.get_actor_status_effects(actor_id).await?;
        
        let mut modifiers = Vec::new();
        
        // Process each status effect
        for effect in status_effects {
            if effect.requires_damage_calculation {
                // Create damage modifier from status effect
                let modifier = DamageModifier {
                    modifier_type: DamageModifierType::Multiplier,
                    value: effect.magnitude,
                    source: format!("status_effect_{}", effect.effect_id),
                    condition: None,
                    properties: effect.properties,
                };
                modifiers.push(modifier);
            }
        }
        
        Ok(modifiers)
    }
}
```

### **5. Action Core Bridge**

```rust
/// Action Core Bridge
pub struct ActionCoreBridge {
    action_core: Arc<dyn ActionCoreInterface>,
    damage_manager: Arc<dyn DamageManagerInterface>,
}

impl ActionCoreBridge {
    /// Get action damage modifiers
    pub async fn get_action_damage_modifiers(
        &self,
        actor_id: &str,
        action_id: &str,
        damage_type: &DamageType
    ) -> Result<Vec<DamageModifier>, DamageError> {
        // Get action definition
        let action_def = self.action_core.get_action_definition(action_id).await?;
        
        let mut modifiers = Vec::new();
        
        // Action effectiveness modifier
        if let Some(effectiveness) = action_def.properties.get("effectiveness") {
            if let Some(eff_value) = effectiveness.as_f64() {
                modifiers.push(DamageModifier {
                    modifier_type: DamageModifierType::Multiplier,
                    value: eff_value,
                    source: format!("action_effectiveness_{}", action_id),
                    condition: None,
                    properties: HashMap::new(),
                });
            }
        }
        
        // Action cooldown modifier
        if let Some(cooldown_reduction) = action_def.properties.get("cooldown_reduction") {
            if let Some(cd_value) = cooldown_reduction.as_f64() {
                modifiers.push(DamageModifier {
                    modifier_type: DamageModifierType::Multiplier,
                    value: 1.0 + cd_value,
                    source: format!("action_cooldown_{}", action_id),
                    condition: None,
                    properties: HashMap::new(),
                });
            }
        }
        
        Ok(modifiers)
    }
}
```

## 📝 **Implementation Priority**

### **Phase 1: Core Integration (Week 1-2)**
1. **Combat Core**: Update damage system design
2. **Damage Manager**: Implement core functionality
3. **Resource Manager Bridge**: Implement resource integration

### **Phase 2: System Integration (Week 3-4)**
1. **Status Core**: Update status damage handling
2. **Element Core**: Update elemental damage handling
3. **Action Core**: Update action damage handling

### **Phase 3: Advanced Features (Week 5-6)**
1. **Advanced Modifiers**: Implement complex modifier system
2. **Performance Optimization**: Implement caching và batch processing
3. **Testing**: Comprehensive testing

### **Phase 4: Polish & Documentation (Week 7-8)**
1. **Documentation**: Complete all documentation
2. **Performance Tuning**: Fine-tune performance
3. **Integration Testing**: End-to-end testing

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
