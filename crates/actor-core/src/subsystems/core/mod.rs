//! Core Subsystems
//!
//! This module contains core subsystems that provide fundamental functionality
//! for the Actor Core system, including event management and notifications.

pub mod stat_change_notifier;
pub mod resource_events;

// Re-export commonly used core subsystems
pub use stat_change_notifier::{StatChangeNotifier, StatChangeEvent, StatChangeListener};
pub use resource_events::{ResourceEventManager, ResourceEvent, ResourceEventType, EventConfig, EventStats};
