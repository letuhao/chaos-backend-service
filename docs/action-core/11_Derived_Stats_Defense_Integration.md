# Derived Stats Defense Integration

## 📋 **Tổng Quan**

Document này mô tả chi tiết mối tương quan giữa các derived stats từ Element Core và các hệ thống khác với Defense Action System. Các derived stats ảnh hưởng trực tiếp đến hiệu quả của defense actions, tạo ra depth và strategic choices cho players.

## 🎯 **Mối Tương Quan Derived Stats vs Defense Actions**

### **1. Element Core Derived Stats**

#### **A. Combat Defense Mechanics**

**ParryRate & BlockRate**
```rust
// Parry rate calculation for defense actions
fn calculate_defense_parry_rate(
    base_parry: f64,
    element_mastery: f64,
    attacker_element_mastery: f64,
    defense_action_bonus: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.0002; // 0.02% per mastery point
    let attacker_penalty = attacker_element_mastery * 0.0001; // 0.01% per attacker mastery
    let action_bonus = defense_action_bonus * 0.1; // 10% per action level
    
    (base_parry + mastery_bonus + action_bonus - attacker_penalty)
        .min(0.75).max(0.0)
}

// Block rate calculation for defense actions
fn calculate_defense_block_rate(
    base_block: f64,
    element_mastery: f64,
    attacker_element_mastery: f64,
    defense_action_bonus: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.0003; // 0.03% per mastery point
    let attacker_penalty = attacker_element_mastery * 0.0001; // 0.01% per attacker mastery
    let action_bonus = defense_action_bonus * 0.15; // 15% per action level
    
    (base_block + mastery_bonus + action_bonus - attacker_penalty)
        .min(0.8).max(0.0)
}
```

**Parry/Block Strength & Shred**
```rust
// Parry strength calculation
fn calculate_parry_strength(
    base_strength: f64,
    element_mastery: f64,
    defense_action_power: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.0005; // 0.05% per mastery point
    let action_power = defense_action_power * 0.2; // 20% per action power
    
    base_strength + mastery_bonus + action_power
}

// Block strength calculation
fn calculate_block_strength(
    base_strength: f64,
    element_mastery: f64,
    defense_action_power: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.0008; // 0.08% per mastery point
    let action_power = defense_action_power * 0.25; // 25% per action power
    
    base_strength + mastery_bonus + action_power
}

// Parry/Block shred calculation
fn calculate_parry_block_shred(
    base_shred: f64,
    element_mastery: f64,
    defense_action_penetration: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.0003; // 0.03% per mastery point
    let action_penetration = defense_action_penetration * 0.15; // 15% per action penetration
    
    base_shred + mastery_bonus + action_penetration
}
```

#### **B. Element Penetration & Absorption**

**Element Penetration**
```rust
// Element penetration for defense actions
fn calculate_defense_element_penetration(
    base_penetration: f64,
    element_mastery: f64,
    target_element_mastery: f64,
    defense_action_type: DefenseActionType
) -> f64 {
    let mastery_bonus = element_mastery * 0.0004; // 0.04% per mastery point
    let target_penalty = target_element_mastery * 0.0002; // 0.02% per target mastery
    
    let action_multiplier = match defense_action_type {
        DefenseActionType::Parry => 1.5,  // Parry has higher penetration
        DefenseActionType::Block => 1.0,  // Block has normal penetration
        DefenseActionType::Dodge => 0.5,  // Dodge has lower penetration
        DefenseActionType::Shield => 1.2, // Shield has moderate penetration
    };
    
    (base_penetration + mastery_bonus - target_penalty) * action_multiplier
}
```

**Element Absorption**
```rust
// Element absorption for defense actions
fn calculate_defense_element_absorption(
    base_absorption: f64,
    element_mastery: f64,
    incoming_element_mastery: f64,
    defense_action_type: DefenseActionType
) -> f64 {
    let mastery_bonus = element_mastery * 0.0006; // 0.06% per mastery point
    let incoming_penalty = incoming_element_mastery * 0.0003; // 0.03% per incoming mastery
    
    let action_multiplier = match defense_action_type {
        DefenseActionType::Shield => 2.0,  // Shield has highest absorption
        DefenseActionType::Block => 1.5,   // Block has high absorption
        DefenseActionType::Parry => 0.8,   // Parry has low absorption
        DefenseActionType::Dodge => 0.0,   // Dodge has no absorption
    };
    
    (base_absorption + mastery_bonus - incoming_penalty) * action_multiplier
}
```

