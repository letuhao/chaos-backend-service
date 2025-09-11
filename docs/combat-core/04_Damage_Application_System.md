# Damage Application System Design

## 📋 **Tổng Quan Hệ Thống**

Tài liệu này mô tả chi tiết hệ thống áp dụng damage, bao gồm xử lý shield order, resource damage application logic, và các cơ chế bảo vệ trong combat.

## 🎯 **Mục Tiêu Thiết Kế**

### **Performance Goals**
- **Ultra-Fast Application**: Damage application trong ~0.05ms
- **Batch Processing**: Xử lý nhiều damage events cùng lúc
- **Memory Efficient**: Tối ưu memory usage cho shield processing
- **Deterministic**: Kết quả nhất quán và có thể reproduce

### **Game Balance Goals**
- **Fair Shield Order**: Thứ tự shield công bằng và logic
- **Resource Protection**: Bảo vệ resources quan trọng
- **Flexible Damage Types**: Hỗ trợ nhiều loại damage khác nhau
- **Status Effect Integration**: Tích hợp với status effects

## 🏗️ **Kiến Trúc Hệ Thống**

### **Core Components**

```
Damage Application System
├── Damage Application Engine
│   ├── Shield Order Processor
│   ├── Resource Damage Calculator
│   ├── Status Effect Processor
│   └── Event Generator
├── Shield Management System
│   ├── Shield Stacking Rules
│   ├── Shield Penetration Logic
│   ├── Shield Breaking System
│   └── Shield Priority Manager
├── Resource Protection System
│   ├── Resource Priority Rules
│   ├── Resource Damage Distribution
│   ├── Resource Regeneration Logic
│   └── Resource Depletion Handling
└── Event System
    ├── Damage Applied Events
    ├── Shield Broken Events
    ├── Resource Depleted Events
    └── Status Effect Events
```

## 🛡️ **Shield Order System**

### **1. Shield as Independent Actors**

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

### **2. Subsystem Shield Registration System**

```rust
// Subsystem shield registration system
pub struct ShieldRegistrationSystem {
    registered_shield_types: HashMap<String, ShieldTypeConfig>,
    active_shields: HashMap<String, Vec<ShieldActor>>,
    max_shield_stacks: usize,
    shield_priority_cache: HashMap<String, Vec<ShieldActor>>,
}

#[derive(Debug, Clone)]
pub struct ShieldTypeConfig {
    pub shield_type: ShieldType,
    pub base_priority: i64,
    pub max_hp: f64,
    pub lifetime_decay_rate: f64,
    pub damage_types: Vec<String>,
    pub restoration_events: Vec<String>,
    pub subsystem_id: String,
}

impl ShieldRegistrationSystem {
    /// Register shield type from subsystem
    pub async fn register_shield_type(
        &mut self,
        subsystem_id: &str,
        config: ShieldTypeConfig,
    ) -> ActorCoreResult<()> {
        let key = format!("{}:{}", subsystem_id, config.shield_type);
        self.registered_shield_types.insert(key, config);
        Ok(())
    }
    
    /// Create shield instance
    pub async fn create_shield(
        &mut self,
        owner_actor_id: &str,
        shield_type: &ShieldType,
        subsystem_id: &str,
        custom_config: Option<ShieldCustomConfig>,
    ) -> ActorCoreResult<ShieldActor> {
        // Check shield stack limit
        if self.get_active_shield_count(owner_actor_id) >= self.max_shield_stacks {
            return Err(ActorCoreError::ShieldStackLimitExceeded(
                format!("Actor {} has reached maximum shield stacks: {}", 
                    owner_actor_id, self.max_shield_stacks)
            ));
        }
        
        // Get shield type configuration
        let key = format!("{}:{:?}", subsystem_id, shield_type);
        let config = self.registered_shield_types.get(&key)
            .ok_or_else(|| ActorCoreError::ShieldTypeNotRegistered(key.clone()))?;
        
        // Create shield actor
        let shield = ShieldActor {
            shield_id: generate_shield_id(),
            shield_type: shield_type.clone(),
            owner_actor_id: owner_actor_id.to_string(),
            shield_hp: config.max_hp,
            max_shield_hp: config.max_hp,
            damage_types: config.damage_types.clone(),
            priority: config.base_priority,
            created_at: current_timestamp(),
            expires_at: custom_config.and_then(|c| c.expires_at),
            lifetime_decay_rate: config.lifetime_decay_rate,
            restoration_events: config.restoration_events.clone(),
            is_active: true,
            subsystem_id: subsystem_id.to_string(),
        };
        
        // Add to active shields
        self.active_shields.entry(owner_actor_id.to_string())
            .or_insert_with(Vec::new)
            .push(shield.clone());
        
        // Invalidate priority cache
        self.invalidate_priority_cache(owner_actor_id);
        
        Ok(shield)
    }
    
    /// Get shields sorted by priority for damage type
    pub async fn get_shields_for_damage(
        &mut self,
        actor_id: &str,
        damage_type: &str,
    ) -> ActorCoreResult<Vec<ShieldActor>> {
        // Check cache first
        let cache_key = format!("{}:{}", actor_id, damage_type);
        if let Some(cached) = self.shield_priority_cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        // Get active shields for actor
        let shields = self.active_shields.get(actor_id)
            .cloned()
            .unwrap_or_default();
        
        // Filter by damage type compatibility
        let compatible_shields: Vec<ShieldActor> = shields
            .into_iter()
            .filter(|shield| shield.is_active && shield.can_take_damage_type(damage_type))
            .collect();
        
        // Sort by priority (lower number = damaged first)
        let mut sorted_shields = compatible_shields;
        sorted_shields.sort_by(|a, b| {
            let priority_a = a.calculate_priority(damage_type);
            let priority_b = b.calculate_priority(damage_type);
            
            // Primary sort: priority (lower first)
            match priority_a.cmp(&priority_b) {
                std::cmp::Ordering::Equal => {
                    // Tie-breaker: prefer newer if remaining > 50% else older
                    let remaining_a = a.shield_hp / a.max_shield_hp;
                    let remaining_b = b.shield_hp / b.max_shield_hp;
                    
                    if remaining_a > 0.5 && remaining_b > 0.5 {
                        // Both > 50%, prefer newer (higher created_at)
                        b.created_at.cmp(&a.created_at)
                    } else if remaining_a <= 0.5 && remaining_b <= 0.5 {
                        // Both <= 50%, prefer older (lower created_at)
                        a.created_at.cmp(&b.created_at)
                    } else {
                        // Mixed, prefer higher remaining percentage
                        remaining_b.partial_cmp(&remaining_a).unwrap_or(std::cmp::Ordering::Equal)
                    }
                }
                other => other,
            }
        });
        
        // Cache result
        self.shield_priority_cache.insert(cache_key, sorted_shields.clone());
        
        Ok(sorted_shields)
    }
    
    /// Apply lifetime decay to all shields
    pub async fn apply_lifetime_decay(&mut self, delta_time: f64) -> ActorCoreResult<()> {
        for shields in self.active_shields.values_mut() {
            for shield in shields.iter_mut() {
                shield.apply_lifetime_decay(delta_time);
            }
        }
        
        // Clean up broken shields
        self.cleanup_broken_shields().await?;
        
        Ok(())
    }
    
    /// Handle shield restoration events
    pub async fn handle_restoration_event(
        &mut self,
        actor_id: &str,
        event_type: &str,
        amount: f64,
    ) -> ActorCoreResult<()> {
        if let Some(shields) = self.active_shields.get_mut(actor_id) {
            for shield in shields.iter_mut() {
                if shield.restoration_events.contains(&event_type.to_string()) {
                    shield.restore_hp(amount);
                }
            }
        }
        Ok(())
    }
}
```

### **3. Enhanced Shield Order Processing**

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
    
    /// Get shields sorted by priority (highest first)
    async fn get_shields_sorted_by_priority(
        &self,
        actor_id: &str,
    ) -> ActorCoreResult<Vec<ShieldInfo>> {
        if let Some(actor_shields) = self.shields.get(actor_id) {
            let mut shields = actor_shields.clone();
            shields.sort_by(|a, b| b.calculate_priority().cmp(&a.calculate_priority()));
            Ok(shields)
        } else {
            Ok(Vec::new())
        }
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
        
        // Apply penetration resistance
        let penetration = self.penetration_calculator
            .calculate_penetration(damage_type, &shield.damage_types)
            .await?;
        
        let penetration_factor = 1.0 - (shield.penetration_resistance * (1.0 - penetration));
        shield_damage *= penetration_factor;
        
        // Apply shield type modifiers
        let type_modifier = self.get_shield_type_modifier(&shield.shield_type, damage_type);
        shield_damage *= type_modifier;
        
        Ok(shield_damage)
    }
}
```

### **3. Shield Stacking Rules**

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

## 💔 **Resource Damage Application Logic**

> True Damage Policy
>
> - True damage bypasses shields and secondary resources.
> - Only absolute immunities (explicit invulnerability flags) can negate true damage.
> - This policy is deterministic and applies globally unless a game mode explicitly overrides it.

### **True Damage as Sacrifice (Cost Model)**

- **Sacrifice Costs**: True-damage actions are sacrifice-type. They consume actor resources on use, typically:
  - `hp_cost`: immediate HP cost (cannot be prevented by shields; can kill the user if configured)
  - `lifespan_cost`: permanent or long-term cost; recovery is rare/expensive and uses consumables
- **Availability**: Rare by design; gated by cooldowns, prerequisites, and consumables.
- **Balancing**: The stronger the bypass and multipliers, the higher the sacrifice cost and longer the cooldown.
- **Configuration**: See `configs/true_damage.yaml` and action fields in `configs/action_schemas.yaml`.

### **Resource Exhaustion System**

- **Concept**: Each resource has one or more exhaustion breakpoints. When a resource falls to or below a breakpoint, the actor suffers deterministic penalties until the resource recovers above the threshold.
- **Per-Actor Configuration**: Breakpoints and effects are actor-specific (class/build dependent). See `configs/resource_exhaustion.yaml`.
- **Deterministic Processing**: Exhaustion checks run after resource distribution and before event emission in the same tick.

#### **Exhaustion Effects (Examples)**

- Mage archetype (Mana-centric):
  - `mana <= 10%`: disable shield/buff activations; -40% damage multipliers for magical types; +25% damage taken from magical types; cast time +30%.
  - `mana == 0`: cannot cast mana-cost actions; break active mana-based shields; vulnerable flag for targeted counters.
- Warrior archetype (HP/Stamina-centric):
  - `stamina <= 15%`: cannot use stamina-cost actions; -30% physical multipliers; -20% move speed; block/parry disabled.
  - `stamina == 0`: action lockout (physical); heavy stagger susceptibility; taunt effectiveness -50%.
- Generic (applies to most actors):
  - `health <= 10%`: emergency penalties or bonuses configurable (e.g., enrage or cripple).

#### **Processing Order**

```
Shields → Resource Distribution → Apply Protection → Update Resource Values → Check Exhaustion → Emit Exhaustion/Recovery Events → Apply Status Interactions
```

#### **Events**

- `ResourceExhaustedEvent { actor_id, resource_type, threshold_id, effects_applied, timestamp }`
- `ResourceRecoveredEvent { actor_id, resource_type, threshold_id, effects_removed, timestamp }`

Effects are idempotent and versioned; repeated transitions within a short window are coalesced.

### **1. Resource Priority System**

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
        if !self.can_be_damaged() {
            return 0.0;
        }
        
        // Apply protection factor
        let effective_damage = damage * (1.0 - self.protection_factor);
        
        // Apply damage
        let actual_damage = self.current_value.min(effective_damage);
        self.current_value -= actual_damage;
        
        // Check for depletion effects
        if self.current_value <= 0.0 {
            self.trigger_depletion_effects();
        }
        
        actual_damage
    }
    
    /// Trigger depletion effects
    fn trigger_depletion_effects(&self) {
        // Implementation for depletion effects
        // This would trigger status effects, events, etc.
    }
}
```

### **2. Enhanced Resource Damage Distribution**

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
    pub is_primary: bool,        // Primary resources (Health, LifeForce, Lifespan)
    pub is_secondary: bool,      // Secondary resources (Mana, Qi, etc.)
    pub is_special: bool,        // Special resources (Soul, Essence, etc.)
    pub is_temporary: bool,      // Temporary resources (TemporaryHealth, etc.)
}

impl ResourceDamageDistributor {
    /// Distribute damage across resources with fairness rules
    pub async fn distribute_damage_with_fairness(
        &mut self,
        actor_id: &str,
        damage: &mut DamageResult,
        protection_result: &ProtectionResult,
    ) -> ActorCoreResult<ResourceDamageResult> {
        let mut remaining_damage = damage.final_damage;
        let mut resource_results = Vec::new();
        let mut total_damage_applied = 0.0;
        let mut resources_depleted = Vec::new();
        
        // Get damage impact map for this damage type
        let impact_map = self.get_damage_impact_map(&damage.damage_type).await?;
        
        // Distribution order: Shields → Temporary → Primary → Secondary → Specials
        let distribution_order = self.get_distribution_order().await?;
        
        for resource_category in distribution_order {
            if remaining_damage <= 0.0 {
                break;
            }
            
            // Get resources in this category
            let category_resources = self.get_resources_by_category(actor_id, &resource_category).await?;
            
            // Distribute damage within this category
            let category_result = self.distribute_damage_in_category(
                &category_resources,
                &impact_map,
                remaining_damage,
                protection_result,
                &damage.damage_type
            ).await?;
            
            // Update totals
            remaining_damage -= category_result.total_damage_applied;
            total_damage_applied += category_result.total_damage_applied;
            resource_results.extend(category_result.resource_results);
            resources_depleted.extend(category_result.resources_depleted);
        }
        
        // Clamp negatives to zero
        remaining_damage = remaining_damage.max(0.0);
        
        // Update damage result
        damage.final_damage = remaining_damage;
        damage.resource_damage_applied = total_damage_applied;
        
        Ok(ResourceDamageResult {
            original_damage: damage.final_damage + total_damage_applied,
            final_damage: damage.final_damage,
            total_damage_applied,
            resource_results,
            resources_depleted,
            processing_time: current_timestamp(),
        })
    }
    
