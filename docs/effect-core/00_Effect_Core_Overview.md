# Effect Core Overview

## 📋 **Tổng Quan**

Effect Core là hệ thống trung tâm quản lý tất cả các effects trong game, được thiết kế dựa trên kinh nghiệm từ Skyrim's Magic Effects system và các game đi trước. Hệ thống này đóng vai trò cầu nối thống nhất giữa Action Core, Status Core, Element Core và các hệ thống tương lai như Talent Core, Perk Core.

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Skyrim-Inspired Architecture**
- **Magic Effects System**: Học hỏi từ Skyrim's sophisticated Magic Effects
- **Condition System**: Complex condition system như Skyrim's Condition Functions
- **Editor ID System**: GUID + Editor ID system tương tự Skyrim
- **Plugin Architecture**: Modular plugin system như Skyrim mods

### **2. Unified Effect Management**
- **Single Source of Truth**: Tất cả effects được quản lý tập trung
- **Consistent Interfaces**: Interface thống nhất cho tất cả effect types
- **Centralized Processing**: Xử lý effects tập trung và hiệu quả
- **Cross-System Integration**: Tích hợp seamless với tất cả systems

### **3. Future-Proof Design**
- **Extensible Architecture**: Dễ dàng thêm effect types mới
- **System Agnostic**: Không phụ thuộc vào specific systems
- **Plugin Support**: Hỗ trợ plugin system như Skyrim
- **Version Control**: Hỗ trợ versioning và migration

## 🏗️ **Kiến Trúc Effect Core**

### **Core Components**

```
Effect Core
├── Effect Registry
│   ├── Effect Type Definitions
│   ├── Effect Categories
│   ├── Effect GUID Management
│   └── Effect Validation
├── Condition System
│   ├── Condition Functions (Skyrim-inspired)
│   ├── Condition Evaluator
│   ├── Condition Cache
│   └── Condition Validator
├── Effect Engine
│   ├── Effect Calculator
│   ├── Effect Processor
│   ├── Effect Scheduler
│   └── Effect Monitor
├── Effect Interfaces
│   ├── Action Effect Interface
│   ├── Status Effect Interface
│   ├── Element Effect Interface
│   ├── Talent Effect Interface
│   └── Perk Effect Interface
└── Integration Bridges
    ├── Action Core Bridge
    ├── Status Core Bridge
    ├── Element Core Bridge
    ├── Talent Core Bridge
    └── Perk Core Bridge
```

## 🎮 **Skyrim Condition System Analysis**

### **1. Skyrim Condition Functions**

Skyrim có hơn 100 condition functions phức tạp:

```
Skyrim Condition Categories
├── Actor Conditions
│   ├── GetActorValue (Health, Magicka, Stamina)
│   ├── GetLevel
│   ├── GetRace
│   ├── GetSex
│   └── IsInCombat
├── Item Conditions
│   ├── HasItem
│   ├── GetItemCount
│   ├── IsEquipped
│   └── GetItemCharge
├── Location Conditions
│   ├── GetInCurrentLocation
│   ├── GetInCurrentLocType
│   ├── IsInInterior
│   └── IsInWater
├── Time Conditions
│   ├── GetCurrentTime
│   ├── GetDayOfWeek
│   └── GetSeason
├── Weather Conditions
│   ├── GetCurrentWeather
│   ├── IsRaining
│   └── IsSnowing
├── Magic Conditions
│   ├── HasMagicEffect
│   ├── GetMagicEffectMagnitude
│   └── HasSpell
├── Relationship Conditions
│   ├── GetRelationshipRank
│   ├── IsHostileToActor
│   └── IsFriendlyToActor
└── Custom Conditions
    ├── GetGlobalValue
    ├── GetQuestCompleted
    └── GetQuestStage
```

### **2. Skyrim Condition Logic**

```javascript
// Skyrim Condition Example
if (GetActorValue Health < 0.5) AND 
   (IsInCombat == 1) AND 
   (HasMagicEffect FireResist == 0) AND
   (GetCurrentWeather == 0) // Clear weather
then
   ApplyEffect Burning
```

### **3. Skyrim Magic Effect Structure**

