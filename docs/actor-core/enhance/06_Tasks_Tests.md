# Phase 6 — Tests: Unit + Property-Based

## Goal
Ensure loader and aggregation are correct & robust.

### Unit Tests
- Loader: happy YAML, fallback JSON, invalid value, duplicate ids, empty list.
- Bucket order: enforce FLAT → MULT → POST_ADD → OVERRIDE.
- Clamping: respect Combiner min/max.

### Property Tests (proptest)
- Randomized contributions across buckets should yield same result regardless of input order.
- Values always in [min, max] after clamping.
- With/without `extra_buckets` feature should yield identical results when extra buckets are not used.

## Commands
```powershell
git checkout -b test/coverage
cargo add proptest@1 --dev
cargo test
cargo test --features extra_buckets
```

## Acceptance Criteria
- >90% coverage on aggregation services (guideline).
- Property tests catch ordering regressions.
