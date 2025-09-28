//! # Element Category
//! 
//! This module defines the element category system for classifying elements.

use serde::{Deserialize, Serialize};

/// Element categories for classification and organization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementCategory {
    /// Physical Elements
    Physical(PhysicalElement),
    
    /// Elemental Elements
    Elemental(ElementalElement),
    
    /// Spiritual Elements
    Spiritual(SpiritualElement),
    
    /// Dimensional Elements
    Dimensional(DimensionalElement),
    
    /// Hybrid Elements
    Hybrid(HybridElement),
    
    /// Special Elements
    Special(SpecialElement),
}

/// Physical element types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhysicalElement {
    /// Five Elements (Wu Xing)
    FiveElements,
    
    /// Metal element
    Metal,
    
    /// Wood element
    Wood,
    
    /// Water element
    Water,
    
    /// Fire element
    Fire,
    
    /// Earth element
    Earth,
    
    /// Wind element
    Wind,
    
    /// Ice element
    Ice,
    
    /// Lightning element
    Lightning,
    
    /// Nature element
    Nature,
    
    /// Stone element
    Stone,
}

/// Elemental element types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementalElement {
    /// Light element
    Light,
    
    /// Dark element
    Dark,
    
    /// Shadow element
    Shadow,
    
    /// Holy element
    Holy,
    
    /// Chaos element
    Chaos,
    
    /// Order element
    Order,
    
    /// Void element
    Void,
    
    /// Aether element
    Aether,
}

/// Spiritual element types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpiritualElement {
    /// Soul element
    Soul,
    
    /// Mind element
    Mind,
    
    /// Spirit element
    Spirit,
    
    /// Will element
    Will,
    
    /// Emotion element
    Emotion,
    
    /// Memory element
    Memory,
    
    /// Dream element
    Dream,
    
    /// Nightmare element
    Nightmare,
}

/// Dimensional element types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DimensionalElement {
    /// Time element
    Time,
    
    /// Space element
    Space,
    
    /// Gravity element
    Gravity,
    
    /// Force element
    Force,
    
    /// Dimension element
    Dimension,
    
    /// Portal element
    Portal,
    
    /// Teleportation element
    Teleportation,
}

/// Hybrid element types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HybridElement {
    /// Fire + Ice
    FireIce,
    
    /// Lightning + Divine
    LightningDivine,
    
    /// Earth + Metal
    EarthMetal,
    
    /// Water + Wind
    WaterWind,
    
    /// Light + Dark
    LightDark,
    
    /// Chaos + Order
    ChaosOrder,
    
    /// Time + Space
    TimeSpace,
    
    /// Soul + Mind
    SoulMind,
}

/// Special element types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecialElement {
    /// Omni element (affects all elements)
    Omni,
    
    /// Neutral element
    Neutral,
    
    /// Pure element
    Pure,
    
    /// Corrupted element
    Corrupted,
    
    /// Blessed element
    Blessed,
    
    /// Cursed element
    Cursed,
    
    /// Ancient element
    Ancient,
    
    /// Primordial element
    Primordial,
}

