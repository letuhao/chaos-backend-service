//! Deprecation Management Example
//! 
//! This example demonstrates the comprehensive deprecation management system
//! for Actor Core, including deprecation tracking, migration guides, and
//! rollback procedures.

use actor_core::prelude::*;
use std::time::{Duration, SystemTime};

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    println!("🎮 Actor Core - Deprecation Management Example");
    println!("===============================================");

    // Example 1: Setting up Deprecation Management
    println!("\n1. Setting up Deprecation Management:");
    
    let mut deprecation_manager = DeprecationManager::new();
    
    // Register default deprecations
    let default_deprecations = default_deprecations::create_default_deprecations();
    for deprecation in default_deprecations {
        deprecation_manager.register_deprecation(deprecation)?;
    }
    
    println!("✅ Registered {} default deprecations", deprecation_manager.list_deprecations().len());

    // Example 2: Creating Custom Deprecations
    println!("\n2. Creating Custom Deprecations:");
    
    let custom_deprecation = DeprecationEntry {
        id: "legacy_subsystem_api".to_string(),
        name: "Legacy Subsystem API".to_string(),
        description: "The legacy subsystem API is deprecated in favor of the new trait-based system".to_string(),
        status: DeprecationStatus::Deprecated,
        deprecated_date: SystemTime::now() - Duration::from_secs(30 * 24 * 60 * 60), // 30 days ago
        removal_date: Some(SystemTime::now() + Duration::from_secs(60 * 24 * 60 * 60)), // 60 days from now
        replacement: Some("New Trait-Based Subsystem API".to_string()),
        migration_guide: Some("https://docs.actor-core.dev/migration/subsystem-api".to_string()),
        severity: DeprecationSeverity::Medium,
        affected_versions: vec!["0.1.0".to_string(), "0.2.0".to_string()],
        breaking_change: true,
        notes: Some("This affects all custom subsystem implementations".to_string()),
    };
    
    deprecation_manager.register_deprecation(custom_deprecation)?;
    println!("✅ Registered custom deprecation: legacy_subsystem_api");

    // Example 3: Checking Deprecation Status
    println!("\n3. Checking Deprecation Status:");
    
    let deprecations = deprecation_manager.list_deprecations();
    for deprecation in deprecations {
        let status_icon = match deprecation.status {
            DeprecationStatus::Stable => "✅",
            DeprecationStatus::Deprecated => "⚠️",
            DeprecationStatus::DeprecatedRemovalImminent => "🚨",
            DeprecationStatus::Removed => "❌",
        };
        
        println!("{} {}: {:?} (Severity: {:?})",
            status_icon,
            deprecation.name,
            deprecation.status,
            deprecation.severity
        );
        
        if deprecation.breaking_change {
            println!("   🔥 Breaking Change: {}", deprecation.description);
        }
        
        if let Some(replacement) = &deprecation.replacement {
            println!("   🔄 Replacement: {}", replacement);
        }
    }

    // Example 4: Deprecation Usage Logging
    println!("\n4. Deprecation Usage Logging:");
    
    // Simulate usage of deprecated features
    deprecation_manager.log_deprecation_usage("old_aggregator_api", Some("user_code.rs:42"));
    deprecation_manager.log_deprecation_usage("legacy_cache_backend", Some("cache_manager.rs:15"));
    deprecation_manager.log_deprecation_usage("legacy_subsystem_api", Some("my_subsystem.rs:8"));
    
    println!("✅ Logged usage of deprecated features (check logs for warnings)");

    // Example 5: Rollback Plan Management
    println!("\n5. Rollback Plan Management:");
    
    // Register default rollback plans
    let default_rollback_plans = default_rollback_plans::create_default_rollback_plans();
    for rollback_plan in default_rollback_plans {
        deprecation_manager.register_rollback_plan(rollback_plan)?;
    }
    
    // Create a custom rollback plan
    let custom_rollback_plan = RollbackPlan {
        id: "rollback_legacy_subsystem".to_string(),
        name: "Rollback Legacy Subsystem API".to_string(),
        description: "Rollback plan for the legacy subsystem API deprecation".to_string(),
        deprecation_id: "legacy_subsystem_api".to_string(),
        steps: vec![
            RollbackStep {
                step_number: 1,
                description: "Stop all services using the new subsystem API".to_string(),
                commands: vec![
                    "systemctl stop actor-core".to_string(),
                ],
                expected_outcome: "Services stopped successfully".to_string(),
                validation_criteria: Some("No actor-core processes running".to_string()),
                estimated_time: Duration::from_secs(30),
                critical: true,
            },
            RollbackStep {
                step_number: 2,
                description: "Revert subsystem implementations to legacy API".to_string(),
                commands: vec![
                    "git checkout v0.1.0 -- src/subsystems/".to_string(),
                    "cargo build".to_string(),
                ],
                expected_outcome: "Legacy API restored and compiled".to_string(),
                validation_criteria: Some("Build completes successfully".to_string()),
                estimated_time: Duration::from_secs(120),
                critical: true,
            },
            RollbackStep {
                step_number: 3,
                description: "Restart services with legacy API".to_string(),
                commands: vec![
                    "systemctl start actor-core".to_string(),
                ],
                expected_outcome: "Services started with legacy API".to_string(),
                validation_criteria: Some("Health check passes".to_string()),
                estimated_time: Duration::from_secs(60),
                critical: true,
            },
        ],
        estimated_duration: Duration::from_secs(210), // 3.5 minutes
        prerequisites: vec![
            "Legacy subsystem implementations available".to_string(),
            "Maintenance window scheduled".to_string(),
        ],
        risk_level: RiskLevel::Medium,
        validation_steps: vec![
            "Verify all services are running".to_string(),
            "Test subsystem functionality".to_string(),
            "Monitor error logs".to_string(),
        ],
    };
    
    deprecation_manager.register_rollback_plan(custom_rollback_plan)?;
    println!("✅ Registered {} rollback plans", deprecation_manager.list_rollback_plans().len());

    // Example 6: Migration Guide Management
    println!("\n6. Migration Guide Management:");
    
    let mut migration_manager = MigrationGuideManager::new();
    
    // Register default migration guides
    let default_guides = default_migration_guides::create_default_migration_guides();
    for guide in default_guides {
        migration_manager.register_guide(guide)?;
    }
    
    println!("✅ Registered {} migration guides", migration_manager.list_guides().len());

    // Example 7: Creating a Custom Migration Guide
    println!("\n7. Creating a Custom Migration Guide:");
    
    let custom_guide = MigrationGuide {
        id: "v0_2_to_v0_3".to_string(),
        from_version: "0.2.0".to_string(),
        to_version: "0.3.0".to_string(),
        title: "Migration from Actor Core v0.2 to v0.3".to_string(),
        description: "This guide covers the migration to the new validation system and observability features".to_string(),
        complexity: MigrationComplexity::Low,
        estimated_time: MigrationTime::Quick,
        breaking_changes: vec![
            BreakingChange {
                change_type: BreakingChangeType::Configuration,
                description: "Validation configuration format has changed".to_string(),
                affected_components: vec!["validation".to_string()],
                impact: ChangeImpact::Low,
                migration_strategy: "Update validation configuration files to new format".to_string(),
            },
        ],
        steps: vec![
            MigrationStep {
                step_number: 1,
                title: "Update Configuration".to_string(),
                instructions: "Update validation configuration to new format".to_string(),
                commands: vec![],
                expected_outcome: "Configuration updated successfully".to_string(),
                validation_criteria: Some("Configuration validation passes".to_string()),
                estimated_time: "10 minutes".to_string(),
                optional: false,
                prerequisites: vec![],
            },
            MigrationStep {
                step_number: 2,
                title: "Enable Observability".to_string(),
                instructions: "Enable new observability features if desired".to_string(),
                commands: vec![],
                expected_outcome: "Observability features configured".to_string(),
                validation_criteria: Some("Metrics collection working".to_string()),
                estimated_time: "15 minutes".to_string(),
                optional: true,
                prerequisites: vec!["Step 1 completed".to_string()],
            },
        ],
        code_examples: vec![
            CodeExample {
                title: "Validation Configuration Update".to_string(),
                description: "Example of updating validation configuration format".to_string(),
                language: "yaml".to_string(),
                before_code: r#"
# Before (v0.2)
validation:
  enabled: true
  strict_mode: false
"#.to_string(),
                after_code: r#"
# After (v0.3)
validation:
  rules:
    max_dimension_length: 64
    max_system_length: 64
    min_contribution_value: -1000000.0
    max_contribution_value: 1000000.0
  strict_mode: false
"#.to_string(),
                explanation: "The validation configuration now uses a rules-based approach with explicit limits".to_string(),
            },
        ],
        pitfalls: vec![
            MigrationPitfall {
                title: "Forgetting to update validation rules".to_string(),
                description: "Not updating validation rules may cause validation failures".to_string(),
                avoidance_strategy: "Test validation with new configuration before deployment".to_string(),
                consequences: "Runtime validation errors and system instability".to_string(),
            },
        ],
        rollback_instructions: Some("To rollback, revert to v0.2 configuration format and disable new features".to_string()),
        resources: vec![
            MigrationResource {
                title: "Validation Configuration Guide".to_string(),
                resource_type: ResourceType::Documentation,
                url: "https://docs.actor-core.dev/validation/configuration".to_string(),
                description: "Complete guide to validation configuration".to_string(),
            },
        ],
        checklist: vec![
            MigrationChecklistItem {
                item: "Update validation configuration format".to_string(),
                critical: true,
                category: ChecklistCategory::Configuration,
            },
            MigrationChecklistItem {
                item: "Test validation with new configuration".to_string(),
                critical: true,
                category: ChecklistCategory::Testing,
            },
            MigrationChecklistItem {
                item: "Enable observability features (optional)".to_string(),
                critical: false,
                category: ChecklistCategory::Configuration,
            },
        ],
    };
    
    migration_manager.register_guide(custom_guide)?;
    println!("✅ Registered custom migration guide: v0_2_to_v0_3");

    // Example 8: Generating Deprecation Reports
    println!("\n8. Generating Deprecation Reports:");
    
    let report = deprecation_manager.generate_report();
    println!("📊 Deprecation Report:");
    println!("   Generated: {:?}", report.generated_at);
    println!("   Total Deprecations: {}", report.total_deprecations);
    println!("   Active Deprecations: {}", report.active_deprecations);
    println!("   Approaching Removal: {}", report.approaching_removal);
    println!("   Critical Deprecations: {}", report.critical_deprecations);

    // Example 9: Migration Guide Templates
    println!("\n9. Migration Guide Templates:");
    
    let template = MigrationGuideTemplate {
        id: "standard_migration".to_string(),
        name: "Standard Migration Template".to_string(),
        description: "Template for standard API migrations".to_string(),
        default_complexity: MigrationComplexity::Medium,
        default_estimated_time: MigrationTime::Short,
        sections: vec![
            TemplateSection {
                id: "steps".to_string(),
                title: "Migration Steps".to_string(),
                description: "Step-by-step migration instructions".to_string(),
                required: true,
                template_content: "Migration steps will be filled from guide data".to_string(),
            },
            TemplateSection {
                id: "code_examples".to_string(),
                title: "Code Examples".to_string(),
                description: "Before and after code examples".to_string(),
                required: true,
                template_content: "Code examples will be filled from guide data".to_string(),
            },
            TemplateSection {
                id: "checklist".to_string(),
                title: "Migration Checklist".to_string(),
                description: "Checklist for ensuring complete migration".to_string(),
                required: false,
                template_content: "Checklist items will be filled from guide data".to_string(),
            },
        ],
    };
    
    migration_manager.register_template(template)?;
    println!("✅ Registered migration guide template");

    // Example 10: Finding Deprecations by Status
    println!("\n10. Finding Deprecations by Status:");
    
    let deprecated_items = deprecation_manager.get_deprecations_by_status(&DeprecationStatus::Deprecated);
    println!("📋 Deprecated Features ({} items):", deprecated_items.len());
    for item in deprecated_items {
        println!("   • {} (ID: {})", item.name, item.id);
        if let Some(removal_date) = item.removal_date {
            let days_until_removal = removal_date.duration_since(SystemTime::now())
                .unwrap_or_default()
                .as_secs() / (24 * 60 * 60);
            println!("     Removal in {} days", days_until_removal);
        }
    }

    // Example 11: Risk Assessment and Rollback Planning
    println!("\n11. Risk Assessment and Rollback Planning:");
    
    let rollback_plans = deprecation_manager.list_rollback_plans();
    for plan in rollback_plans {
        println!("🔄 Rollback Plan: {}", plan.name);
        println!("   Risk Level: {:?}", plan.risk_level);
        println!("   Estimated Duration: {:?}", plan.estimated_duration);
        println!("   Steps: {}", plan.steps.len());
        
        if !plan.prerequisites.is_empty() {
            println!("   Prerequisites:");
            for prereq in &plan.prerequisites {
                println!("     • {}", prereq);
            }
        }
    }

    // Example 12: Migration Guide Analysis
    println!("\n12. Migration Guide Analysis:");
    
    let guides = migration_manager.list_guides();
    for guide in guides {
        println!("📖 Migration Guide: {}", guide.title);
        println!("   From: {} → To: {}", guide.from_version, guide.to_version);
        println!("   Complexity: {:?}", guide.complexity);
        println!("   Estimated Time: {:?}", guide.estimated_time);
        println!("   Breaking Changes: {}", guide.breaking_changes.len());
        println!("   Steps: {}", guide.steps.len());
        println!("   Code Examples: {}", guide.code_examples.len());
        println!("   Pitfalls: {}", guide.pitfalls.len());
    }

    // Example 13: Configuration Management
    println!("\n13. Configuration Management:");
    
    let config = deprecation_manager.get_config();
    println!("⚙️  Deprecation Manager Configuration:");
    println!("   Default Deprecation Period: {:?}", config.default_deprecation_period);
    println!("   Warning Threshold: {} days", config.warning_threshold_days);
    println!("   Critical Threshold: {} days", config.critical_threshold_days);
    println!("   Auto Warnings: {}", config.enable_automatic_warnings);
    println!("   Log Usage: {}", config.log_deprecation_usage);

    // Example 14: Best Practices Summary
    println!("\n14. Best Practices Summary:");
    println!("🎯 Deprecation Management Best Practices:");
    println!("   • Always provide clear migration paths");
    println!("   • Give users sufficient time to migrate");
    println!("   • Document breaking changes thoroughly");
    println!("   • Provide rollback procedures for critical changes");
    println!("   • Use appropriate severity levels");
    println!("   • Monitor deprecation usage in production");
    println!("   • Create comprehensive migration guides");
    println!("   • Test rollback procedures before deployment");

    println!("\n🎉 Deprecation management example completed successfully!");
    println!("\nKey takeaways:");
    println!("• Deprecation management ensures smooth API evolution");
    println!("• Migration guides help users transition between versions");
    println!("• Rollback plans provide safety nets for critical changes");
    println!("• Proper documentation reduces migration friction");
    println!("• Monitoring usage helps prioritize migration efforts");

    Ok(())
}
