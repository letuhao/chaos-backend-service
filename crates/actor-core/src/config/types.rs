//! Configuration types and data structures for the Configuration Hub system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Configuration value with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationValue {
    /// The actual value
    pub value: serde_json::Value,
    /// Data type of the value
    pub value_type: ConfigurationValueType,
    /// Source provider ID
    pub source_provider: String,
    /// Priority of this value
    pub priority: i64,
    /// Timestamp when this value was set
    pub timestamp: DateTime<Utc>,
    /// Whether this value can be overridden
    pub can_override: bool,
    /// Whether this value can be merged
    pub can_merge: bool,
}

impl ConfigurationValue {
    /// Create a new configuration value
    pub fn new(
        value: serde_json::Value,
        value_type: ConfigurationValueType,
        source_provider: String,
        priority: i64,
    ) -> Self {
        Self {
            value,
            value_type,
            source_provider,
            priority,
            timestamp: Utc::now(),
            can_override: true,
            can_merge: true,
        }
    }

    /// Get the provider ID
    pub fn provider_id(&self) -> &str {
        &self.source_provider
    }

    /// Get the priority
    pub fn priority(&self) -> i64 {
        self.priority
    }
}

/// Configuration value types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigurationValueType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
    Duration,
    Size,
    Percentage,
}

/// Merge rule for configuration values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationMergeRule {
    /// Merge strategy to use
    pub strategy: ConfigurationMergeStrategy,
    /// Whether to use pipeline processing
    pub use_pipeline: bool,
    /// Default value if no providers have this config
    pub default_value: Option<serde_json::Value>,
    /// Validation rules for the merged value
    pub validation_rules: Vec<ConfigurationValidationRule>,
}

/// Merge strategies for configuration values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfigurationMergeStrategy {
    /// Use the highest priority value (override)
    Override,
    /// Use the lowest priority value (baseline)
    Baseline,
    /// Sum all values (for numeric values)
    Sum,
    /// Take the maximum value
    Max,
    /// Take the minimum value
    Min,
    /// Calculate average (for numeric values)
    Average,
    /// Multiply all values (for numeric values)
    Multiply,
    /// Intersect ranges (for range values)
    Intersect,
    /// Merge objects/arrays
    Merge,
    /// Concatenate strings
    Concat,
}

/// Validation rule for configuration values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationValidationRule {
    /// Rule name
    pub name: String,
    /// Validation function name
    pub validator: String,
    /// Validation parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

impl ConfigurationValidationRule {
    /// Validate a configuration value
    pub fn validate(&self, value: &ConfigurationValue) -> Result<(), String> {
        match self.validator.as_str() {
            "range" => {
                if let Some(min) = self.parameters.get("min").and_then(|v| v.as_f64()) {
                    if let Some(val) = value.value.as_f64() {
                        if val < min {
                            return Err(format!("Value {} is below minimum {}", val, min));
                        }
                    }
                }
                if let Some(max) = self.parameters.get("max").and_then(|v| v.as_f64()) {
                    if let Some(val) = value.value.as_f64() {
                        if val > max {
                            return Err(format!("Value {} is above maximum {}", val, max));
                        }
                    }
                }
                Ok(())
            },
            "non_negative" => {
                if let Some(val) = value.value.as_f64() {
                    if val < 0.0 {
                        return Err(format!("Value {} must be non-negative", val));
                    }
                }
                Ok(())
            },
            "positive" => {
                if let Some(val) = value.value.as_f64() {
                    if val <= 0.0 {
                        return Err(format!("Value {} must be positive", val));
                    }
                }
                Ok(())
            },
            _ => Ok(()), // Unknown validator, skip validation
        }
    }
}

/// Configuration registry metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationRegistryMetrics {
    pub registered_count: usize,
    pub registration_attempts: u64,
    pub unregistration_attempts: u64,
    pub lookup_attempts: u64,
    pub validation_attempts: u64,
}

impl Default for ConfigurationRegistryMetrics {
    fn default() -> Self {
        Self {
            registered_count: 0,
            registration_attempts: 0,
            unregistration_attempts: 0,
            lookup_attempts: 0,
            validation_attempts: 0,
        }
    }
}

/// Configuration combiner metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationCombinerMetrics {
    pub rule_count: usize,
    pub merge_count: u64,
    pub validation_count: u64,
    pub error_count: u64,
}

impl Default for ConfigurationCombinerMetrics {
    fn default() -> Self {
        Self {
            rule_count: 0,
            merge_count: 0,
            validation_count: 0,
            error_count: 0,
        }
    }
}

/// Configuration aggregator metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationAggregatorMetrics {
    pub total_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub category_requests: u64,
    pub all_config_requests: u64,
    pub error_count: u64,
}

impl Default for ConfigurationAggregatorMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            cache_hits: 0,
            cache_misses: 0,
            category_requests: 0,
            all_config_requests: 0,
            error_count: 0,
        }
    }
}

/// Configuration change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationChange {
    pub provider_id: String,
    pub category: String,
    pub key: String,
    pub old_value: Option<ConfigurationValue>,
    pub new_value: Option<ConfigurationValue>,
    pub timestamp: DateTime<Utc>,
    pub change_type: ConfigurationChangeType,
}

/// Type of configuration change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigurationChangeType {
    Added,
    Updated,
    Removed,
    ProviderRegistered,
    ProviderUnregistered,
}