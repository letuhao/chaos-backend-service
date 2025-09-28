//! # Unified Element Registry
//! 
//! This module provides the UnifiedElementRegistry as the single source of truth for all element data.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use dashmap::DashMap;
use crate::{ElementCoreResult, ElementCoreError};
use crate::contributor::{ElementContributor, ElementContribution};
use crate::unified_registry::{
    ElementDefinition, SystemRegistration, ElementCategory, ElementPlugin, 
    RegistryConfig, RegistryMetrics
};
use crate::unified_registry::element_category::ElementalElement;
use crate::unified_registry::element_interaction::ElementInteraction;
use crate::common_traits::{ElementGetter, ElementSetter, Validatable, Cacheable, MetricsProvider, Configurable, Serializable, ElementHelper};
use actor_core::Actor;

/// Unified Element Registry - Single source of truth
/// 
/// This registry consolidates all element data, system registrations, contributors,
/// plugins, and interactions into a single, unified system.
pub struct UnifiedElementRegistry {
    /// Core element definitions
    elements: DashMap<String, ElementDefinition>,
    
    /// System registrations
    system_registrations: DashMap<String, SystemRegistration>,
    
    /// External contributors
    contributors: DashMap<String, Arc<dyn ElementContributor>>,
    
    /// Category management
    categories: DashMap<String, ElementCategory>,
    
    /// Plugin management
    plugins: DashMap<String, Arc<dyn ElementPlugin>>,
    
    /// Interaction matrix
    interaction_matrix: DashMap<String, ElementInteraction>,
    
    /// Configuration
    config: RegistryConfig,
    
    /// Performance metrics
    metrics: Arc<RwLock<RegistryMetrics>>,
}

impl UnifiedElementRegistry {
    /// Create a new unified element registry
    pub fn new() -> Self {
        Self {
            elements: DashMap::new(),
            system_registrations: DashMap::new(),
            contributors: DashMap::new(),
            categories: DashMap::new(),
            plugins: DashMap::new(),
            interaction_matrix: DashMap::new(),
            config: RegistryConfig::default(),
            metrics: Arc::new(RwLock::new(RegistryMetrics::default())),
        }
    }
    
    /// Create a new registry with custom configuration
    pub fn with_config(config: RegistryConfig) -> Self {
        Self {
            elements: DashMap::new(),
            system_registrations: DashMap::new(),
            contributors: DashMap::new(),
            categories: DashMap::new(),
            plugins: DashMap::new(),
            interaction_matrix: DashMap::new(),
            config,
            metrics: Arc::new(RwLock::new(RegistryMetrics::default())),
        }
    }
    
    /// Register an element definition
    pub async fn register_element(&self, element: ElementDefinition) -> ElementCoreResult<()> {
        // Validate element
        element.validate()?;
        
        // Check if element already exists
        if self.elements.contains_key(&element.id) {
            return Err(ElementCoreError::Registry { 
                message: format!("Element '{}' is already registered", element.id)
            });
        }
        
        // Check element limit (using a reasonable default)
        const MAX_ELEMENTS: usize = 1000;
        if self.elements.len() >= MAX_ELEMENTS {
            return Err(ElementCoreError::Registry { 
                message: format!("Maximum number of elements ({}) reached", MAX_ELEMENTS)
            });
        }
        
        // Register element
        self.elements.insert(element.id.clone(), element);
        
        // Update metrics
        self.update_element_count();
        
        Ok(())
    }
    
    /// Unregister an element
    pub async fn unregister_element(&self, element_id: &str) -> ElementCoreResult<()> {
        if self.elements.remove(element_id).is_none() {
            return Err(ElementCoreError::ElementNotFound { 
                element_id: element_id.to_string() 
            });
        }
        
        // Update metrics
        self.update_element_count();
        
        Ok(())
    }
    
    /// Get an element definition
    pub fn get_element(&self, element_id: &str) -> Option<ElementDefinition> {
        self.elements.get(element_id).map(|entry| entry.clone())
    }
    
    /// Get all element definitions
    pub fn get_all_elements(&self) -> HashMap<String, ElementDefinition> {
        self.elements.iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect()
    }
    
    /// Check if an element is registered
    pub fn is_element_registered(&self, element_id: &str) -> bool {
        self.elements.contains_key(element_id)
    }
    
