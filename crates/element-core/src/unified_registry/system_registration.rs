//! # System Registration
//! 
//! This module defines the system registration structure for external systems.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// System registration information
/// 
/// This struct contains metadata about external systems that have registered
/// with the Element-Core registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRegistration {
    /// System identifier
    pub system_id: String,
    
    /// System name
    pub system_name: String,
    
    /// System description
    pub description: String,
    
    /// System version
    pub version: String,
    
    /// System priority (higher = more important)
    pub priority: i64,
    
    /// System capabilities
    pub capabilities: Vec<SystemCapability>,
    
    /// System configuration
    pub config: HashMap<String, serde_json::Value>,
    
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    
    /// System status
    pub status: SystemStatus,
    
    /// System health
    pub health: SystemHealth,
}

impl SystemRegistration {
    /// Validate system registration
    pub fn validate(&self) -> Result<(), String> {
        if self.system_id.is_empty() {
            return Err("System ID cannot be empty".to_string());
        }
        
        if self.system_name.is_empty() {
            return Err("System name cannot be empty".to_string());
        }
        
        if self.version.is_empty() {
            return Err("System version cannot be empty".to_string());
        }
        
        if self.priority < 0 {
            return Err("System priority cannot be negative".to_string());
        }
        
        Ok(())
    }
}

/// System capabilities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SystemCapability {
    /// Can contribute elemental stats
    ContributeStats,
    
    /// Can handle element events
    HandleEvents,
    
    /// Can provide element definitions
    ProvideDefinitions,
    
    /// Can manage element interactions
    ManageInteractions,
    
    /// Can provide status effects
    ProvideStatusEffects,
    
    /// Can manage element categories
    ManageCategories,
    
    /// Can provide plugins
    ProvidePlugins,
    
    /// Can cache data
    CacheData,
    
    /// Can validate data
    ValidateData,
    
    /// Can monitor performance
    MonitorPerformance,
}

/// System status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemStatus {
    /// System is active and healthy
    Active,
    
    /// System is inactive
    Inactive,
    
    /// System is in maintenance mode
    Maintenance,
    
    /// System has errors
    Error,
    
    /// System is deprecated
    Deprecated,
    
    /// System is unknown
    Unknown,
}

/// System health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Overall health score (0.0 - 1.0)
    pub health_score: f64,
    
    /// Response time in milliseconds
    pub response_time_ms: f64,
    
    /// Error rate (0.0 - 1.0)
    pub error_rate: f64,
    
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    
    /// Health check interval in seconds
    pub check_interval: u64,
    
    /// Health check failures
    pub failure_count: u32,
    
    /// Health check successes
    pub success_count: u32,
}

impl SystemRegistration {
    /// Create a new system registration
    pub fn new(
        system_id: String,
        system_name: String,
        description: String,
        version: String,
        priority: i64,
    ) -> Self {
        let now = Utc::now();
        Self {
            system_id,
            system_name,
            description,
            version,
            priority,
            capabilities: Vec::new(),
            config: HashMap::new(),
            registered_at: now,
            last_activity: now,
            status: SystemStatus::Active,
            health: SystemHealth::new(),
        }
    }
    
    /// Add a system capability
    pub fn add_capability(&mut self, capability: SystemCapability) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }
    
    /// Remove a system capability
    pub fn remove_capability(&mut self, capability: &SystemCapability) {
        self.capabilities.retain(|c| c != capability);
    }
    
    /// Check if system has a specific capability
    pub fn has_capability(&self, capability: &SystemCapability) -> bool {
        self.capabilities.contains(capability)
    }
    
    /// Set system configuration
    pub fn set_config(&mut self, key: String, value: serde_json::Value) {
        self.config.insert(key, value);
    }
    
    /// Get system configuration
    pub fn get_config(&self, key: &str) -> Option<&serde_json::Value> {
        self.config.get(key)
    }
    
    /// Update last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
    }
    
    /// Set system status
    pub fn set_status(&mut self, status: SystemStatus) {
        self.status = status;
        self.update_activity();
    }
    
    /// Update system health
    pub fn update_health(&mut self, health: SystemHealth) {
        self.health = health;
        self.update_activity();
    }
    
    /// Check if system is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.status, SystemStatus::Active) && 
        self.health.health_score > 0.5 &&
        self.health.error_rate < 0.1
    }
    
    /// Check if system is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, SystemStatus::Active)
    }
    
    /// Get system uptime
    pub fn get_uptime(&self) -> chrono::Duration {
        Utc::now() - self.registered_at
    }
    
    /// Get time since last activity
    pub fn get_time_since_last_activity(&self) -> chrono::Duration {
        Utc::now() - self.last_activity
    }
    
}

