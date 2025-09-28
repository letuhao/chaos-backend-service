//! # Elemental System
//! 
//! This module contains the elemental system implementation.

use crate::core::elemental_data::{ElementalSystemData, ElementMasteryLevel, MAX_ELEMENTS};

/// Elemental system implementation
pub struct ElementalSystem {
    data: ElementalSystemData,
}

impl Default for ElementalSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ElementalSystem {
    /// Create a new elemental system instance
    pub fn new() -> Self {
        Self {
            data: ElementalSystemData::new(),
        }
    }
    
    /// Create elemental system from data
    pub fn from_data(data: ElementalSystemData) -> Self {
        Self { data }
    }
    
    /// Get reference to elemental data
    pub fn get_data(&self) -> &ElementalSystemData {
        &self.data
    }
    
    /// Get mutable reference to elemental data
    pub fn get_data_mut(&mut self) -> &mut ElementalSystemData {
        &mut self.data
    }
    
    /// Set elemental data
    pub fn set_data(&mut self, data: ElementalSystemData) {
        self.data = data;
    }
    
    /// Get element mastery level value by index (direct array access - 1-2 ns)
    pub fn get_element_mastery_level_value(&self, index: usize) -> Option<f64> {
        self.data.get_element_mastery_level(index)
    }
    
    /// Set element mastery level by index (direct array access - 1-2 ns)
    pub fn set_element_mastery_level(&mut self, index: usize, value: f64) -> bool {
        self.data.set_element_mastery_level(index, value).is_ok()
    }
    
    /// Get element power point by index (direct array access - 1-2 ns)
    pub fn get_element_power_point(&self, index: usize) -> Option<f64> {
        self.data.get_element_power_point(index)
    }
    
    /// Set element power point by index (direct array access - 1-2 ns)
    pub fn set_element_power_point(&mut self, index: usize, value: f64) -> bool {
        self.data.set_element_power_point(index, value).is_ok()
    }
    
    /// Get element interaction bonus (direct 2D array access - 1-2 ns)
    pub fn get_element_interaction(&self, attacker_index: usize, defender_index: usize) -> Option<f64> {
        self.data.get_element_interaction(attacker_index, defender_index)
    }
    
    /// Set element interaction bonus (direct 2D array access - 1-2 ns)
    pub fn set_element_interaction(&mut self, attacker_index: usize, defender_index: usize, value: f64) -> bool {
        self.data.set_element_interaction(attacker_index, defender_index, value).is_ok()
    }
    
    /// Get all element mastery levels (direct array access - 1-2 ns)
    pub fn get_all_element_mastery_levels(&self) -> &[f64; MAX_ELEMENTS] {
        &self.data.element_mastery_levels
    }
    
    /// Get all element power points (direct array access - 1-2 ns)
    pub fn get_all_element_power_points(&self) -> &[f64; MAX_ELEMENTS] {
        &self.data.power_point
    }
    
    /// Calculate total element mastery bonus
    pub fn calculate_total_element_mastery_bonus(&self) -> f64 {
        self.data.calculate_total_element_mastery_bonus()
    }
    
    /// Calculate element synergy bonus for a specific element
    pub fn calculate_element_synergy_bonus(&self, element_index: usize) -> f64 {
        self.data.calculate_element_synergy_bonus(element_index)
    }
}

// SystemData trait implementation removed - will be implemented later

// SystemContribution trait implementation removed - will be implemented later
// REMOVED: SystemContribution implementation violates data hub pattern
// Element-Core should act as a data hub that aggregates data from external systems,
// not as a monolithic system that contributes directly to Actor-Core stats.
// 
// Instead, external systems (Race-Core, Item-Core, Skill-Core) should:
// 1. Register as ElementContributor with Element-Core
// 2. Contribute elemental data through ElementContribution
// 3. Element-Core aggregates and caches the data
// 4. Actor-Core queries Element-Core for aggregated elemental stats
//
// This maintains the data hub pattern and external contributor pattern.

impl ElementalSystem {
    // ElementalContribution struct removed - will be implemented later
    
    /// Get element mastery rank by index
    pub fn get_element_mastery_level(&self, index: usize) -> Option<ElementMasteryLevel> {
        if index < MAX_ELEMENTS {
            Some(self.data.element_mastery_level_enums[index])
        } else {
            None
        }
    }
    
    /// Update element mastery level based on experience
    pub fn update_element_mastery_level(&mut self, index: usize) -> bool {
        if index < MAX_ELEMENTS {
            let experience = self.data.element_mastery_experience[index] as i64;
            self.data.element_mastery_level_enums[index] = ElementMasteryLevel::from_experience(experience);
            true
        } else {
            false
        }
    }
    
    /// Add experience to element mastery
    pub fn add_element_mastery_experience(&mut self, index: usize, experience: f64) -> bool {
        if index < MAX_ELEMENTS && experience > 0.0 {
            self.data.element_mastery_experience[index] += experience;
            self.update_element_mastery_level(index);
            true
        } else {
            false
        }
    }
    
    /// Get element qi amount by index
    pub fn get_element_qi_amount(&self, index: usize) -> Option<f64> {
        if index < MAX_ELEMENTS {
            Some(self.data.element_qi_amounts[index])
        } else {
            None
        }
    }
    
    /// Set element qi amount by index
    pub fn set_element_qi_amount(&mut self, index: usize, value: f64) -> bool {
        if index < MAX_ELEMENTS && value >= 0.0 {
            let capacity = self.data.element_qi_capacities[index];
            self.data.element_qi_amounts[index] = value.min(capacity);
            true
        } else {
            false
        }
    }
    
    /// Regenerate element qi based on regeneration rates
    pub fn regenerate_element_qi(&mut self, delta_time: f64) {
        for i in 0..MAX_ELEMENTS {
            let regen_rate = self.data.element_qi_regeneration_rates[i];
            let capacity = self.data.element_qi_capacities[i];
            
            let regen_amount = regen_rate * delta_time;
            self.data.element_qi_amounts[i] = (self.data.element_qi_amounts[i] + regen_amount).min(capacity);
        }
    }
}
