# Movement System Derived Stats & Integration

## 📋 **Tổng Quan**

Document này mô tả chi tiết các derived stats của Movement System và cách các hệ thống khác tác động đến Movement System, cũng như cách Movement System tác động đến các hệ thống khác.

## 🔧 **Movement System Derived Stats**

### **1. Status Category System**

```rust
// Status categories for flexible movement restrictions
pub struct StatusCategory {
    pub category_id: String,
    pub category_name: String,
    pub category_name_vi: String,
    pub description: String,
    pub description_vi: String,
    pub status_effects: Vec<String>, // List of status effects in this category
    pub category_type: StatusCategoryType,
    pub base_resistance: f64,
    pub resistance_scaling: f64,
}

pub enum StatusCategoryType {
    MovementRestriction,    // Hạn chế di chuyển
    Control,               // Kiểm soát
    Debuff,                // Debuff
    Buff,                  // Buff
    Transformation,        // Biến đổi
    Environmental,         // Môi trường
    Custom(String),        // Custom category
}

// Example status categories
pub struct StatusCategoryRegistry {
    pub categories: HashMap<String, StatusCategory>,
    pub element_to_categories: HashMap<String, Vec<String>>, // element -> categories
    pub status_to_categories: HashMap<String, Vec<String>>,  // status -> categories
}

impl StatusCategoryRegistry {
    pub fn new() -> Self {
        let mut categories = HashMap::new();
        
        // Movement restriction category
        categories.insert("movement_restriction".to_string(), StatusCategory {
            category_id: "movement_restriction".to_string(),
            category_name: "Movement Restriction".to_string(),
            category_name_vi: "Hạn Chế Di Chuyển".to_string(),
            description: "Status effects that restrict movement".to_string(),
            description_vi: "Hiệu ứng trạng thái hạn chế di chuyển".to_string(),
            status_effects: vec![
                "immobilized".to_string(),
                "rooted".to_string(),
                "slowed".to_string(),
                "stunned".to_string(),
                "paralyzed".to_string(),
                "frozen".to_string(),
                "entangled".to_string(),
            ],
            category_type: StatusCategoryType::MovementRestriction,
            base_resistance: 0.0,
            resistance_scaling: 0.01,
        });
        
        // Control category
        categories.insert("control".to_string(), StatusCategory {
            category_id: "control".to_string(),
            category_name: "Control".to_string(),
            category_name_vi: "Kiểm Soát".to_string(),
            description: "Status effects that control the target".to_string(),
            description_vi: "Hiệu ứng trạng thái kiểm soát mục tiêu".to_string(),
            status_effects: vec![
                "charmed".to_string(),
                "confused".to_string(),
                "fear".to_string(),
                "sleep".to_string(),
                "mesmerized".to_string(),
            ],
            category_type: StatusCategoryType::Control,
            base_resistance: 0.0,
            resistance_scaling: 0.01,
        });
        
        // Debuff category
        categories.insert("debuff".to_string(), StatusCategory {
            category_id: "debuff".to_string(),
            category_name: "Debuff".to_string(),
            category_name_vi: "Debuff".to_string(),
            description: "Negative status effects".to_string(),
            description_vi: "Hiệu ứng trạng thái tiêu cực".to_string(),
            status_effects: vec![
                "poisoned".to_string(),
                "burned".to_string(),
                "bleeding".to_string(),
                "cursed".to_string(),
                "weakened".to_string(),
            ],
            category_type: StatusCategoryType::Debuff,
            base_resistance: 0.0,
            resistance_scaling: 0.01,
        });
        
        // Transformation category
        categories.insert("transformation".to_string(), StatusCategory {
            category_id: "transformation".to_string(),
            category_name: "Transformation".to_string(),
            category_name_vi: "Biến Đổi".to_string(),
            description: "Status effects that transform the target".to_string(),
            description_vi: "Hiệu ứng trạng thái biến đổi mục tiêu".to_string(),
            status_effects: vec![
                "crystallized".to_string(),
                "petrified".to_string(),
                "shapeshifted".to_string(),
                "polymorphed".to_string(),
            ],
            category_type: StatusCategoryType::Transformation,
            base_resistance: 0.0,
            resistance_scaling: 0.01,
        });
        
        Self {
            categories,
            element_to_categories: HashMap::new(),
            status_to_categories: HashMap::new(),
        }
    }
}
```

### **2. Core Movement Stats**

```rust
pub struct MovementDerivedStats {
    // Basic movement stats
    pub movement_speed: f64,              // Tốc độ di chuyển cơ bản
    pub movement_acceleration: f64,       // Gia tốc di chuyển
    pub movement_deceleration: f64,       // Giảm tốc di chuyển
    pub movement_turn_rate: f64,          // Tốc độ quay đầu
    
    // Advanced movement stats
    pub teleportation_ability: f64,       // Khả năng dịch chuyển tức thời
    pub flight_ability: f64,              // Khả năng bay
    pub swimming_ability: f64,            // Khả năng bơi
    pub climbing_ability: f64,            // Khả năng leo trèo
    pub phase_ability: f64,               // Khả năng xuyên qua vật thể
    
    // Movement restrictions (flexible status lists)
    pub movement_restriction_resistance: f64,  // Kháng hạn chế di chuyển
    pub status_resistance: HashMap<String, f64>, // Kháng status effects (flexible)
    pub status_category_resistance: HashMap<String, f64>, // Kháng status categories (flexible)
    
    // Movement techniques (thân pháp)
    pub movement_technique_mastery: f64,       // Tinh thông thân pháp
    pub movement_technique_efficiency: f64,    // Hiệu quả thân pháp
    pub movement_technique_cooldown_reduction: f64, // Giảm cooldown thân pháp
    
    // Terrain adaptation
    pub terrain_adaptation: HashMap<TerrainType, f64>, // Thích ứng địa hình
    pub weather_adaptation: HashMap<WeatherType, f64>, // Thích ứng thời tiết
    
    // Elemental movement (flexible element support)
    pub elemental_movement_bonus: HashMap<String, f64>, // Bonus di chuyển theo nguyên tố
    pub elemental_movement_resistance: HashMap<String, f64>, // Kháng di chuyển nguyên tố
    
    // Custom movement stats (extensible)
    pub custom_movement_stats: HashMap<String, f64>, // Custom movement stats
}
```

