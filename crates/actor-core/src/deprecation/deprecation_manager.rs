//! Deprecation management system for Actor Core.
//!
//! This module provides tools for managing API deprecations, migration timelines,
//! and rollback procedures to ensure smooth transitions and maintain system reliability.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, debug};

use crate::ActorCoreResult;
use crate::ActorCoreError;

/// Deprecation status levels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeprecationStatus {
    /// Feature is stable and not deprecated
    Stable,
    /// Feature is deprecated but still functional
    Deprecated,
    /// Feature is deprecated and will be removed soon
    DeprecatedRemovalImminent,
    /// Feature has been removed
    Removed,
}

/// Deprecation timeline entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeprecationEntry {
    /// Unique identifier for the deprecated feature
    pub id: String,
    /// Name of the deprecated feature
    pub name: String,
    /// Description of what is being deprecated
    pub description: String,
    /// Current deprecation status
    pub status: DeprecationStatus,
    /// Date when the feature was deprecated
    pub deprecated_date: SystemTime,
    /// Date when the feature will be removed
    pub removal_date: Option<SystemTime>,
    /// Replacement feature or migration path
    pub replacement: Option<String>,
    /// Migration guide or documentation
    pub migration_guide: Option<String>,
    /// Severity of the deprecation
    pub severity: DeprecationSeverity,
    /// Affected versions
    pub affected_versions: Vec<String>,
    /// Breaking change indicator
    pub breaking_change: bool,
    /// Additional notes
    pub notes: Option<String>,
}

/// Severity levels for deprecations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeprecationSeverity {
    /// Low severity - minor impact
    Low,
    /// Medium severity - moderate impact
    Medium,
    /// High severity - significant impact
    High,
    /// Critical severity - major breaking change
    Critical,
}

/// Rollback plan for deprecations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackPlan {
    /// Plan identifier
    pub id: String,
    /// Name of the rollback plan
    pub name: String,
    /// Description of what this rollback plan covers
    pub description: String,
    /// Deprecation entry this rollback plan applies to
    pub deprecation_id: String,
    /// Steps to perform the rollback
    pub steps: Vec<RollbackStep>,
    /// Estimated rollback time
    pub estimated_duration: Duration,
    /// Prerequisites for rollback
    pub prerequisites: Vec<String>,
    /// Risk assessment
    pub risk_level: RiskLevel,
    /// Rollback validation steps
    pub validation_steps: Vec<String>,
}

/// Individual step in a rollback plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackStep {
    /// Step number
    pub step_number: u32,
    /// Step description
    pub description: String,
    /// Commands or actions to perform
    pub commands: Vec<String>,
    /// Expected outcome
    pub expected_outcome: String,
    /// Validation criteria
    pub validation_criteria: Option<String>,
    /// Estimated time for this step
    pub estimated_time: Duration,
    /// Whether this step is critical (cannot be skipped)
    pub critical: bool,
}

/// Risk levels for rollback operations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk - minimal impact
    Low,
    /// Medium risk - moderate impact
    Medium,
    /// High risk - significant impact
    High,
    /// Critical risk - major impact
    Critical,
}

/// Deprecation manager for tracking and managing API deprecations.
pub struct DeprecationManager {
    /// Registered deprecation entries
    deprecations: HashMap<String, DeprecationEntry>,
    /// Rollback plans
    rollback_plans: HashMap<String, RollbackPlan>,
    /// Configuration
    config: DeprecationConfig,
}

/// Configuration for deprecation management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeprecationConfig {
    /// Default deprecation period before removal
    pub default_deprecation_period: Duration,
    /// Warning threshold (days before removal)
    pub warning_threshold_days: u64,
    /// Critical threshold (days before removal)
    pub critical_threshold_days: u64,
    /// Enable automatic warnings
    pub enable_automatic_warnings: bool,
    /// Log deprecation usage
    pub log_deprecation_usage: bool,
}

