//! Validation middleware for Actor Core.
//!
//! This module provides middleware components that can be used to automatically
//! validate data at various points in the system.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use crate::types::*;
use crate::validation::*;
use crate::ActorCoreResult;
use crate::ActorCoreError;

/// Validation middleware that wraps other services.
pub struct ValidationMiddleware<T> {
    /// The wrapped service
    inner: Arc<T>,
    /// Validator instance
    validator: Arc<Validator>,
    /// Validation statistics
    stats: Arc<RwLock<ValidationStats>>,
}

/// Validation statistics for monitoring.
#[derive(Debug, Default)]
pub struct ValidationStats {
    /// Total validations performed
    pub total_validations: u64,
    /// Number of validations that passed
    pub passed_validations: u64,
    /// Number of validations that failed
    pub failed_validations: u64,
    /// Number of warnings generated
    pub warnings_generated: u64,
    /// Most common validation errors
    pub error_counts: HashMap<String, u64>,
}

impl<T> ValidationMiddleware<T> {
    /// Create a new validation middleware.
    pub fn new(inner: Arc<T>, validator: Arc<Validator>) -> Self {
        Self {
            inner,
            validator,
            stats: Arc::new(RwLock::new(ValidationStats::default())),
        }
    }

    /// Get validation statistics.
    pub async fn get_stats(&self) -> ValidationStats {
        self.stats.read().await.clone()
    }

    /// Reset validation statistics.
    pub async fn reset_stats(&self) {
        let mut stats = self.stats.write().await;
        *stats = ValidationStats::default();
    }

    /// Validate and record statistics.
    async fn validate_with_stats<F>(&self, validate_fn: F) -> ValidationResult
    where
        F: FnOnce(&Validator) -> ValidationResult,
    {
        let result = validate_fn(&self.validator);
        
        let mut stats = self.stats.write().await;
        stats.total_validations += 1;
        
        if result.is_valid {
            stats.passed_validations += 1;
            if result.has_warnings() {
                stats.warnings_generated += result.warnings.len() as u64;
            }
        } else {
            stats.failed_validations += 1;
            for error in &result.errors {
                *stats.error_counts.entry(error.code.clone()).or_insert(0) += 1;
            }
        }
        
        result
    }

    /// Get the inner service.
    pub fn inner(&self) -> &Arc<T> {
        &self.inner
    }

    /// Get the validator.
    pub fn validator(&self) -> &Arc<Validator> {
        &self.validator
    }
}

/// Validation middleware for aggregator services.
pub struct AggregatorValidationMiddleware {
    /// The wrapped aggregator
    inner: Arc<dyn crate::interfaces::Aggregator>,
    /// Validator instance
    validator: Arc<Validator>,
    /// Validation statistics
    stats: Arc<RwLock<ValidationStats>>,
}

impl AggregatorValidationMiddleware {
    /// Create a new aggregator validation middleware.
    pub fn new(inner: Arc<dyn crate::interfaces::Aggregator>, validator: Arc<Validator>) -> Self {
        Self {
            inner,
            validator,
            stats: Arc::new(RwLock::new(ValidationStats::default())),
        }
    }

    /// Get validation statistics.
    pub async fn get_stats(&self) -> ValidationStats {
        self.stats.read().await.clone()
    }

    /// Validate and record statistics.
    async fn validate_with_stats<F>(&self, validate_fn: F) -> ValidationResult
    where
        F: FnOnce(&Validator) -> ValidationResult,
    {
        let result = validate_fn(&self.validator);
        
        let mut stats = self.stats.write().await;
        stats.total_validations += 1;
        
        if result.is_valid {
            stats.passed_validations += 1;
            if result.has_warnings() {
                stats.warnings_generated += result.warnings.len() as u64;
            }
        } else {
            stats.failed_validations += 1;
            for error in &result.errors {
                *stats.error_counts.entry(error.code.clone()).or_insert(0) += 1;
            }
        }
        
        result
    }
}