    /// Get damage impact map for damage type
    async fn get_damage_impact_map(&self, damage_type: &str) -> ActorCoreResult<&DamageImpactMap> {
        self.damage_impact_maps.get(damage_type)
            .ok_or_else(|| ActorCoreError::DamageTypeNotSupported(damage_type.to_string()))
    }
    
    /// Get distribution order (Shields → Temporary → Primary → Secondary → Specials)
    async fn get_distribution_order(&self) -> ActorCoreResult<Vec<ResourceCategory>> {
        Ok(vec![
            ResourceCategory::Temporary,
            ResourceCategory::Primary,
            ResourceCategory::Secondary,
            ResourceCategory::Special,
        ])
    }
    
    /// Distribute damage within a category
    async fn distribute_damage_in_category(
        &self,
        resources: &[ResourceInfo],
        impact_map: &DamageImpactMap,
        remaining_damage: f64,
        protection_result: &ProtectionResult,
        damage_type: &str,
    ) -> ActorCoreResult<CategoryDistributionResult> {
        let mut category_damage = remaining_damage;
        let mut resource_results = Vec::new();
        let mut total_damage_applied = 0.0;
        let mut resources_depleted = Vec::new();
        
        // Find applicable impact for this category
        let applicable_impacts = impact_map.impact_distribution.iter()
            .filter(|impact| self.resource_matches_category(resources, impact))
            .collect::<Vec<_>>();
        
        if applicable_impacts.is_empty() {
            return Ok(CategoryDistributionResult {
                total_damage_applied: 0.0,
                resource_results,
                resources_depleted,
            });
        }
        
        // Distribute damage based on impact percentages
        for impact in applicable_impacts {
            if category_damage <= 0.0 {
                break;
            }
            
            // Calculate damage for this resource type
            let resource_damage = category_damage * (impact.percentage / 100.0);
            
            // Find matching resources
            let matching_resources = resources.iter()
                .filter(|r| self.resource_matches_impact(r, impact))
                .collect::<Vec<_>>();
            
            // Distribute damage among matching resources
            for resource in matching_resources {
                if resource_damage <= 0.0 {
                    break;
                }
                
                // Apply penetration resistance
                let penetration_factor = 1.0 - impact.penetration_resistance;
                let effective_damage = resource_damage * penetration_factor;
                
                // Apply protection factor
                let protection_factor = protection_result.resource_protections
                    .get(&resource.resource_id)
                    .copied()
                    .unwrap_or(0.0);
                let protected_damage = effective_damage * (1.0 - protection_factor);
                
                // Apply damage to resource
                let actual_damage = resource.apply_damage(protected_damage);
                total_damage_applied += actual_damage;
                
                // Check if resource is depleted
                if resource.current_value <= 0.0 {
                    resources_depleted.push(resource.resource_id.clone());
                }
                
                // Record resource result
                resource_results.push(ResourceDamageResult {
                    resource_id: resource.resource_id.clone(),
                    resource_type: resource.resource_type.clone(),
                    damage_applied: actual_damage,
                    remaining_value: resource.current_value,
                    is_depleted: resource.current_value <= 0.0,
                });
            }
        }
        
        Ok(CategoryDistributionResult {
            total_damage_applied,
            resource_results,
            resources_depleted,
        })
    }
    
    /// Check if resource matches category
    fn resource_matches_category(&self, resources: &[ResourceInfo], impact: &ResourceImpact) -> bool {
        resources.iter().any(|r| self.resource_matches_impact(r, impact))
    }
    
    /// Check if resource matches impact
    fn resource_matches_impact(&self, resource: &ResourceInfo, impact: &ResourceImpact) -> bool {
        resource.resource_type == impact.resource_type
    }
}

#[derive(Debug, Clone)]
pub enum ResourceCategory {
    Temporary,
    Primary,
    Secondary,
    Special,
}

#[derive(Debug, Clone)]
pub struct CategoryDistributionResult {
    pub total_damage_applied: f64,
    pub resource_results: Vec<ResourceDamageResult>,
    pub resources_depleted: Vec<String>,
}
```
    
    /// Get resources sorted by priority (highest first)
    async fn get_resources_sorted_by_priority(
        &self,
        actor_id: &str,
    ) -> ActorCoreResult<Vec<ResourceInfo>> {
        if let Some(actor_resources) = self.resources.get(actor_id) {
            let mut resources = actor_resources.clone();
            resources.sort_by(|a, b| b.calculate_priority().cmp(&a.calculate_priority()));
            Ok(resources)
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Calculate damage to a specific resource
    async fn calculate_resource_damage(
        &self,
        resource: &ResourceInfo,
        incoming_damage: f64,
        damage_type: &str,
    ) -> ActorCoreResult<f64> {
        // Base damage to resource
        let mut resource_damage = incoming_damage;
        
        // Apply damage type modifiers
        let type_modifier = self.get_damage_type_modifier(damage_type, &resource.resource_type);
        resource_damage *= type_modifier;
        
        // Apply protection factor
        let protection_factor = self.protection_calculator
            .calculate_protection(&resource.resource_type, damage_type)
            .await?;
        
        resource_damage *= (1.0 - protection_factor);
        
        Ok(resource_damage)
    }
}
```

### **3. Enhanced Resource Protection System**

```rust
// Enhanced resource protection system with subsystem configuration
pub struct ResourceProtectionSystem {
    protection_rules: ProtectionRules,
    subsystem_protections: HashMap<String, SubsystemProtectionConfig>,
    active_protections: HashMap<String, Vec<ActiveProtection>>,
    protection_processor: ProtectionProcessor,
    regeneration_system: RegenerationSystem,
    depletion_handler: DepletionHandler,
}

#[derive(Debug, Clone)]
pub struct SubsystemProtectionConfig {
    pub subsystem_id: String,
    pub protection_order: i64,           // Lower number = applied first
    pub breakpoint_conditions: Vec<BreakpointCondition>,
    pub protection_conditions: Vec<ProtectionCondition>,
    pub max_protection_factor: f64,      // Max protection this subsystem can provide
    pub stacking_rules: StackingRules,
    pub duration_config: DurationConfig,
}

#[derive(Debug, Clone)]
pub struct BreakpointCondition {
    pub condition_id: String,
    pub condition_type: ConditionType,
    pub threshold: f64,
    pub operator: ComparisonOperator,
    pub protection_modifier: f64,        // How much protection changes at breakpoint
    pub subsystem_id: String,
}

#[derive(Debug, Clone)]
pub struct ProtectionCondition {
    pub condition_id: String,
    pub condition_type: ConditionType,
    pub required_value: f64,
    pub operator: ComparisonOperator,
    pub protection_factor: f64,
    pub subsystem_id: String,
}

#[derive(Debug, Clone)]
pub enum ConditionType {
    HealthPercentage,
    ManaPercentage,
    QiLevel,
    SpiritualEnergy,
    LifeForce,
    Lifespan,
    CustomStat(String),
    ShieldCount,
    StatusEffect(String),
    TimeOfDay,
    CombatState,
}

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqual,
}

#[derive(Debug, Clone)]
pub struct StackingRules {
    pub max_layers: usize,               // Max 2 layers as per requirement
    pub stacking_type: StackingType,
    pub diminishing_factor: f64,         // For multiplicative stacking
}

#[derive(Debug, Clone)]
pub enum StackingType {
    Additive,                            // Simple addition
    Multiplicative,                      // Multiplicative stacking
    Diminishing,                         // Diminishing returns
}

#[derive(Debug, Clone)]
pub struct DurationConfig {
    pub base_duration: u64,              // Base duration in milliseconds
    pub decay_type: DecayType,
    pub decay_rate: f64,                 // Rate of decay per second
    pub min_protection_factor: f64,      // Minimum protection factor (e.g., 0.5 = 50%)
}

#[derive(Debug, Clone)]
pub enum DecayType {
    None,                                // No decay
    Linear,                              // Linear decay to 50%
    Exponential,                         // Exponential decay
    Step,                                // Step-wise decay
}

#[derive(Debug, Clone)]
pub struct ActiveProtection {
    pub protection_id: String,
    pub resource_id: String,
    pub subsystem_id: String,
    pub protection_factor: f64,
    pub max_protection_factor: f64,
    pub created_at: u64,
    pub expires_at: u64,
    pub decay_start: u64,
    pub current_layer: usize,
    pub conditions: Vec<ProtectionCondition>,
    pub is_active: bool,
}

impl ResourceProtectionSystem {
    /// Register subsystem protection configuration
    pub async fn register_subsystem_protection(
        &mut self,
        config: SubsystemProtectionConfig,
    ) -> ActorCoreResult<()> {
        self.subsystem_protections.insert(config.subsystem_id.clone(), config);
        Ok(())
    }
    
    /// Apply protection with subsystem configuration
    pub async fn apply_protection(
        &mut self,
        actor_id: &str,
        resource_id: &str,
        subsystem_id: &str,
        base_protection_factor: f64,
        custom_conditions: Option<Vec<ProtectionCondition>>,
    ) -> ActorCoreResult<()> {
        // Get subsystem configuration
        let subsystem_config = self.subsystem_protections.get(subsystem_id)
            .ok_or_else(|| ActorCoreError::SubsystemNotRegistered(subsystem_id.to_string()))?;
        
        // Check breakpoint conditions
        let breakpoint_modifier = self.evaluate_breakpoint_conditions(
            actor_id, 
            &subsystem_config.breakpoint_conditions
        ).await?;
        
        // Check protection conditions
        let condition_met = self.evaluate_protection_conditions(
            actor_id,
            &subsystem_config.protection_conditions
        ).await?;
        
        if !condition_met {
            return Ok(()); // Conditions not met, no protection applied
        }
        
        // Calculate final protection factor
        let final_protection_factor = (base_protection_factor + breakpoint_modifier)
            .min(subsystem_config.max_protection_factor)
            ; // No global hard cap; immortality via protections is allowed
        
        // Check stacking rules
        let current_protections = self.get_active_protections(actor_id, resource_id).await?;
        let stacking_result = self.apply_stacking_rules(
            &current_protections,
            final_protection_factor,
            &subsystem_config.stacking_rules
        ).await?;
        
        if let Some(new_protection) = stacking_result {
            // Create active protection
            let protection = ActiveProtection {
                protection_id: generate_protection_id(),
                resource_id: resource_id.to_string(),
                subsystem_id: subsystem_id.to_string(),
                protection_factor: new_protection.protection_factor,
                max_protection_factor: subsystem_config.max_protection_factor,
                created_at: current_timestamp(),
                expires_at: current_timestamp() + subsystem_config.duration_config.base_duration,
                decay_start: current_timestamp(),
                current_layer: new_protection.layer,
                conditions: custom_conditions.unwrap_or_default(),
                is_active: true,
            };
            
            // Add to active protections
            self.active_protections.entry(actor_id.to_string())
                .or_insert_with(Vec::new)
                .push(protection);
        }
        
        Ok(())
    }
    
    /// Evaluate breakpoint conditions
    async fn evaluate_breakpoint_conditions(
        &self,
        actor_id: &str,
        conditions: &[BreakpointCondition],
    ) -> ActorCoreResult<f64> {
        let mut total_modifier = 0.0;
        
        for condition in conditions {
            let current_value = self.get_condition_value(actor_id, &condition.condition_type).await?;
            let condition_met = self.evaluate_condition(
                current_value,
                condition.threshold,
                &condition.operator
            );
            
            if condition_met {
                total_modifier += condition.protection_modifier;
            }
        }
        
        Ok(total_modifier)
    }
    
    /// Evaluate protection conditions
    async fn evaluate_protection_conditions(
        &self,
        actor_id: &str,
        conditions: &[ProtectionCondition],
    ) -> ActorCoreResult<bool> {
        for condition in conditions {
            let current_value = self.get_condition_value(actor_id, &condition.condition_type).await?;
            let condition_met = self.evaluate_condition(
                current_value,
                condition.required_value,
                &condition.operator
            );
            
            if !condition_met {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Get condition value for actor
    async fn get_condition_value(
        &self,
        actor_id: &str,
        condition_type: &ConditionType,
    ) -> ActorCoreResult<f64> {
        match condition_type {
            ConditionType::HealthPercentage => {
                // Get health percentage from actor stats
                self.get_resource_percentage(actor_id, "health").await
            }
            ConditionType::ManaPercentage => {
                self.get_resource_percentage(actor_id, "mana").await
            }
            ConditionType::QiLevel => {
                self.get_resource_percentage(actor_id, "qi").await
            }
            ConditionType::SpiritualEnergy => {
                self.get_resource_percentage(actor_id, "spiritual_energy").await
            }
            ConditionType::LifeForce => {
                self.get_resource_percentage(actor_id, "life_force").await
            }
            ConditionType::Lifespan => {
                self.get_resource_percentage(actor_id, "lifespan").await
            }
            ConditionType::CustomStat(stat_name) => {
                self.get_custom_stat_value(actor_id, stat_name).await
            }
            ConditionType::ShieldCount => {
                self.get_shield_count(actor_id).await
            }
            ConditionType::StatusEffect(effect_name) => {
                self.get_status_effect_value(actor_id, effect_name).await
            }
            ConditionType::TimeOfDay => {
                Ok(self.get_time_of_day())
            }
            ConditionType::CombatState => {
                self.get_combat_state_value(actor_id).await
            }
        }
    }
    
    /// Apply stacking rules
    async fn apply_stacking_rules(
        &self,
        current_protections: &[ActiveProtection],
        new_protection_factor: f64,
        stacking_rules: &StackingRules,
    ) -> ActorCoreResult<Option<StackingResult>> {
        if current_protections.len() >= stacking_rules.max_layers {
            return Ok(None); // Max layers reached
        }
        
        let total_protection = match stacking_rules.stacking_type {
            StackingType::Additive => {
                let existing = current_protections.iter()
                    .map(|p| p.protection_factor)
                    .sum::<f64>();
                existing + new_protection_factor
            }
            StackingType::Multiplicative => {
                let existing = current_protections.iter()
                    .map(|p| 1.0 - p.protection_factor)
                    .product::<f64>();
                1.0 - (existing * (1.0 - new_protection_factor))
            }
            StackingType::Diminishing => {
                let existing = current_protections.iter()
                    .map(|p| p.protection_factor)
                    .sum::<f64>();
                existing + (new_protection_factor * stacking_rules.diminishing_factor)
            }
        };
        
        // No global hard cap; final protection constrained only by stacking/subsystem limits
        let final_protection = total_protection;
        
        Ok(Some(StackingResult {
            protection_factor: final_protection,
            layer: current_protections.len() + 1,
        }))
    }
    
    /// Process protection decay
    pub async fn process_protection_decay(
        &mut self,
        actor_id: &str,
        delta_time: f64,
    ) -> ActorCoreResult<()> {
        if let Some(protections) = self.active_protections.get_mut(actor_id) {
            for protection in protections.iter_mut() {
                if !protection.is_active {
                    continue;
                }
                
                // Check if protection has expired
                if current_timestamp() >= protection.expires_at {
                    protection.is_active = false;
                    continue;
                }
                
                // Apply decay
                let decay_elapsed = (current_timestamp() - protection.decay_start) as f64 / 1000.0;
                let decay_amount = protection.max_protection_factor * 0.5 * decay_elapsed; // Decay to 50%
                protection.protection_factor = (protection.max_protection_factor - decay_amount)
                    .max(protection.max_protection_factor * 0.5);
            }
            
            // Clean up expired protections
            protections.retain(|p| p.is_active);
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StackingResult {
    pub protection_factor: f64,
    pub layer: usize,
}
```

