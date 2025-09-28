//! # Elemental Data Structures (Fixed)
//! 
//! This module contains the CORRECT array-based elemental data structures according to docs.
//! 
//! ## Primary Stats (Stored directly)
//! - element_mastery_levels: Cấp độ tinh thông nguyên tố
//! - element_mastery_experience: Kinh nghiệm tinh thông
//! - element_qi_amounts: Lượng khí hiện tại
//! - element_qi_capacities: Dung lượng khí tối đa
//! - element_qi_regeneration_rates: Tốc độ hồi khí
//! 
//! ## Derived Stats (Calculated from Primary + Base Properties)
//! - element_mastery: Tinh thông nguyên tố (derived từ mastery_levels)
//! - power_point: Điểm tấn công (derived từ mastery + base_damage)
//! - defense_point: Điểm phòng thủ (derived từ mastery + base_defense)
//! - crit_rate: Tỷ lệ chí mạng (derived từ mastery + base_crit_rate)
//! - ... và tất cả các derived stats khác

// use serde::{Deserialize, Serialize}; // Removed for now

/// Maximum number of elements supported
pub const MAX_ELEMENTS: usize = 50;

/// Elemental mastery rank enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElementMasteryRank {
    Novice = 0,
    Apprentice = 1,
    Adept = 2,
    Expert = 3,
    Master = 4,
    Grandmaster = 5,
    Transcendent = 6,
}

impl ElementMasteryRank {
    /// Create ElementMasteryRank from experience value
    pub fn from_experience(experience: f64) -> Self {
        match experience as u32 {
            0..=99 => ElementMasteryRank::Novice,
            100..=499 => ElementMasteryRank::Apprentice,
            500..=999 => ElementMasteryRank::Adept,
            1000..=1999 => ElementMasteryRank::Expert,
            2000..=4999 => ElementMasteryRank::Master,
            5000..=9999 => ElementMasteryRank::Grandmaster,
            _ => ElementMasteryRank::Transcendent,
        }
    }
    
    /// Get the minimum experience required for this rank
    pub fn min_experience(&self) -> f64 {
        match self {
            ElementMasteryRank::Novice => 0.0,
            ElementMasteryRank::Apprentice => 100.0,
            ElementMasteryRank::Adept => 500.0,
            ElementMasteryRank::Expert => 1000.0,
            ElementMasteryRank::Master => 2000.0,
            ElementMasteryRank::Grandmaster => 5000.0,
            ElementMasteryRank::Transcendent => 10000.0,
        }
    }
    
    /// Get the maximum experience for this rank
    pub fn max_experience(&self) -> f64 {
        match self {
            ElementMasteryRank::Novice => 99.0,
            ElementMasteryRank::Apprentice => 499.0,
            ElementMasteryRank::Adept => 999.0,
            ElementMasteryRank::Expert => 1999.0,
            ElementMasteryRank::Master => 4999.0,
            ElementMasteryRank::Grandmaster => 9999.0,
            ElementMasteryRank::Transcendent => f64::INFINITY,
        }
    }
}

impl crate::common_traits::Validatable for ElementMasteryRank {
    fn validate(&self) -> crate::ElementCoreResult<()> {
        // ElementMasteryRank is an enum, so it's always valid
        // We could add validation for the experience ranges if needed
        Ok(())
    }
    
    fn get_validation_errors(&self) -> Vec<String> {
        // ElementMasteryRank is always valid
        vec![]
    }
}

/// Elemental system data structure with CORRECT primary/derived separation
#[derive(Debug, Clone)]
pub struct ElementalSystemData {
    // ===== PRIMARY STATS (Stored directly) =====
    
    /// Element mastery levels (primary stat)
    pub element_mastery_levels: [f64; MAX_ELEMENTS],
    
    /// Element mastery experience (primary stat)
    pub element_mastery_experience: [f64; MAX_ELEMENTS],
    
    /// Element mastery ranks (primary stat)
    pub element_mastery_ranks: [ElementMasteryRank; MAX_ELEMENTS],
    
    /// Element qi amounts (primary stat)
    pub element_qi_amounts: [f64; MAX_ELEMENTS],
    
    /// Element qi capacities (primary stat)
    pub element_qi_capacities: [f64; MAX_ELEMENTS],
    
