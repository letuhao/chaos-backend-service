//! Registry loader for external configuration files.
//!
//! This module provides functionality to load CapLayerRegistry and CombinerRegistry
//! from YAML/JSON configuration files at runtime.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::Result;
use thiserror::Error;
use serde::{Deserialize, Serialize};

use crate::interfaces::{CapLayerRegistry, CombinerRegistry, MergeRule};
use crate::enums::{AcrossLayerPolicy, Bucket, CapMode};
use crate::types::Caps;
use crate::ActorCoreResult;

/// Errors that can occur during configuration loading.
#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Invalid YAML: {error}")]
    InvalidYaml { error: String },
    
    #[error("Invalid JSON: {error}")]
    InvalidJson { error: String },
    
    #[error("Validation error: {message}")]
    ValidationError { message: String },
    
    #[error("IO error: {error}")]
    IoError { error: String },
}

/// Cap layer configuration structure for YAML/JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapLayerConfig {
    pub name: String,
    pub priority: i64,
    pub caps: Vec<CapConfig>,
}

/// Individual cap configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapConfig {
    pub id: String,
    pub cap_mode: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// Cap layers configuration root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapLayersConfig {
    pub layers: Vec<CapLayerConfig>,
}

/// Combiner rule configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinerRuleConfig {
    pub id: String,
    pub bucket_order: Vec<String>,
    pub clamp: ClampConfig,
}

/// Clamp configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClampConfig {
    pub min: f64,
    pub max: f64,
}

/// Combiner configuration root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinerConfig {
    pub rules: Vec<CombinerRuleConfig>,
}

/// Load cap layers configuration from a file.
/// 
/// Tries YAML first, then falls back to JSON if YAML fails.
pub fn load_cap_layers<P: AsRef<Path>>(path: P) -> Result<CapLayerRegistryImpl, LoaderError> {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(LoaderError::FileNotFound {
            path: path.to_string_lossy().to_string(),
        });
    }
    
    let content = std::fs::read_to_string(path)
        .map_err(|e| LoaderError::IoError {
            error: e.to_string(),
        })?;
    
    // Try YAML first
    let config = match serde_yaml::from_str::<CapLayersConfig>(&content) {
        Ok(config) => config,
        Err(yaml_err) => {
            // Fall back to JSON
            serde_json::from_str::<CapLayersConfig>(&content)
                .map_err(|e| LoaderError::InvalidJson {
                    error: format!("YAML failed: {}, JSON failed: {}", yaml_err, e),
                })?
        }
    };
    
    validate_cap_layers_config(&config)?;
    convert_cap_layers_config(config)
}

/// Load combiner configuration from a file.
/// 
/// Tries YAML first, then falls back to JSON if YAML fails.
pub fn load_combiner<P: AsRef<Path>>(path: P) -> Result<CombinerRegistryImpl, LoaderError> {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(LoaderError::FileNotFound {
            path: path.to_string_lossy().to_string(),
        });
    }
    
    let content = std::fs::read_to_string(path)
        .map_err(|e| LoaderError::IoError {
            error: e.to_string(),
        })?;
    
    // Try YAML first
    let config = match serde_yaml::from_str::<CombinerConfig>(&content) {
        Ok(config) => config,
        Err(yaml_err) => {
            // Fall back to JSON
            serde_json::from_str::<CombinerConfig>(&content)
                .map_err(|e| LoaderError::InvalidJson {
                    error: format!("YAML failed: {}, JSON failed: {}", yaml_err, e),
                })?
        }
    };
    
    validate_combiner_config(&config)?;
    convert_combiner_config(config)
}

/// Load both cap layers and combiner configurations from a directory.
pub fn load_all<P: AsRef<Path>>(cfg_dir: P) -> Result<(CapLayerRegistryImpl, CombinerRegistryImpl), LoaderError> {
    // Resolve directory order: env override -> provided -> default ./configs
    let resolved_dir: PathBuf = if let Ok(env_dir) = std::env::var("ACTOR_CORE_CONFIG_DIR") {
        PathBuf::from(env_dir)
    } else {
        cfg_dir.as_ref().to_path_buf()
    };

    let cap_layers_path_yaml = resolved_dir.join("cap_layers.yaml");
    let cap_layers_path_json = resolved_dir.join("cap_layers.json");
    let combiner_path_yaml = resolved_dir.join("combiner.yaml");
    let combiner_path_json = resolved_dir.join("combiner.json");

    // Try YAML first, then JSON for each registry
    let cap_layers = if cap_layers_path_yaml.exists() {
        load_cap_layers(&cap_layers_path_yaml)?
    } else {
        load_cap_layers(&cap_layers_path_json)?
    };

    let combiner = if combiner_path_yaml.exists() {
        load_combiner(&combiner_path_yaml)?
    } else {
        load_combiner(&combiner_path_json)?
    };

    Ok((cap_layers, combiner))
}

