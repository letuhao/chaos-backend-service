//! Observability Example
//! 
//! This example demonstrates the comprehensive observability system for Actor Core,
//! including SLOs, metrics collection, and dashboard monitoring.

use actor_core::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> ActorCoreResult<()> {
    println!("🎮 Actor Core - Observability Example");
    println!("=====================================");

    // Example 1: SLO Management
    println!("\n1. Setting up Service Level Objectives:");
    
    let mut slo_manager = SLOManager::new();
    
    // Register default SLOs
    let default_slos = default_slos::create_default_slos();
    for slo in default_slos {
        slo_manager.register_slo(slo)?;
    }
    
    println!("✅ Registered {} default SLOs", slo_manager.list_slos().len());

    // Example 2: Metrics Collection
    println!("\n2. Setting up Metrics Collection:");
    
    let mut metrics_collector = MetricsCollector::new();
    default_metrics::register_default_metrics(&mut metrics_collector)?;
    
    println!("✅ Registered {} default metrics", metrics_collector.list_metrics().len());

    // Example 3: Record some metrics
    println!("\n3. Recording metrics:");
    
    // Simulate actor resolutions
    for i in 0..100 {
        metrics_collector.increment_counter("actor_resolutions_total", 1)?;
        
        // Simulate resolution duration (random between 10-200ms)
        let duration_ms = 10.0 + (i as f64 % 190.0);
        metrics_collector.observe_histogram("actor_resolution_duration_ms", duration_ms)?;
        
        // Record SLO events (mostly successful)
        let success = i % 10 != 0; // 90% success rate
        slo_manager.record_event("actor_core_availability", success, None)?;
        
        // Record latency SLO (mostly under 100ms)
        let under_threshold = duration_ms <= 100.0;
        slo_manager.record_event("actor_core_latency", under_threshold, None)?;
    }
    
    println!("✅ Recorded 100 actor resolution metrics");
    println!("✅ Recorded SLO events for availability and latency");

    // Example 4: Cache metrics
    println!("\n4. Recording cache metrics:");
    
    for i in 0..50 {
        metrics_collector.increment_counter("cache_operations_total", 1)?;
        
        // Simulate cache hit rate of 80%
        let hit = i % 5 != 0;
        if hit {
            metrics_collector.increment_counter("cache_hits_total", 1)?;
        } else {
            metrics_collector.increment_counter("cache_misses_total", 1)?;
        }
        
        // Record cache hit rate SLO
        slo_manager.record_event("actor_core_cache_hit_rate", hit, None)?;
    }
    
    println!("✅ Recorded 50 cache operation metrics");

    // Example 5: Subsystem metrics
    println!("\n5. Recording subsystem metrics:");
    
    for i in 0..25 {
        metrics_collector.increment_counter("subsystem_contributions_total", 1)?;
        
        // Simulate subsystem processing time
        let processing_time = 1.0 + (i as f64 % 20.0);
        metrics_collector.observe_histogram("subsystem_processing_duration_ms", processing_time)?;
        
        // Simulate occasional errors
        if i % 20 == 0 {
            metrics_collector.increment_counter("subsystem_errors_total", 1)?;
        }
    }
    
    println!("✅ Recorded 25 subsystem contribution metrics");

    // Example 6: Memory and system metrics
    println!("\n6. Recording system metrics:");
    
    metrics_collector.set_gauge("memory_usage_bytes", 128 * 1024 * 1024)?; // 128 MB
    metrics_collector.set_gauge("active_actors_count", 1000)?;
    
    println!("✅ Recorded system metrics");

    // Example 7: Validation metrics
    println!("\n7. Recording validation metrics:");
    
    for i in 0..200 {
        metrics_collector.increment_counter("validation_checks_total", 1)?;
        
        // Simulate validation duration
        let validation_time = 0.5 + (i as f64 % 10.0);
        metrics_collector.observe_histogram("validation_duration_ms", validation_time)?;
        
        // Simulate occasional validation failures
        if i % 50 == 0 {
            metrics_collector.increment_counter("validation_failures_total", 1)?;
        }
    }
    
    println!("✅ Recorded 200 validation metrics");

    // Example 8: Check SLO statuses
    println!("\n8. Checking SLO Status:");
    
    for slo in slo_manager.list_slos() {
        if let Ok(status) = slo_manager.calculate_slo_status(&slo.id) {
            let health_icon = if status.is_healthy { "✅" } else { "❌" };
            println!("{} {}: {:.1}% (target: {:.1}%) - Error Budget: {:.1}%",
                health_icon,
                slo.name,
                status.current_success_rate * 100.0,
                status.target_success_rate * 100.0,
                status.error_budget_remaining * 100.0
            );
        }
    }

    // Example 9: Create and use observability dashboard
    println!("\n9. Creating Observability Dashboard:");
    
    let dashboard = DashboardBuilder::new()
        .with_slo_manager(Arc::new(slo_manager))
        .with_metrics_collector(Arc::new(metrics_collector))
        .with_config(DashboardConfig {
            refresh_interval: Duration::from_secs(30),
            include_detailed_metrics: true,
            include_slo_status: true,
            include_system_health: true,
            max_recent_measurements: 10,
            auto_refresh: false,
        })
        .build()?;
    
    println!("✅ Created observability dashboard");

    // Example 10: Generate dashboard reports
    println!("\n10. Generating Dashboard Reports:");
    
    // Text report
    let text_report = dashboard.generate_text_report().await?;
    println!("📊 Text Report:");
    println!("{}", text_report);
    
    // JSON report
    let json_report = dashboard.generate_json_report().await?;
    println!("\n📋 JSON Report (first 500 chars):");
    println!("{}...", &json_report[..json_report.len().min(500)]);
    
    // Health summary
    let health_summary = dashboard.get_health_summary().await?;
    println!("\n🏥 Health Summary:");
    println!("   Overall Health: {:?}", health_summary.overall_health);
    println!("   SLO Health: {:.1}% ({} of {} SLOs healthy)",
        health_summary.slo_health_percentage,
        health_summary.healthy_slos,
        health_summary.total_slos
    );
    println!("   Active Alerts: {}", health_summary.active_alerts);
    println!("   Uptime: {} seconds", health_summary.uptime_seconds);

    // Example 11: Demonstrate SLO violation handling
    println!("\n11. Demonstrating SLO Violation Handling:");
    
    // Create a console violation handler
    let console_handler = ConsoleSLOViolationHandler;
    let mut test_slo_manager = SLOManager::new();
    
    // Register a test SLO with high target
    let test_slo = SLO {
        id: "test_high_target".to_string(),
        name: "Test High Target SLO".to_string(),
        description: "Test SLO with high success rate requirement".to_string(),
        target_success_rate: 0.95, // 95% target
        measurement_window: Duration::from_secs(60),
        metric_type: SLOMetricType::Availability,
        labels: HashMap::new(),
        enabled: true,
        severity: SLOSeverity::High,
        alert_threshold: 0.9,
    };
    
    test_slo_manager.register_slo(test_slo)?;
    test_slo_manager.register_violation_handler(Box::new(console_handler));
    
    // Record mostly failed events to trigger violation
    println!("   Recording events that will trigger SLO violation...");
    for i in 0..20 {
        let success = i % 5 == 0; // Only 20% success rate
        test_slo_manager.record_event("test_high_target", success, None)?;
    }
    
    // Check the status
    if let Ok(status) = test_slo_manager.calculate_slo_status("test_high_target") {
        println!("   Final SLO Status: {:.1}% (target: 95.0%) - Healthy: {}",
            status.current_success_rate * 100.0,
            status.is_healthy
        );
    }

    // Example 12: Metrics snapshot and analysis
    println!("\n12. Metrics Analysis:");
    
    let snapshot = metrics_collector.snapshot();
    println!("📊 Metrics Snapshot Summary:");
    println!("   Timestamp: {:?}", snapshot.timestamp);
    println!("   Counter Metrics: {}", snapshot.counters.len());
    println!("   Gauge Metrics: {}", snapshot.gauges.len());
    println!("   Histogram Metrics: {}", snapshot.histograms.len());
    
    // Show some key metrics
    if let Some(resolutions) = snapshot.counters.get("actor_resolutions_total") {
        println!("   Total Actor Resolutions: {:.0}", resolutions.value);
    }
    
    if let Some(cache_hits) = snapshot.counters.get("cache_hits_total") {
        println!("   Cache Hits: {:.0}", cache_hits.value);
    }
    
    if let Some(memory) = snapshot.gauges.get("memory_usage_bytes") {
        let memory_mb = memory.value / (1024.0 * 1024.0);
        println!("   Memory Usage: {:.1} MB", memory_mb);
    }
    
    // Show histogram data
    if let Some(histogram) = snapshot.histograms.get("actor_resolution_duration_ms") {
        println!("   Resolution Duration Histogram:");
        println!("     Count: {}", histogram.count);
        println!("     Sum: {:.1} ms", histogram.sum);
        println!("     Buckets: {}", histogram.buckets.len());
        
        // Show some bucket data
        for (bound, count) in histogram.buckets.iter().take(5) {
            if *bound == f64::INFINITY {
                println!("       +Inf: {}", count);
            } else {
                println!("       ≤{}ms: {}", bound, count);
            }
        }
    }

    // Example 13: Custom metrics registration
    println!("\n13. Custom Metrics Registration:");
    
    let mut custom_collector = MetricsCollector::new();
    
    // Register custom business metrics
    custom_collector.register_counter(
        "player_levels_gained_total".to_string(),
        "Total number of player level gains".to_string(),
        {
            let mut labels = HashMap::new();
            labels.insert("game_mode".to_string(), "pve".to_string());
            labels
        }
    )?;
    
    custom_collector.register_histogram(
        "quest_completion_time_minutes".to_string(),
        "Time taken to complete quests in minutes".to_string(),
        HashMap::new(),
        vec![1.0, 5.0, 10.0, 30.0, 60.0, 120.0, 300.0],
    )?;
    
    custom_collector.register_gauge(
        "active_guilds_count".to_string(),
        "Current number of active guilds".to_string(),
        HashMap::new(),
        Some(42),
    )?;
    
    // Record some custom metrics
    custom_collector.increment_counter("player_levels_gained_total", 15)?;
    custom_collector.observe_histogram("quest_completion_time_minutes", 25.5)?;
    custom_collector.set_gauge("active_guilds_count", 45)?;
    
    println!("✅ Registered and recorded custom business metrics");

    // Example 14: Performance monitoring
    println!("\n14. Performance Monitoring:");
    
    let start = std::time::Instant::now();
    
    // Simulate some work
    for i in 0..1000 {
        let _ = i * i;
    }
    
    let duration = start.elapsed();
    println!("   Work completed in {:?}", duration);
    
    // Record the performance
    metrics_collector.record_duration("actor_resolution_duration_ms", duration)?;
    println!("✅ Recorded performance metric");

    println!("\n🎉 Observability example completed successfully!");
    println!("\nKey takeaways:");
    println!("• SLOs provide automated monitoring of service quality");
    println!("• Metrics collection enables performance tracking");
    println!("• Dashboard provides comprehensive system visibility");
    println!("• Violation handlers enable automated alerting");
    println!("• Custom metrics support business-specific monitoring");

    Ok(())
}
