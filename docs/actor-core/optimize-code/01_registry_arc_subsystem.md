# 01 – Registry: Use Arc<dyn Subsystem> end-to-end

Goal: Remove Box↔Arc wrapper and simplify trait object plumbing.

Edits
- File: `crates/actor-core/src/interfaces.rs`
  - Trait `PluginRegistry`
    - Change signatures:
      - `fn register(&self, subsystem: Box<dyn Subsystem>) -> ActorCoreResult<()>;`
        → `fn register(&self, subsystem: std::sync::Arc<dyn Subsystem>) -> ActorCoreResult<()>;`
      - `fn get_by_id(&self, system_id: &str) -> Option<Box<dyn Subsystem>>;`
        → `fn get_by_id(&self, system_id: &str) -> Option<std::sync::Arc<dyn Subsystem>>;`
      - `fn get_by_priority(&self) -> Vec<Box<dyn Subsystem>>;`
        → `fn get_by_priority(&self) -> Vec<std::sync::Arc<dyn Subsystem>>;`
      - `fn get_by_priority_range(&self, min_priority: i64, max_priority: i64) -> Vec<Box<dyn Subsystem>>;`
        → `fn get_by_priority_range(&self, min_priority: i64, max_priority: i64) -> Vec<std::sync::Arc<dyn Subsystem>>;`

- File: `crates/actor-core/src/registry.rs`
  - Remove `SubsystemWrapper` struct and its impl block entirely.
  - Struct `PluginRegistryImpl` remains the same.
  - Impl `PluginRegistry for PluginRegistryImpl`
    - Method `register`:
      - Input type from `Box<dyn SubsystemTrait>` → `Arc<dyn SubsystemTrait>`
      - Remove Box→Arc conversion; directly insert the Arc.
    - Method `get_by_id`:
      - Return `Option<Arc<dyn SubsystemTrait>>` and return the stored Arc clone.
    - Method `get_by_priority`:
      - Build `Vec<Arc<dyn SubsystemTrait>>` directly; sort by `priority()`.
    - Method `get_by_priority_range`:
      - Return `Vec<Arc<dyn SubsystemTrait>>` directly; sort by `priority()`.

- File: `crates/actor-core/src/services.rs`
  - Struct `AggregatorImpl` field `subsystem_registry: Arc<dyn PluginRegistry>` unchanged.
  - Update call sites that consume results from `PluginRegistry` to handle `Arc<dyn Subsystem>` instead of `Box` (iterate over Arcs; no `.as_ref()` changes needed for trait calls).

- File: `crates/actor-core/src/interfaces.rs` (tests section)
  - If tests instantiate mocks based on `Box`, update to `Arc::new(MockSubsystem::new(...))` where needed.

Notes
- No behavior change; compile-only API change. Update all use-sites within `actor-core` to Arc.
- Run `cargo test -p actor-core` after edits; fix any type mismatches.