/// Validate cap layers configuration.
fn validate_cap_layers_config(config: &CapLayersConfig) -> Result<(), LoaderError> {
    if config.layers.is_empty() {
        return Err(LoaderError::ValidationError {
            message: "No layers defined".to_string(),
        });
    }
    
    let mut layer_names = std::collections::HashSet::new();
    let mut cap_ids = std::collections::HashSet::new();
    
    for layer in &config.layers {
        // Check for duplicate layer names
        if !layer_names.insert(&layer.name) {
            return Err(LoaderError::ValidationError {
                message: format!("Duplicate layer name: {}", layer.name),
            });
        }
        
        // Check for empty caps
        if layer.caps.is_empty() {
            return Err(LoaderError::ValidationError {
                message: format!("Layer '{}' has no caps defined", layer.name),
            });
        }
        
        for cap in &layer.caps {
            // Check for duplicate cap IDs within the same layer
            if !cap_ids.insert((&layer.name, &cap.id)) {
                return Err(LoaderError::ValidationError {
                    message: format!("Duplicate cap ID '{}' in layer '{}'", cap.id, layer.name),
                });
            }
            
            // Validate cap mode
            if !is_valid_cap_mode(&cap.cap_mode) {
                return Err(LoaderError::ValidationError {
                    message: format!("Invalid cap mode '{}' for cap '{}'", cap.cap_mode, cap.id),
                });
            }
            
            // Validate min/max values
            if let (Some(min), Some(max)) = (cap.min, cap.max) {
                if min > max {
                    return Err(LoaderError::ValidationError {
                        message: format!("Invalid cap range for '{}': min ({}) > max ({})", cap.id, min, max),
                    });
                }
            }
        }
    }
    
    Ok(())
}

/// Validate combiner configuration.
fn validate_combiner_config(config: &CombinerConfig) -> Result<(), LoaderError> {
    if config.rules.is_empty() {
        return Err(LoaderError::ValidationError {
            message: "No rules defined".to_string(),
        });
    }
    
    let mut rule_ids = std::collections::HashSet::new();
    
    for rule in &config.rules {
        // Check for duplicate rule IDs
        if !rule_ids.insert(&rule.id) {
            return Err(LoaderError::ValidationError {
                message: format!("Duplicate rule ID: {}", rule.id),
            });
        }
        
        // Validate bucket order
        if rule.bucket_order.is_empty() {
            return Err(LoaderError::ValidationError {
                message: format!("Empty bucket order for rule '{}'", rule.id),
            });
        }
        
        for bucket in &rule.bucket_order {
            if !is_valid_bucket_type(bucket) {
                return Err(LoaderError::ValidationError {
                    message: format!("Invalid bucket type '{}' in rule '{}'", bucket, rule.id),
                });
            }
        }
        
        // Validate clamp values
        if rule.clamp.min > rule.clamp.max {
            return Err(LoaderError::ValidationError {
                message: format!("Invalid clamp range for rule '{}': min ({}) > max ({})", 
                    rule.id, rule.clamp.min, rule.clamp.max),
            });
        }
    }
    
    Ok(())
}

/// Check if a cap mode string is valid.
fn is_valid_cap_mode(mode: &str) -> bool {
    matches!(mode, "BASELINE" | "ADDITIVE" | "HARD_MIN" | "HARD_MAX" | "OVERRIDE")
}

/// Check if a bucket type string is valid.
fn is_valid_bucket_type(bucket: &str) -> bool {
    let core_buckets = matches!(bucket, "FLAT" | "MULT" | "POST_ADD" | "OVERRIDE");
    #[cfg(feature = "extra_buckets")]
    let extra_buckets = matches!(bucket, "EXPONENTIAL" | "LOGARITHMIC" | "CONDITIONAL");
    #[cfg(not(feature = "extra_buckets"))]
    let extra_buckets = false;
    
    core_buckets || extra_buckets
}

/// Convert cap layers configuration to registry implementation.
fn convert_cap_layers_config(config: CapLayersConfig) -> Result<CapLayerRegistryImpl, LoaderError> {
    let mut layers = HashMap::new();
    
    for layer_config in config.layers {
        let mut caps = HashMap::new();
        
        for cap_config in layer_config.caps {
            let cap_mode = match cap_config.cap_mode.as_str() {
                "BASELINE" => CapMode::Baseline,
                "ADDITIVE" => CapMode::Additive,
                "HARD_MIN" => CapMode::HardMin,
                "HARD_MAX" => CapMode::HardMax,
                "OVERRIDE" => CapMode::Override,
                _ => return Err(LoaderError::ValidationError {
                    message: format!("Invalid cap mode: {}", cap_config.cap_mode),
                }),
            };
            
            let caps_obj = Caps::new(
                cap_config.min.unwrap_or(f64::NEG_INFINITY),
                cap_config.max.unwrap_or(f64::INFINITY),
            );
            
            caps.insert(cap_config.id, (cap_mode, caps_obj));
        }
        
        layers.insert(layer_config.name, (layer_config.priority, caps));
    }
    
    Ok(CapLayerRegistryImpl::new(layers))
}