### **2. Movement Technique Stats**

```rust
pub struct MovementTechniqueStats {
    // Elemental technique mastery (flexible element support)
    pub elemental_technique_mastery: HashMap<String, f64>, // Thân pháp theo nguyên tố (flexible)
    pub elemental_technique_efficiency: HashMap<String, f64>, // Hiệu quả thân pháp theo nguyên tố
    
    // Advanced technique mastery (flexible)
    pub advanced_technique_mastery: HashMap<String, f64>, // Thân pháp nâng cao (flexible)
    pub advanced_technique_efficiency: HashMap<String, f64>, // Hiệu quả thân pháp nâng cao
    
    // Technique effectiveness
    pub technique_speed_multiplier: f64,  // Hệ số tốc độ kỹ thuật
    pub technique_distance_multiplier: f64, // Hệ số khoảng cách kỹ thuật
    pub technique_duration_multiplier: f64, // Hệ số thời gian kỹ thuật
    pub technique_resource_efficiency: f64, // Hiệu quả tài nguyên kỹ thuật
    
    // Custom technique stats (extensible)
    pub custom_technique_stats: HashMap<String, f64>, // Custom technique stats
}

// Movement technique registry for flexible element support
pub struct MovementTechniqueRegistry {
    pub elemental_techniques: HashMap<String, ElementalMovementTechnique>,
    pub advanced_techniques: HashMap<String, AdvancedMovementTechnique>,
    pub custom_techniques: HashMap<String, CustomMovementTechnique>,
}

pub struct ElementalMovementTechnique {
    pub technique_id: String,
    pub technique_name: String,
    pub technique_name_vi: String,
    pub element_id: String,
    pub element_category: String,
    pub base_mastery: f64,
    pub mastery_scaling: f64,
    pub efficiency_scaling: f64,
    pub resource_cost: f64,
    pub cooldown: Duration,
    pub range: f64,
    pub speed_multiplier: f64,
    pub distance_multiplier: f64,
    pub duration_multiplier: f64,
    pub special_effects: Vec<String>,
}

pub struct AdvancedMovementTechnique {
    pub technique_id: String,
    pub technique_name: String,
    pub technique_name_vi: String,
    pub technique_type: AdvancedTechniqueType,
    pub base_mastery: f64,
    pub mastery_scaling: f64,
    pub efficiency_scaling: f64,
    pub resource_cost: f64,
    pub cooldown: Duration,
    pub range: f64,
    pub speed_multiplier: f64,
    pub distance_multiplier: f64,
    pub duration_multiplier: f64,
    pub special_effects: Vec<String>,
}

pub enum AdvancedTechniqueType {
    Teleportation,      // Dịch chuyển tức thời
    PhaseShift,         // Xuyên qua vật thể
    DimensionalWalk,    // Đi bộ không gian
    TimeDilation,       // Giãn nở thời gian
    ShadowStep,         // Bước bóng
    LightSpeed,         // Tốc độ ánh sáng
    Custom(String),     // Custom technique
}

pub struct CustomMovementTechnique {
    pub technique_id: String,
    pub technique_name: String,
    pub technique_name_vi: String,
    pub technique_type: String,
    pub base_mastery: f64,
    pub mastery_scaling: f64,
    pub efficiency_scaling: f64,
    pub resource_cost: f64,
    pub cooldown: Duration,
    pub range: f64,
    pub speed_multiplier: f64,
    pub distance_multiplier: f64,
    pub duration_multiplier: f64,
    pub special_effects: Vec<String>,
    pub custom_properties: HashMap<String, f64>,
}

impl MovementTechniqueRegistry {
    pub fn new() -> Self {
        let mut elemental_techniques = HashMap::new();
        let mut advanced_techniques = HashMap::new();
        let mut custom_techniques = HashMap::new();
        
        // Example elemental techniques
        elemental_techniques.insert("fire_dash".to_string(), ElementalMovementTechnique {
            technique_id: "fire_dash".to_string(),
            technique_name: "Fire Dash".to_string(),
            technique_name_vi: "Thân Pháp Hỏa".to_string(),
            element_id: "fire".to_string(),
            element_category: "elemental".to_string(),
            base_mastery: 100.0,
            mastery_scaling: 0.01,
            efficiency_scaling: 0.005,
            resource_cost: 50.0,
            cooldown: Duration::from_secs_f64(5.0),
            range: 10.0,
            speed_multiplier: 2.0,
            distance_multiplier: 1.5,
            duration_multiplier: 1.0,
            special_effects: vec!["burn_effect".to_string(), "speed_boost".to_string()],
        });
        
        elemental_techniques.insert("water_flow".to_string(), ElementalMovementTechnique {
            technique_id: "water_flow".to_string(),
            technique_name: "Water Flow".to_string(),
            technique_name_vi: "Thân Pháp Thủy".to_string(),
            element_id: "water".to_string(),
            element_category: "elemental".to_string(),
            base_mastery: 100.0,
            mastery_scaling: 0.01,
            efficiency_scaling: 0.005,
            resource_cost: 40.0,
            cooldown: Duration::from_secs_f64(4.0),
            range: 8.0,
            speed_multiplier: 1.8,
            distance_multiplier: 2.0,
            duration_multiplier: 1.2,
            special_effects: vec!["swimming_boost".to_string(), "flow_effect".to_string()],
        });
        
        // Example advanced techniques
        advanced_techniques.insert("teleportation".to_string(), AdvancedMovementTechnique {
            technique_id: "teleportation".to_string(),
            technique_name: "Teleportation".to_string(),
            technique_name_vi: "Dịch Chuyển Tức Thời".to_string(),
            technique_type: AdvancedTechniqueType::Teleportation,
            base_mastery: 500.0,
            mastery_scaling: 0.005,
            efficiency_scaling: 0.002,
            resource_cost: 200.0,
            cooldown: Duration::from_secs_f64(10.0),
            range: 50.0,
            speed_multiplier: 10.0,
            distance_multiplier: 5.0,
            duration_multiplier: 0.1,
            special_effects: vec!["instant_movement".to_string(), "bypass_obstacles".to_string()],
        });
        
        advanced_techniques.insert("shadow_step".to_string(), AdvancedMovementTechnique {
            technique_id: "shadow_step".to_string(),
            technique_name: "Shadow Step".to_string(),
            technique_name_vi: "Bước Bóng".to_string(),
            technique_type: AdvancedTechniqueType::ShadowStep,
            base_mastery: 300.0,
            mastery_scaling: 0.008,
            efficiency_scaling: 0.003,
            resource_cost: 150.0,
            cooldown: Duration::from_secs_f64(8.0),
            range: 15.0,
            speed_multiplier: 3.0,
            distance_multiplier: 2.5,
            duration_multiplier: 0.5,
            special_effects: vec!["stealth_effect".to_string(), "phase_through".to_string()],
        });
        
        Self {
            elemental_techniques,
            advanced_techniques,
            custom_techniques,
        }
    }
}
```

