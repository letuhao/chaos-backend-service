# Elemental Mastery System Design

## Tổng Quan (Overview)

Elemental Mastery System là một hệ thống tu luyện độc lập trong Chaos World MMORPG, được thiết kế như một cultivation system kế thừa từ Actor Core. Hệ thống này cho phép nhân vật phát triển tinh thông (mastery) với các nguyên tố khác nhau thông qua việc tu luyện, training và sử dụng kỹ năng.

### Mục Tiêu Chính (Main Goals)

1. **Hệ Thống Tu Luyện Độc Lập**: Elemental Mastery hoạt động song song với các hệ thống tu luyện khác (hỏa pháp sư, luyện khí, ma pháp, etc.)
2. **Plugin-Based Architecture**: Mỗi element được implement như một plugin độc lập, dễ dàng thêm mới và tùy chỉnh
3. **Decay System**: Mastery sẽ giảm dần theo thời gian nếu không được sử dụng, tạo ra động lực tu luyện liên tục
4. **Element Interactions**: Hệ thống tương sinh tương khắc ảnh hưởng đến việc tu luyện và sử dụng elements
5. **Integration với Element Core**: Cung cấp derived stats cho Element Core system
6. **Integration với Damage Manager**: Cung cấp elemental damage data và modifiers cho Damage Manager

## Kiến Trúc Hệ Thống (System Architecture)

### 1. Plugin-Based Design

```rust
/// Element Plugin Trait
pub trait ElementPlugin: Send + Sync {
    /// Get element identifier
    fn get_element_id(&self) -> String;
    
    /// Get element definition
    fn get_element_definition(&self) -> ElementDefinition;
    
    /// Calculate base mastery
    fn calculate_base_mastery(&self, actor: &Actor) -> f64;
    
    /// Calculate decay rate
    fn calculate_decay_rate(&self, actor: &Actor) -> f64;
    
    /// Get opposite elements
    fn get_opposite_elements(&self) -> Vec<String>;
    
    /// Handle training
    fn handle_training(&self, actor: &mut Actor, training_amount: f64) -> ActorCoreResult<()>;
    
    /// Get derived stats for this element
    fn get_derived_stats(&self, actor: &Actor) -> HashMap<String, f64>;
    
    /// Get training methods for this element
    fn get_training_methods(&self) -> Vec<TrainingMethod>;
    
    /// Get element interactions
    fn get_element_interactions(&self) -> HashMap<String, ElementInteraction>;
    
    /// Get elemental damage modifiers for Damage Manager
    fn get_damage_modifiers(&self, actor: &Actor, damage_type: &str) -> Vec<DamageModifier>;
    
    /// Get elemental damage data for Damage Manager
    fn get_damage_data(&self, actor: &Actor, damage_type: &str) -> ElementDamageData;
    
    /// Check elemental damage immunity
    fn check_damage_immunity(&self, actor: &Actor, damage_type: &str) -> bool;
}
```

### 2. Element Plugin Registry

```rust
/// Element Plugin Registry
pub struct ElementPluginRegistry {
    /// Registered element plugins
    plugins: HashMap<String, Box<dyn ElementPlugin>>,
    /// Element definitions cache
    element_definitions: HashMap<String, ElementDefinition>,
    /// Element interactions matrix
    element_interactions: HashMap<(String, String), ElementInteraction>,
}
```

### 3. Integration với Actor Core

```rust
/// Plugin-Based Element Mastery System Resource Calculator
#[derive(Debug)]
pub struct PluginBasedElementMasterySystem {
    /// Base calculator (inherits from Actor Core)
    base: BaseSystemResourceCalculator,
    /// Element plugin registry
    plugin_registry: ElementPluginRegistry,
    /// Decay configuration
    decay_config: MasteryDecayConfig,
}
```

## Cơ Chế Tu Luyện (Cultivation Mechanics)

### 1. Elemental Mastery Progression System

#### A. Experience Points (Điểm Kinh Nghiệm)
- **Element Experience**: Điểm kinh nghiệm riêng cho từng element
- **Training Experience**: Kinh nghiệm từ việc tu luyện thực tế
- **Combat Experience**: Kinh nghiệm từ việc sử dụng element trong combat
- **Meditation Experience**: Kinh nghiệm từ việc thiền định với element

#### B. Mastery Levels (Cấp Độ Thông Thạo) - Extended System
```rust
pub enum ElementMasteryLevel {
    // Basic Levels (Cấp Cơ Bản)
    Beginner,           // Người mới bắt đầu (0-49)
    Novice,             // Sơ cấp (50-149)
    Apprentice,         // Học việc (150-299)
    Regular,            // Thường xuyên (300-499)
    
    // Intermediate Levels (Cấp Trung Cấp)
    Adept,              // Thành thạo (500-799)
    Expert,             // Chuyên gia (800-1199)
    AdvancedExpert,     // Chuyên gia nâng cao (1200-1799)
    Master,             // Bậc thầy (1800-2499)
    
    // Advanced Levels (Cấp Cao Cấp)
    AdvancedMaster,     // Bậc thầy nâng cao (2500-3499)
    GrandMaster,        // Đại sư (3500-4999)
    Completer,          // Hoàn thiện (5000-6999)
    Transcender,        // Siêu việt (7000-9999)
    
    // Legendary Levels (Cấp Huyền Thoại)
    Sage,               // Hiền triết (10000-14999)
    Archmage,           // Pháp sư tối cao (15000-21999)
    Legendary,          // Huyền thoại (22000-31999)
    Mythic,             // Thần thoại (32000-44999)
    
    // Transcendent Levels (Cấp Siêu Việt)
    Transcendent,       // Siêu việt (45000-64999)
    Celestial,          // Thiên thần (65000-89999)
    Divine,             // Thần thánh (90000-129999)
    Immortal,           // Bất tử (130000-179999)
    
    // Ultimate Levels (Cấp Tối Thượng)
    Eternal,            // Vĩnh cửu (180000-249999)
    Omniscient,         // Toàn tri (250000-349999)
    Omnipotent,         // Toàn năng (350000-499999)
    Supreme,            // Tối thượng (500000+)
}

impl ElementMasteryLevel {
    pub fn from_experience(exp: i64) -> Self {
        match exp {
            0..=999 => ElementMasteryLevel::Beginner,                    // 0-999
            1000..=4999 => ElementMasteryLevel::Novice,                  // 1K-4.9K
            5000..=14999 => ElementMasteryLevel::Apprentice,             // 5K-14.9K
            15000..=49999 => ElementMasteryLevel::Regular,               // 15K-49.9K
            50000..=149999 => ElementMasteryLevel::Adept,                // 50K-149.9K
            150000..=499999 => ElementMasteryLevel::Expert,              // 150K-499.9K
            500000..=1499999 => ElementMasteryLevel::AdvancedExpert,     // 500K-1.49M
            1500000..=4999999 => ElementMasteryLevel::Master,            // 1.5M-4.99M
            5000000..=14999999 => ElementMasteryLevel::AdvancedMaster,   // 5M-14.9M
            15000000..=49999999 => ElementMasteryLevel::GrandMaster,     // 15M-49.9M
            50000000..=149999999 => ElementMasteryLevel::Completer,      // 50M-149.9M
            150000000..=499999999 => ElementMasteryLevel::Transcender,   // 150M-499.9M
            500000000..=1499999999 => ElementMasteryLevel::Sage,         // 500M-1.49B
            1500000000..=4999999999 => ElementMasteryLevel::Archmage,    // 1.5B-4.99B
            5000000000..=14999999999 => ElementMasteryLevel::Legendary,  // 5B-14.9B
            15000000000..=49999999999 => ElementMasteryLevel::Mythic,    // 15B-49.9B
            50000000000..=149999999999 => ElementMasteryLevel::Transcendent, // 50B-149.9B
            150000000000..=499999999999 => ElementMasteryLevel::Celestial,   // 150B-499.9B
            500000000000..=1499999999999 => ElementMasteryLevel::Divine,     // 500B-1.49T
            1500000000000..=4999999999999 => ElementMasteryLevel::Immortal,  // 1.5T-4.99T
            5000000000000..=14999999999999 => ElementMasteryLevel::Eternal,  // 5T-14.9T
            15000000000000..=49999999999999 => ElementMasteryLevel::Omniscient, // 15T-49.9T
            50000000000000..=149999999999999 => ElementMasteryLevel::Omnipotent, // 50T-149.9T
            _ => ElementMasteryLevel::Supreme,                          // 150T+
        }
    }
    
    pub fn get_level_bonus(&self) -> f64 {
        match self {
            // Basic Levels
            ElementMasteryLevel::Beginner => 1.0,
            ElementMasteryLevel::Novice => 1.1,
            ElementMasteryLevel::Apprentice => 1.25,
            ElementMasteryLevel::Regular => 1.4,
            
            // Intermediate Levels
            ElementMasteryLevel::Adept => 1.6,
            ElementMasteryLevel::Expert => 1.8,
            ElementMasteryLevel::AdvancedExpert => 2.0,
            ElementMasteryLevel::Master => 2.3,
            
            // Advanced Levels
            ElementMasteryLevel::AdvancedMaster => 2.7,
            ElementMasteryLevel::GrandMaster => 3.2,
            ElementMasteryLevel::Completer => 3.8,
            ElementMasteryLevel::Transcender => 4.5,
            
            // Legendary Levels
            ElementMasteryLevel::Sage => 5.5,
            ElementMasteryLevel::Archmage => 6.8,
            ElementMasteryLevel::Legendary => 8.5,
            ElementMasteryLevel::Mythic => 10.5,
            
            // Transcendent Levels
            ElementMasteryLevel::Transcendent => 13.0,
            ElementMasteryLevel::Celestial => 16.0,
            ElementMasteryLevel::Divine => 20.0,
            ElementMasteryLevel::Immortal => 25.0,
            
            // Ultimate Levels
            ElementMasteryLevel::Eternal => 32.0,
            ElementMasteryLevel::Omniscient => 40.0,
            ElementMasteryLevel::Omnipotent => 50.0,
            ElementMasteryLevel::Supreme => 65.0,
        }
    }
    
    pub fn get_level_name_vi(&self) -> &'static str {
        match self {
            // Basic Levels
            ElementMasteryLevel::Beginner => "Người Mới Bắt Đầu",
            ElementMasteryLevel::Novice => "Sơ Cấp",
            ElementMasteryLevel::Apprentice => "Học Việc",
            ElementMasteryLevel::Regular => "Thường Xuyên",
            
            // Intermediate Levels
            ElementMasteryLevel::Adept => "Thành Thạo",
            ElementMasteryLevel::Expert => "Chuyên Gia",
            ElementMasteryLevel::AdvancedExpert => "Chuyên Gia Nâng Cao",
            ElementMasteryLevel::Master => "Bậc Thầy",
            
            // Advanced Levels
            ElementMasteryLevel::AdvancedMaster => "Bậc Thầy Nâng Cao",
            ElementMasteryLevel::GrandMaster => "Đại Sư",
            ElementMasteryLevel::Completer => "Hoàn Thiện",
            ElementMasteryLevel::Transcender => "Siêu Việt",
            
            // Legendary Levels
            ElementMasteryLevel::Sage => "Hiền Triết",
            ElementMasteryLevel::Archmage => "Pháp Sư Tối Cao",
            ElementMasteryLevel::Legendary => "Huyền Thoại",
            ElementMasteryLevel::Mythic => "Thần Thoại",
            
            // Transcendent Levels
            ElementMasteryLevel::Transcendent => "Siêu Việt",
            ElementMasteryLevel::Celestial => "Thiên Thần",
            ElementMasteryLevel::Divine => "Thần Thánh",
            ElementMasteryLevel::Immortal => "Bất Tử",
            
            // Ultimate Levels
            ElementMasteryLevel::Eternal => "Vĩnh Cửu",
            ElementMasteryLevel::Omniscient => "Toàn Tri",
            ElementMasteryLevel::Omnipotent => "Toàn Năng",
            ElementMasteryLevel::Supreme => "Tối Thượng",
        }
    }
    
    pub fn get_level_tier(&self) -> MasteryLevelTier {
        match self {
            // Basic Levels
            ElementMasteryLevel::Beginner | 
            ElementMasteryLevel::Novice | 
            ElementMasteryLevel::Apprentice | 
            ElementMasteryLevel::Regular => MasteryLevelTier::Basic,
            
            // Intermediate Levels
            ElementMasteryLevel::Adept | 
            ElementMasteryLevel::Expert | 
            ElementMasteryLevel::AdvancedExpert | 
            ElementMasteryLevel::Master => MasteryLevelTier::Intermediate,
            
            // Advanced Levels
            ElementMasteryLevel::AdvancedMaster | 
            ElementMasteryLevel::GrandMaster | 
            ElementMasteryLevel::Completer | 
            ElementMasteryLevel::Transcender => MasteryLevelTier::Advanced,
            
            // Legendary Levels
            ElementMasteryLevel::Sage | 
            ElementMasteryLevel::Archmage | 
            ElementMasteryLevel::Legendary | 
            ElementMasteryLevel::Mythic => MasteryLevelTier::Legendary,
            
            // Transcendent Levels
            ElementMasteryLevel::Transcendent | 
            ElementMasteryLevel::Celestial | 
            ElementMasteryLevel::Divine | 
            ElementMasteryLevel::Immortal => MasteryLevelTier::Transcendent,
            
            // Ultimate Levels
            ElementMasteryLevel::Eternal | 
            ElementMasteryLevel::Omniscient | 
            ElementMasteryLevel::Omnipotent | 
            ElementMasteryLevel::Supreme => MasteryLevelTier::Ultimate,
        }
    }
}

/// Mastery Level Tiers for categorization
pub enum MasteryLevelTier {
    Basic,          // Cấp Cơ Bản
    Intermediate,   // Cấp Trung Cấp
    Advanced,       // Cấp Cao Cấp
    Legendary,      // Cấp Huyền Thoại
    Transcendent,   // Cấp Siêu Việt
    Ultimate,       // Cấp Tối Thượng
}

impl MasteryLevelTier {
    pub fn get_tier_name_vi(&self) -> &'static str {
        match self {
            MasteryLevelTier::Basic => "Cấp Cơ Bản",
            MasteryLevelTier::Intermediate => "Cấp Trung Cấp",
            MasteryLevelTier::Advanced => "Cấp Cao Cấp",
            MasteryLevelTier::Legendary => "Cấp Huyền Thoại",
            MasteryLevelTier::Transcendent => "Cấp Siêu Việt",
            MasteryLevelTier::Ultimate => "Cấp Tối Thượng",
        }
    }
    
    pub fn get_tier_multiplier(&self) -> f64 {
        match self {
            MasteryLevelTier::Basic => 1.0,
            MasteryLevelTier::Intermediate => 1.5,
            MasteryLevelTier::Advanced => 2.5,
            MasteryLevelTier::Legendary => 4.0,
            MasteryLevelTier::Transcendent => 6.5,
            MasteryLevelTier::Ultimate => 10.0,
        }
    }
}
```

