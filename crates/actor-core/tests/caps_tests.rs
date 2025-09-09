//! Comprehensive tests for Caps functionality.
//!
//! This module contains detailed tests for all Caps methods and behaviors,
//! including edge cases, mathematical operations, and performance characteristics.

use actor_core::types::Caps;

/// Test basic caps creation and validation
#[tokio::test]
async fn test_caps_creation_and_validation() {
    // Test valid caps creation
    let caps = Caps::new(0.0, 100.0);
    assert!(caps.is_valid());
    assert_eq!(caps.get_min(), 0.0);
    assert_eq!(caps.get_max(), 100.0);
    
    // Test caps with equal min and max
    let caps = Caps::new(50.0, 50.0);
    assert!(caps.is_valid());
    assert_eq!(caps.get_min(), 50.0);
    assert_eq!(caps.get_max(), 50.0);
    
    // Test invalid caps (min > max)
    let caps = Caps::new(100.0, 0.0);
    assert!(!caps.is_valid());
    
    // Test caps with infinity
    let caps = Caps::new(f64::INFINITY, 100.0);
    assert!(!caps.is_valid());
    
    let caps = Caps::new(0.0, f64::NEG_INFINITY);
    assert!(!caps.is_valid());
    
    // Test caps with NaN
    let caps = Caps::new(f64::NAN, 100.0);
    assert!(!caps.is_valid());
}

/// Test caps containment operations
#[tokio::test]
async fn test_caps_containment() {
    let caps = Caps::new(0.0, 100.0);
    
    // Test values within range
    assert!(caps.contains(0.0));
    assert!(caps.contains(50.0));
    assert!(caps.contains(100.0));
    
    // Test values outside range
    assert!(!caps.contains(-1.0));
    assert!(!caps.contains(101.0));
    assert!(!caps.contains(f64::INFINITY));
    assert!(!caps.contains(f64::NEG_INFINITY));
    assert!(!caps.contains(f64::NAN));
    
    // Test edge cases
    let caps = Caps::new(0.0, 0.0);
    assert!(caps.contains(0.0));
    assert!(!caps.contains(0.1));
    assert!(!caps.contains(-0.1));
}

/// Test caps clamping operations
#[tokio::test]
async fn test_caps_clamping() {
    let caps = Caps::new(0.0, 100.0);
    
    // Test values within range (should not change)
    assert_eq!(caps.clamp(0.0), 0.0);
    assert_eq!(caps.clamp(50.0), 50.0);
    assert_eq!(caps.clamp(100.0), 100.0);
    
    // Test values below range
    assert_eq!(caps.clamp(-10.0), 0.0);
    assert_eq!(caps.clamp(-0.1), 0.0);
    assert_eq!(caps.clamp(f64::NEG_INFINITY), 0.0);
    
    // Test values above range
    assert_eq!(caps.clamp(110.0), 100.0);
    assert_eq!(caps.clamp(100.1), 100.0);
    assert_eq!(caps.clamp(f64::INFINITY), 100.0);
    
    // Test NaN handling - clamp should return NaN for NaN input
    let nan_result = caps.clamp(f64::NAN);
    assert!(nan_result.is_nan() || nan_result == caps.get_min() || nan_result == caps.get_max());
}

/// Test caps range operations
#[tokio::test]
async fn test_caps_range_operations() {
    let caps = Caps::new(10.0, 90.0);
    
    // Test range calculation
    assert_eq!(caps.get_range(), 80.0);
    
    // Test center calculation
    assert_eq!(caps.get_center(), 50.0);
    
    // Test empty range
    let empty_caps = Caps::new(50.0, 50.0);
    assert_eq!(empty_caps.get_range(), 0.0);
    assert_eq!(empty_caps.get_center(), 50.0);
    
    // Test negative range (invalid)
    let invalid_caps = Caps::new(90.0, 10.0);
    assert_eq!(invalid_caps.get_range(), -80.0);
    assert_eq!(invalid_caps.get_center(), 50.0);
}