## 🔧 **Flexible Status Resistance Calculation**

```rust
impl MovementDerivedStats {
    /// Calculate status resistance for a specific status effect
    pub fn calculate_status_resistance(
        &self,
        status_id: &str,
        status_category: Option<&str>,
        base_resistance: f64
    ) -> f64 {
        let mut total_resistance = base_resistance;
        
        // Add specific status resistance
        if let Some(resistance) = self.status_resistance.get(status_id) {
            total_resistance += resistance;
        }
        
        // Add category resistance
        if let Some(category) = status_category {
            if let Some(category_resistance) = self.status_category_resistance.get(category) {
                total_resistance += category_resistance;
            }
        }
        
        // Add movement restriction resistance
        total_resistance += self.movement_restriction_resistance;
        
        total_resistance
    }
    
    /// Calculate status resistance for multiple status effects
    pub fn calculate_multiple_status_resistance(
        &self,
        status_effects: &[String],
        status_categories: &[String],
        base_resistance: f64
    ) -> f64 {
        let mut total_resistance = base_resistance;
        
        // Add resistance for each status effect
        for status_id in status_effects {
            if let Some(resistance) = self.status_resistance.get(status_id) {
                total_resistance += resistance;
            }
        }
        
        // Add resistance for each category
        for category in status_categories {
            if let Some(category_resistance) = self.status_category_resistance.get(category) {
                total_resistance += category_resistance;
            }
        }
        
        // Add movement restriction resistance
        total_resistance += self.movement_restriction_resistance;
        
        total_resistance
    }
    
    /// Add new status resistance
    pub fn add_status_resistance(&mut self, status_id: String, resistance: f64) {
        self.status_resistance.insert(status_id, resistance);
    }
    
    /// Add new status category resistance
    pub fn add_status_category_resistance(&mut self, category: String, resistance: f64) {
        self.status_category_resistance.insert(category, resistance);
    }
    
    /// Remove status resistance
    pub fn remove_status_resistance(&mut self, status_id: &str) {
        self.status_resistance.remove(status_id);
    }
    
    /// Remove status category resistance
    pub fn remove_status_category_resistance(&mut self, category: &str) {
        self.status_category_resistance.remove(category);
    }
}

impl MovementTechniqueStats {
    /// Calculate elemental technique mastery for a specific element
    pub fn calculate_elemental_technique_mastery(
        &self,
        element_id: &str,
        base_mastery: f64
    ) -> f64 {
        let technique_mastery = self.elemental_technique_mastery
            .get(element_id)
            .copied()
            .unwrap_or(0.0);
        
        base_mastery + technique_mastery
    }
    
    /// Calculate advanced technique mastery for a specific technique
    pub fn calculate_advanced_technique_mastery(
        &self,
        technique_id: &str,
        base_mastery: f64
    ) -> f64 {
        let technique_mastery = self.advanced_technique_mastery
            .get(technique_id)
            .copied()
            .unwrap_or(0.0);
        
        base_mastery + technique_mastery
    }
    
    /// Add new elemental technique mastery
    pub fn add_elemental_technique_mastery(&mut self, element_id: String, mastery: f64) {
        self.elemental_technique_mastery.insert(element_id, mastery);
    }
    
    /// Add new advanced technique mastery
    pub fn add_advanced_technique_mastery(&mut self, technique_id: String, mastery: f64) {
        self.advanced_technique_mastery.insert(technique_id, mastery);
    }
    
    /// Add custom technique stat
    pub fn add_custom_technique_stat(&mut self, stat_name: String, value: f64) {
        self.custom_technique_stats.insert(stat_name, value);
    }
}
```

## 🔗 **System Integration - Input (Các Hệ Thống Tác Động Đến Movement)**

### **1. Element Core Integration**

