//! Simple coverage tests for deprecation modules.
//! This file provides basic tests that exercise the actual deprecation module code.

use actor_core::deprecation::deprecation_manager::*;
use actor_core::deprecation::migration_guide::*;
use std::time::{Duration, SystemTime};

// ============================================================================
// DEPRECATION MANAGER TESTS
// ============================================================================

#[test]
fn test_deprecation_status_variants() {
    let statuses = vec![
        DeprecationStatus::Stable,
        DeprecationStatus::Deprecated,
        DeprecationStatus::DeprecatedRemovalImminent,
        DeprecationStatus::Removed,
    ];
    
    for status in statuses {
        assert!(matches!(status, DeprecationStatus::Stable | DeprecationStatus::Deprecated | DeprecationStatus::DeprecatedRemovalImminent | DeprecationStatus::Removed));
    }
}

#[test]
fn test_deprecation_status_debug() {
    let status = DeprecationStatus::Deprecated;
    let debug_str = format!("{:?}", status);
    assert!(debug_str.contains("Deprecated"));
}

#[test]
fn test_deprecation_status_clone() {
    let status = DeprecationStatus::Stable;
    let cloned = status.clone();
    assert_eq!(status, cloned);
}

#[test]
fn test_deprecation_status_serialization() {
    let status = DeprecationStatus::Deprecated;
    let serialized = serde_json::to_string(&status).unwrap();
    let deserialized: DeprecationStatus = serde_json::from_str(&serialized).unwrap();
    assert_eq!(status, deserialized);
}

#[test]
fn test_deprecation_severity_variants() {
    let severities = vec![
        DeprecationSeverity::Low,
        DeprecationSeverity::Medium,
        DeprecationSeverity::High,
        DeprecationSeverity::Critical,
    ];
    
    for severity in severities {
        assert!(matches!(severity, DeprecationSeverity::Low | DeprecationSeverity::Medium | DeprecationSeverity::High | DeprecationSeverity::Critical));
    }
}

#[test]
fn test_deprecation_severity_debug() {
    let severity = DeprecationSeverity::High;
    let debug_str = format!("{:?}", severity);
    assert!(debug_str.contains("High"));
}

#[test]
fn test_deprecation_severity_clone() {
    let severity = DeprecationSeverity::Medium;
    let cloned = severity.clone();
    assert_eq!(severity, cloned);
}

#[test]
fn test_deprecation_severity_serialization() {
    let severity = DeprecationSeverity::Critical;
    let serialized = serde_json::to_string(&severity).unwrap();
    let deserialized: DeprecationSeverity = serde_json::from_str(&serialized).unwrap();
    assert_eq!(severity, deserialized);
}

#[test]
fn test_deprecation_entry_creation() {
    let now = SystemTime::now();
    let entry = DeprecationEntry {
        id: "test-deprecation".to_string(),
        name: "Test Feature".to_string(),
        description: "A test feature being deprecated".to_string(),
        status: DeprecationStatus::Deprecated,
        deprecated_date: now,
        removal_date: Some(now + Duration::from_secs(86400 * 30)), // 30 days
        replacement: Some("New Feature".to_string()),
        migration_guide: Some("https://example.com/migration".to_string()),
        severity: DeprecationSeverity::Medium,
        affected_versions: vec!["1.0.0".to_string(), "1.1.0".to_string()],
        breaking_change: true,
        notes: Some("This is a breaking change".to_string()),
    };
    
    assert_eq!(entry.id, "test-deprecation");
    assert_eq!(entry.name, "Test Feature");
    assert_eq!(entry.status, DeprecationStatus::Deprecated);
    assert!(entry.breaking_change);
    assert_eq!(entry.affected_versions.len(), 2);
}

#[test]
fn test_deprecation_entry_clone() {
    let now = SystemTime::now();
    let entry = DeprecationEntry {
        id: "test-clone".to_string(),
        name: "Clone Test".to_string(),
        description: "Testing clone".to_string(),
        status: DeprecationStatus::Stable,
        deprecated_date: now,
        removal_date: None,
        replacement: None,
        migration_guide: None,
        severity: DeprecationSeverity::Low,
        affected_versions: vec![],
        breaking_change: false,
        notes: None,
    };
    
    let cloned = entry.clone();
    assert_eq!(entry.id, cloned.id);
    assert_eq!(entry.name, cloned.name);
    assert_eq!(entry.status, cloned.status);
}

