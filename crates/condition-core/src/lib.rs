//! Condition Core - Condition Logic Resolver for Chaos World
//! 
//! This crate provides a simple and efficient condition resolution system
//! that can be used by other systems to evaluate conditions based on
//! configuration files.

pub mod error;
pub mod types;
pub mod resolver;
pub mod functions;
pub mod config;
pub mod data_provider;
pub mod data_accessor;
pub mod element_functions;
pub mod status_functions;
pub mod builder;

pub use error::*;
pub use types::*;
pub use resolver::*;
pub use functions::*;
pub use config::*;
pub use data_provider::*;
pub use data_accessor::*;

/// Re-export commonly used types for convenience
pub use types::{
    ConditionConfig,
    ConditionContext,
    ConditionResolverTrait,
    ConditionOperator,
    ConditionValue,
    ConditionParameter,
    ConditionChainConfig,
    ChainLogic,
    ActorTarget,
    WeatherType,
    WorldState,
};

/// Re-export builder types for convenience
pub use builder::{
    ConditionBuilder,
    ConditionChainBuilder,
    ConditionBuilderFactory,
    ConditionBuilderExt,
    helpers,
};
