//! Memory pool system for high-performance actor core operations.
//!
//! This module provides memory pools for frequently allocated objects
//! to reduce garbage collection pressure and improve performance.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Memory pool manager that manages multiple specialized pools.
pub struct MemoryPoolManager {
    /// Pool for Actor objects
    actor_pool: Arc<ActorPool>,
    /// Pool for SubsystemOutput objects
    subsystem_output_pool: Arc<SubsystemOutputPool>,
    /// Pool for Contribution objects
    contribution_pool: Arc<ContributionPool>,
    /// Pool for Snapshot objects
    snapshot_pool: Arc<SnapshotPool>,
    /// Global statistics
    stats: Arc<PoolStats>,
}

/// Statistics for memory pool usage.
#[derive(Debug, Default)]
pub struct PoolStats {
    /// Total allocations
    pub total_allocations: AtomicUsize,
    /// Total deallocations
    pub total_deallocations: AtomicUsize,
    /// Current pool size
    pub current_pool_size: AtomicUsize,
    /// Peak pool size
    pub peak_pool_size: AtomicUsize,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Average allocation time
    pub avg_allocation_time: Duration,
}

impl Clone for PoolStats {
    fn clone(&self) -> Self {
        Self {
            total_allocations: AtomicUsize::new(self.total_allocations.load(Ordering::Relaxed)),
            total_deallocations: AtomicUsize::new(self.total_deallocations.load(Ordering::Relaxed)),
            current_pool_size: AtomicUsize::new(self.current_pool_size.load(Ordering::Relaxed)),
            peak_pool_size: AtomicUsize::new(self.peak_pool_size.load(Ordering::Relaxed)),
            cache_hit_rate: self.cache_hit_rate,
            avg_allocation_time: self.avg_allocation_time,
        }
    }
}

/// Generic memory pool trait.
pub trait MemoryPool<T>: Send + Sync {
    /// Get an object from the pool.
    fn get(&self) -> PooledObject<T>;
    
    /// Return an object to the pool.
    fn return_object(&self, obj: PooledObject<T>);
    
    /// Get pool statistics.
    fn get_stats(&self) -> PoolStats;
    
    /// Clear the pool.
    fn clear(&self);
    
    /// Get the current size of the pool.
    fn size(&self) -> usize;
    
    /// Get the maximum size of the pool.
    fn max_size(&self) -> usize;
}

/// A pooled object that automatically returns itself to the pool when dropped.
pub struct PooledObject<T> {
    inner: Option<T>,
    pool: Arc<dyn MemoryPool<T>>,
}

impl<T> PooledObject<T> {
    /// Get a reference to the inner object.
    pub fn as_ref(&self) -> &T {
        self.inner.as_ref().unwrap()
    }
    
    /// Get a mutable reference to the inner object.
    pub fn as_mut(&mut self) -> &mut T {
        self.inner.as_mut().unwrap()
    }
    
    /// Take ownership of the inner object.
    pub fn take(mut self) -> T {
        self.inner.take().unwrap()
    }
}

impl<T> Drop for PooledObject<T> {
    fn drop(&mut self) {
        if let Some(obj) = self.inner.take() {
            self.pool.return_object(PooledObject {
                inner: Some(obj),
                pool: self.pool.clone(),
            });
        }
    }
}

impl<T> std::ops::Deref for PooledObject<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> std::ops::DerefMut for PooledObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

/// Actor memory pool.
pub struct ActorPool {
    objects: Mutex<VecDeque<crate::types::Actor>>,
    max_size: usize,
    stats: Arc<PoolStats>,
}

impl ActorPool {
    /// Create a new actor pool.
    pub fn new(max_size: usize) -> Self {
        Self {
            objects: Mutex::new(VecDeque::new()),
            max_size,
            stats: Arc::new(PoolStats::default()),
        }
    }
}

