//! Enums for the Actor Core system.
//!
//! This module contains all the enumeration types used throughout the system,
//! including buckets, cap modes, operators, and other categorical values.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Bucket represents the type of stat contribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Bucket {
    /// Flat additive contribution
    Flat,
    /// Multiplicative contribution
    Mult,
    /// Post-additive contribution (applied after multiplication)
    PostAdd,
    /// Override contribution (replaces previous values)
    Override,
    /// Exponential contribution
    Exponential,
    /// Logarithmic contribution
    Logarithmic,
    /// Conditional contribution (applied based on conditions)
    Conditional,
}

impl Bucket {
    /// Get the priority of this bucket type.
    /// Higher numbers are processed later in the aggregation pipeline.
    pub fn priority(&self) -> u8 {
        match self {
            Bucket::Flat => 1,
            Bucket::Mult => 2,
            Bucket::PostAdd => 3,
            Bucket::Override => 4,
            Bucket::Exponential => 5,
            Bucket::Logarithmic => 6,
            Bucket::Conditional => 7,
        }
    }

    /// Check if this bucket type is valid.
    pub fn is_valid(&self) -> bool {
        true // All variants are valid
    }

    /// Get the display name of this bucket.
    pub fn display_name(&self) -> &'static str {
        match self {
            Bucket::Flat => "Flat",
            Bucket::Mult => "Multiplicative",
            Bucket::PostAdd => "Post-Additive",
            Bucket::Override => "Override",
            Bucket::Exponential => "Exponential",
            Bucket::Logarithmic => "Logarithmic",
            Bucket::Conditional => "Conditional",
        }
    }
}

impl fmt::Display for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// CapMode represents how a cap contribution should be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapMode {
    /// Baseline cap (starting point)
    Baseline,
    /// Additive cap (added to existing caps)
    Additive,
    /// Hard maximum cap (cannot be exceeded)
    HardMax,
    /// Hard minimum cap (cannot go below)
    HardMin,
    /// Override cap (replaces existing caps)
    Override,
}

impl CapMode {
    /// Get the priority of this cap mode.
    /// Higher numbers are processed later.
    pub fn priority(&self) -> u8 {
        match self {
            CapMode::Baseline => 1,
            CapMode::Additive => 2,
            CapMode::HardMax => 3,
            CapMode::HardMin => 4,
            CapMode::Override => 5,
        }
    }

    /// Check if this cap mode is valid.
    pub fn is_valid(&self) -> bool {
        true // All variants are valid
    }

    /// Get the display name of this cap mode.
    pub fn display_name(&self) -> &'static str {
        match self {
            CapMode::Baseline => "Baseline",
            CapMode::Additive => "Additive",
            CapMode::HardMax => "Hard Maximum",
            CapMode::HardMin => "Hard Minimum",
            CapMode::Override => "Override",
        }
    }
}

impl fmt::Display for CapMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Operator represents how values should be combined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Operator {
    /// Sum all values
    Sum,
    /// Take the maximum value
    Max,
    /// Take the minimum value
    Min,
    /// Calculate the average
    Average,
    /// Multiply all values
    Multiply,
    /// Intersect values (for caps)
    Intersect,
}

impl Operator {
    /// Apply the operator to a list of values.
    pub fn apply(&self, values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }

        match self {
            Operator::Sum => values.iter().sum(),
            Operator::Max => values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            Operator::Min => values.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
            Operator::Average => values.iter().sum::<f64>() / values.len() as f64,
            Operator::Multiply => values.iter().product(),
            Operator::Intersect => {
                // For intersection, we want the most restrictive range
                // This is a simplified implementation
                values.iter().fold(f64::INFINITY, |a, &b| a.min(b))
            }
        }
    }

    /// Check if this operator is valid.
    pub fn is_valid(&self) -> bool {
        true // All variants are valid
    }

    /// Get the display name of this operator.
    pub fn display_name(&self) -> &'static str {
        match self {
            Operator::Sum => "Sum",
            Operator::Max => "Maximum",
            Operator::Min => "Minimum",
            Operator::Average => "Average",
            Operator::Multiply => "Multiply",
            Operator::Intersect => "Intersect",
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Layer represents the scope of a cap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Layer {
    /// Realm-scoped cap
    Realm,
    /// World-scoped cap
    World,
    /// Event-scoped cap
    Event,
    /// Guild-scoped cap
    Guild,
    /// Total cap (applies to all)
    Total,
}

impl Layer {
    /// Get the priority of this layer.
    /// Higher numbers are processed later.
    pub fn priority(&self) -> u8 {
        match self {
            Layer::Realm => 1,
            Layer::World => 2,
            Layer::Event => 3,
            Layer::Guild => 4,
            Layer::Total => 5,
        }
    }

    /// Check if this layer is valid.
    pub fn is_valid(&self) -> bool {
        true // All variants are valid
    }

    /// Get the display name of this layer.
    pub fn display_name(&self) -> &'static str {
        match self {
            Layer::Realm => "Realm",
            Layer::World => "World",
            Layer::Event => "Event",
            Layer::Guild => "Guild",
            Layer::Total => "Total",
        }
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Priority represents the priority level of an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Priority {
    /// Lowest priority
    Low = 0,
    /// Normal priority
    Normal = 1,
    /// High priority
    High = 2,
    /// Critical priority
    Critical = 3,
}

impl Priority {
    /// Get the numeric value of this priority.
    pub fn value(&self) -> u8 {
        *self as u8
    }

    /// Check if this priority is valid.
    pub fn is_valid(&self) -> bool {
        true // All variants are valid
    }

    /// Get the display name of this priority.
    pub fn display_name(&self) -> &'static str {
        match self {
            Priority::Low => "Low",
            Priority::Normal => "Normal",
            Priority::High => "High",
            Priority::Critical => "Critical",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// ClampMode represents how values should be clamped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClampMode {
    /// Clamp to both min and max
    Both,
    /// Clamp only to minimum
    MinOnly,
    /// Clamp only to maximum
    MaxOnly,
    /// No clamping
    None,
}

impl ClampMode {
    /// Check if this clamp mode is valid.
    pub fn is_valid(&self) -> bool {
        true // All variants are valid
    }

    /// Get the display name of this clamp mode.
    pub fn display_name(&self) -> &'static str {
        match self {
            ClampMode::Both => "Both",
            ClampMode::MinOnly => "Minimum Only",
            ClampMode::MaxOnly => "Maximum Only",
            ClampMode::None => "None",
        }
    }
}

impl fmt::Display for ClampMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
