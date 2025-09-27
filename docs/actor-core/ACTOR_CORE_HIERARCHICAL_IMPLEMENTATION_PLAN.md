# Actor Core Hierarchical Implementation Plan

## 📋 **Tổng Quan**

Document này mô tả kế hoạch implement **actor-core-hierarchical** crate với mục tiêu:
- **Data Hub**: Chỉ lưu trữ và quản lý data, không xử lý logic của các hệ thống khác
- **Elemental Core Focus**: Chỉ implement cho elemental-core properties
- **Extensible Design**: Dễ dàng thêm hệ thống mới mà không phá vỡ cấu trúc
- **Performance Optimization**: Sử dụng array-based approach

## 🎯 **Nguyên Tắc Thiết Kế**

### **1. Data Hub Principle**
- **Actor-core-hierarchical** là **data hub**, không phải nơi xử lý logic
- **Chỉ lưu trữ data**: resources, primary stats, derived stats, experience points
- **Không xử lý logic**: cultivation logic, magic logic, combat logic, etc.
- **Pure data management**: get/set/aggregate data

### **2. Elemental Core Focus**
- **Chỉ implement elemental-core properties** trong phase này
- **Không đụng tới**: cultivation system, magic system, race system, etc.
- **Clean separation**: Elemental data tách biệt hoàn toàn với các hệ thống khác
- **Future-ready**: Cấu trúc sẵn sàng cho các hệ thống mới

### **3. Extensible Architecture**
- **Modular design**: Mỗi system có module riêng
- **No breaking changes**: Thêm system mới không ảnh hưởng existing systems
- **Plugin architecture**: Dễ dàng add/remove systems
- **Version compatibility**: Backward compatible

## 🏗️ **Crate Structure**

```
chaos-backend-service/
├── crates/
│   ├── actor-core/                    # Base crate (unchanged)
│   │   ├── src/
│   │   │   ├── types.rs              # Base Actor, Subsystem, etc.
│   │   │   ├── aggregator/           # Base aggregation logic
│   │   │   └── ...
│   │   └── Cargo.toml
│   │
│   ├── actor-core-hierarchical/       # New hierarchical crate
│   │   ├── src/
│   │   │   ├── lib.rs                # Main library entry
│   │   │   ├── types/
│   │   │   │   ├── mod.rs            # Module declarations
│   │   │   │   ├── elemental_data.rs # Elemental data structures
│   │   │   │   ├── actor_data.rs     # Actor data structures
│   │   │   │   └── system_data.rs    # System data structures
│   │   │   ├── systems/
│   │   │   │   ├── mod.rs            # System module declarations
│   │   │   │   └── elemental.rs      # Elemental system data only
│   │   │   ├── adapters/
│   │   │   │   ├── mod.rs            # Adapter module declarations
│   │   │   │   ├── actor_adapter.rs  # Actor data adapter
│   │   │   │   └── elemental_adapter.rs # Elemental data adapter
│   │   │   ├── aggregation/
│   │   │   │   ├── mod.rs            # Aggregation module declarations
│   │   │   │   ├── elemental_aggregator.rs # Elemental data aggregation
│   │   │   │   └── base_aggregator.rs # Base aggregation logic
│   │   │   └── utils/
│   │   │       ├── mod.rs            # Utility module declarations
│   │   │       ├── element_indices.rs # Element index constants
│   │   │       └── performance.rs    # Performance utilities
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   └── element-core/                  # Element core (unchanged)
│       ├── src/
│       └── Cargo.toml
```

## 📊 **Data Structures**

### **1. Elemental Data Structures (Phase 1 Only)**