#### C. Power Scale First Calculation System
```rust
/// Calculate Element Mastery (Power Scale) as primary derived stat
fn calculate_element_mastery(&self, actor: &Actor, element_id: &str) -> f64 {
    let experience_data = self.get_element_experience_data(actor, element_id);
    
    // Calculate base power scale from experience data
    let base_power_scale = self.calculate_base_power_scale(&experience_data);
    
    // Apply level bonuses
    let level_bonus = self.calculate_level_bonus(&experience_data);
    
    // Apply tier bonuses
    let tier_bonus = self.calculate_tier_bonus(&experience_data);
    
    // Apply realm bonuses
    let realm_bonus = self.calculate_realm_bonus(&experience_data);
    
    // Apply element-specific modifiers
    let element_modifier = self.get_element_modifier(element_id);
    
    // Calculate final power scale (element_mastery derived stat)
    let final_power_scale = base_power_scale * level_bonus * tier_bonus * realm_bonus * element_modifier;
    
    // Cap power scale at reasonable maximum
    final_power_scale.min(1000000.0) // Cap at 1M power scale
}

/// Calculate base power scale from experience data
fn calculate_base_power_scale(&self, experience_data: &ElementExperienceData) -> f64 {
    let total_experience = experience_data.calculate_total_experience();
    let experience_f64 = total_experience as f64;
    
    // Logarithmic scaling for power scale
    // Power scale grows logarithmically with experience
    let base_power_scale = (experience_f64 / 1000000.0).log10() * 1000.0;
    
    // Ensure minimum power scale
    base_power_scale.max(1.0)
}

/// Calculate level bonus based on power scale
fn calculate_level_bonus(&self, experience_data: &ElementExperienceData) -> f64 {
    let total_experience = experience_data.calculate_total_experience();
    let level = ElementMasteryLevel::from_experience(total_experience);
    level.get_level_bonus()
}

/// Calculate tier bonus based on power scale
fn calculate_tier_bonus(&self, experience_data: &ElementExperienceData) -> f64 {
    let total_experience = experience_data.calculate_total_experience();
    let level = ElementMasteryLevel::from_experience(total_experience);
    let tier = level.get_level_tier();
    tier.get_tier_multiplier()
}

/// Calculate realm bonus based on breakthroughs
fn calculate_realm_bonus(&self, experience_data: &ElementExperienceData) -> f64 {
    let breakthroughs = experience_data.total_breakthroughs;
    
    // Exponential realm bonus (similar to Kim Đan system)
    match breakthroughs {
        0 => 1.0,
        1 => 2.0,      // 2x power scale
        2 => 4.0,      // 4x power scale
        3 => 8.0,      // 8x power scale
        4 => 16.0,     // 16x power scale
        5 => 32.0,     // 32x power scale
        6 => 64.0,     // 64x power scale
        7 => 128.0,    // 128x power scale
        8 => 256.0,    // 256x power scale
        9 => 512.0,    // 512x power scale
        10 => 1024.0,  // 1024x power scale
        11 => 2048.0,  // 2048x power scale
        _ => 4096.0,   // 4096x power scale (cap)
    }
}

/// Calculate required experience from target power scale (reverse formula)
fn calculate_required_experience(&self, target_power_scale: f64, element_id: &str) -> i64 {
    // Reverse the power scale calculation
    // target_power_scale = base_power_scale * level_bonus * tier_bonus * realm_bonus * element_modifier
    
    // Get current experience data to determine current bonuses
    let current_experience_data = self.get_element_experience_data(actor, element_id);
    let current_level_bonus = self.calculate_level_bonus(&current_experience_data);
    let current_tier_bonus = self.calculate_tier_bonus(&current_experience_data);
    let current_realm_bonus = self.calculate_realm_bonus(&current_experience_data);
    let element_modifier = self.get_element_modifier(element_id);
    
    // Calculate required base power scale
    let total_bonus = current_level_bonus * current_tier_bonus * current_realm_bonus * element_modifier;
    let required_base_power_scale = target_power_scale / total_bonus;
    
    // Reverse logarithmic scaling to get required experience
    // base_power_scale = (experience / 1000000.0).log10() * 1000.0
    // experience = 1000000.0 * 10^(base_power_scale / 1000.0)
    let required_experience = 1000000.0 * 10_f64.powf(required_base_power_scale / 1000.0);
    
    required_experience as i64
}

/// Calculate experience needed for next level
fn calculate_experience_to_next_level(&self, actor: &Actor, element_id: &str) -> i64 {
    let current_experience_data = self.get_element_experience_data(actor, element_id);
    let current_experience = current_experience_data.calculate_total_experience();
    let current_level = ElementMasteryLevel::from_experience(current_experience);
    
    // Get next level experience threshold
    let next_level_threshold = self.get_next_level_experience_threshold(&current_level);
    
    // Calculate required experience for next level
    let current_power_scale = self.calculate_element_mastery(actor, element_id);
    let next_level_power_scale = current_power_scale * 1.1; // 10% increase for next level
    
    let required_experience = self.calculate_required_experience(next_level_power_scale, element_id);
    
    required_experience - current_experience
}

fn get_element_experience_data(&self, actor: &Actor, element_id: &str) -> ElementExperienceData {
    let experience_points = actor.get_data().get(&format!("{}_experience", element_id))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    
    let training_time = actor.get_data().get(&format!("{}_training_time", element_id))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    
    let combat_usage = actor.get_data().get(&format!("{}_combat_usage", element_id))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    
    let meditation_time = actor.get_data().get(&format!("{}_meditation_time", element_id))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    
    let total_breakthroughs = actor.get_data().get(&format!("{}_breakthroughs", element_id))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    
    let realm_multiplier = self.calculate_realm_multiplier(total_breakthroughs);
    
    ElementExperienceData {
        element_id: element_id.to_string(),
        experience_points,
        training_time,
        combat_usage,
        meditation_time,
        last_used: SystemTime::now(),
        total_breakthroughs,
        realm_multiplier,
    }
}

fn calculate_realm_multiplier(&self, breakthroughs: u32) -> i64 {
    // Exponential scaling based on breakthroughs (similar to Kim Đan system)
    match breakthroughs {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1000,
        4 => 10000,
        5 => 100000,
        6 => 1000000,
        7 => 10000000,
        8 => 100000000,
        9 => 1000000000,
        10 => 10000000000,
        11 => 100000000000,
        _ => 1000000000000, // Cap at 1T multiplier
    }
}
```

