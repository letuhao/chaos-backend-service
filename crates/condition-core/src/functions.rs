//! Refactored condition functions using data providers

use super::types::*;
use super::error::*;
use super::data_provider::*;
use std::sync::Arc;

/// Get actor resource value (generic) - uses ActorDataProvider
pub struct GetActorResourceFunction {
    data_provider: Option<Arc<dyn ActorDataProvider>>,
}

impl GetActorResourceFunction {
    pub fn new(data_provider: Option<Arc<dyn ActorDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetActorResourceFunction {
    fn name(&self) -> &str {
        "get_actor_resource"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Actor data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(resource_type)) = parameters.first() {
            let value = provider.get_actor_resource(resource_type, &context.target.id).await?;
            Ok(ConditionValue::Float(value))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "resource_type".to_string(),
            })
        }
    }
}

/// Get actor stat value (generic)
pub struct GetActorStatFunction {
    data_provider: Option<Arc<dyn ActorDataProvider>>,
}

impl GetActorStatFunction {
    pub fn new(data_provider: Option<Arc<dyn ActorDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetActorStatFunction {
    fn name(&self) -> &str {
        "get_actor_stat"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Actor data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(stat_name)) = parameters.first() {
            let value = provider.get_actor_stat(stat_name, &context.target.id).await?;
            Ok(ConditionValue::Float(value))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "stat_name".to_string(),
            })
        }
    }
}

/// Get actor derived stat value
pub struct GetActorDerivedStatFunction {
    data_provider: Option<Arc<dyn ActorDataProvider>>,
}

impl GetActorDerivedStatFunction {
    pub fn new(data_provider: Option<Arc<dyn ActorDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetActorDerivedStatFunction {
    fn name(&self) -> &str {
        "get_actor_derived_stat"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Actor data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(stat_name)) = parameters.first() {
            let value = provider.get_actor_derived_stat(stat_name, &context.target.id).await?;
            Ok(ConditionValue::Float(value))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "stat_name".to_string(),
            })
        }
    }
}

/// Check if actor is in combat
pub struct IsActorInCombatFunction {
    data_provider: Option<Arc<dyn ActorDataProvider>>,
}

impl IsActorInCombatFunction {
    pub fn new(data_provider: Option<Arc<dyn ActorDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsActorInCombatFunction {
    fn name(&self) -> &str {
        "is_actor_in_combat"
    }

    async fn evaluate(
        &self,
        _parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Actor data provider not available".to_string(),
            })?;

        let result = provider.is_actor_in_combat(&context.target.id).await?;
        Ok(ConditionValue::Boolean(result))
    }
}

/// Check if actor has status effects of specific type
pub struct HasActorStatusEffectsFunction {
    data_provider: Option<Arc<dyn ActorDataProvider>>,
}

impl HasActorStatusEffectsFunction {
    pub fn new(data_provider: Option<Arc<dyn ActorDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasActorStatusEffectsFunction {
    fn name(&self) -> &str {
        "has_actor_status_effects"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Actor data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(status_type)) = parameters.first() {
            let result = provider.has_actor_status_effects(status_type, &context.target.id).await?;
            Ok(ConditionValue::Boolean(result))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "status_type".to_string(),
            })
        }
    }
}

/// Get actor status effect count
pub struct GetActorStatusEffectCountFunction {
    data_provider: Option<Arc<dyn ActorDataProvider>>,
}

impl GetActorStatusEffectCountFunction {
    pub fn new(data_provider: Option<Arc<dyn ActorDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetActorStatusEffectCountFunction {
    fn name(&self) -> &str {
        "get_actor_status_effect_count"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Actor data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(status_type)) = parameters.first() {
            let count = provider.get_actor_status_effect_count(status_type, &context.target.id).await?;
            Ok(ConditionValue::Integer(count))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "status_type".to_string(),
            })
        }
    }
}


/// Check if resource is below custom threshold
pub struct IsResourceBelowThresholdFunction {
    data_provider: Option<Arc<dyn ResourceDataProvider>>,
}

