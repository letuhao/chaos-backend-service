# Resource Damage Distribution

## Overview
Defines distribution order, impact maps, resource categories, and examples.

## Distribution Order
```
Shields → Temporary Resources → Primary Resources → Secondary Resources → Special Resources
```

## Default Impact Maps
- Physical: Health 100%
- Magical: Health 80%, Mana 20%
- Elemental: Health 85%, Matching-element storage 15%
- Qi/Spiritual: Health 70%, Qi/SpiritualEnergy 30%
- True: Health 100% (bypasses shields and secondary; absolute immunities only)

## Resource Categories
- Primary: Health, LifeForce, Lifespan
- Secondary: Mana, Qi, SpiritualEnergy, ElementalStorage, Guard, Stagger
- Special: Soul, Essence, Vitality
- Temporary: TemporaryHealth, TemporaryMana, TemporaryQi

## Examples
Include examples as in `04_Damage_Application_System.md` and extend as needed.

## Reference: Resource Structures

```rust
// Resource priority and damage application
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceInfo {
    pub resource_id: String,
    pub resource_type: ResourceType,
    pub priority: i64,
    pub current_value: f64,
    pub max_value: f64,
    pub regeneration_rate: f64,
    pub is_protected: bool,
    pub protection_factor: f64,
    pub depletion_effects: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    // Primary resources
    Health,
    Mana,
    Stamina,
    
    // Cultivation resources
    Qi,
    SpiritualEnergy,
    LifeForce,
    Lifespan,
    
    // Special resources
    Soul,
    Essence,
    Vitality,
    
    // Temporary resources
    TemporaryHealth,
    TemporaryMana,
    TemporaryQi,
}

impl ResourceInfo {
    /// Calculate resource priority
    pub fn calculate_priority(&self) -> i64 {
        let base_priority = match self.resource_type {
            ResourceType::Health => 1000,
            ResourceType::LifeForce => 900,
            ResourceType::Lifespan => 800,
            ResourceType::Soul => 700,
            ResourceType::Qi => 600,
            ResourceType::SpiritualEnergy => 500,
            ResourceType::Mana => 400,
            ResourceType::Stamina => 300,
            ResourceType::Essence => 200,
            ResourceType::Vitality => 100,
            _ => 50,
        };
        
        // Add priority based on current value percentage
        let value_priority = (self.current_value / self.max_value * 100.0) as i64;
        
        // Add protection bonus
        let protection_bonus = if self.is_protected { 100 } else { 0 };
        
        base_priority + value_priority + protection_bonus
    }
    
    /// Check if resource can be damaged
    pub fn can_be_damaged(&self) -> bool {
        self.current_value > 0.0 && !self.is_protected
    }
    
    /// Apply damage to resource
    pub fn apply_damage(&mut self, damage: f64) -> f64 {
        if !self.can_be_damaged() { return 0.0; }
        let effective_damage = damage * (1.0 - self.protection_factor);
        let actual_damage = self.current_value.min(effective_damage);
        self.current_value -= actual_damage;
        if self.current_value <= 0.0 { self.trigger_depletion_effects(); }
        actual_damage
    }
    
    /// Trigger depletion effects
    fn trigger_depletion_effects(&self) {
        // Implementation hook
    }
}
```

## Reference: Distribution Engine

```rust
// Enhanced resource damage distribution system with fairness rules
pub struct ResourceDamageDistributor {
    resources: HashMap<String, Vec<ResourceInfo>>,
    distribution_rules: DistributionRules,
    damage_impact_maps: HashMap<String, DamageImpactMap>,
    protection_calculator: ProtectionCalculator,
}

#[derive(Debug, Clone)]
pub struct DamageImpactMap {
    pub damage_type: String,
    pub impact_distribution: Vec<ResourceImpact>,
    pub bypass_secondary: bool,  // True damage bypasses secondary resources
}

#[derive(Debug, Clone)]
pub struct ResourceImpact {
    pub resource_type: ResourceType,
    pub percentage: f64,         // Percentage of residual damage after shields
    pub penetration_resistance: f64,
    pub is_primary: bool,
    pub is_secondary: bool,
    pub is_special: bool,
    pub is_temporary: bool,
}
```

## Reference: Example Distribution Scenarios

```
Physical Damage (1000 damage)
Shields: 200 absorbed
Remaining: 800
→ Health: 800 (100%)
Final: 0 remaining damage

Magical Damage (1000 damage)
Shields: 150 absorbed
Remaining: 850
→ Health: 680 (80% of 850)
→ Mana: 170 (20% of 850)
Final: 0 remaining damage

Qi Damage (1000 damage)
Shields: 50 absorbed
Remaining: 950
→ Health: 665 (70% of 950)
→ Qi: 285 (30% of 950)
Final: 0 remaining damage

True Damage (1000 damage)
Shields: 0 absorbed (bypasses shields)
Remaining: 1000
→ Health: 1000 (100%, bypasses secondary)
Final: 0 remaining damage (unless absolute immunity applies)
```
