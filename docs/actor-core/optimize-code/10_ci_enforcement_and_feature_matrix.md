# 10 – CI enforcement and feature matrix

Goal: Enforce code quality and build/test across feature sets.

Quality gates
- rustfmt: `cargo fmt --all -- --check`
- clippy: `cargo clippy --workspace --all-targets -- -D warnings`
- deny: `cargo deny check`
- coverage (optional): `cargo llvm-cov --workspace --lcov --output-path lcov.info`

Feature matrix (actor-core)
- default
- moka-cache
- fs-cache
- redis-cache
- mongo
- sqlx-db
- cli

CI jobs
- Build + test for each feature set (include combinations where meaningful):
  - `cargo build -p actor-core --features <feat>`
  - `cargo test -p actor-core --features <feat>`
- Lints on default features.
- Optionally run benches on nightly or perf runners.

Artifacts
- Upload clippy, deny, and coverage reports.
