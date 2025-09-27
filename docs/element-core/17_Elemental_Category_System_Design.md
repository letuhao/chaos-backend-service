# Elemental Category System Design

## 📋 **Tổng Quan**

Elemental Category System là một hệ thống phân loại các elements thành các categories dựa trên đặc tính và tương tác của chúng. Hệ thống này giúp quản lý các effects và bonuses một cách có tổ chức, đặc biệt quan trọng cho các skills như Crystal Defense Technique.

## 🎯 **Vấn Đề Cần Giải Quyết**

### **1. Element Classification**
- **Physical Category**: Các elements thuộc về vật lý (Physical, Earth, Metal)
- **Elemental Category**: Các elements thuộc về nguyên tố (Fire, Water, Wood, Air, Lightning)
- **Spiritual Category**: Các elements thuộc về tinh thần (Light, Dark, Life, Death)
- **Dimensional Category**: Các elements thuộc về không gian-thời gian (Time, Space, Void)

### **2. Category-based Effects**
- **Defense Bonuses**: Tăng defense cho các elements trong cùng category
- **Resistance Bonuses**: Tăng resistance cho các elements trong cùng category
- **Synergy Effects**: Hiệu ứng synergy giữa các elements trong cùng category
- **Mastery Bonuses**: Bonus mastery cho các elements trong cùng category

### **3. Skill Integration**
- **Category Targeting**: Skills có thể target specific categories
- **Category Scaling**: Skills scale với category mastery
- **Category Interactions**: Tương tác giữa categories

## 🏗️ **Architecture**

### **Core Components**

```rust
pub struct ElementalCategorySystem {
    category_registry: ElementalCategoryRegistry,
    category_calculator: CategoryCalculator,
    category_interactions: CategoryInteractions,
    category_effects: CategoryEffects,
}

pub struct ElementalCategoryRegistry {
    categories: HashMap<String, ElementalCategory>,
    element_to_category: HashMap<String, String>,
    category_hierarchy: CategoryHierarchy,
}

pub struct ElementalCategory {
    pub category_id: String,
    pub category_name: String,
    pub category_name_vi: String,
    pub description: String,
    pub description_vi: String,
    pub elements: Vec<String>,
    pub category_type: CategoryType,
    pub base_properties: CategoryProperties,
    pub derived_stats: CategoryDerivedStats,
    pub interactions: Vec<CategoryInteraction>,
}

pub struct ElementCategoryMapping {
    pub element_id: String,
    pub primary_category: String,
    pub secondary_categories: Vec<String>,
    pub category_weights: HashMap<String, f64>, // Weight of element in each category
    pub category_affinities: HashMap<String, f64>, // Affinity bonus for each category
}

pub enum CategoryType {
    Physical,    // Vật lý
    Elemental,   // Nguyên tố
    Spiritual,   // Tinh thần
    Dimensional, // Không gian-thời gian
    Hybrid,      // Lai tạp
}
```

## 🔧 **Elemental Categories Definition**

### **1. Physical Category**

```yaml
physical_category:
  category_id: "physical"
  category_name: "Physical"
  category_name_vi: "Vật Lý"
  description: "Elements related to physical matter and material properties"
  description_vi: "Các elements thuộc về vật chất và tính chất vật lý"
  elements:
    - "physical"
    - "earth"
    - "metal"
  category_type: "Physical"
  base_properties:
    defense_multiplier: 1.2
    resistance_bonus: 0.1
    mastery_synergy: 0.05
  derived_stats:
    physical_defense: 1.0
    physical_resistance: 1.0
    physical_mastery_bonus: 1.0
    physical_synergy: 1.0
  interactions:
    - target_category: "elemental"
      interaction_type: "neutral"
      bonus_multiplier: 1.0
    - target_category: "spiritual"
      interaction_type: "weakness"
      bonus_multiplier: 0.8
```

### **2. Elemental Category**

