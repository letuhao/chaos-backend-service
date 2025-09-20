//! Actor Manager
//!
//! This module manages actors in the game world with high-performance operations
//! using shared memory and lock-free programming patterns.

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, warn, debug, error};

use actor_core::prelude::*;

use crate::server::SharedGameState;

/// High-performance actor manager
pub struct ActorManager {
    /// Shared game state
    shared_state: Arc<SharedGameState>,
    
    /// Actor cache for fast lookups
    actor_cache: Arc<RwLock<HashMap<String, Actor>>>,
    
    /// Actor statistics
    stats: Arc<RwLock<ActorStats>>,
}

/// Actor statistics
#[derive(Debug, Clone, Default)]
pub struct ActorStats {
    /// Total actors created
    pub total_created: u64,
    
    /// Total actors destroyed
    pub total_destroyed: u64,
    
    /// Current active actors
    pub active_count: u32,
    
    /// Actors per second (creation rate)
    pub creation_rate: f64,
    
    /// Last update timestamp
    pub last_update: u64,
}

impl ActorManager {
    /// Create a new actor manager
    pub fn new(shared_state: Arc<SharedGameState>) -> Self {
        Self {
            shared_state,
            actor_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ActorStats::default())),
        }
    }
    
    /// Create a new actor
    pub async fn create_actor(&self, name: String, race: String) -> ActorCoreResult<Actor> {
        debug!("Creating new actor: {} ({})", name, race);
        
        // Create actor using Actor Core
        let mut actor = Actor::new(name, race);
        
        // Apply default configurations
        if let Err(e) = self.apply_actor_configurations(&mut actor).await {
            warn!("Failed to apply configurations to actor {}: {}", actor.id, e);
        }
        
        // Add to shared state
        self.shared_state.add_actor(actor.clone());
        
        // Add to cache
        self.actor_cache.write().insert(actor.id.clone(), actor.clone());
        
        // Update statistics
        self.update_stats(|stats| {
            stats.total_created += 1;
            stats.active_count += 1;
        });
        
        info!("Actor created successfully: {} ({})", actor.id, actor.name);
        Ok(actor)
    }
    
    /// Get actor by ID
    pub fn get_actor(&self, actor_id: &str) -> Option<Actor> {
        // Try cache first for performance
        if let Some(actor) = self.actor_cache.read().get(actor_id).cloned() {
            return Some(actor);
        }
        
        // Fallback to shared state
        self.shared_state.get_actor(actor_id)
    }
    
    /// Update actor
    pub async fn update_actor(&self, mut actor: Actor) -> ActorCoreResult<()> {
        debug!("Updating actor: {}", actor.id);
        
        // Apply configurations
        self.apply_actor_configurations(&mut actor).await?;
        
        // Update in shared state
        self.shared_state.add_actor(actor.clone());
        
        // Update cache
        self.actor_cache.write().insert(actor.id.clone(), actor);
        
        debug!("Actor updated successfully: {}", actor.id);
        Ok(())
    }
    
    /// Remove actor
    pub fn remove_actor(&self, actor_id: &str) -> Option<Actor> {
        debug!("Removing actor: {}", actor_id);
        
        // Remove from shared state
        let actor = self.shared_state.remove_actor(actor_id);
        
        // Remove from cache
        self.actor_cache.write().remove(actor_id);
        
        // Update statistics
        if actor.is_some() {
            self.update_stats(|stats| {
                stats.total_destroyed += 1;
                stats.active_count = stats.active_count.saturating_sub(1);
            });
        }
        
        debug!("Actor removed: {}", actor_id);
        actor
    }
    
    /// Get all actors
    pub fn get_all_actors(&self) -> Vec<Actor> {
        self.actor_cache.read().values().cloned().collect()
    }
    
    /// Get actors by race
    pub fn get_actors_by_race(&self, race: &str) -> Vec<Actor> {
        self.actor_cache
            .read()
            .values()
            .filter(|actor| actor.race == race)
            .cloned()
            .collect()
    }
    
    /// Get actors by level range
    pub fn get_actors_by_level(&self, min_level: u32, max_level: u32) -> Vec<Actor> {
        self.actor_cache
            .read()
            .values()
            .filter(|actor| {
                if let Some(level) = actor.data.get("level") {
                    if let Some(level_num) = level.as_u64() {
                        let level = level_num as u32;
                        level >= min_level && level <= max_level
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }
    
    /// Apply actor configurations using Actor Core
    async fn apply_actor_configurations(&self, actor: &mut Actor) -> ActorCoreResult<()> {
        debug!("Applying configurations to actor: {}", actor.id);
        
        // Get configuration manager
        let config_manager = self.shared_state.actor_core.get_config_manager();
        
        // Apply default actor configurations
        if let Ok(Some(health_config)) = config_manager.get_config_value("defaults", "default_actor_health").await {
            if let Some(health_value) = health_config.value.as_u64() {
                actor.data.insert("health".to_string(), serde_json::Value::Number(health_value.into()));
            }
        }
        
        if let Ok(Some(level_config)) = config_manager.get_config_value("defaults", "default_actor_level").await {
            if let Some(level_value) = level_config.value.as_u64() {
                actor.data.insert("level".to_string(), serde_json::Value::Number(level_value.into()));
            }
        }
        
        if let Ok(Some(experience_config)) = config_manager.get_config_value("defaults", "default_actor_experience").await {
            if let Some(exp_value) = experience_config.value.as_u64() {
                actor.data.insert("experience".to_string(), serde_json::Value::Number(exp_value.into()));
            }
        }
        
        debug!("Configurations applied to actor: {}", actor.id);
        Ok(())
    }
    
    /// Update actor statistics
    fn update_stats<F>(&self, f: F)
    where
        F: FnOnce(&mut ActorStats),
    {
        let mut stats = self.stats.write();
        f(&mut stats);
        stats.last_update = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }
    
    /// Get actor statistics
    pub fn get_stats(&self) -> ActorStats {
        self.stats.read().clone()
    }
    
    /// Get active actor count
    pub fn get_active_count(&self) -> u32 {
        self.actor_cache.read().len() as u32
    }
    
    /// Clear all actors (for testing)
    pub fn clear_all_actors(&self) {
        debug!("Clearing all actors");
        
        // Clear shared state
        self.shared_state.actors.write().clear();
        
        // Clear cache
        self.actor_cache.write().clear();
        
        // Reset statistics
        self.update_stats(|stats| {
            stats.active_count = 0;
        });
        
        debug!("All actors cleared");
    }
    
    /// Refresh actor cache from shared state
    pub fn refresh_cache(&self) {
        debug!("Refreshing actor cache");
        
        let shared_actors = self.shared_state.actors.read();
        let mut cache = self.actor_cache.write();
        
        cache.clear();
        for (id, actor) in shared_actors.iter() {
            cache.insert(id.clone(), actor.clone());
        }
        
        debug!("Actor cache refreshed with {} actors", cache.len());
    }
}
