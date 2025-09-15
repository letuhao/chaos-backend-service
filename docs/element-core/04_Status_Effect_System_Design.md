# Status Effect System Design

## 📋 **Tổng Quan**

Tài liệu này mô tả hệ thống status effects cho Element Core, bao gồm 6 derived stats mới cho việc gây và chống status effects, cùng với định nghĩa chi tiết các status effects cho từng element type.

## 🎯 **Status Effect Derived Stats**

### **1. 6 Derived Stats Mới**

```rust
// Status Effect Derived Stats
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEffectStatType {
    // Gây status effects
    StatusProbability,        // Xác suất gây status effect
    StatusDuration,           // Thời gian kéo dài status effect
    StatusIntensity,          // Độ ảnh hưởng của status effect
    
    // Chống status effects
    StatusResistance,         // Kháng status effect
    StatusDurationReduction,  // Giảm thời gian status effect
    StatusIntensityReduction, // Giảm độ ảnh hưởng status effect
}

// Status Effect Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectConfig {
    pub element_type: String,
    pub status_effect: String,
    pub base_probability: f64,        // Base probability to apply
    pub base_duration: f64,           // Base duration in seconds
    pub base_intensity: f64,          // Base intensity multiplier
    pub max_duration: f64,            // Maximum duration cap
    pub max_intensity: f64,           // Maximum intensity cap
    pub stackable: bool,              // Can stack with itself
    pub max_stacks: i32,              // Maximum stacks allowed
    pub refresh_duration: bool,       // Refresh duration on reapply
}
```

### **2. Status Effect Calculation**

```rust
// Status effect calculation engine
pub struct StatusEffectEngine {
    element_registry: Arc<ElementRegistry>,
    probability_engine: Arc<ProbabilityEngine>,
}

impl StatusEffectEngine {
    // Calculate status effect probability including Omni stats
    pub fn calculate_status_probability(
        &self,
        attacker_omni_stats: &HashMap<DerivedStatType, f64>,
        attacker_element_stats: &HashMap<DerivedStatType, f64>,
        defender_omni_stats: &HashMap<DerivedStatType, f64>,
        defender_element_stats: &HashMap<DerivedStatType, f64>,
        element_type: &str,
        status_effect: &str,
    ) -> Result<f64, ElementError> {
        let config = self.get_status_config(element_type, status_effect)?;
        
        // Get attacker's total status probability (Omni + Element)
        let attacker_omni_prob = attacker_omni_stats.get(&DerivedStatType::StatusProbability).copied().unwrap_or(0.0);
        let attacker_element_prob = attacker_element_stats.get(&DerivedStatType::StatusProbability).copied().unwrap_or(0.0);
        let total_attacker_prob = attacker_omni_prob + attacker_element_prob;
        
        // Get defender's total status resistance (Omni + Element)
        let defender_omni_resistance = defender_omni_stats.get(&DerivedStatType::StatusResistance).copied().unwrap_or(0.0);
        let defender_element_resistance = defender_element_stats.get(&DerivedStatType::StatusResistance).copied().unwrap_or(0.0);
        let total_defender_resistance = defender_omni_resistance + defender_element_resistance;
        
        // Calculate base probability
        let base_prob = config.base_probability;
        let stat_difference = total_attacker_prob - total_defender_resistance;
        
        // Apply sigmoid function for smooth probability curve
        let probability = self.probability_engine.calculate_base_probability(
            attacker_omni_prob,
            attacker_element_prob,
            defender_omni_resistance,
            defender_element_resistance,
            100.0,  // scaling factor
            1.0,    // steepness
        );
        
        Ok(probability.clamp(0.0, 1.0))
    }
    
    // Calculate status effect duration
    pub fn calculate_status_duration(
        &self,
        attacker_stats: &HashMap<DerivedStatType, f64>,
        defender_stats: &HashMap<DerivedStatType, f64>,
        element_type: &str,
        status_effect: &str,
    ) -> Result<f64, ElementError> {
        let config = self.get_status_config(element_type, status_effect)?;
        
        // Get attacker's status duration
        let attacker_duration = attacker_stats.get(&DerivedStatType::StatusDuration).copied().unwrap_or(0.0);
        
        // Get defender's duration reduction
        let defender_reduction = defender_stats.get(&DerivedStatType::StatusDurationReduction).copied().unwrap_or(0.0);
        
        // Calculate final duration
        let base_duration = config.base_duration;
        let duration_bonus = attacker_duration - defender_reduction;
        let final_duration = base_duration + duration_bonus;
        
        // Apply caps
        let capped_duration = final_duration.clamp(0.0, config.max_duration);
        
        Ok(capped_duration)
    }
    
    // Calculate status effect intensity
    pub fn calculate_status_intensity(
        &self,
        attacker_stats: &HashMap<DerivedStatType, f64>,
        defender_stats: &HashMap<DerivedStatType, f64>,
        element_type: &str,
        status_effect: &str,
    ) -> Result<f64, ElementError> {
        let config = self.get_status_config(element_type, status_effect)?;
        
        // Get attacker's status intensity
        let attacker_intensity = attacker_stats.get(&DerivedStatType::StatusIntensity).copied().unwrap_or(0.0);
        
        // Get defender's intensity reduction
        let defender_reduction = defender_stats.get(&DerivedStatType::StatusIntensityReduction).copied().unwrap_or(0.0);
        
        // Calculate final intensity
        let base_intensity = config.base_intensity;
        let intensity_bonus = attacker_intensity - defender_reduction;
        let final_intensity = base_intensity + intensity_bonus;
        
        // Apply caps
        let capped_intensity = final_intensity.clamp(0.0, config.max_intensity);
        
        Ok(capped_intensity)
    }
}
```

