use actor_core::types::{Actor, ActorGodClass};
use std::time::Instant;
use std::collections::HashMap;
use serde_json::Value;

fn main() {
    println!("⚖️ Correct Performance Comparison: Actor (HashMap) vs Actor God Class (Hardcoded)");
    println!("=================================================================================");
    
    let iterations = 1_000_000;
    
    // Test Actor with HashMap stats
    println!("\n📊 Testing Actor with HashMap Stats ({} iterations):", iterations);
    let actor_times = benchmark_actor_with_hashmap(iterations);
    
    // Test Actor God Class with hardcoded stats
    println!("\n🏛️ Testing Actor God Class with Hardcoded Stats ({} iterations):", iterations);
    let god_class_times = benchmark_actor_god_class(iterations);
    
    // Compare results
    println!("\n📈 Performance Comparison Results:");
    println!("===================================");
    compare_results(&actor_times, &god_class_times, iterations);
}

fn benchmark_actor_with_hashmap(iterations: usize) -> BenchmarkTimes {
    // Create Actor instance
    let mut actor = Actor::new("Test Actor".to_string(), "Human".to_string());
    
    // Put stats into HashMap with same names as Actor God Class
    let stats = create_stats_hashmap();
    for (key, value) in stats {
        actor.data.insert(key, value);
    }
    
    // Test creation time (just one actor)
    let start = Instant::now();
    let _actor = Actor::new("Test".to_string(), "Human".to_string());
    let creation_time = start.elapsed().as_nanos();
    
    // Test field access time (get stats from HashMap)
    let start = Instant::now();
    let mut sum = 0.0;
    for _ in 0..iterations {
        // Access base fields
        sum += actor.name.len() as f64;
        sum += actor.race.len() as f64;
        let _ = actor.id;
        let _ = actor.lifespan;
        let _ = actor.age;
        let _ = actor.version;
        
        // Access stats from HashMap (same names as God Class)
        if let Some(health) = actor.data.get("total_health").and_then(|v| v.as_f64()) {
            sum += health;
        }
        if let Some(lifespan) = actor.data.get("total_lifespan").and_then(|v| v.as_i64()) {
            sum += lifespan as f64;
        }
        if let Some(wisdom) = actor.data.get("total_wisdom").and_then(|v| v.as_f64()) {
            sum += wisdom;
        }
        if let Some(strength) = actor.data.get("total_strength").and_then(|v| v.as_f64()) {
            sum += strength;
        }
        if let Some(agility) = actor.data.get("total_agility").and_then(|v| v.as_f64()) {
            sum += agility;
        }
        if let Some(endurance) = actor.data.get("total_endurance").and_then(|v| v.as_f64()) {
            sum += endurance;
        }
        if let Some(perception) = actor.data.get("total_perception").and_then(|v| v.as_f64()) {
            sum += perception;
        }
        if let Some(luck) = actor.data.get("total_luck").and_then(|v| v.as_f64()) {
            sum += luck;
        }
        if let Some(mana) = actor.data.get("total_mana").and_then(|v| v.as_f64()) {
            sum += mana;
        }
        if let Some(stamina) = actor.data.get("total_stamina").and_then(|v| v.as_f64()) {
            sum += stamina;
        }
        if let Some(experience) = actor.data.get("total_experience").and_then(|v| v.as_f64()) {
            sum += experience;
        }
        if let Some(level) = actor.data.get("total_level").and_then(|v| v.as_i64()) {
            sum += level as f64;
        }
        
        // Access Jindan stats from HashMap
        if let Some(vital_essence) = actor.data.get("jindan_vital_essence").and_then(|v| v.as_f64()) {
            sum += vital_essence;
        }
        if let Some(qi_control) = actor.data.get("jindan_qi_control").and_then(|v| v.as_f64()) {
            sum += qi_control;
        }
        if let Some(meridian_strength) = actor.data.get("jindan_meridian_strength").and_then(|v| v.as_f64()) {
            sum += meridian_strength;
        }
        if let Some(body_constitution) = actor.data.get("jindan_body_constitution").and_then(|v| v.as_f64()) {
            sum += body_constitution;
        }
        if let Some(soul_consciousness) = actor.data.get("jindan_soul_consciousness").and_then(|v| v.as_f64()) {
            sum += soul_consciousness;
        }
        if let Some(dao_comprehension) = actor.data.get("jindan_dao_comprehension").and_then(|v| v.as_f64()) {
            sum += dao_comprehension;
        }
        if let Some(fortune) = actor.data.get("jindan_fortune").and_then(|v| v.as_f64()) {
            sum += fortune;
        }
        
        // Access RPG stats from HashMap
        if let Some(rpg_strength) = actor.data.get("rpg_strength").and_then(|v| v.as_f64()) {
            sum += rpg_strength;
        }
        if let Some(rpg_intelligence) = actor.data.get("rpg_intelligence").and_then(|v| v.as_f64()) {
            sum += rpg_intelligence;
        }
        if let Some(rpg_willpower) = actor.data.get("rpg_willpower").and_then(|v| v.as_f64()) {
            sum += rpg_willpower;
        }
        if let Some(rpg_agility) = actor.data.get("rpg_agility").and_then(|v| v.as_f64()) {
            sum += rpg_agility;
        }
        if let Some(rpg_speed) = actor.data.get("rpg_speed").and_then(|v| v.as_f64()) {
            sum += rpg_speed;
        }
        if let Some(rpg_endurance) = actor.data.get("rpg_endurance").and_then(|v| v.as_f64()) {
            sum += rpg_endurance;
        }
        if let Some(rpg_perception) = actor.data.get("rpg_perception").and_then(|v| v.as_f64()) {
            sum += rpg_perception;
        }
        if let Some(rpg_luck) = actor.data.get("rpg_luck").and_then(|v| v.as_f64()) {
            sum += rpg_luck;
        }
    }
    let field_access_time = start.elapsed().as_nanos();
    
    // Test field modification time (set stats in HashMap)
    let start = Instant::now();
    for i in 0..iterations {
        actor.touch();
        actor.data.insert("test_key".to_string(), Value::String("test_value".to_string()));
        
        // Modify stats in HashMap
        if let Some(health) = actor.data.get("total_health").and_then(|v| v.as_f64()) {
            actor.data.insert("total_health".to_string(), Value::Number(serde_json::Number::from_f64(health + 1.0).unwrap()));
        }
        if let Some(strength) = actor.data.get("total_strength").and_then(|v| v.as_f64()) {
            actor.data.insert("total_strength".to_string(), Value::Number(serde_json::Number::from_f64(strength + 1.0).unwrap()));
        }
        if let Some(vital_essence) = actor.data.get("jindan_vital_essence").and_then(|v| v.as_f64()) {
            actor.data.insert("jindan_vital_essence".to_string(), Value::Number(serde_json::Number::from_f64(vital_essence + 1.0).unwrap()));
        }
        if let Some(rpg_strength) = actor.data.get("rpg_strength").and_then(|v| v.as_f64()) {
            actor.data.insert("rpg_strength".to_string(), Value::Number(serde_json::Number::from_f64(rpg_strength + 1.0).unwrap()));
        }
    }
    let field_modification_time = start.elapsed().as_nanos();
    
    let total_time = creation_time + field_access_time + field_modification_time;
    
    println!("  Creation: {:.2}ns per actor", creation_time as f64);
    println!("  Field Access: {:.2}ns per access", field_access_time as f64 / (iterations * 30) as f64);
    println!("  Field Modification: {:.2}ns per modification", field_modification_time as f64 / iterations as f64);
    println!("  Total: {:.2}ns per iteration", total_time as f64 / iterations as f64);
    println!("  Total Time: {:.2}ms for {} iterations", total_time as f64 / 1_000_000.0, iterations);
    
    BenchmarkTimes {
        creation: creation_time,
        field_access: field_access_time,
        field_modification: field_modification_time,
        total: total_time,
    }
}