#### **A. Element Mastery Effects**
```rust
// Fire element mastery affects movement
fn calculate_fire_movement_bonus(
    fire_mastery: f64,
    base_movement_speed: f64
) -> f64 {
    let mastery_bonus = fire_mastery * 0.0001; // 0.01% per mastery point
    let speed_bonus = base_movement_speed * mastery_bonus;
    
    // Fire provides speed boost
    speed_bonus * 1.2 // 20% bonus for fire
}

// Water element mastery affects swimming
fn calculate_water_swimming_bonus(
    water_mastery: f64,
    base_swimming_ability: f64
) -> f64 {
    let mastery_bonus = water_mastery * 0.0002; // 0.02% per mastery point
    let swimming_bonus = base_swimming_ability * mastery_bonus;
    
    // Water provides swimming boost
    swimming_bonus * 1.5 // 50% bonus for water
}

// Earth element mastery affects terrain adaptation
fn calculate_earth_terrain_bonus(
    earth_mastery: f64,
    terrain_type: TerrainType
) -> f64 {
    let mastery_bonus = earth_mastery * 0.0001;
    
    match terrain_type {
        TerrainType::Ground => mastery_bonus * 2.0,    // 200% bonus on ground
        TerrainType::Mountain => mastery_bonus * 1.5,  // 150% bonus on mountain
        TerrainType::Desert => mastery_bonus * 1.2,    // 120% bonus on desert
        _ => mastery_bonus * 0.5,                      // 50% bonus on other terrains
    }
}
```

#### **B. Element Category Effects**
```rust
// Physical category affects movement
fn calculate_physical_movement_bonus(
    physical_category_mastery: f64,
    base_movement_speed: f64
) -> f64 {
    let category_bonus = physical_category_mastery * 0.0001;
    base_movement_speed * category_bonus * 1.1 // 10% bonus for physical category
}

// Elemental category affects movement techniques
fn calculate_elemental_technique_bonus(
    elemental_category_mastery: f64,
    technique_mastery: f64
) -> f64 {
    let category_bonus = elemental_category_mastery * 0.0001;
    technique_mastery * (1.0 + category_bonus * 0.5) // 50% of category bonus
}
```

### **2. Cultivation Core Integration**

#### **A. Cultivation Level Effects**
```rust
// Cultivation level affects movement
fn calculate_cultivation_movement_bonus(
    cultivation_level: f64,
    cultivation_realm: u32,
    base_movement_speed: f64
) -> f64 {
    let level_bonus = cultivation_level * 0.01; // 1% per level
    let realm_bonus = cultivation_realm as f64 * 0.05; // 5% per realm
    
    base_movement_speed * (1.0 + level_bonus + realm_bonus)
}

// Cultivation affects movement techniques
fn calculate_cultivation_technique_bonus(
    cultivation_level: f64,
    technique_mastery: f64
) -> f64 {
    let cultivation_bonus = cultivation_level * 0.0001;
    technique_mastery * (1.0 + cultivation_bonus)
}
```

#### **B. Cultivation Realm Effects**
```rust
// Different realms provide different movement bonuses
fn calculate_realm_movement_bonus(
    realm: CultivationRealm,
    base_movement_speed: f64
) -> f64 {
    match realm {
        CultivationRealm::FoundationBuilding => base_movement_speed * 1.1,  // 10% bonus
        CultivationRealm::GoldenCore => base_movement_speed * 1.3,         // 30% bonus
        CultivationRealm::NascentSoul => base_movement_speed * 1.6,        // 60% bonus
        CultivationRealm::SpiritRefining => base_movement_speed * 2.0,     // 100% bonus
        CultivationRealm::SpiritTransformation => base_movement_speed * 2.5, // 150% bonus
        CultivationRealm::SpiritIntegration => base_movement_speed * 3.0,   // 200% bonus
        CultivationRealm::SpiritTranscendence => base_movement_speed * 4.0, // 300% bonus
    }
}
```

### **3. Item/Equipment Integration**

#### **A. Equipment Effects**
```rust
// Equipment affects movement
fn calculate_equipment_movement_bonus(
    equipment: &Equipment,
    base_movement_speed: f64
) -> f64 {
    let mut total_bonus = 0.0;
    
    // Boots provide movement speed
    if let Some(boots) = equipment.get_boots() {
        total_bonus += boots.movement_speed_bonus;
    }
    
    // Armor affects movement (heavier = slower)
    if let Some(armor) = equipment.get_armor() {
        total_bonus -= armor.weight_penalty * 0.1; // 10% penalty per weight unit
    }
    
    // Accessories provide movement bonuses
    for accessory in equipment.get_accessories() {
        total_bonus += accessory.movement_bonus;
    }
    
    base_movement_speed * (1.0 + total_bonus)
}

// Equipment affects movement techniques
fn calculate_equipment_technique_bonus(
    equipment: &Equipment,
    technique_mastery: f64
) -> f64 {
    let mut total_bonus = 0.0;
    
    // Special equipment provides technique bonuses
    for item in equipment.get_all_items() {
        if let Some(technique_bonus) = item.movement_technique_bonus {
            total_bonus += technique_bonus;
        }
    }
    
    technique_mastery * (1.0 + total_bonus)
}
```

#### **B. Mount Effects**
```rust
// Mount affects movement
fn calculate_mount_movement_bonus(
    mount: &Mount,
    base_movement_speed: f64
) -> f64 {
    let mount_speed = mount.base_speed;
    let mount_bonus = mount.speed_multiplier;
    
    base_movement_speed * mount_bonus + mount_speed
}

// Mount affects movement techniques
fn calculate_mount_technique_bonus(
    mount: &Mount,
    technique_mastery: f64
) -> f64 {
    let mount_technique_bonus = mount.technique_bonus;
    technique_mastery * (1.0 + mount_technique_bonus)
}
```

### **4. Talent/Innate Integration**

