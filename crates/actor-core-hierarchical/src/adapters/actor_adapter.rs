//! # Actor Adapter
//! 
//! Specific adapter for converting between actor data formats.

use crate::core::HierarchicalActor;
use crate::adapters::{BaseAdapter, BaseAdapterImpl, AdapterResult, SystemData};
use std::collections::HashMap;

/// Actor-specific adapter implementation
#[derive(Debug)]
pub struct ActorAdapter {
    pub base_adapter: BaseAdapterImpl,
}

impl ActorAdapter {
    /// Create a new actor adapter
    pub fn new() -> Self {
        Self {
            base_adapter: BaseAdapterImpl::new(
                "ActorAdapter".to_string(),
                vec!["hierarchical_actor".to_string(), "base_actor".to_string()],
            ),
        }
    }
    
    /// Convert base actor to hierarchical actor
    pub fn to_hierarchical_actor(&self, base_actor: &dyn SystemData) -> AdapterResult<HierarchicalActor> {
        self.base_adapter.to_hierarchical_actor(base_actor)
    }
    
    /// Convert hierarchical actor to base actor format
    pub fn from_hierarchical_actor(&self, actor: &HierarchicalActor) -> AdapterResult<Box<dyn SystemData>> {
        self.base_adapter.from_hierarchical_actor(actor)
    }
    
    /// Validate hierarchical actor
    pub fn validate_hierarchical_actor(&self, actor: &HierarchicalActor) -> AdapterResult<()> {
        self.base_adapter.validate_hierarchical_actor(actor)
    }
    
    /// Validate base actor
    pub fn validate_base_actor(&self, base_actor: &dyn SystemData) -> AdapterResult<()> {
        self.base_adapter.validate_system_data(base_actor)
    }
}

impl Default for ActorAdapter {
    fn default() -> Self {
        Self::new()
    }
}