impl IsResourceBelowThresholdFunction {
    pub fn new(data_provider: Option<Arc<dyn ResourceDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsResourceBelowThresholdFunction {
    fn name(&self) -> &str {
        "is_resource_below_threshold"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Resource data provider not available".to_string(),
            })?;

        if let (Some(ConditionParameter::String(resource_type)), Some(ConditionParameter::Float(threshold))) = 
            (parameters.get(0), parameters.get(1)) {
            let is_below = provider.is_resource_below_threshold(resource_type, *threshold, &context.target.id).await?;
            Ok(ConditionValue::Boolean(is_below))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "resource_type, threshold".to_string(),
            })
        }
    }
}

/// Check if resource is above custom threshold
pub struct IsResourceAboveThresholdFunction {
    data_provider: Option<Arc<dyn ResourceDataProvider>>,
}

impl IsResourceAboveThresholdFunction {
    pub fn new(data_provider: Option<Arc<dyn ResourceDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsResourceAboveThresholdFunction {
    fn name(&self) -> &str {
        "is_resource_above_threshold"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Resource data provider not available".to_string(),
            })?;

        if let (Some(ConditionParameter::String(resource_type)), Some(ConditionParameter::Float(threshold))) = 
            (parameters.get(0), parameters.get(1)) {
            let is_above = provider.is_resource_above_threshold(resource_type, *threshold, &context.target.id).await?;
            Ok(ConditionValue::Boolean(is_above))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "resource_type, threshold".to_string(),
            })
        }
    }
}

/// Check if resource is below percentage threshold
pub struct IsResourceBelowPercentageFunction {
    data_provider: Option<Arc<dyn ResourceDataProvider>>,
}

impl IsResourceBelowPercentageFunction {
    pub fn new(data_provider: Option<Arc<dyn ResourceDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsResourceBelowPercentageFunction {
    fn name(&self) -> &str {
        "is_resource_below_percentage"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Resource data provider not available".to_string(),
            })?;

        if let (Some(ConditionParameter::String(resource_type)), Some(ConditionParameter::Float(percentage))) = 
            (parameters.get(0), parameters.get(1)) {
            let is_below = provider.is_resource_below_percentage(resource_type, *percentage, &context.target.id).await?;
            Ok(ConditionValue::Boolean(is_below))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "resource_type, percentage".to_string(),
            })
        }
    }
}

/// Check if resource is above percentage threshold
pub struct IsResourceAbovePercentageFunction {
    data_provider: Option<Arc<dyn ResourceDataProvider>>,
}

impl IsResourceAbovePercentageFunction {
    pub fn new(data_provider: Option<Arc<dyn ResourceDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsResourceAbovePercentageFunction {
    fn name(&self) -> &str {
        "is_resource_above_percentage"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Resource data provider not available".to_string(),
            })?;

        if let (Some(ConditionParameter::String(resource_type)), Some(ConditionParameter::Float(percentage))) = 
            (parameters.get(0), parameters.get(1)) {
            let is_above = provider.is_resource_above_percentage(resource_type, *percentage, &context.target.id).await?;
            Ok(ConditionValue::Boolean(is_above))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "resource_type, percentage".to_string(),
            })
        }
    }
}

/// Check if actor has element affinity
pub struct HasElementAffinityFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl HasElementAffinityFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasElementAffinityFunction {
    fn name(&self) -> &str {
        "has_element_affinity"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(element_id)) = parameters.first() {
            let has_affinity = provider.has_element_affinity(element_id, &context.target.id).await?;
            Ok(ConditionValue::Boolean(has_affinity))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_id".to_string(),
            })
        }
    }
}

/// Get element interaction
pub struct GetElementInteractionFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl GetElementInteractionFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetElementInteractionFunction {
    fn name(&self) -> &str {
        "get_element_interaction"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        _context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 2 {
            if let (Some(ConditionParameter::String(source_element)), Some(ConditionParameter::String(target_element))) = 
                (parameters.get(0), parameters.get(1)) {
                let interaction = provider.get_element_interaction(source_element, target_element).await?;
                Ok(ConditionValue::String(interaction))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "source_element and target_element".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "source_element and target_element".to_string(),
            })
        }
    }
}

