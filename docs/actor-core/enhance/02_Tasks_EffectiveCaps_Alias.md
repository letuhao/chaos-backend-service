# Phase 2 — Add `EffectiveCaps` Alias

## Goal
Align code with docs by adding a public alias:
```rust
pub type EffectiveCaps = std::collections::HashMap<String, Caps>;
```

## Exact Commands (PowerShell)
```powershell
git checkout -b feat/effective-caps-alias
# Open the types module and add the alias (usually in src/types/mod.rs or similar)
# Then run checks:
cargo fmt
cargo clippy --all-targets --all-features -D warnings
cargo test
```

## Notes
- Replace doc references to `Snapshot.caps_used` with `EffectiveCaps` where appropriate.
- Do **not** break existing serialization format (ensure alias keeps same layout).

## Acceptance Criteria
- Alias compiles and is exported from crate public API if needed.
- No breaking changes to existing tests.