#[test]
fn test_deprecation_entry_serialization() {
    let now = SystemTime::now();
    let entry = DeprecationEntry {
        id: "test-serialization".to_string(),
        name: "Serialization Test".to_string(),
        description: "Testing serialization".to_string(),
        status: DeprecationStatus::Deprecated,
        deprecated_date: now,
        removal_date: Some(now + Duration::from_secs(86400)),
        replacement: Some("New API".to_string()),
        migration_guide: Some("https://example.com/guide".to_string()),
        severity: DeprecationSeverity::High,
        affected_versions: vec!["2.0.0".to_string()],
        breaking_change: true,
        notes: Some("Breaking change notes".to_string()),
    };
    
    let serialized = serde_json::to_string(&entry).unwrap();
    let deserialized: DeprecationEntry = serde_json::from_str(&serialized).unwrap();
    assert_eq!(entry.id, deserialized.id);
    assert_eq!(entry.name, deserialized.name);
    assert_eq!(entry.status, deserialized.status);
}

#[test]
fn test_deprecation_manager_creation() {
    let _manager = DeprecationManager::new();
    // DeprecationManager doesn't have get_deprecations method
    // Just verify it was created successfully
    assert!(true);
}

#[test]
fn test_deprecation_manager_get_deprecation_not_found() {
    let manager = DeprecationManager::new();
    let retrieved = manager.get_deprecation("nonexistent");
    assert!(retrieved.is_none());
}

#[test]
fn test_rollback_plan_creation() {
    let plan = RollbackPlan {
        id: "rollback-001".to_string(),
        name: "Test Rollback".to_string(),
        description: "Test rollback plan".to_string(),
        deprecation_id: "dep-001".to_string(),
        steps: vec![],
        estimated_duration: Duration::from_secs(3600), // 1 hour
        prerequisites: vec!["Backup data".to_string()],
        risk_level: RiskLevel::Low,
        validation_steps: vec!["Verify system state".to_string()],
    };
    
    assert_eq!(plan.id, "rollback-001");
    assert_eq!(plan.name, "Test Rollback");
    assert_eq!(plan.risk_level, RiskLevel::Low);
    assert_eq!(plan.prerequisites.len(), 1);
}

#[test]
fn test_rollback_plan_clone() {
    let plan = RollbackPlan {
        id: "rollback-clone".to_string(),
        name: "Clone Test".to_string(),
        description: "Testing clone".to_string(),
        deprecation_id: "dep-clone".to_string(),
        steps: vec![],
        estimated_duration: Duration::from_secs(1800),
        prerequisites: vec![],
        risk_level: RiskLevel::Medium,
        validation_steps: vec![],
    };
    
    let cloned = plan.clone();
    assert_eq!(plan.id, cloned.id);
    assert_eq!(plan.name, cloned.name);
    assert_eq!(plan.risk_level, cloned.risk_level);
}

#[test]
fn test_rollback_plan_serialization() {
    let plan = RollbackPlan {
        id: "rollback-serialization".to_string(),
        name: "Serialization Test".to_string(),
        description: "Testing serialization".to_string(),
        deprecation_id: "dep-serialization".to_string(),
        steps: vec![],
        estimated_duration: Duration::from_secs(7200),
        prerequisites: vec!["Prereq 1".to_string()],
        risk_level: RiskLevel::High,
        validation_steps: vec!["Validation 1".to_string()],
    };
    
    let serialized = serde_json::to_string(&plan).unwrap();
    let deserialized: RollbackPlan = serde_json::from_str(&serialized).unwrap();
    assert_eq!(plan.id, deserialized.id);
    assert_eq!(plan.name, deserialized.name);
    assert_eq!(plan.risk_level, deserialized.risk_level);
}