/// Get element mastery level
pub struct GetElementMasteryFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl GetElementMasteryFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetElementMasteryFunction {
    fn name(&self) -> &str {
        "get_element_mastery"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(element_type)) = parameters.first() {
            let mastery = provider.get_element_mastery(element_type, &context.target.id).await?;
            Ok(ConditionValue::Float(mastery))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_type".to_string(),
            })
        }
    }
}

/// Get element resistance value
pub struct GetElementResistanceFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl GetElementResistanceFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetElementResistanceFunction {
    fn name(&self) -> &str {
        "get_element_resistance"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(element_type)) = parameters.first() {
            let resistance = provider.get_element_resistance(element_type, &context.target.id).await?;
            Ok(ConditionValue::Float(resistance))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_type".to_string(),
            })
        }
    }
}

/// Check if actor has element weakness
pub struct HasElementWeaknessFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl HasElementWeaknessFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasElementWeaknessFunction {
    fn name(&self) -> &str {
        "has_element_weakness"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(element_type)) = parameters.first() {
            let has_weakness = provider.is_element_weakness(element_type, &context.target.id).await?;
            Ok(ConditionValue::Boolean(has_weakness))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_type".to_string(),
            })
        }
    }
}

/// Check if elements are in same category
pub struct IsElementSameCategoryFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl IsElementSameCategoryFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsElementSameCategoryFunction {
    fn name(&self) -> &str {
        "is_element_same_category"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        _context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 2 {
            if let (Some(ConditionParameter::String(element1)), Some(ConditionParameter::String(element2))) = 
                (parameters.get(0), parameters.get(1)) {
                let is_same = provider.is_element_same_category(element1, element2).await?;
                Ok(ConditionValue::Boolean(is_same))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "element1 and element2".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element1 and element2".to_string(),
            })
        }
    }
}

/// Check if elements are in generating relationship
pub struct IsElementGeneratingFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl IsElementGeneratingFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsElementGeneratingFunction {
    fn name(&self) -> &str {
        "is_element_generating"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        _context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 2 {
            if let (Some(ConditionParameter::String(source_element)), Some(ConditionParameter::String(target_element))) = 
                (parameters.get(0), parameters.get(1)) {
                let is_generating = provider.is_element_generating(source_element, target_element).await?;
                Ok(ConditionValue::Boolean(is_generating))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "source_element and target_element".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "source_element and target_element".to_string(),
            })
        }
    }
}

/// Check if elements are in overcoming relationship
pub struct IsElementOvercomingFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl IsElementOvercomingFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsElementOvercomingFunction {
    fn name(&self) -> &str {
        "is_element_overcoming"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        _context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 2 {
            if let (Some(ConditionParameter::String(source_element)), Some(ConditionParameter::String(target_element))) = 
                (parameters.get(0), parameters.get(1)) {
                let is_overcoming = provider.is_element_overcoming(source_element, target_element).await?;
                Ok(ConditionValue::Boolean(is_overcoming))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "source_element and target_element".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "source_element and target_element".to_string(),
            })
        }
    }
}

/// Check if elements are neutral
pub struct IsElementNeutralFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl IsElementNeutralFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsElementNeutralFunction {
    fn name(&self) -> &str {
        "is_element_neutral"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        _context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 2 {
            if let (Some(ConditionParameter::String(source_element)), Some(ConditionParameter::String(target_element))) = 
                (parameters.get(0), parameters.get(1)) {
                let is_neutral = provider.is_element_neutral(source_element, target_element).await?;
                Ok(ConditionValue::Boolean(is_neutral))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "source_element and target_element".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "source_element and target_element".to_string(),
            })
        }
    }
}