## 🎯 **Damage Application Engine**

### **1. Main Damage Application Engine**

```rust
// Main damage application engine
pub struct DamageApplicationEngine {
    shield_processor: Arc<ShieldOrderProcessor>,
    resource_distributor: Arc<ResourceDamageDistributor>,
    protection_system: Arc<ResourceProtectionSystem>,
    event_system: Arc<EventSystem>,
}

impl DamageApplicationEngine {
    /// Apply damage to actor
    pub async fn apply_damage(
        &mut self,
        actor_id: &str,
        damage: &mut DamageResult,
    ) -> ActorCoreResult<DamageApplicationResult> {
        let start_time = current_timestamp();
        
        // 1. Process through shields
        let shield_result = self.shield_processor
            .process_damage_through_shields(actor_id, damage)
            .await?;
        
        // 2. Apply resource protections (after shield resolution, before distribution)
        let protection_result = self.protection_system
            .apply_resource_protections(actor_id, damage)
            .await?;
        
        // 3. Distribute damage to resources (with protection factors applied)
        let resource_result = self.resource_distributor
            .distribute_damage_with_protection(actor_id, damage, &protection_result)
            .await?;
        
        // 4. Handle depletion effects
        self.handle_depletion_effects(actor_id, &resource_result).await?;
        
        // 5. Generate events
        self.generate_damage_events(actor_id, damage, &shield_result, &resource_result, &protection_result).await?;
        
        let processing_time = current_timestamp() - start_time;
        
        Ok(DamageApplicationResult {
            actor_id: actor_id.to_string(),
            original_damage: damage.final_damage + shield_result.total_absorbed + resource_result.total_damage_applied,
            final_damage: damage.final_damage,
            shield_result,
            resource_result,
            protection_result,
            processing_time,
            timestamp: current_timestamp(),
        })
    }
    
    /// Handle depletion effects
    async fn handle_depletion_effects(
        &self,
        actor_id: &str,
        resource_result: &ResourceDamageResult,
    ) -> ActorCoreResult<()> {
        for resource_id in &resource_result.resources_depleted {
            self.protection_system
                .handle_depletion(actor_id, resource_id, &ResourceType::Health)
                .await?;
        }
        Ok(())
    }
    
    /// Generate damage events
    async fn generate_damage_events(
        &self,
        actor_id: &str,
        damage: &DamageResult,
        shield_result: &ShieldProcessingResult,
        resource_result: &ResourceDamageResult,
    ) -> ActorCoreResult<()> {
        // Generate damage applied event
        let damage_event = DamageAppliedEvent {
            actor_id: actor_id.to_string(),
            damage: damage.clone(),
            shield_result: shield_result.clone(),
            resource_result: resource_result.clone(),
            timestamp: current_timestamp(),
        };
        
        self.event_system.publish_event(damage_event).await?;
        
        // Generate shield broken events
        for shield_id in &shield_result.shields_broken {
            let shield_event = ShieldBrokenEvent {
                actor_id: actor_id.to_string(),
                shield_id: shield_id.clone(),
                timestamp: current_timestamp(),
            };
            
            self.event_system.publish_event(shield_event).await?;
        }
        
        // Generate resource depleted events
        for resource_id in &resource_result.resources_depleted {
            let resource_event = ResourceDepletedEvent {
                actor_id: actor_id.to_string(),
                resource_id: resource_id.clone(),
                timestamp: current_timestamp(),
            };
            
            self.event_system.publish_event(resource_event).await?;
        }
        
        Ok(())
    }
}
```

### **2. Event System Integration**

```rust
// Damage application events
#[derive(Debug, Clone)]
pub struct DamageAppliedEvent {
    pub actor_id: String,
    pub damage: DamageResult,
    pub shield_result: ShieldProcessingResult,
    pub resource_result: ResourceDamageResult,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ShieldBrokenEvent {
    pub actor_id: String,
    pub shield_id: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ResourceDepletedEvent {
    pub actor_id: String,
    pub resource_id: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ResourceExhaustedEvent {
    pub actor_id: String,
    pub resource_type: ResourceType,
    pub threshold_id: String,
    pub effects_applied: Vec<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ResourceRecoveredEvent {
    pub actor_id: String,
    pub resource_type: ResourceType,
    pub threshold_id: String,
    pub effects_removed: Vec<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ProtectionAppliedEvent {
    pub actor_id: String,
    pub resource_id: String,
    pub protection_id: String,
    pub subsystem_id: String,
    pub protection_factor: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ProtectionResult {
    pub actor_id: String,
    pub resource_protections: HashMap<String, f64>, // resource_id -> protection_factor
    pub active_protections: Vec<ActiveProtection>,
    pub protection_applied: f64,
    pub processing_time: u64,
    pub timestamp: u64,
}

// Event handlers
impl EventHandler for DamageAppliedEvent {
    fn handle(&self, context: &mut EventContext) -> ActorCoreResult<()> {
        // Handle damage applied event
        // This could trigger status effects, passive abilities, etc.
        Ok(())
    }
}
```

## ⚙️ **Configuration System**

### **1. Shield System Configuration**

```yaml
# Shield system configuration
shield_system:
  max_shield_stacks: 10  # Configurable total shield stacks limit
  priority_cache_ttl: 300000  # 5 minutes
  lifetime_decay_interval: 1000  # 1 second
  
# Subsystem shield registrations
subsystem_shields:
  magic_system:
    fire_shield:
      base_priority: 100
      max_hp: 1000
      lifetime_decay_rate: 10.0  # 10 HP per second
      damage_types: ["fire", "magical"]
      restoration_events: ["fire_restoration", "magic_restoration"]
    
    ice_shield:
      base_priority: 150
      max_hp: 800
      lifetime_decay_rate: 8.0
      damage_types: ["ice", "magical"]
      restoration_events: ["ice_restoration", "magic_restoration"]
  
  jindan_system:
    qi_shield:
      base_priority: 200
      max_hp: 1500
      lifetime_decay_rate: 5.0
      damage_types: ["qi", "spiritual"]
      restoration_events: ["qi_restoration", "meditation"]
    
    spiritual_shield:
      base_priority: 250
      max_hp: 1200
      lifetime_decay_rate: 7.0
      damage_types: ["spiritual", "soul"]
      restoration_events: ["spiritual_restoration", "enlightenment"]
  
  rpg_system:
    physical_shield:
      base_priority: 300
      max_hp: 2000
      lifetime_decay_rate: 3.0
      damage_types: ["physical", "slashing", "piercing"]
      restoration_events: ["physical_restoration", "rest"]
    
    armor_shield:
      base_priority: 400
      max_hp: 3000
      lifetime_decay_rate: 2.0
      damage_types: ["physical"]
      restoration_events: ["armor_repair", "rest"]
```

### **2. Shield Priority Formula**

```rust
// Deterministic priority formula
pub fn calculate_priority(&self, damage_type: &str) -> i64 {
    // Base priority from subsystem registration (lower = damaged first)
    let base_priority = self.priority;
    
    // Type-specific modifier for damage type
    let type_modifier = self.get_damage_type_modifier(damage_type);
    
    // Remaining HP percentage modifier
    let hp_percentage = (self.shield_hp / self.max_shield_hp * 100.0) as i64;
    
    // Formula: base + floor(remaining/max × 100) + type_modifier
    base_priority + hp_percentage + type_modifier
}

// Tie-breaker rules
// 1. Lower priority number = damaged first
// 2. If same priority: prefer newer if remaining > 50% else older
// 3. Final tiebreaker: by shield_id
```

### **3. Damage Impact Maps Configuration**

```yaml
# Default damage impact maps (percent of residual after shields before protections)
damage_impact_maps:
  physical:
    damage_type: "physical"
    bypass_secondary: false
    impact_distribution:
      - resource_type: "Health"
        percentage: 100.0
        penetration_resistance: 0.0
        is_primary: true
        is_secondary: false
        is_special: false
        is_temporary: false
  
  magical:
    damage_type: "magical"
    bypass_secondary: false
    impact_distribution:
      - resource_type: "Health"
        percentage: 80.0
        penetration_resistance: 0.0
        is_primary: true
        is_secondary: false
        is_special: false
        is_temporary: false
      - resource_type: "Mana"
        percentage: 20.0
        penetration_resistance: 0.2
        is_primary: false
        is_secondary: true
        is_special: false
        is_temporary: false
  
  fire:
    damage_type: "fire"
    bypass_secondary: false
    impact_distribution:
      - resource_type: "Health"
        percentage: 85.0
        penetration_resistance: 0.0
        is_primary: true
        is_secondary: false
        is_special: false
        is_temporary: false
      - resource_type: "FireStorage"
        percentage: 15.0
        penetration_resistance: 0.1
        is_primary: false
        is_secondary: true
        is_special: false
        is_temporary: false
  
  ice:
    damage_type: "ice"
    bypass_secondary: false
    impact_distribution:
      - resource_type: "Health"
        percentage: 85.0
        penetration_resistance: 0.0
        is_primary: true
        is_secondary: false
        is_special: false
        is_temporary: false
      - resource_type: "IceStorage"
        percentage: 15.0
        penetration_resistance: 0.1
        is_primary: false
        is_secondary: true
        is_special: false
        is_temporary: false
  
  qi:
    damage_type: "qi"
    bypass_secondary: false
    impact_distribution:
      - resource_type: "Health"
        percentage: 70.0
        penetration_resistance: 0.0
        is_primary: true
        is_secondary: false
        is_special: false
        is_temporary: false
      - resource_type: "Qi"
        percentage: 30.0
        penetration_resistance: 0.3
        is_primary: false
        is_secondary: true
        is_special: false
        is_temporary: false
  
  spiritual:
    damage_type: "spiritual"
    bypass_secondary: false
    impact_distribution:
      - resource_type: "Health"
        percentage: 70.0
        penetration_resistance: 0.0
        is_primary: true
        is_secondary: false
        is_special: false
        is_temporary: false
      - resource_type: "SpiritualEnergy"
        percentage: 30.0
        penetration_resistance: 0.3
        is_primary: false
        is_secondary: true
        is_special: false
        is_temporary: false
  
  true:
    damage_type: "true"
    bypass_secondary: true
    impact_distribution:
      - resource_type: "Health"
        percentage: 100.0
        penetration_resistance: 0.0
        is_primary: true
        is_secondary: false
        is_special: false
        is_temporary: false

# Distribution order configuration
distribution_order:
  - "Temporary"    # Temporary resources first
  - "Primary"      # Primary resources (Health, LifeForce, Lifespan)
  - "Secondary"    # Secondary resources (Mana, Qi, SpiritualEnergy)
  - "Special"      # Special resources (Soul, Essence, etc.)

# Resource categories
resource_categories:
  primary:
    - "Health"
    - "LifeForce"
    - "Lifespan"
  secondary:
    - "Mana"
    - "Qi"
    - "SpiritualEnergy"
    - "Stamina"
    - "FireStorage"
    - "IceStorage"
    - "LightningStorage"
    - "EarthStorage"
    - "WindStorage"
    - "WaterStorage"
  special:
    - "Soul"
    - "Essence"
    - "Vitality"
  temporary:
    - "TemporaryHealth"
    - "TemporaryMana"
    - "TemporaryQi"
```