```yaml
elemental_category:
  category_id: "elemental"
  category_name: "Elemental"
  category_name_vi: "Nguyên Tố"
  description: "Elements related to natural forces and elemental energies"
  description_vi: "Các elements thuộc về lực lượng tự nhiên và năng lượng nguyên tố"
  elements:
    - "fire"
    - "water"
    - "wood"
    - "air"
    - "lightning"
  category_type: "Elemental"
  base_properties:
    defense_multiplier: 1.0
    resistance_bonus: 0.15
    mastery_synergy: 0.08
  derived_stats:
    elemental_defense: 1.0
    elemental_resistance: 1.0
    elemental_mastery_bonus: 1.0
    elemental_synergy: 1.0
  interactions:
    - target_category: "physical"
      interaction_type: "neutral"
      bonus_multiplier: 1.0
    - target_category: "spiritual"
      interaction_type: "enhancement"
      bonus_multiplier: 1.2
```

### **3. Spiritual Category**

```yaml
spiritual_category:
  category_id: "spiritual"
  category_name: "Spiritual"
  category_name_vi: "Tinh Thần"
  description: "Elements related to spiritual energy and metaphysical properties"
  description_vi: "Các elements thuộc về năng lượng tinh thần và tính chất siêu hình"
  elements:
    - "light"
    - "dark"
    - "life"
    - "death"
  category_type: "Spiritual"
  base_properties:
    defense_multiplier: 0.8
    resistance_bonus: 0.2
    mastery_synergy: 0.1
  derived_stats:
    spiritual_defense: 1.0
    spiritual_resistance: 1.0
    spiritual_mastery_bonus: 1.0
    spiritual_synergy: 1.0
  interactions:
    - target_category: "physical"
      interaction_type: "enhancement"
      bonus_multiplier: 1.2
    - target_category: "elemental"
      interaction_type: "enhancement"
      bonus_multiplier: 1.2
```

### **4. Dimensional Category**

```yaml
dimensional_category:
  category_id: "dimensional"
  category_name: "Dimensional"
  category_name_vi: "Không Gian-Thời Gian"
  description: "Elements related to space, time, and dimensional properties"
  description_vi: "Các elements thuộc về không gian, thời gian và tính chất chiều không gian"
  elements:
    - "time"
    - "space"
    - "void"
  category_type: "Dimensional"
  base_properties:
    defense_multiplier: 0.6
    resistance_bonus: 0.3
    mastery_synergy: 0.15
  derived_stats:
    dimensional_defense: 1.0
    dimensional_resistance: 1.0
    dimensional_mastery_bonus: 1.0
    dimensional_synergy: 1.0
  interactions:
    - target_category: "physical"
      interaction_type: "transcendence"
      bonus_multiplier: 1.5
    - target_category: "elemental"
      interaction_type: "transcendence"
      bonus_multiplier: 1.5
    - target_category: "spiritual"
      interaction_type: "transcendence"
      bonus_multiplier: 1.5
```

## 🔧 **Category Calculator**

