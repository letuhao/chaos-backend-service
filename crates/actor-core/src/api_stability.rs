//! API Stability and Versioning.
//!
//! This module documents the stability guarantees of the Actor Core API
//! and provides versioning information for compatibility.

/// API Stability Levels.
///
/// This enum defines the stability level of different parts of the API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StabilityLevel {
    /// Stable API - guaranteed to remain compatible across minor versions
    Stable,
    /// Beta API - may change in minor versions but will be deprecated first
    Beta,
    /// Alpha API - experimental, may change without notice
    Alpha,
    /// Internal API - not part of the public API, may change without notice
    Internal,
}

/// API version information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiVersion {
    /// Major version number
    pub major: u32,
    /// Minor version number
    pub minor: u32,
    /// Patch version number
    pub patch: u32,
}

impl ApiVersion {
    /// Create a new API version.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
    
    /// Get the current API version.
    pub fn current() -> Self {
        Self::new(1, 0, 0)
    }
    
    /// Check if this version is compatible with another version.
    ///
    /// Versions are compatible if they have the same major version.
    pub fn is_compatible_with(&self, other: &ApiVersion) -> bool {
        self.major == other.major
    }
    
    /// Get the version as a string.
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// API component with stability information.
#[derive(Debug, Clone)]
pub struct ApiComponent {
    /// Component name
    pub name: &'static str,
    /// Stability level
    pub stability: StabilityLevel,
    /// Version when introduced
    pub introduced_in: ApiVersion,
    /// Version when deprecated (if applicable)
    pub deprecated_in: Option<ApiVersion>,
    /// Version when removed (if applicable)
    pub removed_in: Option<ApiVersion>,
    /// Description of the component
    pub description: &'static str,
}

impl ApiComponent {
    /// Create a new API component.
    pub fn new(
        name: &'static str,
        stability: StabilityLevel,
        introduced_in: ApiVersion,
        description: &'static str,
    ) -> Self {
        Self {
            name,
            stability,
            introduced_in,
            deprecated_in: None,
            removed_in: None,
            description,
        }
    }
    
    /// Mark this component as deprecated.
    pub fn deprecate_in(mut self, version: ApiVersion) -> Self {
        self.deprecated_in = Some(version);
        self
    }
    
    /// Mark this component as removed.
    pub fn remove_in(mut self, version: ApiVersion) -> Self {
        self.removed_in = Some(version);
        self
    }
    
    /// Check if this component is stable.
    pub fn is_stable(&self) -> bool {
        self.stability == StabilityLevel::Stable
    }
    
    /// Check if this component is deprecated.
    pub fn is_deprecated(&self) -> bool {
        self.deprecated_in.is_some()
    }
    
    /// Check if this component is removed.
    pub fn is_removed(&self) -> bool {
        self.removed_in.is_some()
    }
}

/// Registry of API components and their stability.
pub struct ApiRegistry {
    components: Vec<ApiComponent>,
}

impl ApiRegistry {
    /// Create a new API registry.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    
    /// Register an API component.
    pub fn register(mut self, component: ApiComponent) -> Self {
        self.components.push(component);
        self
    }
    
    /// Get all components with a specific stability level.
    pub fn get_by_stability(&self, stability: StabilityLevel) -> Vec<&ApiComponent> {
        self.components
            .iter()
            .filter(|c| c.stability == stability)
            .collect()
    }
    
    /// Get all stable components.
    pub fn get_stable(&self) -> Vec<&ApiComponent> {
        self.get_by_stability(StabilityLevel::Stable)
    }
    
    /// Get all beta components.
    pub fn get_beta(&self) -> Vec<&ApiComponent> {
        self.get_by_stability(StabilityLevel::Beta)
    }
    
    /// Get all alpha components.
    pub fn get_alpha(&self) -> Vec<&ApiComponent> {
        self.get_by_stability(StabilityLevel::Alpha)
    }
    
    /// Find a component by name.
    pub fn find(&self, name: &str) -> Option<&ApiComponent> {
        self.components.iter().find(|c| c.name == name)
    }
    
