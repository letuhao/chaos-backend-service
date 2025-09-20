//! Resource Manager Subsystem
//!
//! This subsystem manages all types of game resources (HP, Mana, Stamina, etc.)
//! through the Actor Core v3 architecture. It follows the "metadata-only aggregator"
//! principle, calculating resource values based on Actor metadata without storing state.

use async_trait::async_trait;
use std::collections::HashMap;
use crate::interfaces::Subsystem;
use crate::types::{Actor, SubsystemOutput, Contribution, CapContribution};
use crate::enums::{Bucket, CapMode};
use crate::ActorCoreResult;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Resource Manager Subsystem for managing game resources
#[derive(Debug, Clone)]
pub struct ResourceManagerSubsystem {
    /// Unique system identifier
    system_id: String,
    /// Processing priority (higher = more important)
    priority: i64,
    /// Resource definitions
    resource_definitions: HashMap<String, ResourceDefinition>,
}

/// Resource definition for different resource types
#[derive(Debug, Clone)]
pub struct ResourceDefinition {
    /// Resource identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Resource category
    pub category: ResourceCategory,
    /// Resource type
    pub resource_type: ResourceType,
    /// Base value calculation
    pub base_value: f64,
    /// Minimum value
    pub min_value: f64,
    /// Maximum value
    pub max_value: f64,
    /// Regeneration rate per second
    pub regen_rate: f64,
    /// Regeneration type
    pub regen_type: RegenType,
    /// Dependencies on other resources
    pub dependencies: Vec<String>,
    /// Additional tags
    pub tags: HashMap<String, String>,
}

/// Resource categories
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceCategory {
    Health,
    Energy,
    Physical,
    Cultivation,
    Special,
}

/// Resource types
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Current,    // Current value (e.g., hp_current)
    Max,        // Maximum value (e.g., hp_max)
    Regen,      // Regeneration rate (e.g., hp_regen)
    Percentage, // Percentage value (e.g., hp_percentage)
}

/// Regeneration types
#[derive(Debug, Clone, PartialEq)]
pub enum RegenType {
    Continuous,  // Regenerates continuously
    Tick,        // Regenerates per tick
    Conditional, // Regenerates based on conditions
    None,        // No regeneration
}

impl ResourceManagerSubsystem {
    /// Create a new Resource Manager Subsystem
    pub fn new() -> Self {
        let resource_definitions = Self::try_load_definitions_from_config()
            .unwrap_or_else(|_| {
                let mut defs = HashMap::new();
                Self::initialize_core_resources(&mut defs);
                defs
            });
        
        Self {
            system_id: "resource_manager".to_string(),
            priority: 100,
            resource_definitions,
        }
    }

    /// Attempt to load resource definitions from ACTOR_CORE_CONFIG_DIR/resources.(yaml|json)
    fn try_load_definitions_from_config() -> Result<HashMap<String, ResourceDefinition>, String> {
        let cfg_dir = std::env::var("ACTOR_CORE_CONFIG_DIR").map_err(|e| e.to_string())?;
        let mut defs: HashMap<String, ResourceDefinition> = HashMap::new();
        let yaml = PathBuf::from(&cfg_dir).join("resources.yaml");
        let json = PathBuf::from(&cfg_dir).join("resources.json");
        if yaml.exists() {
            let content = std::fs::read_to_string(&yaml).map_err(|e| e.to_string())?;
            let cfg: ResourcesConfig = serde_yaml::from_str(&content).map_err(|e| e.to_string())?;
            for r in cfg.resources { defs.insert(r.id.clone(), r.into()); }
            return Ok(defs);
        }
        if json.exists() {
            let content = std::fs::read_to_string(&json).map_err(|e| e.to_string())?;
            let cfg: ResourcesConfig = serde_json::from_str(&content).map_err(|e| e.to_string())?;
            for r in cfg.resources { defs.insert(r.id.clone(), r.into()); }
            return Ok(defs);
        }
        Err("resources config not found".into())
    }
    