## 🔥 **Element Status Effects Definition**

### **1. Ngũ Hành Status Effects**

#### **Hỏa (Fire) - Burning**
```rust
// Burning status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurningStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Damage per tick multiplier
    pub duration: f64,            // Duration in seconds
    pub tick_interval: f64,       // Tick interval in seconds
    pub current_tick: i32,        // Current tick count
    pub total_ticks: i32,         // Total ticks
    pub source_id: String,        // Source actor ID
    pub target_id: String,        // Target actor ID
    pub is_active: bool,          // Is currently active
}

impl BurningStatus {
    // Calculate damage for current tick
    pub fn calculate_tick_damage(&self, base_damage: f64) -> f64 {
        // Burning damage increases with each tick
        let tick_multiplier = 1.0 + (self.current_tick as f64 * 0.1);
        base_damage * self.intensity * tick_multiplier
    }
    
    // Process tick
    pub fn process_tick(&mut self) -> f64 {
        if !self.is_active {
            return 0.0;
        }
        
        let damage = self.calculate_tick_damage(100.0); // Base damage
        self.current_tick += 1;
        
        // Check if status should end
        if self.current_tick >= self.total_ticks {
            self.is_active = false;
        }
        
        damage
    }
}
```

#### **Thủy (Water) - Slow Movement & Slow Attack**
```rust
// Slow status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlowStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Slow intensity (0.0 = no slow, 1.0 = 100% slow)
    pub duration: f64,            // Duration in seconds
    pub movement_slow: f64,       // Movement speed reduction
    pub attack_slow: f64,         // Attack speed reduction
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}

impl SlowStatus {
    // Calculate movement speed multiplier
    pub fn get_movement_multiplier(&self) -> f64 {
        (1.0 - self.movement_slow * self.intensity).clamp(0.1, 1.0)
    }
    
    // Calculate attack speed multiplier
    pub fn get_attack_multiplier(&self) -> f64 {
        (1.0 - self.attack_slow * self.intensity).clamp(0.1, 1.0)
    }
}
```

#### **Thổ (Earth) - Petrification**
```rust
// Petrification status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetrificationStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Petrification intensity
    pub duration: f64,            // Duration in seconds
    pub blunt_vulnerability: f64, // Additional blunt damage vulnerability
    pub movement_penalty: f64,    // Movement speed penalty
    pub attack_penalty: f64,      // Attack speed penalty
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}

impl PetrificationStatus {
    // Calculate additional blunt damage
    pub fn calculate_blunt_bonus(&self, base_blunt_damage: f64) -> f64 {
        base_blunt_damage * self.blunt_vulnerability * self.intensity
    }
    
    // Get movement penalty
    pub fn get_movement_penalty(&self) -> f64 {
        self.movement_penalty * self.intensity
    }
    
    // Get attack penalty
    pub fn get_attack_penalty(&self) -> f64 {
        self.attack_penalty * self.intensity
    }
}
```

#### **Kim (Metal) - Bleeding**
```rust
// Bleeding status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BleedingStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Bleeding intensity
    pub duration: f64,            // Duration in seconds
    pub tick_interval: f64,       // Tick interval in seconds
    pub current_tick: i32,        // Current tick count
    pub total_ticks: i32,         // Total ticks
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}

impl BleedingStatus {
    // Calculate bleeding damage for current tick
    pub fn calculate_tick_damage(&self, base_damage: f64) -> f64 {
        // Bleeding damage is consistent per tick
        base_damage * self.intensity
    }
}
```

