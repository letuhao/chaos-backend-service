//! # YAML Configuration Loader
//! 
//! This module provides YAML configuration loading functionality for element configurations.

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::{ElementCoreResult, ElementCoreError};
use crate::unified_registry::{ElementDefinition, ElementCategory, ElementProperties, DerivedStatConfig, StatusEffectConfig, ElementReferences, ElementAliases};

/// YAML configuration loader for element configurations
/// 
/// This loader handles loading and parsing of YAML configuration files
/// for element definitions, interactions, and other element-related data.
pub struct YamlConfigLoader {
    /// Base configuration directory
    config_dir: PathBuf,
    
    /// Loaded configurations cache
    config_cache: HashMap<String, ElementConfig>,
    
    /// Validation rules
    validation_rules: Vec<ConfigValidationRule>,
}

/// Element configuration loaded from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementConfig {
    /// Configuration version
    pub version: u32,
    
    /// Element definition
    pub element: ElementDefinition,
    
    /// Element interactions
    pub interactions: Vec<ElementInteractionConfig>,
    
    /// Status effects
    pub status_effects: Vec<StatusEffectConfig>,
    
    /// Derived stats configuration
    pub derived_stats: Vec<DerivedStatConfig>,
}

/// Element interaction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementInteractionConfig {
    /// Target element
    pub target_element: String,
    
    /// Interaction type
    pub interaction_type: String,
    
    /// Base multiplier
    pub base_multiplier: f64,
    
    /// Scaling factor
    pub scaling_factor: f64,
    
    /// Special effects
    pub special_effects: Vec<String>,
}

/// Configuration validation rule
pub struct ConfigValidationRule {
    /// Rule name
    pub name: String,
    
    /// Validation function
    pub validator: Box<dyn Fn(&ElementConfig) -> Result<(), String> + Send + Sync>,
}

impl std::fmt::Debug for ConfigValidationRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigValidationRule {{ name: {} }}", self.name)
    }
}

impl Clone for ConfigValidationRule {
    fn clone(&self) -> Self {
        // Note: We can't clone the validator function, so we create a new rule with a default validator
        Self {
            name: self.name.clone(),
            validator: Box::new(|_| Ok(())), // Default validator that always passes
        }
    }
}

