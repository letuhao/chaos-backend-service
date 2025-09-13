//! Stat Change Notification System
//!
//! This module provides a sophisticated stat change notification system for the
//! Enhanced Hybrid Resource Manager to enable smart recalculation.

use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::types::Actor;
use crate::ActorCoreResult;
use serde::{Deserialize, Serialize};

/// Stat Change Notifier for smart resource recalculation
pub struct StatChangeNotifier {
    /// Stat change listeners
    listeners: Arc<RwLock<HashMap<String, Vec<Arc<dyn StatChangeListener + Send + Sync>>>>>,
    /// Resource dependencies mapping
    resource_dependencies: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    /// Stat change history
    change_history: Arc<RwLock<Vec<StatChangeEvent>>>,
    /// Configuration
    config: NotifierConfig,
}

/// Stat change listener trait
#[async_trait]
pub trait StatChangeListener: Send + Sync {
    /// Handle stat change event
    async fn on_stat_change(&self, event: &StatChangeEvent) -> ActorCoreResult<()>;
    
    /// Get listener identifier
    fn listener_id(&self) -> &str;
    
    /// Get interested stat types
    fn interested_stats(&self) -> Vec<String>;
}

/// Stat change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatChangeEvent {
    /// Actor ID
    pub actor_id: String,
    /// Changed stat names
    pub changed_stats: Vec<String>,
    /// Event timestamp
    pub timestamp: u64,
    /// Event source
    pub source: String,
    /// Event metadata
    pub metadata: HashMap<String, String>,
}

/// Notifier configuration
#[derive(Debug, Clone)]
pub struct NotifierConfig {
    /// Maximum history size
    pub max_history_size: usize,
    /// Enable change batching
    pub enable_batching: bool,
    /// Batch timeout in milliseconds
    pub batch_timeout_ms: u64,
    /// Enable dependency tracking
    pub enable_dependency_tracking: bool,
}

impl Default for NotifierConfig {
    fn default() -> Self {
        Self {
            max_history_size: 10000,
            enable_batching: true,
            batch_timeout_ms: 100,
            enable_dependency_tracking: true,
        }
    }
}

