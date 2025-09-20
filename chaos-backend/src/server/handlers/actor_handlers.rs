//! Actor Request Handlers
//!
//! This module handles HTTP requests related to actor management.

use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, debug, error};

use actor_core::prelude::*;

use crate::server::SharedGameState;

/// Actor request handlers
pub struct ActorHandlers;

/// Create actor request
#[derive(Debug, Deserialize)]
pub struct CreateActorRequest {
    pub name: String,
    pub race: String,
}

/// Create actor response
#[derive(Debug, Serialize)]
pub struct CreateActorResponse {
    pub success: bool,
    pub actor_id: Option<String>,
    pub message: String,
}

/// Get actor response
#[derive(Debug, Serialize)]
pub struct GetActorResponse {
    pub success: bool,
    pub actor: Option<Actor>,
    pub message: String,
}

/// Update actor request
#[derive(Debug, Deserialize)]
pub struct UpdateActorRequest {
    pub name: Option<String>,
    pub race: Option<String>,
    pub data: Option<serde_json::Value>,
}

/// Update actor response
#[derive(Debug, Serialize)]
pub struct UpdateActorResponse {
    pub success: bool,
    pub message: String,
}

/// List actors response
#[derive(Debug, Serialize)]
pub struct ListActorsResponse {
    pub success: bool,
    pub actors: Vec<Actor>,
    pub total_count: usize,
    pub message: String,
}

impl ActorHandlers {
    /// Create a new actor
    pub async fn create_actor(
        State(shared_state): State<Arc<SharedGameState>>,
        Json(request): Json<CreateActorRequest>,
    ) -> Result<Json<CreateActorResponse>, StatusCode> {
        debug!("Creating actor: {} ({})", request.name, request.race);
        
        // Validate request
        if request.name.is_empty() {
            return Ok(Json(CreateActorResponse {
                success: false,
                actor_id: None,
                message: "Actor name cannot be empty".to_string(),
            }));
        }
        
        if request.race.is_empty() {
            return Ok(Json(CreateActorResponse {
                success: false,
                actor_id: None,
                message: "Actor race cannot be empty".to_string(),
            }));
        }
        
        // Create actor using actor manager
        match shared_state.actor_manager.create_actor(request.name, request.race).await {
            Ok(actor) => {
                info!("Actor created successfully: {} ({})", actor.id, actor.name);
                Ok(Json(CreateActorResponse {
                    success: true,
                    actor_id: Some(actor.id),
                    message: "Actor created successfully".to_string(),
                }))
            }
            Err(e) => {
                error!("Failed to create actor: {}", e);
                Ok(Json(CreateActorResponse {
                    success: false,
                    actor_id: None,
                    message: format!("Failed to create actor: {}", e),
                }))
            }
        }
    }
    
    /// Get actor by ID
    pub async fn get_actor(
        State(shared_state): State<Arc<SharedGameState>>,
        Path(actor_id): Path<String>,
    ) -> Result<Json<GetActorResponse>, StatusCode> {
        debug!("Getting actor: {}", actor_id);
        
        match shared_state.actor_manager.get_actor(&actor_id) {
            Some(actor) => {
                debug!("Actor found: {}", actor_id);
                Ok(Json(GetActorResponse {
                    success: true,
                    actor: Some(actor),
                    message: "Actor found".to_string(),
                }))
            }
            None => {
                warn!("Actor not found: {}", actor_id);
                Ok(Json(GetActorResponse {
                    success: false,
                    actor: None,
                    message: "Actor not found".to_string(),
                }))
            }
        }
    }
    
