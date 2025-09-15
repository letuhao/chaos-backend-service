# Probability Mechanics Design

## 📋 **Tổng Quan**

Tài liệu này mô tả chi tiết hệ thống probability mechanics cho Element Core, bao gồm các công thức tính toán cho critical hits, accuracy, và các probability-based stats khác.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Smooth Probability Curves**
- Sử dụng sigmoid function để tạo smooth transition
- Tránh hard cutoffs và sudden jumps
- Cho phép gradual scaling từ 0% đến 100%

### **2. Balanced Scaling**
- Attacker quá mạnh so với defender → 100% chance
- Defender quá mạnh so với attacker → 0% chance
- Cân bằng ở giữa → probability tương ứng với stat difference

### **3. Configurable Parameters**
- Scaling factors có thể điều chỉnh
- Different elements có thể có different scaling
- Easy to balance và fine-tune

## 🔢 **Core Probability Formulas**

### **1. Sigmoid Function**

```rust
// Sigmoid function for smooth probability curves
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

// Alternative: Tanh-based sigmoid for different curve shape
pub fn tanh_sigmoid(x: f64) -> f64 {
    (x.tanh() + 1.0) / 2.0
}

// Custom sigmoid with configurable steepness
pub fn custom_sigmoid(x: f64, steepness: f64) -> f64 {
    1.0 / (1.0 + (-x * steepness).exp())
}
```

### **2. Base Probability Calculation**

```rust
// Base probability calculation including Omni stats
pub fn calculate_base_probability(
    attacker_omni_stat: f64,
    attacker_element_stat: f64,
    defender_omni_stat: f64,
    defender_element_stat: f64,
    scaling_factor: f64,
    steepness: f64,
) -> f64 {
    // Calculate total stats (Omni + Element)
    let total_attacker_stat = attacker_omni_stat + attacker_element_stat;
    let total_defender_stat = defender_omni_stat + defender_element_stat;
    
    // Calculate stat difference
    let difference = total_attacker_stat - total_defender_stat;
    let scaled_difference = difference / scaling_factor;
    custom_sigmoid(scaled_difference, steepness)
}
```

## ⚔️ **Critical Hit Mechanics**

### **1. Critical Hit Probability**

```rust
// Critical hit probability calculation including Omni stats
pub fn calculate_crit_probability(
    attacker_omni_crit_rate: f64,
    attacker_element_crit_rate: f64,
    defender_omni_resist_crit_rate: f64,
    defender_element_resist_crit_rate: f64,
    element_type: &str,
) -> f64 {
    let scaling_factor = get_crit_scaling_factor(element_type);
    let steepness = get_crit_steepness(element_type);
    
    calculate_base_probability(
        attacker_omni_crit_rate,
        attacker_element_crit_rate,
        defender_omni_resist_crit_rate,
        defender_element_resist_crit_rate,
        scaling_factor,
        steepness,
    )
}

// Get scaling factor for element type
fn get_crit_scaling_factor(element_type: &str) -> f64 {
    match element_type {
        "fire" => 120.0,      // Fire has higher crit scaling
        "ice" => 100.0,       // Ice has normal crit scaling
        "lightning" => 150.0, // Lightning has highest crit scaling
        "earth" => 80.0,      // Earth has lower crit scaling
        _ => 100.0,           // Default scaling
    }
}

// Get steepness for element type
fn get_crit_steepness(element_type: &str) -> f64 {
    match element_type {
        "fire" => 0.8,        // Fire has moderate steepness
        "ice" => 1.0,         // Ice has normal steepness
        "lightning" => 1.2,   // Lightning has high steepness
        "earth" => 0.6,       // Earth has low steepness
        _ => 1.0,             // Default steepness
    }
}
```

### **2. Critical Damage Calculation**

```rust
// Critical damage calculation
pub fn calculate_crit_damage(
    base_damage: f64,
    attacker_crit_damage: f64,
    defender_resist_crit_damage: f64,
    element_type: &str,
) -> f64 {
    let scaling_factor = get_crit_damage_scaling_factor(element_type);
    let steepness = get_crit_damage_steepness(element_type);
    
    let crit_multiplier = calculate_base_probability(
        attacker_crit_damage,
        defender_resist_crit_damage,
        scaling_factor,
        steepness,
    );
    
    // Convert probability to multiplier (1.0 to 3.0 range)
    let multiplier = 1.0 + (crit_multiplier * 2.0);
    base_damage * multiplier
}

// Get crit damage scaling factor
fn get_crit_damage_scaling_factor(element_type: &str) -> f64 {
    match element_type {
        "fire" => 100.0,
        "ice" => 80.0,
        "lightning" => 120.0,
        "earth" => 90.0,
        _ => 100.0,
    }
}
```