    /// Element qi regeneration rates (primary stat)
    pub element_qi_regeneration_rates: [f64; MAX_ELEMENTS],
    
    // ===== DERIVED STATS (Calculated from Primary + Base Properties) =====
    
    // Core Element Mastery (derived)
    pub element_mastery: [f64; MAX_ELEMENTS],                    // Derived from mastery_levels
    
    // Counterbalance Pairs (derived)
    pub power_point: [f64; MAX_ELEMENTS],                        // Derived from mastery + base_damage
    pub defense_point: [f64; MAX_ELEMENTS],                      // Derived from mastery + base_defense
    pub crit_rate: [f64; MAX_ELEMENTS],                          // Derived from mastery + base_crit_rate
    pub resist_crit_rate: [f64; MAX_ELEMENTS],                   // Derived from mastery + resist
    pub crit_damage: [f64; MAX_ELEMENTS],                        // Derived from mastery + base_crit_damage
    pub resist_crit_damage: [f64; MAX_ELEMENTS],                 // Derived from mastery + resist
    pub accurate_rate: [f64; MAX_ELEMENTS],                      // Derived from mastery + base_accuracy
    pub dodge_rate: [f64; MAX_ELEMENTS],                         // Derived from mastery + dodge
    pub status_probability: [f64; MAX_ELEMENTS],                 // Derived from mastery + status_prob
    pub status_resistance: [f64; MAX_ELEMENTS],                  // Derived from mastery + status_resist
    pub status_duration: [f64; MAX_ELEMENTS],                    // Derived from mastery + duration
    pub status_duration_reduction: [f64; MAX_ELEMENTS],          // Derived from mastery + duration_reduction
    pub status_intensity: [f64; MAX_ELEMENTS],                   // Derived from mastery + intensity
    pub status_intensity_reduction: [f64; MAX_ELEMENTS],         // Derived from mastery + intensity_reduction
    pub element_penetration: [f64; MAX_ELEMENTS],                // Derived from mastery + penetration
    pub element_absorption: [f64; MAX_ELEMENTS],                 // Derived from mastery + absorption
    pub element_amplification: [f64; MAX_ELEMENTS],              // Derived from mastery + amplification
    pub element_reduction: [f64; MAX_ELEMENTS],                  // Derived from mastery + reduction
    pub reflection_rate: [f64; MAX_ELEMENTS],                    // Derived from mastery + reflection
    pub resist_reflection_rate: [f64; MAX_ELEMENTS],             // Derived from mastery + resist_reflection
    pub reflection_damage: [f64; MAX_ELEMENTS],                  // Derived from mastery + reflection_damage
    pub resist_reflection_damage: [f64; MAX_ELEMENTS],           // Derived from mastery + resist_reflection_damage
    
    // Parry System (derived)
    pub parry_rate: [f64; MAX_ELEMENTS],                         // Derived from mastery + parry
    pub parry_break: [f64; MAX_ELEMENTS],                        // Derived from mastery + parry_break
    pub parry_strength: [f64; MAX_ELEMENTS],                     // Derived from mastery + parry_strength
    pub parry_shred: [f64; MAX_ELEMENTS],                        // Derived from mastery + parry_shred
    
    // Block System (derived)
    pub block_rate: [f64; MAX_ELEMENTS],                         // Derived from mastery + block
    pub block_break: [f64; MAX_ELEMENTS],                        // Derived from mastery + block_break
    pub block_strength: [f64; MAX_ELEMENTS],                     // Derived from mastery + block_strength
    pub block_shred: [f64; MAX_ELEMENTS],                        // Derived from mastery + block_shred
    
