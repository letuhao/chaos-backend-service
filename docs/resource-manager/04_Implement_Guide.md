# Resource Manager Implement Guide / 实施指南 / Hướng dẫn triển khai

This guide defines exact paths, types, and commands to implement the Resource Manager subsystem integrated with Actor Core v3.

## 1. Source Layout
- File: `crates/actor-core/src/subsystems/mod.rs`
- File: `crates/actor-core/src/subsystems/resource_manager.rs`
- Touch: `crates/actor-core/src/lib.rs` (pub use)
- Config: `docs/resource-manager/configs/combiner.resources.yaml`
- Config: `docs/resource-manager/configs/cap_layers.resources.yaml`

## 2. Create module scaffolding
1) Create module index
```rust
// crates/actor-core/src/subsystems/mod.rs
pub mod resource_manager;
```

2) Export in lib
```rust
// crates/actor-core/src/lib.rs
pub mod subsystems;
```

## 3. Implement ResourceManagerSubsystem
- File: `crates/actor-core/src/subsystems/resource_manager.rs`
- Struct: `ResourceManagerSubsystem`
- Implements: `crate::interfaces::Subsystem`
- Constructor: `pub fn new() -> Self`
- Fields:
  - `system_id: String`
  - `priority: i64`

Methods:
```rust
impl ResourceManagerSubsystem {
    pub fn new() -> Self { Self { system_id: "resource_manager".into(), priority: 100 } }
}

#[async_trait::async_trait]
impl crate::interfaces::Subsystem for ResourceManagerSubsystem {
    fn system_id(&self) -> &str { &self.system_id }
    fn priority(&self) -> i64 { self.priority }
    async fn contribute(&self, actor: &crate::types::Actor) -> crate::ActorCoreResult<crate::types::SubsystemOutput> {
        // 1) Read actor context (baseline current, max, regen, shield)
        // 2) Emit contributions per dimension using Bucket::Flat/Mult/PostAdd/Override as configured
        // 3) Return SubsystemOutput with added primary contributions
        let mut out = crate::types::SubsystemOutput::new(self.system_id.clone());
        // Example baseline; replace with real actor state access when available
        use crate::enums::Bucket;
        use crate::types::Contribution;
        let defaults = vec![
            Contribution { dimension: "hp_max".into(), bucket: Bucket::Flat, value: 1000.0, system: self.system_id.clone(), priority: Some(100), tags: None },
            Contribution { dimension: "hp_current".into(), bucket: Bucket::Flat, value: 960.0, system: self.system_id.clone(), priority: Some(100), tags: None },
        ];
        for c in defaults { out.add_primary(c); }
        Ok(out)
    }
}
```

## 4. Register subsystem
- File: `crates/actor-core/src/registry.rs`
- In `PluginRegistryImpl::register_defaults()` (or wherever subsystems are registered), add:
```rust
self.register(Box::new(crate::subsystems::resource_manager::ResourceManagerSubsystem::new()))?;
```

## 5. Config loading (ACTOR_CORE_CONFIG_DIR)
- Ensure `registry::loader` already reads `combiner.*.yaml` and `cap_layers.*.yaml`.
- For local testing, set environment variable in test:
```bash
$Env:ACTOR_CORE_CONFIG_DIR = "docs/resource-manager/configs"
```

## 6. Algorithms & flow
- Tick/Regen: add `*_regen * delta_seconds` to `*_current` in Bucket::Flat
- Shield decay: add `-shield_per_second * delta_seconds` to `shield_current`
- Offline catch-up: add `min(offline_seconds, offline_regen_max_seconds) * regen` to current
- Clamp precedence: EffectiveCaps → Combiner clamp_default → constants clamp ranges
- Derived `hp_percentage`: operator-mode (ratio of `hp_current/hp_max`)

## 7. Commands
- Build & test Actor Core crate:
```powershell
cd chaos-backend-service/crates/actor-core
cargo test && cargo test --features extra_buckets
```
- Run specific golden harness:
```powershell
cargo test golden_vector_harness -- --nocapture
```

## 8. Notes
- Follow strong typing; avoid any; use enums for buckets/operators.
- Ensure deterministic ordering (priority DESC, system ASC, value ASC) is preserved.