```rust
// actor-core-hierarchical/src/types/elemental_data.rs
use serde::{Deserialize, Serialize};

/// Elemental system data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementalSystemData {
    // Element mastery data (array-based)
    pub element_mastery_levels: [f64; MAX_ELEMENTS],                    // Mastery levels per element
    pub element_mastery_experience: [f64; MAX_ELEMENTS],                // Mastery experience per element
    pub element_mastery_ranks: [ElementMasteryRank; MAX_ELEMENTS],      // Mastery ranks per element
    
    // Element primary stats (array-based)
    pub element_qi_amounts: [f64; MAX_ELEMENTS],                        // Qi amounts per element
    pub element_qi_capacities: [f64; MAX_ELEMENTS],                     // Qi capacities per element
    pub element_qi_regeneration_rates: [f64; MAX_ELEMENTS],             // Qi regeneration rates per element
    pub element_qi_efficiencies: [f64; MAX_ELEMENTS],                   // Qi efficiencies per element
    pub element_qi_purities: [f64; MAX_ELEMENTS],                       // Qi purities per element
    pub element_qi_affinities: [f64; MAX_ELEMENTS],                     // Qi affinities per element
    pub element_qi_control_levels: [f64; MAX_ELEMENTS],                 // Qi control levels per element
    pub element_qi_manipulation_speeds: [f64; MAX_ELEMENTS],            // Qi manipulation speeds per element
    pub element_qi_stability_levels: [f64; MAX_ELEMENTS],               // Qi stability levels per element
    pub element_qi_resonance_frequencies: [f64; MAX_ELEMENTS],          // Qi resonance frequencies per element
    
    // Element derived stats (array-based)
    pub element_power_points: [f64; MAX_ELEMENTS],                      // Power points per element
    pub element_defense_points: [f64; MAX_ELEMENTS],                    // Defense points per element
    pub element_crit_rates: [f64; MAX_ELEMENTS],                        // Critical rates per element
    pub element_resist_crit_rates: [f64; MAX_ELEMENTS],                 // Resist critical rates per element
    pub element_crit_damages: [f64; MAX_ELEMENTS],                      // Critical damages per element
    pub element_resist_crit_damages: [f64; MAX_ELEMENTS],               // Resist critical damages per element
    pub element_accurate_rates: [f64; MAX_ELEMENTS],                    // Accurate rates per element
    pub element_dodge_rates: [f64; MAX_ELEMENTS],                       // Dodge rates per element
    pub element_status_probabilities: [f64; MAX_ELEMENTS],              // Status probabilities per element
    pub element_status_resistances: [f64; MAX_ELEMENTS],                // Status resistances per element
    pub element_status_durations: [f64; MAX_ELEMENTS],                  // Status durations per element
    pub element_status_duration_reductions: [f64; MAX_ELEMENTS],        // Status duration reductions per element
    pub element_status_intensities: [f64; MAX_ELEMENTS],                // Status intensities per element
    pub element_status_intensity_reductions: [f64; MAX_ELEMENTS],       // Status intensity reductions per element
    pub element_penetrations: [f64; MAX_ELEMENTS],                      // Element penetrations per element
    pub element_absorptions: [f64; MAX_ELEMENTS],                       // Element absorptions per element
    pub element_amplifications: [f64; MAX_ELEMENTS],                    // Element amplifications per element
    pub element_reductions: [f64; MAX_ELEMENTS],                        // Element reductions per element
    pub element_reflection_rates: [f64; MAX_ELEMENTS],                  // Reflection rates per element
    pub element_resist_reflection_rates: [f64; MAX_ELEMENTS],           // Resist reflection rates per element
    pub element_reflection_damages: [f64; MAX_ELEMENTS],                // Reflection damages per element
    pub element_resist_reflection_damages: [f64; MAX_ELEMENTS],         // Resist reflection damages per element
    
    // Parry System (array-based)
    pub element_parry_rates: [f64; MAX_ELEMENTS],                       // Parry rates per element
    pub element_parry_breaks: [f64; MAX_ELEMENTS],                      // Parry breaks per element
    pub element_parry_strengths: [f64; MAX_ELEMENTS],                   // Parry strengths per element
    pub element_parry_shreds: [f64; MAX_ELEMENTS],                      // Parry shreds per element
    
    // Block System (array-based)
    pub element_block_rates: [f64; MAX_ELEMENTS],                       // Block rates per element
    pub element_block_breaks: [f64; MAX_ELEMENTS],                      // Block breaks per element
    pub element_block_strengths: [f64; MAX_ELEMENTS],                   // Block strengths per element
    pub element_block_shreds: [f64; MAX_ELEMENTS],                      // Block shreds per element
    
    // Skill Execution & Performance (array-based)
    pub element_skill_execution_speeds: [f64; MAX_ELEMENTS],            // Skill execution speeds per element
    pub element_skill_cooldown_reductions: [f64; MAX_ELEMENTS],         // Skill cooldown reductions per element
    pub element_attack_skill_effectivenesses: [f64; MAX_ELEMENTS],      // Attack skill effectivenesses per element
    pub element_defense_skill_effectivenesses: [f64; MAX_ELEMENTS],     // Defense skill effectivenesses per element
    pub element_status_skill_effectivenesses: [f64; MAX_ELEMENTS],      // Status skill effectivenesses per element
    pub element_movement_technique_effectivenesses: [f64; MAX_ELEMENTS], // Movement technique effectivenesses per element
    pub element_healing_skill_effectivenesses: [f64; MAX_ELEMENTS],     // Healing skill effectivenesses per element
    pub element_support_skill_effectivenesses: [f64; MAX_ELEMENTS],     // Support skill effectivenesses per element
    pub element_utility_skill_effectivenesses: [f64; MAX_ELEMENTS],     // Utility skill effectivenesses per element
    pub element_skill_effectivenesses: [f64; MAX_ELEMENTS],             // General skill effectivenesses per element
    
    // Resource Management (array-based)
    pub element_resource_regenerations: [f64; MAX_ELEMENTS],            // Resource regenerations per element
    pub element_resource_efficiencies: [f64; MAX_ELEMENTS],             // Resource efficiencies per element
    
    // Social & Economy (array-based)
    pub element_leadership_bonuses: [f64; MAX_ELEMENTS],                // Element leadership bonuses per element
    pub element_teaching_efficiencies: [f64; MAX_ELEMENTS],             // Element teaching efficiencies per element
    pub element_crafting_efficiencies: [f64; MAX_ELEMENTS],             // Element crafting efficiencies per element
    pub element_resource_discoveries: [f64; MAX_ELEMENTS],              // Element resource discoveries per element
    
    // Perception & Detection (array-based)
    pub element_sensitivities: [f64; MAX_ELEMENTS],                     // Element sensitivities per element
    
    // Advanced Combat Mechanics (array-based)
    pub element_mastery_synergy_bonuses: [f64; MAX_ELEMENTS],           // Mastery synergy bonuses per element
    
    // Element Interactions (2D array)
    pub element_interactions: [[f64; MAX_ELEMENTS]; MAX_ELEMENTS],      // Element interaction bonuses
    
    // Feature Flags (2D array)
    pub element_feature_flags: [[bool; MAX_FEATURE_FLAGS]; MAX_ELEMENTS], // Feature flags per element
    
    // Element Resources (array-based)
    pub element_resource_amounts: [f64; MAX_ELEMENTS],                  // Resource amounts per element
    pub element_resource_capacities: [f64; MAX_ELEMENTS],               // Resource capacities per element
    pub element_resource_regeneration_rates: [f64; MAX_ELEMENTS],       // Resource regeneration rates per element
    pub element_resource_efficiencies: [f64; MAX_ELEMENTS],             // Resource efficiencies per element
}

// Compile-time constants
pub const MAX_ELEMENTS: usize = 50;                                     // 50 elements
pub const MAX_FEATURE_FLAGS: usize = 16;                                // Maximum feature flags per element

/// Element Mastery Rank enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementMasteryRank {
    Novice,         // 0-100 experience
    Apprentice,     // 100-500 experience
    Adept,          // 500-1000 experience
    Expert,         // 1000-2500 experience
    Master,         // 2500-5000 experience
    Grandmaster,    // 5000-10000 experience
    Sage,           // 10000-25000 experience
    Transcendent,   // 25000+ experience
}

impl ElementMasteryRank {
    /// Get rank from experience
    pub fn from_experience(experience: f64) -> Self {
        match experience as u32 {
            0..=99 => ElementMasteryRank::Novice,
            100..=499 => ElementMasteryRank::Apprentice,
            500..=999 => ElementMasteryRank::Adept,
            1000..=2499 => ElementMasteryRank::Expert,
            2500..=4999 => ElementMasteryRank::Master,
            5000..=9999 => ElementMasteryRank::Grandmaster,
            10000..=24999 => ElementMasteryRank::Sage,
            _ => ElementMasteryRank::Transcendent,
        }
    }
    
    /// Get experience required for next rank
    pub fn experience_for_next_rank(&self) -> f64 {
        match self {
            ElementMasteryRank::Novice => 100.0,
            ElementMasteryRank::Apprentice => 500.0,
            ElementMasteryRank::Adept => 1000.0,
            ElementMasteryRank::Expert => 2500.0,
            ElementMasteryRank::Master => 5000.0,
            ElementMasteryRank::Grandmaster => 10000.0,
            ElementMasteryRank::Sage => 25000.0,
            ElementMasteryRank::Transcendent => f64::INFINITY,
        }
    }
    
    /// Get rank multiplier for stats
    pub fn get_stat_multiplier(&self) -> f64 {
        match self {
            ElementMasteryRank::Novice => 1.0,
            ElementMasteryRank::Apprentice => 1.1,
            ElementMasteryRank::Adept => 1.25,
            ElementMasteryRank::Expert => 1.5,
            ElementMasteryRank::Master => 2.0,
            ElementMasteryRank::Grandmaster => 3.0,
            ElementMasteryRank::Sage => 5.0,
            ElementMasteryRank::Transcendent => 10.0,
        }
    }
}
```

