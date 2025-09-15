//! Direct coverage tests for api_stability.rs module.
//! This file provides direct tests that exercise the actual api_stability module code.

use actor_core::api_stability::*;

// ============================================================================
// STABILITY LEVEL TESTS
// ============================================================================

#[test]
fn test_stability_level_variants() {
    let stable = StabilityLevel::Stable;
    let beta = StabilityLevel::Beta;
    let alpha = StabilityLevel::Alpha;
    let internal = StabilityLevel::Internal;
    
    assert_eq!(stable, StabilityLevel::Stable);
    assert_eq!(beta, StabilityLevel::Beta);
    assert_eq!(alpha, StabilityLevel::Alpha);
    assert_eq!(internal, StabilityLevel::Internal);
}

#[test]
fn test_stability_level_debug() {
    let stable = StabilityLevel::Stable;
    let debug_string = format!("{:?}", stable);
    assert!(debug_string.contains("Stable"));
}

#[test]
fn test_stability_level_clone() {
    let stable = StabilityLevel::Stable;
    let cloned = stable.clone();
    assert_eq!(stable, cloned);
}

#[test]
fn test_stability_level_copy() {
    let stable = StabilityLevel::Stable;
    let copied = stable;
    assert_eq!(stable, copied);
}

#[test]
fn test_stability_level_partial_eq() {
    let stable1 = StabilityLevel::Stable;
    let stable2 = StabilityLevel::Stable;
    let beta = StabilityLevel::Beta;
    
    assert_eq!(stable1, stable2);
    assert_ne!(stable1, beta);
}

#[test]
fn test_stability_level_eq() {
    let stable1 = StabilityLevel::Stable;
    let stable2 = StabilityLevel::Stable;
    assert_eq!(stable1, stable2);
}

#[test]
fn test_stability_level_hash() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert(StabilityLevel::Stable, "stable");
    map.insert(StabilityLevel::Beta, "beta");
    
    assert_eq!(map.get(&StabilityLevel::Stable), Some(&"stable"));
    assert_eq!(map.get(&StabilityLevel::Beta), Some(&"beta"));
}

// ============================================================================
// API VERSION TESTS
// ============================================================================

#[test]
fn test_api_version_creation() {
    let version = ApiVersion::new(1, 2, 3);
    
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
}

#[test]
fn test_api_version_current() {
    let current = ApiVersion::current();
    
    assert_eq!(current.major, 1);
    assert_eq!(current.minor, 0);
    assert_eq!(current.patch, 0);
}

#[test]
fn test_api_version_is_compatible_with() {
    let version1 = ApiVersion::new(1, 0, 0);
    let version2 = ApiVersion::new(1, 1, 0);
    let version3 = ApiVersion::new(2, 0, 0);
    
    assert!(version1.is_compatible_with(&version2));
    assert!(version2.is_compatible_with(&version1));
    assert!(!version1.is_compatible_with(&version3));
    assert!(!version3.is_compatible_with(&version1));
}

#[test]
fn test_api_version_to_string() {
    let version = ApiVersion::new(1, 2, 3);
    let version_string = version.to_string();
    
    assert_eq!(version_string, "1.2.3");
}

#[test]
fn test_api_version_debug() {
    let version = ApiVersion::new(1, 2, 3);
    let debug_string = format!("{:?}", version);
    assert!(debug_string.contains("1"));
    assert!(debug_string.contains("2"));
    assert!(debug_string.contains("3"));
}

#[test]
fn test_api_version_clone() {
    let version = ApiVersion::new(1, 2, 3);
    let cloned = version.clone();
    
    assert_eq!(version.major, cloned.major);
    assert_eq!(version.minor, cloned.minor);
    assert_eq!(version.patch, cloned.patch);
}

#[test]
fn test_api_version_partial_eq() {
    let version1 = ApiVersion::new(1, 2, 3);
    let version2 = ApiVersion::new(1, 2, 3);
    let version3 = ApiVersion::new(1, 2, 4);
    
    assert_eq!(version1, version2);
    assert_ne!(version1, version3);
}

