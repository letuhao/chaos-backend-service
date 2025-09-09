# Resource Manager Test Guide / 测试指南 / Hướng dẫn kiểm thử

This guide defines the testing plan, coverage, and commands for Resource Manager.

## 1. Test Types
- Golden vectors (JSON) executed via harness.
- Property-based tests (Proptest) for invariants and determinism.
- Operator-mode tests for derived dimensions.
- Readiness and integration smoke tests.

## 2. Golden Vectors
- Location: `docs/resource-manager/golden_vectors/*`
- Harness: `crates/actor-core/tests/golden_vector_harness.rs`
- Cases:
  - case01_damage_and_heal_same_tick
  - case02_ooc_regen
  - case03_shield_decay
  - case04_offline_catchup

Execution:
```powershell
cd chaos-backend-service/crates/actor-core
cargo test golden_vector_harness -- --nocapture
```

## 3. Property Tests
- File: `crates/actor-core/tests/property_proptests.rs`
- Invariants:
  - Determinism: shuffling inputs yields identical snapshot.
  - Clamp: `0 ≤ *_current ≤ *_max` after resolve.
  - Idempotency: applying same batch twice with idempotency key does not double-apply.
  - Monotonicity: increasing regen does not reduce current.

Run:
```powershell
cargo test property_proptests -- --nocapture
```

## 4. Operator Mode Tests
- File: `crates/actor-core/tests/operator_mode_tests.rs`
- Verify: `hp_percentage` computed via operator-mode; fallback clamp defaults and constants.

Run:
```powershell
cargo test operator_mode_tests -- --nocapture
```

## 5. Readiness & Integration
- Readiness API: `crates/actor-core/src/production.rs::check_readiness`
- Add a small test to validate registries and cache round-trip prior to vectors.

## 6. Environment & Config
- Point loader to resource configs:
```powershell
$Env:ACTOR_CORE_CONFIG_DIR = "docs/resource-manager/configs"
```
- Switch caches during perf tests:
```powershell
$Env:ACTOR_CORE_CACHE_KIND = "lock_free"  # or: multi_layer, basic
```

## 7. CI Hints
- Run with `--features extra_buckets`.
- Generate a report of pass/fail per golden case; archive artifacts.