impl YamlConfigLoader {
    /// Create a new YAML configuration loader
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            config_dir,
            config_cache: HashMap::new(),
            validation_rules: Vec::new(),
        }
    }
    
    /// Load element configuration from YAML file
    pub fn load_element_config(&mut self, element_id: &str) -> ElementCoreResult<ElementConfig> {
        // Check cache first
        if let Some(cached_config) = self.config_cache.get(element_id) {
            return Ok(cached_config.clone());
        }
        
        // Load from file
        let file_path = self.config_dir.join(format!("{}_element.yaml", element_id));
        
        if !file_path.exists() {
            return Err(ElementCoreError::Config { 
                message: format!("Element configuration file not found: {}", file_path.display())
            });
        }
        
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| ElementCoreError::Io(e))?;
        
        let config: ElementConfig = serde_yaml::from_str(&content)
            .map_err(|e| ElementCoreError::Config { 
                message: format!("Failed to parse YAML: {}", e)
            })?;
        
        // Validate configuration
        self.validate_config(&config)?;
        
        // Cache the configuration
        self.config_cache.insert(element_id.to_string(), config.clone());
        
        Ok(config)
    }
    
    /// Load all element configurations
    pub fn load_all_configs(&mut self) -> ElementCoreResult<HashMap<String, ElementConfig>> {
        let mut configs = HashMap::new();
        
        // Find all YAML files in the config directory
        let entries = std::fs::read_dir(&self.config_dir)
            .map_err(|e| ElementCoreError::Io(e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| ElementCoreError::Io(e))?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    if file_name.ends_with("_element") {
                        let element_id = file_name.replace("_element", "");
                        match self.load_element_config(&element_id) {
                            Ok(config) => {
                                configs.insert(element_id, config);
                            }
                            Err(e) => {
                                eprintln!("Warning: Failed to load config for {}: {}", element_id, e);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(configs)
    }
    
    /// Load interaction configuration
    pub fn load_interaction_config(&self) -> ElementCoreResult<InteractionConfig> {
        let file_path = self.config_dir.join("interaction_config.yaml");
        
        if !file_path.exists() {
            return Err(ElementCoreError::Config { 
                message: format!("Interaction configuration file not found: {}", file_path.display())
            });
        }
        
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| ElementCoreError::Io(e))?;
        
        let config: InteractionConfig = serde_yaml::from_str(&content)
            .map_err(|e| ElementCoreError::Config { 
                message: format!("Failed to parse YAML: {}", e)
            })?;
        
        Ok(config)
    }
    
    /// Load probability configuration
    pub fn load_probability_config(&self) -> ElementCoreResult<ProbabilityConfig> {
        let file_path = self.config_dir.join("probability_config.yaml");
        
        if !file_path.exists() {
            return Err(ElementCoreError::Config { 
                message: format!("Probability configuration file not found: {}", file_path.display())
            });
        }
        
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| ElementCoreError::Io(e))?;
        
        let config: ProbabilityConfig = serde_yaml::from_str(&content)
            .map_err(|e| ElementCoreError::Config { 
                message: format!("Failed to parse YAML: {}", e)
            })?;
        
        Ok(config)
    }
    
    /// Load status pool configuration
    pub fn load_status_pool_config(&self) -> ElementCoreResult<StatusPoolConfig> {
        let file_path = self.config_dir.join("status_pool.yaml");
        
        if !file_path.exists() {
            return Err(ElementCoreError::Config { 
                message: format!("Status pool configuration file not found: {}", file_path.display())
            });
        }
        
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| ElementCoreError::Io(e))?;
        
        let config: StatusPoolConfig = serde_yaml::from_str(&content)
            .map_err(|e| ElementCoreError::Config { 
                message: format!("Failed to parse YAML: {}", e)
            })?;
        
        Ok(config)
    }
    
    /// Add validation rule
    pub fn add_validation_rule(&mut self, rule: ConfigValidationRule) {
        self.validation_rules.push(rule);
    }
    
    /// Validate configuration
    fn validate_config(&self, config: &ElementConfig) -> ElementCoreResult<()> {
        // Basic validation
        if config.version == 0 {
            return Err(ElementCoreError::Validation { 
                message: "Version cannot be 0".to_string()
            });
        }
        
        if config.element.id.is_empty() {
            return Err(ElementCoreError::Validation { 
                message: "Element ID cannot be empty".to_string()
            });
        }
        
        if config.element.name.is_empty() {
            return Err(ElementCoreError::Validation { 
                message: "Element name cannot be empty".to_string()
            });
        }
        
        // Validate element definition
        config.element.validate()?;
        
        // Apply custom validation rules
        for rule in &self.validation_rules {
            (rule.validator)(config)
                .map_err(|e| ElementCoreError::Validation { 
                    message: format!("Validation rule '{}' failed: {}", rule.name, e)
                })?;
        }
        
        Ok(())
    }
    
    /// Clear configuration cache
    pub fn clear_cache(&mut self) {
        self.config_cache.clear();
    }
    
    /// Get cached configuration
    pub fn get_cached_config(&self, element_id: &str) -> Option<&ElementConfig> {
        self.config_cache.get(element_id)
    }
    
    /// Check if configuration is cached
    pub fn is_cached(&self, element_id: &str) -> bool {
        self.config_cache.contains_key(element_id)
    }
    
    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.config_cache.len()
    }
}

/// Interaction configuration loaded from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConfig {
    /// Configuration version
    pub version: u32,
    
    /// Base trigger probabilities for different relationships
    pub relationships: RelationshipConfig,
    
    /// Interaction dynamics
    pub dynamics: InteractionDynamicsConfig,
    
    /// Element pairs
    pub pairs: HashMap<String, ElementPairConfig>,
    
    /// Special effects
    pub effects: Vec<EffectConfig>,
}

/// Relationship configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipConfig {
    /// Same element trigger probability
    pub same: f64,
    
    /// Generating relationship trigger probability
    pub generating: f64,
    
    /// Overcoming relationship trigger probability
    pub overcoming: f64,
    
    /// Neutral relationship trigger probability
    pub neutral: f64,
}