/// Convert combiner configuration to registry implementation.
fn convert_combiner_config(config: CombinerConfig) -> Result<CombinerRegistryImpl, LoaderError> {
    let mut rules = HashMap::new();
    
    for rule_config in config.rules {
        let bucket_order: Result<Vec<crate::enums::Bucket>, LoaderError> = rule_config.bucket_order
            .into_iter()
            .map(|bucket| match bucket.as_str() {
                "FLAT" => Ok(crate::enums::Bucket::Flat),
                "MULT" => Ok(crate::enums::Bucket::Mult),
                "POST_ADD" => Ok(crate::enums::Bucket::PostAdd),
                "OVERRIDE" => Ok(crate::enums::Bucket::Override),
                #[cfg(feature = "extra_buckets")]
                "EXPONENTIAL" => Ok(crate::enums::Bucket::Exponential),
                #[cfg(feature = "extra_buckets")]
                "LOGARITHMIC" => Ok(crate::enums::Bucket::Logarithmic),
                #[cfg(feature = "extra_buckets")]
                "CONDITIONAL" => Ok(crate::enums::Bucket::Conditional),
                _ => Err(LoaderError::ValidationError {
                    message: format!("Invalid bucket type: {}", bucket),
                }),
            })
            .collect();
        
        let bucket_order = bucket_order?;
        
        let clamp = Caps::new(rule_config.clamp.min, rule_config.clamp.max);
        
        rules.insert(rule_config.id, (bucket_order, clamp));
    }
    
    Ok(CombinerRegistryImpl::new(rules))
}

/// Simple implementation of CapLayerRegistry for loaded configurations.
#[derive(Debug, Clone)]
pub struct CapLayerRegistryImpl {
    layers: HashMap<String, (i64, HashMap<String, (CapMode, Caps)>)>,
}

impl CapLayerRegistryImpl {
    pub fn new(layers: HashMap<String, (i64, HashMap<String, (CapMode, Caps)>)>) -> Self {
        Self { layers }
    }
}

impl CapLayerRegistry for CapLayerRegistryImpl {
    fn get_layer_order(&self) -> Vec<String> {
        let mut layers: Vec<_> = self.layers.iter().collect();
        layers.sort_by_key(|(_, (priority, _))| *priority);
        layers.into_iter().map(|(name, _)| name.clone()).collect()
    }
    
    fn set_layer_order(&self, _order: Vec<String>) -> ActorCoreResult<()> {
        // This is a read-only implementation for loaded configs
        Err(crate::ActorCoreError::ConfigurationError(
            "Cannot modify layer order in loaded configuration".to_string()
        ))
    }
    
    fn get_across_layer_policy(&self) -> AcrossLayerPolicy {
        AcrossLayerPolicy::Intersect
    }
    
    fn set_across_layer_policy(&self, _policy: AcrossLayerPolicy) {
        // This is a read-only implementation for loaded configs
    }
    
    fn validate(&self) -> ActorCoreResult<()> {
        // Validation is already done during loading
        Ok(())
    }
}

/// Simple implementation of CombinerRegistry for loaded configurations.
#[derive(Debug, Clone)]
pub struct CombinerRegistryImpl {
    rules: HashMap<String, (Vec<crate::enums::Bucket>, Caps)>,
}

impl CombinerRegistryImpl {
    pub fn new(rules: HashMap<String, (Vec<crate::enums::Bucket>, Caps)>) -> Self {
        Self { rules }
    }
}

impl CombinerRegistry for CombinerRegistryImpl {
    fn get_rule(&self, dimension: &str) -> Option<MergeRule> {
        self.rules.get(dimension).map(|(_bucket_order, clamp)| {
            MergeRule {
                use_pipeline: true,
                operator: crate::enums::Operator::Sum,
                clamp_default: Some(clamp.clone()),
            }
        })
    }
    
    fn set_rule(&self, _dimension: &str, _rule: MergeRule) -> ActorCoreResult<()> {
        // This is a read-only implementation for loaded configs
        Err(crate::ActorCoreError::ConfigurationError(
            "Cannot modify rules in loaded configuration".to_string()
        ))
    }
    
    fn validate(&self) -> ActorCoreResult<()> {
        // Validation is already done during loading
        Ok(())
    }
}