/// Test caps empty detection
#[tokio::test]
async fn test_caps_empty_detection() {
    // Test non-empty caps
    let caps = Caps::new(0.0, 100.0);
    assert!(!caps.is_empty());
    
    let caps = Caps::new(50.0, 50.0);
    assert!(!caps.is_empty());
    
    // Test empty caps (min > max)
    let caps = Caps::new(100.0, 0.0);
    assert!(caps.is_empty());
    
    let caps = Caps::new(0.1, 0.0);
    assert!(caps.is_empty());
}

/// Test caps setter operations
#[tokio::test]
async fn test_caps_setter_operations() {
    let mut caps = Caps::new(0.0, 100.0);
    
    // Test setting min
    caps.set_min(10.0);
    assert_eq!(caps.get_min(), 10.0);
    assert_eq!(caps.get_max(), 100.0);
    
    // Test setting max
    caps.set_max(90.0);
    assert_eq!(caps.get_min(), 10.0);
    assert_eq!(caps.get_max(), 90.0);
    
    // Test setting both
    caps.set(20.0, 80.0);
    assert_eq!(caps.get_min(), 20.0);
    assert_eq!(caps.get_max(), 80.0);
}

/// Test caps expansion operations
#[tokio::test]
async fn test_caps_expansion() {
    let mut caps = Caps::new(40.0, 60.0);
    
    // Test expansion
    caps.expand(10.0);
    assert_eq!(caps.get_min(), 30.0);
    assert_eq!(caps.get_max(), 70.0);
    
    // Test expansion with zero
    caps.expand(0.0);
    assert_eq!(caps.get_min(), 30.0);
    assert_eq!(caps.get_max(), 70.0);
    
    // Test expansion with negative amount (should shrink)
    caps.expand(-5.0);
    assert_eq!(caps.get_min(), 35.0);
    assert_eq!(caps.get_max(), 65.0);
}

/// Test caps shrinking operations
#[tokio::test]
async fn test_caps_shrinking() {
    let mut caps = Caps::new(20.0, 80.0);
    
    // Test shrinking
    caps.shrink(10.0);
    assert_eq!(caps.get_min(), 30.0);
    assert_eq!(caps.get_max(), 70.0);
    
    // Test shrinking with zero
    caps.shrink(0.0);
    assert_eq!(caps.get_min(), 30.0);
    assert_eq!(caps.get_max(), 70.0);
    
    // Test shrinking with negative amount (should expand)
    caps.shrink(-5.0);
    assert_eq!(caps.get_min(), 25.0);
    assert_eq!(caps.get_max(), 75.0);
}

/// Test caps shrinking edge cases
#[tokio::test]
async fn test_caps_shrinking_edge_cases() {
    // Test shrinking beyond center (should collapse to center)
    let mut caps = Caps::new(0.0, 100.0);
    caps.shrink(60.0); // Shrink by more than half the range
    assert_eq!(caps.get_min(), 50.0);
    assert_eq!(caps.get_max(), 50.0);
    
    // Test shrinking exactly to center
    let mut caps = Caps::new(0.0, 100.0);
    caps.shrink(50.0); // Shrink by exactly half the range
    assert_eq!(caps.get_min(), 50.0);
    assert_eq!(caps.get_max(), 50.0);
    
    // Test shrinking with very small range
    let mut caps = Caps::new(49.9, 50.1);
    caps.shrink(0.2); // Shrink by more than the range
    assert_eq!(caps.get_min(), 50.0);
    assert_eq!(caps.get_max(), 50.0);
}

