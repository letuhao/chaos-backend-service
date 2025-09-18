//! Condition Builder - Programmatic condition creation

use super::types::*;
use super::error::*;

/// Builder for creating conditions programmatically
pub struct ConditionBuilder {
    condition_id: Option<String>,
    function_name: Option<String>,
    parameters: Vec<ConditionParameter>,
    operator: Option<ConditionOperator>,
    value: Option<ConditionValue>,
}

impl ConditionBuilder {
    /// Create a new condition builder
    pub fn new() -> Self {
        Self {
            condition_id: None,
            function_name: None,
            parameters: Vec::new(),
            operator: None,
            value: None,
        }
    }

    /// Set the condition ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.condition_id = Some(id.into());
        self
    }

    /// Set the function name
    pub fn function(mut self, function: impl Into<String>) -> Self {
        self.function_name = Some(function.into());
        self
    }

    /// Add a parameter
    pub fn parameter(mut self, param: impl Into<ConditionParameter>) -> Self {
        self.parameters.push(param.into());
        self
    }

    /// Set the operator
    pub fn operator(mut self, op: ConditionOperator) -> Self {
        self.operator = Some(op);
        self
    }

    /// Set the value
    pub fn value(mut self, value: impl Into<ConditionValue>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Build the condition
    pub fn build(self) -> ConditionResult<ConditionConfig> {
        let condition_id = self.condition_id
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Condition ID is required".to_string(),
            })?;

        let function_name = self.function_name
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Function name is required".to_string(),
            })?;

        let operator = self.operator
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Operator is required".to_string(),
            })?;

        let value = self.value
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Value is required".to_string(),
            })?;

        Ok(ConditionConfig {
            condition_id,
            function_name,
            operator,
            value,
            parameters: self.parameters,
        })
    }
}

impl Default for ConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating condition chains programmatically
pub struct ConditionChainBuilder {
    chain_id: Option<String>,
    logic: Option<ChainLogic>,
    conditions: Vec<ConditionConfig>,
}

impl ConditionChainBuilder {
    /// Create a new condition chain builder
    pub fn new() -> Self {
        Self {
            chain_id: None,
            logic: None,
            conditions: Vec::new(),
        }
    }

    /// Set the chain ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.chain_id = Some(id.into());
        self
    }

    /// Set the logic operator
    pub fn logic(mut self, logic: ChainLogic) -> Self {
        self.logic = Some(logic);
        self
    }

    /// Add a condition to the chain
    pub fn condition(mut self, condition: ConditionConfig) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Build the condition chain
    pub fn build(self) -> ConditionResult<ConditionChainConfig> {
        let chain_id = self.chain_id
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Chain ID is required".to_string(),
            })?;

        let logic = self.logic
            .ok_or_else(|| ConditionError::ConfigError {
                message: "Logic is required".to_string(),
            })?;

        if self.conditions.is_empty() {
            return Err(ConditionError::ConfigError {
                message: "At least one condition is required".to_string(),
            });
        }

        Ok(ConditionChainConfig {
            chain_id,
            logic,
            conditions: self.conditions,
        })
    }
}

impl Default for ConditionChainBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory for creating common condition patterns
pub struct ConditionBuilderFactory;

