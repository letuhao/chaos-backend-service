//! # Adapters Module
//! 
//! This module provides adapter functionality for integration with other cores.
//!
//! - Combat-Core adapter: exposes read-only element combat stats.
//! - Condition-Core adapter: exposes standardized condition queries.
//! - Actor-Core adapter: minimal hooks to map element IDs to indices.
//!
use std::sync::Arc;
use crate::unified_registry::UnifiedElementRegistry;
use crate::core::elemental_system::ElementalSystem;
use crate::core::elemental_data::MAX_ELEMENTS;

/// Combat stats data returned to Combat-Core
#[derive(Debug, Clone)]
pub struct CombatElementStats {
    pub power: f64,
    pub defense: f64,
    pub crit_rate: f64,
    pub crit_damage: f64,
    pub accuracy: f64,
    pub dodge: f64,
}

/// Adapter for Combat-Core to fetch combined omni+element stats if needed
pub struct CombatCoreAdapter {
    pub registry: Arc<UnifiedElementRegistry>,
}

impl CombatCoreAdapter {
    pub fn new(registry: Arc<UnifiedElementRegistry>) -> Self { Self { registry } }

    /// Map element id to index and extract a compact combat view from an `ElementalSystem`
    pub fn get_combat_stats(&self, system: &ElementalSystem, element_id: &str) -> Option<CombatElementStats> {
        let index = self.registry.get_element_index(element_id).ok().flatten()?;
        if index >= MAX_ELEMENTS { return None; }
        Some(CombatElementStats {
            power: system.get_element_power_point(index).unwrap_or(0.0),
            defense: system.get_data().get_element_defense_point(index).unwrap_or(0.0),
            crit_rate: system.get_data().crit_rate[index],
            crit_damage: system.get_data().crit_damage[index],
            accuracy: system.get_data().accurate_rate[index],
            dodge: system.get_data().dodge_rate[index],
        })
    }
}

/// Minimal Condition-Core adapter interface
pub struct ConditionCoreAdapter {
    pub registry: Arc<UnifiedElementRegistry>,
}

impl ConditionCoreAdapter {
    pub fn new(registry: Arc<UnifiedElementRegistry>) -> Self { Self { registry } }

    /// Check if an element id exists in the registry
    pub fn has_element(&self, element_id: &str) -> bool {
        self.registry.is_element_registered(element_id)
    }
}

// TODO: Implement elemental adapters