impl Default for DeprecationConfig {
    fn default() -> Self {
        Self {
            default_deprecation_period: Duration::from_secs(365 * 24 * 60 * 60), // 1 year
            warning_threshold_days: 90,
            critical_threshold_days: 30,
            enable_automatic_warnings: true,
            log_deprecation_usage: true,
        }
    }
}

impl DeprecationManager {
    /// Create a new deprecation manager.
    pub fn new() -> Self {
        Self {
            deprecations: HashMap::new(),
            rollback_plans: HashMap::new(),
            config: DeprecationConfig::default(),
        }
    }

    /// Create a new deprecation manager with custom configuration.
    pub fn with_config(config: DeprecationConfig) -> Self {
        Self {
            deprecations: HashMap::new(),
            rollback_plans: HashMap::new(),
            config,
        }
    }

    /// Register a deprecation entry.
    pub fn register_deprecation(&mut self, deprecation: DeprecationEntry) -> ActorCoreResult<()> {
        let id = deprecation.id.clone();
        
        info!(
            deprecation_id = %id,
            deprecation_name = %deprecation.name,
            status = ?deprecation.status,
            "Registered deprecation entry"
        );

        self.deprecations.insert(id, deprecation);
        Ok(())
    }

    /// Update a deprecation entry.
    pub fn update_deprecation(&mut self, id: &str, deprecation: DeprecationEntry) -> ActorCoreResult<()> {
        if !self.deprecations.contains_key(id) {
            return Err(ActorCoreError::InvalidInput(format!("Deprecation '{}' not found", id)));
        }

        info!(
            deprecation_id = %id,
            "Updated deprecation entry"
        );

        self.deprecations.insert(id.to_string(), deprecation);
        Ok(())
    }

    /// Remove a deprecation entry.
    pub fn remove_deprecation(&mut self, id: &str) -> ActorCoreResult<()> {
        if self.deprecations.remove(id).is_some() {
            info!(deprecation_id = %id, "Removed deprecation entry");
            Ok(())
        } else {
            Err(ActorCoreError::InvalidInput(format!("Deprecation '{}' not found", id)))
        }
    }

    /// Get a deprecation entry by ID.
    pub fn get_deprecation(&self, id: &str) -> Option<&DeprecationEntry> {
        self.deprecations.get(id)
    }

    /// List all deprecation entries.
    pub fn list_deprecations(&self) -> Vec<&DeprecationEntry> {
        self.deprecations.values().collect()
    }

    /// Get deprecations by status.
    pub fn get_deprecations_by_status(&self, status: &DeprecationStatus) -> Vec<&DeprecationEntry> {
        self.deprecations.values()
            .filter(|d| d.status == *status)
            .collect()
    }

    /// Get deprecations approaching removal.
    pub fn get_approaching_removal(&self) -> Vec<&DeprecationEntry> {
        let now = SystemTime::now();
        let warning_threshold = Duration::from_secs(self.config.warning_threshold_days * 24 * 60 * 60);
        let critical_threshold = Duration::from_secs(self.config.critical_threshold_days * 24 * 60 * 60);

        self.deprecations.values()
            .filter(|d| {
                if let Some(removal_date) = d.removal_date {
                    let time_until_removal = removal_date.duration_since(now).unwrap_or_default();
                    time_until_removal <= warning_threshold
                } else {
                    false
                }
            })
            .collect()
    }

    /// Get critical deprecations (approaching removal soon).
    pub fn get_critical_deprecations(&self) -> Vec<&DeprecationEntry> {
        let now = SystemTime::now();
        let critical_threshold = Duration::from_secs(self.config.critical_threshold_days * 24 * 60 * 60);

        self.deprecations.values()
            .filter(|d| {
                if let Some(removal_date) = d.removal_date {
                    let time_until_removal = removal_date.duration_since(now).unwrap_or_default();
                    time_until_removal <= critical_threshold
                } else {
                    false
                }
            })
            .collect()
    }

    /// Check if a feature is deprecated.
    pub fn is_deprecated(&self, id: &str) -> bool {
        self.deprecations.get(id)
            .map(|d| matches!(d.status, DeprecationStatus::Deprecated | DeprecationStatus::DeprecatedRemovalImminent))
            .unwrap_or(false)
    }

