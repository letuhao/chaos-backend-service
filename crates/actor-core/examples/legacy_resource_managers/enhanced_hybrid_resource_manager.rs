//! Enhanced Hybrid Resource Manager Subsystem
//!
//! This subsystem implements the Enhanced Hybrid Resource Manager architecture:
//! - Shared Resources (Folder): Always cached, calculated from all systems
//! - System Resources (Files): Delegated to individual cultivation systems
//! - Database Storage: Inactive actors stored in MongoDB to reduce memory overhead
//! - Smart Recalculation: Only when primary stats change, not on every resource consumption

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::interfaces::Subsystem;
use crate::types::{Actor, SubsystemOutput, Contribution, CapContribution};
use crate::enums::{Bucket, CapMode};
use crate::ActorCoreResult;
use serde::{Deserialize, Serialize};

/// Enhanced Hybrid Resource Manager - Main coordinator
pub struct EnhancedHybridResourceManager {
    /// Unique system identifier
    system_id: String,
    /// Processing priority
    priority: i64,
    /// Shared resource cache (always cached)
    shared_resource_cache: Arc<RwLock<HashMap<String, SharedResource>>>,
    /// System resource managers
    system_managers: HashMap<String, Box<dyn SystemResourceCalculator + Send + Sync>>,
    /// Database storage for inactive actors
    database: Option<Arc<dyn ResourceDatabase + Send + Sync>>,
    /// Stat change notifier
    stat_change_notifier: Arc<dyn StatChangeNotifier + Send + Sync>,
    /// Resource cache system
    resource_cache: Arc<dyn ResourceCache + Send + Sync>,
}

/// Shared resource (always cached)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedResource {
    /// Resource identifier
    pub id: String,
    /// Current value
    pub current: f64,
    /// Maximum value
    pub max: f64,
    /// Regeneration rate
    pub regen_rate: f64,
    /// Last updated timestamp
    pub last_updated: u64,
    /// Resource category
    pub category: ResourceCategory,
    /// Dependencies on other resources
    pub dependencies: Vec<String>,
}

/// Resource categories for shared resources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceCategory {
    Health,      // HP, lifespan
    Energy,      // Mana, spiritual energy
    Physical,    // Stamina, vitality
    Cultivation, // Qi, dao energy
    Special,     // Shield, temporary effects
}

/// System Resource Calculator trait for delegation
#[async_trait]
pub trait SystemResourceCalculator: Send + Sync {
    /// Calculate resources for this system
    async fn calculate_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>>;
    
    /// Get system identifier
    fn system_id(&self) -> &str;
    
    /// Check if this system affects a specific resource
    fn affects_resource(&self, resource_id: &str) -> bool;
    
    /// Notify of stat changes
    async fn notify_stat_change(&self, actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()>;
}

/// Resource Database trait for persistence
#[async_trait]
pub trait ResourceDatabase: Send + Sync {
    /// Store actor resources
    async fn store_actor_resources(&self, actor_id: &str, resources: &HashMap<String, f64>) -> ActorCoreResult<()>;
    
    /// Load actor resources
    async fn load_actor_resources(&self, actor_id: &str) -> ActorCoreResult<HashMap<String, f64>>;
    
    /// Check if actor is active
    async fn is_actor_active(&self, actor_id: &str) -> ActorCoreResult<bool>;
    
    /// Mark actor as inactive
    async fn mark_actor_inactive(&self, actor_id: &str) -> ActorCoreResult<()>;
    
    /// Mark actor as active
    async fn mark_actor_active(&self, actor_id: &str) -> ActorCoreResult<()>;
}

/// Stat Change Notifier trait
#[async_trait]
pub trait StatChangeNotifier: Send + Sync {
    /// Notify of stat changes
    async fn notify_stat_change(&self, actor: &Actor, changed_stats: &[String]) -> ActorCoreResult<()>;
    
    /// Check if resources need recalculation
    async fn needs_recalculation(&self, actor: &Actor, resource_id: &str) -> ActorCoreResult<bool>;
    
    /// Get resources affected by stat changes
    async fn get_affected_resources(&self, changed_stats: &[String]) -> ActorCoreResult<Vec<String>>;
}

/// Resource Cache trait
#[async_trait]
pub trait ResourceCache: Send + Sync {
    /// Get cached resource value
    async fn get(&self, actor_id: &str, resource_id: &str) -> ActorCoreResult<Option<f64>>;
    
    /// Set cached resource value
    async fn set(&self, actor_id: &str, resource_id: &str, value: f64) -> ActorCoreResult<()>;
    
    /// Invalidate cache for actor
    async fn invalidate_actor(&self, actor_id: &str) -> ActorCoreResult<()>;
    
