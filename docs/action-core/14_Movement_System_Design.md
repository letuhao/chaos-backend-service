# Movement System Design

## 📋 **Tổng Quan**

Movement System là một subsystem của Action Core, chịu trách nhiệm quản lý di chuyển của các actors trong và ngoài combat. Hệ thống này bao gồm quản lý tọa độ, movement actions, movement restrictions, và tích hợp với các hệ thống khác như Element Core và Status Core.

## 🎯 **Vấn Đề Cần Giải Quyết**

### **1. Coordinate Management**
- **Position Tracking**: Theo dõi vị trí của actors trong world
- **Coordinate System**: Hệ tọa độ 2D/3D cho world
- **Position Updates**: Cập nhật vị trí real-time
- **Position Validation**: Validate vị trí hợp lệ

### **2. Movement Actions**
- **Basic Movement**: Di chuyển cơ bản (walk, run, sprint)
- **Advanced Movement**: Di chuyển nâng cao (teleport, fly, phase)
- **Movement Techniques**: Kỹ thuật di chuyển (thân pháp)
- **Movement Skills**: Skills di chuyển đặc biệt

### **3. Movement Restrictions**
- **Immobilization**: Bất động (như Crystal Defense Technique)
- **Movement Impairment**: Hạn chế di chuyển
- **Terrain Restrictions**: Hạn chế do địa hình
- **Status Effects**: Hạn chế do status effects

### **4. System Integration**
- **Element Core Integration**: Movement speed dựa trên element mastery
- **Status Core Integration**: Movement restrictions từ status effects
- **Combat Core Integration**: Movement trong combat
- **World Core Integration**: Movement trong world

## 🏗️ **Architecture**

### **Core Components**

```rust
pub struct MovementSystem {
    // Core components
    position_manager: PositionManager,
    movement_action_handler: MovementActionHandler,
    movement_restriction_manager: MovementRestrictionManager,
    movement_calculator: MovementCalculator,
    
    // Integration components
    element_core_client: ElementCoreClient,
    status_core_client: StatusCoreClient,
    combat_core_client: CombatCoreClient,
    world_core_client: WorldCoreClient,
    
    // Performance optimization
    movement_cache: MovementCache,
    batch_processor: MovementBatchProcessor,
    memory_pool: MovementMemoryPool,
    
    // Configuration
    config: MovementConfig,
}

pub struct PositionManager {
    actor_positions: HashMap<ActorId, Position>,
    position_history: PositionHistory,
    position_validator: PositionValidator,
    coordinate_system: CoordinateSystem,
}

pub struct MovementActionHandler {
    action_registry: MovementActionRegistry,
    movement_validator: MovementValidator,
    movement_executor: MovementExecutor,
    movement_effects: MovementEffects,
}
```

## 🔧 **Position Management**

### **1. Position System**

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,  // Optional 3D support
    pub world_id: WorldId,
    pub region_id: RegionId,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionDelta {
    pub dx: f64,
    pub dy: f64,
    pub dz: Option<f64>,
    pub distance: f64,
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
    Up,
    Down,
    Custom(f64), // Custom angle in degrees
}

pub struct CoordinateSystem {
    pub system_type: CoordinateSystemType,
    pub origin: Position,
    pub scale: f64,
    pub bounds: Bounds,
}

pub enum CoordinateSystemType {
    Cartesian2D,
    Cartesian3D,
    Polar2D,
    Spherical3D,
    Custom(String),
}

pub struct Bounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub min_z: Option<f64>,
    pub max_z: Option<f64>,
}
```

### **2. Position Manager Implementation**

```rust
impl PositionManager {
    /// Update actor position
    pub async fn update_position(
        &mut self,
        actor_id: ActorId,
        new_position: Position,
        movement_type: MovementType
    ) -> Result<PositionUpdateResult, MovementError> {
        // Validate position
        self.position_validator.validate_position(new_position)?;
        
        // Get current position
        let current_position = self.actor_positions.get(&actor_id)
            .ok_or_else(|| MovementError::ActorNotFound(actor_id))?;
        
        // Calculate position delta
        let delta = self.calculate_position_delta(*current_position, new_position);
        
        // Check movement restrictions
        self.check_movement_restrictions(actor_id, delta, movement_type).await?;
        
        // Update position
        self.actor_positions.insert(actor_id, new_position);
        
        // Record position history
        self.position_history.record_position(actor_id, new_position);
        
        Ok(PositionUpdateResult {
            old_position: *current_position,
            new_position,
            delta,
            movement_type,
            timestamp: SystemTime::now(),
        })
    }
    
