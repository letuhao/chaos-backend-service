//! Integration tests for the Actor Core system.
//!
//! This module contains end-to-end tests that verify the complete
//! stat aggregation pipeline works correctly.

#[cfg(test)]
mod integration_tests {
    use crate::types::{Actor, Subsystem as SubsystemStruct, Contribution, Caps, Snapshot};
    use crate::enums::{Bucket, CapMode};

    #[tokio::test]
    async fn test_basic_actor_operations() {
        // Create test actor
        let mut actor = Actor::new("TestWarrior".to_string(), "Human".to_string());
        actor.set_lifespan(100);
        actor.set_age(25);

        // Add combat subsystem
        let combat_subsystem = SubsystemStruct::new("combat".to_string(), 10);
        actor.add_subsystem(combat_subsystem);

        // Verify actor properties
        assert_eq!(actor.get_name(), "TestWarrior");
        assert_eq!(actor.get_race(), "Human");
        assert_eq!(actor.get_lifespan(), 100);
        assert_eq!(actor.get_age(), 25);
        assert!(actor.is_valid());

        // Verify subsystem was added
        assert_eq!(actor.get_subsystem_count(), 1);
        assert!(actor.has_subsystem("combat"));
        assert!(!actor.has_subsystem("magic"));

        // Test subsystem retrieval
        let subsystem = actor.get_subsystem("combat");
        assert!(subsystem.is_some());
        let subsystem = subsystem.unwrap();
        assert_eq!(subsystem.get_system_id(), "combat");
        assert_eq!(subsystem.get_priority(), 10);

        println!("✅ Basic actor operations test passed!");
    }

    #[tokio::test]
    async fn test_contribution_aggregation() {
        // Create test contributions
        let mut contributions = Vec::new();
        
        // Add strength contribution
        contributions.push(Contribution::new(
            "strength".to_string(),
            Bucket::Flat,
            10.0,
            "combat".to_string(),
        ));

        // Add health contribution
        contributions.push(Contribution::new(
            "health".to_string(),
            Bucket::Flat,
            100.0,
            "combat".to_string(),
        ));

        // Add multiplicative contribution
        contributions.push(Contribution::new(
            "attack_power".to_string(),
            Bucket::Mult,
            1.5,
            "combat".to_string(),
        ));

        // Verify contributions
        assert_eq!(contributions.len(), 3);
        assert_eq!(contributions[0].dimension, "strength");
        assert_eq!(contributions[0].value, 10.0);
        assert_eq!(contributions[0].bucket, Bucket::Flat);
        assert_eq!(contributions[1].dimension, "health");
        assert_eq!(contributions[1].value, 100.0);
        assert_eq!(contributions[2].dimension, "attack_power");
        assert_eq!(contributions[2].bucket, Bucket::Mult);

        println!("✅ Contribution aggregation test passed!");
    }

    #[tokio::test]
    async fn test_caps_operations() {
        // Create test caps
        let mut caps = Caps::new(0.0, 100.0);
        
        // Test basic operations
        assert_eq!(caps.get_min(), 0.0);
        assert_eq!(caps.get_max(), 100.0);
        assert_eq!(caps.get_range(), 100.0);
        assert_eq!(caps.get_center(), 50.0);
        assert!(!caps.is_empty());
        assert!(caps.is_valid());

        // Test clamping
        assert_eq!(caps.clamp(50.0), 50.0);
        assert_eq!(caps.clamp(150.0), 100.0);
        assert_eq!(caps.clamp(-10.0), 0.0);

        // Test contains
        assert!(caps.contains(50.0));
        assert!(caps.contains(0.0));
        assert!(caps.contains(100.0));
        assert!(!caps.contains(150.0));
        assert!(!caps.contains(-10.0));

        // Test modifications
        caps.expand(10.0);
        assert_eq!(caps.get_min(), -10.0);
        assert_eq!(caps.get_max(), 110.0);

        caps.shrink(20.0);
        assert_eq!(caps.get_min(), 10.0);
        assert_eq!(caps.get_max(), 90.0);

        // Test intersection and union
        let caps2 = Caps::new(25.0, 75.0);
        let intersection = caps.intersection(&caps2);
        assert_eq!(intersection.get_min(), 25.0);
        assert_eq!(intersection.get_max(), 75.0);

        let union = caps.union(&caps2);
        assert_eq!(union.get_min(), 10.0);
        assert_eq!(union.get_max(), 90.0);

        println!("✅ Caps operations test passed!");
    }

