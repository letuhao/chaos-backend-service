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
    #[cfg(feature = "extra_buckets")]
    Exponential,
    /// Logarithmic contribution
    #[cfg(feature = "extra_buckets")]
    Logarithmic,
    /// Conditional contribution (applied based on conditions)
    #[cfg(feature = "extra_buckets")]
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
            #[cfg(feature = "extra_buckets")]
            Bucket::Exponential => 5,
            #[cfg(feature = "extra_buckets")]
            Bucket::Logarithmic => 6,
            #[cfg(feature = "extra_buckets")]
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
            #[cfg(feature = "extra_buckets")]
            Bucket::Exponential => "Exponential",
            #[cfg(feature = "extra_buckets")]
            Bucket::Logarithmic => "Logarithmic",
            #[cfg(feature = "extra_buckets")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_creation_and_properties() {
        let bucket = Bucket::Flat;
        assert_eq!(bucket.priority(), 1);
        assert!(bucket.is_valid());
        assert_eq!(bucket.display_name(), "Flat");
        assert_eq!(bucket.to_string(), "Flat");

        let bucket = Bucket::Mult;
        assert_eq!(bucket.priority(), 2);
        assert!(bucket.is_valid());
        assert_eq!(bucket.display_name(), "Multiplicative");
        assert_eq!(bucket.to_string(), "Multiplicative");

        let bucket = Bucket::PostAdd;
        assert_eq!(bucket.priority(), 3);
        assert!(bucket.is_valid());
        assert_eq!(bucket.display_name(), "Post-Additive");
        assert_eq!(bucket.to_string(), "Post-Additive");

        let bucket = Bucket::Override;
        assert_eq!(bucket.priority(), 4);
        assert!(bucket.is_valid());
        assert_eq!(bucket.display_name(), "Override");
        assert_eq!(bucket.to_string(), "Override");
    }

    #[test]
    fn test_bucket_equality() {
        let bucket1 = Bucket::Flat;
        let bucket2 = Bucket::Flat;
        let bucket3 = Bucket::Mult;

        assert_eq!(bucket1, bucket2);
        assert_ne!(bucket1, bucket3);
    }

    #[test]
    fn test_bucket_serialization() {
        let bucket = Bucket::Flat;
        let serialized = serde_json::to_string(&bucket).unwrap();
        let deserialized: Bucket = serde_json::from_str(&serialized).unwrap();
        assert_eq!(bucket, deserialized);
    }

    #[test]
    fn test_cap_mode_creation_and_properties() {
        let cap_mode = CapMode::Baseline;
        assert_eq!(cap_mode.priority(), 1);
        assert!(cap_mode.is_valid());
        assert_eq!(cap_mode.display_name(), "Baseline");
        assert_eq!(cap_mode.to_string(), "Baseline");

        let cap_mode = CapMode::Additive;
        assert_eq!(cap_mode.priority(), 2);
        assert!(cap_mode.is_valid());
        assert_eq!(cap_mode.display_name(), "Additive");
        assert_eq!(cap_mode.to_string(), "Additive");

        let cap_mode = CapMode::HardMax;
        assert_eq!(cap_mode.priority(), 3);
        assert!(cap_mode.is_valid());
        assert_eq!(cap_mode.display_name(), "Hard Maximum");
        assert_eq!(cap_mode.to_string(), "Hard Maximum");

        let cap_mode = CapMode::HardMin;
        assert_eq!(cap_mode.priority(), 4);
        assert!(cap_mode.is_valid());
        assert_eq!(cap_mode.display_name(), "Hard Minimum");
        assert_eq!(cap_mode.to_string(), "Hard Minimum");

        let cap_mode = CapMode::Override;
        assert_eq!(cap_mode.priority(), 5);
        assert!(cap_mode.is_valid());
        assert_eq!(cap_mode.display_name(), "Override");
        assert_eq!(cap_mode.to_string(), "Override");
    }

    #[test]
    fn test_cap_mode_equality() {
        let cap_mode1 = CapMode::Baseline;
        let cap_mode2 = CapMode::Baseline;
        let cap_mode3 = CapMode::Additive;

        assert_eq!(cap_mode1, cap_mode2);
        assert_ne!(cap_mode1, cap_mode3);
    }

    #[test]
    fn test_cap_mode_serialization() {
        let cap_mode = CapMode::Baseline;
        let serialized = serde_json::to_string(&cap_mode).unwrap();
        let deserialized: CapMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cap_mode, deserialized);
    }

    #[test]
    fn test_operator_creation_and_properties() {
        let operator = Operator::Sum;
        assert!(operator.is_valid());
        assert_eq!(operator.display_name(), "Sum");
        assert_eq!(operator.to_string(), "Sum");

        let operator = Operator::Max;
        assert!(operator.is_valid());
        assert_eq!(operator.display_name(), "Maximum");
        assert_eq!(operator.to_string(), "Maximum");

        let operator = Operator::Min;
        assert!(operator.is_valid());
        assert_eq!(operator.display_name(), "Minimum");
        assert_eq!(operator.to_string(), "Minimum");

        let operator = Operator::Average;
        assert!(operator.is_valid());
        assert_eq!(operator.display_name(), "Average");
        assert_eq!(operator.to_string(), "Average");

        let operator = Operator::Multiply;
        assert!(operator.is_valid());
        assert_eq!(operator.display_name(), "Multiply");
        assert_eq!(operator.to_string(), "Multiply");

        let operator = Operator::Intersect;
        assert!(operator.is_valid());
        assert_eq!(operator.display_name(), "Intersect");
        assert_eq!(operator.to_string(), "Intersect");
    }

    #[test]
    fn test_operator_apply() {
        let values = vec![1.0, 2.0, 3.0, 4.0];

        let operator = Operator::Sum;
        assert_eq!(operator.apply(&values), 10.0);

        let operator = Operator::Max;
        assert_eq!(operator.apply(&values), 4.0);

        let operator = Operator::Min;
        assert_eq!(operator.apply(&values), 1.0);

        let operator = Operator::Average;
        assert_eq!(operator.apply(&values), 2.5);

        let operator = Operator::Multiply;
        assert_eq!(operator.apply(&values), 24.0);

        let operator = Operator::Intersect;
        assert_eq!(operator.apply(&values), 1.0);
    }

    #[test]
    fn test_operator_apply_empty() {
        let values = vec![];
        let operator = Operator::Sum;
        assert_eq!(operator.apply(&values), 0.0);
    }

    #[test]
    fn test_operator_equality() {
        let operator1 = Operator::Sum;
        let operator2 = Operator::Sum;
        let operator3 = Operator::Max;

        assert_eq!(operator1, operator2);
        assert_ne!(operator1, operator3);
    }

    #[test]
    fn test_operator_serialization() {
        let operator = Operator::Sum;
        let serialized = serde_json::to_string(&operator).unwrap();
        let deserialized: Operator = serde_json::from_str(&serialized).unwrap();
        assert_eq!(operator, deserialized);
    }

    #[test]
    fn test_layer_creation_and_properties() {
        let layer = Layer::Realm;
        assert_eq!(layer.priority(), 1);
        assert!(layer.is_valid());
        assert_eq!(layer.display_name(), "Realm");
        assert_eq!(layer.to_string(), "Realm");

        let layer = Layer::World;
        assert_eq!(layer.priority(), 2);
        assert!(layer.is_valid());
        assert_eq!(layer.display_name(), "World");
        assert_eq!(layer.to_string(), "World");

        let layer = Layer::Event;
        assert_eq!(layer.priority(), 3);
        assert!(layer.is_valid());
        assert_eq!(layer.display_name(), "Event");
        assert_eq!(layer.to_string(), "Event");

        let layer = Layer::Guild;
        assert_eq!(layer.priority(), 4);
        assert!(layer.is_valid());
        assert_eq!(layer.display_name(), "Guild");
        assert_eq!(layer.to_string(), "Guild");

        let layer = Layer::Total;
        assert_eq!(layer.priority(), 5);
        assert!(layer.is_valid());
        assert_eq!(layer.display_name(), "Total");
        assert_eq!(layer.to_string(), "Total");
    }

    #[test]
    fn test_layer_equality() {
        let layer1 = Layer::Realm;
        let layer2 = Layer::Realm;
        let layer3 = Layer::World;

        assert_eq!(layer1, layer2);
        assert_ne!(layer1, layer3);
    }

    #[test]
    fn test_layer_serialization() {
        let layer = Layer::Realm;
        let serialized = serde_json::to_string(&layer).unwrap();
        let deserialized: Layer = serde_json::from_str(&serialized).unwrap();
        assert_eq!(layer, deserialized);
    }

    #[test]
    fn test_priority_creation_and_properties() {
        let priority = Priority::Low;
        assert_eq!(priority.value(), 0);
        assert!(priority.is_valid());
        assert_eq!(priority.display_name(), "Low");
        assert_eq!(priority.to_string(), "Low");

        let priority = Priority::Normal;
        assert_eq!(priority.value(), 1);
        assert!(priority.is_valid());
        assert_eq!(priority.display_name(), "Normal");
        assert_eq!(priority.to_string(), "Normal");

        let priority = Priority::High;
        assert_eq!(priority.value(), 2);
        assert!(priority.is_valid());
        assert_eq!(priority.display_name(), "High");
        assert_eq!(priority.to_string(), "High");

        let priority = Priority::Critical;
        assert_eq!(priority.value(), 3);
        assert!(priority.is_valid());
        assert_eq!(priority.display_name(), "Critical");
        assert_eq!(priority.to_string(), "Critical");
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Low < Priority::Normal);
        assert!(Priority::Normal < Priority::High);
        assert!(Priority::High < Priority::Critical);
        assert!(Priority::Low <= Priority::Low);
        assert!(Priority::Critical >= Priority::High);
    }

    #[test]
    fn test_priority_equality() {
        let priority1 = Priority::Normal;
        let priority2 = Priority::Normal;
        let priority3 = Priority::High;

        assert_eq!(priority1, priority2);
        assert_ne!(priority1, priority3);
    }

    #[test]
    fn test_priority_serialization() {
        let priority = Priority::Normal;
        let serialized = serde_json::to_string(&priority).unwrap();
        let deserialized: Priority = serde_json::from_str(&serialized).unwrap();
        assert_eq!(priority, deserialized);
    }

    #[test]
    fn test_clamp_mode_creation_and_properties() {
        let clamp_mode = ClampMode::Both;
        assert!(clamp_mode.is_valid());
        assert_eq!(clamp_mode.display_name(), "Both");
        assert_eq!(clamp_mode.to_string(), "Both");

        let clamp_mode = ClampMode::MinOnly;
        assert!(clamp_mode.is_valid());
        assert_eq!(clamp_mode.display_name(), "Minimum Only");
        assert_eq!(clamp_mode.to_string(), "Minimum Only");

        let clamp_mode = ClampMode::MaxOnly;
        assert!(clamp_mode.is_valid());
        assert_eq!(clamp_mode.display_name(), "Maximum Only");
        assert_eq!(clamp_mode.to_string(), "Maximum Only");

        let clamp_mode = ClampMode::None;
        assert!(clamp_mode.is_valid());
        assert_eq!(clamp_mode.display_name(), "None");
        assert_eq!(clamp_mode.to_string(), "None");
    }

    #[test]
    fn test_clamp_mode_equality() {
        let clamp_mode1 = ClampMode::Both;
        let clamp_mode2 = ClampMode::Both;
        let clamp_mode3 = ClampMode::MinOnly;

        assert_eq!(clamp_mode1, clamp_mode2);
        assert_ne!(clamp_mode1, clamp_mode3);
    }

    #[test]
    fn test_clamp_mode_serialization() {
        let clamp_mode = ClampMode::Both;
        let serialized = serde_json::to_string(&clamp_mode).unwrap();
        let deserialized: ClampMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(clamp_mode, deserialized);
    }

    #[test]
    fn test_all_enums_debug() {
        let bucket = Bucket::Flat;
        let cap_mode = CapMode::Baseline;
        let operator = Operator::Sum;
        let layer = Layer::Realm;
        let priority = Priority::Normal;
        let clamp_mode = ClampMode::Both;

        assert!(format!("{:?}", bucket).contains("Flat"));
        assert!(format!("{:?}", cap_mode).contains("Baseline"));
        assert!(format!("{:?}", operator).contains("Sum"));
        assert!(format!("{:?}", layer).contains("Realm"));
        assert!(format!("{:?}", priority).contains("Normal"));
        assert!(format!("{:?}", clamp_mode).contains("Both"));
    }

    #[test]
    fn test_all_enums_clone() {
        let bucket = Bucket::Flat;
        let cap_mode = CapMode::Baseline;
        let operator = Operator::Sum;
        let layer = Layer::Realm;
        let priority = Priority::Normal;
        let clamp_mode = ClampMode::Both;

        let bucket_clone = bucket.clone();
        let cap_mode_clone = cap_mode.clone();
        let operator_clone = operator.clone();
        let layer_clone = layer.clone();
        let priority_clone = priority.clone();
        let clamp_mode_clone = clamp_mode.clone();

        assert_eq!(bucket, bucket_clone);
        assert_eq!(cap_mode, cap_mode_clone);
        assert_eq!(operator, operator_clone);
        assert_eq!(layer, layer_clone);
        assert_eq!(priority, priority_clone);
        assert_eq!(clamp_mode, clamp_mode_clone);
    }

    #[test]
    fn test_all_enums_hash() {
        use std::collections::HashMap;

        // Test each enum type separately
        let mut bucket_map = HashMap::new();
        bucket_map.insert(Bucket::Flat, "flat_value");
        bucket_map.insert(Bucket::Mult, "mult_value");
        assert_eq!(bucket_map.get(&Bucket::Flat), Some(&"flat_value"));
        assert_eq!(bucket_map.get(&Bucket::Mult), Some(&"mult_value"));

        let mut cap_mode_map = HashMap::new();
        cap_mode_map.insert(CapMode::Baseline, "baseline_value");
        cap_mode_map.insert(CapMode::Additive, "additive_value");
        assert_eq!(cap_mode_map.get(&CapMode::Baseline), Some(&"baseline_value"));
        assert_eq!(cap_mode_map.get(&CapMode::Additive), Some(&"additive_value"));

        let mut operator_map = HashMap::new();
        operator_map.insert(Operator::Sum, "sum_value");
        operator_map.insert(Operator::Max, "max_value");
        assert_eq!(operator_map.get(&Operator::Sum), Some(&"sum_value"));
        assert_eq!(operator_map.get(&Operator::Max), Some(&"max_value"));

        let mut layer_map = HashMap::new();
        layer_map.insert(Layer::Realm, "realm_value");
        layer_map.insert(Layer::World, "world_value");
        assert_eq!(layer_map.get(&Layer::Realm), Some(&"realm_value"));
        assert_eq!(layer_map.get(&Layer::World), Some(&"world_value"));

        let mut priority_map = HashMap::new();
        priority_map.insert(Priority::Normal, "normal_value");
        priority_map.insert(Priority::High, "high_value");
        assert_eq!(priority_map.get(&Priority::Normal), Some(&"normal_value"));
        assert_eq!(priority_map.get(&Priority::High), Some(&"high_value"));

        let mut clamp_mode_map = HashMap::new();
        clamp_mode_map.insert(ClampMode::Both, "both_value");
        clamp_mode_map.insert(ClampMode::MinOnly, "min_only_value");
        assert_eq!(clamp_mode_map.get(&ClampMode::Both), Some(&"both_value"));
        assert_eq!(clamp_mode_map.get(&ClampMode::MinOnly), Some(&"min_only_value"));
    }
}