//! Condition Core Integration Module
//! 
//! This module provides integration between Actor Core and Condition Core,
//! including data providers, conditional subsystems, and conditional modifiers.

pub mod data_providers;
pub mod conditional_subsystems;
pub mod conditional_modifiers;
pub mod integration;

// Re-export specific types to avoid conflicts
pub use data_providers::{
    ActorDataProvider, ResourceDataProvider, CategoryDataProvider,
    ActorRepository, StatCache, ResourceManager, ActorInventory, CategoryRegistry,
    Resource, Item, Actor, Snapshot
};
pub use conditional_subsystems::{
    ConditionalSubsystem, SubsystemOutput, Contribution, CapContribution, 
    SubsystemMeta, CapMode, CapKind, Bucket, ActorCoreResult, ActorCoreError
};
pub use conditional_modifiers::{
    ConditionalModifierSystem, ConditionalModifier, ModifierRegistry,
    ModifierActor, ModifierSnapshot, ModifierType
};
pub use integration::{
    ActorCoreWithConditions, ConditionCache, ConditionEventHandler,
    Aggregator, PluginRegistry
};
