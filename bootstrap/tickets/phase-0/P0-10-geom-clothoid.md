---
id: P0-10
title: "geom: clothoid element"
labels: [type:task, crate:geom, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-09]
---
## Goal
Clothoid (Euler spiral) elements, including partial spirals between two arcs.

## Context
- ADR-0002

## Scope
`crates/nixie-geom/**`

## Acceptance
- [ ] `Clothoid` defined by start curvature, end curvature, length; implements `AlignmentElement`
- [ ] Position error below `COORD_RESOLUTION` for road-practical parameters (spiral parameter A from 20 m to 2000 m, length up to 500 m)
- [ ] Reference values produced by an **independent** method (e.g. high-precision quadrature script committed under `crates/nixie-geom/tests/ref/`), not by the implementation under test; PR description explains the method
- [ ] Tangent and curvature continuity in arc → spiral → arc sequences (property test)
