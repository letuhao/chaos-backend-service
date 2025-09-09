//! Bucket Processor Benchmarks
//! 
//! This module contains specialized benchmarks for the bucket processor,
//! measuring performance of contribution processing, ordering, and clamping.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use actor_core::types::*;
use actor_core::enums::*;
use actor_core::bucket_processor::*;
use std::collections::HashMap;

/// Generate test contributions for benchmarking
fn generate_contributions(count: usize) -> Vec<Contribution> {
    (0..count)
        .map(|i| {
            let bucket = match i % 4 {
                0 => Bucket::Flat,
                1 => Bucket::Mult,
                2 => Bucket::PostAdd,
                _ => Bucket::Override,
            };
            Contribution::new(
                format!("stat_{}", i % 20), // 20 different stat names
                bucket,
                (i as f64) * 0.1 + 1.0, // Values from 1.0 to count*0.1+1.0
                format!("source_{}", i % 10) // 10 different sources
            )
        })
        .collect()
}

/// Generate test caps for benchmarking
fn generate_caps(count: usize) -> HashMap<String, Caps> {
    (0..count)
        .map(|i| {
            let stat_name = format!("stat_{}", i % 20);
            let min = (i as f64) * 0.5;
            let max = (i as f64) * 0.5 + 100.0;
            (stat_name, Caps::new(min, max))
        })
        .collect()
}

/// Benchmark contribution validation
pub pub fn bench_contribution_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("contribution_validation");
    
    for count in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("validate_contributions", count), count, |b, &count| {
            let contributions = generate_contributions(count);
            
            b.iter(|| {
                let result = validate_contributions(&contributions);
                black_box(result)
            })
        });
    }
    
    group.finish();
}

/// Benchmark bucket ordering
pub fn bench_bucket_ordering(c: &mut Criterion) {
    let mut group = c.benchmark_group("bucket_ordering");
    
    for count in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("get_bucket_processing_order", count), count, |b, &_count| {
            b.iter(|| {
                let order = get_bucket_processing_order();
                black_box(order)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("group_contributions_by_bucket", count), count, |b, &count| {
            let contributions = generate_contributions(count);
            
            b.iter(|| {
                let grouped = group_contributions_by_bucket(&contributions);
                black_box(grouped)
            })
        });
    }
    
    group.finish();
}

/// Benchmark contribution processing with different bucket distributions
pub fn bench_contribution_processing_distributions(c: &mut Criterion) {
    let mut group = c.benchmark_group("contribution_processing_distributions");
    
    let distributions = [
        ("uniform", vec![Bucket::Flat, Bucket::Mult, Bucket::PostAdd, Bucket::Override]),
        ("flat_heavy", vec![Bucket::Flat, Bucket::Flat, Bucket::Flat, Bucket::Mult]),
        ("mult_heavy", vec![Bucket::Mult, Bucket::Mult, Bucket::Mult, Bucket::Flat]),
        ("mixed", vec![Bucket::Flat, Bucket::Mult, Bucket::PostAdd, Bucket::Override, Bucket::Flat, Bucket::Mult]),
    ];
    
    for count in [1000, 10000].iter() {
        for (dist_name, buckets) in &distributions {
            group.throughput(Throughput::Elements(*count as u64));
            
            group.bench_with_input(
                BenchmarkId::new(format!("process_{}", dist_name), count), 
                count, 
                |b, &count| {
                    let contributions: Vec<Contribution> = (0..count)
                        .map(|i| {
                            let bucket = buckets[i % buckets.len()];
                            Contribution::new(
                                format!("stat_{}", i % 20),
                                bucket,
                                (i as f64) * 0.1 + 1.0,
                                format!("source_{}", i % 10)
                            )
                        })
                        .collect();
                    
                    let caps = generate_caps(20); // 20 different stat caps
                    let effective_caps = HashMap::new(); // No effective caps for this test
                    
                    b.iter(|| {
                        let result = process_contributions_in_order(contributions.clone(), 0.0, None);
                        black_box(result)
                    })
                }
            );
        }
    }
    
    group.finish();
}

/// Benchmark contribution processing with clamping
pub fn bench_contribution_processing_with_clamping(c: &mut Criterion) {
    let mut group = c.benchmark_group("contribution_processing_with_clamping");
    
    for count in [1000, 10000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("process_with_caps", count), count, |b, &count| {
            let contributions = generate_contributions(count);
            let caps = generate_caps(20); // 20 different stat caps
            let effective_caps = HashMap::new();
            
            b.iter(|| {
                let result = process_contributions_in_order(contributions.clone(), 0.0, None);
                black_box(result)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("process_with_effective_caps", count), count, |b, &count| {
            let contributions = generate_contributions(count);
            let caps = generate_caps(20);
            let effective_caps = generate_caps(20); // Same caps as effective caps
            
            b.iter(|| {
                let result = process_contributions_in_order(contributions.clone(), 0.0, None);
                black_box(result)
            })
        });
    }
    
    group.finish();
}

/// Benchmark contribution processing with different stat name distributions
pub fn bench_contribution_processing_stat_distributions(c: &mut Criterion) {
    let mut group = c.benchmark_group("contribution_processing_stat_distributions");
    
    let stat_distributions = [
        ("few_stats", 5),    // 5 different stat names
        ("many_stats", 50),  // 50 different stat names
        ("very_many_stats", 200), // 200 different stat names
    ];
    
    for count in [1000, 10000].iter() {
        for (dist_name, stat_count) in &stat_distributions {
            group.throughput(Throughput::Elements(*count as u64));
            
            group.bench_with_input(
                BenchmarkId::new(format!("process_{}", dist_name), count), 
                count, 
                |b, &count| {
                    let contributions: Vec<Contribution> = (0..count)
                        .map(|i| {
                            let bucket = match i % 4 {
                                0 => Bucket::Flat,
                                1 => Bucket::Mult,
                                2 => Bucket::PostAdd,
                                _ => Bucket::Override,
                            };
                            Contribution::new(
                                format!("stat_{}", i % stat_count),
                                bucket,
                                (i as f64) * 0.1 + 1.0,
                                format!("source_{}", i % 10)
                            )
                        })
                        .collect();
                    
                    let caps = generate_caps(*stat_count);
                    let effective_caps = HashMap::new();
                    
                    b.iter(|| {
                        let result = process_contributions_in_order(contributions.clone(), 0.0, None);
                        black_box(result)
                    })
                }
            );
        }
    }
    
    group.finish();
}

/// Benchmark contribution processing with different value ranges
pub fn bench_contribution_processing_value_ranges(c: &mut Criterion) {
    let mut group = c.benchmark_group("contribution_processing_value_ranges");
    
    let value_ranges = [
        ("small_values", 0.01, 1.0),
        ("medium_values", 1.0, 100.0),
        ("large_values", 100.0, 10000.0),
        ("mixed_values", 0.001, 100000.0),
    ];
    
    for count in [1000, 10000].iter() {
        for (range_name, min_val, max_val) in &value_ranges {
            group.throughput(Throughput::Elements(*count as u64));
            
            group.bench_with_input(
                BenchmarkId::new(format!("process_{}", range_name), count), 
                count, 
                |b, &count| {
                    let contributions: Vec<Contribution> = (0..count)
                        .map(|i| {
                            let bucket = match i % 4 {
                                0 => Bucket::Flat,
                                1 => Bucket::Mult,
                                2 => Bucket::PostAdd,
                                _ => Bucket::Override,
                            };
                            let value = min_val + (i as f64 / count as f64) * (max_val - min_val);
                            Contribution::new(
                                format!("stat_{}", i % 20),
                                bucket,
                                value,
                                format!("source_{}", i % 10)
                            )
                        })
                        .collect();
                    
                    let caps = generate_caps(20);
                    let effective_caps = HashMap::new();
                    
                    b.iter(|| {
                        let result = process_contributions_in_order(contributions.clone(), 0.0, None);
                        black_box(result)
                    })
                }
            );
        }
    }
    
    group.finish();
}

/// Benchmark contribution processing with different source distributions
pub fn bench_contribution_processing_source_distributions(c: &mut Criterion) {
    let mut group = c.benchmark_group("contribution_processing_source_distributions");
    
    let source_distributions = [
        ("few_sources", 5),    // 5 different sources
        ("many_sources", 50),  // 50 different sources
        ("very_many_sources", 200), // 200 different sources
    ];
    
    for count in [1000, 10000].iter() {
        for (dist_name, source_count) in &source_distributions {
            group.throughput(Throughput::Elements(*count as u64));
            
            group.bench_with_input(
                BenchmarkId::new(format!("process_{}", dist_name), count), 
                count, 
                |b, &count| {
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
                                format!("source_{}", i % source_count)
                            )
                        })
                        .collect();
                    
                    let caps = generate_caps(20);
                    let effective_caps = HashMap::new();
                    
                    b.iter(|| {
                        let result = process_contributions_in_order(contributions.clone(), 0.0, None);
                        black_box(result)
                    })
                }
            );
        }
    }
    
    group.finish();
}

