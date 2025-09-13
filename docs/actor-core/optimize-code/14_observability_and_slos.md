# 14 – Observability and SLOs

Goal: Define what we measure, acceptable thresholds, and how to instrument and alert.

Key metrics
- Aggregator
  - `aggregator.resolve.latency_us` (p50/p95/p99)
  - `aggregator.errors.count`
  - `aggregator.active_subsystems.count`
  - `aggregator.cache.hit` / `aggregator.cache.miss`
- Cache (multi-layer)
  - `cache.l1.hit` / `cache.l1.miss` / `cache.l1.size`
  - `cache.l2.hit` / `cache.l2.miss` / `cache.l2.syncs`
  - `cache.l3.hit` / `cache.l3.miss` / `cache.l3.compactions`
  - `cache.set.count` / `cache.delete.count`
- Registry/Config
  - `registry.plugins.count`
  - `registry.validation.errors`

SLOs (initial)
- Aggregator resolve (single actor) p95 ≤ 400 µs; p50 ≤ 150 µs.
- L1 cache hit rate ≥ 80% under steady-state; L2 ≥ 90% cumulative hit (L1+L2) for hot keys.
- Registry validation errors == 0 in production.

Instrumentation
- Use `tracing` spans around `Aggregator::resolve` and cache get/set paths.
  - Span fields: `actor_id`, `subsystems`, `dimensions`, `hit_layer`.
- Metrics
  - Use lightweight counters/gauges (prefer `AtomicU64` for hot counters; snapshot on read).
  - Provide `get_metrics()` methods that avoid heavy allocations.

Export (optional, feature-gated)
- Under `feature = "cli"`, enable `tracing-subscriber` for pretty/JSON output.
- Provide an example to scrape metrics to stdout or a file for integration with external tooling.

Alerts (suggested)
- p95 aggregator latency > 2x baseline for 15 min → warn.
- L1 hit rate < 60% for 15 min → investigate warming/policy.
- Any registry validation error → page.

Validation
- Add a smoke test that asserts metrics counters increment on synthetic calls.
- Bench runs capture latency metrics (Criterion output) as part of PR checks.
