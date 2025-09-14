//! Validation Example
//! 
//! This example demonstrates the centralized validation system for Actor Core.

use actor_core::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    println!("🔍 Actor Core - Validation Example");
    println!("===================================");

    // Example 1: Valid Contribution
    println!("\n1. Validating a valid contribution:");
    let valid_contribution = create_basic_contribution("strength", 10.0, "equipment");
    let result = validate_contribution(&valid_contribution);
    
    if result.is_valid {
        println!("✅ Contribution is valid");
    } else {
        println!("❌ Contribution validation failed: {:?}", result.errors);
    }

    // Example 2: Invalid Contribution (empty dimension)
    println!("\n2. Validating an invalid contribution:");
    let invalid_contribution = Contribution::new(
        "".to_string(), // Empty dimension - invalid!
        Bucket::Flat,
        10.0,
        "equipment".to_string(),
    );
    let result = validate_contribution(&invalid_contribution);
    
    if result.is_valid {
        println!("✅ Contribution is valid");
    } else {
        println!("❌ Contribution validation failed:");
        for error in &result.errors {
            println!("   - {}: {}", error.code, error.message);
        }
    }

    // Example 3: Invalid Contribution (infinite value)
    println!("\n3. Validating contribution with infinite value:");
    let infinite_contribution = Contribution::new(
        "strength".to_string(),
        Bucket::Flat,
        f64::INFINITY, // Invalid value!
        "equipment".to_string(),
    );
    let result = validate_contribution(&infinite_contribution);
    
    if result.is_valid {
        println!("✅ Contribution is valid");
    } else {
        println!("❌ Contribution validation failed:");
        for error in &result.errors {
            println!("   - {}: {}", error.code, error.message);
        }
    }

    // Example 4: Valid Cap Contribution
    println!("\n4. Validating a valid cap contribution:");
    let valid_cap_contrib = CapContribution::new(
        "equipment".to_string(),
        "strength".to_string(),
        CapMode::Baseline,
        "max".to_string(),
        100.0,
    );
    let result = validate_cap_contribution(&valid_cap_contrib);
    
    if result.is_valid {
        println!("✅ Cap contribution is valid");
    } else {
        println!("❌ Cap contribution validation failed: {:?}", result.errors);
    }

    // Example 5: Invalid Cap Contribution (invalid kind)
    println!("\n5. Validating cap contribution with invalid kind:");
    let invalid_cap_contrib = CapContribution::new(
        "equipment".to_string(),
        "strength".to_string(),
        CapMode::Baseline,
        "invalid_kind".to_string(), // Invalid kind!
        100.0,
    );
    let result = validate_cap_contribution(&invalid_cap_contrib);
    
    if result.is_valid {
        println!("✅ Cap contribution is valid");
    } else {
        println!("❌ Cap contribution validation failed:");
        for error in &result.errors {
            println!("   - {}: {}", error.code, error.message);
        }
    }

    // Example 6: Valid Actor
    println!("\n6. Validating a valid actor:");
    let valid_actor = create_simple_actor("player1", "human", 10);
    let result = validate_actor(&valid_actor);
    
    if result.is_valid {
        println!("✅ Actor is valid");
    } else {
        println!("❌ Actor validation failed: {:?}", result.errors);
    }

    // Example 7: Invalid Actor (empty race)
    println!("\n7. Validating actor with empty race:");
    let invalid_actor = Actor::new("player1".to_string(), "".to_string()); // Empty race!
    let result = validate_actor(&invalid_actor);
    
    if result.is_valid {
        println!("✅ Actor is valid");
    } else {
        println!("❌ Actor validation failed:");
        for error in &result.errors {
            println!("   - {}: {}", error.code, error.message);
        }
    }

    // Example 8: Valid Snapshot
    println!("\n8. Validating a valid snapshot:");
    let mut valid_snapshot = Snapshot::new("player1".parse().unwrap(), 1);
    valid_snapshot.primary.insert("strength".to_string(), 50.0);
    valid_snapshot.derived.insert("health".to_string(), 200.0);
    
    let result = validate_snapshot(&valid_snapshot);
    
    if result.is_valid {
        println!("✅ Snapshot is valid");
    } else {
        println!("❌ Snapshot validation failed: {:?}", result.errors);
    }

    // Example 9: Custom Validation Rules
    println!("\n9. Using custom validation rules:");
    let mut custom_rules = ValidationRules::default();
    custom_rules.max_contribution_value = 50.0; // Lower limit
    
    let validator = Validator::with_rules(custom_rules);
    let high_value_contribution = create_basic_contribution("strength", 75.0, "equipment"); // Above limit!
    let result = validator.validate_contribution(&high_value_contribution);
    
    if result.is_valid {
        println!("✅ Contribution is valid with custom rules");
    } else {
        println!("❌ Contribution validation failed with custom rules:");
        for error in &result.errors {
            println!("   - {}: {}", error.code, error.message);
        }
    }

    // Example 10: Validation with Warnings
    println!("\n10. Validation with warnings:");
    let mut actor_with_invalid_data = create_simple_actor("player1", "human", 10);
    let mut data = HashMap::new();
    data.insert("custom_field".to_string(), serde_json::Value::Bool(true)); // Unsupported type
    actor_with_invalid_data.set_data(data);
    
    let result = validate_actor(&actor_with_invalid_data);
    
    if result.is_valid {
        println!("✅ Actor is valid");
        if result.has_warnings() {
            println!("⚠️  Warnings:");
            for warning in &result.warnings {
                println!("   - {}: {}", warning.code, warning.message);
            }
        }
    } else {
        println!("❌ Actor validation failed:");
        for error in &result.errors {
            println!("   - {}: {}", error.code, error.message);
        }
    }

    // Example 11: Batch Validation
    println!("\n11. Batch validation of multiple contributions:");
    let contributions = vec![
        create_basic_contribution("strength", 10.0, "equipment"),
        create_basic_contribution("agility", 15.0, "equipment"),
        create_basic_contribution("", 20.0, "equipment"), // Invalid - empty dimension
        create_basic_contribution("intelligence", 25.0, "equipment"),
    ];
    
    let result = validators::validate_contributions(&contributions);
    
    if result.is_valid {
        println!("✅ All contributions are valid");
    } else {
        println!("❌ Some contributions failed validation:");
        for error in &result.errors {
            println!("   - {}: {}", error.code, error.message);
        }
    }

    // Example 12: Validation Middleware
    println!("\n12. Using validation middleware:");
    let validator = Arc::new(Validator::new());
    
    // Create a real aggregator and wrap it with validation middleware
    let cache = ServiceFactory::create_cache()?;
    let plugin_registry = ServiceFactory::create_plugin_registry();
    let combiner_registry = ServiceFactory::create_combiner_registry();
    let cap_layers = ServiceFactory::create_cap_layer_registry();
    let caps_provider = ServiceFactory::create_caps_provider(cap_layers);
    let aggregator = ServiceFactory::create_aggregator(
        plugin_registry,
        combiner_registry,
        caps_provider,
        cache,
    );
    let validated_aggregator = ValidationMiddlewareFactory::create_validated_aggregator(
        aggregator,
        validator,
    );
    
    let actor = create_simple_actor("player1", "human", 10);
    let snapshot_result = validated_aggregator.resolve(&actor).await;
    
    match snapshot_result {
        Ok(snapshot) => {
            println!("✅ Actor resolution successful with validation");
            println!("   Snapshot version: {}", snapshot.version);
        }
        Err(e) => {
            println!("❌ Actor resolution failed: {}", e);
        }
    }
    
    // Get validation statistics
    let validation_stats = validated_aggregator.get_stats().await;
    println!("📊 Validation Statistics:");
    println!("   Total validations: {}", validation_stats.total_validations);
    println!("   Passed validations: {}", validation_stats.passed_validations);
    println!("   Failed validations: {}", validation_stats.failed_validations);
    println!("   Warnings generated: {}", validation_stats.warnings_generated);

    println!("\n🎉 Validation example completed successfully!");
    Ok(())
}
