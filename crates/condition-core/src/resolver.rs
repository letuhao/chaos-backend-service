//! Refactored condition resolver with data provider support

use super::types::*;
use super::error::*;
use super::data_provider::*;
use super::functions::*;

/// Main condition resolver with data provider support
pub struct ConditionResolver {
    function_registry: FunctionRegistry,
    data_registry: DataProviderRegistry,
}

impl ConditionResolver {
    /// Create a new condition resolver with data providers
    pub fn new(data_registry: DataProviderRegistry) -> Self {
        let function_registry = create_function_registry_with_providers(&data_registry);
        
        Self {
            function_registry,
            data_registry,
        }
    }

    /// Create a new condition resolver with custom function registry and data providers
    pub fn with_registry(
        function_registry: FunctionRegistry,
        data_registry: DataProviderRegistry,
    ) -> Self {
        Self {
            function_registry,
            data_registry,
        }
    }

    /// Get data provider registry
    pub fn get_data_registry(&self) -> &DataProviderRegistry {
        &self.data_registry
    }

    /// Get mutable data provider registry
    pub fn get_data_registry_mut(&mut self) -> &mut DataProviderRegistry {
        &mut self.data_registry
    }

    /// Evaluate a single condition
    async fn evaluate_single_condition(
        &self,
        condition_config: &ConditionConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool> {
        // Get the function
        let function = self.function_registry
            .get(&condition_config.function_name)
            .ok_or_else(|| ConditionError::FunctionNotFound {
                function_name: condition_config.function_name.clone(),
            })?;

        // Evaluate the function
        let result_value = function
            .evaluate(&condition_config.parameters, context)
            .await?;

        // Compare with expected value using operator
        self.compare_values(&result_value, &condition_config.value, &condition_config.operator)
    }

    /// Compare two values using the specified operator
    fn compare_values(
        &self,
        actual: &ConditionValue,
        expected: &ConditionValue,
        operator: &ConditionOperator,
    ) -> ConditionResult<bool> {
        match operator {
            ConditionOperator::Equal => Ok(actual == expected),
            ConditionOperator::NotEqual => Ok(actual != expected),
            ConditionOperator::GreaterThan => self.compare_numeric(actual, expected, |a, b| a > b),
            ConditionOperator::LessThan => self.compare_numeric(actual, expected, |a, b| a < b),
            ConditionOperator::GreaterThanOrEqual => self.compare_numeric(actual, expected, |a, b| a >= b),
            ConditionOperator::LessThanOrEqual => self.compare_numeric(actual, expected, |a, b| a <= b),
            ConditionOperator::Contains => self.check_contains(actual, expected),
            ConditionOperator::NotContains => self.check_contains(actual, expected).map(|b| !b),
            ConditionOperator::In => self.check_in(actual, expected),
            ConditionOperator::NotIn => self.check_in(actual, expected).map(|b| !b),
        }
    }

    /// Compare numeric values
    fn compare_numeric<F>(
        &self,
        actual: &ConditionValue,
        expected: &ConditionValue,
        compare_fn: F,
    ) -> ConditionResult<bool>
    where
        F: FnOnce(f64, f64) -> bool,
    {
        let actual_num = self.extract_number(actual)?;
        let expected_num = self.extract_number(expected)?;
        Ok(compare_fn(actual_num, expected_num))
    }

    /// Extract number from condition value
    fn extract_number(&self, value: &ConditionValue) -> ConditionResult<f64> {
        match value {
            ConditionValue::Integer(i) => Ok(*i as f64),
            ConditionValue::Float(f) => Ok(*f),
            _ => Err(ConditionError::ConfigError {
                message: format!("Expected numeric value, got: {:?}", value),
            }),
        }
    }

    /// Check if actual contains expected
    fn check_contains(&self, actual: &ConditionValue, expected: &ConditionValue) -> ConditionResult<bool> {
        match (actual, expected) {
            (ConditionValue::String(actual_str), ConditionValue::String(expected_str)) => {
                Ok(actual_str.contains(expected_str))
            }
            (ConditionValue::List(actual_list), expected) => {
                Ok(actual_list.contains(expected))
            }
            _ => Err(ConditionError::ConfigError {
                message: "Contains operator requires string or list values".to_string(),
            }),
        }
    }

    /// Check if actual is in expected list
    fn check_in(&self, actual: &ConditionValue, expected: &ConditionValue) -> ConditionResult<bool> {
        match expected {
            ConditionValue::List(expected_list) => {
                Ok(expected_list.contains(actual))
            }
            _ => Err(ConditionError::ConfigError {
                message: "In operator requires list value".to_string(),
            }),
        }
    }
}

#[async_trait::async_trait]
impl ConditionResolverTrait for ConditionResolver {
    async fn resolve_condition(
        &self,
        condition_config: &ConditionConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool> {
        self.evaluate_single_condition(condition_config, context).await
    }

    async fn resolve_conditions(
        &self,
        condition_configs: &[ConditionConfig],
        context: &ConditionContext,
    ) -> ConditionResult<Vec<bool>> {
        let mut results = Vec::new();
        
        for condition_config in condition_configs {
            let result = self.evaluate_single_condition(condition_config, context).await?;
            results.push(result);
        }
        
        Ok(results)
    }

    async fn resolve_condition_chain(
        &self,
        chain_config: &ConditionChainConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool> {
        if chain_config.conditions.is_empty() {
            return Err(ConditionError::ChainLogicError {
                message: "Empty condition chain".to_string(),
            });
        }

        // Evaluate all conditions in the chain
        let mut results = Vec::new();
        for condition in &chain_config.conditions {
            let result = self.evaluate_single_condition(condition, context).await?;
            results.push(result);
        }

        // Apply chain logic
        match chain_config.logic {
            ChainLogic::And => Ok(results.iter().all(|&b| b)),
            ChainLogic::Or => Ok(results.iter().any(|&b| b)),
            ChainLogic::Not => {
                if results.len() != 1 {
                    return Err(ConditionError::ChainLogicError {
                        message: "Not operator requires exactly one condition".to_string(),
                    });
                }
                Ok(!results[0])
            }
            ChainLogic::Xor => {
                let true_count = results.iter().filter(|&&b| b).count();
                Ok(true_count == 1)
            }
        }
    }
}