    /// Log deprecation usage.
    pub fn log_deprecation_usage(&self, id: &str, context: Option<&str>) {
        if !self.config.log_deprecation_usage {
            return;
        }

        if let Some(deprecation) = self.deprecations.get(id) {
            match deprecation.status {
                DeprecationStatus::Deprecated => {
                    warn!(
                        deprecation_id = %id,
                        deprecation_name = %deprecation.name,
                        context = context.unwrap_or("unknown"),
                        "Deprecated feature usage detected"
                    );
                }
                DeprecationStatus::DeprecatedRemovalImminent => {
                    error!(
                        deprecation_id = %id,
                        deprecation_name = %deprecation.name,
                        context = context.unwrap_or("unknown"),
                        "CRITICAL: Deprecated feature with imminent removal usage detected"
                    );
                }
                _ => {} // Stable or already removed - no logging needed
            }
        }
    }

    /// Register a rollback plan.
    pub fn register_rollback_plan(&mut self, plan: RollbackPlan) -> ActorCoreResult<()> {
        let id = plan.id.clone();
        
        info!(
            rollback_plan_id = %id,
            rollback_plan_name = %plan.name,
            deprecation_id = %plan.deprecation_id,
            "Registered rollback plan"
        );

        self.rollback_plans.insert(id, plan);
        Ok(())
    }

    /// Get a rollback plan by ID.
    pub fn get_rollback_plan(&self, id: &str) -> Option<&RollbackPlan> {
        self.rollback_plans.get(id)
    }

    /// Get rollback plans for a deprecation.
    pub fn get_rollback_plans_for_deprecation(&self, deprecation_id: &str) -> Vec<&RollbackPlan> {
        self.rollback_plans.values()
            .filter(|p| p.deprecation_id == deprecation_id)
            .collect()
    }

    /// List all rollback plans.
    pub fn list_rollback_plans(&self) -> Vec<&RollbackPlan> {
        self.rollback_plans.values().collect()
    }

    /// Generate a deprecation report.
    pub fn generate_report(&self) -> DeprecationReport {
        let now = SystemTime::now();
        let total_deprecations = self.deprecations.len();
        let active_deprecations = self.deprecations.values()
            .filter(|d| matches!(d.status, DeprecationStatus::Deprecated | DeprecationStatus::DeprecatedRemovalImminent))
            .count();
        
        let approaching_removal = self.get_approaching_removal().len();
        let critical_deprecations = self.get_critical_deprecations().len();

        DeprecationReport {
            generated_at: now,
            total_deprecations,
            active_deprecations,
            approaching_removal,
            critical_deprecations,
            deprecations: self.deprecations.clone(),
            rollback_plans: self.rollback_plans.clone(),
        }
    }

    /// Update configuration.
    pub fn update_config(&mut self, config: DeprecationConfig) {
        self.config = config;
        info!("Updated deprecation manager configuration");
    }

    /// Get current configuration.
    pub fn get_config(&self) -> &DeprecationConfig {
        &self.config
    }
}

/// Comprehensive deprecation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeprecationReport {
    /// When this report was generated
    pub generated_at: SystemTime,
    /// Total number of deprecations
    pub total_deprecations: usize,
    /// Number of active deprecations
    pub active_deprecations: usize,
    /// Number of deprecations approaching removal
    pub approaching_removal: usize,
    /// Number of critical deprecations
    pub critical_deprecations: usize,
    /// All deprecation entries
    pub deprecations: HashMap<String, DeprecationEntry>,
    /// All rollback plans
    pub rollback_plans: HashMap<String, RollbackPlan>,
}

/// Default deprecations for Actor Core.
pub mod default_deprecations {
    use super::*;

