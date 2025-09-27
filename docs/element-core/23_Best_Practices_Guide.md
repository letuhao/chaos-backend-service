# Element-Core Best Practices Guide

## 📋 **Overview**

This document provides best practices for implementing and using Element-Core in the Chaos World MMORPG. It covers development guidelines, performance optimization, testing strategies, and common pitfalls to avoid.

**Version**: 1.0  
**Last Updated**: 2024-12-19  
**Status**: Active

---

## 🎯 **Development Guidelines**

### **1. Architecture Principles**

#### **Follow the Data Hub Pattern**
```rust
// ✅ GOOD: Element-Core as data hub only
impl ElementCore {
    pub async fn get_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        // Only aggregate and cache data
        // No business logic here
    }
}

// ❌ BAD: Element-Core with business logic
impl ElementCore {
    pub async fn calculate_combat_damage(&self, attacker: &Actor, defender: &Actor) -> ElementCoreResult<f64> {
        // Business logic should be in Combat-Core
    }
}
```

#### **Use External Contributor Pattern**
```rust
// ✅ GOOD: External system implements ElementContributor
pub struct RaceCoreElementContributor {
    system_id: String,
    priority: i64,
    race_data: Arc<RaceData>,
}

impl ElementContributor for RaceCoreElementContributor {
    async fn contribute_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementContribution> {
        // Race-specific logic here
    }
}

// ❌ BAD: Direct integration with Element-Core
impl ElementCore {
    pub fn get_race_bonus(&self, race: &str, element_type: &str) -> f64 {
        // Race logic should be in Race-Core
    }
}
```

### **2. Performance Best Practices**

#### **Optimize for High-Frequency Operations**
```rust
// ✅ GOOD: Efficient element lookup
impl UnifiedElementRegistry {
    pub fn get_element(&self, element_id: &str) -> Option<&ElementDefinition> {
        // O(1) HashMap lookup
        self.elements.get(element_id)
    }
}

// ❌ BAD: Inefficient element search
impl UnifiedElementRegistry {
    pub fn find_element_by_name(&self, name: &str) -> Option<&ElementDefinition> {
        // O(n) linear search
        self.elements.values().find(|e| e.name == name)
    }
}
```

#### **Use Appropriate Caching Strategies**
```rust
// ✅ GOOD: Smart caching with TTL
impl ElementCache {
    pub async fn get(&self, key: &str) -> Option<ElementStats> {
        if let Some(cached) = self.cache.get(key) {
            if cached.is_valid() {
                return Some(cached.data);
            }
        }
        None
    }
}

// ❌ BAD: No cache invalidation
impl ElementCache {
    pub async fn get(&self, key: &str) -> Option<ElementStats> {
        // Always returns cached data, never invalidates
        self.cache.get(key)
    }
}
```

### **3. Error Handling**

#### **Use Proper Error Types**
```rust
// ✅ GOOD: Specific error types
#[derive(Debug, thiserror::Error)]
pub enum ElementCoreError {
    #[error("Element not found: {element_id}")]
    ElementNotFound { element_id: String },
    
    #[error("Invalid element configuration: {message}")]
    InvalidConfig { message: String },
    
    #[error("System registration failed: {system_id}")]
    RegistrationFailed { system_id: String },
}

// ❌ BAD: Generic error types
pub type ElementCoreResult<T> = Result<T, String>;
```

#### **Handle Errors Gracefully**
```rust
// ✅ GOOD: Graceful error handling
impl ElementCore {
    pub async fn get_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        match self.registry.get_element(element_type) {
            Some(_) => {
                // Process element stats
            }
            None => {
                // Log error and return default stats
                tracing::warn!("Element not found: {}", element_type);
                Ok(ElementStats::default())
            }
        }
    }
}

// ❌ BAD: Panic on error
impl ElementCore {
    pub async fn get_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        let element = self.registry.get_element(element_type).unwrap(); // Panics!
        // Process element stats
    }
}
```

---

## 🚀 **Performance Optimization**

### **1. Memory Management**

#### **Use Efficient Data Structures**
```rust
// ✅ GOOD: Pre-allocated HashMap
pub struct UnifiedElementRegistry {
    elements: HashMap<String, ElementDefinition>,
    // Pre-allocate with expected capacity
}

impl UnifiedElementRegistry {
    pub fn new() -> Self {
        Self {
            elements: HashMap::with_capacity(100), // Expected element count
        }
    }
}

// ❌ BAD: Default HashMap
pub struct UnifiedElementRegistry {
    elements: HashMap<String, ElementDefinition>, // No pre-allocation
}
```

#### **Minimize Allocations**
```rust
// ✅ GOOD: Reuse buffers
impl ElementAggregator {
    pub async fn aggregate_contributions(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        let mut contributions = Vec::with_capacity(10); // Pre-allocate
        // Reuse the same vector for multiple operations
    }
}

// ❌ BAD: Allocate new vectors frequently
impl ElementAggregator {
    pub async fn aggregate_contributions(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        let contributions = Vec::new(); // New allocation every time
    }
}
```

