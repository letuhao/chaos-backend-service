
# Condition Core — Improvement Proposals (Actionable)

> Scope: architecture, config, cache, performance, testing, DevEx, integration, governance/security.  
> Goal: simplify complexity, improve observability, unlock horizontal scale, reduce conflict risk, and harden quality.

---

## 1) Architecture & Governance

### 1.1 Tiered Condition Model
- **T0 (Hot-Path)**: tiny, pure, deterministic functions only (no I/O). Must be **non-alloc**, cache-safe.
- **T1 (Warm-Path)**: reads in-memory state (actor/inventory managers), limited allocs, bounded compute.
- **T2 (Cold-Path)**: needs network/DB; **must** be batchable and offloaded to workers.
- **Rule**: Engine schedules T0 inline, T1 via micro-batches, T2 via queue (backpressure + timeouts).

### 1.2 Versioning & Deprecation
- Introduce **semver** for functions and conditions: `function_id@1`, `condition_id@2`.
- Add **Deprecation Manifest**: `{id, replacedBy, sunsetDate}`; engine warns at load-time.

### 1.3 Capability Flags
- Each function declares: `deterministic, pure, idempotent, side_effects, state_scope (actor|session|world)`.
- Scheduler uses flags to pick cache policy, co-location, and isolation.

---

## 2) Config & Plugin System

### 2.1 Typed Schema + Codegen
- Define JSON Schemas for YAML (conditions/functions).  
- Generate **Rust types** + validators (via `schemars`/`serde_json::schema`), and **TS types** for tooling.
- Ship a `condition lint` CLI that runs schema + cross-ref checks.

### 2.2 Single-Source-of-Truth
- Choose **Hybrid-first**: Interface is source-of-truth; YAML acts as **data adapter** (no logic).  
- Provide **link refs**: YAML can only reference interface-registered functions by id+version.

### 2.3 Plugin Load-Order Guard
- Topo-sort with **cycle reporter**. On cycle: print chain and disable last loaded mod.  
- Provide **“conflict pack”** report (who overrides whom), generate into `/out/conflicts/*.md`.

### 2.4 Namespacing
- Enforce `namespace:function_id@semver` (e.g., `core:get_actor_value@1`, `mod.myteam:is_raining@2`).

---

## 3) Caching & State

### 3.1 Canonical Cache Key
```
<worldId>|<tenant>|<actorId>|<conditionId>@<ver>|<paramsHash>|<stateEpoch>
```
- `stateEpoch` increments on state mutation (HP change, inventory delta) → passive invalidation.

### 3.2 Distributed Cache
- **L1**: per-thread LRU (lock-free where possible).  
- **L2**: Redis Cluster (hash-tag by `<actorId>`).  
- **L3**: optional DB snapshot for long TTL analytics conditions.

### 3.3 Admission & TTL
- Use **TinyLFU** admission for L1.  
- TTL by function category (`actor:30s`, `item:60s`, `time:300s`…), override-per-condition.

### 3.4 Negative Caching & Bloom Filters
- Cache “not found/false” with short TTL.  
- Use Bloom filter per-actor for quick “has_item?/has_spell?” precheck.

---

## 4) Performance & Scheduling

### 4.1 Micro-Batching
- Group N evaluations by (`actorId`, `functionId`) to reuse fetched state (HP, inventory).  
- **Watermarking**: flush batch every X ms or Y items.

### 4.2 Workload Classes
- **Latency class** (UI-critical): deadline 5–10 ms, no network.  
- **Throughput class** (background): deadlines 100–500 ms, allows network.

### 4.3 Zero-Copy Results
- Use small-value optimization and arenas for `ConditionValue` to reduce allocs.

### 4.4 OpenTelemetry
- Trace per evaluation with attributes: `condition_id`, `function_id`, `cache_level`, `ttl`, `miss_reason`.

---

## 5) Testing & Quality

### 5.1 Property & Fuzz
- `proptest` for numeric invariants (e.g., symmetry of operators, clamp ranges).  
- Fuzz parsers (YAML/Interface) against config injections and malformed inputs.

### 5.2 Workload Replay
- Export real sessions as **trace bundles** (NDJSON). Stress engine with time-warp to mimic MMO spikes.

### 5.3 Golden Baselines
- Golden vectors per function (input → expected) across versions.  
- Run **compat suite** to detect breaking changes before releasing function@new version.

### 5.4 Security Tests
- YAML anchor bombs, billion-laughs, oversized arrays, regex DoS for matchers.

---

## 6) DevEx & Tooling

### 6.1 Condition Studio (Web UI)
- Browse functions, categories, live docs from metadata.  
- Try-run with a mock actor; show **explain plan** (cache hits/misses, tiers, timing).

### 6.2 CLI
- `cond lint`, `cond plan`, `cond bench`, `cond heatmap` (per-id QPS & p99).  
- `cond conflicts` to diff plugin load order and overrides.

### 6.3 Codegen
- Generate stubs for new functions with templates incl. metadata, tests, and benchmarks.

---

## 7) Integration Patterns

### 7.1 Actor/Inventory Co-Location
- Run evaluation workers **co-located** with actor shards to keep hot state in-memory.  
- Use a stable shard key = `hash(actorId) % shards`.

### 7.2 Event-Driven Invalidations
- Publish `ActorStateChanged`, `InventoryChanged`, `WeatherChanged` → bump `stateEpoch` and wipe precise keys.

### 7.3 Read Models
- Precompute popular composite conditions (e.g., “low health & in combat”) into a read model per shard.

---

## 8) Security & Limits

- Per-tenant quotas: max conditions, max QPS, max depth for boolean logic.  
- Sandboxing user plugins (WASM or constrained script) with CPU/mem/time limits.  
- Sign plugin bundles; verify signature before load.  
- Feature flags to gate new categories/functions.

---

## 9) Rollout Plan (4 Sprints)

1. **Sprint 1** – Schema & Lint, cache key, versioning, OpenTelemetry basic.
2. **Sprint 2** – L1 TinyLFU, L2 Redis cluster keys, stateEpoch invalidations, micro-batching.
3. **Sprint 3** – Plugin conflict reporter + topo-sort, Condition Studio (MVP), workload trace player.
4. **Sprint 4** – Security fuzz + golden compatibility, throughput class worker pool, shard co-location.

---

## 10) Checklists

### 10.1 Readiness
- [ ] JSON Schemas published & codegen green
- [ ] CLI `cond lint` blocks bad configs
- [ ] OTel traces visible with `condition_id` tags
- [ ] Cache key includes `stateEpoch`

### 10.2 Perf SLOs
- [ ] Single eval p99 < 1 ms (cache hit)
- [ ] Batch 100 evals p99 < 10 ms
- [ ] L1 hit rate > 80%, L1+L2 > 95%

---

## 11) Reference Snippets

### 11.1 Suggested Redis Key
```
RC:<tenant>:<actorId>:<condId>@<ver>:<paramsHash>:<epoch>
```

### 11.2 Function Metadata (extended)
```rust
struct FunctionMetadata {
  id: String,
  version: SemVer,
  category: Category,
  deterministic: bool,
  pure: bool,
  idempotent: bool,
  state_scope: StateScope, // Actor | Session | World
  cache_ttl: Option<Duration>,
  admission: Admission,    // Always | TinyLFU | Never
}
```

---

**Deliverables produced by this doc**: schemas, lint CLI, OTel integration, cache key upgrade, Redis rollout, batcher, plugin conflict reports, studio UI MVP, test harnesses (property/fuzz/golden), shard co-location plan.
