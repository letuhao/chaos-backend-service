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
    
    registry.register(Box::new(HasElementAffinityFunction::new(
        data_registry.get_element_provider()
    )));
    
    registry.register(Box::new(GetElementInteractionFunction::new(
        data_registry.get_element_provider()
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