```yaml
# Skyrim Magic Effect Structure
magic_effect:
  form_id: "0x00012345"
  editor_id: "MGEF_FireDamage"
  name: "Fire Damage"
  description: "Deals fire damage over time"
  
  # Effect Properties
  effect_type: "Value Modifier"
  effect_archetype: "Damage Health"
  effect_delivery: "Target Actor"
  effect_duration: "Duration"
  effect_magnitude: "Magnitude"
  
  # Conditions
  conditions:
    - condition_function: "GetActorValue"
      condition_parameter: "Health"
      condition_operator: "Less Than"
      condition_value: 0.5
    - condition_function: "IsInCombat"
      condition_operator: "Equal"
      condition_value: 1
    - condition_function: "HasMagicEffect"
      condition_parameter: "FireResist"
      condition_operator: "Equal"
      condition_value: 0
  
  # Effect Data
  effect_data:
    base_damage: 10.0
    damage_type: "Fire"
    damage_per_second: true
    visual_effect: "FireParticles"
    audio_effect: "FireSound"
```

## 🔧 **Chaos Effect Core Design**

### **1. Effect Structure (Skyrim-inspired)**

```yaml
# Effect Definition Structure
effect_definition:
  # Basic Info (Skyrim-inspired)
  effect_guid: "550e8400-e29b-41d4-a716-446655440000"
  effect_id: "EFF_FireDamage"  # Editor ID like Skyrim
  effect_name: "Fire Damage"
  effect_name_vi: "Sát Thương Hỏa"
  world_id: "chaos_world"
  
  # Effect Properties
  effect_type: "damage"
  effect_archetype: "elemental_damage"
  effect_delivery: "target_actor"
  effect_duration: "duration"
  effect_magnitude: "magnitude"
  
  # Effect Categories
  categories:
    - "elemental"
    - "damage"
    - "fire"
    - "combat"
  
  # Complex Conditions (Skyrim-inspired)
  conditions:
    - condition_id: "health_condition"
      condition_function: "get_actor_value"
      condition_parameter: "health"
      condition_operator: "less_than"
      condition_value: 0.5
      condition_logic: "AND"
    
    - condition_id: "combat_condition"
      condition_function: "is_in_combat"
      condition_operator: "equal"
      condition_value: true
      condition_logic: "AND"
    
    - condition_id: "fire_resist_condition"
      condition_function: "has_effect"
      condition_parameter: "fire_resistance"
      condition_operator: "equal"
      condition_value: false
      condition_logic: "AND"
    
    - condition_id: "weather_condition"
      condition_function: "get_current_weather"
      condition_operator: "equal"
      condition_value: "clear"
      condition_logic: "AND"
  
  # Effect Data
  effect_data:
    base_damage: 10.0
    damage_type: "fire"
    damage_per_second: true
    scaling_factor: 0.01
    scaling_stat: "fire_mastery"
    
  # Visual/Audio Effects
  visual_effects:
    - effect_id: "fire_particles"
      effect_type: "particle_effect"
      particle_type: "fire"
      particle_color: "#ff4500"
      particle_intensity: 0.8
      
  audio_effects:
    - effect_id: "fire_sound"
      effect_type: "loop_sound"
      sound_type: "fire"
      volume: 0.7
      loop: true
```

### **2. Condition System Design**

