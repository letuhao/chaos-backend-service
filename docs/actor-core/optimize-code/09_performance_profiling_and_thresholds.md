# 09 – Performance profiling and thresholds

Goal: Establish a repeatable workflow for profiling and pass/fail thresholds.

Benches
- Ensure Criterion benches exist: `benches/actor_benchmarks.rs`, `bucket_processor_benchmarks.rs`, `registry_loader_benchmarks.rs` (already present).
- Command: `cargo bench -p actor-core`.

Flamegraph (optional)
- Install: `cargo install flamegraph` (requires perf on Linux).
- Run: `cargo flamegraph -p actor-core --bench actor_benchmarks`.

Thresholds (initial)
- Aggregator resolve (single actor): median ≤ 150 µs (default features).
- Bucket processor (100 contributions): median ≤ 30 µs.
- Cache get (L1 hit): median ≤ 1 µs; set ≤ 5 µs.
- Multi-layer miss→L3: median ≤ 2 ms.

Regression policy
- If median increases by >10% or exceeds absolute thresholds, block merge.
- Document deltas in PR; attach Criterion report.

Automation
- Save Criterion output under `target/criterion/**`; compare previous run (baseline branch `main`).
- Optional: use `cargo-criterion` JSON for CI gating.
