//! Comprehensive tests for types.rs coverage.
//!
//! This module contains detailed tests for all types defined in types.rs,
//! including Actor, Contribution, Snapshot, Caps, Subsystem, ModifierPack,
//! and other core data structures to achieve 80%+ line coverage.

use actor_core::types::*;
use actor_core::enums::{Bucket, CapMode};
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    // === Actor Tests ===

    #[tokio::test]
    async fn test_actor_creation_and_validation() {
        // Test valid actor creation
        let actor = Actor::new("TestActor".to_string(), "Human".to_string());
        assert!(actor.is_valid());
        assert_eq!(actor.get_name(), "TestActor");
        assert_eq!(actor.get_race(), "Human");
        assert_eq!(actor.get_version(), 1);
        assert_eq!(actor.get_subsystem_count(), 0);
        
        // Test invalid actor creation
        let mut invalid_actor = Actor::new("".to_string(), "Human".to_string());
        assert!(!invalid_actor.is_valid());
        
        invalid_actor = Actor::new("TestActor".to_string(), "".to_string());
        assert!(!invalid_actor.is_valid());
    }

    #[tokio::test]
    async fn test_actor_getters_and_setters() {
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Test name operations
        actor.set_name("NewName".to_string());
        assert_eq!(actor.get_name(), "NewName");
        
        // Test race operations
        actor.set_race("Elf".to_string());
        assert_eq!(actor.get_race(), "Elf");
        
        // Test lifespan operations
        actor.set_lifespan(1000);
        assert_eq!(actor.get_lifespan(), 1000);
        
        // Test age operations
        actor.set_age(25);
        assert_eq!(actor.get_age(), 25);
        
        // Test version operations
        let initial_version = actor.get_version();
        actor.touch();
        assert_eq!(actor.get_version(), initial_version + 1);
        
        // Test data operations
        let data = actor.get_data();
        assert!(data.is_empty());
    }

    #[tokio::test]
    async fn test_actor_subsystem_management() {
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Test adding subsystem
        let subsystem = Subsystem::new("combat".to_string(), 10);
        actor.add_subsystem(subsystem);
        assert_eq!(actor.get_subsystem_count(), 1);
        assert!(actor.has_subsystem("combat"));
        
        // Test getting subsystem
        let retrieved = actor.get_subsystem("combat");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().get_system_id(), "combat");
        
        // Test removing subsystem
        let removed = actor.remove_subsystem("combat");
        assert!(removed);
        assert_eq!(actor.get_subsystem_count(), 0);
        assert!(!actor.has_subsystem("combat"));
        
        // Test removing non-existent subsystem
        let removed = actor.remove_subsystem("nonexistent");
        assert!(!removed);
    }

    #[tokio::test]
    async fn test_actor_combat_system() {
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Test initial combat status
        assert!(!actor.is_in_combat());
        assert_eq!(actor.get_combat_duration(), None);
        
        // Test setting combat status
        actor.set_in_combat(true);
        assert!(actor.is_in_combat());
        assert!(actor.get_combat_duration().is_some());
        
        // Test setting combat duration
        actor.set_combat_duration(120);
        assert!(actor.get_combat_duration().unwrap() >= 120);
        
        // Test last combat time
        actor.set_in_combat(false);
        assert!(!actor.is_in_combat());
    }

    #[tokio::test]
    async fn test_actor_buff_management() {
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Test initial buff status
        assert!(!actor.has_buffs());
        assert!(!actor.has_buff("strength"));
        assert!(actor.get_active_buffs().is_empty());
        
        // Test adding buff
        actor.add_buff("strength".to_string());
        assert!(actor.has_buffs());
        assert!(actor.has_buff("strength"));
        assert_eq!(actor.get_active_buffs().len(), 1);
        assert_eq!(actor.get_active_buffs()[0], "strength");
        
        // Test adding duplicate buff
        actor.add_buff("strength".to_string());
        assert_eq!(actor.get_active_buffs().len(), 1);
        
        // Test adding multiple buffs
        actor.add_buff("agility".to_string());
        assert_eq!(actor.get_active_buffs().len(), 2);
        
        // Test removing buff
        let removed = actor.remove_buff("strength");
        assert!(removed);
        assert!(!actor.has_buff("strength"));
        assert_eq!(actor.get_active_buffs().len(), 1);
        
        // Test removing non-existent buff
        let removed = actor.remove_buff("nonexistent");
        assert!(!removed);
        
        // Test clearing all buffs
        actor.clear_buffs();
        assert!(!actor.has_buffs());
        assert!(actor.get_active_buffs().is_empty());
    }

    #[tokio::test]
    async fn test_actor_guild_management() {
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Test initial guild status
        assert!(!actor.is_guild_member());
        assert_eq!(actor.get_guild_id(), None);
        
        // Test setting guild ID
        actor.set_guild_id("guild123".to_string());
        assert!(actor.is_guild_member());
        assert_eq!(actor.get_guild_id(), Some("guild123"));
    }

    #[tokio::test]
    async fn test_actor_online_status() {
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Test initial online status (defaults to true)
        assert!(actor.is_online());
        
        // Test setting offline
        actor.set_online(false);
        assert!(!actor.is_online());
        
        // Test setting online
        actor.set_online(true);
        assert!(actor.is_online());
    }

    #[tokio::test]
    async fn test_actor_subsystem_priority_ordering() {
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Add subsystems with different priorities
        let combat = Subsystem::new("combat".to_string(), 10);
        let magic = Subsystem::new("magic".to_string(), 5);
        let stealth = Subsystem::new("stealth".to_string(), 15);
        
        actor.add_subsystem(combat);
        actor.add_subsystem(magic);
        actor.add_subsystem(stealth);
        
        // Test priority ordering
        let ordered = actor.get_subsystem_by_priority();
        assert_eq!(ordered.len(), 3);
        assert_eq!(ordered[0].get_system_id(), "stealth"); // highest priority
        assert_eq!(ordered[1].get_system_id(), "combat");
        assert_eq!(ordered[2].get_system_id(), "magic"); // lowest priority
    }

    #[tokio::test]
    async fn test_actor_version_management() {
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Test initial version
        assert_eq!(actor.get_version(), 1);
        
        // Test touch method
        actor.touch();
        assert_eq!(actor.get_version(), 2);
        
        // Test update_version method
        actor.update_version();
        assert_eq!(actor.get_version(), 3);
    }

    // === Contribution Tests ===

    #[tokio::test]
    async fn test_contribution_creation_and_validation() {
        // Test valid contribution
        let contribution = Contribution::new(
            "strength".to_string(),
            Bucket::Flat,
            10.0,
            "combat".to_string()
        );
        assert!(contribution.is_valid());
        assert_eq!(contribution.dimension, "strength");
        assert_eq!(contribution.bucket, Bucket::Flat);
        assert_eq!(contribution.value, 10.0);
        assert_eq!(contribution.system, "combat");
        
        // Test invalid contribution
        let invalid_contribution = Contribution {
            dimension: "".to_string(),
            bucket: Bucket::Flat,
            value: 10.0,
            system: "combat".to_string(),
            priority: None,
            tags: None,
        };
        assert!(!invalid_contribution.is_valid());
        
        // Test contribution with negative priority
        let invalid_priority = Contribution {
            dimension: "strength".to_string(),
            bucket: Bucket::Flat,
            value: 10.0,
            system: "combat".to_string(),
            priority: Some(-1),
            tags: None,
        };
        assert!(!invalid_priority.is_valid());
        
        // Test contribution with NaN value
        let invalid_value = Contribution {
            dimension: "strength".to_string(),
            bucket: Bucket::Flat,
            value: f64::NAN,
            system: "combat".to_string(),
            priority: None,
            tags: None,
        };
        assert!(!invalid_value.is_valid());
    }

    #[tokio::test]
    async fn test_contribution_with_priority_and_tags() {
        let mut tags = HashMap::new();
        tags.insert("source".to_string(), "equipment".to_string());
        tags.insert("temporary".to_string(), "true".to_string());
        
        let contribution = Contribution {
            dimension: "strength".to_string(),
            bucket: Bucket::Mult,
            value: 1.2,
            system: "equipment".to_string(),
            priority: Some(5),
            tags: Some(tags),
        };
        
        assert!(contribution.is_valid());
        assert_eq!(contribution.priority, Some(5));
        assert!(contribution.tags.is_some());
        assert_eq!(contribution.tags.as_ref().unwrap().get("source"), Some(&"equipment".to_string()));
    }

    // === CapContribution Tests ===

    #[tokio::test]
    async fn test_cap_contribution_creation_and_validation() {
        // Test valid cap contribution
        let cap_contribution = CapContribution::new(
            "combat".to_string(),
            "strength".to_string(),
            CapMode::HardMax,
            "max".to_string(),
            100.0
        );
        assert!(cap_contribution.is_valid());
        assert_eq!(cap_contribution.system, "combat");
        assert_eq!(cap_contribution.dimension, "strength");
        assert_eq!(cap_contribution.mode, CapMode::HardMax);
        assert_eq!(cap_contribution.kind, "max");
        assert_eq!(cap_contribution.value, 100.0);
        
        // Test invalid cap contribution
        let invalid_cap = CapContribution {
            system: "".to_string(),
            dimension: "strength".to_string(),
            mode: CapMode::HardMax,
            kind: "max".to_string(),
            value: 100.0,
            priority: None,
            scope: None,
            realm: None,
            tags: None,
        };
        assert!(!invalid_cap.is_valid());
        
        // Test cap contribution with negative priority
        let invalid_priority = CapContribution {
            system: "combat".to_string(),
            dimension: "strength".to_string(),
            mode: CapMode::HardMax,
            kind: "max".to_string(),
            value: 100.0,
            priority: Some(-1),
            scope: None,
            realm: None,
            tags: None,
        };
        assert!(!invalid_priority.is_valid());
    }

    #[tokio::test]
    async fn test_cap_contribution_with_scope_and_realm() {
        let mut tags = HashMap::new();
        tags.insert("temporary".to_string(), "true".to_string());
        
        let cap_contribution = CapContribution {
            system: "combat".to_string(),
            dimension: "strength".to_string(),
            mode: CapMode::Baseline,
            kind: "max".to_string(),
            value: 100.0,
            priority: Some(10),
            scope: Some("combat".to_string()),
            realm: Some("pvp".to_string()),
            tags: Some(tags),
        };
        
        assert!(cap_contribution.is_valid());
        assert_eq!(cap_contribution.scope, Some("combat".to_string()));
        assert_eq!(cap_contribution.realm, Some("pvp".to_string()));
        assert_eq!(cap_contribution.priority, Some(10));
    }

    // === Subsystem Tests ===

    #[tokio::test]
    async fn test_subsystem_creation_and_management() {
        let mut subsystem = Subsystem::new("combat".to_string(), 10);
        
        // Test initial state
        assert_eq!(subsystem.get_system_id(), "combat");
        assert_eq!(subsystem.get_priority(), 10);
        assert!(subsystem.is_enabled());
        assert!(subsystem.get_config().is_empty());
        assert!(subsystem.get_data().is_empty());
        
        // Test setting configuration
        let mut config = HashMap::new();
        config.insert("damage_multiplier".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(1.5).unwrap()));
        subsystem.set_config(config);
        assert!(!subsystem.get_config().is_empty());
        
        // Test setting data
        let mut data = HashMap::new();
        data.insert("last_attack".to_string(), serde_json::Value::String("2023-01-01".to_string()));
        subsystem.set_data(data);
        assert!(!subsystem.get_data().is_empty());
        
        // Test setting enabled status
        subsystem.set_enabled(false);
        assert!(!subsystem.is_enabled());
    }

    // === SubsystemOutput Tests ===

    #[tokio::test]
    async fn test_subsystem_output_creation_and_management() {
        let mut output = SubsystemOutput::new("combat".to_string());
        
        // Test initial state
        assert!(output.primary.is_empty());
        assert!(output.derived.is_empty());
        assert!(output.caps.is_empty());
        assert!(output.context.is_none());
        assert_eq!(output.meta.system, "combat");
        
        // Test adding contributions
        let contribution = Contribution::new("strength".to_string(), Bucket::Flat, 10.0, "combat".to_string());
        output.add_primary(contribution);
        assert_eq!(output.primary.len(), 1);
        
        let derived_contribution = Contribution::new("damage".to_string(), Bucket::Mult, 1.2, "combat".to_string());
        output.add_derived(derived_contribution);
        assert_eq!(output.derived.len(), 1);
        
        let cap_contribution = CapContribution::new("combat".to_string(), "strength".to_string(), CapMode::HardMax, "max".to_string(), 100.0);
        output.add_cap(cap_contribution);
        assert_eq!(output.caps.len(), 1);
    }

    // === SubsystemMeta Tests ===

    #[tokio::test]
    async fn test_subsystem_meta_creation() {
        let meta = SubsystemMeta::new("combat".to_string());
        
        assert_eq!(meta.system, "combat");
        assert!(meta.data.is_empty());
    }

    // === ModifierPack Tests ===

    #[tokio::test]
    async fn test_modifier_pack_creation_and_application() {
        let mut pack = ModifierPack::new();
        
        // Test initial state
        assert!(pack.additive_percent.is_empty());
        assert!(pack.multipliers.is_empty());
        assert!(pack.post_add.is_empty());
        
        // Test default implementation
        let default_pack = ModifierPack::default();
        assert!(default_pack.additive_percent.is_empty());
        
        // Test adding modifiers
        pack.additive_percent.insert("strength".to_string(), 50.0);
        pack.multipliers.insert("strength".to_string(), 1.5);
        pack.post_add.insert("strength".to_string(), 10.0);
        
        // Test applying modifiers
        let base_value = 100.0;
        let result = pack.apply("strength", base_value);
        
        // Expected: (100 + 100 * 0.5) * 1.5 + 10 = 150 * 1.5 + 10 = 235.0
        assert_eq!(result, 235.0);
        
        // Test applying to non-existent dimension
        let result = pack.apply("nonexistent", base_value);
        assert_eq!(result, base_value);
    }

    #[tokio::test]
    async fn test_modifier_pack_edge_cases() {
        let mut pack = ModifierPack::new();
        
        // Test with zero values
        pack.additive_percent.insert("test".to_string(), 0.0);
        pack.multipliers.insert("test".to_string(), 1.0);
        pack.post_add.insert("test".to_string(), 0.0);
        
        let result = pack.apply("test", 100.0);
        assert_eq!(result, 100.0);
        
        // Test with negative values
        pack.additive_percent.insert("negative".to_string(), -25.0);
        pack.multipliers.insert("negative".to_string(), 0.5);
        pack.post_add.insert("negative".to_string(), -10.0);
        
        let result = pack.apply("negative", 100.0);
        // Expected: (100 + 100 * -0.25) * 0.5 + (-10) = 75 * 0.5 - 10 = 27.5
        assert_eq!(result, 27.5);
    }

    // === Snapshot Tests ===

    #[tokio::test]
    async fn test_snapshot_creation_and_validation() {
        let actor_id = Uuid::new_v4();
        let snapshot = Snapshot::new(actor_id, 1);
        
        assert_eq!(snapshot.actor_id, actor_id);
        assert_eq!(snapshot.version, 1);
        assert!(snapshot.is_valid());
        assert!(snapshot.primary.is_empty());
        assert!(snapshot.derived.is_empty());
        assert!(snapshot.caps_used.is_empty());
        assert!(snapshot.subsystems_processed.is_empty());
        assert!(snapshot.processing_time.is_none());
        assert!(snapshot.metadata.is_empty());
        
        // Test invalid snapshot
        let invalid_snapshot = Snapshot {
            actor_id,
            primary: HashMap::new(),
            derived: HashMap::new(),
            caps_used: HashMap::new(),
            version: 0, // Invalid version
            created_at: Utc::now(),
            subsystems_processed: Vec::new(),
            processing_time: None,
            metadata: HashMap::new(),
        };
        assert!(!invalid_snapshot.is_valid());
    }

    #[tokio::test]
    async fn test_snapshot_stat_operations() {
        let actor_id = Uuid::new_v4();
        let mut snapshot = Snapshot::new(actor_id, 1);
        
        // Test adding primary contribution
        let contribution = Contribution::new("strength".to_string(), Bucket::Flat, 10.0, "combat".to_string());
        snapshot.add_primary(contribution);
        assert_eq!(snapshot.get_primary("strength"), Some(10.0));
        
        // Test adding derived contribution
        let derived_contribution = Contribution::new("damage".to_string(), Bucket::Flat, 1.2, "combat".to_string());
        snapshot.add_derived(derived_contribution);
        assert_eq!(snapshot.get_derived("damage"), Some(1.2));
        
        // Test adding cap contribution
        let cap_contribution = CapContribution::new("combat".to_string(), "strength".to_string(), CapMode::HardMax, "max".to_string(), 100.0);
        snapshot.add_cap(cap_contribution);
        assert!(snapshot.get_caps("strength").is_some());
        
        // Test non-existent stats
        assert_eq!(snapshot.get_primary("nonexistent"), None);
        assert_eq!(snapshot.get_derived("nonexistent"), None);
        assert_eq!(snapshot.get_caps("nonexistent"), None);
    }

    #[tokio::test]
    async fn test_snapshot_bucket_operations() {
        let actor_id = Uuid::new_v4();
        let mut snapshot = Snapshot::new(actor_id, 1);
        
        // Test Flat bucket
        let flat_contribution = Contribution::new("strength".to_string(), Bucket::Flat, 10.0, "combat".to_string());
        snapshot.add_primary(flat_contribution);
        assert_eq!(snapshot.get_primary("strength"), Some(10.0));
        
        // Test Mult bucket
        let mult_contribution = Contribution::new("strength".to_string(), Bucket::Mult, 1.5, "combat".to_string());
        snapshot.add_primary(mult_contribution);
        assert_eq!(snapshot.get_primary("strength"), Some(15.0)); // 10 * 1.5
        
        // Test PostAdd bucket
        let postadd_contribution = Contribution::new("strength".to_string(), Bucket::PostAdd, 5.0, "combat".to_string());
        snapshot.add_primary(postadd_contribution);
        assert_eq!(snapshot.get_primary("strength"), Some(20.0)); // 15 + 5
        
        // Test Override bucket
        let override_contribution = Contribution::new("strength".to_string(), Bucket::Override, 100.0, "combat".to_string());
        snapshot.add_primary(override_contribution);
        assert_eq!(snapshot.get_primary("strength"), Some(100.0)); // Override
    }

    #[tokio::test]
    async fn test_snapshot_cap_mode_operations() {
        let actor_id = Uuid::new_v4();
        let mut snapshot = Snapshot::new(actor_id, 1);
        
        // Test Baseline mode
        let baseline_cap = CapContribution::new("combat".to_string(), "strength".to_string(), CapMode::Baseline, "max".to_string(), 50.0);
        snapshot.add_cap(baseline_cap);
        let caps = snapshot.get_caps("strength").unwrap();
        assert_eq!(caps.get_min(), 50.0);
        assert_eq!(caps.get_max(), 50.0);
        
        // Test Additive mode
        let additive_cap = CapContribution::new("combat".to_string(), "strength".to_string(), CapMode::Additive, "max".to_string(), 25.0);
        snapshot.add_cap(additive_cap);
        let caps = snapshot.get_caps("strength").unwrap();
        assert_eq!(caps.get_min(), 25.0); // 50 - 25
        assert_eq!(caps.get_max(), 75.0); // 50 + 25
        
        // Test HardMax mode
        let hardmax_cap = CapContribution::new("combat".to_string(), "strength".to_string(), CapMode::HardMax, "max".to_string(), 100.0);
        snapshot.add_cap(hardmax_cap);
        let caps = snapshot.get_caps("strength").unwrap();
        assert_eq!(caps.get_max(), 100.0);
        
        // Test HardMin mode
        let hardmin_cap = CapContribution::new("combat".to_string(), "strength".to_string(), CapMode::HardMin, "min".to_string(), 10.0);
        snapshot.add_cap(hardmin_cap);
        let caps = snapshot.get_caps("strength").unwrap();
        assert_eq!(caps.get_min(), 10.0);
        
        // Test Override mode
        let override_cap = CapContribution::new("combat".to_string(), "strength".to_string(), CapMode::Override, "max".to_string(), 200.0);
        snapshot.add_cap(override_cap);
        let caps = snapshot.get_caps("strength").unwrap();
        assert_eq!(caps.get_min(), 200.0);
        assert_eq!(caps.get_max(), 200.0);
        
        // Test SoftMax mode
        let softmax_cap = CapContribution::new("combat".to_string(), "strength".to_string(), CapMode::SoftMax, "max".to_string(), 150.0);
        snapshot.add_cap(softmax_cap);
        let caps = snapshot.get_caps("strength").unwrap();
        assert_eq!(caps.get_max(), 150.0);
    }

    #[tokio::test]
    async fn test_snapshot_clone_with_new_timestamp() {
        let actor_id = Uuid::new_v4();
        let snapshot = Snapshot::new(actor_id, 1);
        let original_time = snapshot.created_at;
        
        // Wait a small amount to ensure timestamp difference
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        
        let cloned_snapshot = snapshot.clone_with_new_timestamp();
        assert_eq!(cloned_snapshot.actor_id, snapshot.actor_id);
        assert_eq!(cloned_snapshot.version, snapshot.version);
        assert!(cloned_snapshot.created_at > original_time);
    }

    // === Caps Tests ===

    #[tokio::test]
    async fn test_caps_creation_and_validation() {
        // Test valid caps
        let caps = Caps::new(0.0, 100.0);
        assert!(caps.is_valid());
        assert_eq!(caps.get_min(), 0.0);
        assert_eq!(caps.get_max(), 100.0);
        
        // Test caps with equal min and max
        let caps = Caps::new(50.0, 50.0);
        assert!(caps.is_valid());
        
        // Test invalid caps (min > max)
        let caps = Caps::new(100.0, 0.0);
        assert!(!caps.is_valid());
        
        // Test caps with infinity
        let caps = Caps::new(f64::INFINITY, 100.0);
        assert!(!caps.is_valid());
        
        let caps = Caps::new(0.0, f64::NEG_INFINITY);
        assert!(!caps.is_valid());
        
        // Test caps with NaN
        let caps = Caps::new(f64::NAN, 100.0);
        assert!(!caps.is_valid());
    }

    #[tokio::test]
    async fn test_caps_containment_operations() {
        let caps = Caps::new(0.0, 100.0);
        
        // Test values within range
        assert!(caps.contains(0.0));
        assert!(caps.contains(50.0));
        assert!(caps.contains(100.0));
        
        // Test values outside range
        assert!(!caps.contains(-1.0));
        assert!(!caps.contains(101.0));
        
        // Test empty caps
        let empty_caps = Caps::new(10.0, 5.0);
        assert!(empty_caps.is_empty());
        assert!(!empty_caps.contains(7.0));
    }

    #[tokio::test]
    async fn test_caps_clamping_operations() {
        let caps = Caps::new(0.0, 100.0);
        
        // Test values within range
        assert_eq!(caps.clamp(50.0), 50.0);
        assert_eq!(caps.clamp(0.0), 0.0);
        assert_eq!(caps.clamp(100.0), 100.0);
        
        // Test values outside range
        assert_eq!(caps.clamp(-10.0), 0.0);
        assert_eq!(caps.clamp(150.0), 100.0);
    }

    #[tokio::test]
    async fn test_caps_intersection_operations() {
        let caps1 = Caps::new(0.0, 100.0);
        let caps2 = Caps::new(50.0, 150.0);
        
        let intersection = caps1.intersection(&caps2);
        assert_eq!(intersection.get_min(), 50.0);
        assert_eq!(intersection.get_max(), 100.0);
        
        // Test non-overlapping caps
        let caps3 = Caps::new(200.0, 300.0);
        let intersection = caps1.intersection(&caps3);
        assert!(intersection.is_empty());
    }

    #[tokio::test]
    async fn test_caps_union_operations() {
        let caps1 = Caps::new(0.0, 100.0);
        let caps2 = Caps::new(50.0, 150.0);
        
        let union = caps1.union(&caps2);
        assert_eq!(union.get_min(), 0.0);
        assert_eq!(union.get_max(), 150.0);
    }

    #[tokio::test]
    async fn test_caps_range_operations() {
        let caps = Caps::new(10.0, 90.0);
        
        assert_eq!(caps.get_range(), 80.0);
        assert_eq!(caps.get_center(), 50.0);
    }

    #[tokio::test]
    async fn test_caps_expansion_operations() {
        let mut caps = Caps::new(50.0, 100.0);
        
        // Test expansion
        caps.expand(10.0);
        assert_eq!(caps.get_min(), 40.0);
        assert_eq!(caps.get_max(), 110.0);
        
        // Test shrinking
        caps.shrink(5.0);
        assert_eq!(caps.get_min(), 45.0);
        assert_eq!(caps.get_max(), 105.0);
        
        // Test shrinking beyond limits
        caps.shrink(100.0);
        assert_eq!(caps.get_min(), caps.get_max()); // Should be equal when over-shrunk
    }

    #[tokio::test]
    async fn test_caps_setter_operations() {
        let mut caps = Caps::new(0.0, 100.0);
        
        // Test setting both values
        caps.set(25.0, 75.0);
        assert_eq!(caps.get_min(), 25.0);
        assert_eq!(caps.get_max(), 75.0);
        
        // Test setting individual values
        caps.set_min(10.0);
        assert_eq!(caps.get_min(), 10.0);
        
        caps.set_max(90.0);
        assert_eq!(caps.get_max(), 90.0);
    }

    // === Edge Cases and Error Conditions ===

    #[tokio::test]
    async fn test_actor_edge_cases() {
        // Test actor with extreme values
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        actor.set_lifespan(i64::MAX);
        actor.set_age(i64::MAX);
        assert!(actor.is_valid());
        
        // Test actor with negative values
        actor.set_lifespan(-1);
        actor.set_age(-1);
        assert!(actor.is_valid()); // These are allowed
        
        // Test actor with very long strings
        let long_name = "a".repeat(10000);
        let long_race = "b".repeat(10000);
        let actor = Actor::new(long_name, long_race);
        assert!(actor.is_valid());
    }

    #[tokio::test]
    async fn test_contribution_edge_cases() {
        // Test contribution with extreme values
        let contribution = Contribution::new(
            "strength".to_string(),
            Bucket::Flat,
            f64::MAX,
            "combat".to_string()
        );
        assert!(contribution.is_valid());
        
        // Test contribution with very small values
        let contribution = Contribution::new(
            "strength".to_string(),
            Bucket::Flat,
            f64::MIN_POSITIVE,
            "combat".to_string()
        );
        assert!(contribution.is_valid());
        
        // Test contribution with infinity
        let contribution = Contribution::new(
            "strength".to_string(),
            Bucket::Flat,
            f64::INFINITY,
            "combat".to_string()
        );
        assert!(!contribution.is_valid());
    }

    #[tokio::test]
    async fn test_snapshot_edge_cases() {
        let actor_id = Uuid::new_v4();
        let mut snapshot = Snapshot::new(actor_id, 1);
        
        // Test with extreme values
        let extreme_contribution = Contribution::new(
            "strength".to_string(),
            Bucket::Flat,
            f64::MAX,
            "combat".to_string()
        );
        snapshot.add_primary(extreme_contribution);
        assert_eq!(snapshot.get_primary("strength"), Some(f64::MAX));
        
        // Test with zero values
        let zero_contribution = Contribution::new(
            "strength".to_string(),
            Bucket::Flat,
            0.0,
            "combat".to_string()
        );
        snapshot.add_primary(zero_contribution);
        assert_eq!(snapshot.get_primary("strength"), Some(f64::MAX)); // Previous value + 0
    }

    #[tokio::test]
    async fn test_caps_edge_cases() {
        // Test caps with extreme values
        let caps = Caps::new(f64::MIN, f64::MAX);
        assert!(caps.is_valid());
        
        // Test caps with very small range
        let caps = Caps::new(0.0, f64::MIN_POSITIVE);
        assert!(caps.is_valid());
        
        // Test caps with negative values
        let caps = Caps::new(-100.0, -50.0);
        assert!(caps.is_valid());
        assert_eq!(caps.clamp(-75.0), -75.0);
        assert_eq!(caps.clamp(-150.0), -100.0);
        assert_eq!(caps.clamp(-25.0), -50.0);
    }

    // === Performance Tests ===

    #[tokio::test]
    async fn test_actor_performance_many_subsystems() {
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Add many subsystems
        for i in 0..1000 {
            let subsystem = Subsystem::new(format!("subsystem_{}", i), i as i64);
            actor.add_subsystem(subsystem);
        }
        
        assert_eq!(actor.get_subsystem_count(), 1000);
        assert!(actor.has_subsystem("subsystem_500"));
        assert!(!actor.has_subsystem("subsystem_1000"));
        
        // Test priority ordering performance
        let ordered = actor.get_subsystem_by_priority();
        assert_eq!(ordered.len(), 1000);
        // First should be highest priority (999)
        assert_eq!(ordered[0].get_priority(), 999);
    }

    #[tokio::test]
    async fn test_actor_performance_many_buffs() {
        let mut actor = Actor::new("TestActor".to_string(), "Human".to_string());
        
        // Add many buffs
        for i in 0..1000 {
            actor.add_buff(format!("buff_{}", i));
        }
        
        assert_eq!(actor.get_active_buffs().len(), 1000);
        assert!(actor.has_buff("buff_500"));
        assert!(!actor.has_buff("buff_1000"));
        
        // Test removing buffs
        for i in 0..500 {
            actor.remove_buff(&format!("buff_{}", i));
        }
        
        assert_eq!(actor.get_active_buffs().len(), 500);
    }

    #[tokio::test]
    async fn test_snapshot_performance_many_contributions() {
        let actor_id = Uuid::new_v4();
        let mut snapshot = Snapshot::new(actor_id, 1);
        
        // Add many contributions
        for i in 0..1000 {
            let contribution = Contribution::new(
                format!("stat_{}", i),
                Bucket::Flat,
                i as f64,
                "combat".to_string()
            );
            snapshot.add_primary(contribution);
        }
        
        assert_eq!(snapshot.primary.len(), 1000);
        assert_eq!(snapshot.get_primary("stat_500"), Some(500.0));
    }
}