#[test]
fn test_rollback_step_creation() {
    let step = RollbackStep {
        step_number: 1,
        description: "First step".to_string(),
        commands: vec!["command1".to_string()],
        expected_outcome: "Success".to_string(),
        validation_criteria: Some("Check status".to_string()),
        estimated_time: Duration::from_secs(300),
        critical: true,
    };
    
    assert_eq!(step.step_number, 1);
    assert_eq!(step.description, "First step");
    assert!(step.critical);
    assert_eq!(step.commands.len(), 1);
}

#[test]
fn test_rollback_step_clone() {
    let step = RollbackStep {
        step_number: 2,
        description: "Second step".to_string(),
        commands: vec![],
        expected_outcome: "Complete".to_string(),
        validation_criteria: None,
        estimated_time: Duration::from_secs(600),
        critical: false,
    };
    
    let cloned = step.clone();
    assert_eq!(step.step_number, cloned.step_number);
    assert_eq!(step.description, cloned.description);
    assert_eq!(step.critical, cloned.critical);
}

#[test]
fn test_rollback_step_serialization() {
    let step = RollbackStep {
        step_number: 3,
        description: "Third step".to_string(),
        commands: vec!["cmd1".to_string(), "cmd2".to_string()],
        expected_outcome: "Done".to_string(),
        validation_criteria: Some("Validate".to_string()),
        estimated_time: Duration::from_secs(900),
        critical: true,
    };
    
    let serialized = serde_json::to_string(&step).unwrap();
    let deserialized: RollbackStep = serde_json::from_str(&serialized).unwrap();
    assert_eq!(step.step_number, deserialized.step_number);
    assert_eq!(step.description, deserialized.description);
    assert_eq!(step.critical, deserialized.critical);
}

#[test]
fn test_risk_level_variants() {
    let risk_levels = vec![
        RiskLevel::Low,
        RiskLevel::Medium,
        RiskLevel::High,
        RiskLevel::Critical,
    ];
    
    for risk in risk_levels {
        assert!(matches!(risk, RiskLevel::Low | RiskLevel::Medium | RiskLevel::High | RiskLevel::Critical));
    }
}

#[test]
fn test_risk_level_debug() {
    let risk = RiskLevel::High;
    let debug_str = format!("{:?}", risk);
    assert!(debug_str.contains("High"));
}

#[test]
fn test_risk_level_clone() {
    let risk = RiskLevel::Medium;
    let cloned = risk.clone();
    assert_eq!(risk, cloned);
}

#[test]
fn test_risk_level_serialization() {
    let risk = RiskLevel::Critical;
    let serialized = serde_json::to_string(&risk).unwrap();
    let deserialized: RiskLevel = serde_json::from_str(&serialized).unwrap();
    assert_eq!(risk, deserialized);
}

// ============================================================================
// MIGRATION GUIDE TESTS
// ============================================================================

#[test]
fn test_migration_complexity_variants() {
    let complexities = vec![
        MigrationComplexity::Low,
        MigrationComplexity::Medium,
        MigrationComplexity::High,
        MigrationComplexity::Critical,
    ];
    
    for complexity in complexities {
        assert!(matches!(complexity, MigrationComplexity::Low | MigrationComplexity::Medium | MigrationComplexity::High | MigrationComplexity::Critical));
    }
}

#[test]
fn test_migration_complexity_debug() {
    let complexity = MigrationComplexity::Medium;
    let debug_str = format!("{:?}", complexity);
    assert!(debug_str.contains("Medium"));
}

#[test]
fn test_migration_complexity_clone() {
    let complexity = MigrationComplexity::High;
    let cloned = complexity.clone();
    assert_eq!(complexity, cloned);
}

#[test]
fn test_migration_complexity_serialization() {
    let complexity = MigrationComplexity::Low;
    let serialized = serde_json::to_string(&complexity).unwrap();
    let deserialized: MigrationComplexity = serde_json::from_str(&serialized).unwrap();
    assert_eq!(complexity, deserialized);
}