**Element Reflection**
```rust
// Element reflection for defense actions
fn calculate_defense_element_reflection(
    base_reflection: f64,
    element_mastery: f64,
    incoming_element_mastery: f64,
    defense_action_type: DefenseActionType
) -> f64 {
    let mastery_bonus = element_mastery * 0.0005; // 0.05% per mastery point
    let incoming_penalty = incoming_element_mastery * 0.0002; // 0.02% per incoming mastery
    
    let action_multiplier = match defense_action_type {
        DefenseActionType::Parry => 1.8,  // Parry has highest reflection
        DefenseActionType::Shield => 1.5, // Shield has high reflection
        DefenseActionType::Block => 1.0,  // Block has normal reflection
        DefenseActionType::Dodge => 0.0,  // Dodge has no reflection
    };
    
    (base_reflection + mastery_bonus - incoming_penalty) * action_multiplier
}
```

#### **C. Status Effect Resistance**

**Status Resistance**
```rust
// Status resistance for defense actions
fn calculate_defense_status_resistance(
    base_resistance: f64,
    element_mastery: f64,
    incoming_status_power: f64,
    defense_action_type: DefenseActionType
) -> f64 {
    let mastery_bonus = element_mastery * 0.0003; // 0.03% per mastery point
    let incoming_penalty = incoming_status_power * 0.0001; // 0.01% per incoming power
    
    let action_multiplier = match defense_action_type {
        DefenseActionType::Shield => 1.6,  // Shield has highest resistance
        DefenseActionType::Block => 1.3,   // Block has high resistance
        DefenseActionType::Parry => 1.0,   // Parry has normal resistance
        DefenseActionType::Dodge => 0.7,   // Dodge has low resistance
    };
    
    (base_resistance + mastery_bonus - incoming_penalty) * action_multiplier
}
```

**Status Duration Reduction**
```rust
// Status duration reduction for defense actions
fn calculate_defense_status_duration_reduction(
    base_reduction: f64,
    element_mastery: f64,
    incoming_status_duration: f64,
    defense_action_type: DefenseActionType
) -> f64 {
    let mastery_bonus = element_mastery * 0.0002; // 0.02% per mastery point
    let incoming_penalty = incoming_status_duration * 0.0001; // 0.01% per incoming duration
    
    let action_multiplier = match defense_action_type {
        DefenseActionType::Shield => 1.4,  // Shield has highest reduction
        DefenseActionType::Block => 1.2,   // Block has high reduction
        DefenseActionType::Parry => 1.0,   // Parry has normal reduction
        DefenseActionType::Dodge => 0.8,   // Dodge has low reduction
    };
    
    (base_reduction + mastery_bonus - incoming_penalty) * action_multiplier
}
```

#### **D. Skill Execution & Performance**

**Skill Execution Speed**
```rust
// Defense action execution speed
fn calculate_defense_execution_speed(
    base_speed: f64,
    element_mastery: f64,
    defense_action_complexity: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.001; // 0.1% per mastery point
    let complexity_penalty = defense_action_complexity * 0.1; // 10% penalty per complexity
    
    base_speed * (1.0 + mastery_bonus - complexity_penalty)
}
```

**Skill Cooldown Reduction**
```rust
// Defense action cooldown reduction
fn calculate_defense_cooldown_reduction(
    base_cooldown: f64,
    element_mastery: f64,
    defense_action_level: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.0005; // 0.05% per mastery point
    let level_bonus = defense_action_level * 0.02; // 2% per action level
    
    base_cooldown * (1.0 - (mastery_bonus + level_bonus).min(0.5)) // Max 50% reduction
}
```

