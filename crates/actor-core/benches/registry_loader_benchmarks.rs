//! Registry Loader Benchmarks
//! 
//! This module contains benchmarks for the registry loader functionality,
//! measuring performance of YAML/JSON loading and validation.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use actor_core::registry::loader::*;
use actor_core::interfaces::{CapLayerRegistry, CombinerRegistry};
use std::path::Path;
use tempfile::NamedTempFile;
use std::io::Write;

/// Generate test cap layers YAML content
fn generate_cap_layers_yaml(layer_count: usize) -> String {
    let mut content = String::from("cap_layers:\n");
    
    for i in 0..layer_count {
        content.push_str(&format!(
            "  - name: layer_{}\n    priority: {}\n    cap_mode: BASELINE\n",
            i, i * 10
        ));
    }
    
    content.push_str("across_layer_policy: STRICT\n");
    content
}

/// Generate test combiner YAML content
fn generate_combiner_yaml(rule_count: usize) -> String {
    let mut content = String::from("combiner_rules:\n");
    
    for i in 0..rule_count {
        content.push_str(&format!(
            "  - name: rule_{}\n    bucket_order: [Flat, Mult, PostAdd, Override]\n    clamp: true\n",
            i
        ));
    }
    
    content
}

/// Benchmark cap layers YAML loading
pub fn bench_cap_layers_yaml_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("cap_layers_yaml_loading");
    
    for layer_count in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*layer_count as u64));
        
        group.bench_with_input(BenchmarkId::new("load_cap_layers", layer_count), layer_count, |b, &layer_count| {
            let yaml_content = generate_cap_layers_yaml(layer_count);
            let mut temp_file = NamedTempFile::new().unwrap();
            temp_file.write_all(yaml_content.as_bytes()).unwrap();
            let temp_path = temp_file.path();
            
            b.iter(|| {
                let result = load_cap_layers(temp_path);
                black_box(result)
            })
        });
    }
    
    group.finish();
}

/// Benchmark combiner YAML loading
pub fn bench_combiner_yaml_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("combiner_yaml_loading");
    
    for rule_count in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*rule_count as u64));
        
        group.bench_with_input(BenchmarkId::new("load_combiner", rule_count), rule_count, |b, &rule_count| {
            let yaml_content = generate_combiner_yaml(rule_count);
            let mut temp_file = NamedTempFile::new().unwrap();
            temp_file.write_all(yaml_content.as_bytes()).unwrap();
            let temp_path = temp_file.path();
            
            b.iter(|| {
                let result = load_combiner(temp_path);
                black_box(result)
            })
        });
    }
    
    group.finish();
}

/// Benchmark combined loading
pub fn bench_combined_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("combined_loading");
    
    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("load_all", size), size, |b, &size| {
            let cap_layers_content = generate_cap_layers_yaml(size);
            let combiner_content = generate_combiner_yaml(size);
            
            let mut cap_layers_file = NamedTempFile::new().unwrap();
            cap_layers_file.write_all(cap_layers_content.as_bytes()).unwrap();
            let cap_layers_path = cap_layers_file.path();
            
            let mut combiner_file = NamedTempFile::new().unwrap();
            combiner_file.write_all(combiner_content.as_bytes()).unwrap();
            let combiner_path = combiner_file.path();
            
            b.iter(|| {
                let result = load_all(cap_layers_path, combiner_path);
                black_box(result)
            })
        });
    }
    
    group.finish();
}

/// Benchmark YAML parsing performance
pub fn bench_yaml_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("yaml_parsing");
    
    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("parse_yaml", size), size, |b, &size| {
            let yaml_content = generate_cap_layers_yaml(size);
            
            b.iter(|| {
                let result: Result<serde_yaml::Value, _> = serde_yaml::from_str(&yaml_content);
                black_box(result)
            })
        });
    }
    
    group.finish();
}

/// Benchmark JSON parsing performance
pub fn bench_json_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_parsing");
    
    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("parse_json", size), size, |b, &size| {
            let yaml_content = generate_cap_layers_yaml(size);
            let json_content = serde_yaml::from_str::<serde_yaml::Value>(&yaml_content)
                .unwrap()
                .to_string();
            
            b.iter(|| {
                let result: Result<serde_json::Value, _> = serde_json::from_str(&json_content);
                black_box(result)
            })
        });
    }
    
    group.finish();
}

/// Benchmark registry validation
pub fn bench_registry_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_validation");
    
    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("validate_cap_layers", size), size, |b, &size| {
            let yaml_content = generate_cap_layers_yaml(size);
            let mut temp_file = NamedTempFile::new().unwrap();
            temp_file.write_all(yaml_content.as_bytes()).unwrap();
            let temp_path = temp_file.path();
            
            let registry = load_cap_layers(temp_path).unwrap();
            
            b.iter(|| {
                let result = registry.validate();
                black_box(result)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("validate_combiner", size), size, |b, &size| {
            let yaml_content = generate_combiner_yaml(size);
            let mut temp_file = NamedTempFile::new().unwrap();
            temp_file.write_all(yaml_content.as_bytes()).unwrap();
            let temp_path = temp_file.path();
            
            let registry = load_combiner(temp_path).unwrap();
            
            b.iter(|| {
                let result = registry.validate();
                black_box(result)
            })
        });
    }
    
    group.finish();
}

/// Benchmark registry operations
pub fn bench_registry_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_operations");
    
    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("cap_layers_operations", size), size, |b, &size| {
            let yaml_content = generate_cap_layers_yaml(size);
            let mut temp_file = NamedTempFile::new().unwrap();
            temp_file.write_all(yaml_content.as_bytes()).unwrap();
            let temp_path = temp_file.path();
            
            let registry = load_cap_layers(temp_path).unwrap();
            
            b.iter(|| {
                let _layer_order = registry.get_layer_order();
                let _policy = registry.get_across_layer_policy();
                
                for i in 0..size {
                    let _layer = registry.get_layer(&format!("layer_{}", i));
                }
                
                black_box(&registry)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("combiner_operations", size), size, |b, &size| {
            let yaml_content = generate_combiner_yaml(size);
            let mut temp_file = NamedTempFile::new().unwrap();
            temp_file.write_all(yaml_content.as_bytes()).unwrap();
            let temp_path = temp_file.path();
            
            let registry = load_combiner(temp_path).unwrap();
            
            b.iter(|| {
                for i in 0..size {
                    let _rule = registry.get_rule(&format!("rule_{}", i));
                }
                
                black_box(&registry)
            })
        });
    }
    
    group.finish();
}

/// Benchmark file I/O operations
pub fn bench_file_io_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_io_operations");
    
    for size in [1000, 10000, 100000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("read_large_file", size), size, |b, &size| {
            let yaml_content = generate_cap_layers_yaml(size);
            let mut temp_file = NamedTempFile::new().unwrap();
            temp_file.write_all(yaml_content.as_bytes()).unwrap();
            let temp_path = temp_file.path();
            
            b.iter(|| {
                let result = std::fs::read_to_string(temp_path);
                black_box(result)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("write_large_file", size), size, |b, &size| {
            let yaml_content = generate_cap_layers_yaml(size);
            
            b.iter(|| {
                let mut temp_file = NamedTempFile::new().unwrap();
                let result = temp_file.write_all(yaml_content.as_bytes());
                black_box(result)
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_cap_layers_yaml_loading,
    bench_combiner_yaml_loading,
    bench_combined_loading,
    bench_yaml_parsing,
    bench_json_parsing,
    bench_registry_validation,
    bench_registry_operations,
    bench_file_io_operations
);

criterion_main!(benches);
