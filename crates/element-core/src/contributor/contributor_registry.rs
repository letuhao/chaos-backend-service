//! # Element Contributor Registry
//! 
//! This module provides the ElementContributorRegistry for managing external system contributors.

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use dashmap::DashMap;
use crate::{ElementCoreResult, ElementCoreError};
use crate::contributor::{ElementContributor, ElementContribution, ContributorMetadata};
use crate::unified_registry::UnifiedElementRegistry;
use actor_core::Actor;

/// Registry for external system contributors
/// 
/// This registry manages all external systems that contribute to Element-Core,
/// providing thread-safe access and management of contributors.
pub struct ElementContributorRegistry {
    /// Optional backing unified registry (preferred source of truth)
    unified: Option<std::sync::Arc<UnifiedElementRegistry>>,
    /// Registered contributors
    contributors: DashMap<String, Arc<dyn ElementContributor>>,
    
    /// Contributor metadata cache
    metadata_cache: DashMap<String, ContributorMetadata>,
    
    /// Registration order for priority-based processing
    registration_order: Arc<dashmap::DashSet<String>>,
}

impl ElementContributorRegistry {
    /// Create a new contributor registry
    pub fn new() -> Self {
        Self {
            unified: None,
            contributors: DashMap::new(),
            metadata_cache: DashMap::new(),
            registration_order: Arc::new(dashmap::DashSet::new()),
        }
    }
    
    /// Create a contributor registry backed by a UnifiedElementRegistry
    pub fn with_unified_registry(registry: std::sync::Arc<UnifiedElementRegistry>) -> Self {
        Self {
            unified: Some(registry),
            contributors: DashMap::new(),
            metadata_cache: DashMap::new(),
            registration_order: Arc::new(dashmap::DashSet::new()),
        }
    }
    
    /// Register a new contributor
    /// 
    /// # Arguments
    /// * `contributor` - The contributor to register
    /// 
    /// # Returns
    /// * `Ok(())` if registration was successful
    /// * `Err(ElementCoreError)` if registration failed
    pub async fn register_contributor(
        &self,
        contributor: Arc<dyn ElementContributor>
    ) -> ElementCoreResult<()> {
        if let Some(unified) = &self.unified {
            return unified.register_contributor(contributor).await;
        }
        let system_id = contributor.system_id().to_string();
        
        // Check if contributor is already registered
        if self.contributors.contains_key(&system_id) {
            return Err(ElementCoreError::Registry { 
                message: format!("Contributor '{}' is already registered", system_id)
            });
        }
        
        // Get metadata
        let metadata = contributor.get_metadata();
        
        // Register the contributor
        self.contributors.insert(system_id.clone(), contributor);
        self.metadata_cache.insert(system_id.clone(), metadata);
        self.registration_order.insert(system_id);
        
        Ok(())
    }
    
    /// Unregister a contributor
    /// 
    /// # Arguments
    /// * `system_id` - The system ID to unregister
    /// 
    /// # Returns
    /// * `Ok(())` if unregistration was successful
    /// * `Err(ElementCoreError)` if contributor was not found
    pub async fn unregister_contributor(&self, system_id: &str) -> ElementCoreResult<()> {
        if let Some(unified) = &self.unified {
            return unified.unregister_contributor(system_id).await;
        }
        if self.contributors.remove(system_id).is_none() {
            return Err(ElementCoreError::Registry { 
                message: format!("Contributor '{}' not found", system_id)
            });
        }
        
        self.metadata_cache.remove(system_id);
        self.registration_order.remove(system_id);
        
        Ok(())
    }
    
    /// Get a contributor by system ID
    /// 
    /// # Arguments
    /// * `system_id` - The system ID to look up
    /// 
    /// # Returns
    /// * `Some(Arc<dyn ElementContributor>)` if found
    /// * `None` if not found
    pub fn get_contributor(&self, system_id: &str) -> Option<Arc<dyn ElementContributor>> {
        if let Some(unified) = &self.unified {
            return unified.get_contributor(system_id);
        }
        self.contributors.get(system_id).map(|entry| entry.clone())
    }
    
    /// Get all contributors sorted by priority (highest first)
    /// 
    /// # Returns
    /// * Vector of contributors sorted by priority
    pub fn get_contributors_by_priority(&self) -> Vec<Arc<dyn ElementContributor>> {
        let mut contributors: Vec<Arc<dyn ElementContributor>> = if let Some(unified) = &self.unified {
            unified.get_all_contributors()
        } else {
            self.contributors.iter().map(|entry| entry.clone()).collect()
        };
        
        // Sort by priority (highest first)
        contributors.sort_by_key(|contributor| std::cmp::Reverse(contributor.priority()));
        
        contributors
    }
    
