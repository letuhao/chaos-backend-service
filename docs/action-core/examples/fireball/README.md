# Action Core Examples

## 📋 **Tổng Quan**

Thư mục này chứa các ví dụ cụ thể về cách định nghĩa và implement actions trong Action Core system.

## 🔥 **Hỏa Cầu Thuật - Fireball Technique**

### **Mô Tả**

Hỏa Cầu Thuật là một kỹ thuật tấn công cơ bản của Kim Đan hệ thống, tạo ra một quả cầu lửa mạnh mẽ để tấn công kẻ thù với sát thương hỏa nguyên tố.

### **Đặc Điểm Chính**

- **Hệ Thống**: Kim Đan (Golden Pill System)
- **Loại**: Combat Action - Attack
- **Nguyên Tố**: Hỏa (Fire)
- **Tài Nguyên**: Linh khí (Qi) + Thể lực (Stamina)
- **Yêu Cầu**: Kim Đan kỳ 1, Độ tương thích hỏa ≥ 30

### **Files**

1. **`fireball_technique.yaml`** - Action definition trong YAML format
2. **`fireball_technique.rs`** - Rust implementation của action
3. **`README.md`** - Hướng dẫn sử dụng (file này)

### **Cấu Trúc Action**

#### **1. Metadata**
```yaml
action_id: "fireball_technique"
action_name: "Hỏa Cầu Thuật"
action_category: "combat"
action_type: "attack"
action_subtype: "elemental_spell"
```

#### **2. Resource Requirements**
```yaml
resource_requirements:
  - resource_type: "qi"           # Linh khí
    base_consumption: 50.0
    consumption_formula: "base_consumption * (1 + mastery_bonus * 0.1)"
    
  - resource_type: "stamina"      # Thể lực
    base_consumption: 20.0
    consumption_formula: "base_consumption * (1 - efficiency_bonus * 0.2)"
```

#### **3. Execution Conditions**
```yaml
execution_conditions:
  - condition: "self.qi >= 50"
    condition_type: "resource_available"
    required: true
    error_message: "Cần ít nhất 50 linh khí để thi triển Hỏa Cầu Thuật"
    
  - condition: "self.cultivation_realm >= 1"
    condition_type: "cultivation_requirement"
    required: true
    error_message: "Cần đạt Kim Đan kỳ 1 để thi triển kỹ thuật này"
    
  - condition: "self.fire_affinity >= 30"
    condition_type: "element_affinity"
    required: true
    error_message: "Cần độ tương thích hỏa nguyên tố ít nhất 30"
```

#### **4. Effects**
```yaml
effects:
  - effect_id: "fire_damage"
    effect_type: "damage"
    element: "fire"
    base_magnitude: 120.0
    magnitude_formula: "base_magnitude * (1 + fire_mastery * 0.02) * (1 + cultivation_bonus * 0.1)"
    
  - effect_id: "burning_status"
    effect_type: "status_effect"
    status_name: "burning"
    trigger_probability: 0.3
    duration: 8.0
    
  - effect_id: "heat_wave"
    effect_type: "area_effect"
    area_radius: 3.0
    trigger_probability: 0.8
    duration: 2.0
```

### **Cách Sử Dụng**

#### **1. YAML Configuration**

```yaml
# Load action từ YAML file
action_definition: fireball_technique.yaml

# Sử dụng trong game
actions:
  fireball_technique:
    enabled: true
    level_requirement: 5
    class_requirements: ["golden_pill_cultivator", "fire_mage"]
    cultivation_requirements:
      golden_pill_realm: 1
      fire_affinity: 30
```

#### **2. Rust Implementation**

```rust
use action_core::examples::fireball_technique::FireballTechnique;

// Tạo action instance
let fireball_action = FireballTechnique::new();

// Validate action
let context = ActionContext::new(attacker, target, "fireball_technique");
let validation_result = fireball_action.validate(&context);

if validation_result.success {
    // Execute action
    let mut context = ActionContext::new(attacker, target, "fireball_technique");
    let result = fireball_action.execute(&mut context);
    
    if result.success {
        println!("Hỏa Cầu Thuật đã được thi triển thành công!");
        if let Some(damage) = result.damage {
            println!("Sát thương: {:.2}", damage.total_damage);
            if damage.critical_hit {
                println!("Critical Hit!");
            }
        }
    }
} else {
    for error in validation_result.errors {
        println!("Lỗi: {:?}", error);
    }
}
```

### **Tính Năng Nâng Cao**

#### **1. Element Interactions**
```yaml
element_interactions:
  - interaction_type: "overcoming"
    target_element: "wood"
    damage_multiplier: 1.5
    status_probability_bonus: 0.2
    
  - interaction_type: "generating"
    target_element: "earth"
    damage_multiplier: 1.2
    status_probability_bonus: 0.1
    
  - interaction_type: "overcome_by"
    target_element: "water"
    damage_multiplier: 0.7
    status_probability_bonus: -0.3
```

#### **2. Cultivation Integration**
```yaml
cultivation_integration:
  system: "golden_pill"
  realm_requirement: 1
  realm_bonus: 0.1  # 10% bonus per realm level
  
  cultivation_stats:
    - stat_name: "fire_affinity"
      base_value: 30.0
      required: true
      
    - stat_name: "qi_control"
      base_value: 20.0
      required: true
```