    /// Calculate position delta
    fn calculate_position_delta(
        &self,
        from: Position,
        to: Position
    ) -> PositionDelta {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let dz = to.z.zip(from.z).map(|(tz, fz)| tz - fz);
        
        let distance = match dz {
            Some(dz) => (dx * dx + dy * dy + dz * dz).sqrt(),
            None => (dx * dx + dy * dy).sqrt(),
        };
        
        let direction = self.calculate_direction(dx, dy, dz);
        
        PositionDelta {
            dx,
            dy,
            dz,
            distance,
            direction,
        }
    }
    
    /// Calculate direction from delta
    fn calculate_direction(
        &self,
        dx: f64,
        dy: f64,
        dz: Option<f64>
    ) -> Direction {
        let angle = dy.atan2(dx).to_degrees();
        
        match dz {
            Some(dz) => {
                // 3D direction
                if dz > 0.0 {
                    Direction::Up
                } else if dz < 0.0 {
                    Direction::Down
                } else {
                    self.get_2d_direction(angle)
                }
            },
            None => self.get_2d_direction(angle),
        }
    }
    
    fn get_2d_direction(&self, angle: f64) -> Direction {
        match angle {
            a if a >= -22.5 && a < 22.5 => Direction::East,
            a if a >= 22.5 && a < 67.5 => Direction::Northeast,
            a if a >= 67.5 && a < 112.5 => Direction::North,
            a if a >= 112.5 && a < 157.5 => Direction::Northwest,
            a if a >= 157.5 || a < -157.5 => Direction::West,
            a if a >= -157.5 && a < -112.5 => Direction::Southwest,
            a if a >= -112.5 && a < -67.5 => Direction::South,
            a if a >= -67.5 && a < -22.5 => Direction::Southeast,
            _ => Direction::Custom(angle),
        }
    }
}
```

## 🔧 **Movement Actions**

### **1. Movement Action Types**

```rust
pub enum MovementActionType {
    // Basic movement
    Walk,           // Đi bộ
    Run,            // Chạy
    Sprint,         // Chạy nước rút
    
    // Advanced movement
    Teleport,       // Dịch chuyển tức thời
    Fly,            // Bay
    Phase,          // Xuyên qua vật thể
    Swim,           // Bơi
    Climb,          // Leo trèo
    
    // Movement techniques (thân pháp)
    ShadowStep,     // Bước chân bóng
    WindWalk,       // Đi trong gió
    EarthStride,    // Bước chân đất
    WaterFlow,      // Chảy như nước
    FireDash,       // Lao như lửa
    
    // Special movement
    Mount,          // Cưỡi
    Unmount,        // Xuống
    Follow,         // Theo dõi
    Patrol,         // Tuần tra
}

pub struct MovementAction {
    pub action_id: String,
    pub action_name: String,
    pub action_name_vi: String,
    pub action_type: MovementActionType,
    pub movement_properties: MovementProperties,
    pub resource_requirements: Vec<ResourceRequirement>,
    pub execution_conditions: Vec<ExecutionCondition>,
    pub target_requirements: TargetRequirements,
    pub effects: Vec<ActionEffect>,
}