### **2. Actor Data Structures**

```rust
// actor-core-hierarchical/src/types/actor_data.rs
use serde::{Deserialize, Serialize};
use crate::types::elemental_data::ElementalSystemData;

/// Hierarchical actor data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchicalActorData {
    // Core actor information
    pub actor_id: String,
    pub name: String,
    pub current_world: String,
    pub reincarnation_count: u32,
    pub total_lifespan: f64,
    pub age: f64,
    
    // System data (modular design)
    pub elemental_system: ElementalSystemData,          // Elemental system data only
    
    // Future systems (placeholder for extensibility)
    // pub cultivation_system: CultivationSystemData,   // Future: Cultivation system
    // pub magic_system: MagicSystemData,               // Future: Magic system
    // pub race_system: RaceSystemData,                 // Future: Race system
    // pub talent_system: TalentSystemData,             // Future: Talent system
    // pub item_system: ItemSystemData,                 // Future: Item system
    // pub luck_system: LuckSystemData,                 // Future: Luck system
    
    // Global stats cache
    pub global_stats_cache: GlobalStatsCache,
    
    // Performance metrics
    pub performance_metrics: PerformanceMetrics,
}

/// Global stats cache for fast access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStatsCache {
    // Total stats (aggregated from all systems)
    pub total_hp: f64,
    pub total_mp: f64,
    pub total_attack_power: f64,
    pub total_defense: f64,
    pub total_speed: f64,
    pub total_critical_rate: f64,
    pub total_critical_damage: f64,
    pub total_accuracy: f64,
    pub total_dodge_rate: f64,
    
    // System breakdown (contribution from each system)
    pub elemental_contribution: ElementalContribution,
    
    // Future system contributions (placeholder)
    // pub cultivation_contribution: CultivationContribution,
    // pub magic_contribution: MagicContribution,
    // pub race_contribution: RaceContribution,
    
    // Cache metadata
    pub last_updated: std::time::SystemTime,
    pub cache_valid: bool,
}

/// Elemental system contribution to global stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementalContribution {
    pub hp_contribution: f64,
    pub mp_contribution: f64,
    pub attack_power_contribution: f64,
    pub defense_contribution: f64,
    pub speed_contribution: f64,
    pub critical_rate_contribution: f64,
    pub critical_damage_contribution: f64,
    pub accuracy_contribution: f64,
    pub dodge_rate_contribution: f64,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub average_access_time: f64,
    pub last_benchmark: std::time::SystemTime,
}
```