### **4. Resource Protection Configuration**

```yaml
# Resource protection system configuration
resource_protection:
  # Protected resources (always honor protection)
  always_protected:
    - "health"
    - "life_force"
    - "lifespan"
  
  # Configurable protected resources
  configurable_protected:
    - "mana"
    - "qi"
    - "spiritual_energy"
    - "soul"
  
  # Protection limits
  protection_limits:
    hard_cap: null  # No global hard cap (immortality allowed)
    max_layers: 2    # Maximum 2 protection layers
    stacking_type: "multiplicative"  # Multiplicative stacking
  
  # Duration and decay
  duration_config:
    base_duration: 300000  # 5 minutes base duration
    decay_type: "linear"   # Linear decay
    decay_rate: 0.1        # 10% decay per second
    min_protection_factor: 0.5  # Decay to 50% minimum

# Subsystem protection configurations
subsystem_protections:
  magic_system:
    protection_order: 100
    max_protection_factor: 0.60
    stacking_rules:
      max_layers: 2
      stacking_type: "multiplicative"
      diminishing_factor: 0.8
    breakpoint_conditions:
      - condition_id: "low_health"
        condition_type: "HealthPercentage"
        threshold: 25.0
        operator: "LessThan"
        protection_modifier: 0.20  # +20% protection when health < 25%
      - condition_id: "high_mana"
        condition_type: "ManaPercentage"
        threshold: 80.0
        operator: "GreaterThan"
        protection_modifier: 0.15  # +15% protection when mana > 80%
    protection_conditions:
      - condition_id: "has_mana"
        condition_type: "ManaPercentage"
        required_value: 10.0
        operator: "GreaterThan"
        protection_factor: 0.30
    duration_config:
      base_duration: 180000  # 3 minutes
      decay_type: "linear"
      decay_rate: 0.05
      min_protection_factor: 0.3

  jindan_system:
    protection_order: 200
    max_protection_factor: 0.70
    stacking_rules:
      max_layers: 2
      stacking_type: "diminishing"
      diminishing_factor: 0.6
    breakpoint_conditions:
      - condition_id: "low_qi"
        condition_type: "QiLevel"
        threshold: 20.0
        operator: "LessThan"
        protection_modifier: 0.25  # +25% protection when qi < 20%
      - condition_id: "high_spiritual_energy"
        condition_type: "SpiritualEnergy"
        threshold: 90.0
        operator: "GreaterThan"
        protection_modifier: 0.20  # +20% protection when spiritual energy > 90%
    protection_conditions:
      - condition_id: "has_qi"
        condition_type: "QiLevel"
        required_value: 5.0
        operator: "GreaterThan"
        protection_factor: 0.40
      - condition_id: "meditation_state"
        condition_type: "StatusEffect"
        required_value: 1.0
        operator: "Equal"
        protection_factor: 0.50
    duration_config:
      base_duration: 600000  # 10 minutes
      decay_type: "exponential"
      decay_rate: 0.02
      min_protection_factor: 0.4

  rpg_system:
    protection_order: 300
    max_protection_factor: 0.50
    stacking_rules:
      max_layers: 2
      stacking_type: "additive"
      diminishing_factor: 1.0
    breakpoint_conditions:
      - condition_id: "low_health"
        condition_type: "HealthPercentage"
        threshold: 30.0
        operator: "LessThan"
        protection_modifier: 0.30  # +30% protection when health < 30%
      - condition_id: "has_shields"
        condition_type: "ShieldCount"
        threshold: 3.0
        operator: "GreaterThanOrEqual"
        protection_modifier: 0.15  # +15% protection when 3+ shields active
    protection_conditions:
      - condition_id: "in_combat"
        condition_type: "CombatState"
        required_value: 1.0
        operator: "Equal"
        protection_factor: 0.25
    duration_config:
      base_duration: 120000  # 2 minutes
      decay_type: "step"
      decay_rate: 0.1
      min_protection_factor: 0.2
```

## ⚖️ **Damage Distribution Fairness Rules**

### **1. Distribution Order**
```
Shields → Temporary Resources → Primary Resources → Secondary Resources → Special Resources
```

### **2. Default Impact Maps**
- **Physical**: Health 100%
- **Magical**: Health 80%, Mana 20%
- **Elemental**: Health 85%, Matching-element storage 15% (if present)
- **Qi/Spiritual**: Health 70%, Qi/SpiritualEnergy 30%
- **True**: Health 100% (bypasses secondary)
 - **True**: Health 100% (bypasses shields and secondary; absolute immunities only)

### **3. Fairness Principles**
- **Deterministic Order**: Always process in the same order
- **Clamp Negatives**: All negative values clamped to zero
- **Penetration/Resistance**: Applied per resource type
- **Weight Order**: Deterministic weight order for equal percentages
- **Protection Application**: Applied after shield resolution, before distribution

### **4. Resource Categories**
- **Primary**: Health, LifeForce, Lifespan (always protected)
- **Secondary**: Mana, Qi, SpiritualEnergy, ElementalStorage, Guard, Stagger (configurable protection)
- **Special**: Soul, Essence, Vitality (rare protection)
- **Temporary**: TemporaryHealth, TemporaryMana, TemporaryQi (no protection)

### **5. Example Distribution Scenarios**

#### **Physical Damage (1000 damage)**
```
Shields: 200 absorbed
Remaining: 800
→ Health: 800 (100%)
Final: 0 remaining damage
```

#### **Magical Damage (1000 damage)**
```
Shields: 150 absorbed
Remaining: 850
→ Health: 680 (80% of 850)
→ Mana: 170 (20% of 850)
Final: 0 remaining damage
```

#### **Physical with Guard (1000 damage)**
```
Shields: 150 absorbed
Remaining: 850
→ Guard: 255 (30% of 850)
→ Health: 595 (70% of 850)
Final: 0 remaining damage
```

#### **Fire Elemental Damage (1000 damage)**
```
Shields: 100 absorbed
Remaining: 900
→ Health: 765 (85% of 900)
→ FireStorage: 135 (15% of 900)
Final: 0 remaining damage
```

#### **Qi Damage (1000 damage)**
```
Shields: 50 absorbed
Remaining: 950
→ Health: 665 (70% of 950)
→ Qi: 285 (30% of 950)
Final: 0 remaining damage
```

#### **True Damage (1000 damage)**
```
Shields: 0 absorbed (bypasses shields)
Remaining: 1000
→ Health: 1000 (100%, bypasses secondary)
Absolute immunities: if present, reduce to 0
Final: 0 remaining damage (unless absolute immunity applies)
```

## 📊 **Performance vs Complexity**

### **1. Data Layout Optimization**

```rust
// Per-actor fixed-size vectors for shields/resources; versioned snapshots; no allocs on hot path
pub struct ActorCombatState {
    // Fixed-size vectors to avoid allocations on hot path
    shields: [Option<ShieldActor>; MAX_SHIELDS_PER_ACTOR],  // Fixed array
    resources: [Option<ResourceInfo>; MAX_RESOURCES_PER_ACTOR],  // Fixed array
    
    // Versioned snapshots for consistency
    version: u64,
    last_updated: u64,
    
    // Pre-computed sorted indices
    shield_priority_indices: [usize; MAX_SHIELDS_PER_ACTOR],
    resource_priority_indices: [usize; MAX_RESOURCES_PER_ACTOR],
    
    // Cache type modifiers in small arrays
    damage_type_modifiers: [f64; MAX_DAMAGE_TYPES],
    penetration_resistances: [f64; MAX_RESOURCE_TYPES],
}

const MAX_SHIELDS_PER_ACTOR: usize = 32;
const MAX_RESOURCES_PER_ACTOR: usize = 64;
const MAX_DAMAGE_TYPES: usize = 16;
const MAX_RESOURCE_TYPES: usize = 32;

impl ActorCombatState {
    /// Pre-compute sorted shield lists and resource priority lists
    pub fn precompute_priorities(&mut self) {
        // Sort shields by priority (no allocation)
        self.shield_priority_indices.sort_by(|a, b| {
            let shield_a = &self.shields[*a];
            let shield_b = &self.shields[*b];
            shield_a.priority.cmp(&shield_b.priority)
        });
        
        // Sort resources by priority (no allocation)
        self.resource_priority_indices.sort_by(|a, b| {
            let resource_a = &self.resources[*a];
            let resource_b = &self.resources[*b];
            resource_a.priority.cmp(&resource_b.priority)
        });
    }
    
    /// Cache type modifiers in small arrays
    pub fn cache_type_modifiers(&mut self, damage_type: &str) {
        let damage_type_index = self.get_damage_type_index(damage_type);
        self.damage_type_modifiers[damage_type_index] = self.calculate_damage_modifier(damage_type);
    }
}
```

### **2. Hot Path Math Optimization**

```rust
// Hot path math: doubles for floats, int64 for counters/timestamps; branch-light; avoid dynamic dispatch
pub struct HotPathCalculator {
    // Use doubles for all float calculations
    damage_multiplier: f64,
    penetration_factor: f64,
    protection_factor: f64,
    
    // Use int64 for counters and timestamps
    damage_count: i64,
    last_calculation_time: i64,
    
    // Branch-light calculations
    damage_result: f64,
}

impl HotPathCalculator {
    /// Branch-light damage calculation
    #[inline(always)]
    pub fn calculate_damage_fast(
        &mut self,
        base_damage: f64,
        damage_type: &str,
        target_shields: &[ShieldActor],
        target_resources: &[ResourceInfo],
    ) -> f64 {
        // Avoid dynamic dispatch - use direct function calls
        self.damage_multiplier = self.get_damage_multiplier_fast(damage_type);
        self.penetration_factor = self.get_penetration_factor_fast(damage_type);
        self.protection_factor = self.get_protection_factor_fast(target_resources);
        
        // Branch-light calculation
        self.damage_result = base_damage * self.damage_multiplier * self.penetration_factor * (1.0 - self.protection_factor);
        
        // Clamp to zero (single branch)
        if self.damage_result < 0.0 {
            self.damage_result = 0.0;
        }
        
        self.damage_result
    }
    
    /// Get damage multiplier without dynamic dispatch
    #[inline(always)]
    fn get_damage_multiplier_fast(&self, damage_type: &str) -> f64 {
        match damage_type {
            "physical" => 1.0,
            "magical" => 0.8,
            "fire" => 0.85,
            "ice" => 0.85,
            "qi" => 0.7,
            "spiritual" => 0.7,
            "true" => 1.0,
            _ => 1.0,
        }
    }
}
```

### **3. Latency Budget Management**

```rust
// Latency budget: 0.05 ms target per application; profile p95/p99; back off rare features that exceed budget
pub struct LatencyBudgetManager {
    target_latency_ms: f64,  // 0.05 ms target
    p95_latency_ms: f64,
    p99_latency_ms: f64,
    current_latency_ms: f64,
    backoff_threshold: f64,
}

impl LatencyBudgetManager {
    const TARGET_LATENCY_MS: f64 = 0.05;
    const BACKOFF_THRESHOLD_MS: f64 = 0.1;  // 2x target
    
    pub fn new() -> Self {
        Self {
            target_latency_ms: Self::TARGET_LATENCY_MS,
            p95_latency_ms: 0.0,
            p99_latency_ms: 0.0,
            current_latency_ms: 0.0,
            backoff_threshold: Self::BACKOFF_THRESHOLD_MS,
        }
    }
    
    /// Check if we should back off rare features
    pub fn should_backoff_rare_features(&self) -> bool {
        self.current_latency_ms > self.backoff_threshold
    }
    
    /// Profile p95/p99 latencies
    pub fn update_latency_profile(&mut self, new_latency_ms: f64) {
        self.current_latency_ms = new_latency_ms;
        
        // Update p95/p99 (simplified)
        if new_latency_ms > self.p95_latency_ms {
            self.p95_latency_ms = new_latency_ms;
        }
        if new_latency_ms > self.p99_latency_ms {
            self.p99_latency_ms = new_latency_ms;
        }
    }
}
```

## 🔄 **Event System Integration**

### **1. Minimal Events Design**