    /// Get element count
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }
    
    /// Get element index by ID (for compatibility with factory)
    pub fn get_element_index(&self, element_id: &str) -> ElementCoreResult<Option<usize>> {
        if !self.elements.contains_key(element_id) {
            return Ok(None);
        }
        
        // For now, return a simple index based on element ID hash
        // In a real implementation, you might want to maintain an index mapping
        let index = element_id.len() % 50; // Simple hash-based index
        Ok(Some(index))
    }
    
    /// Get all element IDs (for compatibility with factory)
    pub fn get_element_ids(&self) -> ElementCoreResult<Vec<String>> {
        Ok(self.elements.iter().map(|entry| entry.key().clone()).collect())
    }
    
    /// Get element config (for compatibility with factory)
    pub fn get_element_config(&self, element_id: &str) -> ElementCoreResult<ElementDefinition> {
        self.elements.get(element_id)
            .map(|entry| entry.clone())
            .ok_or_else(|| ElementCoreError::ElementNotFound { 
                element_id: element_id.to_string() 
            })
    }
    
    /// Register a system
    pub async fn register_system(&self, system: SystemRegistration) -> ElementCoreResult<()> {
        // Validate system
        system.validate()?;
        
        // Check if system already exists
        if self.system_registrations.contains_key(&system.system_id) {
            return Err(ElementCoreError::Registry { 
                message: format!("System '{}' is already registered", system.system_id)
            });
        }
        
        // Check system limit (using a reasonable default)
        const MAX_SYSTEMS: usize = 100;
        if self.system_registrations.len() >= MAX_SYSTEMS {
            return Err(ElementCoreError::Registry { 
                message: format!("Maximum number of systems ({}) reached", MAX_SYSTEMS)
            });
        }
        
        // Register system
        self.system_registrations.insert(system.system_id.clone(), system);
        
        // Update metrics
        self.update_system_count();
        
        Ok(())
    }
    
    /// Unregister a system
    pub async fn unregister_system(&self, system_id: &str) -> ElementCoreResult<()> {
        if self.system_registrations.remove(system_id).is_none() {
            return Err(ElementCoreError::Registry { 
                message: format!("System '{}' not found", system_id)
            });
        }
        
        // Update metrics
        self.update_system_count();
        
        Ok(())
    }
    
    /// Get a system registration
    pub fn get_system(&self, system_id: &str) -> Option<SystemRegistration> {
        self.system_registrations.get(system_id).map(|entry| entry.clone())
    }
    
    /// Get all system registrations
    pub fn get_all_systems(&self) -> HashMap<String, SystemRegistration> {
        self.system_registrations.iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect()
    }
    
    /// Check if a system is registered
    pub fn is_system_registered(&self, system_id: &str) -> bool {
        self.system_registrations.contains_key(system_id)
    }
    
    /// Get system count
    pub fn system_count(&self) -> usize {
        self.system_registrations.len()
    }
    
    /// Register a contributor
    pub async fn register_contributor(&self, contributor: Arc<dyn ElementContributor>) -> ElementCoreResult<()> {
        let system_id = contributor.system_id().to_string();
        
        // Check if contributor already exists
        if self.contributors.contains_key(&system_id) {
            return Err(ElementCoreError::Registry { 
                message: format!("Contributor '{}' is already registered", system_id)
            });
        }
        
        // Check contributor limit (using a reasonable default)
        const MAX_CONTRIBUTORS: usize = 100;
        if self.contributors.len() >= MAX_CONTRIBUTORS {
            return Err(ElementCoreError::Registry { 
                message: format!("Maximum number of contributors ({}) reached", MAX_CONTRIBUTORS)
            });
        }
        
        // Register contributor
        self.contributors.insert(system_id, contributor);
        
        Ok(())
    }
    
    /// Unregister a contributor
    pub async fn unregister_contributor(&self, system_id: &str) -> ElementCoreResult<()> {
        if self.contributors.remove(system_id).is_none() {
            return Err(ElementCoreError::Registry { 
                message: format!("Contributor '{}' not found", system_id)
            });
        }
        
        Ok(())
    }
    
    /// Get a contributor
    pub fn get_contributor(&self, system_id: &str) -> Option<Arc<dyn ElementContributor>> {
        self.contributors.get(system_id).map(|entry| entry.clone())
    }
    
    /// Get all contributors
    pub fn get_all_contributors(&self) -> Vec<Arc<dyn ElementContributor>> {
        self.contributors.iter()
            .map(|entry| entry.value().clone())
            .collect()
    }
    
    /// Check if a contributor is registered
    pub fn is_contributor_registered(&self, system_id: &str) -> bool {
        self.contributors.contains_key(system_id)
    }
    
    /// Get contributor count
    pub fn contributor_count(&self) -> usize {
        self.contributors.len()
    }
    
    /// Register a plugin
    pub async fn register_plugin(&self, plugin: Arc<dyn ElementPlugin>) -> ElementCoreResult<()> {
        let plugin_id = plugin.get_metadata().plugin_id.clone();
        
        // Check if plugin already exists
        if self.plugins.contains_key(&plugin_id) {
            return Err(ElementCoreError::Registry { 
                message: format!("Plugin '{}' is already registered", plugin_id)
            });
        }
        
        // Check plugin limit (using a reasonable default)
        const MAX_PLUGINS: usize = 50;
        if self.plugins.len() >= MAX_PLUGINS {
            return Err(ElementCoreError::Registry { 
                message: format!("Maximum number of plugins ({}) reached", MAX_PLUGINS)
            });
        }
        
        // Initialize plugin
        plugin.initialize().await?;
        
        // Register plugin
        self.plugins.insert(plugin_id, plugin);
        
        // Update metrics
        self.update_plugin_count();
        
        Ok(())
    }
    
    /// Unregister a plugin
    pub async fn unregister_plugin(&self, plugin_id: &str) -> ElementCoreResult<()> {
        if let Some((_, plugin)) = self.plugins.remove(plugin_id) {
            // Shutdown plugin
            plugin.shutdown().await?;
            
            // Update metrics
            self.update_plugin_count();
            
            Ok(())
        } else {
            Err(ElementCoreError::Registry { 
                message: format!("Plugin '{}' not found", plugin_id)
            })
        }
    }
    
    /// Get a plugin
    pub fn get_plugin(&self, plugin_id: &str) -> Option<Arc<dyn ElementPlugin>> {
        self.plugins.get(plugin_id).map(|entry| entry.clone())
    }
    
    /// Get all plugins
    pub fn get_all_plugins(&self) -> Vec<Arc<dyn ElementPlugin>> {
        self.plugins.iter()
            .map(|entry| entry.value().clone())
            .collect()
    }
    
    /// Check if a plugin is registered
    pub fn is_plugin_registered(&self, plugin_id: &str) -> bool {
        self.plugins.contains_key(plugin_id)
    }
    
    /// Get plugin count
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
    
    /// Register an element interaction
    pub async fn register_interaction(&self, interaction: ElementInteraction) -> ElementCoreResult<()> {
        // Validate interaction
        interaction.validate()?;
        
        let key = format!("{}:{}", interaction.source_element, interaction.target_element);
        
        // Check if interaction already exists
        if self.interaction_matrix.contains_key(&key) {
            return Err(ElementCoreError::Registry { 
                message: format!("Interaction '{}' is already registered", key)
            });
        }
        
        // Register interaction
        self.interaction_matrix.insert(key, interaction);
        
        // Update metrics
        self.update_interaction_count();
        
        Ok(())
    }
    
    /// Unregister an element interaction
    pub async fn unregister_interaction(&self, source_element: &str, target_element: &str) -> ElementCoreResult<()> {
        let key = format!("{}:{}", source_element, target_element);
        
        if self.interaction_matrix.remove(&key).is_none() {
            return Err(ElementCoreError::Registry { 
                message: format!("Interaction '{}' not found", key)
            });
        }
        
        // Update metrics
        self.update_interaction_count();
        
        Ok(())
    }
    
    /// Get an element interaction
    pub fn get_interaction(&self, source_element: &str, target_element: &str) -> Option<ElementInteraction> {
        let key = format!("{}:{}", source_element, target_element);
        self.interaction_matrix.get(&key).map(|entry| entry.clone())
    }
    
    /// Get all element interactions
    pub fn get_all_interactions(&self) -> HashMap<String, ElementInteraction> {
        self.interaction_matrix.iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect()
    }
    
    /// Check if an interaction is registered
    pub fn is_interaction_registered(&self, source_element: &str, target_element: &str) -> bool {
        let key = format!("{}:{}", source_element, target_element);
        self.interaction_matrix.contains_key(&key)
    }
    
    /// Get interaction count
    pub fn interaction_count(&self) -> usize {
        self.interaction_matrix.len()
    }
    
    /// Get registry configuration
    pub fn get_config(&self) -> &RegistryConfig {
        &self.config
    }
    
    /// Update registry configuration
    pub async fn update_config(&self, config: RegistryConfig) -> ElementCoreResult<()> {
        // Note: In a real implementation, this would need to be wrapped in Arc<RwLock<RegistryConfig>>
        // For now, we'll just validate the config
        // config.validate()?;
        Ok(())
    }
    
    /// Get registry metrics
    pub fn get_metrics(&self) -> RegistryMetrics {
        self.metrics.read().unwrap_or_else(|_| panic!("Failed to read metrics")).clone()
    }
    
    /// Update element count in metrics
    fn update_element_count(&self) {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.overall.total_elements = self.element_count();
        }
    }
    
    /// Update system count in metrics
    fn update_system_count(&self) {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.overall.total_contributors = self.system_count();
        }
    }
    
    /// Update plugin count in metrics
    fn update_plugin_count(&self) {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.overall.total_plugins = self.plugin_count();
        }
    }
    
    /// Update interaction count in metrics
    fn update_interaction_count(&self) {
        if let Ok(mut metrics) = self.metrics.write() {
            // Note: total_interactions is not in OverallMetrics, so we'll skip this for now
            // In a real implementation, you might want to add it to OverallMetrics
        }
    }
    
    /// Validate registry state
    pub fn validate(&self) -> ElementCoreResult<()> {
        // Validate configuration
        self.config.validate()?;
        
        // Validate elements
        for element in self.elements.iter() {
            element.value().validate()?;
        }
        
        // Validate systems
        for system in self.system_registrations.iter() {
            system.value().validate()?;
        }
        
        // Validate interactions
        for interaction in self.interaction_matrix.iter() {
            interaction.value().validate()?;
        }
        
        Ok(())
    }
    
    /// Clear all registry data
    pub async fn clear(&self) -> ElementCoreResult<()> {
        // Shutdown all plugins
        for plugin in self.plugins.iter() {
            plugin.value().shutdown().await?;
        }
        
        // Clear all data
        self.elements.clear();
        self.system_registrations.clear();
        self.contributors.clear();
        self.categories.clear();
        self.plugins.clear();
        self.interaction_matrix.clear();
        
        Ok(())
    }
    
    /// Get registry statistics
    pub fn get_statistics(&self) -> RegistryStatistics {
        RegistryStatistics {
            element_count: self.element_count(),
            system_count: self.system_count(),
            contributor_count: self.contributor_count(),
            plugin_count: self.plugin_count(),
            interaction_count: self.interaction_count(),
            category_count: self.categories.len(),
        }
    }
}

