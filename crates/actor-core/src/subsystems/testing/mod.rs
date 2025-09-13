//! Testing and Integration
//!
//! This module contains testing utilities and integration test suites
//! for the Actor Core system.

pub mod integration_tests;

// Re-export commonly used testing components
pub use integration_tests::{IntegrationTestSuite, ComprehensiveTestResults};