## 🎯 **Accuracy Mechanics**

### **1. Hit Accuracy Probability**

```rust
// Hit accuracy probability calculation including Omni stats
pub fn calculate_hit_probability(
    attacker_omni_accuracy: f64,
    attacker_element_accuracy: f64,
    defender_omni_dodge: f64,
    defender_element_dodge: f64,
    element_type: &str,
) -> f64 {
    let scaling_factor = get_accuracy_scaling_factor(element_type);
    let steepness = get_accuracy_steepness(element_type);
    
    calculate_base_probability(
        attacker_omni_accuracy,
        attacker_element_accuracy,
        defender_omni_dodge,
        defender_element_dodge,
        scaling_factor,
        steepness,
    )
}

// Get accuracy scaling factor
fn get_accuracy_scaling_factor(element_type: &str) -> f64 {
    match element_type {
        "fire" => 100.0,      // Fire has normal accuracy
        "ice" => 120.0,       // Ice has higher accuracy scaling
        "lightning" => 80.0,  // Lightning has lower accuracy scaling
        "earth" => 110.0,     // Earth has slightly higher accuracy
        _ => 100.0,
    }
}
```

### **2. Miss Attack Probability**

```rust
// Miss attack probability (inverse of hit probability)
pub fn calculate_miss_probability(
    attacker_accuracy: f64,
    defender_dodge: f64,
    element_type: &str,
) -> f64 {
    1.0 - calculate_hit_probability(attacker_accuracy, defender_dodge, element_type)
}
```

## 🛡️ **Defense Mechanics**

### **1. Damage Reduction Probability**

```rust
// Damage reduction probability
pub fn calculate_damage_reduction_probability(
    attacker_power: f64,
    defender_defense: f64,
    element_type: &str,
) -> f64 {
    let scaling_factor = get_defense_scaling_factor(element_type);
    let steepness = get_defense_steepness(element_type);
    
    // For defense, we want higher defense = higher reduction probability
    calculate_base_probability(
        defender_defense,
        attacker_power,
        scaling_factor,
        steepness,
    )
}

// Get defense scaling factor
fn get_defense_scaling_factor(element_type: &str) -> f64 {
    match element_type {
        "fire" => 100.0,
        "ice" => 120.0,       // Ice has better defense scaling
        "lightning" => 80.0,  // Lightning has worse defense scaling
        "earth" => 150.0,     // Earth has best defense scaling
        _ => 100.0,
    }
}
```

### **2. Shield Absorption Probability**

```rust
// Shield absorption probability
pub fn calculate_shield_absorption_probability(
    incoming_damage: f64,
    shield_absorption: f64,
    element_type: &str,
) -> f64 {
    let scaling_factor = get_absorption_scaling_factor(element_type);
    let steepness = get_absorption_steepness(element_type);
    
    calculate_base_probability(
        shield_absorption,
        incoming_damage,
        scaling_factor,
        steepness,
    )
}
```

## 🔄 **Element Interaction Mechanics**

### **1. Element Advantage System**

