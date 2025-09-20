//! Dynamic validation system for Actor Core.
//!
//! This module provides configuration-based validation that loads validation rules
//! from the Runtime Registry System instead of using hardcoded values.

use std::collections::HashMap;
use std::sync::Arc;
use crate::ActorCoreResult;
use crate::registry::{ResourceRegistry, CategoryRegistry, TagRegistry};

/// Dynamic validator that loads validation rules from configuration
pub struct DynamicValidator {
    resource_registry: Arc<dyn ResourceRegistry>,
    category_registry: Arc<dyn CategoryRegistry>,
    tag_registry: Arc<dyn TagRegistry>,
}

impl DynamicValidator {
    /// Create a new dynamic validator
    pub fn new(
        resource_registry: Arc<dyn ResourceRegistry>,
        category_registry: Arc<dyn CategoryRegistry>,
        tag_registry: Arc<dyn TagRegistry>,
    ) -> Self {
        Self {
            resource_registry,
            category_registry,
            tag_registry,
        }
    }

    /// Validate actor data against configured rules
    pub async fn validate_actor_data(&self, actor_data: &HashMap<String, serde_json::Value>) -> ActorCoreResult<()> {
        // Load validation rules from registries
        let resources = self.resource_registry.get_all_resources().await?;
        let categories = self.category_registry.get_all_categories().await?;
        let tags = self.tag_registry.get_all_tags().await?;

        // Validate each dimension in actor data
        for (dimension, value) in actor_data {
            self.validate_dimension(dimension, value, &resources, &categories, &tags).await?;
        }

        Ok(())
    }

    /// Validate a single dimension
    async fn validate_dimension(
        &self,
        dimension: &str,
        value: &serde_json::Value,
        resources: &[crate::registry::ResourceDefinition],
        categories: &[crate::registry::CategoryDefinition],
        tags: &[crate::registry::TagDefinition],
    ) -> ActorCoreResult<()> {
        // Check if dimension is a valid resource
        if let Some(resource) = resources.iter().find(|r| r.id == dimension) {
            self.validate_resource_value(value, resource).await?;
            return Ok(());
        }

        // Check if dimension is a valid category
        if let Some(category) = categories.iter().find(|c| c.id == dimension) {
            self.validate_category_value(value, category).await?;
            return Ok(());
        }

        // Check if dimension is a valid tag
        if let Some(tag) = tags.iter().find(|t| t.id == dimension) {
            self.validate_tag_value(value, tag).await?;
            return Ok(());
        }

        // If not found in any registry, it's an unknown dimension
        Err(crate::ActorCoreError::ValidationError(format!(
            "Unknown dimension: {}",
            dimension
        )))
    }

    /// Validate resource value
    async fn validate_resource_value(
        &self,
        value: &serde_json::Value,
        resource: &crate::registry::ResourceDefinition,
    ) -> ActorCoreResult<()> {
        let num_value = value.as_f64()
            .ok_or_else(|| crate::ActorCoreError::ValidationError(format!(
                "Resource {} value must be a number",
                resource.id
            )))?;

        // Check min/max bounds
        if num_value < resource.min_value {
            return Err(crate::ActorCoreError::ValidationError(format!(
                "Resource {} value {} is below minimum {}",
                resource.id, num_value, resource.min_value
            )));
        }

        if num_value > resource.max_value {
            return Err(crate::ActorCoreError::ValidationError(format!(
                "Resource {} value {} is above maximum {}",
                resource.id, num_value, resource.max_value
            )));
        }

        Ok(())
    }

    /// Validate category value
    async fn validate_category_value(
        &self,
        value: &serde_json::Value,
        _category: &crate::registry::CategoryDefinition,
    ) -> ActorCoreResult<()> {
        // Categories can have various value types depending on their purpose
        // For now, just ensure it's a valid JSON value
        match value {
            serde_json::Value::String(_) => Ok(()),
            serde_json::Value::Number(_) => Ok(()),
            serde_json::Value::Bool(_) => Ok(()),
            serde_json::Value::Array(_) => Ok(()),
            serde_json::Value::Object(_) => Ok(()),
            serde_json::Value::Null => Ok(()),
        }
    }

    /// Validate tag value
    async fn validate_tag_value(
        &self,
        value: &serde_json::Value,
        tag: &crate::registry::TagDefinition,
    ) -> ActorCoreResult<()> {
        // Tags typically have string values
        match value {
            serde_json::Value::String(s) => {
                if s.is_empty() {
                    return Err(crate::ActorCoreError::ValidationError(format!(
                        "Tag {} value cannot be empty",
                        tag.id
                    )));
                }
                Ok(())
            },
            _ => Err(crate::ActorCoreError::ValidationError(format!(
                "Tag {} value must be a string",
                tag.id
            ))),
        }
    }

    /// Get all valid dimensions from registries
    pub async fn get_valid_dimensions(&self) -> ActorCoreResult<Vec<String>> {
        let mut dimensions = Vec::new();

        // Add resource dimensions
        let resources = self.resource_registry.get_all_resources().await?;
        for resource in resources {
            dimensions.push(resource.id);
        }

        // Add category dimensions
        let categories = self.category_registry.get_all_categories().await?;
        for category in categories {
            dimensions.push(category.id);
        }

        // Add tag dimensions
        let tags = self.tag_registry.get_all_tags().await?;
        for tag in tags {
            dimensions.push(tag.id);
        }

        Ok(dimensions)
    }
}

/// Configuration-based validation rules
pub struct ValidationRules {
    /// Allowed dimensions loaded from configuration
    pub allowed_dimensions: Vec<String>,
    /// Dimension ranges loaded from configuration
    pub dimension_ranges: HashMap<String, (f64, f64)>,
    /// Required dimensions
    pub required_dimensions: Vec<String>,
}

impl ValidationRules {
    /// Create new validation rules
    pub fn new() -> Self {
        Self {
            allowed_dimensions: Vec::new(),
            dimension_ranges: HashMap::new(),
            required_dimensions: Vec::new(),
        }
    }

    /// Load validation rules from configuration
    pub async fn load_from_config(&mut self, _config_path: &str) -> ActorCoreResult<()> {
        // TODO: Implement configuration loading from YAML files
        // This should load validation rules from the Configuration Hub
        // For now, return empty rules
        Ok(())
    }

    /// Add allowed dimension
    pub fn add_allowed_dimension(&mut self, dimension: String) {
        if !self.allowed_dimensions.contains(&dimension) {
            self.allowed_dimensions.push(dimension);
        }
    }

    /// Add dimension range
    pub fn add_dimension_range(&mut self, dimension: String, min: f64, max: f64) {
        self.dimension_ranges.insert(dimension, (min, max));
    }

    /// Add required dimension
    pub fn add_required_dimension(&mut self, dimension: String) {
        if !self.required_dimensions.contains(&dimension) {
            self.required_dimensions.push(dimension);
        }
    }

    /// Check if dimension is allowed
    pub fn is_dimension_allowed(&self, dimension: &str) -> bool {
        self.allowed_dimensions.contains(&dimension.to_string())
    }

    /// Get dimension range
    pub fn get_dimension_range(&self, dimension: &str) -> Option<(f64, f64)> {
        self.dimension_ranges.get(dimension).copied()
    }

    /// Check if dimension is required
    pub fn is_dimension_required(&self, dimension: &str) -> bool {
        self.required_dimensions.contains(&dimension.to_string())
    }
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self::new()
    }
}