    // Skill Execution & Performance (derived)
    pub skill_execution_speed: [f64; MAX_ELEMENTS],              // Derived from mastery + execution_speed
    pub skill_cooldown_reduction: [f64; MAX_ELEMENTS],           // Derived from mastery + cooldown_reduction
    pub attack_skill_effectiveness: [f64; MAX_ELEMENTS],         // Derived from mastery + attack_effectiveness
    pub defense_skill_effectiveness: [f64; MAX_ELEMENTS],        // Derived from mastery + defense_effectiveness
    pub status_skill_effectiveness: [f64; MAX_ELEMENTS],         // Derived from mastery + status_effectiveness
    pub movement_technique_effectiveness: [f64; MAX_ELEMENTS],   // Derived from mastery + movement_effectiveness
    pub healing_skill_effectiveness: [f64; MAX_ELEMENTS],        // Derived from mastery + healing_effectiveness
    pub support_skill_effectiveness: [f64; MAX_ELEMENTS],        // Derived from mastery + support_effectiveness
    pub utility_skill_effectiveness: [f64; MAX_ELEMENTS],        // Derived from mastery + utility_effectiveness
    pub skill_effectiveness: [f64; MAX_ELEMENTS],                // Derived from mastery + general_effectiveness
    
    // Resource Management (derived)
    pub resource_regeneration: [f64; MAX_ELEMENTS],              // Derived from mastery + resource_regen
    pub resource_efficiency: [f64; MAX_ELEMENTS],                // Derived from mastery + resource_efficiency
    
    // Social & Economy (derived)
    pub element_leadership_bonus: [f64; MAX_ELEMENTS],           // Derived from mastery + leadership
    pub element_teaching_efficiency: [f64; MAX_ELEMENTS],        // Derived from mastery + teaching
    pub element_crafting_efficiency: [f64; MAX_ELEMENTS],        // Derived from mastery + crafting
    pub element_resource_discovery: [f64; MAX_ELEMENTS],         // Derived from mastery + resource_discovery
    
    // Perception & Detection (derived)
    pub element_sensitivity: [f64; MAX_ELEMENTS],                // Derived from mastery + sensitivity
    
    // Advanced Combat Mechanics (derived)
    pub mastery_synergy_bonus: [f64; MAX_ELEMENTS],              // Derived from mastery + synergy
    
    // Element Interactions (2D array for interaction bonuses)
    pub element_interaction_bonuses: [[f64; MAX_ELEMENTS]; MAX_ELEMENTS],
    
    // Feature Flags (2D array for feature flags)
    pub feature_flags: [[bool; 16]; MAX_ELEMENTS],               // 16 feature flags per element
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
            // ===== PRIMARY STATS (Initialize with defaults) =====
            element_mastery_levels: [0.0; MAX_ELEMENTS],
            element_mastery_experience: [0.0; MAX_ELEMENTS],
            element_mastery_ranks: [ElementMasteryRank::Novice; MAX_ELEMENTS],
            element_qi_amounts: [0.0; MAX_ELEMENTS],
            element_qi_capacities: [100.0; MAX_ELEMENTS],
            element_qi_regeneration_rates: [1.0; MAX_ELEMENTS],
            
            // ===== DERIVED STATS (Initialize with zeros - will be calculated) =====
            element_mastery: [0.0; MAX_ELEMENTS],
            power_point: [0.0; MAX_ELEMENTS],
            defense_point: [0.0; MAX_ELEMENTS],
            crit_rate: [0.05; MAX_ELEMENTS],
            resist_crit_rate: [0.05; MAX_ELEMENTS],
            crit_damage: [1.0; MAX_ELEMENTS],
            resist_crit_damage: [0.0; MAX_ELEMENTS],
            accurate_rate: [0.5; MAX_ELEMENTS],
            dodge_rate: [0.05; MAX_ELEMENTS],
            status_probability: [0.1; MAX_ELEMENTS],
            status_resistance: [0.1; MAX_ELEMENTS],
            status_duration: [1.0; MAX_ELEMENTS],
            status_duration_reduction: [0.0; MAX_ELEMENTS],
            status_intensity: [1.0; MAX_ELEMENTS],
            status_intensity_reduction: [0.0; MAX_ELEMENTS],
            element_penetration: [0.0; MAX_ELEMENTS],
            element_absorption: [0.0; MAX_ELEMENTS],
            element_amplification: [0.0; MAX_ELEMENTS],
            element_reduction: [0.0; MAX_ELEMENTS],
            reflection_rate: [0.0; MAX_ELEMENTS],
            resist_reflection_rate: [0.0; MAX_ELEMENTS],
            reflection_damage: [0.0; MAX_ELEMENTS],
            resist_reflection_damage: [0.0; MAX_ELEMENTS],
            
