# 04 – Cargo feature gating and dependency slimming

Goal: Reduce compile time by gating heavy/optional dependencies.

Edits
- File: `crates/actor-core/Cargo.toml`
  - Add features:
    - `[features]`:
      - `default = ["inmemory-cache"]`
      - `inmemory-cache = []`
      - `moka-cache = ["moka"]`
      - `fs-cache = ["memmap2"]`
      - `redis-cache = ["redis"]`
      - `mongo = ["mongodb", "bson"]`
      - `sqlx-db = ["sqlx"]`
      - `cli = ["clap", "tracing-subscriber"]`
  - Move corresponding deps under optional where applicable:
    - `moka = { workspace = true, optional = true }`
    - `memmap2 = { workspace = true, optional = true }`
    - `redis = { workspace = true, optional = true }`
    - `mongodb = { version = "2.8", optional = true }`
    - `bson = { version = "2.8", optional = true }`
    - `sqlx = { workspace = true, optional = true }`
    - `clap = { version = "4", features = ["derive"], optional = true }`
    - `tracing-subscriber = { version = "0.3", optional = true }`

Notes
- Ensure code that uses gated deps is behind `#[cfg(feature = "...")]`.
- Update examples/benches that depend on gated features with `--features` in make targets.