pub struct MovementProperties {
    pub base_speed: f64,
    pub max_distance: f64,
    pub movement_duration: Duration,
    pub cooldown_duration: Duration,
    pub resource_cost: f64,
    pub movement_restrictions: Vec<MovementRestriction>,
    pub terrain_requirements: Vec<TerrainRequirement>,
    pub element_requirements: Vec<ElementRequirement>,
}
```

### **2. Movement Action Handler**

```rust
impl MovementActionHandler {
    /// Execute movement action
    pub async fn execute_movement_action(
        &mut self,
        action_id: ActionId,
        actor: &mut Actor,
        target_position: Position,
        action_context: &ActionContext
    ) -> Result<MovementResult, MovementError> {
        // Get movement action definition
        let action = self.action_registry.get_action(&action_id)
            .ok_or_else(|| MovementError::ActionNotFound(action_id))?;
        
        // Validate movement action
        self.movement_validator.validate_movement_action(
            &action,
            actor,
            target_position,
            action_context
        ).await?;
        
        // Calculate movement path
        let movement_path = self.calculate_movement_path(
            actor.get_position(),
            target_position,
            &action
        ).await?;
        
        // Execute movement
        let movement_result = self.movement_executor.execute_movement(
            actor,
            &movement_path,
            &action
        ).await?;
        
        // Apply movement effects
        self.movement_effects.apply_movement_effects(
            actor,
            &movement_result,
            &action
        ).await?;
        
        Ok(movement_result)
    }
    
    /// Calculate movement path
    async fn calculate_movement_path(
        &self,
        from: Position,
        to: Position,
        action: &MovementAction
    ) -> Result<MovementPath, MovementError> {
        let distance = self.calculate_distance(from, to);
        
        if distance > action.movement_properties.max_distance {
            return Err(MovementError::DistanceExceeded(distance, action.movement_properties.max_distance));
        }
        
        let path_points = self.generate_path_points(from, to, action).await?;
        let movement_duration = self.calculate_movement_duration(distance, action).await?;
        
        Ok(MovementPath {
            from,
            to,
            path_points,
            distance,
            duration: movement_duration,
            movement_type: action.action_type,
        })
    }
    
    /// Calculate movement duration
    async fn calculate_movement_duration(
        &self,
        distance: f64,
        action: &MovementAction
    ) -> Result<Duration, MovementError> {
        let base_speed = action.movement_properties.base_speed;
        let derived_stats = self.get_derived_stats().await?;
        
        // Apply movement speed bonuses
        let speed_bonus = self.calculate_movement_speed_bonus(
            &derived_stats,
            action
        ).await?;
        
        let final_speed = base_speed * (1.0 + speed_bonus);
        let duration_seconds = distance / final_speed;
        
        Ok(Duration::from_secs_f64(duration_seconds))
    }
    
    /// Calculate movement speed bonus
    async fn calculate_movement_speed_bonus(
        &self,
        derived_stats: &DerivedStatsSnapshot,
        action: &MovementAction
    ) -> Result<f64, MovementError> {
        let mut total_bonus = 0.0;
        
        // Element-based movement speed bonus
        for element_requirement in &action.movement_properties.element_requirements {
            let element_mastery = derived_stats.element_derived_stats
                .element_mastery
                .get(&element_requirement.element_name)
                .unwrap_or(0.0);
            
            let element_bonus = element_mastery * element_requirement.mastery_multiplier;
            total_bonus += element_bonus;
        }
        
        // General movement speed bonus
        let general_bonus = derived_stats.element_derived_stats.skill_execution_speed * 0.1;
        total_bonus += general_bonus;
        
        Ok(total_bonus)
    }
}

pub struct MovementPath {
    pub from: Position,
    pub to: Position,
    pub path_points: Vec<Position>,
    pub distance: f64,
    pub duration: Duration,
    pub movement_type: MovementActionType,
}

pub struct MovementResult {
    pub success: bool,
    pub final_position: Position,
    pub distance_traveled: f64,
    pub duration: Duration,
    pub resource_consumed: HashMap<String, f64>,
    pub movement_effects: Vec<MovementEffect>,
    pub movement_failure_reason: Option<MovementFailureReason>,
}
```

## 🔧 **Movement Restrictions**

### **1. Movement Restriction System**

```rust
pub struct MovementRestrictionManager {
    restrictions: HashMap<ActorId, Vec<MovementRestriction>>,
    restriction_calculator: RestrictionCalculator,
    restriction_validator: RestrictionValidator,
}

pub struct MovementRestriction {
    pub restriction_id: String,
    pub restriction_type: MovementRestrictionType,
    pub magnitude: f64,
    pub duration: Duration,
    pub conditions: Vec<RestrictionCondition>,
    pub source: RestrictionSource,
}