            // Parry System
            parry_rate: [0.05; MAX_ELEMENTS],
            parry_break: [0.0; MAX_ELEMENTS],
            parry_strength: [0.0; MAX_ELEMENTS],
            parry_shred: [0.0; MAX_ELEMENTS],
            
            // Block System
            block_rate: [0.05; MAX_ELEMENTS],
            block_break: [0.0; MAX_ELEMENTS],
            block_strength: [0.0; MAX_ELEMENTS],
            block_shred: [0.0; MAX_ELEMENTS],
            
            // Skill Execution & Performance
            skill_execution_speed: [1.0; MAX_ELEMENTS],
            skill_cooldown_reduction: [0.0; MAX_ELEMENTS],
            attack_skill_effectiveness: [1.0; MAX_ELEMENTS],
            defense_skill_effectiveness: [1.0; MAX_ELEMENTS],
            status_skill_effectiveness: [1.0; MAX_ELEMENTS],
            movement_technique_effectiveness: [1.0; MAX_ELEMENTS],
            healing_skill_effectiveness: [1.0; MAX_ELEMENTS],
            support_skill_effectiveness: [1.0; MAX_ELEMENTS],
            utility_skill_effectiveness: [1.0; MAX_ELEMENTS],
            skill_effectiveness: [1.0; MAX_ELEMENTS],
            
            // Resource Management
            resource_regeneration: [1.0; MAX_ELEMENTS],
            resource_efficiency: [1.0; MAX_ELEMENTS],
            
            // Social & Economy
            element_leadership_bonus: [0.0; MAX_ELEMENTS],
            element_teaching_efficiency: [1.0; MAX_ELEMENTS],
            element_crafting_efficiency: [1.0; MAX_ELEMENTS],
            element_resource_discovery: [0.0; MAX_ELEMENTS],
            
            // Perception & Detection
            element_sensitivity: [1.0; MAX_ELEMENTS],
            
            // Advanced Combat Mechanics
            mastery_synergy_bonus: [0.0; MAX_ELEMENTS],
            
            // Element Interactions & Feature Flags
            element_interaction_bonuses: [[0.0; MAX_ELEMENTS]; MAX_ELEMENTS],
            feature_flags: [[false; 16]; MAX_ELEMENTS],
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
    
    /// Get element qi amount by index (direct array access - 1-2 ns)
    pub fn get_element_qi_amount(&self, index: usize) -> Option<f64> {
        if index < MAX_ELEMENTS {
            Some(self.element_qi_amounts[index])
        } else {
            None
        }
    }
    
    /// Get element power point by index (derived stat - direct array access - 1-2 ns)
    pub fn get_element_power_point(&self, index: usize) -> Option<f64> {
        if index < MAX_ELEMENTS {
            Some(self.power_point[index])
        } else {
            None
        }
    }
    
    /// Get element defense point by index (derived stat - direct array access - 1-2 ns)
    pub fn get_element_defense_point(&self, index: usize) -> Option<f64> {
        if index < MAX_ELEMENTS {
            Some(self.defense_point[index])
        } else {
            None
        }
    }
    
    /// Set element mastery level by index (direct array access - 1-2 ns)
    pub fn set_element_mastery_level(&mut self, index: usize, level: f64) -> Result<(), crate::ElementCoreError> {
        if index >= MAX_ELEMENTS {
            return Err(crate::ElementCoreError::IndexOutOfBounds { 
                index, 
                max: MAX_ELEMENTS
            });
        }
        
        // Validate level value
        if !level.is_finite() {
            return Err(crate::ElementCoreError::Validation { 
                message: format!("Mastery level must be finite, got {}", level)
            });
        }
        
        if level < 0.0 {
            return Err(crate::ElementCoreError::Validation { 
                message: format!("Mastery level must be non-negative, got {}", level)
            });
        }
        
        if level > 1000.0 {
            return Err(crate::ElementCoreError::Validation { 
                message: format!("Mastery level too high (max 1000.0), got {}", level)
            });
        }
        
        self.element_mastery_levels[index] = level;
        Ok(())
    }
    