#### **3. Status Effects**
```yaml
status_effects:
  - status_name: "burning"
    base_probability: 0.3
    base_duration: 8.0
    base_intensity: 1.0
    max_stacks: 3
    stackable: true
    refresh_duration: true
    
    probability_formula: "base_probability + fire_mastery * 0.005"
    duration_formula: "base_duration * (1 + fire_mastery * 0.05)"
    intensity_formula: "base_intensity * (1 + fire_mastery * 0.02)"
```

### **Formulas và Calculations**

#### **1. Damage Calculation**
```
Final Damage = Base Damage × (1 + Fire Mastery × 0.02) × (1 + Cultivation Bonus × 0.1) × (1 + Element Amplification × 0.05) × Element Interaction Multiplier
```

#### **2. Critical Hit**
```
Critical Chance = Base Critical Chance + Fire Mastery × 0.001
Critical Damage = Base Critical Damage + Fire Mastery × 0.002
```

#### **3. Status Effect Probability**
```
Burning Probability = Base Probability + Fire Mastery × 0.005
Burning Duration = Base Duration × (1 + Fire Mastery × 0.05)
Burning Intensity = Base Intensity × (1 + Fire Mastery × 0.02)
```

#### **4. Resource Consumption**
```
Qi Consumption = Base Consumption × (1 + Mastery Bonus × 0.1)
Stamina Consumption = Base Consumption × (1 - Efficiency Bonus × 0.2)
```

### **Ví Dụ Tính Toán**

#### **Scenario: Kim Đan kỳ 2, Fire Mastery 50**

```rust
// Base stats
let base_damage = 120.0;
let fire_mastery = 50.0;
let cultivation_bonus = 0.2; // 20% for realm 2
let element_amplification = 0.1; // 10%

// Calculate damage
let mastery_bonus = fire_mastery * 0.02; // 1.0
let cultivation_multiplier = 1.0 + cultivation_bonus; // 1.2
let amplification_multiplier = 1.0 + element_amplification; // 1.1

let final_damage = base_damage * (1.0 + mastery_bonus) * cultivation_multiplier * amplification_multiplier;
// = 120.0 × 2.0 × 1.2 × 1.1 = 316.8

// Critical hit
let critical_chance = 0.15 + fire_mastery * 0.001; // 0.2 (20%)
let critical_damage = 1.5 + fire_mastery * 0.002; // 1.6

// Status effects
let burning_probability = 0.3 + fire_mastery * 0.005; // 0.55 (55%)
let burning_duration = 8.0 * (1.0 + fire_mastery * 0.05); // 28.0 seconds
let burning_intensity = 1.0 * (1.0 + fire_mastery * 0.02); // 2.0

// Resource consumption
let qi_consumption = 50.0 * (1.0 + mastery_bonus * 0.1); // 55.0
let stamina_consumption = 20.0 * (1.0 - efficiency_bonus * 0.2); // 18.0
```

### **Testing**

#### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fireball_technique_creation() {
        let action = FireballTechnique::new();
        assert_eq!(action.metadata.id, "fireball_technique");
        assert_eq!(action.metadata.name, "Hỏa Cầu Thuật");
    }
    
    #[test]
    fn test_damage_calculation() {
        let action = FireballTechnique::new();
        let mut context = create_test_context();
        context.attacker.stats.insert("fire_mastery".to_string(), 50.0);
        
        let damage = action.calculate_damage(&context).unwrap();
        assert!(damage.total_damage > 120.0);
    }
    
    #[test]
    fn test_execution_conditions() {
        let action = FireballTechnique::new();
        let mut context = create_test_context();
        
        // Test insufficient qi
        context.attacker.resources.insert("qi".to_string(), 30.0);
        let result = action.validate(&context);
        assert!(!result.success);
    }
}
```

### **Performance Considerations**

#### **1. Caching**
- Action definitions được cache để tránh reload
- Damage calculations được cache cho hot path
- Resource consumption được cache

#### **2. Optimization**
- Formula evaluation được optimize
- Status effect calculations được batch process
- Element interactions được pre-calculated

#### **3. Memory Management**
- Action instances được reuse
- Context objects được pool
- Status effects được efficiently managed

### **Integration Points**

#### **1. Element Core**
- Fire mastery stats
- Element interactions
- Status effect definitions

#### **2. Cultivation Core**
- Golden Pill system integration
- Realm requirements
- Cultivation bonuses

#### **3. Resource Manager**
- Qi consumption
- Stamina consumption
- Resource validation

#### **4. Combat Core**
- Damage calculation
- Critical hit system
- Status effect application

### **Kết Luận**

Hỏa Cầu Thuật là một ví dụ hoàn chỉnh về cách định nghĩa và implement một combat action trong Action Core system. Nó thể hiện:

- ✅ **Unified Interface** - Implement Action trait
- ✅ **Resource Management** - Qi và Stamina consumption
- ✅ **Execution Conditions** - Cultivation và affinity requirements
- ✅ **Element Integration** - Fire element và interactions
- ✅ **Status Effects** - Burning status với formulas
- ✅ **Cultivation Integration** - Golden Pill system
- ✅ **Performance Optimization** - Caching và efficient calculations
- ✅ **Type Safety** - Strong typing cho tất cả components
- ✅ **Flexibility** - Configurable formulas và parameters
- ✅ **Localization** - Vietnamese và English support

Đây là foundation tốt để phát triển các actions phức tạp hơn trong tương lai!