    /// Initialize core resource definitions
    fn initialize_core_resources(definitions: &mut HashMap<String, ResourceDefinition>) {
        // Health Resources
        definitions.insert("hp".to_string(), ResourceDefinition {
            id: "hp".to_string(),
            name: "Health Points".to_string(),
            category: ResourceCategory::Health,
            resource_type: ResourceType::Current,
            base_value: 100.0,
            min_value: 0.0,
            max_value: 10000.0,
            regen_rate: 1.0,
            regen_type: RegenType::Continuous,
            dependencies: vec!["vitality".to_string()],
            tags: HashMap::from([
                ("combat_related".to_string(), "true".to_string()),
                ("critical".to_string(), "true".to_string()),
            ]),
        });
        
        definitions.insert("mana".to_string(), ResourceDefinition {
            id: "mana".to_string(),
            name: "Mana".to_string(),
            category: ResourceCategory::Energy,
            resource_type: ResourceType::Current,
            base_value: 50.0,
            min_value: 0.0,
            max_value: 5000.0,
            regen_rate: 2.0,
            regen_type: RegenType::Continuous,
            dependencies: vec!["intelligence".to_string()],
            tags: HashMap::from([
                ("magic_related".to_string(), "true".to_string()),
                ("castable".to_string(), "true".to_string()),
            ]),
        });
        
        definitions.insert("stamina".to_string(), ResourceDefinition {
            id: "stamina".to_string(),
            name: "Stamina".to_string(),
            category: ResourceCategory::Physical,
            resource_type: ResourceType::Current,
            base_value: 100.0,
            min_value: 0.0,
            max_value: 10000.0,
            regen_rate: 3.0,
            regen_type: RegenType::Continuous,
            dependencies: vec!["vitality".to_string()],
            tags: HashMap::from([
                ("movement_related".to_string(), "true".to_string()),
                ("combat_related".to_string(), "true".to_string()),
            ]),
        });
        
        definitions.insert("shield".to_string(), ResourceDefinition {
            id: "shield".to_string(),
            name: "Shield".to_string(),
            category: ResourceCategory::Special,
            resource_type: ResourceType::Current,
            base_value: 0.0,
            min_value: 0.0,
            max_value: 1000.0,
            regen_rate: 0.0,
            regen_type: RegenType::None,
            dependencies: vec![],
            tags: HashMap::from([
                ("temporary".to_string(), "true".to_string()),
                ("decay".to_string(), "true".to_string()),
            ]),
        });
    }
    
    /// Calculate base resource values for an actor
    fn calculate_base_resources(&self, actor: &Actor) -> HashMap<String, f64> {
        let mut resources = HashMap::new();
        
        for (resource_id, definition) in &self.resource_definitions {
            let base_value = self.calculate_base_value(definition, actor);
            
            // Set current and max values
            resources.insert(format!("{}_current", resource_id), base_value);
            resources.insert(format!("{}_max", resource_id), base_value);
            
            // Calculate regeneration rate
            if definition.regen_rate > 0.0 {
                resources.insert(format!("{}_regen", resource_id), definition.regen_rate);
            }
        }
        
        resources
    }
    
