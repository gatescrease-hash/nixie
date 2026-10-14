---
id: P0-06
title: "ci: Cargo workspace and CI pipeline"
labels: [type:task, area:ci, agent-ok, size:S]
milestone: "Phase 0 — Vertical slice"
blocked_by: []
---
## Goal
Every crate in the `AGENTS.md` repo map exists as a compiling stub, and CI enforces the rules agents will be held to.

## Context
- `AGENTS.md` (repo map, commands)
- ADR-0012 (no literal epsilons), ADR-0014 (C-free wasm32), ADR-0017 (licences, DCO)

## Scope
`Cargo.toml`, `rust-toolchain.toml`, `crates/*/Cargo.toml`, `crates/*/src/lib.rs` (stubs only), `deny.toml`, `scripts/lint-epsilon.sh`, `.github/workflows/ci.yml`

## Acceptance
- [ ] `cargo build --workspace` and `cargo test --workspace` pass
- [ ] `cargo build -p nixie-compiler --target wasm32-unknown-unknown` passes in CI
- [ ] `cargo deny check licenses` with an allow-list (MIT, Apache-2.0, BSD-2/3, ISC, Zlib, Unicode-3.0); LGPL allowed only as an explicit exception for native-only crates
- [ ] `scripts/lint-epsilon.sh` fails on float literals below `1e-3` outside `crates/nixie-units`; wired into CI
- [ ] Clippy runs with `-D warnings` and `clippy::float_cmp` denied
- [ ] A separate, **non-required** CI job `gate0` runs `cargo test -p nixie-golden --features gate0`
- [ ] Toolchain pinned in `rust-toolchain.toml`

## Out of scope
Any real implementation inside the crates.
