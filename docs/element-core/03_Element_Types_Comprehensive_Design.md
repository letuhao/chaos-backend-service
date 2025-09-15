# Element Types Comprehensive Design

## 📋 **Tổng Quan**

Tài liệu này mô tả chi tiết tất cả các loại element trong Element Core, bao gồm tương sinh tương khắc theo ngũ hành, các thuộc tính từ triết học phương Đông, giả kim thuật phương Tây, và các game nổi tiếng.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Tương Sinh Tương Khắc (Ngũ Hành)**
- **Tương Sinh**: Hỗ trợ và tăng cường lẫn nhau
- **Tương Khắc**: Đối kháng và làm suy yếu lẫn nhau
- **Cân Bằng**: Không có element nào quá mạnh

### **2. Đa Dạng Nguồn Gốc**
- **Triết Học Phương Đông**: Ngũ hành, âm dương
- **Giả Kim Thuật Phương Tây**: 4 nguyên tố cổ điển
- **Game References**: Grim Dawn, Diablo, Path of Exile
- **Thần Thoại**: Các hệ thống thần thoại khác nhau

## 🌟 **Element Categories Comprehensive**

### **1. Ngũ Hành (Five Elements)**

```rust
// Ngũ hành elements
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NguhangElement {
    // Ngũ hành cơ bản
    Kim,        // Metal (金) - Kim loại, sắc bén, cứng rắn
    Moc,        // Wood (木) - Gỗ, sinh trưởng, mềm mại
    Thuy,       // Water (水) - Nước, linh hoạt, thấm nhuần
    Hoa,        // Fire (火) - Lửa, nóng bỏng, phá hủy
    Tho,        // Earth (土) - Đất, ổn định, nuôi dưỡng
}

// Tương sinh (Generating Cycle)
pub const NGUHANG_GENERATING: [(NguhangElement, NguhangElement); 5] = [
    (NguhangElement::Kim, NguhangElement::Thuy),  // Kim sinh Thủy
    (NguhangElement::Thuy, NguhangElement::Moc),  // Thủy sinh Mộc
    (NguhangElement::Moc, NguhangElement::Hoa),   // Mộc sinh Hỏa
    (NguhangElement::Hoa, NguhangElement::Tho),   // Hỏa sinh Thổ
    (NguhangElement::Tho, NguhangElement::Kim),   // Thổ sinh Kim
];

// Tương khắc (Overcoming Cycle)
pub const NGUHANG_OVERCOMING: [(NguhangElement, NguhangElement); 5] = [
    (NguhangElement::Kim, NguhangElement::Moc),   // Kim khắc Mộc
    (NguhangElement::Moc, NguhangElement::Tho),   // Mộc khắc Thổ
    (NguhangElement::Tho, NguhangElement::Thuy),  // Thổ khắc Thủy
    (NguhangElement::Thuy, NguhangElement::Hoa),  // Thủy khắc Hỏa
    (NguhangElement::Hoa, NguhangElement::Kim),   // Hỏa khắc Kim
];
```

### **2. Âm Dương (Yin-Yang)**

```rust
// Âm dương elements
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum YinYangElement {
    // Âm (Yin) - Nữ tính, tối, lạnh, thụ động
    Am,         // Âm - Tối, lạnh, thụ động
    Thuy,       // Thủy - Nước, linh hoạt
    Moc,        // Mộc - Gỗ, sinh trưởng
    Tho,        // Thổ - Đất, ổn định
    
    // Dương (Yang) - Nam tính, sáng, nóng, chủ động
    Duong,      // Dương - Sáng, nóng, chủ động
    Hoa,        // Hỏa - Lửa, phá hủy
    Kim,        // Kim - Kim loại, sắc bén
    Phong,      // Phong - Gió, di chuyển
}

// Âm dương tương tác
pub const YINYANG_INTERACTIONS: [(YinYangElement, YinYangElement, f64); 4] = [
    (YinYangElement::Am, YinYangElement::Duong, 0.5),    // Âm vs Dương = cân bằng
    (YinYangElement::Thuy, YinYangElement::Hoa, 0.7),    // Thủy vs Hỏa = thủy thắng
    (YinYangElement::Moc, YinYangElement::Kim, 0.3),     // Mộc vs Kim = kim thắng
    (YinYangElement::Tho, YinYangElement::Phong, 0.6),   // Thổ vs Phong = thổ thắng
];
```

### **3. Classical Western Elements**