    /// Update actor
    pub async fn update_actor(
        State(shared_state): State<Arc<SharedGameState>>,
        Path(actor_id): Path<String>,
        Json(request): Json<UpdateActorRequest>,
    ) -> Result<Json<UpdateActorResponse>, StatusCode> {
        debug!("Updating actor: {}", actor_id);
        
        // Get existing actor
        let mut actor = match shared_state.actor_manager.get_actor(&actor_id) {
            Some(actor) => actor,
            None => {
                warn!("Actor not found for update: {}", actor_id);
                return Ok(Json(UpdateActorResponse {
                    success: false,
                    message: "Actor not found".to_string(),
                }));
            }
        };
        
        // Update actor fields
        if let Some(name) = request.name {
            actor.name = name;
        }
        
        if let Some(race) = request.race {
            actor.race = race;
        }
        
        if let Some(data) = request.data {
            actor.data = data.as_object().unwrap_or(&serde_json::Map::new()).clone();
        }
        
        // Update actor
        match shared_state.actor_manager.update_actor(actor).await {
            Ok(_) => {
                info!("Actor updated successfully: {}", actor_id);
                Ok(Json(UpdateActorResponse {
                    success: true,
                    message: "Actor updated successfully".to_string(),
                }))
            }
            Err(e) => {
                error!("Failed to update actor {}: {}", actor_id, e);
                Ok(Json(UpdateActorResponse {
                    success: false,
                    message: format!("Failed to update actor: {}", e),
                }))
            }
        }
    }
    
    /// Delete actor
    pub async fn delete_actor(
        State(shared_state): State<Arc<SharedGameState>>,
        Path(actor_id): Path<String>,
    ) -> Result<Json<UpdateActorResponse>, StatusCode> {
        debug!("Deleting actor: {}", actor_id);
        
        match shared_state.actor_manager.remove_actor(&actor_id) {
            Some(_) => {
                info!("Actor deleted successfully: {}", actor_id);
                Ok(Json(UpdateActorResponse {
                    success: true,
                    message: "Actor deleted successfully".to_string(),
                }))
            }
            None => {
                warn!("Actor not found for deletion: {}", actor_id);
                Ok(Json(UpdateActorResponse {
                    success: false,
                    message: "Actor not found".to_string(),
                }))
            }
        }
    }
    
    /// List all actors
    pub async fn list_actors(
        State(shared_state): State<Arc<SharedGameState>>,
    ) -> Result<Json<ListActorsResponse>, StatusCode> {
        debug!("Listing all actors");
        
        let actors = shared_state.actor_manager.get_all_actors();
        let total_count = actors.len();
        
        info!("Listed {} actors", total_count);
        Ok(Json(ListActorsResponse {
            success: true,
            actors,
            total_count,
            message: format!("Found {} actors", total_count),
        }))
    }
    
    /// Get actors by race
    pub async fn get_actors_by_race(
        State(shared_state): State<Arc<SharedGameState>>,
        Path(race): Path<String>,
    ) -> Result<Json<ListActorsResponse>, StatusCode> {
        debug!("Getting actors by race: {}", race);
        
        let actors = shared_state.actor_manager.get_actors_by_race(&race);
        let total_count = actors.len();
        
        info!("Found {} actors with race: {}", total_count, race);
        Ok(Json(ListActorsResponse {
            success: true,
            actors,
            total_count,
            message: format!("Found {} actors with race: {}", total_count, race),
        }))
    }
    
    /// Get actors by level range
    pub async fn get_actors_by_level(
        State(shared_state): State<Arc<SharedGameState>>,
        Path((min_level, max_level)): Path<(u32, u32)>,
    ) -> Result<Json<ListActorsResponse>, StatusCode> {
        debug!("Getting actors by level range: {} - {}", min_level, max_level);
        
        let actors = shared_state.actor_manager.get_actors_by_level(min_level, max_level);
        let total_count = actors.len();
        
        info!("Found {} actors in level range: {} - {}", total_count, min_level, max_level);
        Ok(Json(ListActorsResponse {
            success: true,
            actors,
            total_count,
            message: format!("Found {} actors in level range: {} - {}", total_count, min_level, max_level),
        }))
    }
}