#[test]
fn test_api_version_eq() {
    let version1 = ApiVersion::new(1, 2, 3);
    let version2 = ApiVersion::new(1, 2, 3);
    assert_eq!(version1, version2);
}

// ============================================================================
// API COMPONENT TESTS
// ============================================================================

#[test]
fn test_api_component_creation() {
    let component = ApiComponent::new(
        "TestComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Test description",
    );
    
    assert_eq!(component.name, "TestComponent");
    assert_eq!(component.stability, StabilityLevel::Stable);
    assert_eq!(component.introduced_in.major, 1);
    assert_eq!(component.introduced_in.minor, 0);
    assert_eq!(component.introduced_in.patch, 0);
    assert_eq!(component.description, "Test description");
    assert!(component.deprecated_in.is_none());
    assert!(component.removed_in.is_none());
}

#[test]
fn test_api_component_deprecate_in() {
    let component = ApiComponent::new(
        "TestComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Test description",
    ).deprecate_in(ApiVersion::new(2, 0, 0));
    
    assert!(component.deprecated_in.is_some());
    assert_eq!(component.deprecated_in.unwrap().major, 2);
    assert!(component.removed_in.is_none());
}

#[test]
fn test_api_component_remove_in() {
    let component = ApiComponent::new(
        "TestComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Test description",
    ).remove_in(ApiVersion::new(3, 0, 0));
    
    assert!(component.removed_in.is_some());
    assert_eq!(component.removed_in.unwrap().major, 3);
    assert!(component.deprecated_in.is_none());
}

#[test]
fn test_api_component_is_stable() {
    let stable_component = ApiComponent::new(
        "StableComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Stable description",
    );
    
    let beta_component = ApiComponent::new(
        "BetaComponent",
        StabilityLevel::Beta,
        ApiVersion::new(1, 0, 0),
        "Beta description",
    );
    
    assert!(stable_component.is_stable());
    assert!(!beta_component.is_stable());
}

#[test]
fn test_api_component_is_deprecated() {
    let deprecated_component = ApiComponent::new(
        "DeprecatedComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Deprecated description",
    ).deprecate_in(ApiVersion::new(2, 0, 0));
    
    let non_deprecated_component = ApiComponent::new(
        "NonDeprecatedComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Non-deprecated description",
    );
    
    assert!(deprecated_component.is_deprecated());
    assert!(!non_deprecated_component.is_deprecated());
}

#[test]
fn test_api_component_is_removed() {
    let removed_component = ApiComponent::new(
        "RemovedComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Removed description",
    ).remove_in(ApiVersion::new(3, 0, 0));
    
    let non_removed_component = ApiComponent::new(
        "NonRemovedComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Non-removed description",
    );
    
    assert!(removed_component.is_removed());
    assert!(!non_removed_component.is_removed());
}

#[test]
fn test_api_component_debug() {
    let component = ApiComponent::new(
        "TestComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Test description",
    );
    
    let debug_string = format!("{:?}", component);
    assert!(debug_string.contains("TestComponent"));
    assert!(debug_string.contains("Stable"));
}

#[test]
fn test_api_component_clone() {
    let component = ApiComponent::new(
        "TestComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Test description",
    );
    
    let cloned = component.clone();
    assert_eq!(component.name, cloned.name);
    assert_eq!(component.stability, cloned.stability);
    assert_eq!(component.description, cloned.description);
}

// ============================================================================
// API REGISTRY TESTS
// ============================================================================

#[test]
fn test_api_registry_new() {
    let registry = ApiRegistry::new();
    
    assert_eq!(registry.all().len(), 0);
}

#[test]
fn test_api_registry_register() {
    let component = ApiComponent::new(
        "TestComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Test description",
    );
    
    let registry = ApiRegistry::new().register(component);
    
    assert_eq!(registry.all().len(), 1);
    assert_eq!(registry.all()[0].name, "TestComponent");
}

