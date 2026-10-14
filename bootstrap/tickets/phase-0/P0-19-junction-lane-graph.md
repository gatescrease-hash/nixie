---
id: P0-19
title: "junction: Phase 0 movement connectors and lane graph"
labels: [type:task, crate:junction, agent-ok, size:L]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-04, P0-18, P0-03]
---
## Goal
Implement the approach chosen in ADR-0021 so golden g01 compiles to a lane graph.

## Context
- ADR-0021 (from P0-04), ADR-0001 (overrides as escape hatch), ADR-0004 (movement keys)

## Scope
`crates/nixie-junction/**`, wiring in `crates/nixie-compiler/**`

## Acceptance
- [ ] Lane graph = lanes + connectors; every connector belongs to exactly one `MovementKey`
- [ ] Junction polygon per ADR-0021
- [ ] Gate 0 `assert_lane_graph` passes for golden g01
- [ ] Adding an exclusive turn lane leaves all movement IDs unchanged (ADR-0004 test)

## Notes
Labelled `size:L`: once ADR-0021 lands, split into connectors / junction polygon / wiring before claiming.
