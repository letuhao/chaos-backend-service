//! # Elemental Data Structures
//! 
//! This module contains the array-based elemental data structures for high-performance access.

// use serde::{Deserialize, Serialize}; // Removed for now

/// Maximum number of elements supported
pub const MAX_ELEMENTS: usize = 50;

/// Maximum number of feature flags per element
pub const MAX_FEATURE_FLAGS: usize = 16;

/// Elemental system data structure with array-based storage for high performance
#[derive(Debug, Clone)]
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
}

impl Default for ElementalSystemData {
    fn default() -> Self {
        Self::new()
    }
}

impl ElementalSystemData {
    /// Create a new elemental system data instance with default values
    pub fn new() -> Self {
        Self {
            // Initialize all arrays with default values
            element_mastery_levels: [0.0; MAX_ELEMENTS],
            element_mastery_experience: [0.0; MAX_ELEMENTS],
            element_mastery_ranks: [ElementMasteryRank::Novice; MAX_ELEMENTS],
            
            element_qi_amounts: [0.0; MAX_ELEMENTS],
            element_qi_capacities: [100.0; MAX_ELEMENTS],
            element_qi_regeneration_rates: [1.0; MAX_ELEMENTS],
            element_qi_efficiencies: [1.0; MAX_ELEMENTS],
            element_qi_purities: [1.0; MAX_ELEMENTS],
            element_qi_affinities: [1.0; MAX_ELEMENTS],
            element_qi_control_levels: [1.0; MAX_ELEMENTS],
            element_qi_manipulation_speeds: [1.0; MAX_ELEMENTS],
            element_qi_stability_levels: [1.0; MAX_ELEMENTS],
            element_qi_resonance_frequencies: [1.0; MAX_ELEMENTS],
            
            element_power_points: [0.0; MAX_ELEMENTS],
            element_defense_points: [0.0; MAX_ELEMENTS],
            element_crit_rates: [0.05; MAX_ELEMENTS],
            element_resist_crit_rates: [0.05; MAX_ELEMENTS],
            element_crit_damages: [1.5; MAX_ELEMENTS],
            element_resist_crit_damages: [1.5; MAX_ELEMENTS],
            element_accurate_rates: [0.8; MAX_ELEMENTS],
            element_dodge_rates: [0.05; MAX_ELEMENTS],
            element_status_probabilities: [0.1; MAX_ELEMENTS],
            element_status_resistances: [0.1; MAX_ELEMENTS],
            element_status_durations: [5.0; MAX_ELEMENTS],
            element_status_duration_reductions: [0.0; MAX_ELEMENTS],
            element_status_intensities: [1.0; MAX_ELEMENTS],
            element_status_intensity_reductions: [0.0; MAX_ELEMENTS],
            element_penetrations: [0.0; MAX_ELEMENTS],
            element_absorptions: [0.0; MAX_ELEMENTS],
            element_amplifications: [1.0; MAX_ELEMENTS],
            element_reductions: [1.0; MAX_ELEMENTS],
            element_reflection_rates: [0.0; MAX_ELEMENTS],
            element_resist_reflection_rates: [0.0; MAX_ELEMENTS],
            element_reflection_damages: [0.0; MAX_ELEMENTS],
            element_resist_reflection_damages: [0.0; MAX_ELEMENTS],
            
            element_parry_rates: [0.0; MAX_ELEMENTS],
            element_parry_breaks: [0.0; MAX_ELEMENTS],
            element_parry_strengths: [0.0; MAX_ELEMENTS],
            element_parry_shreds: [0.0; MAX_ELEMENTS],
            
            element_block_rates: [0.0; MAX_ELEMENTS],
            element_block_breaks: [0.0; MAX_ELEMENTS],
            element_block_strengths: [0.0; MAX_ELEMENTS],
            element_block_shreds: [0.0; MAX_ELEMENTS],
            
            element_skill_execution_speeds: [1.0; MAX_ELEMENTS],
            element_skill_cooldown_reductions: [0.0; MAX_ELEMENTS],
            element_attack_skill_effectivenesses: [1.0; MAX_ELEMENTS],
            element_defense_skill_effectivenesses: [1.0; MAX_ELEMENTS],
            element_status_skill_effectivenesses: [1.0; MAX_ELEMENTS],
            element_movement_technique_effectivenesses: [1.0; MAX_ELEMENTS],
            element_healing_skill_effectivenesses: [1.0; MAX_ELEMENTS],
            element_support_skill_effectivenesses: [1.0; MAX_ELEMENTS],
            element_utility_skill_effectivenesses: [1.0; MAX_ELEMENTS],
            element_skill_effectivenesses: [1.0; MAX_ELEMENTS],
            
            element_resource_regenerations: [1.0; MAX_ELEMENTS],
            element_resource_efficiencies: [1.0; MAX_ELEMENTS],
            
            element_leadership_bonuses: [0.0; MAX_ELEMENTS],
            element_teaching_efficiencies: [1.0; MAX_ELEMENTS],
            element_crafting_efficiencies: [1.0; MAX_ELEMENTS],
            element_resource_discoveries: [1.0; MAX_ELEMENTS],
            
            element_sensitivities: [1.0; MAX_ELEMENTS],
            
            element_mastery_synergy_bonuses: [0.0; MAX_ELEMENTS],
            
            element_interactions: [[1.0; MAX_ELEMENTS]; MAX_ELEMENTS],
            element_feature_flags: [[false; MAX_FEATURE_FLAGS]; MAX_ELEMENTS],
            
            element_resource_amounts: [0.0; MAX_ELEMENTS],
            element_resource_capacities: [100.0; MAX_ELEMENTS],
            element_resource_regeneration_rates: [1.0; MAX_ELEMENTS],
        }
    }
    