    #[tokio::test]
    async fn test_snapshot_operations() {
        // Create test actor
        let actor = Actor::new("TestMage".to_string(), "Elf".to_string());
        
        // Create snapshot
        let mut snapshot = Snapshot::new(actor.id, actor.version);
        
        // Add primary contributions
        let strength_contrib = Contribution::new(
            "strength".to_string(),
            Bucket::Flat,
            15.0,
            "combat".to_string(),
        );
        snapshot.add_primary(strength_contrib);

        let health_contrib = Contribution::new(
            "health".to_string(),
            Bucket::Flat,
            120.0,
            "combat".to_string(),
        );
        snapshot.add_primary(health_contrib);

        // Add derived contributions
        let attack_power_contrib = Contribution::new(
            "attack_power".to_string(),
            Bucket::Mult,
            1.2,
            "combat".to_string(),
        );
        snapshot.add_derived(attack_power_contrib);

        // Add caps
        let strength_cap = crate::types::CapContribution::new(
            "combat".to_string(),
            "strength".to_string(),
            CapMode::HardMax,
            "combat".to_string(),
            50.0,
        );
        snapshot.add_cap(strength_cap);

        // Verify snapshot
        assert_eq!(snapshot.actor_id, actor.id);
        assert_eq!(snapshot.version, actor.version);
        assert!(snapshot.is_valid());

        // Verify primary stats
        assert_eq!(snapshot.get_primary("strength"), Some(15.0));
        assert_eq!(snapshot.get_primary("health"), Some(120.0));
        assert_eq!(snapshot.get_primary("mana"), None);

        // Verify derived stats (mult bucket starts with 0.0, so 0.0 * 1.2 = 0.0)
        assert_eq!(snapshot.get_derived("attack_power"), Some(0.0));
        assert_eq!(snapshot.get_derived("magic_power"), None);

        // Verify caps
        assert!(snapshot.get_caps("strength").is_some());
        let strength_caps = snapshot.get_caps("strength").unwrap();
        assert_eq!(strength_caps.get_max(), 50.0);

        println!("✅ Snapshot operations test passed!");
    }

    #[tokio::test]
    async fn test_actor_combat_system() {
        // Create test actor
        let mut actor = Actor::new("TestFighter".to_string(), "Orc".to_string());
        
        // Test combat status
        assert!(!actor.is_in_combat());
        actor.set_in_combat(true);
        assert!(actor.is_in_combat());
        actor.set_in_combat(false);
        assert!(!actor.is_in_combat());

        // Test buff system
        assert!(!actor.has_buff("strength_boost"));
        actor.add_buff("strength_boost".to_string());
        assert!(actor.has_buff("strength_boost"));
        assert!(!actor.has_buff("speed_boost"));

        actor.add_buff("speed_boost".to_string());
        assert!(actor.has_buff("speed_boost"));

        let active_buffs = actor.get_active_buffs();
        assert_eq!(active_buffs.len(), 2);
        assert!(active_buffs.contains(&"strength_boost"));
        assert!(active_buffs.contains(&"speed_boost"));

        // Test buff removal
        assert!(actor.remove_buff("strength_boost"));
        assert!(!actor.has_buff("strength_boost"));
        assert!(actor.has_buff("speed_boost"));

        // Test buff clearing
        actor.clear_buffs();
        assert!(!actor.has_buff("speed_boost"));
        assert!(actor.get_active_buffs().is_empty());

        println!("✅ Actor combat system test passed!");
    }

