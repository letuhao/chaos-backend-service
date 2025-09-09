//! Main Benchmark Suite
//! 
//! This module runs all benchmarks for the actor-core system.

use criterion::{criterion_group, criterion_main, Criterion};

// Import all benchmark modules
mod actor_benchmarks;
mod bucket_processor_benchmarks;
mod registry_loader_benchmarks;
mod criterion_config;

/// Run all actor-related benchmarks
fn run_actor_benchmarks(c: &mut Criterion) {
    actor_benchmarks::bench_actor_operations(c);
    actor_benchmarks::bench_caps_operations(c);
    actor_benchmarks::bench_contribution_processing(c);
    actor_benchmarks::bench_aggregation_performance(c);
    actor_benchmarks::bench_cache_performance(c);
    actor_benchmarks::bench_memory_usage(c);
    actor_benchmarks::bench_concurrent_operations(c);
    actor_benchmarks::bench_snapshot_operations(c);
}

/// Run all bucket processor benchmarks
fn run_bucket_processor_benchmarks(c: &mut Criterion) {
    bucket_processor_benchmarks::bench_contribution_validation(c);
    bucket_processor_benchmarks::bench_bucket_ordering(c);
    bucket_processor_benchmarks::bench_contribution_processing_distributions(c);
    bucket_processor_benchmarks::bench_contribution_processing_with_clamping(c);
    bucket_processor_benchmarks::bench_contribution_processing_stat_distributions(c);
    bucket_processor_benchmarks::bench_contribution_processing_value_ranges(c);
    bucket_processor_benchmarks::bench_contribution_processing_source_distributions(c);
    
    #[cfg(feature = "extra_buckets")]
    bucket_processor_benchmarks::bench_contribution_processing_extra_buckets(c);
}

/// Run all registry loader benchmarks
fn run_registry_loader_benchmarks(c: &mut Criterion) {
    registry_loader_benchmarks::bench_cap_layers_yaml_loading(c);
    registry_loader_benchmarks::bench_combiner_yaml_loading(c);
    registry_loader_benchmarks::bench_combined_loading(c);
    registry_loader_benchmarks::bench_yaml_parsing(c);
    registry_loader_benchmarks::bench_json_parsing(c);
    registry_loader_benchmarks::bench_registry_validation(c);
    registry_loader_benchmarks::bench_registry_operations(c);
    registry_loader_benchmarks::bench_file_io_operations(c);
}

/// Run a subset of benchmarks for quick testing
fn run_quick_benchmarks(c: &mut Criterion) {
    // Only run the most important benchmarks for quick feedback
    actor_benchmarks::bench_actor_operations(c);
    actor_benchmarks::bench_caps_operations(c);
    bucket_processor_benchmarks::bench_contribution_processing_distributions(c);
    registry_loader_benchmarks::bench_cap_layers_yaml_loading(c);
}

/// Run comprehensive benchmarks for detailed analysis
fn run_comprehensive_benchmarks(c: &mut Criterion) {
    run_actor_benchmarks(c);
    run_bucket_processor_benchmarks(c);
    run_registry_loader_benchmarks(c);
}

// Define benchmark groups
criterion_group!(
    name = actor_benches;
    config = criterion_config::create_criterion();
    targets = run_actor_benchmarks
);

criterion_group!(
    name = bucket_processor_benches;
    config = criterion_config::create_criterion();
    targets = run_bucket_processor_benchmarks
);

criterion_group!(
    name = registry_loader_benches;
    config = criterion_config::create_criterion();
    targets = run_registry_loader_benchmarks
);

criterion_group!(
    name = quick_benches;
    config = criterion_config::create_fast_criterion();
    targets = run_quick_benchmarks
);

criterion_group!(
    name = comprehensive_benches;
    config = criterion_config::create_thorough_criterion();
    targets = run_comprehensive_benchmarks
);

// Main benchmark runner
criterion_main!(
    actor_benches,
    bucket_processor_benches,
    registry_loader_benches,
    quick_benches,
    comprehensive_benches
);