    /// Set element qi amount by index (direct array access - 1-2 ns)
    pub fn set_element_qi_amount(&mut self, index: usize, amount: f64) -> Result<(), crate::ElementCoreError> {
        if index >= MAX_ELEMENTS {
            return Err(crate::ElementCoreError::IndexOutOfBounds { index, max: MAX_ELEMENTS });
        }
        
        // Validate amount value
        if !amount.is_finite() {
            return Err(crate::ElementCoreError::Validation { 
                message: format!("QI amount must be finite, got {}", amount)
            });
        }
        
        if amount < 0.0 {
            return Err(crate::ElementCoreError::Validation { 
                message: format!("QI amount must be non-negative, got {}", amount)
            });
        }
        
        // Check against capacity
        if amount > self.element_qi_capacities[index] {
            return Err(crate::ElementCoreError::Validation { 
                message: format!("QI amount {} exceeds capacity {}", amount, self.element_qi_capacities[index])
            });
        }
        
        self.element_qi_amounts[index] = amount;
        Ok(())
    }
    
    /// Set element power point by index (derived stat - direct array access - 1-2 ns)
    pub fn set_element_power_point(&mut self, index: usize, power: f64) -> Result<(), crate::ElementCoreError> {
        if index < MAX_ELEMENTS {
            self.power_point[index] = power;
            Ok(())
        } else {
            Err(crate::ElementCoreError::IndexOutOfBounds { index, max: MAX_ELEMENTS })
        }
    }
    
    /// Set element defense point by index (derived stat - direct array access - 1-2 ns)
    pub fn set_element_defense_point(&mut self, index: usize, defense: f64) -> Result<(), crate::ElementCoreError> {
        if index < MAX_ELEMENTS {
            self.defense_point[index] = defense;
            Ok(())
        } else {
            Err(crate::ElementCoreError::IndexOutOfBounds { index, max: MAX_ELEMENTS })
        }
    }
    
    /// Calculate derived stats for an element (based on mastery level and base properties)
    pub fn calculate_derived_stats(&mut self, index: usize, base_damage: f64, base_defense: f64, base_crit_rate: f64, base_crit_damage: f64, base_accuracy: f64) -> Result<(), crate::ElementCoreError> {
        if index >= MAX_ELEMENTS {
            return Err(crate::ElementCoreError::IndexOutOfBounds { index, max: MAX_ELEMENTS });
        }
        
        let mastery_level = self.element_mastery_levels[index];
        let mastery_factor = 1.0 + (mastery_level * 0.1); // 10% bonus per mastery level
        
        // Calculate derived stats based on mastery level and base properties
        self.element_mastery[index] = mastery_level;
        self.power_point[index] = base_damage * mastery_factor;
        self.defense_point[index] = base_defense * mastery_factor;
        self.crit_rate[index] = base_crit_rate * mastery_factor;
        self.crit_damage[index] = base_crit_damage * mastery_factor;
        self.accurate_rate[index] = base_accuracy * mastery_factor;
        
        Ok(())
    }
    
    /// Get total elemental mastery across all elements (sum of mastery levels)
    pub fn get_total_elemental_mastery(&self) -> f64 {
        self.element_mastery_levels.iter().sum()
    }
    
    /// Get total qi amount across all elements
    pub fn get_total_qi_amount(&self) -> f64 {
        self.element_qi_amounts.iter().sum()
    }
    
    /// Get total power points across all elements
    pub fn get_total_power_points(&self) -> f64 {
        self.power_point.iter().sum()
    }
    
    /// Get total defense points across all elements
    pub fn get_total_defense_points(&self) -> f64 {
        self.defense_point.iter().sum()
    }
    
    /// Get element interaction bonus (direct 2D array access - 1-2 ns)
    pub fn get_element_interaction(&self, attacker_index: usize, defender_index: usize) -> Option<f64> {
        if attacker_index < MAX_ELEMENTS && defender_index < MAX_ELEMENTS {
            Some(self.element_interaction_bonuses[attacker_index][defender_index])
        } else {
            None
        }
    }
    
    /// Set element interaction bonus (direct 2D array access - 1-2 ns)
    pub fn set_element_interaction(&mut self, attacker_index: usize, defender_index: usize, value: f64) -> Result<(), crate::ElementCoreError> {
        if attacker_index < MAX_ELEMENTS && defender_index < MAX_ELEMENTS {
            self.element_interaction_bonuses[attacker_index][defender_index] = value;
            Ok(())
        } else {
            Err(crate::ElementCoreError::IndexOutOfBounds { 
                index: attacker_index.max(defender_index), 
                max: MAX_ELEMENTS 
            })
        }
    }
    