/// Check if actor has element status effect
pub struct HasElementStatusEffectFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl HasElementStatusEffectFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasElementStatusEffectFunction {
    fn name(&self) -> &str {
        "has_element_status_effect"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 2 {
            if let (Some(ConditionParameter::String(element_id)), Some(ConditionParameter::String(status_id))) = 
                (parameters.get(0), parameters.get(1)) {
                let has_status = provider.has_element_status_effect(element_id, status_id, &context.target.id).await?;
                Ok(ConditionValue::Boolean(has_status))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "element_id and status_id".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_id and status_id".to_string(),
            })
        }
    }
}

/// Get element status effect count
pub struct GetElementStatusEffectCountFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl GetElementStatusEffectCountFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetElementStatusEffectCountFunction {
    fn name(&self) -> &str {
        "get_element_status_effect_count"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 2 {
            if let (Some(ConditionParameter::String(element_id)), Some(ConditionParameter::String(status_id))) = 
                (parameters.get(0), parameters.get(1)) {
                let count = provider.get_element_status_effect_count(element_id, status_id, &context.target.id).await?;
                Ok(ConditionValue::Integer(count))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "element_id and status_id".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_id and status_id".to_string(),
            })
        }
    }
}

/// Check if element status effect is active
pub struct IsElementStatusEffectActiveFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl IsElementStatusEffectActiveFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsElementStatusEffectActiveFunction {
    fn name(&self) -> &str {
        "is_element_status_effect_active"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 2 {
            if let (Some(ConditionParameter::String(element_id)), Some(ConditionParameter::String(status_id))) = 
                (parameters.get(0), parameters.get(1)) {
                let is_active = provider.is_element_status_effect_active(element_id, status_id, &context.target.id).await?;
                Ok(ConditionValue::Boolean(is_active))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "element_id and status_id".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_id and status_id".to_string(),
            })
        }
    }
}

/// Check if actor has element resource
pub struct HasElementResourceFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl HasElementResourceFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasElementResourceFunction {
    fn name(&self) -> &str {
        "has_element_resource"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 2 {
            if let (Some(ConditionParameter::String(element_id)), Some(ConditionParameter::String(resource_type))) = 
                (parameters.get(0), parameters.get(1)) {
                let has_resource = provider.has_element_resource(element_id, resource_type, &context.target.id).await?;
                Ok(ConditionValue::Boolean(has_resource))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "element_id and resource_type".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_id and resource_type".to_string(),
            })
        }
    }
}

/// Get element resource value
pub struct GetElementResourceValueFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl GetElementResourceValueFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetElementResourceValueFunction {
    fn name(&self) -> &str {
        "get_element_resource_value"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 2 {
            if let (Some(ConditionParameter::String(element_id)), Some(ConditionParameter::String(resource_type))) = 
                (parameters.get(0), parameters.get(1)) {
                let value = provider.get_element_resource_value(element_id, resource_type, &context.target.id).await?;
                Ok(ConditionValue::Float(value))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "element_id and resource_type".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_id and resource_type".to_string(),
            })
        }
    }
}

/// Check if element resource is below threshold
pub struct IsElementResourceBelowThresholdFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl IsElementResourceBelowThresholdFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsElementResourceBelowThresholdFunction {
    fn name(&self) -> &str {
        "is_element_resource_below_threshold"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 3 {
            if let (Some(ConditionParameter::String(element_id)), Some(ConditionParameter::String(resource_type)), Some(ConditionParameter::Float(threshold))) = 
                (parameters.get(0), parameters.get(1), parameters.get(2)) {
                let is_below = provider.is_element_resource_below_threshold(element_id, resource_type, *threshold, &context.target.id).await?;
                Ok(ConditionValue::Boolean(is_below))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "element_id, resource_type, and threshold".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_id, resource_type, and threshold".to_string(),
            })
        }
    }
}

