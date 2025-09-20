# Configuration Hub Implementation Guide

## 🎯 **OVERVIEW**

This guide provides step-by-step implementation instructions for the missing Configuration Hub components in Actor Core, following the established patterns and architecture.

## 🏗️ **IMPLEMENTATION PHASES**

### **Phase 1: Core Configuration Hub (Week 1-2)**

#### **Step 1: Configuration Registry Implementation**
Create `src/config/registry.rs`:

```rust
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, warn, error};

use crate::interfaces::ConfigurationProvider;
use crate::config::types::*;
use crate::ActorCoreResult;

/// Configuration registry implementation
pub struct ConfigurationRegistryImpl {
    providers: Arc<RwLock<HashMap<String, Arc<dyn ConfigurationProvider>>>>,
    metrics: Arc<RwLock<ConfigurationRegistryMetrics>>,
}

impl ConfigurationRegistryImpl {
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(ConfigurationRegistryMetrics::default())),
        }
    }

    /// Get all providers sorted by priority
    fn get_providers_by_priority(&self) -> Vec<Arc<dyn ConfigurationProvider>> {
        let providers = self.providers.read();
        let mut provider_list: Vec<Arc<dyn ConfigurationProvider>> = providers.values().cloned().collect();
        
        // Sort by priority (higher priority first)
        provider_list.sort_by(|a, b| b.priority().cmp(&a.priority()));
        provider_list
    }

    /// Get providers for a specific category
    fn get_providers_for_category(&self, category: &str) -> Vec<Arc<dyn ConfigurationProvider>> {
        let providers = self.providers.read();
        let mut category_providers = Vec::new();
        
        for provider in providers.values() {
            if provider.get_supported_categories().contains(&category.to_string()) {
                category_providers.push(provider.clone());
            }
        }
        
        // Sort by priority (higher priority first)
        category_providers.sort_by(|a, b| b.priority().cmp(&a.priority()));
        category_providers
    }
}

#[async_trait]
impl ConfigurationRegistry for ConfigurationRegistryImpl {
    async fn register_provider(&self, provider: Arc<dyn ConfigurationProvider>) -> ActorCoreResult<()> {
        let provider_id = provider.provider_id().to_string();
        
        if provider_id.is_empty() {
            return Err(crate::ActorCoreError::ConfigurationError(
                "Provider ID cannot be empty".to_string()
            ));
        }

        // Validate provider
        provider.validate_config().await?;

        let mut providers = self.providers.write();
        
        if providers.contains_key(&provider_id) {
            warn!("Overwriting existing configuration provider: {}", provider_id);
        }
        
        providers.insert(provider_id.clone(), provider);
        
        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.registered_count = providers.len();
        metrics.registration_attempts += 1;
        
        info!("Registered configuration provider: {}", provider_id);
        Ok(())
    }

    async fn unregister_provider(&self, provider_id: &str) -> ActorCoreResult<()> {
        let mut providers = self.providers.write();
        
        if providers.remove(provider_id).is_some() {
            // Update metrics
            let mut metrics = self.metrics.write();
            metrics.registered_count = providers.len();
            metrics.unregistration_attempts += 1;
            
            info!("Unregistered configuration provider: {}", provider_id);
            Ok(())
        } else {
            Err(crate::ActorCoreError::RegistryError(
                format!("Configuration provider not found: {}", provider_id)
            ))
        }
    }

    async fn get_provider(&self, provider_id: &str) -> Option<Arc<dyn ConfigurationProvider>> {
        let providers = self.providers.read();
        providers.get(provider_id).cloned()
    }

    async fn get_providers_by_priority(&self) -> Vec<Arc<dyn ConfigurationProvider>> {
        self.get_providers_by_priority()
    }

    async fn get_providers_for_category(&self, category: &str) -> Vec<Arc<dyn ConfigurationProvider>> {
        self.get_providers_for_category(category)
    }

    async fn validate_all_providers(&self) -> ActorCoreResult<()> {
        let providers = self.providers.read();
        
        for (provider_id, provider) in providers.iter() {
            if provider_id.is_empty() {
                return Err(crate::ActorCoreError::ConfigurationError(
                    "Empty provider ID found".to_string()
                ));
            }
            
            if provider.priority() < 0 {
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Invalid priority for provider {}: {}", provider_id, provider.priority())
                ));
            }

            // Validate provider configuration
            if let Err(e) = provider.validate_config().await {
                return Err(crate::ActorCoreError::ConfigurationError(
                    format!("Provider {} validation failed: {}", provider_id, e)
                ));
            }
        }
        
        Ok(())
    }
}

/// Configuration registry metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
```