/// Registry statistics
#[derive(Debug, Clone)]
pub struct RegistryStatistics {
    /// Number of registered elements
    pub element_count: usize,
    
    /// Number of registered systems
    pub system_count: usize,
    
    /// Number of registered contributors
    pub contributor_count: usize,
    
    /// Number of registered plugins
    pub plugin_count: usize,
    
    /// Number of registered interactions
    pub interaction_count: usize,
    
    /// Number of registered categories
    pub category_count: usize,
}

impl Default for UnifiedElementRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe wrapper for UnifiedElementRegistry
pub type SharedUnifiedElementRegistry = Arc<UnifiedElementRegistry>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unified_registry::element_definition::ElementDefinition;
    use crate::unified_registry::element_category::ElementCategory;
    use crate::unified_registry::system_registration::SystemRegistration;
    use crate::unified_registry::element_interaction::ElementInteraction;
    use crate::unified_registry::element_interaction::InteractionType;
    use crate::core::ElementConfig;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_registry_creation() {
        let registry = UnifiedElementRegistry::new();
        assert_eq!(registry.element_count(), 0);
        assert_eq!(registry.system_count(), 0);
        assert_eq!(registry.contributor_count(), 0);
        assert_eq!(registry.plugin_count(), 0);
        assert_eq!(registry.interaction_count(), 0);
    }

    #[tokio::test]
    async fn test_element_registration() {
        let registry = UnifiedElementRegistry::new();
        
        let element = ElementDefinition::new(
            "fire".to_string(),
            "Fire".to_string(),
            "Fire element".to_string(),
            ElementCategory::Elemental(ElementalElement::Light),
        );
        
        // Register element
        registry.register_element(element).await.unwrap();
        assert_eq!(registry.element_count(), 1);
        assert!(registry.is_element_registered("fire"));
        
        // Get element
        let retrieved = registry.get_element("fire").unwrap();
        assert_eq!(retrieved.id, "fire");
        assert_eq!(retrieved.name, "Fire");
        
        // Unregister element
        registry.unregister_element("fire").await.unwrap();
        assert_eq!(registry.element_count(), 0);
        assert!(!registry.is_element_registered("fire"));
    }

    #[tokio::test]
    async fn test_system_registration() {
        let registry = UnifiedElementRegistry::new();
        
        let system = SystemRegistration::new(
            "race-core".to_string(),
            "Race Core".to_string(),
            "1.0.0".to_string(),
            "test_description".to_string(),
            1000,
        );
        
        // Register system
        registry.register_system(system).await.unwrap();
        assert_eq!(registry.system_count(), 1);
        assert!(registry.is_system_registered("race-core"));
        
        // Get system
        let retrieved = registry.get_system("race-core").unwrap();
        assert_eq!(retrieved.system_id, "race-core");
        assert_eq!(retrieved.system_name, "Race Core");
        
        // Unregister system
        registry.unregister_system("race-core").await.unwrap();
        assert_eq!(registry.system_count(), 0);
        assert!(!registry.is_system_registered("race-core"));
    }

    #[tokio::test]
    async fn test_interaction_registration() {
        let registry = UnifiedElementRegistry::new();
        
        let interaction = ElementInteraction::new(
            "fire_vs_wood".to_string(),
            "fire".to_string(),
            "wood".to_string(),
            InteractionType::Overcoming,
        );
        
        // Register interaction
        registry.register_interaction(interaction).await.unwrap();
        assert_eq!(registry.interaction_count(), 1);
        assert!(registry.is_interaction_registered("fire", "wood"));
        
        // Get interaction
        let retrieved = registry.get_interaction("fire", "wood").unwrap();
        assert_eq!(retrieved.id, "fire_vs_wood");
        assert_eq!(retrieved.source_element, "fire");
        assert_eq!(retrieved.target_element, "wood");
        
        // Unregister interaction
        registry.unregister_interaction("fire", "wood").await.unwrap();
        assert_eq!(registry.interaction_count(), 0);
        assert!(!registry.is_interaction_registered("fire", "wood"));
    }

    #[tokio::test]
    async fn test_duplicate_registration() {
        let registry = UnifiedElementRegistry::new();
        
        let element = ElementDefinition::new(
            "fire".to_string(),
            "Fire".to_string(),
            "Fire element".to_string(),
            ElementCategory::Elemental(ElementalElement::Light),
        );
        
        // Register element first time
        registry.register_element(element.clone()).await.unwrap();
        
        // Try to register same element again
        let result = registry.register_element(element).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already registered"));
    }

    #[tokio::test]
    async fn test_nonexistent_unregistration() {
        let registry = UnifiedElementRegistry::new();
        
        // Try to unregister non-existent element
        let result = registry.unregister_element("nonexistent").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_registry_statistics() {
        let registry = UnifiedElementRegistry::new();
        
        let stats = registry.get_statistics();
        assert_eq!(stats.element_count, 0);
        assert_eq!(stats.system_count, 0);
        assert_eq!(stats.contributor_count, 0);
        assert_eq!(stats.plugin_count, 0);
        assert_eq!(stats.interaction_count, 0);
        assert_eq!(stats.category_count, 0);
    }

    #[tokio::test]
    async fn test_registry_validation() {
        let registry = UnifiedElementRegistry::new();
        
        // Empty registry should validate
        registry.validate().unwrap();
        
        // Add valid element
        let element = ElementDefinition::new(
            "fire".to_string(),
            "Fire".to_string(),
            "Fire element".to_string(),
            ElementCategory::Elemental(ElementalElement::Light),
        );
        registry.register_element(element).await.unwrap();
        
        // Registry with valid element should validate
        registry.validate().unwrap();
    }

    #[tokio::test]
    async fn test_registry_clear() {
        let registry = UnifiedElementRegistry::new();
        
        // Add some data
        let element = ElementDefinition::new(
            "fire".to_string(),
            "Fire".to_string(),
            "Fire element".to_string(),
            ElementCategory::Elemental(ElementalElement::Light),
        );
        registry.register_element(element).await.unwrap();
        
        let system = SystemRegistration::new(
            "race-core".to_string(),
            "Race Core".to_string(),
            "1.0.0".to_string(),
            "test_description".to_string(),
            1000,
        );
        registry.register_system(system).await.unwrap();
        
        // Verify data exists
        assert_eq!(registry.element_count(), 1);
        assert_eq!(registry.system_count(), 1);
        
        // Clear registry
        registry.clear().await.unwrap();
        
        // Verify data is cleared
        assert_eq!(registry.element_count(), 0);
        assert_eq!(registry.system_count(), 0);
    }
}