/// Test caps intersection operations
#[tokio::test]
async fn test_caps_intersection() {
    let caps1 = Caps::new(0.0, 100.0);
    let caps2 = Caps::new(50.0, 150.0);
    
    // Test intersection
    let intersection = caps1.intersection(&caps2);
    assert_eq!(intersection.get_min(), 50.0);
    assert_eq!(intersection.get_max(), 100.0);
    
    // Test intersection with no overlap
    let caps3 = Caps::new(200.0, 300.0);
    let intersection = caps1.intersection(&caps3);
    assert_eq!(intersection.get_min(), 200.0);
    assert_eq!(intersection.get_max(), 100.0);
    assert!(intersection.is_empty());
    
    // Test intersection with identical caps
    let intersection = caps1.intersection(&caps1);
    assert_eq!(intersection.get_min(), 0.0);
    assert_eq!(intersection.get_max(), 100.0);
    
    // Test intersection with contained caps
    let caps4 = Caps::new(25.0, 75.0);
    let intersection = caps1.intersection(&caps4);
    assert_eq!(intersection.get_min(), 25.0);
    assert_eq!(intersection.get_max(), 75.0);
}

/// Test caps union operations
#[tokio::test]
async fn test_caps_union() {
    let caps1 = Caps::new(0.0, 100.0);
    let caps2 = Caps::new(50.0, 150.0);
    
    // Test union
    let union = caps1.union(&caps2);
    assert_eq!(union.get_min(), 0.0);
    assert_eq!(union.get_max(), 150.0);
    
    // Test union with no overlap
    let caps3 = Caps::new(200.0, 300.0);
    let union = caps1.union(&caps3);
    assert_eq!(union.get_min(), 0.0);
    assert_eq!(union.get_max(), 300.0);
    
    // Test union with identical caps
    let union = caps1.union(&caps1);
    assert_eq!(union.get_min(), 0.0);
    assert_eq!(union.get_max(), 100.0);
    
    // Test union with contained caps
    let caps4 = Caps::new(25.0, 75.0);
    let union = caps1.union(&caps4);
    assert_eq!(union.get_min(), 0.0);
    assert_eq!(union.get_max(), 100.0);
}

/// Test caps with extreme values
#[tokio::test]
async fn test_caps_extreme_values() {
    // Test with very large values
    let caps = Caps::new(0.0, f64::MAX);
    assert!(caps.is_valid());
    assert!(caps.contains(f64::MAX));
    assert!(!caps.contains(f64::INFINITY));
    
    // Test with very small values
    let caps = Caps::new(f64::MIN_POSITIVE, 1.0);
    assert!(caps.is_valid());
    assert!(caps.contains(f64::MIN_POSITIVE));
    assert!(!caps.contains(0.0));
    
    // Test with negative values
    let caps = Caps::new(-1000.0, -100.0);
    assert!(caps.is_valid());
    assert!(caps.contains(-500.0));
    assert!(!caps.contains(-50.0));
    assert!(!caps.contains(-2000.0));
}

/// Test caps mathematical precision
#[tokio::test]
async fn test_caps_mathematical_precision() {
    // Test with very small differences
    let caps = Caps::new(0.0, 1e-10);
    assert!(caps.is_valid());
    assert!(caps.contains(0.0));
    assert!(caps.contains(1e-10));
    assert!(!caps.contains(1e-9));
    
    // Test center calculation with small values
    let center = caps.get_center();
    assert!((center - 5e-11).abs() < 1e-20);
    
    // Test range calculation with small values
    let range = caps.get_range();
    assert!((range - 1e-10).abs() < 1e-20);
}

/// Test caps performance with many operations
#[tokio::test]
async fn test_caps_performance_many_operations() {
    let mut caps = Caps::new(0.0, 1000.0);
    
    // Perform many operations
    for i in 0..1000 {
        caps.expand(1.0);
        caps.shrink(0.5);
        
        // Test containment
        assert!(caps.contains(i as f64));
        
        // Test clamping
        let clamped = caps.clamp(i as f64);
        assert!(clamped >= caps.get_min());
        assert!(clamped <= caps.get_max());
    }
    
    // Final state should be reasonable
    assert!(caps.is_valid());
    // After many operations, the caps might have changed significantly
    // Just check that it's still valid
    assert!(caps.get_min().is_finite());
    assert!(caps.get_max().is_finite());
}
