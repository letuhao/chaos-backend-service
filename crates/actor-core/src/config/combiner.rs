//! Configuration combiner implementation for the Configuration Hub system

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::info;

use crate::config::types::*;
use crate::ActorCoreResult;

/// Combiner for merging configuration values
#[async_trait]
pub trait ConfigurationCombiner: Send + Sync {
    /// Get merge rule for a specific configuration key
    async fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule>;
    
    /// Set merge rule for a specific configuration key
    async fn set_merge_rule(&self, category: &str, key: &str, rule: ConfigurationMergeRule) -> ActorCoreResult<()>;
    
    /// Merge configuration values from multiple providers
    async fn merge_values(&self, values: Vec<ConfigurationValue>, rule: &ConfigurationMergeRule) -> ActorCoreResult<ConfigurationValue>;
    
    /// Validate merged configuration
    async fn validate_merged_config(&self, config: &ConfigurationValue) -> ActorCoreResult<()>;
    
    /// Get combiner metrics
    async fn get_metrics(&self) -> ConfigurationCombinerMetrics;
}

/// Configuration combiner implementation
pub struct ConfigurationCombinerImpl {
    merge_rules: Arc<RwLock<HashMap<String, ConfigurationMergeRule>>>,
    metrics: Arc<RwLock<ConfigurationCombinerMetrics>>,
}

impl ConfigurationCombinerImpl {
    pub fn new() -> Self {
        Self {
            merge_rules: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(ConfigurationCombinerMetrics::default())),
        }
    }

    /// Load default merge rules for common configuration categories
    /// 
    /// This method loads merge rules from configuration files instead of hardcoding them.
    /// If configuration loading fails, it falls back to reasonable defaults.
    pub fn load_default_rules(&self) -> ActorCoreResult<()> {
        let mut merge_rules = self.merge_rules.write();
        
        // Try to load rules from configuration first
        match self.load_rules_from_config() {
            Ok(config_rules) => {
                for (category, rule) in config_rules {
                    merge_rules.insert(category, rule);
                }
            },
            Err(e) => {
                tracing::warn!("Failed to load merge rules from configuration: {}. Using fallback defaults.", e);
                self.load_fallback_rules(&mut merge_rules);
            }
        }
        
        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.rule_count = merge_rules.len();
        
        Ok(())
    }

    /// Load merge rules from configuration files
    fn load_rules_from_config(&self) -> ActorCoreResult<Vec<(String, ConfigurationMergeRule)>> {
        // TODO: Implement configuration loading from merge_rules.yaml
        // This should load from configs/merge_rules.yaml
        Err(crate::ActorCoreError::ConfigurationError(
            "Configuration loading not yet implemented".to_string()
        ))
    }

    /// Load fallback rules when configuration is not available
    fn load_fallback_rules(&self, merge_rules: &mut HashMap<String, ConfigurationMergeRule>) {
        let fallback_rules = [
            ("element_affinities", ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Sum,
                use_pipeline: true,
                default_value: Some(serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap_or(serde_json::Number::from(0)))),
                validation_rules: vec![],
            }),
            ("element_interactions", ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override,
                use_pipeline: false,
                default_value: None,
                validation_rules: vec![],
            }),
            ("primary_stats", ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Sum,
                use_pipeline: true,
                default_value: Some(serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap_or(serde_json::Number::from(0)))),
                validation_rules: vec![],
            }),
            ("derived_formulas", ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override,
                use_pipeline: false,
                default_value: None,
                validation_rules: vec![],
            }),
            ("performance", ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Min, // Use minimum for safety
                use_pipeline: true,
                default_value: Some(serde_json::Value::Number(serde_json::Number::from(1000000))),
                validation_rules: vec![],
            }),
        ];
        
        for (category, rule) in fallback_rules {
            merge_rules.insert(category.to_string(), rule);
        }
    }
}

#[async_trait]
impl ConfigurationCombiner for ConfigurationCombinerImpl {
    async fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule> {
        let merge_rules = self.merge_rules.read();
        
        // Try exact match first
        if let Some(rule) = merge_rules.get(&format!("{}:{}", category, key)) {
            return Some(rule.clone());
        }
        
        // Try category-level rule
        if let Some(rule) = merge_rules.get(category) {
            return Some(rule.clone());
        }
        
        // Try default rule
        merge_rules.get("default").cloned()
    }