    /// Get all contributors in registration order
    /// 
    /// # Returns
    /// * Vector of contributors in registration order
    pub fn get_contributors_by_registration_order(&self) -> Vec<Arc<dyn ElementContributor>> {
        if self.unified.is_some() {
            // Fallback to priority ordering when backed by unified registry
            return self.get_contributors_by_priority();
        }
        let mut v = Vec::new();
        for system_id in self.registration_order.iter() {
            if let Some(c) = self.get_contributor(&system_id) { v.push(c); }
        }
        v
    }
    
    /// Get contributor metadata
    /// 
    /// # Arguments
    /// * `system_id` - The system ID to look up
    /// 
    /// # Returns
    /// * `Some(ContributorMetadata)` if found
    /// * `None` if not found
    pub fn get_metadata(&self, system_id: &str) -> Option<ContributorMetadata> {
        if let Some(_unified) = &self.unified {
            // Recompute on demand from contributor when using unified backing
            return self.get_contributor(system_id).map(|c| c.get_metadata());
        }
        self.metadata_cache.get(system_id).map(|entry| entry.clone())
    }
    
    /// Get all contributor metadata
    /// 
    /// # Returns
    /// * HashMap of system_id -> ContributorMetadata
    pub fn get_all_metadata(&self) -> HashMap<String, ContributorMetadata> {
        if let Some(unified) = &self.unified {
            let mut map = HashMap::new();
            for c in unified.get_all_contributors() {
                map.insert(c.system_id().to_string(), c.get_metadata());
            }
            map
        } else {
            self.metadata_cache.iter()
                .map(|entry| (entry.key().clone(), entry.value().clone()))
                .collect()
        }
    }
    
    /// Check if a contributor is registered
    /// 
    /// # Arguments
    /// * `system_id` - The system ID to check
    /// 
    /// # Returns
    /// * `true` if registered
    /// * `false` if not registered
    pub fn is_registered(&self, system_id: &str) -> bool {
        if let Some(unified) = &self.unified { unified.is_contributor_registered(system_id) } else { self.contributors.contains_key(system_id) }
    }
    
    /// Get the number of registered contributors
    /// 
    /// # Returns
    /// * Number of registered contributors
    pub fn contributor_count(&self) -> usize {
        if let Some(unified) = &self.unified { unified.contributor_count() } else { self.contributors.len() }
    }
    
    /// Get all system IDs
    /// 
    /// # Returns
    /// * Vector of all registered system IDs
    pub fn get_system_ids(&self) -> Vec<String> {
        if let Some(unified) = &self.unified {
            unified.get_all_contributors().iter().map(|c| c.system_id().to_string()).collect()
        } else {
            self.contributors.iter().map(|entry| entry.key().clone()).collect()
        }
    }
    
    /// Clear all contributors
    pub async fn clear(&self) {
        if self.unified.is_some() {
            // No-op; unified registry owns data
        } else {
            self.contributors.clear();
            self.metadata_cache.clear();
            self.registration_order.clear();
        }
    }
    
    /// Collect contributions from all registered contributors
    /// 
    /// # Arguments
    /// * `actor` - The actor to collect contributions for
    /// * `element_type` - The element type to collect contributions for
    /// 
    /// # Returns
    /// * Vector of contributions sorted by priority
    pub async fn collect_contributions(
        &self,
        actor: &Actor,
        element_type: &str
    ) -> ElementCoreResult<Vec<ElementContribution>> {
        let mut contributions = Vec::new();
        
        // Get contributors sorted by priority
        let contributors = self.get_contributors_by_priority();
        
        for contributor in contributors {
            match contributor.contribute_element_stats(actor, element_type).await {
                Ok(contribution) => {
                    // Basic validation
                    if contribution.system_id != contributor.system_id() {
                        return Err(ElementCoreError::Validation { 
                            message: format!("System ID mismatch: expected {}, got {}", 
                                contributor.system_id(), contribution.system_id)
                        });
                    }
                    contributions.push(contribution);
                }
                Err(e) => {
                    return Err(ElementCoreError::Registry { 
                        message: format!("Failed to collect contribution from {}: {}", 
                            contributor.system_id(), e)
                    });
                }
            }
        }
        
        Ok(contributions)
    }
    
    /// Handle element event for all registered contributors
    /// 
    /// # Arguments
    /// * `event` - The element event to handle
    /// 
    /// # Returns
    /// * `Ok(())` if all contributors handled the event successfully
    /// * `Err(ElementCoreError)` if any contributor failed
    pub async fn handle_element_event(&self, event: &crate::contributor::ElementEvent) -> ElementCoreResult<()> {
        let contributors = self.get_contributors_by_priority();
        
        for contributor in contributors {
            if let Err(e) = contributor.handle_element_event(event).await {
                return Err(ElementCoreError::Registry { 
                    message: format!("Failed to handle element event for {}: {}", 
                        contributor.system_id(), e)
                });
            }
        }
        
        Ok(())
    }
}

impl Default for ElementContributorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe wrapper for ElementContributorRegistry
pub type SharedElementContributorRegistry = Arc<ElementContributorRegistry>;