```rust
// 4 nguyên tố cổ điển phương Tây
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClassicalElement {
    Fire,       // Lửa - Nóng, khô
    Water,      // Nước - Lạnh, ướt
    Air,        // Không khí - Nóng, ướt
    Earth,      // Đất - Lạnh, khô
}

// Tương tác cổ điển
pub const CLASSICAL_INTERACTIONS: [(ClassicalElement, ClassicalElement, f64); 6] = [
    (ClassicalElement::Fire, ClassicalElement::Water, 0.3),   // Fire vs Water = water wins
    (ClassicalElement::Water, ClassicalElement::Fire, 1.7),   // Water vs Fire = water wins
    (ClassicalElement::Air, ClassicalElement::Earth, 1.2),    // Air vs Earth = air wins
    (ClassicalElement::Earth, ClassicalElement::Air, 0.8),    // Earth vs Air = earth wins
    (ClassicalElement::Fire, ClassicalElement::Air, 1.1),     // Fire vs Air = fire wins
    (ClassicalElement::Water, ClassicalElement::Earth, 1.3),  // Water vs Earth = water wins
];
```

### **4. Light & Dark Elements**

```rust
// Ánh sáng và bóng tối
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LightDarkElement {
    // Light elements
    Light,      // Ánh sáng - Thánh thiện, chữa lành
    Holy,       // Thánh - Thiêng liêng, bảo vệ
    Divine,     // Thần thánh - Quyền năng tối cao
    Sacred,     // Linh thiêng - Bảo vệ thiêng liêng
    Radiant,    // Rực rỡ - Ánh sáng mạnh mẽ
    
    // Dark elements
    Dark,       // Bóng tối - Tà ác, phá hủy
    Shadow,     // Bóng - Ẩn nấp, lừa dối
    Unholy,     // Tà ác - Ma quỷ, độc ác
    Void,       // Hư không - Trống rỗng, hấp thụ
    Chaos,      // Hỗn mang - Ngẫu nhiên, không kiểm soát
}

// Light vs Dark interactions
pub const LIGHTDARK_INTERACTIONS: [(LightDarkElement, LightDarkElement, f64); 5] = [
    (LightDarkElement::Light, LightDarkElement::Dark, 1.5),       // Light vs Dark = light wins
    (LightDarkElement::Holy, LightDarkElement::Unholy, 1.8),      // Holy vs Unholy = holy wins
    (LightDarkElement::Divine, LightDarkElement::Void, 1.3),      // Divine vs Void = divine wins
    (LightDarkElement::Sacred, LightDarkElement::Chaos, 1.6),     // Sacred vs Chaos = sacred wins
    (LightDarkElement::Radiant, LightDarkElement::Shadow, 1.4),   // Radiant vs Shadow = radiant wins
];
```

### **5. Life & Death Elements**

```rust
// Sinh và tử
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LifeDeathElement {
    // Life elements
    Life,       // Sự sống - Hồi phục, tăng trưởng
    Nature,     // Thiên nhiên - Cây cối, động vật
    Vitality,   // Sinh lực - Năng lượng sống
    Growth,     // Tăng trưởng - Phát triển, mở rộng
    Renewal,    // Tái sinh - Hồi sinh, làm mới
    
    // Death elements
    Death,      // Cái chết - Kết thúc, phá hủy
    Decay,      // Thối rữa - Phân hủy, suy tàn
    Necromancy, // Tà thuật - Hồi sinh xác chết
    Corruption, // Tham nhũng - Làm bẩn, suy đồi
    Entropy,    // Entropy - Hỗn loạn, tan rã
}

// Life vs Death interactions
pub const LIFEDEATH_INTERACTIONS: [(LifeDeathElement, LifeDeathElement, f64); 5] = [
    (LifeDeathElement::Life, LifeDeathElement::Death, 1.4),       // Life vs Death = life wins
    (LifeDeathElement::Nature, LifeDeathElement::Decay, 1.6),     // Nature vs Decay = nature wins
    (LifeDeathElement::Vitality, LifeDeathElement::Necromancy, 1.3), // Vitality vs Necromancy = vitality wins
    (LifeDeathElement::Growth, LifeDeathElement::Corruption, 1.5), // Growth vs Corruption = growth wins
    (LifeDeathElement::Renewal, LifeDeathElement::Entropy, 1.7),  // Renewal vs Entropy = renewal wins
];
```

### **6. Time & Space Elements**