**Resource Efficiency**
```rust
// Defense action resource efficiency
fn calculate_defense_resource_efficiency(
    base_resource_cost: f64,
    element_mastery: f64,
    defense_action_efficiency: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.0003; // 0.03% per mastery point
    let action_efficiency = defense_action_efficiency * 0.1; // 10% per action efficiency
    
    base_resource_cost * (1.0 - (mastery_bonus + action_efficiency).min(0.3)) // Max 30% reduction
}
```

### **2. Combat Core Derived Stats**

#### **A. Base Combat Stats**

**Defense Point**
```rust
// Defense point calculation for defense actions
fn calculate_defense_point(
    base_defense: f64,
    element_mastery: f64,
    defense_action_power: f64,
    equipment_bonus: f64
) -> f64 {
    let mastery_bonus = element_mastery * 0.0008; // 0.08% per mastery point
    let action_power = defense_action_power * 0.3; // 30% per action power
    let equipment = equipment_bonus * 0.5; // 50% per equipment bonus
    
    base_defense + mastery_bonus + action_power + equipment
}
```

**Critical Hit Resistance**
```rust
// Critical hit resistance for defense actions
fn calculate_defense_critical_resistance(
    base_resistance: f64,
    element_mastery: f64,
    incoming_critical_chance: f64,
    defense_action_type: DefenseActionType
) -> f64 {
    let mastery_bonus = element_mastery * 0.0004; // 0.04% per mastery point
    let incoming_penalty = incoming_critical_chance * 0.0002; // 0.02% per incoming critical
    
    let action_multiplier = match defense_action_type {
        DefenseActionType::Parry => 1.5,  // Parry has highest critical resistance
        DefenseActionType::Block => 1.2,  // Block has high critical resistance
        DefenseActionType::Shield => 1.0, // Shield has normal critical resistance
        DefenseActionType::Dodge => 0.0,  // Dodge has no critical resistance
    };
    
    (base_resistance + mastery_bonus - incoming_penalty) * action_multiplier
}
```

#### **B. Accuracy & Dodge**

**Dodge Rate**
```rust
// Dodge rate calculation for defense actions
fn calculate_defense_dodge_rate(
    base_dodge: f64,
    element_mastery: f64,
    attacker_accuracy: f64,
    defense_action_type: DefenseActionType
) -> f64 {
    let mastery_bonus = element_mastery * 0.0006; // 0.06% per mastery point
    let attacker_penalty = attacker_accuracy * 0.0003; // 0.03% per attacker accuracy
    
    let action_multiplier = match defense_action_type {
        DefenseActionType::Dodge => 2.0,  // Dodge has highest dodge rate
        DefenseActionType::Parry => 1.2,  // Parry has moderate dodge rate
        DefenseActionType::Block => 0.8,  // Block has low dodge rate
        DefenseActionType::Shield => 0.5, // Shield has very low dodge rate
    };
    
    (base_dodge + mastery_bonus - attacker_penalty) * action_multiplier
}
```

### **3. Resource Manager Derived Stats**

#### **A. Resource Regeneration**

**Resource Regeneration**
```rust
// Resource regeneration for defense actions
fn calculate_defense_resource_regeneration(
    base_regeneration: f64,
    element_mastery: f64,
    defense_action_efficiency: f64,
    resource_type: ResourceType
) -> f64 {
    let mastery_bonus = element_mastery * 0.0002; // 0.02% per mastery point
    let action_efficiency = defense_action_efficiency * 0.05; // 5% per action efficiency
    
    let resource_multiplier = match resource_type {
        ResourceType::Health => 1.0,     // Normal regeneration
        ResourceType::Mana => 1.2,       // 20% bonus for mana
        ResourceType::Stamina => 1.5,    // 50% bonus for stamina
        ResourceType::Qi => 1.8,         // 80% bonus for qi
        ResourceType::LifeForce => 0.5,  // 50% penalty for life force
        ResourceType::Vitality => 0.7,   // 30% penalty for vitality
        ResourceType::Energy => 1.3,     // 30% bonus for energy
    };
    
    base_regeneration * (1.0 + mastery_bonus + action_efficiency) * resource_multiplier
}
```

