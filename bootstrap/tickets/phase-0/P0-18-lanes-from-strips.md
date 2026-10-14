---
id: P0-18
title: "compiler: corridor strips to lane edges and lane polygons"
labels: [type:task, crate:compiler, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-11, P0-16]
---
## Goal
Replace the lane stub: corridors compile into lanes with centrelines, edges, polygons and recorded error.

## Context
- ADR-0002, ADR-0004 (lane IDs from strip `AuthoredId` + role)

## Scope
`crates/nixie-compiler/**`

## Acceptance
- [ ] Each driving/bike strip produces a lane with centreline, left/right edges and polygon via `nixie-geom` offsets
- [ ] Lane `DerivedId`s are stable when an unrelated strip changes width (test)
- [ ] Achieved offset error recorded per lane
- [ ] Synthetic corridor tests including a taper
- [ ] GeoJSON export of lanes for debugging (display tessellation)