#[async_trait::async_trait]
impl crate::interfaces::Aggregator for AggregatorValidationMiddleware {
    /// Resolve actor stats with validation.
    async fn resolve(&self, actor: &Actor) -> ActorCoreResult<Snapshot> {
        // Validate actor before processing
        let validation_result = self.validate_with_stats(|validator| {
            validator.validate_actor(actor)
        }).await;

        if !validation_result.is_valid {
            error!("Actor validation failed: {:?}", validation_result.errors);
            return Err(ActorCoreError::InvalidActor(
                validation_result.first_error().unwrap_or_else(|| "Actor validation failed".to_string())
            ));
        }

        if validation_result.has_warnings() {
            warn!("Actor validation warnings: {:?}", validation_result.warnings);
        }

        // Process with inner aggregator
        let snapshot = self.inner.resolve(actor).await?;

        // Validate snapshot after processing
        let snapshot_validation = self.validate_with_stats(|validator| {
            validator.validate_snapshot(&snapshot)
        }).await;

        if !snapshot_validation.is_valid {
            error!("Snapshot validation failed: {:?}", snapshot_validation.errors);
            return Err(ActorCoreError::AggregationError(
                snapshot_validation.first_error().unwrap_or_else(|| "Snapshot validation failed".to_string())
            ));
        }

        if snapshot_validation.has_warnings() {
            warn!("Snapshot validation warnings: {:?}", snapshot_validation.warnings);
        }

        info!("Actor resolution completed successfully with validation");
        Ok(snapshot)
    }

    /// Resolve actor stats with context and validation.
    async fn resolve_with_context(
        &self,
        actor: &Actor,
        context: &HashMap<String, serde_json::Value>,
    ) -> ActorCoreResult<Snapshot> {
        // Validate actor before processing
        let validation_result = self.validate_with_stats(|validator| {
            validator.validate_actor(actor)
        }).await;

        if !validation_result.is_valid {
            error!("Actor validation failed: {:?}", validation_result.errors);
            return Err(ActorCoreError::InvalidActor(
                validation_result.first_error().unwrap_or_else(|| "Actor validation failed".to_string())
            ));
        }

        // Validate context if provided
        if !context.is_empty() {
            let context_validation = self.validate_with_stats(|validator| {
                validator.validate_config(context)
            }).await;

            if !context_validation.is_valid {
                warn!("Context validation failed: {:?}", context_validation.errors);
                // Context validation failures are warnings, not errors
            }
        }

        // Process with inner aggregator
        let snapshot = self.inner.resolve_with_context(actor, context).await?;

        // Validate snapshot after processing
        let snapshot_validation = self.validate_with_stats(|validator| {
            validator.validate_snapshot(&snapshot)
        }).await;

        if !snapshot_validation.is_valid {
            error!("Snapshot validation failed: {:?}", snapshot_validation.errors);
            return Err(ActorCoreError::AggregationError(
                snapshot_validation.first_error().unwrap_or_else(|| "Snapshot validation failed".to_string())
            ));
        }

        info!("Actor resolution with context completed successfully with validation");
        Ok(snapshot)
    }

    /// Get aggregator metrics.
    async fn get_metrics(&self) -> crate::metrics::AggregatorMetrics {
        let mut metrics = self.inner.get_metrics().await;
        
        // Add validation statistics to metrics
        let validation_stats = self.get_stats().await;
        
        // Note: We can't directly add validation stats to AggregatorMetrics
        // as it doesn't have validation fields. In a real implementation,
        // we might extend the metrics structure or create a separate
        // validation metrics structure.
        
        metrics
    }
}

/// Validation middleware for cache services.
pub struct CacheValidationMiddleware {
    /// The wrapped cache
    inner: Arc<dyn crate::interfaces::Cache>,
    /// Validator instance
    validator: Arc<Validator>,
    /// Validation statistics
    stats: Arc<RwLock<ValidationStats>>,
}

impl CacheValidationMiddleware {
    /// Create a new cache validation middleware.
    pub fn new(inner: Arc<dyn crate::interfaces::Cache>, validator: Arc<Validator>) -> Self {
        Self {
            inner,
            validator,
            stats: Arc::new(RwLock::new(ValidationStats::default())),
        }
    }

    /// Get validation statistics.
    pub async fn get_stats(&self) -> ValidationStats {
        self.stats.read().await.clone()
    }