**Resource Efficiency**
```rust
// Resource efficiency for defense actions
fn calculate_defense_resource_efficiency(
    base_efficiency: f64,
    element_mastery: f64,
    defense_action_level: f64,
    resource_type: ResourceType
) -> f64 {
    let mastery_bonus = element_mastery * 0.0004; // 0.04% per mastery point
    let level_bonus = defense_action_level * 0.08; // 8% per action level
    
    let resource_multiplier = match resource_type {
        ResourceType::Health => 1.0,     // Normal efficiency
        ResourceType::Mana => 1.1,       // 10% bonus for mana
        ResourceType::Stamina => 1.3,    // 30% bonus for stamina
        ResourceType::Qi => 1.5,         // 50% bonus for qi
        ResourceType::LifeForce => 0.3,  // 70% penalty for life force
        ResourceType::Vitality => 0.5,   // 50% penalty for vitality
        ResourceType::Energy => 1.2,     // 20% bonus for energy
    };
    
    base_efficiency * (1.0 + mastery_bonus + level_bonus) * resource_multiplier
}
```

### **4. Cultivation Core Derived Stats**

#### **A. Cultivation Level Bonuses**

**Cultivation Level Bonus**
```rust
// Cultivation level bonus for defense actions
fn calculate_cultivation_defense_bonus(
    cultivation_level: f64,
    element_mastery: f64,
    defense_action_type: DefenseActionType
) -> f64 {
    let level_bonus = cultivation_level * 0.01; // 1% per cultivation level
    let mastery_bonus = element_mastery * 0.0001; // 0.01% per mastery point
    
    let action_multiplier = match defense_action_type {
        DefenseActionType::Shield => 1.5,  // Shield benefits most from cultivation
        DefenseActionType::Block => 1.3,   // Block benefits moderately
        DefenseActionType::Parry => 1.1,   // Parry benefits slightly
        DefenseActionType::Dodge => 1.0,   // Dodge benefits least
    };
    
    (level_bonus + mastery_bonus) * action_multiplier
}
```

**Cultivation Realm Bonus**
```rust
// Cultivation realm bonus for defense actions
fn calculate_cultivation_realm_defense_bonus(
    cultivation_realm: u32,
    element_mastery: f64,
    defense_action_power: f64
) -> f64 {
    let realm_bonus = cultivation_realm as f64 * 0.05; // 5% per realm
    let mastery_bonus = element_mastery * 0.0002; // 0.02% per mastery point
    let action_power = defense_action_power * 0.1; // 10% per action power
    
    realm_bonus + mastery_bonus + action_power
}
```

### **5. Equipment & Item Derived Stats**

#### **A. Equipment Bonuses**

**Equipment Defense Bonus**
```rust
// Equipment defense bonus for defense actions
fn calculate_equipment_defense_bonus(
    equipment_defense: f64,
    element_mastery: f64,
    equipment_quality: f64,
    defense_action_type: DefenseActionType
) -> f64 {
    let base_bonus = equipment_defense * 0.8; // 80% of equipment defense
    let mastery_bonus = element_mastery * 0.0003; // 0.03% per mastery point
    let quality_bonus = equipment_quality * 0.2; // 20% per quality level
    
    let action_multiplier = match defense_action_type {
        DefenseActionType::Shield => 1.4,  // Shield benefits most from equipment
        DefenseActionType::Block => 1.2,   // Block benefits moderately
        DefenseActionType::Parry => 1.0,   // Parry benefits normally
        DefenseActionType::Dodge => 0.8,   // Dodge benefits least
    };
    
    (base_bonus + mastery_bonus + quality_bonus) * action_multiplier
}
```

**Equipment Element Resistance**
```rust
// Equipment element resistance for defense actions
fn calculate_equipment_element_resistance(
    equipment_resistance: f64,
    element_mastery: f64,
    incoming_element: String,
    defense_action_type: DefenseActionType
) -> f64 {
    let base_resistance = equipment_resistance * 0.6; // 60% of equipment resistance
    let mastery_bonus = element_mastery * 0.0004; // 0.04% per mastery point
    
    let action_multiplier = match defense_action_type {
        DefenseActionType::Shield => 1.6,  // Shield has highest element resistance
        DefenseActionType::Block => 1.3,   // Block has high element resistance
        DefenseActionType::Parry => 1.0,   // Parry has normal element resistance
        DefenseActionType::Dodge => 0.5,   // Dodge has low element resistance
    };
    
    (base_resistance + mastery_bonus) * action_multiplier
}
```

