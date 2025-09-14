//! Deprecation module coverage tests for Actor Core.

use actor_core::deprecation::{
    DeprecationManager, DeprecationEntry, DeprecationStatus, DeprecationSeverity,
    DeprecationConfig, RollbackPlan, RollbackStep, RiskLevel
};
use actor_core::deprecation::migration_guide::{
    MigrationGuide, MigrationComplexity, MigrationTime, BreakingChange,
    MigrationStep, CodeExample, MigrationPitfall, MigrationResource,
    MigrationChecklistItem, MigrationGuideManager, BreakingChangeType,
    ChangeImpact, ChecklistCategory
};
use std::time::{Duration, SystemTime};

#[test]
fn test_deprecation_status() {
    // Test all deprecation status variants
    let stable = DeprecationStatus::Stable;
    let deprecated = DeprecationStatus::Deprecated;
    let imminent = DeprecationStatus::DeprecatedRemovalImminent;
    let removed = DeprecationStatus::Removed;
    
    assert_eq!(stable, DeprecationStatus::Stable);
    assert_eq!(deprecated, DeprecationStatus::Deprecated);
    assert_eq!(imminent, DeprecationStatus::DeprecatedRemovalImminent);
    assert_eq!(removed, DeprecationStatus::Removed);
    
    // Test debug formatting
    let debug_str = format!("{:?}", stable);
    assert!(debug_str.contains("Stable"));
    
    let debug_str = format!("{:?}", deprecated);
    assert!(debug_str.contains("Deprecated"));
    
    let debug_str = format!("{:?}", imminent);
    assert!(debug_str.contains("DeprecatedRemovalImminent"));
    
    let debug_str = format!("{:?}", removed);
    assert!(debug_str.contains("Removed"));
}

#[test]
fn test_deprecation_severity() {
    // Test all deprecation severity variants
    let low = DeprecationSeverity::Low;
    let medium = DeprecationSeverity::Medium;
    let high = DeprecationSeverity::High;
    let critical = DeprecationSeverity::Critical;
    
    assert_eq!(low, DeprecationSeverity::Low);
    assert_eq!(medium, DeprecationSeverity::Medium);
    assert_eq!(high, DeprecationSeverity::High);
    assert_eq!(critical, DeprecationSeverity::Critical);
    
    // Test debug formatting
    let debug_str = format!("{:?}", low);
    assert!(debug_str.contains("Low"));
    
    let debug_str = format!("{:?}", critical);
    assert!(debug_str.contains("Critical"));
}

#[test]
fn test_deprecation_entry_creation() {
    let now = SystemTime::now();
    let removal_date = now + Duration::from_secs(86400 * 30); // 30 days from now
    
    let entry = DeprecationEntry {
        id: "test-feature-1".to_string(),
        name: "Test Feature".to_string(),
        description: "A test feature that is being deprecated".to_string(),
        status: DeprecationStatus::Deprecated,
        deprecated_date: now,
        removal_date: Some(removal_date),
        replacement: Some("new-test-feature".to_string()),
        migration_guide: Some("https://example.com/migration".to_string()),
        severity: DeprecationSeverity::Medium,
        affected_versions: vec!["1.0.0".to_string(), "1.1.0".to_string()],
        breaking_change: true,
        notes: Some("This is a breaking change".to_string()),
    };
    
    assert_eq!(entry.id, "test-feature-1");
    assert_eq!(entry.name, "Test Feature");
    assert_eq!(entry.description, "A test feature that is being deprecated");
    assert_eq!(entry.status, DeprecationStatus::Deprecated);
    assert_eq!(entry.severity, DeprecationSeverity::Medium);
    assert!(entry.breaking_change);
    assert_eq!(entry.affected_versions.len(), 2);
    assert!(entry.notes.is_some());
}

