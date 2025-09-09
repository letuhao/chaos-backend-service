# Actor‑Core — Missing Items & Completion Tasks (for Cursor AI)
_Date: 2025-09-09_

This document lists **all gaps** found after your latest implementation and provides **copy‑pasteable tasks** for Cursor AI to finish the Actor‑Core according to the design SPEC. Each item includes *What/Why/Where/How/Acceptance Criteria* and **exact commands**.

---

## P0 Blockers (fix these first)

### 1) Public **Function Contracts** not exposed (ComposeCore/BaseFromPrimary/FinalizeDerived/ClampDerived)
**What**: Missing public API surface specified as the highest‑level contract.  
**Why**: Other systems (combat, gameplay logic) depend on stable entry points; ensures determinism & uniform clamps.  
**Where**: Create new module `src/core_api.rs` and re‑export from `lib.rs`.

**How (Implementation Guide)**
- Add file `src/core_api.rs` with the following public signatures (you may delegate to existing services/helpers):
  ```rust
  // src/core_api.rs
  use std::collections::BTreeMap;
  use crate::types::*;

  /// Compose contributions into a single CoreContribution in a deterministic manner.
  /// Determinism requirement: lexicographic order by key when folding maps.
  pub fn compose_core(contribs: &BTreeMap<String, CoreContribution>) -> CoreContribution {
      // TODO: fold in key order; reuse bucket_processor where appropriate.
      unimplemented!()
  }

  /// Compute base Derived stats directly from PrimaryCore (and possibly level or seeds).
  pub fn base_from_primary(primary: &PrimaryCore, level: i64) -> Derived {
      // TODO: implement Phase‑1 formulas per SPEC, keep it pure and deterministic.
      unimplemented!()
  }

  /// Apply flat/mult/post‑add/override maps to the base Derived and return final Derived (pre‑clamp).
  pub fn finalize_derived(base: Derived, flats: &FlatMap, mults: &MultMap) -> Derived {
      // TODO: apply bucket order and return un‑clamped derived; then clamp in clamp_derived().
      unimplemented!()
  }

  /// Clamp every field of Derived to the legal ranges specified in the SPEC.
  pub fn clamp_derived(derived: Derived) -> Derived {
      // TODO: cap each field (e.g., HPMax>=1, stamina>=1, haste in [0.5, 2.0], critChance in [0,1], etc.).
      unimplemented!()
  }
  ```
- Update `src/lib.rs`:
  ```rust
  pub mod core_api;
  pub use core_api::{compose_core, base_from_primary, finalize_derived, clamp_derived};
  ```

**Acceptance Criteria**
- Public functions exist with the exact names above.
- Unit tests prove they run and are deterministic for same inputs.
- `clamp_derived` enforces **all** field ranges (see Item 4).

**Commands (PowerShell)**
```powershell
git checkout -b feat/core-api
ni -ItemType File ./actor-core/src/core_api.rs
git add .
git commit -m "feat(core): expose ComposeCore/BaseFromPrimary/FinalizeDerived/ClampDerived API"
```

---

### 2) Missing **PrimaryCore** & **Derived** data models (SPEC fields)
**What**: Structures representing the primary and derived stat sets are not present as SPEC terms.  
**Why**: Required by Function Contracts and for clamp coverage & property tests.  
**Where**: `src/types_primary.rs`, `src/types_derived.rs` (or inside `types.rs` split by modules).

**How**
- Add subset Phase‑1 fields to start (expand later). Example:
  ```rust
  // src/types_primary.rs
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
  pub struct PrimaryCore {
      pub vitality: f64,
      pub endurance: f64,
      pub constitution: f64,
      // add other needed core stats…
  }

  // src/types_derived.rs
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
  pub struct Derived {
      pub hp_max: f64,       // ≥ 1
      pub stamina_max: f64,  // ≥ 1
      pub speed: f64,        // > 0 (and practical caps per SPEC)
      pub crit_chance: f64,  // 0..=1
      pub crit_multi: f64,   // ≥ 1
      // … add fields incrementally per SPEC sections
  }
  ```
- Re‑export in `lib.rs` and wire into `core_api.rs`.

**Acceptance Criteria**
- Both structs exist and are serialized deterministically.
- No NaN/Inf allowed in constructors; validation returns Result or clamps early.

