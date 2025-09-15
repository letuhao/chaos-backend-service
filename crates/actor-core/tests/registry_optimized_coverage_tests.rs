use actor_core::registry::optimized::{OptimizedPluginRegistry, RegistryStats};

#[tokio::test]
async fn test_registry_stats_new() {
    let stats = RegistryStats::new();
    assert_eq!(stats.total_registrations.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(stats.total_lookups.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(stats.cache_hits.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(stats.cache_misses.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(stats.last_cleanup.load(std::sync::atomic::Ordering::Relaxed), 0);
}

#[tokio::test]
async fn test_registry_stats_record_registration() {
    let stats = RegistryStats::new();
    stats.record_registration();
    assert_eq!(stats.total_registrations.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[tokio::test]
async fn test_registry_stats_record_lookup_hit() {
    let stats = RegistryStats::new();
    stats.record_lookup(true);
    assert_eq!(stats.total_lookups.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(stats.cache_hits.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(stats.cache_misses.load(std::sync::atomic::Ordering::Relaxed), 0);
}

#[tokio::test]
async fn test_registry_stats_record_lookup_miss() {
    let stats = RegistryStats::new();
    stats.record_lookup(false);
    assert_eq!(stats.total_lookups.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(stats.cache_hits.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(stats.cache_misses.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[tokio::test]
async fn test_registry_stats_get_hit_rate() {
    let stats = RegistryStats::new();
    stats.record_lookup(true);
    stats.record_lookup(false);
    assert_eq!(stats.get_hit_rate(), 50.0); // Returns percentage
}

#[tokio::test]
async fn test_registry_stats_get_hit_rate_no_lookups() {
    let stats = RegistryStats::new();
    assert_eq!(stats.get_hit_rate(), 0.0);
}

#[tokio::test]
async fn test_registry_stats_debug() {
    let stats = RegistryStats::new();
    println!("{:?}", stats); // Check Debug impl
    assert!(true);
}

#[tokio::test]
async fn test_optimized_plugin_registry_new() {
    let _registry = OptimizedPluginRegistry::new();
    assert!(true); // Basic creation test
}

#[tokio::test]
async fn test_optimized_plugin_registry_get_stats() {
    let registry = OptimizedPluginRegistry::new();
    let stats = registry.get_stats();
    assert_eq!(stats.total_registrations.load(std::sync::atomic::Ordering::Relaxed), 0);
    assert_eq!(stats.total_lookups.load(std::sync::atomic::Ordering::Relaxed), 0);
}

#[tokio::test]
async fn test_optimized_plugin_registry_cleanup_cache() {
    let registry = OptimizedPluginRegistry::new();
    registry.cleanup_cache().await;
    assert!(true); // Basic cleanup test
}