pub enum MovementRestrictionType {
    Immobilized,        // Bất động hoàn toàn
    Slowed,            // Chậm lại
    Rooted,            // Bị rễ
    Stunned,           // Choáng váng
    Paralyzed,         // Tê liệt
    MovementImpairment, // Hạn chế di chuyển
    TerrainRestriction, // Hạn chế do địa hình
    StatusRestriction,  // Hạn chế do status effect
}

pub enum RestrictionSource {
    StatusEffect(String),
    Terrain(String),
    Skill(String),
    Equipment(String),
    Environment(String),
    Custom(String),
}

pub struct RestrictionCondition {
    pub condition_type: RestrictionConditionType,
    pub condition_value: f64,
    pub condition_operator: RestrictionConditionOperator,
}

pub enum RestrictionConditionType {
    HealthPercentage,
    ManaPercentage,
    StaminaPercentage,
    ElementMastery,
    StatusEffect,
    TerrainType,
    TimeOfDay,
    Custom(String),
}

pub enum RestrictionConditionOperator {
    GreaterThan,
    LessThan,
    EqualTo,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqualTo,
}
```

### **2. Movement Restriction Implementation**

```rust
impl MovementRestrictionManager {
    /// Apply movement restriction to actor
    pub async fn apply_restriction(
        &mut self,
        actor_id: ActorId,
        restriction: MovementRestriction
    ) -> Result<(), MovementError> {
        // Validate restriction
        self.restriction_validator.validate_restriction(&restriction)?;
        
        // Check if restriction conflicts with existing ones
        self.check_restriction_conflicts(actor_id, &restriction).await?;
        
        // Apply restriction
        self.restrictions.entry(actor_id)
            .or_insert_with(Vec::new)
            .push(restriction);
        
        // Notify actor of restriction change
        self.notify_restriction_change(actor_id).await?;
        
        Ok(())
    }
    
    /// Remove movement restriction from actor
    pub async fn remove_restriction(
        &mut self,
        actor_id: ActorId,
        restriction_id: String
    ) -> Result<(), MovementError> {
        if let Some(restrictions) = self.restrictions.get_mut(&actor_id) {
            restrictions.retain(|r| r.restriction_id != restriction_id);
            
            // Notify actor of restriction change
            self.notify_restriction_change(actor_id).await?;
        }
        
        Ok(())
    }
    
