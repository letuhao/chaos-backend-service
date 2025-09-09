//! Utility functions and helpers for the Chaos World backend.

use std::time::{SystemTime, UNIX_EPOCH};

/// Get the current timestamp as milliseconds since Unix epoch.
pub fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Get the current timestamp as seconds since Unix epoch.
pub fn current_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Clamp a value between min and max (inclusive).
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation between two values.
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Check if two floating point values are approximately equal.
pub fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Generate a random string of specified length.
pub fn random_string(length: usize) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    current_timestamp_ms().hash(&mut hasher);
    format!("{:x}", hasher.finish())[..length.min(16)].to_string()
}