impl SystemHealth {
    /// Create a new system health
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            health_score: 1.0,
            response_time_ms: 0.0,
            error_rate: 0.0,
            last_check: now,
            check_interval: 60, // 1 minute
            failure_count: 0,
            success_count: 0,
        }
    }
    
    /// Update health with a successful check
    pub fn record_success(&mut self, response_time_ms: f64) {
        self.success_count += 1;
        self.response_time_ms = response_time_ms;
        self.last_check = Utc::now();
        
        // Calculate health score based on success rate and response time
        let total_checks = self.success_count + self.failure_count;
        let success_rate = if total_checks > 0 {
            self.success_count as f64 / total_checks as f64
        } else {
            1.0
        };
        
        // Health score decreases with response time and error rate
        let response_penalty = if response_time_ms > 1000.0 {
            0.2 // 20% penalty for slow response
        } else if response_time_ms > 500.0 {
            0.1 // 10% penalty for moderate response
        } else {
            0.0 // No penalty for fast response
        };
        
        self.health_score = (success_rate - response_penalty).max(0.0).min(1.0);
    }
    
    /// Update health with a failed check
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_check = Utc::now();
        
        // Calculate health score based on success rate
        let total_checks = self.success_count + self.failure_count;
        let success_rate = if total_checks > 0 {
            self.success_count as f64 / total_checks as f64
        } else {
            0.0
        };
        
        self.health_score = success_rate;
        self.error_rate = 1.0 - success_rate;
    }
    
    /// Check if health check is due
    pub fn is_check_due(&self) -> bool {
        let time_since_last_check = Utc::now() - self.last_check;
        time_since_last_check.num_seconds() >= self.check_interval as i64
    }
    
    /// Get total number of health checks
    pub fn total_checks(&self) -> u32 {
        self.success_count + self.failure_count
    }
    
    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        let total = self.total_checks();
        if total > 0 {
            self.success_count as f64 / total as f64
        } else {
            0.0
        }
    }
    
    /// Reset health statistics
    pub fn reset(&mut self) {
        self.health_score = 1.0;
        self.response_time_ms = 0.0;
        self.error_rate = 0.0;
        self.failure_count = 0;
        self.success_count = 0;
        self.last_check = Utc::now();
    }
}

impl Default for SystemHealth {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SystemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemStatus::Active => write!(f, "Active"),
            SystemStatus::Inactive => write!(f, "Inactive"),
            SystemStatus::Maintenance => write!(f, "Maintenance"),
            SystemStatus::Error => write!(f, "Error"),
            SystemStatus::Deprecated => write!(f, "Deprecated"),
            SystemStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

impl std::fmt::Display for SystemCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemCapability::ContributeStats => write!(f, "ContributeStats"),
            SystemCapability::HandleEvents => write!(f, "HandleEvents"),
            SystemCapability::ProvideDefinitions => write!(f, "ProvideDefinitions"),
            SystemCapability::ManageInteractions => write!(f, "ManageInteractions"),
            SystemCapability::ProvideStatusEffects => write!(f, "ProvideStatusEffects"),
            SystemCapability::ManageCategories => write!(f, "ManageCategories"),
            SystemCapability::ProvidePlugins => write!(f, "ProvidePlugins"),
            SystemCapability::CacheData => write!(f, "CacheData"),
            SystemCapability::ValidateData => write!(f, "ValidateData"),
            SystemCapability::MonitorPerformance => write!(f, "MonitorPerformance"),
        }
    }
}