### **3. System Data Structures**

```rust
// actor-core-hierarchical/src/types/system_data.rs
use serde::{Deserialize, Serialize};

/// System data trait for extensibility
pub trait SystemData {
    fn get_system_name(&self) -> &str;
    fn get_data_size(&self) -> usize;
    fn is_valid(&self) -> bool;
    fn reset(&mut self);
}

/// System contribution trait for aggregation
pub trait SystemContribution {
    fn calculate_hp_contribution(&self) -> f64;
    fn calculate_mp_contribution(&self) -> f64;
    fn calculate_attack_power_contribution(&self) -> f64;
    fn calculate_defense_contribution(&self) -> f64;
    fn calculate_speed_contribution(&self) -> f64;
    fn calculate_critical_rate_contribution(&self) -> f64;
    fn calculate_critical_damage_contribution(&self) -> f64;
    fn calculate_accuracy_contribution(&self) -> f64;
    fn calculate_dodge_rate_contribution(&self) -> f64;
}

/// System registry for managing multiple systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRegistry {
    pub elemental_system_active: bool,
    
    // Future systems (placeholder)
    pub cultivation_system_active: bool,
    pub magic_system_active: bool,
    pub race_system_active: bool,
    pub talent_system_active: bool,
    pub item_system_active: bool,
    pub luck_system_active: bool,
}

impl SystemRegistry {
    pub fn new() -> Self {
        Self {
            elemental_system_active: true,
            
            // Future systems
            cultivation_system_active: false,
            magic_system_active: false,
            race_system_active: false,
            talent_system_active: false,
            item_system_active: false,
            luck_system_active: false,
        }
    }
    
    pub fn activate_system(&mut self, system_name: &str) -> bool {
        match system_name {
            "elemental" => {
                self.elemental_system_active = true;
                true
            },
            "cultivation" => {
                self.cultivation_system_active = true;
                true
            },
            "magic" => {
                self.magic_system_active = true;
                true
            },
            "race" => {
                self.race_system_active = true;
                true
            },
            "talent" => {
                self.talent_system_active = true;
                true
            },
            "item" => {
                self.item_system_active = true;
                true
            },
            "luck" => {
                self.luck_system_active = true;
                true
            },
            _ => false,
        }
    }
    
    pub fn deactivate_system(&mut self, system_name: &str) -> bool {
        match system_name {
            "elemental" => {
                self.elemental_system_active = false;
                true
            },
            "cultivation" => {
                self.cultivation_system_active = false;
                true
            },
            "magic" => {
                self.magic_system_active = false;
                true
            },
            "race" => {
                self.race_system_active = false;
                true
            },
            "talent" => {
                self.talent_system_active = false;
                true
            },
            "item" => {
                self.item_system_active = false;
                true
            },
            "luck" => {
                self.luck_system_active = false;
                true
            },
            _ => false,
        }
    }
    
    pub fn is_system_active(&self, system_name: &str) -> bool {
        match system_name {
            "elemental" => self.elemental_system_active,
            "cultivation" => self.cultivation_system_active,
            "magic" => self.magic_system_active,
            "race" => self.race_system_active,
            "talent" => self.talent_system_active,
            "item" => self.item_system_active,
            "luck" => self.luck_system_active,
            _ => false,
        }
    }
}
```