#### **Step 2: Configuration Combiner Implementation**
Create `src/config/combiner.rs`:

```rust
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, warn, error};

use crate::config::types::*;
use crate::ActorCoreResult;

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
    pub fn load_default_rules(&self) -> ActorCoreResult<()> {
        let mut merge_rules = self.merge_rules.write();
        
        // Default rules for common categories
        let default_rules = [
            ("element_affinities", ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Sum,
                use_pipeline: true,
                default_value: Some(serde_json::Value::Number(0.0.into())),
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
                default_value: Some(serde_json::Value::Number(0.0.into())),
                validation_rules: vec![],
            }),
            ("derived_formulas", ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override,
                use_pipeline: false,
                default_value: None,
                validation_rules: vec![],
            }),
        ];
        
        for (category, rule) in default_rules {
            merge_rules.insert(category.to_string(), rule);
        }
        
        Ok(())
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
        metrics.set_count += 1;
        
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
                        sum_value = serde_json::Value::Number((sum_num + value_num).into());
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
                    value: serde_json::Value::Number(average.into()),
                    value_type: sorted_values[0].value_type.clone(),
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
        if let Some(validation_rules) = &rule.validation_rules {
            for validation_rule in validation_rules {
                if let Err(e) = validation_rule.validate(&result) {
                    return Err(crate::ActorCoreError::ConfigurationError(
                        format!("Validation failed for merged value: {}", e)
                    ));
                }
            }
        }

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

        Ok(())
    }
}

/// Configuration combiner metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
```

#### **Step 3: Configuration Aggregator Implementation**
Create `src/config/aggregator.rs`:

