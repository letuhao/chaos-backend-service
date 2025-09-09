# Phase 1 — Implement Registry Loader (YAML/JSON)

## Goal
Externalize `CapLayerRegistry` and `CombinerRegistry` so they can be loaded from YAML/JSON files at runtime.
Keep a programmatic default as a fallback.

## Files to Create
- `src/registry/loader.rs`
- `configs/cap_layers.yaml`
- `configs/combiner.yaml`
- (Optional) `schemas/cap_layers.schema.json`, `schemas/combiner.schema.json`

## Exact Commands (Windows PowerShell)
```powershell
# 1) Create a working branch
git checkout -b feat/registry-loader

# 2) Add dependencies
cargo add serde@1 --features derive
cargo add serde_yaml@0.9
cargo add serde_json@1
cargo add thiserror@1
cargo add anyhow@1

# 3) Create files/folders
New-Item -ItemType Directory -Force ./src/registry | Out-Null
New-Item -ItemType File -Force ./src/registry/loader.rs | Out-Null
New-Item -ItemType Directory -Force ./configs | Out-Null
New-Item -ItemType File -Force ./configs/cap_layers.yaml | Out-Null
New-Item -ItemType File -Force ./configs/combiner.yaml | Out-Null

# 4) Build + test sanity
cargo fmt
cargo clippy --all-targets --all-features -D warnings
cargo test
```

## Exact Commands (Git Bash)
```bash
git checkout -b feat/registry-loader
cargo add serde@1 --features derive
cargo add serde_yaml@0.9
cargo add serde_json@1
cargo add thiserror@1
cargo add anyhow@1

mkdir -p src/registry configs
: > src/registry/loader.rs
: > configs/cap_layers.yaml
: > configs/combiner.yaml

cargo fmt
cargo clippy --all-targets --all-features -D warnings
cargo test
```

## Loader Responsibilities
- Resolve config path from (in order): env `ACTOR_CORE_CONFIG_DIR`, CLI arg, default `./configs`.
- Load YAML/JSON. Prefer YAML; if load fails, try JSON.
- Validate minimal shape (non-empty lists, unique names, legal enum values).
- Convert to in-memory `CapLayerRegistry` / `CombinerRegistry` types.
- Provide `fn load_cap_layers(path: impl AsRef<Path>) -> Result<CapLayerRegistry>` and `fn load_combiner(...) -> Result<CombinerRegistry>`.
- Provide `fn load_all(cfg_dir: impl AsRef<Path>) -> Result<(CapLayerRegistry, CombinerRegistry)>`.

## Cursor AI Prompt (copy from 10_Cursor_Prompts.md)
- See **P1** chunk.

## Acceptance Criteria
- `src/registry/loader.rs` exists and compiles.
- Unit tests cover: happy path (YAML), fallback to JSON, invalid values, duplicate names.
- Example configs under `/configs` parse successfully in CI.