impl MemoryPool<crate::types::Actor> for ActorPool {
    fn get(&self) -> PooledObject<crate::types::Actor> {
        let start_time = Instant::now();
        
        let mut objects = self.objects.lock().unwrap();
        
        if let Some(obj) = objects.pop_front() {
            self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
            self.stats.current_pool_size.fetch_sub(1, Ordering::Relaxed);
            
            let _allocation_time = start_time.elapsed();
            // Note: In a real implementation, this would use atomic operations or a mutex
            // For now, we'll skip updating the average time to avoid Arc issues
            
            PooledObject {
                inner: Some(obj),
                pool: Arc::new(self.clone()),
            }
        } else {
            // Pool is empty, create new object
            self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
            
            PooledObject {
                inner: Some(crate::types::Actor::new(
                    String::new(),
                    String::new(),
                )),
                pool: Arc::new(self.clone()),
            }
        }
    }
    
    fn return_object(&self, mut obj: PooledObject<crate::types::Actor>) {
        if let Some(actor) = obj.inner.take() {
            let mut objects = self.objects.lock().unwrap();
            
            if objects.len() < self.max_size {
                // Reset the actor to a clean state
                let mut clean_actor = actor;
                clean_actor.name.clear();
                clean_actor.race.clear();
                clean_actor.subsystems.clear();
                clean_actor.data.clear();
                clean_actor.version = 1;
                
                objects.push_back(clean_actor);
                self.stats.total_deallocations.fetch_add(1, Ordering::Relaxed);
                self.stats.current_pool_size.fetch_add(1, Ordering::Relaxed);
                
                let current_size = self.stats.current_pool_size.load(Ordering::Relaxed);
                let peak_size = self.stats.peak_pool_size.load(Ordering::Relaxed);
                if current_size > peak_size {
                    self.stats.peak_pool_size.store(current_size, Ordering::Relaxed);
                }
            }
        }
    }
    
    fn get_stats(&self) -> PoolStats {
        PoolStats {
            total_allocations: AtomicUsize::new(self.stats.total_allocations.load(Ordering::Relaxed)),
            total_deallocations: AtomicUsize::new(self.stats.total_deallocations.load(Ordering::Relaxed)),
            current_pool_size: AtomicUsize::new(self.stats.current_pool_size.load(Ordering::Relaxed)),
            peak_pool_size: AtomicUsize::new(self.stats.peak_pool_size.load(Ordering::Relaxed)),
            cache_hit_rate: self.stats.cache_hit_rate,
            avg_allocation_time: self.stats.avg_allocation_time,
        }
    }
    
    fn clear(&self) {
        let mut objects = self.objects.lock().unwrap();
        objects.clear();
        self.stats.current_pool_size.store(0, Ordering::Relaxed);
    }
    
    fn size(&self) -> usize {
        self.objects.lock().unwrap().len()
    }
    
    fn max_size(&self) -> usize {
        self.max_size
    }
}

impl Clone for ActorPool {
    fn clone(&self) -> Self {
        Self {
            objects: Mutex::new(VecDeque::new()),
            max_size: self.max_size,
            stats: self.stats.clone(),
        }
    }
}

/// SubsystemOutput memory pool.
pub struct SubsystemOutputPool {
    objects: Mutex<VecDeque<crate::types::SubsystemOutput>>,
    max_size: usize,
    stats: Arc<PoolStats>,
}

impl SubsystemOutputPool {
    /// Create a new subsystem output pool.
    pub fn new(max_size: usize) -> Self {
        Self {
            objects: Mutex::new(VecDeque::new()),
            max_size,
            stats: Arc::new(PoolStats::default()),
        }
    }
}