```rust
// Thời gian và không gian
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeSpaceElement {
    // Time elements
    Time,       // Thời gian - Tốc độ, thời lượng
    Temporal,   // Thời gian - Ảnh hưởng thời gian
    Chronos,    // Chronos - Thần thời gian
    Duration,   // Thời lượng - Kéo dài, rút ngắn
    Momentum,   // Động lượng - Tốc độ, gia tốc
    
    // Space elements
    Space,      // Không gian - Khoảng cách, vị trí
    Spatial,    // Không gian - Ảnh hưởng không gian
    Void,       // Hư không - Trống rỗng, hấp thụ
    Dimension,  // Chiều không gian - Đa chiều
    Gravity,    // Trọng lực - Hút, đẩy
}

// Time vs Space interactions
pub const TIMESPACE_INTERACTIONS: [(TimeSpaceElement, TimeSpaceElement, f64); 3] = [
    (TimeSpaceElement::Time, TimeSpaceElement::Space, 1.0),       // Time vs Space = balanced
    (TimeSpaceElement::Temporal, TimeSpaceElement::Spatial, 1.1), // Temporal vs Spatial = temporal wins
    (TimeSpaceElement::Chronos, TimeSpaceElement::Void, 1.2),     // Chronos vs Void = chronos wins
];
```

### **7. Advanced Elements (Grim Dawn, Diablo Style)**

```rust
// Các element nâng cao từ game
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdvancedElement {
    // Physical elements
    Physical,   // Vật lý - Tấn công cơ bản
    Slashing,   // Chém - Kiếm, dao
    Piercing,   // Đâm - Thương, mũi tên
    Blunt,      // Đập - Búa, gậy
    Crushing,   // Nghiền - Sức mạnh lớn
    
    // Magical elements
    Arcane,     // Huyền bí - Ma thuật thuần túy
    Mystical,   // Thần bí - Ma thuật bí ẩn
    Spiritual,  // Tinh thần - Năng lượng tinh thần
    Mental,     // Tâm trí - Tấn công tinh thần
    Psychic,    // Tâm linh - Năng lượng tâm linh
    
    // Elemental combinations
    Steam,      // Hơi nước - Fire + Water
    Lava,       // Dung nham - Fire + Earth
    Ice,        // Băng - Water + Air
    Sand,       // Cát - Earth + Air
    Storm,      // Bão - Air + Lightning
    Lightning,  // Sét - Air + Fire
    Poison,     // Độc - Nature + Death
    Acid,       // Axit - Water + Poison
    Frost,      // Băng giá - Water + Ice
    Magma,      // Magma - Fire + Earth + Lava
    
    // Special elements
    True,       // Thật - Không thể chặn
    Pure,       // Thuần khiết - Không bị ảnh hưởng
    Raw,        // Thô - Năng lượng thô
    Primal,     // Nguyên thủy - Năng lượng gốc
    Cosmic,     // Vũ trụ - Năng lượng vũ trụ
    Ethereal,   // Siêu hình - Không vật chất
    Astral,     // Sao - Năng lượng sao
    Celestial,  // Thiên thể - Năng lượng thiên thể
}
```

### **8. Cultivation Elements (Tu Luyện)**

```rust
// Các element tu luyện
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CultivationElement {
    // Basic cultivation
    Qi,         // Khí - Năng lượng tu luyện cơ bản
    Spiritual,  // Tinh thần - Năng lượng tinh thần
    Dao,        // Đạo - Con đường tu luyện
    Profound,   // Áo nghĩa - Ý nghĩa sâu sắc
    Karma,      // Nghiệp - Nghiệp lực
    Fate,       // Số mệnh - Định mệnh
    
    // Advanced cultivation
    Immortal,   // Bất tử - Năng lượng bất tử
    Transcendent, // Siêu việt - Vượt qua giới hạn
    Divine,     // Thần thánh - Năng lượng thần thánh
    Primordial, // Nguyên thủy - Năng lượng gốc
    Eternal,    // Vĩnh cửu - Năng lượng vĩnh cửu
    Infinite,   // Vô hạn - Năng lượng vô hạn
    
    // Cultivation realms
    Foundation, // Nền tảng - Cảnh giới nền tảng
    Core,       // Lõi - Cảnh giới lõi
    Nascent,    // Sơ sinh - Cảnh giới sơ sinh
    Soul,       // Linh hồn - Cảnh giới linh hồn
    Body,       // Thân thể - Cảnh giới thân thể
    Mind,       // Tâm trí - Cảnh giới tâm trí
    Spirit,     // Tinh thần - Cảnh giới tinh thần
    Unity,      // Hợp nhất - Cảnh giới hợp nhất
}
```

