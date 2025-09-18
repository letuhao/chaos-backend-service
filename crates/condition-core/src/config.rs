//! Configuration loading and parsing

use super::types::*;
use super::error::*;
use std::fs;
use std::path::Path;

/// Load condition configuration from YAML file
pub async fn load_condition_config<P: AsRef<Path>>(path: P) -> ConditionResult<ConditionConfig> {
    let content = fs::read_to_string(path)?;
    let config: ConditionConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

/// Load multiple condition configurations from YAML file
pub async fn load_condition_configs<P: AsRef<Path>>(path: P) -> ConditionResult<Vec<ConditionConfig>> {
    let content = fs::read_to_string(path)?;
    let configs: Vec<ConditionConfig> = serde_yaml::from_str(&content)?;
    Ok(configs)
}

/// Load condition chain configuration from YAML file
pub async fn load_condition_chain_config<P: AsRef<Path>>(path: P) -> ConditionResult<ConditionChainConfig> {
    let content = fs::read_to_string(path)?;
    let config: ConditionChainConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

/// Load condition configuration from YAML string
pub fn parse_condition_config(yaml: &str) -> ConditionResult<ConditionConfig> {
    let config: ConditionConfig = serde_yaml::from_str(yaml)?;
    Ok(config)
}

/// Load multiple condition configurations from YAML string
pub fn parse_condition_configs(yaml: &str) -> ConditionResult<Vec<ConditionConfig>> {
    let configs: Vec<ConditionConfig> = serde_yaml::from_str(yaml)?;
    Ok(configs)
}

/// Load condition chain configuration from YAML string
pub fn parse_condition_chain_config(yaml: &str) -> ConditionResult<ConditionChainConfig> {
    let config: ConditionChainConfig = serde_yaml::from_str(yaml)?;
    Ok(config)
}

/// Validate condition configuration
pub fn validate_condition_config(config: &ConditionConfig) -> ConditionResult<()> {
    if config.condition_id.is_empty() {
        return Err(ConditionError::ConfigError {
            message: "Condition ID cannot be empty".to_string(),
        });
    }

    if config.function_name.is_empty() {
        return Err(ConditionError::ConfigError {
            message: "Function name cannot be empty".to_string(),
        });
    }

    Ok(())
}

/// Validate condition chain configuration
pub fn validate_condition_chain_config(config: &ConditionChainConfig) -> ConditionResult<()> {
    if config.chain_id.is_empty() {
        return Err(ConditionError::ConfigError {
            message: "Chain ID cannot be empty".to_string(),
        });
    }

    if config.conditions.is_empty() {
        return Err(ConditionError::ConfigError {
            message: "Chain must have at least one condition".to_string(),
        });
    }

    for condition in &config.conditions {
        validate_condition_config(condition)?;
    }

    Ok(())
}
