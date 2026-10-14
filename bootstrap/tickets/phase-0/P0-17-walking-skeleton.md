---
id: P0-17
title: "cli: walking skeleton — `nixie compile` and `nixie check` end to end with stubs"
labels: [type:task, crate:cli, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-15, P0-16, P0-07]
---
## Goal
The whole Phase 0 pipeline exists end to end with stub stages, so every later ticket is "replace a stub" or "turn a red assertion green".

## Context
- Phase 0 pipeline: op log → model → lane graph → lane groups → HCM → SUMO export → LandXML export

## Scope
`crates/nixie-cli/**`, stub stage modules in `crates/nixie-compiler/**`, `crates/nixie-golden/**` (wiring only)

## Acceptance
- [ ] `nixie compile <oplog.json> --out <dir>` replays, compiles, writes per-stage outputs; stub stages emit a `NotImplemented` diagnostic naming the owning ticket
- [ ] `nixie check <oplog.json>` exits non-zero on any error or provisional diagnostic (ADR-0006)
- [ ] `nixie-golden` drives the pipeline through the CLI library, not by shelling out
- [ ] Gate 0 suite now fails **at named stages**, not at loading