```rust
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, warn, error, debug};

use crate::config::types::*;
use crate::ActorCoreResult;

/// Configuration aggregator implementation
pub struct ConfigurationAggregatorImpl {
    registry: Arc<dyn ConfigurationRegistry>,
    combiner: Arc<dyn ConfigurationCombiner>,
    cache: Arc<RwLock<HashMap<String, ConfigurationValue>>>,
    metrics: Arc<RwLock<ConfigurationAggregatorMetrics>>,
}

impl ConfigurationAggregatorImpl {
    pub fn new(registry: Arc<dyn ConfigurationRegistry>, combiner: Arc<dyn ConfigurationCombiner>) -> Self {
        Self {
            registry,
            combiner,
            cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(ConfigurationAggregatorMetrics::default())),
        }
    }

    /// Generate cache key for configuration value
    fn generate_cache_key(&self, category: &str, key: &str) -> String {
        format!("{}:{}", category, key)
    }

    /// Get configuration value from cache
    fn get_from_cache(&self, cache_key: &str) -> Option<ConfigurationValue> {
        let cache = self.cache.read();
        cache.get(cache_key).cloned()
    }

    /// Store configuration value in cache
    fn store_in_cache(&self, cache_key: String, value: ConfigurationValue) {
        let mut cache = self.cache.write();
        cache.insert(cache_key, value);
    }

    /// Clear cache
    fn clear_cache(&self) {
        let mut cache = self.cache.write();
        cache.clear();
    }
}

#[async_trait]
impl ConfigurationAggregator for ConfigurationAggregatorImpl {
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>> {
        let cache_key = self.generate_cache_key(category, key);
        
        // Try cache first
        if let Some(cached_value) = self.get_from_cache(&cache_key) {
            debug!("Cache hit for configuration: {}:{}", category, key);
            return Ok(Some(cached_value));
        }

        // Get all providers for this category
        let providers = self.registry.get_providers_for_category(category).await;
        
        if providers.is_empty() {
            debug!("No providers found for category: {}", category);
            return Ok(None);
        }

        // Collect values from all providers
        let mut values = Vec::new();
        for provider in providers {
            if let Some(value) = provider.get_config_value(category, key).await? {
                values.push(value);
            }
        }

        if values.is_empty() {
            debug!("No values found for configuration: {}:{}", category, key);
            return Ok(None);
        }

        // Get merge rule
        let merge_rule = self.combiner.get_merge_rule(category, key).await
            .unwrap_or_else(|| ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override,
                use_pipeline: false,
                default_value: None,
                validation_rules: vec![],
            });

        // Merge values
        let merged_value = self.combiner.merge_values(values, &merge_rule).await?;
        
        // Store in cache
        self.store_in_cache(cache_key, merged_value.clone());

        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.cache_hits += 1;
        metrics.total_requests += 1;

        Ok(Some(merged_value))
    }

    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        let mut result = HashMap::new();
        
        // Get all providers for this category
        let providers = self.registry.get_providers_for_category(category).await;
        
        if providers.is_empty() {
            debug!("No providers found for category: {}", category);
            return Ok(result);
        }

        // Collect all keys from all providers
        let mut all_keys = std::collections::HashSet::new();
        for provider in &providers {
            let category_config = provider.get_category_config(category).await?;
            for key in category_config.keys() {
                all_keys.insert(key.clone());
            }
        }

        // Get each key
        for key in all_keys {
            if let Some(value) = self.get_config_value(category, &key).await? {
                result.insert(key, value);
            }
        }

        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.category_requests += 1;
        metrics.total_requests += 1;

        Ok(result)
    }

    async fn get_all_config(&self) -> ActorCoreResult<HashMap<String, HashMap<String, ConfigurationValue>>> {
        let mut result = HashMap::new();
        
        // Get all providers
        let providers = self.registry.get_providers_by_priority().await;
        
        if providers.is_empty() {
            debug!("No providers registered");
            return Ok(result);
        }

        // Collect all categories from all providers
        let mut all_categories = std::collections::HashSet::new();
        for provider in &providers {
            for category in provider.get_supported_categories() {
                all_categories.insert(category);
            }
        }

        // Get each category
        for category in all_categories {
            let category_config = self.get_category_config(&category).await?;
            if !category_config.is_empty() {
                result.insert(category, category_config);
            }
        }

        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.all_config_requests += 1;
        metrics.total_requests += 1;

        Ok(result)
    }

    async fn refresh_config(&self) -> ActorCoreResult<()> {
        // Clear cache
        self.clear_cache();
        
        // Validate all providers
        self.registry.validate_all_providers().await?;
        
        info!("Configuration refreshed successfully");
        Ok(())
    }

    async fn invalidate_cache(&self) -> ActorCoreResult<()> {
        self.clear_cache();
        info!("Configuration cache invalidated");
        Ok(())
    }
}

/// Configuration aggregator metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
```

### **Phase 2: Configuration Providers (Week 3-4)**

#### **Step 4: File Configuration Provider**
Create `src/config/providers/file_provider.rs`:

```rust
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;
use serde_yaml;
use tokio::fs;
use tracing::{info, warn, error};

use crate::config::types::*;
use crate::ActorCoreResult;

/// File-based configuration provider
pub struct FileConfigurationProvider {
    provider_id: String,
    priority: i64,
    config_path: PathBuf,
    config_data: Arc<RwLock<HashMap<String, HashMap<String, ConfigurationValue>>>>,
    file_watcher: Option<tokio::task::JoinHandle<()>>,
}

impl FileConfigurationProvider {
    pub fn new(provider_id: String, priority: i64, config_path: PathBuf) -> Self {
        Self {
            provider_id,
            priority,
            config_path,
            config_data: Arc::new(RwLock::new(HashMap::new())),
            file_watcher: None,
        }
    }

    /// Load configuration from file
    pub async fn load_from_file(&self) -> ActorCoreResult<()> {
        if !self.config_path.exists() {
            return Err(crate::ActorCoreError::ConfigurationError(
                format!("Configuration file not found: {:?}", self.config_path)
            ));
        }

        let content = fs::read_to_string(&self.config_path).await?;
        let config: serde_yaml::Value = serde_yaml::from_str(&content)?;

        let mut config_data = self.config_data.write();
        config_data.clear();

        if let Some(categories) = config.get("categories").and_then(|v| v.as_mapping()) {
            for (category_key, category_value) in categories {
                let category_name = category_key.as_str().unwrap_or("unknown");
                let mut category_data = HashMap::new();

                if let Some(properties) = category_value.as_mapping() {
                    for (key, value) in properties {
                        let key_name = key.as_str().unwrap_or("unknown");
                        let config_value = ConfigurationValue {
                            value: value.clone(),
                            value_type: self.determine_value_type(value),
                            source_provider: self.provider_id.clone(),
                            priority: self.priority,
                            timestamp: chrono::Utc::now(),
                            can_override: true,
                            can_merge: true,
                        };
                        category_data.insert(key_name.to_string(), config_value);
                    }
                }

                config_data.insert(category_name.to_string(), category_data);
            }
        }

        info!("Loaded configuration from file: {:?}", self.config_path);
        Ok(())
    }

    /// Determine value type from serde_json::Value
    fn determine_value_type(&self, value: &serde_json::Value) -> ConfigurationValueType {
        match value {
            serde_json::Value::String(_) => ConfigurationValueType::String,
            serde_json::Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    ConfigurationValueType::Integer
                } else {
                    ConfigurationValueType::Float
                }
            },
            serde_json::Value::Bool(_) => ConfigurationValueType::Boolean,
            serde_json::Value::Array(_) => ConfigurationValueType::Array,
            serde_json::Value::Object(_) => ConfigurationValueType::Object,
            serde_json::Value::Null => ConfigurationValueType::String, // Default fallback
        }
    }

    /// Start watching file for changes
    pub async fn start_file_watcher(&mut self) -> ActorCoreResult<()> {
        let config_path = self.config_path.clone();
        let config_data = self.config_data.clone();
        let provider_id = self.provider_id.clone();

        let watcher = tokio::spawn(async move {
            let mut last_modified = std::time::SystemTime::UNIX_EPOCH;
            
            loop {
                if let Ok(metadata) = fs::metadata(&config_path).await {
                    if let Ok(modified) = metadata.modified() {
                        if modified > last_modified {
                            last_modified = modified;
                            
                            // Reload configuration
                            let content = match fs::read_to_string(&config_path).await {
                                Ok(content) => content,
                                Err(e) => {
                                    error!("Failed to read configuration file: {}", e);
                                    continue;
                                }
                            };

                            let config: serde_yaml::Value = match serde_yaml::from_str(&content) {
                                Ok(config) => config,
                                Err(e) => {
                                    error!("Failed to parse configuration file: {}", e);
                                    continue;
                                }
                            };

                            // Update configuration data
                            let mut data = config_data.write();
                            data.clear();

                            if let Some(categories) = config.get("categories").and_then(|v| v.as_mapping()) {
                                for (category_key, category_value) in categories {
                                    let category_name = category_key.as_str().unwrap_or("unknown");
                                    let mut category_data = HashMap::new();

                                    if let Some(properties) = category_value.as_mapping() {
                                        for (key, value) in properties {
                                            let key_name = key.as_str().unwrap_or("unknown");
                                            let config_value = ConfigurationValue {
                                                value: value.clone(),
                                                value_type: ConfigurationValueType::String, // Simplified
                                                source_provider: provider_id.clone(),
                                                priority: 100, // Default priority
                                                timestamp: chrono::Utc::now(),
                                                can_override: true,
                                                can_merge: true,
                                            };
                                            category_data.insert(key_name.to_string(), config_value);
                                        }
                                    }

                                    data.insert(category_name.to_string(), category_data);
                                }
                            }

                            info!("Configuration file reloaded: {:?}", config_path);
                        }
                    }
                }

                // Check every 1 second
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        self.file_watcher = Some(watcher);
        Ok(())
    }
}

#[async_trait]
impl ConfigurationProvider for FileConfigurationProvider {
    fn provider_id(&self) -> &str { &self.provider_id }
    fn priority(&self) -> i64 { self.priority }
    
    fn get_supported_categories(&self) -> Vec<String> {
        let config_data = self.config_data.read();
        config_data.keys().cloned().collect()
    }
    
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>> {
        let config_data = self.config_data.read();
        Ok(config_data.get(category)?.get(key).cloned())
    }
    
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>> {
        let config_data = self.config_data.read();
        Ok(config_data.get(category).cloned().unwrap_or_default())
    }
    
    fn get_merge_rule(&self, category: &str, key: &str) -> Option<ConfigurationMergeRule> {
        // File provider uses default merge rules
        None
    }
    
    async fn validate_config(&self) -> ActorCoreResult<()> {
        let config_data = self.config_data.read();
        
        for (category, properties) in config_data.iter() {
            if category.is_empty() {
                return Err(crate::ActorCoreError::ConfigurationError(
                    "Category name cannot be empty".to_string()
                ));
            }
            
            for (key, value) in properties.iter() {
                if key.is_empty() {
                    return Err(crate::ActorCoreError::ConfigurationError(
                        format!("Key name cannot be empty in category: {}", category)
                    ));
                }
                
                if value.priority < 0 {
                    return Err(crate::ActorCoreError::ConfigurationError(
                        format!("Priority must be non-negative for key {} in category {}", key, category)
                    ));
                }
            }
        }
        
        Ok(())
    }
}
```

