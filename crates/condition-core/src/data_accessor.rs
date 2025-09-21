//! Element Data Accessor - Loose coupling data access layer
//! 
//! This module provides a clean interface for accessing element data without
//! tight coupling to the ElementDataProvider trait. This allows functions to
//! access only the data they need without requiring implementation of all trait methods.

use crate::data_provider::ElementDataProvider;
use crate::error::ConditionResult;
use std::sync::Arc;

/// Element Data Accessor - Provides clean data access interface
/// 
/// This struct acts as a facade over the ElementDataProvider trait, providing
/// only the methods that are actually needed by condition functions. This eliminates
/// the tight coupling issue where every mock implementation had to implement
/// all trait methods.
pub struct ElementDataAccessor {
    element_provider: Arc<dyn ElementDataProvider>,
}

impl ElementDataAccessor {
    /// Create a new ElementDataAccessor
    pub fn new(element_provider: Arc<dyn ElementDataProvider>) -> Self {
        Self { element_provider }
    }

    // Core element functions - only implement what's actually needed
    /// Get element mastery level
    pub async fn get_element_mastery(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64> {
        self.element_provider.get_element_mastery(element_id, actor_id).await
    }

    /// Get element resistance
    pub async fn get_element_resistance(&self, element_id: &str, actor_id: &str) -> ConditionResult<f64> {
        self.element_provider.get_element_resistance(element_id, actor_id).await
    }

    /// Check if actor has element affinity
    pub async fn has_element_affinity(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool> {
        self.element_provider.has_element_affinity(element_id, actor_id).await
    }

    /// Check if element is weakness
    pub async fn is_element_weakness(&self, element_id: &str, actor_id: &str) -> ConditionResult<bool> {
        self.element_provider.is_element_weakness(element_id, actor_id).await
    }

    /// Get element interaction type
    pub async fn get_element_interaction(&self, source_element: &str, target_element: &str) -> ConditionResult<String> {
        self.element_provider.get_element_interaction(source_element, target_element).await
    }

    /// List all available elements
    pub async fn list_elements(&self) -> ConditionResult<Vec<String>> {
        self.element_provider.list_elements().await
    }

    // Element interaction functions
    /// Check if two elements are in the same category
    pub async fn is_element_same_category(&self, element1: &str, element2: &str) -> ConditionResult<bool> {
        self.element_provider.is_element_same_category(element1, element2).await
    }

    /// Check if source element generates target element
    pub async fn is_element_generating(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        self.element_provider.is_element_generating(source_element, target_element).await
    }

    /// Check if source element overcomes target element
    pub async fn is_element_overcoming(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        self.element_provider.is_element_overcoming(source_element, target_element).await
    }

    /// Check if elements have neutral interaction
    pub async fn is_element_neutral(&self, source_element: &str, target_element: &str) -> ConditionResult<bool> {
        self.element_provider.is_element_neutral(source_element, target_element).await
    }

    // Element status functions
    /// Check if element has specific status effect
    pub async fn has_element_status_effect(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<bool> {
        self.element_provider.has_element_status_effect(element_id, status_id, actor_id).await
    }

    /// Get element status effect count
    pub async fn get_element_status_effect_count(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<i64> {
        self.element_provider.get_element_status_effect_count(element_id, status_id, actor_id).await
    }

    /// Check if element status effect is active
    pub async fn is_element_status_effect_active(&self, element_id: &str, status_id: &str, actor_id: &str) -> ConditionResult<bool> {
        self.element_provider.is_element_status_effect_active(element_id, status_id, actor_id).await
    }

    // Element resource functions
    /// Check if element has specific resource
    pub async fn has_element_resource(&self, element_id: &str, resource_type: &str, actor_id: &str) -> ConditionResult<bool> {
        self.element_provider.has_element_resource(element_id, resource_type, actor_id).await
    }

    /// Get element resource value
    pub async fn get_element_resource_value(&self, element_id: &str, resource_type: &str, actor_id: &str) -> ConditionResult<f64> {
        self.element_provider.get_element_resource_value(element_id, resource_type, actor_id).await
    }

    /// Check if element resource is below threshold
    pub async fn is_element_resource_below_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool> {
        self.element_provider.is_element_resource_below_threshold(element_id, resource_type, threshold, actor_id).await
    }

    /// Check if element resource is above threshold
    pub async fn is_element_resource_above_threshold(&self, element_id: &str, resource_type: &str, threshold: f64, actor_id: &str) -> ConditionResult<bool> {
        self.element_provider.is_element_resource_above_threshold(element_id, resource_type, threshold, actor_id).await
    }

    // Hybrid element functions
    /// Check if actor has hybrid element
    pub async fn has_hybrid_element(&self, hybrid_id: &str, actor_id: &str) -> ConditionResult<bool> {
        self.element_provider.has_hybrid_element(hybrid_id, actor_id).await
    }

    /// Check if hybrid element is activated
    pub async fn is_hybrid_element_activated(&self, hybrid_id: &str, actor_id: &str) -> ConditionResult<bool> {
        self.element_provider.is_hybrid_element_activated(hybrid_id, actor_id).await
    }

    /// Get hybrid element parent elements
    pub async fn get_hybrid_element_parents(&self, hybrid_id: &str) -> ConditionResult<Vec<String>> {
        self.element_provider.get_hybrid_element_parents(hybrid_id).await
    }

    /// List all hybrid elements
    pub async fn list_hybrid_elements(&self) -> ConditionResult<Vec<String>> {
        self.element_provider.list_hybrid_elements().await
    }

    // Element derived stats functions
    /// Get element derived stat value
    pub async fn get_element_derived_stat(&self, element_id: &str, stat_name: &str, actor_id: &str) -> ConditionResult<f64> {
        self.element_provider.get_element_derived_stat(element_id, stat_name, actor_id).await
    }

    /// Check if element has derived stat
    pub async fn has_element_derived_stat(&self, element_id: &str, stat_name: &str, actor_id: &str) -> ConditionResult<bool> {
        self.element_provider.has_element_derived_stat(element_id, stat_name, actor_id).await
    }

    /// List element derived stats
    pub async fn list_element_derived_stats(&self, element_id: &str) -> ConditionResult<Vec<String>> {
        self.element_provider.list_element_derived_stats(element_id).await
    }
}

/// Trait for element condition functions
/// 
/// This trait defines the interface for individual element condition functions.
/// Each function is self-contained and only depends on the ElementDataAccessor.
#[async_trait::async_trait]
pub trait ElementConditionFunction: Send + Sync {
    /// Get the function name
    fn name(&self) -> &str;
    
    /// Evaluate the condition
    async fn evaluate(
        &self, 
        params: &[crate::types::ConditionParameter], 
        context: &crate::types::ConditionContext, 
        data_accessor: &ElementDataAccessor
    ) -> ConditionResult<bool>;
}

/// Element Function Registry
/// 
/// Manages registration and execution of element condition functions.
/// Uses plugin-based architecture for loose coupling.
pub struct ElementFunctionRegistry {
    functions: std::collections::HashMap<String, Box<dyn ElementConditionFunction>>,
    data_accessor: Arc<ElementDataAccessor>,
}

impl ElementFunctionRegistry {
    /// Create a new ElementFunctionRegistry
    pub fn new(data_accessor: Arc<ElementDataAccessor>) -> Self {
        Self {
            functions: std::collections::HashMap::new(),
            data_accessor,
        }
    }

    /// Register a new element condition function
    pub fn register_function<F: ElementConditionFunction + 'static>(&mut self, function: F) {
        self.functions.insert(function.name().to_string(), Box::new(function));
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&dyn ElementConditionFunction> {
        self.functions.get(name).map(|f| f.as_ref())
    }

    /// Execute a function by name
    pub async fn execute_function(
        &self, 
        name: &str, 
        params: &[crate::types::ConditionParameter], 
        context: &crate::types::ConditionContext
    ) -> ConditionResult<bool> {
        if let Some(function) = self.functions.get(name) {
            function.evaluate(params, context, &self.data_accessor).await
        } else {
            Err(crate::error::ConditionError::FunctionNotFound { function_name: name.to_string() })
        }
    }

    /// List all registered function names
    pub fn list_functions(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
}
