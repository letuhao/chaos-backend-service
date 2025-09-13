//! Service factory for creating actor core services.
//!
//! This module provides a factory for creating the main services
//! used by the actor core system.

use std::sync::Arc;
use crate::interfaces::{PluginRegistry, CombinerRegistry, CapsProvider, Cache, Aggregator};
use crate::registry::RegistryFactory;
use crate::cache::CacheFactory;
use crate::caps_provider::CapsProviderImpl;
use crate::aggregator::AggregatorImpl;
use crate::ActorCoreResult;

/// Factory for creating actor core services.
pub struct ServiceFactory;

impl ServiceFactory {
    /// Create a caps provider with the given cap layer registry.
    pub fn create_caps_provider(cap_layers: Arc<dyn crate::interfaces::CapLayerRegistry>) -> Arc<dyn CapsProvider> {
        Arc::new(CapsProviderImpl::new(cap_layers))
    }

    /// Create an aggregator with the given dependencies.
    pub fn create_aggregator(
        plugin_registry: Arc<dyn PluginRegistry>,
        combiner_registry: Arc<dyn CombinerRegistry>,
        caps_provider: Arc<dyn CapsProvider>,
        cache: Arc<dyn Cache>,
    ) -> Arc<dyn Aggregator> {
        Arc::new(AggregatorImpl::new(
            plugin_registry,
            combiner_registry,
            caps_provider,
            cache,
        ))
    }

    /// Create a plugin registry.
    pub fn create_plugin_registry() -> Arc<dyn PluginRegistry> {
        RegistryFactory::create_plugin_registry()
    }

    /// Create a combiner registry.
    pub fn create_combiner_registry() -> Arc<dyn CombinerRegistry> {
        RegistryFactory::create_combiner_registry()
    }

    /// Create a cap layer registry.
    pub fn create_cap_layer_registry() -> Arc<dyn crate::interfaces::CapLayerRegistry> {
        RegistryFactory::create_cap_layer_registry()
    }

    /// Create a cache.
    pub fn create_cache() -> ActorCoreResult<Arc<dyn Cache>> {
        Ok(CacheFactory::create_default_multi_layer_cache())
    }
}