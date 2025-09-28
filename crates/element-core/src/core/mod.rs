//! # Core Module
//! 
//! This module contains the core elemental system components that form the foundation
//! of the elemental system. It provides the primary data structures, configuration
//! management, and system logic.
//! 
//! ## Components
//! 
//! ### `elemental_data`
//! - **ElementalSystemData**: Main data structure with primary and derived stats
//! - **ElementMasteryRank**: 7-tier mastery ranking system
//! - **Array-based Performance**: Fixed-size arrays for O(1) access
//! - **Comprehensive Validation**: All data integrity checks
//! 
//! ### `elemental_config`
//! - **ElementConfig**: YAML configuration loading
//! - **ElementDefinition**: Element property definitions
//! - **BaseProperties**: Core element statistics
//! - **StatusEffectConfig**: Status effect definitions
//! 
//! ### `elemental_system`
//! - **ElementalSystem**: Main system wrapper
//! - **ElementRegistry**: Element configuration registry
//! - **System Integration**: Thread-safe access patterns
//! 
//! ## Key Features
//! 
//! - **Primary/Derived Stats Separation**: Clear distinction between stored and calculated values
//! - **Performance Optimization**: Array-based structures for game loop efficiency
//! - **Thread Safety**: All operations are safe for concurrent access
//! - **Comprehensive Validation**: Data integrity checking at all levels
//! - **Extensible Design**: Easy to add new stats and properties

pub mod elemental_data;
pub mod elemental_config;
pub mod elemental_system;

pub use elemental_data::*;
pub use elemental_config::*;
pub use elemental_system::*;
