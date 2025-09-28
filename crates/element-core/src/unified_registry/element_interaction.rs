//! # Element Interaction
//! 
//! This module defines the element interaction system for tương sinh tương khắc mechanics.

use serde::{Deserialize, Serialize};

/// Element interaction definition
/// 
/// This struct defines how two elements interact with each other,
/// including the interaction type, multiplier, and special effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementInteraction {
    /// Interaction identifier
    pub id: String,
    
    /// Source element
    pub source_element: String,
    
    /// Target element
    pub target_element: String,
    
    /// Interaction type
    pub interaction_type: InteractionType,
    
    /// Base multiplier
    pub base_multiplier: f64,
    
    /// Scaling factor
    pub scaling_factor: f64,
    
    /// Maximum multiplier
    pub max_multiplier: f64,
    
    /// Minimum multiplier
    pub min_multiplier: f64,
    
    /// Special effects
    pub special_effects: Vec<String>,
    
    /// Conditions
    pub conditions: Vec<String>,
    
    /// Interaction description
    pub description: String,
    
    /// Interaction lore
    pub lore: Option<String>,
}

impl ElementInteraction {
    /// Validate element interaction
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Interaction ID cannot be empty".to_string());
        }
        
        if self.source_element.is_empty() {
            return Err("Source element cannot be empty".to_string());
        }
        
        if self.target_element.is_empty() {
            return Err("Target element cannot be empty".to_string());
        }
        
        if self.base_multiplier < 0.0 {
            return Err("Base multiplier cannot be negative".to_string());
        }
        
        if self.scaling_factor < 0.0 {
            return Err("Scaling factor cannot be negative".to_string());
        }
        
        if self.max_multiplier < self.min_multiplier {
            return Err("Max multiplier cannot be less than min multiplier".to_string());
        }
        
        Ok(())
    }
}

/// Element interaction types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionType {
    /// Generating relationship (tương sinh) - Fire generates Earth
    Generating,
    
    /// Overcoming relationship (tương khắc) - Fire overcomes Metal
    Overcoming,
    
    /// Same element - Fire vs Fire
    Same,
    
    /// Neutral relationship - Fire vs Water
    Neutral,
    
    /// Opposite relationship - Fire vs Ice
    Opposite,
    
    /// Special interaction - unique combinations
    Special,
}

/// Element interaction dynamics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionDynamics {
    /// Trigger scale for mastery difference
    pub trigger_scale: f64,
    
    /// Sigmoid steepness
    pub steepness: f64,
    
    /// Intensity gain rate
    pub intensity_gain: f64,
    
    /// Intensity damping rate
    pub intensity_damping: f64,
    
    /// Decay rate
    pub decay_rate: f64,
    
    /// Refractory gain
    pub refractory_gain: f64,
    
    /// Refractory decay
    pub refractory_decay: f64,
}

/// Element interaction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionResult {
    /// Final multiplier
    pub multiplier: f64,
    
    /// Trigger probability
    pub trigger_probability: f64,
    
    /// Special effects applied
    pub special_effects: Vec<String>,
    
    /// Interaction intensity
    pub intensity: f64,
    
    /// Interaction duration
    pub duration: f64,
    
    /// Success status
    pub success: bool,
    
    /// Error message (if any)
    pub error: Option<String>,
}

impl ElementInteraction {
    /// Create a new element interaction
    pub fn new(
        id: String,
        source_element: String,
        target_element: String,
        interaction_type: InteractionType,
    ) -> Self {
        Self {
            id,
            source_element,
            target_element,
            interaction_type,
            base_multiplier: 1.0,
            scaling_factor: 1.0,
            max_multiplier: 2.0,
            min_multiplier: 0.5,
            special_effects: Vec::new(),
            conditions: Vec::new(),
            description: String::new(),
            lore: None,
        }
    }
    
    /// Add a special effect
    pub fn add_special_effect(&mut self, effect: String) {
        if !self.special_effects.contains(&effect) {
            self.special_effects.push(effect);
        }
    }
    