## 🔄 **Element Interaction System**

### **1. Comprehensive Interaction Matrix**

```rust
// Element interaction system
pub struct ElementInteractionSystem {
    interactions: HashMap<(String, String), ElementInteraction>,
    generating_cycles: HashMap<String, Vec<String>>,
    overcoming_cycles: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementInteraction {
    pub attacker_element: String,
    pub defender_element: String,
    pub damage_multiplier: f64,
    pub interaction_type: InteractionType,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Generating,     // Tương sinh - Hỗ trợ
    Overcoming,     // Tương khắc - Đối kháng
    Neutral,        // Trung tính - Không ảnh hưởng
    Special,        // Đặc biệt - Tương tác đặc biệt
}

impl ElementInteractionSystem {
    // Get damage multiplier for element interaction
    pub fn get_damage_multiplier(
        &self,
        attacker_element: &str,
        defender_element: &str,
    ) -> f64 {
        if let Some(interaction) = self.interactions.get(&(attacker_element.to_string(), defender_element.to_string())) {
            interaction.damage_multiplier
        } else {
            1.0 // Neutral interaction
        }
    }
    
    // Check if elements are in generating cycle
    pub fn is_generating_cycle(&self, element1: &str, element2: &str) -> bool {
        if let Some(cycle) = self.generating_cycles.get(element1) {
            cycle.contains(&element2.to_string())
        } else {
            false
        }
    }
    
    // Check if elements are in overcoming cycle
    pub fn is_overcoming_cycle(&self, element1: &str, element2: &str) -> bool {
        if let Some(cycle) = self.overcoming_cycles.get(element1) {
            cycle.contains(&element2.to_string())
        } else {
            false
        }
    }
}
```

### **2. Ngũ Hành Interaction Rules**

```rust
// Ngũ hành interaction rules
pub fn create_nguhang_interactions() -> ElementInteractionSystem {
    let mut interactions = HashMap::new();
    let mut generating_cycles = HashMap::new();
    let mut overcoming_cycles = HashMap::new();
    
    // Tương sinh (Generating Cycle) - 1.2x damage
    let generating_pairs = [
        ("kim", "thuy"), ("thuy", "moc"), ("moc", "hoa"), 
        ("hoa", "tho"), ("tho", "kim")
    ];
    
    for (from, to) in generating_pairs.iter() {
        interactions.insert(
            (from.to_string(), to.to_string()),
            ElementInteraction {
                attacker_element: from.to_string(),
                defender_element: to.to_string(),
                damage_multiplier: 1.2,
                interaction_type: InteractionType::Generating,
                description: format!("{} sinh {}", from, to),
            }
        );
    }
    
    // Tương khắc (Overcoming Cycle) - 1.5x damage
    let overcoming_pairs = [
        ("kim", "moc"), ("moc", "tho"), ("tho", "thuy"), 
        ("thuy", "hoa"), ("hoa", "kim")
    ];
    
    for (from, to) in overcoming_pairs.iter() {
        interactions.insert(
            (from.to_string(), to.to_string()),
            ElementInteraction {
                attacker_element: from.to_string(),
                defender_element: to.to_string(),
                damage_multiplier: 1.5,
                interaction_type: InteractionType::Overcoming,
                description: format!("{} khắc {}", from, to),
            }
        );
    }
    
    // Setup cycles
    generating_cycles.insert("kim".to_string(), vec!["thuy".to_string()]);
    generating_cycles.insert("thuy".to_string(), vec!["moc".to_string()]);
    generating_cycles.insert("moc".to_string(), vec!["hoa".to_string()]);
    generating_cycles.insert("hoa".to_string(), vec!["tho".to_string()]);
    generating_cycles.insert("tho".to_string(), vec!["kim".to_string()]);
    
    overcoming_cycles.insert("kim".to_string(), vec!["moc".to_string()]);
    overcoming_cycles.insert("moc".to_string(), vec!["tho".to_string()]);
    overcoming_cycles.insert("tho".to_string(), vec!["thuy".to_string()]);
    overcoming_cycles.insert("thuy".to_string(), vec!["hoa".to_string()]);
    overcoming_cycles.insert("hoa".to_string(), vec!["kim".to_string()]);
    
    ElementInteractionSystem {
        interactions,
        generating_cycles,
        overcoming_cycles,
    }
}
```

