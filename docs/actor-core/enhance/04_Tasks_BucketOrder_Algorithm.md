# Phase 4 — Enforce Bucket Order & Clamping

## Goal
Apply contribution buckets in strict order: `FLAT → MULT → POST_ADD → OVERRIDE`, then clamp per Combiner rules.

## Steps
- Centralize ordering in one function (e.g., `apply_buckets_in_order()`).
- Add logs/trace (feature-gated) for debugging order issues.
- Final stage: clamp to [min, max] as defined by Combiner.

## Exact Commands
```powershell
git checkout -b feat/bucket-order
cargo test -p actor-core -- --nocapture
```

## Acceptance Criteria
- Deterministic results for identical inputs regardless of input order.
- Unit tests proving order and clamping.