impl MemoryPool<crate::types::SubsystemOutput> for SubsystemOutputPool {
    fn get(&self) -> PooledObject<crate::types::SubsystemOutput> {
        let start_time = Instant::now();
        
        let mut objects = self.objects.lock().unwrap();
        
        if let Some(obj) = objects.pop_front() {
            self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
            self.stats.current_pool_size.fetch_sub(1, Ordering::Relaxed);
            
            let _allocation_time = start_time.elapsed();
            // Note: In a real implementation, this would use atomic operations or a mutex
            // For now, we'll skip updating the average time to avoid Arc issues
            
            PooledObject {
                inner: Some(obj),
                pool: Arc::new(self.clone()),
            }
        } else {
            // Pool is empty, create new object
            self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
            
            PooledObject {
                inner: Some(crate::types::SubsystemOutput::new(
                    String::new(),
                )),
                pool: Arc::new(self.clone()),
            }
        }
    }
    
    fn return_object(&self, mut obj: PooledObject<crate::types::SubsystemOutput>) {
        if let Some(output) = obj.inner.take() {
            let mut objects = self.objects.lock().unwrap();
            
            if objects.len() < self.max_size {
                // Reset the output to a clean state
                let mut clean_output = output;
                clean_output.primary.clear();
                clean_output.derived.clear();
                clean_output.caps.clear();
                clean_output.context = None;
                clean_output.meta.data.clear();
                
                objects.push_back(clean_output);
                self.stats.total_deallocations.fetch_add(1, Ordering::Relaxed);
                self.stats.current_pool_size.fetch_add(1, Ordering::Relaxed);
                
                let current_size = self.stats.current_pool_size.load(Ordering::Relaxed);
                let peak_size = self.stats.peak_pool_size.load(Ordering::Relaxed);
                if current_size > peak_size {
                    self.stats.peak_pool_size.store(current_size, Ordering::Relaxed);
                }
            }
        }
    }
    
    fn get_stats(&self) -> PoolStats {
        PoolStats {
            total_allocations: AtomicUsize::new(self.stats.total_allocations.load(Ordering::Relaxed)),
            total_deallocations: AtomicUsize::new(self.stats.total_deallocations.load(Ordering::Relaxed)),
            current_pool_size: AtomicUsize::new(self.stats.current_pool_size.load(Ordering::Relaxed)),
            peak_pool_size: AtomicUsize::new(self.stats.peak_pool_size.load(Ordering::Relaxed)),
            cache_hit_rate: self.stats.cache_hit_rate,
            avg_allocation_time: self.stats.avg_allocation_time,
        }
    }
    
    fn clear(&self) {
        let mut objects = self.objects.lock().unwrap();
        objects.clear();
        self.stats.current_pool_size.store(0, Ordering::Relaxed);
    }
    
    fn size(&self) -> usize {
        self.objects.lock().unwrap().len()
    }
    
    fn max_size(&self) -> usize {
        self.max_size
    }
}

impl Clone for SubsystemOutputPool {
    fn clone(&self) -> Self {
        Self {
            objects: Mutex::new(VecDeque::new()),
            max_size: self.max_size,
            stats: self.stats.clone(),
        }
    }
}

/// Contribution memory pool.
pub struct ContributionPool {
    objects: Mutex<VecDeque<crate::types::Contribution>>,
    max_size: usize,
    stats: Arc<PoolStats>,
}

impl ContributionPool {
    /// Create a new contribution pool.
    pub fn new(max_size: usize) -> Self {
        Self {
            objects: Mutex::new(VecDeque::new()),
            max_size,
            stats: Arc::new(PoolStats::default()),
        }
    }
}

impl MemoryPool<crate::types::Contribution> for ContributionPool {
    fn get(&self) -> PooledObject<crate::types::Contribution> {
        let start_time = Instant::now();
        
        let mut objects = self.objects.lock().unwrap();
        
        if let Some(obj) = objects.pop_front() {
            self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
            self.stats.current_pool_size.fetch_sub(1, Ordering::Relaxed);
            
            let _allocation_time = start_time.elapsed();
            // Note: In a real implementation, this would use atomic operations or a mutex
            // For now, we'll skip updating the average time to avoid Arc issues
            
            PooledObject {
                inner: Some(obj),
                pool: Arc::new(self.clone()),
            }
        } else {
            // Pool is empty, create new object
            self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
            
            PooledObject {
                inner: Some(crate::types::Contribution::new(
                    String::new(),
                    crate::enums::Bucket::Flat,
                    0.0,
                    String::new(),
                )),
                pool: Arc::new(self.clone()),
            }
        }
    }
    