### 2. Elemental Mastery Realms (Cảnh Giới Thông Thạo Nguyên Tố)

#### A. Realm System
```rust
pub enum ElementMasteryRealm {
    // Basic Realms
    ElementalAwareness,    // Nhận Thức Nguyên Tố (0-999)
    ElementalControl,      // Khống Chế Nguyên Tố (1000-2999)
    ElementalHarmony,      // Hòa Hợp Nguyên Tố (3000-5999)
    ElementalTranscendence, // Siêu Việt Nguyên Tố (6000-9999)
    ElementalAscension,    // Thăng Hoa Nguyên Tố (10000+)
}

impl ElementMasteryRealm {
    pub fn from_mastery(mastery: f64) -> Self {
        match mastery as u32 {
            0..=999 => ElementMasteryRealm::ElementalAwareness,
            1000..=2999 => ElementMasteryRealm::ElementalControl,
            3000..=5999 => ElementMasteryRealm::ElementalHarmony,
            6000..=9999 => ElementMasteryRealm::ElementalTranscendence,
            _ => ElementMasteryRealm::ElementalAscension,
        }
    }
    
    pub fn get_realm_multiplier(&self) -> f64 {
        match self {
            ElementMasteryRealm::ElementalAwareness => 1.0,
            ElementMasteryRealm::ElementalControl => 2.0,
            ElementMasteryRealm::ElementalHarmony => 5.0,
            ElementMasteryRealm::ElementalTranscendence => 10.0,
            ElementMasteryRealm::ElementalAscension => 25.0,
        }
    }
    
    pub fn get_realm_name_vi(&self) -> &'static str {
        match self {
            ElementMasteryRealm::ElementalAwareness => "Nhận Thức Nguyên Tố",
            ElementMasteryRealm::ElementalControl => "Khống Chế Nguyên Tố",
            ElementMasteryRealm::ElementalHarmony => "Hòa Hợp Nguyên Tố",
            ElementMasteryRealm::ElementalTranscendence => "Siêu Việt Nguyên Tố",
            ElementMasteryRealm::ElementalAscension => "Thăng Hoa Nguyên Tố",
        }
    }
}
```

#### B. Realm Stages (Tiểu Cấp)
```rust
pub enum ElementMasteryStage {
    Early,   // Sơ kỳ
    Mid,     // Trung kỳ
    Late,    // Hậu kỳ
    Peak,    // Đỉnh phong
}

impl ElementMasteryStage {
    pub fn from_mastery(mastery: f64, realm: &ElementMasteryRealm) -> Self {
        let realm_range = match realm {
            ElementMasteryRealm::ElementalAwareness => (0.0, 999.0),
            ElementMasteryRealm::ElementalControl => (1000.0, 2999.0),
            ElementMasteryRealm::ElementalHarmony => (3000.0, 5999.0),
            ElementMasteryRealm::ElementalTranscendence => (6000.0, 9999.0),
            ElementMasteryRealm::ElementalAscension => (10000.0, f64::MAX),
        };
        
        let range_size = realm_range.1 - realm_range.0;
        let position = (mastery - realm_range.0) / range_size;
        
        match position {
            x if x < 0.25 => ElementMasteryStage::Early,
            x if x < 0.5 => ElementMasteryStage::Mid,
            x if x < 0.75 => ElementMasteryStage::Late,
            _ => ElementMasteryStage::Peak,
        }
    }
    
    pub fn get_stage_multiplier(&self) -> f64 {
        match self {
            ElementMasteryStage::Early => 1.0,
            ElementMasteryStage::Mid => 1.25,
            ElementMasteryStage::Late => 1.5,
            ElementMasteryStage::Peak => 1.75,
        }
    }
}
```

### 3. Level Progression System (Hệ Thống Tiến Bộ Cấp Độ)

#### A. Experience Requirements per Level
```rust
pub struct LevelProgressionConfig {
    pub base_experience_required: f64,
    pub experience_scaling_factor: f64,
    pub tier_multiplier: f64,
    pub max_levels_per_tier: u32,
}

impl LevelProgressionConfig {
    pub fn calculate_experience_required(&self, level: u32, tier: &MasteryLevelTier) -> f64 {
        let tier_multiplier = tier.get_tier_multiplier();
        let level_within_tier = level % self.max_levels_per_tier;
        
        // Base experience for level within tier
        let base_exp = self.base_experience_required * 
                      (self.experience_scaling_factor.powi(level_within_tier as i32));
        
        // Apply tier multiplier
        base_exp * tier_multiplier
    }
    
    pub fn get_level_progression_info(&self, current_level: u32) -> LevelProgressionInfo {
        let tier = self.get_tier_for_level(current_level);
        let level_within_tier = current_level % self.max_levels_per_tier;
        let exp_required = self.calculate_experience_required(current_level, &tier);
        let next_level_exp = self.calculate_experience_required(current_level + 1, &tier);
        
        LevelProgressionInfo {
            current_level,
            tier,
            level_within_tier,
            experience_required: exp_required,
            next_level_experience: next_level_exp,
            experience_needed: next_level_exp - exp_required,
        }
    }
}

pub struct LevelProgressionInfo {
    pub current_level: u32,
    pub tier: MasteryLevelTier,
    pub level_within_tier: u32,
    pub experience_required: f64,
    pub next_level_experience: f64,
    pub experience_needed: f64,
}
```

#### B. Level Unlock Requirements
```rust
pub struct LevelUnlockRequirements {
    pub min_cultivation_level: u32,        // Minimum cultivation level required
    pub min_element_mastery: f64,          // Minimum mastery in other elements
    pub required_achievements: Vec<String>, // Required achievements
    pub required_items: Vec<String>,       // Required items or resources
    pub special_conditions: Vec<String>,   // Special unlock conditions
}

impl LevelUnlockRequirements {
    pub fn can_unlock_level(&self, actor: &Actor, target_level: u32) -> bool {
        // Check cultivation level
        let cultivation_level = actor.get_data().get("cultivation_level")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        
        if cultivation_level < self.min_cultivation_level {
            return false;
        }
        
        // Check element mastery requirements
        for (element_id, min_mastery) in &self.element_mastery_requirements {
            let mastery = self.get_element_mastery(actor, element_id);
            if mastery < *min_mastery {
                return false;
            }
        }
        
        // Check achievements
        for achievement in &self.required_achievements {
            if !self.has_achievement(actor, achievement) {
                return false;
            }
        }
        
        // Check items
        for item in &self.required_items {
            if !self.has_item(actor, item) {
                return false;
            }
        }
        
        // Check special conditions
        for condition in &self.special_conditions {
            if !self.evaluate_condition(actor, condition) {
                return false;
            }
        }
        
        true
    }
}
```

#### C. Level Progression Bonuses
```rust
pub struct LevelProgressionBonuses {
    pub stat_bonuses: HashMap<String, f64>,
    pub skill_unlocks: Vec<String>,
    pub passive_abilities: Vec<String>,
    pub special_effects: Vec<String>,
}

impl LevelProgressionBonuses {
    pub fn get_level_bonuses(&self, level: u32, tier: &MasteryLevelTier) -> LevelProgressionBonuses {
        let mut bonuses = LevelProgressionBonuses::new();
        
        // Apply tier-based bonuses
        let tier_multiplier = tier.get_tier_multiplier();
        
        // Stat bonuses scale with level and tier
        for (stat, base_bonus) in &self.stat_bonuses {
            let scaled_bonus = base_bonus * (level as f64 / 100.0) * tier_multiplier;
            bonuses.stat_bonuses.insert(stat.clone(), scaled_bonus);
        }
        
        // Skill unlocks based on level thresholds
        for skill in &self.skill_unlocks {
            if self.should_unlock_skill(level, skill) {
                bonuses.skill_unlocks.push(skill.clone());
            }
        }
        
        // Passive abilities unlock at specific levels
        for ability in &self.passive_abilities {
            if self.should_unlock_ability(level, ability) {
                bonuses.passive_abilities.push(ability.clone());
            }
        }
        
        // Special effects for milestone levels
        for effect in &self.special_effects {
            if self.should_apply_effect(level, effect) {
                bonuses.special_effects.push(effect.clone());
            }
        }
        
        bonuses
    }
}
```

### 4. Power Scale Display System (Hệ Thống Hiển Thị Power Scale)

