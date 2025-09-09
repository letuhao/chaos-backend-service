//! Simple Benchmarks
//! 
//! This module contains basic benchmarks that work with the current API.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use actor_core::types::*;
use actor_core::enums::*;
use actor_core::bucket_processor::*;
use std::collections::HashMap;

/// Benchmark actor creation
fn bench_actor_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("actor_creation");
    
    for count in [1, 10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("create_actors", count), count, |b, &count| {
            b.iter(|| {
                let actors: Vec<Actor> = (0..count)
                    .map(|i| Actor::new(format!("Actor_{}", i), "Human".to_string()))
                    .collect();
                black_box(actors)
            })
        });
    }
    
    group.finish();
}

/// Benchmark caps operations
fn bench_caps_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("caps_operations");
    
    for count in [1, 10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("create_caps", count), count, |b, &count| {
            b.iter(|| {
                let caps: Vec<Caps> = (0..count)
                    .map(|i| Caps::new(i as f64, (i + 100) as f64))
                    .collect();
                black_box(caps)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("caps_operations", count), count, |b, &count| {
            let caps: Vec<Caps> = (0..count)
                .map(|i| Caps::new(i as f64, (i + 100) as f64))
                .collect();
            
            b.iter(|| {
                for (i, cap) in caps.iter().enumerate() {
                    let _union = cap.union(&Caps::new(50.0, 150.0));
                    let _intersection = cap.intersection(&Caps::new(25.0, 125.0));
                    let _clamped = cap.clamp((i as f64) * 1.5);
                    let _contains = cap.contains((i as f64) * 0.8);
                }
                black_box(&caps)
            })
        });
    }
    
    group.finish();
}

/// Benchmark contribution creation
fn bench_contribution_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("contribution_creation");
    
    for count in [10, 100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("create_contributions", count), count, |b, &count| {
            b.iter(|| {
                let contributions: Vec<Contribution> = (0..count)
                    .map(|i| {
                        let bucket = match i % 4 {
                            0 => Bucket::Flat,
                            1 => Bucket::Mult,
                            2 => Bucket::PostAdd,
                            _ => Bucket::Override,
                        };
                        Contribution::new(
                            format!("stat_{}", i % 20),
                            bucket,
                            (i as f64) * 0.1 + 1.0,
                            format!("source_{}", i % 10)
                        )
                    })
                    .collect();
                black_box(contributions)
            })
        });
    }
    
    group.finish();
}

/// Benchmark contribution validation
fn bench_contribution_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("contribution_validation");
    
    for count in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("validate_contributions", count), count, |b, &count| {
            let contributions: Vec<Contribution> = (0..count)
                .map(|i| {
                    let bucket = match i % 4 {
                        0 => Bucket::Flat,
                        1 => Bucket::Mult,
                        2 => Bucket::PostAdd,
                        _ => Bucket::Override,
                    };
                    Contribution::new(
                        format!("stat_{}", i % 20),
                        bucket,
                        (i as f64) * 0.1 + 1.0,
                        format!("source_{}", i % 10)
                    )
                })
                .collect();
            
            b.iter(|| {
                let result = validate_contributions(&contributions);
                black_box(result)
            })
        });
    }
    
    group.finish();
}

/// Benchmark bucket processing
fn bench_bucket_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("bucket_processing");
    
    for count in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("process_contributions", count), count, |b, &count| {
            let contributions: Vec<Contribution> = (0..count)
                .map(|i| {
                    let bucket = match i % 4 {
                        0 => Bucket::Flat,
                        1 => Bucket::Mult,
                        2 => Bucket::PostAdd,
                        _ => Bucket::Override,
                    };
                    Contribution::new(
                        format!("stat_{}", i % 20),
                        bucket,
                        (i as f64) * 0.1 + 1.0,
                        format!("source_{}", i % 10)
                    )
                })
                .collect();
            
            b.iter(|| {
                let result = process_contributions_in_order(contributions.clone(), 0.0, None);
                black_box(result)
            })
        });
    }
    
    group.finish();
}

/// Benchmark snapshot creation
fn bench_snapshot_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("snapshot_creation");
    
    for count in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("create_snapshots", count), count, |b, &count| {
            b.iter(|| {
                let snapshots: Vec<Snapshot> = (0..count)
                    .map(|i| {
                        let mut snapshot = Snapshot::new(uuid::Uuid::new_v4(), i as u64);
                        
                        // Add some contributions
                        for j in 0..10 {
                            let contrib = Contribution::new(
                                format!("stat_{}", j),
                                Bucket::Flat,
                                (i + j) as f64,
                                "benchmark".to_string()
                            );
                            snapshot.add_primary(contrib);
                        }
                        
                        snapshot
                    })
                    .collect();
                black_box(snapshots)
            })
        });
    }
    
    group.finish();
}

/// Benchmark actor data operations
fn bench_actor_data_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("actor_data_operations");
    
    for count in [1, 10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("actor_data_ops", count), count, |b, &count| {
            b.iter(|| {
                let mut actors: Vec<Actor> = (0..count)
                    .map(|i| Actor::new(format!("Actor_{}", i), "Human".to_string()))
                    .collect();
                
                for (i, actor) in actors.iter_mut().enumerate() {
                    let mut data = HashMap::new();
                    data.insert("level".to_string(), serde_json::Value::Number(serde_json::Number::from(i as i64)));
                    data.insert("class".to_string(), serde_json::Value::String("warrior".to_string()));
                    actor.set_data(data);
                    
                    actor.add_buff(format!("buff_{}", i));
                    actor.set_combat_duration(60);
                    actor.set_guild_id(format!("guild_{}", i % 10));
                }
                black_box(actors)
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_actor_creation,
    bench_caps_operations,
    bench_contribution_creation,
    bench_contribution_validation,
    bench_bucket_processing,
    bench_snapshot_creation,
    bench_actor_data_operations
);

criterion_main!(benches);
