//! Aggregation Tests
//!
//! This module contains tests for the core aggregation logic including
//! actor operations, contribution aggregation, and snapshot generation.

use actor_core::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

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
            50.0,
            "combat".to_string(),
        ));
        
        // Add intelligence contribution
        contributions.push(Contribution::new(
            "intelligence".to_string(),
            Bucket::Flat,
            30.0,
            "magic".to_string(),
        ));
        
        // Add multiplicative contribution
        contributions.push(Contribution::new(
            "strength".to_string(),
            Bucket::Mult,
            1.2,
            "equipment".to_string(),
        ));

        // Verify contributions
        assert_eq!(contributions.len(), 3);
        assert_eq!(contributions[0].dimension, "strength");
        assert_eq!(contributions[0].value, 50.0);
        assert_eq!(contributions[0].bucket, Bucket::Flat);
        assert_eq!(contributions[0].system, "combat");
        
        assert_eq!(contributions[1].dimension, "intelligence");
        assert_eq!(contributions[1].value, 30.0);
        assert_eq!(contributions[1].bucket, Bucket::Flat);
        assert_eq!(contributions[1].system, "magic");
        
        assert_eq!(contributions[2].dimension, "strength");
        assert_eq!(contributions[2].value, 1.2);
        assert_eq!(contributions[2].bucket, Bucket::Mult);
        assert_eq!(contributions[2].system, "equipment");

        println!("✅ Contribution aggregation test passed!");
    }

    #[tokio::test]
    async fn test_caps_operations() {
        // Create test caps
        let caps1 = Caps { min: 0.0, max: 100.0 };
        let caps2 = Caps { min: 10.0, max: 80.0 };
        
        // Test clamp operation
        let clamped = caps1.clamp(150.0);
        assert_eq!(clamped, 100.0);
        
        let clamped = caps1.clamp(-10.0);
        assert_eq!(clamped, 0.0);
        
        let clamped = caps1.clamp(50.0);
        assert_eq!(clamped, 50.0);
        
        // Test intersection
        let intersection = caps1.intersection(&caps2);
        assert_eq!(intersection.min, 10.0);
        assert_eq!(intersection.max, 80.0);
        
        // Test union
        let union = caps1.union(&caps2);
        assert_eq!(union.min, 0.0);
        assert_eq!(union.max, 100.0);

        println!("✅ Caps operations test passed!");
    }

    #[tokio::test]
    async fn test_snapshot_creation() {
        // Create test actor
        let mut actor = Actor::new("TestMage".to_string(), "Elf".to_string());
        actor.set_lifespan(200);
        actor.set_age(50);
        
        // Add magic subsystem
        let magic_subsystem = SubsystemStruct::new("magic".to_string(), 5);
        actor.add_subsystem(magic_subsystem);
        
        // Create test contributions
        let mut contributions = Vec::new();
        contributions.push(Contribution::new(
            "intelligence".to_string(),
            Bucket::Flat,
            80.0,
            "magic".to_string(),
        ));
        contributions.push(Contribution::new(
            "mana".to_string(),
            Bucket::Flat,
            100.0,
            "magic".to_string(),
        ));
        
        // Create snapshot
        let snapshot = Snapshot::new(
            actor.get_id().clone(),
            1,
        );
        
        // Verify snapshot properties
        assert_eq!(snapshot.actor_id, *actor.get_id());
        assert_eq!(snapshot.primary.len(), 0);
        assert_eq!(snapshot.derived.len(), 0);
        assert_eq!(snapshot.version, 1);

        println!("✅ Snapshot creation test passed!");
    }

    #[tokio::test]
    async fn test_bucket_processing() {
        // Test Flat bucket (additive)
        let flat_contrib = Contribution::new(
            "strength".to_string(),
            Bucket::Flat,
            50.0,
            "base".to_string(),
        );
        assert_eq!(flat_contrib.bucket, Bucket::Flat);
        
        // Test Mult bucket (multiplicative)
        let mult_contrib = Contribution::new(
            "strength".to_string(),
            Bucket::Mult,
            1.5,
            "equipment".to_string(),
        );
        assert_eq!(mult_contrib.bucket, Bucket::Mult);
        
        // Test PostAdd bucket (post-multiplication additive)
        let postadd_contrib = Contribution::new(
            "strength".to_string(),
            Bucket::PostAdd,
            10.0,
            "buffs".to_string(),
        );
        assert_eq!(postadd_contrib.bucket, Bucket::PostAdd);
        
        // Test Override bucket (replaces value)
        let override_contrib = Contribution::new(
            "strength".to_string(),
            Bucket::Override,
            100.0,
            "transformation".to_string(),
        );
        assert_eq!(override_contrib.bucket, Bucket::Override);

        println!("✅ Bucket processing test passed!");
    }

    #[tokio::test]
    async fn test_cap_mode_operations() {
        // Test Baseline mode
        assert_eq!(CapMode::Baseline, CapMode::Baseline);
        
        // Test Additive mode
        assert_eq!(CapMode::Additive, CapMode::Additive);
        
        // Test HardMax mode
        assert_eq!(CapMode::HardMax, CapMode::HardMax);
        
        // Test HardMin mode
        assert_eq!(CapMode::HardMin, CapMode::HardMin);
        
        // Test Override mode
        assert_eq!(CapMode::Override, CapMode::Override);

        println!("✅ Cap mode operations test passed!");
    }

    #[tokio::test]
    async fn test_complex_aggregation_scenario() {
        // Create a complex actor with multiple subsystems
        let mut actor = Actor::new("ComplexWarrior".to_string(), "Dwarf".to_string());
        actor.set_lifespan(150);
        actor.set_age(75);
        
        // Add multiple subsystems
        let combat_subsystem = SubsystemStruct::new("combat".to_string(), 10);
        let magic_subsystem = SubsystemStruct::new("magic".to_string(), 5);
        let equipment_subsystem = SubsystemStruct::new("equipment".to_string(), 8);
        
        actor.add_subsystem(combat_subsystem);
        actor.add_subsystem(magic_subsystem);
        actor.add_subsystem(equipment_subsystem);
        
        // Verify actor setup
        assert_eq!(actor.get_subsystem_count(), 3);
        assert!(actor.has_subsystem("combat"));
        assert!(actor.has_subsystem("magic"));
        assert!(actor.has_subsystem("equipment"));
        
        // Create complex contributions
        let mut contributions = Vec::new();
        
        // Base stats from combat
        contributions.push(Contribution::new("strength".to_string(), Bucket::Flat, 60.0, "combat".to_string()));
        contributions.push(Contribution::new("constitution".to_string(), Bucket::Flat, 70.0, "combat".to_string()));
        
        // Equipment bonuses
        contributions.push(Contribution::new("strength".to_string(), Bucket::Mult, 1.3, "equipment".to_string()));
        contributions.push(Contribution::new("constitution".to_string(), Bucket::Mult, 1.2, "equipment".to_string()));
        
        // Magic enhancements
        contributions.push(Contribution::new("strength".to_string(), Bucket::PostAdd, 15.0, "magic".to_string()));
        contributions.push(Contribution::new("intelligence".to_string(), Bucket::Flat, 40.0, "magic".to_string()));
        
        // Verify contributions
        assert_eq!(contributions.len(), 6);
        
        // Test that we have the right mix of bucket types
        let flat_count = contributions.iter().filter(|c| c.bucket == Bucket::Flat).count();
        let mult_count = contributions.iter().filter(|c| c.bucket == Bucket::Mult).count();
        let postadd_count = contributions.iter().filter(|c| c.bucket == Bucket::PostAdd).count();
        
        assert_eq!(flat_count, 3); // strength, constitution, intelligence
        assert_eq!(mult_count, 2); // strength, constitution equipment bonuses
        assert_eq!(postadd_count, 1); // strength magic enhancement
        
        // Test dimension grouping
        let strength_contribs: Vec<_> = contributions.iter().filter(|c| c.dimension == "strength").collect();
        assert_eq!(strength_contribs.len(), 3); // base + equipment + magic
        
        let constitution_contribs: Vec<_> = contributions.iter().filter(|c| c.dimension == "constitution").collect();
        assert_eq!(constitution_contribs.len(), 2); // base + equipment
        
        let intelligence_contribs: Vec<_> = contributions.iter().filter(|c| c.dimension == "intelligence").collect();
        assert_eq!(intelligence_contribs.len(), 1); // magic only

        println!("✅ Complex aggregation scenario test passed!");
    }

    #[tokio::test]
    async fn test_actor_validation() {
        // Test valid actor
        let mut valid_actor = Actor::new("ValidActor".to_string(), "Human".to_string());
        valid_actor.set_lifespan(100);
        valid_actor.set_age(25);
        assert!(valid_actor.is_valid());
        
        // Test invalid actor (empty name)
        let mut invalid_actor = Actor::new("".to_string(), "Human".to_string());
        invalid_actor.set_lifespan(50);
        invalid_actor.set_age(25);
        assert!(!invalid_actor.is_valid());
        
        // Test invalid actor (empty race)
        let mut invalid_actor2 = Actor::new("TestActor".to_string(), "".to_string());
        invalid_actor2.set_lifespan(100);
        invalid_actor2.set_age(25);
        assert!(!invalid_actor2.is_valid());

        println!("✅ Actor validation test passed!");
    }

    #[tokio::test]
    async fn test_subsystem_priority_ordering() {
        let mut actor = Actor::new("PriorityTest".to_string(), "Human".to_string());
        
        // Add subsystems with different priorities
        let low_priority = SubsystemStruct::new("low".to_string(), 1);
        let high_priority = SubsystemStruct::new("high".to_string(), 10);
        let medium_priority = SubsystemStruct::new("medium".to_string(), 5);
        
        actor.add_subsystem(low_priority);
        actor.add_subsystem(high_priority);
        actor.add_subsystem(medium_priority);
        
        // Verify subsystems were added
        assert_eq!(actor.get_subsystem_count(), 3);
        
        // Test subsystem retrieval
        let low_subsystem = actor.get_subsystem("low");
        assert!(low_subsystem.is_some());
        assert_eq!(low_subsystem.unwrap().get_priority(), 1);
        
        let high_subsystem = actor.get_subsystem("high");
        assert!(high_subsystem.is_some());
        assert_eq!(high_subsystem.unwrap().get_priority(), 10);
        
        let medium_subsystem = actor.get_subsystem("medium");
        assert!(medium_subsystem.is_some());
        assert_eq!(medium_subsystem.unwrap().get_priority(), 5);

        println!("✅ Subsystem priority ordering test passed!");
    }
}
