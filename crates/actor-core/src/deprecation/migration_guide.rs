//! Migration guide system for Actor Core.
//!
//! This module provides tools for creating and managing migration guides
//! to help users transition between different versions of Actor Core.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tracing::{info, debug};

use crate::ActorCoreResult;
use crate::ActorCoreError;

/// Migration guide for transitioning between Actor Core versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationGuide {
    /// Unique identifier for the migration guide
    pub id: String,
    /// Source version
    pub from_version: String,
    /// Target version
    pub to_version: String,
    /// Migration guide title
    pub title: String,
    /// Brief description of the migration
    pub description: String,
    /// Migration complexity level
    pub complexity: MigrationComplexity,
    /// Estimated migration time
    pub estimated_time: MigrationTime,
    /// Breaking changes in this migration
    pub breaking_changes: Vec<BreakingChange>,
    /// Step-by-step migration instructions
    pub steps: Vec<MigrationStep>,
    /// Code examples for before and after
    pub code_examples: Vec<CodeExample>,
    /// Common pitfalls and how to avoid them
    pub pitfalls: Vec<MigrationPitfall>,
    /// Rollback instructions
    pub rollback_instructions: Option<String>,
    /// Additional resources
    pub resources: Vec<MigrationResource>,
    /// Migration checklist
    pub checklist: Vec<MigrationChecklistItem>,
}

/// Complexity levels for migrations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MigrationComplexity {
    /// Low complexity - minor changes
    Low,
    /// Medium complexity - moderate changes
    Medium,
    /// High complexity - significant changes
    High,
    /// Critical complexity - major breaking changes
    Critical,
}

/// Estimated migration time ranges.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MigrationTime {
    /// Less than 1 hour
    Quick,
    /// 1-4 hours
    Short,
    /// 4-8 hours
    Medium,
    /// 1-3 days
    Long,
    /// More than 3 days
    Extensive,
}

/// Breaking change information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingChange {
    /// Type of breaking change
    pub change_type: BreakingChangeType,
    /// Description of the change
    pub description: String,
    /// Affected components
    pub affected_components: Vec<String>,
    /// Impact assessment
    pub impact: ChangeImpact,
    /// Migration strategy
    pub migration_strategy: String,
}

/// Types of breaking changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BreakingChangeType {
    /// API signature change
    ApiSignature,
    /// Data structure change
    DataStructure,
    /// Configuration change
    Configuration,
    /// Dependency change
    Dependency,
    /// Behavior change
    Behavior,
    /// Removal of feature
    FeatureRemoval,
}

/// Impact assessment for changes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeImpact {
    /// Low impact - minimal code changes needed
    Low,
    /// Medium impact - moderate code changes needed
    Medium,
    /// High impact - significant code changes needed
    High,
    /// Critical impact - major refactoring required
    Critical,
}

/// Individual migration step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStep {
    /// Step number
    pub step_number: u32,
    /// Step title
    pub title: String,
    /// Detailed instructions
    pub instructions: String,
    /// Commands to run (if any)
    pub commands: Vec<String>,
    /// Expected outcome
    pub expected_outcome: String,
    /// Validation criteria
    pub validation_criteria: Option<String>,
    /// Estimated time for this step
    pub estimated_time: String,
    /// Whether this step is optional
    pub optional: bool,
    /// Prerequisites for this step
    pub prerequisites: Vec<String>,
}

/// Code example showing before and after.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    /// Example title
    pub title: String,
    /// Description of what this example demonstrates
    pub description: String,
    /// Language/framework
    pub language: String,
    /// Before code
    pub before_code: String,
    /// After code
    pub after_code: String,
    /// Explanation of the changes
    pub explanation: String,
}

/// Common migration pitfall and how to avoid it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPitfall {
    /// Pitfall title
    pub title: String,
    /// Description of the pitfall
    pub description: String,
    /// How to avoid this pitfall
    pub avoidance_strategy: String,
    /// What happens if you fall into this pitfall
    pub consequences: String,
}

/// Additional resource for migration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationResource {
    /// Resource title
    pub title: String,
    /// Resource type
    pub resource_type: ResourceType,
    /// URL or identifier
    pub url: String,
    /// Description of the resource
    pub description: String,
}

/// Types of migration resources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    /// Documentation
    Documentation,
    /// Tutorial
    Tutorial,
    /// Video
    Video,
    /// Tool
    Tool,
    /// Example repository
    ExampleRepository,
    /// Community discussion
    CommunityDiscussion,
}

/// Checklist item for migration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationChecklistItem {
    /// Checklist item text
    pub item: String,
    /// Whether this item is critical
    pub critical: bool,
    /// Category of the checklist item
    pub category: ChecklistCategory,
}

/// Categories for migration checklist items.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChecklistCategory {
    /// Code changes
    CodeChanges,
    /// Configuration changes
    Configuration,
    /// Testing
    Testing,
    /// Deployment
    Deployment,
    /// Documentation
    Documentation,
    /// Team coordination
    TeamCoordination,
}

