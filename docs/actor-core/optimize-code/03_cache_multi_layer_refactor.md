# 03 – Cache: Refactor multi_layer.rs into submodules

Goal: Reduce 1754-line monolith into clear submodules: policy, layers, backends, warming, compose.

Edits (file → symbol → action)
- File: `crates/actor-core/src/cache/multi_layer.rs`
  - Move type `MultiLayerConfig` (lines ~39–58) to `crates/actor-core/src/cache/multi_layer/policy.rs`.
  - Move enum `EvictionPolicy` (lines ~61–71) to `multi_layer/policy.rs`.
  - Move stats structs `MultiLayerStats`, `L1CacheStats`, `L2CacheStats`, `L3CacheStats` (lines ~73–200) to `multi_layer/metrics.rs`.
  - Move traits `L1Cache`, `L2Cache`, `L3Cache` (lines ~96–166) to `multi_layer/layers.rs`.
  - Keep `MultiLayerCacheManager` in `multi_layer/manager.rs` with only orchestration logic and background sync.
  - Extract file I/O backend helpers (memmap, disk compaction) into `multi_layer/backends/{memmap.rs, disk.rs}`.
  - Extract preloading/warming code paths into `multi_layer/warming.rs`.

New files to create
- `crates/actor-core/src/cache/multi_layer/mod.rs`
  - `pub mod policy; pub mod metrics; pub mod layers; pub mod backends; pub mod warming; pub mod manager;`
  - `pub use policy::*; pub use metrics::*; pub use layers::*; pub use manager::*;`
- `crates/actor-core/src/cache/multi_layer/policy.rs` (config + eviction policy)
- `crates/actor-core/src/cache/multi_layer/metrics.rs` (all stats types)
- `crates/actor-core/src/cache/multi_layer/layers.rs` (L1/L2/L3 traits)
- `crates/actor-core/src/cache/multi_layer/manager.rs` (MultiLayerCacheManager)
- `crates/actor-core/src/cache/multi_layer/backends/{mod.rs, memmap.rs, disk.rs}`
- `crates/actor-core/src/cache/multi_layer/warming.rs`

Compatibility shims
- In `cache.rs`, replace `pub mod multi_layer;` to point at the new folder `multi_layer/` with same public re-exports.

Validation
- Ensure `use` paths updated; compile with `cargo test -p actor-core`.
- Add unit tests per submodule for moved types/traits (copy existing tests if any).
