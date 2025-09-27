//! Enums and related types for the Actor Core system.
//!
//! This module defines various enums used throughout the system
//! for configuration, policies, and operational modes.

use serde::{Deserialize, Serialize};
use tracing;

/// AcrossLayerPolicy defines how caps are combined across layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AcrossLayerPolicy {
    /// Intersect caps (most restrictive)
    Intersect,
    /// Union caps (least restrictive)
    Union,
    /// Prioritized override (later layers override earlier ones)
    PrioritizedOverride,
}

/// Operator defines how values are combined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    /// Sum values
    Sum,
    /// Take maximum value
    Max,
    /// Take minimum value
    Min,
    /// Multiply values
    Multiply,
    /// Average values
    Average,
    /// Intersect values (take minimum)
    Intersect,
}

/// Bucket defines how contributions are processed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Bucket {
    /// Flat contributions (additive)
    Flat,
    /// Multiplicative contributions
    Mult,
    /// Post-addition contributions
    PostAdd,
    /// Override contributions (replaces value)
    Override,
    /// Exponential contributions (extra feature)
    #[cfg(feature = "extra_buckets")]
    Exponential,
    /// Logarithmic contributions (extra feature)
    #[cfg(feature = "extra_buckets")]
    Logarithmic,
    /// Conditional contributions (extra feature)
    #[cfg(feature = "extra_buckets")]
    Conditional,
}

impl Bucket {
    /// Get the priority of this bucket (lower = processed earlier)
    pub fn priority(&self) -> u8 {
        // Load priority from configuration or use hardcoded defaults
        Self::load_bucket_priority(self).unwrap_or_else(|| {
            tracing::warn!("Failed to load bucket priority from config, using hardcoded defaults");
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
        })
    }

    /// Load bucket priority from configuration
    fn load_bucket_priority(&self) -> Option<u8> {
        // TODO: Load from configs/bucket_priorities.yaml
        // For now, return None to use hardcoded defaults
        None
    }

    /// Check if the bucket is valid
    pub fn is_valid(&self) -> bool {
        true // All buckets are valid
    }

    /// Get the display name of the bucket
    pub fn display_name(&self) -> &'static str {
        // Load display name from configuration or use hardcoded defaults
        Self::load_bucket_display_name(self).unwrap_or_else(|| {
            tracing::warn!("Failed to load bucket display name from config, using hardcoded defaults");
            match self {
                Bucket::Flat => "Flat",
                Bucket::Mult => "Mult",
                Bucket::PostAdd => "PostAdd",
                Bucket::Override => "Override",
                #[cfg(feature = "extra_buckets")]
                Bucket::Exponential => "Exponential",
                #[cfg(feature = "extra_buckets")]
                Bucket::Logarithmic => "Logarithmic",
                #[cfg(feature = "extra_buckets")]
                Bucket::Conditional => "Conditional",
            }
        })
    }

    /// Load bucket display name from configuration
    fn load_bucket_display_name(&self) -> Option<&'static str> {
        // TODO: Load from configs/bucket_display_names.yaml
        // For now, return None to use hardcoded defaults
        None
    }
}

/// CapMode defines how cap contributions are applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapMode {
    /// Baseline cap (sets both min and max)
    Baseline,
    /// Additive cap (expands range)
    Additive,
    /// Hard maximum cap
    HardMax,
    /// Hard minimum cap
    HardMin,
    /// Override cap (replaces existing caps)
    Override,
    /// Soft maximum cap (can be exceeded)
    SoftMax,
}

impl CapMode {
    /// Check if the cap mode is valid
    pub fn is_valid(&self) -> bool {
        true // All cap modes are valid
    }

    /// Get the display name of the cap mode
    pub fn display_name(&self) -> &'static str {
        // Load display name from configuration or use hardcoded defaults
        Self::load_cap_mode_display_name(self).unwrap_or_else(|| {
            tracing::warn!("Failed to load cap mode display name from config, using hardcoded defaults");
            match self {
                CapMode::Baseline => "Baseline",
                CapMode::Additive => "Additive",
                CapMode::HardMax => "HardMax",
                CapMode::HardMin => "HardMin",
                CapMode::Override => "Override",
                CapMode::SoftMax => "SoftMax",
            }
        })
    }

    /// Load cap mode display name from configuration
    fn load_cap_mode_display_name(&self) -> Option<&'static str> {
        // TODO: Load from configs/cap_mode_display_names.yaml
        // For now, return None to use hardcoded defaults
        None
    }
}