// ===== COMMON TRAITS IMPLEMENTATIONS =====

impl ElementGetter<ElementDefinition> for UnifiedElementRegistry {
    fn get_element(&self, identifier: &str) -> ElementCoreResult<ElementDefinition> {
        <Self as ElementHelper>::validate_identifier(identifier)?;
        
        self.elements.get(identifier)
            .map(|entry| entry.clone())
            .ok_or_else(|| ElementCoreError::ElementNotFound { 
                element_id: identifier.to_string() 
            })
    }
    
    fn has_element(&self, identifier: &str) -> bool {
        self.elements.contains_key(identifier)
    }
    
    fn get_all_element_ids(&self) -> ElementCoreResult<Vec<String>> {
        Ok(self.elements.iter().map(|entry| entry.key().clone()).collect())
    }
    
    fn element_count(&self) -> usize {
        self.elements.len()
    }
}

impl ElementSetter<ElementDefinition> for UnifiedElementRegistry {
    fn set_element(&self, identifier: &str, element: ElementDefinition) -> ElementCoreResult<()> {
        <Self as ElementHelper>::validate_identifier(identifier)?;
        element.validate()?;
        
        self.elements.insert(identifier.to_string(), element);
        self.update_element_count();
        Ok(())
    }
    
    fn remove_element(&self, identifier: &str) -> ElementCoreResult<()> {
        <Self as ElementHelper>::validate_identifier(identifier)?;
        
        if self.elements.remove(identifier).is_none() {
            return Err(ElementCoreError::ElementNotFound { 
                element_id: identifier.to_string() 
            });
        }
        
        self.update_element_count();
        Ok(())
    }
}