#### **A. Innate Talents**
```rust
// Innate talents affect movement
fn calculate_innate_movement_bonus(
    talents: &[InnateTalent],
    base_movement_speed: f64
) -> f64 {
    let mut total_bonus = 0.0;
    
    for talent in talents {
        match talent.talent_type {
            TalentType::SwiftFeet => total_bonus += 0.2,        // 20% speed bonus
            TalentType::WindWalker => total_bonus += 0.3,       // 30% speed bonus
            TalentType::ShadowStep => total_bonus += 0.25,      // 25% speed bonus
            TalentType::EarthStride => total_bonus += 0.15,     // 15% speed bonus
            TalentType::WaterFlow => total_bonus += 0.18,       // 18% speed bonus
            TalentType::FireDash => total_bonus += 0.22,        // 22% speed bonus
            _ => {} // Other talents don't affect movement
        }
    }
    
    base_movement_speed * (1.0 + total_bonus)
}

// Innate talents affect movement techniques
fn calculate_innate_technique_bonus(
    talents: &[InnateTalent],
    technique_mastery: f64
) -> f64 {
    let mut total_bonus = 0.0;
    
    for talent in talents {
        if talent.affects_movement_techniques {
            total_bonus += talent.technique_bonus;
        }
    }
    
    technique_mastery * (1.0 + total_bonus)
}
```

## 🔗 **System Integration - Output (Movement Tác Động Đến Các Hệ Thống Khác)**

### **1. Combat System Integration**

#### **A. Movement in Combat**
```rust
// Movement affects combat positioning
fn calculate_combat_positioning_bonus(
    movement_speed: f64,
    base_accuracy: f64
) -> f64 {
    let positioning_bonus = movement_speed * 0.001; // 0.1% per movement speed
    base_accuracy * (1.0 + positioning_bonus)
}

// Movement affects dodge chance
fn calculate_movement_dodge_bonus(
    movement_speed: f64,
    base_dodge_rate: f64
) -> f64 {
    let dodge_bonus = movement_speed * 0.0005; // 0.05% per movement speed
    base_dodge_rate + dodge_bonus
}

// Movement affects attack speed
fn calculate_movement_attack_speed_bonus(
    movement_speed: f64,
    base_attack_speed: f64
) -> f64 {
    let attack_speed_bonus = movement_speed * 0.0002; // 0.02% per movement speed
    base_attack_speed * (1.0 + attack_speed_bonus)
}
```

#### **B. Movement Techniques in Combat**
```rust
// Fire movement technique provides combat bonuses
fn calculate_fire_movement_combat_bonus(
    fire_movement_mastery: f64,
    base_damage: f64
) -> f64 {
    let technique_bonus = fire_movement_mastery * 0.0001;
    base_damage * (1.0 + technique_bonus * 0.5) // 50% of technique bonus
}

// Water movement technique provides defense bonuses
fn calculate_water_movement_defense_bonus(
    water_movement_mastery: f64,
    base_defense: f64
) -> f64 {
    let technique_bonus = water_movement_mastery * 0.0001;
    base_defense * (1.0 + technique_bonus * 0.3) // 30% of technique bonus
}
```

### **2. Status System Integration**

#### **A. Movement Restrictions**
```rust
// Movement restrictions affect status effects
fn calculate_movement_restriction_status_penalty(
    movement_restrictions: &[MovementRestriction],
    base_status_resistance: f64
) -> f64 {
    let mut penalty = 0.0;
    
    for restriction in movement_restrictions {
        match restriction.restriction_type {
            MovementRestrictionType::Immobilized => penalty += 0.3,  // 30% penalty
            MovementRestrictionType::Slowed => penalty += 0.1,       // 10% penalty
            MovementRestrictionType::Rooted => penalty += 0.2,       // 20% penalty
            MovementRestrictionType::Stunned => penalty += 0.4,      // 40% penalty
            _ => {} // Other restrictions don't affect status resistance
        }
    }
    
    base_status_resistance * (1.0 - penalty)
}
```

#### **B. Movement Techniques and Status Effects**
```rust
// Movement techniques can provide status immunity
fn calculate_movement_technique_status_immunity(
    technique_mastery: f64,
    technique_type: MovementTechniqueType
) -> Vec<String> {
    let mut immunities = Vec::new();
    
    match technique_type {
        MovementTechniqueType::FireMovement => {
            if technique_mastery > 1000.0 {
                immunities.push("burn_immunity".to_string());
            }
        },
        MovementTechniqueType::WaterMovement => {
            if technique_mastery > 1000.0 {
                immunities.push("freeze_immunity".to_string());
            }
        },
        MovementTechniqueType::ShadowMovement => {
            if technique_mastery > 1500.0 {
                immunities.push("stun_immunity".to_string());
                immunities.push("root_immunity".to_string());
            }
        },
        _ => {} // Other techniques don't provide immunities
    }
    
    immunities
}
```

### **3. Resource System Integration**

#### **A. Movement Resource Consumption**
```rust
// Movement consumes resources
fn calculate_movement_resource_consumption(
    movement_speed: f64,
    distance: f64,
    base_stamina_cost: f64
) -> f64 {
    let speed_multiplier = 1.0 + (movement_speed - 1.0) * 0.5; // 50% more cost per speed unit
    let distance_cost = distance * base_stamina_cost;
    
    distance_cost * speed_multiplier
}

// Movement techniques consume more resources
fn calculate_technique_resource_consumption(
    technique_mastery: f64,
    technique_type: MovementTechniqueType,
    base_resource_cost: f64
) -> f64 {
    let mastery_multiplier = 1.0 + technique_mastery * 0.0001;
    let technique_multiplier = match technique_type {
        MovementTechniqueType::Teleportation => 3.0,      // 300% cost
        MovementTechniqueType::PhaseShift => 2.5,         // 250% cost
        MovementTechniqueType::DimensionalWalk => 4.0,    // 400% cost
        _ => 1.5, // 150% cost for other techniques
    };
    
    base_resource_cost * mastery_multiplier * technique_multiplier
}
```

