//! Status condition functions for Condition Core

use crate::data_accessor::{ElementConditionFunction, ElementDataAccessor, ElementFunctionRegistry};
use crate::data_provider::StatusDataProvider;
use crate::error::{ConditionResult, ConditionError};
use crate::types::{ConditionParameter, ConditionContext};
use std::sync::Arc;
use async_trait::async_trait;

/// Status Data Accessor acts as a facade for StatusDataProvider.
/// It provides a focused interface for status condition functions, reducing their direct dependency
/// on the full StatusDataProvider trait.
pub struct StatusDataAccessor {
    status_provider: Arc<dyn StatusDataProvider + Send + Sync>,
}

impl StatusDataAccessor {
    pub fn new(status_provider: Arc<dyn StatusDataProvider + Send + Sync>) -> Self {
        Self { status_provider }
    }

    // Methods that delegate to status_provider, exposing only what's needed by functions
    pub async fn has_status_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        self.status_provider.has_status_effect(actor_id, effect_id).await
    }

    pub async fn get_status_effect_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32> {
        self.status_provider.get_status_effect_count(actor_id, effect_id).await
    }

    pub async fn get_status_effect_magnitude(&self, actor_id: &str, effect_id: &str) -> ConditionResult<f64> {
        self.status_provider.get_status_effect_magnitude(actor_id, effect_id).await
    }

    pub async fn is_status_effect_active(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        self.status_provider.is_status_effect_active(actor_id, effect_id).await
    }

    pub async fn is_status_effect_expired(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        self.status_provider.is_status_effect_expired(actor_id, effect_id).await
    }

    pub async fn has_status_immunity(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        self.status_provider.has_status_immunity(actor_id, effect_id).await
    }

    pub async fn get_status_immunity_count(&self, actor_id: &str, effect_id: &str) -> ConditionResult<u32> {
        self.status_provider.get_status_immunity_count(actor_id, effect_id).await
    }

    pub async fn is_status_immunity_active(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        self.status_provider.is_status_immunity_active(actor_id, effect_id).await
    }

    pub async fn has_status_category(&self, actor_id: &str, category: &str) -> ConditionResult<bool> {
        self.status_provider.has_status_category(actor_id, category).await
    }

    pub async fn get_status_category_count(&self, actor_id: &str, category: &str) -> ConditionResult<u32> {
        self.status_provider.get_status_category_count(actor_id, category).await
    }

    pub async fn list_status_categories(&self, actor_id: &str) -> ConditionResult<Vec<String>> {
        self.status_provider.list_status_categories(actor_id).await
    }

    pub async fn is_status_effect_stackable(&self, effect_id: &str) -> ConditionResult<bool> {
        self.status_provider.is_status_effect_stackable(effect_id).await
    }

    pub async fn can_status_effect_stack(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        self.status_provider.can_status_effect_stack(actor_id, effect_id).await
    }

    pub async fn get_status_effect_interaction(&self, effect_id: &str, target_effect_id: &str) -> ConditionResult<String> {
        self.status_provider.get_status_effect_interaction(effect_id, target_effect_id).await
    }

    pub async fn get_status_effect_priority(&self, effect_id: &str) -> ConditionResult<i32> {
        self.status_provider.get_status_effect_priority(effect_id).await
    }

    pub async fn get_status_effect_source(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String> {
        self.status_provider.get_status_effect_source(actor_id, effect_id).await
    }

    pub async fn get_status_effect_target(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String> {
        self.status_provider.get_status_effect_target(actor_id, effect_id).await
    }

    pub async fn has_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<bool> {
        self.status_provider.has_status_movement_restriction(actor_id, restriction_type).await
    }

    pub async fn get_status_movement_restriction(&self, actor_id: &str, restriction_type: &str) -> ConditionResult<f64> {
        self.status_provider.get_status_movement_restriction(actor_id, restriction_type).await
    }

    pub async fn has_status_visual_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        self.status_provider.has_status_visual_effect(actor_id, effect_id).await
    }

    pub async fn get_status_visual_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String> {
        self.status_provider.get_status_visual_effect(actor_id, effect_id).await
    }

    pub async fn has_status_audio_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<bool> {
        self.status_provider.has_status_audio_effect(actor_id, effect_id).await
    }

    pub async fn get_status_audio_effect(&self, actor_id: &str, effect_id: &str) -> ConditionResult<String> {
        self.status_provider.get_status_audio_effect(actor_id, effect_id).await
    }

    pub async fn get_status_effect_properties(&self, actor_id: &str, effect_id: &str) -> ConditionResult<std::collections::HashMap<String, serde_json::Value>> {
        self.status_provider.get_status_effect_properties(actor_id, effect_id).await
    }

    pub async fn has_status_effect_property(&self, actor_id: &str, effect_id: &str, property: &str) -> ConditionResult<bool> {
        self.status_provider.has_status_effect_property(actor_id, effect_id, property).await
    }

    pub async fn get_status_effect_property(&self, actor_id: &str, effect_id: &str, property: &str) -> ConditionResult<serde_json::Value> {
        self.status_provider.get_status_effect_property(actor_id, effect_id, property).await
    }
}

