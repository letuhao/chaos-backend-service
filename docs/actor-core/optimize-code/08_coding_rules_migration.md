# 08 – Coding rules migration (types, locks, logging, docs)

Goal: Migrate existing code to conform to the new coding rules without changing behavior.

## A. Types: i64/f64 across domain logic

Targets
- `crates/actor-core/src/types.rs`
- Any domain structs/functions using `i32`/`f32`

Edits
- Replace domain numeric fields to `i64`/`f64`:
  - Example: in `types.rs`, ensure all numeric fields in `Actor`, `Caps`, contributions, and snapshots use `i64`/`f64`.
- Audit helpers/utilities to avoid implicit integer/float coercions.

Validation
- `cargo test -p actor-core`

## B. Concurrency: replace std locks with parking_lot (sync-only paths)

Targets and blocks
- File: `crates/actor-core/src/cache.rs`
  - Struct `InMemoryCache`
    - Field: `storage: Arc<std::sync::RwLock<HashMap<String, CacheEntry>>>`
      → `storage: Arc<parking_lot::RwLock<HashMap<String, CacheEntry>>>`
    - Field: `metrics: Arc<std::sync::RwLock<CacheStats>>`
      → `metrics: Arc<parking_lot::RwLock<CacheStats>>`
    - All occurrences of `.read().unwrap()` / `.write().unwrap()`
      → `.read()` / `.write()`
  - Struct `LockFreeInMemoryCache`
    - Field: `metrics: Arc<std::sync::RwLock<CacheStats>>`
      → `metrics: Arc<parking_lot::RwLock<CacheStats>>`
    - All lock usages in methods: `get`, `set`, `delete`, `clear`, `get_stats`
      → use parking_lot guards (no unwrap)

- File: `crates/actor-core/src/registry.rs`
  - Struct `PluginRegistryImpl`
    - Field: `subsystems: Arc<std::sync::RwLock<HashMap<String, Arc<dyn SubsystemTrait>>>>>`
      → `Arc<parking_lot::RwLock<...>>`
    - Field: `metrics: Arc<std::sync::RwLock<RegistryMetrics>>`
      → `Arc<parking_lot::RwLock<RegistryMetrics>>`
    - Replace `.read().unwrap()` / `.write().unwrap()` accordingly.

- File: `crates/actor-core/src/cache/multi_layer.rs`
  - Struct `MultiLayerCacheManager`
    - Field: `stats: Arc<RwLock<MultiLayerStats>>` (std)
      → `stats: Arc<parking_lot::RwLock<MultiLayerStats>>`
    - Replace imports and lock calls accordingly.

Notes
- Keep `tokio::RwLock` only where locks cross `await`. The above are sync-paths.

Validation
- Compile and run tests after each file change.

## C. Logging: use tracing; remove println!

Targets
- Scan all files in `crates/actor-core/src/**` for `println!`.
- Replace with `tracing::{info,warn,error,debug}` as appropriate; add context fields: `actor_id`, `system_id`, `dimension`.

Validation
- Ensure no `println!` remains in non-test code.

## D. Errors: remove unwrap/expect/panic in non-test code

Targets
- Scan all files in `crates/actor-core/src/**` for `unwrap`, `expect`, `panic!`.
- Replace with `?` and proper `ActorCoreError` variants.

Validation
- Build ensures no stray unwraps remain (except tests/benches).

## E. Public API re-exports via prelude (minimal surface)

Targets
- File: `crates/actor-core/src/lib.rs`
  - Add: `pub mod prelude;`
- File: `crates/actor-core/src/prelude.rs` (new)
  - `pub use crate::types::{Actor, Contribution, Caps, Snapshot};`
  - `pub use crate::interfaces::{Subsystem, Aggregator, CapsProvider, PluginRegistry};`
  - Add others as needed by downstream crates.

Validation
- Consumers can `use actor_core::prelude::*;`

## F. Bilingual module headers and public docs

Targets
- Top of key modules: `services/aggregator.rs`, `registry.rs`, `cache/multi_layer/*`, `interfaces.rs`, `types.rs`.
- Add bilingual headers (Chinese + Vietnamese) describing intent and invariants.

Validation
- Ensure docs compile and render.

## G. Cursor rules presence (reference only)

- `.cursor/rules/*.mdc` already define rules for types, locks, modules, caching, logging/tests.
- No code change here; ensure files exist and `alwaysApply: true` for actor-core globs.