    /// Invalidate cache for resource
    async fn invalidate_resource(&self, resource_id: &str) -> ActorCoreResult<()>;
    
    /// Clear all cache
    async fn clear(&self) -> ActorCoreResult<()>;
}

impl EnhancedHybridResourceManager {
    /// Create a new Enhanced Hybrid Resource Manager
    pub fn new() -> Self {
        Self {
            system_id: "enhanced_hybrid_resource_manager".to_string(),
            priority: 100,
            shared_resource_cache: Arc::new(RwLock::new(HashMap::new())),
            system_managers: HashMap::new(),
            database: None,
            stat_change_notifier: Arc::new(DefaultStatChangeNotifier::new()),
            resource_cache: Arc::new(DefaultResourceCache::new()),
        }
    }
    
    /// Add a system resource manager
    pub fn add_system_manager(&mut self, system_id: String, manager: Box<dyn SystemResourceCalculator + Send + Sync>) {
        self.system_managers.insert(system_id, manager);
    }
    
    /// Set database storage
    pub fn set_database(&mut self, database: Arc<dyn ResourceDatabase + Send + Sync>) {
        self.database = Some(database);
    }
    
    /// Set stat change notifier
    pub fn set_stat_change_notifier(&mut self, notifier: Arc<dyn StatChangeNotifier + Send + Sync>) {
        self.stat_change_notifier = notifier;
    }
    
    /// Set resource cache
    pub fn set_resource_cache(&mut self, cache: Arc<dyn ResourceCache + Send + Sync>) {
        self.resource_cache = cache;
    }
    
    /// Calculate shared resources (always cached)
    async fn calculate_shared_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut shared_resources = HashMap::new();
        
        // Calculate HP (always cached)
        let hp = self.calculate_hp(actor).await?;
        shared_resources.insert("hp_current".to_string(), hp.current);
        shared_resources.insert("hp_max".to_string(), hp.max);
        shared_resources.insert("hp_regen".to_string(), hp.regen_rate);
        
        // Calculate Mana (always cached)
        let mana = self.calculate_mana(actor).await?;
        shared_resources.insert("mana_current".to_string(), mana.current);
        shared_resources.insert("mana_max".to_string(), mana.max);
        shared_resources.insert("mana_regen".to_string(), mana.regen_rate);
        
        // Calculate Lifespan (always cached)
        let lifespan = self.calculate_lifespan(actor).await?;
        shared_resources.insert("lifespan_years".to_string(), lifespan.current);
        shared_resources.insert("age_years".to_string(), lifespan.max);
        
        // Update shared resource cache
        self.update_shared_cache(&actor.id.to_string(), &shared_resources).await?;
        
        Ok(shared_resources)
    }
    
    /// Calculate HP resource
    async fn calculate_hp(&self, actor: &Actor) -> ActorCoreResult<SharedResource> {
        // Base HP from lifespan
        let base_hp = (actor.lifespan as f64) * 10.0;
        
        // Apply race modifier
        let race_modifier = self.get_race_modifier(&actor.race, &ResourceCategory::Health);
        let modified_hp = base_hp * race_modifier;
        
        // Apply age modifier (older = more HP)
        let age_modifier = 1.0 + (actor.age as f64) / 100.0;
        let final_hp = modified_hp * age_modifier;
        
        // Apply cultivation modifiers from all systems
        let cultivation_modifier = self.get_cultivation_modifier(actor, &ResourceCategory::Health).await?;
        let final_hp = final_hp * cultivation_modifier;
        
        Ok(SharedResource {
            id: "hp".to_string(),
            current: final_hp,
            max: final_hp,
            regen_rate: final_hp * 0.01, // 1% per second
            last_updated: chrono::Utc::now().timestamp() as u64,
            category: ResourceCategory::Health,
            dependencies: vec!["lifespan".to_string(), "vitality".to_string()],
        })
    }
    
    /// Calculate Mana resource
    async fn calculate_mana(&self, actor: &Actor) -> ActorCoreResult<SharedResource> {
        // Base mana from age
        let base_mana = (actor.age as f64) * 5.0;
        
        // Apply race modifier
        let race_modifier = self.get_race_modifier(&actor.race, &ResourceCategory::Energy);
        let modified_mana = base_mana * race_modifier;
        
        // Apply cultivation modifiers from all systems
        let cultivation_modifier = self.get_cultivation_modifier(actor, &ResourceCategory::Energy).await?;
        let final_mana = modified_mana * cultivation_modifier;
        
        Ok(SharedResource {
            id: "mana".to_string(),
            current: final_mana,
            max: final_mana,
            regen_rate: final_mana * 0.02, // 2% per second
            last_updated: chrono::Utc::now().timestamp() as u64,
            category: ResourceCategory::Energy,
            dependencies: vec!["intelligence".to_string(), "spirituality".to_string()],
        })
    }
    
