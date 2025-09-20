//! Shared Game State
//!
//! This module defines the shared memory state that all game systems access.
//! Uses lock-free programming and atomic operations for maximum performance.

use std::sync::Arc;
use std::collections::HashMap;
use parking_lot::RwLock;
use tokio::sync::RwLock as AsyncRwLock;

use actor_core::prelude::*;
use actor_core::builder::{ActorCoreBuilder, ActorCoreSystem};

use crate::server::ActorManager;

/// Shared game state accessible by all systems
/// Uses Arc for shared ownership and RwLock for safe concurrent access
#[derive(Debug)]
pub struct SharedGameState {
    /// Actor Core system for configuration management
    pub actor_core: Arc<ActorCoreSystem>,
    
    /// Active actors in the game world
    pub actors: Arc<RwLock<HashMap<String, Actor>>>,
    
    /// Actor manager
    pub actor_manager: ActorManager,
    
    /// Game world state
    pub world_state: Arc<RwLock<WorldState>>,
    
    /// Performance metrics
    pub metrics: Arc<RwLock<PerformanceMetrics>>,
    
    /// Server configuration
    pub server_config: Arc<RwLock<ServerConfig>>,
}

/// World state containing game world information
#[derive(Debug, Clone)]
pub struct WorldState {
    /// Current game time
    pub game_time: u64,
    
    /// World zones
    pub zones: HashMap<String, Zone>,
    
    /// Active events
    pub events: Vec<GameEvent>,
    
    /// World settings
    pub settings: WorldSettings,
}

/// Game zone information
#[derive(Debug, Clone)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub capacity: u32,
    pub current_players: u32,
    pub status: ZoneStatus,
}

/// Zone status
#[derive(Debug, Clone, PartialEq)]
pub enum ZoneStatus {
    Online,
    Maintenance,
    Offline,
    Full,
}

/// Game event
#[derive(Debug, Clone)]
pub struct GameEvent {
    pub id: String,
    pub event_type: EventType,
    pub timestamp: u64,
    pub data: serde_json::Value,
}

/// Event types
#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    PlayerJoin,
    PlayerLeave,
    Combat,
    LevelUp,
    ItemDrop,
    Custom(String),
}

/// World settings
#[derive(Debug, Clone)]
pub struct WorldSettings {
    pub max_players: u32,
    pub tick_rate: u32,
    pub combat_enabled: bool,
    pub pvp_enabled: bool,
    pub experience_rate: f64,
}

/// Performance metrics
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// Frames per second
    pub fps: f64,
    
    /// Average frame time in milliseconds
    pub avg_frame_time_ms: f64,
    
    /// Memory usage in MB
    pub memory_usage_mb: f64,
    
    /// Active actors count
    pub active_actors: u32,
    
    /// Network packets per second
    pub packets_per_second: u32,
    
    /// Database queries per second
    pub db_queries_per_second: u32,
    
    /// Last update timestamp
    pub last_update: u64,
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Server name
    pub name: String,
    
    /// Server port
    pub port: u16,
    
    /// Max connections
    pub max_connections: u32,
    
    /// Tick rate (updates per second)
    pub tick_rate: u32,
    
    /// Enable MongoDB config sync
    pub enable_mongodb_sync: bool,
    
    /// MongoDB connection string
    pub mongodb_connection: String,
    
    /// Config sync interval in seconds
    pub config_sync_interval: u64,
}

impl SharedGameState {
    /// Create a new shared game state
    pub async fn new(server_config: ServerConfig) -> ActorCoreResult<Self> {
        // Initialize Actor Core with MongoDB config if enabled
        let mut builder = ActorCoreBuilder::new()
            .with_metrics(true)
            .with_caching(true)
            .with_log_level("info".to_string());
        
        if server_config.enable_mongodb_sync {
            builder = builder.with_mongodb_config(true);
        }
        
        let actor_core = Arc::new(builder.build().await?);
        
        Ok(Self {
            actor_core,
            actors: Arc::new(RwLock::new(HashMap::new())),
            world_state: Arc::new(RwLock::new(WorldState::default())),
            metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
            server_config: Arc::new(RwLock::new(server_config)),
        })
    }
    
    /// Get actor by ID
    pub fn get_actor(&self, actor_id: &str) -> Option<Actor> {
        self.actors.read().get(actor_id).cloned()
    }
    
    /// Add actor to the world
    pub fn add_actor(&self, actor: Actor) {
        self.actors.write().insert(actor.id.to_string(), actor);
    }
    
    /// Remove actor from the world
    pub fn remove_actor(&self, actor_id: &str) -> Option<Actor> {
        self.actors.write().remove(actor_id)
    }
    
    /// Update performance metrics
    pub fn update_metrics(&self, metrics: PerformanceMetrics) {
        *self.metrics.write() = metrics;
    }
    
    /// Get current performance metrics
    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.read().clone()
    }
    
    /// Update world state
    pub fn update_world_state<F>(&self, f: F) 
    where
        F: FnOnce(&mut WorldState),
    {
        f(&mut self.world_state.write());
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            game_time: 0,
            zones: HashMap::new(),
            events: Vec::new(),
            settings: WorldSettings {
                max_players: 1000,
                tick_rate: 60,
                combat_enabled: true,
                pvp_enabled: false,
                experience_rate: 1.0,
            },
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "Chaos Game Server".to_string(),
            port: 8080,
            max_connections: 1000,
            tick_rate: 60,
            enable_mongodb_sync: true,
            mongodb_connection: "mongodb://localhost:27017".to_string(),
            config_sync_interval: 300,
        }
    }
}