#[test]
fn test_deprecation_entry_serialization() {
    let now = SystemTime::now();
    let entry = DeprecationEntry {
        id: "test-feature-2".to_string(),
        name: "Test Feature 2".to_string(),
        description: "Another test feature".to_string(),
        status: DeprecationStatus::Deprecated,
        deprecated_date: now,
        removal_date: None,
        replacement: None,
        migration_guide: None,
        severity: DeprecationSeverity::Low,
        affected_versions: vec!["2.0.0".to_string()],
        breaking_change: false,
        notes: None,
    };
    
    // Test JSON serialization
    let json = serde_json::to_string(&entry).unwrap();
    let deserialized: DeprecationEntry = serde_json::from_str(&json).unwrap();
    
    assert_eq!(entry.id, deserialized.id);
    assert_eq!(entry.name, deserialized.name);
    assert_eq!(entry.description, deserialized.description);
    assert_eq!(entry.status, deserialized.status);
    assert_eq!(entry.severity, deserialized.severity);
    assert_eq!(entry.breaking_change, deserialized.breaking_change);
    
    // Test YAML serialization
    let yaml = serde_yaml::to_string(&entry).unwrap();
    let deserialized_yaml: DeprecationEntry = serde_yaml::from_str(&yaml).unwrap();
    
    assert_eq!(entry.id, deserialized_yaml.id);
    assert_eq!(entry.name, deserialized_yaml.name);
    assert_eq!(entry.status, deserialized_yaml.status);
}

#[test]
fn test_deprecation_config() {
    let config = DeprecationConfig::default();
    
    assert_eq!(config.default_deprecation_period, Duration::from_secs(365 * 24 * 60 * 60));
    assert_eq!(config.warning_threshold_days, 90);
    assert_eq!(config.critical_threshold_days, 30);
    assert!(config.enable_automatic_warnings);
    assert!(config.log_deprecation_usage);
    
    // Test custom config
    let custom_config = DeprecationConfig {
        default_deprecation_period: Duration::from_secs(180 * 24 * 60 * 60), // 6 months
        warning_threshold_days: 60,
        critical_threshold_days: 14,
        enable_automatic_warnings: false,
        log_deprecation_usage: false,
    };
    
    assert_eq!(custom_config.default_deprecation_period, Duration::from_secs(180 * 24 * 60 * 60));
    assert_eq!(custom_config.warning_threshold_days, 60);
    assert_eq!(custom_config.critical_threshold_days, 14);
    assert!(!custom_config.enable_automatic_warnings);
    assert!(!custom_config.log_deprecation_usage);
}

#[test]
fn test_deprecation_manager() {
    let mut manager = DeprecationManager::new();
    
    let now = SystemTime::now();
    let entry = DeprecationEntry {
        id: "feature-1".to_string(),
        name: "Feature 1".to_string(),
        description: "First feature".to_string(),
        status: DeprecationStatus::Deprecated,
        deprecated_date: now,
        removal_date: None,
        replacement: None,
        migration_guide: None,
        severity: DeprecationSeverity::Low,
        affected_versions: vec!["1.0.0".to_string()],
        breaking_change: false,
        notes: None,
    };
    
    // Test registering deprecation
    let result = manager.register_deprecation(entry.clone());
    assert!(result.is_ok());
    
    // Test getting deprecation
    let retrieved = manager.get_deprecation("feature-1");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "feature-1");
    
    // Test getting non-existent deprecation
    let not_found = manager.get_deprecation("nonexistent");
    assert!(not_found.is_none());
    
    // Test listing deprecations
    let deprecations = manager.list_deprecations();
    assert_eq!(deprecations.len(), 1);
    assert_eq!(deprecations[0].id, "feature-1");
}

#[test]
fn test_rollback_plan() {
    let step = RollbackStep {
        step_number: 1,
        description: "Stop the service".to_string(),
        commands: vec!["systemctl stop myservice".to_string()],
        expected_outcome: "Service stopped".to_string(),
        validation_criteria: Some("Check service status".to_string()),
        estimated_time: Duration::from_secs(30),
        critical: true,
    };
    
    let plan = RollbackPlan {
        id: "rollback-1".to_string(),
        name: "Service Rollback".to_string(),
        description: "Rollback service to previous version".to_string(),
        deprecation_id: "feature-1".to_string(),
        steps: vec![step],
        estimated_duration: Duration::from_secs(300),
        prerequisites: vec!["Backup current state".to_string()],
        risk_level: RiskLevel::Medium,
        validation_steps: vec!["Verify service is running".to_string()],
    };
    
    assert_eq!(plan.id, "rollback-1");
    assert_eq!(plan.name, "Service Rollback");
    assert_eq!(plan.deprecation_id, "feature-1");
    assert_eq!(plan.steps.len(), 1);
    assert_eq!(plan.risk_level, RiskLevel::Medium);
    assert_eq!(plan.prerequisites.len(), 1);
    assert_eq!(plan.validation_steps.len(), 1);
}

