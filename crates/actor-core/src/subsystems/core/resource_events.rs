//! Resource Events System
//!
//! This module provides a comprehensive event system for the Enhanced Hybrid
//! Resource Manager, handling resource changes, notifications, and event
//! propagation across all resource systems.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// use crate::types::Actor;
use crate::ActorCoreResult;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Resource Event Manager
pub struct ResourceEventManager {
    /// Event listeners
    listeners: Arc<RwLock<HashMap<String, Vec<Arc<dyn ResourceEventListener + Send + Sync>>>>>,
    /// Event history
    event_history: Arc<RwLock<Vec<ResourceEvent>>>,
    /// Event filters
    event_filters: Arc<RwLock<HashMap<String, EventFilter>>>,
    /// Configuration
    #[allow(dead_code)]
    config: EventConfig,
}

/// Resource Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceEvent {
    /// Event ID
    pub event_id: String,
    /// Actor ID
    pub actor_id: String,
    /// Event type
    pub event_type: ResourceEventType,
    /// Resource name
    pub resource_name: String,
    /// Old value
    pub old_value: f64,
    /// New value
    pub new_value: f64,
    /// Event timestamp
    pub timestamp: u64,
    /// Event source
    pub source: String,
    /// Event metadata
    pub metadata: HashMap<String, String>,
    /// Event priority
    pub priority: EventPriority,
}

/// Resource Event Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceEventType {
    /// Resource value changed
    ResourceChanged,
    /// Resource regenerated
    ResourceRegenerated,
    /// Resource depleted
    ResourceDepleted,
    /// Resource fully restored
    ResourceFullyRestored,
    /// Resource maximum changed
    ResourceMaxChanged,
    /// Resource rate changed
    ResourceRateChanged,
    /// Resource category changed
    ResourceCategoryChanged,
    /// Resource system activated
    ResourceSystemActivated,
    /// Resource system deactivated
    ResourceSystemDeactivated,
    /// Resource calculation completed
    ResourceCalculationCompleted,
    /// Resource cache updated
    ResourceCacheUpdated,
    /// Resource database stored
    ResourceDatabaseStored,
    /// Resource database loaded
    ResourceDatabaseLoaded,
}

/// Event Priority
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// Event Filter
#[derive(Debug, Clone)]
pub struct EventFilter {
    /// Filter name
    pub name: String,
    /// Event types to filter
    pub event_types: Vec<ResourceEventType>,
    /// Actor IDs to filter
    pub actor_ids: Option<Vec<String>>,
    /// Resource names to filter
    pub resource_names: Option<Vec<String>>,
    /// Priority levels to filter
    pub priorities: Vec<EventPriority>,
    /// Minimum value change threshold
    pub min_value_change: Option<f64>,
    /// Maximum value change threshold
    pub max_value_change: Option<f64>,
}

/// Event Configuration
#[derive(Debug, Clone)]
pub struct EventConfig {
    /// Maximum history size
    pub max_history_size: usize,
    /// Enable event batching
    pub enable_batching: bool,
    /// Batch size
    pub batch_size: usize,
    /// Batch timeout in milliseconds
    pub batch_timeout_ms: u64,
    /// Enable event filtering
    pub enable_filtering: bool,
    /// Enable event persistence
    pub enable_persistence: bool,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self {
            max_history_size: 10000,
            enable_batching: true,
            batch_size: 100,
            batch_timeout_ms: 100,
            enable_filtering: true,
            enable_persistence: true,
            enable_monitoring: true,
        }
    }
}

/// Resource Event Listener
#[async_trait]
pub trait ResourceEventListener: Send + Sync {
    /// Handle a resource event
    async fn handle_event(&self, event: &ResourceEvent) -> ActorCoreResult<()>;
    
    /// Get listener identifier
    fn listener_id(&self) -> &str;
    
    /// Get interested event types
    fn interested_event_types(&self) -> Vec<ResourceEventType>;
    
    /// Get event priority
    fn event_priority(&self) -> EventPriority;
}

