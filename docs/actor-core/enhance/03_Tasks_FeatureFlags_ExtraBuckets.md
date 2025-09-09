# Phase 3 — Gate Extra Buckets Behind Feature Flags

## Goal
Buckets beyond the core 4 (FLAT/MULT/POST_ADD/OVERRIDE) should be behind a feature flag to avoid doc drift.

## Steps
1. In `Cargo.toml`, add:
```toml
[features]
default = []
extra_buckets = []
```
2. In the enum `Bucket`, wrap non-core variants with `#[cfg(feature = "extra_buckets")]`.
3. Add tests that compile with and without the feature.

## Exact Commands (PowerShell)
```powershell
git checkout -b feat/extra-buckets-feature
# Edit Cargo.toml and the enum file
cargo test
cargo test --features extra_buckets
```

## Acceptance Criteria
- Crate compiles and tests pass with both profiles (feature off/on).
- Docs updated to mention the feature toggle (see Phase 9).
