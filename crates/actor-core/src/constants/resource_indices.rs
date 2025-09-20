//! Resource index constants for hybrid approach
//! 
//! This module defines compile-time constants for resource indices,
//! enabling direct array access for core resources while maintaining
//! flexibility for custom resources.

/// Core resource indices - fixed at compile time
/// 
/// These indices are used for direct array access to core resources
/// for optimal performance. Core resources are the most commonly
/// accessed resources in the game.
pub mod core_resources {
    /// Health resource index
    pub const HEALTH: usize = 0;
    
    /// Mana resource index
    pub const MANA: usize = 1;
    
    /// Stamina resource index
    pub const STAMINA: usize = 2;
    
    /// Qi resource index (for martial arts systems)
    pub const QI: usize = 3;
    
    /// Experience resource index
    pub const EXPERIENCE: usize = 4;
    
    /// Level resource index
    pub const LEVEL: usize = 5;
    
    /// Vitality resource index
    pub const VITALITY: usize = 6;
    
    /// Spirit resource index
    pub const SPIRIT: usize = 7;
    
    /// Chi resource index
    pub const CHI: usize = 8;
    
    /// Total number of core resources
    pub const COUNT: usize = 9;
}

/// Custom resource indices - runtime allocated
/// 
/// These indices are used for custom resources that can be added
/// at runtime. Custom resources start after core resources.
pub mod custom_resources {
    /// Starting index for custom resources
    pub const START: usize = core_resources::COUNT;
    
    /// Maximum number of custom resources
    pub const MAX_COUNT: usize = 100;
    
    /// Total capacity for all resources
    pub const TOTAL_CAPACITY: usize = START + MAX_COUNT;
}

/// Resource index validation utilities
pub mod validation {
    use super::*;
    
    /// Check if an index is valid
    pub const fn is_valid_index(index: usize) -> bool {
        index < custom_resources::TOTAL_CAPACITY
    }
    
    /// Check if an index is a core resource
    pub const fn is_core_resource(index: usize) -> bool {
        index < core_resources::COUNT
    }
    
    /// Check if an index is a custom resource
    pub const fn is_custom_resource(index: usize) -> bool {
        index >= custom_resources::START && index < custom_resources::TOTAL_CAPACITY
    }
    
    /// Get the resource name for a core resource index
    pub const fn get_core_resource_name(index: usize) -> Option<&'static str> {
        match index {
            core_resources::HEALTH => Some("health"),
            core_resources::MANA => Some("mana"),
            core_resources::STAMINA => Some("stamina"),
            core_resources::QI => Some("qi"),
            core_resources::EXPERIENCE => Some("experience"),
            core_resources::LEVEL => Some("level"),
            core_resources::VITALITY => Some("vitality"),
            core_resources::SPIRIT => Some("spirit"),
            core_resources::CHI => Some("chi"),
            _ => None,
        }
    }
}

/// Re-export commonly used constants
pub use core_resources::*;
pub use custom_resources::*;
pub use validation::*;