    fn return_object(&self, mut obj: PooledObject<crate::types::Contribution>) {
        if let Some(contribution) = obj.inner.take() {
            let mut objects = self.objects.lock().unwrap();
            
            if objects.len() < self.max_size {
                // Reset the contribution to a clean state
                let mut clean_contribution = contribution;
                clean_contribution.dimension.clear();
                clean_contribution.value = 0.0;
                clean_contribution.system.clear();
                clean_contribution.priority = None;
                clean_contribution.tags = None;
                
                objects.push_back(clean_contribution);
                self.stats.total_deallocations.fetch_add(1, Ordering::Relaxed);
                self.stats.current_pool_size.fetch_add(1, Ordering::Relaxed);
                
                let current_size = self.stats.current_pool_size.load(Ordering::Relaxed);
                let peak_size = self.stats.peak_pool_size.load(Ordering::Relaxed);
                if current_size > peak_size {
                    self.stats.peak_pool_size.store(current_size, Ordering::Relaxed);
                }
            }
        }
    }
    
    fn get_stats(&self) -> PoolStats {
        PoolStats {
            total_allocations: AtomicUsize::new(self.stats.total_allocations.load(Ordering::Relaxed)),
            total_deallocations: AtomicUsize::new(self.stats.total_deallocations.load(Ordering::Relaxed)),
            current_pool_size: AtomicUsize::new(self.stats.current_pool_size.load(Ordering::Relaxed)),
            peak_pool_size: AtomicUsize::new(self.stats.peak_pool_size.load(Ordering::Relaxed)),
            cache_hit_rate: self.stats.cache_hit_rate,
            avg_allocation_time: self.stats.avg_allocation_time,
        }
    }
    
    fn clear(&self) {
        let mut objects = self.objects.lock().unwrap();
        objects.clear();
        self.stats.current_pool_size.store(0, Ordering::Relaxed);
    }
    
    fn size(&self) -> usize {
        self.objects.lock().unwrap().len()
    }
    
    fn max_size(&self) -> usize {
        self.max_size
    }
}

impl Clone for ContributionPool {
    fn clone(&self) -> Self {
        Self {
            objects: Mutex::new(VecDeque::new()),
            max_size: self.max_size,
            stats: self.stats.clone(),
        }
    }
}

/// Snapshot memory pool.
pub struct SnapshotPool {
    objects: Mutex<VecDeque<crate::types::Snapshot>>,
    max_size: usize,
    stats: Arc<PoolStats>,
}

impl SnapshotPool {
    /// Create a new snapshot pool.
    pub fn new(max_size: usize) -> Self {
        Self {
            objects: Mutex::new(VecDeque::new()),
            max_size,
            stats: Arc::new(PoolStats::default()),
        }
    }
}

impl MemoryPool<crate::types::Snapshot> for SnapshotPool {
    fn get(&self) -> PooledObject<crate::types::Snapshot> {
        let start_time = Instant::now();
        
        let mut objects = self.objects.lock().unwrap();
        
        if let Some(obj) = objects.pop_front() {
            self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
            self.stats.current_pool_size.fetch_sub(1, Ordering::Relaxed);
            
            let _allocation_time = start_time.elapsed();
            // Note: In a real implementation, this would use atomic operations or a mutex
            // For now, we'll skip updating the average time to avoid Arc issues
            
            PooledObject {
                inner: Some(obj),
                pool: Arc::new(self.clone()),
            }
        } else {
            // Pool is empty, create new object
            self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
            
            PooledObject {
                inner: Some(crate::types::Snapshot::new(
                    uuid::Uuid::new_v4(),
                    1,
                )),
                pool: Arc::new(self.clone()),
            }
        }
    }
    