### **2. Concurrency**

#### **Use Appropriate Locking**
```rust
// ✅ GOOD: Read-write locks for different access patterns
pub struct UnifiedElementRegistry {
    elements: Arc<RwLock<HashMap<String, ElementDefinition>>>,
    cache: Arc<Mutex<HashMap<String, ElementStats>>>,
}

impl UnifiedElementRegistry {
    pub fn get_element(&self, element_id: &str) -> Option<ElementDefinition> {
        // Read lock for frequent reads
        self.elements.read().unwrap().get(element_id).cloned()
    }
    
    pub fn register_element(&self, element: ElementDefinition) -> ElementCoreResult<()> {
        // Write lock for infrequent writes
        self.elements.write().unwrap().insert(element.id.clone(), element);
        Ok(())
    }
}

// ❌ BAD: Mutex for read-only operations
pub struct UnifiedElementRegistry {
    elements: Arc<Mutex<HashMap<String, ElementDefinition>>>, // Unnecessary locking
}
```

#### **Avoid Deadlocks**
```rust
// ✅ GOOD: Consistent lock ordering
impl ElementCore {
    pub async fn update_element_and_cache(&self, element: ElementDefinition) -> ElementCoreResult<()> {
        // Always lock registry first, then cache
        let mut registry = self.registry.write().await;
        let mut cache = self.cache.write().await;
        
        registry.update_element(element.clone())?;
        cache.invalidate(&element.id);
        Ok(())
    }
}

// ❌ BAD: Inconsistent lock ordering
impl ElementCore {
    pub async fn update_element_and_cache(&self, element: ElementDefinition) -> ElementCoreResult<()> {
        // Different methods might lock in different orders
        self.cache.write().await.invalidate(&element.id);
        self.registry.write().await.update_element(element)?;
        Ok(())
    }
}
```

---

## 🧪 **Testing Strategies**

### **1. Unit Testing**

#### **Test Individual Components**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_element_registry_operations() {
        let mut registry = UnifiedElementRegistry::new();
        
        // Test element registration
        let fire_element = ElementDefinition {
            id: "fire".to_string(),
            name: "Fire".to_string(),
            // ... other fields
        };
        
        assert!(registry.register_element(fire_element).await.is_ok());
        assert!(registry.get_element("fire").is_some());
    }
    
    #[tokio::test]
    async fn test_element_contributor() {
        let contributor = RaceCoreElementContributor::new();
        let actor = Actor::new("test".to_string(), "human".to_string());
        
        let contribution = contributor.contribute_element_stats(&actor, "fire").await.unwrap();
        
        assert_eq!(contribution.system_id, "race-core");
        assert!(contribution.contributions.contains_key("affinity"));
    }
}
```

#### **Test Error Conditions**
```rust
#[cfg(test)]
mod error_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_element_not_found() {
        let registry = UnifiedElementRegistry::new();
        
        let result = registry.get_element("nonexistent");
        assert!(result.is_none());
    }
    
    #[tokio::test]
    async fn test_invalid_element_config() {
        let mut registry = UnifiedElementRegistry::new();
        
        let invalid_element = ElementDefinition {
            id: "".to_string(), // Invalid empty ID
            // ... other fields
        };
        
        let result = registry.register_element(invalid_element).await;
        assert!(result.is_err());
    }
}
```

### **2. Integration Testing**

#### **Test System Interactions**
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_complete_element_flow() {
        // Create Element-Core with all contributors
        let mut element_core = ElementCore::new();
        
        // Register contributors
        element_core.register_contributor(Arc::new(RaceCoreElementContributor::new())).await.unwrap();
        element_core.register_contributor(Arc::new(ItemCoreElementContributor::new())).await.unwrap();
        
        // Test complete flow
        let actor = Actor::new("test".to_string(), "human".to_string());
        let stats = element_core.get_element_stats(&actor, "fire").await.unwrap();
        
        // Verify stats include contributions from all systems
        assert!(stats.power > 0.0);
        assert!(stats.defense > 0.0);
    }
}
```

### **3. Performance Testing**

#### **Benchmark Critical Operations**
```rust
#[cfg(test)]
mod benchmarks {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn benchmark_element_lookup(c: &mut Criterion) {
        let registry = create_test_registry();
        
        c.bench_function("element_lookup", |b| {
            b.iter(|| {
                let element = registry.get_element(black_box("fire"));
                assert!(element.is_some());
            })
        });
    }
    
    fn benchmark_element_aggregation(c: &mut Criterion) {
        let element_core = create_test_element_core();
        let actor = create_test_actor();
        
        c.bench_function("element_aggregation", |b| {
            b.iter(|| {
                let stats = element_core.get_element_stats(black_box(&actor), black_box("fire"));
                assert!(stats.is_ok());
            })
        });
    }
    
    criterion_group!(benches, benchmark_element_lookup, benchmark_element_aggregation);
    criterion_main!(benches);
}
```

