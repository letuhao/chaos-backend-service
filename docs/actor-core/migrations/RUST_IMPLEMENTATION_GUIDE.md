# Actor Core v3: Rust Implementation Guide

**Version:** 1.0  
**Date:** 2025-01-27  
**Status:** Technical Specification  

## Overview

This document provides detailed technical specifications for implementing Actor Core v3 in Rust, including code examples, architectural patterns, and performance optimizations.

## Core Dependencies

```toml
[dependencies]
# Async Runtime
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"

# Concurrency
dashmap = "5.0"
crossbeam = "0.8"
parking_lot = "0.12"

# Caching
moka = "0.12"
memmap2 = "0.9"

# Error Handling
anyhow = "1.0"
thiserror = "1.0"

# Monitoring
tracing = "0.1"
prometheus = "0.13"

# Testing
criterion = "0.5"
proptest = "1.0"
```

## Type System Implementation

### Actor Type
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: String,
    pub name: String,
    pub race: String,
    pub lifespan: i64,
    pub age: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
    pub subsystems: Vec<Subsystem>,
    pub data: HashMap<String, serde_json::Value>,
}

impl Actor {
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.name.is_empty() && self.version >= 0
    }

    pub fn update_version(&mut self) {
        self.version += 1;
        self.updated_at = Utc::now();
    }
}
```

### Contribution Types
```rust
use crate::enums::{Bucket, CapMode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
    pub dimension: String,
    pub bucket: Bucket,
    pub value: f64,
    pub system: String,
    pub priority: Option<i64>,
    pub tags: Option<HashMap<String, String>>,
}

impl Contribution {
    pub fn is_valid(&self) -> bool {
        !self.dimension.is_empty() 
            && !self.system.is_empty() 
            && self.priority.unwrap_or(0) >= 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapContribution {
    pub system: String,
    pub dimension: String,
    pub mode: CapMode,
    pub kind: String,
    pub value: f64,
    pub priority: Option<i64>,
    pub scope: Option<String>,
    pub realm: Option<String>,
    pub tags: Option<HashMap<String, String>>,
}
```

## Trait-Based Architecture

### Core Traits
```rust
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait Subsystem: Send + Sync {
    fn system_id(&self) -> &str;
    fn priority(&self) -> i64;
    async fn contribute(&self, actor: &Actor) -> Result<SubsystemOutput>;
}

#[async_trait]
pub trait Aggregator: Send + Sync {
    async fn resolve(&self, actor: &Actor) -> Result<Snapshot>;
    async fn resolve_batch(&self, actors: &[Actor]) -> Result<Vec<Snapshot>>;
    fn get_cached_snapshot(&self, actor_id: &str) -> Option<Snapshot>;
    fn invalidate_cache(&self, actor_id: &str);
}

#[async_trait]
pub trait CapsProvider: Send + Sync {
    async fn effective_caps_within_layer(
        &self, 
        actor: &Actor, 
        outputs: &[SubsystemOutput], 
        layer: &str
    ) -> Result<EffectiveCaps>;
    
    async fn effective_caps_across_layers(
        &self, 
        actor: &Actor, 
        outputs: &[SubsystemOutput]
    ) -> Result<EffectiveCaps>;
}
```

## Performance-Optimized Cache

### Lock-Free L1 Cache
```rust
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct LockFreeL1Cache {
    cache: DashMap<String, CacheEntry>,
    max_size: usize,
    stats: Arc<CacheStats>,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    value: Arc<serde_json::Value>,
    expires_at: Instant,
    access_count: AtomicU64,
    size: usize,
}

#[derive(Debug)]
struct CacheStats {
    hits: AtomicU64,
    misses: AtomicU64,
    sets: AtomicU64,
    memory_usage: AtomicU64,
}

impl LockFreeL1Cache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: DashMap::new(),
            max_size,
            stats: Arc::new(CacheStats {
                hits: AtomicU64::new(0),
                misses: AtomicU64::new(0),
                sets: AtomicU64::new(0),
                memory_usage: AtomicU64::new(0),
            }),
        }
    }

    pub fn get(&self, key: &str) -> Option<Arc<serde_json::Value>> {
        if let Some(mut entry) = self.cache.get_mut(key) {
            if entry.expires_at > Instant::now() {
                entry.access_count.fetch_add(1, Ordering::Relaxed);
                self.stats.hits.fetch_add(1, Ordering::Relaxed);
                return Some(entry.value.clone());
            } else {
                self.cache.remove(key);
            }
        }
        self.stats.misses.fetch_add(1, Ordering::Relaxed);
        None
    }

    pub fn set(&self, key: String, value: Arc<serde_json::Value>, ttl: Duration) -> Result<()> {
        if self.cache.len() >= self.max_size {
            self.evict()?;
        }

        let entry = CacheEntry {
            value,
            expires_at: Instant::now() + ttl,
            access_count: AtomicU64::new(1),
            size: key.len() + std::mem::size_of::<CacheEntry>(),
        };

        self.cache.insert(key, entry);
        self.stats.sets.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn evict(&self) -> Result<()> {
        // Implement LRU eviction
        let mut oldest_key = None;
        let mut oldest_time = Instant::now();

        for entry in self.cache.iter() {
            if entry.created_at < oldest_time {
                oldest_time = entry.created_at;
                oldest_key = Some(entry.key().clone());
            }
        }

        if let Some(key) = oldest_key {
            self.cache.remove(&key);
        }

        Ok(())
    }
}
```

## Async Aggregator Implementation

```rust
use tokio::task::JoinSet;
use std::sync::Arc;

