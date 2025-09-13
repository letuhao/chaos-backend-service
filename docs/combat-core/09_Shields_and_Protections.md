# Shields and Protections

## Overview
This document defines shield types, merge/replace policies, stacking math, penetration order, reflection rules, and references to configuration files.

## Shield Types & Conflict Policies
- ImmunityShield: single instance (new replaces old)
- AbsorptionShield: merge up to N; value stacking scalar s=0.8; regen stacking scalar r=0.7
- ReflectionShield: replace weakest; no chain; absorb_pct and reflect_pct configurable
- Physical/Armor/Weapon: replace_oldest when cap exceeded; merge if same source
- Magic/Elemental/Spiritual/Qi: merge by school/element; otherwise replace_oldest

See `configs/shields.yaml:type_policies` for exact parameters.

## Processing Order
Priority: Immunity → Absorption → Reflection → Magic/Elemental/Spiritual/Qi → Physical/Armor/Weapon.

## Penetration (Per Shield)
- Order: flat → percent
- Percent cap: configurable per type; default unlimited (100% possible)
- Mappings: `armor_pen`, `magic_pen`, `elemental_pen`, `universal_pen`

See `configs/shields.yaml:penetration` for caps and mappings.

## Reflection
- Reflection absorbs X% post-penetration and reflects Y% of absorbed to the source
- Does not chain; per-reflection caps configurable

See `configs/shields.yaml:type_policies.ReflectionShield.reflect`.

## Merge Precedence
Overrides apply in order: PvP template overrides → area overrides → global shields.yaml.

See `configs/shields.yaml:merge_precedence` and `configs/pvp_templates.yaml:overrides.shields`.

## Events
- ShieldBrokenEvent on break
- ProtectionAppliedEvent for protections on resources

## Examples
- Numeric and type-matched examples are in `04_Damage_Application_System.md` (distribution scenarios) and configs.

## Reference: Shield Actor Types

```rust
// Shield as independent actors with their own HP and behavior
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShieldActor {
    pub shield_id: String,
    pub shield_type: ShieldType,
    pub owner_actor_id: String,
    pub shield_hp: f64,           // Shield's own HP (like actor's HP)
    pub max_shield_hp: f64,       // Maximum shield HP
    pub damage_types: Vec<String>, // Only takes specific damage types
    pub priority: i64,            // Lower number = damaged first
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub lifetime_decay_rate: f64, // HP decay per second
    pub restoration_events: Vec<String>, // Events that restore shield HP
    pub is_active: bool,
    pub subsystem_id: String,     // Which subsystem registered this shield
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShieldType {
    // Physical shields
    PhysicalShield,
    ArmorShield,
    WeaponShield,
    
    // Magical shields
    MagicShield,
    FireShield,
    IceShield,
    LightningShield,
    EarthShield,
    WindShield,
    WaterShield,
    
    // Special shields
    AbsorptionShield,
    ReflectionShield,
    ImmunityShield,
    
    // Cultivation shields
    QiShield,
    SpiritualShield,
    LifeForceShield,
    SoulShield,
}

impl ShieldActor {
    /// Calculate shield priority (lower = damaged first)
    pub fn calculate_priority(&self, damage_type: &str) -> i64 {
        // Base priority from subsystem registration
        let base_priority = self.priority;
        
        // Type-specific modifier for damage type
        let type_modifier = self.get_damage_type_modifier(damage_type);
        
        // Remaining HP percentage modifier
        let hp_percentage = (self.shield_hp / self.max_shield_hp * 100.0) as i64;
        
        // Formula: base + floor(remaining/max × 100) + type_modifier
        base_priority + hp_percentage + type_modifier
    }
    
    /// Check if shield can take this damage type
    pub fn can_take_damage_type(&self, damage_type: &str) -> bool {
        self.damage_types.is_empty() || self.damage_types.contains(&damage_type.to_string())
    }
    
    /// Apply damage to shield (like damage to actor)
    pub fn apply_damage(&mut self, damage: f64) -> f64 {
        if !self.is_active || self.shield_hp <= 0.0 {
            return 0.0;
        }
        
        let actual_damage = self.shield_hp.min(damage);
        self.shield_hp -= actual_damage;
        
        // Check if shield is broken
        if self.shield_hp <= 0.0 {
            self.is_active = false;
        }
        
        actual_damage
    }
    
    /// Apply lifetime decay
    pub fn apply_lifetime_decay(&mut self, delta_time: f64) {
        if self.is_active && self.lifetime_decay_rate > 0.0 {
            let decay = self.lifetime_decay_rate * delta_time;
            self.shield_hp = (self.shield_hp - decay).max(0.0);
            
            if self.shield_hp <= 0.0 {
                self.is_active = false;
            }
        }
    }
    
    /// Restore shield HP from events
    pub fn restore_hp(&mut self, amount: f64) {
        if self.is_active {
            self.shield_hp = (self.shield_hp + amount).min(self.max_shield_hp);
        }
    }
    
    /// Get damage type modifier
    fn get_damage_type_modifier(&self, damage_type: &str) -> i64 {
        match (&self.shield_type, damage_type) {
            (ShieldType::FireShield, "fire") => 50,    // Fire shield resists fire
            (ShieldType::IceShield, "ice") => 50,      // Ice shield resists ice
            (ShieldType::PhysicalShield, "physical") => 30,
            (ShieldType::MagicShield, "magical") => 30,
            _ => 0,
        }
    }
}
```

## Reference: Shield Processing & Penetration

