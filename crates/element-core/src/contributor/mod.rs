//! # Element Contributor Module
//! 
//! This module provides the ElementContributor trait and related structures for external system integration,
//! enabling other systems to contribute elemental data to the element-core system without direct dependencies.
//! 
//! ## Architecture
//! 
//! The contributor module implements the external contributor pattern where:
//! - **External Systems** implement the `ElementContributor` trait
//! - **Contributions** are registered with the element-core system
//! - **Data Aggregation** combines contributions from multiple sources
//! - **Loose Coupling** maintains system independence
//! 
//! ## Components
//! 
//! ### `ElementContributor`
//! - **Trait Definition**: Interface for external system contributions
//! - **Element Contributions**: Methods for contributing elemental data
//! - **Metadata**: System identification and capability information
//! - **Validation**: Contribution validation and error handling
//! 
//! ### `ElementContribution`
//! - **Data Structure**: Represents a single elemental contribution
//! - **Source Tracking**: Identifies the contributing system
//! - **Value Management**: Stores the contributed elemental values
//! - **Timestamp**: Tracks when the contribution was made
//! 
//! ### `ElementContributorRegistry`
//! - **Registry Management**: Tracks all registered contributors
//! - **Contribution Aggregation**: Combines contributions from multiple sources
//! - **Health Monitoring**: Tracks contributor system health
//! - **Performance Metrics**: Monitors contribution performance
//! 
//! ### `ContributorMetadata`
//! - **System Information**: Contributor system identification
//! - **Capabilities**: What elemental data the system can contribute
//! - **Version Information**: System version and compatibility
//! - **Health Status**: Current system health and availability
//! 
//! ## Usage Patterns
//! 
//! ### Implementing a Contributor
//! ```rust
//! impl ElementContributor for MySystem {
//!     fn contribute_elements(&self) -> ElementCoreResult<Vec<ElementContribution>> {
//!         // Return elemental contributions from this system
//!     }
//!     
//!     fn get_metadata(&self) -> ContributorMetadata {
//!         // Return system metadata
//!     }
//! }
//! ```
//! 
//! ### Registering a Contributor
//! ```rust
//! let contributor = MySystem::new();
//! let metadata = contributor.get_metadata();
//! registry.register_contributor(contributor, metadata)?;
//! ```
//! 
//! ### Aggregating Contributions
//! ```rust
//! let contributions = registry.aggregate_contributions()?;
//! let total_stats = registry.calculate_total_stats(&contributions)?;
//! ```
//! 
//! ## Integration Examples
//! 
//! - **Race-Core**: Contributes racial elemental affinities
//! - **Item-Core**: Contributes item-based elemental bonuses
//! - **Skill-Core**: Contributes skill-based elemental effects
//! - **Status-Core**: Contributes status effect elemental modifiers

pub mod element_contributor;
pub mod element_contribution;
pub mod contributor_registry;

pub use element_contributor::*;
pub use element_contribution::*;
pub use contributor_registry::*;