    fn return_object(&self, mut obj: PooledObject<crate::types::Snapshot>) {
        if let Some(snapshot) = obj.inner.take() {
            let mut objects = self.objects.lock().unwrap();
            
            if objects.len() < self.max_size {
                // Reset the snapshot to a clean state
                let mut clean_snapshot = snapshot;
                clean_snapshot.primary.clear();
                clean_snapshot.derived.clear();
                clean_snapshot.caps_used.clear();
                clean_snapshot.subsystems_processed.clear();
                clean_snapshot.processing_time = None;
                clean_snapshot.metadata.clear();
                
                objects.push_back(clean_snapshot);
                self.stats.total_deallocations.fetch_add(1, Ordering::Relaxed);
                self.stats.current_pool_size.fetch_add(1, Ordering::Relaxed);
                
                let current_size = self.stats.current_pool_size.load(Ordering::Relaxed);
                let peak_size = self.stats.peak_pool_size.load(Ordering::Relaxed);
                if current_size > peak_size {
                    self.stats.peak_pool_size.store(current_size, Ordering::Relaxed);
                }
            }
        }
    }
    
    fn get_stats(&self) -> PoolStats {
        PoolStats {
            total_allocations: AtomicUsize::new(self.stats.total_allocations.load(Ordering::Relaxed)),
            total_deallocations: AtomicUsize::new(self.stats.total_deallocations.load(Ordering::Relaxed)),
            current_pool_size: AtomicUsize::new(self.stats.current_pool_size.load(Ordering::Relaxed)),
            peak_pool_size: AtomicUsize::new(self.stats.peak_pool_size.load(Ordering::Relaxed)),
            cache_hit_rate: self.stats.cache_hit_rate,
            avg_allocation_time: self.stats.avg_allocation_time,
        }
    }
    
    fn clear(&self) {
        let mut objects = self.objects.lock().unwrap();
        objects.clear();
        self.stats.current_pool_size.store(0, Ordering::Relaxed);
    }
    
    fn size(&self) -> usize {
        self.objects.lock().unwrap().len()
    }
    
    fn max_size(&self) -> usize {
        self.max_size
    }
}

impl Clone for SnapshotPool {
    fn clone(&self) -> Self {
        Self {
            objects: Mutex::new(VecDeque::new()),
            max_size: self.max_size,
            stats: self.stats.clone(),
        }
    }
}

impl MemoryPoolManager {
    /// Create a new memory pool manager.
    pub fn new() -> Self {
        Self {
            actor_pool: Arc::new(ActorPool::new(1000)),
            subsystem_output_pool: Arc::new(SubsystemOutputPool::new(2000)),
            contribution_pool: Arc::new(ContributionPool::new(10000)),
            snapshot_pool: Arc::new(SnapshotPool::new(500)),
            stats: Arc::new(PoolStats::default()),
        }
    }
    
    /// Get an actor from the pool.
    pub fn get_actor(&self) -> PooledObject<crate::types::Actor> {
        self.actor_pool.get()
    }
    
    /// Get a subsystem output from the pool.
    pub fn get_subsystem_output(&self) -> PooledObject<crate::types::SubsystemOutput> {
        self.subsystem_output_pool.get()
    }
    
    /// Get a contribution from the pool.
    pub fn get_contribution(&self) -> PooledObject<crate::types::Contribution> {
        self.contribution_pool.get()
    }
    
    /// Get a snapshot from the pool.
    pub fn get_snapshot(&self) -> PooledObject<crate::types::Snapshot> {
        self.snapshot_pool.get()
    }
    
    /// Get comprehensive statistics.
    pub fn get_stats(&self) -> PoolStats {
        (*self.stats).clone()
    }
    
    /// Clear all pools.
    pub fn clear_all(&self) {
        self.actor_pool.clear();
        self.subsystem_output_pool.clear();
        self.contribution_pool.clear();
        self.snapshot_pool.clear();
    }
}