```yaml
# Condition System Structure
condition_system:
  # Condition Functions (Skyrim-inspired)
  condition_functions:
    # Actor Conditions
    get_actor_value:
      function_id: "get_actor_value"
      function_name: "Get Actor Value"
      function_name_vi: "Lấy Giá Trị Diễn Viên"
      parameters: ["stat_name", "operator", "value"]
      return_type: "boolean"
      
    get_level:
      function_id: "get_level"
      function_name: "Get Level"
      function_name_vi: "Lấy Cấp Độ"
      parameters: ["operator", "value"]
      return_type: "boolean"
      
    is_in_combat:
      function_id: "is_in_combat"
      function_name: "Is In Combat"
      function_name_vi: "Đang Trong Chiến Đấu"
      parameters: ["operator", "value"]
      return_type: "boolean"
    
    # Item Conditions
    has_item:
      function_id: "has_item"
      function_name: "Has Item"
      function_name_vi: "Có Vật Phẩm"
      parameters: ["item_id", "operator", "count"]
      return_type: "boolean"
      
    is_equipped:
      function_id: "is_equipped"
      function_name: "Is Equipped"
      function_name_vi: "Đang Trang Bị"
      parameters: ["item_id", "operator", "value"]
      return_type: "boolean"
    
    # Location Conditions
    get_in_current_location:
      function_id: "get_in_current_location"
      function_name: "Get In Current Location"
      function_name_vi: "Trong Vị Trí Hiện Tại"
      parameters: ["location_id", "operator", "value"]
      return_type: "boolean"
      
    is_in_interior:
      function_id: "is_in_interior"
      function_name: "Is In Interior"
      function_name_vi: "Trong Nội Thất"
      parameters: ["operator", "value"]
      return_type: "boolean"
    
    # Time Conditions
    get_current_time:
      function_id: "get_current_time"
      function_name: "Get Current Time"
      function_name_vi: "Lấy Thời Gian Hiện Tại"
      parameters: ["operator", "value"]
      return_type: "boolean"
      
    get_day_of_week:
      function_id: "get_day_of_week"
      function_name: "Get Day Of Week"
      function_name_vi: "Lấy Ngày Trong Tuần"
      parameters: ["operator", "value"]
      return_type: "boolean"
    
    # Weather Conditions
    get_current_weather:
      function_id: "get_current_weather"
      function_name: "Get Current Weather"
      function_name_vi: "Lấy Thời Tiết Hiện Tại"
      parameters: ["operator", "value"]
      return_type: "boolean"
      
    is_raining:
      function_id: "is_raining"
      function_name: "Is Raining"
      function_name_vi: "Đang Mưa"
      parameters: ["operator", "value"]
      return_type: "boolean"
    
    # Magic Conditions
    has_effect:
      function_id: "has_effect"
      function_name: "Has Effect"
      function_name_vi: "Có Hiệu Ứng"
      parameters: ["effect_id", "operator", "value"]
      return_type: "boolean"
      
    get_effect_magnitude:
      function_id: "get_effect_magnitude"
      function_name: "Get Effect Magnitude"
      function_name_vi: "Lấy Cường Độ Hiệu Ứng"
      parameters: ["effect_id", "operator", "value"]
      return_type: "boolean"
    
    # Relationship Conditions
    get_relationship_rank:
      function_id: "get_relationship_rank"
      function_name: "Get Relationship Rank"
      function_name_vi: "Lấy Cấp Độ Quan Hệ"
      parameters: ["actor_id", "operator", "value"]
      return_type: "boolean"
      
    is_hostile_to_actor:
      function_id: "is_hostile_to_actor"
      function_name: "Is Hostile To Actor"
      function_name_vi: "Thù Địch Với Diễn Viên"
      parameters: ["actor_id", "operator", "value"]
      return_type: "boolean"
    
    # Custom Conditions
    get_global_value:
      function_id: "get_global_value"
      function_name: "Get Global Value"
      function_name_vi: "Lấy Giá Trị Toàn Cục"
      parameters: ["global_id", "operator", "value"]
      return_type: "boolean"
      
    get_quest_completed:
      function_id: "get_quest_completed"
      function_name: "Get Quest Completed"
      function_name_vi: "Lấy Nhiệm Vụ Hoàn Thành"
      parameters: ["quest_id", "operator", "value"]
      return_type: "boolean"
```

### **3. Effect Categories System**

```yaml
# Effect Categories (Skyrim-inspired)
effect_categories:
  # Primary Categories
  elemental:
    category_id: "elemental"
    category_name: "Elemental"
    category_name_vi: "Nguyên Tố"
    parent_category: null
    child_categories: ["fire", "water", "earth", "wood", "metal"]
    description: "Elemental effects"
    
  combat:
    category_id: "combat"
    category_name: "Combat"
    category_name_vi: "Chiến Đấu"
    parent_category: null
    child_categories: ["damage", "healing", "buff", "debuff"]
    description: "Combat-related effects"
    
  movement:
    category_id: "movement"
    category_name: "Movement"
    category_name_vi: "Di Chuyển"
    parent_category: null
    child_categories: ["speed", "jump", "flight", "teleport"]
    description: "Movement-related effects"
    
  social:
    category_id: "social"
    category_name: "Social"
    category_name_vi: "Xã Hội"
    parent_category: null
    child_categories: ["charisma", "persuasion", "intimidation"]
    description: "Social interaction effects"
    
  environmental:
    category_id: "environmental"
    category_name: "Environmental"
    category_name_vi: "Môi Trường"
    parent_category: null
    child_categories: ["weather", "terrain", "time", "location"]
    description: "Environmental effects"
  
  # Secondary Categories
  fire:
    category_id: "fire"
    category_name: "Fire"
    category_name_vi: "Hỏa"
    parent_category: "elemental"
    child_categories: ["burning", "ignite", "combustion"]
    description: "Fire elemental effects"
    
  damage:
    category_id: "damage"
    category_name: "Damage"
    category_name_vi: "Sát Thương"
    parent_category: "combat"
    child_categories: ["physical_damage", "magical_damage", "true_damage"]
    description: "Damage effects"
    
  healing:
    category_id: "healing"
    category_name: "Healing"
    category_name_vi: "Hồi Máu"
    parent_category: "combat"
    child_categories: ["health_healing", "stamina_healing", "mana_healing"]
    description: "Healing effects"
```

## 🚀 **Implementation Strategy**