    /// Validate and record statistics.
    async fn validate_with_stats<F>(&self, validate_fn: F) -> ValidationResult
    where
        F: FnOnce(&Validator) -> ValidationResult,
    {
        let result = validate_fn(&self.validator);
        
        let mut stats = self.stats.write().await;
        stats.total_validations += 1;
        
        if result.is_valid {
            stats.passed_validations += 1;
            if result.has_warnings() {
                stats.warnings_generated += result.warnings.len() as u64;
            }
        } else {
            stats.failed_validations += 1;
            for error in &result.errors {
                *stats.error_counts.entry(error.code.clone()).or_insert(0) += 1;
            }
        }
        
        result
    }
}

#[async_trait::async_trait]
impl crate::interfaces::Cache for CacheValidationMiddleware {
    /// Get value from cache with validation.
    async fn get(&self, key: &str) -> ActorCoreResult<Option<serde_json::Value>> {
        // Validate key
        if key.is_empty() {
            return Err(ActorCoreError::InvalidInput("Cache key cannot be empty".to_string()));
        }

        let result = self.inner.get(key).await?;

        // If we got a value, try to validate it as a snapshot
        if let Some(value) = &result {
            if let Ok(snapshot) = serde_json::from_value::<Snapshot>(value.clone()) {
                let validation_result = self.validate_with_stats(|validator| {
                    validator.validate_snapshot(&snapshot)
                }).await;

                if !validation_result.is_valid {
                    warn!("Cached snapshot validation failed: {:?}", validation_result.errors);
                    // Return None instead of the invalid cached value
                    return Ok(None);
                }

                if validation_result.has_warnings() {
                    warn!("Cached snapshot validation warnings: {:?}", validation_result.warnings);
                }
            }
        }

        Ok(result)
    }

    /// Set value in cache with validation.
    async fn set(
        &self,
        key: String,
        value: serde_json::Value,
        ttl: Option<u64>,
    ) -> ActorCoreResult<()> {
        // Validate key
        if key.is_empty() {
            return Err(ActorCoreError::InvalidInput("Cache key cannot be empty".to_string()));
        }

        // If the value is a snapshot, validate it
        if let Ok(snapshot) = serde_json::from_value::<Snapshot>(value.clone()) {
            let validation_result = self.validate_with_stats(|validator| {
                validator.validate_snapshot(&snapshot)
            }).await;

            if !validation_result.is_valid {
                error!("Snapshot validation failed before caching: {:?}", validation_result.errors);
                return Err(ActorCoreError::InvalidInput(
                    validation_result.first_error().unwrap_or_else(|| "Snapshot validation failed".to_string())
                ));
            }

            if validation_result.has_warnings() {
                warn!("Snapshot validation warnings before caching: {:?}", validation_result.warnings);
            }
        }

        self.inner.set(key, value, ttl).await
    }

    /// Delete value from cache.
    async fn delete(&self, key: &str) -> ActorCoreResult<bool> {
        // Validate key
        if key.is_empty() {
            return Err(ActorCoreError::InvalidInput("Cache key cannot be empty".to_string()));
        }

        self.inner.delete(key).await
    }

    /// Clear cache.
    async fn clear(&self) -> ActorCoreResult<()> {
        self.inner.clear().await
    }

    /// Get cache statistics.
    async fn get_stats(&self) -> ActorCoreResult<crate::metrics::CacheStats> {
        self.inner.get_stats().await
    }
}

/// Validation middleware for registry services.
pub struct RegistryValidationMiddleware {
    /// The wrapped registry
    inner: Arc<dyn crate::interfaces::PluginRegistry>,
    /// Validator instance
    validator: Arc<Validator>,
    /// Validation statistics
    stats: Arc<RwLock<ValidationStats>>,
}

impl RegistryValidationMiddleware {
    /// Create a new registry validation middleware.
    pub fn new(inner: Arc<dyn crate::interfaces::PluginRegistry>, validator: Arc<Validator>) -> Self {
        Self {
            inner,
            validator,
            stats: Arc::new(RwLock::new(ValidationStats::default())),
        }
    }

    /// Get validation statistics.
    pub async fn get_stats(&self) -> ValidationStats {
        self.stats.read().await.clone()
    }