```rust
// Minimal events: DamageApplied, ShieldBroken, ResourceDepleted (already defined)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatEvent {
    pub event_type: CombatEventType,
    pub actor_id: String,
    pub version: u64,
    pub timestamp: u64,
    pub data: EventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatEventType {
    DamageApplied,
    ShieldBroken,
    ResourceDepleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventData {
    DamageApplied {
        damage_amount: f64,
        damage_type: String,
        source_actor_id: String,
    },
    ShieldBroken {
        shield_id: String,
        shield_type: String,
        remaining_hp: f64,
    },
    ResourceDepleted {
        resource_id: String,
        resource_type: String,
        previous_value: f64,
    },
}
```

### **2. Event Throttling and Coalescing**

```rust
// Throttling: coalesce duplicate ShieldBroken within N ms; cap DamageApplied per actor per tick
pub struct EventThrottler {
    shield_broken_coalesce_ms: u64,
    damage_applied_cap_per_actor: usize,
    damage_applied_cap_per_tick: usize,
    current_tick_events: HashMap<String, usize>,
    coalesce_windows: HashMap<String, u64>,
}

impl EventThrottler {
    const SHIELD_BROKEN_COALESCE_MS: u64 = 100;  // 100ms coalesce window
    const DAMAGE_APPLIED_CAP_PER_ACTOR: usize = 10;
    const DAMAGE_APPLIED_CAP_PER_TICK: usize = 1000;
    
    /// Check if event should be throttled
    pub fn should_throttle_event(&mut self, event: &CombatEvent) -> bool {
        match event.event_type {
            CombatEventType::ShieldBroken => {
                // Coalesce duplicate ShieldBroken within N ms
                let key = format!("{}_{}", event.actor_id, event.data.shield_id());
                let now = current_timestamp();
                let last_emitted = self.coalesce_windows.get(&key).copied().unwrap_or(0);
                
                if now - last_emitted < Self::SHIELD_BROKEN_COALESCE_MS {
                    return true;  // Throttle duplicate
                }
                
                self.coalesce_windows.insert(key, now);
                false
            },
            CombatEventType::DamageApplied => {
                // Cap DamageApplied per actor per tick
                let actor_count = self.current_tick_events.get(&event.actor_id).copied().unwrap_or(0);
                if actor_count >= Self::DAMAGE_APPLIED_CAP_PER_ACTOR {
                    return true;  // Throttle per actor
                }
                
                let total_count: usize = self.current_tick_events.values().sum();
                if total_count >= Self::DAMAGE_APPLIED_CAP_PER_TICK {
                    return true;  // Throttle per tick
                }
                
                self.current_tick_events.insert(event.actor_id.clone(), actor_count + 1);
                false
            },
            _ => false,  // No throttling for other events
        }
    }
}
```

### **3. Event Ordering and Idempotency**

```rust
// Ordering: emit in application order; idempotency key = (actor_id, event_type, version)
pub struct EventOrderer {
    event_queue: VecDeque<CombatEvent>,
    idempotency_keys: HashSet<String>,
}

impl EventOrderer {
    /// Add event with idempotency check
    pub fn add_event(&mut self, event: CombatEvent) -> bool {
        let idempotency_key = format!("{}_{}_{}", event.actor_id, event.event_type, event.version);
        
        if self.idempotency_keys.contains(&idempotency_key) {
            return false;  // Duplicate event
        }
        
        self.idempotency_keys.insert(idempotency_key);
        self.event_queue.push_back(event);
        true
    }
    
    /// Process events in application order
    pub fn process_events(&mut self) -> Vec<CombatEvent> {
        let mut processed_events = Vec::new();
        
        while let Some(event) = self.event_queue.pop_front() {
            processed_events.push(event);
        }
        
        processed_events
    }
}
```

### **4. Async Publishing with Bounded Queue**

```rust
// Async publishing with bounded queue; drop-or-sample policy under backpressure
pub struct AsyncEventPublisher {
    event_queue: Arc<Mutex<VecDeque<CombatEvent>>>,
    max_queue_size: usize,
    drop_policy: DropPolicy,
    sample_rate: f64,
}

#[derive(Debug, Clone)]
pub enum DropPolicy {
    DropOldest,
    DropNewest,
    SampleRandom,
}

impl AsyncEventPublisher {
    const MAX_QUEUE_SIZE: usize = 10000;
    const SAMPLE_RATE: f64 = 0.1;  // 10% sampling under backpressure
    
    /// Publish event with backpressure handling
    pub async fn publish_event(&self, event: CombatEvent) -> bool {
        let mut queue = self.event_queue.lock().await;
        
        if queue.len() >= self.max_queue_size {
            // Handle backpressure
            match self.drop_policy {
                DropPolicy::DropOldest => {
                    queue.pop_front();
                    queue.push_back(event);
                    true
                },
                DropPolicy::DropNewest => {
                    false  // Drop the new event
                },
                DropPolicy::SampleRandom => {
                    if rand::random::<f64>() < self.sample_rate {
                        queue.push_back(event);
                        true
                    } else {
                        false  // Drop due to sampling
                    }
                },
            }
        } else {
            queue.push_back(event);
            true
        }
    }
}
```

## 🔄 **Batch Processing Necessity**

### **1. Micro-batch Processing**

```rust
// Yes; micro-batch per tick: group (actor_id, DamageResult) by target and apply sequentially per target
pub struct BatchProcessor {
    batch_size: usize,
    max_workers: usize,
    target_groups: HashMap<String, Vec<DamageResult>>,
    worker_pool: ThreadPool,
}

impl BatchProcessor {
    const BATCH_SIZE: usize = 256;
    const MAX_WORKERS: usize = 8;
    
    /// Process damage in micro-batches
    pub async fn process_damage_batch(
        &mut self,
        damage_results: Vec<(String, DamageResult)>,
    ) -> ActorCoreResult<Vec<DamageApplicationResult>> {
        // Group by target actor
        self.target_groups.clear();
        for (actor_id, damage_result) in damage_results {
            self.target_groups.entry(actor_id).or_insert_with(Vec::new).push(damage_result);
        }
        
        // Process targets in parallel
        let mut handles = Vec::new();
        for (target_id, damages) in self.target_groups.drain() {
            let handle = self.worker_pool.spawn(async move {
                self.process_target_damages(target_id, damages).await
            });
            handles.push(handle);
        }
        
        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            let target_results = handle.await?;
            results.extend(target_results);
        }
        
        Ok(results)
    }
    
    /// Process damages for a single target sequentially
    async fn process_target_damages(
        &self,
        target_id: String,
        damages: Vec<DamageResult>,
    ) -> ActorCoreResult<Vec<DamageApplicationResult>> {
        let mut results = Vec::new();
        
        // Apply damages sequentially per target
        for damage in damages {
            let result = self.apply_damage_to_target(&target_id, damage).await?;
            results.push(result);
        }
        
        Ok(results)
    }
}
```

### **2. Fair Scheduling and Sharding**

```rust
// Limits: batch_size ≤ 256 per worker; shard by target_id hash; use fair scheduling to avoid starve
pub struct FairScheduler {
    worker_count: usize,
    worker_queues: Vec<VecDeque<(String, DamageResult)>>,
    worker_loads: Vec<usize>,
    shard_hash: fn(&str) -> usize,
}

impl FairScheduler {
    /// Shard by target_id hash
    fn shard_target(&self, target_id: &str) -> usize {
        (self.shard_hash)(target_id) % self.worker_count
    }
    
    /// Fair scheduling to avoid starvation
    pub fn schedule_damage(&mut self, target_id: String, damage: DamageResult) -> usize {
        let shard = self.shard_target(&target_id);
        
        // Check if worker is overloaded
        if self.worker_loads[shard] >= Self::BATCH_SIZE {
            // Find least loaded worker
            let least_loaded = self.worker_loads.iter().enumerate()
                .min_by_key(|(_, &load)| load)
                .map(|(idx, _)| idx)
                .unwrap_or(shard);
            
            self.worker_queues[least_loaded].push_back((target_id, damage));
            self.worker_loads[least_loaded] += 1;
            least_loaded
        } else {
            self.worker_queues[shard].push_back((target_id, damage));
            self.worker_loads[shard] += 1;
            shard
        }
    }
}
```

## 💾 **Caching Strategy**

### **1. L1 Cache (Per-Process)**

```rust
// L1 (per-process): actor-local shield/resource state snapshot + sorted indices; invalidate on state change or timer
pub struct L1CombatCache {
    actor_states: HashMap<String, ActorCombatState>,
    sorted_shield_indices: HashMap<String, Vec<usize>>,
    sorted_resource_indices: HashMap<String, Vec<usize>>,
    cache_ttl_ms: u64,
    last_invalidation: u64,
}

impl L1CombatCache {
    const CACHE_TTL_MS: u64 = 1000;  // 1 second TTL
    
    /// Get cached actor state
    pub fn get_actor_state(&self, actor_id: &str) -> Option<&ActorCombatState> {
        self.actor_states.get(actor_id)
    }
    
    /// Invalidate cache on state change
    pub fn invalidate_actor(&mut self, actor_id: &str) {
        self.actor_states.remove(actor_id);
        self.sorted_shield_indices.remove(actor_id);
        self.sorted_resource_indices.remove(actor_id);
    }
    
    /// Check if cache needs refresh
    pub fn needs_refresh(&self) -> bool {
        let now = current_timestamp();
        now - self.last_invalidation > self.cache_ttl_ms
    }
}
```

### **2. Result Cache (Optional)**

```rust
// Result cache (optional): hash of (damage_type, multipliers bucket, pen bucket) → partial outcomes
pub struct ResultCache {
    cache: HashMap<u64, CachedResult>,
    ttl_ms: u64,
    max_size: usize,
}

#[derive(Debug, Clone)]
pub struct CachedResult {
    pub damage_multiplier: f64,
    pub penetration_factor: f64,
    pub protection_factor: f64,
    pub cached_at: u64,
}

impl ResultCache {
    const TTL_MS: u64 = 100;  // 100ms TTL
    const MAX_SIZE: usize = 1000;
    
    /// Get cached result
    pub fn get_cached_result(&self, key: u64) -> Option<&CachedResult> {
        self.cache.get(&key).and_then(|result| {
            if current_timestamp() - result.cached_at < self.ttl_ms {
                Some(result)
            } else {
                None
            }
        })
    }
    
    /// Cache result
    pub fn cache_result(&mut self, key: u64, result: CachedResult) {
        if self.cache.len() >= self.max_size {
            // Remove oldest entry
            let oldest_key = self.cache.keys().min().copied();
            if let Some(oldest) = oldest_key {
                self.cache.remove(&oldest);
            }
        }
        
        self.cache.insert(key, result);
    }
}
```

## ⚡ **Depletion Effects Handling**

### **1. Depletion Trigger System**

```rust
// Trigger once when value crosses ≤ 0 with monotonic version; queue depletion effects out-of-band
pub struct DepletionEffectHandler {
    depletion_queue: VecDeque<DepletionEvent>,
    cooldowns: HashMap<String, u64>,
    processing_version: u64,
}

#[derive(Debug, Clone)]
pub struct DepletionEvent {
    pub actor_id: String,
    pub resource_id: String,
    pub resource_type: String,
    pub previous_value: f64,
    pub current_value: f64,
    pub version: u64,
    pub timestamp: u64,
}

impl DepletionEffectHandler {
    const DEPLETION_COOLDOWN_MS: u64 = 300;  // 300ms cooldown
    
    /// Check if resource is depleted
    pub fn check_depletion(
        &mut self,
        actor_id: &str,
        resource_id: &str,
        resource_type: &str,
        previous_value: f64,
        current_value: f64,
        version: u64,
    ) -> bool {
        // Check if value crossed ≤ 0
        if previous_value > 0.0 && current_value <= 0.0 {
            // Check cooldown
            let cooldown_key = format!("{}_{}", actor_id, resource_id);
            let now = current_timestamp();
            let last_depletion = self.cooldowns.get(&cooldown_key).copied().unwrap_or(0);
            
            if now - last_depletion >= Self::DEPLETION_COOLDOWN_MS {
                // Queue depletion effect
                self.depletion_queue.push_back(DepletionEvent {
                    actor_id: actor_id.to_string(),
                    resource_id: resource_id.to_string(),
                    resource_type: resource_type.to_string(),
                    previous_value,
                    current_value,
                    version,
                    timestamp: now,
                });
                
                // Update cooldown
                self.cooldowns.insert(cooldown_key, now);
                return true;
            }
        }
        
        false
    }
}
```

### **2. Depletion Processing Order**

```rust
// Ordering: process depletion after all distributions; publish ResourceDepleted; protection may schedule temporary buffs after
impl DepletionEffectHandler {
    /// Process all depletion effects
    pub async fn process_depletion_effects(&mut self) -> ActorCoreResult<Vec<CombatEvent>> {
        let mut events = Vec::new();
        
        // Process depletion after all distributions
        while let Some(depletion_event) = self.depletion_queue.pop_front() {
            // Publish ResourceDepleted event
            let event = CombatEvent {
                event_type: CombatEventType::ResourceDepleted,
                actor_id: depletion_event.actor_id.clone(),
                version: depletion_event.version,
                timestamp: depletion_event.timestamp,
                data: EventData::ResourceDepleted {
                    resource_id: depletion_event.resource_id.clone(),
                    resource_type: depletion_event.resource_type.clone(),
                    previous_value: depletion_event.previous_value,
                },
            };
            
            events.push(event);
            
            // Protection may schedule temporary buffs after depletion
            self.schedule_temporary_buffs(&depletion_event).await?;
        }
        
        Ok(events)
    }
    
    /// Schedule temporary buffs after depletion
    async fn schedule_temporary_buffs(&self, depletion_event: &DepletionEvent) -> ActorCoreResult<()> {
        // Check if protection should schedule temporary buffs
        if self.should_schedule_buffs(depletion_event) {
            // Schedule temporary buffs (implementation depends on buff system)
            self.schedule_protection_buffs(depletion_event).await?;
        }
        
        Ok(())
    }
}
```