```rust
// Element advantage calculation
pub fn calculate_element_advantage(
    attacker_element: &str,
    defender_element: &str,
) -> f64 {
    let advantage_matrix = get_element_advantage_matrix();
    
    advantage_matrix
        .get(attacker_element)
        .and_then(|advantages| advantages.get(defender_element))
        .copied()
        .unwrap_or(1.0) // No advantage if not found
}

// Element advantage matrix
fn get_element_advantage_matrix() -> HashMap<String, HashMap<String, f64>> {
    let mut matrix = HashMap::new();
    
    // Fire advantages
    let mut fire_advantages = HashMap::new();
    fire_advantages.insert("ice".to_string(), 1.5);      // Fire > Ice
    fire_advantages.insert("earth".to_string(), 1.2);    // Fire > Earth
    fire_advantages.insert("water".to_string(), 0.7);    // Fire < Water
    matrix.insert("fire".to_string(), fire_advantages);
    
    // Water advantages
    let mut water_advantages = HashMap::new();
    water_advantages.insert("fire".to_string(), 1.5);    // Water > Fire
    water_advantages.insert("lightning".to_string(), 1.2); // Water > Lightning
    water_advantages.insert("earth".to_string(), 0.8);   // Water < Earth
    matrix.insert("water".to_string(), water_advantages);
    
    // Ice advantages
    let mut ice_advantages = HashMap::new();
    ice_advantages.insert("water".to_string(), 1.3);     // Ice > Water
    ice_advantages.insert("earth".to_string(), 1.1);     // Ice > Earth
    ice_advantages.insert("fire".to_string(), 0.6);      // Ice < Fire
    matrix.insert("ice".to_string(), ice_advantages);
    
    // Lightning advantages
    let mut lightning_advantages = HashMap::new();
    lightning_advantages.insert("water".to_string(), 1.4); // Lightning > Water
    lightning_advantages.insert("earth".to_string(), 0.9); // Lightning < Earth
    matrix.insert("lightning".to_string(), lightning_advantages);
    
    // Earth advantages
    let mut earth_advantages = HashMap::new();
    earth_advantages.insert("lightning".to_string(), 1.3); // Earth > Lightning
    earth_advantages.insert("fire".to_string(), 0.8);     // Earth < Fire
    earth_advantages.insert("water".to_string(), 1.2);    // Earth > Water
    matrix.insert("earth".to_string(), earth_advantages);
    
    matrix
}
```

### **2. Element Resistance System**

```rust
// Element resistance calculation
pub fn calculate_element_resistance(
    attacker_element: &str,
    defender_element: &str,
    base_resistance: f64,
) -> f64 {
    let advantage = calculate_element_advantage(attacker_element, defender_element);
    base_resistance * advantage
}
```

## 📊 **Advanced Probability Mechanics**

### **1. Multi-Element Probability**

```rust
// Multi-element probability calculation
pub fn calculate_multi_element_probability(
    attacker_elements: &[String],
    defender_elements: &[String],
    base_probability: f64,
) -> f64 {
    let mut total_advantage = 0.0;
    let mut count = 0;
    
    for attacker_element in attacker_elements {
        for defender_element in defender_elements {
            let advantage = calculate_element_advantage(attacker_element, defender_element);
            total_advantage += advantage;
            count += 1;
        }
    }
    
    let average_advantage = if count > 0 { total_advantage / count as f64 } else { 1.0 };
    base_probability * average_advantage
}
```

### **2. Conditional Probability**

```rust
// Conditional probability calculation
pub fn calculate_conditional_probability(
    base_probability: f64,
    conditions: &[ProbabilityCondition],
) -> f64 {
    let mut final_probability = base_probability;
    
    for condition in conditions {
        match condition.operator {
            ProbabilityOperator::Multiply => {
                final_probability *= condition.value;
            }
            ProbabilityOperator::Add => {
                final_probability += condition.value;
            }
            ProbabilityOperator::Subtract => {
                final_probability -= condition.value;
            }
            ProbabilityOperator::Divide => {
                if condition.value != 0.0 {
                    final_probability /= condition.value;
                }
            }
        }
    }
    
    final_probability.clamp(0.0, 1.0)
}

// Probability condition
#[derive(Debug, Clone)]
pub struct ProbabilityCondition {
    pub operator: ProbabilityOperator,
    pub value: f64,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum ProbabilityOperator {
    Multiply,
    Add,
    Subtract,
    Divide,
}
```

### **3. Time-Based Probability**

```rust
// Time-based probability calculation
pub fn calculate_time_based_probability(
    base_probability: f64,
    time_factor: f64,
    time_decay_rate: f64,
) -> f64 {
    let time_multiplier = (-time_factor * time_decay_rate).exp();
    base_probability * time_multiplier
}
```

## 🎲 **Random Number Generation**

### **1. Seeded Random Generation**

```rust
// Seeded random number generator
pub struct SeededRng {
    rng: StdRng,
    seed: u64,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }
    
    pub fn gen_probability(&mut self) -> f64 {
        self.rng.gen::<f64>()
    }
    
    pub fn gen_range(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.rng.gen::<f64>()
    }
}
```

### **2. Deterministic Probability Testing**

```rust
// Deterministic probability testing
pub fn test_probability_deterministic(
    probability: f64,
    seed: u64,
    iterations: usize,
) -> f64 {
    let mut rng = SeededRng::new(seed);
    let mut successes = 0;
    
    for _ in 0..iterations {
        if rng.gen_probability() < probability {
            successes += 1;
        }
    }
    
    successes as f64 / iterations as f64
}
```