    /// Validate and record statistics.
    async fn validate_with_stats<F>(&self, validate_fn: F) -> ValidationResult
    where
        F: FnOnce(&Validator) -> ValidationResult,
    {
        let result = validate_fn(&self.validator);
        
        let mut stats = self.stats.write().await;
        stats.total_validations += 1;
        
        if result.is_valid {
            stats.passed_validations += 1;
            if result.has_warnings() {
                stats.warnings_generated += result.warnings.len() as u64;
            }
        } else {
            stats.failed_validations += 1;
            for error in &result.errors {
                *stats.error_counts.entry(error.code.clone()).or_insert(0) += 1;
            }
        }
        
        result
    }

    /// Validate and record statistics (sync version).
    fn validate_with_stats_sync<F>(&self, validate_fn: F) -> ValidationResult
    where
        F: FnOnce(&Validator) -> ValidationResult,
    {
        let result = validate_fn(&self.validator);
        
        // Note: This is a simplified version that doesn't update stats
        // In a real implementation, you might want to use a different approach
        // or make the stats thread-safe for sync access
        result
    }
}

impl crate::interfaces::PluginRegistry for RegistryValidationMiddleware {
    /// Register subsystem with validation.
    fn register(&self, subsystem: Arc<dyn crate::interfaces::Subsystem>) -> ActorCoreResult<()> {
        // Validate subsystem metadata
        let validation_result = self.validate_with_stats_sync(|validator| {
            validator.validate_system(subsystem.system_id(), "system_id", &mut ValidationResult::new())
        });

        if !validation_result.is_valid {
            error!("Subsystem registration validation failed: {:?}", validation_result.errors);
            return Err(ActorCoreError::SubsystemError(
                validation_result.first_error().unwrap_or_else(|| "Subsystem validation failed".to_string())
            ));
        }

        self.inner.register(subsystem)
    }

    /// Unregister subsystem by ID with validation.
    fn unregister(&self, system_id: &str) -> ActorCoreResult<()> {
        // Validate system ID
        if system_id.is_empty() {
            return Err(ActorCoreError::InvalidInput("System ID cannot be empty".to_string()));
        }

        self.inner.unregister(system_id)
    }

    /// Get subsystem by ID.
    fn get_by_id(&self, system_id: &str) -> Option<Arc<dyn crate::interfaces::Subsystem>> {
        // Validate system ID
        if system_id.is_empty() {
            return None;
        }

        self.inner.get_by_id(system_id)
    }

    /// Get all subsystems ordered by priority.
    fn get_by_priority(&self) -> Vec<Arc<dyn crate::interfaces::Subsystem>> {
        self.inner.get_by_priority()
    }

    /// Get subsystems by priority range.
    fn get_by_priority_range(&self, min_priority: i64, max_priority: i64) -> Vec<Arc<dyn crate::interfaces::Subsystem>> {
        self.inner.get_by_priority_range(min_priority, max_priority)
    }

    /// Check if a subsystem is registered.
    fn is_registered(&self, system_id: &str) -> bool {
        if system_id.is_empty() {
            return false;
        }

        self.inner.is_registered(system_id)
    }

    /// Get the number of registered subsystems.
    fn count(&self) -> usize {
        self.inner.count()
    }
}

/// Validation middleware factory for creating validation-enabled services.
pub struct ValidationMiddlewareFactory;

impl ValidationMiddlewareFactory {
    /// Create a validation-enabled aggregator.
    pub fn create_validated_aggregator(
        inner: Arc<dyn crate::interfaces::Aggregator>,
        validator: Arc<Validator>,
    ) -> Arc<AggregatorValidationMiddleware> {
        Arc::new(AggregatorValidationMiddleware::new(inner, validator))
    }

    /// Create a validation-enabled cache.
    pub fn create_validated_cache(
        inner: Arc<dyn crate::interfaces::Cache>,
        validator: Arc<Validator>,
    ) -> Arc<CacheValidationMiddleware> {
        Arc::new(CacheValidationMiddleware::new(inner, validator))
    }

    /// Create a validation-enabled registry.
    pub fn create_validated_registry(
        inner: Arc<dyn crate::interfaces::PluginRegistry>,
        validator: Arc<Validator>,
    ) -> Arc<RegistryValidationMiddleware> {
        Arc::new(RegistryValidationMiddleware::new(inner, validator))
    }
}