## 🎯 **IMPLEMENTATION CHECKLIST**

### **Phase 1: Core Implementation**
- [ ] **ConfigurationRegistryImpl** - Registry for managing configuration providers
- [ ] **ConfigurationCombinerImpl** - Merge logic for configuration values
- [ ] **ConfigurationAggregatorImpl** - Main aggregation and caching logic
- [ ] **ConfigurationProvider trait** - Interface for configuration providers
- [ ] **ConfigurationValue types** - Data structures for configuration values
- [ ] **ConfigurationMergeRule** - Merge strategies and validation rules

### **Phase 2: Configuration Providers**
- [ ] **FileConfigurationProvider** - YAML/JSON file-based configuration
- [ ] **EnvironmentConfigurationProvider** - Environment variable configuration
- [ ] **DatabaseConfigurationProvider** - Database-based configuration
- [ ] **RuntimeConfigurationProvider** - Runtime configuration changes

### **Phase 3: Configuration Management**
- [ ] **ConfigurationLoader** - Load configuration from multiple sources
- [ ] **ConfigurationManager** - High-level configuration management
- [ ] **ConfigurationValidation** - Comprehensive validation pipeline
- [ ] **ConfigurationPersistence** - Save configuration changes

### **Phase 4: Advanced Features**
- [ ] **ConfigurationVersioning** - Version control for configuration changes
- [ ] **ConfigurationTesting** - Test framework for configuration
- [ ] **ConfigurationAnalytics** - Metrics and monitoring
- [ ] **ConfigurationHotReload** - Hot-reload capability

## 🚀 **NEXT STEPS**

1. **Start with Phase 1** - Implement core configuration hub components
2. **Create example providers** - Element Core, Primary Stats, etc.
3. **Add comprehensive tests** - Unit tests, integration tests
4. **Update documentation** - API documentation, usage examples
5. **Performance optimization** - Caching, async processing

This implementation will complete the Configuration Hub and make Actor Core a true hub for all system configurations!