fn benchmark_actor_god_class(iterations: usize) -> BenchmarkTimes {
    // Create Actor God Class instance
    let mut god_class = ActorGodClass::new("Test God Class".to_string(), "Human".to_string());
    
    // Test creation time (just one god class)
    let start = Instant::now();
    let _god_class = ActorGodClass::new("Test".to_string(), "Human".to_string());
    let creation_time = start.elapsed().as_nanos();
    
    // Test field access time (get stats from hardcoded fields)
    let start = Instant::now();
    let mut sum = 0.0;
    for _ in 0..iterations {
        // Access base fields
        sum += god_class.name.len() as f64;
        sum += god_class.race.len() as f64;
        let _ = god_class.id;
        let _ = god_class.lifespan;
        let _ = god_class.age;
        let _ = god_class.version;
        
        // Access hardcoded stats (same names as HashMap)
        sum += god_class.total_health;
        sum += god_class.total_lifespan as f64;
        sum += god_class.total_wisdom;
        sum += god_class.total_strength;
        sum += god_class.total_agility;
        sum += god_class.total_endurance;
        sum += god_class.total_perception;
        sum += god_class.total_luck;
        sum += god_class.total_mana;
        sum += god_class.total_stamina;
        sum += god_class.total_experience;
        sum += god_class.total_level as f64;
        
        // Access Jindan stats from hardcoded fields
        sum += god_class.jindan.vital_essence;
        sum += god_class.jindan.qi_control;
        sum += god_class.jindan.meridian_strength;
        sum += god_class.jindan.body_constitution;
        sum += god_class.jindan.soul_consciousness;
        sum += god_class.jindan.dao_comprehension;
        sum += god_class.jindan.fortune;
        
        // Access RPG stats from hardcoded fields
        sum += god_class.rpg.strength;
        sum += god_class.rpg.intelligence;
        sum += god_class.rpg.willpower;
        sum += god_class.rpg.agility;
        sum += god_class.rpg.speed;
        sum += god_class.rpg.endurance;
        sum += god_class.rpg.perception;
        sum += god_class.rpg.luck;
    }
    let field_access_time = start.elapsed().as_nanos();
    
    // Test field modification time (set stats in hardcoded fields)
    let start = Instant::now();
    for _ in 0..iterations {
        god_class.touch();
        god_class.data.insert("test_key".to_string(), Value::String("test_value".to_string()));
        
        // Modify hardcoded stats
        god_class.total_health += 1.0;
        god_class.total_strength += 1.0;
        god_class.jindan.vital_essence += 1.0;
        god_class.rpg.strength += 1.0;
        god_class.calculate_total_stats();
    }
    let field_modification_time = start.elapsed().as_nanos();
    
    let total_time = creation_time + field_access_time + field_modification_time;
    
    println!("  Creation: {:.2}ns per god class", creation_time as f64);
    println!("  Field Access: {:.2}ns per access", field_access_time as f64 / (iterations * 30) as f64);
    println!("  Field Modification: {:.2}ns per modification", field_modification_time as f64 / iterations as f64);
    println!("  Total: {:.2}ns per iteration", total_time as f64 / iterations as f64);
    println!("  Total Time: {:.2}ms for {} iterations", total_time as f64 / 1_000_000.0, iterations);
    
    BenchmarkTimes {
        creation: creation_time,
        field_access: field_access_time,
        field_modification: field_modification_time,
        total: total_time,
    }
}

