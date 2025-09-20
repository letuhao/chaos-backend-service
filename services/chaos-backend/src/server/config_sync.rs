//! Configuration Sync Service
//!
//! This service handles loading configuration from files and syncing to MongoDB database.
//! Runs in a background task to keep configurations synchronized.

use std::time::Duration;
use tokio::time::interval;
use tokio::task::JoinHandle;
use tracing::{info, warn, error, debug};

use actor_core::prelude::*;
use actor_core::config::mongodb::{MongoDBConfigurationProvider, MongoDBConfig};
use actor_core::config::mongodb_manager::MongoDBConfigManager;

use crate::server::SharedGameState;

/// Configuration synchronization service
pub struct ConfigSyncService {
    /// Shared game state
    shared_state: Arc<SharedGameState>,
    
    /// MongoDB config manager
    mongodb_manager: Option<Arc<MongoDBConfigManager>>,
    
    /// Background task handle
    sync_task: Option<JoinHandle<()>>,
    
    /// Sync interval
    sync_interval: Duration,
    
    /// Service running flag
    is_running: Arc<std::sync::atomic::AtomicBool>,
}

impl ConfigSyncService {
    /// Create a new configuration sync service
    pub fn new(shared_state: Arc<SharedGameState>) -> Self {
        let sync_interval = Duration::from_secs(
            shared_state.server_config.read().config_sync_interval
        );
        
        Self {
            shared_state,
            mongodb_manager: None,
            sync_task: None,
            sync_interval,
            is_running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
    
    /// Initialize MongoDB connection and manager
    pub async fn initialize_mongodb(&mut self) -> ActorCoreResult<()> {
        let server_config = self.shared_state.server_config.read();
        
        if !server_config.enable_mongodb_sync {
            info!("MongoDB sync is disabled, skipping initialization");
            return Ok(());
        }
        
        info!("Initializing MongoDB configuration sync...");
        
        // Load MongoDB configuration
        let mongodb_config = MongoDBConfigurationProvider::load_mongodb_config("configs/mongodb_config.yaml")?;
        
        // Create MongoDB manager
        let mongodb_manager = Arc::new(MongoDBConfigManager::new(mongodb_config).await?);
        
        // Start auto-sync if enabled
        if mongodb_manager.get_sync_status().is_sync_enabled {
            info!("Starting MongoDB auto-sync daemon...");
            mongodb_manager.start_auto_sync().await?;
        }
        
        self.mongodb_manager = Some(mongodb_manager);
        
        info!("MongoDB configuration sync initialized successfully");
        Ok(())
    }
    
    /// Start the configuration sync service
    pub async fn start(&mut self) -> ActorCoreResult<()> {
        if self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            warn!("Config sync service is already running");
            return Ok(());
        }
        
        info!("Starting configuration sync service...");
        
        // Initialize MongoDB if enabled
        self.initialize_mongodb().await?;
        
        // Start background sync task
        let shared_state = self.shared_state.clone();
        let sync_interval = self.sync_interval;
        let is_running = self.is_running.clone();
        
        let sync_task = tokio::spawn(async move {
            let mut interval = interval(sync_interval);
            
            // Set running flag
            is_running.store(true, std::sync::atomic::Ordering::Relaxed);
            info!("Configuration sync service started");
            
            loop {
                interval.tick().await;
                
                if !is_running.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
                
                // Perform configuration sync
                if let Err(e) = Self::perform_config_sync(&shared_state).await {
                    error!("Configuration sync failed: {}", e);
                }
            }
            
            info!("Configuration sync service stopped");
        });
        
        self.sync_task = Some(sync_task);
        
        info!("Configuration sync service started successfully");
        Ok(())
    }
    
    /// Stop the configuration sync service
    pub async fn stop(&mut self) {
        if !self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        
        info!("Stopping configuration sync service...");
        
        // Set running flag to false
        self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);
        
        // Wait for sync task to finish
        if let Some(task) = self.sync_task.take() {
            task.abort();
            let _ = task.await;
        }
        
        info!("Configuration sync service stopped");
    }
    
    /// Perform configuration synchronization
    async fn perform_config_sync(shared_state: &SharedGameState) -> ActorCoreResult<()> {
        debug!("Performing configuration sync...");
        
        // Load configurations from files
        let config_manager = shared_state.actor_core.get_config_manager();
        
        // Get all supported categories
        let categories = config_manager.get_registry().get_supported_categories();
        debug!("Found {} configuration categories", categories.len());
        
        // Sync each category to MongoDB if enabled
        for category in categories {
            debug!("Syncing category: {}", category);
            
            // Get category configuration
            if let Ok(category_config) = config_manager.get_category_config(&category).await {
                debug!("Loaded {} configs for category: {}", category_config.len(), category);
                
                // Here we would save to MongoDB if MongoDB manager is available
                // For now, just log the configuration count
                debug!("Category '{}' has {} configurations", category, category_config.len());
            } else {
                warn!("Failed to load configuration for category: {}", category);
            }
        }
        
        debug!("Configuration sync completed");
        Ok(())
    }
    
    /// Manually trigger configuration sync
    pub async fn sync_now(&self) -> ActorCoreResult<()> {
        info!("Manually triggering configuration sync...");
        Self::perform_config_sync(&self.shared_state).await
    }
    
    /// Get sync status
    pub fn get_sync_status(&self) -> ConfigSyncStatus {
        ConfigSyncStatus {
            is_running: self.is_running.load(std::sync::atomic::Ordering::Relaxed),
            sync_interval_seconds: self.sync_interval.as_secs(),
            mongodb_enabled: self.mongodb_manager.is_some(),
        }
    }
}

/// Configuration sync status
#[derive(Debug, Clone)]
pub struct ConfigSyncStatus {
    pub is_running: bool,
    pub sync_interval_seconds: u64,
    pub mongodb_enabled: bool,
}

impl Drop for ConfigSyncService {
    fn drop(&mut self) {
        if self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            // Set running flag to false
            self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);
            
            // Abort sync task if running
            if let Some(task) = self.sync_task.take() {
                task.abort();
            }
        }
    }
}