impl ResourceEventManager {
    /// Create a new Resource Event Manager
    pub fn new(config: EventConfig) -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
            event_history: Arc::new(RwLock::new(Vec::new())),
            event_filters: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    /// Add an event listener
    pub async fn add_listener(&self, listener: Arc<dyn ResourceEventListener + Send + Sync>) -> ActorCoreResult<()> {
        let _listener_id = listener.listener_id().to_string();
        let event_types = listener.interested_event_types();
        
        let mut listeners = self.listeners.write().await;
        
        for event_type in event_types {
            listeners.entry(format!("{:?}", event_type)).or_insert_with(Vec::new).push(listener.clone());
        }
        
        Ok(())
    }
    
    /// Remove an event listener
    pub async fn remove_listener(&self, listener_id: &str) -> ActorCoreResult<()> {
        let mut listeners = self.listeners.write().await;
        
        for (_, listener_list) in listeners.iter_mut() {
            listener_list.retain(|listener| listener.listener_id() != listener_id);
        }
        
        Ok(())
    }
    
    /// Emit a resource event
    pub async fn emit_event(&self, event: ResourceEvent) -> ActorCoreResult<()> {
        // Check if event passes filters
        if self.config.enable_filtering && !self.passes_filters(&event).await? {
            return Ok(());
        }
        
        // Add to history
        self.add_to_history(&event).await?;
        
        // Notify listeners
        self.notify_listeners(&event).await?;
        
        Ok(())
    }
    
    /// Create a resource changed event
    pub fn create_resource_changed_event(
        &self,
        actor_id: &str,
        resource_name: &str,
        old_value: f64,
        new_value: f64,
        source: &str,
    ) -> ResourceEvent {
        ResourceEvent {
            event_id: self.generate_event_id(),
            actor_id: actor_id.to_string(),
            event_type: ResourceEventType::ResourceChanged,
            resource_name: resource_name.to_string(),
            old_value,
            new_value,
            timestamp: self.get_current_timestamp(),
            source: source.to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::Normal,
        }
    }
    
    /// Create a resource regenerated event
    pub fn create_resource_regenerated_event(
        &self,
        actor_id: &str,
        resource_name: &str,
        amount: f64,
        new_value: f64,
        source: &str,
    ) -> ResourceEvent {
        ResourceEvent {
            event_id: self.generate_event_id(),
            actor_id: actor_id.to_string(),
            event_type: ResourceEventType::ResourceRegenerated,
            resource_name: resource_name.to_string(),
            old_value: new_value - amount,
            new_value,
            timestamp: self.get_current_timestamp(),
            source: source.to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::Low,
        }
    }
    
    /// Create a resource depleted event
    pub fn create_resource_depleted_event(
        &self,
        actor_id: &str,
        resource_name: &str,
        source: &str,
    ) -> ResourceEvent {
        ResourceEvent {
            event_id: self.generate_event_id(),
            actor_id: actor_id.to_string(),
            event_type: ResourceEventType::ResourceDepleted,
            resource_name: resource_name.to_string(),
            old_value: 0.0,
            new_value: 0.0,
            timestamp: self.get_current_timestamp(),
            source: source.to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::High,
        }
    }
    
    /// Create a resource fully restored event
    pub fn create_resource_fully_restored_event(
        &self,
        actor_id: &str,
        resource_name: &str,
        max_value: f64,
        source: &str,
    ) -> ResourceEvent {
        ResourceEvent {
            event_id: self.generate_event_id(),
            actor_id: actor_id.to_string(),
            event_type: ResourceEventType::ResourceFullyRestored,
            resource_name: resource_name.to_string(),
            old_value: 0.0,
            new_value: max_value,
            timestamp: self.get_current_timestamp(),
            source: source.to_string(),
            metadata: HashMap::new(),
            priority: EventPriority::Normal,
        }
    }
    
    /// Add an event filter
    pub async fn add_event_filter(&self, filter: EventFilter) -> ActorCoreResult<()> {
        let mut filters = self.event_filters.write().await;
        filters.insert(filter.name.clone(), filter);
        Ok(())
    }
    