impl ElementCategory {
    /// Get the category name as a string
    pub fn name(&self) -> &'static str {
        match self {
            ElementCategory::Physical(physical) => match physical {
                PhysicalElement::FiveElements => "five_elements",
                PhysicalElement::Metal => "metal",
                PhysicalElement::Wood => "wood",
                PhysicalElement::Water => "water",
                PhysicalElement::Fire => "fire",
                PhysicalElement::Earth => "earth",
                PhysicalElement::Wind => "wind",
                PhysicalElement::Ice => "ice",
                PhysicalElement::Lightning => "lightning",
                PhysicalElement::Nature => "nature",
                PhysicalElement::Stone => "stone",
            },
            ElementCategory::Elemental(elemental) => match elemental {
                ElementalElement::Light => "light",
                ElementalElement::Dark => "dark",
                ElementalElement::Shadow => "shadow",
                ElementalElement::Holy => "holy",
                ElementalElement::Chaos => "chaos",
                ElementalElement::Order => "order",
                ElementalElement::Void => "void",
                ElementalElement::Aether => "aether",
            },
            ElementCategory::Spiritual(spiritual) => match spiritual {
                SpiritualElement::Soul => "soul",
                SpiritualElement::Mind => "mind",
                SpiritualElement::Spirit => "spirit",
                SpiritualElement::Will => "will",
                SpiritualElement::Emotion => "emotion",
                SpiritualElement::Memory => "memory",
                SpiritualElement::Dream => "dream",
                SpiritualElement::Nightmare => "nightmare",
            },
            ElementCategory::Dimensional(dimensional) => match dimensional {
                DimensionalElement::Time => "time",
                DimensionalElement::Space => "space",
                DimensionalElement::Gravity => "gravity",
                DimensionalElement::Force => "force",
                DimensionalElement::Dimension => "dimension",
                DimensionalElement::Portal => "portal",
                DimensionalElement::Teleportation => "teleportation",
            },
            ElementCategory::Hybrid(hybrid) => match hybrid {
                HybridElement::FireIce => "fire_ice",
                HybridElement::LightningDivine => "lightning_divine",
                HybridElement::EarthMetal => "earth_metal",
                HybridElement::WaterWind => "water_wind",
                HybridElement::LightDark => "light_dark",
                HybridElement::ChaosOrder => "chaos_order",
                HybridElement::TimeSpace => "time_space",
                HybridElement::SoulMind => "soul_mind",
            },
            ElementCategory::Special(special) => match special {
                SpecialElement::Omni => "omni",
                SpecialElement::Neutral => "neutral",
                SpecialElement::Pure => "pure",
                SpecialElement::Corrupted => "corrupted",
                SpecialElement::Blessed => "blessed",
                SpecialElement::Cursed => "cursed",
                SpecialElement::Ancient => "ancient",
                SpecialElement::Primordial => "primordial",
            },
        }
    }
    
    /// Get the category description
    pub fn description(&self) -> &'static str {
        match self {
            ElementCategory::Physical(_) => "Physical elements based on natural phenomena",
            ElementCategory::Elemental(_) => "Elemental forces of light, dark, and energy",
            ElementCategory::Spiritual(_) => "Spiritual and mental elements",
            ElementCategory::Dimensional(_) => "Dimensional and space-time elements",
            ElementCategory::Hybrid(_) => "Hybrid combinations of multiple elements",
            ElementCategory::Special(_) => "Special and unique elements",
        }
    }
    
    /// Check if this category is a physical element
    pub fn is_physical(&self) -> bool {
        matches!(self, ElementCategory::Physical(_))
    }
    
    /// Check if this category is an elemental element
    pub fn is_elemental(&self) -> bool {
        matches!(self, ElementCategory::Elemental(_))
    }
    
    /// Check if this category is a spiritual element
    pub fn is_spiritual(&self) -> bool {
        matches!(self, ElementCategory::Spiritual(_))
    }
    
    /// Check if this category is a dimensional element
    pub fn is_dimensional(&self) -> bool {
        matches!(self, ElementCategory::Dimensional(_))
    }
    
    /// Check if this category is a hybrid element
    pub fn is_hybrid(&self) -> bool {
        matches!(self, ElementCategory::Hybrid(_))
    }
    
    /// Check if this category is a special element
    pub fn is_special(&self) -> bool {
        matches!(self, ElementCategory::Special(_))
    }
    
    /// Check if this category is part of the five elements
    pub fn is_five_elements(&self) -> bool {
        matches!(self, ElementCategory::Physical(PhysicalElement::FiveElements))
    }
    
    /// Check if this category is the omni element
    pub fn is_omni(&self) -> bool {
        matches!(self, ElementCategory::Special(SpecialElement::Omni))
    }
    
    /// Get the parent category type
    pub fn parent_type(&self) -> &'static str {
        match self {
            ElementCategory::Physical(_) => "physical",
            ElementCategory::Elemental(_) => "elemental",
            ElementCategory::Spiritual(_) => "spiritual",
            ElementCategory::Dimensional(_) => "dimensional",
            ElementCategory::Hybrid(_) => "hybrid",
            ElementCategory::Special(_) => "special",
        }
    }
}