    async fn set_merge_rule(&self, category: &str, key: &str, rule: ConfigurationMergeRule) -> ActorCoreResult<()> {
        if category.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Category cannot be empty".to_string()
            ));
        }
        
        let mut merge_rules = self.merge_rules.write();
        merge_rules.insert(format!("{}:{}", category, key), rule);
        
        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.rule_count = merge_rules.len();
        metrics.merge_count += 1;
        
        info!("Set merge rule for category: {}, key: {}", category, key);
        Ok(())
    }

    async fn merge_values(&self, values: Vec<ConfigurationValue>, rule: &ConfigurationMergeRule) -> ActorCoreResult<ConfigurationValue> {
        if values.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Cannot merge empty values".to_string()
            ));
        }

        if values.len() == 1 {
            return Ok(values[0].clone());
        }

        // Sort by priority (higher priority first)
        let mut sorted_values = values;
        sorted_values.sort_by(|a, b| b.priority.cmp(&a.priority));

        let result = match rule.strategy {
            ConfigurationMergeStrategy::Override => {
                // Use highest priority value
                sorted_values[0].clone()
            },
            ConfigurationMergeStrategy::Sum => {
                // Sum all values
                let mut sum_value = sorted_values[0].value.clone();
                for value in sorted_values.iter().skip(1) {
                    if let (Some(sum_num), Some(value_num)) = (sum_value.as_f64(), value.value.as_f64()) {
                        sum_value = serde_json::Value::Number(serde_json::Number::from_f64(sum_num + value_num).unwrap_or(serde_json::Number::from(0)));
                    }
                }
                ConfigurationValue {
                    value: sum_value,
                    value_type: sorted_values[0].value_type.clone(),
                    source_provider: "merged".to_string(),
                    priority: sorted_values[0].priority,
                    timestamp: chrono::Utc::now(),
                    can_override: true,
                    can_merge: true,
                }
            },
            ConfigurationMergeStrategy::Max => {
                // Use maximum value
                let mut max_value = sorted_values[0].clone();
                for value in sorted_values.iter().skip(1) {
                    if let (Some(max_num), Some(value_num)) = (max_value.value.as_f64(), value.value.as_f64()) {
                        if value_num > max_num {
                            max_value = value.clone();
                        }
                    }
                }
                max_value
            },
            ConfigurationMergeStrategy::Min => {
                // Use minimum value
                let mut min_value = sorted_values[0].clone();
                for value in sorted_values.iter().skip(1) {
                    if let (Some(min_num), Some(value_num)) = (min_value.value.as_f64(), value.value.as_f64()) {
                        if value_num < min_num {
                            min_value = value.clone();
                        }
                    }
                }
                min_value
            },
            ConfigurationMergeStrategy::Average => {
                // Calculate average value
                let mut sum = 0.0;
                let mut count = 0;
                for value in &sorted_values {
                    if let Some(num) = value.value.as_f64() {
                        sum += num;
                        count += 1;
                    }
                }
                let average = if count > 0 { sum / count as f64 } else { 0.0 };
                ConfigurationValue {
                    value: serde_json::Value::Number(serde_json::Number::from_f64(average).unwrap_or(serde_json::Number::from(0))),
                    value_type: sorted_values[0].value_type.clone(),
                    source_provider: "merged".to_string(),
                    priority: sorted_values[0].priority,
                    timestamp: chrono::Utc::now(),
                    can_override: true,
                    can_merge: true,
                }
            },
            ConfigurationMergeStrategy::Multiply => {
                // Multiply all values
                let mut product = 1.0;
                for value in &sorted_values {
                    if let Some(num) = value.value.as_f64() {
                        product *= num;
                    }
                }
                ConfigurationValue {
                    value: serde_json::Value::Number(serde_json::Number::from_f64(product).unwrap_or(serde_json::Number::from(0))),
                    value_type: sorted_values[0].value_type.clone(),
                    source_provider: "merged".to_string(),
                    priority: sorted_values[0].priority,
                    timestamp: chrono::Utc::now(),
                    can_override: true,
                    can_merge: true,
                }
            },
            ConfigurationMergeStrategy::Concat => {
                // Concatenate string values
                let mut result_string = String::new();
                for value in &sorted_values {
                    if let Some(s) = value.value.as_str() {
                        result_string.push_str(s);
                    }
                }
                ConfigurationValue {
                    value: serde_json::Value::String(result_string),
                    value_type: ConfigurationValueType::String,
                    source_provider: "merged".to_string(),
                    priority: sorted_values[0].priority,
                    timestamp: chrono::Utc::now(),
                    can_override: true,
                    can_merge: true,
                }
            },
            _ => {
                // Default to override
                sorted_values[0].clone()
            }
        };

        // Validate merged result
        if !rule.validation_rules.is_empty() {
            for validation_rule in &rule.validation_rules {
                if let Err(e) = validation_rule.validate(&result) {
                    return Err(crate::ActorCoreError::ConfigurationError(
                        format!("Validation failed for merged value: {}", e)
                    ));
                }
            }
        }

        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.merge_count += 1;
        metrics.validation_count += 1;

        Ok(result)
    }

    async fn validate_merged_config(&self, config: &ConfigurationValue) -> ActorCoreResult<()> {
        // Basic validation
        if config.provider_id().is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Configuration value must have a provider ID".to_string()
            ));
        }

        if config.priority() < 0 {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Configuration value priority must be non-negative".to_string()
            ));
        }

        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.validation_count += 1;

        Ok(())
    }

    async fn get_metrics(&self) -> ConfigurationCombinerMetrics {
        let metrics = self.metrics.read();
        metrics.clone()
    }
}
