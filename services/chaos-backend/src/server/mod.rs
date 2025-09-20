//! Game Server Runtime
//!
//! This module provides the main game server runtime that integrates all game systems
//! in a single process with shared memory architecture for optimal performance.

pub mod game_server;
pub mod config_sync;
pub mod actor_manager;
pub mod handlers;
pub mod shared_state;
pub mod performance_monitor;

// Re-export main types for convenience
pub use game_server::GameServer;
pub use config_sync::ConfigSyncService;
pub use actor_manager::ActorManager;
pub use shared_state::SharedGameState;