    /// Get all components.
    pub fn all(&self) -> &[ApiComponent] {
        &self.components
    }
}

/// Get the global API registry.
pub fn get_api_registry() -> ApiRegistry {
    ApiRegistry::new()
        // Core types - Stable
        .register(ApiComponent::new(
            "Actor",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Core actor data structure representing a character",
        ))
        .register(ApiComponent::new(
            "Contribution",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Individual stat modification from a subsystem",
        ))
        .register(ApiComponent::new(
            "CapContribution",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Stat limit modification from a subsystem",
        ))
        .register(ApiComponent::new(
            "Snapshot",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Aggregated stat state at a point in time",
        ))
        .register(ApiComponent::new(
            "Caps",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Min/max constraints for stat values",
        ))
        
        // Enums - Stable
        .register(ApiComponent::new(
            "Bucket",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Processing mode for contributions",
        ))
        .register(ApiComponent::new(
            "CapMode",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "How cap contributions are applied",
        ))
        .register(ApiComponent::new(
            "CapKind",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Type of cap (Min or Max)",
        ))
        .register(ApiComponent::new(
            "AcrossLayerPolicy",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Policy for handling caps across layers",
        ))
        .register(ApiComponent::new(
            "Operator",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Mathematical operators for cap calculations",
        ))
        
        // Traits - Stable
        .register(ApiComponent::new(
            "Subsystem",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Trait for implementing stat subsystems",
        ))
        .register(ApiComponent::new(
            "Aggregator",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Trait for stat aggregation services",
        ))
        .register(ApiComponent::new(
            "CapsProvider",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Trait for cap calculation services",
        ))
        .register(ApiComponent::new(
            "PluginRegistry",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Trait for managing subsystem plugins",
        ))
        .register(ApiComponent::new(
            "CombinerRegistry",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Trait for managing contribution combination rules",
        ))
        .register(ApiComponent::new(
            "Cache",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Trait for caching services",
        ))
        
        // Error types - Stable
        .register(ApiComponent::new(
            "ActorCoreError",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Main error type for the Actor Core library",
        ))
        .register(ApiComponent::new(
            "ActorCoreResult",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Result type alias for Actor Core operations",
        ))
        
        // Service factory - Stable
        .register(ApiComponent::new(
            "ServiceFactory",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Factory for creating core services",
        ))
        
        // Prelude - Stable
        .register(ApiComponent::new(
            "prelude",
            StabilityLevel::Stable,
            ApiVersion::new(1, 0, 0),
            "Convenience module with commonly used types and functions",
        ))
        
        // Performance monitoring - Beta
        .register(ApiComponent::new(
            "PerformanceProfiler",
            StabilityLevel::Beta,
            ApiVersion::new(1, 0, 0),
            "Performance profiling and monitoring",
        ))
        .register(ApiComponent::new(
            "PerformanceTestSuite",
            StabilityLevel::Beta,
            ApiVersion::new(1, 0, 0),
            "Performance testing framework",
        ))
        .register(ApiComponent::new(
            "PerformanceWorkflowManager",
            StabilityLevel::Beta,
            ApiVersion::new(1, 0, 0),
            "Performance testing workflow orchestration",
        ))
        
        // Observability - Beta
        .register(ApiComponent::new(
            "ObservabilityManager",
            StabilityLevel::Beta,
            ApiVersion::new(1, 0, 0),
            "Observability and monitoring utilities",
        ))
        
        // Metrics - Beta
        .register(ApiComponent::new(
            "SubsystemMetrics",
            StabilityLevel::Beta,
            ApiVersion::new(1, 0, 0),
            "Metrics for subsystem performance",
        ))
        .register(ApiComponent::new(
            "AggregatorMetrics",
            StabilityLevel::Beta,
            ApiVersion::new(1, 0, 0),
            "Metrics for aggregator performance",
        ))
        .register(ApiComponent::new(
            "CacheStats",
            StabilityLevel::Beta,
            ApiVersion::new(1, 0, 0),
            "Cache performance statistics",
        ))
        
        // Internal modules - Internal
        .register(ApiComponent::new(
            "bucket_processor",
            StabilityLevel::Internal,
            ApiVersion::new(1, 0, 0),
            "Internal bucket processing implementation",
        ))
        .register(ApiComponent::new(
            "aggregator",
            StabilityLevel::Internal,
            ApiVersion::new(1, 0, 0),
            "Internal aggregator implementation",
        ))
        .register(ApiComponent::new(
            "cache",
            StabilityLevel::Internal,
            ApiVersion::new(1, 0, 0),
            "Internal cache implementation",
        ))
        .register(ApiComponent::new(
            "registry",
            StabilityLevel::Internal,
            ApiVersion::new(1, 0, 0),
            "Internal registry implementation",
        ))
        .register(ApiComponent::new(
            "production",
            StabilityLevel::Internal,
            ApiVersion::new(1, 0, 0),
            "Internal production utilities and mocks",
        ))
}

/// Check API compatibility between versions.
///
/// # Example
///
/// ```rust
/// use actor_core::api_stability::*;
///
/// let current = ApiVersion::current();
/// let other = ApiVersion::new(1, 1, 0);
///
/// if current.is_compatible_with(&other) {
///     println!("Versions are compatible");
/// } else {
///     println!("Versions are incompatible");
/// }
/// ```
pub fn check_compatibility(current: &ApiVersion, target: &ApiVersion) -> bool {
    current.is_compatible_with(target)
}

/// Get API stability report.
///
/// This function generates a report of all API components and their stability levels.
pub fn get_stability_report() -> String {
    let registry = get_api_registry();
    let mut report = String::new();
    
    report.push_str("# Actor Core API Stability Report\n\n");
    report.push_str(&format!("Generated for version: {}\n\n", ApiVersion::current().to_string()));
    
    // Stable components
    report.push_str("## Stable API (v1.0.0+)\n\n");
    for component in registry.get_stable() {
        report.push_str(&format!("- **{}**: {}\n", component.name, component.description));
    }
    
    // Beta components
    report.push_str("\n## Beta API (may change in minor versions)\n\n");
    for component in registry.get_beta() {
        report.push_str(&format!("- **{}**: {}\n", component.name, component.description));
    }
    
    // Alpha components
    report.push_str("\n## Alpha API (experimental)\n\n");
    for component in registry.get_alpha() {
        report.push_str(&format!("- **{}**: {}\n", component.name, component.description));
    }
    
    report.push_str("\n## Stability Guarantees\n\n");
    report.push_str("- **Stable**: Guaranteed to remain compatible across minor versions\n");
    report.push_str("- **Beta**: May change in minor versions but will be deprecated first\n");
    report.push_str("- **Alpha**: Experimental, may change without notice\n");
    report.push_str("- **Internal**: Not part of the public API, may change without notice\n");
    
    report
}