fn create_stats_hashmap() -> HashMap<String, Value> {
    let mut stats = HashMap::new();
    
    // Total stats
    stats.insert("total_health".to_string(), Value::Number(serde_json::Number::from_f64(2350.0).unwrap()));
    stats.insert("total_lifespan".to_string(), Value::Number(serde_json::Number::from(180)));
    stats.insert("total_wisdom".to_string(), Value::Number(serde_json::Number::from_f64(60.0).unwrap()));
    stats.insert("total_strength".to_string(), Value::Number(serde_json::Number::from_f64(80.0).unwrap()));
    stats.insert("total_agility".to_string(), Value::Number(serde_json::Number::from_f64(90.0).unwrap()));
    stats.insert("total_endurance".to_string(), Value::Number(serde_json::Number::from_f64(70.0).unwrap()));
    stats.insert("total_perception".to_string(), Value::Number(serde_json::Number::from_f64(60.0).unwrap()));
    stats.insert("total_luck".to_string(), Value::Number(serde_json::Number::from_f64(40.0).unwrap()));
    stats.insert("total_mana".to_string(), Value::Number(serde_json::Number::from_f64(1350.0).unwrap()));
    stats.insert("total_stamina".to_string(), Value::Number(serde_json::Number::from_f64(330.0).unwrap()));
    stats.insert("total_experience".to_string(), Value::Number(serde_json::Number::from_f64(40.0).unwrap()));
    stats.insert("total_level".to_string(), Value::Number(serde_json::Number::from(2)));
    
    // Jindan stats
    stats.insert("jindan_vital_essence".to_string(), Value::Number(serde_json::Number::from_f64(100.0).unwrap()));
    stats.insert("jindan_qi_control".to_string(), Value::Number(serde_json::Number::from_f64(80.0).unwrap()));
    stats.insert("jindan_meridian_strength".to_string(), Value::Number(serde_json::Number::from_f64(60.0).unwrap()));
    stats.insert("jindan_body_constitution".to_string(), Value::Number(serde_json::Number::from_f64(70.0).unwrap()));
    stats.insert("jindan_soul_consciousness".to_string(), Value::Number(serde_json::Number::from_f64(50.0).unwrap()));
    stats.insert("jindan_dao_comprehension".to_string(), Value::Number(serde_json::Number::from_f64(40.0).unwrap()));
    stats.insert("jindan_fortune".to_string(), Value::Number(serde_json::Number::from_f64(30.0).unwrap()));
    
    // RPG stats
    stats.insert("rpg_strength".to_string(), Value::Number(serde_json::Number::from_f64(10.0).unwrap()));
    stats.insert("rpg_intelligence".to_string(), Value::Number(serde_json::Number::from_f64(10.0).unwrap()));
    stats.insert("rpg_willpower".to_string(), Value::Number(serde_json::Number::from_f64(10.0).unwrap()));
    stats.insert("rpg_agility".to_string(), Value::Number(serde_json::Number::from_f64(10.0).unwrap()));
    stats.insert("rpg_speed".to_string(), Value::Number(serde_json::Number::from_f64(10.0).unwrap()));
    stats.insert("rpg_endurance".to_string(), Value::Number(serde_json::Number::from_f64(10.0).unwrap()));
    stats.insert("rpg_perception".to_string(), Value::Number(serde_json::Number::from_f64(10.0).unwrap()));
    stats.insert("rpg_luck".to_string(), Value::Number(serde_json::Number::from(10)));
    
    stats
}