impl std::fmt::Display for ElementCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::str::FromStr for ElementCategory {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Physical elements
            "five_elements" => Ok(ElementCategory::Physical(PhysicalElement::FiveElements)),
            "metal" => Ok(ElementCategory::Physical(PhysicalElement::Metal)),
            "wood" => Ok(ElementCategory::Physical(PhysicalElement::Wood)),
            "water" => Ok(ElementCategory::Physical(PhysicalElement::Water)),
            "fire" => Ok(ElementCategory::Physical(PhysicalElement::Fire)),
            "earth" => Ok(ElementCategory::Physical(PhysicalElement::Earth)),
            "wind" => Ok(ElementCategory::Physical(PhysicalElement::Wind)),
            "ice" => Ok(ElementCategory::Physical(PhysicalElement::Ice)),
            "lightning" => Ok(ElementCategory::Physical(PhysicalElement::Lightning)),
            "nature" => Ok(ElementCategory::Physical(PhysicalElement::Nature)),
            "stone" => Ok(ElementCategory::Physical(PhysicalElement::Stone)),
            
            // Elemental elements
            "light" => Ok(ElementCategory::Elemental(ElementalElement::Light)),
            "dark" => Ok(ElementCategory::Elemental(ElementalElement::Dark)),
            "shadow" => Ok(ElementCategory::Elemental(ElementalElement::Shadow)),
            "holy" => Ok(ElementCategory::Elemental(ElementalElement::Holy)),
            "chaos" => Ok(ElementCategory::Elemental(ElementalElement::Chaos)),
            "order" => Ok(ElementCategory::Elemental(ElementalElement::Order)),
            "void" => Ok(ElementCategory::Elemental(ElementalElement::Void)),
            "aether" => Ok(ElementCategory::Elemental(ElementalElement::Aether)),
            
            // Spiritual elements
            "soul" => Ok(ElementCategory::Spiritual(SpiritualElement::Soul)),
            "mind" => Ok(ElementCategory::Spiritual(SpiritualElement::Mind)),
            "spirit" => Ok(ElementCategory::Spiritual(SpiritualElement::Spirit)),
            "will" => Ok(ElementCategory::Spiritual(SpiritualElement::Will)),
            "emotion" => Ok(ElementCategory::Spiritual(SpiritualElement::Emotion)),
            "memory" => Ok(ElementCategory::Spiritual(SpiritualElement::Memory)),
            "dream" => Ok(ElementCategory::Spiritual(SpiritualElement::Dream)),
            "nightmare" => Ok(ElementCategory::Spiritual(SpiritualElement::Nightmare)),
            
            // Dimensional elements
            "time" => Ok(ElementCategory::Dimensional(DimensionalElement::Time)),
            "space" => Ok(ElementCategory::Dimensional(DimensionalElement::Space)),
            "gravity" => Ok(ElementCategory::Dimensional(DimensionalElement::Gravity)),
            "force" => Ok(ElementCategory::Dimensional(DimensionalElement::Force)),
            "dimension" => Ok(ElementCategory::Dimensional(DimensionalElement::Dimension)),
            "portal" => Ok(ElementCategory::Dimensional(DimensionalElement::Portal)),
            "teleportation" => Ok(ElementCategory::Dimensional(DimensionalElement::Teleportation)),
            
            // Hybrid elements
            "fire_ice" => Ok(ElementCategory::Hybrid(HybridElement::FireIce)),
            "lightning_divine" => Ok(ElementCategory::Hybrid(HybridElement::LightningDivine)),
            "earth_metal" => Ok(ElementCategory::Hybrid(HybridElement::EarthMetal)),
            "water_wind" => Ok(ElementCategory::Hybrid(HybridElement::WaterWind)),
            "light_dark" => Ok(ElementCategory::Hybrid(HybridElement::LightDark)),
            "chaos_order" => Ok(ElementCategory::Hybrid(HybridElement::ChaosOrder)),
            "time_space" => Ok(ElementCategory::Hybrid(HybridElement::TimeSpace)),
            "soul_mind" => Ok(ElementCategory::Hybrid(HybridElement::SoulMind)),
            
            // Special elements
            "omni" => Ok(ElementCategory::Special(SpecialElement::Omni)),
            "neutral" => Ok(ElementCategory::Special(SpecialElement::Neutral)),
            "pure" => Ok(ElementCategory::Special(SpecialElement::Pure)),
            "corrupted" => Ok(ElementCategory::Special(SpecialElement::Corrupted)),
            "blessed" => Ok(ElementCategory::Special(SpecialElement::Blessed)),
            "cursed" => Ok(ElementCategory::Special(SpecialElement::Cursed)),
            "ancient" => Ok(ElementCategory::Special(SpecialElement::Ancient)),
            "primordial" => Ok(ElementCategory::Special(SpecialElement::Primordial)),
            
            _ => Err(format!("Unknown element category: {}", s)),
        }
    }
}