pub struct AsyncAggregator {
    combiner_registry: Arc<CombinerRegistry>,
    caps_provider: Arc<CapsProvider>,
    plugin_registry: Arc<PluginRegistry>,
    cache: Arc<MultiLayerCache>,
}

#[async_trait]
impl Aggregator for AsyncAggregator {
    async fn resolve(&self, actor: &Actor) -> Result<Snapshot> {
        // Check cache first
        if let Some(cached) = self.cache.get(&actor.id) {
            return Ok(cached);
        }

        // Get subsystems
        let subsystems = self.plugin_registry.get_by_priority();
        
        // Process subsystems concurrently
        let mut join_set = JoinSet::new();
        for subsystem in subsystems {
            let actor = actor.clone();
            let subsystem = subsystem.clone();
            join_set.spawn(async move {
                subsystem.contribute(&actor).await
            });
        }

        // Collect outputs
        let mut outputs = Vec::new();
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(output)) => outputs.push(output),
                Ok(Err(e)) => tracing::warn!("Subsystem error: {}", e),
                Err(e) => tracing::warn!("Task error: {}", e),
            }
        }

        // Calculate effective caps
        let effective_caps = self.caps_provider
            .effective_caps_across_layers(actor, &outputs)
            .await?;

        // Aggregate stats
        let primary_stats = self.aggregate_primary_stats(&outputs, &effective_caps).await?;
        let derived_stats = self.aggregate_derived_stats(&outputs, &primary_stats, &effective_caps).await?;

        // Create snapshot
        let snapshot = Snapshot {
            actor_id: actor.id.clone(),
            primary: primary_stats,
            derived: derived_stats,
            caps_used: effective_caps,
            version: actor.version,
            created_at: Utc::now(),
        };

        // Cache result
        self.cache.set(actor.id.clone(), Arc::new(snapshot.clone()), Duration::from_secs(3600))?;

        Ok(snapshot)
    }
}
```

## Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ActorCoreError {
    #[error("Invalid actor: {0}")]
    InvalidActor(String),
    
    #[error("Subsystem error: {0}")]
    SubsystemError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Registry error: {0}")]
    RegistryError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ActorCoreError>;
```

## Testing Framework

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_aggregator_resolve() {
        let aggregator = create_test_aggregator().await;
        let actor = create_test_actor();
        
        let snapshot = aggregator.resolve(&actor).await.unwrap();
        
        assert_eq!(snapshot.actor_id, actor.id);
        assert!(!snapshot.primary.is_empty());
    }

    #[test]
    fn test_contribution_validation() {
        let contribution = Contribution {
            dimension: "strength".to_string(),
            bucket: Bucket::Flat,
            value: 100.0,
            system: "test".to_string(),
            priority: Some(100),
            tags: None,
        };
        
        assert!(contribution.is_valid());
    }
}
```

### Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_contribution_roundtrip(contribution in any::<Contribution>()) {
        let json = serde_json::to_string(&contribution).unwrap();
        let deserialized: Contribution = serde_json::from_str(&json).unwrap();
        assert_eq!(contribution, deserialized);
    }
}
```

### Performance Benchmarks
```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_aggregator_resolve(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let aggregator = rt.block_on(create_test_aggregator());
    let actor = create_test_actor();

    c.bench_function("aggregator_resolve", |b| {
        b.to_async(&rt).iter(|| {
            aggregator.resolve(&actor)
        })
    });
}

criterion_group!(benches, benchmark_aggregator_resolve);
criterion_main!(benches);
```

## Configuration Management

```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorCoreConfig {
    pub cache: CacheConfig,
    pub registry: RegistryConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub l1_max_size: usize,
    pub l2_cache_path: String,
    pub l3_cache_dir: String,
    pub ttl: u64,
}

impl ActorCoreConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
```

## Monitoring and Metrics

```rust
use prometheus::{Counter, Histogram, Registry};

pub struct Metrics {
    pub requests_total: Counter,
    pub request_duration: Histogram,
    pub cache_hits: Counter,
    pub cache_misses: Counter,
    pub errors_total: Counter,
}

impl Metrics {
    pub fn new(registry: &Registry) -> Self {
        Self {
            requests_total: Counter::new("actor_core_requests_total", "Total requests")
                .unwrap(),
            request_duration: Histogram::with_opts(
                HistogramOpts::new("actor_core_request_duration_seconds", "Request duration")
            ).unwrap(),
            cache_hits: Counter::new("actor_core_cache_hits_total", "Cache hits")
                .unwrap(),
            cache_misses: Counter::new("actor_core_cache_misses_total", "Cache misses")
                .unwrap(),
            errors_total: Counter::new("actor_core_errors_total", "Total errors")
                .unwrap(),
        }
    }
}
```

This implementation guide provides the technical foundation for migrating Actor Core v3 from Go to Rust, emphasizing performance, safety, and maintainability while maintaining API compatibility.