### **3. Light vs Dark Interaction Rules**

```rust
// Light vs Dark interaction rules
pub fn create_lightdark_interactions() -> ElementInteractionSystem {
    let mut interactions = HashMap::new();
    
    // Light vs Dark - 1.8x damage
    let light_dark_pairs = [
        ("light", "dark"), ("holy", "unholy"), ("divine", "void"),
        ("sacred", "chaos"), ("radiant", "shadow")
    ];
    
    for (light, dark) in light_dark_pairs.iter() {
        interactions.insert(
            (light.to_string(), dark.to_string()),
            ElementInteraction {
                attacker_element: light.to_string(),
                defender_element: dark.to_string(),
                damage_multiplier: 1.8,
                interaction_type: InteractionType::Special,
                description: format!("{} vs {}", light, dark),
            }
        );
        
        // Reverse interaction - 0.3x damage
        interactions.insert(
            (dark.to_string(), light.to_string()),
            ElementInteraction {
                attacker_element: dark.to_string(),
                defender_element: light.to_string(),
                damage_multiplier: 0.3,
                interaction_type: InteractionType::Special,
                description: format!("{} vs {}", dark, light),
            }
        );
    }
    
    ElementInteractionSystem {
        interactions,
        generating_cycles: HashMap::new(),
        overcoming_cycles: HashMap::new(),
    }
}
```

## 🎮 **Game Reference Elements**

### **1. Grim Dawn Elements**

```rust
// Grim Dawn style elements
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GrimDawnElement {
    Physical,   // Vật lý
    Fire,       // Lửa
    Cold,       // Lạnh
    Lightning,  // Sét
    Acid,       // Axit
    Pierce,     // Đâm
    Poison,     // Độc
    Vitality,   // Sinh lực
    Aether,     // Ether
    Chaos,      // Hỗn mang
    Bleeding,   // Chảy máu
    Burn,       // Bỏng
    Frostburn,  // Bỏng lạnh
    Electrocute, // Điện giật
    Poison,     // Độc
    VitalityDecay, // Suy giảm sinh lực
    Aetherfire, // Lửa ether
    Chaosfire,  // Lửa hỗn mang
}
```

### **2. Diablo Elements**

```rust
// Diablo style elements
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiabloElement {
    Physical,   // Vật lý
    Fire,       // Lửa
    Cold,       // Lạnh
    Lightning,  // Sét
    Poison,     // Độc
    Arcane,     // Huyền bí
    Holy,       // Thánh
    Shadow,     // Bóng tối
    Chaos,      // Hỗn mang
    Void,       // Hư không
}
```

### **3. Path of Exile Elements**

```rust
// Path of Exile style elements
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PoEElement {
    Physical,   // Vật lý
    Fire,       // Lửa
    Cold,       // Lạnh
    Lightning,  // Sét
    Chaos,      // Hỗn mang
    Elemental,  // Nguyên tố
    Attack,     // Tấn công
    Spell,      // Phép thuật
    Minion,     // Tay sai
    Totem,      // Totem
    Trap,       // Bẫy
    Mine,       // Mìn
    Curse,      // Lời nguyền
    Aura,       // Hào quang
    Herald,     // Sứ giả
    Banner,     // Cờ hiệu
}
```

## 🧪 **Element Combination System**

### **1. Element Fusion**

