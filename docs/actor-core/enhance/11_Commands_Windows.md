# All Commands (Windows PowerShell)

> Run these from the repo root.

# Pre-flight
rustup show
rustc --version
cargo --version

# Phase 1: Registry Loader
git checkout -b feat/registry-loader
cargo add serde@1 --features derive
cargo add serde_yaml@0.9
cargo add serde_json@1
cargo add thiserror@1
cargo add anyhow@1
New-Item -ItemType Directory -Force ./src/registry | Out-Null
New-Item -ItemType File -Force ./src/registry/loader.rs | Out-Null
New-Item -ItemType Directory -Force ./configs | Out-Null
New-Item -ItemType File -Force ./configs/cap_layers.yaml | Out-Null
New-Item -ItemType File -Force ./configs/combiner.yaml | Out-Null
cargo fmt
cargo clippy --all-targets --all-features -D warnings
cargo test
git add .
git commit -m "feat(registry): loader (YAML/JSON) + sample configs"

# Phase 2: EffectiveCaps Alias
git checkout -b feat/effective-caps-alias
cargo fmt
cargo clippy --all-targets --all-features -D warnings
cargo test
git add .
git commit -m "feat(types): add EffectiveCaps alias and internal refactors"

# Phase 3: Feature Flags
git checkout -b feat/extra-buckets-feature
cargo test
cargo test --features extra_buckets
git add .
git commit -m "feat(features): gate extra buckets behind feature flag"

# Phase 4: Bucket Order & Clamping
git checkout -b feat/bucket-order
cargo test -p actor-core -- --nocapture
git add .
git commit -m "feat(aggregation): enforce bucket ordering and clamping"

# Phase 6: Tests + Property
git checkout -b test/property-based
cargo add proptest@1 --dev
cargo test
cargo test --features extra_buckets
git add .
git commit -m "test: unit & property tests for loader and aggregation"

# Phase 7: CI
git checkout -b chore/ci-guards
rustup component add clippy rustfmt
cargo install cargo-deny --locked
cargo fmt --all
cargo clippy --all-targets --all-features -D warnings
cargo deny check
cargo doc --no-deps
git add .
git commit -m "chore(ci): add fmt/clippy/deny/docs"

# Phase 8: Benchmarks
git checkout -b bench/registries
cargo add criterion@0.5 --dev
cargo bench
git add .
git commit -m "bench: criterion for registries"

# Phase 9: Docs
git checkout -b docs/sync-designs
# (edit markdown docs)
git add designs
git commit -m "docs: sync constants/enums/interfaces & combiner registry docs"