/// Migration guide manager.
pub struct MigrationGuideManager {
    /// Registered migration guides
    guides: HashMap<String, MigrationGuide>,
    /// Migration guide templates
    templates: HashMap<String, MigrationGuideTemplate>,
}

/// Template for creating migration guides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationGuideTemplate {
    /// Template identifier
    pub id: String,
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Default complexity
    pub default_complexity: MigrationComplexity,
    /// Default estimated time
    pub default_estimated_time: MigrationTime,
    /// Template sections
    pub sections: Vec<TemplateSection>,
}

/// Template section for migration guides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    /// Section identifier
    pub id: String,
    /// Section title
    pub title: String,
    /// Section description
    pub description: String,
    /// Whether this section is required
    pub required: bool,
    /// Template content
    pub template_content: String,
}

impl MigrationGuideManager {
    /// Create a new migration guide manager.
    pub fn new() -> Self {
        Self {
            guides: HashMap::new(),
            templates: HashMap::new(),
        }
    }

    /// Register a migration guide.
    pub fn register_guide(&mut self, guide: MigrationGuide) -> ActorCoreResult<()> {
        let id = guide.id.clone();
        
        info!(
            migration_guide_id = %id,
            from_version = %guide.from_version,
            to_version = %guide.to_version,
            "Registered migration guide"
        );

        self.guides.insert(id, guide);
        Ok(())
    }

    /// Get a migration guide by ID.
    pub fn get_guide(&self, id: &str) -> Option<&MigrationGuide> {
        self.guides.get(id)
    }

    /// Get migration guides for a version range.
    pub fn get_guides_for_version_range(&self, from_version: &str, to_version: &str) -> Vec<&MigrationGuide> {
        self.guides.values()
            .filter(|guide| guide.from_version == from_version && guide.to_version == to_version)
            .collect()
    }

    /// Get all migration guides.
    pub fn list_guides(&self) -> Vec<&MigrationGuide> {
        self.guides.values().collect()
    }

    /// Register a migration guide template.
    pub fn register_template(&mut self, template: MigrationGuideTemplate) -> ActorCoreResult<()> {
        let id = template.id.clone();
        
        info!(
            template_id = %id,
            template_name = %template.name,
            "Registered migration guide template"
        );

        self.templates.insert(id, template);
        Ok(())
    }

    /// Get a migration guide template by ID.
    pub fn get_template(&self, id: &str) -> Option<&MigrationGuideTemplate> {
        self.templates.get(id)
    }

    /// List all migration guide templates.
    pub fn list_templates(&self) -> Vec<&MigrationGuideTemplate> {
        self.templates.values().collect()
    }

    /// Generate a migration guide from a template.
    pub fn generate_guide_from_template(
        &self,
        template_id: &str,
        guide_data: MigrationGuideData,
    ) -> ActorCoreResult<MigrationGuide> {
        let template = self.get_template(template_id)
            .ok_or_else(|| ActorCoreError::InvalidInput(format!("Template '{}' not found", template_id)))?;

        let mut guide = MigrationGuide {
            id: guide_data.id,
            from_version: guide_data.from_version,
            to_version: guide_data.to_version,
            title: guide_data.title,
            description: guide_data.description,
            complexity: guide_data.complexity.unwrap_or(template.default_complexity),
            estimated_time: guide_data.estimated_time.unwrap_or(template.default_estimated_time),
            breaking_changes: guide_data.breaking_changes,
            steps: Vec::new(),
            code_examples: Vec::new(),
            pitfalls: Vec::new(),
            rollback_instructions: guide_data.rollback_instructions,
            resources: Vec::new(),
            checklist: Vec::new(),
        };

        // Process template sections
        for section in &template.sections {
            match section.id.as_str() {
                "steps" => {
                    guide.steps = guide_data.steps.clone();
                }
                "code_examples" => {
                    guide.code_examples = guide_data.code_examples.clone();
                }
                "pitfalls" => {
                    guide.pitfalls = guide_data.pitfalls.clone();
                }
                "resources" => {
                    guide.resources = guide_data.resources.clone();
                }
                "checklist" => {
                    guide.checklist = guide_data.checklist.clone();
                }
                _ => {
                    // Handle custom sections
                    debug!(section_id = %section.id, "Processing custom template section");
                }
            }
        }

        Ok(guide)
    }
}

/// Data for generating a migration guide from a template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationGuideData {
    /// Guide ID
    pub id: String,
    /// From version
    pub from_version: String,
    /// To version
    pub to_version: String,
    /// Guide title
    pub title: String,
    /// Guide description
    pub description: String,
    /// Migration complexity (optional, will use template default)
    pub complexity: Option<MigrationComplexity>,
    /// Estimated time (optional, will use template default)
    pub estimated_time: Option<MigrationTime>,
    /// Breaking changes
    pub breaking_changes: Vec<BreakingChange>,
    /// Migration steps
    pub steps: Vec<MigrationStep>,
    /// Code examples
    pub code_examples: Vec<CodeExample>,
    /// Migration pitfalls
    pub pitfalls: Vec<MigrationPitfall>,
    /// Rollback instructions
    pub rollback_instructions: Option<String>,
    /// Additional resources
    pub resources: Vec<MigrationResource>,
    /// Migration checklist
    pub checklist: Vec<MigrationChecklistItem>,
}

