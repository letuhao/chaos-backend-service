use actor_core::types::{Actor, ActorGodClass, JindanStats, RpgStats};
use std::time::Instant;

fn main() {
    println!("🔬 Detailed Performance Analysis: Actor vs Actor God Class");
    println!("=========================================================");
    
    let iterations = 100_000;
    
    // Test 1: Basic field access patterns
    println!("\n📊 Test 1: Basic Field Access Patterns");
    test_basic_field_access(iterations);
    
    // Test 2: Stat calculation performance
    println!("\n🧮 Test 2: Stat Calculation Performance");
    test_stat_calculation(iterations);
    
    // Test 3: Memory access patterns
    println!("\n💾 Test 3: Memory Access Patterns");
    test_memory_access_patterns(iterations);
    
    // Test 4: Real-world simulation
    println!("\n🎮 Test 4: Real-world Game Simulation");
    test_real_world_simulation(iterations);
    
    // Test 5: Aggregator compatibility
    println!("\n⚙️ Test 5: Aggregator Compatibility");
    test_aggregator_compatibility(iterations);
}

fn test_basic_field_access(iterations: usize) {
    let actor = Actor::new("Test Actor".to_string(), "Human".to_string());
    let god_class = ActorGodClass::new("Test God Class".to_string(), "Human".to_string());
    
    // Test Actor field access
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = actor.name;
        let _ = actor.race;
        let _ = actor.id;
        let _ = actor.lifespan;
        let _ = actor.age;
        let _ = actor.version;
    }
    let actor_time = start.elapsed().as_nanos();
    
    // Test God Class field access
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = god_class.name;
        let _ = god_class.race;
        let _ = god_class.id;
        let _ = god_class.lifespan;
        let _ = god_class.age;
        let _ = god_class.version;
    }
    let god_class_time = start.elapsed().as_nanos();
    
    println!("  Actor field access: {:.2}ns per iteration", actor_time as f64 / iterations as f64);
    println!("  God Class field access: {:.2}ns per iteration", god_class_time as f64 / iterations as f64);
    println!("  Overhead: {:.1}%", ((god_class_time as f64 - actor_time as f64) / actor_time as f64) * 100.0);
}

fn test_stat_calculation(iterations: usize) {
    let mut god_class = ActorGodClass::new("Test".to_string(), "Human".to_string());
    
    // Test stat calculation performance
    let start = Instant::now();
    for i in 0..iterations {
        god_class.jindan.vital_essence += 0.1;
        god_class.jindan.qi_control += 0.1;
        god_class.rpg.strength += 0.1;
        god_class.rpg.intelligence += 0.1;
        
        if i % 100 == 0 {
            god_class.calculate_total_stats();
        }
    }
    let calculation_time = start.elapsed().as_nanos();
    
    println!("  Stat calculation: {:.2}ns per iteration", calculation_time as f64 / iterations as f64);
    println!("  Total stats: Health={:.2}, Lifespan={}, Wisdom={:.2}", 
        god_class.total_health, god_class.total_lifespan, god_class.total_wisdom);
}

fn test_memory_access_patterns(iterations: usize) {
    let actors: Vec<Actor> = (0..1000).map(|i| Actor::new(format!("Actor_{}", i), "Human".to_string())).collect();
    let god_classes: Vec<ActorGodClass> = (0..1000).map(|i| ActorGodClass::new(format!("GodClass_{}", i), "Human".to_string())).collect();
    
    // Test sequential access
    let start = Instant::now();
    let mut sum = 0;
    for _ in 0..iterations {
        for actor in &actors {
            sum += actor.name.len();
        }
    }
    let actor_sequential = start.elapsed().as_nanos();
    
    let start = Instant::now();
    let mut sum = 0.0;
    for _ in 0..iterations {
        for god_class in &god_classes {
            sum += god_class.total_health;
        }
    }
    let god_class_sequential = start.elapsed().as_nanos();
    
    // Test random access
    let start = Instant::now();
    let mut sum = 0;
    for _ in 0..iterations {
        let index = (sum % 1000) as usize;
        sum += actors[index].name.len();
    }
    let actor_random = start.elapsed().as_nanos();
    
    let start = Instant::now();
    let mut sum = 0.0;
    for _ in 0..iterations {
        let index = (sum as usize % 1000) as usize;
        sum += god_classes[index].total_health;
    }
    let god_class_random = start.elapsed().as_nanos();
    
    println!("  Sequential access:");
    println!("    Actor: {:.2}ns per access", actor_sequential as f64 / (iterations * 1000) as f64);
    println!("    God Class: {:.2}ns per access", god_class_sequential as f64 / (iterations * 1000) as f64);
    println!("  Random access:");
    println!("    Actor: {:.2}ns per access", actor_random as f64 / iterations as f64);
    println!("    God Class: {:.2}ns per access", god_class_random as f64 / iterations as f64);
}