    /// Get element mastery level by index (direct array access - 1-2 ns)
    pub fn get_element_mastery_level(&self, index: usize) -> Option<f64> {
        if index < MAX_ELEMENTS {
            Some(self.element_mastery_levels[index])
        } else {
            None
        }
    }
    
    /// Set element mastery level by index (direct array access - 1-2 ns)
    pub fn set_element_mastery_level(&mut self, index: usize, value: f64) -> bool {
        if index < MAX_ELEMENTS {
            self.element_mastery_levels[index] = value;
            true
        } else {
            false
        }
    }
    
    /// Get element power point by index (direct array access - 1-2 ns)
    pub fn get_element_power_point(&self, index: usize) -> Option<f64> {
        if index < MAX_ELEMENTS {
            Some(self.element_power_points[index])
        } else {
            None
        }
    }
    
    /// Set element power point by index (direct array access - 1-2 ns)
    pub fn set_element_power_point(&mut self, index: usize, value: f64) -> bool {
        if index < MAX_ELEMENTS {
            self.element_power_points[index] = value;
            true
        } else {
            false
        }
    }
    
    /// Get element interaction bonus (direct 2D array access - 1-2 ns)
    pub fn get_element_interaction(&self, attacker_index: usize, defender_index: usize) -> Option<f64> {
        if attacker_index < MAX_ELEMENTS && defender_index < MAX_ELEMENTS {
            Some(self.element_interactions[attacker_index][defender_index])
        } else {
            None
        }
    }
    
    /// Set element interaction bonus (direct 2D array access - 1-2 ns)
    pub fn set_element_interaction(&mut self, attacker_index: usize, defender_index: usize, value: f64) -> bool {
        if attacker_index < MAX_ELEMENTS && defender_index < MAX_ELEMENTS {
            self.element_interactions[attacker_index][defender_index] = value;
            true
        } else {
            false
        }
    }
    
    /// Get all element mastery levels (direct array access - 1-2 ns)
    pub fn get_all_element_mastery_levels(&self) -> &[f64; MAX_ELEMENTS] {
        &self.element_mastery_levels
    }
    
    /// Get all element power points (direct array access - 1-2 ns)
    pub fn get_all_element_power_points(&self) -> &[f64; MAX_ELEMENTS] {
        &self.element_power_points
    }
    
    /// Calculate total element mastery bonus
    pub fn calculate_total_element_mastery_bonus(&self) -> f64 {
        let mut total_bonus = 0.0;
        for i in 0..MAX_ELEMENTS {
            let rank = self.element_mastery_ranks[i];
            let level = self.element_mastery_levels[i];
            total_bonus += level * rank.get_stat_multiplier();
        }
        total_bonus
    }
    
    /// Calculate element synergy bonus for a specific element
    pub fn calculate_element_synergy_bonus(&self, element_index: usize) -> f64 {
        if element_index >= MAX_ELEMENTS {
            return 0.0;
        }
        
        let mut synergy_bonus = 0.0;
        for i in 0..MAX_ELEMENTS {
            if i != element_index {
                let interaction = self.element_interactions[element_index][i];
                let other_level = self.element_mastery_levels[i];
                synergy_bonus += interaction * other_level * 0.1; // 10% of interaction bonus
            }
        }
        
        synergy_bonus
    }
}

/// Element Mastery Rank enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Default for ElementMasteryRank {
    fn default() -> Self {
        ElementMasteryRank::Novice
    }
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
    
    /// Get rank name as string
    pub fn to_string(&self) -> &'static str {
        match self {
            ElementMasteryRank::Novice => "Novice",
            ElementMasteryRank::Apprentice => "Apprentice",
            ElementMasteryRank::Adept => "Adept",
            ElementMasteryRank::Expert => "Expert",
            ElementMasteryRank::Master => "Master",
            ElementMasteryRank::Grandmaster => "Grandmaster",
            ElementMasteryRank::Sage => "Sage",
            ElementMasteryRank::Transcendent => "Transcendent",
        }
    }
}