**Commands**
```powershell
git checkout -b feat/types-primary-derived
ni -ItemType File ./actor-core/src/types_primary.rs
ni -ItemType File ./actor-core/src/types_derived.rs
git add .
git commit -m "feat(types): add PrimaryCore & Derived structs for core API"
```

---

### 3) **Combat Registries** (DamageType/DefenceType/AmplifierType) & Register* APIs
**What**: Registry interfaces & functions to register damage/defence/amplifier types are missing.  
**Why**: SPEC expects runtime‑extensible registries for combat calculations.  
**Where**: Create `src/registry/combat.rs` and re‑export from `lib.rs`.

**How**
```rust
// src/registry/combat.rs
use std::collections::HashMap;
use std::sync::RwLock;

pub trait DamageType: Send + Sync { /* fn id(&self)->&str; fn apply(&self, base:f64)->f64; … */ }
pub trait DefenceType: Send + Sync { /* … */ }
pub trait AmplifierType: Send + Sync { /* … */ }

lazy_static::lazy_static! {
    static ref DAMAGE_REG: RwLock<HashMap<String, Box<dyn DamageType>>> = RwLock::new(HashMap::new());
    static ref DEFENCE_REG: RwLock<HashMap<String, Box<dyn DefenceType>>> = RwLock::new(HashMap::new());
    static ref AMPLIFIER_REG: RwLock<HashMap<String, Box<dyn AmplifierType>>> = RwLock::new(HashMap::new());
}

pub fn register_damage_type(id: &str, t: Box<dyn DamageType>) { /* insert with dedup check */ }
pub fn register_defence_type(id: &str, t: Box<dyn DefenceType>) { /* … */ }
pub fn register_amplifier_type(id: &str, t: Box<dyn AmplifierType>) { /* … */ }
pub fn get_damage_type(id:&str)->Option<Box<dyn DamageType>> { /* clone boxed or Arc */ }
```
- Add tests: register duplicate → Err; get unknown → None.

**Acceptance Criteria**
- Thread‑safe registries; tests for register/get/duplicate behaviour.

**Commands**
```powershell
git checkout -b feat/registry-combat
ni -ItemType File ./actor-core/src/registry/combat.rs
git add .
git commit -m "feat(registry): damage/defence/amplifier registries + register_* APIs"
```

---

### 4) **Clamp coverage** incomplete (Derived‑level rules)
**What**: Only low‑level `Caps::clamp` exists; full Derived clamp (per field) is not implemented.  
**Why**: Prevent invalid stats; SPEC mandates strict ranges (e.g., hp_max≥1, stamina≥1, crit_chance∈[0,1], haste in [0.5, 2.0], etc.).  
**Where**: Implement in `core_api::clamp_derived()`.

**How**
- Implement field‑wise clamps in `clamp_derived()` with unit tests.
- Reject NaN/Inf by clamping to nearest legal bound or returning error (decide one policy).

**Acceptance Criteria**
- Unit tests for each field’s legal range; property tests ensure result ∈ legal set.

**Commands**
```powershell
git checkout -b fix/clamp-derived
git add .
git commit -m "feat(core): add full clamp_derived coverage"
```

---

### 5) **Determinism inside the same bucket**
**What**: Contributions within the same bucket are grouped in `HashMap`; order can be non‑deterministic.  
**Why**: SPEC requires stable application order for reproducible runs.  
**Where**: `bucket_processor.rs` (and any site applying same‑bucket folds).

**How**
- Before folding a bucket, **sort** contributions by `(dimension ASC, source_id ASC)` or store in `BTreeMap`.
- Keep existing bucket order (FLAT → MULT → POST_ADD → OVERRIDE → extras).

**Acceptance Criteria**
- Property test confirms same result after random shuffles of contributions.

**Commands**
```powershell
git checkout -b fix/determinism-in-bucket
git add .
git commit -m "fix(core): stable sort inside bucket processing"
```

---

### 6) **Lexicographic merge** for map‑level composition
**What**: When merging multi‑map contributions, must iterate keys in lexicographic order.  
**Why**: To guarantee identical results across platforms/runs.  
**Where**: Any map merge in `services.rs` or `bucket_processor.rs`.

