//! # Elemental Adapter
//! 
//! This module provides simple adapter functionality for element-core integration.
//! Elemental data is pure data - no conversion or aggregation needed.

use element_core::{ElementalSystemData, ElementalSystem, ElementalRegistry};
use std::sync::Arc;

/// Simple elemental adapter - no complex conversions needed
pub struct ElementalAdapter {
    registry: Arc<ElementalRegistry>,
}

impl ElementalAdapter {
    /// Create a new elemental adapter
    pub fn new(registry: Arc<ElementalRegistry>) -> Self {
        Self { registry }
    }

    /// Get elemental system data (pure data access)
    pub fn get_elemental_data<'a>(&self, elemental_system: &'a ElementalSystem) -> &'a ElementalSystemData {
        elemental_system.get_data()
    }

    /// Get mutable elemental system data
    pub fn get_elemental_data_mut<'a>(&self, elemental_system: &'a mut ElementalSystem) -> &'a mut ElementalSystemData {
        elemental_system.get_data_mut()
    }

    /// Create new elemental system
    pub fn create_elemental_system(&self) -> ElementalSystem {
        ElementalSystem::new()
    }

    /// Get registry reference
    pub fn get_registry(&self) -> Arc<ElementalRegistry> {
        self.registry.clone()
    }
}

/// Simple elemental data converter trait
pub trait ElementalDataConverter {
    /// Convert to ElementalSystemData
    fn to_elemental_system_data(&self) -> ElementalSystemData;
}

/// Elemental system data converter implementation
impl ElementalDataConverter for ElementalSystemData {
    fn to_elemental_system_data(&self) -> ElementalSystemData {
        self.clone()
    }
}