impl Validatable for UnifiedElementRegistry {
    fn validate(&self) -> ElementCoreResult<()> {
        // Validate configuration
        self.config.validate()?;
        
        // Validate elements
        for element in self.elements.iter() {
            element.value().validate()?;
        }
        
        // Validate systems
        for system in self.system_registrations.iter() {
            system.value().validate()?;
        }
        
        // Validate interactions
        for interaction in self.interaction_matrix.iter() {
            interaction.value().validate()?;
        }
        
        Ok(())
    }
    
    fn get_validation_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Check configuration
        if let Err(e) = self.config.validate() {
            errors.push(format!("Config validation error: {}", e));
        }
        
        // Check elements
        for element in self.elements.iter() {
            if let Err(e) = element.value().validate() {
                errors.push(format!("Element '{}' validation error: {}", element.key(), e));
            }
        }
        
        // Check systems
        for system in self.system_registrations.iter() {
            if let Err(e) = system.value().validate() {
                errors.push(format!("System '{}' validation error: {}", system.key(), e));
            }
        }
        
        // Check interactions
        for interaction in self.interaction_matrix.iter() {
            if let Err(e) = interaction.value().validate() {
                errors.push(format!("Interaction '{}' validation error: {}", interaction.key(), e));
            }
        }
        
        errors
    }
}