#[test]
fn test_risk_level() {
    let low = RiskLevel::Low;
    let medium = RiskLevel::Medium;
    let high = RiskLevel::High;
    let critical = RiskLevel::Critical;
    
    assert_eq!(low, RiskLevel::Low);
    assert_eq!(medium, RiskLevel::Medium);
    assert_eq!(high, RiskLevel::High);
    assert_eq!(critical, RiskLevel::Critical);
    
    // Test debug formatting
    let debug_str = format!("{:?}", low);
    assert!(debug_str.contains("Low"));
    
    let debug_str = format!("{:?}", critical);
    assert!(debug_str.contains("Critical"));
}

#[test]
fn test_migration_complexity() {
    let low = MigrationComplexity::Low;
    let medium = MigrationComplexity::Medium;
    let high = MigrationComplexity::High;
    let critical = MigrationComplexity::Critical;
    
    assert_eq!(low, MigrationComplexity::Low);
    assert_eq!(medium, MigrationComplexity::Medium);
    assert_eq!(high, MigrationComplexity::High);
    assert_eq!(critical, MigrationComplexity::Critical);
    
    // Test debug formatting
    let debug_str = format!("{:?}", low);
    assert!(debug_str.contains("Low"));
    
    let debug_str = format!("{:?}", critical);
    assert!(debug_str.contains("Critical"));
}

#[test]
fn test_migration_time() {
    let quick = MigrationTime::Quick;
    let short = MigrationTime::Short;
    let medium = MigrationTime::Medium;
    let long = MigrationTime::Long;
    let extensive = MigrationTime::Extensive;
    
    // Test debug formatting
    let debug_str = format!("{:?}", quick);
    assert!(debug_str.contains("Quick"));
    
    let debug_str = format!("{:?}", short);
    assert!(debug_str.contains("Short"));
    
    let debug_str = format!("{:?}", medium);
    assert!(debug_str.contains("Medium"));
    
    let debug_str = format!("{:?}", long);
    assert!(debug_str.contains("Long"));
    
    let debug_str = format!("{:?}", extensive);
    assert!(debug_str.contains("Extensive"));
}

#[test]
fn test_breaking_change() {
    let breaking_change = BreakingChange {
        change_type: BreakingChangeType::ApiSignature,
        description: "API signature changed".to_string(),
        affected_components: vec!["api".to_string(), "client".to_string()],
        impact: ChangeImpact::High,
        migration_strategy: "Update all API calls".to_string(),
    };
    
    assert_eq!(breaking_change.change_type, BreakingChangeType::ApiSignature);
    assert_eq!(breaking_change.description, "API signature changed");
    assert_eq!(breaking_change.affected_components.len(), 2);
    assert_eq!(breaking_change.impact, ChangeImpact::High);
    assert_eq!(breaking_change.migration_strategy, "Update all API calls");
    
    // Test debug formatting
    let debug_str = format!("{:?}", breaking_change);
    assert!(debug_str.contains("API signature changed"));
}

#[test]
fn test_breaking_change_type() {
    let api_sig = BreakingChangeType::ApiSignature;
    let data_struct = BreakingChangeType::DataStructure;
    let config = BreakingChangeType::Configuration;
    let dependency = BreakingChangeType::Dependency;
    let behavior = BreakingChangeType::Behavior;
    let removal = BreakingChangeType::FeatureRemoval;
    
    assert_eq!(api_sig, BreakingChangeType::ApiSignature);
    assert_eq!(data_struct, BreakingChangeType::DataStructure);
    assert_eq!(config, BreakingChangeType::Configuration);
    assert_eq!(dependency, BreakingChangeType::Dependency);
    assert_eq!(behavior, BreakingChangeType::Behavior);
    assert_eq!(removal, BreakingChangeType::FeatureRemoval);
    
    // Test debug formatting
    let debug_str = format!("{:?}", api_sig);
    assert!(debug_str.contains("ApiSignature"));
}

#[test]
fn test_change_impact() {
    let low = ChangeImpact::Low;
    let medium = ChangeImpact::Medium;
    let high = ChangeImpact::High;
    let critical = ChangeImpact::Critical;
    
    assert_eq!(low, ChangeImpact::Low);
    assert_eq!(medium, ChangeImpact::Medium);
    assert_eq!(high, ChangeImpact::High);
    assert_eq!(critical, ChangeImpact::Critical);
    
    // Test debug formatting
    let debug_str = format!("{:?}", low);
    assert!(debug_str.contains("Low"));
    
    let debug_str = format!("{:?}", critical);
    assert!(debug_str.contains("Critical"));
}