**How**
- Convert `HashMap` to `BTreeMap` or collect keys, sort, then fold.

**Acceptance Criteria**
- Property test: random insertion order → identical outputs.

---

### 7) **Property‑based tests** (proptest) still missing for key invariants
**What**: File `property_tests.rs` contains mostly `#[test]` unit tests; need true property tests.  
**Why**: Catch ordering regressions & clamp invariants at scale.  
**Where**: `tests/property_tests.rs`

**How**
- Add `proptest!` suites:
  - **Ordering invariance**: shuffled contributions give same result.
  - **Clamp invariant**: `clamp_derived` always yields legal values.
  - **Monotonicity** (phase‑1): increasing PrimaryCore should not decrease monotone Derived fields.

**Acceptance Criteria**
- `cargo test` passes with & without `--features extra_buckets`; properties hold for thousands of cases.

**Commands**
```powershell
git checkout -b test/proptest-core
git add .
git commit -m "test(property): add proptest suites for ordering/clamp/monotonicity"
```

---

### 8) **Config dir override** via env var
**What**: `ACTOR_CORE_CONFIG_DIR` not yet supported.  
**Why**: Allow runtime override for ops/CI.  
**Where**: `registry/loader.rs`

**How**
```rust
let cfg_dir = std::env::var("ACTOR_CORE_CONFIG_DIR").ok()
    .map(PathBuf::from)
    .unwrap_or_else(|| PathBuf::from("./configs"));
```
- Try YAML, then JSON fallback; bubble errors with context.

**Acceptance Criteria**
- Test: set env var to temp dir with valid/invalid files; loader behaves as expected.

**Commands**
```powershell
git checkout -b feat/loader-env-override
git add .
git commit -m "feat(loader): ACTOR_CORE_CONFIG_DIR env override"
```

---

## P1 Enhancements

### 9) **Docs Sync**
- Update design docs to include: new public APIs, PrimaryCore/Derived structs, full clamp table, env override, determinism notes, extra_buckets feature.

### 10) **CI: cargo‑deny & docs**
- Add `cargo-deny` workflow or `make audit`. Ensure licenses and advisories are checked.
- Ensure `cargo doc --no-deps` passes on CI.

---

## Test Plan (combined)
- **Unit**: core_api functions; loader env override; registries duplicate/unknown; clamp cases.
- **Property**: ordering invariance; clamp invariants; no NaN/Inf; (optional) composition idempotence for neutral inputs.
- **Bench**: unchanged; ensure benches compile with/without `extra_buckets`.

---

## “Definition of Done” Checklist
- [ ] `core_api.rs` with **compose_core/base_from_primary/finalize_derived/clamp_derived** (public, re‑exported).
- [ ] `PrimaryCore` & `Derived` structs (serde + validation).
- [ ] Full **clamp_derived** coverage for all Derived fields.
- [ ] Determinism: stable sort inside bucket + lexicographic merge.
- [ ] Combat registries + `register_*` APIs with tests.
- [ ] Loader: **ACTOR_CORE_CONFIG_DIR** override + tests.
- [ ] Property tests (proptest) for ordering/clamp/monotonicity.
- [ ] Docs synced; CI (fmt/clippy/deny/doc) passes in one shot.

---

## Cursor AI — Copy‑Paste Prompt (per PR)
> **Task**: Implement Item X from *Missing Items & Completion Tasks*.  
> **Context files**: `src/core_api.rs`, `src/types_primary.rs`, `src/types_derived.rs`, `src/bucket_processor.rs`, `src/services.rs`, `src/registry/loader.rs`, `src/registry/combat.rs`, `tests/property_tests.rs`.  
> **Requirements**: Deterministic outputs, stable order, NaN/Inf guard, clamp per SPEC. Provide unit & property tests. Ensure `cargo fmt`, `clippy -D warnings`, `cargo test` all pass (with and without `--features extra_buckets`).

---

## Quick Commands (Windows PowerShell)
```powershell
# Run full suite
cargo fmt
cargo clippy --all-targets --all-features -D warnings
$env:ACTOR_CORE_CONFIG_DIR = (Resolve-Path ./actor-core/configs)
cargo test
cargo test --features extra_buckets
cargo doc --no-deps
```
