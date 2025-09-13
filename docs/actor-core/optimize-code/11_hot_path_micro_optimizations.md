# 11 – Hot path micro-optimizations

Goal: Targeted improvements without architectural changes.

Aggregator and bucket processor
- Use small-vector optimization (e.g., `arrayvec`/`smallvec`) for short contribution lists (feature-gated).
- Optionally use `rustc_hash::FxHashMap` for per-dimension aggregation under `feature = "fxhash"`.
- Avoid allocations by reusing buffers where possible.
- Pre-sort contributions once by bucket/priority to avoid repeated sorts.

Metrics
- Replace lock-protected counters with `AtomicU64` where write-only increments are sufficient;
  snapshot into structs for read APIs.

Iteration determinism
- Where output ordering matters, use deterministic iteration (sorted keys or `BTreeMap`).

Inlining and branches
- Annotate trivial getters with `#[inline]`.
- Avoid unpredictable branches in hot loops; hoist conditionals.

Validation
- Benchmark before/after; guard with features to disable on regressions.
