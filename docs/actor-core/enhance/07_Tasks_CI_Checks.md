# Phase 7 — CI Checks

## Goal
Keep repo healthy and enforce standards.

### Tools
- `cargo fmt`
- `cargo clippy --all-targets --all-features -D warnings`
- `cargo deny` (optional)
- `cargo doc --no-deps` (docs build)

## Commands
```powershell
git checkout -b chore/ci-guards
rustup component add clippy rustfmt
cargo install cargo-deny --locked
cargo fmt --all
cargo clippy --all-targets --all-features -D warnings
cargo deny check
cargo doc --no-deps
```