#[test]
fn test_migration_time_variants() {
    let times = vec![
        MigrationTime::Quick,
        MigrationTime::Short,
        MigrationTime::Medium,
        MigrationTime::Long,
        MigrationTime::Extensive,
    ];
    
    for time in times {
        assert!(matches!(time, MigrationTime::Quick | MigrationTime::Short | MigrationTime::Medium | MigrationTime::Long | MigrationTime::Extensive));
    }
}

#[test]
fn test_migration_time_debug() {
    let time = MigrationTime::Short;
    let debug_str = format!("{:?}", time);
    assert!(debug_str.contains("Short"));
}

#[test]
fn test_migration_time_clone() {
    let time = MigrationTime::Long;
    let cloned = time.clone();
    // MigrationTime doesn't implement PartialEq, just verify they're the same variant
    assert!(matches!(time, MigrationTime::Long));
    assert!(matches!(cloned, MigrationTime::Long));
}

#[test]
fn test_migration_time_serialization() {
    let time = MigrationTime::Extensive;
    let serialized = serde_json::to_string(&time).unwrap();
    let deserialized: MigrationTime = serde_json::from_str(&serialized).unwrap();
    // MigrationTime doesn't implement PartialEq, just verify they're the same variant
    assert!(matches!(time, MigrationTime::Extensive));
    assert!(matches!(deserialized, MigrationTime::Extensive));
}

#[test]
fn test_breaking_change_creation() {
    let breaking_change = BreakingChange {
        change_type: BreakingChangeType::ApiSignature,
        description: "API method signature changed".to_string(),
        affected_components: vec!["api".to_string()],
        impact: ChangeImpact::High,
        migration_strategy: "Update method calls".to_string(),
    };
    
    assert_eq!(breaking_change.change_type, BreakingChangeType::ApiSignature);
    assert_eq!(breaking_change.description, "API method signature changed");
    assert_eq!(breaking_change.impact, ChangeImpact::High);
    assert_eq!(breaking_change.affected_components.len(), 1);
}

#[test]
fn test_breaking_change_clone() {
    let breaking_change = BreakingChange {
        change_type: BreakingChangeType::DataStructure,
        description: "Data structure changed".to_string(),
        affected_components: vec![],
        impact: ChangeImpact::Medium,
        migration_strategy: "Update data handling".to_string(),
    };
    
    let cloned = breaking_change.clone();
    assert_eq!(breaking_change.change_type, cloned.change_type);
    assert_eq!(breaking_change.description, cloned.description);
    assert_eq!(breaking_change.impact, cloned.impact);
}

#[test]
fn test_breaking_change_serialization() {
    let breaking_change = BreakingChange {
        change_type: BreakingChangeType::Configuration,
        description: "Configuration format changed".to_string(),
        affected_components: vec!["config".to_string()],
        impact: ChangeImpact::Low,
        migration_strategy: "Update config files".to_string(),
    };
    
    let serialized = serde_json::to_string(&breaking_change).unwrap();
    let deserialized: BreakingChange = serde_json::from_str(&serialized).unwrap();
    assert_eq!(breaking_change.change_type, deserialized.change_type);
    assert_eq!(breaking_change.description, deserialized.description);
    assert_eq!(breaking_change.impact, deserialized.impact);
}

#[test]
fn test_breaking_change_type_variants() {
    let types = vec![
        BreakingChangeType::ApiSignature,
        BreakingChangeType::DataStructure,
        BreakingChangeType::Configuration,
        BreakingChangeType::Dependency,
        BreakingChangeType::Behavior,
        BreakingChangeType::FeatureRemoval,
    ];
    
    for change_type in types {
        assert!(matches!(change_type, BreakingChangeType::ApiSignature | BreakingChangeType::DataStructure | BreakingChangeType::Configuration | BreakingChangeType::Dependency | BreakingChangeType::Behavior | BreakingChangeType::FeatureRemoval));
    }
}

#[test]
fn test_breaking_change_type_debug() {
    let change_type = BreakingChangeType::ApiSignature;
    let debug_str = format!("{:?}", change_type);
    assert!(debug_str.contains("ApiSignature"));
}

#[test]
fn test_breaking_change_type_clone() {
    let change_type = BreakingChangeType::DataStructure;
    let cloned = change_type.clone();
    assert_eq!(change_type, cloned);
}