/// Check if element resource is above threshold
pub struct IsElementResourceAboveThresholdFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl IsElementResourceAboveThresholdFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsElementResourceAboveThresholdFunction {
    fn name(&self) -> &str {
        "is_element_resource_above_threshold"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 3 {
            if let (Some(ConditionParameter::String(element_id)), Some(ConditionParameter::String(resource_type)), Some(ConditionParameter::Float(threshold))) = 
                (parameters.get(0), parameters.get(1), parameters.get(2)) {
                let is_above = provider.is_element_resource_above_threshold(element_id, resource_type, *threshold, &context.target.id).await?;
                Ok(ConditionValue::Boolean(is_above))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "element_id, resource_type, and threshold".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_id, resource_type, and threshold".to_string(),
            })
        }
    }
}

/// Check if actor has hybrid element
pub struct HasHybridElementFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl HasHybridElementFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasHybridElementFunction {
    fn name(&self) -> &str {
        "has_hybrid_element"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(hybrid_id)) = parameters.first() {
            let has_hybrid = provider.has_hybrid_element(hybrid_id, &context.target.id).await?;
            Ok(ConditionValue::Boolean(has_hybrid))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "hybrid_id".to_string(),
            })
        }
    }
}

/// Check if hybrid element is activated
pub struct IsHybridElementActivatedFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl IsHybridElementActivatedFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsHybridElementActivatedFunction {
    fn name(&self) -> &str {
        "is_hybrid_element_activated"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(hybrid_id)) = parameters.first() {
            let is_activated = provider.is_hybrid_element_activated(hybrid_id, &context.target.id).await?;
            Ok(ConditionValue::Boolean(is_activated))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "hybrid_id".to_string(),
            })
        }
    }
}

/// Get hybrid element parents
pub struct GetHybridElementParentsFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl GetHybridElementParentsFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetHybridElementParentsFunction {
    fn name(&self) -> &str {
        "get_hybrid_element_parents"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        _context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(hybrid_id)) = parameters.first() {
            let parents = provider.get_hybrid_element_parents(hybrid_id).await?;
            let parent_values = parents.into_iter().map(ConditionValue::String).collect();
            Ok(ConditionValue::List(parent_values))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "hybrid_id".to_string(),
            })
        }
    }
}

/// Get element derived stat
pub struct GetElementDerivedStatFunction {
    data_provider: Option<Arc<dyn ElementDataProvider>>,
}

impl GetElementDerivedStatFunction {
    pub fn new(data_provider: Option<Arc<dyn ElementDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetElementDerivedStatFunction {
    fn name(&self) -> &str {
        "get_element_derived_stat"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Element data provider not available".to_string(),
            })?;

        if parameters.len() >= 2 {
            if let (Some(ConditionParameter::String(element_id)), Some(ConditionParameter::String(stat_name))) = 
                (parameters.get(0), parameters.get(1)) {
                let stat_value = provider.get_element_derived_stat(element_id, stat_name, &context.target.id).await?;
                Ok(ConditionValue::Float(stat_value))
            } else {
                Err(ConditionError::InvalidParameter {
                    function_name: self.name().to_string(),
                    parameter: "element_id and stat_name".to_string(),
                })
            }
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "element_id and stat_name".to_string(),
            })
        }
    }
}

/// Check if actor has category item
pub struct HasCategoryItemFunction {
    data_provider: Option<Arc<dyn CategoryDataProvider>>,
}

/// Check if category is available
pub struct IsCategoryAvailableFunction {
    data_provider: Option<Arc<dyn CategoryDataProvider>>,
}

impl IsCategoryAvailableFunction {
    pub fn new(data_provider: Option<Arc<dyn CategoryDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsCategoryAvailableFunction {
    fn name(&self) -> &str {
        "is_category_available"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Category data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(category_id)) = parameters.first() {
            let is_available = provider.is_category_available(category_id, &context.target.id).await?;
            Ok(ConditionValue::Boolean(is_available))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "category_id".to_string(),
            })
        }
    }
}

impl HasCategoryItemFunction {
    pub fn new(data_provider: Option<Arc<dyn CategoryDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasCategoryItemFunction {
    fn name(&self) -> &str {
        "has_category_item"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Category data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(category)) = parameters.first() {
            let has_item = provider.has_category_item(category, &context.target.id).await?;
            Ok(ConditionValue::Boolean(has_item))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "category".to_string(),
            })
        }
    }
}