    /// Calculate Lifespan resource
    async fn calculate_lifespan(&self, actor: &Actor) -> ActorCoreResult<SharedResource> {
        // Lifespan is immutable, but we track age
        let lifespan = actor.lifespan as f64;
        let age = actor.age as f64;
        
        Ok(SharedResource {
            id: "lifespan".to_string(),
            current: age,
            max: lifespan,
            regen_rate: 0.0, // No regeneration
            last_updated: chrono::Utc::now().timestamp() as u64,
            category: ResourceCategory::Health,
            dependencies: vec![],
        })
    }
    
    /// Get race modifier for resource calculation
    fn get_race_modifier(&self, race: &str, category: &ResourceCategory) -> f64 {
        match (race, category) {
            ("Human", ResourceCategory::Health) => 1.0,
            ("Elf", ResourceCategory::Energy) => 1.2,
            ("Dwarf", ResourceCategory::Physical) => 1.1,
            ("Orc", ResourceCategory::Health) => 1.3,
            _ => 1.0,
        }
    }
    
    /// Get cultivation modifier from all systems
    async fn get_cultivation_modifier(&self, actor: &Actor, category: &ResourceCategory) -> ActorCoreResult<f64> {
        let mut total_modifier = 1.0;
        
        // Check each system manager
        for (_system_id, manager) in &self.system_managers {
            if manager.affects_resource(&format!("{}_current", category_to_resource_id(category))) {
                // Get system-specific modifier
                let system_modifier = self.get_system_modifier(manager.as_ref(), actor, category).await?;
                total_modifier *= system_modifier;
            }
        }
        
        Ok(total_modifier)
    }
    
    /// Get modifier from a specific system
    async fn get_system_modifier(&self, _manager: &dyn SystemResourceCalculator, _actor: &Actor, category: &ResourceCategory) -> ActorCoreResult<f64> {
        // This would be implemented by each system manager
        // For now, return a default modifier
        match category {
            ResourceCategory::Health => Ok(1.5),
            ResourceCategory::Energy => Ok(2.0),
            ResourceCategory::Cultivation => Ok(3.0),
            _ => Ok(1.1),
        }
    }
    
    /// Update shared resource cache
    async fn update_shared_cache(&self, actor_id: &str, resources: &HashMap<String, f64>) -> ActorCoreResult<()> {
        let mut cache = self.shared_resource_cache.write().await;
        
        for (resource_id, value) in resources {
            let shared_resource = SharedResource {
                id: resource_id.clone(),
                current: *value,
                max: *value,
                regen_rate: 0.0,
                last_updated: chrono::Utc::now().timestamp() as u64,
                category: ResourceCategory::Health, // This should be determined by resource_id
                dependencies: vec![],
            };
            cache.insert(format!("{}:{}", actor_id, resource_id), shared_resource);
        }
        
        Ok(())
    }
    
    /// Calculate system resources (delegated to child systems)
    async fn calculate_system_resources(&self, actor: &Actor) -> ActorCoreResult<HashMap<String, f64>> {
        let mut system_resources = HashMap::new();
        
        // Delegate to each system manager
        for (system_id, manager) in &self.system_managers {
            let system_result = manager.calculate_resources(actor).await?;
            
            // Merge results
            for (resource_id, value) in system_result {
                system_resources.insert(format!("{}:{}", system_id, resource_id), value);
            }
        }
        
        Ok(system_resources)
    }
    
    /// Create contributions from resource values
    fn create_contributions(&self, resources: HashMap<String, f64>, _actor: &Actor) -> SubsystemOutput {
        let mut output = SubsystemOutput::new(self.system_id.clone());
        
        for (dimension, value) in resources {
            let bucket = Bucket::Flat;
            let mut contribution = Contribution::new(
                dimension.clone(),
                bucket,
                value,
                self.system_id.clone(),
            );
            contribution.priority = Some(100);
            output.add_primary(contribution);
        }
        
        // Add cap contributions for resource limits
        self.add_cap_contributions(&mut output);
        
        output
    }
    