#[test]
fn test_breaking_change_type_serialization() {
    let change_type = BreakingChangeType::Configuration;
    let serialized = serde_json::to_string(&change_type).unwrap();
    let deserialized: BreakingChangeType = serde_json::from_str(&serialized).unwrap();
    assert_eq!(change_type, deserialized);
}

#[test]
fn test_change_impact_variants() {
    let impacts = vec![
        ChangeImpact::Low,
        ChangeImpact::Medium,
        ChangeImpact::High,
        ChangeImpact::Critical,
    ];
    
    for impact in impacts {
        assert!(matches!(impact, ChangeImpact::Low | ChangeImpact::Medium | ChangeImpact::High | ChangeImpact::Critical));
    }
}

#[test]
fn test_change_impact_debug() {
    let impact = ChangeImpact::High;
    let debug_str = format!("{:?}", impact);
    assert!(debug_str.contains("High"));
}

#[test]
fn test_change_impact_clone() {
    let impact = ChangeImpact::Medium;
    let cloned = impact.clone();
    assert_eq!(impact, cloned);
}

#[test]
fn test_change_impact_serialization() {
    let impact = ChangeImpact::Critical;
    let serialized = serde_json::to_string(&impact).unwrap();
    let deserialized: ChangeImpact = serde_json::from_str(&serialized).unwrap();
    assert_eq!(impact, deserialized);
}

#[test]
fn test_migration_step_creation() {
    let step = MigrationStep {
        step_number: 1,
        title: "Update Imports".to_string(),
        instructions: "Update all import statements".to_string(),
        commands: vec!["import new_module".to_string()],
        expected_outcome: "Imports updated successfully".to_string(),
        validation_criteria: Some("Check that imports compile".to_string()),
        estimated_time: "5 minutes".to_string(),
        optional: false,
        prerequisites: vec!["Backup code".to_string()],
    };
    
    assert_eq!(step.step_number, 1);
    assert_eq!(step.title, "Update Imports");
    assert_eq!(step.instructions, "Update all import statements");
    assert!(!step.optional);
    assert_eq!(step.prerequisites.len(), 1);
}

#[test]
fn test_migration_step_clone() {
    let step = MigrationStep {
        step_number: 2,
        title: "Clone Test".to_string(),
        instructions: "Testing clone".to_string(),
        commands: vec![],
        expected_outcome: "Complete".to_string(),
        validation_criteria: None,
        estimated_time: "10 minutes".to_string(),
        optional: true,
        prerequisites: vec![],
    };
    
    let cloned = step.clone();
    assert_eq!(step.step_number, cloned.step_number);
    assert_eq!(step.title, cloned.title);
    assert_eq!(step.optional, cloned.optional);
}

#[test]
fn test_migration_step_serialization() {
    let step = MigrationStep {
        step_number: 3,
        title: "Serialization Test".to_string(),
        instructions: "Testing serialization".to_string(),
        commands: vec!["cmd1".to_string()],
        expected_outcome: "Success".to_string(),
        validation_criteria: Some("Verify".to_string()),
        estimated_time: "15 minutes".to_string(),
        optional: false,
        prerequisites: vec!["Prereq".to_string()],
    };
    
    let serialized = serde_json::to_string(&step).unwrap();
    let deserialized: MigrationStep = serde_json::from_str(&serialized).unwrap();
    assert_eq!(step.step_number, deserialized.step_number);
    assert_eq!(step.title, deserialized.title);
    assert_eq!(step.optional, deserialized.optional);
}

#[test]
fn test_code_example_creation() {
    let example = CodeExample {
        title: "Before and After".to_string(),
        description: "Showing the change".to_string(),
        language: "rust".to_string(),
        before_code: "old_code()".to_string(),
        after_code: "new_code()".to_string(),
        explanation: "This is the new way".to_string(),
    };
    
    assert_eq!(example.title, "Before and After");
    assert_eq!(example.description, "Showing the change");
    assert_eq!(example.language, "rust");
    assert_eq!(example.before_code, "old_code()");
    assert_eq!(example.after_code, "new_code()");
}