#[test]
fn test_api_registry_get_by_stability() {
    let stable_component = ApiComponent::new(
        "StableComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Stable description",
    );
    
    let beta_component = ApiComponent::new(
        "BetaComponent",
        StabilityLevel::Beta,
        ApiVersion::new(1, 0, 0),
        "Beta description",
    );
    
    let registry = ApiRegistry::new()
        .register(stable_component)
        .register(beta_component);
    
    let stable_components = registry.get_by_stability(StabilityLevel::Stable);
    let beta_components = registry.get_by_stability(StabilityLevel::Beta);
    
    assert_eq!(stable_components.len(), 1);
    assert_eq!(beta_components.len(), 1);
    assert_eq!(stable_components[0].name, "StableComponent");
    assert_eq!(beta_components[0].name, "BetaComponent");
}

#[test]
fn test_api_registry_get_stable() {
    let stable_component = ApiComponent::new(
        "StableComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Stable description",
    );
    
    let beta_component = ApiComponent::new(
        "BetaComponent",
        StabilityLevel::Beta,
        ApiVersion::new(1, 0, 0),
        "Beta description",
    );
    
    let registry = ApiRegistry::new()
        .register(stable_component)
        .register(beta_component);
    
    let stable_components = registry.get_stable();
    
    assert_eq!(stable_components.len(), 1);
    assert_eq!(stable_components[0].name, "StableComponent");
}

#[test]
fn test_api_registry_get_beta() {
    let stable_component = ApiComponent::new(
        "StableComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Stable description",
    );
    
    let beta_component = ApiComponent::new(
        "BetaComponent",
        StabilityLevel::Beta,
        ApiVersion::new(1, 0, 0),
        "Beta description",
    );
    
    let registry = ApiRegistry::new()
        .register(stable_component)
        .register(beta_component);
    
    let beta_components = registry.get_beta();
    
    assert_eq!(beta_components.len(), 1);
    assert_eq!(beta_components[0].name, "BetaComponent");
}

#[test]
fn test_api_registry_get_alpha() {
    let alpha_component = ApiComponent::new(
        "AlphaComponent",
        StabilityLevel::Alpha,
        ApiVersion::new(1, 0, 0),
        "Alpha description",
    );
    
    let registry = ApiRegistry::new().register(alpha_component);
    
    let alpha_components = registry.get_alpha();
    
    assert_eq!(alpha_components.len(), 1);
    assert_eq!(alpha_components[0].name, "AlphaComponent");
}

#[test]
fn test_api_registry_find() {
    let component1 = ApiComponent::new(
        "Component1",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Description 1",
    );
    
    let component2 = ApiComponent::new(
        "Component2",
        StabilityLevel::Beta,
        ApiVersion::new(1, 0, 0),
        "Description 2",
    );
    
    let registry = ApiRegistry::new()
        .register(component1)
        .register(component2);
    
    let found = registry.find("Component1");
    let not_found = registry.find("NonExistentComponent");
    
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Component1");
    assert!(not_found.is_none());
}

#[test]
fn test_api_registry_all() {
    let component1 = ApiComponent::new(
        "Component1",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Description 1",
    );
    
    let component2 = ApiComponent::new(
        "Component2",
        StabilityLevel::Beta,
        ApiVersion::new(1, 0, 0),
        "Description 2",
    );
    
    let registry = ApiRegistry::new()
        .register(component1)
        .register(component2);
    
    let all_components = registry.all();
    
    assert_eq!(all_components.len(), 2);
    assert_eq!(all_components[0].name, "Component1");
    assert_eq!(all_components[1].name, "Component2");
}

// ============================================================================
// GLOBAL API REGISTRY TESTS
// ============================================================================

#[test]
fn test_get_api_registry() {
    let registry = get_api_registry();
    
    // Should have many components registered
    assert!(!registry.all().is_empty());
    
    // Should have stable components
    let stable_components = registry.get_stable();
    assert!(!stable_components.is_empty());
    
    // Should have beta components
    let beta_components = registry.get_beta();
    assert!(!beta_components.is_empty());
}