## 🔄 **Defense Action Effectiveness Calculation**

### **1. Overall Defense Effectiveness**

```rust
// Overall defense effectiveness calculation
fn calculate_defense_effectiveness(
    defense_action: &dyn Action,
    defender: &Actor,
    incoming_attack: &IncomingAttack,
    derived_stats: &DerivedStatsSnapshot
) -> Result<DefenseEffectiveness, DefenseError> {
    let mut effectiveness = DefenseEffectiveness::new();
    
    // Get element mastery for defense action
    let element_mastery = derived_stats.element_derived_stats
        .get(&defense_action.get_primary_element())
        .unwrap_or(0.0);
    
    // Calculate base defense effectiveness
    effectiveness.base_effectiveness = calculate_base_defense_effectiveness(
        defense_action,
        element_mastery,
        incoming_attack
    )?;
    
    // Calculate parry/block rates
    effectiveness.parry_rate = calculate_defense_parry_rate(
        derived_stats.element_derived_stats.parry_rate,
        element_mastery,
        incoming_attack.attacker_element_mastery,
        defense_action.get_power()
    );
    
    effectiveness.block_rate = calculate_defense_block_rate(
        derived_stats.element_derived_stats.block_rate,
        element_mastery,
        incoming_attack.attacker_element_mastery,
        defense_action.get_power()
    );
    
    // Calculate dodge rate
    effectiveness.dodge_rate = calculate_defense_dodge_rate(
        derived_stats.element_derived_stats.dodge_rate,
        element_mastery,
        incoming_attack.attacker_accuracy,
        defense_action.get_type()
    );
    
    // Calculate damage mitigation
    effectiveness.damage_mitigation = calculate_damage_mitigation(
        defense_action,
        element_mastery,
        incoming_attack,
        derived_stats
    )?;
    
    // Calculate status resistance
    effectiveness.status_resistance = calculate_defense_status_resistance(
        derived_stats.element_derived_stats.status_resistance,
        element_mastery,
        incoming_attack.status_power,
        defense_action.get_type()
    );
    
    // Calculate counter-attack chance
    effectiveness.counter_attack_chance = calculate_counter_attack_chance(
        defense_action,
        element_mastery,
        incoming_attack,
        derived_stats
    )?;
    
    // Calculate resource efficiency
    effectiveness.resource_efficiency = calculate_defense_resource_efficiency(
        derived_stats.resource_stats.resource_efficiency,
        element_mastery,
        defense_action.get_level(),
        defense_action.get_resource_type()
    );
    
    Ok(effectiveness)
}
```

### **2. Defense Action Type Multipliers**

```rust
// Defense action type multipliers
pub struct DefenseActionTypeMultipliers {
    pub parry_multiplier: f64,
    pub block_multiplier: f64,
    pub dodge_multiplier: f64,
    pub shield_multiplier: f64,
}

impl DefenseActionTypeMultipliers {
    pub fn new() -> Self {
        Self {
            parry_multiplier: 1.0,
            block_multiplier: 1.0,
            dodge_multiplier: 1.0,
            shield_multiplier: 1.0,
        }
    }
    
    pub fn get_multiplier(&self, action_type: DefenseActionType) -> f64 {
        match action_type {
            DefenseActionType::Parry => self.parry_multiplier,
            DefenseActionType::Block => self.block_multiplier,
            DefenseActionType::Dodge => self.dodge_multiplier,
            DefenseActionType::Shield => self.shield_multiplier,
        }
    }
    
    pub fn apply_element_mastery_bonus(&mut self, element_mastery: f64) {
        let mastery_bonus = element_mastery * 0.0001; // 0.01% per mastery point
        
        self.parry_multiplier += mastery_bonus;
        self.block_multiplier += mastery_bonus;
        self.dodge_multiplier += mastery_bonus;
        self.shield_multiplier += mastery_bonus;
    }
}
```

