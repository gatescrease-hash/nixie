---
id: P0-16
title: "compiler: dependency graph and incremental invalidation"
labels: [type:task, crate:compiler, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-13]
---
## Goal
The incremental engine: authored changes recompile only the derived objects that depend on them.

## Context
- ADR-0001 (overrides), ADR-0014 (incremental compiler)

## Scope
`crates/nixie-compiler/**`

## Acceptance
- [ ] `Stage` trait: declares authored inputs, produces derived outputs keyed by `DerivedId`
- [ ] Dirty propagation from an op to dependent nodes; test asserts **how many** nodes recompile after editing one corridor
- [ ] Validator hook returning typed diagnostics (error / warning / provisional)
- [ ] Override registry: attach, rebase (when exactly one successor), orphan-and-flag; property test from ADR-0001 (in = rebased + orphaned + attached)
- [ ] Builds for wasm32