#### **B. Movement Resource Regeneration**
```rust
// Movement affects resource regeneration
fn calculate_movement_resource_regeneration(
    movement_speed: f64,
    base_regeneration: f64
) -> f64 {
    // Faster movement reduces regeneration
    let speed_penalty = (movement_speed - 1.0) * 0.1; // 10% penalty per speed unit
    base_regeneration * (1.0 - speed_penalty)
}
```

## 🎯 **Crystal Defense Technique Integration Examples**

### **1. Crystal Defense Status Category Integration**

```rust
// Crystal Defense Technique với flexible status categories
impl CrystalDefenseTechnique {
    pub async fn apply_crystallization_status_with_categories(
        &self,
        actor: &mut Actor,
        movement_system: &mut MovementSystem,
        status_category_registry: &StatusCategoryRegistry
    ) -> Result<(), ActionError> {
        // Get transformation category
        let transformation_category = status_category_registry
            .categories
            .get("transformation")
            .ok_or(ActionError::CategoryNotFound("transformation".to_string()))?;
        
        // Check if crystallized is in transformation category
        if transformation_category.status_effects.contains(&"crystallized".to_string()) {
            // Apply crystallization with category-based resistance
            let category_resistance = actor.get_movement_derived_stats()
                .status_category_resistance
                .get("transformation")
                .copied()
                .unwrap_or(0.0);
            
            // Apply movement restrictions
            let immobilization = MovementRestriction {
                restriction_id: "crystallization_immobilization".to_string(),
                restriction_type: MovementRestrictionType::Immobilized,
                magnitude: 1.0,
                duration: Duration::from_secs_f64(5.0),
                source: RestrictionSource::StatusEffect("crystallized".to_string()),
                conditions: vec![],
            };
            
            movement_system.apply_restriction(actor.get_id(), immobilization).await?;
            
            // Set movement speed to 0
            actor.set_movement_speed(0.0);
            
            // Apply category-based resistance bonus
            let resistance_bonus = category_resistance * 0.5; // 50% of category resistance
            actor.add_status_resistance("crystallized".to_string(), resistance_bonus);
            
            Ok(())
        } else {
            Err(ActionError::InvalidStatusCategory("crystallized not in transformation category".to_string()))
        }
    }
}
```

### **2. Crystal Defense Flexible Element Support**

```rust
// Crystal Defense Technique với flexible element support
impl CrystalDefenseTechnique {
    pub async fn apply_crystallization_with_elements(
        &self,
        actor: &mut Actor,
        movement_system: &mut MovementSystem,
        movement_technique_registry: &MovementTechniqueRegistry
    ) -> Result<(), ActionError> {
        // Get crystal movement technique
        let crystal_technique = movement_technique_registry
            .custom_techniques
            .get("crystal_movement")
            .ok_or(ActionError::TechniqueNotFound("crystal_movement".to_string()))?;
        
        // Calculate elemental technique mastery for earth and metal
        let earth_mastery = actor.get_movement_technique_stats()
            .elemental_technique_mastery
            .get("earth")
            .copied()
            .unwrap_or(0.0);
        
        let metal_mastery = actor.get_movement_technique_stats()
            .elemental_technique_mastery
            .get("metal")
            .copied()
            .unwrap_or(0.0);
        
        // Calculate combined mastery
        let combined_mastery = (earth_mastery + metal_mastery) / 2.0;
        
        // Apply crystallization with mastery-based effects
        if combined_mastery >= crystal_technique.base_mastery {
            // Apply movement restrictions
            let immobilization = MovementRestriction {
                restriction_id: "crystallization_immobilization".to_string(),
                restriction_type: MovementRestrictionType::Immobilized,
                magnitude: 1.0,
                duration: Duration::from_secs_f64(5.0),
                source: RestrictionSource::StatusEffect("crystallized".to_string()),
                conditions: vec![],
            };
            
            movement_system.apply_restriction(actor.get_id(), immobilization).await?;
            
            // Set movement speed to 0
            actor.set_movement_speed(0.0);
            
            // Apply mastery-based defense bonus
            let mastery_bonus = combined_mastery * crystal_technique.mastery_scaling;
            let defense_multiplier = crystal_technique.custom_properties
                .get("defense_multiplier")
                .copied()
                .unwrap_or(20.0);
            
            let final_defense_multiplier = defense_multiplier * (1.0 + mastery_bonus);
            actor.set_defense_multiplier(final_defense_multiplier);
            
            Ok(())
        } else {
            Err(ActionError::InsufficientMastery(format!(
                "Required mastery: {}, Current mastery: {}",
                crystal_technique.base_mastery, combined_mastery
            )))
        }
    }
}
```

### **3. Crystal Defense Movement Restrictions**

```rust
// Crystal Defense Technique immobilizes user
impl CrystalDefenseTechnique {
    pub async fn apply_movement_restrictions(
        &self,
        actor: &mut Actor,
        movement_system: &mut MovementSystem
    ) -> Result<(), ActionError> {
        // Apply immobilization restriction
        let immobilization = MovementRestriction {
            restriction_id: "crystallization_immobilization".to_string(),
            restriction_type: MovementRestrictionType::Immobilized,
            magnitude: 1.0,
            duration: Duration::from_secs_f64(5.0),
            source: RestrictionSource::StatusEffect("crystallized".to_string()),
            conditions: vec![],
        };
        
        movement_system.apply_restriction(actor.get_id(), immobilization).await?;
        
        // Set movement speed to 0
        actor.set_movement_speed(0.0);
        
        // Apply movement technique cooldown
        actor.set_movement_technique_cooldown(Duration::from_secs_f64(5.0));
        
        Ok(())
    }
    
    pub async fn remove_movement_restrictions(
        &self,
        actor: &mut Actor,
        movement_system: &mut MovementSystem
    ) -> Result<(), ActionError> {
        // Remove immobilization restriction
        movement_system.remove_restriction(
            actor.get_id(),
            "crystallization_immobilization".to_string()
        ).await?;
        
        // Restore movement speed
        let base_movement_speed = actor.get_base_movement_speed();
        let derived_stats = actor.get_derived_stats();
        let final_speed = self.calculate_final_movement_speed(base_movement_speed, &derived_stats);
        actor.set_movement_speed(final_speed);
        
        // Apply movement fatigue
        let fatigue_restriction = MovementRestriction {
            restriction_id: "crystallization_fatigue".to_string(),
            restriction_type: MovementRestrictionType::Slowed,
            magnitude: 0.5, // 50% speed reduction
            duration: Duration::from_secs_f64(10.0),
            source: RestrictionSource::StatusEffect("crystal_fatigue".to_string()),
            conditions: vec![],
        };
        
        movement_system.apply_restriction(actor.get_id(), fatigue_restriction).await?;
        
        Ok(())
    }
}
```

