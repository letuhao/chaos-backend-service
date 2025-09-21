//! Element Condition Functions - Plugin-based implementations
//! 
//! This module contains individual element condition function implementations
//! using the new plugin-based architecture. Each function is self-contained
//! and only depends on the ElementDataAccessor.

use crate::data_accessor::{ElementConditionFunction, ElementDataAccessor, ElementFunctionRegistry};
use crate::error::ConditionResult;
use crate::types::{ConditionParameter, ConditionContext};

/// Get Element Mastery Function
pub struct GetElementMasteryFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetElementMasteryFunction {
    fn name(&self) -> &str {
        "get_element_mastery"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.is_empty() {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 1, actual: 0 });
        }

        let element_id = params[0].as_string()?;
        let mastery = data_accessor.get_element_mastery(element_id, &context.target.id).await?;
        
        // For now, return true if mastery > 0, but this should be configurable
        Ok(mastery > 0.0)
    }
}

/// Has Element Affinity Function
pub struct HasElementAffinityFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for HasElementAffinityFunction {
    fn name(&self) -> &str {
        "has_element_affinity"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.is_empty() {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 1, actual: 0 });
        }

        let element_id = params[0].as_string()?;
        data_accessor.has_element_affinity(element_id, &context.target.id).await
    }
}

/// Get Element Resistance Function
pub struct GetElementResistanceFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetElementResistanceFunction {
    fn name(&self) -> &str {
        "get_element_resistance"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.is_empty() {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 1, actual: 0 });
        }

        let element_id = params[0].as_string()?;
        let resistance = data_accessor.get_element_resistance(element_id, &context.target.id).await?;
        
        // Return true if resistance > 0
        Ok(resistance > 0.0)
    }
}

/// Is Element Weakness Function
pub struct IsElementWeaknessFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsElementWeaknessFunction {
    fn name(&self) -> &str {
        "is_element_weakness"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.is_empty() {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 1, actual: 0 });
        }

        let element_id = params[0].as_string()?;
        data_accessor.is_element_weakness(element_id, &context.target.id).await
    }
}

/// Get Element Interaction Function
pub struct GetElementInteractionFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetElementInteractionFunction {
    fn name(&self) -> &str {
        "get_element_interaction"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let source_element = params[0].as_string()?;
        let target_element = params[1].as_string()?;
        let interaction = data_accessor.get_element_interaction(source_element, target_element).await?;
        
        // Return true if interaction is not neutral
        Ok(interaction != "neutral")
    }
}

/// Is Element Same Category Function
pub struct IsElementSameCategoryFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsElementSameCategoryFunction {
    fn name(&self) -> &str {
        "is_element_same_category"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let element1 = params[0].as_string()?;
        let element2 = params[1].as_string()?;
        data_accessor.is_element_same_category(element1, element2).await
    }
}

/// Is Element Generating Function
pub struct IsElementGeneratingFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsElementGeneratingFunction {
    fn name(&self) -> &str {
        "is_element_generating"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let source_element = params[0].as_string()?;
        let target_element = params[1].as_string()?;
        data_accessor.is_element_generating(source_element, target_element).await
    }
}

/// Is Element Overcoming Function
pub struct IsElementOvercomingFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsElementOvercomingFunction {
    fn name(&self) -> &str {
        "is_element_overcoming"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let source_element = params[0].as_string()?;
        let target_element = params[1].as_string()?;
        data_accessor.is_element_overcoming(source_element, target_element).await
    }
}

/// Is Element Neutral Function
pub struct IsElementNeutralFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsElementNeutralFunction {
    fn name(&self) -> &str {
        "is_element_neutral"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let source_element = params[0].as_string()?;
        let target_element = params[1].as_string()?;
        data_accessor.is_element_neutral(source_element, target_element).await
    }
}

/// Has Element Status Effect Function
pub struct HasElementStatusEffectFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for HasElementStatusEffectFunction {
    fn name(&self) -> &str {
        "has_element_status_effect"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let element_id = params[0].as_string()?;
        let status_id = params[1].as_string()?;
        data_accessor.has_element_status_effect(element_id, status_id, &context.target.id).await
    }
}

/// Get Element Status Effect Count Function
pub struct GetElementStatusEffectCountFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetElementStatusEffectCountFunction {
    fn name(&self) -> &str {
        "get_element_status_effect_count"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let element_id = params[0].as_string()?;
        let status_id = params[1].as_string()?;
        let count = data_accessor.get_element_status_effect_count(element_id, status_id, &context.target.id).await?;
        
        // Return true if count > 0
        Ok(count > 0)
    }
}

/// Is Element Status Effect Active Function
pub struct IsElementStatusEffectActiveFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsElementStatusEffectActiveFunction {
    fn name(&self) -> &str {
        "is_element_status_effect_active"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let element_id = params[0].as_string()?;
        let status_id = params[1].as_string()?;
        data_accessor.is_element_status_effect_active(element_id, status_id, &context.target.id).await
    }
}