```rust
pub struct CategoryCalculator {
    category_registry: ElementalCategoryRegistry,
    element_core_client: ElementCoreClient,
}

impl CategoryCalculator {
    /// Calculate category-based defense bonus
    pub async fn calculate_category_defense_bonus(
        &self,
        actor: &Actor,
        target_categories: &[String],
        base_defense: f64,
        defense_multiplier: f64
    ) -> Result<f64, CategoryError> {
        let mut total_bonus = 0.0;
        
        for category_id in target_categories {
            let category = self.category_registry.get_category(category_id)?;
            let category_mastery = self.calculate_category_mastery(actor, category).await?;
            
            let category_bonus = base_defense * category.derived_stats.physical_defense 
                * (1.0 + category_mastery * category.base_properties.mastery_synergy);
            
            total_bonus += category_bonus;
        }
        
        Ok(total_bonus * defense_multiplier)
    }
    
    /// Calculate category mastery for an actor
    pub async fn calculate_category_mastery(
        &self,
        actor: &Actor,
        category: &ElementalCategory
    ) -> Result<f64, CategoryError> {
        let mut total_mastery = 0.0;
        let mut element_count = 0;
        
        for element_id in &category.elements {
            let element_mastery = actor.get_element_mastery(element_id);
            total_mastery += element_mastery;
            element_count += 1;
        }
        
        if element_count > 0 {
            Ok(total_mastery / element_count as f64)
        } else {
            Ok(0.0)
        }
    }
    
    /// Calculate category resistance bonus
    pub async fn calculate_category_resistance_bonus(
        &self,
        actor: &Actor,
        target_categories: &[String],
        incoming_element: &str
    ) -> Result<f64, CategoryError> {
        let mut total_resistance = 0.0;
        
        for category_id in target_categories {
            let category = self.category_registry.get_category(category_id)?;
            let category_mastery = self.calculate_category_mastery(actor, category).await?;
            
            let resistance_bonus = category.base_properties.resistance_bonus 
                * (1.0 + category_mastery * 0.001);
            
            total_resistance += resistance_bonus;
        }
        
        Ok(total_resistance)
    }
    
    /// Calculate category synergy bonus
    pub async fn calculate_category_synergy_bonus(
        &self,
        actor: &Actor,
        primary_category: &str,
        secondary_categories: &[String]
    ) -> Result<f64, CategoryError> {
        let primary = self.category_registry.get_category(primary_category)?;
        let primary_mastery = self.calculate_category_mastery(actor, primary).await?;
        
        let mut synergy_bonus = 0.0;
        
        for category_id in secondary_categories {
            let category = self.category_registry.get_category(category_id)?;
            let category_mastery = self.calculate_category_mastery(actor, category).await?;
            
            let interaction = primary.get_interaction_with(category_id)?;
            let synergy = primary_mastery * category_mastery * interaction.bonus_multiplier 
                * primary.base_properties.mastery_synergy;
            
            synergy_bonus += synergy;
        }
        
        Ok(synergy_bonus)
    }
}
```

## 🔧 **Category Effects**

```rust
pub struct CategoryEffects {
    category_registry: ElementalCategoryRegistry,
    effect_calculator: CategoryEffectCalculator,
}

impl CategoryEffects {
    /// Apply category-based effects to an actor
    pub async fn apply_category_effects(
        &self,
        actor: &mut Actor,
        target_categories: &[String],
        effect_type: CategoryEffectType,
        magnitude: f64
    ) -> Result<Vec<CategoryEffectResult>, CategoryError> {
        let mut results = Vec::new();
        
        for category_id in target_categories {
            let category = self.category_registry.get_category(category_id)?;
            let category_mastery = self.calculate_category_mastery(actor, category).await?;
            
            let effect_result = self.apply_category_effect(
                actor,
                category,
                effect_type,
                magnitude,
                category_mastery
            ).await?;
            
            results.push(effect_result);
        }
        
        Ok(results)
    }
    
    async fn apply_category_effect(
        &self,
        actor: &mut Actor,
        category: &ElementalCategory,
        effect_type: CategoryEffectType,
        magnitude: f64,
        category_mastery: f64
    ) -> Result<CategoryEffectResult, CategoryError> {
        match effect_type {
            CategoryEffectType::DefenseBonus => {
                let bonus = magnitude * category.derived_stats.physical_defense 
                    * (1.0 + category_mastery * 0.001);
                actor.add_defense_bonus(bonus);
                Ok(CategoryEffectResult::DefenseBonus(bonus))
            },
            CategoryEffectType::ResistanceBonus => {
                let bonus = magnitude * category.derived_stats.physical_resistance 
                    * (1.0 + category_mastery * 0.001);
                actor.add_resistance_bonus(bonus);
                Ok(CategoryEffectResult::ResistanceBonus(bonus))
            },
            CategoryEffectType::MasteryBonus => {
                let bonus = magnitude * category.derived_stats.physical_mastery_bonus 
                    * (1.0 + category_mastery * 0.001);
                actor.add_mastery_bonus(category.category_id.clone(), bonus);
                Ok(CategoryEffectResult::MasteryBonus(bonus))
            },
        }
    }
}

pub enum CategoryEffectType {
    DefenseBonus,
    ResistanceBonus,
    MasteryBonus,
    SynergyBonus,
}

pub enum CategoryEffectResult {
    DefenseBonus(f64),
    ResistanceBonus(f64),
    MasteryBonus(f64),
    SynergyBonus(f64),
}
```

