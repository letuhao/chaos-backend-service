//! # Element Contribution Structure
//! 
//! This module defines the ElementContribution struct for standardized data exchange.

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Standardized contribution format for external systems
/// 
/// This struct represents a contribution from an external system to Element-Core.
/// It contains all the necessary information for Element-Core to aggregate and
/// process the contribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementContribution {
    /// System that contributed this data
    pub system_id: String,
    
    /// Element type this contribution applies to
    pub element_type: String,
    
    /// Stat contributions (stat_name -> value)
    pub stat_contributions: HashMap<String, f64>,
    
    /// Priority weight (higher = more important)
    pub priority: i64,
    
    /// Timestamp when this contribution was created
    pub timestamp: DateTime<Utc>,
}

impl ElementContribution {
    /// Create a new element contribution
    pub fn new(
        system_id: String,
        element_type: String,
        stat_contributions: HashMap<String, f64>,
        priority: i64,
    ) -> Self {
        Self {
            system_id,
            element_type,
            stat_contributions,
            priority,
            timestamp: Utc::now(),
        }
    }
    
    /// Create a contribution with a specific timestamp
    pub fn with_timestamp(
        system_id: String,
        element_type: String,
        stat_contributions: HashMap<String, f64>,
        priority: i64,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            system_id,
            element_type,
            stat_contributions,
            priority,
            timestamp,
        }
    }
    
    /// Add a stat contribution
    pub fn add_stat(&mut self, stat_name: String, value: f64) -> Result<(), crate::ElementCoreError> {
        // Validate stat name
        if stat_name.is_empty() {
            return Err(crate::ElementCoreError::Validation { 
                message: "Stat name cannot be empty".to_string()
            });
        }
        
        if stat_name.len() > 100 {
            return Err(crate::ElementCoreError::Validation { 
                message: format!("Stat name too long (max 100 chars), got {} chars", stat_name.len())
            });
        }
        
        // Validate value
        if !value.is_finite() {
            return Err(crate::ElementCoreError::Validation { 
                message: format!("Stat value must be finite, got {}", value)
            });
        }
        
        self.stat_contributions.insert(stat_name, value);
        Ok(())
    }
    
    /// Get a stat contribution value
    pub fn get_stat(&self, stat_name: &str) -> Option<f64> {
        self.stat_contributions.get(stat_name).copied()
    }
    
    /// Check if this contribution has a specific stat
    pub fn has_stat(&self, stat_name: &str) -> bool {
        self.stat_contributions.contains_key(stat_name)
    }
    
    /// Get all stat names
    pub fn get_stat_names(&self) -> Vec<String> {
        self.stat_contributions.keys().cloned().collect()
    }
    
    /// Get the number of stat contributions
    pub fn stat_count(&self) -> usize {
        self.stat_contributions.len()
    }
    
    /// Check if this contribution is empty
    pub fn is_empty(&self) -> bool {
        self.stat_contributions.is_empty()
    }
    
    /// Merge another contribution into this one
    /// 
    /// Stats from the other contribution will override stats in this one
    /// if they have the same name.
    pub fn merge(&mut self, other: ElementContribution) -> Result<(), crate::ElementCoreError> {
        for (stat_name, value) in other.stat_contributions {
            self.add_stat(stat_name, value)?;
        }
        
        // Update timestamp to the more recent one
        if other.timestamp > self.timestamp {
            self.timestamp = other.timestamp;
        }
        
        Ok(())
    }
    
    /// Create a copy with only specific stats
    pub fn filter_stats(&self, stat_names: &[String]) -> Self {
        let mut filtered_contributions = HashMap::new();
        
        for stat_name in stat_names {
            if let Some(&value) = self.stat_contributions.get(stat_name) {
                filtered_contributions.insert(stat_name.clone(), value);
            }
        }
        
        Self {
            system_id: self.system_id.clone(),
            element_type: self.element_type.clone(),
            stat_contributions: filtered_contributions,
            priority: self.priority,
            timestamp: self.timestamp,
        }
    }
    
    /// Create a copy with stats matching a prefix
    pub fn filter_stats_by_prefix(&self, prefix: &str) -> Self {
        let mut filtered_contributions = HashMap::new();
        
        for (stat_name, &value) in &self.stat_contributions {
            if stat_name.starts_with(prefix) {
                filtered_contributions.insert(stat_name.clone(), value);
            }
        }
        
        Self {
            system_id: self.system_id.clone(),
            element_type: self.element_type.clone(),
            stat_contributions: filtered_contributions,
            priority: self.priority,
            timestamp: self.timestamp,
        }
    }
}

impl Default for ElementContribution {
    fn default() -> Self {
        Self {
            system_id: String::new(),
            element_type: String::new(),
            stat_contributions: HashMap::new(),
            priority: 0,
            timestamp: Utc::now(),
        }
    }
}

/// Builder for ElementContribution
pub struct ElementContributionBuilder {
    system_id: String,
    element_type: String,
    stat_contributions: HashMap<String, f64>,
    priority: i64,
    timestamp: Option<DateTime<Utc>>,
}

impl ElementContributionBuilder {
    /// Create a new builder
    pub fn new(system_id: String, element_type: String) -> Self {
        Self {
            system_id,
            element_type,
            stat_contributions: HashMap::new(),
            priority: 0,
            timestamp: None,
        }
    }
    
    /// Set the priority
    pub fn priority(mut self, priority: i64) -> Self {
        self.priority = priority;
        self
    }
    
    /// Set the timestamp
    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
    
    /// Add a stat contribution
    pub fn add_stat(mut self, stat_name: String, value: f64) -> Self {
        self.stat_contributions.insert(stat_name, value);
        self
    }
    
    /// Add multiple stat contributions
    pub fn add_stats(mut self, stats: HashMap<String, f64>) -> Self {
        self.stat_contributions.extend(stats);
        self
    }
    
    /// Build the ElementContribution
    pub fn build(self) -> ElementContribution {
        ElementContribution {
            system_id: self.system_id,
            element_type: self.element_type,
            stat_contributions: self.stat_contributions,
            priority: self.priority,
            timestamp: self.timestamp.unwrap_or_else(Utc::now),
        }
    }
}
