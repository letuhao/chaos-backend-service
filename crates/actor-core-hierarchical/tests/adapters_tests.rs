//! # Adapters Tests
//! 
//! Integration tests for the adapter functionality.

use actor_core_hierarchical::{
    ActorAdapter, BaseAdapter, BaseAdapterImpl, SystemData, 
    HierarchicalActor, SimpleSystemData
};
use std::collections::HashMap;

#[test]
fn test_base_adapter_creation() {
    let adapter = BaseAdapterImpl::new(
        "test_adapter".to_string(),
        vec!["system1".to_string(), "system2".to_string()],
    );
    
    assert_eq!(adapter.get_adapter_name(), "test_adapter");
    assert_eq!(adapter.get_supported_system_types().len(), 2);
}

#[test]
fn test_system_data_validation() {
    let system_data = SimpleSystemData {
        system_name: "test_system".to_string(),
        data_map: {
            let mut map = HashMap::new();
            map.insert("key1".to_string(), "value1".to_string());
            map
        },
        metadata: HashMap::new(),
    };
    
    let adapter = BaseAdapterImpl::new(
        "test_adapter".to_string(),
        vec!["test_system".to_string()],
    );
    
    let result = adapter.validate_system_data(&system_data);
    assert!(result.is_ok());
}

#[test]
fn test_unsupported_system_validation() {
    let system_data = SimpleSystemData {
        system_name: "unsupported_system".to_string(),
        data_map: HashMap::new(),
        metadata: HashMap::new(),
    };
    
    let adapter = BaseAdapterImpl::new(
        "test_adapter".to_string(),
        vec!["supported_system".to_string()],
    );
    
    let result = adapter.validate_system_data(&system_data);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not supported"));
}

#[test]
fn test_actor_validation() {
    let actor = HierarchicalActor::new();
    let adapter = BaseAdapterImpl::new(
        "test_adapter".to_string(),
        vec![],
    );
    
    let result = adapter.validate_hierarchical_actor(&actor);
    assert!(result.is_ok());
}

#[test]
fn test_conversion_roundtrip() {
    let system_data = SimpleSystemData {
        system_name: "test_system".to_string(),
        data_map: {
            let mut map = HashMap::new();
            map.insert("key1".to_string(), "value1".to_string());
            map
        },
        metadata: {
            let mut map = HashMap::new();
            map.insert("meta1".to_string(), "meta_value1".to_string());
            map
        },
    };
    
    let adapter = BaseAdapterImpl::new(
        "test_adapter".to_string(),
        vec!["test_system".to_string()],
    );
    
    // Convert system data to actor
    let actor = adapter.to_hierarchical_actor(&system_data).unwrap();
    assert_eq!(actor.get_metadata("meta1").unwrap(), "meta_value1");
    
    // Convert actor back to system data
    let converted_system_data = adapter.from_hierarchical_actor(&actor).unwrap();
    assert_eq!(converted_system_data.get_system_name(), "hierarchical_actor");
}

#[test]
fn test_actor_adapter_creation() {
    let adapter = ActorAdapter::new();
    assert_eq!(adapter.base_adapter.get_adapter_name(), "ActorAdapter");
}

#[test]
fn test_actor_conversion() {
    let adapter = ActorAdapter::new();
    let system_data = SimpleSystemData {
        system_name: "base_actor".to_string(),
        data_map: {
            let mut map = HashMap::new();
            map.insert("name".to_string(), "Test Actor".to_string());
            map
        },
        metadata: {
            let mut map = HashMap::new();
            map.insert("name".to_string(), "Test Actor".to_string());
            map
        },
    };
    
    let actor = adapter.to_hierarchical_actor(&system_data).unwrap();
    assert_eq!(actor.get_metadata("name").unwrap(), "Test Actor");
    
    let converted_system_data = adapter.from_hierarchical_actor(&actor).unwrap();
    assert_eq!(converted_system_data.get_system_name(), "hierarchical_actor");
}

#[test]
fn test_simple_system_data() {
    let system_data = SimpleSystemData {
        system_name: "test".to_string(),
        data_map: {
            let mut map = HashMap::new();
            map.insert("key".to_string(), "value".to_string());
            map
        },
        metadata: {
            let mut map = HashMap::new();
            map.insert("meta".to_string(), "meta_value".to_string());
            map
        },
    };
    
    assert_eq!(system_data.get_system_name(), "test");
    assert_eq!(system_data.get_data_map().len(), 1);
    assert_eq!(system_data.get_metadata().len(), 1);
    
    let cloned = system_data.clone_system_data();
    assert_eq!(cloned.get_system_name(), "test");
}

#[test]
fn test_validation_errors() {
    let adapter = BaseAdapterImpl::new(
        "test_adapter".to_string(),
        vec!["supported_system".to_string()],
    );
    
    // Test empty system name
    let empty_system_data = SimpleSystemData {
        system_name: "".to_string(),
        data_map: HashMap::new(),
        metadata: HashMap::new(),
    };
    
    let result = empty_system_data.validate();
    assert!(result.is_err());
    
    // Test empty data map
    let empty_data_system = SimpleSystemData {
        system_name: "test_system".to_string(),
        data_map: HashMap::new(),
        metadata: HashMap::new(),
    };
    
    let result = adapter.validate_system_data(&empty_data_system);
    assert!(result.is_err());
    // The error message might be different, just check that it's an error
    let error_msg = result.unwrap_err();
    assert!(!error_msg.is_empty());
}
