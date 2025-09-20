//! Configuration aggregator implementation for the Configuration Hub system

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug};

use crate::config::types::*;
use crate::config::registry::ConfigurationRegistry;
use crate::config::provider::ConfigurationProvider;
use crate::config::combiner::ConfigurationCombiner;
use crate::ActorCoreResult;

/// Aggregator for processing configurations from all providers
#[async_trait]
pub trait ConfigurationAggregator: Send + Sync {
    /// Get configuration value for a specific key
    async fn get_config_value(&self, category: &str, key: &str) -> ActorCoreResult<Option<ConfigurationValue>>;
    
    /// Get all configuration values for a category
    async fn get_category_config(&self, category: &str) -> ActorCoreResult<HashMap<String, ConfigurationValue>>;
    
    /// Get all configuration values
    async fn get_all_config(&self) -> ActorCoreResult<HashMap<String, HashMap<String, ConfigurationValue>>>;
    
    /// Refresh configuration from all providers
    async fn refresh_config(&self) -> ActorCoreResult<()>;
    
    /// Invalidate configuration cache
    async fn invalidate_cache(&self) -> ActorCoreResult<()>;
    
    /// Get aggregator metrics
    async fn get_metrics(&self) -> ConfigurationAggregatorMetrics;
    
    /// Get all providers
    fn get_providers(&self) -> Vec<Arc<dyn ConfigurationProvider>>;
}

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
            
            // Update metrics
            let mut metrics = self.metrics.write();
            metrics.cache_hits += 1;
            metrics.total_requests += 1;
            
            return Ok(Some(cached_value));
        }

        // Get all providers for this category
        let providers = self.registry.get_providers_for_category(category).await;
        
        if providers.is_empty() {
            debug!("No providers found for category: {}", category);
            
            // Update metrics
            let mut metrics = self.metrics.write();
            metrics.cache_misses += 1;
            metrics.total_requests += 1;
            
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
            
            // Update metrics
            let mut metrics = self.metrics.write();
            metrics.cache_misses += 1;
            metrics.total_requests += 1;
            
            return Ok(None);
        }

        // Get merge rule
        let merge_rule = self.combiner.get_merge_rule(category, key).await
            .unwrap_or_else(|| ConfigurationMergeRule {
                strategy: ConfigurationMergeStrategy::Override,
                use_pipeline: false,
                // TODO: Load default_value from configuration
                default_value: None,
                validation_rules: vec![],
            });

        // Merge values
        let merged_value = self.combiner.merge_values(values, &merge_rule).await?;
        
        // Store in cache
        self.store_in_cache(cache_key, merged_value.clone());

        // Update metrics
        let mut metrics = self.metrics.write();
        metrics.cache_misses += 1;
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

    async fn get_metrics(&self) -> ConfigurationAggregatorMetrics {
        let metrics = self.metrics.read();
        metrics.clone()
    }
    
    fn get_providers(&self) -> Vec<Arc<dyn ConfigurationProvider>> {
        self.registry.get_all_providers()
    }
}
