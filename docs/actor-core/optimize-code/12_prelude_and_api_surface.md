# 12 – Prelude and public API surface

Goal: Provide a minimal, stable import surface for consumers.

Files
- `crates/actor-core/src/prelude.rs` (new)
  - Export only high-level types and traits:
    - `pub use crate::types::{Actor, Contribution, Caps, Snapshot};`
    - `pub use crate::interfaces::{Subsystem, Aggregator, CapsProvider, PluginRegistry};`
    - `pub use crate::enums::*;`
    - `pub use crate::metrics::{SubsystemMetrics, AggregatorMetrics, CapStatistics, CacheStats};`
- `crates/actor-core/src/lib.rs`
  - `pub mod prelude;`
  - Avoid blanket `pub use` of entire implementation modules.

Policy
- Keep concrete implementations opt-in (e.g., `services::aggregator::AggregatorImpl`).
- Document deprecation plan for any legacy re-exports (timeline and versions).

Validation
- Ensure services and tests compile by importing `prelude::*`.