impl StatChangeNotifier {
    /// Create a new stat change notifier
    pub fn new(config: NotifierConfig) -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
            resource_dependencies: Arc::new(RwLock::new(HashMap::new())),
            change_history: Arc::new(RwLock::new(Vec::new())),
            config,
        }
    }
    
    /// Add a stat change listener
    pub async fn add_listener(&self, listener: Arc<dyn StatChangeListener + Send + Sync>) -> ActorCoreResult<()> {
        let _listener_id = listener.listener_id().to_string();
        let interested_stats = listener.interested_stats();
        
        let mut listeners = self.listeners.write().await;
        
        for stat in interested_stats {
            listeners.entry(stat).or_insert_with(Vec::new).push(listener.clone());
        }
        
        Ok(())
    }
    
    /// Remove a stat change listener
    pub async fn remove_listener(&self, listener_id: &str) -> ActorCoreResult<()> {
        let mut listeners = self.listeners.write().await;
        
        for (_, listener_list) in listeners.iter_mut() {
            listener_list.retain(|listener| listener.listener_id() != listener_id);
        }
        
        Ok(())
    }
    
    /// Notify of stat changes
    pub async fn notify_stat_change(&self, actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()> {
        if changed_stats.is_empty() {
            return Ok(());
        }
        
        let event = StatChangeEvent {
            actor_id: actor.id.to_string(),
            changed_stats: changed_stats.to_vec(),
            timestamp: chrono::Utc::now().timestamp() as u64,
            source: "actor_core".to_string(),
            metadata: HashMap::new(),
        };
        
        // Add to history
        self.add_to_history(&event).await?;
        
        // Notify listeners
        self.notify_listeners(&event).await?;
        
        // Update resource dependencies if enabled
        if self.config.enable_dependency_tracking {
            self.update_resource_dependencies(&event).await?;
        }
        
        Ok(())
    }
    
    /// Check if resources need recalculation
    pub async fn needs_recalculation(&self, actor: &Actor, resource_id: &str) -> ActorCoreResult<bool> {
        if !self.config.enable_dependency_tracking {
            return Ok(true);
        }
        
        let dependencies = {
            let deps = self.resource_dependencies.read().await;
            deps.get(resource_id).cloned().unwrap_or_default()
        };
        
        if dependencies.is_empty() {
            return Ok(false);
        }
        
        // Check if any dependent stats have changed recently
        let recent_changes = self.get_recent_changes(&actor.id.to_string(), 60).await?; // Last 60 seconds
        
        for change in recent_changes {
            for changed_stat in &change.changed_stats {
                if dependencies.contains(changed_stat) {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// Get resources affected by stat changes
    pub async fn get_affected_resources(&self, changed_stats: &[String]) -> ActorCoreResult<Vec<String>> {
        let mut affected_resources = HashSet::new();
        
        let dependencies = self.resource_dependencies.read().await;
        
        for stat in changed_stats {
            for (resource_id, resource_deps) in dependencies.iter() {
                if resource_deps.contains(stat) {
                    affected_resources.insert(resource_id.clone());
                }
            }
        }
        
        Ok(affected_resources.into_iter().collect())
    }
    
    /// Add resource dependency
    pub async fn add_resource_dependency(&self, resource_id: String, stat_name: String) -> ActorCoreResult<()> {
        let mut dependencies = self.resource_dependencies.write().await;
        dependencies.entry(resource_id).or_insert_with(HashSet::new).insert(stat_name);
        Ok(())
    }
    
    /// Remove resource dependency
    pub async fn remove_resource_dependency(&self, resource_id: &str, stat_name: &str) -> ActorCoreResult<()> {
        let mut dependencies = self.resource_dependencies.write().await;
        if let Some(deps) = dependencies.get_mut(resource_id) {
            deps.remove(stat_name);
        }
        Ok(())
    }
    
    /// Get stat change history
    pub async fn get_history(&self, actor_id: Option<&str>, limit: Option<usize>) -> ActorCoreResult<Vec<StatChangeEvent>> {
        let history = self.change_history.read().await;
        let mut events = history.clone();
        
        // Filter by actor if specified
        if let Some(actor_id) = actor_id {
            events.retain(|event| event.actor_id == actor_id);
        }
        
        // Sort by timestamp (newest first)
        events.sort_by_key(|event| std::cmp::Reverse(event.timestamp));
        
        // Apply limit
        if let Some(limit) = limit {
            events.truncate(limit);
        }
        
        Ok(events)
    }
    
    /// Clear history
    pub async fn clear_history(&self) -> ActorCoreResult<()> {
        let mut history = self.change_history.write().await;
        history.clear();
        Ok(())
    }
    
    /// Add event to history
    async fn add_to_history(&self, event: &StatChangeEvent) -> ActorCoreResult<()> {
        let mut history = self.change_history.write().await;
        history.push(event.clone());
        
        // Trim history if it exceeds max size
        if history.len() > self.config.max_history_size {
            let excess = history.len() - self.config.max_history_size;
            history.drain(0..excess);
        }
        
        Ok(())
    }
    
    /// Notify all relevant listeners
    async fn notify_listeners(&self, event: &StatChangeEvent) -> ActorCoreResult<()> {
        let listeners = self.listeners.read().await;
        
        for changed_stat in &event.changed_stats {
            if let Some(stat_listeners) = listeners.get(changed_stat) {
                for listener in stat_listeners {
                    if let Err(e) = listener.on_stat_change(event).await {
                        // Log error but continue with other listeners
                        eprintln!("Error notifying listener {}: {}", listener.listener_id(), e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Update resource dependencies based on stat changes
    async fn update_resource_dependencies(&self, _event: &StatChangeEvent) -> ActorCoreResult<()> {
        // This would analyze the stat changes and update resource dependencies
        // For now, it's a placeholder
        Ok(())
    }
    
    /// Get recent changes for an actor
    async fn get_recent_changes(&self, actor_id: &str, seconds: u64) -> ActorCoreResult<Vec<StatChangeEvent>> {
        let history = self.change_history.read().await;
        let cutoff_time = chrono::Utc::now().timestamp() as u64 - seconds;
        
        let recent_changes: Vec<_> = history
            .iter()
            .filter(|event| event.actor_id == actor_id && event.timestamp > cutoff_time)
            .cloned()
            .collect();
        
        Ok(recent_changes)
    }
}

/// Default stat change listener implementation
pub struct DefaultStatChangeListener {
    listener_id: String,
    interested_stats: Vec<String>,
}

impl DefaultStatChangeListener {
    pub fn new(listener_id: String, interested_stats: Vec<String>) -> Self {
        Self {
            listener_id,
            interested_stats,
        }
    }
}

#[async_trait]
impl StatChangeListener for DefaultStatChangeListener {
    async fn on_stat_change(&self, _event: &StatChangeEvent) -> ActorCoreResult<()> {
        // Default implementation - no-op
        Ok(())
    }
    
    fn listener_id(&self) -> &str {
        &self.listener_id
    }
    
    fn interested_stats(&self) -> Vec<String> {
        self.interested_stats.clone()
    }
}

/// Resource recalculation listener
pub struct ResourceRecalculationListener {
    listener_id: String,
    resource_cache: Arc<dyn crate::subsystems::enhanced_hybrid_resource_manager::ResourceCache + Send + Sync>,
}

impl ResourceRecalculationListener {
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
impl StatChangeListener for ResourceRecalculationListener {
    async fn on_stat_change(&self, event: &StatChangeEvent) -> ActorCoreResult<()> {
        // Invalidate cache for the actor when stats change
        self.resource_cache.invalidate_actor(&event.actor_id).await?;
        Ok(())
    }
    
    fn listener_id(&self) -> &str {
        &self.listener_id
    }
    
    fn interested_stats(&self) -> Vec<String> {
        vec![
            "lifespan".to_string(),
            "age".to_string(),
            "cultivation_level".to_string(),
            "level".to_string(),
            "intelligence".to_string(),
            "vitality".to_string(),
        ]
    }
}