#### **Mộc (Wood) - Poison**
```rust
// Poison status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoisonStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Poison intensity
    pub duration: f64,            // Duration in seconds
    pub tick_interval: f64,       // Tick interval in seconds
    pub current_tick: i32,        // Current tick count
    pub total_ticks: i32,         // Total ticks
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}

impl PoisonStatus {
    // Calculate poison damage for current tick
    pub fn calculate_tick_damage(&self, base_damage: f64) -> f64 {
        // Poison damage is consistent per tick
        base_damage * self.intensity
    }
}
```

### **2. Light & Dark Status Effects**

#### **Light - Purification**
```rust
// Purification status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurificationStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Purification intensity
    pub duration: f64,            // Duration in seconds
    pub healing_per_tick: f64,    // Healing per tick
    pub debuff_removal_chance: f64, // Chance to remove debuffs
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}
```

#### **Dark - Corruption**
```rust
// Corruption status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorruptionStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Corruption intensity
    pub duration: f64,            // Duration in seconds
    pub damage_per_tick: f64,     // Damage per tick
    pub stat_reduction: f64,      // Stat reduction percentage
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}
```

### **3. Life & Death Status Effects**

#### **Life - Regeneration**
```rust
// Regeneration status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegenerationStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Regeneration intensity
    pub duration: f64,            // Duration in seconds
    pub healing_per_tick: f64,    // Healing per tick
    pub stat_boost: f64,          // Stat boost percentage
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}
```

#### **Death - Decay**
```rust
// Decay status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Decay intensity
    pub duration: f64,            // Duration in seconds
    pub damage_per_tick: f64,     // Damage per tick
    pub max_hp_reduction: f64,    // Maximum HP reduction
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}
```

### **4. Time & Space Status Effects**

#### **Time - Temporal Distortion**
```rust
// Temporal Distortion status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalDistortionStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Distortion intensity
    pub duration: f64,            // Duration in seconds
    pub cooldown_reduction: f64,  // Cooldown reduction percentage
    pub action_speed_boost: f64,  // Action speed boost
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}
```

#### **Space - Spatial Lock**
```rust
// Spatial Lock status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialLockStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Lock intensity
    pub duration: f64,            // Duration in seconds
    pub movement_lock: bool,      // Complete movement lock
    pub teleport_prevention: bool, // Prevent teleportation
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}
```

### **5. Advanced Status Effects**

#### **Mental - Confusion**
```rust
// Confusion status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfusionStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Confusion intensity
    pub duration: f64,            // Duration in seconds
    pub wrong_target_chance: f64, // Chance to target wrong enemy
    pub skill_failure_chance: f64, // Chance for skills to fail
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}
```

#### **Psychic - Charm**
```rust
// Charm status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharmStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Charm intensity
    pub duration: f64,            // Duration in seconds
    pub control_chance: f64,      // Chance to control target
    pub damage_reduction: f64,    // Damage reduction against charmer
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}
```

#### **Void - Entropy**
```rust
// Entropy status effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyStatus {
    pub id: String,
    pub element_type: String,
    pub intensity: f64,           // Entropy intensity
    pub duration: f64,            // Duration in seconds
    pub stat_decay: f64,          // Stat decay per tick
    pub skill_cooldown_increase: f64, // Skill cooldown increase
    pub source_id: String,
    pub target_id: String,
    pub is_active: bool,
}
```

## 🎮 **Status Effect Management System**

### **1. Status Effect Manager**