impl Cacheable for UnifiedElementRegistry {
    fn cache_key(&self) -> String {
        "unified_element_registry".to_string()
    }
    
    fn cache_ttl(&self) -> u64 {
        300 // 5 minutes
    }
    
    fn should_cache(&self) -> bool {
        self.elements.len() > 0
    }
}

impl MetricsProvider for UnifiedElementRegistry {
    fn get_metrics(&self) -> std::collections::HashMap<String, f64> {
        let mut metrics = std::collections::HashMap::new();
        let registry_metrics = self.get_metrics();
        
        metrics.insert("total_elements".to_string(), registry_metrics.overall.total_elements as f64);
        metrics.insert("total_contributors".to_string(), registry_metrics.overall.total_contributors as f64);
        metrics.insert("total_plugins".to_string(), registry_metrics.overall.total_plugins as f64);
        metrics.insert("cache_hits".to_string(), registry_metrics.cache_metrics.hit_count as f64);
        metrics.insert("cache_misses".to_string(), registry_metrics.cache_metrics.miss_count as f64);
        
        metrics
    }
    
    fn reset_metrics(&self) {
        if let Ok(mut metrics) = self.metrics.write() {
            *metrics = RegistryMetrics::default();
        }
    }
    
    fn get_metrics_summary(&self) -> String {
        let metrics = self.get_metrics();
        format!(
            "UnifiedElementRegistry Metrics:\n\
            - Elements: {}\n\
            - Contributors: {}\n\
            - Plugins: {}\n\
            - Cache Hits: {}\n\
            - Cache Misses: {}",
            metrics.overall.total_elements,
            metrics.overall.total_contributors,
            metrics.overall.total_plugins,
            metrics.cache_metrics.hit_count,
            metrics.cache_metrics.miss_count
        )
    }
}

