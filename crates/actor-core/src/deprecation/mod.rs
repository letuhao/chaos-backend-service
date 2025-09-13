//! Deprecation management module for Actor Core.
//!
//! This module provides comprehensive tools for managing API deprecations,
//! migration guides, and rollback procedures to ensure smooth transitions
//! between different versions of Actor Core.

pub mod deprecation_manager;
pub mod migration_guide;

// Re-export the main deprecation types and functions
pub use deprecation_manager::{
    DeprecationManager,
    DeprecationEntry,
    DeprecationStatus,
    DeprecationSeverity,
    DeprecationConfig,
    DeprecationReport,
    RollbackPlan,
    RollbackStep,
    RiskLevel,
    default_deprecations,
    default_rollback_plans,
};

// Re-export migration guide types
pub use migration_guide::{
    MigrationGuideManager,
    MigrationGuide,
    MigrationComplexity,
    MigrationTime,
    BreakingChange,
    BreakingChangeType,
    ChangeImpact,
    MigrationStep,
    CodeExample,
    MigrationPitfall,
    MigrationResource,
    ResourceType,
    MigrationChecklistItem,
    ChecklistCategory,
    MigrationGuideTemplate,
    TemplateSection,
    MigrationGuideData,
    default_migration_guides,
};