    /// Remove an event filter
    pub async fn remove_event_filter(&self, filter_name: &str) -> ActorCoreResult<()> {
        let mut filters = self.event_filters.write().await;
        filters.remove(filter_name);
        Ok(())
    }
    
    /// Get event history
    pub async fn get_event_history(
        &self,
        actor_id: Option<&str>,
        event_type: Option<&ResourceEventType>,
        limit: Option<usize>,
    ) -> ActorCoreResult<Vec<ResourceEvent>> {
        let history = self.event_history.read().await;
        let mut events = history.clone();
        
        // Filter by actor ID
        if let Some(actor_id) = actor_id {
            events.retain(|event| event.actor_id == actor_id);
        }
        
        // Filter by event type
        if let Some(event_type) = event_type {
            events.retain(|event| event.event_type == *event_type);
        }
        
        // Sort by timestamp (newest first)
        events.sort_by_key(|event| std::cmp::Reverse(event.timestamp));
        
        // Apply limit
        if let Some(limit) = limit {
            events.truncate(limit);
        }
        
        Ok(events)
    }
    
    /// Get event statistics
    pub async fn get_event_stats(&self) -> ActorCoreResult<EventStats> {
        let history = self.event_history.read().await;
        
        let mut total_events = 0;
        let mut event_type_counts = HashMap::new();
        let mut actor_event_counts = HashMap::new();
        let mut resource_event_counts = HashMap::new();
        
        for event in history.iter() {
            total_events += 1;
            
            // Count by event type
            let event_type_key = format!("{:?}", event.event_type);
            *event_type_counts.entry(event_type_key).or_insert(0) += 1;
            
            // Count by actor
            *actor_event_counts.entry(event.actor_id.clone()).or_insert(0) += 1;
            
            // Count by resource
            *resource_event_counts.entry(event.resource_name.clone()).or_insert(0) += 1;
        }
        
        Ok(EventStats {
            total_events,
            event_type_counts,
            actor_event_counts,
            resource_event_counts,
        })
    }
    
    /// Clear event history
    pub async fn clear_history(&self) -> ActorCoreResult<()> {
        let mut history = self.event_history.write().await;
        history.clear();
        Ok(())
    }
    
    /// Check if event passes filters
    async fn passes_filters(&self, event: &ResourceEvent) -> ActorCoreResult<bool> {
        let filters = self.event_filters.read().await;
        
        for filter in filters.values() {
            // Check event type
            if !filter.event_types.is_empty() && !filter.event_types.contains(&event.event_type) {
                continue;
            }
            
            // Check actor ID
            if let Some(ref actor_ids) = filter.actor_ids {
                if !actor_ids.contains(&event.actor_id) {
                    continue;
                }
            }
            
            // Check resource name
            if let Some(ref resource_names) = filter.resource_names {
                if !resource_names.contains(&event.resource_name) {
                    continue;
                }
            }
            
            // Check priority
            if !filter.priorities.is_empty() && !filter.priorities.contains(&event.priority) {
                continue;
            }
            
            // Check value change thresholds
            let value_change = (event.new_value - event.old_value).abs();
            if let Some(min_change) = filter.min_value_change {
                if value_change < min_change {
                    continue;
                }
            }
            if let Some(max_change) = filter.max_value_change {
                if value_change > max_change {
                    continue;
                }
            }
            
            // Event passes this filter
            return Ok(true);
        }
        
        // No filters or event doesn't pass any filter
        Ok(filters.is_empty())
    }
    
    /// Add event to history
    async fn add_to_history(&self, event: &ResourceEvent) -> ActorCoreResult<()> {
        let mut history = self.event_history.write().await;
        history.push(event.clone());
        
        // Trim history if it exceeds max size
        if history.len() > self.config.max_history_size {
            let excess = history.len() - self.config.max_history_size;
            history.drain(0..excess);
        }
        
        Ok(())
    }
    
