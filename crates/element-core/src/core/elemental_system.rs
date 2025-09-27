//! # Elemental System
//! 
//! This module contains the elemental system implementation.

use crate::core::elemental_data::{ElementalSystemData, ElementMasteryRank, MAX_ELEMENTS};

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
    
    /// Get element mastery level by index (direct array access - 1-2 ns)
    pub fn get_element_mastery_level(&self, index: usize) -> Option<f64> {
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
/*
impl SystemContribution for ElementalSystem {
    fn calculate_hp_contribution(&self) -> f64 {
        // Calculate HP contribution based on elemental mastery levels
        let total_mastery = self.calculate_total_element_mastery_bonus();
        total_mastery * 10.0 // 10 HP per mastery point
    }
    
    fn calculate_mp_contribution(&self) -> f64 {
        // Calculate MP contribution based on qi amounts
        let total_qi: f64 = self.data.element_qi_amounts.iter().sum();
        total_qi * 0.5 // 0.5 MP per qi point
    }
    
    fn calculate_attack_power_contribution(&self) -> f64 {
        // Calculate attack power contribution based on power points
        let total_power: f64 = self.data.element_power_points.iter().sum();
        total_power * 1.0 // 1:1 ratio
    }
    
    fn calculate_defense_contribution(&self) -> f64 {
        // Calculate defense contribution based on defense points
        let total_defense: f64 = self.data.element_defense_points.iter().sum();
        total_defense * 1.0 // 1:1 ratio
    }
    
    fn calculate_speed_contribution(&self) -> f64 {
        // Calculate speed contribution based on qi manipulation speeds
        let total_speed: f64 = self.data.element_qi_manipulation_speeds.iter().sum();
        total_speed * 0.1 // 0.1 speed per manipulation speed point
    }
    
    fn calculate_critical_rate_contribution(&self) -> f64 {
        // Calculate critical rate contribution based on crit rates
        let total_crit_rate: f64 = self.data.element_crit_rates.iter().sum();
        total_crit_rate * 0.01 // Convert to percentage
    }
    
    fn calculate_critical_damage_contribution(&self) -> f64 {
        // Calculate critical damage contribution based on crit damages
        let total_crit_damage: f64 = self.data.element_crit_damages.iter().sum();
        total_crit_damage * 0.1 // Scale down
    }
    
    fn calculate_accuracy_contribution(&self) -> f64 {
        // Calculate accuracy contribution based on accurate rates
        let total_accuracy: f64 = self.data.element_accurate_rates.iter().sum();
        total_accuracy * 0.01 // Convert to percentage
    }
    
    fn calculate_dodge_rate_contribution(&self) -> f64 {
        // Calculate dodge rate contribution based on dodge rates
        let total_dodge_rate: f64 = self.data.element_dodge_rates.iter().sum();
        total_dodge_rate * 0.01 // Convert to percentage
    }
}
*/

impl ElementalSystem {
    // ElementalContribution struct removed - will be implemented later
    
    /// Get element mastery rank by index
    pub fn get_element_mastery_rank(&self, index: usize) -> Option<ElementMasteryRank> {
        if index < MAX_ELEMENTS {
            Some(self.data.element_mastery_ranks[index])
        } else {
            None
        }
    }
    
    /// Update element mastery rank based on experience
    pub fn update_element_mastery_rank(&mut self, index: usize) -> bool {
        if index < MAX_ELEMENTS {
            let experience = self.data.element_mastery_experience[index];
            self.data.element_mastery_ranks[index] = ElementMasteryRank::from_experience(experience);
            true
        } else {
            false
        }
    }
    
    /// Add experience to element mastery
    pub fn add_element_mastery_experience(&mut self, index: usize, experience: f64) -> bool {
        if index < MAX_ELEMENTS && experience > 0.0 {
            self.data.element_mastery_experience[index] += experience;
            self.update_element_mastery_rank(index);
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