#[test]
fn test_migration_step() {
    let step = MigrationStep {
        step_number: 1,
        title: "Update imports".to_string(),
        instructions: "Update your import statements".to_string(),
        commands: vec!["cargo update".to_string()],
        expected_outcome: "Dependencies updated".to_string(),
        validation_criteria: Some("Check compilation".to_string()),
        estimated_time: "15 minutes".to_string(),
        optional: false,
        prerequisites: vec!["Backup current code".to_string()],
    };
    
    assert_eq!(step.step_number, 1);
    assert_eq!(step.title, "Update imports");
    assert_eq!(step.instructions, "Update your import statements");
    assert_eq!(step.commands.len(), 1);
    assert_eq!(step.expected_outcome, "Dependencies updated");
    assert!(step.validation_criteria.is_some());
    assert_eq!(step.estimated_time, "15 minutes");
    assert!(!step.optional);
    assert_eq!(step.prerequisites.len(), 1);
    
    // Test debug formatting
    let debug_str = format!("{:?}", step);
    assert!(debug_str.contains("Update imports"));
}

#[test]
fn test_code_example() {
    let example = CodeExample {
        title: "Before and After".to_string(),
        description: "Example showing the change".to_string(),
        language: "rust".to_string(),
        before_code: "old_code()".to_string(),
        after_code: "new_code()".to_string(),
        explanation: "The function name changed".to_string(),
    };
    
    assert_eq!(example.title, "Before and After");
    assert_eq!(example.description, "Example showing the change");
    assert_eq!(example.language, "rust");
    assert_eq!(example.before_code, "old_code()");
    assert_eq!(example.after_code, "new_code()");
    assert_eq!(example.explanation, "The function name changed");
    
    // Test debug formatting
    let debug_str = format!("{:?}", example);
    assert!(debug_str.contains("Before and After"));
}

#[test]
fn test_migration_pitfall() {
    let pitfall = MigrationPitfall {
        title: "Common Mistake".to_string(),
        description: "A common mistake during migration".to_string(),
        avoidance_strategy: "Follow the migration guide carefully".to_string(),
        consequences: "Application will fail to compile".to_string(),
    };
    
    assert_eq!(pitfall.title, "Common Mistake");
    assert_eq!(pitfall.description, "A common mistake during migration");
    assert_eq!(pitfall.avoidance_strategy, "Follow the migration guide carefully");
    assert_eq!(pitfall.consequences, "Application will fail to compile");
    
    // Test debug formatting
    let debug_str = format!("{:?}", pitfall);
    assert!(debug_str.contains("Common Mistake"));
}

#[test]
fn test_migration_resource() {
    let resource = MigrationResource {
        title: "Documentation".to_string(),
        resource_type: actor_core::deprecation::migration_guide::ResourceType::Documentation,
        url: "https://example.com/docs".to_string(),
        description: "Official documentation".to_string(),
    };
    
    assert_eq!(resource.title, "Documentation");
    assert_eq!(resource.url, "https://example.com/docs");
    assert_eq!(resource.description, "Official documentation");
    
    // Test debug formatting
    let debug_str = format!("{:?}", resource);
    assert!(debug_str.contains("Documentation"));
}

#[test]
fn test_migration_checklist_item() {
    let item = MigrationChecklistItem {
        item: "Update dependencies".to_string(),
        critical: true,
        category: ChecklistCategory::CodeChanges,
    };
    
    assert_eq!(item.item, "Update dependencies");
    assert!(item.critical);
    assert_eq!(item.category, ChecklistCategory::CodeChanges);
    
    // Test debug formatting
    let debug_str = format!("{:?}", item);
    assert!(debug_str.contains("Update dependencies"));
}

#[test]
fn test_migration_guide_creation() {
    let guide = MigrationGuide {
        id: "guide-1".to_string(),
        from_version: "1.0.0".to_string(),
        to_version: "2.0.0".to_string(),
        title: "Migration from 1.0 to 2.0".to_string(),
        description: "Complete migration guide".to_string(),
        complexity: MigrationComplexity::High,
        estimated_time: MigrationTime::Long,
        breaking_changes: vec![],
        steps: vec![],
        code_examples: vec![],
        pitfalls: vec![],
        rollback_instructions: Some("Rollback instructions".to_string()),
        resources: vec![],
        checklist: vec![],
    };
    
    assert_eq!(guide.id, "guide-1");
    assert_eq!(guide.from_version, "1.0.0");
    assert_eq!(guide.to_version, "2.0.0");
    assert_eq!(guide.title, "Migration from 1.0 to 2.0");
    assert_eq!(guide.description, "Complete migration guide");
    assert_eq!(guide.complexity, MigrationComplexity::High);
    // Test debug formatting for estimated_time
    let debug_str = format!("{:?}", guide.estimated_time);
    assert!(debug_str.contains("Long"));
    assert!(guide.rollback_instructions.is_some());
    
    // Test debug formatting
    let debug_str = format!("{:?}", guide);
    assert!(debug_str.contains("guide-1"));
    assert!(debug_str.contains("Migration from 1.0 to 2.0"));
}

