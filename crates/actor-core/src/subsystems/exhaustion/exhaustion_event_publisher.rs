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
    /// Last emit times for coalescing (keyed by idempotency key)
    last_emit: Arc<RwLock<HashMap<String, u64>>>,
    /// Coalescing window in milliseconds
    coalesce_window_ms: u64,
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
            last_emit: Arc::new(RwLock::new(HashMap::new())),
            coalesce_window_ms: 100, // TODO: Load default coalesce window from configuration
        }
    }

    /// Create a new in-memory event publisher with custom coalescing window
    pub fn with_coalesce_window(coalesce_window_ms: u64) -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(EventStats::default())),
            last_emit: Arc::new(RwLock::new(HashMap::new())),
            coalesce_window_ms,
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
    async fn publish_event(&self, mut event: ExhaustionEvent) -> ActorCoreResult<()> {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        let idempotency_key = &event.idempotency_key;
        
        // Check for coalescing
        let should_coalesce = {
            let last_emit = self.last_emit.read().await;
            if let Some(last_time) = last_emit.get(idempotency_key) {
                now - last_time < self.coalesce_window_ms
            } else {
                false
            }
        };

        if should_coalesce {
            // Mark as coalesced and update last emit time
            event.coalesced = true;
            {
                let mut last_emit = self.last_emit.write().await;
                last_emit.insert(idempotency_key.clone(), now);
            }
        } else {
            // Update last emit time for new event
            {
                let mut last_emit = self.last_emit.write().await;
                last_emit.insert(idempotency_key.clone(), now);
            }
        }

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

        if !event.coalesced {
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
        } else {
            info!(
                "Coalesced exhaustion event: {} for actor {} resource {} threshold {}",
                match event.event_type {
                    super::resource_exhaustion::ExhaustionEventType::ResourceExhausted => "ResourceExhausted",
                    super::resource_exhaustion::ExhaustionEventType::ResourceRecovered => "ResourceRecovered",
                },
                event.actor_id,
                event.resource_type,
                event.threshold_id
            );
        }

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