## 🧪 **Testing & Validation**

### **1. Probability Distribution Tests**

```rust
#[cfg(test)]
mod probability_tests {
    use super::*;
    
    #[test]
    fn test_crit_probability_scaling() {
        // Test that higher attacker crit rate increases probability
        let prob1 = calculate_crit_probability(100.0, 50.0, "fire");
        let prob2 = calculate_crit_probability(200.0, 50.0, "fire");
        
        assert!(prob2 > prob1);
    }
    
    #[test]
    fn test_crit_probability_limits() {
        // Test that probability stays within bounds
        let prob = calculate_crit_probability(1000.0, 0.0, "fire");
        assert!(prob >= 0.0);
        assert!(prob <= 1.0);
    }
    
    #[test]
    fn test_accuracy_probability_balance() {
        // Test that equal stats give ~50% probability
        let prob = calculate_hit_probability(100.0, 100.0, "fire");
        assert!((prob - 0.5).abs() < 0.1);
    }
    
    #[test]
    fn test_element_advantage() {
        // Test fire advantage over ice
        let advantage = calculate_element_advantage("fire", "ice");
        assert!(advantage > 1.0);
        
        // Test fire disadvantage against water
        let disadvantage = calculate_element_advantage("fire", "water");
        assert!(disadvantage < 1.0);
    }
}
```

### **2. Performance Tests**

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_probability_calculation_performance() {
        let start = Instant::now();
        
        for _ in 0..10000 {
            let _ = calculate_crit_probability(100.0, 50.0, "fire");
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should complete in < 100ms
    }
}
```

## 🔧 **Configuration System**

### **1. Probability Configuration**

```yaml
# probability_config.yaml
version: 1
scaling_factors:
  crit_rate:
    fire: 120.0
    water: 100.0
    ice: 110.0
    lightning: 150.0
    earth: 80.0
  crit_damage:
    fire: 100.0
    water: 90.0
    ice: 80.0
    lightning: 120.0
    earth: 110.0
  accuracy:
    fire: 100.0
    water: 120.0
    ice: 110.0
    lightning: 80.0
    earth: 110.0
  defense:
    fire: 100.0
    water: 100.0
    ice: 120.0
    lightning: 80.0
    earth: 150.0

steepness_factors:
  crit_rate:
    fire: 0.8
    water: 1.0
    ice: 1.0
    lightning: 1.2
    earth: 0.6
  crit_damage:
    fire: 1.0
    water: 0.9
    ice: 0.8
    lightning: 1.1
    earth: 1.0
  accuracy:
    fire: 1.0
    water: 1.1
    ice: 1.0
    lightning: 0.9
    earth: 1.0
  defense:
    fire: 1.0
    water: 1.0
    ice: 1.1
    lightning: 0.9
    earth: 1.2

element_advantages:
  fire:
    ice: 1.5
    earth: 1.2
    water: 0.7
  water:
    fire: 1.5
    lightning: 1.2
    earth: 0.8
  ice:
    water: 1.3
    earth: 1.1
    fire: 0.6
  lightning:
    water: 1.4
    earth: 0.9
  earth:
    lightning: 1.3
    fire: 0.8
    water: 1.2
```

## 🎯 **Next Steps**

### **Phase 1: Core Probability System**
1. **Sigmoid Functions**: Implement base probability calculations
2. **Element Scaling**: Element-specific scaling factors
3. **Basic Mechanics**: Critical hit, accuracy, defense
4. **Configuration**: YAML-based configuration system

### **Phase 2: Advanced Features**
1. **Element Interactions**: Rock-paper-scissors system
2. **Multi-Element Support**: Multiple elements per actor
3. **Conditional Probability**: Complex condition system
4. **Time-Based Mechanics**: Time decay and bonuses

### **Phase 3: Integration**
1. **Combat Integration**: Replace current probability system
2. **Shield Integration**: Element-based shield mechanics
3. **Talent Integration**: Element-based talent effects
4. **Item Integration**: Element-based item bonuses

### **Phase 4: Optimization**
1. **Performance Tuning**: Optimize calculations
2. **Caching**: Cache probability calculations
3. **Batch Processing**: Process multiple calculations
4. **Monitoring**: Performance metrics and logging

---

*Tài liệu này sẽ được cập nhật khi có thêm yêu cầu và feedback từ team.*