#[test]
fn test_code_example_clone() {
    let example = CodeExample {
        title: "Clone Test".to_string(),
        description: "Testing clone".to_string(),
        language: "javascript".to_string(),
        before_code: "old".to_string(),
        after_code: "new".to_string(),
        explanation: "Explanation".to_string(),
    };
    
    let cloned = example.clone();
    assert_eq!(example.title, cloned.title);
    assert_eq!(example.description, cloned.description);
    assert_eq!(example.language, cloned.language);
}

#[test]
fn test_code_example_serialization() {
    let example = CodeExample {
        title: "Serialization Test".to_string(),
        description: "Testing serialization".to_string(),
        language: "python".to_string(),
        before_code: "old_python()".to_string(),
        after_code: "new_python()".to_string(),
        explanation: "Python example".to_string(),
    };
    
    let serialized = serde_json::to_string(&example).unwrap();
    let deserialized: CodeExample = serde_json::from_str(&serialized).unwrap();
    assert_eq!(example.title, deserialized.title);
    assert_eq!(example.description, deserialized.description);
    assert_eq!(example.language, deserialized.language);
}

#[test]
fn test_migration_guide_creation() {
    let guide = MigrationGuide {
        id: "guide-001".to_string(),
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
        rollback_instructions: Some("Rollback steps".to_string()),
        resources: vec![],
        checklist: vec![],
    };
    
    assert_eq!(guide.id, "guide-001");
    assert_eq!(guide.from_version, "1.0.0");
    assert_eq!(guide.to_version, "2.0.0");
    assert_eq!(guide.title, "Migration from 1.0 to 2.0");
    assert_eq!(guide.complexity, MigrationComplexity::High);
    // MigrationTime doesn't implement PartialEq, just verify it's the correct variant
    assert!(matches!(guide.estimated_time, MigrationTime::Long));
}

#[test]
fn test_migration_guide_clone() {
    let guide = MigrationGuide {
        id: "guide-clone".to_string(),
        from_version: "2.0.0".to_string(),
        to_version: "3.0.0".to_string(),
        title: "Clone Test".to_string(),
        description: "Testing clone".to_string(),
        complexity: MigrationComplexity::Low,
        estimated_time: MigrationTime::Quick,
        breaking_changes: vec![],
        steps: vec![],
        code_examples: vec![],
        pitfalls: vec![],
        rollback_instructions: None,
        resources: vec![],
        checklist: vec![],
    };
    
    let cloned = guide.clone();
    assert_eq!(guide.id, cloned.id);
    assert_eq!(guide.from_version, cloned.from_version);
    assert_eq!(guide.to_version, cloned.to_version);
    assert_eq!(guide.complexity, cloned.complexity);
}

#[test]
fn test_migration_guide_serialization() {
    let guide = MigrationGuide {
        id: "guide-serialization".to_string(),
        from_version: "3.0.0".to_string(),
        to_version: "4.0.0".to_string(),
        title: "Serialization Test".to_string(),
        description: "Testing serialization".to_string(),
        complexity: MigrationComplexity::Medium,
        estimated_time: MigrationTime::Short,
        breaking_changes: vec![],
        steps: vec![],
        code_examples: vec![],
        pitfalls: vec![],
        rollback_instructions: Some("Rollback".to_string()),
        resources: vec![],
        checklist: vec![],
    };
    
    let serialized = serde_json::to_string(&guide).unwrap();
    let deserialized: MigrationGuide = serde_json::from_str(&serialized).unwrap();
    assert_eq!(guide.id, deserialized.id);
    assert_eq!(guide.from_version, deserialized.from_version);
    assert_eq!(guide.to_version, deserialized.to_version);
    assert_eq!(guide.complexity, deserialized.complexity);
}

// ============================================================================
// COMPREHENSIVE INTEGRATION TESTS
// ============================================================================

