//! Caps Provider implementation for the Actor Core system.
//!
//! This module contains the concrete implementation of the CapsProvider trait
//! responsible for cap calculation and management.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;

use crate::interfaces::{
    CapsProvider, CapLayerRegistry
};
use crate::enums::AcrossLayerPolicy;
use crate::metrics::CapStatistics;
use crate::types::*;
use crate::ActorCoreResult;

/// CapsProviderImpl is the implementation of the CapsProvider trait.
pub struct CapsProviderImpl {
    /// Registry for cap layer configuration
    cap_layer_registry: Arc<dyn CapLayerRegistry>,
    /// Metrics for performance monitoring
    #[allow(dead_code)]
    metrics: Arc<RwLock<CapStatistics>>,
}

impl CapsProviderImpl {
    /// Create a new caps provider instance.
    pub fn new(cap_layer_registry: Arc<dyn CapLayerRegistry>) -> Self {
        Self {
            cap_layer_registry,
            metrics: Arc::new(RwLock::new(CapStatistics::default())),
        }
    }
}

#[async_trait]
impl CapsProvider for CapsProviderImpl {
    async fn effective_caps_within_layer(
        &self,
        _actor: &Actor,
        outputs: &[SubsystemOutput],
        layer: &str,
    ) -> ActorCoreResult<HashMap<String, Caps>> {
        let mut caps_by_dimension: HashMap<String, Vec<&CapContribution>> = HashMap::new();
        
        // Group cap contributions by dimension
        for output in outputs {
            for cap in &output.caps {
                if cap.scope.as_deref() == Some(layer) {
                    caps_by_dimension
                        .entry(cap.dimension.clone())
                        .or_insert_with(Vec::new)
                        .push(cap);
                }
            }
        }

        let mut effective_caps = HashMap::new();
        
        // Process each dimension
        for (dimension, caps) in caps_by_dimension {
            let mut min_caps = Vec::new();
            let mut max_caps = Vec::new();
            
            // Separate min and max caps
            for cap in caps {
                match cap.kind.as_str() {
                    "min" => min_caps.push(cap),
                    "max" => max_caps.push(cap),
                    _ => {} // Ignore unknown kinds
                }
            }
            
            // Calculate effective min cap
            let effective_min = if min_caps.is_empty() {
                0.0
            } else {
                min_caps.iter()
                    .map(|cap| cap.value)
                    .fold(f64::NEG_INFINITY, f64::max)
            };
            
            // Calculate effective max cap
            let effective_max = if max_caps.is_empty() {
                f64::INFINITY
            } else {
                max_caps.iter()
                    .map(|cap| cap.value)
                    .fold(f64::INFINITY, f64::min)
            };
            
            effective_caps.insert(dimension, Caps::new(effective_min, effective_max));
        }
        
        Ok(effective_caps)
    }

    async fn effective_caps_across_layers(
        &self,
        actor: &Actor,
        outputs: &[SubsystemOutput],
    ) -> ActorCoreResult<HashMap<String, Caps>> {
        let layer_order = self.cap_layer_registry.get_layer_order();
        let policy = self.cap_layer_registry.get_across_layer_policy();
        
        let mut final_caps = HashMap::new();
        
        // Get caps for each layer
        let mut layer_caps = Vec::new();
        for layer in &layer_order {
            let caps = self.effective_caps_within_layer(actor, outputs, layer).await?;
            layer_caps.push(caps);
        }
        
        // Combine caps across layers based on policy
        match policy {
            AcrossLayerPolicy::Intersect => {
                // Start with infinite range and intersect with each layer
                for layer_cap in layer_caps {
                    for (dimension, caps) in layer_cap {
                        let entry = final_caps.entry(dimension).or_insert_with(|| Caps::new(f64::NEG_INFINITY, f64::INFINITY));
                        *entry = entry.intersection(&caps);
                    }
                }
            }
            AcrossLayerPolicy::Union => {
                // Start with empty range and union with each layer
                for layer_cap in layer_caps {
                    for (dimension, caps) in layer_cap {
                        let entry = final_caps.entry(dimension).or_insert_with(|| Caps::new(f64::INFINITY, f64::NEG_INFINITY));
                        *entry = entry.union(&caps);
                    }
                }
            }
            AcrossLayerPolicy::PrioritizedOverride => {
                // Later layers override earlier ones
                for layer_cap in layer_caps {
                    for (dimension, caps) in layer_cap {
                        final_caps.insert(dimension, caps);
                    }
                }
            }
        }
        
        Ok(final_caps)
    }

    fn get_layer_order(&self) -> Vec<String> {
        self.cap_layer_registry.get_layer_order()
    }

    fn get_across_layer_policy(&self) -> AcrossLayerPolicy {
        self.cap_layer_registry.get_across_layer_policy()
    }

    fn validate_caps(&self, dimension: &str, caps: &Caps) -> ActorCoreResult<()> {
        if !caps.is_valid() {
            return Err(crate::ActorCoreError::InvalidInput(
                format!("Invalid caps for dimension {}: min={}, max={}", dimension, caps.min, caps.max)
            ));
        }
        
        if caps.min < 0.0 {
            return Err(crate::ActorCoreError::InvalidInput(
                format!("Caps min value cannot be negative for dimension {}: {}", dimension, caps.min)
            ));
        }
        
        Ok(())
    }

    async fn get_caps_for_dimension(
        &self,
        dimension: &str,
        _actor: &Actor,
    ) -> ActorCoreResult<Option<Caps>> {
        // This is a simplified implementation
        // In a real implementation, this would query the database or cache
        warn!("get_caps_for_dimension not fully implemented for dimension: {}", dimension);
        Ok(None)
    }

    fn get_supported_dimensions(&self) -> Vec<String> {
        // Return all supported dimensions
        crate::constants::all_dimensions().iter().map(|s| s.to_string()).collect()
    }

    fn get_cap_statistics(&self) -> CapStatistics {
        // This is a simplified implementation
        // In a real implementation, this would return actual statistics
        CapStatistics::default()
    }

    fn validate(&self) -> ActorCoreResult<()> {
        self.cap_layer_registry.validate()
    }
}