//! Exhaustion Event Publisher
//!
//! This module provides implementations for publishing exhaustion events.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::ActorCoreResult;
use super::resource_exhaustion::{ExhaustionEvent, ExhaustionEventPublisher};

/// Simple in-memory event publisher for testing
pub struct InMemoryEventPublisher {
    /// Published events
    events: Arc<RwLock<Vec<ExhaustionEvent>>>,
    /// Event statistics
    stats: Arc<RwLock<EventStats>>,
}

/// Event statistics
#[derive(Debug, Default, Clone)]
pub struct EventStats {
    /// Total events published
    pub total_events: u64,
    /// Events by type
    pub events_by_type: HashMap<String, u64>,
    /// Coalesced events
    pub coalesced_events: u64,
    /// Failed events
    pub failed_events: u64,
}

impl InMemoryEventPublisher {
    /// Create a new in-memory event publisher
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(EventStats::default())),
        }
    }

    /// Get all published events
    pub async fn get_events(&self) -> Vec<ExhaustionEvent> {
        self.events.read().await.clone()
    }

    /// Get event statistics
    pub async fn get_stats(&self) -> EventStats {
        self.stats.read().await.clone()
    }

    /// Clear all events
    pub async fn clear_events(&self) {
        self.events.write().await.clear();
        *self.stats.write().await = EventStats::default();
    }
}

#[async_trait]
impl ExhaustionEventPublisher for InMemoryEventPublisher {
    async fn publish_event(&self, event: ExhaustionEvent) -> ActorCoreResult<()> {
        // Store event
        {
            let mut events = self.events.write().await;
            events.push(event.clone());
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_events += 1;
            
            let event_type = match event.event_type {
                super::resource_exhaustion::ExhaustionEventType::ResourceExhausted => "ResourceExhausted",
                super::resource_exhaustion::ExhaustionEventType::ResourceRecovered => "ResourceRecovered",
            };
            
            *stats.events_by_type.entry(event_type.to_string()).or_insert(0) += 1;
            
            if event.coalesced {
                stats.coalesced_events += 1;
            }
        }

        info!(
            "Published exhaustion event: {} for actor {} resource {} threshold {}",
            match event.event_type {
                super::resource_exhaustion::ExhaustionEventType::ResourceExhausted => "ResourceExhausted",
                super::resource_exhaustion::ExhaustionEventType::ResourceRecovered => "ResourceRecovered",
            },
            event.actor_id,
            event.resource_type,
            event.threshold_id
        );

        Ok(())
    }
}

/// Logging event publisher that logs events to console
pub struct LoggingEventPublisher;

#[async_trait]
impl ExhaustionEventPublisher for LoggingEventPublisher {
    async fn publish_event(&self, event: ExhaustionEvent) -> ActorCoreResult<()> {
        info!(
            "Exhaustion Event: {} - Actor: {} Resource: {} Threshold: {} Effects: {}",
            match event.event_type {
                super::resource_exhaustion::ExhaustionEventType::ResourceExhausted => "EXHAUSTED",
                super::resource_exhaustion::ExhaustionEventType::ResourceRecovered => "RECOVERED",
            },
            event.actor_id,
            event.resource_type,
            event.threshold_id,
            event.effects.len()
        );

        for effect in &event.effects {
            info!(
                "  Effect: {} - Values: {:?} Categories: {:?} Modifier: {:?}",
                effect.effect_type,
                effect.values,
                effect.categories,
                effect.modifier
            );
        }

        Ok(())
    }
}

/// No-op event publisher for testing
pub struct NoOpEventPublisher;

#[async_trait]
impl ExhaustionEventPublisher for NoOpEventPublisher {
    async fn publish_event(&self, _event: ExhaustionEvent) -> ActorCoreResult<()> {
        // Do nothing
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::resource_exhaustion::{ExhaustionEvent, ExhaustionEventType, EffectConfig};
    use chrono::Utc;

    #[tokio::test]
    async fn test_in_memory_event_publisher() {
        let publisher = InMemoryEventPublisher::new();
        
        let event = ExhaustionEvent {
            event_type: ExhaustionEventType::ResourceExhausted,
            actor_id: "test_actor".to_string(),
            resource_type: "mana".to_string(),
            threshold_id: "low_mana".to_string(),
            effects: vec![EffectConfig {
                effect_type: "disable_tags".to_string(),
                values: Some(vec!["shield_activation".to_string()]),
                categories: None,
                modifier: None,
                name: None,
                value: None,
                level: None,
                resource: None,
            }],
            timestamp: Utc::now(),
            idempotency_key: "test_key".to_string(),
            coalesced: false,
        };

        // Publish event
        publisher.publish_event(event.clone()).await.unwrap();

        // Check events
        let events = publisher.get_events().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].actor_id, "test_actor");
        assert_eq!(events[0].resource_type, "mana");

        // Check stats
        let stats = publisher.get_stats().await;
        assert_eq!(stats.total_events, 1);
        assert_eq!(stats.events_by_type.get("ResourceExhausted"), Some(&1));
    }

    #[tokio::test]
    async fn test_logging_event_publisher() {
        let publisher = LoggingEventPublisher;
        
        let event = ExhaustionEvent {
            event_type: ExhaustionEventType::ResourceRecovered,
            actor_id: "test_actor".to_string(),
            resource_type: "stamina".to_string(),
            threshold_id: "low_stamina".to_string(),
            effects: vec![],
            timestamp: Utc::now(),
            idempotency_key: "test_key".to_string(),
            coalesced: false,
        };

        // Should not panic
        publisher.publish_event(event).await.unwrap();
    }

    #[tokio::test]
    async fn test_no_op_event_publisher() {
        let publisher = NoOpEventPublisher;
        
        let event = ExhaustionEvent {
            event_type: ExhaustionEventType::ResourceExhausted,
            actor_id: "test_actor".to_string(),
            resource_type: "mana".to_string(),
            threshold_id: "low_mana".to_string(),
            effects: vec![],
            timestamp: Utc::now(),
            idempotency_key: "test_key".to_string(),
            coalesced: false,
        };

        // Should not panic
        publisher.publish_event(event).await.unwrap();
    }
}
