---
id: P0-11
title: "geom: offset curves to tolerance with recorded error"
labels: [type:task, crate:geom, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-10]
---
## Goal
Approximate offset curves (constant and linearly varying width) with a guaranteed and **recorded** maximum error.

## Context
- ADR-0002 (offsets are approximations with achieved error recorded)

## Scope
`crates/nixie-geom/**`

## Acceptance
- [ ] `offset(alignment, width_fn, tol) -> OffsetCurve { polyline, achieved_error }` with adaptive subdivision
- [ ] Supports constant width and linear tapers along a station range
- [ ] Returns a typed error (not garbage) when the offset exceeds the local radius of curvature
- [ ] Property test: dense-sample error ≤ reported `achieved_error` ≤ `OFFSET_CAD`
- [ ] Benchmark recorded for a 1 km mixed alignment (no threshold yet)

## Out of scope
Biarc output (Phase 1).