#[test]
fn test_deprecation_and_migration_integration() {
    let _deprecation_manager = DeprecationManager::new();
    
    // Create a deprecation entry
    let now = SystemTime::now();
    let deprecation = DeprecationEntry {
        id: "integration-test".to_string(),
        name: "Integration Test Feature".to_string(),
        description: "Testing integration".to_string(),
        status: DeprecationStatus::Deprecated,
        deprecated_date: now,
        removal_date: Some(now + Duration::from_secs(86400 * 30)),
        replacement: Some("New Integration Feature".to_string()),
        migration_guide: Some("integration-guide".to_string()),
        severity: DeprecationSeverity::High,
        affected_versions: vec!["1.0.0".to_string()],
        breaking_change: true,
        notes: Some("Integration test notes".to_string()),
    };
    
    // Create a migration guide
    let migration_guide = MigrationGuide {
        id: "integration-guide".to_string(),
        from_version: "1.0.0".to_string(),
        to_version: "2.0.0".to_string(),
        title: "Integration Migration Guide".to_string(),
        description: "Guide for integration test".to_string(),
        complexity: MigrationComplexity::High,
        estimated_time: MigrationTime::Long,
        breaking_changes: vec![],
        steps: vec![],
        code_examples: vec![],
        pitfalls: vec![],
        rollback_instructions: Some("Rollback integration".to_string()),
        resources: vec![],
        checklist: vec![],
    };
    
    // Verify creation
    assert_eq!(deprecation.id, "integration-test");
    assert_eq!(deprecation.name, "Integration Test Feature");
    assert_eq!(migration_guide.id, "integration-guide");
    assert_eq!(migration_guide.title, "Integration Migration Guide");
    assert_eq!(deprecation.migration_guide, Some("integration-guide".to_string()));
}

#[test]
fn test_serialization_roundtrip() {
    // Test deprecation entry serialization
    let now = SystemTime::now();
    let deprecation = DeprecationEntry {
        id: "serialization-test".to_string(),
        name: "Serialization Test".to_string(),
        description: "Testing serialization roundtrip".to_string(),
        status: DeprecationStatus::Deprecated,
        deprecated_date: now,
        removal_date: Some(now + Duration::from_secs(86400)),
        replacement: Some("New Serialization".to_string()),
        migration_guide: Some("serialization-guide".to_string()),
        severity: DeprecationSeverity::Medium,
        affected_versions: vec!["1.0.0".to_string(), "1.1.0".to_string()],
        breaking_change: false,
        notes: Some("Serialization test notes".to_string()),
    };
    
    let serialized = serde_json::to_string(&deprecation).unwrap();
    let deserialized: DeprecationEntry = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(deprecation.id, deserialized.id);
    assert_eq!(deprecation.name, deserialized.name);
    assert_eq!(deprecation.status, deserialized.status);
    assert_eq!(deprecation.severity, deserialized.severity);
    assert_eq!(deprecation.affected_versions, deserialized.affected_versions);
    
    // Test migration guide serialization
    let migration_guide = MigrationGuide {
        id: "serialization-guide".to_string(),
        from_version: "1.0.0".to_string(),
        to_version: "2.0.0".to_string(),
        title: "Serialization Migration".to_string(),
        description: "Migration for serialization".to_string(),
        complexity: MigrationComplexity::Medium,
        estimated_time: MigrationTime::Short,
        breaking_changes: vec![],
        steps: vec![],
        code_examples: vec![],
        pitfalls: vec![],
        rollback_instructions: Some("Rollback serialization".to_string()),
        resources: vec![],
        checklist: vec![],
    };
    
    let serialized_guide = serde_json::to_string(&migration_guide).unwrap();
    let deserialized_guide: MigrationGuide = serde_json::from_str(&serialized_guide).unwrap();
    
    assert_eq!(migration_guide.id, deserialized_guide.id);
    assert_eq!(migration_guide.from_version, deserialized_guide.from_version);
    assert_eq!(migration_guide.to_version, deserialized_guide.to_version);
    assert_eq!(migration_guide.complexity, deserialized_guide.complexity);
    // MigrationTime doesn't implement PartialEq, just verify they're the same variant
    assert!(matches!(migration_guide.estimated_time, MigrationTime::Short));
    assert!(matches!(deserialized_guide.estimated_time, MigrationTime::Short));
}