### **2. Crystal Defense Movement Effects on Other Systems**

```rust
// Crystal Defense affects combat positioning
fn calculate_crystal_defense_combat_effects(
    actor: &Actor,
    base_accuracy: f64,
    base_dodge_rate: f64
) -> (f64, f64) {
    // Immobilized = better accuracy but worse dodge
    let accuracy_bonus = 0.2; // 20% accuracy bonus (stationary = more accurate)
    let dodge_penalty = -0.5; // 50% dodge penalty (can't dodge when immobilized)
    
    let final_accuracy = base_accuracy * (1.0 + accuracy_bonus);
    let final_dodge_rate = (base_dodge_rate + dodge_penalty).max(0.0);
    
    (final_accuracy, final_dodge_rate)
}

// Crystal Defense affects resource regeneration
fn calculate_crystal_defense_resource_effects(
    actor: &Actor,
    base_regeneration: f64
) -> f64 {
    // Immobilized = better resource regeneration (can focus on cultivation)
    let regeneration_bonus = 0.3; // 30% regeneration bonus
    base_regeneration * (1.0 + regeneration_bonus)
}

// Crystal Defense affects status resistance
fn calculate_crystal_defense_status_effects(
    actor: &Actor,
    base_status_resistance: f64
) -> f64 {
    // Crystallized = better status resistance (crystal form is resistant)
    let resistance_bonus = 0.5; // 50% resistance bonus
    base_status_resistance * (1.0 + resistance_bonus)
}
```

## 📝 **YAML Configuration Examples**

### **1. Status Category Configuration**

```yaml
# status_categories.yaml
status_categories:
  movement_restriction:
    category_id: "movement_restriction"
    category_name: "Movement Restriction"
    category_name_vi: "Hạn Chế Di Chuyển"
    description: "Status effects that restrict movement"
    description_vi: "Hiệu ứng trạng thái hạn chế di chuyển"
    status_effects:
      - "immobilized"
      - "rooted"
      - "slowed"
      - "stunned"
      - "paralyzed"
      - "frozen"
      - "entangled"
    category_type: "MovementRestriction"
    base_resistance: 0.0
    resistance_scaling: 0.01
  
  control:
    category_id: "control"
    category_name: "Control"
    category_name_vi: "Kiểm Soát"
    description: "Status effects that control the target"
    description_vi: "Hiệu ứng trạng thái kiểm soát mục tiêu"
    status_effects:
      - "charmed"
      - "confused"
      - "fear"
      - "sleep"
      - "mesmerized"
    category_type: "Control"
    base_resistance: 0.0
    resistance_scaling: 0.01
  
  transformation:
    category_id: "transformation"
    category_name: "Transformation"
    category_name_vi: "Biến Đổi"
    description: "Status effects that transform the target"
    description_vi: "Hiệu ứng trạng thái biến đổi mục tiêu"
    status_effects:
      - "crystallized"
      - "petrified"
      - "shapeshifted"
      - "polymorphed"
    category_type: "Transformation"
    base_resistance: 0.0
    resistance_scaling: 0.01

# Element to category mappings
element_category_mappings:
  fire:
    element_id: "fire"
    categories: ["movement_restriction", "debuff"]
    category_weights:
      movement_restriction: 0.3
      debuff: 0.7
  
  water:
    element_id: "water"
    categories: ["movement_restriction", "control"]
    category_weights:
      movement_restriction: 0.4
      control: 0.6
  
  earth:
    element_id: "earth"
    categories: ["movement_restriction", "transformation"]
    category_weights:
      movement_restriction: 0.5
      transformation: 0.5
```

### **2. Movement Technique Configuration**