#### A. Power Scale Information Structure
```rust
pub struct ElementPowerScaleInfo {
    pub element_id: String,
    pub power_scale: f64,                    // element_mastery derived stat
    pub experience: i64,                     // Total experience points
    pub experience_tier: ExperienceTier,     // Current experience tier
    pub mastery_level: ElementMasteryLevel,  // Current mastery level
    pub mastery_tier: MasteryLevelTier,      // Current mastery tier
    pub realm: ElementMasteryRealm,          // Current realm
    pub stage: ElementMasteryStage,          // Current stage
    pub power_tier: ElementalPowerTier,      // Current power tier
    pub experience_to_next_level: i64,       // Experience needed for next level
    pub power_scale_to_next_level: f64,     // Power scale needed for next level
    pub realm_multiplier: f64,              // Current realm multiplier
    pub level_bonus: f64,                   // Current level bonus
    pub tier_bonus: f64,                    // Current tier bonus
}

impl ElementPowerScaleInfo {
    pub fn new(actor: &Actor, element_id: &str, mastery_system: &ElementalMasterySystem) -> Self {
        let experience_data = mastery_system.get_element_experience_data(actor, element_id);
        let total_experience = experience_data.calculate_total_experience();
        let power_scale = mastery_system.calculate_element_mastery(actor, element_id);
        
        let mastery_level = ElementMasteryLevel::from_experience(total_experience);
        let mastery_tier = mastery_level.get_level_tier();
        let experience_tier = experience_data.get_experience_tier();
        
        let realm = ElementMasteryRealm::from_mastery(power_scale);
        let stage = ElementMasteryStage::from_mastery(power_scale, &realm);
        let power_tier = ElementalPowerTier::from_power(power_scale);
        
        let experience_to_next_level = mastery_system.calculate_experience_to_next_level(actor, element_id);
        let power_scale_to_next_level = power_scale * 1.1; // 10% increase for next level
        
        let realm_multiplier = mastery_system.calculate_realm_bonus(&experience_data);
        let level_bonus = mastery_system.calculate_level_bonus(&experience_data);
        let tier_bonus = mastery_system.calculate_tier_bonus(&experience_data);
        
        Self {
            element_id: element_id.to_string(),
            power_scale,
            experience: total_experience,
            experience_tier,
            mastery_level,
            mastery_tier,
            realm,
            stage,
            power_tier,
            experience_to_next_level,
            power_scale_to_next_level,
            realm_multiplier,
            level_bonus,
            tier_bonus,
        }
    }
    
    /// Get formatted display string for UI
    pub fn get_display_string(&self) -> String {
        format!(
            "Element: {}\n\
             Power Scale: {:.2}\n\
             Experience: {}\n\
             Level: {} ({})\n\
             Tier: {} ({})\n\
             Realm: {} - {}\n\
             Power Tier: {}\n\
             Experience to Next Level: {}\n\
             Power Scale to Next Level: {:.2}\n\
             Realm Multiplier: {:.2}x\n\
             Level Bonus: {:.2}x\n\
             Tier Bonus: {:.2}x",
            self.element_id,
            self.power_scale,
            self.format_experience(self.experience),
            self.mastery_level.get_level_name_vi(),
            self.mastery_level as u32,
            self.mastery_tier.get_tier_name_vi(),
            self.mastery_tier as u32,
            self.realm.get_realm_name_vi(),
            self.stage as u32,
            self.power_tier.get_tier_name_vi(),
            self.format_experience(self.experience_to_next_level),
            self.power_scale_to_next_level,
            self.realm_multiplier,
            self.level_bonus,
            self.tier_bonus
        )
    }
    
    /// Format experience with appropriate units (K, M, B, T)
    fn format_experience(&self, exp: i64) -> String {
        match exp {
            0..=999 => format!("{}", exp),
            1000..=999999 => format!("{:.1}K", exp as f64 / 1000.0),
            1000000..=999999999 => format!("{:.1}M", exp as f64 / 1000000.0),
            1000000000..=999999999999 => format!("{:.1}B", exp as f64 / 1000000000.0),
            _ => format!("{:.1}T", exp as f64 / 1000000000000.0),
        }
    }
}
```

#### B. Power Scale Calculation with Reverse Formula
```rust
/// Calculate Elemental Power Scale (Primary Derived Stat)
fn calculate_elemental_power_scale(&self, actor: &Actor, element_id: &str) -> f64 {
    // This is the primary derived stat - element_mastery
    self.calculate_element_mastery(actor, element_id)
}

/// Calculate required experience for target power scale
fn calculate_experience_for_power_scale(&self, target_power_scale: f64, actor: &Actor, element_id: &str) -> i64 {
    let current_experience_data = self.get_element_experience_data(actor, element_id);
    
    // Get current bonuses
    let current_level_bonus = self.calculate_level_bonus(&current_experience_data);
    let current_tier_bonus = self.calculate_tier_bonus(&current_experience_data);
    let current_realm_bonus = self.calculate_realm_bonus(&current_experience_data);
    let element_modifier = self.get_element_modifier(element_id);
    
    // Calculate total bonus multiplier
    let total_bonus = current_level_bonus * current_tier_bonus * current_realm_bonus * element_modifier;
    
    // Calculate required base power scale
    let required_base_power_scale = target_power_scale / total_bonus;
    
    // Reverse logarithmic formula to get required experience
    // base_power_scale = (experience / 1000000.0).log10() * 1000.0
    // experience = 1000000.0 * 10^(base_power_scale / 1000.0)
    let required_experience = 1000000.0 * 10_f64.powf(required_base_power_scale / 1000.0);
    
    required_experience as i64
}

/// Calculate power scale progression info
fn get_power_scale_progression_info(&self, actor: &Actor, element_id: &str) -> PowerScaleProgressionInfo {
    let current_power_scale = self.calculate_elemental_power_scale(actor, element_id);
    let current_experience_data = self.get_element_experience_data(actor, element_id);
    let current_experience = current_experience_data.calculate_total_experience();
    
    // Calculate next level power scale
    let next_level_power_scale = current_power_scale * 1.1; // 10% increase
    let experience_for_next_level = self.calculate_experience_for_power_scale(next_level_power_scale, actor, element_id);
    
    PowerScaleProgressionInfo {
        current_power_scale,
        current_experience,
        next_level_power_scale,
        experience_for_next_level,
        experience_needed: experience_for_next_level - current_experience,
        power_scale_progress: (current_power_scale / next_level_power_scale) * 100.0,
    }
}

pub struct PowerScaleProgressionInfo {
    pub current_power_scale: f64,
    pub current_experience: i64,
    pub next_level_power_scale: f64,
    pub experience_for_next_level: i64,
    pub experience_needed: i64,
    pub power_scale_progress: f64, // Percentage progress to next level
}
```

#### B. Power Scale Tiers
```rust
pub enum ElementalPowerTier {
    Mortal,        // Phàm nhân (0-999)
    Transcendent,  // Siêu phàm (1000-9999)
    Divine,        // Thần thánh (10000-99999)
    Immortal,      // Bất tử (100000-999999)
    Celestial,     // Thiên thần (1000000+)
}

impl ElementalPowerTier {
    pub fn from_power(power: f64) -> Self {
        match power as u32 {
            0..=999 => ElementalPowerTier::Mortal,
            1000..=9999 => ElementalPowerTier::Transcendent,
            10000..=99999 => ElementalPowerTier::Divine,
            100000..=999999 => ElementalPowerTier::Immortal,
            _ => ElementalPowerTier::Celestial,
        }
    }
    
    pub fn get_tier_name_vi(&self) -> &'static str {
        match self {
            ElementalPowerTier::Mortal => "Phàm Nhân",
            ElementalPowerTier::Transcendent => "Siêu Phàm",
            ElementalPowerTier::Divine => "Thần Thánh",
            ElementalPowerTier::Immortal => "Bất Tử",
            ElementalPowerTier::Celestial => "Thiên Thần",
        }
    }
}
```

### 4. Realm Scaling Model (Mô Hình Thang Đo Cảnh Giới)

#### A. Exponential Realm Gap System
```rust
pub struct ElementalRealmScaling {
    pub realm_base_a: f64,           // Base factor A
    pub realm_base_b: f64,           // Exponential factor B
    pub stage_multipliers: HashMap<ElementMasteryStage, f64>,
    pub realm_ranks: HashMap<ElementMasteryRealm, u32>,
    pub baselines: HashMap<ElementMasteryRealm, f64>,
    pub min_max_deltas: HashMap<ElementMasteryRealm, (f64, f64)>,
}

impl ElementalRealmScaling {
    pub fn new() -> Self {
        Self {
            realm_base_a: 1.0,
            realm_base_b: 8.0,  // Exponential gap between realms
            stage_multipliers: HashMap::from([
                (ElementMasteryStage::Early, 1.0),
                (ElementMasteryStage::Mid, 1.25),
                (ElementMasteryStage::Late, 1.5),
                (ElementMasteryStage::Peak, 1.75),
            ]),
            realm_ranks: HashMap::from([
                (ElementMasteryRealm::ElementalAwareness, 0),
                (ElementMasteryRealm::ElementalControl, 1),
                (ElementMasteryRealm::ElementalHarmony, 2),
                (ElementMasteryRealm::ElementalTranscendence, 3),
                (ElementMasteryRealm::ElementalAscension, 4),
            ]),
            baselines: HashMap::from([
                (ElementMasteryRealm::ElementalAwareness, 100.0),
                (ElementMasteryRealm::ElementalControl, 1000.0),
                (ElementMasteryRealm::ElementalHarmony, 5000.0),
                (ElementMasteryRealm::ElementalTranscendence, 25000.0),
                (ElementMasteryRealm::ElementalAscension, 100000.0),
            ]),
            min_max_deltas: HashMap::from([
                (ElementMasteryRealm::ElementalAwareness, (5.0, 15.0)),
                (ElementMasteryRealm::ElementalControl, (10.0, 30.0)),
                (ElementMasteryRealm::ElementalHarmony, (25.0, 75.0)),
                (ElementMasteryRealm::ElementalTranscendence, (100.0, 300.0)),
                (ElementMasteryRealm::ElementalAscension, (500.0, 1500.0)),
            ]),
        }
    }
    
    /// Calculate realm gap multiplier
    pub fn calculate_realm_gap(&self, realm: &ElementMasteryRealm) -> f64 {
        let rank = self.realm_ranks.get(realm).copied().unwrap_or(0);
        self.realm_base_a * self.realm_base_b.powi(rank as i32)
    }
    
    /// Calculate breakthrough stats
    pub fn calculate_breakthrough_stats(
        &self,
        prev_mastery: f64,
        new_realm: &ElementMasteryRealm,
        new_stage: &ElementMasteryStage,
        seed: &str
    ) -> f64 {
        let realm_gap = self.calculate_realm_gap(new_realm);
        let stage_multiplier = self.stage_multipliers.get(new_stage).copied().unwrap_or(1.0);
        let baseline = self.baselines.get(new_realm).copied().unwrap_or(0.0);
        
        // Calculate base stats after breakthrough
        let base_stats = (prev_mastery.max(baseline) * realm_gap * stage_multiplier);
        
        // Add random delta
        let delta_range = self.min_max_deltas.get(new_realm).copied().unwrap_or((0.0, 0.0));
        let delta = self.calculate_deterministic_delta(seed, delta_range.0, delta_range.1) * stage_multiplier;
        
        base_stats + delta
    }
    
    /// Calculate deterministic delta using seed
    fn calculate_deterministic_delta(&self, seed: &str, min: f64, max: f64) -> f64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let hash = hasher.finish();
        
        let normalized = (hash % 10000) as f64 / 10000.0;
        min + (normalized * (max - min))
    }
}
```