### **3. Element Interaction Effects**

```rust
// Element interaction effects on defense actions
fn calculate_element_interaction_defense_bonus(
    defender_element: String,
    attacker_element: String,
    defense_action_type: DefenseActionType
) -> f64 {
    let interaction_bonus = get_element_interaction_bonus(
        &defender_element,
        &attacker_element
    );
    
    let action_multiplier = match defense_action_type {
        DefenseActionType::Parry => 1.5,  // Parry benefits most from element interaction
        DefenseActionType::Block => 1.2,  // Block benefits moderately
        DefenseActionType::Shield => 1.0, // Shield benefits normally
        DefenseActionType::Dodge => 0.8,  // Dodge benefits least
    };
    
    interaction_bonus * action_multiplier
}
```

## 📊 **Performance Optimization**

### **1. Derived Stats Caching**

```rust
// Derived stats cache for defense actions
pub struct DefenseDerivedStatsCache {
    cache: HashMap<String, DefenseEffectiveness>,
    cache_ttl: Duration,
    max_cache_size: usize,
}

impl DefenseDerivedStatsCache {
    pub fn get_effectiveness(&self, key: &str) -> Option<&DefenseEffectiveness> {
        self.cache.get(key)
    }
    
    pub fn set_effectiveness(&mut self, key: String, effectiveness: DefenseEffectiveness) {
        if self.cache.len() >= self.max_cache_size {
            self.evict_oldest();
        }
        self.cache.insert(key, effectiveness);
    }
    
    fn evict_oldest(&mut self) {
        // Implement LRU eviction
    }
}
```

### **2. Batch Processing**

```rust
// Batch processing for defense actions
pub struct DefenseBatchProcessor {
    batch_size: usize,
    processing_queue: Vec<DefenseActionRequest>,
}

impl DefenseBatchProcessor {
    pub async fn process_defense_batch(&mut self) -> Result<Vec<DefenseResult>, DefenseError> {
        let mut results = Vec::new();
        
        for batch in self.processing_queue.chunks(self.batch_size) {
            let batch_results = self.process_defense_batch_chunk(batch).await?;
            results.extend(batch_results);
        }
        
        self.processing_queue.clear();
        Ok(results)
    }
    
    async fn process_defense_batch_chunk(
        &self,
        batch: &[DefenseActionRequest]
    ) -> Result<Vec<DefenseResult>, DefenseError> {
        // Process batch in parallel
        let futures: Vec<_> = batch.iter()
            .map(|request| self.process_defense_request(request))
            .collect();
        
        let results = futures::future::join_all(futures).await;
        results.into_iter().collect()
    }
}
```

## 🧪 **Testing Strategy**