// Status Condition Function Implementations

/// Check if actor has status effect
pub struct HasStatusEffectFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl HasStatusEffectFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasStatusEffectFunction {
    fn name(&self) -> &str {
        "has_status_effect"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let has_effect = provider.has_status_effect(&context.target.id, effect_id).await?;
            Ok(ConditionValue::Boolean(has_effect))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Get count of specific status effect
pub struct GetStatusEffectCountFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl GetStatusEffectCountFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetStatusEffectCountFunction {
    fn name(&self) -> &str {
        "get_status_effect_count"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let count = provider.get_status_effect_count(&context.target.id, effect_id).await?;
            Ok(ConditionValue::Integer(count as i64))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Get magnitude of specific status effect
pub struct GetStatusEffectMagnitudeFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl GetStatusEffectMagnitudeFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetStatusEffectMagnitudeFunction {
    fn name(&self) -> &str {
        "get_status_effect_magnitude"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let magnitude = provider.get_status_effect_magnitude(&context.target.id, effect_id).await?;
            Ok(ConditionValue::Float(magnitude))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Check if status effect is active
pub struct IsStatusEffectActiveFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl IsStatusEffectActiveFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsStatusEffectActiveFunction {
    fn name(&self) -> &str {
        "is_status_effect_active"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let is_active = provider.is_status_effect_active(&context.target.id, effect_id).await?;
            Ok(ConditionValue::Boolean(is_active))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Check if status effect is expired
pub struct IsStatusEffectExpiredFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl IsStatusEffectExpiredFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsStatusEffectExpiredFunction {
    fn name(&self) -> &str {
        "is_status_effect_expired"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let is_expired = provider.is_status_effect_expired(&context.target.id, effect_id).await?;
            Ok(ConditionValue::Boolean(is_expired))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Check if actor has immunity to specific effect
pub struct HasStatusImmunityFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl HasStatusImmunityFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasStatusImmunityFunction {
    fn name(&self) -> &str {
        "has_status_immunity"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let has_immunity = provider.has_status_immunity(&context.target.id, effect_id).await?;
            Ok(ConditionValue::Boolean(has_immunity))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Get count of specific immunity
pub struct GetStatusImmunityCountFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl GetStatusImmunityCountFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetStatusImmunityCountFunction {
    fn name(&self) -> &str {
        "get_status_immunity_count"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let count = provider.get_status_immunity_count(&context.target.id, effect_id).await?;
            Ok(ConditionValue::Integer(count as i64))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Check if immunity is active
pub struct IsStatusImmunityActiveFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl IsStatusImmunityActiveFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsStatusImmunityActiveFunction {
    fn name(&self) -> &str {
        "is_status_immunity_active"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let is_active = provider.is_status_immunity_active(&context.target.id, effect_id).await?;
            Ok(ConditionValue::Boolean(is_active))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Check if actor has effects in specific category
pub struct HasStatusCategoryFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl HasStatusCategoryFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasStatusCategoryFunction {
    fn name(&self) -> &str {
        "has_status_category"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(category)) = parameters.first() {
            let has_category = provider.has_status_category(&context.target.id, category).await?;
            Ok(ConditionValue::Boolean(has_category))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "category".to_string(),
            })
        }
    }
}

/// Get count of effects in specific category
pub struct GetStatusCategoryCountFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl GetStatusCategoryCountFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetStatusCategoryCountFunction {
    fn name(&self) -> &str {
        "get_status_category_count"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(category)) = parameters.first() {
            let count = provider.get_status_category_count(&context.target.id, category).await?;
            Ok(ConditionValue::Integer(count as i64))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "category".to_string(),
            })
        }
    }
}

/// Check if effect can be stacked
pub struct IsStatusEffectStackableFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl IsStatusEffectStackableFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for IsStatusEffectStackableFunction {
    fn name(&self) -> &str {
        "is_status_effect_stackable"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        _context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let is_stackable = provider.is_status_effect_stackable(effect_id).await?;
            Ok(ConditionValue::Boolean(is_stackable))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Check if effect can stack on actor
pub struct CanStatusEffectStackFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl CanStatusEffectStackFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for CanStatusEffectStackFunction {
    fn name(&self) -> &str {
        "can_status_effect_stack"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let can_stack = provider.can_status_effect_stack(&context.target.id, effect_id).await?;
            Ok(ConditionValue::Boolean(can_stack))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Get interaction between two effects
pub struct GetStatusEffectInteractionFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl GetStatusEffectInteractionFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetStatusEffectInteractionFunction {
    fn name(&self) -> &str {
        "get_status_effect_interaction"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        _context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if parameters.len() != 2 {
            return Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "expected 2 parameters: effect_id, target_effect_id".to_string(),
            });
        }

        let effect_id = parameters[0].as_string()?;
        let target_effect_id = parameters[1].as_string()?;
        let interaction = provider.get_status_effect_interaction(effect_id, target_effect_id).await?;
        Ok(ConditionValue::String(interaction))
    }
}

/// Get priority of status effect
pub struct GetStatusEffectPriorityFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl GetStatusEffectPriorityFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetStatusEffectPriorityFunction {
    fn name(&self) -> &str {
        "get_status_effect_priority"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        _context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let priority = provider.get_status_effect_priority(effect_id).await?;
            Ok(ConditionValue::Integer(priority as i64))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Check if actor has movement restriction
pub struct HasStatusMovementRestrictionFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl HasStatusMovementRestrictionFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasStatusMovementRestrictionFunction {
    fn name(&self) -> &str {
        "has_status_movement_restriction"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(restriction_type)) = parameters.first() {
            let has_restriction = provider.has_status_movement_restriction(&context.target.id, restriction_type).await?;
            Ok(ConditionValue::Boolean(has_restriction))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "restriction_type".to_string(),
            })
        }
    }
}

/// Get magnitude of movement restriction
pub struct GetStatusMovementRestrictionFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl GetStatusMovementRestrictionFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for GetStatusMovementRestrictionFunction {
    fn name(&self) -> &str {
        "get_status_movement_restriction"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(restriction_type)) = parameters.first() {
            let magnitude = provider.get_status_movement_restriction(&context.target.id, restriction_type).await?;
            Ok(ConditionValue::Float(magnitude))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "restriction_type".to_string(),
            })
        }
    }
}

/// Check if actor has visual effect
pub struct HasStatusVisualEffectFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl HasStatusVisualEffectFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasStatusVisualEffectFunction {
    fn name(&self) -> &str {
        "has_status_visual_effect"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let has_visual = provider.has_status_visual_effect(&context.target.id, effect_id).await?;
            Ok(ConditionValue::Boolean(has_visual))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Check if actor has audio effect
pub struct HasStatusAudioEffectFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl HasStatusAudioEffectFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasStatusAudioEffectFunction {
    fn name(&self) -> &str {
        "has_status_audio_effect"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if let Some(ConditionParameter::String(effect_id)) = parameters.first() {
            let has_audio = provider.has_status_audio_effect(&context.target.id, effect_id).await?;
            Ok(ConditionValue::Boolean(has_audio))
        } else {
            Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "effect_id".to_string(),
            })
        }
    }
}

/// Check if effect has specific property
pub struct HasStatusEffectPropertyFunction {
    data_provider: Option<Arc<dyn StatusDataProvider>>,
}

impl HasStatusEffectPropertyFunction {
    pub fn new(data_provider: Option<Arc<dyn StatusDataProvider>>) -> Self {
        Self { data_provider }
    }
}

#[async_trait::async_trait]
impl ConditionFunction for HasStatusEffectPropertyFunction {
    fn name(&self) -> &str {
        "has_status_effect_property"
    }

    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue> {
        let provider = self.data_provider.as_ref()
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Status data provider not available".to_string(),
            })?;

        if parameters.len() != 2 {
            return Err(ConditionError::InvalidParameter {
                function_name: self.name().to_string(),
                parameter: "expected 2 parameters: effect_id, property".to_string(),
            });
        }

        let effect_id = parameters[0].as_string()?;
        let property = parameters[1].as_string()?;
        let has_property = provider.has_status_effect_property(&context.target.id, effect_id, property).await?;
        Ok(ConditionValue::Boolean(has_property))
    }
}

/// Create function registry with data providers
pub fn create_function_registry_with_providers(
    data_registry: &DataProviderRegistry,
) -> FunctionRegistry {
    let mut registry = FunctionRegistry::new();
    
    // Register Actor Data Provider functions
    registry.register(Box::new(GetActorResourceFunction::new(
        data_registry.get_actor_provider()
    )));
    
    registry.register(Box::new(GetActorStatFunction::new(
        data_registry.get_actor_provider()
    )));
    
    registry.register(Box::new(GetActorDerivedStatFunction::new(
        data_registry.get_actor_provider()
    )));
    
    registry.register(Box::new(IsActorInCombatFunction::new(
        data_registry.get_actor_provider()
    )));
    
    registry.register(Box::new(HasActorStatusEffectsFunction::new(
        data_registry.get_actor_provider()
    )));
    
    registry.register(Box::new(GetActorStatusEffectCountFunction::new(
        data_registry.get_actor_provider()
    )));
    
    // Register Resource Data Provider functions
    registry.register(Box::new(IsResourceBelowThresholdFunction::new(
        data_registry.get_resource_provider()
    )));
    
    registry.register(Box::new(IsResourceAboveThresholdFunction::new(
        data_registry.get_resource_provider()
    )));
    
    registry.register(Box::new(IsResourceBelowPercentageFunction::new(
        data_registry.get_resource_provider()
    )));
    
    registry.register(Box::new(IsResourceAbovePercentageFunction::new(
        data_registry.get_resource_provider()
    )));
    
    // Register Element Data Provider functions
    registry.register(Box::new(GetElementMasteryFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(GetElementResistanceFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(HasElementAffinityFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(HasElementWeaknessFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(GetElementInteractionFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(IsElementSameCategoryFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(IsElementGeneratingFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(IsElementOvercomingFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(IsElementNeutralFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(HasElementStatusEffectFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(GetElementStatusEffectCountFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(IsElementStatusEffectActiveFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(HasElementResourceFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(GetElementResourceValueFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(IsElementResourceBelowThresholdFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(IsElementResourceAboveThresholdFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(HasHybridElementFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(IsHybridElementActivatedFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(GetHybridElementParentsFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(GetElementDerivedStatFunction::new(
        data_registry.get_element_provider()
    )));
    
    // Register Status Data Provider functions
    registry.register(Box::new(HasStatusEffectFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(GetStatusEffectCountFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(GetStatusEffectMagnitudeFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(IsStatusEffectActiveFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(IsStatusEffectExpiredFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(HasStatusImmunityFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(GetStatusImmunityCountFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(IsStatusImmunityActiveFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(HasStatusCategoryFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(GetStatusCategoryCountFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(IsStatusEffectStackableFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(CanStatusEffectStackFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(GetStatusEffectInteractionFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(GetStatusEffectPriorityFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(HasStatusMovementRestrictionFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(GetStatusMovementRestrictionFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(HasStatusVisualEffectFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(HasStatusAudioEffectFunction::new(
        data_registry.get_status_provider()
    )));
    
    registry.register(Box::new(HasStatusEffectPropertyFunction::new(
        data_registry.get_status_provider()
    )));
    
    // Register Category Data Provider functions
    registry.register(Box::new(HasCategoryItemFunction::new(
        data_registry.get_category_provider()
    )));
    
    registry.register(Box::new(IsCategoryAvailableFunction::new(
        data_registry.get_category_provider()
    )));
    
    registry
}