## 🔧 **Category Interactions**

```rust
pub struct CategoryInteractions {
    interactions: HashMap<String, CategoryInteraction>,
}

pub struct CategoryInteraction {
    pub source_category: String,
    pub target_category: String,
    pub interaction_type: CategoryInteractionType,
    pub bonus_multiplier: f64,
    pub conditions: Vec<CategoryCondition>,
}

pub enum CategoryInteractionType {
    Neutral,      // Trung tính
    Enhancement,  // Tăng cường
    Weakness,     // Yếu đuối
    Transcendence, // Siêu việt
    Suppression,  // Áp chế
}

pub struct CategoryCondition {
    pub condition_type: CategoryConditionType,
    pub condition_value: f64,
    pub condition_operator: CategoryConditionOperator,
}

pub enum CategoryConditionType {
    MasteryLevel,
    CategoryLevel,
    ElementCount,
    Custom(String),
}

pub enum CategoryConditionOperator {
    GreaterThan,
    LessThan,
    EqualTo,
    GreaterThanOrEqual,
    LessThanOrEqual,
}
```

## 📊 **Configuration Files**

### **1. Elemental Category Configuration**

```yaml
# chaos-backend-service/docs/element-core/configs/elemental_categories.yaml
version: 1.0

categories:
  physical:
    category_id: "physical"
    category_name: "Physical"
    category_name_vi: "Vật Lý"
    description: "Elements related to physical matter and material properties"
    description_vi: "Các elements thuộc về vật chất và tính chất vật lý"
    elements:
      - "physical"
      - "earth"
      - "metal"
    category_type: "Physical"
    base_properties:
      defense_multiplier: 1.2
      resistance_bonus: 0.1
      mastery_synergy: 0.05
    derived_stats:
      physical_defense: 1.0
      physical_resistance: 1.0
      physical_mastery_bonus: 1.0
      physical_synergy: 1.0
    interactions:
      - target_category: "elemental"
        interaction_type: "neutral"
        bonus_multiplier: 1.0
      - target_category: "spiritual"
        interaction_type: "weakness"
        bonus_multiplier: 0.8

  elemental:
    category_id: "elemental"
    category_name: "Elemental"
    category_name_vi: "Nguyên Tố"
    description: "Elements related to natural forces and elemental energies"
    description_vi: "Các elements thuộc về lực lượng tự nhiên và năng lượng nguyên tố"
    elements:
      - "fire"
      - "water"
      - "wood"
      - "air"
      - "lightning"
    category_type: "Elemental"
    base_properties:
      defense_multiplier: 1.0
      resistance_bonus: 0.15
      mastery_synergy: 0.08
    derived_stats:
      elemental_defense: 1.0
      elemental_resistance: 1.0
      elemental_mastery_bonus: 1.0
      elemental_synergy: 1.0
    interactions:
      - target_category: "physical"
        interaction_type: "neutral"
        bonus_multiplier: 1.0
      - target_category: "spiritual"
        interaction_type: "enhancement"
        bonus_multiplier: 1.2

  spiritual:
    category_id: "spiritual"
    category_name: "Spiritual"
    category_name_vi: "Tinh Thần"
    description: "Elements related to spiritual energy and metaphysical properties"
    description_vi: "Các elements thuộc về năng lượng tinh thần và tính chất siêu hình"
    elements:
      - "light"
      - "dark"
      - "life"
      - "death"
    category_type: "Spiritual"
    base_properties:
      defense_multiplier: 0.8
      resistance_bonus: 0.2
      mastery_synergy: 0.1
    derived_stats:
      spiritual_defense: 1.0
      spiritual_resistance: 1.0
      spiritual_mastery_bonus: 1.0
      spiritual_synergy: 1.0
    interactions:
      - target_category: "physical"
        interaction_type: "enhancement"
        bonus_multiplier: 1.2
      - target_category: "elemental"
        interaction_type: "enhancement"
        bonus_multiplier: 1.2

  dimensional:
    category_id: "dimensional"
    category_name: "Dimensional"
    category_name_vi: "Không Gian-Thời Gian"
    description: "Elements related to space, time, and dimensional properties"
    description_vi: "Các elements thuộc về không gian, thời gian và tính chất chiều không gian"
    elements:
      - "time"
      - "space"
      - "void"
    category_type: "Dimensional"
    base_properties:
      defense_multiplier: 0.6
      resistance_bonus: 0.3
      mastery_synergy: 0.15
    derived_stats:
      dimensional_defense: 1.0
      dimensional_resistance: 1.0
      dimensional_mastery_bonus: 1.0
      dimensional_synergy: 1.0
    interactions:
      - target_category: "physical"
        interaction_type: "transcendence"
        bonus_multiplier: 1.5
      - target_category: "elemental"
        interaction_type: "transcendence"
        bonus_multiplier: 1.5
      - target_category: "spiritual"
        interaction_type: "transcendence"
        bonus_multiplier: 1.5

# Category hierarchy
hierarchy:
  - level: 1
    categories: ["physical", "elemental"]
  - level: 2
    categories: ["spiritual"]
  - level: 3
    categories: ["dimensional"]

# Element to category mappings (Multi-category support)
element_category_mappings:
  # Fire element - belongs to both Elemental and Physical categories
  fire:
    element_id: "fire"
    primary_category: "elemental"
    secondary_categories: ["physical"]
    category_weights:
      elemental: 0.8
      physical: 0.2
    category_affinities:
      elemental: 1.0
      physical: 0.3
  
  # Earth element - belongs to Physical, Elemental, and Spiritual categories
  earth:
    element_id: "earth"
    primary_category: "physical"
    secondary_categories: ["elemental", "spiritual"]
    category_weights:
      physical: 0.6
      elemental: 0.3
      spiritual: 0.1
    category_affinities:
      physical: 1.0
      elemental: 0.7
      spiritual: 0.4
  
  # Lightning element - belongs to Elemental and Dimensional categories
  lightning:
    element_id: "lightning"
    primary_category: "elemental"
    secondary_categories: ["dimensional"]
    category_weights:
      elemental: 0.7
      dimensional: 0.3
    category_affinities:
      elemental: 1.0
      dimensional: 0.6
  
  # Light element - belongs to Spiritual and Elemental categories
  light:
    element_id: "light"
    primary_category: "spiritual"
    secondary_categories: ["elemental"]
    category_weights:
      spiritual: 0.8
      elemental: 0.2
    category_affinities:
      spiritual: 1.0
      elemental: 0.5

# Global category settings
global_settings:
  max_categories_per_actor: 4
  category_switching_cooldown: 30.0
  category_mastery_decay_rate: 0.01
  category_synergy_threshold: 1000.0
  multi_category_support: true
  max_categories_per_element: 3
```