### **Phase 1: Foundation (2 weeks)**
1. **Create Effect Core Structure**
   - Effect Registry
   - Effect GUID Management
   - Effect Categories
   - Basic Effect Types

2. **Implement Condition System**
   - Condition Functions (Skyrim-inspired)
   - Condition Evaluator
   - Condition Cache
   - Condition Validator

### **Phase 2: Core Engine (2 weeks)**
1. **Effect Engine**
   - Effect Calculator
   - Effect Processor
   - Effect Scheduler
   - Effect Monitor

2. **Effect Interfaces**
   - Action Effect Interface
   - Status Effect Interface
   - Element Effect Interface

### **Phase 3: Integration (2 weeks)**
1. **Integration Bridges**
   - Action Core Bridge
   - Status Core Bridge
   - Element Core Bridge

2. **System Integration**
   - Update existing systems
   - Migrate existing effects
   - Test integration

### **Phase 4: Advanced Features (2 weeks)**
1. **Advanced Condition System**
   - Complex condition logic
   - Condition combinations
   - Performance optimization

2. **Plugin System**
   - Plugin architecture
   - Hot reload support
   - Mod support

## 📁 **Configuration File Structure**

```
chaos-backend-service/docs/effect-core/
├── 00_Effect_Core_Overview.md
├── 01_Effect_Core_Architecture_Design.md
├── 02_Effect_Core_Condition_System_Design.md
├── 03_Effect_Core_Integration_Design.md
├── 04_Effect_Core_Configuration_System_Design.md
├── 05_Effect_Core_API_Design.md
├── 06_Effect_Core_Performance_Design.md
├── 07_Effect_Core_Error_Handling_Design.md
├── 08_Effect_Core_Testing_Strategy.md
├── 09_Effect_Core_Implementation_Guide.md
├── 10_Effect_Core_Plugin_System_Design.md
├── configs/
│   ├── core/
│   │   ├── effect_types.yaml
│   │   ├── effect_categories.yaml
│   │   ├── condition_functions.yaml
│   │   └── effect_validation_rules.yaml
│   ├── effects/
│   │   ├── damage_effects/
│   │   ├── healing_effects/
│   │   ├── buff_effects/
│   │   ├── debuff_effects/
│   │   ├── status_effects/
│   │   ├── movement_effects/
│   │   └── environmental_effects/
│   ├── conditions/
│   │   ├── actor_conditions.yaml
│   │   ├── item_conditions.yaml
│   │   ├── location_conditions.yaml
│   │   ├── time_conditions.yaml
│   │   ├── weather_conditions.yaml
│   │   ├── magic_conditions.yaml
│   │   ├── relationship_conditions.yaml
│   │   └── custom_conditions.yaml
│   ├── interfaces/
│   │   ├── action_effect_interface.yaml
│   │   ├── status_effect_interface.yaml
│   │   ├── element_effect_interface.yaml
│   │   ├── talent_effect_interface.yaml
│   │   └── perk_effect_interface.yaml
│   ├── integrations/
│   │   ├── action_core_bridge.yaml
│   │   ├── status_core_bridge.yaml
│   │   ├── element_core_bridge.yaml
│   │   ├── talent_core_bridge.yaml
│   │   └── perk_core_bridge.yaml
│   └── plugins/
│       ├── elemental_magic_plugin.yaml
│       ├── combat_system_plugin.yaml
│       ├── movement_system_plugin.yaml
│       └── custom_effects_plugin.yaml
└── README.md
```

## 🎯 **Key Features**

### **1. Skyrim-Inspired Design**
- **Complex Condition System**: Hơn 100 condition functions như Skyrim
- **Editor ID System**: GUID + Editor ID system
- **Plugin Architecture**: Modular plugin system
- **Magic Effect Structure**: Tương tự Skyrim's Magic Effects

### **2. Advanced Condition Logic**
- **Condition Functions**: Actor, Item, Location, Time, Weather, Magic, Relationship
- **Condition Combinations**: AND, OR, NOT logic
- **Condition Caching**: Performance optimization
- **Condition Validation**: Comprehensive validation

### **3. Unified Effect Management**
- **Single Source of Truth**: Tất cả effects ở một nơi
- **Consistent Interfaces**: Interface thống nhất
- **Centralized Processing**: Xử lý tập trung
- **Cross-System Integration**: Tích hợp với tất cả systems

### **4. Future-Proof Architecture**
- **Extensible Design**: Dễ dàng thêm effect types
- **Plugin Support**: Hỗ trợ plugin system
- **Version Control**: Hỗ trợ versioning
- **Migration Support**: Hỗ trợ migration

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Complete  
**Maintainer**: Chaos World Team