#### B. Realm Breakthrough System
```rust
pub struct ElementalRealmBreakthrough {
    pub element_id: String,
    pub current_realm: ElementMasteryRealm,
    pub current_stage: ElementMasteryStage,
    pub current_mastery: f64,
    pub breakthrough_requirements: HashMap<ElementMasteryRealm, BreakthroughRequirement>,
}

pub struct BreakthroughRequirement {
    pub min_mastery: f64,
    pub required_elements: Vec<String>,  // Required opposite elements for balance
    pub breakthrough_cost: HashMap<String, f64>, // Resource costs
    pub breakthrough_time: Duration,
    pub success_rate: f64,
}

impl ElementalRealmBreakthrough {
    pub fn can_breakthrough(&self, target_realm: &ElementMasteryRealm) -> bool {
        let requirement = self.breakthrough_requirements.get(target_realm)?;
        self.current_mastery >= requirement.min_mastery
    }
    
    pub fn attempt_breakthrough(
        &mut self,
        target_realm: &ElementMasteryRealm,
        target_stage: &ElementMasteryStage,
        scaling: &ElementalRealmScaling,
        seed: &str
    ) -> Result<f64, BreakthroughError> {
        if !self.can_breakthrough(target_realm) {
            return Err(BreakthroughError::InsufficientMastery);
        }
        
        let new_mastery = scaling.calculate_breakthrough_stats(
            self.current_mastery,
            target_realm,
            target_stage,
            seed
        );
        
        self.current_realm = target_realm.clone();
        self.current_stage = target_stage.clone();
        self.current_mastery = new_mastery;
        
        Ok(new_mastery)
    }
}
```

### 5. Decay System

#### A. Time-Based Decay
- **Base Decay Rate**: 0.5% per day
- **Long Absence Threshold**: 7 days
- **Long Absence Multiplier**: 1.5x decay after 7 days

#### B. Opposite Element Decay
- **Fire vs Water**: Sử dụng Water element sẽ tăng decay rate của Fire mastery
- **Light vs Dark**: Sử dụng Dark element sẽ tăng decay rate của Light mastery
- **Decay Multiplier**: 2.0x khi sử dụng opposite element

#### C. Decay Calculation
```rust
fn calculate_mastery_decay(&self, actor: &Actor, element_id: &str) -> f64 {
    let base_decay = self.decay_config.time_decay_rate;
    let decay_multiplier = plugin.calculate_decay_rate(actor);
    
    // Check for opposite element usage
    let opposite_elements = plugin.get_opposite_elements();
    let opposite_element_usage = self.calculate_opposite_element_usage(actor, &opposite_elements);
    let opposite_decay_multiplier = if opposite_element_usage > 0.0 {
        self.decay_config.opposite_element_decay_multiplier
    } else {
        1.0
    };
    
    base_decay * decay_multiplier * opposite_decay_multiplier
}
```

### 3. Training Methods

#### A. Fire Element Training Methods
```rust
fn get_training_methods(&self) -> Vec<TrainingMethod> {
    vec![
        TrainingMethod {
            id: "fire_meditation".to_string(),
            name: "Fire Meditation".to_string(),
            description: "Meditate with fire element to increase mastery".to_string(),
            efficiency_multiplier: 1.0,
            requirements: vec![
                TrainingRequirement {
                    stat_name: "fire_mastery".to_string(),
                    min_value: 0.0,
                    description: "No minimum mastery required".to_string(),
                },
            ],
            rewards: vec![
                TrainingReward {
                    stat_name: "fire_experience".to_string(),
                    amount: 10.0,
                    description: "Gain fire experience".to_string(),
                },
            ],
        },
        TrainingMethod {
            id: "fire_combat_training".to_string(),
            name: "Fire Combat Training".to_string(),
            description: "Practice fire-based combat techniques".to_string(),
            efficiency_multiplier: 1.5,
            requirements: vec![
                TrainingRequirement {
                    stat_name: "fire_mastery".to_string(),
                    min_value: 10.0,
                    description: "Requires 10% fire mastery".to_string(),
                },
            ],
            rewards: vec![
                TrainingReward {
                    stat_name: "fire_experience".to_string(),
                    amount: 15.0,
                    description: "Gain fire experience".to_string(),
                },
            ],
        },
    ]
}
```

#### B. Training Efficiency
- **Base Efficiency**: 1.0x
- **Method Bonuses**: 1.5x for combat training, 2.0x for advanced methods
- **Location Bonuses**: Volcano (+2.0x), Fire Temple (+1.5x)
- **Equipment Bonuses**: Fire Staff (+1.2x), Fire Robes (+1.1x)

## Element Interactions

### 1. Ngũ Hành (Five Elements) Interactions

#### A. Tương Sinh (Generating)
- **Fire → Earth**: Fire generates Earth (0.6x multiplier)
- **Earth → Metal**: Earth generates Metal (0.6x multiplier)
- **Metal → Water**: Metal generates Water (0.6x multiplier)
- **Water → Wood**: Water generates Wood (0.6x multiplier)
- **Wood → Fire**: Wood generates Fire (0.6x multiplier)

#### B. Tương Khắc (Overcoming)
- **Fire → Water**: Fire is overcome by Water (1.5x multiplier)
- **Water → Fire**: Water is overcome by Fire (1.5x multiplier)
- **Earth → Wood**: Earth is overcome by Wood (1.5x multiplier)
- **Wood → Earth**: Wood is overcome by Earth (1.5x multiplier)
- **Metal → Fire**: Metal is overcome by Fire (1.5x multiplier)

### 2. Light/Dark Interactions

#### A. Light vs Dark
- **Light → Dark**: Light is overcome by Dark (2.0x multiplier)
- **Dark → Light**: Dark is overcome by Light (2.0x multiplier)

### 3. Advanced Element Interactions

#### A. Life/Death
- **Life → Death**: Life is overcome by Death (1.8x multiplier)
- **Death → Life**: Death is overcome by Life (1.8x multiplier)

#### B. Time/Space
- **Time → Space**: Time is overcome by Space (1.7x multiplier)
- **Space → Time**: Space is overcome by Time (1.7x multiplier)

## Derived Stats Integration

### 1. Element Mastery vs Experience Points Distinction

#### A. Experience Points (Điểm Kinh Nghiệm) - Raw Data with i64 Scaling
```rust
pub struct ElementExperienceData {
    pub element_id: String,
    pub experience_points: i64,        // Raw experience points (i64 for large scaling)
    pub training_time: i64,            // Time spent training (minutes)
    pub combat_usage: i64,             // Combat usage count
    pub meditation_time: i64,          // Meditation time (minutes)
    pub last_used: SystemTime,         // Last time element was used
    pub total_breakthroughs: u32,      // Number of realm breakthroughs
    pub realm_multiplier: i64,         // Current realm experience multiplier
}

impl ElementExperienceData {
    pub fn calculate_total_experience(&self) -> i64 {
        let base_experience = self.experience_points + 
                             (self.training_time * 100) +     // 100 exp per minute training
                             (self.combat_usage * 50) +       // 50 exp per combat usage
                             (self.meditation_time * 150);    // 150 exp per minute meditation
        
        // Apply realm multiplier for exponential scaling
        base_experience * self.realm_multiplier
    }
    
    pub fn get_experience_tier(&self) -> ExperienceTier {
        let total_exp = self.calculate_total_experience();
        match total_exp {
            0..=999 => ExperienceTier::Mortal,
            1000..=9999 => ExperienceTier::Transcendent,
            10000..=99999 => ExperienceTier::Divine,
            100000..=999999 => ExperienceTier::Immortal,
            1000000..=9999999 => ExperienceTier::Celestial,
            10000000..=99999999 => ExperienceTier::Legendary,
            100000000..=999999999 => ExperienceTier::Mythic,
            1000000000..=9999999999 => ExperienceTier::Transcendent,
            _ => ExperienceTier::Supreme,
        }
    }
}

pub enum ExperienceTier {
    Mortal,        // 0-999 exp
    Transcendent,  // 1K-9.9K exp
    Divine,        // 10K-99.9K exp
    Immortal,      // 100K-999.9K exp
    Celestial,     // 1M-9.9M exp
    Legendary,     // 10M-99.9M exp
    Mythic,        // 100M-999.9M exp
    Transcendent,  // 1B-9.9B exp
    Supreme,       // 10B+ exp
}

impl ExperienceTier {
    pub fn get_tier_multiplier(&self) -> i64 {
        match self {
            ExperienceTier::Mortal => 1,
            ExperienceTier::Transcendent => 10,
            ExperienceTier::Divine => 100,
            ExperienceTier::Immortal => 1000,
            ExperienceTier::Celestial => 10000,
            ExperienceTier::Legendary => 100000,
            ExperienceTier::Mythic => 1000000,
            ExperienceTier::Transcendent => 10000000,
            ExperienceTier::Supreme => 100000000,
        }
    }
    
    pub fn get_tier_name_vi(&self) -> &'static str {
        match self {
            ExperienceTier::Mortal => "Phàm Nhân",
            ExperienceTier::Transcendent => "Siêu Phàm",
            ExperienceTier::Divine => "Thần Thánh",
            ExperienceTier::Immortal => "Bất Tử",
            ExperienceTier::Celestial => "Thiên Thần",
            ExperienceTier::Legendary => "Huyền Thoại",
            ExperienceTier::Mythic => "Thần Thoại",
            ExperienceTier::Transcendent => "Siêu Việt",
            ExperienceTier::Supreme => "Tối Thượng",
        }
    }
}
```