## 🔧 **Implementation Plan**

### **Phase 1: Create Crate Structure (1 week)**
1. **Create actor-core-hierarchical crate**
2. **Set up module structure**
3. **Add dependencies**
4. **Create basic types**

### **Phase 2: Implement Elemental Data (1 week)**
1. **Implement ElementalSystemData struct**
2. **Implement ElementMasteryRank enum**
3. **Add element index constants**
4. **Add basic validation**

### **Phase 3: Implement Actor Data (1 week)**
1. **Implement HierarchicalActorData struct**
2. **Implement GlobalStatsCache**
3. **Implement PerformanceMetrics**
4. **Add system registry**

### **Phase 4: Implement Adapters (1 week)**
1. **Implement ActorAdapter**
2. **Implement ElementalAdapter**
3. **Add data conversion logic**
4. **Add validation**

### **Phase 5: Implement Aggregation (1 week)**
1. **Implement ElementalAggregator**
2. **Implement BaseAggregator**
3. **Add performance optimization**
4. **Add caching**

### **Phase 6: Testing & Integration (1 week)**
1. **Unit tests**
2. **Integration tests**
3. **Performance benchmarks**
4. **Documentation**

## 🎯 **Key Features**

### **1. Data Hub Only**
- **Pure data storage**: No business logic
- **Get/Set operations**: Simple data access
- **Aggregation**: Combine data from systems
- **Caching**: Fast access to aggregated data