struct BenchmarkTimes {
    creation: u128,
    field_access: u128,
    field_modification: u128,
    total: u128,
}

fn compare_results(actor_times: &BenchmarkTimes, god_class_times: &BenchmarkTimes, iterations: usize) {
    let actor_creation_per_item = actor_times.creation as f64;
    let god_class_creation_per_item = god_class_times.creation as f64;
    
    let actor_access_per_item = actor_times.field_access as f64 / (iterations * 30) as f64;
    let god_class_access_per_item = god_class_times.field_access as f64 / (iterations * 30) as f64;
    
    let actor_modification_per_item = actor_times.field_modification as f64 / iterations as f64;
    let god_class_modification_per_item = god_class_times.field_modification as f64 / iterations as f64;
    
    let actor_total_per_item = actor_times.total as f64 / iterations as f64;
    let god_class_total_per_item = god_class_times.total as f64 / iterations as f64;
    
    println!("  Creation Performance:");
    println!("    Actor (HashMap): {:.2}ns per item", actor_creation_per_item);
    println!("    God Class: {:.2}ns per item", god_class_creation_per_item);
    println!("    Overhead: {:.1}%", ((god_class_creation_per_item - actor_creation_per_item) / actor_creation_per_item * 100.0));
    
    println!("  Field Access Performance:");
    println!("    Actor (HashMap): {:.2}ns per access", actor_access_per_item);
    println!("    God Class: {:.2}ns per access", god_class_access_per_item);
    println!("    Speed Improvement: {:.1}x faster", actor_access_per_item / god_class_access_per_item);
    
    println!("  Field Modification Performance:");
    println!("    Actor (HashMap): {:.2}ns per modification", actor_modification_per_item);
    println!("    God Class: {:.2}ns per modification", god_class_modification_per_item);
    println!("    Speed Improvement: {:.1}x faster", actor_modification_per_item / god_class_modification_per_item);
    
    println!("  Total Performance:");
    println!("    Actor (HashMap): {:.2}ns per iteration", actor_total_per_item);
    println!("    God Class: {:.2}ns per iteration", god_class_total_per_item);
    println!("    Speed Improvement: {:.1}x faster", actor_total_per_item / god_class_total_per_item);
    
    // Summary with totals
    println!("\n📊 Summary ({} iterations):", iterations);
    println!("  Actor (HashMap) Total Time: {:.2}ms", actor_times.total as f64 / 1_000_000.0);
    println!("  God Class Total Time: {:.2}ms", god_class_times.total as f64 / 1_000_000.0);
    println!("  Time Saved: {:.2}ms", (actor_times.total - god_class_times.total) as f64 / 1_000_000.0);
    println!("  Performance Gain: {:.1}%", ((actor_times.total - god_class_times.total) as f64 / actor_times.total as f64) * 100.0);
    
    // Performance analysis
    println!("\n🔍 Performance Analysis:");
    let field_access_speedup = actor_access_per_item / god_class_access_per_item;
    let field_modification_speedup = actor_modification_per_item / god_class_modification_per_item;
    let total_speedup = actor_total_per_item / god_class_total_per_item;
    
    if field_access_speedup > 10.0 {
        println!("  🚀 God Class field access is dramatically faster ({:.1}x speedup)", field_access_speedup);
    } else if field_access_speedup > 5.0 {
        println!("  🚀 God Class field access is significantly faster ({:.1}x speedup)", field_access_speedup);
    } else if field_access_speedup > 2.0 {
        println!("  ✅ God Class field access is faster ({:.1}x speedup)", field_access_speedup);
    } else if field_access_speedup > 1.0 {
        println!("  ⚠️  God Class field access is slightly faster ({:.1}x speedup)", field_access_speedup);
    } else {
        println!("  ❌ God Class field access is slower ({:.1}x speedup)", field_access_speedup);
    }
    
    if field_modification_speedup > 5.0 {
        println!("  🚀 God Class field modification is significantly faster ({:.1}x speedup)", field_modification_speedup);
    } else if field_modification_speedup > 2.0 {
        println!("  ✅ God Class field modification is faster ({:.1}x speedup)", field_modification_speedup);
    } else if field_modification_speedup > 1.0 {
        println!("  ⚠️  God Class field modification is slightly faster ({:.1}x speedup)", field_modification_speedup);
    } else {
        println!("  ❌ God Class field modification is slower ({:.1}x speedup)", field_modification_speedup);
    }
    
    if total_speedup > 10.0 {
        println!("  🚀 God Class total performance is dramatically better ({:.1}x speedup)", total_speedup);
    } else if total_speedup > 5.0 {
        println!("  🚀 God Class total performance is significantly better ({:.1}x speedup)", total_speedup);
    } else if total_speedup > 2.0 {
        println!("  ✅ God Class total performance is better ({:.1}x speedup)", total_speedup);
    } else if total_speedup > 1.0 {
        println!("  ⚠️  God Class total performance is slightly better ({:.1}x speedup)", total_speedup);
    } else {
        println!("  ❌ God Class total performance is worse ({:.1}x speedup)", total_speedup);
    }
}