#### B. Element Mastery (Thông Thạo Nguyên Tố) - Derived Stats
```rust
pub struct ElementMasteryStats {
    pub element_id: String,
    pub mastery_level: ElementMasteryLevel,
    pub mastery_realm: ElementMasteryRealm,
    pub mastery_stage: ElementMasteryStage,
    pub power_tier: ElementalPowerTier,
    pub mastery_value: f64,            // Final calculated mastery value
    pub power_scale: f64,              // Power scale value
    pub derived_stats: HashMap<String, f64>, // All derived stats from mastery
}

impl ElementMasteryStats {
    pub fn from_experience(exp_data: &ElementExperienceData) -> Self {
        let total_exp = exp_data.calculate_total_experience();
        let mastery_level = ElementMasteryLevel::from_experience(total_exp);
        let mastery_value = Self::calculate_mastery_value(exp_data, &mastery_level);
        let mastery_realm = ElementMasteryRealm::from_mastery(mastery_value);
        let mastery_stage = ElementMasteryStage::from_mastery(mastery_value, &mastery_realm);
        let power_scale = Self::calculate_power_scale(mastery_value, &mastery_realm, &mastery_stage);
        let power_tier = ElementalPowerTier::from_power(power_scale);
        
        Self {
            element_id: exp_data.element_id.clone(),
            mastery_level,
            mastery_realm,
            mastery_stage,
            power_tier,
            mastery_value,
            power_scale,
            derived_stats: Self::calculate_derived_stats(mastery_value, &mastery_realm, &mastery_stage),
        }
    }
    
    fn calculate_mastery_value(exp_data: &ElementExperienceData, level: &ElementMasteryLevel) -> f64 {
        let base_mastery = exp_data.calculate_total_experience() / 1000.0;
        let level_bonus = level.get_level_bonus();
        let realm_bonus = if exp_data.total_breakthroughs > 0 {
            1.0 + (exp_data.total_breakthroughs as f64 * 0.1)
        } else {
            1.0
        };
        
        base_mastery * level_bonus * realm_bonus
    }
    
    fn calculate_power_scale(mastery: f64, realm: &ElementMasteryRealm, stage: &ElementMasteryStage) -> f64 {
        let base_power = mastery * 0.1;
        let realm_multiplier = realm.get_realm_multiplier();
        let stage_multiplier = stage.get_stage_multiplier();
        
        base_power * realm_multiplier * stage_multiplier
    }
    
    fn calculate_derived_stats(mastery: f64, realm: &ElementMasteryRealm, stage: &ElementMasteryStage) -> HashMap<String, f64> {
        let realm_multiplier = realm.get_realm_multiplier();
        let stage_multiplier = stage.get_stage_multiplier();
        let total_multiplier = realm_multiplier * stage_multiplier;
        
        HashMap::from([
            ("element_mastery".to_string(), mastery),
            ("element_power_scale".to_string(), mastery * total_multiplier),
            ("element_control".to_string(), mastery * 0.1 * total_multiplier),
            ("element_harmony".to_string(), mastery * 0.08 * total_multiplier),
            ("element_transcendence".to_string(), mastery * 0.05 * total_multiplier),
        ])
    }
}
```

### 2. Element-Specific Derived Stats

#### A. Fire Element Stats
```rust
fn get_derived_stats(&self, actor: &Actor) -> HashMap<String, f64> {
    let mastery = self.calculate_base_mastery(actor);
    let intelligence = actor.get_data().get("intelligence")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    let wisdom = actor.get_data().get("wisdom")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    let dexterity = actor.get_data().get("dexterity")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    
    HashMap::from([
        ("fire_attack_power".to_string(), mastery * 0.1 + intelligence * 0.05),
        ("fire_defense".to_string(), mastery * 0.08 + wisdom * 0.03),
        ("fire_crit_rate".to_string(), mastery * 0.02 + dexterity * 0.01),
        ("fire_accuracy".to_string(), mastery * 0.015 + dexterity * 0.02),
        ("fire_buff_resistance".to_string(), mastery * 0.01),
        ("fire_buff_reception".to_string(), mastery * 0.01),
    ])
}
```

#### B. Water Element Stats
```rust
fn get_derived_stats(&self, actor: &Actor) -> HashMap<String, f64> {
    let mastery = self.calculate_base_mastery(actor);
    let intelligence = actor.get_data().get("intelligence")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    let wisdom = actor.get_data().get("wisdom")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    let constitution = actor.get_data().get("constitution")
        .and_then(|v| v.as_f64())
        .unwrap_or(10.0);
    
    HashMap::from([
        ("water_attack_power".to_string(), mastery * 0.1 + intelligence * 0.05),
        ("water_defense".to_string(), mastery * 0.08 + wisdom * 0.03),
        ("water_crit_rate".to_string(), mastery * 0.02 + constitution * 0.01),
        ("water_accuracy".to_string(), mastery * 0.015 + constitution * 0.02),
        ("water_buff_resistance".to_string(), mastery * 0.01),
        ("water_buff_reception".to_string(), mastery * 0.01),
    ])
}
```

### 2. Integration với Element Core

#### A. Element Mastery Stats Provider
```rust
/// Element Mastery Stats Provider for Element Core
pub struct ElementMasteryStatsProvider {
    /// Element mastery system
    mastery_system: PluginBasedElementMasterySystem,
}

impl ElementMasteryStatsProvider {
    /// Get element mastery stats for Element Core
    pub async fn get_element_mastery_stats(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        self.mastery_system.calculate_resources(actor).await
    }
    
    /// Get specific element mastery
    pub async fn get_element_mastery(&self, actor: &Actor, element: ElementType) -> ActorCoreResult<f64> {
        let element_id = format!("{}_mastery", element.to_string());
        let resources = self.get_element_mastery_stats(actor).await?;
        Ok(resources.get(&format!("{}_current", element_id)).copied().unwrap_or(0.0))
    }
}
```

## Configuration System

### 1. YAML Configuration