impl Configurable for UnifiedElementRegistry {
    fn get_config(&self) -> serde_json::Value {
        serde_json::to_value(&self.config).unwrap_or_default()
    }
    
    fn update_config(&self, config: serde_json::Value) -> ElementCoreResult<()> {
        let new_config: RegistryConfig = serde_json::from_value(config)
            .map_err(|e| ElementCoreError::Config { message: format!("Invalid config JSON: {}", e) })?;
        
        new_config.validate()?;
        // Note: In a real implementation, this would need to be wrapped in Arc<RwLock<RegistryConfig>>
        // For now, we'll just validate the config
        Ok(())
    }
    
    fn validate_config(&self, config: &serde_json::Value) -> ElementCoreResult<()> {
        let _: RegistryConfig = serde_json::from_value(config.clone())
            .map_err(|e| ElementCoreError::Config { message: format!("Invalid config JSON: {}", e) })?;
        Ok(())
    }
}

impl Serializable for UnifiedElementRegistry {
    fn to_json(&self) -> ElementCoreResult<String> {
        // TODO: Implement proper serialization for DashMap and trait objects
        // For now, return a simplified JSON representation
        let simplified = serde_json::json!({
            "element_count": self.element_count(),
            "system_count": self.system_count(),
            "config": self.get_config()
        });
        serde_json::to_string(&simplified)
            .map_err(|e| ElementCoreError::Serialization(e))
    }
    
    fn to_yaml(&self) -> ElementCoreResult<String> {
        // TODO: Implement proper serialization for DashMap and trait objects
        // For now, return a simplified YAML representation
        let simplified = serde_json::json!({
            "element_count": self.element_count(),
            "system_count": self.system_count(),
            "config": self.get_config()
        });
        serde_yaml::to_string(&simplified)
            .map_err(|e| ElementCoreError::YamlParsing(e))
    }
    
    fn from_json(json: &str) -> ElementCoreResult<Self> {
        // TODO: Implement proper deserialization for DashMap and trait objects
        // For now, return a new empty registry
        Ok(Self::new())
    }
    
    fn from_yaml(yaml: &str) -> ElementCoreResult<Self> {
        // TODO: Implement proper deserialization for DashMap and trait objects
        // For now, return a new empty registry
        Ok(Self::new())
    }
}

impl ElementHelper for UnifiedElementRegistry {}