### **3. Loop Prevention**

```rust
// Guard against loops: depletion effects cannot immediately re-trigger themselves in same tick
impl DepletionEffectHandler {
    /// Check if depletion effect should be processed
    pub fn should_process_depletion(&self, event: &DepletionEvent) -> bool {
        // Check if this is the same tick
        if event.version == self.processing_version {
            return false;  // Skip same tick
        }
        
        // Check if resource is still depleted
        // (This would require checking current resource state)
        true
    }
    
    /// Process depletion with loop prevention
    pub async fn process_depletion_with_loop_prevention(
        &mut self,
        event: DepletionEvent,
    ) -> ActorCoreResult<Option<CombatEvent>> {
        if !self.should_process_depletion(&event) {
            return Ok(None);
        }
        
        // Update processing version
        self.processing_version = event.version;
        
        // Process depletion effect
        let event = CombatEvent {
            event_type: CombatEventType::ResourceDepleted,
            actor_id: event.actor_id,
            version: event.version,
            timestamp: event.timestamp,
            data: EventData::ResourceDepleted {
                resource_id: event.resource_id,
                resource_type: event.resource_type,
                previous_value: event.previous_value,
            },
        };
        
        Ok(Some(event))
    }
}
```

## 🔥 **Simple Combat Example: Fire Punch vs Fire Shields**

### **Scenario Setup**
- **Actor A (Attacker)**: Fire Cultivator with high Fire Attack stats
- **Actor B (Target)**: Fire Cultivator with Fire Defense stats and 10 Fire Shield stacks
- **Attack**: Fire Punch (Physical + Fire damage + Burning effect)
- **Shields**: Fire Shield stacks (absorbs fire damage)

### **Actor A Stats (Fire Cultivator)**
```rust
// Actor A - Fire Cultivator (High Fire Attack)
let actor_a_stats = ActorStats {
    // Primary Stats
    strength: 150,        // Scales physical damage
    fire_attack: 200,     // Scales fire damage
    fire_crit_chance: 0.25,  // 25% crit chance
    fire_crit_damage: 2.5,   // 250% crit damage
    
    // Movement & Attack Stats
    movement_speed: 8.5,      // 8.5 units per second
    attack_speed: 1.2,        // 1.2 attacks per second
    accuracy: 85,             // 85% base accuracy
    
    // Fire Punch Base Damage
    fire_punch_physical: 100.0,  // Base physical damage
    fire_punch_fire: 80.0,       // Base fire damage
    
    // Attack Range
    fire_punch_min_range: 1.0,   // Minimum range
    fire_punch_max_range: 3.0,   // Maximum range
    
    // Status Effect Chance
    leg_bone_break_chance: 0.25, // 25% chance to break leg bone
    
    // Burning Effect
    burning_damage: 20.0,        // Burning damage per tick
    burning_duration: 5,         // 5 ticks
};

// Actor A Position
let actor_a_position = Position {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
```

### **Actor B Stats (Fire Cultivator)**
```rust
// Actor B - Fire Cultivator (High Fire Defense)
let actor_b_stats = ActorStats {
    // Primary Stats
    vitality: 180,        // Scales health
    fire_defense: 120,    // Scales fire damage reduction
    fire_resistance: 0.3, // 30% fire resistance
    
    // Movement & Defense Stats
    movement_speed: 7.0,      // 7.0 units per second
    attack_speed: 1.0,        // 1.0 attacks per second
    evasion: 25,              // 25% base evasion
    
    // Health Calculation
    base_health: 1000.0,
    vitality_bonus: 180.0,  // 180 vitality = +180 health
    total_health: 1180.0,   // 1000 + 180
    
    // Fire Shield Stacks
    fire_shield_stacks: 10,
    fire_shield_hp_per_stack: 120.0,  // 120 HP per stack (scaled by fire_defense)
    total_shield_hp: 1200.0,          // 10 × 120 = 1200 HP
    
    // Status Effects
    leg_bone_broken: false,
    movement_speed_modifier: 1.0,  // Normal movement speed
};

// Actor B Position (starts at range 2.5)
let actor_b_position = Position {
    x: 2.5,
    y: 0.0,
    z: 0.0,
};
```

### **Attack 1: Hit (Normal Attack)**
```rust
// Actor A attacks Actor B with Fire Punch
// Range Check: Distance = 2.5, Min Range = 1.0, Max Range = 3.0
// Range Check: 1.0 ≤ 2.5 ≤ 3.0 → IN RANGE ✓

// Accuracy vs Evasion Check
// Hit Chance = 85% (accuracy) - 25% (evasion) = 60% hit chance
// Random Roll: 45% → HIT! ✓

// Damage Calculation:
// Physical: 100 base + (150 strength × 0.8) = 220 physical damage
// Fire: 80 base + (200 fire_attack × 0.6) = 200 fire damage
// Total: 420 damage (220 physical + 200 fire)

// Status Effect Check (25% chance)
// Random Roll: 15% → LEG BONE BROKEN! ✓

let fire_punch_damage = DamageResult {
    final_damage: 420.0,  // 220 physical + 200 fire
    damage_type: "fire_punch",
    shield_absorbed: 0.0,
    resource_damage_applied: 0.0,
    timestamp: current_timestamp(),
    is_critical: false,
    burning_applied: true,  // Burning effect applied
    leg_bone_broken: true,  // Leg bone broken effect applied
    range_check: true,      // Attack in range
    hit_check: true,        // Attack hit
};
```

// Actor B's Fire Shields (10 stacks = 1200 total shield HP)
let fire_shields = vec![
    ShieldActor {
        id: "fire_shield_1".to_string(),
        shield_type: ShieldType::Fire,
        current_hp: 120.0,  // Each shield has 120 HP (scaled by fire_defense)
        max_hp: 120.0,
        priority: 500,  // Medium priority
        subsystem_id: "fire_cultivation".to_string(),
        damage_type_filter: Some("fire".to_string()),
        lifetime_remaining: 30000,  // 30 seconds
        decay_rate: 0.0,  // No decay
        custom_config: None,
    },
    ShieldActor {
        id: "fire_shield_2".to_string(),
        shield_type: ShieldType::Fire,
        current_hp: 120.0,
        max_hp: 120.0,
        priority: 500,
        subsystem_id: "fire_cultivation".to_string(),
        damage_type_filter: Some("fire".to_string()),
        lifetime_remaining: 30000,
        decay_rate: 0.0,
        custom_config: None,
    },
    // ... 8 more fire shields (total 1200 HP)
];

// Damage application result
let result = DamageApplicationResult {
    actor_id: "actor_b".to_string(),
    original_damage: 420.0,
    final_damage: 220.0,  // 200 fire damage absorbed by shields
    shield_result: ShieldProcessingResult {
        original_damage: 420.0,
        final_damage: 220.0,
        total_absorbed: 200.0,  // Fire damage absorbed
        shield_results: vec![
            ShieldResult {
                shield_id: "fire_shield_1".to_string(),
                shield_type: ShieldType::Fire,
                damage_absorbed: 120.0,  // Shield 1 completely absorbed
                remaining_hp: 0.0,
                is_broken: true,
                subsystem_id: "fire_cultivation".to_string(),
            },
            ShieldResult {
                shield_id: "fire_shield_2".to_string(),
                shield_type: ShieldType::Fire,
                damage_absorbed: 80.0,   // Shield 2 partially absorbed
                remaining_hp: 40.0,
                is_broken: false,
                subsystem_id: "fire_cultivation".to_string(),
            },
        ],
        shields_broken: vec!["fire_shield_1".to_string()],  // 1 shield broken
        processing_time: 0.02,
    },
    resource_result: ResourceDamageResult {
        original_damage: 220.0,  // Physical damage after shield absorption
        final_damage: 0.0,  // All damage applied to health
        total_damage_applied: 220.0,
        resource_results: vec![
            ResourceDamageResultEntry {
                resource_id: "health".to_string(),
                resource_type: ResourceType::Health,
                damage_applied: 220.0,
                remaining_value: 960.0,  // 1180 - 220 = 960 HP
                is_depleted: false,
            },
        ],
        resources_depleted: vec![],
        processing_time: 0.01,
    },
    // ... other results
};
```

### **Attack 2: Miss (Out of Range)**
```rust
// Actor A attacks Actor B with Fire Punch (MISS - OUT OF RANGE)
// Actor B moved away due to leg bone broken effect
// New Actor B Position: x: 3.5, y: 0.0, z: 0.0
// Range Check: Distance = 3.5, Min Range = 1.0, Max Range = 3.0
// Range Check: 3.5 > 3.0 → OUT OF RANGE ✗

let fire_punch_damage = DamageResult {
    final_damage: 0.0,  // Missed attack - out of range
    damage_type: "fire_punch",
    shield_absorbed: 0.0,
    resource_damage_applied: 0.0,
    timestamp: current_timestamp(),
    is_critical: false,
    burning_applied: false,  // No burning on miss
    leg_bone_broken: false,  // No status effect on miss
    range_check: false,      // Attack out of range
    hit_check: false,        // Attack missed
    miss_reason: "out_of_range".to_string(),
};

// Actor B's Status After Attack 1
let actor_b_status_after_attack_1 = ActorBStatus {
    leg_bone_broken: true,
    movement_speed_modifier: 0.5,  // 50% movement speed (leg broken)
    current_movement_speed: 3.5,   // 7.0 × 0.5 = 3.5 units/sec
    position: Position { x: 3.5, y: 0.0, z: 0.0 },  // Moved away
};

// Result: No damage, no shield interaction
let result = DamageApplicationResult {
    actor_id: "actor_b".to_string(),
    original_damage: 0.0,
    final_damage: 0.0,
    shield_result: ShieldProcessingResult {
        original_damage: 0.0,
        final_damage: 0.0,
        total_absorbed: 0.0,
        shield_results: vec![],  // No shields hit
        shields_broken: vec![],
        processing_time: 0.01,
    },
    // ... other results
};
```

### **Attack 3: Critical Hit - Massive Damage**
```rust
// Actor A attacks Actor B with Fire Punch (CRITICAL HIT!)
// Actor A moved closer to get in range
// New Actor A Position: x: 1.0, y: 0.0, z: 0.0
// New Actor B Position: x: 3.5, y: 0.0, z: 0.0
// Range Check: Distance = 2.5, Min Range = 1.0, Max Range = 3.0
// Range Check: 1.0 ≤ 2.5 ≤ 3.0 → IN RANGE ✓

// Accuracy vs Evasion Check
// Hit Chance = 85% (accuracy) - 25% (evasion) = 60% hit chance
// Random Roll: 30% → HIT! ✓

// Critical Hit Check
// Crit Chance = 25%
// Random Roll: 20% → CRITICAL HIT! ✓

// Damage Calculation (Critical):
// Physical: (100 base + (150 strength × 0.8)) × 2.5 crit = 550 physical damage
// Fire: (80 base + (200 fire_attack × 0.6)) × 2.5 crit = 500 fire damage
// Total: 1050 damage (550 physical + 500 fire)

let fire_punch_damage = DamageResult {
    final_damage: 1050.0,  // 550 physical + 500 fire (CRITICAL!)
    damage_type: "fire_punch",
    shield_absorbed: 0.0,
    resource_damage_applied: 0.0,
    timestamp: current_timestamp(),
    is_critical: true,      // CRITICAL HIT!
    burning_applied: true,  // Burning effect applied
};

// Actor B's Fire Shields (damaged from previous attacks)
// Total remaining: 1080 HP (1200 - 120 from Attack 1)
let fire_shields = vec![
    ShieldActor {
        id: "fire_shield_2".to_string(),
        shield_type: ShieldType::Fire,
        current_hp: 40.0,  // Damaged from Attack 1
        max_hp: 120.0,
        priority: 500,
        subsystem_id: "fire_cultivation".to_string(),
        damage_type_filter: Some("fire".to_string()),
        lifetime_remaining: 25000,
        decay_rate: 0.0,
        custom_config: None,
    },
    ShieldActor {
        id: "fire_shield_3".to_string(),
        shield_type: ShieldType::Fire,
        current_hp: 120.0,  // Undamaged
        max_hp: 120.0,
        priority: 500,
        subsystem_id: "fire_cultivation".to_string(),
        damage_type_filter: Some("fire".to_string()),
        lifetime_remaining: 25000,
        decay_rate: 0.0,
        custom_config: None,
    },
    // ... 7 more fire shields (all at 120 HP)
];