### **2. Elemental Core Focus**
- **Only elemental properties**: No other systems
- **Array-based performance**: Fast access
- **Complete coverage**: All elemental stats
- **Future-ready**: Easy to extend

### **3. Extensible Design**
- **Modular architecture**: Each system separate
- **Plugin system**: Easy to add/remove systems
- **No breaking changes**: Backward compatible
- **Version management**: Handle system versions

### **4. Performance Optimization**
- **Array-based access**: 1-2 ns access time
- **Cache optimization**: 95%+ cache hit rate
- **Memory efficiency**: Contiguous memory layout
- **SIMD ready**: Vector operations support

## 🚀 **Usage Example**

```rust
use actor_core_hierarchical::*;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    // Create hierarchical actor data
    let mut actor_data = HierarchicalActorData::new(
        "reincarnated_warrior_001".to_string(),
        "Ancient Warrior".to_string(),
        "cultivation_world".to_string(),
    );
    
    // Set elemental mastery levels (direct array access - 1-2 ns)
    actor_data.elemental_system.element_mastery_levels[0] = 50.0; // Fire
    actor_data.elemental_system.element_mastery_levels[1] = 45.0; // Water
    actor_data.elemental_system.element_mastery_levels[2] = 40.0; // Earth
    
    // Set elemental power points (direct array access - 1-2 ns)
    actor_data.elemental_system.element_power_points[0] = 100.0; // Fire
    actor_data.elemental_system.element_power_points[1] = 90.0;  // Water
    actor_data.elemental_system.element_power_points[2] = 95.0;  // Earth
    
    // Set element interactions (direct 2D array access - 1-2 ns)
    actor_data.elemental_system.element_interactions[0][1] = 0.7; // Fire vs Water
    actor_data.elemental_system.element_interactions[1][0] = 1.3; // Water vs Fire
    
    // Aggregate stats
    let aggregator = ElementalAggregator::new();
    let aggregated_stats = aggregator.aggregate_elemental_contribution(&actor_data.elemental_system).await?;
    
    // Update global cache
    actor_data.global_stats_cache.elemental_contribution = aggregated_stats;
    actor_data.global_stats_cache.total_hp = aggregated_stats.hp_contribution;
    actor_data.global_stats_cache.total_mp = aggregated_stats.mp_contribution;
    actor_data.global_stats_cache.total_attack_power = aggregated_stats.attack_power_contribution;
    
    // Get total stats (cached - 1-2 ns)
    let total_hp = actor_data.global_stats_cache.total_hp;
    let total_mp = actor_data.global_stats_cache.total_mp;
    let total_attack_power = actor_data.global_stats_cache.total_attack_power;
    
    println!("Total HP: {}", total_hp);
    println!("Total MP: {}", total_mp);
    println!("Total Attack Power: {}", total_attack_power);
    
    // Convert to base actor for compatibility
    let base_actor = ActorAdapter::to_base_actor(&actor_data);
    
    Ok(())
}
```

## 🎯 **Conclusion**

**Actor-core-hierarchical** sẽ là **data hub** hoàn hảo với:

1. **Pure data management** - Không xử lý logic của systems khác
2. **Elemental Core focus** - Chỉ implement elemental properties
3. **Extensible design** - Dễ dàng thêm systems mới
4. **High performance** - Array-based access với 1-2 ns
5. **Future-ready** - Sẵn sàng cho các systems mới

Approach này đảm bảo **clean separation** và **extensibility** cho tương lai! 🎉

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Implementation Plan Complete  
**Maintainer**: Chaos World Team
