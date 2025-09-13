# 06 – Tests and benches updates

Goal: Define the test layout (no implementation in this plan).

Target structure (actor-core)
- `crates/actor-core/tests/`
  - `interfaces_tests.rs`
  - `production_readiness_tests.rs`
  - `services_aggregator_tests.rs`
  - `cache_multi_layer_tests.rs`
  - `registry_tests.rs`
  - `property_proptests.rs`
- `crates/actor-core/benches/`
  - existing benches remain

Mappings (from implementation files to test files)
- `src/interfaces.rs` → `tests/interfaces_tests.rs`
- `src/production.rs` → `tests/production_readiness_tests.rs`
- `src/services.rs` (or `services/aggregator.rs`) → `tests/services_aggregator_tests.rs`
- `src/cache.rs` and `src/cache/multi_layer.rs` → `tests/cache_multi_layer_tests.rs`
- `src/registry.rs` → `tests/registry_tests.rs`
- Property-based tests → `tests/property_proptests.rs`

Guidelines
- Keep in-file tests only when private internals are required; otherwise prefer `tests/`.
- Use public API or `pub(crate)` test helpers rather than accessing private symbols.
- Use `tracing` (no println!).

Run (after implementation is done)
- `cargo test -p actor-core`
- `cargo bench -p actor-core` (optional for hot paths)