    /// Notify listeners
    async fn notify_listeners(&self, event: &ResourceEvent) -> ActorCoreResult<()> {
        let listeners = self.listeners.read().await;
        let event_type_key = format!("{:?}", event.event_type);
        
        if let Some(event_listeners) = listeners.get(&event_type_key) {
            for listener in event_listeners {
                if let Err(e) = listener.handle_event(event).await {
                    // Log error but continue with other listeners
                    eprintln!("Error notifying listener {}: {}", listener.listener_id(), e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Generate event ID
    fn generate_event_id(&self) -> String {
        use uuid::Uuid;
        Uuid::new_v4().to_string()
    }
    
    /// Get current timestamp
    fn get_current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Event Statistics
#[derive(Debug, Clone)]
pub struct EventStats {
    /// Total number of events
    pub total_events: usize,
    /// Event counts by type
    pub event_type_counts: HashMap<String, usize>,
    /// Event counts by actor
    pub actor_event_counts: HashMap<String, usize>,
    /// Event counts by resource
    pub resource_event_counts: HashMap<String, usize>,
}

/// Default Resource Event Listener
pub struct DefaultResourceEventListener {
    listener_id: String,
    interested_event_types: Vec<ResourceEventType>,
    event_priority: EventPriority,
}

impl DefaultResourceEventListener {
    pub fn new(
        listener_id: String,
        interested_event_types: Vec<ResourceEventType>,
        event_priority: EventPriority,
    ) -> Self {
        Self {
            listener_id,
            interested_event_types,
            event_priority,
        }
    }
}

#[async_trait]
impl ResourceEventListener for DefaultResourceEventListener {
    async fn handle_event(&self, event: &ResourceEvent) -> ActorCoreResult<()> {
        // Default implementation - just log the event
        println!("Default listener {} handled event: {:?}", self.listener_id, event.event_type);
        Ok(())
    }
    
    fn listener_id(&self) -> &str {
        &self.listener_id
    }
    
    fn interested_event_types(&self) -> Vec<ResourceEventType> {
        self.interested_event_types.clone()
    }
    
    fn event_priority(&self) -> EventPriority {
        self.event_priority.clone()
    }
}

/// Resource Change Listener
pub struct ResourceChangeListener {
    listener_id: String,
    #[allow(dead_code)]
    resource_cache: Arc<dyn crate::subsystems::enhanced_hybrid_resource_manager::ResourceCache + Send + Sync>,
}

impl ResourceChangeListener {
    pub fn new(
        listener_id: String,
        resource_cache: Arc<dyn crate::subsystems::enhanced_hybrid_resource_manager::ResourceCache + Send + Sync>,
    ) -> Self {
        Self {
            listener_id,
            resource_cache,
        }
    }
}

#[async_trait]
impl ResourceEventListener for ResourceChangeListener {
    async fn handle_event(&self, event: &ResourceEvent) -> ActorCoreResult<()> {
        match event.event_type {
            ResourceEventType::ResourceChanged => {
                // Update cache when resource changes
                // This would be implemented to update the resource cache
                println!("Resource {} changed for actor {}: {} -> {}", 
                    event.resource_name, event.actor_id, event.old_value, event.new_value);
            },
            ResourceEventType::ResourceDepleted => {
                // Handle resource depletion
                println!("Resource {} depleted for actor {}", event.resource_name, event.actor_id);
            },
            ResourceEventType::ResourceFullyRestored => {
                // Handle resource full restoration
                println!("Resource {} fully restored for actor {}", event.resource_name, event.actor_id);
            },
            _ => {
                // Handle other event types
                println!("Handling event type: {:?}", event.event_type);
            }
        }
        
        Ok(())
    }
    
    fn listener_id(&self) -> &str {
        &self.listener_id
    }
    
    fn interested_event_types(&self) -> Vec<ResourceEventType> {
        vec![
            ResourceEventType::ResourceChanged,
            ResourceEventType::ResourceDepleted,
            ResourceEventType::ResourceFullyRestored,
        ]
    }
    
    fn event_priority(&self) -> EventPriority {
        EventPriority::High
    }
}