```rust
// Status effect manager
pub struct StatusEffectManager {
    active_statuses: HashMap<String, Vec<Box<dyn StatusEffect>>>,
    status_configs: HashMap<String, StatusEffectConfig>,
    event_dispatcher: Arc<ElementEventDispatcher>,
}

impl StatusEffectManager {
    // Apply status effect
    pub fn apply_status_effect(
        &mut self,
        target_id: &str,
        element_type: &str,
        status_effect: &str,
        attacker_stats: &HashMap<DerivedStatType, f64>,
        defender_stats: &HashMap<DerivedStatType, f64>,
    ) -> Result<bool, ElementError> {
        // Calculate probability
        let probability = self.calculate_status_probability(
            attacker_stats,
            defender_stats,
            element_type,
            status_effect,
        )?;
        
        // Check if status should be applied
        if !self.check_probability(probability) {
            return Ok(false);
        }
        
        // Calculate duration and intensity
        let duration = self.calculate_status_duration(
            attacker_stats,
            defender_stats,
            element_type,
            status_effect,
        )?;
        
        let intensity = self.calculate_status_intensity(
            attacker_stats,
            defender_stats,
            element_type,
            status_effect,
        )?;
        
        // Create status effect
        let status = self.create_status_effect(
            element_type,
            status_effect,
            duration,
            intensity,
        )?;
        
        // Apply to target
        self.add_status_to_target(target_id, status)?;
        
        Ok(true)
    }
    
    // Process all status effects for a target
    pub fn process_target_statuses(&mut self, target_id: &str, delta_time: f64) -> Result<Vec<StatusEffectResult>, ElementError> {
        let mut results = Vec::new();
        
        if let Some(statuses) = self.active_statuses.get_mut(target_id) {
            let mut to_remove = Vec::new();
            
            for (index, status) in statuses.iter_mut().enumerate() {
                let result = status.process_tick(delta_time)?;
                results.push(result);
                
                if !status.is_active() {
                    to_remove.push(index);
                }
            }
            
            // Remove inactive statuses
            for &index in to_remove.iter().rev() {
                statuses.remove(index);
            }
        }
        
        Ok(results)
    }
}
```

### **2. Status Effect Interface**

```rust
// Status effect trait
pub trait StatusEffect: Send + Sync {
    fn get_id(&self) -> &str;
    fn get_element_type(&self) -> &str;
    fn get_duration(&self) -> f64;
    fn get_intensity(&self) -> f64;
    fn is_active(&self) -> bool;
    fn process_tick(&mut self, delta_time: f64) -> Result<StatusEffectResult, ElementError>;
    fn clone_box(&self) -> Box<dyn StatusEffect>;
}

// Status effect result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectResult {
    pub status_id: String,
    pub element_type: String,
    pub damage: f64,
    pub healing: f64,
    pub stat_modifications: HashMap<String, f64>,
    pub special_effects: Vec<String>,
    pub is_finished: bool,
}
```

## 🧪 **Testing & Validation**

### **1. Status Effect Tests**

```rust
#[cfg(test)]
mod status_effect_tests {
    use super::*;
    
    #[test]
    fn test_burning_status_calculation() {
        let mut burning = BurningStatus {
            id: "test_burning".to_string(),
            element_type: "hoa".to_string(),
            intensity: 1.5,
            duration: 10.0,
            tick_interval: 1.0,
            current_tick: 0,
            total_ticks: 10,
            source_id: "attacker".to_string(),
            target_id: "target".to_string(),
            is_active: true,
        };
        
        // Test first tick
        let damage1 = burning.calculate_tick_damage(100.0);
        assert_eq!(damage1, 150.0); // 100 * 1.5 * 1.0
        
        // Process tick
        burning.process_tick();
        
        // Test second tick
        let damage2 = burning.calculate_tick_damage(100.0);
        assert_eq!(damage2, 165.0); // 100 * 1.5 * 1.1
    }
    
    #[test]
    fn test_slow_status_calculation() {
        let slow = SlowStatus {
            id: "test_slow".to_string(),
            element_type: "thuy".to_string(),
            intensity: 0.5,
            duration: 5.0,
            movement_slow: 0.8,
            attack_slow: 0.6,
            source_id: "attacker".to_string(),
            target_id: "target".to_string(),
            is_active: true,
        };
        
        let movement_mult = slow.get_movement_multiplier();
        let attack_mult = slow.get_attack_multiplier();
        
        assert_eq!(movement_mult, 0.6); // 1.0 - 0.8 * 0.5
        assert_eq!(attack_mult, 0.7);  // 1.0 - 0.6 * 0.5
    }
}
```

## 🎯 **Next Steps**

### **Phase 1: Core Status System**
1. **Status Effect Engine**: Implement core status calculation
2. **Basic Status Effects**: Implement Ngũ Hành status effects
3. **Status Manager**: Implement status management system
4. **Testing**: Create comprehensive test suite

### **Phase 2: Advanced Status Effects**
1. **Light/Dark Status**: Implement purification and corruption
2. **Life/Death Status**: Implement regeneration and decay
3. **Time/Space Status**: Implement temporal and spatial effects
4. **Mental Status**: Implement confusion and charm

### **Phase 3: Integration & Optimization**
1. **Combat Integration**: Integrate with combat system
2. **Performance Optimization**: Optimize status processing
3. **UI Integration**: Create status effect display
4. **Balance Testing**: Test status effect balance

---

*Tài liệu này sẽ được cập nhật khi có thêm yêu cầu và feedback từ team.*