```rust
// Enhanced shield order processing with subsystem support
pub struct ShieldOrderProcessor {
    registration_system: Arc<ShieldRegistrationSystem>,
    penetration_calculator: PenetrationCalculator,
}

impl ShieldOrderProcessor {
    /// Process damage through shields with subsystem priority
    pub async fn process_damage_through_shields(
        &mut self,
        actor_id: &str,
        damage: &mut DamageResult,
    ) -> ActorCoreResult<ShieldProcessingResult> {
        let mut remaining_damage = damage.final_damage;
        let mut shield_results = Vec::new();
        let mut total_absorbed = 0.0;
        let mut shields_broken = Vec::new();
        
        // Get shields sorted by priority (lower number = damaged first)
        let shields = self.registration_system
            .get_shields_for_damage(actor_id, &damage.damage_type)
            .await?;
        
        // Linear scan through active, compatible shields
        for mut shield in shields {
            if remaining_damage <= 0.0 {
                break; // Stop when damage <= 0
            }
            
            if !shield.is_active || shield.shield_hp <= 0.0 {
                continue;
            }
            
            // Apply damage to shield (like damage to actor)
            let shield_damage = self.calculate_shield_damage(&shield, remaining_damage, &damage.damage_type).await?;
            let actual_damage = shield.apply_damage(shield_damage);
            
            remaining_damage -= actual_damage;
            total_absorbed += actual_damage;
            
            // Check if shield is broken
            if shield.shield_hp <= 0.0 {
                shields_broken.push(shield.shield_id.clone());
                shield.is_active = false;
            }
            
            // Record shield result
            shield_results.push(ShieldResult {
                shield_id: shield.shield_id.clone(),
                shield_type: shield.shield_type.clone(),
                damage_absorbed: actual_damage,
                remaining_hp: shield.shield_hp,
                is_broken: shield.shield_hp <= 0.0,
                subsystem_id: shield.subsystem_id.clone(),
            });
            
            // Update shield in registration system
            self.registration_system.update_shield(actor_id, &shield).await?;
        }
        
        // Update damage result
        damage.final_damage = remaining_damage;
        damage.shield_absorbed = total_absorbed;
        
        Ok(ShieldProcessingResult {
            original_damage: damage.final_damage + total_absorbed,
            final_damage: damage.final_damage,
            total_absorbed,
            shield_results,
            shields_broken,
            processing_time: current_timestamp(),
        })
    }
    
    /// Calculate damage to a specific shield
    async fn calculate_shield_damage(
        &self,
        shield: &ShieldInfo,
        incoming_damage: f64,
        damage_type: &str,
    ) -> ActorCoreResult<f64> {
        // Base damage to shield
        let mut shield_damage = incoming_damage;
        
        // Apply penetration: flat then percent; percent cap configurable (default unlimited)
        let pen = self.penetration_calculator
            .calculate_penetration(damage_type, &shield.shield_type, &shield.damage_types)
            .await?; // { flat, percent, cap }

        let after_flat = (shield_damage - pen.flat.max(0.0)).max(0.0);
        let pct = match pen.cap { Some(cap) => pen.percent.min(cap), None => pen.percent };
        shield_damage = after_flat * (1.0 - pct.max(0.0).min(1.0));
        
        // Apply shield type modifiers (after penetration)
        let type_modifier = self.get_shield_type_modifier(&shield.shield_type, damage_type);
        shield_damage *= type_modifier;
        
        Ok(shield_damage)
    }
}
```

## Reference: Shield Stacking Rules

```rust
// Shield stacking rules and management
pub struct ShieldRules {
    pub max_shields_per_type: HashMap<ShieldType, usize>,
    pub stacking_modifiers: HashMap<ShieldType, f64>,
    pub conflict_resolution: ConflictResolutionStrategy,
}

#[derive(Debug, Clone)]
pub enum ConflictResolutionStrategy {
    /// Replace oldest shield of same type
    ReplaceOldest,
    /// Replace weakest shield of same type
    ReplaceWeakest,
    /// Merge shields of same type
    Merge,
    /// Reject new shield
    Reject,
}

impl ShieldRules {
    /// Check if new shield can be added
    pub fn can_add_shield(
        &self,
        existing_shields: &[ShieldInfo],
        new_shield: &ShieldInfo,
    ) -> bool {
        let max_count = self.max_shields_per_type
            .get(&new_shield.shield_type)
            .copied()
            .unwrap_or(1);
        
        let current_count = existing_shields
            .iter()
            .filter(|s| s.shield_type == new_shield.shield_type && s.is_active())
            .count();
        
        current_count < max_count
    }
    
    /// Resolve shield conflicts
    pub fn resolve_conflict(
        &self,
        existing_shields: &mut Vec<ShieldInfo>,
        new_shield: ShieldInfo,
    ) -> ActorCoreResult<Option<ShieldInfo>> {
        if self.can_add_shield(existing_shields, &new_shield) {
            return Ok(Some(new_shield));
        }
        
        match self.conflict_resolution {
            ConflictResolutionStrategy::ReplaceOldest => {
                self.replace_oldest_shield(existing_shields, new_shield)
            }
            ConflictResolutionStrategy::ReplaceWeakest => {
                self.replace_weakest_shield(existing_shields, new_shield)
            }
            ConflictResolutionStrategy::Merge => {
                self.merge_shields(existing_shields, new_shield)
            }
            ConflictResolutionStrategy::Reject => {
                Ok(None)
            }
        }
    }
}
```