// Damage application result
let result = DamageApplicationResult {
    actor_id: "actor_b".to_string(),
    original_damage: 1050.0,
    final_damage: 550.0,  // 500 fire damage absorbed by shields
    shield_result: ShieldProcessingResult {
        original_damage: 1050.0,
        final_damage: 550.0,
        total_absorbed: 500.0,  // Fire damage absorbed
        shield_results: vec![
            ShieldResult {
                shield_id: "fire_shield_2".to_string(),
                shield_type: ShieldType::Fire,
                damage_absorbed: 40.0,  // Shield 2 completely absorbed
                remaining_hp: 0.0,
                is_broken: true,
                subsystem_id: "fire_cultivation".to_string(),
            },
            ShieldResult {
                shield_id: "fire_shield_3".to_string(),
                shield_type: ShieldType::Fire,
                damage_absorbed: 120.0,  // Shield 3 completely absorbed
                remaining_hp: 0.0,
                is_broken: true,
                subsystem_id: "fire_cultivation".to_string(),
            },
            ShieldResult {
                shield_id: "fire_shield_4".to_string(),
                shield_type: ShieldType::Fire,
                damage_absorbed: 120.0,  // Shield 4 completely absorbed
                remaining_hp: 0.0,
                is_broken: true,
                subsystem_id: "fire_cultivation".to_string(),
            },
            ShieldResult {
                shield_id: "fire_shield_5".to_string(),
                shield_type: ShieldType::Fire,
                damage_absorbed: 120.0,  // Shield 5 completely absorbed
                remaining_hp: 0.0,
                is_broken: true,
                subsystem_id: "fire_cultivation".to_string(),
            },
            ShieldResult {
                shield_id: "fire_shield_6".to_string(),
                shield_type: ShieldType::Fire,
                damage_absorbed: 100.0,  // Shield 6 partially absorbed
                remaining_hp: 20.0,
                is_broken: false,
                subsystem_id: "fire_cultivation".to_string(),
            },
        ],
        shields_broken: vec![
            "fire_shield_2".to_string(),
            "fire_shield_3".to_string(),
            "fire_shield_4".to_string(),
            "fire_shield_5".to_string(),
        ],  // 4 shields broken!
        processing_time: 0.02,
    },
    resource_result: ResourceDamageResult {
        original_damage: 550.0,  // Physical damage after shield absorption
        final_damage: 0.0,  // All damage applied to health
        total_damage_applied: 550.0,
        resource_results: vec![
            ResourceDamageResultEntry {
                resource_id: "health".to_string(),
                resource_type: ResourceType::Health,
                damage_applied: 550.0,
                remaining_value: 410.0,  // 960 - 550 = 410 HP
                is_depleted: false,
            },
        ],
        resources_depleted: vec![],
        processing_time: 0.01,
    },
    // ... other results
};
```

### **Attack 4: Hit - Actor B Dies**
```rust
// Actor A attacks Actor B with Fire Punch (final attack)
// Damage Calculation:
// Physical: 100 base + (150 strength × 0.8) = 220 physical damage
// Fire: 80 base + (200 fire_attack × 0.6) = 200 fire damage
// Total: 420 damage (220 physical + 200 fire)

let fire_punch_damage = DamageResult {
    final_damage: 420.0,  // 220 physical + 200 fire
    damage_type: "fire_punch",
    shield_absorbed: 0.0,
    resource_damage_applied: 0.0,
    timestamp: current_timestamp(),
    is_critical: false,
    burning_applied: true,  // Burning effect applied
};

// Actor B's remaining Fire Shields (5 stacks left = 600 HP total)
let fire_shields = vec![
    ShieldActor {
        id: "fire_shield_6".to_string(),
        shield_type: ShieldType::Fire,
        current_hp: 20.0,  // Damaged from Attack 3
        max_hp: 120.0,
        priority: 500,
        subsystem_id: "fire_cultivation".to_string(),
        damage_type_filter: Some("fire".to_string()),
        lifetime_remaining: 20000,
        decay_rate: 0.0,
        custom_config: None,
    },
    ShieldActor {
        id: "fire_shield_7".to_string(),
        shield_type: ShieldType::Fire,
        current_hp: 120.0,  // Undamaged
        max_hp: 120.0,
        priority: 500,
        subsystem_id: "fire_cultivation".to_string(),
        damage_type_filter: Some("fire".to_string()),
        lifetime_remaining: 20000,
        decay_rate: 0.0,
        custom_config: None,
    },
    // ... 3 more fire shields (all at 120 HP)
];

// Actor B's Health (damaged from previous attacks)
let actor_b_health = ResourceInfo {
    resource_id: "health".to_string(),
    resource_type: ResourceType::Health,
    current_value: 410.0,  // Health from previous attacks
    max_value: 1180.0,
    regen_rate: 10.0,
    priority: 1000,  // Highest priority
    is_protected: true,
    protection_factor: 0.0,
};

// Damage application result
let result = DamageApplicationResult {
    actor_id: "actor_b".to_string(),
    original_damage: 420.0,
    final_damage: 0.0,  // All damage absorbed by shields and health
    shield_result: ShieldProcessingResult {
        original_damage: 420.0,
        final_damage: 220.0,  // 200 fire absorbed by shields
        total_absorbed: 200.0,
        shield_results: vec![
            ShieldResult {
                shield_id: "fire_shield_6".to_string(),
                shield_type: ShieldType::Fire,
                damage_absorbed: 20.0,  // Shield 6 completely absorbed
                remaining_hp: 0.0,
                is_broken: true,
                subsystem_id: "fire_cultivation".to_string(),
            },
            ShieldResult {
                shield_id: "fire_shield_7".to_string(),
                shield_type: ShieldType::Fire,
                damage_absorbed: 120.0,  // Shield 7 completely absorbed
                remaining_hp: 0.0,
                is_broken: true,
                subsystem_id: "fire_cultivation".to_string(),
            },
            ShieldResult {
                shield_id: "fire_shield_8".to_string(),
                shield_type: ShieldType::Fire,
                damage_absorbed: 60.0,   // Shield 8 partially absorbed
                remaining_hp: 60.0,
                is_broken: false,
                subsystem_id: "fire_cultivation".to_string(),
            },
        ],
        shields_broken: vec![
            "fire_shield_6".to_string(),
            "fire_shield_7".to_string(),
        ],  // 2 more shields broken
        processing_time: 0.02,
    },
    resource_result: ResourceDamageResult {
        original_damage: 220.0,  // Physical damage after shield absorption
        final_damage: 0.0,  // All damage applied to health
        total_damage_applied: 220.0,
        resource_results: vec![
            ResourceDamageResultEntry {
                resource_id: "health".to_string(),
                resource_type: ResourceType::Health,
                damage_applied: 220.0,
                remaining_value: 190.0,  // 410 - 220 = 190 HP
                is_depleted: false,
            },
        ],
        resources_depleted: vec![],
        processing_time: 0.01,
    },
    // ... other results
};
```

### **Burning Effect Processing**
```rust
// Burning effects from previous attacks
let burning_effects = vec![
    BurningEffect {
        actor_id: "actor_b".to_string(),
        damage_per_tick: 20.0,
        remaining_ticks: 4,  // From Attack 1
        total_damage: 80.0,  // 4 ticks × 20 damage
    },
    BurningEffect {
        actor_id: "actor_b".to_string(),
        damage_per_tick: 20.0,
        remaining_ticks: 5,  // From Attack 3 (critical)
        total_damage: 100.0, // 5 ticks × 20 damage
    },
];

// Process burning damage
let total_burning_damage = 180.0;  // 80 + 100
let final_health = 190.0 - 180.0;  // 190 - 180 = 10 HP

// Actor B survives with 10 HP!
```

### **Attack 5: Miss (Accuracy vs Evasion)**
```rust
// Actor A attacks Actor B with Fire Punch (MISS - EVASION)
// Actor A Position: x: 1.0, y: 0.0, z: 0.0
// Actor B Position: x: 3.5, y: 0.0, z: 0.0
// Range Check: Distance = 2.5, Min Range = 1.0, Max Range = 3.0
// Range Check: 1.0 ≤ 2.5 ≤ 3.0 → IN RANGE ✓

// Accuracy vs Evasion Check
// Hit Chance = 85% (accuracy) - 25% (evasion) = 60% hit chance
// Random Roll: 75% → MISS! ✗ (Rolled above hit chance)

let fire_punch_damage = DamageResult {
    final_damage: 0.0,  // Missed attack - evaded
    damage_type: "fire_punch",
    shield_absorbed: 0.0,
    resource_damage_applied: 0.0,
    timestamp: current_timestamp(),
    is_critical: false,
    burning_applied: false,  // No burning on miss
    leg_bone_broken: false,  // No status effect on miss
    range_check: true,       // Attack in range
    hit_check: false,        // Attack missed
    miss_reason: "evasion".to_string(),
};

// Result: No damage, no shield interaction
let result = DamageApplicationResult {
    actor_id: "actor_b".to_string(),
    original_damage: 0.0,
    final_damage: 0.0,
    shield_result: ShieldProcessingResult {
        original_damage: 0.0,
        final_damage: 0.0,
        total_absorbed: 0.0,
        shield_results: vec![],  // No shields hit
        shields_broken: vec![],
        processing_time: 0.01,
    },
    // ... other results
};
```

### **Combat Summary**
```
Attack 1: Fire Punch → Hit → 200 fire damage absorbed by shields, 220 physical to health
         → Leg Bone Broken Effect Applied (25% chance)
         → Actor B movement speed reduced to 50%
Attack 2: Fire Punch → Miss (Out of Range) → Actor B moved to x: 3.5 (out of range 3.0)
Attack 3: Fire Punch → CRITICAL HIT → 500 fire damage breaks 4 shields, 550 physical to health  
         → Actor A moved closer to x: 1.0 to get in range
Attack 4: Fire Punch → Hit → 200 fire damage breaks 2 shields, 220 physical to health
Attack 5: Fire Punch → Miss (Evasion) → 75% roll > 60% hit chance

Total Damage Dealt: 1890 (3 hits × 420 + 1 crit × 1050 - 2 misses)
Total Fire Damage Absorbed: 900 (3 hits × 200 + 1 crit × 500)
Total Physical Damage: 990 (3 hits × 220 + 1 crit × 550)
Shields Broken: 7 stacks (fire_shield_1, 2, 3, 4, 5, 6, 7)
Shields Remaining: 3 stacks (360 HP total)
Burning Damage: 180 (80 + 100 from DoT effects)
Status Effects: Leg Bone Broken (movement speed -50%)
Actor B Status: ALIVE (10 HP remaining after burning)
```

### **Detailed Stat Calculations**
```rust
// Input Stats
let actor_a_input = ActorAInputStats {
    strength: 150,
    fire_attack: 200,
    fire_crit_chance: 0.25,
    fire_crit_damage: 2.5,
    movement_speed: 8.5,
    attack_speed: 1.2,
    accuracy: 85,
    fire_punch_physical: 100.0,
    fire_punch_fire: 80.0,
    fire_punch_min_range: 1.0,
    fire_punch_max_range: 3.0,
    leg_bone_break_chance: 0.25,
    burning_damage: 20.0,
    burning_duration: 5,
};

let actor_b_input = ActorBInputStats {
    vitality: 180,
    fire_defense: 120,
    fire_resistance: 0.3,
    movement_speed: 7.0,
    attack_speed: 1.0,
    evasion: 25,
    base_health: 1000.0,
    fire_shield_stacks: 10,
    fire_shield_hp_per_stack: 120.0,
};

// Output Calculations
let actor_a_output = ActorAOutputStats {
    // Damage Calculations
    physical_damage_per_attack: 220.0,  // 100 + (150 × 0.8)
    fire_damage_per_attack: 200.0,      // 80 + (200 × 0.6)
    total_damage_per_attack: 420.0,     // 220 + 200
    critical_damage: 1050.0,            // 420 × 2.5
    
    // Combat Mechanics
    hit_chance: 0.60,                   // 85% - 25% = 60%
    critical_hit_chance: 0.25,          // 25%
    leg_bone_break_chance: 0.25,        // 25%
    
    // Movement & Positioning
    movement_speed: 8.5,                // units per second
    attack_range: 3.0,                  // maximum range
    attack_speed: 1.2,                  // attacks per second
};

let actor_b_output = ActorBOutputStats {
    // Health & Defense
    total_health: 1180.0,               // 1000 + 180
    total_shield_hp: 1200.0,            // 10 × 120
    fire_resistance: 0.3,               // 30% fire damage reduction
    
    // Movement & Defense
    base_movement_speed: 7.0,           // units per second
    current_movement_speed: 3.5,        // 7.0 × 0.5 (leg broken)
    evasion: 0.25,                      // 25% evasion chance
    
    // Status Effects
    leg_bone_broken: true,              // Applied in Attack 1
    movement_speed_modifier: 0.5,       // 50% speed reduction
    burning_effects: 2,                 // 2 burning effects active
};

// Combat Results
let combat_results = CombatResults {
    total_attacks: 5,
    successful_hits: 3,
    misses: 2,
    critical_hits: 1,
    range_misses: 1,
    evasion_misses: 1,
    
    total_damage_dealt: 1890.0,
    fire_damage_absorbed: 900.0,
    physical_damage_dealt: 990.0,
    burning_damage: 180.0,
    
    shields_broken: 7,
    shields_remaining: 3,
    
    final_health: 10.0,
    combat_outcome: "actor_b_survives".to_string(),
};
```

### **Enhanced Stats Impact**
```
Actor A (Fire Cultivator):
- Strength: 150 → +120 physical damage per attack
- Fire Attack: 200 → +120 fire damage per attack  
- Crit Chance: 25% → Critical hit on Attack 3
- Crit Damage: 250% → Massive damage multiplier
- Movement Speed: 8.5 → Faster than Actor B
- Attack Speed: 1.2 → 1.2 attacks per second
- Accuracy: 85% → 60% hit chance (85% - 25% evasion)
- Attack Range: 3.0 → Maximum range for Fire Punch
- Leg Bone Break: 25% → Applied in Attack 1