/// Has Element Resource Function
pub struct HasElementResourceFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for HasElementResourceFunction {
    fn name(&self) -> &str {
        "has_element_resource"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let element_id = params[0].as_string()?;
        let resource_type = params[1].as_string()?;
        data_accessor.has_element_resource(element_id, resource_type, &context.target.id).await
    }
}

/// Get Element Resource Value Function
pub struct GetElementResourceValueFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetElementResourceValueFunction {
    fn name(&self) -> &str {
        "get_element_resource_value"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let element_id = params[0].as_string()?;
        let resource_type = params[1].as_string()?;
        let value = data_accessor.get_element_resource_value(element_id, resource_type, &context.target.id).await?;
        
        // Return true if value > 0
        Ok(value > 0.0)
    }
}

/// Is Element Resource Below Threshold Function
pub struct IsElementResourceBelowThresholdFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsElementResourceBelowThresholdFunction {
    fn name(&self) -> &str {
        "is_element_resource_below_threshold"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 3 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 3, actual: params.len() });
        }

        let element_id = params[0].as_string()?;
        let resource_type = params[1].as_string()?;
        let threshold = params[2].as_float()?;
        data_accessor.is_element_resource_below_threshold(element_id, resource_type, threshold, &context.target.id).await
    }
}

/// Is Element Resource Above Threshold Function
pub struct IsElementResourceAboveThresholdFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsElementResourceAboveThresholdFunction {
    fn name(&self) -> &str {
        "is_element_resource_above_threshold"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 3 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 3, actual: params.len() });
        }

        let element_id = params[0].as_string()?;
        let resource_type = params[1].as_string()?;
        let threshold = params[2].as_float()?;
        data_accessor.is_element_resource_above_threshold(element_id, resource_type, threshold, &context.target.id).await
    }
}

/// Has Hybrid Element Function
pub struct HasHybridElementFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for HasHybridElementFunction {
    fn name(&self) -> &str {
        "has_hybrid_element"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.is_empty() {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 1, actual: 0 });
        }

        let hybrid_id = params[0].as_string()?;
        data_accessor.has_hybrid_element(hybrid_id, &context.target.id).await
    }
}

/// Is Hybrid Element Activated Function
pub struct IsHybridElementActivatedFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for IsHybridElementActivatedFunction {
    fn name(&self) -> &str {
        "is_hybrid_element_activated"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.is_empty() {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 1, actual: 0 });
        }

        let hybrid_id = params[0].as_string()?;
        data_accessor.is_hybrid_element_activated(hybrid_id, &context.target.id).await
    }
}

/// Get Hybrid Element Parents Function
pub struct GetHybridElementParentsFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetHybridElementParentsFunction {
    fn name(&self) -> &str {
        "get_hybrid_element_parents"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.is_empty() {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 1, actual: 0 });
        }

        let hybrid_id = params[0].as_string()?;
        let parents = data_accessor.get_hybrid_element_parents(hybrid_id).await?;
        
        // Return true if has parents
        Ok(!parents.is_empty())
    }
}

/// Get Element Derived Stat Function
pub struct GetElementDerivedStatFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for GetElementDerivedStatFunction {
    fn name(&self) -> &str {
        "get_element_derived_stat"
    }

    async fn evaluate(
        &self,
        params: &[ConditionParameter],
        context: &ConditionContext,
        data_accessor: &ElementDataAccessor,
    ) -> ConditionResult<bool> {
        if params.len() < 2 {
            return Err(crate::error::ConditionError::InvalidParameterCount { expected: 2, actual: params.len() });
        }

        let element_id = params[0].as_string()?;
        let stat_name = params[1].as_string()?;
        let value = data_accessor.get_element_derived_stat(element_id, stat_name, &context.target.id).await?;
        
        // Return true if value > 0
        Ok(value > 0.0)
    }
}

/// Helper function to create and register all element functions
pub fn create_element_function_registry(data_accessor: std::sync::Arc<ElementDataAccessor>) -> ElementFunctionRegistry {
    let mut registry = ElementFunctionRegistry::new(data_accessor);
    
    // Register all element functions
    registry.register_function(GetElementMasteryFunction);
    registry.register_function(HasElementAffinityFunction);
    registry.register_function(GetElementResistanceFunction);
    registry.register_function(IsElementWeaknessFunction);
    registry.register_function(GetElementInteractionFunction);
    registry.register_function(IsElementSameCategoryFunction);
    registry.register_function(IsElementGeneratingFunction);
    registry.register_function(IsElementOvercomingFunction);
    registry.register_function(IsElementNeutralFunction);
    registry.register_function(HasElementStatusEffectFunction);
    registry.register_function(GetElementStatusEffectCountFunction);
    registry.register_function(IsElementStatusEffectActiveFunction);
    registry.register_function(HasElementResourceFunction);
    registry.register_function(GetElementResourceValueFunction);
    registry.register_function(IsElementResourceBelowThresholdFunction);
    registry.register_function(IsElementResourceAboveThresholdFunction);
    registry.register_function(HasHybridElementFunction);
    registry.register_function(IsHybridElementActivatedFunction);
    registry.register_function(GetHybridElementParentsFunction);
    registry.register_function(GetElementDerivedStatFunction);
    
    registry
}
