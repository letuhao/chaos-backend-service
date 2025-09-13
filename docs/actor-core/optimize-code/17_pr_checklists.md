# 17 – PR Checklists (per phase)

Use these in PR descriptions. Link to the corresponding phase document and rules.

General (all PRs)
- [ ] Docs: reference `docs/actor-core/optimize-code/XX_*.md`
- [ ] Build/tests: `cargo test -p actor-core` green
- [ ] Lints: fmt/clippy/deny clean
- [ ] Benches: within thresholds (if applicable)
- [ ] Changelog: updated for any API changes

Phase 01 – Registry (Arc<dyn Subsystem>)
- [ ] `interfaces.rs` signatures updated to Arc
- [ ] `registry.rs` wrapper removed; methods return/accept Arc
- [ ] `services.rs` call sites updated
- [ ] Tests pass; no behavior change

Phase 02 – Services split
- [ ] `services/aggregator.rs` created; logic moved
- [ ] `services/caps_provider.rs` created (if impls exist)
- [ ] `lib.rs` re-exports adjusted
- [ ] No logic changes; imports only

Phase 03 – Cache multi-layer refactor
- [ ] `multi_layer/` submodules created per plan
- [ ] Types/traits/stats moved to respective files
- [ ] `cache.rs` re-exports updated
- [ ] Behavior verified with tests

Phase 06 – Tests layout
- [ ] `tests/*.rs` files created per mappings
- [ ] In-file tests removed only when safe
- [ ] Imports use prelude or new paths

Phase 07 – Constants/Enums/Interfaces split
- [ ] `AcrossLayerPolicy` moved to `enums.rs`
- [ ] Metrics types moved to `metrics.rs`; `lib.rs` exposes `mod metrics`
- [ ] `interfaces.rs` trait-only; imports updated elsewhere

Phase 08 – Coding rules migration
- [ ] i64/f64 domain types enforced
- [ ] parking_lot locks in sync paths; no std locks there
- [ ] tracing replaces println!; context fields added
- [ ] unwrap/expect/panic removed from non-test code
- [ ] Bilingual headers added

Phase 09 – Profiling
- [ ] Criterion benches run
- [ ] Thresholds met; deltas within 10%

Phase 10 – CI matrix
- [ ] Workflows updated for features and gates

Phase 12 – Prelude & API
- [ ] `prelude.rs` added; `lib.rs` uses it
- [ ] Minimize public re-exports

Phase 14 – Observability
- [ ] Spans on `Aggregator::resolve`; cache get/set
- [ ] Metrics getters cheap; smoke test added

Phase 15 – Deprecation & rollback
- [ ] Shims/deprecations documented; timeline stated
- [ ] Rollback steps noted