```rust
// Element fusion system
pub struct ElementFusionSystem {
    fusion_recipes: HashMap<Vec<String>, String>,
    fusion_multipliers: HashMap<String, f64>,
}

impl ElementFusionSystem {
    // Fuse two elements
    pub fn fuse_elements(
        &self,
        element1: &str,
        element2: &str,
    ) -> Option<String> {
        let fusion_key = vec![element1.to_string(), element2.to_string()];
        self.fusion_recipes.get(&fusion_key).cloned()
    }
    
    // Get fusion multiplier
    pub fn get_fusion_multiplier(&self, fused_element: &str) -> f64 {
        self.fusion_multipliers.get(fused_element).copied().unwrap_or(1.0)
    }
}

// Fusion recipes
pub fn create_fusion_recipes() -> HashMap<Vec<String>, String> {
    let mut recipes = HashMap::new();
    
    // Basic fusions
    recipes.insert(vec!["fire".to_string(), "water".to_string()], "steam".to_string());
    recipes.insert(vec!["fire".to_string(), "earth".to_string()], "lava".to_string());
    recipes.insert(vec!["water".to_string(), "air".to_string()], "ice".to_string());
    recipes.insert(vec!["earth".to_string(), "air".to_string()], "sand".to_string());
    recipes.insert(vec!["air".to_string(), "lightning".to_string()], "storm".to_string());
    
    // Advanced fusions
    recipes.insert(vec!["nature".to_string(), "death".to_string()], "poison".to_string());
    recipes.insert(vec!["water".to_string(), "poison".to_string()], "acid".to_string());
    recipes.insert(vec!["water".to_string(), "ice".to_string()], "frost".to_string());
    recipes.insert(vec!["fire".to_string(), "earth".to_string(), "lava".to_string()], "magma".to_string());
    
    // Cultivation fusions
    recipes.insert(vec!["qi".to_string(), "spiritual".to_string()], "dao".to_string());
    recipes.insert(vec!["dao".to_string(), "profound".to_string()], "karma".to_string());
    recipes.insert(vec!["karma".to_string(), "fate".to_string()], "immortal".to_string());
    
    recipes
}
```

## 🎯 **Element Configuration Examples**

### **1. Complete Element Registry**

```yaml
# complete_element_registry.yaml
version: 1
elements:
  # Ngũ hành
  - id: "kim"
    name: "Kim"
    category: "nguhang"
    description: "Kim loại, sắc bén, cứng rắn"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "crit_rate"
      - "penetration"
    is_active: true
    
  - id: "moc"
    name: "Mộc"
    category: "nguhang"
    description: "Gỗ, sinh trưởng, mềm mại"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "healing_rate"
      - "growth_rate"
    is_active: true
    
  # Light & Dark
  - id: "light"
    name: "Light"
    category: "lightdark"
    description: "Ánh sáng, thánh thiện, chữa lành"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "healing_rate"
      - "purification_rate"
    is_active: true
    
  - id: "dark"
    name: "Dark"
    category: "lightdark"
    description: "Bóng tối, tà ác, phá hủy"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "corruption_rate"
      - "decay_rate"
    is_active: true
    
  # Cultivation
  - id: "qi"
    name: "Khí"
    category: "cultivation"
    description: "Năng lượng tu luyện cơ bản"
    derived_stats:
      - "power_point"
      - "defense_point"
      - "cultivation_rate"
      - "spiritual_energy"
    is_active: true
```

### **2. Element Interaction Configuration**

```yaml
# element_interactions.yaml
version: 1
interactions:
  # Ngũ hành tương sinh
  - attacker: "kim"
    defender: "thuy"
    multiplier: 1.2
    type: "generating"
    description: "Kim sinh Thủy"
    
  # Ngũ hành tương khắc
  - attacker: "kim"
    defender: "moc"
    multiplier: 1.5
    type: "overcoming"
    description: "Kim khắc Mộc"
    
  # Light vs Dark
  - attacker: "light"
    defender: "dark"
    multiplier: 1.8
    type: "special"
    description: "Light vs Dark"
    
  - attacker: "dark"
    defender: "light"
    multiplier: 0.3
    type: "special"
    description: "Dark vs Light"
    
  # Life vs Death
  - attacker: "life"
    defender: "death"
    multiplier: 1.4
    type: "special"
    description: "Life vs Death"
    
  - attacker: "death"
    defender: "life"
    multiplier: 0.6
    type: "special"
    description: "Death vs Life"
```

## 🎯 **Next Steps**

### **Phase 1: Core Element Types**
1. **Ngũ Hành System**: Implement tương sinh tương khắc
2. **Light/Dark System**: Implement ánh sáng vs bóng tối
3. **Life/Death System**: Implement sinh vs tử
4. **Basic Interactions**: Implement basic interaction matrix

### **Phase 2: Advanced Elements**
1. **Cultivation Elements**: Implement tu luyện elements
2. **Game Reference Elements**: Implement Grim Dawn, Diablo elements
3. **Element Fusion**: Implement element combination system
4. **Advanced Interactions**: Implement complex interaction rules

### **Phase 3: Integration & Testing**
1. **System Integration**: Integrate with existing systems
2. **Performance Testing**: Test with large number of elements
3. **Balance Testing**: Test element balance and interactions
4. **Documentation**: Complete documentation and examples

---

*Tài liệu này sẽ được cập nhật khi có thêm yêu cầu và feedback từ team.*