---

## ⚠️ **Common Pitfalls**

### **1. Architecture Violations**

#### **Don't Put Business Logic in Element-Core**
```rust
// ❌ BAD: Business logic in Element-Core
impl ElementCore {
    pub async fn calculate_combat_damage(&self, attacker: &Actor, defender: &Actor) -> ElementCoreResult<f64> {
        // This should be in Combat-Core
        let attacker_stats = self.get_element_stats(attacker, "fire").await?;
        let defender_stats = self.get_element_stats(defender, "water").await?;
        
        let base_damage = attacker_stats.power - defender_stats.defense;
        let interaction_factor = self.get_interaction_factor("fire", "water");
        
        Ok(base_damage * interaction_factor)
    }
}

// ✅ GOOD: Element-Core only provides data
impl ElementCore {
    pub async fn get_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        // Only aggregate and return data
    }
    
    pub fn get_interaction_factor(&self, attacker_element: &str, defender_element: &str) -> f64 {
        // Only return interaction factor
    }
}
```

#### **Don't Create Circular Dependencies**
```rust
// ❌ BAD: Circular dependency
impl ElementCore {
    pub fn new() -> Self {
        let combat_core = CombatCore::new(self.clone()); // Element-Core depends on Combat-Core
        Self {
            combat_core, // Combat-Core depends on Element-Core
        }
    }
}

// ✅ GOOD: No circular dependencies
impl ElementCore {
    pub fn new() -> Self {
        Self {
            // No direct dependencies on other cores
        }
    }
}
```

### **2. Performance Issues**

#### **Don't Block on I/O Operations**
```rust
// ❌ BAD: Blocking I/O in async context
impl ElementCore {
    pub async fn load_element_config(&self, path: &str) -> ElementCoreResult<()> {
        let content = std::fs::read_to_string(path)?; // Blocking!
        // Process content
        Ok(())
    }
}

// ✅ GOOD: Non-blocking I/O
impl ElementCore {
    pub async fn load_element_config(&self, path: &str) -> ElementCoreResult<()> {
        let content = tokio::fs::read_to_string(path).await?; // Non-blocking
        // Process content
        Ok(())
    }
}
```

#### **Don't Allocate in Hot Paths**
```rust
// ❌ BAD: Allocations in hot path
impl ElementCore {
    pub fn get_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        let mut contributions = Vec::new(); // Allocation every call
        for contributor in &self.contributors {
            contributions.push(contributor.contribute(actor, element_type)?);
        }
        // Process contributions
    }
}

// ✅ GOOD: Reuse pre-allocated buffers
impl ElementCore {
    pub fn get_element_stats(&self, actor: &Actor, element_type: &str) -> ElementCoreResult<ElementStats> {
        let mut contributions = self.contribution_buffer.borrow_mut(); // Reuse buffer
        contributions.clear();
        
        for contributor in &self.contributors {
            contributions.push(contributor.contribute(actor, element_type)?);
        }
        // Process contributions
    }
}
```

### **3. Error Handling Issues**

#### **Don't Ignore Errors**
```rust
// ❌ BAD: Ignoring errors
impl ElementCore {
    pub async fn register_contributor(&mut self, contributor: Arc<dyn ElementContributor>) {
        self.contributors.insert(contributor.system_id().to_string(), contributor);
        // What if insertion fails?
    }
}

// ✅ GOOD: Proper error handling
impl ElementCore {
    pub async fn register_contributor(&mut self, contributor: Arc<dyn ElementContributor>) -> ElementCoreResult<()> {
        let system_id = contributor.system_id().to_string();
        
        if self.contributors.contains_key(&system_id) {
            return Err(ElementCoreError::SystemAlreadyRegistered { system_id });
        }
        
        self.contributors.insert(system_id, contributor);
        Ok(())
    }
}
```

---

## 📚 **Related Documents**

- [Unified Architecture Design](20_Unified_Architecture_Design.md) - Target architecture
- [Element Registry Design](04_Element_Registry_Design.md) - Registry implementation
- [Migration Guide](21_Migration_Guide.md) - Migration from old architecture
- [Element Core Overview](00_Element_Core_Overview.md) - Main overview

---

## 🔄 **Evolution Strategy**

### **Version 1.0 (Current)**
- Basic best practices
- Performance guidelines
- Testing strategies

### **Version 2.0 (Future)**
- Advanced optimization techniques
- Machine learning integration
- Enhanced monitoring

### **Version 3.0 (Future)**
- AI-powered optimization
- Predictive performance tuning
- Advanced analytics

---

**Last Updated**: 2024-12-19  
**Version**: 1.0  
**Status**: Active  
**Next Review**: 2024-12-26
