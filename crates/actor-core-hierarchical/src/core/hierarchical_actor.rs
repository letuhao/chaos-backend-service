//! # Hierarchical Actor
//! 
//! Core hierarchical actor data structure for managing actor properties across multiple game systems.

use element_core::{ElementalSystem, ElementalSystemData};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Hierarchical actor data structure
pub struct HierarchicalActor {
    /// Unique actor identifier
    pub id: String,
    
    /// Actor name
    pub name: String,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Elemental system data
    pub elemental_system: ElementalSystem,
    
    /// Global stats cache for fast access
    pub global_stats_cache: HashMap<String, f64>,
    
    /// System contributions cache
    pub system_contributions: HashMap<String, Vec<SystemContribution>>,
    
    /// Actor metadata
    pub metadata: HashMap<String, String>,
}

/// System contribution for hierarchical aggregation
#[derive(Debug, Clone)]
pub struct SystemContribution {
    /// System name (e.g., "elemental", "cultivation", "magic")
    pub system_name: String,
    
    /// Stat name (e.g., "health", "mana", "strength")
    pub stat_name: String,
    
    /// Contribution value
    pub value: f64,
    
    /// Contribution priority (higher = more important)
    pub priority: u32,
    
    /// Timestamp of contribution
    pub timestamp: DateTime<Utc>,
}

impl Default for HierarchicalActor {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for HierarchicalActor {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            elemental_system: ElementalSystem::new(), // Create new instance since ElementalSystem doesn't implement Clone
            global_stats_cache: self.global_stats_cache.clone(),
            system_contributions: self.system_contributions.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl std::fmt::Debug for HierarchicalActor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HierarchicalActor")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .field("elemental_system", &"ElementalSystem")
            .field("global_stats_cache", &self.global_stats_cache)
            .field("system_contributions", &self.system_contributions)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl HierarchicalActor {
    /// Create a new hierarchical actor
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: "Unnamed Actor".to_string(),
            created_at: now,
            updated_at: now,
            elemental_system: ElementalSystem::new(),
            global_stats_cache: HashMap::new(),
            system_contributions: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Create a new hierarchical actor with specific ID and name
    pub fn with_id_and_name(id: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            created_at: now,
            updated_at: now,
            elemental_system: ElementalSystem::new(),
            global_stats_cache: HashMap::new(),
            system_contributions: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Get actor ID
    pub fn get_id(&self) -> &str {
        &self.id
    }
    
    /// Get actor name
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    /// Set actor name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now();
    }
    
    /// Get elemental system data
    pub fn get_elemental_system(&self) -> &ElementalSystem {
        &self.elemental_system
    }
    
    /// Get mutable elemental system data
    pub fn get_elemental_system_mut(&mut self) -> &mut ElementalSystem {
        self.updated_at = Utc::now();
        &mut self.elemental_system
    }
    
    /// Get elemental system data
    pub fn get_elemental_data(&self) -> &ElementalSystemData {
        self.elemental_system.get_data()
    }
    
    /// Get mutable elemental system data
    pub fn get_elemental_data_mut(&mut self) -> &mut ElementalSystemData {
        self.updated_at = Utc::now();
        self.elemental_system.get_data_mut()
    }
    
    /// Add system contribution
    pub fn add_system_contribution(&mut self, contribution: SystemContribution) {
        let system_name = contribution.system_name.clone();
        self.system_contributions
            .entry(system_name)
            .or_insert_with(Vec::new)
            .push(contribution);
        self.updated_at = Utc::now();
    }
    
    /// Get system contributions
    pub fn get_system_contributions(&self, system_name: &str) -> Option<&Vec<SystemContribution>> {
        self.system_contributions.get(system_name)
    }
    
    /// Update global stats cache
    pub fn update_global_stats_cache(&mut self, stats: HashMap<String, f64>) {
        self.global_stats_cache = stats;
        self.updated_at = Utc::now();
    }
    
    /// Get global stats cache
    pub fn get_global_stats_cache(&self) -> &HashMap<String, f64> {
        &self.global_stats_cache
    }
    
    /// Set metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.updated_at = Utc::now();
    }
    
    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
    
    /// Get all metadata
    pub fn get_all_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
    
    /// Get creation timestamp
    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    /// Get last update timestamp
    pub fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