    /// Check if actor can move
    pub async fn can_actor_move(
        &self,
        actor_id: ActorId,
        movement_type: MovementActionType,
        target_position: Position
    ) -> Result<bool, MovementError> {
        let restrictions = self.restrictions.get(&actor_id)
            .ok_or_else(|| MovementError::ActorNotFound(actor_id))?;
        
        for restriction in restrictions {
            if !self.restriction_calculator.can_move_with_restriction(
                restriction,
                movement_type,
                target_position
            ).await? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Calculate movement speed with restrictions
    pub async fn calculate_restricted_speed(
        &self,
        actor_id: ActorId,
        base_speed: f64,
        movement_type: MovementActionType
    ) -> Result<f64, MovementError> {
        let restrictions = self.restrictions.get(&actor_id)
            .ok_or_else(|| MovementError::ActorNotFound(actor_id))?;
        
        let mut final_speed = base_speed;
        
        for restriction in restrictions {
            let speed_multiplier = self.restriction_calculator
                .calculate_speed_multiplier(restriction, movement_type).await?;
            final_speed *= speed_multiplier;
        }
        
        Ok(final_speed.max(0.0)) // Speed cannot be negative
    }
}

impl RestrictionCalculator {
    /// Calculate speed multiplier for restriction
    pub async fn calculate_speed_multiplier(
        &self,
        restriction: &MovementRestriction,
        movement_type: MovementActionType
    ) -> Result<f64, MovementError> {
        match restriction.restriction_type {
            MovementRestrictionType::Immobilized => Ok(0.0),
            MovementRestrictionType::Slowed => Ok(1.0 - restriction.magnitude),
            MovementRestrictionType::Rooted => Ok(0.0),
            MovementRestrictionType::Stunned => Ok(0.0),
            MovementRestrictionType::Paralyzed => Ok(0.0),
            MovementRestrictionType::MovementImpairment => Ok(1.0 - restriction.magnitude),
            MovementRestrictionType::TerrainRestriction => {
                // Check if movement type is allowed on this terrain
                self.check_terrain_restriction(restriction, movement_type).await
            },
            MovementRestrictionType::StatusRestriction => {
                // Check if movement type is allowed with this status
                self.check_status_restriction(restriction, movement_type).await
            },
        }
    }
}
```

## 🔧 **Movement Techniques (Thân Pháp)**

### **1. Movement Technique System**

```rust
pub struct MovementTechniqueSystem {
    technique_registry: MovementTechniqueRegistry,
    technique_calculator: MovementTechniqueCalculator,
    technique_effects: MovementTechniqueEffects,
}

pub struct MovementTechnique {
    pub technique_id: String,
    pub technique_name: String,
    pub technique_name_vi: String,
    pub description: String,
    pub description_vi: String,
    pub technique_type: MovementTechniqueType,
    pub element_requirements: Vec<ElementRequirement>,
    pub cultivation_requirements: CultivationRequirement,
    pub technique_properties: MovementTechniqueProperties,
    pub resource_requirements: Vec<ResourceRequirement>,
    pub execution_conditions: Vec<ExecutionCondition>,
    pub effects: Vec<ActionEffect>,
}

pub enum MovementTechniqueType {
    // Basic techniques
    BasicMovement,      // Di chuyển cơ bản
    EnhancedMovement,   // Di chuyển nâng cao
    
    // Elemental techniques
    FireMovement,       // Thân pháp hỏa
    WaterMovement,      // Thân pháp thủy
    EarthMovement,      // Thân pháp thổ
    WoodMovement,       // Thân pháp mộc
    MetalMovement,      // Thân pháp kim
    AirMovement,        // Thân pháp phong
    LightningMovement,  // Thân pháp lôi
    
    // Advanced techniques
    ShadowMovement,     // Thân pháp bóng
    LightMovement,      // Thân pháp quang
    DarkMovement,       // Thân pháp tối
    LifeMovement,       // Thân pháp sinh
    DeathMovement,      // Thân pháp tử
    TimeMovement,       // Thân pháp thời gian
    SpaceMovement,      // Thân pháp không gian
    
    // Special techniques
    Teleportation,      // Dịch chuyển tức thời
    PhaseShift,         // Dịch chuyển pha
    DimensionalWalk,    // Đi bộ chiều không gian
}

pub struct MovementTechniqueProperties {
    pub base_speed_multiplier: f64,
    pub max_distance_multiplier: f64,
    pub movement_duration_multiplier: f64,
    pub cooldown_duration: Duration,
    pub resource_cost_multiplier: f64,
    pub special_effects: Vec<SpecialMovementEffect>,
    pub terrain_bonuses: HashMap<TerrainType, f64>,
    pub element_bonuses: HashMap<String, f64>,
}
```

### **2. Movement Technique Examples**

```yaml
# Fire Movement Technique (Thân Pháp Hỏa)
fire_movement_technique:
  technique_id: "fire_movement_technique"
  technique_name: "Fire Movement Technique"
  technique_name_vi: "Thân Pháp Hỏa"
  description: "A movement technique that uses fire element to enhance movement speed and create fire trails"
  description_vi: "Kỹ thuật di chuyển sử dụng nguyên tố hỏa để tăng tốc độ di chuyển và tạo ra vệt lửa"
  technique_type: "FireMovement"
  element_requirements:
    - element_name: "fire"
      min_mastery: 500
      mastery_multiplier: 0.001
  cultivation_requirements:
    min_level: 30
    min_realm: "Foundation Building"
  technique_properties:
    base_speed_multiplier: 1.5
    max_distance_multiplier: 1.2
    movement_duration_multiplier: 0.8
    cooldown_duration: "10.0s"
    resource_cost_multiplier: 1.2
    special_effects:
      - effect_type: "FireTrail"
        magnitude: 1.0
        duration: "5.0s"
      - effect_type: "FireResistance"
        magnitude: 0.3
        duration: "10.0s"
    terrain_bonuses:
      lava: 2.0
      desert: 1.5
      plains: 1.2
    element_bonuses:
      fire: 1.5
      earth: 1.2
      metal: 1.1
  resource_requirements:
    - resource_type: "Mana"
      min_value: 100
      max_value: 200
    - resource_type: "Qi"
      min_value: 50
      max_value: 100
  execution_conditions:
    - condition: "self.fire_mastery >= 500"
      description: "Must have at least 500 fire mastery"
    - condition: "self.health_percentage > 0.5"
      description: "Must have at least 50% health"

# Water Movement Technique (Thân Pháp Thủy)
water_movement_technique:
  technique_id: "water_movement_technique"
  technique_name: "Water Movement Technique"
  technique_name_vi: "Thân Pháp Thủy"
  description: "A movement technique that uses water element to flow through terrain and create water paths"
  description_vi: "Kỹ thuật di chuyển sử dụng nguyên tố thủy để chảy qua địa hình và tạo ra đường nước"
  technique_type: "WaterMovement"
  element_requirements:
    - element_name: "water"
      min_mastery: 500
      mastery_multiplier: 0.001
  cultivation_requirements:
    min_level: 30
    min_realm: "Foundation Building"
  technique_properties:
    base_speed_multiplier: 1.3
    max_distance_multiplier: 1.5
    movement_duration_multiplier: 0.9
    cooldown_duration: "8.0s"
    resource_cost_multiplier: 1.1
    special_effects:
      - effect_type: "WaterPath"
        magnitude: 1.0
        duration: "8.0s"
      - effect_type: "WaterResistance"
        magnitude: 0.4
        duration: "12.0s"
    terrain_bonuses:
      water: 2.5
      swamp: 2.0
      forest: 1.3
    element_bonuses:
      water: 1.5
      wood: 1.3
      ice: 1.2
  resource_requirements:
    - resource_type: "Mana"
      min_value: 80
      max_value: 160
    - resource_type: "Qi"
      min_value: 40
      max_value: 80
  execution_conditions:
    - condition: "self.water_mastery >= 500"
      description: "Must have at least 500 water mastery"
    - condition: "self.health_percentage > 0.4"
      description: "Must have at least 40% health"
```

## 🔧 **Integration with Crystal Defense Technique**

### **Updated Crystal Defense Technique with Movement Restrictions**

```yaml
# Updated crystal_defense_technique.yaml
movement_properties:
  # Movement restrictions during crystallization
  movement_restrictions:
    - restriction_type: "Immobilized"
      magnitude: 1.0
      duration: "5.0s"
      source: "StatusEffect"
      conditions:
        - condition_type: "StatusEffect"
          condition_value: 1.0
          condition_operator: "EqualTo"
  
  # Movement effects after crystallization
  post_crystallization_effects:
    - effect_type: "MovementFatigue"
      magnitude: 0.5
      duration: "10.0s"
      description: "Reduced movement speed after crystallization ends"
  
  # Movement technique requirements
  movement_technique_requirements:
    - technique_type: "EarthMovement"
      min_level: 5
      required: true
    - technique_type: "MetalMovement"
      min_level: 5
      required: true
```

### **Updated Crystal Defense Technique Implementation**

```rust
// Updated CrystalDefenseTechnique with movement restrictions
impl CrystalDefenseTechnique {
    /// Apply crystallization effects including movement restrictions
    pub async fn apply_crystallization_effects(
        &self,
        actor: &mut Actor,
        movement_system: &mut MovementSystem
    ) -> Result<(), ActionError> {
        // Apply movement restriction
        let movement_restriction = MovementRestriction {
            restriction_id: "crystallization_immobilization".to_string(),
            restriction_type: MovementRestrictionType::Immobilized,
            magnitude: 1.0,
            duration: Duration::from_secs_f64(5.0),
            conditions: vec![],
            source: RestrictionSource::StatusEffect("crystallized".to_string()),
        };
        
        movement_system.apply_restriction(
            actor.get_id(),
            movement_restriction
        ).await?;
        
        // Apply other crystallization effects
        self.apply_defense_bonus(actor).await?;
        self.apply_elemental_resistance(actor).await?;
        self.apply_status_immunity(actor).await?;
        
        Ok(())
    }
    
    /// Remove crystallization effects including movement restrictions
    pub async fn remove_crystallization_effects(
        &self,
        actor: &mut Actor,
        movement_system: &mut MovementSystem
    ) -> Result<(), ActionError> {
        // Remove movement restriction
        movement_system.remove_restriction(
            actor.get_id(),
            "crystallization_immobilization".to_string()
        ).await?;
        
        // Apply movement fatigue
        let fatigue_restriction = MovementRestriction {
            restriction_id: "crystallization_fatigue".to_string(),
            restriction_type: MovementRestrictionType::Slowed,
            magnitude: 0.5,
            duration: Duration::from_secs_f64(10.0),
            conditions: vec![],
            source: RestrictionSource::StatusEffect("crystal_fatigue".to_string()),
        };
        
        movement_system.apply_restriction(
            actor.get_id(),
            fatigue_restriction
        ).await?;
        
        // Remove other crystallization effects
        self.remove_defense_bonus(actor).await?;
        self.remove_elemental_resistance(actor).await?;
        self.remove_status_immunity(actor).await?;
        
        Ok(())
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
    fn test_position_calculation() {
        let position_manager = PositionManager::new();
        let from = Position { x: 0.0, y: 0.0, z: None, world_id: 1, region_id: 1, timestamp: SystemTime::now() };
        let to = Position { x: 3.0, y: 4.0, z: None, world_id: 1, region_id: 1, timestamp: SystemTime::now() };
        
        let delta = position_manager.calculate_position_delta(from, to);
        assert_eq!(delta.dx, 3.0);
        assert_eq!(delta.dy, 4.0);
        assert_eq!(delta.distance, 5.0);
    }
    
    #[test]
    fn test_movement_restriction_application() {
        let mut restriction_manager = MovementRestrictionManager::new();
        let actor_id = ActorId::new();
        let restriction = MovementRestriction {
            restriction_id: "test_immobilization".to_string(),
            restriction_type: MovementRestrictionType::Immobilized,
            magnitude: 1.0,
            duration: Duration::from_secs_f64(5.0),
            conditions: vec![],
            source: RestrictionSource::StatusEffect("test".to_string()),
        };
        
        restriction_manager.apply_restriction(actor_id, restriction).await?;
        
        let can_move = restriction_manager.can_actor_move(
            actor_id,
            MovementActionType::Walk,
            Position::default()
        ).await?;
        
        assert!(!can_move);
    }
}
```

### **2. Integration Tests**

```rust
#[tokio::test]
async fn test_crystal_defense_movement_restrictions() {
    let mut crystal_defense = CrystalDefenseTechnique::new();
    let mut movement_system = MovementSystem::new();
    let mut actor = create_test_actor();
    
    // Apply crystallization
    crystal_defense.apply_crystallization_effects(&mut actor, &mut movement_system).await?;
    
    // Test movement restriction
    let can_move = movement_system.can_actor_move(
        actor.get_id(),
        MovementActionType::Walk,
        Position::default()
    ).await?;
    
    assert!(!can_move);
    
    // Wait for crystallization to end
    tokio::time::sleep(Duration::from_secs_f64(5.1)).await;
    
    // Remove crystallization
    crystal_defense.remove_crystallization_effects(&mut actor, &mut movement_system).await?;
    
    // Test movement after crystallization
    let can_move_after = movement_system.can_actor_move(
        actor.get_id(),
        MovementActionType::Walk,
        Position::default()
    ).await?;
    
    assert!(can_move_after);
}
```

## 📝 **Implementation Notes**

### **1. Performance Considerations**
- **Position Caching**: Cache actor positions for performance
- **Batch Updates**: Process position updates in batches
- **Lazy Calculation**: Calculate movement paths only when needed

### **2. Scalability Considerations**
- **Spatial Indexing**: Use spatial indexing for large numbers of actors
- **Region-based Updates**: Update positions by region
- **Async Processing**: Process movement asynchronously

### **3. Integration Considerations**
- **Element Core**: Movement speed based on element mastery
- **Status Core**: Movement restrictions from status effects
- **Combat Core**: Movement in combat scenarios
- **World Core**: Movement in world environment

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