#### A. Elemental Mastery System Configuration
```yaml
# elemental_mastery_system.yaml
elemental_mastery_system:
  # Realm scaling configuration
  realm_scaling:
    realm_base_a: 1.0
    realm_base_b: 8.0  # Exponential gap between realms
    
    # Stage multipliers
    stage_multipliers:
      early: 1.0
      mid: 1.25
      late: 1.5
      peak: 1.75
    
    # Realm ranks (for exponential calculation)
    realm_ranks:
      elemental_awareness: 0
      elemental_control: 1
      elemental_harmony: 2
      elemental_transcendence: 3
      elemental_ascension: 4
    
    # Baselines for each realm
    baselines:
      elemental_awareness: 100.0
      elemental_control: 1000.0
      elemental_harmony: 5000.0
      elemental_transcendence: 25000.0
      elemental_ascension: 100000.0
    
    # Min/Max deltas for random bonuses
    min_max_deltas:
      elemental_awareness: [5.0, 15.0]
      elemental_control: [10.0, 30.0]
      elemental_harmony: [25.0, 75.0]
      elemental_transcendence: [100.0, 300.0]
      elemental_ascension: [500.0, 1500.0]

  # Mastery level configuration - Extended System with i64 Scaling
  mastery_levels:
    # Basic Levels (Cấp Cơ Bản)
    beginner:
      min_experience: 0
      max_experience: 999
      level_bonus: 1.0
      name_vi: "Người Mới Bắt Đầu"
      tier: "basic"
      experience_tier: "mortal"
    
    novice:
      min_experience: 1000
      max_experience: 4999
      level_bonus: 1.1
      name_vi: "Sơ Cấp"
      tier: "basic"
      experience_tier: "transcendent"
    
    apprentice:
      min_experience: 5000
      max_experience: 14999
      level_bonus: 1.25
      name_vi: "Học Việc"
      tier: "basic"
      experience_tier: "divine"
    
    regular:
      min_experience: 15000
      max_experience: 49999
      level_bonus: 1.4
      name_vi: "Thường Xuyên"
      tier: "basic"
      experience_tier: "immortal"
    
    # Intermediate Levels (Cấp Trung Cấp)
    adept:
      min_experience: 50000
      max_experience: 149999
      level_bonus: 1.6
      name_vi: "Thành Thạo"
      tier: "intermediate"
      experience_tier: "celestial"
    
    expert:
      min_experience: 150000
      max_experience: 499999
      level_bonus: 1.8
      name_vi: "Chuyên Gia"
      tier: "intermediate"
      experience_tier: "legendary"
    
    advanced_expert:
      min_experience: 500000
      max_experience: 1499999
      level_bonus: 2.0
      name_vi: "Chuyên Gia Nâng Cao"
      tier: "intermediate"
      experience_tier: "mythic"
    
    master:
      min_experience: 1500000
      max_experience: 4999999
      level_bonus: 2.3
      name_vi: "Bậc Thầy"
      tier: "intermediate"
      experience_tier: "transcendent"
    
    # Advanced Levels (Cấp Cao Cấp)
    advanced_master:
      min_experience: 2500
      max_experience: 3499
      level_bonus: 2.7
      name_vi: "Bậc Thầy Nâng Cao"
      tier: "advanced"
    
    grand_master:
      min_experience: 3500
      max_experience: 4999
      level_bonus: 3.2
      name_vi: "Đại Sư"
      tier: "advanced"
    
    completer:
      min_experience: 5000
      max_experience: 6999
      level_bonus: 3.8
      name_vi: "Hoàn Thiện"
      tier: "advanced"
    
    transcender:
      min_experience: 7000
      max_experience: 9999
      level_bonus: 4.5
      name_vi: "Siêu Việt"
      tier: "advanced"
    
    # Legendary Levels (Cấp Huyền Thoại)
    sage:
      min_experience: 10000
      max_experience: 14999
      level_bonus: 5.5
      name_vi: "Hiền Triết"
      tier: "legendary"
    
    archmage:
      min_experience: 15000
      max_experience: 21999
      level_bonus: 6.8
      name_vi: "Pháp Sư Tối Cao"
      tier: "legendary"
    
    legendary:
      min_experience: 22000
      max_experience: 31999
      level_bonus: 8.5
      name_vi: "Huyền Thoại"
      tier: "legendary"
    
    mythic:
      min_experience: 32000
      max_experience: 44999
      level_bonus: 10.5
      name_vi: "Thần Thoại"
      tier: "legendary"
    
    # Transcendent Levels (Cấp Siêu Việt)
    transcendent:
      min_experience: 45000
      max_experience: 64999
      level_bonus: 13.0
      name_vi: "Siêu Việt"
      tier: "transcendent"
    
    celestial:
      min_experience: 65000
      max_experience: 89999
      level_bonus: 16.0
      name_vi: "Thiên Thần"
      tier: "transcendent"
    
    divine:
      min_experience: 90000
      max_experience: 129999
      level_bonus: 20.0
      name_vi: "Thần Thánh"
      tier: "transcendent"
    
    immortal:
      min_experience: 130000
      max_experience: 179999
      level_bonus: 25.0
      name_vi: "Bất Tử"
      tier: "transcendent"
    
    # Ultimate Levels (Cấp Tối Thượng)
    eternal:
      min_experience: 180000
      max_experience: 249999
      level_bonus: 32.0
      name_vi: "Vĩnh Cửu"
      tier: "ultimate"
    
    omniscient:
      min_experience: 250000
      max_experience: 349999
      level_bonus: 40.0
      name_vi: "Toàn Tri"
      tier: "ultimate"
    
    omnipotent:
      min_experience: 350000
      max_experience: 499999
      level_bonus: 50.0
      name_vi: "Toàn Năng"
      tier: "ultimate"
    
    supreme:
      min_experience: 500000
      max_experience: 999999999
      level_bonus: 65.0
      name_vi: "Tối Thượng"
      tier: "ultimate"

  # Mastery level tiers configuration
  mastery_tiers:
    basic:
      name_vi: "Cấp Cơ Bản"
      tier_multiplier: 1.0
      description: "Các cấp độ cơ bản cho người mới bắt đầu"
    
    intermediate:
      name_vi: "Cấp Trung Cấp"
      tier_multiplier: 1.5
      description: "Các cấp độ trung cấp cho người chơi có kinh nghiệm"
    
    advanced:
      name_vi: "Cấp Cao Cấp"
      tier_multiplier: 2.5
      description: "Các cấp độ cao cấp cho người chơi thành thạo"
    
    legendary:
      name_vi: "Cấp Huyền Thoại"
      tier_multiplier: 4.0
      description: "Các cấp độ huyền thoại cho người chơi xuất sắc"
    
    transcendent:
      name_vi: "Cấp Siêu Việt"
      tier_multiplier: 6.5
      description: "Các cấp độ siêu việt cho người chơi đỉnh cao"
    
    ultimate:
      name_vi: "Cấp Tối Thượng"
      tier_multiplier: 10.0
      description: "Các cấp độ tối thượng cho người chơi vĩ đại"

  # Power tier configuration
  power_tiers:
    mortal:
      min_power: 0
      max_power: 999
      name_vi: "Phàm Nhân"
    
    transcendent:
      min_power: 1000
      max_power: 9999
      name_vi: "Siêu Phàm"
    
    divine:
      min_power: 10000
      max_power: 99999
      name_vi: "Thần Thánh"
    
    immortal:
      min_power: 100000
      max_power: 999999
      name_vi: "Bất Tử"
    
    celestial:
      min_power: 1000000
      max_power: 999999999
      name_vi: "Thiên Thần"

  # Power Scale First System Configuration
  power_scale_system:
    # Base power scale calculation
    base_power_scale_formula: "log10(experience / 1000000.0) * 1000.0"
    min_power_scale: 1.0
    max_power_scale: 1000000.0
    
    # Level progression (power scale increase per level)
    level_progression_rate: 0.1  # 10% increase per level
    
    # Experience calculation weights with i64 scaling
    experience_weights:
      base_experience: 1
      training_time: 100       # per minute (i64)
      combat_usage: 50         # per usage (i64)
      meditation_time: 150     # per minute (i64)
    
    # Realm multipliers for power scale (exponential scaling)
    realm_multipliers:
      0: 1.0                   # No breakthroughs (1x power scale)
      1: 2.0                   # First breakthrough (2x power scale)
      2: 4.0                   # Second breakthrough (4x power scale)
      3: 8.0                   # Third breakthrough (8x power scale)
      4: 16.0                  # Fourth breakthrough (16x power scale)
      5: 32.0                  # Fifth breakthrough (32x power scale)
      6: 64.0                  # Sixth breakthrough (64x power scale)
      7: 128.0                 # Seventh breakthrough (128x power scale)
      8: 256.0                 # Eighth breakthrough (256x power scale)
      9: 512.0                 # Ninth breakthrough (512x power scale)
      10: 1024.0               # Tenth breakthrough (1024x power scale)
      11: 2048.0               # Eleventh breakthrough (2048x power scale)
      12: 4096.0               # Twelfth breakthrough (4096x power scale - cap)
    
    # Power scale display configuration
    display_config:
      power_scale_precision: 2        # Decimal places for power scale
      experience_format_units: ["", "K", "M", "B", "T"]  # Experience unit suffixes
      experience_format_thresholds: [1000, 1000000, 1000000000, 1000000000000]  # Unit thresholds
      
    # Reverse formula configuration
    reverse_formula:
      # Formula to calculate required experience from target power scale
      # experience = 1000000.0 * 10^(base_power_scale / 1000.0)
      base_experience: 1000000.0
      power_scale_divisor: 1000.0
      exponential_base: 10.0

  # Decay configuration
  decay_config:
    time_decay_rate: 0.5  # 0.5% per day
    opposite_element_decay_multiplier: 2.0  # 2x decay when using opposite
    long_absence_threshold: 7.0  # 7 days
    long_absence_decay_multiplier: 1.5  # 1.5x decay after long absence

  # Breakthrough requirements
  breakthrough_requirements:
    elemental_control:
      min_mastery: 1000.0
      required_elements: []
      breakthrough_cost:
        qi: 1000.0
        vitality: 500.0
      breakthrough_time: "1h"
      success_rate: 0.8
    
    elemental_harmony:
      min_mastery: 3000.0
      required_elements: ["opposite_element"]
      breakthrough_cost:
        qi: 5000.0
        vitality: 2500.0
        life_force: 100.0
      breakthrough_time: "4h"
      success_rate: 0.6
    
    elemental_transcendence:
      min_mastery: 6000.0
      required_elements: ["opposite_element", "neutral_element"]
      breakthrough_cost:
        qi: 25000.0
        vitality: 12500.0
        life_force: 500.0
        energy: 1000.0
      breakthrough_time: "12h"
      success_rate: 0.4
    
    elemental_ascension:
      min_mastery: 10000.0
      required_elements: ["all_elements"]
      breakthrough_cost:
        qi: 100000.0
        vitality: 50000.0
        life_force: 2500.0
        energy: 5000.0
        soul_essence: 100.0
      breakthrough_time: "24h"
      success_rate: 0.2
```

#### B. Element Plugins Configuration
```yaml
# element_plugins.yaml
elements:
  fire:
    plugin_type: "fire"
    enabled: true
    element_modifier: 1.0
    element_scaling: 1.0
    opposite_elements: ["water"]
    
  water:
    plugin_type: "water"
    enabled: true
    element_modifier: 1.0
    element_scaling: 1.0
    opposite_elements: ["fire"]
    
  earth:
    plugin_type: "custom"
    enabled: true
    element_modifier: 1.2
    element_scaling: 1.1
    opposite_elements: ["wood"]
    config:
      name: "Earth Mastery"
      category: "ngu_hanh"
      training_methods:
        - id: "earth_meditation"
          name: "Earth Meditation"
          efficiency_multiplier: 1.0
          requirements: []
          rewards:
            - stat_name: "earth_experience"
              amount: 10.0
      interactions:
        - target_element: "wood"
          interaction_type: "overcoming"
          multiplier: 1.5
        - target_element: "fire"
          interaction_type: "generating"
          multiplier: 0.6
```

### 2. Runtime Configuration Loading

```rust
impl PluginBasedElementMasterySystem {
    /// Load plugins from configuration
    pub fn load_plugins_from_config(&mut self, config_path: &str) -> ActorCoreResult<()> {
        let config = load_yaml_config(config_path)?;
        
        for (element_id, element_config) in config.elements {
            let plugin = self.create_plugin_from_config(element_id, element_config)?;
            self.register_element_plugin(Box::new(plugin));
        }
        
        Ok(())
    }
}
```

## Event System

### 1. Element Mastery Events

```rust
/// Element Mastery Events
#[derive(Debug, Clone)]
pub enum ElementMasteryEvent {
    /// Mastery level changed
    MasteryChanged {
        element_id: String,
        old_mastery: f64,
        new_mastery: f64,
    },
    /// Mastery decay occurred
    MasteryDecay {
        element_id: String,
        decay_amount: f64,
        new_mastery: f64,
    },
    /// Mastery gained from training
    MasteryGained {
        element_id: String,
        gain_amount: f64,
        new_mastery: f64,
    },
    /// Element training completed
    ElementTrainingCompleted {
        element_id: String,
        training_amount: f64,
        new_experience: f64,
    },
    /// Opposite element usage detected
    OppositeElementUsage {
        element_id: String,
        opposite_element_id: String,
        decay_multiplier: f64,
    },
}
```

### 2. Event Handler

```rust
/// Element Mastery Event Handler
pub struct ElementMasteryEventHandler {
    /// Event listeners
    listeners: Vec<Box<dyn Fn(ElementMasteryEvent) + Send + Sync>>,
}

impl ElementMasteryEventHandler {
    /// Create a new event handler
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
    
    /// Add an event listener
    pub fn add_listener<F>(&mut self, listener: F)
    where
        F: Fn(ElementMasteryEvent) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }
    
    /// Emit an event
    pub fn emit(&self, event: ElementMasteryEvent) {
        for listener in &self.listeners {
            listener(event.clone());
        }
    }
}
```

## Performance Considerations

### 1. Caching Strategy

#### A. Mastery Calculation Cache
```rust
pub struct MasteryCalculationCache {
    /// Cached mastery values
    mastery_cache: HashMap<String, f64>,
    /// Cache timestamps
    cache_timestamps: HashMap<String, SystemTime>,
    /// Cache TTL
    cache_ttl: Duration,
}
```

#### B. Derived Stats Cache
```rust
pub struct DerivedStatsCache {
    /// Cached derived stats
    stats_cache: HashMap<String, HashMap<String, f64>>,
    /// Cache invalidation triggers
    invalidation_triggers: HashMap<String, Vec<String>>,
}
```

### 2. Lazy Loading

#### A. Plugin Lazy Loading
```rust
impl ElementPluginRegistry {
    /// Load plugin only when needed
    pub fn get_plugin_lazy(&mut self, element_id: &str) -> ActorCoreResult<&dyn ElementPlugin> {
        if !self.plugins.contains_key(element_id) {
            self.load_plugin_from_config(element_id)?;
        }
        Ok(self.plugins.get(element_id).unwrap().as_ref())
    }
}
```