    /// Add cap contributions for resource limits
    fn add_cap_contributions(&self, output: &mut SubsystemOutput) {
        // Add caps for shared resources
        let shared_resources = vec!["hp_current", "mana_current", "lifespan_years"];
        
        for resource_id in shared_resources {
            // Add min cap (0 for most resources)
            let cap_contribution = CapContribution::new(
                self.system_id.clone(),
                resource_id.to_string(),
                CapMode::HardMin,
                "min".to_string(),
                0.0,
            );
            output.add_cap(cap_contribution);
            
            // Add max cap (reasonable limits)
            let max_value = match resource_id {
                "hp_current" => 10000.0,
                "mana_current" => 5000.0,
                "lifespan_years" => 1000.0,
                _ => f64::INFINITY,
            };
            
            if max_value < f64::INFINITY {
                let cap_contribution = CapContribution::new(
                    self.system_id.clone(),
                    resource_id.to_string(),
                    CapMode::HardMax,
                    "max".to_string(),
                    max_value,
                );
                output.add_cap(cap_contribution);
            }
        }
    }
}

#[async_trait]
impl Subsystem for EnhancedHybridResourceManager {
    /// Get the unique identifier for this subsystem
    fn system_id(&self) -> &str {
        &self.system_id
    }
    
    /// Get the priority of this subsystem
    fn priority(&self) -> i64 {
        self.priority
    }
    
    /// Contribute to actor stats
    async fn contribute(&self, actor: &Actor) -> ActorCoreResult<SubsystemOutput> {
        // Check if actor is active (in memory) or inactive (in database)
        let is_active = if let Some(db) = &self.database {
            db.is_actor_active(&actor.id.to_string()).await.unwrap_or(true)
        } else {
            true
        };
        
        if !is_active {
            // Load from database
            if let Some(db) = &self.database {
                let resources = db.load_actor_resources(&actor.id.to_string()).await?;
                return Ok(self.create_contributions(resources, actor));
            }
        }
        
        // Calculate shared resources (always cached)
        let shared_resources = self.calculate_shared_resources(actor).await?;
        
        // Calculate system resources (delegated)
        let system_resources = self.calculate_system_resources(actor).await?;
        
        // Merge all resources
        let mut all_resources = shared_resources;
        all_resources.extend(system_resources);
        
        // Store in database if actor is inactive
        if let Some(db) = &self.database {
            db.store_actor_resources(&actor.id.to_string(), &all_resources).await?;
        }
        
        // Create contributions
        let output = self.create_contributions(all_resources, actor);
        
        Ok(output)
    }
}

/// Helper function to convert category to resource ID
fn category_to_resource_id(category: &ResourceCategory) -> &'static str {
    match category {
        ResourceCategory::Health => "hp",
        ResourceCategory::Energy => "mana",
        ResourceCategory::Physical => "stamina",
        ResourceCategory::Cultivation => "qi",
        ResourceCategory::Special => "shield",
    }
}

/// Default Stat Change Notifier implementation
#[derive(Debug)]
pub struct DefaultStatChangeNotifier;

impl DefaultStatChangeNotifier {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl StatChangeNotifier for DefaultStatChangeNotifier {
    async fn notify_stat_change(&self, _actor: &Actor, _changed_stats: &[String]) -> ActorCoreResult<()> {
        // Default implementation - no-op
        Ok(())
    }
    
    async fn needs_recalculation(&self, _actor: &Actor, _resource_id: &str) -> ActorCoreResult<bool> {
        // Default implementation - always needs recalculation
        Ok(true)
    }
    
    async fn get_affected_resources(&self, _changed_stats: &[String]) -> ActorCoreResult<Vec<String>> {
        // Default implementation - return empty list
        Ok(vec![])
    }
}

/// Default Resource Cache implementation
#[derive(Debug)]
pub struct DefaultResourceCache {
    cache: Arc<RwLock<HashMap<String, f64>>>,
}

impl DefaultResourceCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ResourceCache for DefaultResourceCache {
    async fn get(&self, actor_id: &str, resource_id: &str) -> ActorCoreResult<Option<f64>> {
        let cache = self.cache.read().await;
        Ok(cache.get(&format!("{}:{}", actor_id, resource_id)).copied())
    }
    
    async fn set(&self, actor_id: &str, resource_id: &str, value: f64) -> ActorCoreResult<()> {
        let mut cache = self.cache.write().await;
        cache.insert(format!("{}:{}", actor_id, resource_id), value);
        Ok(())
    }
    
    async fn invalidate_actor(&self, actor_id: &str) -> ActorCoreResult<()> {
        let mut cache = self.cache.write().await;
        cache.retain(|key, _| !key.starts_with(&format!("{}:", actor_id)));
        Ok(())
    }
    
    async fn invalidate_resource(&self, resource_id: &str) -> ActorCoreResult<()> {
        let mut cache = self.cache.write().await;
        cache.retain(|key, _| !key.ends_with(&format!(":{}", resource_id)));
        Ok(())
    }
    
    async fn clear(&self) -> ActorCoreResult<()> {
        let mut cache = self.cache.write().await;
        cache.clear();
        Ok(())
    }
}