#[test]
fn test_migration_guide_serialization() {
    let guide = MigrationGuide {
        id: "guide-2".to_string(),
        from_version: "1.1.0".to_string(),
        to_version: "1.2.0".to_string(),
        title: "Minor Migration".to_string(),
        description: "Minor version migration".to_string(),
        complexity: MigrationComplexity::Low,
        estimated_time: MigrationTime::Short,
        breaking_changes: vec![],
        steps: vec![],
        code_examples: vec![],
        pitfalls: vec![],
        rollback_instructions: None,
        resources: vec![],
        checklist: vec![],
    };
    
    // Test JSON serialization
    let json = serde_json::to_string(&guide).unwrap();
    let deserialized: MigrationGuide = serde_json::from_str(&json).unwrap();
    
    assert_eq!(guide.id, deserialized.id);
    assert_eq!(guide.from_version, deserialized.from_version);
    assert_eq!(guide.to_version, deserialized.to_version);
    assert_eq!(guide.title, deserialized.title);
    assert_eq!(guide.complexity, deserialized.complexity);
    // Test debug formatting for estimated_time comparison
    let debug_str1 = format!("{:?}", guide.estimated_time);
    let debug_str2 = format!("{:?}", deserialized.estimated_time);
    assert_eq!(debug_str1, debug_str2);
    
    // Test YAML serialization
    let yaml = serde_yaml::to_string(&guide).unwrap();
    let deserialized_yaml: MigrationGuide = serde_yaml::from_str(&yaml).unwrap();
    
    assert_eq!(guide.id, deserialized_yaml.id);
    assert_eq!(guide.from_version, deserialized_yaml.from_version);
    assert_eq!(guide.to_version, deserialized_yaml.to_version);
}

#[test]
fn test_migration_guide_manager() {
    let mut manager = MigrationGuideManager::new();
    
    let guide = MigrationGuide {
        id: "guide-3".to_string(),
        from_version: "2.0.0".to_string(),
        to_version: "3.0.0".to_string(),
        title: "Major Migration".to_string(),
        description: "Major version migration".to_string(),
        complexity: MigrationComplexity::Critical,
        estimated_time: MigrationTime::Extensive,
        breaking_changes: vec![],
        steps: vec![],
        code_examples: vec![],
        pitfalls: vec![],
        rollback_instructions: Some("Complex rollback".to_string()),
        resources: vec![],
        checklist: vec![],
    };
    
    manager.register_guide(guide.clone()).unwrap();
    
    assert_eq!(manager.list_guides().len(), 1);
    assert!(manager.get_guide("guide-3").is_some());
    assert!(manager.get_guide("nonexistent").is_none());
    
    // Test getting guides by version range
    let guides_for_version = manager.get_guides_for_version_range("2.0.0", "3.0.0");
    assert_eq!(guides_for_version.len(), 1);
    assert_eq!(guides_for_version[0].id, "guide-3");
}

#[test]
fn test_checklist_category() {
    let code_changes = ChecklistCategory::CodeChanges;
    let config = ChecklistCategory::Configuration;
    let testing = ChecklistCategory::Testing;
    let deployment = ChecklistCategory::Deployment;
    let documentation = ChecklistCategory::Documentation;
    let team_coordination = ChecklistCategory::TeamCoordination;
    
    assert_eq!(code_changes, ChecklistCategory::CodeChanges);
    assert_eq!(config, ChecklistCategory::Configuration);
    assert_eq!(testing, ChecklistCategory::Testing);
    assert_eq!(deployment, ChecklistCategory::Deployment);
    assert_eq!(documentation, ChecklistCategory::Documentation);
    assert_eq!(team_coordination, ChecklistCategory::TeamCoordination);
    
    // Test debug formatting
    let debug_str = format!("{:?}", code_changes);
    assert!(debug_str.contains("CodeChanges"));
    
    let debug_str = format!("{:?}", team_coordination);
    assert!(debug_str.contains("TeamCoordination"));
}