## Testing Strategy

### 1. Unit Tests

#### A. Plugin Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fire_plugin_mastery_calculation() {
        let plugin = FireElementPlugin;
        let mut actor = Actor::new();
        actor.set_data("fire_experience", 1000.0.into());
        actor.set_data("fire_training_time", 100.0.into());
        
        let mastery = plugin.calculate_base_mastery(&actor);
        assert_eq!(mastery, 2.0); // (1000/1000) + (100/100) = 2.0
    }
    
    #[test]
    fn test_fire_plugin_decay_calculation() {
        let plugin = FireElementPlugin;
        let mut actor = Actor::new();
        actor.set_data("fire_last_used", (SystemTime::now() - Duration::from_secs(86400 * 8)).into());
        
        let decay_rate = plugin.calculate_decay_rate(&actor);
        assert_eq!(decay_rate, 1.5); // Long absence multiplier
    }
}
```

### 2. Integration Tests

#### A. System Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_element_mastery_system_integration() {
        let mut system = PluginBasedElementMasterySystem::new();
        system.register_element_plugin(Box::new(FireElementPlugin));
        
        let mut actor = Actor::new();
        system.handle_element_training(&mut actor, "fire", 100.0).await.unwrap();
        
        let mastery = system.calculate_element_mastery(&actor, "fire");
        assert!(mastery > 0.0);
    }
}
```

## Future Enhancements

### 1. Advanced Features

#### A. Element Fusion
- **Fire + Water = Steam**: Tạo ra element mới từ việc kết hợp
- **Light + Dark = Balance**: Element cân bằng từ opposite elements
- **Time + Space = Void**: Element không gian-thời gian

#### B. Element Mastery Trees
- **Specialization Paths**: Fire Mage, Fire Warrior, Fire Healer
- **Cross-Element Synergies**: Fire + Earth = Lava, Water + Air = Storm
- **Mastery Bonuses**: Unlock special abilities at high mastery levels

#### C. Element Locations
- **Elemental Shrines**: Special locations for training specific elements
- **Elemental Dungeons**: Dungeons with element-specific challenges
- **Elemental Events**: World events that boost specific element training

### 2. Social Features

#### A. Element Guilds
- **Fire Guild**: Guild focused on fire element mastery
- **Elemental Competitions**: PvP competitions based on element mastery
- **Mentorship System**: High mastery players can mentor others

#### B. Element Trading
- **Elemental Essences**: Tradeable items that boost element mastery
- **Mastery Transfer**: Temporary transfer of mastery between players
- **Elemental Artifacts**: Rare items that provide permanent mastery bonuses

## Summary

### 1. Elemental Mastery System Architecture

#### A. Core Components
- **Experience Points System**: Raw data tracking with i64 scaling (experience, training time, combat usage, meditation)
- **Mastery Levels**: 24 levels across 6 tiers from Beginner to Supreme with exponential bonuses
- **Mastery Level Tiers**: 6 tiers (Basic, Intermediate, Advanced, Legendary, Transcendent, Ultimate)
- **Mastery Realms**: 5 realms from Elemental Awareness to Elemental Ascension
- **Realm Stages**: 4 stages (Early, Mid, Late, Peak) within each realm
- **Power Tiers**: 5 tiers from Mortal to Celestial based on power scale
- **Level Progression System**: Complex progression with unlock requirements and bonuses
- **Experience Tiers**: 9 tiers from Mortal (0-999) to Supreme (10B+) for scaling management

#### B. Experience Scaling System (i64)
```yaml
# Experience Ranges by Level
Beginner:        0 - 999           (Mortal Tier)
Novice:          1K - 4.9K         (Transcendent Tier)
Apprentice:      5K - 14.9K        (Divine Tier)
Regular:         15K - 49.9K       (Immortal Tier)
Adept:           50K - 149.9K      (Celestial Tier)
Expert:          150K - 499.9K     (Legendary Tier)
Advanced Expert: 500K - 1.49M      (Mythic Tier)
Master:          1.5M - 4.99M      (Transcendent Tier)
Advanced Master: 5M - 14.9M        (Supreme Tier)
Grand Master:    15M - 49.9M       (Supreme Tier)
Completer:       50M - 149.9M      (Supreme Tier)
Transcender:     150M - 499.9M     (Supreme Tier)
Sage:            500M - 1.49B      (Supreme Tier)
Archmage:        1.5B - 4.99B      (Supreme Tier)
Legendary:       5B - 14.9B        (Supreme Tier)
Mythic:          15B - 49.9B       (Supreme Tier)
Transcendent:    50B - 149.9B      (Supreme Tier)
Celestial:       150B - 499.9B     (Supreme Tier)
Divine:          500B - 1.49T      (Supreme Tier)
Immortal:        1.5T - 4.99T      (Supreme Tier)
Eternal:         5T - 14.9T        (Supreme Tier)
Omniscient:      15T - 49.9T       (Supreme Tier)
Omnipotent:      50T - 149.9T      (Supreme Tier)
Supreme:         150T+             (Supreme Tier)
```

#### C. Realm Multiplier System
```yaml
# Exponential Scaling Based on Breakthroughs
0 breakthroughs:  1x multiplier
1 breakthrough:   10x multiplier
2 breakthroughs:  100x multiplier
3 breakthroughs:  1,000x multiplier
4 breakthroughs:  10,000x multiplier
5 breakthroughs:  100,000x multiplier
6 breakthroughs:  1,000,000x multiplier
7 breakthroughs:  10,000,000x multiplier
8 breakthroughs:  100,000,000x multiplier
9 breakthroughs:  1,000,000,000x multiplier
10 breakthroughs: 10,000,000,000x multiplier
11 breakthroughs: 100,000,000,000x multiplier
12+ breakthroughs: 1,000,000,000,000x multiplier (cap)
```

#### B. Key Distinctions - Power Scale First System
- **Power Scale (element_mastery)**: Primary derived stat, calculated first from experience data
- **Experience Points**: Raw data stored in actor data, used to calculate power scale
- **Reverse Formula**: Experience requirements calculated from target power scale
- **Transparency**: Both power scale and experience are visible to players
- **Combat Integration**: Power scale directly used in combat calculations

#### C. Power Scale First Calculation Flow
```yaml
# Calculation Flow
1. Experience Data (i64) → Base Power Scale (f64)
2. Base Power Scale → Apply Level Bonuses → Apply Tier Bonuses → Apply Realm Bonuses → Apply Element Modifiers
3. Final Power Scale = element_mastery derived stat
4. Reverse Formula: Target Power Scale → Required Experience (i64)

# Example Calculation
Experience: 1,000,000 (1M)
Base Power Scale: log10(1M / 1M) * 1000 = 0 * 1000 = 0 → min(1.0) = 1.0
Level Bonus: 1.6x (Adept level)
Tier Bonus: 1.5x (Intermediate tier)
Realm Bonus: 2.0x (1 breakthrough)
Element Modifier: 1.0x (Fire element)
Final Power Scale: 1.0 * 1.6 * 1.5 * 2.0 * 1.0 = 4.8

# Reverse Calculation
Target Power Scale: 5.28 (10% increase for next level)
Required Base Power Scale: 5.28 / (1.6 * 1.5 * 2.0 * 1.0) = 1.1
Required Experience: 1,000,000 * 10^(1.1 / 1000) = 1,000,000 * 10^0.0011 ≈ 1,002,540
```

### 2. Realm Scaling Model

#### A. Exponential Gap System
- **Formula**: `Gap(R) = A * (B ^ RealmRank[R])`
- **A = 1.0, B = 8.0**: Creates exponential gaps between realms
- **Deterministic RNG**: Uses seed for consistent results
- **Baseline Protection**: Prevents stat reduction after breakthrough

#### B. Breakthrough System
- **Requirements**: Minimum mastery + resource costs + time
- **Success Rates**: Decrease with higher realms (0.8 → 0.2)
- **Element Balance**: Requires opposite elements for higher realms
- **Resource Costs**: Qi, Vitality, Life Force, Energy, Soul Essence

### 3. Integration Points

#### A. Element Core Integration
- **Derived Stats**: `element_mastery`, `element_power_scale`, `element_control`, etc.
- **Combat Bonuses**: Mastery affects all elemental combat calculations
- **Status Effects**: Mastery influences status probability and intensity

#### B. Actor Core Integration
- **SystemResourceCalculator**: Implements Actor Core trait
- **Event System**: Emits mastery change events
- **Resource Management**: Manages elemental resources

### 4. Configuration System

#### A. YAML Configuration
- **Realm Scaling**: Configurable exponential factors
- **Mastery Levels**: Customizable level ranges and bonuses
- **Power Tiers**: Adjustable power thresholds
- **Breakthrough Requirements**: Flexible resource costs and requirements

#### B. Element Plugins
- **Plugin-Based**: Each element is a separate plugin
- **Custom Elements**: Support for custom element definitions
- **Element Modifiers**: Individual scaling factors per element

## Conclusion

Elemental Mastery System cung cấp một framework tu luyện hoàn chỉnh và linh hoạt cho việc phát triển tinh thông nguyên tố trong Chaos World MMORPG. Hệ thống phân biệt rõ ràng giữa:

1. **Experience Points** (dữ liệu thô) và **Element Mastery** (derived stats)
2. **Mastery Levels** (cấp độ thông thạo) và **Mastery Realms** (cảnh giới tu luyện)
3. **Power Scale** (thang đo sức mạnh) và **Power Tiers** (bậc sức mạnh)

Với kiến trúc plugin-based và realm scaling model dựa trên hệ thống tu luyện cũ, hệ thống có thể dễ dàng mở rộng với các element mới và cơ chế tu luyện phức tạp hơn.

Hệ thống tích hợp chặt chẽ với Element Core để cung cấp derived stats cho combat system, đồng thời duy trì tính độc lập như một cultivation system riêng biệt trong Actor Core framework.

---

**Related Documents:**
- [Element Core Overview](00_Element_Core_Overview.md)
- [Element Types Comprehensive Design](03_Element_Types_Comprehensive_Design.md)
- [Status Effect System Design](04_Status_Effect_System_Design.md)
- [Implementation Notes](06_Implementation_Notes.md)
- [Resource Manager Integration Design](07_Resource_Manager_Integration_Design.md)
