# 05 – Logging and metrics consistency

Goal: Consistent tracing fields and cheap metrics across components.

Edits
- Files: `crates/actor-core/src/services.rs` (or `services/aggregator.rs`), `registry.rs`, `cache.rs`, `cache/multi_layer/*`
  - Replace println! with tracing macros where any occur.
  - Ensure spans/fields include `actor_id`, `system_id`, `dimension` when relevant.
  - Metrics structs: access via RwLock where synchronous; prefer parking_lot locks.
  - Expose cheap getters; avoid heavy cloning in `get_stats` (return copy or lightweight struct).

Validation
- Compile with `--features cli` to ensure tracing-subscriber used only in CLI or tests.
