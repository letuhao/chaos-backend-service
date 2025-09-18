//! Core types for Condition Core

use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use crate::error::ConditionResult;

/// Main trait for condition resolution
#[async_trait::async_trait]
pub trait ConditionResolverTrait {
    /// Resolve a single condition
    async fn resolve_condition(
        &self,
        condition_config: &ConditionConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool>;

    /// Resolve multiple conditions
    async fn resolve_conditions(
        &self,
        condition_configs: &[ConditionConfig],
        context: &ConditionContext,
    ) -> ConditionResult<Vec<bool>>;

    /// Resolve a condition chain with logical operators
    async fn resolve_condition_chain(
        &self,
        chain_config: &ConditionChainConfig,
        context: &ConditionContext,
    ) -> ConditionResult<bool>;
}

/// Configuration for a single condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionConfig {
    pub condition_id: String,
    pub function_name: String,
    pub operator: ConditionOperator,
    pub value: ConditionValue,
    pub parameters: Vec<ConditionParameter>,
}

/// Logical operators for condition evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    NotContains,
    In,
    NotIn,
}

/// Values that can be used in conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<ConditionValue>),
}

/// Parameters for condition functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionParameter {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl From<&str> for ConditionParameter {
    fn from(s: &str) -> Self {
        ConditionParameter::String(s.to_string())
    }
}

impl From<String> for ConditionParameter {
    fn from(s: String) -> Self {
        ConditionParameter::String(s)
    }
}

impl From<i64> for ConditionParameter {
    fn from(i: i64) -> Self {
        ConditionParameter::Integer(i)
    }
}

impl From<f64> for ConditionParameter {
    fn from(f: f64) -> Self {
        ConditionParameter::Float(f)
    }
}

impl From<bool> for ConditionParameter {
    fn from(b: bool) -> Self {
        ConditionParameter::Boolean(b)
    }
}

/// Configuration for a chain of conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionChainConfig {
    pub chain_id: String,
    pub logic: ChainLogic,
    pub conditions: Vec<ConditionConfig>,
}

/// Logical operators for condition chains
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ChainLogic {
    And,
    Or,
    Not,
    Xor,
}

/// Context information for condition evaluation
#[derive(Debug, Clone)]
pub struct ConditionContext {
    pub target: ActorTarget,
    pub world_id: String,
    pub current_time: SystemTime,
    pub current_weather: WeatherType,
    pub world_state: WorldState,
}

/// Target actor information
#[derive(Debug, Clone)]
pub struct ActorTarget {
    pub id: String,
}

/// Weather types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherType {
    Clear,
    Rain,
    Snow,
    Storm,
    Fog,
}

/// World state information
#[derive(Debug, Clone)]
pub struct WorldState {
    pub time_of_day: f64,
    pub season: String,
    pub temperature: f64,
    pub humidity: f64,
}

/// Trait for condition functions
#[async_trait::async_trait]
pub trait ConditionFunction: Send + Sync {
    /// Get the function name
    fn name(&self) -> &str;

    /// Evaluate the function with given parameters and context
    async fn evaluate(
        &self,
        parameters: &[ConditionParameter],
        context: &ConditionContext,
    ) -> ConditionResult<ConditionValue>;
}

/// Registry for condition functions
pub struct FunctionRegistry {
    functions: std::collections::HashMap<String, Box<dyn ConditionFunction>>,
}

impl FunctionRegistry {
    /// Create a new function registry
    pub fn new() -> Self {
        Self {
            functions: std::collections::HashMap::new(),
        }
    }

    /// Register a function
    pub fn register(&mut self, function: Box<dyn ConditionFunction>) {
        self.functions.insert(function.name().to_string(), function);
    }

    /// Get a function by name
    pub fn get(&self, name: &str) -> Option<&dyn ConditionFunction> {
        self.functions.get(name).map(|f| f.as_ref())
    }

    /// List all registered functions
    pub fn list(&self) -> Vec<&str> {
        self.functions.keys().map(|k| k.as_str()).collect()
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}