#[test]
fn test_get_api_registry_contains_core_types() {
    let registry = get_api_registry();
    
    // Check for core stable types
    assert!(registry.find("Actor").is_some());
    assert!(registry.find("Contribution").is_some());
    assert!(registry.find("CapContribution").is_some());
    assert!(registry.find("Snapshot").is_some());
    assert!(registry.find("Caps").is_some());
}

#[test]
fn test_get_api_registry_contains_enums() {
    let registry = get_api_registry();
    
    // Check for enum types
    assert!(registry.find("Bucket").is_some());
    assert!(registry.find("CapMode").is_some());
    assert!(registry.find("CapKind").is_some());
    assert!(registry.find("AcrossLayerPolicy").is_some());
    assert!(registry.find("Operator").is_some());
}

#[test]
fn test_get_api_registry_contains_traits() {
    let registry = get_api_registry();
    
    // Check for trait types
    assert!(registry.find("Subsystem").is_some());
    assert!(registry.find("Aggregator").is_some());
    assert!(registry.find("CapsProvider").is_some());
    assert!(registry.find("PluginRegistry").is_some());
    assert!(registry.find("CombinerRegistry").is_some());
    assert!(registry.find("Cache").is_some());
}

#[test]
fn test_get_api_registry_contains_error_types() {
    let registry = get_api_registry();
    
    // Check for error types
    assert!(registry.find("ActorCoreError").is_some());
    assert!(registry.find("ActorCoreResult").is_some());
}

#[test]
fn test_get_api_registry_contains_services() {
    let registry = get_api_registry();
    
    // Check for service types
    assert!(registry.find("ServiceFactory").is_some());
    assert!(registry.find("prelude").is_some());
}

#[test]
fn test_get_api_registry_contains_performance_components() {
    let registry = get_api_registry();
    
    // Check for performance components (Beta)
    assert!(registry.find("PerformanceProfiler").is_some());
    assert!(registry.find("PerformanceTestSuite").is_some());
    assert!(registry.find("PerformanceWorkflowManager").is_some());
}

#[test]
fn test_get_api_registry_contains_observability_components() {
    let registry = get_api_registry();
    
    // Check for observability components (Beta)
    assert!(registry.find("ObservabilityManager").is_some());
}

#[test]
fn test_get_api_registry_contains_metrics_components() {
    let registry = get_api_registry();
    
    // Check for metrics components (Beta)
    assert!(registry.find("SubsystemMetrics").is_some());
    assert!(registry.find("AggregatorMetrics").is_some());
    assert!(registry.find("CacheStats").is_some());
}

#[test]
fn test_get_api_registry_contains_internal_components() {
    let registry = get_api_registry();
    
    // Check for internal components
    assert!(registry.find("bucket_processor").is_some());
    assert!(registry.find("aggregator").is_some());
    assert!(registry.find("cache").is_some());
    assert!(registry.find("registry").is_some());
    assert!(registry.find("production").is_some());
}

// ============================================================================
// UTILITY FUNCTION TESTS
// ============================================================================

#[test]
fn test_check_compatibility() {
    let version1 = ApiVersion::new(1, 0, 0);
    let version2 = ApiVersion::new(1, 1, 0);
    let version3 = ApiVersion::new(2, 0, 0);
    
    assert!(check_compatibility(&version1, &version2));
    assert!(check_compatibility(&version2, &version1));
    assert!(!check_compatibility(&version1, &version3));
    assert!(!check_compatibility(&version3, &version1));
}

#[test]
fn test_get_stability_report() {
    let report = get_stability_report();
    
    // Should contain the header
    assert!(report.contains("# Actor Core API Stability Report"));
    assert!(report.contains("Generated for version: 1.0.0"));
    
    // Should contain stability sections
    assert!(report.contains("## Stable API (v1.0.0+)"));
    assert!(report.contains("## Beta API (may change in minor versions)"));
    assert!(report.contains("## Alpha API (experimental)"));
    
    // Should contain stability guarantees
    assert!(report.contains("## Stability Guarantees"));
    assert!(report.contains("**Stable**: Guaranteed to remain compatible"));
    assert!(report.contains("**Beta**: May change in minor versions"));
    assert!(report.contains("**Alpha**: Experimental"));
    assert!(report.contains("**Internal**: Not part of the public API"));
    
    // Should contain some component names
    assert!(report.contains("**Actor**:"));
    assert!(report.contains("**Contribution**:"));
    assert!(report.contains("**PerformanceProfiler**:"));
}