/// Interaction dynamics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionDynamicsConfig {
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

/// Element pair configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementPairConfig {
    /// Generating elements
    pub generating: Vec<String>,
    
    /// Overcoming elements
    pub overcoming: Vec<String>,
    
    /// Neutral elements
    pub neutral: Vec<String>,
}

/// Effect configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectConfig {
    /// Effect identifier
    pub id: String,
    
    /// When to apply the effect
    pub when: EffectCondition,
    
    /// Who to apply the effect to
    pub apply_to: String,
    
    /// Pool ID for the effect
    pub pool_id: String,
    
    /// Dynamics modifications
    pub dynamics_mod: Option<DynamicsModification>,
}

/// Effect condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectCondition {
    /// Attacker element
    pub attacker: String,
    
    /// Defender element
    pub defender: String,
    
    /// Relationship type
    pub relationship: String,
}

/// Dynamics modification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicsModification {
    /// Intensity gain modification
    pub intensity_gain_mod: f64,
    
    /// Intensity damping modification
    pub intensity_damping_mod: f64,
}

/// Probability configuration loaded from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbabilityConfig {
    /// Configuration version
    pub version: u32,
    
    /// Sigmoid configurations
    pub sigmoid: SigmoidConfig,
    
    /// Custom probability functions
    pub custom_functions: HashMap<String, CustomFunctionConfig>,
}

/// Sigmoid configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigmoidConfig {
    /// Default steepness
    pub default_steepness: f64,
    
    /// Default midpoint
    pub default_midpoint: f64,
    
    /// Element-specific configurations
    pub element_configs: HashMap<String, ElementSigmoidConfig>,
}

/// Element-specific sigmoid configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementSigmoidConfig {
    /// Steepness for this element
    pub steepness: f64,
    
    /// Midpoint for this element
    pub midpoint: f64,
    
    /// Scaling factor
    pub scaling_factor: f64,
}

/// Custom function configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFunctionConfig {
    /// Function name
    pub name: String,
    
    /// Function parameters
    pub parameters: HashMap<String, f64>,
    
    /// Function type
    pub function_type: String,
}

/// Status pool configuration loaded from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPoolConfig {
    /// Configuration version
    pub version: u32,
    
    /// Status effect pools
    pub pools: HashMap<String, StatusPool>,
}

/// Status pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPool {
    /// Pool name
    pub name: String,
    
    /// Pool description
    pub description: String,
    
    /// Status effects in this pool
    pub effects: Vec<StatusEffectPoolEntry>,
}

/// Status effect pool entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectPoolEntry {
    /// Effect name
    pub name: String,
    
    /// Effect type
    pub effect_type: String,
    
    /// Base probability
    pub base_probability: f64,
    
    /// Base duration
    pub base_duration: f64,
    
    /// Base intensity
    pub base_intensity: f64,
    
    /// Tick interval
    pub tick_interval: f64,
    
    /// Maximum stacks
    pub max_stacks: u32,
    
    /// Whether effect is stackable
    pub stackable: bool,
    
    /// Whether duration refreshes on re-application
    pub refresh_duration: bool,
    
    /// Effect values
    pub effects: Option<HashMap<String, f64>>,
    
    /// HP heal per tick
    pub hp_heal_per_tick: Option<f64>,
    
    /// Stamina heal per tick
    pub stamina_heal_per_tick: Option<f64>,
    
    /// Dynamics configuration
    pub dynamics: StatusDynamicsConfig,
}

/// Status dynamics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDynamicsConfig {
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

impl Default for YamlConfigLoader {
    fn default() -> Self {
        Self::new(PathBuf::from("configs"))
    }
}

impl ConfigValidationRule {
    /// Create a new validation rule
    pub fn new<F>(name: String, validator: F) -> Self
    where
        F: Fn(&ElementConfig) -> Result<(), String> + Send + Sync + 'static,
    {
        Self {
            name,
            validator: Box::new(validator),
        }
    }
}