    /// Calculate base value for a specific resource
    fn calculate_base_value(&self, definition: &ResourceDefinition, actor: &Actor) -> f64 {
        let mut base_value = definition.base_value;
        
        // Apply lifespan modifier for health resources
        if definition.category == ResourceCategory::Health {
            base_value += (actor.lifespan as f64) * 10.0;
        }
        
        // Apply age modifier for energy resources
        if definition.category == ResourceCategory::Energy {
            base_value += (actor.age as f64) * 5.0;
        }
        
        // Apply race modifier
        let race_modifier = self.get_race_modifier(&actor.race, &definition.category);
        base_value *= race_modifier;
        
        // Apply cultivation modifier if applicable
        let cultivation_modifier = self.get_cultivation_modifier(actor, &definition.category);
        base_value *= cultivation_modifier;
        
        // Ensure within bounds
        base_value.max(definition.min_value).min(definition.max_value)
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
    
    /// Get cultivation modifier for resource calculation
    fn get_cultivation_modifier(&self, actor: &Actor, category: &ResourceCategory) -> f64 {
        // Check if actor has cultivation subsystems
        if actor.has_subsystem("jindan_system") || actor.has_subsystem("cultivation_system") {
            match category {
                ResourceCategory::Health => 1.5,
                ResourceCategory::Energy => 2.0,
                ResourceCategory::Cultivation => 3.0,
                _ => 1.1,
            }
        } else {
            1.0
        }
    }
    
    /// Create contributions from resource values
    fn create_contributions(&self, resources: HashMap<String, f64>, actor: &Actor) -> SubsystemOutput {
        let mut output = SubsystemOutput::new(self.system_id.clone());
        
        for (dimension, value) in resources {
            let bucket = self.get_bucket_for_dimension(&dimension);
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
        
        // Emit tick/decay/offline adjustments
        self.emit_time_based_contributions(&mut output, actor);
        
        output
    }
    
    /// Get bucket type for a dimension
    fn get_bucket_for_dimension(&self, _dimension: &str) -> Bucket {
        Bucket::Flat
    }
    
    /// Add cap contributions for resource limits
    fn add_cap_contributions(&self, output: &mut SubsystemOutput) {
        for (resource_id, definition) in &self.resource_definitions {
            // Add min cap
            if definition.min_value > 0.0 {
                let cap_contribution = CapContribution::new(
                    self.system_id.clone(),
                    format!("{}_current", resource_id),
                    CapMode::HardMin,
                    "min".to_string(),
                    definition.min_value,
                );
                output.add_cap(cap_contribution);
            }
            
            // Add max cap
            if definition.max_value < f64::INFINITY {
                let cap_contribution = CapContribution::new(
                    self.system_id.clone(),
                    format!("{}_current", resource_id),
                    CapMode::HardMax,
                    "max".to_string(),
                    definition.max_value,
                );
                output.add_cap(cap_contribution);
            }
        }
    }
}

#[async_trait]
impl Subsystem for ResourceManagerSubsystem {
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
        // Calculate base resources
        let resources = self.calculate_base_resources(actor);
        
        // Create contributions
        let output = self.create_contributions(resources, actor);
        
        Ok(output)
    }
}

// === Resource config structures ===

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ResourcesConfig { resources: Vec<ResourceDefConfig> }

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ResourceDefConfig {
    id: String,
    name: Option<String>,
    category: Option<String>,
    resource_type: Option<String>,
    base_value: Option<f64>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    regen_rate: Option<f64>,
    regen_type: Option<String>,
    dependencies: Option<Vec<String>>,
    tags: Option<HashMap<String, String>>,
}

impl From<ResourceDefConfig> for ResourceDefinition {
    fn from(cfg: ResourceDefConfig) -> Self {
        let category = match cfg.category.as_deref() {
            Some("health") => ResourceCategory::Health,
            Some("energy") => ResourceCategory::Energy,
            Some("physical") => ResourceCategory::Physical,
            Some("cultivation") => ResourceCategory::Cultivation,
            _ => ResourceCategory::Special,
        };
        let resource_type = match cfg.resource_type.as_deref() {
            Some("current") => ResourceType::Current,
            Some("max") => ResourceType::Max,
            Some("regen") => ResourceType::Regen,
            Some("percentage") => ResourceType::Percentage,
            _ => ResourceType::Current,
        };
        let regen_type = match cfg.regen_type.as_deref() {
            Some("continuous") => RegenType::Continuous,
            Some("tick") => RegenType::Tick,
            Some("conditional") => RegenType::Conditional,
            _ => RegenType::None,
        };
        ResourceDefinition {
            id: cfg.id,
            name: cfg.name.unwrap_or_else(|| "".to_string()),
            category,
            resource_type,
            base_value: cfg.base_value.unwrap_or(0.0),
            min_value: cfg.min_value.unwrap_or(0.0),
            max_value: cfg.max_value.unwrap_or(f64::INFINITY),
            regen_rate: cfg.regen_rate.unwrap_or(0.0),
            regen_type,
            dependencies: cfg.dependencies.unwrap_or_default(),
            tags: cfg.tags.unwrap_or_default(),
        }
    }
}

impl ResourceManagerSubsystem {
    fn emit_time_based_contributions(&self, output: &mut SubsystemOutput, actor: &Actor) {
        let delta = actor.get_data().get("tick_delta_seconds").and_then(|v| v.as_f64()).unwrap_or(0.0);
        if delta > 0.0 {
            // regen to current
            for (rid, def) in &self.resource_definitions {
                if def.regen_rate > 0.0 {
                    let dim = format!("{}_current", rid);
                    let mut c = Contribution::new(dim, Bucket::Flat, def.regen_rate * delta, self.system_id.clone());
                    c.priority = Some(90);
                    output.add_primary(c);
                }
            }
            // shield decay
            if let Some(decay) = actor.get_data().get("shield_decay_per_second").and_then(|v| v.as_f64()) {
                let mut c = Contribution::new("shield_current".into(), Bucket::Flat, -decay * delta, self.system_id.clone());
                c.priority = Some(95);
                output.add_primary(c);
            }
        }
        // offline catch-up
        if let Some(off_secs) = actor.get_data().get("offline_seconds").and_then(|v| v.as_f64()) {
            let max_secs = actor.get_data().get("offline_regen_max_seconds").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let window = off_secs.min(max_secs);
            if window > 0.0 {
                for (rid, def) in &self.resource_definitions {
                    if def.regen_rate > 0.0 {
                        // apply to mana/stamina always, hp only if out of combat
                        let apply = if rid == "hp" { !actor.is_in_combat() } else { true };
                        if apply {
                            let dim = format!("{}_current", rid);
                            let mut c = Contribution::new(dim, Bucket::Flat, def.regen_rate * window, self.system_id.clone());
                            c.priority = Some(85);
                            output.add_primary(c);
                        }
                    }
                }
            }
        }
    }
}

/// Convenience registration helper
pub fn register_with(plugin: &dyn crate::interfaces::PluginRegistry) -> crate::ActorCoreResult<()> {
    plugin.register(std::sync::Arc::new(ResourceManagerSubsystem::new()))
}