### **1. Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_defense_parry_rate_calculation() {
        let base_parry = 0.3;
        let element_mastery = 1000.0;
        let attacker_mastery = 500.0;
        let action_bonus = 0.1;
        
        let parry_rate = calculate_defense_parry_rate(
            base_parry,
            element_mastery,
            attacker_mastery,
            action_bonus
        );
        
        assert!(parry_rate > base_parry);
        assert!(parry_rate <= 0.75); // Max parry rate
    }
    
    #[test]
    fn test_defense_block_rate_calculation() {
        let base_block = 0.4;
        let element_mastery = 1000.0;
        let attacker_mastery = 500.0;
        let action_bonus = 0.15;
        
        let block_rate = calculate_defense_block_rate(
            base_block,
            element_mastery,
            attacker_mastery,
            action_bonus
        );
        
        assert!(block_rate > base_block);
        assert!(block_rate <= 0.8); // Max block rate
    }
    
    #[test]
    fn test_defense_dodge_rate_calculation() {
        let base_dodge = 0.2;
        let element_mastery = 1000.0;
        let attacker_accuracy = 0.8;
        let action_type = DefenseActionType::Dodge;
        
        let dodge_rate = calculate_defense_dodge_rate(
            base_dodge,
            element_mastery,
            attacker_accuracy,
            action_type
        );
        
        assert!(dodge_rate > base_dodge);
        assert!(dodge_rate <= 1.0); // Max dodge rate
    }
    
    #[test]
    fn test_defense_damage_mitigation_calculation() {
        let base_mitigation = 0.5;
        let element_mastery = 1000.0;
        let incoming_damage = 1000.0;
        let action_type = DefenseActionType::Block;
        
        let mitigation = calculate_damage_mitigation(
            base_mitigation,
            element_mastery,
            incoming_damage,
            action_type
        );
        
        assert!(mitigation > base_mitigation);
        assert!(mitigation <= 1.0); // Max mitigation
    }
}
```

### **2. Integration Tests**

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_defense_action_with_element_core_integration() {
        let defense_system = DefenseActionSystem::new();
        let element_core = ElementCoreClient::new();
        let defense_action = BlockAction::new();
        let defender = create_test_actor();
        let incoming_attack = create_test_incoming_attack();
        
        // Get derived stats from Element Core
        let derived_stats = element_core.get_derived_stats(&defender).await?;
        
        // Calculate defense effectiveness
        let effectiveness = defense_system.calculate_defense_effectiveness(
            &defense_action,
            &defender,
            &incoming_attack,
            &derived_stats
        ).await?;
        
        assert!(effectiveness.base_effectiveness > 0.0);
        assert!(effectiveness.parry_rate > 0.0);
        assert!(effectiveness.block_rate > 0.0);
    }
    
    #[tokio::test]
    async fn test_defense_action_with_combat_core_integration() {
        let defense_system = DefenseActionSystem::new();
        let combat_core = CombatCoreClient::new();
        let defense_action = ParryAction::new();
        let defender = create_test_actor();
        let incoming_attack = create_test_incoming_attack();
        
        // Calculate defense effectiveness
        let effectiveness = defense_system.calculate_defense_effectiveness(
            &defense_action,
            &defender,
            &incoming_attack,
            &derived_stats
        ).await?;
        
        // Apply defense to incoming attack
        let mitigated_damage = combat_core.apply_defense_mitigation(
            &incoming_attack,
            &effectiveness
        ).await?;
        
        assert!(mitigated_damage < incoming_attack.damage);
    }
}
```

### **3. Performance Tests**

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[tokio::test]
    async fn test_defense_effectiveness_calculation_performance() {
        let defense_system = DefenseActionSystem::new();
        let defense_action = BlockAction::new();
        let defender = create_test_actor();
        let incoming_attack = create_test_incoming_attack();
        let derived_stats = create_test_derived_stats();
        
        let start = Instant::now();
        
        for _ in 0..1000 {
            let _ = defense_system.calculate_defense_effectiveness(
                &defense_action,
                &defender,
                &incoming_attack,
                &derived_stats
            ).await;
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should complete in < 100ms
    }
    
    #[tokio::test]
    async fn test_defense_batch_processing_performance() {
        let mut processor = DefenseBatchProcessor::new();
        let requests = create_test_defense_requests(100);
        
        let start = Instant::now();
        
        let results = processor.process_defense_batch().await?;
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50); // Should complete in < 50ms
        assert_eq!(results.len(), 100);
    }
}
```

## 🔗 **Integration Points**

### **1. Element Core Integration**
- **Element Mastery**: Access element mastery levels
- **Element Interactions**: Handle element interactions
- **Element Synergy**: Calculate element synergy bonuses
- **Element Resistance**: Calculate element resistance

### **2. Combat Core Integration**
- **Damage Mitigation**: Apply damage mitigation
- **Status Effect Resistance**: Handle status effect resistance
- **Critical Hit Resistance**: Handle critical hit resistance
- **Counter-Attack System**: Handle counter-attack system

### **3. Resource Manager Integration**
- **Resource Consumption**: Handle resource consumption
- **Resource Regeneration**: Handle resource regeneration
- **Resource Efficiency**: Handle resource efficiency
- **Resource Validation**: Validate resource availability

### **4. Cultivation Core Integration**
- **Cultivation Level**: Access cultivation level
- **Cultivation Realm**: Access cultivation realm
- **Cultivation Bonuses**: Apply cultivation bonuses
- **Cultivation Effects**: Handle cultivation effects

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
