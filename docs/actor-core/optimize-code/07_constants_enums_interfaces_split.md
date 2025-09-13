# 07 – Split constants, enums, and interfaces from implementations

Goal: Ensure `constants/`, `enums`, and `interfaces` contain only data/constants and trait definitions, with no concrete implementations. Move metrics/enums currently misplaced and update imports consistently.

Edits
- File: `crates/actor-core/src/interfaces.rs`
  - Keep: trait definitions only (`Subsystem`, `Aggregator`, `CapsProvider`, `PluginRegistry`, `CombinerRegistry`, `CapLayerRegistry`, and optional traits).
  - Move out the following types to dedicated modules:
    - Move enum `AcrossLayerPolicy` → `crates/actor-core/src/enums.rs` (new or append).
    - Move structs `SubsystemMetrics`, `AggregatorMetrics`, `CapStatistics`, `CacheStats` → new file `crates/actor-core/src/metrics.rs`.
  - Remove the inline `#[cfg(test)] mod tests` module; tests should live in `crates/actor-core/tests/interfaces_tests.rs` (see 06_tests_updates.md).

- File: `crates/actor-core/src/enums.rs`
  - Append the definition of `AcrossLayerPolicy`:
    - Signature remains identical (`Intersect`, `Union`, `PrioritizedOverride`).
  - Ensure `serde` derives are preserved.

- File: `crates/actor-core/src/metrics.rs` (new)
  - Add the following structs with the same fields and derives:
    - `SubsystemMetrics`
    - `AggregatorMetrics`
    - `CapStatistics`
    - `CacheStats`
  - Preserve `Default` impls exactly.

- File: `crates/actor-core/src/lib.rs`
  - Add: `pub mod metrics;`
  - Update re-exports:
    - Replace `pub use interfaces::{..., Aggregator, CapsProvider, PluginRegistry, CombinerRegistry, CapLayerRegistry, Cache};` to drop metric types and `AcrossLayerPolicy` from this list.
    - Add `pub use metrics::{SubsystemMetrics, AggregatorMetrics, CapStatistics, CacheStats};`
    - Ensure `pub use enums::*;` now includes `AcrossLayerPolicy` consumers.

- File: `crates/actor-core/src/services.rs` (or `services/aggregator.rs` after split)
  - Update imports of moved types:
    - `use crate::interfaces::AggregatorMetrics` → `use crate::metrics::AggregatorMetrics`
    - If any reference to `CacheStats`/`CapStatistics` exists, update to `crate::metrics::*`.
    - If any code imports `AcrossLayerPolicy` from interfaces, change to `use crate::enums::AcrossLayerPolicy`.

- File: `crates/actor-core/src/registry.rs`
  - Update imports of `AcrossLayerPolicy` and metric structs similarly:
    - `crate::interfaces::{..., AcrossLayerPolicy}` → `crate::enums::AcrossLayerPolicy`
    - Any metrics types → `crate::metrics::*`.

- File: `crates/actor-core/src/cache.rs` and `src/cache/multi_layer.rs`
  - Update `CacheStats` usage to `crate::metrics::CacheStats`.

Search/replace guidance
- Replace imports:
  - `use crate::interfaces::AcrossLayerPolicy` → `use crate::enums::AcrossLayerPolicy`
  - `use crate::interfaces::AggregatorMetrics` → `use crate::metrics::AggregatorMetrics`
  - `use crate::interfaces::CapStatistics` → `use crate::metrics::CapStatistics`
  - `use crate::interfaces::CacheStats` → `use crate::metrics::CacheStats`

Validation
- Build and test: `cargo test -p actor-core`.
- Ensure no concrete implementations remain in `interfaces.rs`.
- Confirm public API re-exports in `lib.rs` expose metrics and enums from their new modules.