    /// Get all element mastery levels (for compatibility)
    pub fn get_all_element_mastery_levels(&self) -> Vec<f64> {
        self.element_mastery_levels.to_vec()
    }
    
    /// Get all element power points (for compatibility)
    pub fn get_all_element_power_points(&self) -> Vec<f64> {
        self.power_point.to_vec()
    }
    
    /// Calculate total element mastery bonus (for compatibility)
    pub fn calculate_total_element_mastery_bonus(&self) -> f64 {
        self.get_total_elemental_mastery()
    }
    
    /// Calculate element synergy bonus (for compatibility)
    pub fn calculate_element_synergy_bonus(&self, element_index: usize) -> f64 {
        if element_index < MAX_ELEMENTS {
            self.mastery_synergy_bonus[element_index]
        } else {
            0.0
        }
    }
}

impl crate::common_traits::Validatable for ElementalSystemData {
    fn validate(&self) -> crate::ElementCoreResult<()> {
        // Validate mastery levels are non-negative
        for (i, level) in self.element_mastery_levels.iter().enumerate() {
            if *level < 0.0 {
                return Err(crate::ElementCoreError::Validation { 
                    message: format!("Element mastery level at index {} is negative: {}", i, level)
                });
            }
        }
        
        // Validate experience is non-negative
        for (i, exp) in self.element_mastery_experience.iter().enumerate() {
            if *exp < 0.0 {
                return Err(crate::ElementCoreError::Validation { 
                    message: format!("Element mastery experience at index {} is negative: {}", i, exp)
                });
            }
        }
        
        // Validate qi amounts are non-negative
        for (i, qi) in self.element_qi_amounts.iter().enumerate() {
            if *qi < 0.0 {
                return Err(crate::ElementCoreError::Validation { 
                    message: format!("Element qi amount at index {} is negative: {}", i, qi)
                });
            }
        }
        
        // Validate qi capacities are positive
        for (i, capacity) in self.element_qi_capacities.iter().enumerate() {
            if *capacity <= 0.0 {
                return Err(crate::ElementCoreError::Validation { 
                    message: format!("Element qi capacity at index {} is not positive: {}", i, capacity)
                });
            }
        }
        
        // Validate qi regeneration rates are non-negative
        for (i, rate) in self.element_qi_regeneration_rates.iter().enumerate() {
            if *rate < 0.0 {
                return Err(crate::ElementCoreError::Validation { 
                    message: format!("Element qi regeneration rate at index {} is negative: {}", i, rate)
                });
            }
        }
        
        // Validate qi amounts don't exceed capacities
        for (i, (amount, capacity)) in self.element_qi_amounts.iter().zip(self.element_qi_capacities.iter()).enumerate() {
            if *amount > *capacity {
                return Err(crate::ElementCoreError::Validation { 
                    message: format!("Element qi amount at index {} exceeds capacity: {} > {}", i, amount, capacity)
                });
            }
        }
        
        // Validate derived stats are non-negative
        let derived_stats = [
            &self.element_mastery,
            &self.power_point,
            &self.defense_point,
            &self.crit_rate,
            &self.resist_crit_rate,
            &self.crit_damage,
            &self.resist_crit_damage,
            &self.accurate_rate,
            &self.dodge_rate,
            &self.status_probability,
            &self.status_resistance,
            &self.status_duration,
            &self.status_duration_reduction,
            &self.status_intensity,
            &self.status_intensity_reduction,
            &self.element_penetration,
            &self.element_absorption,
            &self.element_amplification,
            &self.element_reduction,
            &self.reflection_rate,
            &self.resist_reflection_rate,
        ];
        
        for (stat_idx, stat_array) in derived_stats.iter().enumerate() {
            for (i, value) in stat_array.iter().enumerate() {
                if *value < 0.0 {
                    return Err(crate::ElementCoreError::Validation { 
                        message: format!("Derived stat {} at index {} is negative: {}", stat_idx, i, value)
                    });
                }
            }
        }
        
        // Validate rates are between 0 and 1
        let rate_stats = [
            (&self.crit_rate, "crit_rate"),
            (&self.resist_crit_rate, "resist_crit_rate"),
            (&self.accurate_rate, "accurate_rate"),
            (&self.dodge_rate, "dodge_rate"),
            (&self.status_probability, "status_probability"),
            (&self.status_resistance, "status_resistance"),
            (&self.reflection_rate, "reflection_rate"),
            (&self.resist_reflection_rate, "resist_reflection_rate"),
        ];
        
        for (stat_array, stat_name) in rate_stats.iter() {
            for (i, value) in stat_array.iter().enumerate() {
                if *value < 0.0 || *value > 1.0 {
                    return Err(crate::ElementCoreError::Validation { 
                        message: format!("Rate stat {} at index {} is out of range [0,1]: {}", stat_name, i, value)
                    });
                }
            }
        }
        
        // Validate damage multipliers are positive
        let damage_stats = [
            (&self.crit_damage, "crit_damage"),
            (&self.resist_crit_damage, "resist_crit_damage"),
        ];
        
        for (stat_array, stat_name) in damage_stats.iter() {
            for (i, value) in stat_array.iter().enumerate() {
                if *value < 1.0 {
                    return Err(crate::ElementCoreError::Validation { 
                        message: format!("Damage stat {} at index {} is less than 1.0: {}", stat_name, i, value)
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn get_validation_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Check mastery levels
        for (i, level) in self.element_mastery_levels.iter().enumerate() {
            if *level < 0.0 {
                errors.push(format!("Element mastery level at index {} is negative: {}", i, level));
            }
        }
        
        // Check experience
        for (i, exp) in self.element_mastery_experience.iter().enumerate() {
            if *exp < 0.0 {
                errors.push(format!("Element mastery experience at index {} is negative: {}", i, exp));
            }
        }
        
        // Check qi amounts
        for (i, qi) in self.element_qi_amounts.iter().enumerate() {
            if *qi < 0.0 {
                errors.push(format!("Element qi amount at index {} is negative: {}", i, qi));
            }
        }
        
        // Check qi capacities
        for (i, capacity) in self.element_qi_capacities.iter().enumerate() {
            if *capacity <= 0.0 {
                errors.push(format!("Element qi capacity at index {} is not positive: {}", i, capacity));
            }
        }
        
        // Check qi regeneration rates
        for (i, rate) in self.element_qi_regeneration_rates.iter().enumerate() {
            if *rate < 0.0 {
                errors.push(format!("Element qi regeneration rate at index {} is negative: {}", i, rate));
            }
        }
        
        // Check qi amounts don't exceed capacities
        for (i, (amount, capacity)) in self.element_qi_amounts.iter().zip(self.element_qi_capacities.iter()).enumerate() {
            if *amount > *capacity {
                errors.push(format!("Element qi amount at index {} exceeds capacity: {} > {}", i, amount, capacity));
            }
        }
        
        // Check derived stats
        let derived_stats = [
            (&self.element_mastery, "element_mastery"),
            (&self.power_point, "power_point"),
            (&self.defense_point, "defense_point"),
            (&self.crit_rate, "crit_rate"),
            (&self.resist_crit_rate, "resist_crit_rate"),
            (&self.crit_damage, "crit_damage"),
            (&self.resist_crit_damage, "resist_crit_damage"),
            (&self.accurate_rate, "accurate_rate"),
            (&self.dodge_rate, "dodge_rate"),
            (&self.status_probability, "status_probability"),
            (&self.status_resistance, "status_resistance"),
            (&self.status_duration, "status_duration"),
            (&self.status_duration_reduction, "status_duration_reduction"),
            (&self.status_intensity, "status_intensity"),
            (&self.status_intensity_reduction, "status_intensity_reduction"),
            (&self.element_penetration, "element_penetration"),
            (&self.element_absorption, "element_absorption"),
            (&self.element_amplification, "element_amplification"),
            (&self.element_reduction, "element_reduction"),
            (&self.reflection_rate, "reflection_rate"),
            (&self.resist_reflection_rate, "resist_reflection_rate"),
        ];
        
        for (stat_array, stat_name) in derived_stats.iter() {
            for (i, value) in stat_array.iter().enumerate() {
                if *value < 0.0 {
                    errors.push(format!("Derived stat {} at index {} is negative: {}", stat_name, i, value));
                }
            }
        }
        
        // Check rate stats are between 0 and 1
        let rate_stats = [
            (&self.crit_rate, "crit_rate"),
            (&self.resist_crit_rate, "resist_crit_rate"),
            (&self.accurate_rate, "accurate_rate"),
            (&self.dodge_rate, "dodge_rate"),
            (&self.status_probability, "status_probability"),
            (&self.status_resistance, "status_resistance"),
            (&self.reflection_rate, "reflection_rate"),
            (&self.resist_reflection_rate, "resist_reflection_rate"),
        ];
        
        for (stat_array, stat_name) in rate_stats.iter() {
            for (i, value) in stat_array.iter().enumerate() {
                if *value < 0.0 || *value > 1.0 {
                    errors.push(format!("Rate stat {} at index {} is out of range [0,1]: {}", stat_name, i, value));
                }
            }
        }
        
        // Check damage multipliers are positive
        let damage_stats = [
            (&self.crit_damage, "crit_damage"),
            (&self.resist_crit_damage, "resist_crit_damage"),
        ];
        
        for (stat_array, stat_name) in damage_stats.iter() {
            for (i, value) in stat_array.iter().enumerate() {
                if *value < 1.0 {
                    errors.push(format!("Damage stat {} at index {} is less than 1.0: {}", stat_name, i, value));
                }
            }
        }
        
        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_elemental_system_data_creation() {
        let data = ElementalSystemData::new();
        assert_eq!(data.element_mastery_levels[0], 0.0);
        assert_eq!(data.element_qi_amounts[0], 0.0);
        assert_eq!(data.element_qi_capacities[0], 100.0);
    }
    
    #[test]
    fn test_derived_stats_calculation() {
        let mut data = ElementalSystemData::new();
        
        // Set mastery level
        data.set_element_mastery_level(0, 10.0).unwrap();
        
        // Calculate derived stats
        data.calculate_derived_stats(0, 100.0, 80.0, 0.15, 1.5, 0.85).unwrap();
        
        // Check derived stats
        assert_eq!(data.element_mastery[0], 10.0);
        assert_eq!(data.power_point[0], 200.0); // 100.0 * (1.0 + 10.0 * 0.1)
        assert_eq!(data.defense_point[0], 160.0); // 80.0 * (1.0 + 10.0 * 0.1)
        assert_eq!(data.crit_rate[0], 0.3); // 0.15 * (1.0 + 10.0 * 0.1)
    }
    
    #[test]
    fn test_direct_array_access_performance() {
        let mut data = ElementalSystemData::new();
        
        // Test direct array access (should be 1-2 ns)
        data.element_mastery_levels[0] = 5.0;
        data.element_qi_amounts[0] = 100.0;
        data.power_point[0] = 150.0;
        
        assert_eq!(data.element_mastery_levels[0], 5.0);
        assert_eq!(data.element_qi_amounts[0], 100.0);
        assert_eq!(data.power_point[0], 150.0);
    }
    
    #[test]
    fn test_bounds_checking() {
        let mut data = ElementalSystemData::new();
        
        // Test bounds checking
        assert!(data.set_element_mastery_level(MAX_ELEMENTS, 10.0).is_err());
        assert!(data.set_element_qi_amount(MAX_ELEMENTS + 1, 100.0).is_err());
        assert!(data.calculate_derived_stats(MAX_ELEMENTS, 100.0, 80.0, 0.15, 1.5, 0.85).is_err());
    }
    
    #[test]
    fn test_total_calculations() {
        let mut data = ElementalSystemData::new();
        
        // Set some values
        data.element_mastery_levels[0] = 5.0;
        data.element_mastery_levels[1] = 10.0;
        data.element_qi_amounts[0] = 100.0;
        data.element_qi_amounts[1] = 200.0;
        data.power_point[0] = 150.0;
        data.power_point[1] = 300.0;
        
        // Calculate totals
        let total_mastery = data.get_total_elemental_mastery();
        let total_qi = data.get_total_qi_amount();
        let total_power = data.get_total_power_points();
        
        assert_eq!(total_mastery, 15.0);
        assert_eq!(total_qi, 300.0);
        assert_eq!(total_power, 450.0);
    }
}