    /// Add a condition
    pub fn add_condition(&mut self, condition: String) {
        if !self.conditions.contains(&condition) {
            self.conditions.push(condition);
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    
    /// Set lore
    pub fn set_lore(&mut self, lore: String) {
        self.lore = Some(lore);
    }
    
    /// Calculate interaction multiplier based on mastery levels
    pub fn calculate_multiplier(&self, source_mastery: f64, target_mastery: f64) -> f64 {
        let mastery_diff = source_mastery - target_mastery;
        let multiplier = self.base_multiplier + (mastery_diff * self.scaling_factor);
        multiplier.max(self.min_multiplier).min(self.max_multiplier)
    }
    
    /// Calculate trigger probability based on mastery difference
    pub fn calculate_trigger_probability(
        &self,
        source_mastery: f64,
        target_mastery: f64,
        dynamics: &InteractionDynamics,
    ) -> f64 {
        let base_trigger = match self.interaction_type {
            InteractionType::Same => 0.0,
            InteractionType::Generating => 0.3,
            InteractionType::Overcoming => 0.8,
            InteractionType::Neutral => 0.1,
            InteractionType::Opposite => 0.5,
            InteractionType::Special => 0.6,
        };
        
        let mastery_diff = source_mastery - target_mastery;
        let normalized_diff = mastery_diff / dynamics.trigger_scale;
        let sigmoid_value = sigmoid(normalized_diff, dynamics.steepness);
        
        (base_trigger + sigmoid_value).clamp(0.0, 1.0)
    }
    
    /// Check if conditions are met
    pub fn check_conditions(&self, source_mastery: f64, target_mastery: f64) -> bool {
        // TODO: Implement condition checking logic
        // For now, always return true
        true
    }
    
    /// Get interaction type as string
    pub fn get_interaction_type_string(&self) -> &'static str {
        match self.interaction_type {
            InteractionType::Generating => "generating",
            InteractionType::Overcoming => "overcoming",
            InteractionType::Same => "same",
            InteractionType::Neutral => "neutral",
            InteractionType::Opposite => "opposite",
            InteractionType::Special => "special",
        }
    }
    
    /// Check if this is a generating interaction
    pub fn is_generating(&self) -> bool {
        matches!(self.interaction_type, InteractionType::Generating)
    }
    
    /// Check if this is an overcoming interaction
    pub fn is_overcoming(&self) -> bool {
        matches!(self.interaction_type, InteractionType::Overcoming)
    }
    
    /// Check if this is a same element interaction
    pub fn is_same(&self) -> bool {
        matches!(self.interaction_type, InteractionType::Same)
    }
    
    /// Check if this is a neutral interaction
    pub fn is_neutral(&self) -> bool {
        matches!(self.interaction_type, InteractionType::Neutral)
    }
    
    /// Check if this is an opposite interaction
    pub fn is_opposite(&self) -> bool {
        matches!(self.interaction_type, InteractionType::Opposite)
    }
    
    /// Check if this is a special interaction
    pub fn is_special(&self) -> bool {
        matches!(self.interaction_type, InteractionType::Special)
    }
    
}

impl InteractionDynamics {
    /// Create default interaction dynamics
    pub fn default() -> Self {
        Self {
            trigger_scale: 50.0,
            steepness: 1.0,
            intensity_gain: 0.02,
            intensity_damping: 0.01,
            decay_rate: 0.05,
            refractory_gain: 0.5,
            refractory_decay: 0.1,
        }
    }
    
    /// Create custom interaction dynamics
    pub fn new(
        trigger_scale: f64,
        steepness: f64,
        intensity_gain: f64,
        intensity_damping: f64,
        decay_rate: f64,
        refractory_gain: f64,
        refractory_decay: f64,
    ) -> Self {
        Self {
            trigger_scale,
            steepness,
            intensity_gain,
            intensity_damping,
            decay_rate,
            refractory_gain,
            refractory_decay,
        }
    }
    
    /// Validate dynamics parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.trigger_scale <= 0.0 {
            return Err("Trigger scale must be positive".to_string());
        }
        
        if self.steepness <= 0.0 {
            return Err("Steepness must be positive".to_string());
        }
        
        if self.intensity_gain < 0.0 {
            return Err("Intensity gain cannot be negative".to_string());
        }
        
        if self.intensity_damping < 0.0 {
            return Err("Intensity damping cannot be negative".to_string());
        }
        
        if self.decay_rate < 0.0 {
            return Err("Decay rate cannot be negative".to_string());
        }
        
        if self.refractory_gain < 0.0 {
            return Err("Refractory gain cannot be negative".to_string());
        }
        
        if self.refractory_decay < 0.0 {
            return Err("Refractory decay cannot be negative".to_string());
        }
        
        Ok(())
    }
}

impl InteractionResult {
    /// Create a successful interaction result
    pub fn success(
        multiplier: f64,
        trigger_probability: f64,
        special_effects: Vec<String>,
        intensity: f64,
        duration: f64,
    ) -> Self {
        Self {
            multiplier,
            trigger_probability,
            special_effects,
            intensity,
            duration,
            success: true,
            error: None,
        }
    }
    
    /// Create a failed interaction result
    pub fn failure(error: String) -> Self {
        Self {
            multiplier: 1.0,
            trigger_probability: 0.0,
            special_effects: Vec::new(),
            intensity: 0.0,
            duration: 0.0,
            success: false,
            error: Some(error),
        }
    }
    
    /// Check if the interaction was successful
    pub fn is_success(&self) -> bool {
        self.success
    }
    
    /// Check if the interaction failed
    pub fn is_failure(&self) -> bool {
        !self.success
    }
    
    /// Get the error message if any
    pub fn get_error(&self) -> Option<&String> {
        self.error.as_ref()
    }
    
    /// Get the final damage multiplier
    pub fn get_damage_multiplier(&self) -> f64 {
        if self.success {
            self.multiplier
        } else {
            1.0
        }
    }
    
    /// Get the trigger probability
    pub fn get_trigger_probability(&self) -> f64 {
        if self.success {
            self.trigger_probability
        } else {
            0.0
        }
    }
}

impl Default for InteractionDynamics {
    fn default() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for InteractionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InteractionType::Generating => write!(f, "Generating (Tương Sinh)"),
            InteractionType::Overcoming => write!(f, "Overcoming (Tương Khắc)"),
            InteractionType::Same => write!(f, "Same Element"),
            InteractionType::Neutral => write!(f, "Neutral"),
            InteractionType::Opposite => write!(f, "Opposite"),
            InteractionType::Special => write!(f, "Special"),
        }
    }
}

/// Sigmoid function for smooth transitions
fn sigmoid(x: f64, steepness: f64) -> f64 {
    1.0 / (1.0 + (-steepness * x).exp())
}

/// Create a key for element interaction lookup
pub fn create_interaction_key(source: &str, target: &str) -> String {
    format!("{}:{}", source, target)
}

/// Parse interaction key
pub fn parse_interaction_key(key: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = key.split(':').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid interaction key format: {}", key));
    }
    
    Ok((parts[0].to_string(), parts[1].to_string()))
}