// ============================================================================
// COMPREHENSIVE USAGE TESTS
// ============================================================================

#[test]
fn test_api_stability_workflow() {
    // Create a custom registry
    let custom_component = ApiComponent::new(
        "CustomComponent",
        StabilityLevel::Alpha,
        ApiVersion::new(1, 0, 0),
        "Custom description",
    ).deprecate_in(ApiVersion::new(2, 0, 0));
    
    let registry = ApiRegistry::new().register(custom_component);
    
    // Test finding the component
    let found = registry.find("CustomComponent");
    assert!(found.is_some());
    let component = found.unwrap();
    
    // Test component properties
    assert_eq!(component.name, "CustomComponent");
    assert_eq!(component.stability, StabilityLevel::Alpha);
    assert!(component.is_deprecated());
    assert!(!component.is_stable());
    assert!(!component.is_removed());
    
    // Test getting by stability
    let alpha_components = registry.get_by_stability(StabilityLevel::Alpha);
    assert_eq!(alpha_components.len(), 1);
    assert_eq!(alpha_components[0].name, "CustomComponent");
}

#[test]
fn test_version_compatibility_workflow() {
    let current_version = ApiVersion::current();
    let future_version = ApiVersion::new(1, 5, 0);
    let breaking_version = ApiVersion::new(2, 0, 0);
    
    // Test compatibility checks
    assert!(current_version.is_compatible_with(&future_version));
    assert!(future_version.is_compatible_with(&current_version));
    assert!(!current_version.is_compatible_with(&breaking_version));
    assert!(!breaking_version.is_compatible_with(&current_version));
    
    // Test utility function
    assert!(check_compatibility(&current_version, &future_version));
    assert!(!check_compatibility(&current_version, &breaking_version));
}

#[test]
fn test_stability_level_ordering() {
    use std::collections::HashMap;
    
    let mut stability_map = HashMap::new();
    stability_map.insert(StabilityLevel::Stable, 1);
    stability_map.insert(StabilityLevel::Beta, 2);
    stability_map.insert(StabilityLevel::Alpha, 3);
    stability_map.insert(StabilityLevel::Internal, 4);
    
    assert_eq!(stability_map.get(&StabilityLevel::Stable), Some(&1));
    assert_eq!(stability_map.get(&StabilityLevel::Beta), Some(&2));
    assert_eq!(stability_map.get(&StabilityLevel::Alpha), Some(&3));
    assert_eq!(stability_map.get(&StabilityLevel::Internal), Some(&4));
}

#[test]
fn test_api_component_chain_operations() {
    let component = ApiComponent::new(
        "ChainedComponent",
        StabilityLevel::Stable,
        ApiVersion::new(1, 0, 0),
        "Chained description",
    )
    .deprecate_in(ApiVersion::new(2, 0, 0))
    .remove_in(ApiVersion::new(3, 0, 0));
    
    assert!(component.is_deprecated());
    assert!(component.is_removed());
    assert!(component.is_stable());
}

#[test]
fn test_api_registry_chaining() {
    let registry = ApiRegistry::new()
        .register(ApiComponent::new(
            "Component1",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Description 1",
        ))
        .register(ApiComponent::new(
            "Component2",
            StabilityLevel::Beta,
            ApiVersion::new(1, 0, 0),
            "Description 2",
        ))
        .register(ApiComponent::new(
            "Component3",
            StabilityLevel::Alpha,
            ApiVersion::new(1, 0, 0),
            "Description 3",
        ));
    
    assert_eq!(registry.all().len(), 3);
    assert_eq!(registry.get_stable().len(), 1);
    assert_eq!(registry.get_beta().len(), 1);
    assert_eq!(registry.get_alpha().len(), 1);
}