/// Trait for status condition functions
/// This trait defines the interface for individual status condition functions.
/// Each function is self-contained and only depends on the StatusDataAccessor.
#[async_trait]
pub trait StatusConditionFunction: Send + Sync {
    /// Get the function name
    fn name(&self) -> &str;

    /// Evaluate the condition
    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool>;
}

/// Status Function Registry
/// Manages registration and execution of status condition functions.
/// Uses plugin-based architecture for loose coupling.
pub struct StatusFunctionRegistry {
    functions: std::collections::HashMap<String, Arc<dyn StatusConditionFunction>>,
    data_accessor: StatusDataAccessor,
}

impl StatusFunctionRegistry {
    pub fn new(status_provider: Arc<dyn StatusDataProvider + Send + Sync>) -> Self {
        Self {
            functions: std::collections::HashMap::new(),
            data_accessor: StatusDataAccessor::new(status_provider),
        }
    }

    pub fn register_function<F>(&mut self, function: F)
    where
        F: StatusConditionFunction + 'static,
    {
        self.functions.insert(function.name().to_string(), Arc::new(function));
    }

    pub async fn execute_function(
        &self,
        name: &str,
        params: &[ConditionParameter],
        context: &ConditionContext
    ) -> ConditionResult<bool> {
        if let Some(function) = self.functions.get(name) {
            function.evaluate(params, context, &self.data_accessor).await
        } else {
            Err(ConditionError::FunctionNotFound { function_name: name.to_string() })
        }
    }

    pub fn list_functions(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }
}

// Status Condition Function Implementations

/// Check if actor has specific status effect
pub struct HasStatusEffectFunction;

#[async_trait]
impl StatusConditionFunction for HasStatusEffectFunction {
    fn name(&self) -> &str {
        "has_status_effect"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        data_accessor.has_status_effect(&context.target.id, effect_id).await
    }
}

/// Get count of specific status effect
pub struct GetStatusEffectCountFunction;

#[async_trait]
impl StatusConditionFunction for GetStatusEffectCountFunction {
    fn name(&self) -> &str {
        "get_status_effect_count"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        let count = data_accessor.get_status_effect_count(&context.target.id, effect_id).await?;
        Ok(count > 0)
    }
}

/// Get magnitude of specific status effect
pub struct GetStatusEffectMagnitudeFunction;

#[async_trait]
impl StatusConditionFunction for GetStatusEffectMagnitudeFunction {
    fn name(&self) -> &str {
        "get_status_effect_magnitude"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        let magnitude = data_accessor.get_status_effect_magnitude(&context.target.id, effect_id).await?;
        Ok(magnitude > 0.0)
    }
}

/// Check if status effect is active
pub struct IsStatusEffectActiveFunction;

#[async_trait]
impl StatusConditionFunction for IsStatusEffectActiveFunction {
    fn name(&self) -> &str {
        "is_status_effect_active"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        data_accessor.is_status_effect_active(&context.target.id, effect_id).await
    }
}

/// Check if status effect is expired
pub struct IsStatusEffectExpiredFunction;

#[async_trait]
impl StatusConditionFunction for IsStatusEffectExpiredFunction {
    fn name(&self) -> &str {
        "is_status_effect_expired"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        data_accessor.is_status_effect_expired(&context.target.id, effect_id).await
    }
}

/// Check if actor has immunity to specific effect
pub struct HasStatusImmunityFunction;

#[async_trait]
impl StatusConditionFunction for HasStatusImmunityFunction {
    fn name(&self) -> &str {
        "has_status_immunity"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        data_accessor.has_status_immunity(&context.target.id, effect_id).await
    }
}

/// Get count of specific immunity
pub struct GetStatusImmunityCountFunction;

#[async_trait]
impl StatusConditionFunction for GetStatusImmunityCountFunction {
    fn name(&self) -> &str {
        "get_status_immunity_count"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        let count = data_accessor.get_status_immunity_count(&context.target.id, effect_id).await?;
        Ok(count > 0)
    }
}

/// Check if immunity is active
pub struct IsStatusImmunityActiveFunction;

#[async_trait]
impl StatusConditionFunction for IsStatusImmunityActiveFunction {
    fn name(&self) -> &str {
        "is_status_immunity_active"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        data_accessor.is_status_immunity_active(&context.target.id, effect_id).await
    }
}

/// Check if actor has effects in specific category
pub struct HasStatusCategoryFunction;

#[async_trait]
impl StatusConditionFunction for HasStatusCategoryFunction {
    fn name(&self) -> &str {
        "has_status_category"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let category = params[0].as_string()?;
        data_accessor.has_status_category(&context.target.id, category).await
    }
}

/// Get count of effects in specific category
pub struct GetStatusCategoryCountFunction;

#[async_trait]
impl StatusConditionFunction for GetStatusCategoryCountFunction {
    fn name(&self) -> &str {
        "get_status_category_count"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let category = params[0].as_string()?;
        let count = data_accessor.get_status_category_count(&context.target.id, category).await?;
        Ok(count > 0)
    }
}