fn test_real_world_simulation(iterations: usize) {
    let mut actors: Vec<Actor> = (0..100).map(|i| Actor::new(format!("Actor_{}", i), "Human".to_string())).collect();
    let mut god_classes: Vec<ActorGodClass> = (0..100).map(|i| ActorGodClass::new(format!("GodClass_{}", i), "Human".to_string())).collect();
    
    // Simulate game loop with Actor
    let start = Instant::now();
    for _ in 0..iterations {
        for actor in &mut actors {
            actor.touch();
            actor.data.insert("health".to_string(), serde_json::Value::Number(100.into()));
            actor.data.insert("mana".to_string(), serde_json::Value::Number(50.into()));
        }
    }
    let actor_simulation = start.elapsed().as_nanos();
    
    // Simulate game loop with God Class
    let start = Instant::now();
    for _ in 0..iterations {
        for god_class in &mut god_classes {
            god_class.touch();
            god_class.jindan.vital_essence += 0.1;
            god_class.rpg.strength += 0.1;
            god_class.calculate_total_stats();
        }
    }
    let god_class_simulation = start.elapsed().as_nanos();
    
    println!("  Game loop simulation (100 entities):");
    println!("    Actor: {:.2}ns per frame", actor_simulation as f64 / iterations as f64);
    println!("    God Class: {:.2}ns per frame", god_class_simulation as f64 / iterations as f64);
    println!("    Overhead: {:.1}%", ((god_class_simulation as f64 - actor_simulation as f64) / actor_simulation as f64) * 100.0);
}

fn test_aggregator_compatibility(iterations: usize) {
    let actors: Vec<Actor> = (0..100).map(|i| Actor::new(format!("Actor_{}", i), "Human".to_string())).collect();
    let god_classes: Vec<ActorGodClass> = (0..100).map(|i| ActorGodClass::new(format!("GodClass_{}", i), "Human".to_string())).collect();
    
    // Test Actor with subsystems (simulating aggregator)
    let start = Instant::now();
    for _ in 0..iterations {
        for actor in &actors {
            let _ = actor.subsystems.len();
            let _ = actor.data.len();
            // Simulate aggregator processing
            let _ = actor.name.len() + actor.race.len();
        }
    }
    let actor_aggregator = start.elapsed().as_nanos();
    
    // Test God Class with subsystems (simulating aggregator)
    let start = Instant::now();
    for _ in 0..iterations {
        for god_class in &god_classes {
            let _ = god_class.subsystems.len();
            let _ = god_class.data.len();
            // Simulate aggregator processing with hardcoded stats
            let _ = god_class.total_health + god_class.total_lifespan as f64;
        }
    }
    let god_class_aggregator = start.elapsed().as_nanos();
    
    println!("  Aggregator compatibility test:");
    println!("    Actor: {:.2}ns per entity", actor_aggregator as f64 / (iterations * 100) as f64);
    println!("    God Class: {:.2}ns per entity", god_class_aggregator as f64 / (iterations * 100) as f64);
    println!("    Overhead: {:.1}%", ((god_class_aggregator as f64 - actor_aggregator as f64) / actor_aggregator as f64) * 100.0);
}

// Additional analysis functions
fn analyze_cache_performance() {
    use std::mem;
    
    println!("\n🔍 Cache Performance Analysis:");
    
    let actor_size = mem::size_of::<Actor>();
    let god_class_size = mem::size_of::<ActorGodClass>();
    let jindan_size = mem::size_of::<JindanStats>();
    let rpg_size = mem::size_of::<RpgStats>();
    
    println!("  Memory layout:");
    println!("    Actor: {} bytes", actor_size);
    println!("    God Class: {} bytes", god_class_size);
    println!("    Jindan Stats: {} bytes", jindan_size);
    println!("    RPG Stats: {} bytes", rpg_size);
    
    // Cache line analysis
    let cache_line_size = 64;
    let actor_lines = (actor_size + cache_line_size - 1) / cache_line_size;
    let god_class_lines = (god_class_size + cache_line_size - 1) / cache_line_size;
    
    println!("  Cache lines (64 bytes each):");
    println!("    Actor: {} lines", actor_lines);
    println!("    God Class: {} lines", god_class_lines);
    println!("    Additional lines: {}", god_class_lines - actor_lines);
    
    // Memory efficiency
    let actor_efficiency = (actor_size as f64 / cache_line_size as f64) * 100.0;
    let god_class_efficiency = (god_class_size as f64 / (god_class_lines * cache_line_size) as f64) * 100.0;
    
    println!("  Memory efficiency:");
    println!("    Actor: {:.1}% of cache lines used", actor_efficiency);
    println!("    God Class: {:.1}% of cache lines used", god_class_efficiency);
}