/// Benchmark contribution processing with extra buckets feature
#[cfg(feature = "extra_buckets")]
pub fn bench_contribution_processing_extra_buckets(c: &mut Criterion) {
    let mut group = c.benchmark_group("contribution_processing_extra_buckets");
    
    for count in [1000, 10000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        
        group.bench_with_input(BenchmarkId::new("process_with_extra_buckets", count), count, |b, &count| {
            let contributions: Vec<Contribution> = (0..count)
                .map(|i| {
                    let bucket = match i % 7 {
                        0 => Bucket::Flat,
                        1 => Bucket::Mult,
                        2 => Bucket::PostAdd,
                        3 => Bucket::Override,
                        4 => Bucket::Exponential,
                        5 => Bucket::Logarithmic,
                        _ => Bucket::Conditional,
                    };
                    Contribution::new(
                        format!("stat_{}", i % 20),
                        bucket,
                        (i as f64) * 0.1 + 1.0,
                        format!("source_{}", i % 10)
                    )
                })
                .collect();
            
            let caps = generate_caps(20);
            let effective_caps = HashMap::new();
            
            b.iter(|| {
                let result = process_contributions_in_order(contributions.clone(), 0.0, None);
                black_box(result)
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_contribution_validation,
    bench_bucket_ordering,
    bench_contribution_processing_distributions,
    bench_contribution_processing_with_clamping,
    bench_contribution_processing_stat_distributions,
    bench_contribution_processing_value_ranges,
    bench_contribution_processing_source_distributions,
);

#[cfg(feature = "extra_buckets")]
criterion_group!(
    extra_bucket_benches,
    bench_contribution_processing_extra_buckets
);

#[cfg(feature = "extra_buckets")]
criterion_main!(benches, extra_bucket_benches);

#[cfg(not(feature = "extra_buckets"))]
criterion_main!(benches);