```yaml
# movement_techniques.yaml
elemental_techniques:
  fire_dash:
    technique_id: "fire_dash"
    technique_name: "Fire Dash"
    technique_name_vi: "Thân Pháp Hỏa"
    element_id: "fire"
    element_category: "elemental"
    base_mastery: 100.0
    mastery_scaling: 0.01
    efficiency_scaling: 0.005
    resource_cost: 50.0
    cooldown: 5.0
    range: 10.0
    speed_multiplier: 2.0
    distance_multiplier: 1.5
    duration_multiplier: 1.0
    special_effects:
      - "burn_effect"
      - "speed_boost"
  
  water_flow:
    technique_id: "water_flow"
    technique_name: "Water Flow"
    technique_name_vi: "Thân Pháp Thủy"
    element_id: "water"
    element_category: "elemental"
    base_mastery: 100.0
    mastery_scaling: 0.01
    efficiency_scaling: 0.005
    resource_cost: 40.0
    cooldown: 4.0
    range: 8.0
    speed_multiplier: 1.8
    distance_multiplier: 2.0
    duration_multiplier: 1.2
    special_effects:
      - "swimming_boost"
      - "flow_effect"

advanced_techniques:
  teleportation:
    technique_id: "teleportation"
    technique_name: "Teleportation"
    technique_name_vi: "Dịch Chuyển Tức Thời"
    technique_type: "Teleportation"
    base_mastery: 500.0
    mastery_scaling: 0.005
    efficiency_scaling: 0.002
    resource_cost: 200.0
    cooldown: 10.0
    range: 50.0
    speed_multiplier: 10.0
    distance_multiplier: 5.0
    duration_multiplier: 0.1
    special_effects:
      - "instant_movement"
      - "bypass_obstacles"
  
  shadow_step:
    technique_id: "shadow_step"
    technique_name: "Shadow Step"
    technique_name_vi: "Bước Bóng"
    technique_type: "ShadowStep"
    base_mastery: 300.0
    mastery_scaling: 0.008
    efficiency_scaling: 0.003
    resource_cost: 150.0
    cooldown: 8.0
    range: 15.0
    speed_multiplier: 3.0
    distance_multiplier: 2.5
    duration_multiplier: 0.5
    special_effects:
      - "stealth_effect"
      - "phase_through"

custom_techniques:
  crystal_movement:
    technique_id: "crystal_movement"
    technique_name: "Crystal Movement"
    technique_name_vi: "Thân Pháp Kết Tinh"
    technique_type: "Transformation"
    base_mastery: 200.0
    mastery_scaling: 0.007
    efficiency_scaling: 0.004
    resource_cost: 100.0
    cooldown: 6.0
    range: 12.0
    speed_multiplier: 1.5
    distance_multiplier: 1.8
    duration_multiplier: 2.0
    special_effects:
      - "crystal_formation"
      - "defense_boost"
    custom_properties:
      defense_multiplier: 20.0
      defense_bonus: 100000.0
      elemental_resistance: 0.8
      status_resistance: 0.5
```

### **3. Movement Derived Stats Configuration**

```yaml
# movement_derived_stats.yaml
movement_derived_stats:
  # Basic movement stats
  movement_speed: 1.0
  movement_acceleration: 1.0
  movement_deceleration: 1.0
  movement_turn_rate: 1.0
  
  # Advanced movement stats
  teleportation_ability: 0.0
  flight_ability: 0.0
  swimming_ability: 0.0
  climbing_ability: 0.0
  phase_ability: 0.0
  
  # Movement restrictions
  movement_restriction_resistance: 0.0
  
  # Status resistance (flexible)
  status_resistance:
    immobilized: 0.0
    rooted: 0.0
    slowed: 0.0
    stunned: 0.0
    paralyzed: 0.0
    frozen: 0.0
    entangled: 0.0
  
  # Status category resistance (flexible)
  status_category_resistance:
    movement_restriction: 0.0
    control: 0.0
    debuff: 0.0
    transformation: 0.0
  
  # Movement techniques
  movement_technique_mastery: 0.0
  movement_technique_efficiency: 0.0
  movement_technique_cooldown_reduction: 0.0
  
  # Terrain adaptation
  terrain_adaptation:
    ground: 1.0
    mountain: 0.8
    desert: 0.9
    forest: 1.1
    water: 0.7
    air: 0.5
  
  # Weather adaptation
  weather_adaptation:
    sunny: 1.0
    rainy: 0.9
    snowy: 0.8
    stormy: 0.7
    foggy: 0.6
  
  # Elemental movement (flexible)
  elemental_movement_bonus:
    fire: 0.0
    water: 0.0
    earth: 0.0
    wood: 0.0
    metal: 0.0
    air: 0.0
    lightning: 0.0
  
  elemental_movement_resistance:
    fire: 0.0
    water: 0.0
    earth: 0.0
    wood: 0.0
    metal: 0.0
    air: 0.0
    lightning: 0.0
  
  # Custom movement stats (extensible)
  custom_movement_stats:
    crystal_movement_bonus: 0.0
    shadow_movement_bonus: 0.0
    light_movement_bonus: 0.0
```

## 🧪 **Testing Strategy**

### **1. Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_movement_derived_stats_calculation() {
        let movement_stats = MovementDerivedStats::new();
        let base_speed = 1.0;
        let fire_mastery = 1000.0;
        
        let final_speed = calculate_fire_movement_bonus(fire_mastery, base_speed);
        assert!(final_speed > base_speed);
    }
    
    #[test]
    fn test_cultivation_movement_bonus() {
        let cultivation_level = 50.0;
        let cultivation_realm = 3; // Golden Core
        let base_speed = 1.0;
        
        let final_speed = calculate_cultivation_movement_bonus(
            cultivation_level,
            cultivation_realm,
            base_speed
        );
        
        assert!(final_speed > base_speed);
    }
    
    #[test]
    fn test_movement_restriction_effects() {
        let restrictions = vec![
            MovementRestriction {
                restriction_type: MovementRestrictionType::Immobilized,
                magnitude: 1.0,
                // ...
            }
        ];
        
        let base_resistance = 0.5;
        let final_resistance = calculate_movement_restriction_status_penalty(
            &restrictions,
            base_resistance
        );
        
        assert!(final_resistance < base_resistance);
    }
}
```

### **2. Integration Tests**

```rust
#[tokio::test]
async fn test_crystal_defense_movement_integration() {
    let crystal_defense = CrystalDefenseTechnique::new();
    let mut movement_system = MovementSystem::new();
    let mut actor = create_test_actor();
    
    // Apply crystallization
    crystal_defense.apply_movement_restrictions(&mut actor, &mut movement_system).await?;
    
    // Test movement restriction
    let can_move = movement_system.can_actor_move(
        actor.get_id(),
        MovementActionType::Walk,
        Position::default()
    ).await?;
    
    assert!(!can_move);
    
    // Test combat effects
    let (accuracy, dodge_rate) = calculate_crystal_defense_combat_effects(
        &actor,
        0.8, // base accuracy
        0.3  // base dodge rate
    );
    
    assert!(accuracy > 0.8); // Should be higher
    assert!(dodge_rate < 0.3); // Should be lower
}
```

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