    #[tokio::test]
    async fn test_actor_guild_system() {
        // Create test actor
        let mut actor = Actor::new("TestGuildMember".to_string(), "Human".to_string());
        
        // Test guild membership
        assert!(!actor.is_guild_member());
        assert!(actor.get_guild_id().is_none());

        // Join guild
        actor.set_guild_id("guild_123".to_string());
        assert!(actor.is_guild_member());
        assert_eq!(actor.get_guild_id(), Some("guild_123"));

        // Leave guild (set to None by removing the key)
        actor.data.remove("guild_id");
        assert!(!actor.is_guild_member());
        assert!(actor.get_guild_id().is_none());

        println!("✅ Actor guild system test passed!");
    }

    #[tokio::test]
    async fn test_actor_version_management() {
        // Create test actor
        let mut actor = Actor::new("TestVersioned".to_string(), "Human".to_string());
        let initial_version = actor.get_version();
        
        // Test touch() - should increment version and update timestamp
        let before_touch = actor.get_updated_at();
        actor.touch();
        let after_touch = actor.get_updated_at();
        
        assert_eq!(actor.get_version(), initial_version + 1);
        assert!(after_touch > before_touch);
        
        // Test update_version() - should only increment version
        let version_before = actor.get_version();
        let timestamp_before = actor.get_updated_at();
        actor.update_version();
        let timestamp_after = actor.get_updated_at();
        
        assert_eq!(actor.get_version(), version_before + 1);
        assert_eq!(timestamp_after, timestamp_before); // Should not change

        println!("✅ Actor version management test passed!");
    }

    #[tokio::test]
    async fn test_comprehensive_actor_lifecycle() {
        // Create actor
        let mut actor = Actor::new("TestHero".to_string(), "Elf".to_string());
        actor.set_lifespan(200);
        actor.set_age(50);

        // Add multiple subsystems
        let combat_subsystem = SubsystemStruct::new("combat".to_string(), 10);
        let magic_subsystem = SubsystemStruct::new("magic".to_string(), 15);
        let stealth_subsystem = SubsystemStruct::new("stealth".to_string(), 5);

        actor.add_subsystem(combat_subsystem);
        actor.add_subsystem(magic_subsystem);
        actor.add_subsystem(stealth_subsystem);

        // Verify all subsystems
        assert_eq!(actor.get_subsystem_count(), 3);
        assert!(actor.has_subsystem("combat"));
        assert!(actor.has_subsystem("magic"));
        assert!(actor.has_subsystem("stealth"));

        // Test subsystem priority ordering
        let subsystems_by_priority = actor.get_subsystem_by_priority();
        assert_eq!(subsystems_by_priority.len(), 3);
        // Should be ordered by priority (highest first)
        assert_eq!(subsystems_by_priority[0].get_system_id(), "magic");
        assert_eq!(subsystems_by_priority[1].get_system_id(), "combat");
        assert_eq!(subsystems_by_priority[2].get_system_id(), "stealth");

        // Remove a subsystem
        assert!(actor.remove_subsystem("stealth"));
        assert_eq!(actor.get_subsystem_count(), 2);
        assert!(!actor.has_subsystem("stealth"));

        // Test combat and buffs
        actor.set_in_combat(true);
        actor.add_buff("battle_rage".to_string());
        actor.add_buff("magic_armor".to_string());

        assert!(actor.is_in_combat());
        assert!(actor.has_buff("battle_rage"));
        assert!(actor.has_buff("magic_armor"));

        // Test guild membership
        actor.set_guild_id("heroes_guild".to_string());
        assert!(actor.is_guild_member());
        assert_eq!(actor.get_guild_id(), Some("heroes_guild"));

        // Verify final state
        assert!(actor.is_valid());
        assert_eq!(actor.get_name(), "TestHero");
        assert_eq!(actor.get_race(), "Elf");
        assert_eq!(actor.get_lifespan(), 200);
        assert_eq!(actor.get_age(), 50);

        println!("✅ Comprehensive actor lifecycle test passed!");
        println!("   Final actor state: {} ({}), {} years old", 
                actor.get_name(), actor.get_race(), actor.get_age());
        println!("   Subsystems: {}", actor.get_subsystem_count());
        println!("   In combat: {}", actor.is_in_combat());
        println!("   Active buffs: {:?}", actor.get_active_buffs());
        println!("   Guild: {:?}", actor.get_guild_id());
    }
}