Actor B (Fire Cultivator):
- Vitality: 180 → +180 health (1180 total)
- Fire Defense: 120 → +20 HP per shield stack (1200 total)
- Fire Resistance: 30% → Reduced fire damage (not shown in example)
- Fire Shields: 10 stacks → 1200 HP total protection
- Movement Speed: 7.0 → Slower than Actor A
- Attack Speed: 1.0 → 1.0 attacks per second
- Evasion: 25% → 25% chance to dodge attacks
- Status Effect: Leg Bone Broken → Movement speed -50% (3.5 units/sec)
```

### **Shield Status After Combat**
```
Initial Fire Shields: 10 stacks (1200 HP total)
Fire Damage Absorbed: 900 HP
Shields Broken: 7 stacks (fire_shield_1, 2, 3, 4, 5, 6, 7)
Shields Remaining: 3 stacks (360 HP total)
Shield Effectiveness: 100% (all fire damage absorbed)
Physical Damage Bypass: 990 HP (bypassed shields, hit health directly)
Burning Damage: 180 HP (DoT effects after combat)
```

### **Events Generated**
```rust
// Events from the enhanced combat sequence
let events = vec![
    // Attack 1 - Normal Hit
    CombatEvent {
        event_type: CombatEventType::DamageApplied,
        actor_id: "actor_b".to_string(),
        version: 1,
        timestamp: 1000,
        data: EventData::DamageApplied {
            damage_amount: 220.0,  // Physical damage after shield absorption
            damage_type: "fire_punch",
            source_actor_id: "actor_a".to_string(),
        },
    },
    
    // Attack 1 - Shield Broken
    CombatEvent {
        event_type: CombatEventType::ShieldBroken,
        actor_id: "actor_b".to_string(),
        version: 1,
        timestamp: 1000,
        data: EventData::ShieldBroken {
            shield_id: "fire_shield_1".to_string(),
            shield_type: "fire".to_string(),
            remaining_hp: 0.0,
        },
    },
    
    // Attack 1 - Burning Applied
    CombatEvent {
        event_type: CombatEventType::StatusEffectApplied,
        actor_id: "actor_b".to_string(),
        version: 1,
        timestamp: 1000,
        data: EventData::StatusEffectApplied {
            effect_id: "burning".to_string(),
            effect_type: "damage_over_time".to_string(),
            duration: 5,
            damage_per_tick: 20.0,
        },
    },
    
    // Attack 3 - Critical Hit
    CombatEvent {
        event_type: CombatEventType::CriticalHit,
        actor_id: "actor_b".to_string(),
        version: 3,
        timestamp: 3000,
        data: EventData::CriticalHit {
            damage_amount: 1050.0,
            damage_type: "fire_punch",
            source_actor_id: "actor_a".to_string(),
            crit_multiplier: 2.5,
        },
    },
    
    // Attack 3 - Multiple Shields Broken
    CombatEvent {
        event_type: CombatEventType::ShieldBroken,
        actor_id: "actor_b".to_string(),
        version: 3,
        timestamp: 3000,
        data: EventData::ShieldBroken {
            shield_id: "fire_shield_2".to_string(),
            shield_type: "fire".to_string(),
            remaining_hp: 0.0,
        },
    },
    // ... Similar events for fire_shield_3, 4, 5
    
    // Attack 4 - Final Hit
    CombatEvent {
        event_type: CombatEventType::DamageApplied,
        actor_id: "actor_b".to_string(),
        version: 4,
        timestamp: 4000,
        data: EventData::DamageApplied {
            damage_amount: 220.0,  // Physical damage after shield absorption
            damage_type: "fire_punch",
            source_actor_id: "actor_a".to_string(),
        },
    },
    
    // Burning Damage Processing
    CombatEvent {
        event_type: CombatEventType::StatusEffectTick,
        actor_id: "actor_b".to_string(),
        version: 5,
        timestamp: 5000,
        data: EventData::StatusEffectTick {
            effect_id: "burning".to_string(),
            damage_amount: 180.0,  // Total burning damage
            remaining_ticks: 0,
        },
    },
    
    // Final Status - Actor B Survives
    CombatEvent {
        event_type: CombatEventType::ActorStatusUpdate,
        actor_id: "actor_b".to_string(),
        version: 6,
        timestamp: 6000,
        data: EventData::ActorStatusUpdate {
            health_remaining: 10.0,
            shields_remaining: 3,
            status: "alive".to_string(),
        },
    },
];
```

## 📊 **Performance Optimizations**

### **1. Shield System Optimizations**

```rust
// Performance optimizations for shield system
impl ShieldRegistrationSystem {
    /// Pre-sort shields only on changes (not per-hit)
    pub async fn pre_sort_shields_on_change(&mut self, actor_id: &str) -> ActorCoreResult<()> {
        // Only re-sort when shields are added/removed/modified
        // This avoids per-hit sorting overhead
        self.invalidate_priority_cache(actor_id);
        Ok(())
    }
    
    /// Get shields with linear scan (O(n) per hit)
    pub async fn get_shields_linear_scan(
        &self,
        actor_id: &str,
        damage_type: &str,
    ) -> ActorCoreResult<Vec<ShieldActor>> {
        // Linear scan through pre-sorted shields
        // Stop when damage <= 0
        let shields = self.active_shields.get(actor_id)
            .cloned()
            .unwrap_or_default();
        
        let compatible_shields: Vec<ShieldActor> = shields
            .into_iter()
            .filter(|shield| shield.is_active && shield.can_take_damage_type(damage_type))
            .collect();
        
        Ok(compatible_shields)
    }
    
    /// Cleanup broken shields to prevent memory bloat
    async fn cleanup_broken_shields(&mut self) -> ActorCoreResult<()> {
        for shields in self.active_shields.values_mut() {
            shields.retain(|shield| shield.is_active && shield.shield_hp > 0.0);
        }
        Ok(())
    }
}
```

### **2. Batch Processing**

```rust
// Batch damage application for multiple actors
impl DamageApplicationEngine {
    /// Apply damage to multiple actors in batch
    pub async fn apply_damage_batch(
        &self,
        damage_applications: &[(String, DamageResult)],
    ) -> ActorCoreResult<Vec<DamageApplicationResult>> {
        let mut results = Vec::new();
        
        // Process in parallel
        let mut tasks = Vec::new();
        for (actor_id, mut damage) in damage_applications.iter() {
            let engine = self.clone();
            let actor_id = actor_id.clone();
            let task = tokio::spawn(async move {
                engine.apply_damage(&actor_id, &mut damage).await
            });
            tasks.push(task);
        }
        
        // Wait for all tasks to complete
        for task in tasks {
            match task.await {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(ActorCoreError::TaskError(e.to_string())),
            }
        }
        
        Ok(results)
    }
}
```

### **2. Caching and Pre-calculation**

```rust
// Cached damage application for repeated patterns
impl DamageApplicationEngine {
    /// Get cached damage application result
    pub async fn get_cached_result(
        &self,
        actor_id: &str,
        damage_hash: u64,
    ) -> Option<DamageApplicationResult> {
        // Implementation for cached damage application
        None
    }
    
    /// Cache damage application result
    pub async fn cache_result(
        &self,
        actor_id: &str,
        damage_hash: u64,
        result: DamageApplicationResult,
    ) -> ActorCoreResult<()> {
        // Implementation for caching damage application
        Ok(())
    }
}
```

## 🧪 **Testing Strategy**

### **1. Unit Tests**
- Shield order processing tests
- Resource damage distribution tests
- Protection system tests
- Event generation tests

### **2. Integration Tests**
- End-to-end damage application tests
- Multi-actor damage application tests
- Shield and resource interaction tests
- Performance tests

### **3. Load Tests**
- High-frequency damage application tests
- Large-scale combat scenario tests
- Memory usage tests
- Performance benchmark tests

## ❓ **Questions for Discussion**

1. **Shield as Actors**: Việc coi shield như actors độc lập có hợp lý không?
2. **Subsystem Registration**: Hệ thống đăng ký shield từ subsystems có linh hoạt không?
3. **Priority Formula**: Công thức tính priority có công bằng và deterministic không?
4. **Shield Stack Limits**: Giới hạn số lượng shield stacks có phù hợp không?
5. **Lifetime Decay**: Hệ thống decay theo thời gian có realistic không?
6. **Restoration Events**: Hệ thống restore shield HP từ events có hiệu quả không?
7. **Performance Impact**: Performance impact của shield system có chấp nhận được không?
8. **Damage Type Filtering**: Lọc damage types cho shields có cần thiết không?
9. **Resource Protection Balance**: Hệ thống bảo vệ resource có cân bằng không?
10. **Subsystem Protection Configuration**: Cấu hình protection từ subsystems có linh hoạt không?
11. **Breakpoint Conditions**: Hệ thống breakpoint conditions có realistic không?
12. **Protection Stacking**: Quy tắc stacking protection có công bằng không?
13. **Protection Decay**: Hệ thống decay protection có cần thiết không?
14. **Always Protected Resources**: Việc luôn bảo vệ Health, LifeForce, Lifespan có hợp lý không?
15. **Damage Distribution Fairness**: Hệ thống phân phối damage có công bằng không?
16. **Impact Map Percentages**: Tỷ lệ phân phối damage trong impact maps có hợp lý không?
17. **Distribution Order**: Thứ tự phân phối Shields → Temporary → Primary → Secondary → Special có logic không?
18. **True Damage Bypass**: Việc True damage bypass secondary resources có cân bằng không?
19. **Elemental Storage**: Hệ thống elemental storage cho elemental damage có realistic không?
20. **Performance vs Complexity**: Cân bằng giữa performance và complexity có hợp lý không?
21. **Data Layout Optimization**: Fixed-size vectors và versioned snapshots có hiệu quả không?
22. **Hot Path Math**: Branch-light calculations và tránh dynamic dispatch có cần thiết không?
23. **Latency Budget**: 0.05ms target latency có realistic cho MMORPG không?
24. **Event Throttling**: Coalescing và capping events có hiệu quả không?
25. **Batch Processing**: Micro-batch processing có cần thiết cho scale không?
26. **Caching Strategy**: L1 cache và result cache có tối ưu không?
27. **Depletion Effects**: Cooldown và loop prevention có cần thiết không?
28. **Event System Integration**: Tích hợp event system có hiệu quả không?

## 🎯 **Next Steps**

1. **Implement Shield as Actors System**: Xây dựng hệ thống shield như actors độc lập
2. **Implement Subsystem Registration**: Xây dựng hệ thống đăng ký shield từ subsystems
3. **Implement Shield Priority System**: Xây dựng hệ thống priority với formula deterministic
4. **Implement Enhanced Resource Protection System**: Xây dựng hệ thống bảo vệ resource với subsystem configuration
5. **Implement Breakpoint Conditions**: Xây dựng hệ thống breakpoint conditions và protection conditions
6. **Implement Protection Stacking**: Xây dựng hệ thống stacking rules cho protection
7. **Implement Damage Distribution Fairness**: Xây dựng hệ thống phân phối damage với fairness rules
8. **Implement Impact Maps**: Xây dựng hệ thống impact maps cho các damage types
9. **Implement Resource Categories**: Xây dựng hệ thống phân loại resources
10. **Implement Performance Optimizations**: Xây dựng data layout optimization và hot path math
11. **Implement Event System**: Xây dựng hệ thống event với throttling và coalescing
12. **Implement Batch Processing**: Xây dựng micro-batch processing và fair scheduling
13. **Implement Caching Strategy**: Xây dựng L1 cache và result cache
14. **Implement Depletion Effects**: Xây dựng depletion effects handling với loop prevention
15. **Testing & Optimization**: Test và optimize toàn bộ system

---

## 🧭 **Action Queue Modes (Realtime vs Turn-based)**

### **Mode Binding**
- Areas/instances declare `combat_mode` via world-core. Actors entering a turn-based area attach to a shared encounter instance.

### **Schedulers**
- Realtime: tick batching per `server_timing.yaml`; ability queue window honored.
- Turn-based: lockstep rounds with barriers; phases defined in `turn_based.yaml`.

### **Initiative & AP**
- Initiative (per round): `initiative.formula` from `turn_based.yaml` (speed/haste weighted, seeded tiebreaker).
- Variable AP: `action_points.base_ap_per_round`; AP cost per action computed by `ap_cost_formula` using action `duration_s`/`cooldown_s`.
- Minor actions: actions with AP < `minor_action_threshold_ap`.

### **Phases**
- Input window: default 1500 ms; AFK timeout 5000 ms (see `server_timing.yaml` and `turn_based.yaml`).
- Execution: resolve in descending initiative; each action fully applies before the next.
- End-of-round: regen, DoT/HoT ticks, exhaustion checks, event flush.

### **Mode Switching**
- RT→TB: finish current tick, snapshot, enqueue at next turn boundary.
- TB→RT: finish current turn if acting; release at next tick.

### **Events**
- TurnStarted, ActorTurnStarted, ActorTurnEnded, TurnEnded (see `turn_based.yaml`).

*Tài liệu này sẽ được cập nhật khi hệ thống phát triển và có thêm yêu cầu mới.*