/// Default migration guides for Actor Core.
pub mod default_migration_guides {
    use super::*;

    /// Create default migration guides for Actor Core.
    pub fn create_default_migration_guides() -> Vec<MigrationGuide> {
        vec![
            MigrationGuide {
                id: "v0_1_to_v0_2".to_string(),
                from_version: "0.1.0".to_string(),
                to_version: "0.2.0".to_string(),
                title: "Migration from Actor Core v0.1 to v0.2".to_string(),
                description: "This guide covers the migration from Actor Core v0.1 to v0.2, including the new async API and improved validation system".to_string(),
                complexity: MigrationComplexity::Medium,
                estimated_time: MigrationTime::Short,
                breaking_changes: vec![
                    BreakingChange {
                        change_type: BreakingChangeType::ApiSignature,
                        description: "Aggregator API is now async".to_string(),
                        affected_components: vec!["aggregator".to_string()],
                        impact: ChangeImpact::Medium,
                        migration_strategy: "Update all calls to use async/await syntax".to_string(),
                    },
                    BreakingChange {
                        change_type: BreakingChangeType::DataStructure,
                        description: "Contribution structure has new required fields".to_string(),
                        affected_components: vec!["types".to_string(), "subsystems".to_string()],
                        impact: ChangeImpact::Low,
                        migration_strategy: "Update Contribution creation to include new fields".to_string(),
                    },
                ],
                steps: vec![
                    MigrationStep {
                        step_number: 1,
                        title: "Update Dependencies".to_string(),
                        instructions: "Update your Cargo.toml to use Actor Core v0.2".to_string(),
                        commands: vec![
                            "cargo update actor-core".to_string(),
                        ],
                        expected_outcome: "Dependencies updated successfully".to_string(),
                        validation_criteria: Some("cargo check passes".to_string()),
                        estimated_time: "5 minutes".to_string(),
                        optional: false,
                        prerequisites: vec![],
                    },
                    MigrationStep {
                        step_number: 2,
                        title: "Update Aggregator Usage".to_string(),
                        instructions: "Update all aggregator calls to use async/await".to_string(),
                        commands: vec![],
                        expected_outcome: "All aggregator calls are now async".to_string(),
                        validation_criteria: Some("Code compiles without errors".to_string()),
                        estimated_time: "30 minutes".to_string(),
                        optional: false,
                        prerequisites: vec!["Step 1 completed".to_string()],
                    },
                    MigrationStep {
                        step_number: 3,
                        title: "Update Contribution Creation".to_string(),
                        instructions: "Update Contribution struct usage to include new fields".to_string(),
                        commands: vec![],
                        expected_outcome: "All Contribution instances include required fields".to_string(),
                        validation_criteria: Some("All tests pass".to_string()),
                        estimated_time: "15 minutes".to_string(),
                        optional: false,
                        prerequisites: vec!["Step 2 completed".to_string()],
                    },
                ],
                code_examples: vec![
                    CodeExample {
                        title: "Async Aggregator Usage".to_string(),
                        description: "Example of updating aggregator calls to async".to_string(),
                        language: "rust".to_string(),
                        before_code: r#"
// Before (v0.1)
let snapshot = aggregator.resolve(&actor)?;
"#.to_string(),
                        after_code: r#"
// After (v0.2)
let snapshot = aggregator.resolve(&actor).await?;
"#.to_string(),
                        explanation: "The resolve method is now async and must be awaited".to_string(),
                    },
                ],
                pitfalls: vec![
                    MigrationPitfall {
                        title: "Forgetting to await async calls".to_string(),
                        description: "Not awaiting async aggregator calls will result in compilation errors".to_string(),
                        avoidance_strategy: "Use async/await syntax consistently and enable async-related compiler warnings".to_string(),
                        consequences: "Compilation failures and runtime errors".to_string(),
                    },
                ],
                rollback_instructions: Some("To rollback, revert to v0.1 and remove async/await syntax from aggregator calls".to_string()),
                resources: vec![
                    MigrationResource {
                        title: "Async Programming in Rust".to_string(),
                        resource_type: ResourceType::Documentation,
                        url: "https://rust-lang.github.io/async-book/".to_string(),
                        description: "Official Rust async programming guide".to_string(),
                    },
                ],
                checklist: vec![
                    MigrationChecklistItem {
                        item: "Update Cargo.toml dependencies".to_string(),
                        critical: true,
                        category: ChecklistCategory::CodeChanges,
                    },
                    MigrationChecklistItem {
                        item: "Update all aggregator.resolve() calls to async".to_string(),
                        critical: true,
                        category: ChecklistCategory::CodeChanges,
                    },
                    MigrationChecklistItem {
                        item: "Update Contribution struct usage".to_string(),
                        critical: false,
                        category: ChecklistCategory::CodeChanges,
                    },
                    MigrationChecklistItem {
                        item: "Run full test suite".to_string(),
                        critical: true,
                        category: ChecklistCategory::Testing,
                    },
                ],
            },
        ]
    }
}