/// Check if effect can be stacked
pub struct IsStatusEffectStackableFunction;

#[async_trait]
impl StatusConditionFunction for IsStatusEffectStackableFunction {
    fn name(&self) -> &str {
        "is_status_effect_stackable"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        data_accessor.is_status_effect_stackable(effect_id).await
    }
}

/// Check if effect can stack on actor
pub struct CanStatusEffectStackFunction;

#[async_trait]
impl StatusConditionFunction for CanStatusEffectStackFunction {
    fn name(&self) -> &str {
        "can_status_effect_stack"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        data_accessor.can_status_effect_stack(&context.target.id, effect_id).await
    }
}

/// Get interaction between two effects
pub struct GetStatusEffectInteractionFunction;

#[async_trait]
impl StatusConditionFunction for GetStatusEffectInteractionFunction {
    fn name(&self) -> &str {
        "get_status_effect_interaction"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 2 {
            return Err(ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        let target_effect_id = params[1].as_string()?;
        let interaction = data_accessor.get_status_effect_interaction(effect_id, target_effect_id).await?;
        Ok(!interaction.is_empty())
    }
}

/// Get priority of status effect
pub struct GetStatusEffectPriorityFunction;

#[async_trait]
impl StatusConditionFunction for GetStatusEffectPriorityFunction {
    fn name(&self) -> &str {
        "get_status_effect_priority"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        let priority = data_accessor.get_status_effect_priority(effect_id).await?;
        Ok(priority > 0)
    }
}

/// Check if actor has movement restriction
pub struct HasStatusMovementRestrictionFunction;

#[async_trait]
impl StatusConditionFunction for HasStatusMovementRestrictionFunction {
    fn name(&self) -> &str {
        "has_status_movement_restriction"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let restriction_type = params[0].as_string()?;
        data_accessor.has_status_movement_restriction(&context.target.id, restriction_type).await
    }
}

/// Get magnitude of movement restriction
pub struct GetStatusMovementRestrictionFunction;

#[async_trait]
impl StatusConditionFunction for GetStatusMovementRestrictionFunction {
    fn name(&self) -> &str {
        "get_status_movement_restriction"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let restriction_type = params[0].as_string()?;
        let magnitude = data_accessor.get_status_movement_restriction(&context.target.id, restriction_type).await?;
        Ok(magnitude > 0.0)
    }
}

/// Check if actor has visual effect
pub struct HasStatusVisualEffectFunction;

#[async_trait]
impl StatusConditionFunction for HasStatusVisualEffectFunction {
    fn name(&self) -> &str {
        "has_status_visual_effect"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        data_accessor.has_status_visual_effect(&context.target.id, effect_id).await
    }
}

/// Check if actor has audio effect
pub struct HasStatusAudioEffectFunction;

#[async_trait]
impl StatusConditionFunction for HasStatusAudioEffectFunction {
    fn name(&self) -> &str {
        "has_status_audio_effect"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 1 {
            return Err(ConditionError::InvalidParameterCount { expected: 1, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        data_accessor.has_status_audio_effect(&context.target.id, effect_id).await
    }
}

/// Check if effect has specific property
pub struct HasStatusEffectPropertyFunction;

#[async_trait]
impl StatusConditionFunction for HasStatusEffectPropertyFunction {
    fn name(&self) -> &str {
        "has_status_effect_property"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &StatusDataAccessor
    ) -> ConditionResult<bool> {
        if params.len() != 2 {
            return Err(ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }
        let effect_id = params[0].as_string()?;
        let property = params[1].as_string()?;
        data_accessor.has_status_effect_property(&context.target.id, effect_id, property).await
    }
}

/// Create status function registry with all status condition functions
pub fn create_status_function_registry(
    status_provider: Arc<dyn StatusDataProvider + Send + Sync>,
) -> StatusFunctionRegistry {
    let mut registry = StatusFunctionRegistry::new(status_provider);
    
    // Register all status condition functions
    registry.register_function(HasStatusEffectFunction);
    registry.register_function(GetStatusEffectCountFunction);
    registry.register_function(GetStatusEffectMagnitudeFunction);
    registry.register_function(IsStatusEffectActiveFunction);
    registry.register_function(IsStatusEffectExpiredFunction);
    registry.register_function(HasStatusImmunityFunction);
    registry.register_function(GetStatusImmunityCountFunction);
    registry.register_function(IsStatusImmunityActiveFunction);
    registry.register_function(HasStatusCategoryFunction);
    registry.register_function(GetStatusCategoryCountFunction);
    registry.register_function(IsStatusEffectStackableFunction);
    registry.register_function(CanStatusEffectStackFunction);
    registry.register_function(GetStatusEffectInteractionFunction);
    registry.register_function(GetStatusEffectPriorityFunction);
    registry.register_function(HasStatusMovementRestrictionFunction);
    registry.register_function(GetStatusMovementRestrictionFunction);
    registry.register_function(HasStatusVisualEffectFunction);
    registry.register_function(HasStatusAudioEffectFunction);
    registry.register_function(HasStatusEffectPropertyFunction);
    
    registry
}
