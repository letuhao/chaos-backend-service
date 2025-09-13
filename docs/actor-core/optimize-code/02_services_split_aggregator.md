# 02 – Services: Split aggregator and caps provider

Goal: Improve navigability by splitting `services.rs` into focused modules.

Edits
- File: `crates/actor-core/src/services.rs`
  - Extract struct `AggregatorImpl` and all associated methods into `crates/actor-core/src/services/aggregator.rs`.
  - Extract `CapsProvider` concrete implementation(s) (if present) into `crates/actor-core/src/services/caps_provider.rs`.
  - Leave a shim `mod` file that `pub mod aggregator; pub mod caps_provider;` and (temporarily) `pub use services::{aggregator::*, caps_provider::*};` in `lib.rs` to preserve public API.

- File: `crates/actor-core/src/lib.rs`
  - Replace `pub mod services;` with:
    - `pub mod services { pub mod aggregator; pub mod caps_provider; }`
    - Add compatibility re-exports if needed.

- Files to create:
  - `crates/actor-core/src/services/aggregator.rs` (copy exact code blocks for `AggregatorImpl` and helpers: `aggregate_primary_stats`, `process_contributions_with_operator`, `aggregate_derived_stats`, and any private helpers used solely by aggregator).
  - `crates/actor-core/src/services/caps_provider.rs` (copy caps provider impls if present; otherwise leave placeholder module with TODO removed before commit).

Notes
- No logic changes; only module moves. Preserve imports and visibility exactly.
- After moves, run `cargo test -p actor-core` to confirm no regressions.