impl ConditionBuilderFactory {
    /// Create a health check condition
    pub fn health_check(threshold: f64) -> ConditionBuilder {
        ConditionBuilder::new()
            .id("health_check")
            .function("get_actor_resource")
            .parameter("health")
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(threshold))
    }

    /// Create a mana check condition
    pub fn mana_check(threshold: f64) -> ConditionBuilder {
        ConditionBuilder::new()
            .id("mana_check")
            .function("get_actor_resource")
            .parameter("mana")
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(threshold))
    }

    /// Create a stamina check condition
    pub fn stamina_check(threshold: f64) -> ConditionBuilder {
        ConditionBuilder::new()
            .id("stamina_check")
            .function("get_actor_resource")
            .parameter("stamina")
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(threshold))
    }

    /// Create an element mastery check condition
    pub fn element_mastery_check(element: &str, threshold: f64) -> ConditionBuilder {
        ConditionBuilder::new()
            .id("element_mastery_check")
            .function("get_element_mastery")
            .parameter(element)
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(threshold))
    }

    /// Create a category item check condition
    pub fn has_category_item(category: &str) -> ConditionBuilder {
        ConditionBuilder::new()
            .id("has_category_item")
            .function("has_category_item")
            .parameter(category)
            .operator(ConditionOperator::Equal)
            .value(ConditionValue::Boolean(true))
    }

    /// Create a resource low check condition
    pub fn is_resource_low(resource: &str) -> ConditionBuilder {
        ConditionBuilder::new()
            .id("is_resource_low")
            .function("is_resource_low")
            .parameter(resource)
            .operator(ConditionOperator::Equal)
            .value(ConditionValue::Boolean(true))
    }

    /// Create a health and mana check chain
    pub fn health_and_mana_check(health_threshold: f64, mana_threshold: f64) -> ConditionResult<ConditionChainConfig> {
        let chain = ConditionChainBuilder::new()
            .id("health_and_mana_check")
            .logic(ChainLogic::And)
            .condition(Self::health_check(health_threshold).build()?)
            .condition(Self::mana_check(mana_threshold).build()?)
            .build()?;
        
        Ok(chain)
    }

    /// Create a resource check chain
    pub fn resource_check_chain(health_threshold: f64, mana_threshold: f64, stamina_threshold: f64) -> ConditionResult<ConditionChainConfig> {
        let chain = ConditionChainBuilder::new()
            .id("resource_check_chain")
            .logic(ChainLogic::And)
            .condition(Self::health_check(health_threshold).build()?)
            .condition(Self::mana_check(mana_threshold).build()?)
            .condition(Self::stamina_check(stamina_threshold).build()?)
            .build()?;
        
        Ok(chain)
    }

    /// Create an element mastery check chain
    pub fn element_mastery_chain(elements: &[(&str, f64)]) -> ConditionResult<ConditionChainConfig> {
        let mut builder = ConditionChainBuilder::new()
            .id("element_mastery_chain")
            .logic(ChainLogic::And);

        for (element, threshold) in elements {
            builder = builder.condition(
                Self::element_mastery_check(element, *threshold).build()?
            );
        }

        builder.build()
    }
}

/// Extension traits for easier condition building
pub trait ConditionBuilderExt {
    /// Create a condition builder with ID
    fn condition(id: impl Into<String>) -> ConditionBuilder;
    
    /// Create a condition chain builder with ID
    fn chain(id: impl Into<String>) -> ConditionChainBuilder;
}

impl ConditionBuilderExt for ConditionBuilder {
    fn condition(id: impl Into<String>) -> ConditionBuilder {
        ConditionBuilder::new().id(id)
    }
    
    fn chain(id: impl Into<String>) -> ConditionChainBuilder {
        ConditionChainBuilder::new().id(id)
    }
}

/// Helper functions for common condition patterns
pub mod helpers {
    use super::*;

    /// Create a simple resource check
    pub fn resource_check(resource: &str, threshold: f64) -> ConditionBuilder {
        ConditionBuilder::new()
            .id(format!("{}_check", resource))
            .function("get_actor_resource")
            .parameter(resource)
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(threshold))
    }

    /// Create a simple element mastery check
    pub fn element_check(element: &str, threshold: f64) -> ConditionBuilder {
        ConditionBuilder::new()
            .id(format!("{}_mastery_check", element))
            .function("get_element_mastery")
            .parameter(element)
            .operator(ConditionOperator::GreaterThan)
            .value(ConditionValue::Float(threshold))
    }

    /// Create a simple category check
    pub fn category_check(category: &str) -> ConditionBuilder {
        ConditionBuilder::new()
            .id(format!("has_{}_check", category))
            .function("has_category_item")
            .parameter(category)
            .operator(ConditionOperator::Equal)
            .value(ConditionValue::Boolean(true))
    }

    /// Create a simple AND chain
    pub fn and_chain(conditions: Vec<ConditionConfig>) -> ConditionResult<ConditionChainConfig> {
        ConditionChainBuilder::new()
            .id("and_chain")
            .logic(ChainLogic::And)
            .conditions(conditions)
            .build()
    }

    /// Create a simple OR chain
    pub fn or_chain(conditions: Vec<ConditionConfig>) -> ConditionResult<ConditionChainConfig> {
        ConditionChainBuilder::new()
            .id("or_chain")
            .logic(ChainLogic::Or)
            .conditions(conditions)
            .build()
    }
}

impl ConditionChainBuilder {
    /// Add multiple conditions at once
    pub fn conditions(mut self, conditions: Vec<ConditionConfig>) -> Self {
        self.conditions.extend(conditions);
        self
    }
}