    /// Create default deprecations for Actor Core.
    pub fn create_default_deprecations() -> Vec<DeprecationEntry> {
        vec![
            // Example deprecations - these would be real deprecations in production
            DeprecationEntry {
                id: "old_aggregator_api".to_string(),
                name: "Old Aggregator API".to_string(),
                description: "The old aggregator API is deprecated in favor of the new async API".to_string(),
                status: DeprecationStatus::Deprecated,
                deprecated_date: SystemTime::UNIX_EPOCH + Duration::from_secs(1609459200), // 2021-01-01
                removal_date: Some(SystemTime::UNIX_EPOCH + Duration::from_secs(1672531200)), // 2023-01-01
                replacement: Some("New Async Aggregator API".to_string()),
                migration_guide: Some("https://docs.actor-core.dev/migration/aggregator-api".to_string()),
                severity: DeprecationSeverity::High,
                affected_versions: vec!["0.1.0".to_string(), "0.2.0".to_string()],
                breaking_change: true,
                notes: Some("This is a breaking change that requires code migration".to_string()),
            },
            DeprecationEntry {
                id: "legacy_cache_backend".to_string(),
                name: "Legacy Cache Backend".to_string(),
                description: "The legacy cache backend is deprecated in favor of the new multi-layer cache".to_string(),
                status: DeprecationStatus::DeprecatedRemovalImminent,
                deprecated_date: SystemTime::UNIX_EPOCH + Duration::from_secs(1577836800), // 2020-01-01
                removal_date: Some(SystemTime::UNIX_EPOCH + Duration::from_secs(1640995200)), // 2022-01-01
                replacement: Some("Multi-Layer Cache System".to_string()),
                migration_guide: Some("https://docs.actor-core.dev/migration/cache-backend".to_string()),
                severity: DeprecationSeverity::Critical,
                affected_versions: vec!["0.0.1".to_string(), "0.1.0".to_string()],
                breaking_change: true,
                notes: Some("Critical: This backend will be removed in the next major version".to_string()),
            },
        ]
    }
}

/// Default rollback plans for Actor Core.
pub mod default_rollback_plans {
    use super::*;

    /// Create default rollback plans for Actor Core.
    pub fn create_default_rollback_plans() -> Vec<RollbackPlan> {
        vec![
            RollbackPlan {
                id: "rollback_old_aggregator".to_string(),
                name: "Rollback Old Aggregator API".to_string(),
                description: "Rollback plan for the old aggregator API deprecation".to_string(),
                deprecation_id: "old_aggregator_api".to_string(),
                steps: vec![
                    RollbackStep {
                        step_number: 1,
                        description: "Stop all services using the new aggregator API".to_string(),
                        commands: vec![
                            "systemctl stop actor-core".to_string(),
                            "docker stop actor-core-container".to_string(),
                        ],
                        expected_outcome: "All services stopped successfully".to_string(),
                        validation_criteria: Some("No processes running on port 8080".to_string()),
                        estimated_time: Duration::from_secs(30),
                        critical: true,
                    },
                    RollbackStep {
                        step_number: 2,
                        description: "Revert to previous version with old API".to_string(),
                        commands: vec![
                            "git checkout v0.1.0".to_string(),
                            "cargo build --release".to_string(),
                        ],
                        expected_outcome: "Previous version compiled successfully".to_string(),
                        validation_criteria: Some("Build completes without errors".to_string()),
                        estimated_time: Duration::from_secs(300),
                        critical: true,
                    },
                    RollbackStep {
                        step_number: 3,
                        description: "Start services with old API".to_string(),
                        commands: vec![
                            "systemctl start actor-core".to_string(),
                            "docker start actor-core-container".to_string(),
                        ],
                        expected_outcome: "Services started successfully".to_string(),
                        validation_criteria: Some("Health check passes".to_string()),
                        estimated_time: Duration::from_secs(60),
                        critical: true,
                    },
                ],
                estimated_duration: Duration::from_secs(390), // 6.5 minutes
                prerequisites: vec![
                    "Previous version available in git".to_string(),
                    "Backup of current configuration".to_string(),
                    "Maintenance window scheduled".to_string(),
                ],
                risk_level: RiskLevel::High,
                validation_steps: vec![
                    "Verify all services are running".to_string(),
                    "Run health checks".to_string(),
                    "Test basic functionality".to_string(),
                    "Monitor error logs".to_string(),
                ],
            },
        ]
    }
}