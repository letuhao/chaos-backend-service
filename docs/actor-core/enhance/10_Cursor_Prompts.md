# Cursor AI — Copy-Paste Prompts

## P1 — Registry Loader
You are Cursor AI. Read `01_Tasks_RegistryLoader.md` and `configs/*.yaml`. Implement `src/registry/loader.rs` with functions:
- `load_cap_layers(path)`
- `load_combiner(path)`
- `load_all(cfg_dir)`
Use `serde_yaml`, `serde_json`, `thiserror`, `anyhow`. Validate schema minimally (non-empty, unique names, legal enum variants). Write unit tests covering YAML/JSON/invalid cases.

## P2 — EffectiveCaps Alias
Add `pub type EffectiveCaps = std::collections::HashMap<String, Caps>;` in the types module and refactor references where appropriate without changing serialization. Run fmt/clippy/tests.

## P3 — Feature Flags for Extra Buckets
Add `[features] extra_buckets` to Cargo.toml. Wrap non-core bucket variants in `#[cfg(feature = "extra_buckets")]`. Provide tests compiling with and without the feature.

## P4 — Bucket Order & Clamping
Centralize bucket order application in one function and clamp per Combiner rules. Add tests that prove determinism and proper clamping.

## P5 — Config Samples
Ensure loader reads `configs/cap_layers.yaml` and `configs/combiner.yaml`. If missing, create fallbacks.

## P6 — Tests
Add property-based tests using `proptest` as described in `06_Tasks_Tests.md`.

## P7 — CI Checks
Wire `fmt`, `clippy`, optional `deny`, and docs build into CI. Make sure all pass.

## P8 — Benchmarks
Add Criterion benchmarks comparing programmatic vs file-loaded registries.

## P9 — Docs Sync
Update design docs to reflect alias, features, YAML examples, and ordering rules.
