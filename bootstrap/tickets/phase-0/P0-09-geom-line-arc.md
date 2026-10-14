---
id: P0-09
title: "geom: line and circular-arc alignment elements"
labels: [type:task, crate:geom, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-08]
---
## Goal
Exact horizontal alignment elements for lines and arcs, evaluated by true distance.

## Context
- ADR-0002 (exact centrelines), ADR-0003 (true distance internally), ADR-0012

## Scope
`crates/nixie-geom/**`

## Acceptance
- [ ] `AlignmentElement` trait: `length`, `point_at(s)`, `tangent_at(s)`, `curvature_at(s)`, `project(p) -> (s, lateral_offset)`
- [ ] `Line` and `Arc` implementations; `HorizontalAlignment` as an ordered element list with a G1 continuity check using a named tolerance
- [ ] Coordinates are relative to a project local origin (f64)
- [ ] Property tests (proptest): `project(point_at(s)).s ≈ s` within `COORD_RESOLUTION`; tangent continuity at joins
- [ ] Builds for wasm32; no C dependencies