## 🔧 **Integration with Crystal Defense Technique**

### **Updated Crystal Defense Technique**

```yaml
# Updated crystal_defense_technique.yaml
elemental_properties:
  # Primary elements
  primary_elements: ["earth", "metal"]
  secondary_elements: []
  
  # Elemental categories (NEW)
  target_categories:
    - "physical"    # Physical category
    - "elemental"   # Elemental category
  
  # Category-based defense bonuses
  category_defense_bonuses:
    physical: 20.0  # 20x multiplier for physical category
    elemental: 20.0 # 20x multiplier for elemental category
  
  # Category-based resistance
  category_resistance:
    physical: 0.8   # 80% physical category resistance
    elemental: 0.8  # 80% elemental category resistance
```

### **Updated Defense Bonus Calculation**

```rust
// Updated defense bonus calculation with categories
fn calculate_defense_bonus(
    &self,
    actor_defense_point: f64,
    actor_physical_defense: f64,
    actor_elemental_defense: f64,
    earth_mastery: f64,
    metal_mastery: f64,
    derived_stats: &ElementDerivedStats,
    category_calculator: &CategoryCalculator  // NEW
) -> f64 {
    // Base defense calculation
    let base_defense = actor_defense_point + actor_physical_defense + actor_elemental_defense;
    
    // Elemental mastery bonus
    let elemental_bonus = (earth_mastery + metal_mastery) * 0.0001;
    
    // Category-based bonus (NEW)
    let category_bonus = category_calculator.calculate_category_defense_bonus(
        actor,
        &["physical", "elemental"],  // Target categories
        base_defense,
        20.0  // 20x multiplier
    ).await?;
    
    // Apply all bonuses
    let final_defense = (base_defense * self.defense_properties.defense_multiplier 
        + self.defense_properties.defense_bonus) 
        * (1.0 + elemental_bonus) 
        + category_bonus;  // NEW
    
    // Apply derived stats bonuses
    let derived_bonus = derived_stats.defense_point * 0.1;
    
    final_defense + derived_bonus
}
```

