---
id: P0-21
title: "hcm: HcmEngine trait and signalized intersection adapter"
labels: [type:task, crate:hcm, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-05, P0-20, P0-03]
---
## Goal
HCM 7 signalized analysis for golden g01, behind a Nixie interface.

## Context
- ADR-0016 (engines behind interfaces), ADR-0022 (engine choice, from P0-05)

## Scope
`crates/nixie-hcm/**`

## Acceptance
- [ ] `HcmEngine` trait taking lane groups, control and volumes keyed by stable IDs; results keyed by the same IDs
- [ ] Implementation per ADR-0022
- [ ] Gate 0 `assert_hcm` passes for golden g01 within the published example's reported precision
- [ ] Builds for wasm32 (browser tier runs HCM per ADR-0014)
