---
id: P0-26
title: "golden: single-junction recompile budget as a CI assertion"
labels: [type:task, crate:golden, agent-ok, size:S]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-19]
---
## Goal
The < 50 ms single-junction recompile budget from ADR-0019 is measured and enforced.

## Context
- ADR-0019

## Scope
`crates/nixie-golden/**`, `.github/workflows/ci.yml` (budget job only)

## Acceptance
- [ ] Benchmark: edit one strip width on golden g01, recompile incrementally; report median and p95
- [ ] Gate 0 assertion fails when p95 exceeds the budget constant from `nixie-units`
- [ ] Runs on a pinned runner class; results posted as a CI summary