## 🧪 **Testing Strategy**

### **1. Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_category_defense_bonus_calculation() {
        let category_system = ElementalCategorySystem::new();
        let actor = create_test_actor();
        
        let defense_bonus = category_system.calculate_category_defense_bonus(
            &actor,
            &["physical", "elemental"],
            1000.0,  // base defense
            20.0     // multiplier
        ).await?;
        
        assert!(defense_bonus > 20000.0); // Should be 20x base defense
    }
    
    #[test]
    fn test_category_mastery_calculation() {
        let category_system = ElementalCategorySystem::new();
        let actor = create_test_actor();
        
        let physical_mastery = category_system.calculate_category_mastery(
            &actor,
            &category_system.get_category("physical")?
        ).await?;
        
        assert!(physical_mastery > 0.0);
    }
}
```

### **2. Integration Tests**

```rust
#[tokio::test]
async fn test_crystal_defense_with_categories() {
    let crystal_defense = CrystalDefenseTechnique::new();
    let category_system = ElementalCategorySystem::new();
    let actor = create_test_actor();
    
    let defense_bonus = crystal_defense.calculate_defense_bonus(
        actor.get_defense_point(),
        actor.get_physical_defense(),
        actor.get_elemental_defense(),
        actor.get_element_mastery("earth"),
        actor.get_element_mastery("metal"),
        &derived_stats,
        &category_system  // NEW
    ).await?;
    
    // Should include category bonuses
    assert!(defense_bonus > 100000.0);
}
```

## 📝 **Implementation Notes**

### **1. Performance Considerations**
- **Category Caching**: Cache category calculations for performance
- **Lazy Evaluation**: Calculate category bonuses only when needed
- **Batch Processing**: Process multiple categories in batches

### **2. Balance Considerations**
- **Category Balance**: Ensure categories are balanced against each other
- **Mastery Scaling**: Balance category mastery scaling
- **Synergy Limits**: Set limits on category synergy bonuses

### **3. Extensibility**
- **New Categories**: Easy to add new categories
- **Custom Interactions**: Support for custom category interactions
- **Dynamic Categories**: Support for dynamic category assignment

## 📚 **Related Documents**

- [00_Element_Core_Overview.md](./00_Element_Core_Overview.md) - Element Core overview
- [11_Advanced_Derived_Stats_Design.md](./11_Advanced_Derived_Stats_Design.md) - Derived stats system
- [16_Hybrid_Subsystem_Design.md](./16_Hybrid_Subsystem_Design.md) - Hybrid elements system
- [18_Universal_Element_Registry_Design.md](./18_Universal_Element_Registry_Design.md) - Universal element registry
- [19_Stats_Distribution_Design.md](./19_Stats_Distribution_Design.md) - External system integration

---

**Last Updated**: 2025-01-27  
**Version**: 1.0  
**Status**: Design Phase  
**Maintainer**: Chaos World Team
